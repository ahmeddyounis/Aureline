//! The reusable M5 docs browser-handoff banner and the shared docs-browser
//! component consumers, projected the same way across every claimed M5 docs-browser,
//! onboarding-tour, glossary-card, AI-evidence-follow, and support/help surface a user
//! reaches when documentation context leaves Aureline's governed docs surface.
//!
//! Aureline's frozen docs-browser component matrix
//! ([`crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`])
//! names the browser-handoff banner as the last governed docs-browser component family
//! and freezes the controlled handoff-reason vocabulary alongside the corpus classes,
//! version scopes, source providers, freshness states, pack states, docs surface
//! families, deployment lines, consumer surfaces, accessibility routes, qualification
//! classes, and downgrade triggers. This module closes the B102 lane by *implementing*
//! that banner as a reusable primitive and *adopting* it — together with the already-built
//! search-bar, result-row, symbol-reference-card, source/version-badge, docs-pack-row, and
//! stale-example-finding-row primitives — across the shared docs-browser, onboarding,
//! glossary, AI-evidence, and support/help consumers, so a handoff banner always explains
//! its destination reason, its privacy consequence, its return path, and why Aureline
//! could not or should not satisfy the request in-product, and never flattens the
//! interaction into a raw URL jump that strips source/version/freshness/pack context.
//!
//! The module has two halves:
//!
//! 1. A handoff resolver — [`resolve_docs_handoff_banner`] — that takes one handoff's
//!    banner title, handoff reason, destination, source/version/freshness/pack context,
//!    declared privacy exposure, and governed return anchor, and produces one
//!    [`M5ResolvedDocsHandoffBanner`] carrying the derived in-product necessity (cannot
//!    serve in-product versus should defer to a canonical external source versus the user
//!    explicitly requested a browser), the derived privacy consequence (never understated
//!    below what actually leaves the boundary), the derived return-path posture (context
//!    preserved versus anchored), and the open/copy-return-anchor/stay-in-product/export
//!    actions — never a raw URL jump that strips source or version context.
//! 2. A consumer matrix — [`M5DocsHandoffConsumerPacket`] — that binds one row per claimed
//!    M5 handoff consumer (the docs browser, the onboarding tour, the glossary card, the
//!    AI-evidence follow link, and the support/help view) to the shared handoff-banner
//!    anatomy and, crucially, to the reused docs search-bar, result-row, reference-card,
//!    source/version-badge, docs-pack-row, and stale-example-finding-row components, so the
//!    docs-browser components stay consistent across help, onboarding, AI, and support
//!    rather than drifting by feature, and the return-path and privacy vocabulary survives
//!    export/support with the same words shown in-product.
//!
//! The handoff reason ([`M5DocsHandoffReason`]), corpus class ([`M5DocsCorpusClass`]),
//! version scope ([`M5DocsVersionScope`]), source provider ([`M5DocsSourceProvider`]),
//! freshness state ([`M5DocsFreshnessState`]), pack state ([`M5DocsPackState`]), docs
//! surface family ([`M5DocsSurfaceFamily`]), deployment line ([`M5DocsDeploymentLine`]),
//! consumer surface ([`M5DocsConsumerSurface`]), accessibility route
//! ([`M5DocsAccessibilityRoute`]), qualification class ([`M5DocsQualificationClass`]), and
//! downgrade trigger ([`M5DocsDowngradeTrigger`]) are reused verbatim from the frozen
//! docs-browser component matrix. This module mints new vocabulary only for what that
//! matrix left implicit about the handoff banner itself and its shared consumers: the
//! consumers, the in-product necessity, the privacy exposure and consequence, the
//! return-path posture, the actions, the anatomy parts, the export fields, and the shared
//! component identities. No M5 docs surface invents a second handoff grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, document bodies, and prompt text
//! stay outside the support boundary; every banner title, destination, return anchor, and
//! context representation is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_docs_handoff_consumer_ai_evidence_preview_narrowed,
    seeded_m5_docs_handoff_consumer_onboarding_tour_beta_narrowed,
    seeded_m5_docs_handoff_consumer_packet, M5_DOCS_HANDOFF_CONSUMER_PACKET_ID,
};

// The handoff reason, corpus class, version scope, source provider, freshness state, pack
// state, docs surface family, deployment line, consumer surface, accessibility routes,
// qualification classes, and downgrade triggers are frozen once, in the docs-browser
// component matrix. This lane reuses them verbatim so it never invents a parallel handoff
// or docs-component vocabulary.
pub use crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix::{
    M5DocsAccessibilityRoute, M5DocsConsumerSurface, M5DocsCorpusClass, M5DocsDeploymentLine,
    M5DocsDowngradeTrigger, M5DocsFreshnessState, M5DocsHandoffReason, M5DocsPackState,
    M5DocsQualificationClass, M5DocsSourceProvider, M5DocsSurfaceFamily, M5DocsVersionScope,
};

