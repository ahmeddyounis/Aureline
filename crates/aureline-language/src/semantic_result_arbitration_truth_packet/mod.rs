//! Semantic-result arbitration, disagreement-detail, and fallback-banner
//! truth packet.
//!
//! This module is the language-owned contract for the per-answer arbitration
//! inspector that keeps definition, references, hierarchy, and completion
//! results trustworthy across the M5 search, docs, framework, notebook, and
//! generated-source consumers. Where the sibling
//! [`crate::provider_status_surface_truth_packet`] certifies the reusable
//! provider-status strip, capability-negotiation drawer, and result-provenance
//! pill UI objects, this packet certifies the *result* each of those objects
//! anchors: which provider won, what the alternate providers said, where
//! confidence and completeness changed, and when a semantic answer degraded to
//! heuristic, file-local, or text behavior.
//!
//! Each row binds one result lane on one consumer surface together with:
//!
//! - an **arbitration inspector** block — the acting (winning) provider
//!   family, the basis on which it won, whether the alternate providers stay
//!   inspectable, and the route that opens the detail;
//! - a **disagreement detail** block — the conflict class, whether the
//!   conflict changes target identity, scope coverage, or refactor safety, and
//!   how that disagreement is made visible; and
//! - a **fallback banner** block — the result tier, the banner shown when a
//!   semantic answer degraded, the guarantees that were *retained*, the
//!   guarantees that were *lost*, the scope the surface may still claim, any
//!   skipped-coverage gap, and (for a mutating follow-up) the typed preview
//!   completeness and rollback checkpoint that the launch-language refactor
//!   safety model still requires.
//!
//! The packet reuses the closed provider-family, conflict, completeness,
//! support, evidence, known-limit, downgrade-automation, confidence, and
//! consumer-surface vocabularies frozen by the
//! [`crate::provider_refactor_matrix_truth_packet`] matrix instead of minting a
//! local synonym set, and adds only the arbitration-inspector,
//! disagreement-impact, and fallback-banner vocabulary those result lanes need
//! on top.
//!
//! The validator narrows below stable — it never silently publishes — whenever
//! a row would hide truth the source documents require to stay inspectable: a
//! disagreement collapsed into ranking-only output that drops the losing
//! provider, a materially conflicting result fused without a visible detail
//! path, a degraded answer with no fallback banner or with no recorded lost
//! guarantee, an all-results or whole-workspace claim resting only on lexical
//! evidence, whole-workspace wording kept after excluded roots / unloaded
//! slices / generated-only edges were skipped, an opaque spinner standing in
//! for an inspection route, or a mutating follow-up that bypasses typed
//! preview completeness and a rollback checkpoint.
//!
//! The packet is metadata-only: it never admits raw source bodies, raw
//! notebook outputs, raw generated artifacts, provider payloads, secrets, or
//! ambient credentials past the boundary. It carries opaque ids, closed
//! vocabulary tokens, and export-safe refs only.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::provider_refactor_matrix_truth_packet::{
    CompletenessClass, ConfidenceClass, ConflictClass, ConsumerSurface, DowngradeAutomationClass,
    EvidenceClass, FindingSeverity, KnownLimitClass, PromotionState, ProviderFamilyClass,
    SupportClass,
};

/// Stable record-kind tag for [`SemanticResultArbitrationTruthPacket`].
pub const SEMANTIC_RESULT_ARBITRATION_TRUTH_PACKET_RECORD_KIND: &str =
    "semantic_result_arbitration_truth_stable_packet";

/// Stable record-kind tag for [`SemanticResultArbitrationTruthSupportExport`].
pub const SEMANTIC_RESULT_ARBITRATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "semantic_result_arbitration_truth_support_export";

/// Integer schema version for the semantic-result arbitration truth packet.
pub const SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_REF: &str =
    "schemas/language/semantic_result_arbitration_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const SEMANTIC_RESULT_ARBITRATION_TRUTH_DOC_REF: &str =
    "docs/m5/arbitration-inspectors-disagreement-detail-and-semantic-to-text-fallback-banners.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const SEMANTIC_RESULT_ARBITRATION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/language/m5/arbitration-inspectors-disagreement-detail-and-semantic-to-text-fallback-banners.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SEMANTIC_RESULT_ARBITRATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/language/m5/semantic_result_arbitration_truth_packet";

/// Repo-relative path of the checked-in stable packet.
pub const SEMANTIC_RESULT_ARBITRATION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/language/m5/semantic_result_arbitration_truth_packet.json";

/// Repo-relative path of the sibling surface packet whose objects anchor these
/// results.
pub const SEMANTIC_RESULT_ARBITRATION_SURFACE_SOURCE_REF: &str =
    "artifacts/language/m5/provider_status_surface_truth_packet.json";

/// Closed result-surface vocabulary. Every required surface MUST have rows in
/// any stable packet so disagreement and partial-truth stay inspectable in the
/// search, docs, framework, notebook, and generated-source consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultSurfaceClass {
    /// Search / find-all results surface.
    SearchSurface,
    /// Docs-linked hover / definition surface.
    DocsSurface,
    /// Framework-pack / framework-analyzer result surface.
    FrameworkSurface,
    /// Notebook-aware result surface.
    NotebookSurface,
    /// Generated / scaffolded source result surface.
    GeneratedSourceSurface,
}

