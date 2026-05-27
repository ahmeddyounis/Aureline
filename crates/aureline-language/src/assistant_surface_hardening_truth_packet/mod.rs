//! Assistant-surface hardening truth packet for the M4 stable lane.
//!
//! This module is the language-owned contract that pins how the
//! completion, signature help, snippet session, code action,
//! additional edit, source labeling, and AI ghost text assistant
//! surfaces stay one boundary truth across the editor language pack,
//! framework pack panel, language settings/help, CLI/headless
//! inspector, support export, release proof index, Help/About proof
//! card, and the conformance dashboard. The seven assistant-surface
//! lanes are certified at the M4 launch-hardened grade. Surfaces MUST
//! NOT mint local copies or paraphrase assistant-surface posture; they
//! read this packet verbatim.
//!
//! Every row binds a closed `assistant_surface_lane_class`,
//! `assistant_surface_row_class`, `support_class`,
//! `provider_source_class`, `side_effect_class`,
//! `preview_requirement_class`, `snippet_session_field_class`,
//! `code_action_field_class`, `cross_cut_condition_class`,
//! `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `assistant_surface_confidence_class`
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the
//! row is narrowed below launch-hardened, declares a non-`none_declared`
//! known limit, or binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! source bodies, raw completion proposals, raw snippet bodies, raw
//! ghost-text output, secrets, ambient credentials, or any other
//! private material past the boundary. A row that claims
//! `launch_hardened` while leaving its known limit, downgrade
//! automation, or evidence class unbound is refused; the validator
//! narrows below launch-hardened instead of inheriting an adjacent
//! certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`AssistantSurfaceHardeningTruthPacket`].
pub const ASSISTANT_SURFACE_HARDENING_TRUTH_PACKET_RECORD_KIND: &str =
    "assistant_surface_hardening_truth_stable_packet";

/// Stable record-kind tag for [`AssistantSurfaceHardeningTruthSupportExport`].
pub const ASSISTANT_SURFACE_HARDENING_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "assistant_surface_hardening_truth_support_export";

/// Integer schema version for the assistant-surface hardening truth packet.
pub const ASSISTANT_SURFACE_HARDENING_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ASSISTANT_SURFACE_HARDENING_TRUTH_SCHEMA_REF: &str =
    "schemas/language/assistant_surface_hardening_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const ASSISTANT_SURFACE_HARDENING_TRUTH_DOC_REF: &str =
    "docs/languages/m4/harden-completion-signature-help-snippets-code-actions-additional.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const ASSISTANT_SURFACE_HARDENING_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m4/harden-completion-signature-help-snippets-code-actions-additional.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const ASSISTANT_SURFACE_HARDENING_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m4/assistant_surface_hardening_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const ASSISTANT_SURFACE_HARDENING_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m4/assistant_surface_hardening_truth_packet.json";

/// Closed assistant-surface lane vocabulary. Every required lane MUST
/// have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantSurfaceLaneClass {
    /// Deterministic and fallback completion lane.
    CompletionLane,
    /// Signature help lane.
    SignatureHelpLane,
    /// Snippet session lane.
    SnippetSessionLane,
    /// Code action lane.
    CodeActionLane,
    /// Additional-edit admission lane.
    AdditionalEditLane,
    /// Source labeling lane (provider/source attribution).
    SourceLabelingLane,
    /// AI ghost-text lane.
    AiGhostTextLane,
}

impl AssistantSurfaceLaneClass {
    /// Every required assistant-surface lane, in declaration order.
    pub const REQUIRED: [Self; 7] = [
        Self::CompletionLane,
        Self::SignatureHelpLane,
        Self::SnippetSessionLane,
        Self::CodeActionLane,
        Self::AdditionalEditLane,
        Self::SourceLabelingLane,
        Self::AiGhostTextLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompletionLane => "completion_lane",
            Self::SignatureHelpLane => "signature_help_lane",
            Self::SnippetSessionLane => "snippet_session_lane",
            Self::CodeActionLane => "code_action_lane",
            Self::AdditionalEditLane => "additional_edit_lane",
            Self::SourceLabelingLane => "source_labeling_lane",
            Self::AiGhostTextLane => "ai_ghost_text_lane",
        }
    }
}

/// Closed assistant-surface row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantSurfaceRowClass {
    /// The lane's headline qualification row.
    AssistantSurfaceQuality,
    /// Provider/source-class binding row distinguishing deterministic,
    /// cached/local-word, snippet-only, and AI ghost-text classes.
    ProviderSourceBinding,
    /// Side-effect admission row exposing additional edits, generated
    /// files, dependency/config changes, or protected-surface writes.
    SideEffectAdmission,
    /// Snippet session truth row covering one of the required snippet
    /// session field bindings (label/source, placeholder index/count,
    /// exit route, multi-cursor compatibility).
    SnippetSessionTruth,
    /// Code action truth row covering one of the required code action
    /// field bindings (provider/source, side-effect class, partial-
    /// support reason, preview requirement).
    CodeActionTruth,
    /// Cross-cut condition row covering one of the required conditions
    /// (IME, multi-cursor, large-file, restricted-mode, degraded-
    /// provider).
    CrossCutCondition,
    /// Launch-language coverage row certifying a language touchpoint.
    LaunchLanguageCoverage,
    /// Precisely labeled unsupported-gap row on a lane.
    UnsupportedGap,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl AssistantSurfaceRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssistantSurfaceQuality => "assistant_surface_quality",
            Self::ProviderSourceBinding => "provider_source_binding",
            Self::SideEffectAdmission => "side_effect_admission",
            Self::SnippetSessionTruth => "snippet_session_truth",
            Self::CodeActionTruth => "code_action_truth",
            Self::CrossCutCondition => "cross_cut_condition",
            Self::LaunchLanguageCoverage => "launch_language_coverage",
            Self::UnsupportedGap => "unsupported_gap",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }
}

