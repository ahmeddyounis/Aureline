//! Bounded, identity-checked I/O for shell-owned JSON artifacts.
//!
//! These helpers are intentionally private to the shell crate. Public record
//! shapes stay with their owning modules while filesystem reads and durable
//! rewrites share the same resource and stale-target guardrails.

use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;

static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Reads one regular file without following a final-component symlink and
/// stops after `max_bytes + 1` bytes.
///
/// Metadata captured before open, from the open handle, and after the read
/// must describe the same object. Callers therefore never parse bytes from a
/// path that changed identity while it was being inspected.
pub(crate) fn read_bounded_regular_file(path: &Path, max_bytes: u64) -> io::Result<Vec<u8>> {
    Ok(read_bounded_regular_file_with_identity(path, max_bytes)?.bytes)
}

/// Bounded bytes plus the filesystem generation observed for those bytes.
#[derive(Debug)]
pub(crate) struct BoundedArtifactRead {
    pub(crate) bytes: Vec<u8>,
    pub(crate) identity: ArtifactIdentity,
}

/// Bounded UTF-8 text plus the filesystem generation observed for that text.
#[derive(Debug)]
pub(crate) struct BoundedUtf8ArtifactRead {
    pub(crate) text: String,
    pub(crate) identity: ArtifactIdentity,
}

/// Identity-returning form used by durable stores for compare-before-write.
pub(crate) fn read_bounded_regular_file_with_identity(
    path: &Path,
    max_bytes: u64,
) -> io::Result<BoundedArtifactRead> {
    let before = fs::symlink_metadata(path)?;
    require_regular_nonsymlink(&before)?;
    require_size_within_limit(&before, max_bytes)?;
    let before_stamp = FileStamp::from_metadata(&before);

    let file = File::open(path)?;
    let opened = file.metadata()?;
    require_regular_nonsymlink(&opened)?;
    require_size_within_limit(&opened, max_bytes)?;
    let opened_stamp = FileStamp::from_metadata(&opened);
    if before_stamp != opened_stamp {
        return Err(invalid_data("artifact identity changed while opening"));
    }

    let initial_capacity = usize::try_from(opened.len())
        .unwrap_or(usize::MAX)
        .min(64 * 1024);
    let mut bytes = Vec::with_capacity(initial_capacity);
    file.take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > max_bytes {
        return Err(invalid_data("artifact exceeds configured byte limit"));
    }

    let after = fs::symlink_metadata(path)?;
    require_regular_nonsymlink(&after)?;
    if FileStamp::from_metadata(&after) != opened_stamp {
        return Err(invalid_data("artifact identity changed while reading"));
    }

    Ok(BoundedArtifactRead {
        bytes,
        identity: ArtifactIdentity(opened_stamp),
    })
}

/// Reads identity-stable UTF-8 text without exposing invalid bytes in the
/// resulting error. Retain `identity` and pass it to
/// [`write_atomic_regular_file`] for a compare-before-write update.
pub(crate) fn read_bounded_utf8_regular_file_with_identity(
    path: &Path,
    max_bytes: u64,
) -> io::Result<BoundedUtf8ArtifactRead> {
    let read = read_bounded_regular_file_with_identity(path, max_bytes)?;
    let text = String::from_utf8(read.bytes)
        .map_err(|_| invalid_data("artifact must contain valid UTF-8"))?;
    Ok(BoundedUtf8ArtifactRead {
        text,
        identity: read.identity,
    })
}

/// Convenience reader for bounded UTF-8 artifacts that are not subsequently
/// rewritten by the caller.
pub(crate) fn read_bounded_utf8_regular_file(path: &Path, max_bytes: u64) -> io::Result<String> {
    Ok(read_bounded_utf8_regular_file_with_identity(path, max_bytes)?.text)
}

/// Resolves a durable artifact path to one canonical parent directory.
///
/// The immediate parent must be a real directory rather than a symlink. The
/// resolved path can then be retained by a store so later writes do not follow
/// a parent link whose destination changed after startup.
pub(crate) fn prepare_durable_artifact_path(path: &Path) -> io::Result<PathBuf> {
    let file_name = path
        .file_name()
        .filter(|name| !name.is_empty())
        .ok_or_else(|| invalid_input("durable artifact path has no file name"))?;
    let parent = normalized_parent(path);
    fs::create_dir_all(parent)?;
    let parent_metadata = fs::symlink_metadata(parent)?;
    if parent_metadata.file_type().is_symlink() || !parent_metadata.is_dir() {
        return Err(invalid_data(
            "durable artifact parent must be a non-symlink directory",
        ));
    }
    let canonical_parent = fs::canonicalize(parent)?;
    Ok(canonical_parent.join(file_name))
}

