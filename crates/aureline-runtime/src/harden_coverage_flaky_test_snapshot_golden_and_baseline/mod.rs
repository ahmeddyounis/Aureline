//! Coverage / flaky-test / snapshot-golden / baseline-truth packet for
//! the M4 stable lane.
//!
//! This module pins how the coverage, flaky-triage, snapshot/golden,
//! and baseline-truth surfaces serialize one canonical truth across
//! their four wedges (`stability_verdict_separation`,
//! `quarantine_mute_renewal_truth`,
//! `ai_candidate_source_attribution`, `coverage_impact_truth`). Stable
//! claims MUST keep the stability verdict separated from the
//! quarantine-versus-mute state; muted and quarantined tests MUST
//! remain visible, filterable, countable, and exportable with explicit
//! renewal / expiry / removal semantics rather than hiding indefinitely;
//! AI-generated tests, automated baseline changes, and imported CI
//! evidence MUST stay attached to the same session/attempt and review
//! checkpoint lineage; and coverage impact derived from AI-generated or
//! sandbox-run candidate tests MUST stay explicitly `measured`,
//! `estimated`, `stale`, or `not_comparable` per target/environment
//! family so a single passing run never silently promotes a candidate
//! into trusted stable coverage proof.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `coverage_flaky_snapshot_baseline_quality` row cannot prove:
//!
//! - the four wedges (`stability_verdict_separation`,
//!   `quarantine_mute_renewal_truth`,
//!   `ai_candidate_source_attribution`, `coverage_impact_truth`) each
//!   have a structured `wedge_admission` row,
//! - the six stability-verdict classes (`stable`, `flaky`, `failing`,
//!   `quarantined`, `muted`, `unknown`) each have a structured
//!   `stability_verdict_admission` row so stability cannot collapse
//!   into a coarse pass/fail bit,
//! - the four quarantine-mute states (`active`, `expiring_soon`,
//!   `expired_pending_renewal`, `removed`) each have a structured
//!   `quarantine_mute_state_admission` row so quarantine / mute carry
//!   renewal / expiry / removal semantics,
//! - the four test-source classes (`human_authored`,
//!   `candidate_ai_test`, `automated_baseline`,
//!   `imported_ci_evidence`) each have a structured
//!   `test_source_admission` row so AI-generated tests, automated
//!   baseline mutations, ordinary human-authored tests, and imported CI
//!   evidence never collapse into a single promotion narrative,
//! - the four coverage-impact classes (`measured`, `estimated`,
//!   `stale`, `not_comparable`) each have a structured
//!   `coverage_impact_admission` row so candidate test coverage cannot
//!   silently upgrade,
//! - the three candidate-lineage classes (`session_attempt_bound`,
//!   `review_checkpoint_bound`, `imported_ci_bound`) each have a
//!   structured `candidate_lineage_admission` row so AI tests,
//!   baseline mutations, and imported CI evidence attach to the same
//!   session/attempt + review-checkpoint lineage before they can
//!   influence promotion,
//! - the five consumer-surface bindings (`coverage_surface`,
//!   `flaky_triage_surface`, `snapshot_golden_surface`,
//!   `baseline_surface`, `release_packet_surface`) each carry a
//!   `consumer_surface_binding` row attesting the stability-verdict,
//!   quarantine-mute-state, test-source, coverage-impact, and
//!   candidate-lineage vocabularies they are required to preserve,
//! - one stable `execution_context_id` (or equivalent lineage object)
//!   threads through every certified lane.
//!
//! Every row binds a closed `coverage_quality_lane_class`,
//! `coverage_quality_row_class`, `support_class`, `wedge_class`,
//! `stability_verdict_class`, `quarantine_mute_state_class`,
//! `test_source_class`, `coverage_impact_class`,
//! `candidate_lineage_class`, `consumer_surface_class`,
//! `evidence_class`, `known_limit_class`, `downgrade_automation_class`,
//! and `coverage_quality_confidence_class` plus an `evidence_refs`
//! array and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or binds
//! a non-`none` downgrade automation.
//!
//! The packet is metadata-only — it never admits raw test bodies, raw
//! coverage payloads, raw snapshot byte streams, raw baseline diffs,
//! raw runner scrollback, raw command lines, raw process environment
//! bytes, secrets, or ambient credentials past the boundary. A row
//! that claims `launch_stable` while leaving its known limit,
//! downgrade automation, or evidence class unbound is refused; the
//! validator narrows below launch-stable instead of inheriting an
//! adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`CoverageQualityTruthPacket`].
pub const COVERAGE_QUALITY_TRUTH_PACKET_RECORD_KIND: &str =
    "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_stable_packet";

/// Stable record-kind tag for [`CoverageQualityTruthSupportExport`].
pub const COVERAGE_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_support_export";

/// Integer schema version for the coverage-quality truth packet.
pub const COVERAGE_QUALITY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const COVERAGE_QUALITY_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const COVERAGE_QUALITY_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/harden-coverage-flaky-test-snapshot-golden-and-baseline.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const COVERAGE_QUALITY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/harden-coverage-flaky-test-snapshot-golden-and-baseline.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const COVERAGE_QUALITY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/harden_coverage_flaky_test_snapshot_golden_and_baseline";

/// Repo-relative path of the checked-in stable packet.
pub const COVERAGE_QUALITY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.json";

/// Closed coverage-quality lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageQualityLaneClass {
    /// Coverage truth lane (line/branch/file coverage and delta).
    CoverageLane,
    /// Flaky-test triage lane (verdicts, retries, attempt history).
    FlakyTestLane,
    /// Snapshot / golden-file truth lane (recorded baselines, diffs).
    SnapshotGoldenLane,
    /// Baseline-truth lane (mutation, governance, promotion).
    BaselineTruthLane,
}

impl CoverageQualityLaneClass {
    /// Every required lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::CoverageLane,
        Self::FlakyTestLane,
        Self::SnapshotGoldenLane,
        Self::BaselineTruthLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageLane => "coverage_lane",
            Self::FlakyTestLane => "flaky_test_lane",
            Self::SnapshotGoldenLane => "snapshot_golden_lane",
            Self::BaselineTruthLane => "baseline_truth_lane",
        }
    }
}

/// Closed coverage-quality row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageQualityRowClass {
    /// The lane's headline coverage / flaky / snapshot / baseline
    /// quality row.
    CoverageFlakySnapshotBaselineQuality,
    /// A row admitting one of the four wedges.
    WedgeAdmission,
    /// A row admitting one stability-verdict class (`stable`, `flaky`,
    /// `failing`, `quarantined`, `muted`, `unknown`).
    StabilityVerdictAdmission,
    /// A row admitting one quarantine-mute state class (`active`,
    /// `expiring_soon`, `expired_pending_renewal`, `removed`).
    QuarantineMuteStateAdmission,
    /// A row admitting one test-source class (`human_authored`,
    /// `candidate_ai_test`, `automated_baseline`,
    /// `imported_ci_evidence`).
    TestSourceAdmission,
    /// A row admitting one coverage-impact class (`measured`,
    /// `estimated`, `stale`, `not_comparable`).
    CoverageImpactAdmission,
    /// A row admitting one candidate-lineage class
    /// (`session_attempt_bound`, `review_checkpoint_bound`,
    /// `imported_ci_bound`).
    CandidateLineageAdmission,
    /// A row binding one consumer surface (`coverage_surface`,
    /// `flaky_triage_surface`, `snapshot_golden_surface`,
    /// `baseline_surface`, `release_packet_surface`) and attesting that
    /// the surface preserves the vocabularies it is required to
    /// preserve.
    ConsumerSurfaceBinding,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into emitted coverage-quality truth and
    /// downstream consumer surfaces.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl CoverageQualityRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageFlakySnapshotBaselineQuality => {
                "coverage_flaky_snapshot_baseline_quality"
            }
            Self::WedgeAdmission => "wedge_admission",
            Self::StabilityVerdictAdmission => "stability_verdict_admission",
            Self::QuarantineMuteStateAdmission => "quarantine_mute_state_admission",
            Self::TestSourceAdmission => "test_source_admission",
            Self::CoverageImpactAdmission => "coverage_impact_admission",
            Self::CandidateLineageAdmission => "candidate_lineage_admission",
            Self::ConsumerSurfaceBinding => "consumer_surface_binding",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound wedge.
    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    /// True when this row class requires a bound stability verdict.
    pub const fn requires_stability_verdict(self) -> bool {
        matches!(self, Self::StabilityVerdictAdmission)
    }

    /// True when this row class requires a bound quarantine-mute state.
    pub const fn requires_quarantine_mute_state(self) -> bool {
        matches!(self, Self::QuarantineMuteStateAdmission)
    }

    /// True when this row class requires a bound test-source class.
    pub const fn requires_test_source(self) -> bool {
        matches!(self, Self::TestSourceAdmission)
    }

    /// True when this row class requires a bound coverage-impact class.
    pub const fn requires_coverage_impact(self) -> bool {
        matches!(self, Self::CoverageImpactAdmission)
    }

    /// True when this row class requires a bound candidate-lineage class.
    pub const fn requires_candidate_lineage(self) -> bool {
        matches!(self, Self::CandidateLineageAdmission)
    }

    /// True when this row class requires a bound consumer surface.
    pub const fn requires_consumer_surface(self) -> bool {
        matches!(self, Self::ConsumerSurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to a coverage-quality row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-stable grade for the lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable.
    LaunchStableBelow,
    /// Row is at beta-grade only.
    BetaGradeOnly,
    /// Row is at preview only.
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl SupportClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchStable => "launch_stable",
            Self::LaunchStableBelow => "launch_stable_below",
            Self::BetaGradeOnly => "beta_grade_only",
            Self::PreviewOnly => "preview_only",
            Self::Unsupported => "unsupported",
            Self::SupportUnbound => "support_unbound",
        }
    }

    /// True when this support class satisfies the support-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    /// True when the support class must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::LaunchStable)
    }
}