impl ResultSurfaceClass {
    /// Every required result surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::SearchSurface,
        Self::DocsSurface,
        Self::FrameworkSurface,
        Self::NotebookSurface,
        Self::GeneratedSourceSurface,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchSurface => "search_surface",
            Self::DocsSurface => "docs_surface",
            Self::FrameworkSurface => "framework_surface",
            Self::NotebookSurface => "notebook_surface",
            Self::GeneratedSourceSurface => "generated_source_surface",
        }
    }
}

/// Closed result-lane vocabulary. The packet certifies the four
/// semantic-answer lanes that depend on arbitration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultLaneClass {
    /// Definition / go-to-target lookup.
    Definition,
    /// Reference / find-all-references set.
    References,
    /// Call / type hierarchy.
    Hierarchy,
    /// Completion result set.
    Completion,
}

impl ResultLaneClass {
    /// Every required result lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::Definition,
        Self::References,
        Self::Hierarchy,
        Self::Completion,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Definition => "definition",
            Self::References => "references",
            Self::Hierarchy => "hierarchy",
            Self::Completion => "completion",
        }
    }
}

/// Closed arbitration-basis vocabulary: why the winning provider was chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArbitrationBasisClass {
    /// A single admissible provider answered authoritatively.
    SingleProviderAuthoritative,
    /// The provider held the highest semantic authority for the lane.
    HighestSemanticAuthority,
    /// A framework overlay took precedence over a generic provider.
    FrameworkOverlayPrecedence,
    /// A policy or trust decision selected the result.
    PolicyTrustOverride,
    /// The provider won on freshness / recency.
    FreshnessRecency,
    /// The provider was the only admissible source after others were excluded.
    OnlyAdmissibleProvider,
    /// No semantic winner existed; the row narrowed to a labeled fallback.
    NarrowedNoSemanticWinner,
    /// Row does not bind an arbitration basis.
    NotApplicable,
    /// Row has no bound arbitration basis; this never qualifies certified.
    BasisUnbound,
}

impl ArbitrationBasisClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleProviderAuthoritative => "single_provider_authoritative",
            Self::HighestSemanticAuthority => "highest_semantic_authority",
            Self::FrameworkOverlayPrecedence => "framework_overlay_precedence",
            Self::PolicyTrustOverride => "policy_trust_override",
            Self::FreshnessRecency => "freshness_recency",
            Self::OnlyAdmissibleProvider => "only_admissible_provider",
            Self::NarrowedNoSemanticWinner => "narrowed_no_semantic_winner",
            Self::NotApplicable => "not_applicable",
            Self::BasisUnbound => "basis_unbound",
        }
    }

    /// True when this basis names a concrete reason a provider won.
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::NotApplicable | Self::BasisUnbound)
    }
}

/// Closed alternate-provider visibility vocabulary: whether the providers that
/// did not win stay inspectable. A disagreement that drops its loser collapses
/// truth and is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlternateProviderVisibilityClass {
    /// A single provider answered; there is no alternate to preserve.
    NotApplicableSingleProvider,
    /// The alternate providers and what they said stay inspectable.
    AlternatesPreservedInspectable,
    /// The disagreement was collapsed into a ranking-only result; the loser is
    /// no longer inspectable.
    AlternatesCollapsedRankingOnly,
    /// The arbitration recorded a conflict but exposed no alternate at all.
    NoAlternatesExposed,
}

impl AlternateProviderVisibilityClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableSingleProvider => "not_applicable_single_provider",
            Self::AlternatesPreservedInspectable => "alternates_preserved_inspectable",
            Self::AlternatesCollapsedRankingOnly => "alternates_collapsed_ranking_only",
            Self::NoAlternatesExposed => "no_alternates_exposed",
        }
    }

    /// True when the losing providers are dropped from the result.
    pub const fn loses_alternates(self) -> bool {
        matches!(
            self,
            Self::AlternatesCollapsedRankingOnly | Self::NoAlternatesExposed
        )
    }
}

/// Closed inspector-route vocabulary: how a user opens the arbitration detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorRouteClass {
    /// No detail route is bound (only valid when nothing needs inspecting).
    NotApplicable,
    /// Opens the arbitration inspector that explains why the winner won.
    OpenArbitrationInspector,
    /// Opens the disagreement detail showing competing results.
    OpenDisagreementDetail,
    /// Opens the result-provenance pill detail.
    OpenProvenancePill,
    /// An opaque loading spinner stands in for a real inspection route.
    OpaqueSpinner,
}

impl InspectorRouteClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::OpenArbitrationInspector => "open_arbitration_inspector",
            Self::OpenDisagreementDetail => "open_disagreement_detail",
            Self::OpenProvenancePill => "open_provenance_pill",
            Self::OpaqueSpinner => "opaque_spinner",
        }
    }

    /// True when this route actually opens an inspectable detail.
    pub const fn is_inspectable(self) -> bool {
        matches!(
            self,
            Self::OpenArbitrationInspector
                | Self::OpenDisagreementDetail
                | Self::OpenProvenancePill
        )
    }
}

