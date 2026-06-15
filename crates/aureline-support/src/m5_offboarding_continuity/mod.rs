//! Offboarding-safe cleanup plans with offline / mirror continuity warnings and
//! certified-workspace or policy-bundle pin protection for the heavy artifact
//! families the M5 depth lanes add.
//!
//! An offboarding continuity plan is the operator-facing object the shell shows
//! *before* an account offboarding, device reset, workspace wipe, or sign-out
//! cleanup removes anything. Where the [`crate::m5_clear_data_review`] sheet is
//! the per-class cleanup review, this plan is the honest, whole-offboarding
//! summary: for every family it touches it states whether the bytes are
//! **exportable durable state** the user should take with them or
//! **non-portable derived data** that simply rebuilds, what offline / mirror /
//! certified-workspace continuity its removal would break, which families are
//! pinned by an offline bundle, a certified template/archetype, or a
//! last-known-good policy bundle and therefore stay protected unless explicitly
//! reviewed away, and a portability-honesty headline that never implies the user
//! exported everything when only caches were cleared.
//!
//! This module mints no new storage primitive: the storage-class,
//! artifact-family, authority, and pin-source vocabularies re-export verbatim
//! from [`crate::m5_storage_governance`]; the offline-rebuild-risk vocabulary
//! from [`crate::m5_storage_inspector`]; and the workspace-scope, initiator, and
//! export-before-delete vocabularies from [`crate::m5_clear_data_review`]. Only
//! the offboarding-flow, portability, continuity-warning, disposition, and
//! portability-honesty labels are introduced here, and they are bounded
//! explanatory tokens that resolve back to the frozen artifact-family matrix at
//! [`M5_ARTIFACT_FAMILY_MATRIX_REF`].
//!
//! ## What this owns
//!
//! - The [`OffboardingContinuityPlan`] record — one offboarding under review,
//!   carrying its flow, initiator, affected workspaces, disposed and retained
//!   rows, byte totals, continuity warnings, export-before-delete options,
//!   guardrail notices, the protected families it retained, and the
//!   portability-honesty headline. Mirrors the boundary schema at
//!   [`M5_OFFBOARDING_CONTINUITY_SCHEMA_REF`].
//! - The [`OffboardingContinuityRow`] record — one family-on-one-workspace line
//!   with its portability class, continuity warnings, offline-rebuild risk,
//!   disposition, reviewed-away state, pin sources, export posture, byte split,
//!   and a human-readable continuity note.
//! - The [`OffboardingContinuityCorpus`] container — folds every seeded scenario
//!   plan into one validated bundle, checks the cross-record safety contract, and
//!   projects a metadata-safe [`OffboardingContinuitySupportExport`] the
//!   Help / About / diagnostics / support-bundle surfaces quote without leaking
//!   raw payloads, paths, or credentials.
//! - The [`compose_offboarding_plan`] projection — the first real consumer: it
//!   folds the frozen [`M5ArtifactFamilyStorageMatrix`] plus an offboarding
//!   request into a plan that is correct by construction (protected and
//!   continuity-pinned families are retained unless explicitly reviewed away,
//!   export-before-delete is required on protected classes, continuity warnings
//!   derive from the pins actually present, and the portability headline never
//!   over-promises).
//!
//! ## What this does NOT own
//!
//! - Live byte-level deletion, sign-out, or device-reset execution. Those belong
//!   to the runtime crates; this module is the shared truth model the offboarding
//!   review, clear-data review, pin/retention manager, and support export
//!   project. A plan describes a *proposed* offboarding.
//! - The runtime storage-class vocabulary or the artifact-family matrix, which
//!   stay frozen in [`crate::m5_storage_governance`].

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_clear_data_review::{
    ExportBeforeDeleteClass, ExportBeforeDeleteOption, InitiatorClass, WorkspaceScope,
};
use crate::m5_storage_governance::{
    ArtifactFamilyId, AuthorityClass, M5ArtifactFamilyRow, M5ArtifactFamilyStorageMatrix,
    PinSourceClass, M5_ARTIFACT_FAMILY_MATRIX_REF,
};
use crate::m5_storage_inspector::OfflineRebuildRiskClass;
use crate::storage_inspector::StorageClassId;

#[cfg(test)]
mod tests;

/// Frozen schema version shared by every record in this module.
pub const M5_OFFBOARDING_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for an offboarding continuity plan.
pub const M5_OFFBOARDING_CONTINUITY_PLAN_RECORD_KIND: &str = "m5_offboarding_continuity_plan";

/// Stable record-kind tag for one offboarding continuity row.
pub const M5_OFFBOARDING_CONTINUITY_ROW_RECORD_KIND: &str = "m5_offboarding_continuity_row";

/// Stable record-kind tag for the support-export envelope.
pub const M5_OFFBOARDING_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_offboarding_continuity_support_export";

/// Stable record-kind tag for one support-export row.
pub const M5_OFFBOARDING_CONTINUITY_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "m5_offboarding_continuity_support_export_row";

/// Repository-relative path of the boundary schema for the plan.
pub const M5_OFFBOARDING_CONTINUITY_SCHEMA_REF: &str =
    "schemas/storage/m5_offboarding_continuity.schema.json";

/// Repository-relative path of the reviewer contract doc every plan quotes.
pub const M5_OFFBOARDING_CONTINUITY_DOC_REF: &str =
    "docs/storage/m5_offboarding_continuity_contract.md";

/// Repository-relative path of the canonical runtime storage-class contract.
pub const RUNTIME_STORAGE_CLASSES_REF: &str = "artifacts/runtime/storage_classes.yaml";

/// The metadata-safe redaction class every plan and export envelope carries.
pub const METADATA_SAFE_DEFAULT: &str = "metadata_safe_default";

/// The stable action id that opens the storage inspector from a plan.
pub const OPEN_STORAGE_INSPECTOR_ACTION_REF: &str = "action.storage.open_inspector";

/// The stable action id that opens the class-selective clear-data review.
pub const OPEN_CLEAR_DATA_REVIEW_ACTION_REF: &str = "action.storage.open_clear_data_review";

// --------------------------------------------------------------------------
// Closed vocabularies introduced by this lane.
//
// All of offboarding-flow, portability, continuity-warning, disposition, and
// portability-honesty are bounded explanatory tokens. They resolve against the
// frozen artifact-family matrix and the runtime storage-class contract; this
// lane mints no new storage primitive.
// --------------------------------------------------------------------------

/// Which offboarding scenario a plan governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingFlowClass {
    /// A user or admin offboarding an account from this device.
    AccountOffboarding,
    /// A full device reset that removes local state.
    DeviceReset,
    /// A single-workspace wipe.
    WorkspaceWipe,
    /// A sign-out cleanup that trims local caches.
    SignOutCleanup,
}

/// How the bytes a family holds map to what the user can take away — the
/// exportable-durable-state versus derived-data distinction the offboarding
/// must be honest about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortabilityClass {
    /// User-owned recovery state (local history, checkpoints): the user's own
    /// durable truth, portable through an export.
    ExportableDurableState,
    /// Captured evidence of a specific run: not reproducible, portable only by
    /// exporting before any removal.
    CapturedEvidenceExportToRetain,
    /// Imported packs (docs / model / template / extension / prebuild) pinned by
    /// an offline, mirror, certified, or release source: not user data, but
    /// rebuildable from that pinned or offline source.
    RebuildableFromPinnedOrOfflineSource,
    /// A pure derived cache: nothing portable is lost; it rebuilds on demand.
    NonPortableDerivedCache,
}

impl PortabilityClass {
    /// True for the classes whose loss the user must be able to export first —
    /// durable state and captured evidence.
    pub const fn is_durable(self) -> bool {
        matches!(
            self,
            Self::ExportableDurableState | Self::CapturedEvidenceExportToRetain
        )
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportableDurableState => "exportable_durable_state",
            Self::CapturedEvidenceExportToRetain => "captured_evidence_export_to_retain",
            Self::RebuildableFromPinnedOrOfflineSource => {
                "rebuildable_from_pinned_or_offline_source"
            }
            Self::NonPortableDerivedCache => "non_portable_derived_cache",
        }
    }
}

