//! Frozen M5 docs-source, docs-result, docs-pack-manifest, version-match,
//! citation-set, and browser-handoff matrix.
//!
//! This module locks the canonical M5 documentation object model into one
//! export-safe packet. Each [`M5DocsObjectRow`] names one governed documentation
//! object — the docs source descriptor, the docs result object, the docs-pack
//! manifest, the derived-explanation citation set, the version-match/freshness
//! state, the stale-example finding, and the browser-handoff object — and binds
//! it to its qualification class, required fields, the controlled state
//! vocabularies it carries, the concrete vocabulary tokens it admits, evidence
//! requirements, the proof packet that keeps it current, downgrade triggers,
//! rollback posture, source contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether docs/help/onboarding/AI
//! rows may publish documentation claims. Docs browser, AI, onboarding, support,
//! and extension surfaces consume this packet rather than re-expressing docs
//! truth ad hoc: source class, locale, version match, freshness, mirror/offline
//! posture, trust class, and citation basis stay visible; project docs never
//! masquerade as vendor docs; derived explanations never outlive their citation
//! sets; and browser handoff cannot silently share context or impersonate a
//! governed docs surface.
//!
//! The controlled vocabularies mirror the canonical tokens already owned by the
//! docs-browser, docs-pack, derived-explanation, locale-overlay, and scoped
//! browser-handoff runtimes; the matrix freezes them in one self-describing
//! [`M5DocsContractsVocabularySet`] rather than minting parallel tokens. It
//! references the upstream source/result/pack/citation and browser-handoff
//! contracts by id. Raw document bodies, raw source files, rendered HTML, raw
//! URLs, raw provider payloads, credentials, and live vendor-doc snapshots stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json`](../../../../schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json).
//! The contract doc is
//! [`docs/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix.md`](../../../../docs/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/`](../../../../fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DocsContractsMatrixPacket`].
pub const M5_DOCS_CONTRACTS_MATRIX_RECORD_KIND: &str =
    "freeze_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix";

/// Schema version for M5 docs-contracts matrix records.
pub const M5_DOCS_CONTRACTS_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF: &str =
    "schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json";

/// Repo-relative path of the M5 docs-contracts matrix contract doc.
pub const M5_DOCS_CONTRACTS_MATRIX_DOC_REF: &str =
    "docs/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix.md";

/// Repo-relative path of the frozen docs source/result-pack/citation contract.
pub const M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF: &str =
    "schemas/docs/stable_docs_source_result_pack_and_citation.schema.json";

/// Repo-relative path of the frozen docs-pack manifest contract.
pub const M5_DOCS_CONTRACTS_PACK_MANIFEST_CONTRACT_REF: &str =
    "schemas/docs/docs_pack_manifest.schema.json";

/// Repo-relative path of the frozen derived-explanation descriptor contract.
pub const M5_DOCS_CONTRACTS_DERIVED_EXPLANATION_CONTRACT_REF: &str =
    "schemas/docs/derived_explanation_descriptor.schema.json";

/// Repo-relative path of the frozen docs-browser truth packet contract.
pub const M5_DOCS_CONTRACTS_DOCS_BROWSER_CONTRACT_REF: &str =
    "schemas/docs/docs_browser_truth_packet.schema.json";

/// Repo-relative path of the frozen browser-handoff packet contract.
pub const M5_DOCS_CONTRACTS_BROWSER_HANDOFF_CONTRACT_REF: &str =
    "schemas/integration/browser_handoff_packet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_CONTRACTS_MATRIX_FIXTURE_DIR: &str =
    "fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_DOCS_CONTRACTS_MATRIX_SUMMARY_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix.md";

/// One of the seven governed M5 documentation objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsObjectKind {
    /// Descriptor naming a documentation source's class, trust, locale, and
    /// mirror/offline posture.
    DocsSourceDescriptor,
    /// Result object returned by docs search / recall over a source.
    DocsResultObject,
    /// Docs-pack manifest describing an installed/mirrored documentation pack.
    DocsPackManifest,
    /// Derived-explanation citation set binding a generated explanation to its
    /// citations.
    DerivedExplanationCitationSet,
    /// Version-match / freshness state between a source and the active build.
    VersionMatchState,
    /// Stale-example finding flagging documented examples that drifted.
    StaleExampleFinding,
    /// Browser-handoff object describing why and how the product opens an
    /// external surface.
    BrowserHandoffObject,
}

