// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Bounded, no-symlink workspace metadata reads shared by discovery lanes.

use std::fmt;
use std::fs::{File, Metadata};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoundedWorkspaceReadError {
    WorkspaceUnavailable,
    Symlink,
    NotRegular,
    OutsideWorkspace,
    TooLarge,
    ChangedDuringRead,
    ReadFailed,
    InvalidUtf8,
}

impl fmt::Display for BoundedWorkspaceReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::WorkspaceUnavailable => "workspace root is unavailable",
            Self::Symlink => "workspace metadata file is a symlink",
            Self::NotRegular => "workspace metadata path is not a regular file",
            Self::OutsideWorkspace => "workspace metadata file resolves outside the workspace",
            Self::TooLarge => "workspace metadata file exceeds the discovery byte limit",
            Self::ChangedDuringRead => "workspace metadata file changed during discovery",
            Self::ReadFailed => "workspace metadata file could not be read safely",
            Self::InvalidUtf8 => "workspace metadata file is not valid UTF-8",
        })
    }
}

pub(crate) fn read_bounded_workspace_utf8(
    workspace_root: &Path,
    relative_path: &Path,
    max_bytes: u64,
) -> Result<Option<String>, BoundedWorkspaceReadError> {
    let Some(bytes) = read_bounded_workspace_bytes(workspace_root, relative_path, max_bytes)?
    else {
        return Ok(None);
    };
    String::from_utf8(bytes)
        .map(Some)
        .map_err(|_| BoundedWorkspaceReadError::InvalidUtf8)
}

/// Reads exact workspace-relative bytes while rejecting escape, symlink,
/// non-regular, oversized, and identity-changing inputs.
pub(crate) fn read_bounded_workspace_bytes(
    workspace_root: &Path,
    relative_path: &Path,
    max_bytes: u64,
) -> Result<Option<Vec<u8>>, BoundedWorkspaceReadError> {
    if relative_path.is_absolute()
        || relative_path
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return Err(BoundedWorkspaceReadError::OutsideWorkspace);
    }
    let canonical_root = workspace_root
        .canonicalize()
        .map_err(|_| BoundedWorkspaceReadError::WorkspaceUnavailable)?;
    if !canonical_root.is_dir() {
        return Err(BoundedWorkspaceReadError::WorkspaceUnavailable);
    }
    let path = canonical_root.join(relative_path);
    if path_has_symlink_component(&canonical_root, relative_path) {
        return Err(BoundedWorkspaceReadError::Symlink);
    }
    let before = match std::fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(BoundedWorkspaceReadError::ReadFailed),
    };
    if metadata_is_redirect(&before) {
        return Err(BoundedWorkspaceReadError::Symlink);
    }
    if !before.is_file() {
        return Err(BoundedWorkspaceReadError::NotRegular);
    }
    if before.len() > max_bytes {
        return Err(BoundedWorkspaceReadError::TooLarge);
    }
    let canonical_path = path
        .canonicalize()
        .map_err(|_| BoundedWorkspaceReadError::ReadFailed)?;
    if !canonical_path.starts_with(&canonical_root) {
        return Err(BoundedWorkspaceReadError::OutsideWorkspace);
    }

    let mut file = File::open(&path).map_err(|_| BoundedWorkspaceReadError::ReadFailed)?;
    let opened = file
        .metadata()
        .map_err(|_| BoundedWorkspaceReadError::ReadFailed)?;
    if !opened.is_file() || opened.len() > max_bytes || !same_file_identity(&before, &opened) {
        return Err(BoundedWorkspaceReadError::ChangedDuringRead);
    }

    let capacity =
        usize::try_from(opened.len()).map_err(|_| BoundedWorkspaceReadError::TooLarge)?;
    let mut bytes = Vec::with_capacity(capacity);
    Read::by_ref(&mut file)
        .take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|_| BoundedWorkspaceReadError::ReadFailed)?;
    if bytes.len() as u64 > max_bytes {
        return Err(BoundedWorkspaceReadError::TooLarge);
    }

    let descriptor_after = file
        .metadata()
        .map_err(|_| BoundedWorkspaceReadError::ReadFailed)?;
    let path_after = std::fs::symlink_metadata(&path)
        .map_err(|_| BoundedWorkspaceReadError::ChangedDuringRead)?;
    let canonical_after = path
        .canonicalize()
        .map_err(|_| BoundedWorkspaceReadError::ChangedDuringRead)?;
    if path_has_symlink_component(&canonical_root, relative_path)
        || metadata_is_redirect(&path_after)
        || !path_after.is_file()
        || !canonical_after.starts_with(&canonical_root)
        || !same_file_identity(&opened, &descriptor_after)
        || !same_file_identity(&opened, &path_after)
    {
        return Err(BoundedWorkspaceReadError::ChangedDuringRead);
    }

    Ok(Some(bytes))
}

