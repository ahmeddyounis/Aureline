// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Local filesystem root adapter.
//!
//! This root resolves `file://` presentation URIs into the VFS identity model.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::capabilities::{
    AtomicWriteMode, CapabilityFlags, CaseSensitivity, FallbackIdentityTokenKind,
    NormalizationForm, RootCapabilityEnvelope, RootClass, StrongestIdentityTokenKind,
    SymlinkEscapePolicy,
};
use crate::identity::{
    AliasSet, CanonicalFilesystemObject, FallbackIdentityToken, IdentityRecord, IdentityToken,
    LogicalWorkspaceIdentity, PresentationPath, TrustState,
};
use crate::save::{GenerationToken, GenerationTokenKind, PermissionSnapshot};
use crate::uri_model::VfsUri;

use super::{RootIoError, RootResolveError, VfsRoot};

/// Errors returned when constructing a [`LocalFilesystemRoot`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalFilesystemRootError {
    MountPathNotAbsolute(PathBuf),
    MountPathUnavailable { path: PathBuf, detail: String },
    MountPathNotDirectory(PathBuf),
}

impl std::fmt::Display for LocalFilesystemRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MountPathNotAbsolute(path) => write!(f, "mount path must be absolute: {path:?}"),
            Self::MountPathUnavailable { path, detail } => {
                write!(f, "mount path is unavailable: {path:?}: {detail}")
            }
            Self::MountPathNotDirectory(path) => {
                write!(f, "mount path must be a directory: {path:?}")
            }
        }
    }
}

impl std::error::Error for LocalFilesystemRootError {}

/// A root backed by the host filesystem.
#[derive(Debug, Clone)]
pub struct LocalFilesystemRoot {
    envelope: RootCapabilityEnvelope,
    workspace_id: String,
    root_badge: String,
    mount_path: PathBuf,
    trust_state: TrustState,
    policy_scope: Option<String>,
}

impl LocalFilesystemRoot {
    /// Creates a local filesystem root mounted at `mount_path`.
    pub fn new(
        workspace_id: impl Into<String>,
        root_id: impl Into<String>,
        mount_path: PathBuf,
    ) -> Result<Self, LocalFilesystemRootError> {
        if !mount_path.is_absolute() {
            return Err(LocalFilesystemRootError::MountPathNotAbsolute(mount_path));
        }
        let mount_path = mount_path.canonicalize().map_err(|err| {
            LocalFilesystemRootError::MountPathUnavailable {
                path: mount_path.clone(),
                detail: err.to_string(),
            }
        })?;
        if !mount_path.is_dir() {
            return Err(LocalFilesystemRootError::MountPathNotDirectory(mount_path));
        }

        let root_class = if cfg!(windows) {
            RootClass::LocalWindowsLike
        } else {
            RootClass::LocalPosixLike
        };

        let capability_flags = CapabilityFlags {
            supports_atomic_replace: true,
            supports_in_place_write: true,
            supports_conditional_remote_write: false,
            case_sensitivity: default_case_sensitivity(),
            unicode_normalization: NormalizationForm::MixedObserved,
            supports_case_only_rename: true,
            supports_unicode_normalization_rename: true,
            symlink_escape_policy: SymlinkEscapePolicy::Warn,
            read_only: false,
            policy_constrained: false,
            review_required_before_save: false,
            review_required_before_rename: false,
            remote_container_adaptation: false,
        };

        let strongest_identity_token_kind = match root_class {
            RootClass::LocalPosixLike => StrongestIdentityTokenKind::DeviceInodeGeneration,
            RootClass::LocalWindowsLike => StrongestIdentityTokenKind::WindowsObjectId,
            _ => StrongestIdentityTokenKind::ContentHashOnly,
        };

        let envelope = RootCapabilityEnvelope {
            root_id: root_id.into(),
            root_class,
            capability_flags,
            strongest_identity_token_kind,
            fallback_identity_token_kinds: vec![FallbackIdentityTokenKind::InodeMtimeSize],
            preferred_save_mode: AtomicWriteMode::AtomicReplace,
            permitted_save_modes: vec![
                AtomicWriteMode::AtomicReplace,
                AtomicWriteMode::InPlaceWrite,
            ],
            watcher_source: crate::watcher::WatcherSource::OsNativeWatcher,
            mount_graph_hash: None,
        };

        Ok(Self {
            envelope,
            workspace_id: workspace_id.into(),
            root_badge: "local".to_owned(),
            mount_path,
            trust_state: TrustState::PendingEvaluation,
            policy_scope: None,
        })
    }

