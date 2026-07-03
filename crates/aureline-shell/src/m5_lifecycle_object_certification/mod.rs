//! Canonical lifecycle-object certification for every long-lived M5 object family.
//!
//! The [frozen lifecycle matrix][matrix] already binds each long-lived M5 object family — the
//! workspace, the extension, the remote session, the collaboration session, the AI action, the
//! update/rollback, the notebook runtime, the request/API run, the preview session, the pipeline
//! run, the data session, the profiler capture, and the companion session — to an explicit state
//! machine, one visible primary status surface, one exportable status code, one controlled
//! last-failure reason, and one named recovery affordance, and it names the protected journeys
//! that must show milestone checkpoints instead of anonymous spinners. This lane is the
//! **certification capstone** over that matrix: for every governed object family it certifies
//! that the four user-facing lifecycle bindings the matrix promises actually hold across every
//! consumer surface — that the object **binds its state to one primary status surface**, **exports
//! one stable status code**, **reports one controlled last-failure reason**, and **surfaces one
//! named recovery affordance** — and that the same state-truth vocabulary survives a headless or
//! companion-adjacent execution rather than degrading into a surface-specific heuristic.
//!
//! Three records carry the truth:
//!
//! - the per-family **certification row** ([`LifecycleObjectRow`]): one row per
//!   [`M5LifecycleObjectFamily`] naming the object bindings it certifies (pulled from the matrix),
//!   its status-surface / status-code / last-failure-reason / recovery-affordance posture, whether
//!   the same vocabulary survives headless/companion-adjacent execution, the consumer surfaces it
//!   evaluated, any active waiver, and a derived green/yellow/red [`LifecycleObjectStatus`].
//! - the release **certification packet** ([`LifecycleObjectPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   object causes ([`LifecycleObjectCause`]), and the blocking findings the lane refuses to ship
//!   with.
//! - the **certification dashboard** ([`LifecycleObjectDashboard`]): a light projection the
//!   product UI / CLI / diagnostics / support / telemetry automation reads to auto-narrow a
//!   governed object family's lifecycle claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment
//! an object relocates its primary status surface behind a disclosed, waivered fallback, exports a
//! disclosed partial status code, discloses a generic (rather than fully specific) last-failure
//! reason class, or offers a disclosed reduced recovery affordance; it drops to `red` if an object
//! loses or splits its primary status surface, its status code stops exporting, its last-failure
//! reason goes missing or raw, its named recovery affordance disappears, the same state-truth
//! vocabulary is lost in a headless/companion-adjacent execution, or the row fails to certify every
//! consumer surface the matrix declares for that family. That derivation is the auto-narrowing the
//! acceptance criteria require, and the consumer-surface completeness check is the lint that
//! prevents a certification from silently regressing into a partial, single-surface view — the
//! exact regression that would force support and diagnostics back onto surface-specific heuristics.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The object family, state, primary-status-surface,
//! recovery-affordance, last-failure-reason, consumer-surface, downgrade-trigger, and qualification
//! vocabulary is re-exported by reference from the already frozen [matrix], and every object
//! binding is pulled straight from that matrix's seeded packet, so this lane mints no parallel
//! lifecycle vocabulary and cannot certify a family — or a binding — the matrix does not freeze.
//! Only the certification-specific vocabulary ([`M5LifecycleBinding`], [`LifecycleObjectStatus`],
//! [`StatusSurfaceBindingState`], [`StatusCodeExportState`], [`LastFailureReasonState`],
//! [`RecoveryAffordanceBindingState`], [`LifecycleObjectWaiver`], [`LifecycleObjectCause`],
//! [`LifecycleObjectFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix as matrix;

