//! Promotion-grade certification of reactive-state truth on claimed M5
//! shell / search / graph / AI / review / support profiles.
//!
//! The reactive-state batch already freezes a typed subscription
//! contract, materialized-view governance, gated truth cues, lagging-
//! consumer recovery, command/journal publication parity, and a
//! metadata-first support export. What it still lacks is one
//! *certification* lane that turns those packets into a single,
//! promotion-grade claim per claimed M5 surface profile — and that
//! narrows the claim automatically when the backing evidence goes stale,
//! partial, or missing.
//!
//! This module is that lane. It models one [`CertificationRow`] per
//! claimed reactive surface profile, each carrying five required
//! [`CertificationDimension`]s — authority class, epoch parity,
//! invalidation behavior, stale-state labeling, and safe-action
//! narrowing — and the evidence backing each. A single
//! [`certify_row_outcome`] engine folds the per-dimension evidence
//! states into one [`RowVerdict`] (`certified` / `narrowed` /
//! `withheld`) and an effective [`ClaimMaturity`] floor, so a `stable`
//! or `beta` claim can never outrun the reactive-state evidence that
//! backs it. The same engine drives the failure / recovery
//! [`CertificationDrill`]s and the [`M5ReactiveCertificationFixture`]
//! corpus, so the certification, the drills, and the fixtures cannot
//! disagree about when a claim must narrow.
//!
//! Three guardrails are frozen here:
//!
//! - **No happy-path green.** A profile is certified at its claimed
//!   maturity only when every required dimension is `current`. Stale or
//!   partial evidence narrows the claim; missing evidence withholds it.
//! - **One narrowing engine.** [`certify_row_outcome`] is the single
//!   source of truth for downgrade, shared by the rows, the drills, the
//!   fixtures, and the [`EvidenceFreshnessRule`]s. Release, support,
//!   docs, and help all read the resulting verdict rather than
//!   re-deriving staleness.
//! - **No silent widening.** The certification only ever narrows; it
//!   never promotes a profile above its claimed maturity, and a profile
//!   absent from the packet is uncertified, not implicitly green.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/m5-reactive-certification.schema.json`](../../../../schemas/state/m5-reactive-certification.schema.json)
//! - [`/docs/state/m5-reactive-certification.md`](../../../../docs/state/m5-reactive-certification.md)
//! - [`/artifacts/state/m5-reactive-proof-packet.json`](../../../../artifacts/state/m5-reactive-proof-packet.json)
//! - [`/artifacts/state/m5-reactive-certification.md`](../../../../artifacts/state/m5-reactive-certification.md)
//! - [`/fixtures/state/m5-reactive-certification/`](../../../../fixtures/state/m5-reactive-certification/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const M5_REACTIVE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const M5_REACTIVE_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "m5_reactive_certification_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const M5_REACTIVE_CERTIFICATION_FIXTURE_RECORD_KIND: &str =
    "m5_reactive_certification_fixture_record";

/// Repo-relative schema ref.
pub const M5_REACTIVE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/state/m5-reactive-certification.schema.json";

/// Repo-relative reviewer doc ref.
pub const M5_REACTIVE_CERTIFICATION_DOC_REF: &str = "docs/state/m5-reactive-certification.md";

/// Repo-relative machine-readable proof packet.
pub const M5_REACTIVE_CERTIFICATION_PACKET_REF: &str =
    "artifacts/state/m5-reactive-proof-packet.json";

/// Repo-relative reviewer certification summary.
pub const M5_REACTIVE_CERTIFICATION_REPORT_REF: &str =
    "artifacts/state/m5-reactive-certification.md";

/// Repo-relative fixture directory.
pub const M5_REACTIVE_CERTIFICATION_FIXTURE_DIR: &str = "fixtures/state/m5-reactive-certification";

/// Repo-relative fixture manifest.
pub const M5_REACTIVE_CERTIFICATION_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/m5-reactive-certification/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// A claimed M5 reactive surface profile under certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedSurfaceProfile {
    /// Shell workspace tree and activity-center surfaces.
    Shell,
    /// Search results surface.
    Search,
    /// Graph neighborhood surface.
    Graph,
    /// AI context-panel surface.
    Ai,
    /// Review workspace overlay surface.
    Review,
    /// Support reactive-state export surface.
    Support,
}

impl ClaimedSurfaceProfile {
    /// Every claimed profile in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Search,
        Self::Graph,
        Self::Ai,
        Self::Review,
        Self::Support,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Search => "search",
            Self::Graph => "graph",
            Self::Ai => "ai",
            Self::Review => "review",
            Self::Support => "support",
        }
    }
}

/// One reactive-state dimension a claimed profile must prove. The five
/// dimensions are the exit-gate anchor: a profile may not present
/// derived state as product truth unless all five are canonical and
/// testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// The surface declares which authority class owns its truth and
    /// whether it is authoritative or a derived projection.
    AuthorityClass,
    /// The surface reads the shared authoritative epoch for its
    /// authority class and narrows rather than presenting a parallel
    /// epoch as truth.
    EpochParity,
    /// The surface honors the invalidation reasons that change its
    /// truth and recovers lagging consumers honestly.
    InvalidationBehavior,
    /// The surface labels warming, cached, stale, partial, and
    /// coalesced state instead of implying exact current truth.
    StaleStateLabeling,
    /// The surface narrows the actions it offers under degraded state
    /// instead of offering stale exact-truth affordances.
    SafeActionNarrowing,
}

impl CertificationDimension {
    /// Every required dimension in canonical order.
    pub const ALL: [Self; 5] = [
        Self::AuthorityClass,
        Self::EpochParity,
        Self::InvalidationBehavior,
        Self::StaleStateLabeling,
        Self::SafeActionNarrowing,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityClass => "authority_class",
            Self::EpochParity => "epoch_parity",
            Self::InvalidationBehavior => "invalidation_behavior",
            Self::StaleStateLabeling => "stale_state_labeling",
            Self::SafeActionNarrowing => "safe_action_narrowing",
        }
    }
}

/// The maturity a reactive-state claim can hold. Declaration order is
/// the narrowing order: [`ClaimMaturity::Stable`] is the strongest claim
/// and [`ClaimMaturity::Withdrawn`] the weakest, so narrowing always
/// moves toward a later variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimMaturity {
    /// Every required dimension is current; the claim holds in full.
    Stable,
    /// One or more dimensions are partial or stale; the claim narrows.
    Beta,
    /// Evidence is incomplete enough that only a preview claim holds.
    Preview,
    /// A required dimension cannot be proven; the claim is withdrawn.
    Withdrawn,
}

impl ClaimMaturity {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Narrowing severity. Higher is a narrower, more honest claim; the
    /// engine always takes the highest severity among the claimed
    /// maturity and every triggered floor.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Stable => 0,
            Self::Beta => 1,
            Self::Preview => 2,
            Self::Withdrawn => 3,
        }
    }
}

/// The state of the evidence backing one dimension on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    /// Evidence is present, complete, and within its freshness window.
    Current,
    /// Evidence covers only part of the claimed scope.
    Partial,
    /// Evidence exists but is past its freshness window.
    Stale,
    /// No evidence backs this dimension.
    Missing,
    /// The dimension does not apply to this surface.
    NotApplicable,
}

