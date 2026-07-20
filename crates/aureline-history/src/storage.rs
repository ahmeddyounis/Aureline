use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Maximum serialized size of one durable history or mutation-journal record.
pub(crate) const MAX_HISTORY_RECORD_BYTES: usize = 2 * 1024 * 1024;

/// Maximum size of one local-history body object.
pub(crate) const MAX_HISTORY_BODY_BYTES: usize = 64 * 1024 * 1024;

pub(crate) const MAX_STORAGE_ID_BYTES: usize = 160;

static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(1);

/// Error returned when history persistence fails.
#[derive(Debug)]
pub enum HistoryError {
    /// Filesystem persistence failed.
    Io(std::io::Error),
    /// JSON serialization failed.
    Json(serde_json::Error),
    /// Record-kind validation failed against the record-class registry.
    RecordRegistry(aureline_records::RecordRegistryError),
    /// A caller supplied a path or opaque identifier outside the store contract.
    InvalidInput(&'static str),
    /// A durable record or body exceeded its published storage bound.
    TooLarge(&'static str),
    /// A content-addressed body did not match its declared digest.
    Integrity(&'static str),
}

impl std::fmt::Display for HistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "history io error: {err}"),
            Self::Json(err) => write!(f, "history json error: {err}"),
            Self::RecordRegistry(err) => write!(f, "history record registry error: {err}"),
            Self::InvalidInput(detail) => write!(f, "history input rejected: {detail}"),
            Self::TooLarge(detail) => write!(f, "history storage bound exceeded: {detail}"),
            Self::Integrity(detail) => write!(f, "history integrity failure: {detail}"),
        }
    }
}

impl std::error::Error for HistoryError {}

impl From<std::io::Error> for HistoryError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for HistoryError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<aureline_records::RecordRegistryError> for HistoryError {
    fn from(value: aureline_records::RecordRegistryError) -> Self {
        Self::RecordRegistry(value)
    }
}

/// Identifier source used by history stores.
#[derive(Debug, Clone)]
pub struct IdSource {
    prefix: &'static str,
    next_seq: u64,
}

impl IdSource {
    /// Creates a new id source with a stable prefix.
    pub const fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            next_seq: 1,
        }
    }

    /// Mints a new opaque id.
    pub fn mint(&mut self) -> String {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.saturating_add(1);
        let stamp = unix_nanos();
        format!("{prefix}-{stamp:020}-{seq:06}", prefix = self.prefix)
    }
}

fn unix_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

pub(crate) fn validate_storage_id(value: &str) -> Result<(), HistoryError> {
    if value.is_empty()
        || value.len() > MAX_STORAGE_ID_BYTES
        || value == "."
        || value == ".."
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(HistoryError::InvalidInput(
            "durable record id is not a bounded filename-safe opaque id",
        ));
    }
    Ok(())
}

/// Storage root for history persistence.
#[derive(Debug, Clone)]
pub struct HistoryStorageRoot {
    root: PathBuf,
}

impl HistoryStorageRoot {
    /// Creates a storage root at `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Returns the on-disk root path.
    pub fn path(&self) -> &Path {
        &self.root
    }