/// Closed disagreement-impact vocabulary: what the conflict changes. A
/// conflict that changes target identity, scope coverage, or refactor safety
/// MUST surface a visible detail path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisagreementImpactClass {
    /// No disagreement, or it changes nothing the user must act on.
    None,
    /// Providers disagree on which target the answer points to.
    TargetIdentityChanged,
    /// Providers disagree on how much scope the answer covers.
    ScopeCoverageChanged,
    /// Providers disagree in a way that changes refactor / edit safety.
    RefactorSafetyChanged,
    /// Providers disagree only on freshness / recency.
    FreshnessOnly,
}

impl DisagreementImpactClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::TargetIdentityChanged => "target_identity_changed",
            Self::ScopeCoverageChanged => "scope_coverage_changed",
            Self::RefactorSafetyChanged => "refactor_safety_changed",
            Self::FreshnessOnly => "freshness_only",
        }
    }

    /// True when the conflict materially changes target identity, scope, or
    /// refactor safety and therefore owes a visible detail path.
    pub const fn changes_material(self) -> bool {
        matches!(
            self,
            Self::TargetIdentityChanged | Self::ScopeCoverageChanged | Self::RefactorSafetyChanged
        )
    }
}

/// Closed disagreement-visibility vocabulary: how the disagreement is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisagreementVisibilityClass {
    /// No disagreement to render.
    None,
    /// An inline conflict panel is attached to the answer.
    InlineConflictPanel,
    /// A side-panel disagreement inspector is offered.
    SidePanelInspector,
    /// The result is blocked until the disagreement is reviewed.
    BlockedUntilReview,
}

impl DisagreementVisibilityClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::InlineConflictPanel => "inline_conflict_panel",
            Self::SidePanelInspector => "side_panel_inspector",
            Self::BlockedUntilReview => "blocked_until_review",
        }
    }

    /// True when the disagreement is made visible to the user.
    pub const fn is_visible(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Closed result-tier vocabulary: the confidence grade of the answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultTierClass {
    /// Exact, complete, live semantic result.
    ExactSemantic,
    /// Cached semantic result, still semantic but not freshly proven.
    CachedSemantic,
    /// Partial semantic result with a visible scope label.
    PartialSemantic,
    /// Heuristic / structural result, not semantic.
    HeuristicStructural,
    /// Text / lexical result only.
    TextLexical,
    /// No admissible result.
    Unavailable,
}

impl ResultTierClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSemantic => "exact_semantic",
            Self::CachedSemantic => "cached_semantic",
            Self::PartialSemantic => "partial_semantic",
            Self::HeuristicStructural => "heuristic_structural",
            Self::TextLexical => "text_lexical",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when this tier is an exact, complete, live semantic answer.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::ExactSemantic)
    }

    /// True when this tier rests on lexical or heuristic evidence and may not
    /// claim semantic completeness.
    pub const fn is_lexical_or_heuristic(self) -> bool {
        matches!(self, Self::TextLexical | Self::HeuristicStructural)
    }
}

/// Closed fallback-banner vocabulary: the banner shown when a semantic answer
/// degraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackBannerClass {
    /// No fallback banner; the answer is exact semantic.
    None,
    /// Semantic answer degraded to a text / lexical answer.
    SemanticToTextFallback,
    /// Semantic answer degraded to a heuristic / structural answer.
    SemanticToHeuristicFallback,
    /// Semantic answer degraded to file-local behavior.
    SemanticToFileLocalFallback,
    /// A cached semantic result was reused with an explicit label.
    CachedSemanticReuse,
    /// No result is available; the banner says so.
    UnavailableBanner,
}

impl FallbackBannerClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SemanticToTextFallback => "semantic_to_text_fallback",
            Self::SemanticToHeuristicFallback => "semantic_to_heuristic_fallback",
            Self::SemanticToFileLocalFallback => "semantic_to_file_local_fallback",
            Self::CachedSemanticReuse => "cached_semantic_reuse",
            Self::UnavailableBanner => "unavailable_banner",
        }
    }

    /// True when a banner is being shown.
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Closed retained-guarantee vocabulary: the guarantee that still holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetainedGuaranteeClass {
    /// Full semantic guarantee across the claimed scope.
    FullSemanticGuarantee,
    /// Semantic guarantee within the active file only.
    FileLocalSemantic,
    /// Structural matches only.
    StructuralMatchOnly,
    /// Lexical matches only.
    LexicalMatchOnly,
    /// Nothing is retained.
    NoneRetained,
}

impl RetainedGuaranteeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSemanticGuarantee => "full_semantic_guarantee",
            Self::FileLocalSemantic => "file_local_semantic",
            Self::StructuralMatchOnly => "structural_match_only",
            Self::LexicalMatchOnly => "lexical_match_only",
            Self::NoneRetained => "none_retained",
        }
    }

    /// True when the retained guarantee is still a semantic one.
    pub const fn is_semantic(self) -> bool {
        matches!(self, Self::FullSemanticGuarantee | Self::FileLocalSemantic)
    }
}

