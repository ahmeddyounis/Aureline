//! Filesystem-identity hardened beta projection.
//!
//! Builds on the alpha five-layer identity model (presentation path, logical
//! workspace identity, canonical filesystem object, alias set, save-target
//! token) and the alpha alias inspector, save-target review, and external-
//! change compare projections by binding them into one typed case row that
//! reviewers can read, and into one metadata-safe support packet that the
//! support-export pipeline consumes verbatim.
//!
//! The beta contract folds five claims into one row:
//!
//! - The UI can explain the opened path, the canonical mutable object, the
//!   alias set, and the save-target token.
//! - Difficult cases — symlinks, junctions, hardlink siblings, case-only drift,
//!   Unicode normalization, bind-mount, container-mount, archive-inner, and
//!   remote-agent overlays — are exercised as fixtures, not anecdotes.
//! - Conflict-resolution (compare-before-write) is bound to the same save
//!   token, so writes never silently overwrite a target that diverged.
//! - Support exports preserve the same `filesystem_identity_ref` editor, git,
//!   restore, and mutation flows used.
//! - The packet excludes raw private material and ambient authority, and
//!   never declares destructive resets or drops user-authored files.
//!
//! Bound to the boundary schema at
//! [`/schemas/state/filesystem_identity_beta.schema.json`](../../../../schemas/state/filesystem_identity_beta.schema.json),
//! the reviewer doc at
//! [`/docs/state/m3/filesystem_identity_beta.md`](../../../../docs/state/m3/filesystem_identity_beta.md),
//! and the protected fixture corpus at
//! [`/fixtures/recovery/m3/filesystem_identity/`](../../../../fixtures/recovery/m3/filesystem_identity/).

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::capabilities::AtomicWriteMode;
use crate::identity::{
    inspect_aliases, AliasKind, ExternalChangeCompareOutcome, ExternalChangeCompareRecord,
    ExternalChangeResolutionAction, PathTruthClass, SaveTargetReviewBlocker,
    SaveTargetReviewRecord, TrustState,
};
use crate::roots::VfsRoot;
use crate::save::{GenerationTokenKind, SaveTargetToken};

/// Stable record-kind tag for a filesystem-identity beta case record.
pub const FILESYSTEM_IDENTITY_BETA_CASE_RECORD_KIND: &str = "filesystem_identity_beta_case_record";

/// Stable record-kind tag for the metadata-safe support packet projection.
pub const FILESYSTEM_IDENTITY_BETA_SUPPORT_PACKET_RECORD_KIND: &str =
    "filesystem_identity_beta_support_packet_record";

/// Frozen schema version for filesystem-identity beta records.
pub const FILESYSTEM_IDENTITY_BETA_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this module mirrors.
pub const FILESYSTEM_IDENTITY_BETA_SCHEMA_REF: &str =
    "schemas/state/filesystem_identity_beta.schema.json";

/// Reviewer doc ref quoted by every emitted packet.
pub const FILESYSTEM_IDENTITY_BETA_DOC_REF: &str = "docs/state/m3/filesystem_identity_beta.md";

/// Repo-relative path of the protected corpus manifest.
pub const FILESYSTEM_IDENTITY_BETA_CORPUS_MANIFEST_REF: &str =
    "fixtures/recovery/m3/filesystem_identity/manifest.yaml";

/// Repo-relative path of the protected corpus directory.
pub const FILESYSTEM_IDENTITY_BETA_CORPUS_DIR: &str = "fixtures/recovery/m3/filesystem_identity";

/// Repo-relative path of the ADR that froze the identity model.
pub const FILESYSTEM_IDENTITY_BETA_ADR_REF: &str = "docs/adr/0006-vfs-save-cache-identity.md";

/// Repo-relative path of the closed cross-surface vocabulary.
pub const FILESYSTEM_IDENTITY_BETA_VOCABULARY_DOC_REF: &str =
    "docs/filesystem/filesystem_identity_vocabulary.md";

/// Closed difficulty-class vocabulary covered by the beta corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DifficultyClass {
    SymlinkAlias,
    JunctionAlias,
    HardlinkSibling,
    CaseOnlyDrift,
    UnicodeNormalization,
    BindMountOverlay,
    ContainerMountOverlay,
    ArchiveInnerOverlay,
    RemoteAgentOverlay,
}

impl DifficultyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SymlinkAlias => "symlink_alias",
            Self::JunctionAlias => "junction_alias",
            Self::HardlinkSibling => "hardlink_sibling",
            Self::CaseOnlyDrift => "case_only_drift",
            Self::UnicodeNormalization => "unicode_normalization",
            Self::BindMountOverlay => "bind_mount_overlay",
            Self::ContainerMountOverlay => "container_mount_overlay",
            Self::ArchiveInnerOverlay => "archive_inner_overlay",
            Self::RemoteAgentOverlay => "remote_agent_overlay",
        }
    }
}

/// Beta lanes the corpus must exercise before release-candidate promotion.
///
/// Symlink, case-only drift, Unicode normalization, and an overlay class are
/// the four acceptance lanes the beta row owes; the corpus is allowed to
/// declare additional difficulty classes but it must cover at least these.
pub const REQUIRED_DIFFICULTY_CLASSES: [DifficultyClass; 4] = [
    DifficultyClass::SymlinkAlias,
    DifficultyClass::CaseOnlyDrift,
    DifficultyClass::UnicodeNormalization,
    DifficultyClass::BindMountOverlay,
];

/// Closed root-class mirror of [`crate::capabilities::RootClass`] with serde
/// derives so the case record stays serializable without leaking the alpha
/// enum's vocabulary out of band.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaRootClass {
    LocalPosixLike,
    LocalWindowsLike,
    RemoteAgentMount,
    ContainerMount,
    VirtualGeneratedDocument,
    ArchiveLikeView,
}

impl BetaRootClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPosixLike => "local_posix_like",
            Self::LocalWindowsLike => "local_windows_like",
            Self::RemoteAgentMount => "remote_agent_mount",
            Self::ContainerMount => "container_mount",
            Self::VirtualGeneratedDocument => "virtual_generated_document",
            Self::ArchiveLikeView => "archive_like_view",
        }
    }

    /// Maps an alpha [`crate::capabilities::RootClass`] onto the beta token.
    pub fn from_alpha(value: crate::capabilities::RootClass) -> Self {
        match value {
            crate::capabilities::RootClass::LocalPosixLike => Self::LocalPosixLike,
            crate::capabilities::RootClass::LocalWindowsLike => Self::LocalWindowsLike,
            crate::capabilities::RootClass::RemoteAgentMount => Self::RemoteAgentMount,
            crate::capabilities::RootClass::ContainerMount => Self::ContainerMount,
            crate::capabilities::RootClass::VirtualGeneratedDocument => {
                Self::VirtualGeneratedDocument
            }
            crate::capabilities::RootClass::ArchiveLikeView => Self::ArchiveLikeView,
        }
    }
}

