//! One reusable M5 review primitive — the test-generation suggestion card — so a reviewer sees
//! *why* a test was proposed, *what* it assumes, and *which* classes of change it would apply
//! **before** any apply-capable action is offered. A test-generation suggestion card always names
//! its trigger source (an uncovered line or branch, a failing bug repro, a regression-guard gap, a
//! missing assertion, or a manual request), the target symbols / files it covers, the uncovered
//! path or bug context behind it, its generated-test assumption summary, the distinct review
//! classes it separates its churn into (assertion changes, helper / fixture additions, and
//! snapshot / golden updates), its generated file count, and its sandbox-run / open-diff / apply /
//! rollback actions.
//!
//! Aureline's frozen test-intelligence component matrix
//! ([`crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix`])
//! names the test-generation suggestion card as one governed component family and freezes its
//! controlled vocabulary — the generated-test assumption classes and the generated-test apply
//! scopes, plus the provenance classes, surface families, deployment lines, consumer surfaces,
//! accessibility routes, qualification classes, and downgrade triggers. This module *implements*
//! that contract as one reusable resolver so a user can tell — from the card alone — what triggered
//! the suggestion, what it assumed, how many files it generated, and exactly which classes of
//! change an apply would touch. Above all, an apply-capable action never understates its churn: a
//! snapshot / golden update or a helper / fixture addition can never be applied through an
//! assertion-only click, and every apply-capable proposal keeps the same diff-first preview,
//! rollback, and evidence rules as an ordinary multi-file mutation flow.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_test_generation_suggestion_card`] — takes one card's trigger source, target
//!    symbol / file refs, uncovered-path / bug context ref, generated-test assumption classes,
//!    the review classes it separates its churn into, its apply scope, its generated file count,
//!    provenance class, whether it offers a sandbox run / a diff-first preview / a rollback, and an
//!    opaque card identity, and produces one [`M5ResolvedSuggestionCard`] carrying the derived
//!    suggestion posture (an assertion-only, fixture-and-assertion, snapshot-included, full-bundle,
//!    review-required, or apply-blocked suggestion — one distinct posture per apply scope), whether
//!    the suggestion is apply-capable, whether it discloses its assumption summary, whether it
//!    preserves preview / rollback parity, and the bounded reveal / run-in-sandbox / open-diff /
//!    apply-reviewed-classes / rollback / export actions. It refuses to resolve an apply-capable
//!    scope that understates its review classes, refuses an apply-capable proposal that omits a
//!    diff-first preview or a rollback, and refuses an apply-capable generated card that hides its
//!    assumption summary, so a generated-test review can never collapse to one opaque diff path.
//!
//! A single parity matrix — [`M5SuggestionCardComponentsPacket`] — binds one row per claimed M5 AI
//! test-review consumer (the suggestion review panel, the editor inline suggestion, the test-tree
//! suggestion, the headless / CLI suggestion, and the suggestion export) to the shared card
//! anatomy, the same trigger sources, review classes, assumption classes, apply scopes, suggestion
//! postures, bounded actions, export fields, and non-visual accessibility routes, so the
//! suggestion-card vocabulary stays identical across the review panel, the editor, the test tree,
//! CI / headless, and support consumers — the acceptance-criterion parity that keeps AI-assisted
//! test suggestions review-first and assumption-visible everywhere with one vocabulary.
//!
//! The generated-test assumption class ([`M5GeneratedAssumptionClass`]), generated-test apply scope
//! ([`M5GeneratedApplyScope`]), provenance class ([`M5TestIntelligenceProvenanceClass`]), surface
//! family ([`M5TestIntelligenceSurfaceFamily`]), deployment line
//! ([`M5TestIntelligenceDeploymentLine`]), consumer surface ([`M5TestIntelligenceConsumerSurface`]),
//! accessibility route ([`M5TestIntelligenceAccessibilityRoute`]), qualification class
//! ([`M5TestIntelligenceQualificationClass`]), and downgrade trigger
//! ([`M5TestIntelligenceDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the component itself: its
//! review consumers, the trigger source, the review-class separation, the derived posture, the
//! bounded action set, the anatomy, and the export field set. No M5 AI test-review surface invents
//! a second suggestion-card grammar.
//!
//! Raw generated source, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every card identity, target ref, and context ref is carried only as an opaque,
//! export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_suggestion_card_components_editor_suggestion_inline_beta_narrowed,
    seeded_m5_suggestion_card_components_packet,
    seeded_m5_suggestion_card_components_suggestion_review_panel_preview_narrowed,
    M5_SUGGESTION_CARD_COMPONENTS_PACKET_ID,
};