impl M5DocsObjectKind {
    /// Every governed object, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DocsSourceDescriptor,
        Self::DocsResultObject,
        Self::DocsPackManifest,
        Self::DerivedExplanationCitationSet,
        Self::VersionMatchState,
        Self::StaleExampleFinding,
        Self::BrowserHandoffObject,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSourceDescriptor => "docs_source_descriptor",
            Self::DocsResultObject => "docs_result_object",
            Self::DocsPackManifest => "docs_pack_manifest",
            Self::DerivedExplanationCitationSet => "derived_explanation_citation_set",
            Self::VersionMatchState => "version_match_state",
            Self::StaleExampleFinding => "stale_example_finding",
            Self::BrowserHandoffObject => "browser_handoff_object",
        }
    }

    /// Controlled state vocabularies this object kind MUST declare.
    pub fn required_state_vocabularies(self) -> &'static [M5DocsContractStateVocabulary] {
        use M5DocsContractStateVocabulary as V;
        match self {
            Self::DocsSourceDescriptor => &[
                V::SourceClass,
                V::TrustClass,
                V::LocaleMatch,
                V::MirrorOfflinePosture,
            ],
            Self::DocsResultObject => &[
                V::SourceClass,
                V::TrustClass,
                V::VersionMatchState,
                V::FreshnessState,
            ],
            Self::DocsPackManifest => &[
                V::SourceClass,
                V::VersionMatchState,
                V::MirrorOfflinePosture,
                V::LocaleMatch,
            ],
            Self::DerivedExplanationCitationSet => {
                &[V::SourceClass, V::TrustClass, V::FreshnessState]
            }
            Self::VersionMatchState => &[V::VersionMatchState, V::FreshnessState],
            Self::StaleExampleFinding => &[V::VersionMatchState, V::FreshnessState],
            Self::BrowserHandoffObject => {
                &[V::BrowserHandoffReason, V::BrowserHandoffPrivacyConsequence]
            }
        }
    }
}

/// Qualification class for an M5 documentation object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsContractsQualificationClass {
    /// Object qualifies for the Stable claim.
    Stable,
    /// Object is narrowed to Beta.
    Beta,
    /// Object is narrowed to Preview.
    Preview,
    /// Object is experimental and not claimed.
    Experimental,
    /// Object is unavailable on this build.
    Unavailable,
    /// Object is held pending upstream resolution.
    Held,
}

impl M5DocsContractsQualificationClass {
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

    /// Whether the object may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Names one of the controlled state vocabularies a documentation object carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsContractStateVocabulary {
    /// Documentation source class.
    SourceClass,
    /// Version-match state.
    VersionMatchState,
    /// Freshness state.
    FreshnessState,
    /// Trust class.
    TrustClass,
    /// Locale-match state.
    LocaleMatch,
    /// Mirror/offline posture.
    MirrorOfflinePosture,
    /// Browser-handoff reason.
    BrowserHandoffReason,
    /// Browser-handoff privacy consequence.
    BrowserHandoffPrivacyConsequence,
}

impl M5DocsContractStateVocabulary {
    /// Every vocabulary, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SourceClass,
        Self::VersionMatchState,
        Self::FreshnessState,
        Self::TrustClass,
        Self::LocaleMatch,
        Self::MirrorOfflinePosture,
        Self::BrowserHandoffReason,
        Self::BrowserHandoffPrivacyConsequence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClass => "source_class",
            Self::VersionMatchState => "version_match_state",
            Self::FreshnessState => "freshness_state",
            Self::TrustClass => "trust_class",
            Self::LocaleMatch => "locale_match",
            Self::MirrorOfflinePosture => "mirror_offline_posture",
            Self::BrowserHandoffReason => "browser_handoff_reason",
            Self::BrowserHandoffPrivacyConsequence => "browser_handoff_privacy_consequence",
        }
    }
}

/// Controlled source class for a documentation object.
///
/// Mirrors the canonical docs-browser source-class vocabulary so docs browser,
/// AI, onboarding, support, and extension surfaces share one set of tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsContractSourceClass {
    /// Workspace-local project docs that ship with the repository or workspace.
    ProjectDocs,
    /// Signed, mirrored copy of official vendor / framework / language docs.
    MirroredOfficialDocs,
    /// Docs pack contributed by an installed extension.
    ExtensionDocsPack,
    /// Live external docs that require an explicit browser handoff.
    LiveExternalDocs,
    /// Curated knowledge-pack content (tutorials, glossaries, runbooks).
    CuratedKnowledgePack,
    /// Generated reference built from source identity and the running build.
    GeneratedReference,
    /// Derived explanation that is never primary authority.
    DerivedExplanation,
}

