//! Stable docs-source, result, pack-manifest, and derived-citation contract.
//!
//! This module binds the docs browser, help cards, onboarding and learning
//! surfaces, AI explainers, and support exports to one metadata contract. It
//! reuses the canonical source-class, version-match, freshness, mirror/offline,
//! docs-pack, stale-example, render, validation, citation, and precedence
//! vocabularies already owned by this crate.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    CitationInferenceMarker, CitationSourceClass, DocsExternalOpenState, DocsFreshnessClass,
    DocsMirrorOfflinePosture, DocsPackLocalAvailability, DocsPackManifest, DocsPackPinState,
    DocsRenderMode, DocsValidationResultClass, SourcePrecedenceClass, StaleExampleFindingClass,
    VersionMatchState,
};

/// Stable record-kind tag for [`StableDocsSourceResultPackCitationPacket`].
pub const STABLE_DOCS_CONTRACT_RECORD_KIND: &str = "stable_docs_source_result_pack_citation_packet";

/// Stable record-kind tag for [`StableDocsSupportExport`].
pub const STABLE_DOCS_CONTRACT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stable_docs_source_result_pack_citation_support_export";

/// Integer schema version for the stable docs-source/result/pack/citation packet.
pub const STABLE_DOCS_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the human contract note.
pub const STABLE_DOCS_CONTRACT_DOC_REF: &str =
    "docs/contracts/stable_docs_source_result_pack_and_citation.md";

/// Repo-relative path of the artifact narrative.
pub const STABLE_DOCS_CONTRACT_ARTIFACT_DOC_REF: &str =
    "artifacts/docs/stable_docs_source_result_pack_and_citation.md";

/// Repo-relative path of the JSON schema placeholder.
pub const STABLE_DOCS_CONTRACT_SCHEMA_REF: &str =
    "schemas/docs/stable_docs_source_result_pack_and_citation.schema.json";

/// Repo-relative fixture corpus directory.
pub const STABLE_DOCS_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/docs/stable_docs_source_result_pack_and_citation";

/// Repo-relative path of the checked-in stable packet artifact.
pub const STABLE_DOCS_CONTRACT_ARTIFACT_REF: &str =
    "artifacts/docs/stable_docs_source_result_pack_and_citation.json";

/// Consumer surface that must reuse this packet without local badge vocabularies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDocsConsumerSurface {
    /// Docs browser result list and source detail.
    DocsBrowser,
    /// Help and About cards.
    HelpAbout,
    /// Onboarding and guided-tour steps.
    Onboarding,
    /// AI explanation and context-inspector packets.
    AiExplainer,
    /// Support bundle export.
    SupportExport,
    /// Extension or help API.
    ExtensionHelpApi,
    /// Docs-pack and knowledge-pack detail sheets.
    PackDetailSheet,
    /// Citation drawer.
    CitationDrawer,
}

impl StableDocsConsumerSurface {
    /// Every required stable consumer surface.
    pub const REQUIRED: [Self; 8] = [
        Self::DocsBrowser,
        Self::HelpAbout,
        Self::Onboarding,
        Self::AiExplainer,
        Self::SupportExport,
        Self::ExtensionHelpApi,
        Self::PackDetailSheet,
        Self::CitationDrawer,
    ];

    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::HelpAbout => "help_about",
            Self::Onboarding => "onboarding",
            Self::AiExplainer => "ai_explainer",
            Self::SupportExport => "support_export",
            Self::ExtensionHelpApi => "extension_help_api",
            Self::PackDetailSheet => "pack_detail_sheet",
            Self::CitationDrawer => "citation_drawer",
        }
    }
}

/// Trust and support class displayed by pack detail sheets and exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDocsSupportTrustClass {
    /// First-party project or product source.
    FirstPartyAuthoritative,
    /// Signed mirror of an upstream source.
    SignedMirrorVerified,
    /// Curated pack with declared owner support.
    CuratedSupported,
    /// Extension or community pack with limited support.
    CommunitySupported,
    /// Online-only provider handoff.
    OnlineOnlyProvider,
    /// Derived explanation, never primary authority.
    DerivedInferenceOnly,
}

impl StableDocsSupportTrustClass {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyAuthoritative => "first_party_authoritative",
            Self::SignedMirrorVerified => "signed_mirror_verified",
            Self::CuratedSupported => "curated_supported",
            Self::CommunitySupported => "community_supported",
            Self::OnlineOnlyProvider => "online_only_provider",
            Self::DerivedInferenceOnly => "derived_inference_only",
        }
    }
}

/// Whether a detail sheet describes a docs pack or knowledge pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDocsPackDetailSheetKind {
    /// Documentation pack detail sheet.
    DocsPack,
    /// Knowledge-pack detail sheet.
    KnowledgePack,
}

impl StableDocsPackDetailSheetKind {
    /// Every detail sheet kind required by stable projections.
    pub const REQUIRED: [Self; 2] = [Self::DocsPack, Self::KnowledgePack];

    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPack => "docs_pack",
            Self::KnowledgePack => "knowledge_pack",
        }
    }
}

/// Export posture for derived citation sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableExportPosture {
    /// Export carries stable references only and omits raw pack bodies.
    ReferenceOnlyDefault,
    /// Export may include local snippets after explicit policy review.
    SnippetsAfterReview,
    /// Export is blocked by policy.
    BlockedByPolicy,
}

impl StableExportPosture {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReferenceOnlyDefault => "reference_only_default",
            Self::SnippetsAfterReview => "snippets_after_review",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }
}

/// Docs-source descriptor reused by docs/help/onboarding/AI/support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDocsSourceDescriptor {
    /// Stable source id used by results and citations.
    pub source_id: String,
    /// Canonical source class.
    pub source_class: CitationSourceClass,
    /// Provider or pack id.
    pub provider_or_pack_id: String,
    /// Provider or pack revision.
    pub provider_or_pack_revision_ref: String,
    /// Owner shown by detail sheets and exports.
    pub owner_ref: String,
    /// BCP-47 locale.
    pub locale: String,
    /// Trust/support class.
    pub trust_support_class: StableDocsSupportTrustClass,
    /// Browser or external-open posture.
    pub browser_handoff: DocsExternalOpenState,
    /// Handoff packet when external open is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Mirror/offline posture.
    pub mirror_offline_posture: DocsMirrorOfflinePosture,
    /// Precedence class used when multiple sources answer the same subject.
    pub precedence_class: SourcePrecedenceClass,
    /// Version-match state at mint time.
    pub version_match_state: VersionMatchState,
    /// Freshness state at mint time.
    pub freshness_state: DocsFreshnessClass,
    /// Pack manifest ref when the source resolves through a pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_manifest_ref: Option<String>,
    /// Disclosure note for derived, stale, handoff, mirror, or omitted-source posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
    /// True when raw bodies, raw URLs, secrets, and provider payloads are excluded.
    pub raw_boundary_material_excluded: bool,
}

impl StableDocsSourceDescriptor {
    fn is_well_formed(&self) -> bool {
        !self.source_id.trim().is_empty()
            && !self.provider_or_pack_id.trim().is_empty()
            && !self.provider_or_pack_revision_ref.trim().is_empty()
            && !self.owner_ref.trim().is_empty()
            && !self.locale.trim().is_empty()
    }

