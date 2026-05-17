//! Beta migration center and learnability surface.
//!
//! The migration center is the page-level projection that gives a new
//! or migrating user one canonical learnability surface. It links the
//! existing docs/help anchors, the wizard mapping report retained by
//! [`crate::migration_wizard`], the incumbent-flow known-limits lane
//! retained by [`crate::migration_corpus`], the glossary packs, and
//! the recovery routes owned by [`crate::recovery`] under one shared
//! contract ref so the live shell, the headless inspector, and the
//! support-export wrapper quote the same vocabulary.
//!
//! Three invariants ride on every entry-point row:
//!
//! 1. **No account or marketplace detour.** Every row must reach its
//!    target directly. Rows that require a hidden marketplace install
//!    or account sign-in to be useful are a contract bug the
//!    validator rejects.
//! 2. **Keyboard reachable.** Every row must either invoke a
//!    canonical [`aureline_commands`] command, sit on a focus path
//!    reachable from the migration center page, or quote a stable
//!    keyboard route ref. Mouse-only or hover-only entry points are
//!    rejected.
//! 3. **Beta-claim parity.** Every row carries the same
//!    [`LearnabilityClaim`] vocabulary (claim class, lifecycle class,
//!    freshness class, review window, evidence date) the rest of the
//!    beta program uses, so the migration center never paints a
//!    fresher or stalker claim than its source surface.
//!
//! The seeded page seeds zero defects; the validator and the headless
//! inspector are what surface a regression when a row drops a
//! required anchor, weakens the keyboard-reach posture, masquerades
//! lifecycle state, or drifts from the support row.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::docs_browser::{seeded_truth_wiring_report, SurfaceTruthBinding, TruthSurfaceClass};
use crate::help_packs::onboarding_beta::{
    seeded_onboarding_help_pack_beta_manifest, OnboardingHelpPackBetaManifest,
    ONBOARDING_HELP_PACK_BETA_FIXTURE_REF, ONBOARDING_HELP_PACK_BETA_SUPPORT_EXPORT_FIXTURE_REF,
};
use crate::migration_corpus::{
    seeded_migration_scoreboard, MIGRATION_CORPUS_SHARED_CONTRACT_REF, MIGRATION_SCOREBOARD_ID,
};
use crate::migration_wizard::{seeded_migration_wizard_page, MIGRATION_WIZARD_SHARED_CONTRACT_REF};

/// Beta schema version exported with every record.
pub const MIGRATION_CENTER_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every migration-center row.
pub const MIGRATION_CENTER_BETA_SHARED_CONTRACT_REF: &str = "shell:migration_center_beta:v1";

/// Stable record kind for [`MigrationCenterPage`] payloads.
pub const MIGRATION_CENTER_BETA_PAGE_RECORD_KIND: &str = "shell_migration_center_beta_page_record";

/// Stable record kind for [`MigrationCenterSection`] payloads.
pub const MIGRATION_CENTER_BETA_SECTION_RECORD_KIND: &str =
    "shell_migration_center_beta_section_record";

/// Stable record kind for [`MigrationCenterEntryPoint`] payloads.
pub const MIGRATION_CENTER_BETA_ENTRY_RECORD_KIND: &str =
    "shell_migration_center_beta_entry_record";

/// Stable record kind for [`MigrationCenterDefect`] payloads.
pub const MIGRATION_CENTER_BETA_DEFECT_RECORD_KIND: &str =
    "shell_migration_center_beta_defect_record";

/// Stable record kind for [`MigrationCenterSupportRow`] payloads.
pub const MIGRATION_CENTER_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "shell_migration_center_beta_support_row_record";

/// Stable record kind for [`MigrationCenterSupportExport`] payloads.
pub const MIGRATION_CENTER_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_migration_center_beta_support_export_record";

/// Stable migration-center page id.
pub const MIGRATION_CENTER_PAGE_ID: &str = "shell:migration_center_beta:page:v1";

/// Generation timestamp baked into every seeded record.
const GENERATED_AT: &str = "2026-05-15T00:00:00Z";

/// Required review window for beta-grade learnability claims, in days.
pub const BETA_REVIEW_WINDOW_DAYS: u32 = 90;

/// Closed section vocabulary the migration center renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionKind {
    /// Canonical docs/help anchors users open from the page.
    DocsHelpAnchors,
    /// Glossary packs that name aureline-specific concepts.
    GlossaryPacks,
    /// Retained mapping reports from the migration wizard.
    MigrationReports,
    /// Incumbent-flow known-limits lane from the migration corpus.
    KnownLimits,
    /// Recovery routes owned by the recovery surface.
    RecoveryRoutes,
    /// Explicit exits from first-run confusion.
    FirstRunConfusionExits,
}

impl SectionKind {
    /// Stable schema token recorded on the section.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelpAnchors => "docs_help_anchors",
            Self::GlossaryPacks => "glossary_packs",
            Self::MigrationReports => "migration_reports",
            Self::KnownLimits => "known_limits",
            Self::RecoveryRoutes => "recovery_routes",
            Self::FirstRunConfusionExits => "first_run_confusion_exits",
        }
    }

    /// Reviewer-facing label for the section.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::DocsHelpAnchors => "Docs and help anchors",
            Self::GlossaryPacks => "Glossary",
            Self::MigrationReports => "Migration reports",
            Self::KnownLimits => "Known limits",
            Self::RecoveryRoutes => "Recovery routes",
            Self::FirstRunConfusionExits => "First-run confusion exits",
        }
    }

    /// Required sections in canonical order.
    pub fn required_sections() -> [Self; 6] {
        [
            Self::DocsHelpAnchors,
            Self::GlossaryPacks,
            Self::MigrationReports,
            Self::KnownLimits,
            Self::RecoveryRoutes,
            Self::FirstRunConfusionExits,
        ]
    }
}

/// Closed entry-point vocabulary the migration center renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    /// Opens a docs/help anchor.
    DocsHelpAnchor,
    /// Opens a glossary pack entry.
    GlossaryAnchor,
    /// Reopens a retained migration report.
    MigrationReportReopen,
    /// Opens a known-limits scoreboard row.
    KnownLimitsLane,
    /// Triggers a recovery route action.
    RecoveryAction,
    /// Opens a first-run confusion exit (e.g. restart safe, reset
    /// preferences, restore from checkpoint, open command palette).
    FirstRunConfusionExit,
    /// Quotes the canonical keymap reference.
    KeymapReference,
}

impl EntryKind {
    /// Stable schema token recorded on the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelpAnchor => "docs_help_anchor",
            Self::GlossaryAnchor => "glossary_anchor",
            Self::MigrationReportReopen => "migration_report_reopen",
            Self::KnownLimitsLane => "known_limits_lane",
            Self::RecoveryAction => "recovery_action",
            Self::FirstRunConfusionExit => "first_run_confusion_exit",
            Self::KeymapReference => "keymap_reference",
        }
    }
}

