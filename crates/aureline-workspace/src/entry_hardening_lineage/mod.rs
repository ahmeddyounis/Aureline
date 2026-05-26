//! Workspace entry hardening lineage: the governed, export-safe projection
//! that finalizes target-kind truth, distinct verbs, durable post-entry
//! checkpoints, and failure-repair honesty for the Open / Clone / Import /
//! Add root / Restore / Resume flows into one record per posture.
//!
//! The projection ingests an already-built [`ProjectEntryReviewRecord`]
//! verbatim. The entry review module owns the heavy lifting (admission
//! packet, review sheets, destination collision review, post-entry handoff
//! card, failure repair state, surface parity). This module promotes that
//! record into a stable-line lineage artifact that proves the four
//! invariants the entry-hardening lane is anchored on:
//!
//! - **Verb truth.** The entry verb stayed distinct, the resulting mode
//!   belongs to that verb's allowed set, and the review sheet kind matches
//!   the verb / target combination.
//! - **Target-kind truth.** The target topology class is named explicitly
//!   (`durable_open`, `opened_sparse`, `pointer_only`, `nested_child`,
//!   `parent_root`, `imported_packet`, `inspect_only_staging`,
//!   `restore_target`, `acquired_not_fetched`), non-durable staging is
//!   labelled, and destination collisions force an explicit choice.
//! - **Durable checkpoint.** The post-entry handoff card names deferred
//!   work, blocked/recommended/optional readiness, and same-weight
//!   continuity actions (`Set up later`, `Open minimal`, `Cancel`). The
//!   admission checkpoint route is present and citable.
//! - **No hidden side effects.** Clone never grants trust, defers
//!   dependency restore and tasks; import defers durable writes and state
//!   rehydration before review; open workspace forbids silent schema
//!   upgrades. Failed attempts preserve typed inputs, chosen destination,
//!   and redacted diagnostics.
//!
//! When the projection cannot prove a claim on the captured posture it
//! auto-narrows below Stable with a named [`EntryHardeningNarrowReason`]
//! instead of inheriting an adjacent green row. The record carries no raw
//! source bytes (`raw_payload_excluded = true`) and is safe for support
//! export.

use serde::{Deserialize, Serialize};

use crate::admission::checkpoint::ReadinessTaskClass;
use crate::entry::{
    EntryDestinationCollisionClass, EntryReviewRequirementClass, EntryReviewSheetKind,
    ProjectEntryReviewRecord,
};
use crate::{
    AdmissionAction, AdmissionSourceSurface, EntryDeferredWorkClass, EntryVerb, ResultingMode,
    TargetKind,
};

/// Schema version for [`EntryHardeningLineageRecord`].
pub const ENTRY_HARDENING_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the entry hardening lineage record.
pub const ENTRY_HARDENING_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/entry_hardening_lineage.schema.json";

/// Stable record-kind tag for the entry hardening lineage record.
pub const ENTRY_HARDENING_LINEAGE_RECORD_KIND: &str = "entry_hardening_lineage_record";

// ---------------------------------------------------------------------------
// Target topology class.
// ---------------------------------------------------------------------------

/// Post-entry target topology class derived from the review sheet and the
/// collision review. The class names how the target was acquired or opened
/// so later search, Git, trust, restore, and support surfaces inherit truthful
/// topology metadata instead of reverse-engineering it from path strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryTargetTopologyClass {
    /// Open against an existing durable target.
    DurableOpen,
    /// Clone landed bytes but no working tree was fetched (clone-only).
    AcquiredNotFetched,
    /// Open or clone restricted to a sparse-checkout or partial-filter scope.
    OpenedSparse,
    /// Open or clone is LFS pointer-only (no large-file hydration yet).
    PointerOnly,
    /// Target sits inside an outer repository (nested child posture).
    NestedChild,
    /// Target is a repository or workspace root that contains other roots.
    ParentRoot,
    /// Import staged a packet for review without durable writes.
    ImportedPacket,
    /// Inspect-only flow that uses non-durable staging.
    InspectOnlyStaging,
    /// Restore or resume target re-projects prior session state.
    RestoreTarget,
}

impl EntryTargetTopologyClass {
    /// Returns the stable snake_case token for this topology class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableOpen => "durable_open",
            Self::AcquiredNotFetched => "acquired_not_fetched",
            Self::OpenedSparse => "opened_sparse",
            Self::PointerOnly => "pointer_only",
            Self::NestedChild => "nested_child",
            Self::ParentRoot => "parent_root",
            Self::ImportedPacket => "imported_packet",
            Self::InspectOnlyStaging => "inspect_only_staging",
            Self::RestoreTarget => "restore_target",
        }
    }
}

// ---------------------------------------------------------------------------
// Inspection hooks.
// ---------------------------------------------------------------------------

/// Class of pre-commit inspection / repair hook available before the entry
/// commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryInspectionHookClass {
    /// Open the verb-specific entry review sheet.
    ReviewEntry,
    /// Inspect the destination collision review sheet.
    InspectCollision,
    /// Inspect the post-entry handoff card before commit.
    InspectHandoff,
    /// Inspect the failure repair state (typed inputs, redacted diagnostics).
    InspectFailureRepair,
    /// Export the entry hardening lineage record (support-safe, no raw bytes).
    Export,
}

impl EntryInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewEntry => "review_entry",
            Self::InspectCollision => "inspect_collision",
            Self::InspectHandoff => "inspect_handoff",
            Self::InspectFailureRepair => "inspect_failure_repair",
            Self::Export => "export",
        }
    }
}