impl DocsContractSourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProjectDocs,
        Self::MirroredOfficialDocs,
        Self::ExtensionDocsPack,
        Self::LiveExternalDocs,
        Self::CuratedKnowledgePack,
        Self::GeneratedReference,
        Self::DerivedExplanation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::ExtensionDocsPack => "extension_docs_pack",
            Self::LiveExternalDocs => "live_external_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::GeneratedReference => "generated_reference",
            Self::DerivedExplanation => "derived_explanation",
        }
    }
}

/// Controlled version-match state between a documentation source and the active
/// build or workspace revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsContractVersionMatchState {
    /// Docs source exactly matches the active build or workspace revision.
    ExactBuildMatch,
    /// Docs source is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Docs source is incompatible with the active target.
    IncompatibleDriftDetected,
    /// Pre-release docs have not completed verification.
    PreReleaseUnverified,
    /// The target build or workspace revision could not be verified.
    UnknownTargetBuild,
}

impl DocsContractVersionMatchState {
    /// Every version-match state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExactBuildMatch,
        Self::CompatibleMinorDrift,
        Self::IncompatibleDriftDetected,
        Self::PreReleaseUnverified,
        Self::UnknownTargetBuild,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }
}

/// Controlled freshness state for a documentation object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsContractFreshnessState {
    /// Source was live and authoritative at mint time.
    AuthoritativeLive,
    /// Cached source remained within its freshness window.
    WarmCached,
    /// Cached source was usable only with degraded disclosure.
    DegradedCached,
    /// Source was stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
}

impl DocsContractFreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AuthoritativeLive,
        Self::WarmCached,
        Self::DegradedCached,
        Self::Stale,
        Self::Unverified,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }
}

/// Controlled trust class for a documentation object's source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsContractTrustClass {
    /// First-party authoritative source (workspace-owned project docs or
    /// generated reference bound to the running build).
    FirstPartyAuthoritative,
    /// Mirror was signed and verified against the published source.
    SignedMirrorVerified,
    /// Extension pack was signed by a verified publisher.
    ExtensionPackSigned,
    /// Live provider source resolved through an explicit handoff.
    LiveProviderHandoff,
    /// Derived inference; never primary authority.
    DerivedInferenceOnly,
}

impl DocsContractTrustClass {
    /// Every trust class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FirstPartyAuthoritative,
        Self::SignedMirrorVerified,
        Self::ExtensionPackSigned,
        Self::LiveProviderHandoff,
        Self::DerivedInferenceOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyAuthoritative => "first_party_authoritative",
            Self::SignedMirrorVerified => "signed_mirror_verified",
            Self::ExtensionPackSigned => "extension_pack_signed",
            Self::LiveProviderHandoff => "live_provider_handoff",
            Self::DerivedInferenceOnly => "derived_inference_only",
        }
    }
}

/// Controlled locale-match state for a documentation object.
///
/// Mirrors the canonical locale-overlay coverage vocabulary so localized docs
/// never imply reviewed parity they do not have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsContractLocaleMatch {
    /// Source-language content rendered without a translation overlay.
    SourceLanguageOriginal,
    /// Requested locale has complete reviewed coverage for the source revision.
    TranslatedComplete,
    /// Requested locale is reviewed for only part of the pack.
    TranslatedPartial,
    /// Requested locale was reviewed against an older source revision.
    TranslatedStale,
    /// Requested locale falls back to source-language content.
    SourceLanguageFallback,
    /// Requested locale overlay is missing or not installed.
    LocaleNotInstalled,
}

impl DocsContractLocaleMatch {
    /// Every locale-match state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceLanguageOriginal,
        Self::TranslatedComplete,
        Self::TranslatedPartial,
        Self::TranslatedStale,
        Self::SourceLanguageFallback,
        Self::LocaleNotInstalled,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLanguageOriginal => "source_language_original",
            Self::TranslatedComplete => "translated_complete",
            Self::TranslatedPartial => "translated_partial",
            Self::TranslatedStale => "translated_stale",
            Self::SourceLanguageFallback => "source_language_fallback",
            Self::LocaleNotInstalled => "locale_not_installed",
        }
    }
}

