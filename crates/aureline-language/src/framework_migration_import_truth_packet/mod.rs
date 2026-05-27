//! Framework-specific migration and import guidance truth packet for
//! the M4 stable lane.
//!
//! This module is the language-owned contract that pins how the
//! framework migration guidance, import guidance, and unsupported-gap
//! labeling lanes stay one boundary truth across the launch bundles.
//! The packet ships framework-specific migration and import guidance
//! with unsupported-gap labeling so the editor language pack, framework
//! pack panel, language settings/help, CLI/headless inspector, support
//! export, release proof index, Help/About proof card, and the
//! conformance dashboard all read one record. Surfaces MUST NOT mint
//! local copies or paraphrase migration posture; they read this packet
//! verbatim.
//!
//! Every row binds a closed `migration_lane_class`,
//! `framework_migration_row_class`, `support_class`,
//! `outcome_label_class`, `rollback_checkpoint_class`,
//! `diagnostic_preservation_class`, `launch_bundle_class`,
//! `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and
//! `framework_migration_confidence_class` plus an `evidence_refs`
//! array and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or
//! binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! imported artifact bodies, source bodies, dependency manifests,
//! secrets, ambient credentials, or any other private material past
//! the boundary. A row that claims `launch_stable` while leaving its
//! known limit, downgrade automation, evidence class, outcome label,
//! rollback checkpoint, or diagnostic preservation unbound is refused;
//! the validator narrows below launch-stable instead of inheriting an
//! adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`FrameworkMigrationImportTruthPacket`].
pub const FRAMEWORK_MIGRATION_IMPORT_TRUTH_PACKET_RECORD_KIND: &str =
    "framework_migration_import_truth_stable_packet";

/// Stable record-kind tag for [`FrameworkMigrationImportTruthSupportExport`].
pub const FRAMEWORK_MIGRATION_IMPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "framework_migration_import_truth_support_export";

/// Integer schema version for the framework-migration-import truth packet.
pub const FRAMEWORK_MIGRATION_IMPORT_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const FRAMEWORK_MIGRATION_IMPORT_TRUTH_SCHEMA_REF: &str =
    "schemas/language/framework_migration_import_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const FRAMEWORK_MIGRATION_IMPORT_TRUTH_DOC_REF: &str =
    "docs/languages/m4/finalize-framework-specific-migration-and-import-guidance-with.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const FRAMEWORK_MIGRATION_IMPORT_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m4/finalize-framework-specific-migration-and-import-guidance-with.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const FRAMEWORK_MIGRATION_IMPORT_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m4/framework_migration_import_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const FRAMEWORK_MIGRATION_IMPORT_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m4/framework_migration_import_truth_packet.json";

/// Closed migration-lane vocabulary. Every required lane MUST have at
/// least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationLaneClass {
    /// Framework-specific migration guidance lane (framework upgrade
    /// or replacement guidance for a launch bundle).
    FrameworkMigrationGuidanceLane,
    /// Import guidance lane (import-statement / module-resolution
    /// migration for a launch bundle).
    ImportGuidanceLane,
    /// Unsupported-gap labeling lane (precisely labeling features
    /// without a stable migration mapping).
    UnsupportedGapLabelingLane,
}

impl MigrationLaneClass {
    /// Every required migration lane, in declaration order.
    pub const REQUIRED: [Self; 3] = [
        Self::FrameworkMigrationGuidanceLane,
        Self::ImportGuidanceLane,
        Self::UnsupportedGapLabelingLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkMigrationGuidanceLane => "framework_migration_guidance_lane",
            Self::ImportGuidanceLane => "import_guidance_lane",
            Self::UnsupportedGapLabelingLane => "unsupported_gap_labeling_lane",
        }
    }
}

/// Closed framework-migration row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkMigrationRowClass {
    /// The lane's headline migration-guidance qualification row.
    MigrationGuidanceQuality,
    /// Outcome-label truth row binding exactly one outcome label
    /// (exact_match, translated_match, partial_match, shimmed_match,
    /// unsupported_gap).
    OutcomeLabelTruth,
    /// Rollback-checkpoint admission row binding a checkpoint class.
    RollbackCheckpointAdmission,
    /// Diagnostic-preservation admission row binding a diagnostic class.
    DiagnosticPreservationAdmission,
    /// Launch-bundle coverage row certifying one launch bundle touchpoint.
    LaunchBundleCoverage,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl FrameworkMigrationRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigrationGuidanceQuality => "migration_guidance_quality",
            Self::OutcomeLabelTruth => "outcome_label_truth",
            Self::RollbackCheckpointAdmission => "rollback_checkpoint_admission",
            Self::DiagnosticPreservationAdmission => "diagnostic_preservation_admission",
            Self::LaunchBundleCoverage => "launch_bundle_coverage",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }
}

