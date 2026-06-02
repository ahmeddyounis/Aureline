//! Adapter stability truth packet for the M4 stable lane.
//!
//! This module is the language-owned contract that pins how the
//! formatter, linter, and test-adapter lanes stay one boundary truth
//! across degraded-provider conditions on the launch wedges. The packet
//! ships formatter / linter / test-adapter stability with degraded-
//! provider truth so the editor language pack, framework pack panel,
//! language settings/help, CLI/headless inspector, support export,
//! release proof index, Help/About proof card, and the conformance
//! dashboard all read one record. Surfaces MUST NOT mint local copies
//! or paraphrase adapter posture; they read this packet verbatim.
//!
//! Every row binds a closed `adapter_lane_class`,
//! `adapter_stability_row_class`, `support_class`,
//! `adapter_capability_class`, `degraded_provider_class`,
//! `adapter_outcome_class`, `launch_wedge_class`, `evidence_class`,
//! `known_limit_class`, `downgrade_automation_class`, and
//! `adapter_stability_confidence_class` plus an `evidence_refs` array
//! and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or binds
//! a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! source bodies, raw formatter or linter output, raw test logs,
//! secrets, ambient credentials, or any other private material past
//! the boundary. A row that claims `launch_stable` while leaving its
//! known limit, downgrade automation, evidence class, adapter
//! capability, degraded-provider state, or adapter outcome unbound is
//! refused; the validator narrows below launch-stable instead of
//! inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`AdapterStabilityTruthPacket`].
pub const ADAPTER_STABILITY_TRUTH_PACKET_RECORD_KIND: &str =
    "adapter_stability_truth_stable_packet";

/// Stable record-kind tag for [`AdapterStabilityTruthSupportExport`].
pub const ADAPTER_STABILITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "adapter_stability_truth_support_export";

/// Integer schema version for the adapter stability truth packet.
pub const ADAPTER_STABILITY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ADAPTER_STABILITY_TRUTH_SCHEMA_REF: &str =
    "schemas/language/adapter_stability_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const ADAPTER_STABILITY_TRUTH_DOC_REF: &str =
    "docs/languages/m4/ship-formatter-linter-and-test-adapter-stability-with.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const ADAPTER_STABILITY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m4/ship-formatter-linter-and-test-adapter-stability-with.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const ADAPTER_STABILITY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m4/adapter_stability_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const ADAPTER_STABILITY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m4/adapter_stability_truth_packet.json";

/// Closed adapter-lane vocabulary. Every required lane MUST have at
/// least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterLaneClass {
    /// Formatter adapter lane.
    FormatterLane,
    /// Linter adapter lane.
    LinterLane,
    /// Test-adapter lane.
    TestAdapterLane,
}

impl AdapterLaneClass {
    /// Every required adapter lane, in declaration order.
    pub const REQUIRED: [Self; 3] = [Self::FormatterLane, Self::LinterLane, Self::TestAdapterLane];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FormatterLane => "formatter_lane",
            Self::LinterLane => "linter_lane",
            Self::TestAdapterLane => "test_adapter_lane",
        }
    }
}

/// Closed adapter-stability row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterStabilityRowClass {
    /// The lane's headline qualification row.
    AdapterStabilityQuality,
    /// Adapter-capability truth row binding exactly one capability
    /// (discover, execute, report).
    AdapterCapabilityTruth,
    /// Degraded-provider admission row binding a degraded-provider class.
    DegradedProviderAdmission,
    /// Adapter-outcome admission row binding an outcome class.
    AdapterOutcomeAdmission,
    /// Launch-wedge coverage row certifying one launch wedge touchpoint.
    LaunchWedgeCoverage,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl AdapterStabilityRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterStabilityQuality => "adapter_stability_quality",
            Self::AdapterCapabilityTruth => "adapter_capability_truth",
            Self::DegradedProviderAdmission => "degraded_provider_admission",
            Self::AdapterOutcomeAdmission => "adapter_outcome_admission",
            Self::LaunchWedgeCoverage => "launch_wedge_coverage",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }
}

/// Closed support-class vocabulary applied to an adapter-stability row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-stable grade.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable; narrowing is disclosed.
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

