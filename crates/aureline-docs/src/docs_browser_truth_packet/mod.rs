//! Docs-browser source/result truth packet.
//!
//! This module owns the stable contract that certifies docs-browser source
//! descriptors, docs-result objects, machine-readable version-match /
//! source-class / freshness enums, and symbol-linked docs flows. It binds the
//! promise that every claimed stable docs-browser row:
//!
//! - Pins a [`DocsBrowserSourceDescriptor`] carrying source class, provider or
//!   pack id, locale, trust class, and browser-handoff capability.
//! - Pins a [`DocsBrowserResultObject`] carrying result id, title,
//!   docs-source ref, version-match state, freshness state, symbol refs where
//!   present, and snippet or citation anchors.
//! - Reuses the closed [`DocsBrowserSourceClass`], [`DocsBrowserVersionMatchState`],
//!   and [`DocsBrowserFreshnessState`] enums on the shell, AI inspector,
//!   onboarding, support export, and extension API surfaces instead of
//!   minting surface-local prose.
//! - Preserves the docs-result object identity through every symbol-linked
//!   flow (peek, split, browser handoff, support export, and AI handoff)
//!   rather than regenerating anonymous rows at each step.
//!
//! The packet is intentionally metadata-only — it carries no raw document
//! bodies, no raw provider URLs, no secrets, and no ambient credentials —
//! and is read verbatim by the docs-browser shell, peek overlay, split pane,
//! browser-handoff packet, AI context inspector, extension API,
//! onboarding tour, support export, and the release proof index.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`DocsBrowserTruthPacket`].
pub const DOCS_BROWSER_TRUTH_PACKET_RECORD_KIND: &str = "docs_browser_truth_packet";

/// Stable record-kind tag for [`DocsBrowserTruthSupportExport`].
pub const DOCS_BROWSER_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_browser_truth_support_export";

/// Integer schema version for the docs-browser truth packet.
pub const DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the reviewer doc.
pub const DOCS_BROWSER_TRUTH_PACKET_DOC_REF: &str = "docs/search/m4/docs_browser_truth_packet.md";

/// Repo-relative path of the milestone-level doc note.
pub const DOCS_BROWSER_TRUTH_PACKET_MILESTONE_DOC_REF: &str =
    "docs/m4/docs_browser_truth_packet.md";

/// Repo-relative path of the human-readable artifact narrative.
pub const DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/docs_browser_truth_packet.md";

/// Repo-relative path of the JSON schema.
pub const DOCS_BROWSER_TRUTH_PACKET_SCHEMA_REF: &str =
    "schemas/docs/docs_browser_truth_packet.schema.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const DOCS_BROWSER_TRUTH_PACKET_FIXTURE_DIR: &str =
    "fixtures/search/m4/docs_browser_truth_packet";

/// Repo-relative path of the checked-in stable docs-browser packet.
pub const DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/docs_browser_truth_packet.json";

/// Closed source class for a docs-browser source descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserSourceClass {
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

impl DocsBrowserSourceClass {
    /// Every required closed source class.
    pub const REQUIRED: [Self; 5] = [
        Self::ProjectDocs,
        Self::MirroredOfficialDocs,
        Self::ExtensionDocsPack,
        Self::LiveExternalDocs,
        Self::DerivedExplanation,
    ];

    /// Stable token used in fixtures, schemas, support exports, and the API.
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

    /// True when this source class always requires an explicit browser
    /// handoff to open the canonical source body.
    pub const fn requires_browser_handoff(self) -> bool {
        matches!(self, Self::LiveExternalDocs)
    }

    /// True when this source class is never primary authority on its own.
    pub const fn is_derived_only(self) -> bool {
        matches!(self, Self::DerivedExplanation)
    }
}

/// Trust class for a docs-browser source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserTrustClass {
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

impl DocsBrowserTrustClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyAuthoritative => "first_party_authoritative",
            Self::SignedMirrorVerified => "signed_mirror_verified",
            Self::ExtensionPackSigned => "extension_pack_signed",
            Self::LiveProviderHandoff => "live_provider_handoff",
            Self::DerivedInferenceOnly => "derived_inference_only",
        }
    }

    /// True when this trust class can stand as primary authority.
    pub const fn is_authoritative(self) -> bool {
        !matches!(self, Self::DerivedInferenceOnly)
    }
}

/// Browser-handoff capability for a docs-browser source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserHandoffCapability {
    /// The source resolves locally; no browser handoff is required.
    NotRequiredLocal,
    /// A browser handoff is available through an explicit user action.
    AvailableExplicit,
    /// A browser handoff would apply but is blocked by policy.
    BlockedByPolicy,
    /// A browser handoff would apply but is unavailable (offline, etc.).
    UnavailableDisclosed,
}

impl DocsBrowserHandoffCapability {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequiredLocal => "not_required_local",
            Self::AvailableExplicit => "available_explicit",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::UnavailableDisclosed => "unavailable_disclosed",
        }
    }

    /// True when the surface must render a visible disclosure note.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::BlockedByPolicy | Self::UnavailableDisclosed)
    }

    /// True when an explicit handoff packet ref must be carried.
    pub const fn requires_handoff_packet_ref(self) -> bool {
        matches!(self, Self::AvailableExplicit)
    }
}

/// Machine-readable version-match state shared by shell, AI, onboarding,
/// support export, and extension surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserVersionMatchState {
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

impl DocsBrowserVersionMatchState {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }

    /// True when the result row must carry a visible downgrade note.
    pub const fn lowers_certainty(self) -> bool {
        !matches!(self, Self::ExactBuildMatch)
    }
}

/// Machine-readable freshness state shared by shell, AI, onboarding, support
/// export, and extension surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserFreshnessState {
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

impl DocsBrowserFreshnessState {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    /// True when the result lowers certainty.
    pub const fn lowers_certainty(self) -> bool {
        matches!(self, Self::DegradedCached | Self::Stale | Self::Unverified)
    }
}

/// Symbol-link availability for a docs-browser result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserSymbolLinkClass {
    /// Exact symbol anchor is available; peek and split can open it directly.
    SymbolAnchorAvailable,
    /// Partial symbol link (e.g. fuzzy match) requires a disclosure note.
    SymbolAnchorPartial,
    /// Symbol link is missing; the row must disclose the gap.
    SymbolAnchorMissingDisclosed,
    /// Row is not symbol-linked (general docs page, runbook, etc.).
    NotSymbolLinked,
}

impl DocsBrowserSymbolLinkClass {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SymbolAnchorAvailable => "symbol_anchor_available",
            Self::SymbolAnchorPartial => "symbol_anchor_partial",
            Self::SymbolAnchorMissingDisclosed => "symbol_anchor_missing_disclosed",
            Self::NotSymbolLinked => "not_symbol_linked",
        }
    }

    /// True when the row must carry at least one symbol ref.
    pub const fn requires_symbol_ref(self) -> bool {
        matches!(
            self,
            Self::SymbolAnchorAvailable | Self::SymbolAnchorPartial
        )
    }

    /// True when the row must carry a disclosure note.
    pub const fn requires_disclosure(self) -> bool {
        matches!(
            self,
            Self::SymbolAnchorPartial | Self::SymbolAnchorMissingDisclosed
        )
    }
}

