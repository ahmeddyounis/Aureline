//! Bounded, generation-checked I/O for shell-owned JSON artifacts.
//!
//! These helpers are intentionally private to the shell crate. Public record
//! shapes stay with their owning modules while filesystem reads and durable
//! rewrites share the same resource and stale-target guardrails.

#[cfg(test)]
use std::cell::Cell;
use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;

static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Reads one regular file without following untrusted ancestor or
/// final-component path redirects and stops after `max_bytes + 1` bytes.
///
/// Metadata captured before open, from the open handle, and after the read
/// must remain stable, and a resolved-parent metadata token is checked around
/// those operations. This rejects observed replacement; `std` path APIs cannot
/// exclude an attacker swapping and restoring a parent entirely between two
/// checks.
pub(crate) fn read_bounded_regular_file(path: &Path, max_bytes: u64) -> io::Result<Vec<u8>> {
    Ok(read_bounded_regular_file_with_identity(path, max_bytes)?.bytes)
}

/// Bounded bytes plus the filesystem generation observed for those bytes.
#[derive(Debug)]
pub(crate) struct BoundedArtifactRead {
    pub(crate) bytes: Vec<u8>,
    pub(crate) identity: ArtifactIdentity,
}

/// Identity-returning form used by durable stores for compare-before-write.
pub(crate) fn read_bounded_regular_file_with_identity(
    path: &Path,
    max_bytes: u64,
) -> io::Result<BoundedArtifactRead> {
    read_bounded_regular_file_with_identity_impl(path, max_bytes, |_| {}, |_| {})
}

/// Reads a private durable artifact and rejects permissive parent or file
/// modes before returning any bytes to its owner.
pub(crate) fn read_bounded_private_regular_file_with_identity(
    path: &Path,
    max_bytes: u64,
) -> io::Result<BoundedArtifactRead> {
    let resolved = resolve_artifact_path(path, false)?;
    let parent = artifact_parent(&resolved)?;
    let parent_token = observed_directory_metadata_token(parent)?;
    require_private_artifact_parent(parent)?;
    let before = fs::symlink_metadata(&resolved)?;
    require_regular_nonsymlink(&before)?;
    require_private_artifact_file(&before)?;
    let before_identity = ArtifactIdentity(FileStamp::from_metadata(&before));
    require_directory_metadata_stable(parent, parent_token)?;

    let read = read_bounded_regular_file_with_identity(&resolved, max_bytes)?;
    if read.identity != before_identity {
        return Err(invalid_data(
            "private durable artifact changed during permission validation",
        ));
    }

    let after = fs::symlink_metadata(&resolved)?;
    require_regular_nonsymlink(&after)?;
    require_private_artifact_file(&after)?;
    if ArtifactIdentity(FileStamp::from_metadata(&after)) != read.identity {
        return Err(invalid_data(
            "private durable artifact changed during permission validation",
        ));
    }
    require_directory_metadata_stable(parent, parent_token)?;
    Ok(read)
}

fn read_bounded_regular_file_with_identity_impl<AfterParentPinned, AfterRead>(
    path: &Path,
    max_bytes: u64,
    after_parent_pinned: AfterParentPinned,
    after_read_before_validation: AfterRead,
) -> io::Result<BoundedArtifactRead>
where
    AfterParentPinned: FnOnce(&Path),
    AfterRead: FnOnce(&Path),
{
    let path = resolve_artifact_path(path, false)?;
    let parent = artifact_parent(&path)?;
    let parent_token = observed_directory_metadata_token(parent)?;
    after_parent_pinned(parent);
    require_directory_metadata_stable(parent, parent_token)?;

    let before = fs::symlink_metadata(&path)?;
    require_regular_nonsymlink(&before)?;
    require_size_within_limit(&before, max_bytes)?;
    let before_stamp = FileStamp::from_metadata(&before);
    require_directory_metadata_stable(parent, parent_token)?;

    let mut file = File::open(&path)?;
    let opened = file.metadata()?;
    require_regular_nonsymlink(&opened)?;
    require_size_within_limit(&opened, max_bytes)?;
    let opened_stamp = FileStamp::from_metadata(&opened);
    if before_stamp != opened_stamp {
        return Err(invalid_data("artifact identity changed while opening"));
    }
    require_directory_metadata_stable(parent, parent_token)?;

    let initial_capacity = usize::try_from(opened.len())
        .unwrap_or(usize::MAX)
        .min(64 * 1024);
    let mut bytes = Vec::with_capacity(initial_capacity);
    Read::by_ref(&mut file)
        .take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > max_bytes {
        return Err(invalid_data("artifact exceeds configured byte limit"));
    }

    after_read_before_validation(&path);

    let descriptor_after = file.metadata()?;
    require_regular_nonsymlink(&descriptor_after)?;
    require_size_within_limit(&descriptor_after, max_bytes)?;
    let descriptor_after_stamp = FileStamp::from_metadata(&descriptor_after);
    if descriptor_after_stamp != opened_stamp {
        return Err(invalid_data("artifact identity changed while reading"));
    }

    let path_after = fs::symlink_metadata(&path)?;
    require_regular_nonsymlink(&path_after)?;
    require_size_within_limit(&path_after, max_bytes)?;
    if FileStamp::from_metadata(&path_after) != descriptor_after_stamp {
        return Err(invalid_data("artifact identity changed while reading"));
    }
    require_directory_metadata_stable(parent, parent_token)?;

    Ok(BoundedArtifactRead {
        bytes,
        identity: ArtifactIdentity(descriptor_after_stamp),
    })
}