/// Closed support-class vocabulary applied to an assistant-surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-hardened grade.
    LaunchHardened,
    /// Row is intentionally narrowed below launch-hardened; narrowing is disclosed.
    LaunchHardenedBelow,
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
            Self::LaunchHardened => "launch_hardened",
            Self::LaunchHardenedBelow => "launch_hardened_below",
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
        !matches!(self, Self::LaunchHardened)
    }
}

/// Provider/source class for an assistant-surface proposal. Distinguishes
/// deterministic completion, cached/local-word fallback, snippet-only
/// suggestions, and AI ghost text by stable provider/source rather than
/// by theme accident.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSourceClass {
    /// Deterministic completion from a language server, parser, or schema.
    DeterministicCompletion,
    /// Cached or local-word fallback proposal.
    CachedLocalWordFallback,
    /// Snippet-only suggestion (template body, no semantic provider).
    SnippetOnlySuggestion,
    /// AI ghost-text proposal sourced from a generative provider.
    AiGhostText,
    /// Row is not bound to a provider/source class.
    NotApplicable,
}

impl ProviderSourceClass {
    /// Every distinct provider/source class required by lanes that
    /// surface deterministic vs AI ghost-text boundaries.
    pub const REQUIRED_FOR_PROVIDER_BINDING: [Self; 4] = [
        Self::DeterministicCompletion,
        Self::CachedLocalWordFallback,
        Self::SnippetOnlySuggestion,
        Self::AiGhostText,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeterministicCompletion => "deterministic_completion",
            Self::CachedLocalWordFallback => "cached_local_word_fallback",
            Self::SnippetOnlySuggestion => "snippet_only_suggestion",
            Self::AiGhostText => "ai_ghost_text",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed side-effect class. Any accepted proposal that introduces
/// additional edits, generated files, dependency/config changes, or
/// protected-surface writes MUST expose its side-effect class before
/// apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// No side effect beyond the inline insertion.
    NoSideEffect,
    /// Side effect: additional edits attached to the proposal.
    AdditionalEdits,
    /// Side effect: generated files attached to the proposal.
    GeneratedFiles,
    /// Side effect: dependency or config change attached to the proposal.
    DependencyOrConfigChange,
    /// Side effect: write to a protected surface (e.g., lockfile,
    /// generated artifact, vendored copy).
    ProtectedSurfaceWrite,
    /// Row is not bound to a side-effect class.
    NotApplicable,
    /// Row has no bound side-effect class; this never qualifies stable
    /// for a row class that requires a binding.
    SideEffectUnbound,
}

impl SideEffectClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSideEffect => "no_side_effect",
            Self::AdditionalEdits => "additional_edits",
            Self::GeneratedFiles => "generated_files",
            Self::DependencyOrConfigChange => "dependency_or_config_change",
            Self::ProtectedSurfaceWrite => "protected_surface_write",
            Self::NotApplicable => "not_applicable",
            Self::SideEffectUnbound => "side_effect_unbound",
        }
    }

    /// True when this side-effect class is bound (any value other than
    /// `side_effect_unbound`).
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SideEffectUnbound)
    }
}

/// Closed preview-requirement class. Code-action proposals that mutate
/// multiple files, generated files, dependencies, or protected surfaces
/// MUST require preview before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewRequirementClass {
    /// Preview is not required for this row.
    PreviewNotRequired,
    /// Preview is required for multi-file mutations.
    PreviewRequiredForMultiFile,
    /// Preview is required for generated-file mutations.
    PreviewRequiredForGeneratedFile,
    /// Preview is required for dependency or config changes.
    PreviewRequiredForDependencyChange,
    /// Preview is required for protected-surface writes.
    PreviewRequiredForProtectedSurface,
    /// Row is not bound to a preview requirement.
    NotApplicable,
    /// Row has no bound preview requirement; this never qualifies stable
    /// for a row class that requires a binding.
    PreviewUnbound,
}

impl PreviewRequirementClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewNotRequired => "preview_not_required",
            Self::PreviewRequiredForMultiFile => "preview_required_for_multi_file",
            Self::PreviewRequiredForGeneratedFile => "preview_required_for_generated_file",
            Self::PreviewRequiredForDependencyChange => "preview_required_for_dependency_change",
            Self::PreviewRequiredForProtectedSurface => "preview_required_for_protected_surface",
            Self::NotApplicable => "not_applicable",
            Self::PreviewUnbound => "preview_unbound",
        }
    }

    /// True when this preview-requirement class is bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::PreviewUnbound)
    }
}

/// Closed snippet session field vocabulary. A snippet session MUST
/// surface label/source, placeholder index/count, exit route, and
/// multi-cursor compatibility — a `snippet_session_truth` row binds
/// exactly one field at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnippetSessionFieldClass {
    /// The row proves the snippet/source label is surfaced.
    SnippetOrSourceLabel,
    /// The row proves the placeholder index/count is surfaced.
    PlaceholderIndexCount,
    /// The row proves the snippet exit route is surfaced.
    ExitRoute,
    /// The row proves multi-cursor compatibility is surfaced.
    MultiCursorCompatibility,
    /// Row is not a snippet-session row.
    NotApplicable,
}

impl SnippetSessionFieldClass {
    /// Every required snippet session field, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::SnippetOrSourceLabel,
        Self::PlaceholderIndexCount,
        Self::ExitRoute,
        Self::MultiCursorCompatibility,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnippetOrSourceLabel => "snippet_or_source_label",
            Self::PlaceholderIndexCount => "placeholder_index_count",
            Self::ExitRoute => "exit_route",
            Self::MultiCursorCompatibility => "multi_cursor_compatibility",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed code action field vocabulary. A code-action entry MUST
/// preserve provider/source, side-effect class, partial-support reason,
/// and preview requirement — a `code_action_truth` row binds exactly
/// one field at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionFieldClass {
    /// The row proves the provider/source field is preserved.
    ProviderOrSource,
    /// The row proves the side-effect class field is preserved.
    #[serde(rename = "side_effect_class")]
    SideEffectClassField,
    /// The row proves the partial-support reason field is preserved.
    PartialSupportReason,
    /// The row proves the preview-requirement field is preserved.
    #[serde(rename = "preview_requirement")]
    PreviewRequirementField,
    /// Row is not a code-action row.
    NotApplicable,
}