/// Closed keyboard-reach posture every entry must declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardReachClass {
    /// Entry invokes a canonical command id from the command registry.
    KeyboardFirstCommandInvocation,
    /// Entry sits on a stable focus path reachable from the page.
    KeyboardReachableFocusPath,
    /// Entry quotes a documented keymap chord that resolves to a
    /// canonical command.
    KeyboardChordReference,
}

impl KeyboardReachClass {
    /// Stable schema token recorded on the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFirstCommandInvocation => "keyboard_first_command_invocation",
            Self::KeyboardReachableFocusPath => "keyboard_reachable_focus_path",
            Self::KeyboardChordReference => "keyboard_chord_reference",
        }
    }
}

/// Closed lifecycle vocabulary aligned with the rest of the beta program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleClass {
    /// Surface ships under the stable promise.
    Stable,
    /// Surface ships under the beta promise.
    Beta,
    /// Surface ships under the alpha promise.
    Alpha,
    /// Surface ships under the preview/limited-availability promise.
    Preview,
    /// Surface ships under labs / experiment posture.
    LabsExperiment,
}

impl LifecycleClass {
    /// Stable schema token recorded on the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Alpha => "alpha",
            Self::Preview => "preview",
            Self::LabsExperiment => "labs_experiment",
        }
    }
}

/// Closed freshness vocabulary the migration center inherits from the
/// release-truth surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Evidence is current and inside the review window.
    Current,
    /// Evidence is current but the review window is about to expire.
    ReviewDueSoon,
    /// Evidence review window has elapsed.
    ReviewOverdue,
    /// Evidence has not yet been verified for this release.
    UnverifiedButCurrent,
    /// Source is degraded but the current claim is still honest.
    DegradedButCurrent,
}

impl FreshnessClass {
    /// Stable schema token recorded on the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::ReviewDueSoon => "review_due_soon",
            Self::ReviewOverdue => "review_overdue",
            Self::UnverifiedButCurrent => "unverified_but_current",
            Self::DegradedButCurrent => "degraded_but_current",
        }
    }

    /// Returns `true` when this freshness state must not appear on
    /// claimed beta rows without an explicit downgrade.
    pub const fn is_blocking_for_beta(self) -> bool {
        matches!(self, Self::ReviewOverdue)
    }
}

/// Closed claim vocabulary every learnability surface inherits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimClass {
    /// Row is claimed at the stable promise level.
    Stable,
    /// Row is claimed at the beta promise level.
    Beta,
    /// Row is claimed at the alpha promise level.
    Alpha,
    /// Row is claimed only as a labs/experiment preview.
    LabsExperiment,
    /// Row carries no public promise yet.
    NotClaimed,
}

impl ClaimClass {
    /// Stable schema token recorded on the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable_claimed",
            Self::Beta => "beta_claimed",
            Self::Alpha => "alpha_claimed",
            Self::LabsExperiment => "labs_experiment_claimed",
            Self::NotClaimed => "not_claimed",
        }
    }
}

/// Closed recovery-route vocabulary owned by the recovery surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryRouteClass {
    /// Restart the shell into safe mode.
    SafeMode,
    /// Open the factory-reset review sheet.
    FactoryResetReview,
    /// Open the corruption rescue sheet.
    CorruptionRescue,
    /// Open the restore chooser.
    RestoreChooser,
    /// Open a crash-loop containment card.
    CrashLoopContainment,
    /// Roll back the last migration apply via the wizard checkpoint.
    MigrationRollback,
    /// Export support evidence before any state change.
    SupportExport,
}

impl RecoveryRouteClass {
    /// Stable schema token recorded on the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::FactoryResetReview => "factory_reset_review",
            Self::CorruptionRescue => "corruption_rescue",
            Self::RestoreChooser => "restore_chooser",
            Self::CrashLoopContainment => "crash_loop_containment",
            Self::MigrationRollback => "migration_rollback",
            Self::SupportExport => "support_export",
        }
    }
}

/// Beta learnability claim every page and row inherits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityClaim {
    /// Claim class for this row.
    pub claim_class: ClaimClass,
    /// Stable schema token for [`Self::claim_class`].
    pub claim_class_token: String,
    /// Lifecycle class for this row.
    pub lifecycle_class: LifecycleClass,
    /// Stable schema token for [`Self::lifecycle_class`].
    pub lifecycle_class_token: String,
    /// Freshness class for this row.
    pub freshness_class: FreshnessClass,
    /// Stable schema token for [`Self::freshness_class`].
    pub freshness_class_token: String,
    /// Review window in days the claim is current for.
    pub review_window_days: u32,
    /// Evidence date the claim was last validated.
    pub evidence_date: String,
    /// Generation as-of timestamp.
    pub as_of: String,
}

impl LearnabilityClaim {
    /// Build a beta claim from the seeded posture.
    pub fn beta_current() -> Self {
        Self::new(
            ClaimClass::Beta,
            LifecycleClass::Beta,
            FreshnessClass::Current,
        )
    }

    fn new(claim_class: ClaimClass, lifecycle: LifecycleClass, freshness: FreshnessClass) -> Self {
        Self {
            claim_class,
            claim_class_token: claim_class.as_str().to_owned(),
            lifecycle_class: lifecycle,
            lifecycle_class_token: lifecycle.as_str().to_owned(),
            freshness_class: freshness,
            freshness_class_token: freshness.as_str().to_owned(),
            review_window_days: BETA_REVIEW_WINDOW_DAYS,
            evidence_date: GENERATED_AT.to_owned(),
            as_of: GENERATED_AT.to_owned(),
        }
    }
}

/// One section in the migration center page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCenterSection {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub section_id: String,
    pub section_kind: SectionKind,
    pub section_kind_token: String,
    pub title_label: String,
    pub description_label: String,
    /// Stable entry ids in render order.
    pub entry_ids: Vec<String>,
}

