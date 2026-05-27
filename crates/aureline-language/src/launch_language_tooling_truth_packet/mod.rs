//! Launch-tooling language certification truth packet for the M4
//! stable lane.
//!
//! This module is the language-owned contract that pins how the
//! shell/bash, SQL, Markdown, JSON/YAML, and Git-oriented launch
//! language tooling lanes stay one boundary truth across the editor
//! language pack, framework pack panel, language settings/help,
//! CLI/headless inspector, support export, release proof index,
//! Help/About proof card, and the conformance dashboard. The five
//! launch-tooling lanes are certified at the M4 launch-support
//! grade — distinct from (and intentionally narrower than) the
//! replacement-grade daily-driver lanes pinned by
//! [`crate::daily_driver_quality_truth_packet`] and the other launch
//! daily-driver packets. Surfaces MUST NOT mint local copies or
//! paraphrase launch-tooling posture; they read this packet verbatim.
//!
//! Every row binds a closed `tooling_lane_class`,
//! `launch_tooling_row_class`, `support_class`,
//! `daily_loop_step_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `launch_tooling_confidence_class`
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the
//! row is narrowed below launch support, declares a non-`none_declared`
//! known limit, or binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! source bodies, shell command transcripts, SQL queries, raw Markdown
//! or JSON/YAML payloads, secrets, ambient credentials, or Git
//! credentials past the boundary. A row that claims `launch_support`
//! while leaving its known limit, downgrade automation, or evidence
//! class unbound is refused; the validator narrows below launch
//! support instead of inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`LaunchLanguageToolingTruthPacket`].
pub const LAUNCH_LANGUAGE_TOOLING_TRUTH_PACKET_RECORD_KIND: &str =
    "launch_language_tooling_truth_stable_packet";

/// Stable record-kind tag for [`LaunchLanguageToolingTruthSupportExport`].
pub const LAUNCH_LANGUAGE_TOOLING_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "launch_language_tooling_truth_support_export";

/// Integer schema version for the launch-tooling truth packet.
pub const LAUNCH_LANGUAGE_TOOLING_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const LAUNCH_LANGUAGE_TOOLING_TRUTH_SCHEMA_REF: &str =
    "schemas/language/launch_language_tooling_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const LAUNCH_LANGUAGE_TOOLING_TRUTH_DOC_REF: &str =
    "docs/languages/m4/certify-shell-bash-sql-markdown-json-yaml-and.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const LAUNCH_LANGUAGE_TOOLING_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m4/certify-shell-bash-sql-markdown-json-yaml-and.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const LAUNCH_LANGUAGE_TOOLING_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m4/launch_language_tooling_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const LAUNCH_LANGUAGE_TOOLING_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m4/launch_language_tooling_truth_packet.json";

/// Closed launch-tooling lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolingLaneClass {
    /// Shell and bash launch-tooling lane.
    ShellBashLane,
    /// SQL launch-tooling lane.
    SqlLane,
    /// Markdown launch-tooling lane.
    MarkdownLane,
    /// JSON and YAML launch-tooling lane.
    JsonYamlLane,
    /// Git-oriented launch-tooling lane (commit message,
    /// gitconfig, gitignore, gitattributes, diff/patch authoring).
    GitOrientedLane,
}

impl ToolingLaneClass {
    /// Every required launch-tooling lane, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::ShellBashLane,
        Self::SqlLane,
        Self::MarkdownLane,
        Self::JsonYamlLane,
        Self::GitOrientedLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellBashLane => "shell_bash_lane",
            Self::SqlLane => "sql_lane",
            Self::MarkdownLane => "markdown_lane",
            Self::JsonYamlLane => "json_yaml_lane",
            Self::GitOrientedLane => "git_oriented_lane",
        }
    }
}