// The generated-test assumption class, generated-test apply scope, provenance class, surface
// family, deployment line, consumer surface, accessibility route, qualification class, and
// downgrade triggers are frozen once, in the test-intelligence component matrix. This primitive
// reuses them verbatim so it never invents parallel generated-test vocabulary.
pub use crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix::{
    M5GeneratedApplyScope, M5GeneratedAssumptionClass, M5TestIntelligenceAccessibilityRoute,
    M5TestIntelligenceConsumerSurface, M5TestIntelligenceDeploymentLine,
    M5TestIntelligenceDowngradeTrigger, M5TestIntelligenceProvenanceClass,
    M5TestIntelligenceQualificationClass, M5TestIntelligenceSurfaceFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SuggestionCardComponentsPacket`].
pub const M5_SUGGESTION_CARD_COMPONENTS_RECORD_KIND: &str =
    "implement_m5_test_generation_suggestion_cards_with_uncovered_path_or_bug_trigger_truth_assumption_summaries_helper_fixture_snapshot_separation_sandbox_validation_and_diff_first_apply_parity_across_claimed_m5_ai_test_flows";

/// Schema version for M5 test-generation-suggestion-card records.
pub const M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the test-generation-suggestion-card boundary schema (the canonical packet
/// schema).
pub const M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_REF: &str =
    "schemas/ui/m5-test-generation-suggestion-card.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SUGGESTION_CARD_COMPONENTS_DOC_REF: &str =
    "docs/testing/m5_test_generation_suggestion_card_primitive.md";

/// Repo-relative path of the frozen test-intelligence component matrix this primitive narrows from.
pub const M5_SUGGESTION_CARD_COMPONENTS_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-intelligence-component-matrix.schema.json";

/// Repo-relative path of the test-generation-suggestion / diff-first-apply contract the card binds
/// its trigger / assumption / apply truth against.
pub const M5_SUGGESTION_CARD_COMPONENTS_TEST_GENERATION_REF: &str =
    "schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SUGGESTION_CARD_COMPONENTS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-test-generation-suggestion-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SUGGESTION_CARD_COMPONENTS_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-generation-suggestion-card-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SUGGESTION_CARD_COMPONENTS_CSV_REF: &str =
    "artifacts/release/m5-test-generation-suggestion-card-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SUGGESTION_CARD_COMPONENTS_REPORT_REF: &str =
    "artifacts/design/m5-test-generation-suggestion-card-primitive.md";

/// One claimed M5 AI test-review consumer that renders the shared test-generation suggestion card.
/// These are the consumers the acceptance criteria name — the suggestion review panel, the editor
/// inline suggestion, the test-tree suggestion, the headless / CLI suggestion, and the suggestion
/// export — so the same suggestion grammar works across every claimed AI test-review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuggestionCardConsumerSurface {
    /// The test-generation suggestion review panel surface.
    SuggestionReviewPanel,
    /// The editor inline-suggestion surface.
    EditorSuggestionInline,
    /// The test-tree suggestion surface.
    TestTreeSuggestion,
    /// The headless / CLI suggestion surface.
    HeadlessCliSuggestion,
    /// The suggestion export surface.
    SuggestionExport,
}

impl M5SuggestionCardConsumerSurface {
    /// Every claimed AI test-review consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SuggestionReviewPanel,
        Self::EditorSuggestionInline,
        Self::TestTreeSuggestion,
        Self::HeadlessCliSuggestion,
        Self::SuggestionExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuggestionReviewPanel => "suggestion_review_panel",
            Self::EditorSuggestionInline => "editor_suggestion_inline",
            Self::TestTreeSuggestion => "test_tree_suggestion",
            Self::HeadlessCliSuggestion => "headless_cli_suggestion",
            Self::SuggestionExport => "suggestion_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SuggestionReviewPanel => "Suggestion Review Panel",
            Self::EditorSuggestionInline => "Editor Inline Suggestion",
            Self::TestTreeSuggestion => "Test-Tree Suggestion",
            Self::HeadlessCliSuggestion => "Headless / CLI Suggestion",
            Self::SuggestionExport => "Suggestion Export",
        }
    }
}

/// Controlled generation trigger source — *why* a test-generation suggestion was proposed, so a
/// suggestion never hides the uncovered path or the bug context behind it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GenerationTriggerSource {
    /// An uncovered line drove the suggestion.
    UncoveredLine,
    /// An uncovered branch drove the suggestion.
    UncoveredBranch,
    /// A failing bug repro drove the suggestion.
    FailingBugRepro,
    /// A regression-guard gap drove the suggestion.
    RegressionGuardGap,
    /// A missing-assertion gap drove the suggestion.
    MissingAssertionGap,
    /// A manual request drove the suggestion.
    ManualRequest,
}

impl M5GenerationTriggerSource {
    /// Every trigger source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UncoveredLine,
        Self::UncoveredBranch,
        Self::FailingBugRepro,
        Self::RegressionGuardGap,
        Self::MissingAssertionGap,
        Self::ManualRequest,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UncoveredLine => "uncovered_line",
            Self::UncoveredBranch => "uncovered_branch",
            Self::FailingBugRepro => "failing_bug_repro",
            Self::RegressionGuardGap => "regression_guard_gap",
            Self::MissingAssertionGap => "missing_assertion_gap",
            Self::ManualRequest => "manual_request",
        }
    }

    /// True when the suggestion was driven by an uncovered path or a bug repro, so the card must
    /// name that uncovered-path / bug context.
    pub const fn is_uncovered_path_or_bug(self) -> bool {
        matches!(
            self,
            Self::UncoveredLine | Self::UncoveredBranch | Self::FailingBugRepro
        )
    }
}

/// Controlled review class — the distinct classes of change a test-generation suggestion separates
/// its churn into, so assertion changes, helper / fixture additions, and snapshot / golden updates
/// are never bundled into one opaque apply path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeneratedReviewClass {
    /// An assertion change.
    AssertionChange,
    /// A helper or fixture addition.
    HelperOrFixtureAddition,
    /// A snapshot or golden update.
    SnapshotOrGoldenUpdate,
}

impl M5GeneratedReviewClass {
    /// Every review class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::AssertionChange,
        Self::HelperOrFixtureAddition,
        Self::SnapshotOrGoldenUpdate,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssertionChange => "assertion_change",
            Self::HelperOrFixtureAddition => "helper_or_fixture_addition",
            Self::SnapshotOrGoldenUpdate => "snapshot_or_golden_update",
        }
    }

    /// True when the class is a non-assertion churn (a helper / fixture addition or a snapshot /
    /// golden update) that an apply-capable scope must name rather than hide behind assertions.
    pub const fn is_non_assertion_churn(self) -> bool {
        !matches!(self, Self::AssertionChange)
    }
}

/// The derived posture of a test-generation suggestion card — one distinct posture per apply scope,
/// so a full-bundle proposal never reads as an assertion-only apply. Computed 1:1 from the apply
/// scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuggestionPosture {
    /// An assertion-only suggestion.
    AssertionOnlySuggestion,
    /// A fixture-and-assertion suggestion.
    FixtureAndAssertionSuggestion,
    /// A snapshot-included suggestion.
    SnapshotIncludedSuggestion,
    /// A full-bundle suggestion (held to review-first, never one-click apply).
    FullBundleSuggestion,
    /// A review-required suggestion.
    ReviewRequiredSuggestion,
    /// An apply-blocked suggestion.
    ApplyBlockedSuggestion,
}

impl M5SuggestionPosture {
    /// Every suggestion posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AssertionOnlySuggestion,
        Self::FixtureAndAssertionSuggestion,
        Self::SnapshotIncludedSuggestion,
        Self::FullBundleSuggestion,
        Self::ReviewRequiredSuggestion,
        Self::ApplyBlockedSuggestion,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssertionOnlySuggestion => "assertion_only_suggestion",
            Self::FixtureAndAssertionSuggestion => "fixture_and_assertion_suggestion",
            Self::SnapshotIncludedSuggestion => "snapshot_included_suggestion",
            Self::FullBundleSuggestion => "full_bundle_suggestion",
            Self::ReviewRequiredSuggestion => "review_required_suggestion",
            Self::ApplyBlockedSuggestion => "apply_blocked_suggestion",
        }
    }

    /// The frozen apply scope this posture maps 1:1 to.
    pub const fn apply_scope(self) -> M5GeneratedApplyScope {
        match self {
            Self::AssertionOnlySuggestion => M5GeneratedApplyScope::AssertionOnly,
            Self::FixtureAndAssertionSuggestion => M5GeneratedApplyScope::FixtureAndAssertion,
            Self::SnapshotIncludedSuggestion => M5GeneratedApplyScope::SnapshotIncluded,
            Self::FullBundleSuggestion => M5GeneratedApplyScope::FullBundleApply,
            Self::ReviewRequiredSuggestion => M5GeneratedApplyScope::ReviewRequired,
            Self::ApplyBlockedSuggestion => M5GeneratedApplyScope::ApplyBlocked,
        }
    }

    /// True when this posture may offer an apply-capable action from the card. A full bundle,
    /// review-required, or apply-blocked suggestion is never one-click apply — it is always held to
    /// a review-first path.
    pub const fn is_apply_capable(self) -> bool {
        matches!(
            self,
            Self::AssertionOnlySuggestion
                | Self::FixtureAndAssertionSuggestion
                | Self::SnapshotIncludedSuggestion
        )
    }

    /// True when this posture flags a state a reviewer should act on before trusting a one-click
    /// apply.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::SnapshotIncludedSuggestion
                | Self::FullBundleSuggestion
                | Self::ReviewRequiredSuggestion
                | Self::ApplyBlockedSuggestion
        )
    }
}

/// The set of review classes an apply-capable scope is allowed to touch, so a snapshot / golden
/// update or a helper / fixture addition can never be applied through a narrower click. A
/// non-apply-capable scope (a full bundle, a review-required, or an apply-blocked proposal) never
/// offers a one-click apply, so it is held to a review-first path regardless of its churn.
fn apply_scope_allows(scope: M5GeneratedApplyScope, class: M5GeneratedReviewClass) -> bool {
    use M5GeneratedReviewClass as Class;
    match scope {
        M5GeneratedApplyScope::AssertionOnly => matches!(class, Class::AssertionChange),
        M5GeneratedApplyScope::FixtureAndAssertion => {
            matches!(
                class,
                Class::AssertionChange | Class::HelperOrFixtureAddition
            )
        }
        M5GeneratedApplyScope::SnapshotIncluded => matches!(
            class,
            Class::AssertionChange | Class::HelperOrFixtureAddition | Class::SnapshotOrGoldenUpdate
        ),
        // Non-apply-capable scopes never offer a one-click apply, so they impose no per-class
        // ceiling — the reviewer must open the diff and apply per class.
        M5GeneratedApplyScope::FullBundleApply
        | M5GeneratedApplyScope::ReviewRequired
        | M5GeneratedApplyScope::ApplyBlocked => true,
    }
}

/// One bounded action a test-generation suggestion card offers, so a card never hides its
/// reveal / sandbox-run / open-diff / apply / rollback / export affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuggestionCardAction {
    /// Reveal the card's trigger source, targets, context, assumption summary, review classes, and
    /// generated file count.
    RevealSuggestionDetails,
    /// Run the generated test in a sandbox before applying.
    RunInSandbox,
    /// Open the diff-first preview.
    OpenDiffPreview,
    /// Apply the separately reviewed classes.
    ApplyReviewedClasses,
    /// Roll back a previously applied suggestion.
    RollbackApplied,
    /// Export the suggestion as test evidence.
    ExportSuggestion,
}

impl M5SuggestionCardAction {
    /// Every suggestion action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RevealSuggestionDetails,
        Self::RunInSandbox,
        Self::OpenDiffPreview,
        Self::ApplyReviewedClasses,
        Self::RollbackApplied,
        Self::ExportSuggestion,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealSuggestionDetails => "reveal_suggestion_details",
            Self::RunInSandbox => "run_in_sandbox",
            Self::OpenDiffPreview => "open_diff_preview",
            Self::ApplyReviewedClasses => "apply_reviewed_classes",
            Self::RollbackApplied => "rollback_applied",
            Self::ExportSuggestion => "export_suggestion",
        }
    }
}

/// Controlled suggestion-card anatomy part. The parts in [`M5SuggestionCardAnatomyPart::MANDATORY`]
/// are required on every card so the trigger source, target symbols, uncovered-path / bug context,
/// assumption summary, review-class separation, and generated file count are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuggestionCardAnatomyPart {
    /// The trigger-source cue.
    TriggerSourceCue,
    /// The target-symbols cue.
    TargetSymbolsCue,
    /// The uncovered-path / bug context cue.
    TriggerContextCue,
    /// The assumption-summary cue.
    AssumptionSummaryCue,
    /// The review-class separation cue.
    ReviewClassCue,
    /// The generated-file-count cue.
    GeneratedFileCountCue,
    /// The apply-scope cue.
    ApplyScopeCue,
    /// The provenance cue.
    ProvenanceCue,
}

impl M5SuggestionCardAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TriggerSourceCue,
        Self::TargetSymbolsCue,
        Self::TriggerContextCue,
        Self::AssumptionSummaryCue,
        Self::ReviewClassCue,
        Self::GeneratedFileCountCue,
        Self::ApplyScopeCue,
        Self::ProvenanceCue,
    ];

    /// The anatomy parts every suggestion card must render.
    pub const MANDATORY: [Self; 6] = [
        Self::TriggerSourceCue,
        Self::TargetSymbolsCue,
        Self::TriggerContextCue,
        Self::AssumptionSummaryCue,
        Self::ReviewClassCue,
        Self::GeneratedFileCountCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TriggerSourceCue => "trigger_source_cue",
            Self::TargetSymbolsCue => "target_symbols_cue",
            Self::TriggerContextCue => "trigger_context_cue",
            Self::AssumptionSummaryCue => "assumption_summary_cue",
            Self::ReviewClassCue => "review_class_cue",
            Self::GeneratedFileCountCue => "generated_file_count_cue",
            Self::ApplyScopeCue => "apply_scope_cue",
            Self::ProvenanceCue => "provenance_cue",
        }
    }
}

/// A field the suggestion-card export carries so card truth is reconstructable. The fields in
/// [`M5SuggestionCardExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuggestionCardExportField {
    /// The trigger source.
    TriggerSource,
    /// The target refs.
    TargetRefs,
    /// The uncovered-path / bug context.
    TriggerContext,
    /// The generated-test assumption classes.
    AssumptionClasses,
    /// The separated review classes.
    ReviewClasses,
    /// The apply scope.
    ApplyScope,
    /// The generated file count.
    GeneratedFileCount,
    /// The derived suggestion posture.
    SuggestionPosture,
    /// The available actions.
    AvailableActions,
}

impl M5SuggestionCardExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TriggerSource,
        Self::TargetRefs,
        Self::TriggerContext,
        Self::AssumptionClasses,
        Self::ReviewClasses,
        Self::ApplyScope,
        Self::GeneratedFileCount,
        Self::SuggestionPosture,
        Self::AvailableActions,
    ];

    /// The export fields every suggestion card must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::TriggerSource,
        Self::TargetRefs,
        Self::AssumptionClasses,
        Self::ReviewClasses,
        Self::ApplyScope,
        Self::SuggestionPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TriggerSource => "trigger_source",
            Self::TargetRefs => "target_refs",
            Self::TriggerContext => "trigger_context",
            Self::AssumptionClasses => "assumption_classes",
            Self::ReviewClasses => "review_classes",
            Self::ApplyScope => "apply_scope",
            Self::GeneratedFileCount => "generated_file_count",
            Self::SuggestionPosture => "suggestion_posture",
            Self::AvailableActions => "available_actions",
        }
    }
}