/// Captured-vs-live status for the docs-browser truth packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserCapturedVsLive {
    /// Packet was captured from a live in-product session.
    Live,
    /// Packet was reconstructed from a captured snapshot.
    CapturedSnapshot,
    /// Packet was reconstructed by replaying a captured snapshot after a
    /// narrowed-scope rerun.
    NarrowedScopeRerun,
}

impl DocsBrowserCapturedVsLive {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::NarrowedScopeRerun => "narrowed_scope_rerun",
        }
    }
}

/// Closed consumer surface that must inherit the docs-browser truth packet
/// verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserConsumerSurface {
    /// In-product docs-browser shell.
    DocsBrowserShell,
    /// Peek overlay over the docs-browser row.
    PeekOverlay,
    /// Split-pane companion view.
    SplitPane,
    /// Browser-handoff packet (open in external browser).
    BrowserHandoffPacket,
    /// AI context inspector projection.
    AiContextInspector,
    /// Extension API projection.
    ExtensionApi,
    /// Onboarding tour projection.
    OnboardingTour,
    /// Support export bundle projection.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl DocsBrowserConsumerSurface {
    /// Every required consumer surface in declaration order.
    pub const REQUIRED: [Self; 9] = [
        Self::DocsBrowserShell,
        Self::PeekOverlay,
        Self::SplitPane,
        Self::BrowserHandoffPacket,
        Self::AiContextInspector,
        Self::ExtensionApi,
        Self::OnboardingTour,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::PeekOverlay => "peek_overlay",
            Self::SplitPane => "split_pane",
            Self::BrowserHandoffPacket => "browser_handoff_packet",
            Self::AiContextInspector => "ai_context_inspector",
            Self::ExtensionApi => "extension_api",
            Self::OnboardingTour => "onboarding_tour",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// Closed promotion state for the docs-browser truth packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserPromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet has reviewable findings and must remain narrowed below stable.
    NarrowedBelowStable,
    /// Packet has blocker findings and cannot publish on stable surfaces.
    BlocksStable,
}