/// Closed launch-tooling row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchToolingRowClass {
    /// The lane's headline launch-tooling qualification row.
    LaunchToolingQuality,
    /// One step of the certified daily loop on a lane.
    DailyLoopStep,
    /// Framework / formatter / linter pack evidence row on a lane.
    FrameworkPack,
    /// Migration evidence row on a lane.
    MigrationEvidence,
    /// Archetype repo / fixture-repo evidence row on a lane.
    ArchetypeRepoEvidence,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl LaunchToolingRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchToolingQuality => "launch_tooling_quality",
            Self::DailyLoopStep => "daily_loop_step",
            Self::FrameworkPack => "framework_pack",
            Self::MigrationEvidence => "migration_evidence",
            Self::ArchetypeRepoEvidence => "archetype_repo_evidence",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound daily-loop step.
    pub const fn requires_daily_loop_step(self) -> bool {
        matches!(self, Self::DailyLoopStep)
    }
}

/// Closed support-class vocabulary applied to a launch-tooling row. A
/// row is never `launch_support` while its known limit, downgrade
/// automation, or evidence class is unbound; the validator demotes it
/// instead of inheriting an adjacent launch-support row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-support grade for the launch-tooling lane.
    LaunchSupport,
    /// Row is intentionally narrowed below launch support; the narrowing is disclosed.
    LaunchSupportBelow,
    /// Row is at beta-grade only (capability sample, not launch support).
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
            Self::LaunchSupport => "launch_support",
            Self::LaunchSupportBelow => "launch_support_below",
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
        !matches!(self, Self::LaunchSupport)
    }
}

/// Closed daily-loop step vocabulary. The full daily loop MUST be
/// covered for each lane that claims `launch_support`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyLoopStepClass {
    /// The row certifies the open/import step of the daily loop.
    OpenOrImport,
    /// The row certifies the navigate step of the daily loop.
    Navigate,
    /// The row certifies the edit step of the daily loop.
    Edit,
    /// The row certifies the complete step of the daily loop.
    Complete,
    /// The row certifies the refactor step of the daily loop.
    Refactor,
    /// The row certifies the run/test/debug step of the daily loop.
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
    /// `launch_support` lane MUST cover every step.
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

/// Closed known-limit vocabulary attached to a launch-tooling row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies a framework / formatter / linter subset.
    FrameworkSubsetOnly,
    /// The row only certifies a language subset (e.g., one dialect/version).
    LanguageSubsetOnly,
    /// The row only certifies an archetype subset (specific repos).
    ArchetypeSubsetOnly,
    /// The row only certifies a migration subset.
    MigrationSubsetOnly,
    /// The row certifies a launch-grade tooling capability and intentionally
    /// excludes replacement-grade daily-driver behavior.
    LaunchToolingScopeOnly,
    /// The row certifies an unsupported runtime target gap.
    UnsupportedRuntimeTarget,
    /// The row certifies a beta-grade-only capability gap.
    BetaCapabilitySampleOnly,
    /// The row has no bound known limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::FrameworkSubsetOnly => "framework_subset_only",
            Self::LanguageSubsetOnly => "language_subset_only",
            Self::ArchetypeSubsetOnly => "archetype_subset_only",
            Self::MigrationSubsetOnly => "migration_subset_only",
            Self::LaunchToolingScopeOnly => "launch_tooling_scope_only",
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

/// Closed downgrade-automation vocabulary attached to a launch-tooling row.
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
    /// Automatically narrow when the framework pack drops below depth.
    AutoNarrowOnFrameworkGap,
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
            Self::AutoNarrowOnFailedMigration => "auto_narrow_on_failed_migration",
            Self::AutoNarrowOnFrameworkGap => "auto_narrow_on_framework_gap",
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

/// Closed confidence-class vocabulary for a launch-tooling row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchToolingConfidenceClass {
    /// High confidence — the lane can certify launch support.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch support.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch support until evidence grows.
    LowConfidence,
}

