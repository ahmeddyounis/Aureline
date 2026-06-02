//! Canonical stable truth model for the migration-center diff, rollback, and
//! unsupported-gap taxonomy of an imported-user flow.
//!
//! ## Why one disclosure record per imported-user flow
//!
//! A switching user who imports a profile asks three questions before they trust
//! the result: *what exactly changed (the diff), can I undo it (the rollback),
//! and what did not come across (the unsupported-gap taxonomy)?* When the
//! migration center, the settings import history, the command palette, the
//! support export, Help/About, and docs each answer those questions with their
//! own bespoke status text, they drift — a row implies the import was lossless
//! while the taxonomy already shows unsupported gaps, a flow implies rollback is
//! available when no pre-apply checkpoint was verified for it, or an
//! unsupported gap is hidden until after apply.
//!
//! This module mints one governed [`MigrationFlowDisclosureRecord`] per imported
//! source ecosystem. The record binds, for a single canonical migration
//! identity:
//!
//! - **The diff** — a before/after review that is shown *before* apply, with
//!   every row carrying both sides and citing one rollback checkpoint.
//! - **The rollback** — a checkpoint minted before apply that protects every
//!   touched domain, with undo and compare routes when (and only when) the
//!   evidence is live for *this* flow.
//! - **The unsupported-gap taxonomy** — the canonical Exact / Translated /
//!   Partial / Shimmed / Unsupported counts, and the union of Unsupported and
//!   Shimmed gaps made visible before apply rather than discovered as missing
//!   behaviour afterwards.
//!
//! The taxonomy, domains, and source-ecosystem vocabulary are **not** reinvented
//! here: they are the canonical [`crate::import::diff_review`] and
//! [`crate::migration_corpus`] types, so there is no parallel model.
//!
//! ## The honesty invariants
//!
//! The builder refuses to mint a record that would lie. Each is a [`BuildError`],
//! not a warning, so a dishonest projection fails the row instead of shipping:
//!
//! - **No claim the product cannot prove.** A claim ceiling may not assert the
//!   diff was reviewed before apply unless it was, rollback availability unless a
//!   pre-apply checkpoint is verified for *this* flow with undo and compare, the
//!   absence of unsupported gaps unless the taxonomy has none, or full-fidelity
//!   import unless no Partial/Shimmed/Unsupported rows exist.
//! - **Automatic narrowing below Stable.** A flow missing any pillar of evidence
//!   (diff reviewed before apply, a live verified rollback, gaps visible before
//!   apply, a complete taxonomy) is narrowed below Stable with a named reason
//!   rather than inheriting an adjacent green row.
//! - **Gaps are never hidden.** Every gap in the taxonomy is visible before
//!   apply, and the record keeps a Review-gaps recovery route.
//! - **Recovery before trust.** Every flow exposes Reopen-report and
//!   Export-support routes; a flow with a live rollback also exposes Undo and
//!   Compare; a flow with gaps also exposes Review-gaps.
//! - **One model across surfaces.** The migration center, settings import
//!   history, and command-palette projections share identity and recovery
//!   behaviour, and the reopen surfaces stay settings / help / support-export.
//! - **Same routes everywhere.** The same flow is reachable from the migration
//!   center, settings import history, command palette, and a menu command, each
//!   keyboard reachable and pointing at the same flow.
//! - **Accessible in every layout.** Tab order, row narration (which discloses
//!   the source ecosystem), action labels, and recovery affordances are present
//!   and reachable in normal, high-contrast, and zoomed layouts.
//! - **No detour behind account or managed services.** Every row stays available
//!   without an account and without managed services.
//!
//! The record is the canonical truth source for this lane (suggested-output stem
//! `finish-the-migration-center-diff-rollback-and-unsupported`); its boundary
//! schema is
//! `schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json`
//! and its contract narrative is
//! `docs/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`.

use serde::{Deserialize, Serialize};

use crate::import::diff_review::{ImportMappingClassification, ImportReviewDomain};
use crate::migration_corpus::IncumbentEcosystem;

/// Stable record-kind tag carried in serialized disclosure records.
pub const MIGRATION_FLOW_DISCLOSURE_RECORD_KIND: &str = "migration_flow_disclosure_record";

/// Schema version for the [`MigrationFlowDisclosureRecord`] payload shape.
pub const MIGRATION_FLOW_DISCLOSURE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const MIGRATION_FLOW_DISCLOSURE_SHARED_CONTRACT_REF: &str =
    "shell:migration_flow_disclosure_stable:v1";