impl DocsBrowserPromotionState {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`DocsBrowserTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Packet identity refs are empty.
    MissingPacketIdentity,
    /// Packet declared no docs-browser source descriptors.
    MissingSourceDescriptors,
    /// Packet declared no docs-browser result objects.
    MissingResultObjects,
    /// A source descriptor field required for stable claims is empty.
    SourceDescriptorIncomplete,
    /// A docs-result object references a docs-source ref that no descriptor declared.
    ResultSourceRefUnpinned,
    /// A docs-result object lost its stable identity (result id, title, anchor refs).
    ResultIdentityIncomplete,
    /// A docs-result row missed a required disclosure note for its citation/anchor state.
    ResultDisclosureMissing,
    /// A symbol-linked row dropped its symbol refs even though the link class declared them.
    SymbolRefsMissing,
    /// A symbol-linked row lost identity in a peek / split / handoff / support / export step.
    SymbolFlowIdentityLost,
    /// A symbol flow references a result id no result object declared.
    SymbolFlowResultRefUnpinned,
    /// A browser-handoff capability was declared available without a packet ref.
    BrowserHandoffPacketMissing,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection drops or remints docs-browser truth.
    ConsumerProjectionDrift,
    /// A consumer projection drops the source-class taxonomy binding.
    SourceClassTaxonomyDropped,
    /// A consumer projection drops the version-match vocabulary binding.
    VersionMatchVocabularyDropped,
    /// A consumer projection drops the freshness vocabulary binding.
    FreshnessVocabularyDropped,
    /// A consumer projection drops the symbol-flow identity binding.
    SymbolFlowIdentityDropped,
    /// A row admits raw URLs, raw bodies, secrets, or provider payloads.
    RawBoundaryMaterialPresent,
    /// The packet covers fewer source classes than the stable claim requires.
    RequiredSourceClassCoverageMissing,
    /// A row pinned the `derived_explanation` source class without a downgrade note.
    DerivedExplanationNotDowngraded,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl DocsBrowserFindingKind {
    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingSourceDescriptors => "missing_source_descriptors",
            Self::MissingResultObjects => "missing_result_objects",
            Self::SourceDescriptorIncomplete => "source_descriptor_incomplete",
            Self::ResultSourceRefUnpinned => "result_source_ref_unpinned",
            Self::ResultIdentityIncomplete => "result_identity_incomplete",
            Self::ResultDisclosureMissing => "result_disclosure_missing",
            Self::SymbolRefsMissing => "symbol_refs_missing",
            Self::SymbolFlowIdentityLost => "symbol_flow_identity_lost",
            Self::SymbolFlowResultRefUnpinned => "symbol_flow_result_ref_unpinned",
            Self::BrowserHandoffPacketMissing => "browser_handoff_packet_missing",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::SourceClassTaxonomyDropped => "source_class_taxonomy_dropped",
            Self::VersionMatchVocabularyDropped => "version_match_vocabulary_dropped",
            Self::FreshnessVocabularyDropped => "freshness_vocabulary_dropped",
            Self::SymbolFlowIdentityDropped => "symbol_flow_identity_dropped",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::RequiredSourceClassCoverageMissing => "required_source_class_coverage_missing",
            Self::DerivedExplanationNotDowngraded => "derived_explanation_not_downgraded",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the docs-browser validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserValidationFinding {
    /// Closed finding kind.
    pub finding_kind: DocsBrowserFindingKind,
    /// Finding severity.
    pub severity: DocsBrowserFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl DocsBrowserValidationFinding {
    fn new(
        finding_kind: DocsBrowserFindingKind,
        severity: DocsBrowserFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Docs-browser source descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserSourceDescriptor {
    /// Stable source id used by results and symbol flows.
    pub source_id: String,
    /// Closed source class.
    pub source_class: DocsBrowserSourceClass,
    /// Stable provider or pack id.
    pub provider_or_pack_id: String,
    /// Stable provider or pack revision ref.
    pub provider_or_pack_revision_ref: String,
    /// Locale rendered by this source (BCP-47).
    pub locale: String,
    /// Trust class for this source.
    pub trust_class: DocsBrowserTrustClass,
    /// Browser-handoff capability for this source.
    pub browser_handoff: DocsBrowserHandoffCapability,
    /// Browser-handoff packet ref when handoff is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Disclosure note for blocked or unavailable handoff postures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_disclosure_note: Option<String>,
    /// True when raw URLs, raw bodies, secrets, and provider payloads are excluded.
    pub raw_boundary_material_excluded: bool,
}

impl DocsBrowserSourceDescriptor {
    fn is_complete(&self) -> bool {
        !self.source_id.trim().is_empty()
            && !self.provider_or_pack_id.trim().is_empty()
            && !self.provider_or_pack_revision_ref.trim().is_empty()
            && !self.locale.trim().is_empty()
    }
}

/// One symbol reference attached to a docs-browser result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserSymbolRef {
    /// Stable symbol id.
    pub symbol_id: String,
    /// Redaction-aware display label.
    pub display_label: String,
    /// Source code ref backing the symbol.
    pub source_ref: String,
    /// Symbol-kind token (`function`, `type`, `module`, …).
    pub symbol_kind_token: String,
}

/// One citation or snippet anchor inside a docs-browser result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserCitationAnchor {
    /// Stable anchor id.
    pub anchor_id: String,
    /// Target ref (docs section, snippet, code span, …).
    pub target_ref: String,
    /// Anchor class token (`section`, `heading`, `snippet`, `api`, …).
    pub anchor_class_token: String,
}

/// Docs-browser result object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserResultObject {
    /// Stable result id.
    pub result_id: String,
    /// User-visible title.
    pub title: String,
    /// Docs-source ref (matches a [`DocsBrowserSourceDescriptor::source_id`]).
    pub docs_source_ref: String,
    /// Machine-readable version-match state.
    pub version_match_state: DocsBrowserVersionMatchState,
    /// Machine-readable freshness state.
    pub freshness_state: DocsBrowserFreshnessState,
    /// Symbol-link availability for the row.
    pub symbol_link_class: DocsBrowserSymbolLinkClass,
    /// Symbol refs preserved by symbol-linked rows.
    #[serde(default)]
    pub symbol_refs: Vec<DocsBrowserSymbolRef>,
    /// Citation or snippet anchors preserved by the row.
    #[serde(default)]
    pub citation_anchors: Vec<DocsBrowserCitationAnchor>,
    /// Optional snippet anchor ref for the visible preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet_anchor_ref: Option<String>,
    /// Downgrade note when version/freshness/symbol-link state lowers certainty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_note: Option<String>,
    /// True when raw URLs, raw bodies, secrets, and provider payloads are excluded.
    pub raw_boundary_material_excluded: bool,
}

impl DocsBrowserResultObject {
    fn requires_downgrade_note(&self) -> bool {
        self.version_match_state.lowers_certainty() || self.freshness_state.lowers_certainty()
    }

    fn has_downgrade_note(&self) -> bool {
        self.downgrade_note
            .as_deref()
            .map(|note| !note.trim().is_empty())
            .unwrap_or(false)
    }
}

/// Step inside a symbol-linked docs flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsBrowserSymbolFlowStep {
    /// Result was opened in the docs-browser shell.
    OpenInBrowser,
    /// Result was peeked from a symbol.
    Peek,
    /// Result was opened in a split pane next to the source.
    Split,
    /// Result was handed off to the external browser through the handoff packet.
    BrowserHandoff,
    /// Result was preserved in a support export.
    SupportExport,
    /// Result was preserved as evidence in an AI handoff.
    AiHandoff,
}

impl DocsBrowserSymbolFlowStep {
    /// Every closed flow step in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::Peek,
        Self::Split,
        Self::BrowserHandoff,
        Self::SupportExport,
        Self::AiHandoff,
    ];

    /// Stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInBrowser => "open_in_browser",
            Self::Peek => "peek",
            Self::Split => "split",
            Self::BrowserHandoff => "browser_handoff",
            Self::SupportExport => "support_export",
            Self::AiHandoff => "ai_handoff",
        }
    }
}

/// Symbol-linked docs flow that preserves docs-result object identity through
/// every step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserSymbolFlow {
    /// Stable flow id.
    pub flow_id: String,
    /// Originating symbol id.
    pub origin_symbol_id: String,
    /// Linked docs-result object id (matches `DocsBrowserResultObject::result_id`).
    pub linked_result_id: String,
    /// Flow steps that preserved the docs-result object identity.
    pub steps_preserving_identity: Vec<DocsBrowserSymbolFlowStep>,
    /// True when the flow preserves the docs-source descriptor reference verbatim.
    pub preserves_docs_source_ref: bool,
    /// True when the flow preserves the symbol refs verbatim.
    pub preserves_symbol_refs: bool,
    /// True when the flow preserves the citation anchors verbatim.
    pub preserves_citation_anchors: bool,
    /// Capture timestamp for the flow record.
    pub captured_at: String,
}

impl DocsBrowserSymbolFlow {
    fn preserves_required_steps(&self) -> bool {
        let observed: BTreeSet<_> = self.steps_preserving_identity.iter().copied().collect();
        DocsBrowserSymbolFlowStep::REQUIRED
            .iter()
            .all(|step| observed.contains(step))
    }

    fn fully_preserves_identity(&self) -> bool {
        self.preserves_docs_source_ref
            && self.preserves_symbol_refs
            && self.preserves_citation_anchors
            && self.preserves_required_steps()
    }
}

/// Consumer projection proving a surface reads the same packet without
/// reinventing docs-browser truth locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: DocsBrowserConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub packet_id_ref: String,
    /// Render timestamp for the projection.
    pub rendered_at: String,
    /// True when the surface preserves the closed source-class taxonomy verbatim.
    pub preserves_source_class: bool,
    /// True when the surface preserves the closed version-match vocabulary verbatim.
    pub preserves_version_match: bool,
    /// True when the surface preserves the closed freshness vocabulary verbatim.
    pub preserves_freshness: bool,
    /// True when the surface preserves the trust class verbatim.
    pub preserves_trust_class: bool,
    /// True when the surface preserves the browser-handoff capability verbatim.
    pub preserves_browser_handoff: bool,
    /// True when the surface preserves the docs-result object identity verbatim.
    pub preserves_result_object_identity: bool,
    /// True when the surface preserves the symbol-flow identity verbatim.
    pub preserves_symbol_flow_identity: bool,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl DocsBrowserConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_source_class
            && self.preserves_version_match
            && self.preserves_freshness
            && self.preserves_trust_class
            && self.preserves_browser_handoff
            && self.preserves_result_object_identity
            && self.preserves_symbol_flow_identity
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`DocsBrowserTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id the packet describes.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Captured-vs-live status.
    pub captured_vs_live: DocsBrowserCapturedVsLive,
    /// Docs-browser source descriptors.
    #[serde(default)]
    pub sources: Vec<DocsBrowserSourceDescriptor>,
    /// Docs-browser result objects.
    #[serde(default)]
    pub results: Vec<DocsBrowserResultObject>,
    /// Symbol-linked docs flows preserving result-object identity.
    #[serde(default)]
    pub symbol_flows: Vec<DocsBrowserSymbolFlow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DocsBrowserConsumerProjection>,
    /// Source contract refs (docs / fixtures / schema / artifact) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Stable docs-browser truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id the packet describes.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Captured-vs-live status.
    pub captured_vs_live: DocsBrowserCapturedVsLive,
    /// Docs-browser source descriptors.
    #[serde(default)]
    pub sources: Vec<DocsBrowserSourceDescriptor>,
    /// Docs-browser result objects.
    #[serde(default)]
    pub results: Vec<DocsBrowserResultObject>,
    /// Symbol-linked docs flows preserving result-object identity.
    #[serde(default)]
    pub symbol_flows: Vec<DocsBrowserSymbolFlow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<DocsBrowserConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: DocsBrowserPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<DocsBrowserValidationFinding>,
}