/// One pre-commit inspection / repair hook offered before the entry commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryInspectionHook {
    /// Hook class.
    pub hook_class: EntryInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable for this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-commit inspection / repair hook table for the
/// entry hardening lineage. All hooks are available by default; postures may
/// model a degraded subset to prove the corresponding narrow reason.
pub fn default_entry_hardening_inspection_hooks() -> Vec<EntryInspectionHook> {
    vec![
        EntryInspectionHook {
            hook_class: EntryInspectionHookClass::ReviewEntry,
            action_id: "entry_hardening.review_entry".to_owned(),
            label: "Review entry".to_owned(),
            available: true,
            disclosure:
                "Opens the verb-specific entry review sheet before any durable write or trust change."
                    .to_owned(),
        },
        EntryInspectionHook {
            hook_class: EntryInspectionHookClass::InspectCollision,
            action_id: "entry_hardening.inspect_collision".to_owned(),
            label: "Inspect destination collision".to_owned(),
            available: true,
            disclosure:
                "Shows the destination collision review with safe actions and forbids generic overwrite."
                    .to_owned(),
        },
        EntryInspectionHook {
            hook_class: EntryInspectionHookClass::InspectHandoff,
            action_id: "entry_hardening.inspect_handoff".to_owned(),
            label: "Inspect post-entry handoff".to_owned(),
            available: true,
            disclosure:
                "Shows what Aureline intentionally did not run yet and offers Set up later / Open minimal / Cancel."
                    .to_owned(),
        },
        EntryInspectionHook {
            hook_class: EntryInspectionHookClass::InspectFailureRepair,
            action_id: "entry_hardening.inspect_failure_repair".to_owned(),
            label: "Inspect failure repair".to_owned(),
            available: true,
            disclosure:
                "Shows typed source input, chosen destination, and redacted diagnostics preserved after failure."
                    .to_owned(),
        },
        EntryInspectionHook {
            hook_class: EntryInspectionHookClass::Export,
            action_id: "entry_hardening.export".to_owned(),
            label: "Export entry hardening lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this entry hardening lineage record for support without raw inputs."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Narrow reasons + stable qualification.
// ---------------------------------------------------------------------------

/// Named reason an entry hardening lineage record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryHardeningNarrowReason {
    /// The review sheet kind does not match the verb / target combination.
    ReviewSheetMismatch,
    /// Clone review is missing or admits silent trust grant.
    CloneGrantsTrustSilently,
    /// Clone review admits silent dependency restore or task execution.
    CloneRunsSetupSilently,
    /// Import review admits durable writes before review.
    ImportWritesBeforeReview,
    /// Import review admits state rehydration before review.
    ImportRehydratesBeforeReview,
    /// Open-workspace review admits silent schema upgrades.
    WorkspaceManifestUpgradesSilently,
    /// Destination collision present but no explicit choice required.
    DestinationCollisionNoExplicitChoice,
    /// Post-entry handoff card is missing both `Set up later` and
    /// `Open minimal` continuity actions.
    HandoffMissingContinuityPaths,
    /// Failed attempt repair state would lose typed inputs, destination, or
    /// redacted diagnostics.
    FailureRepairLosesState,
    /// Repair state's source-input label appears to leak a secret.
    FailureRepairLeaksSecret,
    /// Cross-surface parity drift: a surface changed verb / target / mode.
    SurfaceParityDrift,
    /// A DeepLink surface is covered but no deep-link intent review fires.
    DeepLinkIntentReviewMissing,
    /// A required pre-commit inspection hook is unavailable.
    InspectionHookUnavailable,
    /// Workspace / root context refs are empty (would break support export).
    LineageExportUnsafe,
}

impl EntryHardeningNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewSheetMismatch => "review_sheet_mismatch",
            Self::CloneGrantsTrustSilently => "clone_grants_trust_silently",
            Self::CloneRunsSetupSilently => "clone_runs_setup_silently",
            Self::ImportWritesBeforeReview => "import_writes_before_review",
            Self::ImportRehydratesBeforeReview => "import_rehydrates_before_review",
            Self::WorkspaceManifestUpgradesSilently => "workspace_manifest_upgrades_silently",
            Self::DestinationCollisionNoExplicitChoice => {
                "destination_collision_no_explicit_choice"
            }
            Self::HandoffMissingContinuityPaths => "handoff_missing_continuity_paths",
            Self::FailureRepairLosesState => "failure_repair_loses_state",
            Self::FailureRepairLeaksSecret => "failure_repair_leaks_secret",
            Self::SurfaceParityDrift => "surface_parity_drift",
            Self::DeepLinkIntentReviewMissing => "deep_link_intent_review_missing",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for an entry hardening lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryHardeningQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not qualified.
    pub narrow_reasons: Vec<EntryHardeningNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// Verb truth posture projected from the entry review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerbTruthSummary {
    /// Entry verb the user activated.
    pub entry_verb: EntryVerb,
    /// Target kind the verb resolved against.
    pub target_kind: TargetKind,
    /// Resulting mode admitted before commit.
    pub resulting_mode: ResultingMode,
    /// Source surface that initiated the activation.
    pub source_surface: AdmissionSourceSurface,
    /// Review sheet kind matched by the verb + target combination.
    pub review_sheet_kind: EntryReviewSheetKind,
    /// True when the verb still produces a distinct review sheet
    /// (no collapse to a generic "Get started" action).
    pub verb_stays_distinct: bool,
    /// True when the sheet kind matches the expected verb / target pairing.
    pub sheet_matches_verb: bool,
}

/// Target-kind / topology truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetKindTruthSummary {
    /// Topology class derived from the entry review and collision review.
    pub topology_class: EntryTargetTopologyClass,
    /// Destination collision class (or `no_collision`).
    pub destination_collision_class: EntryDestinationCollisionClass,
    /// True when destination collision forces an explicit user choice.
    pub explicit_choice_required_when_colliding: bool,
    /// True when staging is labelled as non-durable rather than presented
    /// as a normal open.
    pub non_durable_staging_labelled: bool,
    /// True when the resulting mode is one of the verb's distinct outcomes
    /// (Clone only / Clone and review / Clone and open / Clone and add /
    /// Inspect only / Open minimal etc.).
    pub resulting_mode_is_distinct_outcome: bool,
    /// True when the topology class is consistent with the verb (for
    /// example, restore_target only for Restore / Resume; imported_packet
    /// only for Import / StartFromSnapshot).
    pub topology_consistent_with_verb: bool,
    /// Explainer line for tooltip / shell.
    pub summary: String,
}