pub(crate) fn workspace_regular_file_exists(workspace_root: &Path, relative_path: &Path) -> bool {
    let Some((canonical_root, path)) = contained_workspace_path(workspace_root, relative_path)
    else {
        return false;
    };
    let Ok(before) = std::fs::symlink_metadata(&path) else {
        return false;
    };
    if path_has_symlink_component(&canonical_root, relative_path)
        || metadata_is_redirect(&before)
        || !before.is_file()
    {
        return false;
    }
    let Ok(canonical_path) = path.canonicalize() else {
        return false;
    };
    if !canonical_path.starts_with(&canonical_root) {
        return false;
    }
    let Ok(file) = File::open(&path) else {
        return false;
    };
    let Ok(opened) = file.metadata() else {
        return false;
    };
    let Ok(after) = std::fs::symlink_metadata(&path) else {
        return false;
    };
    let Ok(canonical_after) = path.canonicalize() else {
        return false;
    };
    !path_has_symlink_component(&canonical_root, relative_path)
        && opened.is_file()
        && after.is_file()
        && !metadata_is_redirect(&after)
        && canonical_after.starts_with(&canonical_root)
        && same_file_identity(&before, &opened)
        && same_file_identity(&opened, &after)
}

pub(crate) fn workspace_path_is_present(workspace_root: &Path, relative_path: &Path) -> bool {
    let Some((_, path)) = contained_workspace_path(workspace_root, relative_path) else {
        return false;
    };
    std::fs::symlink_metadata(path).is_ok()
}

pub(crate) fn workspace_regular_directory_exists(
    workspace_root: &Path,
    relative_path: &Path,
) -> bool {
    let Some((canonical_root, path)) = contained_workspace_path(workspace_root, relative_path)
    else {
        return false;
    };
    let Ok(before) = std::fs::symlink_metadata(&path) else {
        return false;
    };
    if path_has_symlink_component(&canonical_root, relative_path)
        || metadata_is_redirect(&before)
        || !before.is_dir()
    {
        return false;
    }
    let Ok(canonical_path) = path.canonicalize() else {
        return false;
    };
    if !canonical_path.starts_with(&canonical_root) {
        return false;
    }
    let Ok(after) = std::fs::symlink_metadata(&path) else {
        return false;
    };
    !path_has_symlink_component(&canonical_root, relative_path)
        && after.is_dir()
        && !metadata_is_redirect(&after)
        && same_object_identity(&before, &after)
}

fn contained_workspace_path(
    workspace_root: &Path,
    relative_path: &Path,
) -> Option<(std::path::PathBuf, std::path::PathBuf)> {
    if relative_path.is_absolute()
        || relative_path
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return None;
    }
    let canonical_root = workspace_root.canonicalize().ok()?;
    canonical_root
        .is_dir()
        .then(|| (canonical_root.clone(), canonical_root.join(relative_path)))
}

fn path_has_symlink_component(canonical_root: &Path, relative_path: &Path) -> bool {
    let mut current = PathBuf::from(canonical_root);
    for component in relative_path.components() {
        let std::path::Component::Normal(component) = component else {
            return true;
        };
        current.push(component);
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) if metadata_is_redirect(&metadata) => return true,
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return false,
            Err(_) => return true,
        }
    }
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

pub(crate) fn is_safe_reference_text(value: &str) -> bool {
    value.chars().all(|character| {
        !character.is_control()
            && !matches!(
                character,
                '\u{061c}'
                    | '\u{200b}'
                    | '\u{200e}'
                    | '\u{200f}'
                    | '\u{202a}'..='\u{202e}'
                    | '\u{2060}'
                    | '\u{2066}'..='\u{2069}'
                    | '\u{feff}'
            )
    })
}