impl CodeActionFieldClass {
    /// Every required code-action field, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::ProviderOrSource,
        Self::SideEffectClassField,
        Self::PartialSupportReason,
        Self::PreviewRequirementField,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOrSource => "provider_or_source",
            Self::SideEffectClassField => "side_effect_class",
            Self::PartialSupportReason => "partial_support_reason",
            Self::PreviewRequirementField => "preview_requirement",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed cross-cut condition vocabulary. A lane that claims
/// `launch_hardened` MUST cover every cross-cut condition required by
/// the addendum: IME, multi-cursor, large-file, restricted-mode, and
/// degraded-provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrossCutConditionClass {
    /// IME (input method editor) condition.
    Ime,
    /// Multi-cursor condition.
    MultiCursor,
    /// Large-file condition.
    LargeFile,
    /// Restricted-mode condition.
    RestrictedMode,
    /// Degraded-provider condition.
    DegradedProvider,
    /// Row is not a cross-cut row.
    NotApplicable,
}

impl CrossCutConditionClass {
    /// Every required cross-cut condition, in declaration order.
    pub const REQUIRED_FOR_LAUNCH: [Self; 5] = [
        Self::Ime,
        Self::MultiCursor,
        Self::LargeFile,
        Self::RestrictedMode,
        Self::DegradedProvider,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ime => "ime",
            Self::MultiCursor => "multi_cursor",
            Self::LargeFile => "large_file",
            Self::RestrictedMode => "restricted_mode",
            Self::DegradedProvider => "degraded_provider",
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

/// Closed known-limit vocabulary attached to an assistant-surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The row only certifies a provider subset.
    ProviderSubsetOnly,
    /// The row only certifies a language subset.
    LanguageSubsetOnly,
    /// The row only certifies an archetype subset.
    ArchetypeSubsetOnly,
    /// The row only certifies a condition subset (IME, multi-cursor, etc.).
    ConditionSubsetOnly,
    /// The row only certifies a snippet/code-action field subset.
    FieldSubsetOnly,
    /// The row certifies a side-effect-class-only narrowing
    /// (e.g., admits AdditionalEdits but not GeneratedFiles).
    SideEffectScopeOnly,
    /// The row only certifies a preview-requirement subset.
    PreviewSubsetOnly,
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
            Self::ProviderSubsetOnly => "provider_subset_only",
            Self::LanguageSubsetOnly => "language_subset_only",
            Self::ArchetypeSubsetOnly => "archetype_subset_only",
            Self::ConditionSubsetOnly => "condition_subset_only",
            Self::FieldSubsetOnly => "field_subset_only",
            Self::SideEffectScopeOnly => "side_effect_scope_only",
            Self::PreviewSubsetOnly => "preview_subset_only",
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

/// Closed downgrade-automation vocabulary attached to an assistant-surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a certified fixture is missing or stale.
    AutoNarrowOnMissingFixture,
    /// Automatically narrow when a certified archetype repo is missing.
    AutoNarrowOnMissingArchetype,
    /// Automatically narrow when a provider class drops below depth.
    AutoNarrowOnProviderGap,
    /// Automatically narrow when a cross-cut condition fails.
    AutoNarrowOnConditionFailure,
    /// Automatically narrow when ghost-text labeling regresses.
    AutoNarrowOnGhostTextLabelDrift,
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
            Self::AutoNarrowOnProviderGap => "auto_narrow_on_provider_gap",
            Self::AutoNarrowOnConditionFailure => "auto_narrow_on_condition_failure",
            Self::AutoNarrowOnGhostTextLabelDrift => "auto_narrow_on_ghost_text_label_drift",
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

/// Closed confidence-class vocabulary for an assistant-surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantSurfaceConfidenceClass {
    /// High confidence — the lane can certify launch-hardened.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-hardened.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-hardened until evidence grows.
    LowConfidence,
}

impl AssistantSurfaceConfidenceClass {
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

/// Closed validation-finding vocabulary for the assistant-surface hardening packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required assistant-surface lane has no row.
    MissingAssistantSurfaceLaneCoverage,
    /// A lane claiming launch_hardened is missing a required cross-cut condition.
    MissingCrossCutConditionCoverage,
    /// A lane claiming launch_hardened is missing a required provider/source binding.
    MissingProviderSourceBindingCoverage,
    /// A lane claiming launch_hardened is missing a required snippet-session field.
    MissingSnippetSessionFieldCoverage,
    /// A lane claiming launch_hardened is missing a required code-action field.
    MissingCodeActionFieldCoverage,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row has no bound side-effect class on a row-class that requires it.
    MissingSideEffectClass,
    /// A row has no bound preview-requirement class on a row-class that requires it.
    MissingPreviewRequirementClass,
    /// A row claims launch_hardened while one or more bindings is unbound.
    LaunchHardenedWithUnboundBinding,
    /// A row narrowed below launch-hardened drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A provider/source binding row drops its provider/source class.
    ProviderSourceClassNotApplicable,
    /// A non-provider-source row binds a provider/source class.
    ProviderSourceClassNotPermittedOnRowClass,
    /// A side-effect admission row drops its side-effect class.
    SideEffectClassNotApplicable,
    /// A non-side-effect row binds a non-`not_applicable` side-effect class.
    SideEffectClassNotPermittedOnRowClass,
    /// A snippet-session truth row drops its snippet-session field binding.
    SnippetSessionFieldNotApplicable,
    /// A non-snippet-session row binds a snippet-session field.
    SnippetSessionFieldNotPermittedOnRowClass,
    /// A code-action truth row drops its code-action field binding.
    CodeActionFieldNotApplicable,
    /// A non-code-action row binds a code-action field.
    CodeActionFieldNotPermittedOnRowClass,
    /// A cross-cut condition row drops its condition binding.
    CrossCutConditionNotApplicable,
    /// A non-cross-cut row binds a cross-cut condition.
    CrossCutConditionNotPermittedOnRowClass,
    /// A preview-requirement is bound on a row class that does not permit it.
    PreviewRequirementNotPermittedOnRowClass,
    /// A row admits raw source bodies or other private material.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops assistant-surface truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the provider/source-class vocabulary.
    ProviderSourceClassVocabularyCollapsed,
    /// A projection collapses the side-effect-class vocabulary.
    SideEffectClassVocabularyCollapsed,
    /// A projection collapses the preview-requirement vocabulary.
    PreviewRequirementVocabularyCollapsed,
    /// A projection collapses the snippet-session-field vocabulary.
    SnippetSessionFieldVocabularyCollapsed,
    /// A projection collapses the code-action-field vocabulary.
    CodeActionFieldVocabularyCollapsed,
    /// A projection collapses the cross-cut-condition vocabulary.
    CrossCutConditionVocabularyCollapsed,
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
            Self::MissingAssistantSurfaceLaneCoverage => {
                "missing_assistant_surface_lane_coverage"
            }
            Self::MissingCrossCutConditionCoverage => "missing_cross_cut_condition_coverage",
            Self::MissingProviderSourceBindingCoverage => {
                "missing_provider_source_binding_coverage"
            }
            Self::MissingSnippetSessionFieldCoverage => "missing_snippet_session_field_coverage",
            Self::MissingCodeActionFieldCoverage => "missing_code_action_field_coverage",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingSideEffectClass => "missing_side_effect_class",
            Self::MissingPreviewRequirementClass => "missing_preview_requirement_class",
            Self::LaunchHardenedWithUnboundBinding => "launch_hardened_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::ProviderSourceClassNotApplicable => "provider_source_class_not_applicable",
            Self::ProviderSourceClassNotPermittedOnRowClass => {
                "provider_source_class_not_permitted_on_row_class"
            }
            Self::SideEffectClassNotApplicable => "side_effect_class_not_applicable",
            Self::SideEffectClassNotPermittedOnRowClass => {
                "side_effect_class_not_permitted_on_row_class"
            }
            Self::SnippetSessionFieldNotApplicable => "snippet_session_field_not_applicable",
            Self::SnippetSessionFieldNotPermittedOnRowClass => {
                "snippet_session_field_not_permitted_on_row_class"
            }
            Self::CodeActionFieldNotApplicable => "code_action_field_not_applicable",
            Self::CodeActionFieldNotPermittedOnRowClass => {
                "code_action_field_not_permitted_on_row_class"
            }
            Self::CrossCutConditionNotApplicable => "cross_cut_condition_not_applicable",
            Self::CrossCutConditionNotPermittedOnRowClass => {
                "cross_cut_condition_not_permitted_on_row_class"
            }
            Self::PreviewRequirementNotPermittedOnRowClass => {
                "preview_requirement_not_permitted_on_row_class"
            }
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::ProviderSourceClassVocabularyCollapsed => {
                "provider_source_class_vocabulary_collapsed"
            }
            Self::SideEffectClassVocabularyCollapsed => "side_effect_class_vocabulary_collapsed",
            Self::PreviewRequirementVocabularyCollapsed => {
                "preview_requirement_vocabulary_collapsed"
            }
            Self::SnippetSessionFieldVocabularyCollapsed => {
                "snippet_session_field_vocabulary_collapsed"
            }
            Self::CodeActionFieldVocabularyCollapsed => "code_action_field_vocabulary_collapsed",
            Self::CrossCutConditionVocabularyCollapsed => {
                "cross_cut_condition_vocabulary_collapsed"
            }
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the assistant-surface packet verbatim.
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

/// One assistant-surface hardening row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurfaceHardeningRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Assistant-surface lane this row certifies.
    pub lane_class: AssistantSurfaceLaneClass,
    /// Row class.
    pub row_class: AssistantSurfaceRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Provider/source class for the row (or `not_applicable`).
    pub provider_source_class: ProviderSourceClass,
    /// Side-effect class for the row.
    pub side_effect_class: SideEffectClass,
    /// Preview-requirement class for the row.
    pub preview_requirement_class: PreviewRequirementClass,
    /// Snippet-session field this row binds (or `not_applicable`).
    pub snippet_session_field_class: SnippetSessionFieldClass,
    /// Code-action field this row binds (or `not_applicable`).
    pub code_action_field_class: CodeActionFieldClass,
    /// Cross-cut condition this row certifies (or `not_applicable`).
    pub cross_cut_condition_class: CrossCutConditionClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: AssistantSurfaceConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_hardened`, declares a non-`none_declared` known limit,
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

impl AssistantSurfaceHardeningRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.side_effect_class.is_bound()
            && self.preview_requirement_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurfaceHardeningConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Assistant-surface packet id consumed by the projection.
    pub assistant_surface_hardening_packet_id_ref: String,
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
    /// True when the provider/source-class vocabulary is preserved verbatim.
    pub preserves_provider_source_class_vocabulary: bool,
    /// True when the side-effect-class vocabulary is preserved verbatim.
    pub preserves_side_effect_class_vocabulary: bool,
    /// True when the preview-requirement vocabulary is preserved verbatim.
    pub preserves_preview_requirement_vocabulary: bool,
    /// True when the snippet-session-field vocabulary is preserved verbatim.
    pub preserves_snippet_session_field_vocabulary: bool,
    /// True when the code-action-field vocabulary is preserved verbatim.
    pub preserves_code_action_field_vocabulary: bool,
    /// True when the cross-cut-condition vocabulary is preserved verbatim.
    pub preserves_cross_cut_condition_vocabulary: bool,
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

impl AssistantSurfaceHardeningConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.assistant_surface_hardening_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_provider_source_class_vocabulary
            && self.preserves_side_effect_class_vocabulary
            && self.preserves_preview_requirement_vocabulary
            && self.preserves_snippet_session_field_vocabulary
            && self.preserves_code_action_field_vocabulary
            && self.preserves_cross_cut_condition_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`AssistantSurfaceHardeningTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurfaceHardeningTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Assistant-surface lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<AssistantSurfaceLaneClass>,
    /// Assistant-surface rows.
    #[serde(default)]
    pub rows: Vec<AssistantSurfaceHardeningRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<AssistantSurfaceHardeningConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet certifying completion, signature help,
/// snippet session, code action, additional edit, source labeling, and
/// AI ghost text boundaries at the M4 launch-hardened grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurfaceHardeningTruthPacket {
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
    /// Assistant-surface lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<AssistantSurfaceLaneClass>,
    /// Assistant-surface rows.
    #[serde(default)]
    pub rows: Vec<AssistantSurfaceHardeningRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<AssistantSurfaceHardeningConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl AssistantSurfaceHardeningTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: AssistantSurfaceHardeningTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: ASSISTANT_SURFACE_HARDENING_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: ASSISTANT_SURFACE_HARDENING_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable assistant-surface invariants.
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
            .map(AssistantSurfaceLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(AssistantSurfaceRowClass::as_str)
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

    /// Returns the unique provider/source-class tokens observed across rows.
    pub fn provider_source_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.provider_source_class);
        }
        set.into_iter().map(ProviderSourceClass::as_str).collect()
    }

    /// Returns the unique side-effect-class tokens observed across rows.
    pub fn side_effect_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.side_effect_class);
        }
        set.into_iter().map(SideEffectClass::as_str).collect()
    }

    /// Returns the unique preview-requirement tokens observed across rows.
    pub fn preview_requirement_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.preview_requirement_class);
        }
        set.into_iter().map(PreviewRequirementClass::as_str).collect()
    }

    /// Returns the unique snippet-session-field tokens observed across rows.
    pub fn snippet_session_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.snippet_session_field_class);
        }
        set.into_iter().map(SnippetSessionFieldClass::as_str).collect()
    }

    /// Returns the unique code-action-field tokens observed across rows.
    pub fn code_action_field_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.code_action_field_class);
        }
        set.into_iter().map(CodeActionFieldClass::as_str).collect()
    }

    /// Returns the unique cross-cut-condition tokens observed across rows.
    pub fn cross_cut_condition_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.cross_cut_condition_class);
        }
        set.into_iter().map(CrossCutConditionClass::as_str).collect()
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
    ) -> AssistantSurfaceHardeningTruthSupportExport {
        AssistantSurfaceHardeningTruthSupportExport {
            record_kind: ASSISTANT_SURFACE_HARDENING_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ASSISTANT_SURFACE_HARDENING_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            assistant_surface_hardening_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            assistant_surface_hardening_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != ASSISTANT_SURFACE_HARDENING_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "assistant-surface packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != ASSISTANT_SURFACE_HARDENING_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "assistant-surface packet has the wrong schema version",
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
                FindingKind::MissingAssistantSurfaceLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered assistant-surface lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingAssistantSurfaceLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers assistant-surface lane {}", lane.as_str()),
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
        row: &AssistantSurfaceHardeningRow,
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
        if !row.side_effect_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSideEffectClass,
                FindingSeverity::Blocker,
                format!("row {} has no bound side-effect class", row.row_id),
            ));
        }
        if !row.preview_requirement_class.is_bound() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingPreviewRequirementClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has no bound preview-requirement class",
                    row.row_id
                ),
            ));
        }

        if matches!(row.support_class, SupportClass::LaunchHardened)
            && !row.all_bindings_satisfied()
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchHardenedWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims launch_hardened while a binding (support, known limit, downgrade automation, evidence, side-effect, or preview-requirement) is unbound",
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
        let is_provider_source = matches!(row.row_class, AssistantSurfaceRowClass::ProviderSourceBinding);
        let is_side_effect = matches!(row.row_class, AssistantSurfaceRowClass::SideEffectAdmission);
        let is_snippet_session = matches!(row.row_class, AssistantSurfaceRowClass::SnippetSessionTruth);
        let is_code_action = matches!(row.row_class, AssistantSurfaceRowClass::CodeActionTruth);
        let is_cross_cut = matches!(row.row_class, AssistantSurfaceRowClass::CrossCutCondition);

        if is_provider_source
            && matches!(row.provider_source_class, ProviderSourceClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::ProviderSourceClassNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a provider_source_binding but has no bound provider/source class",
                    row.row_id
                ),
            ));
        }
        if !is_provider_source
            && !matches!(row.provider_source_class, ProviderSourceClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::ProviderSourceClassNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds provider/source class {}; only provider_source_binding rows may bind a provider/source class",
                    row.row_id,
                    row.row_class.as_str(),
                    row.provider_source_class.as_str()
                ),
            ));
        }

        if is_side_effect
            && matches!(row.side_effect_class, SideEffectClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::SideEffectClassNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a side_effect_admission but has no bound side-effect class",
                    row.row_id
                ),
            ));
        }
        if !is_side_effect
            && !matches!(
                row.side_effect_class,
                SideEffectClass::NotApplicable | SideEffectClass::SideEffectUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::SideEffectClassNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds side-effect class {}; only side_effect_admission rows may bind a side-effect class",
                    row.row_id,
                    row.row_class.as_str(),
                    row.side_effect_class.as_str()
                ),
            ));
        }

        if is_snippet_session
            && matches!(
                row.snippet_session_field_class,
                SnippetSessionFieldClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::SnippetSessionFieldNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a snippet_session_truth but has no bound snippet-session field",
                    row.row_id
                ),
            ));
        }
        if !is_snippet_session
            && !matches!(
                row.snippet_session_field_class,
                SnippetSessionFieldClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::SnippetSessionFieldNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds snippet-session field {}; only snippet_session_truth rows may bind a snippet-session field",
                    row.row_id,
                    row.row_class.as_str(),
                    row.snippet_session_field_class.as_str()
                ),
            ));
        }

        if is_code_action
            && matches!(row.code_action_field_class, CodeActionFieldClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::CodeActionFieldNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a code_action_truth but has no bound code-action field",
                    row.row_id
                ),
            ));
        }
        if !is_code_action
            && !matches!(row.code_action_field_class, CodeActionFieldClass::NotApplicable)
        {
            findings.push(ValidationFinding::new(
                FindingKind::CodeActionFieldNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds code-action field {}; only code_action_truth rows may bind a code-action field",
                    row.row_id,
                    row.row_class.as_str(),
                    row.code_action_field_class.as_str()
                ),
            ));
        }

        if is_cross_cut
            && matches!(
                row.cross_cut_condition_class,
                CrossCutConditionClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::CrossCutConditionNotApplicable,
                FindingSeverity::Blocker,
                format!(
                    "row {} is a cross_cut_condition but has no bound cross-cut condition",
                    row.row_id
                ),
            ));
        }
        if !is_cross_cut
            && !matches!(
                row.cross_cut_condition_class,
                CrossCutConditionClass::NotApplicable
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::CrossCutConditionNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds cross-cut condition {}; only cross_cut_condition rows may bind a cross-cut condition",
                    row.row_id,
                    row.row_class.as_str(),
                    row.cross_cut_condition_class.as_str()
                ),
            ));
        }

        // Preview requirement: only code-action and side-effect rows may bind a non-`not_applicable` value.
        if !is_code_action
            && !is_side_effect
            && !matches!(
                row.preview_requirement_class,
                PreviewRequirementClass::NotApplicable | PreviewRequirementClass::PreviewUnbound
            )
        {
            findings.push(ValidationFinding::new(
                FindingKind::PreviewRequirementNotPermittedOnRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} has row class {} but binds preview-requirement {}; only code_action_truth or side_effect_admission rows may bind a preview requirement",
                    row.row_id,
                    row.row_class.as_str(),
                    row.preview_requirement_class.as_str()
                ),
            ));
        }

        if matches!(
            row.confidence_class,
            AssistantSurfaceConfidenceClass::LowConfidence
        ) && matches!(row.support_class, SupportClass::LaunchHardened)
        {
            findings.push(ValidationFinding::new(
                FindingKind::LaunchHardenedWithUnboundBinding,
                FindingSeverity::Warning,
                format!(
                    "row {} claims launch_hardened at low_confidence; narrowing until evidence grows",
                    row.row_id
                ),
            ));
        }
    }

    fn append_per_lane_coverage_findings(
        &self,
        lane: AssistantSurfaceLaneClass,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let lane_claims_hardened = self.rows.iter().any(|row| {
            row.lane_class == lane
                && matches!(row.row_class, AssistantSurfaceRowClass::AssistantSurfaceQuality)
                && matches!(row.support_class, SupportClass::LaunchHardened)
        });
        if !lane_claims_hardened {
            return;
        }

        for condition in CrossCutConditionClass::REQUIRED_FOR_LAUNCH {
            let covered = self.rows.iter().any(|row| {
                row.lane_class == lane
                    && matches!(row.row_class, AssistantSurfaceRowClass::CrossCutCondition)
                    && row.cross_cut_condition_class == condition
            });
            if !covered {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingCrossCutConditionCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_hardened but has no cross_cut_condition row for {}",
                        lane.as_str(),
                        condition.as_str()
                    ),
                ));
            }
        }

        if lane_requires_provider_source_binding(lane) {
            for provider in ProviderSourceClass::REQUIRED_FOR_PROVIDER_BINDING {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == lane
                        && matches!(
                            row.row_class,
                            AssistantSurfaceRowClass::ProviderSourceBinding
                        )
                        && row.provider_source_class == provider
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingProviderSourceBindingCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_hardened but has no provider_source_binding row for {}",
                            lane.as_str(),
                            provider.as_str()
                        ),
                    ));
                }
            }
        }

        if matches!(lane, AssistantSurfaceLaneClass::SnippetSessionLane) {
            for field in SnippetSessionFieldClass::REQUIRED {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == lane
                        && matches!(row.row_class, AssistantSurfaceRowClass::SnippetSessionTruth)
                        && row.snippet_session_field_class == field
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingSnippetSessionFieldCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "snippet_session_lane claims launch_hardened but has no snippet_session_truth row for {}",
                            field.as_str()
                        ),
                    ));
                }
            }
        }

        if matches!(lane, AssistantSurfaceLaneClass::CodeActionLane) {
            for field in CodeActionFieldClass::REQUIRED {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == lane
                        && matches!(row.row_class, AssistantSurfaceRowClass::CodeActionTruth)
                        && row.code_action_field_class == field
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingCodeActionFieldCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "code_action_lane claims launch_hardened but has no code_action_truth row for {}",
                            field.as_str()
                        ),
                    ));
                }
            }
        }
    }

    fn append_projection_findings(
        &self,
        projection: &AssistantSurfaceHardeningConsumerProjection,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if !projection.preserves_truth_for(&self.packet_id) {
            findings.push(ValidationFinding::new(
                FindingKind::ConsumerProjectionDrift,
                FindingSeverity::Blocker,
                format!(
                    "projection {} does not preserve assistant-surface truth",
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
        if !projection.preserves_provider_source_class_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::ProviderSourceClassVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the provider/source-class vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_side_effect_class_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::SideEffectClassVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the side-effect-class vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_preview_requirement_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::PreviewRequirementVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the preview-requirement vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_snippet_session_field_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::SnippetSessionFieldVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the snippet-session-field vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_code_action_field_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::CodeActionFieldVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the code-action-field vocabulary",
                    projection.projection_ref
                ),
            ));
        }
        if !projection.preserves_cross_cut_condition_vocabulary {
            findings.push(ValidationFinding::new(
                FindingKind::CrossCutConditionVocabularyCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "projection {} collapses the cross-cut-condition vocabulary",
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

fn lane_requires_provider_source_binding(lane: AssistantSurfaceLaneClass) -> bool {
    matches!(
        lane,
        AssistantSurfaceLaneClass::CompletionLane
            | AssistantSurfaceLaneClass::SourceLabelingLane
            | AssistantSurfaceLaneClass::AiGhostTextLane
    )
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
pub struct AssistantSurfaceHardeningTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub assistant_surface_hardening_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub assistant_surface_hardening_packet: AssistantSurfaceHardeningTruthPacket,
}

impl AssistantSurfaceHardeningTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == ASSISTANT_SURFACE_HARDENING_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == ASSISTANT_SURFACE_HARDENING_TRUTH_SCHEMA_VERSION
            && self.assistant_surface_hardening_packet_id_ref
                == self.assistant_surface_hardening_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.assistant_surface_hardening_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable assistant-surface packet.
#[derive(Debug)]
pub enum AssistantSurfaceHardeningTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for AssistantSurfaceHardeningTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "assistant-surface packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "assistant-surface packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AssistantSurfaceHardeningTruthArtifactError {}

/// Returns the checked-in stable assistant-surface hardening truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_assistant_surface_hardening_truth_packet(
) -> Result<AssistantSurfaceHardeningTruthPacket, AssistantSurfaceHardeningTruthArtifactError> {
    let packet: AssistantSurfaceHardeningTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m4/assistant_surface_hardening_truth_packet.json"
    )))
    .map_err(AssistantSurfaceHardeningTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(AssistantSurfaceHardeningTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        ASSISTANT_SURFACE_HARDENING_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        ASSISTANT_SURFACE_HARDENING_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn base_row(
        row_id: &str,
        lane: AssistantSurfaceLaneClass,
        row_class: AssistantSurfaceRowClass,
    ) -> AssistantSurfaceHardeningRow {
        AssistantSurfaceHardeningRow {
            row_id: row_id.to_owned(),
            lane_class: lane,
            row_class,
            support_class: SupportClass::LaunchHardened,
            provider_source_class: ProviderSourceClass::NotApplicable,
            side_effect_class: SideEffectClass::NotApplicable,
            preview_requirement_class: PreviewRequirementClass::NotApplicable,
            snippet_session_field_class: SnippetSessionFieldClass::NotApplicable,
            code_action_field_class: CodeActionFieldClass::NotApplicable,
            cross_cut_condition_class: CrossCutConditionClass::NotApplicable,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnMissingFixture,
            confidence_class: AssistantSurfaceConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_missing_fixture", doc_ref())),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn quality_row(lane: AssistantSurfaceLaneClass, prefix: &str) -> AssistantSurfaceHardeningRow {
        let mut row = base_row(
            &format!("row:{prefix}:quality"),
            lane,
            AssistantSurfaceRowClass::AssistantSurfaceQuality,
        );
        row.evidence_class = EvidenceClass::ArchetypeRepoEvidence;
        row.downgrade_automation_class = DowngradeAutomationClass::AutoBlockOnMissingEvidence;
        row.disclosure_ref = Some(format!("{}#auto_block_on_missing_evidence", doc_ref()));
        row.evidence_refs = vec![doc_ref(), fixture_ref()];
        row
    }

    fn cross_cut_rows(
        lane: AssistantSurfaceLaneClass,
        prefix: &str,
    ) -> Vec<AssistantSurfaceHardeningRow> {
        CrossCutConditionClass::REQUIRED_FOR_LAUNCH
            .into_iter()
            .map(|cond| {
                let mut row = base_row(
                    &format!("row:{prefix}:condition:{}", cond.as_str()),
                    lane,
                    AssistantSurfaceRowClass::CrossCutCondition,
                );
                row.cross_cut_condition_class = cond;
                row.evidence_class = EvidenceClass::ConformanceSuiteEvidence;
                row
            })
            .collect()
    }

    fn provider_source_rows(
        lane: AssistantSurfaceLaneClass,
        prefix: &str,
    ) -> Vec<AssistantSurfaceHardeningRow> {
        ProviderSourceClass::REQUIRED_FOR_PROVIDER_BINDING
            .into_iter()
            .map(|provider| {
                let mut row = base_row(
                    &format!("row:{prefix}:provider:{}", provider.as_str()),
                    lane,
                    AssistantSurfaceRowClass::ProviderSourceBinding,
                );
                row.provider_source_class = provider;
                row
            })
            .collect()
    }

    fn snippet_session_rows(prefix: &str) -> Vec<AssistantSurfaceHardeningRow> {
        SnippetSessionFieldClass::REQUIRED
            .into_iter()
            .map(|field| {
                let mut row = base_row(
                    &format!("row:{prefix}:snippet_field:{}", field.as_str()),
                    AssistantSurfaceLaneClass::SnippetSessionLane,
                    AssistantSurfaceRowClass::SnippetSessionTruth,
                );
                row.snippet_session_field_class = field;
                row
            })
            .collect()
    }

    fn code_action_rows(prefix: &str) -> Vec<AssistantSurfaceHardeningRow> {
        CodeActionFieldClass::REQUIRED
            .into_iter()
            .map(|field| {
                let mut row = base_row(
                    &format!("row:{prefix}:code_action_field:{}", field.as_str()),
                    AssistantSurfaceLaneClass::CodeActionLane,
                    AssistantSurfaceRowClass::CodeActionTruth,
                );
                row.code_action_field_class = field;
                if matches!(field, CodeActionFieldClass::PreviewRequirementField) {
                    row.preview_requirement_class =
                        PreviewRequirementClass::PreviewRequiredForMultiFile;
                }
                row
            })
            .collect()
    }

    fn additional_edit_row(prefix: &str) -> AssistantSurfaceHardeningRow {
        let mut row = base_row(
            &format!("row:{prefix}:additional_edit:additional_edits"),
            AssistantSurfaceLaneClass::AdditionalEditLane,
            AssistantSurfaceRowClass::SideEffectAdmission,
        );
        row.side_effect_class = SideEffectClass::AdditionalEdits;
        row.preview_requirement_class = PreviewRequirementClass::PreviewRequiredForMultiFile;
        row.known_limit_class = KnownLimitClass::SideEffectScopeOnly;
        row.disclosure_ref = Some(format!("{}#side_effect_scope_only", doc_ref()));
        row
    }

    fn projection(surface: ConsumerSurface) -> AssistantSurfaceHardeningConsumerProjection {
        AssistantSurfaceHardeningConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            assistant_surface_hardening_packet_id_ref: "packet:m4:assistant_surface_hardening"
                .to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_provider_source_class_vocabulary: true,
            preserves_side_effect_class_vocabulary: true,
            preserves_preview_requirement_vocabulary: true,
            preserves_snippet_session_field_vocabulary: true,
            preserves_code_action_field_vocabulary: true,
            preserves_cross_cut_condition_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(
        lane: AssistantSurfaceLaneClass,
        prefix: &str,
    ) -> Vec<AssistantSurfaceHardeningRow> {
        let mut rows = vec![quality_row(lane, prefix)];
        rows.extend(cross_cut_rows(lane, prefix));
        if lane_requires_provider_source_binding(lane) {
            rows.extend(provider_source_rows(lane, prefix));
        }
        if matches!(lane, AssistantSurfaceLaneClass::SnippetSessionLane) {
            rows.extend(snippet_session_rows(prefix));
        }
        if matches!(lane, AssistantSurfaceLaneClass::CodeActionLane) {
            rows.extend(code_action_rows(prefix));
        }
        if matches!(lane, AssistantSurfaceLaneClass::AdditionalEditLane) {
            rows.push(additional_edit_row(prefix));
        }
        rows
    }

    fn sample_input() -> AssistantSurfaceHardeningTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(AssistantSurfaceLaneClass::CompletionLane, "completion"));
        rows.extend(lane_rows(
            AssistantSurfaceLaneClass::SignatureHelpLane,
            "signature_help",
        ));
        rows.extend(lane_rows(
            AssistantSurfaceLaneClass::SnippetSessionLane,
            "snippet_session",
        ));
        rows.extend(lane_rows(AssistantSurfaceLaneClass::CodeActionLane, "code_action"));
        rows.extend(lane_rows(
            AssistantSurfaceLaneClass::AdditionalEditLane,
            "additional_edit",
        ));
        rows.extend(lane_rows(
            AssistantSurfaceLaneClass::SourceLabelingLane,
            "source_labeling",
        ));
        rows.extend(lane_rows(AssistantSurfaceLaneClass::AiGhostTextLane, "ai_ghost_text"));
        AssistantSurfaceHardeningTruthPacketInput {
            packet_id: "packet:m4:assistant_surface_hardening".to_owned(),
            workflow_or_surface_id: "workflow.language.assistant_surface_hardening".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: AssistantSurfaceLaneClass::REQUIRED.to_vec(),
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
            AssistantSurfaceLaneClass::CompletionLane.as_str(),
            "completion_lane"
        );
        assert_eq!(
            AssistantSurfaceLaneClass::AiGhostTextLane.as_str(),
            "ai_ghost_text_lane"
        );
        assert_eq!(
            AssistantSurfaceRowClass::AssistantSurfaceQuality.as_str(),
            "assistant_surface_quality"
        );
        assert_eq!(SupportClass::LaunchHardened.as_str(), "launch_hardened");
        assert_eq!(SupportClass::SupportUnbound.as_str(), "support_unbound");
        assert_eq!(
            ProviderSourceClass::AiGhostText.as_str(),
            "ai_ghost_text"
        );
        assert_eq!(
            SideEffectClass::ProtectedSurfaceWrite.as_str(),
            "protected_surface_write"
        );
        assert_eq!(
            PreviewRequirementClass::PreviewRequiredForGeneratedFile.as_str(),
            "preview_required_for_generated_file"
        );
        assert_eq!(
            SnippetSessionFieldClass::ExitRoute.as_str(),
            "exit_route"
        );
        assert_eq!(
            CodeActionFieldClass::PartialSupportReason.as_str(),
            "partial_support_reason"
        );
        assert_eq!(
            CrossCutConditionClass::DegradedProvider.as_str(),
            "degraded_provider"
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
            FindingKind::LaunchHardenedWithUnboundBinding.as_str(),
            "launch_hardened_with_unbound_binding"
        );
        assert_eq!(
            FindingKind::MissingCrossCutConditionCoverage.as_str(),
            "missing_cross_cut_condition_coverage"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(sample_input());
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
                "support:m4:assistant_surface_hardening",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_hardened_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LaunchHardenedWithUnboundBinding
        }));
    }

    #[test]
    fn missing_cross_cut_condition_for_launch_hardened_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(row.lane_class == AssistantSurfaceLaneClass::CompletionLane
                && matches!(row.row_class, AssistantSurfaceRowClass::CrossCutCondition)
                && row.cross_cut_condition_class == CrossCutConditionClass::DegradedProvider)
        });
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingCrossCutConditionCoverage
        }));
    }

    #[test]
    fn missing_provider_source_binding_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(row.lane_class == AssistantSurfaceLaneClass::AiGhostTextLane
                && matches!(row.row_class, AssistantSurfaceRowClass::ProviderSourceBinding)
                && row.provider_source_class == ProviderSourceClass::AiGhostText)
        });
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingProviderSourceBindingCoverage
        }));
    }

    #[test]
    fn missing_snippet_session_field_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(row.lane_class == AssistantSurfaceLaneClass::SnippetSessionLane
                && matches!(row.row_class, AssistantSurfaceRowClass::SnippetSessionTruth)
                && row.snippet_session_field_class == SnippetSessionFieldClass::ExitRoute)
        });
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingSnippetSessionFieldCoverage
        }));
    }

    #[test]
    fn missing_code_action_field_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(row.lane_class == AssistantSurfaceLaneClass::CodeActionLane
                && matches!(row.row_class, AssistantSurfaceRowClass::CodeActionTruth)
                && row.code_action_field_class == CodeActionFieldClass::PreviewRequirementField)
        });
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingCodeActionFieldCoverage
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchHardenedBelow;
        input.rows[0].disclosure_ref = None;
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
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
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_provider_source_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_provider_source_class_vocabulary = false;
            }
        }
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::ProviderSourceClassVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = AssistantSurfaceHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