/// Reviewer-facing notice rendered on every disclosure surface.
pub const MIGRATION_FLOW_DISCLOSURE_NOTICE: &str =
    "Migration disclosure truth: the migration center, settings import history, command palette, \
     support exports, Help/About, and docs show the same before/after diff (reviewed before \
     apply), the same rollback posture, and the same Exact/Translated/Partial/Shimmed/Unsupported \
     taxonomy with every unsupported gap visible before apply; no row claims the diff was \
     reviewed, rollback is available, there are no unsupported gaps, or the import was \
     full-fidelity unless the product can prove it; a flow missing any pillar of evidence is \
     narrowed below Stable with a named reason rather than inheriting an adjacent green row; the \
     same flow opens from every surface, keyboard-first; and every row stays available without an \
     account or managed services.";

/// Canonical durable-object URI scheme. Every minted ref must be one of these.
pub const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a canonical object ref.
const MAX_REF_CHARS: usize = 200;

/// Object-class segments that are generic landing destinations rather than a
/// specific durable object. A ref pointing at one is rejected so chrome cannot
/// wire an affordance to a dashboard home.
const GENERIC_LANDING_CLASSES: &[&str] = &[
    "home",
    "dashboard",
    "landing",
    "index",
    "overview",
    "start",
    "root",
];

/// Returns true when `reference` is a canonical durable-object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

/// Public claim class for the lane, reusing the stable lifecycle cutline.
///
/// `Stable` sits at or above the launch cutline; everything else is narrowed
/// below it. The builder *derives* this from the evidence, so a flow can never
/// publish a claim wider than its proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The diff/rollback/gap-taxonomy disclosure is replacement-grade.
    Stable,
    /// Narrowed to the beta promise.
    Beta,
    /// Narrowed to the preview/limited-availability promise.
    Preview,
    /// No public promise yet.
    NotClaimed,
}

impl StableClaimClass {
    /// Returns the stable string vocabulary for this claim class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::NotClaimed => "not_claimed",
        }
    }

    /// Returns `true` when the claim sits at or above the launch cutline.
    pub const fn at_or_above_cutline(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Closed reason a flow is narrowed below Stable. Required whenever the claim
/// class is below the cutline; forbidden when it is Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableNarrowingReason {
    /// The before/after diff was not proven to be shown before apply.
    DiffNotReviewedBeforeApply,
    /// No pre-apply rollback checkpoint is verified for this flow, or undo /
    /// compare routes are unavailable.
    RollbackEvidenceIncomplete,
    /// At least one unsupported gap is not visible before apply.
    UnsupportedGapsHiddenBeforeApply,
    /// The taxonomy is incomplete: a row is unclassified, or no classification
    /// is present.
    TaxonomyIncomplete,
}

impl StableNarrowingReason {
    /// Returns the stable string vocabulary for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffNotReviewedBeforeApply => "diff_not_reviewed_before_apply",
            Self::RollbackEvidenceIncomplete => "rollback_evidence_incomplete",
            Self::UnsupportedGapsHiddenBeforeApply => "unsupported_gaps_hidden_before_apply",
            Self::TaxonomyIncomplete => "taxonomy_incomplete",
        }
    }
}

/// Surface a flow can be reached from. The same flow must be reachable from all
/// four so the migration center and in-product import surfaces stay consistent
/// for keyboard-only and assistive-technology users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationRouteSurface {
    /// The migration center page.
    MigrationCenter,
    /// The settings import-history list.
    SettingsImportHistory,
    /// The command palette.
    CommandPalette,
    /// An application menu command.
    MenuCommand,
}

impl MigrationRouteSurface {
    /// Returns the stable string vocabulary for this route surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigrationCenter => "migration_center",
            Self::SettingsImportHistory => "settings_import_history",
            Self::CommandPalette => "command_palette",
            Self::MenuCommand => "menu_command",
        }
    }

    /// The four surfaces that must all be able to reach a flow.
    pub const REQUIRED: [Self; 4] = [
        Self::MigrationCenter,
        Self::SettingsImportHistory,
        Self::CommandPalette,
        Self::MenuCommand,
    ];
}

/// Layout mode an accessibility disclosure is checked under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutMode {
    /// Default desktop layout.
    Normal,
    /// High-contrast theme.
    HighContrast,
    /// Zoomed / enlarged layout.
    Zoomed,
}

impl LayoutMode {
    /// Returns the stable string vocabulary for this layout mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::HighContrast => "high_contrast",
            Self::Zoomed => "zoomed",
        }
    }

    /// The three layout modes every disclosure must hold in.
    pub const REQUIRED: [Self; 3] = [Self::Normal, Self::HighContrast, Self::Zoomed];
}

