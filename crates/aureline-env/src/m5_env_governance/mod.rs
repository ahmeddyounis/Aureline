//! Governance-grade certification of environment-capsule truth on
//! claimed M5 template / starter / prebuild / devcontainer / remote /
//! managed-workspace profiles.
//!
//! M5 templates, starters, prebuilds, remote/container/devcontainer
//! flows, and managed workspaces all need a typed *environment capsule*:
//! the source digest that identifies an environment, the target plan it
//! materializes to, the toolchain plan it pins, the trust-gated
//! lifecycle hooks it declares, the service graph it stands up, the
//! prebuild fingerprint it reuses, and the runtime materialization that
//! must stay semantically aligned with the same capsule object across
//! desktop, CLI, AI, support, and managed rows. This module freezes one
//! canonical matrix over those objects so later M5 surfaces stop
//! inferring environment truth from side effects.
//!
//! The module models one [`CapsuleRow`] per claimed
//! [`EnvironmentProfile`], each carrying the seven required
//! [`CapsuleDimension`]s and the evidence backing each. A single
//! [`certify_capsule_outcome`] engine folds the per-dimension evidence
//! into one [`RowVerdict`] (`certified` / `narrowed` / `withheld`), an
//! effective [`ClaimMaturity`] floor, **and** a narrowed
//! [`WarmStartPosture`], so a `stable` or `beta` claim — and any
//! `warm_full_reuse` promise — can never outrun the environment evidence
//! that backs it. The same engine drives the failure / recovery
//! [`CapsuleDrill`]s and the [`M5EnvGovernanceFixture`] corpus, so the
//! certification, the drills, and the fixtures cannot disagree about
//! when a claim must narrow.
//!
//! Four guardrails are frozen here:
//!
//! - **No happy-path green.** A profile is certified at its claimed
//!   maturity only when every required dimension is `current`. Stale or
//!   partial evidence narrows the claim; missing evidence withholds it.
//! - **Prebuilds are accelerators, not authorities.** When the source
//!   digest or prebuild fingerprint goes partial or stale, the engine
//!   narrows the warm-start posture: a `warm_full_reuse` claim drops to
//!   partial reuse or a cold build instead of presenting a stale warm
//!   snapshot as current truth.
//! - **One narrowing engine.** [`certify_capsule_outcome`] is the single
//!   source of truth for downgrade, shared by the rows, the drills, the
//!   fixtures, the [`EvidenceFreshnessRule`]s, and the
//!   [`WarmStartRule`]s. Release, support, docs, and help all read the
//!   resulting verdict and posture rather than re-deriving staleness.
//! - **No silent widening.** The certification only ever narrows; it
//!   never promotes a profile above its claimed maturity or warm-start
//!   posture, and a profile absent from the packet is uncertified, not
//!   implicitly green.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/env/m5-env-governance.schema.json`](../../../../schemas/env/m5-env-governance.schema.json)
//! - [`/docs/env/m5-env-governance.md`](../../../../docs/env/m5-env-governance.md)
//! - [`/artifacts/env/m5-env-proof-packet.json`](../../../../artifacts/env/m5-env-proof-packet.json)
//! - [`/artifacts/env/m5-env-governance.md`](../../../../artifacts/env/m5-env-governance.md)
//! - [`/fixtures/env/m5-env-governance/`](../../../../fixtures/env/m5-env-governance/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const M5_ENV_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const M5_ENV_GOVERNANCE_PACKET_RECORD_KIND: &str = "m5_env_governance_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const M5_ENV_GOVERNANCE_FIXTURE_RECORD_KIND: &str = "m5_env_governance_fixture_record";

/// Repo-relative schema ref.
pub const M5_ENV_GOVERNANCE_SCHEMA_REF: &str = "schemas/env/m5-env-governance.schema.json";

/// Repo-relative reviewer doc ref.
pub const M5_ENV_GOVERNANCE_DOC_REF: &str = "docs/env/m5-env-governance.md";

/// Repo-relative machine-readable proof packet.
pub const M5_ENV_GOVERNANCE_PACKET_REF: &str = "artifacts/env/m5-env-proof-packet.json";

/// Repo-relative reviewer certification summary.
pub const M5_ENV_GOVERNANCE_REPORT_REF: &str = "artifacts/env/m5-env-governance.md";

/// Repo-relative fixture directory.
pub const M5_ENV_GOVERNANCE_FIXTURE_DIR: &str = "fixtures/env/m5-env-governance";

/// Repo-relative fixture manifest.
pub const M5_ENV_GOVERNANCE_FIXTURE_MANIFEST_REF: &str =
    "fixtures/env/m5-env-governance/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// A claimed M5 environment surface profile under certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentProfile {
    /// A workspace template hydrated into a managed workspace.
    WorkspaceTemplate,
    /// A starter / seed project opened from the start center.
    Starter,
    /// A prebuilt environment snapshot reused on warm start.
    Prebuild,
    /// A devcontainer-defined environment.
    Devcontainer,
    /// A remote-host or container runtime materialization.
    RemoteContainer,
    /// A managed-workspace row's environment.
    ManagedWorkspace,
}

impl EnvironmentProfile {
    /// Every claimed profile in canonical order.
    pub const ALL: [Self; 6] = [
        Self::WorkspaceTemplate,
        Self::Starter,
        Self::Prebuild,
        Self::Devcontainer,
        Self::RemoteContainer,
        Self::ManagedWorkspace,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceTemplate => "workspace_template",
            Self::Starter => "starter",
            Self::Prebuild => "prebuild",
            Self::Devcontainer => "devcontainer",
            Self::RemoteContainer => "remote_container",
            Self::ManagedWorkspace => "managed_workspace",
        }
    }
}

/// One environment-capsule dimension a claimed profile must prove. The
/// seven dimensions are the exit-gate anchor: a profile may not present
/// an environment as trustworthy unless all seven are canonical and
/// testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapsuleDimension {
    /// The capsule is identified by a typed, versioned digest of its
    /// defining inputs (template, lockfiles, devcontainer/config), so
    /// its identity is inspectable and diffable.
    SourceDigest,
    /// The capsule declares its materialization target plan instead of
    /// inferring the target from side effects.
    TargetPlan,
    /// The capsule pins a deterministic toolchain plan (language /
    /// runtime versions and components).
    ToolchainPlan,
    /// Lifecycle hooks are declared and trust-gated rather than silently
    /// executed at hydration.
    TrustHooks,
    /// The capsule declares the service graph it materializes (services,
    /// ports, and dependencies).
    ServiceGraph,
    /// Prebuild reuse is validated against the source-digest fingerprint
    /// and invalidates rather than presenting a stale warm snapshot as
    /// authoritative.
    PrebuildFingerprint,
    /// Runtime materialization stays semantically aligned with the same
    /// capsule object across desktop, CLI, AI, support, and managed
    /// surfaces.
    MaterializationParity,
}

impl CapsuleDimension {
    /// Every required dimension in canonical order.
    pub const ALL: [Self; 7] = [
        Self::SourceDigest,
        Self::TargetPlan,
        Self::ToolchainPlan,
        Self::TrustHooks,
        Self::ServiceGraph,
        Self::PrebuildFingerprint,
        Self::MaterializationParity,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceDigest => "source_digest",
            Self::TargetPlan => "target_plan",
            Self::ToolchainPlan => "toolchain_plan",
            Self::TrustHooks => "trust_hooks",
            Self::ServiceGraph => "service_graph",
            Self::PrebuildFingerprint => "prebuild_fingerprint",
            Self::MaterializationParity => "materialization_parity",
        }
    }

    /// Whether degraded evidence on this dimension narrows the warm-start
    /// posture. Warm reuse is only trustworthy when the capsule's
    /// identity ([`CapsuleDimension::SourceDigest`]) and its cached
    /// artifact ([`CapsuleDimension::PrebuildFingerprint`]) are current,
    /// so those two dimensions — and only those — govern warm start.
    pub const fn governs_warm_start(self) -> bool {
        matches!(self, Self::SourceDigest | Self::PrebuildFingerprint)
    }
}

/// The maturity an environment claim can hold. Declaration order is the
/// narrowing order: [`ClaimMaturity::Stable`] is the strongest claim and
/// [`ClaimMaturity::Withdrawn`] the weakest, so narrowing always moves
/// toward a later variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimMaturity {
    /// Every required dimension is current; the claim holds in full.
    Stable,
    /// One or more dimensions are partial; the claim narrows.
    Beta,
    /// Evidence is stale enough that only a preview claim holds.
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

