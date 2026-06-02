//! Launch-language conformance pack publication truth packet for the
//! M4 stable lane.
//!
//! This module is the language-owned contract that pins how conformance
//! packs, support-class evidence, and downgrade rules stay one boundary
//! truth for every stable launch-language lane (Python, TypeScript /
//! JavaScript, Rust, Go, Java / Kotlin, and C / C++). Surfaces such as
//! the editor language pack, framework pack panel, language
//! settings / help, CLI / headless inspector, support export, release
//! proof index, Help / About proof card, and the conformance dashboard
//! MUST read this packet verbatim — they do not mint local copies or
//! paraphrase conformance posture.
//!
//! Every row binds a closed `launch_language_lane_class`,
//! `conformance_pack_row_class`, `support_class`,
//! `support_class_evidence_class`, `downgrade_rule_class`,
//! `daily_loop_step_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `conformance_pack_confidence_class`
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the
//! row is narrowed below launch-stable, declares a non-`none_declared`
//! known limit, or binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! source bodies, raw fixture payloads, raw archetype-repo bodies,
//! secrets, ambient credentials, or other private material past the
//! boundary. A row that claims `launch_stable` while leaving its
//! support, support-class-evidence, downgrade-rule, evidence, known
//! limit, or downgrade automation class unbound is refused; the
//! validator narrows below launch-stable instead of inheriting an
//! adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`PublishLaunchLanguageConformancePacksTruthPacket`].
pub const PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_PACKET_RECORD_KIND: &str =
    "publish_launch_language_conformance_packs_truth_stable_packet";

/// Stable record-kind tag for
/// [`PublishLaunchLanguageConformancePacksTruthSupportExport`].
pub const PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "publish_launch_language_conformance_packs_truth_support_export";

/// Integer schema version for the conformance-packs truth packet.
pub const PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SCHEMA_REF: &str =
    "schemas/language/publish_launch_language_conformance_packs_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_DOC_REF: &str =
    "docs/languages/m4/publish-launch-language-conformance-packs-support-class-evidence.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m4/publish-launch-language-conformance-packs-support-class-evidence.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m4/publish_launch_language_conformance_packs_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m4/publish_launch_language_conformance_packs_truth_packet.json";

/// Closed launch-language lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchLanguageLaneClass {
    /// Python launch-language lane.
    PythonLaunchLanguageLane,
    /// TypeScript / JavaScript launch-language lane.
    TypescriptJavascriptLaunchLanguageLane,
    /// Rust launch-language lane.
    RustLaunchLanguageLane,
    /// Go launch-language lane.
    GoLaunchLanguageLane,
    /// Java / Kotlin launch-language lane.
    JavaKotlinLaunchLanguageLane,
    /// C / C++ launch-language lane.
    CCppLaunchLanguageLane,
}

impl LaunchLanguageLaneClass {
    /// Every required launch-language lane, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::PythonLaunchLanguageLane,
        Self::TypescriptJavascriptLaunchLanguageLane,
        Self::RustLaunchLanguageLane,
        Self::GoLaunchLanguageLane,
        Self::JavaKotlinLaunchLanguageLane,
        Self::CCppLaunchLanguageLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PythonLaunchLanguageLane => "python_launch_language_lane",
            Self::TypescriptJavascriptLaunchLanguageLane => {
                "typescript_javascript_launch_language_lane"
            }
            Self::RustLaunchLanguageLane => "rust_launch_language_lane",
            Self::GoLaunchLanguageLane => "go_launch_language_lane",
            Self::JavaKotlinLaunchLanguageLane => "java_kotlin_launch_language_lane",
            Self::CCppLaunchLanguageLane => "c_cpp_launch_language_lane",
        }
    }
}