/// What removing a family would break for offline / mirror / certified-workspace
/// continuity — the implications kept visible before any deletion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityWarningClass {
    /// Removal has no offline / mirror / certified / recovery consequence.
    NoContinuityImpact,
    /// Removal loses offline readiness backed by an offline-entitlement bundle.
    OfflineReadinessLost,
    /// Removal breaks continuity with a release / mirror artifact graph.
    MirrorContinuityBroken,
    /// Removal breaks certified-workspace readiness backed by a certified
    /// template or archetype.
    CertifiedWorkspaceReadinessBroken,
    /// Removal loses the last-known-good policy bundle continuity.
    PolicyBundleContinuityLost,
    /// Removal loses captured evidence continuity that cannot be reproduced.
    EvidenceContinuityLost,
    /// Removal loses user-owned recovery-state continuity.
    RecoveryStateContinuityLost,
}

impl ContinuityWarningClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoContinuityImpact => "no_continuity_impact",
            Self::OfflineReadinessLost => "offline_readiness_lost",
            Self::MirrorContinuityBroken => "mirror_continuity_broken",
            Self::CertifiedWorkspaceReadinessBroken => "certified_workspace_readiness_broken",
            Self::PolicyBundleContinuityLost => "policy_bundle_continuity_lost",
            Self::EvidenceContinuityLost => "evidence_continuity_lost",
            Self::RecoveryStateContinuityLost => "recovery_state_continuity_lost",
        }
    }
}

/// What the offboarding does to a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingDispositionClass {
    /// A pure derived cache disposed; rebuilds on demand, nothing portable lost.
    DisposeRebuildable,
    /// A protected or continuity-pinned family the operator explicitly reviewed
    /// away; exported before removal.
    ExportThenDispose,
    /// Captured evidence or user-owned recovery state retained by default; never
    /// removed without an explicit, exported review.
    RetainedProtectedContinuity,
    /// An offline / certified / policy / mirror-pinned pack retained to keep
    /// continuity; removable only through an explicit review.
    RetainedForOfflineContinuity,
    /// A pure cache the operator chose to keep.
    RetainedNotSelected,
}

impl OffboardingDispositionClass {
    /// True when this disposition removes the family's bytes.
    pub const fn is_disposed(self) -> bool {
        matches!(self, Self::DisposeRebuildable | Self::ExportThenDispose)
    }

    /// True when this disposition keeps the family's bytes.
    pub const fn is_retained(self) -> bool {
        !self.is_disposed()
    }

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisposeRebuildable => "dispose_rebuildable",
            Self::ExportThenDispose => "export_then_dispose",
            Self::RetainedProtectedContinuity => "retained_protected_continuity",
            Self::RetainedForOfflineContinuity => "retained_for_offline_continuity",
            Self::RetainedNotSelected => "retained_not_selected",
        }
    }
}

/// The plan-level portability headline — what the user actually kept or lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortabilityHonestyClass {
    /// Nothing was disposed; every family is retained.
    NothingDisposedAllRetained,
    /// Only non-portable derived caches were disposed; durable state is retained.
    CachesOnlyRemovedDurableRetained,
    /// Some exportable durable state or captured evidence was reviewed away, but
    /// each was exported before removal.
    DurableStateExportedBeforeRemoval,
}

impl PortabilityHonestyClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NothingDisposedAllRetained => "nothing_disposed_all_retained",
            Self::CachesOnlyRemovedDurableRetained => "caches_only_removed_durable_retained",
            Self::DurableStateExportedBeforeRemoval => "durable_state_exported_before_removal",
        }
    }
}

// --------------------------------------------------------------------------
// Derivation helpers — the frozen mapping from storage class plus the pins
// actually present to portability, continuity warnings, offline-rebuild risk,
// disposition, and export posture. These resolve the frozen vocabularies; they
// mint nothing new.
// --------------------------------------------------------------------------

/// True for the protected storage classes — captured evidence and user-owned
/// recovery state.
const fn is_protected_class(class: StorageClassId) -> bool {
    matches!(
        class,
        StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
    )
}

/// True when a pin source backs offline / mirror / certified / policy continuity.
const fn is_continuity_pin(pin: PinSourceClass) -> bool {
    matches!(
        pin,
        PinSourceClass::OfflineBundleRef
            | PinSourceClass::CertifiedArchetypeOrTemplateRef
            | PinSourceClass::PolicyBundleLastKnownGoodRef
            | PinSourceClass::ReleaseArtifactGraphRef
    )
}

/// True when any pin present backs offline / mirror / certified / policy
/// continuity.
fn any_continuity_pin(pins: &[PinSourceClass]) -> bool {
    pins.iter().copied().any(is_continuity_pin)
}

/// The portability class for a storage class and the pins present.
fn portability_for(class: StorageClassId, pins: &[PinSourceClass]) -> PortabilityClass {
    match class {
        StorageClassId::UserOwnedRecoveryState => PortabilityClass::ExportableDurableState,
        StorageClassId::EvidenceSupportCache => PortabilityClass::CapturedEvidenceExportToRetain,
        StorageClassId::ArtifactCache | StorageClassId::PrebuildEnvironmentCache => {
            if any_continuity_pin(pins) {
                PortabilityClass::RebuildableFromPinnedOrOfflineSource
            } else {
                PortabilityClass::NonPortableDerivedCache
            }
        }
        StorageClassId::InteractiveHotCache | StorageClassId::KnowledgeCache => {
            PortabilityClass::NonPortableDerivedCache
        }
    }
}

/// The continuity warnings a removal would raise, sorted and de-duplicated.
fn continuity_warnings_for(
    class: StorageClassId,
    pins: &[PinSourceClass],
) -> Vec<ContinuityWarningClass> {
    let mut set: BTreeSet<ContinuityWarningClass> = BTreeSet::new();
    match class {
        StorageClassId::UserOwnedRecoveryState => {
            set.insert(ContinuityWarningClass::RecoveryStateContinuityLost);
        }
        StorageClassId::EvidenceSupportCache => {
            set.insert(ContinuityWarningClass::EvidenceContinuityLost);
        }
        _ => {}
    }
    for pin in pins {
        match pin {
            PinSourceClass::OfflineBundleRef => {
                set.insert(ContinuityWarningClass::OfflineReadinessLost);
            }
            PinSourceClass::CertifiedArchetypeOrTemplateRef => {
                set.insert(ContinuityWarningClass::CertifiedWorkspaceReadinessBroken);
            }
            PinSourceClass::PolicyBundleLastKnownGoodRef => {
                set.insert(ContinuityWarningClass::PolicyBundleContinuityLost);
            }
            PinSourceClass::ReleaseArtifactGraphRef => {
                set.insert(ContinuityWarningClass::MirrorContinuityBroken);
            }
            _ => {}
        }
    }
    if set.is_empty() {
        set.insert(ContinuityWarningClass::NoContinuityImpact);
    }
    set.into_iter().collect()
}

/// The offline-rebuild risk for a storage class and the pins present.
fn offline_rebuild_risk_for(
    class: StorageClassId,
    pins: &[PinSourceClass],
) -> OfflineRebuildRiskClass {
    match class {
        StorageClassId::UserOwnedRecoveryState | StorageClassId::EvidenceSupportCache => {
            OfflineRebuildRiskClass::NotRebuildableAfterRemoval
        }
        StorageClassId::InteractiveHotCache => OfflineRebuildRiskClass::SafeToRemoveOffline,
        StorageClassId::KnowledgeCache => OfflineRebuildRiskClass::RebuildRequiresNetworkResync,
        StorageClassId::ArtifactCache | StorageClassId::PrebuildEnvironmentCache => {
            if pins
                .iter()
                .any(|pin| *pin == PinSourceClass::PolicyBundleLastKnownGoodRef)
            {
                OfflineRebuildRiskClass::RebuildRequiresAdminOrPolicySignedPack
            } else if any_continuity_pin(pins) {
                OfflineRebuildRiskClass::RebuildRequiresMirrorOrOfflineBundle
            } else {
                OfflineRebuildRiskClass::RebuildRequiresNetworkResync
            }
        }
    }
}