/// Role a recovery action plays, used for placement and confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionRole {
    /// Opens or reopens the canonical migration artifact.
    Primary,
    /// Repairs, restores, or compares the imported state.
    Recovery,
    /// Non-destructive review or export.
    Secondary,
}

impl RecoveryActionRole {
    /// Returns the stable string vocabulary for this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Recovery => "recovery",
            Self::Secondary => "secondary",
        }
    }
}

/// Closed recovery-action vocabulary exposed on a migration flow row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationRecoveryAction {
    /// Reopen the retained migration mapping report.
    ReopenMigrationReport,
    /// Compare the captured before and after state.
    CompareBeforeAfter,
    /// Undo the apply via the pre-apply rollback checkpoint.
    UndoViaRollback,
    /// Review the unsupported / bridge gaps surfaced before apply.
    ReviewUnsupportedGaps,
    /// Export a redacted support packet for the migration.
    ExportSupportPacket,
}

impl MigrationRecoveryAction {
    /// Returns the stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopenMigrationReport => "reopen_migration_report",
            Self::CompareBeforeAfter => "compare_before_after",
            Self::UndoViaRollback => "undo_via_rollback",
            Self::ReviewUnsupportedGaps => "review_unsupported_gaps",
            Self::ExportSupportPacket => "export_support_packet",
        }
    }

    /// Returns the reviewer-facing action label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::ReopenMigrationReport => "Reopen migration report",
            Self::CompareBeforeAfter => "Compare before and after",
            Self::UndoViaRollback => "Undo via rollback checkpoint",
            Self::ReviewUnsupportedGaps => "Review unsupported gaps",
            Self::ExportSupportPacket => "Export support packet",
        }
    }

    /// Returns the placement / confirmation role for this action.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::ReopenMigrationReport => RecoveryActionRole::Primary,
            Self::CompareBeforeAfter | Self::UndoViaRollback => RecoveryActionRole::Recovery,
            Self::ReviewUnsupportedGaps | Self::ExportSupportPacket => {
                RecoveryActionRole::Secondary
            }
        }
    }

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }
}

/// Returns the recovery actions a flow must expose, in rendered order, given
/// whether a live rollback exists and whether the taxonomy carries gaps.
pub fn required_recovery_actions(
    live_rollback: bool,
    has_gaps: bool,
) -> Vec<MigrationRecoveryAction> {
    let mut actions = vec![MigrationRecoveryAction::ReopenMigrationReport];
    if live_rollback {
        actions.push(MigrationRecoveryAction::CompareBeforeAfter);
        actions.push(MigrationRecoveryAction::UndoViaRollback);
    }
    if has_gaps {
        actions.push(MigrationRecoveryAction::ReviewUnsupportedGaps);
    }
    actions.push(MigrationRecoveryAction::ExportSupportPacket);
    actions
}

/// The before/after diff disclosure for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffDisclosure {
    /// Canonical import-diff-preview ref.
    pub diff_preview_ref: String,
    /// Whether the diff is shown before apply.
    pub reviewed_before_apply: bool,
    /// Number of before/after rows in the diff.
    pub row_count: u32,
    /// Whether every row carries both a before and an after side.
    pub every_row_has_before_after: bool,
    /// Whether every row cites the one shared rollback checkpoint.
    pub every_row_uses_one_checkpoint: bool,
}

impl DiffDisclosure {
    /// Returns `true` when the diff is a reviewable before/after surface.
    pub fn is_reviewable_before_apply(&self) -> bool {
        self.reviewed_before_apply
            && self.row_count > 0
            && self.every_row_has_before_after
            && self.every_row_uses_one_checkpoint
    }
}

/// The rollback disclosure for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDisclosure {
    /// Canonical rollback-checkpoint ref.
    pub checkpoint_ref: String,
    /// Canonical migration-restore-record ref.
    pub restore_record_ref: String,
    /// Whether the checkpoint was minted before apply.
    pub created_before_apply: bool,
    /// Whether the checkpoint protects every domain the apply may touch.
    pub protects_every_domain: bool,
    /// Whether a live pre-apply checkpoint is verified for *this* flow, rather
    /// than referenced from an adjacent flow's apply session.
    pub verified_for_this_flow: bool,
    /// Whether an undo route restores from the checkpoint.
    pub undo_available: bool,
    /// Canonical undo-action ref, present iff [`Self::undo_available`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_action_ref: Option<String>,
    /// Whether a compare route shows before vs after.
    pub compare_available: bool,
    /// Canonical compare-action ref, present iff [`Self::compare_available`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_action_ref: Option<String>,
}