// ---- test-generation-suggestion-card resolver ----------------------------

/// The full input to the test-generation-suggestion-card resolver for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardResolutionInput {
    /// The trigger source behind the suggestion.
    pub trigger_source: M5GenerationTriggerSource,
    /// The target symbol / file refs the suggestion covers (opaque; must be non-empty).
    pub target_refs: Vec<String>,
    /// The opaque uncovered-path / bug context ref (must be non-empty).
    pub trigger_context_ref: String,
    /// The generated-test assumption classes the suggestion discloses.
    pub assumption_classes: Vec<M5GeneratedAssumptionClass>,
    /// The distinct review classes the suggestion separates its churn into (must be non-empty).
    pub review_classes: Vec<M5GeneratedReviewClass>,
    /// The apply scope the suggestion would apply.
    pub apply_scope: M5GeneratedApplyScope,
    /// The number of files the suggestion would generate.
    pub generated_file_count: u32,
    /// The provenance class behind the suggestion.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// Whether the card offers a sandbox run before applying.
    pub offers_sandbox_run: bool,
    /// Whether the card offers a diff-first preview.
    pub offers_diff_preview: bool,
    /// Whether the card offers a rollback of an applied suggestion.
    pub offers_rollback: bool,
    /// The opaque stable card identity (must be non-empty).
    pub suggestion_identity_ref: String,
}