impl DocsBrowserTruthPacket {
    /// Materialize a packet and record derived validation findings.
    pub fn materialize(input: DocsBrowserTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DOCS_BROWSER_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            captured_vs_live: input.captured_vs_live,
            sources: input.sources,
            results: input.results,
            symbol_flows: input.symbol_flows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: DocsBrowserPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validate the packet against stable docs-browser invariants.
    pub fn validate(&self) -> Vec<DocsBrowserValidationFinding> {
        self.derived_findings(true)
    }

    /// True when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == DocsBrowserFindingSeverity::Blocker)
    }

    /// True when at least one consumer projection preserves this packet for
    /// `surface`.
    pub fn has_projection_for(&self, surface: DocsBrowserConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique source-class tokens carried across result objects.
    pub fn source_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for source in &self.sources {
            set.insert(source.source_class);
        }
        set.into_iter()
            .map(DocsBrowserSourceClass::as_str)
            .collect()
    }

    /// Returns the unique version-match-state tokens carried across result
    /// objects.
    pub fn version_match_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for result in &self.results {
            set.insert(result.version_match_state);
        }
        set.into_iter()
            .map(DocsBrowserVersionMatchState::as_str)
            .collect()
    }

    /// Returns the unique freshness-state tokens carried across result objects.
    pub fn freshness_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for result in &self.results {
            set.insert(result.freshness_state);
        }
        set.into_iter()
            .map(DocsBrowserFreshnessState::as_str)
            .collect()
    }

    /// Returns the unique symbol-link-class tokens carried across result objects.
    pub fn symbol_link_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for result in &self.results {
            set.insert(result.symbol_link_class);
        }
        set.into_iter()
            .map(DocsBrowserSymbolLinkClass::as_str)
            .collect()
    }

    /// Build a support export wrapping the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DocsBrowserTruthSupportExport {
        DocsBrowserTruthSupportExport {
            record_kind: DOCS_BROWSER_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    fn source_id_set(&self) -> BTreeSet<&str> {
        self.sources.iter().map(|s| s.source_id.as_str()).collect()
    }

    fn result_id_set(&self) -> BTreeSet<&str> {
        self.results.iter().map(|r| r.result_id.as_str()).collect()
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<DocsBrowserValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != DOCS_BROWSER_TRUTH_PACKET_RECORD_KIND {
            findings.push(DocsBrowserValidationFinding::new(
                DocsBrowserFindingKind::WrongRecordKind,
                DocsBrowserFindingSeverity::Blocker,
                "docs-browser truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION
        {
            findings.push(DocsBrowserValidationFinding::new(
                DocsBrowserFindingKind::WrongSchemaVersion,
                DocsBrowserFindingSeverity::Blocker,
                "docs-browser truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(DocsBrowserValidationFinding::new(
                DocsBrowserFindingKind::MissingPacketIdentity,
                DocsBrowserFindingSeverity::Blocker,
                "packet id, workflow id, and capture timestamp are required",
            ));
        }
        if self.sources.is_empty() {
            findings.push(DocsBrowserValidationFinding::new(
                DocsBrowserFindingKind::MissingSourceDescriptors,
                DocsBrowserFindingSeverity::Blocker,
                "packet must declare at least one docs-browser source descriptor",
            ));
        }
        if self.results.is_empty() {
            findings.push(DocsBrowserValidationFinding::new(
                DocsBrowserFindingKind::MissingResultObjects,
                DocsBrowserFindingSeverity::Blocker,
                "packet must declare at least one docs-browser result object",
            ));
        }

        let source_set = self.source_id_set();
        let result_set = self.result_id_set();

        for source in &self.sources {
            if !source.is_complete() {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::SourceDescriptorIncomplete,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "source descriptor {} drops a required identity field",
                        source.source_id
                    ),
                ));
            }
            if source.browser_handoff.requires_handoff_packet_ref()
                && source
                    .browser_handoff_packet_ref
                    .as_deref()
                    .map(|value| value.trim().is_empty())
                    .unwrap_or(true)
            {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::BrowserHandoffPacketMissing,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "source descriptor {} declares available browser handoff but has no packet ref",
                        source.source_id
                    ),
                ));
            }
            if source.browser_handoff.requires_disclosure()
                && source
                    .policy_disclosure_note
                    .as_deref()
                    .map(|note| note.trim().is_empty())
                    .unwrap_or(true)
            {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::ResultDisclosureMissing,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "source descriptor {} has blocked/unavailable handoff but no disclosure note",
                        source.source_id
                    ),
                ));
            }
            if !source.raw_boundary_material_excluded {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::RawBoundaryMaterialPresent,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "source descriptor {} admits raw boundary material",
                        source.source_id
                    ),
                ));
            }
        }

        for result in &self.results {
            if result.result_id.trim().is_empty()
                || result.title.trim().is_empty()
                || result.docs_source_ref.trim().is_empty()
            {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::ResultIdentityIncomplete,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "result {} drops a required identity field",
                        result.result_id
                    ),
                ));
            }
            if !source_set.contains(result.docs_source_ref.as_str()) {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::ResultSourceRefUnpinned,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "result {} references unpinned docs-source ref {}",
                        result.result_id, result.docs_source_ref
                    ),
                ));
            }
            if result.symbol_link_class.requires_symbol_ref() && result.symbol_refs.is_empty() {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::SymbolRefsMissing,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "result {} declares symbol link {} but ships without symbol refs",
                        result.result_id,
                        result.symbol_link_class.as_str()
                    ),
                ));
            }
            if result.symbol_link_class.requires_disclosure()
                && result
                    .downgrade_note
                    .as_deref()
                    .map(|note| note.trim().is_empty())
                    .unwrap_or(true)
            {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::ResultDisclosureMissing,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "result {} declares symbol link {} but has no disclosure note",
                        result.result_id,
                        result.symbol_link_class.as_str()
                    ),
                ));
            }
            if result.requires_downgrade_note() && !result.has_downgrade_note() {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::ResultDisclosureMissing,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "result {} has non-stable version/freshness state but no downgrade note",
                        result.result_id
                    ),
                ));
            }
            if !result.raw_boundary_material_excluded {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::RawBoundaryMaterialPresent,
                    DocsBrowserFindingSeverity::Blocker,
                    format!("result {} admits raw boundary material", result.result_id),
                ));
            }

            // Derived-explanation rows must downgrade.
            if let Some(source) = self
                .sources
                .iter()
                .find(|source| source.source_id == result.docs_source_ref)
            {
                if (source.source_class.is_derived_only()
                    || source.trust_class == DocsBrowserTrustClass::DerivedInferenceOnly)
                    && !result.has_downgrade_note()
                {
                    findings.push(DocsBrowserValidationFinding::new(
                        DocsBrowserFindingKind::DerivedExplanationNotDowngraded,
                        DocsBrowserFindingSeverity::Blocker,
                        format!(
                            "result {} cites a derived-explanation source but has no downgrade note",
                            result.result_id
                        ),
                    ));
                }
            }
        }

        for flow in &self.symbol_flows {
            if flow.flow_id.trim().is_empty()
                || flow.origin_symbol_id.trim().is_empty()
                || flow.linked_result_id.trim().is_empty()
                || flow.captured_at.trim().is_empty()
            {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::SymbolFlowIdentityLost,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "symbol flow {} drops a required identity field",
                        flow.flow_id
                    ),
                ));
            }
            if !result_set.contains(flow.linked_result_id.as_str()) {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::SymbolFlowResultRefUnpinned,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "symbol flow {} references unpinned result {}",
                        flow.flow_id, flow.linked_result_id
                    ),
                ));
            }
            if !flow.fully_preserves_identity() {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::SymbolFlowIdentityLost,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "symbol flow {} drops docs-result identity through peek/split/handoff/export/AI",
                        flow.flow_id
                    ),
                ));
            }
        }

        // Required source-class coverage on stable claims.
        let observed_source_classes: BTreeSet<DocsBrowserSourceClass> = self
            .sources
            .iter()
            .map(|source| source.source_class)
            .collect();
        for required in DocsBrowserSourceClass::REQUIRED {
            if !observed_source_classes.contains(&required) {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::RequiredSourceClassCoverageMissing,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "packet does not cover required source class {}",
                        required.as_str()
                    ),
                ));
            }
        }

        for required_surface in DocsBrowserConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::MissingConsumerProjection,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::ConsumerProjectionDrift,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve docs-browser truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_source_class {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::SourceClassTaxonomyDropped,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the source-class taxonomy",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_version_match {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::VersionMatchVocabularyDropped,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the version-match vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_freshness {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::FreshnessVocabularyDropped,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the freshness vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_symbol_flow_identity {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::SymbolFlowIdentityDropped,
                    DocsBrowserFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the symbol-flow identity binding",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != DocsBrowserFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(DocsBrowserValidationFinding::new(
                    DocsBrowserFindingKind::PromotionStateMismatch,
                    DocsBrowserFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[DocsBrowserValidationFinding],
) -> DocsBrowserPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == DocsBrowserFindingSeverity::Blocker)
    {
        DocsBrowserPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == DocsBrowserFindingSeverity::Warning)
    {
        DocsBrowserPromotionState::NarrowedBelowStable
    } else {
        DocsBrowserPromotionState::Stable
    }
}