/// Closed trust-state mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaTrustState {
    Trusted,
    Restricted,
    PendingEvaluation,
}

impl BetaTrustState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::PendingEvaluation => "pending_evaluation",
        }
    }

    /// Maps an alpha [`TrustState`] onto the beta token.
    pub const fn from_alpha(value: TrustState) -> Self {
        match value {
            TrustState::Trusted => Self::Trusted,
            TrustState::Restricted => Self::Restricted,
            TrustState::PendingEvaluation => Self::PendingEvaluation,
        }
    }
}

/// Closed alias-kind mirror with serde derives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaAliasKind {
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

impl BetaAliasKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
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

    /// Maps the alpha [`AliasKind`] onto the beta token.
    pub const fn from_alpha(kind: AliasKind) -> Self {
        match kind {
            AliasKind::Symlink => Self::Symlink,
            AliasKind::Junction => Self::Junction,
            AliasKind::HardlinkSibling => Self::HardlinkSibling,
            AliasKind::CaseOnlyVariant => Self::CaseOnlyVariant,
            AliasKind::UnicodeNormalizationVariant => Self::UnicodeNormalizationVariant,
            AliasKind::RemoteAlias => Self::RemoteAlias,
            AliasKind::BindMountAlias => Self::BindMountAlias,
            AliasKind::ContainerMountAlias => Self::ContainerMountAlias,
            AliasKind::ArchiveInnerAlias => Self::ArchiveInnerAlias,
        }
    }
}

/// Closed path-truth class mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaPathTruthClass {
    Direct,
    DirectWithKnownAliases,
    ViaSymlink,
    ViaJunction,
    ViaHardlinkSibling,
    ViaCaseOnlyVariant,
    ViaUnicodeNormalizationVariant,
    ViaRemoteAlias,
    ViaBindMountAlias,
    ViaContainerMountAlias,
    ViaArchiveInnerAlias,
    DivergentUnknown,
}

impl BetaPathTruthClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::DirectWithKnownAliases => "direct_with_known_aliases",
            Self::ViaSymlink => "via_symlink",
            Self::ViaJunction => "via_junction",
            Self::ViaHardlinkSibling => "via_hardlink_sibling",
            Self::ViaCaseOnlyVariant => "via_case_only_variant",
            Self::ViaUnicodeNormalizationVariant => "via_unicode_normalization_variant",
            Self::ViaRemoteAlias => "via_remote_alias",
            Self::ViaBindMountAlias => "via_bind_mount_alias",
            Self::ViaContainerMountAlias => "via_container_mount_alias",
            Self::ViaArchiveInnerAlias => "via_archive_inner_alias",
            Self::DivergentUnknown => "divergent_unknown",
        }
    }

    /// Maps the alpha [`PathTruthClass`] onto the beta token.
    pub const fn from_alpha(value: PathTruthClass) -> Self {
        match value {
            PathTruthClass::Direct => Self::Direct,
            PathTruthClass::DirectWithKnownAliases => Self::DirectWithKnownAliases,
            PathTruthClass::ViaSymlink => Self::ViaSymlink,
            PathTruthClass::ViaJunction => Self::ViaJunction,
            PathTruthClass::ViaHardlinkSibling => Self::ViaHardlinkSibling,
            PathTruthClass::ViaCaseOnlyVariant => Self::ViaCaseOnlyVariant,
            PathTruthClass::ViaUnicodeNormalizationVariant => Self::ViaUnicodeNormalizationVariant,
            PathTruthClass::ViaRemoteAlias => Self::ViaRemoteAlias,
            PathTruthClass::ViaBindMountAlias => Self::ViaBindMountAlias,
            PathTruthClass::ViaContainerMountAlias => Self::ViaContainerMountAlias,
            PathTruthClass::ViaArchiveInnerAlias => Self::ViaArchiveInnerAlias,
            PathTruthClass::DivergentUnknown => Self::DivergentUnknown,
        }
    }
}

/// Closed atomic-write-mode mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaAtomicWriteMode {
    AtomicReplace,
    InPlaceWrite,
    ConditionalRemoteWrite,
    Blocked,
}

impl BetaAtomicWriteMode {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtomicReplace => "atomic_replace",
            Self::InPlaceWrite => "in_place_write",
            Self::ConditionalRemoteWrite => "conditional_remote_write",
            Self::Blocked => "blocked",
        }
    }

    /// Maps the alpha [`AtomicWriteMode`] onto the beta token.
    pub const fn from_alpha(value: AtomicWriteMode) -> Self {
        match value {
            AtomicWriteMode::AtomicReplace => Self::AtomicReplace,
            AtomicWriteMode::InPlaceWrite => Self::InPlaceWrite,
            AtomicWriteMode::ConditionalRemoteWrite => Self::ConditionalRemoteWrite,
            AtomicWriteMode::Blocked => Self::Blocked,
        }
    }
}

/// Closed save-token-kind mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaSaveTokenKind {
    FileIdGeneration,
    DeviceInodeGeneration,
    WindowsObjectId,
    ProviderObjectIdRevision,
    InodeMtimeSize,
    RemoteRevisionToken,
    ContentHash,
}

impl BetaSaveTokenKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileIdGeneration => "file_id_generation",
            Self::DeviceInodeGeneration => "device_inode_generation",
            Self::WindowsObjectId => "windows_object_id",
            Self::ProviderObjectIdRevision => "provider_object_id_revision",
            Self::InodeMtimeSize => "inode_mtime_size",
            Self::RemoteRevisionToken => "remote_revision_token",
            Self::ContentHash => "content_hash",
        }
    }

    /// Maps the alpha [`GenerationTokenKind`] onto the beta token.
    pub const fn from_alpha(value: GenerationTokenKind) -> Self {
        match value {
            GenerationTokenKind::FileIdGeneration => Self::FileIdGeneration,
            GenerationTokenKind::DeviceInodeGeneration => Self::DeviceInodeGeneration,
            GenerationTokenKind::WindowsObjectId => Self::WindowsObjectId,
            GenerationTokenKind::ProviderObjectIdRevision => Self::ProviderObjectIdRevision,
            GenerationTokenKind::InodeMtimeSize => Self::InodeMtimeSize,
            GenerationTokenKind::RemoteRevisionToken => Self::RemoteRevisionToken,
            GenerationTokenKind::ContentHash => Self::ContentHash,
        }
    }
}

/// Closed save-target-review-blocker mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaSaveTargetReviewBlocker {
    ReadOnly,
    PolicyConstrained,
    ReviewRequiredBeforeSave,
    ReviewRequiredBeforeRename,
    NotWritablePerSnapshot,
    AtomicWriteModeBlocked,
    DivergentUnknownAlias,
    UntrustedWorkspace,
}

