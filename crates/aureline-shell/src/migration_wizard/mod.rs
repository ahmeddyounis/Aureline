//! Beta-grade migration wizard projection.
//!
//! The migration wizard is the page-level surface that wraps the
//! source-detection seed in [`crate::import`] and the import diff
//! review packet in [`crate::import::diff_review`] into one guided
//! flow. The shell, the headless inspector
//! (`aureline_shell_migration_wizard`), and the support-export
//! wrapper consume the same projection so the live UI, CLI, and
//! support evidence quote the same wizard truth.
//!
//! The projection adds the things a beta-grade migration wizard
//! expects on top of the existing classifier and diff review:
//!
//! - a [`WizardStage`] vocabulary that names every reviewable point
//!   in the wizard lifecycle so surfaces never invent their own
//!   status names;
//! - a [`WizardMappingReport`] that classifies every imported item
//!   as `Exact`, `Translated`, `Partial`, `Shimmed`, or
//!   `Unsupported` and is retained after the apply so the user can
//!   reopen it from settings, help, or support export;
//! - a [`WizardRollbackRequirementBinding`] that records the rollback
//!   checkpoint requirement without pretending the preview created one;
//! - typed [`WizardCompareAction`] and [`WizardUndoAction`] rows so
//!   compare-and-undo paths are first-class instead of toast-only
//!   prose; and
//! - explicit [`UnsupportedGapRow`] visibility so unsupported items
//!   surface during preview rather than as hidden missing behavior
//!   after apply.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under
//! `fixtures/migration/m3/migration_wizard/` are bit-for-bit equal
//! to the seeded page produced by [`seeded_migration_wizard_page`].

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::import::diff_review::{
    materialize_import_diff_review_packet, support_safe_target_descriptor, ImportDiffReviewPacket,
    ImportDiffReviewRow, ImportMappingClassification, ImportReportReopenSurface,
    ImportReviewDomain,
};
use crate::import::{
    CompetitorConfigClassification, CompetitorConfigClassifier, ImportReviewRecord,
};

/// Beta migration wizard schema version exported with every record.
pub const MIGRATION_WIZARD_SCHEMA_VERSION: u32 = 2;

/// Stable shared contract ref consumed by every beta migration-wizard row.
pub const MIGRATION_WIZARD_SHARED_CONTRACT_REF: &str = "shell:migration_wizard_beta:v2";

/// Stable record kind for [`MigrationWizardPage`] payloads.
pub const MIGRATION_WIZARD_PAGE_RECORD_KIND: &str = "shell_migration_wizard_beta_page_record";

/// Stable record kind for [`MigrationWizardSupportExport`] payloads.
pub const MIGRATION_WIZARD_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_migration_wizard_beta_support_export_record";

/// Stable record kind for [`MigrationWizardIssueTemplateExport`] payloads.
pub const MIGRATION_WIZARD_ISSUE_TEMPLATE_EXPORT_RECORD_KIND: &str =
    "shell_migration_wizard_beta_issue_template_export_record";

/// Stable record kind for [`MigrationSessionHeader`] payloads.
pub const MIGRATION_WIZARD_HEADER_RECORD_KIND: &str = "shell_migration_wizard_beta_header_record";

/// Stable record kind for [`WizardMappingReport`] payloads.
pub const MIGRATION_WIZARD_MAPPING_REPORT_RECORD_KIND: &str =
    "shell_migration_wizard_beta_mapping_report_record";

/// Stable record kind for [`WizardMappingReportRow`] payloads.
pub const MIGRATION_WIZARD_MAPPING_ROW_RECORD_KIND: &str =
    "shell_migration_wizard_beta_mapping_row_record";

/// Stable record kind for [`UnsupportedGapRow`] payloads.
pub const MIGRATION_WIZARD_UNSUPPORTED_GAP_RECORD_KIND: &str =
    "shell_migration_wizard_beta_unsupported_gap_record";

/// Generation timestamp used by every seeded record so fixtures stay stable.
const GENERATED_AT: &str = "2026-05-15T00:00:00Z";

/// Stable wizard stage class that names a reviewable point in the flow.
///
/// Surfaces consume these tokens verbatim and MUST NOT invent their
/// own status names. The order reflects the lifecycle: a wizard
/// session may only move forward between stages or jump to
/// [`WizardStage::RolledBack`] after [`WizardStage::Applied`] /
/// [`WizardStage::PartiallyApplied`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WizardStage {
    /// User has not yet selected a readable source config root.
    SelectingSource,
    /// Source root has been detected and classified read-only.
    SourceDetected,
    /// Diff review packet is ready and unsupported gaps are visible.
    PreviewReady,
    /// Rollback checkpoint is materialized and the apply gate is open.
    CheckpointReady,
    /// Apply is running against the reviewed preview and checkpoint.
    Applying,
    /// Apply completed and the mapping report is retained.
    Applied,
    /// Apply landed some rows; blocked rows remain visible in the report.
    PartiallyApplied,
    /// Apply was denied by a pre-apply gate (stale preview, missing
    /// checkpoint, or policy lock). No durable state was mutated.
    Blocked,
    /// User triggered the undo path; the checkpoint restored prior state.
    RolledBack,
}

impl WizardStage {
    /// Returns the stable schema token for this stage.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectingSource => "selecting_source",
            Self::SourceDetected => "source_detected",
            Self::PreviewReady => "preview_ready",
            Self::CheckpointReady => "checkpoint_ready",
            Self::Applying => "applying",
            Self::Applied => "applied",
            Self::PartiallyApplied => "partially_applied",
            Self::Blocked => "blocked",
            Self::RolledBack => "rolled_back",
        }
    }

    /// Returns the reviewer-facing label for this stage.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::SelectingSource => "Select source",
            Self::SourceDetected => "Source detected",
            Self::PreviewReady => "Preview ready",
            Self::CheckpointReady => "Checkpoint ready",
            Self::Applying => "Applying",
            Self::Applied => "Applied",
            Self::PartiallyApplied => "Partially applied",
            Self::Blocked => "Blocked",
            Self::RolledBack => "Rolled back",
        }
    }

    /// Returns `true` when this stage represents a post-apply state.
    pub const fn is_post_apply(self) -> bool {
        matches!(
            self,
            Self::Applied | Self::PartiallyApplied | Self::RolledBack
        )
    }

    /// Returns `true` when durable state may have been mutated.
    pub const fn may_have_mutated_state(self) -> bool {
        matches!(
            self,
            Self::Applying | Self::Applied | Self::PartiallyApplied | Self::RolledBack
        )
    }
}

/// One row in the wizard's history of admitted stages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardStageTransition {
    /// Stage that the wizard entered.
    pub stage: WizardStage,
    /// Whether the wizard was permitted to mutate durable state at this stage.
    pub durable_writes_authorized: bool,
    /// Reviewer-facing summary recorded with the transition.
    pub summary: String,
}

/// Pre-apply rollback-checkpoint requirement binding.
///
/// A diff-review packet is a dry-run artifact: it can prove that a checkpoint
/// is required, but it cannot prove that apply subsequently created one. The
/// execution/orchestration layer must publish real checkpoint evidence before
/// advancing the wizard beyond [`WizardStage::PreviewReady`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardRollbackRequirementBinding {
    /// Opaque requirement ref; this is not a checkpoint handle.
    pub requirement_ref: String,
    /// Stable state showing when the requirement must be satisfied.
    pub requirement_state: String,
    /// Declared checkpoint scope.
    pub checkpoint_scope: String,
    /// State refs that a later execution checkpoint must protect.
    pub required_protected_state_refs: Vec<String>,
    /// Domains a later checkpoint must protect, in deterministic order.
    pub protected_domains: Vec<ImportReviewDomain>,
    /// Action hints that become available once real checkpoint evidence exists.
    pub rollback_action_hints: Vec<String>,
    /// Reviewer-facing narrative.
    pub narrative: String,
}

/// Source/target descriptor pair shown at the top of the wizard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardSourceTargetDescriptor {
    /// Detected source family classification.
    pub source_classification: CompetitorConfigClassification,
    /// Redaction-safe source descriptor.
    pub source_descriptor: String,
    /// Destination workspace or profile descriptor.
    pub target_descriptor: String,
    /// Source ecosystem token recorded in the packet.
    pub source_ecosystem_id: String,
}

/// Version truth class shown on the source-tool chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceVersionTruthClass {
    /// The importer read a concrete source version.
    DetectedExact,
    /// The source was recognized by markers, but the exact version was not read.
    MarkerOnlyVersionUnknown,
}

impl SourceVersionTruthClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DetectedExact => "detected_exact",
            Self::MarkerOnlyVersionUnknown => "marker_only_version_unknown",
        }
    }
}

/// Source tool/version chip rendered in the migration header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationSourceToolChip {
    /// Stable source-tool id.
    pub source_tool_id: String,
    /// Reviewer-facing source tool label.
    pub source_tool_label: String,
    /// Version chip text. When exact version is unavailable, this must say so.
    pub source_version_label: String,
    /// Truth class for the source version chip.
    pub version_truth_class: SourceVersionTruthClass,
    /// Redaction-safe evidence ref for the source version or marker.
    pub version_evidence_ref: String,
}

/// Scope kind for the target side of a migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationTargetScopeKind {
    /// Changes land in an Aureline profile.
    Profile,
    /// Changes land in an Aureline workspace.
    Workspace,
}

impl MigrationTargetScopeKind {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Profile => "profile",
            Self::Workspace => "workspace",
        }
    }
}

/// Target profile/workspace truth rendered in the migration header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationTargetScope {
    /// Target scope kind.
    pub scope_kind: MigrationTargetScopeKind,
    /// Stable target ref selected by the user.
    pub target_ref: String,
    /// Reviewer-facing target label.
    pub target_label: String,
    /// Short sentence describing where writes land.
    pub writes_land_in: String,
}

/// Checkpoint-requirement notice rendered in the migration header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCheckpointRequirementNotice {
    /// Opaque rollback requirement ref; never a checkpoint handle.
    pub requirement_ref: String,
    /// Whether apply must create and verify a checkpoint first.
    pub required_before_apply: bool,
    /// User-facing notice text.
    pub notice_label: String,
}

/// Header action used for restore and compatibility inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationHeaderAction {
    /// Stable action id.
    pub action_id: String,
    /// Reviewer-facing action label.
    pub action_label: String,
    /// Stable action token used by UI and CLI.
    pub action_token: String,
    /// Ref opened by the action.
    pub target_ref: String,
    /// Whether the action is currently enabled.
    pub enabled: bool,
}

