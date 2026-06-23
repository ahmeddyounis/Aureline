//! Operator-surface qualification packet: the certification lane that binds the
//! M5 operator-surface truth sources into one per-family claim verdict and
//! auto-narrows a claimed operator family when its ownership, freshness, or
//! continuity proof is stale or failing.
//!
//! The product claims a set of operator surfaces ([`OperatorSurfaceClass`]):
//! operational overview boards, triage inboxes, action plans, evidence handoff
//! bundles, shift digests, service-ownership / on-call strips, runbook-step
//! cards, maintenance / read-only / drain notices, failover / migration notices,
//! and embedded provider/auth boundary states. Each one is governed by its own
//! frozen truth lane in this crate (the operator-surface matrix plus the
//! overview-board, triage-inbox, action-plan, handoff-digest, response-pane,
//! maintenance-window, and embedded-dashboard lanes). This lane does **not**
//! re-prove those contracts; it consumes them as proof sources and projects one
//! qualification packet that release automation, About/help, service-health,
//! compatibility, and support export all render instead of restating
//! operator-surface quality claims by hand.
//!
//! The packet pins three things:
//!
//! 1. **Proof dimensions** — the closed set of operator-surface claims a family
//!    is certified on ([`ProofDimension`]): the canonical-matrix binding,
//!    overview truth, triage truth, action-plan continuity, handoff-bundle
//!    fidelity, service ownership, runbook-step authority, maintenance/failover
//!    communication, and embedded-boundary honesty. Each dimension cites the
//!    upstream lane(s) that prove it and carries a freshness budget.
//! 2. **Proof freshness + failure state** — every dimension resolves to one
//!    [`ProofState`] (`fresh`, `stale`, `failing`, or `missing`) derived from the
//!    upstream lane's pass state and its capture stamp against the evaluation
//!    stamp. Stale or failing operator-surface evidence is the trigger that
//!    narrows the affected claim — exactly the silent-aging gap the guardrail
//!    forbids.
//! 3. **Per-family claim support** — for each claimed operator family, the packet
//!    evaluates the canonical-matrix dimension plus the dimension that governs
//!    that family and resolves a [`ClaimSupportClass`] (`fully_supported`,
//!    `narrowed`, or `blocked`). A family stays `fully_supported` only when every
//!    dimension it claims is fresh; a critical-dimension failure (the matrix
//!    binding, runbook-step authority, or embedded-boundary honesty) blocks the
//!    claim, everything else narrows it, and the narrowing/blocking dimensions are
//!    named.
//!
//! [`project_operator_qualification`] is the release-automation entry point: it
//! takes the evaluation stamp and a list of [`ProofInput`]s and computes the
//! packet. [`operator_qualification_packet`] is the canonical binding that feeds
//! it the real in-code proof sources, so the checked-in fixture and the replay
//! gate freeze the certified state byte-for-byte. The record carries no endpoint
//! URLs, credential bodies, or raw provider payloads, so it is safe for support
//! export.

use serde::{Deserialize, Serialize};

use crate::m5_action_plans::{action_plan_set, M5_ACTION_PLANS_AS_OF, M5_ACTION_PLANS_SCHEMA_REF};
use crate::m5_embedded_dashboards::{
    embedded_surface_set, M5_EMBEDDED_DASHBOARDS_AS_OF, M5_EMBEDDED_DASHBOARDS_SCHEMA_REF,
};
use crate::m5_handoff_digests::{
    handoff_digest_set, M5_HANDOFF_DIGESTS_AS_OF, M5_HANDOFF_DIGESTS_SCHEMA_REF,
};
use crate::m5_maintenance_windows::{
    maintenance_window_set, M5_MAINTENANCE_WINDOWS_AS_OF, M5_MAINTENANCE_WINDOWS_SCHEMA_REF,
};
use crate::m5_operator_boards::{
    operator_board_set, M5_OPERATOR_BOARDS_AS_OF, M5_OPERATOR_BOARDS_SCHEMA_REF,
};
use crate::m5_operator_surfaces::{
    operator_surface_matrix, OperatorSurfaceClass, M5_OPERATOR_SURFACES_AS_OF,
    M5_OPERATOR_SURFACES_SCHEMA_REF,
};
use crate::m5_response_panes::{
    response_pane_set, M5_RESPONSE_PANES_AS_OF, M5_RESPONSE_PANES_SCHEMA_REF,
};
use crate::m5_triage_inbox::{triage_inbox_set, M5_TRIAGE_INBOX_AS_OF, M5_TRIAGE_INBOX_SCHEMA_REF};

#[cfg(test)]
mod tests;

/// Schema version for the operator-surface qualification packet.
pub const M5_OPERATOR_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the operator-surface qualification packet.
pub const M5_OPERATOR_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/ops/m5-operator-qualification.schema.json";

/// Stable record-kind tag for the operator-surface qualification packet.
pub const M5_OPERATOR_QUALIFICATION_RECORD_KIND: &str = "m5_operator_qualification_packet";

/// Stable id for the canonical operator-surface qualification packet.
pub const M5_OPERATOR_QUALIFICATION_PACKET_ID: &str = "m5-operator-qualification:packet:0001";