impl RollbackDisclosure {
    /// Returns `true` when rollback is provably available for this flow.
    pub fn is_live_for_flow(&self) -> bool {
        self.created_before_apply
            && self.protects_every_domain
            && self.verified_for_this_flow
            && self.undo_available
            && self.compare_available
    }
}

/// One Unsupported or Shimmed gap surfaced before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedGapDisclosure {
    /// Stable gap id.
    pub gap_id: String,
    /// Import domain the gap lives in.
    pub domain: ImportReviewDomain,
    /// Classification of the gap (`Unsupported` or `Shimmed`).
    pub classification: ImportMappingClassification,
    /// Redaction-aware source object label.
    pub source_label: String,
    /// Reviewer-facing description of the gap.
    pub gap_summary: String,
    /// Whether the gap is visible during preview, before apply.
    pub visible_before_apply: bool,
    /// Whether the gap remains visible in the retained report.
    pub retained_after_apply: bool,
    /// Docs/help refs that explain the gap (repo-relative source paths).
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that carry the gap into the export.
    pub support_export_refs: Vec<String>,
}

/// The Exact / Translated / Partial / Shimmed / Unsupported taxonomy for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GapTaxonomy {
    /// Number of `Exact` rows.
    pub exact: u32,
    /// Number of `Translated` rows.
    pub translated: u32,
    /// Number of `Partial` rows.
    pub partial: u32,
    /// Number of `Shimmed` rows.
    pub shimmed: u32,
    /// Number of `Unsupported` rows.
    pub unsupported: u32,
    /// Distinct classifications present, in canonical order.
    pub classifications_present: Vec<ImportMappingClassification>,
    /// Whether every Unsupported / Shimmed gap is visible before apply.
    pub unsupported_gaps_visible_before_apply: bool,
    /// The Unsupported / Shimmed gaps, sorted by gap id.
    pub gaps: Vec<UnsupportedGapDisclosure>,
}

impl GapTaxonomy {
    /// Returns the total number of classified rows.
    pub const fn total(&self) -> u32 {
        self.exact + self.translated + self.partial + self.shimmed + self.unsupported
    }

    /// Returns `true` when the import would be full-fidelity (no Partial,
    /// Shimmed, or Unsupported rows).
    pub const fn is_full_fidelity(&self) -> bool {
        self.partial == 0 && self.shimmed == 0 && self.unsupported == 0
    }

    /// Returns `true` when there are no Unsupported or Shimmed gaps.
    pub fn has_no_gaps(&self) -> bool {
        self.gaps.is_empty()
    }

    /// Returns `true` when the taxonomy is complete: at least one classification
    /// is present and every gap is visible before apply.
    pub fn is_complete(&self) -> bool {
        !self.classifications_present.is_empty()
            && self.total() > 0
            && self.unsupported_gaps_visible_before_apply
            && self.gaps.iter().all(|gap| gap.visible_before_apply)
    }
}

/// The public claim ceiling: what a flow row is allowed to assert. Each field
/// must be provable from the flow's real evidence; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MigrationClaimCeiling {
    /// Whether the row may claim the diff was reviewed before apply.
    pub asserts_diff_reviewed_before_apply: bool,
    /// Whether the row may claim rollback is available for this flow.
    pub asserts_rollback_available: bool,
    /// Whether the row may claim there are no unsupported gaps.
    pub asserts_no_unsupported_gaps: bool,
    /// Whether the row may claim the import was full-fidelity.
    pub asserts_full_fidelity_import: bool,
}

/// The derived stable-claim verdict for a flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableQualification {
    /// The derived claim class (Stable when fully qualified, else narrowed).
    pub claim_class: StableClaimClass,
    /// Whether the flow qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// The reasons the flow is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<StableNarrowingReason>,
}

/// One recovery route exposed on a flow row before the user commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRouteRecord {
    /// Stable action id from the canonical recovery vocabulary.
    pub action_id: String,
    /// Compact label rendered in rows and narrated by assistive tech.
    pub action_label: String,
    /// Placement / confirmation role.
    pub action_role: RecoveryActionRole,
    /// Whether the action is keyboard reachable.
    pub keyboard_reachable: bool,
}

/// One route to the same flow from one entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryRouteRecord {
    /// Surface that exposes the route.
    pub surface: MigrationRouteSurface,
    /// Canonical route ref pointing at the flow on this surface.
    pub route_ref: String,
    /// Whether the route is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether the route activates the same canonical flow identity.
    pub activates_same_flow: bool,
}

