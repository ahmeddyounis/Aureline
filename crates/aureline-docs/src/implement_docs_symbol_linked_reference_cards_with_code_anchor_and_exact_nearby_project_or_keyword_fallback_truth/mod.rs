//! One reusable M5 symbol-linked reference-card primitive: the initiating
//! file/symbol code anchor, the symbol anchor kind, the derived symbol-linkage
//! strength (exact symbol match, nearby version match, project-specific override, or
//! keyword fallback), the source provider, the version/package scope, the cited
//! source revision, and the derived freshness posture, projected the same way across
//! every claimed M5 editor-hover/peek, docs-browser, AI-explanation, onboarding, and
//! support knowledge surface Aureline jumps to when it goes from code to docs.
//!
//! Aureline's frozen docs-browser component matrix
//! ([`crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`])
//! names the symbol-linked reference card as one governed component family and
//! freezes its controlled vocabulary — the symbol anchors, the match states, the
//! project-doc override reasons, the source providers, the freshness states, the
//! corpus classes, the version scopes, the docs surface families, the deployment
//! lines, the consumer surfaces, the accessibility routes, the qualification classes,
//! and the downgrade triggers. This module *implements* that reference-card contract
//! as one reusable primitive so a user can tell — from the card alone — which
//! file/symbol the card was opened *from*, how strong the symbol linkage actually is
//! (an exact symbol match, a nearby version match, a project-specific override, or a
//! keyword fallback), where the doc comes from, whether its cited revision reads as
//! current or is explicitly cached, mirrored, or stale, and why the card appeared,
//! instead of that truth drifting by editor hover, peek, docs browser, AI
//! explanation, onboarding step, or support evidence path.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_reference_card`] — that takes one card's title, the
//!    initiating file and symbol it was opened from, its symbol anchor, corpus class,
//!    source provider, match state, override reason, version scope, freshness, cited
//!    source revision, and open action, and produces one
//!    [`M5ResolvedDocsReferenceCard`] carrying the derived symbol-linkage strength
//!    (exact-symbol versus nearby-version versus project-specific versus
//!    keyword-fallback versus heuristic versus unresolved-no-linkage) — never showing
//!    an unresolved anchor as an exact symbol match — the derived freshness posture
//!    (current-live versus recently-synced versus cached/mirrored-explicit-not-live
//!    versus stale-flagged versus unknown) — never showing a cached, mirrored, or
//!    stale cited revision as live — and a self-contained
//!    [`M5DocsReferenceCardLinkageDisclosure`] that always names why the card
//!    appeared and how strong the linkage is rather than blending every case into one
//!    "docs found" card.
//! 2. A parity matrix — [`M5DocsReferenceCardPrimitivePacket`] — that binds one row
//!    per claimed M5 reference-card consumer (the editor hover/peek, the docs-browser
//!    card, the AI-explanation card, the onboarding reference card, and the support
//!    evidence card) to the shared card anatomy, the same linkage strengths, freshness
//!    postures, symbol anchors, match states, override reasons, export fields, and
//!    non-visual accessibility routes, so the anchor/linkage/source/version/freshness
//!    vocabulary stays identical across editor hover, peek, the docs browser, AI
//!    answers, onboarding, and support evidence.
//!
//! The symbol anchor ([`M5DocsSymbolAnchor`]), corpus class ([`M5DocsCorpusClass`]),
//! version scope ([`M5DocsVersionScope`]), source provider ([`M5DocsSourceProvider`]),
//! freshness state ([`M5DocsFreshnessState`]), match state ([`M5DocsMatchState`]),
//! override reason ([`M5DocsOverrideReason`]), docs surface family
//! ([`M5DocsSurfaceFamily`]), deployment line ([`M5DocsDeploymentLine`]), consumer
//! surface ([`M5DocsConsumerSurface`]), accessibility route
//! ([`M5DocsAccessibilityRoute`]), qualification class ([`M5DocsQualificationClass`]),
//! and downgrade trigger ([`M5DocsDowngradeTrigger`]) are reused verbatim from the
//! frozen docs-browser component matrix. This module mints new vocabulary only for
//! what that matrix left implicit about the reference card itself: its card consumers,
//! its anatomy parts, its derived symbol-linkage strengths, its derived freshness
//! postures, and its export fields. No M5 docs surface invents a second reference-card
//! grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, initiating source bodies, and
//! doc bodies stay outside the support boundary; every card title, initiating anchor,
//! cited revision, and open-action target is carried only as an opaque, export-safe
//! representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_reference_card_primitive_ai_explanation_preview_narrowed,
    seeded_m5_reference_card_primitive_onboarding_reference_beta_narrowed,
    seeded_m5_reference_card_primitive_packet, M5_DOCS_REFERENCE_CARD_PRIMITIVE_PACKET_ID,
};