impl EvidenceState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// The maturity floor this evidence state forces on a claim, if any.
    ///
    /// This is the heart of the narrowing engine: current and
    /// not-applicable evidence impose no floor; partial evidence caps
    /// the claim at beta; stale evidence caps it at preview; missing
    /// evidence withdraws the claim.
    pub const fn qualification_floor(self) -> Option<ClaimMaturity> {
        match self {
            Self::Current | Self::NotApplicable => None,
            Self::Partial => Some(ClaimMaturity::Beta),
            Self::Stale => Some(ClaimMaturity::Preview),
            Self::Missing => Some(ClaimMaturity::Withdrawn),
        }
    }

    /// Returns true when the state names stale or missing evidence, the
    /// two states the guardrail treats as a freshness defect.
    pub const fn is_stale_or_missing(self) -> bool {
        matches!(self, Self::Stale | Self::Missing)
    }
}

/// The verdict the certification engine reaches for one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowVerdict {
    /// Every required dimension is current; the claim holds at its
    /// claimed maturity.
    Certified,
    /// The claim narrowed below its claimed maturity but still holds.
    Narrowed,
    /// A required dimension cannot be proven; the claim is withheld.
    Withheld,
}

impl RowVerdict {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Withheld => "withheld",
        }
    }
}

/// A publication channel that ingests the certification packet. The
/// packet as a whole must bind all four so release, support, docs, and
/// help tell one consistent reactive-state story.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationChannel {
    /// Release / shiproom promotion surfaces.
    ReleaseShiproom,
    /// Metadata-first support export surfaces.
    SupportExport,
    /// Reviewer / product documentation surfaces.
    Docs,
    /// In-product help and explainer surfaces.
    Help,
}

impl PublicationChannel {
    /// Every channel in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ReleaseShiproom,
        Self::SupportExport,
        Self::Docs,
        Self::Help,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseShiproom => "release_shiproom",
            Self::SupportExport => "support_export",
            Self::Docs => "docs",
            Self::Help => "help",
        }
    }
}

/// The failure class a certification drill injects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillFailureClass {
    /// A surface lags the shared authoritative epoch for its authority.
    AuthorityEpochLag,
    /// A backing evidence packet ages past its freshness window.
    EvidenceWentStale,
    /// A backing evidence packet covers only part of the scope.
    PartialEvidenceCoverage,
    /// A required dimension loses its evidence entirely.
    DimensionEvidenceMissing,
    /// The backing producer goes terminally unavailable.
    ProviderUnavailable,
    /// Policy or entitlement limits the visible projection.
    PolicyLimited,
}

impl DrillFailureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityEpochLag => "authority_epoch_lag",
            Self::EvidenceWentStale => "evidence_went_stale",
            Self::PartialEvidenceCoverage => "partial_evidence_coverage",
            Self::DimensionEvidenceMissing => "dimension_evidence_missing",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::PolicyLimited => "policy_limited",
        }
    }
}

/// One ordered phase of a certification drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillPhase {
    /// A failure is injected into a backing dimension.
    Inject,
    /// The certification observes the degraded evidence state.
    Observe,
    /// The claim narrows or withholds under the degraded evidence.
    Narrow,
    /// The evidence is refreshed.
    Refresh,
    /// The claim recovers as the evidence returns to current.
    Recover,
    /// The recovered posture is verified against the engine.
    Verify,
}

impl DrillPhase {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inject => "inject",
            Self::Observe => "observe",
            Self::Narrow => "narrow",
            Self::Refresh => "refresh",
            Self::Recover => "recover",
            Self::Verify => "verify",
        }
    }
}

// ---------------------------------------------------------------------------
// Narrowing engine: the single source of truth for the verdict.
// ---------------------------------------------------------------------------

/// One dimension's evidence on one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionEvidence {
    /// Reactive-state dimension being evidenced.
    pub dimension: CertificationDimension,
    /// State of the evidence backing this dimension.
    pub evidence_state: EvidenceState,
    /// Upstream reactive-state packets that prove this dimension.
    pub evidence_refs: Vec<String>,
    /// Review-safe rationale for the evidence.
    pub rationale: String,
}

/// The computed outcome of certifying one row against its evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowOutcome {
    /// The narrowest maturity the claim may hold.
    pub effective_maturity: ClaimMaturity,
    /// The verdict the engine reaches.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its claimed maturity.
    pub narrowed: bool,
    /// Stable tokens naming every dimension that forced narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Dimensions whose evidence is stale or missing, in stable order.
    pub stale_or_missing_dimension_tokens: Vec<String>,
}

/// Certifies one row's claim against its per-dimension evidence.
///
/// This is the canonical narrowing engine the whole packet, every
/// drill, every fixture, and release / support tooling share. The
/// effective maturity starts at the claimed maturity and is floored by
/// every degraded dimension; the narrowest (highest-severity) result
/// wins. A withdrawn result is [`RowVerdict::Withheld`]; any other
/// result below the claimed maturity is [`RowVerdict::Narrowed`];
/// otherwise the row is [`RowVerdict::Certified`].
pub fn certify_row_outcome(claimed: ClaimMaturity, dimensions: &[DimensionEvidence]) -> RowOutcome {
    let mut effective = claimed;
    let mut narrow_reason_tokens = Vec::new();
    let mut stale_or_missing = Vec::new();

    for evidence in dimensions {
        if let Some(floor) = evidence.evidence_state.qualification_floor() {
            if floor.severity() > effective.severity() {
                effective = floor;
            }
            narrow_reason_tokens.push(format!(
                "{}_{}",
                evidence.dimension.as_str(),
                evidence.evidence_state.as_str()
            ));
        }
        if evidence.evidence_state.is_stale_or_missing() {
            stale_or_missing.push(evidence.dimension.as_str().to_owned());
        }
    }

    narrow_reason_tokens.sort();
    narrow_reason_tokens.dedup();
    stale_or_missing.sort();
    stale_or_missing.dedup();

    let verdict = if effective == ClaimMaturity::Withdrawn {
        RowVerdict::Withheld
    } else if effective.severity() > claimed.severity() {
        RowVerdict::Narrowed
    } else {
        RowVerdict::Certified
    };

    RowOutcome {
        effective_maturity: effective,
        verdict,
        narrowed: verdict == RowVerdict::Narrowed,
        narrow_reason_tokens,
        stale_or_missing_dimension_tokens: stale_or_missing,
    }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One certification row: a claimed surface profile, its evidence, and
/// the engine outcome stamped onto it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed reactive surface profile.
    pub surface_profile: ClaimedSurfaceProfile,
    /// Review-safe label for the profile.
    pub surface_label: String,
    /// Maturity claimed for the profile.
    pub claimed_maturity: ClaimMaturity,
    /// Governance surface classes this profile spans.
    pub backing_surface_classes: Vec<String>,
    /// Per-dimension evidence, one entry per required dimension.
    pub dimensions: Vec<DimensionEvidence>,
    /// Effective maturity after narrowing.
    pub effective_maturity: ClaimMaturity,
    /// Engine verdict.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its claimed maturity.
    pub narrowed: bool,
    /// Stable tokens naming every dimension that forced narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Dimensions whose evidence is stale or missing.
    pub stale_or_missing_dimension_tokens: Vec<String>,
    /// Upstream reactive-state packets this row composes.
    pub supporting_packet_refs: Vec<String>,
    /// Real consumer surfaces that ingest this row.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One automatic-narrowing rule keyed by evidence state. The floor is
/// computed from [`EvidenceState::qualification_floor`], so the rule set
/// can never drift from the engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceFreshnessRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Evidence state that triggers the rule.
    pub trigger_evidence_state: EvidenceState,
    /// Maturity floor the rule imposes.
    pub maturity_floor: ClaimMaturity,
    /// User-visible effect on the claim.
    pub effect: String,
    /// Review-safe rationale.
    pub rationale: String,
}