/// The resolved test-generation-suggestion-card truth for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSuggestionCard {
    /// The trigger source.
    pub trigger_source: M5GenerationTriggerSource,
    /// The target refs, preserved exactly from the input.
    pub target_refs: Vec<String>,
    /// The uncovered-path / bug context ref, preserved exactly from the input.
    pub trigger_context_ref: String,
    /// The assumption classes, preserved exactly from the input.
    pub assumption_classes: Vec<M5GeneratedAssumptionClass>,
    /// The review classes, preserved exactly from the input.
    pub review_classes: Vec<M5GeneratedReviewClass>,
    /// The apply scope.
    pub apply_scope: M5GeneratedApplyScope,
    /// The generated file count, preserved from the input.
    pub generated_file_count: u32,
    /// The provenance class.
    pub provenance_class: M5TestIntelligenceProvenanceClass,
    /// The opaque card identity, preserved exactly from the input.
    pub suggestion_identity_ref: String,
    /// The derived suggestion posture.
    pub suggestion_posture: M5SuggestionPosture,
    /// The bounded actions this card offers.
    pub available_actions: Vec<M5SuggestionCardAction>,
    /// True when the card may offer a one-click apply from the card.
    pub is_apply_capable: bool,
    /// True when the suggestion was driven by an uncovered path or a bug repro.
    pub is_uncovered_path_or_bug: bool,
    /// True when the review classes carry non-assertion churn (a helper / fixture addition or a
    /// snapshot / golden update).
    pub bundles_non_assertion_churn: bool,
    /// True when the apply scope names every review class it would apply (always true after
    /// resolution — an apply-capable scope that understates its churn fails resolution).
    pub apply_scope_names_every_class: bool,
    /// True when the card discloses its generated-test assumption summary whenever an apply-capable
    /// card generates files (always true after resolution — an apply-capable generated card that
    /// hides its assumptions fails resolution).
    pub discloses_assumption_summary: bool,
    /// True when the card offers a sandbox run.
    pub offers_sandbox_run: bool,
    /// True when the card offers a diff-first preview.
    pub offers_diff_preview: bool,
    /// True when the card offers a rollback.
    pub offers_rollback: bool,
    /// True when the card preserves diff-first preview and rollback parity for every apply-capable
    /// proposal (always true after resolution — an apply-capable card that drops preview or
    /// rollback fails resolution).
    pub preserves_preview_and_rollback: bool,
    /// True when the card flags a state a reviewer should act on before a one-click apply.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_test_generation_suggestion_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SuggestionCardResolutionError {
    /// The suggestion identity ref was empty.
    EmptySuggestionIdentity,
    /// The target refs were empty — the card would not name what it covers.
    EmptyTargetReference,
    /// The uncovered-path / bug context ref was empty.
    EmptyTriggerContext,
    /// The review classes were empty — the card would not separate its churn.
    MissingReviewClasses,
    /// An apply-capable generated card disclosed no assumption summary — the assumptions would be
    /// hidden behind the generated assertions.
    GeneratedWithoutAssumptionSummary,
    /// An apply-capable scope understated its review classes — a helper / fixture addition or a
    /// snapshot / golden update would be applied through a narrower click.
    ApplyScopeUnderstatesReviewClasses,
    /// An apply-capable proposal omitted a diff-first preview or a rollback — it would not match
    /// the preview / rollback rules of an ordinary multi-file mutation flow.
    ApplyWithoutDiffPreviewOrRollback,
    /// A card descriptor carried forbidden material.
    ForbiddenSuggestionMaterial,
}

impl M5SuggestionCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySuggestionIdentity => "empty_suggestion_identity",
            Self::EmptyTargetReference => "empty_target_reference",
            Self::EmptyTriggerContext => "empty_trigger_context",
            Self::MissingReviewClasses => "missing_review_classes",
            Self::GeneratedWithoutAssumptionSummary => "generated_without_assumption_summary",
            Self::ApplyScopeUnderstatesReviewClasses => "apply_scope_understates_review_classes",
            Self::ApplyWithoutDiffPreviewOrRollback => "apply_without_diff_preview_or_rollback",
            Self::ForbiddenSuggestionMaterial => "forbidden_suggestion_material",
        }
    }
}

impl fmt::Display for M5SuggestionCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "test generation suggestion card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SuggestionCardResolutionError {}