/// The export-before-delete posture for a storage class and the pins present.
fn export_class_for(class: StorageClassId, pins: &[PinSourceClass]) -> ExportBeforeDeleteClass {
    if is_protected_class(class) {
        ExportBeforeDeleteClass::ExportRequiredBeforeDelete
    } else if any_continuity_pin(pins) {
        ExportBeforeDeleteClass::ExportOfferedOptional
    } else {
        ExportBeforeDeleteClass::ExportNotApplicableDisposable
    }
}

/// The disposition for a family given its protection posture and the operator's
/// request. Protected and continuity-pinned families are retained unless
/// explicitly reviewed away.
fn disposition_for(
    protected: bool,
    continuity_pinned: bool,
    requested_disposal: bool,
    reviewed_away: bool,
) -> OffboardingDispositionClass {
    if protected || continuity_pinned {
        if reviewed_away {
            OffboardingDispositionClass::ExportThenDispose
        } else if protected {
            OffboardingDispositionClass::RetainedProtectedContinuity
        } else {
            OffboardingDispositionClass::RetainedForOfflineContinuity
        }
    } else if requested_disposal {
        OffboardingDispositionClass::DisposeRebuildable
    } else {
        OffboardingDispositionClass::RetainedNotSelected
    }
}

/// A deterministic, human-readable continuity note for one row.
fn continuity_note_for(
    class: StorageClassId,
    portability: PortabilityClass,
    disposition: OffboardingDispositionClass,
) -> String {
    let stake = match portability {
        PortabilityClass::ExportableDurableState => {
            "User-owned recovery state (local history and checkpoints) — exportable durable state with no rebuild path."
        }
        PortabilityClass::CapturedEvidenceExportToRetain => {
            "Captured evidence of a specific run — cannot be reproduced; export before any removal."
        }
        PortabilityClass::RebuildableFromPinnedOrOfflineSource => {
            "Backs offline, mirror, or certified-workspace readiness — rebuildable from its pinned or offline source."
        }
        PortabilityClass::NonPortableDerivedCache => {
            "Rebuildable derived cache — nothing portable is lost; regenerated on demand."
        }
    };
    let fate = match disposition {
        OffboardingDispositionClass::DisposeRebuildable => {
            " It is disposed; it rebuilds on demand."
        }
        OffboardingDispositionClass::ExportThenDispose => {
            " It was explicitly reviewed away and exported before removal."
        }
        OffboardingDispositionClass::RetainedProtectedContinuity => {
            " It is retained; removal needs an explicit, exported review."
        }
        OffboardingDispositionClass::RetainedForOfflineContinuity => {
            " It is retained to keep continuity unless you explicitly review it away."
        }
        OffboardingDispositionClass::RetainedNotSelected => " It is retained.",
    };
    let _ = class;
    format!("{stake}{fate}")
}

// --------------------------------------------------------------------------
// Records.
// --------------------------------------------------------------------------

/// One family-on-one-workspace line in an offboarding continuity plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingContinuityRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub family_id: ArtifactFamilyId,
    pub storage_class_id: StorageClassId,
    pub workspace_scope_ref: String,
    pub workspace_label: String,
    pub authority_class: AuthorityClass,
    pub portability_class: PortabilityClass,
    /// Continuity implications of removing this family, sorted.
    pub continuity_warnings: Vec<ContinuityWarningClass>,
    pub offline_rebuild_risk_class: OfflineRebuildRiskClass,
    pub disposition: OffboardingDispositionClass,
    /// True when the operator explicitly reviewed a protected / continuity-pinned
    /// family away. Always false for pure caches.
    pub reviewed_away: bool,
    /// True for captured evidence and user-owned recovery classes.
    pub protected_continuity: bool,
    /// True when an offline / mirror / certified / policy pin protects this
    /// family.
    pub continuity_pinned: bool,
    /// True when this plan keeps the family's bytes.
    pub continuity_preserved: bool,
    /// The pins actually present on this family, sorted.
    #[serde(default)]
    pub pin_source_classes: Vec<PinSourceClass>,
    pub export_before_delete_class: ExportBeforeDeleteClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_action_ref: Option<String>,
    /// Total bytes of this family on this workspace.
    pub total_bytes: u64,
    /// Bytes the offboarding removes (zero for a retained row).
    pub disposed_bytes: u64,
    /// Bytes the offboarding keeps (equals `total_bytes` when retained).
    pub retained_bytes: u64,
    /// Human-readable continuity note; never empty.
    pub continuity_note: String,
    pub note: String,
}

impl OffboardingContinuityRow {
    /// True when this row's storage class is a protected class.
    pub const fn is_protected_class(&self) -> bool {
        is_protected_class(self.storage_class_id)
    }
}

/// One proposed offboarding under review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingContinuityPlan {
    pub record_kind: String,
    pub schema_version: u32,
    pub plan_id: String,
    pub emitted_at: String,
    pub title: String,
    pub offboarding_flow_class: OffboardingFlowClass,
    pub initiator_class: InitiatorClass,
    pub affected_workspaces: Vec<WorkspaceScope>,
    pub disposed_rows: Vec<OffboardingContinuityRow>,
    pub retained_rows: Vec<OffboardingContinuityRow>,
    pub total_disposed_bytes: u64,
    pub total_retained_bytes: u64,
    /// The active continuity losses the operator is accepting, sorted.
    #[serde(default)]
    pub continuity_warnings: Vec<ContinuityWarningClass>,
    #[serde(default)]
    pub export_before_delete_options: Vec<ExportBeforeDeleteOption>,
    /// The protected / continuity-pinned families this plan keeps, sorted.
    #[serde(default)]
    pub protected_families_retained: Vec<ArtifactFamilyId>,
    #[serde(default)]
    pub guardrail_notices: Vec<String>,
    pub portability_honesty_class: PortabilityHonestyClass,
    /// The portability headline; never empty.
    pub portability_summary: String,
    pub open_inspector_action_ref: String,
    pub open_clear_data_review_action_ref: String,
    pub matrix_ref: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub redaction_class: String,
    pub raw_content_exported: bool,
    pub export_safe: bool,
    pub note: String,
}

impl OffboardingContinuityPlan {
    /// Iterates every row, disposed then retained.
    pub fn all_rows(&self) -> impl Iterator<Item = &OffboardingContinuityRow> {
        self.disposed_rows.iter().chain(self.retained_rows.iter())
    }

