//! Two reusable M5 compare-and-export primitives — the retention/export *card* and the
//! history-export *manifest* — so a local-history, refactor, import, AI-apply, recovery, or
//! support flow keeps its recovery baselines and outbound artifacts explicit: what survives,
//! what expires, what is metadata-only, which baseline a diff is measured against
//! (current-versus-snapshot, snapshot-versus-disk, or snapshot-versus-Git HEAD), and how
//! redaction shapes any patch or evidence export — never a bare "download" that hides the
//! baseline, the scope, or the redaction posture behind it.
//!
//! Aureline's frozen local-history / write-scope component matrix
//! ([`crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix`])
//! names the retention/export card and the history-export manifest as two governed component
//! families and freezes their controlled vocabulary — the retention postures, the
//! export-redaction postures, the export-manifest classes, the surface families, the
//! deployment lines, the consumer surfaces, the accessibility routes, the qualification
//! classes, and the downgrade triggers. This module *implements* that contract as two reusable
//! primitives so a user can tell — from the card and the manifest alone — how long a checkpoint
//! survives, whether it is purge-pending, expired, or metadata-only, which cross-baseline
//! comparisons are on offer, whether an outbound patch or evidence bundle can be exported at
//! all, how redaction scrubs it, whether its actor lineage, checkpoint identity, and scope are
//! preserved, and — before any share — that no export defaults to a raw secret-bearing content
//! body.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_retention_export_card`] — takes one card's retention posture, export-redaction
//!    posture, the cross-baseline comparisons the underlying surface / artifact class supports,
//!    whether comparison is available, whether the card is metadata-only, whether the export
//!    path is ready, and its opaque card label, and produces one
//!    [`M5ResolvedRetentionExportCard`] carrying the derived card posture (fully-shareable
//!    versus metadata-only versus purge-scheduled versus policy-restricted versus
//!    nothing-retained versus export-blocked), the available cross-baseline comparisons,
//!    whether an export can commit, and the bounded inspect-retention / review-redaction /
//!    compare-baseline / export-patch / export-evidence / request-extension actions. It always
//!    discloses its retention and redaction posture and never hides the compare baseline.
//! 2. [`resolve_history_export_manifest`] — takes one manifest's class, export-redaction
//!    posture, primary compare baseline, whether it preserves actor lineage / checkpoint
//!    identity / scope, whether it would carry raw content bodies, whether the export path is
//!    ready, and its opaque manifest label, and produces one
//!    [`M5ResolvedHistoryExportManifest`] carrying the derived manifest disposition
//!    (full-evidence versus redacted-share versus policy-restricted versus lineage-incomplete
//!    versus raw-body-withheld versus export-blocked), whether the manifest is shareable, and
//!    the bounded inspect-manifest / view-lineage / review-redaction / export-manifest /
//!    request-unredacted actions. It always keeps its primary baseline explicit and never
//!    defaults to a raw secret-bearing content body — a manifest that would carry one is held
//!    back, not shared.
//!
//! A single parity matrix — [`M5CompareExportPacket`] — binds one row per claimed M5 history /
//! recovery consumer (local-history timeline, refactor evidence, import/migration session,
//! AI-apply evidence, recovery center, and support export desk) to the shared card and manifest
//! anatomy, the same retention postures, export-redaction postures, manifest classes, compare
//! baselines, card postures, manifest dispositions, bounded actions, export fields, and
//! non-visual accessibility routes, so the baseline / retention / redaction vocabulary stays
//! identical across history, refactor, import, AI, recovery, and support surfaces without ever
//! collapsing an export into a generic download.
//!
//! The retention posture ([`M5RetentionPosture`]), export-redaction posture
//! ([`M5ExportRedactionPosture`]), export-manifest class ([`M5ExportManifestClass`]), surface
//! family ([`M5HistorySurfaceFamily`]), deployment line ([`M5HistoryDeploymentLine`]), consumer
//! surface ([`M5HistoryConsumerSurface`]), accessibility route
//! ([`M5HistoryAccessibilityRoute`]), qualification class ([`M5HistoryQualificationClass`]),
//! and downgrade trigger ([`M5HistoryDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! card and the manifest themselves: their compare-export consumers, their cross-baseline
//! comparisons, their anatomy parts, their derived card posture, their derived manifest
//! disposition, their bounded actions, and their export fields. No M5 compare / export surface
//! invents a second baseline or redaction grammar.
//!
//! Raw checkpoint bodies, diffs, pasted paths, credentials, and private endpoints stay outside
//! the support boundary; every card identity, manifest identity, and baseline descriptor is
//! carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_compare_export_ai_apply_evidence_beta_narrowed,
    seeded_m5_compare_export_import_migration_session_preview_narrowed,
    seeded_m5_compare_export_packet, M5_COMPARE_EXPORT_PACKET_ID,
};

