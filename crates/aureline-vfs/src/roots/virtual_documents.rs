//! Virtual and generated document roots.
//!
//! These roots model documents that are not ordinary host filesystem files
//! (docs/help panes, generated previews, provider-backed views). They still
//! resolve through the same VFS identity model so editor/search/explorer
//! consumers do not special-case by "magic strings".

use std::collections::BTreeMap;

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

/// Specification for one virtual or generated document stored in-memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualDocumentSpec {
    pub document_id: String,
    pub display_label: String,
    pub kind: VirtualDocumentKind,
    pub content: Vec<u8>,
}

/// Classifies an in-memory document as virtual (provider-backed) or generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualDocumentKind {
    Virtual,
    Generated,
}

impl VirtualDocumentKind {
    fn uri_for(
        &self,
        workspace_id: &str,
        root_id: &str,
        document_id: &str,
    ) -> Result<VfsUri, crate::uri_model::UriError> {
        match self {
            Self::Virtual => VfsUri::virtual_document_uri(workspace_id, root_id, document_id),
            Self::Generated => VfsUri::generated_document_uri(workspace_id, root_id, document_id),
        }
    }

    fn logical_prefix(&self) -> &'static str {
        match self {
            Self::Virtual => "__virtual__",
            Self::Generated => "__generated__",
        }
    }
}

/// Errors returned by [`VirtualDocumentRoot`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualDocumentRootError {
    DuplicateDocumentId(String),
    UriBuildFailed(String),
}

impl std::fmt::Display for VirtualDocumentRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateDocumentId(id) => write!(f, "duplicate document id: {id}"),
            Self::UriBuildFailed(detail) => write!(f, "uri build failed: {detail}"),
        }
    }
}

impl std::error::Error for VirtualDocumentRootError {}

/// In-memory root for virtual and generated documents.
#[derive(Debug, Clone)]
pub struct VirtualDocumentRoot {
    envelope: RootCapabilityEnvelope,
    workspace_id: String,
    root_badge: String,
    trust_state: TrustState,
    policy_scope: Option<String>,
    documents: BTreeMap<String, VirtualDocumentSpec>,
}

impl VirtualDocumentRoot {
    /// Creates a new virtual document root.
    pub fn new(workspace_id: impl Into<String>, root_id: impl Into<String>) -> Self {
        let capability_flags = CapabilityFlags {
            supports_atomic_replace: false,
            supports_in_place_write: false,
            supports_conditional_remote_write: false,
            case_sensitivity: CaseSensitivity::Sensitive,
            unicode_normalization: NormalizationForm::None,
            supports_case_only_rename: false,
            supports_unicode_normalization_rename: false,
            symlink_escape_policy: SymlinkEscapePolicy::Block,
            read_only: true,
            policy_constrained: false,
            review_required_before_save: false,
            review_required_before_rename: false,
            remote_container_adaptation: false,
        };

        let envelope = RootCapabilityEnvelope {
            root_id: root_id.into(),
            root_class: RootClass::VirtualGeneratedDocument,
            capability_flags,
            strongest_identity_token_kind: StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs,
            fallback_identity_token_kinds: vec![FallbackIdentityTokenKind::ContentHash],
            preferred_save_mode: AtomicWriteMode::Blocked,
            permitted_save_modes: vec![AtomicWriteMode::Blocked],
            watcher_source: crate::watcher::WatcherSource::PollingFallback,
            mount_graph_hash: None,
        };

        Self {
            envelope,
            workspace_id: workspace_id.into(),
            root_badge: "virtual".to_owned(),
            trust_state: TrustState::Trusted,
            policy_scope: None,
            documents: BTreeMap::new(),
        }
    }

    /// Registers a document in the root.
    pub fn add_document(
        &mut self,
        spec: VirtualDocumentSpec,
    ) -> Result<VfsUri, VirtualDocumentRootError> {
        if self.documents.contains_key(&spec.document_id) {
            return Err(VirtualDocumentRootError::DuplicateDocumentId(
                spec.document_id,
            ));
        }
        let uri = spec
            .kind
            .uri_for(
                &self.workspace_id,
                &self.envelope.root_id,
                &spec.document_id,
            )
            .map_err(|err| VirtualDocumentRootError::UriBuildFailed(err.to_string()))?;
        self.documents.insert(spec.document_id.clone(), spec);
        Ok(uri)
    }

    fn parse_document_ref(&self, uri: &VfsUri) -> Option<(VirtualDocumentKind, String)> {
        let hier = uri.split_hierarchical()?;
        if hier.authority != self.workspace_id {
            return None;
        }
        if !matches!(hier.scheme, "virtual" | "generated") {
            return None;
        }
        let mut segments = hier.path_segments();
        let root_id = segments.next()?;
        if root_id != self.envelope.root_id {
            return None;
        }
        let doc_id = segments.collect::<Vec<_>>().join("/");
        let kind = if hier.scheme == "generated" {
            VirtualDocumentKind::Generated
        } else {
            VirtualDocumentKind::Virtual
        };
        Some((kind, doc_id))
    }

    fn content_hash(bytes: &[u8]) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("fnv64:{hash:016x}")
    }
}

impl VfsRoot for VirtualDocumentRoot {
    fn envelope(&self) -> &RootCapabilityEnvelope {
        &self.envelope
    }

    fn root_badge(&self) -> &str {
        &self.root_badge
    }

    fn claims_uri(&self, uri: &VfsUri) -> bool {
        self.parse_document_ref(uri).is_some()
    }

