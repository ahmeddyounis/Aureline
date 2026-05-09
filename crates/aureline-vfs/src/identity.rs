//! Filesystem-identity layers.
//!
//! Mirrors layers 1-4 of the frozen model in
//! `docs/adr/0006-vfs-save-cache-identity.md` and the vocabulary
//! at `docs/filesystem/filesystem_identity_vocabulary.md`. The
//! fifth layer (the save-target token) lives in [`crate::save`]
//! because the token binds a capability-mode + generation token
//! and is only issuable through the save pipeline.

use crate::capabilities::{
    FallbackIdentityTokenKind, NormalizationForm, StrongestIdentityTokenKind,
};
use crate::uri_model::VfsUri;

/// Layer 1: the path the user opened, verbatim. Tabs, breadcrumbs,
/// copy / paste, and CLI output preserve `uri` where safe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationPath {
    pub uri: VfsUri,
    pub display_label: String,
    pub root_badge: String,
}

/// Workspace trust posture recorded on the logical workspace
/// identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustState {
    Trusted,
    Restricted,
    PendingEvaluation,
}

impl TrustState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::PendingEvaluation => "pending_evaluation",
        }
    }
}

/// Layer 2: the workspace object the product is tracking.
/// `logical_uri` is the workspace-relative address that search,
/// history, AI, review, and CLI all use regardless of alias.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalWorkspaceIdentity {
    pub workspace_id: String,
    pub root_id: String,
    pub logical_uri: VfsUri,
    pub trust_state: TrustState,
    pub policy_scope: Option<String>,
}

/// A single identity token. `kind` is the strongest-token
/// vocabulary; `value` is an opaque equality-comparable handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityToken {
    pub kind: StrongestIdentityTokenKind,
    pub value: String,
}

/// A fallback identity token. Separate type from
/// [`IdentityToken`] because the fallback vocabulary is narrower.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackIdentityToken {
    pub kind: FallbackIdentityTokenKind,
    pub value: String,
}

/// Layer 3: the real underlying filesystem object. Save and
/// external-change decisions target this object first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalFilesystemObject {
    pub canonical_uri: VfsUri,
    pub normalization_form: NormalizationForm,
    pub strongest_identity_token: IdentityToken,
    pub fallback_identity_tokens: Vec<FallbackIdentityToken>,
}

/// Frozen alias-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AliasKind {
    Symlink,
    Junction,
    HardlinkSibling,
    CaseOnlyVariant,
    UnicodeNormalizationVariant,
    RemoteAlias,
    BindMountAlias,
    ContainerMountAlias,
    ArchiveInnerAlias,
}

impl AliasKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Symlink => "symlink",
            Self::Junction => "junction",
            Self::HardlinkSibling => "hardlink_sibling",
            Self::CaseOnlyVariant => "case_only_variant",
            Self::UnicodeNormalizationVariant => "unicode_normalization_variant",
            Self::RemoteAlias => "remote_alias",
            Self::BindMountAlias => "bind_mount_alias",
            Self::ContainerMountAlias => "container_mount_alias",
            Self::ArchiveInnerAlias => "archive_inner_alias",
        }
    }
}

/// A single alias with the step-by-step resolution chain the
/// support surface quotes verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alias {
    pub alias_uri: VfsUri,
    pub alias_kind: AliasKind,
    pub resolution_chain: Vec<String>,
}

/// Layer 4: all known alternative paths to the same canonical
/// object. Authoritative for duplicate-tab prevention and alias
/// disclosure; no surface may dedupe by path-string comparison.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AliasSet {
    pub aliases: Vec<Alias>,
}

/// Combined layers 1-4 record. Every file-open, rename, save,
/// autosave, compare, restore, and AI / apply flow carries this
/// record where the underlying root can provide the layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityRecord {
    pub presentation_path: PresentationPath,
    pub logical_workspace_identity: LogicalWorkspaceIdentity,
    pub canonical_filesystem_object: CanonicalFilesystemObject,
    pub alias_set: AliasSet,
}