impl BetaSaveTargetReviewBlocker {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PolicyConstrained => "policy_constrained",
            Self::ReviewRequiredBeforeSave => "review_required_before_save",
            Self::ReviewRequiredBeforeRename => "review_required_before_rename",
            Self::NotWritablePerSnapshot => "not_writable_per_snapshot",
            Self::AtomicWriteModeBlocked => "atomic_write_mode_blocked",
            Self::DivergentUnknownAlias => "divergent_unknown_alias",
            Self::UntrustedWorkspace => "untrusted_workspace",
        }
    }

    /// Maps the alpha blocker onto the beta token.
    pub const fn from_alpha(value: SaveTargetReviewBlocker) -> Self {
        match value {
            SaveTargetReviewBlocker::ReadOnly => Self::ReadOnly,
            SaveTargetReviewBlocker::PolicyConstrained => Self::PolicyConstrained,
            SaveTargetReviewBlocker::ReviewRequiredBeforeSave => Self::ReviewRequiredBeforeSave,
            SaveTargetReviewBlocker::ReviewRequiredBeforeRename => Self::ReviewRequiredBeforeRename,
            SaveTargetReviewBlocker::NotWritablePerSnapshot => Self::NotWritablePerSnapshot,
            SaveTargetReviewBlocker::AtomicWriteModeBlocked => Self::AtomicWriteModeBlocked,
            SaveTargetReviewBlocker::DivergentUnknownAlias => Self::DivergentUnknownAlias,
            SaveTargetReviewBlocker::UntrustedWorkspace => Self::UntrustedWorkspace,
        }
    }
}

/// Closed compare-outcome mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaCompareOutcome {
    Unchanged,
    ExternalChangeDetected,
    SaveConflict,
    WrongTargetPrevented,
    CurrentBytesUnavailable,
}

impl BetaCompareOutcome {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::ExternalChangeDetected => "external_change_detected",
            Self::SaveConflict => "save_conflict",
            Self::WrongTargetPrevented => "wrong_target_prevented",
            Self::CurrentBytesUnavailable => "current_bytes_unavailable",
        }
    }

    /// Maps the alpha [`ExternalChangeCompareOutcome`] onto the beta token.
    pub const fn from_alpha(value: ExternalChangeCompareOutcome) -> Self {
        match value {
            ExternalChangeCompareOutcome::Unchanged => Self::Unchanged,
            ExternalChangeCompareOutcome::ExternalChangeDetected => Self::ExternalChangeDetected,
            ExternalChangeCompareOutcome::SaveConflict => Self::SaveConflict,
            ExternalChangeCompareOutcome::WrongTargetPrevented => Self::WrongTargetPrevented,
            ExternalChangeCompareOutcome::CurrentBytesUnavailable => Self::CurrentBytesUnavailable,
        }
    }
}

/// Closed resolution-action mirror.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BetaResolutionAction {
    Write,
    Compare,
    ReloadExternal,
    Merge,
    SaveAs,
    Recompare,
    OpenAliasDetails,
    Cancel,
}

impl BetaResolutionAction {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Write => "write",
            Self::Compare => "compare",
            Self::ReloadExternal => "reload_external",
            Self::Merge => "merge",
            Self::SaveAs => "save_as",
            Self::Recompare => "recompare",
            Self::OpenAliasDetails => "open_alias_details",
            Self::Cancel => "cancel",
        }
    }

    /// Maps the alpha [`ExternalChangeResolutionAction`] onto the beta token.
    pub const fn from_alpha(value: ExternalChangeResolutionAction) -> Self {
        match value {
            ExternalChangeResolutionAction::Write => Self::Write,
            ExternalChangeResolutionAction::Compare => Self::Compare,
            ExternalChangeResolutionAction::ReloadExternal => Self::ReloadExternal,
            ExternalChangeResolutionAction::Merge => Self::Merge,
            ExternalChangeResolutionAction::SaveAs => Self::SaveAs,
            ExternalChangeResolutionAction::Recompare => Self::Recompare,
            ExternalChangeResolutionAction::OpenAliasDetails => Self::OpenAliasDetails,
            ExternalChangeResolutionAction::Cancel => Self::Cancel,
        }
    }
}

/// One row in the beta alias-inspection projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasInspectionEntry {
    pub alias_uri: String,
    pub alias_kind: BetaAliasKind,
    pub resolution_chain: Vec<String>,
    pub is_canonical: bool,
    pub is_presentation: bool,
}

/// Beta alias-inspection projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasInspection {
    pub presentation_uri: String,
    pub canonical_uri: String,
    pub logical_uri: String,
    pub display_label: String,
    pub root_badge: String,
    pub entries: Vec<AliasInspectionEntry>,
    pub distinct_alias_kinds: Vec<BetaAliasKind>,
    pub presentation_alias_missing: bool,
}

/// Beta save-target review projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveTargetReview {
    pub writes_to_canonical_uri: String,
    pub path_truth_class: BetaPathTruthClass,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub opens_via_alias_kind: Option<BetaAliasKind>,
    pub atomic_write_mode: BetaAtomicWriteMode,
    pub save_token_kind: BetaSaveTokenKind,
    pub save_token_value: String,
    pub save_redirects_target: bool,
    pub review_required_before_save: bool,
    pub review_required_before_rename: bool,
    pub blockers: Vec<BetaSaveTargetReviewBlocker>,
}

/// Beta compare-before-write projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictResolution {
    pub compare_outcome: BetaCompareOutcome,
    pub review_required: bool,
    pub silent_overwrite_forbidden: bool,
    pub resolution_actions: Vec<BetaResolutionAction>,
}

/// Beta support-export alignment row.
///
/// Re-export of the alpha `FilesystemIdentityReferenceSet` so the support
/// packet preserves the same identity ref editor, git, restore, and mutation
/// flows used.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportAlignment {
    pub filesystem_identity_ref: String,
    pub editor_file_identity_ref: String,
    pub git_file_identity_ref: String,
    pub restore_file_identity_ref: String,
    pub mutation_file_identity_ref: String,
    pub support_export_file_identity_ref: String,
    pub all_refs_agree: bool,
}

/// Safety baseline pinned on every emitted case and packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseSafety {
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub destructive_resets_present: bool,
    pub preserves_user_authored_files: bool,
}

impl CaseSafety {
    /// Returns the metadata-safe baseline every emitted case pins.
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
            preserves_user_authored_files: true,
        }
    }
}

/// Companion refs cited on each case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseReferences {
    pub adr_ref: String,
    pub vocabulary_doc_ref: String,
    pub beta_doc_ref: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub alpha_fixture_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub recovery_ladder_alpha_ref: Option<String>,
}