// The retention posture, export-redaction posture, export-manifest class, surface family,
// deployment line, consumer surface, accessibility route, qualification class, and downgrade
// triggers are frozen once, in the local-history / write-scope component matrix. These
// primitives reuse them verbatim so they never invent a parallel retention / redaction
// vocabulary.
pub use crate::freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix::{
    M5ExportManifestClass, M5ExportRedactionPosture, M5HistoryAccessibilityRoute,
    M5HistoryConsumerSurface, M5HistoryDeploymentLine, M5HistoryDowngradeTrigger,
    M5HistoryQualificationClass, M5HistorySurfaceFamily, M5RetentionPosture,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CompareExportPacket`].
pub const M5_COMPARE_EXPORT_RECORD_KIND: &str =
    "ship_m5_cross_baseline_compare_and_export_flows_so_current_versus_snapshot_snapshot_versus_disk_snapshot_versus_git_and_patch_or_evidence_export_stay_explicit_across_claimed_m5_history_refactor_import_ai_paths";

/// Schema version for M5 compare-export records.
pub const M5_COMPARE_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the retention/export-card boundary schema.
pub const M5_COMPARE_EXPORT_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-retention-export-card.schema.json";

/// Repo-relative path of the history-export-manifest boundary schema.
pub const M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF: &str =
    "schemas/ui/m5-history-export-manifest.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COMPARE_EXPORT_DOC_REF: &str = "docs/recovery/m5_retention_export_card_primitive.md";

/// Repo-relative path of the frozen local-history / write-scope component matrix these
/// primitives narrow from.
pub const M5_COMPARE_EXPORT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-local-history-write-scope-component-matrix.schema.json";

/// Repo-relative path of the local-history retention-card contract this primitive binds its
/// retention and export-redaction truth against.
pub const M5_COMPARE_EXPORT_RETENTION_CARD_REF: &str =
    "schemas/recovery/local_history_retention_card.schema.json";

/// Repo-relative path of the Git history-review contract this primitive binds its
/// snapshot-versus-Git compare baseline against.
pub const M5_COMPARE_EXPORT_GIT_HISTORY_REF: &str = "schemas/git/git_history_review.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_COMPARE_EXPORT_FIXTURE_DIR: &str = "fixtures/ui/m5-retention-export-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COMPARE_EXPORT_ARTIFACT_REF: &str =
    "artifacts/release/m5-retention-export-card-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_COMPARE_EXPORT_CSV_REF: &str =
    "artifacts/release/m5-retention-export-card-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COMPARE_EXPORT_REPORT_REF: &str =
    "artifacts/design/m5-retention-export-card-primitive.md";

/// One claimed M5 compare / export consumer that renders the shared retention/export card and
/// history-export manifest. These are the consumers the acceptance criteria name — a
/// local-history, refactor, import, AI-apply, recovery, or support flow — so the same card and
/// manifest grammar works across every claimed history / recovery surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompareExportConsumerSurface {
    /// The editor local-history timeline surface.
    LocalHistoryTimeline,
    /// The refactor-transaction evidence surface.
    RefactorEvidence,
    /// The importer / migration-session surface.
    ImportMigrationSession,
    /// The AI-apply evidence surface.
    AiApplyEvidence,
    /// The recovery-center surface.
    RecoveryCenter,
    /// The support export-desk surface.
    SupportExportDesk,
}

impl M5CompareExportConsumerSurface {
    /// Every claimed compare / export consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalHistoryTimeline,
        Self::RefactorEvidence,
        Self::ImportMigrationSession,
        Self::AiApplyEvidence,
        Self::RecoveryCenter,
        Self::SupportExportDesk,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHistoryTimeline => "local_history_timeline",
            Self::RefactorEvidence => "refactor_evidence",
            Self::ImportMigrationSession => "import_migration_session",
            Self::AiApplyEvidence => "ai_apply_evidence",
            Self::RecoveryCenter => "recovery_center",
            Self::SupportExportDesk => "support_export_desk",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalHistoryTimeline => "Local History Timeline",
            Self::RefactorEvidence => "Refactor Evidence",
            Self::ImportMigrationSession => "Import/Migration Session",
            Self::AiApplyEvidence => "AI Apply Evidence",
            Self::RecoveryCenter => "Recovery Center",
            Self::SupportExportDesk => "Support Export Desk",
        }
    }
}

/// The cross-baseline comparison a compare/export surface can make explicit, so a diff always
/// names which baseline it is measured against instead of leaving the user to infer whether a
/// change is versus the live buffer, the snapshot, the on-disk file, or Git HEAD.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompareBaseline {
    /// The current working buffer versus a saved snapshot.
    CurrentVsSnapshot,
    /// A snapshot versus the current on-disk file (external drift).
    SnapshotVsDisk,
    /// A snapshot versus Git HEAD.
    SnapshotVsGitHead,
    /// A snapshot versus another snapshot on the timeline.
    SnapshotVsSnapshot,
}

impl M5CompareBaseline {
    /// Every compare baseline, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CurrentVsSnapshot,
        Self::SnapshotVsDisk,
        Self::SnapshotVsGitHead,
        Self::SnapshotVsSnapshot,
    ];

    /// The three baselines the acceptance criteria name explicitly.
    pub const NAMED: [Self; 3] = [
        Self::CurrentVsSnapshot,
        Self::SnapshotVsDisk,
        Self::SnapshotVsGitHead,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentVsSnapshot => "current_vs_snapshot",
            Self::SnapshotVsDisk => "snapshot_vs_disk",
            Self::SnapshotVsGitHead => "snapshot_vs_git_head",
            Self::SnapshotVsSnapshot => "snapshot_vs_snapshot",
        }
    }
}

/// The derived posture of a retention/export card — the resolver's verdict about what survives,
/// what expires, and whether an export can commit. Computed in a fixed blocking-first order, so
/// an export-blocked, expired, or policy-restricted card never reads as a fully-shareable one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetentionExportCardPosture {
    /// Full metadata is retained and shareable.
    FullyShareable,
    /// Only metadata survives (bodies omitted or session-only retention).
    MetadataOnlySurvives,
    /// A purge is scheduled; export before it purges.
    PurgeScheduled,
    /// Export is gated by policy.
    PolicyRestricted,
    /// Nothing is retained (expired and purged).
    NothingRetained,
    /// Export is unavailable on this surface.
    ExportBlocked,
}

impl M5RetentionExportCardPosture {
    /// Every card posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullyShareable,
        Self::MetadataOnlySurvives,
        Self::PurgeScheduled,
        Self::PolicyRestricted,
        Self::NothingRetained,
        Self::ExportBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyShareable => "fully_shareable",
            Self::MetadataOnlySurvives => "metadata_only_survives",
            Self::PurgeScheduled => "purge_scheduled",
            Self::PolicyRestricted => "policy_restricted",
            Self::NothingRetained => "nothing_retained",
            Self::ExportBlocked => "export_blocked",
        }
    }

    /// True when a card at this posture can still commit an export.
    pub const fn can_export(self) -> bool {
        !matches!(self, Self::ExportBlocked | Self::NothingRetained)
    }

    /// True when the card needs operator attention before an export commits.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::PurgeScheduled
                | Self::PolicyRestricted
                | Self::NothingRetained
                | Self::ExportBlocked
        )
    }
}

/// One bounded action a retention/export card offers, so a card never hides its inspect /
/// review-redaction / compare-baseline / export / request-extension affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetentionExportCardAction {
    /// Inspect the retention posture (inspect-only, never mutating).
    InspectRetention,
    /// Review how redaction shapes any export.
    ReviewRedaction,
    /// Compare across an available cross-baseline.
    CompareBaseline,
    /// Export a redaction-shaped patch.
    ExportPatch,
    /// Export a redaction-shaped evidence bundle.
    ExportEvidence,
    /// Request a retention extension before a purge.
    RequestRetentionExtension,
}

impl M5RetentionExportCardAction {
    /// Every card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectRetention,
        Self::ReviewRedaction,
        Self::CompareBaseline,
        Self::ExportPatch,
        Self::ExportEvidence,
        Self::RequestRetentionExtension,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectRetention => "inspect_retention",
            Self::ReviewRedaction => "review_redaction",
            Self::CompareBaseline => "compare_baseline",
            Self::ExportPatch => "export_patch",
            Self::ExportEvidence => "export_evidence",
            Self::RequestRetentionExtension => "request_retention_extension",
        }
    }
}

/// Controlled retention/export-card anatomy part the shared card surfaces. The parts in
/// [`M5RetentionExportCardAnatomyPart::MANDATORY`] are required on every card so the retention
/// posture, redaction posture, compare baseline, survival summary, and action row are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetentionExportCardAnatomyPart {
    /// The retention-posture cue.
    RetentionPostureCue,
    /// The export-redaction cue.
    ExportRedactionCue,
    /// The cross-baseline-compare cue.
    BaselineCompareCue,
    /// The what-survives summary cue.
    SurvivalSummaryCue,
    /// The what-expires cue.
    ExpiryCue,
    /// The metadata-only cue.
    MetadataOnlyCue,
    /// The bounded action row (inspect / review / compare / export / request).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5RetentionExportCardAnatomyPart {
    /// Every card anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RetentionPostureCue,
        Self::ExportRedactionCue,
        Self::BaselineCompareCue,
        Self::SurvivalSummaryCue,
        Self::ExpiryCue,
        Self::MetadataOnlyCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The card anatomy parts every card must render.
    pub const MANDATORY: [Self; 5] = [
        Self::RetentionPostureCue,
        Self::ExportRedactionCue,
        Self::BaselineCompareCue,
        Self::SurvivalSummaryCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetentionPostureCue => "retention_posture_cue",
            Self::ExportRedactionCue => "export_redaction_cue",
            Self::BaselineCompareCue => "baseline_compare_cue",
            Self::SurvivalSummaryCue => "survival_summary_cue",
            Self::ExpiryCue => "expiry_cue",
            Self::MetadataOnlyCue => "metadata_only_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the card export carries so retention/export-card truth is reconstructable. The
/// fields in [`M5RetentionExportCardExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetentionExportCardExportField {
    /// The opaque card label.
    CardLabel,
    /// The retention posture.
    RetentionPosture,
    /// The export-redaction posture.
    ExportRedaction,
    /// The derived card posture.
    CardPosture,
    /// The available cross-baseline comparisons.
    AvailableBaselines,
    /// Whether the export can commit.
    CanExport,
    /// Whether the card needs attention.
    NeedsAttention,
    /// The bounded available actions.
    AvailableActions,
}

impl M5RetentionExportCardExportField {
    /// Every card export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CardLabel,
        Self::RetentionPosture,
        Self::ExportRedaction,
        Self::CardPosture,
        Self::AvailableBaselines,
        Self::CanExport,
        Self::NeedsAttention,
        Self::AvailableActions,
    ];

    /// The card export fields every card must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::CardLabel,
        Self::RetentionPosture,
        Self::ExportRedaction,
        Self::CardPosture,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CardLabel => "card_label",
            Self::RetentionPosture => "retention_posture",
            Self::ExportRedaction => "export_redaction",
            Self::CardPosture => "card_posture",
            Self::AvailableBaselines => "available_baselines",
            Self::CanExport => "can_export",
            Self::NeedsAttention => "needs_attention",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// The derived disposition of a history-export manifest — the resolver's verdict about whether
/// the manifest can be shared, and why not. Computed in a fixed blocking-first order, so an
/// export-blocked, raw-body-bearing, lineage-incomplete, or policy-restricted manifest never
/// reads as a full-evidence bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExportManifestDisposition {
    /// A full-evidence bundle with lineage, identity, and scope preserved.
    FullEvidence,
    /// A properly redaction-shaped share.
    RedactedShare,
    /// A manifest gated by policy.
    PolicyRestricted,
    /// A manifest whose actor lineage, checkpoint identity, or scope is not fully preserved.
    LineageIncomplete,
    /// A manifest that would carry raw content bodies and is held back.
    RawBodyWithheld,
    /// A manifest whose export path is unavailable.
    ExportBlocked,
}