/// Header state consumed by UI, CLI, support export, and issue templates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationSessionHeader {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the header.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable wizard session id used by UI and CLI.
    pub wizard_session_id: String,
    /// In-packet review correlation reused by support and issue-template exports.
    pub migration_review_ref: String,
    /// Source tool/version chips.
    pub source_tool: MigrationSourceToolChip,
    /// Target profile/workspace truth.
    pub target_scope: MigrationTargetScope,
    /// Checkpoint-requirement notice.
    pub checkpoint_requirement_notice: MigrationCheckpointRequirementNotice,
    /// Restore action kept visible but disabled until real checkpoint evidence exists.
    pub restore_action: MigrationHeaderAction,
    /// Compatibility-report open action kept visible before apply.
    pub compatibility_report_action: MigrationHeaderAction,
    /// Whether partial-apply context is visible in the header.
    pub partial_apply_context_visible: bool,
    /// Whether downgrade/narrowing context is visible in the header.
    pub downgrade_context_visible: bool,
    /// Whether restore context is visible in the header.
    pub restore_context_visible: bool,
    /// Support export ref that quotes this header.
    pub support_export_ref: String,
    /// Issue-template ref that quotes this header.
    pub issue_template_ref: String,
}

impl MigrationSessionHeader {
    /// Returns true when the preview header answers source, target, checkpoint
    /// requirement, and compatibility-inspector questions without overclaiming
    /// an executable restore action.
    pub fn answers_required_questions(&self) -> bool {
        !self.source_tool.source_tool_label.trim().is_empty()
            && !self.source_tool.source_version_label.trim().is_empty()
            && !self.target_scope.target_ref.trim().is_empty()
            && !self.target_scope.writes_land_in.trim().is_empty()
            && self.checkpoint_requirement_notice.required_before_apply
            && !self
                .checkpoint_requirement_notice
                .requirement_ref
                .trim()
                .is_empty()
            && !self.restore_action.enabled
            && !self.restore_action.target_ref.trim().is_empty()
            && self.compatibility_report_action.enabled
            && !self
                .compatibility_report_action
                .target_ref
                .trim()
                .is_empty()
    }

    /// Returns true when review/apply aftermath still shows the required context.
    pub fn aftermath_context_visible(&self) -> bool {
        self.partial_apply_context_visible
            && self.downgrade_context_visible
            && self.restore_context_visible
    }
}

/// One classified mapping row that survives after apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardMappingReportRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Stable row id quoted across surfaces.
    pub row_id: String,
    /// Import domain for grouped review.
    pub domain: ImportReviewDomain,
    /// Required Exact / Translated / Partial / Shimmed / Unsupported class.
    pub classification: ImportMappingClassification,
    /// Stable source object ref retained for support evidence.
    pub source_item_ref: String,
    /// Redaction-aware source label.
    pub source_label: String,
    /// Target object ref produced by the importer, when one exists.
    pub target_item_ref: Option<String>,
    /// Redaction-aware target label.
    pub target_label: String,
    /// Reviewer-facing current value summary.
    pub before_value_label: String,
    /// Reviewer-facing imported value summary.
    pub after_value_label: String,
    /// Rollback requirement governing this row.
    pub rollback_requirement_ref: String,
    /// Caveat retained for `Partial`, `Shimmed`, and `Unsupported` rows.
    pub lossy_or_unsupported_note: Option<String>,
    /// Docs/help refs that can reopen the row after first run.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that preserve the row.
    pub support_export_refs: Vec<String>,
    /// True when this row remains visible after apply.
    pub retained_after_apply: bool,
}

impl WizardMappingReportRow {
    fn from_diff_row(row: &ImportDiffReviewRow) -> Self {
        Self {
            record_kind: MIGRATION_WIZARD_MAPPING_ROW_RECORD_KIND.to_owned(),
            schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
            row_id: row.row_id.clone(),
            domain: row.domain,
            classification: row.classification,
            source_item_ref: row.source_item_ref.clone(),
            source_label: row.source_label.clone(),
            target_item_ref: row.target_item_ref.clone(),
            target_label: row.target_label.clone(),
            before_value_label: row.before_value_label.clone(),
            after_value_label: row.after_value_label.clone(),
            rollback_requirement_ref: row.rollback_requirement_ref.clone(),
            lossy_or_unsupported_note: row.lossy_or_unsupported_note.clone(),
            docs_help_refs: row.docs_help_refs.clone(),
            support_export_refs: row.support_export_refs.clone(),
            retained_after_apply: row.retained_after_apply()
                || row.classification == ImportMappingClassification::Exact
                || row.classification == ImportMappingClassification::Translated,
        }
    }
}

/// One pre-apply unsupported-gap row surfaced before apply.
///
/// The wizard requires the union of all `Unsupported` and bridge-required
/// rows to be visible during preview so users discover the gap immediately
/// instead of as hidden missing behavior after import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedGapRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Stable gap id.
    pub gap_id: String,
    /// Domain the gap lives in.
    pub domain: ImportReviewDomain,
    /// Classification of the gap (`Unsupported` or `Shimmed`).
    pub classification: ImportMappingClassification,
    /// Redaction-aware source object label.
    pub source_label: String,
    /// Reviewer-facing description of the gap.
    pub gap_summary: String,
    /// True when the gap is visible during preview, before apply.
    pub visible_before_apply: bool,
    /// True when the gap remains visible in the retained report.
    pub retained_after_apply: bool,
    /// Docs/help refs explaining the gap.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that carry the gap into the export.
    pub support_export_refs: Vec<String>,
}

/// Per-classification count summary for the mapping report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardClassificationSummary {
    /// Number of `Exact` rows.
    pub exact: usize,
    /// Number of `Translated` rows.
    pub translated: usize,
    /// Number of `Partial` rows.
    pub partial: usize,
    /// Number of `Shimmed` rows.
    pub shimmed: usize,
    /// Number of `Unsupported` rows.
    pub unsupported: usize,
}

impl WizardClassificationSummary {
    /// Returns the per-classification counts for the given report rows.
    pub fn from_rows(rows: &[WizardMappingReportRow]) -> Self {
        let mut summary = Self {
            exact: 0,
            translated: 0,
            partial: 0,
            shimmed: 0,
            unsupported: 0,
        };
        for row in rows {
            match row.classification {
                ImportMappingClassification::Exact => summary.exact += 1,
                ImportMappingClassification::Translated => summary.translated += 1,
                ImportMappingClassification::Partial => summary.partial += 1,
                ImportMappingClassification::Shimmed => summary.shimmed += 1,
                ImportMappingClassification::Unsupported => summary.unsupported += 1,
            }
        }
        summary
    }

    /// Returns the total number of classified rows.
    pub const fn total(&self) -> usize {
        self.exact + self.translated + self.partial + self.shimmed + self.unsupported
    }
}

/// Retained mapping report that survives after apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardMappingReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the report.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable mapping report id.
    pub mapping_report_id: String,
    /// In-packet migration review correlation the report was generated from.
    pub migration_review_ref: String,
    /// Source / target descriptors header.
    pub descriptors: WizardSourceTargetDescriptor,
    /// Mapping rows, classified per-row, sorted by `row_id`.
    pub rows: Vec<WizardMappingReportRow>,
    /// Classification counts by token.
    pub classification_summary: WizardClassificationSummary,
    /// Distinct classifications present in the report.
    pub classifications_present: Vec<ImportMappingClassification>,
    /// Pre-apply unsupported / bridge gaps, surfaced before apply and
    /// retained after apply.
    pub unsupported_gaps: Vec<UnsupportedGapRow>,
    /// Rollback requirement ref retained with the report.
    pub rollback_requirement_ref: String,
    /// Shortcut delta digest ref retained with the report.
    pub shortcut_delta_report_ref: String,
    /// True when the report survives first-run onboarding.
    pub retained_after_first_run: bool,
    /// Reopen links for settings, help, and support/export surfaces.
    pub reopen_links: Vec<WizardReopenLink>,
    /// Timestamp captured when the report was generated.
    pub generated_at: String,
}

impl WizardMappingReport {
    /// Returns `true` when every required reopen surface is wired up.
    pub fn has_required_reopen_surfaces(&self) -> bool {
        [
            ImportReportReopenSurface::Settings,
            ImportReportReopenSurface::Help,
            ImportReportReopenSurface::SupportExport,
        ]
        .into_iter()
        .all(|surface| self.reopen_links.iter().any(|link| link.surface == surface))
    }

    /// Returns `true` when every row is classified.
    pub fn every_row_classified(&self) -> bool {
        self.classification_summary.total() == self.rows.len()
    }

    /// Returns `true` when at least one row carries each required class.
    pub fn covers_every_required_classification(&self) -> bool {
        [
            ImportMappingClassification::Exact,
            ImportMappingClassification::Translated,
            ImportMappingClassification::Partial,
            ImportMappingClassification::Shimmed,
            ImportMappingClassification::Unsupported,
        ]
        .into_iter()
        .all(|class| self.classifications_present.contains(&class))
    }
}

/// Link that reopens the retained mapping report from one product surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardReopenLink {
    /// Surface that owns the reopen affordance.
    pub surface: ImportReportReopenSurface,
    /// Stable surface-specific action ref.
    pub action_ref: String,
    /// Migration report ref reopened by the action.
    pub migration_report_ref: String,
    /// Shortcut delta report ref carried with the report.
    pub shortcut_delta_report_ref: String,
    /// Reviewer-facing label.
    pub label: String,
}

/// Compare path exposed after apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardCompareAction {
    /// Stable compare action id.
    pub compare_action_id: String,
    /// Domain the compare action covers.
    pub domain: ImportReviewDomain,
    /// State ref that captures the pre-apply value.
    pub before_state_ref: String,
    /// State ref that captures the post-apply value.
    pub after_state_ref: String,
    /// Reviewer-facing action label.
    pub action_label: String,
    /// Required action token surfaced by the activity center.
    pub action_token: String,
}

/// Undo path exposed after apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardUndoAction {
    /// Stable undo action id.
    pub undo_action_id: String,
    /// Checkpoint ref the undo restores from.
    pub checkpoint_ref: String,
    /// Restore record ref written by the undo path.
    pub restore_record_ref: String,
    /// True when the undo requires explicit user confirmation.
    pub requires_confirmation: bool,
    /// Reviewer-facing action label.
    pub action_label: String,
    /// Required action token surfaced by the activity center.
    pub action_token: String,
}

/// Apply gate class for the wizard at the current stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WizardApplyGate {
    /// Preview exists but is missing the rollback checkpoint.
    BlockedNoCheckpoint,
    /// Preview is stale and a fresh dry run is required.
    StaleRequiresReplan,
    /// Apply is denied because at least one blocking gap remains.
    RequiresManualReview,
    /// Checkpoint is ready; apply may proceed.
    AllowedCheckpointReady,
    /// Apply already completed; the gate is exhausted.
    AlreadyApplied,
}