    /// True when the plan is metadata-safe and carries no raw payload.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported && self.redaction_class == METADATA_SAFE_DEFAULT
    }

    /// Convenience: true when this plan validates with zero violations.
    pub fn is_valid(&self) -> bool {
        let mut violations = Vec::new();
        self.validate_into(&mut violations, &self.plan_id);
        violations.is_empty()
    }

    /// Validates the plan against the offboarding-safe cleanup contract.
    pub fn validate(&self) -> Vec<M5OffboardingContinuityViolation> {
        let mut violations = Vec::new();
        self.validate_into(&mut violations, &self.plan_id);
        violations
    }

    /// Validates this plan, attributing each violation to `target_ref`.
    pub fn validate_into(
        &self,
        violations: &mut Vec<M5OffboardingContinuityViolation>,
        target_ref: &str,
    ) {
        let target = target_ref;
        if self.schema_version != M5_OFFBOARDING_CONTINUITY_SCHEMA_VERSION {
            push(
                violations,
                "plan.schema_version",
                target,
                "schema_version must be 1",
            );
        }
        if self.record_kind != M5_OFFBOARDING_CONTINUITY_PLAN_RECORD_KIND {
            push(
                violations,
                "plan.record_kind",
                target,
                "record_kind must be m5_offboarding_continuity_plan",
            );
        }
        if self.schema_ref != M5_OFFBOARDING_CONTINUITY_SCHEMA_REF {
            push(
                violations,
                "plan.schema_ref",
                target,
                "schema_ref must pin the plan boundary schema",
            );
        }
        if self.doc_ref != M5_OFFBOARDING_CONTINUITY_DOC_REF {
            push(
                violations,
                "plan.doc_ref",
                target,
                "doc_ref must pin the plan contract doc",
            );
        }
        if self.matrix_ref != M5_ARTIFACT_FAMILY_MATRIX_REF {
            push(
                violations,
                "plan.matrix_ref",
                target,
                "matrix_ref must pin the artifact-family storage matrix",
            );
        }
        if self.title.trim().is_empty() {
            push(violations, "plan.title", target, "title must be non-empty");
        }
        if self.portability_summary.trim().is_empty() {
            push(
                violations,
                "plan.portability_summary",
                target,
                "portability_summary must never be hidden",
            );
        }
        if self.redaction_class != METADATA_SAFE_DEFAULT {
            push(
                violations,
                "plan.redaction_class",
                target,
                "redaction_class must be metadata_safe_default",
            );
        }
        if self.raw_content_exported {
            push(
                violations,
                "plan.raw_content_exported",
                target,
                "raw_content_exported must be false",
            );
        }
        if self.export_safe != self.is_export_safe() {
            push(
                violations,
                "plan.export_safe",
                target,
                "export_safe must equal the computed metadata-safe posture",
            );
        }
        if self.open_inspector_action_ref != OPEN_STORAGE_INSPECTOR_ACTION_REF {
            push(
                violations,
                "plan.open_inspector_action_ref",
                target,
                "open_inspector_action_ref must offer the inspector action",
            );
        }
        if self.open_clear_data_review_action_ref != OPEN_CLEAR_DATA_REVIEW_ACTION_REF {
            push(
                violations,
                "plan.open_clear_data_review_action_ref",
                target,
                "open_clear_data_review_action_ref must offer the review action",
            );
        }
        if self.affected_workspaces.is_empty() {
            push(
                violations,
                "plan.affected_workspaces",
                target,
                "at least one affected workspace must be listed",
            );
        }

        let workspace_refs: BTreeSet<&str> = self
            .affected_workspaces
            .iter()
            .map(|w| w.scope_ref.as_str())
            .collect();

        let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
        let mut computed_disposed: u64 = 0;
        let mut computed_retained: u64 = 0;

        for row in &self.disposed_rows {
            if !row.disposition.is_disposed() {
                push(
                    violations,
                    "plan.row_bucket_mismatch",
                    &row.row_id,
                    "a disposed-bucket row must carry a disposed disposition",
                );
            }
            self.validate_row(violations, row, &workspace_refs);
            if !seen_ids.insert(row.row_id.as_str()) {
                push(
                    violations,
                    "plan.duplicate_row_id",
                    &row.row_id,
                    "row_id must be unique across the plan",
                );
            }
            computed_disposed = computed_disposed.saturating_add(row.disposed_bytes);
            computed_retained = computed_retained.saturating_add(row.retained_bytes);
        }
        for row in &self.retained_rows {
            if !row.disposition.is_retained() {
                push(
                    violations,
                    "plan.row_bucket_mismatch",
                    &row.row_id,
                    "a retained-bucket row must carry a retained disposition",
                );
            }
            self.validate_row(violations, row, &workspace_refs);
            if !seen_ids.insert(row.row_id.as_str()) {
                push(
                    violations,
                    "plan.duplicate_row_id",
                    &row.row_id,
                    "row_id must be unique across the plan",
                );
            }
            computed_disposed = computed_disposed.saturating_add(row.disposed_bytes);
            computed_retained = computed_retained.saturating_add(row.retained_bytes);
        }

        if computed_disposed != self.total_disposed_bytes {
            push(
                violations,
                "plan.disposed_total",
                target,
                "total_disposed_bytes must equal the sum of disposed_bytes",
            );
        }
        if computed_retained != self.total_retained_bytes {
            push(
                violations,
                "plan.retained_total",
                target,
                "total_retained_bytes must equal the sum of retained_bytes",
            );
        }

        // Plan-level continuity warnings are the active losses across disposed rows.
        let computed_warnings = self.compute_active_continuity_warnings();
        if self.continuity_warnings != computed_warnings {
            push(
                violations,
                "plan.continuity_warnings",
                target,
                "continuity_warnings must equal the active losses across disposed rows",
            );
        }

        // The protected / continuity-pinned families this plan keeps.
        let computed_retained_families = self.compute_protected_families_retained();
        if self.protected_families_retained != computed_retained_families {
            push(violations, "plan.protected_families_retained", target, "protected_families_retained must equal the retained protected / continuity-pinned families");
        }

        // The portability headline must match the computed reality.
        let computed_honesty = self.compute_portability_honesty();
        if self.portability_honesty_class != computed_honesty {
            push(
                violations,
                "plan.portability_honesty",
                target,
                "portability_honesty_class must match the disposed / retained reality",
            );
        }

        // Every export-required row must have a matching offered option.
        for row in self.all_rows() {
            if row.export_before_delete_class == ExportBeforeDeleteClass::ExportRequiredBeforeDelete
            {
                let offered = self.export_before_delete_options.iter().any(|o| {
                    o.family_id == row.family_id
                        && o.export_class == ExportBeforeDeleteClass::ExportRequiredBeforeDelete
                });
                if !offered {
                    push(
                        violations,
                        "plan.export_option_missing",
                        &row.row_id,
                        "an export-required row must have a matching export-before-delete option",
                    );
                }
            }
        }
    }

    fn validate_row(
        &self,
        violations: &mut Vec<M5OffboardingContinuityViolation>,
        row: &OffboardingContinuityRow,
        workspace_refs: &BTreeSet<&str>,
    ) {
        let target = &row.row_id;
        if row.schema_version != M5_OFFBOARDING_CONTINUITY_SCHEMA_VERSION {
            push(
                violations,
                "row.schema_version",
                target,
                "schema_version must be 1",
            );
        }
        if row.record_kind != M5_OFFBOARDING_CONTINUITY_ROW_RECORD_KIND {
            push(
                violations,
                "row.record_kind",
                target,
                "record_kind must be m5_offboarding_continuity_row",
            );
        }
        if row.row_id.trim().is_empty() {
            push(violations, "row.row_id", target, "row_id must be non-empty");
        }
        if !workspace_refs.contains(row.workspace_scope_ref.as_str()) {
            push(
                violations,
                "row.unknown_workspace",
                target,
                "workspace_scope_ref must reference an affected workspace",
            );
        }
        if row.continuity_note.trim().is_empty() {
            push(
                violations,
                "row.continuity_note",
                target,
                "continuity_note must never be hidden",
            );
        }
        if row.total_bytes != row.disposed_bytes.saturating_add(row.retained_bytes) {
            push(
                violations,
                "row.byte_arithmetic",
                target,
                "total_bytes must equal disposed_bytes + retained_bytes",
            );
        }

        // Derived fields must track the storage class and the pins present.
        let pins = row.pin_source_classes.as_slice();
        let expected_protected = is_protected_class(row.storage_class_id);
        if row.protected_continuity != expected_protected {
            push(violations, "row.protected_continuity", target, "protected_continuity must be true exactly for evidence and user-owned recovery classes");
        }
        let expected_continuity_pinned = any_continuity_pin(pins);
        if row.continuity_pinned != expected_continuity_pinned {
            push(violations, "row.continuity_pinned", target, "continuity_pinned must be true exactly when an offline / mirror / certified / policy pin is present");
        }
        if row.portability_class != portability_for(row.storage_class_id, pins) {
            push(
                violations,
                "row.portability_class",
                target,
                "portability_class must derive from the storage class and the pins present",
            );
        }
        if row.continuity_warnings != continuity_warnings_for(row.storage_class_id, pins) {
            push(
                violations,
                "row.continuity_warnings",
                target,
                "continuity_warnings must derive from the storage class and the pins present",
            );
        }
        if row.offline_rebuild_risk_class != offline_rebuild_risk_for(row.storage_class_id, pins) {
            push(violations, "row.offline_rebuild_risk_class", target, "offline_rebuild_risk_class must derive from the storage class and the pins present");
        }
        if row.export_before_delete_class != export_class_for(row.storage_class_id, pins) {
            push(
                violations,
                "row.export_before_delete_class",
                target,
                "export_before_delete_class must derive from the protection posture",
            );
        }
        let expected_export_ref = row.export_before_delete_class
            != ExportBeforeDeleteClass::ExportNotApplicableDisposable;
        if row.export_action_ref.is_some() != expected_export_ref {
            push(
                violations,
                "row.export_action_ref",
                target,
                "export_action_ref is present exactly when an export path applies",
            );
        }
        if row.continuity_preserved != row.disposition.is_retained() {
            push(
                violations,
                "row.continuity_preserved",
                target,
                "continuity_preserved must equal whether the row is retained",
            );
        }

        // Byte split follows the disposition.
        if row.disposition.is_disposed() {
            if row.disposed_bytes != row.total_bytes || row.retained_bytes != 0 {
                push(
                    violations,
                    "row.disposed_byte_split",
                    target,
                    "a disposed row removes all of its bytes",
                );
            }
        } else if row.retained_bytes != row.total_bytes || row.disposed_bytes != 0 {
            push(
                violations,
                "row.retained_byte_split",
                target,
                "a retained row keeps all of its bytes",
            );
        }

        self.validate_disposition_rules(violations, row);
    }

    /// Each disposition constrains the protection posture, reviewed-away state,
    /// and export posture so no protected or continuity-pinned family is removed
    /// without an explicit, exported review.
    fn validate_disposition_rules(
        &self,
        violations: &mut Vec<M5OffboardingContinuityViolation>,
        row: &OffboardingContinuityRow,
    ) {
        let target = &row.row_id;
        let protected = row.protected_continuity;
        let continuity_pinned = row.continuity_pinned;

        // A protected class must always require export-before-delete, regardless
        // of bucket.
        if protected
            && row.export_before_delete_class != ExportBeforeDeleteClass::ExportRequiredBeforeDelete
        {
            push(
                violations,
                "row.protected_export_required",
                target,
                "a protected class must require export-before-delete",
            );
        }

        match row.disposition {
            OffboardingDispositionClass::DisposeRebuildable => {
                if protected || continuity_pinned {
                    push(violations, "row.dispose_rebuildable_protected", target, "dispose_rebuildable is reserved for non-protected, non-continuity-pinned caches");
                }
                if row.reviewed_away {
                    push(
                        violations,
                        "row.dispose_rebuildable_reviewed",
                        target,
                        "a pure cache is never reviewed_away",
                    );
                }
                if row.portability_class != PortabilityClass::NonPortableDerivedCache {
                    push(
                        violations,
                        "row.dispose_rebuildable_portability",
                        target,
                        "dispose_rebuildable requires a non_portable_derived_cache",
                    );
                }
            }
            OffboardingDispositionClass::ExportThenDispose => {
                if !(protected || continuity_pinned) {
                    push(violations, "row.export_then_dispose_unprotected", target, "export_then_dispose is reserved for protected or continuity-pinned families");
                }
                if !row.reviewed_away {
                    push(
                        violations,
                        "row.export_then_dispose_unreviewed",
                        target,
                        "export_then_dispose requires an explicit reviewed_away",
                    );
                }
                if row.export_before_delete_class
                    == ExportBeforeDeleteClass::ExportNotApplicableDisposable
                {
                    push(
                        violations,
                        "row.export_then_dispose_no_export",
                        target,
                        "export_then_dispose must offer or require an export path",
                    );
                }
            }
            OffboardingDispositionClass::RetainedProtectedContinuity => {
                if !protected {
                    push(
                        violations,
                        "row.retained_protected_unprotected",
                        target,
                        "retained_protected_continuity is reserved for protected classes",
                    );
                }
                if row.reviewed_away {
                    push(
                        violations,
                        "row.retained_protected_reviewed",
                        target,
                        "a retained protected family is not reviewed_away",
                    );
                }
            }
            OffboardingDispositionClass::RetainedForOfflineContinuity => {
                if protected || !continuity_pinned {
                    push(violations, "row.retained_offline_misclassified", target, "retained_for_offline_continuity is reserved for continuity-pinned, non-protected packs");
                }
                if row.reviewed_away {
                    push(
                        violations,
                        "row.retained_offline_reviewed",
                        target,
                        "a retained continuity-pinned family is not reviewed_away",
                    );
                }
            }
            OffboardingDispositionClass::RetainedNotSelected => {
                if protected || continuity_pinned {
                    push(violations, "row.retained_not_selected_protected", target, "retained_not_selected is reserved for non-protected, non-continuity-pinned caches");
                }
                if row.reviewed_away {
                    push(
                        violations,
                        "row.retained_not_selected_reviewed",
                        target,
                        "a pure cache is never reviewed_away",
                    );
                }
            }
        }
    }

    fn compute_active_continuity_warnings(&self) -> Vec<ContinuityWarningClass> {
        let mut set: BTreeSet<ContinuityWarningClass> = BTreeSet::new();
        for row in &self.disposed_rows {
            for warning in &row.continuity_warnings {
                if *warning != ContinuityWarningClass::NoContinuityImpact {
                    set.insert(*warning);
                }
            }
        }
        set.into_iter().collect()
    }

    fn compute_protected_families_retained(&self) -> Vec<ArtifactFamilyId> {
        let mut families: Vec<ArtifactFamilyId> = self
            .retained_rows
            .iter()
            .filter(|row| row.protected_continuity || row.continuity_pinned)
            .map(|row| row.family_id)
            .collect();
        families.sort();
        families.dedup();
        families
    }

    fn compute_portability_honesty(&self) -> PortabilityHonestyClass {
        if self.disposed_rows.is_empty() {
            PortabilityHonestyClass::NothingDisposedAllRetained
        } else if self
            .disposed_rows
            .iter()
            .any(|row| row.portability_class.is_durable())
        {
            PortabilityHonestyClass::DurableStateExportedBeforeRemoval
        } else {
            PortabilityHonestyClass::CachesOnlyRemovedDurableRetained
        }
    }
}