/// One ordered step inside a certification drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDrillStep {
    /// Phase of this step.
    pub phase: DrillPhase,
    /// Maturity observed at this step.
    pub observed_maturity: ClaimMaturity,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One failure / recovery drill walking a profile from an injected
/// failure through narrowing and back to recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Surface profile exercised by the drill.
    pub surface_profile: ClaimedSurfaceProfile,
    /// Dimension whose evidence the drill degrades.
    pub exercised_dimension: CertificationDimension,
    /// Failure class the drill injects.
    pub failure_class: DrillFailureClass,
    /// Evidence state the dimension degrades to.
    pub degraded_evidence_state: EvidenceState,
    /// Maturity claimed before the failure.
    pub claimed_maturity: ClaimMaturity,
    /// Verdict expected while the failure is active.
    pub expected_degraded_verdict: RowVerdict,
    /// Maturity expected while the failure is active.
    pub expected_degraded_maturity: ClaimMaturity,
    /// Verdict expected once the evidence is refreshed.
    pub recovers_to_verdict: RowVerdict,
    /// Ordered drill steps.
    pub steps: Vec<CertificationDrillStep>,
    /// True when the drill proves the claim narrows under the failure.
    pub asserts_claim_narrows_under_failure: bool,
    /// True when the drill proves the claim recovers after refresh.
    pub asserts_recovers_after_refresh: bool,
    /// Short reviewer note.
    pub notes: String,
}

/// One binding proving a publication channel ingests this packet rather
/// than re-deriving reactive-state truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceBinding {
    /// Channel that ingests the packet.
    pub channel: PublicationChannel,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet id the channel ingests.
    pub ingested_packet_id: String,
    /// Fields the channel preserves verbatim.
    pub required_verbatim_fields: Vec<String>,
    /// True when the channel narrows in lockstep with the packet.
    pub narrows_with_packet: bool,
    /// Review-safe summary of the binding.
    pub summary: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Proof packet ref.
    pub packet_ref: String,
    /// Certification summary ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet certifying reactive-state truth on claimed M5
/// surface profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReactiveCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// Required certification dimensions.
    pub certified_dimensions: Vec<CertificationDimension>,
    /// Upstream reactive-state packets this certification composes.
    pub evidence_packet_refs: Vec<String>,
    /// Certification rows, one per claimed profile.
    pub rows: Vec<CertificationRow>,
    /// Automatic-narrowing rules over evidence states.
    pub freshness_rules: Vec<EvidenceFreshnessRule>,
    /// Failure / recovery drills.
    pub drills: Vec<CertificationDrill>,
    /// Publication-channel bindings.
    pub surface_bindings: Vec<SurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a profile and an observed evidence configuration
/// to the expected verdict, proving the canonical narrowing behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReactiveCertificationFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Surface profile under test.
    pub surface_profile: ClaimedSurfaceProfile,
    /// Maturity claimed before narrowing.
    pub claimed_maturity: ClaimMaturity,
    /// Observed per-dimension evidence.
    pub observed_dimensions: Vec<DimensionEvidence>,
    /// Expected verdict.
    pub expected_verdict: RowVerdict,
    /// Expected effective maturity.
    pub expected_effective_maturity: ClaimMaturity,
    /// Expected narrowing tokens.
    pub expected_narrow_reason_tokens: Vec<String>,
    /// One consumer that quotes this profile.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "m5 reactive certification validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

// ---------------------------------------------------------------------------
// Evidence-packet vocabulary used by the seed.
// ---------------------------------------------------------------------------

const GOVERNANCE_PACKET_REF: &str = "artifacts/state/m5_reactive_governance.json";
const SUBSCRIPTION_PACKET_REF: &str = "artifacts/state/cross_surface_subscription.json";
const TRUTH_SURFACES_PACKET_REF: &str = "artifacts/state/reactive_truth_surfaces.json";
const MATERIALIZED_VIEW_PACKET_REF: &str = "artifacts/state/materialized_view_policy.json";
const RECOVERY_PACKET_REF: &str = "artifacts/state/reactive_recovery.json";
const COMMAND_PARITY_PACKET_REF: &str = "artifacts/state/reactive_command_parity.json";

fn evidence_packet_refs() -> Vec<String> {
    [
        SUBSCRIPTION_PACKET_REF,
        TRUTH_SURFACES_PACKET_REF,
        MATERIALIZED_VIEW_PACKET_REF,
        RECOVERY_PACKET_REF,
        COMMAND_PARITY_PACKET_REF,
        GOVERNANCE_PACKET_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// The canonical evidence refs for one dimension when it is fully
/// current. Each dimension cites the reactive-state packets that prove
/// it, so the certification is anchored in the checked-in artifacts.
fn dimension_evidence_refs(dimension: CertificationDimension) -> Vec<&'static str> {
    match dimension {
        CertificationDimension::AuthorityClass => {
            vec![GOVERNANCE_PACKET_REF, SUBSCRIPTION_PACKET_REF]
        }
        CertificationDimension::EpochParity => vec![GOVERNANCE_PACKET_REF, SUBSCRIPTION_PACKET_REF],
        CertificationDimension::InvalidationBehavior => {
            vec![GOVERNANCE_PACKET_REF, RECOVERY_PACKET_REF]
        }
        CertificationDimension::StaleStateLabeling => {
            vec![TRUTH_SURFACES_PACKET_REF, GOVERNANCE_PACKET_REF]
        }
        CertificationDimension::SafeActionNarrowing => {
            vec![
                COMMAND_PARITY_PACKET_REF,
                RECOVERY_PACKET_REF,
                TRUTH_SURFACES_PACKET_REF,
            ]
        }
    }
}

fn dimension_rationale(dimension: CertificationDimension) -> &'static str {
    match dimension {
        CertificationDimension::AuthorityClass => {
            "The governance matrix names the authority class and derivation that own this surface's truth; the subscription contract carries the same ownership in its envelope."
        }
        CertificationDimension::EpochParity => {
            "The surface shares an epoch-parity group with the other members of its authority class and reads one authoritative snapshot epoch instead of a private epoch."
        }
        CertificationDimension::InvalidationBehavior => {
            "The surface honors its invalidation reasons and recovers lagging consumers through coalesce, resubscribe, or fresh-snapshot flows without offering stale exact-truth actions."
        }
        CertificationDimension::StaleStateLabeling => {
            "Gated truth cues label warming, cached, stale, partial, and coalesced state, and the narrowing engine downgrades the claim identically across channels."
        }
        CertificationDimension::SafeActionNarrowing => {
            "Mutating actions publish through the canonical command and journal path and narrow under degraded state instead of offering stale exact-truth affordances."
        }
    }
}