    /// Creates a local filesystem root mounted at the host root for diagnostics.
    ///
    /// This intentionally has host-wide path authority and therefore must not
    /// be used for steady-state workspace file operations. Production callers
    /// must use [`Self::new`] with the selected workspace root so membership can
    /// be proven at every operation.
    pub fn host_root(workspace_id: impl Into<String>, root_id: impl Into<String>) -> Self {
        let mount_path = default_mount_path();
        Self::new(workspace_id, root_id, mount_path).expect("host_root mount path must be absolute")
    }

    /// Overrides the trust posture projected into identity records.
    ///
    /// Roots begin in [`TrustState::PendingEvaluation`]; callers may only set a
    /// wider state after the workspace trust authority has resolved it.
    pub fn with_trust_state(mut self, trust_state: TrustState) -> Self {
        self.trust_state = trust_state;
        self
    }

    /// Returns the workspace identity this root is attached to.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Returns the canonical workspace root enforced by this adapter.
    pub fn mount_path(&self) -> &Path {
        &self.mount_path
    }

    fn claims_path(&self, path: &Path) -> bool {
        path.is_absolute()
            && path
                .canonicalize()
                .is_ok_and(|canonical| canonical.starts_with(&self.mount_path))
    }

    fn canonical_path_for_uri(&self, uri: &VfsUri) -> Result<PathBuf, RootResolveError> {
        let Some(path) = uri.file_path() else {
            return Err(RootResolveError::NotInRoot(uri.clone()));
        };
        if !path.is_absolute() {
            return Err(RootResolveError::NotInRoot(uri.clone()));
        }
        let canonical = path
            .canonicalize()
            .map_err(|_| RootResolveError::NotInRoot(uri.clone()))?;
        if !canonical.starts_with(&self.mount_path) {
            return Err(RootResolveError::NotInRoot(uri.clone()));
        }
        Ok(canonical)
    }

    fn canonical_path_for_io(
        &self,
        uri: &VfsUri,
        operation: &'static str,
    ) -> Result<PathBuf, RootIoError> {
        self.canonical_path_for_uri(uri)
            .map_err(|_| RootIoError::NotSupported {
                uri: uri.clone(),
                operation,
            })
    }

    fn display_label_for_path(path: &Path) -> String {
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("untitled")
            .to_owned()
    }

    fn logical_uri_for_canonical_path(
        &self,
        canonical_path: &Path,
    ) -> Result<VfsUri, RootResolveError> {
        let relative = canonical_path
            .strip_prefix(&self.mount_path)
            .unwrap_or(canonical_path);
        let logical_path = relative.to_string_lossy().replace('\\', "/");
        VfsUri::workspace_logical_uri(
            &self.workspace_id,
            &self.envelope.root_id,
            logical_path.as_ref(),
        )
        .map_err(|err| RootResolveError::UriInvalid {
            uri: logical_path.to_string(),
            detail: err.to_string(),
        })
    }
}

impl VfsRoot for LocalFilesystemRoot {
    fn workspace_id(&self) -> Option<&str> {
        Some(&self.workspace_id)
    }

    fn envelope(&self) -> &RootCapabilityEnvelope {
        &self.envelope
    }

    fn root_badge(&self) -> &str {
        &self.root_badge
    }

    fn claims_uri(&self, uri: &VfsUri) -> bool {
        if uri.scheme() != "file" {
            return false;
        }
        uri.file_path().is_some_and(|path| self.claims_path(&path))
    }