/// Atomically replaces one bounded regular file after comparing its current
/// identity with the identity observed before serialization.
pub(crate) fn write_atomic_regular_file(
    path: &Path,
    bytes: &[u8],
    max_bytes: u64,
    expected_identity: Option<ArtifactIdentity>,
) -> io::Result<ArtifactIdentity> {
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > max_bytes {
        return Err(invalid_data("artifact exceeds configured byte limit"));
    }

    let file_name = path
        .file_name()
        .filter(|name| !name.is_empty())
        .ok_or_else(|| invalid_input("durable artifact path has no file name"))?;
    let parent = normalized_parent(path);
    fs::create_dir_all(parent)?;
    let parent_metadata = fs::symlink_metadata(parent)?;
    if parent_metadata.file_type().is_symlink() || !parent_metadata.is_dir() {
        return Err(invalid_data(
            "durable artifact parent must be a non-symlink directory",
        ));
    }
    let canonical_parent = fs::canonicalize(parent)?;
    let target = canonical_parent.join(file_name);
    let target_before = optional_regular_stamp(&target)?;
    if target_before.as_ref().map(|(identity, _)| *identity) != expected_identity {
        return Err(invalid_data(
            "durable artifact target changed since the last accepted read",
        ));
    }

    let mut pending = create_pending_temp(&canonical_parent)?;
    pending.file_mut().write_all(bytes)?;
    pending.file_mut().flush()?;
    if let Some((_, permissions)) = &target_before {
        pending.file_mut().set_permissions(permissions.clone())?;
    }
    pending.file_mut().sync_all()?;
    pending.close();

    let current = optional_regular_stamp(&target)?;
    let identity_unchanged = match (&target_before, &current) {
        (None, None) => true,
        (Some((before, _)), Some((after, _))) => before == after,
        _ => false,
    };
    if !identity_unchanged {
        return Err(invalid_data(
            "durable artifact target changed before atomic replace",
        ));
    }

    fs::rename(pending.path(), &target)?;
    pending.disarm();

    let after = fs::symlink_metadata(&target)?;
    require_regular_nonsymlink(&after)?;
    if after.len() != u64::try_from(bytes.len()).unwrap_or(u64::MAX) {
        return Err(invalid_data(
            "durable artifact length changed after atomic replace",
        ));
    }
    sync_directory(&canonical_parent)?;
    Ok(ArtifactIdentity(FileStamp::from_metadata(&after)))
}

/// Serializes JSON into a writer that refuses to grow past `max_bytes`.
pub(crate) fn to_bounded_json_pretty<T: ?Sized + Serialize>(
    value: &T,
    max_bytes: usize,
) -> Result<Vec<u8>, serde_json::Error> {
    let mut writer = BoundedVecWriter::new(max_bytes);
    serde_json::to_writer_pretty(&mut writer, value)?;
    Ok(writer.into_inner())
}

fn normalized_parent(path: &Path) -> &Path {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn require_regular_nonsymlink(metadata: &Metadata) -> io::Result<()> {
    if metadata.file_type().is_symlink() {
        return Err(invalid_data("artifact path must not be a symlink"));
    }
    if !metadata.is_file() {
        return Err(invalid_data("artifact path must identify a regular file"));
    }
    Ok(())
}

fn require_size_within_limit(metadata: &Metadata, max_bytes: u64) -> io::Result<()> {
    if metadata.len() > max_bytes {
        return Err(invalid_data("artifact exceeds configured byte limit"));
    }
    Ok(())
}

fn optional_regular_stamp(path: &Path) -> io::Result<Option<(ArtifactIdentity, fs::Permissions)>> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            require_regular_nonsymlink(&metadata)?;
            Ok(Some((
                ArtifactIdentity(FileStamp::from_metadata(&metadata)),
                metadata.permissions(),
            )))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn create_pending_temp(parent: &Path) -> io::Result<PendingTemp> {
    for _ in 0..32 {
        let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let name = format!(".aureline-artifact-tmp-{}-{sequence}", std::process::id());
        let path = parent.join(name);
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => {
                if let Err(error) = restrict_new_file_permissions(&file) {
                    drop(file);
                    let _ = fs::remove_file(&path);
                    return Err(error);
                }
                return Ok(PendingTemp {
                    file: Some(file),
                    path,
                    armed: true,
                });
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "unable to allocate durable artifact temporary file",
    ))
}

#[cfg(unix)]
fn restrict_new_file_permissions(file: &File) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    file.set_permissions(fs::Permissions::from_mode(0o600))
}

