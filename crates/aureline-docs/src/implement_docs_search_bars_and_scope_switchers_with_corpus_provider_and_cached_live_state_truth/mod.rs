//! One reusable M5 docs-search-bar / docs-scope-switcher primitive: corpus class,
//! source provider, provider availability, cached/live/mirrored retrieval mode,
//! version/package scope, and keyboard hint, projected the same way across every
//! claimed M5 docs, help, onboarding, and AI search entrypoint.
//!
//! Aureline's frozen docs-browser component matrix
//! ([`crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`])
//! names the docs search bar and the docs scope switcher as two governed component
//! families and freezes their controlled vocabulary — the corpus classes, the
//! version scopes, the source providers, the freshness states, the docs surface
//! families, the deployment lines, the consumer surfaces, the accessibility routes,
//! the qualification classes, and the downgrade triggers. This module *implements*
//! that search-bar / scope-switcher contract as one reusable primitive so a user
//! can tell — from the search bar and its scope switcher alone — what corpus and
//! provider Aureline is searching, whether results are live, cached, mirrored, or
//! narrowed, and how to reach the bar by keyboard, before they read any result,
//! instead of that truth drifting by search palette, hover peek, onboarding tour,
//! or AI-context panel.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_docs_search`] — that takes one search bar's label,
//!    scope target, searched corpus classes, source provider, provider
//!    availability, retrieval mode, version scope, and keyboard hint, and produces
//!    one [`M5ResolvedDocsSearch`] carrying the derived search-availability posture
//!    (live-ready versus cached-ready versus mirrored-ready versus narrowed versus
//!    degraded versus blocked) and — whenever the search is narrowed, degraded, or
//!    blocked — a self-contained [`M5DocsSearchDegradedBanner`] that names the exact
//!    limit reason, the corpus in scope, the retrieval mode, and the next action
//!    rather than returning empty results with no explanation. The resolver never
//!    shows cached or mirrored content as live, and never masks the corpus or the
//!    provider.
//! 2. A parity matrix — [`M5DocsSearchPrimitivePacket`] — that binds one row per
//!    claimed M5 docs-search consumer (the docs-browser search, the onboarding /
//!    tutorial lookup, the AI citation-follow flow, the support / help search, and
//!    the CLI docs search) to the shared search-bar anatomy, the same
//!    availability postures, provider availabilities, retrieval modes, limit
//!    reasons, next actions, export fields, and non-visual accessibility routes, so
//!    the corpus/provider/scope vocabulary stays identical across the docs browser,
//!    onboarding, AI, support/help, and the CLI.
//!
//! The corpus class ([`M5DocsCorpusClass`]), version scope ([`M5DocsVersionScope`]),
//! source provider ([`M5DocsSourceProvider`]), freshness state
//! ([`M5DocsFreshnessState`]), docs surface family ([`M5DocsSurfaceFamily`]),
//! deployment line ([`M5DocsDeploymentLine`]), consumer surface
//! ([`M5DocsConsumerSurface`]), accessibility route ([`M5DocsAccessibilityRoute`]),
//! qualification class ([`M5DocsQualificationClass`]), and downgrade trigger
//! ([`M5DocsDowngradeTrigger`]) are reused verbatim from the frozen docs-browser
//! component matrix. This module mints new vocabulary only for what that matrix left
//! implicit about the search bar and scope switcher themselves: their search
//! consumers, their anatomy parts, their provider availabilities, their retrieval
//! modes, their search-availability postures, their search-limit reasons, their
//! next actions, and their export fields. No M5 docs surface invents a second
//! search-bar grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user query bodies stay
//! outside the support boundary; every search-bar label, scope target, and keyboard
//! hint is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_docs_search_primitive_ai_citation_follow_preview_narrowed,
    seeded_m5_docs_search_primitive_onboarding_lookup_beta_narrowed,
    seeded_m5_docs_search_primitive_packet, M5_DOCS_SEARCH_PRIMITIVE_PACKET_ID,
};