#[cfg(unix)]
fn same_file_identity(left: &Metadata, right: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    left.dev() == right.dev()
        && left.ino() == right.ino()
        && left.len() == right.len()
        && left.mode() == right.mode()
        && left.mtime() == right.mtime()
        && left.mtime_nsec() == right.mtime_nsec()
        && left.ctime() == right.ctime()
        && left.ctime_nsec() == right.ctime_nsec()
}

#[cfg(unix)]
fn same_object_identity(left: &Metadata, right: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    left.dev() == right.dev() && left.ino() == right.ino() && left.mode() == right.mode()
}

#[cfg(not(unix))]
fn same_file_identity(left: &Metadata, right: &Metadata) -> bool {
    left.len() == right.len()
        && left.file_type() == right.file_type()
        && left.created().ok() == right.created().ok()
        && left.modified().ok() == right.modified().ok()
}

#[cfg(not(unix))]
fn same_object_identity(left: &Metadata, right: &Metadata) -> bool {
    left.file_type() == right.file_type()
        && left.created().ok() == right.created().ok()
        && left.modified().ok() == right.modified().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_text_rejects_controls_and_directional_formatting() {
        assert!(is_safe_reference_text("tests/test_valid.py"));
        assert!(!is_safe_reference_text("tests/test_spoof\n.py"));
        assert!(!is_safe_reference_text("tests/test_\u{202e}spoof.py"));
        assert!(!is_safe_reference_text("tests/test_\u{200b}spoof.py"));
    }

    #[test]
    fn byte_reads_preserve_non_utf8_content_exactly() {
        let workspace = TempWorkspace::new("raw-bytes");
        let expected = vec![0xff, 0x00, 0x80, b'a'];
        std::fs::write(workspace.path().join("metadata.bin"), &expected).expect("metadata");

        assert_eq!(
            read_bounded_workspace_bytes(workspace.path(), Path::new("metadata.bin"), 16),
            Ok(Some(expected))
        );
        assert_eq!(
            read_bounded_workspace_utf8(workspace.path(), Path::new("metadata.bin"), 16),
            Err(BoundedWorkspaceReadError::InvalidUtf8)
        );
    }

    #[test]
    fn byte_reads_enforce_the_declared_limit() {
        let workspace = TempWorkspace::new("byte-limit");
        std::fs::write(workspace.path().join("metadata.bin"), [0_u8; 17]).expect("metadata");

        assert_eq!(
            read_bounded_workspace_bytes(workspace.path(), Path::new("metadata.bin"), 16),
            Err(BoundedWorkspaceReadError::TooLarge)
        );
    }

    #[test]
    fn regular_files_are_not_classified_as_path_redirects() {
        let workspace = TempWorkspace::new("regular-file");
        let path = workspace.path().join("metadata.toml");
        std::fs::write(&path, "safe = true\n").expect("metadata");
        let metadata = std::fs::symlink_metadata(path).expect("metadata identity");

        assert!(!metadata_is_redirect(&metadata));
    }

    #[cfg(unix)]
    #[test]
    fn bounded_reads_reject_parent_directory_symlinks() {
        use std::os::unix::fs::symlink;

        let workspace = TempWorkspace::new("parent-symlink");
        std::fs::create_dir_all(workspace.path().join("real")).expect("real directory");
        std::fs::write(workspace.path().join("real/metadata.toml"), "safe = true\n")
            .expect("metadata");
        symlink("real", workspace.path().join("linked")).expect("parent symlink");

        assert_eq!(
            read_bounded_workspace_utf8(workspace.path(), Path::new("linked/metadata.toml"), 1_024,),
            Err(BoundedWorkspaceReadError::Symlink)
        );
        assert!(!workspace_regular_file_exists(
            workspace.path(),
            Path::new("linked/metadata.toml")
        ));
        assert!(!workspace_regular_directory_exists(
            workspace.path(),
            Path::new("linked")
        ));
    }

    struct TempWorkspace {
        path: PathBuf,
    }

    impl TempWorkspace {
        fn new(label: &str) -> Self {
            let unique = format!(
                "aureline-bounded-file-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("clock after epoch")
                    .as_nanos()
            );
            let path = std::env::temp_dir().join(unique);
            let _ = std::fs::remove_dir_all(&path);
            std::fs::create_dir_all(&path).expect("temp workspace");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempWorkspace {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}