impl LaunchToolingConfidenceClass {
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

/// Closed validation-finding vocabulary for the launch-tooling packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required launch-tooling lane has no row.
    MissingToolingLaneCoverage,
    /// A lane claiming launch_support is missing a certified daily-loop step.
    MissingDailyLoopStepCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row claims launch_support while one or more bindings is unbound.
    LaunchSupportWithUnboundBinding,
    /// A row narrowed below launch support drops its disclosure ref.
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
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops launch-tooling truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
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
            Self::MissingToolingLaneCoverage => "missing_tooling_lane_coverage",
            Self::MissingDailyLoopStepCoverage => "missing_daily_loop_step_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::LaunchSupportWithUnboundBinding => "launch_support_with_unbound_binding",
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
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
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

/// Consumer surface that must inherit the launch-tooling packet verbatim.
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

/// One launch-tooling row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchLanguageToolingRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Launch-tooling lane class this row certifies.
    pub lane_class: ToolingLaneClass,
    /// Launch-tooling row class.
    pub row_class: LaunchToolingRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Daily-loop step certified by the row (or `not_applicable`).
    pub daily_loop_step_class: DailyLoopStepClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: LaunchToolingConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_support`, declares a non-`none_declared` known limit,
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

impl LaunchLanguageToolingRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchLanguageToolingConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Launch-tooling packet id consumed by the projection.
    pub launch_language_tooling_packet_id_ref: String,
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
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl LaunchLanguageToolingConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.launch_language_tooling_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
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

/// Constructor input for [`LaunchLanguageToolingTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchLanguageToolingTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Launch-tooling lane classes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ToolingLaneClass>,
    /// Launch-tooling rows.
    #[serde(default)]
    pub rows: Vec<LaunchLanguageToolingRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<LaunchLanguageToolingConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying shell/bash, SQL, Markdown,
/// JSON/YAML, and Git-oriented launch-tooling quality for the M4
/// stable lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchLanguageToolingTruthPacket {
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
    /// Launch-tooling lane classes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<ToolingLaneClass>,
    /// Launch-tooling rows.
    #[serde(default)]
    pub rows: Vec<LaunchLanguageToolingRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<LaunchLanguageToolingConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl LaunchLanguageToolingTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: LaunchLanguageToolingTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: LAUNCH_LANGUAGE_TOOLING_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_LANGUAGE_TOOLING_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable launch-tooling invariants.
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
        set.into_iter().map(ToolingLaneClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(LaunchToolingRowClass::as_str).collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(SupportClass::as_str).collect()
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
    ) -> LaunchLanguageToolingTruthSupportExport {
        LaunchLanguageToolingTruthSupportExport {
            record_kind: LAUNCH_LANGUAGE_TOOLING_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: LAUNCH_LANGUAGE_TOOLING_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            launch_language_tooling_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            launch_language_tooling_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != LAUNCH_LANGUAGE_TOOLING_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "launch-tooling packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != LAUNCH_LANGUAGE_TOOLING_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "launch-tooling packet has the wrong schema version",
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
                FindingKind::MissingToolingLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered launch-tooling lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingToolingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers launch-tooling lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
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

            if matches!(row.support_class, SupportClass::LaunchSupport)
                && !row.all_bindings_satisfied()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchSupportWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims launch_support while a binding (support, known limit, downgrade automation, or evidence) is unbound",
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
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
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

            if matches!(
                row.confidence_class,
                LaunchToolingConfidenceClass::LowConfidence
            ) && matches!(row.support_class, SupportClass::LaunchSupport)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchSupportWithUnboundBinding,
                    FindingSeverity::Warning,
                    format!(
                        "row {} claims launch_support at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
            }
        }

        for lane in &self.covered_lanes {
            let lane_claims_launch = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, LaunchToolingRowClass::LaunchToolingQuality)
                    && matches!(row.support_class, SupportClass::LaunchSupport)
            });
            if lane_claims_launch {
                for step in DailyLoopStepClass::REQUIRED_FOR_LAUNCH {
                    let covered = self.rows.iter().any(|row| {
                        row.lane_class == *lane
                            && matches!(row.row_class, LaunchToolingRowClass::DailyLoopStep)
                            && row.daily_loop_step_class == step
                    });
                    if !covered {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingDailyLoopStepCoverage,
                            FindingSeverity::Blocker,
                            format!(
                                "lane {} claims launch_support but has no daily_loop_step row for {}",
                                lane.as_str(),
                                step.as_str()
                            ),
                        ));
                    }
                }
            }
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
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve launch-tooling truth",
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
pub struct LaunchLanguageToolingTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub launch_language_tooling_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub launch_language_tooling_packet: LaunchLanguageToolingTruthPacket,
}