/// Closed lost-guarantee vocabulary: the guarantee the degraded answer gave up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LostGuaranteeClass {
    /// Nothing was lost.
    NoneLost,
    /// Lost cross-file semantic resolution.
    LostCrossFileSemantic,
    /// Lost whole-workspace scope coverage.
    LostWholeWorkspaceScope,
    /// Lost the safe-wide-rename guarantee.
    LostSafeRenameGuarantee,
    /// Lost the all-references completeness guarantee.
    LostAllReferencesGuarantee,
    /// Lost confidence that the target identity is exact.
    LostSemanticTargetIdentity,
}

impl LostGuaranteeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneLost => "none_lost",
            Self::LostCrossFileSemantic => "lost_cross_file_semantic",
            Self::LostWholeWorkspaceScope => "lost_whole_workspace_scope",
            Self::LostSafeRenameGuarantee => "lost_safe_rename_guarantee",
            Self::LostAllReferencesGuarantee => "lost_all_references_guarantee",
            Self::LostSemanticTargetIdentity => "lost_semantic_target_identity",
        }
    }

    /// True when a guarantee was lost and must be disclosed.
    pub const fn is_lost(self) -> bool {
        !matches!(self, Self::NoneLost)
    }
}

/// Closed claim-scope vocabulary: the breadth the surface may still claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimScopeClass {
    /// Whole-workspace, all-results claim (every match across the workspace).
    WholeWorkspaceAllResults,
    /// Results limited to the currently loaded slice.
    LoadedSliceResults,
    /// Results limited to the active file.
    ActiveFileResults,
    /// Results limited to the open notebook cells.
    OpenCellsResults,
    /// Results that explicitly exclude generated edges.
    GeneratedExcludedResults,
    /// A single resolved target.
    SingleTarget,
}

impl ClaimScopeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeWorkspaceAllResults => "whole_workspace_all_results",
            Self::LoadedSliceResults => "loaded_slice_results",
            Self::ActiveFileResults => "active_file_results",
            Self::OpenCellsResults => "open_cells_results",
            Self::GeneratedExcludedResults => "generated_excluded_results",
            Self::SingleTarget => "single_target",
        }
    }

    /// True when this scope claims the whole workspace / all results.
    pub const fn is_whole_workspace(self) -> bool {
        matches!(self, Self::WholeWorkspaceAllResults)
    }
}

/// Closed coverage-gap vocabulary: which roots / slices / edges were skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageGapClass {
    /// No coverage gap; the claimed scope was fully scanned.
    None,
    /// Excluded roots were skipped.
    ExcludedRootsSkipped,
    /// Unloaded slices were skipped.
    UnloadedSlicesSkipped,
    /// Generated-only edges were skipped.
    GeneratedOnlyEdgesSkipped,
    /// Notebook cells were not scanned.
    NotebookCellsUnscanned,
}

impl CoverageGapClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ExcludedRootsSkipped => "excluded_roots_skipped",
            Self::UnloadedSlicesSkipped => "unloaded_slices_skipped",
            Self::GeneratedOnlyEdgesSkipped => "generated_only_edges_skipped",
            Self::NotebookCellsUnscanned => "notebook_cells_unscanned",
        }
    }

    /// True when some roots / slices / edges were skipped.
    pub const fn is_gap(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Closed anchor-action vocabulary: what action the result anchors. A mutating
/// follow-up must bind typed preview completeness and a rollback checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorActionClass {
    /// The result is read-only (inspect / display).
    ResultOnly,
    /// The result anchors navigation only.
    NavigationOnly,
    /// The result anchors a completion insertion.
    CompletionInsert,
    /// The result anchors a mutating follow-up that runs through preview.
    MutatingFollowupPreview,
}

impl AnchorActionClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResultOnly => "result_only",
            Self::NavigationOnly => "navigation_only",
            Self::CompletionInsert => "completion_insert",
            Self::MutatingFollowupPreview => "mutating_followup_preview",
        }
    }

    /// True when the anchored action mutates source and owes preview + rollback.
    pub const fn is_mutating(self) -> bool {
        matches!(self, Self::MutatingFollowupPreview)
    }
}