pub use matrix::{
    M5LastFailureReasonClass, M5LifecycleConsumerSurface, M5LifecycleDowngradeTrigger,
    M5LifecycleObjectFamily, M5LifecycleQualificationClass, M5LifecycleState,
    M5PrimaryStatusSurface, M5RecoveryAffordance,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_lifecycle_object_certification_packet,
    seeded_m5_lifecycle_object_certification_packet_companion_recovery_missing_blocked,
    seeded_m5_lifecycle_object_certification_packet_data_last_failure_missing_blocked,
    seeded_m5_lifecycle_object_certification_packet_extension_headless_parity_lost_blocked,
    seeded_m5_lifecycle_object_certification_packet_notebook_status_surface_missing_blocked,
    seeded_m5_lifecycle_object_certification_packet_request_status_code_unexportable_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "lifecycle:m5_lifecycle_object_certification:v1";

/// Stable record kind for [`LifecycleObjectPacket`] payloads.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_object_certification_packet_record";

/// Stable record kind for [`LifecycleObjectDashboard`] payloads.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_DASHBOARD_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_object_certification_dashboard_record";

/// Stable record kind for [`LifecycleObjectSupportExport`] payloads.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "lifecycle_m5_lifecycle_object_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_PACKET_ID: &str =
    "m5-lifecycle-object-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_DASHBOARD_ID: &str =
    "m5-lifecycle-object-certification-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-lifecycle-object-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_SOURCE_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-lifecycle-object-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_REPORT_REF: &str =
    "artifacts/lifecycle/m5-lifecycle-object-certification.md";

/// Published certification-packet artifact ref.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-lifecycle-object-certification-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-lifecycle-object-certification-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-lifecycle-object-certification-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-lifecycle-object-certification-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_DOC_REF: &str =
    "docs/lifecycle/m5_lifecycle_object_certification_contract.md";

/// Repo-relative ref to the frozen lifecycle object-state schema.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_OBJECT_STATE_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF;

/// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_JOURNEY_CHECKPOINT_SCHEMA_REF: &str =
    matrix::M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF;

/// Frozen lifecycle-matrix contract doc this proof mirrors.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_MATRIX_DOC_REF: &str =
    matrix::M5_LIFECYCLE_MATRIX_DOC_REF;

/// State-object inventory this proof mirrors for status-surface / status-code binding.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_STATE_OBJECT_INVENTORY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF;

/// State-class recovery reference this proof mirrors for the recovery-affordance binding.
pub const M5_LIFECYCLE_OBJECT_CERTIFICATION_STATE_CLASS_RECOVERY_REF: &str =
    matrix::M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF;

/// Every governed long-lived object family the certification must cover, in canonical order.
/// These are exactly the families the frozen lifecycle matrix freezes; a certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_OBJECT_FAMILIES: [M5LifecycleObjectFamily; 13] = M5LifecycleObjectFamily::ALL;

/// Every lifecycle binding each object row certifies, in canonical order.
pub const REQUIRED_BINDINGS: [M5LifecycleBinding; 4] = M5LifecycleBinding::ALL;

/// One of the four user-facing lifecycle bindings each object row certifies.
///
/// These are exactly the four bindings the spec requires every long-lived M5 object to expose:
/// one primary in-product status surface, one exportable status code, one controlled last-failure
/// reason, and one named recovery affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleBinding {
    /// One visible primary in-product status surface.
    PrimaryStatusSurface,
    /// One stable exportable status code.
    ExportableStatusCode,
    /// One controlled last-failure reason.
    LastFailureReason,
    /// One named recovery affordance.
    NamedRecoveryAffordance,
}

impl M5LifecycleBinding {
    /// Every lifecycle binding, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PrimaryStatusSurface,
        Self::ExportableStatusCode,
        Self::LastFailureReason,
        Self::NamedRecoveryAffordance,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryStatusSurface => "primary_status_surface",
            Self::ExportableStatusCode => "exportable_status_code",
            Self::LastFailureReason => "last_failure_reason",
            Self::NamedRecoveryAffordance => "named_recovery_affordance",
        }
    }
}

/// The derived lifecycle-object-certification light an object family carries.
///
/// `green` means the object binds its state to one primary status surface, exports one stable
/// status code, reports one controlled last-failure reason, and surfaces one named recovery
/// affordance, and the same state-truth vocabulary survives a headless/companion-adjacent
/// execution across every declared consumer surface. `yellow` is a disclosed narrowing (a
/// waivered status-surface relocation, a disclosed partial status-code export, a disclosed generic
/// last-failure reason, or a disclosed reduced recovery affordance). `red` is blocked: a primary
/// status surface is lost or split, a status code stops exporting, a last-failure reason goes
/// missing or raw, a named recovery affordance disappears, the state-truth vocabulary is lost in a
/// headless/companion-adjacent execution, or the row did not certify every declared consumer
/// surface — and it may not keep a lifecycle claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleObjectStatus {
    /// Full standing: all four bindings hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl LifecycleObjectStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the object binds its lifecycle state to one visible primary status surface.
///
/// `bound_to_one_primary_surface` means the object publishes its state through exactly one visible
/// primary status surface. `disclosed_surface_relocation` means, when the object's canonical
/// surface is unavailable (for example a companion badge whose device dropped), its state is
/// relocated to a disclosed, waivered still-visible fallback surface rather than disappearing — a
/// yellow narrowing. `status_surface_missing_or_split` means the object lost its single primary
/// status surface or split it across competing surfaces — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusSurfaceBindingState {
    /// The object publishes its state through one visible primary status surface.
    BoundToOnePrimarySurface,
    /// The object relocates its state to a disclosed, waivered fallback surface.
    DisclosedSurfaceRelocation,
    /// The object lost or split its single primary status surface — a blocker.
    StatusSurfaceMissingOrSplit,
}