    fn identity_record(
        &self,
        presentation_uri: &VfsUri,
    ) -> Result<IdentityRecord, RootResolveError> {
        if !self.claims_uri(presentation_uri) {
            return Err(RootResolveError::NotInRoot(presentation_uri.clone()));
        }

        let canonical_path = self.canonical_path_for_uri(presentation_uri)?;
        let canonical_uri = VfsUri::file_url_for_path(&canonical_path).ok_or_else(|| {
            RootResolveError::IoFailure {
                uri: presentation_uri.clone(),
                detail: "could not canonicalize file uri".to_owned(),
            }
        })?;
        let logical_uri = self.logical_uri_for_canonical_path(&canonical_path)?;

        let strongest_identity_token = self.read_strongest_identity_token(&canonical_uri)?;
        let fallback_identity_tokens = self.read_fallback_identity_tokens(&canonical_uri)?;

        Ok(IdentityRecord {
            presentation_path: PresentationPath {
                uri: presentation_uri.clone(),
                display_label: Self::display_label_for_path(&canonical_path),
                root_badge: self.root_badge.clone(),
            },
            logical_workspace_identity: LogicalWorkspaceIdentity {
                workspace_id: self.workspace_id.clone(),
                root_id: self.envelope.root_id.clone(),
                logical_uri,
                trust_state: self.trust_state,
                policy_scope: self.policy_scope.clone(),
            },
            canonical_filesystem_object: CanonicalFilesystemObject {
                canonical_uri,
                normalization_form: NormalizationForm::MixedObserved,
                strongest_identity_token,
                fallback_identity_tokens,
            },
            alias_set: AliasSet {
                aliases: Vec::new(),
            },
        })
    }