/// How much of an environment a warm start may reuse. Declaration order
/// is the narrowing order: [`WarmStartPosture::WarmFullReuse`] is the
/// strongest claim and [`WarmStartPosture::ColdBuild`] the most
/// conservative, so narrowing always moves toward a colder posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WarmStartPosture {
    /// The whole environment is reused from a current prebuild snapshot.
    WarmFullReuse,
    /// Only part of the environment is reused; the rest is rebuilt.
    WarmPartialReuse,
    /// No warm reuse is trustworthy; the environment is rebuilt cold.
    ColdBuild,
}

impl WarmStartPosture {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarmFullReuse => "warm_full_reuse",
            Self::WarmPartialReuse => "warm_partial_reuse",
            Self::ColdBuild => "cold_build",
        }
    }

    /// Narrowing severity. Higher is a colder, more conservative
    /// posture; the engine always takes the highest severity among the
    /// claimed posture and every triggered floor.
    pub const fn severity(self) -> u8 {
        match self {
            Self::WarmFullReuse => 0,
            Self::WarmPartialReuse => 1,
            Self::ColdBuild => 2,
        }
    }
}

/// The state of the evidence backing one dimension on one profile.
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
    /// The dimension does not apply to this profile.
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
    /// This is the heart of the maturity-narrowing engine: current and
    /// not-applicable evidence impose no floor; partial evidence caps the
    /// claim at beta; stale evidence caps it at preview; missing evidence
    /// withdraws the claim.
    pub const fn qualification_floor(self) -> Option<ClaimMaturity> {
        match self {
            Self::Current | Self::NotApplicable => None,
            Self::Partial => Some(ClaimMaturity::Beta),
            Self::Stale => Some(ClaimMaturity::Preview),
            Self::Missing => Some(ClaimMaturity::Withdrawn),
        }
    }

    /// The warm-start posture floor this evidence state forces, if any,
    /// when it lands on a warm-start-governing dimension.
    ///
    /// Partial source/prebuild evidence caps warm reuse at partial; stale
    /// or missing source/prebuild evidence forces a cold build, because a
    /// stale fingerprint can no longer prove the cached artifact matches
    /// the current source.
    pub const fn warm_start_floor(self) -> Option<WarmStartPosture> {
        match self {
            Self::Current | Self::NotApplicable => None,
            Self::Partial => Some(WarmStartPosture::WarmPartialReuse),
            Self::Stale | Self::Missing => Some(WarmStartPosture::ColdBuild),
        }
    }

    /// Returns true when the state names stale or missing evidence, the
    /// two states the guardrail treats as a freshness defect.
    pub const fn is_stale_or_missing(self) -> bool {
        matches!(self, Self::Stale | Self::Missing)
    }
}

/// The verdict the certification engine reaches for one capsule row.
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

/// How a profile's environment is materialized at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterializationClass {
    /// Materialized natively on the local host.
    LocalNative,
    /// Materialized in a local container.
    Container,
    /// Materialized on a remote host.
    RemoteHost,
    /// Materialized from a devcontainer definition.
    Devcontainer,
    /// Materialized in a managed cloud workspace.
    ManagedCloud,
}

impl MaterializationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalNative => "local_native",
            Self::Container => "container",
            Self::RemoteHost => "remote_host",
            Self::Devcontainer => "devcontainer",
            Self::ManagedCloud => "managed_cloud",
        }
    }
}

/// A publication channel that ingests the governance packet. The packet
/// as a whole must bind all four so release, support, docs, and help
/// tell one consistent environment story.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationChannel {
    /// Release / shiproom promotion surfaces.
    ReleaseShiproom,
    /// Metadata-first support export surfaces.
    SupportExport,
    /// Reviewer / product documentation surfaces.
    Docs,
    /// In-product help and why-this-environment inspectors.
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

/// The failure class an environment drill injects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillFailureClass {
    /// The source digest drifts from the materialized environment.
    SourceDigestDrift,
    /// The pinned toolchain plan ages past its freshness window.
    ToolchainPlanStale,
    /// A lifecycle hook would run without passing its trust gate.
    TrustHookUngated,
    /// The declared service graph covers only part of the services.
    ServiceGraphIncomplete,
    /// The prebuild fingerprint no longer matches the source digest.
    PrebuildFingerprintMismatch,
    /// Runtime materialization diverges from the capsule object across
    /// surfaces.
    MaterializationSkew,
}

impl DrillFailureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceDigestDrift => "source_digest_drift",
            Self::ToolchainPlanStale => "toolchain_plan_stale",
            Self::TrustHookUngated => "trust_hook_ungated",
            Self::ServiceGraphIncomplete => "service_graph_incomplete",
            Self::PrebuildFingerprintMismatch => "prebuild_fingerprint_mismatch",
            Self::MaterializationSkew => "materialization_skew",
        }
    }
}

/// One ordered phase of an environment drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillPhase {
    /// A failure is injected into a backing dimension.
    Inject,
    /// The certification observes the degraded evidence state.
    Observe,
    /// The claim and/or warm-start posture narrows under the failure.
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

/// One dimension's evidence on one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionEvidence {
    /// Capsule dimension being evidenced.
    pub dimension: CapsuleDimension,
    /// State of the evidence backing this dimension.
    pub evidence_state: EvidenceState,
    /// Upstream environment packets that prove this dimension.
    pub evidence_refs: Vec<String>,
    /// Review-safe rationale for the evidence.
    pub rationale: String,
}

/// The computed outcome of certifying one row against its evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleOutcome {
    /// The narrowest maturity the claim may hold.
    pub effective_maturity: ClaimMaturity,
    /// The verdict the engine reaches.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its claimed maturity.
    pub narrowed: bool,
    /// Stable tokens naming every dimension that forced maturity
    /// narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Dimensions whose evidence is stale or missing, in stable order.
    pub stale_or_missing_dimension_tokens: Vec<String>,
    /// The coldest warm-start posture the claim may hold.
    pub effective_warm_start_posture: WarmStartPosture,
    /// True when the warm-start posture narrowed below the claimed one.
    pub warm_start_downgraded: bool,
    /// Stable tokens naming every warm-start-governing dimension that
    /// forced a colder posture.
    pub warm_start_downgrade_tokens: Vec<String>,
}

