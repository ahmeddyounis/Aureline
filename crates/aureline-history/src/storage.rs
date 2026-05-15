use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Error returned when history persistence fails.
#[derive(Debug)]
pub enum HistoryError {
    /// Filesystem persistence failed.
    Io(std::io::Error),
    /// JSON serialization failed.
    Json(serde_json::Error),
    /// Record-kind validation failed against the record-class registry.
    RecordRegistry(aureline_records::RecordRegistryError),
}

impl std::fmt::Display for HistoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "history io error: {err}"),
            Self::Json(err) => write!(f, "history json error: {err}"),
            Self::RecordRegistry(err) => write!(f, "history record registry error: {err}"),
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
        create_dir_all(path)?;
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
        write_new_file(path, json.as_bytes())?;
        Ok(())
    }

    /// Writes a brand-new binary blob with best-effort durability.
    pub fn write_new_blob(&self, path: &Path, bytes: &[u8]) -> Result<(), HistoryError> {
        if let Some(parent) = path.parent() {
            self.ensure_dir(parent)?;
        }
        write_new_file(path, bytes)?;
        Ok(())
    }
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), HistoryError> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}
