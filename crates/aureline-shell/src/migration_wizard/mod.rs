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
//! - a [`WizardRollbackCheckpointBinding`] that proves a rollback
//!   checkpoint exists before durable state is mutated;
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
    materialize_import_diff_review_packet, ImportDiffReviewPacket, ImportDiffReviewRow,
    ImportMappingClassification, ImportReportReopenSurface, ImportReviewDomain,
};
use crate::import::{
    CompetitorConfigClassification, CompetitorConfigClassifier, ImportReviewRecord,
};

/// Beta migration wizard schema version exported with every record.
pub const MIGRATION_WIZARD_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta migration-wizard row.
pub const MIGRATION_WIZARD_SHARED_CONTRACT_REF: &str = "shell:migration_wizard_beta:v1";

/// Stable record kind for [`MigrationWizardPage`] payloads.
pub const MIGRATION_WIZARD_PAGE_RECORD_KIND: &str = "shell_migration_wizard_beta_page_record";

/// Stable record kind for [`MigrationWizardSupportExport`] payloads.
pub const MIGRATION_WIZARD_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_migration_wizard_beta_support_export_record";

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

/// Pre-apply rollback checkpoint binding required before apply.
///
/// The wizard ingests the checkpoint produced by
/// [`crate::import::diff_review`]. The binding adds the explicit
/// "no durable writes until this row is present" invariant that the
/// validator enforces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WizardRollbackCheckpointBinding {
    /// Checkpoint ref minted before apply.
    pub checkpoint_ref: String,
    /// Migration restore record paired with the checkpoint.
    pub restore_record_ref: String,
    /// Migration session that owns the checkpoint.
    pub migration_session_ref: String,
    /// True when the checkpoint was minted before apply was allowed.
    pub created_before_apply: bool,
    /// True when the checkpoint protects every domain the apply may touch.
    pub protects_every_domain: bool,
    /// Domains the checkpoint protects, in deterministic order.
    pub protected_domains: Vec<ImportReviewDomain>,
    /// Action hints surfaced to the user for compare / restore / export.
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
    /// Rollback checkpoint ref that protects this row.
    pub rollback_checkpoint_ref: String,
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
            rollback_checkpoint_ref: row.rollback_checkpoint_ref.clone(),
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
    /// Migration session ref the report was generated from.
    pub migration_session_ref: String,
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
    /// Rollback checkpoint ref retained with the report.
    pub rollback_checkpoint_ref: String,
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
    /// True when the wizard is at or past `CheckpointReady`.
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
    /// Migration session ref the wizard owns.
    pub migration_session_ref: String,
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
    /// Rollback checkpoint binding the wizard minted before apply.
    pub rollback_checkpoint: WizardRollbackCheckpointBinding,
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
            "wizard: stage={}, rows={}, gaps={}, checkpoint={}",
            self.current_stage.display_label(),
            self.mapping_report.rows.len(),
            self.mapping_report.unsupported_gaps.len(),
            self.rollback_checkpoint.checkpoint_ref
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
    /// Wizard page quoted in full.
    pub page: MigrationWizardPage,
    /// Stable wizard session id, mapping row ids, gap ids, and
    /// compare/undo action ids in deterministic order.
    pub case_ids: Vec<String>,
}

impl MigrationWizardSupportExport {
    /// Builds the support-export wrapper for a wizard page.
    pub fn from_page(support_export_id: impl Into<String>, page: MigrationWizardPage) -> Self {
        let mut case_ids = Vec::new();
        case_ids.push(page.wizard_session_id.clone());
        case_ids.push(page.mapping_report.mapping_report_id.clone());
        case_ids.push(page.rollback_checkpoint.checkpoint_ref.clone());
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
        Self {
            record_kind: MIGRATION_WIZARD_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
            shared_contract_ref: MIGRATION_WIZARD_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            page,
            case_ids,
        }
    }
}

/// Validation error produced by [`validate_migration_wizard_page`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum MigrationWizardValidationError {
    /// At least one row is missing a classification.
    UnclassifiedRow { row_id: String },
    /// The mapping report does not cover every required classification.
    MissingRequiredClassification { classification: String },
    /// An apply-stage transition admitted durable writes without a
    /// rollback checkpoint.
    ApplyAuthorizedBeforeCheckpoint { stage: String },
    /// The rollback checkpoint was not created before apply.
    CheckpointNotCreatedBeforeApply,
    /// The mapping report is not retained after first run.
    MappingReportNotRetained,
    /// At least one unsupported gap is hidden before apply.
    UnsupportedGapHiddenBeforeApply { gap_id: String },
    /// A post-apply stage is missing a compare or undo path.
    PostApplyPathsMissing { stage: String },
    /// The reopen links are missing one of settings / help / support_export.
    ReopenLinksIncomplete { surface: String },
    /// The diff preview ref is empty.
    DiffPreviewRefMissing,
}