// The symbol anchor, corpus class, version scope, source provider, freshness state,
// match state, override reason, docs surface family, deployment line, consumer
// surface, accessibility routes, qualification classes, and downgrade triggers are
// frozen once, in the docs-browser component matrix. This primitive reuses them
// verbatim so it never invents a parallel reference-card vocabulary.
pub use crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix::{
    M5DocsAccessibilityRoute, M5DocsConsumerSurface, M5DocsCorpusClass, M5DocsDeploymentLine,
    M5DocsDowngradeTrigger, M5DocsFreshnessState, M5DocsMatchState, M5DocsOverrideReason,
    M5DocsQualificationClass, M5DocsSourceProvider, M5DocsSurfaceFamily, M5DocsSymbolAnchor,
    M5DocsVersionScope,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DocsReferenceCardPrimitivePacket`].
pub const M5_DOCS_REFERENCE_CARD_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_docs_symbol_linked_reference_cards_with_code_anchor_symbol_anchor_linkage_strength_source_version_and_freshness_truth";

/// Schema version for M5 symbol-linked-reference-card-primitive records.
pub const M5_DOCS_REFERENCE_CARD_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the symbol-linked-reference-card boundary schema.
pub const M5_DOCS_REFERENCE_CARD_SCHEMA_REF: &str =
    "schemas/docs/m5-symbol-linked-reference-card-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DOCS_REFERENCE_CARD_DOC_REF: &str =
    "docs/docs/m5/implement_docs_symbol_linked_reference_cards_with_code_anchor_and_exact_nearby_project_or_keyword_fallback_truth.md";

/// Repo-relative path of the frozen docs-browser component matrix this primitive
/// narrows from.
pub const M5_DOCS_REFERENCE_CARD_COMPONENT_MATRIX_REF: &str =
    "schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json";

/// Repo-relative path of the stable docs-source/result contract this primitive binds
/// against.
pub const M5_DOCS_REFERENCE_CARD_SOURCE_RESULT_REF: &str =
    "schemas/docs/stable_docs_source_result_pack_and_citation.schema.json";

/// Repo-relative path of the docs-source precedence / ranking-parity contract this
/// primitive keeps source/linkage truth consistent with.
pub const M5_DOCS_REFERENCE_CARD_SOURCE_PRECEDENCE_REF: &str =
    "schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_REFERENCE_CARD_FIXTURE_DIR: &str =
    "fixtures/docs/m5/m5-symbol-linked-reference-card-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_REFERENCE_CARD_ARTIFACT_REF: &str =
    "artifacts/docs/m5/m5-symbol-linked-reference-card-primitive/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DOCS_REFERENCE_CARD_CSV_REF: &str =
    "artifacts/docs/m5/m5-symbol-linked-reference-card-primitive/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DOCS_REFERENCE_CARD_REPORT_REF: &str =
    "artifacts/docs/m5/m5-symbol-linked-reference-card-primitive.md";

/// One claimed M5 reference-card consumer that renders the shared symbol-linked
/// reference card. These are the entrypoints the acceptance criteria name — the
/// editor hover/peek, the docs browser, AI explanations, onboarding, and support
/// evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsReferenceCardConsumerSurface {
    /// The editor hover / peek reference card.
    EditorHoverPeek,
    /// The docs-browser reference card.
    DocsBrowserCard,
    /// The AI-explanation reference card.
    AiExplanationCard,
    /// The onboarding step reference card.
    OnboardingReferenceCard,
    /// The support / evidence reference card.
    SupportEvidenceCard,
}

impl M5DocsReferenceCardConsumerSurface {
    /// Every claimed reference-card consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EditorHoverPeek,
        Self::DocsBrowserCard,
        Self::AiExplanationCard,
        Self::OnboardingReferenceCard,
        Self::SupportEvidenceCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorHoverPeek => "editor_hover_peek",
            Self::DocsBrowserCard => "docs_browser_card",
            Self::AiExplanationCard => "ai_explanation_card",
            Self::OnboardingReferenceCard => "onboarding_reference_card",
            Self::SupportEvidenceCard => "support_evidence_card",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EditorHoverPeek => "Editor Hover / Peek",
            Self::DocsBrowserCard => "Docs-Browser Card",
            Self::AiExplanationCard => "AI-Explanation Card",
            Self::OnboardingReferenceCard => "Onboarding Reference Card",
            Self::SupportEvidenceCard => "Support Evidence Card",
        }
    }
}

/// One anatomy part the shared reference card surfaces. The parts in
/// [`M5DocsReferenceCardAnatomyPart::MANDATORY`] are required on every card so a user
/// can see where the card was opened from and how strong the linkage is before
/// trusting it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsReferenceCardAnatomyPart {
    /// The card title label.
    CardTitleLabel,
    /// The initiating file/symbol code anchor the card was opened from.
    InitiatingCodeAnchor,
    /// The symbol-anchor-kind badge.
    SymbolAnchorBadge,
    /// The symbol-linkage-strength cue.
    LinkageStrengthCue,
    /// The source-provider / source badge.
    SourceProviderBadge,
    /// The version / package scope badge.
    VersionScopeBadge,
    /// The cited source revision.
    CitedSourceRevision,
    /// The open action.
    OpenAction,
}

impl M5DocsReferenceCardAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CardTitleLabel,
        Self::InitiatingCodeAnchor,
        Self::SymbolAnchorBadge,
        Self::LinkageStrengthCue,
        Self::SourceProviderBadge,
        Self::VersionScopeBadge,
        Self::CitedSourceRevision,
        Self::OpenAction,
    ];

    /// The anatomy parts every reference card must render.
    pub const MANDATORY: [Self; 6] = [
        Self::CardTitleLabel,
        Self::InitiatingCodeAnchor,
        Self::SymbolAnchorBadge,
        Self::LinkageStrengthCue,
        Self::SourceProviderBadge,
        Self::OpenAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CardTitleLabel => "card_title_label",
            Self::InitiatingCodeAnchor => "initiating_code_anchor",
            Self::SymbolAnchorBadge => "symbol_anchor_badge",
            Self::LinkageStrengthCue => "linkage_strength_cue",
            Self::SourceProviderBadge => "source_provider_badge",
            Self::VersionScopeBadge => "version_scope_badge",
            Self::CitedSourceRevision => "cited_source_revision",
            Self::OpenAction => "open_action",
        }
    }
}

/// The derived symbol-linkage strength — the resolver's honest verdict about why a
/// reference card appeared and how strongly it links to the symbol it claims: an exact
/// symbol match, a nearby version match, a project-specific override, a keyword
/// fallback, a heuristic (mirror/cache-served) linkage, or an unresolved anchor with
/// no symbol linkage. An unresolved anchor is never shown as an exact symbol match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSymbolLinkageStrength {
    /// An exact symbol match at the exact version.
    ExactSymbolLinkage,
    /// The symbol resolved but only against a nearby version.
    NearbyVersionLinkage,
    /// A project-specific doc took precedence for this symbol.
    ProjectSpecificLinkage,
    /// No symbol resolved; the card was matched by keyword text.
    KeywordFallbackLinkage,
    /// The symbol resolved only heuristically (served from mirror/cache).
    HeuristicLinkage,
    /// The anchor is unresolved and no symbol linkage exists (missing-scope /
    /// policy-limited stub).
    UnresolvedNoLinkage,
}

impl M5DocsSymbolLinkageStrength {
    /// Every linkage strength, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactSymbolLinkage,
        Self::NearbyVersionLinkage,
        Self::ProjectSpecificLinkage,
        Self::KeywordFallbackLinkage,
        Self::HeuristicLinkage,
        Self::UnresolvedNoLinkage,
    ];

    /// The four linkage states the acceptance criteria require to stay explicit rather
    /// than blend into one "docs found" card.
    pub const NAMED_STATES: [Self; 4] = [
        Self::ExactSymbolLinkage,
        Self::NearbyVersionLinkage,
        Self::ProjectSpecificLinkage,
        Self::KeywordFallbackLinkage,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSymbolLinkage => "exact_symbol_linkage",
            Self::NearbyVersionLinkage => "nearby_version_linkage",
            Self::ProjectSpecificLinkage => "project_specific_linkage",
            Self::KeywordFallbackLinkage => "keyword_fallback_linkage",
            Self::HeuristicLinkage => "heuristic_linkage",
            Self::UnresolvedNoLinkage => "unresolved_no_linkage",
        }
    }

    /// A short, color-independent glyph label so the linkage cue never relies on color
    /// alone.
    pub const fn glyph_label(self) -> &'static str {
        match self {
            Self::ExactSymbolLinkage => "[exact-symbol]",
            Self::NearbyVersionLinkage => "[nearby-version]",
            Self::ProjectSpecificLinkage => "[project-doc]",
            Self::KeywordFallbackLinkage => "[keyword-only]",
            Self::HeuristicLinkage => "[heuristic]",
            Self::UnresolvedNoLinkage => "[no-linkage]",
        }
    }

    /// Review-safe phrase for the linkage disclosure headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ExactSymbolLinkage => "the symbol resolved to an exact match at this version",
            Self::NearbyVersionLinkage => "the symbol resolved only against a nearby version",
            Self::ProjectSpecificLinkage => {
                "a project-specific doc took precedence for this symbol"
            }
            Self::KeywordFallbackLinkage => {
                "no symbol resolved and the card was matched by keyword"
            }
            Self::HeuristicLinkage => {
                "the symbol resolved only heuristically from a mirror or cache"
            }
            Self::UnresolvedNoLinkage => "the anchor is unresolved and no symbol linkage exists",
        }
    }

    /// True when the card reflects an exact symbol match — the strongest linkage.
    pub const fn is_exact_symbol_linkage(self) -> bool {
        matches!(self, Self::ExactSymbolLinkage)
    }

    /// True when no symbol actually resolved (keyword fallback or unresolved anchor) —
    /// the linkage a user must never mistake for an exact match.
    pub const fn is_keyword_or_unresolved(self) -> bool {
        matches!(
            self,
            Self::KeywordFallbackLinkage | Self::UnresolvedNoLinkage
        )
    }

    /// True when a project-specific doc decided the card.
    pub const fn is_project_specific(self) -> bool {
        matches!(self, Self::ProjectSpecificLinkage)
    }
}

/// The derived freshness posture of a cited source revision — the resolver's verdict
/// about whether the revision reads as current-live, recently-synced-current, cached
/// or mirrored (explicit, never live), stale, or unknown. A cached, mirrored, or stale
/// cited revision is never shown as live even when its declared freshness would suggest
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsCardFreshnessPosture {
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

impl M5DocsCardFreshnessPosture {
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

    /// True when the cited revision reads as live/current (live or
    /// recently-synced-current).
    pub const fn is_live_current(self) -> bool {
        matches!(self, Self::CurrentLive | Self::RecentlySyncedCurrent)
    }

    /// True when the cited revision is a cached or mirrored copy shown explicitly,
    /// never as live.
    pub const fn is_explicit_not_live(self) -> bool {
        matches!(
            self,
            Self::CachedExplicitNotLive | Self::MirroredExplicitNotLive
        )
    }

    /// True when the cited revision is stale or of unknown freshness.
    pub const fn is_stale_or_unknown(self) -> bool {
        matches!(self, Self::StaleFlagged | Self::FreshnessUnknown)
    }
}

/// A field the support / export packet carries so reference-card identity is
/// reconstructable from the shared model. The fields in
/// [`M5DocsReferenceCardExportField::MANDATORY`] are required so the same anchor and
/// source descriptors survive export/support/AI evidence paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsReferenceCardExportField {
    /// The derived symbol-linkage strength.
    LinkageStrength,
    /// The symbol anchor kind.
    SymbolAnchor,
    /// The initiating file/symbol code anchor.
    InitiatingAnchor,
    /// The source provider.
    SourceProvider,
    /// The corpus class.
    CorpusClass,
    /// The version / package scope.
    VersionScope,
    /// The match state.
    MatchState,
    /// The override reason.
    OverrideReason,
    /// The declared freshness state.
    FreshnessState,
    /// The cited source revision.
    CitedSourceRevision,
}

impl M5DocsReferenceCardExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::LinkageStrength,
        Self::SymbolAnchor,
        Self::InitiatingAnchor,
        Self::SourceProvider,
        Self::CorpusClass,
        Self::VersionScope,
        Self::MatchState,
        Self::OverrideReason,
        Self::FreshnessState,
        Self::CitedSourceRevision,
    ];

    /// The export fields every reference-card export must carry so identity survives.
    pub const MANDATORY: [Self; 6] = [
        Self::LinkageStrength,
        Self::SymbolAnchor,
        Self::InitiatingAnchor,
        Self::SourceProvider,
        Self::VersionScope,
        Self::FreshnessState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinkageStrength => "linkage_strength",
            Self::SymbolAnchor => "symbol_anchor",
            Self::InitiatingAnchor => "initiating_anchor",
            Self::SourceProvider => "source_provider",
            Self::CorpusClass => "corpus_class",
            Self::VersionScope => "version_scope",
            Self::MatchState => "match_state",
            Self::OverrideReason => "override_reason",
            Self::FreshnessState => "freshness_state",
            Self::CitedSourceRevision => "cited_source_revision",
        }
    }
}

/// A self-contained linkage disclosure: the symbol-linkage strength, the symbol
/// anchor, the version scope, the override reason, and the source provider, so a user
/// can always tell why a reference card appeared and how strong the linkage is from the
/// disclosure alone rather than every case blending into one "docs found" card. Unlike
/// a rank-reason disclosure, this is always present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardLinkageDisclosure {
    /// The derived symbol-linkage strength.
    pub linkage_strength: M5DocsSymbolLinkageStrength,
    /// The symbol anchor the card is linked to.
    pub symbol_anchor: M5DocsSymbolAnchor,
    /// The version / package scope the card is bound to.
    pub version_scope: M5DocsVersionScope,
    /// The override reason behind the card.
    pub override_reason: M5DocsOverrideReason,
    /// The source provider behind the cited doc.
    pub source_provider: M5DocsSourceProvider,
    /// A deterministic, self-contained headline naming the linkage strength, the
    /// symbol anchor, the version scope, and the source provider.
    pub headline: String,
}

/// The full input to the reference-card resolver for one card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardResolutionInput {
    /// The opaque, export-safe card title.
    pub card_title_repr: String,
    /// The opaque, export-safe initiating file the card was opened from. Must be
    /// non-empty so the code anchor is preserved.
    pub initiating_file_repr: String,
    /// The opaque, export-safe initiating symbol the card was opened from. Must be
    /// non-empty so the code anchor is preserved.
    pub initiating_symbol_repr: String,
    /// The symbol anchor kind the card resolves to.
    pub symbol_anchor: M5DocsSymbolAnchor,
    /// The corpus class the cited doc belongs to.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider behind the cited doc.
    pub source_provider: M5DocsSourceProvider,
    /// The match state relating the card to the query and local corpus.
    pub match_state: M5DocsMatchState,
    /// The project-doc override reason (or no-override) behind the card.
    pub override_reason: M5DocsOverrideReason,
    /// The version / package scope in effect.
    pub version_scope: M5DocsVersionScope,
    /// The declared freshness state of the cited revision.
    pub freshness_state: M5DocsFreshnessState,
    /// The opaque, export-safe cited source revision. May be empty when no revision is
    /// cited.
    pub cited_source_revision_repr: String,
    /// The opaque, export-safe open-action target. Must be non-empty so the card is
    /// actionable.
    pub open_action_target_repr: String,
}

/// The resolved anchor / linkage / source / version / freshness truth for one
/// reference card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDocsReferenceCard {
    /// The opaque card title.
    pub card_title_repr: String,
    /// The opaque initiating file.
    pub initiating_file_repr: String,
    /// The opaque initiating symbol.
    pub initiating_symbol_repr: String,
    /// The symbol anchor kind.
    pub symbol_anchor: M5DocsSymbolAnchor,
    /// The corpus class.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider.
    pub source_provider: M5DocsSourceProvider,
    /// The match state.
    pub match_state: M5DocsMatchState,
    /// The override reason.
    pub override_reason: M5DocsOverrideReason,
    /// The version / package scope.
    pub version_scope: M5DocsVersionScope,
    /// The declared freshness state.
    pub freshness_state: M5DocsFreshnessState,
    /// The opaque cited source revision.
    pub cited_source_revision_repr: String,
    /// The opaque open-action target.
    pub open_action_target_repr: String,
    /// The derived symbol-linkage strength.
    pub linkage_strength: M5DocsSymbolLinkageStrength,
    /// True when the card reflects an exact symbol match.
    pub is_exact_symbol_linkage: bool,
    /// True when the symbol anchor actually resolved (never for an unresolved anchor).
    pub is_symbol_resolved: bool,
    /// The derived freshness posture.
    pub freshness_posture: M5DocsCardFreshnessPosture,
    /// True when the cited revision reads as live/current (never true for cached,
    /// mirrored, or stale revisions).
    pub shows_as_live: bool,
    /// The always-present linkage disclosure naming why the card appeared.
    pub linkage_disclosure: M5DocsReferenceCardLinkageDisclosure,
}

/// Errors returned by [`resolve_reference_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DocsReferenceCardResolutionError {
    /// The card title was empty.
    EmptyCardTitle,
    /// The initiating file or symbol was empty (the code anchor must be preserved).
    EmptyInitiatingAnchor,
    /// The open-action target was empty (the card must be actionable).
    EmptyOpenActionTarget,
    /// A card representation carried forbidden material.
    ForbiddenCardMaterial,
}

impl M5DocsReferenceCardResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyCardTitle => "empty_card_title",
            Self::EmptyInitiatingAnchor => "empty_initiating_anchor",
            Self::EmptyOpenActionTarget => "empty_open_action_target",
            Self::ForbiddenCardMaterial => "forbidden_card_material",
        }
    }
}