    fn requires_disclosure(&self) -> bool {
        self.browser_handoff.requires_disclosure()
            || self.freshness_state.lowers_certainty()
            || self.version_match_state != VersionMatchState::ExactBuildMatch
            || self.source_class == CitationSourceClass::DerivedExplanation
            || self.precedence_class == SourcePrecedenceClass::ProjectVendorDisagreementInspectable
    }

    fn has_disclosure(&self) -> bool {
        self.disclosure_note
            .as_deref()
            .map(|note| !note.trim().is_empty())
            .unwrap_or(false)
    }
}

/// Docs-result object reused by docs/help/onboarding/AI/support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDocsResultObject {
    /// Stable result id.
    pub result_id: String,
    /// User-visible title.
    pub title: String,
    /// Docs-source ref.
    pub docs_source_ref: String,
    /// Version-match state.
    pub version_match_state: VersionMatchState,
    /// Freshness state.
    pub freshness_state: DocsFreshnessClass,
    /// Symbol refs where present.
    #[serde(default)]
    pub symbol_refs: Vec<String>,
    /// Citation or docs anchor refs.
    #[serde(default)]
    pub citation_anchor_refs: Vec<String>,
    /// Snippet anchor for preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet_anchor_ref: Option<String>,
    /// Docs render config consumed by this result.
    pub docs_render_config_ref: String,
    /// Docs suggestion refs bound to this result.
    #[serde(default)]
    pub docs_suggestion_refs: Vec<String>,
    /// Docs validation result refs bound to this result.
    #[serde(default)]
    pub docs_validation_result_refs: Vec<String>,
    /// Precedence class retained by the result row.
    pub precedence_class: SourcePrecedenceClass,
    /// Omitted source markers shown in drawers and exports.
    #[serde(default)]
    pub omitted_source_markers: Vec<String>,
    /// Inference markers shown in drawers and exports.
    #[serde(default)]
    pub inference_markers: Vec<CitationInferenceMarker>,
    /// True when raw boundary material is excluded.
    pub raw_boundary_material_excluded: bool,
}

impl StableDocsResultObject {
    fn is_well_formed(&self) -> bool {
        !self.result_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.docs_source_ref.trim().is_empty()
            && !self.docs_render_config_ref.trim().is_empty()
            && !self.citation_anchor_refs.is_empty()
            && !self.docs_validation_result_refs.is_empty()
    }
}

/// Actions exposed by docs-pack and knowledge-pack detail sheets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StablePackActionSet {
    /// Stable update action ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_action_ref: Option<String>,
    /// Stable remove action ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_action_ref: Option<String>,
    /// Stable offline-availability action ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_action_ref: Option<String>,
    /// Stable citation-view action ref.
    pub view_citations_action_ref: String,
}

impl StablePackActionSet {
    fn is_well_formed(&self, availability: DocsPackLocalAvailability) -> bool {
        !self.view_citations_action_ref.trim().is_empty()
            && self
                .remove_action_ref
                .as_deref()
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false)
            && (availability == DocsPackLocalAvailability::MirrorOfflinePinned
                || self
                    .offline_action_ref
                    .as_deref()
                    .map(|value| !value.trim().is_empty())
                    .unwrap_or(false))
    }
}

/// Detail sheet for docs packs and knowledge packs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDocsPackDetailSheet {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Detail sheet kind.
    pub sheet_kind: StableDocsPackDetailSheetKind,
    /// Pack manifest id.
    pub pack_id_ref: String,
    /// Owner shown to the user.
    pub owner_ref: String,
    /// Version or revision shown to the user.
    pub version_ref: String,
    /// Locale coverage refs.
    pub locale_coverage_refs: Vec<String>,
    /// Trust/support class.
    pub trust_support_class: StableDocsSupportTrustClass,
    /// Pin state.
    pub pin_state: DocsPackPinState,
    /// Offline/local availability.
    pub local_availability: DocsPackLocalAvailability,
    /// Detail-sheet actions.
    pub actions: StablePackActionSet,
    /// Browser or external-open state.
    pub browser_handoff: DocsExternalOpenState,
    /// True when the source is online-only.
    pub online_only: bool,
    /// Disclosure note for online-only, unavailable, stale, or quarantined state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
}

impl StableDocsPackDetailSheet {
    fn is_well_formed(&self) -> bool {
        !self.sheet_id.trim().is_empty()
            && !self.pack_id_ref.trim().is_empty()
            && !self.owner_ref.trim().is_empty()
            && !self.version_ref.trim().is_empty()
            && !self.locale_coverage_refs.is_empty()
            && self.actions.is_well_formed(self.local_availability)
    }

    fn requires_disclosure(&self) -> bool {
        self.online_only
            || self.browser_handoff.requires_disclosure()
            || matches!(
                self.local_availability,
                DocsPackLocalAvailability::UnavailableDisclosed
                    | DocsPackLocalAvailability::NotInstalled
                    | DocsPackLocalAvailability::Quarantined
            )
    }

    fn has_disclosure(&self) -> bool {
        self.disclosure_note
            .as_deref()
            .map(|note| !note.trim().is_empty())
            .unwrap_or(false)
    }
}

/// Derived citation set exported without bundling full docs packs by default.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDerivedCitationSet {
    /// Stable citation-set id.
    pub citation_set_id: String,
    /// Derived explanation, glossary, or tour step ref.
    pub derived_object_ref: String,
    /// Pack manifest ids the citations resolve through.
    pub source_pack_id_refs: Vec<String>,
    /// Cited files.
    pub cited_file_refs: Vec<String>,
    /// Cited symbols.
    pub cited_symbol_refs: Vec<String>,
    /// Cited docs refs.
    pub cited_docs_refs: Vec<String>,
    /// Graph epoch.
    pub graph_epoch_ref: String,
    /// Locale.
    pub locale: String,
    /// Derivation tool ref.
    pub derivation_tool_ref: String,
    /// Derivation tool version.
    pub derivation_tool_version: String,
    /// Export posture.
    pub export_posture: StableExportPosture,
    /// Omitted-source markers.
    #[serde(default)]
    pub omitted_source_markers: Vec<String>,
    /// Inference markers.
    #[serde(default)]
    pub inference_markers: Vec<CitationInferenceMarker>,
    /// True when raw pack bodies are excluded.
    pub raw_pack_bodies_excluded: bool,
    /// True when raw URLs are excluded.
    pub raw_urls_excluded: bool,
}

impl StableDerivedCitationSet {
    fn is_well_formed(&self) -> bool {
        !self.citation_set_id.trim().is_empty()
            && !self.derived_object_ref.trim().is_empty()
            && !self.source_pack_id_refs.is_empty()
            && (!self.cited_file_refs.is_empty()
                || !self.cited_symbol_refs.is_empty()
                || !self.cited_docs_refs.is_empty())
            && !self.graph_epoch_ref.trim().is_empty()
            && !self.locale.trim().is_empty()
            && !self.derivation_tool_ref.trim().is_empty()
            && !self.derivation_tool_version.trim().is_empty()
            && !self.inference_markers.is_empty()
    }
}

