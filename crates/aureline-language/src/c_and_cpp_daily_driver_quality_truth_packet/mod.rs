//! Stable C and C++ daily-driver quality truth packet for the M4
//! stable lane.
//!
//! This module is the language-owned contract that pins how C and
//! C++ replacement-grade daily-driver quality stays one boundary
//! truth across the editor language pack, framework pack panel,
//! language settings/help, CLI/headless inspector, support export,
//! release proof index, Help/About proof card, and the conformance
//! dashboard. Every row binds a closed `language_lane_class`,
//! `daily_driver_row_class`, `support_class`, `daily_loop_step_class`,
//! `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `daily_driver_confidence_class`
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the
//! row is narrowed below replacement grade, declares a non-`none_declared`
//! known limit, or binds a non-`none` downgrade automation.
//!
//! The packet pins C and C++ daily-driver quality across three
//! intertwined truths beyond the bare daily loop:
//!
//! 1. The **CMake / Ninja build workspace truth** — every row that
//!    crosses the C / C++ build boundary (`CMakeLists.txt` top-level
//!    and subdirectory targets, `CMakePresets.json` /
//!    `CMakeUserPresets.json` configure / build / test / package
//!    presets, `CMakeCache.txt`, `cmake/` modules and toolchains,
//!    Ninja `build.ninja` and Ninja Multi-Config generators,
//!    `CMAKE_BUILD_TYPE` (Debug / Release / RelWithDebInfo /
//!    MinSizeRel), `CMAKE_TOOLCHAIN_FILE`, vcpkg `vcpkg.json` /
//!    `vcpkg-configuration.json` and Conan `conanfile.txt` /
//!    `conanfile.py` package-manager integration, and
//!    `compile_commands.json` export
//!    (`CMAKE_EXPORT_COMPILE_COMMANDS=ON`)) binds a dedicated
//!    `build_workspace_row` and a disclosure ref so the daily-driver
//!    row never confuses one workspace layout for another.
//! 2. The **compile / run / debug fidelity parity** — every row that
//!    certifies the run/test/debug step on C or C++ archetype repos
//!    binds a dedicated `compile_run_debug_row`
//!    (`cmake --build` driving Ninja or Make targets, `ctest` /
//!    `ctest --output-on-failure` test invocation, executable launch
//!    via `cmake --build --target <target>` or `ninja <target>`;
//!    clang / gcc / MSVC column- and range-accurate compile
//!    diagnostics; SARIF import from `clang-tidy` and `cppcheck`;
//!    LLDB and GDB native debuggers; `lldb-dap` and CodeLLDB DAP
//!    bridges; launch and attach modes; `gdbserver` /
//!    `lldb-server` remote-debug transports; core-dump open) so a
//!    beta-grade capability sample cannot masquerade as a
//!    replacement-grade C / C++ run-debug daily-driver surface.
//! 3. The **clangd rename / navigation parity** — every row that
//!    certifies symbol navigation, completion, or refactor on C or
//!    C++ archetype repos binds a dedicated
//!    `clangd_navigation_row` (clangd LSP `textDocument/rename` /
//!    `textDocument/definition` / `textDocument/references` /
//!    `textDocument/typeDefinition` / extract / inline /
//!    code-action; type-hierarchy and call-hierarchy; header /
//!    source pairing (`.h` / `.c`, `.hpp` / `.cc` / `.cpp` /
//!    `.cxx`); include resolution from `compile_commands.json` `-I`
//!    paths; cross-translation-unit background index;
//!    `clang-format` / `.clang-format` formatting and
//!    `clang-tidy` / `.clang-tidy` quick-fixes) so large-workspace
//!    rename and navigation posture is never inferred from a tiny
//!    single-translation-unit sample.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! source bodies, secrets, ambient credentials, or provider payloads.
//! A row that claims `replacement_grade` while leaving its known limit,
//! downgrade automation, or evidence class unbound is refused; the
//! validator narrows below replacement grade instead of inheriting an
//! adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`CAndCppDailyDriverQualityTruthPacket`].
pub const C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_PACKET_RECORD_KIND: &str =
    "c_and_cpp_daily_driver_quality_truth_stable_packet";

/// Stable record-kind tag for [`CAndCppDailyDriverQualityTruthSupportExport`].
pub const C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "c_and_cpp_daily_driver_quality_truth_support_export";

/// Integer schema version for the C and C++ daily-driver quality packet.
pub const C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_REF: &str =
    "schemas/language/c_and_cpp_daily_driver_quality_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_DOC_REF: &str =
    "docs/languages/m4/stabilize-c-and-cpp-daily-driver-quality-with.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m4/stabilize-c-and-cpp-daily-driver-quality-with.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m4/c_and_cpp_daily_driver_quality_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m4/c_and_cpp_daily_driver_quality_truth_packet.json";

