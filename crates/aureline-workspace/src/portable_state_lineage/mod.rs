//! Portable-state lineage: the governed, export-safe projection that
//! finalizes the portable-state export/import package, the remembered-state
//! inspector, and the restore-provenance card into one record per posture.
//!
//! The projection ingests a live [`PortableStateAlphaPackage`] verbatim. The
//! state-packages module owns the heavy lifting (state-class separation,
//! redaction manifest, machine-local exclusions, topology adjustments,
//! placeholder cards). This module promotes the validated package into a
//! stable-line lineage artifact that proves the four claims the
//! portable-state lane is anchored on:
//!
//! - **State-class separation truth.** Workspace authority, window topology,
//!   profile defaults, and machine-local hints are all present, each
//!   classified honestly (`local_only`, `portable`, `shared`, `machine_local`),
//!   and the machine-local class is not exported as authority.
//! - **Restore-provenance truth.** A controlled restore-fidelity class
//!   (`Exact restore`, `Compatible restore`, `Layout only`, `Recovered drafts`,
//!   or `Evidence only`) is named for the package, topology adjustments
//!   verify visible bounds and preserve pane-id provenance, and the placeholder
//!   summary covers live / context-only / placeholder-only postures.
//! - **Exclusion / redaction honesty.** The redaction manifest names every
//!   required rule (raw secret material, approval ticket, delegated
//!   credential, live authority handle, machine-unique handle, state root),
//!   the machine-local exclusion catalogue names the required reasons
//!   (contains live handle, display-hint best-effort only, state-root only,
//!   credential-store only), and `machine_local_exclusions_reviewed` is
//!   true before any export commits.
//! - **No-rerun honesty.** Context-only and placeholder-only panes carry
//!   `ExplicitUserActionRequired` and `PlaceholderPreserved` guardrails so
//!   restore never silently reruns a terminal command, task, debugger,
//!   notebook kernel, preview server, or remote session.
//!
//! In addition the record carries the package producer ref, schema version,
//! and integrity hash so import / replay surfaces can pin the source
//! producer before applying. When the projection cannot prove a claim on the
//! captured posture it auto-narrows below Stable with a named
//! [`PortableStateLineageNarrowReason`]. The record carries no raw source
//! bytes (`raw_payload_excluded = true`) and is safe for support export.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::state_packages::{
    MachineLocalExclusionReason, NoRerunGuardrail, PaneRestorePosture, PersistenceClassification,
    PortableStateAlphaPackage, PortableStateAlphaValidationError, RedactionRuleClass,
    RememberedStateActionKind, RememberedStateInspector, RestoreCandidateClass,
    SerializedStateClass, SurfaceRestorePosture,
};

/// Schema version for [`PortableStateLineageRecord`].
pub const PORTABLE_STATE_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the portable-state lineage record.
pub const PORTABLE_STATE_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/portable_state_lineage.schema.json";

/// Stable record-kind tag for the portable-state lineage record.
pub const PORTABLE_STATE_LINEAGE_RECORD_KIND: &str = "portable_state_lineage_record";

// ---------------------------------------------------------------------------
// Restore fidelity class.
// ---------------------------------------------------------------------------

/// Controlled restore-fidelity class the record claims for the package as a
/// whole, derived from the per-class restore candidates and the placeholder
/// summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelityClass {
    /// Every required portable class restores exactly; no placeholders.
    Exact,
    /// Restore is honest but downgrades one or more classes through a named
    /// translation or fallback (for example a DPI re-bucketing).
    Compatible,
    /// Restore reopens window topology only; live surfaces stay placeholders
    /// or context-only.
    LayoutOnly,
    /// Restore recovers a drafted buffer or local-session context that the
    /// previous session had not durably saved.
    RecoveredDrafts,
    /// Restore can only show evidence; live capability is not available
    /// (missing extension, missing remote, policy-blocked surface).
    EvidenceOnly,
}

impl RestoreFidelityClass {
    /// Returns the stable snake_case token for this fidelity class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::LayoutOnly => "layout_only",
            Self::RecoveredDrafts => "recovered_drafts",
            Self::EvidenceOnly => "evidence_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Inspection hooks.
// ---------------------------------------------------------------------------

/// Class of pre-destructive inspection / repair hook available before the
/// portable-state package is exported, imported, applied, or cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableStateInspectionHookClass {
    /// Open the remembered-state inspector for the package.
    InspectInspector,
    /// Open the export review sheet (state-class table, redaction manifest).
    InspectExportReview,
    /// Open the restore-provenance card (fidelity class + adjustments).
    InspectRestoreProvenance,
    /// Compare current state with the package before applying.
    CompareBeforeApply,
    /// Create a one-step rollback checkpoint before applying.
    RollbackCheckpoint,
    /// Export the lineage record (support-safe, no raw bytes).
    Export,
    /// Clear remembered-state classes selectively without deleting unrelated
    /// workspace or profile state.
    Clear,
}