impl M5ExportManifestDisposition {
    /// Every manifest disposition, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullEvidence,
        Self::RedactedShare,
        Self::PolicyRestricted,
        Self::LineageIncomplete,
        Self::RawBodyWithheld,
        Self::ExportBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullEvidence => "full_evidence",
            Self::RedactedShare => "redacted_share",
            Self::PolicyRestricted => "policy_restricted",
            Self::LineageIncomplete => "lineage_incomplete",
            Self::RawBodyWithheld => "raw_body_withheld",
            Self::ExportBlocked => "export_blocked",
        }
    }

    /// True when a manifest at this disposition can be shared.
    pub const fn is_shareable(self) -> bool {
        matches!(self, Self::FullEvidence | Self::RedactedShare)
    }

    /// True when the manifest needs operator attention before it can be shared.
    pub const fn needs_attention(self) -> bool {
        !matches!(self, Self::FullEvidence)
    }
}

/// One bounded action a history-export manifest offers, so lineage stays inspectable, the
/// redaction posture stays reviewable, and an unredacted export is an explicit request rather
/// than a default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExportManifestAction {
    /// Inspect the manifest contents (always available).
    InspectManifest,
    /// Inspect the actor lineage and checkpoint identity (always available).
    ViewLineage,
    /// Review the redaction posture (always available).
    ReviewRedaction,
    /// Export the shareable manifest.
    ExportManifest,
    /// Request an unredacted export through the approval path.
    RequestUnredactedExport,
}

impl M5ExportManifestAction {
    /// Every manifest action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InspectManifest,
        Self::ViewLineage,
        Self::ReviewRedaction,
        Self::ExportManifest,
        Self::RequestUnredactedExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectManifest => "inspect_manifest",
            Self::ViewLineage => "view_lineage",
            Self::ReviewRedaction => "review_redaction",
            Self::ExportManifest => "export_manifest",
            Self::RequestUnredactedExport => "request_unredacted_export",
        }
    }
}

/// Controlled history-export-manifest anatomy part the shared manifest surfaces. The parts in
/// [`M5ExportManifestAnatomyPart::MANDATORY`] are required on every manifest so the manifest
/// class, redaction posture, compare baseline, actor lineage, and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExportManifestAnatomyPart {
    /// The manifest-class cue.
    ManifestClassCue,
    /// The export-redaction cue.
    ExportRedactionCue,
    /// The primary-baseline cue.
    BaselineCue,
    /// The actor-lineage cue.
    ActorLineageCue,
    /// The checkpoint-identity cue.
    CheckpointIdentityCue,
    /// The scope cue.
    ScopeCue,
    /// The bounded action row.
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5ExportManifestAnatomyPart {
    /// Every manifest anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ManifestClassCue,
        Self::ExportRedactionCue,
        Self::BaselineCue,
        Self::ActorLineageCue,
        Self::CheckpointIdentityCue,
        Self::ScopeCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The manifest anatomy parts every manifest must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ManifestClassCue,
        Self::ExportRedactionCue,
        Self::BaselineCue,
        Self::ActorLineageCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestClassCue => "manifest_class_cue",
            Self::ExportRedactionCue => "export_redaction_cue",
            Self::BaselineCue => "baseline_cue",
            Self::ActorLineageCue => "actor_lineage_cue",
            Self::CheckpointIdentityCue => "checkpoint_identity_cue",
            Self::ScopeCue => "scope_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the manifest export carries so history-export-manifest truth is reconstructable. The
/// fields in [`M5ExportManifestExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExportManifestExportField {
    /// The opaque manifest label.
    ManifestLabel,
    /// The manifest class.
    ManifestClass,
    /// The export-redaction posture.
    ExportRedaction,
    /// The primary compare baseline.
    PrimaryBaseline,
    /// The derived manifest disposition.
    ManifestDisposition,
    /// Whether the manifest is shareable.
    IsShareable,
    /// Whether the manifest omits raw bodies (always true).
    OmitsRawBodies,
    /// The bounded available actions.
    AvailableActions,
}

impl M5ExportManifestExportField {
    /// Every manifest export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ManifestLabel,
        Self::ManifestClass,
        Self::ExportRedaction,
        Self::PrimaryBaseline,
        Self::ManifestDisposition,
        Self::IsShareable,
        Self::OmitsRawBodies,
        Self::AvailableActions,
    ];

    /// The manifest export fields every manifest must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ManifestLabel,
        Self::ManifestClass,
        Self::PrimaryBaseline,
        Self::ManifestDisposition,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestLabel => "manifest_label",
            Self::ManifestClass => "manifest_class",
            Self::ExportRedaction => "export_redaction",
            Self::PrimaryBaseline => "primary_baseline",
            Self::ManifestDisposition => "manifest_disposition",
            Self::IsShareable => "is_shareable",
            Self::OmitsRawBodies => "omits_raw_bodies",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when an export-redaction posture removes or restricts something from the export (any
/// posture other than a full-metadata export).
pub const fn redaction_is_restrictive(redaction: M5ExportRedactionPosture) -> bool {
    !matches!(redaction, M5ExportRedactionPosture::FullMetadata)
}

/// True when a retention posture will (or already did) drop history — session-only,
/// purge-pending, or expired.
pub const fn retention_is_expiring(retention: M5RetentionPosture) -> bool {
    matches!(
        retention,
        M5RetentionPosture::SessionOnly
            | M5RetentionPosture::PurgePending
            | M5RetentionPosture::ExpiredPurged
    )
}

// ---- retention/export-card resolver -------------------------------------

/// The full input to the retention/export-card resolver for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetentionExportCardResolutionInput {
    /// The retention posture the card carries.
    pub retention_posture: M5RetentionPosture,
    /// The export-redaction posture the card carries.
    pub export_redaction: M5ExportRedactionPosture,
    /// The cross-baseline comparisons this surface / artifact class supports.
    pub supported_baselines: Vec<M5CompareBaseline>,
    /// True when cross-baseline comparison is available on this surface.
    pub baseline_comparison_available: bool,
    /// True when only metadata survives (bodies are not retained).
    pub is_metadata_only: bool,
    /// True when the export path for this card is available.
    pub export_path_ready: bool,
    /// The opaque card label (must be non-empty).
    pub card_label: String,
}