/// Certifies one row's claim against its per-dimension evidence.
///
/// This is the canonical narrowing engine the whole packet, every drill,
/// every fixture, and release / support tooling share. The effective
/// maturity starts at the claimed maturity and is floored by every
/// degraded dimension; the warm-start posture starts at the claimed
/// posture and is floored by every degraded warm-start-governing
/// dimension; the narrowest (highest-severity) result wins on each axis.
/// A withdrawn maturity is [`RowVerdict::Withheld`]; any other maturity
/// below the claimed one is [`RowVerdict::Narrowed`]; otherwise the row
/// is [`RowVerdict::Certified`].
pub fn certify_capsule_outcome(
    claimed_maturity: ClaimMaturity,
    claimed_warm_start_posture: WarmStartPosture,
    dimensions: &[DimensionEvidence],
) -> CapsuleOutcome {
    let mut effective_maturity = claimed_maturity;
    let mut effective_warm_start = claimed_warm_start_posture;
    let mut narrow_reason_tokens = Vec::new();
    let mut warm_start_downgrade_tokens = Vec::new();
    let mut stale_or_missing = Vec::new();

    for evidence in dimensions {
        if let Some(floor) = evidence.evidence_state.qualification_floor() {
            if floor.severity() > effective_maturity.severity() {
                effective_maturity = floor;
            }
            narrow_reason_tokens.push(format!(
                "{}_{}",
                evidence.dimension.as_str(),
                evidence.evidence_state.as_str()
            ));
        }
        if evidence.dimension.governs_warm_start() {
            if let Some(floor) = evidence.evidence_state.warm_start_floor() {
                if floor.severity() > effective_warm_start.severity() {
                    effective_warm_start = floor;
                }
                warm_start_downgrade_tokens.push(format!(
                    "{}_{}",
                    evidence.dimension.as_str(),
                    evidence.evidence_state.as_str()
                ));
            }
        }
        if evidence.evidence_state.is_stale_or_missing() {
            stale_or_missing.push(evidence.dimension.as_str().to_owned());
        }
    }

    narrow_reason_tokens.sort();
    narrow_reason_tokens.dedup();
    warm_start_downgrade_tokens.sort();
    warm_start_downgrade_tokens.dedup();
    stale_or_missing.sort();
    stale_or_missing.dedup();

    let verdict = if effective_maturity == ClaimMaturity::Withdrawn {
        RowVerdict::Withheld
    } else if effective_maturity.severity() > claimed_maturity.severity() {
        RowVerdict::Narrowed
    } else {
        RowVerdict::Certified
    };

    CapsuleOutcome {
        effective_maturity,
        verdict,
        narrowed: verdict == RowVerdict::Narrowed,
        narrow_reason_tokens,
        stale_or_missing_dimension_tokens: stale_or_missing,
        effective_warm_start_posture: effective_warm_start,
        warm_start_downgraded: effective_warm_start.severity()
            > claimed_warm_start_posture.severity(),
        warm_start_downgrade_tokens,
    }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One capsule row: a claimed environment profile, its evidence, and the
/// engine outcome stamped onto it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// Review-safe label for the profile.
    pub profile_label: String,
    /// How this profile materializes its environment.
    pub materialization_class: MaterializationClass,
    /// Maturity claimed for the profile.
    pub claimed_maturity: ClaimMaturity,
    /// Warm-start posture claimed for the profile.
    pub claimed_warm_start_posture: WarmStartPosture,
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
    /// Stable tokens naming every dimension that forced maturity
    /// narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Dimensions whose evidence is stale or missing.
    pub stale_or_missing_dimension_tokens: Vec<String>,
    /// Effective warm-start posture after narrowing.
    pub effective_warm_start_posture: WarmStartPosture,
    /// True when the warm-start posture narrowed below the claimed one.
    pub warm_start_downgraded: bool,
    /// Stable tokens naming every warm-start-governing dimension that
    /// forced a colder posture.
    pub warm_start_downgrade_tokens: Vec<String>,
    /// Review-safe "why this environment" inspector line.
    pub why_this_environment: String,
    /// Upstream environment packets this row composes.
    pub supporting_evidence_refs: Vec<String>,
    /// Real consumer surfaces that ingest this row.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One automatic maturity-narrowing rule keyed by evidence state. The
/// floor is computed from [`EvidenceState::qualification_floor`], so the
/// rule set can never drift from the engine.
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

/// One automatic warm-start-narrowing rule keyed by evidence state on a
/// warm-start-governing dimension. The floor is computed from
/// [`EvidenceState::warm_start_floor`], so the rule set can never drift
/// from the engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarmStartRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Evidence state that triggers the rule.
    pub trigger_evidence_state: EvidenceState,
    /// Warm-start posture floor the rule imposes.
    pub warm_start_floor: WarmStartPosture,
    /// User-visible effect on warm start.
    pub effect: String,
    /// Review-safe rationale.
    pub rationale: String,
}

/// One ordered step inside an environment drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleDrillStep {
    /// Phase of this step.
    pub phase: DrillPhase,
    /// Maturity observed at this step.
    pub observed_maturity: ClaimMaturity,
    /// Warm-start posture observed at this step.
    pub observed_warm_start_posture: WarmStartPosture,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One failure / recovery drill walking a profile from an injected
/// failure through narrowing and back to recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapsuleDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Environment profile exercised by the drill.
    pub profile: EnvironmentProfile,
    /// Dimension whose evidence the drill degrades.
    pub exercised_dimension: CapsuleDimension,
    /// Failure class the drill injects.
    pub failure_class: DrillFailureClass,
    /// Evidence state the dimension degrades to.
    pub degraded_evidence_state: EvidenceState,
    /// Maturity claimed before the failure.
    pub claimed_maturity: ClaimMaturity,
    /// Warm-start posture claimed before the failure.
    pub claimed_warm_start_posture: WarmStartPosture,
    /// Verdict expected while the failure is active.
    pub expected_degraded_verdict: RowVerdict,
    /// Maturity expected while the failure is active.
    pub expected_degraded_maturity: ClaimMaturity,
    /// Warm-start posture expected while the failure is active.
    pub expected_degraded_warm_start_posture: WarmStartPosture,
    /// Verdict expected once the evidence is refreshed.
    pub recovers_to_verdict: RowVerdict,
    /// Ordered drill steps.
    pub steps: Vec<CapsuleDrillStep>,
    /// True when the drill proves the claim narrows under the failure.
    pub asserts_claim_narrows_under_failure: bool,
    /// True when the drill proves the claim recovers after refresh.
    pub asserts_recovers_after_refresh: bool,
    /// Short reviewer note.
    pub notes: String,
}

/// One binding proving a publication channel ingests this packet rather
/// than re-deriving environment truth.
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

/// Top-level packet governing environment-capsule truth on claimed M5
/// environment profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvGovernancePacket {
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
    /// Required capsule dimensions.
    pub certified_dimensions: Vec<CapsuleDimension>,
    /// Upstream environment packets this matrix composes.
    pub evidence_packet_refs: Vec<String>,
    /// Capsule rows, one per claimed profile.
    pub rows: Vec<CapsuleRow>,
    /// Automatic maturity-narrowing rules over evidence states.
    pub freshness_rules: Vec<EvidenceFreshnessRule>,
    /// Automatic warm-start-narrowing rules over evidence states.
    pub warm_start_rules: Vec<WarmStartRule>,
    /// Failure / recovery drills.
    pub drills: Vec<CapsuleDrill>,
    /// Publication-channel bindings.
    pub surface_bindings: Vec<SurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a profile and an observed evidence configuration
/// to the expected verdict and warm-start posture, proving the canonical
/// narrowing behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvGovernanceFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Environment profile under test.
    pub profile: EnvironmentProfile,
    /// Maturity claimed before narrowing.
    pub claimed_maturity: ClaimMaturity,
    /// Warm-start posture claimed before narrowing.
    pub claimed_warm_start_posture: WarmStartPosture,
    /// Observed per-dimension evidence.
    pub observed_dimensions: Vec<DimensionEvidence>,
    /// Expected verdict.
    pub expected_verdict: RowVerdict,
    /// Expected effective maturity.
    pub expected_effective_maturity: ClaimMaturity,
    /// Expected effective warm-start posture.
    pub expected_warm_start_posture: WarmStartPosture,
    /// Expected maturity-narrowing tokens.
    pub expected_narrow_reason_tokens: Vec<String>,
    /// Expected warm-start-downgrade tokens.
    pub expected_warm_start_downgrade_tokens: Vec<String>,
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
        writeln!(f, "m5 env governance validation failed")?;
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

const BUILD_IDENTITY_REF: &str = "artifacts/build/build_identity.json";
const ARCHETYPE_CONFIDENCE_REF: &str = "artifacts/workspace/archetype_confidence_rows.yaml";
const HOST_BOUNDARY_REF: &str = "artifacts/remote/host_boundary_matrix.yaml";
const STATE_ROOT_REF: &str = "artifacts/install/state_root_matrix.yaml";
const EXECUTION_SCOPE_REF: &str = "artifacts/runtime/execution_scope_matrix.yaml";
const AUTHORITY_CLASSES_REF: &str = "artifacts/runtime/authority_classes.yaml";
const MANAGED_LIFECYCLE_REF: &str = "artifacts/runtime/managed_workspace_lifecycle.yaml";
const WARM_START_CHOOSER_REF: &str = "artifacts/entry/warm_start_chooser_contract.md";
const ENV_STARTER_SUMMARY_REF: &str = "artifacts/entry/environment_starter_summary_contract.md";