/// Validates a wizard page against the M3 acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_migration_wizard_page(
    page: &MigrationWizardPage,
) -> Result<(), Vec<MigrationWizardValidationError>> {
    let mut errors = Vec::new();

    if page.import_diff_preview_ref.trim().is_empty() {
        errors.push(MigrationWizardValidationError::DiffPreviewRefMissing);
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

    if !page.rollback_checkpoint.created_before_apply {
        errors.push(MigrationWizardValidationError::CheckpointNotCreatedBeforeApply);
    }

    let mut checkpoint_seen = false;
    for transition in &page.stage_history {
        if matches!(transition.stage, WizardStage::CheckpointReady) {
            checkpoint_seen = true;
        }
        if transition.durable_writes_authorized && !checkpoint_seen {
            errors.push(
                MigrationWizardValidationError::ApplyAuthorizedBeforeCheckpoint {
                    stage: transition.stage.as_str().to_owned(),
                },
            );
        }
    }

    if !page.mapping_report.retained_after_first_run {
        errors.push(MigrationWizardValidationError::MappingReportNotRetained);
    }

    for gap in &page.mapping_report.unsupported_gaps {
        if !gap.visible_before_apply {
            errors.push(
                MigrationWizardValidationError::UnsupportedGapHiddenBeforeApply {
                    gap_id: gap.gap_id.clone(),
                },
            );
        }
    }

    if page.current_stage.is_post_apply()
        && (page.compare_actions.is_empty() || page.undo_actions.is_empty())
    {
        errors.push(MigrationWizardValidationError::PostApplyPathsMissing {
            stage: page.current_stage.as_str().to_owned(),
        });
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
        target_descriptor: packet.destination_workspace_target.clone(),
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
        migration_session_ref: packet.migration_session_ref.clone(),
        descriptors: descriptors.clone(),
        rows,
        classification_summary,
        classifications_present: classifications_present_vec,
        unsupported_gaps,
        rollback_checkpoint_ref: packet.rollback_checkpoint.checkpoint_ref.clone(),
        shortcut_delta_report_ref: packet
            .shortcut_delta_report
            .shortcut_delta_report_id
            .clone(),
        retained_after_first_run: true,
        reopen_links,
        generated_at: GENERATED_AT.to_owned(),
    };

    let protected_domains = protected_domains_for(packet);
    let rollback_checkpoint = WizardRollbackCheckpointBinding {
        checkpoint_ref: packet.rollback_checkpoint.checkpoint_ref.clone(),
        restore_record_ref: packet.rollback_checkpoint.restore_record_ref.clone(),
        migration_session_ref: packet.migration_session_ref.clone(),
        created_before_apply: packet.rollback_checkpoint.created_before_apply,
        protects_every_domain: !protected_domains.is_empty(),
        protected_domains,
        rollback_action_hints: packet.rollback_checkpoint.rollback_action_hints.clone(),
        narrative:
            "Rollback checkpoint minted before apply protects every domain the wizard may touch."
                .to_owned(),
    };

    let compare_actions = build_compare_actions(packet);
    let undo_actions = build_undo_actions(packet);

    let stage_history = build_stage_history(target_stage, &mapping_report);
    let apply_gate = apply_gate_for(target_stage, packet, &mapping_report);

    let summary = WizardSummary {
        mapping_row_count: mapping_report.rows.len(),
        unsupported_gap_count: mapping_report.unsupported_gaps.len(),
        compare_action_count: compare_actions.len(),
        undo_action_count: undo_actions.len(),
        checkpoint_minted: matches!(
            target_stage,
            WizardStage::CheckpointReady
                | WizardStage::Applying
                | WizardStage::Applied
                | WizardStage::PartiallyApplied
                | WizardStage::Blocked
                | WizardStage::RolledBack
        ),
        no_durable_writes_before_checkpoint: stage_history.iter().all(|t| {
            !t.durable_writes_authorized || t.stage as u8 >= WizardStage::CheckpointReady as u8
        }),
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

    MigrationWizardPage {
        record_kind: MIGRATION_WIZARD_PAGE_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_WIZARD_SCHEMA_VERSION,
        shared_contract_ref: MIGRATION_WIZARD_SHARED_CONTRACT_REF.to_owned(),
        wizard_session_id,
        migration_session_ref: packet.migration_session_ref.clone(),
        current_stage: target_stage,
        stage_history,
        descriptors,
        import_diff_preview_ref: packet.import_diff_preview_ref.clone(),
        mapping_report,
        rollback_checkpoint,
        compare_actions,
        undo_actions,
        apply_gate,
        summary,
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Seeded fixture builder used by the headless inspector and the
/// integration test. The seed mirrors the JSON checked in under
/// `fixtures/migration/m3/migration_wizard/`.
pub fn seeded_migration_wizard_page() -> MigrationWizardPage {
    let review = seeded_review_record();
    let packet = materialize_import_diff_review_packet(&review);
    build_migration_wizard_page(&packet, WizardStage::Applied)
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

fn build_undo_actions(packet: &ImportDiffReviewPacket) -> Vec<WizardUndoAction> {
    let mut actions = Vec::new();
    let suffix = stable_suffix(&packet.import_review_id);
    actions.push(WizardUndoAction {
        undo_action_id: format!("migration-wizard-undo:{suffix}:restore-from-checkpoint"),
        checkpoint_ref: packet.rollback_checkpoint.checkpoint_ref.clone(),
        restore_record_ref: packet.rollback_checkpoint.restore_record_ref.clone(),
        requires_confirmation: true,
        action_label: "Restore from rollback checkpoint".to_owned(),
        action_token: "restore_from_checkpoint".to_owned(),
    });
    actions.push(WizardUndoAction {
        undo_action_id: format!("migration-wizard-undo:{suffix}:export-for-support"),
        checkpoint_ref: packet.rollback_checkpoint.checkpoint_ref.clone(),
        restore_record_ref: packet.rollback_checkpoint.restore_record_ref.clone(),
        requires_confirmation: false,
        action_label: "Export migration report for support".to_owned(),
        action_token: "export_for_support".to_owned(),
    });
    actions
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
            "Rollback checkpoint {} minted before apply.",
            report.rollback_checkpoint_ref
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

fn apply_gate_for(
    stage: WizardStage,
    packet: &ImportDiffReviewPacket,
    report: &WizardMappingReport,
) -> WizardApplyGate {
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
        WizardStage::CheckpointReady => {
            if packet.apply_gate_class == "requires_manual_review" {
                WizardApplyGate::RequiresManualReview
            } else if packet.apply_gate_class == "stale_requires_replan" {
                WizardApplyGate::StaleRequiresReplan
            } else {
                WizardApplyGate::AllowedCheckpointReady
            }
        }
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
    fn seeded_wizard_exposes_post_apply_paths() {
        let page = seeded_migration_wizard_page();
        assert!(page.post_apply_paths_are_visible());
        assert!(!page.compare_actions.is_empty());
        assert!(!page.undo_actions.is_empty());
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
    fn checkpoint_is_required_before_durable_writes() {
        let page = seeded_migration_wizard_page();
        let mut seen_checkpoint = false;
        for transition in &page.stage_history {
            if matches!(transition.stage, WizardStage::CheckpointReady) {
                seen_checkpoint = true;
            }
            if transition.durable_writes_authorized {
                assert!(
                    seen_checkpoint,
                    "stage {} authorized writes before checkpoint",
                    transition.stage.as_str()
                );
            }
        }
    }

    #[test]
    fn validation_flags_apply_before_checkpoint() {
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
            validate_migration_wizard_page(&page).expect_err("must flag apply before checkpoint");
        assert!(errors.iter().any(|err| matches!(
            err,
            MigrationWizardValidationError::ApplyAuthorizedBeforeCheckpoint { .. }
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
    fn validation_flags_missing_post_apply_paths() {
        let mut page = seeded_migration_wizard_page();
        page.current_stage = WizardStage::Applied;
        page.compare_actions.clear();
        let errors =
            validate_migration_wizard_page(&page).expect_err("must flag missing post-apply paths");
        assert!(errors.iter().any(|err| matches!(
            err,
            MigrationWizardValidationError::PostApplyPathsMissing { .. }
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
    fn support_export_quotes_every_case_id() {
        let page = seeded_migration_wizard_page();
        let export = MigrationWizardSupportExport::from_page(
            "support-export:migration-wizard:001",
            page.clone(),
        );
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
            .contains(&page.rollback_checkpoint.checkpoint_ref));
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
    fn apply_gate_progression_matches_stage() {
        let review = seeded_review_record();
        let packet = materialize_import_diff_review_packet(&review);
        let pre_checkpoint = build_migration_wizard_page(&packet, WizardStage::PreviewReady);
        assert!(matches!(
            pre_checkpoint.apply_gate,
            WizardApplyGate::RequiresManualReview | WizardApplyGate::BlockedNoCheckpoint
        ));
        let checkpoint_ready = build_migration_wizard_page(&packet, WizardStage::CheckpointReady);
        assert!(matches!(
            checkpoint_ready.apply_gate,
            WizardApplyGate::RequiresManualReview | WizardApplyGate::AllowedCheckpointReady
        ));
        let applied = build_migration_wizard_page(&packet, WizardStage::Applied);
        assert!(matches!(
            applied.apply_gate,
            WizardApplyGate::AlreadyApplied
        ));
    }
}