// --------------------------------------------------------------------------
// Matrix-backed composer — the first real consumer.
// --------------------------------------------------------------------------

/// One family the operator's offboarding touches, fed to
/// [`compose_offboarding_plan`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingFamilySelection {
    pub family_id: ArtifactFamilyId,
    pub workspace_scope_ref: String,
    pub workspace_label: String,
    pub total_bytes: u64,
    /// True when the operator wants this family removed by the offboarding.
    pub requested_disposal: bool,
    /// True when the operator explicitly reviewed a protected / continuity-pinned
    /// family away. Ignored for pure caches.
    #[serde(default)]
    pub reviewed_away: bool,
    /// The pins actually present on this family.
    #[serde(default)]
    pub pin_source_classes: Vec<PinSourceClass>,
}

/// The request [`compose_offboarding_plan`] folds into a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingContinuityRequest {
    pub plan_id: String,
    pub emitted_at: String,
    pub title: String,
    pub offboarding_flow_class: OffboardingFlowClass,
    pub initiator_class: InitiatorClass,
    pub workspaces: Vec<WorkspaceScope>,
    pub selections: Vec<OffboardingFamilySelection>,
    #[serde(default)]
    pub note: String,
}

/// Folds the frozen artifact-family matrix plus an offboarding request into a
/// plan that is correct by construction: captured evidence, user-owned recovery
/// state, and offline / certified / policy-pinned packs are retained unless
/// explicitly reviewed away; export-before-delete is required on protected
/// classes; continuity warnings derive from the pins actually present; and the
/// portability headline never implies the user exported everything when only
/// caches were cleared.
///
/// The `matrix` argument anchors the plan to the same frozen storage classes and
/// pin sources the storage-governance lane validates; the composer reads no
/// private mapping of its own.
pub fn compose_offboarding_plan(
    matrix: &M5ArtifactFamilyStorageMatrix,
    request: &OffboardingContinuityRequest,
) -> OffboardingContinuityPlan {
    let mut disposed_rows = Vec::new();
    let mut retained_rows = Vec::new();
    let mut export_options: BTreeMap<ArtifactFamilyId, ExportBeforeDeleteOption> = BTreeMap::new();
    let mut guardrail_notices: Vec<String> = Vec::new();

    let mut total_disposed: u64 = 0;
    let mut total_retained: u64 = 0;

    for (index, selection) in request.selections.iter().enumerate() {
        let Some(matrix_row) = matrix.family(selection.family_id) else {
            continue;
        };
        // Only pins admissible under the matrix row count toward continuity.
        let pins = admissible_pins(matrix_row, &selection.pin_source_classes);
        let class = matrix_row.storage_class_id;
        let protected = is_protected_class(class);
        let continuity_pinned = any_continuity_pin(&pins);

        let disposition = disposition_for(
            protected,
            continuity_pinned,
            selection.requested_disposal,
            selection.reviewed_away,
        );
        // A protected or continuity-pinned family requested for removal without an
        // explicit review stays retained, and the plan says why.
        if (protected || continuity_pinned)
            && selection.requested_disposal
            && !selection.reviewed_away
        {
            let notice = format!(
                "{} is protected for offline / certified / policy continuity; explicitly review it away to remove it.",
                matrix_row.label
            );
            if !guardrail_notices.contains(&notice) {
                guardrail_notices.push(notice);
            }
        }

        let portability = portability_for(class, &pins);
        let warnings = continuity_warnings_for(class, &pins);
        let offline_risk = offline_rebuild_risk_for(class, &pins);
        let export_class = export_class_for(class, &pins);
        let reviewed_away = (protected || continuity_pinned) && selection.reviewed_away;

        let export_action_ref = if export_class
            != ExportBeforeDeleteClass::ExportNotApplicableDisposable
        {
            let action = format!("export.m5_offboarding.{}", selection.family_id.as_str());
            export_options
                .entry(selection.family_id)
                .or_insert_with(|| ExportBeforeDeleteOption {
                    family_id: selection.family_id,
                    export_class,
                    export_path_label: format!("Export {} before offboarding", matrix_row.label),
                    export_action_ref: action.clone(),
                });
            Some(action)
        } else {
            None
        };

        // If the operator reviewed away an active continuity-bearing family,
        // surface the loss as a guardrail notice too.
        if disposition.is_disposed() && reviewed_away {
            for warning in &warnings {
                if *warning == ContinuityWarningClass::NoContinuityImpact {
                    continue;
                }
                let notice = format!(
                    "Removing {} accepts: {}.",
                    matrix_row.label,
                    warning.as_str().replace('_', " ")
                );
                if !guardrail_notices.contains(&notice) {
                    guardrail_notices.push(notice);
                }
            }
        }

        let (disposed_bytes, retained_bytes) = if disposition.is_disposed() {
            (selection.total_bytes, 0)
        } else {
            (0, selection.total_bytes)
        };

        let row = OffboardingContinuityRow {
            record_kind: M5_OFFBOARDING_CONTINUITY_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_OFFBOARDING_CONTINUITY_SCHEMA_VERSION,
            row_id: format!(
                "offboarding_row.{}.{}.{}",
                request.plan_id,
                selection.family_id.as_str(),
                index
            ),
            family_id: selection.family_id,
            storage_class_id: class,
            workspace_scope_ref: selection.workspace_scope_ref.clone(),
            workspace_label: selection.workspace_label.clone(),
            authority_class: matrix_row.authority_class,
            portability_class: portability,
            continuity_warnings: warnings,
            offline_rebuild_risk_class: offline_risk,
            disposition,
            reviewed_away,
            protected_continuity: protected,
            continuity_pinned,
            continuity_preserved: disposition.is_retained(),
            pin_source_classes: pins,
            export_before_delete_class: export_class,
            export_action_ref,
            total_bytes: selection.total_bytes,
            disposed_bytes,
            retained_bytes,
            continuity_note: continuity_note_for(class, portability, disposition),
            note: String::new(),
        };

        if row.disposition.is_disposed() {
            total_disposed = total_disposed.saturating_add(row.disposed_bytes);
            disposed_rows.push(row);
        } else {
            total_retained = total_retained.saturating_add(row.retained_bytes);
            retained_rows.push(row);
        }
    }

    let mut export_before_delete_options: Vec<ExportBeforeDeleteOption> =
        export_options.into_values().collect();
    export_before_delete_options.sort_by(|a, b| a.family_id.cmp(&b.family_id));

    let mut plan = OffboardingContinuityPlan {
        record_kind: M5_OFFBOARDING_CONTINUITY_PLAN_RECORD_KIND.to_owned(),
        schema_version: M5_OFFBOARDING_CONTINUITY_SCHEMA_VERSION,
        plan_id: request.plan_id.clone(),
        emitted_at: request.emitted_at.clone(),
        title: request.title.clone(),
        offboarding_flow_class: request.offboarding_flow_class,
        initiator_class: request.initiator_class,
        affected_workspaces: request.workspaces.clone(),
        disposed_rows,
        retained_rows,
        total_disposed_bytes: total_disposed,
        total_retained_bytes: total_retained,
        continuity_warnings: Vec::new(),
        export_before_delete_options,
        protected_families_retained: Vec::new(),
        guardrail_notices,
        portability_honesty_class: PortabilityHonestyClass::NothingDisposedAllRetained,
        portability_summary: String::new(),
        open_inspector_action_ref: OPEN_STORAGE_INSPECTOR_ACTION_REF.to_owned(),
        open_clear_data_review_action_ref: OPEN_CLEAR_DATA_REVIEW_ACTION_REF.to_owned(),
        matrix_ref: M5_ARTIFACT_FAMILY_MATRIX_REF.to_owned(),
        schema_ref: M5_OFFBOARDING_CONTINUITY_SCHEMA_REF.to_owned(),
        doc_ref: M5_OFFBOARDING_CONTINUITY_DOC_REF.to_owned(),
        redaction_class: METADATA_SAFE_DEFAULT.to_owned(),
        raw_content_exported: false,
        export_safe: true,
        note: request.note.clone(),
    };

    plan.continuity_warnings = plan.compute_active_continuity_warnings();
    plan.protected_families_retained = plan.compute_protected_families_retained();
    plan.portability_honesty_class = plan.compute_portability_honesty();
    plan.portability_summary = portability_summary_for(plan.portability_honesty_class);
    plan
}