fn evidence_packet_refs() -> Vec<String> {
    [
        BUILD_IDENTITY_REF,
        ARCHETYPE_CONFIDENCE_REF,
        HOST_BOUNDARY_REF,
        STATE_ROOT_REF,
        EXECUTION_SCOPE_REF,
        AUTHORITY_CLASSES_REF,
        MANAGED_LIFECYCLE_REF,
        WARM_START_CHOOSER_REF,
        ENV_STARTER_SUMMARY_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// The canonical evidence refs for one dimension when it is fully
/// current. Each dimension cites the environment packets that prove it,
/// so the matrix is anchored in the checked-in artifacts.
fn dimension_evidence_refs(dimension: CapsuleDimension) -> Vec<&'static str> {
    match dimension {
        CapsuleDimension::SourceDigest => vec![BUILD_IDENTITY_REF, ARCHETYPE_CONFIDENCE_REF],
        CapsuleDimension::TargetPlan => vec![HOST_BOUNDARY_REF, STATE_ROOT_REF],
        CapsuleDimension::ToolchainPlan => vec![STATE_ROOT_REF, BUILD_IDENTITY_REF],
        CapsuleDimension::TrustHooks => vec![EXECUTION_SCOPE_REF, AUTHORITY_CLASSES_REF],
        CapsuleDimension::ServiceGraph => vec![MANAGED_LIFECYCLE_REF, HOST_BOUNDARY_REF],
        CapsuleDimension::PrebuildFingerprint => vec![WARM_START_CHOOSER_REF, BUILD_IDENTITY_REF],
        CapsuleDimension::MaterializationParity => {
            vec![ENV_STARTER_SUMMARY_REF, MANAGED_LIFECYCLE_REF]
        }
    }
}

fn dimension_rationale(dimension: CapsuleDimension) -> &'static str {
    match dimension {
        CapsuleDimension::SourceDigest => {
            "The capsule is identified by a typed, versioned digest of its defining inputs, so its identity is inspectable, diffable, and mirrorable rather than implied by side effects."
        }
        CapsuleDimension::TargetPlan => {
            "The capsule declares its materialization target plan — local, container, remote, devcontainer, or managed — instead of inferring the target from whatever happened to run."
        }
        CapsuleDimension::ToolchainPlan => {
            "The capsule pins a deterministic toolchain plan of language and runtime versions and components, so the same capsule resolves the same toolchain across surfaces."
        }
        CapsuleDimension::TrustHooks => {
            "Lifecycle hooks are declared and trust-gated against the execution-scope and authority contracts, never silently executed during template hydration or warm start."
        }
        CapsuleDimension::ServiceGraph => {
            "The capsule declares the service graph it materializes — services, ports, and dependencies — so a partial graph is labeled partial rather than presented as the whole environment."
        }
        CapsuleDimension::PrebuildFingerprint => {
            "Prebuild reuse is validated against the source-digest fingerprint and invalidates when the fingerprint drifts, so a prebuild stays an accelerator rather than an authority."
        }
        CapsuleDimension::MaterializationParity => {
            "Runtime materialization stays semantically aligned with the same capsule object consumed by desktop, CLI, AI, support, and managed rows instead of forking a parallel model."
        }
    }
}