impl LaunchLanguageToolingTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == LAUNCH_LANGUAGE_TOOLING_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == LAUNCH_LANGUAGE_TOOLING_TRUTH_SCHEMA_VERSION
            && self.launch_language_tooling_packet_id_ref
                == self.launch_language_tooling_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.launch_language_tooling_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable launch-tooling packet.
#[derive(Debug)]
pub enum LaunchLanguageToolingTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for LaunchLanguageToolingTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "launch-tooling packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "launch-tooling packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for LaunchLanguageToolingTruthArtifactError {}

/// Returns the checked-in stable launch-tooling truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_launch_language_tooling_truth_packet(
) -> Result<LaunchLanguageToolingTruthPacket, LaunchLanguageToolingTruthArtifactError> {
    let packet: LaunchLanguageToolingTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m4/launch_language_tooling_truth_packet.json"
    )))
    .map_err(LaunchLanguageToolingTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(LaunchLanguageToolingTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lane_evidence_ref() -> String {
        LAUNCH_LANGUAGE_TOOLING_TRUTH_DOC_REF.to_owned()
    }

    fn quality_row(row_id: &str, lane: ToolingLaneClass) -> LaunchLanguageToolingRow {
        LaunchLanguageToolingRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class: LaunchToolingRowClass::LaunchToolingQuality,
            support_class: SupportClass::LaunchSupport,
            daily_loop_step_class: DailyLoopStepClass::NotApplicable,
            evidence_class: EvidenceClass::ArchetypeRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: LaunchToolingConfidenceClass::HighConfidence,
            evidence_refs: vec![lane_evidence_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_block_on_missing_evidence",
                LAUNCH_LANGUAGE_TOOLING_TRUTH_DOC_REF
            )),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn loop_step_row(
        row_id: &str,
        lane: ToolingLaneClass,
        step: DailyLoopStepClass,
    ) -> LaunchLanguageToolingRow {
        LaunchLanguageToolingRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class: LaunchToolingRowClass::DailyLoopStep,
            support_class: SupportClass::LaunchSupport,
            daily_loop_step_class: step,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
            confidence_class: LaunchToolingConfidenceClass::HighConfidence,
            evidence_refs: vec![lane_evidence_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_missing_fixture",
                LAUNCH_LANGUAGE_TOOLING_TRUTH_DOC_REF
            )),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> LaunchLanguageToolingConsumerProjection {
        LaunchLanguageToolingConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            launch_language_tooling_packet_id_ref: "packet:m4:launch_language_tooling".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_daily_loop_step_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: ToolingLaneClass, prefix: &str) -> Vec<LaunchLanguageToolingRow> {
        let mut out = vec![quality_row(&format!("row:{prefix}:quality"), lane)];
        for step in DailyLoopStepClass::REQUIRED_FOR_LAUNCH {
            out.push(loop_step_row(
                &format!("row:{prefix}:loop:{}", step.as_str()),
                lane,
                step,
            ));
        }
        out
    }

    fn sample_input() -> LaunchLanguageToolingTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(ToolingLaneClass::ShellBashLane, "shell"));
        rows.extend(lane_rows(ToolingLaneClass::SqlLane, "sql"));
        rows.extend(lane_rows(ToolingLaneClass::MarkdownLane, "md"));
        rows.extend(lane_rows(ToolingLaneClass::JsonYamlLane, "jy"));
        rows.extend(lane_rows(ToolingLaneClass::GitOrientedLane, "git"));
        LaunchLanguageToolingTruthPacketInput {
            packet_id: "packet:m4:launch_language_tooling".to_owned(),
            workflow_or_surface_id: "workflow.language.launch_language_tooling".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: ToolingLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![lane_evidence_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(ToolingLaneClass::ShellBashLane.as_str(), "shell_bash_lane");
        assert_eq!(ToolingLaneClass::SqlLane.as_str(), "sql_lane");
        assert_eq!(ToolingLaneClass::MarkdownLane.as_str(), "markdown_lane");
        assert_eq!(ToolingLaneClass::JsonYamlLane.as_str(), "json_yaml_lane");
        assert_eq!(
            ToolingLaneClass::GitOrientedLane.as_str(),
            "git_oriented_lane"
        );
        assert_eq!(
            LaunchToolingRowClass::LaunchToolingQuality.as_str(),
            "launch_tooling_quality"
        );
        assert_eq!(SupportClass::LaunchSupport.as_str(), "launch_support");
        assert_eq!(
            SupportClass::LaunchSupportBelow.as_str(),
            "launch_support_below"
        );
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(DailyLoopStepClass::Recover.as_str(), "recover");
        assert_eq!(EvidenceClass::EvidenceUnbound.as_str(), "evidence_unbound");
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            KnownLimitClass::LaunchToolingScopeOnly.as_str(),
            "launch_tooling_scope_only"
        );
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
            FindingKind::EvidenceClassVocabularyCollapsed.as_str(),
            "evidence_class_vocabulary_collapsed"
        );
        assert_eq!(
            FindingKind::LaunchSupportWithUnboundBinding.as_str(),
            "launch_support_with_unbound_binding"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = LaunchLanguageToolingTruthPacket::materialize(sample_input());
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
                "support:m4:launch_language_tooling",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_support_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = LaunchLanguageToolingTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LaunchSupportWithUnboundBinding
        }));
    }

    #[test]
    fn missing_daily_loop_step_for_launch_support_blocks() {
        let mut input = sample_input();
        input
            .rows
            .retain(|row| row.daily_loop_step_class != DailyLoopStepClass::Recover);
        let packet = LaunchLanguageToolingTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDailyLoopStepCoverage));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchSupportBelow;
        input.rows[0].disclosure_ref = None;
        let packet = LaunchLanguageToolingTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef
        }));
    }

    #[test]
    fn daily_loop_step_not_applicable_on_loop_row_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, LaunchToolingRowClass::DailyLoopStep)
                && row.lane_class == ToolingLaneClass::ShellBashLane
                && row.daily_loop_step_class == DailyLoopStepClass::Edit
            {
                row.daily_loop_step_class = DailyLoopStepClass::NotApplicable;
                break;
            }
        }
        let packet = LaunchLanguageToolingTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::DailyLoopStepNotApplicable));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input
            .consumer_projections
            .retain(|projection| projection.consumer_surface != ConsumerSurface::ConformanceDashboard);
        let packet = LaunchLanguageToolingTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_evidence_class_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_evidence_class_vocabulary = false;
            }
        }
        let packet = LaunchLanguageToolingTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::EvidenceClassVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = LaunchLanguageToolingTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }

    #[test]
    fn narrowed_below_launch_with_disclosure_keeps_stable() {
        let mut input = sample_input();
        // Markdown lane downgrades to below-launch with disclosure; remove its
        // loop-step rows since they are no longer required when no
        // launch_support quality row is present.
        input.rows.retain(|row| {
            !(row.lane_class == ToolingLaneClass::MarkdownLane
                && matches!(row.row_class, LaunchToolingRowClass::DailyLoopStep))
        });
        for row in &mut input.rows {
            if row.lane_class == ToolingLaneClass::MarkdownLane
                && matches!(row.row_class, LaunchToolingRowClass::LaunchToolingQuality)
            {
                row.support_class = SupportClass::LaunchSupportBelow;
                row.known_limit_class = KnownLimitClass::FrameworkSubsetOnly;
                row.disclosure_ref = Some(format!(
                    "{}#framework_subset_only",
                    LAUNCH_LANGUAGE_TOOLING_TRUTH_DOC_REF
                ));
            }
        }
        let packet = LaunchLanguageToolingTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::Stable);
        assert!(packet.validation_findings.is_empty());
    }
}