impl PortableStateInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectInspector => "inspect_inspector",
            Self::InspectExportReview => "inspect_export_review",
            Self::InspectRestoreProvenance => "inspect_restore_provenance",
            Self::CompareBeforeApply => "compare_before_apply",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::Export => "export",
            Self::Clear => "clear",
        }
    }
}

/// One pre-destructive inspection / repair hook offered before the
/// portable-state package commits to a destructive action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateInspectionHook {
    /// Hook class.
    pub hook_class: PortableStateInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable for this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-destructive inspection / repair hook table.
pub fn default_portable_state_inspection_hooks() -> Vec<PortableStateInspectionHook> {
    vec![
        PortableStateInspectionHook {
            hook_class: PortableStateInspectionHookClass::InspectInspector,
            action_id: "portable_state.inspect_inspector".to_owned(),
            label: "Inspect remembered state".to_owned(),
            available: true,
            disclosure:
                "Opens the remembered-state inspector with every state-class row, classification, schema version, and last-write timestamp."
                    .to_owned(),
        },
        PortableStateInspectionHook {
            hook_class: PortableStateInspectionHookClass::InspectExportReview,
            action_id: "portable_state.inspect_export_review".to_owned(),
            label: "Inspect export review".to_owned(),
            available: true,
            disclosure:
                "Opens the export review sheet showing which state classes are portable, local-only, shared, or redacted."
                    .to_owned(),
        },
        PortableStateInspectionHook {
            hook_class: PortableStateInspectionHookClass::InspectRestoreProvenance,
            action_id: "portable_state.inspect_restore_provenance".to_owned(),
            label: "Inspect restore provenance".to_owned(),
            available: true,
            disclosure:
                "Opens the restore-provenance card with the controlled fidelity class, topology adjustments, and placeholder summary."
                    .to_owned(),
        },
        PortableStateInspectionHook {
            hook_class: PortableStateInspectionHookClass::CompareBeforeApply,
            action_id: "portable_state.compare_before_apply".to_owned(),
            label: "Compare before apply".to_owned(),
            available: true,
            disclosure:
                "Produces a reviewable structured diff between current state and the package before any apply."
                    .to_owned(),
        },
        PortableStateInspectionHook {
            hook_class: PortableStateInspectionHookClass::RollbackCheckpoint,
            action_id: "portable_state.rollback_checkpoint".to_owned(),
            label: "Create rollback checkpoint".to_owned(),
            available: true,
            disclosure:
                "Captures a one-step rollback checkpoint before applying the package so any apply can be undone."
                    .to_owned(),
        },
        PortableStateInspectionHook {
            hook_class: PortableStateInspectionHookClass::Export,
            action_id: "portable_state.export".to_owned(),
            label: "Export portable-state lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this portable-state lineage record for support without raw payload bytes."
                    .to_owned(),
        },
        PortableStateInspectionHook {
            hook_class: PortableStateInspectionHookClass::Clear,
            action_id: "portable_state.clear".to_owned(),
            label: "Clear remembered state".to_owned(),
            available: true,
            disclosure:
                "Clears the chosen remembered-state class without deleting unrelated workspace or profile state."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a portable-state lineage record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableStateLineageNarrowReason {
    /// The package failed its own validator (state-class separation broke).
    PackageValidatorFailed,
    /// Workspace authority, window topology, profile defaults, or machine-local
    /// hints are missing from the package body.
    StateClassesIncomplete,
    /// Machine-local hints are classified as portable or shared instead of
    /// machine-local.
    MachineLocalHintsMisclassified,
    /// A required redaction rule is missing (raw secret / approval / delegated
    /// credential / live authority handle / machine-unique handle / state root).
    RedactionRuleMissing,
    /// Machine-local exclusions were not reviewed before export.
    MachineLocalExclusionsNotReviewed,
    /// A required machine-local exclusion reason is missing.
    MachineLocalExclusionReasonMissing,
    /// A topology adjustment changed displays without verifying visible bounds.
    TopologyAdjustmentUnverified,
    /// A topology adjustment lost pane-id provenance.
    TopologyAdjustmentLostPaneIds,
    /// A non-live pane is missing the explicit-user-action / placeholder-preserved
    /// no-rerun guardrails.
    PlaceholderMissingNoRerunGuardrail,
    /// A required pre-destructive inspection hook is unavailable.
    InspectionHookUnavailable,
    /// Producer attribution (producer ref / schema version / integrity hash)
    /// is incomplete.
    ProducerAttributionIncomplete,
    /// Package or workspace ref is empty (would break support export).
    LineageExportUnsafe,
}

impl PortableStateLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageValidatorFailed => "package_validator_failed",
            Self::StateClassesIncomplete => "state_classes_incomplete",
            Self::MachineLocalHintsMisclassified => "machine_local_hints_misclassified",
            Self::RedactionRuleMissing => "redaction_rule_missing",
            Self::MachineLocalExclusionsNotReviewed => "machine_local_exclusions_not_reviewed",
            Self::MachineLocalExclusionReasonMissing => "machine_local_exclusion_reason_missing",
            Self::TopologyAdjustmentUnverified => "topology_adjustment_unverified",
            Self::TopologyAdjustmentLostPaneIds => "topology_adjustment_lost_pane_ids",
            Self::PlaceholderMissingNoRerunGuardrail => "placeholder_missing_no_rerun_guardrail",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a portable-state lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not qualified.
    pub narrow_reasons: Vec<PortableStateLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One state-class row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateLineageClassRow {
    /// Stable class row id.
    pub class_id: String,
    /// State class kind.
    pub class_kind: SerializedStateClass,
    /// Persistence classification.
    pub classification: PersistenceClassification,
    /// Restore candidate class for the row.
    pub restore_candidate: RestoreCandidateClass,
    /// Whether the row is included in the export body.
    pub export_allowed: bool,
    /// Whether the row may be cleared selectively.
    pub clear_allowed: bool,
    /// Pane count (for window-topology and local-session-context rows).
    pub pane_count: usize,
    /// Linked profile artifact refs (for profile-defaults rows).
    pub linked_profile_artifact_refs: Vec<String>,
    /// Schema version pinned by the row.
    pub schema_version: u32,
}

/// State-class separation truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateClassSeparationSummary {
    /// All separated state classes carried by the package.
    pub class_rows: Vec<PortableStateLineageClassRow>,
    /// True when workspace authority is present.
    pub workspace_authority_present: bool,
    /// True when window topology is present.
    pub window_topology_present: bool,
    /// True when profile defaults are present.
    pub profile_defaults_present: bool,
    /// True when machine-local hints are present.
    pub machine_local_hints_present: bool,
    /// True when machine-local hints are classified as machine-local (not
    /// portable / shared / local-only).
    pub machine_local_hints_classified_correctly: bool,
    /// True when machine-local hints are excluded from carried-body export.
    pub machine_local_hints_not_exported_as_body: bool,
}

/// Restore-provenance truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenanceSummary {
    /// Controlled restore-fidelity class for the whole package.
    pub restore_fidelity_class: RestoreFidelityClass,
    /// Source topology snapshot refs.
    pub source_snapshot_refs: Vec<String>,
    /// Restore provenance record refs.
    pub restore_provenance_refs: Vec<String>,
    /// Total live pane postures.
    pub live_pane_count: usize,
    /// Total context-only pane postures.
    pub context_only_pane_count: usize,
    /// Total placeholder-only pane postures.
    pub placeholder_only_pane_count: usize,
    /// True when at least one topology adjustment is recorded.
    pub topology_adjustments_recorded: bool,
    /// True when every topology adjustment verified visible bounds.
    pub all_adjustments_visible_bounds_verified: bool,
    /// True when every topology adjustment preserved pane-id provenance.
    pub all_adjustments_preserve_pane_ids: bool,
    /// True when the placeholder summary covers live / context-only /
    /// placeholder-only postures (so missing extensions, missing remotes, and
    /// non-reentrant live surfaces preserve spatial context).
    pub placeholder_summary_covers_required_postures: bool,
}

/// Exclusion / redaction honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExclusionHonestySummary {
    /// All redaction rules attached to the package.
    pub redaction_rules: Vec<RedactionRuleClass>,
    /// True when every required redaction rule is present.
    pub redaction_rules_complete: bool,
    /// True when machine-local exclusions were reviewed before export.
    pub machine_local_exclusions_reviewed: bool,
    /// All machine-local exclusion reasons named by the package.
    pub machine_local_exclusion_reasons: Vec<MachineLocalExclusionReason>,
    /// True when every required machine-local exclusion reason is present.
    pub machine_local_exclusion_reasons_complete: bool,
    /// Number of machine-local exclusion rows.
    pub machine_local_exclusion_count: usize,
    /// True when raw paths and raw hosts are explicitly excluded.
    pub paths_and_hosts_excluded: bool,
}