/// Evaluation stamp for the canonical packet. Held as a constant so the
/// canonical binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_OPERATOR_QUALIFICATION_AS_OF: &str = "2026-06-22T00:00:00Z";

/// Default freshness budget, in days, before a passing proof is treated as
/// stale. Release automation may pass a tighter budget per dimension.
pub const DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS: i64 = 30;

// ---------------------------------------------------------------------------
// Proof dimensions.
// ---------------------------------------------------------------------------

/// The closed set of operator-surface claims a family is certified on.
///
/// Each dimension names one operator-surface claim and cites the upstream lane
/// whose freshness and pass state decide whether the claim holds. The set is the
/// union of the qualification-coverage requirements (overview/triage truth,
/// service ownership/on-call, runbook-step authority, handoff/export continuity,
/// maintenance/failover communication, embedded boundary honesty) plus the
/// shared canonical-matrix binding every family is anchored to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofDimension {
    /// Every operator family resolves through the one frozen operator-surface
    /// matrix, so dashboards and queues point at the same canonical objects
    /// incident/support/review/admin flows use. A broken matrix binding makes
    /// the whole surface vocabulary untrustworthy.
    CanonicalMatrixBinding,
    /// Operational overview boards downgrade any would-be-green tile whose
    /// evidence is stale, partial, or cached, and carry owner/blocker truth.
    OverviewTruth,
    /// Triage inboxes name their order and narrowing reasons and point at the
    /// same canonical incident/support/review/admin objects.
    TriageTruth,
    /// Action plans keep per-item local-versus-external state, linked evidence,
    /// and approval/policy truth so a local checkoff never resolves a
    /// provider-owned object on its own.
    ActionPlanContinuity,
    /// Handoff bundles and shift digests preserve object identity, ownership,
    /// redaction, scope, and live-versus-snapshot truth on export.
    HandoffBundleFidelity,
    /// Service-ownership / on-call strips keep owner, contract state, and
    /// local-continuity posture visible.
    ServiceOwnership,
    /// Runbook-step cards keep mutating-step preview/approval admission honest:
    /// a mutating step is previewed and admitted before it runs.
    RunbookStepAuthority,
    /// Maintenance / read-only / drain and failover / migration notices carry
    /// exact times, named blocked write classes, and local-safe / publish-later
    /// continuity rather than generic outage copy.
    MaintenanceFailoverCommunication,
    /// Embedded provider/auth boundary states disclose owner/origin and
    /// capability truth and never impersonate a native approval.
    EmbeddedBoundaryHonesty,
}

impl ProofDimension {
    /// All proof dimensions, in packet order.
    pub const ALL: [Self; 9] = [
        Self::CanonicalMatrixBinding,
        Self::OverviewTruth,
        Self::TriageTruth,
        Self::ActionPlanContinuity,
        Self::HandoffBundleFidelity,
        Self::ServiceOwnership,
        Self::RunbookStepAuthority,
        Self::MaintenanceFailoverCommunication,
        Self::EmbeddedBoundaryHonesty,
    ];

    /// Returns the stable schema token for this dimension.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalMatrixBinding => "canonical_matrix_binding",
            Self::OverviewTruth => "overview_truth",
            Self::TriageTruth => "triage_truth",
            Self::ActionPlanContinuity => "action_plan_continuity",
            Self::HandoffBundleFidelity => "handoff_bundle_fidelity",
            Self::ServiceOwnership => "service_ownership",
            Self::RunbookStepAuthority => "runbook_step_authority",
            Self::MaintenanceFailoverCommunication => "maintenance_failover_communication",
            Self::EmbeddedBoundaryHonesty => "embedded_boundary_honesty",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CanonicalMatrixBinding => "Canonical-matrix binding",
            Self::OverviewTruth => "Overview-board truth",
            Self::TriageTruth => "Triage-inbox truth",
            Self::ActionPlanContinuity => "Action-plan continuity",
            Self::HandoffBundleFidelity => "Handoff-bundle fidelity",
            Self::ServiceOwnership => "Service ownership / on-call",
            Self::RunbookStepAuthority => "Runbook-step authority",
            Self::MaintenanceFailoverCommunication => "Maintenance / failover communication",
            Self::EmbeddedBoundaryHonesty => "Embedded boundary honesty",
        }
    }

    /// Whether failing or missing proof on this dimension blocks the claim rather
    /// than merely narrowing it.
    ///
    /// Three dimensions are critical safety properties the operator surface
    /// cannot ship around: the canonical-matrix binding (a divergent matrix makes
    /// every surface point at the wrong objects), runbook-step authority (a
    /// mutating step running without preview/approval is unsafe), and
    /// embedded-boundary honesty (a webview/auth surface impersonating a native
    /// approval is a trust violation). Every other dimension degrades honestly —
    /// it narrows the claim and discloses the limit rather than blocking outright.
    pub const fn is_critical(self) -> bool {
        matches!(
            self,
            Self::CanonicalMatrixBinding
                | Self::RunbookStepAuthority
                | Self::EmbeddedBoundaryHonesty
        )
    }

    /// Returns true when this dimension's proof governs the given operator family.
    ///
    /// The canonical-matrix binding governs every family — they are all anchored
    /// to the one frozen matrix. Every other dimension governs only the family
    /// (or families) it directly proves, so a maintenance notice is never
    /// penalized when triage proof ages out.
    pub fn applies_to(self, family: OperatorSurfaceClass) -> bool {
        use OperatorSurfaceClass::*;
        match self {
            Self::CanonicalMatrixBinding => true,
            Self::OverviewTruth => matches!(family, OperationalOverviewBoard),
            Self::TriageTruth => matches!(family, TriageInbox),
            Self::ActionPlanContinuity => matches!(family, ActionPlan),
            Self::HandoffBundleFidelity => matches!(family, HandoffBundle | ShiftDigest),
            Self::ServiceOwnership => matches!(family, ServiceOwnershipStrip),
            Self::RunbookStepAuthority => matches!(family, RunbookStepCard),
            Self::MaintenanceFailoverCommunication => {
                matches!(family, MaintenanceNotice | FailoverNotice)
            }
            Self::EmbeddedBoundaryHonesty => matches!(family, EmbeddedBoundaryState),
        }
    }
}