/// The resolved retention/export-card truth for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRetentionExportCard {
    /// The retention posture the card carries.
    pub retention_posture: M5RetentionPosture,
    /// The export-redaction posture the card carries.
    pub export_redaction: M5ExportRedactionPosture,
    /// The opaque card label, preserved exactly from the input.
    pub card_label: String,
    /// The derived card posture.
    pub card_posture: M5RetentionExportCardPosture,
    /// The cross-baseline comparisons on offer (empty when comparison is unavailable).
    pub available_baselines: Vec<M5CompareBaseline>,
    /// The bounded actions this card offers.
    pub available_actions: Vec<M5RetentionExportCardAction>,
    /// True when an export can commit.
    pub can_export: bool,
    /// True when a cross-baseline comparison is offered.
    pub baseline_comparison_offered: bool,
    /// True when the card needs operator attention before an export commits.
    pub needs_attention: bool,
    /// Always true: the card discloses its retention and redaction posture.
    pub discloses_retention_and_redaction: bool,
    /// Always false: the card never hides the compare baseline behind a generic export.
    pub hides_compare_baseline: bool,
}

/// Errors returned by [`resolve_retention_export_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RetentionExportCardResolutionError {
    /// The card label was empty.
    EmptyCardLabel,
    /// A card descriptor carried forbidden material.
    ForbiddenCardMaterial,
}

impl M5RetentionExportCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyCardLabel => "empty_card_label",
            Self::ForbiddenCardMaterial => "forbidden_card_material",
        }
    }
}

impl fmt::Display for M5RetentionExportCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "retention export card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RetentionExportCardResolutionError {}

/// Resolves one retention/export card from its declared retention and export state.
///
/// The derived card posture is computed in a fixed blocking-first order: an unavailable export
/// path or a blocked redaction posture wins first, then a fully-expired retention with nothing
/// left, then a policy-restricted export, then a purge-pending retention, then a metadata-only
/// survival, and otherwise a fully-shareable card. The available cross-baseline comparisons are
/// carried only when comparison is available, so the card never claims a compare path it does
/// not have — and always discloses its retention and redaction posture.
pub fn resolve_retention_export_card(
    input: &M5RetentionExportCardResolutionInput,
) -> Result<M5ResolvedRetentionExportCard, M5RetentionExportCardResolutionError> {
    if input.card_label.trim().is_empty() {
        return Err(M5RetentionExportCardResolutionError::EmptyCardLabel);
    }
    if value_repr_is_forbidden(&input.card_label) {
        return Err(M5RetentionExportCardResolutionError::ForbiddenCardMaterial);
    }

    let card_posture = derive_card_posture(
        input.retention_posture,
        input.export_redaction,
        input.is_metadata_only,
        input.export_path_ready,
    );
    let available_baselines = if input.baseline_comparison_available {
        input.supported_baselines.clone()
    } else {
        Vec::new()
    };
    let baseline_comparison_offered = !available_baselines.is_empty();
    let can_export = card_posture.can_export();
    let available_actions =
        derive_card_actions(card_posture, can_export, baseline_comparison_offered);

    Ok(M5ResolvedRetentionExportCard {
        retention_posture: input.retention_posture,
        export_redaction: input.export_redaction,
        card_label: input.card_label.clone(),
        card_posture,
        available_baselines,
        available_actions,
        can_export,
        baseline_comparison_offered,
        needs_attention: card_posture.needs_attention(),
        discloses_retention_and_redaction: true,
        hides_compare_baseline: false,
    })
}

/// The fixed blocking-first card-posture ladder.
fn derive_card_posture(
    retention: M5RetentionPosture,
    redaction: M5ExportRedactionPosture,
    is_metadata_only: bool,
    export_path_ready: bool,
) -> M5RetentionExportCardPosture {
    if !export_path_ready || matches!(redaction, M5ExportRedactionPosture::ExportBlocked) {
        M5RetentionExportCardPosture::ExportBlocked
    } else if matches!(retention, M5RetentionPosture::ExpiredPurged) {
        M5RetentionExportCardPosture::NothingRetained
    } else if matches!(redaction, M5ExportRedactionPosture::PolicyRestricted) {
        M5RetentionExportCardPosture::PolicyRestricted
    } else if matches!(retention, M5RetentionPosture::PurgePending) {
        M5RetentionExportCardPosture::PurgeScheduled
    } else if is_metadata_only
        || matches!(redaction, M5ExportRedactionPosture::BodiesOmitted)
        || matches!(retention, M5RetentionPosture::SessionOnly)
    {
        M5RetentionExportCardPosture::MetadataOnlySurvives
    } else {
        M5RetentionExportCardPosture::FullyShareable
    }
}

/// Derives the bounded card action set from the posture and export / comparison signals.
///
/// Inspect-retention and review-redaction are always offered so the retention and redaction
/// truth is always inspectable; compare-baseline follows the comparison-offered signal;
/// export-patch and export-evidence follow the appliable state; request-extension is offered
/// only for a purge-scheduled or nothing-retained card.
fn derive_card_actions(
    posture: M5RetentionExportCardPosture,
    can_export: bool,
    baseline_comparison_offered: bool,
) -> Vec<M5RetentionExportCardAction> {
    use M5RetentionExportCardAction as Action;
    let mut actions = vec![Action::InspectRetention, Action::ReviewRedaction];
    if baseline_comparison_offered {
        actions.push(Action::CompareBaseline);
    }
    if can_export {
        actions.push(Action::ExportPatch);
        actions.push(Action::ExportEvidence);
    }
    if matches!(
        posture,
        M5RetentionExportCardPosture::PurgeScheduled
            | M5RetentionExportCardPosture::NothingRetained
    ) {
        actions.push(Action::RequestRetentionExtension);
    }
    actions
}

// ---- history-export-manifest resolver -----------------------------------

/// The full input to the history-export-manifest resolver for one manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryExportManifestResolutionInput {
    /// The manifest class.
    pub manifest_class: M5ExportManifestClass,
    /// The export-redaction posture the manifest carries.
    pub export_redaction: M5ExportRedactionPosture,
    /// The primary compare baseline the manifest's diffs are measured against.
    pub primary_baseline: M5CompareBaseline,
    /// True when the manifest preserves actor lineage.
    pub preserves_actor_lineage: bool,
    /// True when the manifest preserves checkpoint identity.
    pub preserves_checkpoint_identity: bool,
    /// True when the manifest preserves the restore / apply scope.
    pub preserves_scope: bool,
    /// True when the manifest would carry raw content bodies.
    pub includes_raw_bodies: bool,
    /// True when the export path for this manifest is available.
    pub export_path_ready: bool,
    /// The opaque manifest label (must be non-empty).
    pub manifest_label: String,
}

