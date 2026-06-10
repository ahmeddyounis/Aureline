//! Docs search, symbol-linked reference cards, and code-anchor-preserving deep
//! links.
//!
//! This module ships the M5 docs-search depth lane as one export-safe truth
//! packet. A [`DocsSearchLinkPacket`] binds three surfaces that must stay
//! consistent:
//!
//! - **Docs search results** — ranked [`DocsSearchLinkResultRow`] hits, each
//!   carrying a source/version/freshness [`DocsSearchLinkChipSet`], an explicit
//!   ranking reason, and the open-raw / open-source escapes that keep derived
//!   results honest.
//! - **Symbol-linked reference cards** — [`DocsSearchLinkSymbolCard`] entries
//!   that bind a project subject (symbol, file, setting, command, …) to one or
//!   more citation anchors with a [`DocsSearchLinkResolutionClass`], its fallback
//!   chain, and a project-vs-vendor truth cue.
//! - **Code-anchor-preserving deep links** — [`DocsSearchLinkDeepLink`] entries
//!   whose [`DocsSearchLinkCodeAnchor`] (file ref, symbol ref, line anchor, and
//!   revision) survives export and any browser handoff while keeping a safe
//!   return path back to the IDE.
//!
//! [`DocsSearchLinkPacket::materialize`] computes the validation findings and the
//! promotion state (`stable`, `narrowed_below_stable`, or `blocks_stable`) from
//! the input, so an uncited symbol card, an unresolved link without a repair
//! hook, a deep link that drops its code anchor, or a vendor row that looks more
//! authoritative than proven automatically narrows or blocks before it reaches a
//! consumer surface. The packet is an inspectable, serde-serializable truth
//! packet: it carries no raw query text, no raw document or source bodies, no raw
//! provider payloads, and no credentials — only metadata, chip truth, resolution
//! classes, ranking reasons, and contract references.
//!
//! The boundary schema is
//! [`schemas/docs/ship-docs-search-symbol-linked-reference-cards-and-code-anchor-preserving-deep-links.schema.json`](../../../../schemas/docs/ship-docs-search-symbol-linked-reference-cards-and-code-anchor-preserving-deep-links.schema.json).
//! The contract doc is
//! [`docs/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links.md`](../../../../docs/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/`](../../../../fixtures/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DocsSearchLinkPacket`].
pub const DOCS_SEARCH_LINK_RECORD_KIND: &str =
    "docs_search_symbol_linked_reference_cards_and_code_anchor_deep_links";

/// Record-kind tag carried by the support-export wrapper.
pub const DOCS_SEARCH_LINK_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_search_symbol_linked_reference_cards_and_code_anchor_deep_links_support_export";

/// Schema version for docs-search symbol-link records.
pub const DOCS_SEARCH_LINK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_SEARCH_LINK_SCHEMA_REF: &str =
    "schemas/docs/ship-docs-search-symbol-linked-reference-cards-and-code-anchor-preserving-deep-links.schema.json";

/// Repo-relative path of the contract doc.
pub const DOCS_SEARCH_LINK_DOC_REF: &str =
    "docs/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_SEARCH_LINK_FIXTURE_DIR: &str =
    "fixtures/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_SEARCH_LINK_ARTIFACT_REF: &str =
    "artifacts/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_SEARCH_LINK_SUMMARY_REF: &str =
    "artifacts/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links.md";

/// Reuses the frozen symbol-linked-reference boundary contract.
pub const DOCS_SEARCH_LINK_SYMBOL_REFERENCE_CONTRACT_REF: &str =
    "schemas/docs/symbol_linked_reference.schema.json";

/// Reuses the frozen symbol-link validation corpus.
pub const DOCS_SEARCH_LINK_VALIDATION_MANIFEST_REF: &str =
    "fixtures/docs/symbol_link_validation_manifest.yaml";

/// Kind of a docs-search result row, projected as the result-kind chip.
///
/// Tokens match the frozen docs-browser `result_kind` vocabulary so every
/// surface keeps one set of result labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkResultKind {
    /// A docs page hit.
    DocsPageResult,
    /// A generated reference hit.
    GeneratedReferenceResult,
    /// A symbol-linked reference hit.
    SymbolReferenceResult,
    /// A support-runbook step hit.
    RunbookStepResult,
    /// A curated knowledge-pack hit.
    CuratedPackResult,
    /// A vendor/provider docs hit.
    VendorProviderResult,
    /// A derived-explanation hit.
    DerivedExplanationResult,
}

impl DocsSearchLinkResultKind {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPageResult => "docs_page_result",
            Self::GeneratedReferenceResult => "generated_reference_result",
            Self::SymbolReferenceResult => "symbol_reference_result",
            Self::RunbookStepResult => "runbook_step_result",
            Self::CuratedPackResult => "curated_pack_result",
            Self::VendorProviderResult => "vendor_provider_result",
            Self::DerivedExplanationResult => "derived_explanation_result",
        }
    }
}

/// Source class for a recalled docs node, projected as the source chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkSourceClass {
    /// Workspace-local project docs.
    ProjectDocs,
    /// Generated API/reference docs.
    GeneratedReference,
    /// Pinned, signed mirror of official upstream docs.
    MirroredOfficialDocs,
    /// Curated knowledge pack.
    CuratedKnowledgePack,
    /// Support runbook content.
    SupportRunbook,
    /// Vendor/provider docs surfaced inspect-only.
    VendorProviderDocs,
    /// Third-party extension docs pack.
    ExtensionDocsPack,
}

impl DocsSearchLinkSourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::SupportRunbook => "support_runbook",
            Self::VendorProviderDocs => "vendor_provider_docs",
            Self::ExtensionDocsPack => "extension_docs_pack",
        }
    }

    /// Whether this class is vendor/provider docs the product only inspects.
    pub const fn is_vendor_provider(self) -> bool {
        matches!(self, Self::VendorProviderDocs)
    }
}

/// Version-match state for a recalled source, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkVersionMatch {
    /// Source exactly matches the active build/workspace revision.
    ExactBuildMatch,
    /// Source is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Source drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release source has not completed verification.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl DocsSearchLinkVersionMatch {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }

    /// Whether this state may be presented as a confident current-version match.
    pub const fn is_confident_current(self) -> bool {
        matches!(self, Self::ExactBuildMatch)
    }
}

/// Freshness state for a recalled source, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkFreshness {
    /// Source was live and authoritative at recall time.
    AuthoritativeLive,
    /// Cached source within its freshness window.
    WarmCached,
    /// Cached source usable only with degraded disclosure.
    DegradedCached,
    /// Source is stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
}

impl DocsSearchLinkFreshness {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    /// Whether this state may claim live authoritative freshness.
    pub const fn is_authoritative_live(self) -> bool {
        matches!(self, Self::AuthoritativeLive)
    }
}