// The corpus class, version scope, source provider, freshness state, docs surface
// family, deployment line, consumer surface, accessibility routes, qualification
// classes, and downgrade triggers are frozen once, in the docs-browser component
// matrix. This primitive reuses them verbatim so it never invents a parallel
// search-bar vocabulary.
pub use crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix::{
    M5DocsAccessibilityRoute, M5DocsConsumerSurface, M5DocsCorpusClass, M5DocsDeploymentLine,
    M5DocsDowngradeTrigger, M5DocsFreshnessState, M5DocsQualificationClass, M5DocsSourceProvider,
    M5DocsSurfaceFamily, M5DocsVersionScope,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DocsSearchPrimitivePacket`].
pub const M5_DOCS_SEARCH_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_docs_search_bars_and_scope_switchers_with_corpus_class_provider_availability_and_cached_live_state_truth";

/// Schema version for M5 docs-search-primitive records.
pub const M5_DOCS_SEARCH_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the docs-search-bar / scope-switcher boundary schema.
pub const M5_DOCS_SEARCH_SCHEMA_REF: &str =
    "schemas/docs/m5-docs-search-bar-and-scope-switcher-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DOCS_SEARCH_DOC_REF: &str =
    "docs/docs/m5/implement_docs_search_bars_and_scope_switchers_with_corpus_provider_and_cached_live_state_truth.md";

/// Repo-relative path of the frozen docs-browser component matrix this primitive
/// narrows from.
pub const M5_DOCS_SEARCH_COMPONENT_MATRIX_REF: &str =
    "schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json";

/// Repo-relative path of the stable docs-source/result contract this primitive
/// binds against.
pub const M5_DOCS_SEARCH_SOURCE_RESULT_REF: &str =
    "schemas/docs/stable_docs_source_result_pack_and_citation.schema.json";

/// Repo-relative path of the docs-source precedence / ranking-parity contract this
/// primitive keeps corpus/provider truth consistent with.
pub const M5_DOCS_SEARCH_SOURCE_PRECEDENCE_REF: &str =
    "schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_SEARCH_FIXTURE_DIR: &str =
    "fixtures/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_SEARCH_ARTIFACT_REF: &str =
    "artifacts/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DOCS_SEARCH_CSV_REF: &str =
    "artifacts/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DOCS_SEARCH_REPORT_REF: &str =
    "artifacts/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive.md";

/// One claimed M5 docs-search consumer that renders the shared search bar and its
/// scope switcher. These are the entrypoints the acceptance criteria name — the
/// docs browser, onboarding / tutorial lookup, AI citation-follow, support / help,
/// and the CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSearchConsumerSurface {
    /// The docs-browser search bar.
    DocsBrowserSearch,
    /// The onboarding / tutorial lookup search.
    OnboardingTutorialLookup,
    /// The AI citation-follow search flow.
    AiCitationFollow,
    /// The support / help search.
    SupportHelpSearch,
    /// The CLI / headless docs search.
    CliDocsSearch,
}

impl M5DocsSearchConsumerSurface {
    /// Every claimed docs-search consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DocsBrowserSearch,
        Self::OnboardingTutorialLookup,
        Self::AiCitationFollow,
        Self::SupportHelpSearch,
        Self::CliDocsSearch,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserSearch => "docs_browser_search",
            Self::OnboardingTutorialLookup => "onboarding_tutorial_lookup",
            Self::AiCitationFollow => "ai_citation_follow",
            Self::SupportHelpSearch => "support_help_search",
            Self::CliDocsSearch => "cli_docs_search",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DocsBrowserSearch => "Docs-Browser Search",
            Self::OnboardingTutorialLookup => "Onboarding / Tutorial Lookup",
            Self::AiCitationFollow => "AI Citation-Follow",
            Self::SupportHelpSearch => "Support / Help Search",
            Self::CliDocsSearch => "CLI Docs Search",
        }
    }
}

/// One anatomy part the shared search bar / scope switcher surfaces. The parts in
/// [`M5DocsSearchBarAnatomyPart::MANDATORY`] are required on every bar so a user can
/// orient before reading any result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSearchBarAnatomyPart {
    /// The corpus-scope label naming the corpus classes searched.
    CorpusScopeLabel,
    /// The provider-availability cue.
    ProviderAvailabilityCue,
    /// The scope-target switcher naming the version / package scope.
    ScopeTargetSwitcher,
    /// The cached / live / mirrored retrieval-mode cue.
    RetrievalModeCue,
    /// The keyboard hint reaching the bar.
    KeyboardHint,
    /// The query input field.
    QueryInputField,
    /// The degraded-state banner (shown when narrowed, degraded, or blocked).
    DegradedStateBanner,
    /// The derived search-availability verdict.
    SearchAvailabilityVerdict,
}

impl M5DocsSearchBarAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CorpusScopeLabel,
        Self::ProviderAvailabilityCue,
        Self::ScopeTargetSwitcher,
        Self::RetrievalModeCue,
        Self::KeyboardHint,
        Self::QueryInputField,
        Self::DegradedStateBanner,
        Self::SearchAvailabilityVerdict,
    ];

    /// The anatomy parts every search bar must render before any result is read.
    pub const MANDATORY: [Self; 4] = [
        Self::CorpusScopeLabel,
        Self::ScopeTargetSwitcher,
        Self::KeyboardHint,
        Self::SearchAvailabilityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusScopeLabel => "corpus_scope_label",
            Self::ProviderAvailabilityCue => "provider_availability_cue",
            Self::ScopeTargetSwitcher => "scope_target_switcher",
            Self::RetrievalModeCue => "retrieval_mode_cue",
            Self::KeyboardHint => "keyboard_hint",
            Self::QueryInputField => "query_input_field",
            Self::DegradedStateBanner => "degraded_state_banner",
            Self::SearchAvailabilityVerdict => "search_availability_verdict",
        }
    }
}

/// Controlled provider-availability state behind a search bar, so a search bar never
/// leaves whether its provider is reachable, degraded, mirror-only, policy-limited,
/// or unavailable implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsProviderAvailability {
    /// The provider is fully available.
    ProviderAvailable,
    /// The provider is reachable but degraded (reduced corpus / slow).
    ProviderDegraded,
    /// Only a mirror of the provider is reachable.
    ProviderMirrorOnly,
    /// Policy limits the corpus this provider may serve.
    ProviderPolicyLimited,
    /// The provider is unavailable / offline.
    ProviderUnavailable,
    /// The provider availability has not yet been evaluated.
    ProviderAvailabilityUnknown,
}

impl M5DocsProviderAvailability {
    /// Every provider-availability state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderAvailable,
        Self::ProviderDegraded,
        Self::ProviderMirrorOnly,
        Self::ProviderPolicyLimited,
        Self::ProviderUnavailable,
        Self::ProviderAvailabilityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAvailable => "provider_available",
            Self::ProviderDegraded => "provider_degraded",
            Self::ProviderMirrorOnly => "provider_mirror_only",
            Self::ProviderPolicyLimited => "provider_policy_limited",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ProviderAvailabilityUnknown => "provider_availability_unknown",
        }
    }
}

/// Controlled retrieval mode — whether results are served live, cached, mirrored,
/// bundled-offline, or from no corpus at all, so a search bar never presents cached
/// or mirrored documentation as live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsRetrievalMode {
    /// Results are served live from the provider.
    LiveRetrieval,
    /// Results are served from a local cache.
    CachedRetrieval,
    /// Results are served from a mirror.
    MirroredRetrieval,
    /// Results are served from a bundled offline copy.
    OfflineBundledRetrieval,
    /// No local corpus is available to serve results.
    NoCorpusAvailable,
    /// The retrieval mode has not yet been evaluated.
    RetrievalModeUnknown,
}

impl M5DocsRetrievalMode {
    /// Every retrieval mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveRetrieval,
        Self::CachedRetrieval,
        Self::MirroredRetrieval,
        Self::OfflineBundledRetrieval,
        Self::NoCorpusAvailable,
        Self::RetrievalModeUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveRetrieval => "live_retrieval",
            Self::CachedRetrieval => "cached_retrieval",
            Self::MirroredRetrieval => "mirrored_retrieval",
            Self::OfflineBundledRetrieval => "offline_bundled_retrieval",
            Self::NoCorpusAvailable => "no_corpus_available",
            Self::RetrievalModeUnknown => "retrieval_mode_unknown",
        }
    }
}

/// The derived headline search-availability posture of a search bar — the resolver's
/// verdict about whether the search is live-ready, cached-ready, mirrored-ready,
/// narrowed, degraded, or blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSearchAvailability {
    /// Ready: provider available, results served live.
    SearchLiveReady,
    /// Ready: results served from a disclosed cache.
    SearchCachedReady,
    /// Ready: results served from a disclosed mirror.
    SearchMirroredReady,
    /// Narrowed: the provider is degraded and the corpus is reduced.
    NarrowedProviderDegraded,
    /// Narrowed: policy limits the corpus this search may reach.
    NarrowedPolicyLimited,
    /// Degraded: the provider is unavailable and results are served from a copy.
    DegradedProviderUnavailable,
    /// Degraded: no local corpus covers the query while offline.
    DegradedOfflineNoCorpus,
    /// Blocked: the search availability is unknown / not yet evaluated.
    BlockedUnknownState,
}

impl M5DocsSearchAvailability {
    /// Every search-availability posture, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SearchLiveReady,
        Self::SearchCachedReady,
        Self::SearchMirroredReady,
        Self::NarrowedProviderDegraded,
        Self::NarrowedPolicyLimited,
        Self::DegradedProviderUnavailable,
        Self::DegradedOfflineNoCorpus,
        Self::BlockedUnknownState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchLiveReady => "search_live_ready",
            Self::SearchCachedReady => "search_cached_ready",
            Self::SearchMirroredReady => "search_mirrored_ready",
            Self::NarrowedProviderDegraded => "narrowed_provider_degraded",
            Self::NarrowedPolicyLimited => "narrowed_policy_limited",
            Self::DegradedProviderUnavailable => "degraded_provider_unavailable",
            Self::DegradedOfflineNoCorpus => "degraded_offline_no_corpus",
            Self::BlockedUnknownState => "blocked_unknown_state",
        }
    }

    /// True when the search is ready to return results (possibly from a disclosed
    /// cache or mirror).
    pub const fn is_ready(self) -> bool {
        matches!(
            self,
            Self::SearchLiveReady | Self::SearchCachedReady | Self::SearchMirroredReady
        )
    }

    /// True when the search is narrowed below a clean live-ready claim.
    pub const fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::NarrowedProviderDegraded | Self::NarrowedPolicyLimited
        )
    }

    /// True when the search is degraded (offline or provider-unavailable).
    pub const fn is_degraded(self) -> bool {
        matches!(
            self,
            Self::DegradedProviderUnavailable | Self::DegradedOfflineNoCorpus
        )
    }

    /// True when the search is blocked on an unknown state.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BlockedUnknownState)
    }

    /// The specific search-limit reason for a narrowed, degraded, or blocked
    /// posture, if any. Returns `None` for a ready posture.
    pub const fn limit_reason(self) -> Option<M5DocsSearchLimitReason> {
        Some(match self {
            Self::NarrowedProviderDegraded => {
                M5DocsSearchLimitReason::ProviderDegradedReducedCorpus
            }
            Self::NarrowedPolicyLimited => M5DocsSearchLimitReason::PolicyLimitedScope,
            Self::DegradedProviderUnavailable => {
                M5DocsSearchLimitReason::ProviderUnavailableOffline
            }
            Self::DegradedOfflineNoCorpus => M5DocsSearchLimitReason::NoLocalCorpusOffline,
            Self::BlockedUnknownState => M5DocsSearchLimitReason::SearchStateUnknown,
            Self::SearchLiveReady | Self::SearchCachedReady | Self::SearchMirroredReady => {
                return None
            }
        })
    }
}

/// The exact reason a search is narrowed, degraded, or blocked, so a degraded search
/// bar never returns empty results with no explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSearchLimitReason {
    /// The provider is degraded and the searchable corpus is reduced.
    ProviderDegradedReducedCorpus,
    /// Policy limits the corpus this search may reach.
    PolicyLimitedScope,
    /// The provider is unavailable and results are served from a local copy.
    ProviderUnavailableOffline,
    /// No local corpus covers the query while offline.
    NoLocalCorpusOffline,
    /// The search availability has not yet been evaluated.
    SearchStateUnknown,
}

impl M5DocsSearchLimitReason {
    /// Every limit reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderDegradedReducedCorpus,
        Self::PolicyLimitedScope,
        Self::ProviderUnavailableOffline,
        Self::NoLocalCorpusOffline,
        Self::SearchStateUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderDegradedReducedCorpus => "provider_degraded_reduced_corpus",
            Self::PolicyLimitedScope => "policy_limited_scope",
            Self::ProviderUnavailableOffline => "provider_unavailable_offline",
            Self::NoLocalCorpusOffline => "no_local_corpus_offline",
            Self::SearchStateUnknown => "search_state_unknown",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ProviderDegradedReducedCorpus => {
                "the docs provider is degraded and the searchable corpus is reduced"
            }
            Self::PolicyLimitedScope => "policy limits the docs corpus this search may reach",
            Self::ProviderUnavailableOffline => {
                "the docs provider is unavailable and results are served from a local copy"
            }
            Self::NoLocalCorpusOffline => "no local docs corpus covers this query while offline",
            Self::SearchStateUnknown => "the docs search availability has not yet been evaluated",
        }
    }

    /// The next action a user should take to widen or clear this reason.
    pub const fn next_action(self) -> M5DocsSearchNextAction {
        match self {
            Self::ProviderDegradedReducedCorpus => M5DocsSearchNextAction::UseCachedCorpus,
            Self::PolicyLimitedScope => M5DocsSearchNextAction::RequestPolicyAccess,
            Self::ProviderUnavailableOffline => M5DocsSearchNextAction::RetryWhenOnline,
            Self::NoLocalCorpusOffline => M5DocsSearchNextAction::ImportOrHandOffToBrowser,
            Self::SearchStateUnknown => M5DocsSearchNextAction::RunAvailabilityCheck,
        }
    }
}

/// The next action named on a degraded-state banner, so a narrowed or offline search
/// is actionable from the bar itself rather than from a dead-end empty result list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSearchNextAction {
    /// Search the cached corpus.
    UseCachedCorpus,
    /// Request policy access to widen the corpus.
    RequestPolicyAccess,
    /// Retry the search when the provider is back online.
    RetryWhenOnline,
    /// Import a docs pack or hand off to a browser.
    ImportOrHandOffToBrowser,
    /// Run the availability check.
    RunAvailabilityCheck,
}

impl M5DocsSearchNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UseCachedCorpus,
        Self::RequestPolicyAccess,
        Self::RetryWhenOnline,
        Self::ImportOrHandOffToBrowser,
        Self::RunAvailabilityCheck,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseCachedCorpus => "use_cached_corpus",
            Self::RequestPolicyAccess => "request_policy_access",
            Self::RetryWhenOnline => "retry_when_online",
            Self::ImportOrHandOffToBrowser => "import_or_hand_off_to_browser",
            Self::RunAvailabilityCheck => "run_availability_check",
        }
    }
}

/// A field the support / export packet carries so search-bar and scope-switcher
/// truth is reconstructable from the shared model. The fields in
/// [`M5DocsSearchBarExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSearchBarExportField {
    /// The corpus classes searched.
    CorpusClass,
    /// The source provider.
    SourceProvider,
    /// The provider availability.
    ProviderAvailability,
    /// The retrieval mode (cached / live / mirrored).
    RetrievalMode,
    /// The version / package scope.
    VersionScope,
    /// The opaque scope-target representation.
    ScopeTarget,
    /// The keyboard hint.
    KeyboardHint,
    /// The derived search-availability posture.
    SearchAvailability,
    /// The search-limit reason (when narrowed, degraded, or blocked).
    LimitReason,
    /// The next action (when narrowed, degraded, or blocked).
    NextAction,
}

impl M5DocsSearchBarExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::CorpusClass,
        Self::SourceProvider,
        Self::ProviderAvailability,
        Self::RetrievalMode,
        Self::VersionScope,
        Self::ScopeTarget,
        Self::KeyboardHint,
        Self::SearchAvailability,
        Self::LimitReason,
        Self::NextAction,
    ];

    /// The export fields every search-bar export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::CorpusClass,
        Self::SourceProvider,
        Self::ProviderAvailability,
        Self::RetrievalMode,
        Self::SearchAvailability,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusClass => "corpus_class",
            Self::SourceProvider => "source_provider",
            Self::ProviderAvailability => "provider_availability",
            Self::RetrievalMode => "retrieval_mode",
            Self::VersionScope => "version_scope",
            Self::ScopeTarget => "scope_target",
            Self::KeyboardHint => "keyboard_hint",
            Self::SearchAvailability => "search_availability",
            Self::LimitReason => "limit_reason",
            Self::NextAction => "next_action",
        }
    }
}