/// One entry-point row in the migration center page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCenterEntryPoint {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub entry_id: String,
    pub section_id: String,
    pub section_kind: SectionKind,
    pub section_kind_token: String,
    pub entry_kind: EntryKind,
    pub entry_kind_token: String,
    pub title_label: String,
    pub description_label: String,

    /// Canonical command id the entry invokes (when keyboard-first).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// Canonical docs/help anchor ref the entry opens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_help_anchor_ref: Option<String>,
    /// Glossary pack item ref the entry opens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossary_pack_ref: Option<String>,
    /// Migration report ref the entry reopens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_report_ref: Option<String>,
    /// Known-limits scoreboard row ref the entry opens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub known_limit_ref: Option<String>,
    /// Recovery route class for the entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_route_class: Option<RecoveryRouteClass>,
    /// Stable schema token for [`Self::recovery_route_class`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_route_class_token: Option<String>,
    /// Documented keymap chord ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_chord_ref: Option<String>,
    /// Help-pack id that backs this migration-center entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_pack_id: Option<String>,
    /// Help-pack item ref that backs this migration-center entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_pack_item_ref: Option<String>,
    /// Active help-pack version ref for support/export reconstruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_pack_version_ref: Option<String>,
    /// Fallback token inherited from the help-pack item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_pack_fallback_state_token: Option<String>,

    /// Keyboard-reach posture for the entry.
    pub keyboard_reach: KeyboardReachClass,
    pub keyboard_reach_token: String,

    /// MUST be false for every claimed beta row.
    pub requires_account_detour: bool,
    /// MUST be false for every claimed beta row.
    pub requires_marketplace_detour: bool,

    /// Per-row learnability claim. MUST match the page's claim
    /// vocabulary unless the row is explicitly downgraded.
    pub learnability_claim: LearnabilityClaim,

    /// Support-export row id that mirrors this entry.
    pub support_row_id: String,
}

/// Support-export row that mirrors one [`MigrationCenterEntryPoint`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCenterSupportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub support_row_id: String,
    pub entry_id: String,
    pub section_kind_token: String,
    pub entry_kind_token: String,
    pub keyboard_reach_token: String,
    pub claim_class_token: String,
    pub lifecycle_class_token: String,
    pub freshness_class_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_help_anchor_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_report_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub known_limit_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_route_class_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_pack_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_pack_item_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_pack_version_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_pack_fallback_state_token: Option<String>,
    pub raw_body_exported: bool,
}

impl MigrationCenterSupportRow {
    fn from_entry(entry: &MigrationCenterEntryPoint) -> Self {
        Self {
            record_kind: MIGRATION_CENTER_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: MIGRATION_CENTER_BETA_SCHEMA_VERSION,
            shared_contract_ref: MIGRATION_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
            support_row_id: entry.support_row_id.clone(),
            entry_id: entry.entry_id.clone(),
            section_kind_token: entry.section_kind_token.clone(),
            entry_kind_token: entry.entry_kind_token.clone(),
            keyboard_reach_token: entry.keyboard_reach_token.clone(),
            claim_class_token: entry.learnability_claim.claim_class_token.clone(),
            lifecycle_class_token: entry.learnability_claim.lifecycle_class_token.clone(),
            freshness_class_token: entry.learnability_claim.freshness_class_token.clone(),
            command_id: entry.command_id.clone(),
            docs_help_anchor_ref: entry.docs_help_anchor_ref.clone(),
            migration_report_ref: entry.migration_report_ref.clone(),
            known_limit_ref: entry.known_limit_ref.clone(),
            recovery_route_class_token: entry.recovery_route_class_token.clone(),
            help_pack_id: entry.help_pack_id.clone(),
            help_pack_item_ref: entry.help_pack_item_ref.clone(),
            help_pack_version_ref: entry.help_pack_version_ref.clone(),
            help_pack_fallback_state_token: entry.help_pack_fallback_state_token.clone(),
            raw_body_exported: false,
        }
    }
}

/// Closed defect vocabulary the audit emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationCenterDefectKind {
    /// Entry required hidden account sign-in to reach the target.
    EntryRequiresAccountDetour,
    /// Entry required a hidden marketplace install to reach the target.
    EntryRequiresMarketplaceDetour,
    /// Entry's keyboard-reach posture is missing or invalid.
    EntryKeyboardUnreachable,
    /// Entry that declared a command_id is missing it, or vice versa.
    EntryCommandIdMissing,
    /// Docs/help anchor entry is missing its anchor ref.
    DocsAnchorRefMissing,
    /// Glossary anchor entry is missing its glossary pack ref.
    GlossaryPackRefMissing,
    /// Migration report reopen entry is missing its report ref.
    MigrationReportRefMissing,
    /// Known-limits lane entry is missing its scoreboard row ref.
    KnownLimitRefMissing,
    /// Recovery action entry is missing its route class.
    RecoveryRouteClassMissing,
    /// Page is missing one of the required section kinds.
    SectionMissingRequiredKind,
    /// Section references an entry id that does not resolve.
    SectionUnknownEntry,
    /// Two entries share the same entry id.
    DuplicateEntryId,
    /// Two sections share the same section kind.
    DuplicateSectionKind,
    /// Row's freshness class is `review_overdue` while still claimed.
    FreshnessReviewOverdueOnClaimedRow,
    /// Row's claim class is `beta_claimed` but lifecycle/freshness
    /// drifted from the page baseline.
    ClaimLifecycleFreshnessDrift,
    /// Support row's tokens drifted from the live row.
    SupportRowVocabularyDrift,
    /// Support row's entry id does not resolve to a live entry.
    SupportRowUnknownEntry,
    /// Claimed row is missing the help-pack item ref that backs guidance.
    HelpPackItemRefMissing,
}

impl MigrationCenterDefectKind {
    /// Stable schema token recorded on the defect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntryRequiresAccountDetour => "entry_requires_account_detour",
            Self::EntryRequiresMarketplaceDetour => "entry_requires_marketplace_detour",
            Self::EntryKeyboardUnreachable => "entry_keyboard_unreachable",
            Self::EntryCommandIdMissing => "entry_command_id_missing",
            Self::DocsAnchorRefMissing => "docs_anchor_ref_missing",
            Self::GlossaryPackRefMissing => "glossary_pack_ref_missing",
            Self::MigrationReportRefMissing => "migration_report_ref_missing",
            Self::KnownLimitRefMissing => "known_limit_ref_missing",
            Self::RecoveryRouteClassMissing => "recovery_route_class_missing",
            Self::SectionMissingRequiredKind => "section_missing_required_kind",
            Self::SectionUnknownEntry => "section_unknown_entry",
            Self::DuplicateEntryId => "duplicate_entry_id",
            Self::DuplicateSectionKind => "duplicate_section_kind",
            Self::FreshnessReviewOverdueOnClaimedRow => "freshness_review_overdue_on_claimed_row",
            Self::ClaimLifecycleFreshnessDrift => "claim_lifecycle_freshness_drift",
            Self::SupportRowVocabularyDrift => "support_row_vocabulary_drift",
            Self::SupportRowUnknownEntry => "support_row_unknown_entry",
            Self::HelpPackItemRefMissing => "help_pack_item_ref_missing",
        }
    }
}

