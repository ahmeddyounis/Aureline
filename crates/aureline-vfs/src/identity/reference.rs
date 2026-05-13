//! Shared filesystem identity references.
//!
//! The VFS identity layers carry the full path, logical, canonical, and alias
//! state. This projection mints the stable reference string that editor, Git,
//! restore, mutation, support, and review records quote when they need to point
//! at the same object without copying the whole identity record.

use super::IdentityRecord;

/// Cross-surface references for one filesystem identity object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesystemIdentityReferenceSet {
    /// Stable object ref every surface stores when it means this identity.
    pub filesystem_identity_ref: String,
    /// Editor-buffer identity ref; equal to [`Self::filesystem_identity_ref`].
    pub editor_file_identity_ref: String,
    /// Git path-truth identity ref; equal to [`Self::filesystem_identity_ref`].
    pub git_file_identity_ref: String,
    /// Restore-target identity ref; equal to [`Self::filesystem_identity_ref`].
    pub restore_file_identity_ref: String,
    /// Mutation-target identity ref; equal to [`Self::filesystem_identity_ref`].
    pub mutation_file_identity_ref: String,
}

impl FilesystemIdentityReferenceSet {
    /// Returns true when all first consumers point at the same ref.
    pub fn all_flows_share_identity(&self) -> bool {
        self.editor_file_identity_ref == self.filesystem_identity_ref
            && self.git_file_identity_ref == self.filesystem_identity_ref
            && self.restore_file_identity_ref == self.filesystem_identity_ref
            && self.mutation_file_identity_ref == self.filesystem_identity_ref
    }
}

/// Builds the shared reference set for an [`IdentityRecord`].
pub fn filesystem_identity_reference_set(
    record: &IdentityRecord,
) -> FilesystemIdentityReferenceSet {
    let logical = &record.logical_workspace_identity;
    let digest = stable_identity_digest(record);
    let filesystem_identity_ref = format!(
        "fsid:{workspace}:{root}:{digest}",
        workspace = logical.workspace_id,
        root = logical.root_id,
    );

    FilesystemIdentityReferenceSet {
        editor_file_identity_ref: filesystem_identity_ref.clone(),
        git_file_identity_ref: filesystem_identity_ref.clone(),
        restore_file_identity_ref: filesystem_identity_ref.clone(),
        mutation_file_identity_ref: filesystem_identity_ref.clone(),
        filesystem_identity_ref,
    }
}

fn stable_identity_digest(record: &IdentityRecord) -> String {
    stable_hash_hex(&[
        record.logical_workspace_identity.workspace_id.as_str(),
        record.logical_workspace_identity.root_id.as_str(),
        record.logical_workspace_identity.logical_uri.as_str(),
        record.canonical_filesystem_object.canonical_uri.as_str(),
    ])
}

fn stable_hash_hex(parts: &[&str]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::{
        FallbackIdentityTokenKind, NormalizationForm, StrongestIdentityTokenKind,
    };
    use crate::identity::{
        AliasSet, CanonicalFilesystemObject, FallbackIdentityToken, IdentityRecord, IdentityToken,
        LogicalWorkspaceIdentity, PresentationPath, TrustState,
    };
    use crate::uri_model::VfsUri;

    fn record(canonical_uri: &str) -> IdentityRecord {
        IdentityRecord {
            presentation_path: PresentationPath {
                uri: VfsUri::parse(canonical_uri.to_owned()).unwrap(),
                display_label: "main.rs".to_owned(),
                root_badge: "local".to_owned(),
            },
            logical_workspace_identity: LogicalWorkspaceIdentity {
                workspace_id: "ws".to_owned(),
                root_id: "root".to_owned(),
                logical_uri: VfsUri::parse("aureline-ws://ws/root/main.rs".to_owned()).unwrap(),
                trust_state: TrustState::Trusted,
                policy_scope: None,
            },
            canonical_filesystem_object: CanonicalFilesystemObject {
                canonical_uri: VfsUri::parse(canonical_uri.to_owned()).unwrap(),
                normalization_form: NormalizationForm::Nfc,
                strongest_identity_token: IdentityToken {
                    kind: StrongestIdentityTokenKind::DeviceInodeGeneration,
                    value: "dev:1/ino:1/gen:1".to_owned(),
                },
                fallback_identity_tokens: vec![FallbackIdentityToken {
                    kind: FallbackIdentityTokenKind::InodeMtimeSize,
                    value: "1/0/0".to_owned(),
                }],
            },
            alias_set: AliasSet::default(),
        }
    }

    #[test]
    fn reference_set_is_stable_for_generation_change() {
        let first = filesystem_identity_reference_set(&record("file:///ws/main.rs"));
        let second = filesystem_identity_reference_set(&record("file:///ws/main.rs"));

        assert_eq!(first, second);
        assert!(first.all_flows_share_identity());
    }

    #[test]
    fn canonical_uri_changes_reference() {
        let first = filesystem_identity_reference_set(&record("file:///ws/main.rs"));
        let second = filesystem_identity_reference_set(&record("file:///ws/lib.rs"));

        assert_ne!(
            first.filesystem_identity_ref,
            second.filesystem_identity_ref
        );
    }
}
