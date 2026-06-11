//! Per-profile Doctor/repair/container certification with a non-inheriting
//! promotion gate that auto-narrows any underqualified M5 profile.
//!
//! Where the frozen maturity matrix certifies each recovery *capability* against
//! each *deployment profile*, this module certifies every marketed **M5 product
//! profile** — notebook, request/API, database, profiler, remote preview, sync,
//! companion, and incident — for combined Doctor, guided-repair, and
//! container/devcontainer maturity. Each [`ProfileCertification`] names the
//! profile's own Doctor/repair/container maturities, the freshness of its
//! qualification packet, its diagnosis-latency state, its engine reachability, and
//! its container-boundary proof, then publishes a qualification that no input can
//! exceed.
//!
//! The model is a release-control gate, not a label store. The qualification a
//! profile may *publish* is derived deterministically: a profile that is stale,
//! diagnosis-latency red, repair-underqualified, engine-blocked, or missing its
//! container/devcontainer boundary proof cannot publish a full certification, and
//! its [`CertificationDecision`] records whether the gate promoted it, narrowed it
//! to provisional or underqualified, or failed promotion and withheld the claim.
//! Because [`ProfileCertification::published_qualification`],
//! [`ProfileCertification::certification_decision`], and the recomputed
//! [`ProfileCertification::narrowing_reasons`] are all validated against the gate,
//! release/public-truth tooling can prove that underqualified profiles narrow
//! automatically before publication and that no profile publishes beyond what its
//! own qualification packet supports.
//!
//! Certification stays profile-specific and freshness-bound. The packet pins the
//! marketed-profile vocabulary and requires exactly one row per claimed profile, so
//! a strong desktop/notebook lane never implies container/devcontainer or
//! blocked-user recovery maturity on an unrelated profile, and a profile never
//! inherits trust from an adjacent certified one. Every profile must carry its own
//! current qualification-packet, latency-corpus, rollback, and compatibility refs.
//!
//! The packet is checked in at
//! `artifacts/doctor/m5/m5-profile-doctor-repair-container-certification.json` and
//! embedded here, so this typed consumer and any CI gate agree on every profile
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, or mount/port/tunnel
//! secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5-profile Doctor/repair/container certification schema version.
pub const M5_PROFILE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_PROFILE_CERTIFICATION_RECORD_KIND: &str =
    "m5_profile_doctor_repair_container_certification";

/// Repo-relative path to the checked-in packet.
pub const M5_PROFILE_CERTIFICATION_PATH: &str =
    "artifacts/doctor/m5/m5-profile-doctor-repair-container-certification.json";

/// Embedded checked-in packet JSON.
pub const M5_PROFILE_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/doctor/m5/m5-profile-doctor-repair-container-certification.json"
));

/// A marketed M5 product profile the certification makes claims about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5Profile {
    /// Notebook kernels and notebook recovery.
    Notebook,
    /// Request/API auth and environment recovery.
    RequestApi,
    /// Database target recovery.
    Database,
    /// Profiler/replay instrumentation recovery.
    Profiler,
    /// Remote-preview route recovery.
    RemotePreview,
    /// Sync/offboarding/device-registry recovery.
    Sync,
    /// Companion handoff recovery.
    Companion,
    /// Incident-packet recovery.
    Incident,
}

impl M5Profile {
    /// Every marketed profile, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Notebook,
        Self::RequestApi,
        Self::Database,
        Self::Profiler,
        Self::RemotePreview,
        Self::Sync,
        Self::Companion,
        Self::Incident,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::RequestApi => "request_api",
            Self::Database => "database",
            Self::Profiler => "profiler",
            Self::RemotePreview => "remote_preview",
            Self::Sync => "sync",
            Self::Companion => "companion",
            Self::Incident => "incident",
        }
    }
}

/// Qualification class of a profile's Doctor/repair/container claim.
///
/// Ordered low-to-high by [`QualificationClass::rank`]: an
/// [`QualificationClass::Unsupported`] profile carries no claim, and a
/// [`QualificationClass::Certified`] profile carries a full, current,
/// evidence-backed claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationClass {
    /// Full, current, evidence-backed certification.
    Certified,
    /// Partial coverage; published as provisional/beta depth only.
    Provisional,
    /// Below the bar for any positive depth claim.
    Underqualified,
    /// Not supported on this profile; carries no claim.
    Unsupported,
}