/// Closed coverage-quality wedge vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `wedge_admission` row for each
/// required wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Stability-verdict separation wedge — explicit stability verdict
    /// separated from quarantine-versus-mute state.
    StabilityVerdictSeparation,
    /// Quarantine / mute renewal-truth wedge — muted and quarantined
    /// tests remain visible, filterable, countable, exportable with
    /// renewal / expiry / removal semantics.
    QuarantineMuteRenewalTruth,
    /// AI / candidate source attribution wedge — distinct
    /// `candidate_ai_test`, `human_authored`, `automated_baseline`,
    /// `imported_ci_evidence` source classes.
    AiCandidateSourceAttribution,
    /// Coverage impact truth wedge — coverage delta classified
    /// `measured`, `estimated`, `stale`, `not_comparable` per
    /// target/environment family.
    CoverageImpactTruth,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::StabilityVerdictSeparation,
        Self::QuarantineMuteRenewalTruth,
        Self::AiCandidateSourceAttribution,
        Self::CoverageImpactTruth,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StabilityVerdictSeparation => "stability_verdict_separation",
            Self::QuarantineMuteRenewalTruth => "quarantine_mute_renewal_truth",
            Self::AiCandidateSourceAttribution => "ai_candidate_source_attribution",
            Self::CoverageImpactTruth => "coverage_impact_truth",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed stability-verdict vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `stability_verdict_admission` row for
/// each required verdict so the verdict cannot collapse into a coarse
/// pass / fail bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilityVerdictClass {
    /// `stable` — verdict: consistently passing within the verdict
    /// window.
    Stable,
    /// `flaky` — verdict: known-flaky within the verdict window.
    Flaky,
    /// `failing` — verdict: currently failing within the verdict window.
    Failing,
    /// `quarantined` — verdict-orthogonal: case is quarantined; the
    /// underlying verdict (stable / flaky / failing) is still
    /// observable independently.
    Quarantined,
    /// `muted` — verdict-orthogonal: case output is muted; the
    /// underlying verdict is still observable independently.
    Muted,
    /// `unknown` — verdict: no signal yet (e.g. new case, no recent
    /// run).
    Unknown,
    /// The row is not bound to a stability verdict.
    NotApplicable,
}

impl StabilityVerdictClass {
    /// Every required stability verdict for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::Stable,
        Self::Flaky,
        Self::Failing,
        Self::Quarantined,
        Self::Muted,
        Self::Unknown,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Flaky => "flaky",
            Self::Failing => "failing",
            Self::Quarantined => "quarantined",
            Self::Muted => "muted",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed quarantine-mute state vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `quarantine_mute_state_admission` row
/// for each required state so quarantine and mute carry renewal /
/// expiry / removal semantics instead of indefinite hidden debt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineMuteStateClass {
    /// `active` — quarantine or mute is in force; renewal window
    /// open.
    Active,
    /// `expiring_soon` — quarantine or mute renewal window is within
    /// the documented warning horizon.
    ExpiringSoon,
    /// `expired_pending_renewal` — quarantine or mute renewal has
    /// lapsed; the row MUST be renewed or removed before the lane can
    /// re-certify.
    ExpiredPendingRenewal,
    /// `removed` — quarantine or mute has been lifted; the case
    /// returns to ordinary stability-verdict accounting.
    Removed,
    /// The row is not bound to a quarantine-mute state.
    NotApplicable,
}

impl QuarantineMuteStateClass {
    /// Every required quarantine-mute state for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::Active,
        Self::ExpiringSoon,
        Self::ExpiredPendingRenewal,
        Self::Removed,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiringSoon => "expiring_soon",
            Self::ExpiredPendingRenewal => "expired_pending_renewal",
            Self::Removed => "removed",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed test-source vocabulary. Every lane claiming `launch_stable`
/// MUST publish a `test_source_admission` row for each required source
/// class so AI-generated tests, automated baseline mutations, ordinary
/// human-authored tests, and imported CI evidence never collapse into
/// the same promotion narrative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestSourceClass {
    /// `human_authored` — ordinary human-authored tests, baselines, or
    /// coverage proof.
    HumanAuthored,
    /// `candidate_ai_test` — AI-proposed test or AI-proposed baseline
    /// mutation in the candidate state.
    CandidateAiTest,
    /// `automated_baseline` — automated baseline change emitted by a
    /// reviewer-blessed automation (not promoted from a candidate).
    AutomatedBaseline,
    /// `imported_ci_evidence` — coverage / flaky / snapshot / baseline
    /// signal imported from an external CI system.
    ImportedCiEvidence,
    /// The row is not bound to a test-source class.
    NotApplicable,
}

impl TestSourceClass {
    /// Every required test-source class for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::HumanAuthored,
        Self::CandidateAiTest,
        Self::AutomatedBaseline,
        Self::ImportedCiEvidence,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanAuthored => "human_authored",
            Self::CandidateAiTest => "candidate_ai_test",
            Self::AutomatedBaseline => "automated_baseline",
            Self::ImportedCiEvidence => "imported_ci_evidence",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this source class requires session/attempt + review
    /// checkpoint lineage attestation before it can influence
    /// promotion.
    pub const fn requires_candidate_lineage(self) -> bool {
        matches!(
            self,
            Self::CandidateAiTest | Self::AutomatedBaseline | Self::ImportedCiEvidence
        )
    }
}

/// Closed coverage-impact vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `coverage_impact_admission` row for
/// each required impact class so a single passing AI / sandbox run
/// cannot silently upgrade a candidate into trusted stable coverage
/// proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageImpactClass {
    /// `measured` — coverage impact was measured against the same
    /// target/environment family as the trusted stable baseline.
    Measured,
    /// `estimated` — coverage impact was estimated; either the
    /// target/environment family differs or the sample size is too
    /// small for trusted measurement.
    Estimated,
    /// `stale` — coverage impact is older than the documented
    /// freshness window for the target/environment family.
    Stale,
    /// `not_comparable` — coverage impact cannot be compared (e.g.
    /// candidate suite changes the instrumentation contract).
    NotComparable,
    /// The row is not bound to a coverage-impact class.
    NotApplicable,
}

impl CoverageImpactClass {
    /// Every required coverage impact for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::Measured,
        Self::Estimated,
        Self::Stale,
        Self::NotComparable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measured => "measured",
            Self::Estimated => "estimated",
            Self::Stale => "stale",
            Self::NotComparable => "not_comparable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed candidate-lineage vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `candidate_lineage_admission` row for
/// each required lineage class so AI tests, baseline mutations, and
/// imported CI evidence attach to the same session/attempt + review
/// checkpoint lineage before they can influence promotion or claim
/// packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateLineageClass {
    /// `session_attempt_bound` — candidate is bound to the same
    /// session/attempt id as the run that produced it.
    SessionAttemptBound,
    /// `review_checkpoint_bound` — candidate is bound to a review
    /// checkpoint id (human or automated review).
    ReviewCheckpointBound,
    /// `imported_ci_bound` — imported CI evidence is bound to a
    /// stable importer attestation id.
    ImportedCiBound,
    /// The row is not bound to a candidate-lineage class.
    NotApplicable,
}

impl CandidateLineageClass {
    /// Every required candidate-lineage class for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 3] = [
        Self::SessionAttemptBound,
        Self::ReviewCheckpointBound,
        Self::ImportedCiBound,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionAttemptBound => "session_attempt_bound",
            Self::ReviewCheckpointBound => "review_checkpoint_bound",
            Self::ImportedCiBound => "imported_ci_bound",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed consumer-surface vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `consumer_surface_binding` row for
/// each required surface so the stability-verdict, quarantine-mute,
/// test-source, coverage-impact, and candidate-lineage vocabularies
/// survive into product chrome, export bundles, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceBindingClass {
    /// Coverage surface (coverage map, gutter, file/branch coverage,
    /// coverage delta panel).
    CoverageSurface,
    /// Flaky-triage surface (verdict + quarantine/mute filter chips,
    /// renewal warnings).
    FlakyTriageSurface,
    /// Snapshot / golden surface (review diff, accept / reject /
    /// promote).
    SnapshotGoldenSurface,
    /// Baseline surface (baseline mutation review, AI baseline
    /// candidacy chrome).
    BaselineSurface,
    /// Release packet surface (release / support packet that separates
    /// human vs AI tests, measured vs estimated coverage, ordinary
    /// baseline vs AI-proposed baseline).
    ReleasePacketSurface,
    /// The row is not bound to a consumer surface.
    NotApplicable,
}

