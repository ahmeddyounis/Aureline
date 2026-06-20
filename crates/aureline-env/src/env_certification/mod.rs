//! Promotion-grade certification of the whole environment-truth lane on
//! every claimed M5 local, container, remote, devcontainer, and managed
//! target class.
//!
//! The sibling lanes each freeze one slice of environment truth: the
//! [`crate::capsules`] capsule object and the [`crate::m5_env_governance`]
//! capsule-dimension matrix prove **capsule identity**, the
//! [`crate::workspace_templates`] packet proves **template composition**,
//! the [`crate::prebuilds`] fingerprint packet proves **prebuild
//! compatibility and invalidation**, the [`crate::hook_review`] packet
//! proves **lifecycle-hook truth**, and the
//! [`crate::runtime_materialization`] packet proves **runtime-instance
//! parity**. The [`crate::env_diagnostics`] report keeps all of them
//! portable, comparable, and diagnosable across online, mirrored, and
//! offline profiles.
//!
//! What none of those lanes does on its own is answer the promotion
//! question: *for this claimed target class, is every aspect of the
//! environment-truth lane proven current, or must the claim narrow?* This
//! module is that certification. It models one [`CertificationRow`] per
//! claimed [`TargetClass`], each carrying the five required
//! [`CertificationAspect`]s and the upstream evidence backing each, and
//! folds them through one [`certify_environment_lane`] engine into a single
//! promotion-grade [`RowVerdict`], an effective [`ClaimMaturity`], **and** a
//! narrowed [`WarmStartPosture`].
//!
//! The narrowing engine reuses the exact per-state floor functions the
//! capsule-dimension matrix uses — [`EvidenceState::qualification_floor`]
//! and [`EvidenceState::warm_start_floor`] — so the certification, the
//! per-dimension governance packet, the drills, and the fixtures can never
//! disagree about when a claim must narrow. Four guardrails are frozen
//! here:
//!
//! - **No promotion on stale or partial evidence.** A target class is
//!   certified at its claimed maturity only when every required aspect is
//!   `current`. Partial evidence narrows the claim to beta, stale evidence
//!   narrows it to preview, and missing evidence withholds it and blocks
//!   promotion — a target class that "opened once" cannot stay green while
//!   its capsule, template, or prebuild evidence is stale or incomplete.
//! - **Prebuilds and capsules govern warm start.** When the capsule
//!   identity or the prebuild-compatibility evidence goes partial or stale,
//!   the engine narrows the warm-start posture: a `warm_full_reuse` claim
//!   drops to partial reuse or a cold build instead of serving a stale warm
//!   snapshot as current truth.
//! - **One narrowing engine.** [`certify_environment_lane`] is the single
//!   source of truth for the lane verdict, shared by the rows, the drills,
//!   the fixtures, the [`EvidenceFreshnessRule`]s, and the
//!   [`WarmStartRule`]s. Release, support, docs, and help read the verdict
//!   and posture rather than re-deriving staleness.
//! - **Certification only narrows.** It never promotes a target class above
//!   its claimed maturity or warm-start posture, and a target class absent
//!   from the packet is uncertified, not implicitly green.
//!
//! Each aspect binds the **real** checked-in upstream lane artifact as its
//! evidence, so the certification is a composition of the frozen lanes
//! rather than fresh prose. The packet is mirrored, byte-for-byte, by the
//! checked-in schema, reviewer doc, proof packet, certification report, and
//! fixture corpus named on this module's constants.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_env_governance::{
    ClaimMaturity, EvidenceFreshnessRule, EvidenceState, PublicationChannel, RowVerdict,
    SurfaceBinding, WarmStartPosture, WarmStartRule,
};

use crate::{
    ENVIRONMENT_CAPSULE_PROOF_REF, ENVIRONMENT_CAPSULE_SCHEMA_REF, ENV_DIAGNOSTICS_RUNBOOK_REF,
    HOOK_REVIEW_PROOF_REF, HOOK_REVIEW_SCHEMA_REF, M5_ENV_GOVERNANCE_PACKET_REF,
    PREBUILD_FINGERPRINT_PACKET_REF, PREBUILD_FINGERPRINT_PROOF_REF, PREBUILD_REUSE_DRILLS_REF,
    RUNTIME_MATERIALIZATION_PROOF_REF, RUNTIME_MATERIALIZATION_SCHEMA_REF,
    WORKSPACE_TEMPLATE_PROOF_REF, WORKSPACE_TEMPLATE_SCHEMA_REF,
};

/// Schema version stamped onto packets and fixtures.
pub const ENV_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const ENV_CERTIFICATION_PACKET_RECORD_KIND: &str = "env_certification_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const ENV_CERTIFICATION_FIXTURE_RECORD_KIND: &str = "env_certification_fixture_record";

/// Repo-relative schema ref.
pub const ENV_CERTIFICATION_SCHEMA_REF: &str = "schemas/env/m5-env-certification.schema.json";

/// Repo-relative reviewer doc ref.
pub const ENV_CERTIFICATION_DOC_REF: &str = "docs/env/m5-env-certification.md";

/// Repo-relative machine-readable proof packet.
pub const ENV_CERTIFICATION_PACKET_REF: &str = "artifacts/env/m5-env-certification-packet.json";

/// Repo-relative reviewer certification summary.
pub const ENV_CERTIFICATION_REPORT_REF: &str = "artifacts/env/m5-env-certification.md";

/// Repo-relative fixture directory.
pub const ENV_CERTIFICATION_FIXTURE_DIR: &str = "fixtures/env/m5-env-certification";

/// Repo-relative fixture manifest.
pub const ENV_CERTIFICATION_FIXTURE_MANIFEST_REF: &str =
    "fixtures/env/m5-env-certification/manifest.yaml";

/// Stable packet id every binding ingests.
pub const ENV_CERTIFICATION_PACKET_ID: &str = "env.env_certification.v1";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// A claimed M5 deployment target class under certification. Each target
/// class is one materialization surface a capsule resolves onto, certified
/// as a whole environment rather than as a generic "workspace started"
/// label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetClass {
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

impl TargetClass {
    /// Every claimed target class in canonical order.
    pub const ALL: [Self; 5] = [
        Self::LocalNative,
        Self::Container,
        Self::RemoteHost,
        Self::Devcontainer,
        Self::ManagedCloud,
    ];

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

    /// Review-safe label for the target class.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalNative => "Local native runtime",
            Self::Container => "Local container runtime",
            Self::RemoteHost => "Remote host runtime",
            Self::Devcontainer => "Devcontainer runtime",
            Self::ManagedCloud => "Managed cloud workspace",
        }
    }
}

/// One aspect of the environment-truth lane a claimed target class must
/// prove. The five aspects are the exit-gate anchor: a target class may not
/// present a trustworthy environment unless capsule identity, template
/// composition, prebuild compatibility, lifecycle-hook truth, and
/// runtime-instance parity are all proven against their frozen upstream
/// lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationAspect {
    /// The capsule object is identified by a typed, versioned digest and
    /// certified across its seven capsule dimensions.
    CapsuleIdentity,
    /// Template hydration composes the same capsule object without forking
    /// the execution or trust model.
    TemplateComposition,
    /// Prebuild reuse is keyed on a compatibility fingerprint that
    /// invalidates rather than serving a stale warm snapshot.
    PrebuildCompatibility,
    /// Repo-defined lifecycle hooks stay trust-gated and reviewable rather
    /// than silently executed.
    LifecycleHookTruth,
    /// The runtime instance materialized for the capsule stays semantically
    /// aligned with its declared target across surfaces.
    RuntimeInstanceParity,
}