/// Kind of product subject a symbol-linked reference card binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkSubjectKind {
    /// A code symbol.
    Symbol,
    /// A file.
    File,
    /// A setting.
    Setting,
    /// A command.
    Command,
    /// A capability lifecycle id.
    CapabilityLifecycle,
    /// A keybinding.
    Keybinding,
    /// A runbook step.
    RunbookStep,
    /// A glossary term.
    GlossaryTerm,
    /// An onboarding step.
    OnboardingStep,
}

impl DocsSearchLinkSubjectKind {
    /// Stable token recorded in the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Symbol => "symbol",
            Self::File => "file",
            Self::Setting => "setting",
            Self::Command => "command",
            Self::CapabilityLifecycle => "capability_lifecycle",
            Self::Keybinding => "keybinding",
            Self::RunbookStep => "runbook_step",
            Self::GlossaryTerm => "glossary_term",
            Self::OnboardingStep => "onboarding_step",
        }
    }
}

/// Resolution class for a symbol-linked reference, re-exported from the frozen
/// symbol-link validation corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkResolutionClass {
    /// The exact symbol matched an authoritative entry.
    ExactSymbolMatch,
    /// A nearby-version entry was used inside the compat window.
    NearbyVersionFallback,
    /// A package-level guide was used when no symbol page exists.
    PackageLevelGuideFallback,
    /// Project docs outranked a vendor match.
    ProjectDocsOutranksVendorMatch,
    /// Vendor docs override project docs by disclosed policy.
    VendorOverridesProjectDisclosed,
    /// No entry resolved; a refresh is required.
    UnresolvedRequiresRefresh,
    /// No coverage exists yet; the request is routed to support.
    NoClaimYetSupportRouted,
}

impl DocsSearchLinkResolutionClass {
    /// Stable token recorded in the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSymbolMatch => "exact_symbol_match",
            Self::NearbyVersionFallback => "nearby_version_fallback",
            Self::PackageLevelGuideFallback => "package_level_guide_fallback",
            Self::ProjectDocsOutranksVendorMatch => "project_docs_outranks_vendor_match",
            Self::VendorOverridesProjectDisclosed => "vendor_overrides_project_disclosed",
            Self::UnresolvedRequiresRefresh => "unresolved_requires_refresh",
            Self::NoClaimYetSupportRouted => "no_claim_yet_support_routed",
        }
    }

    /// Whether this class resolved a cited entry (versus an unresolved state).
    pub const fn is_resolved(self) -> bool {
        !matches!(
            self,
            Self::UnresolvedRequiresRefresh | Self::NoClaimYetSupportRouted
        )
    }

    /// Whether the resolver traversed fallback rungs before settling here.
    pub const fn traversed_fallback(self) -> bool {
        !matches!(self, Self::ExactSymbolMatch)
    }
}

/// Project-vs-vendor authority cue for a symbol-linked reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkProjectVendorCue {
    /// Only the project pack claims authority.
    ProjectAuthoritativeOnly,
    /// The project outranks the vendor by default.
    ProjectOutranksVendorDefault,
    /// The vendor overrides the project by policy.
    VendorOverridesProjectByPolicy,
    /// The vendor overlay is inspect-only.
    VendorProviderOverlayInspectOnly,
    /// The project makes no claim; vendor docs are available.
    NoProjectClaimVendorAvailable,
}

impl DocsSearchLinkProjectVendorCue {
    /// Stable token recorded in the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectAuthoritativeOnly => "project_authoritative_only",
            Self::ProjectOutranksVendorDefault => "project_outranks_vendor_default",
            Self::VendorOverridesProjectByPolicy => "vendor_overrides_project_by_policy",
            Self::VendorProviderOverlayInspectOnly => "vendor_provider_overlay_inspect_only",
            Self::NoProjectClaimVendorAvailable => "no_project_claim_vendor_available",
        }
    }
}

/// Derived-explanation reuse state for a symbol-linked reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkReuseState {
    /// Reusable because at least one citation anchor backs the claim.
    ReusableWithCitationAnchor,
    /// Refused because the claim is uncited.
    RefusedUncited,
    /// Refused because the pack signature is unverified.
    RefusedSignatureUnverified,
    /// Refused because mirror continuity is broken.
    RefusedMirrorContinuityBroken,
    /// Refused because the vendor overlay requires higher trust.
    RefusedVendorOverlayRequiresHigherTrust,
    /// Refused because the source is an external status feed.
    RefusedExternalStatusFeed,
}

impl DocsSearchLinkReuseState {
    /// Stable token recorded in the card.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReusableWithCitationAnchor => "reusable_with_citation_anchor",
            Self::RefusedUncited => "refused_uncited",
            Self::RefusedSignatureUnverified => "refused_signature_unverified",
            Self::RefusedMirrorContinuityBroken => "refused_mirror_continuity_broken",
            Self::RefusedVendorOverlayRequiresHigherTrust => {
                "refused_vendor_overlay_requires_higher_trust"
            }
            Self::RefusedExternalStatusFeed => "refused_external_status_feed",
        }
    }

    /// Whether this state is a typed refusal that needs a repair hook.
    pub const fn is_refused(self) -> bool {
        !matches!(self, Self::ReusableWithCitationAnchor)
    }
}

/// Reason a reference or deep link resolves through an out-of-product browser
/// handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkBrowserHandoffReason {
    /// External docs or runbook.
    ExternalDocsOrRunbook,
    /// Provider consent flow.
    ProviderConsentFlow,
    /// License or portal acceptance.
    LicenseOrPortalAcceptance,
    /// Admin-only surface.
    AdminOnlySurface,
    /// Step-up authentication required.
    StepUpRequired,
    /// Mutation is not supported inside the product.
    MutationNotSupportedInProduct,
}

impl DocsSearchLinkBrowserHandoffReason {
    /// Stable token recorded in the reference/deep link.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExternalDocsOrRunbook => "external_docs_or_runbook",
            Self::ProviderConsentFlow => "provider_consent_flow",
            Self::LicenseOrPortalAcceptance => "license_or_portal_acceptance",
            Self::AdminOnlySurface => "admin_only_surface",
            Self::StepUpRequired => "step_up_required",
            Self::MutationNotSupportedInProduct => "mutation_not_supported_in_product",
        }
    }
}

/// Kind of repair hook offered when a reference or disclosure is degraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkRepairHookKind {
    /// Refresh the docs-pack freshness.
    RefreshFreshness,
    /// Migrate to a replacement reference after a rename or removal.
    MigrateToReplacement,
    /// Reconnect a provider.
    ReconnectProvider,
    /// Request an admin policy change.
    RequestAdminPolicyChange,
    /// Contact support.
    ContactSupport,
}

impl DocsSearchLinkRepairHookKind {
    /// Stable token recorded in the hook.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshFreshness => "refresh_freshness",
            Self::MigrateToReplacement => "migrate_to_replacement",
            Self::ReconnectProvider => "reconnect_provider",
            Self::RequestAdminPolicyChange => "request_admin_policy_change",
            Self::ContactSupport => "contact_support",
        }
    }
}