/// One filesystem-identity beta case record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemIdentityBetaCase {
    pub schema_version: u32,
    pub record_kind: String,
    pub case_id: String,
    pub title: String,
    pub difficulty_class: DifficultyClass,
    pub root_class: BetaRootClass,
    pub trust_state: BetaTrustState,
    pub alias_inspection: AliasInspection,
    pub save_target_review: SaveTargetReview,
    pub conflict_resolution: ConflictResolution,
    pub support_export_alignment: SupportExportAlignment,
    pub safety: CaseSafety,
    pub references: CaseReferences,
    pub captured_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reviewer_summary: Option<String>,
}

/// One fixture-bound entry in the beta corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemIdentityBetaCorpusEntry {
    pub fixture_ref: String,
    pub case: FilesystemIdentityBetaCase,
}

/// Beta corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemIdentityBetaCorpus {
    pub entries: Vec<FilesystemIdentityBetaCorpusEntry>,
}

/// One row in the beta support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportPacketCaseRow {
    pub case_id: String,
    pub difficulty_class: DifficultyClass,
    pub presentation_uri: String,
    pub canonical_uri: String,
    pub logical_uri: String,
    pub writes_to_canonical_uri: String,
    pub path_truth_class: BetaPathTruthClass,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub opens_via_alias_kind: Option<BetaAliasKind>,
    pub save_token_kind: BetaSaveTokenKind,
    pub compare_outcome: BetaCompareOutcome,
    pub filesystem_identity_ref: String,
    pub all_refs_agree: bool,
    pub distinct_alias_kinds: Vec<BetaAliasKind>,
}

impl SupportPacketCaseRow {
    fn from_case(case: &FilesystemIdentityBetaCase) -> Self {
        Self {
            case_id: case.case_id.clone(),
            difficulty_class: case.difficulty_class,
            presentation_uri: case.alias_inspection.presentation_uri.clone(),
            canonical_uri: case.alias_inspection.canonical_uri.clone(),
            logical_uri: case.alias_inspection.logical_uri.clone(),
            writes_to_canonical_uri: case.save_target_review.writes_to_canonical_uri.clone(),
            path_truth_class: case.save_target_review.path_truth_class,
            opens_via_alias_kind: case.save_target_review.opens_via_alias_kind,
            save_token_kind: case.save_target_review.save_token_kind,
            compare_outcome: case.conflict_resolution.compare_outcome,
            filesystem_identity_ref: case
                .support_export_alignment
                .filesystem_identity_ref
                .clone(),
            all_refs_agree: case.support_export_alignment.all_refs_agree,
            distinct_alias_kinds: case.alias_inspection.distinct_alias_kinds.clone(),
        }
    }
}

/// Metadata-safe support packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemIdentityBetaSupportPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub schema_ref: String,
    pub corpus_manifest_ref: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub required_difficulty_classes: Vec<DifficultyClass>,
    pub cases: Vec<SupportPacketCaseRow>,
}

impl FilesystemIdentityBetaSupportPacket {
    /// Returns true when the packet preserves the bounded beta contract.
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        if self.cases.is_empty() {
            return false;
        }
        let covered: BTreeSet<DifficultyClass> =
            self.cases.iter().map(|row| row.difficulty_class).collect();
        for required in &REQUIRED_DIFFICULTY_CLASSES {
            if !covered.contains(required) {
                return false;
            }
        }
        self.cases.iter().all(|row| row.all_refs_agree)
    }
}

/// One validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesystemIdentityBetaViolation {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesystemIdentityBetaValidationReport {
    pub violations: Vec<FilesystemIdentityBetaViolation>,
}

impl fmt::Display for FilesystemIdentityBetaValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} filesystem-identity beta violation(s)",
            self.violations.len()
        )
    }
}

impl Error for FilesystemIdentityBetaValidationReport {}

/// Filesystem-identity beta evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemIdentityBetaEvaluator;

impl FilesystemIdentityBetaEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates one case record.
    ///
    /// # Errors
    ///
    /// Returns [`FilesystemIdentityBetaValidationReport`] when the record
    /// fails any of the closed safety checks (record-kind/schema-version
    /// mismatch, missing alias rows for non-direct cases, divergent
    /// `writes_to_canonical_uri`, support-export ref disagreement,
    /// dropped user-authored-files preservation, admitted raw private
    /// material, admitted ambient authority, declared destructive reset,
    /// or an empty resolution-action list).
    pub fn validate_case(
        &self,
        case: &FilesystemIdentityBetaCase,
    ) -> Result<(), FilesystemIdentityBetaValidationReport> {
        let violations = validate_case(case);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(FilesystemIdentityBetaValidationReport { violations })
        }
    }

    /// Validates a corpus against the required-difficulty-class coverage.
    ///
    /// # Errors
    ///
    /// Returns [`FilesystemIdentityBetaValidationReport`] when a required
    /// difficulty class has no seeded case, when a case fails its safety
    /// checks, or when duplicate `case_id`/`fixture_ref` rows appear.
    pub fn validate_corpus(
        &self,
        corpus: &FilesystemIdentityBetaCorpus,
    ) -> Result<(), FilesystemIdentityBetaValidationReport> {
        let violations = validate_corpus(corpus);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(FilesystemIdentityBetaValidationReport { violations })
        }
    }

    /// Builds the metadata-safe support packet projection from a corpus.
    ///
    /// # Errors
    ///
    /// Returns [`FilesystemIdentityBetaValidationReport`] when the corpus
    /// fails [`Self::validate_corpus`].
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &FilesystemIdentityBetaCorpus,
    ) -> Result<FilesystemIdentityBetaSupportPacket, FilesystemIdentityBetaValidationReport> {
        self.validate_corpus(corpus)?;
        let cases = corpus
            .entries
            .iter()
            .map(|entry| SupportPacketCaseRow::from_case(&entry.case))
            .collect::<Vec<_>>();
        Ok(FilesystemIdentityBetaSupportPacket {
            schema_version: FILESYSTEM_IDENTITY_BETA_SCHEMA_VERSION,
            record_kind: FILESYSTEM_IDENTITY_BETA_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: FILESYSTEM_IDENTITY_BETA_DOC_REF.to_owned(),
            schema_ref: FILESYSTEM_IDENTITY_BETA_SCHEMA_REF.to_owned(),
            corpus_manifest_ref: FILESYSTEM_IDENTITY_BETA_CORPUS_MANIFEST_REF.to_owned(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_difficulty_classes: REQUIRED_DIFFICULTY_CLASSES.to_vec(),
            cases,
        })
    }

    /// Compiles a typed beta case from the alpha primitives without
    /// reading raw private bytes off the disk root.
    ///
    /// The compiler folds:
    ///
    /// - the alpha [`SaveTargetReviewRecord`] (presentation + canonical
    ///   + path-truth + token + blockers),
    /// - the alpha [`ExternalChangeCompareRecord`] (compare-before-write
    ///   outcome and review actions),
    /// - the alias-inspection projection derived from the bound
    ///   [`SaveTargetToken`].
    ///
    /// Callers attach the `case_id`, `title`, `difficulty_class`, and
    /// captured-at timestamp.
    #[allow(clippy::too_many_arguments)]
    pub fn compile_from_alpha(
        &self,
        case_id: impl Into<String>,
        title: impl Into<String>,
        difficulty_class: DifficultyClass,
        root_class: BetaRootClass,
        captured_at: impl Into<String>,
        token: &SaveTargetToken,
        review: &SaveTargetReviewRecord,
        compare: &ExternalChangeCompareRecord,
    ) -> FilesystemIdentityBetaCase {
        let alpha_inspection = inspect_aliases(&token.identity);
        let alias_inspection = AliasInspection {
            presentation_uri: alpha_inspection.presentation_uri.as_str().to_owned(),
            canonical_uri: alpha_inspection.canonical_uri.as_str().to_owned(),
            logical_uri: alpha_inspection.logical_uri.as_str().to_owned(),
            display_label: alpha_inspection.display_label.clone(),
            root_badge: alpha_inspection.root_badge.clone(),
            entries: alpha_inspection
                .entries
                .iter()
                .map(|entry| AliasInspectionEntry {
                    alias_uri: entry.alias_uri.as_str().to_owned(),
                    alias_kind: BetaAliasKind::from_alpha(entry.alias_kind),
                    resolution_chain: entry.resolution_chain.clone(),
                    is_canonical: entry.is_canonical,
                    is_presentation: entry.is_presentation,
                })
                .collect(),
            distinct_alias_kinds: alpha_inspection
                .distinct_alias_kinds
                .iter()
                .map(|kind| BetaAliasKind::from_alpha(*kind))
                .collect(),
            presentation_alias_missing: alpha_inspection.presentation_alias_missing,
        };

        let save_target_review = SaveTargetReview {
            writes_to_canonical_uri: review.writes_to_canonical_uri.as_str().to_owned(),
            path_truth_class: BetaPathTruthClass::from_alpha(review.path_truth_class),
            opens_via_alias_kind: review.opens_via_alias_kind.map(BetaAliasKind::from_alpha),
            atomic_write_mode: BetaAtomicWriteMode::from_alpha(review.atomic_write_mode),
            save_token_kind: BetaSaveTokenKind::from_alpha(review.pinned_generation_token_kind),
            save_token_value: review.pinned_generation_token_value.clone(),
            save_redirects_target: review.save_redirects_target,
            review_required_before_save: review.review_required_before_save,
            review_required_before_rename: review.review_required_before_rename,
            blockers: review
                .blockers
                .iter()
                .map(|blocker| BetaSaveTargetReviewBlocker::from_alpha(*blocker))
                .collect(),
        };

        let conflict_resolution = ConflictResolution {
            compare_outcome: BetaCompareOutcome::from_alpha(compare.outcome),
            review_required: compare.review_required,
            silent_overwrite_forbidden: compare.silent_overwrite_forbidden,
            resolution_actions: compare
                .resolution_actions
                .iter()
                .map(|action| BetaResolutionAction::from_alpha(*action))
                .collect(),
        };

        let refs = &compare.identity_references;
        let support_export_alignment = SupportExportAlignment {
            filesystem_identity_ref: refs.filesystem_identity_ref.clone(),
            editor_file_identity_ref: refs.editor_file_identity_ref.clone(),
            git_file_identity_ref: refs.git_file_identity_ref.clone(),
            restore_file_identity_ref: refs.restore_file_identity_ref.clone(),
            mutation_file_identity_ref: refs.mutation_file_identity_ref.clone(),
            support_export_file_identity_ref: refs.filesystem_identity_ref.clone(),
            all_refs_agree: refs.all_flows_share_identity(),
        };

        FilesystemIdentityBetaCase {
            schema_version: FILESYSTEM_IDENTITY_BETA_SCHEMA_VERSION,
            record_kind: FILESYSTEM_IDENTITY_BETA_CASE_RECORD_KIND.to_owned(),
            case_id: case_id.into(),
            title: title.into(),
            difficulty_class,
            root_class,
            trust_state: BetaTrustState::from_alpha(review.trust_state),
            alias_inspection,
            save_target_review,
            conflict_resolution,
            support_export_alignment,
            safety: CaseSafety::metadata_safe_baseline(),
            references: CaseReferences {
                adr_ref: FILESYSTEM_IDENTITY_BETA_ADR_REF.to_owned(),
                vocabulary_doc_ref: FILESYSTEM_IDENTITY_BETA_VOCABULARY_DOC_REF.to_owned(),
                beta_doc_ref: FILESYSTEM_IDENTITY_BETA_DOC_REF.to_owned(),
                alpha_fixture_ref: None,
                recovery_ladder_alpha_ref: Some("docs/support/recovery_ladder_alpha.md".to_owned()),
            },
            captured_at: captured_at.into(),
            reviewer_summary: None,
        }
    }
}

/// Convenience helper that compiles a case directly from a [`VfsRoot`] and a
/// bound [`SaveTargetToken`]. Useful to wire a case row from a synthetic root
/// scenario without manually invoking [`review_save_target`] and
/// [`compare_external_change`].
///
/// Note: this helper reads the canonical bytes of the bound canonical URI so
/// the diff projection is honest. It does NOT export the bytes outside the
/// caller's scope.
///
/// [`review_save_target`]: crate::identity::review_save_target
/// [`compare_external_change`]: crate::identity::compare_external_change
#[allow(clippy::too_many_arguments)]
pub fn compile_case_from_root(
    case_id: impl Into<String>,
    title: impl Into<String>,
    difficulty_class: DifficultyClass,
    root: &dyn VfsRoot,
    token: &SaveTargetToken,
    local_content: &[u8],
    captured_at: impl Into<String>,
    counters: &mut crate::hooks::HookCounters,
) -> FilesystemIdentityBetaCase {
    let review = crate::identity::review_save_target(token);
    let compare = crate::identity::compare_external_change(root, token, local_content, counters);
    let root_class = BetaRootClass::from_alpha(root.envelope().root_class);
    FilesystemIdentityBetaEvaluator::new().compile_from_alpha(
        case_id,
        title,
        difficulty_class,
        root_class,
        captured_at,
        token,
        &review,
        &compare,
    )
}

/// Loads a YAML-encoded [`FilesystemIdentityBetaCase`].
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`FilesystemIdentityBetaCase`].
pub fn load_filesystem_identity_beta_case(
    yaml: &str,
) -> Result<FilesystemIdentityBetaCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

