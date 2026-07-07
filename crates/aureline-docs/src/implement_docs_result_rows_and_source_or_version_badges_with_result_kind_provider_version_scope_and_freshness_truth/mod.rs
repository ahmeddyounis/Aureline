//! One reusable M5 docs-result-row / source-version-badge primitive: result kind,
//! source provider, derived source-badge class, version/package scope, symbol-match
//! confidence, freshness, freshness posture, and rank-reason disclosure, projected
//! the same way across every claimed M5 docs, AI-answer, onboarding, and support
//! knowledge surface.
//!
//! Aureline's frozen docs-browser component matrix
//! ([`crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`])
//! names the docs result row and the docs source/version badge as two governed
//! component families and freezes their controlled vocabulary — the match states,
//! the project-doc override reasons, the source providers, the freshness states, the
//! corpus classes, the version scopes, the docs surface families, the deployment
//! lines, the consumer surfaces, the accessibility routes, the qualification
//! classes, and the downgrade triggers. This module *implements* that result-row /
//! source-badge contract as one reusable primitive so a user can tell — from the
//! result row and its source/version badge alone — what kind of result they are
//! looking at, whether it is a local/project doc or an upstream/vendor doc, whether
//! its freshness reads as current or is explicitly cached, mirrored, or stale, and
//! why a project doc outranked vendor docs, before they open it, instead of that
//! truth drifting by docs browser, AI-answer citation, onboarding step, or support
//! answer.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_docs_result_row`] — that takes one result's title,
//!    result kind, corpus class, source provider, match state, override reason,
//!    symbol-match confidence, version scope, freshness, and open action, and
//!    produces one [`M5ResolvedDocsResultRow`] carrying the derived source-badge
//!    class (local-project versus workspace-spec versus first-party-reference versus
//!    cached/mirrored-reference versus live-vendor-upstream versus
//!    extension-contributed versus AI-derived), the derived freshness posture
//!    (current-live versus recently-synced versus cached/mirrored-explicit-not-live
//!    versus stale-flagged versus unknown) — never showing a cached, mirrored, or
//!    stale result as live — and, whenever project-doc precedence, version adjacency,
//!    or mirror freshness materially affects which result wins, a self-contained
//!    [`M5DocsRankReasonDisclosure`] that names the exact rank factor and override
//!    reason rather than silently reordering results.
//! 2. A parity matrix — [`M5DocsResultRowPrimitivePacket`] — that binds one row per
//!    claimed M5 docs-result consumer (the docs-browser result list, the AI-answer
//!    citation, the onboarding step reference, the support answer result, and the
//!    CLI result list) to the shared result-row anatomy, the same source-badge
//!    classes, freshness postures, match states, override reasons, rank factors,
//!    symbol-match confidences, export fields, and non-visual accessibility routes,
//!    so the source/provider/version/freshness vocabulary stays identical across the
//!    docs browser, AI answers, onboarding, support, and the CLI.
//!
//! The corpus class ([`M5DocsCorpusClass`]), version scope ([`M5DocsVersionScope`]),
//! source provider ([`M5DocsSourceProvider`]), freshness state
//! ([`M5DocsFreshnessState`]), match state ([`M5DocsMatchState`]), override reason
//! ([`M5DocsOverrideReason`]), docs surface family ([`M5DocsSurfaceFamily`]),
//! deployment line ([`M5DocsDeploymentLine`]), consumer surface
//! ([`M5DocsConsumerSurface`]), accessibility route ([`M5DocsAccessibilityRoute`]),
//! qualification class ([`M5DocsQualificationClass`]), and downgrade trigger
//! ([`M5DocsDowngradeTrigger`]) are reused verbatim from the frozen docs-browser
//! component matrix. This module mints new vocabulary only for what that matrix left
//! implicit about the result row and source/version badge themselves: their result
//! consumers, their anatomy parts, their result kinds, their derived source-badge
//! classes, their symbol-match confidences, their derived freshness postures, their
//! rank factors, and their export fields. No M5 docs surface invents a second
//! result-row grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and result bodies stay
//! outside the support boundary; every result title and open-action target is
//! carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_docs_result_row_primitive_ai_citation_preview_narrowed,
    seeded_m5_docs_result_row_primitive_onboarding_reference_beta_narrowed,
    seeded_m5_docs_result_row_primitive_packet, M5_DOCS_RESULT_ROW_PRIMITIVE_PACKET_ID,
};