/// Controlled mirror/offline posture for a documentation object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsContractMirrorOfflinePosture {
    /// Source is live or online-only and requires explicit policy-aware handoff.
    LiveOnline,
    /// Source is local project documentation.
    LocalProjectPack,
    /// Source is locally generated reference material.
    GeneratedLocal,
    /// Source resolves through a signed or verified mirror.
    MirroredPack,
    /// Source is pinned for offline use.
    OfflinePinnedPack,
    /// Source resolves through a warm local cache.
    CachedLocal,
    /// Source pack or mirror is not installed locally.
    NotInstalled,
    /// Source came from a support pack.
    SupportPack,
}

impl DocsContractMirrorOfflinePosture {
    /// Every mirror/offline posture, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LiveOnline,
        Self::LocalProjectPack,
        Self::GeneratedLocal,
        Self::MirroredPack,
        Self::OfflinePinnedPack,
        Self::CachedLocal,
        Self::NotInstalled,
        Self::SupportPack,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveOnline => "live_online",
            Self::LocalProjectPack => "local_project_pack",
            Self::GeneratedLocal => "generated_local",
            Self::MirroredPack => "mirrored_pack",
            Self::OfflinePinnedPack => "offline_pinned_pack",
            Self::CachedLocal => "cached_local",
            Self::NotInstalled => "not_installed",
            Self::SupportPack => "support_pack",
        }
    }
}

/// Controlled reason a browser handoff was offered.
///
/// Mirrors the canonical scoped browser-handoff reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsContractBrowserHandoffReason {
    /// The exact anchor is not available locally; the upstream page has it.
    ExactAnchorUnavailableLocally,
    /// The live upstream version is newer than the local mirror.
    LiveVersionNewerThanMirror,
    /// The content is not mirrored; only the upstream source has it.
    SourceNotMirrored,
    /// A review thread requires the hosted review view.
    ReviewThreadRequiresHostedView,
    /// The reader explicitly asked to open in a browser surface.
    UserRequestedOpenInBrowser,
}

impl DocsContractBrowserHandoffReason {
    /// Every browser-handoff reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExactAnchorUnavailableLocally,
        Self::LiveVersionNewerThanMirror,
        Self::SourceNotMirrored,
        Self::ReviewThreadRequiresHostedView,
        Self::UserRequestedOpenInBrowser,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAnchorUnavailableLocally => "exact_anchor_unavailable_locally",
            Self::LiveVersionNewerThanMirror => "live_version_newer_than_mirror",
            Self::SourceNotMirrored => "source_not_mirrored",
            Self::ReviewThreadRequiresHostedView => "review_thread_requires_hosted_view",
            Self::UserRequestedOpenInBrowser => "user_requested_open_in_browser",
        }
    }
}

/// Controlled privacy consequence of a browser handoff.
///
/// Names exactly what context, if any, crosses the boundary when the product
/// opens an external surface. Context-sharing handoffs that exceed the qualified
/// scope are blocked rather than performed silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsContractBrowserHandoffPrivacyConsequence {
    /// The handoff opens with no workspace context shared.
    NoContextShared,
    /// Only the resolved destination URL or anchor crosses the boundary.
    ScopedUrlOnly,
    /// The user's query terms are shared and the sharing is disclosed.
    QueryTermsDisclosed,
    /// The handoff opens an isolated session that shares no prior state.
    IsolatedSession,
    /// A context-sharing handoff exceeded the qualified scope and was blocked.
    SharedContextBlocked,
}

impl DocsContractBrowserHandoffPrivacyConsequence {
    /// Every privacy consequence, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoContextShared,
        Self::ScopedUrlOnly,
        Self::QueryTermsDisclosed,
        Self::IsolatedSession,
        Self::SharedContextBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoContextShared => "no_context_shared",
            Self::ScopedUrlOnly => "scoped_url_only",
            Self::QueryTermsDisclosed => "query_terms_disclosed",
            Self::IsolatedSession => "isolated_session",
            Self::SharedContextBlocked => "shared_context_blocked",
        }
    }
}

/// Evidence requirement level for an object row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsContractsEvidenceRequirement {
    /// At least one proof packet is required.
    Required,
    /// Proof is recommended but not blocking.
    Recommended,
    /// Proof is optional.
    Optional,
    /// Not applicable for this object's current qualification.
    NotApplicable,
}