/// Closed language-lane vocabulary. Every required lane MUST have at
/// least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageLaneClass {
    /// C and C++ replacement-grade daily-driver lane.
    CAndCppDailyDriverLane,
}

impl LanguageLaneClass {
    /// Every required language lane, in declaration order.
    pub const REQUIRED: [Self; 1] = [Self::CAndCppDailyDriverLane];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CAndCppDailyDriverLane => "c_and_cpp_daily_driver_lane",
        }
    }
}

/// Closed daily-driver row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyDriverRowClass {
    /// The lane's headline daily-driver qualification row.
    DailyDriverQuality,
    /// One step of the certified daily loop on a lane.
    DailyLoopStep,
    /// Framework pack evidence row on a lane.
    FrameworkPack,
    /// Migration evidence row on a lane.
    MigrationEvidence,
    /// Archetype repo / fixture-repo evidence row on a lane.
    ArchetypeRepoEvidence,
    /// CMake / Ninja build-workspace evidence row.
    BuildWorkspaceRow,
    /// Compile / run / debug fidelity evidence row.
    CompileRunDebugRow,
    /// clangd LSP rename / navigation evidence row.
    ClangdNavigationRow,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl DailyDriverRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DailyDriverQuality => "daily_driver_quality",
            Self::DailyLoopStep => "daily_loop_step",
            Self::FrameworkPack => "framework_pack",
            Self::MigrationEvidence => "migration_evidence",
            Self::ArchetypeRepoEvidence => "archetype_repo_evidence",
            Self::BuildWorkspaceRow => "build_workspace_row",
            Self::CompileRunDebugRow => "compile_run_debug_row",
            Self::ClangdNavigationRow => "clangd_navigation_row",
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

/// Closed support-class vocabulary applied to a row. A row is never
/// `replacement_grade` while its known limit, downgrade automation, or
/// evidence class is unbound; the validator demotes it instead of
/// inheriting an adjacent replacement-grade row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims replacement-grade daily-driver support.
    ReplacementGrade,
    /// Row is intentionally narrowed below replacement grade; the narrowing is disclosed.
    DailyDriverBelowReplacement,
    /// Row is at beta-grade only (capability sample, not daily driver).
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
            Self::ReplacementGrade => "replacement_grade",
            Self::DailyDriverBelowReplacement => "daily_driver_below_replacement",
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
        !matches!(self, Self::ReplacementGrade)
    }
}

/// Closed daily-loop step vocabulary. The full daily loop MUST be
/// covered for each lane that claims `replacement_grade`.
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
    /// `replacement_grade` lane MUST cover every step.
    pub const REQUIRED_FOR_REPLACEMENT: [Self; 9] = [
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
    /// The row is backed by framework-migration evidence.
    FrameworkMigrationEvidence,
    /// The row is backed by design-partner evidence.
    DesignPartnerEvidence,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a conformance suite run.
    ConformanceSuiteEvidence,
    /// The row is backed by a benchmark / fitness function capture.
    BenchmarkEvidence,
    /// The row is backed by a CMake / Ninja build-workspace capture.
    BuildWorkspaceEvidence,
    /// The row is backed by a compile / run / debug fidelity capture.
    CompileRunDebugEvidence,
    /// The row is backed by a clangd rename and navigation capture.
    ClangdNavigationEvidence,
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
            Self::BuildWorkspaceEvidence => "build_workspace_evidence",
            Self::CompileRunDebugEvidence => "compile_run_debug_evidence",
            Self::ClangdNavigationEvidence => "clangd_navigation_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this evidence class satisfies the evidence-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary attached to a daily-driver row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies a framework subset.
    FrameworkSubsetOnly,
    /// The row only certifies a language subset (e.g., C without C++).
    LanguageSubsetOnly,
    /// The row only certifies an archetype subset (specific repos).
    ArchetypeSubsetOnly,
    /// The row only certifies a migration subset.
    MigrationSubsetOnly,
    /// The row only certifies a subset of the CMake / Ninja build-workspace surface.
    BuildWorkspaceSubsetOnly,
    /// The row only certifies a subset of the compile / run / debug surface.
    CompileRunDebugSubsetOnly,
    /// The row only certifies a subset of the clangd rename / navigation surface.
    ClangdNavigationSubsetOnly,
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
            Self::FrameworkSubsetOnly => "framework_subset_only",
            Self::LanguageSubsetOnly => "language_subset_only",
            Self::ArchetypeSubsetOnly => "archetype_subset_only",
            Self::MigrationSubsetOnly => "migration_subset_only",
            Self::BuildWorkspaceSubsetOnly => "build_workspace_subset_only",
            Self::CompileRunDebugSubsetOnly => "compile_run_debug_subset_only",
            Self::ClangdNavigationSubsetOnly => "clangd_navigation_subset_only",
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

/// Closed downgrade-automation vocabulary attached to a daily-driver row.
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
    /// Automatically narrow when the CMake / Ninja build-workspace surface is unproven.
    AutoNarrowOnUnprovenBuildWorkspace,
    /// Automatically narrow when the compile / run / debug surface is unproven.
    AutoNarrowOnUnprovenCompileRunDebug,
    /// Automatically narrow when the clangd rename / navigation surface is unproven.
    AutoNarrowOnUnprovenClangdNavigation,
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
            Self::AutoNarrowOnUnprovenBuildWorkspace => "auto_narrow_on_unproven_build_workspace",
            Self::AutoNarrowOnUnprovenCompileRunDebug => {
                "auto_narrow_on_unproven_compile_run_debug"
            }
            Self::AutoNarrowOnUnprovenClangdNavigation => {
                "auto_narrow_on_unproven_clangd_navigation"
            }
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

/// Closed confidence-class vocabulary for a daily-driver row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DailyDriverConfidenceClass {
    /// High confidence — the lane can certify replacement grade.
    HighConfidence,
    /// Medium confidence — the lane narrows below replacement grade.
    MediumConfidence,
    /// Low confidence — the lane narrows below replacement grade until evidence grows.
    LowConfidence,
}