// The corpus class, version scope, source provider, freshness state, match state,
// override reason, docs surface family, deployment line, consumer surface,
// accessibility routes, qualification classes, and downgrade triggers are frozen
// once, in the docs-browser component matrix. This primitive reuses them verbatim so
// it never invents a parallel result-row or source-badge vocabulary.
pub use crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix::{
    M5DocsAccessibilityRoute, M5DocsConsumerSurface, M5DocsCorpusClass, M5DocsDeploymentLine,
    M5DocsDowngradeTrigger, M5DocsFreshnessState, M5DocsMatchState, M5DocsOverrideReason,
    M5DocsQualificationClass, M5DocsSourceProvider, M5DocsSurfaceFamily, M5DocsVersionScope,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DocsResultRowPrimitivePacket`].
pub const M5_DOCS_RESULT_ROW_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_docs_result_rows_and_source_or_version_badges_with_result_kind_provider_version_scope_symbol_match_confidence_freshness_and_rank_reason_truth";

/// Schema version for M5 docs-result-row-primitive records.
pub const M5_DOCS_RESULT_ROW_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the docs-result-row / source-version-badge boundary schema.
pub const M5_DOCS_RESULT_ROW_SCHEMA_REF: &str =
    "schemas/docs/m5-docs-result-row-and-source-version-badge-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DOCS_RESULT_ROW_DOC_REF: &str =
    "docs/docs/m5/implement_docs_result_rows_and_source_or_version_badges_with_result_kind_provider_version_scope_and_freshness_truth.md";

/// Repo-relative path of the frozen docs-browser component matrix this primitive
/// narrows from.
pub const M5_DOCS_RESULT_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json";

/// Repo-relative path of the stable docs-source/result contract this primitive binds
/// against.
pub const M5_DOCS_RESULT_ROW_SOURCE_RESULT_REF: &str =
    "schemas/docs/stable_docs_source_result_pack_and_citation.schema.json";

/// Repo-relative path of the docs-source precedence / ranking-parity contract this
/// primitive keeps source/rank-reason truth consistent with.
pub const M5_DOCS_RESULT_ROW_SOURCE_PRECEDENCE_REF: &str =
    "schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_RESULT_ROW_FIXTURE_DIR: &str =
    "fixtures/docs/m5/m5-docs-result-row-and-source-version-badge-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_RESULT_ROW_ARTIFACT_REF: &str =
    "artifacts/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DOCS_RESULT_ROW_CSV_REF: &str =
    "artifacts/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DOCS_RESULT_ROW_REPORT_REF: &str =
    "artifacts/docs/m5/m5-docs-result-row-and-source-version-badge-primitive.md";

/// One claimed M5 docs-result consumer that renders the shared result row and its
/// source/version badge. These are the entrypoints the acceptance criteria name —
/// the docs browser, AI answers, onboarding, support, and the CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsResultConsumerSurface {
    /// The docs-browser result list.
    DocsBrowserResult,
    /// The AI-answer citation result.
    AiAnswerCitation,
    /// The onboarding step reference.
    OnboardingStepReference,
    /// The support answer result.
    SupportAnswerResult,
    /// The CLI / headless result list.
    CliResultList,
}

impl M5DocsResultConsumerSurface {
    /// Every claimed docs-result consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DocsBrowserResult,
        Self::AiAnswerCitation,
        Self::OnboardingStepReference,
        Self::SupportAnswerResult,
        Self::CliResultList,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserResult => "docs_browser_result",
            Self::AiAnswerCitation => "ai_answer_citation",
            Self::OnboardingStepReference => "onboarding_step_reference",
            Self::SupportAnswerResult => "support_answer_result",
            Self::CliResultList => "cli_result_list",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DocsBrowserResult => "Docs-Browser Result",
            Self::AiAnswerCitation => "AI-Answer Citation",
            Self::OnboardingStepReference => "Onboarding Step Reference",
            Self::SupportAnswerResult => "Support Answer Result",
            Self::CliResultList => "CLI Result List",
        }
    }
}

/// One anatomy part the shared result row / source-version badge surfaces. The parts
/// in [`M5DocsResultRowAnatomyPart::MANDATORY`] are required on every row so a user
/// can distinguish and act on the result before opening it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsResultRowAnatomyPart {
    /// The result title label.
    TitleLabel,
    /// The result-kind tag.
    ResultKindTag,
    /// The source-provider / source-badge.
    SourceProviderBadge,
    /// The version / package scope badge.
    VersionScopeBadge,
    /// The symbol-match-confidence cue.
    SymbolMatchConfidenceCue,
    /// The freshness badge.
    FreshnessBadge,
    /// The rank-reason disclosure (shown when ranking is materially overridden).
    RankReasonDisclosure,
    /// The open action.
    OpenAction,
}

impl M5DocsResultRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TitleLabel,
        Self::ResultKindTag,
        Self::SourceProviderBadge,
        Self::VersionScopeBadge,
        Self::SymbolMatchConfidenceCue,
        Self::FreshnessBadge,
        Self::RankReasonDisclosure,
        Self::OpenAction,
    ];

    /// The anatomy parts every result row must render before it is opened.
    pub const MANDATORY: [Self; 6] = [
        Self::TitleLabel,
        Self::ResultKindTag,
        Self::SourceProviderBadge,
        Self::VersionScopeBadge,
        Self::FreshnessBadge,
        Self::OpenAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TitleLabel => "title_label",
            Self::ResultKindTag => "result_kind_tag",
            Self::SourceProviderBadge => "source_provider_badge",
            Self::VersionScopeBadge => "version_scope_badge",
            Self::SymbolMatchConfidenceCue => "symbol_match_confidence_cue",
            Self::FreshnessBadge => "freshness_badge",
            Self::RankReasonDisclosure => "rank_reason_disclosure",
            Self::OpenAction => "open_action",
        }
    }
}

/// Controlled result kind — the shape of the documentation object a result row
/// represents, so a result row never leaves what kind of thing it points at
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsResultKind {
    /// A documentation page.
    DocPage,
    /// An API symbol / reference entry.
    ApiSymbolEntry,
    /// A guide / tutorial section.
    GuideSection,
    /// A code-symbol anchor (source-derived).
    CodeSymbolAnchor,
    /// A changelog / release-notes entry.
    ChangelogEntry,
    /// An example snippet.
    ExampleSnippet,
}

impl M5DocsResultKind {
    /// Every result kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DocPage,
        Self::ApiSymbolEntry,
        Self::GuideSection,
        Self::CodeSymbolAnchor,
        Self::ChangelogEntry,
        Self::ExampleSnippet,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocPage => "doc_page",
            Self::ApiSymbolEntry => "api_symbol_entry",
            Self::GuideSection => "guide_section",
            Self::CodeSymbolAnchor => "code_symbol_anchor",
            Self::ChangelogEntry => "changelog_entry",
            Self::ExampleSnippet => "example_snippet",
        }
    }
}

/// The derived source-badge class — the honest, non-color-only distinction a
/// source/version badge draws between a local/project doc, a workspace spec, a
/// first-party reference, a cached/mirrored reference, a live vendor/upstream doc, an
/// extension-contributed doc, and an AI-derived explanation. This is the resolver's
/// verdict about where a result actually comes from, so a user can tell local from
/// upstream at row level before opening it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSourceBadgeClass {
    /// Project-specific / local documentation.
    LocalProjectDocs,
    /// A workspace spec / source-derived symbol doc from this codebase.
    WorkspaceSpec,
    /// First-party hosted canonical reference.
    FirstPartyReference,
    /// A cached or mirrored reference copy.
    CachedMirroredReference,
    /// Live vendor / third-party upstream documentation.
    LiveVendorUpstream,
    /// Extension- / community-contributed documentation.
    ExtensionContributed,
    /// AI-derived explanation.
    AiDerivedExplanation,
}

impl M5DocsSourceBadgeClass {
    /// Every source-badge class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalProjectDocs,
        Self::WorkspaceSpec,
        Self::FirstPartyReference,
        Self::CachedMirroredReference,
        Self::LiveVendorUpstream,
        Self::ExtensionContributed,
        Self::AiDerivedExplanation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalProjectDocs => "local_project_docs",
            Self::WorkspaceSpec => "workspace_spec",
            Self::FirstPartyReference => "first_party_reference",
            Self::CachedMirroredReference => "cached_mirrored_reference",
            Self::LiveVendorUpstream => "live_vendor_upstream",
            Self::ExtensionContributed => "extension_contributed",
            Self::AiDerivedExplanation => "ai_derived_explanation",
        }
    }

    /// A short, color-independent glyph label so the badge distinction never relies
    /// on color alone.
    pub const fn glyph_label(self) -> &'static str {
        match self {
            Self::LocalProjectDocs => "[project]",
            Self::WorkspaceSpec => "[workspace]",
            Self::FirstPartyReference => "[first-party]",
            Self::CachedMirroredReference => "[cached/mirrored]",
            Self::LiveVendorUpstream => "[vendor]",
            Self::ExtensionContributed => "[extension]",
            Self::AiDerivedExplanation => "[ai-derived]",
        }
    }

    /// True when the badge class is a local or project-scoped source — the
    /// distinction a user needs before opening upstream/vendor docs.
    pub const fn is_local_or_project(self) -> bool {
        matches!(self, Self::LocalProjectDocs | Self::WorkspaceSpec)
    }
}

/// Controlled symbol-match confidence — how confidently a result resolves to the
/// symbol it claims, so a result row never presents a heuristic or unresolved symbol
/// match as an exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSymbolMatchConfidence {
    /// An exact symbol match.
    ExactSymbolMatch,
    /// A strong (high-confidence) match.
    StrongMatch,
    /// A partial match.
    PartialMatch,
    /// A heuristic / inferred match.
    HeuristicMatch,
    /// An unresolved symbol.
    UnresolvedSymbol,
    /// The result is not symbol-scoped.
    NotSymbolScoped,
}

impl M5DocsSymbolMatchConfidence {
    /// Every symbol-match confidence, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactSymbolMatch,
        Self::StrongMatch,
        Self::PartialMatch,
        Self::HeuristicMatch,
        Self::UnresolvedSymbol,
        Self::NotSymbolScoped,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSymbolMatch => "exact_symbol_match",
            Self::StrongMatch => "strong_match",
            Self::PartialMatch => "partial_match",
            Self::HeuristicMatch => "heuristic_match",
            Self::UnresolvedSymbol => "unresolved_symbol",
            Self::NotSymbolScoped => "not_symbol_scoped",
        }
    }
}

/// The derived freshness posture of a result — the resolver's verdict about whether
/// the result reads as current-live, recently-synced-current, cached or mirrored
/// (explicit, never live), stale, or unknown. A cached, mirrored, or stale result is
/// never shown as live even when its declared freshness would suggest it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsResultFreshnessPosture {
    /// Live and current.
    CurrentLive,
    /// Recently synced and treated as current.
    RecentlySyncedCurrent,
    /// A cached copy, shown explicitly and never as live.
    CachedExplicitNotLive,
    /// A mirrored copy, shown explicitly and never as live.
    MirroredExplicitNotLive,
    /// Stale / expired, flagged explicitly.
    StaleFlagged,
    /// Freshness unknown / not evaluated.
    FreshnessUnknown,
}

impl M5DocsResultFreshnessPosture {
    /// Every freshness posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CurrentLive,
        Self::RecentlySyncedCurrent,
        Self::CachedExplicitNotLive,
        Self::MirroredExplicitNotLive,
        Self::StaleFlagged,
        Self::FreshnessUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLive => "current_live",
            Self::RecentlySyncedCurrent => "recently_synced_current",
            Self::CachedExplicitNotLive => "cached_explicit_not_live",
            Self::MirroredExplicitNotLive => "mirrored_explicit_not_live",
            Self::StaleFlagged => "stale_flagged",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }

    /// True when the result reads as live/current (live or recently-synced-current).
    pub const fn is_live_current(self) -> bool {
        matches!(self, Self::CurrentLive | Self::RecentlySyncedCurrent)
    }

    /// True when the result is a cached or mirrored copy shown explicitly, never as
    /// live.
    pub const fn is_explicit_not_live(self) -> bool {
        matches!(
            self,
            Self::CachedExplicitNotLive | Self::MirroredExplicitNotLive
        )
    }

    /// True when the result is stale or of unknown freshness.
    pub const fn is_stale_or_unknown(self) -> bool {
        matches!(self, Self::StaleFlagged | Self::FreshnessUnknown)
    }
}

/// The material rank factor that decided which docs result won, so a result row
/// never silently reorders results without saying why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsRankFactor {
    /// A project doc took precedence over vendor/upstream docs.
    ProjectDocPrecedence,
    /// A nearby / adjacent version match materially affected ranking.
    VersionAdjacency,
    /// Local freshness beat a staler mirror / source.
    MirrorFreshness,
    /// An explicit user preference decided the ranking.
    ExplicitPreference,
    /// The vendor source was unavailable and a fallback source ranked.
    VendorFallback,
    /// A policy-scoped rule decided the ranking.
    PolicyScopedRanking,
}

impl M5DocsRankFactor {
    /// Every rank factor, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProjectDocPrecedence,
        Self::VersionAdjacency,
        Self::MirrorFreshness,
        Self::ExplicitPreference,
        Self::VendorFallback,
        Self::PolicyScopedRanking,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocPrecedence => "project_doc_precedence",
            Self::VersionAdjacency => "version_adjacency",
            Self::MirrorFreshness => "mirror_freshness",
            Self::ExplicitPreference => "explicit_preference",
            Self::VendorFallback => "vendor_fallback",
            Self::PolicyScopedRanking => "policy_scoped_ranking",
        }
    }

    /// Review-safe phrase for the rank-reason disclosure headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ProjectDocPrecedence => "a project doc took precedence over vendor docs",
            Self::VersionAdjacency => "a nearby version match materially affected the ranking",
            Self::MirrorFreshness => "local freshness outranked a staler mirror",
            Self::ExplicitPreference => "an explicit user preference decided the ranking",
            Self::VendorFallback => "the vendor source was unavailable and a fallback ranked",
            Self::PolicyScopedRanking => "a policy-scoped rule decided the ranking",
        }
    }
}

/// A field the support / export packet carries so result-row and source/version-badge
/// truth is reconstructable from the shared model. The fields in
/// [`M5DocsResultRowExportField::MANDATORY`] are required so the badge/state
/// vocabulary stays stable across UI, docs/help, exports, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsResultRowExportField {
    /// The result kind.
    ResultKind,
    /// The derived source-badge class.
    SourceBadgeClass,
    /// The source provider.
    SourceProvider,
    /// The corpus class.
    CorpusClass,
    /// The version / package scope.
    VersionScope,
    /// The symbol-match confidence.
    SymbolMatchConfidence,
    /// The declared freshness state.
    FreshnessState,
    /// The derived freshness posture.
    FreshnessPosture,
    /// The rank factor (when ranking is materially overridden).
    RankFactor,
    /// The override reason.
    OverrideReason,
}

impl M5DocsResultRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ResultKind,
        Self::SourceBadgeClass,
        Self::SourceProvider,
        Self::CorpusClass,
        Self::VersionScope,
        Self::SymbolMatchConfidence,
        Self::FreshnessState,
        Self::FreshnessPosture,
        Self::RankFactor,
        Self::OverrideReason,
    ];

    /// The export fields every result-row export must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::ResultKind,
        Self::SourceBadgeClass,
        Self::SourceProvider,
        Self::VersionScope,
        Self::FreshnessState,
        Self::FreshnessPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResultKind => "result_kind",
            Self::SourceBadgeClass => "source_badge_class",
            Self::SourceProvider => "source_provider",
            Self::CorpusClass => "corpus_class",
            Self::VersionScope => "version_scope",
            Self::SymbolMatchConfidence => "symbol_match_confidence",
            Self::FreshnessState => "freshness_state",
            Self::FreshnessPosture => "freshness_posture",
            Self::RankFactor => "rank_factor",
            Self::OverrideReason => "override_reason",
        }
    }
}

/// A self-contained rank-reason disclosure: the material rank factor, the override
/// reason, the version scope, and the source-badge class, so a result whose ranking
/// was materially decided by project-doc precedence, version adjacency, or mirror
/// freshness is understood from the disclosure alone rather than reordered silently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsRankReasonDisclosure {
    /// The material rank factor.
    pub rank_factor: M5DocsRankFactor,
    /// The override reason behind the ranking.
    pub override_reason: M5DocsOverrideReason,
    /// The version / package scope the winning result is bound to.
    pub version_scope: M5DocsVersionScope,
    /// The source-badge class of the winning result.
    pub source_badge_class: M5DocsSourceBadgeClass,
    /// A deterministic, self-contained headline naming the rank factor, the override
    /// reason, the badge class, and the version scope.
    pub headline: String,
}

/// The full input to the docs-result-row resolver for one result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsResultRowResolutionInput {
    /// The opaque, export-safe result title.
    pub title_repr: String,
    /// The result kind.
    pub result_kind: M5DocsResultKind,
    /// The corpus class the result belongs to.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider behind the result.
    pub source_provider: M5DocsSourceProvider,
    /// The match state relating the result to the query and local corpus.
    pub match_state: M5DocsMatchState,
    /// The project-doc override reason (or no-override) behind the ranking.
    pub override_reason: M5DocsOverrideReason,
    /// The symbol-match confidence.
    pub symbol_match_confidence: M5DocsSymbolMatchConfidence,
    /// The version / package scope in effect.
    pub version_scope: M5DocsVersionScope,
    /// The declared freshness state.
    pub freshness_state: M5DocsFreshnessState,
    /// The opaque, export-safe open-action target. Must be non-empty so the row is
    /// actionable.
    pub open_action_target_repr: String,
}

/// The resolved result-kind / source / version / freshness / rank truth for one
/// result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDocsResultRow {
    /// The opaque result title.
    pub title_repr: String,
    /// The result kind.
    pub result_kind: M5DocsResultKind,
    /// The corpus class.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider.
    pub source_provider: M5DocsSourceProvider,
    /// The match state.
    pub match_state: M5DocsMatchState,
    /// The override reason.
    pub override_reason: M5DocsOverrideReason,
    /// The symbol-match confidence.
    pub symbol_match_confidence: M5DocsSymbolMatchConfidence,
    /// The version / package scope.
    pub version_scope: M5DocsVersionScope,
    /// The declared freshness state.
    pub freshness_state: M5DocsFreshnessState,
    /// The opaque open-action target.
    pub open_action_target_repr: String,
    /// The derived source-badge class.
    pub source_badge_class: M5DocsSourceBadgeClass,
    /// True when the source-badge class is local / project-scoped.
    pub is_local_or_project: bool,
    /// The derived freshness posture.
    pub freshness_posture: M5DocsResultFreshnessPosture,
    /// True when the result reads as live/current (never true for cached, mirrored,
    /// or stale results).
    pub shows_as_live: bool,
    /// The rank-reason disclosure, present when the ranking was materially decided.
    pub rank_reason_disclosure: Option<M5DocsRankReasonDisclosure>,
}

/// Errors returned by [`resolve_docs_result_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DocsResultRowResolutionError {
    /// The result title was empty.
    EmptyTitle,
    /// The open-action target was empty (the row must be actionable).
    EmptyOpenActionTarget,
    /// A result title or open-action target carried forbidden material.
    ForbiddenResultMaterial,
}

impl M5DocsResultRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyTitle => "empty_title",
            Self::EmptyOpenActionTarget => "empty_open_action_target",
            Self::ForbiddenResultMaterial => "forbidden_result_material",
        }
    }
}