/// Builds the seven fully-current dimensions for a healthy row.
fn current_dimensions() -> Vec<DimensionEvidence> {
    CapsuleDimension::ALL
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

fn supporting_evidence_refs(dimensions: &[DimensionEvidence]) -> Vec<String> {
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
    profile: EnvironmentProfile,
    profile_label: &str,
    materialization_class: MaterializationClass,
    claimed_maturity: ClaimMaturity,
    claimed_warm_start_posture: WarmStartPosture,
    backing_surface_classes: &[&str],
    why_this_environment: &str,
    consumer_refs: &[&str],
    notes: &str,
) -> CapsuleRow {
    let dimensions = current_dimensions();
    let outcome =
        certify_capsule_outcome(claimed_maturity, claimed_warm_start_posture, &dimensions);
    let supporting_evidence_refs = supporting_evidence_refs(&dimensions);
    CapsuleRow {
        row_id: row_id.to_owned(),
        profile,
        profile_label: profile_label.to_owned(),
        materialization_class,
        claimed_maturity,
        claimed_warm_start_posture,
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
        effective_warm_start_posture: outcome.effective_warm_start_posture,
        warm_start_downgraded: outcome.warm_start_downgraded,
        warm_start_downgrade_tokens: outcome.warm_start_downgrade_tokens,
        why_this_environment: why_this_environment.to_owned(),
        supporting_evidence_refs,
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
            .expect("freshness rules only encode triggers that impose a maturity floor"),
        effect: effect.to_owned(),
        rationale: rationale.to_owned(),
    }
}

fn warm_start_rule(
    rule_id: &str,
    trigger: EvidenceState,
    effect: &str,
    rationale: &str,
) -> WarmStartRule {
    WarmStartRule {
        rule_id: rule_id.to_owned(),
        trigger_evidence_state: trigger,
        warm_start_floor: trigger
            .warm_start_floor()
            .expect("warm-start rules only encode triggers that impose a posture floor"),
        effect: effect.to_owned(),
        rationale: rationale.to_owned(),
    }
}

fn step(
    phase: DrillPhase,
    observed_maturity: ClaimMaturity,
    observed_warm_start_posture: WarmStartPosture,
    narration: &str,
) -> CapsuleDrillStep {
    CapsuleDrillStep {
        phase,
        observed_maturity,
        observed_warm_start_posture,
        narration: narration.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn drill(
    drill_id: &str,
    title: &str,
    profile: EnvironmentProfile,
    exercised_dimension: CapsuleDimension,
    failure_class: DrillFailureClass,
    degraded_evidence_state: EvidenceState,
    claimed_maturity: ClaimMaturity,
    claimed_warm_start_posture: WarmStartPosture,
    steps: Vec<CapsuleDrillStep>,
    notes: &str,
) -> CapsuleDrill {
    // The degraded posture is computed from the same engine the rows
    // use, so a drill can never disagree with the certification.
    let mut degraded = current_dimensions();
    for evidence in &mut degraded {
        if evidence.dimension == exercised_dimension {
            evidence.evidence_state = degraded_evidence_state;
        }
    }
    let degraded_outcome =
        certify_capsule_outcome(claimed_maturity, claimed_warm_start_posture, &degraded);
    CapsuleDrill {
        drill_id: drill_id.to_owned(),
        title: title.to_owned(),
        profile,
        exercised_dimension,
        failure_class,
        degraded_evidence_state,
        claimed_maturity,
        claimed_warm_start_posture,
        expected_degraded_verdict: degraded_outcome.verdict,
        expected_degraded_maturity: degraded_outcome.effective_maturity,
        expected_degraded_warm_start_posture: degraded_outcome.effective_warm_start_posture,
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

const PACKET_ID: &str = "env.m5_env_governance.v1";

const REQUIRED_VERBATIM_FIELDS: [&str; 7] = [
    "row_id",
    "profile",
    "claimed_maturity",
    "effective_maturity",
    "verdict",
    "effective_warm_start_posture",
    "narrow_reason_tokens",
];

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in environment-governance packet this lane
/// freezes.
pub fn seeded_m5_env_governance_packet() -> M5EnvGovernancePacket {
    let rows = vec![
        row(
            "env.capsule.workspace_template",
            EnvironmentProfile::WorkspaceTemplate,
            "Workspace template hydration",
            MaterializationClass::LocalNative,
            ClaimMaturity::Stable,
            WarmStartPosture::ColdBuild,
            &["workspace_template", "scaffold_planner"],
            "This environment is the template's source digest hydrated locally: the target plan, toolchain plan, trust-gated hooks, and service graph all derive from the template capsule rather than from whatever the scaffold happened to run.",
            &[
                "crates/aureline-templates/src/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/mod.rs",
                "crates/aureline-scaffold/src/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/mod.rs",
            ],
            "A workspace template hydrates a fresh capsule and cold-builds; it never claims warm reuse, and its lifecycle hooks stay trust-gated.",
        ),
        row(
            "env.capsule.starter",
            EnvironmentProfile::Starter,
            "Starter project entry",
            MaterializationClass::LocalNative,
            ClaimMaturity::Stable,
            WarmStartPosture::WarmPartialReuse,
            &["project_starter", "start_center"],
            "This environment is a starter capsule opened from the start center: its source digest pins the seed, and a current prebuild fingerprint lets it warm-reuse cached dependencies while the rest is rebuilt.",
            &[
                "crates/aureline-workspace/src/entry/mod.rs",
                "crates/aureline-scaffold/src/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/mod.rs",
            ],
            "A starter warm-reuses cached dependencies only while the source digest and prebuild fingerprint are current; otherwise it narrows the reuse posture.",
        ),
        row(
            "env.capsule.prebuild",
            EnvironmentProfile::Prebuild,
            "Prebuild snapshot reuse",
            MaterializationClass::Container,
            ClaimMaturity::Beta,
            WarmStartPosture::WarmFullReuse,
            &["prebuild_snapshot", "warm_start"],
            "This environment is a prebuilt snapshot whose fingerprint currently matches the source digest, so the whole capsule is warm-reused; a fingerprint mismatch invalidates the snapshot rather than serving stale truth.",
            &[
                "crates/aureline-runtime/src/capsule_resolver/mod.rs",
                "crates/aureline-runtime/src/env_inspect/mod.rs",
            ],
            "A prebuild claims full warm reuse only while its fingerprint matches the source digest; a stale fingerprint narrows the maturity and forces a cold build.",
        ),
        row(
            "env.capsule.devcontainer",
            EnvironmentProfile::Devcontainer,
            "Devcontainer environment",
            MaterializationClass::Devcontainer,
            ClaimMaturity::Beta,
            WarmStartPosture::WarmPartialReuse,
            &["devcontainer_definition"],
            "This environment materializes a devcontainer definition: the capsule declares its target plan, toolchain plan, trust-gated hooks, and service graph from the devcontainer rather than inferring them.",
            &[
                "crates/aureline-runtime/src/execution_context/mod.rs",
                "crates/aureline-runtime/src/env_inspect/mod.rs",
            ],
            "A devcontainer warm-reuses a partial layer cache; an incomplete service graph or stale toolchain narrows the claim rather than implying the whole environment.",
        ),
        row(
            "env.capsule.remote_container",
            EnvironmentProfile::RemoteContainer,
            "Remote container runtime",
            MaterializationClass::RemoteHost,
            ClaimMaturity::Beta,
            WarmStartPosture::WarmPartialReuse,
            &["remote_container_runtime"],
            "This environment is materialized on a remote host within its declared boundary: the capsule's target plan and service graph match the host-boundary matrix, and materialization parity is checked against the same capsule object.",
            &[
                "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs",
                "crates/aureline-runtime/src/capsule_resolver/mod.rs",
            ],
            "A remote container narrows when its toolchain plan goes stale or its materialization diverges from the capsule object across surfaces.",
        ),
        row(
            "env.capsule.managed_workspace",
            EnvironmentProfile::ManagedWorkspace,
            "Managed-workspace environment",
            MaterializationClass::ManagedCloud,
            ClaimMaturity::Beta,
            WarmStartPosture::WarmFullReuse,
            &["managed_workspace_row"],
            "This environment is a managed-workspace row materialized in the cloud: its capsule, prebuild fingerprint, and service graph are mirrored so support and release read the captured environment claim, not live truth.",
            &[
                "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs",
                "crates/aureline-support/src/bundle/mod.rs",
            ],
            "A managed workspace claims full warm reuse from a current prebuild; a materialization skew narrows the claim, and a missing fingerprint forces a cold build.",
        ),
    ];

    let freshness_rules = vec![
        freshness_rule(
            "freshness.partial_narrows_to_beta",
            EvidenceState::Partial,
            "A claimed profile with partial evidence on any required dimension narrows to at most a beta claim.",
            "Partial environment evidence proves only part of the claimed capsule, so the profile may not present a stable environment guarantee.",
        ),
        freshness_rule(
            "freshness.stale_narrows_to_preview",
            EvidenceState::Stale,
            "A claimed profile with stale evidence on any required dimension narrows to at most a preview claim.",
            "Stale environment evidence may no longer reflect the current source or platform truth, so the profile drops below beta until the evidence is refreshed.",
        ),
        freshness_rule(
            "freshness.missing_withholds_claim",
            EvidenceState::Missing,
            "A claimed profile missing evidence on any required dimension is withheld; promotion fails until the dimension is proven.",
            "A required environment dimension with no backing evidence cannot be proven, so the profile may not be promoted at its claimed maturity.",
        ),
    ];

    let warm_start_rules = vec![
        warm_start_rule(
            "warm_start.partial_narrows_to_partial_reuse",
            EvidenceState::Partial,
            "Partial source-digest or prebuild-fingerprint evidence narrows the warm-start posture to at most partial reuse.",
            "A partially proven fingerprint cannot prove the whole cached environment matches the current source, so only part of the environment may be reused.",
        ),
        warm_start_rule(
            "warm_start.stale_forces_cold_build",
            EvidenceState::Stale,
            "Stale source-digest or prebuild-fingerprint evidence forces a cold build.",
            "A stale fingerprint can no longer prove the cached artifact matches the current source, so the environment must be rebuilt rather than served warm.",
        ),
        warm_start_rule(
            "warm_start.missing_forces_cold_build",
            EvidenceState::Missing,
            "Missing source-digest or prebuild-fingerprint evidence forces a cold build.",
            "Without a source digest or fingerprint the environment cannot be identified for reuse, so warm start is unavailable and the environment is rebuilt cold.",
        ),
    ];

    let drills = vec![
        drill(
            "drill.env_governance.workspace_template_trust_hook_missing",
            "Template hydration is withheld when trust-hook evidence is missing",
            EnvironmentProfile::WorkspaceTemplate,
            CapsuleDimension::TrustHooks,
            DrillFailureClass::TrustHookUngated,
            EvidenceState::Missing,
            ClaimMaturity::Stable,
            WarmStartPosture::ColdBuild,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Stable,
                    WarmStartPosture::ColdBuild,
                    "The template declares a lifecycle hook with no trust-gate evidence binding it to the execution-scope and authority contracts.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Stable,
                    WarmStartPosture::ColdBuild,
                    "Trust-hooks evidence is observed missing for the template profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Withdrawn,
                    WarmStartPosture::ColdBuild,
                    "The certification withholds the template claim; hydration is blocked until the hook is trust-gated rather than silently executed.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Withdrawn,
                    WarmStartPosture::ColdBuild,
                    "The hook is bound to its trust gate and the evidence is restored.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Stable,
                    WarmStartPosture::ColdBuild,
                    "Trust-hooks evidence returns current; the claim recovers to its stable maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Stable,
                    WarmStartPosture::ColdBuild,
                    "The recovered posture matches the certification engine for a fully current template row.",
                ),
            ],
            "Missing trust-hook evidence withholds the template claim rather than letting an ungated hook run during hydration.",
        ),
        drill(
            "drill.env_governance.starter_source_digest_partial",
            "Starter narrows to beta on partial source-digest coverage",
            EnvironmentProfile::Starter,
            CapsuleDimension::SourceDigest,
            DrillFailureClass::SourceDigestDrift,
            EvidenceState::Partial,
            ClaimMaturity::Stable,
            WarmStartPosture::WarmPartialReuse,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Stable,
                    WarmStartPosture::WarmPartialReuse,
                    "The starter's source digest covers only part of its defining inputs after a seed input changes under it.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Stable,
                    WarmStartPosture::WarmPartialReuse,
                    "Source-digest evidence is observed partial for the starter profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The certified claim narrows to beta; the starter labels the capsule as partially identified and keeps warm reuse at partial.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The source digest is recomputed across the remaining inputs.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Stable,
                    WarmStartPosture::WarmPartialReuse,
                    "Source-digest evidence returns current; the claim recovers to stable.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Stable,
                    WarmStartPosture::WarmPartialReuse,
                    "The recovered posture matches the certification engine for a fully current starter row.",
                ),
            ],
            "Partial source-digest coverage narrows the starter claim to beta without withholding it.",
        ),
        drill(
            "drill.env_governance.prebuild_fingerprint_stale",
            "Prebuild narrows to preview and cold-builds when its fingerprint goes stale",
            EnvironmentProfile::Prebuild,
            CapsuleDimension::PrebuildFingerprint,
            DrillFailureClass::PrebuildFingerprintMismatch,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            WarmStartPosture::WarmFullReuse,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "The source digest advances while the prebuild snapshot stays pinned, so its fingerprint trails the current source.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "Prebuild-fingerprint evidence is observed stale for the prebuild profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    WarmStartPosture::ColdBuild,
                    "The certified claim narrows to preview and the warm-start posture drops to a cold build; the stale snapshot is invalidated instead of being served as current truth.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    WarmStartPosture::ColdBuild,
                    "The prebuild is rebuilt and re-fingerprinted against the current source digest.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "Prebuild-fingerprint evidence returns current; the claim recovers to beta with full warm reuse.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "The recovered posture matches the certification engine for a fully current prebuild row.",
                ),
            ],
            "A stale prebuild fingerprint narrows the maturity to preview and forces a cold build, never letting an approximate warm snapshot imply trustworthy reuse.",
        ),
        drill(
            "drill.env_governance.devcontainer_service_graph_stale",
            "Devcontainer narrows to preview when its service graph ages out",
            EnvironmentProfile::Devcontainer,
            CapsuleDimension::ServiceGraph,
            DrillFailureClass::ServiceGraphIncomplete,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            WarmStartPosture::WarmPartialReuse,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The devcontainer's declared service graph ages past its freshness window after a service definition changes.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "Service-graph evidence is observed stale for the devcontainer profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    WarmStartPosture::WarmPartialReuse,
                    "The certified claim narrows to preview; the devcontainer labels its service graph stale rather than implying the whole environment.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    WarmStartPosture::WarmPartialReuse,
                    "The service graph is recaptured from the devcontainer definition.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "Service-graph evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The recovered posture matches the certification engine for a fully current devcontainer row.",
                ),
            ],
            "Stale service-graph evidence narrows the devcontainer claim below beta even though it worked once on a happy-path capture.",
        ),
        drill(
            "drill.env_governance.remote_container_toolchain_stale",
            "Remote container narrows to preview when its toolchain plan goes stale",
            EnvironmentProfile::RemoteContainer,
            CapsuleDimension::ToolchainPlan,
            DrillFailureClass::ToolchainPlanStale,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            WarmStartPosture::WarmPartialReuse,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The pinned toolchain plan ages past its freshness window after the remote host's base image rolls.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "Toolchain-plan evidence is observed stale for the remote-container profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    WarmStartPosture::WarmPartialReuse,
                    "The certified claim narrows to preview until the toolchain plan is re-pinned against the current host.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    WarmStartPosture::WarmPartialReuse,
                    "The toolchain plan is re-pinned against the rolled base image.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "Toolchain-plan evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The recovered posture matches the certification engine for a fully current remote-container row.",
                ),
            ],
            "A stale toolchain plan narrows the remote-container claim to preview until it is re-pinned against the current host.",
        ),
        drill(
            "drill.env_governance.managed_workspace_materialization_skew",
            "Managed workspace narrows to preview on materialization skew",
            EnvironmentProfile::ManagedWorkspace,
            CapsuleDimension::MaterializationParity,
            DrillFailureClass::MaterializationSkew,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            WarmStartPosture::WarmFullReuse,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "The managed-workspace materialization diverges from the capsule object the desktop and support surfaces read.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "Materialization-parity evidence is observed stale for the managed-workspace profile.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    WarmStartPosture::WarmFullReuse,
                    "The certified claim narrows to preview; the managed row is labeled skewed rather than presented as aligned live truth.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    WarmStartPosture::WarmFullReuse,
                    "The materialization is re-aligned with the capsule object and the parity evidence is recaptured.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "Materialization-parity evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "The recovered posture matches the certification engine for a fully current managed-workspace row.",
                ),
            ],
            "Materialization skew narrows the managed-workspace claim to preview; warm reuse stays unchanged because the prebuild fingerprint is still current.",
        ),
    ];

    let surface_bindings = vec![
        binding(
            PublicationChannel::ReleaseShiproom,
            "artifacts/release/shiproom_dashboard.json",
            "The shiproom dashboard reads the per-row verdict, effective maturity, and warm-start posture and holds promotion for any narrowed or withheld release-scope environment profile.",
        ),
        binding(
            PublicationChannel::SupportExport,
            "crates/aureline-support/src/bundle/mod.rs",
            "The metadata-first support bundle re-exports the per-row verdict, narrowing tokens, warm-start posture, and stale-or-missing dimensions without raw paths, credentials, or provider payloads.",
        ),
        binding(
            PublicationChannel::Docs,
            "docs/env/m5-env-governance.md",
            "The reviewer documentation quotes the certified dimensions, freshness and warm-start rules, and per-row verdicts directly from the packet.",
        ),
        binding(
            PublicationChannel::Help,
            "crates/aureline-runtime/src/env_inspect/mod.rs",
            "The in-product why-this-environment inspector reuses the same verdict and warm-start vocabulary so help never tells a greener story than the packet.",
        ),
    ];

    M5EnvGovernancePacket {
        record_kind: M5_ENV_GOVERNANCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_ENV_GOVERNANCE_SCHEMA_VERSION,
        packet_id: PACKET_ID.to_owned(),
        title: "Environment-capsule, template, prebuild-fingerprint, and runtime-materialization governance for claimed M5 environment profiles"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: M5_ENV_GOVERNANCE_DOC_REF.to_owned(),
            schema_ref: M5_ENV_GOVERNANCE_SCHEMA_REF.to_owned(),
            packet_ref: M5_ENV_GOVERNANCE_PACKET_REF.to_owned(),
            report_ref: M5_ENV_GOVERNANCE_REPORT_REF.to_owned(),
            fixture_manifest_ref: M5_ENV_GOVERNANCE_FIXTURE_MANIFEST_REF.to_owned(),
        },
        certified_dimensions: CapsuleDimension::ALL.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        rows,
        freshness_rules,
        warm_start_rules,
        drills,
        surface_bindings,
        invariants: vec![
            "Each claimed M5 environment profile is certified only when every required capsule dimension — source digest, target plan, toolchain plan, trust hooks, service graph, prebuild fingerprint, and materialization parity — is proven current.".to_owned(),
            "One narrowing engine folds per-dimension evidence into a verdict and a warm-start posture: partial evidence narrows to beta, stale evidence narrows to preview, missing evidence withholds the claim, and stale or partial source/prebuild evidence narrows warm reuse.".to_owned(),
            "Prebuilds are accelerators rather than authorities: a warm-full-reuse claim drops to partial reuse or a cold build whenever the source digest or prebuild fingerprint outruns current truth.".to_owned(),
            "Templates cannot invent a parallel execution model and lifecycle hooks stay trust-gated; a profile absent from the packet is uncertified rather than implicitly green, and the certification only narrows, never widens.".to_owned(),
            "Release, support, docs, and help all read the same per-row verdict, warm-start posture, and narrowing tokens instead of re-deriving environment staleness.".to_owned(),
            "Failure and recovery drills exercise each profile through narrowing and back, computed from the same engine so the certification, drills, and fixtures cannot disagree.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture corpus this lane freezes.
pub fn seeded_m5_env_governance_fixtures() -> Vec<M5EnvGovernanceFixture> {
    let mut fixtures = Vec::new();

    // One healthy fixture per profile, pinning the certified verdict.
    for profile in EnvironmentProfile::ALL {
        let (claimed_maturity, claimed_warm_start) = claimed_posture_for(profile);
        fixtures.push(fixture(
            &format!("fixture.m5_env_governance.{}_certified", profile.as_str()),
            profile,
            claimed_maturity,
            claimed_warm_start,
            current_dimensions(),
            consumer_ref_for(profile),
            "A fully current profile certifies at its claimed maturity and warm-start posture with no narrowing tokens.",
        ));
    }

    // Degraded fixtures exercising every floor and verdict.
    fixtures.push(fixture(
        "fixture.m5_env_governance.prebuild_fingerprint_stale",
        EnvironmentProfile::Prebuild,
        ClaimMaturity::Beta,
        WarmStartPosture::WarmFullReuse,
        degraded_dimensions(CapsuleDimension::PrebuildFingerprint, EvidenceState::Stale),
        consumer_ref_for(EnvironmentProfile::Prebuild),
        "A stale prebuild fingerprint narrows the beta prebuild claim to a preview verdict and forces a cold build instead of full warm reuse.",
    ));
    fixtures.push(fixture(
        "fixture.m5_env_governance.starter_source_digest_partial",
        EnvironmentProfile::Starter,
        ClaimMaturity::Stable,
        WarmStartPosture::WarmPartialReuse,
        degraded_dimensions(CapsuleDimension::SourceDigest, EvidenceState::Partial),
        consumer_ref_for(EnvironmentProfile::Starter),
        "Partial source-digest evidence narrows the stable starter claim to a beta verdict while warm reuse stays at partial.",
    ));
    fixtures.push(fixture(
        "fixture.m5_env_governance.workspace_template_trust_hook_missing",
        EnvironmentProfile::WorkspaceTemplate,
        ClaimMaturity::Stable,
        WarmStartPosture::ColdBuild,
        degraded_dimensions(CapsuleDimension::TrustHooks, EvidenceState::Missing),
        consumer_ref_for(EnvironmentProfile::WorkspaceTemplate),
        "Missing trust-hook evidence withholds the template claim entirely.",
    ));
    fixtures.push(fixture(
        "fixture.m5_env_governance.managed_workspace_materialization_stale",
        EnvironmentProfile::ManagedWorkspace,
        ClaimMaturity::Beta,
        WarmStartPosture::WarmFullReuse,
        degraded_dimensions(CapsuleDimension::MaterializationParity, EvidenceState::Stale),
        consumer_ref_for(EnvironmentProfile::ManagedWorkspace),
        "Stale materialization-parity evidence narrows the managed-workspace claim to preview while warm reuse stays full because the fingerprint is current.",
    ));

    fixtures
}

fn claimed_posture_for(profile: EnvironmentProfile) -> (ClaimMaturity, WarmStartPosture) {
    match profile {
        EnvironmentProfile::WorkspaceTemplate => {
            (ClaimMaturity::Stable, WarmStartPosture::ColdBuild)
        }
        EnvironmentProfile::Starter => (ClaimMaturity::Stable, WarmStartPosture::WarmPartialReuse),
        EnvironmentProfile::Prebuild => (ClaimMaturity::Beta, WarmStartPosture::WarmFullReuse),
        EnvironmentProfile::Devcontainer => {
            (ClaimMaturity::Beta, WarmStartPosture::WarmPartialReuse)
        }
        EnvironmentProfile::RemoteContainer => {
            (ClaimMaturity::Beta, WarmStartPosture::WarmPartialReuse)
        }
        EnvironmentProfile::ManagedWorkspace => {
            (ClaimMaturity::Beta, WarmStartPosture::WarmFullReuse)
        }
    }
}

fn consumer_ref_for(profile: EnvironmentProfile) -> &'static str {
    match profile {
        EnvironmentProfile::WorkspaceTemplate => {
            "crates/aureline-templates/src/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/mod.rs"
        }
        EnvironmentProfile::Starter => "crates/aureline-workspace/src/entry/mod.rs",
        EnvironmentProfile::Prebuild => "crates/aureline-runtime/src/capsule_resolver/mod.rs",
        EnvironmentProfile::Devcontainer => "crates/aureline-runtime/src/execution_context/mod.rs",
        EnvironmentProfile::RemoteContainer => {
            "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs"
        }
        EnvironmentProfile::ManagedWorkspace => "crates/aureline-support/src/bundle/mod.rs",
    }
}