impl DailyDriverConfidenceClass {
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

/// Closed validation-finding vocabulary for the C and C++
/// daily-driver packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required language lane has no row.
    MissingLanguageLaneCoverage,
    /// A lane claiming replacement_grade is missing a certified daily-loop step.
    MissingDailyLoopStepCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row claims replacement_grade while one or more bindings is unbound.
    ReplacementGradeWithUnboundBinding,
    /// A row narrowed below replacement grade drops its disclosure ref.
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
    /// A consumer projection remints or drops daily-driver truth.
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
            Self::MissingLanguageLaneCoverage => "missing_language_lane_coverage",
            Self::MissingDailyLoopStepCoverage => "missing_daily_loop_step_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::ReplacementGradeWithUnboundBinding => "replacement_grade_with_unbound_binding",
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

/// Consumer surface that must inherit the C and C++ daily-driver
/// packet verbatim.
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

/// One C and C++ daily-driver quality row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CAndCppDailyDriverQualityRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Language lane class this row certifies.
    pub lane_class: LanguageLaneClass,
    /// Daily-driver row class.
    pub row_class: DailyDriverRowClass,
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
    pub confidence_class: DailyDriverConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `replacement_grade`, declares a non-`none_declared` known limit,
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

impl CAndCppDailyDriverQualityRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CAndCppDailyDriverQualityConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Daily-driver packet id consumed by the projection.
    pub c_and_cpp_daily_driver_quality_packet_id_ref: String,
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

impl CAndCppDailyDriverQualityConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.c_and_cpp_daily_driver_quality_packet_id_ref == packet_id
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

/// Constructor input for
/// [`CAndCppDailyDriverQualityTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CAndCppDailyDriverQualityTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Language lane classes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<LanguageLaneClass>,
    /// Daily-driver rows.
    #[serde(default)]
    pub rows: Vec<CAndCppDailyDriverQualityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CAndCppDailyDriverQualityConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying C and C++ replacement-grade
/// daily-driver quality on the M4 stable lane with explicit
/// CMake / Ninja build-workspace, compile / run / debug fidelity, and
/// clangd rename / navigation truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CAndCppDailyDriverQualityTruthPacket {
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
    /// Language lane classes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<LanguageLaneClass>,
    /// Daily-driver rows.
    #[serde(default)]
    pub rows: Vec<CAndCppDailyDriverQualityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CAndCppDailyDriverQualityConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl CAndCppDailyDriverQualityTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: CAndCppDailyDriverQualityTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable daily-driver invariants.
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
        set.into_iter().map(LanguageLaneClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(DailyDriverRowClass::as_str).collect()
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
    ) -> CAndCppDailyDriverQualityTruthSupportExport {
        CAndCppDailyDriverQualityTruthSupportExport {
            record_kind: C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
                .to_owned(),
            schema_version: C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            c_and_cpp_daily_driver_quality_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            c_and_cpp_daily_driver_quality_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "c and cpp daily-driver quality packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "c and cpp daily-driver quality packet has the wrong schema version",
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
                FindingKind::MissingLanguageLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered language lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLanguageLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers language lane {}", lane.as_str()),
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

            if matches!(row.support_class, SupportClass::ReplacementGrade)
                && !row.all_bindings_satisfied()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ReplacementGradeWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims replacement_grade while a binding (support, known limit, downgrade automation, or evidence) is unbound",
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
                DailyDriverConfidenceClass::LowConfidence
            ) && matches!(row.support_class, SupportClass::ReplacementGrade)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ReplacementGradeWithUnboundBinding,
                    FindingSeverity::Warning,
                    format!(
                        "row {} claims replacement_grade at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
            }
        }