impl CertificationAspect {
    /// Every required aspect in canonical order.
    pub const ALL: [Self; 5] = [
        Self::CapsuleIdentity,
        Self::TemplateComposition,
        Self::PrebuildCompatibility,
        Self::LifecycleHookTruth,
        Self::RuntimeInstanceParity,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapsuleIdentity => "capsule_identity",
            Self::TemplateComposition => "template_composition",
            Self::PrebuildCompatibility => "prebuild_compatibility",
            Self::LifecycleHookTruth => "lifecycle_hook_truth",
            Self::RuntimeInstanceParity => "runtime_instance_parity",
        }
    }

    /// Whether degraded evidence on this aspect narrows the warm-start
    /// posture. Warm reuse is only trustworthy when the capsule's identity
    /// ([`CertificationAspect::CapsuleIdentity`]) and its cached artifact
    /// ([`CertificationAspect::PrebuildCompatibility`]) are current, so
    /// those two aspects — and only those — govern warm start.
    pub const fn governs_warm_start(self) -> bool {
        matches!(self, Self::CapsuleIdentity | Self::PrebuildCompatibility)
    }
}

/// The failure class a certification drill injects into one aspect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationFailureClass {
    /// The capsule identity drifts from the materialized environment.
    CapsuleIdentityDrift,
    /// Template composition forks the execution or trust model.
    TemplateCompositionFork,
    /// The prebuild fingerprint no longer proves compatibility.
    PrebuildInvalidation,
    /// A lifecycle hook would run without passing its trust gate.
    LifecycleHookUngated,
    /// The runtime instance diverges from the capsule's declared target.
    RuntimeInstanceSkew,
}

impl CertificationFailureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapsuleIdentityDrift => "capsule_identity_drift",
            Self::TemplateCompositionFork => "template_composition_fork",
            Self::PrebuildInvalidation => "prebuild_invalidation",
            Self::LifecycleHookUngated => "lifecycle_hook_ungated",
            Self::RuntimeInstanceSkew => "runtime_instance_skew",
        }
    }
}

/// One ordered phase of a certification drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillPhase {
    /// A failure is injected into a backing aspect.
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
// Narrowing engine: the single source of truth for the lane verdict.
// ---------------------------------------------------------------------------

/// One aspect's evidence on one target class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectEvidence {
    /// Certification aspect being evidenced.
    pub aspect: CertificationAspect,
    /// State of the upstream-lane evidence backing this aspect.
    pub evidence_state: EvidenceState,
    /// Checked-in upstream lane artifacts that prove this aspect.
    pub evidence_refs: Vec<String>,
    /// Review-safe rationale for the evidence.
    pub rationale: String,
}

/// The computed outcome of certifying one target class against its
/// per-aspect evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneOutcome {
    /// The narrowest maturity the claim may hold.
    pub effective_maturity: ClaimMaturity,
    /// The verdict the engine reaches.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its claimed maturity.
    pub narrowed: bool,
    /// Stable tokens naming every aspect that forced maturity narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Aspects whose evidence is stale or missing, in stable order.
    pub stale_or_missing_aspect_tokens: Vec<String>,
    /// The coldest warm-start posture the claim may hold.
    pub effective_warm_start_posture: WarmStartPosture,
    /// True when the warm-start posture narrowed below the claimed one.
    pub warm_start_downgraded: bool,
    /// Stable tokens naming every warm-start-governing aspect that forced a
    /// colder posture.
    pub warm_start_downgrade_tokens: Vec<String>,
}