/// Reads identity-stable UTF-8 text without exposing invalid bytes in the
/// resulting error.
pub(crate) fn read_bounded_utf8_regular_file(path: &Path, max_bytes: u64) -> io::Result<String> {
    let read = read_bounded_regular_file_with_identity(path, max_bytes)?;
    String::from_utf8(read.bytes).map_err(|_| invalid_data("artifact must contain valid UTF-8"))
}

/// Resolves a durable artifact path to one canonical parent directory.
///
/// Every ancestor observed during resolution must be a real directory rather
/// than a path redirect, apart from the exact macOS `/var` and `/tmp` platform
/// aliases. This returns a normalized name, not a directory capability;
/// readers and writers must capture and recheck parent metadata around every
/// later name-based operation.
pub(crate) fn prepare_durable_artifact_path(path: &Path) -> io::Result<PathBuf> {
    resolve_artifact_path(path, true)
}

/// Installs one bounded regular file after comparing its current identity with
/// the identity observed before serialization. Existing-target replacement
/// uses `std::fs::rename`, whose supported Unix and Windows implementations
/// replace the destination name without a destructive remove-first window.
/// Parent metadata is checked around staging and immediately before install,
/// but `std` has no portable directory-handle-relative rename primitive, so a
/// final swap wholly inside the last check-to-rename window cannot be excluded.
pub(crate) fn write_atomic_regular_file(
    path: &Path,
    bytes: &[u8],
    max_bytes: u64,
    expected_identity: Option<ArtifactIdentity>,
) -> io::Result<ArtifactWriteOutcome> {
    write_atomic_regular_file_impl(path, bytes, max_bytes, expected_identity, |_| {}, |_| {})
}

fn write_atomic_regular_file_impl<AfterParentPinned, BeforeInstall>(
    path: &Path,
    bytes: &[u8],
    max_bytes: u64,
    expected_identity: Option<ArtifactIdentity>,
    after_parent_pinned: AfterParentPinned,
    before_install: BeforeInstall,
) -> io::Result<ArtifactWriteOutcome>
where
    AfterParentPinned: FnOnce(&Path),
    BeforeInstall: FnOnce(&Path),
{
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > max_bytes {
        return Err(invalid_data("artifact exceeds configured byte limit"));
    }

    let target = prepare_durable_artifact_path(path)?;
    let canonical_parent = artifact_parent(&target)?.to_owned();
    let parent_token = observed_directory_metadata_token(&canonical_parent)?;
    require_private_artifact_parent(&canonical_parent)?;
    let directory_sync_handle = open_directory_sync_handle(&canonical_parent, parent_token)?;
    after_parent_pinned(&canonical_parent);
    require_directory_metadata_stable(&canonical_parent, parent_token)?;

    let target_before = optional_regular_identity(&target)?;
    require_directory_metadata_stable(&canonical_parent, parent_token)?;
    if target_before != expected_identity {
        return Err(invalid_data(
            "durable artifact target changed since the last accepted read",
        ));
    }
    let mut pending = create_pending_temp(&canonical_parent, parent_token)?;
    if let Err(error) = require_directory_metadata_stable(&canonical_parent, parent_token) {
        pending.scrub_and_abandon();
        return Err(error);
    }
    pending.file_mut().write_all(bytes)?;
    pending.file_mut().flush()?;
    pending.file_mut().sync_all()?;
    before_install(&canonical_parent);
    if let Err(error) = require_directory_metadata_stable(&canonical_parent, parent_token) {
        pending.scrub_and_abandon();
        return Err(error);
    }

    let current_result = optional_regular_identity(&target);
    if let Err(error) = require_directory_metadata_stable(&canonical_parent, parent_token) {
        pending.scrub_and_abandon();
        return Err(error);
    }
    let current = current_result?;
    let identity_unchanged = match (target_before, current) {
        (None, None) => true,
        (Some(before), Some(after)) => before == after,
        _ => false,
    };
    if !identity_unchanged {
        return Err(invalid_data(
            "durable artifact target changed before atomic replace",
        ));
    }
    if let Err(error) = require_directory_metadata_stable(&canonical_parent, parent_token) {
        pending.scrub_and_abandon();
        return Err(error);
    }

    if let Err(rename_error) = fs::rename(pending.path(), &target) {
        if let Err(parent_error) =
            require_directory_metadata_stable(&canonical_parent, parent_token)
        {
            pending.scrub_and_abandon();
            return Err(parent_error);
        }
        return Err(rename_error);
    }
    pending.mark_installed(&target);

    let installed_identity = match validate_installed_artifact(
        &canonical_parent,
        parent_token,
        &target,
        bytes,
        &pending,
    ) {
        Ok(identity) => identity,
        Err(_) => {
            return Ok(ArtifactWriteOutcome::CommitStateUncertain {
                installed_identity: None,
            });
        }
    };
    // Once the installed name and still-open handle identify the same file,
    // a later directory-sync error cannot safely remove the replacement: the
    // predecessor is already gone. Disarm before that durability boundary and
    // keep the new identity so callers do not roll their memory state back.
    pending.disarm();
    if artifact_io_failpoint(ArtifactIoFailpoint::BeforeDirectorySync).is_err()
        || sync_directory(directory_sync_handle.as_ref()).is_err()
    {
        return Ok(ArtifactWriteOutcome::CommitStateUncertain {
            installed_identity: Some(installed_identity),
        });
    }
    let final_identity =
        match validate_final_artifact(&canonical_parent, parent_token, &target, installed_identity)
        {
            Ok(identity) => identity,
            Err(_) => {
                return Ok(ArtifactWriteOutcome::CommitStateUncertain {
                    installed_identity: None,
                });
            }
        };
    Ok(ArtifactWriteOutcome::Durable(final_identity))
}