/// Resolves one test-generation suggestion card from its declared trigger and apply state.
///
/// The derived suggestion posture is 1:1 with the apply scope — assertion-only, fixture-and-
/// assertion, snapshot-included, full-bundle, review-required, or apply-blocked — so a full-bundle
/// proposal never reads as an assertion-only apply. An **apply-capable scope may only be offered
/// when it names every review class it would apply**; otherwise resolution fails with
/// `ApplyScopeUnderstatesReviewClasses`, so a snapshot / golden update or a helper / fixture
/// addition can never be applied through a narrower click and assertion, fixture, and snapshot
/// churn are always separated before any apply-capable action. An **apply-capable generated card
/// must disclose its assumption summary**; otherwise resolution fails with
/// `GeneratedWithoutAssumptionSummary`, so generated assumptions are never hidden. An
/// **apply-capable proposal must keep a diff-first preview and a rollback**; otherwise resolution
/// fails with `ApplyWithoutDiffPreviewOrRollback`, so an AI-assisted proposal preserves the same
/// preview, rollback, and evidence rules as an ordinary multi-file mutation flow. The trigger
/// source, targets, context, assumption summary, review classes, and generated file count are
/// always carried.
pub fn resolve_test_generation_suggestion_card(
    input: &M5SuggestionCardResolutionInput,
) -> Result<M5ResolvedSuggestionCard, M5SuggestionCardResolutionError> {
    if input.suggestion_identity_ref.trim().is_empty() {
        return Err(M5SuggestionCardResolutionError::EmptySuggestionIdentity);
    }
    if input.target_refs.is_empty() || input.target_refs.iter().all(|r| r.trim().is_empty()) {
        return Err(M5SuggestionCardResolutionError::EmptyTargetReference);
    }
    if input.trigger_context_ref.trim().is_empty() {
        return Err(M5SuggestionCardResolutionError::EmptyTriggerContext);
    }
    if input.review_classes.is_empty() {
        return Err(M5SuggestionCardResolutionError::MissingReviewClasses);
    }
    if value_repr_is_forbidden(&input.suggestion_identity_ref)
        || value_repr_is_forbidden(&input.trigger_context_ref)
        || input
            .target_refs
            .iter()
            .any(|target| value_repr_is_forbidden(target))
    {
        return Err(M5SuggestionCardResolutionError::ForbiddenSuggestionMaterial);
    }

    let suggestion_posture = derive_suggestion_posture(input.apply_scope);
    let is_apply_capable = suggestion_posture.is_apply_capable();

    if is_apply_capable
        && !input
            .review_classes
            .iter()
            .all(|class| apply_scope_allows(input.apply_scope, *class))
    {
        return Err(M5SuggestionCardResolutionError::ApplyScopeUnderstatesReviewClasses);
    }
    if is_apply_capable && input.generated_file_count > 0 && input.assumption_classes.is_empty() {
        return Err(M5SuggestionCardResolutionError::GeneratedWithoutAssumptionSummary);
    }
    if is_apply_capable && !(input.offers_diff_preview && input.offers_rollback) {
        return Err(M5SuggestionCardResolutionError::ApplyWithoutDiffPreviewOrRollback);
    }

    let bundles_non_assertion_churn = input
        .review_classes
        .iter()
        .any(|class| class.is_non_assertion_churn());
    let discloses_assumption_summary =
        !input.assumption_classes.is_empty() || input.generated_file_count == 0;
    let available_actions = derive_suggestion_actions(
        is_apply_capable,
        input.offers_sandbox_run,
        input.offers_diff_preview,
        input.offers_rollback,
    );

    Ok(M5ResolvedSuggestionCard {
        trigger_source: input.trigger_source,
        target_refs: input.target_refs.clone(),
        trigger_context_ref: input.trigger_context_ref.clone(),
        assumption_classes: input.assumption_classes.clone(),
        review_classes: input.review_classes.clone(),
        apply_scope: input.apply_scope,
        generated_file_count: input.generated_file_count,
        provenance_class: input.provenance_class,
        suggestion_identity_ref: input.suggestion_identity_ref.clone(),
        suggestion_posture,
        available_actions,
        is_apply_capable,
        is_uncovered_path_or_bug: input.trigger_source.is_uncovered_path_or_bug(),
        bundles_non_assertion_churn,
        apply_scope_names_every_class: true,
        discloses_assumption_summary,
        offers_sandbox_run: input.offers_sandbox_run,
        offers_diff_preview: input.offers_diff_preview,
        offers_rollback: input.offers_rollback,
        preserves_preview_and_rollback: !is_apply_capable
            || (input.offers_diff_preview && input.offers_rollback),
        needs_attention: suggestion_posture.needs_attention(),
    })
}

/// The 1:1 apply-scope → suggestion-posture map.
fn derive_suggestion_posture(scope: M5GeneratedApplyScope) -> M5SuggestionPosture {
    match scope {
        M5GeneratedApplyScope::AssertionOnly => M5SuggestionPosture::AssertionOnlySuggestion,
        M5GeneratedApplyScope::FixtureAndAssertion => {
            M5SuggestionPosture::FixtureAndAssertionSuggestion
        }
        M5GeneratedApplyScope::SnapshotIncluded => M5SuggestionPosture::SnapshotIncludedSuggestion,
        M5GeneratedApplyScope::FullBundleApply => M5SuggestionPosture::FullBundleSuggestion,
        M5GeneratedApplyScope::ReviewRequired => M5SuggestionPosture::ReviewRequiredSuggestion,
        M5GeneratedApplyScope::ApplyBlocked => M5SuggestionPosture::ApplyBlockedSuggestion,
    }
}

/// Derives the bounded suggestion-action set. Reveal and export are always offered; the sandbox,
/// diff-first preview, apply-reviewed-classes, and rollback actions are offered only when the card
/// declares them and (for apply) is apply-capable.
fn derive_suggestion_actions(
    is_apply_capable: bool,
    offers_sandbox_run: bool,
    offers_diff_preview: bool,
    offers_rollback: bool,
) -> Vec<M5SuggestionCardAction> {
    use M5SuggestionCardAction as Action;
    let mut actions = vec![Action::RevealSuggestionDetails];
    if offers_sandbox_run {
        actions.push(Action::RunInSandbox);
    }
    if offers_diff_preview {
        actions.push(Action::OpenDiffPreview);
    }
    if is_apply_capable {
        actions.push(Action::ApplyReviewedClasses);
    }
    if offers_rollback {
        actions.push(Action::RollbackApplied);
    }
    actions.push(Action::ExportSuggestion);
    actions
}

// ---- worked cases --------------------------------------------------------

/// One worked test-generation-suggestion-card resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardResolutionCase {
    /// The resolver input.
    pub input: M5SuggestionCardResolutionInput,
    /// The resolved truth. Must equal `resolve_test_generation_suggestion_card(&input)`.
    pub resolved: M5ResolvedSuggestionCard,
}

impl M5SuggestionCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SuggestionCardResolutionInput) -> Self {
        let resolved = resolve_test_generation_suggestion_card(&input)
            .expect("seed suggestion card case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_test_generation_suggestion_card(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved card preserves the input identity, targets, review classes, and
    /// assumptions exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.suggestion_identity_ref == self.input.suggestion_identity_ref
            && self.resolved.target_refs == self.input.target_refs
            && self.resolved.review_classes == self.input.review_classes
            && self.resolved.assumption_classes == self.input.assumption_classes
    }
}