impl ConsumerSurfaceBindingClass {
    /// Every required consumer surface for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::CoverageSurface,
        Self::FlakyTriageSurface,
        Self::SnapshotGoldenSurface,
        Self::BaselineSurface,
        Self::ReleasePacketSurface,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageSurface => "coverage_surface",
            Self::FlakyTriageSurface => "flaky_triage_surface",
            Self::SnapshotGoldenSurface => "snapshot_golden_surface",
            Self::BaselineSurface => "baseline_surface",
            Self::ReleasePacketSurface => "release_packet_surface",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this surface MUST attest that it preserves the
    /// stability-verdict vocabulary.
    pub const fn requires_stability_verdict_attestation(self) -> bool {
        matches!(self, Self::FlakyTriageSurface | Self::ReleasePacketSurface)
    }

    /// True when this surface MUST attest that it preserves the
    /// quarantine-mute state vocabulary (renewal / expiry / removal).
    pub const fn requires_quarantine_mute_state_attestation(self) -> bool {
        matches!(self, Self::FlakyTriageSurface | Self::ReleasePacketSurface)
    }

    /// True when this surface MUST attest that it preserves the
    /// test-source vocabulary (human / AI / automated baseline /
    /// imported CI).
    pub const fn requires_test_source_attestation(self) -> bool {
        matches!(
            self,
            Self::CoverageSurface
                | Self::FlakyTriageSurface
                | Self::SnapshotGoldenSurface
                | Self::BaselineSurface
                | Self::ReleasePacketSurface
        )
    }

    /// True when this surface MUST attest that it preserves the
    /// coverage-impact vocabulary (`measured`, `estimated`, `stale`,
    /// `not_comparable`).
    pub const fn requires_coverage_impact_attestation(self) -> bool {
        matches!(self, Self::CoverageSurface | Self::ReleasePacketSurface)
    }

    /// True when this surface MUST attest that it preserves the
    /// candidate-lineage vocabulary.
    pub const fn requires_candidate_lineage_attestation(self) -> bool {
        matches!(
            self,
            Self::BaselineSurface | Self::SnapshotGoldenSurface | Self::ReleasePacketSurface
        )
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// The row is backed by an automated functional / unit suite.
    AutomatedFunctionalEvidence,
    /// The row is backed by a conformance / interoperability suite.
    ConformanceSuiteEvidence,
    /// The row is backed by a failure / recovery drill.
    FailureRecoveryDrillEvidence,
    /// The row is backed by design-QA / UX validation.
    DesignQaEvidence,
    /// The row is backed by release-evidence review.
    ReleaseEvidenceReview,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a benchmark / fitness-function capture.
    BenchmarkEvidence,
    /// The row is backed by a docs/help disclosure (gap label only).
    DocsDisclosureEvidence,
    /// The row has no bound evidence class; this never qualifies stable.
    EvidenceUnbound,
}

impl EvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::DesignQaEvidence => "design_qa_evidence",
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::BenchmarkEvidence => "benchmark_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this evidence class satisfies the evidence-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary attached to a coverage-quality row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the coverage subset.
    CoverageLaneSubsetOnly,
    /// The lane only certifies the flaky-triage subset.
    FlakyTestLaneSubsetOnly,
    /// The lane only certifies the snapshot / golden subset.
    SnapshotGoldenLaneSubsetOnly,
    /// The lane only certifies the baseline-truth subset.
    BaselineTruthLaneSubsetOnly,
    /// The lane only certifies a subset of the four wedges.
    WedgeAdmissionSubsetOnly,
    /// The lane only certifies a subset of the six stability verdicts.
    StabilityVerdictSubsetOnly,
    /// The lane only certifies a subset of the four quarantine-mute
    /// states.
    QuarantineMuteStateSubsetOnly,
    /// The lane only certifies a subset of the four test-source
    /// classes.
    TestSourceSubsetOnly,
    /// The lane only certifies a subset of the four coverage-impact
    /// classes.
    CoverageImpactSubsetOnly,
    /// The lane only certifies a subset of the three candidate-lineage
    /// classes.
    CandidateLineageSubsetOnly,
    /// The lane only certifies a subset of the five consumer surfaces.
    ConsumerSurfaceSubsetOnly,
    /// The lane reports stability-verdict attestation skew on one or
    /// more surfaces.
    StabilityVerdictAttestationSkewDeclared,
    /// The lane reports quarantine-mute attestation skew on one or
    /// more surfaces.
    QuarantineMuteAttestationSkewDeclared,
    /// The lane reports test-source attestation skew on one or more
    /// surfaces.
    TestSourceAttestationSkewDeclared,
    /// The lane reports coverage-impact attestation skew on one or
    /// more surfaces.
    CoverageImpactAttestationSkewDeclared,
    /// The lane reports candidate-lineage attestation skew on one or
    /// more surfaces.
    CandidateLineageAttestationSkewDeclared,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known limit class; this never qualifies
    /// stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::CoverageLaneSubsetOnly => "coverage_lane_subset_only",
            Self::FlakyTestLaneSubsetOnly => "flaky_test_lane_subset_only",
            Self::SnapshotGoldenLaneSubsetOnly => "snapshot_golden_lane_subset_only",
            Self::BaselineTruthLaneSubsetOnly => "baseline_truth_lane_subset_only",
            Self::WedgeAdmissionSubsetOnly => "wedge_admission_subset_only",
            Self::StabilityVerdictSubsetOnly => "stability_verdict_subset_only",
            Self::QuarantineMuteStateSubsetOnly => "quarantine_mute_state_subset_only",
            Self::TestSourceSubsetOnly => "test_source_subset_only",
            Self::CoverageImpactSubsetOnly => "coverage_impact_subset_only",
            Self::CandidateLineageSubsetOnly => "candidate_lineage_subset_only",
            Self::ConsumerSurfaceSubsetOnly => "consumer_surface_subset_only",
            Self::StabilityVerdictAttestationSkewDeclared => {
                "stability_verdict_attestation_skew_declared"
            }
            Self::QuarantineMuteAttestationSkewDeclared => {
                "quarantine_mute_attestation_skew_declared"
            }
            Self::TestSourceAttestationSkewDeclared => "test_source_attestation_skew_declared",
            Self::CoverageImpactAttestationSkewDeclared => {
                "coverage_impact_attestation_skew_declared"
            }
            Self::CandidateLineageAttestationSkewDeclared => {
                "candidate_lineage_attestation_skew_declared"
            }
            Self::BetaCapabilitySampleOnly => "beta_capability_sample_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// True when this known-limit class satisfies the limit-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// True when this known-limit class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}

/// Closed downgrade-automation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a required wedge admission is missing.
    AutoNarrowOnWedgeAdmissionGap,
    /// Automatically narrow when a required stability-verdict
    /// admission is missing.
    AutoNarrowOnStabilityVerdictGap,
    /// Automatically narrow when a required quarantine-mute state
    /// admission is missing.
    AutoNarrowOnQuarantineMuteStateGap,
    /// Automatically narrow when a required test-source admission is
    /// missing.
    AutoNarrowOnTestSourceGap,
    /// Automatically narrow when a required coverage-impact admission
    /// is missing.
    AutoNarrowOnCoverageImpactGap,
    /// Automatically narrow when a required candidate-lineage
    /// admission is missing.
    AutoNarrowOnCandidateLineageGap,
    /// Automatically narrow when a required consumer-surface binding
    /// is missing.
    AutoNarrowOnConsumerSurfaceGap,
    /// Automatically narrow when a consumer-surface row drops a
    /// required stability-verdict attestation.
    AutoNarrowOnStabilityVerdictAttestationGap,
    /// Automatically narrow when a consumer-surface row drops a
    /// required quarantine-mute attestation.
    AutoNarrowOnQuarantineMuteAttestationGap,
    /// Automatically narrow when a consumer-surface row drops a
    /// required test-source attestation.
    AutoNarrowOnTestSourceAttestationGap,
    /// Automatically narrow when a consumer-surface row drops a
    /// required coverage-impact attestation.
    AutoNarrowOnCoverageImpactAttestationGap,
    /// Automatically narrow when a consumer-surface row drops a
    /// required candidate-lineage attestation.
    AutoNarrowOnCandidateLineageAttestationGap,
    /// Automatically narrow when the lineage object breaks
    /// (`execution_context_id` does not thread through emitted truth).
    AutoNarrowOnLineageBreak,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnWedgeAdmissionGap => "auto_narrow_on_wedge_admission_gap",
            Self::AutoNarrowOnStabilityVerdictGap => "auto_narrow_on_stability_verdict_gap",
            Self::AutoNarrowOnQuarantineMuteStateGap => "auto_narrow_on_quarantine_mute_state_gap",
            Self::AutoNarrowOnTestSourceGap => "auto_narrow_on_test_source_gap",
            Self::AutoNarrowOnCoverageImpactGap => "auto_narrow_on_coverage_impact_gap",
            Self::AutoNarrowOnCandidateLineageGap => "auto_narrow_on_candidate_lineage_gap",
            Self::AutoNarrowOnConsumerSurfaceGap => "auto_narrow_on_consumer_surface_gap",
            Self::AutoNarrowOnStabilityVerdictAttestationGap => {
                "auto_narrow_on_stability_verdict_attestation_gap"
            }
            Self::AutoNarrowOnQuarantineMuteAttestationGap => {
                "auto_narrow_on_quarantine_mute_attestation_gap"
            }
            Self::AutoNarrowOnTestSourceAttestationGap => {
                "auto_narrow_on_test_source_attestation_gap"
            }
            Self::AutoNarrowOnCoverageImpactAttestationGap => {
                "auto_narrow_on_coverage_impact_attestation_gap"
            }
            Self::AutoNarrowOnCandidateLineageAttestationGap => {
                "auto_narrow_on_candidate_lineage_attestation_gap"
            }
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding
    /// invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit
    /// disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for a coverage-quality row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageQualityConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until
    /// evidence grows.
    LowConfidence,
}