impl StatusSurfaceBindingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundToOnePrimarySurface => "bound_to_one_primary_surface",
            Self::DisclosedSurfaceRelocation => "disclosed_surface_relocation",
            Self::StatusSurfaceMissingOrSplit => "status_surface_missing_or_split",
        }
    }

    /// `true` when the object binds to one primary surface at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::BoundToOnePrimarySurface)
    }

    /// `true` when the object took a disclosed surface-relocation narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedSurfaceRelocation)
    }
}

/// How the object exports one stable status code across every consumer surface.
///
/// `stable_code_exports_everywhere` means the object's status code exports identically to UI, CLI,
/// docs, diagnostics, support, and telemetry. `disclosed_partial_export` means the status code
/// exports in a disclosed reduced form on a subset of surfaces (for example a headless capture that
/// exports a coarse code until finalized) while still naming the same controlled state — a yellow
/// narrowing. `status_code_unexportable` means the status code stopped exporting on an export path
/// — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusCodeExportState {
    /// The status code exports identically across every consumer surface.
    StableCodeExportsEverywhere,
    /// The status code exports in a disclosed reduced form on a subset of surfaces.
    DisclosedPartialExport,
    /// The status code stopped exporting on an export path — a blocker.
    StatusCodeUnexportable,
}

impl StatusCodeExportState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableCodeExportsEverywhere => "stable_code_exports_everywhere",
            Self::DisclosedPartialExport => "disclosed_partial_export",
            Self::StatusCodeUnexportable => "status_code_unexportable",
        }
    }

    /// `true` when the status code exports everywhere at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::StableCodeExportsEverywhere)
    }

    /// `true` when the object took a disclosed partial-export narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialExport)
    }
}

/// How the object reports one controlled last-failure reason rather than raw text.
///
/// `controlled_reason_reported` means the object always reports its last failure as one of its
/// controlled reason classes. `disclosed_generic_reason` means the object falls back to a
/// disclosed generic (still-controlled) reason class when the specific class is not yet available
/// — a yellow narrowing. `last_failure_reason_missing_or_raw` means the object dropped its
/// last-failure reason or reported raw text instead of a controlled class — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LastFailureReasonState {
    /// The object reports one controlled last-failure reason class.
    ControlledReasonReported,
    /// The object discloses a generic (still-controlled) reason class.
    DisclosedGenericReason,
    /// The object dropped its last-failure reason or reported raw text — a blocker.
    LastFailureReasonMissingOrRaw,
}

impl LastFailureReasonState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlledReasonReported => "controlled_reason_reported",
            Self::DisclosedGenericReason => "disclosed_generic_reason",
            Self::LastFailureReasonMissingOrRaw => "last_failure_reason_missing_or_raw",
        }
    }

    /// `true` when the object reports a controlled reason at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::ControlledReasonReported)
    }

    /// `true` when the object took a disclosed generic-reason narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedGenericReason)
    }
}

/// How the object surfaces one named recovery affordance for its degraded states.
///
/// `named_recovery_present` means the object always names the single recovery action a user can
/// take. `disclosed_reduced_recovery` means the object offers a disclosed reduced recovery (for
/// example a rebuild affordance that requires a dependency to return first) while still naming a
/// path forward — a yellow narrowing. `recovery_affordance_missing` means the object lost its named
/// recovery affordance, leaving a degraded state with no user action — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAffordanceBindingState {
    /// The object names one recovery affordance for its degraded states.
    NamedRecoveryPresent,
    /// The object offers a disclosed reduced recovery affordance.
    DisclosedReducedRecovery,
    /// The object lost its named recovery affordance — a blocker.
    RecoveryAffordanceMissing,
}

impl RecoveryAffordanceBindingState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedRecoveryPresent => "named_recovery_present",
            Self::DisclosedReducedRecovery => "disclosed_reduced_recovery",
            Self::RecoveryAffordanceMissing => "recovery_affordance_missing",
        }
    }

    /// `true` when the object names one recovery affordance at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::NamedRecoveryPresent)
    }

    /// `true` when the object took a disclosed reduced-recovery narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedRecovery)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow)
/// rather than blocked — never lets a lost status surface, an unexportable status code, a missing
/// last-failure reason, or a missing recovery affordance hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleObjectWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The object family the waiver applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl LifecycleObjectWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked an object family's certification.
///
/// The trigger token mirrors the frozen [`M5LifecycleDowngradeTrigger`] vocabulary so a cause
/// never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleObjectCause {
    /// The object family the cause applies to.
    pub object_family: M5LifecycleObjectFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5LifecycleDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause
    /// is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl LifecycleObjectCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed long-lived object family, certified across its status-surface, status-code,