/// Typed defect emitted by the audit validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCenterDefect {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub defect_id: String,
    pub defect_kind: MigrationCenterDefectKind,
    pub defect_kind_token: String,
    pub row_id: String,
    pub field: String,
    pub note: String,
}

impl MigrationCenterDefect {
    fn new(
        defect_kind: MigrationCenterDefectKind,
        row_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let row_id = row_id.into();
        Self {
            record_kind: MIGRATION_CENTER_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: MIGRATION_CENTER_BETA_SCHEMA_VERSION,
            shared_contract_ref: MIGRATION_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "ux:defect:migration-center:{}:{}",
                defect_kind.as_str(),
                row_id
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            row_id,
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Reviewer-facing summary banner for the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCenterSummary {
    pub section_count: usize,
    pub entry_count: usize,
    pub keyboard_first_count: usize,
    pub recovery_route_count: usize,
    pub docs_help_count: usize,
    pub known_limit_count: usize,
    pub migration_report_reopen_count: usize,
    pub glossary_anchor_count: usize,
    pub first_run_exit_count: usize,
    pub defect_count: usize,
}

/// Beta migration-center page record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCenterPage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_id: String,
    pub generated_at: String,
    pub learnability_claim: LearnabilityClaim,
    /// Current release-truth binding that joins the migration center to the
    /// generated claim manifest and compatibility report.
    pub release_truth_binding: SurfaceTruthBinding,
    pub upstream_refs: MigrationCenterUpstreamRefs,
    pub sections: Vec<MigrationCenterSection>,
    pub entries: Vec<MigrationCenterEntryPoint>,
    pub support_rows: Vec<MigrationCenterSupportRow>,
    pub defects: Vec<MigrationCenterDefect>,
    pub summary: MigrationCenterSummary,
}

impl MigrationCenterPage {
    /// Returns `true` when every required section kind is present.
    pub fn covers_every_required_section(&self) -> bool {
        SectionKind::required_sections()
            .iter()
            .all(|required| self.sections.iter().any(|s| s.section_kind == *required))
    }

    /// Returns the entry with the given id.
    pub fn entry(&self, entry_id: &str) -> Option<&MigrationCenterEntryPoint> {
        self.entries.iter().find(|entry| entry.entry_id == entry_id)
    }
}

/// Upstream refs the migration center pivots on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCenterUpstreamRefs {
    pub wizard_session_ref: String,
    pub wizard_mapping_report_ref: String,
    pub wizard_rollback_checkpoint_ref: String,
    pub wizard_shared_contract_ref: String,
    pub corpus_scoreboard_ref: String,
    pub corpus_shared_contract_ref: String,
    pub help_pack_manifest_ref: String,
    pub help_pack_manifest_id: String,
    pub help_pack_version_ref: String,
    pub help_pack_support_export_ref: String,
}

/// Support-export wrapper for the migration-center page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCenterSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub support_export_id: String,
    pub generated_at: String,
    pub page: MigrationCenterPage,
    pub case_ids: Vec<String>,
    pub defect_kinds_present: Vec<String>,
    pub raw_private_material_excluded: bool,
}

impl MigrationCenterSupportExport {
    /// Builds the support-export wrapper for a page.
    pub fn from_page(
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: MigrationCenterPage,
    ) -> Self {
        let mut case_ids: Vec<String> = Vec::new();
        case_ids.push(page.page_id.clone());
        for section in &page.sections {
            case_ids.push(section.section_id.clone());
        }
        for entry in &page.entries {
            case_ids.push(entry.entry_id.clone());
            case_ids.push(entry.support_row_id.clone());
        }
        let mut defect_kinds_present: BTreeSet<String> = BTreeSet::new();
        for defect in &page.defects {
            defect_kinds_present.insert(defect.defect_kind_token.clone());
        }
        Self {
            record_kind: MIGRATION_CENTER_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MIGRATION_CENTER_BETA_SCHEMA_VERSION,
            shared_contract_ref: MIGRATION_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            page,
            case_ids,
            defect_kinds_present: defect_kinds_present.into_iter().collect(),
            raw_private_material_excluded: true,
        }
    }
}