/// No-rerun honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoRerunHonestySummary {
    /// True when every non-live pane carries the explicit-user-action and
    /// placeholder-preserved guardrails.
    pub all_non_live_panes_no_rerun_guarded: bool,
    /// True when every placeholder-only pane offers safe actions.
    pub all_placeholder_panes_offer_safe_actions: bool,
    /// True when remembered-state actions cover inspect, export, compare,
    /// and clear so support and crash-recovery flows can reach all four.
    pub remembered_state_actions_complete: bool,
}

/// Producer attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducerAttributionSummary {
    /// Opaque producer build / instance ref.
    pub producer_ref: String,
    /// Package schema version.
    pub schema_version: u32,
    /// Opaque package integrity hash (stable across reserialization).
    pub integrity_hash: String,
    /// Package creation timestamp.
    pub created_at: String,
    /// True when producer attribution fields are non-empty.
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe portable-state lineage record per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub portable_state_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Opaque ref to the source portable-state package.
    pub package_ref: String,
    /// Opaque ref to the package manifest.
    pub manifest_ref: String,
    /// Workspace ref the package describes.
    pub workspace_ref: String,
    /// Producer attribution pillar.
    pub producer_attribution: ProducerAttributionSummary,
    /// State-class separation pillar.
    pub state_class_separation: StateClassSeparationSummary,
    /// Restore-provenance pillar.
    pub restore_provenance: RestoreProvenanceSummary,
    /// Exclusion / redaction pillar.
    pub exclusion_honesty: ExclusionHonestySummary,
    /// No-rerun honesty pillar.
    pub no_rerun_honesty: NoRerunHonestySummary,
    /// Remembered-state inspector projection.
    pub remembered_state_inspector: RememberedStateInspector,
    /// Pre-destructive inspection / repair hooks.
    pub inspection_hooks: Vec<PortableStateInspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: PortableStateLineageQualification,
    /// Whether the record is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl PortableStateLineageRecord {
    /// Returns true when the record is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == PORTABLE_STATE_LINEAGE_SCHEMA_REF
            && self.record_kind == PORTABLE_STATE_LINEAGE_RECORD_KIND
            && !self.package_ref.trim().is_empty()
            && !self.manifest_ref.trim().is_empty()
            && !self.workspace_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(
        &self,
        class: PortableStateInspectionHookClass,
    ) -> Option<&PortableStateInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed portable-state lineage record from a live