/// A self-contained degraded-state banner: the exact reason, the corpus in scope,
/// the retrieval mode, and the next action, so a narrowed, offline, mirror-only, or
/// policy-limited search is understood from the banner alone rather than presenting
/// empty results with no explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchDegradedBanner {
    /// The exact limit reason.
    pub reason: M5DocsSearchLimitReason,
    /// The next action a user should take.
    pub next_action: M5DocsSearchNextAction,
    /// The corpus classes still in scope under the limit.
    pub limited_corpus_classes: Vec<M5DocsCorpusClass>,
    /// The retrieval mode results are served from under the limit.
    pub retrieval_mode: M5DocsRetrievalMode,
    /// The provider availability behind the limit.
    pub provider_availability: M5DocsProviderAvailability,
    /// The version / package scope the search is bound to.
    pub version_scope: M5DocsVersionScope,
    /// A deterministic, self-contained headline naming the reason, the corpus, the
    /// retrieval mode, and the next action — never an empty result list.
    pub headline: String,
}

/// The full input to the docs-search resolver for one search bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchResolutionInput {
    /// The opaque, export-safe search-bar label.
    pub search_bar_label: String,
    /// The opaque, export-safe scope target (never left implicit).
    pub scope_target_repr: String,
    /// The corpus classes this bar searches. Must be non-empty so the corpus is
    /// explicit.
    pub corpus_classes: Vec<M5DocsCorpusClass>,
    /// The source provider behind the results.
    pub source_provider: M5DocsSourceProvider,
    /// The provider availability.
    pub provider_availability: M5DocsProviderAvailability,
    /// The retrieval mode (cached / live / mirrored).
    pub retrieval_mode: M5DocsRetrievalMode,
    /// The version / package scope in effect.
    pub version_scope: M5DocsVersionScope,
    /// The opaque keyboard hint reaching the bar. Must be non-empty so the bar is
    /// keyboard complete.
    pub keyboard_hint_repr: String,
    /// The freshness reading disclosed alongside the retrieval mode.
    pub freshness_state: M5DocsFreshnessState,
}