impl WizardApplyGate {
    /// Returns the stable schema token for this apply gate class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedNoCheckpoint => "blocked_no_checkpoint",
            Self::StaleRequiresReplan => "stale_requires_replan",
            Self::RequiresManualReview => "requires_manual_review",
            Self::AllowedCheckpointReady => "allowed_checkpoint_ready",
            Self::AlreadyApplied => "already_applied",
        }
    }
}

/// Summary banner shown at the top of the wizard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardSummary {
    /// Number of mapping rows present.
    pub mapping_row_count: usize,
    /// Number of pre-apply unsupported / bridge gaps.
    pub unsupported_gap_count: usize,
    /// Number of compare actions exposed after apply.
    pub compare_action_count: usize,
    /// Number of undo actions exposed after apply.
    pub undo_action_count: usize,
    /// True when a real execution checkpoint has been bound to this page.
    pub checkpoint_minted: bool,
    /// True when the wizard never authorized apply without a checkpoint.
    pub no_durable_writes_before_checkpoint: bool,
    /// True when unsupported gaps are visible during preview.
    pub unsupported_gaps_visible_before_apply: bool,
    /// True when the report survives first run.
    pub mapping_report_retained: bool,
}

/// Beta-grade migration wizard page record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable wizard session id used to pivot across surfaces.
    pub wizard_session_id: String,
    /// In-packet review correlation; this is not a durable migration session.
    pub migration_review_ref: String,
    /// Header state rendered by UI, CLI, support export, and issue template.
    pub header: MigrationSessionHeader,
    /// Current wizard stage.
    pub current_stage: WizardStage,
    /// Stage transitions, in order. The wizard MUST start at
    /// [`WizardStage::SelectingSource`] and never authorize durable
    /// writes before [`WizardStage::CheckpointReady`].
    pub stage_history: Vec<WizardStageTransition>,
    /// Source/target descriptors header.
    pub descriptors: WizardSourceTargetDescriptor,
    /// Import diff preview ref (the existing
    /// [`ImportDiffReviewPacket`] the wizard wraps).
    pub import_diff_preview_ref: String,
    /// Mapping report retained after first run.
    pub mapping_report: WizardMappingReport,
    /// Rollback checkpoint requirement carried by the dry-run preview.
    pub rollback_requirement: WizardRollbackRequirementBinding,
    /// Compare paths exposed after apply.
    pub compare_actions: Vec<WizardCompareAction>,
    /// Undo paths exposed after apply.
    pub undo_actions: Vec<WizardUndoAction>,
    /// Apply gate class for the current stage.
    pub apply_gate: WizardApplyGate,
    /// Reviewer-facing summary banner.
    pub summary: WizardSummary,
    /// Timestamp captured when the page was generated.
    pub generated_at: String,
}

impl MigrationWizardPage {
    /// Returns `true` when the page is past the apply stage with a
    /// retained report and exposed compare/undo paths.
    pub fn post_apply_paths_are_visible(&self) -> bool {
        self.current_stage.is_post_apply()
            && !self.compare_actions.is_empty()
            && !self.undo_actions.is_empty()
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "wizard: session={}, migration_review={}, stage={}, rows={}, gaps={}, checkpoint_requirement={}",
            self.wizard_session_id,
            self.migration_review_ref,
            self.current_stage.display_label(),
            self.mapping_report.rows.len(),
            self.mapping_report.unsupported_gaps.len(),
            self.rollback_requirement.requirement_ref
        ));
        lines.push(format!(
            "header: source={} version={} target={} restore={} compatibility={}",
            self.header.source_tool.source_tool_label,
            self.header.source_tool.source_version_label,
            self.header.target_scope.writes_land_in,
            self.header.restore_action.target_ref,
            self.header.compatibility_report_action.target_ref
        ));
        lines.push(format!(
            "classifications: exact={}, translated={}, partial={}, shimmed={}, unsupported={}",
            self.mapping_report.classification_summary.exact,
            self.mapping_report.classification_summary.translated,
            self.mapping_report.classification_summary.partial,
            self.mapping_report.classification_summary.shimmed,
            self.mapping_report.classification_summary.unsupported
        ));
        for row in self.mapping_report.rows.iter().take(3) {
            lines.push(format!(
                "{}: {} -> {} ({})",
                row.domain.display_label(),
                row.before_value_label,
                row.after_value_label,
                row.classification.display_label()
            ));
        }
        for gap in &self.mapping_report.unsupported_gaps {
            lines.push(format!(
                "gap: {} -- {} [{}]",
                gap.domain.display_label(),
                gap.gap_summary,
                gap.classification.display_label()
            ));
        }
        lines
    }
}

/// Support-export wrapper that quotes the wizard page plus every
/// stable id reviewers need to pivot between surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// In-packet migration review correlation reused across surfaces.
    pub migration_review_ref: String,
    /// Header quoted in full so screenshots are not needed.
    pub header: MigrationSessionHeader,
    /// Issue template ref generated from the same session.
    pub issue_template_ref: String,
    /// Wizard page quoted in full.
    pub page: MigrationWizardPage,
    /// Stable wizard session id, mapping row ids, gap ids, and
    /// compare/undo action ids in deterministic order.
    pub case_ids: Vec<String>,
}

impl MigrationWizardSupportExport {
    /// Builds the support-export wrapper for a validated wizard page.
    ///
    /// # Errors
    /// Returns the page's typed validation defects rather than exporting a
    /// tampered or internally inconsistent preview.
    pub fn from_page(
        support_export_id: impl Into<String>,
        page: MigrationWizardPage,
    ) -> Result<Self, Vec<MigrationWizardValidationError>> {
        validate_migration_wizard_page(&page)?;
        let support_export_id = support_export_id.into();
        if !has_bounded_opaque_ref(&support_export_id, "support-export:") {
            return Err(vec![MigrationWizardValidationError::EvidenceJoinDrift {
                field: "support_export_id".to_owned(),
            }]);
        }
        let mut case_ids = vec![
            page.migration_review_ref.clone(),
            page.wizard_session_id.clone(),
            page.header
                .checkpoint_requirement_notice
                .requirement_ref
                .clone(),
            page.header.restore_action.action_id.clone(),
            page.header.compatibility_report_action.action_id.clone(),
            page.header.issue_template_ref.clone(),
            page.mapping_report.mapping_report_id.clone(),
        ];
        case_ids.reserve(
            page.mapping_report.rows.len()
                + page.mapping_report.unsupported_gaps.len()
                + page.compare_actions.len()
                + page.undo_actions.len(),
        );
        for row in &page.mapping_report.rows {
            case_ids.push(row.row_id.clone());
        }
        for gap in &page.mapping_report.unsupported_gaps {
            case_ids.push(gap.gap_id.clone());
        }
        for compare in &page.compare_actions {
            case_ids.push(compare.compare_action_id.clone());
        }
        for undo in &page.undo_actions {
            case_ids.push(undo.undo_action_id.clone());
        }
        Ok(Self {
            record_kind: MIGRATION_WIZARD_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
            shared_contract_ref: MIGRATION_WIZARD_SHARED_CONTRACT_REF.to_owned(),
            support_export_id,
            migration_review_ref: page.migration_review_ref.clone(),
            header: page.header.clone(),
            issue_template_ref: page.header.issue_template_ref.clone(),
            page,
            case_ids,
        })
    }
}

/// Issue-template projection for migration support handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationWizardIssueTemplateExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable issue-template ref.
    pub issue_template_ref: String,
    /// In-packet migration review correlation reused across surfaces.
    pub migration_review_ref: String,
    /// Stable wizard session id.
    pub wizard_session_id: String,
    /// Support export id that quotes the same session.
    pub support_export_id: String,
    /// Header quoted in full.
    pub header: MigrationSessionHeader,
    /// Redaction-safe body lines for the template.
    pub body_lines: Vec<String>,
    /// Stable ids copied from support export.
    pub case_ids: Vec<String>,
}

impl MigrationWizardIssueTemplateExport {
    /// Builds an issue-template projection from a support export.
    pub fn from_support_export(export: &MigrationWizardSupportExport) -> Self {
        let page = &export.page;
        let header = export.header.clone();
        let body_lines = vec![
            format!("migration_review_ref: {}", export.migration_review_ref),
            format!("wizard_session_id: {}", page.wizard_session_id),
            format!(
                "source: {} / {} ({})",
                header.source_tool.source_tool_label,
                header.source_tool.source_version_label,
                header.source_tool.version_truth_class.as_str()
            ),
            format!("target: {}", header.target_scope.writes_land_in),
            format!(
                "checkpoint_requirement: {} required_before_apply: {} restore_enabled: {}",
                header.checkpoint_requirement_notice.requirement_ref,
                header.checkpoint_requirement_notice.required_before_apply,
                header.restore_action.enabled
            ),
            format!(
                "compatibility_report: {}",
                header.compatibility_report_action.target_ref
            ),
            format!(
                "classification_counts: exact={} translated={} partial={} shimmed={} unsupported={}",
                page.mapping_report.classification_summary.exact,
                page.mapping_report.classification_summary.translated,
                page.mapping_report.classification_summary.partial,
                page.mapping_report.classification_summary.shimmed,
                page.mapping_report.classification_summary.unsupported
            ),
        ];
        Self {
            record_kind: MIGRATION_WIZARD_ISSUE_TEMPLATE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
            shared_contract_ref: MIGRATION_WIZARD_SHARED_CONTRACT_REF.to_owned(),
            issue_template_ref: header.issue_template_ref.clone(),
            migration_review_ref: export.migration_review_ref.clone(),
            wizard_session_id: page.wizard_session_id.clone(),
            support_export_id: export.support_export_id.clone(),
            header,
            body_lines,
            case_ids: export.case_ids.clone(),
        }
    }
}

/// Validation error produced by [`validate_migration_wizard_page`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum MigrationWizardValidationError {
    /// A record kind, schema version, or shared-contract identity drifted.
    EnvelopeInvalid { field: String },
    /// Cross-record source, target, review, or report identity drifted.
    EvidenceJoinDrift { field: String },
    /// At least one row is missing a classification.
    UnclassifiedRow { row_id: String },
    /// The mapping report does not cover every required classification.
    MissingRequiredClassification { classification: String },
    /// A dry-run page claimed a stage that requires execution evidence.
    UnprovenLifecycleStage { stage: String },
    /// The rollback-checkpoint requirement is missing or malformed.
    RollbackRequirementMissing,
    /// Mapping rows, counts, classification vocabulary, or gap projections drifted.
    MappingReportDrift { field: String },
    /// Preview stage history, gate, actions, or summary claimed inconsistent truth.
    PreviewLifecycleDrift { field: String },
    /// The mapping report is not retained after first run.
    MappingReportNotRetained,
    /// At least one unsupported gap is hidden before apply.
    UnsupportedGapHiddenBeforeApply { gap_id: String },
    /// The reopen links are missing one of settings / help / support_export.
    ReopenLinksIncomplete { surface: String },
    /// The diff preview ref is empty.
    DiffPreviewRefMissing,
    /// Header state is incomplete.
    HeaderIncomplete,
    /// Header and page session identifiers drifted apart.
    HeaderSessionMismatch,
}