/// The resolved history-export-manifest truth for one manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedHistoryExportManifest {
    /// The manifest class.
    pub manifest_class: M5ExportManifestClass,
    /// The export-redaction posture the manifest carries.
    pub export_redaction: M5ExportRedactionPosture,
    /// The primary compare baseline the manifest's diffs are measured against.
    pub primary_baseline: M5CompareBaseline,
    /// The opaque manifest label, preserved exactly from the input.
    pub manifest_label: String,
    /// The derived manifest disposition.
    pub manifest_disposition: M5ExportManifestDisposition,
    /// The bounded actions this manifest offers.
    pub available_actions: Vec<M5ExportManifestAction>,
    /// True when the manifest is shareable.
    pub is_shareable: bool,
    /// True when the manifest needs operator attention before it can be shared.
    pub needs_attention: bool,
    /// True when the manifest preserves actor lineage.
    pub preserves_actor_lineage: bool,
    /// True when the manifest preserves checkpoint identity.
    pub preserves_checkpoint_identity: bool,
    /// True when the manifest preserves the restore / apply scope.
    pub preserves_scope: bool,
    /// Always true: no shareable manifest defaults to a raw secret-bearing content body.
    pub omits_raw_bodies: bool,
    /// Always true: the manifest keeps its primary baseline explicit.
    pub baseline_is_explicit: bool,
    /// Always false: the manifest never collapses its export into a generic download.
    pub flattens_into_generic_download: bool,
}

/// Errors returned by [`resolve_history_export_manifest`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5HistoryExportManifestResolutionError {
    /// The manifest label was empty.
    EmptyManifestLabel,
    /// A manifest descriptor carried forbidden material.
    ForbiddenManifestMaterial,
}

impl M5HistoryExportManifestResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyManifestLabel => "empty_manifest_label",
            Self::ForbiddenManifestMaterial => "forbidden_manifest_material",
        }
    }
}

impl fmt::Display for M5HistoryExportManifestResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "history export manifest resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5HistoryExportManifestResolutionError {}

/// Resolves one history-export manifest from its declared class and export state.
///
/// The derived manifest disposition is computed in a fixed blocking-first order: an unavailable
/// export path or a blocked redaction posture wins first, then a manifest that would carry raw
/// content bodies is held back, then an incomplete actor lineage / checkpoint identity / scope,
/// then a policy-restricted export, then a properly redaction-shaped share, and otherwise a
/// full-evidence bundle. A manifest is shareable only when it is full-evidence or a redacted
/// share, and it always keeps its primary baseline explicit; no shareable manifest ever
/// defaults to a raw secret-bearing content body.
pub fn resolve_history_export_manifest(
    input: &M5HistoryExportManifestResolutionInput,
) -> Result<M5ResolvedHistoryExportManifest, M5HistoryExportManifestResolutionError> {
    if input.manifest_label.trim().is_empty() {
        return Err(M5HistoryExportManifestResolutionError::EmptyManifestLabel);
    }
    if value_repr_is_forbidden(&input.manifest_label) {
        return Err(M5HistoryExportManifestResolutionError::ForbiddenManifestMaterial);
    }

    let manifest_disposition = derive_manifest_disposition(
        input.manifest_class,
        input.export_redaction,
        input.preserves_actor_lineage,
        input.preserves_checkpoint_identity,
        input.preserves_scope,
        input.includes_raw_bodies,
        input.export_path_ready,
    );
    let is_shareable = manifest_disposition.is_shareable();
    let available_actions = derive_manifest_actions(manifest_disposition, is_shareable);

    Ok(M5ResolvedHistoryExportManifest {
        manifest_class: input.manifest_class,
        export_redaction: input.export_redaction,
        primary_baseline: input.primary_baseline,
        manifest_label: input.manifest_label.clone(),
        manifest_disposition,
        available_actions,
        is_shareable,
        needs_attention: manifest_disposition.needs_attention(),
        preserves_actor_lineage: input.preserves_actor_lineage,
        preserves_checkpoint_identity: input.preserves_checkpoint_identity,
        preserves_scope: input.preserves_scope,
        omits_raw_bodies: true,
        baseline_is_explicit: true,
        flattens_into_generic_download: false,
    })
}

/// The fixed blocking-first manifest-disposition ladder.
#[allow(clippy::too_many_arguments)]
fn derive_manifest_disposition(
    manifest_class: M5ExportManifestClass,
    redaction: M5ExportRedactionPosture,
    preserves_actor_lineage: bool,
    preserves_checkpoint_identity: bool,
    preserves_scope: bool,
    includes_raw_bodies: bool,
    export_path_ready: bool,
) -> M5ExportManifestDisposition {
    if !export_path_ready || matches!(redaction, M5ExportRedactionPosture::ExportBlocked) {
        M5ExportManifestDisposition::ExportBlocked
    } else if includes_raw_bodies {
        M5ExportManifestDisposition::RawBodyWithheld
    } else if !preserves_actor_lineage || !preserves_checkpoint_identity || !preserves_scope {
        M5ExportManifestDisposition::LineageIncomplete
    } else if matches!(redaction, M5ExportRedactionPosture::PolicyRestricted) {
        M5ExportManifestDisposition::PolicyRestricted
    } else if matches!(
        redaction,
        M5ExportRedactionPosture::PathsRedacted
            | M5ExportRedactionPosture::BodiesOmitted
            | M5ExportRedactionPosture::CredentialsScrubbed
    ) || matches!(manifest_class, M5ExportManifestClass::RedactedShare)
    {
        M5ExportManifestDisposition::RedactedShare
    } else {
        M5ExportManifestDisposition::FullEvidence
    }
}

/// Derives the bounded manifest action set.
///
/// Inspect-manifest, view-lineage, and review-redaction are always offered so the manifest
/// contents, actor lineage, and redaction posture are always inspectable; export-manifest
/// follows the shareable state; request-unredacted-export is offered only when the manifest is
/// held back for a raw body, policy, or incomplete lineage.
fn derive_manifest_actions(
    disposition: M5ExportManifestDisposition,
    is_shareable: bool,
) -> Vec<M5ExportManifestAction> {
    use M5ExportManifestAction as Action;
    let mut actions = vec![
        Action::InspectManifest,
        Action::ViewLineage,
        Action::ReviewRedaction,
    ];
    if is_shareable {
        actions.push(Action::ExportManifest);
    }
    if matches!(
        disposition,
        M5ExportManifestDisposition::RawBodyWithheld
            | M5ExportManifestDisposition::PolicyRestricted
            | M5ExportManifestDisposition::LineageIncomplete
    ) {
        actions.push(Action::RequestUnredactedExport);
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked retention/export-card resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RetentionExportCardResolutionCase {
    /// The resolver input.
    pub input: M5RetentionExportCardResolutionInput,
    /// The resolved truth. Must equal `resolve_retention_export_card(&input)`.
    pub resolved: M5ResolvedRetentionExportCard,
}

impl M5RetentionExportCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RetentionExportCardResolutionInput) -> Self {
        let resolved = resolve_retention_export_card(&input).expect("seed card case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_retention_export_card(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved card label preserves the input label exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.card_label == self.input.card_label
    }
}

/// One worked history-export-manifest resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoryExportManifestResolutionCase {
    /// The resolver input.
    pub input: M5HistoryExportManifestResolutionInput,
    /// The resolved truth. Must equal `resolve_history_export_manifest(&input)`.
    pub resolved: M5ResolvedHistoryExportManifest,
}

impl M5HistoryExportManifestResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5HistoryExportManifestResolutionInput) -> Self {
        let resolved =
            resolve_history_export_manifest(&input).expect("seed manifest case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_history_export_manifest(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved manifest label preserves the input label exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.manifest_label == self.input.manifest_label
    }
}