/// Closed support-class vocabulary applied to a framework-migration row.
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

/// Closed outcome-label vocabulary. A lane that claims `launch_stable`
/// MUST cover every outcome label the spec requires from real imported
/// artifacts: `exact_match`, `translated_match`, `partial_match`,
/// `shimmed_match`, and `unsupported_gap`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeLabelClass {
    /// Exact match — the imported artifact maps to a stable target without translation.
    ExactMatch,
    /// Translated match — the imported artifact maps to a stable target via translation.
    TranslatedMatch,
    /// Partial match — only part of the imported artifact maps to a stable target.
    PartialMatch,
    /// Shimmed match — the imported artifact maps via a runtime shim or compatibility layer.
    ShimmedMatch,
    /// Unsupported gap — the imported artifact has no stable mapping; the gap is labeled.
    UnsupportedGap,
    /// Row is not an outcome-label truth row.
    NotApplicable,
    /// Row has no bound outcome label; this never qualifies stable for a row class
    /// that requires a binding.
    LabelUnbound,
}

impl OutcomeLabelClass {
    /// Every required outcome label a lane that claims `launch_stable`
    /// MUST cover, in declaration order.
    pub const REQUIRED_FOR_LAUNCH: [Self; 5] = [
        Self::ExactMatch,
        Self::TranslatedMatch,
        Self::PartialMatch,
        Self::ShimmedMatch,
        Self::UnsupportedGap,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactMatch => "exact_match",
            Self::TranslatedMatch => "translated_match",
            Self::PartialMatch => "partial_match",
            Self::ShimmedMatch => "shimmed_match",
            Self::UnsupportedGap => "unsupported_gap",
            Self::NotApplicable => "not_applicable",
            Self::LabelUnbound => "label_unbound",
        }
    }

    /// True when this outcome label is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LabelUnbound)
    }
}

/// Closed rollback-checkpoint vocabulary. A
/// `rollback_checkpoint_admission` row binds exactly one checkpoint
/// class. A lane that claims `launch_stable` MUST preserve a rollback
/// checkpoint when mapping fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackCheckpointClass {
    /// Rollback checkpoint is preserved across the mapping attempt.
    CheckpointPreserved,
    /// Rollback checkpoint is preserved together with mapping diagnostics.
    CheckpointWithDiagnostics,
    /// Rollback checkpoint creation is pending; lane is narrowed.
    CheckpointPending,
    /// Row is not a rollback-checkpoint admission row.
    NotApplicable,
    /// Row has no bound rollback-checkpoint class.
    CheckpointUnbound,
}

impl RollbackCheckpointClass {
    /// Every required rollback-checkpoint state a lane that claims
    /// `launch_stable` MUST cover, in declaration order.
    pub const REQUIRED_FOR_LAUNCH: [Self; 1] = [Self::CheckpointPreserved];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointPreserved => "checkpoint_preserved",
            Self::CheckpointWithDiagnostics => "checkpoint_with_diagnostics",
            Self::CheckpointPending => "checkpoint_pending",
            Self::NotApplicable => "not_applicable",
            Self::CheckpointUnbound => "checkpoint_unbound",
        }
    }

    /// True when this rollback-checkpoint class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::CheckpointUnbound)
    }
}

/// Closed diagnostic-preservation vocabulary. A
/// `diagnostic_preservation_admission` row binds exactly one
/// preservation class. A lane that claims `launch_stable` MUST
/// preserve diagnostics when mapping fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticPreservationClass {
    /// All mapping diagnostics are preserved.
    DiagnosticsPreserved,
    /// Some mapping diagnostics are preserved; partial coverage disclosed.
    DiagnosticsPartial,
    /// Mapping diagnostics are absent; lane is narrowed.
    DiagnosticsAbsent,
    /// Row is not a diagnostic-preservation admission row.
    NotApplicable,
    /// Row has no bound diagnostic-preservation class.
    DiagnosticUnbound,
}

impl DiagnosticPreservationClass {
    /// Every required diagnostic-preservation state a lane that claims
    /// `launch_stable` MUST cover, in declaration order.
    pub const REQUIRED_FOR_LAUNCH: [Self; 1] = [Self::DiagnosticsPreserved];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticsPreserved => "diagnostics_preserved",
            Self::DiagnosticsPartial => "diagnostics_partial",
            Self::DiagnosticsAbsent => "diagnostics_absent",
            Self::NotApplicable => "not_applicable",
            Self::DiagnosticUnbound => "diagnostic_unbound",
        }
    }

    /// True when this diagnostic-preservation class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::DiagnosticUnbound)
    }
}