/// Builds a typed defect list for `entries` and `support_rows`.
pub fn audit_migration_center_rows(
    sections: &[MigrationCenterSection],
    entries: &[MigrationCenterEntryPoint],
    support_rows: &[MigrationCenterSupportRow],
) -> Vec<MigrationCenterDefect> {
    let mut defects: Vec<MigrationCenterDefect> = Vec::new();

    let mut seen_entry_ids: BTreeSet<&str> = BTreeSet::new();
    for entry in entries {
        if !seen_entry_ids.insert(entry.entry_id.as_str()) {
            defects.push(MigrationCenterDefect::new(
                MigrationCenterDefectKind::DuplicateEntryId,
                &entry.entry_id,
                "entry_id",
                "two entries share the same entry_id",
            ));
        }

        if entry.requires_account_detour {
            defects.push(MigrationCenterDefect::new(
                MigrationCenterDefectKind::EntryRequiresAccountDetour,
                &entry.entry_id,
                "requires_account_detour",
                "claimed beta entry must not require hidden account sign-in",
            ));
        }
        if entry.requires_marketplace_detour {
            defects.push(MigrationCenterDefect::new(
                MigrationCenterDefectKind::EntryRequiresMarketplaceDetour,
                &entry.entry_id,
                "requires_marketplace_detour",
                "claimed beta entry must not require hidden marketplace install",
            ));
        }
        if entry.learnability_claim.claim_class == ClaimClass::Beta
            && option_empty(&entry.help_pack_item_ref)
        {
            defects.push(MigrationCenterDefect::new(
                MigrationCenterDefectKind::HelpPackItemRefMissing,
                &entry.entry_id,
                "help_pack_item_ref",
                "claimed beta entry must resolve to a versioned help-pack item",
            ));
        }

        match entry.keyboard_reach {
            KeyboardReachClass::KeyboardFirstCommandInvocation => {
                if entry
                    .command_id
                    .as_deref()
                    .map(str::trim)
                    .unwrap_or("")
                    .is_empty()
                {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::EntryCommandIdMissing,
                        &entry.entry_id,
                        "command_id",
                        "keyboard_first_command_invocation requires a command_id",
                    ));
                }
            }
            KeyboardReachClass::KeyboardChordReference => {
                if entry
                    .keyboard_chord_ref
                    .as_deref()
                    .map(str::trim)
                    .unwrap_or("")
                    .is_empty()
                {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::EntryKeyboardUnreachable,
                        &entry.entry_id,
                        "keyboard_chord_ref",
                        "keyboard_chord_reference requires a keyboard_chord_ref",
                    ));
                }
            }
            KeyboardReachClass::KeyboardReachableFocusPath => {}
        }

        match entry.entry_kind {
            EntryKind::DocsHelpAnchor => {
                if option_empty(&entry.docs_help_anchor_ref) {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::DocsAnchorRefMissing,
                        &entry.entry_id,
                        "docs_help_anchor_ref",
                        "docs_help_anchor entry must quote a docs_help_anchor_ref",
                    ));
                }
            }
            EntryKind::GlossaryAnchor => {
                if option_empty(&entry.glossary_pack_ref) {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::GlossaryPackRefMissing,
                        &entry.entry_id,
                        "glossary_pack_ref",
                        "glossary_anchor entry must quote a glossary_pack_ref",
                    ));
                }
            }
            EntryKind::MigrationReportReopen => {
                if option_empty(&entry.migration_report_ref) {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::MigrationReportRefMissing,
                        &entry.entry_id,
                        "migration_report_ref",
                        "migration_report_reopen entry must quote a migration_report_ref",
                    ));
                }
            }
            EntryKind::KnownLimitsLane => {
                if option_empty(&entry.known_limit_ref) {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::KnownLimitRefMissing,
                        &entry.entry_id,
                        "known_limit_ref",
                        "known_limits_lane entry must quote a known_limit_ref",
                    ));
                }
            }
            EntryKind::RecoveryAction => {
                if entry.recovery_route_class.is_none() {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::RecoveryRouteClassMissing,
                        &entry.entry_id,
                        "recovery_route_class",
                        "recovery_action entry must quote a recovery_route_class",
                    ));
                }
            }
            EntryKind::FirstRunConfusionExit | EntryKind::KeymapReference => {}
        }

        let claim = &entry.learnability_claim;
        if claim.claim_class == ClaimClass::Beta && claim.freshness_class.is_blocking_for_beta() {
            defects.push(MigrationCenterDefect::new(
                MigrationCenterDefectKind::FreshnessReviewOverdueOnClaimedRow,
                &entry.entry_id,
                "learnability_claim.freshness_class",
                "claimed beta row must not surface as review_overdue without explicit downgrade",
            ));
        }
        if claim.claim_class == ClaimClass::Beta && claim.lifecycle_class != LifecycleClass::Beta {
            defects.push(MigrationCenterDefect::new(
                MigrationCenterDefectKind::ClaimLifecycleFreshnessDrift,
                &entry.entry_id,
                "learnability_claim.lifecycle_class",
                "beta_claimed row must declare lifecycle_class=beta",
            ));
        }
    }

    let mut seen_section_kinds: BTreeSet<SectionKind> = BTreeSet::new();
    for section in sections {
        if !seen_section_kinds.insert(section.section_kind) {
            defects.push(MigrationCenterDefect::new(
                MigrationCenterDefectKind::DuplicateSectionKind,
                &section.section_id,
                "section_kind",
                "two sections share the same section_kind",
            ));
        }
        for entry_id in &section.entry_ids {
            if !entries.iter().any(|e| &e.entry_id == entry_id) {
                defects.push(MigrationCenterDefect::new(
                    MigrationCenterDefectKind::SectionUnknownEntry,
                    &section.section_id,
                    "entry_ids",
                    format!("section references unknown entry id {entry_id}"),
                ));
            }
        }
    }

    for required in SectionKind::required_sections() {
        if !seen_section_kinds.contains(&required) {
            defects.push(MigrationCenterDefect::new(
                MigrationCenterDefectKind::SectionMissingRequiredKind,
                MIGRATION_CENTER_PAGE_ID,
                "sections",
                format!("page must include section kind {}", required.as_str()),
            ));
        }
    }

    for support in support_rows {
        match entries.iter().find(|e| e.entry_id == support.entry_id) {
            None => {
                defects.push(MigrationCenterDefect::new(
                    MigrationCenterDefectKind::SupportRowUnknownEntry,
                    &support.support_row_id,
                    "entry_id",
                    "support row references an unknown entry id",
                ));
            }
            Some(entry) => {
                if support.section_kind_token != entry.section_kind_token
                    || support.entry_kind_token != entry.entry_kind_token
                    || support.keyboard_reach_token != entry.keyboard_reach_token
                    || support.claim_class_token != entry.learnability_claim.claim_class_token
                    || support.lifecycle_class_token
                        != entry.learnability_claim.lifecycle_class_token
                    || support.freshness_class_token
                        != entry.learnability_claim.freshness_class_token
                    || support.command_id != entry.command_id
                    || support.docs_help_anchor_ref != entry.docs_help_anchor_ref
                    || support.migration_report_ref != entry.migration_report_ref
                    || support.known_limit_ref != entry.known_limit_ref
                    || support.recovery_route_class_token != entry.recovery_route_class_token
                    || support.help_pack_id != entry.help_pack_id
                    || support.help_pack_item_ref != entry.help_pack_item_ref
                    || support.help_pack_version_ref != entry.help_pack_version_ref
                    || support.help_pack_fallback_state_token
                        != entry.help_pack_fallback_state_token
                {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::SupportRowVocabularyDrift,
                        &support.support_row_id,
                        "support_row",
                        "support row drifted from the live entry vocabulary",
                    ));
                }
                if support.raw_body_exported {
                    defects.push(MigrationCenterDefect::new(
                        MigrationCenterDefectKind::SupportRowVocabularyDrift,
                        &support.support_row_id,
                        "raw_body_exported",
                        "support row must not export raw body material",
                    ));
                }
            }
        }
    }

    defects
}