impl QualificationClass {
    /// Every qualification class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Provisional,
        Self::Underqualified,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Provisional => "provisional",
            Self::Underqualified => "underqualified",
            Self::Unsupported => "unsupported",
        }
    }

    /// Monotonic rank; higher means a stronger claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unsupported => 0,
            Self::Underqualified => 1,
            Self::Provisional => 2,
            Self::Certified => 3,
        }
    }

    /// The weaker (lower-rank) of two qualification classes.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }

    /// Whether this class is at or below the underqualified bar.
    pub const fn is_underqualified_or_worse(self) -> bool {
        self.rank() <= Self::Underqualified.rank()
    }
}

/// Freshness of a profile's qualification packet relative to its freshness SLO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationFreshness {
    /// Proof is current within its freshness SLO.
    Current,
    /// Proof is present but past its freshness SLO.
    Stale,
    /// Proof has expired and no longer backs a live claim.
    Expired,
    /// Proof freshness cannot be established.
    Unknown,
}

impl CertificationFreshness {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Expired, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        }
    }

    /// Whether the proof is current within its freshness SLO.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Highest qualification this freshness alone permits a profile to publish.
    ///
    /// Only `current` proof may publish a full certification; stale or unknown
    /// proof narrows to provisional, and expired proof narrows to underqualified.
    pub const fn qualification_ceiling(self) -> QualificationClass {
        match self {
            Self::Current => QualificationClass::Certified,
            Self::Stale | Self::Unknown => QualificationClass::Provisional,
            Self::Expired => QualificationClass::Underqualified,
        }
    }

    /// Whether this freshness is the headline `stale` narrowing trigger.
    ///
    /// Stale and expired proof both raise the [`NarrowingReason::Stale`] trigger;
    /// `unknown` lowers the ceiling but is treated as a soft state, not a headline
    /// trigger.
    pub const fn is_stale_trigger(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

/// First-actionable diagnosis-latency state for a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosisLatencyState {
    /// Within the first-actionable latency budget.
    Green,
    /// Approaching the budget; depth narrows to provisional.
    Amber,
    /// Past the budget; depth narrows to underqualified.
    Red,
    /// Latency could not be measured.
    Unmeasured,
}

impl DiagnosisLatencyState {
    /// Every latency state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Green, Self::Amber, Self::Red, Self::Unmeasured];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Amber => "amber",
            Self::Red => "red",
            Self::Unmeasured => "unmeasured",
        }
    }

    /// Highest qualification this latency state permits a profile to publish.
    pub const fn qualification_ceiling(self) -> QualificationClass {
        match self {
            Self::Green => QualificationClass::Certified,
            Self::Amber | Self::Unmeasured => QualificationClass::Provisional,
            Self::Red => QualificationClass::Underqualified,
        }
    }

    /// Whether this state is the headline `diagnosis_latency_red` trigger.
    pub const fn is_red_trigger(self) -> bool {
        matches!(self, Self::Red)
    }
}

/// Reachability of the engine/route a profile depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineReachability {
    /// Engine/route is reachable.
    Reachable,
    /// Engine/route is degraded; depth narrows to provisional.
    Degraded,
    /// Engine/route is blocked; depth narrows to underqualified.
    Blocked,
    /// No engine/route is required for this profile.
    NotApplicable,
}

impl EngineReachability {
    /// Every reachability class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Reachable,
        Self::Degraded,
        Self::Blocked,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachable => "reachable",
            Self::Degraded => "degraded",
            Self::Blocked => "blocked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Highest qualification this reachability permits a profile to publish.
    pub const fn qualification_ceiling(self) -> QualificationClass {
        match self {
            Self::Reachable | Self::NotApplicable => QualificationClass::Certified,
            Self::Degraded => QualificationClass::Provisional,
            Self::Blocked => QualificationClass::Underqualified,
        }
    }