/// Kind of code anchor a deep link preserves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkAnchorKind {
    /// A symbol reference.
    SymbolRef,
    /// A file and line range.
    FileLineRange,
    /// A docs citation anchor.
    DocsAnchor,
    /// A setting id.
    SettingId,
    /// A command id.
    CommandId,
}

impl DocsSearchLinkAnchorKind {
    /// Stable token recorded in the deep link.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SymbolRef => "symbol_ref",
            Self::FileLineRange => "file_line_range",
            Self::DocsAnchor => "docs_anchor",
            Self::SettingId => "setting_id",
            Self::CommandId => "command_id",
        }
    }
}

/// Consumer surface that must project the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkConsumerSurface {
    /// Docs browser / reader.
    DocsBrowser,
    /// Search shell results.
    SearchShell,
    /// AI explanation overlay quoting a row.
    AiExplanationOverlay,
    /// Retrieval-debug inspector.
    RetrievalInspector,
    /// Glossary / reference card.
    GlossaryCard,
    /// Onboarding / guided tour step.
    Onboarding,
    /// Support / export packet.
    SupportExport,
    /// Help / About surface.
    HelpAbout,
}

impl DocsSearchLinkConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::SearchShell => "search_shell",
            Self::AiExplanationOverlay => "ai_explanation_overlay",
            Self::RetrievalInspector => "retrieval_inspector",
            Self::GlossaryCard => "glossary_card",
            Self::Onboarding => "onboarding",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Severity of a validation finding or resolution disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkFindingSeverity {
    /// Blocks a Stable claim; the packet must narrow.
    Blocking,
    /// Narrows below Stable but the packet stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl DocsSearchLinkFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Class of a resolution disclosure attached to a row, card, or deep link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkDisclosureClass {
    /// A nearer-version entry exists for the active build.
    NearbyVersionFallback,
    /// A package-level guide was used in place of a symbol page.
    PackageGuideFallback,
    /// Project docs outranked a vendor match.
    ProjectOutranksVendor,
    /// Vendor docs override project docs by disclosed policy.
    VendorOverridesProject,
    /// No entry resolved; a refresh is required.
    UnresolvedRequiresRefresh,
    /// No coverage exists yet; the request is routed to support.
    NoClaimSupportRouted,
    /// The deep link resolved to a non-exact (degraded) code anchor.
    DeepLinkAnchorDegraded,
    /// The reference was superseded by a later one after a rename or removal.
    SupersededReference,
}

impl DocsSearchLinkDisclosureClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NearbyVersionFallback => "nearby_version_fallback",
            Self::PackageGuideFallback => "package_guide_fallback",
            Self::ProjectOutranksVendor => "project_outranks_vendor",
            Self::VendorOverridesProject => "vendor_overrides_project",
            Self::UnresolvedRequiresRefresh => "unresolved_requires_refresh",
            Self::NoClaimSupportRouted => "no_claim_support_routed",
            Self::DeepLinkAnchorDegraded => "deep_link_anchor_degraded",
            Self::SupersededReference => "superseded_reference",
        }
    }

    /// Whether this disclosure class must carry a repair hook.
    pub const fn requires_repair_hook(self) -> bool {
        matches!(
            self,
            Self::UnresolvedRequiresRefresh
                | Self::NoClaimSupportRouted
                | Self::SupersededReference
        )
    }
}

/// Promotion state computed for the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkPromotionState {
    /// Packet qualifies for the Stable claim.
    Stable,
    /// Packet narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Packet has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl DocsSearchLinkPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`DocsSearchLinkPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSearchLinkFindingKind {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The search returned no rows.
    ResultRowsEmpty,
    /// Result ranks are not strictly increasing from 1.
    ResultRankNotMonotonic,
    /// A result id is duplicated.
    DuplicateResultId,
    /// A row is missing its explicit ranking reason.
    RankingReasonMissing,
    /// A row is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// A row links a symbol card id that is absent from the cards.
    SymbolCardOrphan,
    /// A row links a deep link id that is absent from the deep links.
    DeepLinkRefOrphan,
    /// A symbol card id is duplicated.
    DuplicateSymbolCardId,
    /// A deep link id is duplicated.
    DuplicateDeepLinkId,
    /// A resolved symbol card carries no citation anchor.
    SymbolCardUncited,
    /// A card's resolution fallback chain is inconsistent with its class.
    ResolutionFallbackInconsistent,
    /// A vendor-overlay card is missing its browser handoff / descriptor.
    VendorOverlayMissingHandoff,
    /// An unresolved card is missing its repair hook.
    UnresolvedMissingRepairHook,
    /// A refused-reuse card is missing its repair hook.
    RefusedReuseMissingRepairHook,
    /// A vendor row is presented as project-authoritative.
    ProjectVsVendorTruthCollapsed,
    /// A deep link does not preserve its code anchor across export.
    DeepLinkAnchorNotPreserved,
    /// A deep link is missing its safe return path.
    DeepLinkReturnPathMissing,
    /// A deep link's code anchor is incomplete.
    DeepLinkCodeAnchorIncomplete,
    /// A deep link with a browser handoff is missing its destination descriptor.
    DeepLinkHandoffMissingDescriptor,
    /// A disclosure is incomplete (missing summary, ref, or required repair hook).
    DisclosureIncomplete,
    /// A disclosure references an id absent from rows, cards, or deep links.
    DisclosureOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw query text, raw bodies, or secrets crossed the export boundary.
    RawBoundaryMaterialPresent,
}

impl DocsSearchLinkFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ResultRowsEmpty => "result_rows_empty",
            Self::ResultRankNotMonotonic => "result_rank_not_monotonic",
            Self::DuplicateResultId => "duplicate_result_id",
            Self::RankingReasonMissing => "ranking_reason_missing",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::SymbolCardOrphan => "symbol_card_orphan",
            Self::DeepLinkRefOrphan => "deep_link_ref_orphan",
            Self::DuplicateSymbolCardId => "duplicate_symbol_card_id",
            Self::DuplicateDeepLinkId => "duplicate_deep_link_id",
            Self::SymbolCardUncited => "symbol_card_uncited",
            Self::ResolutionFallbackInconsistent => "resolution_fallback_inconsistent",
            Self::VendorOverlayMissingHandoff => "vendor_overlay_missing_handoff",
            Self::UnresolvedMissingRepairHook => "unresolved_missing_repair_hook",
            Self::RefusedReuseMissingRepairHook => "refused_reuse_missing_repair_hook",
            Self::ProjectVsVendorTruthCollapsed => "project_vs_vendor_truth_collapsed",
            Self::DeepLinkAnchorNotPreserved => "deep_link_anchor_not_preserved",
            Self::DeepLinkReturnPathMissing => "deep_link_return_path_missing",
            Self::DeepLinkCodeAnchorIncomplete => "deep_link_code_anchor_incomplete",
            Self::DeepLinkHandoffMissingDescriptor => "deep_link_handoff_missing_descriptor",
            Self::DisclosureIncomplete => "disclosure_incomplete",
            Self::DisclosureOrphan => "disclosure_orphan",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::RequiredSurfaceCoverageMissing => "required_surface_coverage_missing",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
        }
    }

    /// Default severity for this finding kind; every validation finding blocks.
    pub const fn default_severity(self) -> DocsSearchLinkFindingSeverity {
        DocsSearchLinkFindingSeverity::Blocking
    }
}

