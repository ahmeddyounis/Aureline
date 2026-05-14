//! Citation-preserving docs-node records and evidence projections.
//!
//! The types in this module are the alpha contract shared by docs/help,
//! search, graph explainers, onboarding/help packs, support exports, and AI
//! evidence packets. They model citation identity only: source bodies and raw
//! provider URLs remain owned by their source systems.

use serde::{Deserialize, Serialize};

/// Schema version shared by the docs-node, citation-anchor, drawer, and export records.
pub const DOCS_CITATION_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DocsNodeIdentity`] payloads.
pub const DOCS_NODE_ALPHA_RECORD_KIND: &str = "docs_node_alpha_record";

/// Stable record-kind tag carried by [`CitationAnchorAlpha`] payloads.
pub const CITATION_ANCHOR_ALPHA_RECORD_KIND: &str = "citation_anchor_alpha_record";

/// Stable record-kind tag carried by [`CitationDrawerEvidenceView`] payloads.
pub const CITATION_DRAWER_ALPHA_RECORD_KIND: &str = "citation_drawer_alpha_record";

/// Stable record-kind tag carried by [`CitationEvidenceExport`] payloads.
pub const CITATION_EVIDENCE_EXPORT_ALPHA_RECORD_KIND: &str =
    "citation_evidence_export_alpha_record";

/// Documentation node family rendered or exported by a knowledge surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsNodeKind {
    /// Product help or docs/help article.
    ProductHelp,
    /// Project, API, package, language, or framework reference page.
    ReferencePage,
    /// Glossary or terminology item in a help or knowledge pack.
    GlossaryItem,
    /// Onboarding card or first-run teaching item.
    OnboardingCard,
    /// Guided-tour or learning step.
    GuidedTourStep,
    /// Derived explainer or architecture summary.
    DerivedExplainer,
    /// AI evidence source row that cites docs or knowledge material.
    AiEvidenceSource,
    /// Support runbook or support-only help item.
    SupportRunbook,
}

impl DocsNodeKind {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductHelp => "product_help",
            Self::ReferencePage => "reference_page",
            Self::GlossaryItem => "glossary_item",
            Self::OnboardingCard => "onboarding_card",
            Self::GuidedTourStep => "guided_tour_step",
            Self::DerivedExplainer => "derived_explainer",
            Self::AiEvidenceSource => "ai_evidence_source",
            Self::SupportRunbook => "support_runbook",
        }
    }
}

/// Source class for docs nodes and citation anchors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationSourceClass {
    /// Workspace-local docs, ADRs, runbooks, or onboarding notes.
    ProjectDocs,
    /// Generated reference bound to source and build identity.
    GeneratedReference,
    /// Signed mirror of official vendor, framework, or language docs.
    MirroredOfficialDocs,
    /// Curated glossary, tutorial, or onboarding knowledge pack.
    CuratedKnowledgePack,
    /// Live vendor or provider docs that require explicit handoff.
    VendorProviderDocs,
    /// Support runbook or support pack source.
    SupportRunbook,
    /// Derived explanation that is never primary authority.
    DerivedExplanation,
}

impl CitationSourceClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::VendorProviderDocs => "vendor_provider_docs",
            Self::SupportRunbook => "support_runbook",
            Self::DerivedExplanation => "derived_explanation",
        }
    }

    /// Returns true when the source can stand as primary authority.
    pub const fn is_authoritative_input(self) -> bool {
        !matches!(self, Self::VendorProviderDocs | Self::DerivedExplanation)
    }
}

/// Scope class that bounds where a docs node is authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsScopeClass {
    /// Current workspace or repo scope.
    Workspace,
    /// Product docs/help scope.
    DocsHelp,
    /// Onboarding and first-run teaching scope.
    Onboarding,
    /// Glossary or help-pack scope.
    HelpPack,
    /// Graph or codebase explainer scope.
    Explainer,
    /// AI evidence or context-inspector scope.
    AiEvidence,
    /// Support export or repair packet scope.
    SupportExport,
}

impl DocsScopeClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::DocsHelp => "docs_help",
            Self::Onboarding => "onboarding",
            Self::HelpPack => "help_pack",
            Self::Explainer => "explainer",
            Self::AiEvidence => "ai_evidence",
            Self::SupportExport => "support_export",
        }
    }
}

/// Freshness class captured when a docs node or citation is minted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsFreshnessClass {
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

impl DocsFreshnessClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    /// Returns true when the source lowers certainty.
    pub const fn lowers_certainty(self) -> bool {
        matches!(self, Self::DegradedCached | Self::Stale | Self::Unverified)
    }
}

/// Build or version match between a docs source and the active product/workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionMatchState {
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

impl VersionMatchState {
    /// Returns the stable string token.
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

/// Locality or install posture for a citation-bearing source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationLocalityClass {
    /// Source is a local project docs pack.
    LocalProjectPack,
    /// Source is a generated local artifact.
    GeneratedLocal,
    /// Source resolves through a verified mirror or offline pack.
    MirroredOffline,
    /// Source resolves through a warm local cache.
    CachedLocal,
    /// Source requires live vendor or provider handoff.
    VendorLive,
    /// Source is referenced but not installed locally.
    NotInstalled,
    /// Source resolves through a support pack.
    SupportPack,
}

impl CitationLocalityClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalProjectPack => "local_project_pack",
            Self::GeneratedLocal => "generated_local",
            Self::MirroredOffline => "mirrored_offline",
            Self::CachedLocal => "cached_local",
            Self::VendorLive => "vendor_live",
            Self::NotInstalled => "not_installed",
            Self::SupportPack => "support_pack",
        }
    }
}

/// Availability state for an exact citation anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationAnchorAvailability {
    /// Exact anchor exists and can be opened.
    ExactAnchorAvailable,
    /// Source is citable but the requested exact anchor is unavailable.
    AnchorUnavailableDisclosed,
    /// Anchor exists but is hidden by policy.
    HiddenByPolicy,
    /// Anchor was intentionally omitted from the export.
    OmittedByPolicy,
    /// Source is not citation-bearing.
    NotCitationBearing,
}

impl CitationAnchorAvailability {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAnchorAvailable => "exact_anchor_available",
            Self::AnchorUnavailableDisclosed => "anchor_unavailable_disclosed",
            Self::HiddenByPolicy => "hidden_by_policy",
            Self::OmittedByPolicy => "omitted_by_policy",
            Self::NotCitationBearing => "not_citation_bearing",
        }
    }

    /// Returns true when a note must explain the missing, hidden, or omitted anchor.
    pub const fn requires_note(self) -> bool {
        matches!(
            self,
            Self::AnchorUnavailableDisclosed | Self::HiddenByPolicy | Self::OmittedByPolicy
        )
    }

    /// Returns true when the anchor can be opened exactly.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::ExactAnchorAvailable)
    }
}

/// Locale overlay and source-language fallback state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleOverlayState {
    /// Active content is canonical source-language content.
    SourceLanguageOriginal,
    /// Requested locale is available and reviewed.
    RequestedLocaleAvailable,
    /// Requested locale is available but partial or machine assisted.
    LocaleOverlayPartial,
    /// Requested locale was authored against an older source revision.
    LocaleOverlayStale,
    /// Requested locale is missing and source-language fallback is visible.
    LocaleMissingFallbackToSourceLanguage,
    /// Requested locale pack is missing or not installed.
    LocaleMissingNotInstalled,
    /// No translation exists and source-language content is shown.
    UntranslatedSourceLanguageOnly,
}

impl LocaleOverlayState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLanguageOriginal => "source_language_original",
            Self::RequestedLocaleAvailable => "requested_locale_available",
            Self::LocaleOverlayPartial => "locale_overlay_partial",
            Self::LocaleOverlayStale => "locale_overlay_stale",
            Self::LocaleMissingFallbackToSourceLanguage => {
                "locale_missing_fallback_to_source_language"
            }
            Self::LocaleMissingNotInstalled => "locale_missing_not_installed",
            Self::UntranslatedSourceLanguageOnly => "untranslated_source_language_only",
        }
    }

    /// Returns true when export must preserve a source-language fallback ref.
    pub const fn requires_source_language_fallback(self) -> bool {
        matches!(
            self,
            Self::LocaleOverlayPartial
                | Self::LocaleOverlayStale
                | Self::LocaleMissingFallbackToSourceLanguage
                | Self::UntranslatedSourceLanguageOnly
        )
    }
}

/// Whether a cited claim is raw source truth or derived inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationInferenceMarker {
    /// Claim is a direct source citation.
    RawSource,
    /// Claim is derived from one or more cited sources.
    Inference,
    /// Claim is a heuristic bridge over partial evidence.
    Heuristic,
    /// Claim is generated summary text with cited basis.
    GeneratedSummary,
}

impl CitationInferenceMarker {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawSource => "raw_source",
            Self::Inference => "inference",
            Self::Heuristic => "heuristic",
            Self::GeneratedSummary => "generated_summary",
        }
    }
}

/// Confidence class for a citation row or derived claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationConfidenceClass {
    /// Evidence supports the claim for the declared scope.
    EvidenceBacked,
    /// Claim is explicitly inferred from the cited set.
    Inferred,
    /// Claim has low confidence and must not be rendered as certain.
    LowConfidence,
    /// Confidence is unknown or unverified.
    UnknownUnverified,
}

impl CitationConfidenceClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceBacked => "evidence_backed",
            Self::Inferred => "inferred",
            Self::LowConfidence => "low_confidence",
            Self::UnknownUnverified => "unknown_unverified",
        }
    }
}

/// Inspectable relationship between project docs and vendor docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePrecedenceClass {
    /// Project docs are the only cited authority.
    ProjectAuthoritativeOnly,
    /// Project docs outrank vendor docs by default.
    ProjectOutranksVendorDefault,
    /// Vendor docs override project docs under an explicit policy or source rule.
    VendorOverrideDisclosed,
    /// Both project and vendor docs remain visible because they disagree.
    ProjectVendorDisagreementInspectable,
    /// No precedence issue applies.
    NotApplicable,
}

impl SourcePrecedenceClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectAuthoritativeOnly => "project_authoritative_only",
            Self::ProjectOutranksVendorDefault => "project_outranks_vendor_default",
            Self::VendorOverrideDisclosed => "vendor_override_disclosed",
            Self::ProjectVendorDisagreementInspectable => "project_vendor_disagreement_inspectable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Stable docs-node identity consumed by citation-aware surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsNodeIdentity {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable docs-node id.
    pub docs_node_id: String,
    /// Documentation node kind.
    pub doc_kind: DocsNodeKind,
    /// Source class for this node.
    pub source_class: CitationSourceClass,
    /// Scope where this node is authoritative.
    pub scope_class: DocsScopeClass,
    /// Owning pack or source id.
    pub source_pack_ref: String,
    /// Owning pack revision or source snapshot.
    pub source_pack_revision_ref: String,
    /// Version or revision represented by the node.
    pub version_or_revision_ref: String,
    /// Version-match state against the active target.
    pub version_match_state: VersionMatchState,
    /// Freshness state at mint time.
    pub freshness_class: DocsFreshnessClass,
    /// Locality posture for the source.
    pub locality_class: CitationLocalityClass,
    /// Canonical source locale.
    pub source_locale: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective rendered locale.
    pub effective_locale: String,
    /// Locale overlay and fallback posture.
    pub locale_overlay_state: LocaleOverlayState,
    /// Source-language fallback ref when fallback applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_language_fallback_ref: Option<String>,
    /// Citation-anchor availability for this node.
    pub citation_availability: CitationAnchorAvailability,
    /// Citation anchor refs backing this node.
    pub citation_anchor_refs: Vec<String>,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Hidden, omitted, fallback, or missing-anchor disclosure note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_or_omitted_note: Option<String>,
}

impl DocsNodeIdentity {
    /// Builds a docs-node identity with stable record metadata.
    pub fn new(input: DocsNodeIdentityInput) -> Self {
        Self {
            record_kind: DOCS_NODE_ALPHA_RECORD_KIND.to_owned(),
            schema_version: DOCS_CITATION_ALPHA_SCHEMA_VERSION,
            docs_node_id: input.docs_node_id,
            doc_kind: input.doc_kind,
            source_class: input.source_class,
            scope_class: input.scope_class,
            source_pack_ref: input.source_pack_ref,
            source_pack_revision_ref: input.source_pack_revision_ref,
            version_or_revision_ref: input.version_or_revision_ref,
            version_match_state: input.version_match_state,
            freshness_class: input.freshness_class,
            locality_class: input.locality_class,
            source_locale: input.source_locale,
            requested_locale: input.requested_locale,
            effective_locale: input.effective_locale,
            locale_overlay_state: input.locale_overlay_state,
            source_language_fallback_ref: input.source_language_fallback_ref,
            citation_availability: input.citation_availability,
            citation_anchor_refs: input.citation_anchor_refs,
            exact_reopen_ref: input.exact_reopen_ref,
            hidden_or_omitted_note: input.hidden_or_omitted_note,
        }
    }

    /// Returns true when this node can open at least one exact citation.
    pub fn has_exact_anchor(&self) -> bool {
        self.citation_availability.is_exact() && !self.citation_anchor_refs.is_empty()
    }

    /// Returns true when freshness, version, locale, locality, or anchor state lowers certainty.
    pub fn degrades_certainty(&self) -> bool {
        self.freshness_class.lowers_certainty()
            || self.version_match_state != VersionMatchState::ExactBuildMatch
            || self.locality_class == CitationLocalityClass::NotInstalled
            || self.citation_availability != CitationAnchorAvailability::ExactAnchorAvailable
            || self
                .locale_overlay_state
                .requires_source_language_fallback()
    }

    /// Validates docs-node identity without resolving source bodies.
    pub fn validate(&self) -> Vec<CitationTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != DOCS_NODE_ALPHA_RECORD_KIND {
            violations.push(CitationTruthViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_CITATION_ALPHA_SCHEMA_VERSION {
            violations.push(CitationTruthViolation::WrongSchemaVersion);
        }
        if self.docs_node_id.trim().is_empty()
            || self.source_pack_ref.trim().is_empty()
            || self.source_pack_revision_ref.trim().is_empty()
            || self.version_or_revision_ref.trim().is_empty()
            || self.exact_reopen_ref.trim().is_empty()
        {
            violations.push(CitationTruthViolation::MissingStableIdentity);
        }
        if self.source_locale.trim().is_empty()
            || self.requested_locale.trim().is_empty()
            || self.effective_locale.trim().is_empty()
        {
            violations.push(CitationTruthViolation::MissingLocaleTruth);
        }
        if self.citation_availability == CitationAnchorAvailability::ExactAnchorAvailable
            && self.citation_anchor_refs.is_empty()
        {
            violations.push(CitationTruthViolation::ExactAnchorMissing);
        }
        if self.citation_availability.requires_note()
            && self
                .hidden_or_omitted_note
                .as_deref()
                .map_or(true, |note| note.trim().is_empty())
        {
            violations.push(CitationTruthViolation::MissingAnchorDisclosure);
        }
        if self
            .locale_overlay_state
            .requires_source_language_fallback()
            && self
                .source_language_fallback_ref
                .as_deref()
                .map_or(true, |fallback| fallback.trim().is_empty())
        {
            violations.push(CitationTruthViolation::MissingSourceLanguageFallback);
        }
        violations
    }
}

/// Constructor fields for [`DocsNodeIdentity`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsNodeIdentityInput {
    /// Stable docs-node id.
    pub docs_node_id: String,
    /// Documentation node kind.
    pub doc_kind: DocsNodeKind,
    /// Source class for this node.
    pub source_class: CitationSourceClass,
    /// Scope where this node is authoritative.
    pub scope_class: DocsScopeClass,
    /// Owning pack or source id.
    pub source_pack_ref: String,
    /// Owning pack revision or source snapshot.
    pub source_pack_revision_ref: String,
    /// Version or revision represented by the node.
    pub version_or_revision_ref: String,
    /// Version-match state against the active target.
    pub version_match_state: VersionMatchState,
    /// Freshness state at mint time.
    pub freshness_class: DocsFreshnessClass,
    /// Locality posture for the source.
    pub locality_class: CitationLocalityClass,
    /// Canonical source locale.
    pub source_locale: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective rendered locale.
    pub effective_locale: String,
    /// Locale overlay and fallback posture.
    pub locale_overlay_state: LocaleOverlayState,
    /// Source-language fallback ref when fallback applies.
    pub source_language_fallback_ref: Option<String>,
    /// Citation-anchor availability for this node.
    pub citation_availability: CitationAnchorAvailability,
    /// Citation anchor refs backing this node.
    pub citation_anchor_refs: Vec<String>,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Hidden, omitted, fallback, or missing-anchor disclosure note.
    pub hidden_or_omitted_note: Option<String>,
}

/// Citation-anchor row with pack, locale, freshness, and inference truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationAnchorAlpha {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable citation anchor id.
    pub anchor_id: String,
    /// Docs node this anchor belongs to.
    pub docs_node_ref: String,
    /// Source class for this anchor.
    pub source_class: CitationSourceClass,
    /// Owning pack or source id.
    pub source_pack_ref: String,
    /// Owning pack revision or source snapshot.
    pub source_pack_revision_ref: String,
    /// Stable target ref such as a docs page, symbol, code span, or help-pack item.
    pub target_ref: String,
    /// Exact anchor ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_anchor_ref: Option<String>,
    /// Locale rendered for the anchor.
    pub locale: String,
    /// Version-match state at mint time.
    pub version_match_state: VersionMatchState,
    /// Freshness state at mint time.
    pub freshness_class: DocsFreshnessClass,
    /// Locality posture for the source.
    pub locality_class: CitationLocalityClass,
    /// Citation availability state.
    pub citation_availability: CitationAnchorAvailability,
    /// Inference marker for this anchor.
    pub inference_marker: CitationInferenceMarker,
    /// Confidence class for this anchor.
    pub confidence_class: CitationConfidenceClass,
    /// Hidden, omitted, missing-anchor, or inference disclosure note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_or_omitted_note: Option<String>,
}

impl CitationAnchorAlpha {
    /// Builds a citation anchor with stable record metadata.
    pub fn new(input: CitationAnchorAlphaInput) -> Self {
        Self {
            record_kind: CITATION_ANCHOR_ALPHA_RECORD_KIND.to_owned(),
            schema_version: DOCS_CITATION_ALPHA_SCHEMA_VERSION,
            anchor_id: input.anchor_id,
            docs_node_ref: input.docs_node_ref,
            source_class: input.source_class,
            source_pack_ref: input.source_pack_ref,
            source_pack_revision_ref: input.source_pack_revision_ref,
            target_ref: input.target_ref,
            exact_anchor_ref: input.exact_anchor_ref,
            locale: input.locale,
            version_match_state: input.version_match_state,
            freshness_class: input.freshness_class,
            locality_class: input.locality_class,
            citation_availability: input.citation_availability,
            inference_marker: input.inference_marker,
            confidence_class: input.confidence_class,
            hidden_or_omitted_note: input.hidden_or_omitted_note,
        }
    }

    /// Validates the anchor without resolving source bodies.
    pub fn validate(&self) -> Vec<CitationTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != CITATION_ANCHOR_ALPHA_RECORD_KIND {
            violations.push(CitationTruthViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_CITATION_ALPHA_SCHEMA_VERSION {
            violations.push(CitationTruthViolation::WrongSchemaVersion);
        }
        if self.anchor_id.trim().is_empty()
            || self.docs_node_ref.trim().is_empty()
            || self.source_pack_ref.trim().is_empty()
            || self.source_pack_revision_ref.trim().is_empty()
            || self.target_ref.trim().is_empty()
            || self.locale.trim().is_empty()
        {
            violations.push(CitationTruthViolation::MissingStableIdentity);
        }
        if self.citation_availability == CitationAnchorAvailability::ExactAnchorAvailable
            && self
                .exact_anchor_ref
                .as_deref()
                .map_or(true, |anchor| anchor.trim().is_empty())
        {
            violations.push(CitationTruthViolation::ExactAnchorMissing);
        }
        if self.citation_availability.requires_note()
            && self
                .hidden_or_omitted_note
                .as_deref()
                .map_or(true, |note| note.trim().is_empty())
        {
            violations.push(CitationTruthViolation::MissingAnchorDisclosure);
        }
        if self.source_class == CitationSourceClass::DerivedExplanation
            && self.inference_marker == CitationInferenceMarker::RawSource
        {
            violations.push(CitationTruthViolation::DerivedClaimMissingInferenceMarker);
        }
        violations
    }
}

/// Constructor fields for [`CitationAnchorAlpha`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationAnchorAlphaInput {
    /// Stable citation anchor id.
    pub anchor_id: String,
    /// Docs node this anchor belongs to.
    pub docs_node_ref: String,
    /// Source class for this anchor.
    pub source_class: CitationSourceClass,
    /// Owning pack or source id.
    pub source_pack_ref: String,
    /// Owning pack revision or source snapshot.
    pub source_pack_revision_ref: String,
    /// Stable target ref such as a docs page, symbol, code span, or help-pack item.
    pub target_ref: String,
    /// Exact anchor ref when available.
    pub exact_anchor_ref: Option<String>,
    /// Locale rendered for the anchor.
    pub locale: String,
    /// Version-match state at mint time.
    pub version_match_state: VersionMatchState,
    /// Freshness state at mint time.
    pub freshness_class: DocsFreshnessClass,
    /// Locality posture for the source.
    pub locality_class: CitationLocalityClass,
    /// Citation availability state.
    pub citation_availability: CitationAnchorAvailability,
    /// Inference marker for this anchor.
    pub inference_marker: CitationInferenceMarker,
    /// Confidence class for this anchor.
    pub confidence_class: CitationConfidenceClass,
    /// Hidden, omitted, missing-anchor, or inference disclosure note.
    pub hidden_or_omitted_note: Option<String>,
}

/// One row rendered in a citation drawer or equivalent evidence view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationDrawerRow {
    /// Stable row id inside the drawer.
    pub row_id: String,
    /// Citation anchor id or disclosed missing-anchor ref.
    pub citation_ref: String,
    /// Source class token.
    pub source_class: CitationSourceClass,
    /// Source pack revision ref.
    pub source_pack_revision_ref: String,
    /// Target ref cited by this row.
    pub target_ref: String,
    /// Exact anchor ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_anchor_ref: Option<String>,
    /// Freshness token for the row.
    pub freshness_class: DocsFreshnessClass,
    /// Locality token for the row.
    pub locality_class: CitationLocalityClass,
    /// Citation availability state.
    pub citation_availability: CitationAnchorAvailability,
    /// Inference marker for this row.
    pub inference_marker: CitationInferenceMarker,
    /// Confidence state for this row.
    pub confidence_class: CitationConfidenceClass,
    /// Hidden, omitted, missing-anchor, or inference disclosure note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_or_omitted_note: Option<String>,
}

impl CitationDrawerRow {
    /// Builds a drawer row from a citation anchor.
    pub fn from_anchor(anchor: &CitationAnchorAlpha) -> Self {
        Self {
            row_id: format!("citation-row:{}", anchor.anchor_id),
            citation_ref: anchor.anchor_id.clone(),
            source_class: anchor.source_class,
            source_pack_revision_ref: anchor.source_pack_revision_ref.clone(),
            target_ref: anchor.target_ref.clone(),
            exact_anchor_ref: anchor.exact_anchor_ref.clone(),
            freshness_class: anchor.freshness_class,
            locality_class: anchor.locality_class,
            citation_availability: anchor.citation_availability,
            inference_marker: anchor.inference_marker,
            confidence_class: anchor.confidence_class,
            hidden_or_omitted_note: anchor.hidden_or_omitted_note.clone(),
        }
    }

    /// Returns true when the row lowers certainty because citation quality is incomplete.
    pub fn degrades_certainty(&self) -> bool {
        self.freshness_class.lowers_certainty()
            || self.citation_availability != CitationAnchorAvailability::ExactAnchorAvailable
            || matches!(
                self.confidence_class,
                CitationConfidenceClass::LowConfidence | CitationConfidenceClass::UnknownUnverified
            )
            || matches!(
                self.inference_marker,
                CitationInferenceMarker::Inference
                    | CitationInferenceMarker::Heuristic
                    | CitationInferenceMarker::GeneratedSummary
            )
    }
}

/// Citation drawer or equivalent evidence view opened from a docs/help row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationDrawerEvidenceView {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable drawer id.
    pub drawer_id: String,
    /// Docs node described by this drawer.
    pub docs_node: DocsNodeIdentity,
    /// Citation rows displayed in the drawer.
    pub rows: Vec<CitationDrawerRow>,
    /// Project-vendor precedence posture.
    pub project_vendor_precedence: SourcePrecedenceClass,
    /// Stable non-canvas fallback ref for list/table/breadcrumb rendering.
    pub non_canvas_fallback_ref: String,
}

impl CitationDrawerEvidenceView {
    /// Builds a drawer with stable record metadata.
    pub fn new(input: CitationDrawerEvidenceViewInput) -> Self {
        Self {
            record_kind: CITATION_DRAWER_ALPHA_RECORD_KIND.to_owned(),
            schema_version: DOCS_CITATION_ALPHA_SCHEMA_VERSION,
            drawer_id: input.drawer_id,
            docs_node: input.docs_node,
            rows: input.rows,
            project_vendor_precedence: input.project_vendor_precedence,
            non_canvas_fallback_ref: input.non_canvas_fallback_ref,
        }
    }

    /// Builds a drawer from a docs node and citation anchors.
    pub fn from_node_and_anchors(
        drawer_id: impl Into<String>,
        docs_node: DocsNodeIdentity,
        anchors: impl IntoIterator<Item = CitationAnchorAlpha>,
        project_vendor_precedence: SourcePrecedenceClass,
        non_canvas_fallback_ref: impl Into<String>,
    ) -> Self {
        let rows = anchors
            .into_iter()
            .map(|anchor| CitationDrawerRow::from_anchor(&anchor))
            .collect();
        Self::new(CitationDrawerEvidenceViewInput {
            drawer_id: drawer_id.into(),
            docs_node,
            rows,
            project_vendor_precedence,
            non_canvas_fallback_ref: non_canvas_fallback_ref.into(),
        })
    }

    /// Returns true when any row or the owning docs node lowers certainty.
    pub fn degrades_certainty(&self) -> bool {
        self.docs_node.degrades_certainty() || self.rows.iter().any(|row| row.degrades_certainty())
    }

    /// Validates drawer linkage and citation availability.
    pub fn validate(&self) -> Vec<CitationTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != CITATION_DRAWER_ALPHA_RECORD_KIND {
            violations.push(CitationTruthViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_CITATION_ALPHA_SCHEMA_VERSION {
            violations.push(CitationTruthViolation::WrongSchemaVersion);
        }
        if self.drawer_id.trim().is_empty() || self.non_canvas_fallback_ref.trim().is_empty() {
            violations.push(CitationTruthViolation::MissingStableIdentity);
        }
        violations.extend(self.docs_node.validate());
        if self.docs_node.citation_availability == CitationAnchorAvailability::ExactAnchorAvailable
            && self.rows.is_empty()
        {
            violations.push(CitationTruthViolation::DrawerMissingCitationRows);
        }
        for row in &self.rows {
            if row.source_pack_revision_ref != self.docs_node.source_pack_revision_ref {
                violations.push(CitationTruthViolation::DrawerRowRevisionMismatch);
            }
            if row.citation_availability.requires_note()
                && row
                    .hidden_or_omitted_note
                    .as_deref()
                    .map_or(true, |note| note.trim().is_empty())
            {
                violations.push(CitationTruthViolation::MissingAnchorDisclosure);
            }
        }
        violations
    }
}

/// Constructor fields for [`CitationDrawerEvidenceView`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationDrawerEvidenceViewInput {
    /// Stable drawer id.
    pub drawer_id: String,
    /// Docs node described by this drawer.
    pub docs_node: DocsNodeIdentity,
    /// Citation rows displayed in the drawer.
    pub rows: Vec<CitationDrawerRow>,
    /// Project-vendor precedence posture.
    pub project_vendor_precedence: SourcePrecedenceClass,
    /// Stable non-canvas fallback ref for list/table/breadcrumb rendering.
    pub non_canvas_fallback_ref: String,
}

/// Help-pack, glossary, translated-hint, or onboarding item preserved in export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpPackItemEvidence {
    /// Stable pack id.
    pub pack_id: String,
    /// Stable item id inside the pack.
    pub item_id: String,
    /// Item kind token such as `glossary_item` or `onboarding_card`.
    pub item_kind: DocsNodeKind,
    /// Pack revision ref.
    pub pack_revision_ref: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective rendered locale.
    pub effective_locale: String,
    /// Locale overlay and fallback posture.
    pub locale_overlay_state: LocaleOverlayState,
    /// Source-language fallback ref when fallback applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_language_fallback_ref: Option<String>,
    /// Citation anchor refs preserved for reconstruction.
    pub citation_anchor_refs: Vec<String>,
    /// Citation availability for this item.
    pub citation_availability: CitationAnchorAvailability,
}

impl HelpPackItemEvidence {
    /// Validates locale and citation reconstruction fields.
    pub fn validate(&self) -> Vec<CitationTruthViolation> {
        let mut violations = Vec::new();
        if self.pack_id.trim().is_empty()
            || self.item_id.trim().is_empty()
            || self.pack_revision_ref.trim().is_empty()
            || self.requested_locale.trim().is_empty()
            || self.effective_locale.trim().is_empty()
        {
            violations.push(CitationTruthViolation::MissingStableIdentity);
        }
        if self
            .locale_overlay_state
            .requires_source_language_fallback()
            && self
                .source_language_fallback_ref
                .as_deref()
                .map_or(true, |fallback| fallback.trim().is_empty())
        {
            violations.push(CitationTruthViolation::MissingSourceLanguageFallback);
        }
        if self.citation_availability == CitationAnchorAvailability::ExactAnchorAvailable
            && self.citation_anchor_refs.is_empty()
        {
            violations.push(CitationTruthViolation::ExactAnchorMissing);
        }
        violations
    }
}

/// Export-safe citation evidence packet for support, sharing, onboarding, and AI handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationEvidenceExport {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Surface family that produced the export.
    pub surface_ref: String,
    /// Export generation timestamp or deterministic fixture stamp.
    pub generated_at: String,
    /// Docs nodes preserved in the export.
    pub docs_nodes: Vec<DocsNodeIdentity>,
    /// Citation drawers preserved in the export.
    pub citation_drawers: Vec<CitationDrawerEvidenceView>,
    /// Help-pack, glossary, translated-hint, or onboarding item rows.
    pub help_pack_items: Vec<HelpPackItemEvidence>,
    /// AI evidence packet refs that consumed this citation model.
    pub ai_evidence_packet_refs: Vec<String>,
    /// Graph or explainer packet refs that consumed this citation model.
    pub explainer_packet_refs: Vec<String>,
}

impl CitationEvidenceExport {
    /// Builds an export with stable record metadata.
    pub fn new(input: CitationEvidenceExportInput) -> Self {
        Self {
            record_kind: CITATION_EVIDENCE_EXPORT_ALPHA_RECORD_KIND.to_owned(),
            schema_version: DOCS_CITATION_ALPHA_SCHEMA_VERSION,
            export_id: input.export_id,
            surface_ref: input.surface_ref,
            generated_at: input.generated_at,
            docs_nodes: input.docs_nodes,
            citation_drawers: input.citation_drawers,
            help_pack_items: input.help_pack_items,
            ai_evidence_packet_refs: input.ai_evidence_packet_refs,
            explainer_packet_refs: input.explainer_packet_refs,
        }
    }

    /// Validates the export and every nested citation object.
    pub fn validate(&self) -> Vec<CitationTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != CITATION_EVIDENCE_EXPORT_ALPHA_RECORD_KIND {
            violations.push(CitationTruthViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_CITATION_ALPHA_SCHEMA_VERSION {
            violations.push(CitationTruthViolation::WrongSchemaVersion);
        }
        if self.export_id.trim().is_empty()
            || self.surface_ref.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            violations.push(CitationTruthViolation::MissingStableIdentity);
        }
        if self.docs_nodes.is_empty() {
            violations.push(CitationTruthViolation::ExportMissingDocsNodes);
        }
        for node in &self.docs_nodes {
            violations.extend(node.validate());
        }
        for drawer in &self.citation_drawers {
            violations.extend(drawer.validate());
        }
        for item in &self.help_pack_items {
            violations.extend(item.validate());
        }
        violations
    }

    /// Deterministic JSON serialization for support/export fixtures.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only export fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("citation evidence export serializes")
    }
}

/// Constructor fields for [`CitationEvidenceExport`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationEvidenceExportInput {
    /// Stable export id.
    pub export_id: String,
    /// Surface family that produced the export.
    pub surface_ref: String,
    /// Export generation timestamp or deterministic fixture stamp.
    pub generated_at: String,
    /// Docs nodes preserved in the export.
    pub docs_nodes: Vec<DocsNodeIdentity>,
    /// Citation drawers preserved in the export.
    pub citation_drawers: Vec<CitationDrawerEvidenceView>,
    /// Help-pack, glossary, translated-hint, or onboarding item rows.
    pub help_pack_items: Vec<HelpPackItemEvidence>,
    /// AI evidence packet refs that consumed this citation model.
    pub ai_evidence_packet_refs: Vec<String>,
    /// Graph or explainer packet refs that consumed this citation model.
    pub explainer_packet_refs: Vec<String>,
}

/// Validation failure emitted by the citation alpha model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationTruthViolation {
    /// Record kind did not match the expected discriminator.
    WrongRecordKind,
    /// Schema version did not match the supported alpha version.
    WrongSchemaVersion,
    /// A required stable identity field was empty.
    MissingStableIdentity,
    /// Locale, requested locale, or effective locale was missing.
    MissingLocaleTruth,
    /// Exact-anchor availability was declared without an anchor ref.
    ExactAnchorMissing,
    /// Missing, hidden, or omitted anchor state lacked a disclosure note.
    MissingAnchorDisclosure,
    /// Locale fallback was required but no source-language fallback ref was present.
    MissingSourceLanguageFallback,
    /// A derived claim lacked an inference marker.
    DerivedClaimMissingInferenceMarker,
    /// A citation drawer had no rows for a citation-bearing node.
    DrawerMissingCitationRows,
    /// A citation drawer row pointed at a different pack revision than its docs node.
    DrawerRowRevisionMismatch,
    /// A support or citation export contained no docs-node rows.
    ExportMissingDocsNodes,
}

impl CitationTruthViolation {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingStableIdentity => "missing_stable_identity",
            Self::MissingLocaleTruth => "missing_locale_truth",
            Self::ExactAnchorMissing => "exact_anchor_missing",
            Self::MissingAnchorDisclosure => "missing_anchor_disclosure",
            Self::MissingSourceLanguageFallback => "missing_source_language_fallback",
            Self::DerivedClaimMissingInferenceMarker => "derived_claim_missing_inference_marker",
            Self::DrawerMissingCitationRows => "drawer_missing_citation_rows",
            Self::DrawerRowRevisionMismatch => "drawer_row_revision_mismatch",
            Self::ExportMissingDocsNodes => "export_missing_docs_nodes",
        }
    }
}