fn validate_corpus(corpus: &FilesystemIdentityBetaCorpus) -> Vec<FilesystemIdentityBetaViolation> {
    let mut violations = Vec::new();

    if corpus.entries.is_empty() {
        push_violation(
            &mut violations,
            "corpus.empty",
            FILESYSTEM_IDENTITY_BETA_CORPUS_DIR,
            "corpus must contain at least one filesystem-identity beta case",
        );
        return violations;
    }

    let mut case_ids = BTreeSet::new();
    let mut fixture_refs = BTreeSet::new();
    let mut seen: BTreeSet<DifficultyClass> = BTreeSet::new();

    for entry in &corpus.entries {
        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_fixture_ref",
                &entry.fixture_ref,
                "fixture_ref must be unique within the corpus",
            );
        }
        let case = &entry.case;
        if !case_ids.insert(case.case_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_case_id",
                &case.case_id,
                "case_id must be unique within the corpus",
            );
        }
        seen.insert(case.difficulty_class);
        violations.extend(validate_case(case));
    }

    for required in REQUIRED_DIFFICULTY_CLASSES {
        if !seen.contains(&required) {
            push_violation(
                &mut violations,
                "corpus.required_difficulty_class_missing",
                required.as_str(),
                format!(
                    "required difficulty class {} has no seeded case",
                    required.as_str()
                ),
            );
        }
    }

    violations
}

fn validate_case(case: &FilesystemIdentityBetaCase) -> Vec<FilesystemIdentityBetaViolation> {
    let mut violations = Vec::new();
    let target = case.case_id.as_str();

    if case.schema_version != FILESYSTEM_IDENTITY_BETA_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "case.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if case.record_kind != FILESYSTEM_IDENTITY_BETA_CASE_RECORD_KIND {
        push_violation(
            &mut violations,
            "case.record_kind",
            target,
            "record_kind must be filesystem_identity_beta_case_record",
        );
    }
    if case.case_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.case_id",
            target,
            "case_id must be non-empty",
        );
    }
    if case.title.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.title",
            target,
            "title must be non-empty",
        );
    }

    validate_alias_inspection(&mut violations, target, case);
    validate_save_target_review(&mut violations, target, case);
    validate_conflict_resolution(&mut violations, target, &case.conflict_resolution);
    validate_support_export_alignment(&mut violations, target, &case.support_export_alignment);
    validate_safety(&mut violations, target, &case.safety);
    validate_references(&mut violations, target, &case.references);

    violations
}

fn validate_alias_inspection(
    violations: &mut Vec<FilesystemIdentityBetaViolation>,
    target: &str,
    case: &FilesystemIdentityBetaCase,
) {
    let inspection = &case.alias_inspection;
    if inspection.presentation_uri.trim().is_empty()
        || inspection.canonical_uri.trim().is_empty()
        || inspection.logical_uri.trim().is_empty()
    {
        push_violation(
            violations,
            "case.alias_inspection.uri",
            target,
            "presentation_uri, canonical_uri, and logical_uri must be non-empty",
        );
    }
    if inspection.display_label.trim().is_empty() {
        push_violation(
            violations,
            "case.alias_inspection.display_label",
            target,
            "display_label must be non-empty",
        );
    }
    if inspection.root_badge.trim().is_empty() {
        push_violation(
            violations,
            "case.alias_inspection.root_badge",
            target,
            "root_badge must be non-empty",
        );
    }
    let needs_alias = case.save_target_review.path_truth_class != BetaPathTruthClass::Direct
        && case.save_target_review.path_truth_class != BetaPathTruthClass::DivergentUnknown;
    if needs_alias && inspection.entries.is_empty() {
        push_violation(
            violations,
            "case.alias_inspection.entries_empty",
            target,
            "alias_inspection.entries must declare at least one alias for non-direct cases",
        );
    }
    let mut alias_uris = BTreeSet::new();
    for entry in &inspection.entries {
        if entry.alias_uri.trim().is_empty() {
            push_violation(
                violations,
                "case.alias_inspection.entry_uri",
                target,
                "alias entry alias_uri must be non-empty",
            );
        }
        if entry.resolution_chain.is_empty() {
            push_violation(
                violations,
                "case.alias_inspection.entry_chain",
                target,
                "alias entry resolution_chain must declare at least one step",
            );
        }
        if !alias_uris.insert(entry.alias_uri.clone()) {
            push_violation(
                violations,
                "case.alias_inspection.entry_duplicate",
                target,
                format!(
                    "duplicate alias_uri {} in alias_inspection.entries",
                    entry.alias_uri
                ),
            );
        }
    }
    if inspection.presentation_alias_missing
        && case.save_target_review.path_truth_class != BetaPathTruthClass::DivergentUnknown
    {
        push_violation(
            violations,
            "case.alias_inspection.presentation_alias_missing",
            target,
            "presentation_alias_missing is only admitted for divergent_unknown cases",
        );
    }
}

fn validate_save_target_review(
    violations: &mut Vec<FilesystemIdentityBetaViolation>,
    target: &str,
    case: &FilesystemIdentityBetaCase,
) {
    let review = &case.save_target_review;
    if review.writes_to_canonical_uri.trim().is_empty() {
        push_violation(
            violations,
            "case.save_target_review.writes_to_canonical_uri",
            target,
            "writes_to_canonical_uri must be non-empty",
        );
    }
    if review.writes_to_canonical_uri != case.alias_inspection.canonical_uri {
        push_violation(
            violations,
            "case.save_target_review.writes_to_canonical_uri_mismatch",
            target,
            "writes_to_canonical_uri must equal alias_inspection.canonical_uri so save lands at the canonical object",
        );
    }
    if review.save_token_value.trim().is_empty() {
        push_violation(
            violations,
            "case.save_target_review.save_token_value",
            target,
            "save_token_value must be non-empty",
        );
    }
    if case.alias_inspection.presentation_uri != case.alias_inspection.canonical_uri
        && !review.save_redirects_target
    {
        push_violation(
            violations,
            "case.save_target_review.save_redirects_target",
            target,
            "save_redirects_target must be true when presentation_uri differs from canonical_uri",
        );
    }
    if review.path_truth_class == BetaPathTruthClass::Direct && review.save_redirects_target {
        push_violation(
            violations,
            "case.save_target_review.direct_does_not_redirect",
            target,
            "save_redirects_target must be false when path_truth_class is direct",
        );
    }
    if let Some(kind) = review.opens_via_alias_kind {
        let expected =
            BetaPathTruthClass::from_alpha(PathTruthClass::from_alias_kind(alias_kind_back(kind)));
        if expected != review.path_truth_class {
            push_violation(
                violations,
                "case.save_target_review.alias_path_truth_mismatch",
                target,
                format!(
                    "opens_via_alias_kind {} must agree with path_truth_class {}",
                    kind.as_str(),
                    review.path_truth_class.as_str()
                ),
            );
        }
    }
}