/// The resolved corpus / provider / scope / availability truth for one search bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDocsSearch {
    /// The opaque search-bar label.
    pub search_bar_label: String,
    /// The opaque scope target.
    pub scope_target_repr: String,
    /// The corpus classes searched.
    pub corpus_classes: Vec<M5DocsCorpusClass>,
    /// The count of corpus classes in scope.
    pub corpus_count: usize,
    /// The source provider.
    pub source_provider: M5DocsSourceProvider,
    /// The provider availability.
    pub provider_availability: M5DocsProviderAvailability,
    /// The retrieval mode.
    pub retrieval_mode: M5DocsRetrievalMode,
    /// The version / package scope.
    pub version_scope: M5DocsVersionScope,
    /// The keyboard hint.
    pub keyboard_hint_repr: String,
    /// The freshness reading.
    pub freshness_state: M5DocsFreshnessState,
    /// The derived search-availability posture.
    pub search_availability: M5DocsSearchAvailability,
    /// True when the search is ready.
    pub is_ready: bool,
    /// True when the search is narrowed.
    pub is_narrowed: bool,
    /// True when the search is degraded.
    pub is_degraded: bool,
    /// True when the search is blocked.
    pub is_blocked: bool,
    /// The degraded-state banner, present when narrowed, degraded, or blocked.
    pub degraded_banner: Option<M5DocsSearchDegradedBanner>,
}