impl M5DocsContractsEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow an object below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsContractsDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Pinned, signed mirror is offline or unavailable.
    MirrorOffline,
    /// Source version no longer matches the indexed/pinned version.
    SourceVersionMismatch,
    /// Freshness window for the docs source expired.
    FreshnessExpired,
    /// Source trust narrowed.
    TrustNarrowing,
    /// Citation set backing a derived explanation expired.
    CitationSetExpired,
    /// Source class could no longer be verified.
    SourceClassUnverified,
    /// A browser handoff would risk leaking context beyond the qualified scope.
    HandoffContextLeakRisk,
    /// Locale overlay drifted from the source revision.
    LocaleSkewDetected,
    /// An upstream dependency object narrowed.
    UpstreamDependencyNarrowed,
}

impl M5DocsContractsDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::MirrorOffline,
        Self::SourceVersionMismatch,
        Self::FreshnessExpired,
        Self::TrustNarrowing,
        Self::CitationSetExpired,
        Self::SourceClassUnverified,
        Self::HandoffContextLeakRisk,
        Self::LocaleSkewDetected,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::MirrorOffline => "mirror_offline",
            Self::SourceVersionMismatch => "source_version_mismatch",
            Self::FreshnessExpired => "freshness_expired",
            Self::TrustNarrowing => "trust_narrowing",
            Self::CitationSetExpired => "citation_set_expired",
            Self::SourceClassUnverified => "source_class_unverified",
            Self::HandoffContextLeakRisk => "handoff_context_leak_risk",
            Self::LocaleSkewDetected => "locale_skew_detected",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for an object row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsContractsRollbackPosture {
    /// Source stays labeled and never silently impersonates a higher-trust class.
    SourceLabeledNeverImpersonated,
    /// Derived explanation is bound to its citation set and expires with it.
    CitationBoundExpiresWithCitations,
    /// Browser handoff stays isolated and preserves a safe return path to the IDE.
    HandoffIsolatedReturnPathPreserved,
    /// Version and freshness truth stays visible and never silently upgrades.
    VersionFreshnessVisible,
    /// Not applicable for the object's current qualification.
    NotApplicable,
}

impl M5DocsContractsRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLabeledNeverImpersonated => "source_labeled_never_impersonated",
            Self::CitationBoundExpiresWithCitations => "citation_bound_expires_with_citations",
            Self::HandoffIsolatedReturnPathPreserved => "handoff_isolated_return_path_preserved",
            Self::VersionFreshnessVisible => "version_freshness_visible",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project a documentation object's qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsContractsConsumerSurface {
    /// Docs browser / reader surface.
    DocsBrowser,
    /// Docs / code search surface.
    DocsSearch,
    /// AI explain / answer surface.
    AiExplain,
    /// Onboarding / tour surface.
    Onboarding,
    /// Help / About surface.
    HelpAbout,
    /// Support / export packet.
    SupportExport,
    /// Extension API consumer.
    ExtensionApi,
    /// Release center / publish review.
    ReleaseCenter,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Browser companion / handoff follow-up.
    BrowserCompanion,
}

impl M5DocsContractsConsumerSurface {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::DocsSearch => "docs_search",
            Self::AiExplain => "ai_explain",
            Self::Onboarding => "onboarding",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
            Self::ExtensionApi => "extension_api",
            Self::ReleaseCenter => "release_center",
            Self::Diagnostics => "diagnostics",
            Self::BrowserCompanion => "browser_companion",
        }
    }
}