/// One row in the primitive matrix: one compare / export consumer bound to the shared card and
/// manifest anatomy, retention postures, export-redaction postures, manifest classes, compare
/// baselines, card postures, manifest dispositions, bounded actions, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompareExportRow {
    /// Compare / export consumer family.
    pub consumer_surface: M5CompareExportConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5HistoryQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 history / recovery surface families that render / consume these components.
    pub surface_families: Vec<M5HistorySurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5HistoryDeploymentLine>,
    /// Card anatomy parts this row renders (must include the mandatory parts).
    pub card_anatomy_parts: Vec<M5RetentionExportCardAnatomyPart>,
    /// Manifest anatomy parts this row renders (must include the mandatory parts).
    pub manifest_anatomy_parts: Vec<M5ExportManifestAnatomyPart>,
    /// Retention postures this consumer distinguishes.
    pub retention_postures: Vec<M5RetentionPosture>,
    /// Export-redaction postures this consumer distinguishes.
    pub export_redactions: Vec<M5ExportRedactionPosture>,
    /// Manifest classes this consumer distinguishes.
    pub manifest_classes: Vec<M5ExportManifestClass>,
    /// Compare baselines this consumer distinguishes.
    pub compare_baselines: Vec<M5CompareBaseline>,
    /// Card postures this consumer distinguishes.
    pub card_postures: Vec<M5RetentionExportCardPosture>,
    /// Manifest dispositions this consumer distinguishes.
    pub manifest_dispositions: Vec<M5ExportManifestDisposition>,
    /// Bounded card actions this consumer offers.
    pub card_actions: Vec<M5RetentionExportCardAction>,
    /// Bounded manifest actions this consumer offers.
    pub manifest_actions: Vec<M5ExportManifestAction>,
    /// Card export fields this row carries (must include the mandatory fields).
    pub card_export_fields: Vec<M5RetentionExportCardExportField>,
    /// Manifest export fields this row carries (must include the mandatory fields).
    pub manifest_export_fields: Vec<M5ExportManifestExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5HistoryAccessibilityRoute>,
    /// History / recovery subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5HistoryConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5HistoryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked retention/export-card resolutions proving the card resolver on this consumer.
    pub card_examples: Vec<M5RetentionExportCardResolutionCase>,
    /// Worked history-export-manifest resolutions proving the manifest resolver on this
    /// consumer.
    pub manifest_examples: Vec<M5HistoryExportManifestResolutionCase>,
    /// Hard invariant: this consumer never hides the export baseline. MUST be `false`.
    pub hides_export_baseline: bool,
    /// Hard invariant: this consumer never hides retention or redaction posture. MUST be
    /// `false`.
    pub hides_retention_or_redaction: bool,
    /// Hard invariant: this consumer never defaults to raw sensitive content bodies. MUST
    /// be `false`.
    pub defaults_to_raw_content_bodies: bool,
    /// Hard invariant: this consumer never collapses an export into a generic download. MUST be
    /// `false`.
    pub collapses_export_into_generic_download: bool,
}