    /// Ensures the given directory exists.
    pub fn ensure_dir(&self, path: &Path) -> Result<(), HistoryError> {
        let relative = self.checked_relative(path)?;
        self.ensure_root()?;

        let mut current = self.root.clone();
        for component in relative.components() {
            let Component::Normal(component) = component else {
                return Err(HistoryError::InvalidInput(
                    "history path contains a non-normal component",
                ));
            };
            current.push(component);
            match fs::symlink_metadata(&current) {
                Ok(metadata) if metadata.file_type().is_symlink() => {
                    return Err(HistoryError::InvalidInput(
                        "history directory may not be a symlink",
                    ));
                }
                Ok(metadata) if !metadata.is_dir() => {
                    return Err(HistoryError::InvalidInput(
                        "history directory path is not a directory",
                    ));
                }
                Ok(_) => {}
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    match fs::create_dir(&current) {
                        Ok(()) => {}
                        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                            let metadata = fs::symlink_metadata(&current)?;
                            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                                return Err(HistoryError::InvalidInput(
                                    "history directory was replaced during creation",
                                ));
                            }
                        }
                        Err(err) => return Err(HistoryError::Io(err)),
                    }
                }
                Err(err) => return Err(HistoryError::Io(err)),
            }
        }
        Ok(())
    }

    /// Writes a brand-new JSON record file with best-effort durability.
    pub fn write_new_json<T: serde::Serialize>(
        &self,
        path: &Path,
        value: &T,
    ) -> Result<(), HistoryError> {
        if let Some(parent) = path.parent() {
            self.ensure_dir(parent)?;
        }
        let json = serde_json::to_string_pretty(value)?;
        if json.len() > MAX_HISTORY_RECORD_BYTES {
            return Err(HistoryError::TooLarge("serialized history record"));
        }
        self.publish_new_file(path, json.as_bytes())?;
        Ok(())
    }

    /// Writes a brand-new binary blob with best-effort durability.
    pub fn write_new_blob(&self, path: &Path, bytes: &[u8]) -> Result<(), HistoryError> {
        if bytes.len() > MAX_HISTORY_BODY_BYTES {
            return Err(HistoryError::TooLarge("local-history body object"));
        }
        if let Some(parent) = path.parent() {
            self.ensure_dir(parent)?;
        }
        self.publish_new_file(path, bytes)
    }

    /// Reads a regular store file through a strict byte bound.
    pub(crate) fn read_bounded_file(
        &self,
        path: &Path,
        max_bytes: usize,
    ) -> Result<Vec<u8>, HistoryError> {
        self.checked_relative(path)?;
        let metadata = fs::symlink_metadata(path)?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(HistoryError::InvalidInput(
                "history object is not a regular file",
            ));
        }
        if metadata.len() > max_bytes as u64 {
            return Err(HistoryError::TooLarge("history object read"));
        }

        let file = fs::File::open(path)?;
        let opened_metadata = file.metadata()?;
        if !opened_metadata.is_file() || opened_metadata.len() > max_bytes as u64 {
            return Err(HistoryError::InvalidInput(
                "history object changed during read",
            ));
        }
        let mut bytes = Vec::with_capacity(opened_metadata.len() as usize);
        file.take(max_bytes as u64 + 1).read_to_end(&mut bytes)?;
        if bytes.len() > max_bytes {
            return Err(HistoryError::TooLarge("history object read"));
        }
        Ok(bytes)
    }

    fn checked_relative<'a>(&self, path: &'a Path) -> Result<&'a Path, HistoryError> {
        let relative = path
            .strip_prefix(&self.root)
            .map_err(|_| HistoryError::InvalidInput("history path escapes storage root"))?;
        if relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(HistoryError::InvalidInput(
                "history path contains a non-normal component",
            ));
        }
        Ok(relative)
    }

    fn ensure_root(&self) -> Result<(), HistoryError> {
        if self.root.as_os_str().is_empty()
            || self
                .root
                .components()
                .any(|component| matches!(component, Component::ParentDir))
        {
            return Err(HistoryError::InvalidInput("invalid history storage root"));
        }
        match fs::symlink_metadata(&self.root) {
            Ok(metadata) if metadata.file_type().is_symlink() => Err(HistoryError::InvalidInput(
                "history storage root may not be a symlink",
            )),
            Ok(metadata) if !metadata.is_dir() => Err(HistoryError::InvalidInput(
                "history storage root is not a directory",
            )),
            Ok(_) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir_all(&self.root)?;
                let metadata = fs::symlink_metadata(&self.root)?;
                if metadata.file_type().is_symlink() || !metadata.is_dir() {
                    return Err(HistoryError::InvalidInput(
                        "history storage root creation was redirected",
                    ));
                }
                Ok(())
            }
            Err(err) => Err(HistoryError::Io(err)),
        }
    }

    fn publish_new_file(&self, path: &Path, bytes: &[u8]) -> Result<(), HistoryError> {
        self.checked_relative(path)?;
        let parent = path
            .parent()
            .ok_or(HistoryError::InvalidInput("history file has no parent"))?;
        self.ensure_dir(parent)?;

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(HistoryError::InvalidInput("history filename is not UTF-8"))?;
        if file_name.is_empty() || file_name == "." || file_name == ".." {
            return Err(HistoryError::InvalidInput("invalid history filename"));
        }

        let temporary = unique_temporary_path(parent, file_name);
        let write_result = (|| -> Result<(), HistoryError> {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)?;
            file.write_all(bytes)?;
            file.sync_all()?;
            fs::hard_link(&temporary, path)?;
            sync_directory(parent)?;
            Ok(())
        })();
        let _ = fs::remove_file(&temporary);
        write_result
    }
}

fn unique_temporary_path(parent: &Path, file_name: &str) -> PathBuf {
    let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    parent.join(format!(
        ".{file_name}.tmp-{}-{sequence}-{}",
        std::process::id(),
        unix_nanos()
    ))
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), HistoryError> {
    fs::File::open(path)?.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), HistoryError> {
    Ok(())
}
