//! Frozen M5 docs-search-bar, docs-scope-switcher, docs-result-row,
//! symbol-linked-reference-card, docs-source/version-badge, docs-pack-row,
//! stale-example-finding-row, and docs-handoff-banner component matrix.
//!
//! This module locks Aureline's reusable documentation-browser and
//! knowledge-surface components into one export-safe packet. Every component
//! family M5 claims that still drifts too easily by search palette, hover peek,
//! onboarding tour, or AI-context panel — the docs search bar, the scope
//! switcher, the result row, the symbol-linked reference card, the source/version
//! badge, the docs-pack row, the stale-example finding row, and the
//! browser-handoff banner — is named once here and constrained by the same
//! corpus-class, provider/source, version/package-scope, symbol-anchor,
//! project-doc-override, freshness, pin/mirror/offline/quarantine, stale-example,
//! and browser-handoff-reason rules regardless of the surface family that renders
//! it.
//!
//! What this matrix freezes is the stable vocabulary for the *components*
//! themselves: the component families, the corpus classes, the version/package
//! scopes, the search match states and project-doc override reasons, the symbol
//! anchors, the source providers and freshness states, the docs-pack states, the
//! stale-example statuses, the browser-handoff reasons, the deployment lines every
//! component must survive, the non-visual accessibility routes, and the mandatory
//! labels every component must be able to show. It does not re-architect docs
//! retrieval, citation assembly, or docs-pack distribution that already own those
//! records — it is the shared component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 docs, help,
//! onboarding, or AI surface may publish a corpus, source, version, symbol,
//! override, freshness, pack-state, stale-example, or handoff claim. Docs-browser,
//! help-center, onboarding, AI-context, search-palette, hover-peek, support, and
//! admin surfaces all consume this packet so one search bar names the corpus it is
//! searching, one scope switcher names its version/package scope, one result row
//! states its match state and why a project doc outranked vendor docs, one symbol
//! card names its anchor and resolution, one source/version badge names its
//! provider and freshness, one docs-pack row states whether the pack is pinned,
//! mirrored, offline, or quarantined, one stale-example row states its staleness,
//! and one handoff banner states exactly why Aureline had to hand off to a
//! browser. No M5 lane invents a second docs-status grammar, masks a corpus or
//! source provenance, shows stale or cached documentation as live, or hides a
//! handoff or override reason.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5DocsBrowserVocabularySet`] rather than minted per surface. Raw URLs, raw
//! tokens, credentials, private endpoints, and user text bodies stay outside the
//! support boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_docs_browser_component_matrix,
    seeded_m5_docs_browser_component_matrix_handoff_banner_preview_narrowed,
    seeded_m5_docs_browser_component_matrix_stale_example_finding_row_beta_narrowed,
    M5_DOCS_BROWSER_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DocsBrowserMatrixPacket`].
pub const M5_DOCS_BROWSER_MATRIX_RECORD_KIND: &str =
    "freeze_m5_docs_search_bar_result_row_symbol_linked_reference_card_docs_source_version_badge_docs_pack_row_stale_example_finding_row_and_handoff_banner_component_matrix";

/// Schema version for M5 docs-browser-component-matrix records.
pub const M5_DOCS_BROWSER_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the docs-browser-components boundary schema.
pub const M5_DOCS_BROWSER_SCHEMA_REF: &str =
    "schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DOCS_BROWSER_DOC_REF: &str =
    "docs/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix.md";

/// Repo-relative path of the stable docs-source/result/pack/citation object
/// contract this matrix binds against.
pub const M5_DOCS_BROWSER_SOURCE_RESULT_REF: &str =
    "schemas/docs/stable_docs_source_result_pack_and_citation.schema.json";

/// Repo-relative path of the symbol-linked-reference contract this matrix binds
/// against.
pub const M5_DOCS_BROWSER_SYMBOL_REF: &str = "schemas/docs/symbol_linked_reference.schema.json";

/// Repo-relative path of the docs-pack-manifest contract this matrix binds
/// against.
pub const M5_DOCS_BROWSER_PACK_REF: &str = "schemas/docs/docs_pack_manifest.schema.json";

/// Repo-relative path of the browser-handoff-packet contract this matrix binds
/// against.
pub const M5_DOCS_BROWSER_HANDOFF_REF: &str =
    "schemas/integration/browser_handoff_packet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_BROWSER_FIXTURE_DIR: &str =
    "fixtures/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_BROWSER_ARTIFACT_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DOCS_BROWSER_CSV_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DOCS_BROWSER_REPORT_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix.md";

/// One of the eight governed docs-browser component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsBrowserComponentFamily {
    /// A docs search bar carrying the corpus classes it searches.
    DocsSearchBar,
    /// A docs scope switcher naming the version / package scope in effect.
    DocsScopeSwitcher,
    /// A docs result row carrying its match state and project-doc override reason.
    DocsResultRow,
    /// A symbol-linked reference card carrying its symbol anchor and resolution.
    SymbolLinkedReferenceCard,
    /// A docs source / version badge naming provider and freshness.
    DocsSourceVersionBadge,
    /// A docs-pack row naming pin / mirror / offline / quarantine state.
    DocsPackRow,
    /// A stale-example finding row naming its stale-example status.
    StaleExampleFindingRow,
    /// A browser-handoff banner naming exactly why Aureline handed off.
    DocsHandoffBanner,
}

impl M5DocsBrowserComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DocsSearchBar,
        Self::DocsScopeSwitcher,
        Self::DocsResultRow,
        Self::SymbolLinkedReferenceCard,
        Self::DocsSourceVersionBadge,
        Self::DocsPackRow,
        Self::StaleExampleFindingRow,
        Self::DocsHandoffBanner,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSearchBar => "docs_search_bar",
            Self::DocsScopeSwitcher => "docs_scope_switcher",
            Self::DocsResultRow => "docs_result_row",
            Self::SymbolLinkedReferenceCard => "symbol_linked_reference_card",
            Self::DocsSourceVersionBadge => "docs_source_version_badge",
            Self::DocsPackRow => "docs_pack_row",
            Self::StaleExampleFindingRow => "stale_example_finding_row",
            Self::DocsHandoffBanner => "docs_handoff_banner",
        }
    }

    /// `true` when this family is a docs search bar and must therefore declare the
    /// corpus classes it searches.
    pub const fn is_search_bar(self) -> bool {
        matches!(self, Self::DocsSearchBar)
    }

    /// `true` when this family is a docs scope switcher and must therefore declare
    /// its version / package scopes.
    pub const fn is_scope_switcher(self) -> bool {
        matches!(self, Self::DocsScopeSwitcher)
    }

    /// `true` when this family is a docs result row and must therefore declare its
    /// match states and project-doc override reasons.
    pub const fn is_result_row(self) -> bool {
        matches!(self, Self::DocsResultRow)
    }

    /// `true` when this family is a symbol-linked reference card and must therefore
    /// declare its symbol anchors.
    pub const fn is_symbol_card(self) -> bool {
        matches!(self, Self::SymbolLinkedReferenceCard)
    }

    /// `true` when this family is a source / version badge and must therefore
    /// declare its source providers and freshness states.
    pub const fn is_source_badge(self) -> bool {
        matches!(self, Self::DocsSourceVersionBadge)
    }

    /// `true` when this family is a docs-pack row and must therefore declare its
    /// pack states.
    pub const fn is_pack_row(self) -> bool {
        matches!(self, Self::DocsPackRow)
    }

    /// `true` when this family is a stale-example finding row and must therefore
    /// declare its stale-example statuses.
    pub const fn is_stale_example(self) -> bool {
        matches!(self, Self::StaleExampleFindingRow)
    }

    /// `true` when this family is a browser-handoff banner and must therefore
    /// declare its handoff reasons.
    pub const fn is_handoff_banner(self) -> bool {
        matches!(self, Self::DocsHandoffBanner)
    }
}

/// Controlled corpus class — what body of documentation a component is drawn from,
/// so a search bar or result never leaves the corpus implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsCorpusClass {
    /// First-party Aureline documentation.
    FirstPartyDocs,
    /// API reference documentation.
    ApiReference,
    /// A guide or tutorial.
    GuideTutorial,
    /// A codebase symbol / source-derived doc.
    CodebaseSymbol,
    /// Community-contributed documentation.
    CommunityContributed,
    /// Vendor / dependency documentation.
    VendorDependency,
    /// Release notes / changelog.
    ReleaseNotesChangelog,
}

impl M5DocsCorpusClass {
    /// Every corpus class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FirstPartyDocs,
        Self::ApiReference,
        Self::GuideTutorial,
        Self::CodebaseSymbol,
        Self::CommunityContributed,
        Self::VendorDependency,
        Self::ReleaseNotesChangelog,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyDocs => "first_party_docs",
            Self::ApiReference => "api_reference",
            Self::GuideTutorial => "guide_tutorial",
            Self::CodebaseSymbol => "codebase_symbol",
            Self::CommunityContributed => "community_contributed",
            Self::VendorDependency => "vendor_dependency",
            Self::ReleaseNotesChangelog => "release_notes_changelog",
        }
    }
}

/// Controlled version / package scope — how tightly a component is bound to a
/// version, so a scope switcher never leaves the version scope implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsVersionScope {
    /// An exact version match.
    ExactVersionMatch,
    /// A nearby version (close but not exact).
    NearbyVersion,
    /// Project-specific documentation.
    ProjectSpecific,
    /// The latest stable version.
    LatestStable,
    /// A pinned version range.
    PinnedRange,
    /// Unversioned documentation.
    Unversioned,
}

impl M5DocsVersionScope {
    /// Every version scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactVersionMatch,
        Self::NearbyVersion,
        Self::ProjectSpecific,
        Self::LatestStable,
        Self::PinnedRange,
        Self::Unversioned,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactVersionMatch => "exact_version_match",
            Self::NearbyVersion => "nearby_version",
            Self::ProjectSpecific => "project_specific",
            Self::LatestStable => "latest_stable",
            Self::PinnedRange => "pinned_range",
            Self::Unversioned => "unversioned",
        }
    }
}

/// Controlled result match state — how a result relates to the query and the local
/// corpus, so a result row never presents a nearby, mirrored, cached, or stale
/// match as an exact live one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsMatchState {
    /// An exact match.
    ExactMatch,
    /// A nearby / approximate match.
    NearbyMatch,
    /// A project-specific match.
    ProjectSpecificMatch,
    /// A match served from a mirror.
    MirroredMatch,
    /// A match served from cache.
    CachedMatch,
    /// A match known to be stale.
    StaleMatch,
}

impl M5DocsMatchState {
    /// Every match state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactMatch,
        Self::NearbyMatch,
        Self::ProjectSpecificMatch,
        Self::MirroredMatch,
        Self::CachedMatch,
        Self::StaleMatch,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactMatch => "exact_match",
            Self::NearbyMatch => "nearby_match",
            Self::ProjectSpecificMatch => "project_specific_match",
            Self::MirroredMatch => "mirrored_match",
            Self::CachedMatch => "cached_match",
            Self::StaleMatch => "stale_match",
        }
    }
}

/// Controlled project-doc override reason — why a project doc outranked vendor
/// docs, so a result row never silently reorders results without saying why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsOverrideReason {
    /// A project pin overrode the default source.
    ProjectPinnedOverride,
    /// Local freshness overrode a staler source.
    LocalFreshnessOverride,
    /// An explicit user preference overrode the default.
    ExplicitUserPreference,
    /// The vendor source was unavailable.
    VendorSourceUnavailable,
    /// A policy-scoped override applied.
    PolicyScopedOverride,
    /// No override applied (default ranking).
    NoOverride,
}

impl M5DocsOverrideReason {
    /// Every override reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProjectPinnedOverride,
        Self::LocalFreshnessOverride,
        Self::ExplicitUserPreference,
        Self::VendorSourceUnavailable,
        Self::PolicyScopedOverride,
        Self::NoOverride,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectPinnedOverride => "project_pinned_override",
            Self::LocalFreshnessOverride => "local_freshness_override",
            Self::ExplicitUserPreference => "explicit_user_preference",
            Self::VendorSourceUnavailable => "vendor_source_unavailable",
            Self::PolicyScopedOverride => "policy_scoped_override",
            Self::NoOverride => "no_override",
        }
    }
}

/// Controlled symbol anchor — what code entity a symbol-linked reference card
/// points at, so a reference card never shows an unresolved anchor as resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSymbolAnchor {
    /// A function symbol.
    FunctionSymbol,
    /// A type / struct / enum symbol.
    TypeSymbol,
    /// A module symbol.
    ModuleSymbol,
    /// A field or method member.
    FieldOrMethod,
    /// A macro symbol.
    MacroSymbol,
    /// An unresolved anchor.
    UnresolvedAnchor,
}

impl M5DocsSymbolAnchor {
    /// Every symbol anchor, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FunctionSymbol,
        Self::TypeSymbol,
        Self::ModuleSymbol,
        Self::FieldOrMethod,
        Self::MacroSymbol,
        Self::UnresolvedAnchor,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FunctionSymbol => "function_symbol",
            Self::TypeSymbol => "type_symbol",
            Self::ModuleSymbol => "module_symbol",
            Self::FieldOrMethod => "field_or_method",
            Self::MacroSymbol => "macro_symbol",
            Self::UnresolvedAnchor => "unresolved_anchor",
        }
    }
}

/// Controlled source provider — where a doc actually comes from, so a
/// source/version badge never masks a mirrored, third-party, or AI-derived origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSourceProvider {
    /// Bundled locally with the product.
    BundledLocal,
    /// Served from a mirrored registry.
    MirroredRegistry,
    /// First-party hosted documentation.
    FirstPartyHosted,
    /// Third-party hosted documentation.
    ThirdPartyHosted,
    /// An offline import.
    OfflineImport,
    /// AI-derived explanation.
    AiDerived,
}

impl M5DocsSourceProvider {
    /// Every source provider, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BundledLocal,
        Self::MirroredRegistry,
        Self::FirstPartyHosted,
        Self::ThirdPartyHosted,
        Self::OfflineImport,
        Self::AiDerived,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundledLocal => "bundled_local",
            Self::MirroredRegistry => "mirrored_registry",
            Self::FirstPartyHosted => "first_party_hosted",
            Self::ThirdPartyHosted => "third_party_hosted",
            Self::OfflineImport => "offline_import",
            Self::AiDerived => "ai_derived",
        }
    }
}

/// Controlled freshness state — how current a doc is, so a source/version badge or
/// result row never shows cached or expired documentation as live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsFreshnessState {
    /// Live and current.
    LiveCurrent,
    /// Recently synced.
    RecentlySynced,
    /// Cached / offline copy.
    CachedOffline,
    /// Stale / expired.
    StaleExpired,
    /// Freshness unknown.
    UnknownFreshness,
}

impl M5DocsFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveCurrent,
        Self::RecentlySynced,
        Self::CachedOffline,
        Self::StaleExpired,
        Self::UnknownFreshness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveCurrent => "live_current",
            Self::RecentlySynced => "recently_synced",
            Self::CachedOffline => "cached_offline",
            Self::StaleExpired => "stale_expired",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }
}

/// Controlled docs-pack state — the lifecycle posture of a docs pack, so a
/// docs-pack row never shows a quarantined or offline pack as freely trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsPackState {
    /// The pack is pinned to an exact version.
    PinnedPack,
    /// The pack is served from a mirror.
    MirroredPack,
    /// The pack is available offline only.
    OfflinePack,
    /// The pack is quarantined pending review.
    QuarantinedPack,
    /// An update to the pack is available.
    UpdateAvailable,
    /// The pack is unpinned and tracking upstream.
    UnpinnedTracking,
}

impl M5DocsPackState {
    /// Every docs-pack state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PinnedPack,
        Self::MirroredPack,
        Self::OfflinePack,
        Self::QuarantinedPack,
        Self::UpdateAvailable,
        Self::UnpinnedTracking,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedPack => "pinned_pack",
            Self::MirroredPack => "mirrored_pack",
            Self::OfflinePack => "offline_pack",
            Self::QuarantinedPack => "quarantined_pack",
            Self::UpdateAvailable => "update_available",
            Self::UnpinnedTracking => "unpinned_tracking",
        }
    }
}

/// Controlled stale-example status — the integrity of a documented example, so a
/// stale-example finding row never shows a drifted or broken example as current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsStaleExampleStatus {
    /// The example is current.
    ExampleCurrent,
    /// The API signature the example uses has drifted.
    ApiSignatureDrifted,
    /// The example uses a deprecated symbol.
    DeprecatedSymbolUsed,
    /// The example links to a broken target.
    BrokenLinkTarget,
    /// The example is bound to a mismatched version.
    VersionMismatchExample,
    /// The example is unverified.
    UnverifiedExample,
}

impl M5DocsStaleExampleStatus {
    /// Every stale-example status, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExampleCurrent,
        Self::ApiSignatureDrifted,
        Self::DeprecatedSymbolUsed,
        Self::BrokenLinkTarget,
        Self::VersionMismatchExample,
        Self::UnverifiedExample,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExampleCurrent => "example_current",
            Self::ApiSignatureDrifted => "api_signature_drifted",
            Self::DeprecatedSymbolUsed => "deprecated_symbol_used",
            Self::BrokenLinkTarget => "broken_link_target",
            Self::VersionMismatchExample => "version_mismatch_example",
            Self::UnverifiedExample => "unverified_example",
        }
    }
}

/// Controlled browser-handoff reason — why Aureline had to hand a docs task off to
/// a browser, so a handoff banner never leaves the reason implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffReason {
    /// No local corpus covers the request.
    NoLocalCorpus,
    /// The content is interactive and cannot render locally.
    InteractiveContentRequired,
    /// The source is auth-gated.
    AuthGatedSource,
    /// The content requires dynamic rendering.
    DynamicRenderingRequired,
    /// The browser is the external canonical source.
    ExternalCanonicalSource,
    /// The user explicitly requested a browser.
    UserRequestedBrowser,
}

impl M5DocsHandoffReason {
    /// Every handoff reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoLocalCorpus,
        Self::InteractiveContentRequired,
        Self::AuthGatedSource,
        Self::DynamicRenderingRequired,
        Self::ExternalCanonicalSource,
        Self::UserRequestedBrowser,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLocalCorpus => "no_local_corpus",
            Self::InteractiveContentRequired => "interactive_content_required",
            Self::AuthGatedSource => "auth_gated_source",
            Self::DynamicRenderingRequired => "dynamic_rendering_required",
            Self::ExternalCanonicalSource => "external_canonical_source",
            Self::UserRequestedBrowser => "user_requested_browser",
        }
    }
}

/// Claimed M5 docs surface family that renders / consumes a docs-browser component.
/// This is the docs analog of the shell-zone surface family: no component may
/// invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSurfaceFamily {
    /// The docs browser surface.
    DocsBrowser,
    /// The help center surface.
    HelpCenter,
    /// The onboarding surface.
    Onboarding,
    /// The AI-context surface.
    AiContext,
    /// The search-palette surface.
    SearchPalette,
    /// The hover-peek surface.
    HoverPeek,
    /// The support-desk surface.
    SupportDesk,
    /// The admin-review surface.
    AdminReview,
}

impl M5DocsSurfaceFamily {
    /// Every docs surface family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DocsBrowser,
        Self::HelpCenter,
        Self::Onboarding,
        Self::AiContext,
        Self::SearchPalette,
        Self::HoverPeek,
        Self::SupportDesk,
        Self::AdminReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::HelpCenter => "help_center",
            Self::Onboarding => "onboarding",
            Self::AiContext => "ai_context",
            Self::SearchPalette => "search_palette",
            Self::HoverPeek => "hover_peek",
            Self::SupportDesk => "support_desk",
            Self::AdminReview => "admin_review",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// corpus, source, or freshness never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5DocsDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Docs subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsConsumerSurface {
    /// The docs-browser UI.
    DocsBrowserUi,
    /// The Help / About surface.
    HelpAbout,
    /// The search palette.
    SearchPalette,
    /// The hover-peek surface.
    HoverPeek,
    /// The onboarding tour.
    OnboardingTour,
    /// The AI-context panel.
    AiContextPanel,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The admin console.
    AdminConsole,
    /// The general product UI.
    ProductUi,
}

impl M5DocsConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::DocsBrowserUi,
        Self::HelpAbout,
        Self::SearchPalette,
        Self::HoverPeek,
        Self::OnboardingTour,
        Self::AiContextPanel,
        Self::SupportExport,
        Self::CliInspect,
        Self::AdminConsole,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserUi => "docs_browser_ui",
            Self::HelpAbout => "help_about",
            Self::SearchPalette => "search_palette",
            Self::HoverPeek => "hover_peek",
            Self::OnboardingTour => "onboarding_tour",
            Self::AiContextPanel => "ai_context_panel",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::AdminConsole => "admin_console",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no docs truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5DocsAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed docs-browser component must be able to show. The first
/// three are hard requirements on every component; the remaining three close the
/// acceptance-criteria ambiguity about corpus class, source provider, and
/// freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsRequiredLabel {
    /// The component's stable identity / what docs object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The corpus class behind the component's content.
    CorpusClass,
    /// The source provider behind the component's content.
    SourceProvider,
    /// The freshness reading behind the component's content.
    Freshness,
}

impl M5DocsRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::CorpusClass,
        Self::SourceProvider,
        Self::Freshness,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::CorpusClass => "corpus_class",
            Self::SourceProvider => "source_provider",
            Self::Freshness => "freshness",
        }
    }
}

/// Qualification class for an M5 docs-browser-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5DocsQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a docs-browser component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsDowngradeTrigger {
    /// A search bar left its corpus class unstated.
    CorpusClassUnstated,
    /// A component masked its source provider.
    SourceProviderMasked,
    /// A scope switcher left the version scope unstated.
    VersionScopeUnstated,
    /// A symbol card hid an unresolved anchor.
    SymbolAnchorUnresolvedHidden,
    /// A result row hid the project-doc override reason.
    ProjectOverrideReasonHidden,
    /// A component hid its freshness reading.
    FreshnessHidden,
    /// A docs-pack row misrepresented its pin / mirror / offline / quarantine
    /// state.
    PackStateMisrepresented,
    /// A stale-example row showed a stale example as current.
    StaleExampleShownAsCurrent,
    /// A handoff banner left the handoff reason unstated.
    HandoffReasonUnstated,
    /// A component showed mirrored or cached content as live.
    MirroredOrCachedShownAsLive,
    /// A docs-pack row showed a quarantined pack as trusted.
    QuarantinedPackShownAsTrusted,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5DocsDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::CorpusClassUnstated,
        Self::SourceProviderMasked,
        Self::VersionScopeUnstated,
        Self::SymbolAnchorUnresolvedHidden,
        Self::ProjectOverrideReasonHidden,
        Self::FreshnessHidden,
        Self::PackStateMisrepresented,
        Self::StaleExampleShownAsCurrent,
        Self::HandoffReasonUnstated,
        Self::MirroredOrCachedShownAsLive,
        Self::QuarantinedPackShownAsTrusted,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusClassUnstated => "corpus_class_unstated",
            Self::SourceProviderMasked => "source_provider_masked",
            Self::VersionScopeUnstated => "version_scope_unstated",
            Self::SymbolAnchorUnresolvedHidden => "symbol_anchor_unresolved_hidden",
            Self::ProjectOverrideReasonHidden => "project_override_reason_hidden",
            Self::FreshnessHidden => "freshness_hidden",
            Self::PackStateMisrepresented => "pack_state_misrepresented",
            Self::StaleExampleShownAsCurrent => "stale_example_shown_as_current",
            Self::HandoffReasonUnstated => "handoff_reason_unstated",
            Self::MirroredOrCachedShownAsLive => "mirrored_or_cached_shown_as_live",
            Self::QuarantinedPackShownAsTrusted => "quarantined_pack_shown_as_trusted",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed docs-browser component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBrowserComponentRow {
    /// Governed component family.
    pub component_family: M5DocsBrowserComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5DocsQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 docs surface families that render / consume this component.
    pub surface_families: Vec<M5DocsSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5DocsDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5DocsRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5DocsRequiredLabel>,
    /// Corpus classes this component names (search-bar only).
    pub corpus_classes: Vec<M5DocsCorpusClass>,
    /// Version scopes this component names (scope-switcher only).
    pub version_scopes: Vec<M5DocsVersionScope>,
    /// Match states this component distinguishes (result-row only).
    pub match_states: Vec<M5DocsMatchState>,
    /// Project-doc override reasons this component discloses (result-row only).
    pub override_reasons: Vec<M5DocsOverrideReason>,
    /// Symbol anchors this component distinguishes (symbol-card only).
    pub symbol_anchors: Vec<M5DocsSymbolAnchor>,
    /// Source providers this component names (source-badge only).
    pub source_providers: Vec<M5DocsSourceProvider>,
    /// Freshness states this component distinguishes (source-badge only).
    pub freshness_states: Vec<M5DocsFreshnessState>,
    /// Docs-pack states this component distinguishes (pack-row only).
    pub pack_states: Vec<M5DocsPackState>,
    /// Stale-example statuses this component distinguishes (stale-example only).
    pub stale_example_statuses: Vec<M5DocsStaleExampleStatus>,
    /// Handoff reasons this component names (handoff-banner only).
    pub handoff_reasons: Vec<M5DocsHandoffReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5DocsAccessibilityRoute>,
    /// Docs subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5DocsConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5DocsDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks a corpus class or source
    /// provider provenance. MUST be `false`.
    pub masks_corpus_or_source_provenance: bool,
    /// Hard invariant: this component never shows stale or cached content as live
    /// / current. MUST be `false`.
    pub shows_stale_or_cached_as_live_current: bool,
    /// Hard invariant: this component never invents a private docs-status grammar.
    /// MUST be `false`.
    pub invents_private_docs_status_grammar: bool,
    /// Hard invariant: this component never hides a handoff reason or a project-doc
    /// override reason. MUST be `false`.
    pub hides_handoff_reason_or_override_reason: bool,
}

impl M5DocsBrowserComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5DocsRequiredLabel> = self.required_labels.iter().copied().collect();
        M5DocsRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_corpus_or_source_provenance
            && !self.shows_stale_or_cached_as_live_current
            && !self.invents_private_docs_status_grammar
            && !self.hides_handoff_reason_or_override_reason
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBrowserVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Corpus-class tokens.
    pub corpus_classes: Vec<String>,
    /// Version-scope tokens.
    pub version_scopes: Vec<String>,
    /// Match-state tokens.
    pub match_states: Vec<String>,
    /// Override-reason tokens.
    pub override_reasons: Vec<String>,
    /// Symbol-anchor tokens.
    pub symbol_anchors: Vec<String>,
    /// Source-provider tokens.
    pub source_providers: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Docs-pack-state tokens.
    pub pack_states: Vec<String>,
    /// Stale-example-status tokens.
    pub stale_example_statuses: Vec<String>,
    /// Handoff-reason tokens.
    pub handoff_reasons: Vec<String>,
    /// Docs-surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5DocsBrowserVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5DocsBrowserComponentFamily::ALL, |v| v.as_str()),
            corpus_classes: tokens(&M5DocsCorpusClass::ALL, |v| v.as_str()),
            version_scopes: tokens(&M5DocsVersionScope::ALL, |v| v.as_str()),
            match_states: tokens(&M5DocsMatchState::ALL, |v| v.as_str()),
            override_reasons: tokens(&M5DocsOverrideReason::ALL, |v| v.as_str()),
            symbol_anchors: tokens(&M5DocsSymbolAnchor::ALL, |v| v.as_str()),
            source_providers: tokens(&M5DocsSourceProvider::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5DocsFreshnessState::ALL, |v| v.as_str()),
            pack_states: tokens(&M5DocsPackState::ALL, |v| v.as_str()),
            stale_example_statuses: tokens(&M5DocsStaleExampleStatus::ALL, |v| v.as_str()),
            handoff_reasons: tokens(&M5DocsHandoffReason::ALL, |v| v.as_str()),
            surface_families: tokens(&M5DocsSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DocsDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5DocsConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5DocsAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5DocsRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5DocsBrowserGovernanceReview {
    /// The search bar shows its corpus classes and source providers.
    pub search_bar_shows_corpus_and_source: bool,
    /// The scope switcher shows its version scope.
    pub scope_switcher_shows_version_scope: bool,
    /// The result row shows its match state and project-doc override reason.
    pub result_row_shows_match_state_and_override_reason: bool,
    /// The symbol card shows its anchor and resolution.
    pub symbol_card_shows_anchor_and_resolution: bool,
    /// The source/version badge shows provider and freshness.
    pub source_badge_shows_provider_and_freshness: bool,
    /// The docs-pack row shows pin / mirror / offline / quarantine state.
    pub pack_row_shows_pin_mirror_offline_quarantine: bool,
    /// The stale-example row shows its stale-example status.
    pub stale_example_row_shows_stale_status: bool,
    /// The handoff banner shows its handoff reason.
    pub handoff_banner_shows_reason: bool,
    /// Live and cached / mirrored documentation are never conflated.
    pub live_versus_cached_never_conflated: bool,
    /// No component invents a second docs-status grammar.
    pub no_component_invents_second_status_grammar: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel docs-browser vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBrowserConsumerProjection {
    /// Search and result surfaces consume the shared corpus / match vocabulary.
    pub search_and_result_surfaces_consume_corpus_vocabulary: bool,
    /// Badge surfaces consume the source / freshness vocabulary.
    pub badge_surfaces_consume_source_and_freshness_vocabulary: bool,
    /// Docs-pack surfaces consume the pack-state vocabulary.
    pub pack_surfaces_consume_pack_state_vocabulary: bool,
    /// Handoff surfaces consume the handoff-reason vocabulary.
    pub handoff_surfaces_consume_handoff_reason_vocabulary: bool,
    /// Support / export reads a single canonical docs-browser source.
    pub support_export_reads_single_source: bool,
    /// Onboarding and AI surfaces read a single canonical docs-browser source.
    pub onboarding_and_ai_surfaces_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBrowserProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the docs-browser lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBrowserReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting docs-browser audit for the lane.
    pub docs_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DocsBrowserMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsBrowserMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5DocsBrowserComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsBrowserVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsBrowserGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsBrowserConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsBrowserProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsBrowserReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 docs-browser-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsBrowserMatrixPacket {
    /// Record kind; must equal [`M5_DOCS_BROWSER_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_BROWSER_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5DocsBrowserComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsBrowserVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsBrowserGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsBrowserConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsBrowserProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsBrowserReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsBrowserMatrixPacket {
    /// Builds an M5 docs-browser-component matrix packet from stable-lane input.
    pub fn new(input: M5DocsBrowserMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_BROWSER_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_BROWSER_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 docs-browser-component matrix invariants.
    pub fn validate(&self) -> Vec<M5DocsBrowserMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_BROWSER_MATRIX_RECORD_KIND {
            violations.push(M5DocsBrowserMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_BROWSER_MATRIX_SCHEMA_VERSION {
            violations.push(M5DocsBrowserMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsBrowserMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 docs browser matrix packet serializes"),
        ) {
            violations.push(M5DocsBrowserMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 docs browser matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Docs-Search-Bar, Docs-Scope-Switcher, Docs-Result-Row, Symbol-Linked-Reference-Card, Docs-Source-Version-Badge, Docs-Pack-Row, Stale-Example-Finding-Row, and Handoff-Banner Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Corpus classes: {}\n",
            self.vocabulary_set.corpus_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Handoff reasons: {}\n",
            self.vocabulary_set.handoff_reasons.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 docs-browser matrix export.
#[derive(Debug)]
pub enum M5DocsBrowserMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsBrowserMatrixViolation>),
}

impl fmt::Display for M5DocsBrowserMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 docs browser matrix export parse failed: {error}"
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
                    "m5 docs browser matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsBrowserMatrixArtifactError {}

/// Validation failures emitted by [`M5DocsBrowserMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsBrowserMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A search-bar component declares no corpus classes.
    CorpusClassMissing,
    /// A scope-switcher component declares no version scopes.
    VersionScopeMissing,
    /// A result-row component declares no match states.
    MatchStateMissing,
    /// A result-row component declares no override reasons.
    OverrideReasonMissing,
    /// A symbol-card component declares no symbol anchors.
    SymbolAnchorMissing,
    /// A source-badge component declares no source providers.
    SourceProviderMissing,
    /// A source-badge component declares no freshness states.
    FreshnessStateMissing,
    /// A pack-row component declares no pack states.
    PackStateMissing,
    /// A stale-example component declares no stale-example statuses.
    StaleExampleStatusMissing,
    /// A handoff-banner component declares no handoff reasons.
    HandoffReasonMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked corpus/source, stale-as-live,
    /// private status grammar, or hidden handoff/override reason).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DocsBrowserMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::CorpusClassMissing => "corpus_class_missing",
            Self::VersionScopeMissing => "version_scope_missing",
            Self::MatchStateMissing => "match_state_missing",
            Self::OverrideReasonMissing => "override_reason_missing",
            Self::SymbolAnchorMissing => "symbol_anchor_missing",
            Self::SourceProviderMissing => "source_provider_missing",
            Self::FreshnessStateMissing => "freshness_state_missing",
            Self::PackStateMissing => "pack_state_missing",
            Self::StaleExampleStatusMissing => "stale_example_status_missing",
            Self::HandoffReasonMissing => "handoff_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 docs-browser matrix export.
pub fn current_stable_m5_docs_browser_component_matrix_export(
) -> Result<M5DocsBrowserMatrixPacket, M5DocsBrowserMatrixArtifactError> {
    let packet: M5DocsBrowserMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/support_export.json"
    )))
    .map_err(M5DocsBrowserMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsBrowserMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DocsBrowserMatrixPacket,
    violations: &mut Vec<M5DocsBrowserMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_BROWSER_SCHEMA_REF,
        M5_DOCS_BROWSER_DOC_REF,
        M5_DOCS_BROWSER_SOURCE_RESULT_REF,
        M5_DOCS_BROWSER_SYMBOL_REF,
        M5_DOCS_BROWSER_PACK_REF,
        M5_DOCS_BROWSER_HANDOFF_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsBrowserMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DocsBrowserMatrixPacket,
    violations: &mut Vec<M5DocsBrowserMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DocsBrowserMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5DocsBrowserMatrixPacket,
    violations: &mut Vec<M5DocsBrowserMatrixViolation>,
) {
    let present: BTreeSet<M5DocsBrowserComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5DocsBrowserComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5DocsBrowserMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5DocsBrowserMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5DocsBrowserMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_search_bar() && row.corpus_classes.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::CorpusClassMissing);
        }
        if family.is_scope_switcher() && row.version_scopes.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::VersionScopeMissing);
        }
        if family.is_result_row() && row.match_states.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::MatchStateMissing);
        }
        if family.is_result_row() && row.override_reasons.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::OverrideReasonMissing);
        }
        if family.is_symbol_card() && row.symbol_anchors.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::SymbolAnchorMissing);
        }
        if family.is_source_badge() && row.source_providers.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::SourceProviderMissing);
        }
        if family.is_source_badge() && row.freshness_states.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::FreshnessStateMissing);
        }
        if family.is_pack_row() && row.pack_states.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::PackStateMissing);
        }
        if family.is_stale_example() && row.stale_example_statuses.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::StaleExampleStatusMissing);
        }
        if family.is_handoff_banner() && row.handoff_reasons.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::HandoffReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DocsBrowserMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DocsBrowserMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5DocsBrowserMatrixPacket,
    violations: &mut Vec<M5DocsBrowserMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.search_bar_shows_corpus_and_source,
        review.scope_switcher_shows_version_scope,
        review.result_row_shows_match_state_and_override_reason,
        review.symbol_card_shows_anchor_and_resolution,
        review.source_badge_shows_provider_and_freshness,
        review.pack_row_shows_pin_mirror_offline_quarantine,
        review.stale_example_row_shows_stale_status,
        review.handoff_banner_shows_reason,
        review.live_versus_cached_never_conflated,
        review.no_component_invents_second_status_grammar,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DocsBrowserMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsBrowserMatrixPacket,
    violations: &mut Vec<M5DocsBrowserMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.search_and_result_surfaces_consume_corpus_vocabulary,
        projection.badge_surfaces_consume_source_and_freshness_vocabulary,
        projection.pack_surfaces_consume_pack_state_vocabulary,
        projection.handoff_surfaces_consume_handoff_reason_vocabulary,
        projection.support_export_reads_single_source,
        projection.onboarding_and_ai_surfaces_read_single_source,
    ] {
        if !ok {
            violations.push(M5DocsBrowserMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsBrowserMatrixPacket,
    violations: &mut Vec<M5DocsBrowserMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsBrowserMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DocsBrowserMatrixPacket,
    violations: &mut Vec<M5DocsBrowserMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.docs_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DocsBrowserMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