/// The subset of the operator-supplied pins that are admissible under the
/// family's matrix row, sorted. Drops any pin the matrix does not allow so a
/// caller can never invent continuity an artifact family does not carry.
fn admissible_pins(row: &M5ArtifactFamilyRow, supplied: &[PinSourceClass]) -> Vec<PinSourceClass> {
    let mut pins: Vec<PinSourceClass> = supplied
        .iter()
        .copied()
        .filter(|pin| row.pin_source_classes.contains(pin))
        .collect();
    pins.sort();
    pins.dedup();
    pins
}

/// The deterministic portability headline for a plan.
fn portability_summary_for(honesty: PortabilityHonestyClass) -> String {
    match honesty {
        PortabilityHonestyClass::NothingDisposedAllRetained => {
            "Nothing was removed; every family is retained. Use the export paths to take your durable data with you.".to_owned()
        }
        PortabilityHonestyClass::CachesOnlyRemovedDurableRetained => {
            "Only rebuildable derived caches were removed; your exportable durable state and captured evidence are unchanged. This offboarding did not export everything.".to_owned()
        }
        PortabilityHonestyClass::DurableStateExportedBeforeRemoval => {
            "Some exportable durable state or captured evidence was explicitly reviewed away; each was exported before removal.".to_owned()
        }
    }
}

// --------------------------------------------------------------------------
// Corpus container, entries, and loaders.
// --------------------------------------------------------------------------

/// One seeded scenario plan paired with its repository-relative fixture path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingContinuityEntry {
    pub fixture_ref: String,
    pub plan: OffboardingContinuityPlan,
}

/// The validated bundle of seeded offboarding continuity plans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingContinuityCorpus {
    pub plans: Vec<OffboardingContinuityEntry>,
}