/// Closed adapter-capability vocabulary. A lane that claims
/// `launch_stable` MUST cover every capability required by the adapter
/// stability model: discover, execute, and report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterCapabilityClass {
    /// Discovery — locating the formatter binary, lint rules, or test
    /// targets on the launch wedge.
    Discover,
    /// Execution — running the formatter, linter, or test adapter.
    Execute,
    /// Reporting — surfacing diff, diagnostics, or test results to
    /// consumer surfaces.
    Report,
    /// Row is not an adapter-capability row.
    NotApplicable,
}

impl AdapterCapabilityClass {
    /// Every required adapter capability, in declaration order.
    pub const REQUIRED_FOR_LAUNCH: [Self; 3] = [Self::Discover, Self::Execute, Self::Report];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Discover => "discover",
            Self::Execute => "execute",
            Self::Report => "report",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed degraded-provider vocabulary. A `degraded_provider_admission`
/// row binds exactly one provider state class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedProviderClass {
    /// Provider is fully healthy.
    ProviderHealthy,
    /// Provider is degraded but the adapter surfaces a visible warning.
    ProviderDegradedWarned,
    /// Provider is unavailable — adapter refuses and labels the gap.
    ProviderUnavailable,
    /// Provider is misconfigured — adapter refuses with a configuration cue.
    ProviderMisconfigured,
    /// Provider timed out — adapter refuses and surfaces a timeout cue.
    ProviderTimedOut,
    /// Row is not a degraded-provider admission row.
    NotApplicable,
    /// Row has no bound degraded-provider class; this never qualifies stable
    /// for a row class that requires a binding.
    StateUnbound,
}

impl DegradedProviderClass {
    /// Every degraded-provider state a lane that claims `launch_stable`
    /// MUST cover, in declaration order.
    pub const REQUIRED_FOR_LAUNCH: [Self; 3] = [
        Self::ProviderHealthy,
        Self::ProviderDegradedWarned,
        Self::ProviderUnavailable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderHealthy => "provider_healthy",
            Self::ProviderDegradedWarned => "provider_degraded_warned",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ProviderMisconfigured => "provider_misconfigured",
            Self::ProviderTimedOut => "provider_timed_out",
            Self::NotApplicable => "not_applicable",
            Self::StateUnbound => "state_unbound",
        }
    }

    /// True when this degraded-provider class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::StateUnbound)
    }
}

/// Closed adapter-outcome vocabulary. An `adapter_outcome_admission`
/// row binds exactly one outcome class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterOutcomeClass {
    /// Adapter run passed without warnings.
    Passed,
    /// Adapter run passed with caveats.
    PassedWithWarnings,
    /// Adapter run failed.
    Failed,
    /// Adapter run is blocked by missing prerequisites.
    Blocked,
    /// Adapter run has not been attempted.
    Pending,
    /// Adapter run is unsupported on this row.
    Unsupported,
    /// Row is not an adapter-outcome admission row.
    NotApplicable,
    /// Row has no bound adapter-outcome class; this never qualifies stable
    /// for a row class that requires a binding.
    OutcomeUnbound,
}

impl AdapterOutcomeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::PassedWithWarnings => "passed_with_warnings",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
            Self::Pending => "pending",
            Self::Unsupported => "unsupported",
            Self::NotApplicable => "not_applicable",
            Self::OutcomeUnbound => "outcome_unbound",
        }
    }

    /// True when this adapter-outcome class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::OutcomeUnbound)
    }
}

/// Closed launch-wedge vocabulary. A `launch_wedge_coverage` row binds
/// exactly one launch wedge class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchWedgeClass {
    /// Python launch wedge.
    PythonWedge,
    /// TypeScript / JavaScript launch wedge.
    TypescriptJavascriptWedge,
    /// Rust launch wedge.
    RustWedge,
    /// Go launch wedge.
    GoWedge,
    /// Java / Kotlin launch wedge.
    JavaKotlinWedge,
    /// C / C++ launch wedge.
    CCppWedge,
    /// Row is not a launch-wedge coverage row.
    NotApplicable,
}

