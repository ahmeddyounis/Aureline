//! Refactor transaction truth packet for the M4 stable lane.
//!
//! This module is the language-owned contract that pins how the
//! claimed launch-language refactor classes (rename, extract function,
//! inline symbol, move symbol, update imports, and cross-file
//! signature change) stay one boundary truth across the preview,
//! validate, apply, and rollback transaction phases. The packet
//! finalizes the refactor transaction model and the preview, validate,
//! and rollback corpus so the editor language pack, framework pack
//! panel, language settings/help, CLI/headless inspector, support
//! export, release proof index, Help/About proof card, and the
//! conformance dashboard all read one record. Surfaces MUST NOT mint
//! local copies or paraphrase refactor transaction posture; they read
//! this packet verbatim.
//!
//! Every row binds a closed `refactor_class_lane_class`,
//! `refactor_transaction_row_class`, `support_class`,
//! `transaction_phase_class`, `preview_completeness_class`,
//! `validation_outcome_class`, `rollback_path_class`,
//! `launch_language_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and
//! `refactor_transaction_confidence_class` plus an `evidence_refs`
//! array and a `disclosure_ref` whenever the row is narrowed below
//! launch-stable, declares a non-`none_declared` known limit, or binds
//! a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! source bodies, refactor diffs, generated artifact bodies, secrets,
//! ambient credentials, or any other private material past the
//! boundary. A row that claims `launch_stable` while leaving its
//! known limit, downgrade automation, evidence class, preview
//! completeness, validation outcome, or rollback path unbound is
//! refused; the validator narrows below launch-stable instead of
//! inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`RefactorTransactionTruthPacket`].
pub const REFACTOR_TRANSACTION_TRUTH_PACKET_RECORD_KIND: &str =
    "refactor_transaction_truth_stable_packet";

/// Stable record-kind tag for [`RefactorTransactionTruthSupportExport`].
pub const REFACTOR_TRANSACTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "refactor_transaction_truth_support_export";

/// Integer schema version for the refactor transaction truth packet.
pub const REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REFACTOR_TRANSACTION_TRUTH_SCHEMA_REF: &str =
    "schemas/language/refactor_transaction_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const REFACTOR_TRANSACTION_TRUTH_DOC_REF: &str =
    "docs/languages/m4/finalize-the-refactor-transaction-model-plus-preview-validate.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const REFACTOR_TRANSACTION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m4/finalize-the-refactor-transaction-model-plus-preview-validate.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m4/refactor_transaction_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const REFACTOR_TRANSACTION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m4/refactor_transaction_truth_packet.json";

/// Closed refactor-class lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorClassLaneClass {
    /// Symbol rename refactor lane.
    RenameSymbolLane,
    /// Extract function refactor lane.
    ExtractFunctionLane,
    /// Inline symbol refactor lane.
    InlineSymbolLane,
    /// Move symbol refactor lane.
    MoveSymbolLane,
    /// Update imports refactor lane.
    UpdateImportsLane,
    /// Cross-file signature change refactor lane.
    CrossFileSignatureChangeLane,
}

impl RefactorClassLaneClass {
    /// Every required refactor-class lane, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::RenameSymbolLane,
        Self::ExtractFunctionLane,
        Self::InlineSymbolLane,
        Self::MoveSymbolLane,
        Self::UpdateImportsLane,
        Self::CrossFileSignatureChangeLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenameSymbolLane => "rename_symbol_lane",
            Self::ExtractFunctionLane => "extract_function_lane",
            Self::InlineSymbolLane => "inline_symbol_lane",
            Self::MoveSymbolLane => "move_symbol_lane",
            Self::UpdateImportsLane => "update_imports_lane",
            Self::CrossFileSignatureChangeLane => "cross_file_signature_change_lane",
        }
    }
}

/// Closed refactor-transaction row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorTransactionRowClass {
    /// The lane's headline qualification row.
    RefactorTransactionQuality,
    /// Transaction-phase truth row binding exactly one phase
    /// (preview, validate, apply, or rollback).
    TransactionPhaseTruth,
    /// Preview-outcome admission row binding a preview completeness class.
    PreviewOutcomeAdmission,
    /// Validation-hook admission row binding a validation outcome class.
    ValidationHookAdmission,
    /// Rollback-drill admission row binding a rollback path class.
    RollbackDrillAdmission,
    /// Launch-language coverage row certifying one launch language touchpoint.
    LaunchLanguageCoverage,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl RefactorTransactionRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefactorTransactionQuality => "refactor_transaction_quality",
            Self::TransactionPhaseTruth => "transaction_phase_truth",
            Self::PreviewOutcomeAdmission => "preview_outcome_admission",
            Self::ValidationHookAdmission => "validation_hook_admission",
            Self::RollbackDrillAdmission => "rollback_drill_admission",
            Self::LaunchLanguageCoverage => "launch_language_coverage",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }
}