/// last-failure-reason, and recovery-affordance bindings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleObjectRow {
    /// The object family being certified.
    pub object_family: M5LifecycleObjectFamily,
    /// Short reviewer-facing object label.
    pub object_label: String,
    /// Qualification class the matrix earned for this object.
    pub qualification: M5LifecycleQualificationClass,
    /// Owner role accountable for keeping this object governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short scope summary. Pulled from the matrix.
    pub scope_summary: String,
    /// The single visible primary status surface the object binds to. Pulled from the matrix.
    pub primary_status_surface: M5PrimaryStatusSurface,
    /// The one exportable status-code field name. Pulled from the matrix.
    pub status_code_export_field: String,
    /// The one last-failure-reason field name. Pulled from the matrix.
    pub last_failure_reason_field: String,
    /// The one named recovery affordance. Pulled from the matrix.
    pub recovery_affordance: M5RecoveryAffordance,
    /// Consumer surfaces the matrix declares this object must project to.
    pub required_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
    /// Status-surface binding posture.
    pub status_surface_binding: StatusSurfaceBindingState,
    /// Status-code export posture.
    pub status_code_export: StatusCodeExportState,
    /// Last-failure-reason posture.
    pub last_failure_reason: LastFailureReasonState,
    /// Recovery-affordance binding posture.
    pub recovery_affordance_binding: RecoveryAffordanceBindingState,
    /// `true` when the same state-truth vocabulary survives a headless or companion-adjacent
    /// execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to this object. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    /// Active waiver, when a disclosed status-surface relocation is in force.
    pub active_waiver: Option<LifecycleObjectWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: LifecycleObjectStatus,
    /// The exact object causes that narrowed or blocked this row.
    pub object_causes: Vec<LifecycleObjectCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl LifecycleObjectRow {
    /// `true` when the row certified every consumer surface the matrix declares for this object —
    /// no declared surface is left uncertified and none is invented.
    pub fn consumer_surfaces_complete(&self) -> bool {
        let mut evaluated: Vec<&str> = self
            .evaluated_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = self
            .required_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        evaluated.sort_unstable();
        required.sort_unstable();
        !required.is_empty() && evaluated == required
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.status_surface_binding,
            StatusSurfaceBindingState::StatusSurfaceMissingOrSplit
        ) {
            return true;
        }
        if matches!(
            self.status_code_export,
            StatusCodeExportState::StatusCodeUnexportable
        ) {
            return true;
        }
        if matches!(
            self.last_failure_reason,
            LastFailureReasonState::LastFailureReasonMissingOrRaw
        ) {
            return true;
        }
        if matches!(
            self.recovery_affordance_binding,
            RecoveryAffordanceBindingState::RecoveryAffordanceMissing
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.status_surface_binding.is_disclosed_narrowing()
            || self.status_code_export.is_disclosed_narrowing()
            || self.last_failure_reason.is_disclosed_narrowing()
            || self.recovery_affordance_binding.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the object posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> LifecycleObjectStatus {
        if self.has_hard_blocker() {
            LifecycleObjectStatus::Red
        } else if self.has_narrowing() {
            LifecycleObjectStatus::Yellow
        } else {
            LifecycleObjectStatus::Green
        }
    }

    /// Recomputes the exact object causes for the row, in deterministic order (status-surface,
    /// status-code, last-failure-reason, recovery-affordance, then headless parity).
    pub fn recompute_causes(&self) -> Vec<LifecycleObjectCause> {
        let mut causes = Vec::new();
        match self.status_surface_binding {
            StatusSurfaceBindingState::BoundToOnePrimarySurface => {}
            StatusSurfaceBindingState::DisclosedSurfaceRelocation => {
                causes.push(LifecycleObjectCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object's canonical primary status surface is unavailable, so its \
                             state is relocated to a disclosed, waivered still-visible fallback \
                             surface rather than disappearing; the relocation is disclosed and the \
                             single-surface binding is restored when the dependency returns."
                        .to_owned(),
                });
            }
            StatusSurfaceBindingState::StatusSurfaceMissingOrSplit => {
                causes.push(LifecycleObjectCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StatusSurfaceMissing,
                    disclosed: false,
                    detail:
                        "The object lost its single visible primary status surface or split its \
                             state across competing surfaces, so users can no longer read one \
                             authoritative status."
                            .to_owned(),
                });
            }
        }
        match self.status_code_export {
            StatusCodeExportState::StableCodeExportsEverywhere => {}
            StatusCodeExportState::DisclosedPartialExport => {
                causes.push(LifecycleObjectCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object's status code exports in a disclosed reduced form on a \
                             subset of surfaces while still naming the same controlled state, so the \
                             export is narrowed and disclosed rather than lost."
                        .to_owned(),
                });
            }
            StatusCodeExportState::StatusCodeUnexportable => {
                causes.push(LifecycleObjectCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::StatusCodeUnexportable,
                    disclosed: false,
                    detail: "The object's stable status code stopped exporting on an export path, so \
                             support, CLI, or telemetry can no longer read the same code the UI shows."
                        .to_owned(),
                });
            }
        }
        match self.last_failure_reason {
            LastFailureReasonState::ControlledReasonReported => {}
            LastFailureReasonState::DisclosedGenericReason => {
                causes.push(LifecycleObjectCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The object falls back to a disclosed generic but still-controlled \
                             last-failure reason class when the specific class is not yet available, \
                             so the reason is narrowed and disclosed rather than raw or missing."
                        .to_owned(),
                });
            }
            LastFailureReasonState::LastFailureReasonMissingOrRaw => {
                causes.push(LifecycleObjectCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::LastFailureReasonMissing,
                    disclosed: false,
                    detail:
                        "The object dropped its controlled last-failure reason or reported raw \
                             text instead of a controlled reason class, so support and diagnostics \
                             fall back to surface-specific heuristics."
                            .to_owned(),
                });
            }
        }
        match self.recovery_affordance_binding {
            RecoveryAffordanceBindingState::NamedRecoveryPresent => {}
            RecoveryAffordanceBindingState::DisclosedReducedRecovery => {
                causes.push(LifecycleObjectCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail:
                        "The object offers a disclosed reduced recovery affordance while still \
                             naming a path forward, so recovery is narrowed and disclosed rather \
                             than absent."
                            .to_owned(),
                });
            }
            RecoveryAffordanceBindingState::RecoveryAffordanceMissing => {
                causes.push(LifecycleObjectCause {
                    object_family: self.object_family,
                    trigger: M5LifecycleDowngradeTrigger::RecoveryAffordanceMissing,
                    disclosed: false,
                    detail:
                        "The object lost its named recovery affordance, leaving a degraded state \
                             with no named user action to take."
                            .to_owned(),
                });
            }
        }
        if !self.headless_parity_preserved {
            causes.push(LifecycleObjectCause {
                object_family: self.object_family,
                trigger: M5LifecycleDowngradeTrigger::StateVocabularyDrift,
                disclosed: false,
                detail:
                    "A headless or companion-adjacent execution of this object lost the shared \
                         state-truth vocabulary, so the same object reports a different state \
                         language depending on how it runs."
                        .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed status-surface relocation may only stay yellow (rather than red) when a waiver
    /// discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.status_surface_binding,
            StatusSurfaceBindingState::DisclosedSurfaceRelocation
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<LifecycleObjectFinding> {
        let mut findings = Vec::new();
        let object = self.object_family.as_str().to_owned();

        if !self.consumer_surfaces_complete() {
            findings.push(LifecycleObjectFinding::ConsumerSurfacesIncomplete {
                object: object.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(LifecycleObjectFinding::HeadlessParityLost {
                object: object.clone(),
            });
        }
        if matches!(
            self.status_surface_binding,
            StatusSurfaceBindingState::StatusSurfaceMissingOrSplit
        ) {
            findings.push(LifecycleObjectFinding::StatusSurfaceMissingOrSplit {
                object: object.clone(),
            });
        }
        if matches!(
            self.status_code_export,
            StatusCodeExportState::StatusCodeUnexportable
        ) {
            findings.push(LifecycleObjectFinding::StatusCodeUnexportable {
                object: object.clone(),
            });
        }
        if matches!(
            self.last_failure_reason,
            LastFailureReasonState::LastFailureReasonMissingOrRaw
        ) {
            findings.push(LifecycleObjectFinding::LastFailureReasonMissingOrRaw {
                object: object.clone(),
            });
        }
        if matches!(
            self.recovery_affordance_binding,
            RecoveryAffordanceBindingState::RecoveryAffordanceMissing
        ) {
            findings.push(LifecycleObjectFinding::RecoveryAffordanceMissing {
                object: object.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, LifecycleObjectStatus::Green) && !self.has_reason() {
            findings.push(LifecycleObjectFinding::NarrowedRowWithoutReason {
                object: object.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(LifecycleObjectFinding::NarrowedRowWithoutWaiver {
                object: object.clone(),
            });
        }
        // An attached waiver must still be active and must point at this object family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.object_family != self.object_family {
                findings.push(LifecycleObjectFinding::WaiverObjectMismatch {
                    object: object.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(LifecycleObjectFinding::WaiverExpired {
                    object: object.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(LifecycleObjectFinding::RowStatusStale {
                object: object.clone(),
            });
        }
        if self.object_causes != self.recompute_causes() {
            findings.push(LifecycleObjectFinding::RowCausesStale { object });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} surface={} code={} reason={} recovery={} headless={} surfaces={} waiver={}",
            self.object_family.as_str(),
            self.derived_status.as_str(),
            self.status_surface_binding.as_str(),
            self.status_code_export.as_str(),
            self.last_failure_reason.as_str(),
            self.recovery_affordance_binding.as_str(),
            self.headless_parity_preserved,
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the lifecycle-object certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum LifecycleObjectFinding {
    /// A governed object family has no certification row.
    ObjectMissing {
        /// The missing object token.
        object: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The object token.
        object: String,
    },
    /// A headless/companion-adjacent execution lost the shared state-truth vocabulary.
    HeadlessParityLost {
        /// The object token.
        object: String,
    },
    /// The object lost or split its single primary status surface.
    StatusSurfaceMissingOrSplit {
        /// The object token.
        object: String,
    },
    /// The object's stable status code stopped exporting.
    StatusCodeUnexportable {
        /// The object token.
        object: String,
    },
    /// The object dropped its controlled last-failure reason or reported raw text.
    LastFailureReasonMissingOrRaw {
        /// The object token.
        object: String,
    },
    /// The object lost its named recovery affordance.
    RecoveryAffordanceMissing {
        /// The object token.
        object: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The object token.
        object: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The object token.
        object: String,
    },
    /// An attached waiver does not point at the row's object family.
    WaiverObjectMismatch {
        /// The object token.
        object: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The object token.
        object: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The object token.
        object: String,
    },
    /// The declared object causes do not match the recomputed causes.
    RowCausesStale {
        /// The object token.
        object: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered object families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl LifecycleObjectFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ObjectMissing { .. } => "object_missing",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::StatusSurfaceMissingOrSplit { .. } => "status_surface_missing_or_split",
            Self::StatusCodeUnexportable { .. } => "status_code_unexportable",
            Self::LastFailureReasonMissingOrRaw { .. } => "last_failure_reason_missing_or_raw",
            Self::RecoveryAffordanceMissing { .. } => "recovery_affordance_missing",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverObjectMismatch { .. } => "waiver_object_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ObjectMissing { object }
            | Self::ConsumerSurfacesIncomplete { object }
            | Self::HeadlessParityLost { object }
            | Self::StatusSurfaceMissingOrSplit { object }
            | Self::StatusCodeUnexportable { object }
            | Self::LastFailureReasonMissingOrRaw { object }
            | Self::RecoveryAffordanceMissing { object }
            | Self::NarrowedRowWithoutReason { object }
            | Self::NarrowedRowWithoutWaiver { object }
            | Self::WaiverObjectMismatch { object, .. }
            | Self::WaiverExpired { object, .. }
            | Self::RowStatusStale { object }
            | Self::RowCausesStale { object } => object,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release lifecycle-object-certification packet shared by the product UI / CLI / diagnostics /
/// support / telemetry automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleObjectPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen lifecycle matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen lifecycle object-state schema.
    pub object_state_schema_ref: String,
    /// Repo-relative ref to the frozen lifecycle journey-checkpoint schema.
    pub journey_checkpoint_schema_ref: String,
    /// Frozen lifecycle-matrix contract doc this proof mirrors.
    pub matrix_doc_ref: String,
    /// State-object inventory this proof mirrors for status-surface / status-code binding.
    pub state_object_inventory_ref: String,
    /// State-class recovery reference this proof mirrors for the recovery-affordance binding.
    pub state_class_recovery_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four lifecycle bindings every object row certifies.
    pub required_bindings: Vec<String>,
    /// The thirteen governed object families the certification must cover.
    pub required_object_families: Vec<String>,
    /// Per-family certification rows, in canonical order.
    pub rows: Vec<LifecycleObjectRow>,
    /// Object families certified, in canonical (sorted) order.
    pub covered_object_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-binding) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<LifecycleObjectWaiver>,
    /// Every exact object cause, in row then cause order.
    pub object_causes: Vec<LifecycleObjectCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<LifecycleObjectFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Lifecycle / release automation refs that consume this packet to auto-narrow governed
    /// objects.
    pub lifecycle_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl LifecycleObjectPacket {
    /// Returns the certification row for `object_family`, if present.
    pub fn row(&self, object_family: M5LifecycleObjectFamily) -> Option<&LifecycleObjectRow> {
        self.rows
            .iter()
            .find(|row| row.object_family == object_family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.object_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.object_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.object_family.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the lifecycle automation consumes.
    pub fn dashboard(&self) -> LifecycleObjectDashboard {
        LifecycleObjectDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 lifecycle-object-certification packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per object family naming its
    /// status, the four binding postures, headless parity, the evaluated-surface count, and the
    /// waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_family,status,status_surface_binding,status_code_export,last_failure_reason,recovery_affordance_binding,headless_parity,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.object_family.as_str(),
                row.derived_status.as_str(),
                row.status_surface_binding.as_str(),
                row.status_code_export.as_str(),
                row.last_failure_reason.as_str(),
                row.recovery_affordance_binding.as_str(),
                row.headless_parity_preserved,
                row.evaluated_consumer_surfaces.len(),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 lifecycle-object certification: status-surface, status-code, last-failure-reason, and recovery-affordance truth on every long-lived M5 object\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_lifecycle_object_certification`](../../crates/aureline-shell/src/m5_lifecycle_object_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_object_certification -- markdown > \\\n  artifacts/lifecycle/m5-lifecycle-object-certification.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!(
            "- Required bindings: {}\n",
            self.required_bindings
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Object families certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full binding): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Object family | Status | Status surface | Status code | Last-failure reason | Recovery affordance | Headless | Waiver |\n\
             | ------------- | ------ | -------------- | ----------- | ------------------- | ------------------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.object_label,
                row.derived_status.as_str(),
                row.status_surface_binding.as_str(),
                row.status_code_export.as_str(),
                row.last_failure_reason.as_str(),
                row.recovery_affordance_binding.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&LifecycleObjectRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, LifecycleObjectStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every long-lived M5 object binds one primary status surface, exports one stable status code, reports one controlled last-failure reason, and surfaces one named recovery affordance across every declared consumer surface.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.object_family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact object causes\n\n");
        if self.object_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.object_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.object_family.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.object_family.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_object_certification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_lifecycle_object_certification_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleObjectDashboardRow {
    /// The object family.
    pub object_family: M5LifecycleObjectFamily,
    /// Short object label.
    pub object_label: String,
    /// Derived green/yellow/red status.
    pub status: LifecycleObjectStatus,
    /// Number of declared consumer surfaces certified for this object.
    pub evaluated_surface_count: usize,
    /// Status-surface binding posture.
    pub status_surface_binding: StatusSurfaceBindingState,
    /// Status-code export posture.
    pub status_code_export: StatusCodeExportState,
    /// Last-failure-reason posture.
    pub last_failure_reason: LastFailureReasonState,
    /// Recovery-affordance binding posture.
    pub recovery_affordance_binding: RecoveryAffordanceBindingState,
    /// `true` when headless/companion-adjacent parity is preserved.
    pub headless_parity_preserved: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the product UI / CLI / diagnostics / support / telemetry
/// automation reads to auto-narrow a governed object family's lifecycle claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleObjectDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<LifecycleObjectDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Lifecycle / release automation refs that consume the dashboard.
    pub lifecycle_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl LifecycleObjectDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &LifecycleObjectPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| LifecycleObjectDashboardRow {
                object_family: row.object_family,
                object_label: row.object_label.clone(),
                status: row.derived_status,
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                status_surface_binding: row.status_surface_binding,
                status_code_export: row.status_code_export,
                last_failure_reason: row.last_failure_reason,
                recovery_affordance_binding: row.recovery_affordance_binding,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .object_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_LIFECYCLE_OBJECT_CERTIFICATION_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_OBJECT_CERTIFICATION_SCHEMA_VERSION,
            dashboard_id: M5_LIFECYCLE_OBJECT_CERTIFICATION_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            lifecycle_automation_refs: packet.lifecycle_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 lifecycle-object-certification dashboard serializes")
    }
}

/// Support-export wrapper for the lifecycle-object-certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleObjectSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: LifecycleObjectPacket,
    /// Dashboard quoted in full.
    pub dashboard: LifecycleObjectDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl LifecycleObjectSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each object family, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the lifecycle automation
    /// — can name the same object and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: LifecycleObjectPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.object_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_LIFECYCLE_OBJECT_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_OBJECT_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_lifecycle_object_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleObjectInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen lifecycle matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family certification rows.
    pub rows: Vec<LifecycleObjectRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The certification packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`LifecycleObjectPacket`] from the exact build identity, the frozen matrix ref, and
/// the per-family certification rows.
///
/// Each row's derived status and object causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_lifecycle_object_certification_packet(
    input: LifecycleObjectInput,
) -> LifecycleObjectPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<LifecycleObjectRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.object_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<LifecycleObjectFinding> = Vec::new();

    // Every governed object family must carry a certification row.
    let present: BTreeSet<M5LifecycleObjectFamily> =
        rows.iter().map(|row| row.object_family).collect();
    for object_family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&object_family) {
            blocking_findings.push(LifecycleObjectFinding::ObjectMissing {
                object: object_family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_object_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|object_family| object_family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, LifecycleObjectStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, LifecycleObjectStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, LifecycleObjectStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(LifecycleObjectFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<LifecycleObjectWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let object_causes: Vec<LifecycleObjectCause> = rows
        .iter()
        .flat_map(|row| row.object_causes.clone())
        .collect();

    let required_bindings: Vec<String> = REQUIRED_BINDINGS
        .iter()
        .map(|binding| binding.as_str().to_owned())
        .collect();
    let required_object_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|object_family| object_family.as_str().to_owned())
        .collect();

    let mut packet = LifecycleObjectPacket {
        record_kind: M5_LIFECYCLE_OBJECT_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_LIFECYCLE_OBJECT_CERTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_LIFECYCLE_OBJECT_CERTIFICATION_PACKET_ID.to_owned(),
        source_schema_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Status-surface, status-code, last-failure-reason, and recovery-affordance truth \
                   on every long-lived M5 object: the workspace, extension, remote session, \
                   collaboration session, AI action, update/rollback, notebook runtime, request/API \
                   run, preview session, pipeline run, data session, profiler capture, and companion \
                   session each certified so the object binds its state to one visible primary \
                   status surface, exports one stable status code, reports one controlled \
                   last-failure reason, and surfaces one named recovery affordance across every \
                   declared consumer surface — with the same state-truth vocabulary preserved in \
                   headless and companion-adjacent execution — and each object's green/yellow/red \
                   claim auto-narrowed from its four binding postures."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        object_state_schema_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_OBJECT_STATE_SCHEMA_REF
            .to_owned(),
        journey_checkpoint_schema_ref:
            M5_LIFECYCLE_OBJECT_CERTIFICATION_JOURNEY_CHECKPOINT_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_MATRIX_DOC_REF.to_owned(),
        state_object_inventory_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_STATE_OBJECT_INVENTORY_REF
            .to_owned(),
        state_class_recovery_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_STATE_CLASS_RECOVERY_REF
            .to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_bindings,
        required_object_families,
        rows,
        covered_object_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        object_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        lifecycle_automation_refs: vec![
            "lifecycle_status.object_certification_registry".to_owned(),
            "release_automation.auto_narrow.lifecycle_object_certification_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.lifecycle_object_certification".to_owned(),
            M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-lifecycle-object-certification".to_owned()],
        published_report_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_DASHBOARD_REF
            .to_owned(),
        published_doc_ref: M5_LIFECYCLE_OBJECT_CERTIFICATION_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(LifecycleObjectFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_lifecycle_object_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum LifecycleObjectValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The declared required bindings do not match the lane constants.
    RequiredBindingsStale,
    /// The declared required object families do not match the lane constants.
    RequiredObjectFamiliesStale,
    /// The rows do not cover all thirteen governed object families.
    CoverageIncomplete,
    /// The declared covered object families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared object causes do not match the recomputed causes.
    ObjectCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the lifecycle-object-certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed object family
/// carries a current certification row; each row's status is the derived auto-narrowed value, never
/// asserted; a green row cannot keep a claim while a primary status surface is lost or split, a
/// status code stops exporting, a last-failure reason goes missing or raw, a named recovery
/// affordance disappears, headless/companion-adjacent parity is lost, or the row fails to certify
/// every declared consumer surface; and a disclosed narrowing is backed by a reason and, where
/// required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_lifecycle_object_certification_packet(
    packet: &LifecycleObjectPacket,
) -> Result<(), Vec<LifecycleObjectValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(LifecycleObjectValidationError::NoRows);
    }
    if packet.record_kind != M5_LIFECYCLE_OBJECT_CERTIFICATION_PACKET_RECORD_KIND {
        errors.push(LifecycleObjectValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_LIFECYCLE_OBJECT_CERTIFICATION_SCHEMA_VERSION {
        errors.push(LifecycleObjectValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(LifecycleObjectValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(LifecycleObjectValidationError::MatrixPacketRefMissing);
    }
    let expected_bindings: Vec<String> = REQUIRED_BINDINGS
        .iter()
        .map(|binding| binding.as_str().to_owned())
        .collect();
    if packet.required_bindings != expected_bindings {
        errors.push(LifecycleObjectValidationError::RequiredBindingsStale);
    }
    let expected_object_families: Vec<String> = REQUIRED_OBJECT_FAMILIES
        .iter()
        .map(|object_family| object_family.as_str().to_owned())
        .collect();
    if packet.required_object_families != expected_object_families {
        errors.push(LifecycleObjectValidationError::RequiredObjectFamiliesStale);
    }

    let present: BTreeSet<M5LifecycleObjectFamily> =
        packet.rows.iter().map(|row| row.object_family).collect();
    let coverage_complete = REQUIRED_OBJECT_FAMILIES
        .iter()
        .all(|object_family| present.contains(object_family));
    if !coverage_complete || packet.rows.len() != REQUIRED_OBJECT_FAMILIES.len() {
        errors.push(LifecycleObjectValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|object_family| object_family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_object_families {
        errors.push(LifecycleObjectValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), LifecycleObjectStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), LifecycleObjectStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), LifecycleObjectStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(LifecycleObjectValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<LifecycleObjectWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(LifecycleObjectValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<LifecycleObjectCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.object_causes {
        errors.push(LifecycleObjectValidationError::ObjectCausesStale);
    }

    let mut recomputed: Vec<LifecycleObjectFinding> = Vec::new();
    for object_family in REQUIRED_OBJECT_FAMILIES {
        if !present.contains(&object_family) {
            recomputed.push(LifecycleObjectFinding::ObjectMissing {
                object: object_family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(LifecycleObjectFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(LifecycleObjectFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(LifecycleObjectValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(LifecycleObjectValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(LifecycleObjectValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(LifecycleObjectValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(LifecycleObjectValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(LifecycleObjectValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