impl LaunchWedgeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PythonWedge => "python_wedge",
            Self::TypescriptJavascriptWedge => "typescript_javascript_wedge",
            Self::RustWedge => "rust_wedge",
            Self::GoWedge => "go_wedge",
            Self::JavaKotlinWedge => "java_kotlin_wedge",
            Self::CCppWedge => "c_cpp_wedge",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// The row is backed by certified archetype-repo evidence.
    ArchetypeRepoEvidence,
    /// The row is backed by framework- / formatter- / linter-migration evidence.
    FrameworkMigrationEvidence,
    /// The row is backed by design-partner evidence.
    DesignPartnerEvidence,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a conformance suite run.
    ConformanceSuiteEvidence,
    /// The row is backed by a benchmark / fitness function capture.
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
            Self::ArchetypeRepoEvidence => "archetype_repo_evidence",
            Self::FrameworkMigrationEvidence => "framework_migration_evidence",
            Self::DesignPartnerEvidence => "design_partner_evidence",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
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

/// Closed known-limit vocabulary attached to an adapter-stability row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies an adapter-lane subset.
    AdapterSubsetOnly,
    /// The row only certifies a launch-wedge subset.
    WedgeSubsetOnly,
    /// The row only certifies a provider-state subset.
    ProviderStateSubsetOnly,
    /// The row only certifies an adapter-capability subset.
    CapabilitySubsetOnly,
    /// The row only certifies an outcome subset.
    OutcomeSubsetOnly,
    /// The row only certifies an archetype subset.
    ArchetypeSubsetOnly,
    /// The row certifies an unsupported runtime target gap.
    UnsupportedRuntimeTarget,
    /// The row certifies a beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known-limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::AdapterSubsetOnly => "adapter_subset_only",
            Self::WedgeSubsetOnly => "wedge_subset_only",
            Self::ProviderStateSubsetOnly => "provider_state_subset_only",
            Self::CapabilitySubsetOnly => "capability_subset_only",
            Self::OutcomeSubsetOnly => "outcome_subset_only",
            Self::ArchetypeSubsetOnly => "archetype_subset_only",
            Self::UnsupportedRuntimeTarget => "unsupported_runtime_target",
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