/// Validates a wizard page against the M3 acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_migration_wizard_page(
    page: &MigrationWizardPage,
) -> Result<(), Vec<MigrationWizardValidationError>> {
    let mut errors = Vec::new();

    for (field, valid) in [
        (
            "page",
            page.record_kind == MIGRATION_WIZARD_PAGE_RECORD_KIND
                && page.schema_version == MIGRATION_WIZARD_SCHEMA_VERSION
                && page.shared_contract_ref == MIGRATION_WIZARD_SHARED_CONTRACT_REF,
        ),
        (
            "header",
            page.header.record_kind == MIGRATION_WIZARD_HEADER_RECORD_KIND
                && page.header.schema_version == MIGRATION_WIZARD_SCHEMA_VERSION
                && page.header.shared_contract_ref == MIGRATION_WIZARD_SHARED_CONTRACT_REF,
        ),
        (
            "mapping_report",
            page.mapping_report.record_kind == MIGRATION_WIZARD_MAPPING_REPORT_RECORD_KIND
                && page.mapping_report.schema_version == MIGRATION_WIZARD_SCHEMA_VERSION
                && page.mapping_report.shared_contract_ref == MIGRATION_WIZARD_SHARED_CONTRACT_REF,
        ),
    ] {
        if !valid {
            errors.push(MigrationWizardValidationError::EnvelopeInvalid {
                field: field.to_owned(),
            });
        }
    }

    if !has_bounded_opaque_ref(&page.import_diff_preview_ref, "import-preview:") {
        errors.push(MigrationWizardValidationError::DiffPreviewRefMissing);
    }

    if !has_bounded_opaque_ref(&page.migration_review_ref, "migration-review:")
        || page.mapping_report.migration_review_ref != page.migration_review_ref
    {
        errors.push(MigrationWizardValidationError::EvidenceJoinDrift {
            field: "migration_review_ref".to_owned(),
        });
    }

    if page.header.wizard_session_id != page.wizard_session_id
        || page.header.migration_review_ref != page.migration_review_ref
        || page.header.checkpoint_requirement_notice.requirement_ref
            != page.rollback_requirement.requirement_ref
    {
        errors.push(MigrationWizardValidationError::HeaderSessionMismatch);
    }

    if !has_bounded_opaque_ref(&page.wizard_session_id, "shell:migration-wizard:")
        || !has_bounded_opaque_ref(&page.mapping_report.mapping_report_id, "mapping-report:")
        || !has_bounded_opaque_ref(
            &page.mapping_report.shortcut_delta_report_ref,
            "shortcut_delta_digest:",
        )
        || !has_bounded_opaque_ref(
            &page.header.compatibility_report_action.target_ref,
            "compatibility-report:",
        )
        || !has_bounded_opaque_ref(
            &page.header.support_export_ref,
            "support-export:migration-wizard:",
        )
        || !has_bounded_opaque_ref(
            &page.header.issue_template_ref,
            "issue-template:migration-wizard:",
        )
    {
        errors.push(MigrationWizardValidationError::EvidenceJoinDrift {
            field: "typed_object_refs".to_owned(),
        });
    }

    let review_suffix = page.migration_review_ref.strip_prefix("migration-review:");
    if !review_suffix.is_some_and(|suffix| {
        page.wizard_session_id == format!("shell:migration-wizard:{suffix}")
            && page.import_diff_preview_ref == format!("import-preview:{suffix}")
            && page.mapping_report.mapping_report_id == format!("mapping-report:{suffix}")
            && page.mapping_report.shortcut_delta_report_ref
                == format!("shortcut_delta_digest:{suffix}")
            && page.rollback_requirement.requirement_ref == format!("rollback-requirement:{suffix}")
    }) {
        errors.push(MigrationWizardValidationError::EvidenceJoinDrift {
            field: "review_object_refs".to_owned(),
        });
    }

    if page.descriptors != page.mapping_report.descriptors
        || page.descriptors.source_descriptor
            != page.descriptors.source_classification.display_label()
        || page.descriptors.source_ecosystem_id
            != source_ecosystem_id(page.descriptors.source_classification)
        || page.descriptors.target_descriptor.len() > 256
        || !support_safe_target_descriptor(&page.descriptors.target_descriptor)
            .eq(&page.descriptors.target_descriptor)
        || page.header.target_scope != target_scope_for(&page.descriptors.target_descriptor)
        || page.header.source_tool != source_tool_chip_for(page.descriptors.source_classification)
        || page.mapping_report.rollback_requirement_ref != page.rollback_requirement.requirement_ref
    {
        errors.push(MigrationWizardValidationError::EvidenceJoinDrift {
            field: "source_target_or_requirement".to_owned(),
        });
    }

    if !page.header.answers_required_questions()
        || !header_matches_page_truth(page)
        || (page.current_stage.is_post_apply() && !page.header.aftermath_context_visible())
    {
        errors.push(MigrationWizardValidationError::HeaderIncomplete);
    }

    if !page.mapping_report.every_row_classified() {
        for row in &page.mapping_report.rows {
            if !matches!(
                row.classification,
                ImportMappingClassification::Exact
                    | ImportMappingClassification::Translated
                    | ImportMappingClassification::Partial
                    | ImportMappingClassification::Shimmed
                    | ImportMappingClassification::Unsupported
            ) {
                errors.push(MigrationWizardValidationError::UnclassifiedRow {
                    row_id: row.row_id.clone(),
                });
            }
        }
    }

    for required in [
        ImportMappingClassification::Exact,
        ImportMappingClassification::Translated,
        ImportMappingClassification::Partial,
        ImportMappingClassification::Shimmed,
        ImportMappingClassification::Unsupported,
    ] {
        if !page
            .mapping_report
            .classifications_present
            .contains(&required)
        {
            errors.push(
                MigrationWizardValidationError::MissingRequiredClassification {
                    classification: required.as_str().to_owned(),
                },
            );
        }
    }

    if !has_bounded_opaque_ref(
        &page.rollback_requirement.requirement_ref,
        "rollback-requirement:",
    ) || page.rollback_requirement.requirement_state != "required_before_apply"
        || page.rollback_requirement.checkpoint_scope != "profile_only"
        || page
            .rollback_requirement
            .required_protected_state_refs
            .is_empty()
        || has_empty_duplicate_or_unsafe_refs(
            &page.rollback_requirement.required_protected_state_refs,
        )
        || page.rollback_requirement.protected_domains.is_empty()
        || page
            .rollback_requirement
            .protected_domains
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        || !page
            .rollback_requirement
            .rollback_action_hints
            .iter()
            .map(String::as_str)
            .eq([
                "compare_before_restore",
                "restore_from_checkpoint",
                "export_for_support",
            ])
        || !review_suffix.is_some_and(|suffix| {
            page.rollback_requirement
                .required_protected_state_refs
                .iter()
                .map(String::as_str)
                .eq([
                    format!("protected-state:{suffix}:profile-settings-before-import"),
                    format!("protected-state:{suffix}:keybindings-before-import"),
                ]
                .iter()
                .map(String::as_str))
        })
        || page.rollback_requirement.narrative
            != "Apply remains blocked until execution creates and verifies a rollback checkpoint for every affected domain."
        || !is_safe_support_text(&page.rollback_requirement.narrative, 1024)
        || !page
            .mapping_report
            .rows
            .iter()
            .all(|row| row.rollback_requirement_ref == page.rollback_requirement.requirement_ref)
    {
        errors.push(MigrationWizardValidationError::RollbackRequirementMissing);
    }

    let expected_domains: Vec<ImportReviewDomain> = page
        .mapping_report
        .rows
        .iter()
        .map(|row| row.domain)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if page.rollback_requirement.protected_domains != expected_domains {
        errors.push(MigrationWizardValidationError::MappingReportDrift {
            field: "protected_domains".to_owned(),
        });
    }

    validate_mapping_report_integrity(page, &mut errors);

    for transition in &page.stage_history {
        if transition.durable_writes_authorized
            || matches!(
                transition.stage,
                WizardStage::CheckpointReady
                    | WizardStage::Applying
                    | WizardStage::Applied
                    | WizardStage::PartiallyApplied
                    | WizardStage::RolledBack
            )
        {
            errors.push(MigrationWizardValidationError::UnprovenLifecycleStage {
                stage: transition.stage.as_str().to_owned(),
            });
        }
    }

    if matches!(
        page.current_stage,
        WizardStage::CheckpointReady
            | WizardStage::Applying
            | WizardStage::Applied
            | WizardStage::PartiallyApplied
            | WizardStage::RolledBack
    ) {
        errors.push(MigrationWizardValidationError::UnprovenLifecycleStage {
            stage: page.current_stage.as_str().to_owned(),
        });
    }

    let expected_history = build_stage_history(page.current_stage, &page.mapping_report);
    if page.stage_history != expected_history {
        errors.push(MigrationWizardValidationError::PreviewLifecycleDrift {
            field: "stage_history".to_owned(),
        });
    }

    if page.apply_gate != apply_gate_for(page.current_stage, &page.mapping_report) {
        errors.push(MigrationWizardValidationError::PreviewLifecycleDrift {
            field: "apply_gate".to_owned(),
        });
    }

    if page.summary.mapping_row_count != page.mapping_report.rows.len()
        || page.summary.unsupported_gap_count != page.mapping_report.unsupported_gaps.len()
        || page.summary.compare_action_count != page.compare_actions.len()
        || page.summary.undo_action_count != page.undo_actions.len()
        || page.summary.checkpoint_minted
        || !page.summary.no_durable_writes_before_checkpoint
        || page.summary.unsupported_gaps_visible_before_apply
            != page
                .mapping_report
                .unsupported_gaps
                .iter()
                .all(|gap| gap.visible_before_apply)
        || page.summary.mapping_report_retained != page.mapping_report.retained_after_first_run
        || !page.undo_actions.is_empty()
    {
        errors.push(MigrationWizardValidationError::PreviewLifecycleDrift {
            field: "summary_or_actions".to_owned(),
        });
    }

    if page.generated_at != page.mapping_report.generated_at
        || !is_utc_timestamp(&page.generated_at)
    {
        errors.push(MigrationWizardValidationError::EnvelopeInvalid {
            field: "generated_at".to_owned(),
        });
    }

    if !page.mapping_report.retained_after_first_run {
        errors.push(MigrationWizardValidationError::MappingReportNotRetained);
    }

    for gap in &page.mapping_report.unsupported_gaps {
        if !gap.visible_before_apply {
            errors.push(
                MigrationWizardValidationError::UnsupportedGapHiddenBeforeApply {
                    gap_id: privacy_safe_gap_id(&gap.gap_id),
                },
            );
        }
    }

    for required in [
        ImportReportReopenSurface::Settings,
        ImportReportReopenSurface::Help,
        ImportReportReopenSurface::SupportExport,
    ] {
        if !page
            .mapping_report
            .reopen_links
            .iter()
            .any(|link| link.surface == required)
        {
            errors.push(MigrationWizardValidationError::ReopenLinksIncomplete {
                surface: required.as_str().to_owned(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn has_bounded_opaque_ref(reference: &str, prefix: &str) -> bool {
    reference.len() <= 256
        && reference.strip_prefix(prefix).is_some_and(|identifier| {
            !identifier.is_empty()
                && identifier.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-')
                })
        })
}

fn is_bounded_single_line(value: &str, maximum_bytes: usize) -> bool {
    !value.is_empty()
        && value == value.trim()
        && value.len() <= maximum_bytes
        && !value.chars().any(char::is_control)
}

fn is_safe_support_text(value: &str, maximum_bytes: usize) -> bool {
    is_bounded_single_line(value, maximum_bytes)
        && !value.starts_with('/')
        && !value.starts_with('\\')
        && !value.starts_with('~')
        && !value.contains("://")
        && !contains_file_scheme(value)
        && !value.contains("../")
        && !value.contains("..\\")
        && !value.as_bytes().get(1).is_some_and(|byte| *byte == b':')
        && !contains_absolute_path(value)
}

fn contains_absolute_path(value: &str) -> bool {
    value.split_whitespace().any(looks_like_absolute_path)
        || value
            .split(|character: char| {
                matches!(
                    character,
                    '=' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | ':' | '`' | '"' | '\''
                )
            })
            .any(looks_like_absolute_path)
}

fn contains_file_scheme(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.match_indices("file:").any(|(index, _)| {
        index == 0
            || lower[..index].chars().next_back().is_some_and(|character| {
                character.is_whitespace()
                    || matches!(
                        character,
                        '/' | '\\' | '=' | '(' | '[' | '{' | ',' | ';' | ':' | '`' | '"' | '\''
                    )
            })
    })
}

fn looks_like_absolute_path(token: &str) -> bool {
    let token = token.trim_matches(|character: char| {
        matches!(
            character,
            ',' | ';' | '(' | ')' | '[' | ']' | '`' | '"' | '\''
        )
    });
    (token.starts_with('/') && token.len() > 1)
        || (token.starts_with('\\') && token.len() > 1)
        || (token.starts_with('~') && token.len() > 1)
        || (token.as_bytes().get(1) == Some(&b':')
            && token
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphabetic)
            && token
                .as_bytes()
                .get(2)
                .is_some_and(|byte| matches!(*byte, b'/' | b'\\')))
}

fn privacy_safe_gap_id(gap_id: &str) -> String {
    if has_bounded_opaque_ref(gap_id, "migration-wizard-gap:") {
        gap_id.to_owned()
    } else {
        "[redacted invalid gap id]".to_owned()
    }
}

fn is_utc_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return false;
    }
    let Some(year) = timestamp_number(bytes, 0, 4) else {
        return false;
    };
    let Some(month) = timestamp_number(bytes, 5, 7) else {
        return false;
    };
    let Some(day) = timestamp_number(bytes, 8, 10) else {
        return false;
    };
    let Some(hour) = timestamp_number(bytes, 11, 13) else {
        return false;
    };
    let Some(minute) = timestamp_number(bytes, 14, 16) else {
        return false;
    };
    let Some(second) = timestamp_number(bytes, 17, 19) else {
        return false;
    };
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return false,
    };
    year >= 1970 && (1..=days_in_month).contains(&day) && hour < 24 && minute < 60 && second < 60
}