/// [`PortableStateAlphaPackage`] using the default inspection-hook set.
pub fn project_portable_state_lineage(
    posture_id: impl Into<String>,
    package: &PortableStateAlphaPackage,
) -> PortableStateLineageRecord {
    project_portable_state_lineage_with_hooks(
        posture_id,
        package,
        default_portable_state_inspection_hooks(),
    )
}

/// Like [`project_portable_state_lineage`] but with an explicit
/// inspection-hook set (for testing degraded-hook postures).
pub fn project_portable_state_lineage_with_hooks(
    posture_id: impl Into<String>,
    package: &PortableStateAlphaPackage,
    inspection_hooks: Vec<PortableStateInspectionHook>,
) -> PortableStateLineageRecord {
    let posture_id: String = posture_id.into();
    let package_validation = package.validate();

    let state_class_separation = project_state_class_separation(package);
    let restore_provenance = project_restore_provenance(package);
    let exclusion_honesty = project_exclusion_honesty(package);
    let no_rerun_honesty = project_no_rerun_honesty(package);
    let producer_attribution = project_producer_attribution(package);

    let remembered_state_inspector = match package.inspector() {
        Ok(inspector) => inspector,
        Err(_) => RememberedStateInspector {
            package_id: package.package_id.clone(),
            schema_version: package.schema_version,
            rows: Vec::new(),
        },
    };

    let mut narrow_reasons = Vec::new();

    if package_validation.is_err() {
        narrow_reasons.push(PortableStateLineageNarrowReason::PackageValidatorFailed);
    }

    if !(state_class_separation.workspace_authority_present
        && state_class_separation.window_topology_present
        && state_class_separation.profile_defaults_present
        && state_class_separation.machine_local_hints_present)
    {
        narrow_reasons.push(PortableStateLineageNarrowReason::StateClassesIncomplete);
    }
    if !(state_class_separation.machine_local_hints_classified_correctly
        && state_class_separation.machine_local_hints_not_exported_as_body)
    {
        narrow_reasons.push(PortableStateLineageNarrowReason::MachineLocalHintsMisclassified);
    }

    if !exclusion_honesty.redaction_rules_complete {
        narrow_reasons.push(PortableStateLineageNarrowReason::RedactionRuleMissing);
    }
    if !exclusion_honesty.machine_local_exclusions_reviewed {
        narrow_reasons.push(PortableStateLineageNarrowReason::MachineLocalExclusionsNotReviewed);
    }
    if !exclusion_honesty.machine_local_exclusion_reasons_complete {
        narrow_reasons
            .push(PortableStateLineageNarrowReason::MachineLocalExclusionReasonMissing);
    }

    if restore_provenance.topology_adjustments_recorded
        && !restore_provenance.all_adjustments_visible_bounds_verified
    {
        narrow_reasons.push(PortableStateLineageNarrowReason::TopologyAdjustmentUnverified);
    }
    if restore_provenance.topology_adjustments_recorded
        && !restore_provenance.all_adjustments_preserve_pane_ids
    {
        narrow_reasons.push(PortableStateLineageNarrowReason::TopologyAdjustmentLostPaneIds);
    }

    if !no_rerun_honesty.all_non_live_panes_no_rerun_guarded {
        narrow_reasons.push(PortableStateLineageNarrowReason::PlaceholderMissingNoRerunGuardrail);
    }

    let required_hooks = [
        PortableStateInspectionHookClass::InspectInspector,
        PortableStateInspectionHookClass::InspectExportReview,
        PortableStateInspectionHookClass::InspectRestoreProvenance,
        PortableStateInspectionHookClass::CompareBeforeApply,
        PortableStateInspectionHookClass::RollbackCheckpoint,
        PortableStateInspectionHookClass::Export,
        PortableStateInspectionHookClass::Clear,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(PortableStateLineageNarrowReason::InspectionHookUnavailable);
    }

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons.push(PortableStateLineageNarrowReason::ProducerAttributionIncomplete);
    }

    let workspace_ref = package.workspace_ref.clone();
    if package.package_id.trim().is_empty()
        || package.manifest_id.trim().is_empty()
        || workspace_ref.trim().is_empty()
    {
        narrow_reasons.push(PortableStateLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = PortableStateLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &state_class_separation,
        &restore_provenance,
        &exclusion_honesty,
        &stable_qualification,
    );

    PortableStateLineageRecord {
        record_kind: PORTABLE_STATE_LINEAGE_RECORD_KIND.to_owned(),
        portable_state_lineage_schema_version: PORTABLE_STATE_LINEAGE_SCHEMA_VERSION,
        schema_ref: PORTABLE_STATE_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        package_ref: format!("portable_state_package:{}", package.package_id),
        manifest_ref: format!("portable_state_manifest:{}", package.manifest_id),
        workspace_ref,
        producer_attribution,
        state_class_separation,
        restore_provenance,
        exclusion_honesty,
        no_rerun_honesty,
        remembered_state_inspector,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_state_class_separation(
    package: &PortableStateAlphaPackage,
) -> StateClassSeparationSummary {
    let class_rows: Vec<PortableStateLineageClassRow> = package
        .state_classes
        .iter()
        .map(|row| PortableStateLineageClassRow {
            class_id: row.class_id.clone(),
            class_kind: row.class_kind,
            classification: row.classification,
            restore_candidate: row.restore_candidate,
            export_allowed: row.export_allowed,
            clear_allowed: row.clear_allowed,
            pane_count: row.pane_restore_postures.len(),
            linked_profile_artifact_refs: row.linked_profile_artifact_refs.clone(),
            schema_version: row.schema_binding.schema_version,
        })
        .collect();

    let observed_classes: BTreeSet<_> = package
        .state_classes
        .iter()
        .map(|row| row.class_kind)
        .collect();

    let machine_local_row = package
        .state_classes
        .iter()
        .find(|row| row.class_kind == SerializedStateClass::MachineLocalHints);
    let machine_local_hints_classified_correctly = machine_local_row
        .map(|row| row.classification == PersistenceClassification::MachineLocal)
        .unwrap_or(false);
    let machine_local_hints_not_exported_as_body = machine_local_row
        .map(|row| {
            !row.export_allowed
                && !matches!(
                    row.export_mode,
                    crate::state_packages::ExportMode::CarriedBody
                )
        })
        .unwrap_or(false);

    StateClassSeparationSummary {
        class_rows,
        workspace_authority_present: observed_classes
            .contains(&SerializedStateClass::WorkspaceAuthority),
        window_topology_present: observed_classes.contains(&SerializedStateClass::WindowTopology),
        profile_defaults_present: observed_classes.contains(&SerializedStateClass::ProfileDefaults),
        machine_local_hints_present: observed_classes
            .contains(&SerializedStateClass::MachineLocalHints),
        machine_local_hints_classified_correctly,
        machine_local_hints_not_exported_as_body,
    }
}

fn project_restore_provenance(package: &PortableStateAlphaPackage) -> RestoreProvenanceSummary {
    let mut live = 0usize;
    let mut context_only = 0usize;
    let mut placeholder_only = 0usize;
    for state_class in &package.state_classes {
        for pane in &state_class.pane_restore_postures {
            match pane.restore_posture {
                SurfaceRestorePosture::Live => live += 1,
                SurfaceRestorePosture::ContextOnly => context_only += 1,
                SurfaceRestorePosture::PlaceholderOnly => placeholder_only += 1,
                SurfaceRestorePosture::Excluded => {}
            }
        }
    }

    let adjustments = &package.restore_provenance.topology_adjustments;
    let topology_adjustments_recorded = !adjustments.is_empty();
    let all_adjustments_visible_bounds_verified = adjustments
        .iter()
        .all(|adj| !adj.display_topology_changed || adj.visible_bounds_verified);
    let all_adjustments_preserve_pane_ids = adjustments
        .iter()
        .all(|adj| !adj.display_topology_changed || !adj.affected_pane_ids.is_empty());

    let placeholder_postures: BTreeSet<_> = package
        .restore_provenance
        .placeholder_summary
        .iter()
        .map(|pane| pane.restore_posture)
        .collect();
    // The placeholder summary must, at minimum, cover context-only and
    // placeholder-only postures so missing extensions / non-reentrant live
    // surfaces preserve spatial context.
    let placeholder_summary_covers_required_postures = placeholder_postures
        .contains(&SurfaceRestorePosture::ContextOnly)
        && placeholder_postures.contains(&SurfaceRestorePosture::PlaceholderOnly);

    let restore_fidelity_class = derive_restore_fidelity_class(package);

    RestoreProvenanceSummary {
        restore_fidelity_class,
        source_snapshot_refs: package.restore_provenance.source_snapshot_refs.clone(),
        restore_provenance_refs: package.restore_provenance.restore_provenance_refs.clone(),
        live_pane_count: live,
        context_only_pane_count: context_only,
        placeholder_only_pane_count: placeholder_only,
        topology_adjustments_recorded,
        all_adjustments_visible_bounds_verified,
        all_adjustments_preserve_pane_ids,
        placeholder_summary_covers_required_postures,
    }
}

fn project_exclusion_honesty(package: &PortableStateAlphaPackage) -> ExclusionHonestySummary {
    let redaction_rules = package.redaction_manifest.rules.clone();
    let rules_set: BTreeSet<_> = redaction_rules.iter().copied().collect();
    let required_rules = [
        RedactionRuleClass::RawSecretMaterialExcluded,
        RedactionRuleClass::ApprovalTicketExcluded,
        RedactionRuleClass::DelegatedCredentialExcluded,
        RedactionRuleClass::LiveAuthorityHandleExcluded,
        RedactionRuleClass::MachineUniqueHandleExcluded,
        RedactionRuleClass::StateRootExcluded,
    ];
    let redaction_rules_complete = required_rules
        .iter()
        .all(|required| rules_set.contains(required));

    let paths_and_hosts_excluded = rules_set.contains(&RedactionRuleClass::RawPathExcluded)
        && rules_set.contains(&RedactionRuleClass::RawHostExcluded);

    let machine_local_exclusion_reasons: Vec<MachineLocalExclusionReason> = package
        .machine_local_exclusions
        .iter()
        .map(|row| row.reason)
        .collect();
    let reasons_set: BTreeSet<_> = machine_local_exclusion_reasons.iter().copied().collect();
    let required_reasons = [
        MachineLocalExclusionReason::ContainsLiveHandle,
        MachineLocalExclusionReason::DisplayHintBestEffortOnly,
        MachineLocalExclusionReason::StateRootOnly,
        MachineLocalExclusionReason::CredentialStoreOnly,
    ];
    let machine_local_exclusion_reasons_complete = required_reasons
        .iter()
        .all(|required| reasons_set.contains(required));

    ExclusionHonestySummary {
        redaction_rules,
        redaction_rules_complete,
        machine_local_exclusions_reviewed: package
            .redaction_manifest
            .machine_local_exclusions_reviewed,
        machine_local_exclusion_reasons,
        machine_local_exclusion_reasons_complete,
        machine_local_exclusion_count: package.machine_local_exclusions.len(),
        paths_and_hosts_excluded,
    }
}

fn project_no_rerun_honesty(package: &PortableStateAlphaPackage) -> NoRerunHonestySummary {
    let mut all_non_live_panes_no_rerun_guarded = true;
    let mut all_placeholder_panes_offer_safe_actions = true;
    for state_class in &package.state_classes {
        for pane in &state_class.pane_restore_postures {
            check_pane_guardrails(
                pane,
                &mut all_non_live_panes_no_rerun_guarded,
                &mut all_placeholder_panes_offer_safe_actions,
            );
        }
    }
    for pane in &package.restore_provenance.placeholder_summary {
        check_pane_guardrails(
            pane,
            &mut all_non_live_panes_no_rerun_guarded,
            &mut all_placeholder_panes_offer_safe_actions,
        );
    }

    let action_kinds: BTreeSet<_> = package
        .actions
        .iter()
        .map(|action| action.action)
        .collect();
    let remembered_state_actions_complete = [
        RememberedStateActionKind::Inspect,
        RememberedStateActionKind::Export,
        RememberedStateActionKind::Compare,
        RememberedStateActionKind::Clear,
    ]
    .iter()
    .all(|required| action_kinds.contains(required));

    NoRerunHonestySummary {
        all_non_live_panes_no_rerun_guarded,
        all_placeholder_panes_offer_safe_actions,
        remembered_state_actions_complete,
    }
}

fn check_pane_guardrails(
    pane: &PaneRestorePosture,
    no_rerun_ok: &mut bool,
    safe_actions_ok: &mut bool,
) {
    if pane.restore_posture.requires_manual_action() {
        let guardrails: BTreeSet<_> = pane.no_rerun_guardrails.iter().copied().collect();
        if !guardrails.contains(&NoRerunGuardrail::ExplicitUserActionRequired)
            || !guardrails.contains(&NoRerunGuardrail::PlaceholderPreserved)
        {
            *no_rerun_ok = false;
        }
    }
    if pane.restore_posture == SurfaceRestorePosture::PlaceholderOnly {
        match &pane.placeholder_card {
            Some(card) if !card.safe_actions.is_empty() => {}
            _ => *safe_actions_ok = false,
        }
    }
}

fn project_producer_attribution(
    package: &PortableStateAlphaPackage,
) -> ProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(package);
    let producer_attribution_complete = !package.producer_ref.trim().is_empty()
        && !package.created_at.trim().is_empty()
        && package.schema_version == crate::state_packages::PORTABLE_STATE_ALPHA_SCHEMA_VERSION;
    ProducerAttributionSummary {
        producer_ref: package.producer_ref.clone(),
        schema_version: package.schema_version,
        integrity_hash,
        created_at: package.created_at.clone(),
        producer_attribution_complete,
    }
}

fn derive_restore_fidelity_class(package: &PortableStateAlphaPackage) -> RestoreFidelityClass {
    let mut any_exact = false;
    let mut any_compatible = false;
    let mut any_layout_only = false;
    let mut all_excluded_or_layout_with_placeholders = true;
    let mut any_recovered_drafts = false;

    for row in &package.state_classes {
        if !matches!(row.class_kind, SerializedStateClass::MachineLocalHints) {
            match row.restore_candidate {
                RestoreCandidateClass::ExactRestore => any_exact = true,
                RestoreCandidateClass::CompatibleRestore => any_compatible = true,
                RestoreCandidateClass::LayoutOnly => any_layout_only = true,
                RestoreCandidateClass::Excluded => {}
            }
            if !matches!(
                row.restore_candidate,
                RestoreCandidateClass::Excluded | RestoreCandidateClass::LayoutOnly
            ) {
                all_excluded_or_layout_with_placeholders = false;
            }
        }
        if row.class_kind == SerializedStateClass::LocalSessionContext
            && row
                .pane_restore_postures
                .iter()
                .any(|pane| pane.placeholder_card.is_some())
        {
            // A local-session-context with a placeholder card carrying
            // last_known_label is the projection's recovered-drafts signal.
            any_recovered_drafts = true;
        }
    }

    let only_placeholder_panes = package
        .state_classes
        .iter()
        .flat_map(|row| row.pane_restore_postures.iter())
        .all(|pane| {
            matches!(
                pane.restore_posture,
                SurfaceRestorePosture::PlaceholderOnly | SurfaceRestorePosture::Excluded
            )
        });

    if only_placeholder_panes && !any_exact && !any_compatible {
        return RestoreFidelityClass::EvidenceOnly;
    }
    if all_excluded_or_layout_with_placeholders && (any_layout_only || only_placeholder_panes) {
        if any_recovered_drafts {
            return RestoreFidelityClass::RecoveredDrafts;
        }
        return RestoreFidelityClass::LayoutOnly;
    }
    if any_compatible || any_layout_only {
        return RestoreFidelityClass::Compatible;
    }
    if any_exact {
        return RestoreFidelityClass::Exact;
    }
    RestoreFidelityClass::EvidenceOnly
}

fn compute_integrity_hash(package: &PortableStateAlphaPackage) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let inputs = [
        package.package_id.as_str(),
        package.manifest_id.as_str(),
        package.workspace_ref.as_str(),
        package.producer_ref.as_str(),
        package.created_at.as_str(),
    ];
    for input in inputs {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for row in &package.state_classes {
        for part in [row.class_id.as_str(), row.last_written_at.as_str()] {
            for byte in part.as_bytes() {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x100000001b3);
            }
            hash ^= 0xfe;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("psl:{hash:016x}")
}

fn hook_available(
    hooks: &[PortableStateInspectionHook],
    class: PortableStateInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    state_class_separation: &StateClassSeparationSummary,
    restore_provenance: &RestoreProvenanceSummary,
    exclusion_honesty: &ExclusionHonestySummary,
    qualification: &PortableStateLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Portable-state lineage proven Stable: {classes} state classes, restore_fidelity_class={fidelity}, panes(live/context/placeholder)={live}/{context}/{placeholder}, exclusions={exclusions}.",
            classes = state_class_separation.class_rows.len(),
            fidelity = restore_provenance.restore_fidelity_class.as_str(),
            live = restore_provenance.live_pane_count,
            context = restore_provenance.context_only_pane_count,
            placeholder = restore_provenance.placeholder_only_pane_count,
            exclusions = exclusion_honesty.machine_local_exclusion_count,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Portable-state lineage narrowed below Stable ({fidelity}): {reasons}.",
            fidelity = restore_provenance.restore_fidelity_class.as_str(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a portable-state lineage record.
/// The same projection is consumed by the workspace portable-state status
/// surface, the headless CLI emitter, Help/About, and support export.
pub fn portable_state_lineage_lines(record: &PortableStateLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Portable-state lineage — {} ({})",
        record.restore_provenance.restore_fidelity_class.as_str(),
        record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} package={} manifest={} posture={}",
        record.workspace_ref, record.package_ref, record.manifest_ref, record.posture_id
    ));
    lines.push(format!(
        "producer={} schema_version={} integrity_hash={} created_at={}",
        record.producer_attribution.producer_ref,
        record.producer_attribution.schema_version,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.created_at,
    ));
    lines.push(format!(
        "state_classes={} workspace_authority={} window_topology={} profile_defaults={} machine_local_hints={} machine_local_hints_machine_local={}",
        record.state_class_separation.class_rows.len(),
        record.state_class_separation.workspace_authority_present,
        record.state_class_separation.window_topology_present,
        record.state_class_separation.profile_defaults_present,
        record.state_class_separation.machine_local_hints_present,
        record
            .state_class_separation
            .machine_local_hints_classified_correctly,
    ));
    lines.push("State class rows:".to_owned());
    for row in &record.state_class_separation.class_rows {
        lines.push(format!(
            "  - {kind} {id} classification={class} restore_candidate={candidate} export_allowed={export} clear_allowed={clear} panes={panes} linked_profile_artifacts={artifacts}",
            kind = row.class_kind.as_str(),
            id = row.class_id,
            class = row.classification.as_str(),
            candidate = row.restore_candidate.as_str(),
            export = row.export_allowed,
            clear = row.clear_allowed,
            panes = row.pane_count,
            artifacts = row.linked_profile_artifact_refs.len(),
        ));
    }
    lines.push(format!(
        "Restore provenance: fidelity={} live={} context_only={} placeholder_only={} adjustments_recorded={} adjustments_visible_bounds_verified={} adjustments_preserve_pane_ids={} placeholder_summary_covers_required={}",
        record.restore_provenance.restore_fidelity_class.as_str(),
        record.restore_provenance.live_pane_count,
        record.restore_provenance.context_only_pane_count,
        record.restore_provenance.placeholder_only_pane_count,
        record.restore_provenance.topology_adjustments_recorded,
        record.restore_provenance.all_adjustments_visible_bounds_verified,
        record.restore_provenance.all_adjustments_preserve_pane_ids,
        record
            .restore_provenance
            .placeholder_summary_covers_required_postures,
    ));
    lines.push(format!(
        "Exclusion honesty: redaction_rules_complete={} machine_local_exclusions_reviewed={} machine_local_exclusion_reasons_complete={} machine_local_exclusion_count={} paths_and_hosts_excluded={}",
        record.exclusion_honesty.redaction_rules_complete,
        record.exclusion_honesty.machine_local_exclusions_reviewed,
        record.exclusion_honesty.machine_local_exclusion_reasons_complete,
        record.exclusion_honesty.machine_local_exclusion_count,
        record.exclusion_honesty.paths_and_hosts_excluded,
    ));
    lines.push(format!(
        "No-rerun honesty: non_live_panes_guarded={} placeholder_panes_have_safe_actions={} remembered_state_actions_complete={}",
        record.no_rerun_honesty.all_non_live_panes_no_rerun_guarded,
        record
            .no_rerun_honesty
            .all_placeholder_panes_offer_safe_actions,
        record.no_rerun_honesty.remembered_state_actions_complete,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }
    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }
    lines.push(record.summary.clone());
    lines
}

#[allow(dead_code)]
fn _validator_error_pulls_in(_: PortableStateAlphaValidationError) {}

#[cfg(test)]
mod tests;