/// Closed support-class vocabulary applied to a refactor-transaction row.
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

/// Closed transaction-phase vocabulary. A lane that claims
/// `launch_stable` MUST cover every transaction phase required by the
/// transaction model: preview, validate, apply, and rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionPhaseClass {
    /// Preview phase — the refactor surfaces its target set and diff before apply.
    Preview,
    /// Validate phase — the refactor admits a validation result (typecheck,
    /// diagnostics rerun, tests, dependency graph, generated outputs).
    Validate,
    /// Apply phase — the refactor commits the grouped mutation lineage.
    Apply,
    /// Rollback phase — the refactor exposes a revert route after apply.
    Rollback,
    /// Row is not a transaction-phase row.
    NotApplicable,
}

impl TransactionPhaseClass {
    /// Every required transaction phase, in declaration order.
    pub const REQUIRED_FOR_LAUNCH: [Self; 4] =
        [Self::Preview, Self::Validate, Self::Apply, Self::Rollback];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Validate => "validate",
            Self::Apply => "apply",
            Self::Rollback => "rollback",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed preview-completeness vocabulary. A `preview_outcome_admission`
/// row binds exactly one completeness class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewCompletenessClass {
    /// Preview captures the complete target set with current semantic evidence.
    Complete,
    /// Preview captures a partial target set with a visible label.
    Partial,
    /// Preview is blocked pending refresh, scope, or provider health.
    Blocked,
    /// Preview is unsupported on this row.
    Unsupported,
    /// Row is not a preview-outcome admission row.
    NotApplicable,
    /// Row has no bound preview-completeness class; this never qualifies stable
    /// for a row class that requires a binding.
    PreviewUnbound,
}

impl PreviewCompletenessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Blocked => "blocked",
            Self::Unsupported => "unsupported",
            Self::NotApplicable => "not_applicable",
            Self::PreviewUnbound => "preview_unbound",
        }
    }

    /// True when this preview-completeness class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::PreviewUnbound)
    }
}

/// Closed validation-outcome vocabulary. A `validation_hook_admission`
/// row binds exactly one outcome class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationOutcomeClass {
    /// Validation passed without warnings.
    Passed,
    /// Validation passed with caveats.
    PassedWithWarnings,
    /// Validation ran and failed.
    Failed,
    /// Validation is blocked by missing prerequisites.
    Blocked,
    /// Validation has not run yet.
    Pending,
    /// Validation is unsupported on this row.
    Unsupported,
    /// Row is not a validation-hook admission row.
    NotApplicable,
    /// Row has no bound validation-outcome class; this never qualifies stable
    /// for a row class that requires a binding.
    OutcomeUnbound,
}

impl ValidationOutcomeClass {
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

    /// True when this validation-outcome class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::OutcomeUnbound)
    }
}

/// Closed rollback-path vocabulary. A `rollback_drill_admission` row
/// binds exactly one rollback path class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPathClass {
    /// Exact undo is available through a local-history checkpoint.
    ExactUndoViaLocalHistoryCheckpoint,
    /// Revert is available as a compensating workspace diff.
    CompensatingRevertViaWorkspaceDiff,
    /// Grouped mutation-journal entry owns the revert.
    GroupedMutationJournalRevert,
    /// Generated artifacts must regenerate before replay.
    RegenerateFirstThenReplay,
    /// Manual review is required before an automatic route can be claimed.
    ManualReviewRequiredNoAutomaticPath,
    /// No safe automatic rollback exists.
    NoSafeRollbackAvailable,
    /// Row is not a rollback-drill admission row.
    NotApplicable,
    /// Row has no bound rollback-path class; this never qualifies stable
    /// for a row class that requires a binding.
    RollbackUnbound,
}

impl RollbackPathClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndoViaLocalHistoryCheckpoint => "exact_undo_via_local_history_checkpoint",
            Self::CompensatingRevertViaWorkspaceDiff => "compensating_revert_via_workspace_diff",
            Self::GroupedMutationJournalRevert => "grouped_mutation_journal_revert",
            Self::RegenerateFirstThenReplay => "regenerate_first_then_replay",
            Self::ManualReviewRequiredNoAutomaticPath => "manual_review_required_no_automatic_path",
            Self::NoSafeRollbackAvailable => "no_safe_rollback_available",
            Self::NotApplicable => "not_applicable",
            Self::RollbackUnbound => "rollback_unbound",
        }
    }

    /// True when this rollback-path class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::RollbackUnbound)
    }
}