/// Durable post-entry checkpoint posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCheckpointSummary {
    /// Opaque admission checkpoint id.
    pub admission_checkpoint_id: String,
    /// Opaque handoff card id.
    pub handoff_card_id: String,
    /// Deferred work the entry intentionally did not run yet.
    pub deferred_work: Vec<EntryDeferredWorkClass>,
    /// Readiness tasks that block now.
    pub blocking_now_tasks: Vec<ReadinessTaskClass>,
    /// Readiness tasks recommended soon.
    pub recommended_soon_tasks: Vec<ReadinessTaskClass>,
    /// Readiness tasks optional later.
    pub optional_later_tasks: Vec<ReadinessTaskClass>,
    /// Primary next action offered after entry.
    pub primary_next_action: AdmissionAction,
    /// Safe alternate actions offered alongside the primary action.
    pub safe_alternate_actions: Vec<AdmissionAction>,
    /// True when the `Set up later` continuity action is offered.
    pub set_up_later_offered: bool,
    /// True when the `Open minimal` continuity action is offered.
    pub open_minimal_offered: bool,
    /// True when the `Cancel` continuity action is offered.
    pub cancel_offered: bool,
    /// True when the entry's resulting state is exportable / shareable.
    pub export_or_share_state_available: bool,
}

/// Side-effect posture: proves no hidden trust, setup, hook, dependency,
/// runtime, or rehydration side effects ran during entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectPosture {
    /// Clone never silently grants trust (clone-only invariant).
    pub clone_never_grants_trust: bool,
    /// Clone defers dependency restore.
    pub dependency_restore_deferred: bool,
    /// Clone defers repo task execution and hooks.
    pub task_execution_deferred: bool,
    /// Import refuses durable writes before review.
    pub no_durable_write_before_review: bool,
    /// Import refuses state rehydration before review.
    pub no_state_rehydration_before_review: bool,
    /// Open-workspace forbids silent schema upgrades.
    pub silent_workspace_upgrade_forbidden: bool,
    /// Dropped meaning (lossy migrations) stays disclosed instead of hidden.
    pub dropped_meaning_disclosed: bool,
    /// Deferred work classes explicitly listed in the handoff card.
    pub deferred_work_classes: Vec<EntryDeferredWorkClass>,
}

/// Failure-repair posture: proves typed inputs, destination, and redacted
/// diagnostics survive a failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureRepairTruth {
    /// True when typed source input is preserved on failure.
    pub typed_source_input_preserved: bool,
    /// True when the chosen destination is preserved on failure.
    pub chosen_destination_preserved: bool,
    /// True when redacted diagnostics are preserved on failure.
    pub redacted_diagnostics_preserved: bool,
    /// True when the redacted source-input label appears free of secrets.
    pub source_input_redacted: bool,
    /// True when at least one repair action is offered.
    pub repair_actions_present: bool,
    /// Repair actions offered after failure.
    pub repair_actions: Vec<AdmissionAction>,
}

/// Cross-surface parity posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParityTruth {
    /// Source surfaces with a parity row attached to this activation.
    pub covered_surfaces: Vec<AdmissionSourceSurface>,
    /// True when every covered surface preserves the verb.
    pub all_surfaces_preserve_verb: bool,
    /// True when every covered surface preserves the resulting mode.
    pub all_surfaces_preserve_mode: bool,
    /// True when every covered surface preserves the target kind.
    pub all_surfaces_preserve_target_kind: bool,
    /// True when every covered surface uses the same review model.
    pub same_review_model_on_all: bool,
    /// True when the DeepLink surface is covered (relevant for the
    /// deep-link intent review requirement).
    pub deep_link_surface_covered: bool,
    /// True when the DeepLink surface, if present, carries the deep-link
    /// intent review requirement.
    pub deep_link_intent_review_present: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe entry hardening lineage record per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryHardeningLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub entry_hardening_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Opaque reference to the source entry review record.
    pub entry_review_ref: String,
    /// Opaque reference to the admission review packet.
    pub admission_review_ref: String,
    /// Opaque reference to the admission checkpoint route.
    pub admission_checkpoint_ref: String,
    /// Verb truth pillar.
    pub verb_truth: VerbTruthSummary,
    /// Target-kind / topology truth pillar.
    pub target_kind_truth: TargetKindTruthSummary,
    /// Durable post-entry checkpoint pillar.
    pub durable_checkpoint: DurableCheckpointSummary,
    /// Side-effect posture pillar.
    pub side_effect_posture: SideEffectPosture,
    /// Failure-repair truth pillar.
    pub failure_repair_truth: FailureRepairTruth,
    /// Cross-surface parity truth pillar.
    pub surface_parity_truth: SurfaceParityTruth,
    /// Pre-commit inspection / repair hooks.
    pub inspection_hooks: Vec<EntryInspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: EntryHardeningQualification,
    /// Whether the record is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl EntryHardeningLineageRecord {
    /// Returns true when the record is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == ENTRY_HARDENING_LINEAGE_SCHEMA_REF
            && self.record_kind == ENTRY_HARDENING_LINEAGE_RECORD_KIND
            && !self.entry_review_ref.trim().is_empty()
            && !self.admission_review_ref.trim().is_empty()
            && !self.admission_checkpoint_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(&self, class: EntryInspectionHookClass) -> Option<&EntryInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed entry hardening lineage record from a live