/// Closed validation-finding vocabulary for the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// A required result surface has no row.
    MissingSurfaceCoverage,
    /// A required result lane has no row.
    MissingLaneCoverage,
    /// A row admits raw source bodies past the boundary.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority / credentials past the boundary.
    AmbientAuthorityPresent,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row does not name a concrete acting provider family.
    MissingProviderFamily,
    /// A row does not bind a concrete arbitration basis.
    MissingArbitrationBasis,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A row claims certified while a required binding is unbound.
    CertifiedWithUnboundBinding,
    /// A narrowed row carries no disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row declares a known limit without a disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row binds a downgrade automation without a disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A required consumer projection is missing or does not preserve the packet.
    MissingConsumerProjection,
    /// A disagreement was collapsed into ranking-only output; the loser is lost.
    LosingProviderCollapsed,
    /// A materially conflicting result has no visible detail path.
    DisagreementDetailPathMissing,
    /// An opaque spinner stands in for a real inspection route.
    OpaqueInspectorRoute,
    /// A materially conflicting result was fused silently into an exact answer.
    SilentFusionOfConflict,
    /// A degraded result has no fallback banner or no recorded lost guarantee.
    FallbackBannerMissing,
    /// An exact result carries a fallback banner or a lost guarantee.
    FallbackBannerOnExactResult,
    /// A whole-workspace / all-results claim rests on lexical evidence only.
    OverclaimedScopeOnLexicalEvidence,
    /// Whole-workspace wording is kept after coverage was skipped.
    WholeWorkspaceWordingWithCoverageGap,
    /// A mutating follow-up bypasses typed preview completeness or rollback.
    MutatingAnchorBypassesPreview,
    /// A text / lexical result overstates a retained semantic guarantee.
    RetainedGuaranteeOverstated,
    /// A row claims certified at low confidence; it narrows until evidence grows.
    CertifiedAtLowConfidence,
    /// The stored promotion state does not match the derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSurfaceCoverage => "missing_surface_coverage",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::MissingProviderFamily => "missing_provider_family",
            Self::MissingArbitrationBasis => "missing_arbitration_basis",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::CertifiedWithUnboundBinding => "certified_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::LosingProviderCollapsed => "losing_provider_collapsed",
            Self::DisagreementDetailPathMissing => "disagreement_detail_path_missing",
            Self::OpaqueInspectorRoute => "opaque_inspector_route",
            Self::SilentFusionOfConflict => "silent_fusion_of_conflict",
            Self::FallbackBannerMissing => "fallback_banner_missing",
            Self::FallbackBannerOnExactResult => "fallback_banner_on_exact_result",
            Self::OverclaimedScopeOnLexicalEvidence => "overclaimed_scope_on_lexical_evidence",
            Self::WholeWorkspaceWordingWithCoverageGap => {
                "whole_workspace_wording_with_coverage_gap"
            }
            Self::MutatingAnchorBypassesPreview => "mutating_anchor_bypasses_preview",
            Self::RetainedGuaranteeOverstated => "retained_guarantee_overstated",
            Self::CertifiedAtLowConfidence => "certified_at_low_confidence",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
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

/// One result-arbitration row binding a surface and lane to the arbitration,
/// disagreement, and fallback-banner truth it must show.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultArbitrationRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Host surface this row renders on.
    pub result_surface_class: ResultSurfaceClass,
    /// Result lane this row covers.
    pub result_lane_class: ResultLaneClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Acting (winning) provider family.
    pub acting_provider_family_class: ProviderFamilyClass,
    /// Basis on which the acting provider won.
    pub arbitration_basis_class: ArbitrationBasisClass,
    /// Whether the alternate (losing) providers stay inspectable.
    pub alternate_provider_visibility_class: AlternateProviderVisibilityClass,
    /// Route that opens the arbitration / disagreement detail.
    pub inspector_route_class: InspectorRouteClass,
    /// Conflict class for the result.
    pub conflict_class: ConflictClass,
    /// What the disagreement changes (target identity, scope, safety, …).
    pub disagreement_impact_class: DisagreementImpactClass,
    /// How the disagreement is made visible.
    pub disagreement_visibility_class: DisagreementVisibilityClass,
    /// Confidence tier of the answer.
    pub result_tier_class: ResultTierClass,
    /// Fallback banner shown when the answer degraded.
    pub fallback_banner_class: FallbackBannerClass,
    /// Guarantee that still holds.
    pub retained_guarantee_class: RetainedGuaranteeClass,
    /// Guarantee the degraded answer gave up.
    pub lost_guarantee_class: LostGuaranteeClass,
    /// Breadth the surface may still claim.
    pub claim_scope_class: ClaimScopeClass,
    /// Which roots / slices / edges were skipped.
    pub coverage_gap_class: CoverageGapClass,
    /// What action the result anchors.
    pub anchor_action_class: AnchorActionClass,
    /// Typed preview completeness for a mutating follow-up (or `not_applicable`).
    pub preview_completeness_class: CompletenessClass,
    /// Rollback checkpoint ref required for a mutating follow-up.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Disclosure ref required when the row is narrowed, declares a known
    /// limit, or binds a non-`none` automation.
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