/// The chip set rendered for one docs-search result row.
///
/// These three chips are the source/version/freshness truth a reader sees; every
/// consumer surface that sets `preserves_chips` must project them verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkChipSet {
    /// Source-class chip.
    pub source_class: DocsSearchLinkSourceClass,
    /// Version-match chip.
    pub version_match: DocsSearchLinkVersionMatch,
    /// Freshness chip.
    pub freshness: DocsSearchLinkFreshness,
}

/// A repair hook offered when a reference or disclosure is degraded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkRepairHook {
    /// Hook kind.
    pub hook_kind: DocsSearchLinkRepairHookKind,
    /// Opaque hook id.
    pub hook_id: String,
    /// Display label rendered on the repair affordance.
    pub display_label: String,
}

/// One ranked docs-search result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkResultRow {
    /// 1-based rank.
    pub rank: u32,
    /// Stable result id within this search.
    pub result_id: String,
    /// Result kind token.
    pub result_kind: DocsSearchLinkResultKind,
    /// User-visible title (no raw body).
    pub display_title: String,
    /// Docs-node ref (no raw body).
    pub doc_node_ref: String,
    /// Owning pack id.
    pub pack_id_ref: String,
    /// Owning pack revision ref.
    pub pack_revision_ref: String,
    /// Source/version/freshness chips.
    pub chips: DocsSearchLinkChipSet,
    /// Explicit, human-readable ranking reason.
    pub ranking_reason: String,
    /// Citation anchor refs backing the row.
    pub citation_anchor_refs: Vec<String>,
    /// Symbol-linked reference card this row binds to, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_card_id_ref: Option<String>,
    /// Code-anchor-preserving deep link this row opens, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep_link_id_ref: Option<String>,
    /// Open-raw escape ref (open the underlying node).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
}

/// A symbol-linked reference card binding a product subject to citation anchors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkSymbolCard {
    /// Stable card / reference id.
    pub card_id: String,
    /// Kind of product subject.
    pub subject_kind: DocsSearchLinkSubjectKind,
    /// Opaque subject ref (no raw source bytes / paths).
    pub subject_ref: String,
    /// Short human-legible label (no raw source bytes).
    pub display_label: String,
    /// Source class of the bound entry.
    pub source_class: DocsSearchLinkSourceClass,
    /// Owning pack id.
    pub pack_id_ref: String,
    /// Owning pack revision ref.
    pub pack_revision_ref: String,
    /// Resolution class.
    pub resolution_class: DocsSearchLinkResolutionClass,
    /// Ordered resolution fallback chain (empty for an exact match).
    pub resolution_fallback_chain: Vec<DocsSearchLinkResolutionClass>,
    /// Project-vs-vendor truth cue.
    pub project_vs_vendor_cue: DocsSearchLinkProjectVendorCue,
    /// Derived-explanation reuse state.
    pub reuse_state: DocsSearchLinkReuseState,
    /// Citation anchor refs backing the card.
    pub citation_anchor_refs: Vec<String>,
    /// Browser handoff reason, when the card resolves out of product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason: Option<DocsSearchLinkBrowserHandoffReason>,
    /// Destination descriptor ref backing a browser handoff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_descriptor_ref: Option<String>,
    /// Repair hook for unresolved / refused states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook: Option<DocsSearchLinkRepairHook>,
    /// Later reference that supersedes this one after a rename / removal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseded_by_ref: Option<String>,
}

/// The code anchor a deep link preserves across export and handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkCodeAnchor {
    /// Opaque file ref (no raw path bytes).
    pub file_ref: String,
    /// Opaque symbol ref, when the anchor binds a symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_ref: Option<String>,
    /// Opaque line-anchor ref, when the anchor binds a line range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_anchor_ref: Option<String>,
    /// Revision the anchor was minted against; preserves exactness across drift.
    pub revision_ref: String,
}

/// A code-anchor-preserving deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkDeepLink {
    /// Stable deep link id.
    pub deep_link_id: String,
    /// Kind of code anchor this link preserves.
    pub anchor_kind: DocsSearchLinkAnchorKind,
    /// Short human-legible label.
    pub display_label: String,
    /// Opaque subject ref the link targets.
    pub target_subject_ref: String,
    /// The preserved code anchor.
    pub code_anchor: DocsSearchLinkCodeAnchor,
    /// Whether the code anchor survives export verbatim.
    pub preserves_anchor_across_export: bool,
    /// Safe return path back to the IDE after any handoff.
    pub return_path_ref: String,
    /// Browser handoff reason, when the link resolves out of product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason: Option<DocsSearchLinkBrowserHandoffReason>,
    /// Destination descriptor ref backing a browser handoff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_descriptor_ref: Option<String>,
}

/// A resolution disclosure attached to a row, card, or deep link by id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkDisclosure {
    /// Disclosure class; kept distinct so states never collapse.
    pub disclosure_class: DocsSearchLinkDisclosureClass,
    /// Disclosure severity.
    pub severity: DocsSearchLinkFindingSeverity,
    /// The row / card / deep link this disclosure annotates.
    pub subject_id_ref: String,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// Repair hook for unresolved / superseded states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook: Option<DocsSearchLinkRepairHook>,
}

/// How a consumer surface projects the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkConsumerProjection {
    /// Surface that consumes the packet.
    pub surface: DocsSearchLinkConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves the symbol-link resolution (class + chain + cue).
    pub preserves_symbol_resolution: bool,
    /// Whether the surface preserves the citation anchors.
    pub preserves_citation_anchors: bool,
    /// Whether the surface preserves the code-anchor deep links.
    pub preserves_code_anchor_deep_links: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
}

impl DocsSearchLinkConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_symbol_resolution
            && self.preserves_citation_anchors
            && self.preserves_code_anchor_deep_links
            && self.preserves_open_raw_open_source_escape
    }
}