/// One row in the primitive matrix: one AI test-review consumer bound to the shared card anatomy,
/// trigger sources, review classes, assumption classes, apply scopes, suggestion postures, bounded
/// actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardConsumerRow {
    /// AI test-review consumer family.
    pub consumer_surface: M5SuggestionCardConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TestIntelligenceQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 test surface families that render / consume this component.
    pub surface_families: Vec<M5TestIntelligenceSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5TestIntelligenceDeploymentLine>,
    /// Suggestion-card anatomy parts this consumer renders (must include the mandatory parts).
    pub suggestion_anatomy_parts: Vec<M5SuggestionCardAnatomyPart>,
    /// Trigger sources this consumer distinguishes.
    pub trigger_sources: Vec<M5GenerationTriggerSource>,
    /// Review classes this consumer separates its churn into.
    pub review_classes: Vec<M5GeneratedReviewClass>,
    /// Generated-test assumption classes this consumer distinguishes.
    pub assumption_classes: Vec<M5GeneratedAssumptionClass>,
    /// Apply scopes this consumer distinguishes.
    pub apply_scopes: Vec<M5GeneratedApplyScope>,
    /// Suggestion postures this consumer distinguishes.
    pub suggestion_postures: Vec<M5SuggestionPosture>,
    /// Provenance classes this consumer distinguishes.
    pub provenance_classes: Vec<M5TestIntelligenceProvenanceClass>,
    /// Bounded suggestion actions this consumer offers.
    pub suggestion_actions: Vec<M5SuggestionCardAction>,
    /// Suggestion export fields this consumer carries (must include the mandatory fields).
    pub suggestion_export_fields: Vec<M5SuggestionCardExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TestIntelligenceAccessibilityRoute>,
    /// Test-intelligence subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TestIntelligenceConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TestIntelligenceDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Worked suggestion-card resolutions proving the resolver on this consumer.
    pub suggestion_examples: Vec<M5SuggestionCardResolutionCase>,
    /// Hard invariant: this consumer never bundles assumption, fixture, or snapshot churn into one
    /// opaque apply path. MUST be `false`.
    pub bundles_assumption_fixture_or_snapshot_into_opaque_apply: bool,
    /// Hard invariant: this consumer never hides the trigger source or the target symbols behind a
    /// bare suggestion. MUST be `false`.
    pub hides_trigger_source_or_target_symbols: bool,
    /// Hard invariant: this consumer never hides the assumption summary or the generated file
    /// count. MUST be `false`.
    pub hides_assumption_summary_or_generated_file_count: bool,
    /// Hard invariant: this consumer never invents an alternate label for a governed suggestion or
    /// apply state. MUST be `false`.
    pub invents_alternate_suggestion_or_apply_state_label: bool,
}