/// One row in the M5 docs-contracts matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsObjectRow {
    /// Governed documentation object.
    pub object_kind: M5DocsObjectKind,
    /// Qualification class earned by this object.
    pub qualification: M5DocsContractsQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required fields the object must carry.
    pub required_fields: Vec<String>,
    /// Controlled state vocabularies this object carries.
    pub state_vocabularies: Vec<M5DocsContractStateVocabulary>,
    /// Source classes admitted by this object.
    pub source_classes: Vec<DocsContractSourceClass>,
    /// Version-match states admitted by this object.
    pub version_match_states: Vec<DocsContractVersionMatchState>,
    /// Freshness states admitted by this object.
    pub freshness_states: Vec<DocsContractFreshnessState>,
    /// Trust classes admitted by this object.
    pub trust_classes: Vec<DocsContractTrustClass>,
    /// Locale-match states admitted by this object.
    pub locale_matches: Vec<DocsContractLocaleMatch>,
    /// Mirror/offline postures admitted by this object.
    pub mirror_offline_postures: Vec<DocsContractMirrorOfflinePosture>,
    /// Browser-handoff reasons admitted by this object.
    pub handoff_reasons: Vec<DocsContractBrowserHandoffReason>,
    /// Browser-handoff privacy consequences admitted by this object.
    pub handoff_privacy_consequences: Vec<DocsContractBrowserHandoffPrivacyConsequence>,
    /// Evidence requirement level.
    pub evidence_requirement: M5DocsContractsEvidenceRequirement,
    /// Proof packet refs that keep this object current.
    pub required_proof_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this object.
    pub downgrade_triggers: Vec<M5DocsContractsDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5DocsContractsRollbackPosture,
    /// Source contract refs consumed by this object.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this object's qualification.
    pub consumer_surfaces: Vec<M5DocsContractsConsumerSurface>,
}

impl M5DocsObjectRow {
    /// Returns true when the row declares the given vocabulary.
    fn declares(&self, vocab: M5DocsContractStateVocabulary) -> bool {
        self.state_vocabularies.contains(&vocab)
    }