/// Support-export wrapper preserving the product docs-browser packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsBrowserTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export packet id preserved by the export.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub export_packet: DocsBrowserTruthPacket,
}

impl DocsBrowserTruthSupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DOCS_BROWSER_TRUTH_PACKET_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DOCS_BROWSER_TRUTH_PACKET_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable docs-browser packet.
#[derive(Debug)]
pub enum DocsBrowserTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<DocsBrowserValidationFinding>),
}

impl fmt::Display for DocsBrowserTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "docs-browser truth packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs-browser truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsBrowserTruthArtifactError {}

/// Returns the constructor input for the seeded stable docs-browser truth
/// packet. The same input is used by the in-process artifact function, by the
/// CLI emitter, and by the fixture-replay tests so that all three paths cannot
/// drift.
pub fn seeded_stable_docs_browser_truth_packet_input() -> DocsBrowserTruthPacketInput {
    seed::seeded_stable_input()
}

/// Materialize the checked-in stable docs-browser truth packet from the seed.
///
/// # Errors
///
/// Returns an artifact error if the materialized packet fails validation.
pub fn current_stable_docs_browser_truth_packet(
) -> Result<DocsBrowserTruthPacket, DocsBrowserTruthArtifactError> {
    let packet =
        DocsBrowserTruthPacket::materialize(seeded_stable_docs_browser_truth_packet_input());
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DocsBrowserTruthArtifactError::Validation(findings))
    }
}

mod seed {
    use super::*;