    fn identity_record(
        &self,
        presentation_uri: &VfsUri,
    ) -> Result<IdentityRecord, RootResolveError> {
        let Some((kind, document_id)) = self.parse_document_ref(presentation_uri) else {
            return Err(RootResolveError::NotInRoot(presentation_uri.clone()));
        };
        let spec = self
            .documents
            .get(&document_id)
            .ok_or_else(|| RootResolveError::UnknownPresentation(presentation_uri.clone()))?;

        let canonical_uri = kind
            .uri_for(&self.workspace_id, &self.envelope.root_id, &document_id)
            .map_err(|err| RootResolveError::UriInvalid {
                uri: presentation_uri.to_string(),
                detail: err.to_string(),
            })?;
        let logical_path = format!("{}/{}", kind.logical_prefix(), document_id);
        let logical_uri = VfsUri::workspace_logical_uri(
            &self.workspace_id,
            &self.envelope.root_id,
            &logical_path,
        )
        .map_err(|err| RootResolveError::UriInvalid {
            uri: logical_path.clone(),
            detail: err.to_string(),
        })?;

        let strongest_identity_token = IdentityToken {
            kind: StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs,
            value: format!("doc:{document_id}"),
        };
        let fallback_identity_tokens = vec![FallbackIdentityToken {
            kind: FallbackIdentityTokenKind::ContentHash,
            value: Self::content_hash(&spec.content),
        }];

        Ok(IdentityRecord {
            presentation_path: PresentationPath {
                uri: presentation_uri.clone(),
                display_label: spec.display_label.clone(),
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
                normalization_form: NormalizationForm::None,
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
        let Some((_, document_id)) = self.parse_document_ref(canonical_uri) else {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        };
        self.documents
            .get(&document_id)
            .ok_or_else(|| RootResolveError::UnknownCanonical(canonical_uri.clone()))?;
        Ok(IdentityToken {
            kind: StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs,
            value: format!("doc:{document_id}"),
        })
    }

    fn read_fallback_identity_tokens(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<Vec<FallbackIdentityToken>, RootResolveError> {
        let Some((_, document_id)) = self.parse_document_ref(canonical_uri) else {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        };
        let Some(spec) = self.documents.get(&document_id) else {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        };
        Ok(vec![FallbackIdentityToken {
            kind: FallbackIdentityTokenKind::ContentHash,
            value: Self::content_hash(&spec.content),
        }])
    }

    fn read_generation_token(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<GenerationToken, RootResolveError> {
        let Some((_, document_id)) = self.parse_document_ref(canonical_uri) else {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        };
        let Some(spec) = self.documents.get(&document_id) else {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        };
        Ok(GenerationToken {
            kind: GenerationTokenKind::ContentHash,
            value: Self::content_hash(&spec.content),
        })
    }

    fn permission_snapshot(
        &self,
        canonical_uri: &VfsUri,
    ) -> Result<PermissionSnapshot, RootResolveError> {
        if !self.claims_uri(canonical_uri) {
            return Err(RootResolveError::UnknownCanonical(canonical_uri.clone()));
        }
        Ok(PermissionSnapshot::read_only_default())
    }

    fn read_bytes(&self, canonical_uri: &VfsUri) -> Result<Vec<u8>, RootIoError> {
        let Some((_, document_id)) = self.parse_document_ref(canonical_uri) else {
            return Err(RootIoError::NotSupported {
                uri: canonical_uri.clone(),
                operation: "read_bytes",
            });
        };
        let Some(spec) = self.documents.get(&document_id) else {
            return Err(RootIoError::IoFailure {
                uri: canonical_uri.clone(),
                detail: "document not found".to_owned(),
            });
        };
        Ok(spec.content.clone())
    }

    fn write_bytes(
        &mut self,
        canonical_uri: &VfsUri,
        _new_content: Vec<u8>,
    ) -> Result<(), RootIoError> {
        Err(RootIoError::NotSupported {
            uri: canonical_uri.clone(),
            operation: "write_bytes",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtual_document_root_resolves_identity_and_generation_token() {
        let mut root = VirtualDocumentRoot::new("ws-test", "root-virtual");
        let uri = root
            .add_document(VirtualDocumentSpec {
                document_id: "docs/help/intro".to_owned(),
                display_label: "intro.md".to_owned(),
                kind: VirtualDocumentKind::Virtual,
                content: b"hello\n".to_vec(),
            })
            .expect("document registration should succeed");

        let identity = root
            .identity_record(&uri)
            .expect("identity record should resolve");
        assert_eq!(identity.presentation_path.uri, uri);
        assert_eq!(identity.presentation_path.root_badge, "virtual");
        assert_eq!(
            identity
                .canonical_filesystem_object
                .strongest_identity_token
                .kind,
            StrongestIdentityTokenKind::LogicalDocumentIdSourceRefs
        );

        let gen = root
            .read_generation_token(&identity.canonical_filesystem_object.canonical_uri)
            .expect("generation token should read");
        assert_eq!(gen.kind, GenerationTokenKind::ContentHash);
        assert!(
            gen.value.starts_with("fnv64:"),
            "expected content hash token: {}",
            gen.value
        );
    }

    #[test]
    fn unknown_document_yields_unknown_presentation_error() {
        let root = VirtualDocumentRoot::new("ws-test", "root-virtual");
        let uri = VfsUri::virtual_document_uri("ws-test", "root-virtual", "missing")
            .expect("uri build should succeed");
        let err = root
            .identity_record(&uri)
            .expect_err("expected lookup failure");
        assert_eq!(err, RootResolveError::UnknownPresentation(uri));
    }
}
