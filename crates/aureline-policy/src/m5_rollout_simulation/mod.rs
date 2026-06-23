//! M5 admin-plane *dry-run rollout simulation*: the typed previews Aureline
//! shows before a policy import, promotion, bundle rollout, mirror-source change,
//! trust-root change, or route/egress expansion is allowed to widen privilege or
//! feature access on its claimed managed-cloud, self-hosted, sovereign/air-gapped,
//! and mirrored/offline profiles.
//!
//! Where [`m5_admin_plane`](crate::m5_admin_plane) *names and freezes the
//! contract* — the surface families, the shared state vocabulary, and the proof
//! packets that keep them current — and
//! [`m5_admin_render`](crate::m5_admin_render) *renders the current admin state*,
//! this lane *simulates the next state*. Every rollout scenario is a forward
//! dry-run: it shows which endpoints and features a change would move, classifies
//! the change as **tightening** (a restriction) or **widening** (new permission,
//! new egress class, new AI provider, registry-source change, or trust-root
//! change), and states the review strength, staged-rollout requirement, and
//! rollback path a promotion must clear *before* it can broaden access. Nothing
//! here applies a change: every scenario is a [`RolloutScenario::dry_run`] preview.
//!
//! Each packet *binds back to the matrix*. The simulation renders the
//! [`PolicyDiff`](crate::m5_admin_plane::AdminSurfaceClass::PolicyDiff) and
//! [`EndpointPostureCard`](crate::m5_admin_plane::AdminSurfaceClass::EndpointPostureCard)
//! surfaces forward in time, so every endpoint posture state it shows — before
//! and after — and the per-profile claim state must be one the matrix declares
//! applicable for the endpoint-posture surface
//! ([`RolloutSimulationInvariant`] `rollout_sim.surface_states_within_matrix`). An
//! edit that shows a state the matrix does not admit, or binds a surface the
//! matrix does not define, flips an invariant and fails the freeze gate.
//!
//! The honesty rules the spec requires are enforced, not just described:
//!
//! - **Widening is gated harder than tightening.** A widening or mixed change
//!   always requires review at least as strong as two-person control, a staged
//!   (never immediate) rollout, and a non-instant rollback, and it always names at
//!   least one widening dimension; a pure tightening never needs more than a single
//!   admin review (`rollout_sim.widening_requires_stronger_review`).
//! - **No silent green.** When the simulation evidence, an offline mirror's
//!   freshness, or the endpoint-posture evidence goes stale, the profile's managed
//!   claim auto-narrows off confirmed and names exactly which dimension went stale
//!   (`rollout_sim.claim_auto_narrows_on_stale`); a scenario whose own simulation
//!   evidence is stale can never read as safe-to-promote
//!   (`rollout_sim.stale_scenarios_held`).
//! - **Every scenario is a reviewable dry-run.** Each names impacted endpoints and
//!   features, a review requirement, a staged-rollout requirement, and a rollback
//!   path before any promotion (`rollout_sim.scenarios_are_reviewable_dry_runs`).
//!
//! There is exactly one typed packet per claimed managed-bearing profile, consumed
//! identically by the shell admin center, CLI/headless inspect, Help/About,
//! support export, and release evidence, so the simulated blast radius is the same
//! bytes on every surface by construction (`rollout_sim.consumer_parity`).
//!
//! The record carries no endpoint URLs, hostnames, credentials, raw provider
//! payloads, or absolute paths — only opaque object refs, stable tokens, rendered
//! metadata-safe value summaries, and short reviewable sentences — so it is safe
//! to embed in a support export verbatim.

use serde::{Deserialize, Serialize};

use crate::m5_admin_plane::{
    admin_plane_matrix, all_unique, is_export_safe_ref, AdminConsumerClass,
    AdminDeploymentProfileClass, AdminPathClass, AdminRedactionClass, AdminStateClass,
    AdminSurfaceClass, M5_ADMIN_PLANE_MATRIX_ID,
};
use crate::m5_admin_render::{EvidenceAgeClass, OwnerEscalationRoleClass};

#[cfg(test)]
mod tests;

/// Schema version for the rollout-simulation bundle.
pub const M5_ROLLOUT_SIMULATION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the rollout-simulation bundle.
pub const M5_ROLLOUT_SIMULATION_SCHEMA_REF: &str =
    "schemas/admin/m5-rollout-simulation.schema.json";

/// Stable record-kind tag for the rollout-simulation bundle.
pub const M5_ROLLOUT_SIMULATION_RECORD_KIND: &str = "m5_rollout_simulation_bundle";

/// Stable id for the canonical rollout-simulation bundle.
pub const M5_ROLLOUT_SIMULATION_BUNDLE_ID: &str = "m5-rollout-simulation:bundle:0001";

/// Evaluation stamp for the canonical bundle. Held as a constant so the binding
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ROLLOUT_SIMULATION_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The matrix this simulation layer binds back to.
pub const M5_ROLLOUT_SIMULATION_MATRIX_REF: &str =
    "fixtures/admin/m5-admin-plane/canonical_matrix.json";

/// The freeze gate that keeps the simulation bundle current.
pub const M5_ROLLOUT_SIMULATION_FREEZE_GATE_REF: &str =
    "crates/aureline-policy/tests/m5_rollout_simulation.rs";

// ---------------------------------------------------------------------------
// Token enums.
// ---------------------------------------------------------------------------

/// The change a dry-run simulates — the rollout flows the spec requires a managed
/// plane to preview before they apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutChangeKindClass {
    /// Importing a new policy bundle.
    PolicyImport,
    /// Promoting a staged bundle to enforced.
    PolicyPromotion,
    /// Rolling a bundle out to endpoints.
    BundleRollout,
    /// Changing the mirror source a profile syncs from.
    MirrorSourceChange,
    /// Changing a trust root / signing key.
    TrustRootChange,
    /// Expanding a network route or egress class.
    RouteEgressExpansion,
}

impl RolloutChangeKindClass {
    /// All change kinds, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::PolicyImport,
        Self::PolicyPromotion,
        Self::BundleRollout,
        Self::MirrorSourceChange,
        Self::TrustRootChange,
        Self::RouteEgressExpansion,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyImport => "policy_import",
            Self::PolicyPromotion => "policy_promotion",
            Self::BundleRollout => "bundle_rollout",
            Self::MirrorSourceChange => "mirror_source_change",
            Self::TrustRootChange => "trust_root_change",
            Self::RouteEgressExpansion => "route_egress_expansion",
        }
    }
}

/// Whether a change tightens (restricts) or widens (broadens) privilege or
/// feature access. Widening is the class the spec requires to clear stronger
/// review and a staged rollout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeDirectionClass {
    /// The change only restricts: it removes a permission, narrows egress, or
    /// drops a provider. The safest class.
    Tightening,
    /// The change broadens access: a new permission, egress class, AI provider,
    /// registry source, or trust root.
    Widening,
    /// The change both tightens one control and widens another; treated as
    /// widening for review purposes.
    Mixed,
    /// The change moves nothing user-visible; surfaced for completeness.
    NoEffect,
}

impl ChangeDirectionClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tightening => "tightening",
            Self::Widening => "widening",
            Self::Mixed => "mixed",
            Self::NoEffect => "no_effect",
        }
    }

    /// Whether the change broadens access, so it must clear the stronger
    /// review/staging/rollback floor.
    pub const fn is_widening(self) -> bool {
        matches!(self, Self::Widening | Self::Mixed)
    }
}

/// The specific way a widening change broadens access — the dimensions the spec
/// calls out as needing stronger review than a simple restriction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WideningDimensionClass {
    /// A new capability / permission is granted.
    NewPermission,
    /// A new network egress class is opened.
    NewEgressClass,
    /// A new AI provider is allowed.
    NewAiProvider,
    /// The registry / package / extension source changes.
    RegistrySourceChange,
    /// A trust root / signing key changes.
    TrustRootChange,
}