/// Closed launch-language vocabulary. A `launch_language_coverage` row
/// binds exactly one launch language class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchLanguageClass {
    /// Python launch language.
    Python,
    /// TypeScript / JavaScript launch language.
    TypescriptJavascript,
    /// Rust launch language.
    Rust,
    /// Go launch language.
    Go,
    /// Java / Kotlin launch languages.
    JavaKotlin,
    /// C / C++ launch languages.
    CCpp,
    /// Row is not a launch-language coverage row.
    NotApplicable,
}

impl LaunchLanguageClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::TypescriptJavascript => "typescript_javascript",
            Self::Rust => "rust",
            Self::Go => "go",
            Self::JavaKotlin => "java_kotlin",
            Self::CCpp => "c_cpp",
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

/// Closed known-limit vocabulary attached to a refactor-transaction row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies a refactor-class subset.
    RefactorSubsetOnly,
    /// The row only certifies a launch-language subset.
    LanguageSubsetOnly,
    /// The row only certifies an archetype subset.
    ArchetypeSubsetOnly,
    /// The row only certifies a transaction-phase subset.
    TransactionPhaseSubsetOnly,
    /// The row only certifies a rollback-path subset.
    RollbackPathSubsetOnly,
    /// The row only certifies a preview-completeness subset.
    PreviewSubsetOnly,
    /// The row only certifies a validation-outcome subset.
    ValidationSubsetOnly,
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
            Self::RefactorSubsetOnly => "refactor_subset_only",
            Self::LanguageSubsetOnly => "language_subset_only",
            Self::ArchetypeSubsetOnly => "archetype_subset_only",
            Self::TransactionPhaseSubsetOnly => "transaction_phase_subset_only",
            Self::RollbackPathSubsetOnly => "rollback_path_subset_only",
            Self::PreviewSubsetOnly => "preview_subset_only",
            Self::ValidationSubsetOnly => "validation_subset_only",
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

/// Closed downgrade-automation vocabulary attached to a refactor-transaction row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a certified fixture is missing or stale.
    AutoNarrowOnMissingFixture,
    /// Automatically narrow when a certified archetype repo is missing.
    AutoNarrowOnMissingArchetype,
    /// Automatically narrow when validation fails on the row.
    AutoNarrowOnValidationFailure,
    /// Automatically narrow when a rollback drill fails on the row.
    AutoNarrowOnRollbackDrillFailure,
    /// Automatically narrow when preview drops below complete coverage.
    AutoNarrowOnPreviewPartial,
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
            Self::AutoNarrowOnValidationFailure => "auto_narrow_on_validation_failure",
            Self::AutoNarrowOnRollbackDrillFailure => "auto_narrow_on_rollback_drill_failure",
            Self::AutoNarrowOnPreviewPartial => "auto_narrow_on_preview_partial",
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

/// Closed confidence-class vocabulary for a refactor-transaction row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefactorTransactionConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl RefactorTransactionConfidenceClass {
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

