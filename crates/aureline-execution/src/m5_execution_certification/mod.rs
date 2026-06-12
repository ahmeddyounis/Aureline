//! Canonical M5 execution-certification matrix: the single qualification report that
//! aggregates the build-intelligence, target-discovery, host-boundary,
//! managed-workspace lifecycle, cluster-context, mutation/handoff-review, and
//! live-resource depth lanes into one non-inheriting certification gate.
//!
//! Each [`CertificationRow`] certifies one M5 execution or ops-adjacent depth lane
//! ([`CertifiedLane`]) against the canonical evidence packet it draws from, and
//! answers, for that lane, who owns the evidence ([`CertificationRow::owner`]), how
//! fresh that evidence is ([`EvidenceFreshness`]), how much of the claimed profile it
//! covers ([`ProfileCoverage`]), how its qualification drills came out
//! ([`DrillOutcome`]), and how the evidence was attested ([`EvidenceProvenance`]).
//! The row then publishes a [`QualificationLevel`] no input can exceed.
//!
//! The [`QualificationLevel`] a lane may publish is the weakest ceiling implied by its
//! observed states, so stale or expired evidence, partial or absent profile coverage,
//! a regressed or failed drill, or unverified or unverifiable attestation all narrow or
//! withhold the published qualification automatically. The guardrail this enforces: a
//! lane never graduates to a blanket "managed ready" or "remote parity" claim by
//! inertia — a lane whose proof has gone stale, whose drills regressed, or whose
//! coverage shrank is downgraded to a narrower deployment-profile or lifecycle label,
//! or withdrawn from publication entirely, rather than left quietly green. The
//! [`CertificationDecision`] records the gate's action — certify the lane, qualify it to
//! a narrower profile, provision it to a narrower lifecycle, or withdraw it — and the
//! recomputed [`DowngradeReason`]s and [`DowngradePath`] explain it; all are validated
//! against the gate.
//!
//! The certification model is a report and a receipt, not a marketing badge. Every lane
//! resolves through one certification row that names its source packet, its drill
//! evidence, and a machine-readable certification receipt before any marketed row, docs
//! badge, help/service-health surface, or support export restates it. Because each row
//! also carries a release-evidence ref, a service-health ref, a docs-badge ref, and a
//! support-export ref, release evidence, help/service-health, docs, and support exports
//! ingest the *same* certification packet rather than parallel spreadsheets, so a stale
//! lane cannot stay green in one surface while it is downgraded in another.
//!
//! The lane vocabulary is closed and provenance-bound. [`CertifiedLane`] is the single
//! controlled vocabulary the certification reuses, and each lane is pinned to the
//! canonical execution-truth packet it certifies via [`CertifiedLane::source_packet`], so
//! a certified build-intelligence lane never lends its confidence to a withdrawn
//! live-resource lane, and no ops-adjacent lane inherits a broader local or desktop claim.
//!
//! The packet is checked in at
//! `artifacts/execution/m5/m5-execution-certification.json` and embedded here. It is
//! metadata-only: every field is a typed state or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, host tokens, or control-plane secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 execution-certification matrix schema version.
pub const M5_EXECUTION_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_EXECUTION_CERTIFICATION_RECORD_KIND: &str = "m5_execution_certification_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_EXECUTION_CERTIFICATION_PATH: &str =
    "artifacts/execution/m5/m5-execution-certification.json";

/// Embedded checked-in packet JSON.
pub const M5_EXECUTION_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/execution/m5/m5-execution-certification.json"
));

/// An M5 execution or ops-adjacent depth lane the certification graduates.
///
/// Each lane is certified from the canonical execution-truth packet it draws its
/// evidence from, so the certification aggregates the B16 packets into one report
/// instead of re-deriving each lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedLane {
    /// Build-intelligence depth: build-event-stream, protocol, structured-import, and
    /// heuristic adapter parity and health.
    BuildIntelligence,
    /// Target-context discovery depth: how a target was discovered and how certain that is.
    TargetContextDiscovery,
    /// Host-boundary depth: where work ran and how certain that attribution is.
    HostBoundary,
    /// Managed-workspace lifecycle depth: provision, suspend/resume, rebuild, and expiry.
    ManagedWorkspaceLifecycle,
    /// Cluster-context and infrastructure depth: desired/rendered/plan/live views.
    ClusterContextInfrastructure,
    /// Mutation and handoff review depth: preview/apply/handoff actor and rollback semantics.
    MutationHandoffReview,
    /// Live-resource target-context depth: live-resource operations and their context.
    LiveResourceContext,
}