impl WideningDimensionClass {
    /// All widening dimensions, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::NewPermission,
        Self::NewEgressClass,
        Self::NewAiProvider,
        Self::RegistrySourceChange,
        Self::TrustRootChange,
    ];

    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NewPermission => "new_permission",
            Self::NewEgressClass => "new_egress_class",
            Self::NewAiProvider => "new_ai_provider",
            Self::RegistrySourceChange => "registry_source_change",
            Self::TrustRootChange => "trust_root_change",
        }
    }
}

/// How strongly a change must be reviewed before promotion. Ordered by strength
/// via [`ReviewRequirementClass::rank`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRequirementClass {
    /// The local user can apply it without an admin review.
    SelfServiceLocal,
    /// A single admin review is enough.
    SingleAdminReview,
    /// Two-person / dual-control review is required.
    DualControlReview,
    /// Security/compliance sign-off is required.
    SecurityComplianceReview,
    /// Blocked until a residency/tenant/key/endpoint boundary is rechecked.
    BlockedPendingBoundaryRecheck,
}

impl ReviewRequirementClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfServiceLocal => "self_service_local",
            Self::SingleAdminReview => "single_admin_review",
            Self::DualControlReview => "dual_control_review",
            Self::SecurityComplianceReview => "security_compliance_review",
            Self::BlockedPendingBoundaryRecheck => "blocked_pending_boundary_recheck",
        }
    }

    /// Numeric strength: higher is stronger. Used to assert widening is gated
    /// harder than tightening.
    pub const fn rank(self) -> u8 {
        match self {
            Self::SelfServiceLocal => 0,
            Self::SingleAdminReview => 1,
            Self::DualControlReview => 2,
            Self::SecurityComplianceReview => 3,
            Self::BlockedPendingBoundaryRecheck => 4,
        }
    }
}

/// How a change must be staged out to endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutStagingClass {
    /// May apply to all endpoints at once (only safe for restrictions).
    ImmediateAllowed,
    /// Must roll out by ring: canary, then ring, then fleet.
    StagedRingRequired,
    /// May only arrive as a manually imported, signed bundle (sovereign /
    /// air-gapped / offline).
    PinnedManualSignedOnly,
}

impl RolloutStagingClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImmediateAllowed => "immediate_allowed",
            Self::StagedRingRequired => "staged_ring_required",
            Self::PinnedManualSignedOnly => "pinned_manual_signed_only",
        }
    }

    /// Whether the change may apply to every endpoint at once.
    pub const fn is_immediate(self) -> bool {
        matches!(self, Self::ImmediateAllowed)
    }
}

/// The rollback path a change leaves behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackRequirementClass {
    /// Revert instantly to the prior local state.
    InstantLocalRevert,
    /// Revert by unwinding the staged rollout ring by ring.
    StagedRevert,
    /// Revert requires importing a signed rollback bundle (offline / pinned).
    SignedRollbackBundle,
    /// Revert requires a manual re-import and operator step.
    ManualReimportRequired,
}

impl RollbackRequirementClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstantLocalRevert => "instant_local_revert",
            Self::StagedRevert => "staged_revert",
            Self::SignedRollbackBundle => "signed_rollback_bundle",
            Self::ManualReimportRequired => "manual_reimport_required",
        }
    }

    /// Whether reverting is a free instant local action (insufficient for a
    /// widening change).
    pub const fn is_instant(self) -> bool {
        matches!(self, Self::InstantLocalRevert)
    }
}

/// What the dry-run concluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulationOutcomeClass {
    /// Safe to promote as-is.
    SafeToPromote,
    /// Promote, but only through the required staged rollout.
    PromoteWithStagedRollout,
    /// Hold for the named review before promoting.
    HoldForReview,
    /// Blocked: the simulation evidence is stale and must refresh first.
    BlockedStaleEvidence,
    /// Blocked: a boundary changed and must be rechecked first.
    BlockedBoundaryRecheck,
}

impl SimulationOutcomeClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeToPromote => "safe_to_promote",
            Self::PromoteWithStagedRollout => "promote_with_staged_rollout",
            Self::HoldForReview => "hold_for_review",
            Self::BlockedStaleEvidence => "blocked_stale_evidence",
            Self::BlockedBoundaryRecheck => "blocked_boundary_recheck",
        }
    }

    /// Whether the outcome asserts the change is safe to promote now.
    pub const fn is_safe_to_promote(self) -> bool {
        matches!(self, Self::SafeToPromote)
    }

    /// Whether the outcome blocks promotion outright.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedStaleEvidence | Self::BlockedBoundaryRecheck
        )
    }
}

/// Why a profile's managed claim auto-narrowed off confirmed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimNarrowReasonClass {
    /// The rollout-simulation evidence is past its soft-refresh window.
    SimulationEvidenceStale,
    /// The offline mirror's last sync is past its soft-refresh window.
    MirrorFreshnessStale,
    /// The endpoint-posture evidence is past its soft-refresh window.
    EndpointPostureStale,
}

impl ClaimNarrowReasonClass {
    /// Stable snake_case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SimulationEvidenceStale => "simulation_evidence_stale",
            Self::MirrorFreshnessStale => "mirror_freshness_stale",
            Self::EndpointPostureStale => "endpoint_posture_stale",
        }
    }
}

// ---------------------------------------------------------------------------
// Scenario record structs.
// ---------------------------------------------------------------------------

/// One representative endpoint a change would move, with its before/after
/// posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactedEndpoint {
    /// Opaque device/install/ring ref.
    pub endpoint_ref: String,
    /// One reviewable label.
    pub label: String,
    /// The posture before the change.
    pub posture_before: AdminStateClass,
    /// The posture the change would move it to.
    pub posture_after: AdminStateClass,
    /// One reviewable sentence of the per-endpoint impact.
    pub impact_note: String,
}

/// One feature family a change would move.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactedFeature {
    /// The affected feature family.
    pub feature_family: String,
    /// One reviewable sentence of the user-visible consequence.
    pub effect_note: String,
    /// Whether this feature's access is newly broadened by the change.
    pub newly_widened: bool,
}

/// One dry-run rollout scenario: a forward preview of a single change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// The change this scenario simulates.
    pub change_kind: RolloutChangeKindClass,
    /// Human-readable title.
    pub title: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// Whether the change tightens or widens access.
    pub direction: ChangeDirectionClass,
    /// The widening dimensions this change broadens, empty for a tightening.
    pub widening_dimensions: Vec<WideningDimensionClass>,
    /// The representative endpoints the change would move.
    pub impacted_endpoints: Vec<ImpactedEndpoint>,
    /// The feature families the change would move.
    pub impacted_features: Vec<ImpactedFeature>,
    /// The review strength required before promotion.
    pub review_requirement: ReviewRequirementClass,
    /// The staged-rollout requirement.
    pub staging: RolloutStagingClass,
    /// The rollback path the change leaves behind.
    pub rollback: RollbackRequirementClass,
    /// The freshness of this scenario's simulation evidence.
    pub simulation_freshness: EvidenceAgeClass,
    /// What the dry-run concluded.
    pub outcome: SimulationOutcomeClass,
    /// Who owns the change.
    pub owner: OwnerEscalationRoleClass,
    /// Who the change escalates to, if anyone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escalation_owner: Option<OwnerEscalationRoleClass>,
    /// The redaction rule applied to this scenario on export.
    pub redaction: AdminRedactionClass,
    /// One reviewable sentence stating the review/rollback requirement.
    pub review_note: String,
    /// Always true: a scenario simulates a change, it never applies one.
    pub dry_run: bool,
}