/// Errors returned by [`resolve_docs_search`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DocsSearchResolutionError {
    /// The search-bar label was empty.
    EmptySearchBarLabel,
    /// The scope target was empty (scope must be explicit).
    EmptyScopeTarget,
    /// The corpus set was empty (the corpus must be explicit).
    EmptyCorpusSet,
    /// The keyboard hint was empty (the bar must be keyboard complete).
    EmptyKeyboardHint,
    /// A search-bar label, scope target, or keyboard hint carried forbidden
    /// material.
    ForbiddenSearchMaterial,
}

impl M5DocsSearchResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySearchBarLabel => "empty_search_bar_label",
            Self::EmptyScopeTarget => "empty_scope_target",
            Self::EmptyCorpusSet => "empty_corpus_set",
            Self::EmptyKeyboardHint => "empty_keyboard_hint",
            Self::ForbiddenSearchMaterial => "forbidden_search_material",
        }
    }
}

impl fmt::Display for M5DocsSearchResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "docs-search resolution error: {}", self.as_str())
    }
}

impl Error for M5DocsSearchResolutionError {}

/// Resolves one docs search bar from its declared state.
///
/// The derived search-availability posture is the headline verdict, computed in a
/// fixed blocking-first order: an unknown provider availability or unknown retrieval
/// mode blocks first, then a no-corpus retrieval degrades to offline-no-corpus, then
/// an unavailable provider degrades, then a policy-limited provider narrows, then a
/// degraded provider narrows, then a mirror-only provider reads as mirrored-ready,
/// and only an available provider serving live, cached, mirrored, or bundled results
/// reads as ready — with cached and mirrored retrieval kept explicit rather than
/// shown as live. The corpus, provider, scope, and retrieval mode are carried
/// explicitly, and a narrowed, degraded, or blocked search always produces a
/// self-contained banner instead of empty results with no explanation.
pub fn resolve_docs_search(
    input: &M5DocsSearchResolutionInput,
) -> Result<M5ResolvedDocsSearch, M5DocsSearchResolutionError> {
    if input.search_bar_label.trim().is_empty() {
        return Err(M5DocsSearchResolutionError::EmptySearchBarLabel);
    }
    if input.scope_target_repr.trim().is_empty() {
        return Err(M5DocsSearchResolutionError::EmptyScopeTarget);
    }
    if input.corpus_classes.is_empty() {
        return Err(M5DocsSearchResolutionError::EmptyCorpusSet);
    }
    if input.keyboard_hint_repr.trim().is_empty() {
        return Err(M5DocsSearchResolutionError::EmptyKeyboardHint);
    }
    if value_repr_is_forbidden(&input.search_bar_label)
        || value_repr_is_forbidden(&input.scope_target_repr)
        || value_repr_is_forbidden(&input.keyboard_hint_repr)
    {
        return Err(M5DocsSearchResolutionError::ForbiddenSearchMaterial);
    }

    let search_availability =
        derive_search_availability(input.provider_availability, input.retrieval_mode);

    let is_ready = search_availability.is_ready();
    let is_narrowed = search_availability.is_narrowed();
    let is_degraded = search_availability.is_degraded();
    let is_blocked = search_availability.is_blocked();

    let degraded_banner = search_availability.limit_reason().map(|reason| {
        let next_action = reason.next_action();
        let headline = format!(
            "Docs search limited: {} — {} corpus class(es) in {} scope, {} retrieval; next: {}",
            reason.phrase(),
            input.corpus_classes.len(),
            input.version_scope.as_str(),
            input.retrieval_mode.as_str(),
            next_action.as_str()
        );
        M5DocsSearchDegradedBanner {
            reason,
            next_action,
            limited_corpus_classes: input.corpus_classes.clone(),
            retrieval_mode: input.retrieval_mode,
            provider_availability: input.provider_availability,
            version_scope: input.version_scope,
            headline,
        }
    });

    Ok(M5ResolvedDocsSearch {
        search_bar_label: input.search_bar_label.clone(),
        scope_target_repr: input.scope_target_repr.clone(),
        corpus_classes: input.corpus_classes.clone(),
        corpus_count: input.corpus_classes.len(),
        source_provider: input.source_provider,
        provider_availability: input.provider_availability,
        retrieval_mode: input.retrieval_mode,
        version_scope: input.version_scope,
        keyboard_hint_repr: input.keyboard_hint_repr.clone(),
        freshness_state: input.freshness_state,
        search_availability,
        is_ready,
        is_narrowed,
        is_degraded,
        is_blocked,
        degraded_banner,
    })
}