impl M5SuggestionCardConsumerRow {
    /// True when the row declares every mandatory suggestion anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SuggestionCardAnatomyPart> =
            self.suggestion_anatomy_parts.iter().copied().collect();
        M5SuggestionCardAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory suggestion export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5SuggestionCardExportField> =
            self.suggestion_export_fields.iter().copied().collect();
        M5SuggestionCardExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.bundles_assumption_fixture_or_snapshot_into_opaque_apply
            && !self.hides_trigger_source_or_target_symbols
            && !self.hides_assumption_summary_or_generated_file_count
            && !self.invents_alternate_suggestion_or_apply_state_label
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardVocabularySet {
    /// AI test-review consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Suggestion-anatomy-part tokens.
    pub suggestion_anatomy_parts: Vec<String>,
    /// Trigger-source tokens.
    pub trigger_sources: Vec<String>,
    /// Review-class tokens.
    pub review_classes: Vec<String>,
    /// Suggestion-posture tokens.
    pub suggestion_postures: Vec<String>,
    /// Suggestion-action tokens.
    pub suggestion_actions: Vec<String>,
    /// Suggestion-export-field tokens.
    pub suggestion_export_fields: Vec<String>,
    /// Generated-test assumption-class tokens (reused from the frozen matrix).
    pub assumption_classes: Vec<String>,
    /// Generated-test apply-scope tokens (reused from the frozen matrix).
    pub apply_scopes: Vec<String>,
    /// Provenance-class tokens (reused from the frozen matrix).
    pub provenance_classes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5SuggestionCardVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5SuggestionCardConsumerSurface::ALL, |v| v.as_str()),
            suggestion_anatomy_parts: tokens(&M5SuggestionCardAnatomyPart::ALL, |v| v.as_str()),
            trigger_sources: tokens(&M5GenerationTriggerSource::ALL, |v| v.as_str()),
            review_classes: tokens(&M5GeneratedReviewClass::ALL, |v| v.as_str()),
            suggestion_postures: tokens(&M5SuggestionPosture::ALL, |v| v.as_str()),
            suggestion_actions: tokens(&M5SuggestionCardAction::ALL, |v| v.as_str()),
            suggestion_export_fields: tokens(&M5SuggestionCardExportField::ALL, |v| v.as_str()),
            assumption_classes: tokens(&M5GeneratedAssumptionClass::ALL, |v| v.as_str()),
            apply_scopes: tokens(&M5GeneratedApplyScope::ALL, |v| v.as_str()),
            provenance_classes: tokens(&M5TestIntelligenceProvenanceClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TestIntelligenceSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TestIntelligenceDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TestIntelligenceAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardGovernanceReview {
    /// The suggestion card shows its trigger source and its target symbols.
    pub card_shows_trigger_source_and_targets: bool,
    /// The suggestion card shows its uncovered-path / bug context.
    pub card_shows_trigger_context: bool,
    /// The suggestion card shows its assumption summary.
    pub card_shows_assumption_summary: bool,
    /// The suggestion card separates its churn into distinct review classes.
    pub card_separates_review_classes: bool,
    /// The suggestion card shows its generated file count.
    pub card_shows_generated_file_count: bool,
    /// The suggestion card offers a sandbox run and an open-diff preview.
    pub card_offers_sandbox_run_and_open_diff: bool,
    /// An apply-capable scope never understates the review classes it would apply.
    pub apply_scope_never_understates_churn: bool,
    /// A generated-test review never hides assumption, fixture, or snapshot churn in one opaque
    /// apply path.
    pub generated_never_hides_assumption_fixture_or_snapshot_churn: bool,
    /// AI-assisted proposals preserve the same preview, rollback, and evidence rules as ordinary
    /// multi-file mutation flows.
    pub ai_proposals_preserve_preview_rollback_evidence_parity: bool,
    /// The component keeps the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The component keeps the same truth across every review consumer surface.
    pub components_stable_across_consumer_surfaces: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// The support / export packet reconstructs suggestion truth.
    pub support_export_reconstructs_suggestion_truth: bool,
    /// Later M5 review components cannot invent parallel suggestion vocabulary.
    pub later_components_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardConsumerProjection {
    /// Suggestion surfaces consume the shared trigger / assumption / apply vocabulary.
    pub suggestion_surfaces_consume_shared_vocabulary: bool,
    /// The suggestion-posture resolver reads a single canonical source.
    pub suggestion_posture_reads_single_source: bool,
    /// The apply-scope truth reads a single canonical source.
    pub apply_scope_reads_single_source: bool,
    /// The CI and support/export consumers read the same suggestion vocabulary.
    pub ci_and_support_read_same_suggestion_vocabulary: bool,
    /// Headless and desktop review read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the suggestion-card component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting test-evidence audit.
    pub test_evidence_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SuggestionCardComponentsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SuggestionCardComponentsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// AI test-review consumer rows.
    pub rows: Vec<M5SuggestionCardConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SuggestionCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SuggestionCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SuggestionCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SuggestionCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SuggestionCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 test-generation-suggestion-card primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuggestionCardComponentsPacket {
    /// Record kind; must equal [`M5_SUGGESTION_CARD_COMPONENTS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// AI test-review consumer rows.
    pub rows: Vec<M5SuggestionCardConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SuggestionCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SuggestionCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SuggestionCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SuggestionCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SuggestionCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SuggestionCardComponentsPacket {
    /// Builds an M5 suggestion-card-components primitive packet from stable-lane input.
    pub fn new(input: M5SuggestionCardComponentsPacketInput) -> Self {
        Self {
            record_kind: M5_SUGGESTION_CARD_COMPONENTS_RECORD_KIND.to_owned(),
            schema_version: M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 suggestion-card-components primitive invariants.
    pub fn validate(&self) -> Vec<M5SuggestionCardComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SUGGESTION_CARD_COMPONENTS_RECORD_KIND {
            violations.push(M5SuggestionCardComponentViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_VERSION {
            violations.push(M5SuggestionCardComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SuggestionCardComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_suggestion_posture_coverage(self, &mut violations);
        validate_apply_capability_separation(self, &mut violations);
        validate_assumption_disclosure(self, &mut violations);
        validate_preview_rollback_parity(self, &mut violations);
        validate_trigger_source_coverage(self, &mut violations);
        validate_review_class_coverage(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 suggestion card components packet serializes"),
        ) {
            violations.push(M5SuggestionCardComponentViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 suggestion card components packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per AI test-review consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,suggestion_anatomy,suggestion_postures,trigger_sources,review_classes,apply_scopes,suggestion_actions,suggestion_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.suggestion_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.suggestion_postures, |v| v.as_str()),
                join_tokens(&row.trigger_sources, |v| v.as_str()),
                join_tokens(&row.review_classes, |v| v.as_str()),
                join_tokens(&row.apply_scopes, |v| v.as_str()),
                join_tokens(&row.suggestion_actions, |v| v.as_str()),
                row.suggestion_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Test-Generation-Suggestion-Card Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Review consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Suggestion postures: {}\n",
            self.vocabulary_set.suggestion_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Review classes: {}\n",
            self.vocabulary_set.review_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Trigger sources: {}\n",
            self.vocabulary_set.trigger_sources.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Review consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked suggestions: {}\n",
                row.suggestion_examples.len()
            ));
            for case in &row.suggestion_examples {
                out.push_str(&format!(
                    "    - card `{}` (`{}`) -> `{}` (apply-capable `{}`, churn `{}`, assumptions `{}`, gen `{}`)\n",
                    case.resolved.suggestion_identity_ref,
                    case.resolved.trigger_source.as_str(),
                    case.resolved.suggestion_posture.as_str(),
                    case.resolved.is_apply_capable,
                    case.resolved.bundles_non_assertion_churn,
                    case.resolved.discloses_assumption_summary,
                    case.resolved.generated_file_count,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 suggestion-card-components export.
#[derive(Debug)]
pub enum M5SuggestionCardComponentArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SuggestionCardComponentViolation>),
}

impl fmt::Display for M5SuggestionCardComponentArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 suggestion card components export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 suggestion card components export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SuggestionCardComponentArtifactError {}

/// Validation failures emitted by [`M5SuggestionCardComponentsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SuggestionCardComponentViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required AI test-review consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A review consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory suggestion anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory suggestion export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked suggestion resolutions.
    ExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every suggestion posture.
    SuggestionPostureCoverageUnproven,
    /// The worked resolutions do not prove both an apply-capable proposal that names its churn and a
    /// multi-class proposal held to review-first.
    ApplyCapabilitySeparationUnproven,
    /// The worked resolutions do not prove an apply-capable generated card that discloses its
    /// assumption summary.
    AssumptionDisclosureUnproven,
    /// The worked resolutions do not prove an apply-capable proposal that keeps a diff-first preview
    /// and a rollback.
    PreviewRollbackParityUnproven,
    /// The worked resolutions do not exercise an uncovered-path / bug trigger and a manual request.
    TriggerSourceCoverageUnproven,
    /// The worked resolutions do not exercise the assertion, fixture, and snapshot review classes.
    ReviewClassCoverageUnproven,
    /// A worked resolution does not preserve its exact identity and scope.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SuggestionCardComponentViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleMissing => "example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::SuggestionPostureCoverageUnproven => "suggestion_posture_coverage_unproven",
            Self::ApplyCapabilitySeparationUnproven => "apply_capability_separation_unproven",
            Self::AssumptionDisclosureUnproven => "assumption_disclosure_unproven",
            Self::PreviewRollbackParityUnproven => "preview_rollback_parity_unproven",
            Self::TriggerSourceCoverageUnproven => "trigger_source_coverage_unproven",
            Self::ReviewClassCoverageUnproven => "review_class_coverage_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 suggestion-card-components export.
pub fn current_stable_m5_suggestion_card_components_export(
) -> Result<M5SuggestionCardComponentsPacket, M5SuggestionCardComponentArtifactError> {
    let packet: M5SuggestionCardComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-generation-suggestion-card-primitive-proof/support_export.json"
    )))
    .map_err(M5SuggestionCardComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SuggestionCardComponentArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_REF,
        M5_SUGGESTION_CARD_COMPONENTS_DOC_REF,
        M5_SUGGESTION_CARD_COMPONENTS_COMPONENT_MATRIX_REF,
        M5_SUGGESTION_CARD_COMPONENTS_TEST_GENERATION_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SuggestionCardComponentViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SuggestionCardComponentViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let present: BTreeSet<M5SuggestionCardConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5SuggestionCardConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5SuggestionCardComponentViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.suggestion_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.trigger_sources.is_empty()
            || row.review_classes.is_empty()
            || row.assumption_classes.is_empty()
            || row.apply_scopes.is_empty()
            || row.suggestion_postures.is_empty()
            || row.provenance_classes.is_empty()
            || row.suggestion_actions.is_empty()
            || row.suggestion_export_fields.is_empty()
        {
            violations.push(M5SuggestionCardComponentViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5SuggestionCardComponentViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5SuggestionCardComponentViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5SuggestionCardComponentViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SuggestionCardComponentViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SuggestionCardComponentViolation::DowngradeTriggersMissing);
        }
        if row.suggestion_examples.is_empty() {
            violations.push(M5SuggestionCardComponentViolation::ExampleMissing);
        }
        if row
            .suggestion_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5SuggestionCardComponentViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SuggestionCardComponentViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5SuggestionCardComponentViolation::RowInvariantViolated);
        }
    }
}

/// Every suggestion posture must be exercised by some worked resolution — the proof that an
/// assertion-only, fixture-and-assertion, snapshot-included, full-bundle, review-required, and
/// apply-blocked suggestion each get a distinct posture rather than one collapsed verdict.
fn validate_suggestion_posture_coverage(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let exercised: BTreeSet<M5SuggestionPosture> = packet
        .rows
        .iter()
        .flat_map(|row| row.suggestion_examples.iter())
        .map(|case| case.resolved.suggestion_posture)
        .collect();
    let covered = M5SuggestionPosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5SuggestionCardComponentViolation::SuggestionPostureCoverageUnproven);
    }
}

/// At least one worked resolution must prove an apply-capable proposal that names its churn, and at
/// least one must prove a multi-class proposal held to a review-first (non-apply-capable) path — the
/// acceptance-criterion example that assertion, fixture, and snapshot churn are separated into
/// distinct review classes before any apply-capable action is offered.
fn validate_apply_capability_separation(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let has_apply_capable = packet.rows.iter().any(|row| {
        row.suggestion_examples
            .iter()
            .any(|case| case.resolved.is_apply_capable)
    });
    let has_multi_class_review_first = packet.rows.iter().any(|row| {
        row.suggestion_examples
            .iter()
            .any(|case| !case.resolved.is_apply_capable && case.resolved.review_classes.len() > 1)
    });
    if !(has_apply_capable && has_multi_class_review_first) {
        violations.push(M5SuggestionCardComponentViolation::ApplyCapabilitySeparationUnproven);
    }
}

/// At least one worked resolution must prove an apply-capable generated card that discloses its
/// assumption summary — the acceptance-criterion requirement that generated-test review packets stop
/// hiding assumption churn inside one opaque diff path.
fn validate_assumption_disclosure(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let has_disclosed_assumptions = packet.rows.iter().any(|row| {
        row.suggestion_examples.iter().any(|case| {
            case.resolved.is_apply_capable
                && case.resolved.generated_file_count > 0
                && case.resolved.discloses_assumption_summary
                && !case.resolved.assumption_classes.is_empty()
        })
    });
    if !has_disclosed_assumptions {
        violations.push(M5SuggestionCardComponentViolation::AssumptionDisclosureUnproven);
    }
}

/// At least one worked resolution must prove an apply-capable proposal that keeps a diff-first
/// preview and a rollback — the acceptance-criterion requirement that AI-assisted test proposals
/// preserve the same preview, rollback, and evidence rules as ordinary multi-file mutation flows.
fn validate_preview_rollback_parity(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let has_preview_rollback = packet.rows.iter().any(|row| {
        row.suggestion_examples.iter().any(|case| {
            case.resolved.is_apply_capable
                && case.resolved.offers_diff_preview
                && case.resolved.offers_rollback
                && case.resolved.preserves_preview_and_rollback
        })
    });
    if !has_preview_rollback {
        violations.push(M5SuggestionCardComponentViolation::PreviewRollbackParityUnproven);
    }
}

/// The worked resolutions must exercise an uncovered-path / bug trigger and a manual request — the
/// requirement that a suggestion always names the uncovered path or the bug context behind it.
fn validate_trigger_source_coverage(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let cases: Vec<&M5SuggestionCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.suggestion_examples.iter())
        .collect();
    let has_uncovered_or_bug = cases
        .iter()
        .any(|case| case.resolved.trigger_source.is_uncovered_path_or_bug());
    let has_manual = cases
        .iter()
        .any(|case| case.resolved.trigger_source == M5GenerationTriggerSource::ManualRequest);
    if !(has_uncovered_or_bug && has_manual) {
        violations.push(M5SuggestionCardComponentViolation::TriggerSourceCoverageUnproven);
    }
}

/// The worked resolutions must exercise the assertion, helper / fixture, and snapshot / golden
/// review classes — the acceptance-criterion requirement that generated changes are separated into
/// distinct review classes rather than one opaque bundle.
fn validate_review_class_coverage(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let exercised: BTreeSet<M5GeneratedReviewClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.suggestion_examples.iter())
        .flat_map(|case| case.resolved.review_classes.iter().copied())
        .collect();
    let covered = M5GeneratedReviewClass::ALL
        .iter()
        .all(|class| exercised.contains(class));
    if !covered {
        violations.push(M5SuggestionCardComponentViolation::ReviewClassCoverageUnproven);
    }
}

/// Every worked resolution must preserve its exact identity and scope — the invariant that the
/// component never rewrites the user's suggestion identity, targets, or review classes.
fn validate_identity_preservation(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.suggestion_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5SuggestionCardComponentViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.card_shows_trigger_source_and_targets,
        review.card_shows_trigger_context,
        review.card_shows_assumption_summary,
        review.card_separates_review_classes,
        review.card_shows_generated_file_count,
        review.card_offers_sandbox_run_and_open_diff,
        review.apply_scope_never_understates_churn,
        review.generated_never_hides_assumption_fixture_or_snapshot_churn,
        review.ai_proposals_preserve_preview_rollback_evidence_parity,
        review.components_stable_across_deployment_lines,
        review.components_stable_across_consumer_surfaces,
        review.every_component_declares_accessibility_route,
        review.support_export_reconstructs_suggestion_truth,
        review.later_components_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5SuggestionCardComponentViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.suggestion_surfaces_consume_shared_vocabulary,
        projection.suggestion_posture_reads_single_source,
        projection.apply_scope_reads_single_source,
        projection.ci_and_support_read_same_suggestion_vocabulary,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5SuggestionCardComponentViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SuggestionCardComponentViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SuggestionCardComponentsPacket,
    violations: &mut Vec<M5SuggestionCardComponentViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SuggestionCardComponentViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