impl CoverageQualityConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
        }
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet narrows below stable.
    NarrowedBelowStable,
    /// Packet has a blocker finding.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required lane has no row.
    MissingLaneCoverage,
    /// A lane claiming launch_stable is missing a required wedge
    /// admission.
    MissingWedgeAdmissionCoverage,
    /// A lane claiming launch_stable is missing a required
    /// stability-verdict admission.
    MissingStabilityVerdictCoverage,
    /// A lane claiming launch_stable is missing a required
    /// quarantine-mute state admission.
    MissingQuarantineMuteStateCoverage,
    /// A lane claiming launch_stable is missing a required test-source
    /// admission.
    MissingTestSourceCoverage,
    /// A lane claiming launch_stable is missing a required
    /// coverage-impact admission.
    MissingCoverageImpactCoverage,
    /// A lane claiming launch_stable is missing a required
    /// candidate-lineage admission.
    MissingCandidateLineageCoverage,
    /// A lane claiming launch_stable is missing a required
    /// consumer-surface binding.
    MissingConsumerSurfaceCoverage,
    /// A lane claiming launch_stable is missing the required lineage
    /// admission row.
    MissingLineageAdmission,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row claims launch_stable while one or more bindings is
    /// unbound.
    LaunchStableWithUnboundBinding,
    /// A row narrowed below launch_stable drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A wedge-admission row drops its wedge binding.
    WedgeNotApplicable,
    /// A non-wedge row binds a wedge it cannot certify.
    WedgeNotPermittedOnRowClass,
    /// A stability-verdict-admission row drops its verdict binding.
    StabilityVerdictNotApplicable,
    /// A non-stability-verdict row binds a verdict it cannot certify.
    StabilityVerdictNotPermittedOnRowClass,
    /// A quarantine-mute-state-admission row drops its state binding.
    QuarantineMuteStateNotApplicable,
    /// A non-quarantine-mute-state row binds a state it cannot certify.
    QuarantineMuteStateNotPermittedOnRowClass,
    /// A test-source-admission row drops its source binding.
    TestSourceNotApplicable,
    /// A non-test-source row binds a source it cannot certify.
    TestSourceNotPermittedOnRowClass,
    /// A coverage-impact-admission row drops its impact binding.
    CoverageImpactNotApplicable,
    /// A non-coverage-impact row binds an impact it cannot certify.
    CoverageImpactNotPermittedOnRowClass,
    /// A candidate-lineage-admission row drops its lineage binding.
    CandidateLineageNotApplicable,
    /// A non-candidate-lineage row binds a lineage it cannot certify.
    CandidateLineageNotPermittedOnRowClass,
    /// A consumer-surface-binding row drops its surface binding.
    ConsumerSurfaceNotApplicable,
    /// A non-consumer-surface row binds a surface it cannot certify.
    ConsumerSurfaceNotPermittedOnRowClass,
    /// A consumer-surface row fails to attest the stability-verdict
    /// vocabulary it must preserve.
    ConsumerSurfaceMissingStabilityVerdictAttestation,
    /// A consumer-surface row fails to attest the quarantine-mute
    /// vocabulary it must preserve.
    ConsumerSurfaceMissingQuarantineMuteAttestation,
    /// A consumer-surface row fails to attest the test-source
    /// vocabulary it must preserve.
    ConsumerSurfaceMissingTestSourceAttestation,
    /// A consumer-surface row fails to attest the coverage-impact
    /// vocabulary it must preserve.
    ConsumerSurfaceMissingCoverageImpactAttestation,
    /// A consumer-surface row fails to attest the candidate-lineage
    /// vocabulary it must preserve.
    ConsumerSurfaceMissingCandidateLineageAttestation,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// A test-source-admission row in a candidate / automated /
    /// imported source class declares it does not attach to the
    /// required session/attempt + review-checkpoint lineage.
    CandidateSourceNotLineageBound,
    /// A row admits raw test bodies, raw coverage payloads, raw
    /// snapshot byte streams, raw baseline diffs, raw runner
    /// scrollback bodies, raw command lines, or raw process
    /// environment bytes past the boundary.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the wedge vocabulary.
    WedgeVocabularyCollapsed,
    /// A projection collapses the stability-verdict vocabulary.
    StabilityVerdictVocabularyCollapsed,
    /// A projection collapses the quarantine-mute-state vocabulary.
    QuarantineMuteStateVocabularyCollapsed,
    /// A projection collapses the test-source vocabulary.
    TestSourceVocabularyCollapsed,
    /// A projection collapses the coverage-impact vocabulary.
    CoverageImpactVocabularyCollapsed,
    /// A projection collapses the candidate-lineage vocabulary.
    CandidateLineageVocabularyCollapsed,
    /// A projection collapses the consumer-surface vocabulary.
    ConsumerSurfaceVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::MissingWedgeAdmissionCoverage => "missing_wedge_admission_coverage",
            Self::MissingStabilityVerdictCoverage => "missing_stability_verdict_coverage",
            Self::MissingQuarantineMuteStateCoverage => "missing_quarantine_mute_state_coverage",
            Self::MissingTestSourceCoverage => "missing_test_source_coverage",
            Self::MissingCoverageImpactCoverage => "missing_coverage_impact_coverage",
            Self::MissingCandidateLineageCoverage => "missing_candidate_lineage_coverage",
            Self::MissingConsumerSurfaceCoverage => "missing_consumer_surface_coverage",
            Self::MissingLineageAdmission => "missing_lineage_admission",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::WedgeNotApplicable => "wedge_not_applicable",
            Self::WedgeNotPermittedOnRowClass => "wedge_not_permitted_on_row_class",
            Self::StabilityVerdictNotApplicable => "stability_verdict_not_applicable",
            Self::StabilityVerdictNotPermittedOnRowClass => {
                "stability_verdict_not_permitted_on_row_class"
            }
            Self::QuarantineMuteStateNotApplicable => "quarantine_mute_state_not_applicable",
            Self::QuarantineMuteStateNotPermittedOnRowClass => {
                "quarantine_mute_state_not_permitted_on_row_class"
            }
            Self::TestSourceNotApplicable => "test_source_not_applicable",
            Self::TestSourceNotPermittedOnRowClass => "test_source_not_permitted_on_row_class",
            Self::CoverageImpactNotApplicable => "coverage_impact_not_applicable",
            Self::CoverageImpactNotPermittedOnRowClass => {
                "coverage_impact_not_permitted_on_row_class"
            }
            Self::CandidateLineageNotApplicable => "candidate_lineage_not_applicable",
            Self::CandidateLineageNotPermittedOnRowClass => {
                "candidate_lineage_not_permitted_on_row_class"
            }
            Self::ConsumerSurfaceNotApplicable => "consumer_surface_not_applicable",
            Self::ConsumerSurfaceNotPermittedOnRowClass => {
                "consumer_surface_not_permitted_on_row_class"
            }
            Self::ConsumerSurfaceMissingStabilityVerdictAttestation => {
                "consumer_surface_missing_stability_verdict_attestation"
            }
            Self::ConsumerSurfaceMissingQuarantineMuteAttestation => {
                "consumer_surface_missing_quarantine_mute_attestation"
            }
            Self::ConsumerSurfaceMissingTestSourceAttestation => {
                "consumer_surface_missing_test_source_attestation"
            }
            Self::ConsumerSurfaceMissingCoverageImpactAttestation => {
                "consumer_surface_missing_coverage_impact_attestation"
            }
            Self::ConsumerSurfaceMissingCandidateLineageAttestation => {
                "consumer_surface_missing_candidate_lineage_attestation"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::CandidateSourceNotLineageBound => "candidate_source_not_lineage_bound",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WedgeVocabularyCollapsed => "wedge_vocabulary_collapsed",
            Self::StabilityVerdictVocabularyCollapsed => "stability_verdict_vocabulary_collapsed",
            Self::QuarantineMuteStateVocabularyCollapsed => {
                "quarantine_mute_state_vocabulary_collapsed"
            }
            Self::TestSourceVocabularyCollapsed => "test_source_vocabulary_collapsed",
            Self::CoverageImpactVocabularyCollapsed => "coverage_impact_vocabulary_collapsed",
            Self::CandidateLineageVocabularyCollapsed => "candidate_lineage_vocabulary_collapsed",
            Self::ConsumerSurfaceVocabularyCollapsed => "consumer_surface_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Coverage surface (map / gutter / delta panel).
    CoverageSurface,
    /// Flaky-triage surface (verdict + quarantine/mute filter chips).
    FlakyTriageSurface,
    /// Snapshot / golden surface (review diff, accept/reject/promote).
    SnapshotGoldenSurface,
    /// Baseline surface (baseline mutation review, AI candidacy
    /// chrome).
    BaselineSurface,
    /// Release / support packet surface.
    ReleasePacketSurface,
    /// AI tool surface (AI proposals citing durable lineage).
    AiToolSurface,
    /// CLI / headless inspection surface (`aureline coverage ...`,
    /// `aureline test ...`).
    CliHeadless,
    /// Evidence export bundle surface.
    EvidenceExport,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Help/About proof card surface.
    HelpAbout,
    /// Conformance dashboard surface.
    ConformanceDashboard,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 12] = [
        Self::CoverageSurface,
        Self::FlakyTriageSurface,
        Self::SnapshotGoldenSurface,
        Self::BaselineSurface,
        Self::ReleasePacketSurface,
        Self::AiToolSurface,
        Self::CliHeadless,
        Self::EvidenceExport,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageSurface => "coverage_surface",
            Self::FlakyTriageSurface => "flaky_triage_surface",
            Self::SnapshotGoldenSurface => "snapshot_golden_surface",
            Self::BaselineSurface => "baseline_surface",
            Self::ReleasePacketSurface => "release_packet_surface",
            Self::AiToolSurface => "ai_tool_surface",
            Self::CliHeadless => "cli_headless",
            Self::EvidenceExport => "evidence_export",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One coverage-quality truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageQualityRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Coverage-quality lane this row certifies.
    pub lane_class: CoverageQualityLaneClass,
    /// Row class.
    pub row_class: CoverageQualityRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Stability verdict bound by the row (or `not_applicable`).
    pub stability_verdict_class: StabilityVerdictClass,
    /// Quarantine-mute state bound by the row (or `not_applicable`).
    pub quarantine_mute_state_class: QuarantineMuteStateClass,
    /// Test source bound by the row (or `not_applicable`).
    pub test_source_class: TestSourceClass,
    /// Coverage impact bound by the row (or `not_applicable`).
    pub coverage_impact_class: CoverageImpactClass,
    /// Candidate lineage bound by the row (or `not_applicable`).
    pub candidate_lineage_class: CandidateLineageClass,
    /// Consumer surface bound by the row (or `not_applicable`).
    pub consumer_surface_class: ConsumerSurfaceBindingClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: CoverageQualityConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit, or
    /// binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token (or equivalent lineage object reference). Required when
    /// `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For test_source_admission rows whose source class is
    /// `candidate_ai_test`, `automated_baseline`, or
    /// `imported_ci_evidence`, true when the row attests it is bound
    /// to the same session/attempt + review-checkpoint lineage that
    /// the candidate-lineage admissions cover.
    #[serde(default)]
    pub attests_candidate_lineage_bound: bool,
    /// For consumer_surface_binding rows, true when the surface
    /// preserves the stability-verdict vocabulary verbatim.
    #[serde(default)]
    pub attests_stability_verdict_preserved: bool,
    /// For consumer_surface_binding rows, true when the surface
    /// preserves the quarantine-mute-state vocabulary verbatim.
    #[serde(default)]
    pub attests_quarantine_mute_state_preserved: bool,
    /// For consumer_surface_binding rows, true when the surface
    /// preserves the test-source vocabulary verbatim.
    #[serde(default)]
    pub attests_test_source_preserved: bool,
    /// For consumer_surface_binding rows, true when the surface
    /// preserves the coverage-impact vocabulary verbatim.
    #[serde(default)]
    pub attests_coverage_impact_preserved: bool,
    /// For consumer_surface_binding rows, true when the surface
    /// preserves the candidate-lineage vocabulary verbatim.
    #[serde(default)]
    pub attests_candidate_lineage_preserved: bool,
    /// True when raw test bodies, raw coverage payloads, raw snapshot
    /// byte streams, raw baseline diffs, raw runner scrollback bodies,
    /// raw command lines, or raw process environment bytes are
    /// excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this
    /// row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl CoverageQualityRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageQualityConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Coverage-quality packet id consumed by the projection.
    pub coverage_quality_truth_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the lane vocabulary is preserved verbatim.
    pub preserves_lane_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the wedge vocabulary is preserved verbatim.
    pub preserves_wedge_vocabulary: bool,
    /// True when the stability-verdict vocabulary is preserved verbatim.
    pub preserves_stability_verdict_vocabulary: bool,
    /// True when the quarantine-mute-state vocabulary is preserved verbatim.
    pub preserves_quarantine_mute_state_vocabulary: bool,
    /// True when the test-source vocabulary is preserved verbatim.
    pub preserves_test_source_vocabulary: bool,
    /// True when the coverage-impact vocabulary is preserved verbatim.
    pub preserves_coverage_impact_vocabulary: bool,
    /// True when the candidate-lineage vocabulary is preserved verbatim.
    pub preserves_candidate_lineage_vocabulary: bool,
    /// True when the consumer-surface vocabulary is preserved verbatim.
    pub preserves_consumer_surface_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl CoverageQualityConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.coverage_quality_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_stability_verdict_vocabulary
            && self.preserves_quarantine_mute_state_vocabulary
            && self.preserves_test_source_vocabulary
            && self.preserves_coverage_impact_vocabulary
            && self.preserves_candidate_lineage_vocabulary
            && self.preserves_consumer_surface_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`CoverageQualityTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageQualityTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Coverage-quality lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<CoverageQualityLaneClass>,
    /// Coverage-quality rows.
    #[serde(default)]
    pub rows: Vec<CoverageQualityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CoverageQualityConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying coverage, flaky-test,
/// snapshot/golden, and baseline-truth surfaces at the M4 launch-stable
/// grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageQualityTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Coverage-quality lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<CoverageQualityLaneClass>,
    /// Coverage-quality rows.
    #[serde(default)]
    pub rows: Vec<CoverageQualityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CoverageQualityConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl CoverageQualityTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: CoverageQualityTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: COVERAGE_QUALITY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: COVERAGE_QUALITY_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable coverage-quality invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane_class);
        }
        set.into_iter()
            .map(CoverageQualityLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(CoverageQualityRowClass::as_str)
            .collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(SupportClass::as_str).collect()
    }

    /// Returns the unique wedge tokens observed across rows.
    pub fn wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.wedge_class);
        }
        set.into_iter().map(WedgeClass::as_str).collect()
    }

    /// Returns the unique stability-verdict tokens observed across rows.
    pub fn stability_verdict_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.stability_verdict_class);
        }
        set.into_iter().map(StabilityVerdictClass::as_str).collect()
    }

    /// Returns the unique quarantine-mute-state tokens observed across
    /// rows.
    pub fn quarantine_mute_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.quarantine_mute_state_class);
        }
        set.into_iter()
            .map(QuarantineMuteStateClass::as_str)
            .collect()
    }

    /// Returns the unique test-source tokens observed across rows.
    pub fn test_source_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.test_source_class);
        }
        set.into_iter().map(TestSourceClass::as_str).collect()
    }

    /// Returns the unique coverage-impact tokens observed across rows.
    pub fn coverage_impact_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.coverage_impact_class);
        }
        set.into_iter().map(CoverageImpactClass::as_str).collect()
    }

    /// Returns the unique candidate-lineage tokens observed across rows.
    pub fn candidate_lineage_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.candidate_lineage_class);
        }
        set.into_iter().map(CandidateLineageClass::as_str).collect()
    }

    /// Returns the unique consumer-surface tokens observed across rows.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.consumer_surface_class);
        }
        set.into_iter()
            .map(ConsumerSurfaceBindingClass::as_str)
            .collect()
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter().map(EvidenceClass::as_str).collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter().map(KnownLimitClass::as_str).collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> CoverageQualityTruthSupportExport {
        CoverageQualityTruthSupportExport {
            record_kind: COVERAGE_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: COVERAGE_QUALITY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            coverage_quality_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            coverage_quality_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != COVERAGE_QUALITY_TRUTH_PACKET_RECORD_KIND {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "coverage-quality truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != COVERAGE_QUALITY_TRUTH_SCHEMA_VERSION {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "coverage-quality truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered coverage-quality lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers coverage-quality lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            self.collect_row_findings(row, &mut findings);
        }

        for lane in &self.covered_lanes {
            self.collect_lane_coverage_findings(*lane, &mut findings);
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            self.collect_projection_findings(projection, &mut findings);
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }

    fn collect_row_findings(
        &self,
        row: &CoverageQualityRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                format!("row {} identity or timestamp is empty", row.row_id),
            ));
        }
        if !row.raw_source_material_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::RawSourceMaterialPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits raw test bodies, raw coverage payloads, raw snapshot bytes, raw baseline diffs, raw scrollback bodies, raw command lines, or raw env bytes past the boundary",
                    row.row_id
                ),
            ));
        }
        if !row.secrets_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::SecretsPresent,
                FindingSeverity::Blocker,
                format!("row {} admits secrets past the boundary", row.row_id),
            ));
        }
        if !row.ambient_authority_excluded {
            findings.push(ValidationFinding::new(
                FindingKind::AmbientAuthorityPresent,
                FindingSeverity::Blocker,
                format!(
                    "row {} admits ambient authority/credentials past the boundary",
                    row.row_id
                ),
            ));
        }

        if !row.support_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSupportClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound support class", row.row_id),
            ));
        }
        if !row.known_limit_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingKnownLimit,
                FindingSeverity::Blocker,
                format!("row {} has no bound known-limit class", row.row_id),
            ));
        }
        if !row.downgrade_automation_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDowngradeAutomation,
                FindingSeverity::Blocker,
                format!("row {} has no bound downgrade-automation class", row.row_id),
            ));
        }
        if !row.evidence_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound evidence class", row.row_id),
            ));
        }

        if matches!(row.support_class, SupportClass::LaunchStable) && !row.all_bindings_satisfied()
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchStableWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims launch_stable while a binding (support, known limit, downgrade automation, or evidence) is unbound",
                    row.row_id
                ),
            ));
        }

        if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
            findings.push(ValidationFinding::new(
                FindingKind::NarrowedRowMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} has support class {} without a disclosure ref",
                    row.row_id,
                    row.support_class.as_str()
                ),
            ));
        }
        if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
            findings.push(ValidationFinding::new(
                FindingKind::KnownLimitMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} discloses known limit {} without a disclosure ref",
                    row.row_id,
                    row.known_limit_class.as_str()
                ),
            ));
        }
        if row
            .downgrade_automation_class
            .requires_explicit_disclosure()
            && row.disclosure_ref.is_none()
        {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeAutomationMissingDisclosureRef,
                FindingSeverity::Blocker,
                format!(
                    "row {} binds downgrade automation {} without a disclosure ref",
                    row.row_id,
                    row.downgrade_automation_class.as_str()
                ),
            ));
        }

        if row.evidence_refs.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceRefs,
                FindingSeverity::Blocker,
                format!("row {} carries no evidence refs", row.row_id),
            ));
        }

        if row.row_class.requires_wedge() && matches!(row.wedge_class, WedgeClass::NotApplicable) {
            findings.push(ValidationFinding::new(
                FindingKind::WedgeNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a wedge_admission but has no bound wedge",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_wedge() && !matches!(row.wedge_class, WedgeClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::WedgeNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds wedge {}; only wedge_admission rows may bind a wedge",
                    row.row_id,
                    row.row_class.as_str(),
                    row.wedge_class.as_str()
                ),
            ));
        }

        if row.row_class.requires_stability_verdict()
            && matches!(
                row.stability_verdict_class,
                StabilityVerdictClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::StabilityVerdictNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a stability_verdict_admission but has no bound verdict",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_stability_verdict()
            && !matches!(
                row.stability_verdict_class,
                StabilityVerdictClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::StabilityVerdictNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds stability verdict {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.stability_verdict_class.as_str()
                ),
            ));
        }

        if row.row_class.requires_quarantine_mute_state()
            && matches!(
                row.quarantine_mute_state_class,
                QuarantineMuteStateClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::QuarantineMuteStateNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a quarantine_mute_state_admission but has no bound state",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_quarantine_mute_state()
            && !matches!(
                row.quarantine_mute_state_class,
                QuarantineMuteStateClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::QuarantineMuteStateNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds quarantine-mute state {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.quarantine_mute_state_class.as_str()
                ),
            ));
        }

        if row.row_class.requires_test_source()
            && matches!(row.test_source_class, TestSourceClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::TestSourceNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a test_source_admission but has no bound source",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_test_source()
            && !matches!(row.test_source_class, TestSourceClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::TestSourceNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds test source {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.test_source_class.as_str()
                ),
            ));
        }
        if matches!(row.row_class, CoverageQualityRowClass::TestSourceAdmission)
            && row.test_source_class.requires_candidate_lineage()
            && !row.attests_candidate_lineage_bound
        {
            findings.push(ValidationFinding::new(
                FindingKind::CandidateSourceNotLineageBound,
                FindingSeverity::Blocker,
                format!(
                    "row {} declares test source {} but does not attest session/attempt + review-checkpoint lineage",
                    row.row_id,
                    row.test_source_class.as_str()
                ),
            ));
        }

        if row.row_class.requires_coverage_impact()
            && matches!(
                row.coverage_impact_class,
                CoverageImpactClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::CoverageImpactNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a coverage_impact_admission but has no bound impact",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_coverage_impact()
            && !matches!(
                row.coverage_impact_class,
                CoverageImpactClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::CoverageImpactNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds coverage impact {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.coverage_impact_class.as_str()
                ),
            ));
        }

        if row.row_class.requires_candidate_lineage()
            && matches!(
                row.candidate_lineage_class,
                CandidateLineageClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::CandidateLineageNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a candidate_lineage_admission but has no bound lineage class",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_candidate_lineage()
            && !matches!(
                row.candidate_lineage_class,
                CandidateLineageClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::CandidateLineageNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds candidate lineage {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.candidate_lineage_class.as_str()
                ),
            ));
        }

        if row.row_class.requires_consumer_surface()
            && matches!(
                row.consumer_surface_class,
                ConsumerSurfaceBindingClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerSurfaceNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a consumer_surface_binding but has no bound surface",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_consumer_surface()
            && !matches!(
                row.consumer_surface_class,
                ConsumerSurfaceBindingClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerSurfaceNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds consumer surface {}",
                    row.row_id,
                    row.row_class.as_str(),
                    row.consumer_surface_class.as_str()
                ),
            ));
        }

        if matches!(
            row.row_class,
            CoverageQualityRowClass::ConsumerSurfaceBinding
        ) {
            if row
                .consumer_surface_class
                .requires_stability_verdict_attestation()
                && !row.attests_stability_verdict_preserved
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceMissingStabilityVerdictAttestation,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds consumer surface {} but does not attest stability-verdict preservation",
                        row.row_id,
                        row.consumer_surface_class.as_str()
                    ),
                ));
            }
            if row
                .consumer_surface_class
                .requires_quarantine_mute_state_attestation()
                && !row.attests_quarantine_mute_state_preserved
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceMissingQuarantineMuteAttestation,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds consumer surface {} but does not attest quarantine-mute-state preservation",
                        row.row_id,
                        row.consumer_surface_class.as_str()
                    ),
                ));
            }
            if row
                .consumer_surface_class
                .requires_test_source_attestation()
                && !row.attests_test_source_preserved
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceMissingTestSourceAttestation,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds consumer surface {} but does not attest test-source preservation",
                        row.row_id,
                        row.consumer_surface_class.as_str()
                    ),
                ));
            }
            if row
                .consumer_surface_class
                .requires_coverage_impact_attestation()
                && !row.attests_coverage_impact_preserved
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceMissingCoverageImpactAttestation,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds consumer surface {} but does not attest coverage-impact preservation",
                        row.row_id,
                        row.consumer_surface_class.as_str()
                    ),
                ));
            }
            if row
                .consumer_surface_class
                .requires_candidate_lineage_attestation()
                && !row.attests_candidate_lineage_preserved
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceMissingCandidateLineageAttestation,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds consumer surface {} but does not attest candidate-lineage preservation",
                        row.row_id,
                        row.consumer_surface_class.as_str()
                    ),
                ));
            }
        }

        if matches!(row.row_class, CoverageQualityRowClass::LineageAdmission)
            && row
                .execution_context_id_binding
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                .unwrap_or(true)
        {
            findings.push(ValidationFinding::new(
                FindingKind::LineageAdmissionMissingExecutionContextId,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a lineage_admission but has no bound execution_context_id",
                    row.row_id
                ),
            ));
        }

        if matches!(
            row.confidence_class,
            CoverageQualityConfidenceClass::LowConfidence
        ) && matches!(row.support_class, SupportClass::LaunchStable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchStableWithUnboundBinding,
                FindingSeverity::Warning,
                format!(
                    "row {} claims launch_stable at low_confidence; narrowing until evidence grows",
                    row.row_id
                ),
            ));
        }
    }

    fn collect_lane_coverage_findings(
        &self,
        lane: CoverageQualityLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_launch = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    CoverageQualityRowClass::CoverageFlakySnapshotBaselineQuality
                )
                && matches!(row.support_class, SupportClass::LaunchStable)
        });
        if !lane_claims_launch {
            return;
        }

        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(row.row_class, CoverageQualityRowClass::WedgeAdmission)
                    && row.wedge_class == wedge
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingWedgeAdmissionCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no wedge_admission row for {}",
                        lane.as_str(),
                        wedge.as_str()
                    ),
                ));
            }
        }

        for verdict in StabilityVerdictClass::REQUIRED_FOR_LAUNCH_STABLE {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        CoverageQualityRowClass::StabilityVerdictAdmission
                    )
                    && row.stability_verdict_class == verdict
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingStabilityVerdictCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no stability_verdict_admission row for {}",
                        lane.as_str(),
                        verdict.as_str()
                    ),
                ));
            }
        }

        for state in QuarantineMuteStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        CoverageQualityRowClass::QuarantineMuteStateAdmission
                    )
                    && row.quarantine_mute_state_class == state
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingQuarantineMuteStateCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no quarantine_mute_state_admission row for {}",
                        lane.as_str(),
                        state.as_str()
                    ),
                ));
            }
        }

        for source in TestSourceClass::REQUIRED_FOR_LAUNCH_STABLE {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(row.row_class, CoverageQualityRowClass::TestSourceAdmission)
                    && row.test_source_class == source
                    && (!source.requires_candidate_lineage() || row.attests_candidate_lineage_bound)
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingTestSourceCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no fully-attested test_source_admission row for {}",
                        lane.as_str(),
                        source.as_str()
                    ),
                ));
            }
        }

        for impact in CoverageImpactClass::REQUIRED_FOR_LAUNCH_STABLE {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        CoverageQualityRowClass::CoverageImpactAdmission
                    )
                    && row.coverage_impact_class == impact
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingCoverageImpactCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no coverage_impact_admission row for {}",
                        lane.as_str(),
                        impact.as_str()
                    ),
                ));
            }
        }

        for lineage in CandidateLineageClass::REQUIRED_FOR_LAUNCH_STABLE {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        CoverageQualityRowClass::CandidateLineageAdmission
                    )
                    && row.candidate_lineage_class == lineage
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingCandidateLineageCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no candidate_lineage_admission row for {}",
                        lane.as_str(),
                        lineage.as_str()
                    ),
                ));
            }
        }

        for surface in ConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        CoverageQualityRowClass::ConsumerSurfaceBinding
                    )
                    && row.consumer_surface_class == surface
                    && (!surface.requires_stability_verdict_attestation()
                        || row.attests_stability_verdict_preserved)
                    && (!surface.requires_quarantine_mute_state_attestation()
                        || row.attests_quarantine_mute_state_preserved)
                    && (!surface.requires_test_source_attestation()
                        || row.attests_test_source_preserved)
                    && (!surface.requires_coverage_impact_attestation()
                        || row.attests_coverage_impact_preserved)
                    && (!surface.requires_candidate_lineage_attestation()
                        || row.attests_candidate_lineage_preserved)
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerSurfaceCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no fully-attested consumer_surface_binding row for {}",
                        lane.as_str(),
                        surface.as_str()
                    ),
                ));
            }
        }

        let has_lineage = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(row.row_class, CoverageQualityRowClass::LineageAdmission)
                && row
                    .execution_context_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false)
        });
        if !has_lineage {
            findings.push(ValidationFinding::new(
                FindingKind::MissingLineageAdmission,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no lineage_admission row binding execution_context_id",
                    lane.as_str()
                ),
            ));
        }
    }

    fn collect_projection_findings(
        &self,
        projection: &CoverageQualityConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve coverage-quality truth",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_lane_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::LaneVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the lane vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_row_class_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::RowClassVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the row-class vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_support_class_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::SupportClassVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the support-class vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_wedge_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::WedgeVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the wedge vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_stability_verdict_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::StabilityVerdictVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the stability-verdict vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_quarantine_mute_state_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::QuarantineMuteStateVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the quarantine-mute-state vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_test_source_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::TestSourceVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the test-source vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_coverage_impact_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::CoverageImpactVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the coverage-impact vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_candidate_lineage_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::CandidateLineageVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the candidate-lineage vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_consumer_surface_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerSurfaceVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the consumer-surface vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_known_limit_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::KnownLimitVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the known-limit vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_downgrade_automation_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeAutomationVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the downgrade-automation vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_evidence_class_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::EvidenceClassVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the evidence-class vocabulary",
                    projection.projection_ref
                ),
            ));
        }
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageQualityTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub coverage_quality_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub coverage_quality_truth_packet: CoverageQualityTruthPacket,
}

impl CoverageQualityTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == COVERAGE_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == COVERAGE_QUALITY_TRUTH_SCHEMA_VERSION
            && self.coverage_quality_truth_packet_id_ref
                == self.coverage_quality_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.coverage_quality_truth_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum CoverageQualityTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for CoverageQualityTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "coverage-quality truth packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "coverage-quality truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CoverageQualityTruthArtifactError {}

/// Returns the checked-in stable coverage-quality truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or
/// validate.
pub fn current_stable_coverage_quality_truth_packet(
) -> Result<CoverageQualityTruthPacket, CoverageQualityTruthArtifactError> {
    let packet: CoverageQualityTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.json"
    )))
    .map_err(CoverageQualityTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(CoverageQualityTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        COVERAGE_QUALITY_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        COVERAGE_QUALITY_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: CoverageQualityLaneClass) -> CoverageQualityRow {
        CoverageQualityRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: CoverageQualityRowClass::CoverageFlakySnapshotBaselineQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            stability_verdict_class: StabilityVerdictClass::NotApplicable,
            quarantine_mute_state_class: QuarantineMuteStateClass::NotApplicable,
            test_source_class: TestSourceClass::NotApplicable,
            coverage_impact_class: CoverageImpactClass::NotApplicable,
            candidate_lineage_class: CandidateLineageClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: CoverageQualityConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            attests_candidate_lineage_bound: false,
            attests_stability_verdict_preserved: false,
            attests_quarantine_mute_state_preserved: false,
            attests_test_source_preserved: false,
            attests_coverage_impact_preserved: false,
            attests_candidate_lineage_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn admission_row(
        prefix: &str,
        suffix: &str,
        lane: CoverageQualityLaneClass,
        row_class: CoverageQualityRowClass,
        downgrade: DowngradeAutomationClass,
    ) -> CoverageQualityRow {
        CoverageQualityRow {
            row_id: format!("row:{prefix}:{}:{}", row_class.as_str(), suffix),
            lane_class: lane,
            row_class,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            stability_verdict_class: StabilityVerdictClass::NotApplicable,
            quarantine_mute_state_class: QuarantineMuteStateClass::NotApplicable,
            test_source_class: TestSourceClass::NotApplicable,
            coverage_impact_class: CoverageImpactClass::NotApplicable,
            candidate_lineage_class: CandidateLineageClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: downgrade,
            confidence_class: CoverageQualityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#{}", doc_ref(), downgrade.as_str())),
            execution_context_id_binding: None,
            attests_candidate_lineage_bound: false,
            attests_stability_verdict_preserved: false,
            attests_quarantine_mute_state_preserved: false,
            attests_test_source_preserved: false,
            attests_coverage_impact_preserved: false,
            attests_candidate_lineage_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn lane_rows(lane: CoverageQualityLaneClass, prefix: &str) -> Vec<CoverageQualityRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            let mut row = admission_row(
                prefix,
                wedge.as_str(),
                lane,
                CoverageQualityRowClass::WedgeAdmission,
                DowngradeAutomationClass::AutoNarrowOnWedgeAdmissionGap,
            );
            row.wedge_class = wedge;
            row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
            out.push(row);
        }
        for verdict in StabilityVerdictClass::REQUIRED_FOR_LAUNCH_STABLE {
            let mut row = admission_row(
                prefix,
                verdict.as_str(),
                lane,
                CoverageQualityRowClass::StabilityVerdictAdmission,
                DowngradeAutomationClass::AutoNarrowOnStabilityVerdictGap,
            );
            row.stability_verdict_class = verdict;
            out.push(row);
        }
        for state in QuarantineMuteStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            let mut row = admission_row(
                prefix,
                state.as_str(),
                lane,
                CoverageQualityRowClass::QuarantineMuteStateAdmission,
                DowngradeAutomationClass::AutoNarrowOnQuarantineMuteStateGap,
            );
            row.quarantine_mute_state_class = state;
            out.push(row);
        }
        for source in TestSourceClass::REQUIRED_FOR_LAUNCH_STABLE {
            let mut row = admission_row(
                prefix,
                source.as_str(),
                lane,
                CoverageQualityRowClass::TestSourceAdmission,
                DowngradeAutomationClass::AutoNarrowOnTestSourceGap,
            );
            row.test_source_class = source;
            row.attests_candidate_lineage_bound = source.requires_candidate_lineage();
            out.push(row);
        }
        for impact in CoverageImpactClass::REQUIRED_FOR_LAUNCH_STABLE {
            let mut row = admission_row(
                prefix,
                impact.as_str(),
                lane,
                CoverageQualityRowClass::CoverageImpactAdmission,
                DowngradeAutomationClass::AutoNarrowOnCoverageImpactGap,
            );
            row.coverage_impact_class = impact;
            out.push(row);
        }
        for lineage in CandidateLineageClass::REQUIRED_FOR_LAUNCH_STABLE {
            let mut row = admission_row(
                prefix,
                lineage.as_str(),
                lane,
                CoverageQualityRowClass::CandidateLineageAdmission,
                DowngradeAutomationClass::AutoNarrowOnCandidateLineageGap,
            );
            row.candidate_lineage_class = lineage;
            out.push(row);
        }
        for surface in ConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            let mut row = admission_row(
                prefix,
                surface.as_str(),
                lane,
                CoverageQualityRowClass::ConsumerSurfaceBinding,
                DowngradeAutomationClass::AutoNarrowOnConsumerSurfaceGap,
            );
            row.consumer_surface_class = surface;
            row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
            row.attests_stability_verdict_preserved =
                surface.requires_stability_verdict_attestation();
            row.attests_quarantine_mute_state_preserved =
                surface.requires_quarantine_mute_state_attestation();
            row.attests_test_source_preserved = surface.requires_test_source_attestation();
            row.attests_coverage_impact_preserved = surface.requires_coverage_impact_attestation();
            row.attests_candidate_lineage_preserved =
                surface.requires_candidate_lineage_attestation();
            out.push(row);
        }
        let mut lineage = admission_row(
            prefix,
            "lineage",
            lane,
            CoverageQualityRowClass::LineageAdmission,
            DowngradeAutomationClass::AutoNarrowOnLineageBreak,
        );
        lineage.execution_context_id_binding =
            Some(format!("exec:m4:{prefix}:coverage_quality_lineage"));
        out.push(lineage);
        out
    }

    fn projection(surface: ConsumerSurface) -> CoverageQualityConsumerProjection {
        CoverageQualityConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            coverage_quality_truth_packet_id_ref:
                "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_stability_verdict_vocabulary: true,
            preserves_quarantine_mute_state_vocabulary: true,
            preserves_test_source_vocabulary: true,
            preserves_coverage_impact_vocabulary: true,
            preserves_candidate_lineage_vocabulary: true,
            preserves_consumer_surface_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn sample_input() -> CoverageQualityTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(
            CoverageQualityLaneClass::CoverageLane,
            "coverage",
        ));
        rows.extend(lane_rows(CoverageQualityLaneClass::FlakyTestLane, "flaky"));
        rows.extend(lane_rows(
            CoverageQualityLaneClass::SnapshotGoldenLane,
            "snapshot",
        ));
        rows.extend(lane_rows(
            CoverageQualityLaneClass::BaselineTruthLane,
            "baseline",
        ));
        CoverageQualityTruthPacketInput {
            packet_id: "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline"
                    .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: CoverageQualityLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            CoverageQualityLaneClass::CoverageLane.as_str(),
            "coverage_lane"
        );
        assert_eq!(
            CoverageQualityLaneClass::BaselineTruthLane.as_str(),
            "baseline_truth_lane"
        );
        assert_eq!(
            CoverageQualityRowClass::CoverageFlakySnapshotBaselineQuality.as_str(),
            "coverage_flaky_snapshot_baseline_quality"
        );
        assert_eq!(
            CoverageQualityRowClass::TestSourceAdmission.as_str(),
            "test_source_admission"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(
            WedgeClass::StabilityVerdictSeparation.as_str(),
            "stability_verdict_separation"
        );
        assert_eq!(StabilityVerdictClass::Quarantined.as_str(), "quarantined");
        assert_eq!(StabilityVerdictClass::Muted.as_str(), "muted");
        assert_eq!(
            QuarantineMuteStateClass::ExpiredPendingRenewal.as_str(),
            "expired_pending_renewal"
        );
        assert_eq!(
            TestSourceClass::CandidateAiTest.as_str(),
            "candidate_ai_test"
        );
        assert_eq!(
            TestSourceClass::AutomatedBaseline.as_str(),
            "automated_baseline"
        );
        assert_eq!(
            TestSourceClass::ImportedCiEvidence.as_str(),
            "imported_ci_evidence"
        );
        assert_eq!(CoverageImpactClass::Measured.as_str(), "measured");
        assert_eq!(
            CoverageImpactClass::NotComparable.as_str(),
            "not_comparable"
        );
        assert_eq!(
            CandidateLineageClass::SessionAttemptBound.as_str(),
            "session_attempt_bound"
        );
        assert_eq!(
            CandidateLineageClass::ReviewCheckpointBound.as_str(),
            "review_checkpoint_bound"
        );
        assert_eq!(
            ConsumerSurfaceBindingClass::ReleasePacketSurface.as_str(),
            "release_packet_surface"
        );
        assert_eq!(EvidenceClass::EvidenceUnbound.as_str(), "evidence_unbound");
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            DowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
        );
        assert_eq!(
            ConsumerSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(ConsumerSurface::AiToolSurface.as_str(), "ai_tool_surface");
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::CandidateSourceNotLineageBound.as_str(),
            "candidate_source_not_lineage_bound"
        );
        assert_eq!(
            FindingKind::ConsumerSurfaceMissingCandidateLineageAttestation.as_str(),
            "consumer_surface_missing_candidate_lineage_attestation"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = CoverageQualityTruthPacket::materialize(sample_input());
        assert_eq!(
            packet.promotion_state,
            PromotionState::Stable,
            "expected stable but got findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|f| f.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export(
                "support:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn missing_stability_verdict_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                CoverageQualityRowClass::StabilityVerdictAdmission
            ) && row.stability_verdict_class == StabilityVerdictClass::Quarantined
                && row.lane_class == CoverageQualityLaneClass::FlakyTestLane)
        });
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingStabilityVerdictCoverage));
    }

    #[test]
    fn missing_quarantine_mute_state_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                CoverageQualityRowClass::QuarantineMuteStateAdmission
            ) && row.quarantine_mute_state_class
                == QuarantineMuteStateClass::ExpiredPendingRenewal
                && row.lane_class == CoverageQualityLaneClass::FlakyTestLane)
        });
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingQuarantineMuteStateCoverage
        }));
    }

    #[test]
    fn candidate_ai_test_without_lineage_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, CoverageQualityRowClass::TestSourceAdmission)
                && row.lane_class == CoverageQualityLaneClass::CoverageLane
                && row.test_source_class == TestSourceClass::CandidateAiTest
            {
                row.attests_candidate_lineage_bound = false;
                break;
            }
        }
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::CandidateSourceNotLineageBound));
    }

    #[test]
    fn missing_coverage_impact_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                CoverageQualityRowClass::CoverageImpactAdmission
            ) && row.coverage_impact_class == CoverageImpactClass::NotComparable
                && row.lane_class == CoverageQualityLaneClass::CoverageLane)
        });
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingCoverageImpactCoverage));
    }

    #[test]
    fn consumer_surface_missing_candidate_lineage_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                CoverageQualityRowClass::ConsumerSurfaceBinding
            ) && row.lane_class == CoverageQualityLaneClass::BaselineTruthLane
                && row.consumer_surface_class == ConsumerSurfaceBindingClass::BaselineSurface
            {
                row.attests_candidate_lineage_preserved = false;
                break;
            }
        }
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::ConsumerSurfaceMissingCandidateLineageAttestation
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, CoverageQualityRowClass::LineageAdmission)
                && row.lane_class == CoverageQualityLaneClass::CoverageLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != ConsumerSurface::ConformanceDashboard
        });
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_coverage_impact_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_coverage_impact_vocabulary = false;
            }
        }
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::CoverageImpactVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = CoverageQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