/// Validate a migration-center page; returns the typed defect list.
///
/// # Errors
/// Returns the audited defect list when validation fails.
pub fn validate_migration_center_page(
    page: &MigrationCenterPage,
) -> Result<(), Vec<MigrationCenterDefect>> {
    let defects = audit_migration_center_rows(&page.sections, &page.entries, &page.support_rows);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Builds the seeded migration-center page consumed by the headless
/// inspector, the live shell, and the integration test.
pub fn seeded_migration_center_page() -> MigrationCenterPage {
    let claim = LearnabilityClaim::beta_current();
    let truth_report = seeded_truth_wiring_report();
    let release_truth_binding = truth_report
        .binding_for(TruthSurfaceClass::MigrationCenter)
        .expect("seeded truth report must include migration-center binding")
        .clone();

    let wizard = seeded_migration_wizard_page();
    let scoreboard = seeded_migration_scoreboard();
    let help_pack = seeded_onboarding_help_pack_beta_manifest();

    let mapping_report_ref = wizard.mapping_report.mapping_report_id.clone();
    let checkpoint_ref = wizard.rollback_checkpoint.checkpoint_ref.clone();
    let wizard_session_ref = wizard.wizard_session_id.clone();
    let first_known_limit_ref = scoreboard
        .sections
        .first()
        .and_then(|s| s.rows.first())
        .map(|row| row.flow_id.clone())
        .unwrap_or_else(|| "migration_corpus:known_limit:beta:unknown".to_owned());
    let second_known_limit_ref = scoreboard
        .sections
        .get(1)
        .and_then(|s| s.rows.first())
        .map(|row| row.flow_id.clone())
        .unwrap_or_else(|| first_known_limit_ref.clone());

    let mut entries = vec![
        build_entry(
            "shell:migration_center_beta:entry:first_run_overview_docs",
            SectionKind::DocsHelpAnchors,
            EntryKind::DocsHelpAnchor,
            "Start with the first-run overview",
            "Open the docs anchor that explains the first useful work after install.",
            None,
            Some("docs:help:m3:first_run_overview".to_owned()),
            None,
            None,
            None,
            None,
            None,
            KeyboardReachClass::KeyboardReachableFocusPath,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:open_docs_browser_command",
            SectionKind::DocsHelpAnchors,
            EntryKind::DocsHelpAnchor,
            "Open the docs browser",
            "Launch the in-product docs browser from the command palette.",
            Some("cmd:docs.open_in_browser".to_owned()),
            Some("docs:help:m3:docs_browser_contract".to_owned()),
            None,
            None,
            None,
            None,
            None,
            KeyboardReachClass::KeyboardFirstCommandInvocation,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:glossary_pack_truth_terms",
            SectionKind::GlossaryPacks,
            EntryKind::GlossaryAnchor,
            "Aureline truth terms",
            "Open the glossary pack that defines claim, lifecycle, and freshness vocabulary.",
            None,
            None,
            Some("glossary:pack:truth_terms:v1".to_owned()),
            None,
            None,
            None,
            None,
            KeyboardReachClass::KeyboardReachableFocusPath,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:glossary_pack_migration_terms",
            SectionKind::GlossaryPacks,
            EntryKind::GlossaryAnchor,
            "Migration vocabulary",
            "Open the glossary pack that names Exact, Translated, Partial, Shimmed, and Unsupported.",
            None,
            None,
            Some("glossary:pack:migration_terms:v1".to_owned()),
            None,
            None,
            None,
            None,
            KeyboardReachClass::KeyboardReachableFocusPath,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:reopen_wizard_mapping_report",
            SectionKind::MigrationReports,
            EntryKind::MigrationReportReopen,
            "Reopen the migration mapping report",
            "Reopen the wizard mapping report retained after first run.",
            Some("cmd:workspace.import_profile".to_owned()),
            None,
            None,
            Some(mapping_report_ref.clone()),
            None,
            None,
            None,
            KeyboardReachClass::KeyboardFirstCommandInvocation,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:reopen_rollback_checkpoint",
            SectionKind::MigrationReports,
            EntryKind::MigrationReportReopen,
            "Reopen the migration rollback checkpoint",
            "Reopen the wizard's rollback checkpoint binding so the apply can be undone.",
            Some("cmd:workspace.restore_from_checkpoint".to_owned()),
            None,
            None,
            Some(checkpoint_ref.clone()),
            None,
            None,
            None,
            KeyboardReachClass::KeyboardFirstCommandInvocation,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:known_limits_vs_code_command_palette",
            SectionKind::KnownLimits,
            EntryKind::KnownLimitsLane,
            "Known limits: command palette parity",
            "Open the scoreboard row that records VS Code command palette parity.",
            None,
            None,
            None,
            None,
            Some(first_known_limit_ref.clone()),
            None,
            None,
            KeyboardReachClass::KeyboardReachableFocusPath,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:known_limits_jetbrains_keymap",
            SectionKind::KnownLimits,
            EntryKind::KnownLimitsLane,
            "Known limits: JetBrains keymap",
            "Open the scoreboard row that records JetBrains keymap parity.",
            None,
            None,
            None,
            None,
            Some(second_known_limit_ref.clone()),
            None,
            None,
            KeyboardReachClass::KeyboardReachableFocusPath,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:recovery_safe_mode",
            SectionKind::RecoveryRoutes,
            EntryKind::RecoveryAction,
            "Restart in safe mode",
            "Restart the shell into safe mode for first-run repair without state deletion.",
            None,
            None,
            None,
            None,
            None,
            Some(RecoveryRouteClass::SafeMode),
            None,
            KeyboardReachClass::KeyboardReachableFocusPath,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:recovery_restore_checkpoint",
            SectionKind::RecoveryRoutes,
            EntryKind::RecoveryAction,
            "Restore from a saved checkpoint",
            "Roll back to the wizard rollback checkpoint or another restore-chooser entry.",
            Some("cmd:workspace.restore_from_checkpoint".to_owned()),
            None,
            None,
            None,
            None,
            Some(RecoveryRouteClass::RestoreChooser),
            None,
            KeyboardReachClass::KeyboardFirstCommandInvocation,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:recovery_support_export",
            SectionKind::RecoveryRoutes,
            EntryKind::RecoveryAction,
            "Export support evidence",
            "Export a redacted support packet before any state change.",
            None,
            None,
            None,
            None,
            None,
            Some(RecoveryRouteClass::SupportExport),
            None,
            KeyboardReachClass::KeyboardReachableFocusPath,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:first_run_command_palette",
            SectionKind::FirstRunConfusionExits,
            EntryKind::FirstRunConfusionExit,
            "Open the command palette",
            "Open the command palette to invoke any keyboard-reachable action by name.",
            Some("cmd:command_palette.open".to_owned()),
            None,
            None,
            None,
            None,
            None,
            None,
            KeyboardReachClass::KeyboardFirstCommandInvocation,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:first_run_settings_open",
            SectionKind::FirstRunConfusionExits,
            EntryKind::FirstRunConfusionExit,
            "Open settings",
            "Open the settings root to review or change defaults.",
            Some("cmd:settings.open".to_owned()),
            None,
            None,
            None,
            None,
            None,
            None,
            KeyboardReachClass::KeyboardFirstCommandInvocation,
            &claim,
        ),
        build_entry(
            "shell:migration_center_beta:entry:keymap_reference_chord",
            SectionKind::FirstRunConfusionExits,
            EntryKind::KeymapReference,
            "Show keymap reference",
            "Quote the documented keymap chord for opening the command palette.",
            None,
            None,
            None,
            None,
            None,
            None,
            Some("keymap:chord:command_palette.open".to_owned()),
            KeyboardReachClass::KeyboardChordReference,
            &claim,
        ),
    ];

    attach_help_pack_refs(&mut entries, &help_pack);

    let sections = build_sections(&entries);
    let support_rows: Vec<MigrationCenterSupportRow> = entries
        .iter()
        .map(MigrationCenterSupportRow::from_entry)
        .collect();

    let upstream_refs = MigrationCenterUpstreamRefs {
        wizard_session_ref,
        wizard_mapping_report_ref: mapping_report_ref,
        wizard_rollback_checkpoint_ref: checkpoint_ref,
        wizard_shared_contract_ref: MIGRATION_WIZARD_SHARED_CONTRACT_REF.to_owned(),
        corpus_scoreboard_ref: MIGRATION_SCOREBOARD_ID.to_owned(),
        corpus_shared_contract_ref: MIGRATION_CORPUS_SHARED_CONTRACT_REF.to_owned(),
        help_pack_manifest_ref: ONBOARDING_HELP_PACK_BETA_FIXTURE_REF.to_owned(),
        help_pack_manifest_id: help_pack.manifest_id.clone(),
        help_pack_version_ref: help_pack.manifest_version_ref.clone(),
        help_pack_support_export_ref: ONBOARDING_HELP_PACK_BETA_SUPPORT_EXPORT_FIXTURE_REF
            .to_owned(),
    };

    let defects = audit_migration_center_rows(&sections, &entries, &support_rows);
    let summary = build_summary(&sections, &entries, defects.len());

    MigrationCenterPage {
        record_kind: MIGRATION_CENTER_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_CENTER_BETA_SCHEMA_VERSION,
        shared_contract_ref: MIGRATION_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
        page_id: MIGRATION_CENTER_PAGE_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        learnability_claim: claim,
        release_truth_binding,
        upstream_refs,
        sections,
        entries,
        support_rows,
        defects,
        summary,
    }
}

fn build_sections(entries: &[MigrationCenterEntryPoint]) -> Vec<MigrationCenterSection> {
    SectionKind::required_sections()
        .into_iter()
        .map(|kind| {
            let entry_ids: Vec<String> = entries
                .iter()
                .filter(|e| e.section_kind == kind)
                .map(|e| e.entry_id.clone())
                .collect();
            MigrationCenterSection {
                record_kind: MIGRATION_CENTER_BETA_SECTION_RECORD_KIND.to_owned(),
                schema_version: MIGRATION_CENTER_BETA_SCHEMA_VERSION,
                shared_contract_ref: MIGRATION_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
                section_id: format!("shell:migration_center_beta:section:{}", kind.as_str()),
                section_kind: kind,
                section_kind_token: kind.as_str().to_owned(),
                title_label: kind.display_label().to_owned(),
                description_label: section_description(kind).to_owned(),
                entry_ids,
            }
        })
        .collect()
}

fn build_summary(
    sections: &[MigrationCenterSection],
    entries: &[MigrationCenterEntryPoint],
    defect_count: usize,
) -> MigrationCenterSummary {
    let keyboard_first_count = entries
        .iter()
        .filter(|e| e.keyboard_reach == KeyboardReachClass::KeyboardFirstCommandInvocation)
        .count();
    let recovery_route_count = entries
        .iter()
        .filter(|e| e.entry_kind == EntryKind::RecoveryAction)
        .count();
    let docs_help_count = entries
        .iter()
        .filter(|e| e.entry_kind == EntryKind::DocsHelpAnchor)
        .count();
    let known_limit_count = entries
        .iter()
        .filter(|e| e.entry_kind == EntryKind::KnownLimitsLane)
        .count();
    let migration_report_reopen_count = entries
        .iter()
        .filter(|e| e.entry_kind == EntryKind::MigrationReportReopen)
        .count();
    let glossary_anchor_count = entries
        .iter()
        .filter(|e| e.entry_kind == EntryKind::GlossaryAnchor)
        .count();
    let first_run_exit_count = entries
        .iter()
        .filter(|e| {
            e.entry_kind == EntryKind::FirstRunConfusionExit
                || e.entry_kind == EntryKind::KeymapReference
        })
        .count();
    MigrationCenterSummary {
        section_count: sections.len(),
        entry_count: entries.len(),
        keyboard_first_count,
        recovery_route_count,
        docs_help_count,
        known_limit_count,
        migration_report_reopen_count,
        glossary_anchor_count,
        first_run_exit_count,
        defect_count,
    }
}

fn section_description(kind: SectionKind) -> &'static str {
    match kind {
        SectionKind::DocsHelpAnchors => "Current docs and help anchors users open from the page.",
        SectionKind::GlossaryPacks => "Glossary packs that define aureline-specific vocabulary.",
        SectionKind::MigrationReports => {
            "Retained migration reports and rollback checkpoints from the wizard."
        }
        SectionKind::KnownLimits => "Known limits from the incumbent-flow scoreboard.",
        SectionKind::RecoveryRoutes => "Recovery routes owned by the recovery surface.",
        SectionKind::FirstRunConfusionExits => {
            "Explicit exits from first-run confusion: command palette, settings, keymap chord."
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn build_entry(
    entry_id: &str,
    section_kind: SectionKind,
    entry_kind: EntryKind,
    title: &str,
    description: &str,
    command_id: Option<String>,
    docs_help_anchor_ref: Option<String>,
    glossary_pack_ref: Option<String>,
    migration_report_ref: Option<String>,
    known_limit_ref: Option<String>,
    recovery_route_class: Option<RecoveryRouteClass>,
    keyboard_chord_ref: Option<String>,
    keyboard_reach: KeyboardReachClass,
    claim: &LearnabilityClaim,
) -> MigrationCenterEntryPoint {
    let section_id = format!(
        "shell:migration_center_beta:section:{}",
        section_kind.as_str()
    );
    let support_row_id = format!("{entry_id}::support");
    MigrationCenterEntryPoint {
        record_kind: MIGRATION_CENTER_BETA_ENTRY_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_CENTER_BETA_SCHEMA_VERSION,
        shared_contract_ref: MIGRATION_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
        entry_id: entry_id.to_owned(),
        section_id,
        section_kind,
        section_kind_token: section_kind.as_str().to_owned(),
        entry_kind,
        entry_kind_token: entry_kind.as_str().to_owned(),
        title_label: title.to_owned(),
        description_label: description.to_owned(),
        command_id,
        docs_help_anchor_ref,
        glossary_pack_ref,
        migration_report_ref,
        known_limit_ref,
        recovery_route_class,
        recovery_route_class_token: recovery_route_class.map(|c| c.as_str().to_owned()),
        keyboard_chord_ref,
        help_pack_id: None,
        help_pack_item_ref: None,
        help_pack_version_ref: None,
        help_pack_fallback_state_token: None,
        keyboard_reach,
        keyboard_reach_token: keyboard_reach.as_str().to_owned(),
        requires_account_detour: false,
        requires_marketplace_detour: false,
        learnability_claim: claim.clone(),
        support_row_id,
    }
}

fn attach_help_pack_refs(
    entries: &mut [MigrationCenterEntryPoint],
    manifest: &OnboardingHelpPackBetaManifest,
) {
    for entry in entries {
        let Some(item_id) = help_pack_item_for_entry(&entry.entry_id) else {
            continue;
        };
        if let Some(item) = manifest.item(item_id) {
            entry.help_pack_id = Some(item.pack_id.clone());
            entry.help_pack_item_ref = Some(item.item_id.clone());
            entry.help_pack_version_ref = Some(item.pack_version_ref.clone());
            entry.help_pack_fallback_state_token =
                Some(item.source_language_fallback.fallback_class.clone());
        }
    }
}

fn help_pack_item_for_entry(entry_id: &str) -> Option<&'static str> {
    let item = match entry_id {
        "shell:migration_center_beta:entry:first_run_overview_docs"
        | "shell:migration_center_beta:entry:first_run_settings_open" => {
            "ohp:item:first_run.open_folder"
        }
        "shell:migration_center_beta:entry:open_docs_browser_command" => {
            "ohp:item:offline.docs_browser_mirror"
        }
        "shell:migration_center_beta:entry:glossary_pack_truth_terms" => {
            "ohp:item:glossary.release_truth"
        }
        "shell:migration_center_beta:entry:glossary_pack_migration_terms"
        | "shell:migration_center_beta:entry:known_limits_vs_code_command_palette"
        | "shell:migration_center_beta:entry:known_limits_jetbrains_keymap" => {
            "ohp:item:glossary.migration_outcomes"
        }
        "shell:migration_center_beta:entry:reopen_wizard_mapping_report"
        | "shell:migration_center_beta:entry:first_run_command_palette"
        | "shell:migration_center_beta:entry:keymap_reference_chord" => {
            "ohp:item:keymap_bridge.command_palette"
        }
        "shell:migration_center_beta:entry:reopen_rollback_checkpoint"
        | "shell:migration_center_beta:entry:recovery_safe_mode"
        | "shell:migration_center_beta:entry:recovery_restore_checkpoint" => {
            "ohp:item:recovery.restore_checkpoint"
        }
        "shell:migration_center_beta:entry:recovery_support_export" => {
            "ohp:item:recovery.support_export"
        }
        _ => return None,
    };
    Some(item)
}

fn option_empty(value: &Option<String>) -> bool {
    value.as_deref().map(str::trim).unwrap_or("").is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_migration_center_page();
        assert!(page.defects.is_empty());
        validate_migration_center_page(&page).expect("seeded page validates");
    }

    #[test]
    fn seeded_page_covers_every_required_section() {
        let page = seeded_migration_center_page();
        assert!(page.covers_every_required_section());
        assert_eq!(page.sections.len(), SectionKind::required_sections().len());
    }

    #[test]
    fn seeded_page_has_recovery_and_keyboard_routes() {
        let page = seeded_migration_center_page();
        assert!(page.summary.recovery_route_count >= 1);
        assert!(page.summary.keyboard_first_count >= 1);
        assert!(page.summary.migration_report_reopen_count >= 1);
        assert!(page.summary.known_limit_count >= 1);
        assert!(page.summary.glossary_anchor_count >= 1);
    }

    #[test]
    fn seeded_page_quotes_upstream_refs() {
        let page = seeded_migration_center_page();
        assert_eq!(
            page.upstream_refs.wizard_shared_contract_ref,
            MIGRATION_WIZARD_SHARED_CONTRACT_REF
        );
        assert_eq!(
            page.upstream_refs.corpus_shared_contract_ref,
            MIGRATION_CORPUS_SHARED_CONTRACT_REF
        );
        assert!(!page.upstream_refs.wizard_mapping_report_ref.is_empty());
        assert!(!page.upstream_refs.wizard_rollback_checkpoint_ref.is_empty());
    }

    #[test]
    fn seeded_page_quotes_release_truth_binding() {
        let page = seeded_migration_center_page();
        assert_eq!(
            page.release_truth_binding.surface_class,
            TruthSurfaceClass::MigrationCenter
        );
        assert!(page
            .release_truth_binding
            .claim_row_ids
            .contains(&"m3_claim_row:beta_surface.importer_and_migration".to_owned()));
        assert!(!page.release_truth_binding.compatibility_row_refs.is_empty());
        assert!(page
            .release_truth_binding
            .missing_compatibility_row_refs
            .is_empty());
    }

    #[test]
    fn account_detour_drill_emits_defect() {
        let mut page = seeded_migration_center_page();
        page.entries[0].requires_account_detour = true;
        let defects =
            audit_migration_center_rows(&page.sections, &page.entries, &page.support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == MigrationCenterDefectKind::EntryRequiresAccountDetour));
    }

    #[test]
    fn missing_command_id_drill_emits_defect() {
        let mut page = seeded_migration_center_page();
        let idx = page
            .entries
            .iter()
            .position(|e| e.keyboard_reach == KeyboardReachClass::KeyboardFirstCommandInvocation)
            .expect("seeded page must include a keyboard-first entry");
        page.entries[idx].command_id = None;
        let defects =
            audit_migration_center_rows(&page.sections, &page.entries, &page.support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == MigrationCenterDefectKind::EntryCommandIdMissing));
    }

    #[test]
    fn support_row_drift_drill_emits_defect() {
        let mut page = seeded_migration_center_page();
        page.support_rows[0].keyboard_reach_token = "made_up".to_owned();
        let defects =
            audit_migration_center_rows(&page.sections, &page.entries, &page.support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == MigrationCenterDefectKind::SupportRowVocabularyDrift));
    }

    #[test]
    fn support_export_round_trips() {
        let page = seeded_migration_center_page();
        let export = MigrationCenterSupportExport::from_page(
            "support-export:migration-center-beta:001",
            "2026-05-15T00:00:00Z",
            page.clone(),
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.case_ids.contains(&page.page_id));
        for entry in &page.entries {
            assert!(export.case_ids.contains(&entry.entry_id));
            assert!(export.case_ids.contains(&entry.support_row_id));
        }
    }

    #[test]
    fn review_overdue_on_claimed_row_emits_defect() {
        let mut page = seeded_migration_center_page();
        page.entries[0].learnability_claim.freshness_class = FreshnessClass::ReviewOverdue;
        page.entries[0].learnability_claim.freshness_class_token =
            FreshnessClass::ReviewOverdue.as_str().to_owned();
        let defects =
            audit_migration_center_rows(&page.sections, &page.entries, &page.support_rows);
        assert!(defects.iter().any(
            |d| d.defect_kind == MigrationCenterDefectKind::FreshnessReviewOverdueOnClaimedRow
        ));
    }
}