impl ResultArbitrationRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
            && self.acting_provider_family_class.is_concrete()
            && self.arbitration_basis_class.is_concrete()
    }

    /// True when this row records a provider disagreement (not single-provider).
    fn has_disagreement(&self) -> bool {
        matches!(
            self.conflict_class,
            ConflictClass::ArbitratedWinnerLoserPreserved
                | ConflictClass::UnresolvedDisagreementSurfaced
        )
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultArbitrationConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Surface packet id consumed by the projection.
    pub surface_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the result-surface vocabulary is preserved verbatim.
    pub preserves_result_surface_vocabulary: bool,
    /// True when the result-lane vocabulary is preserved verbatim.
    pub preserves_result_lane_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the provider-family vocabulary is preserved verbatim.
    pub preserves_provider_family_vocabulary: bool,
    /// True when the arbitration-basis vocabulary is preserved verbatim.
    pub preserves_arbitration_basis_vocabulary: bool,
    /// True when the alternate-provider-visibility vocabulary is preserved.
    pub preserves_alternate_provider_visibility_vocabulary: bool,
    /// True when the inspector-route vocabulary is preserved verbatim.
    pub preserves_inspector_route_vocabulary: bool,
    /// True when the conflict vocabulary is preserved verbatim.
    pub preserves_conflict_vocabulary: bool,
    /// True when the disagreement-impact vocabulary is preserved verbatim.
    pub preserves_disagreement_impact_vocabulary: bool,
    /// True when the disagreement-visibility vocabulary is preserved verbatim.
    pub preserves_disagreement_visibility_vocabulary: bool,
    /// True when the result-tier vocabulary is preserved verbatim.
    pub preserves_result_tier_vocabulary: bool,
    /// True when the fallback-banner vocabulary is preserved verbatim.
    pub preserves_fallback_banner_vocabulary: bool,
    /// True when the retained-guarantee vocabulary is preserved verbatim.
    pub preserves_retained_guarantee_vocabulary: bool,
    /// True when the lost-guarantee vocabulary is preserved verbatim.
    pub preserves_lost_guarantee_vocabulary: bool,
    /// True when the claim-scope vocabulary is preserved verbatim.
    pub preserves_claim_scope_vocabulary: bool,
    /// True when the coverage-gap vocabulary is preserved verbatim.
    pub preserves_coverage_gap_vocabulary: bool,
    /// True when the anchor-action vocabulary is preserved verbatim.
    pub preserves_anchor_action_vocabulary: bool,
    /// True when the completeness vocabulary is preserved verbatim.
    pub preserves_completeness_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl ResultArbitrationConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.surface_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_result_surface_vocabulary
            && self.preserves_result_lane_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_provider_family_vocabulary
            && self.preserves_arbitration_basis_vocabulary
            && self.preserves_alternate_provider_visibility_vocabulary
            && self.preserves_inspector_route_vocabulary
            && self.preserves_conflict_vocabulary
            && self.preserves_disagreement_impact_vocabulary
            && self.preserves_disagreement_visibility_vocabulary
            && self.preserves_result_tier_vocabulary
            && self.preserves_fallback_banner_vocabulary
            && self.preserves_retained_guarantee_vocabulary
            && self.preserves_lost_guarantee_vocabulary
            && self.preserves_claim_scope_vocabulary
            && self.preserves_coverage_gap_vocabulary
            && self.preserves_anchor_action_vocabulary
            && self.preserves_completeness_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`SemanticResultArbitrationTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticResultArbitrationTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Surfaces the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<ResultSurfaceClass>,
    /// Result-arbitration rows.
    #[serde(default)]
    pub rows: Vec<ResultArbitrationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ResultArbitrationConsumerProjection>,
    /// Source contracts (surface packet / docs / schema / fixtures) consumed.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Language-owned packet binding the arbitration inspector, disagreement
/// detail, and semantic-to-text fallback banner for definition, references,
/// hierarchy, and completion results across the M5 search, docs, framework,
/// notebook, and generated-source surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticResultArbitrationTruthPacket {
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
    /// Surfaces the packet covers.
    #[serde(default)]
    pub covered_surfaces: Vec<ResultSurfaceClass>,
    /// Result-arbitration rows.
    #[serde(default)]
    pub rows: Vec<ResultArbitrationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ResultArbitrationConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl SemanticResultArbitrationTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: SemanticResultArbitrationTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: SEMANTIC_RESULT_ARBITRATION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_surfaces: input.covered_surfaces,
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

    /// Re-validates the packet against stable result-arbitration invariants.
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

    /// Returns the unique result-surface tokens observed across rows.
    pub fn result_surface_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.result_surface_class.as_str())
    }

    /// Returns the unique result-lane tokens observed across rows.
    pub fn result_lane_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.result_lane_class.as_str())
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.support_class.as_str())
    }

    /// Returns the unique provider-family tokens observed across rows.
    pub fn provider_family_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.acting_provider_family_class.as_str())
    }

    /// Returns the unique arbitration-basis tokens observed across rows.
    pub fn arbitration_basis_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.arbitration_basis_class.as_str())
    }

    /// Returns the unique alternate-provider-visibility tokens across rows.
    pub fn alternate_provider_visibility_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.alternate_provider_visibility_class.as_str())
    }

    /// Returns the unique inspector-route tokens observed across rows.
    pub fn inspector_route_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.inspector_route_class.as_str())
    }

    /// Returns the unique conflict tokens observed across rows.
    pub fn conflict_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.conflict_class.as_str())
    }

    /// Returns the unique disagreement-impact tokens observed across rows.
    pub fn disagreement_impact_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.disagreement_impact_class.as_str())
    }

    /// Returns the unique disagreement-visibility tokens observed across rows.
    pub fn disagreement_visibility_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.disagreement_visibility_class.as_str())
    }

    /// Returns the unique result-tier tokens observed across rows.
    pub fn result_tier_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.result_tier_class.as_str())
    }

    /// Returns the unique fallback-banner tokens observed across rows.
    pub fn fallback_banner_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.fallback_banner_class.as_str())
    }

    /// Returns the unique retained-guarantee tokens observed across rows.
    pub fn retained_guarantee_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.retained_guarantee_class.as_str())
    }

    /// Returns the unique lost-guarantee tokens observed across rows.
    pub fn lost_guarantee_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.lost_guarantee_class.as_str())
    }

    /// Returns the unique claim-scope tokens observed across rows.
    pub fn claim_scope_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.claim_scope_class.as_str())
    }

    /// Returns the unique coverage-gap tokens observed across rows.
    pub fn coverage_gap_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.coverage_gap_class.as_str())
    }

    /// Returns the unique anchor-action tokens observed across rows.
    pub fn anchor_action_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.anchor_action_class.as_str())
    }

    /// Returns the unique completeness tokens observed across rows.
    pub fn completeness_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.preview_completeness_class.as_str())
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.evidence_class.as_str())
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.known_limit_class.as_str())
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        self.unique_tokens(|row| row.downgrade_automation_class.as_str())
    }

    fn unique_tokens(
        &self,
        project: impl Fn(&ResultArbitrationRow) -> &'static str,
    ) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(project(row));
        }
        set.into_iter().collect()
    }

    /// Builds a support export wrapping the exact packet shown to product
    /// surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> SemanticResultArbitrationTruthSupportExport {
        SemanticResultArbitrationTruthSupportExport {
            record_kind: SEMANTIC_RESULT_ARBITRATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            surface_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            result_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != SEMANTIC_RESULT_ARBITRATION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "result packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "result packet has the wrong schema version",
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
        if self.covered_surfaces.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingSurfaceCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered surface",
            ));
        }

        for surface in ResultSurfaceClass::REQUIRED {
            let present = self
                .rows
                .iter()
                .any(|row| row.result_surface_class == surface);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSurfaceCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers surface {}", surface.as_str()),
                ));
            }
        }
        for lane in ResultLaneClass::REQUIRED {
            let present = self.rows.iter().any(|row| row.result_lane_class == lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            self.append_per_row_findings(row, &mut findings);
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
        row: &ResultArbitrationRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                format!("row {} identity or timestamp is empty", row.row_id),
            ));
        }

        // Boundary discipline: no raw bodies, secrets, or ambient authority.
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

        // Binding discipline.
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
        if !row.acting_provider_family_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingProviderFamily,
                FindingSeverity::Blocker,
                format!(
                    "row {} must name a concrete acting provider family",
                    row.row_id
                ),
            ));
        }
        if !row.arbitration_basis_class.is_concrete() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingArbitrationBasis,
                FindingSeverity::Blocker,
                format!("row {} must bind a concrete arbitration basis", row.row_id),
            ));
        }
        if row.evidence_refs.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingEvidenceRefs,
                FindingSeverity::Blocker,
                format!("row {} carries no evidence refs", row.row_id),
            ));
        }

        if matches!(row.support_class, SupportClass::Certified) && !row.all_bindings_satisfied() {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedWithUnboundBinding,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims certified while a binding (support, provider family, arbitration basis, known limit, downgrade automation, or evidence) is unbound",
                    row.row_id
                ),
            ));
        }

        // Disclosure discipline.
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

        self.append_arbitration_findings(row, findings);
        self.append_fallback_banner_findings(row, findings);
        self.append_refactor_safety_findings(row, findings);

        if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
            && matches!(row.support_class, SupportClass::Certified)
        {
            findings.push(ValidationFinding::new(
                FindingKind::CertifiedAtLowConfidence,
                FindingSeverity::Warning,
                format!(
                    "row {} claims certified at low_confidence; narrowing until evidence grows",
                    row.row_id
                ),
            ));
        }
    }

    fn append_arbitration_findings(
        &self,
        row: &ResultArbitrationRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        // The losing provider and the downgrade reason must stay inspectable;
        // a disagreement must never collapse into ranking-only output.
        if row.has_disagreement() && row.alternate_provider_visibility_class.loses_alternates() {
            findings.push(ValidationFinding::new(
                FindingKind::LosingProviderCollapsed,
                FindingSeverity::Blocker,
                format!(
                    "row {} records a disagreement but collapses the losing provider into ranking-only output",
                    row.row_id
                ),
            ));
        }

        // A conflict that changes target identity, scope, or refactor safety
        // must surface a visible detail path.
        if row.disagreement_impact_class.changes_material()
            && (!row.disagreement_visibility_class.is_visible()
                || !row.inspector_route_class.is_inspectable())
        {
            findings.push(ValidationFinding::new(
                FindingKind::DisagreementDetailPathMissing,
                FindingSeverity::Blocker,
                format!(
                    "row {} changes target identity/scope/safety but offers no visible disagreement detail path",
                    row.row_id
                ),
            ));
        }

        // An opaque spinner can never stand in for a real inspection route.
        if matches!(
            row.inspector_route_class,
            InspectorRouteClass::OpaqueSpinner
        ) {
            findings.push(ValidationFinding::new(
                FindingKind::OpaqueInspectorRoute,
                FindingSeverity::Blocker,
                format!(
                    "row {} uses an opaque spinner in place of an inspection route",
                    row.row_id
                ),
            ));
        }

        // Materially conflicting results must not be silently fused into an
        // exact answer with no visible disagreement.
        if matches!(
            row.disagreement_impact_class,
            DisagreementImpactClass::TargetIdentityChanged
        ) && row.result_tier_class.is_exact()
            && !row.disagreement_visibility_class.is_visible()
        {
            findings.push(ValidationFinding::new(
                FindingKind::SilentFusionOfConflict,
                FindingSeverity::Blocker,
                format!(
                    "row {} silently fuses a target-identity conflict into an exact answer",
                    row.row_id
                ),
            ));
        }
    }

    fn append_fallback_banner_findings(
        &self,
        row: &ResultArbitrationRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        if row.result_tier_class.is_exact() {
            // An exact result must not carry a fallback banner or a lost
            // guarantee.
            if row.fallback_banner_class.is_present() || row.lost_guarantee_class.is_lost() {
                findings.push(ValidationFinding::new(
                    FindingKind::FallbackBannerOnExactResult,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is exact semantic yet carries a fallback banner or lost guarantee",
                        row.row_id
                    ),
                ));
            }
        } else {
            // A degraded result must preserve the guarantees that remain and
            // the guarantees that were lost.
            if !row.fallback_banner_class.is_present() || !row.lost_guarantee_class.is_lost() {
                findings.push(ValidationFinding::new(
                    FindingKind::FallbackBannerMissing,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} degraded below exact semantic but has no fallback banner or no recorded lost guarantee",
                        row.row_id
                    ),
                ));
            }
        }

        // A whole-workspace / all-results claim must not rest on lexical or
        // heuristic evidence only.
        if row.result_tier_class.is_lexical_or_heuristic()
            && row.claim_scope_class.is_whole_workspace()
        {
            findings.push(ValidationFinding::new(
                FindingKind::OverclaimedScopeOnLexicalEvidence,
                FindingSeverity::Blocker,
                format!(
                    "row {} claims whole-workspace/all-results scope on lexical or heuristic evidence",
                    row.row_id
                ),
            ));
        }

        // Whole-workspace wording must drop when roots / slices / edges were
        // skipped.
        if row.coverage_gap_class.is_gap() && row.claim_scope_class.is_whole_workspace() {
            findings.push(ValidationFinding::new(
                FindingKind::WholeWorkspaceWordingWithCoverageGap,
                FindingSeverity::Blocker,
                format!(
                    "row {} keeps whole-workspace wording after {} was skipped",
                    row.row_id,
                    row.coverage_gap_class.as_str()
                ),
            ));
        }

        // A text/lexical result may not advertise a retained semantic
        // guarantee.
        if matches!(row.result_tier_class, ResultTierClass::TextLexical)
            && row.retained_guarantee_class.is_semantic()
        {
            findings.push(ValidationFinding::new(
                FindingKind::RetainedGuaranteeOverstated,
                FindingSeverity::Blocker,
                format!(
                    "row {} is text/lexical yet claims a retained semantic guarantee",
                    row.row_id
                ),
            ));
        }
    }

    fn append_refactor_safety_findings(
        &self,
        row: &ResultArbitrationRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        // The launch-language refactor safety model still applies: a mutating
        // follow-up must bind typed preview completeness and a rollback
        // checkpoint, even for an M5-only artifact or framework pack.
        if row.anchor_action_class.is_mutating()
            && (!row.preview_completeness_class.is_concrete()
                || row.rollback_checkpoint_ref.is_none())
        {
            findings.push(ValidationFinding::new(
                FindingKind::MutatingAnchorBypassesPreview,
                FindingSeverity::Blocker,
                format!(
                    "row {} anchors a mutating follow-up without typed preview completeness and a rollback checkpoint",
                    row.row_id
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
pub struct SemanticResultArbitrationTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub surface_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub result_packet: SemanticResultArbitrationTruthPacket,
}

impl SemanticResultArbitrationTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == SEMANTIC_RESULT_ARBITRATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == SEMANTIC_RESULT_ARBITRATION_TRUTH_SCHEMA_VERSION
            && self.surface_packet_id_ref == self.result_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.result_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable result packet.
#[derive(Debug)]
pub enum SemanticResultArbitrationTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for SemanticResultArbitrationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(formatter, "result packet parse failed: {error}"),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "result packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for SemanticResultArbitrationTruthArtifactError {}

/// Returns the checked-in stable semantic-result arbitration truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or
/// validate.
pub fn current_stable_semantic_result_arbitration_truth_packet(
) -> Result<SemanticResultArbitrationTruthPacket, SemanticResultArbitrationTruthArtifactError> {
    let packet: SemanticResultArbitrationTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m5/semantic_result_arbitration_truth_packet.json"
    )))
    .map_err(SemanticResultArbitrationTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(SemanticResultArbitrationTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests;