impl CertifiedLane {
    /// Every certified lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::BuildIntelligence,
        Self::TargetContextDiscovery,
        Self::HostBoundary,
        Self::ManagedWorkspaceLifecycle,
        Self::ClusterContextInfrastructure,
        Self::MutationHandoffReview,
        Self::LiveResourceContext,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildIntelligence => "build_intelligence",
            Self::TargetContextDiscovery => "target_context_discovery",
            Self::HostBoundary => "host_boundary",
            Self::ManagedWorkspaceLifecycle => "managed_workspace_lifecycle",
            Self::ClusterContextInfrastructure => "cluster_context_infrastructure",
            Self::MutationHandoffReview => "mutation_handoff_review",
            Self::LiveResourceContext => "live_resource_context",
        }
    }

    /// Repo-relative path to the canonical execution-truth packet this lane certifies.
    ///
    /// The certification is pinned to this packet so a lane never certifies a claim its
    /// own source packet does not back, and the `packet_ref` recorded on every row is
    /// validated against it.
    pub const fn source_packet(self) -> &'static str {
        match self {
            Self::BuildIntelligence => "artifacts/execution/m5/m5-adapter-parity-and-health.json",
            Self::TargetContextDiscovery => "artifacts/execution/m5/m5-target-discovery.json",
            Self::HostBoundary => "artifacts/execution/m5/m5-host-boundary.json",
            Self::ManagedWorkspaceLifecycle | Self::ClusterContextInfrastructure => {
                "artifacts/execution/m5/m5-build-and-host-governance.json"
            }
            Self::MutationHandoffReview | Self::LiveResourceContext => {
                "artifacts/execution/m5/m5-mutation-and-handoff-review.json"
            }
        }
    }

    /// Whether this is an ops-adjacent lane — a managed-workspace, service-plane,
    /// cluster-context, or live-resource lane that must narrow safely instead of
    /// inheriting a broader local or desktop claim.
    pub const fn is_ops_adjacent(self) -> bool {
        matches!(
            self,
            Self::ManagedWorkspaceLifecycle
                | Self::ClusterContextInfrastructure
                | Self::MutationHandoffReview
                | Self::LiveResourceContext
        )
    }
}

/// How qualified a lane's published certification claim is.
///
/// Ordered low-to-high by [`QualificationLevel::rank`]: a [`QualificationLevel::Withdrawn`]
/// lane has no publishable claim, and a [`QualificationLevel::Certified`] lane is backed by
/// fresh, fully-covered, drill-passing, verified evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationLevel {
    /// Certified with current, complete, verified proof.
    Certified,
    /// Narrowed to a narrower deployment-profile or provider-family label.
    ProfileQualified,
    /// Narrowed to a narrower lifecycle label; provisional.
    LifecycleProvisional,
    /// Withdrawn from publication; no publishable claim.
    Withdrawn,
}

impl QualificationLevel {
    /// Every qualification level, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::ProfileQualified,
        Self::LifecycleProvisional,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::ProfileQualified => "profile_qualified",
            Self::LifecycleProvisional => "lifecycle_provisional",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Monotonic rank; higher means more qualified.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Withdrawn => 0,
            Self::LifecycleProvisional => 1,
            Self::ProfileQualified => 2,
            Self::Certified => 3,
        }
    }

    /// The weaker (lower-rank) of two qualification levels.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// How fresh the evidence backing a lane is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is recent and within tolerance.
    Recent,
    /// The evidence is stale; caps at lifecycle-provisional.
    Stale,
    /// The evidence is expired; caps at withdrawn.
    Expired,
}

impl EvidenceFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Fresh, Self::Recent, Self::Stale, Self::Expired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Stale => "stale",
            Self::Expired => "expired",
        }
    }

    /// Highest qualification level this freshness state permits a lane to publish.
    pub const fn qualification_ceiling(self) -> QualificationLevel {
        match self {
            Self::Fresh | Self::Recent => QualificationLevel::Certified,
            Self::Stale => QualificationLevel::LifecycleProvisional,
            Self::Expired => QualificationLevel::Withdrawn,
        }
    }

    /// Whether this state raises the [`DowngradeReason::StaleEvidence`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

/// How much of the claimed profile the lane's evidence covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileCoverage {
    /// Coverage spans the full claimed profile.
    Full,
    /// Coverage is partial; caps at profile-qualified.
    Partial,
    /// Coverage is minimal; caps at lifecycle-provisional.
    Minimal,
    /// Coverage is absent; caps at withdrawn.
    Absent,
}

impl ProfileCoverage {
    /// Every coverage state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Full, Self::Partial, Self::Minimal, Self::Absent];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Partial => "partial",
            Self::Minimal => "minimal",
            Self::Absent => "absent",
        }
    }

    /// Highest qualification level this coverage state permits a lane to publish.
    pub const fn qualification_ceiling(self) -> QualificationLevel {
        match self {
            Self::Full => QualificationLevel::Certified,
            Self::Partial => QualificationLevel::ProfileQualified,
            Self::Minimal => QualificationLevel::LifecycleProvisional,
            Self::Absent => QualificationLevel::Withdrawn,
        }
    }

    /// Whether this state raises the [`DowngradeReason::PartialProfileCoverage`] trigger.
    pub const fn is_partial_trigger(self) -> bool {
        matches!(self, Self::Partial | Self::Minimal | Self::Absent)
    }
}