/// Certifies one target class's claim against its per-aspect evidence.
///
/// This is the canonical lane-narrowing engine the whole packet, every
/// drill, and every fixture share. It reuses the exact per-state floor
/// functions the capsule-dimension matrix uses
/// ([`EvidenceState::qualification_floor`] and
/// [`EvidenceState::warm_start_floor`]), so the lane certification and the
/// capsule governance can never disagree about a downgrade. The effective
/// maturity starts at the claimed maturity and is floored by every degraded
/// aspect; the warm-start posture starts at the claimed posture and is
/// floored by every degraded warm-start-governing aspect; the narrowest
/// (highest-severity) result wins on each axis. A withdrawn maturity is
/// [`RowVerdict::Withheld`]; any other maturity below the claimed one is
/// [`RowVerdict::Narrowed`]; otherwise the target class is
/// [`RowVerdict::Certified`].
pub fn certify_environment_lane(
    claimed_maturity: ClaimMaturity,
    claimed_warm_start_posture: WarmStartPosture,
    aspects: &[AspectEvidence],
) -> LaneOutcome {
    let mut effective_maturity = claimed_maturity;
    let mut effective_warm_start = claimed_warm_start_posture;
    let mut narrow_reason_tokens = Vec::new();
    let mut warm_start_downgrade_tokens = Vec::new();
    let mut stale_or_missing = Vec::new();

    for evidence in aspects {
        if let Some(floor) = evidence.evidence_state.qualification_floor() {
            if floor.severity() > effective_maturity.severity() {
                effective_maturity = floor;
            }
            narrow_reason_tokens.push(format!(
                "{}_{}",
                evidence.aspect.as_str(),
                evidence.evidence_state.as_str()
            ));
        }
        if evidence.aspect.governs_warm_start() {
            if let Some(floor) = evidence.evidence_state.warm_start_floor() {
                if floor.severity() > effective_warm_start.severity() {
                    effective_warm_start = floor;
                }
                warm_start_downgrade_tokens.push(format!(
                    "{}_{}",
                    evidence.aspect.as_str(),
                    evidence.evidence_state.as_str()
                ));
            }
        }
        if evidence.evidence_state.is_stale_or_missing() {
            stale_or_missing.push(evidence.aspect.as_str().to_owned());
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

    LaneOutcome {
        effective_maturity,
        verdict,
        narrowed: verdict == RowVerdict::Narrowed,
        narrow_reason_tokens,
        stale_or_missing_aspect_tokens: stale_or_missing,
        effective_warm_start_posture: effective_warm_start,
        warm_start_downgraded: effective_warm_start.severity()
            > claimed_warm_start_posture.severity(),
        warm_start_downgrade_tokens,
    }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

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

/// One certification row: a claimed target class, its per-aspect evidence,
/// and the engine outcome stamped onto it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed target class.
    pub target_class: TargetClass,
    /// Review-safe label for the target class.
    pub target_class_label: String,
    /// Maturity claimed for the target class.
    pub claimed_maturity: ClaimMaturity,
    /// Warm-start posture claimed for the target class.
    pub claimed_warm_start_posture: WarmStartPosture,
    /// Governance surface classes this target class spans.
    pub backing_surface_classes: Vec<String>,
    /// Per-aspect evidence, one entry per required aspect.
    pub aspects: Vec<AspectEvidence>,
    /// Effective maturity after narrowing.
    pub effective_maturity: ClaimMaturity,
    /// Engine verdict.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its claimed maturity.
    pub narrowed: bool,
    /// Stable tokens naming every aspect that forced maturity narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Aspects whose evidence is stale or missing.
    pub stale_or_missing_aspect_tokens: Vec<String>,
    /// Effective warm-start posture after narrowing.
    pub effective_warm_start_posture: WarmStartPosture,
    /// True when the warm-start posture narrowed below the claimed one.
    pub warm_start_downgraded: bool,
    /// Stable tokens naming every warm-start-governing aspect that forced a
    /// colder posture.
    pub warm_start_downgrade_tokens: Vec<String>,
    /// Review-safe "why this certification" inspector line.
    pub why_this_certification: String,
    /// Upstream lane artifacts this row composes.
    pub supporting_evidence_refs: Vec<String>,
    /// Real consumer surfaces that ingest this row.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One ordered step inside a certification drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDrillStep {
    /// Phase of this step.
    pub phase: DrillPhase,
    /// Maturity observed at this step.
    pub observed_maturity: ClaimMaturity,
    /// Warm-start posture observed at this step.
    pub observed_warm_start_posture: WarmStartPosture,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One failure / recovery drill walking a target class from an injected
/// aspect failure through narrowing and back to recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Target class exercised by the drill.
    pub target_class: TargetClass,
    /// Aspect whose evidence the drill degrades.
    pub exercised_aspect: CertificationAspect,
    /// Failure class the drill injects.
    pub failure_class: CertificationFailureClass,
    /// Evidence state the aspect degrades to.
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
    /// True when the degraded claim blocks promotion.
    pub blocks_promotion_while_degraded: bool,
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

/// The rolled-up promotion decision over every certified target class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionSummary {
    /// True when any claimed target class is withheld, so promotion is
    /// held until its evidence is restored.
    pub promotion_blocked: bool,
    /// Target classes that certified at their claimed maturity.
    pub certified_target_class_tokens: Vec<String>,
    /// Target classes whose claim narrowed below its claimed maturity.
    pub narrowed_target_class_tokens: Vec<String>,
    /// Target classes whose claim is withheld and block promotion.
    pub held_target_class_tokens: Vec<String>,
    /// Review-safe summary of the promotion decision.
    pub summary: String,
}

/// Top-level packet certifying the whole environment-truth lane on every
/// claimed M5 target class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvCertificationPacket {
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
    /// Required certification aspects.
    pub certified_aspects: Vec<CertificationAspect>,
    /// Upstream environment lane packets this certification composes.
    pub lane_evidence_refs: Vec<String>,
    /// Certification rows, one per claimed target class.
    pub rows: Vec<CertificationRow>,
    /// Automatic maturity-narrowing rules over evidence states.
    pub freshness_rules: Vec<EvidenceFreshnessRule>,
    /// Automatic warm-start-narrowing rules over evidence states.
    pub warm_start_rules: Vec<WarmStartRule>,
    /// Failure / recovery drills.
    pub drills: Vec<CertificationDrill>,
    /// Publication-channel bindings.
    pub surface_bindings: Vec<SurfaceBinding>,
    /// Rolled-up promotion decision.
    pub promotion: PromotionSummary,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a target class and an observed evidence
/// configuration to the expected verdict and warm-start posture, proving
/// the canonical lane-narrowing behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvCertificationFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Target class under test.
    pub target_class: TargetClass,
    /// Maturity claimed before narrowing.
    pub claimed_maturity: ClaimMaturity,
    /// Warm-start posture claimed before narrowing.
    pub claimed_warm_start_posture: WarmStartPosture,
    /// Observed per-aspect evidence.
    pub observed_aspects: Vec<AspectEvidence>,
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
    /// True when this fixture's verdict blocks promotion.
    pub blocks_promotion: bool,
    /// One consumer that quotes this target class.
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
        writeln!(f, "env certification validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

// ---------------------------------------------------------------------------
// Upstream-lane evidence vocabulary used by the seed.
// ---------------------------------------------------------------------------

/// The composed upstream environment lane packets this certification folds
/// into one promotion-grade lane verdict. Every ref is a checked-in
/// artifact, so the certification is a composition of the frozen lanes.
fn lane_evidence_refs() -> Vec<String> {
    [
        ENVIRONMENT_CAPSULE_PROOF_REF,
        WORKSPACE_TEMPLATE_PROOF_REF,
        PREBUILD_FINGERPRINT_PACKET_REF,
        HOOK_REVIEW_PROOF_REF,
        RUNTIME_MATERIALIZATION_PROOF_REF,
        ENV_DIAGNOSTICS_RUNBOOK_REF,
        M5_ENV_GOVERNANCE_PACKET_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// The canonical upstream-lane evidence refs for one aspect when it is
/// fully current. Each aspect cites the frozen lane that proves it, so the
/// certification is anchored in the checked-in lane artifacts.
fn aspect_evidence_refs(aspect: CertificationAspect) -> Vec<&'static str> {
    match aspect {
        CertificationAspect::CapsuleIdentity => vec![
            ENVIRONMENT_CAPSULE_PROOF_REF,
            ENVIRONMENT_CAPSULE_SCHEMA_REF,
            M5_ENV_GOVERNANCE_PACKET_REF,
        ],
        CertificationAspect::TemplateComposition => {
            vec![WORKSPACE_TEMPLATE_PROOF_REF, WORKSPACE_TEMPLATE_SCHEMA_REF]
        }
        CertificationAspect::PrebuildCompatibility => vec![
            PREBUILD_FINGERPRINT_PACKET_REF,
            PREBUILD_FINGERPRINT_PROOF_REF,
            PREBUILD_REUSE_DRILLS_REF,
        ],
        CertificationAspect::LifecycleHookTruth => {
            vec![HOOK_REVIEW_PROOF_REF, HOOK_REVIEW_SCHEMA_REF]
        }
        CertificationAspect::RuntimeInstanceParity => vec![
            RUNTIME_MATERIALIZATION_PROOF_REF,
            RUNTIME_MATERIALIZATION_SCHEMA_REF,
        ],
    }
}

fn aspect_rationale(aspect: CertificationAspect) -> &'static str {
    match aspect {
        CertificationAspect::CapsuleIdentity => {
            "The capsule object is identified by a typed, versioned digest and certified across its seven capsule dimensions, so identity is inspectable, diffable, and mirrorable rather than implied by side effects."
        }
        CertificationAspect::TemplateComposition => {
            "Template hydration composes the same capsule object with workflow-bundle, archetype, and docs layers, so a starter cannot fork the execution or trust model from the core capsule."
        }
        CertificationAspect::PrebuildCompatibility => {
            "Prebuild reuse is keyed on a compatibility fingerprint over source, capsule, platform, policy, extension, and toolchain inputs, so a drifted fingerprint invalidates the snapshot instead of serving it as current truth."
        }
        CertificationAspect::LifecycleHookTruth => {
            "Repo-defined lifecycle hooks are surfaced as trust-gated review objects, so policy-denied or restricted hooks become visible holds rather than silent side effects during hydration or warm start."
        }
        CertificationAspect::RuntimeInstanceParity => {
            "The runtime instance materialized for the capsule — process namespace, mounts, ports, service readiness, and secret projection — stays semantically aligned with the capsule's declared target across desktop, CLI, AI, and support."
        }
    }
}

/// Builds the five fully-current aspects for a healthy row.
fn current_aspects() -> Vec<AspectEvidence> {
    CertificationAspect::ALL
        .into_iter()
        .map(|aspect| AspectEvidence {
            aspect,
            evidence_state: EvidenceState::Current,
            evidence_refs: aspect_evidence_refs(aspect)
                .into_iter()
                .map(str::to_owned)
                .collect(),
            rationale: aspect_rationale(aspect).to_owned(),
        })
        .collect()
}

fn degraded_aspects(aspect: CertificationAspect, state: EvidenceState) -> Vec<AspectEvidence> {
    let mut aspects = current_aspects();
    for evidence in &mut aspects {
        if evidence.aspect == aspect {
            evidence.evidence_state = state;
        }
    }
    aspects
}

fn supporting_evidence_refs(aspects: &[AspectEvidence]) -> Vec<String> {
    let mut refs: BTreeSet<String> = BTreeSet::new();
    for aspect in aspects {
        for reference in &aspect.evidence_refs {
            refs.insert(reference.clone());
        }
    }
    refs.into_iter().collect()
}

fn claimed_posture_for(target_class: TargetClass) -> (ClaimMaturity, WarmStartPosture) {
    match target_class {
        TargetClass::LocalNative => (ClaimMaturity::Stable, WarmStartPosture::WarmPartialReuse),
        TargetClass::Container => (ClaimMaturity::Beta, WarmStartPosture::WarmFullReuse),
        TargetClass::RemoteHost => (ClaimMaturity::Beta, WarmStartPosture::WarmPartialReuse),
        TargetClass::Devcontainer => (ClaimMaturity::Beta, WarmStartPosture::WarmPartialReuse),
        TargetClass::ManagedCloud => (ClaimMaturity::Beta, WarmStartPosture::WarmFullReuse),
    }
}

fn consumer_refs_for(target_class: TargetClass) -> Vec<&'static str> {
    match target_class {
        TargetClass::LocalNative => vec![
            "crates/aureline-workspace/src/entry/mod.rs",
            "crates/aureline-runtime/src/execution_context/mod.rs",
        ],
        TargetClass::Container => vec![
            "crates/aureline-runtime/src/capsule_resolver/mod.rs",
            "crates/aureline-runtime/src/env_inspect/mod.rs",
        ],
        TargetClass::RemoteHost => vec![
            "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs",
            "crates/aureline-runtime/src/capsule_resolver/mod.rs",
        ],
        TargetClass::Devcontainer => vec![
            "crates/aureline-runtime/src/execution_context/mod.rs",
            "crates/aureline-runtime/src/env_inspect/mod.rs",
        ],
        TargetClass::ManagedCloud => vec![
            "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs",
            "crates/aureline-support/src/bundle/mod.rs",
        ],
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    target_class: TargetClass,
    backing_surface_classes: &[&str],
    why_this_certification: &str,
    notes: &str,
) -> CertificationRow {
    let (claimed_maturity, claimed_warm_start_posture) = claimed_posture_for(target_class);
    let aspects = current_aspects();
    let outcome = certify_environment_lane(claimed_maturity, claimed_warm_start_posture, &aspects);
    let supporting_evidence_refs = supporting_evidence_refs(&aspects);
    CertificationRow {
        row_id: row_id.to_owned(),
        target_class,
        target_class_label: target_class.label().to_owned(),
        claimed_maturity,
        claimed_warm_start_posture,
        backing_surface_classes: backing_surface_classes
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        aspects,
        effective_maturity: outcome.effective_maturity,
        verdict: outcome.verdict,
        narrowed: outcome.narrowed,
        narrow_reason_tokens: outcome.narrow_reason_tokens,
        stale_or_missing_aspect_tokens: outcome.stale_or_missing_aspect_tokens,
        effective_warm_start_posture: outcome.effective_warm_start_posture,
        warm_start_downgraded: outcome.warm_start_downgraded,
        warm_start_downgrade_tokens: outcome.warm_start_downgrade_tokens,
        why_this_certification: why_this_certification.to_owned(),
        supporting_evidence_refs,
        consumer_refs: consumer_refs_for(target_class)
            .into_iter()
            .map(str::to_owned)
            .collect(),
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
) -> CertificationDrillStep {
    CertificationDrillStep {
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
    target_class: TargetClass,
    exercised_aspect: CertificationAspect,
    failure_class: CertificationFailureClass,
    degraded_evidence_state: EvidenceState,
    steps: Vec<CertificationDrillStep>,
    notes: &str,
) -> CertificationDrill {
    let (claimed_maturity, claimed_warm_start_posture) = claimed_posture_for(target_class);
    // The degraded posture is computed from the same engine the rows use,
    // so a drill can never disagree with the certification.
    let degraded = degraded_aspects(exercised_aspect, degraded_evidence_state);
    let degraded_outcome =
        certify_environment_lane(claimed_maturity, claimed_warm_start_posture, &degraded);
    CertificationDrill {
        drill_id: drill_id.to_owned(),
        title: title.to_owned(),
        target_class,
        exercised_aspect,
        failure_class,
        degraded_evidence_state,
        claimed_maturity,
        claimed_warm_start_posture,
        expected_degraded_verdict: degraded_outcome.verdict,
        expected_degraded_maturity: degraded_outcome.effective_maturity,
        expected_degraded_warm_start_posture: degraded_outcome.effective_warm_start_posture,
        blocks_promotion_while_degraded: degraded_outcome.verdict == RowVerdict::Withheld,
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
        ingested_packet_id: ENV_CERTIFICATION_PACKET_ID.to_owned(),
        required_verbatim_fields: REQUIRED_VERBATIM_FIELDS
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        narrows_with_packet: true,
        summary: summary.to_owned(),
    }
}

const REQUIRED_VERBATIM_FIELDS: [&str; 7] = [
    "row_id",
    "target_class",
    "claimed_maturity",
    "effective_maturity",
    "verdict",
    "effective_warm_start_posture",
    "narrow_reason_tokens",
];

fn promotion_summary(rows: &[CertificationRow]) -> PromotionSummary {
    let mut certified = Vec::new();
    let mut narrowed = Vec::new();
    let mut held = Vec::new();
    for capsule_row in rows {
        match capsule_row.verdict {
            RowVerdict::Certified => certified.push(capsule_row.target_class.as_str().to_owned()),
            RowVerdict::Narrowed => narrowed.push(capsule_row.target_class.as_str().to_owned()),
            RowVerdict::Withheld => held.push(capsule_row.target_class.as_str().to_owned()),
        }
    }
    certified.sort();
    narrowed.sort();
    held.sort();
    let promotion_blocked = !held.is_empty();
    let summary = if promotion_blocked {
        format!(
            "Promotion is held: {} target class(es) withhold the environment claim until their evidence is restored.",
            held.len()
        )
    } else if !narrowed.is_empty() {
        format!(
            "Promotion proceeds with {} narrowed target class(es); each claim is published at its narrowed maturity rather than its claimed one.",
            narrowed.len()
        )
    } else {
        "Every claimed target class is certified at its claimed maturity and warm-start posture; the environment lane may be promoted.".to_owned()
    };
    PromotionSummary {
        promotion_blocked,
        certified_target_class_tokens: certified,
        narrowed_target_class_tokens: narrowed,
        held_target_class_tokens: held,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in environment-certification packet this lane
/// freezes.
pub fn seeded_env_certification_packet() -> EnvCertificationPacket {
    let rows = vec![
        row(
            "env.cert.local_native",
            TargetClass::LocalNative,
            &["local_runtime", "project_starter"],
            "This target class is the capsule materialized natively on the local host: capsule identity, template composition, prebuild compatibility, lifecycle-hook truth, and runtime-instance parity are each proven against their frozen lanes before the local environment claims to be trustworthy.",
            "A local-native environment certifies at stable with partial warm reuse; a partial template composition or stale capsule narrows the claim rather than trusting a one-time open.",
        ),
        row(
            "env.cert.container",
            TargetClass::Container,
            &["container_runtime", "prebuild_snapshot"],
            "This target class is the capsule materialized in a local container: a current prebuild fingerprint lets the whole environment warm-reuse, and every other aspect is proven against its lane before the container claims full reuse.",
            "A container environment claims full warm reuse only while its prebuild compatibility is current; a stale fingerprint narrows the maturity and forces a cold build.",
        ),
        row(
            "env.cert.remote_host",
            TargetClass::RemoteHost,
            &["remote_container_runtime"],
            "This target class is the capsule materialized on a remote host within its declared boundary: the runtime instance is checked for parity against the capsule's declared target, and warm reuse stays partial.",
            "A remote-host environment narrows when its runtime-instance parity skews or its capsule identity goes stale, never presenting a wrong-target run as aligned.",
        ),
        row(
            "env.cert.devcontainer",
            TargetClass::Devcontainer,
            &["devcontainer_definition"],
            "This target class is the capsule materialized from a devcontainer definition: lifecycle-hook truth keeps the devcontainer's post-create and bootstrap actions trust-gated, and every other aspect is proven against its lane.",
            "A devcontainer environment is withheld when its lifecycle-hook truth is missing, so an ungated hook can never run silently during hydration.",
        ),
        row(
            "env.cert.managed_cloud",
            TargetClass::ManagedCloud,
            &["managed_workspace_row"],
            "This target class is the capsule materialized in a managed cloud workspace: its capsule, prebuild fingerprint, and runtime instance are mirrored so support and release read the captured environment claim, not live truth.",
            "A managed-cloud environment claims full warm reuse from a current prebuild and capsule; a stale capsule narrows the claim and forces a cold build.",
        ),
    ];

    let freshness_rules = vec![
        freshness_rule(
            "freshness.partial_narrows_to_beta",
            EvidenceState::Partial,
            "A claimed target class with partial evidence on any required aspect narrows to at most a beta claim.",
            "Partial lane evidence proves only part of the claimed environment, so the target class may not present a stable environment guarantee.",
        ),
        freshness_rule(
            "freshness.stale_narrows_to_preview",
            EvidenceState::Stale,
            "A claimed target class with stale evidence on any required aspect narrows to at most a preview claim.",
            "Stale lane evidence may no longer reflect the current source or platform truth, so the target class drops below beta until the evidence is refreshed.",
        ),
        freshness_rule(
            "freshness.missing_withholds_and_blocks_promotion",
            EvidenceState::Missing,
            "A claimed target class missing evidence on any required aspect is withheld and blocks promotion until the aspect is proven.",
            "A required environment aspect with no backing lane evidence cannot be proven, so the target class may not be promoted at its claimed maturity.",
        ),
    ];

    let warm_start_rules = vec![
        warm_start_rule(
            "warm_start.partial_narrows_to_partial_reuse",
            EvidenceState::Partial,
            "Partial capsule-identity or prebuild-compatibility evidence narrows the warm-start posture to at most partial reuse.",
            "A partially proven capsule or fingerprint cannot prove the whole cached environment matches the current source, so only part of the environment may be reused.",
        ),
        warm_start_rule(
            "warm_start.stale_forces_cold_build",
            EvidenceState::Stale,
            "Stale capsule-identity or prebuild-compatibility evidence forces a cold build.",
            "A stale capsule or fingerprint can no longer prove the cached artifact matches the current source, so the environment must be rebuilt rather than served warm.",
        ),
        warm_start_rule(
            "warm_start.missing_forces_cold_build",
            EvidenceState::Missing,
            "Missing capsule-identity or prebuild-compatibility evidence forces a cold build.",
            "Without a capsule identity or fingerprint the environment cannot be identified for reuse, so warm start is unavailable and the environment is rebuilt cold.",
        ),
    ];

    let drills = vec![
        drill(
            "drill.env_cert.local_native_template_partial",
            "Local-native narrows to beta on partial template composition",
            TargetClass::LocalNative,
            CertificationAspect::TemplateComposition,
            CertificationFailureClass::TemplateCompositionFork,
            EvidenceState::Partial,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Stable,
                    WarmStartPosture::WarmPartialReuse,
                    "The starter template composes only part of the capsule's layers after a workflow-bundle reference drifts under it.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Stable,
                    WarmStartPosture::WarmPartialReuse,
                    "Template-composition evidence is observed partial for the local-native target class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The certified claim narrows to beta; the local environment labels its template composition partial rather than implying the whole environment.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The template composition is recaptured from the current workflow-bundle and archetype layers.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Stable,
                    WarmStartPosture::WarmPartialReuse,
                    "Template-composition evidence returns current; the claim recovers to stable.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Stable,
                    WarmStartPosture::WarmPartialReuse,
                    "The recovered posture matches the certification engine for a fully current local-native row.",
                ),
            ],
            "Partial template composition narrows the local-native claim to beta without withholding it, even though the environment opened once on a happy-path capture.",
        ),
        drill(
            "drill.env_cert.container_prebuild_stale",
            "Container narrows to preview and cold-builds when its prebuild compatibility goes stale",
            TargetClass::Container,
            CertificationAspect::PrebuildCompatibility,
            CertificationFailureClass::PrebuildInvalidation,
            EvidenceState::Stale,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "The source digest advances while the container's prebuild snapshot stays pinned, so its fingerprint trails the current source.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "Prebuild-compatibility evidence is observed stale for the container target class.",
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
                    "Prebuild-compatibility evidence returns current; the claim recovers to beta with full warm reuse.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "The recovered posture matches the certification engine for a fully current container row.",
                ),
            ],
            "A stale prebuild fingerprint narrows the container claim to preview and forces a cold build, never letting an approximate warm snapshot imply trustworthy reuse.",
        ),
        drill(
            "drill.env_cert.remote_host_runtime_skew",
            "Remote-host narrows to preview on runtime-instance skew",
            TargetClass::RemoteHost,
            CertificationAspect::RuntimeInstanceParity,
            CertificationFailureClass::RuntimeInstanceSkew,
            EvidenceState::Stale,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The remote-host runtime instance diverges from the capsule's declared target after the host's base image rolls under it.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "Runtime-instance-parity evidence is observed stale for the remote-host target class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    WarmStartPosture::WarmPartialReuse,
                    "The certified claim narrows to preview; the remote environment labels its runtime instance skewed rather than presenting a wrong-target run as aligned.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    WarmStartPosture::WarmPartialReuse,
                    "The runtime instance is re-derived from the capsule and re-checked against its declared target.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "Runtime-instance-parity evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The recovered posture matches the certification engine for a fully current remote-host row.",
                ),
            ],
            "Stale runtime-instance parity narrows the remote-host claim to preview until the instance is re-aligned with the capsule's declared target.",
        ),
        drill(
            "drill.env_cert.devcontainer_hook_missing",
            "Devcontainer is withheld and blocks promotion when lifecycle-hook truth is missing",
            TargetClass::Devcontainer,
            CertificationAspect::LifecycleHookTruth,
            CertificationFailureClass::LifecycleHookUngated,
            EvidenceState::Missing,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The devcontainer declares a post-create hook with no trust-gate evidence binding it to the hook-review lane.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "Lifecycle-hook-truth evidence is observed missing for the devcontainer target class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Withdrawn,
                    WarmStartPosture::WarmPartialReuse,
                    "The certification withholds the devcontainer claim and blocks promotion; hydration is held until the hook is trust-gated rather than silently executed.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Withdrawn,
                    WarmStartPosture::WarmPartialReuse,
                    "The hook is surfaced as a trust-gated review object and the evidence is restored.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "Lifecycle-hook-truth evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmPartialReuse,
                    "The recovered posture matches the certification engine for a fully current devcontainer row.",
                ),
            ],
            "Missing lifecycle-hook truth withholds the devcontainer claim and blocks promotion rather than letting an ungated hook run during hydration.",
        ),
        drill(
            "drill.env_cert.managed_cloud_capsule_stale",
            "Managed-cloud narrows to preview and cold-builds when its capsule identity goes stale",
            TargetClass::ManagedCloud,
            CertificationAspect::CapsuleIdentity,
            CertificationFailureClass::CapsuleIdentityDrift,
            EvidenceState::Stale,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "The managed-cloud capsule digest ages past its freshness window after a defining input changes upstream of the mirror.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "Capsule-identity evidence is observed stale for the managed-cloud target class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    WarmStartPosture::ColdBuild,
                    "The certified claim narrows to preview and the warm-start posture drops to a cold build; the stale capsule can no longer prove the cached environment matches the current source.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    WarmStartPosture::ColdBuild,
                    "The capsule digest is recomputed and re-mirrored against the current defining inputs.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "Capsule-identity evidence returns current; the claim recovers to beta with full warm reuse.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    WarmStartPosture::WarmFullReuse,
                    "The recovered posture matches the certification engine for a fully current managed-cloud row.",
                ),
            ],
            "A stale capsule identity narrows the managed-cloud claim to preview and forces a cold build, so a mirrored warm snapshot can never outrun the current source.",
        ),
    ];

    let surface_bindings = vec![
        binding(
            PublicationChannel::ReleaseShiproom,
            "artifacts/release/shiproom_dashboard.json",
            "The shiproom dashboard reads the per-row verdict, effective maturity, warm-start posture, and the rolled-up promotion decision, and holds promotion for any narrowed or withheld release-scope target class.",
        ),
        binding(
            PublicationChannel::SupportExport,
            "crates/aureline-support/src/bundle/mod.rs",
            "The metadata-first support bundle re-exports the per-row verdict, narrowing tokens, warm-start posture, and stale-or-missing aspects without raw paths, credentials, or provider payloads.",
        ),
        binding(
            PublicationChannel::Docs,
            "docs/env/m5-env-certification.md",
            "The reviewer documentation quotes the certified aspects, freshness and warm-start rules, per-row verdicts, and the promotion decision directly from the packet.",
        ),
        binding(
            PublicationChannel::Help,
            "crates/aureline-runtime/src/env_inspect/mod.rs",
            "The in-product why-this-environment inspector reuses the same verdict and warm-start vocabulary so help never tells a greener story than the packet.",
        ),
    ];

    let promotion = promotion_summary(&rows);

    EnvCertificationPacket {
        record_kind: ENV_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: ENV_CERTIFICATION_SCHEMA_VERSION,
        packet_id: ENV_CERTIFICATION_PACKET_ID.to_owned(),
        title: "Promotion-grade certification of environment-capsule, workspace-template, prebuild-fingerprint, and runtime-materialization truth on every claimed M5 target class"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: ENV_CERTIFICATION_DOC_REF.to_owned(),
            schema_ref: ENV_CERTIFICATION_SCHEMA_REF.to_owned(),
            packet_ref: ENV_CERTIFICATION_PACKET_REF.to_owned(),
            report_ref: ENV_CERTIFICATION_REPORT_REF.to_owned(),
            fixture_manifest_ref: ENV_CERTIFICATION_FIXTURE_MANIFEST_REF.to_owned(),
        },
        certified_aspects: CertificationAspect::ALL.to_vec(),
        lane_evidence_refs: lane_evidence_refs(),
        rows,
        freshness_rules,
        warm_start_rules,
        drills,
        surface_bindings,
        promotion,
        invariants: vec![
            "Each claimed M5 target class is certified only when every required environment aspect — capsule identity, template composition, prebuild compatibility, lifecycle-hook truth, and runtime-instance parity — is proven current against its frozen upstream lane.".to_owned(),
            "One narrowing engine folds per-aspect evidence into a verdict and a warm-start posture: partial evidence narrows to beta, stale evidence narrows to preview, missing evidence withholds the claim, and stale or partial capsule/prebuild evidence narrows warm reuse.".to_owned(),
            "A withheld target class blocks promotion; the rolled-up promotion decision can never report green while any claimed local, container, remote, devcontainer, or managed target class cannot prove every aspect.".to_owned(),
            "Prebuilds and capsules are accelerators rather than authorities: a warm-full-reuse claim drops to partial reuse or a cold build whenever the capsule identity or prebuild compatibility outruns current truth, so a target class that opened once cannot stay green on stale evidence.".to_owned(),
            "The certification only narrows, never widens; a target class absent from the packet is uncertified rather than implicitly green, and the lane never silently widens a claim beyond the target classes the packet proves.".to_owned(),
            "Release, support, docs, and help all read the same per-row verdict, warm-start posture, narrowing tokens, and promotion decision instead of re-deriving environment staleness.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture corpus this lane freezes.
pub fn seeded_env_certification_fixtures() -> Vec<EnvCertificationFixture> {
    let mut fixtures = Vec::new();

    // One healthy fixture per target class, pinning the certified verdict.
    for target_class in TargetClass::ALL {
        fixtures.push(fixture(
            &format!("fixture.env_certification.{}_certified", target_class.as_str()),
            target_class,
            current_aspects(),
            "A fully current target class certifies at its claimed maturity and warm-start posture with no narrowing tokens.",
        ));
    }

    // Degraded fixtures exercising every floor and verdict.
    fixtures.push(fixture(
        "fixture.env_certification.container_prebuild_stale",
        TargetClass::Container,
        degraded_aspects(CertificationAspect::PrebuildCompatibility, EvidenceState::Stale),
        "A stale prebuild compatibility narrows the beta container claim to a preview verdict and forces a cold build instead of full warm reuse.",
    ));
    fixtures.push(fixture(
        "fixture.env_certification.local_native_template_partial",
        TargetClass::LocalNative,
        degraded_aspects(CertificationAspect::TemplateComposition, EvidenceState::Partial),
        "Partial template composition narrows the stable local-native claim to a beta verdict while warm reuse stays at partial.",
    ));
    fixtures.push(fixture(
        "fixture.env_certification.devcontainer_hook_missing",
        TargetClass::Devcontainer,
        degraded_aspects(CertificationAspect::LifecycleHookTruth, EvidenceState::Missing),
        "Missing lifecycle-hook truth withholds the devcontainer claim entirely and blocks promotion.",
    ));
    fixtures.push(fixture(
        "fixture.env_certification.managed_cloud_capsule_stale",
        TargetClass::ManagedCloud,
        degraded_aspects(CertificationAspect::CapsuleIdentity, EvidenceState::Stale),
        "A stale capsule identity narrows the managed-cloud claim to preview and forces a cold build because capsule identity governs warm start.",
    ));

    fixtures
}

fn consumer_ref_for(target_class: TargetClass) -> &'static str {
    consumer_refs_for(target_class)[0]
}