/// Returns the dimension that specifically governs a family (besides the shared
/// canonical-matrix binding every family claims).
fn primary_dimension(family: OperatorSurfaceClass) -> ProofDimension {
    use OperatorSurfaceClass::*;
    match family {
        OperationalOverviewBoard => ProofDimension::OverviewTruth,
        TriageInbox => ProofDimension::TriageTruth,
        ActionPlan => ProofDimension::ActionPlanContinuity,
        HandoffBundle | ShiftDigest => ProofDimension::HandoffBundleFidelity,
        ServiceOwnershipStrip => ProofDimension::ServiceOwnership,
        RunbookStepCard => ProofDimension::RunbookStepAuthority,
        MaintenanceNotice | FailoverNotice => ProofDimension::MaintenanceFailoverCommunication,
        EmbeddedBoundaryState => ProofDimension::EmbeddedBoundaryHonesty,
    }
}

// ---------------------------------------------------------------------------
// Proof state + claim support.
// ---------------------------------------------------------------------------

/// The resolved freshness / failure state of one proof dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofState {
    /// Proof is present, passing, and captured within its freshness budget.
    Fresh,
    /// Proof is present and was passing, but captured outside its freshness
    /// budget — it has silently aged out.
    Stale,
    /// Proof is present but the upstream contract did not hold.
    Failing,
    /// No proof was supplied for this dimension.
    Missing,
}

impl ProofState {
    /// Returns the stable schema token for this proof state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Failing => "failing",
            Self::Missing => "missing",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Fresh",
            Self::Stale => "Stale (aged out)",
            Self::Failing => "Failing",
            Self::Missing => "Missing",
        }
    }

    /// Whether the proof is fresh and passing (the only state that keeps a claim
    /// fully supported).
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::Fresh)
    }
}

/// The certified support level of one operator family's claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimSupportClass {
    /// Every dimension the family claims is fresh and passing.
    FullySupported,
    /// At least one claimed dimension is stale or failing, but no critical safety
    /// dimension failed; the claim is degraded and discloses its limits.
    Narrowed,
    /// A critical safety dimension (matrix binding, runbook-step authority, or
    /// embedded-boundary honesty) failed or is missing; the family's claim is
    /// withdrawn.
    Blocked,
}

impl ClaimSupportClass {
    /// Returns the stable schema token for this support level.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullySupported => "Fully supported",
            Self::Narrowed => "Narrowed",
            Self::Blocked => "Blocked",
        }
    }

    /// Severity rank; higher is worse. Used to fold per-dimension effects into a
    /// family verdict.
    const fn severity(self) -> u8 {
        match self {
            Self::FullySupported => 0,
            Self::Narrowed => 1,
            Self::Blocked => 2,
        }
    }

    /// Returns the worse of two support levels.
    fn worst(self, other: Self) -> Self {
        if other.severity() > self.severity() {
            other
        } else {
            self
        }
    }
}