impl fmt::Display for M5DocsResultRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "docs-result-row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DocsResultRowResolutionError {}

/// Resolves one docs result row from its declared state.
///
/// The derived source-badge class is computed in a fixed, specific-first order: an
/// AI-derived provider reads as an AI-derived explanation, then a project-specific
/// scope reads as local project docs, then a codebase-symbol corpus reads as a
/// workspace spec, then a community-contributed corpus reads as extension-contributed,
/// then a vendor-dependency corpus or third-party provider reads as live vendor
/// upstream, then a mirrored, offline-import, or bundled-local provider reads as a
/// cached/mirrored reference, and a first-party hosted provider reads as a first-party
/// reference. The freshness posture keeps a cached, mirrored, or stale result explicit
/// rather than shown as live, and a materially overridden ranking always produces a
/// self-contained rank-reason disclosure rather than a silent reorder.
pub fn resolve_docs_result_row(
    input: &M5DocsResultRowResolutionInput,
) -> Result<M5ResolvedDocsResultRow, M5DocsResultRowResolutionError> {
    if input.title_repr.trim().is_empty() {
        return Err(M5DocsResultRowResolutionError::EmptyTitle);
    }
    if input.open_action_target_repr.trim().is_empty() {
        return Err(M5DocsResultRowResolutionError::EmptyOpenActionTarget);
    }
    if value_repr_is_forbidden(&input.title_repr)
        || value_repr_is_forbidden(&input.open_action_target_repr)
    {
        return Err(M5DocsResultRowResolutionError::ForbiddenResultMaterial);
    }

    let source_badge_class = derive_source_badge_class(
        input.source_provider,
        input.corpus_class,
        input.version_scope,
    );
    let is_local_or_project = source_badge_class.is_local_or_project();

    let freshness_posture = derive_freshness_posture(input.freshness_state, input.match_state);
    let shows_as_live = freshness_posture.is_live_current();

    let rank_reason_disclosure = derive_rank_factor(input.override_reason, input.version_scope)
        .map(|rank_factor| {
            let headline = format!(
                "Ranked here because {} — {} {} result in {} scope (override: {})",
                rank_factor.phrase(),
                source_badge_class.glyph_label(),
                source_badge_class.as_str(),
                input.version_scope.as_str(),
                input.override_reason.as_str()
            );
            M5DocsRankReasonDisclosure {
                rank_factor,
                override_reason: input.override_reason,
                version_scope: input.version_scope,
                source_badge_class,
                headline,
            }
        });

    Ok(M5ResolvedDocsResultRow {
        title_repr: input.title_repr.clone(),
        result_kind: input.result_kind,
        corpus_class: input.corpus_class,
        source_provider: input.source_provider,
        match_state: input.match_state,
        override_reason: input.override_reason,
        symbol_match_confidence: input.symbol_match_confidence,
        version_scope: input.version_scope,
        freshness_state: input.freshness_state,
        open_action_target_repr: input.open_action_target_repr.clone(),
        source_badge_class,
        is_local_or_project,
        freshness_posture,
        shows_as_live,
        rank_reason_disclosure,
    })
}