fn validate_installed_artifact(
    parent: &Path,
    parent_token: DirectoryMetadataToken,
    target: &Path,
    bytes: &[u8],
    pending: &PendingTemp,
) -> io::Result<ArtifactIdentity> {
    artifact_io_failpoint(ArtifactIoFailpoint::AfterRenameBeforeValidation)?;
    require_directory_metadata_stable(parent, parent_token)?;
    let after_result = fs::symlink_metadata(target);
    require_directory_metadata_stable(parent, parent_token)?;
    let after = after_result?;
    require_regular_nonsymlink(&after)?;
    require_private_artifact_file(&after)?;
    if after.len() != u64::try_from(bytes.len()).unwrap_or(u64::MAX) {
        return Err(invalid_data(
            "durable artifact length changed after atomic replace",
        ));
    }
    if !pending.open_file_matches(&after)? {
        return Err(invalid_data(
            "installed durable artifact does not match the staged file",
        ));
    }
    Ok(ArtifactIdentity(FileStamp::from_metadata(&after)))
}

fn validate_final_artifact(
    parent: &Path,
    parent_token: DirectoryMetadataToken,
    target: &Path,
    installed_identity: ArtifactIdentity,
) -> io::Result<ArtifactIdentity> {
    require_directory_metadata_stable(parent, parent_token)?;
    let final_metadata = fs::symlink_metadata(target)?;
    require_regular_nonsymlink(&final_metadata)?;
    require_private_artifact_file(&final_metadata)?;
    let final_identity = ArtifactIdentity(FileStamp::from_metadata(&final_metadata));
    if final_identity != installed_identity {
        return Err(invalid_data(
            "durable artifact changed after atomic replacement",
        ));
    }
    require_directory_metadata_stable(parent, parent_token)?;
    Ok(final_identity)
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

fn artifact_parent(path: &Path) -> io::Result<&Path> {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| invalid_input("durable artifact path has no parent"))
}

#[cfg(any(unix, windows))]
fn observed_directory_metadata_token(path: &Path) -> io::Result<DirectoryMetadataToken> {
    let metadata = fs::symlink_metadata(path)?;
    require_direct_directory(&metadata)?;
    Ok(DirectoryMetadataToken(DirectoryStamp::from_metadata(
        &metadata,
    )))
}

#[cfg(not(any(unix, windows)))]
fn observed_directory_metadata_token(_path: &Path) -> io::Result<DirectoryMetadataToken> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "stable directory metadata is unavailable on this platform",
    ))
}

fn require_directory_metadata_stable(
    path: &Path,
    expected: DirectoryMetadataToken,
) -> io::Result<()> {
    let observed = observed_directory_metadata_token(path)?;
    if observed != expected {
        return Err(invalid_data(
            "durable artifact parent metadata changed during filesystem access",
        ));
    }
    Ok(())
}