    fn read_strongest_identity_token(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<IdentityToken, RootResolveError> {
        if canonical_uri.scheme() != "file" {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        }
        let canonical_path = self.canonical_path_for_uri(canonical_uri)?;
        let metadata =
            std::fs::metadata(&canonical_path).map_err(|err| RootResolveError::IoFailure {
                uri: canonical_uri.clone(),
                detail: err.to_string(),
            })?;

        let gen = generation_counter_hint(&metadata);
        let (kind, value) = strongest_token_for_metadata(&metadata, gen);
        Ok(IdentityToken { kind, value })
    }

    fn read_fallback_identity_tokens(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<Vec<FallbackIdentityToken>, RootResolveError> {
        if canonical_uri.scheme() != "file" {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        }
        let canonical_path = self.canonical_path_for_uri(canonical_uri)?;
        let metadata =
            std::fs::metadata(&canonical_path).map_err(|err| RootResolveError::IoFailure {
                uri: canonical_uri.clone(),
                detail: err.to_string(),
            })?;

        Ok(vec![FallbackIdentityToken {
            kind: FallbackIdentityTokenKind::InodeMtimeSize,
            value: inode_mtime_size_fallback(&metadata),
        }])
    }

    fn read_generation_token(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<GenerationToken, RootResolveError> {
        let identity = self.read_strongest_identity_token(canonical_uri)?;
        Ok(GenerationToken {
            kind: match identity.kind {
                StrongestIdentityTokenKind::FileIdGeneration => {
                    GenerationTokenKind::FileIdGeneration
                }
                StrongestIdentityTokenKind::DeviceInodeGeneration => {
                    GenerationTokenKind::DeviceInodeGeneration
                }
                StrongestIdentityTokenKind::WindowsObjectId => GenerationTokenKind::WindowsObjectId,
                StrongestIdentityTokenKind::ProviderObjectIdRevision => {
                    GenerationTokenKind::ProviderObjectIdRevision
                }
                StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs => {
                    GenerationTokenKind::ContentHash
                }
                StrongestIdentityTokenKind::ContentHashOnly => GenerationTokenKind::ContentHash,
            },
            value: identity.value,
        })
    }

    fn permission_snapshot(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<PermissionSnapshot, RootResolveError> {
        if canonical_uri.scheme() != "file" {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        }
        let canonical_path = self.canonical_path_for_uri(canonical_uri)?;
        let writable = std::fs::OpenOptions::new()
            .write(true)
            .open(&canonical_path)
            .is_ok();
        let metadata =
            std::fs::metadata(&canonical_path).map_err(|err| RootResolveError::IoFailure {
                uri: canonical_uri.clone(),
                detail: err.to_string(),
            })?;
        Ok(permission_snapshot_for_metadata(writable, &metadata))
    }

    fn read_bytes(&self, canonical_uri: &VfsUri) -> Result<Vec<u8>, RootIoError> {
        let path = self.canonical_path_for_io(canonical_uri, "read_bytes_outside_root_scope")?;
        std::fs::read(&path).map_err(|err| RootIoError::IoFailure {
            uri: canonical_uri.clone(),
            detail: err.to_string(),
        })
    }

    fn write_bytes(
        &mut self,
        canonical_uri: &VfsUri,
        new_content: Vec<u8>,
    ) -> Result<(), RootIoError> {
        let path = self.canonical_path_for_io(canonical_uri, "write_bytes_outside_root_scope")?;
        std::fs::write(&path, new_content).map_err(|err| RootIoError::IoFailure {
            uri: canonical_uri.clone(),
            detail: err.to_string(),
        })
    }
}

fn default_mount_path() -> PathBuf {
    #[cfg(windows)]
    {
        use std::path::Component;
        if let Ok(cwd) = std::env::current_dir() {
            let mut comps = cwd.components();
            if let Some(Component::Prefix(prefix)) = comps.next() {
                return PathBuf::from(prefix.as_os_str()).join("\\");
            }
        }
        PathBuf::from("C:\\")
    }
    #[cfg(not(windows))]
    {
        PathBuf::from("/")
    }
}

fn default_case_sensitivity() -> CaseSensitivity {
    if cfg!(windows) {
        return CaseSensitivity::InsensitivePreserving;
    }
    if cfg!(target_os = "macos") {
        return CaseSensitivity::InsensitivePreserving;
    }
    CaseSensitivity::Sensitive
}

fn generation_counter_hint(metadata: &std::fs::Metadata) -> u128 {
    let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    modified
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0)
        .saturating_add(metadata.len() as u128)
}

fn strongest_token_for_metadata(
    metadata: &std::fs::Metadata,
    gen: u128,
) -> (StrongestIdentityTokenKind, String) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        let dev = metadata.dev();
        let ino = metadata.ino();
        (
            StrongestIdentityTokenKind::DeviceInodeGeneration,
            format!("dev:{dev}/ino:{ino}/gen:{gen}"),
        )
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        let serial = metadata.volume_serial_number().unwrap_or_default();
        let idx = ((metadata.file_index_high() as u64) << 32) | metadata.file_index_low() as u64;
        (
            StrongestIdentityTokenKind::WindowsObjectId,
            format!("vol:{serial}/idx:{idx}/gen:{gen}"),
        )
    }
    #[cfg(not(any(unix, windows)))]
    {
        (
            StrongestIdentityTokenKind::ContentHashOnly,
            format!("len:{}/gen:{gen}", metadata.len()),
        )
    }
}

fn inode_mtime_size_fallback(metadata: &std::fs::Metadata) -> String {
    let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let secs = modified
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("mtime:{secs}/len:{}", metadata.len())
}

fn permission_snapshot_for_metadata(
    writable: bool,
    metadata: &std::fs::Metadata,
) -> PermissionSnapshot {
    let mode = permission_mode_string(metadata);
    let (owner, group) = owner_group_strings(metadata);
    PermissionSnapshot {
        writable,
        mode,
        owner,
        group,
        acl_summary: None,
    }
}

fn permission_mode_string(metadata: &std::fs::Metadata) -> String {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        format!("{:04o}", metadata.permissions().mode() & 0o7777)
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        "unknown".to_owned()
    }
}