    /// Returns true when the token vec for `vocab` is non-empty.
    fn vocab_tokens_present(&self, vocab: M5DocsContractStateVocabulary) -> bool {
        use M5DocsContractStateVocabulary as V;
        match vocab {
            V::SourceClass => !self.source_classes.is_empty(),
            V::VersionMatchState => !self.version_match_states.is_empty(),
            V::FreshnessState => !self.freshness_states.is_empty(),
            V::TrustClass => !self.trust_classes.is_empty(),
            V::LocaleMatch => !self.locale_matches.is_empty(),
            V::MirrorOfflinePosture => !self.mirror_offline_postures.is_empty(),
            V::BrowserHandoffReason => !self.handoff_reasons.is_empty(),
            V::BrowserHandoffPrivacyConsequence => !self.handoff_privacy_consequences.is_empty(),
        }
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
///
/// Each field lists every canonical token for one controlled vocabulary, in
/// declaration order. The matrix validates each list against the typed `ALL`
/// arrays so the frozen vocabulary cannot silently drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsContractsVocabularySet {
    /// Source-class tokens.
    pub source_classes: Vec<String>,
    /// Version-match-state tokens.
    pub version_match_states: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Trust-class tokens.
    pub trust_classes: Vec<String>,
    /// Locale-match tokens.
    pub locale_matches: Vec<String>,
    /// Mirror/offline posture tokens.
    pub mirror_offline_postures: Vec<String>,
    /// Browser-handoff reason tokens.
    pub browser_handoff_reasons: Vec<String>,
    /// Browser-handoff privacy-consequence tokens.
    pub browser_handoff_privacy_consequences: Vec<String>,
}

impl M5DocsContractsVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            source_classes: DocsContractSourceClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            version_match_states: DocsContractVersionMatchState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            freshness_states: DocsContractFreshnessState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            trust_classes: DocsContractTrustClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            locale_matches: DocsContractLocaleMatch::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            mirror_offline_postures: DocsContractMirrorOfflinePosture::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            browser_handoff_reasons: DocsContractBrowserHandoffReason::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            browser_handoff_privacy_consequences: DocsContractBrowserHandoffPrivacyConsequence::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Trust and provenance review block.
///
/// Every flag is a hard invariant; all must hold for the matrix to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsContractsTrustReview {
    /// Source class, locale, version match, and freshness stay visible.
    pub source_class_locale_version_freshness_visible: bool,
    /// Project docs never masquerade as vendor docs.
    pub project_docs_never_masquerade_as_vendor: bool,
    /// Derived explanations never outlive their citation sets.
    pub derived_explanations_never_outlive_citation_sets: bool,
    /// Citations stay bound to source identity and version.
    pub citations_bound_to_source_and_version: bool,
    /// Version match and freshness are never silently upgraded.
    pub version_match_and_freshness_never_silently_upgraded: bool,
    /// Mirror/offline state stays disclosed.
    pub mirror_offline_state_disclosed: bool,
    /// Browser handoff never silently shares context.
    pub handoff_never_silently_shares_context: bool,
    /// Browser handoff never impersonates a governed docs surface.
    pub handoff_never_impersonates_governed_docs: bool,
    /// Stale examples are surfaced rather than hidden.
    pub stale_examples_surfaced_not_hidden: bool,
    /// No speculative knowledge-platform or hosted-search product is in scope.
    pub no_speculative_knowledge_platform_or_hosted_search: bool,
    /// Downgrade narrows the claim rather than hiding the object.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified objects automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsContractsConsumerProjection {
    /// Docs browser consumes the shared object model.
    pub docs_browser_consumes_object_model: bool,
    /// Docs search shows result-object truth.
    pub docs_search_shows_result_object_truth: bool,
    /// AI explain shows the derived-explanation citation set.
    pub ai_explain_shows_citation_set: bool,
    /// Onboarding shows source and freshness truth.
    pub onboarding_shows_source_and_freshness: bool,
    /// Support export shows the shared object model.
    pub support_export_shows_object_model: bool,
    /// Extension API consumes the same object model.
    pub extension_api_consumes_same_object_model: bool,
    /// Release center shows qualification truth.
    pub release_center_shows_qualification: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_qualification: bool,
    /// Browser companion shows handoff reason and privacy consequence.
    pub browser_companion_shows_handoff_reason_and_privacy: bool,
    /// Preview / Labs surfaces are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_objects: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsContractsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the object.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the docs-contracts lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsContractsReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet for the lane.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every object.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every object.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`M5DocsContractsMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsContractsMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object rows.
    pub object_rows: Vec<M5DocsObjectRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsContractsVocabularySet,
    /// Trust review block.
    pub trust_review: M5DocsContractsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsContractsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsContractsProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DocsContractsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 docs-contracts matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsContractsMatrixPacket {
    /// Record kind; must equal [`M5_DOCS_CONTRACTS_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_CONTRACTS_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object rows.
    pub object_rows: Vec<M5DocsObjectRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsContractsVocabularySet,
    /// Trust review block.
    pub trust_review: M5DocsContractsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsContractsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsContractsProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DocsContractsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsContractsMatrixPacket {
    /// Builds an M5 docs-contracts matrix packet from stable-lane input.
    pub fn new(input: M5DocsContractsMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_CONTRACTS_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_CONTRACTS_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            object_rows: input.object_rows,
            vocabulary_set: input.vocabulary_set,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 docs-contracts matrix invariants.
    pub fn validate(&self) -> Vec<M5DocsContractsMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_CONTRACTS_MATRIX_RECORD_KIND {
            violations.push(M5DocsContractsMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_CONTRACTS_MATRIX_SCHEMA_VERSION {
            violations.push(M5DocsContractsMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsContractsMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_object_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 docs-contracts matrix packet serializes"),
        ) {
            violations.push(M5DocsContractsMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 docs-contracts matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .object_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Docs Source, Result, Pack, Version-Match, Citation-Set, and Browser-Handoff Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Objects: {} ({} stable)\n",
            self.object_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Objects\n\n");
        for row in &self.object_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.object_kind.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Vocabularies: {}\n",
                row.state_vocabularies
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 docs-contracts matrix export.
#[derive(Debug)]
pub enum M5DocsContractsMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsContractsMatrixViolation>),
}

impl fmt::Display for M5DocsContractsMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 docs-contracts matrix export parse failed: {error}"
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
                    "m5 docs-contracts matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsContractsMatrixArtifactError {}

/// Validation failures emitted by [`M5DocsContractsMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsContractsMatrixViolation {
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
    /// A required governed object is missing from the matrix.
    RequiredObjectMissing,
    /// An object row is incomplete.
    ObjectRowIncomplete,
    /// An object row omits a vocabulary its kind requires.
    RequiredVocabularyMissing,
    /// A declared vocabulary has no concrete tokens.
    DeclaredVocabularyHasNoTokens,
    /// A token vec is populated for a vocabulary the row does not declare.
    UndeclaredVocabularyHasTokens,
    /// An object claiming Stable is missing required proof packet refs.
    StableObjectMissingProof,
    /// An object has no downgrade triggers.
    DowngradeTriggersMissing,
    /// An object has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5DocsContractsMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ObjectRowIncomplete => "object_row_incomplete",
            Self::RequiredVocabularyMissing => "required_vocabulary_missing",
            Self::DeclaredVocabularyHasNoTokens => "declared_vocabulary_has_no_tokens",
            Self::UndeclaredVocabularyHasTokens => "undeclared_vocabulary_has_tokens",
            Self::StableObjectMissingProof => "stable_object_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 docs-contracts matrix export.
pub fn current_stable_m5_docs_contracts_matrix_export(
) -> Result<M5DocsContractsMatrixPacket, M5DocsContractsMatrixArtifactError> {
    let packet: M5DocsContractsMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/support_export.json"
    )))
    .map_err(M5DocsContractsMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsContractsMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DocsContractsMatrixPacket,
    violations: &mut Vec<M5DocsContractsMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF,
        M5_DOCS_CONTRACTS_MATRIX_DOC_REF,
        M5_DOCS_CONTRACTS_SOURCE_RESULT_PACK_CONTRACT_REF,
        M5_DOCS_CONTRACTS_PACK_MANIFEST_CONTRACT_REF,
        M5_DOCS_CONTRACTS_DERIVED_EXPLANATION_CONTRACT_REF,
        M5_DOCS_CONTRACTS_DOCS_BROWSER_CONTRACT_REF,
        M5_DOCS_CONTRACTS_BROWSER_HANDOFF_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsContractsMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DocsContractsMatrixPacket,
    violations: &mut Vec<M5DocsContractsMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DocsContractsMatrixViolation::VocabularySetDrift);
    }
}

fn validate_object_rows(
    packet: &M5DocsContractsMatrixPacket,
    violations: &mut Vec<M5DocsContractsMatrixViolation>,
) {
    let present: BTreeSet<M5DocsObjectKind> = packet
        .object_rows
        .iter()
        .map(|row| row.object_kind)
        .collect();
    for required in M5DocsObjectKind::ALL {
        if !present.contains(&required) {
            violations.push(M5DocsContractsMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.object_rows {
        if row.scope_summary.trim().is_empty()
            || row.required_fields.is_empty()
            || row.state_vocabularies.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5DocsContractsMatrixViolation::ObjectRowIncomplete);
        }

        for required_vocab in row.object_kind.required_state_vocabularies() {
            if !row.declares(*required_vocab) {
                violations.push(M5DocsContractsMatrixViolation::RequiredVocabularyMissing);
            }
        }

        for vocab in M5DocsContractStateVocabulary::ALL {
            let declared = row.declares(vocab);
            let has_tokens = row.vocab_tokens_present(vocab);
            if declared && !has_tokens {
                violations.push(M5DocsContractsMatrixViolation::DeclaredVocabularyHasNoTokens);
            }
            if !declared && has_tokens {
                violations.push(M5DocsContractsMatrixViolation::UndeclaredVocabularyHasTokens);
            }
        }

        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DocsContractsMatrixViolation::StableObjectMissingProof);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DocsContractsMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DocsContractsMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5DocsContractsMatrixPacket,
    violations: &mut Vec<M5DocsContractsMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.source_class_locale_version_freshness_visible,
        review.project_docs_never_masquerade_as_vendor,
        review.derived_explanations_never_outlive_citation_sets,
        review.citations_bound_to_source_and_version,
        review.version_match_and_freshness_never_silently_upgraded,
        review.mirror_offline_state_disclosed,
        review.handoff_never_silently_shares_context,
        review.handoff_never_impersonates_governed_docs,
        review.stale_examples_surfaced_not_hidden,
        review.no_speculative_knowledge_platform_or_hosted_search,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5DocsContractsMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsContractsMatrixPacket,
    violations: &mut Vec<M5DocsContractsMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.docs_browser_consumes_object_model,
        projection.docs_search_shows_result_object_truth,
        projection.ai_explain_shows_citation_set,
        projection.onboarding_shows_source_and_freshness,
        projection.support_export_shows_object_model,
        projection.extension_api_consumes_same_object_model,
        projection.release_center_shows_qualification,
        projection.help_about_shows_qualification,
        projection.browser_companion_shows_handoff_reason_and_privacy,
        projection.preview_labs_label_for_unqualified_objects,
    ] {
        if !ok {
            violations.push(M5DocsContractsMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsContractsMatrixPacket,
    violations: &mut Vec<M5DocsContractsMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsContractsMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DocsContractsMatrixPacket,
    violations: &mut Vec<M5DocsContractsMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(M5DocsContractsMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