/// Citation drawer parity record for a consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableCitationDrawerParity {
    /// Consumer surface.
    pub consumer_surface: StableDocsConsumerSurface,
    /// Stable drawer id.
    pub drawer_id: String,
    /// Result ref shown by the drawer.
    pub result_id_ref: String,
    /// Citation set ref shown by the drawer.
    pub citation_set_id_ref: String,
    /// Supporting files retained by the drawer.
    pub supporting_file_refs: Vec<String>,
    /// Supporting symbols retained by the drawer.
    pub supporting_symbol_refs: Vec<String>,
    /// Supporting docs anchors retained by the drawer.
    pub supporting_docs_refs: Vec<String>,
    /// Omitted-source markers retained by the drawer.
    pub omitted_source_markers: Vec<String>,
    /// Inference markers retained by the drawer.
    pub inference_markers: Vec<CitationInferenceMarker>,
    /// True when source/result anchors are preserved.
    pub preserves_supporting_anchors: bool,
    /// True when the drawer can be exported.
    pub exportable_without_raw_pack: bool,
}

impl StableCitationDrawerParity {
    fn preserves_truth(&self) -> bool {
        !self.drawer_id.trim().is_empty()
            && !self.result_id_ref.trim().is_empty()
            && !self.citation_set_id_ref.trim().is_empty()
            && (!self.supporting_file_refs.is_empty()
                || !self.supporting_symbol_refs.is_empty()
                || !self.supporting_docs_refs.is_empty())
            && !self.inference_markers.is_empty()
            && self.preserves_supporting_anchors
            && self.exportable_without_raw_pack
    }
}

/// Consumer projection proving a surface reused this contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDocsConsumerProjection {
    /// Consumer surface.
    pub consumer_surface: StableDocsConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub packet_id_ref: String,
    /// True when the source descriptor is preserved.
    pub preserves_source_descriptor: bool,
    /// True when the result object is preserved.
    pub preserves_result_object: bool,
    /// True when docs-pack manifests are preserved.
    pub preserves_pack_manifest: bool,
    /// True when version-match enums are reused.
    pub preserves_version_match: bool,
    /// True when freshness enums are reused.
    pub preserves_freshness: bool,
    /// True when source precedence is preserved.
    pub preserves_source_precedence: bool,
    /// True when detail-sheet owner/version/actions are preserved.
    pub preserves_detail_sheet: bool,
    /// True when citation identity is preserved.
    pub preserves_citation_basis: bool,
    /// True when browser-handoff and publish-boundary state is preserved.
    pub preserves_browser_handoff: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl StableDocsConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && !self.projection_ref.trim().is_empty()
            && self.preserves_source_descriptor
            && self.preserves_result_object
            && self.preserves_pack_manifest
            && self.preserves_version_match
            && self.preserves_freshness
            && self.preserves_source_precedence
            && self.preserves_detail_sheet
            && self.preserves_citation_basis
            && self.preserves_browser_handoff
            && self.raw_private_material_excluded
    }
}

/// Promotion state for the stable docs contract packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDocsPromotionState {
    /// Packet certifies the stable claim.
    Stable,
    /// Packet must remain narrowed below stable.
    NarrowedBelowStable,
    /// Packet blocks stable publication.
    BlocksStable,
}

impl StableDocsPromotionState {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one stable docs contract finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDocsFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding.
    Warning,
    /// Blocker finding.
    Blocker,
}