/// The fixed, specific-first source-badge-class ladder.
fn derive_source_badge_class(
    provider: M5DocsSourceProvider,
    corpus: M5DocsCorpusClass,
    scope: M5DocsVersionScope,
) -> M5DocsSourceBadgeClass {
    use M5DocsCorpusClass as Corpus;
    use M5DocsSourceProvider as Provider;
    use M5DocsVersionScope as Scope;

    if matches!(provider, Provider::AiDerived) {
        M5DocsSourceBadgeClass::AiDerivedExplanation
    } else if matches!(scope, Scope::ProjectSpecific) {
        M5DocsSourceBadgeClass::LocalProjectDocs
    } else if matches!(corpus, Corpus::CodebaseSymbol) {
        M5DocsSourceBadgeClass::WorkspaceSpec
    } else if matches!(corpus, Corpus::CommunityContributed) {
        M5DocsSourceBadgeClass::ExtensionContributed
    } else if matches!(corpus, Corpus::VendorDependency)
        || matches!(provider, Provider::ThirdPartyHosted)
    {
        M5DocsSourceBadgeClass::LiveVendorUpstream
    } else if matches!(
        provider,
        Provider::MirroredRegistry | Provider::OfflineImport | Provider::BundledLocal
    ) {
        M5DocsSourceBadgeClass::CachedMirroredReference
    } else {
        M5DocsSourceBadgeClass::FirstPartyReference
    }
}