/// The support effect a proof state has on a claim, given whether the dimension
/// is critical.
fn dimension_effect(state: ProofState, critical: bool) -> ClaimSupportClass {
    match state {
        ProofState::Fresh => ClaimSupportClass::FullySupported,
        ProofState::Stale => ClaimSupportClass::Narrowed,
        ProofState::Failing | ProofState::Missing => {
            if critical {
                ClaimSupportClass::Blocked
            } else {
                ClaimSupportClass::Narrowed
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Release-automation input.
// ---------------------------------------------------------------------------

/// One raw proof observation fed to [`project_operator_qualification`] by release
/// automation.
///
/// Release automation knows where each operator-surface proof was last captured
/// and whether it passed; this struct carries that verbatim so the projection
/// derives the freshness / failure state deterministically rather than the caller
/// pre-deciding the verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofInput {
    /// Dimension this observation proves.
    pub dimension: ProofDimension,
    /// Primary upstream lane schema ref this proof is drawn from.
    pub proof_source_ref: String,
    /// All upstream lane refs that must hold for the proof to pass, including the
    /// primary.
    pub contributing_proof_refs: Vec<String>,
    /// Capture stamp of the proof, or `None` when no proof exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_as_of: Option<String>,
    /// Whether every contributing upstream contract held when captured.
    pub passing: bool,
    /// Freshness budget, in days, before the proof is treated as stale.
    pub freshness_budget_days: i64,
    /// Human-readable note about the proof.
    pub detail: String,
}

impl ProofInput {
    /// Resolves this input to a [`ProofState`] against the evaluation stamp.
    pub fn resolve_state(&self, evaluated_as_of: &str) -> ProofState {
        derive_proof_state(
            self.captured_as_of.as_deref(),
            self.passing,
            self.freshness_budget_days,
            evaluated_as_of,
        )
    }
}

// ---------------------------------------------------------------------------
// Projected records.
// ---------------------------------------------------------------------------

/// The resolved global proof for one dimension, shared across every family that
/// claims it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionProof {
    /// Dimension this proof covers.
    pub dimension: ProofDimension,
    /// Human-readable dimension label.
    pub label: String,
    /// Whether failing / missing proof on this dimension blocks rather than
    /// narrows a claim.
    pub critical: bool,
    /// Primary upstream lane schema ref.
    pub proof_source_ref: String,
    /// All upstream lane refs that contribute to this proof.
    pub contributing_proof_refs: Vec<String>,
    /// Capture stamp, when proof exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_as_of: Option<String>,
    /// Freshness budget, in days.
    pub freshness_budget_days: i64,
    /// Resolved freshness / failure state.
    pub state: ProofState,
    /// Human-readable note.
    pub detail: String,
}

/// One family's verdict on a single proof dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionVerdict {
    /// Dimension this verdict covers.
    pub dimension: ProofDimension,
    /// Whether the family claims this dimension at all.
    pub applicable: bool,
    /// Resolved proof state for the dimension.
    pub state: ProofState,
    /// The support effect this dimension contributes to the family claim
    /// (`fully_supported` when not applicable or fresh).
    pub effect: ClaimSupportClass,
}

/// One claimed operator family's certified qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyQualificationRow {
    /// Operator family this row certifies.
    pub surface: OperatorSurfaceClass,
    /// Stable, product-unique surface id.
    pub surface_id: String,
    /// Human-readable family label.
    pub label: String,
    /// The dimension that specifically governs this family, besides the shared
    /// canonical-matrix binding.
    pub primary_dimension: ProofDimension,
    /// Resolved support level for the family's claim.
    pub support: ClaimSupportClass,
    /// Per-dimension verdicts, one per [`ProofDimension`] in packet order.
    pub dimension_verdicts: Vec<DimensionVerdict>,
    /// Applicable dimensions that narrowed the claim, in dimension order.
    pub narrowed_by: Vec<ProofDimension>,
    /// Applicable dimensions that blocked the claim, in dimension order.
    pub blocked_by: Vec<ProofDimension>,
    /// Human-readable summary of the family verdict.
    pub summary: String,
}

impl FamilyQualificationRow {
    /// Returns true when every dimension the family claims is fresh.
    pub fn is_fully_supported(&self) -> bool {
        self.support == ClaimSupportClass::FullySupported
    }

    /// Returns the verdict for a given dimension, when present.
    pub fn verdict(&self, dimension: ProofDimension) -> Option<&DimensionVerdict> {
        self.dimension_verdicts
            .iter()
            .find(|verdict| verdict.dimension == dimension)
    }
}

/// Cross-family rollup of the qualification packet, for service-health and About
/// surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationRollup {
    /// Number of families fully supported.
    pub fully_supported: usize,
    /// Number of families narrowed.
    pub narrowed: usize,
    /// Number of families blocked.
    pub blocked: usize,
    /// Number of dimensions whose proof is stale.
    pub stale_dimensions: usize,
    /// Number of dimensions whose proof is failing.
    pub failing_dimensions: usize,
    /// Number of dimensions whose proof is missing.
    pub missing_dimensions: usize,
}

/// One frozen invariant the packet must satisfy, with the result of evaluating it
/// over the packet's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// Human-readable statement of the invariant.
    pub statement: String,
    /// Whether the invariant holds on the built packet.
    pub holds: bool,
}

/// The operator-surface qualification packet: per-family claim verdicts derived
/// from the operator-surface proof sources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorQualificationPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_operator_qualification_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Evaluation stamp the proof freshness is measured against.
    pub as_of: String,
    /// Resolved global proof per dimension, in dimension order.
    pub dimensions: Vec<DimensionProof>,
    /// Per-family qualification rows, one per [`OperatorSurfaceClass`].
    pub families: Vec<FamilyQualificationRow>,
    /// Cross-family rollup.
    pub rollup: QualificationRollup,
    /// Frozen invariants and whether each holds on this packet.
    pub invariants: Vec<QualificationInvariant>,
    /// Whether the packet is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl OperatorQualificationPacket {
    /// Returns true when every frozen invariant holds on this packet.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the packet is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_OPERATOR_QUALIFICATION_SCHEMA_REF
            && self.record_kind == M5_OPERATOR_QUALIFICATION_RECORD_KIND
    }

    /// Returns the qualification row for a family, when present.
    pub fn family(&self, surface: OperatorSurfaceClass) -> Option<&FamilyQualificationRow> {
        self.families.iter().find(|row| row.surface == surface)
    }

    /// Returns the resolved proof for a dimension, when present.
    pub fn dimension(&self, dimension: ProofDimension) -> Option<&DimensionProof> {
        self.dimensions
            .iter()
            .find(|proof| proof.dimension == dimension)
    }

    /// Returns true when the family's claim is fully supported.
    pub fn is_family_fully_supported(&self, surface: OperatorSurfaceClass) -> bool {
        self.family(surface)
            .is_some_and(FamilyQualificationRow::is_fully_supported)
    }
}