/// Closed validation-finding vocabulary for the refactor-transaction packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required refactor-class lane has no row.
    MissingRefactorClassLaneCoverage,
    /// A lane claiming launch_stable is missing a required transaction phase.
    MissingTransactionPhaseCoverage,
    /// A lane claiming launch_stable is missing a preview-outcome admission row.
    MissingPreviewOutcomeCoverage,
    /// A lane claiming launch_stable is missing a validation-hook admission row.
    MissingValidationHookCoverage,
    /// A lane claiming launch_stable is missing a rollback-drill admission row.
    MissingRollbackDrillCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row has no bound preview-completeness class on a row-class that requires it.
    MissingPreviewCompletenessClass,
    /// A row has no bound validation-outcome class on a row-class that requires it.
    MissingValidationOutcomeClass,
    /// A row has no bound rollback-path class on a row-class that requires it.
    MissingRollbackPathClass,
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
    /// A transaction-phase truth row drops its phase binding.
    TransactionPhaseNotApplicable,
    /// A non-transaction-phase row binds a transaction phase.
    TransactionPhaseNotPermittedOnRowClass,
    /// A preview-outcome admission row drops its preview-completeness class.
    PreviewCompletenessNotApplicable,
    /// A non-preview-outcome row binds a preview-completeness class.
    PreviewCompletenessNotPermittedOnRowClass,
    /// A validation-hook admission row drops its validation-outcome class.
    ValidationOutcomeNotApplicable,
    /// A non-validation-hook row binds a validation-outcome class.
    ValidationOutcomeNotPermittedOnRowClass,
    /// A rollback-drill admission row drops its rollback-path class.
    RollbackPathNotApplicable,
    /// A non-rollback-drill row binds a rollback-path class.
    RollbackPathNotPermittedOnRowClass,
    /// A launch-language coverage row drops its launch-language binding.
    LaunchLanguageNotApplicable,
    /// A non-launch-language-coverage row binds a launch-language class.
    LaunchLanguageNotPermittedOnRowClass,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops refactor-transaction truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the transaction-phase vocabulary.
    TransactionPhaseVocabularyCollapsed,
    /// A projection collapses the preview-completeness vocabulary.
    PreviewCompletenessVocabularyCollapsed,
    /// A projection collapses the validation-outcome vocabulary.
    ValidationOutcomeVocabularyCollapsed,
    /// A projection collapses the rollback-path vocabulary.
    RollbackPathVocabularyCollapsed,
    /// A projection collapses the launch-language vocabulary.
    LaunchLanguageVocabularyCollapsed,
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
            Self::MissingRefactorClassLaneCoverage => "missing_refactor_class_lane_coverage",
            Self::MissingTransactionPhaseCoverage => "missing_transaction_phase_coverage",
            Self::MissingPreviewOutcomeCoverage => "missing_preview_outcome_coverage",
            Self::MissingValidationHookCoverage => "missing_validation_hook_coverage",
            Self::MissingRollbackDrillCoverage => "missing_rollback_drill_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingPreviewCompletenessClass => "missing_preview_completeness_class",
            Self::MissingValidationOutcomeClass => "missing_validation_outcome_class",
            Self::MissingRollbackPathClass => "missing_rollback_path_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::TransactionPhaseNotApplicable => "transaction_phase_not_applicable",
            Self::TransactionPhaseNotPermittedOnRowClass => {
                "transaction_phase_not_permitted_on_row_class"
            }
            Self::PreviewCompletenessNotApplicable => "preview_completeness_not_applicable",
            Self::PreviewCompletenessNotPermittedOnRowClass => {
                "preview_completeness_not_permitted_on_row_class"
            }
            Self::ValidationOutcomeNotApplicable => "validation_outcome_not_applicable",
            Self::ValidationOutcomeNotPermittedOnRowClass => {
                "validation_outcome_not_permitted_on_row_class"
            }
            Self::RollbackPathNotApplicable => "rollback_path_not_applicable",
            Self::RollbackPathNotPermittedOnRowClass => "rollback_path_not_permitted_on_row_class",
            Self::LaunchLanguageNotApplicable => "launch_language_not_applicable",
            Self::LaunchLanguageNotPermittedOnRowClass => {
                "launch_language_not_permitted_on_row_class"
            }
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::TransactionPhaseVocabularyCollapsed => "transaction_phase_vocabulary_collapsed",
            Self::PreviewCompletenessVocabularyCollapsed => {
                "preview_completeness_vocabulary_collapsed"
            }
            Self::ValidationOutcomeVocabularyCollapsed => "validation_outcome_vocabulary_collapsed",
            Self::RollbackPathVocabularyCollapsed => "rollback_path_vocabulary_collapsed",
            Self::LaunchLanguageVocabularyCollapsed => "launch_language_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the refactor-transaction packet verbatim.
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

/// One refactor-transaction row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorTransactionRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Refactor-class lane this row certifies.
    pub lane_class: RefactorClassLaneClass,
    /// Row class.
    pub row_class: RefactorTransactionRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Transaction phase the row binds (or `not_applicable`).
    pub transaction_phase_class: TransactionPhaseClass,
    /// Preview completeness class (or `not_applicable`).
    pub preview_completeness_class: PreviewCompletenessClass,
    /// Validation outcome class (or `not_applicable`).
    pub validation_outcome_class: ValidationOutcomeClass,
    /// Rollback path class (or `not_applicable`).
    pub rollback_path_class: RollbackPathClass,
    /// Launch language class (or `not_applicable`).
    pub launch_language_class: LaunchLanguageClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: RefactorTransactionConfidenceClass,
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

impl RefactorTransactionRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.preview_completeness_class.is_bound()
            && self.validation_outcome_class.is_bound()
            && self.rollback_path_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorTransactionConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Refactor-transaction packet id consumed by the projection.
    pub refactor_transaction_packet_id_ref: String,
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
    /// True when the transaction-phase vocabulary is preserved verbatim.
    pub preserves_transaction_phase_vocabulary: bool,
    /// True when the preview-completeness vocabulary is preserved verbatim.
    pub preserves_preview_completeness_vocabulary: bool,
    /// True when the validation-outcome vocabulary is preserved verbatim.
    pub preserves_validation_outcome_vocabulary: bool,
    /// True when the rollback-path vocabulary is preserved verbatim.
    pub preserves_rollback_path_vocabulary: bool,
    /// True when the launch-language vocabulary is preserved verbatim.
    pub preserves_launch_language_vocabulary: bool,
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