fn require_direct_directory(metadata: &Metadata) -> io::Result<()> {
    if metadata_is_redirect(metadata) || !metadata.is_dir() {
        return Err(invalid_data(
            "durable artifact parent must be a direct directory",
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn require_private_artifact_parent(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::symlink_metadata(path)?;
    require_direct_directory(&metadata)?;
    if metadata.permissions().mode() & 0o022 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "private durable artifact parent is writable outside its owner",
        ));
    }
    Ok(())
}

#[cfg(not(unix))]
fn require_private_artifact_parent(_path: &Path) -> io::Result<()> {
    Ok(())
}

#[cfg(unix)]
fn require_private_artifact_file(metadata: &Metadata) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    if metadata.permissions().mode() & 0o077 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "private durable artifact file permissions are too broad",
        ));
    }
    Ok(())
}

#[cfg(not(unix))]
fn require_private_artifact_file(_metadata: &Metadata) -> io::Result<()> {
    Ok(())
}

fn resolve_artifact_path(path: &Path, create_missing_parent: bool) -> io::Result<PathBuf> {
    let file_name = path
        .file_name()
        .filter(|name| !name.is_empty())
        .ok_or_else(|| invalid_input("durable artifact path has no file name"))?;
    let parent = normalized_parent(path);
    let absolute_parent = if parent.is_absolute() {
        parent.to_owned()
    } else {
        std::env::current_dir()?.join(parent)
    };
    let canonical_parent =
        resolve_directory_without_untrusted_links(&absolute_parent, create_missing_parent)?;
    Ok(canonical_parent.join(file_name))
}

fn resolve_directory_without_untrusted_links(
    directory: &Path,
    create_missing: bool,
) -> io::Result<PathBuf> {
    if directory
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(invalid_input(
            "durable artifact paths must not contain parent traversal",
        ));
    }

    let mut resolved = PathBuf::new();
    let mut normal_component_depth = 0_usize;

    for component in directory.components() {
        match component {
            Component::Prefix(prefix) => resolved.push(prefix.as_os_str()),
            Component::RootDir => resolved.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(invalid_input(
                    "durable artifact paths must not contain parent traversal",
                ));
            }
            Component::Normal(segment) => {
                resolved.push(segment);
                match fs::symlink_metadata(&resolved) {
                    Ok(metadata) => {
                        if metadata_is_redirect(&metadata) {
                            if !allow_trusted_platform_root_alias(
                                &resolved,
                                &metadata,
                                normal_component_depth,
                            ) {
                                return Err(invalid_data(
                                    "durable artifact ancestors must not be path redirects",
                                ));
                            }
                            let followed = fs::metadata(&resolved)?;
                            if !followed.is_dir() {
                                return Err(invalid_data(
                                    "durable artifact ancestor must be a directory",
                                ));
                            }
                        } else if !metadata.is_dir() {
                            return Err(invalid_data(
                                "durable artifact ancestor must be a directory",
                            ));
                        }
                    }
                    Err(error) if error.kind() == io::ErrorKind::NotFound && create_missing => {
                        let parent = resolved.parent().ok_or_else(|| {
                            invalid_input("durable artifact directory has no parent")
                        })?;
                        let canonical_parent = fs::canonicalize(parent)?;
                        let parent_token = observed_directory_metadata_token(&canonical_parent)?;
                        create_private_directory(&resolved)?;
                        require_directory_metadata_stable(&canonical_parent, parent_token)?;
                        let metadata = fs::symlink_metadata(&resolved)?;
                        if metadata_is_redirect(&metadata) || !metadata.is_dir() {
                            return Err(invalid_data(
                                "created artifact ancestor is not a direct directory",
                            ));
                        }
                    }
                    Err(error) => return Err(error),
                }
                normal_component_depth = normal_component_depth.saturating_add(1);
            }
        }
    }

    let canonical = fs::canonicalize(&resolved)?;
    let metadata = fs::symlink_metadata(&canonical)?;
    require_direct_directory(&metadata)?;
    Ok(canonical)
}

#[cfg(target_os = "macos")]
fn allow_trusted_platform_root_alias(
    path: &Path,
    metadata: &Metadata,
    normal_component_depth: usize,
) -> bool {
    use std::os::unix::fs::MetadataExt;

    // Permit only the two macOS aliases required by standard temporary and
    // state paths. Exact canonical targets prevent this exception from
    // becoming a generic root-owned-first-hop symlink policy.
    if normal_component_depth != 0 || !metadata.file_type().is_symlink() || metadata.uid() != 0 {
        return false;
    }
    let approved_target = if path == Path::new("/var") {
        Path::new("/private/var")
    } else if path == Path::new("/tmp") {
        Path::new("/private/tmp")
    } else {
        return false;
    };
    let Some(parent) = path.parent() else {
        return false;
    };
    let Ok(parent_metadata) = fs::symlink_metadata(parent) else {
        return false;
    };
    parent_metadata.is_dir()
        && parent_metadata.uid() == 0
        && parent_metadata.mode() & 0o022 == 0
        && fs::canonicalize(path).is_ok_and(|canonical| canonical == approved_target)
}