fn owner_group_strings(metadata: &std::fs::Metadata) -> (Option<String>, Option<String>) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        (
            Some(metadata.uid().to_string()),
            Some(metadata.gid().to_string()),
        )
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        (None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("aureline-vfs-{label}-{nonce}"));
        std::fs::create_dir_all(&path).expect("temp root create");
        path
    }

    #[test]
    fn local_filesystem_root_resolves_file_identity_under_mount() {
        let tmp_root = fixture_root("local-root");
        let file_path = tmp_root.join("note.txt");
        std::fs::write(&file_path, b"hello\n").expect("temp file write");

        let root = LocalFilesystemRoot::new("ws-test", "root-local", tmp_root.clone())
            .expect("root build should succeed");
        let uri = VfsUri::file_url_for_path(&file_path).expect("file uri build");
        let identity = root
            .identity_record(&uri)
            .expect("identity record should resolve");
        assert_eq!(identity.presentation_path.uri, uri);
        assert_eq!(identity.presentation_path.root_badge, "local");
        assert_eq!(identity.logical_workspace_identity.workspace_id, "ws-test");
        assert_eq!(identity.logical_workspace_identity.root_id, "root-local");
        assert_eq!(
            identity.logical_workspace_identity.trust_state,
            TrustState::PendingEvaluation
        );
        assert_eq!(
            identity.logical_workspace_identity.logical_uri.scheme(),
            "aureline-ws"
        );
        assert_eq!(
            identity.canonical_filesystem_object.canonical_uri.scheme(),
            "file"
        );

        let _ = std::fs::remove_dir_all(&tmp_root);
    }

    #[test]
    fn local_filesystem_root_rejects_files_outside_mount() {
        let mount = std::env::temp_dir();
        let file_path = std::env::temp_dir()
            .parent()
            .unwrap_or(&mount)
            .join("outside.txt");
        let root = LocalFilesystemRoot::new("ws-test", "root-local", mount)
            .expect("root build should succeed");
        let uri = VfsUri::file_url_for_path(&file_path).unwrap_or_else(|| {
            VfsUri::parse("file:///outside.txt".to_owned()).expect("fallback uri parse")
        });
        assert!(!root.claims_uri(&uri));
        let err = root
            .identity_record(&uri)
            .expect_err("expected scope rejection");
        assert_eq!(err, RootResolveError::NotInRoot(uri));
    }

    #[test]
    fn local_filesystem_root_requires_an_existing_directory_mount() {
        let tmp_root = fixture_root("mount-validation");
        let missing = tmp_root.join("missing-workspace");
        let missing_err = LocalFilesystemRoot::new("ws-test", "root-local", missing.clone())
            .expect_err("missing mount must fail closed");
        assert!(matches!(
            missing_err,
            LocalFilesystemRootError::MountPathUnavailable { path, .. } if path == missing
        ));

        let file_mount = tmp_root.join("not-a-directory");
        std::fs::write(&file_mount, b"not a root").expect("fixture file write");
        let file_err = LocalFilesystemRoot::new("ws-test", "root-local", file_mount.clone())
            .expect_err("file mount must fail closed");
        assert!(matches!(
            file_err,
            LocalFilesystemRootError::MountPathNotDirectory(path)
                if path.file_name() == file_mount.file_name()
        ));

        let _ = std::fs::remove_dir_all(&tmp_root);
    }

    #[test]
    fn raw_io_rejects_cross_workspace_paths() {
        let fixture = fixture_root("cross-workspace");
        let workspace_a = fixture.join("workspace-a");
        let workspace_b = fixture.join("workspace-b");
        std::fs::create_dir_all(&workspace_a).expect("workspace A create");
        std::fs::create_dir_all(&workspace_b).expect("workspace B create");
        let own_file = workspace_a.join("own.txt");
        let foreign_file = workspace_b.join("foreign.txt");
        std::fs::write(&own_file, b"own").expect("own file write");
        std::fs::write(&foreign_file, b"foreign").expect("foreign file write");

        let mut root =
            LocalFilesystemRoot::new("ws-a", "root-a", workspace_a).expect("workspace root build");
        let own_uri = VfsUri::file_url_for_path(&own_file).expect("own uri");
        let foreign_uri = VfsUri::file_url_for_path(&foreign_file).expect("foreign uri");

        assert_eq!(root.read_bytes(&own_uri).expect("own read"), b"own");
        assert!(matches!(
            root.read_bytes(&foreign_uri),
            Err(RootIoError::NotSupported {
                operation: "read_bytes_outside_root_scope",
                ..
            })
        ));
        assert!(matches!(
            root.write_bytes(&foreign_uri, b"overwritten".to_vec()),
            Err(RootIoError::NotSupported {
                operation: "write_bytes_outside_root_scope",
                ..
            })
        ));
        assert_eq!(
            std::fs::read(&foreign_file).expect("foreign file remains"),
            b"foreign"
        );

        let _ = std::fs::remove_dir_all(&fixture);
    }

    #[test]
    fn raw_write_rejects_nonexistent_lexical_escape_and_does_not_create() {
        let fixture = fixture_root("lexical-escape");
        let workspace = fixture.join("workspace");
        let nested = workspace.join("nested");
        std::fs::create_dir_all(&nested).expect("nested workspace create");
        let escaped_target = fixture.join("escaped.txt");
        let traversal = nested.join("..").join("..").join("escaped.txt");
        let traversal_uri =
            VfsUri::file_url_for_path_lossy(&traversal).expect("lossy traversal uri");

        let mut root =
            LocalFilesystemRoot::new("ws-a", "root-a", workspace).expect("workspace root build");
        assert!(!root.claims_uri(&traversal_uri));
        assert!(matches!(
            root.write_bytes(&traversal_uri, b"created outside".to_vec()),
            Err(RootIoError::NotSupported {
                operation: "write_bytes_outside_root_scope",
                ..
            })
        ));
        assert!(!escaped_target.exists());

        let _ = std::fs::remove_dir_all(&fixture);
    }

    #[cfg(unix)]
    #[test]
    fn raw_io_rejects_symlink_escape_but_allows_in_root_symlink() {
        use std::os::unix::fs::symlink;

        let fixture = fixture_root("symlink-scope");
        let workspace = fixture.join("workspace");
        let outside = fixture.join("outside");
        std::fs::create_dir_all(&workspace).expect("workspace create");
        std::fs::create_dir_all(&outside).expect("outside create");
        let inside_file = workspace.join("inside.txt");
        let outside_file = outside.join("secret.txt");
        std::fs::write(&inside_file, b"inside").expect("inside write");
        std::fs::write(&outside_file, b"secret").expect("outside write");
        let inside_link = workspace.join("inside-link.txt");
        let outside_link = workspace.join("outside-link.txt");
        symlink(&inside_file, &inside_link).expect("inside symlink");
        symlink(&outside_file, &outside_link).expect("outside symlink");

        let mut root =
            LocalFilesystemRoot::new("ws-a", "root-a", workspace).expect("workspace root build");
        let inside_uri = VfsUri::file_url_for_path_lossy(&inside_link).expect("inside link uri");
        let outside_uri = VfsUri::file_url_for_path_lossy(&outside_link).expect("outside link uri");

        assert_eq!(
            root.read_bytes(&inside_uri).expect("in-root symlink read"),
            b"inside"
        );
        assert!(matches!(
            root.read_bytes(&outside_uri),
            Err(RootIoError::NotSupported {
                operation: "read_bytes_outside_root_scope",
                ..
            })
        ));
        assert!(matches!(
            root.write_bytes(&outside_uri, b"overwritten".to_vec()),
            Err(RootIoError::NotSupported {
                operation: "write_bytes_outside_root_scope",
                ..
            })
        ));
        assert_eq!(
            std::fs::read(&outside_file).expect("outside file remains"),
            b"secret"
        );

        let _ = std::fs::remove_dir_all(&fixture);
    }
}