/// Closed downgrade-automation vocabulary attached to an
/// adapter-stability row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a certified fixture is missing or stale.
    AutoNarrowOnMissingFixture,
    /// Automatically narrow when a certified archetype repo is missing.
    AutoNarrowOnMissingArchetype,
    /// Automatically narrow when the provider is degraded or unavailable.
    AutoNarrowOnDegradedProvider,
    /// Automatically narrow when adapter outcome regresses to failed.
    AutoNarrowOnOutcomeFailure,
    /// Automatically narrow when a required adapter capability is missing.
    AutoNarrowOnCapabilityGap,
    /// Automatically demote when confidence drops below the certified bar.
    AutoDemoteOnLowConfidence,
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
            Self::AutoNarrowOnMissingFixture => "auto_narrow_on_missing_fixture",
            Self::AutoNarrowOnMissingArchetype => "auto_narrow_on_missing_archetype",
            Self::AutoNarrowOnDegradedProvider => "auto_narrow_on_degraded_provider",
            Self::AutoNarrowOnOutcomeFailure => "auto_narrow_on_outcome_failure",
            Self::AutoNarrowOnCapabilityGap => "auto_narrow_on_capability_gap",
            Self::AutoDemoteOnLowConfidence => "auto_demote_on_low_confidence",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for an adapter-stability row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterStabilityConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl AdapterStabilityConfidenceClass {
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
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
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

/// Closed validation-finding vocabulary for the adapter-stability packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required adapter lane has no row.
    MissingAdapterLaneCoverage,
    /// A lane claiming launch_stable is missing a required adapter capability.
    MissingAdapterCapabilityCoverage,
    /// A lane claiming launch_stable is missing a required degraded-provider state.
    MissingDegradedProviderCoverage,
    /// A lane claiming launch_stable is missing an adapter-outcome admission row.
    MissingAdapterOutcomeCoverage,
    /// A lane claiming launch_stable is missing a launch-wedge coverage row.
    MissingLaunchWedgeCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row has no bound degraded-provider class on a row-class that requires it.
    MissingDegradedProviderClass,
    /// A row has no bound adapter-outcome class on a row-class that requires it.
    MissingAdapterOutcomeClass,
    /// A row claims launch_stable while one or more bindings is unbound.
    LaunchStableWithUnboundBinding,
    /// A row narrowed below launch-stable drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// An adapter-capability truth row drops its capability binding.
    AdapterCapabilityNotApplicable,
    /// A non-adapter-capability row binds an adapter capability.
    AdapterCapabilityNotPermittedOnRowClass,
    /// A degraded-provider admission row drops its provider-state class.
    DegradedProviderNotApplicable,
    /// A non-degraded-provider row binds a degraded-provider class.
    DegradedProviderNotPermittedOnRowClass,
    /// An adapter-outcome admission row drops its outcome class.
    AdapterOutcomeNotApplicable,
    /// A non-adapter-outcome row binds an adapter-outcome class.
    AdapterOutcomeNotPermittedOnRowClass,
    /// A launch-wedge coverage row drops its launch-wedge binding.
    LaunchWedgeNotApplicable,
    /// A non-launch-wedge-coverage row binds a launch-wedge class.
    LaunchWedgeNotPermittedOnRowClass,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops adapter-stability truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the adapter-capability vocabulary.
    AdapterCapabilityVocabularyCollapsed,
    /// A projection collapses the degraded-provider vocabulary.
    DegradedProviderVocabularyCollapsed,
    /// A projection collapses the adapter-outcome vocabulary.
    AdapterOutcomeVocabularyCollapsed,
    /// A projection collapses the launch-wedge vocabulary.
    LaunchWedgeVocabularyCollapsed,
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
            Self::MissingAdapterLaneCoverage => "missing_adapter_lane_coverage",
            Self::MissingAdapterCapabilityCoverage => "missing_adapter_capability_coverage",
            Self::MissingDegradedProviderCoverage => "missing_degraded_provider_coverage",
            Self::MissingAdapterOutcomeCoverage => "missing_adapter_outcome_coverage",
            Self::MissingLaunchWedgeCoverage => "missing_launch_wedge_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingDegradedProviderClass => "missing_degraded_provider_class",
            Self::MissingAdapterOutcomeClass => "missing_adapter_outcome_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::AdapterCapabilityNotApplicable => "adapter_capability_not_applicable",
            Self::AdapterCapabilityNotPermittedOnRowClass => {
                "adapter_capability_not_permitted_on_row_class"
            }
            Self::DegradedProviderNotApplicable => "degraded_provider_not_applicable",
            Self::DegradedProviderNotPermittedOnRowClass => {
                "degraded_provider_not_permitted_on_row_class"
            }
            Self::AdapterOutcomeNotApplicable => "adapter_outcome_not_applicable",
            Self::AdapterOutcomeNotPermittedOnRowClass => {
                "adapter_outcome_not_permitted_on_row_class"
            }
            Self::LaunchWedgeNotApplicable => "launch_wedge_not_applicable",
            Self::LaunchWedgeNotPermittedOnRowClass => "launch_wedge_not_permitted_on_row_class",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::AdapterCapabilityVocabularyCollapsed => "adapter_capability_vocabulary_collapsed",
            Self::DegradedProviderVocabularyCollapsed => "degraded_provider_vocabulary_collapsed",
            Self::AdapterOutcomeVocabularyCollapsed => "adapter_outcome_vocabulary_collapsed",
            Self::LaunchWedgeVocabularyCollapsed => "launch_wedge_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the adapter-stability packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Editor language-pack surface.
    EditorLanguagePack,
    /// Framework pack panel surface.
    FrameworkPackPanel,
    /// Language settings / help surface.
    LanguageSettings,
    /// CLI or headless inspection surface.
    CliHeadless,
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
    pub const REQUIRED: [Self; 8] = [
        Self::EditorLanguagePack,
        Self::FrameworkPackPanel,
        Self::LanguageSettings,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorLanguagePack => "editor_language_pack",
            Self::FrameworkPackPanel => "framework_pack_panel",
            Self::LanguageSettings => "language_settings",
            Self::CliHeadless => "cli_headless",
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

/// One adapter-stability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterStabilityRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Adapter lane this row certifies.
    pub lane_class: AdapterLaneClass,
    /// Row class.
    pub row_class: AdapterStabilityRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Adapter-capability class (or `not_applicable`).
    pub adapter_capability_class: AdapterCapabilityClass,
    /// Degraded-provider class (or `not_applicable`).
    pub degraded_provider_class: DegradedProviderClass,
    /// Adapter-outcome class (or `not_applicable`).
    pub adapter_outcome_class: AdapterOutcomeClass,
    /// Launch-wedge class (or `not_applicable`).
    pub launch_wedge_class: LaunchWedgeClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: AdapterStabilityConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit,
    /// or binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl AdapterStabilityRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.degraded_provider_class.is_bound()
            && self.adapter_outcome_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterStabilityConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Adapter-stability packet id consumed by the projection.
    pub adapter_stability_packet_id_ref: String,
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
    /// True when the adapter-capability vocabulary is preserved verbatim.
    pub preserves_adapter_capability_vocabulary: bool,
    /// True when the degraded-provider vocabulary is preserved verbatim.
    pub preserves_degraded_provider_vocabulary: bool,
    /// True when the adapter-outcome vocabulary is preserved verbatim.
    pub preserves_adapter_outcome_vocabulary: bool,
    /// True when the launch-wedge vocabulary is preserved verbatim.
    pub preserves_launch_wedge_vocabulary: bool,
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

impl AdapterStabilityConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.adapter_stability_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_adapter_capability_vocabulary
            && self.preserves_degraded_provider_vocabulary
            && self.preserves_adapter_outcome_vocabulary
            && self.preserves_launch_wedge_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`AdapterStabilityTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterStabilityTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Adapter lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<AdapterLaneClass>,
    /// Adapter-stability rows.
    #[serde(default)]
    pub rows: Vec<AdapterStabilityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<AdapterStabilityConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying formatter, linter, and test-adapter
/// stability with degraded-provider truth across the launch wedges at
/// the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterStabilityTruthPacket {
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
    /// Adapter lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<AdapterLaneClass>,
    /// Adapter-stability rows.
    #[serde(default)]
    pub rows: Vec<AdapterStabilityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<AdapterStabilityConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl AdapterStabilityTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: AdapterStabilityTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: ADAPTER_STABILITY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: ADAPTER_STABILITY_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable adapter-stability invariants.
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
        set.into_iter().map(AdapterLaneClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(AdapterStabilityRowClass::as_str)
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

    /// Returns the unique adapter-capability tokens observed across rows.
    pub fn adapter_capability_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.adapter_capability_class);
        }
        set.into_iter()
            .map(AdapterCapabilityClass::as_str)
            .collect()
    }

    /// Returns the unique degraded-provider tokens observed across rows.
    pub fn degraded_provider_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.degraded_provider_class);
        }
        set.into_iter().map(DegradedProviderClass::as_str).collect()
    }

    /// Returns the unique adapter-outcome tokens observed across rows.
    pub fn adapter_outcome_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.adapter_outcome_class);
        }
        set.into_iter().map(AdapterOutcomeClass::as_str).collect()
    }

    /// Returns the unique launch-wedge tokens observed across rows.
    pub fn launch_wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.launch_wedge_class);
        }
        set.into_iter().map(LaunchWedgeClass::as_str).collect()
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

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> AdapterStabilityTruthSupportExport {
        AdapterStabilityTruthSupportExport {
            record_kind: ADAPTER_STABILITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ADAPTER_STABILITY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            adapter_stability_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            adapter_stability_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != ADAPTER_STABILITY_TRUTH_PACKET_RECORD_KIND {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "adapter-stability packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != ADAPTER_STABILITY_TRUTH_SCHEMA_VERSION {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "adapter-stability packet has the wrong schema version",
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
                FindingKind::MissingAdapterLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered adapter lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingAdapterLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers adapter lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            self.append_per_row_findings(row, &mut findings);
        }

        for lane in &self.covered_lanes {
            self.append_per_lane_coverage_findings(*lane, &mut findings);
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
            self.append_projection_findings(projection, &mut findings);
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

    fn append_per_row_findings(
        &self,
        row: &AdapterStabilityRow,
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
                    "row {} admits raw source bodies past the boundary",
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
        if !row.degraded_provider_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDegradedProviderClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound degraded-provider class", row.row_id),
            ));
        }
        if !row.adapter_outcome_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingAdapterOutcomeClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound adapter-outcome class", row.row_id),
            ));
        }

        if matches!(row.support_class, SupportClass::LaunchStable) && !row.all_bindings_satisfied()
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchStableWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims launch_stable while a binding (support, known limit, downgrade automation, evidence, degraded-provider, or adapter-outcome) is unbound",
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

        let is_capability = matches!(
            row.row_class,
            AdapterStabilityRowClass::AdapterCapabilityTruth
        );
        let is_degraded_provider = matches!(
            row.row_class,
            AdapterStabilityRowClass::DegradedProviderAdmission
        );
        let is_adapter_outcome = matches!(
            row.row_class,
            AdapterStabilityRowClass::AdapterOutcomeAdmission
        );
        let is_launch_wedge =
            matches!(row.row_class, AdapterStabilityRowClass::LaunchWedgeCoverage);

        if is_capability
            && matches!(
                row.adapter_capability_class,
                AdapterCapabilityClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::AdapterCapabilityNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is an adapter_capability_truth but has no bound adapter capability",
                    row.row_id
                ),
            ));
        }
        if !is_capability
            && !matches!(
                row.adapter_capability_class,
                AdapterCapabilityClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::AdapterCapabilityNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds adapter capability {}; only adapter_capability_truth rows may bind a capability",
                    row.row_id,
                    row.row_class.as_str(),
                    row.adapter_capability_class.as_str()
                ),
            ));
        }

        if is_degraded_provider
            && matches!(
                row.degraded_provider_class,
                DegradedProviderClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::DegradedProviderNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a degraded_provider_admission but has no bound degraded-provider class",
                    row.row_id
                ),
            ));
        }
        if !is_degraded_provider
            && !matches!(
                row.degraded_provider_class,
                DegradedProviderClass::NotApplicable | DegradedProviderClass::StateUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::DegradedProviderNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds degraded-provider {}; only degraded_provider_admission rows may bind a degraded-provider class",
                    row.row_id,
                    row.row_class.as_str(),
                    row.degraded_provider_class.as_str()
                ),
            ));
        }

        if is_adapter_outcome
            && matches!(
                row.adapter_outcome_class,
                AdapterOutcomeClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::AdapterOutcomeNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is an adapter_outcome_admission but has no bound adapter-outcome class",
                    row.row_id
                ),
            ));
        }
        if !is_adapter_outcome
            && !matches!(
                row.adapter_outcome_class,
                AdapterOutcomeClass::NotApplicable | AdapterOutcomeClass::OutcomeUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::AdapterOutcomeNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds adapter outcome {}; only adapter_outcome_admission rows may bind an adapter outcome",
                    row.row_id,
                    row.row_class.as_str(),
                    row.adapter_outcome_class.as_str()
                ),
            ));
        }

        if is_launch_wedge && matches!(row.launch_wedge_class, LaunchWedgeClass::NotApplicable) {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchWedgeNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a launch_wedge_coverage but has no bound launch wedge",
                    row.row_id
                ),
            ));
        }
        if !is_launch_wedge && !matches!(row.launch_wedge_class, LaunchWedgeClass::NotApplicable) {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchWedgeNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds launch wedge {}; only launch_wedge_coverage rows may bind a launch wedge",
                    row.row_id,
                    row.row_class.as_str(),
                    row.launch_wedge_class.as_str()
                ),
            ));
        }

        if matches!(
            row.confidence_class,
            AdapterStabilityConfidenceClass::LowConfidence
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

    fn append_per_lane_coverage_findings(
        &self,
        lane: AdapterLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_stable = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    AdapterStabilityRowClass::AdapterStabilityQuality
                )
                && matches!(row.support_class, SupportClass::LaunchStable)
        });
        if !lane_claims_stable {
            return;
        }

        for capability in AdapterCapabilityClass::REQUIRED_FOR_LAUNCH {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        AdapterStabilityRowClass::AdapterCapabilityTruth
                    )
                    && row.adapter_capability_class == capability
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingAdapterCapabilityCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no adapter_capability_truth row for {}",
                        lane.as_str(),
                        capability.as_str()
                    ),
                ));
            }
        }

        for provider_state in DegradedProviderClass::REQUIRED_FOR_LAUNCH {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        AdapterStabilityRowClass::DegradedProviderAdmission
                    )
                    && row.degraded_provider_class == provider_state
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDegradedProviderCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no degraded_provider_admission row for {}",
                        lane.as_str(),
                        provider_state.as_str()
                    ),
                ));
            }
        }

        let outcome_covered = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    AdapterStabilityRowClass::AdapterOutcomeAdmission
                )
        });
        if !outcome_covered {
            findings.push(ValidationFinding::new(
                FindingKind::MissingAdapterOutcomeCoverage,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no adapter_outcome_admission row",
                    lane.as_str()
                ),
            ));
        }

        let wedge_covered = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(row.row_class, AdapterStabilityRowClass::LaunchWedgeCoverage)
        });
        if !wedge_covered {
            findings.push(ValidationFinding::new(
                FindingKind::MissingLaunchWedgeCoverage,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no launch_wedge_coverage row",
                    lane.as_str()
                ),
            ));
        }
    }

    fn append_projection_findings(
        &self,
        projection: &AdapterStabilityConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve adapter-stability truth",
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
        if !projection.preserves_adapter_capability_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::AdapterCapabilityVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the adapter-capability vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_degraded_provider_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::DegradedProviderVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the degraded-provider vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_adapter_outcome_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::AdapterOutcomeVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the adapter-outcome vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_launch_wedge_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchWedgeVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the launch-wedge vocabulary",
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
pub struct AdapterStabilityTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub adapter_stability_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub adapter_stability_packet: AdapterStabilityTruthPacket,
}

impl AdapterStabilityTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == ADAPTER_STABILITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == ADAPTER_STABILITY_TRUTH_SCHEMA_VERSION
            && self.adapter_stability_packet_id_ref == self.adapter_stability_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.adapter_stability_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable adapter-stability packet.
#[derive(Debug)]
pub enum AdapterStabilityTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for AdapterStabilityTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "adapter-stability packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "adapter-stability packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AdapterStabilityTruthArtifactError {}

/// Returns the checked-in stable adapter-stability truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_adapter_stability_truth_packet(
) -> Result<AdapterStabilityTruthPacket, AdapterStabilityTruthArtifactError> {
    let packet: AdapterStabilityTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m4/adapter_stability_truth_packet.json"
    )))
    .map_err(AdapterStabilityTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(AdapterStabilityTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        ADAPTER_STABILITY_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        ADAPTER_STABILITY_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn base_row(
        row_id: &str,
        lane: AdapterLaneClass,
        row_class: AdapterStabilityRowClass,
    ) -> AdapterStabilityRow {
        AdapterStabilityRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class,
            support_class: SupportClass::LaunchStable,
            adapter_capability_class: AdapterCapabilityClass::NotApplicable,
            degraded_provider_class: DegradedProviderClass::NotApplicable,
            adapter_outcome_class: AdapterOutcomeClass::NotApplicable,
            launch_wedge_class: LaunchWedgeClass::NotApplicable,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
            confidence_class: AdapterStabilityConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn quality_row(lane: AdapterLaneClass, prefix: &str) -> AdapterStabilityRow {
        let mut row = base_row(
            &format!("row:{prefix}:quality"),
            lane,
            AdapterStabilityRowClass::AdapterStabilityQuality,
        );
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
        row.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
        row.evidence_refs = vec![doc_ref(), fixture_ref()];
        row
    }

    fn capability_rows(lane: AdapterLaneClass, prefix: &str) -> Vec<AdapterStabilityRow> {
        AdapterCapabilityClass::REQUIRED_FOR_LAUNCH
            .into_iter()
            .map(|capability| {
                let mut row = base_row(
                    &format!("row:{prefix}:capability:{}", capability.as_str()),
                    lane,
                    AdapterStabilityRowClass::AdapterCapabilityTruth,
                );
                row.adapter_capability_class = capability;
                row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
                row
            })
            .collect()
    }

    fn degraded_provider_rows(lane: AdapterLaneClass, prefix: &str) -> Vec<AdapterStabilityRow> {
        DegradedProviderClass::REQUIRED_FOR_LAUNCH
            .into_iter()
            .map(|state| {
                let mut row = base_row(
                    &format!("row:{prefix}:degraded_provider:{}", state.as_str()),
                    lane,
                    AdapterStabilityRowClass::DegradedProviderAdmission,
                );
                row.degraded_provider_class = state;
                row.evidence_class = EvidenceClass::FixtureRepoEvidence;
                row
            })
            .collect()
    }

    fn outcome_row(lane: AdapterLaneClass, prefix: &str) -> AdapterStabilityRow {
        let mut row = base_row(
            &format!("row:{prefix}:outcome"),
            lane,
            AdapterStabilityRowClass::AdapterOutcomeAdmission,
        );
        row.adapter_outcome_class = AdapterOutcomeClass::Passed;
        row.evidence_class = EvidenceClass::FixtureRepoEvidence;
        row
    }

    fn wedge_row(
        lane: AdapterLaneClass,
        prefix: &str,
        wedge: LaunchWedgeClass,
    ) -> AdapterStabilityRow {
        let mut row = base_row(
            &format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane,
            AdapterStabilityRowClass::LaunchWedgeCoverage,
        );
        row.launch_wedge_class = wedge;
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row
    }

    fn projection(surface: ConsumerSurface) -> AdapterStabilityConsumerProjection {
        AdapterStabilityConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            adapter_stability_packet_id_ref: "packet:m4:adapter_stability".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_adapter_capability_vocabulary: true,
            preserves_degraded_provider_vocabulary: true,
            preserves_adapter_outcome_vocabulary: true,
            preserves_launch_wedge_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(
        lane: AdapterLaneClass,
        prefix: &str,
        wedge: LaunchWedgeClass,
    ) -> Vec<AdapterStabilityRow> {
        let mut rows = vec![quality_row(lane, prefix)];
        rows.extend(capability_rows(lane, prefix));
        rows.extend(degraded_provider_rows(lane, prefix));
        rows.push(outcome_row(lane, prefix));
        rows.push(wedge_row(lane, prefix, wedge));
        rows
    }

    fn sample_input() -> AdapterStabilityTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(
            AdapterLaneClass::FormatterLane,
            "formatter",
            LaunchWedgeClass::PythonWedge,
        ));
        rows.extend(lane_rows(
            AdapterLaneClass::LinterLane,
            "linter",
            LaunchWedgeClass::TypescriptJavascriptWedge,
        ));
        rows.extend(lane_rows(
            AdapterLaneClass::TestAdapterLane,
            "tests",
            LaunchWedgeClass::RustWedge,
        ));
        let mut projections = Vec::new();
        for surface in ConsumerSurface::REQUIRED {
            projections.push(projection(surface));
        }
        AdapterStabilityTruthPacketInput {
            packet_id: "packet:m4:adapter_stability".to_owned(),
            workflow_or_surface_id: "workflow.language.adapter_stability".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: AdapterLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: projections,
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(AdapterLaneClass::FormatterLane.as_str(), "formatter_lane");
        assert_eq!(AdapterLaneClass::LinterLane.as_str(), "linter_lane");
        assert_eq!(
            AdapterLaneClass::TestAdapterLane.as_str(),
            "test_adapter_lane"
        );
        assert_eq!(
            AdapterStabilityRowClass::AdapterStabilityQuality.as_str(),
            "adapter_stability_quality"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(AdapterCapabilityClass::Discover.as_str(), "discover");
        assert_eq!(AdapterCapabilityClass::Report.as_str(), "report");
        assert_eq!(
            DegradedProviderClass::ProviderHealthy.as_str(),
            "provider_healthy"
        );
        assert_eq!(
            DegradedProviderClass::StateUnbound.as_str(),
            "state_unbound"
        );
        assert_eq!(AdapterOutcomeClass::Passed.as_str(), "passed");
        assert_eq!(
            AdapterOutcomeClass::OutcomeUnbound.as_str(),
            "outcome_unbound"
        );
        assert_eq!(LaunchWedgeClass::PythonWedge.as_str(), "python_wedge");
        assert_eq!(LaunchWedgeClass::CCppWedge.as_str(), "c_cpp_wedge");
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
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            FindingKind::MissingDegradedProviderCoverage.as_str(),
            "missing_degraded_provider_coverage"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = AdapterStabilityTruthPacket::materialize(sample_input());
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
            .support_export("support:m4:adapter_stability", "2026-05-26T12:00:10Z")
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = AdapterStabilityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_degraded_provider_state_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(row.lane_class == AdapterLaneClass::FormatterLane
                && matches!(
                    row.row_class,
                    AdapterStabilityRowClass::DegradedProviderAdmission
                )
                && row.degraded_provider_class == DegradedProviderClass::ProviderUnavailable)
        });
        let packet = AdapterStabilityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDegradedProviderCoverage));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = AdapterStabilityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|p| p.consumer_surface != ConsumerSurface::ConformanceDashboard);
        let packet = AdapterStabilityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_degraded_provider_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_degraded_provider_vocabulary = false;
            }
        }
        let packet = AdapterStabilityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(
            |finding| finding.finding_kind == FindingKind::DegradedProviderVocabularyCollapsed
        ));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = AdapterStabilityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