const PLAN_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/storage/m5_offboarding_continuity_cases/account_offboarding_durable_retained.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_offboarding_continuity_cases/account_offboarding_durable_retained.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_offboarding_continuity_cases/device_reset_caches_only.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_offboarding_continuity_cases/device_reset_caches_only.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_offboarding_continuity_cases/offline_certified_policy_pins_retained.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_offboarding_continuity_cases/offline_certified_policy_pins_retained.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_offboarding_continuity_cases/offline_bundle_reviewed_away_continuity_warned.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_offboarding_continuity_cases/offline_bundle_reviewed_away_continuity_warned.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_offboarding_continuity_cases/workspace_wipe_reviewed_away_export_first.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_offboarding_continuity_cases/workspace_wipe_reviewed_away_export_first.yaml"
        )),
    ),
];

/// Strongly typed error returned by the corpus loader.
#[derive(Debug)]
pub enum OffboardingContinuityLoadError {
    Yaml {
        fixture_ref: String,
        source: serde_yaml::Error,
    },
}

impl fmt::Display for OffboardingContinuityLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml {
                fixture_ref,
                source,
            } => write!(
                f,
                "offboarding-continuity yaml parse error in {fixture_ref}: {source}"
            ),
        }
    }
}

impl Error for OffboardingContinuityLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml { source, .. } => Some(source),
        }
    }
}

/// Loads the checked-in offboarding continuity scenario corpus.
pub fn current_offboarding_continuity_corpus(
) -> Result<OffboardingContinuityCorpus, OffboardingContinuityLoadError> {
    let plans = PLAN_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<OffboardingContinuityPlan>(yaml)
                .map(|plan| OffboardingContinuityEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    plan,
                })
                .map_err(|source| OffboardingContinuityLoadError::Yaml {
                    fixture_ref: (*fixture_ref).to_owned(),
                    source,
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(OffboardingContinuityCorpus { plans })
}

/// The canonical scenario requests the seeded corpus is composed from. The dump
/// example and the corpus replay test both fold these through
/// [`compose_offboarding_plan`] so the checked-in fixtures can never drift from
/// the composer.
pub fn seeded_offboarding_requests() -> Vec<OffboardingContinuityRequest> {
    vec![
        account_offboarding_request(),
        device_reset_request(),
        offline_certified_policy_request(),
        offline_reviewed_away_request(),
        workspace_wipe_request(),
    ]
}

impl OffboardingContinuityCorpus {
    /// Returns the plan with the given id, if present.
    pub fn plan(&self, plan_id: &str) -> Option<&OffboardingContinuityPlan> {
        self.plans
            .iter()
            .find(|entry| entry.plan.plan_id == plan_id)
            .map(|entry| &entry.plan)
    }

    /// Validates every seeded plan against the safety contract, attributing each
    /// violation to its originating fixture.
    pub fn validate(&self) -> Vec<M5OffboardingContinuityViolation> {
        let mut violations = Vec::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.plans {
            if !seen.insert(entry.plan.plan_id.as_str()) {
                push(
                    &mut violations,
                    "corpus.duplicate_plan_id",
                    &entry.fixture_ref,
                    "plan_id must be unique across the corpus",
                );
            }
            entry
                .plan
                .validate_into(&mut violations, &entry.fixture_ref);
        }
        violations
    }

    /// Projects the corpus into a metadata-safe support / export envelope the
    /// Help / About / diagnostics / support-bundle surfaces quote without leaking
    /// raw payloads, paths, or credentials.
    pub fn support_export(
        &self,
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> OffboardingContinuitySupportExport {
        let mut plans: Vec<OffboardingContinuitySupportExportRow> = self
            .plans
            .iter()
            .map(|entry| OffboardingContinuitySupportExportRow::from_plan(&entry.plan))
            .collect();
        plans.sort_by(|a, b| a.plan_id.cmp(&b.plan_id));

        let protected_retained_family_count = self
            .plans
            .iter()
            .map(|entry| entry.plan.protected_families_retained.len() as u32)
            .sum();
        let durable_disposed_plan_count = self
            .plans
            .iter()
            .filter(|entry| {
                entry.plan.portability_honesty_class
                    == PortabilityHonestyClass::DurableStateExportedBeforeRemoval
            })
            .count() as u32;
        let continuity_warning_plan_count = self
            .plans
            .iter()
            .filter(|entry| !entry.plan.continuity_warnings.is_empty())
            .count() as u32;

        OffboardingContinuitySupportExport {
            record_kind: M5_OFFBOARDING_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_OFFBOARDING_CONTINUITY_SCHEMA_VERSION,
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            schema_ref: M5_OFFBOARDING_CONTINUITY_SCHEMA_REF.to_owned(),
            doc_ref: M5_OFFBOARDING_CONTINUITY_DOC_REF.to_owned(),
            matrix_ref: M5_ARTIFACT_FAMILY_MATRIX_REF.to_owned(),
            runtime_storage_classes_ref: RUNTIME_STORAGE_CLASSES_REF.to_owned(),
            plan_count: self.plans.len() as u32,
            protected_retained_family_count,
            durable_disposed_plan_count,
            continuity_warning_plan_count,
            raw_content_exported: false,
            redaction_class: METADATA_SAFE_DEFAULT.to_owned(),
            plans,
        }
    }
}

// --------------------------------------------------------------------------
// Support-export projection.
// --------------------------------------------------------------------------

/// One metadata-safe summary row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingContinuitySupportExportRow {
    pub record_kind: String,
    pub plan_id: String,
    pub offboarding_flow_class: OffboardingFlowClass,
    pub initiator_class: InitiatorClass,
    pub portability_honesty_class: PortabilityHonestyClass,
    pub affected_workspace_count: u32,
    pub disposed_row_count: u32,
    pub retained_row_count: u32,
    pub total_disposed_bytes: u64,
    pub total_retained_bytes: u64,
    pub continuity_warning_count: u32,
    pub protected_retained_family_count: u32,
    pub export_option_count: u32,
    pub guardrail_notice_count: u32,
    pub reviewed_away_row_count: u32,
}

impl OffboardingContinuitySupportExportRow {
    fn from_plan(plan: &OffboardingContinuityPlan) -> Self {
        Self {
            record_kind: M5_OFFBOARDING_CONTINUITY_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            plan_id: plan.plan_id.clone(),
            offboarding_flow_class: plan.offboarding_flow_class,
            initiator_class: plan.initiator_class,
            portability_honesty_class: plan.portability_honesty_class,
            affected_workspace_count: plan.affected_workspaces.len() as u32,
            disposed_row_count: plan.disposed_rows.len() as u32,
            retained_row_count: plan.retained_rows.len() as u32,
            total_disposed_bytes: plan.total_disposed_bytes,
            total_retained_bytes: plan.total_retained_bytes,
            continuity_warning_count: plan.continuity_warnings.len() as u32,
            protected_retained_family_count: plan.protected_families_retained.len() as u32,
            export_option_count: plan.export_before_delete_options.len() as u32,
            guardrail_notice_count: plan.guardrail_notices.len() as u32,
            reviewed_away_row_count: plan.all_rows().filter(|row| row.reviewed_away).count() as u32,
        }
    }
}

/// The metadata-safe support-export envelope folded from the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingContinuitySupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub envelope_id: String,
    pub captured_at: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub matrix_ref: String,
    pub runtime_storage_classes_ref: String,
    pub plan_count: u32,
    pub protected_retained_family_count: u32,
    pub durable_disposed_plan_count: u32,
    pub continuity_warning_plan_count: u32,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub plans: Vec<OffboardingContinuitySupportExportRow>,
}

impl OffboardingContinuitySupportExport {
    /// True when the envelope is metadata-safe and plan-complete.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported
            && self.redaction_class == METADATA_SAFE_DEFAULT
            && self.plans.len() as u32 == self.plan_count
    }
}

// --------------------------------------------------------------------------
// Violations.
// --------------------------------------------------------------------------

/// A validation violation surfaced by the offboarding-continuity harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OffboardingContinuityViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

impl fmt::Display for M5OffboardingContinuityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.check_id, self.target_ref, self.message
        )
    }
}