/// How a lane's qualification drills came out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillOutcome {
    /// Every drill passed.
    Passed,
    /// Some drills passed; caps at profile-qualified.
    PartiallyPassed,
    /// Drills were inconclusive; caps at lifecycle-provisional.
    Inconclusive,
    /// A drill failed; caps at withdrawn.
    Failed,
}

impl DrillOutcome {
    /// Every drill outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Passed,
        Self::PartiallyPassed,
        Self::Inconclusive,
        Self::Failed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::PartiallyPassed => "partially_passed",
            Self::Inconclusive => "inconclusive",
            Self::Failed => "failed",
        }
    }

    /// Highest qualification level this drill outcome permits a lane to publish.
    pub const fn qualification_ceiling(self) -> QualificationLevel {
        match self {
            Self::Passed => QualificationLevel::Certified,
            Self::PartiallyPassed => QualificationLevel::ProfileQualified,
            Self::Inconclusive => QualificationLevel::LifecycleProvisional,
            Self::Failed => QualificationLevel::Withdrawn,
        }
    }

    /// Whether this state raises the [`DowngradeReason::DrillRegression`] trigger.
    pub const fn is_regression_trigger(self) -> bool {
        matches!(
            self,
            Self::PartiallyPassed | Self::Inconclusive | Self::Failed
        )
    }
}

/// How the evidence backing a lane was attested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceProvenance {
    /// The evidence was verified against the live packet.
    Verified,
    /// The evidence is attested by a recorded snapshot; caps at profile-qualified.
    Attested,
    /// The evidence is unverified; caps at lifecycle-provisional.
    Unverified,
    /// The evidence cannot be verified; caps at withdrawn.
    Unverifiable,
}

impl EvidenceProvenance {
    /// Every provenance state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Verified,
        Self::Attested,
        Self::Unverified,
        Self::Unverifiable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Attested => "attested",
            Self::Unverified => "unverified",
            Self::Unverifiable => "unverifiable",
        }
    }

    /// Highest qualification level this provenance state permits a lane to publish.
    pub const fn qualification_ceiling(self) -> QualificationLevel {
        match self {
            Self::Verified => QualificationLevel::Certified,
            Self::Attested => QualificationLevel::ProfileQualified,
            Self::Unverified => QualificationLevel::LifecycleProvisional,
            Self::Unverifiable => QualificationLevel::Withdrawn,
        }
    }

    /// Whether this state raises the [`DowngradeReason::UnverifiedEvidence`] trigger.
    pub const fn is_unverified_trigger(self) -> bool {
        matches!(self, Self::Unverified | Self::Unverifiable)
    }
}

/// The recovery path surfaced when a lane's certification is narrowed or withdrawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradePath {
    /// Re-run the lane's drills and refresh its proof.
    RefreshEvidence,
    /// Publish a narrower deployment-profile or provider-family label.
    NarrowProfile,
    /// Publish a narrower lifecycle label.
    NarrowLifecycle,
    /// Withdraw the lane from publication.
    WithdrawClaim,
    /// No downgrade is needed; only valid when the lane is certified.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl DowngradePath {
    /// Every downgrade path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RefreshEvidence,
        Self::NarrowProfile,
        Self::NarrowLifecycle,
        Self::WithdrawClaim,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshEvidence => "refresh_evidence",
            Self::NarrowProfile => "narrow_profile",
            Self::NarrowLifecycle => "narrow_lifecycle",
            Self::WithdrawClaim => "withdraw_claim",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real downgrade path the lane owner can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A headline reason the certification gate narrows a lane.
///
/// These are the canonical downgrade reasons: stale evidence, partial profile coverage,
/// a drill regression, and unverified evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// The lane's evidence is stale or expired.
    StaleEvidence,
    /// The lane's profile coverage is partial, minimal, or absent.
    PartialProfileCoverage,
    /// The lane's drills regressed, were inconclusive, or failed.
    DrillRegression,
    /// The lane's evidence is unverified or unverifiable.
    UnverifiedEvidence,
}

impl DowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StaleEvidence,
        Self::PartialProfileCoverage,
        Self::DrillRegression,
        Self::UnverifiedEvidence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleEvidence => "stale_evidence",
            Self::PartialProfileCoverage => "partial_profile_coverage",
            Self::DrillRegression => "drill_regression",
            Self::UnverifiedEvidence => "unverified_evidence",
        }
    }
}

/// The action the certification gate takes on a lane relative to a clean certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDecision {
    /// No narrowing; the lane is certified.
    Certify,
    /// The lane is qualified to a narrower deployment-profile label.
    QualifyProfile,
    /// The lane is provisioned to a narrower lifecycle label.
    ProvisionLifecycle,
    /// The lane's claim is withdrawn from publication.
    Withdraw,
}

