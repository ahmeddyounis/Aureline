//! Local filesystem root adapter.
//!
//! This root resolves `file://` presentation URIs into the VFS identity model.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::capabilities::{
    AtomicWriteMode, CapabilityFlags, CaseSensitivity, FallbackIdentityTokenKind, NormalizationForm,
    RootCapabilityEnvelope, RootClass, StrongestIdentityTokenKind, SymlinkEscapePolicy,
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
}

impl std::fmt::Display for LocalFilesystemRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MountPathNotAbsolute(path) => write!(f, "mount path must be absolute: {path:?}"),
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
        let mut mount_path = mount_path;
        if !mount_path.is_absolute() {
            return Err(LocalFilesystemRootError::MountPathNotAbsolute(mount_path));
        }
        if let Ok(canonical) = mount_path.canonicalize() {
            mount_path = canonical;
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
            permitted_save_modes: vec![AtomicWriteMode::AtomicReplace, AtomicWriteMode::InPlaceWrite],
            watcher_source: crate::watcher::WatcherSource::OsNativeWatcher,
            mount_graph_hash: None,
        };

        Ok(Self {
            envelope,
            workspace_id: workspace_id.into(),
            root_badge: "local".to_owned(),
            mount_path,
            trust_state: TrustState::Trusted,
            policy_scope: None,
        })
    }

    /// Creates a local filesystem root mounted at the host root.
    pub fn host_root(workspace_id: impl Into<String>, root_id: impl Into<String>) -> Self {
        let mount_path = default_mount_path();
        Self::new(workspace_id, root_id, mount_path)
            .expect("host_root mount path must be absolute")
    }

    fn claims_path(&self, path: &Path) -> bool {
        match path.canonicalize() {
            Ok(canonical) => canonical.starts_with(&self.mount_path),
            Err(_) => path.starts_with(&self.mount_path),
        }
    }

    fn canonical_path_for_uri(&self, uri: &VfsUri) -> Result<PathBuf, RootResolveError> {
        let Some(path) = uri.file_path() else {
            return Err(RootResolveError::NotInRoot(uri.clone()));
        };
        if !self.claims_path(&path) {
            return Err(RootResolveError::NotInRoot(uri.clone()));
        }
        Ok(path.canonicalize().unwrap_or(path))
    }

    fn display_label_for_path(path: &Path) -> String {
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("untitled")
            .to_owned()
    }

    fn logical_uri_for_canonical_path(&self, canonical_path: &Path) -> Result<VfsUri, RootResolveError> {
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
        uri.file_path()
            .is_some_and(|path| self.claims_path(&path))
    }

    fn identity_record(&self, presentation_uri: &VfsUri) -> Result<IdentityRecord, RootResolveError> {
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
            alias_set: AliasSet { aliases: Vec::new() },
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
        let metadata = std::fs::metadata(&canonical_path).map_err(|err| RootResolveError::IoFailure {
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
        let metadata = std::fs::metadata(&canonical_path).map_err(|err| RootResolveError::IoFailure {
            uri: canonical_uri.clone(),
            detail: err.to_string(),
        })?;

        Ok(vec![FallbackIdentityToken {
            kind: FallbackIdentityTokenKind::InodeMtimeSize,
            value: inode_mtime_size_fallback(&metadata),
        }])
    }

    fn read_generation_token(&self, canonical_uri: &VfsUri) -> Result<GenerationToken, RootResolveError> {
        let identity = self.read_strongest_identity_token(canonical_uri)?;
        Ok(GenerationToken {
            kind: match identity.kind {
                StrongestIdentityTokenKind::FileIdGeneration => GenerationTokenKind::FileIdGeneration,
                StrongestIdentityTokenKind::DeviceInodeGeneration => {
                    GenerationTokenKind::DeviceInodeGeneration
                }
                StrongestIdentityTokenKind::WindowsObjectId => GenerationTokenKind::WindowsObjectId,
                StrongestIdentityTokenKind::ProviderObjectIdRevision => {
                    GenerationTokenKind::ProviderObjectIdRevision
                }
                StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs => GenerationTokenKind::ContentHash,
                StrongestIdentityTokenKind::ContentHashOnly => GenerationTokenKind::ContentHash,
            },
            value: identity.value,
        })
    }

    fn permission_snapshot(&self, canonical_uri: &VfsUri) -> Result<PermissionSnapshot, RootResolveError> {
        if canonical_uri.scheme() != "file" {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        }
        let canonical_path = self.canonical_path_for_uri(canonical_uri)?;
        let writable = std::fs::OpenOptions::new()
            .write(true)
            .open(&canonical_path)
            .is_ok();
        let metadata = std::fs::metadata(&canonical_path).map_err(|err| RootResolveError::IoFailure {
            uri: canonical_uri.clone(),
            detail: err.to_string(),
        })?;
        Ok(permission_snapshot_for_metadata(writable, &metadata))
    }

    fn read_bytes(&self, canonical_uri: &VfsUri) -> Result<Vec<u8>, RootIoError> {
        let Some(path) = canonical_uri.file_path() else {
            return Err(RootIoError::NotSupported {
                uri: canonical_uri.clone(),
                operation: "read_bytes",
            });
        };
        std::fs::read(&path).map_err(|err| RootIoError::IoFailure {
            uri: canonical_uri.clone(),
            detail: err.to_string(),
        })
    }

    fn write_bytes(&mut self, canonical_uri: &VfsUri, new_content: Vec<u8>) -> Result<(), RootIoError> {
        let Some(path) = canonical_uri.file_path() else {
            return Err(RootIoError::NotSupported {
                uri: canonical_uri.clone(),
                operation: "write_bytes",
            });
        };
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

fn strongest_token_for_metadata(metadata: &std::fs::Metadata, gen: u128) -> (StrongestIdentityTokenKind, String) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        let dev = metadata.dev();
        let ino = metadata.ino();
        return (
            StrongestIdentityTokenKind::DeviceInodeGeneration,
            format!("dev:{dev}/ino:{ino}/gen:{gen}"),
        );
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        let serial = metadata.volume_serial_number().unwrap_or_default();
        let idx = ((metadata.file_index_high() as u64) << 32) | metadata.file_index_low() as u64;
        return (
            StrongestIdentityTokenKind::WindowsObjectId,
            format!("vol:{serial}/idx:{idx}/gen:{gen}"),
        );
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

fn permission_snapshot_for_metadata(writable: bool, metadata: &std::fs::Metadata) -> PermissionSnapshot {
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
        return format!("{:04o}", metadata.permissions().mode() & 0o7777);
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
        return (Some(metadata.uid().to_string()), Some(metadata.gid().to_string()));
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

    #[test]
    fn local_filesystem_root_resolves_file_identity_under_mount() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let tmp_root = std::env::temp_dir().join(format!("aureline-vfs-local-root-{nonce}"));
        std::fs::create_dir_all(&tmp_root).expect("temp root create");
        let file_path = tmp_root.join("note.txt");
        std::fs::write(&file_path, b"hello\n").expect("temp file write");

        let root = LocalFilesystemRoot::new("ws-test", "root-local", tmp_root.clone())
            .expect("root build should succeed");
        let uri = VfsUri::file_url_for_path(&file_path).expect("file uri build");
        let identity = root.identity_record(&uri).expect("identity record should resolve");
        assert_eq!(identity.presentation_path.uri, uri);
        assert_eq!(identity.presentation_path.root_badge, "local");
        assert_eq!(identity.logical_workspace_identity.workspace_id, "ws-test");
        assert_eq!(identity.logical_workspace_identity.root_id, "root-local");
        assert_eq!(
            identity.logical_workspace_identity.logical_uri.scheme(),
            "aureline-ws"
        );
        assert_eq!(
            identity
                .canonical_filesystem_object
                .canonical_uri
                .scheme(),
            "file"
        );

        let _ = std::fs::remove_dir_all(&tmp_root);
    }

    #[test]
    fn local_filesystem_root_rejects_files_outside_mount() {
        let mount = std::env::temp_dir();
        let file_path = std::env::temp_dir().parent().unwrap_or(&mount).join("outside.txt");
        let root = LocalFilesystemRoot::new("ws-test", "root-local", mount)
            .expect("root build should succeed");
        let uri = VfsUri::file_url_for_path(&file_path).unwrap_or_else(|| {
            VfsUri::parse("file:///outside.txt".to_owned()).expect("fallback uri parse")
        });
        assert!(!root.claims_uri(&uri));
        let err = root.identity_record(&uri).expect_err("expected scope rejection");
        assert_eq!(err, RootResolveError::NotInRoot(uri));
    }
}