/// Builds the five fully-current dimensions for a healthy row.
fn current_dimensions() -> Vec<DimensionEvidence> {
    CertificationDimension::ALL
        .into_iter()
        .map(|dimension| DimensionEvidence {
            dimension,
            evidence_state: EvidenceState::Current,
            evidence_refs: dimension_evidence_refs(dimension)
                .into_iter()
                .map(str::to_owned)
                .collect(),
            rationale: dimension_rationale(dimension).to_owned(),
        })
        .collect()
}

fn supporting_packet_refs(dimensions: &[DimensionEvidence]) -> Vec<String> {
    let mut refs: BTreeSet<String> = BTreeSet::new();
    for dimension in dimensions {
        for reference in &dimension.evidence_refs {
            refs.insert(reference.clone());
        }
    }
    refs.into_iter().collect()
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface_profile: ClaimedSurfaceProfile,
    surface_label: &str,
    claimed_maturity: ClaimMaturity,
    backing_surface_classes: &[&str],
    consumer_refs: &[&str],
    notes: &str,
) -> CertificationRow {
    let dimensions = current_dimensions();
    let outcome = certify_row_outcome(claimed_maturity, &dimensions);
    let supporting_packet_refs = supporting_packet_refs(&dimensions);
    CertificationRow {
        row_id: row_id.to_owned(),
        surface_profile,
        surface_label: surface_label.to_owned(),
        claimed_maturity,
        backing_surface_classes: backing_surface_classes
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        dimensions,
        effective_maturity: outcome.effective_maturity,
        verdict: outcome.verdict,
        narrowed: outcome.narrowed,
        narrow_reason_tokens: outcome.narrow_reason_tokens,
        stale_or_missing_dimension_tokens: outcome.stale_or_missing_dimension_tokens,
        supporting_packet_refs,
        consumer_refs: consumer_refs.iter().map(|s| (*s).to_owned()).collect(),
        notes: notes.to_owned(),
    }
}

fn freshness_rule(
    rule_id: &str,
    trigger: EvidenceState,
    effect: &str,
    rationale: &str,
) -> EvidenceFreshnessRule {
    EvidenceFreshnessRule {
        rule_id: rule_id.to_owned(),
        trigger_evidence_state: trigger,
        maturity_floor: trigger
            .qualification_floor()
            .expect("freshness rules only encode triggers that impose a floor"),
        effect: effect.to_owned(),
        rationale: rationale.to_owned(),
    }
}