    /// Whether this state is the headline `engine_blocked` trigger.
    pub const fn is_blocked_trigger(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Container/devcontainer boundary-proof state for a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryProof {
    /// Mount/port/tunnel boundary scope is verified.
    Verified,
    /// Boundary scope is only partially verified; depth narrows to provisional.
    Partial,
    /// Boundary scope is unverified; depth narrows to underqualified.
    Unverified,
    /// The profile has no container/devcontainer boundary.
    NotApplicable,
}

impl BoundaryProof {
    /// Every boundary-proof class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Verified,
        Self::Partial,
        Self::Unverified,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Partial => "partial",
            Self::Unverified => "unverified",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the profile carries a container/devcontainer boundary at all.
    pub const fn is_applicable(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }

    /// Highest qualification this boundary-proof state permits a profile to
    /// publish.
    pub const fn qualification_ceiling(self) -> QualificationClass {
        match self {
            Self::Verified | Self::NotApplicable => QualificationClass::Certified,
            Self::Partial => QualificationClass::Provisional,
            Self::Unverified => QualificationClass::Underqualified,
        }
    }

    /// Whether this state is the headline `boundary_proof_missing` trigger.
    pub const fn is_missing_trigger(self) -> bool {
        matches!(self, Self::Unverified)
    }
}

/// A headline reason the certification gate narrows a profile.
///
/// These are the canonical release-control triggers: stale evidence, a red
/// diagnosis-latency budget, an underqualified repair lane, a blocked engine, and
/// missing container/devcontainer boundary proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The qualification packet is stale or expired.
    Stale,
    /// The diagnosis-latency budget is red.
    DiagnosisLatencyRed,
    /// The guided-repair lane is underqualified or unsupported.
    RepairUnderqualified,
    /// The engine/route the profile depends on is blocked.
    EngineBlocked,
    /// The container/devcontainer boundary proof is missing.
    BoundaryProofMissing,
}

impl NarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Stale,
        Self::DiagnosisLatencyRed,
        Self::RepairUnderqualified,
        Self::EngineBlocked,
        Self::BoundaryProofMissing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stale => "stale",
            Self::DiagnosisLatencyRed => "diagnosis_latency_red",
            Self::RepairUnderqualified => "repair_underqualified",
            Self::EngineBlocked => "engine_blocked",
            Self::BoundaryProofMissing => "boundary_proof_missing",
        }
    }
}

/// The action the certification gate takes on a profile relative to a full
/// certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDecision {
    /// No narrowing; the profile publishes a full certification.
    Promote,
    /// Narrow the published claim to provisional/beta depth.
    NarrowToProvisional,
    /// Narrow the published claim to underqualified (no positive depth claim).
    NarrowToUnderqualified,
    /// Fail promotion and withhold the certification entirely.
    FailPromotion,
}

impl CertificationDecision {
    /// Every certification decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Promote,
        Self::NarrowToProvisional,
        Self::NarrowToUnderqualified,
        Self::FailPromotion,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promote => "promote",
            Self::NarrowToProvisional => "narrow_to_provisional",
            Self::NarrowToUnderqualified => "narrow_to_underqualified",
            Self::FailPromotion => "fail_promotion",
        }
    }

    /// The certification decision implied by a published qualification.
    pub const fn for_published(qualification: QualificationClass) -> Self {
        match qualification {
            QualificationClass::Certified => Self::Promote,
            QualificationClass::Provisional => Self::NarrowToProvisional,
            QualificationClass::Underqualified => Self::NarrowToUnderqualified,
            QualificationClass::Unsupported => Self::FailPromotion,
        }
    }

    /// Whether the gate narrowed or withheld the profile.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Promote)
    }
}