fn degraded_dimensions(
    dimension: CapsuleDimension,
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

#[allow(clippy::too_many_arguments)]
fn fixture(
    fixture_id: &str,
    profile: EnvironmentProfile,
    claimed_maturity: ClaimMaturity,
    claimed_warm_start_posture: WarmStartPosture,
    observed_dimensions: Vec<DimensionEvidence>,
    consumer_ref: &str,
    notes: &str,
) -> M5EnvGovernanceFixture {
    let outcome = certify_capsule_outcome(
        claimed_maturity,
        claimed_warm_start_posture,
        &observed_dimensions,
    );
    M5EnvGovernanceFixture {
        record_kind: M5_ENV_GOVERNANCE_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: M5_ENV_GOVERNANCE_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        profile,
        claimed_maturity,
        claimed_warm_start_posture,
        observed_dimensions,
        expected_verdict: outcome.verdict,
        expected_effective_maturity: outcome.effective_maturity,
        expected_warm_start_posture: outcome.effective_warm_start_posture,
        expected_narrow_reason_tokens: outcome.narrow_reason_tokens,
        expected_warm_start_downgrade_tokens: outcome.warm_start_downgrade_tokens,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in packet contract.
pub fn validate_m5_env_governance_packet(
    packet: &M5EnvGovernancePacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != M5_ENV_GOVERNANCE_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != M5_ENV_GOVERNANCE_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.doc_ref != M5_ENV_GOVERNANCE_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != M5_ENV_GOVERNANCE_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != M5_ENV_GOVERNANCE_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != M5_ENV_GOVERNANCE_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != M5_ENV_GOVERNANCE_FIXTURE_MANIFEST_REF {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.certified_dimensions != CapsuleDimension::ALL.to_vec() {
        report.push(
            "packet.certified_dimensions",
            "packet must certify every required dimension in canonical order",
        );
    }
    if packet.evidence_packet_refs.is_empty() {
        report.push(
            "packet.evidence_packet_refs",
            "packet must cite the upstream environment evidence packets",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    let mut covered_profiles = BTreeSet::new();
    for capsule_row in &packet.rows {
        if !covered_profiles.insert(capsule_row.profile) {
            report.push(
                "row.profile_unique",
                format!("duplicate profile {}", capsule_row.profile.as_str()),
            );
        }
        validate_row(&mut report, capsule_row);
    }
    for required in EnvironmentProfile::ALL {
        if !covered_profiles.contains(&required) {
            report.push(
                "packet.covered_profile",
                format!("packet must certify profile {}", required.as_str()),
            );
        }
    }

    validate_freshness_rules(&mut report, packet);
    validate_warm_start_rules(&mut report, packet);
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
    for required in CapsuleDimension::ALL {
        if !seen.contains(&required) {
            report.push(
                "dimension.coverage",
                format!("{owner} must evidence dimension {}", required.as_str()),
            );
        }
    }
}

fn validate_row(report: &mut ValidationReport, capsule_row: &CapsuleRow) {
    if capsule_row.row_id.trim().is_empty() {
        report.push("row.id", "row must carry a stable id");
    }
    if capsule_row.profile_label.trim().is_empty() {
        report.push(
            "row.profile_label",
            format!("row {} must carry a profile label", capsule_row.row_id),
        );
    }
    if capsule_row.backing_surface_classes.is_empty() {
        report.push(
            "row.backing_surface_classes",
            format!(
                "row {} must name its backing surface classes",
                capsule_row.row_id
            ),
        );
    }
    if capsule_row.why_this_environment.trim().is_empty() {
        report.push(
            "row.why_this_environment",
            format!(
                "row {} must carry a why-this-environment inspector line",
                capsule_row.row_id
            ),
        );
    }
    if capsule_row.consumer_refs.is_empty() {
        report.push(
            "row.consumer_refs",
            format!(
                "row {} must cite at least one consumer ref",
                capsule_row.row_id
            ),
        );
    }
    if capsule_row.notes.trim().is_empty() {
        report.push(
            "row.notes",
            format!("row {} must carry a reviewer note", capsule_row.row_id),
        );
    }

    validate_dimensions(
        report,
        &format!("row {}", capsule_row.row_id),
        &capsule_row.dimensions,
    );

    // The stamped outcome must equal what the engine computes.
    let outcome = certify_capsule_outcome(
        capsule_row.claimed_maturity,
        capsule_row.claimed_warm_start_posture,
        &capsule_row.dimensions,
    );
    if capsule_row.effective_maturity != outcome.effective_maturity {
        report.push(
            "row.effective_maturity",
            format!(
                "row {} effective_maturity {} disagrees with the engine ({})",
                capsule_row.row_id,
                capsule_row.effective_maturity.as_str(),
                outcome.effective_maturity.as_str()
            ),
        );
    }
    if capsule_row.verdict != outcome.verdict {
        report.push(
            "row.verdict",
            format!(
                "row {} verdict {} disagrees with the engine ({})",
                capsule_row.row_id,
                capsule_row.verdict.as_str(),
                outcome.verdict.as_str()
            ),
        );
    }
    if capsule_row.narrowed != outcome.narrowed {
        report.push(
            "row.narrowed",
            format!(
                "row {} narrowed flag disagrees with the engine",
                capsule_row.row_id
            ),
        );
    }
    if capsule_row.narrow_reason_tokens != outcome.narrow_reason_tokens {
        report.push(
            "row.narrow_reason_tokens",
            format!(
                "row {} narrow_reason_tokens disagree with the engine",
                capsule_row.row_id
            ),
        );
    }
    if capsule_row.stale_or_missing_dimension_tokens != outcome.stale_or_missing_dimension_tokens {
        report.push(
            "row.stale_or_missing_dimension_tokens",
            format!(
                "row {} stale_or_missing_dimension_tokens disagree with the engine",
                capsule_row.row_id
            ),
        );
    }
    if capsule_row.effective_warm_start_posture != outcome.effective_warm_start_posture {
        report.push(
            "row.effective_warm_start_posture",
            format!(
                "row {} effective_warm_start_posture {} disagrees with the engine ({})",
                capsule_row.row_id,
                capsule_row.effective_warm_start_posture.as_str(),
                outcome.effective_warm_start_posture.as_str()
            ),
        );
    }
    if capsule_row.warm_start_downgraded != outcome.warm_start_downgraded {
        report.push(
            "row.warm_start_downgraded",
            format!(
                "row {} warm_start_downgraded flag disagrees with the engine",
                capsule_row.row_id
            ),
        );
    }
    if capsule_row.warm_start_downgrade_tokens != outcome.warm_start_downgrade_tokens {
        report.push(
            "row.warm_start_downgrade_tokens",
            format!(
                "row {} warm_start_downgrade_tokens disagree with the engine",
                capsule_row.row_id
            ),
        );
    }

    let expected_support = supporting_evidence_refs(&capsule_row.dimensions);
    if capsule_row.supporting_evidence_refs != expected_support {
        report.push(
            "row.supporting_evidence_refs",
            format!(
                "row {} supporting_evidence_refs must equal the union of its dimension evidence refs",
                capsule_row.row_id
            ),
        );
    }
}

fn validate_freshness_rules(report: &mut ValidationReport, packet: &M5EnvGovernancePacket) {
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
                    "rule {} trigger {} imposes no maturity floor and must not be a rule",
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
                    "packet must encode a freshness rule for {} evidence",
                    required.as_str()
                ),
            );
        }
    }
}

fn validate_warm_start_rules(report: &mut ValidationReport, packet: &M5EnvGovernancePacket) {
    if packet.warm_start_rules.is_empty() {
        report.push(
            "packet.warm_start_rules",
            "packet must declare warm-start rules",
        );
    }
    let mut covered = BTreeSet::new();
    for rule in &packet.warm_start_rules {
        covered.insert(rule.trigger_evidence_state);
        match rule.trigger_evidence_state.warm_start_floor() {
            Some(expected) if expected == rule.warm_start_floor => {}
            Some(expected) => report.push(
                "warm_start_rule.floor",
                format!(
                    "rule {} warm-start floor {} disagrees with the engine ({})",
                    rule.rule_id,
                    rule.warm_start_floor.as_str(),
                    expected.as_str()
                ),
            ),
            None => report.push(
                "warm_start_rule.trigger",
                format!(
                    "rule {} trigger {} imposes no warm-start floor and must not be a rule",
                    rule.rule_id,
                    rule.trigger_evidence_state.as_str()
                ),
            ),
        }
        if rule.effect.trim().is_empty() || rule.rationale.trim().is_empty() {
            report.push(
                "warm_start_rule.prose",
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
                "packet.warm_start_rule_coverage",
                format!(
                    "packet must encode a warm-start rule for {} evidence",
                    required.as_str()
                ),
            );
        }
    }
}

fn validate_drills(report: &mut ValidationReport, packet: &M5EnvGovernancePacket) {
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
    let mut has_warm_start_downgrade = false;
    for capsule_drill in &packet.drills {
        if !drill_ids.insert(capsule_drill.drill_id.as_str()) {
            report.push(
                "drill.id_unique",
                format!("duplicate drill_id {}", capsule_drill.drill_id),
            );
        }
        drilled_profiles.insert(capsule_drill.profile);

        // Recompute the degraded outcome from the engine.
        let mut degraded = current_dimensions();
        for evidence in &mut degraded {
            if evidence.dimension == capsule_drill.exercised_dimension {
                evidence.evidence_state = capsule_drill.degraded_evidence_state;
            }
        }
        let degraded_outcome = certify_capsule_outcome(
            capsule_drill.claimed_maturity,
            capsule_drill.claimed_warm_start_posture,
            &degraded,
        );
        if capsule_drill.expected_degraded_verdict != degraded_outcome.verdict {
            report.push(
                "drill.degraded_verdict",
                format!(
                    "drill {} degraded verdict disagrees with the engine",
                    capsule_drill.drill_id
                ),
            );
        }
        if capsule_drill.expected_degraded_maturity != degraded_outcome.effective_maturity {
            report.push(
                "drill.degraded_maturity",
                format!(
                    "drill {} degraded maturity disagrees with the engine",
                    capsule_drill.drill_id
                ),
            );
        }
        if capsule_drill.expected_degraded_warm_start_posture
            != degraded_outcome.effective_warm_start_posture
        {
            report.push(
                "drill.degraded_warm_start_posture",
                format!(
                    "drill {} degraded warm-start posture disagrees with the engine",
                    capsule_drill.drill_id
                ),
            );
        }
        if degraded_outcome.verdict == RowVerdict::Certified {
            report.push(
                "drill.must_degrade",
                format!(
                    "drill {} must inject a failure that actually narrows or withholds",
                    capsule_drill.drill_id
                ),
            );
        }
        match degraded_outcome.verdict {
            RowVerdict::Narrowed => has_narrowed = true,
            RowVerdict::Withheld => has_withheld = true,
            RowVerdict::Certified => {}
        }
        if degraded_outcome.warm_start_downgraded {
            has_warm_start_downgrade = true;
        }
        if capsule_drill.recovers_to_verdict != RowVerdict::Certified {
            report.push(
                "drill.recovers",
                format!("drill {} must recover to certified", capsule_drill.drill_id),
            );
        }
        if !capsule_drill.asserts_claim_narrows_under_failure
            || !capsule_drill.asserts_recovers_after_refresh
        {
            report.push(
                "drill.assertions",
                format!(
                    "drill {} must assert it narrows under failure and recovers after refresh",
                    capsule_drill.drill_id
                ),
            );
        }
        validate_drill_steps(report, capsule_drill);
    }
    for required in EnvironmentProfile::ALL {
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
    if !has_warm_start_downgrade {
        report.push(
            "packet.warm_start_downgrade_drill",
            "packet must drill at least one warm-start downgrade",
        );
    }
}

fn validate_drill_steps(report: &mut ValidationReport, capsule_drill: &CapsuleDrill) {
    if capsule_drill.steps.is_empty() {
        report.push(
            "drill.steps",
            format!("drill {} must declare steps", capsule_drill.drill_id),
        );
        return;
    }
    if capsule_drill.steps.first().map(|s| s.phase) != Some(DrillPhase::Inject) {
        report.push(
            "drill.first_phase",
            format!(
                "drill {} must begin with an inject step",
                capsule_drill.drill_id
            ),
        );
    }
    if capsule_drill.steps.last().map(|s| s.phase) != Some(DrillPhase::Verify) {
        report.push(
            "drill.last_phase",
            format!(
                "drill {} must end with a verify step",
                capsule_drill.drill_id
            ),
        );
    }
    let has_narrow = capsule_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Narrow);
    let has_recover = capsule_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Recover);
    if !has_narrow || !has_recover {
        report.push(
            "drill.phases",
            format!(
                "drill {} must include a narrow step and a recover step",
                capsule_drill.drill_id
            ),
        );
    }
    for (index, drill_step) in capsule_drill.steps.iter().enumerate() {
        if drill_step.narration.trim().is_empty() {
            report.push(
                "drill.step_narration",
                format!("drill {} step {index} must narrate", capsule_drill.drill_id),
            );
        }
    }
}

fn validate_surface_bindings(report: &mut ValidationReport, packet: &M5EnvGovernancePacket) {
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
pub fn validate_m5_env_governance_fixture(
    fixture: &M5EnvGovernanceFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != M5_ENV_GOVERNANCE_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != M5_ENV_GOVERNANCE_SCHEMA_VERSION {
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

    let outcome = certify_capsule_outcome(
        fixture.claimed_maturity,
        fixture.claimed_warm_start_posture,
        &fixture.observed_dimensions,
    );
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
    if fixture.expected_warm_start_posture != outcome.effective_warm_start_posture {
        report.push(
            "fixture.expected_warm_start_posture",
            format!(
                "fixture {} expected warm-start posture {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_warm_start_posture.as_str(),
                outcome.effective_warm_start_posture.as_str()
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
    if fixture.expected_warm_start_downgrade_tokens != outcome.warm_start_downgrade_tokens {
        report.push(
            "fixture.expected_warm_start_downgrade_tokens",
            format!(
                "fixture {} expected warm-start downgrade tokens disagree with the engine",
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