fn validate_conflict_resolution(
    violations: &mut Vec<FilesystemIdentityBetaViolation>,
    target: &str,
    resolution: &ConflictResolution,
) {
    if resolution.resolution_actions.is_empty() {
        push_violation(
            violations,
            "case.conflict_resolution.resolution_actions",
            target,
            "resolution_actions must declare at least one action",
        );
    }
    let actions: BTreeSet<BetaResolutionAction> =
        resolution.resolution_actions.iter().copied().collect();
    if actions.len() != resolution.resolution_actions.len() {
        push_violation(
            violations,
            "case.conflict_resolution.resolution_actions_duplicate",
            target,
            "resolution_actions must not contain duplicate entries",
        );
    }
    let direct_write_allowed =
        resolution.compare_outcome == BetaCompareOutcome::Unchanged && !resolution.review_required;
    if resolution.compare_outcome != BetaCompareOutcome::Unchanged && !resolution.review_required {
        push_violation(
            violations,
            "case.conflict_resolution.non_unchanged_must_review",
            target,
            "non-unchanged compare_outcome must set review_required = true",
        );
    }
    if !direct_write_allowed && actions.contains(&BetaResolutionAction::Write) {
        push_violation(
            violations,
            "case.conflict_resolution.write_only_when_unchanged",
            target,
            "resolution_actions must only include write when compare_outcome is unchanged and review is not required",
        );
    }
    if resolution.silent_overwrite_forbidden
        && resolution.compare_outcome == BetaCompareOutcome::Unchanged
    {
        push_violation(
            violations,
            "case.conflict_resolution.silent_overwrite_forbidden_implies_review",
            target,
            "silent_overwrite_forbidden cannot be true when compare_outcome is unchanged",
        );
    }
}

fn validate_support_export_alignment(
    violations: &mut Vec<FilesystemIdentityBetaViolation>,
    target: &str,
    alignment: &SupportExportAlignment,
) {
    for (field, value) in [
        (
            "filesystem_identity_ref",
            alignment.filesystem_identity_ref.as_str(),
        ),
        (
            "editor_file_identity_ref",
            alignment.editor_file_identity_ref.as_str(),
        ),
        (
            "git_file_identity_ref",
            alignment.git_file_identity_ref.as_str(),
        ),
        (
            "restore_file_identity_ref",
            alignment.restore_file_identity_ref.as_str(),
        ),
        (
            "mutation_file_identity_ref",
            alignment.mutation_file_identity_ref.as_str(),
        ),
        (
            "support_export_file_identity_ref",
            alignment.support_export_file_identity_ref.as_str(),
        ),
    ] {
        if value.trim().is_empty() {
            push_violation(
                violations,
                "case.support_export_alignment.empty_ref",
                target,
                format!("support_export_alignment.{field} must be non-empty"),
            );
        }
    }
    if alignment.editor_file_identity_ref != alignment.filesystem_identity_ref
        || alignment.git_file_identity_ref != alignment.filesystem_identity_ref
        || alignment.restore_file_identity_ref != alignment.filesystem_identity_ref
        || alignment.mutation_file_identity_ref != alignment.filesystem_identity_ref
        || alignment.support_export_file_identity_ref != alignment.filesystem_identity_ref
    {
        push_violation(
            violations,
            "case.support_export_alignment.refs_disagree",
            target,
            "editor/git/restore/mutation/support_export refs must equal filesystem_identity_ref",
        );
    }
    if !alignment.all_refs_agree {
        push_violation(
            violations,
            "case.support_export_alignment.all_refs_agree",
            target,
            "all_refs_agree must be true so save/conflict/support flows share one identity",
        );
    }
}