/// The fixed blocking-first search-availability ladder.
fn derive_search_availability(
    provider: M5DocsProviderAvailability,
    retrieval: M5DocsRetrievalMode,
) -> M5DocsSearchAvailability {
    use M5DocsProviderAvailability as Provider;
    use M5DocsRetrievalMode as Retrieval;

    if matches!(provider, Provider::ProviderAvailabilityUnknown)
        || matches!(retrieval, Retrieval::RetrievalModeUnknown)
    {
        M5DocsSearchAvailability::BlockedUnknownState
    } else if matches!(retrieval, Retrieval::NoCorpusAvailable) {
        M5DocsSearchAvailability::DegradedOfflineNoCorpus
    } else if matches!(provider, Provider::ProviderUnavailable) {
        M5DocsSearchAvailability::DegradedProviderUnavailable
    } else if matches!(provider, Provider::ProviderPolicyLimited) {
        M5DocsSearchAvailability::NarrowedPolicyLimited
    } else if matches!(provider, Provider::ProviderDegraded) {
        M5DocsSearchAvailability::NarrowedProviderDegraded
    } else if matches!(provider, Provider::ProviderMirrorOnly)
        || matches!(retrieval, Retrieval::MirroredRetrieval)
    {
        M5DocsSearchAvailability::SearchMirroredReady
    } else if matches!(
        retrieval,
        Retrieval::CachedRetrieval | Retrieval::OfflineBundledRetrieval
    ) {
        M5DocsSearchAvailability::SearchCachedReady
    } else {
        M5DocsSearchAvailability::SearchLiveReady
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs search-bar truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchResolutionCase {
    /// The resolver input.
    pub input: M5DocsSearchResolutionInput,
    /// The resolved truth. Must equal `resolve_docs_search(&input)`.
    pub resolved: M5ResolvedDocsSearch,
}

impl M5DocsSearchResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DocsSearchResolutionInput) -> Self {
        let resolved = resolve_docs_search(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_docs_search(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one docs-search consumer bound to the shared
/// search-bar anatomy, availability postures, provider availabilities, retrieval
/// modes, limit reasons, next actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchRow {
    /// Docs-search consumer family.
    pub consumer_surface: M5DocsSearchConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5DocsQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 docs surface families that render / consume this bar.
    pub surface_families: Vec<M5DocsSurfaceFamily>,
    /// Deployment lines this bar keeps the same truth across.
    pub deployment_lines: Vec<M5DocsDeploymentLine>,
    /// Anatomy parts this bar renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5DocsSearchBarAnatomyPart>,
    /// Corpus classes this bar names.
    pub corpus_classes: Vec<M5DocsCorpusClass>,
    /// Source providers this bar names.
    pub source_providers: Vec<M5DocsSourceProvider>,
    /// Provider availabilities this bar distinguishes.
    pub provider_availabilities: Vec<M5DocsProviderAvailability>,
    /// Retrieval modes this bar distinguishes.
    pub retrieval_modes: Vec<M5DocsRetrievalMode>,
    /// Version scopes this bar names.
    pub version_scopes: Vec<M5DocsVersionScope>,
    /// Freshness states this bar discloses.
    pub freshness_states: Vec<M5DocsFreshnessState>,
    /// Search-availability postures this bar distinguishes.
    pub search_availabilities: Vec<M5DocsSearchAvailability>,
    /// Search-limit reasons this bar names.
    pub limit_reasons: Vec<M5DocsSearchLimitReason>,
    /// Next actions this bar names.
    pub next_actions: Vec<M5DocsSearchNextAction>,
    /// Export fields this bar carries (must include the mandatory fields).
    pub export_fields: Vec<M5DocsSearchBarExportField>,
    /// Non-visual accessibility routes this bar offers.
    pub accessibility_routes: Vec<M5DocsAccessibilityRoute>,
    /// Docs subsystems that consume this bar's projection.
    pub consumer_surfaces: Vec<M5DocsConsumerSurface>,
    /// Downgrade triggers that apply to this bar.
    pub downgrade_triggers: Vec<M5DocsDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5DocsSearchResolutionCase>,
    /// Hard invariant: this bar never masks the corpus class or the source provider.
    /// MUST be `false`.
    pub masks_corpus_or_provider: bool,
    /// Hard invariant: this bar never shows cached or mirrored content as live.
    /// MUST be `false`.
    pub shows_cached_or_mirrored_as_live: bool,
    /// Hard invariant: this bar never invents a private search grammar. MUST be
    /// `false`.
    pub invents_private_search_grammar: bool,
    /// Hard invariant: this bar never hides a degraded-state reason behind empty
    /// results. MUST be `false`.
    pub hides_degraded_state_reason: bool,
}

impl M5DocsSearchRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DocsSearchBarAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DocsSearchBarAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DocsSearchBarExportField> =
            self.export_fields.iter().copied().collect();
        M5DocsSearchBarExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_corpus_or_provider
            && !self.shows_cached_or_mirrored_as_live
            && !self.invents_private_search_grammar
            && !self.hides_degraded_state_reason
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchVocabularySet {
    /// Docs-search consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Provider-availability tokens.
    pub provider_availabilities: Vec<String>,
    /// Retrieval-mode tokens.
    pub retrieval_modes: Vec<String>,
    /// Search-availability-posture tokens.
    pub search_availabilities: Vec<String>,
    /// Search-limit-reason tokens.
    pub limit_reasons: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Corpus-class tokens (reused from the frozen matrix).
    pub corpus_classes: Vec<String>,
    /// Version-scope tokens (reused from the frozen matrix).
    pub version_scopes: Vec<String>,
    /// Source-provider tokens (reused from the frozen matrix).
    pub source_providers: Vec<String>,
    /// Freshness-state tokens (reused from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5DocsSearchVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5DocsSearchConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DocsSearchBarAnatomyPart::ALL, |v| v.as_str()),
            provider_availabilities: tokens(&M5DocsProviderAvailability::ALL, |v| v.as_str()),
            retrieval_modes: tokens(&M5DocsRetrievalMode::ALL, |v| v.as_str()),
            search_availabilities: tokens(&M5DocsSearchAvailability::ALL, |v| v.as_str()),
            limit_reasons: tokens(&M5DocsSearchLimitReason::ALL, |v| v.as_str()),
            next_actions: tokens(&M5DocsSearchNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DocsSearchBarExportField::ALL, |v| v.as_str()),
            corpus_classes: tokens(&M5DocsCorpusClass::ALL, |v| v.as_str()),
            version_scopes: tokens(&M5DocsVersionScope::ALL, |v| v.as_str()),
            source_providers: tokens(&M5DocsSourceProvider::ALL, |v| v.as_str()),
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
pub struct M5DocsSearchGovernanceReview {
    /// One search-bar primitive carries corpus, provider, scope, and retrieval truth
    /// on every consumer.
    pub one_primitive_carries_search_truth: bool,
    /// The corpus class and source provider are shown before any result is read.
    pub corpus_and_provider_always_shown: bool,
    /// The version / package scope is explicit, never left implicit.
    pub scope_always_explicit: bool,
    /// Cached or mirrored content is never shown as live.
    pub cached_or_mirrored_never_shown_as_live: bool,
    /// The keyboard hint keeps the bar keyboard complete on every consumer.
    pub keyboard_hint_keeps_bar_complete: bool,
    /// A narrowed, degraded, or blocked search always shows a self-contained banner.
    pub degraded_state_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and next action, never empty results.
    pub banner_names_exact_reason_and_next_action: bool,
    /// The support / export packet reconstructs search-bar truth.
    pub support_export_reconstructs_search_truth: bool,
    /// No consumer invents a second search-bar grammar.
    pub no_surface_invents_second_search_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel search-bar vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchConsumerProjection {
    /// Docs-browser, onboarding, AI, support/help, and CLI consumers all consume the
    /// shared primitive.
    pub search_surfaces_consume_shared_primitive: bool,
    /// The availability resolver reads a single canonical source.
    pub availability_resolver_reads_single_source: bool,
    /// The provider-availability cue reads a single canonical source.
    pub provider_availability_reads_single_source: bool,
    /// The retrieval-mode cue reads a single canonical source.
    pub retrieval_mode_reads_single_source: bool,
    /// Support / export reads a single canonical search-bar source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the search-bar primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchReleasePosture {
    /// Ref of the supporting proof packet.
    pub proof_packet_ref: String,
    /// Ref of the supporting search-bar audit.
    pub search_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DocsSearchPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsSearchPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Search rows.
    pub search_rows: Vec<M5DocsSearchRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsSearchVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsSearchGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsSearchConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsSearchProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsSearchReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 docs-search-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsSearchPrimitivePacket {
    /// Record kind; must equal [`M5_DOCS_SEARCH_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_SEARCH_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Search rows.
    pub search_rows: Vec<M5DocsSearchRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsSearchVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsSearchGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsSearchConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsSearchProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsSearchReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsSearchPrimitivePacket {
    /// Builds an M5 docs-search-primitive packet from stable-lane input.
    pub fn new(input: M5DocsSearchPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_SEARCH_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_SEARCH_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            search_rows: input.search_rows,
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

    /// Validates the M5 docs-search-primitive invariants.
    pub fn validate(&self) -> Vec<M5DocsSearchPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_SEARCH_PRIMITIVE_RECORD_KIND {
            violations.push(M5DocsSearchPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_SEARCH_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5DocsSearchPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsSearchPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_search_rows(self, &mut violations);
        validate_availability_coverage(self, &mut violations);
        validate_scope_and_keyboard_explicit(self, &mut violations);
        validate_degraded_banner_calm_explicit(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 docs-search primitive packet serializes"),
        ) {
            violations.push(M5DocsSearchPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 docs-search primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per docs-search consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,search_availabilities,provider_availabilities,retrieval_modes,limit_reasons,next_actions,export_fields,example_count\n",
        );
        for row in &self.search_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.search_availabilities, |v| v.as_str()),
                join_tokens(&row.provider_availabilities, |v| v.as_str()),
                join_tokens(&row.retrieval_modes, |v| v.as_str()),
                join_tokens(&row.limit_reasons, |v| v.as_str()),
                join_tokens(&row.next_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .search_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Docs-Search-Bar and Scope-Switcher Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Docs-search consumers: {} ({} stable)\n",
            self.search_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Search-availability postures: {}\n",
            self.vocabulary_set.search_availabilities.join(", ")
        ));
        out.push_str(&format!(
            "- Provider availabilities: {}\n",
            self.vocabulary_set.provider_availabilities.join(", ")
        ));
        out.push_str(&format!(
            "- Retrieval modes: {}\n",
            self.vocabulary_set.retrieval_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Limit reasons: {}\n",
            self.vocabulary_set.limit_reasons.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Docs-search consumers\n\n");
        for row in &self.search_rows {
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
                let banner = match &case.resolved.degraded_banner {
                    Some(banner) => banner.reason.as_str(),
                    None => "ready",
                };
                out.push_str(&format!(
                    "    - `{}` via `{}` → `{}` (retrieval `{}`, freshness `{}`, banner `{}`)\n",
                    case.resolved.scope_target_repr,
                    case.resolved.provider_availability.as_str(),
                    case.resolved.search_availability.as_str(),
                    case.resolved.retrieval_mode.as_str(),
                    case.resolved.freshness_state.as_str(),
                    banner
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 docs-search-primitive export.
#[derive(Debug)]
pub enum M5DocsSearchPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsSearchPrimitiveViolation>),
}

impl fmt::Display for M5DocsSearchPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 docs-search primitive export parse failed: {error}"
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
                    "m5 docs-search primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsSearchPrimitiveArtifactError {}

/// Validation failures emitted by [`M5DocsSearchPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsSearchPrimitiveViolation {
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
    /// A required docs-search consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A search row is incomplete.
    SearchRowIncomplete,
    /// A search row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A search row declares no provider availabilities.
    ProviderAvailabilityMissing,
    /// A search row declares no retrieval modes.
    RetrievalModeMissing,
    /// A search row declares no search-availability postures.
    SearchAvailabilityMissing,
    /// A search row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A search row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A search row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A search row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A search row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A search row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves both a ready and a not-ready search.
    AvailabilityCoverageUnproven,
    /// No worked resolution proves an explicit scope target and keyboard hint.
    ScopeAndKeyboardExplicitUnproven,
    /// No worked resolution proves a degraded search with a self-contained banner.
    DegradedBannerCalmExplicitUnproven,
    /// A search row violates a hard invariant.
    SearchInvariantViolated,
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

impl M5DocsSearchPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::SearchRowIncomplete => "search_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::ProviderAvailabilityMissing => "provider_availability_missing",
            Self::RetrievalModeMissing => "retrieval_mode_missing",
            Self::SearchAvailabilityMissing => "search_availability_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::AvailabilityCoverageUnproven => "availability_coverage_unproven",
            Self::ScopeAndKeyboardExplicitUnproven => "scope_and_keyboard_explicit_unproven",
            Self::DegradedBannerCalmExplicitUnproven => "degraded_banner_calm_explicit_unproven",
            Self::SearchInvariantViolated => "search_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 docs-search-primitive export.
pub fn current_stable_m5_docs_search_primitive_export(
) -> Result<M5DocsSearchPrimitivePacket, M5DocsSearchPrimitiveArtifactError> {
    let packet: M5DocsSearchPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/support_export.json"
    )))
    .map_err(M5DocsSearchPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsSearchPrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_SEARCH_SCHEMA_REF,
        M5_DOCS_SEARCH_DOC_REF,
        M5_DOCS_SEARCH_COMPONENT_MATRIX_REF,
        M5_DOCS_SEARCH_SOURCE_RESULT_REF,
        M5_DOCS_SEARCH_SOURCE_PRECEDENCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsSearchPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DocsSearchPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_search_rows(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    let present: BTreeSet<M5DocsSearchConsumerSurface> = packet
        .search_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5DocsSearchConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5DocsSearchPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.search_rows {
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
            || row.limit_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5DocsSearchPrimitiveViolation::SearchRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DocsSearchPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.provider_availabilities.is_empty() {
            violations.push(M5DocsSearchPrimitiveViolation::ProviderAvailabilityMissing);
        }
        if row.retrieval_modes.is_empty() {
            violations.push(M5DocsSearchPrimitiveViolation::RetrievalModeMissing);
        }
        if row.search_availabilities.is_empty() {
            violations.push(M5DocsSearchPrimitiveViolation::SearchAvailabilityMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DocsSearchPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5DocsAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5DocsSearchPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DocsSearchPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DocsSearchPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5DocsSearchPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DocsSearchPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DocsSearchPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DocsSearchPrimitiveViolation::SearchInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove a ready search and at
/// least one must prove a not-ready (narrowed, degraded, or blocked) search — the
/// acceptance-criterion example that a user can tell live from cached, mirrored, or
/// narrowed before they act.
fn validate_availability_coverage(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    let has_ready = packet.search_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_ready)
    });
    let has_not_ready = packet.search_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_narrowed || case.resolved.is_degraded || case.resolved.is_blocked
        })
    });
    if !(has_ready && has_not_ready) {
        violations.push(M5DocsSearchPrimitiveViolation::AvailabilityCoverageUnproven);
    }
}

/// At least one worked resolution across the matrix must carry a non-empty scope
/// target and a non-empty keyboard hint — the acceptance-criterion example that the
/// scope is explicit and the bar stays keyboard complete.
fn validate_scope_and_keyboard_explicit(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    let proven = packet.search_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            !case.resolved.scope_target_repr.trim().is_empty()
                && !case.resolved.keyboard_hint_repr.trim().is_empty()
        })
    });
    if !proven {
        violations.push(M5DocsSearchPrimitiveViolation::ScopeAndKeyboardExplicitUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a narrowed, degraded,
/// or blocked search whose banner carries a specific reason, a next action, the
/// corpus in scope, and a non-empty headline — the acceptance-criterion example that
/// a degraded lookup degrades to calm explicit messaging rather than empty results.
fn validate_degraded_banner_calm_explicit(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    let proven = packet.search_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            (case.resolved.is_narrowed || case.resolved.is_degraded || case.resolved.is_blocked)
                && case
                    .resolved
                    .degraded_banner
                    .as_ref()
                    .is_some_and(|banner| {
                        !banner.headline.trim().is_empty()
                            && !banner.limited_corpus_classes.is_empty()
                    })
        })
    });
    if !proven {
        violations.push(M5DocsSearchPrimitiveViolation::DegradedBannerCalmExplicitUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_search_truth,
        review.corpus_and_provider_always_shown,
        review.scope_always_explicit,
        review.cached_or_mirrored_never_shown_as_live,
        review.keyboard_hint_keeps_bar_complete,
        review.degraded_state_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_next_action,
        review.support_export_reconstructs_search_truth,
        review.no_surface_invents_second_search_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DocsSearchPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.search_surfaces_consume_shared_primitive,
        projection.availability_resolver_reads_single_source,
        projection.provider_availability_reads_single_source,
        projection.retrieval_mode_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DocsSearchPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsSearchPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DocsSearchPrimitivePacket,
    violations: &mut Vec<M5DocsSearchPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.search_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DocsSearchPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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