// ---------------------------------------------------------------------------
// Projection (release-automation entry point).
// ---------------------------------------------------------------------------

/// Projects the operator-surface qualification packet from a set of proof
/// observations.
///
/// This is the release-automation entry point. It resolves each
/// [`ProofDimension`] to a [`ProofState`] against `evaluated_as_of`, then for
/// every claimed operator family evaluates the canonical-matrix dimension plus the
/// dimension that governs that family and folds them into one
/// [`ClaimSupportClass`]. A family stays fully supported only when every dimension
/// it claims is fresh; a stale or failing dimension narrows the claim and a
/// critical-dimension failure blocks it, so operator-surface proof that has aged
/// out can never leave a family green.
///
/// Dimensions with no supplied input resolve to [`ProofState::Missing`].
pub fn project_operator_qualification(
    evaluated_as_of: impl Into<String>,
    proofs: &[ProofInput],
) -> OperatorQualificationPacket {
    let evaluated_as_of = evaluated_as_of.into();

    let dimensions: Vec<DimensionProof> = ProofDimension::ALL
        .iter()
        .map(|dimension| project_dimension(*dimension, proofs, &evaluated_as_of))
        .collect();

    let families: Vec<FamilyQualificationRow> = OperatorSurfaceClass::ALL
        .iter()
        .map(|surface| project_family(*surface, &dimensions))
        .collect();

    let rollup = build_rollup(&dimensions, &families);
    let invariants = build_invariants(&dimensions, &families);
    let summary = build_summary(&rollup);

    OperatorQualificationPacket {
        record_kind: M5_OPERATOR_QUALIFICATION_RECORD_KIND.to_owned(),
        m5_operator_qualification_schema_version: M5_OPERATOR_QUALIFICATION_SCHEMA_VERSION,
        schema_ref: M5_OPERATOR_QUALIFICATION_SCHEMA_REF.to_owned(),
        packet_id: M5_OPERATOR_QUALIFICATION_PACKET_ID.to_owned(),
        as_of: evaluated_as_of,
        dimensions,
        families,
        rollup,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

fn project_dimension(
    dimension: ProofDimension,
    proofs: &[ProofInput],
    evaluated_as_of: &str,
) -> DimensionProof {
    match proofs.iter().find(|input| input.dimension == dimension) {
        Some(input) => DimensionProof {
            dimension,
            label: dimension.label().to_owned(),
            critical: dimension.is_critical(),
            proof_source_ref: input.proof_source_ref.clone(),
            contributing_proof_refs: input.contributing_proof_refs.clone(),
            captured_as_of: input.captured_as_of.clone(),
            freshness_budget_days: input.freshness_budget_days,
            state: input.resolve_state(evaluated_as_of),
            detail: input.detail.clone(),
        },
        None => DimensionProof {
            dimension,
            label: dimension.label().to_owned(),
            critical: dimension.is_critical(),
            proof_source_ref: String::new(),
            contributing_proof_refs: Vec::new(),
            captured_as_of: None,
            freshness_budget_days: 0,
            state: ProofState::Missing,
            detail: "no proof supplied for this dimension".to_owned(),
        },
    }
}

fn project_family(
    surface: OperatorSurfaceClass,
    dimensions: &[DimensionProof],
) -> FamilyQualificationRow {
    let mut verdicts = Vec::with_capacity(dimensions.len());
    let mut support = ClaimSupportClass::FullySupported;
    let mut narrowed_by = Vec::new();
    let mut blocked_by = Vec::new();

    for proof in dimensions {
        let applicable = proof.dimension.applies_to(surface);
        let effect = if applicable {
            dimension_effect(proof.state, proof.critical)
        } else {
            ClaimSupportClass::FullySupported
        };

        if applicable {
            match effect {
                ClaimSupportClass::Narrowed => narrowed_by.push(proof.dimension),
                ClaimSupportClass::Blocked => blocked_by.push(proof.dimension),
                ClaimSupportClass::FullySupported => {}
            }
            support = support.worst(effect);
        }

        verdicts.push(DimensionVerdict {
            dimension: proof.dimension,
            applicable,
            state: proof.state,
            effect,
        });
    }

    let summary = build_family_summary(surface, support, &narrowed_by, &blocked_by);

    FamilyQualificationRow {
        surface,
        surface_id: surface.surface_id(),
        label: surface.label().to_owned(),
        primary_dimension: primary_dimension(surface),
        support,
        dimension_verdicts: verdicts,
        narrowed_by,
        blocked_by,
        summary,
    }
}

fn build_rollup(
    dimensions: &[DimensionProof],
    families: &[FamilyQualificationRow],
) -> QualificationRollup {
    let count_state = |state: ProofState| dimensions.iter().filter(|d| d.state == state).count();
    QualificationRollup {
        fully_supported: families
            .iter()
            .filter(|f| f.support == ClaimSupportClass::FullySupported)
            .count(),
        narrowed: families
            .iter()
            .filter(|f| f.support == ClaimSupportClass::Narrowed)
            .count(),
        blocked: families
            .iter()
            .filter(|f| f.support == ClaimSupportClass::Blocked)
            .count(),
        stale_dimensions: count_state(ProofState::Stale),
        failing_dimensions: count_state(ProofState::Failing),
        missing_dimensions: count_state(ProofState::Missing),
    }
}

fn build_invariants(
    dimensions: &[DimensionProof],
    families: &[FamilyQualificationRow],
) -> Vec<QualificationInvariant> {
    let dimension_set_complete = ProofDimension::ALL
        .iter()
        .all(|dimension| dimensions.iter().any(|proof| proof.dimension == *dimension));

    let every_family_present = OperatorSurfaceClass::ALL
        .iter()
        .all(|surface| families.iter().any(|row| row.surface == *surface));

    // Every family claims the canonical-matrix binding, so dashboards and queues
    // resolve through the same frozen matrix rather than a parallel truth model.
    let every_family_anchored_to_matrix = families.iter().all(|row| {
        row.verdict(ProofDimension::CanonicalMatrixBinding)
            .is_some_and(|verdict| verdict.applicable)
    });

    // No family stays fully supported while any dimension it claims is not fresh —
    // the core guardrail against silent aging.
    let no_green_with_nonfresh_proof = families.iter().all(|row| {
        row.support != ClaimSupportClass::FullySupported
            || row
                .dimension_verdicts
                .iter()
                .all(|verdict| !verdict.applicable || verdict.state.is_ok())
    });

    // Every non-green family names the dimension(s) responsible — no unattributed
    // downgrade.
    let every_downgrade_is_named = families.iter().all(|row| match row.support {
        ClaimSupportClass::FullySupported => {
            row.narrowed_by.is_empty() && row.blocked_by.is_empty()
        }
        ClaimSupportClass::Narrowed => !row.narrowed_by.is_empty(),
        ClaimSupportClass::Blocked => !row.blocked_by.is_empty(),
    });

    // A failing or missing critical dimension blocks every family that claims it.
    let critical_failure_blocks = families.iter().all(|row| {
        row.dimension_verdicts.iter().all(|verdict| {
            !(verdict.applicable
                && verdict.dimension.is_critical()
                && matches!(verdict.state, ProofState::Failing | ProofState::Missing))
                || row.support == ClaimSupportClass::Blocked
        })
    });

    // The explicit release-evidence dimensions are all present.
    let acceptance_dimensions_present = [
        ProofDimension::ServiceOwnership,
        ProofDimension::RunbookStepAuthority,
        ProofDimension::HandoffBundleFidelity,
        ProofDimension::MaintenanceFailoverCommunication,
        ProofDimension::EmbeddedBoundaryHonesty,
    ]
    .iter()
    .all(|dimension| dimensions.iter().any(|proof| proof.dimension == *dimension));

    vec![
        QualificationInvariant {
            invariant_id: "dimension_set_complete".to_owned(),
            statement: "Every proof dimension resolves to exactly one global proof.".to_owned(),
            holds: dimension_set_complete,
        },
        QualificationInvariant {
            invariant_id: "every_claimed_family_present".to_owned(),
            statement: "Every claimed operator family has a qualification row.".to_owned(),
            holds: every_family_present,
        },
        QualificationInvariant {
            invariant_id: "every_family_anchored_to_canonical_matrix".to_owned(),
            statement: "Every operator family claims the canonical-matrix binding, so dashboards \
                        and queues resolve through the same frozen matrix."
                .to_owned(),
            holds: every_family_anchored_to_matrix,
        },
        QualificationInvariant {
            invariant_id: "no_fully_supported_family_with_nonfresh_proof".to_owned(),
            statement:
                "A family stays fully supported only when every dimension it claims is fresh."
                    .to_owned(),
            holds: no_green_with_nonfresh_proof,
        },
        QualificationInvariant {
            invariant_id: "every_downgrade_is_named".to_owned(),
            statement: "Every narrowed or blocked family names the responsible dimension(s)."
                .to_owned(),
            holds: every_downgrade_is_named,
        },
        QualificationInvariant {
            invariant_id: "critical_failure_blocks_claim".to_owned(),
            statement:
                "A failing or missing critical dimension blocks every family that claims it."
                    .to_owned(),
            holds: critical_failure_blocks,
        },
        QualificationInvariant {
            invariant_id: "release_evidence_dimensions_present".to_owned(),
            statement: "Service-ownership, runbook-step authority, handoff-bundle fidelity, \
                        maintenance/failover communication, and embedded-boundary honesty rows \
                        are all present."
                .to_owned(),
            holds: acceptance_dimensions_present,
        },
    ]
}

fn build_family_summary(
    surface: OperatorSurfaceClass,
    support: ClaimSupportClass,
    narrowed_by: &[ProofDimension],
    blocked_by: &[ProofDimension],
) -> String {
    match support {
        ClaimSupportClass::FullySupported => format!(
            "{label}: fully supported; every claimed dimension is fresh.",
            label = surface.label(),
        ),
        ClaimSupportClass::Narrowed => format!(
            "{label}: narrowed by {reasons}.",
            label = surface.label(),
            reasons = join_dimensions(narrowed_by),
        ),
        ClaimSupportClass::Blocked => format!(
            "{label}: blocked by {reasons}.",
            label = surface.label(),
            reasons = join_dimensions(blocked_by),
        ),
    }
}

fn build_summary(rollup: &QualificationRollup) -> String {
    format!(
        "Operator-surface qualification: {full} fully supported, {narrowed} narrowed, {blocked} \
         blocked across {total} families ({stale} stale, {failing} failing, {missing} missing \
         dimension proof(s)).",
        full = rollup.fully_supported,
        narrowed = rollup.narrowed,
        blocked = rollup.blocked,
        total = rollup.fully_supported + rollup.narrowed + rollup.blocked,
        stale = rollup.stale_dimensions,
        failing = rollup.failing_dimensions,
        missing = rollup.missing_dimensions,
    )
}

fn join_dimensions(dimensions: &[ProofDimension]) -> String {
    if dimensions.is_empty() {
        return "(none)".to_owned();
    }
    dimensions
        .iter()
        .map(|dimension| dimension.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------------------
// Canonical binding to the in-code proof sources.
// ---------------------------------------------------------------------------

/// Builds the canonical operator-surface qualification packet by binding the real
/// in-code proof sources.
///
/// Each dimension's pass state is read from its upstream lane's
/// `all_invariants_hold` and its capture stamp from the lane's `AS_OF` constant,
/// then [`project_operator_qualification`] folds them into the per-family
/// verdicts. The checked-in fixture and the replay gate freeze the result so the
/// certified state cannot drift from the published artifact.
pub fn operator_qualification_packet() -> OperatorQualificationPacket {
    project_operator_qualification(M5_OPERATOR_QUALIFICATION_AS_OF, &canonical_proof_inputs())
}

/// The canonical proof inputs, read from the in-code operator-surface lanes.
fn canonical_proof_inputs() -> Vec<ProofInput> {
    let matrix_ok = operator_surface_matrix().all_invariants_hold();
    let boards_ok = operator_board_set().all_invariants_hold();
    let triage_ok = triage_inbox_set().all_invariants_hold();
    let plans_ok = action_plan_set().all_invariants_hold();
    let handoff_ok = handoff_digest_set().all_invariants_hold();
    let panes_ok = response_pane_set().all_invariants_hold();
    let windows_ok = maintenance_window_set().all_invariants_hold();
    let embedded_ok = embedded_surface_set().all_invariants_hold();

    let matrix = M5_OPERATOR_SURFACES_SCHEMA_REF.to_owned();
    let boards = M5_OPERATOR_BOARDS_SCHEMA_REF.to_owned();
    let triage = M5_TRIAGE_INBOX_SCHEMA_REF.to_owned();
    let plans = M5_ACTION_PLANS_SCHEMA_REF.to_owned();
    let handoff = M5_HANDOFF_DIGESTS_SCHEMA_REF.to_owned();
    let panes = M5_RESPONSE_PANES_SCHEMA_REF.to_owned();
    let windows = M5_MAINTENANCE_WINDOWS_SCHEMA_REF.to_owned();
    let embedded = M5_EMBEDDED_DASHBOARDS_SCHEMA_REF.to_owned();

    vec![
        ProofInput {
            dimension: ProofDimension::CanonicalMatrixBinding,
            proof_source_ref: matrix.clone(),
            contributing_proof_refs: vec![matrix.clone()],
            captured_as_of: Some(M5_OPERATOR_SURFACES_AS_OF.to_owned()),
            passing: matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Every operator surface resolves through the one frozen operator-surface \
                     matrix, binding dashboards and queues to canonical objects."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::OverviewTruth,
            proof_source_ref: boards.clone(),
            contributing_proof_refs: vec![boards, matrix.clone()],
            captured_as_of: Some(M5_OPERATOR_BOARDS_AS_OF.to_owned()),
            passing: boards_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail:
                "Overview boards downgrade unconfirmed-green tiles and keep owner/blocker-waiver \
                     truth and canonical object linkage."
                    .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::TriageTruth,
            proof_source_ref: triage.clone(),
            contributing_proof_refs: vec![triage, matrix.clone()],
            captured_as_of: Some(M5_TRIAGE_INBOX_AS_OF.to_owned()),
            passing: triage_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Triage inboxes name order and narrowing reasons and point at the same \
                     incident/support/review/admin objects."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::ActionPlanContinuity,
            proof_source_ref: plans.clone(),
            contributing_proof_refs: vec![plans, matrix.clone()],
            captured_as_of: Some(M5_ACTION_PLANS_AS_OF.to_owned()),
            passing: plans_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Action plans keep per-item local-versus-external state so a local checkoff \
                     never resolves a provider-owned object on its own."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::HandoffBundleFidelity,
            proof_source_ref: handoff.clone(),
            contributing_proof_refs: vec![handoff, matrix.clone()],
            captured_as_of: Some(M5_HANDOFF_DIGESTS_AS_OF.to_owned()),
            passing: handoff_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Handoff bundles and shift digests preserve object identity, ownership, \
                     redaction, scope, and live-versus-snapshot truth on export."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::ServiceOwnership,
            proof_source_ref: panes.clone(),
            contributing_proof_refs: vec![panes.clone(), matrix.clone()],
            captured_as_of: Some(M5_RESPONSE_PANES_AS_OF.to_owned()),
            passing: panes_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Service-ownership / on-call strips keep owner, contract state, and \
                     local-continuity posture visible."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::RunbookStepAuthority,
            proof_source_ref: panes.clone(),
            contributing_proof_refs: vec![panes, matrix.clone()],
            captured_as_of: Some(M5_RESPONSE_PANES_AS_OF.to_owned()),
            passing: panes_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Runbook-step cards preview and admit a mutating step before it runs."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::MaintenanceFailoverCommunication,
            proof_source_ref: windows.clone(),
            contributing_proof_refs: vec![windows, matrix.clone()],
            captured_as_of: Some(M5_MAINTENANCE_WINDOWS_AS_OF.to_owned()),
            passing: windows_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Maintenance and failover notices carry exact times, named blocked write \
                     classes, and local-safe / publish-later continuity."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::EmbeddedBoundaryHonesty,
            proof_source_ref: embedded.clone(),
            contributing_proof_refs: vec![embedded, matrix],
            captured_as_of: Some(M5_EMBEDDED_DASHBOARDS_AS_OF.to_owned()),
            passing: embedded_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Embedded provider/auth boundaries disclose owner/origin and capability truth \
                     and never impersonate a native approval."
                .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Freshness derivation.
// ---------------------------------------------------------------------------

/// Derives the proof state from a capture stamp, pass state, freshness budget,
/// and evaluation stamp.
///
/// Missing capture means no proof; a non-passing proof is failing; otherwise the
/// proof is stale when its age exceeds the budget. A capture stamp that cannot be
/// parsed is treated as stale: the evidence exists but its age cannot be trusted,
/// so the conservative outcome is to narrow.
fn derive_proof_state(
    captured_as_of: Option<&str>,
    passing: bool,
    budget_days: i64,
    evaluated_as_of: &str,
) -> ProofState {
    let Some(captured) = captured_as_of else {
        return ProofState::Missing;
    };
    if !passing {
        return ProofState::Failing;
    }
    match (
        parse_civil_days(captured),
        parse_civil_days(evaluated_as_of),
    ) {
        (Some(captured_days), Some(evaluated_days)) => {
            if evaluated_days - captured_days > budget_days.max(0) {
                ProofState::Stale
            } else {
                ProofState::Fresh
            }
        }
        _ => ProofState::Stale,
    }
}

/// Parses the `YYYY-MM-DD` date prefix of an ISO 8601 stamp into a day count.
fn parse_civil_days(stamp: &str) -> Option<i64> {
    let date = stamp.get(..10)?;
    let mut parts = date.split('-');
    let year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some(days_from_civil(year, month, day))
}

/// Returns the number of days since the Unix epoch for a proleptic Gregorian
/// date, using Howard Hinnant's `days_from_civil` algorithm.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146097 + day_of_era - 719468
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the export-safe human-readable lines for a qualification packet.
///
/// This is the shared projection consumed by the operator About/help surface, the
/// service-health and compatibility surfaces, the headless CLI emitter, and
/// support export, so none of them clone the certified state from each other.
pub fn operator_qualification_lines(packet: &OperatorQualificationPacket) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Operator-surface qualification — {} (as of {})",
        packet.packet_id, packet.as_of
    ));
    lines.push(format!(
        "rollup: fully_supported={} narrowed={} blocked={} | stale={} failing={} missing={}",
        packet.rollup.fully_supported,
        packet.rollup.narrowed,
        packet.rollup.blocked,
        packet.rollup.stale_dimensions,
        packet.rollup.failing_dimensions,
        packet.rollup.missing_dimensions,
    ));

    lines.push("Proof dimensions:".to_owned());
    for proof in &packet.dimensions {
        lines.push(format!(
            "  {dim} [{state}] critical={critical} budget={budget}d source={source} captured={captured}",
            dim = proof.dimension.as_str(),
            state = proof.state.as_str(),
            critical = proof.critical,
            budget = proof.freshness_budget_days,
            source = if proof.proof_source_ref.is_empty() {
                "(none)"
            } else {
                proof.proof_source_ref.as_str()
            },
            captured = proof.captured_as_of.as_deref().unwrap_or("(none)"),
        ));
    }

    lines.push("Families:".to_owned());
    for family in &packet.families {
        lines.push(format!(
            "  {surface} [{support}] primary={primary}{detail}",
            surface = family.surface.as_str(),
            support = family.support.as_str(),
            primary = family.primary_dimension.as_str(),
            detail = family_reason_suffix(family),
        ));
    }

    lines.push(packet.summary.clone());
    lines
}

fn family_reason_suffix(family: &FamilyQualificationRow) -> String {
    match family.support {
        ClaimSupportClass::FullySupported => String::new(),
        ClaimSupportClass::Narrowed => {
            format!(" narrowed_by={}", join_dimensions(&family.narrowed_by))
        }
        ClaimSupportClass::Blocked => {
            format!(" blocked_by={}", join_dimensions(&family.blocked_by))
        }
    }
}