fn validate_safety(
    violations: &mut Vec<FilesystemIdentityBetaViolation>,
    target: &str,
    safety: &CaseSafety,
) {
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "case.safety.raw_private_material_excluded",
            target,
            "raw_private_material_excluded must be true",
        );
    }
    if !safety.ambient_authority_excluded {
        push_violation(
            violations,
            "case.safety.ambient_authority_excluded",
            target,
            "ambient_authority_excluded must be true",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "case.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "case.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn validate_references(
    violations: &mut Vec<FilesystemIdentityBetaViolation>,
    target: &str,
    refs: &CaseReferences,
) {
    if refs.adr_ref != FILESYSTEM_IDENTITY_BETA_ADR_REF {
        push_violation(
            violations,
            "case.references.adr_ref",
            target,
            format!("references.adr_ref must pin {FILESYSTEM_IDENTITY_BETA_ADR_REF}"),
        );
    }
    if refs.vocabulary_doc_ref != FILESYSTEM_IDENTITY_BETA_VOCABULARY_DOC_REF {
        push_violation(
            violations,
            "case.references.vocabulary_doc_ref",
            target,
            format!(
                "references.vocabulary_doc_ref must pin {FILESYSTEM_IDENTITY_BETA_VOCABULARY_DOC_REF}"
            ),
        );
    }
    if refs.beta_doc_ref != FILESYSTEM_IDENTITY_BETA_DOC_REF {
        push_violation(
            violations,
            "case.references.beta_doc_ref",
            target,
            format!("references.beta_doc_ref must pin {FILESYSTEM_IDENTITY_BETA_DOC_REF}"),
        );
    }
}

fn push_violation(
    violations: &mut Vec<FilesystemIdentityBetaViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(FilesystemIdentityBetaViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

const fn alias_kind_back(kind: BetaAliasKind) -> AliasKind {
    match kind {
        BetaAliasKind::Symlink => AliasKind::Symlink,
        BetaAliasKind::Junction => AliasKind::Junction,
        BetaAliasKind::HardlinkSibling => AliasKind::HardlinkSibling,
        BetaAliasKind::CaseOnlyVariant => AliasKind::CaseOnlyVariant,
        BetaAliasKind::UnicodeNormalizationVariant => AliasKind::UnicodeNormalizationVariant,
        BetaAliasKind::RemoteAlias => AliasKind::RemoteAlias,
        BetaAliasKind::BindMountAlias => AliasKind::BindMountAlias,
        BetaAliasKind::ContainerMountAlias => AliasKind::ContainerMountAlias,
        BetaAliasKind::ArchiveInnerAlias => AliasKind::ArchiveInnerAlias,
    }
}

/// Returns the checked-in M3 filesystem-identity beta corpus.
///
/// # Errors
///
/// Returns a YAML parse error when a fixture does not match the
/// [`FilesystemIdentityBetaCase`] shape.
pub fn current_filesystem_identity_beta_corpus(
) -> Result<FilesystemIdentityBetaCorpus, serde_yaml::Error> {
    let entries = CASE_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<FilesystemIdentityBetaCase>(yaml).map(|case| {
                FilesystemIdentityBetaCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    case,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(FilesystemIdentityBetaCorpus { entries })
}

const CASE_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/recovery/m3/filesystem_identity/symlink_alias_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/filesystem_identity/symlink_alias_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/filesystem_identity/case_only_drift_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/filesystem_identity/case_only_drift_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/filesystem_identity/unicode_normalization_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/filesystem_identity/unicode_normalization_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/filesystem_identity/bind_mount_overlay_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/filesystem_identity/bind_mount_overlay_case.yaml"
        )),
    ),
];

/// Returns the set of fixture refs the corpus loads, in declaration order.
pub fn current_filesystem_identity_beta_fixture_refs() -> impl Iterator<Item = &'static str> {
    CASE_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aligned_refs(ident: &str) -> SupportExportAlignment {
        SupportExportAlignment {
            filesystem_identity_ref: ident.to_owned(),
            editor_file_identity_ref: ident.to_owned(),
            git_file_identity_ref: ident.to_owned(),
            restore_file_identity_ref: ident.to_owned(),
            mutation_file_identity_ref: ident.to_owned(),
            support_export_file_identity_ref: ident.to_owned(),
            all_refs_agree: true,
        }
    }

    fn direct_case() -> FilesystemIdentityBetaCase {
        FilesystemIdentityBetaCase {
            schema_version: FILESYSTEM_IDENTITY_BETA_SCHEMA_VERSION,
            record_kind: FILESYSTEM_IDENTITY_BETA_CASE_RECORD_KIND.to_owned(),
            case_id: "case:test:direct".to_owned(),
            title: "Direct open".to_owned(),
            difficulty_class: DifficultyClass::SymlinkAlias,
            root_class: BetaRootClass::LocalPosixLike,
            trust_state: BetaTrustState::Trusted,
            alias_inspection: AliasInspection {
                presentation_uri: "file:///ws/doc".to_owned(),
                canonical_uri: "file:///ws/doc".to_owned(),
                logical_uri: "aureline-ws://ws/root/doc".to_owned(),
                display_label: "doc".to_owned(),
                root_badge: "local".to_owned(),
                entries: vec![],
                distinct_alias_kinds: vec![],
                presentation_alias_missing: false,
            },
            save_target_review: SaveTargetReview {
                writes_to_canonical_uri: "file:///ws/doc".to_owned(),
                path_truth_class: BetaPathTruthClass::Direct,
                opens_via_alias_kind: None,
                atomic_write_mode: BetaAtomicWriteMode::AtomicReplace,
                save_token_kind: BetaSaveTokenKind::DeviceInodeGeneration,
                save_token_value: "dev:1/ino:1/gen:1".to_owned(),
                save_redirects_target: false,
                review_required_before_save: false,
                review_required_before_rename: false,
                blockers: vec![],
            },
            conflict_resolution: ConflictResolution {
                compare_outcome: BetaCompareOutcome::Unchanged,
                review_required: false,
                silent_overwrite_forbidden: false,
                resolution_actions: vec![BetaResolutionAction::Write],
            },
            support_export_alignment: aligned_refs("fsid:ws:root:abc"),
            safety: CaseSafety::metadata_safe_baseline(),
            references: CaseReferences {
                adr_ref: FILESYSTEM_IDENTITY_BETA_ADR_REF.to_owned(),
                vocabulary_doc_ref: FILESYSTEM_IDENTITY_BETA_VOCABULARY_DOC_REF.to_owned(),
                beta_doc_ref: FILESYSTEM_IDENTITY_BETA_DOC_REF.to_owned(),
                alpha_fixture_ref: None,
                recovery_ladder_alpha_ref: None,
            },
            captured_at: "2026-05-16T00:00:00Z".to_owned(),
            reviewer_summary: None,
        }
    }

    #[test]
    fn direct_case_passes_validation() {
        let case = direct_case();
        FilesystemIdentityBetaEvaluator::new()
            .validate_case(&case)
            .expect("direct case must validate");
    }

    #[test]
    fn refuses_misaligned_support_export_refs() {
        let mut case = direct_case();
        case.support_export_alignment.editor_file_identity_ref = "fsid:other".to_owned();
        let err = FilesystemIdentityBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("misaligned refs must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.support_export_alignment.refs_disagree"));
    }

    #[test]
    fn refuses_destructive_reset() {
        let mut case = direct_case();
        case.safety.destructive_resets_present = true;
        let err = FilesystemIdentityBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("destructive reset must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.safety.destructive_resets_present"));
    }

    #[test]
    fn refuses_dropped_user_files_preservation() {
        let mut case = direct_case();
        case.safety.preserves_user_authored_files = false;
        let err = FilesystemIdentityBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("dropped user-files preservation must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.safety.preserves_user_authored_files"));
    }

    #[test]
    fn refuses_admitted_raw_private_material() {
        let mut case = direct_case();
        case.safety.raw_private_material_excluded = false;
        let err = FilesystemIdentityBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("admitted raw private material must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.safety.raw_private_material_excluded"));
    }

    #[test]
    fn refuses_writes_to_non_canonical_uri() {
        let mut case = direct_case();
        case.save_target_review.writes_to_canonical_uri = "file:///ws/elsewhere".to_owned();
        let err = FilesystemIdentityBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("non-canonical write target must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| { v.check_id == "case.save_target_review.writes_to_canonical_uri_mismatch" }));
    }

    #[test]
    fn refuses_write_action_on_external_change() {
        let mut case = direct_case();
        case.conflict_resolution.compare_outcome = BetaCompareOutcome::ExternalChangeDetected;
        case.conflict_resolution.review_required = true;
        case.conflict_resolution.silent_overwrite_forbidden = true;
        case.conflict_resolution.resolution_actions = vec![
            BetaResolutionAction::Compare,
            BetaResolutionAction::Write,
            BetaResolutionAction::Cancel,
        ];
        let err = FilesystemIdentityBetaEvaluator::new()
            .validate_case(&case)
            .expect_err("write action on external change must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.conflict_resolution.write_only_when_unchanged"));
    }

    #[test]
    fn checked_in_corpus_loads_and_validates() {
        let corpus =
            current_filesystem_identity_beta_corpus().expect("checked-in corpus must parse");
        FilesystemIdentityBetaEvaluator::new()
            .validate_corpus(&corpus)
            .expect("checked-in corpus must validate");
        for required in REQUIRED_DIFFICULTY_CLASSES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.case.difficulty_class == required),
                "missing required difficulty class {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn support_packet_is_export_safe() {
        let corpus = current_filesystem_identity_beta_corpus().unwrap();
        let packet = FilesystemIdentityBetaEvaluator::new()
            .support_packet("packet:test", "2026-05-16T00:00:00Z", &corpus)
            .expect("packet must build");
        assert!(packet.is_export_safe());
        assert_eq!(packet.cases.len(), corpus.entries.len());
        assert_eq!(packet.doc_ref, FILESYSTEM_IDENTITY_BETA_DOC_REF);
        assert_eq!(packet.schema_ref, FILESYSTEM_IDENTITY_BETA_SCHEMA_REF);
    }
}