fn fixture(
    fixture_id: &str,
    target_class: TargetClass,
    observed_aspects: Vec<AspectEvidence>,
    notes: &str,
) -> EnvCertificationFixture {
    let (claimed_maturity, claimed_warm_start_posture) = claimed_posture_for(target_class);
    let outcome = certify_environment_lane(
        claimed_maturity,
        claimed_warm_start_posture,
        &observed_aspects,
    );
    EnvCertificationFixture {
        record_kind: ENV_CERTIFICATION_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: ENV_CERTIFICATION_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        target_class,
        claimed_maturity,
        claimed_warm_start_posture,
        observed_aspects,
        expected_verdict: outcome.verdict,
        expected_effective_maturity: outcome.effective_maturity,
        expected_warm_start_posture: outcome.effective_warm_start_posture,
        expected_narrow_reason_tokens: outcome.narrow_reason_tokens,
        expected_warm_start_downgrade_tokens: outcome.warm_start_downgrade_tokens,
        blocks_promotion: outcome.verdict == RowVerdict::Withheld,
        consumer_ref: consumer_ref_for(target_class).to_owned(),
        notes: notes.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in packet contract.
pub fn validate_env_certification_packet(
    packet: &EnvCertificationPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != ENV_CERTIFICATION_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != ENV_CERTIFICATION_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != ENV_CERTIFICATION_PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.doc_ref != ENV_CERTIFICATION_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != ENV_CERTIFICATION_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != ENV_CERTIFICATION_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != ENV_CERTIFICATION_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != ENV_CERTIFICATION_FIXTURE_MANIFEST_REF {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.certified_aspects != CertificationAspect::ALL.to_vec() {
        report.push(
            "packet.certified_aspects",
            "packet must certify every required aspect in canonical order",
        );
    }
    if packet.lane_evidence_refs.is_empty() {
        report.push(
            "packet.lane_evidence_refs",
            "packet must cite the upstream environment lane packets",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    let mut covered_target_classes = BTreeSet::new();
    for certification_row in &packet.rows {
        if !covered_target_classes.insert(certification_row.target_class) {
            report.push(
                "row.target_class_unique",
                format!(
                    "duplicate target class {}",
                    certification_row.target_class.as_str()
                ),
            );
        }
        validate_row(&mut report, certification_row);
    }
    for required in TargetClass::ALL {
        if !covered_target_classes.contains(&required) {
            report.push(
                "packet.covered_target_class",
                format!("packet must certify target class {}", required.as_str()),
            );
        }
    }

    validate_freshness_rules(&mut report, packet);
    validate_warm_start_rules(&mut report, packet);
    validate_drills(&mut report, packet);
    validate_surface_bindings(&mut report, packet);
    validate_promotion(&mut report, packet);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_aspects(report: &mut ValidationReport, owner: &str, aspects: &[AspectEvidence]) {
    let mut seen = BTreeSet::new();
    for evidence in aspects {
        if !seen.insert(evidence.aspect) {
            report.push(
                "aspect.unique",
                format!("{owner} repeats aspect {}", evidence.aspect.as_str()),
            );
        }
        if evidence.evidence_state != EvidenceState::Missing && evidence.evidence_refs.is_empty() {
            report.push(
                "aspect.evidence_refs",
                format!(
                    "{owner} aspect {} must cite evidence unless it is missing",
                    evidence.aspect.as_str()
                ),
            );
        }
        if evidence.rationale.trim().is_empty() {
            report.push(
                "aspect.rationale",
                format!(
                    "{owner} aspect {} must carry a rationale",
                    evidence.aspect.as_str()
                ),
            );
        }
    }
    for required in CertificationAspect::ALL {
        if !seen.contains(&required) {
            report.push(
                "aspect.coverage",
                format!("{owner} must evidence aspect {}", required.as_str()),
            );
        }
    }
}

fn validate_row(report: &mut ValidationReport, certification_row: &CertificationRow) {
    if certification_row.row_id.trim().is_empty() {
        report.push("row.id", "row must carry a stable id");
    }
    if certification_row.target_class_label.trim().is_empty() {
        report.push(
            "row.target_class_label",
            format!(
                "row {} must carry a target-class label",
                certification_row.row_id
            ),
        );
    }
    if certification_row.backing_surface_classes.is_empty() {
        report.push(
            "row.backing_surface_classes",
            format!(
                "row {} must name its backing surface classes",
                certification_row.row_id
            ),
        );
    }
    if certification_row.why_this_certification.trim().is_empty() {
        report.push(
            "row.why_this_certification",
            format!(
                "row {} must carry a why-this-certification inspector line",
                certification_row.row_id
            ),
        );
    }
    if certification_row.consumer_refs.is_empty() {
        report.push(
            "row.consumer_refs",
            format!(
                "row {} must cite at least one consumer ref",
                certification_row.row_id
            ),
        );
    }
    if certification_row.notes.trim().is_empty() {
        report.push(
            "row.notes",
            format!(
                "row {} must carry a reviewer note",
                certification_row.row_id
            ),
        );
    }

    validate_aspects(
        report,
        &format!("row {}", certification_row.row_id),
        &certification_row.aspects,
    );

    // The stamped outcome must equal what the engine computes.
    let outcome = certify_environment_lane(
        certification_row.claimed_maturity,
        certification_row.claimed_warm_start_posture,
        &certification_row.aspects,
    );
    if certification_row.effective_maturity != outcome.effective_maturity {
        report.push(
            "row.effective_maturity",
            format!(
                "row {} effective_maturity {} disagrees with the engine ({})",
                certification_row.row_id,
                certification_row.effective_maturity.as_str(),
                outcome.effective_maturity.as_str()
            ),
        );
    }
    if certification_row.verdict != outcome.verdict {
        report.push(
            "row.verdict",
            format!(
                "row {} verdict {} disagrees with the engine ({})",
                certification_row.row_id,
                certification_row.verdict.as_str(),
                outcome.verdict.as_str()
            ),
        );
    }
    if certification_row.narrowed != outcome.narrowed {
        report.push(
            "row.narrowed",
            format!(
                "row {} narrowed flag disagrees with the engine",
                certification_row.row_id
            ),
        );
    }
    if certification_row.narrow_reason_tokens != outcome.narrow_reason_tokens {
        report.push(
            "row.narrow_reason_tokens",
            format!(
                "row {} narrow_reason_tokens disagree with the engine",
                certification_row.row_id
            ),
        );
    }
    if certification_row.stale_or_missing_aspect_tokens != outcome.stale_or_missing_aspect_tokens {
        report.push(
            "row.stale_or_missing_aspect_tokens",
            format!(
                "row {} stale_or_missing_aspect_tokens disagree with the engine",
                certification_row.row_id
            ),
        );
    }
    if certification_row.effective_warm_start_posture != outcome.effective_warm_start_posture {
        report.push(
            "row.effective_warm_start_posture",
            format!(
                "row {} effective_warm_start_posture {} disagrees with the engine ({})",
                certification_row.row_id,
                certification_row.effective_warm_start_posture.as_str(),
                outcome.effective_warm_start_posture.as_str()
            ),
        );
    }
    if certification_row.warm_start_downgraded != outcome.warm_start_downgraded {
        report.push(
            "row.warm_start_downgraded",
            format!(
                "row {} warm_start_downgraded flag disagrees with the engine",
                certification_row.row_id
            ),
        );
    }
    if certification_row.warm_start_downgrade_tokens != outcome.warm_start_downgrade_tokens {
        report.push(
            "row.warm_start_downgrade_tokens",
            format!(
                "row {} warm_start_downgrade_tokens disagree with the engine",
                certification_row.row_id
            ),
        );
    }

    let expected_support = supporting_evidence_refs(&certification_row.aspects);
    if certification_row.supporting_evidence_refs != expected_support {
        report.push(
            "row.supporting_evidence_refs",
            format!(
                "row {} supporting_evidence_refs must equal the union of its aspect evidence refs",
                certification_row.row_id
            ),
        );
    }
}

fn validate_freshness_rules(report: &mut ValidationReport, packet: &EnvCertificationPacket) {
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

fn validate_warm_start_rules(report: &mut ValidationReport, packet: &EnvCertificationPacket) {
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

fn validate_drills(report: &mut ValidationReport, packet: &EnvCertificationPacket) {
    if packet.drills.is_empty() {
        report.push(
            "packet.drills",
            "packet must declare failure/recovery drills",
        );
    }
    let mut drill_ids = BTreeSet::new();
    let mut drilled_target_classes = BTreeSet::new();
    let mut has_narrowed = false;
    let mut has_withheld = false;
    let mut has_warm_start_downgrade = false;
    for certification_drill in &packet.drills {
        if !drill_ids.insert(certification_drill.drill_id.as_str()) {
            report.push(
                "drill.id_unique",
                format!("duplicate drill_id {}", certification_drill.drill_id),
            );
        }
        drilled_target_classes.insert(certification_drill.target_class);

        // Recompute the degraded outcome from the engine.
        let degraded = degraded_aspects(
            certification_drill.exercised_aspect,
            certification_drill.degraded_evidence_state,
        );
        let degraded_outcome = certify_environment_lane(
            certification_drill.claimed_maturity,
            certification_drill.claimed_warm_start_posture,
            &degraded,
        );
        if certification_drill.expected_degraded_verdict != degraded_outcome.verdict {
            report.push(
                "drill.degraded_verdict",
                format!(
                    "drill {} degraded verdict disagrees with the engine",
                    certification_drill.drill_id
                ),
            );
        }
        if certification_drill.expected_degraded_maturity != degraded_outcome.effective_maturity {
            report.push(
                "drill.degraded_maturity",
                format!(
                    "drill {} degraded maturity disagrees with the engine",
                    certification_drill.drill_id
                ),
            );
        }
        if certification_drill.expected_degraded_warm_start_posture
            != degraded_outcome.effective_warm_start_posture
        {
            report.push(
                "drill.degraded_warm_start_posture",
                format!(
                    "drill {} degraded warm-start posture disagrees with the engine",
                    certification_drill.drill_id
                ),
            );
        }
        if certification_drill.blocks_promotion_while_degraded
            != (degraded_outcome.verdict == RowVerdict::Withheld)
        {
            report.push(
                "drill.blocks_promotion",
                format!(
                    "drill {} blocks_promotion_while_degraded disagrees with the engine",
                    certification_drill.drill_id
                ),
            );
        }
        if degraded_outcome.verdict == RowVerdict::Certified {
            report.push(
                "drill.must_degrade",
                format!(
                    "drill {} must inject a failure that actually narrows or withholds",
                    certification_drill.drill_id
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
        if certification_drill.recovers_to_verdict != RowVerdict::Certified {
            report.push(
                "drill.recovers",
                format!(
                    "drill {} must recover to certified",
                    certification_drill.drill_id
                ),
            );
        }
        if !certification_drill.asserts_claim_narrows_under_failure
            || !certification_drill.asserts_recovers_after_refresh
        {
            report.push(
                "drill.assertions",
                format!(
                    "drill {} must assert it narrows under failure and recovers after refresh",
                    certification_drill.drill_id
                ),
            );
        }
        validate_drill_steps(report, certification_drill);
    }
    for required in TargetClass::ALL {
        if !drilled_target_classes.contains(&required) {
            report.push(
                "packet.drilled_target_class",
                format!("packet must drill target class {}", required.as_str()),
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

fn validate_drill_steps(report: &mut ValidationReport, certification_drill: &CertificationDrill) {
    if certification_drill.steps.is_empty() {
        report.push(
            "drill.steps",
            format!("drill {} must declare steps", certification_drill.drill_id),
        );
        return;
    }
    if certification_drill.steps.first().map(|s| s.phase) != Some(DrillPhase::Inject) {
        report.push(
            "drill.first_phase",
            format!(
                "drill {} must begin with an inject step",
                certification_drill.drill_id
            ),
        );
    }
    if certification_drill.steps.last().map(|s| s.phase) != Some(DrillPhase::Verify) {
        report.push(
            "drill.last_phase",
            format!(
                "drill {} must end with a verify step",
                certification_drill.drill_id
            ),
        );
    }
    let has_narrow = certification_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Narrow);
    let has_recover = certification_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Recover);
    if !has_narrow || !has_recover {
        report.push(
            "drill.phases",
            format!(
                "drill {} must include a narrow step and a recover step",
                certification_drill.drill_id
            ),
        );
    }
    for (index, drill_step) in certification_drill.steps.iter().enumerate() {
        if drill_step.narration.trim().is_empty() {
            report.push(
                "drill.step_narration",
                format!(
                    "drill {} step {index} must narrate",
                    certification_drill.drill_id
                ),
            );
        }
    }
}

fn validate_surface_bindings(report: &mut ValidationReport, packet: &EnvCertificationPacket) {
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

fn validate_promotion(report: &mut ValidationReport, packet: &EnvCertificationPacket) {
    let expected = promotion_summary(&packet.rows);
    if packet.promotion.promotion_blocked != expected.promotion_blocked {
        report.push(
            "promotion.blocked",
            "promotion_blocked disagrees with the rolled-up row verdicts",
        );
    }
    if packet.promotion.certified_target_class_tokens != expected.certified_target_class_tokens {
        report.push(
            "promotion.certified",
            "certified_target_class_tokens disagree with the rolled-up row verdicts",
        );
    }
    if packet.promotion.narrowed_target_class_tokens != expected.narrowed_target_class_tokens {
        report.push(
            "promotion.narrowed",
            "narrowed_target_class_tokens disagree with the rolled-up row verdicts",
        );
    }
    if packet.promotion.held_target_class_tokens != expected.held_target_class_tokens {
        report.push(
            "promotion.held",
            "held_target_class_tokens disagree with the rolled-up row verdicts",
        );
    }
    if packet.promotion.summary.trim().is_empty() {
        report.push("promotion.summary", "promotion must carry a summary");
    }
}

/// Validates one checked-in fixture against the frozen contract.
pub fn validate_env_certification_fixture(
    fixture: &EnvCertificationFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != ENV_CERTIFICATION_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != ENV_CERTIFICATION_SCHEMA_VERSION {
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

    validate_aspects(
        &mut report,
        &format!("fixture {}", fixture.fixture_id),
        &fixture.observed_aspects,
    );

    let outcome = certify_environment_lane(
        fixture.claimed_maturity,
        fixture.claimed_warm_start_posture,
        &fixture.observed_aspects,
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
    if fixture.blocks_promotion != (outcome.verdict == RowVerdict::Withheld) {
        report.push(
            "fixture.blocks_promotion",
            format!(
                "fixture {} blocks_promotion disagrees with the engine",
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