fn timestamp_number(bytes: &[u8], start: usize, end: usize) -> Option<u32> {
    bytes
        .get(start..end)?
        .iter()
        .all(u8::is_ascii_digit)
        .then_some(())?;
    std::str::from_utf8(bytes.get(start..end)?)
        .ok()?
        .parse()
        .ok()
}

fn has_empty_duplicate_or_unsafe_refs(values: &[String]) -> bool {
    values.len() > 64
        || values.iter().enumerate().any(|(index, value)| {
            !is_safe_support_text(value, 320) || values[..index].contains(value)
        })
}

fn header_matches_page_truth(page: &MigrationWizardPage) -> bool {
    let Some(suffix) = page.migration_review_ref.strip_prefix("migration-review:") else {
        return false;
    };
    let expected_partial_context = page.current_stage.is_post_apply()
        || page.mapping_report.classification_summary.partial > 0
        || page.mapping_report.classification_summary.unsupported > 0;
    let expected_downgrade_context = page.current_stage.is_post_apply()
        || page.mapping_report.classification_summary.partial > 0
        || page.mapping_report.classification_summary.shimmed > 0
        || page.mapping_report.classification_summary.unsupported > 0;
    page.header
        .checkpoint_requirement_notice
        .required_before_apply
        && page.header.checkpoint_requirement_notice.notice_label
            == format!(
                "Rollback checkpoint required before apply: {}",
                page.rollback_requirement.requirement_ref
            )
        && page.header.restore_action.action_id == format!("migration-header-restore:{suffix}")
        && page.header.restore_action.action_label == "Restore unavailable until checkpoint exists"
        && page.header.restore_action.action_token == "restore_unavailable_pending_checkpoint"
        && page.header.restore_action.target_ref == page.rollback_requirement.requirement_ref
        && !page.header.restore_action.enabled
        && page.header.compatibility_report_action.action_id
            == format!("migration-header-compatibility:{suffix}")
        && page.header.compatibility_report_action.action_label == "Open compatibility report"
        && page.header.compatibility_report_action.action_token == "open_compatibility_report"
        && page.header.compatibility_report_action.target_ref
            == format!("compatibility-report:{suffix}")
        && page.header.compatibility_report_action.enabled
        && page.header.partial_apply_context_visible == expected_partial_context
        && page.header.downgrade_context_visible == expected_downgrade_context
        && page.header.restore_context_visible
        && page.header.support_export_ref == format!("support-export:migration-wizard:{suffix}")
        && page.header.issue_template_ref == format!("issue-template:migration-wizard:{suffix}")
}

fn validate_mapping_report_integrity(
    page: &MigrationWizardPage,
    errors: &mut Vec<MigrationWizardValidationError>,
) {
    let report = &page.mapping_report;
    let source_ecosystem = source_ecosystem_id(page.descriptors.source_classification);
    let rows_have_valid_envelopes = report.rows.len() <= 4096
        && !report.rows.is_empty()
        && report.rows.iter().all(|row| {
            let row_id_prefix = format!(
                "import-diff-row:{source_ecosystem}:{}:{}:",
                row.domain.as_str(),
                row.classification.as_str()
            );
            let Some(item_suffix) = row.row_id.strip_prefix(&row_id_prefix) else {
                return false;
            };
            let expected_source_ref = format!("{source_ecosystem}:{item_suffix}");
            let expected_target_ref = format!("aureline:{}:{item_suffix}", row.domain.as_str());
            row.record_kind == MIGRATION_WIZARD_MAPPING_ROW_RECORD_KIND
                && row.schema_version == MIGRATION_WIZARD_SCHEMA_VERSION
                && has_bounded_opaque_ref(&row.row_id, &row_id_prefix)
                && row.source_item_ref == expected_source_ref
                && is_safe_support_text(&row.source_label, 320)
                && row.target_label == row.domain.display_label()
                && is_safe_support_text(&row.before_value_label, 1024)
                && is_safe_support_text(&row.after_value_label, 1024)
                && row.rollback_requirement_ref == page.rollback_requirement.requirement_ref
                && row.retained_after_apply
                && !has_empty_duplicate_or_unsafe_refs(&row.docs_help_refs)
                && !has_empty_duplicate_or_unsafe_refs(&row.support_export_refs)
                && !row.docs_help_refs.is_empty()
                && !row.support_export_refs.is_empty()
                && row.docs_help_refs.iter().all(|reference| {
                    reference.starts_with("docs/") || has_bounded_opaque_ref(reference, "help:")
                })
                && row
                    .support_export_refs
                    .iter()
                    .all(|reference| has_bounded_opaque_ref(reference, "support:"))
                && row
                    .lossy_or_unsupported_note
                    .as_deref()
                    .map(|note| is_safe_support_text(note, 1024))
                    .unwrap_or(true)
                && match row.classification {
                    ImportMappingClassification::Unsupported => {
                        row.target_item_ref.is_none() && row.lossy_or_unsupported_note.is_some()
                    }
                    ImportMappingClassification::Partial | ImportMappingClassification::Shimmed => {
                        row.target_item_ref.as_deref() == Some(expected_target_ref.as_str())
                            && row
                                .lossy_or_unsupported_note
                                .as_deref()
                                .is_some_and(|note| is_safe_support_text(note, 1024))
                    }
                    ImportMappingClassification::Exact
                    | ImportMappingClassification::Translated => {
                        row.target_item_ref.as_deref() == Some(expected_target_ref.as_str())
                    }
                }
        })
        && !report
            .rows
            .windows(2)
            .any(|pair| pair[0].row_id >= pair[1].row_id);
    if !rows_have_valid_envelopes {
        errors.push(MigrationWizardValidationError::MappingReportDrift {
            field: "rows".to_owned(),
        });
    }

    let expected_summary = WizardClassificationSummary::from_rows(&report.rows);
    let expected_classifications: Vec<ImportMappingClassification> = report
        .rows
        .iter()
        .map(|row| row.classification)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if report.classification_summary != expected_summary
        || report.classifications_present != expected_classifications
    {
        errors.push(MigrationWizardValidationError::MappingReportDrift {
            field: "classification_summary".to_owned(),
        });
    }

    let gap_rows: Vec<&WizardMappingReportRow> = report
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.classification,
                ImportMappingClassification::Shimmed | ImportMappingClassification::Unsupported
            )
        })
        .collect();
    let gaps_match_rows = report.unsupported_gaps.len() == gap_rows.len()
        && report.unsupported_gaps.len() <= 4096
        && report
            .unsupported_gaps
            .iter()
            .zip(gap_rows.iter())
            .all(|(gap, row)| {
                gap.record_kind == MIGRATION_WIZARD_UNSUPPORTED_GAP_RECORD_KIND
                    && gap.schema_version == MIGRATION_WIZARD_SCHEMA_VERSION
                    && gap.gap_id == format!("migration-wizard-gap:{}", row.row_id)
                    && gap.domain == row.domain
                    && gap.classification == row.classification
                    && gap.source_label == row.source_label
                    && gap.visible_before_apply
                    && gap.retained_after_apply
                    && gap.docs_help_refs == row.docs_help_refs
                    && gap.support_export_refs == row.support_export_refs
                    && gap.gap_summary
                        == row.lossy_or_unsupported_note.as_deref().unwrap_or_default()
                    && is_safe_support_text(&gap.gap_summary, 1024)
            });
    if !gaps_match_rows {
        errors.push(MigrationWizardValidationError::MappingReportDrift {
            field: "unsupported_gaps".to_owned(),
        });
    }

    let mut reopen_surfaces = BTreeSet::new();
    let reopen_links_valid = report.reopen_links.len() == 3
        && report.reopen_links.iter().all(|link| {
            let Some(suffix) = page.migration_review_ref.strip_prefix("migration-review:") else {
                return false;
            };
            let (expected_action_ref, expected_label) = match link.surface {
                ImportReportReopenSurface::Settings => (
                    format!("settings:profile.import_history.open_mapping_report:{suffix}"),
                    "Open migration mapping report",
                ),
                ImportReportReopenSurface::Help => (
                    format!("help:migration.open_mapping_report:{suffix}"),
                    "Open import mapping report",
                ),
                ImportReportReopenSurface::SupportExport => (
                    format!("support:export.include_mapping_report:{suffix}"),
                    "Include mapping report in support export",
                ),
            };
            reopen_surfaces.insert(link.surface.as_str())
                && link.migration_report_ref == report.mapping_report_id
                && link.shortcut_delta_report_ref == report.shortcut_delta_report_ref
                && link.action_ref == expected_action_ref
                && link.label == expected_label
        });
    if !reopen_links_valid {
        errors.push(MigrationWizardValidationError::MappingReportDrift {
            field: "reopen_links".to_owned(),
        });
    }

    let expected_domains: Vec<ImportReviewDomain> = report
        .rows
        .iter()
        .map(|row| row.domain)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut compare_domains: Vec<ImportReviewDomain> = page
        .compare_actions
        .iter()
        .map(|action| action.domain)
        .collect();
    compare_domains.sort();
    compare_domains.dedup();
    let compare_actions_valid = page.compare_actions.len() == compare_domains.len()
        && compare_domains == expected_domains
        && !page
            .compare_actions
            .windows(2)
            .any(|pair| pair[0].compare_action_id >= pair[1].compare_action_id)
        && page.compare_actions.iter().all(|action| {
            let Some(suffix) = page.migration_review_ref.strip_prefix("migration-review:") else {
                return false;
            };
            action.compare_action_id
                == format!(
                    "migration-wizard-compare:{suffix}:{}",
                    action.domain.as_str()
                )
                && action.before_state_ref == format!("before:{suffix}:{}", action.domain.as_str())
                && action.after_state_ref == format!("after:{suffix}:{}", action.domain.as_str())
                && action.action_label
                    == format!(
                        "Compare {} before and after import",
                        action.domain.display_label()
                    )
                && action.action_token == "compare_before_restore"
        });
    if !compare_actions_valid {
        errors.push(MigrationWizardValidationError::PreviewLifecycleDrift {
            field: "compare_actions".to_owned(),
        });
    }

    if page.header.restore_action.enabled
        || page.header.restore_action.target_ref != page.rollback_requirement.requirement_ref
        || page.header.restore_action.action_token != "restore_unavailable_pending_checkpoint"
        || !page.header.compatibility_report_action.enabled
        || page.header.compatibility_report_action.action_token != "open_compatibility_report"
    {
        errors.push(MigrationWizardValidationError::PreviewLifecycleDrift {
            field: "header_actions".to_owned(),
        });
    }
}