/// Constructor input for [`DocsSearchLinkPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable search label (never raw query text).
    pub search_label: String,
    /// Opaque digest/ref for the originating query (never raw query text).
    pub query_digest_ref: String,
    /// Ranked search result rows.
    pub search_results: Vec<DocsSearchLinkResultRow>,
    /// Symbol-linked reference cards.
    pub symbol_cards: Vec<DocsSearchLinkSymbolCard>,
    /// Code-anchor-preserving deep links.
    pub code_anchor_deep_links: Vec<DocsSearchLinkDeepLink>,
    /// Resolution disclosures attached by id.
    pub resolution_disclosures: Vec<DocsSearchLinkDisclosure>,
    /// Consumer projections.
    pub consumer_projections: Vec<DocsSearchLinkConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// A single validation finding on the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkValidationFinding {
    /// Finding kind.
    pub finding_kind: DocsSearchLinkFindingKind,
    /// Finding severity.
    pub severity: DocsSearchLinkFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Export-safe docs-search symbol-link packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkPacket {
    /// Record kind; must equal [`DOCS_SEARCH_LINK_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_SEARCH_LINK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable search label.
    pub search_label: String,
    /// Opaque digest/ref for the originating query.
    pub query_digest_ref: String,
    /// Ranked search result rows.
    pub search_results: Vec<DocsSearchLinkResultRow>,
    /// Symbol-linked reference cards.
    pub symbol_cards: Vec<DocsSearchLinkSymbolCard>,
    /// Code-anchor-preserving deep links.
    pub code_anchor_deep_links: Vec<DocsSearchLinkDeepLink>,
    /// Resolution disclosures.
    pub resolution_disclosures: Vec<DocsSearchLinkDisclosure>,
    /// Consumer projections.
    pub consumer_projections: Vec<DocsSearchLinkConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: DocsSearchLinkPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<DocsSearchLinkValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every packet must project.
const REQUIRED_SURFACES: [DocsSearchLinkConsumerSurface; 4] = [
    DocsSearchLinkConsumerSurface::DocsBrowser,
    DocsSearchLinkConsumerSurface::SearchShell,
    DocsSearchLinkConsumerSurface::RetrievalInspector,
    DocsSearchLinkConsumerSurface::SupportExport,
];

impl DocsSearchLinkPacket {
    /// Materializes a packet, computing validation findings and the promotion
    /// state from the input.
    pub fn materialize(input: DocsSearchLinkPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_search_results(&input, &mut findings);
        check_symbol_cards(&input, &mut findings);
        check_deep_links(&input, &mut findings);
        check_disclosures(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.resolution_disclosures);

        Self {
            record_kind: DOCS_SEARCH_LINK_RECORD_KIND.to_owned(),
            schema_version: DOCS_SEARCH_LINK_SCHEMA_VERSION,
            packet_id: input.packet_id,
            search_label: input.search_label,
            query_digest_ref: input.query_digest_ref,
            search_results: input.search_results,
            symbol_cards: input.symbol_cards,
            code_anchor_deep_links: input.code_anchor_deep_links,
            resolution_disclosures: input.resolution_disclosures,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the packet qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == DocsSearchLinkPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> DocsSearchLinkSupportExport {
        DocsSearchLinkSupportExport {
            record_kind: DOCS_SEARCH_LINK_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_SEARCH_LINK_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: DOCS_SEARCH_LINK_SCHEMA_REF.to_owned(),
            doc_ref: DOCS_SEARCH_LINK_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs search link packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Docs Search, Symbol Reference Cards, and Code-Anchor Deep Links\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Search: {}\n", self.search_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Rows: {} | Symbol cards: {} | Deep links: {} | Disclosures: {}\n",
            self.search_results.len(),
            self.symbol_cards.len(),
            self.code_anchor_deep_links.len(),
            self.resolution_disclosures.len(),
        ));
        out.push_str("\n## Results\n\n");
        for row in &self.search_results {
            out.push_str(&format!(
                "{}. `{}` [{}] — {} / {} / {}\n",
                row.rank,
                row.result_id,
                row.result_kind.as_str(),
                row.chips.source_class.as_str(),
                row.chips.version_match.as_str(),
                row.chips.freshness.as_str(),
            ));
            out.push_str(&format!("   - Reason: {}\n", row.ranking_reason));
        }
        if !self.symbol_cards.is_empty() {
            out.push_str("\n## Symbol reference cards\n\n");
            for card in &self.symbol_cards {
                out.push_str(&format!(
                    "- `{}` [{}/{}]: {} ({})\n",
                    card.card_id,
                    card.subject_kind.as_str(),
                    card.resolution_class.as_str(),
                    card.display_label,
                    card.project_vs_vendor_cue.as_str(),
                ));
            }
        }
        if !self.code_anchor_deep_links.is_empty() {
            out.push_str("\n## Code-anchor deep links\n\n");
            for link in &self.code_anchor_deep_links {
                out.push_str(&format!(
                    "- `{}` [{}]: {} (anchor preserved: {})\n",
                    link.deep_link_id,
                    link.anchor_kind.as_str(),
                    link.display_label,
                    link.preserves_anchor_across_export,
                ));
            }
        }
        if !self.resolution_disclosures.is_empty() {
            out.push_str("\n## Resolution disclosures\n\n");
            for disclosure in &self.resolution_disclosures {
                out.push_str(&format!(
                    "- `{}` [{}/{}]: {}\n",
                    disclosure.subject_id_ref,
                    disclosure.disclosure_class.as_str(),
                    disclosure.severity.as_str(),
                    disclosure.summary,
                ));
            }
        }
        out
    }
}

/// Support-export envelope for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSearchLinkSupportExport {
    /// Record kind; must equal [`DOCS_SEARCH_LINK_SUPPORT_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Contract doc ref.
    pub doc_ref: String,
    /// The wrapped packet.
    pub packet: DocsSearchLinkPacket,
}

/// Errors emitted when reading the checked-in support export.
#[derive(Debug)]
pub enum DocsSearchLinkArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: DocsSearchLinkPromotionState,
        /// Promotion state computed by re-materialization.
        computed: DocsSearchLinkPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<DocsSearchLinkValidationFinding>),
}