/// Closed launch-bundle vocabulary. A `launch_bundle_coverage` row
/// binds exactly one launch bundle class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchBundleClass {
    /// Python launch bundle.
    PythonLaunchBundle,
    /// TypeScript / JavaScript launch bundle.
    TypescriptJavascriptLaunchBundle,
    /// Rust launch bundle.
    RustLaunchBundle,
    /// Go launch bundle.
    GoLaunchBundle,
    /// Java / Kotlin launch bundle.
    JavaKotlinLaunchBundle,
    /// C / C++ launch bundle.
    CCppLaunchBundle,
    /// Row is not a launch-bundle coverage row.
    NotApplicable,
}

impl LaunchBundleClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PythonLaunchBundle => "python_launch_bundle",
            Self::TypescriptJavascriptLaunchBundle => "typescript_javascript_launch_bundle",
            Self::RustLaunchBundle => "rust_launch_bundle",
            Self::GoLaunchBundle => "go_launch_bundle",
            Self::JavaKotlinLaunchBundle => "java_kotlin_launch_bundle",
            Self::CCppLaunchBundle => "c_cpp_launch_bundle",
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
    /// The row is backed by framework- or library-migration evidence.
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

/// Closed known-limit vocabulary attached to a framework-migration row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies a launch-bundle subset.
    BundleSubsetOnly,
    /// The row only certifies a framework / library subset.
    FrameworkSubsetOnly,
    /// The row only certifies an outcome-label subset.
    OutcomeLabelSubsetOnly,
    /// The row only certifies a checkpoint subset.
    CheckpointSubsetOnly,
    /// The row only certifies a diagnostic subset.
    DiagnosticSubsetOnly,
    /// The row only certifies an archetype subset (specific repos).
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
            Self::BundleSubsetOnly => "bundle_subset_only",
            Self::FrameworkSubsetOnly => "framework_subset_only",
            Self::OutcomeLabelSubsetOnly => "outcome_label_subset_only",
            Self::CheckpointSubsetOnly => "checkpoint_subset_only",
            Self::DiagnosticSubsetOnly => "diagnostic_subset_only",
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

/// Closed downgrade-automation vocabulary attached to a
/// framework-migration row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a certified fixture is missing or stale.
    AutoNarrowOnMissingFixture,
    /// Automatically narrow when a certified archetype repo is missing.
    AutoNarrowOnMissingArchetype,
    /// Automatically narrow when an unsupported gap is detected.
    AutoNarrowOnUnsupportedGap,
    /// Automatically narrow when rollback checkpoint creation fails.
    AutoNarrowOnCheckpointFailure,
    /// Automatically narrow when mapping diagnostics are lost.
    AutoNarrowOnDiagnosticLoss,
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
            Self::AutoNarrowOnUnsupportedGap => "auto_narrow_on_unsupported_gap",
            Self::AutoNarrowOnCheckpointFailure => "auto_narrow_on_checkpoint_failure",
            Self::AutoNarrowOnDiagnosticLoss => "auto_narrow_on_diagnostic_loss",
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

/// Closed confidence-class vocabulary for a framework-migration row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkMigrationConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl FrameworkMigrationConfidenceClass {
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