fn push(
    violations: &mut Vec<M5OffboardingContinuityViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(M5OffboardingContinuityViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}

// --------------------------------------------------------------------------
// Seeded scenario requests.
//
// These are the canonical inputs the checked-in fixtures are composed from. They
// cover account offboarding, device reset, a sign-out cleanup that keeps offline
// / certified / policy pins, an offboarding that reviews an offline pack away,
// and a workspace wipe that reviews evidence and recovery state away after an
// export.
// --------------------------------------------------------------------------

fn ws(scope_ref: &str, label: &str) -> WorkspaceScope {
    WorkspaceScope {
        scope_ref: scope_ref.to_owned(),
        label: label.to_owned(),
    }
}

fn account_offboarding_request() -> OffboardingContinuityRequest {
    OffboardingContinuityRequest {
        plan_id: "offboarding_continuity.account_offboarding_durable_retained.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Account offboarding for Project Alpha".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::AccountOffboarding,
        initiator_class: InitiatorClass::LocalUser,
        workspaces: vec![ws("ws.alpha", "Project Alpha")],
        selections: vec![
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::GeneratedPreview,
                workspace_scope_ref: "ws.alpha".to_owned(),
                workspace_label: "Project Alpha".to_owned(),
                total_bytes: 1_200_000_000,
                requested_disposal: true,
                reviewed_away: false,
                pin_source_classes: vec![],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::DocsPack,
                workspace_scope_ref: "ws.alpha".to_owned(),
                workspace_label: "Project Alpha".to_owned(),
                total_bytes: 540_000_000,
                requested_disposal: false,
                reviewed_away: false,
                pin_source_classes: vec![PinSourceClass::OfflineBundleRef],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::ReviewIncidentEvidence,
                workspace_scope_ref: "ws.alpha".to_owned(),
                workspace_label: "Project Alpha".to_owned(),
                total_bytes: 420_000_000,
                requested_disposal: false,
                reviewed_away: false,
                pin_source_classes: vec![PinSourceClass::ReviewPackRef],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                workspace_scope_ref: "ws.alpha".to_owned(),
                workspace_label: "Project Alpha".to_owned(),
                total_bytes: 95_000_000,
                requested_disposal: false,
                reviewed_away: false,
                pin_source_classes: vec![PinSourceClass::ExplicitUserPin],
            },
        ],
        note: String::new(),
    }
}

fn device_reset_request() -> OffboardingContinuityRequest {
    OffboardingContinuityRequest {
        plan_id: "offboarding_continuity.device_reset_caches_only.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Device reset for Project Beta".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::DeviceReset,
        initiator_class: InitiatorClass::LocalUser,
        workspaces: vec![ws("ws.beta", "Project Beta")],
        selections: vec![
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::GeneratedPreview,
                workspace_scope_ref: "ws.beta".to_owned(),
                workspace_label: "Project Beta".to_owned(),
                total_bytes: 800_000_000,
                requested_disposal: true,
                reviewed_away: false,
                pin_source_classes: vec![],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::NotebookOutput,
                workspace_scope_ref: "ws.beta".to_owned(),
                workspace_label: "Project Beta".to_owned(),
                total_bytes: 1_100_000_000,
                requested_disposal: true,
                reviewed_away: false,
                pin_source_classes: vec![],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::PrebuildLayer,
                workspace_scope_ref: "ws.beta".to_owned(),
                workspace_label: "Project Beta".to_owned(),
                total_bytes: 3_200_000_000,
                requested_disposal: true,
                reviewed_away: false,
                pin_source_classes: vec![],
            },
        ],
        note: String::new(),
    }
}

fn offline_certified_policy_request() -> OffboardingContinuityRequest {
    OffboardingContinuityRequest {
        plan_id: "offboarding_continuity.offline_certified_policy_pins_retained.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Sign-out cleanup for Project Gamma".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::SignOutCleanup,
        initiator_class: InitiatorClass::AdminOrTenantPolicy,
        workspaces: vec![ws("ws.gamma", "Project Gamma")],
        selections: vec![
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::GeneratedPreview,
                workspace_scope_ref: "ws.gamma".to_owned(),
                workspace_label: "Project Gamma".to_owned(),
                total_bytes: 600_000_000,
                requested_disposal: true,
                reviewed_away: false,
                pin_source_classes: vec![],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::DocsPack,
                workspace_scope_ref: "ws.gamma".to_owned(),
                workspace_label: "Project Gamma".to_owned(),
                total_bytes: 540_000_000,
                requested_disposal: false,
                reviewed_away: false,
                pin_source_classes: vec![PinSourceClass::OfflineBundleRef],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::TemplatePack,
                workspace_scope_ref: "ws.gamma".to_owned(),
                workspace_label: "Project Gamma".to_owned(),
                total_bytes: 210_000_000,
                requested_disposal: false,
                reviewed_away: false,
                pin_source_classes: vec![PinSourceClass::CertifiedArchetypeOrTemplateRef],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::ReviewIncidentEvidence,
                workspace_scope_ref: "ws.gamma".to_owned(),
                workspace_label: "Project Gamma".to_owned(),
                total_bytes: 510_000_000,
                requested_disposal: false,
                reviewed_away: false,
                pin_source_classes: vec![PinSourceClass::PolicyBundleLastKnownGoodRef],
            },
        ],
        note: String::new(),
    }
}

fn offline_reviewed_away_request() -> OffboardingContinuityRequest {
    OffboardingContinuityRequest {
        plan_id: "offboarding_continuity.offline_bundle_reviewed_away_continuity_warned.v1"
            .to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Offboarding that reviews offline packs away for Project Delta".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::AccountOffboarding,
        initiator_class: InitiatorClass::OffboardingWorkflow,
        workspaces: vec![ws("ws.delta", "Project Delta")],
        selections: vec![
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::GeneratedPreview,
                workspace_scope_ref: "ws.delta".to_owned(),
                workspace_label: "Project Delta".to_owned(),
                total_bytes: 300_000_000,
                requested_disposal: true,
                reviewed_away: false,
                pin_source_classes: vec![],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::DocsPack,
                workspace_scope_ref: "ws.delta".to_owned(),
                workspace_label: "Project Delta".to_owned(),
                total_bytes: 540_000_000,
                requested_disposal: true,
                reviewed_away: true,
                pin_source_classes: vec![PinSourceClass::OfflineBundleRef],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::TemplatePack,
                workspace_scope_ref: "ws.delta".to_owned(),
                workspace_label: "Project Delta".to_owned(),
                total_bytes: 210_000_000,
                requested_disposal: true,
                reviewed_away: true,
                pin_source_classes: vec![PinSourceClass::CertifiedArchetypeOrTemplateRef],
            },
        ],
        note: String::new(),
    }
}

fn workspace_wipe_request() -> OffboardingContinuityRequest {
    OffboardingContinuityRequest {
        plan_id: "offboarding_continuity.workspace_wipe_reviewed_away_export_first.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Workspace wipe for Project Epsilon".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::WorkspaceWipe,
        initiator_class: InitiatorClass::LocalUser,
        workspaces: vec![ws("ws.epsilon", "Project Epsilon")],
        selections: vec![
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::GeneratedPreview,
                workspace_scope_ref: "ws.epsilon".to_owned(),
                workspace_label: "Project Epsilon".to_owned(),
                total_bytes: 250_000_000,
                requested_disposal: true,
                reviewed_away: false,
                pin_source_classes: vec![],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::ReviewIncidentEvidence,
                workspace_scope_ref: "ws.epsilon".to_owned(),
                workspace_label: "Project Epsilon".to_owned(),
                total_bytes: 420_000_000,
                requested_disposal: true,
                reviewed_away: true,
                pin_source_classes: vec![PinSourceClass::ReviewPackRef],
            },
            OffboardingFamilySelection {
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                workspace_scope_ref: "ws.epsilon".to_owned(),
                workspace_label: "Project Epsilon".to_owned(),
                total_bytes: 140_000_000,
                requested_disposal: true,
                reviewed_away: true,
                pin_source_classes: vec![PinSourceClass::ExplicitUserPin],
            },
        ],
        note: String::new(),
    }
}