/// Accessibility disclosure for one layout mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutModeDisclosure {
    /// Layout mode this disclosure was checked under.
    pub mode: LayoutMode,
    /// Whether the row narration is available in this mode.
    pub row_narration_available: bool,
    /// Whether the recovery affordances stay reachable in this mode.
    pub recovery_affordances_reachable: bool,
}

/// Accessibility disclosure for one flow row across the required layout modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityDisclosure {
    /// Position of the row in the surface tab order.
    pub focus_order_index: u32,
    /// Number of keyboard tab stops the row and its actions expose.
    pub tab_stop_count: u32,
    /// Row narration read by assistive tech; discloses the source ecosystem.
    pub row_narration: String,
    /// Action labels in rendered order, narrated by assistive technology.
    pub action_labels: Vec<String>,
    /// Per-layout-mode disclosures for normal, high-contrast, and zoomed.
    pub layout_modes: Vec<LayoutModeDisclosure>,
}

/// Cross-surface parity between the migration center and settings projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParity {
    /// Migration center row id for this flow.
    pub migration_center_row_id: String,
    /// Settings import-history row id for this flow.
    pub settings_import_history_row_id: String,
    /// Command-palette command id that opens this flow.
    pub command_palette_command_id: String,
    /// Recovery action ids shared by both surfaces.
    pub recovery_action_ids: Vec<String>,
    /// Reopen surfaces (settings / help / support_export) the report retains.
    pub reopen_surfaces: Vec<String>,
    /// Whether the projections agree on identity and recovery behaviour.
    pub parity_holds: bool,
}

/// Upstream ids the record is a genuine projection of, kept for support
/// traceability. These are upstream source refs, not canonical durable objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamRefs {
    /// Wizard session id the diff/rollback evidence came from.
    pub wizard_session_ref: String,
    /// Wizard mapping report id retained after first run.
    pub wizard_mapping_report_ref: String,
    /// Raw upstream rollback-checkpoint ref.
    pub rollback_checkpoint_ref: String,
    /// Raw upstream import-diff-preview ref.
    pub import_diff_preview_ref: String,
    /// Migration corpus scoreboard id the taxonomy came from.
    pub corpus_scoreboard_ref: String,
    /// Source-ecosystem corpus section ref.
    pub corpus_section_ref: String,
}