/// Closed validation-finding vocabulary for the framework-migration packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required migration lane has no row.
    MissingMigrationLaneCoverage,
    /// A lane claiming launch_stable is missing a required outcome label.
    MissingOutcomeLabelCoverage,
    /// A lane claiming launch_stable is missing a required rollback-checkpoint state.
    MissingRollbackCheckpointCoverage,
    /// A lane claiming launch_stable is missing a required diagnostic-preservation state.
    MissingDiagnosticPreservationCoverage,
    /// A lane claiming launch_stable is missing a launch-bundle coverage row.
    MissingLaunchBundleCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row has no bound outcome-label class on a row-class that requires it.
    MissingOutcomeLabelClass,
    /// A row has no bound rollback-checkpoint class on a row-class that requires it.
    MissingRollbackCheckpointClass,
    /// A row has no bound diagnostic-preservation class on a row-class that requires it.
    MissingDiagnosticPreservationClass,
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
    /// An outcome-label truth row drops its outcome-label binding.
    OutcomeLabelNotApplicable,
    /// A non-outcome-label row binds an outcome label.
    OutcomeLabelNotPermittedOnRowClass,
    /// A rollback-checkpoint admission row drops its checkpoint class.
    RollbackCheckpointNotApplicable,
    /// A non-rollback-checkpoint row binds a checkpoint class.
    RollbackCheckpointNotPermittedOnRowClass,
    /// A diagnostic-preservation admission row drops its diagnostic class.
    DiagnosticPreservationNotApplicable,
    /// A non-diagnostic-preservation row binds a diagnostic class.
    DiagnosticPreservationNotPermittedOnRowClass,
    /// A launch-bundle coverage row drops its launch-bundle binding.
    LaunchBundleNotApplicable,
    /// A non-launch-bundle-coverage row binds a launch-bundle class.
    LaunchBundleNotPermittedOnRowClass,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops framework-migration truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the outcome-label vocabulary.
    OutcomeLabelVocabularyCollapsed,
    /// A projection collapses the rollback-checkpoint vocabulary.
    RollbackCheckpointVocabularyCollapsed,
    /// A projection collapses the diagnostic-preservation vocabulary.
    DiagnosticPreservationVocabularyCollapsed,
    /// A projection collapses the launch-bundle vocabulary.
    LaunchBundleVocabularyCollapsed,
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
            Self::MissingMigrationLaneCoverage => "missing_migration_lane_coverage",
            Self::MissingOutcomeLabelCoverage => "missing_outcome_label_coverage",
            Self::MissingRollbackCheckpointCoverage => "missing_rollback_checkpoint_coverage",
            Self::MissingDiagnosticPreservationCoverage => {
                "missing_diagnostic_preservation_coverage"
            }
            Self::MissingLaunchBundleCoverage => "missing_launch_bundle_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingOutcomeLabelClass => "missing_outcome_label_class",
            Self::MissingRollbackCheckpointClass => "missing_rollback_checkpoint_class",
            Self::MissingDiagnosticPreservationClass => "missing_diagnostic_preservation_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::OutcomeLabelNotApplicable => "outcome_label_not_applicable",
            Self::OutcomeLabelNotPermittedOnRowClass => "outcome_label_not_permitted_on_row_class",
            Self::RollbackCheckpointNotApplicable => "rollback_checkpoint_not_applicable",
            Self::RollbackCheckpointNotPermittedOnRowClass => {
                "rollback_checkpoint_not_permitted_on_row_class"
            }
            Self::DiagnosticPreservationNotApplicable => "diagnostic_preservation_not_applicable",
            Self::DiagnosticPreservationNotPermittedOnRowClass => {
                "diagnostic_preservation_not_permitted_on_row_class"
            }
            Self::LaunchBundleNotApplicable => "launch_bundle_not_applicable",
            Self::LaunchBundleNotPermittedOnRowClass => "launch_bundle_not_permitted_on_row_class",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::OutcomeLabelVocabularyCollapsed => "outcome_label_vocabulary_collapsed",
            Self::RollbackCheckpointVocabularyCollapsed => {
                "rollback_checkpoint_vocabulary_collapsed"
            }
            Self::DiagnosticPreservationVocabularyCollapsed => {
                "diagnostic_preservation_vocabulary_collapsed"
            }
            Self::LaunchBundleVocabularyCollapsed => "launch_bundle_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the framework-migration packet verbatim.
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

/// One framework-migration row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkMigrationImportRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Migration lane this row certifies.
    pub lane_class: MigrationLaneClass,
    /// Row class.
    pub row_class: FrameworkMigrationRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Outcome-label class (or `not_applicable`).
    pub outcome_label_class: OutcomeLabelClass,
    /// Rollback-checkpoint class (or `not_applicable`).
    pub rollback_checkpoint_class: RollbackCheckpointClass,
    /// Diagnostic-preservation class (or `not_applicable`).
    pub diagnostic_preservation_class: DiagnosticPreservationClass,
    /// Launch-bundle class (or `not_applicable`).
    pub launch_bundle_class: LaunchBundleClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: FrameworkMigrationConfidenceClass,
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

impl FrameworkMigrationImportRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.outcome_label_class.is_bound()
            && self.rollback_checkpoint_class.is_bound()
            && self.diagnostic_preservation_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkMigrationImportConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Framework-migration packet id consumed by the projection.
    pub framework_migration_import_packet_id_ref: String,
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
    /// True when the outcome-label vocabulary is preserved verbatim.
    pub preserves_outcome_label_vocabulary: bool,
    /// True when the rollback-checkpoint vocabulary is preserved verbatim.
    pub preserves_rollback_checkpoint_vocabulary: bool,
    /// True when the diagnostic-preservation vocabulary is preserved verbatim.
    pub preserves_diagnostic_preservation_vocabulary: bool,
    /// True when the launch-bundle vocabulary is preserved verbatim.
    pub preserves_launch_bundle_vocabulary: bool,
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