/// Closed conformance-pack row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformancePackRowClass {
    /// The lane's headline conformance-pack qualification row.
    ConformancePackQuality,
    /// Support-class evidence admission row binding one
    /// `support_class_evidence_class`.
    SupportClassEvidenceAdmission,
    /// Downgrade-rule admission row binding one `downgrade_rule_class`.
    DowngradeRuleAdmission,
    /// One step of the certified daily loop on a lane.
    DailyLoopStep,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl ConformancePackRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConformancePackQuality => "conformance_pack_quality",
            Self::SupportClassEvidenceAdmission => "support_class_evidence_admission",
            Self::DowngradeRuleAdmission => "downgrade_rule_admission",
            Self::DailyLoopStep => "daily_loop_step",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound daily-loop step.
    pub const fn requires_daily_loop_step(self) -> bool {
        matches!(self, Self::DailyLoopStep)
    }

    /// True when this row class requires a bound support-class-evidence class.
    pub const fn requires_support_class_evidence(self) -> bool {
        matches!(self, Self::SupportClassEvidenceAdmission)
    }

    /// True when this row class requires a bound downgrade-rule class.
    pub const fn requires_downgrade_rule(self) -> bool {
        matches!(self, Self::DowngradeRuleAdmission)
    }
}

/// Closed support-class vocabulary applied to a conformance-pack row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-stable grade for the launch-language lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch stable; the narrowing
    /// is disclosed.
    LaunchStableBelow,
    /// Row is at beta-grade only (capability sample, not launch stable).
    BetaGradeOnly,
    /// Row is at preview only (under-review wedge).
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

/// Closed support-class-evidence vocabulary. A
/// `support_class_evidence_admission` row binds exactly one
/// support-class evidence class so the conformance pack can prove that
/// its claimed support class is backed by current fixture-repo,
/// archetype, conformance, benchmark, or design-partner evidence rather
/// than docs-only assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClassEvidenceClass {
    /// The support class is backed by a certified archetype repo capture.
    ArchetypeRepoBacked,
    /// The support class is backed by a fixture corpus capture.
    FixtureCorpusBacked,
    /// The support class is backed by a conformance suite run.
    ConformanceSuiteBacked,
    /// The support class is backed by a benchmark / fitness function capture.
    BenchmarkBacked,
    /// The support class is backed by design-partner evidence.
    DesignPartnerBacked,
    /// The support class is backed by framework- / library-migration evidence.
    FrameworkMigrationBacked,
    /// The support class is only backed by a docs / help disclosure
    /// (gap label only).
    DocsDisclosureOnly,
    /// Row is not a support-class-evidence admission row.
    NotApplicable,
    /// The support-class evidence class is unbound; this never qualifies stable.
    EvidenceUnbound,
}

impl SupportClassEvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchetypeRepoBacked => "archetype_repo_backed",
            Self::FixtureCorpusBacked => "fixture_corpus_backed",
            Self::ConformanceSuiteBacked => "conformance_suite_backed",
            Self::BenchmarkBacked => "benchmark_backed",
            Self::DesignPartnerBacked => "design_partner_backed",
            Self::FrameworkMigrationBacked => "framework_migration_backed",
            Self::DocsDisclosureOnly => "docs_disclosure_only",
            Self::NotApplicable => "not_applicable",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this support-class evidence class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed downgrade-rule vocabulary. A `downgrade_rule_admission` row
/// binds exactly one downgrade rule the conformance pack publishes for
/// the lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeRuleClass {
    /// Narrow the lane when a certified fixture is missing or stale.
    NarrowOnMissingFixture,
    /// Narrow the lane when a certified archetype repo is missing.
    NarrowOnMissingArchetype,
    /// Narrow the lane when a migration probe fails.
    NarrowOnFailedMigration,
    /// Narrow the lane when confidence drops below the certified bar.
    NarrowOnLowConfidence,
    /// Block stable publication when required evidence is missing.
    BlockOnMissingEvidence,
    /// Block stable publication when an unsupported gap is detected.
    BlockOnUnsupportedGap,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Row is not a downgrade-rule admission row.
    NotApplicable,
    /// The downgrade rule is unbound; this never qualifies stable.
    RuleUnbound,
}

impl DowngradeRuleClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowOnMissingFixture => "narrow_on_missing_fixture",
            Self::NarrowOnMissingArchetype => "narrow_on_missing_archetype",
            Self::NarrowOnFailedMigration => "narrow_on_failed_migration",
            Self::NarrowOnLowConfidence => "narrow_on_low_confidence",
            Self::BlockOnMissingEvidence => "block_on_missing_evidence",
            Self::BlockOnUnsupportedGap => "block_on_unsupported_gap",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::NotApplicable => "not_applicable",
            Self::RuleUnbound => "rule_unbound",
        }
    }

    /// True when this downgrade-rule class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::RuleUnbound)
    }
}