impl M5CompareExportRow {
    /// True when the row declares every mandatory card anatomy part.
    fn declares_mandatory_card_anatomy(&self) -> bool {
        let present: BTreeSet<M5RetentionExportCardAnatomyPart> =
            self.card_anatomy_parts.iter().copied().collect();
        M5RetentionExportCardAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory manifest anatomy part.
    fn declares_mandatory_manifest_anatomy(&self) -> bool {
        let present: BTreeSet<M5ExportManifestAnatomyPart> =
            self.manifest_anatomy_parts.iter().copied().collect();
        M5ExportManifestAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory card export field.
    fn declares_mandatory_card_export(&self) -> bool {
        let present: BTreeSet<M5RetentionExportCardExportField> =
            self.card_export_fields.iter().copied().collect();
        M5RetentionExportCardExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory manifest export field.
    fn declares_mandatory_manifest_export(&self) -> bool {
        let present: BTreeSet<M5ExportManifestExportField> =
            self.manifest_export_fields.iter().copied().collect();
        M5ExportManifestExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_export_baseline
            && !self.hides_retention_or_redaction
            && !self.defaults_to_raw_content_bodies
            && !self.collapses_export_into_generic_download
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompareExportVocabularySet {
    /// Compare / export-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Card-anatomy-part tokens.
    pub card_anatomy_parts: Vec<String>,
    /// Manifest-anatomy-part tokens.
    pub manifest_anatomy_parts: Vec<String>,
    /// Card-posture tokens.
    pub card_postures: Vec<String>,
    /// Manifest-disposition tokens.
    pub manifest_dispositions: Vec<String>,
    /// Compare-baseline tokens.
    pub compare_baselines: Vec<String>,
    /// Card-action tokens.
    pub card_actions: Vec<String>,
    /// Manifest-action tokens.
    pub manifest_actions: Vec<String>,
    /// Card-export-field tokens.
    pub card_export_fields: Vec<String>,
    /// Manifest-export-field tokens.
    pub manifest_export_fields: Vec<String>,
    /// Retention-posture tokens (reused from the frozen matrix).
    pub retention_postures: Vec<String>,
    /// Export-redaction tokens (reused from the frozen matrix).
    pub export_redactions: Vec<String>,
    /// Manifest-class tokens (reused from the frozen matrix).
    pub manifest_classes: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5CompareExportVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5CompareExportConsumerSurface::ALL, |v| v.as_str()),
            card_anatomy_parts: tokens(&M5RetentionExportCardAnatomyPart::ALL, |v| v.as_str()),
            manifest_anatomy_parts: tokens(&M5ExportManifestAnatomyPart::ALL, |v| v.as_str()),
            card_postures: tokens(&M5RetentionExportCardPosture::ALL, |v| v.as_str()),
            manifest_dispositions: tokens(&M5ExportManifestDisposition::ALL, |v| v.as_str()),
            compare_baselines: tokens(&M5CompareBaseline::ALL, |v| v.as_str()),
            card_actions: tokens(&M5RetentionExportCardAction::ALL, |v| v.as_str()),
            manifest_actions: tokens(&M5ExportManifestAction::ALL, |v| v.as_str()),
            card_export_fields: tokens(&M5RetentionExportCardExportField::ALL, |v| v.as_str()),
            manifest_export_fields: tokens(&M5ExportManifestExportField::ALL, |v| v.as_str()),
            retention_postures: tokens(&M5RetentionPosture::ALL, |v| v.as_str()),
            export_redactions: tokens(&M5ExportRedactionPosture::ALL, |v| v.as_str()),
            manifest_classes: tokens(&M5ExportManifestClass::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5HistoryAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompareExportGovernanceReview {
    /// One primitive pair carries card and manifest truth on every consumer.
    pub one_primitive_carries_card_and_manifest_truth: bool,
    /// The compare baseline is always explicit (current/snapshot/disk/Git).
    pub compare_baseline_always_explicit: bool,
    /// Retention posture is always disclosed.
    pub retention_posture_always_disclosed: bool,
    /// Export-redaction posture is always disclosed.
    pub export_redaction_always_disclosed: bool,
    /// What survives, expires, and is metadata-only is always stated.
    pub survival_and_expiry_always_stated: bool,
    /// No export defaults to raw secret-bearing content bodies.
    pub no_export_defaults_to_raw_bodies: bool,
    /// Actor lineage, checkpoint identity, and scope survive export.
    pub lineage_identity_and_scope_survive_export: bool,
    /// An export is never collapsed into a generic download.
    pub export_never_generic_download: bool,
    /// The support / export packet reconstructs card and manifest truth.
    pub support_export_reconstructs_card_and_manifest_truth: bool,
    /// No consumer invents a second baseline or redaction grammar.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompareExportConsumerProjection {
    /// History, refactor, import, AI, recovery, and support consumers all consume the shared
    /// primitive pair.
    pub compare_export_surfaces_consume_shared_primitive: bool,
    /// The card-posture resolver reads a single canonical source.
    pub card_posture_reads_single_source: bool,
    /// The manifest-disposition resolver reads a single canonical source.
    pub manifest_disposition_reads_single_source: bool,
    /// The bounded-action derivation reads a single canonical source.
    pub actions_read_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompareExportProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompareExportReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting recovery audit.
    pub recovery_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CompareExportPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompareExportPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Compare / export rows.
    pub rows: Vec<M5CompareExportRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CompareExportVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CompareExportGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompareExportConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompareExportProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CompareExportReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 compare-export primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompareExportPacket {
    /// Record kind; must equal [`M5_COMPARE_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COMPARE_EXPORT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Compare / export rows.
    pub rows: Vec<M5CompareExportRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CompareExportVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CompareExportGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompareExportConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompareExportProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CompareExportReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CompareExportPacket {
    /// Builds an M5 compare-export primitive packet from stable-lane input.
    pub fn new(input: M5CompareExportPacketInput) -> Self {
        Self {
            record_kind: M5_COMPARE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_COMPARE_EXPORT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 compare-export primitive invariants.
    pub fn validate(&self) -> Vec<M5CompareExportViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMPARE_EXPORT_RECORD_KIND {
            violations.push(M5CompareExportViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COMPARE_EXPORT_SCHEMA_VERSION {
            violations.push(M5CompareExportViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CompareExportViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_card_baseline_coverage(self, &mut violations);
        validate_card_retention_coverage(self, &mut violations);
        validate_card_redaction_coverage(self, &mut violations);
        validate_card_export_coverage(self, &mut violations);
        validate_manifest_baseline_coverage(self, &mut violations);
        validate_manifest_shareable_coverage(self, &mut violations);
        validate_manifest_raw_body_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 compare-export packet serializes"),
        ) {
            violations.push(M5CompareExportViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 compare-export packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per compare / export consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,card_anatomy,manifest_anatomy,retention_postures,export_redactions,compare_baselines,card_postures,manifest_dispositions,card_actions,manifest_actions,card_examples,manifest_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.card_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.manifest_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.retention_postures, |v| v.as_str()),
                join_tokens(&row.export_redactions, |v| v.as_str()),
                join_tokens(&row.compare_baselines, |v| v.as_str()),
                join_tokens(&row.card_postures, |v| v.as_str()),
                join_tokens(&row.manifest_dispositions, |v| v.as_str()),
                join_tokens(&row.card_actions, |v| v.as_str()),
                join_tokens(&row.manifest_actions, |v| v.as_str()),
                row.card_examples.len(),
                row.manifest_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Retention/Export-Card & History-Export-Manifest Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Compare / export consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Card postures: {}\n",
            self.vocabulary_set.card_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Manifest dispositions: {}\n",
            self.vocabulary_set.manifest_dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Compare baselines: {}\n",
            self.vocabulary_set.compare_baselines.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Compare / export consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked cards: {}\n", row.card_examples.len()));
            for case in &row.card_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (export `{}`, compare `{}`, redaction `{}`)\n",
                    case.resolved.card_label,
                    case.resolved.retention_posture.as_str(),
                    case.resolved.card_posture.as_str(),
                    case.resolved.can_export,
                    case.resolved.baseline_comparison_offered,
                    case.resolved.export_redaction.as_str(),
                ));
            }
            out.push_str(&format!(
                "  - Worked manifests: {}\n",
                row.manifest_examples.len()
            ));
            for case in &row.manifest_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (shareable `{}`, baseline `{}`, redaction `{}`)\n",
                    case.resolved.manifest_label,
                    case.resolved.manifest_class.as_str(),
                    case.resolved.manifest_disposition.as_str(),
                    case.resolved.is_shareable,
                    case.resolved.primary_baseline.as_str(),
                    case.resolved.export_redaction.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 compare-export export.
#[derive(Debug)]
pub enum M5CompareExportArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CompareExportViolation>),
}

impl fmt::Display for M5CompareExportArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 compare-export export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 compare-export export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CompareExportArtifactError {}

/// Validation failures emitted by [`M5CompareExportPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CompareExportViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required compare / export consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A compare / export row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory card anatomy parts.
    MandatoryCardAnatomyMissing,
    /// A row omits one of the mandatory manifest anatomy parts.
    MandatoryManifestAnatomyMissing,
    /// A row omits one of the mandatory card export fields.
    MandatoryCardExportMissing,
    /// A row omits one of the mandatory manifest export fields.
    MandatoryManifestExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked card resolutions.
    CardExampleMissing,
    /// A row declares no worked manifest resolutions.
    ManifestExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked card resolution proves each acceptance-named compare baseline.
    CardBaselineCoverageUnproven,
    /// No worked card resolution proves both an expiring / metadata-only and a retained card.
    CardRetentionCoverageUnproven,
    /// No worked card resolution proves a restrictive export-redaction posture.
    CardRedactionCoverageUnproven,
    /// No worked card resolution proves both an exportable and a blocked / nothing-retained
    /// card.
    CardExportCoverageUnproven,
    /// No worked manifest resolution proves each acceptance-named compare baseline.
    ManifestBaselineCoverageUnproven,
    /// No worked manifest resolution proves both a shareable and a held-back manifest.
    ManifestShareableCoverageUnproven,
    /// A worked manifest resolution defaults to a raw body, or none proves preserved lineage.
    ManifestRawBodyPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CompareExportViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryCardAnatomyMissing => "mandatory_card_anatomy_missing",
            Self::MandatoryManifestAnatomyMissing => "mandatory_manifest_anatomy_missing",
            Self::MandatoryCardExportMissing => "mandatory_card_export_missing",
            Self::MandatoryManifestExportMissing => "mandatory_manifest_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::CardExampleMissing => "card_example_missing",
            Self::ManifestExampleMissing => "manifest_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::CardBaselineCoverageUnproven => "card_baseline_coverage_unproven",
            Self::CardRetentionCoverageUnproven => "card_retention_coverage_unproven",
            Self::CardRedactionCoverageUnproven => "card_redaction_coverage_unproven",
            Self::CardExportCoverageUnproven => "card_export_coverage_unproven",
            Self::ManifestBaselineCoverageUnproven => "manifest_baseline_coverage_unproven",
            Self::ManifestShareableCoverageUnproven => "manifest_shareable_coverage_unproven",
            Self::ManifestRawBodyPreservationUnproven => "manifest_raw_body_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 compare-export export.
pub fn current_stable_m5_compare_export_export(
) -> Result<M5CompareExportPacket, M5CompareExportArtifactError> {
    let packet: M5CompareExportPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-retention-export-card-primitive-proof/support_export.json"
    )))
    .map_err(M5CompareExportArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CompareExportArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COMPARE_EXPORT_CARD_SCHEMA_REF,
        M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF,
        M5_COMPARE_EXPORT_DOC_REF,
        M5_COMPARE_EXPORT_COMPONENT_MATRIX_REF,
        M5_COMPARE_EXPORT_RETENTION_CARD_REF,
        M5_COMPARE_EXPORT_GIT_HISTORY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CompareExportViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CompareExportViolation::VocabularySetDrift);
    }
}

fn validate_rows(packet: &M5CompareExportPacket, violations: &mut Vec<M5CompareExportViolation>) {
    let present: BTreeSet<M5CompareExportConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5CompareExportConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5CompareExportViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.card_anatomy_parts.is_empty()
            || row.manifest_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.retention_postures.is_empty()
            || row.export_redactions.is_empty()
            || row.manifest_classes.is_empty()
            || row.compare_baselines.is_empty()
            || row.card_postures.is_empty()
            || row.manifest_dispositions.is_empty()
            || row.card_actions.is_empty()
            || row.manifest_actions.is_empty()
        {
            violations.push(M5CompareExportViolation::RowIncomplete);
        }
        if !row.declares_mandatory_card_anatomy() {
            violations.push(M5CompareExportViolation::MandatoryCardAnatomyMissing);
        }
        if !row.declares_mandatory_manifest_anatomy() {
            violations.push(M5CompareExportViolation::MandatoryManifestAnatomyMissing);
        }
        if !row.declares_mandatory_card_export() {
            violations.push(M5CompareExportViolation::MandatoryCardExportMissing);
        }
        if !row.declares_mandatory_manifest_export() {
            violations.push(M5CompareExportViolation::MandatoryManifestExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5CompareExportViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CompareExportViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CompareExportViolation::DowngradeTriggersMissing);
        }
        if row.card_examples.is_empty() {
            violations.push(M5CompareExportViolation::CardExampleMissing);
        }
        if row.manifest_examples.is_empty() {
            violations.push(M5CompareExportViolation::ManifestExampleMissing);
        }
        if row
            .card_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .manifest_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5CompareExportViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5CompareExportViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5CompareExportViolation::RowInvariantViolated);
        }
    }
}

/// Every acceptance-named compare baseline (current-versus-snapshot, snapshot-versus-disk, and
/// snapshot-versus-Git HEAD) must appear across some worked card's available comparisons — the
/// acceptance-criterion example that baselines are explicit across recovery flows.
fn validate_card_baseline_coverage(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let covered: BTreeSet<M5CompareBaseline> = packet
        .rows
        .iter()
        .flat_map(|row| row.card_examples.iter())
        .flat_map(|case| case.resolved.available_baselines.iter().copied())
        .collect();
    if !M5CompareBaseline::NAMED
        .iter()
        .all(|baseline| covered.contains(baseline))
    {
        violations.push(M5CompareExportViolation::CardBaselineCoverageUnproven);
    }
}

/// At least one worked card resolution must prove an expiring or metadata-only retention and at
/// least one must prove a retained card — the acceptance-criterion example that what survives,
/// expires, and is metadata-only stays explicit.
fn validate_card_retention_coverage(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let has_expiring = packet.rows.iter().any(|row| {
        row.card_examples.iter().any(|case| {
            matches!(
                case.resolved.card_posture,
                M5RetentionExportCardPosture::MetadataOnlySurvives
                    | M5RetentionExportCardPosture::PurgeScheduled
                    | M5RetentionExportCardPosture::NothingRetained
            )
        })
    });
    let has_retained = packet.rows.iter().any(|row| {
        row.card_examples.iter().any(|case| {
            matches!(
                case.resolved.card_posture,
                M5RetentionExportCardPosture::FullyShareable
            )
        })
    });
    if !(has_expiring && has_retained) {
        violations.push(M5CompareExportViolation::CardRetentionCoverageUnproven);
    }
}

/// At least one worked card resolution must prove a restrictive export-redaction posture — the
/// acceptance-criterion example that redaction is never hidden behind a generic export.
fn validate_card_redaction_coverage(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.card_examples
            .iter()
            .any(|case| redaction_is_restrictive(case.resolved.export_redaction))
    });
    if !proven {
        violations.push(M5CompareExportViolation::CardRedactionCoverageUnproven);
    }
}

/// At least one worked card resolution must prove an exportable card and at least one must prove
/// a blocked / nothing-retained card — the acceptance-criterion example that a card never claims
/// an export path it does not have.
fn validate_card_export_coverage(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let has_exportable = packet.rows.iter().any(|row| {
        row.card_examples
            .iter()
            .any(|case| case.resolved.can_export)
    });
    let has_blocked = packet.rows.iter().any(|row| {
        row.card_examples
            .iter()
            .any(|case| !case.resolved.can_export)
    });
    if !(has_exportable && has_blocked) {
        violations.push(M5CompareExportViolation::CardExportCoverageUnproven);
    }
}

/// Every acceptance-named compare baseline must appear across some worked manifest's primary
/// baseline — the acceptance-criterion example that outbound export baselines are explicit.
fn validate_manifest_baseline_coverage(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let covered: BTreeSet<M5CompareBaseline> = packet
        .rows
        .iter()
        .flat_map(|row| row.manifest_examples.iter())
        .map(|case| case.resolved.primary_baseline)
        .collect();
    if !M5CompareBaseline::NAMED
        .iter()
        .all(|baseline| covered.contains(baseline))
    {
        violations.push(M5CompareExportViolation::ManifestBaselineCoverageUnproven);
    }
}

/// At least one worked manifest resolution must prove a shareable manifest and at least one must
/// prove a held-back manifest — the acceptance-criterion example that a manifest never claims a
/// share path it does not have.
fn validate_manifest_shareable_coverage(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let has_shareable = packet.rows.iter().any(|row| {
        row.manifest_examples
            .iter()
            .any(|case| case.resolved.is_shareable)
    });
    let has_held = packet.rows.iter().any(|row| {
        row.manifest_examples
            .iter()
            .any(|case| !case.resolved.is_shareable)
    });
    if !(has_shareable && has_held) {
        violations.push(M5CompareExportViolation::ManifestShareableCoverageUnproven);
    }
}

/// Every worked manifest resolution must omit raw bodies, and at least one must prove a manifest
/// whose actor lineage, checkpoint identity, and scope are all preserved — the
/// acceptance-criterion example that export preserves lineage and never defaults to a raw
/// secret-bearing content body.
fn validate_manifest_raw_body_preservation(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let all_omit = packet
        .rows
        .iter()
        .flat_map(|row| row.manifest_examples.iter())
        .all(|case| case.resolved.omits_raw_bodies && case.preserves_identity());
    let has_full_lineage = packet.rows.iter().any(|row| {
        row.manifest_examples.iter().any(|case| {
            case.resolved.preserves_actor_lineage
                && case.resolved.preserves_checkpoint_identity
                && case.resolved.preserves_scope
        })
    });
    if !(all_omit && has_full_lineage) {
        violations.push(M5CompareExportViolation::ManifestRawBodyPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_card_and_manifest_truth,
        review.compare_baseline_always_explicit,
        review.retention_posture_always_disclosed,
        review.export_redaction_always_disclosed,
        review.survival_and_expiry_always_stated,
        review.no_export_defaults_to_raw_bodies,
        review.lineage_identity_and_scope_survive_export,
        review.export_never_generic_download,
        review.support_export_reconstructs_card_and_manifest_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5CompareExportViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.compare_export_surfaces_consume_shared_primitive,
        projection.card_posture_reads_single_source,
        projection.manifest_disposition_reads_single_source,
        projection.actions_read_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5CompareExportViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CompareExportViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CompareExportPacket,
    violations: &mut Vec<M5CompareExportViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.recovery_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CompareExportViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