impl FrameworkMigrationImportConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.framework_migration_import_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_outcome_label_vocabulary
            && self.preserves_rollback_checkpoint_vocabulary
            && self.preserves_diagnostic_preservation_vocabulary
            && self.preserves_launch_bundle_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`FrameworkMigrationImportTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkMigrationImportTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Migration lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<MigrationLaneClass>,
    /// Framework-migration rows.
    #[serde(default)]
    pub rows: Vec<FrameworkMigrationImportRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<FrameworkMigrationImportConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying framework-specific migration and
/// import guidance with unsupported-gap labeling for the launch bundles
/// at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkMigrationImportTruthPacket {
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
    /// Migration lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<MigrationLaneClass>,
    /// Framework-migration rows.
    #[serde(default)]
    pub rows: Vec<FrameworkMigrationImportRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<FrameworkMigrationImportConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl FrameworkMigrationImportTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: FrameworkMigrationImportTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: FRAMEWORK_MIGRATION_IMPORT_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_MIGRATION_IMPORT_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable framework-migration invariants.
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
        set.into_iter().map(MigrationLaneClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(FrameworkMigrationRowClass::as_str)
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

    /// Returns the unique outcome-label tokens observed across rows.
    pub fn outcome_label_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.outcome_label_class);
        }
        set.into_iter().map(OutcomeLabelClass::as_str).collect()
    }

    /// Returns the unique rollback-checkpoint tokens observed across rows.
    pub fn rollback_checkpoint_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.rollback_checkpoint_class);
        }
        set.into_iter()
            .map(RollbackCheckpointClass::as_str)
            .collect()
    }

    /// Returns the unique diagnostic-preservation tokens observed across rows.
    pub fn diagnostic_preservation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.diagnostic_preservation_class);
        }
        set.into_iter()
            .map(DiagnosticPreservationClass::as_str)
            .collect()
    }

    /// Returns the unique launch-bundle tokens observed across rows.
    pub fn launch_bundle_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.launch_bundle_class);
        }
        set.into_iter().map(LaunchBundleClass::as_str).collect()
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
    ) -> FrameworkMigrationImportTruthSupportExport {
        FrameworkMigrationImportTruthSupportExport {
            record_kind: FRAMEWORK_MIGRATION_IMPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_MIGRATION_IMPORT_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            framework_migration_import_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            framework_migration_import_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != FRAMEWORK_MIGRATION_IMPORT_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "framework-migration packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != FRAMEWORK_MIGRATION_IMPORT_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "framework-migration packet has the wrong schema version",
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
                FindingKind::MissingMigrationLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered migration lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingMigrationLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers migration lane {}", lane.as_str()),
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
        row: &FrameworkMigrationImportRow,
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
                format!(
                    "row {} has no bound downgrade-automation class",
                    row.row_id
                ),
            ));
        }
        if !row.evidence_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound evidence class", row.row_id),
            ));
        }
        if !row.outcome_label_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingOutcomeLabelClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound outcome-label class", row.row_id),
            ));
        }
        if !row.rollback_checkpoint_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRollbackCheckpointClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has no bound rollback-checkpoint class",
                    row.row_id
                ),
            ));
        }
        if !row.diagnostic_preservation_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingDiagnosticPreservationClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has no bound diagnostic-preservation class",
                    row.row_id
                ),
            ));
        }

        if matches!(row.support_class, SupportClass::LaunchStable) && !row.all_bindings_satisfied()
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchStableWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims launch_stable while a binding (support, known limit, downgrade automation, evidence, outcome label, rollback checkpoint, or diagnostic preservation) is unbound",
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

        let is_outcome_label = matches!(row.row_class, FrameworkMigrationRowClass::OutcomeLabelTruth);
        let is_rollback = matches!(
            row.row_class,
            FrameworkMigrationRowClass::RollbackCheckpointAdmission
        );
        let is_diagnostic = matches!(
            row.row_class,
            FrameworkMigrationRowClass::DiagnosticPreservationAdmission
        );
        let is_launch_bundle = matches!(
            row.row_class,
            FrameworkMigrationRowClass::LaunchBundleCoverage
        );

        if is_outcome_label && matches!(row.outcome_label_class, OutcomeLabelClass::NotApplicable) {
            findings.push(ValidationFinding::new(
                FindingKind::OutcomeLabelNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is an outcome_label_truth but has no bound outcome label",
                    row.row_id
                ),
            ));
        }
        if !is_outcome_label
            && !matches!(
                row.outcome_label_class,
                OutcomeLabelClass::NotApplicable | OutcomeLabelClass::LabelUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::OutcomeLabelNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds outcome label {}; only outcome_label_truth rows may bind an outcome label",
                    row.row_id,
                    row.row_class.as_str(),
                    row.outcome_label_class.as_str()
                ),
            ));
        }

        if is_rollback
            && matches!(
                row.rollback_checkpoint_class,
                RollbackCheckpointClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackCheckpointNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a rollback_checkpoint_admission but has no bound checkpoint class",
                    row.row_id
                ),
            ));
        }
        if !is_rollback
            && !matches!(
                row.rollback_checkpoint_class,
                RollbackCheckpointClass::NotApplicable
                    | RollbackCheckpointClass::CheckpointUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackCheckpointNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds rollback checkpoint {}; only rollback_checkpoint_admission rows may bind a checkpoint class",
                    row.row_id,
                    row.row_class.as_str(),
                    row.rollback_checkpoint_class.as_str()
                ),
            ));
        }

        if is_diagnostic
            && matches!(
                row.diagnostic_preservation_class,
                DiagnosticPreservationClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::DiagnosticPreservationNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a diagnostic_preservation_admission but has no bound diagnostic class",
                    row.row_id
                ),
            ));
        }
        if !is_diagnostic
            && !matches!(
                row.diagnostic_preservation_class,
                DiagnosticPreservationClass::NotApplicable
                    | DiagnosticPreservationClass::DiagnosticUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::DiagnosticPreservationNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds diagnostic preservation {}; only diagnostic_preservation_admission rows may bind a diagnostic class",
                    row.row_id,
                    row.row_class.as_str(),
                    row.diagnostic_preservation_class.as_str()
                ),
            ));
        }

        if is_launch_bundle && matches!(row.launch_bundle_class, LaunchBundleClass::NotApplicable) {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchBundleNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a launch_bundle_coverage but has no bound launch bundle",
                    row.row_id
                ),
            ));
        }
        if !is_launch_bundle && !matches!(row.launch_bundle_class, LaunchBundleClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchBundleNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds launch bundle {}; only launch_bundle_coverage rows may bind a launch bundle",
                    row.row_id,
                    row.row_class.as_str(),
                    row.launch_bundle_class.as_str()
                ),
            ));
        }

        if matches!(
            row.confidence_class,
            FrameworkMigrationConfidenceClass::LowConfidence
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
        lane: MigrationLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_stable = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    FrameworkMigrationRowClass::MigrationGuidanceQuality
                )
                && matches!(row.support_class, SupportClass::LaunchStable)
        });
        if !lane_claims_stable {
            return;
        }

        for label in OutcomeLabelClass::REQUIRED_FOR_LAUNCH {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(row.row_class, FrameworkMigrationRowClass::OutcomeLabelTruth)
                    && row.outcome_label_class == label
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingOutcomeLabelCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no outcome_label_truth row for {}",
                        lane.as_str(),
                        label.as_str()
                    ),
                ));
            }
        }

        for checkpoint in RollbackCheckpointClass::REQUIRED_FOR_LAUNCH {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        FrameworkMigrationRowClass::RollbackCheckpointAdmission
                    )
                    && row.rollback_checkpoint_class == checkpoint
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRollbackCheckpointCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no rollback_checkpoint_admission row for {}",
                        lane.as_str(),
                        checkpoint.as_str()
                    ),
                ));
            }
        }

        for diagnostic in DiagnosticPreservationClass::REQUIRED_FOR_LAUNCH {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        FrameworkMigrationRowClass::DiagnosticPreservationAdmission
                    )
                    && row.diagnostic_preservation_class == diagnostic
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDiagnosticPreservationCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no diagnostic_preservation_admission row for {}",
                        lane.as_str(),
                        diagnostic.as_str()
                    ),
                ));
            }
        }

        let bundle_covered = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    FrameworkMigrationRowClass::LaunchBundleCoverage
                )
        });
        if !bundle_covered {
            findings.push(ValidationFinding::new(
                FindingKind::MissingLaunchBundleCoverage,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no launch_bundle_coverage row",
                    lane.as_str()
                ),
            ));
        }
    }

    fn append_projection_findings(
        &self,
        projection: &FrameworkMigrationImportConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve framework-migration truth",
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
        if !projection.preserves_outcome_label_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::OutcomeLabelVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the outcome-label vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_rollback_checkpoint_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackCheckpointVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the rollback-checkpoint vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_diagnostic_preservation_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::DiagnosticPreservationVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the diagnostic-preservation vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_launch_bundle_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchBundleVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the launch-bundle vocabulary",
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
pub struct FrameworkMigrationImportTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub framework_migration_import_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub framework_migration_import_packet: FrameworkMigrationImportTruthPacket,
}

impl FrameworkMigrationImportTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == FRAMEWORK_MIGRATION_IMPORT_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == FRAMEWORK_MIGRATION_IMPORT_TRUTH_SCHEMA_VERSION
            && self.framework_migration_import_packet_id_ref
                == self.framework_migration_import_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.framework_migration_import_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable framework-migration packet.
#[derive(Debug)]
pub enum FrameworkMigrationImportTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for FrameworkMigrationImportTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "framework-migration packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "framework-migration packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for FrameworkMigrationImportTruthArtifactError {}

/// Returns the checked-in stable framework-migration truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_framework_migration_import_truth_packet(
) -> Result<FrameworkMigrationImportTruthPacket, FrameworkMigrationImportTruthArtifactError> {
    let packet: FrameworkMigrationImportTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m4/framework_migration_import_truth_packet.json"
    )))
    .map_err(FrameworkMigrationImportTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(FrameworkMigrationImportTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        FRAMEWORK_MIGRATION_IMPORT_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        FRAMEWORK_MIGRATION_IMPORT_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn base_row(
        row_id: &str,
        lane: MigrationLaneClass,
        row_class: FrameworkMigrationRowClass,
    ) -> FrameworkMigrationImportRow {
        FrameworkMigrationImportRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class,
            support_class: SupportClass::LaunchStable,
            outcome_label_class: OutcomeLabelClass::NotApplicable,
            rollback_checkpoint_class: RollbackCheckpointClass::NotApplicable,
            diagnostic_preservation_class: DiagnosticPreservationClass::NotApplicable,
            launch_bundle_class: LaunchBundleClass::NotApplicable,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
            confidence_class: FrameworkMigrationConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn quality_row(lane: MigrationLaneClass, prefix: &str) -> FrameworkMigrationImportRow {
        let mut row = base_row(
            &format!("row:{prefix}:quality"),
            lane,
            FrameworkMigrationRowClass::MigrationGuidanceQuality,
        );
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
        row.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
        row.evidence_refs = vec![doc_ref(), fixture_ref()];
        row
    }

    fn outcome_rows(lane: MigrationLaneClass, prefix: &str) -> Vec<FrameworkMigrationImportRow> {
        OutcomeLabelClass::REQUIRED_FOR_LAUNCH
            .into_iter()
            .map(|label| {
                let mut row = base_row(
                    &format!("row:{prefix}:outcome:{}", label.as_str()),
                    lane,
                    FrameworkMigrationRowClass::OutcomeLabelTruth,
                );
                row.outcome_label_class = label;
                row.evidence_class = EvidenceClass::FrameworkMigrationEvidence;
                row
            })
            .collect()
    }

    fn rollback_row(lane: MigrationLaneClass, prefix: &str) -> FrameworkMigrationImportRow {
        let mut row = base_row(
            &format!("row:{prefix}:rollback:checkpoint_preserved"),
            lane,
            FrameworkMigrationRowClass::RollbackCheckpointAdmission,
        );
        row.rollback_checkpoint_class = RollbackCheckpointClass::CheckpointPreserved;
        row.evidence_class = EvidenceClass::FixtureRepoEvidence;
        row
    }

    fn diagnostic_row(lane: MigrationLaneClass, prefix: &str) -> FrameworkMigrationImportRow {
        let mut row = base_row(
            &format!("row:{prefix}:diagnostic:diagnostics_preserved"),
            lane,
            FrameworkMigrationRowClass::DiagnosticPreservationAdmission,
        );
        row.diagnostic_preservation_class = DiagnosticPreservationClass::DiagnosticsPreserved;
        row.evidence_class = EvidenceClass::FixtureRepoEvidence;
        row
    }

    fn bundle_row(
        lane: MigrationLaneClass,
        prefix: &str,
        bundle: LaunchBundleClass,
    ) -> FrameworkMigrationImportRow {
        let mut row = base_row(
            &format!("row:{prefix}:bundle:{}", bundle.as_str()),
            lane,
            FrameworkMigrationRowClass::LaunchBundleCoverage,
        );
        row.launch_bundle_class = bundle;
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row
    }

    fn projection(surface: ConsumerSurface) -> FrameworkMigrationImportConsumerProjection {
        FrameworkMigrationImportConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            framework_migration_import_packet_id_ref:
                "packet:m4:framework_migration_import".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_outcome_label_vocabulary: true,
            preserves_rollback_checkpoint_vocabulary: true,
            preserves_diagnostic_preservation_vocabulary: true,
            preserves_launch_bundle_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(
        lane: MigrationLaneClass,
        prefix: &str,
        bundle: LaunchBundleClass,
    ) -> Vec<FrameworkMigrationImportRow> {
        let mut rows = vec![quality_row(lane, prefix)];
        rows.extend(outcome_rows(lane, prefix));
        rows.push(rollback_row(lane, prefix));
        rows.push(diagnostic_row(lane, prefix));
        rows.push(bundle_row(lane, prefix, bundle));
        rows
    }

    fn sample_input() -> FrameworkMigrationImportTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(
            MigrationLaneClass::FrameworkMigrationGuidanceLane,
            "framework",
            LaunchBundleClass::PythonLaunchBundle,
        ));
        rows.extend(lane_rows(
            MigrationLaneClass::ImportGuidanceLane,
            "import",
            LaunchBundleClass::TypescriptJavascriptLaunchBundle,
        ));
        rows.extend(lane_rows(
            MigrationLaneClass::UnsupportedGapLabelingLane,
            "gap",
            LaunchBundleClass::RustLaunchBundle,
        ));
        let mut projections = Vec::new();
        for surface in ConsumerSurface::REQUIRED {
            projections.push(projection(surface));
        }
        FrameworkMigrationImportTruthPacketInput {
            packet_id: "packet:m4:framework_migration_import".to_owned(),
            workflow_or_surface_id: "workflow.language.framework_migration_import".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: MigrationLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: projections,
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            MigrationLaneClass::FrameworkMigrationGuidanceLane.as_str(),
            "framework_migration_guidance_lane"
        );
        assert_eq!(
            MigrationLaneClass::ImportGuidanceLane.as_str(),
            "import_guidance_lane"
        );
        assert_eq!(
            MigrationLaneClass::UnsupportedGapLabelingLane.as_str(),
            "unsupported_gap_labeling_lane"
        );
        assert_eq!(
            FrameworkMigrationRowClass::MigrationGuidanceQuality.as_str(),
            "migration_guidance_quality"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(OutcomeLabelClass::ExactMatch.as_str(), "exact_match");
        assert_eq!(
            OutcomeLabelClass::TranslatedMatch.as_str(),
            "translated_match"
        );
        assert_eq!(OutcomeLabelClass::PartialMatch.as_str(), "partial_match");
        assert_eq!(OutcomeLabelClass::ShimmedMatch.as_str(), "shimmed_match");
        assert_eq!(
            OutcomeLabelClass::UnsupportedGap.as_str(),
            "unsupported_gap"
        );
        assert_eq!(OutcomeLabelClass::LabelUnbound.as_str(), "label_unbound");
        assert_eq!(
            RollbackCheckpointClass::CheckpointPreserved.as_str(),
            "checkpoint_preserved"
        );
        assert_eq!(
            RollbackCheckpointClass::CheckpointUnbound.as_str(),
            "checkpoint_unbound"
        );
        assert_eq!(
            DiagnosticPreservationClass::DiagnosticsPreserved.as_str(),
            "diagnostics_preserved"
        );
        assert_eq!(
            DiagnosticPreservationClass::DiagnosticUnbound.as_str(),
            "diagnostic_unbound"
        );
        assert_eq!(
            LaunchBundleClass::PythonLaunchBundle.as_str(),
            "python_launch_bundle"
        );
        assert_eq!(
            LaunchBundleClass::CCppLaunchBundle.as_str(),
            "c_cpp_launch_bundle"
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
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            FindingKind::MissingOutcomeLabelCoverage.as_str(),
            "missing_outcome_label_coverage"
        );
        assert_eq!(
            FindingKind::OutcomeLabelVocabularyCollapsed.as_str(),
            "outcome_label_vocabulary_collapsed"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = FrameworkMigrationImportTruthPacket::materialize(sample_input());
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
                "support:m4:framework_migration_import",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = FrameworkMigrationImportTruthPacket::materialize(input);
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
    fn missing_outcome_label_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(row.lane_class == MigrationLaneClass::FrameworkMigrationGuidanceLane
                && matches!(row.row_class, FrameworkMigrationRowClass::OutcomeLabelTruth)
                && row.outcome_label_class == OutcomeLabelClass::UnsupportedGap)
        });
        let packet = FrameworkMigrationImportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingOutcomeLabelCoverage));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = FrameworkMigrationImportTruthPacket::materialize(input);
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
        let packet = FrameworkMigrationImportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_outcome_label_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_outcome_label_vocabulary = false;
            }
        }
        let packet = FrameworkMigrationImportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::OutcomeLabelVocabularyCollapsed));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = FrameworkMigrationImportTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