impl RolloutScenario {
    /// Whether this scenario broadens access.
    pub fn is_widening(&self) -> bool {
        self.direction.is_widening()
    }

    /// Whether this scenario meets the widening floor: review at least
    /// dual-control, a staged (non-immediate) rollout, a non-instant rollback,
    /// and at least one named widening dimension.
    pub fn meets_widening_floor(&self) -> bool {
        self.review_requirement.rank() >= ReviewRequirementClass::DualControlReview.rank()
            && !self.staging.is_immediate()
            && !self.rollback.is_instant()
            && !self.widening_dimensions.is_empty()
    }

    /// Whether this scenario stays within the tightening ceiling: at most a
    /// single admin review and no widening dimensions.
    pub fn within_tightening_ceiling(&self) -> bool {
        self.review_requirement.rank() <= ReviewRequirementClass::SingleAdminReview.rank()
            && self.widening_dimensions.is_empty()
    }

    /// Whether this is a genuinely light tightening: self-service or single-admin
    /// review and an immediate rollout. Used to prove tightening is not
    /// over-gated.
    pub fn is_light_tightening(&self) -> bool {
        self.direction == ChangeDirectionClass::Tightening
            && self.review_requirement.rank() <= ReviewRequirementClass::SingleAdminReview.rank()
            && self.staging.is_immediate()
    }
}

// ---------------------------------------------------------------------------
// Per-profile packet and bundle.
// ---------------------------------------------------------------------------

/// The dry-run rollout simulation for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutSimulationPacket {
    /// The admin path / profile this packet simulates.
    pub profile: AdminPathClass,
    /// Stable, namespaced profile id from the matrix.
    pub profile_id: String,
    /// The deployment profile this maps to.
    pub deployment_profile: AdminDeploymentProfileClass,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The consumers that render this packet (identical bytes for each).
    pub consumers: Vec<AdminConsumerClass>,
    /// The matrix surfaces this simulation renders forward.
    pub bound_surfaces: Vec<AdminSurfaceClass>,
    /// The dry-run scenarios.
    pub scenarios: Vec<RolloutScenario>,
    /// Whether this profile syncs from an offline mirror whose freshness counts.
    pub mirror_backed: bool,
    /// The freshness of the offline mirror's last sync (`fresh` when no mirror).
    pub mirror_freshness: EvidenceAgeClass,
    /// The freshness of the endpoint-posture evidence.
    pub endpoint_posture_freshness: EvidenceAgeClass,
    /// The freshness of the rollout-simulation evidence (the stalest scenario).
    pub simulation_freshness: EvidenceAgeClass,
    /// The managed-claim state after auto-narrowing: confirmed when all evidence
    /// is fresh, downgraded otherwise.
    pub claim_state: AdminStateClass,
    /// Why the claim auto-narrowed, empty when confirmed.
    pub narrow_reasons: Vec<ClaimNarrowReasonClass>,
    /// One reviewable sentence stating the claim posture.
    pub claim_note: String,
}

impl RolloutSimulationPacket {
    /// Whether the managed claim is confirmed (no auto-narrowing).
    pub fn claim_confirmed(&self) -> bool {
        self.claim_state == AdminStateClass::ActiveEnforced && self.narrow_reasons.is_empty()
    }

    /// Resolves a scenario by id within this packet.
    pub fn scenario(&self, scenario_id: &str) -> Option<&RolloutScenario> {
        self.scenarios.iter().find(|s| s.scenario_id == scenario_id)
    }
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutSimulationInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the simulated bundle satisfies the invariant.
    pub holds: bool,
}

/// The frozen rollout-simulation bundle: one packet per claimed managed-bearing
/// profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutSimulationBundle {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_rollout_simulation_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The matrix this simulation layer binds back to.
    pub matrix_ref: String,
    /// The matrix id this simulation layer binds back to.
    pub matrix_id: String,
    /// The freeze gate that keeps this bundle current.
    pub freeze_gate_ref: String,
    /// One reviewable summary sentence.
    pub summary: String,
    /// The per-profile simulation packets.
    pub profiles: Vec<RolloutSimulationPacket>,
    /// The computed invariants.
    pub invariants: Vec<RolloutSimulationInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the bundle fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RolloutSimulationValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for RolloutSimulationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rollout-simulation bundle invalid: {}", self.reason)
    }
}

impl std::error::Error for RolloutSimulationValidationError {}

/// The profiles the simulation bundle covers, in bundle order.
pub const SIMULATED_PROFILES: [AdminPathClass; 4] = [
    AdminPathClass::ManagedCloud,
    AdminPathClass::SelfHosted,
    AdminPathClass::SovereignAirGapped,
    AdminPathClass::MirroredOffline,
];

/// The matrix surfaces every packet renders forward.
pub const BOUND_SURFACES: [AdminSurfaceClass; 2] = [
    AdminSurfaceClass::PolicyDiff,
    AdminSurfaceClass::EndpointPostureCard,
];

/// The consumers every packet must serve identically for cross-surface parity.
const PARITY_CONSUMERS: [AdminConsumerClass; 5] = [
    AdminConsumerClass::ShellAdminCenter,
    AdminConsumerClass::CliHeadless,
    AdminConsumerClass::HelpAbout,
    AdminConsumerClass::SupportExport,
    AdminConsumerClass::ReleaseEvidence,
];

impl RolloutSimulationBundle {
    /// Returns the packet for a profile, if present.
    pub fn packet(&self, profile: AdminPathClass) -> Option<&RolloutSimulationPacket> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Every scenario across every profile, in bundle order.
    pub fn scenarios(&self) -> impl Iterator<Item = &RolloutScenario> {
        self.profiles.iter().flat_map(|p| p.scenarios.iter())
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or opaque token.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.file_refs().into_iter().all(is_export_safe_ref)
            && self.token_ids().into_iter().all(is_safe_token)
    }

    /// The repo-relative file refs carried by the bundle, for export-safety
    /// auditing.
    fn file_refs(&self) -> [&str; 3] {
        [
            self.schema_ref.as_str(),
            self.matrix_ref.as_str(),
            self.freeze_gate_ref.as_str(),
        ]
    }

    /// Every stable token id carried by the bundle, for export-safety auditing.
    fn token_ids(&self) -> Vec<&str> {
        let mut ids = Vec::new();
        for p in &self.profiles {
            ids.push(p.profile_id.as_str());
            for s in &p.scenarios {
                ids.push(s.scenario_id.as_str());
                for e in &s.impacted_endpoints {
                    ids.push(e.endpoint_ref.as_str());
                }
            }
        }
        ids
    }