#[cfg(not(target_os = "macos"))]
fn allow_trusted_platform_root_alias(
    _path: &Path,
    _metadata: &Metadata,
    _normal_component_depth: usize,
) -> bool {
    false
}

fn metadata_is_redirect(metadata: &Metadata) -> bool {
    metadata.file_type().is_symlink() || metadata_is_platform_redirect(metadata)
}

#[cfg(windows)]
fn metadata_is_platform_redirect(metadata: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn metadata_is_platform_redirect(_metadata: &Metadata) -> bool {
    false
}

fn require_regular_nonsymlink(metadata: &Metadata) -> io::Result<()> {
    if metadata_is_redirect(metadata) {
        return Err(invalid_data("artifact path must not be a path redirect"));
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

fn optional_regular_identity(path: &Path) -> io::Result<Option<ArtifactIdentity>> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            require_regular_nonsymlink(&metadata)?;
            Ok(Some(ArtifactIdentity(FileStamp::from_metadata(&metadata))))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

#[cfg(unix)]
fn create_private_directory(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;

    let mut builder = fs::DirBuilder::new();
    builder.mode(0o700);
    builder.create(path)
}

#[cfg(not(unix))]
fn create_private_directory(path: &Path) -> io::Result<()> {
    fs::DirBuilder::new().create(path)
}

fn create_pending_temp(
    parent: &Path,
    parent_token: DirectoryMetadataToken,
) -> io::Result<PendingTemp> {
    for _ in 0..32 {
        let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let name = format!(".aureline-artifact-tmp-{}-{sequence}", std::process::id());
        let path = parent.join(name);
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => {
                let mut pending = PendingTemp {
                    file: Some(file),
                    path,
                    parent: parent.to_owned(),
                    parent_token,
                    armed: true,
                };
                restrict_new_file_permissions(pending.file_mut())?;
                return Ok(pending);
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
fn open_directory_sync_handle(
    path: &Path,
    expected: DirectoryMetadataToken,
) -> io::Result<Option<File>> {
    let directory = File::open(path)?;
    let metadata = directory.metadata()?;
    require_direct_directory(&metadata)?;
    if DirectoryMetadataToken(DirectoryStamp::from_metadata(&metadata)) != expected {
        return Err(invalid_data(
            "durable artifact parent metadata changed while opening",
        ));
    }
    Ok(Some(directory))
}

#[cfg(not(unix))]
fn open_directory_sync_handle(
    _path: &Path,
    _expected: DirectoryMetadataToken,
) -> io::Result<Option<File>> {
    Ok(None)
}

#[cfg(unix)]
fn sync_directory(directory: Option<&File>) -> io::Result<()> {
    directory
        .ok_or_else(|| invalid_data("durable artifact directory handle is unavailable"))?
        .sync_all()
}

#[cfg(not(unix))]
fn sync_directory(_directory: Option<&File>) -> io::Result<()> {
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
    parent: PathBuf,
    parent_token: DirectoryMetadataToken,
    armed: bool,
}

impl PendingTemp {
    fn file_mut(&mut self) -> &mut File {
        self.file
            .as_mut()
            .expect("pending temporary file remains open until close")
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn mark_installed(&mut self, target: &Path) {
        self.path = target.to_owned();
    }

    fn open_file_matches(&self, metadata: &Metadata) -> io::Result<bool> {
        let handle = self
            .file
            .as_ref()
            .ok_or_else(|| invalid_data("pending durable artifact handle is unavailable"))?;
        let handle_metadata = handle.metadata()?;
        require_regular_nonsymlink(&handle_metadata)?;
        Ok(FileStamp::from_metadata(&handle_metadata) == FileStamp::from_metadata(metadata))
    }

    fn disarm(&mut self) {
        self.armed = false;
    }

    fn scrub_and_abandon(&mut self) {
        self.scrub_open_handle();
        self.armed = false;
    }

    fn scrub_open_handle(&mut self) {
        if let Some(file) = self.file.as_mut() {
            let _ = file.set_len(0);
            let _ = file.sync_all();
        }
    }

    #[cfg(unix)]
    fn cleanup_path_if_still_owned(&self) {
        if require_directory_metadata_stable(&self.parent, self.parent_token).is_err() {
            return;
        }
        let Some(file) = self.file.as_ref() else {
            return;
        };
        let Ok(handle_metadata) = file.metadata() else {
            return;
        };
        let Ok(path_metadata) = fs::symlink_metadata(&self.path) else {
            return;
        };
        if require_regular_nonsymlink(&handle_metadata).is_err()
            || require_regular_nonsymlink(&path_metadata).is_err()
            || FileStamp::from_metadata(&handle_metadata)
                != FileStamp::from_metadata(&path_metadata)
        {
            return;
        }
        let _ = fs::remove_file(&self.path);
    }

    #[cfg(not(unix))]
    fn cleanup_path_if_still_owned(&self) {
        // The supported Rust standard library does not expose a unique
        // Windows file ID. Scrub through the open handle, but do not authorize
        // pathname deletion from creation-time/attribute metadata alone.
    }
}

impl Drop for PendingTemp {
    fn drop(&mut self) {
        if self.armed {
            self.scrub_open_handle();
            self.cleanup_path_if_still_owned();
        }
        self.file.take();
    }
}

struct BoundedVecWriter {
    bytes: Vec<u8>,
    max_bytes: usize,
}

/// Opaque filesystem generation token retained by durable stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ArtifactIdentity(FileStamp);

/// Last accepted generation for a durable store. `Indeterminate` is entered
/// only after rename when no installed identity can be proven; another write
/// is blocked until a fresh store instance rereads the pathname.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArtifactGenerationExpectation {
    Known(Option<ArtifactIdentity>),
    Indeterminate,
}

/// Outcome of a durable artifact write. An ordinary `Err` is returned only
/// before the install commit point; callers must retain their intended memory
/// state when the commit or final directory durability cannot be proven.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArtifactWriteOutcome {
    /// The installed object was synchronized before rename and its final
    /// namespace identity was revalidated. Platforms with directory-handle
    /// sync support also synchronized the containing directory.
    Durable(ArtifactIdentity),
    /// Rename succeeded, but a later check could not prove the final durable
    /// state. A known identity remains safe for compare-before-write when
    /// present; otherwise the store must be reopened before another mutation.
    CommitStateUncertain {
        installed_identity: Option<ArtifactIdentity>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArtifactIoFailpoint {
    AfterRenameBeforeValidation,
    BeforeDirectorySync,
}

#[cfg(test)]
thread_local! {
    static ARTIFACT_IO_FAILPOINT: Cell<Option<ArtifactIoFailpoint>> = Cell::new(None);
}

#[cfg(test)]
pub(crate) struct ArtifactIoFailpointGuard;

#[cfg(test)]
impl Drop for ArtifactIoFailpointGuard {
    fn drop(&mut self) {
        ARTIFACT_IO_FAILPOINT.with(|configured| configured.set(None));
    }
}

#[cfg(test)]
pub(crate) fn inject_artifact_io_failure(
    failpoint: ArtifactIoFailpoint,
) -> ArtifactIoFailpointGuard {
    ARTIFACT_IO_FAILPOINT.with(|configured| configured.set(Some(failpoint)));
    ArtifactIoFailpointGuard
}

#[cfg(test)]
fn artifact_io_failpoint(failpoint: ArtifactIoFailpoint) -> io::Result<()> {
    let fires = ARTIFACT_IO_FAILPOINT.with(|configured| configured.get() == Some(failpoint));
    if fires {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "synthetic durable artifact failure",
        ));
    }
    Ok(())
}

#[cfg(not(test))]
fn artifact_io_failpoint(_failpoint: ArtifactIoFailpoint) -> io::Result<()> {
    Ok(())
}

/// A best-effort parent-directory metadata stability token. On Windows the
/// Rust 1.75 standard library does not expose a unique directory object ID, so
/// this must not be treated as one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectoryMetadataToken(DirectoryStamp);

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
struct DirectoryStamp {
    device: u64,
    inode: u64,
    mode: u32,
}

#[cfg(unix)]
impl DirectoryStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::unix::fs::MetadataExt;

        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
            mode: metadata.mode(),
        }
    }
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectoryStamp {
    file_attributes: u32,
    creation_time: u64,
}

#[cfg(windows)]
impl DirectoryStamp {
    fn from_metadata(metadata: &Metadata) -> Self {
        use std::os::windows::fs::MetadataExt;

        Self {
            file_attributes: metadata.file_attributes(),
            creation_time: metadata.creation_time(),
        }
    }
}

#[cfg(not(any(unix, windows)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectoryStamp;

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

    #[test]
    fn bounded_read_checks_the_open_descriptor_after_reading() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("rows.json");
        fs::write(&path, b"1234").expect("write fixture");

        let error = read_bounded_regular_file_with_identity_impl(
            &path,
            1024,
            |_| {},
            |resolved| {
                fs::write(resolved, b"x").expect("mutate open file through a second handle");
            },
        )
        .expect_err("descriptor mutation must invalidate the read");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(error.to_string(), "artifact identity changed while reading");
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

    #[cfg(unix)]
    #[test]
    fn bounded_io_rejects_symlinked_ancestors_without_creating_outside() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir");
        let outside = directory.path().join("outside");
        let alias = directory.path().join("state");
        fs::create_dir(&outside).expect("create outside directory");
        fs::write(outside.join("rows.json"), b"[]").expect("write outside file");
        symlink(&outside, &alias).expect("create ancestor symlink");

        let read_error = read_bounded_regular_file(&alias.join("rows.json"), 1024)
            .expect_err("read through an ancestor symlink must fail");
        assert_eq!(read_error.kind(), io::ErrorKind::InvalidData);

        let write_error =
            write_atomic_regular_file(&alias.join("nested").join("rows.json"), b"[1]", 1024, None)
                .expect_err("write through an ancestor symlink must fail");
        assert_eq!(write_error.kind(), io::ErrorKind::InvalidData);
        assert!(!outside.join("nested").exists());
        assert_eq!(fs::read(outside.join("rows.json")).expect("target"), b"[]");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn platform_root_aliases_are_limited_to_exact_macos_pairs() {
        for (alias, target) in [("/var", "/private/var"), ("/tmp", "/private/tmp")] {
            let alias = Path::new(alias);
            let metadata = fs::symlink_metadata(alias).expect("macOS platform alias metadata");
            assert!(allow_trusted_platform_root_alias(alias, &metadata, 0));
            assert_eq!(
                fs::canonicalize(alias).expect("canonical macOS platform alias"),
                Path::new(target)
            );
        }

        let metadata = fs::symlink_metadata("/etc").expect("macOS /etc metadata");
        assert!(!allow_trusted_platform_root_alias(
            Path::new("/etc"),
            &metadata,
            0
        ));
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    #[test]
    fn non_macos_unix_never_allows_platform_root_aliases() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir");
        let target = directory.path().join("target");
        let alias = directory.path().join("alias");
        fs::create_dir(&target).expect("create target");
        symlink(&target, &alias).expect("create alias");
        let metadata = fs::symlink_metadata(&alias).expect("alias metadata");

        assert!(!allow_trusted_platform_root_alias(&alias, &metadata, 0));
    }

    #[cfg(unix)]
    #[test]
    fn bounded_read_rejects_parent_swap_after_identity_is_pinned() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir");
        let state = directory.path().join("state");
        let outside = directory.path().join("outside");
        fs::create_dir(&state).expect("create state");
        fs::create_dir(&outside).expect("create outside");
        fs::write(state.join("rows.json"), b"accepted").expect("write accepted file");
        fs::write(outside.join("rows.json"), b"private!").expect("write outside file");

        let error = read_bounded_regular_file_with_identity_impl(
            &state.join("rows.json"),
            1024,
            |parent| {
                fs::rename(parent, parent.with_file_name("state-moved")).expect("move parent");
                symlink(&outside, parent).expect("replace parent with redirect");
            },
            |_| {},
        )
        .expect_err("parent replacement must invalidate the read");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(
            fs::read(outside.join("rows.json")).expect("outside file"),
            b"private!"
        );
    }

    #[cfg(unix)]
    #[test]
    fn atomic_write_rejects_parent_swap_before_temp_creation() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir");
        let state = directory.path().join("state");
        let outside = directory.path().join("outside");
        fs::create_dir(&state).expect("create state");
        fs::create_dir(&outside).expect("create outside");

        let error = write_atomic_regular_file_impl(
            &state.join("rows.json"),
            b"sensitive",
            1024,
            None,
            |parent| {
                fs::rename(parent, parent.with_file_name("state-moved")).expect("move parent");
                symlink(&outside, parent).expect("replace parent with redirect");
            },
            |_| {},
        )
        .expect_err("parent replacement must block staging");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(fs::read_dir(&outside)
            .expect("outside directory")
            .next()
            .is_none());
    }

    #[cfg(unix)]
    #[test]
    fn atomic_write_scrubs_stage_when_parent_swaps_before_install() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("tempdir");
        let state = directory.path().join("state");
        let moved = directory.path().join("state-moved");
        let outside = directory.path().join("outside");
        fs::create_dir(&state).expect("create state");
        fs::create_dir(&outside).expect("create outside");

        let error = write_atomic_regular_file_impl(
            &state.join("rows.json"),
            b"sensitive",
            1024,
            None,
            |_| {},
            |parent| {
                fs::rename(parent, parent.with_file_name("state-moved")).expect("move parent");
                symlink(&outside, parent).expect("replace parent with redirect");
            },
        )
        .expect_err("parent replacement must block install");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(!outside.join("rows.json").exists());
        assert!(fs::read_dir(&outside)
            .expect("outside directory")
            .next()
            .is_none());
        let staged = fs::read_dir(&moved)
            .expect("moved state directory")
            .map(|entry| entry.expect("staged entry"))
            .find(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(".aureline-artifact-tmp-")
            })
            .expect("scrubbed staged file remains in moved directory");
        assert_eq!(staged.metadata().expect("staged metadata").len(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn pending_drop_scrubs_moved_inode_without_deleting_replacement_path() {
        let directory = tempfile::tempdir().expect("tempdir");
        let parent = directory.path().join("state");
        let moved_parent = directory.path().join("state-moved");
        let outside = directory.path().join("outside");
        fs::create_dir(&parent).expect("create state");
        fs::create_dir(&outside).expect("create outside");
        let parent_token = observed_directory_metadata_token(&parent).expect("parent token");
        let mut pending = create_pending_temp(&parent, parent_token).expect("pending temp");
        pending
            .file_mut()
            .write_all(b"private-staged-payload")
            .expect("write staged payload");
        pending.file_mut().sync_all().expect("sync staged payload");
        let pending_name = pending
            .path()
            .file_name()
            .expect("pending file name")
            .to_owned();

        fs::rename(&parent, &moved_parent).expect("move original parent");
        fs::create_dir(&parent).expect("create replacement parent");
        let replacement = parent.join(&pending_name);
        fs::write(&replacement, b"replacement-safe").expect("write replacement sentinel");
        let outside_sentinel = outside.join("outside-sentinel.txt");
        fs::write(&outside_sentinel, b"outside-safe").expect("write outside sentinel");

        drop(pending);

        assert_eq!(
            fs::read(moved_parent.join(&pending_name)).expect("read scrubbed moved inode"),
            b""
        );
        assert_eq!(
            fs::read(&replacement).expect("read replacement sentinel"),
            b"replacement-safe"
        );
        assert_eq!(
            fs::read(&outside_sentinel).expect("read outside sentinel"),
            b"outside-safe"
        );
    }

    #[cfg(unix)]
    #[test]
    fn newly_created_artifact_directories_are_private() {
        use std::os::unix::fs::PermissionsExt;

        let directory = tempfile::tempdir().expect("tempdir");
        let nested = directory.path().join("state").join("activity");
        let path = nested.join("rows.json");
        prepare_durable_artifact_path(&path).expect("prepare durable path");

        for created in [directory.path().join("state"), nested] {
            let mode = fs::symlink_metadata(created)
                .expect("created directory metadata")
                .permissions()
                .mode();
            assert_eq!(mode & 0o077, 0);
        }
    }

    #[cfg(unix)]
    #[test]
    fn durable_writer_rejects_parent_writable_outside_its_owner() {
        use std::os::unix::fs::PermissionsExt;

        let directory = tempfile::tempdir().expect("tempdir");
        let parent = directory.path().join("shared-state");
        fs::create_dir(&parent).expect("create shared parent");
        fs::set_permissions(&parent, fs::Permissions::from_mode(0o777))
            .expect("make parent broadly writable");
        let path = parent.join("rows.json");

        let error = write_atomic_regular_file(&path, b"private", 1024, None)
            .expect_err("broadly writable parent must fail closed");
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        assert!(!path.exists());
    }

    #[test]
    fn durable_artifact_paths_reject_parent_traversal() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("state").join("..").join("rows.json");

        let error = prepare_durable_artifact_path(&path).expect_err("traversal must fail");
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(!directory.path().join("state").exists());
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
    fn bounded_json_writer_emits_each_chunk_once() {
        let value = vec!["one", "two", "three"];
        let bytes = to_bounded_json_pretty(&value, 1024).expect("serialize bounded JSON");
        let decoded: Vec<String> = serde_json::from_slice(&bytes).expect("parse bounded JSON");
        assert_eq!(decoded, value);
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

    #[cfg(any(unix, windows))]
    #[test]
    fn atomic_write_replaces_an_existing_target_without_a_missing_name_window() {
        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("rows.json");
        let first =
            write_atomic_regular_file(&path, b"old", 1024, None).expect("write initial artifact");
        let first_identity = match first {
            ArtifactWriteOutcome::Durable(identity) => identity,
            ArtifactWriteOutcome::CommitStateUncertain { .. } => {
                panic!("initial artifact durability must be known")
            }
        };

        let second = write_atomic_regular_file(&path, b"new", 1024, Some(first_identity))
            .expect("replace existing artifact");
        assert!(matches!(second, ArtifactWriteOutcome::Durable(_)));
        assert_eq!(fs::read(&path).expect("read replacement"), b"new");
    }

    #[cfg(windows)]
    #[test]
    fn windows_sharing_violation_preserves_existing_target() {
        use std::os::windows::fs::OpenOptionsExt;

        let directory = tempfile::tempdir().expect("tempdir");
        let path = directory.path().join("rows.json");
        let first =
            write_atomic_regular_file(&path, b"old", 1024, None).expect("write initial artifact");
        let first_identity = match first {
            ArtifactWriteOutcome::Durable(identity) => identity,
            ArtifactWriteOutcome::CommitStateUncertain { .. } => {
                panic!("initial artifact durability must be known")
            }
        };
        let _exclusive = OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(&path)
            .expect("open destination without delete sharing");

        write_atomic_regular_file(&path, b"new", 1024, Some(first_identity))
            .expect_err("sharing violation must fail before rename commits");
        assert_eq!(fs::read(&path).expect("prior target remains"), b"old");
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