impl CertificationDecision {
    /// Every certification decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certify,
        Self::QualifyProfile,
        Self::ProvisionLifecycle,
        Self::Withdraw,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certify => "certify",
            Self::QualifyProfile => "qualify_profile",
            Self::ProvisionLifecycle => "provision_lifecycle",
            Self::Withdraw => "withdraw",
        }
    }

    /// Whether the gate narrowed or withdrew the lane's certification.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Certify)
    }

    /// The decision implied by a published qualification level.
    pub const fn for_level(level: QualificationLevel) -> Self {
        match level {
            QualificationLevel::Certified => Self::Certify,
            QualificationLevel::ProfileQualified => Self::QualifyProfile,
            QualificationLevel::LifecycleProvisional => Self::ProvisionLifecycle,
            QualificationLevel::Withdrawn => Self::Withdraw,
        }
    }
}

/// One certification row for an M5 execution or ops-adjacent depth lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationRow {
    /// Stable certification-row id.
    pub lane_id: String,
    /// Depth lane this row certifies.
    pub lane: CertifiedLane,
    /// Owner accountable for the lane's evidence and drills.
    pub owner: String,
    /// How fresh the lane's evidence is.
    pub evidence_freshness: EvidenceFreshness,
    /// How much of the claimed profile the lane's evidence covers.
    pub profile_coverage: ProfileCoverage,
    /// How the lane's qualification drills came out.
    pub drill_outcome: DrillOutcome,
    /// How the lane's evidence was attested.
    pub evidence_provenance: EvidenceProvenance,
    /// Qualification the lane's own evidence asserts, before the gate.
    pub declared_qualification: QualificationLevel,
    /// Qualification actually published after the gate narrows the lane.
    ///
    /// Must equal [`CertificationRow::effective_qualification`].
    pub published_qualification: QualificationLevel,
    /// Decision the gate takes; must equal the recomputed decision.
    pub certification_decision: CertificationDecision,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Downgrade path surfaced when the certification is narrowed or withdrawn.
    pub downgrade_path: DowngradePath,
    /// Supported deployment-profile or provider-family labels this lane still backs.
    #[serde(default)]
    pub supported_profiles: Vec<String>,
    /// Caveats attached to the published claim.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale or missing.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref to the canonical execution-truth packet this lane certifies.
    ///
    /// Must equal [`CertifiedLane::source_packet`].
    pub packet_ref: String,
    /// Ref to the qualification-drill evidence.
    pub drill_ref: String,
    /// Ref to the lane's supporting evidence.
    pub evidence_ref: String,
    /// Ref to the machine-readable certification receipt for audit and release evidence.
    pub certification_receipt_ref: String,
    /// Ref binding this row into the release-evidence surface.
    pub release_evidence_ref: String,
    /// Ref binding this row into the help/service-health surface.
    pub service_health_ref: String,
    /// Ref binding this row into the docs-badge surface.
    pub docs_badge_ref: String,
    /// Ref binding this row into the support-export surface.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl CertificationRow {
    /// The qualification the lane's own evidence asserted, before environmental narrowing.
    pub fn capability_floor(&self) -> QualificationLevel {
        self.declared_qualification
    }

    /// The qualification the gate permits this lane to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the evidence
    /// freshness, profile coverage, drill outcome, and evidence provenance, so stale or
    /// expired evidence, partial or absent coverage, a regressed drill, or unverified
    /// evidence can never publish a certified claim.
    pub fn effective_qualification(&self) -> QualificationLevel {
        self.capability_floor()
            .min(self.evidence_freshness.qualification_ceiling())
            .min(self.profile_coverage.qualification_ceiling())
            .min(self.drill_outcome.qualification_ceiling())
            .min(self.evidence_provenance.qualification_ceiling())
    }

    /// The headline downgrade reasons recomputed from the lane's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<DowngradeReason> {
        let mut reasons = Vec::new();
        if self.evidence_freshness.is_stale_trigger() {
            reasons.push(DowngradeReason::StaleEvidence);
        }
        if self.profile_coverage.is_partial_trigger() {
            reasons.push(DowngradeReason::PartialProfileCoverage);
        }
        if self.drill_outcome.is_regression_trigger() {
            reasons.push(DowngradeReason::DrillRegression);
        }
        if self.evidence_provenance.is_unverified_trigger() {
            reasons.push(DowngradeReason::UnverifiedEvidence);
        }
        reasons
    }

    /// The decision the gate must record for this lane, derived from its effective
    /// qualification.
    pub fn required_decision(&self) -> CertificationDecision {
        CertificationDecision::for_level(self.effective_qualification())
    }

    /// Whether the lane publishes a clean certified claim.
    pub fn is_certified(&self) -> bool {
        self.effective_qualification() == QualificationLevel::Certified
    }

    /// Whether the gate narrowed the published qualification below what the lane declared.
    ///
    /// This is the automatic downgrade: a stale, partial, regressed, or unverified lane
    /// that declared a stronger claim has its published qualification lowered rather than
    /// left green.
    pub fn is_downgraded(&self) -> bool {
        self.effective_qualification().rank() < self.capability_floor().rank()
    }

    /// Whether the lane carries its own non-empty source, drill, evidence, receipt, and
    /// downstream-consumer refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.packet_ref.trim().is_empty()
            && !self.drill_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.certification_receipt_ref.trim().is_empty()
            && !self.release_evidence_ref.trim().is_empty()
            && !self.service_health_ref.trim().is_empty()
            && !self.docs_badge_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published qualification, decision, and downgrade reasons all agree
    /// with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_qualification == self.effective_qualification()
            && self.certification_decision == self.required_decision()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ExecutionCertificationSummary {
    /// Total lane rows.
    pub total_lanes: usize,
    /// Number of claimed lanes.
    pub lane_count: usize,
    /// Lanes published as certified.
    pub certified_lanes: usize,
    /// Lanes narrowed to a profile-qualified claim.
    pub profile_qualified_lanes: usize,
    /// Lanes narrowed to a lifecycle-provisional claim.
    pub lifecycle_provisional_lanes: usize,
    /// Lanes withdrawn from publication.
    pub withdrawn_lanes: usize,
    /// Lanes the gate certified.
    pub certify_decisions: usize,
    /// Lanes the gate qualified to a narrower profile.
    pub qualify_profile_decisions: usize,
    /// Lanes the gate provisioned to a narrower lifecycle.
    pub provision_lifecycle_decisions: usize,
    /// Lanes the gate withdrew.
    pub withdraw_decisions: usize,
    /// Lanes whose published qualification was downgraded below what they declared.
    pub downgraded_lanes: usize,
    /// Ops-adjacent lanes.
    pub ops_adjacent_lanes: usize,
    /// Lanes whose evidence is stale or expired.
    pub stale_evidence_lanes: usize,
    /// Lanes carrying at least one downgrade reason.
    pub lanes_with_downgrade_reasons: usize,
}