/// Builds a [`MigrationWizardPage`] from a [`ImportDiffReviewPacket`]
/// at the given target wizard stage.
///
/// The transformation is deterministic and is the only mint-from-truth
/// path used by the seeded fixture builder, the headless inspector,
/// and the integration test.
pub fn build_migration_wizard_page(
    packet: &ImportDiffReviewPacket,
    target_stage: WizardStage,
) -> MigrationWizardPage {
    let descriptors = WizardSourceTargetDescriptor {
        source_classification: packet.source_classification,
        source_descriptor: packet.source_classification.display_label().to_owned(),
        target_descriptor: support_safe_target_descriptor(&packet.destination_workspace_target),
        source_ecosystem_id: source_ecosystem_id(packet.source_classification).to_owned(),
    };

    let mut rows: Vec<WizardMappingReportRow> = packet
        .rows
        .iter()
        .map(WizardMappingReportRow::from_diff_row)
        .collect();
    rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));

    let mut classifications_present: BTreeSet<ImportMappingClassification> = BTreeSet::new();
    for row in &rows {
        classifications_present.insert(row.classification);
    }
    let classifications_present_vec: Vec<ImportMappingClassification> =
        classifications_present.into_iter().collect();

    let classification_summary = WizardClassificationSummary::from_rows(&rows);

    let unsupported_gaps = build_unsupported_gaps(packet);

    let mapping_report_id = format!("mapping-report:{}", stable_suffix(&packet.import_review_id));

    let reopen_links = vec![
        WizardReopenLink {
            surface: ImportReportReopenSurface::Settings,
            action_ref: format!(
                "settings:profile.import_history.open_mapping_report:{}",
                stable_suffix(&packet.import_review_id)
            ),
            migration_report_ref: mapping_report_id.clone(),
            shortcut_delta_report_ref: packet
                .shortcut_delta_report
                .shortcut_delta_report_id
                .clone(),
            label: "Open migration mapping report".to_owned(),
        },
        WizardReopenLink {
            surface: ImportReportReopenSurface::Help,
            action_ref: format!(
                "help:migration.open_mapping_report:{}",
                stable_suffix(&packet.import_review_id)
            ),
            migration_report_ref: mapping_report_id.clone(),
            shortcut_delta_report_ref: packet
                .shortcut_delta_report
                .shortcut_delta_report_id
                .clone(),
            label: "Open import mapping report".to_owned(),
        },
        WizardReopenLink {
            surface: ImportReportReopenSurface::SupportExport,
            action_ref: format!(
                "support:export.include_mapping_report:{}",
                stable_suffix(&packet.import_review_id)
            ),
            migration_report_ref: mapping_report_id.clone(),
            shortcut_delta_report_ref: packet
                .shortcut_delta_report
                .shortcut_delta_report_id
                .clone(),
            label: "Include mapping report in support export".to_owned(),
        },
    ];

    let mapping_report = WizardMappingReport {
        record_kind: MIGRATION_WIZARD_MAPPING_REPORT_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
        shared_contract_ref: MIGRATION_WIZARD_SHARED_CONTRACT_REF.to_owned(),
        mapping_report_id,
        migration_review_ref: packet.migration_review_ref.clone(),
        descriptors: descriptors.clone(),
        rows,
        classification_summary,
        classifications_present: classifications_present_vec,
        unsupported_gaps,
        rollback_requirement_ref: packet.rollback_requirement.requirement_ref.clone(),
        shortcut_delta_report_ref: packet
            .shortcut_delta_report
            .shortcut_delta_report_id
            .clone(),
        retained_after_first_run: true,
        reopen_links,
        generated_at: GENERATED_AT.to_owned(),
    };

    let protected_domains = protected_domains_for(packet);
    let rollback_requirement = WizardRollbackRequirementBinding {
        requirement_ref: packet.rollback_requirement.requirement_ref.clone(),
        requirement_state: packet.rollback_requirement.requirement_state.clone(),
        checkpoint_scope: packet.rollback_requirement.checkpoint_scope.clone(),
        required_protected_state_refs: packet
            .rollback_requirement
            .required_protected_state_refs
            .clone(),
        protected_domains,
        rollback_action_hints: packet.rollback_requirement.rollback_action_hints.clone(),
        narrative: "Apply remains blocked until execution creates and verifies a rollback checkpoint for every affected domain."
            .to_owned(),
    };

    let compare_actions = build_compare_actions(packet);
    let undo_actions = Vec::new();

    // This constructor receives only a dry-run packet. Do not allow a caller's
    // requested presentation stage to manufacture execution lifecycle evidence.
    let current_stage = preview_evidenced_stage(target_stage);

    let stage_history = build_stage_history(current_stage, &mapping_report);
    let apply_gate = apply_gate_for(current_stage, &mapping_report);

    let summary = WizardSummary {
        mapping_row_count: mapping_report.rows.len(),
        unsupported_gap_count: mapping_report.unsupported_gaps.len(),
        compare_action_count: compare_actions.len(),
        undo_action_count: undo_actions.len(),
        checkpoint_minted: false,
        no_durable_writes_before_checkpoint: stage_history
            .iter()
            .all(|transition| !transition.durable_writes_authorized),
        unsupported_gaps_visible_before_apply: mapping_report
            .unsupported_gaps
            .iter()
            .all(|gap| gap.visible_before_apply),
        mapping_report_retained: mapping_report.retained_after_first_run,
    };

    let wizard_session_id = format!(
        "shell:migration-wizard:{}",
        stable_suffix(&packet.import_review_id)
    );
    let header = build_session_header(packet, &wizard_session_id, current_stage, &mapping_report);

    MigrationWizardPage {
        record_kind: MIGRATION_WIZARD_PAGE_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
        shared_contract_ref: MIGRATION_WIZARD_SHARED_CONTRACT_REF.to_owned(),
        wizard_session_id,
        migration_review_ref: packet.migration_review_ref.clone(),
        header,
        current_stage,
        stage_history,
        descriptors,
        import_diff_preview_ref: packet.import_diff_preview_ref.clone(),
        mapping_report,
        rollback_requirement,
        compare_actions,
        undo_actions,
        apply_gate,
        summary,
        generated_at: GENERATED_AT.to_owned(),
    }
}