/// One certification row for a marketed M5 profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileCertification {
    /// Stable profile-certification id.
    pub profile_id: String,
    /// Marketed M5 profile this row certifies.
    pub m5_profile: M5Profile,
    /// Qualification the profile's own packet claims, before the gate.
    pub declared_qualification: QualificationClass,
    /// Qualification actually published after the gate narrows the profile.
    ///
    /// Must equal [`ProfileCertification::effective_qualification`].
    pub published_qualification: QualificationClass,
    /// Project Doctor maturity for this profile.
    pub doctor_maturity: QualificationClass,
    /// Guided-repair maturity for this profile.
    pub repair_maturity: QualificationClass,
    /// Container/devcontainer maturity for this profile.
    pub container_maturity: QualificationClass,
    /// Freshness of the profile's qualification packet.
    pub evidence_freshness: CertificationFreshness,
    /// First-actionable diagnosis-latency state.
    pub diagnosis_latency_state: DiagnosisLatencyState,
    /// Reachability of the engine/route the profile depends on.
    pub engine_reachability: EngineReachability,
    /// Container/devcontainer boundary-proof state.
    pub boundary_proof: BoundaryProof,
    /// Decision the gate takes; must equal the recomputed decision.
    pub certification_decision: CertificationDecision,
    /// Headline narrowing reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Ref to the profile's current Doctor/repair/container qualification packet.
    pub qualification_packet_ref: String,
    /// Ref to the profile's diagnosis-latency proof corpus.
    pub latency_corpus_ref: String,
    /// Ref to the profile's durable rollback path.
    pub rollback_ref: String,
    /// Ref to the profile's compatibility/downgrade story.
    pub compatibility_ref: String,
    /// Ref to the profile's container/devcontainer boundary proof, required when
    /// the profile carries a boundary.
    #[serde(default)]
    pub container_boundary_ref: Option<String>,
    /// Ref binding this row into support exports, Help/About, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl ProfileCertification {
    /// The combined Doctor/repair/container maturity the profile's own lanes
    /// assessed, before environmental narrowing.
    pub fn capability_floor(&self) -> QualificationClass {
        self.declared_qualification
            .min(self.doctor_maturity)
            .min(self.repair_maturity)
            .min(self.container_maturity)
    }

    /// The qualification the gate permits this profile to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the evidence
    /// freshness, diagnosis-latency state, engine reachability, and boundary proof,
    /// so a stale, latency-red, engine-blocked, repair-underqualified, or
    /// boundary-missing profile can never publish a full certification.
    pub fn effective_qualification(&self) -> QualificationClass {
        self.capability_floor()
            .min(self.evidence_freshness.qualification_ceiling())
            .min(self.diagnosis_latency_state.qualification_ceiling())
            .min(self.engine_reachability.qualification_ceiling())
            .min(self.boundary_proof.qualification_ceiling())
    }

    /// The headline narrowing reasons recomputed from the profile's observed
    /// states.
    pub fn computed_narrowing_reasons(&self) -> Vec<NarrowingReason> {
        let mut reasons = Vec::new();
        if self.evidence_freshness.is_stale_trigger() {
            reasons.push(NarrowingReason::Stale);
        }
        if self.diagnosis_latency_state.is_red_trigger() {
            reasons.push(NarrowingReason::DiagnosisLatencyRed);
        }
        if self.repair_maturity.is_underqualified_or_worse() {
            reasons.push(NarrowingReason::RepairUnderqualified);
        }
        if self.engine_reachability.is_blocked_trigger() {
            reasons.push(NarrowingReason::EngineBlocked);
        }
        if self.boundary_proof.is_missing_trigger() {
            reasons.push(NarrowingReason::BoundaryProofMissing);
        }
        reasons
    }

    /// The decision the gate must record for this profile.
    pub fn required_decision(&self) -> CertificationDecision {
        CertificationDecision::for_published(self.effective_qualification())
    }

    /// Whether the profile may publish a full certification.
    pub fn is_promotable(&self) -> bool {
        self.effective_qualification() == QualificationClass::Certified
    }

    /// Whether the profile carries its own non-empty qualification packet,
    /// latency corpus, rollback, and compatibility refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.qualification_packet_ref.trim().is_empty()
            && !self.latency_corpus_ref.trim().is_empty()
            && !self.rollback_ref.trim().is_empty()
            && !self.compatibility_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether a profile with a container/devcontainer boundary carries a
    /// non-empty boundary-proof ref.
    pub fn boundary_ref_consistent(&self) -> bool {
        if self.boundary_proof.is_applicable() {
            self.container_boundary_ref
                .as_deref()
                .is_some_and(|r| !r.trim().is_empty())
        } else {
            true
        }
    }

    /// Whether the stored published qualification, decision, and narrowing reasons
    /// all agree with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_qualification == self.effective_qualification()
            && self.certification_decision == self.required_decision()
            && self.narrowing_reasons == self.computed_narrowing_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ProfileCertificationSummary {
    /// Total profiles.
    pub total_profiles: usize,
    /// Number of marketed profiles claimed.
    pub profile_count: usize,
    /// Profiles published as certified.
    pub certified_profiles: usize,
    /// Profiles published as provisional.
    pub provisional_profiles: usize,
    /// Profiles published as underqualified.
    pub underqualified_profiles: usize,
    /// Profiles published as unsupported.
    pub unsupported_profiles: usize,
    /// Profiles that may publish a full certification.
    pub promotable_profiles: usize,
    /// Profiles the gate narrowed or withheld in any way.
    pub narrowed_profiles: usize,
    /// Profiles the gate failed promotion on.
    pub failed_promotion_profiles: usize,
    /// Profiles with current proof freshness.
    pub current_freshness_profiles: usize,
    /// Profiles with a red diagnosis-latency state.
    pub latency_red_profiles: usize,
    /// Profiles with a blocked engine.
    pub engine_blocked_profiles: usize,
    /// Profiles with an unverified container/devcontainer boundary.
    pub boundary_unverified_profiles: usize,
    /// Profiles carrying at least one narrowing reason.
    pub profiles_with_narrowing_reasons: usize,
}