    pub(super) fn seeded_stable_input() -> DocsBrowserTruthPacketInput {
        let packet_id = "packet:m4:docs_browser_truth:stable".to_owned();
        let sources = vec![
            DocsBrowserSourceDescriptor {
                source_id: "src:project_docs:workspace".to_owned(),
                source_class: DocsBrowserSourceClass::ProjectDocs,
                provider_or_pack_id: "pack:project-docs:aureline-workspace".to_owned(),
                provider_or_pack_revision_ref: "rev:project-docs:aureline-workspace@2026.05.26"
                    .to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsBrowserTrustClass::FirstPartyAuthoritative,
                browser_handoff: DocsBrowserHandoffCapability::NotRequiredLocal,
                browser_handoff_packet_ref: None,
                policy_disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
            DocsBrowserSourceDescriptor {
                source_id: "src:mirrored:rust-std".to_owned(),
                source_class: DocsBrowserSourceClass::MirroredOfficialDocs,
                provider_or_pack_id: "pack:mirrored-official:rust-std".to_owned(),
                provider_or_pack_revision_ref: "rev:mirrored-official:rust-std@1.78.0".to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsBrowserTrustClass::SignedMirrorVerified,
                browser_handoff: DocsBrowserHandoffCapability::NotRequiredLocal,
                browser_handoff_packet_ref: None,
                policy_disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
            DocsBrowserSourceDescriptor {
                source_id: "src:extension_pack:python-stdlib".to_owned(),
                source_class: DocsBrowserSourceClass::ExtensionDocsPack,
                provider_or_pack_id: "pack:extension-docs:python-stdlib".to_owned(),
                provider_or_pack_revision_ref: "rev:extension-docs:python-stdlib@3.12.4".to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsBrowserTrustClass::ExtensionPackSigned,
                browser_handoff: DocsBrowserHandoffCapability::NotRequiredLocal,
                browser_handoff_packet_ref: None,
                policy_disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
            DocsBrowserSourceDescriptor {
                source_id: "src:live_external:vendor-portal".to_owned(),
                source_class: DocsBrowserSourceClass::LiveExternalDocs,
                provider_or_pack_id: "provider:vendor-portal".to_owned(),
                provider_or_pack_revision_ref: "rev:vendor-portal:beta-2026.05.25".to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsBrowserTrustClass::LiveProviderHandoff,
                browser_handoff: DocsBrowserHandoffCapability::AvailableExplicit,
                browser_handoff_packet_ref: Some(
                    "browser-handoff:vendor-portal:default".to_owned(),
                ),
                policy_disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
            DocsBrowserSourceDescriptor {
                source_id: "src:derived:explainer".to_owned(),
                source_class: DocsBrowserSourceClass::DerivedExplanation,
                provider_or_pack_id: "pack:derived-explainer:graph-summarizer".to_owned(),
                provider_or_pack_revision_ref: "rev:derived-explainer:graph-summarizer@2026.05.26"
                    .to_owned(),
                locale: "en-US".to_owned(),
                trust_class: DocsBrowserTrustClass::DerivedInferenceOnly,
                browser_handoff: DocsBrowserHandoffCapability::NotRequiredLocal,
                browser_handoff_packet_ref: None,
                policy_disclosure_note: None,
                raw_boundary_material_excluded: true,
            },
        ];

        let results = vec![
            DocsBrowserResultObject {
                result_id: "result:project_docs:contributing-overview".to_owned(),
                title: "Contributing overview".to_owned(),
                docs_source_ref: "src:project_docs:workspace".to_owned(),
                version_match_state: DocsBrowserVersionMatchState::ExactBuildMatch,
                freshness_state: DocsBrowserFreshnessState::AuthoritativeLive,
                symbol_link_class: DocsBrowserSymbolLinkClass::NotSymbolLinked,
                symbol_refs: Vec::new(),
                citation_anchors: vec![DocsBrowserCitationAnchor {
                    anchor_id: "anchor:project_docs:contributing-overview:summary".to_owned(),
                    target_ref: "docs-anchor:project_docs:contributing#summary".to_owned(),
                    anchor_class_token: "section".to_owned(),
                }],
                snippet_anchor_ref: Some(
                    "snippet:project_docs:contributing#summary".to_owned(),
                ),
                downgrade_note: None,
                raw_boundary_material_excluded: true,
            },
            DocsBrowserResultObject {
                result_id: "result:mirrored:vec-push".to_owned(),
                title: "Vec::push — Rust std reference".to_owned(),
                docs_source_ref: "src:mirrored:rust-std".to_owned(),
                version_match_state: DocsBrowserVersionMatchState::CompatibleMinorDrift,
                freshness_state: DocsBrowserFreshnessState::WarmCached,
                symbol_link_class: DocsBrowserSymbolLinkClass::SymbolAnchorAvailable,
                symbol_refs: vec![DocsBrowserSymbolRef {
                    symbol_id: "symbol:rust-std:Vec::push".to_owned(),
                    display_label: "Vec::push".to_owned(),
                    source_ref: "src:rust-std:vec.rs#push".to_owned(),
                    symbol_kind_token: "function".to_owned(),
                }],
                citation_anchors: vec![DocsBrowserCitationAnchor {
                    anchor_id: "anchor:mirrored:rust-std:vec-push".to_owned(),
                    target_ref: "docs-anchor:rust-std:Vec::push".to_owned(),
                    anchor_class_token: "api".to_owned(),
                }],
                snippet_anchor_ref: Some(
                    "snippet:rust-std:Vec::push#example".to_owned(),
                ),
                downgrade_note: Some(
                    "Mirrored docs are pinned to 1.78.0; workspace toolchain is 1.79.0 (compatible minor drift; cached today).".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            DocsBrowserResultObject {
                result_id: "result:extension_pack:list-comprehension".to_owned(),
                title: "List comprehensions — Python stdlib".to_owned(),
                docs_source_ref: "src:extension_pack:python-stdlib".to_owned(),
                version_match_state: DocsBrowserVersionMatchState::ExactBuildMatch,
                freshness_state: DocsBrowserFreshnessState::WarmCached,
                symbol_link_class: DocsBrowserSymbolLinkClass::NotSymbolLinked,
                symbol_refs: Vec::new(),
                citation_anchors: vec![DocsBrowserCitationAnchor {
                    anchor_id: "anchor:extension_pack:python-stdlib:list-comprehension".to_owned(),
                    target_ref: "docs-anchor:python-stdlib:list-comprehension".to_owned(),
                    anchor_class_token: "section".to_owned(),
                }],
                snippet_anchor_ref: Some(
                    "snippet:python-stdlib:list-comprehension#example".to_owned(),
                ),
                downgrade_note: Some(
                    "Extension-pack content is cached locally; the pack is signed.".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            DocsBrowserResultObject {
                result_id: "result:live_external:vendor-portal-quota".to_owned(),
                title: "Quota policy — vendor portal".to_owned(),
                docs_source_ref: "src:live_external:vendor-portal".to_owned(),
                version_match_state: DocsBrowserVersionMatchState::PreReleaseUnverified,
                freshness_state: DocsBrowserFreshnessState::Unverified,
                symbol_link_class: DocsBrowserSymbolLinkClass::SymbolAnchorPartial,
                symbol_refs: vec![DocsBrowserSymbolRef {
                    symbol_id: "symbol:vendor-portal:QuotaPolicy".to_owned(),
                    display_label: "QuotaPolicy".to_owned(),
                    source_ref: "src:vendor-portal:quota.rs#QuotaPolicy".to_owned(),
                    symbol_kind_token: "type".to_owned(),
                }],
                citation_anchors: vec![DocsBrowserCitationAnchor {
                    anchor_id: "anchor:vendor-portal:quota-policy".to_owned(),
                    target_ref: "docs-anchor:vendor-portal:quota-policy".to_owned(),
                    anchor_class_token: "heading".to_owned(),
                }],
                snippet_anchor_ref: None,
                downgrade_note: Some(
                    "Live external docs are unverified pre-release; open in browser for the canonical reference (partial symbol link).".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
            DocsBrowserResultObject {
                result_id: "result:derived:router-overview".to_owned(),
                title: "Router architecture explainer (derived)".to_owned(),
                docs_source_ref: "src:derived:explainer".to_owned(),
                version_match_state: DocsBrowserVersionMatchState::ExactBuildMatch,
                freshness_state: DocsBrowserFreshnessState::AuthoritativeLive,
                symbol_link_class: DocsBrowserSymbolLinkClass::SymbolAnchorMissingDisclosed,
                symbol_refs: Vec::new(),
                citation_anchors: vec![DocsBrowserCitationAnchor {
                    anchor_id: "anchor:derived:router-overview".to_owned(),
                    target_ref: "docs-anchor:derived-explainer:router-overview".to_owned(),
                    anchor_class_token: "section".to_owned(),
                }],
                snippet_anchor_ref: None,
                downgrade_note: Some(
                    "Derived explanation; not primary authority. Symbol link unavailable — open the cited graph view for grounded evidence.".to_owned(),
                ),
                raw_boundary_material_excluded: true,
            },
        ];

        let symbol_flows = vec![
            DocsBrowserSymbolFlow {
                flow_id: "flow:vec-push:peek-to-handoff".to_owned(),
                origin_symbol_id: "symbol:workspace:my_vec.push".to_owned(),
                linked_result_id: "result:mirrored:vec-push".to_owned(),
                steps_preserving_identity: DocsBrowserSymbolFlowStep::REQUIRED.to_vec(),
                preserves_docs_source_ref: true,
                preserves_symbol_refs: true,
                preserves_citation_anchors: true,
                captured_at: "2026-05-26T12:00:01Z".to_owned(),
            },
            DocsBrowserSymbolFlow {
                flow_id: "flow:vendor-portal:peek-to-handoff".to_owned(),
                origin_symbol_id: "symbol:workspace:quota_check".to_owned(),
                linked_result_id: "result:live_external:vendor-portal-quota".to_owned(),
                steps_preserving_identity: DocsBrowserSymbolFlowStep::REQUIRED.to_vec(),
                preserves_docs_source_ref: true,
                preserves_symbol_refs: true,
                preserves_citation_anchors: true,
                captured_at: "2026-05-26T12:00:02Z".to_owned(),
            },
        ];

        let consumer_projections = DocsBrowserConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| DocsBrowserConsumerProjection {
                consumer_surface: surface,
                projection_ref: format!("projection:docs_browser_truth:{}", surface.as_str()),
                packet_id_ref: packet_id.clone(),
                rendered_at: "2026-05-26T12:00:03Z".to_owned(),
                preserves_source_class: true,
                preserves_version_match: true,
                preserves_freshness: true,
                preserves_trust_class: true,
                preserves_browser_handoff: true,
                preserves_result_object_identity: true,
                preserves_symbol_flow_identity: true,
                preserves_same_packet: true,
                supports_json_export: true,
                raw_private_material_excluded: true,
                ambient_authority_excluded: true,
            })
            .collect();

        DocsBrowserTruthPacketInput {
            packet_id,
            workflow_or_surface_id: "workflow.docs.browser.stable".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            captured_vs_live: DocsBrowserCapturedVsLive::Live,
            sources,
            results,
            symbol_flows,
            consumer_projections,
            source_contract_refs: vec![
                DOCS_BROWSER_TRUTH_PACKET_DOC_REF.to_owned(),
                DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_DOC_REF.to_owned(),
                DOCS_BROWSER_TRUTH_PACKET_SCHEMA_REF.to_owned(),
                DOCS_BROWSER_TRUTH_PACKET_FIXTURE_DIR.to_owned(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_source(
        source_id: &str,
        source_class: DocsBrowserSourceClass,
        trust_class: DocsBrowserTrustClass,
        browser_handoff: DocsBrowserHandoffCapability,
    ) -> DocsBrowserSourceDescriptor {
        DocsBrowserSourceDescriptor {
            source_id: source_id.to_owned(),
            source_class,
            provider_or_pack_id: format!("provider:{source_id}"),
            provider_or_pack_revision_ref: format!("rev:{source_id}@stable"),
            locale: "en-US".to_owned(),
            trust_class,
            browser_handoff,
            browser_handoff_packet_ref: matches!(
                browser_handoff,
                DocsBrowserHandoffCapability::AvailableExplicit
            )
            .then(|| format!("handoff:{source_id}")),
            policy_disclosure_note: browser_handoff
                .requires_disclosure()
                .then(|| format!("Browser handoff disclosed for {source_id}")),
            raw_boundary_material_excluded: true,
        }
    }

    fn sample_result(
        result_id: &str,
        docs_source_ref: &str,
        version_match_state: DocsBrowserVersionMatchState,
        freshness_state: DocsBrowserFreshnessState,
        symbol_link_class: DocsBrowserSymbolLinkClass,
    ) -> DocsBrowserResultObject {
        let needs_symbol = symbol_link_class.requires_symbol_ref();
        let needs_note = symbol_link_class.requires_disclosure()
            || version_match_state.lowers_certainty()
            || freshness_state.lowers_certainty();
        DocsBrowserResultObject {
            result_id: result_id.to_owned(),
            title: format!("Title for {result_id}"),
            docs_source_ref: docs_source_ref.to_owned(),
            version_match_state,
            freshness_state,
            symbol_link_class,
            symbol_refs: needs_symbol
                .then(|| {
                    vec![DocsBrowserSymbolRef {
                        symbol_id: format!("symbol:{result_id}"),
                        display_label: format!("symbol_{result_id}"),
                        source_ref: format!("src:{result_id}#sym"),
                        symbol_kind_token: "function".to_owned(),
                    }]
                })
                .unwrap_or_default(),
            citation_anchors: vec![DocsBrowserCitationAnchor {
                anchor_id: format!("anchor:{result_id}"),
                target_ref: format!("docs:{result_id}#section"),
                anchor_class_token: "section".to_owned(),
            }],
            snippet_anchor_ref: Some(format!("snippet:{result_id}")),
            downgrade_note: needs_note.then(|| format!("Disclosure note for {result_id}")),
            raw_boundary_material_excluded: true,
        }
    }

    fn sample_symbol_flow(flow_id: &str, result_id: &str) -> DocsBrowserSymbolFlow {
        DocsBrowserSymbolFlow {
            flow_id: flow_id.to_owned(),
            origin_symbol_id: format!("symbol:{flow_id}"),
            linked_result_id: result_id.to_owned(),
            steps_preserving_identity: DocsBrowserSymbolFlowStep::REQUIRED.to_vec(),
            preserves_docs_source_ref: true,
            preserves_symbol_refs: true,
            preserves_citation_anchors: true,
            captured_at: "2026-05-26T12:00:01Z".to_owned(),
        }
    }

    fn sample_projection(
        surface: DocsBrowserConsumerSurface,
        packet_id: &str,
    ) -> DocsBrowserConsumerProjection {
        DocsBrowserConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            packet_id_ref: packet_id.to_owned(),
            rendered_at: "2026-05-26T12:00:02Z".to_owned(),
            preserves_source_class: true,
            preserves_version_match: true,
            preserves_freshness: true,
            preserves_trust_class: true,
            preserves_browser_handoff: true,
            preserves_result_object_identity: true,
            preserves_symbol_flow_identity: true,
            preserves_same_packet: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn baseline_input(packet_id: &str) -> DocsBrowserTruthPacketInput {
        let sources = vec![
            sample_source(
                "src:project_docs:workspace",
                DocsBrowserSourceClass::ProjectDocs,
                DocsBrowserTrustClass::FirstPartyAuthoritative,
                DocsBrowserHandoffCapability::NotRequiredLocal,
            ),
            sample_source(
                "src:mirrored:rust-std",
                DocsBrowserSourceClass::MirroredOfficialDocs,
                DocsBrowserTrustClass::SignedMirrorVerified,
                DocsBrowserHandoffCapability::NotRequiredLocal,
            ),
            sample_source(
                "src:extension_pack:python-stdlib",
                DocsBrowserSourceClass::ExtensionDocsPack,
                DocsBrowserTrustClass::ExtensionPackSigned,
                DocsBrowserHandoffCapability::NotRequiredLocal,
            ),
            sample_source(
                "src:live_external:vendor-portal",
                DocsBrowserSourceClass::LiveExternalDocs,
                DocsBrowserTrustClass::LiveProviderHandoff,
                DocsBrowserHandoffCapability::AvailableExplicit,
            ),
            sample_source(
                "src:derived:explainer",
                DocsBrowserSourceClass::DerivedExplanation,
                DocsBrowserTrustClass::DerivedInferenceOnly,
                DocsBrowserHandoffCapability::NotRequiredLocal,
            ),
        ];
        let results = vec![
            sample_result(
                "result:project_docs:overview",
                "src:project_docs:workspace",
                DocsBrowserVersionMatchState::ExactBuildMatch,
                DocsBrowserFreshnessState::AuthoritativeLive,
                DocsBrowserSymbolLinkClass::SymbolAnchorAvailable,
            ),
            sample_result(
                "result:mirrored:vec-push",
                "src:mirrored:rust-std",
                DocsBrowserVersionMatchState::CompatibleMinorDrift,
                DocsBrowserFreshnessState::WarmCached,
                DocsBrowserSymbolLinkClass::SymbolAnchorAvailable,
            ),
            sample_result(
                "result:extension_pack:list_comprehension",
                "src:extension_pack:python-stdlib",
                DocsBrowserVersionMatchState::ExactBuildMatch,
                DocsBrowserFreshnessState::WarmCached,
                DocsBrowserSymbolLinkClass::NotSymbolLinked,
            ),
            sample_result(
                "result:live_external:vendor-portal",
                "src:live_external:vendor-portal",
                DocsBrowserVersionMatchState::PreReleaseUnverified,
                DocsBrowserFreshnessState::Unverified,
                DocsBrowserSymbolLinkClass::SymbolAnchorPartial,
            ),
            sample_result(
                "result:derived:explainer",
                "src:derived:explainer",
                DocsBrowserVersionMatchState::ExactBuildMatch,
                DocsBrowserFreshnessState::AuthoritativeLive,
                DocsBrowserSymbolLinkClass::SymbolAnchorMissingDisclosed,
            ),
        ];
        let symbol_flows = vec![
            sample_symbol_flow("flow:vec_push", "result:mirrored:vec-push"),
            sample_symbol_flow("flow:project_overview", "result:project_docs:overview"),
        ];
        let consumer_projections = DocsBrowserConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| sample_projection(surface, packet_id))
            .collect();
        DocsBrowserTruthPacketInput {
            packet_id: packet_id.to_owned(),
            workflow_or_surface_id: "workflow.docs.browser.baseline".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            captured_vs_live: DocsBrowserCapturedVsLive::Live,
            sources,
            results,
            symbol_flows,
            consumer_projections,
            source_contract_refs: vec![DOCS_BROWSER_TRUTH_PACKET_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(DocsBrowserSourceClass::ProjectDocs.as_str(), "project_docs");
        assert_eq!(
            DocsBrowserSourceClass::MirroredOfficialDocs.as_str(),
            "mirrored_official_docs"
        );
        assert_eq!(
            DocsBrowserSourceClass::ExtensionDocsPack.as_str(),
            "extension_docs_pack"
        );
        assert_eq!(
            DocsBrowserSourceClass::LiveExternalDocs.as_str(),
            "live_external_docs"
        );
        assert_eq!(
            DocsBrowserSourceClass::DerivedExplanation.as_str(),
            "derived_explanation"
        );
        assert_eq!(
            DocsBrowserVersionMatchState::IncompatibleDriftDetected.as_str(),
            "incompatible_drift_detected"
        );
        assert_eq!(
            DocsBrowserFreshnessState::DegradedCached.as_str(),
            "degraded_cached"
        );
        assert_eq!(
            DocsBrowserSymbolLinkClass::SymbolAnchorMissingDisclosed.as_str(),
            "symbol_anchor_missing_disclosed"
        );
        assert_eq!(
            DocsBrowserConsumerSurface::BrowserHandoffPacket.as_str(),
            "browser_handoff_packet"
        );
        assert_eq!(
            DocsBrowserFindingKind::SymbolFlowIdentityLost.as_str(),
            "symbol_flow_identity_lost"
        );
    }

    #[test]
    fn baseline_packet_certifies_stable() {
        let packet = DocsBrowserTruthPacket::materialize(baseline_input(
            "packet:m4:docs_browser_truth:baseline",
        ));
        assert_eq!(
            packet.promotion_state,
            DocsBrowserPromotionState::Stable,
            "unexpected findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
    }

    #[test]
    fn missing_required_source_class_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_browser_truth:no_extension_pack");
        input
            .sources
            .retain(|source| source.source_class != DocsBrowserSourceClass::ExtensionDocsPack);
        // Also drop the dependent result so the result still references a known source.
        input
            .results
            .retain(|result| result.docs_source_ref != "src:extension_pack:python-stdlib");
        let packet = DocsBrowserTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DocsBrowserPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsBrowserFindingKind::RequiredSourceClassCoverageMissing
        }));
    }

    #[test]
    fn symbol_flow_dropping_split_step_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_browser_truth:flow_loses_split");
        if let Some(flow) = input.symbol_flows.first_mut() {
            flow.steps_preserving_identity
                .retain(|step| *step != DocsBrowserSymbolFlowStep::Split);
        }
        let packet = DocsBrowserTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DocsBrowserPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsBrowserFindingKind::SymbolFlowIdentityLost
        }));
    }

    #[test]
    fn result_referencing_unpinned_source_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_browser_truth:unpinned_source");
        if let Some(result) = input.results.first_mut() {
            result.docs_source_ref = "src:ghost".to_owned();
        }
        let packet = DocsBrowserTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DocsBrowserPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsBrowserFindingKind::ResultSourceRefUnpinned
        }));
    }

    #[test]
    fn live_external_handoff_without_packet_ref_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_browser_truth:missing_handoff_ref");
        if let Some(source) = input
            .sources
            .iter_mut()
            .find(|source| source.source_class == DocsBrowserSourceClass::LiveExternalDocs)
        {
            source.browser_handoff_packet_ref = None;
        }
        let packet = DocsBrowserTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DocsBrowserPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsBrowserFindingKind::BrowserHandoffPacketMissing
        }));
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_browser_truth:missing_projection");
        input.consumer_projections.pop();
        let packet = DocsBrowserTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DocsBrowserPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsBrowserFindingKind::MissingConsumerProjection
        }));
    }

    #[test]
    fn consumer_projection_dropping_source_class_blocks_stable() {
        let mut input =
            baseline_input("packet:m4:docs_browser_truth:projection_drops_source_class");
        if let Some(projection) = input.consumer_projections.first_mut() {
            projection.preserves_source_class = false;
        }
        let packet = DocsBrowserTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DocsBrowserPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsBrowserFindingKind::SourceClassTaxonomyDropped
        }));
    }

    #[test]
    fn derived_explanation_without_downgrade_blocks_stable() {
        let mut input = baseline_input("packet:m4:docs_browser_truth:derived_not_downgraded");
        if let Some(result) = input
            .results
            .iter_mut()
            .find(|result| result.result_id == "result:derived:explainer")
        {
            result.downgrade_note = None;
            result.symbol_link_class = DocsBrowserSymbolLinkClass::NotSymbolLinked;
        }
        let packet = DocsBrowserTruthPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            DocsBrowserPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == DocsBrowserFindingKind::DerivedExplanationNotDowngraded
        }));
    }
}