// The already-built docs-browser primitives whose canonical schema/doc refs the shared
// consumers cite, so the handoff lane proves the components are reused rather than
// re-invented per feature.
use crate::implement_docs_pack_rows_and_stale_example_finding_rows_with_pin_offline_refresh_quarantine_update_remove_actions_and_version_drift_truth::{
    M5_DOCS_PACK_FINDING_DOC_REF, M5_DOCS_PACK_FINDING_SCHEMA_REF,
};
use crate::implement_docs_result_rows_and_source_or_version_badges_with_result_kind_provider_version_scope_and_freshness_truth::{
    M5_DOCS_RESULT_ROW_DOC_REF, M5_DOCS_RESULT_ROW_SCHEMA_REF,
};
use crate::implement_docs_search_bars_and_scope_switchers_with_corpus_provider_and_cached_live_state_truth::{
    M5_DOCS_SEARCH_DOC_REF, M5_DOCS_SEARCH_SCHEMA_REF,
};
use crate::implement_docs_symbol_linked_reference_cards_with_code_anchor_and_exact_nearby_project_or_keyword_fallback_truth::{
    M5_DOCS_REFERENCE_CARD_DOC_REF, M5_DOCS_REFERENCE_CARD_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DocsHandoffConsumerPacket`].
pub const M5_DOCS_HANDOFF_CONSUMER_RECORD_KIND: &str =
    "add_m5_browser_handoff_banners_and_shared_docs_browser_onboarding_glossary_ai_and_support_consumers";

/// Schema version for M5 docs handoff-banner / shared-consumer records.
pub const M5_DOCS_HANDOFF_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the handoff-banner / shared-consumer boundary schema.
pub const M5_DOCS_HANDOFF_CONSUMER_SCHEMA_REF: &str =
    "schemas/docs/add-browser-handoff-banners-and-shared-docs-browser-onboarding-glossary-ai-and-support-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DOCS_HANDOFF_CONSUMER_DOC_REF: &str =
    "docs/docs/m5/add_browser_handoff_banners_and_shared_docs_browser_onboarding_glossary_ai_and_support_consumers.md";

/// Repo-relative path of the frozen docs-browser component matrix this lane narrows the
/// handoff-banner family from.
pub const M5_DOCS_HANDOFF_CONSUMER_COMPONENT_MATRIX_REF: &str =
    "schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json";

/// Repo-relative path of the browser-handoff-packet contract this lane keeps its
/// destination/return/privacy truth consistent with.
pub const M5_DOCS_HANDOFF_CONSUMER_HANDOFF_PACKET_REF: &str =
    "schemas/integration/browser_handoff_packet.schema.json";

/// Repo-relative path of the stable docs-source/result contract the shared components
/// bind against.
pub const M5_DOCS_HANDOFF_CONSUMER_SOURCE_RESULT_REF: &str =
    "schemas/docs/stable_docs_source_result_pack_and_citation.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_HANDOFF_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/docs/m5/m5-docs-handoff-banner-and-shared-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_HANDOFF_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/docs/m5/m5-docs-handoff-banner-and-shared-consumers/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DOCS_HANDOFF_CONSUMER_CSV_REF: &str =
    "artifacts/docs/m5/m5-docs-handoff-banner-and-shared-consumers/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DOCS_HANDOFF_CONSUMER_REPORT_REF: &str =
    "artifacts/docs/m5/m5-docs-handoff-banner-and-shared-consumers.md";

/// One claimed M5 handoff consumer that renders the shared browser-handoff banner and the
/// reused docs-browser components. These are the entrypoints the acceptance criteria name —
/// the docs browser, the onboarding tour, the glossary card, the AI-evidence follow link,
/// and the support/help view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffConsumerSurface {
    /// The docs browser surface.
    DocsBrowser,
    /// The onboarding tour step.
    OnboardingTour,
    /// The glossary card.
    GlossaryCard,
    /// The AI-evidence follow link.
    AiEvidenceFollow,
    /// The support / help view.
    SupportHelp,
}

impl M5DocsHandoffConsumerSurface {
    /// Every claimed handoff consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DocsBrowser,
        Self::OnboardingTour,
        Self::GlossaryCard,
        Self::AiEvidenceFollow,
        Self::SupportHelp,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::OnboardingTour => "onboarding_tour",
            Self::GlossaryCard => "glossary_card",
            Self::AiEvidenceFollow => "ai_evidence_follow",
            Self::SupportHelp => "support_help",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DocsBrowser => "Docs Browser",
            Self::OnboardingTour => "Onboarding Tour",
            Self::GlossaryCard => "Glossary Card",
            Self::AiEvidenceFollow => "AI-Evidence Follow Link",
            Self::SupportHelp => "Support / Help",
        }
    }
}

/// One shared docs-browser component the consumers reuse instead of re-inventing per
/// feature. Each carries the canonical schema/doc contract of the primitive that owns it,
/// so a consumer never grows its own parallel search bar, result row, badge, pack row, or
/// handoff banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsSharedComponent {
    /// The docs search bar + scope switcher primitive.
    SearchBar,
    /// The docs result row primitive.
    ResultRow,
    /// The symbol-linked reference card primitive.
    ReferenceCard,
    /// The docs source/version badge primitive.
    SourceVersionBadge,
    /// The docs-pack row primitive.
    PackRow,
    /// The stale-example finding row primitive.
    StaleExampleRow,
    /// The browser-handoff banner primitive (implemented in this lane).
    HandoffBanner,
}

impl M5DocsSharedComponent {
    /// Every shared component, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SearchBar,
        Self::ResultRow,
        Self::ReferenceCard,
        Self::SourceVersionBadge,
        Self::PackRow,
        Self::StaleExampleRow,
        Self::HandoffBanner,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchBar => "search_bar",
            Self::ResultRow => "result_row",
            Self::ReferenceCard => "reference_card",
            Self::SourceVersionBadge => "source_version_badge",
            Self::PackRow => "pack_row",
            Self::StaleExampleRow => "stale_example_row",
            Self::HandoffBanner => "handoff_banner",
        }
    }

    /// The canonical schema contract of the primitive that owns this component.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::SearchBar => M5_DOCS_SEARCH_SCHEMA_REF,
            Self::ResultRow | Self::SourceVersionBadge => M5_DOCS_RESULT_ROW_SCHEMA_REF,
            Self::ReferenceCard => M5_DOCS_REFERENCE_CARD_SCHEMA_REF,
            Self::PackRow | Self::StaleExampleRow => M5_DOCS_PACK_FINDING_SCHEMA_REF,
            Self::HandoffBanner => M5_DOCS_HANDOFF_CONSUMER_SCHEMA_REF,
        }
    }

    /// The canonical contract doc of the primitive that owns this component.
    pub const fn canonical_doc_ref(self) -> &'static str {
        match self {
            Self::SearchBar => M5_DOCS_SEARCH_DOC_REF,
            Self::ResultRow | Self::SourceVersionBadge => M5_DOCS_RESULT_ROW_DOC_REF,
            Self::ReferenceCard => M5_DOCS_REFERENCE_CARD_DOC_REF,
            Self::PackRow | Self::StaleExampleRow => M5_DOCS_PACK_FINDING_DOC_REF,
            Self::HandoffBanner => M5_DOCS_HANDOFF_CONSUMER_DOC_REF,
        }
    }
}

/// The derived in-product necessity — the resolver's honest verdict about why Aureline had
/// to hand the request to a browser: it cannot serve the content in-product, it should
/// defer to an external canonical source, or the user explicitly requested a browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffNecessity {
    /// Aureline cannot serve the content in-product (no local corpus, interactive-only, or
    /// dynamic rendering).
    CannotServeInProduct,
    /// Aureline should defer to an external canonical / auth-gated source.
    ShouldDeferToCanonical,
    /// The user explicitly requested a browser.
    UserRequestedExternal,
}

impl M5DocsHandoffNecessity {
    /// Every necessity, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::CannotServeInProduct,
        Self::ShouldDeferToCanonical,
        Self::UserRequestedExternal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CannotServeInProduct => "cannot_serve_in_product",
            Self::ShouldDeferToCanonical => "should_defer_to_canonical",
            Self::UserRequestedExternal => "user_requested_external",
        }
    }

    /// Review-safe phrase for the disclosure headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::CannotServeInProduct => "Aureline cannot render this content inside the product",
            Self::ShouldDeferToCanonical => {
                "Aureline is deferring to the external canonical source"
            }
            Self::UserRequestedExternal => "you asked to open this in a browser",
        }
    }

    /// True when an in-product alternative genuinely exists (so a stay-in-product action is
    /// honest).
    pub const fn in_product_alternative_exists(self) -> bool {
        !matches!(self, Self::CannotServeInProduct)
    }
}

/// The declared privacy exposure of a handoff — how much crosses Aureline's boundary when
/// the user follows the banner. This is the caller's honest declaration; the resolver
/// derives the user-facing consequence from it and never understates it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffPrivacyExposure {
    /// Nothing leaves the boundary (an already-bundled or local target).
    NoDataLeaves,
    /// Only anonymous query terms leave.
    AnonymousQueryLeaves,
    /// The document / query context leaves.
    DocumentContextLeaves,
    /// An identified request (tied to the user) leaves.
    IdentifiedRequestLeaves,
    /// An external account / sign-in is required.
    ExternalAccountRequired,
}

impl M5DocsHandoffPrivacyExposure {
    /// Every exposure, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoDataLeaves,
        Self::AnonymousQueryLeaves,
        Self::DocumentContextLeaves,
        Self::IdentifiedRequestLeaves,
        Self::ExternalAccountRequired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDataLeaves => "no_data_leaves",
            Self::AnonymousQueryLeaves => "anonymous_query_leaves",
            Self::DocumentContextLeaves => "document_context_leaves",
            Self::IdentifiedRequestLeaves => "identified_request_leaves",
            Self::ExternalAccountRequired => "external_account_required",
        }
    }
}

/// The derived privacy consequence — the resolver's user-facing verdict about what the
/// handoff exposes, never understated below the declared exposure and always escalated to
/// an identified request when the destination is auth-gated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffPrivacyConsequence {
    /// Nothing leaves — the interaction stays fully in-product.
    StaysFullyInProduct,
    /// Only an anonymous lookup leaves.
    AnonymousLookupOnly,
    /// The query / document context is shared with the destination.
    QueryContextShared,
    /// An identified request (tied to the user) is shared.
    IdentifiedRequestShared,
    /// An external account and the user's identity are shared.
    ExternalAccountAndIdentityShared,
}

impl M5DocsHandoffPrivacyConsequence {
    /// Every consequence, in declaration order (increasing leakage).
    pub const ALL: [Self; 5] = [
        Self::StaysFullyInProduct,
        Self::AnonymousLookupOnly,
        Self::QueryContextShared,
        Self::IdentifiedRequestShared,
        Self::ExternalAccountAndIdentityShared,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaysFullyInProduct => "stays_fully_in_product",
            Self::AnonymousLookupOnly => "anonymous_lookup_only",
            Self::QueryContextShared => "query_context_shared",
            Self::IdentifiedRequestShared => "identified_request_shared",
            Self::ExternalAccountAndIdentityShared => "external_account_and_identity_shared",
        }
    }

    /// Review-safe phrase for the disclosure headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::StaysFullyInProduct => "nothing leaves Aureline",
            Self::AnonymousLookupOnly => "only an anonymous lookup leaves Aureline",
            Self::QueryContextShared => "your query context is shared with the destination",
            Self::IdentifiedRequestShared => "an identified request is shared with the destination",
            Self::ExternalAccountAndIdentityShared => {
                "an external account and your identity are shared with the destination"
            }
        }
    }

    /// Leakage rank used for the honesty-first escalation ladder.
    const fn rank(self) -> u8 {
        match self {
            Self::StaysFullyInProduct => 0,
            Self::AnonymousLookupOnly => 1,
            Self::QueryContextShared => 2,
            Self::IdentifiedRequestShared => 3,
            Self::ExternalAccountAndIdentityShared => 4,
        }
    }

    /// True when the handoff actually crosses Aureline's boundary.
    pub const fn leaves_boundary(self) -> bool {
        !matches!(self, Self::StaysFullyInProduct)
    }
}

/// The derived return-path posture — whether following the banner and coming back keeps the
/// source/version context, or only returns to a governed anchor without round-tripping the
/// context. A handoff never collapses into a raw URL jump with no governed return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffReturnPathPosture {
    /// The return anchor round-trips the source/version context.
    ContextPreservedReturn,
    /// The return anchor returns to a governed docs anchor, but the source/version context
    /// is not round-tripped.
    AnchoredReturn,
    /// No governed return path (the resolver never yields this — a handoff always carries a
    /// return anchor).
    NoGovernedReturn,
}

impl M5DocsHandoffReturnPathPosture {
    /// Every return-path posture, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ContextPreservedReturn,
        Self::AnchoredReturn,
        Self::NoGovernedReturn,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContextPreservedReturn => "context_preserved_return",
            Self::AnchoredReturn => "anchored_return",
            Self::NoGovernedReturn => "no_governed_return",
        }
    }

    /// True when this posture round-trips the source/version context.
    pub const fn preserves_context(self) -> bool {
        matches!(self, Self::ContextPreservedReturn)
    }
}

/// One action a handoff banner exposes: open the destination in a browser, copy the
/// governed return anchor, stay in-product (when an alternative exists), or export the
/// handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffAction {
    /// Open the destination in a browser.
    OpenInBrowser,
    /// Copy the governed return anchor.
    CopyReturnAnchor,
    /// Stay in-product (only when an in-product alternative exists).
    StayInProduct,
    /// Export the handoff packet.
    ExportHandoffPacket,
}

impl M5DocsHandoffAction {
    /// Every handoff action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenInBrowser,
        Self::CopyReturnAnchor,
        Self::StayInProduct,
        Self::ExportHandoffPacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInBrowser => "open_in_browser",
            Self::CopyReturnAnchor => "copy_return_anchor",
            Self::StayInProduct => "stay_in_product",
            Self::ExportHandoffPacket => "export_handoff_packet",
        }
    }
}

/// One anatomy part the shared handoff banner surfaces. The parts in
/// [`M5DocsHandoffBannerAnatomyPart::MANDATORY`] are required on every banner so a user can
/// see the destination reason, the in-product blocker, the privacy consequence, the
/// source/version context, the return path, and the actions before leaving Aureline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffBannerAnatomyPart {
    /// The banner title label.
    BannerTitleLabel,
    /// The destination + handoff-reason badge.
    DestinationReasonBadge,
    /// The in-product blocker note (why Aureline could not / should not satisfy in-product).
    InProductBlockerNote,
    /// The privacy-consequence notice.
    PrivacyConsequenceNotice,
    /// The source/version/freshness context badge carried through the handoff.
    SourceVersionContextBadge,
    /// The return-path anchor.
    ReturnPathAnchor,
    /// The open/copy/stay/export action cluster.
    HandoffActionCluster,
}

impl M5DocsHandoffBannerAnatomyPart {
    /// Every banner anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::BannerTitleLabel,
        Self::DestinationReasonBadge,
        Self::InProductBlockerNote,
        Self::PrivacyConsequenceNotice,
        Self::SourceVersionContextBadge,
        Self::ReturnPathAnchor,
        Self::HandoffActionCluster,
    ];

    /// The anatomy parts every consumer must render.
    pub const MANDATORY: [Self; 6] = [
        Self::DestinationReasonBadge,
        Self::InProductBlockerNote,
        Self::PrivacyConsequenceNotice,
        Self::SourceVersionContextBadge,
        Self::ReturnPathAnchor,
        Self::HandoffActionCluster,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BannerTitleLabel => "banner_title_label",
            Self::DestinationReasonBadge => "destination_reason_badge",
            Self::InProductBlockerNote => "in_product_blocker_note",
            Self::PrivacyConsequenceNotice => "privacy_consequence_notice",
            Self::SourceVersionContextBadge => "source_version_context_badge",
            Self::ReturnPathAnchor => "return_path_anchor",
            Self::HandoffActionCluster => "handoff_action_cluster",
        }
    }
}

/// A field the support / export packet carries so handoff-banner identity is
/// reconstructable from the shared model. The fields in
/// [`M5DocsHandoffExportField::MANDATORY`] are required so the handoff reason, necessity,
/// privacy consequence, return anchor, and source/version/freshness context survive
/// export/support paths with the same vocabulary shown in-product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsHandoffExportField {
    /// The handoff reason.
    HandoffReason,
    /// The destination.
    Destination,
    /// The derived in-product necessity.
    InProductNecessity,
    /// The declared privacy exposure.
    PrivacyExposure,
    /// The derived privacy consequence.
    PrivacyConsequence,
    /// The governed return anchor.
    ReturnAnchor,
    /// The source provider carried through the handoff.
    SourceProvider,
    /// The version scope carried through the handoff.
    VersionScope,
    /// The freshness state carried through the handoff.
    FreshnessState,
    /// The pack state carried through the handoff.
    PackContext,
}

impl M5DocsHandoffExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::HandoffReason,
        Self::Destination,
        Self::InProductNecessity,
        Self::PrivacyExposure,
        Self::PrivacyConsequence,
        Self::ReturnAnchor,
        Self::SourceProvider,
        Self::VersionScope,
        Self::FreshnessState,
        Self::PackContext,
    ];

    /// The export fields every consumer must carry so identity survives export/support.
    pub const MANDATORY: [Self; 7] = [
        Self::HandoffReason,
        Self::InProductNecessity,
        Self::PrivacyConsequence,
        Self::ReturnAnchor,
        Self::SourceProvider,
        Self::VersionScope,
        Self::FreshnessState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandoffReason => "handoff_reason",
            Self::Destination => "destination",
            Self::InProductNecessity => "in_product_necessity",
            Self::PrivacyExposure => "privacy_exposure",
            Self::PrivacyConsequence => "privacy_consequence",
            Self::ReturnAnchor => "return_anchor",
            Self::SourceProvider => "source_provider",
            Self::VersionScope => "version_scope",
            Self::FreshnessState => "freshness_state",
            Self::PackContext => "pack_context",
        }
    }
}

// ---- handoff resolver ---------------------------------------------------

/// The full input to the handoff-banner resolver for one handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsHandoffBannerResolutionInput {
    /// The opaque, export-safe banner title. Must be non-empty.
    pub banner_title_repr: String,
    /// The handoff reason (why Aureline is handing off).
    pub handoff_reason: M5DocsHandoffReason,
    /// The opaque, export-safe destination label. Must be non-empty and must not be a raw
    /// URL.
    pub destination_repr: String,
    /// The corpus class carried through the handoff.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider carried through the handoff.
    pub source_provider: M5DocsSourceProvider,
    /// The version scope carried through the handoff.
    pub version_scope: M5DocsVersionScope,
    /// The freshness state carried through the handoff.
    pub freshness_state: M5DocsFreshnessState,
    /// The pack state carried through the handoff.
    pub pack_state: M5DocsPackState,
    /// The declared privacy exposure of the handoff.
    pub privacy_exposure: M5DocsHandoffPrivacyExposure,
    /// The opaque, export-safe governed return anchor. Must be non-empty so the handoff is
    /// never a raw URL jump with no return path.
    pub return_anchor_repr: String,
    /// The opaque, export-safe source context stamped on the return anchor. May be empty.
    pub return_context_source_repr: String,
    /// The opaque, export-safe version context stamped on the return anchor. May be empty.
    pub return_context_version_repr: String,
}

/// The resolved destination/return/privacy truth for one handoff banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDocsHandoffBanner {
    /// The opaque banner title.
    pub banner_title_repr: String,
    /// The handoff reason.
    pub handoff_reason: M5DocsHandoffReason,
    /// The opaque destination label.
    pub destination_repr: String,
    /// The corpus class carried through.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider carried through.
    pub source_provider: M5DocsSourceProvider,
    /// The version scope carried through.
    pub version_scope: M5DocsVersionScope,
    /// The freshness state carried through.
    pub freshness_state: M5DocsFreshnessState,
    /// The pack state carried through.
    pub pack_state: M5DocsPackState,
    /// The declared privacy exposure.
    pub privacy_exposure: M5DocsHandoffPrivacyExposure,
    /// The opaque governed return anchor.
    pub return_anchor_repr: String,
    /// The opaque source context stamped on the return anchor.
    pub return_context_source_repr: String,
    /// The opaque version context stamped on the return anchor.
    pub return_context_version_repr: String,
    /// The derived in-product necessity.
    pub necessity: M5DocsHandoffNecessity,
    /// The derived privacy consequence (never understated).
    pub privacy_consequence: M5DocsHandoffPrivacyConsequence,
    /// The derived return-path posture.
    pub return_path_posture: M5DocsHandoffReturnPathPosture,
    /// True when the boundary change is disclosed (destination + reason + necessity all
    /// stated).
    pub boundary_disclosed: bool,
    /// True when the handoff actually crosses Aureline's boundary.
    pub privacy_leaves_boundary: bool,
    /// True when the return anchor round-trips the source/version context.
    pub preserves_return_context: bool,
    /// The open/copy-return-anchor/stay-in-product/export actions the banner exposes.
    pub available_actions: Vec<M5DocsHandoffAction>,
    /// A deterministic, self-contained disclosure headline naming the destination, the
    /// necessity, the privacy consequence, the return path, and the source/version context.
    pub disclosure_headline: String,
}

/// Errors returned by the handoff-banner resolver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DocsHandoffResolutionError {
    /// The banner title was empty.
    EmptyBannerTitle,
    /// The destination was empty.
    EmptyDestination,
    /// The governed return anchor was empty (a handoff must never be a raw URL jump).
    MissingReturnPath,
    /// A representation carried forbidden material (a raw URL, token, or credential).
    ForbiddenHandoffMaterial,
}

impl M5DocsHandoffResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyBannerTitle => "empty_banner_title",
            Self::EmptyDestination => "empty_destination",
            Self::MissingReturnPath => "missing_return_path",
            Self::ForbiddenHandoffMaterial => "forbidden_handoff_material",
        }
    }
}

impl fmt::Display for M5DocsHandoffResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "handoff resolution error: {}", self.as_str())
    }
}

impl Error for M5DocsHandoffResolutionError {}

/// Resolves one handoff banner from its declared destination and privacy exposure.
///
/// The derived necessity keeps "why Aureline handed off" honest: a no-local-corpus,
/// interactive, or dynamic-rendering reason reads as cannot-serve-in-product; an auth-gated
/// or external-canonical reason reads as should-defer-to-canonical; and an explicit
/// browser request reads as user-requested. The derived privacy consequence is never
/// understated below the declared exposure and is escalated to at least an identified
/// request whenever the destination is auth-gated. The return-path posture round-trips the
/// source/version context when the caller stamps it on the return anchor, and the banner
/// always carries a governed return anchor so it never collapses into a raw URL jump.
pub fn resolve_docs_handoff_banner(
    input: &M5DocsHandoffBannerResolutionInput,
) -> Result<M5ResolvedDocsHandoffBanner, M5DocsHandoffResolutionError> {
    if input.banner_title_repr.trim().is_empty() {
        return Err(M5DocsHandoffResolutionError::EmptyBannerTitle);
    }
    if input.destination_repr.trim().is_empty() {
        return Err(M5DocsHandoffResolutionError::EmptyDestination);
    }
    if input.return_anchor_repr.trim().is_empty() {
        return Err(M5DocsHandoffResolutionError::MissingReturnPath);
    }
    if value_repr_is_forbidden(&input.banner_title_repr)
        || value_repr_is_forbidden(&input.destination_repr)
        || value_repr_is_forbidden(&input.return_anchor_repr)
        || value_repr_is_forbidden(&input.return_context_source_repr)
        || value_repr_is_forbidden(&input.return_context_version_repr)
    {
        return Err(M5DocsHandoffResolutionError::ForbiddenHandoffMaterial);
    }

    let necessity = derive_handoff_necessity(input.handoff_reason);
    let privacy_consequence =
        derive_privacy_consequence(input.privacy_exposure, input.handoff_reason);
    let preserves_return_context = !input.return_context_source_repr.trim().is_empty()
        && !input.return_context_version_repr.trim().is_empty();
    let return_path_posture = if preserves_return_context {
        M5DocsHandoffReturnPathPosture::ContextPreservedReturn
    } else {
        M5DocsHandoffReturnPathPosture::AnchoredReturn
    };
    let boundary_disclosed = true;
    let privacy_leaves_boundary = privacy_consequence.leaves_boundary();
    let available_actions = derive_handoff_actions(necessity);

    let disclosure_headline = format!(
        "This opens {} because {} ({}); {} — return via {} ({} context on a {} pack from {}, {})",
        input.destination_repr,
        necessity.phrase(),
        input.handoff_reason.as_str(),
        privacy_consequence.phrase(),
        input.return_anchor_repr,
        if preserves_return_context {
            "with preserved source/version"
        } else {
            "anchored"
        },
        input.pack_state.as_str(),
        input.source_provider.as_str(),
        input.version_scope.as_str(),
    );

    Ok(M5ResolvedDocsHandoffBanner {
        banner_title_repr: input.banner_title_repr.clone(),
        handoff_reason: input.handoff_reason,
        destination_repr: input.destination_repr.clone(),
        corpus_class: input.corpus_class,
        source_provider: input.source_provider,
        version_scope: input.version_scope,
        freshness_state: input.freshness_state,
        pack_state: input.pack_state,
        privacy_exposure: input.privacy_exposure,
        return_anchor_repr: input.return_anchor_repr.clone(),
        return_context_source_repr: input.return_context_source_repr.clone(),
        return_context_version_repr: input.return_context_version_repr.clone(),
        necessity,
        privacy_consequence,
        return_path_posture,
        boundary_disclosed,
        privacy_leaves_boundary,
        preserves_return_context,
        available_actions,
        disclosure_headline,
    })
}

/// Maps a handoff reason to the honest in-product necessity.
fn derive_handoff_necessity(reason: M5DocsHandoffReason) -> M5DocsHandoffNecessity {
    use M5DocsHandoffReason as Reason;
    match reason {
        Reason::NoLocalCorpus
        | Reason::InteractiveContentRequired
        | Reason::DynamicRenderingRequired => M5DocsHandoffNecessity::CannotServeInProduct,
        Reason::AuthGatedSource | Reason::ExternalCanonicalSource => {
            M5DocsHandoffNecessity::ShouldDeferToCanonical
        }
        Reason::UserRequestedBrowser => M5DocsHandoffNecessity::UserRequestedExternal,
    }
}

/// Maps a declared exposure to the user-facing privacy consequence, escalating to at least
/// an identified request when the destination is auth-gated so the consequence is never
/// understated.
fn derive_privacy_consequence(
    exposure: M5DocsHandoffPrivacyExposure,
    reason: M5DocsHandoffReason,
) -> M5DocsHandoffPrivacyConsequence {
    use M5DocsHandoffPrivacyConsequence as Consequence;
    use M5DocsHandoffPrivacyExposure as Exposure;

    let base = match exposure {
        Exposure::NoDataLeaves => Consequence::StaysFullyInProduct,
        Exposure::AnonymousQueryLeaves => Consequence::AnonymousLookupOnly,
        Exposure::DocumentContextLeaves => Consequence::QueryContextShared,
        Exposure::IdentifiedRequestLeaves => Consequence::IdentifiedRequestShared,
        Exposure::ExternalAccountRequired => Consequence::ExternalAccountAndIdentityShared,
    };
    if matches!(reason, M5DocsHandoffReason::AuthGatedSource)
        && base.rank() < Consequence::IdentifiedRequestShared.rank()
    {
        Consequence::IdentifiedRequestShared
    } else {
        base
    }
}

/// The open/copy/stay/export action set for a handoff, emitted in canonical
/// [`M5DocsHandoffAction::ALL`] order. Open-in-browser, copy-return-anchor, and export are
/// always available; stay-in-product appears only when an in-product alternative exists.
fn derive_handoff_actions(necessity: M5DocsHandoffNecessity) -> Vec<M5DocsHandoffAction> {
    use M5DocsHandoffAction as Action;

    let mut actions = Vec::new();
    for action in Action::ALL {
        let include = match action {
            Action::OpenInBrowser => true,
            Action::CopyReturnAnchor => true,
            Action::StayInProduct => necessity.in_product_alternative_exists(),
            Action::ExportHandoffPacket => true,
        };
        if include {
            actions.push(action);
        }
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked handoff resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsHandoffBannerResolutionCase {
    /// The resolver input.
    pub input: M5DocsHandoffBannerResolutionInput,
    /// The resolved truth. Must equal `resolve_docs_handoff_banner(&input)`.
    pub resolved: M5ResolvedDocsHandoffBanner,
}

impl M5DocsHandoffBannerResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DocsHandoffBannerResolutionInput) -> Self {
        let resolved =
            resolve_docs_handoff_banner(&input).expect("seed handoff resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_docs_handoff_banner(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the consumer matrix: one handoff consumer bound to the shared handoff-banner
/// anatomy, the reused docs-browser components, and the same handoff reasons, necessities,
/// privacy exposures/consequences, return-path postures, actions, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsHandoffConsumerRow {
    /// Handoff consumer family.
    pub consumer_surface: M5DocsHandoffConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5DocsQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 docs surface families that render / consume this consumer.
    pub surface_families: Vec<M5DocsSurfaceFamily>,
    /// Deployment lines this consumer keeps the same truth across.
    pub deployment_lines: Vec<M5DocsDeploymentLine>,
    /// The shared docs-browser components this consumer reuses (never re-invents).
    pub reused_components: Vec<M5DocsSharedComponent>,
    /// Handoff-banner anatomy parts this consumer renders (must include the mandatory
    /// parts).
    pub banner_anatomy_parts: Vec<M5DocsHandoffBannerAnatomyPart>,
    /// Handoff reasons this consumer distinguishes.
    pub handoff_reasons: Vec<M5DocsHandoffReason>,
    /// In-product necessities this consumer distinguishes.
    pub necessities: Vec<M5DocsHandoffNecessity>,
    /// Privacy exposures this consumer distinguishes.
    pub privacy_exposures: Vec<M5DocsHandoffPrivacyExposure>,
    /// Privacy consequences this consumer distinguishes.
    pub privacy_consequences: Vec<M5DocsHandoffPrivacyConsequence>,
    /// Return-path postures this consumer distinguishes.
    pub return_path_postures: Vec<M5DocsHandoffReturnPathPosture>,
    /// Handoff actions this consumer offers.
    pub handoff_actions: Vec<M5DocsHandoffAction>,
    /// Corpus classes these banners name.
    pub corpus_classes: Vec<M5DocsCorpusClass>,
    /// Source providers these banners name.
    pub source_providers: Vec<M5DocsSourceProvider>,
    /// Version scopes these banners name.
    pub version_scopes: Vec<M5DocsVersionScope>,
    /// Freshness states these banners disclose.
    pub freshness_states: Vec<M5DocsFreshnessState>,
    /// Pack states these banners carry through.
    pub pack_states: Vec<M5DocsPackState>,
    /// Export fields these banners carry (must include the mandatory fields).
    pub export_fields: Vec<M5DocsHandoffExportField>,
    /// Non-visual accessibility routes these banners offer.
    pub accessibility_routes: Vec<M5DocsAccessibilityRoute>,
    /// Docs subsystems that consume these banners' projection.
    pub consumer_surfaces: Vec<M5DocsConsumerSurface>,
    /// Downgrade triggers that apply to these banners.
    pub downgrade_triggers: Vec<M5DocsDowngradeTrigger>,
    /// Proof packet refs that keep these banners current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by these banners.
    pub source_contract_refs: Vec<String>,
    /// Worked handoff resolution cases proving the resolver on this consumer.
    pub handoff_examples: Vec<M5DocsHandoffBannerResolutionCase>,
    /// Hard invariant: this consumer never strips the source/version/freshness/pack context
    /// through the handoff. MUST be `false`.
    pub strips_source_version_context: bool,
    /// Hard invariant: this consumer never understates the privacy consequence below what
    /// actually leaves the boundary. MUST be `false`.
    pub understates_privacy_consequence: bool,
    /// Hard invariant: this consumer never flattens the handoff into a raw URL jump with no
    /// governed return. MUST be `false`.
    pub flattens_to_raw_url_jump: bool,
    /// Hard invariant: this consumer never invents a private handoff grammar. MUST be
    /// `false`.
    pub invents_private_handoff_grammar: bool,
}

impl M5DocsHandoffConsumerRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DocsHandoffBannerAnatomyPart> =
            self.banner_anatomy_parts.iter().copied().collect();
        M5DocsHandoffBannerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DocsHandoffExportField> =
            self.export_fields.iter().copied().collect();
        M5DocsHandoffExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.strips_source_version_context
            && !self.understates_privacy_consequence
            && !self.flattens_to_raw_url_jump
            && !self.invents_private_handoff_grammar
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsHandoffVocabularySet {
    /// Handoff-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Shared-component tokens.
    pub shared_components: Vec<String>,
    /// Banner anatomy-part tokens.
    pub banner_anatomy_parts: Vec<String>,
    /// In-product necessity tokens.
    pub necessities: Vec<String>,
    /// Privacy-exposure tokens.
    pub privacy_exposures: Vec<String>,
    /// Privacy-consequence tokens.
    pub privacy_consequences: Vec<String>,
    /// Return-path-posture tokens.
    pub return_path_postures: Vec<String>,
    /// Handoff-action tokens.
    pub handoff_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Handoff-reason tokens (reused from the frozen matrix).
    pub handoff_reasons: Vec<String>,
    /// Corpus-class tokens (reused from the frozen matrix).
    pub corpus_classes: Vec<String>,
    /// Version-scope tokens (reused from the frozen matrix).
    pub version_scopes: Vec<String>,
    /// Source-provider tokens (reused from the frozen matrix).
    pub source_providers: Vec<String>,
    /// Freshness-state tokens (reused from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Pack-state tokens (reused from the frozen matrix).
    pub pack_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5DocsHandoffVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5DocsHandoffConsumerSurface::ALL, |v| v.as_str()),
            shared_components: tokens(&M5DocsSharedComponent::ALL, |v| v.as_str()),
            banner_anatomy_parts: tokens(&M5DocsHandoffBannerAnatomyPart::ALL, |v| v.as_str()),
            necessities: tokens(&M5DocsHandoffNecessity::ALL, |v| v.as_str()),
            privacy_exposures: tokens(&M5DocsHandoffPrivacyExposure::ALL, |v| v.as_str()),
            privacy_consequences: tokens(&M5DocsHandoffPrivacyConsequence::ALL, |v| v.as_str()),
            return_path_postures: tokens(&M5DocsHandoffReturnPathPosture::ALL, |v| v.as_str()),
            handoff_actions: tokens(&M5DocsHandoffAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DocsHandoffExportField::ALL, |v| v.as_str()),
            handoff_reasons: tokens(&M5DocsHandoffReason::ALL, |v| v.as_str()),
            corpus_classes: tokens(&M5DocsCorpusClass::ALL, |v| v.as_str()),
            version_scopes: tokens(&M5DocsVersionScope::ALL, |v| v.as_str()),
            source_providers: tokens(&M5DocsSourceProvider::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5DocsFreshnessState::ALL, |v| v.as_str()),
            pack_states: tokens(&M5DocsPackState::ALL, |v| v.as_str()),
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
pub struct M5DocsHandoffGovernanceReview {
    /// One shared banner primitive carries handoff truth on every consumer.
    pub shared_banner_carries_truth: bool,
    /// The destination reason is always stated.
    pub destination_reason_always_stated: bool,
    /// The privacy consequence is always stated.
    pub privacy_consequence_always_stated: bool,
    /// A governed return path is always present.
    pub return_path_always_present: bool,
    /// The in-product blocker (why Aureline could not / should not satisfy) is always
    /// explained.
    pub in_product_blocker_always_explained: bool,
    /// The source/version/freshness/pack context is preserved through the handoff.
    pub source_version_context_preserved_through_handoff: bool,
    /// The docs-browser components stay consistent across consumers rather than drifting by
    /// feature.
    pub components_consistent_across_consumers: bool,
    /// No consumer invents a second handoff grammar.
    pub no_consumer_invents_second_handoff_grammar: bool,
    /// Every consumer declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The privacy consequence is never understated below what actually leaves the
    /// boundary.
    pub privacy_consequence_never_understated: bool,
    /// Later M5 rows cannot invent parallel handoff vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsHandoffConsumerProjection {
    /// Docs-browser, onboarding, glossary, AI-evidence, and support consumers all consume
    /// the shared banner.
    pub consumers_consume_shared_banner: bool,
    /// The privacy consequence reads a single canonical source.
    pub privacy_consequence_reads_single_source: bool,
    /// The return path reads a single canonical source.
    pub return_path_reads_single_source: bool,
    /// The shared-component reuse reads a single canonical source.
    pub component_reuse_reads_single_source: bool,
    /// Support / export reads a single canonical handoff source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsHandoffProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the handoff lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsHandoffReleasePosture {
    /// Ref of the supporting proof packet.
    pub proof_packet_ref: String,
    /// Ref of the supporting handoff audit.
    pub handoff_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DocsHandoffConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsHandoffConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Handoff consumer rows.
    pub consumer_rows: Vec<M5DocsHandoffConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsHandoffVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsHandoffGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsHandoffProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsHandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 docs handoff-banner / shared-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsHandoffConsumerPacket {
    /// Record kind; must equal [`M5_DOCS_HANDOFF_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_HANDOFF_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Handoff consumer rows.
    pub consumer_rows: Vec<M5DocsHandoffConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsHandoffVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsHandoffGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsHandoffConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsHandoffProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsHandoffReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsHandoffConsumerPacket {
    /// Builds an M5 handoff-consumer packet from stable-lane input.
    pub fn new(input: M5DocsHandoffConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_HANDOFF_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_HANDOFF_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 handoff-consumer invariants.
    pub fn validate(&self) -> Vec<M5DocsHandoffConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_HANDOFF_CONSUMER_RECORD_KIND {
            violations.push(M5DocsHandoffConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_HANDOFF_CONSUMER_SCHEMA_VERSION {
            violations.push(M5DocsHandoffConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsHandoffConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_boundary_clarity(self, &mut violations);
        validate_privacy_honesty(self, &mut violations);
        validate_return_path_parity(self, &mut violations);
        validate_component_reuse(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 handoff-consumer packet serializes"),
        ) {
            violations.push(M5DocsHandoffConsumerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 handoff-consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per handoff consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,reused_components,handoff_reasons,necessities,privacy_consequences,return_path_postures,handoff_actions,export_fields,handoff_examples\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.reused_components, |v| v.as_str()),
                join_tokens(&row.handoff_reasons, |v| v.as_str()),
                join_tokens(&row.necessities, |v| v.as_str()),
                join_tokens(&row.privacy_consequences, |v| v.as_str()),
                join_tokens(&row.return_path_postures, |v| v.as_str()),
                join_tokens(&row.handoff_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.handoff_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Docs Handoff Banner & Shared Docs-Browser Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Handoff consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Necessities: {}\n",
            self.vocabulary_set.necessities.join(", ")
        ));
        out.push_str(&format!(
            "- Privacy consequences: {}\n",
            self.vocabulary_set.privacy_consequences.join(", ")
        ));
        out.push_str(&format!(
            "- Shared components: {}\n",
            self.vocabulary_set.shared_components.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Handoff consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Reused components: {}\n",
                join_tokens(&row.reused_components, |v| v.as_str())
            ));
            out.push_str(&format!(
                "  - Worked handoff banners: {}\n",
                row.handoff_examples.len()
            ));
            for case in &row.handoff_examples {
                out.push_str(&format!(
                    "    - `{}` → {} / {} (return `{}`, context-preserved `{}`, leaves-boundary `{}`)\n",
                    case.resolved.destination_repr,
                    case.resolved.necessity.as_str(),
                    case.resolved.privacy_consequence.as_str(),
                    case.resolved.return_path_posture.as_str(),
                    case.resolved.preserves_return_context,
                    case.resolved.privacy_leaves_boundary,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 handoff-consumer export.
#[derive(Debug)]
pub enum M5DocsHandoffConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsHandoffConsumerViolation>),
}

impl fmt::Display for M5DocsHandoffConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 handoff-consumer export parse failed: {error}"
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
                    "m5 handoff-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsHandoffConsumerArtifactError {}

/// Validation failures emitted by [`M5DocsHandoffConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsHandoffConsumerViolation {
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
    /// A required handoff consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A handoff consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A row omits one of the mandatory banner anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked handoff cases.
    HandoffResolutionMissing,
    /// A worked case does not match a fresh resolve of its input.
    HandoffResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked handoff proves the boundary is disclosed with source/version context
    /// preserved.
    BoundaryClarityUnproven,
    /// A worked handoff understates its privacy consequence, or the matrix does not prove
    /// both a stays-in-product and a leaves-boundary handoff.
    PrivacyConsequenceHonestyUnproven,
    /// A worked handoff lacks a governed return anchor / copy / export action, or no
    /// context-preserved return is proven.
    ReturnPathParityUnproven,
    /// The shared docs-browser components are not all reused by at least two consumers, or a
    /// reused component's canonical schema is not cited.
    ComponentReuseUnproven,
    /// A handoff consumer row violates a hard invariant.
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

impl M5DocsHandoffConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::HandoffResolutionMissing => "handoff_resolution_missing",
            Self::HandoffResolutionDrift => "handoff_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::BoundaryClarityUnproven => "boundary_clarity_unproven",
            Self::PrivacyConsequenceHonestyUnproven => "privacy_consequence_honesty_unproven",
            Self::ReturnPathParityUnproven => "return_path_parity_unproven",
            Self::ComponentReuseUnproven => "component_reuse_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 handoff-consumer export.
pub fn current_stable_m5_docs_handoff_consumer_export(
) -> Result<M5DocsHandoffConsumerPacket, M5DocsHandoffConsumerArtifactError> {
    let packet: M5DocsHandoffConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-docs-handoff-banner-and-shared-consumers/support_export.json"
    )))
    .map_err(M5DocsHandoffConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsHandoffConsumerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_HANDOFF_CONSUMER_SCHEMA_REF,
        M5_DOCS_HANDOFF_CONSUMER_DOC_REF,
        M5_DOCS_HANDOFF_CONSUMER_COMPONENT_MATRIX_REF,
        M5_DOCS_HANDOFF_CONSUMER_HANDOFF_PACKET_REF,
        M5_DOCS_HANDOFF_CONSUMER_SOURCE_RESULT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsHandoffConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DocsHandoffConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let present: BTreeSet<M5DocsHandoffConsumerSurface> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5DocsHandoffConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5DocsHandoffConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.reused_components.is_empty()
            || row.banner_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.handoff_reasons.is_empty()
            || row.necessities.is_empty()
            || row.privacy_exposures.is_empty()
            || row.privacy_consequences.is_empty()
            || row.return_path_postures.is_empty()
            || row.handoff_actions.is_empty()
            || row.corpus_classes.is_empty()
            || row.source_providers.is_empty()
            || row.version_scopes.is_empty()
            || row.freshness_states.is_empty()
            || row.pack_states.is_empty()
        {
            violations.push(M5DocsHandoffConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DocsHandoffConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DocsHandoffConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5DocsAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5DocsHandoffConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DocsHandoffConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DocsHandoffConsumerViolation::DowngradeTriggersMissing);
        }
        if row.handoff_examples.is_empty() {
            violations.push(M5DocsHandoffConsumerViolation::HandoffResolutionMissing);
        }
        if row
            .handoff_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DocsHandoffConsumerViolation::HandoffResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DocsHandoffConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DocsHandoffConsumerViolation::RowInvariantViolated);
        }
    }
}

/// Every worked handoff must disclose the boundary change (destination + reason +
/// necessity) and some worked handoff must prove the source/version context is preserved on
/// return, so a user is never left guessing why the boundary changed and the handoff never
/// strips source/version context (AC1).
fn validate_boundary_clarity(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let all_disclosed = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.handoff_examples.iter())
        .all(|case| {
            case.resolved.boundary_disclosed && !case.resolved.destination_repr.trim().is_empty()
        });
    let context_preserved_proven = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.handoff_examples.iter())
        .any(|case| {
            case.resolved.preserves_return_context
                && case.resolved.return_path_posture.preserves_context()
        });
    if !(all_disclosed && context_preserved_proven) {
        violations.push(M5DocsHandoffConsumerViolation::BoundaryClarityUnproven);
    }
}

/// No worked handoff may understate its privacy consequence: a handoff that stays in-product
/// must not actually leave the boundary, and an auth-gated handoff must read as at least an
/// identified request. The matrix must also prove both a stays-in-product and a
/// leaves-boundary handoff so the honest contrast is always present (AC3).
fn validate_privacy_honesty(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let cases = || {
        packet
            .consumer_rows
            .iter()
            .flat_map(|row| row.handoff_examples.iter())
    };

    let no_understatement = cases().all(|case| {
        let stays_but_leaves = !case.resolved.privacy_consequence.leaves_boundary()
            && case.resolved.privacy_leaves_boundary;
        let auth_gated_understated = matches!(
            case.resolved.handoff_reason,
            M5DocsHandoffReason::AuthGatedSource
        ) && case.resolved.privacy_consequence.rank()
            < M5DocsHandoffPrivacyConsequence::IdentifiedRequestShared.rank();
        !stays_but_leaves && !auth_gated_understated
    });
    let has_stays_in_product = cases().any(|case| !case.resolved.privacy_leaves_boundary);
    let has_leaves_boundary = cases().any(|case| case.resolved.privacy_leaves_boundary);

    if !(no_understatement && has_stays_in_product && has_leaves_boundary) {
        violations.push(M5DocsHandoffConsumerViolation::PrivacyConsequenceHonestyUnproven);
    }
}

/// Every worked handoff must carry a governed return anchor plus the copy-return-anchor and
/// export-handoff-packet actions, and some worked handoff must prove a context-preserved
/// return, so the return path survives export/support and the handoff never flattens into a
/// raw URL jump (AC1/AC3).
fn validate_return_path_parity(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let all_have_return = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.handoff_examples.iter())
        .all(|case| {
            !case.resolved.return_anchor_repr.trim().is_empty()
                && case
                    .resolved
                    .available_actions
                    .contains(&M5DocsHandoffAction::CopyReturnAnchor)
                && case
                    .resolved
                    .available_actions
                    .contains(&M5DocsHandoffAction::ExportHandoffPacket)
        });
    let context_return_proven = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.handoff_examples.iter())
        .any(|case| case.resolved.return_path_posture.preserves_context());
    if !(all_have_return && context_return_proven) {
        violations.push(M5DocsHandoffConsumerViolation::ReturnPathParityUnproven);
    }
}