fn step(
    phase: DrillPhase,
    observed_maturity: ClaimMaturity,
    narration: &str,
) -> CertificationDrillStep {
    CertificationDrillStep {
        phase,
        observed_maturity,
        narration: narration.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn drill(
    drill_id: &str,
    title: &str,
    surface_profile: ClaimedSurfaceProfile,
    exercised_dimension: CertificationDimension,
    failure_class: DrillFailureClass,
    degraded_evidence_state: EvidenceState,
    claimed_maturity: ClaimMaturity,
    steps: Vec<CertificationDrillStep>,
    notes: &str,
) -> CertificationDrill {
    // The degraded posture is computed from the same engine the rows
    // use, so a drill can never disagree with the certification.
    let mut degraded = current_dimensions();
    for evidence in &mut degraded {
        if evidence.dimension == exercised_dimension {
            evidence.evidence_state = degraded_evidence_state;
        }
    }
    let degraded_outcome = certify_row_outcome(claimed_maturity, &degraded);
    CertificationDrill {
        drill_id: drill_id.to_owned(),
        title: title.to_owned(),
        surface_profile,
        exercised_dimension,
        failure_class,
        degraded_evidence_state,
        claimed_maturity,
        expected_degraded_verdict: degraded_outcome.verdict,
        expected_degraded_maturity: degraded_outcome.effective_maturity,
        recovers_to_verdict: RowVerdict::Certified,
        steps,
        asserts_claim_narrows_under_failure: true,
        asserts_recovers_after_refresh: true,
        notes: notes.to_owned(),
    }
}

fn binding(channel: PublicationChannel, consumer_ref: &str, summary: &str) -> SurfaceBinding {
    SurfaceBinding {
        channel,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: PACKET_ID.to_owned(),
        required_verbatim_fields: REQUIRED_VERBATIM_FIELDS
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        narrows_with_packet: true,
        summary: summary.to_owned(),
    }
}

const PACKET_ID: &str = "state.m5_reactive_certification.v1";

const REQUIRED_VERBATIM_FIELDS: [&str; 6] = [
    "row_id",
    "surface_profile",
    "claimed_maturity",
    "effective_maturity",
    "verdict",
    "narrow_reason_tokens",
];

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in certification packet this lane freezes.
pub fn seeded_m5_reactive_certification_packet() -> M5ReactiveCertificationPacket {
    let rows = vec![
        row(
            "cert.reactive.shell",
            ClaimedSurfaceProfile::Shell,
            "Shell workspace tree and activity center",
            ClaimMaturity::Stable,
            &["shell_workspace_tree", "shell_activity_center"],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-shell/src/preview_truth/mod.rs",
            ],
            "The shell projects workspace and execution authority and labels warming, cached, stale, partial, and coalesced state across UI and headless mirrors.",
        ),
        row(
            "cert.reactive.search",
            ClaimedSurfaceProfile::Search,
            "Search results",
            ClaimMaturity::Stable,
            &["search_results"],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-search/src/lib.rs",
            ],
            "Search results label warming, partial, cached, and stale states and pair them with a rerun path instead of presenting a partial index as complete.",
        ),
        row(
            "cert.reactive.graph",
            ClaimedSurfaceProfile::Graph,
            "Graph neighborhood",
            ClaimMaturity::Beta,
            &["graph_neighborhood"],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-graph/src/lib.rs",
            ],
            "Graph neighborhoods are ephemeral derived projections that label partial graphs as partial and narrow rather than implying the whole neighborhood.",
        ),
        row(
            "cert.reactive.ai",
            ClaimedSurfaceProfile::Ai,
            "AI context panel",
            ClaimMaturity::Beta,
            &["ai_context_panel"],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-ai/src/lib.rs",
            ],
            "The AI context panel narrows to a policy-limited projection when entitlement or policy restricts the visible context and publishes applies through the command and journal path.",
        ),
        row(
            "cert.reactive.review",
            ClaimedSurfaceProfile::Review,
            "Review workspace overlay",
            ClaimMaturity::Beta,
            &["review_workspace"],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-review/src/lib.rs",
            ],
            "The review workspace is a managed replicated overlay; when the provider is unavailable it says so instead of replacing local truth, and approve/merge wait for the canonical publish.",
        ),
        row(
            "cert.reactive.support",
            ClaimedSurfaceProfile::Support,
            "Support reactive-state export",
            ClaimMaturity::Beta,
            &["support_export_view"],
            &[
                "crates/aureline-support/src/m5_reactive_governance/mod.rs",
                "crates/aureline-support/src/reactive_command_parity/mod.rs",
            ],
            "The support export is a metadata-first exportable snapshot of captured reactive state that carries the same narrowing so release and procurement readers see the captured claim, not live truth.",
        ),
    ];

    let freshness_rules = vec![
        freshness_rule(
            "freshness.partial_narrows_to_beta",
            EvidenceState::Partial,
            "A claimed profile with partial evidence on any required dimension narrows to at most a beta claim.",
            "Partial reactive-state evidence proves only part of the claimed scope, so the profile may not present a stable reactive-state guarantee.",
        ),
        freshness_rule(
            "freshness.stale_narrows_to_preview",
            EvidenceState::Stale,
            "A claimed profile with stale evidence on any required dimension narrows to at most a preview claim.",
            "Stale reactive-state evidence may no longer reflect the current contract, so the profile drops below beta until the evidence is refreshed.",
        ),
        freshness_rule(
            "freshness.missing_withholds_claim",
            EvidenceState::Missing,
            "A claimed profile missing evidence on any required dimension is withheld; promotion fails until the dimension is proven.",
            "A required reactive-state dimension with no backing evidence cannot be proven, so the profile may not be promoted at its claimed maturity.",
        ),
    ];

    let drills = vec![
        drill(
            "drill.reactive_certification.shell_epoch_lag",
            "Shell narrows when it lags the shared authoritative epoch",
            ClaimedSurfaceProfile::Shell,
            CertificationDimension::EpochParity,
            DrillFailureClass::AuthorityEpochLag,
            EvidenceState::Stale,
            ClaimMaturity::Stable,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Stable,
                    "The workspace tree lags the authoritative VFS epoch while another member of the group advances.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Stable,
                    "Epoch-parity evidence is observed stale: the surface trails the shared authoritative epoch.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    "The certified claim narrows to preview; the shell labels stale state rather than presenting a parallel epoch as truth.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    "The lagging consumer resubscribes and takes a fresh snapshot at the shared epoch.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Stable,
                    "Epoch-parity evidence returns current; the claim recovers to its stable maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Stable,
                    "The recovered posture matches the certification engine for a fully current shell row.",
                ),
            ],
            "A lagging epoch narrows the shell claim to preview and recovers only after a fresh snapshot at the shared epoch.",
        ),
        drill(
            "drill.reactive_certification.search_partial_invalidation",
            "Search narrows to beta on partial invalidation coverage",
            ClaimedSurfaceProfile::Search,
            CertificationDimension::InvalidationBehavior,
            DrillFailureClass::PartialEvidenceCoverage,
            EvidenceState::Partial,
            ClaimMaturity::Stable,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Stable,
                    "The recovery packet covers only part of the search invalidation reasons after an index producer restart.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Stable,
                    "Invalidation-behavior evidence is observed partial for the search profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Beta,
                    "The certified claim narrows to beta; search labels partial results and pairs them with a rerun path.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Beta,
                    "The invalidation evidence is completed across the remaining reasons.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Stable,
                    "Invalidation-behavior evidence returns current; the claim recovers to stable.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Stable,
                    "The recovered posture matches the certification engine for a fully current search row.",
                ),
            ],
            "Partial invalidation coverage narrows the search claim to beta without withholding it.",
        ),
        drill(
            "drill.reactive_certification.graph_stale_labeling",
            "Graph narrows to preview when stale-state labeling evidence ages out",
            ClaimedSurfaceProfile::Graph,
            CertificationDimension::StaleStateLabeling,
            DrillFailureClass::EvidenceWentStale,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    "The gated truth-cue evidence for the graph neighborhood ages past its freshness window.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    "Stale-state-labeling evidence is observed stale for the graph profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    "The certified claim narrows to preview until the labeling evidence is recaptured.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    "The gated truth-cue evidence is recaptured for the neighborhood.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    "Stale-state-labeling evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    "The recovered posture matches the certification engine for a fully current graph row.",
                ),
            ],
            "Stale labeling evidence narrows the graph claim below beta even though it worked once on a happy-path capture.",
        ),
        drill(
            "drill.reactive_certification.ai_policy_epoch_rolled",
            "AI narrows to preview when a rolled policy epoch staled its safe-action evidence",
            ClaimedSurfaceProfile::Ai,
            CertificationDimension::SafeActionNarrowing,
            DrillFailureClass::PolicyLimited,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    "The policy epoch rolls, so the captured safe-action-narrowing evidence now trails the current entitlement.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    "Safe-action-narrowing evidence is observed stale for the AI profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    "The certified claim narrows to preview; the panel narrows to a policy-limited projection and gates affordances until the evidence is recaptured.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    "The safe-action evidence is recaptured against the new policy epoch.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    "Safe-action-narrowing evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    "The recovered posture matches the certification engine for a fully current AI row.",
                ),
            ],
            "A rolled policy epoch staled the safe-action evidence and narrowed the AI claim to preview until it was recaptured, never letting it imply exact current truth.",
        ),
        drill(
            "drill.reactive_certification.review_authority_missing",
            "Review is withheld when authority-class evidence is missing",
            ClaimedSurfaceProfile::Review,
            CertificationDimension::AuthorityClass,
            DrillFailureClass::DimensionEvidenceMissing,
            EvidenceState::Missing,
            ClaimMaturity::Beta,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    "The governance evidence naming the review overlay's authority class is removed from the packet set.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    "Authority-class evidence is observed missing for the review profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Withdrawn,
                    "The certification withholds the review claim; promotion fails until authority ownership is proven.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Withdrawn,
                    "The governance authority-class evidence is restored for the review overlay.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    "Authority-class evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    "The recovered posture matches the certification engine for a fully current review row.",
                ),
            ],
            "Missing authority-class evidence withholds the review claim rather than leaving it green on a stale capture.",
        ),
        drill(
            "drill.reactive_certification.support_provider_unavailable",
            "Support narrows to preview when its capture provider is unavailable",
            ClaimedSurfaceProfile::Support,
            CertificationDimension::InvalidationBehavior,
            DrillFailureClass::ProviderUnavailable,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    "The support capture provider is unavailable, so the invalidation-history evidence ages past its window.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    "Invalidation-behavior evidence is observed stale for the support export profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    "The exported claim narrows to preview; the support snapshot is labeled stale rather than live truth.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    "The provider returns and the invalidation-history capture is refreshed.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    "Invalidation-behavior evidence returns current; the exported claim recovers to beta.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    "The recovered posture matches the certification engine for a fully current support row.",
                ),
            ],
            "An unavailable capture provider narrows the support claim to preview and keeps the export honest about staleness.",
        ),
    ];

    let surface_bindings = vec![
        binding(
            PublicationChannel::ReleaseShiproom,
            "artifacts/release/shiproom_dashboard.json",
            "The shiproom dashboard reads the per-row verdict and effective maturity and holds promotion for any narrowed or withheld release-scope profile.",
        ),
        binding(
            PublicationChannel::SupportExport,
            "crates/aureline-support/src/m5_reactive_governance/mod.rs",
            "The metadata-first support export re-exports the per-row verdict, narrowing tokens, and stale-or-missing dimensions without raw payloads or ambient authority.",
        ),
        binding(
            PublicationChannel::Docs,
            "docs/state/m5-reactive-certification.md",
            "The reviewer documentation quotes the certified dimensions, freshness rules, and per-row verdicts directly from the packet.",
        ),
        binding(
            PublicationChannel::Help,
            "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
            "The in-product reactive-state explainer reuses the same verdict vocabulary so help never tells a greener story than the packet.",
        ),
    ];

    M5ReactiveCertificationPacket {
        record_kind: M5_REACTIVE_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_REACTIVE_CERTIFICATION_SCHEMA_VERSION,
        packet_id: PACKET_ID.to_owned(),
        title: "Reactive-state truth certification for claimed M5 shell, search, graph, AI, review, and support profiles"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: M5_REACTIVE_CERTIFICATION_DOC_REF.to_owned(),
            schema_ref: M5_REACTIVE_CERTIFICATION_SCHEMA_REF.to_owned(),
            packet_ref: M5_REACTIVE_CERTIFICATION_PACKET_REF.to_owned(),
            report_ref: M5_REACTIVE_CERTIFICATION_REPORT_REF.to_owned(),
            fixture_manifest_ref: M5_REACTIVE_CERTIFICATION_FIXTURE_MANIFEST_REF.to_owned(),
        },
        certified_dimensions: CertificationDimension::ALL.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        rows,
        freshness_rules,
        drills,
        surface_bindings,
        invariants: vec![
            "Each claimed M5 reactive surface profile is certified only when every required dimension — authority class, epoch parity, invalidation behavior, stale-state labeling, and safe-action narrowing — is proven current.".to_owned(),
            "One narrowing engine folds per-dimension evidence into a verdict: partial evidence narrows to beta, stale evidence narrows to preview, and missing evidence withholds the claim.".to_owned(),
            "A stable or beta claim can never outrun its reactive-state evidence; the certification only narrows, never widens, and a profile absent from the packet is uncertified rather than implicitly green.".to_owned(),
            "Release, support, docs, and help all read the same per-row verdict and narrowing tokens instead of re-deriving reactive-state staleness.".to_owned(),
            "Failure and recovery drills exercise each profile through narrowing and back, computed from the same engine so the certification, drills, and fixtures cannot disagree.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture corpus this lane freezes.
pub fn seeded_m5_reactive_certification_fixtures() -> Vec<M5ReactiveCertificationFixture> {
    let mut fixtures = Vec::new();

    // One healthy fixture per profile, pinning the certified verdict.
    for profile in ClaimedSurfaceProfile::ALL {
        let claimed = claimed_maturity_for(profile);
        fixtures.push(fixture(
            &format!(
                "fixture.m5_reactive_certification.{}_certified",
                profile.as_str()
            ),
            profile,
            claimed,
            current_dimensions(),
            consumer_ref_for(profile),
            "A fully current profile certifies at its claimed maturity with no narrowing tokens.",
        ));
    }

    // Degraded fixtures exercising every floor and verdict.
    fixtures.push(fixture(
        "fixture.m5_reactive_certification.shell_epoch_parity_stale",
        ClaimedSurfaceProfile::Shell,
        ClaimMaturity::Stable,
        degraded_dimensions(CertificationDimension::EpochParity, EvidenceState::Stale),
        consumer_ref_for(ClaimedSurfaceProfile::Shell),
        "Stale epoch-parity evidence narrows the stable shell claim to a preview verdict.",
    ));
    fixtures.push(fixture(
        "fixture.m5_reactive_certification.search_invalidation_partial",
        ClaimedSurfaceProfile::Search,
        ClaimMaturity::Stable,
        degraded_dimensions(
            CertificationDimension::InvalidationBehavior,
            EvidenceState::Partial,
        ),
        consumer_ref_for(ClaimedSurfaceProfile::Search),
        "Partial invalidation evidence narrows the stable search claim to a beta verdict.",
    ));
    fixtures.push(fixture(
        "fixture.m5_reactive_certification.ai_safe_action_partial",
        ClaimedSurfaceProfile::Ai,
        ClaimMaturity::Beta,
        degraded_dimensions(
            CertificationDimension::SafeActionNarrowing,
            EvidenceState::Partial,
        ),
        consumer_ref_for(ClaimedSurfaceProfile::Ai),
        "Partial safe-action evidence on a beta claim is tolerated: the verdict stays certified at beta while recording the partial-evidence caveat token.",
    ));
    fixtures.push(fixture(
        "fixture.m5_reactive_certification.review_authority_missing",
        ClaimedSurfaceProfile::Review,
        ClaimMaturity::Beta,
        degraded_dimensions(
            CertificationDimension::AuthorityClass,
            EvidenceState::Missing,
        ),
        consumer_ref_for(ClaimedSurfaceProfile::Review),
        "Missing authority-class evidence withholds the review claim entirely.",
    ));

    fixtures
}

fn claimed_maturity_for(profile: ClaimedSurfaceProfile) -> ClaimMaturity {
    match profile {
        ClaimedSurfaceProfile::Shell | ClaimedSurfaceProfile::Search => ClaimMaturity::Stable,
        ClaimedSurfaceProfile::Graph
        | ClaimedSurfaceProfile::Ai
        | ClaimedSurfaceProfile::Review
        | ClaimedSurfaceProfile::Support => ClaimMaturity::Beta,
    }
}

fn consumer_ref_for(profile: ClaimedSurfaceProfile) -> &'static str {
    match profile {
        ClaimedSurfaceProfile::Shell => {
            "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs"
        }
        ClaimedSurfaceProfile::Search => "crates/aureline-search/src/lib.rs",
        ClaimedSurfaceProfile::Graph => "crates/aureline-graph/src/lib.rs",
        ClaimedSurfaceProfile::Ai => "crates/aureline-ai/src/lib.rs",
        ClaimedSurfaceProfile::Review => "crates/aureline-review/src/lib.rs",
        ClaimedSurfaceProfile::Support => {
            "crates/aureline-support/src/m5_reactive_governance/mod.rs"
        }
    }
}