impl fmt::Display for M5DocsReferenceCardResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "reference-card resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DocsReferenceCardResolutionError {}

/// Resolves one symbol-linked reference card from its declared state.
///
/// The derived symbol-linkage strength is computed in a fixed, honesty-first order: an
/// unresolved anchor never reads as an exact symbol match — it reads as a keyword
/// fallback, or, when even the match is stale, as an unresolved no-linkage stub. Then
/// a project-pinned/project-specific card reads as project-specific linkage, an exact
/// match at a non-nearby version reads as exact symbol linkage, a nearby version or
/// nearby match reads as nearby-version linkage, a mirror/cache/stale match reads as
/// heuristic linkage, and anything else reads as a keyword fallback. The freshness
/// posture keeps a cached, mirrored, or stale cited revision explicit rather than shown
/// as live, and every card carries a self-contained linkage disclosure so the reason it
/// appeared is never blended away.
pub fn resolve_reference_card(
    input: &M5DocsReferenceCardResolutionInput,
) -> Result<M5ResolvedDocsReferenceCard, M5DocsReferenceCardResolutionError> {
    if input.card_title_repr.trim().is_empty() {
        return Err(M5DocsReferenceCardResolutionError::EmptyCardTitle);
    }
    if input.initiating_file_repr.trim().is_empty()
        || input.initiating_symbol_repr.trim().is_empty()
    {
        return Err(M5DocsReferenceCardResolutionError::EmptyInitiatingAnchor);
    }
    if input.open_action_target_repr.trim().is_empty() {
        return Err(M5DocsReferenceCardResolutionError::EmptyOpenActionTarget);
    }
    if value_repr_is_forbidden(&input.card_title_repr)
        || value_repr_is_forbidden(&input.initiating_file_repr)
        || value_repr_is_forbidden(&input.initiating_symbol_repr)
        || value_repr_is_forbidden(&input.cited_source_revision_repr)
        || value_repr_is_forbidden(&input.open_action_target_repr)
    {
        return Err(M5DocsReferenceCardResolutionError::ForbiddenCardMaterial);
    }

    let linkage_strength = derive_linkage_strength(
        input.symbol_anchor,
        input.match_state,
        input.version_scope,
        input.override_reason,
    );
    let is_exact_symbol_linkage = linkage_strength.is_exact_symbol_linkage();
    let is_symbol_resolved = !matches!(input.symbol_anchor, M5DocsSymbolAnchor::UnresolvedAnchor);

    let freshness_posture = derive_freshness_posture(input.freshness_state, input.match_state);
    let shows_as_live = freshness_posture.is_live_current();

    let headline = format!(
        "This card appeared because {} — {} {} linkage on a {} anchor in {} scope (source: {}, override: {})",
        linkage_strength.phrase(),
        linkage_strength.glyph_label(),
        linkage_strength.as_str(),
        input.symbol_anchor.as_str(),
        input.version_scope.as_str(),
        input.source_provider.as_str(),
        input.override_reason.as_str()
    );
    let linkage_disclosure = M5DocsReferenceCardLinkageDisclosure {
        linkage_strength,
        symbol_anchor: input.symbol_anchor,
        version_scope: input.version_scope,
        override_reason: input.override_reason,
        source_provider: input.source_provider,
        headline,
    };

    Ok(M5ResolvedDocsReferenceCard {
        card_title_repr: input.card_title_repr.clone(),
        initiating_file_repr: input.initiating_file_repr.clone(),
        initiating_symbol_repr: input.initiating_symbol_repr.clone(),
        symbol_anchor: input.symbol_anchor,
        corpus_class: input.corpus_class,
        source_provider: input.source_provider,
        match_state: input.match_state,
        override_reason: input.override_reason,
        version_scope: input.version_scope,
        freshness_state: input.freshness_state,
        cited_source_revision_repr: input.cited_source_revision_repr.clone(),
        open_action_target_repr: input.open_action_target_repr.clone(),
        linkage_strength,
        is_exact_symbol_linkage,
        is_symbol_resolved,
        freshness_posture,
        shows_as_live,
        linkage_disclosure,
    })
}