/// Every shared docs-browser component must be reused by at least two consumers, and each
/// reused component's canonical schema must be cited in the packet's source contracts, so
/// the components stay consistent across consumers rather than drifting by feature (AC2).
fn validate_component_reuse(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let cited: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for component in M5DocsSharedComponent::ALL {
        let reuse_count = packet
            .consumer_rows
            .iter()
            .filter(|row| row.reused_components.contains(&component))
            .count();
        if reuse_count < 2 || !cited.contains(component.canonical_schema_ref()) {
            violations.push(M5DocsHandoffConsumerViolation::ComponentReuseUnproven);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.shared_banner_carries_truth,
        review.destination_reason_always_stated,
        review.privacy_consequence_always_stated,
        review.return_path_always_present,
        review.in_product_blocker_always_explained,
        review.source_version_context_preserved_through_handoff,
        review.components_consistent_across_consumers,
        review.no_consumer_invents_second_handoff_grammar,
        review.every_row_declares_accessibility_route,
        review.privacy_consequence_never_understated,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DocsHandoffConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.consumers_consume_shared_banner,
        projection.privacy_consequence_reads_single_source,
        projection.return_path_reads_single_source,
        projection.component_reuse_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DocsHandoffConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsHandoffConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DocsHandoffConsumerPacket,
    violations: &mut Vec<M5DocsHandoffConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.handoff_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DocsHandoffConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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