/// A redaction-safe export row projected from a profile certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProfileCertificationExportRow {
    /// Profile-certification id.
    pub profile_id: String,
    /// Marketed-profile token.
    pub m5_profile: String,
    /// Declared qualification token.
    pub declared_qualification: String,
    /// Published qualification token.
    pub published_qualification: String,
    /// Doctor maturity token.
    pub doctor_maturity: String,
    /// Repair maturity token.
    pub repair_maturity: String,
    /// Container maturity token.
    pub container_maturity: String,
    /// Evidence freshness token.
    pub evidence_freshness: String,
    /// Diagnosis-latency state token.
    pub diagnosis_latency_state: String,
    /// Engine reachability token.
    pub engine_reachability: String,
    /// Boundary-proof token.
    pub boundary_proof: String,
    /// Certification decision token.
    pub certification_decision: String,
    /// Narrowing reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Qualification-packet ref.
    pub qualification_packet_ref: String,
    /// Whether the profile publishes a full certification.
    pub publication_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProfileCertificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub profiles: Vec<M5ProfileCertificationExportRow>,
    /// Whether every profile's published claim and decision agree with the gate.
    pub all_profiles_gate_consistent: bool,
    /// Profiles that may publish a full certification.
    pub promotable_count: usize,
    /// Profiles the gate narrowed or withheld.
    pub narrowed_count: usize,
    /// Profiles the gate failed promotion on.
    pub failed_promotion_count: usize,
}

/// The typed M5-profile Doctor/repair/container certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ProfileCertification {
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
    /// Marketed profiles the packet claims; one row per profile.
    pub m5_profiles: Vec<M5Profile>,
    /// Closed qualification-class vocabulary.
    pub qualification_classes: Vec<QualificationClass>,
    /// Closed certification-freshness vocabulary.
    pub certification_freshness_classes: Vec<CertificationFreshness>,
    /// Closed diagnosis-latency-state vocabulary.
    pub diagnosis_latency_states: Vec<DiagnosisLatencyState>,
    /// Closed engine-reachability vocabulary.
    pub engine_reachability_classes: Vec<EngineReachability>,
    /// Closed boundary-proof vocabulary.
    pub boundary_proof_classes: Vec<BoundaryProof>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed certification-decision vocabulary.
    pub certification_decisions: Vec<CertificationDecision>,
    /// Certification rows, one per marketed profile.
    #[serde(default)]
    pub profiles: Vec<ProfileCertification>,
    /// Summary counts.
    pub summary: M5ProfileCertificationSummary,
}

impl M5ProfileCertification {
    /// Returns the row for a marketed profile.
    pub fn profile(&self, profile: M5Profile) -> Option<&ProfileCertification> {
        self.profiles.iter().find(|p| p.m5_profile == profile)
    }

    /// Profiles that may publish a full certification.
    pub fn promotable_profiles(&self) -> impl Iterator<Item = &ProfileCertification> {
        self.profiles.iter().filter(|p| p.is_promotable())
    }

    /// Profiles the gate narrowed or withheld in any way.
    pub fn narrowed_profiles(&self) -> impl Iterator<Item = &ProfileCertification> {
        self.profiles
            .iter()
            .filter(|p| p.required_decision().is_narrowed())
    }