/// Finding vocabulary for the stable docs contract packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDocsFindingKind {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity is incomplete.
    MissingPacketIdentity,
    /// A source descriptor is incomplete.
    SourceDescriptorIncomplete,
    /// A result object is incomplete.
    ResultObjectIncomplete,
    /// A result references an unpinned source.
    ResultSourceRefUnpinned,
    /// Source/result version or freshness disagrees.
    SourceResultTruthMismatch,
    /// Render, suggestion, or validation binding is missing.
    RenderSuggestionValidationBindingMissing,
    /// Source precedence coverage is missing or reordered.
    SourcePrecedenceCoverageMissing,
    /// Pack detail sheet is incomplete.
    PackDetailSheetIncomplete,
    /// Pack detail sheet references an unpinned manifest.
    PackDetailSheetManifestRefUnpinned,
    /// Pack detail sheet action or visibility is missing.
    PackOwnerVersionActionVisibilityMissing,
    /// Citation set is incomplete.
    CitationSetIdentityIncomplete,
    /// Citation set references an unpinned pack.
    CitationSetPackRefUnpinned,
    /// Citation export bundles raw pack bodies by default.
    CitationSetBundlesRawPack,
    /// Citation drawer dropped supporting anchors, omitted-source, or inference markers.
    CitationDrawerParityDropped,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection dropped contract truth.
    ConsumerProjectionDrift,
    /// Mirror/offline or handoff disclosure is missing.
    BoundaryDisclosureMissing,
    /// Raw boundary material is present.
    RawBoundaryMaterialPresent,
    /// Required version, freshness, stale, or pack postures are not represented.
    RequiredDrillCoverageMissing,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl StableDocsFindingKind {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::SourceDescriptorIncomplete => "source_descriptor_incomplete",
            Self::ResultObjectIncomplete => "result_object_incomplete",
            Self::ResultSourceRefUnpinned => "result_source_ref_unpinned",
            Self::SourceResultTruthMismatch => "source_result_truth_mismatch",
            Self::RenderSuggestionValidationBindingMissing => {
                "render_suggestion_validation_binding_missing"
            }
            Self::SourcePrecedenceCoverageMissing => "source_precedence_coverage_missing",
            Self::PackDetailSheetIncomplete => "pack_detail_sheet_incomplete",
            Self::PackDetailSheetManifestRefUnpinned => "pack_detail_sheet_manifest_ref_unpinned",
            Self::PackOwnerVersionActionVisibilityMissing => {
                "pack_owner_version_action_visibility_missing"
            }
            Self::CitationSetIdentityIncomplete => "citation_set_identity_incomplete",
            Self::CitationSetPackRefUnpinned => "citation_set_pack_ref_unpinned",
            Self::CitationSetBundlesRawPack => "citation_set_bundles_raw_pack",
            Self::CitationDrawerParityDropped => "citation_drawer_parity_dropped",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::BoundaryDisclosureMissing => "boundary_disclosure_missing",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::RequiredDrillCoverageMissing => "required_drill_coverage_missing",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDocsValidationFinding {
    /// Finding kind.
    pub finding_kind: StableDocsFindingKind,
    /// Severity.
    pub severity: StableDocsFindingSeverity,
    /// Support-safe summary.
    pub summary: String,
}

impl StableDocsValidationFinding {
    fn new(
        finding_kind: StableDocsFindingKind,
        severity: StableDocsFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`StableDocsSourceResultPackCitationPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDocsSourceResultPackCitationInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Source descriptors.
    pub sources: Vec<StableDocsSourceDescriptor>,
    /// Result objects.
    pub results: Vec<StableDocsResultObject>,
    /// Docs-pack manifests.
    pub pack_manifests: Vec<DocsPackManifest>,
    /// Docs-pack and knowledge-pack detail sheets.
    pub pack_detail_sheets: Vec<StableDocsPackDetailSheet>,
    /// Derived citation sets.
    pub derived_citation_sets: Vec<StableDerivedCitationSet>,
    /// Citation drawer parity rows.
    pub citation_drawers: Vec<StableCitationDrawerParity>,
    /// Consumer projections.
    pub consumer_projections: Vec<StableDocsConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Stale-example drill classes covered by the packet.
    pub stale_example_drill_classes: Vec<StaleExampleFindingClass>,
    /// Render modes covered by the packet.
    pub render_modes: Vec<DocsRenderMode>,
    /// Validation result classes covered by the packet.
    pub validation_result_classes: Vec<DocsValidationResultClass>,
}

/// Stable source/result/pack/citation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDocsSourceResultPackCitationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Source descriptors.
    pub sources: Vec<StableDocsSourceDescriptor>,
    /// Result objects.
    pub results: Vec<StableDocsResultObject>,
    /// Docs-pack manifests.
    pub pack_manifests: Vec<DocsPackManifest>,
    /// Docs-pack and knowledge-pack detail sheets.
    pub pack_detail_sheets: Vec<StableDocsPackDetailSheet>,
    /// Derived citation sets.
    pub derived_citation_sets: Vec<StableDerivedCitationSet>,
    /// Citation drawer parity rows.
    pub citation_drawers: Vec<StableCitationDrawerParity>,
    /// Consumer projections.
    pub consumer_projections: Vec<StableDocsConsumerProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Stale-example drill classes.
    pub stale_example_drill_classes: Vec<StaleExampleFindingClass>,
    /// Render modes.
    pub render_modes: Vec<DocsRenderMode>,
    /// Validation result classes.
    pub validation_result_classes: Vec<DocsValidationResultClass>,
    /// Derived promotion state.
    pub promotion_state: StableDocsPromotionState,
    /// Validation findings.
    #[serde(default)]
    pub validation_findings: Vec<StableDocsValidationFinding>,
}

impl StableDocsSourceResultPackCitationPacket {
    /// Materializes the packet and records validation findings.
    pub fn materialize(input: StableDocsSourceResultPackCitationInput) -> Self {
        let mut packet = Self {
            record_kind: STABLE_DOCS_CONTRACT_RECORD_KIND.to_owned(),
            schema_version: STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            sources: input.sources,
            results: input.results,
            pack_manifests: input.pack_manifests,
            pack_detail_sheets: input.pack_detail_sheets,
            derived_citation_sets: input.derived_citation_sets,
            citation_drawers: input.citation_drawers,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            stale_example_drill_classes: input.stale_example_drill_classes,
            render_modes: input.render_modes,
            validation_result_classes: input.validation_result_classes,
            promotion_state: StableDocsPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates stable invariants.
    pub fn validate(&self) -> Vec<StableDocsValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker findings exist.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == StableDocsFindingSeverity::Blocker)
    }

    /// Returns true when a surface preserves this packet.
    pub fn has_projection_for(&self, surface: StableDocsConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Builds a support export preserving the packet verbatim.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> StableDocsSupportExport {
        StableDocsSupportExport {
            record_kind: STABLE_DOCS_CONTRACT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: STABLE_DOCS_CONTRACT_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    fn source_id_set(&self) -> BTreeSet<&str> {
        self.sources
            .iter()
            .map(|source| source.source_id.as_str())
            .collect()
    }

    fn pack_id_set(&self) -> BTreeSet<&str> {
        self.pack_manifests
            .iter()
            .map(|manifest| manifest.pack_id.as_str())
            .collect()
    }

    fn result_id_set(&self) -> BTreeSet<&str> {
        self.results
            .iter()
            .map(|result| result.result_id.as_str())
            .collect()
    }

    fn citation_set_id_set(&self) -> BTreeSet<&str> {
        self.derived_citation_sets
            .iter()
            .map(|set| set.citation_set_id.as_str())
            .collect()
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<StableDocsValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != STABLE_DOCS_CONTRACT_RECORD_KIND {
            findings.push(StableDocsValidationFinding::new(
                StableDocsFindingKind::WrongRecordKind,
                StableDocsFindingSeverity::Blocker,
                "stable docs packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != STABLE_DOCS_CONTRACT_SCHEMA_VERSION {
            findings.push(StableDocsValidationFinding::new(
                StableDocsFindingKind::WrongSchemaVersion,
                StableDocsFindingSeverity::Blocker,
                "stable docs packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(StableDocsValidationFinding::new(
                StableDocsFindingKind::MissingPacketIdentity,
                StableDocsFindingSeverity::Blocker,
                "packet id, workflow id, and generated timestamp are required",
            ));
        }

        let source_ids = self.source_id_set();
        let pack_ids = self.pack_id_set();
        let result_ids = self.result_id_set();
        let citation_set_ids = self.citation_set_id_set();

        for source in &self.sources {
            if !source.is_well_formed() {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::SourceDescriptorIncomplete,
                    StableDocsFindingSeverity::Blocker,
                    format!("source descriptor {} is incomplete", source.source_id),
                ));
            }
            if source.browser_handoff == DocsExternalOpenState::Available
                && source
                    .browser_handoff_packet_ref
                    .as_deref()
                    .map_or(true, |value| value.trim().is_empty())
            {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::BoundaryDisclosureMissing,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "source descriptor {} is missing its browser handoff packet",
                        source.source_id
                    ),
                ));
            }
            if source.requires_disclosure() && !source.has_disclosure() {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::BoundaryDisclosureMissing,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "source descriptor {} needs a disclosure note",
                        source.source_id
                    ),
                ));
            }
            if !source.raw_boundary_material_excluded {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::RawBoundaryMaterialPresent,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "source descriptor {} admits raw boundary material",
                        source.source_id
                    ),
                ));
            }
        }

        for result in &self.results {
            if !result.is_well_formed() {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::ResultObjectIncomplete,
                    StableDocsFindingSeverity::Blocker,
                    format!("result object {} is incomplete", result.result_id),
                ));
            }
            if !source_ids.contains(result.docs_source_ref.as_str()) {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::ResultSourceRefUnpinned,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "result {} references an unpinned docs source",
                        result.result_id
                    ),
                ));
            }
            if result.docs_suggestion_refs.is_empty()
                || result.docs_validation_result_refs.is_empty()
                || result.docs_render_config_ref.trim().is_empty()
            {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::RenderSuggestionValidationBindingMissing,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "result {} is not bound to render/suggestion/validation refs",
                        result.result_id
                    ),
                ));
            }
            if let Some(source) = self
                .sources
                .iter()
                .find(|source| source.source_id == result.docs_source_ref)
            {
                if source.version_match_state != result.version_match_state
                    || source.freshness_state != result.freshness_state
                    || source.precedence_class != result.precedence_class
                {
                    findings.push(StableDocsValidationFinding::new(
                        StableDocsFindingKind::SourceResultTruthMismatch,
                        StableDocsFindingSeverity::Blocker,
                        format!(
                            "result {} drifted from its source descriptor truth",
                            result.result_id
                        ),
                    ));
                }
            }
            if !result.raw_boundary_material_excluded {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::RawBoundaryMaterialPresent,
                    StableDocsFindingSeverity::Blocker,
                    format!("result {} admits raw boundary material", result.result_id),
                ));
            }
        }

        let source_classes: Vec<CitationSourceClass> = self
            .sources
            .iter()
            .map(|source| source.source_class)
            .collect();
        for required in [
            CitationSourceClass::ProjectDocs,
            CitationSourceClass::GeneratedReference,
            CitationSourceClass::MirroredOfficialDocs,
            CitationSourceClass::CuratedKnowledgePack,
        ] {
            if !source_classes.contains(&required) {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::SourcePrecedenceCoverageMissing,
                    StableDocsFindingSeverity::Blocker,
                    format!("packet does not cover source class {}", required.as_str()),
                ));
            }
        }
        if !self.sources.iter().any(|source| {
            source.precedence_class == SourcePrecedenceClass::ProjectOutranksVendorDefault
        }) {
            findings.push(StableDocsValidationFinding::new(
                StableDocsFindingKind::SourcePrecedenceCoverageMissing,
                StableDocsFindingSeverity::Blocker,
                "packet does not prove project docs outrank vendor docs",
            ));
        }

        for sheet in &self.pack_detail_sheets {
            if !sheet.is_well_formed() {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::PackDetailSheetIncomplete,
                    StableDocsFindingSeverity::Blocker,
                    format!("pack detail sheet {} is incomplete", sheet.sheet_id),
                ));
            }
            if !pack_ids.contains(sheet.pack_id_ref.as_str()) {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::PackDetailSheetManifestRefUnpinned,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "pack detail sheet {} references an unpinned manifest",
                        sheet.sheet_id
                    ),
                ));
            }
            if sheet.requires_disclosure() && !sheet.has_disclosure() {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::BoundaryDisclosureMissing,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "pack detail sheet {} needs a disclosure note",
                        sheet.sheet_id
                    ),
                ));
            }
            if sheet.owner_ref.trim().is_empty()
                || sheet.version_ref.trim().is_empty()
                || !sheet.actions.is_well_formed(sheet.local_availability)
            {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::PackOwnerVersionActionVisibilityMissing,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "pack detail sheet {} hides owner, version, or required actions",
                        sheet.sheet_id
                    ),
                ));
            }
        }
        let sheet_kinds: BTreeSet<StableDocsPackDetailSheetKind> = self
            .pack_detail_sheets
            .iter()
            .map(|sheet| sheet.sheet_kind)
            .collect();
        for required in StableDocsPackDetailSheetKind::REQUIRED {
            if !sheet_kinds.contains(&required) {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::PackDetailSheetIncomplete,
                    StableDocsFindingSeverity::Blocker,
                    format!("packet does not cover {} detail sheets", required.as_str()),
                ));
            }
        }

        for citation_set in &self.derived_citation_sets {
            if !citation_set.is_well_formed() {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::CitationSetIdentityIncomplete,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "citation set {} is incomplete",
                        citation_set.citation_set_id
                    ),
                ));
            }
            for pack_ref in &citation_set.source_pack_id_refs {
                if !pack_ids.contains(pack_ref.as_str()) {
                    findings.push(StableDocsValidationFinding::new(
                        StableDocsFindingKind::CitationSetPackRefUnpinned,
                        StableDocsFindingSeverity::Blocker,
                        format!(
                            "citation set {} references unpinned pack {}",
                            citation_set.citation_set_id, pack_ref
                        ),
                    ));
                }
            }
            if citation_set.export_posture != StableExportPosture::ReferenceOnlyDefault
                || !citation_set.raw_pack_bodies_excluded
            {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::CitationSetBundlesRawPack,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "citation set {} is not reference-only by default",
                        citation_set.citation_set_id
                    ),
                ));
            }
            if !citation_set.raw_urls_excluded {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::RawBoundaryMaterialPresent,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "citation set {} admits raw URLs",
                        citation_set.citation_set_id
                    ),
                ));
            }
        }

        for drawer in &self.citation_drawers {
            if !drawer.preserves_truth()
                || !result_ids.contains(drawer.result_id_ref.as_str())
                || !citation_set_ids.contains(drawer.citation_set_id_ref.as_str())
            {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::CitationDrawerParityDropped,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "citation drawer {} does not preserve citation basis",
                        drawer.drawer_id
                    ),
                ));
            }
        }
        for required in [
            StableDocsConsumerSurface::DocsBrowser,
            StableDocsConsumerSurface::HelpAbout,
            StableDocsConsumerSurface::Onboarding,
            StableDocsConsumerSurface::AiExplainer,
            StableDocsConsumerSurface::SupportExport,
        ] {
            if !self
                .citation_drawers
                .iter()
                .any(|drawer| drawer.consumer_surface == required && drawer.preserves_truth())
            {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::CitationDrawerParityDropped,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "packet does not preserve citation-drawer parity for {}",
                        required.as_str()
                    ),
                ));
            }
        }

        for required in StableDocsConsumerSurface::REQUIRED {
            if !self.has_projection_for(required) {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::MissingConsumerProjection,
                    StableDocsFindingSeverity::Blocker,
                    format!("packet is missing {} projection", required.as_str()),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::ConsumerProjectionDrift,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "projection {} dropped stable docs contract truth",
                        projection.projection_ref
                    ),
                ));
            }
        }

        self.validate_drill_coverage(&mut findings);

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != StableDocsFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::PromotionStateMismatch,
                    StableDocsFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }

    fn validate_drill_coverage(&self, findings: &mut Vec<StableDocsValidationFinding>) {
        let version_states: Vec<VersionMatchState> = self
            .results
            .iter()
            .map(|result| result.version_match_state)
            .collect();
        if !version_states.contains(&VersionMatchState::CompatibleMinorDrift) {
            findings.push(StableDocsValidationFinding::new(
                StableDocsFindingKind::RequiredDrillCoverageMissing,
                StableDocsFindingSeverity::Blocker,
                "packet does not cover nearby-version compatible drift",
            ));
        }
        let freshness_states: Vec<DocsFreshnessClass> = self
            .results
            .iter()
            .map(|result| result.freshness_state)
            .collect();
        if !freshness_states.contains(&DocsFreshnessClass::Stale) {
            findings.push(StableDocsValidationFinding::new(
                StableDocsFindingKind::RequiredDrillCoverageMissing,
                StableDocsFindingSeverity::Blocker,
                "packet does not cover stale docs freshness",
            ));
        }
        let stale_classes: BTreeSet<StaleExampleFindingClass> =
            self.stale_example_drill_classes.iter().copied().collect();
        for required in [
            StaleExampleFindingClass::NearbyVersion,
            StaleExampleFindingClass::StaleExample,
            StaleExampleFindingClass::QuarantinedPack,
        ] {
            if !stale_classes.contains(&required) {
                findings.push(StableDocsValidationFinding::new(
                    StableDocsFindingKind::RequiredDrillCoverageMissing,
                    StableDocsFindingSeverity::Blocker,
                    format!(
                        "packet does not cover stale-example drill {}",
                        required.as_str()
                    ),
                ));
            }
        }
        if !self
            .pack_manifests
            .iter()
            .any(|manifest| manifest.local_availability == DocsPackLocalAvailability::Quarantined)
        {
            findings.push(StableDocsValidationFinding::new(
                StableDocsFindingKind::RequiredDrillCoverageMissing,
                StableDocsFindingSeverity::Blocker,
                "packet does not cover quarantined pack availability",
            ));
        }
        if !self
            .sources
            .iter()
            .any(|source| source.browser_handoff == DocsExternalOpenState::Available)
        {
            findings.push(StableDocsValidationFinding::new(
                StableDocsFindingKind::RequiredDrillCoverageMissing,
                StableDocsFindingSeverity::Blocker,
                "packet does not cover browser-handoff-required state",
            ));
        }
    }
}

fn promotion_state_for_findings(
    findings: &[StableDocsValidationFinding],
) -> StableDocsPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == StableDocsFindingSeverity::Blocker)
    {
        StableDocsPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == StableDocsFindingSeverity::Warning)
    {
        StableDocsPromotionState::NarrowedBelowStable
    } else {
        StableDocsPromotionState::Stable
    }
}

/// Support-export wrapper preserving the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableDocsSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Exported packet id.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Exact packet preserved by the export.
    pub export_packet: StableDocsSourceResultPackCitationPacket,
}

impl StableDocsSupportExport {
    /// Returns true when the export preserves the same packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == STABLE_DOCS_CONTRACT_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == STABLE_DOCS_CONTRACT_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in stable artifact.
#[derive(Debug)]
pub enum StableDocsContractArtifactError {
    /// Artifact JSON failed to parse.
    Packet(serde_json::Error),
    /// Artifact failed validation.
    Validation(Vec<StableDocsValidationFinding>),
}

impl fmt::Display for StableDocsContractArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(formatter, "stable docs packet parse failed: {error}"),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "stable docs packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for StableDocsContractArtifactError {}

/// Returns the seeded stable packet input.
pub fn seeded_stable_docs_source_result_pack_and_citation_input(
) -> StableDocsSourceResultPackCitationInput {
    seed::seeded_stable_input()
}

/// Materializes the checked-in stable packet from the seed.
///
/// # Errors
///
/// Returns an error when the seeded packet fails its own stable invariants.
pub fn current_stable_docs_source_result_pack_and_citation_packet(
) -> Result<StableDocsSourceResultPackCitationPacket, StableDocsContractArtifactError> {
    let packet = StableDocsSourceResultPackCitationPacket::materialize(
        seeded_stable_docs_source_result_pack_and_citation_input(),
    );
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(StableDocsContractArtifactError::Validation(findings))
    }
}

mod seed {
    use crate::{
        DocsPackChannel, DocsPackMirrorLineage, DocsPackMirrorState, DocsPackPublishableState,
        DocsPackRefreshState, DocsPackSignatureStatus, DocsPackSignerClass, DocsPackSigningBlock,
        DocsPackSourceClass, DocsPackVersionRange,
    };

    use super::*;

    pub(super) fn seeded_stable_input() -> StableDocsSourceResultPackCitationInput {
        let packet_id = "packet:stable_docs_source_result_pack_citation:001".to_owned();
        StableDocsSourceResultPackCitationInput {
            packet_id: packet_id.clone(),
            workflow_or_surface_id: "workflow:docs_help_onboarding_ai_support:stable".to_owned(),
            generated_at: "2026-06-04T16:00:00Z".to_owned(),
            sources: sources(),
            results: results(),
            pack_manifests: manifests(),
            pack_detail_sheets: detail_sheets(),
            derived_citation_sets: citation_sets(),
            citation_drawers: drawers(),
            consumer_projections: StableDocsConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(|surface| projection(surface, &packet_id))
                .collect(),
            source_contract_refs: vec![
                STABLE_DOCS_CONTRACT_DOC_REF.to_owned(),
                STABLE_DOCS_CONTRACT_ARTIFACT_DOC_REF.to_owned(),
                STABLE_DOCS_CONTRACT_SCHEMA_REF.to_owned(),
            ],
            stale_example_drill_classes: vec![
                StaleExampleFindingClass::NearbyVersion,
                StaleExampleFindingClass::StaleExample,
                StaleExampleFindingClass::QuarantinedPack,
            ],
            render_modes: vec![
                DocsRenderMode::Rendered,
                DocsRenderMode::MirroredOnly,
                DocsRenderMode::BrowserHandoffOnly,
                DocsRenderMode::NotRendered,
            ],
            validation_result_classes: vec![
                DocsValidationResultClass::ValidatedClean,
                DocsValidationResultClass::ValidatedWithWarnings,
                DocsValidationResultClass::ValidatedFailed,
            ],
        }
    }

    fn source(
        source_id: &str,
        source_class: CitationSourceClass,
        pack_id: &str,
        owner_ref: &str,
        trust_support_class: StableDocsSupportTrustClass,
        browser_handoff: DocsExternalOpenState,
        mirror_offline_posture: DocsMirrorOfflinePosture,
        precedence_class: SourcePrecedenceClass,
        version_match_state: VersionMatchState,
        freshness_state: DocsFreshnessClass,
        disclosure_note: Option<&str>,
    ) -> StableDocsSourceDescriptor {
        StableDocsSourceDescriptor {
            source_id: source_id.to_owned(),
            source_class,
            provider_or_pack_id: pack_id.to_owned(),
            provider_or_pack_revision_ref: format!("{pack_id}@2026.06"),
            owner_ref: owner_ref.to_owned(),
            locale: "en-US".to_owned(),
            trust_support_class,
            browser_handoff,
            browser_handoff_packet_ref: (browser_handoff == DocsExternalOpenState::Available)
                .then(|| format!("browser-handoff:{source_id}:001")),
            mirror_offline_posture,
            precedence_class,
            version_match_state,
            freshness_state,
            pack_manifest_ref: Some(pack_id.to_owned()),
            disclosure_note: disclosure_note.map(str::to_owned),
            raw_boundary_material_excluded: true,
        }
    }

    fn sources() -> Vec<StableDocsSourceDescriptor> {
        vec![
            source(
                "source:project_docs:workspace",
                CitationSourceClass::ProjectDocs,
                "pack:project_docs",
                "owner:workspace",
                StableDocsSupportTrustClass::FirstPartyAuthoritative,
                DocsExternalOpenState::NotRequired,
                DocsMirrorOfflinePosture::LocalProjectPack,
                SourcePrecedenceClass::ProjectOutranksVendorDefault,
                VersionMatchState::ExactBuildMatch,
                DocsFreshnessClass::AuthoritativeLive,
                None,
            ),
            source(
                "source:generated_reference:commands",
                CitationSourceClass::GeneratedReference,
                "pack:generated_reference",
                "owner:aureline-build",
                StableDocsSupportTrustClass::FirstPartyAuthoritative,
                DocsExternalOpenState::NotRequired,
                DocsMirrorOfflinePosture::GeneratedLocal,
                SourcePrecedenceClass::NotApplicable,
                VersionMatchState::ExactBuildMatch,
                DocsFreshnessClass::AuthoritativeLive,
                None,
            ),
            source(
                "source:mirrored_vendor:react",
                CitationSourceClass::MirroredOfficialDocs,
                "pack:mirrored_vendor_react",
                "owner:react-upstream",
                StableDocsSupportTrustClass::SignedMirrorVerified,
                DocsExternalOpenState::NotRequired,
                DocsMirrorOfflinePosture::OfflinePinnedPack,
                SourcePrecedenceClass::ProjectOutranksVendorDefault,
                VersionMatchState::CompatibleMinorDrift,
                DocsFreshnessClass::WarmCached,
                Some("Mirrored vendor docs are nearby-version compatible and rank below project docs for repo-specific answers."),
            ),
            source(
                "source:knowledge_pack:onboarding",
                CitationSourceClass::CuratedKnowledgePack,
                "pack:knowledge_onboarding",
                "owner:learning-team",
                StableDocsSupportTrustClass::CuratedSupported,
                DocsExternalOpenState::NotRequired,
                DocsMirrorOfflinePosture::CachedLocal,
                SourcePrecedenceClass::NotApplicable,
                VersionMatchState::ExactBuildMatch,
                DocsFreshnessClass::Stale,
                Some("The onboarding pack is stale and must be labeled until refreshed."),
            ),
            source(
                "source:vendor_live:portal",
                CitationSourceClass::VendorProviderDocs,
                "pack:vendor_live_portal",
                "owner:vendor-portal",
                StableDocsSupportTrustClass::OnlineOnlyProvider,
                DocsExternalOpenState::Available,
                DocsMirrorOfflinePosture::LiveOnline,
                SourcePrecedenceClass::ProjectVendorDisagreementInspectable,
                VersionMatchState::UnknownTargetBuild,
                DocsFreshnessClass::Unverified,
                Some("Live provider docs require explicit browser handoff and remain inspectable below project docs."),
            ),
            source(
                "source:derived:architecture_tour",
                CitationSourceClass::DerivedExplanation,
                "pack:knowledge_onboarding",
                "owner:explainer-pipeline",
                StableDocsSupportTrustClass::DerivedInferenceOnly,
                DocsExternalOpenState::NotRequired,
                DocsMirrorOfflinePosture::CachedLocal,
                SourcePrecedenceClass::NotApplicable,
                VersionMatchState::ExactBuildMatch,
                DocsFreshnessClass::WarmCached,
                Some("Derived explanations are citation-backed summaries, not primary authority."),
            ),
        ]
    }

    fn result(
        result_id: &str,
        title: &str,
        source_ref: &str,
        version_match_state: VersionMatchState,
        freshness_state: DocsFreshnessClass,
        precedence_class: SourcePrecedenceClass,
        inference_markers: Vec<CitationInferenceMarker>,
    ) -> StableDocsResultObject {
        StableDocsResultObject {
            result_id: result_id.to_owned(),
            title: title.to_owned(),
            docs_source_ref: source_ref.to_owned(),
            version_match_state,
            freshness_state,
            symbol_refs: vec!["symbol:workspace:router".to_owned()],
            citation_anchor_refs: vec![format!("citation:{result_id}:anchor")],
            snippet_anchor_ref: Some(format!("snippet:{result_id}:preview")),
            docs_render_config_ref: format!("render-config:{result_id}"),
            docs_suggestion_refs: vec![format!("docs-suggestion:{result_id}")],
            docs_validation_result_refs: vec![format!("docs-validation:{result_id}")],
            precedence_class,
            omitted_source_markers: vec!["omitted:raw-pack-body".to_owned()],
            inference_markers,
            raw_boundary_material_excluded: true,
        }
    }

    fn results() -> Vec<StableDocsResultObject> {
        vec![
            result(
                "result:project_docs:router",
                "Workspace routing contract",
                "source:project_docs:workspace",
                VersionMatchState::ExactBuildMatch,
                DocsFreshnessClass::AuthoritativeLive,
                SourcePrecedenceClass::ProjectOutranksVendorDefault,
                vec![CitationInferenceMarker::RawSource],
            ),
            result(
                "result:mirrored_vendor:nearby",
                "React router nearby-version guide",
                "source:mirrored_vendor:react",
                VersionMatchState::CompatibleMinorDrift,
                DocsFreshnessClass::WarmCached,
                SourcePrecedenceClass::ProjectOutranksVendorDefault,
                vec![CitationInferenceMarker::RawSource],
            ),
            result(
                "result:knowledge:onboarding_stale",
                "Onboarding architecture tour",
                "source:knowledge_pack:onboarding",
                VersionMatchState::ExactBuildMatch,
                DocsFreshnessClass::Stale,
                SourcePrecedenceClass::NotApplicable,
                vec![CitationInferenceMarker::GeneratedSummary],
            ),
            result(
                "result:vendor_live:handoff",
                "Provider portal deployment docs",
                "source:vendor_live:portal",
                VersionMatchState::UnknownTargetBuild,
                DocsFreshnessClass::Unverified,
                SourcePrecedenceClass::ProjectVendorDisagreementInspectable,
                vec![CitationInferenceMarker::Inference],
            ),
            result(
                "result:derived:architecture_tour",
                "Architecture tour summary",
                "source:derived:architecture_tour",
                VersionMatchState::ExactBuildMatch,
                DocsFreshnessClass::WarmCached,
                SourcePrecedenceClass::NotApplicable,
                vec![CitationInferenceMarker::GeneratedSummary],
            ),
        ]
    }

    fn manifest(
        pack_id: &str,
        display_label: &str,
        source_class: DocsPackSourceClass,
        availability: DocsPackLocalAvailability,
        publishable_state: DocsPackPublishableState,
    ) -> DocsPackManifest {
        let is_mirror = source_class == DocsPackSourceClass::MirroredOfficialDocs;
        DocsPackManifest {
            pack_id: pack_id.to_owned(),
            pack_revision_ref: format!("{pack_id}@2026.06"),
            display_label: display_label.to_owned(),
            source_class,
            source_channel: DocsPackChannel::Stable,
            signing: DocsPackSigningBlock {
                signature_status: DocsPackSignatureStatus::SignedAndVerified,
                signer_class: if is_mirror {
                    DocsPackSignerClass::OfficialUpstreamMirror
                } else {
                    DocsPackSignerClass::FirstPartyProject
                },
                signing_authority_ref: format!("signer:{pack_id}"),
                signing_chain_digest: Some(format!("digest:{pack_id}:chain")),
            },
            version_range: DocsPackVersionRange {
                min_inclusive_ref: "2026.06.0".to_owned(),
                max_inclusive_ref: "2026.06.x".to_owned(),
            },
            refresh_state: if availability == DocsPackLocalAvailability::Quarantined {
                DocsPackRefreshState::Stale
            } else {
                DocsPackRefreshState::AuthoritativeLive
            },
            last_refresh_at: Some("2026-06-04T15:00:00Z".to_owned()),
            mirror_lineage: DocsPackMirrorLineage {
                mirror_state: if is_mirror {
                    DocsPackMirrorState::Continuous
                } else {
                    DocsPackMirrorState::NotApplicable
                },
                mirror_of_pack_id: is_mirror.then(|| "upstream:react-docs".to_owned()),
                upstream_revision_ref: is_mirror.then(|| "react@19.1".to_owned()),
                predecessor_revision_ref: is_mirror.then(|| "react@19.0".to_owned()),
                air_gapped_origin_label: None,
                offline_expiration_at: None,
            },
            pin_state: if availability == DocsPackLocalAvailability::MirrorOfflinePinned {
                DocsPackPinState::PinnedOffline
            } else {
                DocsPackPinState::Pinned
            },
            local_availability: availability,
            publishable_state,
            publishable_blocking_reasons: if publishable_state
                == DocsPackPublishableState::Quarantined
            {
                vec!["quarantined_pack".to_owned()]
            } else {
                Vec::new()
            },
            manifest_schema_version: 1,
            disclosure_note: (availability == DocsPackLocalAvailability::Quarantined).then(|| {
                "Pack is quarantined; identity remains exportable but rendering is denied."
                    .to_owned()
            }),
            raw_boundary_material_excluded: true,
        }
    }

    fn manifests() -> Vec<DocsPackManifest> {
        vec![
            manifest(
                "pack:project_docs",
                "Workspace project docs",
                DocsPackSourceClass::ProjectDocs,
                DocsPackLocalAvailability::AvailableLocal,
                DocsPackPublishableState::Publishable,
            ),
            manifest(
                "pack:generated_reference",
                "Generated command reference",
                DocsPackSourceClass::GeneratedReference,
                DocsPackLocalAvailability::AvailableLocal,
                DocsPackPublishableState::Publishable,
            ),
            manifest(
                "pack:mirrored_vendor_react",
                "React mirrored docs",
                DocsPackSourceClass::MirroredOfficialDocs,
                DocsPackLocalAvailability::MirrorOfflinePinned,
                DocsPackPublishableState::Publishable,
            ),
            manifest(
                "pack:knowledge_onboarding",
                "Onboarding knowledge pack",
                DocsPackSourceClass::CuratedKnowledgePack,
                DocsPackLocalAvailability::AvailableLocal,
                DocsPackPublishableState::Publishable,
            ),
            manifest(
                "pack:vendor_live_portal",
                "Vendor portal online handoff",
                DocsPackSourceClass::ExtensionDocsPack,
                DocsPackLocalAvailability::NotInstalled,
                DocsPackPublishableState::Publishable,
            ),
            manifest(
                "pack:quarantined_examples",
                "Quarantined example pack",
                DocsPackSourceClass::SupportRunbook,
                DocsPackLocalAvailability::Quarantined,
                DocsPackPublishableState::Quarantined,
            ),
        ]
    }

    fn actions(pack_id: &str) -> StablePackActionSet {
        StablePackActionSet {
            update_action_ref: Some(format!("action:{pack_id}:update")),
            remove_action_ref: Some(format!("action:{pack_id}:remove")),
            offline_action_ref: Some(format!("action:{pack_id}:make_offline")),
            view_citations_action_ref: format!("action:{pack_id}:view_citations"),
        }
    }

    fn detail_sheets() -> Vec<StableDocsPackDetailSheet> {
        vec![
            StableDocsPackDetailSheet {
                sheet_id: "sheet:docs_pack:mirrored_vendor_react".to_owned(),
                sheet_kind: StableDocsPackDetailSheetKind::DocsPack,
                pack_id_ref: "pack:mirrored_vendor_react".to_owned(),
                owner_ref: "owner:react-upstream".to_owned(),
                version_ref: "react@19.1".to_owned(),
                locale_coverage_refs: vec![
                    "locale:en-US".to_owned(),
                    "locale:fr-FR:partial".to_owned(),
                ],
                trust_support_class: StableDocsSupportTrustClass::SignedMirrorVerified,
                pin_state: DocsPackPinState::PinnedOffline,
                local_availability: DocsPackLocalAvailability::MirrorOfflinePinned,
                actions: actions("pack:mirrored_vendor_react"),
                browser_handoff: DocsExternalOpenState::NotRequired,
                online_only: false,
                disclosure_note: None,
            },
            StableDocsPackDetailSheet {
                sheet_id: "sheet:knowledge_pack:onboarding".to_owned(),
                sheet_kind: StableDocsPackDetailSheetKind::KnowledgePack,
                pack_id_ref: "pack:knowledge_onboarding".to_owned(),
                owner_ref: "owner:learning-team".to_owned(),
                version_ref: "onboarding@2026.06".to_owned(),
                locale_coverage_refs: vec![
                    "locale:en-US".to_owned(),
                    "locale:es-ES:review_pending".to_owned(),
                ],
                trust_support_class: StableDocsSupportTrustClass::CuratedSupported,
                pin_state: DocsPackPinState::Pinned,
                local_availability: DocsPackLocalAvailability::AvailableLocal,
                actions: actions("pack:knowledge_onboarding"),
                browser_handoff: DocsExternalOpenState::NotRequired,
                online_only: false,
                disclosure_note: None,
            },
            StableDocsPackDetailSheet {
                sheet_id: "sheet:docs_pack:vendor_live_portal".to_owned(),
                sheet_kind: StableDocsPackDetailSheetKind::DocsPack,
                pack_id_ref: "pack:vendor_live_portal".to_owned(),
                owner_ref: "owner:vendor-portal".to_owned(),
                version_ref: "online-current".to_owned(),
                locale_coverage_refs: vec!["locale:en-US:online_only".to_owned()],
                trust_support_class: StableDocsSupportTrustClass::OnlineOnlyProvider,
                pin_state: DocsPackPinState::Unpinned,
                local_availability: DocsPackLocalAvailability::NotInstalled,
                actions: actions("pack:vendor_live_portal"),
                browser_handoff: DocsExternalOpenState::Available,
                online_only: true,
                disclosure_note: Some(
                    "Online-only pack requires browser handoff; no local pack body is exported."
                        .to_owned(),
                ),
            },
        ]
    }

    fn citation_sets() -> Vec<StableDerivedCitationSet> {
        vec![StableDerivedCitationSet {
            citation_set_id: "citation-set:architecture_tour:001".to_owned(),
            derived_object_ref: "result:derived:architecture_tour".to_owned(),
            source_pack_id_refs: vec![
                "pack:project_docs".to_owned(),
                "pack:mirrored_vendor_react".to_owned(),
                "pack:knowledge_onboarding".to_owned(),
            ],
            cited_file_refs: vec!["file:src/router.rs".to_owned()],
            cited_symbol_refs: vec!["symbol:workspace:router".to_owned()],
            cited_docs_refs: vec![
                "docs:workspace:architecture#router".to_owned(),
                "docs:react:router#nearby-version".to_owned(),
            ],
            graph_epoch_ref: "graph-epoch:2026-06-04T15:58:00Z".to_owned(),
            locale: "en-US".to_owned(),
            derivation_tool_ref: "tool:docs-explainer".to_owned(),
            derivation_tool_version: "1.0.0".to_owned(),
            export_posture: StableExportPosture::ReferenceOnlyDefault,
            omitted_source_markers: vec!["omitted:raw-pack-body".to_owned()],
            inference_markers: vec![
                CitationInferenceMarker::RawSource,
                CitationInferenceMarker::GeneratedSummary,
            ],
            raw_pack_bodies_excluded: true,
            raw_urls_excluded: true,
        }]
    }

    fn drawers() -> Vec<StableCitationDrawerParity> {
        [
            StableDocsConsumerSurface::DocsBrowser,
            StableDocsConsumerSurface::HelpAbout,
            StableDocsConsumerSurface::Onboarding,
            StableDocsConsumerSurface::AiExplainer,
            StableDocsConsumerSurface::SupportExport,
        ]
        .iter()
        .copied()
        .map(|surface| StableCitationDrawerParity {
            consumer_surface: surface,
            drawer_id: format!("drawer:{}:architecture_tour", surface.as_str()),
            result_id_ref: "result:derived:architecture_tour".to_owned(),
            citation_set_id_ref: "citation-set:architecture_tour:001".to_owned(),
            supporting_file_refs: vec!["file:src/router.rs".to_owned()],
            supporting_symbol_refs: vec!["symbol:workspace:router".to_owned()],
            supporting_docs_refs: vec!["docs:workspace:architecture#router".to_owned()],
            omitted_source_markers: vec!["omitted:raw-pack-body".to_owned()],
            inference_markers: vec![CitationInferenceMarker::GeneratedSummary],
            preserves_supporting_anchors: true,
            exportable_without_raw_pack: true,
        })
        .collect()
    }

    fn projection(
        surface: StableDocsConsumerSurface,
        packet_id: &str,
    ) -> StableDocsConsumerProjection {
        StableDocsConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}:stable_docs_contract", surface.as_str()),
            packet_id_ref: packet_id.to_owned(),
            preserves_source_descriptor: true,
            preserves_result_object: true,
            preserves_pack_manifest: true,
            preserves_version_match: true,
            preserves_freshness: true,
            preserves_source_precedence: true,
            preserves_detail_sheet: true,
            preserves_citation_basis: true,
            preserves_browser_handoff: true,
            raw_private_material_excluded: true,
        }
    }
}