/// The fixed, honesty-first symbol-linkage-strength ladder.
fn derive_linkage_strength(
    anchor: M5DocsSymbolAnchor,
    match_state: M5DocsMatchState,
    version_scope: M5DocsVersionScope,
    override_reason: M5DocsOverrideReason,
) -> M5DocsSymbolLinkageStrength {
    use M5DocsMatchState as Match;
    use M5DocsOverrideReason as Override;
    use M5DocsSymbolAnchor as Anchor;
    use M5DocsVersionScope as Scope;

    if matches!(anchor, Anchor::UnresolvedAnchor) {
        // An unresolved anchor never reads as an exact symbol match. It is a keyword
        // fallback, or — when even the match is stale — a bare no-linkage stub.
        if matches!(match_state, Match::StaleMatch) {
            M5DocsSymbolLinkageStrength::UnresolvedNoLinkage
        } else {
            M5DocsSymbolLinkageStrength::KeywordFallbackLinkage
        }
    } else if matches!(override_reason, Override::ProjectPinnedOverride)
        || matches!(match_state, Match::ProjectSpecificMatch)
        || matches!(version_scope, Scope::ProjectSpecific)
    {
        M5DocsSymbolLinkageStrength::ProjectSpecificLinkage
    } else if matches!(match_state, Match::ExactMatch)
        && !matches!(version_scope, Scope::NearbyVersion)
    {
        M5DocsSymbolLinkageStrength::ExactSymbolLinkage
    } else if matches!(version_scope, Scope::NearbyVersion)
        || matches!(match_state, Match::NearbyMatch)
    {
        M5DocsSymbolLinkageStrength::NearbyVersionLinkage
    } else if matches!(
        match_state,
        Match::MirroredMatch | Match::CachedMatch | Match::StaleMatch
    ) {
        M5DocsSymbolLinkageStrength::HeuristicLinkage
    } else {
        M5DocsSymbolLinkageStrength::KeywordFallbackLinkage
    }
}