        for lane in &self.covered_lanes {
            let lane_claims_replacement = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, DailyDriverRowClass::DailyDriverQuality)
                    && matches!(row.support_class, SupportClass::ReplacementGrade)
            });
            if lane_claims_replacement {
                for step in DailyLoopStepClass::REQUIRED_FOR_REPLACEMENT {
                    let covered = self.rows.iter().any(|row| {
                        row.lane_class == *lane
                            && matches!(row.row_class, DailyDriverRowClass::DailyLoopStep)
                            && row.daily_loop_step_class == step
                    });
                    if !covered {
                        findings.push(ValidationFinding::new(
                            FindingKind::MissingDailyLoopStepCoverage,
                            FindingSeverity::Blocker,
                            format!(
                                "lane {} claims replacement_grade but has no daily_loop_step row for {}",
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
                        "projection {} does not preserve c and cpp daily-driver quality truth",
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
pub struct CAndCppDailyDriverQualityTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub c_and_cpp_daily_driver_quality_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub c_and_cpp_daily_driver_quality_packet: CAndCppDailyDriverQualityTruthPacket,
}

impl CAndCppDailyDriverQualityTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_VERSION
            && self.c_and_cpp_daily_driver_quality_packet_id_ref
                == self.c_and_cpp_daily_driver_quality_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .c_and_cpp_daily_driver_quality_packet
                .validate()
                .is_empty()
    }
}

/// Errors emitted when reading the checked-in stable C and C++
/// daily-driver packet.
#[derive(Debug)]
pub enum CAndCppDailyDriverQualityTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for CAndCppDailyDriverQualityTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "c and cpp daily-driver quality packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "c and cpp daily-driver quality packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CAndCppDailyDriverQualityTruthArtifactError {}

/// Returns the checked-in stable C and C++ daily-driver quality
/// truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_c_and_cpp_daily_driver_quality_truth_packet(
) -> Result<CAndCppDailyDriverQualityTruthPacket, CAndCppDailyDriverQualityTruthArtifactError> {
    let packet: CAndCppDailyDriverQualityTruthPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/language/m4/c_and_cpp_daily_driver_quality_truth_packet.json"
        )
    ))
    .map_err(CAndCppDailyDriverQualityTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(CAndCppDailyDriverQualityTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lane_evidence_ref() -> String {
        C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_DOC_REF.to_owned()
    }

    fn quality_row(row_id: &str, lane: LanguageLaneClass) -> CAndCppDailyDriverQualityRow {
        CAndCppDailyDriverQualityRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class: DailyDriverRowClass::DailyDriverQuality,
            support_class: SupportClass::ReplacementGrade,
            daily_loop_step_class: DailyLoopStepClass::NotApplicable,
            evidence_class: EvidenceClass::ArchetypeRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: DailyDriverConfidenceClass::HighConfidence,
            evidence_refs: vec![lane_evidence_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_block_on_missing_evidence",
                C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_DOC_REF
            )),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn loop_step_row(
        row_id: &str,
        lane: LanguageLaneClass,
        step: DailyLoopStepClass,
    ) -> CAndCppDailyDriverQualityRow {
        CAndCppDailyDriverQualityRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class: DailyDriverRowClass::DailyLoopStep,
            support_class: SupportClass::ReplacementGrade,
            daily_loop_step_class: step,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
            confidence_class: DailyDriverConfidenceClass::HighConfidence,
            evidence_refs: vec![lane_evidence_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_missing_fixture",
                C_AND_CPP_DAILY_DRIVER_QUALITY_TRUTH_DOC_REF
            )),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> CAndCppDailyDriverQualityConsumerProjection {
        CAndCppDailyDriverQualityConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            c_and_cpp_daily_driver_quality_packet_id_ref:
                "packet:m4:c_and_cpp_daily_driver_quality".to_owned(),
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

    fn lane_rows(lane: LanguageLaneClass, prefix: &str) -> Vec<CAndCppDailyDriverQualityRow> {
        let mut out = vec![quality_row(&format!("row:{prefix}:quality"), lane)];
        for step in DailyLoopStepClass::REQUIRED_FOR_REPLACEMENT {
            out.push(loop_step_row(
                &format!("row:{prefix}:loop:{}", step.as_str()),
                lane,
                step,
            ));
        }
        out
    }

    fn sample_input() -> CAndCppDailyDriverQualityTruthPacketInput {
        let rows = lane_rows(LanguageLaneClass::CAndCppDailyDriverLane, "c_cpp");
        CAndCppDailyDriverQualityTruthPacketInput {
            packet_id: "packet:m4:c_and_cpp_daily_driver_quality".to_owned(),
            workflow_or_surface_id: "workflow.language.c_and_cpp_daily_driver_quality".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: LanguageLaneClass::REQUIRED.to_vec(),
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
        assert_eq!(
            LanguageLaneClass::CAndCppDailyDriverLane.as_str(),
            "c_and_cpp_daily_driver_lane"
        );
        assert_eq!(
            DailyDriverRowClass::BuildWorkspaceRow.as_str(),
            "build_workspace_row"
        );
        assert_eq!(
            DailyDriverRowClass::CompileRunDebugRow.as_str(),
            "compile_run_debug_row"
        );
        assert_eq!(
            DailyDriverRowClass::ClangdNavigationRow.as_str(),
            "clangd_navigation_row"
        );
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(DailyLoopStepClass::Recover.as_str(), "recover");
        assert_eq!(
            EvidenceClass::BuildWorkspaceEvidence.as_str(),
            "build_workspace_evidence"
        );
        assert_eq!(
            EvidenceClass::CompileRunDebugEvidence.as_str(),
            "compile_run_debug_evidence"
        );
        assert_eq!(
            EvidenceClass::ClangdNavigationEvidence.as_str(),
            "clangd_navigation_evidence"
        );
        assert_eq!(EvidenceClass::EvidenceUnbound.as_str(), "evidence_unbound");
        assert_eq!(
            KnownLimitClass::BuildWorkspaceSubsetOnly.as_str(),
            "build_workspace_subset_only"
        );
        assert_eq!(
            KnownLimitClass::CompileRunDebugSubsetOnly.as_str(),
            "compile_run_debug_subset_only"
        );
        assert_eq!(
            KnownLimitClass::ClangdNavigationSubsetOnly.as_str(),
            "clangd_navigation_subset_only"
        );
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            DowngradeAutomationClass::AutoNarrowOnUnprovenBuildWorkspace.as_str(),
            "auto_narrow_on_unproven_build_workspace"
        );
        assert_eq!(
            DowngradeAutomationClass::AutoNarrowOnUnprovenCompileRunDebug.as_str(),
            "auto_narrow_on_unproven_compile_run_debug"
        );
        assert_eq!(
            DowngradeAutomationClass::AutoNarrowOnUnprovenClangdNavigation.as_str(),
            "auto_narrow_on_unproven_clangd_navigation"
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
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = CAndCppDailyDriverQualityTruthPacket::materialize(sample_input());
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
                "support:m4:c_and_cpp_daily_driver_quality",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn replacement_grade_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = CAndCppDailyDriverQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::ReplacementGradeWithUnboundBinding
        }));
    }

    #[test]
    fn missing_daily_loop_step_for_replacement_grade_blocks() {
        let mut input = sample_input();
        input
            .rows
            .retain(|row| row.daily_loop_step_class != DailyLoopStepClass::Recover);
        let packet = CAndCppDailyDriverQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDailyLoopStepCoverage));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::DailyDriverBelowReplacement;
        input.rows[0].disclosure_ref = None;
        let packet = CAndCppDailyDriverQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef
        }));
    }

    #[test]
    fn daily_loop_step_not_applicable_on_loop_row_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, DailyDriverRowClass::DailyLoopStep)
                && row.daily_loop_step_class == DailyLoopStepClass::Edit
            {
                row.daily_loop_step_class = DailyLoopStepClass::NotApplicable;
                break;
            }
        }
        let packet = CAndCppDailyDriverQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::DailyLoopStepNotApplicable));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != ConsumerSurface::ConformanceDashboard
        });
        let packet = CAndCppDailyDriverQualityTruthPacket::materialize(input);
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
        let packet = CAndCppDailyDriverQualityTruthPacket::materialize(input);
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
        let packet = CAndCppDailyDriverQualityTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