fn build_session_header(
    packet: &ImportDiffReviewPacket,
    wizard_session_id: &str,
    target_stage: WizardStage,
    mapping_report: &WizardMappingReport,
) -> MigrationSessionHeader {
    let suffix = stable_suffix(&packet.import_review_id);
    let target_scope = target_scope_for(&support_safe_target_descriptor(
        &packet.destination_workspace_target,
    ));
    let source_tool = source_tool_chip_for(packet.source_classification);
    MigrationSessionHeader {
        record_kind: MIGRATION_WIZARD_HEADER_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
        shared_contract_ref: MIGRATION_WIZARD_SHARED_CONTRACT_REF.to_owned(),
        wizard_session_id: wizard_session_id.to_owned(),
        migration_review_ref: packet.migration_review_ref.clone(),
        source_tool,
        target_scope,
        checkpoint_requirement_notice: MigrationCheckpointRequirementNotice {
            requirement_ref: packet.rollback_requirement.requirement_ref.clone(),
            required_before_apply: packet
                .rollback_requirement
                .requires_checkpoint_before_apply(),
            notice_label: format!(
                "Rollback checkpoint required before apply: {}",
                packet.rollback_requirement.requirement_ref
            ),
        },
        restore_action: MigrationHeaderAction {
            action_id: format!("migration-header-restore:{suffix}"),
            action_label: "Restore unavailable until checkpoint exists".to_owned(),
            action_token: "restore_unavailable_pending_checkpoint".to_owned(),
            target_ref: packet.rollback_requirement.requirement_ref.clone(),
            enabled: false,
        },
        compatibility_report_action: MigrationHeaderAction {
            action_id: format!("migration-header-compatibility:{suffix}"),
            action_label: "Open compatibility report".to_owned(),
            action_token: "open_compatibility_report".to_owned(),
            target_ref: format!("compatibility-report:{suffix}"),
            enabled: true,
        },
        partial_apply_context_visible: target_stage.is_post_apply()
            || mapping_report.classification_summary.partial > 0
            || mapping_report.classification_summary.unsupported > 0,
        downgrade_context_visible: target_stage.is_post_apply()
            || mapping_report.classification_summary.partial > 0
            || mapping_report.classification_summary.shimmed > 0
            || mapping_report.classification_summary.unsupported > 0,
        restore_context_visible: true,
        support_export_ref: format!("support-export:migration-wizard:{suffix}"),
        issue_template_ref: format!("issue-template:migration-wizard:{suffix}"),
    }
}

fn source_tool_chip_for(classification: CompetitorConfigClassification) -> MigrationSourceToolChip {
    match classification {
        CompetitorConfigClassification::VSCodeWorkspaceRoot => MigrationSourceToolChip {
            source_tool_id: "source-tool:vs_code_code_oss".to_owned(),
            source_tool_label: "VS Code / Code OSS".to_owned(),
            source_version_label: "version not read (marker-only)".to_owned(),
            version_truth_class: SourceVersionTruthClass::MarkerOnlyVersionUnknown,
            version_evidence_ref: "source-marker:.vscode".to_owned(),
        },
        CompetitorConfigClassification::JetBrainsIdeaRoot => MigrationSourceToolChip {
            source_tool_id: "source-tool:jetbrains_family".to_owned(),
            source_tool_label: "JetBrains IDE family".to_owned(),
            source_version_label: "version not read (marker-only)".to_owned(),
            version_truth_class: SourceVersionTruthClass::MarkerOnlyVersionUnknown,
            version_evidence_ref: "source-marker:.idea".to_owned(),
        },
        CompetitorConfigClassification::UnknownConfigRoot => MigrationSourceToolChip {
            source_tool_id: "source-tool:unknown".to_owned(),
            source_tool_label: "Unknown source tool".to_owned(),
            source_version_label: "version unavailable".to_owned(),
            version_truth_class: SourceVersionTruthClass::MarkerOnlyVersionUnknown,
            version_evidence_ref: "source-marker:unresolved".to_owned(),
        },
    }
}

fn target_scope_for(target: &str) -> MigrationTargetScope {
    let (scope_kind, target_label) = if target.starts_with("workspace:") {
        (
            MigrationTargetScopeKind::Workspace,
            target.trim_start_matches("workspace:").to_owned(),
        )
    } else if target.starts_with("profile:") {
        (
            MigrationTargetScopeKind::Profile,
            target.trim_start_matches("profile:").to_owned(),
        )
    } else {
        (MigrationTargetScopeKind::Profile, target.to_owned())
    };
    MigrationTargetScope {
        scope_kind,
        target_ref: target.to_owned(),
        target_label,
        writes_land_in: format!("{} {}", scope_kind.as_str(), target),
    }
}

/// Seeded fixture builder used by the headless inspector and the
/// integration test. The seed mirrors the JSON checked in under
/// `fixtures/migration/m3/migration_wizard/`.
pub fn seeded_migration_wizard_page() -> MigrationWizardPage {
    let review = seeded_review_record();
    let packet = materialize_import_diff_review_packet(&review);
    build_migration_wizard_page(&packet, WizardStage::PreviewReady)
}

fn seeded_review_record() -> ImportReviewRecord {
    use std::path::Path;
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/import/m1_classifier_cases/vscode_workspace");
    CompetitorConfigClassifier::new().build_review(&fixture_root, "profile:default")
}

fn build_unsupported_gaps(packet: &ImportDiffReviewPacket) -> Vec<UnsupportedGapRow> {
    let mut gaps = Vec::new();
    for row in &packet.rows {
        if matches!(
            row.classification,
            ImportMappingClassification::Unsupported | ImportMappingClassification::Shimmed
        ) {
            let summary = row.lossy_or_unsupported_note.clone().unwrap_or_else(|| {
                format!("{} requires manual review.", row.domain.display_label())
            });
            gaps.push(UnsupportedGapRow {
                record_kind: MIGRATION_WIZARD_UNSUPPORTED_GAP_RECORD_KIND.to_owned(),
                schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
                gap_id: format!("migration-wizard-gap:{}", row.row_id),
                domain: row.domain,
                classification: row.classification,
                source_label: row.source_label.clone(),
                gap_summary: summary,
                visible_before_apply: true,
                retained_after_apply: true,
                docs_help_refs: row.docs_help_refs.clone(),
                support_export_refs: row.support_export_refs.clone(),
            });
        }
    }
    gaps.sort_by(|left, right| left.gap_id.cmp(&right.gap_id));
    gaps
}

fn protected_domains_for(packet: &ImportDiffReviewPacket) -> Vec<ImportReviewDomain> {
    let mut domains: BTreeSet<ImportReviewDomain> = BTreeSet::new();
    for row in &packet.rows {
        domains.insert(row.domain);
    }
    domains.into_iter().collect()
}

fn build_compare_actions(packet: &ImportDiffReviewPacket) -> Vec<WizardCompareAction> {
    let mut actions = Vec::new();
    let mut seen_domains: BTreeSet<ImportReviewDomain> = BTreeSet::new();
    for row in &packet.rows {
        if seen_domains.insert(row.domain) {
            actions.push(WizardCompareAction {
                compare_action_id: format!(
                    "migration-wizard-compare:{}:{}",
                    stable_suffix(&packet.import_review_id),
                    row.domain.as_str()
                ),
                domain: row.domain,
                before_state_ref: row.before_state_ref.clone(),
                after_state_ref: row.after_state_ref.clone(),
                action_label: format!(
                    "Compare {} before and after import",
                    row.domain.display_label()
                ),
                action_token: "compare_before_restore".to_owned(),
            });
        }
    }
    actions.sort_by(|left, right| left.compare_action_id.cmp(&right.compare_action_id));
    actions
}

fn preview_evidenced_stage(requested: WizardStage) -> WizardStage {
    match requested {
        WizardStage::SelectingSource
        | WizardStage::SourceDetected
        | WizardStage::PreviewReady
        | WizardStage::Blocked => requested,
        WizardStage::CheckpointReady
        | WizardStage::Applying
        | WizardStage::Applied
        | WizardStage::PartiallyApplied
        | WizardStage::RolledBack => WizardStage::PreviewReady,
    }
}

fn build_stage_history(
    target_stage: WizardStage,
    mapping_report: &WizardMappingReport,
) -> Vec<WizardStageTransition> {
    let ordered = [
        WizardStage::SelectingSource,
        WizardStage::SourceDetected,
        WizardStage::PreviewReady,
        WizardStage::CheckpointReady,
        WizardStage::Applying,
        WizardStage::Applied,
    ];
    let mut history = Vec::new();
    for stage in ordered {
        if (stage as u8) > (target_stage as u8) {
            break;
        }
        let durable_writes_authorized = matches!(
            stage,
            WizardStage::Applying | WizardStage::Applied | WizardStage::PartiallyApplied
        );
        history.push(WizardStageTransition {
            stage,
            durable_writes_authorized,
            summary: stage_summary(stage, mapping_report),
        });
        if stage == target_stage {
            return history;
        }
    }
    if matches!(
        target_stage,
        WizardStage::PartiallyApplied | WizardStage::Blocked | WizardStage::RolledBack
    ) {
        let durable_writes_authorized = matches!(
            target_stage,
            WizardStage::PartiallyApplied | WizardStage::RolledBack
        );
        history.push(WizardStageTransition {
            stage: target_stage,
            durable_writes_authorized,
            summary: stage_summary(target_stage, mapping_report),
        });
    }
    history
}

fn stage_summary(stage: WizardStage, report: &WizardMappingReport) -> String {
    match stage {
        WizardStage::SelectingSource => "User asked to import an existing tool profile.".to_owned(),
        WizardStage::SourceDetected => {
            format!("Source detected: {}.", report.descriptors.source_descriptor)
        }
        WizardStage::PreviewReady => format!(
            "Preview ready: {} mapping row(s) classified, {} gap(s) visible.",
            report.rows.len(),
            report.unsupported_gaps.len()
        ),
        WizardStage::CheckpointReady => format!(
            "Rollback checkpoint evidence must satisfy requirement {} before apply.",
            report.rollback_requirement_ref
        ),
        WizardStage::Applying => {
            "Apply running against the reviewed preview and checkpoint.".to_owned()
        }
        WizardStage::Applied => {
            "Apply completed; mapping report retained for compare and undo.".to_owned()
        }
        WizardStage::PartiallyApplied => {
            "Apply partially completed; blocked rows remain visible in the report.".to_owned()
        }
        WizardStage::Blocked => {
            "Apply was denied by a pre-apply gate; durable state was not mutated.".to_owned()
        }
        WizardStage::RolledBack => {
            "Undo path triggered; checkpoint restored prior state.".to_owned()
        }
    }
}

fn apply_gate_for(stage: WizardStage, report: &WizardMappingReport) -> WizardApplyGate {
    match stage {
        WizardStage::SelectingSource | WizardStage::SourceDetected => {
            WizardApplyGate::BlockedNoCheckpoint
        }
        WizardStage::PreviewReady => {
            if has_blocking_review(report) {
                WizardApplyGate::RequiresManualReview
            } else {
                WizardApplyGate::BlockedNoCheckpoint
            }
        }
        WizardStage::CheckpointReady => WizardApplyGate::BlockedNoCheckpoint,
        WizardStage::Applying
        | WizardStage::Applied
        | WizardStage::PartiallyApplied
        | WizardStage::RolledBack => WizardApplyGate::AlreadyApplied,
        WizardStage::Blocked => WizardApplyGate::RequiresManualReview,
    }
}