impl fmt::Display for DocsSearchLinkArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "docs search link export parse failed: {error}")
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "docs search link promotion drift: recorded {} but computed {}",
                recorded.as_str(),
                computed.as_str()
            ),
            Self::NotCleanStable(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs search link export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsSearchLinkArtifactError {}

/// Reads and re-validates the checked-in stable support export.
pub fn current_stable_docs_search_link_export(
) -> Result<DocsSearchLinkSupportExport, DocsSearchLinkArtifactError> {
    let export: DocsSearchLinkSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/support_export.json"
    )))
    .map_err(DocsSearchLinkArtifactError::SupportExport)?;

    // Re-materialize from the recorded packet's fields and confirm the recorded
    // promotion state and findings agree with a fresh computation.
    let recomputed = DocsSearchLinkPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(DocsSearchLinkArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(DocsSearchLinkArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &DocsSearchLinkPacket) -> DocsSearchLinkPacketInput {
    DocsSearchLinkPacketInput {
        packet_id: packet.packet_id.clone(),
        search_label: packet.search_label.clone(),
        query_digest_ref: packet.query_digest_ref.clone(),
        search_results: packet.search_results.clone(),
        symbol_cards: packet.symbol_cards.clone(),
        code_anchor_deep_links: packet.code_anchor_deep_links.clone(),
        resolution_disclosures: packet.resolution_disclosures.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<DocsSearchLinkValidationFinding>,
    kind: DocsSearchLinkFindingKind,
    summary: impl Into<String>,
) {
    findings.push(DocsSearchLinkValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &DocsSearchLinkPacketInput,
    findings: &mut Vec<DocsSearchLinkValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.search_label.trim().is_empty()
        || input.query_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            DocsSearchLinkFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_search_results(
    input: &DocsSearchLinkPacketInput,
    findings: &mut Vec<DocsSearchLinkValidationFinding>,
) {
    if input.search_results.is_empty() {
        push_finding(
            findings,
            DocsSearchLinkFindingKind::ResultRowsEmpty,
            "search returned no rows",
        );
        return;
    }

    let card_ids: BTreeSet<&str> = input
        .symbol_cards
        .iter()
        .map(|card| card.card_id.as_str())
        .collect();
    let deep_link_ids: BTreeSet<&str> = input
        .code_anchor_deep_links
        .iter()
        .map(|link| link.deep_link_id.as_str())
        .collect();

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for (index, row) in input.search_results.iter().enumerate() {
        let expected_rank = (index as u32) + 1;
        if row.rank != expected_rank {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::ResultRankNotMonotonic,
                format!(
                    "row `{}` has rank {} but expected {}",
                    row.result_id, row.rank, expected_rank
                ),
            );
        }
        if !seen_ids.insert(row.result_id.as_str()) {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DuplicateResultId,
                format!("duplicate result id `{}`", row.result_id),
            );
        }
        if row.ranking_reason.trim().is_empty() {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::RankingReasonMissing,
                format!("row `{}` is missing a ranking reason", row.result_id),
            );
        }
        if row.open_raw_escape_ref.trim().is_empty() || row.open_source_escape_ref.trim().is_empty()
        {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::OpenRawOpenSourceEscapeMissing,
                format!(
                    "row `{}` must keep open-raw and open-source escapes",
                    row.result_id
                ),
            );
        }
        if let Some(card_ref) = row.symbol_card_id_ref.as_deref() {
            if !card_ids.contains(card_ref) {
                push_finding(
                    findings,
                    DocsSearchLinkFindingKind::SymbolCardOrphan,
                    format!(
                        "row `{}` links unknown symbol card `{}`",
                        row.result_id, card_ref
                    ),
                );
            }
        }
        if let Some(link_ref) = row.deep_link_id_ref.as_deref() {
            if !deep_link_ids.contains(link_ref) {
                push_finding(
                    findings,
                    DocsSearchLinkFindingKind::DeepLinkRefOrphan,
                    format!(
                        "row `{}` links unknown deep link `{}`",
                        row.result_id, link_ref
                    ),
                );
            }
        }
    }
}

fn check_symbol_cards(
    input: &DocsSearchLinkPacketInput,
    findings: &mut Vec<DocsSearchLinkValidationFinding>,
) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for card in &input.symbol_cards {
        if !seen_ids.insert(card.card_id.as_str()) {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DuplicateSymbolCardId,
                format!("duplicate symbol card id `{}`", card.card_id),
            );
        }

        // Fallback chain must be empty for an exact match and otherwise end with
        // the settled resolution class.
        let chain = &card.resolution_fallback_chain;
        let chain_ok = if card.resolution_class.traversed_fallback() {
            chain.last() == Some(&card.resolution_class) && chain.len() >= 2
        } else {
            chain.is_empty()
        };
        if !chain_ok {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::ResolutionFallbackInconsistent,
                format!(
                    "card `{}` resolution `{}` has an inconsistent fallback chain",
                    card.card_id,
                    card.resolution_class.as_str()
                ),
            );
        }

        // Citation / repair-hook obligations.
        if card.resolution_class.is_resolved() {
            if card.citation_anchor_refs.is_empty() {
                push_finding(
                    findings,
                    DocsSearchLinkFindingKind::SymbolCardUncited,
                    format!(
                        "resolved card `{}` carries no citation anchor",
                        card.card_id
                    ),
                );
            }
        } else if card.repair_hook.is_none() {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::UnresolvedMissingRepairHook,
                format!(
                    "unresolved card `{}` must carry a repair hook",
                    card.card_id
                ),
            );
        }

        // A typed refusal must offer a repair hook rather than silently degrade.
        if card.reuse_state.is_refused() && card.repair_hook.is_none() {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::RefusedReuseMissingRepairHook,
                format!(
                    "refused card `{}` ({}) must carry a repair hook",
                    card.card_id,
                    card.reuse_state.as_str()
                ),
            );
        }

        // Vendor-overlay / override cards must declare their browser handoff.
        let needs_handoff = card.resolution_class
            == DocsSearchLinkResolutionClass::VendorOverridesProjectDisclosed
            || card.project_vs_vendor_cue
                == DocsSearchLinkProjectVendorCue::VendorProviderOverlayInspectOnly;
        if needs_handoff
            && (card.browser_handoff_reason.is_none()
                || card
                    .destination_descriptor_ref
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    .unwrap_or(true))
        {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::VendorOverlayMissingHandoff,
                format!(
                    "vendor-overlay card `{}` must carry a browser handoff and destination descriptor",
                    card.card_id
                ),
            );
        }

        // A vendor source must not be presented as project-authoritative.
        let project_authoritative = matches!(
            card.project_vs_vendor_cue,
            DocsSearchLinkProjectVendorCue::ProjectAuthoritativeOnly
                | DocsSearchLinkProjectVendorCue::ProjectOutranksVendorDefault
        );
        let vendor_resolution = matches!(
            card.resolution_class,
            DocsSearchLinkResolutionClass::VendorOverridesProjectDisclosed
        );
        if (card.source_class.is_vendor_provider() && project_authoritative)
            || (vendor_resolution
                && card.project_vs_vendor_cue
                    != DocsSearchLinkProjectVendorCue::VendorOverridesProjectByPolicy)
        {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::ProjectVsVendorTruthCollapsed,
                format!(
                    "card `{}` presents vendor docs as project-authoritative",
                    card.card_id
                ),
            );
        }
    }
}

fn check_deep_links(
    input: &DocsSearchLinkPacketInput,
    findings: &mut Vec<DocsSearchLinkValidationFinding>,
) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for link in &input.code_anchor_deep_links {
        if !seen_ids.insert(link.deep_link_id.as_str()) {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DuplicateDeepLinkId,
                format!("duplicate deep link id `{}`", link.deep_link_id),
            );
        }
        if !link.preserves_anchor_across_export {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DeepLinkAnchorNotPreserved,
                format!(
                    "deep link `{}` must preserve its code anchor across export",
                    link.deep_link_id
                ),
            );
        }
        if link.return_path_ref.trim().is_empty() {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DeepLinkReturnPathMissing,
                format!(
                    "deep link `{}` must keep a safe return path to the IDE",
                    link.deep_link_id
                ),
            );
        }
        if link.code_anchor.file_ref.trim().is_empty()
            || link.code_anchor.revision_ref.trim().is_empty()
        {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DeepLinkCodeAnchorIncomplete,
                format!(
                    "deep link `{}` code anchor must carry a file ref and revision",
                    link.deep_link_id
                ),
            );
        }
        if link.browser_handoff_reason.is_some()
            && link
                .destination_descriptor_ref
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                .unwrap_or(true)
        {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DeepLinkHandoffMissingDescriptor,
                format!(
                    "deep link `{}` with a browser handoff must carry a destination descriptor",
                    link.deep_link_id
                ),
            );
        }
    }
}