/// A redaction-safe export row projected from a certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionCertificationExportRow {
    /// Certification-row id.
    pub lane_id: String,
    /// Lane token.
    pub lane: String,
    /// Owner accountable for the lane.
    pub owner: String,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Profile-coverage token.
    pub profile_coverage: String,
    /// Drill-outcome token.
    pub drill_outcome: String,
    /// Evidence-provenance token.
    pub evidence_provenance: String,
    /// Declared-qualification token.
    pub declared_qualification: String,
    /// Published-qualification token.
    pub published_qualification: String,
    /// Certification-decision token.
    pub certification_decision: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Downgrade-path token.
    pub downgrade_path: String,
    /// Supported deployment-profile or provider-family labels.
    pub supported_profiles: Vec<String>,
    /// Caveats attached to the published claim.
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale or missing.
    pub stale_or_missing_fields: Vec<String>,
    /// Source-packet ref this lane certifies.
    pub packet_ref: String,
    /// Certification-receipt ref.
    pub certification_receipt_ref: String,
    /// Release-evidence ref.
    pub release_evidence_ref: String,
    /// Service-health ref.
    pub service_health_ref: String,
    /// Docs-badge ref.
    pub docs_badge_ref: String,
    /// Support-export ref.
    pub support_export_ref: String,
    /// Whether the lane is ops-adjacent.
    pub ops_adjacent: bool,
    /// Whether the lane publishes a certified claim.
    pub certified: bool,
    /// Whether the published qualification was downgraded below the declared claim.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the canonical certification index
/// downstream surfaces render instead of restating each lane's qualification by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionCertificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub lanes: Vec<M5ExecutionCertificationExportRow>,
    /// Whether every lane's published qualification and decision agree with the gate.
    pub all_lanes_gate_consistent: bool,
    /// Lanes that publish a certified claim.
    pub certified_count: usize,
    /// Lanes the gate narrowed or withdrew.
    pub narrowed_count: usize,
    /// Lanes the gate withdrew entirely.
    pub withdrawn_count: usize,
}

/// The typed M5 execution-certification matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ExecutionCertificationMatrix {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Claimed lanes; one row per lane.
    pub lanes: Vec<CertifiedLane>,
    /// Closed qualification-level vocabulary.
    pub qualification_levels: Vec<QualificationLevel>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_states: Vec<EvidenceFreshness>,
    /// Closed profile-coverage vocabulary.
    pub profile_coverage_states: Vec<ProfileCoverage>,
    /// Closed drill-outcome vocabulary.
    pub drill_outcomes: Vec<DrillOutcome>,
    /// Closed evidence-provenance vocabulary.
    pub evidence_provenance_states: Vec<EvidenceProvenance>,
    /// Closed downgrade-path vocabulary.
    pub downgrade_paths: Vec<DowngradePath>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed certification-decision vocabulary.
    pub certification_decisions: Vec<CertificationDecision>,
    /// Certification rows, one per claimed lane.
    #[serde(default)]
    pub certifications: Vec<CertificationRow>,
    /// Summary counts.
    pub summary: M5ExecutionCertificationSummary,
}

impl M5ExecutionCertificationMatrix {
    /// Returns the row for a claimed lane.
    pub fn certification(&self, lane: CertifiedLane) -> Option<&CertificationRow> {
        self.certifications.iter().find(|c| c.lane == lane)
    }