/// Closed daily-loop step vocabulary. A lane that claims
/// `launch_stable` MUST cover every certified step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopStepClass {
    /// The row certifies the open / import step of the daily loop.
    OpenOrImport,
    /// The row certifies the navigate step of the daily loop.
    Navigate,
    /// The row certifies the edit step of the daily loop.
    Edit,
    /// The row certifies the complete step of the daily loop.
    Complete,
    /// The row certifies the refactor step of the daily loop.
    Refactor,
    /// The row certifies the run / test / debug step of the daily loop.
    RunTestDebug,
    /// The row certifies the review step of the daily loop.
    Review,
    /// The row certifies the migrate step of the daily loop.
    Migrate,
    /// The row certifies the recover step of the daily loop.
    Recover,
    /// The row is not bound to a daily-loop step (non-loop row classes).
    NotApplicable,
}

impl DailyLoopStepClass {
    /// Every certified daily-loop step in declaration order. A
    /// `launch_stable` lane MUST cover every step.
    pub const REQUIRED_FOR_LAUNCH: [Self; 9] = [
        Self::OpenOrImport,
        Self::Navigate,
        Self::Edit,
        Self::Complete,
        Self::Refactor,
        Self::RunTestDebug,
        Self::Review,
        Self::Migrate,
        Self::Recover,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenOrImport => "open_or_import",
            Self::Navigate => "navigate",
            Self::Edit => "edit",
            Self::Complete => "complete",
            Self::Refactor => "refactor",
            Self::RunTestDebug => "run_test_debug",
            Self::Review => "review",
            Self::Migrate => "migrate",
            Self::Recover => "recover",
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
    /// The row is backed by framework- / library-migration evidence.
    FrameworkMigrationEvidence,
    /// The row is backed by design-partner evidence.
    DesignPartnerEvidence,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a conformance suite run.
    ConformanceSuiteEvidence,
    /// The row is backed by a benchmark / fitness function capture.
    BenchmarkEvidence,
    /// The row is backed by a docs / help disclosure (gap label only).
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

/// Closed known-limit vocabulary attached to a conformance-pack row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies a language subset (e.g., one dialect / version).
    LanguageSubsetOnly,
    /// The row only certifies a framework / library subset.
    FrameworkSubsetOnly,
    /// The row only certifies an archetype subset (specific repos).
    ArchetypeSubsetOnly,
    /// The row only certifies a daily-loop-step subset.
    DailyLoopStepSubsetOnly,
    /// The row only certifies a migration subset.
    MigrationSubsetOnly,
    /// The row certifies an unsupported runtime target gap.
    UnsupportedRuntimeTarget,
    /// The row certifies a beta-grade-only capability gap.
    BetaCapabilitySampleOnly,
    /// The row has no bound known-limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::LanguageSubsetOnly => "language_subset_only",
            Self::FrameworkSubsetOnly => "framework_subset_only",
            Self::ArchetypeSubsetOnly => "archetype_subset_only",
            Self::DailyLoopStepSubsetOnly => "daily_loop_step_subset_only",
            Self::MigrationSubsetOnly => "migration_subset_only",
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

/// Closed downgrade-automation vocabulary attached to a conformance-pack row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a certified fixture is missing or stale.
    AutoNarrowOnMissingFixture,
    /// Automatically narrow when a certified archetype repo is missing.
    AutoNarrowOnMissingArchetype,
    /// Automatically narrow when a migration probe fails.
    AutoNarrowOnFailedMigration,
    /// Automatically demote when confidence drops below the certified bar.
    AutoDemoteOnLowConfidence,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Automatically block when an unsupported gap is detected.
    AutoBlockOnUnsupportedGap,
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
            Self::AutoNarrowOnFailedMigration => "auto_narrow_on_failed_migration",
            Self::AutoDemoteOnLowConfidence => "auto_demote_on_low_confidence",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::AutoBlockOnUnsupportedGap => "auto_block_on_unsupported_gap",
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

/// Closed confidence-class vocabulary for a conformance-pack row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformancePackConfidenceClass {
    /// High confidence — the lane can certify launch stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch stable until evidence grows.
    LowConfidence,
}

impl ConformancePackConfidenceClass {
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

/// Closed validation-finding vocabulary for the conformance-pack packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required launch-language lane has no row.
    MissingLaneCoverage,
    /// A lane claiming launch_stable is missing a certified daily-loop step.
    MissingDailyLoopStepCoverage,
    /// A lane claiming launch_stable is missing a
    /// support_class_evidence_admission row.
    MissingSupportClassEvidenceCoverage,
    /// A lane claiming launch_stable is missing a downgrade_rule_admission row.
    MissingDowngradeRuleCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row has no bound support-class-evidence class on a row-class
    /// that requires it.
    MissingSupportClassEvidence,
    /// A row has no bound downgrade-rule class on a row-class that requires it.
    MissingDowngradeRule,
    /// A row claims launch_stable while one or more bindings is unbound.
    LaunchStableWithUnboundBinding,
    /// A row narrowed below launch stable drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A daily-loop-step row drops its daily-loop step binding.
    DailyLoopStepNotApplicable,
    /// A non-daily-loop row binds a daily-loop step it cannot certify.
    DailyLoopStepNotPermittedOnRowClass,
    /// A support_class_evidence_admission row drops its
    /// support-class-evidence binding.
    SupportClassEvidenceNotApplicable,
    /// A non-support-class-evidence row binds a support-class-evidence class.
    SupportClassEvidenceNotPermittedOnRowClass,
    /// A downgrade_rule_admission row drops its downgrade-rule binding.
    DowngradeRuleNotApplicable,
    /// A non-downgrade-rule row binds a downgrade-rule class.
    DowngradeRuleNotPermittedOnRowClass,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority / credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops conformance-pack truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the support-class-evidence vocabulary.
    SupportClassEvidenceVocabularyCollapsed,
    /// A projection collapses the downgrade-rule vocabulary.
    DowngradeRuleVocabularyCollapsed,
    /// A projection collapses the daily-loop-step vocabulary.
    DailyLoopStepVocabularyCollapsed,
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
            Self::MissingDailyLoopStepCoverage => "missing_daily_loop_step_coverage",
            Self::MissingSupportClassEvidenceCoverage => "missing_support_class_evidence_coverage",
            Self::MissingDowngradeRuleCoverage => "missing_downgrade_rule_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingSupportClassEvidence => "missing_support_class_evidence",
            Self::MissingDowngradeRule => "missing_downgrade_rule",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::DailyLoopStepNotApplicable => "daily_loop_step_not_applicable",
            Self::DailyLoopStepNotPermittedOnRowClass => {
                "daily_loop_step_not_permitted_on_row_class"
            }
            Self::SupportClassEvidenceNotApplicable => "support_class_evidence_not_applicable",
            Self::SupportClassEvidenceNotPermittedOnRowClass => {
                "support_class_evidence_not_permitted_on_row_class"
            }
            Self::DowngradeRuleNotApplicable => "downgrade_rule_not_applicable",
            Self::DowngradeRuleNotPermittedOnRowClass => {
                "downgrade_rule_not_permitted_on_row_class"
            }
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::SupportClassEvidenceVocabularyCollapsed => {
                "support_class_evidence_vocabulary_collapsed"
            }
            Self::DowngradeRuleVocabularyCollapsed => "downgrade_rule_vocabulary_collapsed",
            Self::DailyLoopStepVocabularyCollapsed => "daily_loop_step_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the conformance-pack packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Editor language-pack surface (in-editor support badge / hover).
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
    /// Help / About proof card surface.
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

/// One conformance-pack row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishLaunchLanguageConformancePacksRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Launch-language lane this row certifies.
    pub lane_class: LaunchLanguageLaneClass,
    /// Conformance-pack row class.
    pub row_class: ConformancePackRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Support-class-evidence class (or `not_applicable`).
    pub support_class_evidence_class: SupportClassEvidenceClass,
    /// Downgrade-rule class (or `not_applicable`).
    pub downgrade_rule_class: DowngradeRuleClass,
    /// Daily-loop step certified by the row (or `not_applicable`).
    pub daily_loop_step_class: DailyLoopStepClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConformancePackConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit, or
    /// binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// True when raw source bodies are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority / credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl PublishLaunchLanguageConformancePacksRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.support_class_evidence_class.is_bound()
            && self.downgrade_rule_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishLaunchLanguageConformancePacksConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Conformance-pack packet id consumed by the projection.
    pub publish_launch_language_conformance_packs_packet_id_ref: String,
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
    /// True when the support-class-evidence vocabulary is preserved verbatim.
    pub preserves_support_class_evidence_vocabulary: bool,
    /// True when the downgrade-rule vocabulary is preserved verbatim.
    pub preserves_downgrade_rule_vocabulary: bool,
    /// True when the daily-loop-step vocabulary is preserved verbatim.
    pub preserves_daily_loop_step_vocabulary: bool,
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
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl PublishLaunchLanguageConformancePacksConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.publish_launch_language_conformance_packs_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_support_class_evidence_vocabulary
            && self.preserves_downgrade_rule_vocabulary
            && self.preserves_daily_loop_step_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`PublishLaunchLanguageConformancePacksTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishLaunchLanguageConformancePacksTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Launch-language lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<LaunchLanguageLaneClass>,
    /// Conformance-pack rows.
    #[serde(default)]
    pub rows: Vec<PublishLaunchLanguageConformancePacksRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<PublishLaunchLanguageConformancePacksConsumerProjection>,
    /// Source contracts (docs / schema / fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying launch-language conformance packs,
/// support-class evidence, and downgrade rules for every stable lane at
/// the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishLaunchLanguageConformancePacksTruthPacket {
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
    /// Launch-language lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<LaunchLanguageLaneClass>,
    /// Conformance-pack rows.
    #[serde(default)]
    pub rows: Vec<PublishLaunchLanguageConformancePacksRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<PublishLaunchLanguageConformancePacksConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl PublishLaunchLanguageConformancePacksTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: PublishLaunchLanguageConformancePacksTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_PACKET_RECORD_KIND
                .to_owned(),
            schema_version: PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable conformance-pack invariants.
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
            .map(LaunchLanguageLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(ConformancePackRowClass::as_str)
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

    /// Returns the unique support-class-evidence tokens observed across rows.
    pub fn support_class_evidence_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class_evidence_class);
        }
        set.into_iter()
            .map(SupportClassEvidenceClass::as_str)
            .collect()
    }

    /// Returns the unique downgrade-rule tokens observed across rows.
    pub fn downgrade_rule_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_rule_class);
        }
        set.into_iter().map(DowngradeRuleClass::as_str).collect()
    }

    /// Returns the unique daily-loop-step tokens observed across rows.
    pub fn daily_loop_step_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.daily_loop_step_class);
        }
        set.into_iter().map(DailyLoopStepClass::as_str).collect()
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
    ) -> PublishLaunchLanguageConformancePacksTruthSupportExport {
        PublishLaunchLanguageConformancePacksTruthSupportExport {
            record_kind: PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SUPPORT_EXPORT_RECORD_KIND
                .to_owned(),
            schema_version: PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            publish_launch_language_conformance_packs_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            publish_launch_language_conformance_packs_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind
                != PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "conformance-pack packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "conformance-pack packet has the wrong schema version",
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
                "packet must declare at least one covered launch-language lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers launch-language lane {}", lane.as_str()),
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
        row: &PublishLaunchLanguageConformancePacksRow,
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
        if !row.support_class_evidence_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSupportClassEvidence,
                FindingSeverity::Blocker,
                format!(
                    "row {} has no bound support-class-evidence class",
                    row.row_id
                ),
            ));
        }
        if !row.downgrade_rule_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDowngradeRule,
                FindingSeverity::Blocker,
                format!("row {} has no bound downgrade-rule class", row.row_id),
            ));
        }

        if matches!(row.support_class, SupportClass::LaunchStable) && !row.all_bindings_satisfied()
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchStableWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims launch_stable while a binding (support, known limit, downgrade automation, evidence, support-class evidence, or downgrade rule) is unbound",
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

        if row.row_class.requires_daily_loop_step()
            && matches!(row.daily_loop_step_class, DailyLoopStepClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::DailyLoopStepNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a daily_loop_step but has no bound daily-loop step",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_daily_loop_step()
            && !matches!(row.daily_loop_step_class, DailyLoopStepClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::DailyLoopStepNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds daily-loop step {}; only daily_loop_step rows may bind a step",
                    row.row_id,
                    row.row_class.as_str(),
                    row.daily_loop_step_class.as_str()
                ),
            ));
        }

        if row.row_class.requires_support_class_evidence()
            && matches!(
                row.support_class_evidence_class,
                SupportClassEvidenceClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::SupportClassEvidenceNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a support_class_evidence_admission but has no bound support-class-evidence class",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_support_class_evidence()
            && !matches!(
                row.support_class_evidence_class,
                SupportClassEvidenceClass::NotApplicable
                    | SupportClassEvidenceClass::EvidenceUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::SupportClassEvidenceNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds support-class evidence {}; only support_class_evidence_admission rows may bind a class",
                    row.row_id,
                    row.row_class.as_str(),
                    row.support_class_evidence_class.as_str()
                ),
            ));
        }

        if row.row_class.requires_downgrade_rule()
            && matches!(row.downgrade_rule_class, DowngradeRuleClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeRuleNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a downgrade_rule_admission but has no bound downgrade-rule class",
                    row.row_id
                ),
            ));
        }
        if !row.row_class.requires_downgrade_rule()
            && !matches!(
                row.downgrade_rule_class,
                DowngradeRuleClass::NotApplicable | DowngradeRuleClass::RuleUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeRuleNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds downgrade rule {}; only downgrade_rule_admission rows may bind a rule",
                    row.row_id,
                    row.row_class.as_str(),
                    row.downgrade_rule_class.as_str()
                ),
            ));
        }

        if matches!(
            row.confidence_class,
            ConformancePackConfidenceClass::LowConfidence
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
        lane: LaunchLanguageLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_stable = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    ConformancePackRowClass::ConformancePackQuality
                )
                && matches!(row.support_class, SupportClass::LaunchStable)
        });
        if !lane_claims_stable {
            return;
        }

        for step in DailyLoopStepClass::REQUIRED_FOR_LAUNCH {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(row.row_class, ConformancePackRowClass::DailyLoopStep)
                    && row.daily_loop_step_class == step
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDailyLoopStepCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no daily_loop_step row for {}",
                        lane.as_str(),
                        step.as_str()
                    ),
                ));
            }
        }

        let evidence_covered = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    ConformancePackRowClass::SupportClassEvidenceAdmission
                )
                && row.support_class_evidence_class.is_bound()
                && !matches!(
                    row.support_class_evidence_class,
                    SupportClassEvidenceClass::NotApplicable
                )
        });
        if !evidence_covered {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSupportClassEvidenceCoverage,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no support_class_evidence_admission row",
                    lane.as_str()
                ),
            ));
        }

        let downgrade_covered = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    ConformancePackRowClass::DowngradeRuleAdmission
                )
                && row.downgrade_rule_class.is_bound()
                && !matches!(row.downgrade_rule_class, DowngradeRuleClass::NotApplicable)
        });
        if !downgrade_covered {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDowngradeRuleCoverage,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no downgrade_rule_admission row",
                    lane.as_str()
                ),
            ));
        }
    }

    fn append_projection_findings(
        &self,
        projection: &PublishLaunchLanguageConformancePacksConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve conformance-pack truth",
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
        if !projection.preserves_support_class_evidence_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::SupportClassEvidenceVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the support-class-evidence vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_downgrade_rule_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::DowngradeRuleVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the downgrade-rule vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_daily_loop_step_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::DailyLoopStepVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the daily-loop-step vocabulary",
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
pub struct PublishLaunchLanguageConformancePacksTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub publish_launch_language_conformance_packs_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub publish_launch_language_conformance_packs_packet:
        PublishLaunchLanguageConformancePacksTruthPacket,
}

impl PublishLaunchLanguageConformancePacksTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind
            == PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_SCHEMA_VERSION
            && self.publish_launch_language_conformance_packs_packet_id_ref
                == self
                    .publish_launch_language_conformance_packs_packet
                    .packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .publish_launch_language_conformance_packs_packet
                .validate()
                .is_empty()
    }
}

/// Errors emitted when reading the checked-in stable conformance-pack packet.
#[derive(Debug)]
pub enum PublishLaunchLanguageConformancePacksTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for PublishLaunchLanguageConformancePacksTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "conformance-pack packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "conformance-pack packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PublishLaunchLanguageConformancePacksTruthArtifactError {}

/// Returns the checked-in stable conformance-pack truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_publish_launch_language_conformance_packs_truth_packet() -> Result<
    PublishLaunchLanguageConformancePacksTruthPacket,
    PublishLaunchLanguageConformancePacksTruthArtifactError,
> {
    let packet: PublishLaunchLanguageConformancePacksTruthPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/language/m4/publish_launch_language_conformance_packs_truth_packet.json"
        )))
        .map_err(PublishLaunchLanguageConformancePacksTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(PublishLaunchLanguageConformancePacksTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        PUBLISH_LAUNCH_LANGUAGE_CONFORMANCE_PACKS_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn base_row(
        row_id: &str,
        lane: LaunchLanguageLaneClass,
        row_class: ConformancePackRowClass,
    ) -> PublishLaunchLanguageConformancePacksRow {
        PublishLaunchLanguageConformancePacksRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class,
            support_class: SupportClass::LaunchStable,
            support_class_evidence_class: SupportClassEvidenceClass::NotApplicable,
            downgrade_rule_class: DowngradeRuleClass::NotApplicable,
            daily_loop_step_class: DailyLoopStepClass::NotApplicable,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
            confidence_class: ConformancePackConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn quality_row(
        lane: LaunchLanguageLaneClass,
        prefix: &str,
    ) -> PublishLaunchLanguageConformancePacksRow {
        let mut row = base_row(
            &format!("row:{prefix}:quality"),
            lane,
            ConformancePackRowClass::ConformancePackQuality,
        );
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
        row.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
        row.evidence_refs = vec![doc_ref(), fixture_ref()];
        row
    }

    fn support_evidence_row(
        lane: LaunchLanguageLaneClass,
        prefix: &str,
    ) -> PublishLaunchLanguageConformancePacksRow {
        let mut row = base_row(
            &format!("row:{prefix}:support_evidence"),
            lane,
            ConformancePackRowClass::SupportClassEvidenceAdmission,
        );
        row.support_class_evidence_class = SupportClassEvidenceClass::ArchetypeRepoBacked;
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
        row.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
        row
    }

    fn downgrade_rule_row(
        lane: LaunchLanguageLaneClass,
        prefix: &str,
    ) -> PublishLaunchLanguageConformancePacksRow {
        let mut row = base_row(
            &format!("row:{prefix}:downgrade_rule"),
            lane,
            ConformancePackRowClass::DowngradeRuleAdmission,
        );
        row.downgrade_rule_class = DowngradeRuleClass::NarrowOnMissingFixture;
        row.evidence_class = EvidenceClass::DocsDisclosureEvidence;
        row
    }

    fn loop_step_row(
        lane: LaunchLanguageLaneClass,
        prefix: &str,
        step: DailyLoopStepClass,
    ) -> PublishLaunchLanguageConformancePacksRow {
        let mut row = base_row(
            &format!("row:{prefix}:loop:{}", step.as_str()),
            lane,
            ConformancePackRowClass::DailyLoopStep,
        );
        row.daily_loop_step_class = step;
        row.evidence_class = EvidenceClass::FixtureRepoEvidence;
        row
    }

    fn projection(
        surface: ConsumerSurface,
    ) -> PublishLaunchLanguageConformancePacksConsumerProjection {
        PublishLaunchLanguageConformancePacksConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            publish_launch_language_conformance_packs_packet_id_ref:
                "packet:m4:publish_launch_language_conformance_packs".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_support_class_evidence_vocabulary: true,
            preserves_downgrade_rule_vocabulary: true,
            preserves_daily_loop_step_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(
        lane: LaunchLanguageLaneClass,
        prefix: &str,
    ) -> Vec<PublishLaunchLanguageConformancePacksRow> {
        let mut rows = vec![
            quality_row(lane, prefix),
            support_evidence_row(lane, prefix),
            downgrade_rule_row(lane, prefix),
        ];
        for step in DailyLoopStepClass::REQUIRED_FOR_LAUNCH {
            rows.push(loop_step_row(lane, prefix, step));
        }
        rows
    }

    fn sample_input() -> PublishLaunchLanguageConformancePacksTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(
            LaunchLanguageLaneClass::PythonLaunchLanguageLane,
            "py",
        ));
        rows.extend(lane_rows(
            LaunchLanguageLaneClass::TypescriptJavascriptLaunchLanguageLane,
            "tsjs",
        ));
        rows.extend(lane_rows(
            LaunchLanguageLaneClass::RustLaunchLanguageLane,
            "rs",
        ));
        rows.extend(lane_rows(
            LaunchLanguageLaneClass::GoLaunchLanguageLane,
            "go",
        ));
        rows.extend(lane_rows(
            LaunchLanguageLaneClass::JavaKotlinLaunchLanguageLane,
            "jvm",
        ));
        rows.extend(lane_rows(
            LaunchLanguageLaneClass::CCppLaunchLanguageLane,
            "cc",
        ));
        let mut projections = Vec::new();
        for surface in ConsumerSurface::REQUIRED {
            projections.push(projection(surface));
        }
        PublishLaunchLanguageConformancePacksTruthPacketInput {
            packet_id: "packet:m4:publish_launch_language_conformance_packs".to_owned(),
            workflow_or_surface_id: "workflow.language.publish_launch_language_conformance_packs"
                .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: LaunchLanguageLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: projections,
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            LaunchLanguageLaneClass::PythonLaunchLanguageLane.as_str(),
            "python_launch_language_lane"
        );
        assert_eq!(
            LaunchLanguageLaneClass::TypescriptJavascriptLaunchLanguageLane.as_str(),
            "typescript_javascript_launch_language_lane"
        );
        assert_eq!(
            LaunchLanguageLaneClass::CCppLaunchLanguageLane.as_str(),
            "c_cpp_launch_language_lane"
        );
        assert_eq!(
            ConformancePackRowClass::ConformancePackQuality.as_str(),
            "conformance_pack_quality"
        );
        assert_eq!(
            ConformancePackRowClass::SupportClassEvidenceAdmission.as_str(),
            "support_class_evidence_admission"
        );
        assert_eq!(
            ConformancePackRowClass::DowngradeRuleAdmission.as_str(),
            "downgrade_rule_admission"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(
            SupportClassEvidenceClass::ArchetypeRepoBacked.as_str(),
            "archetype_repo_backed"
        );
        assert_eq!(
            SupportClassEvidenceClass::EvidenceUnbound.as_str(),
            "evidence_unbound"
        );
        assert_eq!(
            DowngradeRuleClass::NarrowOnMissingFixture.as_str(),
            "narrow_on_missing_fixture"
        );
        assert_eq!(DowngradeRuleClass::RuleUnbound.as_str(), "rule_unbound");
        assert_eq!(DailyLoopStepClass::Recover.as_str(), "recover");
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
            FindingKind::MissingSupportClassEvidenceCoverage.as_str(),
            "missing_support_class_evidence_coverage"
        );
        assert_eq!(
            FindingKind::MissingDowngradeRuleCoverage.as_str(),
            "missing_downgrade_rule_coverage"
        );
        assert_eq!(
            FindingKind::SupportClassEvidenceVocabularyCollapsed.as_str(),
            "support_class_evidence_vocabulary_collapsed"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(sample_input());
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
                "support:m4:publish_launch_language_conformance_packs",
                "2026-05-26T12:00:10Z",
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(input);
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
    fn missing_daily_loop_step_for_launch_stable_blocks() {
        let mut input = sample_input();
        input
            .rows
            .retain(|row| row.daily_loop_step_class != DailyLoopStepClass::Recover);
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDailyLoopStepCoverage));
    }

    #[test]
    fn missing_support_class_evidence_admission_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !matches!(
                row.row_class,
                ConformancePackRowClass::SupportClassEvidenceAdmission
            )
        });
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingSupportClassEvidenceCoverage
        }));
    }

    #[test]
    fn missing_downgrade_rule_admission_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !matches!(
                row.row_class,
                ConformancePackRowClass::DowngradeRuleAdmission
            )
        });
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDowngradeRuleCoverage));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef
        }));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|p| p.consumer_surface != ConsumerSurface::ConformanceDashboard);
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_support_class_evidence_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_support_class_evidence_vocabulary = false;
            }
        }
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::SupportClassEvidenceVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = PublishLaunchLanguageConformancePacksTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