#[cfg(not(unix))]
fn restrict_new_file_permissions(_file: &File) -> io::Result<()> {
    Ok(())
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> io::Result<()> {
    Ok(())
}

fn invalid_data(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn invalid_input(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

struct PendingTemp {
    file: Option<File>,
    path: PathBuf,
    armed: bool,
}

impl PendingTemp {
    fn file_mut(&mut self) -> &mut File {
        self.file
            .as_mut()
            .expect("pending temporary file remains open until close")
    }

    fn close(&mut self) {
        self.file.take();
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for PendingTemp {
    fn drop(&mut self) {
        self.file.take();
        if self.armed {
            let _ = fs::remove_file(&self.path);
        }
    }
}

struct BoundedVecWriter {
    bytes: Vec<u8>,
    max_bytes: usize,
}

/// Opaque filesystem generation token retained by durable stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ArtifactIdentity(FileStamp);

impl BoundedVecWriter {
    fn new(max_bytes: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(max_bytes.min(64 * 1024)),
            max_bytes,
        }
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BoundedVecWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if buffer.len() > self.max_bytes.saturating_sub(self.bytes.len()) {
            return Err(invalid_data("serialized artifact exceeds byte limit"));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(unix)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileStamp {
    device: u64,
    inode: u64,
    size: u64,
    mode: u32,
    modified_seconds: i64,
    modified_nanoseconds: i64,
    changed_seconds: i64,
    changed_nanoseconds: i64,
}

#[cfg(unix)]
impl FileStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            size: metadata.size(),
            mode: metadata.mode(),
            modified_seconds: metadata.mtime(),
            modified_nanoseconds: metadata.mtime_nsec(),
            changed_seconds: metadata.ctime(),
            changed_nanoseconds: metadata.ctime_nsec(),
        }
    }
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileStamp {
    file_attributes: u32,
    creation_time: u64,
    last_write_time: u64,
    file_size: u64,
}

#[cfg(windows)]
impl FileStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::windows::fs::MetadataExt;

        Self {
            file_attributes: metadata.file_attributes(),
            creation_time: metadata.creation_time(),
            last_write_time: metadata.last_write_time(),
            file_size: metadata.file_size(),
        }
    }
}

#[cfg(not(any(unix, windows)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileStamp {
    size: u64,
    modified: Option<std::time::SystemTime>,
}

#[cfg(not(any(unix, windows)))]
impl FileStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        Self {
            size: metadata.len(),
            modified: metadata.modified().ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_read_rejects_oversized_files_without_echoing_paths() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("private-workspace-name.json");
        fs::write(&path, b"12345").expect("write fixture");

        let error = read_bounded_regular_file(&path, 4).expect_err("must reject oversized file");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(!error.to_string().contains("private-workspace-name"));
    }

    #[cfg(unix)]
    #[test]
    fn bounded_read_and_atomic_write_reject_final_symlinks() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir");
        let target = directory.path().join("target.json");
        let link = directory.path().join("rows.json");
        fs::write(&target, b"[]").expect("write target");
        symlink(&target, &link).expect("create symlink");

        let read_error = read_bounded_regular_file(&link, 1024).expect_err("read must reject");
        assert_eq!(read_error.kind(), io::ErrorKind::InvalidData);
        let write_error =
            write_atomic_regular_file(&link, b"[1]", 1024, None).expect_err("write must reject");
        assert_eq!(write_error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(fs::read(&target).expect("target remains"), b"[]");
    }

    #[test]
    fn atomic_write_preserves_prior_file_when_serialized_payload_is_too_large() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("rows.json");
        fs::write(&path, b"old").expect("write prior file");

        let error = write_atomic_regular_file(&path, b"too-large", 3, None)
            .expect_err("oversized write must fail");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(fs::read(&path).expect("read prior file"), b"old");
    }

    #[test]
    fn bounded_json_writer_refuses_memory_growth_past_limit() {
        let value = vec!["sensitive".repeat(32)];
        let error = to_bounded_json_pretty(&value, 16).expect_err("must enforce output cap");
        assert!(error.is_io());
    }

    #[test]
    fn bounded_utf8_reader_rejects_invalid_text_without_echoing_bytes() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("settings.json");
        fs::write(&path, [0xff, 0xfe, b's']).expect("write invalid UTF-8");

        let error = read_bounded_utf8_regular_file(&path, 1024).expect_err("must reject");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(error.to_string(), "artifact must contain valid UTF-8");
    }

    #[test]
    fn atomic_write_rejects_a_stale_generation_token() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("rows.json");
        fs::write(&path, b"old").expect("write prior file");
        let observed = read_bounded_regular_file_with_identity(&path, 1024)
            .expect("read identity")
            .identity;
        fs::write(&path, b"external-change").expect("external change");

        let error = write_atomic_regular_file(&path, b"ours", 1024, Some(observed))
            .expect_err("stale generation must fail");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(
            fs::read(&path).expect("external file remains"),
            b"external-change"
        );
    }

    #[cfg(unix)]
    #[test]
    fn new_atomic_artifacts_are_private_by_default() {
        use std::os::unix::fs::PermissionsExt;

        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("rows.json");
        write_atomic_regular_file(&path, b"[]", 1024, None).expect("write artifact");

        let mode = fs::symlink_metadata(&path)
            .expect("metadata")
            .permissions()
            .mode();
        assert_eq!(mode & 0o077, 0);
    }
}