fn check_disclosures(
    input: &DocsSearchLinkPacketInput,
    findings: &mut Vec<DocsSearchLinkValidationFinding>,
) {
    let known_ids: BTreeSet<&str> = input
        .search_results
        .iter()
        .map(|row| row.result_id.as_str())
        .chain(input.symbol_cards.iter().map(|card| card.card_id.as_str()))
        .chain(
            input
                .code_anchor_deep_links
                .iter()
                .map(|link| link.deep_link_id.as_str()),
        )
        .collect();

    for disclosure in &input.resolution_disclosures {
        let repair_ok =
            !disclosure.disclosure_class.requires_repair_hook() || disclosure.repair_hook.is_some();
        if disclosure.summary.trim().is_empty()
            || disclosure.subject_id_ref.trim().is_empty()
            || !repair_ok
        {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DisclosureIncomplete,
                format!(
                    "disclosure `{}` for `{}` is incomplete",
                    disclosure.disclosure_class.as_str(),
                    disclosure.subject_id_ref
                ),
            );
        }
        if !disclosure.subject_id_ref.trim().is_empty()
            && !known_ids.contains(disclosure.subject_id_ref.as_str())
        {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::DisclosureOrphan,
                format!(
                    "disclosure references unknown subject `{}`",
                    disclosure.subject_id_ref
                ),
            );
        }
    }
}