    /// Re-checks structural consistency and returns an error on the first
    /// failure. Complements the computed invariants with the coverage and
    /// resolution checks a consumer relies on.
    pub fn validate(&self) -> Result<(), RolloutSimulationValidationError> {
        let fail = |reason: String| Err(RolloutSimulationValidationError { reason });

        if self.record_kind != M5_ROLLOUT_SIMULATION_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_ROLLOUT_SIMULATION_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if self.matrix_id != M5_ADMIN_PLANE_MATRIX_ID {
            return fail(format!("unexpected matrix_id {}", self.matrix_id));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }

        for profile in SIMULATED_PROFILES {
            if self
                .profiles
                .iter()
                .filter(|p| p.profile == profile)
                .count()
                != 1
            {
                return fail(format!(
                    "profile {} not present exactly once",
                    profile.as_str()
                ));
            }
        }
        if !all_unique(self.profiles.iter().map(|p| p.profile_id.as_str())) {
            return fail("profile ids are not unique".to_owned());
        }

        for packet in &self.profiles {
            validate_packet(packet)
                .map_err(|reason| RolloutSimulationValidationError { reason })?;
        }

        if !self.is_support_export_safe() {
            return fail("bundle is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

/// Whether a stable token id is safe to export: non-empty and carries no URL
/// scheme or absolute path.
fn is_safe_token(token: &str) -> bool {
    !token.is_empty() && !token.starts_with('/') && !token.contains("://")
}

/// Per-packet structural floor checks, shared by [`RolloutSimulationBundle::validate`].
fn validate_packet(packet: &RolloutSimulationPacket) -> Result<(), String> {
    if packet.profile_id != packet.profile.path_id() {
        return Err(format!(
            "profile id mismatch for {}",
            packet.profile.as_str()
        ));
    }
    if packet.scenarios.is_empty() {
        return Err(format!("{}: no scenarios", packet.profile.as_str()));
    }
    if !all_unique(packet.scenarios.iter().map(|s| s.scenario_id.as_str())) {
        return Err(format!(
            "{}: scenario ids not unique",
            packet.profile.as_str()
        ));
    }
    for scenario in &packet.scenarios {
        if scenario.impacted_endpoints.is_empty() {
            return Err(format!(
                "{}: scenario {} names no impacted endpoint",
                packet.profile.as_str(),
                scenario.scenario_id
            ));
        }
        if scenario.impacted_features.is_empty() {
            return Err(format!(
                "{}: scenario {} names no impacted feature",
                packet.profile.as_str(),
                scenario.scenario_id
            ));
        }
        if !scenario.dry_run {
            return Err(format!(
                "{}: scenario {} is not a dry run",
                packet.profile.as_str(),
                scenario.scenario_id
            ));
        }
        if !all_unique(
            scenario
                .impacted_endpoints
                .iter()
                .map(|e| e.endpoint_ref.as_str()),
        ) {
            return Err(format!(
                "{}: scenario {} has duplicate endpoint refs",
                packet.profile.as_str(),
                scenario.scenario_id
            ));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical rollout-simulation bundle.
///
/// Deterministic: the same bytes every call. The per-profile freshness and
/// claim-narrowing fields and the invariant `holds` flags are computed from the
/// scenarios, so an inconsistent edit flips an invariant rather than silently
/// passing.
pub fn rollout_simulation_bundle() -> RolloutSimulationBundle {
    let profiles: Vec<RolloutSimulationPacket> = SIMULATED_PROFILES
        .iter()
        .map(|p| simulate_profile(*p))
        .collect();
    let invariants = compute_invariants(&profiles);

    RolloutSimulationBundle {
        record_kind: M5_ROLLOUT_SIMULATION_RECORD_KIND.to_owned(),
        m5_rollout_simulation_schema_version: M5_ROLLOUT_SIMULATION_SCHEMA_VERSION,
        schema_ref: M5_ROLLOUT_SIMULATION_SCHEMA_REF.to_owned(),
        bundle_id: M5_ROLLOUT_SIMULATION_BUNDLE_ID.to_owned(),
        as_of: M5_ROLLOUT_SIMULATION_AS_OF.to_owned(),
        matrix_ref: M5_ROLLOUT_SIMULATION_MATRIX_REF.to_owned(),
        matrix_id: M5_ADMIN_PLANE_MATRIX_ID.to_owned(),
        freeze_gate_ref: M5_ROLLOUT_SIMULATION_FREEZE_GATE_REF.to_owned(),
        summary: "Dry-run rollout simulation — policy imports, promotions, bundle rollouts, \
                  mirror-source changes, trust-root changes, and route/egress expansions previewed \
                  before they widen privilege or feature access — bound back to the frozen \
                  admin-plane matrix and simulated identically for shell, CLI/headless, Help/About, \
                  support export, and release evidence across the managed-cloud, self-hosted, \
                  sovereign/air-gapped, and mirrored/offline profiles. Widening is gated harder \
                  than tightening, and a profile's managed claim auto-narrows when simulation, \
                  mirror, or endpoint-posture evidence goes stale."
            .to_owned(),
        profiles,
        invariants,
        raw_payload_excluded: true,
    }
}

fn endpoint(
    endpoint_ref: &str,
    label: &str,
    before: AdminStateClass,
    after: AdminStateClass,
    note: &str,
) -> ImpactedEndpoint {
    ImpactedEndpoint {
        endpoint_ref: endpoint_ref.to_owned(),
        label: label.to_owned(),
        posture_before: before,
        posture_after: after,
        impact_note: note.to_owned(),
    }
}

fn feature(feature_family: &str, effect_note: &str, newly_widened: bool) -> ImpactedFeature {
    ImpactedFeature {
        feature_family: feature_family.to_owned(),
        effect_note: effect_note.to_owned(),
        newly_widened,
    }
}

/// The stalest of a set of evidence ages — the conservative aggregate that drives
/// auto-narrowing. [`EvidenceAgeClass`] orders fresh < recent < stale < ...
fn worst_age(ages: impl IntoIterator<Item = EvidenceAgeClass>) -> EvidenceAgeClass {
    ages.into_iter().max().unwrap_or(EvidenceAgeClass::Fresh)
}

/// Computes the auto-narrowed claim state and its reasons from the three evidence
/// freshness inputs. Confirmed only when every input is fresh.
fn compute_claim(
    simulation: EvidenceAgeClass,
    mirror: EvidenceAgeClass,
    posture: EvidenceAgeClass,
    mirror_backed: bool,
) -> (AdminStateClass, Vec<ClaimNarrowReasonClass>) {
    let mut reasons = Vec::new();
    if simulation.is_stale() {
        reasons.push(ClaimNarrowReasonClass::SimulationEvidenceStale);
    }
    if mirror_backed && mirror.is_stale() {
        reasons.push(ClaimNarrowReasonClass::MirrorFreshnessStale);
    }
    if posture.is_stale() {
        reasons.push(ClaimNarrowReasonClass::EndpointPostureStale);
    }

    if reasons.is_empty() {
        return (AdminStateClass::ActiveEnforced, reasons);
    }
    // A stale offline mirror downgrades to the last-known state; any other stale
    // evidence downgrades to the no-silent-green unconfirmed state.
    let state = if mirror_backed && mirror.is_stale() {
        AdminStateClass::MirrorOfflineLastKnown
    } else {
        AdminStateClass::UnconfirmedStale
    };
    (state, reasons)
}

fn simulate_profile(profile: AdminPathClass) -> RolloutSimulationPacket {
    use AdminConsumerClass::*;

    let consumers = vec![
        ShellAdminCenter,
        CliHeadless,
        HelpAbout,
        SupportExport,
        ReleaseEvidence,
        ManagedService,
    ];

    let (deployment_profile, mirror_backed, mirror_freshness, posture_freshness, summary) =
        match profile {
            AdminPathClass::ManagedCloud => (
                AdminDeploymentProfileClass::ManagedCloud,
                false,
                EvidenceAgeClass::Fresh,
                EvidenceAgeClass::Fresh,
                "Managed-cloud profile: the control plane is online and posture is fresh; \
                 widening changes preview a staged ring rollout while tightening can apply \
                 immediately.",
            ),
            AdminPathClass::SelfHosted => (
                AdminDeploymentProfileClass::SelfHosted,
                false,
                EvidenceAgeClass::Fresh,
                EvidenceAgeClass::Recent,
                "Self-hosted profile: the customer operates the control plane; a trust-root \
                 rotation is held for a boundary recheck and a registry-source change previews a \
                 staged rollout.",
            ),
            AdminPathClass::SovereignAirGapped => (
                AdminDeploymentProfileClass::SovereignAirGapped,
                false,
                EvidenceAgeClass::Fresh,
                EvidenceAgeClass::Stale,
                "Sovereign / air-gapped profile: no outbound control plane; every change arrives \
                 as a signed offline bundle, the managed claim auto-narrows because simulation and \
                 posture evidence are past their windows, and a stale promotion is held.",
            ),
            AdminPathClass::MirroredOffline => (
                AdminDeploymentProfileClass::ManagedCloud,
                true,
                EvidenceAgeClass::Stale,
                EvidenceAgeClass::Recent,
                "Mirrored / offline profile: the managed source is offline; the managed claim \
                 auto-narrows to last-known because the mirror is stale, and a mirror-source \
                 change previews a staged rollout.",
            ),
            _ => (
                AdminDeploymentProfileClass::IndividualLocal,
                false,
                EvidenceAgeClass::Fresh,
                EvidenceAgeClass::Fresh,
                "Local profile.",
            ),
        };

    let scenarios = simulate_scenarios(profile);
    let simulation_freshness = worst_age(scenarios.iter().map(|s| s.simulation_freshness));
    let (claim_state, narrow_reasons) = compute_claim(
        simulation_freshness,
        mirror_freshness,
        posture_freshness,
        mirror_backed,
    );

    let claim_note = if narrow_reasons.is_empty() {
        "All simulation, mirror, and endpoint-posture evidence is fresh; the managed claim is \
         confirmed."
            .to_owned()
    } else {
        let dims: Vec<&str> = narrow_reasons.iter().map(|r| r.as_str()).collect();
        format!(
            "Managed claim auto-narrowed off confirmed because evidence is stale: {}.",
            dims.join(", ")
        )
    };

    RolloutSimulationPacket {
        profile,
        profile_id: profile.path_id(),
        deployment_profile,
        summary: summary.to_owned(),
        consumers,
        bound_surfaces: BOUND_SURFACES.to_vec(),
        scenarios,
        mirror_backed,
        mirror_freshness,
        endpoint_posture_freshness: posture_freshness,
        simulation_freshness,
        claim_state,
        narrow_reasons,
        claim_note,
    }
}

fn simulate_scenarios(profile: AdminPathClass) -> Vec<RolloutScenario> {
    use AdminRedactionClass::MetadataSafeDefault;
    use AdminStateClass::*;
    use ChangeDirectionClass::*;
    use OwnerEscalationRoleClass::*;
    use ReviewRequirementClass::*;
    use RollbackRequirementClass::*;
    use RolloutChangeKindClass::*;
    use RolloutStagingClass::*;
    use SimulationOutcomeClass::*;
    use WideningDimensionClass as Dim;

    match profile {
        AdminPathClass::ManagedCloud => vec![
            RolloutScenario {
                scenario_id: "rollout_sim.managed_cloud.tighten_telemetry".to_owned(),
                change_kind: PolicyImport,
                title: "Import managed policy bundle that shortens telemetry retention".to_owned(),
                summary: "A pure tightening: importing the next managed bundle shortens diagnostics \
                          retention. It can apply immediately and revert instantly."
                    .to_owned(),
                direction: Tightening,
                widening_dimensions: Vec::new(),
                impacted_endpoints: vec![endpoint(
                    "endpoint:managed:fleet",
                    "Managed fleet",
                    ActiveEnforced,
                    ActiveEnforced,
                    "Retention window shortens; no access is broadened.",
                )],
                impacted_features: vec![feature(
                    "Diagnostics",
                    "Telemetry retention is shortened from 90 to 30 days.",
                    false,
                )],
                review_requirement: SingleAdminReview,
                staging: ImmediateAllowed,
                rollback: InstantLocalRevert,
                simulation_freshness: EvidenceAgeClass::Fresh,
                outcome: SafeToPromote,
                owner: OrgAdmin,
                escalation_owner: None,
                redaction: MetadataSafeDefault,
                review_note: "Restriction only: a single admin review clears it, it may apply at \
                              once, and it reverts instantly."
                    .to_owned(),
                dry_run: true,
            },
            RolloutScenario {
                scenario_id: "rollout_sim.managed_cloud.egress_expansion".to_owned(),
                change_kind: RouteEgressExpansion,
                title: "Open a new egress class for a managed integration".to_owned(),
                summary: "A mixed change: it opens a new egress class while removing an unused one. \
                          Because it broadens egress it needs security review, a staged ring \
                          rollout, and a staged rollback."
                    .to_owned(),
                direction: Mixed,
                widening_dimensions: vec![Dim::NewEgressClass],
                impacted_endpoints: vec![endpoint(
                    "endpoint:managed:fleet",
                    "Managed fleet",
                    ActiveEnforced,
                    PendingManagedSync,
                    "The new egress class reaches endpoints ring by ring after review.",
                )],
                impacted_features: vec![feature(
                    "Networking",
                    "A new egress destination class is reachable from managed endpoints.",
                    true,
                )],
                review_requirement: SecurityComplianceReview,
                staging: StagedRingRequired,
                rollback: StagedRevert,
                simulation_freshness: EvidenceAgeClass::Fresh,
                outcome: PromoteWithStagedRollout,
                owner: SecurityOwner,
                escalation_owner: Some(ComplianceOwner),
                redaction: MetadataSafeDefault,
                review_note: "New egress widens reach: security/compliance sign-off, a staged ring \
                              rollout, and a staged rollback are required before promotion."
                    .to_owned(),
                dry_run: true,
            },
            RolloutScenario {
                scenario_id: "rollout_sim.managed_cloud.add_ai_provider".to_owned(),
                change_kind: PolicyPromotion,
                title: "Promote a staged bundle that allows a new AI provider".to_owned(),
                summary: "A widening promotion: it allows a new AI provider. It needs \
                          security/compliance review, a staged rollout, and a staged rollback."
                    .to_owned(),
                direction: Widening,
                widening_dimensions: vec![Dim::NewAiProvider],
                impacted_endpoints: vec![endpoint(
                    "endpoint:managed:fleet",
                    "Managed fleet",
                    ActiveEnforced,
                    PendingManagedSync,
                    "The new provider becomes selectable ring by ring after review.",
                )],
                impacted_features: vec![feature(
                    "AI / assistants",
                    "A new AI provider is added to the approved managed list.",
                    true,
                )],
                review_requirement: SecurityComplianceReview,
                staging: StagedRingRequired,
                rollback: StagedRevert,
                simulation_freshness: EvidenceAgeClass::Fresh,
                outcome: PromoteWithStagedRollout,
                owner: SecurityOwner,
                escalation_owner: Some(ComplianceOwner),
                redaction: MetadataSafeDefault,
                review_note: "A new provider widens data flow: security/compliance sign-off and a \
                              staged rollout are required before promotion."
                    .to_owned(),
                dry_run: true,
            },
        ],
        AdminPathClass::SelfHosted => vec![
            RolloutScenario {
                scenario_id: "rollout_sim.self_hosted.tighten_egress".to_owned(),
                change_kind: BundleRollout,
                title: "Roll out a self-hosted bundle that narrows egress to internal endpoints"
                    .to_owned(),
                summary: "A tightening rollout: it narrows egress to customer-internal endpoints. A \
                          single admin review clears it, staged by ring for safety."
                    .to_owned(),
                direction: Tightening,
                widening_dimensions: Vec::new(),
                impacted_endpoints: vec![endpoint(
                    "endpoint:self_hosted:fleet",
                    "Self-hosted fleet",
                    ActiveEnforced,
                    ActiveEnforced,
                    "Egress narrows to internal endpoints; nothing is broadened.",
                )],
                impacted_features: vec![feature(
                    "Networking",
                    "Egress is restricted to self-hosted endpoints.",
                    false,
                )],
                review_requirement: SingleAdminReview,
                staging: StagedRingRequired,
                rollback: StagedRevert,
                simulation_freshness: EvidenceAgeClass::Fresh,
                outcome: SafeToPromote,
                owner: SecurityOwner,
                escalation_owner: None,
                redaction: MetadataSafeDefault,
                review_note: "Restriction only: a single admin review clears it; it is staged by \
                              ring for operational safety, not because it broadens access."
                    .to_owned(),
                dry_run: true,
            },
            RolloutScenario {
                scenario_id: "rollout_sim.self_hosted.rotate_trust_root".to_owned(),
                change_kind: TrustRootChange,
                title: "Rotate the customer trust root".to_owned(),
                summary: "A trust-root rotation: it changes the root every signature verifies \
                          against, so it is held pending a boundary recheck before it can apply."
                    .to_owned(),
                direction: Widening,
                widening_dimensions: vec![Dim::TrustRootChange],
                impacted_endpoints: vec![endpoint(
                    "endpoint:self_hosted:fleet",
                    "Self-hosted fleet",
                    ActiveEnforced,
                    BoundaryChangedRecheckRequired,
                    "Verification is held until the new root is rechecked at the boundary.",
                )],
                impacted_features: vec![feature(
                    "Security / trust",
                    "The trust root every bundle verifies against is replaced.",
                    true,
                )],
                review_requirement: BlockedPendingBoundaryRecheck,
                staging: PinnedManualSignedOnly,
                rollback: SignedRollbackBundle,
                simulation_freshness: EvidenceAgeClass::Recent,
                outcome: BlockedBoundaryRecheck,
                owner: SecurityOwner,
                escalation_owner: Some(ComplianceOwner),
                redaction: MetadataSafeDefault,
                review_note: "A trust-root change is the strongest widening: it is blocked pending \
                              a boundary recheck, arrives only as a signed bundle, and reverts only \
                              via a signed rollback bundle."
                    .to_owned(),
                dry_run: true,
            },
            RolloutScenario {
                scenario_id: "rollout_sim.self_hosted.registry_and_permission".to_owned(),
                change_kind: PolicyImport,
                title: "Import a bundle that grants a new capability and changes the registry source"
                    .to_owned(),
                summary: "A widening import: it grants a new capability permission and repoints the \
                          extension registry. Both broaden supply, so it needs security review and \
                          a staged rollout."
                    .to_owned(),
                direction: Widening,
                widening_dimensions: vec![Dim::NewPermission, Dim::RegistrySourceChange],
                impacted_endpoints: vec![endpoint(
                    "endpoint:self_hosted:fleet",
                    "Self-hosted fleet",
                    ActiveEnforced,
                    PendingManagedSync,
                    "The new capability and registry source reach endpoints ring by ring.",
                )],
                impacted_features: vec![
                    feature(
                        "Capabilities",
                        "A new capability permission is granted to managed launches.",
                        true,
                    ),
                    feature(
                        "Extensions",
                        "The extension registry source is repointed.",
                        true,
                    ),
                ],
                review_requirement: SecurityComplianceReview,
                staging: StagedRingRequired,
                rollback: StagedRevert,
                simulation_freshness: EvidenceAgeClass::Recent,
                outcome: PromoteWithStagedRollout,
                owner: SecurityOwner,
                escalation_owner: Some(ComplianceOwner),
                redaction: MetadataSafeDefault,
                review_note: "A new permission and a registry-source change both widen supply: \
                              security/compliance sign-off and a staged rollout are required."
                    .to_owned(),
                dry_run: true,
            },
        ],
        AdminPathClass::SovereignAirGapped => vec![
            RolloutScenario {
                scenario_id: "rollout_sim.sovereign.tighten_providers".to_owned(),
                change_kind: PolicyImport,
                title: "Import a fresh signed offline bundle that narrows the provider list"
                    .to_owned(),
                summary: "A tightening import: a freshly signed offline bundle narrows the allowed \
                          provider list. It arrives only as a signed bundle and reverts via a \
                          signed rollback bundle."
                    .to_owned(),
                direction: Tightening,
                widening_dimensions: Vec::new(),
                impacted_endpoints: vec![endpoint(
                    "endpoint:sovereign:image",
                    "Sovereign image",
                    ActiveEnforced,
                    ActiveEnforced,
                    "The provider list narrows; no access is broadened.",
                )],
                impacted_features: vec![feature(
                    "AI / assistants",
                    "The allowed offline provider list is narrowed.",
                    false,
                )],
                review_requirement: SingleAdminReview,
                staging: PinnedManualSignedOnly,
                rollback: SignedRollbackBundle,
                simulation_freshness: EvidenceAgeClass::Recent,
                outcome: SafeToPromote,
                owner: SecurityOwner,
                escalation_owner: None,
                redaction: MetadataSafeDefault,
                review_note: "Restriction only: a single admin review clears it; pinned-signed \
                              delivery and a signed rollback are inherent to the air-gapped profile, \
                              not extra gating for widening."
                    .to_owned(),
                dry_run: true,
            },
            RolloutScenario {
                scenario_id: "rollout_sim.sovereign.rotate_pinned_root".to_owned(),
                change_kind: TrustRootChange,
                title: "Rotate the pinned offline trust root".to_owned(),
                summary: "A trust-root rotation on an air-gapped image: it changes the pinned \
                          offline root, so it is held for security/compliance review before the \
                          signed bundle is accepted."
                    .to_owned(),
                direction: Widening,
                widening_dimensions: vec![Dim::TrustRootChange],
                impacted_endpoints: vec![endpoint(
                    "endpoint:sovereign:image",
                    "Sovereign image",
                    ActiveEnforced,
                    UnconfirmedStale,
                    "Verification is held until the rotated root is reviewed and accepted.",
                )],
                impacted_features: vec![feature(
                    "Security / trust",
                    "The pinned offline trust root is replaced.",
                    true,
                )],
                review_requirement: SecurityComplianceReview,
                staging: PinnedManualSignedOnly,
                rollback: SignedRollbackBundle,
                simulation_freshness: EvidenceAgeClass::Recent,
                outcome: HoldForReview,
                owner: SecurityOwner,
                escalation_owner: Some(ComplianceOwner),
                redaction: MetadataSafeDefault,
                review_note: "A trust-root change widens trust: security/compliance sign-off is \
                              required, delivery is a signed bundle, and rollback is a signed \
                              bundle."
                    .to_owned(),
                dry_run: true,
            },
            RolloutScenario {
                scenario_id: "rollout_sim.sovereign.stale_promotion".to_owned(),
                change_kind: PolicyPromotion,
                title: "Promote a staged offline bundle whose simulation evidence is stale"
                    .to_owned(),
                summary: "A tightening promotion that is blocked: its simulation evidence is past \
                          the soft-refresh window, so it can never read as safe to promote until a \
                          fresh signed bundle is imported."
                    .to_owned(),
                direction: Tightening,
                widening_dimensions: Vec::new(),
                impacted_endpoints: vec![endpoint(
                    "endpoint:sovereign:image",
                    "Sovereign image",
                    ActiveEnforced,
                    UnconfirmedStale,
                    "The promotion is held; the last-known value is shown unconfirmed.",
                )],
                impacted_features: vec![feature(
                    "Policy",
                    "The promotion is held because its simulation evidence is stale.",
                    false,
                )],
                review_requirement: SingleAdminReview,
                staging: PinnedManualSignedOnly,
                rollback: SignedRollbackBundle,
                simulation_freshness: EvidenceAgeClass::Stale,
                outcome: BlockedStaleEvidence,
                owner: SecurityOwner,
                escalation_owner: Some(ComplianceOwner),
                redaction: MetadataSafeDefault,
                review_note: "Stale evidence: the promotion is blocked until a fresh signed bundle \
                              refreshes the simulation, never auto-promoted on a stale preview."
                    .to_owned(),
                dry_run: true,
            },
        ],
        AdminPathClass::MirroredOffline => vec![
            RolloutScenario {
                scenario_id: "rollout_sim.mirrored.switch_mirror_source".to_owned(),
                change_kind: MirrorSourceChange,
                title: "Switch to the backup mirror source".to_owned(),
                summary: "A widening change: repointing to a backup mirror source changes where \
                          managed truth is fetched from, so it needs security review and a staged \
                          rollout, and is held while the current mirror is stale."
                    .to_owned(),
                direction: Widening,
                widening_dimensions: vec![Dim::RegistrySourceChange],
                impacted_endpoints: vec![endpoint(
                    "endpoint:mirrored:fleet",
                    "Mirror-backed fleet",
                    MirrorOfflineLastKnown,
                    PendingManagedSync,
                    "Endpoints would resync from the backup source ring by ring after review.",
                )],
                impacted_features: vec![feature(
                    "Policy distribution",
                    "The mirror source managed bundles are fetched from is repointed.",
                    true,
                )],
                review_requirement: SecurityComplianceReview,
                staging: StagedRingRequired,
                rollback: StagedRevert,
                simulation_freshness: EvidenceAgeClass::Recent,
                outcome: HoldForReview,
                owner: OrgAdmin,
                escalation_owner: Some(SecurityOwner),
                redaction: MetadataSafeDefault,
                review_note: "A mirror-source change widens where truth comes from: security review \
                              and a staged rollout are required, and it is held while the mirror is \
                              stale."
                    .to_owned(),
                dry_run: true,
            },
            RolloutScenario {
                scenario_id: "rollout_sim.mirrored.apply_last_synced_tightening".to_owned(),
                change_kind: BundleRollout,
                title: "Apply the last-synced mirrored bundle tightening".to_owned(),
                summary: "A tightening from the last-synced mirror: a diagnostics restriction. It \
                          may apply immediately and reverts instantly, but the result is labeled \
                          last-known while the mirror is offline."
                    .to_owned(),
                direction: Tightening,
                widening_dimensions: Vec::new(),
                impacted_endpoints: vec![endpoint(
                    "endpoint:mirrored:fleet",
                    "Mirror-backed fleet",
                    MirrorOfflineLastKnown,
                    MirrorOfflineLastKnown,
                    "The restriction applies from the last sync; the value stays labeled last-known.",
                )],
                impacted_features: vec![feature(
                    "Diagnostics",
                    "A diagnostics restriction from the last mirror sync is applied.",
                    false,
                )],
                review_requirement: SingleAdminReview,
                staging: ImmediateAllowed,
                rollback: InstantLocalRevert,
                simulation_freshness: EvidenceAgeClass::Recent,
                outcome: SafeToPromote,
                owner: OrgAdmin,
                escalation_owner: None,
                redaction: MetadataSafeDefault,
                review_note: "Restriction only: a single admin review clears it and it reverts \
                              instantly; the value is labeled last-known while the mirror is offline."
                    .to_owned(),
                dry_run: true,
            },
        ],
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> RolloutSimulationInvariant {
    RolloutSimulationInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(profiles: &[RolloutSimulationPacket]) -> Vec<RolloutSimulationInvariant> {
    let matrix = admin_plane_matrix();

    let endpoint_state_admitted = |state: AdminStateClass| -> bool {
        matrix
            .surface(AdminSurfaceClass::EndpointPostureCard)
            .is_some_and(|entry| entry.applicable_states.contains(&state))
    };

    let mut out = Vec::new();

    // Every state shown forward is one the matrix admits for the endpoint surface.
    out.push(invariant(
        "rollout_sim.surface_states_within_matrix",
        "Every endpoint posture state a scenario shows — before and after — and every per-profile \
         claim state is one the frozen admin-plane matrix declares applicable for the \
         endpoint-posture surface, so the simulation cannot drift from the contract.",
        profiles.iter().all(|p| {
            endpoint_state_admitted(p.claim_state)
                && p.scenarios.iter().all(|s| {
                    s.impacted_endpoints.iter().all(|e| {
                        endpoint_state_admitted(e.posture_before)
                            && endpoint_state_admitted(e.posture_after)
                    })
                })
        }),
    ));

    // The surfaces the simulation renders forward exist in the matrix and are
    // locally explainable / typed, never portal-only.
    out.push(invariant(
        "rollout_sim.bound_surfaces_in_matrix",
        "Each profile binds the policy-diff and endpoint-posture surfaces, and both are present in \
         the matrix, locally explainable, and typed rather than portal-only.",
        profiles.iter().all(|p| {
            BOUND_SURFACES.iter().all(|surface| p.bound_surfaces.contains(surface))
        }) && BOUND_SURFACES.iter().all(|surface| {
            matrix
                .surface(*surface)
                .is_some_and(|e| e.locally_explainable && e.typed_not_portal_only)
        }),
    ));

    // Widening is gated harder than tightening.
    out.push(invariant(
        "rollout_sim.widening_requires_stronger_review",
        "Every widening or mixed scenario clears at least dual-control review, a staged \
         (non-immediate) rollout, a non-instant rollback, and names a widening dimension; every \
         pure tightening needs at most a single admin review and names no widening dimension.",
        profiles.iter().all(|p| {
            p.scenarios.iter().all(|s| match s.direction {
                ChangeDirectionClass::Widening | ChangeDirectionClass::Mixed => {
                    s.meets_widening_floor()
                }
                ChangeDirectionClass::Tightening | ChangeDirectionClass::NoEffect => {
                    s.within_tightening_ceiling()
                }
            })
        }),
    ));

    // Tightening is not over-gated: at least one tightening across the bundle is a
    // genuinely light, immediate restriction, proving restrictions are not held to
    // the widening floor.
    out.push(invariant(
        "rollout_sim.tightening_not_overgated",
        "At least one tightening scenario across the bundle is a light, immediately-applicable \
         restriction with no more than a single admin review, so simple restrictions are not held \
         to the widening floor.",
        profiles
            .iter()
            .flat_map(|p| p.scenarios.iter())
            .any(RolloutScenario::is_light_tightening),
    ));

    // Every scenario is a reviewable dry-run with the required fields.
    out.push(invariant(
        "rollout_sim.scenarios_are_reviewable_dry_runs",
        "Every scenario is a dry-run preview that names at least one impacted endpoint and feature, \
         a review requirement, a staged-rollout requirement, and a rollback path before any \
         promotion.",
        profiles.iter().all(|p| {
            p.scenarios.iter().all(|s| {
                s.dry_run
                    && !s.impacted_endpoints.is_empty()
                    && !s.impacted_features.is_empty()
                    && !s.review_note.is_empty()
            })
        }),
    ));

    // No-silent-green: a scenario with stale simulation evidence is never safe to
    // promote.
    out.push(invariant(
        "rollout_sim.stale_scenarios_held",
        "A scenario whose own simulation evidence is stale is held (blocked for stale evidence) \
         and never read as safe to promote.",
        profiles.iter().all(|p| {
            p.scenarios.iter().all(|s| {
                if s.simulation_freshness.is_stale() {
                    s.outcome == SimulationOutcomeClass::BlockedStaleEvidence
                } else {
                    true
                }
            })
        }),
    ));

    // A boundary-recheck block lines up review, outcome, and pinned delivery.
    out.push(invariant(
        "rollout_sim.boundary_recheck_consistent",
        "A scenario blocked pending a boundary recheck reports the boundary-recheck outcome and a \
         pinned, signed-only delivery, and vice versa, so a boundary block is never silently \
         promotable.",
        profiles.iter().all(|p| {
            p.scenarios.iter().all(|s| {
                let review_blocks =
                    s.review_requirement == ReviewRequirementClass::BlockedPendingBoundaryRecheck;
                let outcome_blocks = s.outcome == SimulationOutcomeClass::BlockedBoundaryRecheck;
                if review_blocks || outcome_blocks {
                    review_blocks
                        && outcome_blocks
                        && s.staging == RolloutStagingClass::PinnedManualSignedOnly
                } else {
                    true
                }
            })
        }),
    ));

    // Claim auto-narrows off confirmed exactly when evidence is stale, naming the
    // stale dimension.
    out.push(invariant(
        "rollout_sim.claim_auto_narrows_on_stale",
        "A profile's managed claim reads confirmed only when its simulation, mirror, and \
         endpoint-posture evidence are all fresh; when any is stale the claim downgrades off \
         confirmed and names exactly which dimension went stale.",
        profiles.iter().all(|p| {
            let (expected_state, expected_reasons) = compute_claim(
                p.simulation_freshness,
                p.mirror_freshness,
                p.endpoint_posture_freshness,
                p.mirror_backed,
            );
            p.claim_state == expected_state && p.narrow_reasons == expected_reasons
        }),
    ));

    // The per-profile simulation freshness is the stalest scenario.
    out.push(invariant(
        "rollout_sim.simulation_freshness_is_worst_case",
        "Each profile's reported simulation freshness is the stalest of its scenarios, so a single \
         stale scenario cannot hide behind fresher siblings.",
        profiles.iter().all(|p| {
            p.simulation_freshness == worst_age(p.scenarios.iter().map(|s| s.simulation_freshness))
        }),
    ));

    // Widening dimensions are consistent with the direction.
    out.push(invariant(
        "rollout_sim.widening_dimensions_consistent",
        "A scenario names at least one widening dimension exactly when it widens or mixes, and none \
         when it only tightens or has no effect.",
        profiles.iter().all(|p| {
            p.scenarios.iter().all(|s| {
                if s.direction.is_widening() {
                    !s.widening_dimensions.is_empty()
                } else {
                    s.widening_dimensions.is_empty()
                }
            })
        }),
    ));

    // A feature is flagged newly-widened only on a widening scenario.
    out.push(invariant(
        "rollout_sim.widened_features_only_on_widening",
        "A scenario flags an impacted feature as newly widened only when the change itself widens \
         or mixes, so a tightening never claims to broaden a feature.",
        profiles.iter().all(|p| {
            p.scenarios.iter().all(|s| {
                if s.direction.is_widening() {
                    true
                } else {
                    s.impacted_features.iter().all(|f| !f.newly_widened)
                }
            })
        }),
    ));

    // Every claimed managed-bearing profile is simulated.
    out.push(invariant(
        "rollout_sim.profiles_covered",
        "The bundle simulates the managed-cloud, self-hosted, sovereign/air-gapped, and \
         mirrored/offline profiles.",
        SIMULATED_PROFILES
            .iter()
            .all(|profile| profiles.iter().any(|p| p.profile == *profile)),
    ));

    // Every rollout flow is exercised somewhere in the bundle.
    out.push(invariant(
        "rollout_sim.change_kinds_covered",
        "Every rollout flow — policy import, promotion, bundle rollout, mirror-source change, \
         trust-root change, and route/egress expansion — is simulated somewhere in the bundle.",
        RolloutChangeKindClass::ALL.iter().all(|kind| {
            profiles
                .iter()
                .flat_map(|p| p.scenarios.iter())
                .any(|s| s.change_kind == *kind)
        }),
    ));

    // Every widening dimension is exercised somewhere in the bundle.
    out.push(invariant(
        "rollout_sim.widening_dimensions_covered",
        "Every widening dimension — new permission, new egress class, new AI provider, \
         registry-source change, and trust-root change — is simulated somewhere in the bundle.",
        WideningDimensionClass::ALL.iter().all(|dim| {
            profiles
                .iter()
                .flat_map(|p| p.scenarios.iter())
                .any(|s| s.widening_dimensions.contains(dim))
        }),
    ));

    // Cross-surface parity: one typed packet serves every required consumer.
    out.push(invariant(
        "rollout_sim.consumer_parity",
        "Each profile is one typed packet consumed identically by shell, CLI/headless, Help/About, \
         support export, and release evidence, so the simulated blast radius is identical across \
         surfaces by construction.",
        profiles
            .iter()
            .all(|p| PARITY_CONSUMERS.iter().all(|c| p.consumers.contains(c))),
    ));

    // Stable ids are unique within their scope.
    out.push(invariant(
        "rollout_sim.stable_ids_unique",
        "Profile ids, scenario ids, and endpoint refs are unique within their scope, so a consumer \
         can resolve any object by a stable id.",
        all_unique(profiles.iter().map(|p| p.profile_id.as_str()))
            && profiles.iter().all(|p| {
                all_unique(p.scenarios.iter().map(|s| s.scenario_id.as_str()))
                    && p.scenarios.iter().all(|s| {
                        all_unique(s.impacted_endpoints.iter().map(|e| e.endpoint_ref.as_str()))
                    })
            }),
    ));

    // Export safety, surfaced as a computed invariant for release automation.
    out.push(invariant(
        "rollout_sim.export_safe",
        "Every stable profile, scenario, and endpoint id is an opaque token with no URL scheme or \
         absolute path, so the bundle is safe to embed in a support export verbatim.",
        profiles.iter().all(|p| {
            is_safe_token(p.profile_id.as_str())
                && p.scenarios.iter().all(|s| {
                    is_safe_token(s.scenario_id.as_str())
                        && s.impacted_endpoints
                            .iter()
                            .all(|e| is_safe_token(e.endpoint_ref.as_str()))
                })
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the bundle as human-readable lines for CLI/headless and support.
pub fn rollout_simulation_lines(bundle: &RolloutSimulationBundle) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Rollout-simulation bundle — {} ({})",
        bundle.bundle_id, bundle.as_of
    ));
    lines.push(bundle.summary.clone());
    lines.push(format!(
        "Profiles: {}  Invariants: {}  (binds matrix {})",
        bundle.profiles.len(),
        bundle.invariants.len(),
        bundle.matrix_id,
    ));

    for p in &bundle.profiles {
        lines.push(format!("Profile {} [{}]", p.profile.as_str(), p.profile_id));
        lines.push(format!("  {}", p.summary));
        lines.push(format!(
            "  Claim: {} (sim={} mirror={} posture={}){}",
            p.claim_state.as_str(),
            p.simulation_freshness.as_str(),
            p.mirror_freshness.as_str(),
            p.endpoint_posture_freshness.as_str(),
            if p.narrow_reasons.is_empty() {
                String::new()
            } else {
                let r: Vec<&str> = p.narrow_reasons.iter().map(|x| x.as_str()).collect();
                format!(" narrowed: {}", r.join(", "))
            }
        ));
        lines.push("  Scenarios:".to_owned());
        for s in &p.scenarios {
            lines.push(format!(
                "    - {} [{}] dir={} review={} staging={} rollback={} outcome={}",
                s.scenario_id,
                s.change_kind.as_str(),
                s.direction.as_str(),
                s.review_requirement.as_str(),
                s.staging.as_str(),
                s.rollback.as_str(),
                s.outcome.as_str(),
            ));
            if !s.widening_dimensions.is_empty() {
                let dims: Vec<&str> = s.widening_dimensions.iter().map(|d| d.as_str()).collect();
                lines.push(format!("        widens: {}", dims.join(", ")));
            }
            for e in &s.impacted_endpoints {
                lines.push(format!(
                    "        endpoint {} {}→{}",
                    e.endpoint_ref,
                    e.posture_before.as_str(),
                    e.posture_after.as_str(),
                ));
            }
        }
    }

    lines.push("Invariants:".to_owned());
    for i in &bundle.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