impl RefactorTransactionConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.refactor_transaction_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_transaction_phase_vocabulary
            && self.preserves_preview_completeness_vocabulary
            && self.preserves_validation_outcome_vocabulary
            && self.preserves_rollback_path_vocabulary
            && self.preserves_launch_language_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`RefactorTransactionTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorTransactionTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Refactor-class lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<RefactorClassLaneClass>,
    /// Refactor-transaction rows.
    #[serde(default)]
    pub rows: Vec<RefactorTransactionRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<RefactorTransactionConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying the refactor transaction model and
/// the preview/validate/rollback corpus across the claimed
/// launch-language refactor classes at the M4 launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorTransactionTruthPacket {
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
    /// Refactor-class lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<RefactorClassLaneClass>,
    /// Refactor-transaction rows.
    #[serde(default)]
    pub rows: Vec<RefactorTransactionRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<RefactorTransactionConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl RefactorTransactionTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: RefactorTransactionTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: REFACTOR_TRANSACTION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable refactor-transaction invariants.
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
            .map(RefactorClassLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(RefactorTransactionRowClass::as_str)
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

    /// Returns the unique transaction-phase tokens observed across rows.
    pub fn transaction_phase_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.transaction_phase_class);
        }
        set.into_iter().map(TransactionPhaseClass::as_str).collect()
    }

    /// Returns the unique preview-completeness tokens observed across rows.
    pub fn preview_completeness_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.preview_completeness_class);
        }
        set.into_iter()
            .map(PreviewCompletenessClass::as_str)
            .collect()
    }

    /// Returns the unique validation-outcome tokens observed across rows.
    pub fn validation_outcome_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.validation_outcome_class);
        }
        set.into_iter()
            .map(ValidationOutcomeClass::as_str)
            .collect()
    }

    /// Returns the unique rollback-path tokens observed across rows.
    pub fn rollback_path_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.rollback_path_class);
        }
        set.into_iter().map(RollbackPathClass::as_str).collect()
    }

    /// Returns the unique launch-language tokens observed across rows.
    pub fn launch_language_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.launch_language_class);
        }
        set.into_iter().map(LaunchLanguageClass::as_str).collect()
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
    ) -> RefactorTransactionTruthSupportExport {
        RefactorTransactionTruthSupportExport {
            record_kind: REFACTOR_TRANSACTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            refactor_transaction_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            refactor_transaction_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != REFACTOR_TRANSACTION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "refactor-transaction packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "refactor-transaction packet has the wrong schema version",
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
                FindingKind::MissingRefactorClassLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered refactor-class lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRefactorClassLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers refactor-class lane {}", lane.as_str()),
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
        row: &RefactorTransactionRow,
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
        if !row.preview_completeness_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingPreviewCompletenessClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound preview-completeness class", row.row_id),
            ));
        }
        if !row.validation_outcome_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingValidationOutcomeClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound validation-outcome class", row.row_id),
            ));
        }
        if !row.rollback_path_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRollbackPathClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound rollback-path class", row.row_id),
            ));
        }

        if matches!(row.support_class, SupportClass::LaunchStable) && !row.all_bindings_satisfied()
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchStableWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims launch_stable while a binding (support, known limit, downgrade automation, evidence, preview-completeness, validation-outcome, or rollback-path) is unbound",
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

        // Row-class / field-binding rules.
        let is_transaction_phase = matches!(
            row.row_class,
            RefactorTransactionRowClass::TransactionPhaseTruth
        );
        let is_preview_outcome = matches!(
            row.row_class,
            RefactorTransactionRowClass::PreviewOutcomeAdmission
        );
        let is_validation_hook = matches!(
            row.row_class,
            RefactorTransactionRowClass::ValidationHookAdmission
        );
        let is_rollback_drill = matches!(
            row.row_class,
            RefactorTransactionRowClass::RollbackDrillAdmission
        );
        let is_launch_language = matches!(
            row.row_class,
            RefactorTransactionRowClass::LaunchLanguageCoverage
        );

        if is_transaction_phase
            && matches!(
                row.transaction_phase_class,
                TransactionPhaseClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::TransactionPhaseNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a transaction_phase_truth but has no bound transaction phase",
                    row.row_id
                ),
            ));
        }
        if !is_transaction_phase
            && !matches!(
                row.transaction_phase_class,
                TransactionPhaseClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::TransactionPhaseNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds transaction phase {}; only transaction_phase_truth rows may bind a phase",
                    row.row_id,
                    row.row_class.as_str(),
                    row.transaction_phase_class.as_str()
                ),
            ));
        }

        if is_preview_outcome
            && matches!(
                row.preview_completeness_class,
                PreviewCompletenessClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::PreviewCompletenessNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a preview_outcome_admission but has no bound preview completeness class",
                    row.row_id
                ),
            ));
        }
        if !is_preview_outcome
            && !matches!(
                row.preview_completeness_class,
                PreviewCompletenessClass::NotApplicable | PreviewCompletenessClass::PreviewUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::PreviewCompletenessNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds preview completeness {}; only preview_outcome_admission rows may bind preview completeness",
                    row.row_id,
                    row.row_class.as_str(),
                    row.preview_completeness_class.as_str()
                ),
            ));
        }

        if is_validation_hook
            && matches!(
                row.validation_outcome_class,
                ValidationOutcomeClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::ValidationOutcomeNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a validation_hook_admission but has no bound validation outcome class",
                    row.row_id
                ),
            ));
        }
        if !is_validation_hook
            && !matches!(
                row.validation_outcome_class,
                ValidationOutcomeClass::NotApplicable | ValidationOutcomeClass::OutcomeUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::ValidationOutcomeNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds validation outcome {}; only validation_hook_admission rows may bind a validation outcome",
                    row.row_id,
                    row.row_class.as_str(),
                    row.validation_outcome_class.as_str()
                ),
            ));
        }

        if is_rollback_drill && matches!(row.rollback_path_class, RollbackPathClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackPathNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a rollback_drill_admission but has no bound rollback path class",
                    row.row_id
                ),
            ));
        }
        if !is_rollback_drill
            && !matches!(
                row.rollback_path_class,
                RollbackPathClass::NotApplicable | RollbackPathClass::RollbackUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackPathNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds rollback path {}; only rollback_drill_admission rows may bind a rollback path",
                    row.row_id,
                    row.row_class.as_str(),
                    row.rollback_path_class.as_str()
                ),
            ));
        }

        if is_launch_language
            && matches!(
                row.launch_language_class,
                LaunchLanguageClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchLanguageNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a launch_language_coverage but has no bound launch language",
                    row.row_id
                ),
            ));
        }
        if !is_launch_language
            && !matches!(
                row.launch_language_class,
                LaunchLanguageClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchLanguageNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds launch language {}; only launch_language_coverage rows may bind a launch language",
                    row.row_id,
                    row.row_class.as_str(),
                    row.launch_language_class.as_str()
                ),
            ));
        }

        if matches!(
            row.confidence_class,
            RefactorTransactionConfidenceClass::LowConfidence
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
        lane: RefactorClassLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_stable = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    RefactorTransactionRowClass::RefactorTransactionQuality
                )
                && matches!(row.support_class, SupportClass::LaunchStable)
        });
        if !lane_claims_stable {
            return;
        }

        for phase in TransactionPhaseClass::REQUIRED_FOR_LAUNCH {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(
                        row.row_class,
                        RefactorTransactionRowClass::TransactionPhaseTruth
                    )
                    && row.transaction_phase_class == phase
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingTransactionPhaseCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no transaction_phase_truth row for {}",
                        lane.as_str(),
                        phase.as_str()
                    ),
                ));
            }
        }

        let preview_covered = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    RefactorTransactionRowClass::PreviewOutcomeAdmission
                )
        });
        if !preview_covered {
            findings.push(ValidationFinding::new(
                FindingKind::MissingPreviewOutcomeCoverage,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no preview_outcome_admission row",
                    lane.as_str()
                ),
            ));
        }

        let validation_covered = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    RefactorTransactionRowClass::ValidationHookAdmission
                )
        });
        if !validation_covered {
            findings.push(ValidationFinding::new(
                FindingKind::MissingValidationHookCoverage,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no validation_hook_admission row",
                    lane.as_str()
                ),
            ));
        }

        let rollback_covered = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(
                    row.row_class,
                    RefactorTransactionRowClass::RollbackDrillAdmission
                )
        });
        if !rollback_covered {
            findings.push(ValidationFinding::new(
                FindingKind::MissingRollbackDrillCoverage,
                FindingSeverity::Blocker,
                format!(
                    "lane {} claims launch_stable but has no rollback_drill_admission row",
                    lane.as_str()
                ),
            ));
        }
    }

    fn append_projection_findings(
        &self,
        projection: &RefactorTransactionConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve refactor-transaction truth",
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
        if !projection.preserves_transaction_phase_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::TransactionPhaseVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the transaction-phase vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_preview_completeness_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::PreviewCompletenessVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the preview-completeness vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_validation_outcome_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::ValidationOutcomeVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the validation-outcome vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_rollback_path_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::RollbackPathVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the rollback-path vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_launch_language_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchLanguageVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the launch-language vocabulary",
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
pub struct RefactorTransactionTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub refactor_transaction_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub refactor_transaction_packet: RefactorTransactionTruthPacket,
}

impl RefactorTransactionTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == REFACTOR_TRANSACTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == REFACTOR_TRANSACTION_TRUTH_SCHEMA_VERSION
            && self.refactor_transaction_packet_id_ref == self.refactor_transaction_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.refactor_transaction_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable refactor-transaction packet.
#[derive(Debug)]
pub enum RefactorTransactionTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for RefactorTransactionTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "refactor-transaction packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "refactor-transaction packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RefactorTransactionTruthArtifactError {}

/// Returns the checked-in stable refactor-transaction truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_refactor_transaction_truth_packet(
) -> Result<RefactorTransactionTruthPacket, RefactorTransactionTruthArtifactError> {
    let packet: RefactorTransactionTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m4/refactor_transaction_truth_packet.json"
    )))
    .map_err(RefactorTransactionTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(RefactorTransactionTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        REFACTOR_TRANSACTION_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        REFACTOR_TRANSACTION_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn base_row(
        row_id: &str,
        lane: RefactorClassLaneClass,
        row_class: RefactorTransactionRowClass,
    ) -> RefactorTransactionRow {
        RefactorTransactionRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class,
            support_class: SupportClass::LaunchStable,
            transaction_phase_class: TransactionPhaseClass::NotApplicable,
            preview_completeness_class: PreviewCompletenessClass::NotApplicable,
            validation_outcome_class: ValidationOutcomeClass::NotApplicable,
            rollback_path_class: RollbackPathClass::NotApplicable,
            launch_language_class: LaunchLanguageClass::NotApplicable,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
            confidence_class: RefactorTransactionConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn quality_row(lane: RefactorClassLaneClass, prefix: &str) -> RefactorTransactionRow {
        let mut row = base_row(
            &format!("row:{prefix}:quality"),
            lane,
            RefactorTransactionRowClass::RefactorTransactionQuality,
        );
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
        row.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
        row.evidence_refs = vec![doc_ref(), fixture_ref()];
        row
    }

    fn phase_rows(lane: RefactorClassLaneClass, prefix: &str) -> Vec<RefactorTransactionRow> {
        TransactionPhaseClass::REQUIRED_FOR_LAUNCH
            .into_iter()
            .map(|phase| {
                let mut row = base_row(
                    &format!("row:{prefix}:phase:{}", phase.as_str()),
                    lane,
                    RefactorTransactionRowClass::TransactionPhaseTruth,
                );
                row.transaction_phase_class = phase;
                row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
                row
            })
            .collect()
    }

    fn preview_row(lane: RefactorClassLaneClass, prefix: &str) -> RefactorTransactionRow {
        let mut row = base_row(
            &format!("row:{prefix}:preview_outcome"),
            lane,
            RefactorTransactionRowClass::PreviewOutcomeAdmission,
        );
        row.preview_completeness_class = PreviewCompletenessClass::Complete;
        row.evidence_class = EvidenceClass::FixtureRepoEvidence;
        row
    }

    fn validation_row(lane: RefactorClassLaneClass, prefix: &str) -> RefactorTransactionRow {
        let mut row = base_row(
            &format!("row:{prefix}:validation_hook"),
            lane,
            RefactorTransactionRowClass::ValidationHookAdmission,
        );
        row.validation_outcome_class = ValidationOutcomeClass::Passed;
        row.evidence_class = EvidenceClass::FixtureRepoEvidence;
        row
    }

    fn rollback_row(lane: RefactorClassLaneClass, prefix: &str) -> RefactorTransactionRow {
        let mut row = base_row(
            &format!("row:{prefix}:rollback_drill"),
            lane,
            RefactorTransactionRowClass::RollbackDrillAdmission,
        );
        row.rollback_path_class = RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint;
        row.evidence_class = EvidenceClass::FixtureRepoEvidence;
        row
    }

    fn launch_language_row(
        lane: RefactorClassLaneClass,
        prefix: &str,
        language: LaunchLanguageClass,
    ) -> RefactorTransactionRow {
        let mut row = base_row(
            &format!("row:{prefix}:launch_language:{}", language.as_str()),
            lane,
            RefactorTransactionRowClass::LaunchLanguageCoverage,
        );
        row.launch_language_class = language;
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row
    }

    fn projection(surface: ConsumerSurface) -> RefactorTransactionConsumerProjection {
        RefactorTransactionConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            refactor_transaction_packet_id_ref: "packet:m4:refactor_transaction".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_transaction_phase_vocabulary: true,
            preserves_preview_completeness_vocabulary: true,
            preserves_validation_outcome_vocabulary: true,
            preserves_rollback_path_vocabulary: true,
            preserves_launch_language_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: RefactorClassLaneClass, prefix: &str) -> Vec<RefactorTransactionRow> {
        let mut rows = vec![quality_row(lane, prefix)];
        rows.extend(phase_rows(lane, prefix));
        rows.push(preview_row(lane, prefix));
        rows.push(validation_row(lane, prefix));
        rows.push(rollback_row(lane, prefix));
        rows.push(launch_language_row(
            lane,
            prefix,
            LaunchLanguageClass::Python,
        ));
        rows
    }

    fn sample_input() -> RefactorTransactionTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(
            RefactorClassLaneClass::RenameSymbolLane,
            "rename",
        ));
        rows.extend(lane_rows(
            RefactorClassLaneClass::ExtractFunctionLane,
            "extract",
        ));
        rows.extend(lane_rows(
            RefactorClassLaneClass::InlineSymbolLane,
            "inline",
        ));
        rows.extend(lane_rows(RefactorClassLaneClass::MoveSymbolLane, "move"));
        rows.extend(lane_rows(
            RefactorClassLaneClass::UpdateImportsLane,
            "imports",
        ));
        rows.extend(lane_rows(
            RefactorClassLaneClass::CrossFileSignatureChangeLane,
            "signature",
        ));
        let mut projections = Vec::new();
        for surface in ConsumerSurface::REQUIRED {
            projections.push(projection(surface));
        }
        RefactorTransactionTruthPacketInput {
            packet_id: "packet:m4:refactor_transaction".to_owned(),
            workflow_or_surface_id: "workflow.language.refactor_transaction".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: RefactorClassLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: projections,
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            RefactorClassLaneClass::RenameSymbolLane.as_str(),
            "rename_symbol_lane"
        );
        assert_eq!(
            RefactorClassLaneClass::CrossFileSignatureChangeLane.as_str(),
            "cross_file_signature_change_lane"
        );
        assert_eq!(
            RefactorTransactionRowClass::RefactorTransactionQuality.as_str(),
            "refactor_transaction_quality"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(TransactionPhaseClass::Preview.as_str(), "preview");
        assert_eq!(TransactionPhaseClass::Rollback.as_str(), "rollback");
        assert_eq!(PreviewCompletenessClass::Complete.as_str(), "complete");
        assert_eq!(
            PreviewCompletenessClass::PreviewUnbound.as_str(),
            "preview_unbound"
        );
        assert_eq!(ValidationOutcomeClass::Passed.as_str(), "passed");
        assert_eq!(
            ValidationOutcomeClass::OutcomeUnbound.as_str(),
            "outcome_unbound"
        );
        assert_eq!(
            RollbackPathClass::ExactUndoViaLocalHistoryCheckpoint.as_str(),
            "exact_undo_via_local_history_checkpoint"
        );
        assert_eq!(
            RollbackPathClass::RollbackUnbound.as_str(),
            "rollback_unbound"
        );
        assert_eq!(LaunchLanguageClass::Python.as_str(), "python");
        assert_eq!(LaunchLanguageClass::JavaKotlin.as_str(), "java_kotlin");
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
            FindingKind::MissingTransactionPhaseCoverage.as_str(),
            "missing_transaction_phase_coverage"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = RefactorTransactionTruthPacket::materialize(sample_input());
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
            .support_export("support:m4:refactor_transaction", "2026-05-26T12:00:10Z")
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = RefactorTransactionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LaunchStableWithUnboundBinding
        }));
    }

    #[test]
    fn missing_transaction_phase_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(row.lane_class == RefactorClassLaneClass::RenameSymbolLane
                && matches!(
                    row.row_class,
                    RefactorTransactionRowClass::TransactionPhaseTruth
                )
                && row.transaction_phase_class == TransactionPhaseClass::Rollback)
        });
        let packet = RefactorTransactionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingTransactionPhaseCoverage
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = RefactorTransactionTruthPacket::materialize(input);
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
        let packet = RefactorTransactionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_rollback_path_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_rollback_path_vocabulary = false;
            }
        }
        let packet = RefactorTransactionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::RollbackPathVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = RefactorTransactionTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