/// [`ProjectEntryReviewRecord`].
pub fn project_entry_hardening_lineage(
    posture_id: impl Into<String>,
    entry_review: &ProjectEntryReviewRecord,
) -> EntryHardeningLineageRecord {
    project_entry_hardening_lineage_with_hooks(
        posture_id,
        entry_review,
        default_entry_hardening_inspection_hooks(),
    )
}

/// Like [`project_entry_hardening_lineage`] but with an explicit
/// inspection-hook set (for testing degraded-hook postures).
pub fn project_entry_hardening_lineage_with_hooks(
    posture_id: impl Into<String>,
    entry_review: &ProjectEntryReviewRecord,
    inspection_hooks: Vec<EntryInspectionHook>,
) -> EntryHardeningLineageRecord {
    let posture_id: String = posture_id.into();

    let admission = &entry_review.admission_review_packet;
    let admission_checkpoint = &entry_review.admission_checkpoint_route;

    let verb_truth = project_verb_truth(entry_review);
    let target_kind_truth = project_target_kind_truth(entry_review);
    let durable_checkpoint = project_durable_checkpoint(entry_review);
    let side_effect_posture = project_side_effect_posture(entry_review);
    let failure_repair_truth = project_failure_repair_truth(entry_review);
    let surface_parity_truth = project_surface_parity_truth(entry_review);

    let mut narrow_reasons = Vec::new();

    if !verb_truth.sheet_matches_verb {
        narrow_reasons.push(EntryHardeningNarrowReason::ReviewSheetMismatch);
    }

    // Clone / Import / Open-workspace guard rails.
    if entry_review.entry_verb == EntryVerb::Clone {
        if !side_effect_posture.clone_never_grants_trust {
            narrow_reasons.push(EntryHardeningNarrowReason::CloneGrantsTrustSilently);
        }
        if !side_effect_posture.dependency_restore_deferred
            || !side_effect_posture.task_execution_deferred
        {
            narrow_reasons.push(EntryHardeningNarrowReason::CloneRunsSetupSilently);
        }
    }
    if matches!(
        entry_review.entry_verb,
        EntryVerb::Import | EntryVerb::StartFromSnapshot
    ) {
        if !side_effect_posture.no_durable_write_before_review {
            narrow_reasons.push(EntryHardeningNarrowReason::ImportWritesBeforeReview);
        }
        if !side_effect_posture.no_state_rehydration_before_review {
            narrow_reasons.push(EntryHardeningNarrowReason::ImportRehydratesBeforeReview);
        }
    }
    if verb_truth.review_sheet_kind == EntryReviewSheetKind::OpenWorkspaceManifest
        && !side_effect_posture.silent_workspace_upgrade_forbidden
    {
        narrow_reasons.push(EntryHardeningNarrowReason::WorkspaceManifestUpgradesSilently);
    }

    if entry_review.destination_collision_review.is_some()
        && !target_kind_truth.explicit_choice_required_when_colliding
    {
        narrow_reasons.push(EntryHardeningNarrowReason::DestinationCollisionNoExplicitChoice);
    }

    if !durable_checkpoint.set_up_later_offered && !durable_checkpoint.open_minimal_offered {
        narrow_reasons.push(EntryHardeningNarrowReason::HandoffMissingContinuityPaths);
    }

    if !failure_repair_truth.typed_source_input_preserved
        || !failure_repair_truth.chosen_destination_preserved
        || !failure_repair_truth.redacted_diagnostics_preserved
        || !failure_repair_truth.repair_actions_present
    {
        narrow_reasons.push(EntryHardeningNarrowReason::FailureRepairLosesState);
    }
    if !failure_repair_truth.source_input_redacted {
        narrow_reasons.push(EntryHardeningNarrowReason::FailureRepairLeaksSecret);
    }

    if !surface_parity_truth.all_surfaces_preserve_verb
        || !surface_parity_truth.all_surfaces_preserve_mode
        || !surface_parity_truth.all_surfaces_preserve_target_kind
        || !surface_parity_truth.same_review_model_on_all
    {
        narrow_reasons.push(EntryHardeningNarrowReason::SurfaceParityDrift);
    }
    if surface_parity_truth.deep_link_surface_covered
        && !surface_parity_truth.deep_link_intent_review_present
    {
        narrow_reasons.push(EntryHardeningNarrowReason::DeepLinkIntentReviewMissing);
    }

    if !inspection_hooks
        .iter()
        .all(|hook| hook.available || hook.hook_class == EntryInspectionHookClass::InspectCollision)
    {
        narrow_reasons.push(EntryHardeningNarrowReason::InspectionHookUnavailable);
    }

    let entry_review_ref = format!("entry_review:{}", entry_review.entry_review_id);
    let admission_review_ref = format!("admission_review:{}", admission.admission_review_id);
    let admission_checkpoint_ref = format!(
        "admission_checkpoint:{}",
        admission_checkpoint.checkpoint.admission_checkpoint_id
    );

    if entry_review_ref
        .trim_start_matches("entry_review:")
        .is_empty()
        || admission_review_ref
            .trim_start_matches("admission_review:")
            .is_empty()
        || admission_checkpoint_ref
            .trim_start_matches("admission_checkpoint:")
            .is_empty()
    {
        narrow_reasons.push(EntryHardeningNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = EntryHardeningQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        entry_review,
        &verb_truth,
        &target_kind_truth,
        &durable_checkpoint,
        &stable_qualification,
    );

    EntryHardeningLineageRecord {
        record_kind: ENTRY_HARDENING_LINEAGE_RECORD_KIND.to_owned(),
        entry_hardening_lineage_schema_version: ENTRY_HARDENING_LINEAGE_SCHEMA_VERSION,
        schema_ref: ENTRY_HARDENING_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        entry_review_ref,
        admission_review_ref,
        admission_checkpoint_ref,
        verb_truth,
        target_kind_truth,
        durable_checkpoint,
        side_effect_posture,
        failure_repair_truth,
        surface_parity_truth,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_verb_truth(entry_review: &ProjectEntryReviewRecord) -> VerbTruthSummary {
    let expected_sheet_kind =
        expected_sheet_kind_for(entry_review.entry_verb, entry_review.target_kind);
    let sheet_matches_verb = entry_review.review_sheet.review_sheet_kind == expected_sheet_kind;
    VerbTruthSummary {
        entry_verb: entry_review.entry_verb,
        target_kind: entry_review.target_kind,
        resulting_mode: entry_review.resulting_mode,
        source_surface: entry_review.source_surface,
        review_sheet_kind: entry_review.review_sheet.review_sheet_kind,
        verb_stays_distinct: verb_produces_distinct_sheet(entry_review.entry_verb),
        sheet_matches_verb,
    }
}

fn project_target_kind_truth(entry_review: &ProjectEntryReviewRecord) -> TargetKindTruthSummary {
    let topology_class = derive_topology_class(entry_review);
    let collision_class = entry_review
        .destination_collision_review
        .as_ref()
        .map(|review| review.collision_class)
        .unwrap_or(EntryDestinationCollisionClass::NoCollision);
    let explicit_choice_required_when_colliding = entry_review
        .destination_collision_review
        .as_ref()
        .map(|review| review.requires_explicit_choice)
        .unwrap_or(true);
    let non_durable_staging_labelled = match topology_class {
        EntryTargetTopologyClass::InspectOnlyStaging | EntryTargetTopologyClass::ImportedPacket => {
            entry_review
                .review_sheet
                .import_review
                .as_ref()
                .map(|import| import.no_durable_write_before_review)
                .unwrap_or(true)
        }
        _ => true,
    };
    let resulting_mode_is_distinct_outcome =
        resulting_mode_is_distinct(entry_review.entry_verb, entry_review.resulting_mode);
    let topology_consistent_with_verb =
        topology_class_consistent_with_verb(topology_class, entry_review.entry_verb);
    let summary = format!(
        "{} produced {} topology with collision {}",
        entry_review.entry_verb.as_str(),
        topology_class.as_str(),
        collision_class.as_str()
    );
    TargetKindTruthSummary {
        topology_class,
        destination_collision_class: collision_class,
        explicit_choice_required_when_colliding,
        non_durable_staging_labelled,
        resulting_mode_is_distinct_outcome,
        topology_consistent_with_verb,
        summary,
    }
}

fn project_durable_checkpoint(entry_review: &ProjectEntryReviewRecord) -> DurableCheckpointSummary {
    let handoff = &entry_review.post_entry_handoff_card;
    let admission_checkpoint = &entry_review.admission_checkpoint_route.checkpoint;
    let set_up_later_offered = handoff
        .safe_alternate_actions
        .iter()
        .any(|action| *action == AdmissionAction::SetUpLater);
    let open_minimal_offered = handoff
        .safe_alternate_actions
        .iter()
        .any(|action| *action == AdmissionAction::OpenMinimal);
    let cancel_offered = handoff
        .safe_alternate_actions
        .iter()
        .any(|action| *action == AdmissionAction::Cancel);
    DurableCheckpointSummary {
        admission_checkpoint_id: admission_checkpoint.admission_checkpoint_id.clone(),
        handoff_card_id: handoff.handoff_card_id.clone(),
        deferred_work: handoff.not_yet_done.clone(),
        blocking_now_tasks: handoff.blocked_tasks.clone(),
        recommended_soon_tasks: handoff.recommended_tasks.clone(),
        optional_later_tasks: handoff.optional_tasks.clone(),
        primary_next_action: handoff.primary_next_action,
        safe_alternate_actions: handoff.safe_alternate_actions.clone(),
        set_up_later_offered,
        open_minimal_offered,
        cancel_offered,
        export_or_share_state_available: handoff.export_or_share_state_available,
    }
}

fn project_side_effect_posture(entry_review: &ProjectEntryReviewRecord) -> SideEffectPosture {
    let clone = entry_review.review_sheet.clone_review.as_ref();
    let import = entry_review.review_sheet.import_review.as_ref();
    let open_workspace = entry_review.review_sheet.open_workspace_review.as_ref();

    let clone_never_grants_trust = clone.map(|c| c.clone_never_grants_trust).unwrap_or(true);
    let dependency_restore_deferred = clone.map(|c| c.dependency_restore_deferred).unwrap_or(true);
    let task_execution_deferred = clone.map(|c| c.task_execution_deferred).unwrap_or(true);
    let no_durable_write_before_review = import
        .map(|i| i.no_durable_write_before_review)
        .unwrap_or(true);
    let no_state_rehydration_before_review = import
        .map(|i| i.no_state_rehydration_before_review)
        .unwrap_or(true);
    let silent_workspace_upgrade_forbidden = open_workspace
        .map(|w| w.silent_upgrade_forbidden)
        .unwrap_or(true);
    let dropped_meaning_disclosed = open_workspace
        .map(|w| w.dropped_meaning_disclosed)
        .unwrap_or(true);

    SideEffectPosture {
        clone_never_grants_trust,
        dependency_restore_deferred,
        task_execution_deferred,
        no_durable_write_before_review,
        no_state_rehydration_before_review,
        silent_workspace_upgrade_forbidden,
        dropped_meaning_disclosed,
        deferred_work_classes: entry_review.post_entry_handoff_card.not_yet_done.clone(),
    }
}

fn project_failure_repair_truth(entry_review: &ProjectEntryReviewRecord) -> FailureRepairTruth {
    let repair = &entry_review.failure_repair_state;
    let repair_actions_present = !repair.repair_actions.is_empty();
    let source_input_redacted = !appears_to_leak_secret(&repair.source_input_label);
    FailureRepairTruth {
        typed_source_input_preserved: repair.typed_source_input_preserved,
        chosen_destination_preserved: repair.chosen_destination_preserved,
        redacted_diagnostics_preserved: repair.redacted_diagnostics_preserved,
        source_input_redacted,
        repair_actions_present,
        repair_actions: repair.repair_actions.clone(),
    }
}

fn project_surface_parity_truth(entry_review: &ProjectEntryReviewRecord) -> SurfaceParityTruth {
    let mut covered_surfaces = Vec::with_capacity(entry_review.surface_parity.len());
    let mut all_verb = true;
    let mut all_mode = true;
    let mut all_target = true;
    let mut all_same_review_model = true;
    let mut deep_link_surface_covered = false;
    let mut deep_link_intent_review_present = false;
    for parity in &entry_review.surface_parity {
        covered_surfaces.push(parity.source_surface);
        all_verb &= parity.entry_verb == entry_review.entry_verb;
        all_mode &= parity.resulting_mode == entry_review.resulting_mode;
        all_target &= parity.target_kind == entry_review.target_kind;
        all_same_review_model &= parity.same_review_model;
        if parity.source_surface == AdmissionSourceSurface::DeepLink {
            deep_link_surface_covered = true;
            if parity.review_requirement
                == EntryReviewRequirementClass::DeepLinkIntentReviewRequired
            {
                deep_link_intent_review_present = true;
            }
        }
    }
    SurfaceParityTruth {
        covered_surfaces,
        all_surfaces_preserve_verb: all_verb,
        all_surfaces_preserve_mode: all_mode,
        all_surfaces_preserve_target_kind: all_target,
        same_review_model_on_all: all_same_review_model,
        deep_link_surface_covered,
        deep_link_intent_review_present,
    }
}

// ---------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------

fn expected_sheet_kind_for(entry_verb: EntryVerb, target_kind: TargetKind) -> EntryReviewSheetKind {
    match entry_verb {
        EntryVerb::Open
            if matches!(
                target_kind,
                TargetKind::WorkspaceManifest | TargetKind::WorksetManifest
            ) =>
        {
            EntryReviewSheetKind::OpenWorkspaceManifest
        }
        EntryVerb::Open => EntryReviewSheetKind::OpenLocalTarget,
        EntryVerb::Clone => EntryReviewSheetKind::CloneRepository,
        EntryVerb::AddRoot => EntryReviewSheetKind::AddRoot,
        EntryVerb::Import | EntryVerb::StartFromSnapshot => EntryReviewSheetKind::ImportArtifact,
        EntryVerb::Restore | EntryVerb::Resume => EntryReviewSheetKind::RestoreState,
    }
}

const fn verb_produces_distinct_sheet(entry_verb: EntryVerb) -> bool {
    matches!(
        entry_verb,
        EntryVerb::Open
            | EntryVerb::Clone
            | EntryVerb::AddRoot
            | EntryVerb::Import
            | EntryVerb::Restore
            | EntryVerb::Resume
            | EntryVerb::StartFromSnapshot
    )
}

fn resulting_mode_is_distinct(entry_verb: EntryVerb, resulting_mode: ResultingMode) -> bool {
    match entry_verb {
        EntryVerb::Clone => matches!(
            resulting_mode,
            ResultingMode::CloneOnly
                | ResultingMode::CloneThenReview
                | ResultingMode::CloneThenOpen
                | ResultingMode::CloneThenAdd
        ),
        EntryVerb::Import | EntryVerb::StartFromSnapshot => matches!(
            resulting_mode,
            ResultingMode::ExtractThenReview
                | ResultingMode::CompareBeforeRestore
                | ResultingMode::ApplyToActiveWorkspace
                | ResultingMode::InspectOnly
                | ResultingMode::OpenPrebuildWithSetupActions
                | ResultingMode::OpenPrebuildMinimal
        ),
        EntryVerb::Restore | EntryVerb::Resume => matches!(
            resulting_mode,
            ResultingMode::RestoreLastSession
                | ResultingMode::RestoreFromCheckpoint
                | ResultingMode::ResumeLiveSession
                | ResultingMode::OpenPrebuildMinimal
        ),
        EntryVerb::AddRoot => matches!(
            resulting_mode,
            ResultingMode::WorkspaceWithRoots | ResultingMode::WorksetSlice
        ),
        EntryVerb::Open => matches!(
            resulting_mode,
            ResultingMode::SingleFile
                | ResultingMode::Folder
                | ResultingMode::RepoRoot
                | ResultingMode::WorkspaceCandidate
                | ResultingMode::WorkspaceWithRoots
                | ResultingMode::WorksetSlice
                | ResultingMode::InspectOnly
        ),
    }
}

fn topology_class_consistent_with_verb(
    topology_class: EntryTargetTopologyClass,
    entry_verb: EntryVerb,
) -> bool {
    match topology_class {
        EntryTargetTopologyClass::AcquiredNotFetched => entry_verb == EntryVerb::Clone,
        EntryTargetTopologyClass::OpenedSparse => {
            matches!(entry_verb, EntryVerb::Clone | EntryVerb::Open)
        }
        EntryTargetTopologyClass::PointerOnly => {
            matches!(entry_verb, EntryVerb::Clone | EntryVerb::Open)
        }
        EntryTargetTopologyClass::NestedChild | EntryTargetTopologyClass::ParentRoot => matches!(
            entry_verb,
            EntryVerb::Open | EntryVerb::AddRoot | EntryVerb::Clone
        ),
        EntryTargetTopologyClass::ImportedPacket => {
            matches!(entry_verb, EntryVerb::Import | EntryVerb::StartFromSnapshot)
        }
        EntryTargetTopologyClass::InspectOnlyStaging => matches!(
            entry_verb,
            EntryVerb::Import | EntryVerb::StartFromSnapshot | EntryVerb::Open
        ),
        EntryTargetTopologyClass::RestoreTarget => {
            matches!(entry_verb, EntryVerb::Restore | EntryVerb::Resume)
        }
        EntryTargetTopologyClass::DurableOpen => matches!(
            entry_verb,
            EntryVerb::Open | EntryVerb::AddRoot | EntryVerb::Clone
        ),
    }
}

fn derive_topology_class(entry_review: &ProjectEntryReviewRecord) -> EntryTargetTopologyClass {
    if matches!(
        entry_review.entry_verb,
        EntryVerb::Restore | EntryVerb::Resume
    ) {
        return EntryTargetTopologyClass::RestoreTarget;
    }

    if matches!(
        entry_review.entry_verb,
        EntryVerb::Import | EntryVerb::StartFromSnapshot
    ) {
        if let Some(import) = entry_review.review_sheet.import_review.as_ref() {
            if import.inspect_only {
                return EntryTargetTopologyClass::InspectOnlyStaging;
            }
            return EntryTargetTopologyClass::ImportedPacket;
        }
        return EntryTargetTopologyClass::ImportedPacket;
    }

    if entry_review.resulting_mode == ResultingMode::InspectOnly {
        return EntryTargetTopologyClass::InspectOnlyStaging;
    }

    if let Some(collision) = entry_review.destination_collision_review.as_ref() {
        match collision.collision_class {
            EntryDestinationCollisionClass::NestedRepository => {
                return EntryTargetTopologyClass::NestedChild;
            }
            EntryDestinationCollisionClass::ExistingRepoRoot
            | EntryDestinationCollisionClass::ExistingWorktree
            | EntryDestinationCollisionClass::ExistingWorkspaceManifest => {
                return EntryTargetTopologyClass::ParentRoot;
            }
            _ => {}
        }
    }

    if let Some(clone) = entry_review.review_sheet.clone_review.as_ref() {
        if clone.partial_filter_label.is_some()
            || matches!(
                clone.clone_depth_class,
                crate::entry::CloneDepthClass::PartialCloneFiltered
            )
        {
            return EntryTargetTopologyClass::OpenedSparse;
        }
        if matches!(
            clone.clone_depth_class,
            crate::entry::CloneDepthClass::ShallowDepth
        ) {
            return EntryTargetTopologyClass::OpenedSparse;
        }
        if entry_review.resulting_mode == ResultingMode::CloneOnly {
            return EntryTargetTopologyClass::AcquiredNotFetched;
        }
        if matches!(
            clone.lfs_posture,
            crate::admission::LfsPosture::NotRequested
        ) && clone.partial_filter_label.is_none()
        {
            // LFS hydration explicitly suppressed: pointer-only posture so
            // later surfaces know large files are not local yet.
            return EntryTargetTopologyClass::PointerOnly;
        }
    }

    EntryTargetTopologyClass::DurableOpen
}

fn appears_to_leak_secret(label: &str) -> bool {
    let lowered = label.to_ascii_lowercase();
    if lowered.contains("password") || lowered.contains("secret") || lowered.contains("token") {
        // Allow the explicit redaction sentinel.
        if !lowered.contains("<redacted>") {
            return true;
        }
    }
    if let Some(scheme_idx) = lowered.find("://") {
        let after_scheme = &lowered[scheme_idx + 3..];
        if let Some(at_idx) = after_scheme.find('@') {
            let credentials = &after_scheme[..at_idx];
            if !credentials.is_empty() && !credentials.contains("<redacted>") {
                return true;
            }
        }
    }
    false
}

fn build_summary(
    entry_review: &ProjectEntryReviewRecord,
    verb_truth: &VerbTruthSummary,
    target_kind_truth: &TargetKindTruthSummary,
    durable_checkpoint: &DurableCheckpointSummary,
    qualification: &EntryHardeningQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Entry hardening lineage proven Stable: {} verb produced {} topology, {} resulting mode, {} deferred-work classes.",
            verb_truth.entry_verb.as_str(),
            target_kind_truth.topology_class.as_str(),
            verb_truth.resulting_mode.as_str(),
            durable_checkpoint.deferred_work.len()
        )
    } else {
        format!(
            "Entry hardening lineage narrowed below Stable for {} ({}): {} reason(s).",
            entry_review.entry_verb.as_str(),
            entry_review.resulting_mode.as_str(),
            qualification.narrow_reasons.len()
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of an entry hardening lineage
/// record. Used by the workspace status surface, the headless CLI emitter,
/// Help/About, and support export so they all render the same lines from the
/// same record.
pub fn entry_hardening_lineage_lines(record: &EntryHardeningLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Entry hardening lineage: posture={} verb={} target={} mode={} topology={}",
        record.posture_id,
        record.verb_truth.entry_verb.as_str(),
        record.verb_truth.target_kind.as_str(),
        record.verb_truth.resulting_mode.as_str(),
        record.target_kind_truth.topology_class.as_str(),
    ));
    lines.push(format!(
        "Stable qualification: {} ({} narrow reason(s))",
        record.stable_qualification.level,
        record.stable_qualification.narrow_reasons.len()
    ));
    for reason in &record.stable_qualification.narrow_reasons {
        lines.push(format!("  - narrow: {}", reason.as_str()));
    }
    lines.push(format!(
        "Verb truth: distinct={} sheet_matches_verb={} sheet_kind={}",
        record.verb_truth.verb_stays_distinct,
        record.verb_truth.sheet_matches_verb,
        record.verb_truth.review_sheet_kind.as_str(),
    ));
    lines.push(format!(
        "Target-kind truth: collision={} explicit_choice={} non_durable_staging_labelled={}",
        record
            .target_kind_truth
            .destination_collision_class
            .as_str(),
        record
            .target_kind_truth
            .explicit_choice_required_when_colliding,
        record.target_kind_truth.non_durable_staging_labelled,
    ));
    lines.push(format!(
        "Durable checkpoint: checkpoint_id={} handoff_id={} primary_next_action={} set_up_later={} open_minimal={} cancel={}",
        record.durable_checkpoint.admission_checkpoint_id,
        record.durable_checkpoint.handoff_card_id,
        record.durable_checkpoint.primary_next_action.as_str(),
        record.durable_checkpoint.set_up_later_offered,
        record.durable_checkpoint.open_minimal_offered,
        record.durable_checkpoint.cancel_offered,
    ));
    lines.push(format!(
        "Deferred work classes: {}",
        if record.side_effect_posture.deferred_work_classes.is_empty() {
            "(none)".to_owned()
        } else {
            record
                .side_effect_posture
                .deferred_work_classes
                .iter()
                .map(|class| class.as_str().to_owned())
                .collect::<Vec<_>>()
                .join(", ")
        }
    ));
    lines.push(format!(
        "Side-effect posture: clone_never_grants_trust={} deps_deferred={} tasks_deferred={} import_no_durable_write={} import_no_rehydrate={} workspace_no_silent_upgrade={}",
        record.side_effect_posture.clone_never_grants_trust,
        record.side_effect_posture.dependency_restore_deferred,
        record.side_effect_posture.task_execution_deferred,
        record.side_effect_posture.no_durable_write_before_review,
        record.side_effect_posture.no_state_rehydration_before_review,
        record.side_effect_posture.silent_workspace_upgrade_forbidden,
    ));
    lines.push(format!(
        "Failure repair truth: inputs_preserved={} destination_preserved={} diagnostics_preserved={} input_redacted={} repair_actions={}",
        record.failure_repair_truth.typed_source_input_preserved,
        record.failure_repair_truth.chosen_destination_preserved,
        record.failure_repair_truth.redacted_diagnostics_preserved,
        record.failure_repair_truth.source_input_redacted,
        record.failure_repair_truth.repair_actions.len(),
    ));
    lines.push(format!(
        "Surface parity: covered={} preserve_verb={} preserve_mode={} preserve_target={} same_review_model={} deep_link_intent_review_present={}",
        record.surface_parity_truth.covered_surfaces.len(),
        record.surface_parity_truth.all_surfaces_preserve_verb,
        record.surface_parity_truth.all_surfaces_preserve_mode,
        record.surface_parity_truth.all_surfaces_preserve_target_kind,
        record.surface_parity_truth.same_review_model_on_all,
        record.surface_parity_truth.deep_link_intent_review_present,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  - {} [{}] available={} action_id={}",
            hook.label,
            hook.hook_class.as_str(),
            hook.available,
            hook.action_id,
        ));
    }
    lines.push(format!("Summary: {}", record.summary));
    lines
}

#[cfg(test)]
mod tests;