    /// Profiles the gate failed promotion on.
    pub fn failed_promotion_profiles(&self) -> impl Iterator<Item = &ProfileCertification> {
        self.profiles
            .iter()
            .filter(|p| p.required_decision() == CertificationDecision::FailPromotion)
    }

    /// Whether every profile's stored published claim, decision, and reasons agree
    /// with the recomputed gate decision.
    pub fn all_profiles_gate_consistent(&self) -> bool {
        self.profiles.iter().all(|p| p.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5ProfileCertificationSummary {
        let count_published = |qualification: QualificationClass| {
            self.profiles
                .iter()
                .filter(|p| p.published_qualification == qualification)
                .count()
        };
        M5ProfileCertificationSummary {
            total_profiles: self.profiles.len(),
            profile_count: self.m5_profiles.len(),
            certified_profiles: count_published(QualificationClass::Certified),
            provisional_profiles: count_published(QualificationClass::Provisional),
            underqualified_profiles: count_published(QualificationClass::Underqualified),
            unsupported_profiles: count_published(QualificationClass::Unsupported),
            promotable_profiles: self.promotable_profiles().count(),
            narrowed_profiles: self.narrowed_profiles().count(),
            failed_promotion_profiles: self.failed_promotion_profiles().count(),
            current_freshness_profiles: self
                .profiles
                .iter()
                .filter(|p| p.evidence_freshness.is_current())
                .count(),
            latency_red_profiles: self
                .profiles
                .iter()
                .filter(|p| p.diagnosis_latency_state.is_red_trigger())
                .count(),
            engine_blocked_profiles: self
                .profiles
                .iter()
                .filter(|p| p.engine_reachability.is_blocked_trigger())
                .count(),
            boundary_unverified_profiles: self
                .profiles
                .iter()
                .filter(|p| p.boundary_proof.is_missing_trigger())
                .count(),
            profiles_with_narrowing_reasons: self
                .profiles
                .iter()
                .filter(|p| !p.narrowing_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — Help/About,
    /// docs/migration, support exports, and release/public-truth packets — render
    /// instead of restating M5-profile recovery/container status text by hand.
    pub fn export_projection(&self) -> M5ProfileCertificationExportProjection {
        let profiles = self
            .profiles
            .iter()
            .map(|p| M5ProfileCertificationExportRow {
                profile_id: p.profile_id.clone(),
                m5_profile: p.m5_profile.as_str().to_owned(),
                declared_qualification: p.declared_qualification.as_str().to_owned(),
                published_qualification: p.published_qualification.as_str().to_owned(),
                doctor_maturity: p.doctor_maturity.as_str().to_owned(),
                repair_maturity: p.repair_maturity.as_str().to_owned(),
                container_maturity: p.container_maturity.as_str().to_owned(),
                evidence_freshness: p.evidence_freshness.as_str().to_owned(),
                diagnosis_latency_state: p.diagnosis_latency_state.as_str().to_owned(),
                engine_reachability: p.engine_reachability.as_str().to_owned(),
                boundary_proof: p.boundary_proof.as_str().to_owned(),
                certification_decision: p.certification_decision.as_str().to_owned(),
                narrowing_reasons: p
                    .narrowing_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                qualification_packet_ref: p.qualification_packet_ref.clone(),
                publication_ready: p.is_promotable(),
                summary: format!(
                    "{}: declared {}, published {} ({}), freshness {}, latency {}, engine {}, boundary {}",
                    p.m5_profile.as_str(),
                    p.declared_qualification.as_str(),
                    p.published_qualification.as_str(),
                    p.certification_decision.as_str(),
                    p.evidence_freshness.as_str(),
                    p.diagnosis_latency_state.as_str(),
                    p.engine_reachability.as_str(),
                    p.boundary_proof.as_str()
                ),
            })
            .collect();
        M5ProfileCertificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            profiles,
            all_profiles_gate_consistent: self.all_profiles_gate_consistent(),
            promotable_count: self.promotable_profiles().count(),
            narrowed_count: self.narrowed_profiles().count(),
            failed_promotion_count: self.failed_promotion_profiles().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5ProfileCertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<M5Profile> = self.m5_profiles.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_profiles = BTreeSet::new();
        for row in &self.profiles {
            if !seen_ids.insert(row.profile_id.clone()) {
                violations.push(M5ProfileCertificationViolation::DuplicateProfileId {
                    profile_id: row.profile_id.clone(),
                });
            }
            if !seen_profiles.insert(row.m5_profile) {
                violations.push(M5ProfileCertificationViolation::DuplicateProfileRow {
                    profile: row.m5_profile.as_str(),
                });
            }
            if !claimed.contains(&row.m5_profile) {
                violations.push(M5ProfileCertificationViolation::UnclaimedProfileRow {
                    profile_id: row.profile_id.clone(),
                    profile: row.m5_profile.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed profile must carry its own row, so a profile never
        // inherits trust from an adjacent certified one.
        for &profile in &self.m5_profiles {
            if !seen_profiles.contains(&profile) {
                violations.push(M5ProfileCertificationViolation::MissingProfileRow {
                    profile: profile.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5ProfileCertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ProfileCertificationViolation>) {
        if self.schema_version != M5_PROFILE_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5ProfileCertificationViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_PROFILE_CERTIFICATION_RECORD_KIND {
            violations.push(M5ProfileCertificationViolation::UnsupportedRecordKind {
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
                violations.push(M5ProfileCertificationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("m5_profiles", self.m5_profiles == M5Profile::ALL.to_vec()),
            (
                "qualification_classes",
                self.qualification_classes == QualificationClass::ALL.to_vec(),
            ),
            (
                "certification_freshness_classes",
                self.certification_freshness_classes == CertificationFreshness::ALL.to_vec(),
            ),
            (
                "diagnosis_latency_states",
                self.diagnosis_latency_states == DiagnosisLatencyState::ALL.to_vec(),
            ),
            (
                "engine_reachability_classes",
                self.engine_reachability_classes == EngineReachability::ALL.to_vec(),
            ),
            (
                "boundary_proof_classes",
                self.boundary_proof_classes == BoundaryProof::ALL.to_vec(),
            ),
            (
                "narrowing_reasons",
                self.narrowing_reasons == NarrowingReason::ALL.to_vec(),
            ),
            (
                "certification_decisions",
                self.certification_decisions == CertificationDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5ProfileCertificationViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &ProfileCertification,
        violations: &mut Vec<M5ProfileCertificationViolation>,
    ) {
        for (field, value) in [
            ("profile_id", &row.profile_id),
            ("qualification_packet_ref", &row.qualification_packet_ref),
            ("latency_corpus_ref", &row.latency_corpus_ref),
            ("rollback_ref", &row.rollback_ref),
            ("compatibility_ref", &row.compatibility_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ProfileCertificationViolation::EmptyField {
                    id: row.profile_id.clone(),
                    field_name: field,
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.narrowing_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5ProfileCertificationViolation::DuplicateNarrowingReason {
                    profile_id: row.profile_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // A profile that carries a container/devcontainer boundary must reference
        // its boundary proof, so "missing container/devcontainer boundary proof"
        // can never hide behind an absent ref.
        if !row.boundary_ref_consistent() {
            violations.push(M5ProfileCertificationViolation::MissingBoundaryProofRef {
                profile_id: row.profile_id.clone(),
            });
        }

        // The published qualification must equal the gate's recomputed decision,
        // so a profile can never publish beyond what its freshness, latency,
        // engine, repair, and boundary states support.
        let effective = row.effective_qualification();
        if row.published_qualification != effective {
            violations.push(
                M5ProfileCertificationViolation::OverstatedPublishedQualification {
                    profile_id: row.profile_id.clone(),
                    published: row.published_qualification.as_str(),
                    computed: effective.as_str(),
                },
            );
        }

        // The recorded decision must match the published qualification, so release
        // tooling proves underqualified profiles narrow automatically.
        let required = row.required_decision();
        if row.certification_decision != required {
            violations.push(M5ProfileCertificationViolation::DecisionMismatch {
                profile_id: row.profile_id.clone(),
                declared: row.certification_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded narrowing reasons must equal the reasons recomputed from
        // the observed states, so a narrowing can never be asserted or hidden by
        // hand.
        let computed = row.computed_narrowing_reasons();
        if row.narrowing_reasons != computed {
            violations.push(M5ProfileCertificationViolation::NarrowingReasonsMismatch {
                profile_id: row.profile_id.clone(),
            });
        }

        // A promotable profile must be genuinely clean: current freshness, green
        // latency, a reachable/not-applicable engine, verified/not-applicable
        // boundary, all-certified capabilities, and no narrowing reason. This is
        // the non-inheritance guardrail.
        if row.is_promotable()
            && (!row.evidence_freshness.is_current()
                || row.diagnosis_latency_state != DiagnosisLatencyState::Green
                || row.engine_reachability.qualification_ceiling() != QualificationClass::Certified
                || row.boundary_proof.qualification_ceiling() != QualificationClass::Certified
                || row.capability_floor() != QualificationClass::Certified
                || !row.narrowing_reasons.is_empty())
        {
            violations.push(M5ProfileCertificationViolation::PromotedProfileNotClean {
                profile_id: row.profile_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5-profile certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ProfileCertificationViolation {
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
    /// A profile-certification id appears more than once.
    DuplicateProfileId {
        /// Duplicate profile id.
        profile_id: String,
    },
    /// A marketed profile carries more than one row.
    DuplicateProfileRow {
        /// Profile token.
        profile: &'static str,
    },
    /// A claimed marketed profile has no row.
    MissingProfileRow {
        /// Profile token.
        profile: &'static str,
    },
    /// A row covers a profile the packet does not claim.
    UnclaimedProfileRow {
        /// Row id.
        profile_id: String,
        /// Profile token.
        profile: &'static str,
    },
    /// A row lists a narrowing reason more than once.
    DuplicateNarrowingReason {
        /// Row id.
        profile_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A profile with a container/devcontainer boundary lacks a boundary-proof ref.
    MissingBoundaryProofRef {
        /// Row id.
        profile_id: String,
    },
    /// A profile publishes a qualification beyond what its proof supports.
    OverstatedPublishedQualification {
        /// Row id.
        profile_id: String,
        /// Published-qualification token.
        published: &'static str,
        /// Computed effective-qualification token.
        computed: &'static str,
    },
    /// A profile's decision disagrees with its published qualification.
    DecisionMismatch {
        /// Row id.
        profile_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A profile's narrowing reasons disagree with the recomputed reasons.
    NarrowingReasonsMismatch {
        /// Row id.
        profile_id: String,
    },
    /// A promotable profile still carries a narrowing reason or a non-clean state.
    PromotedProfileNotClean {
        /// Row id.
        profile_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5ProfileCertificationViolation {
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
            Self::DuplicateProfileId { profile_id } => {
                write!(f, "duplicate profile id {profile_id}")
            }
            Self::DuplicateProfileRow { profile } => {
                write!(f, "duplicate row for profile {profile}")
            }
            Self::MissingProfileRow { profile } => {
                write!(f, "missing row for claimed profile {profile}")
            }
            Self::UnclaimedProfileRow {
                profile_id,
                profile,
            } => {
                write!(f, "row {profile_id} covers unclaimed profile {profile}")
            }
            Self::DuplicateNarrowingReason { profile_id, reason } => {
                write!(f, "row {profile_id} repeats narrowing reason {reason}")
            }
            Self::MissingBoundaryProofRef { profile_id } => {
                write!(
                    f,
                    "row {profile_id} has a container boundary but no boundary-proof ref"
                )
            }
            Self::OverstatedPublishedQualification {
                profile_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {profile_id} publishes qualification {published} but the gate computes {computed}"
                )
            }
            Self::DecisionMismatch {
                profile_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {profile_id} records decision {declared} but the gate requires {required}"
                )
            }
            Self::NarrowingReasonsMismatch { profile_id } => {
                write!(
                    f,
                    "row {profile_id} narrowing reasons disagree with the gate"
                )
            }
            Self::PromotedProfileNotClean { profile_id } => {
                write!(
                    f,
                    "row {profile_id} is promotable but carries a narrowing reason or non-clean state"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5ProfileCertificationViolation {}

/// Loads the embedded M5-profile certification packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5ProfileCertification`].
pub fn current_m5_profile_certification() -> Result<M5ProfileCertification, serde_json::Error> {
    serde_json::from_str(M5_PROFILE_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