/// Validated input used to mint a [`MigrationFlowDisclosureRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationFlowDisclosureInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Canonical migration-session ref.
    pub migration_session_ref: String,
    /// Source ecosystem this flow imported from.
    pub source_ecosystem: IncumbentEcosystem,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The before/after diff disclosure.
    pub diff: DiffDisclosure,
    /// The rollback disclosure.
    pub rollback: RollbackDisclosure,
    /// The Exact/Translated/Partial/Shimmed/Unsupported taxonomy.
    pub taxonomy: GapTaxonomy,
    /// Public claim ceiling for this flow.
    pub claim_ceiling: MigrationClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same flow.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the flow stays available without an account.
    pub available_without_account: bool,
    /// Whether the flow stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: UpstreamRefs,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed migration-flow disclosure record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationFlowDisclosureRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Canonical migration-session ref.
    pub migration_session_ref: String,
    /// Source ecosystem this flow imported from.
    pub source_ecosystem: IncumbentEcosystem,
    /// Compact source-ecosystem label (the vocabulary docs / Help/About ingest).
    pub source_ecosystem_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The before/after diff disclosure.
    pub diff: DiffDisclosure,
    /// The rollback disclosure.
    pub rollback: RollbackDisclosure,
    /// The Exact/Translated/Partial/Shimmed/Unsupported taxonomy.
    pub taxonomy: GapTaxonomy,
    /// Public claim ceiling.
    pub claim_ceiling: MigrationClaimCeiling,
    /// The derived stable-claim verdict (Stable, or narrowed with reasons).
    pub stable_qualification: StableQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Cross-surface parity block.
    pub surfaces: SurfaceParity,
    /// Per-surface routes to the same flow.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the flow stays available without an account.
    pub available_without_account: bool,
    /// Whether the flow stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or gapped to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: UpstreamRefs,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`MigrationFlowDisclosureRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// The claim ceiling asserted the diff was reviewed when it was not.
    OverclaimsDiffReviewed,
    /// The claim ceiling asserted rollback availability it cannot prove.
    OverclaimsRollbackAvailable,
    /// The claim ceiling asserted there were no unsupported gaps when there are.
    OverclaimsNoUnsupportedGaps,
    /// The claim ceiling asserted a full-fidelity import it cannot prove.
    OverclaimsFullFidelity,
    /// A taxonomy gap count did not match the gap rows present.
    TaxonomyGapCountMismatch,
    /// An undo / compare ref was present without availability, or vice versa.
    RollbackRefAvailabilityMismatch { field: &'static str },
    /// A gap was not visible before apply.
    GapHiddenBeforeApply { gap_id: String },
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: MigrationRecoveryAction },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// The two surface projections disagreed on identity or recovery behaviour.
    SurfaceParityBroken,
    /// A required reopen surface was missing.
    ReopenSurfaceMissing { surface: &'static str },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: MigrationRouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: MigrationRouteSurface },
    /// An entry route did not activate the same canonical flow.
    RouteTargetsDifferentFlow { surface: MigrationRouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: MigrationRouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// The row narration did not disclose the source ecosystem.
    NarrationOmitsEcosystem,
    /// A flow was hidden when no account was present.
    HiddenWithoutAccount,
    /// A flow was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::OverclaimsDiffReviewed => write!(
                f,
                "claim ceiling may not assert the diff was reviewed before apply unless it was"
            ),
            Self::OverclaimsRollbackAvailable => write!(
                f,
                "claim ceiling may not assert rollback availability without a verified pre-apply checkpoint, undo, and compare"
            ),
            Self::OverclaimsNoUnsupportedGaps => write!(
                f,
                "claim ceiling may not assert there are no unsupported gaps when the taxonomy has them"
            ),
            Self::OverclaimsFullFidelity => write!(
                f,
                "claim ceiling may not assert a full-fidelity import with Partial/Shimmed/Unsupported rows"
            ),
            Self::TaxonomyGapCountMismatch => write!(
                f,
                "taxonomy gap rows must match the Unsupported and Shimmed counts"
            ),
            Self::RollbackRefAvailabilityMismatch { field } => write!(
                f,
                "rollback `{field}` must be present iff the matching route is available"
            ),
            Self::GapHiddenBeforeApply { gap_id } => {
                write!(f, "gap `{gap_id}` must be visible before apply")
            }
            Self::MissingRecoveryRoute { action } => write!(
                f,
                "flow must expose recovery route `{}`",
                action.as_str()
            ),
            Self::RecoveryRouteNotKeyboardReachable { action_id } => write!(
                f,
                "recovery route `{action_id}` must be keyboard reachable"
            ),
            Self::SurfaceParityBroken => write!(
                f,
                "migration center and settings projections must share identity and recovery behaviour"
            ),
            Self::ReopenSurfaceMissing { surface } => {
                write!(f, "reopen surface `{surface}` is missing")
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentFlow { surface } => write!(
                f,
                "entry route surface `{}` must activate the same flow",
                surface.as_str()
            ),
            Self::DuplicateRouteSurface { surface } => {
                write!(f, "entry route surface `{}` is duplicated", surface.as_str())
            }
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(f, "accessibility layout mode `{}` is missing", mode.as_str())
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::NarrationOmitsEcosystem => {
                write!(f, "row narration must disclose the source ecosystem")
            }
            Self::HiddenWithoutAccount => {
                write!(f, "a migration flow row must stay available without an account")
            }
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a migration flow row must stay available without managed services"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl MigrationFlowDisclosureRecord {
    /// Builds a governed disclosure record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about the diff, the rollback, the unsupported-gap taxonomy, recovery,
    /// cross-surface parity, route reachability, or accessibility. The stable
    /// claim class is *derived* from the evidence, so a flow can never publish a
    /// claim wider than its proof.
    pub fn build(input: MigrationFlowDisclosureInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        require_ref("migration_session_ref", &input.migration_session_ref)?;
        require_ref("diff.diff_preview_ref", &input.diff.diff_preview_ref)?;
        require_ref("rollback.checkpoint_ref", &input.rollback.checkpoint_ref)?;
        require_ref(
            "rollback.restore_record_ref",
            &input.rollback.restore_record_ref,
        )?;
        require_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_ref("narrative_refs", narrative)?;
        }

        // --- rollback ref / availability consistency -------------------------
        match (
            &input.rollback.undo_available,
            &input.rollback.undo_action_ref,
        ) {
            (true, Some(reference)) => require_ref("rollback.undo_action_ref", reference)?,
            (false, None) => {}
            _ => {
                return Err(BuildError::RollbackRefAvailabilityMismatch {
                    field: "undo_action_ref",
                })
            }
        }
        match (
            &input.rollback.compare_available,
            &input.rollback.compare_action_ref,
        ) {
            (true, Some(reference)) => require_ref("rollback.compare_action_ref", reference)?,
            (false, None) => {}
            _ => {
                return Err(BuildError::RollbackRefAvailabilityMismatch {
                    field: "compare_action_ref",
                })
            }
        }

        // --- taxonomy integrity ----------------------------------------------
        let gap_unsupported = input
            .taxonomy
            .gaps
            .iter()
            .filter(|gap| gap.classification == ImportMappingClassification::Unsupported)
            .count() as u32;
        let gap_shimmed = input
            .taxonomy
            .gaps
            .iter()
            .filter(|gap| gap.classification == ImportMappingClassification::Shimmed)
            .count() as u32;
        if gap_unsupported != input.taxonomy.unsupported || gap_shimmed != input.taxonomy.shimmed {
            return Err(BuildError::TaxonomyGapCountMismatch);
        }
        for gap in &input.taxonomy.gaps {
            if !gap.visible_before_apply {
                return Err(BuildError::GapHiddenBeforeApply {
                    gap_id: gap.gap_id.clone(),
                });
            }
        }

        // --- claim ceiling: never claim what the product cannot prove ---------
        let diff_reviewed = input.diff.is_reviewable_before_apply();
        let rollback_live = input.rollback.is_live_for_flow();
        let has_gaps = !input.taxonomy.has_no_gaps();
        let taxonomy_complete = input.taxonomy.is_complete();

        if input.claim_ceiling.asserts_diff_reviewed_before_apply && !diff_reviewed {
            return Err(BuildError::OverclaimsDiffReviewed);
        }
        if input.claim_ceiling.asserts_rollback_available && !rollback_live {
            return Err(BuildError::OverclaimsRollbackAvailable);
        }
        if input.claim_ceiling.asserts_no_unsupported_gaps && has_gaps {
            return Err(BuildError::OverclaimsNoUnsupportedGaps);
        }
        if input.claim_ceiling.asserts_full_fidelity_import && !input.taxonomy.is_full_fidelity() {
            return Err(BuildError::OverclaimsFullFidelity);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in required_recovery_actions(rollback_live, has_gaps) {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- cross-surface parity --------------------------------------------
        if !input.surfaces.parity_holds {
            return Err(BuildError::SurfaceParityBroken);
        }
        let parity_ids: Vec<&str> = input
            .surfaces
            .recovery_action_ids
            .iter()
            .map(String::as_str)
            .collect();
        if parity_ids != route_ids {
            return Err(BuildError::SurfaceParityBroken);
        }
        for required in ["settings", "help", "support_export"] {
            if !input
                .surfaces
                .reopen_surfaces
                .iter()
                .any(|surface| surface == required)
            {
                return Err(BuildError::ReopenSurfaceMissing { surface: required });
            }
        }

        // --- route parity across surfaces ------------------------------------
        let mut seen_surfaces = Vec::new();
        for route in &input.routes {
            if seen_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_surfaces.push(route.surface);
            require_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_flow {
                return Err(BuildError::RouteTargetsDifferentFlow {
                    surface: route.surface,
                });
            }
        }
        for required in MigrationRouteSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        let ecosystem_label = input.source_ecosystem.display_label().to_string();
        if !input.accessibility.row_narration.contains(&ecosystem_label) {
            return Err(BuildError::NarrationOmitsEcosystem);
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability: never bury a flow behind account or services ------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- derive the stable-claim verdict from the evidence ---------------
        let mut narrowing_reasons = Vec::new();
        if !diff_reviewed {
            narrowing_reasons.push(StableNarrowingReason::DiffNotReviewedBeforeApply);
        }
        if !rollback_live {
            narrowing_reasons.push(StableNarrowingReason::RollbackEvidenceIncomplete);
        }
        if !input.taxonomy.unsupported_gaps_visible_before_apply
            || !input
                .taxonomy
                .gaps
                .iter()
                .all(|gap| gap.visible_before_apply)
        {
            narrowing_reasons.push(StableNarrowingReason::UnsupportedGapsHiddenBeforeApply);
        }
        if !taxonomy_complete {
            narrowing_reasons.push(StableNarrowingReason::TaxonomyIncomplete);
        }
        let claim_class = if narrowing_reasons.is_empty() {
            StableClaimClass::Stable
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = StableQualification {
            claim_class,
            qualifies_stable: narrowing_reasons.is_empty(),
            narrowing_reasons,
        };

        let honesty_marker_present = !stable_qualification.qualifies_stable
            || has_gaps
            || !input.taxonomy.is_full_fidelity();

        Ok(Self {
            record_kind: MIGRATION_FLOW_DISCLOSURE_RECORD_KIND.to_string(),
            schema_version: MIGRATION_FLOW_DISCLOSURE_SCHEMA_VERSION,
            notice: MIGRATION_FLOW_DISCLOSURE_NOTICE.to_string(),
            shared_contract_ref: MIGRATION_FLOW_DISCLOSURE_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            migration_session_ref: input.migration_session_ref,
            source_ecosystem: input.source_ecosystem,
            source_ecosystem_label: ecosystem_label,
            title: input.title,
            summary: input.summary,
            diff: input.diff,
            rollback: input.rollback,
            taxonomy: input.taxonomy,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            surfaces: input.surfaces,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: input.upstream,
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("migration_flow_disclosure: {}", self.record_id),
            format!("migration_session_ref: {}", self.migration_session_ref),
            format!("as_of: {}", self.as_of),
            format!(
                "source_ecosystem: {} ({})",
                self.source_ecosystem.as_str(),
                self.source_ecosystem_label
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "diff: rows={} reviewed_before_apply={} before_after={} one_checkpoint={}",
                self.diff.row_count,
                self.diff.reviewed_before_apply,
                self.diff.every_row_has_before_after,
                self.diff.every_row_uses_one_checkpoint
            ),
            format!(
                "rollback: created_before_apply={} protects_every_domain={} verified_for_flow={} undo={} compare={}",
                self.rollback.created_before_apply,
                self.rollback.protects_every_domain,
                self.rollback.verified_for_this_flow,
                self.rollback.undo_available,
                self.rollback.compare_available
            ),
            format!(
                "taxonomy: exact={} translated={} partial={} shimmed={} unsupported={} gaps_visible_before_apply={}",
                self.taxonomy.exact,
                self.taxonomy.translated,
                self.taxonomy.partial,
                self.taxonomy.shimmed,
                self.taxonomy.unsupported,
                self.taxonomy.unsupported_gaps_visible_before_apply
            ),
            format!(
                "claim_ceiling: diff_reviewed={} rollback_available={} no_unsupported_gaps={} full_fidelity={}",
                self.claim_ceiling.asserts_diff_reviewed_before_apply,
                self.claim_ceiling.asserts_rollback_available,
                self.claim_ceiling.asserts_no_unsupported_gaps,
                self.claim_ceiling.asserts_full_fidelity_import
            ),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ];
        lines.push("gaps:".to_string());
        for gap in &self.taxonomy.gaps {
            lines.push(format!(
                "  - {} ({}) [{}] visible_before_apply={} -- {}",
                gap.gap_id,
                gap.domain.as_str(),
                gap.classification.as_str(),
                gap.visible_before_apply,
                gap.gap_summary
            ));
        }
        lines.push("recovery_routes:".to_string());
        for route in &self.recovery_routes {
            lines.push(format!(
                "  - {} ({}) role={} keyboard={}",
                route.action_id,
                route.action_label,
                route.action_role.as_str(),
                route.keyboard_reachable
            ));
        }
        lines.push(format!(
            "surfaces: migration_center={} settings={} command={} parity_holds={} reopen=[{}]",
            self.surfaces.migration_center_row_id,
            self.surfaces.settings_import_history_row_id,
            self.surfaces.command_palette_command_id,
            self.surfaces.parity_holds,
            self.surfaces.reopen_surfaces.join(", ")
        ));
        lines.push("routes:".to_string());
        for route in &self.routes {
            lines.push(format!(
                "  - {} -> {} keyboard={} same_flow={}",
                route.surface.as_str(),
                route.route_ref,
                route.keyboard_reachable,
                route.activates_same_flow
            ));
        }
        lines.push(format!(
            "accessibility: tab_order={} tab_stops={} narration={:?}",
            self.accessibility.focus_order_index,
            self.accessibility.tab_stop_count,
            self.accessibility.row_narration
        ));
        for mode in &self.accessibility.layout_modes {
            lines.push(format!(
                "  layout {} narration={} affordances_reachable={}",
                mode.mode.as_str(),
                mode.row_narration_available,
                mode.recovery_affordances_reachable
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "upstream: wizard_session={} mapping_report={} checkpoint={} diff_preview={} scoreboard={} section={}",
            self.upstream.wizard_session_ref,
            self.upstream.wizard_mapping_report_ref,
            self.upstream.rollback_checkpoint_ref,
            self.upstream.import_diff_preview_ref,
            self.upstream.corpus_scoreboard_ref,
            self.upstream.corpus_section_ref
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}