    /// Lanes that publish a certified claim.
    pub fn certified_lanes(&self) -> impl Iterator<Item = &CertificationRow> {
        self.certifications.iter().filter(|c| c.is_certified())
    }

    /// Lanes the gate narrowed or withdrew in any way.
    pub fn narrowed_lanes(&self) -> impl Iterator<Item = &CertificationRow> {
        self.certifications
            .iter()
            .filter(|c| c.required_decision().is_narrowed())
    }

    /// Lanes the gate withdrew entirely.
    pub fn withdrawn_lanes(&self) -> impl Iterator<Item = &CertificationRow> {
        self.certifications
            .iter()
            .filter(|c| c.required_decision() == CertificationDecision::Withdraw)
    }

    /// Whether every lane's stored published qualification, decision, and reasons agree with
    /// the recomputed gate decision.
    pub fn all_lanes_gate_consistent(&self) -> bool {
        self.certifications.iter().all(|c| c.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5ExecutionCertificationSummary {
        let count_published = |level: QualificationLevel| {
            self.certifications
                .iter()
                .filter(|c| c.published_qualification == level)
                .count()
        };
        let count_decision = |decision: CertificationDecision| {
            self.certifications
                .iter()
                .filter(|c| c.certification_decision == decision)
                .count()
        };
        M5ExecutionCertificationSummary {
            total_lanes: self.certifications.len(),
            lane_count: self.lanes.len(),
            certified_lanes: count_published(QualificationLevel::Certified),
            profile_qualified_lanes: count_published(QualificationLevel::ProfileQualified),
            lifecycle_provisional_lanes: count_published(QualificationLevel::LifecycleProvisional),
            withdrawn_lanes: count_published(QualificationLevel::Withdrawn),
            certify_decisions: count_decision(CertificationDecision::Certify),
            qualify_profile_decisions: count_decision(CertificationDecision::QualifyProfile),
            provision_lifecycle_decisions: count_decision(
                CertificationDecision::ProvisionLifecycle,
            ),
            withdraw_decisions: count_decision(CertificationDecision::Withdraw),
            downgraded_lanes: self
                .certifications
                .iter()
                .filter(|c| c.is_downgraded())
                .count(),
            ops_adjacent_lanes: self
                .certifications
                .iter()
                .filter(|c| c.lane.is_ops_adjacent())
                .count(),
            stale_evidence_lanes: self
                .certifications
                .iter()
                .filter(|c| c.evidence_freshness.is_stale_trigger())
                .count(),
            lanes_with_downgrade_reasons: self
                .certifications
                .iter()
                .filter(|c| !c.downgrade_reasons.is_empty())
                .count(),
        }
    }

    /// Produces the certification index downstream surfaces — release evidence,
    /// help/service-health, docs badges, and support exports — render instead of restating
    /// each lane's qualification posture by hand.
    pub fn export_projection(&self) -> M5ExecutionCertificationExportProjection {
        let lanes = self
            .certifications
            .iter()
            .map(|c| M5ExecutionCertificationExportRow {
                lane_id: c.lane_id.clone(),
                lane: c.lane.as_str().to_owned(),
                owner: c.owner.clone(),
                evidence_freshness: c.evidence_freshness.as_str().to_owned(),
                profile_coverage: c.profile_coverage.as_str().to_owned(),
                drill_outcome: c.drill_outcome.as_str().to_owned(),
                evidence_provenance: c.evidence_provenance.as_str().to_owned(),
                declared_qualification: c.declared_qualification.as_str().to_owned(),
                published_qualification: c.published_qualification.as_str().to_owned(),
                certification_decision: c.certification_decision.as_str().to_owned(),
                downgrade_reasons: c
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                downgrade_path: c.downgrade_path.as_str().to_owned(),
                supported_profiles: c.supported_profiles.clone(),
                caveats: c.caveats.clone(),
                stale_or_missing_fields: c.stale_or_missing_fields.clone(),
                packet_ref: c.packet_ref.clone(),
                certification_receipt_ref: c.certification_receipt_ref.clone(),
                release_evidence_ref: c.release_evidence_ref.clone(),
                service_health_ref: c.service_health_ref.clone(),
                docs_badge_ref: c.docs_badge_ref.clone(),
                support_export_ref: c.support_export_ref.clone(),
                ops_adjacent: c.lane.is_ops_adjacent(),
                certified: c.is_certified(),
                downgraded: c.is_downgraded(),
                summary: format!(
                    "{}: freshness {}, coverage {}, drills {}, provenance {}, declared {}, published {} ({}), downgrade {}",
                    c.lane.as_str(),
                    c.evidence_freshness.as_str(),
                    c.profile_coverage.as_str(),
                    c.drill_outcome.as_str(),
                    c.evidence_provenance.as_str(),
                    c.declared_qualification.as_str(),
                    c.published_qualification.as_str(),
                    c.certification_decision.as_str(),
                    c.downgrade_path.as_str()
                ),
            })
            .collect();
        M5ExecutionCertificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            lanes,
            all_lanes_gate_consistent: self.all_lanes_gate_consistent(),
            certified_count: self.certified_lanes().count(),
            narrowed_count: self.narrowed_lanes().count(),
            withdrawn_count: self.withdrawn_lanes().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5ExecutionCertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<CertifiedLane> = self.lanes.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_lanes = BTreeSet::new();
        for row in &self.certifications {
            if !seen_ids.insert(row.lane_id.clone()) {
                violations.push(M5ExecutionCertificationViolation::DuplicateLaneId {
                    lane_id: row.lane_id.clone(),
                });
            }
            if !seen_lanes.insert(row.lane) {
                violations.push(M5ExecutionCertificationViolation::DuplicateLaneRow {
                    lane: row.lane.as_str(),
                });
            }
            if !claimed.contains(&row.lane) {
                violations.push(M5ExecutionCertificationViolation::UnclaimedLaneRow {
                    lane_id: row.lane_id.clone(),
                    lane: row.lane.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed lane must carry its own row, so a lane never inherits a certified
        // claim from an adjacent one.
        for &lane in &self.lanes {
            if !seen_lanes.contains(&lane) {
                violations.push(M5ExecutionCertificationViolation::MissingLaneRow {
                    lane: lane.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5ExecutionCertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ExecutionCertificationViolation>) {
        if self.schema_version != M5_EXECUTION_CERTIFICATION_SCHEMA_VERSION {
            violations.push(
                M5ExecutionCertificationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_EXECUTION_CERTIFICATION_RECORD_KIND {
            violations.push(M5ExecutionCertificationViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ExecutionCertificationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("lanes", self.lanes == CertifiedLane::ALL.to_vec()),
            (
                "qualification_levels",
                self.qualification_levels == QualificationLevel::ALL.to_vec(),
            ),
            (
                "evidence_freshness_states",
                self.evidence_freshness_states == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "profile_coverage_states",
                self.profile_coverage_states == ProfileCoverage::ALL.to_vec(),
            ),
            (
                "drill_outcomes",
                self.drill_outcomes == DrillOutcome::ALL.to_vec(),
            ),
            (
                "evidence_provenance_states",
                self.evidence_provenance_states == EvidenceProvenance::ALL.to_vec(),
            ),
            (
                "downgrade_paths",
                self.downgrade_paths == DowngradePath::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == DowngradeReason::ALL.to_vec(),
            ),
            (
                "certification_decisions",
                self.certification_decisions == CertificationDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5ExecutionCertificationViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &CertificationRow,
        violations: &mut Vec<M5ExecutionCertificationViolation>,
    ) {
        for (field, value) in [
            ("lane_id", &row.lane_id),
            ("owner", &row.owner),
            ("packet_ref", &row.packet_ref),
            ("drill_ref", &row.drill_ref),
            ("evidence_ref", &row.evidence_ref),
            ("certification_receipt_ref", &row.certification_receipt_ref),
            ("release_evidence_ref", &row.release_evidence_ref),
            ("service_health_ref", &row.service_health_ref),
            ("docs_badge_ref", &row.docs_badge_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ExecutionCertificationViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: field,
                });
            }
        }

        // The lane's source packet must be the canonical execution-truth packet it
        // certifies, so a lane never certifies a claim its own source packet does not back.
        if row.packet_ref != row.lane.source_packet() {
            violations.push(M5ExecutionCertificationViolation::SourcePacketMismatch {
                lane_id: row.lane_id.clone(),
                expected: row.lane.source_packet(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    M5ExecutionCertificationViolation::DuplicateDowngradeReason {
                        lane_id: row.lane_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }

        // The published qualification must equal the gate's recomputed ceiling, so a stale,
        // partial, regressed, or unverified lane can never read as certified.
        let effective = row.effective_qualification();
        if row.published_qualification != effective {
            violations.push(M5ExecutionCertificationViolation::OverstatedQualification {
                lane_id: row.lane_id.clone(),
                published: row.published_qualification.as_str(),
                computed: effective.as_str(),
            });
        }

        // The recorded decision must match the gate's recomputed decision.
        let required = row.required_decision();
        if row.certification_decision != required {
            violations.push(M5ExecutionCertificationViolation::DecisionMismatch {
                lane_id: row.lane_id.clone(),
                declared: row.certification_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded downgrade reasons must equal the reasons recomputed from the observed
        // states, so a downgrade can never be asserted or hidden by hand.
        let computed = row.computed_downgrade_reasons();
        if row.downgrade_reasons != computed {
            violations.push(
                M5ExecutionCertificationViolation::DowngradeReasonsMismatch {
                    lane_id: row.lane_id.clone(),
                },
            );
        }

        // A narrowed or withdrawn lane must offer a real downgrade path, list at least one
        // caveat, and name what is stale or missing, so a degraded lane never drops its
        // recovery semantics or hides why it was narrowed.
        if row.certification_decision.is_narrowed() {
            if !row.downgrade_path.is_offered() {
                violations.push(M5ExecutionCertificationViolation::MissingDowngradePath {
                    lane_id: row.lane_id.clone(),
                });
            }
            if row.caveats.is_empty() {
                violations.push(M5ExecutionCertificationViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5ExecutionCertificationViolation::EmptyField {
                    id: row.lane_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // A lane that still backs a publishable claim must name at least one supported
        // deployment-profile or provider-family label.
        if row.published_qualification != QualificationLevel::Withdrawn
            && row.supported_profiles.is_empty()
        {
            violations.push(M5ExecutionCertificationViolation::EmptyField {
                id: row.lane_id.clone(),
                field_name: "supported_profiles",
            });
        }

        // A certified lane must be genuinely clean: a certified ceiling on every input, a
        // certified capability floor, no downgrade reason, no caveat, no stale-or-missing
        // field, and a no-op downgrade path. This is the non-inheritance guardrail — a lane
        // never graduates to a blanket certified claim by inertia.
        if row.is_certified()
            && (row.evidence_freshness.qualification_ceiling() != QualificationLevel::Certified
                || row.profile_coverage.qualification_ceiling() != QualificationLevel::Certified
                || row.drill_outcome.qualification_ceiling() != QualificationLevel::Certified
                || row.evidence_provenance.qualification_ceiling() != QualificationLevel::Certified
                || row.capability_floor() != QualificationLevel::Certified
                || !row.downgrade_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.downgrade_path.is_offered())
        {
            violations.push(M5ExecutionCertificationViolation::CertifiedLaneNotClean {
                lane_id: row.lane_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 execution-certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ExecutionCertificationViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A certification-row id appears more than once.
    DuplicateLaneId {
        /// Duplicate lane id.
        lane_id: String,
    },
    /// A claimed lane carries more than one row.
    DuplicateLaneRow {
        /// Lane token.
        lane: &'static str,
    },
    /// A claimed lane has no row.
    MissingLaneRow {
        /// Lane token.
        lane: &'static str,
    },
    /// A row covers a lane the packet does not claim.
    UnclaimedLaneRow {
        /// Row id.
        lane_id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// A row's source packet is not the canonical packet for its lane.
    SourcePacketMismatch {
        /// Row id.
        lane_id: String,
        /// Expected source-packet path.
        expected: &'static str,
    },
    /// A row lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Row id.
        lane_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A lane publishes a qualification beyond what its evidence supports.
    OverstatedQualification {
        /// Row id.
        lane_id: String,
        /// Published qualification token.
        published: &'static str,
        /// Computed effective qualification token.
        computed: &'static str,
    },
    /// A lane's decision disagrees with its gate decision.
    DecisionMismatch {
        /// Row id.
        lane_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A lane's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Row id.
        lane_id: String,
    },
    /// A narrowed or withdrawn lane offers no downgrade path.
    MissingDowngradePath {
        /// Row id.
        lane_id: String,
    },
    /// A certified lane still carries a downgrade reason or a non-clean state.
    CertifiedLaneNotClean {
        /// Row id.
        lane_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5ExecutionCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateLaneId { lane_id } => {
                write!(f, "duplicate lane id {lane_id}")
            }
            Self::DuplicateLaneRow { lane } => {
                write!(f, "duplicate row for lane {lane}")
            }
            Self::MissingLaneRow { lane } => {
                write!(f, "missing row for claimed lane {lane}")
            }
            Self::UnclaimedLaneRow { lane_id, lane } => {
                write!(f, "row {lane_id} covers unclaimed lane {lane}")
            }
            Self::SourcePacketMismatch { lane_id, expected } => {
                write!(
                    f,
                    "row {lane_id} packet_ref must be the canonical source packet {expected}"
                )
            }
            Self::DuplicateDowngradeReason { lane_id, reason } => {
                write!(f, "row {lane_id} repeats downgrade reason {reason}")
            }
            Self::OverstatedQualification {
                lane_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {lane_id} publishes qualification {published} but the gate computes {computed}"
                )
            }
            Self::DecisionMismatch {
                lane_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {lane_id} records decision {declared} but the gate requires {required}"
                )
            }
            Self::DowngradeReasonsMismatch { lane_id } => {
                write!(f, "row {lane_id} downgrade reasons disagree with the gate")
            }
            Self::MissingDowngradePath { lane_id } => {
                write!(
                    f,
                    "row {lane_id} is narrowed or withdrawn but offers no downgrade path"
                )
            }
            Self::CertifiedLaneNotClean { lane_id } => {
                write!(
                    f,
                    "row {lane_id} is certified but carries a downgrade reason or non-clean state"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5ExecutionCertificationViolation {}

/// Loads the embedded M5 execution-certification matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5ExecutionCertificationMatrix`].
pub fn current_m5_execution_certification_matrix(
) -> Result<M5ExecutionCertificationMatrix, serde_json::Error> {
    serde_json::from_str(M5_EXECUTION_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