fn check_consumer_projections(
    input: &DocsSearchLinkPacketInput,
    findings: &mut Vec<DocsSearchLinkValidationFinding>,
) {
    let present: BTreeSet<DocsSearchLinkConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::ConsumerProjectionPacketIdMismatch,
                format!(
                    "surface `{}` references packet `{}`",
                    projection.surface.as_str(),
                    projection.packet_id_ref
                ),
            );
        }
        if !projection.preserves_all() {
            push_finding(
                findings,
                DocsSearchLinkFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &DocsSearchLinkPacketInput,
    findings: &mut Vec<DocsSearchLinkValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("docs search link input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            DocsSearchLinkFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw query text, raw bodies, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across the validation
/// findings and the attached resolution disclosures.
fn promotion_state(
    findings: &[DocsSearchLinkValidationFinding],
    disclosures: &[DocsSearchLinkDisclosure],
) -> DocsSearchLinkPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == DocsSearchLinkFindingSeverity::Blocking)
        || disclosures
            .iter()
            .any(|disclosure| disclosure.severity == DocsSearchLinkFindingSeverity::Blocking);
    if any_blocking {
        return DocsSearchLinkPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == DocsSearchLinkFindingSeverity::Narrowing)
        || disclosures
            .iter()
            .any(|disclosure| disclosure.severity == DocsSearchLinkFindingSeverity::Narrowing);
    if any_narrowing {
        DocsSearchLinkPromotionState::NarrowedBelowStable
    } else {
        DocsSearchLinkPromotionState::Stable
    }
}

/// Heuristic that rejects obviously forbidden material in the export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw_query:")
                || lower.contains("raw_body:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable docs-search symbol-link input used by the producer, tests, and
/// fixtures.
pub fn seeded_stable_docs_search_link_input() -> DocsSearchLinkPacketInput {
    let packet_id = "packet:m5:docs_search_link:http_client_publish".to_owned();
    DocsSearchLinkPacketInput {
        packet_id: packet_id.clone(),
        search_label: "docs search: http client publish example".to_owned(),
        query_digest_ref: "querydigest:sha256:http-client-publish".to_owned(),
        search_results: vec![
            DocsSearchLinkResultRow {
                rank: 1,
                result_id: "result:project_docs:http_client_get".to_owned(),
                result_kind: DocsSearchLinkResultKind::SymbolReferenceResult,
                display_title: "http::Client::get".to_owned(),
                doc_node_ref: "docnode:project-docs:api/http-client#get".to_owned(),
                pack_id_ref: "pack:project-docs:aureline-workspace".to_owned(),
                pack_revision_ref: "pack-rev:project:aureline:2026.05.13".to_owned(),
                chips: DocsSearchLinkChipSet {
                    source_class: DocsSearchLinkSourceClass::ProjectDocs,
                    version_match: DocsSearchLinkVersionMatch::ExactBuildMatch,
                    freshness: DocsSearchLinkFreshness::AuthoritativeLive,
                },
                ranking_reason:
                    "exact symbol match in project docs at the active build with strong overlap"
                        .to_owned(),
                citation_anchor_refs: vec!["anchor:project:symbol:http::Client::get".to_owned()],
                symbol_card_id_ref: Some("symref:project:symbol:http-client-get".to_owned()),
                deep_link_id_ref: Some("deeplink:symbol:http-client-get".to_owned()),
                open_raw_escape_ref: "open-raw:docnode:project-docs:api/http-client".to_owned(),
                open_source_escape_ref: "open-source:repo:docs/api/http-client.md".to_owned(),
            },
            DocsSearchLinkResultRow {
                rank: 2,
                result_id: "result:mirrored:http_publish_guide".to_owned(),
                result_kind: DocsSearchLinkResultKind::DocsPageResult,
                display_title: "Publishing with the HTTP client".to_owned(),
                doc_node_ref: "docnode:mirror:http/publish-guide".to_owned(),
                pack_id_ref: "pack:mirrored-official:http-stdlib".to_owned(),
                pack_revision_ref: "pack-rev:mirror:http-stdlib:2026.03.22".to_owned(),
                chips: DocsSearchLinkChipSet {
                    source_class: DocsSearchLinkSourceClass::MirroredOfficialDocs,
                    version_match: DocsSearchLinkVersionMatch::CompatibleMinorDrift,
                    freshness: DocsSearchLinkFreshness::WarmCached,
                },
                ranking_reason:
                    "pinned, signed mirror of the official publish guide within the compat window"
                        .to_owned(),
                citation_anchor_refs: vec!["anchor:mirror:http-stdlib:page:publish".to_owned()],
                symbol_card_id_ref: None,
                deep_link_id_ref: Some("deeplink:file:http-publish-example".to_owned()),
                open_raw_escape_ref: "open-raw:docnode:mirror:http/publish-guide".to_owned(),
                open_source_escape_ref: "open-source:mirror:http-stdlib/publish-guide".to_owned(),
            },
            DocsSearchLinkResultRow {
                rank: 3,
                result_id: "result:curated:http_patterns".to_owned(),
                result_kind: DocsSearchLinkResultKind::CuratedPackResult,
                display_title: "HTTP client patterns".to_owned(),
                doc_node_ref: "docnode:knowledge-pack:http-patterns".to_owned(),
                pack_id_ref: "pack:curated:http-patterns".to_owned(),
                pack_revision_ref: "pack-rev:curated:http-patterns:2026.02.10".to_owned(),
                chips: DocsSearchLinkChipSet {
                    source_class: DocsSearchLinkSourceClass::CuratedKnowledgePack,
                    version_match: DocsSearchLinkVersionMatch::UnknownTargetBuild,
                    freshness: DocsSearchLinkFreshness::DegradedCached,
                },
                ranking_reason:
                    "curated knowledge pack match; target build unknown so version is disclosed"
                        .to_owned(),
                citation_anchor_refs: vec!["anchor:curated:http-patterns:overview".to_owned()],
                symbol_card_id_ref: Some("symref:project:guide:http-overview".to_owned()),
                deep_link_id_ref: None,
                open_raw_escape_ref: "open-raw:docnode:knowledge-pack:http-patterns".to_owned(),
                open_source_escape_ref: "open-source:pack:curated:http-patterns".to_owned(),
            },
        ],
        symbol_cards: vec![
            DocsSearchLinkSymbolCard {
                card_id: "symref:project:symbol:http-client-get".to_owned(),
                subject_kind: DocsSearchLinkSubjectKind::Symbol,
                subject_ref: "symbol:project:http::Client::get".to_owned(),
                display_label: "http::Client::get".to_owned(),
                source_class: DocsSearchLinkSourceClass::ProjectDocs,
                pack_id_ref: "pack:project-docs:aureline-workspace".to_owned(),
                pack_revision_ref: "pack-rev:project:aureline:2026.05.13".to_owned(),
                resolution_class: DocsSearchLinkResolutionClass::ExactSymbolMatch,
                resolution_fallback_chain: Vec::new(),
                project_vs_vendor_cue: DocsSearchLinkProjectVendorCue::ProjectAuthoritativeOnly,
                reuse_state: DocsSearchLinkReuseState::ReusableWithCitationAnchor,
                citation_anchor_refs: vec!["anchor:project:symbol:http::Client::get".to_owned()],
                browser_handoff_reason: None,
                destination_descriptor_ref: None,
                repair_hook: None,
                superseded_by_ref: None,
            },
            DocsSearchLinkSymbolCard {
                card_id: "symref:project:guide:http-overview".to_owned(),
                subject_kind: DocsSearchLinkSubjectKind::Symbol,
                subject_ref: "symbol:project:http::Client::publish".to_owned(),
                display_label: "http::Client::publish".to_owned(),
                source_class: DocsSearchLinkSourceClass::ProjectDocs,
                pack_id_ref: "pack:project-docs:aureline-workspace".to_owned(),
                pack_revision_ref: "pack-rev:project:aureline:2026.05.13".to_owned(),
                resolution_class: DocsSearchLinkResolutionClass::PackageLevelGuideFallback,
                resolution_fallback_chain: vec![
                    DocsSearchLinkResolutionClass::ExactSymbolMatch,
                    DocsSearchLinkResolutionClass::PackageLevelGuideFallback,
                ],
                project_vs_vendor_cue: DocsSearchLinkProjectVendorCue::ProjectOutranksVendorDefault,
                reuse_state: DocsSearchLinkReuseState::ReusableWithCitationAnchor,
                citation_anchor_refs: vec!["anchor:project:page:http/overview".to_owned()],
                browser_handoff_reason: None,
                destination_descriptor_ref: None,
                repair_hook: None,
                superseded_by_ref: None,
            },
        ],
        code_anchor_deep_links: vec![
            DocsSearchLinkDeepLink {
                deep_link_id: "deeplink:symbol:http-client-get".to_owned(),
                anchor_kind: DocsSearchLinkAnchorKind::SymbolRef,
                display_label: "Open http::Client::get".to_owned(),
                target_subject_ref: "symbol:project:http::Client::get".to_owned(),
                code_anchor: DocsSearchLinkCodeAnchor {
                    file_ref: "fileref:project:src/http/client.rs".to_owned(),
                    symbol_ref: Some("symbol:project:http::Client::get".to_owned()),
                    line_anchor_ref: Some("lineanchor:project:src/http/client.rs#L142".to_owned()),
                    revision_ref: "rev:project:aureline:2026.05.13".to_owned(),
                },
                preserves_anchor_across_export: true,
                return_path_ref: "return-path:ide:editor:open-symbol".to_owned(),
                browser_handoff_reason: None,
                destination_descriptor_ref: None,
            },
            DocsSearchLinkDeepLink {
                deep_link_id: "deeplink:file:http-publish-example".to_owned(),
                anchor_kind: DocsSearchLinkAnchorKind::FileLineRange,
                display_label: "Open publish example".to_owned(),
                target_subject_ref: "file:project:examples/http_publish.rs".to_owned(),
                code_anchor: DocsSearchLinkCodeAnchor {
                    file_ref: "fileref:project:examples/http_publish.rs".to_owned(),
                    symbol_ref: None,
                    line_anchor_ref: Some(
                        "lineanchor:project:examples/http_publish.rs#L8-L24".to_owned(),
                    ),
                    revision_ref: "rev:project:aureline:2026.05.13".to_owned(),
                },
                preserves_anchor_across_export: true,
                return_path_ref: "return-path:ide:editor:open-file-range".to_owned(),
                browser_handoff_reason: None,
                destination_descriptor_ref: None,
            },
        ],
        resolution_disclosures: vec![DocsSearchLinkDisclosure {
            disclosure_class: DocsSearchLinkDisclosureClass::PackageGuideFallback,
            severity: DocsSearchLinkFindingSeverity::Advisory,
            subject_id_ref: "symref:project:guide:http-overview".to_owned(),
            summary: "no dedicated symbol page exists yet; the package-level guide is shown"
                .to_owned(),
            repair_hook: None,
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    }
}

fn required_projections(packet_id: &str) -> Vec<DocsSearchLinkConsumerProjection> {
    [
        DocsSearchLinkConsumerSurface::DocsBrowser,
        DocsSearchLinkConsumerSurface::SearchShell,
        DocsSearchLinkConsumerSurface::AiExplanationOverlay,
        DocsSearchLinkConsumerSurface::RetrievalInspector,
        DocsSearchLinkConsumerSurface::GlossaryCard,
        DocsSearchLinkConsumerSurface::Onboarding,
        DocsSearchLinkConsumerSurface::SupportExport,
        DocsSearchLinkConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| DocsSearchLinkConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_symbol_resolution: true,
        preserves_citation_anchors: true,
        preserves_code_anchor_deep_links: true,
        preserves_open_raw_open_source_escape: true,
    })
    .collect()
}