fn has_blocking_review(report: &WizardMappingReport) -> bool {
    report.classification_summary.unsupported > 0
        || report
            .unsupported_gaps
            .iter()
            .any(|gap| gap.classification == ImportMappingClassification::Unsupported)
}

fn source_ecosystem_id(classification: CompetitorConfigClassification) -> &'static str {
    match classification {
        CompetitorConfigClassification::VSCodeWorkspaceRoot => "vs_code_code_oss",
        CompetitorConfigClassification::JetBrainsIdeaRoot => "jetbrains_family",
        CompetitorConfigClassification::UnknownConfigRoot => "generic_import",
    }
}

fn stable_suffix(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut previous_dash = false;
    for ch in value.chars() {
        let next = if ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' {
            previous_dash = false;
            ch.to_ascii_lowercase()
        } else {
            if previous_dash {
                continue;
            }
            previous_dash = true;
            '-'
        };
        out.push(next);
    }
    out.trim_matches('-').to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_wizard_page_passes_validation() {
        let page = seeded_migration_wizard_page();
        validate_migration_wizard_page(&page).expect("seeded wizard page must validate");
    }

    #[test]
    fn seeded_wizard_page_classifies_every_required_class() {
        let page = seeded_migration_wizard_page();
        assert!(page.mapping_report.covers_every_required_classification());
        assert!(page.mapping_report.every_row_classified());
    }

    #[test]
    fn seeded_wizard_stays_preview_only_without_execution_evidence() {
        let page = seeded_migration_wizard_page();
        assert_eq!(page.current_stage, WizardStage::PreviewReady);
        assert!(!page.summary.checkpoint_minted);
        assert!(!page.header.restore_action.enabled);
        assert!(!page.post_apply_paths_are_visible());
        assert!(!page.compare_actions.is_empty());
        assert!(page.undo_actions.is_empty());
    }

    #[test]
    fn unsupported_gaps_are_visible_before_apply() {
        let page = seeded_migration_wizard_page();
        assert!(!page.mapping_report.unsupported_gaps.is_empty());
        assert!(page
            .mapping_report
            .unsupported_gaps
            .iter()
            .all(|gap| gap.visible_before_apply));
        assert!(page
            .mapping_report
            .unsupported_gaps
            .iter()
            .all(|gap| gap.retained_after_apply));
    }

    #[test]
    fn preview_never_authorizes_durable_writes() {
        let page = seeded_migration_wizard_page();
        for transition in &page.stage_history {
            assert!(!transition.durable_writes_authorized);
        }
    }

    #[test]
    fn validation_flags_unproven_apply_lifecycle() {
        let mut page = seeded_migration_wizard_page();
        page.stage_history = vec![
            WizardStageTransition {
                stage: WizardStage::SelectingSource,
                durable_writes_authorized: false,
                summary: "select".to_owned(),
            },
            WizardStageTransition {
                stage: WizardStage::Applying,
                durable_writes_authorized: true,
                summary: "applying".to_owned(),
            },
        ];
        let errors =
            validate_migration_wizard_page(&page).expect_err("must flag unproven apply stage");
        assert!(errors.iter().any(|err| matches!(
            err,
            MigrationWizardValidationError::UnprovenLifecycleStage { .. }
        )));
    }

    #[test]
    fn validation_flags_hidden_unsupported_gap() {
        let mut page = seeded_migration_wizard_page();
        page.mapping_report.unsupported_gaps[0].visible_before_apply = false;
        let errors =
            validate_migration_wizard_page(&page).expect_err("must flag hidden unsupported gap");
        assert!(errors.iter().any(|err| matches!(
            err,
            MigrationWizardValidationError::UnsupportedGapHiddenBeforeApply { .. }
        )));
    }

    #[test]
    fn validation_redacts_private_gap_ids_from_errors() {
        let mut page = seeded_migration_wizard_page();
        page.mapping_report.unsupported_gaps[0].gap_id =
            "/Users/alice/Secret Project/extensions.json".to_owned();
        page.mapping_report.unsupported_gaps[0].visible_before_apply = false;

        let errors =
            validate_migration_wizard_page(&page).expect_err("private gap id must fail closed");
        let rendered = format!("{errors:?}");
        assert!(rendered.contains("[redacted invalid gap id]"));
        for forbidden in ["/Users/alice", "Secret Project", "extensions.json"] {
            assert!(!rendered.contains(forbidden), "error leaked {forbidden:?}");
        }
    }

    #[test]
    fn validation_rejects_private_support_labels_and_invalid_timestamps() {
        let mut page = seeded_migration_wizard_page();
        page.mapping_report.rows[0].before_value_label =
            "Compare /Users/alice/Secret Project/settings.json before apply".to_owned();
        page.generated_at = "2026-02-30T00:00:00Z".to_owned();

        let errors = validate_migration_wizard_page(&page)
            .expect_err("private labels and impossible timestamps must fail closed");
        assert!(errors.iter().any(|error| matches!(
            error,
            MigrationWizardValidationError::MappingReportDrift { field } if field == "rows"
        )));
        assert!(errors.iter().any(|error| matches!(
            error,
            MigrationWizardValidationError::EnvelopeInvalid { field }
                if field == "generated_at"
        )));
        assert!(MigrationWizardSupportExport::from_page(
            "support-export:migration-wizard:private-label",
            page
        )
        .is_err());
    }

    #[test]
    fn support_text_rejects_trim_drift_file_uris_and_delimited_paths() {
        assert!(is_safe_support_text("VS Code / Code OSS", 320));
        for private in [
            " leading whitespace",
            "trailing whitespace ",
            "file:/Users/alice/private.json",
            "target=/Users/alice/private.json",
            "target:\\Users\\alice\\private.json",
        ] {
            assert!(
                !is_safe_support_text(private, 320),
                "accepted private support text {private:?}"
            );
        }
    }

    #[test]
    fn validation_flags_a_post_apply_stage_without_execution_evidence() {
        let mut page = seeded_migration_wizard_page();
        page.current_stage = WizardStage::Applied;
        let errors =
            validate_migration_wizard_page(&page).expect_err("must flag unproven post-apply state");
        assert!(errors.iter().any(|err| matches!(
            err,
            MigrationWizardValidationError::UnprovenLifecycleStage { .. }
        )));
    }

    #[test]
    fn validation_flags_unretained_report() {
        let mut page = seeded_migration_wizard_page();
        page.mapping_report.retained_after_first_run = false;
        let errors =
            validate_migration_wizard_page(&page).expect_err("must flag unretained report");
        assert!(errors.iter().any(|err| matches!(
            err,
            MigrationWizardValidationError::MappingReportNotRetained
        )));
    }

    #[test]
    fn validation_recomputes_mapping_and_gap_truth_before_export() {
        let mut page = seeded_migration_wizard_page();
        page.mapping_report.classification_summary.exact += 1;
        page.mapping_report.unsupported_gaps[0].source_label = "different-source".to_owned();
        page.summary.mapping_row_count += 1;

        let errors = validate_migration_wizard_page(&page)
            .expect_err("derived mapping and summary truth must be exact");
        assert!(errors.iter().any(|error| matches!(
            error,
            MigrationWizardValidationError::MappingReportDrift { .. }
        )));
        assert!(errors.iter().any(|error| matches!(
            error,
            MigrationWizardValidationError::PreviewLifecycleDrift { .. }
        )));
        assert!(MigrationWizardSupportExport::from_page(
            "support-export:migration-wizard:tampered",
            page
        )
        .is_err());
    }

    #[test]
    fn wizard_support_projection_hashes_raw_target_paths_and_urls() {
        let review = CompetitorConfigClassifier::new().build_review(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../fixtures/import/m1_classifier_cases/vscode_workspace"),
            "https://alice@example.invalid/Secret Workspace?token=abc",
        );
        let packet = materialize_import_diff_review_packet(&review);
        let page = build_migration_wizard_page(&packet, WizardStage::PreviewReady);
        validate_migration_wizard_page(&page).expect("redacted target page validates");
        let serialized = serde_json::to_string(
            &MigrationWizardSupportExport::from_page(
                "support-export:migration-wizard:redacted-target",
                page,
            )
            .expect("support projection"),
        )
        .expect("serialize support projection");
        for forbidden in ["alice@example.invalid", "Secret Workspace", "token=abc"] {
            assert!(
                !serialized.contains(forbidden),
                "support export leaked {forbidden:?}"
            );
        }
        assert!(serialized.contains("destination-target:"));
    }

    #[test]
    fn support_export_quotes_every_case_id() {
        let page = seeded_migration_wizard_page();
        let export = MigrationWizardSupportExport::from_page(
            "support-export:migration-wizard:001",
            page.clone(),
        )
        .expect("seeded page produces support export");
        assert_eq!(
            export.shared_contract_ref,
            MIGRATION_WIZARD_SHARED_CONTRACT_REF
        );
        assert!(export.case_ids.contains(&page.wizard_session_id));
        assert!(export
            .case_ids
            .contains(&page.mapping_report.mapping_report_id));
        assert!(export
            .case_ids
            .contains(&page.rollback_requirement.requirement_ref));
        for row in &page.mapping_report.rows {
            assert!(export.case_ids.contains(&row.row_id));
        }
        for gap in &page.mapping_report.unsupported_gaps {
            assert!(export.case_ids.contains(&gap.gap_id));
        }
        for compare in &page.compare_actions {
            assert!(export.case_ids.contains(&compare.compare_action_id));
        }
        for undo in &page.undo_actions {
            assert!(export.case_ids.contains(&undo.undo_action_id));
        }
    }

    #[test]
    fn dry_run_builder_refuses_to_invent_later_stages() {
        let review = seeded_review_record();
        let packet = materialize_import_diff_review_packet(&review);
        let pre_checkpoint = build_migration_wizard_page(&packet, WizardStage::PreviewReady);
        assert!(matches!(
            pre_checkpoint.apply_gate,
            WizardApplyGate::RequiresManualReview | WizardApplyGate::BlockedNoCheckpoint
        ));
        let checkpoint_ready = build_migration_wizard_page(&packet, WizardStage::CheckpointReady);
        assert_eq!(checkpoint_ready.current_stage, WizardStage::PreviewReady);
        assert!(!checkpoint_ready.summary.checkpoint_minted);
        assert!(matches!(
            checkpoint_ready.apply_gate,
            WizardApplyGate::RequiresManualReview | WizardApplyGate::BlockedNoCheckpoint
        ));
        let applied = build_migration_wizard_page(&packet, WizardStage::Applied);
        assert_eq!(applied.current_stage, WizardStage::PreviewReady);
        assert!(matches!(
            applied.apply_gate,
            WizardApplyGate::RequiresManualReview | WizardApplyGate::BlockedNoCheckpoint
        ));
    }
}