/// The freshness-posture ladder: a cached, mirrored, or stale match is never shown as
/// live even when the declared freshness would suggest it.
fn derive_freshness_posture(
    freshness: M5DocsFreshnessState,
    match_state: M5DocsMatchState,
) -> M5DocsResultFreshnessPosture {
    use M5DocsFreshnessState as Fresh;
    use M5DocsMatchState as Match;

    match freshness {
        Fresh::LiveCurrent => match match_state {
            Match::CachedMatch => M5DocsResultFreshnessPosture::CachedExplicitNotLive,
            Match::MirroredMatch => M5DocsResultFreshnessPosture::MirroredExplicitNotLive,
            Match::StaleMatch => M5DocsResultFreshnessPosture::StaleFlagged,
            _ => M5DocsResultFreshnessPosture::CurrentLive,
        },
        Fresh::RecentlySynced => M5DocsResultFreshnessPosture::RecentlySyncedCurrent,
        Fresh::CachedOffline => match match_state {
            Match::MirroredMatch => M5DocsResultFreshnessPosture::MirroredExplicitNotLive,
            _ => M5DocsResultFreshnessPosture::CachedExplicitNotLive,
        },
        Fresh::StaleExpired => M5DocsResultFreshnessPosture::StaleFlagged,
        Fresh::UnknownFreshness => M5DocsResultFreshnessPosture::FreshnessUnknown,
    }
}