/// The freshness-posture ladder: a cached, mirrored, or stale match is never shown as
/// live even when the declared freshness would suggest it.
fn derive_freshness_posture(
    freshness: M5DocsFreshnessState,
    match_state: M5DocsMatchState,
) -> M5DocsCardFreshnessPosture {
    use M5DocsFreshnessState as Fresh;
    use M5DocsMatchState as Match;

    match freshness {
        Fresh::LiveCurrent => match match_state {
            Match::CachedMatch => M5DocsCardFreshnessPosture::CachedExplicitNotLive,
            Match::MirroredMatch => M5DocsCardFreshnessPosture::MirroredExplicitNotLive,
            Match::StaleMatch => M5DocsCardFreshnessPosture::StaleFlagged,
            _ => M5DocsCardFreshnessPosture::CurrentLive,
        },
        Fresh::RecentlySynced => M5DocsCardFreshnessPosture::RecentlySyncedCurrent,
        Fresh::CachedOffline => match match_state {
            Match::MirroredMatch => M5DocsCardFreshnessPosture::MirroredExplicitNotLive,
            _ => M5DocsCardFreshnessPosture::CachedExplicitNotLive,
        },
        Fresh::StaleExpired => M5DocsCardFreshnessPosture::StaleFlagged,
        Fresh::UnknownFreshness => M5DocsCardFreshnessPosture::FreshnessUnknown,
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs reference-card truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardResolutionCase {
    /// The resolver input.
    pub input: M5DocsReferenceCardResolutionInput,
    /// The resolved truth. Must equal `resolve_reference_card(&input)`.
    pub resolved: M5ResolvedDocsReferenceCard,
}

impl M5DocsReferenceCardResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DocsReferenceCardResolutionInput) -> Self {
        let resolved = resolve_reference_card(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_reference_card(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one reference-card consumer bound to the shared
/// card anatomy, linkage strengths, freshness postures, symbol anchors, match states,
/// override reasons, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardRow {
    /// Reference-card consumer family.
    pub consumer_surface: M5DocsReferenceCardConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5DocsQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 docs surface families that render / consume this card.
    pub surface_families: Vec<M5DocsSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5DocsDeploymentLine>,
    /// Anatomy parts this card renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5DocsReferenceCardAnatomyPart>,
    /// Symbol anchors this card distinguishes.
    pub symbol_anchors: Vec<M5DocsSymbolAnchor>,
    /// Linkage strengths this card distinguishes.
    pub linkage_strengths: Vec<M5DocsSymbolLinkageStrength>,
    /// Corpus classes this card names.
    pub corpus_classes: Vec<M5DocsCorpusClass>,
    /// Source providers this card names.
    pub source_providers: Vec<M5DocsSourceProvider>,
    /// Match states this card distinguishes.
    pub match_states: Vec<M5DocsMatchState>,
    /// Override reasons this card names.
    pub override_reasons: Vec<M5DocsOverrideReason>,
    /// Version scopes this card names.
    pub version_scopes: Vec<M5DocsVersionScope>,
    /// Freshness states this card discloses.
    pub freshness_states: Vec<M5DocsFreshnessState>,
    /// Freshness postures this card distinguishes.
    pub freshness_postures: Vec<M5DocsCardFreshnessPosture>,
    /// Export fields this card carries (must include the mandatory fields).
    pub export_fields: Vec<M5DocsReferenceCardExportField>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5DocsAccessibilityRoute>,
    /// Docs subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5DocsConsumerSurface>,
    /// Downgrade triggers that apply to this card.
    pub downgrade_triggers: Vec<M5DocsDowngradeTrigger>,
    /// Proof packet refs that keep this card current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5DocsReferenceCardResolutionCase>,
    /// Hard invariant: this card never masks the source provider or the version scope.
    /// MUST be `false`.
    pub masks_source_or_version: bool,
    /// Hard invariant: this card never shows a cached, mirrored, or stale cited
    /// revision as live. MUST be `false`.
    pub shows_cached_or_stale_as_live: bool,
    /// Hard invariant: this card never invents a private card grammar. MUST be
    /// `false`.
    pub invents_private_card_grammar: bool,
    /// Hard invariant: this card never hides the symbol linkage strength or the
    /// initiating anchor. MUST be `false`.
    pub hides_symbol_linkage: bool,
}

impl M5DocsReferenceCardRow {
    /// True when the card declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DocsReferenceCardAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DocsReferenceCardAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the card declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DocsReferenceCardExportField> =
            self.export_fields.iter().copied().collect();
        M5DocsReferenceCardExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the card's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_source_or_version
            && !self.shows_cached_or_stale_as_live
            && !self.invents_private_card_grammar
            && !self.hides_symbol_linkage
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardVocabularySet {
    /// Reference-card consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Linkage-strength tokens.
    pub linkage_strengths: Vec<String>,
    /// Freshness-posture tokens.
    pub freshness_postures: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Symbol-anchor tokens (reused from the frozen matrix).
    pub symbol_anchors: Vec<String>,
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

impl M5DocsReferenceCardVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5DocsReferenceCardConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DocsReferenceCardAnatomyPart::ALL, |v| v.as_str()),
            linkage_strengths: tokens(&M5DocsSymbolLinkageStrength::ALL, |v| v.as_str()),
            freshness_postures: tokens(&M5DocsCardFreshnessPosture::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DocsReferenceCardExportField::ALL, |v| v.as_str()),
            symbol_anchors: tokens(&M5DocsSymbolAnchor::ALL, |v| v.as_str()),
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
pub struct M5DocsReferenceCardGovernanceReview {
    /// One reference-card primitive carries anchor, linkage, source, and freshness
    /// truth on every consumer.
    pub one_primitive_carries_card_truth: bool,
    /// The initiating file/symbol code anchor is preserved on every card.
    pub initiating_anchor_always_preserved: bool,
    /// The symbol-linkage strength is always explicit.
    pub linkage_strength_always_explicit: bool,
    /// Exact, nearby, project-specific, and keyword-fallback states never blend into
    /// one "docs found" card.
    pub exact_nearby_project_keyword_never_blended: bool,
    /// The cited source revision stays visible.
    pub cited_source_revision_visible: bool,
    /// A cached, mirrored, or stale cited revision is never shown as live.
    pub cached_or_stale_never_shown_as_live: bool,
    /// Reference-card identity survives export/support/AI evidence paths with the same
    /// anchor and source descriptors.
    pub reference_card_identity_survives_export: bool,
    /// The badge / state vocabulary stays stable across UI, docs/help, exports, and
    /// support packets.
    pub badge_state_vocabulary_stable_across_surfaces: bool,
    /// No consumer invents a second reference-card grammar.
    pub no_surface_invents_second_card_grammar: bool,
    /// Every card declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel reference-card vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardConsumerProjection {
    /// Editor hover/peek, docs-browser, AI-explanation, onboarding, and support
    /// consumers all consume the shared primitive.
    pub card_surfaces_consume_shared_primitive: bool,
    /// The linkage strength reads a single canonical source.
    pub linkage_strength_reads_single_source: bool,
    /// The initiating anchor reads a single canonical source.
    pub anchor_reads_single_source: bool,
    /// The freshness posture reads a single canonical source.
    pub freshness_posture_reads_single_source: bool,
    /// Support / export reads a single canonical reference-card source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the reference-card primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardReleasePosture {
    /// Ref of the supporting proof packet.
    pub proof_packet_ref: String,
    /// Ref of the supporting reference-card audit.
    pub reference_card_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DocsReferenceCardPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsReferenceCardPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Reference-card rows.
    pub reference_card_rows: Vec<M5DocsReferenceCardRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsReferenceCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsReferenceCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsReferenceCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsReferenceCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsReferenceCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 symbol-linked-reference-card-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsReferenceCardPrimitivePacket {
    /// Record kind; must equal [`M5_DOCS_REFERENCE_CARD_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_REFERENCE_CARD_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Reference-card rows.
    pub reference_card_rows: Vec<M5DocsReferenceCardRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsReferenceCardVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsReferenceCardGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsReferenceCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsReferenceCardProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsReferenceCardReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsReferenceCardPrimitivePacket {
    /// Builds an M5 symbol-linked-reference-card-primitive packet from stable-lane
    /// input.
    pub fn new(input: M5DocsReferenceCardPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_REFERENCE_CARD_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_REFERENCE_CARD_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            reference_card_rows: input.reference_card_rows,
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

    /// Validates the M5 symbol-linked-reference-card-primitive invariants.
    pub fn validate(&self) -> Vec<M5DocsReferenceCardPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_REFERENCE_CARD_PRIMITIVE_RECORD_KIND {
            violations.push(M5DocsReferenceCardPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_REFERENCE_CARD_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5DocsReferenceCardPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsReferenceCardPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_reference_card_rows(self, &mut violations);
        validate_linkage_state_coverage(self, &mut violations);
        validate_anchor_identity(self, &mut violations);
        validate_freshness_visibility(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 reference-card primitive packet serializes"),
        ) {
            violations.push(M5DocsReferenceCardPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 reference-card primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per reference-card consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,symbol_anchors,linkage_strengths,freshness_postures,match_states,override_reasons,export_fields,example_count\n",
        );
        for row in &self.reference_card_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.symbol_anchors, |v| v.as_str()),
                join_tokens(&row.linkage_strengths, |v| v.as_str()),
                join_tokens(&row.freshness_postures, |v| v.as_str()),
                join_tokens(&row.match_states, |v| v.as_str()),
                join_tokens(&row.override_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .reference_card_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Symbol-Linked Reference-Card Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Reference-card consumers: {} ({} stable)\n",
            self.reference_card_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Linkage strengths: {}\n",
            self.vocabulary_set.linkage_strengths.join(", ")
        ));
        out.push_str(&format!(
            "- Freshness postures: {}\n",
            self.vocabulary_set.freshness_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Symbol anchors: {}\n",
            self.vocabulary_set.symbol_anchors.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Reference-card consumers\n\n");
        for row in &self.reference_card_rows {
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
                out.push_str(&format!(
                    "    - `{}` from `{}::{}` → linkage `{}` (anchor `{}`, posture `{}`, resolved `{}`)\n",
                    case.resolved.card_title_repr,
                    case.resolved.initiating_file_repr,
                    case.resolved.initiating_symbol_repr,
                    case.resolved.linkage_strength.as_str(),
                    case.resolved.symbol_anchor.as_str(),
                    case.resolved.freshness_posture.as_str(),
                    case.resolved.is_symbol_resolved,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 reference-card-primitive export.
#[derive(Debug)]
pub enum M5DocsReferenceCardPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsReferenceCardPrimitiveViolation>),
}

impl fmt::Display for M5DocsReferenceCardPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 reference-card primitive export parse failed: {error}"
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
                    "m5 reference-card primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsReferenceCardPrimitiveArtifactError {}

/// Validation failures emitted by [`M5DocsReferenceCardPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsReferenceCardPrimitiveViolation {
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
    /// A required reference-card consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A reference-card row is incomplete.
    ReferenceCardRowIncomplete,
    /// A reference-card row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A reference-card row declares no symbol anchors.
    SymbolAnchorMissing,
    /// A reference-card row declares no linkage strengths.
    LinkageStrengthMissing,
    /// A reference-card row declares no freshness postures.
    FreshnessPostureMissing,
    /// A reference-card row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A reference-card row declares no accessibility routes (or misses keyboard
    /// focus).
    AccessibilityRouteMissing,
    /// A reference-card row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A reference-card row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A reference-card row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A reference-card row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves each of the exact, nearby, project-specific, and
    /// keyword-fallback linkage states.
    LinkageStateCoverageUnproven,
    /// No worked resolution proves both a resolved and an unresolved anchor, or a card
    /// dropped its initiating anchor.
    AnchorIdentityUnproven,
    /// No worked resolution proves both a live and a not-live freshness posture.
    FreshnessVisibilityUnproven,
    /// A reference-card row violates a hard invariant.
    CardInvariantViolated,
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

impl M5DocsReferenceCardPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ReferenceCardRowIncomplete => "reference_card_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::SymbolAnchorMissing => "symbol_anchor_missing",
            Self::LinkageStrengthMissing => "linkage_strength_missing",
            Self::FreshnessPostureMissing => "freshness_posture_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::LinkageStateCoverageUnproven => "linkage_state_coverage_unproven",
            Self::AnchorIdentityUnproven => "anchor_identity_unproven",
            Self::FreshnessVisibilityUnproven => "freshness_visibility_unproven",
            Self::CardInvariantViolated => "card_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 reference-card-primitive export.
pub fn current_stable_m5_reference_card_primitive_export(
) -> Result<M5DocsReferenceCardPrimitivePacket, M5DocsReferenceCardPrimitiveArtifactError> {
    let packet: M5DocsReferenceCardPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-symbol-linked-reference-card-primitive/support_export.json"
    )))
    .map_err(M5DocsReferenceCardPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsReferenceCardPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_REFERENCE_CARD_SCHEMA_REF,
        M5_DOCS_REFERENCE_CARD_DOC_REF,
        M5_DOCS_REFERENCE_CARD_COMPONENT_MATRIX_REF,
        M5_DOCS_REFERENCE_CARD_SOURCE_RESULT_REF,
        M5_DOCS_REFERENCE_CARD_SOURCE_PRECEDENCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsReferenceCardPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DocsReferenceCardPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_reference_card_rows(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    let present: BTreeSet<M5DocsReferenceCardConsumerSurface> = packet
        .reference_card_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5DocsReferenceCardConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5DocsReferenceCardPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.reference_card_rows {
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
            || row.symbol_anchors.is_empty()
        {
            violations.push(M5DocsReferenceCardPrimitiveViolation::ReferenceCardRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.symbol_anchors.is_empty() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::SymbolAnchorMissing);
        }
        if row.linkage_strengths.is_empty() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::LinkageStrengthMissing);
        }
        if row.freshness_postures.is_empty() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::FreshnessPostureMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5DocsAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5DocsReferenceCardPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DocsReferenceCardPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DocsReferenceCardPrimitiveViolation::CardInvariantViolated);
        }
    }
}

/// Every one of the four named linkage states — exact symbol, nearby version,
/// project-specific, and keyword fallback — must be proven by some worked resolution so
/// they stay explicit rather than blending into one "docs found" card (the acceptance
/// criterion that exact/nearby/project-specific/keyword-fallback states remain
/// explicit).
fn validate_linkage_state_coverage(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    for required in M5DocsSymbolLinkageStrength::NAMED_STATES {
        let proven = packet.reference_card_rows.iter().any(|row| {
            row.example_resolutions
                .iter()
                .any(|case| case.resolved.linkage_strength == required)
        });
        if !proven {
            violations.push(M5DocsReferenceCardPrimitiveViolation::LinkageStateCoverageUnproven);
            return;
        }
    }
}

/// Every worked resolution must preserve the initiating file/symbol anchor, and the
/// matrix must prove both a resolved and an unresolved anchor so a user can always tell
/// how strong the linkage is (the acceptance criterion that reference-card identity
/// survives with the same anchor and that symbol linkage strength is explicit).
fn validate_anchor_identity(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    let all_anchors_preserved = packet.reference_card_rows.iter().all(|row| {
        row.example_resolutions.iter().all(|case| {
            !case.resolved.initiating_file_repr.trim().is_empty()
                && !case.resolved.initiating_symbol_repr.trim().is_empty()
        })
    });
    let has_resolved = packet.reference_card_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_symbol_resolved)
    });
    let has_unresolved = packet.reference_card_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| !case.resolved.is_symbol_resolved)
    });
    if !(all_anchors_preserved && has_resolved && has_unresolved) {
        violations.push(M5DocsReferenceCardPrimitiveViolation::AnchorIdentityUnproven);
    }
}

/// At least one worked resolution must prove a live/current cited revision and at least
/// one a not-live (cached, mirrored, stale, or unknown) revision — the acceptance
/// criterion example that freshness stays visible wherever a card is reused.
fn validate_freshness_visibility(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    let has_live = packet.reference_card_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.freshness_posture.is_live_current())
    });
    let has_not_live = packet.reference_card_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.freshness_posture.is_explicit_not_live()
                || case.resolved.freshness_posture.is_stale_or_unknown()
        })
    });
    if !(has_live && has_not_live) {
        violations.push(M5DocsReferenceCardPrimitiveViolation::FreshnessVisibilityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_card_truth,
        review.initiating_anchor_always_preserved,
        review.linkage_strength_always_explicit,
        review.exact_nearby_project_keyword_never_blended,
        review.cited_source_revision_visible,
        review.cached_or_stale_never_shown_as_live,
        review.reference_card_identity_survives_export,
        review.badge_state_vocabulary_stable_across_surfaces,
        review.no_surface_invents_second_card_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DocsReferenceCardPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.card_surfaces_consume_shared_primitive,
        projection.linkage_strength_reads_single_source,
        projection.anchor_reads_single_source,
        projection.freshness_posture_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DocsReferenceCardPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsReferenceCardPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DocsReferenceCardPrimitivePacket,
    violations: &mut Vec<M5DocsReferenceCardPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.reference_card_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DocsReferenceCardPrimitiveViolation::ReleasePostureIncomplete);
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