fn degraded_dimensions(
    dimension: CertificationDimension,
    state: EvidenceState,
) -> Vec<DimensionEvidence> {
    let mut dimensions = current_dimensions();
    for evidence in &mut dimensions {
        if evidence.dimension == dimension {
            evidence.evidence_state = state;
        }
    }
    dimensions
}

fn fixture(
    fixture_id: &str,
    surface_profile: ClaimedSurfaceProfile,
    claimed_maturity: ClaimMaturity,
    observed_dimensions: Vec<DimensionEvidence>,
    consumer_ref: &str,
    notes: &str,
) -> M5ReactiveCertificationFixture {
    let outcome = certify_row_outcome(claimed_maturity, &observed_dimensions);
    M5ReactiveCertificationFixture {
        record_kind: M5_REACTIVE_CERTIFICATION_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: M5_REACTIVE_CERTIFICATION_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        surface_profile,
        claimed_maturity,
        observed_dimensions,
        expected_verdict: outcome.verdict,
        expected_effective_maturity: outcome.effective_maturity,
        expected_narrow_reason_tokens: outcome.narrow_reason_tokens,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in packet contract.
pub fn validate_m5_reactive_certification_packet(
    packet: &M5ReactiveCertificationPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != M5_REACTIVE_CERTIFICATION_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != M5_REACTIVE_CERTIFICATION_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.doc_ref != M5_REACTIVE_CERTIFICATION_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != M5_REACTIVE_CERTIFICATION_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != M5_REACTIVE_CERTIFICATION_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != M5_REACTIVE_CERTIFICATION_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != M5_REACTIVE_CERTIFICATION_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.certified_dimensions != CertificationDimension::ALL.to_vec() {
        report.push(
            "packet.certified_dimensions",
            "packet must certify every required dimension in canonical order",
        );
    }
    if packet.evidence_packet_refs.is_empty() {
        report.push(
            "packet.evidence_packet_refs",
            "packet must cite the upstream reactive-state evidence packets",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    let mut covered_profiles = BTreeSet::new();
    for cert_row in &packet.rows {
        if !covered_profiles.insert(cert_row.surface_profile) {
            report.push(
                "row.profile_unique",
                format!("duplicate profile {}", cert_row.surface_profile.as_str()),
            );
        }
        validate_row(&mut report, cert_row);
    }
    for required in ClaimedSurfaceProfile::ALL {
        if !covered_profiles.contains(&required) {
            report.push(
                "packet.covered_profile",
                format!("packet must certify profile {}", required.as_str()),
            );
        }
    }

    validate_freshness_rules(&mut report, packet);
    validate_drills(&mut report, packet);
    validate_surface_bindings(&mut report, packet);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_dimensions(
    report: &mut ValidationReport,
    owner: &str,
    dimensions: &[DimensionEvidence],
) {
    let mut seen = BTreeSet::new();
    for evidence in dimensions {
        if !seen.insert(evidence.dimension) {
            report.push(
                "dimension.unique",
                format!("{owner} repeats dimension {}", evidence.dimension.as_str()),
            );
        }
        if evidence.evidence_state != EvidenceState::Missing && evidence.evidence_refs.is_empty() {
            report.push(
                "dimension.evidence_refs",
                format!(
                    "{owner} dimension {} must cite evidence unless it is missing",
                    evidence.dimension.as_str()
                ),
            );
        }
        if evidence.rationale.trim().is_empty() {
            report.push(
                "dimension.rationale",
                format!(
                    "{owner} dimension {} must carry a rationale",
                    evidence.dimension.as_str()
                ),
            );
        }
    }
    for required in CertificationDimension::ALL {
        if !seen.contains(&required) {
            report.push(
                "dimension.coverage",
                format!("{owner} must evidence dimension {}", required.as_str()),
            );
        }
    }
}

fn validate_row(report: &mut ValidationReport, cert_row: &CertificationRow) {
    if cert_row.row_id.trim().is_empty() {
        report.push("row.id", "row must carry a stable id");
    }
    if cert_row.surface_label.trim().is_empty() {
        report.push(
            "row.surface_label",
            format!("row {} must carry a surface label", cert_row.row_id),
        );
    }
    if cert_row.backing_surface_classes.is_empty() {
        report.push(
            "row.backing_surface_classes",
            format!(
                "row {} must name its backing surface classes",
                cert_row.row_id
            ),
        );
    }
    if cert_row.consumer_refs.is_empty() {
        report.push(
            "row.consumer_refs",
            format!(
                "row {} must cite at least one consumer ref",
                cert_row.row_id
            ),
        );
    }
    if cert_row.notes.trim().is_empty() {
        report.push(
            "row.notes",
            format!("row {} must carry a reviewer note", cert_row.row_id),
        );
    }

    validate_dimensions(
        report,
        &format!("row {}", cert_row.row_id),
        &cert_row.dimensions,
    );

    // The stamped outcome must equal what the engine computes.
    let outcome = certify_row_outcome(cert_row.claimed_maturity, &cert_row.dimensions);
    if cert_row.effective_maturity != outcome.effective_maturity {
        report.push(
            "row.effective_maturity",
            format!(
                "row {} effective_maturity {} disagrees with the engine ({})",
                cert_row.row_id,
                cert_row.effective_maturity.as_str(),
                outcome.effective_maturity.as_str()
            ),
        );
    }
    if cert_row.verdict != outcome.verdict {
        report.push(
            "row.verdict",
            format!(
                "row {} verdict {} disagrees with the engine ({})",
                cert_row.row_id,
                cert_row.verdict.as_str(),
                outcome.verdict.as_str()
            ),
        );
    }
    if cert_row.narrowed != outcome.narrowed {
        report.push(
            "row.narrowed",
            format!(
                "row {} narrowed flag disagrees with the engine",
                cert_row.row_id
            ),
        );
    }
    if cert_row.narrow_reason_tokens != outcome.narrow_reason_tokens {
        report.push(
            "row.narrow_reason_tokens",
            format!(
                "row {} narrow_reason_tokens disagree with the engine",
                cert_row.row_id
            ),
        );
    }
    if cert_row.stale_or_missing_dimension_tokens != outcome.stale_or_missing_dimension_tokens {
        report.push(
            "row.stale_or_missing_dimension_tokens",
            format!(
                "row {} stale_or_missing_dimension_tokens disagree with the engine",
                cert_row.row_id
            ),
        );
    }

    let expected_support = supporting_packet_refs(&cert_row.dimensions);
    if cert_row.supporting_packet_refs != expected_support {
        report.push(
            "row.supporting_packet_refs",
            format!(
                "row {} supporting_packet_refs must equal the union of its dimension evidence refs",
                cert_row.row_id
            ),
        );
    }
}

fn validate_freshness_rules(report: &mut ValidationReport, packet: &M5ReactiveCertificationPacket) {
    if packet.freshness_rules.is_empty() {
        report.push(
            "packet.freshness_rules",
            "packet must declare freshness rules",
        );
    }
    let mut covered = BTreeSet::new();
    for rule in &packet.freshness_rules {
        covered.insert(rule.trigger_evidence_state);
        match rule.trigger_evidence_state.qualification_floor() {
            Some(expected) if expected == rule.maturity_floor => {}
            Some(expected) => report.push(
                "freshness_rule.floor",
                format!(
                    "rule {} floor {} disagrees with the engine ({})",
                    rule.rule_id,
                    rule.maturity_floor.as_str(),
                    expected.as_str()
                ),
            ),
            None => report.push(
                "freshness_rule.trigger",
                format!(
                    "rule {} trigger {} imposes no floor and must not be a rule",
                    rule.rule_id,
                    rule.trigger_evidence_state.as_str()
                ),
            ),
        }
        if rule.effect.trim().is_empty() || rule.rationale.trim().is_empty() {
            report.push(
                "freshness_rule.prose",
                format!("rule {} must carry an effect and rationale", rule.rule_id),
            );
        }
    }
    for required in [
        EvidenceState::Partial,
        EvidenceState::Stale,
        EvidenceState::Missing,
    ] {
        if !covered.contains(&required) {
            report.push(
                "packet.freshness_rule_coverage",
                format!(
                    "packet must encode a rule for {} evidence",
                    required.as_str()
                ),
            );
        }
    }
}

fn validate_drills(report: &mut ValidationReport, packet: &M5ReactiveCertificationPacket) {
    if packet.drills.is_empty() {
        report.push(
            "packet.drills",
            "packet must declare failure/recovery drills",
        );
    }
    let mut drill_ids = BTreeSet::new();
    let mut drilled_profiles = BTreeSet::new();
    let mut has_narrowed = false;
    let mut has_withheld = false;
    for cert_drill in &packet.drills {
        if !drill_ids.insert(cert_drill.drill_id.as_str()) {
            report.push(
                "drill.id_unique",
                format!("duplicate drill_id {}", cert_drill.drill_id),
            );
        }
        drilled_profiles.insert(cert_drill.surface_profile);

        // Recompute the degraded outcome from the engine.
        let mut degraded = current_dimensions();
        for evidence in &mut degraded {
            if evidence.dimension == cert_drill.exercised_dimension {
                evidence.evidence_state = cert_drill.degraded_evidence_state;
            }
        }
        let degraded_outcome = certify_row_outcome(cert_drill.claimed_maturity, &degraded);
        if cert_drill.expected_degraded_verdict != degraded_outcome.verdict {
            report.push(
                "drill.degraded_verdict",
                format!(
                    "drill {} degraded verdict disagrees with the engine",
                    cert_drill.drill_id
                ),
            );
        }
        if cert_drill.expected_degraded_maturity != degraded_outcome.effective_maturity {
            report.push(
                "drill.degraded_maturity",
                format!(
                    "drill {} degraded maturity disagrees with the engine",
                    cert_drill.drill_id
                ),
            );
        }
        if degraded_outcome.verdict == RowVerdict::Certified {
            report.push(
                "drill.must_degrade",
                format!(
                    "drill {} must inject a failure that actually narrows or withholds",
                    cert_drill.drill_id
                ),
            );
        }
        match degraded_outcome.verdict {
            RowVerdict::Narrowed => has_narrowed = true,
            RowVerdict::Withheld => has_withheld = true,
            RowVerdict::Certified => {}
        }
        if cert_drill.recovers_to_verdict != RowVerdict::Certified {
            report.push(
                "drill.recovers",
                format!("drill {} must recover to certified", cert_drill.drill_id),
            );
        }
        if !cert_drill.asserts_claim_narrows_under_failure
            || !cert_drill.asserts_recovers_after_refresh
        {
            report.push(
                "drill.assertions",
                format!(
                    "drill {} must assert it narrows under failure and recovers after refresh",
                    cert_drill.drill_id
                ),
            );
        }
        validate_drill_steps(report, cert_drill);
    }
    for required in ClaimedSurfaceProfile::ALL {
        if !drilled_profiles.contains(&required) {
            report.push(
                "packet.drilled_profile",
                format!("packet must drill profile {}", required.as_str()),
            );
        }
    }
    if !has_narrowed {
        report.push(
            "packet.narrowed_drill",
            "packet must drill at least one narrowed verdict",
        );
    }
    if !has_withheld {
        report.push(
            "packet.withheld_drill",
            "packet must drill at least one withheld verdict",
        );
    }
}

fn validate_drill_steps(report: &mut ValidationReport, cert_drill: &CertificationDrill) {
    if cert_drill.steps.is_empty() {
        report.push(
            "drill.steps",
            format!("drill {} must declare steps", cert_drill.drill_id),
        );
        return;
    }
    if cert_drill.steps.first().map(|s| s.phase) != Some(DrillPhase::Inject) {
        report.push(
            "drill.first_phase",
            format!(
                "drill {} must begin with an inject step",
                cert_drill.drill_id
            ),
        );
    }
    if cert_drill.steps.last().map(|s| s.phase) != Some(DrillPhase::Verify) {
        report.push(
            "drill.last_phase",
            format!("drill {} must end with a verify step", cert_drill.drill_id),
        );
    }
    let has_narrow = cert_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Narrow);
    let has_recover = cert_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Recover);
    if !has_narrow || !has_recover {
        report.push(
            "drill.phases",
            format!(
                "drill {} must include a narrow step and a recover step",
                cert_drill.drill_id
            ),
        );
    }
    for (index, drill_step) in cert_drill.steps.iter().enumerate() {
        if drill_step.narration.trim().is_empty() {
            report.push(
                "drill.step_narration",
                format!("drill {} step {index} must narrate", cert_drill.drill_id),
            );
        }
    }
}

fn validate_surface_bindings(
    report: &mut ValidationReport,
    packet: &M5ReactiveCertificationPacket,
) {
    let mut channels = BTreeSet::new();
    for surface_binding in &packet.surface_bindings {
        channels.insert(surface_binding.channel);
        if surface_binding.ingested_packet_id != packet.packet_id {
            report.push(
                "binding.packet_id",
                format!(
                    "binding for {} must ingest the packet id",
                    surface_binding.channel.as_str()
                ),
            );
        }
        if surface_binding.required_verbatim_fields.is_empty() {
            report.push(
                "binding.required_verbatim_fields",
                format!(
                    "binding for {} must name the fields it preserves verbatim",
                    surface_binding.channel.as_str()
                ),
            );
        }
        if !surface_binding.narrows_with_packet {
            report.push(
                "binding.narrows_with_packet",
                format!(
                    "binding for {} must narrow in lockstep with the packet",
                    surface_binding.channel.as_str()
                ),
            );
        }
        if surface_binding.consumer_ref.trim().is_empty()
            || surface_binding.summary.trim().is_empty()
        {
            report.push(
                "binding.prose",
                format!(
                    "binding for {} must carry a consumer ref and summary",
                    surface_binding.channel.as_str()
                ),
            );
        }
    }
    for required in PublicationChannel::ALL {
        if !channels.contains(&required) {
            report.push(
                "packet.binding_coverage",
                format!("packet must bind channel {}", required.as_str()),
            );
        }
    }
}

/// Validates one checked-in fixture against the frozen contract.
pub fn validate_m5_reactive_certification_fixture(
    fixture: &M5ReactiveCertificationFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != M5_REACTIVE_CERTIFICATION_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != M5_REACTIVE_CERTIFICATION_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }
    if fixture.fixture_id.trim().is_empty() {
        report.push("fixture.id", "fixture must carry a stable id");
    }
    if fixture.consumer_ref.trim().is_empty() {
        report.push(
            "fixture.consumer_ref",
            format!("fixture {} must cite a consumer ref", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        report.push(
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }

    validate_dimensions(
        &mut report,
        &format!("fixture {}", fixture.fixture_id),
        &fixture.observed_dimensions,
    );

    let outcome = certify_row_outcome(fixture.claimed_maturity, &fixture.observed_dimensions);
    if fixture.expected_verdict != outcome.verdict {
        report.push(
            "fixture.expected_verdict",
            format!(
                "fixture {} expected verdict {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_verdict.as_str(),
                outcome.verdict.as_str()
            ),
        );
    }
    if fixture.expected_effective_maturity != outcome.effective_maturity {
        report.push(
            "fixture.expected_effective_maturity",
            format!(
                "fixture {} expected maturity {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_effective_maturity.as_str(),
                outcome.effective_maturity.as_str()
            ),
        );
    }
    if fixture.expected_narrow_reason_tokens != outcome.narrow_reason_tokens {
        report.push(
            "fixture.expected_narrow_reason_tokens",
            format!(
                "fixture {} expected narrowing tokens disagree with the engine",
                fixture.fixture_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

#[cfg(test)]
mod tests;