/// The rank-factor derivation: an explicit override reason maps to its factor, a
/// no-override nearby-version match reads as version adjacency, and a plain
/// no-override default ranking produces no disclosure.
fn derive_rank_factor(
    override_reason: M5DocsOverrideReason,
    version_scope: M5DocsVersionScope,
) -> Option<M5DocsRankFactor> {
    use M5DocsOverrideReason as Override;
    Some(match override_reason {
        Override::ProjectPinnedOverride => M5DocsRankFactor::ProjectDocPrecedence,
        Override::LocalFreshnessOverride => M5DocsRankFactor::MirrorFreshness,
        Override::ExplicitUserPreference => M5DocsRankFactor::ExplicitPreference,
        Override::VendorSourceUnavailable => M5DocsRankFactor::VendorFallback,
        Override::PolicyScopedOverride => M5DocsRankFactor::PolicyScopedRanking,
        Override::NoOverride => {
            if matches!(version_scope, M5DocsVersionScope::NearbyVersion) {
                M5DocsRankFactor::VersionAdjacency
            } else {
                return None;
            }
        }
    })
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs result-row truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsResultRowResolutionCase {
    /// The resolver input.
    pub input: M5DocsResultRowResolutionInput,
    /// The resolved truth. Must equal `resolve_docs_result_row(&input)`.
    pub resolved: M5ResolvedDocsResultRow,
}

impl M5DocsResultRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DocsResultRowResolutionInput) -> Self {
        let resolved = resolve_docs_result_row(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_docs_result_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one docs-result consumer bound to the shared
/// result-row anatomy, source-badge classes, freshness postures, match states,
/// override reasons, rank factors, symbol-match confidences, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsResultRow {
    /// Docs-result consumer family.
    pub consumer_surface: M5DocsResultConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5DocsQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 docs surface families that render / consume this row.
    pub surface_families: Vec<M5DocsSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5DocsDeploymentLine>,
    /// Anatomy parts this row renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5DocsResultRowAnatomyPart>,
    /// Result kinds this row distinguishes.
    pub result_kinds: Vec<M5DocsResultKind>,
    /// Corpus classes this row names.
    pub corpus_classes: Vec<M5DocsCorpusClass>,
    /// Source providers this row names.
    pub source_providers: Vec<M5DocsSourceProvider>,
    /// Source-badge classes this row distinguishes.
    pub source_badge_classes: Vec<M5DocsSourceBadgeClass>,
    /// Match states this row distinguishes.
    pub match_states: Vec<M5DocsMatchState>,
    /// Override reasons this row names.
    pub override_reasons: Vec<M5DocsOverrideReason>,
    /// Rank factors this row names.
    pub rank_factors: Vec<M5DocsRankFactor>,
    /// Symbol-match confidences this row distinguishes.
    pub symbol_match_confidences: Vec<M5DocsSymbolMatchConfidence>,
    /// Version scopes this row names.
    pub version_scopes: Vec<M5DocsVersionScope>,
    /// Freshness states this row discloses.
    pub freshness_states: Vec<M5DocsFreshnessState>,
    /// Freshness postures this row distinguishes.
    pub freshness_postures: Vec<M5DocsResultFreshnessPosture>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5DocsResultRowExportField>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5DocsAccessibilityRoute>,
    /// Docs subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5DocsConsumerSurface>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5DocsDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5DocsResultRowResolutionCase>,
    /// Hard invariant: this row never masks the source provider or the version scope.
    /// MUST be `false`.
    pub masks_source_or_version: bool,
    /// Hard invariant: this row never shows a cached, mirrored, or stale result as
    /// live. MUST be `false`.
    pub shows_cached_or_stale_as_live: bool,
    /// Hard invariant: this row never invents a private result grammar. MUST be
    /// `false`.
    pub invents_private_result_grammar: bool,
    /// Hard invariant: this row never hides the rank reason behind a silent reorder.
    /// MUST be `false`.
    pub hides_rank_reason: bool,
}

impl M5DocsResultRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DocsResultRowAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DocsResultRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DocsResultRowExportField> =
            self.export_fields.iter().copied().collect();
        M5DocsResultRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_source_or_version
            && !self.shows_cached_or_stale_as_live
            && !self.invents_private_result_grammar
            && !self.hides_rank_reason
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsResultRowVocabularySet {
    /// Docs-result consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Result-kind tokens.
    pub result_kinds: Vec<String>,
    /// Source-badge-class tokens.
    pub source_badge_classes: Vec<String>,
    /// Symbol-match-confidence tokens.
    pub symbol_match_confidences: Vec<String>,
    /// Freshness-posture tokens.
    pub freshness_postures: Vec<String>,
    /// Rank-factor tokens.
    pub rank_factors: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Corpus-class tokens (reused from the frozen matrix).
    pub corpus_classes: Vec<String>,
    /// Version-scope tokens (reused from the frozen matrix).
    pub version_scopes: Vec<String>,
    /// Source-provider tokens (reused from the frozen matrix).
    pub source_providers: Vec<String>,
    /// Match-state tokens (reused from the frozen matrix).
    pub match_states: Vec<String>,
    /// Override-reason tokens (reused from the frozen matrix).
    pub override_reasons: Vec<String>,
    /// Freshness-state tokens (reused from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5DocsResultRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5DocsResultConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DocsResultRowAnatomyPart::ALL, |v| v.as_str()),
            result_kinds: tokens(&M5DocsResultKind::ALL, |v| v.as_str()),
            source_badge_classes: tokens(&M5DocsSourceBadgeClass::ALL, |v| v.as_str()),
            symbol_match_confidences: tokens(&M5DocsSymbolMatchConfidence::ALL, |v| v.as_str()),
            freshness_postures: tokens(&M5DocsResultFreshnessPosture::ALL, |v| v.as_str()),
            rank_factors: tokens(&M5DocsRankFactor::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DocsResultRowExportField::ALL, |v| v.as_str()),
            corpus_classes: tokens(&M5DocsCorpusClass::ALL, |v| v.as_str()),
            version_scopes: tokens(&M5DocsVersionScope::ALL, |v| v.as_str()),
            source_providers: tokens(&M5DocsSourceProvider::ALL, |v| v.as_str()),
            match_states: tokens(&M5DocsMatchState::ALL, |v| v.as_str()),
            override_reasons: tokens(&M5DocsOverrideReason::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5DocsFreshnessState::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5DocsAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5DocsResultRowGovernanceReview {
    /// One result-row primitive carries kind, source, version, and freshness truth on
    /// every consumer.
    pub one_primitive_carries_result_truth: bool,
    /// The source provider and version scope are shown before any result is opened.
    pub source_and_version_always_shown: bool,
    /// Local / project docs are distinguishable from upstream / vendor docs at row
    /// level.
    pub local_vs_upstream_distinguishable_at_row_level: bool,
    /// A cached, mirrored, or stale result is never shown as live.
    pub cached_or_stale_never_shown_as_live: bool,
    /// Version / freshness state stays visible wherever a result is reused.
    pub version_freshness_visible_on_every_reuse: bool,
    /// The rank reason stays inspectable when ranking is materially overridden.
    pub rank_reason_stays_inspectable: bool,
    /// The badge / state vocabulary stays stable across UI, docs/help, exports, and
    /// support packets.
    pub badge_state_vocabulary_stable_across_surfaces: bool,
    /// The support / export packet reconstructs result-row truth.
    pub support_export_reconstructs_result_truth: bool,
    /// No consumer invents a second result-row grammar.
    pub no_surface_invents_second_result_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel result-row vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsResultRowConsumerProjection {
    /// Docs-browser, AI-answer, onboarding, support, and CLI consumers all consume the
    /// shared primitive.
    pub result_surfaces_consume_shared_primitive: bool,
    /// The source-badge class reads a single canonical source.
    pub source_badge_reads_single_source: bool,
    /// The freshness posture reads a single canonical source.
    pub freshness_posture_reads_single_source: bool,
    /// The rank-reason disclosure reads a single canonical source.
    pub rank_reason_reads_single_source: bool,
    /// Support / export reads a single canonical result-row source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsResultRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the result-row primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsResultRowReleasePosture {
    /// Ref of the supporting proof packet.
    pub proof_packet_ref: String,
    /// Ref of the supporting result-row audit.
    pub result_row_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DocsResultRowPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsResultRowPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Result rows.
    pub result_rows: Vec<M5DocsResultRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsResultRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsResultRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsResultRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsResultRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsResultRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 docs-result-row-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsResultRowPrimitivePacket {
    /// Record kind; must equal [`M5_DOCS_RESULT_ROW_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_RESULT_ROW_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Result rows.
    pub result_rows: Vec<M5DocsResultRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsResultRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsResultRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsResultRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsResultRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsResultRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsResultRowPrimitivePacket {
    /// Builds an M5 docs-result-row-primitive packet from stable-lane input.
    pub fn new(input: M5DocsResultRowPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_RESULT_ROW_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_RESULT_ROW_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            result_rows: input.result_rows,
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

    /// Validates the M5 docs-result-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5DocsResultRowPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_RESULT_ROW_PRIMITIVE_RECORD_KIND {
            violations.push(M5DocsResultRowPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_RESULT_ROW_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5DocsResultRowPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsResultRowPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_result_rows(self, &mut violations);
        validate_local_vs_upstream_coverage(self, &mut violations);
        validate_freshness_visibility(self, &mut violations);
        validate_rank_reason_inspectable(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 docs-result-row primitive packet serializes"),
        ) {
            violations.push(M5DocsResultRowPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 docs-result-row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per docs-result consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,result_kinds,source_badge_classes,freshness_postures,match_states,rank_factors,export_fields,example_count\n",
        );
        for row in &self.result_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.result_kinds, |v| v.as_str()),
                join_tokens(&row.source_badge_classes, |v| v.as_str()),
                join_tokens(&row.freshness_postures, |v| v.as_str()),
                join_tokens(&row.match_states, |v| v.as_str()),
                join_tokens(&row.rank_factors, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .result_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Docs-Result-Row and Source-Version-Badge Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Docs-result consumers: {} ({} stable)\n",
            self.result_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Source-badge classes: {}\n",
            self.vocabulary_set.source_badge_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Freshness postures: {}\n",
            self.vocabulary_set.freshness_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Rank factors: {}\n",
            self.vocabulary_set.rank_factors.join(", ")
        ));
        out.push_str(&format!(
            "- Result kinds: {}\n",
            self.vocabulary_set.result_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Docs-result consumers\n\n");
        for row in &self.result_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let rank = match &case.resolved.rank_reason_disclosure {
                    Some(disclosure) => disclosure.rank_factor.as_str(),
                    None => "default_ranking",
                };
                out.push_str(&format!(
                    "    - `{}` → badge `{}` (kind `{}`, posture `{}`, confidence `{}`, rank `{}`)\n",
                    case.resolved.title_repr,
                    case.resolved.source_badge_class.as_str(),
                    case.resolved.result_kind.as_str(),
                    case.resolved.freshness_posture.as_str(),
                    case.resolved.symbol_match_confidence.as_str(),
                    rank
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 docs-result-row-primitive export.
#[derive(Debug)]
pub enum M5DocsResultRowPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsResultRowPrimitiveViolation>),
}

impl fmt::Display for M5DocsResultRowPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 docs-result-row primitive export parse failed: {error}"
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
                    "m5 docs-result-row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsResultRowPrimitiveArtifactError {}

/// Validation failures emitted by [`M5DocsResultRowPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsResultRowPrimitiveViolation {
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
    /// A required docs-result consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A result row is incomplete.
    ResultRowIncomplete,
    /// A result row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A result row declares no result kinds.
    ResultKindMissing,
    /// A result row declares no source-badge classes.
    SourceBadgeClassMissing,
    /// A result row declares no freshness postures.
    FreshnessPostureMissing,
    /// A result row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A result row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A result row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A result row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A result row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A result row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves both a local/project and an upstream/vendor result.
    LocalVsUpstreamCoverageUnproven,
    /// No worked resolution proves both a live and a not-live freshness posture.
    FreshnessVisibilityUnproven,
    /// No worked resolution proves a materially overridden ranking with an
    /// inspectable rank-reason disclosure.
    RankReasonInspectableUnproven,
    /// A result row violates a hard invariant.
    ResultInvariantViolated,
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

impl M5DocsResultRowPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ResultRowIncomplete => "result_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::ResultKindMissing => "result_kind_missing",
            Self::SourceBadgeClassMissing => "source_badge_class_missing",
            Self::FreshnessPostureMissing => "freshness_posture_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::LocalVsUpstreamCoverageUnproven => "local_vs_upstream_coverage_unproven",
            Self::FreshnessVisibilityUnproven => "freshness_visibility_unproven",
            Self::RankReasonInspectableUnproven => "rank_reason_inspectable_unproven",
            Self::ResultInvariantViolated => "result_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 docs-result-row-primitive export.
pub fn current_stable_m5_docs_result_row_primitive_export(
) -> Result<M5DocsResultRowPrimitivePacket, M5DocsResultRowPrimitiveArtifactError> {
    let packet: M5DocsResultRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/support_export.json"
    )))
    .map_err(M5DocsResultRowPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsResultRowPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_RESULT_ROW_SCHEMA_REF,
        M5_DOCS_RESULT_ROW_DOC_REF,
        M5_DOCS_RESULT_ROW_COMPONENT_MATRIX_REF,
        M5_DOCS_RESULT_ROW_SOURCE_RESULT_REF,
        M5_DOCS_RESULT_ROW_SOURCE_PRECEDENCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsResultRowPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DocsResultRowPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_result_rows(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5DocsResultConsumerSurface> = packet
        .result_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5DocsResultConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5DocsResultRowPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.result_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.corpus_classes.is_empty()
            || row.source_providers.is_empty()
            || row.version_scopes.is_empty()
            || row.freshness_states.is_empty()
            || row.match_states.is_empty()
            || row.override_reasons.is_empty()
            || row.rank_factors.is_empty()
            || row.symbol_match_confidences.is_empty()
        {
            violations.push(M5DocsResultRowPrimitiveViolation::ResultRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DocsResultRowPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.result_kinds.is_empty() {
            violations.push(M5DocsResultRowPrimitiveViolation::ResultKindMissing);
        }
        if row.source_badge_classes.is_empty() {
            violations.push(M5DocsResultRowPrimitiveViolation::SourceBadgeClassMissing);
        }
        if row.freshness_postures.is_empty() {
            violations.push(M5DocsResultRowPrimitiveViolation::FreshnessPostureMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DocsResultRowPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5DocsAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5DocsResultRowPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DocsResultRowPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DocsResultRowPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5DocsResultRowPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DocsResultRowPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DocsResultRowPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DocsResultRowPrimitiveViolation::ResultInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove a local/project result
/// and at least one must prove an upstream/vendor result — the acceptance-criterion
/// example that a user can distinguish local/project docs from upstream/vendor docs at
/// row level before opening them.
fn validate_local_vs_upstream_coverage(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    let has_local = packet.result_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_local_or_project)
    });
    let has_upstream = packet.result_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| !case.resolved.is_local_or_project)
    });
    if !(has_local && has_upstream) {
        violations.push(M5DocsResultRowPrimitiveViolation::LocalVsUpstreamCoverageUnproven);
    }
}

/// At least one worked resolution must prove a live/current result and at least one a
/// not-live (cached, mirrored, stale, or unknown) result — the acceptance-criterion
/// example that version / freshness state stays visible wherever a result is reused.
fn validate_freshness_visibility(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    let has_live = packet.result_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.freshness_posture.is_live_current())
    });
    let has_not_live = packet.result_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.freshness_posture.is_explicit_not_live()
                || case.resolved.freshness_posture.is_stale_or_unknown()
        })
    });
    if !(has_live && has_not_live) {
        violations.push(M5DocsResultRowPrimitiveViolation::FreshnessVisibilityUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a materially overridden
/// ranking whose rank-reason disclosure carries a specific factor, an override reason,
/// and a non-empty headline — the acceptance-criterion example that a rank reason
/// stays inspectable rather than reordering results silently.
fn validate_rank_reason_inspectable(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    let proven = packet.result_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved
                .rank_reason_disclosure
                .as_ref()
                .is_some_and(|disclosure| !disclosure.headline.trim().is_empty())
        })
    });
    if !proven {
        violations.push(M5DocsResultRowPrimitiveViolation::RankReasonInspectableUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_result_truth,
        review.source_and_version_always_shown,
        review.local_vs_upstream_distinguishable_at_row_level,
        review.cached_or_stale_never_shown_as_live,
        review.version_freshness_visible_on_every_reuse,
        review.rank_reason_stays_inspectable,
        review.badge_state_vocabulary_stable_across_surfaces,
        review.support_export_reconstructs_result_truth,
        review.no_surface_invents_second_result_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DocsResultRowPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.result_surfaces_consume_shared_primitive,
        projection.source_badge_reads_single_source,
        projection.freshness_posture_reads_single_source,
        projection.rank_reason_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DocsResultRowPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsResultRowPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DocsResultRowPrimitivePacket,
    violations: &mut Vec<M5DocsResultRowPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.result_row_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DocsResultRowPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces
/// a stray comma.
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
