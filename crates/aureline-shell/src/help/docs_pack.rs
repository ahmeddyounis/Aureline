//! Docs-pack alpha manifest model and shell-side projections.
//!
//! The manifest is the first shell-readable knowledge model for packaged
//! product help, onboarding content, mirrored reference packs, and support
//! runbooks. It keeps source class, version match, freshness, support class,
//! client scope, locality, locale fallback, and citation anchors on one
//! record so help, onboarding, support export, and docs-browser surfaces do
//! not derive those axes independently.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::embedded::boundary_card::{FreshnessClass, SourceClass, VersionMatchState};

/// Stable path to the checked-in docs-pack alpha manifest.
pub const CURRENT_DOCS_PACK_MANIFEST_PATH: &str = "artifacts/docs/docs_pack_alpha_manifest.yaml";

const CURRENT_DOCS_PACK_MANIFEST_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/docs/docs_pack_alpha_manifest.yaml"
));

/// Current schema version for [`DocsPackAlphaManifest`].
pub const DOCS_PACK_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Current record kind for [`DocsPackAlphaManifest`].
pub const DOCS_PACK_ALPHA_RECORD_KIND: &str = "docs_pack_alpha_manifest_record";

/// Governed docs-pack manifest consumed by shell help and onboarding surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackAlphaManifest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Running build identity this manifest was projected against.
    pub running_build_identity_ref: String,
    /// Projection timestamp.
    pub generated_at: String,
    /// Related schema or artifact refs.
    pub contract_refs: BTreeMap<String, String>,
    /// Pack summaries that docs nodes may cite.
    pub packs: Vec<DocsPackSummary>,
    /// Stable docs-node rows available to consumers.
    pub docs_nodes: Vec<DocsPackNode>,
    /// Source-strip projections keyed by docs-node metadata.
    pub source_strips: Vec<DocsPackSourceStrip>,
    /// Citation drawers opened from docs-node metadata.
    pub citation_drawers: Vec<DocsPackCitationDrawer>,
    /// Protected fixture rows proving acceptance states.
    pub protected_proofs: Vec<DocsPackProofCase>,
}

impl DocsPackAlphaManifest {
    /// Returns the pack summary for `pack_id`.
    pub fn pack(&self, pack_id: &str) -> Option<&DocsPackSummary> {
        self.packs.iter().find(|pack| pack.pack_id == pack_id)
    }

    /// Returns the docs node for `doc_node_id`.
    pub fn node(&self, doc_node_id: &str) -> Option<&DocsPackNode> {
        self.docs_nodes
            .iter()
            .find(|node| node.doc_node_id == doc_node_id)
    }

    /// Returns the first docs node that owns `help_anchor_id`.
    pub fn node_for_help_anchor(&self, help_anchor_id: &str) -> Option<&DocsPackNode> {
        self.docs_nodes
            .iter()
            .find(|node| node.help_anchor_id.as_deref() == Some(help_anchor_id))
    }

    /// Returns the source strip for `source_strip_ref`.
    pub fn source_strip(&self, source_strip_ref: &str) -> Option<&DocsPackSourceStrip> {
        self.source_strips
            .iter()
            .find(|strip| strip.source_strip_ref == source_strip_ref)
    }

    /// Returns the citation drawer for `citation_drawer_ref`.
    pub fn citation_drawer(&self, citation_drawer_ref: &str) -> Option<&DocsPackCitationDrawer> {
        self.citation_drawers
            .iter()
            .find(|drawer| drawer.citation_drawer_ref == citation_drawer_ref)
    }

    /// Returns a render-ready source strip and citation drawer for a docs node.
    pub fn citation_drawer_for_node(
        &self,
        doc_node_id: &str,
    ) -> Option<DocsPackCitationDrawerProjection> {
        let node = self.node(doc_node_id)?;
        let source_strip = self.source_strip(&node.source_strip_ref)?;
        let citation_drawer = self.citation_drawer(&node.citation_drawer_ref)?;
        Some(DocsPackCitationDrawerProjection {
            doc_node_id: node.doc_node_id.clone(),
            exact_reopen_ref: node.exact_reopen_ref.clone(),
            source_strip: source_strip.clone(),
            citation_drawer: citation_drawer.clone(),
        })
    }

    /// Projects compact badge tokens for a docs node.
    pub fn badge_projection_for_node(&self, doc_node_id: &str) -> Option<DocsPackBadgeProjection> {
        let node = self.node(doc_node_id)?;
        Some(DocsPackBadgeProjection::from_node(node))
    }

    /// Validates pack, node, source-strip, and citation-drawer linkage.
    pub fn validate(&self) -> Result<(), Vec<DocsPackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != DOCS_PACK_ALPHA_RECORD_KIND {
            findings.push(DocsPackValidationFinding::new(
                self.manifest_id.clone(),
                "manifest record_kind is not docs_pack_alpha_manifest_record",
            ));
        }
        if self.schema_version != DOCS_PACK_ALPHA_SCHEMA_VERSION {
            findings.push(DocsPackValidationFinding::new(
                self.manifest_id.clone(),
                "manifest schema version is unsupported",
            ));
        }

        let pack_ids = self
            .packs
            .iter()
            .map(|pack| pack.pack_id.as_str())
            .collect::<BTreeSet<_>>();
        let source_strip_refs = self
            .source_strips
            .iter()
            .map(|strip| strip.source_strip_ref.as_str())
            .collect::<BTreeSet<_>>();
        let citation_drawer_refs = self
            .citation_drawers
            .iter()
            .map(|drawer| drawer.citation_drawer_ref.as_str())
            .collect::<BTreeSet<_>>();

        for pack in &self.packs {
            if pack.client_scopes.is_empty() {
                findings.push(DocsPackValidationFinding::new(
                    pack.pack_id.clone(),
                    "pack has no client scopes",
                ));
            }
            if !pack.available_locales.contains(&pack.primary_locale) {
                findings.push(DocsPackValidationFinding::new(
                    pack.pack_id.clone(),
                    "pack available_locales does not include primary_locale",
                ));
            }
            if pack.locality_state == DocsPackLocalityState::NotInstalled
                && pack.install_state != DocsPackInstallState::NotInstalled
            {
                findings.push(DocsPackValidationFinding::new(
                    pack.pack_id.clone(),
                    "pack locality says not installed but install_state differs",
                ));
            }
        }

        for node in &self.docs_nodes {
            match self.pack(&node.source_pack_ref) {
                Some(pack) => {
                    if pack.pack_revision_ref != node.source_pack_revision_ref {
                        findings.push(DocsPackValidationFinding::new(
                            node.doc_node_id.clone(),
                            "node pack revision does not match the owning pack",
                        ));
                    }
                }
                None => findings.push(DocsPackValidationFinding::new(
                    node.doc_node_id.clone(),
                    "node references an unknown docs pack",
                )),
            }

            if !pack_ids.contains(node.source_pack_ref.as_str()) {
                findings.push(DocsPackValidationFinding::new(
                    node.doc_node_id.clone(),
                    "node source_pack_ref is not declared by packs",
                ));
            }
            if !source_strip_refs.contains(node.source_strip_ref.as_str()) {
                findings.push(DocsPackValidationFinding::new(
                    node.doc_node_id.clone(),
                    "node source_strip_ref cannot be resolved",
                ));
            }
            if !citation_drawer_refs.contains(node.citation_drawer_ref.as_str()) {
                findings.push(DocsPackValidationFinding::new(
                    node.doc_node_id.clone(),
                    "node citation_drawer_ref cannot be resolved",
                ));
            }
            if node.citation_anchor_availability == DocsPackCitationAnchorAvailability::Available
                && node.citation_anchor_refs.is_empty()
            {
                findings.push(DocsPackValidationFinding::new(
                    node.doc_node_id.clone(),
                    "node declares available citations without anchor refs",
                ));
            }
            if node.citation_anchor_availability
                == DocsPackCitationAnchorAvailability::RequiredMissing
                && !node.citation_anchor_refs.is_empty()
            {
                findings.push(DocsPackValidationFinding::new(
                    node.doc_node_id.clone(),
                    "node declares missing citations but still carries anchor refs",
                ));
            }
            if node.locale_availability
                == DocsPackLocaleAvailability::RequestedLocaleMissingFallbackToPrimary
                && node.source_language_fallback_ref.is_none()
            {
                findings.push(DocsPackValidationFinding::new(
                    node.doc_node_id.clone(),
                    "locale fallback node is missing source_language_fallback_ref",
                ));
            }
            if node.install_state == DocsPackInstallState::NotInstalled && node.renderable {
                findings.push(DocsPackValidationFinding::new(
                    node.doc_node_id.clone(),
                    "not-installed node must not render body content",
                ));
            }
        }

        for drawer in &self.citation_drawers {
            if let Some(node) = self.node(&drawer.doc_node_id) {
                if node.citation_drawer_ref != drawer.citation_drawer_ref {
                    findings.push(DocsPackValidationFinding::new(
                        drawer.citation_drawer_ref.clone(),
                        "citation drawer ref does not match node metadata",
                    ));
                }
                if node.source_strip_ref != drawer.source_strip_ref {
                    findings.push(DocsPackValidationFinding::new(
                        drawer.citation_drawer_ref.clone(),
                        "citation drawer source strip does not match node metadata",
                    ));
                }
                let drawer_refs = drawer
                    .rows
                    .iter()
                    .map(|row| row.citation_ref.as_str())
                    .collect::<BTreeSet<_>>();
                for anchor_ref in &node.citation_anchor_refs {
                    if !drawer_refs.contains(anchor_ref.as_str()) {
                        findings.push(DocsPackValidationFinding::new(
                            drawer.citation_drawer_ref.clone(),
                            "citation drawer does not contain every node anchor ref",
                        ));
                    }
                }
                for row in &drawer.rows {
                    if row.pack_revision_ref != node.source_pack_revision_ref {
                        findings.push(DocsPackValidationFinding::new(
                            drawer.citation_drawer_ref.clone(),
                            "citation drawer row pack revision does not match node revision",
                        ));
                    }
                }
            } else {
                findings.push(DocsPackValidationFinding::new(
                    drawer.citation_drawer_ref.clone(),
                    "citation drawer references an unknown docs node",
                ));
            }
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Parses the checked-in docs-pack alpha manifest.
///
/// # Errors
///
/// Returns a YAML parse error if the checked-in manifest stops matching the
/// shell-side manifest model.
pub fn current_docs_pack_manifest() -> Result<DocsPackAlphaManifest, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_DOCS_PACK_MANIFEST_YAML)
}

/// Summary of one docs pack available to docs-node consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackSummary {
    /// Stable pack id.
    pub pack_id: String,
    /// Stable pack revision ref.
    pub pack_revision_ref: String,
    /// Human-readable version label.
    pub display_version: String,
    /// Source class for the pack.
    pub source_class: DocsPackSourceClass,
    /// Version or revision represented by the pack.
    pub version_or_revision: String,
    /// Version match state against the running build.
    pub version_match_state: DocsPackVersionMatchState,
    /// Freshness state for the pack.
    pub freshness_class: DocsPackFreshnessClass,
    /// Support class for the pack.
    pub support_class: DocsPackSupportClass,
    /// Client scopes allowed to consume this pack.
    pub client_scopes: Vec<DocsPackClientScope>,
    /// Canonical source locale.
    pub primary_locale: String,
    /// Locales available in this pack.
    pub available_locales: Vec<String>,
    /// Locality posture.
    pub locality_state: DocsPackLocalityState,
    /// Installation posture.
    pub install_state: DocsPackInstallState,
    /// Compact offline badge token.
    pub offline_badge: String,
    /// Last refresh timestamp when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refreshed_at: Option<String>,
    /// Offline expiration timestamp when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_expiration_at: Option<String>,
    /// Citation-anchor availability for pack-backed rows.
    pub citation_anchor_availability: DocsPackCitationAnchorAvailability,
}

/// Stable docs-node row consumed by help, onboarding, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackNode {
    /// Stable docs-node id.
    pub doc_node_id: String,
    /// Documentation kind.
    pub doc_kind: DocsPackDocKind,
    /// User-facing title.
    pub title: String,
    /// User-facing summary.
    pub summary: String,
    /// Owning pack id.
    pub source_pack_ref: String,
    /// Owning pack revision ref.
    pub source_pack_revision_ref: String,
    /// Source class for the node.
    pub source_class: DocsPackSourceClass,
    /// Scope class for the node.
    pub source_scope: DocsPackScopeClass,
    /// Canonical source locale.
    pub primary_locale: String,
    /// Requested locale for the consumer.
    pub requested_locale: String,
    /// Effective rendered locale.
    pub effective_locale: String,
    /// Version or revision represented by the node.
    pub version_or_revision: String,
    /// Version match state against the running build.
    pub version_match_state: DocsPackVersionMatchState,
    /// Freshness state for the node.
    pub freshness_class: DocsPackFreshnessClass,
    /// Support class for the node.
    pub support_class: DocsPackSupportClass,
    /// Client scopes allowed to render this node.
    pub client_scopes: Vec<DocsPackClientScope>,
    /// Locality posture for this node.
    pub locality_state: DocsPackLocalityState,
    /// Installation posture for this node.
    pub install_state: DocsPackInstallState,
    /// Locale availability for this node.
    pub locale_availability: DocsPackLocaleAvailability,
    /// Source-language fallback ref when fallback is in effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_language_fallback_ref: Option<String>,
    /// Citation-anchor availability for the node.
    pub citation_anchor_availability: DocsPackCitationAnchorAvailability,
    /// Citation anchors backing the node.
    pub citation_anchor_refs: Vec<String>,
    /// Source strip ref opened from node metadata.
    pub source_strip_ref: String,
    /// Citation drawer ref opened from node metadata.
    pub citation_drawer_ref: String,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Help anchor id when command or onboarding backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_anchor_id: Option<String>,
    /// Command id when command backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// Whether the body is renderable in product.
    pub renderable: bool,
    /// Typed degraded reasons.
    pub degraded_reasons: Vec<String>,
}

impl DocsPackNode {
    /// Returns true when the node is usable on `client_scope`.
    pub fn supports_client_scope(&self, client_scope: DocsPackClientScope) -> bool {
        self.client_scopes.contains(&client_scope)
    }

    /// Returns true when the node carries at least one usable citation anchor.
    pub fn has_citation_anchor(&self) -> bool {
        self.citation_anchor_availability == DocsPackCitationAnchorAvailability::Available
            && !self.citation_anchor_refs.is_empty()
    }
}

/// Source-strip projection opened from docs-node metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackSourceStrip {
    /// Stable source-strip ref.
    pub source_strip_ref: String,
    /// Docs-node id this strip describes.
    pub doc_node_id: String,
    /// Source class for this strip.
    pub source_class: DocsPackSourceClass,
    /// Source pack id.
    pub source_pack_ref: String,
    /// Source pack revision ref.
    pub source_pack_revision_ref: String,
    /// Version or revision label.
    pub version_or_revision: String,
    /// Locale rendered by this strip.
    pub locale: String,
    /// Freshness state shown on the strip.
    pub freshness_class: DocsPackFreshnessClass,
    /// Citation drawer opened from this strip.
    pub citation_drawer_ref: String,
}

/// Citation drawer row for one anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackCitationRow {
    /// Stable citation ref.
    pub citation_ref: String,
    /// Citation kind token.
    pub citation_kind: String,
    /// Source anchor ref.
    pub anchor_ref: String,
    /// Pack revision ref the citation was resolved against.
    pub pack_revision_ref: String,
    /// Freshness state for this citation.
    pub freshness_class: DocsPackFreshnessClass,
    /// Why this citation is present.
    pub role: String,
}

/// Citation drawer opened from docs-node metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackCitationDrawer {
    /// Stable citation drawer ref.
    pub citation_drawer_ref: String,
    /// Docs-node id this drawer describes.
    pub doc_node_id: String,
    /// Source strip ref shown above drawer rows.
    pub source_strip_ref: String,
    /// Citation rows.
    pub rows: Vec<DocsPackCitationRow>,
}

/// Reopen projection containing source-strip and citation-drawer metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackCitationDrawerProjection {
    /// Docs-node id being reopened.
    pub doc_node_id: String,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Source strip shown before citation rows.
    pub source_strip: DocsPackSourceStrip,
    /// Citation drawer rows.
    pub citation_drawer: DocsPackCitationDrawer,
}

/// Compact badge projection for render surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackBadgeProjection {
    /// Docs-node id.
    pub doc_node_id: String,
    /// Source class token.
    pub source_class_token: String,
    /// Version match token.
    pub version_match_token: String,
    /// Freshness token.
    pub freshness_token: String,
    /// Support class token.
    pub support_class_token: String,
    /// Client scope tokens.
    pub client_scope_tokens: Vec<String>,
    /// Locality token.
    pub locality_token: String,
    /// Install state token.
    pub install_state_token: String,
    /// Locale availability token.
    pub locale_availability_token: String,
    /// Citation availability token.
    pub citation_anchor_availability_token: String,
}

impl DocsPackBadgeProjection {
    /// Builds a compact badge projection from one docs node.
    pub fn from_node(node: &DocsPackNode) -> Self {
        Self {
            doc_node_id: node.doc_node_id.clone(),
            source_class_token: node.source_class.as_str().to_owned(),
            version_match_token: node.version_match_state.as_str().to_owned(),
            freshness_token: node.freshness_class.as_str().to_owned(),
            support_class_token: node.support_class.as_str().to_owned(),
            client_scope_tokens: node
                .client_scopes
                .iter()
                .map(|scope| scope.as_str().to_owned())
                .collect(),
            locality_token: node.locality_state.as_str().to_owned(),
            install_state_token: node.install_state.as_str().to_owned(),
            locale_availability_token: node.locale_availability.as_str().to_owned(),
            citation_anchor_availability_token: node
                .citation_anchor_availability
                .as_str()
                .to_owned(),
        }
    }
}

/// Protected proof row in the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackProofCase {
    /// Stable proof id.
    pub proof_id: String,
    /// Fixture path relative to the repository root.
    pub fixture_ref: String,
    /// Docs-node ref exercised by the proof.
    pub docs_node_ref: String,
    /// States exercised by the proof.
    pub exercised_states: Vec<String>,
}

/// Validation finding for docs-pack alpha manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPackValidationFinding {
    /// Row id that failed validation.
    pub row_ref: String,
    /// Validation message.
    pub message: String,
}

impl DocsPackValidationFinding {
    fn new(row_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            row_ref: row_ref.into(),
            message: message.into(),
        }
    }
}

/// Docs-node kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackDocKind {
    /// Product help page.
    ProductHelp,
    /// Onboarding step or teaching card.
    OnboardingStep,
    /// Reference page from a package or language source.
    ReferencePage,
    /// Troubleshooting runbook.
    TroubleshootingRunbook,
    /// Migration note.
    MigrationNote,
    /// Stale-example warning.
    StaleExampleWarning,
}

impl DocsPackDocKind {
    /// Returns the stable token for this doc kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductHelp => "product_help",
            Self::OnboardingStep => "onboarding_step",
            Self::ReferencePage => "reference_page",
            Self::TroubleshootingRunbook => "troubleshooting_runbook",
            Self::MigrationNote => "migration_note",
            Self::StaleExampleWarning => "stale_example_warning",
        }
    }
}

/// Docs-pack source class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackSourceClass {
    /// Project-owned documentation.
    ProjectDocs,
    /// Generated reference documentation.
    GeneratedReference,
    /// Signed mirror of official upstream docs.
    MirroredOfficialDocs,
    /// Curated knowledge pack.
    CuratedKnowledgePack,
    /// Support runbook pack.
    SupportRunbook,
}

impl DocsPackSourceClass {
    /// Returns the stable token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedReference => "generated_reference",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::SupportRunbook => "support_runbook",
        }
    }

    /// Maps this source class into the docs/help badge vocabulary.
    pub const fn to_boundary_source_class(self) -> SourceClass {
        match self {
            Self::ProjectDocs => SourceClass::ProjectDocs,
            Self::GeneratedReference => SourceClass::GeneratedReference,
            Self::MirroredOfficialDocs => SourceClass::MirroredOfficialDocs,
            Self::CuratedKnowledgePack => SourceClass::CuratedKnowledgePack,
            Self::SupportRunbook => SourceClass::SupportRunbook,
        }
    }
}

/// Scope class for a docs node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackScopeClass {
    /// Launch wedge content.
    LaunchWedge,
    /// Onboarding content.
    Onboarding,
    /// Troubleshooting content.
    Troubleshooting,
    /// Docs-maintenance content.
    DocsMaintenance,
}

impl DocsPackScopeClass {
    /// Returns the stable token for this scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchWedge => "launch_wedge",
            Self::Onboarding => "onboarding",
            Self::Troubleshooting => "troubleshooting",
            Self::DocsMaintenance => "docs_maintenance",
        }
    }
}

/// Version-match state for docs packs and docs nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackVersionMatchState {
    /// Pack exactly matches the running build.
    ExactBuildMatch,
    /// Pack is close enough but not exact.
    CompatibleMinorDrift,
    /// Pack is incompatible with the running client.
    IncompatibleDriftDetected,
    /// Pack is a pre-release whose verification is incomplete.
    PreReleaseUnverified,
    /// Running target build cannot be resolved.
    UnknownTargetBuild,
}

impl DocsPackVersionMatchState {
    /// Returns the stable token for this version-match state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }

    /// Maps this value into the docs/help badge vocabulary.
    pub const fn to_boundary_version_match_state(self) -> VersionMatchState {
        match self {
            Self::ExactBuildMatch => VersionMatchState::ExactBuildMatch,
            Self::CompatibleMinorDrift => VersionMatchState::CompatibleMinorDrift,
            Self::IncompatibleDriftDetected => VersionMatchState::IncompatibleDriftDetected,
            Self::PreReleaseUnverified => VersionMatchState::PreReleaseUnverified,
            Self::UnknownTargetBuild => VersionMatchState::UnknownTargetBuild,
        }
    }
}

/// Freshness class for docs packs and docs nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackFreshnessClass {
    /// Live authoritative content.
    AuthoritativeLive,
    /// Cache is inside the accepted freshness window.
    WarmCached,
    /// Cache is usable but degraded.
    DegradedCached,
    /// Cache or mirror is stale.
    Stale,
    /// Freshness is unknown.
    Unverified,
}

impl DocsPackFreshnessClass {
    /// Returns the stable token for this freshness class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    /// Maps this value into the docs/help badge vocabulary.
    pub const fn to_boundary_freshness_class(self) -> FreshnessClass {
        match self {
            Self::AuthoritativeLive => FreshnessClass::AuthoritativeLive,
            Self::WarmCached => FreshnessClass::WarmCached,
            Self::DegradedCached => FreshnessClass::DegradedCached,
            Self::Stale => FreshnessClass::Stale,
            Self::Unverified => FreshnessClass::Unverified,
        }
    }
}

/// Support class for docs-pack content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackSupportClass {
    /// Certified content.
    Certified,
    /// Supported content.
    Supported,
    /// Limited support content.
    Limited,
    /// Community support content.
    Community,
    /// Experimental content.
    Experimental,
    /// Retest pending content.
    RetestPending,
    /// Evidence-stale content.
    EvidenceStale,
}

impl DocsPackSupportClass {
    /// Returns the stable token for this support class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Community => "community",
            Self::Experimental => "experimental",
            Self::RetestPending => "retest_pending",
            Self::EvidenceStale => "evidence_stale",
        }
    }
}

/// Client scopes that may consume docs-pack content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackClientScope {
    /// Native desktop product.
    DesktopProduct,
    /// CLI or headless command path.
    Cli,
    /// Browser companion surface.
    CompanionSurface,
    /// Remote agent surface.
    RemoteAgent,
    /// SDK or API consumer.
    SdkOrApi,
    /// Managed admin surface.
    ManagedAdminSurface,
}

impl DocsPackClientScope {
    /// Returns the stable token for this client scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopProduct => "desktop_product",
            Self::Cli => "cli",
            Self::CompanionSurface => "companion_surface",
            Self::RemoteAgent => "remote_agent",
            Self::SdkOrApi => "sdk_or_api",
            Self::ManagedAdminSurface => "managed_admin_surface",
        }
    }
}

/// Locality posture for docs-pack content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackLocalityState {
    /// Live installed content is current.
    LiveInstalledCurrent,
    /// Warm local cache is being used.
    WarmLocalCache,
    /// Mirror-only content is being used.
    MirrorOnly,
    /// Local-only pack is being used.
    LocalOnly,
    /// Pack is referenced but not installed.
    NotInstalled,
}

impl DocsPackLocalityState {
    /// Returns the stable token for this locality state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveInstalledCurrent => "live_installed_current",
            Self::WarmLocalCache => "warm_local_cache",
            Self::MirrorOnly => "mirror_only",
            Self::LocalOnly => "local_only",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// Installation posture for docs-pack content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackInstallState {
    /// Pack is installed.
    Installed,
    /// Cached copy is installed without live owner reachability.
    CachedOnly,
    /// Verified mirror-only copy is installed.
    MirrorOnlyVerified,
    /// Pack is not installed.
    NotInstalled,
}

impl DocsPackInstallState {
    /// Returns the stable token for this install state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::CachedOnly => "cached_only",
            Self::MirrorOnlyVerified => "mirror_only_verified",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// Locale availability for docs-pack content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackLocaleAvailability {
    /// Requested locale is authoritative.
    RequestedLocaleAuthoritative,
    /// Requested locale is partial.
    RequestedLocalePartial,
    /// Requested locale falls back to the primary source language.
    RequestedLocaleMissingFallbackToPrimary,
    /// Requested locale-specific content is not installed.
    RequestedLocaleNotInstalled,
}

impl DocsPackLocaleAvailability {
    /// Returns the stable token for this locale availability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedLocaleAuthoritative => "requested_locale_authoritative",
            Self::RequestedLocalePartial => "requested_locale_partial",
            Self::RequestedLocaleMissingFallbackToPrimary => {
                "requested_locale_missing_fallback_to_primary"
            }
            Self::RequestedLocaleNotInstalled => "requested_locale_not_installed",
        }
    }
}

/// Citation-anchor availability for docs-pack content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackCitationAnchorAvailability {
    /// Citation anchors are available.
    Available,
    /// Citation anchors are required but missing.
    RequiredMissing,
    /// Citation anchors are not required.
    NotRequired,
}

impl DocsPackCitationAnchorAvailability {
    /// Returns the stable token for this citation availability state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::RequiredMissing => "required_missing",
            Self::NotRequired => "not_required",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    struct FixtureCase {
        docs_node_ref: String,
        expected: FixtureExpected,
    }

    #[derive(Debug, Deserialize)]
    struct FixtureExpected {
        source_pack_ref: Option<String>,
        source_pack_revision_ref: Option<String>,
        source_class: Option<DocsPackSourceClass>,
        requested_locale: Option<String>,
        effective_locale: Option<String>,
        version_match_state: Option<DocsPackVersionMatchState>,
        freshness_class: Option<DocsPackFreshnessClass>,
        support_class: Option<DocsPackSupportClass>,
        client_scopes: Option<Vec<DocsPackClientScope>>,
        locality_state: Option<DocsPackLocalityState>,
        install_state: Option<DocsPackInstallState>,
        locale_availability: Option<DocsPackLocaleAvailability>,
        source_language_fallback_ref: Option<String>,
        citation_anchor_availability: Option<DocsPackCitationAnchorAvailability>,
        citation_anchor_refs: Option<Vec<String>>,
        source_strip_ref: Option<String>,
        citation_drawer_ref: Option<String>,
        exact_reopen_ref: Option<String>,
        renderable: Option<bool>,
        degraded_reasons: Option<Vec<String>>,
    }

    fn fixture_path(name: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/docs/docs_pack_alpha")
            .join(format!("{name}.yaml"))
    }

    fn load_fixture(name: &str) -> FixtureCase {
        let payload = std::fs::read_to_string(fixture_path(name)).expect("fixture must read");
        serde_yaml::from_str(&payload).expect("fixture must parse")
    }

    fn assert_fixture(name: &str) {
        let manifest = current_docs_pack_manifest().expect("manifest parses");
        manifest.validate().expect("manifest validates");
        let fixture = load_fixture(name);
        let node = manifest
            .node(&fixture.docs_node_ref)
            .expect("fixture docs node exists in manifest");
        let expected = fixture.expected;

        if let Some(value) = expected.source_pack_ref {
            assert_eq!(node.source_pack_ref, value);
        }
        if let Some(value) = expected.source_pack_revision_ref {
            assert_eq!(node.source_pack_revision_ref, value);
        }
        if let Some(value) = expected.source_class {
            assert_eq!(node.source_class, value);
        }
        if let Some(value) = expected.requested_locale {
            assert_eq!(node.requested_locale, value);
        }
        if let Some(value) = expected.effective_locale {
            assert_eq!(node.effective_locale, value);
        }
        if let Some(value) = expected.version_match_state {
            assert_eq!(node.version_match_state, value);
        }
        if let Some(value) = expected.freshness_class {
            assert_eq!(node.freshness_class, value);
        }
        if let Some(value) = expected.support_class {
            assert_eq!(node.support_class, value);
        }
        if let Some(value) = expected.client_scopes {
            assert_eq!(node.client_scopes, value);
        }
        if let Some(value) = expected.locality_state {
            assert_eq!(node.locality_state, value);
        }
        if let Some(value) = expected.install_state {
            assert_eq!(node.install_state, value);
        }
        if let Some(value) = expected.locale_availability {
            assert_eq!(node.locale_availability, value);
        }
        if let Some(value) = expected.source_language_fallback_ref {
            assert_eq!(
                node.source_language_fallback_ref.as_deref(),
                Some(value.as_str())
            );
        }
        if let Some(value) = expected.citation_anchor_availability {
            assert_eq!(node.citation_anchor_availability, value);
        }
        if let Some(value) = expected.citation_anchor_refs {
            assert_eq!(node.citation_anchor_refs, value);
        }
        if let Some(value) = expected.source_strip_ref {
            assert_eq!(node.source_strip_ref, value);
        }
        if let Some(value) = expected.citation_drawer_ref {
            assert_eq!(node.citation_drawer_ref, value);
        }
        if let Some(value) = expected.exact_reopen_ref {
            assert_eq!(node.exact_reopen_ref, value);
        }
        if let Some(value) = expected.renderable {
            assert_eq!(node.renderable, value);
        }
        if let Some(value) = expected.degraded_reasons {
            assert_eq!(node.degraded_reasons, value);
        }
    }

    #[test]
    fn current_manifest_validates_and_projects_badges() {
        let manifest = current_docs_pack_manifest().expect("manifest parses");
        manifest.validate().expect("manifest validates");

        let badge = manifest
            .badge_projection_for_node("docs-node:language-reference.mirror-stale")
            .expect("badge projection exists");
        assert_eq!(badge.source_class_token, "mirrored_official_docs");
        assert_eq!(badge.version_match_token, "compatible_minor_drift");
        assert_eq!(badge.freshness_token, "stale");
        assert_eq!(badge.support_class_token, "limited");
        assert_eq!(badge.locality_token, "mirror_only");
        assert_eq!(badge.citation_anchor_availability_token, "available");
    }

    #[test]
    fn launch_node_reopens_to_source_strip_and_citation_drawer() {
        let manifest = current_docs_pack_manifest().expect("manifest parses");
        let projection = manifest
            .citation_drawer_for_node("docs-node:project-entry.open-folder")
            .expect("citation drawer projection exists");

        assert_eq!(
            projection.exact_reopen_ref,
            "reopen:docs-node:project-entry.open-folder@pack-rev:project:aureline:2026.05.13-01#en-US"
        );
        assert_eq!(
            projection.source_strip.source_pack_revision_ref,
            "pack-rev:project:aureline:2026.05.13-01"
        );
        assert_eq!(projection.citation_drawer.rows.len(), 1);
        assert_eq!(
            projection.citation_drawer.rows[0].pack_revision_ref,
            "pack-rev:project:aureline:2026.05.13-01"
        );
    }

    #[test]
    fn locale_overlay_preserves_source_fallback_and_citation_continuity() {
        let manifest = current_docs_pack_manifest().expect("manifest parses");
        let node = manifest
            .node("docs-node:onboarding.keymap-bridge")
            .expect("fallback node exists");

        assert_eq!(node.requested_locale, "es-MX");
        assert_eq!(node.effective_locale, "en-US");
        assert_eq!(
            node.locale_availability,
            DocsPackLocaleAvailability::RequestedLocaleMissingFallbackToPrimary
        );
        assert_eq!(
            node.source_language_fallback_ref.as_deref(),
            Some("docs-node:onboarding.keymap-bridge#en-US")
        );
        assert!(node.has_citation_anchor());
        let drawer = manifest
            .citation_drawer_for_node(&node.doc_node_id)
            .expect("fallback node has drawer");
        assert_eq!(drawer.citation_drawer.rows.len(), 2);
    }

    #[test]
    fn protected_fixtures_match_manifest_rows() {
        for fixture in [
            "canonical_reopen_citation",
            "locale_fallback_continuity",
            "mirror_stale_locality",
            "not_installed_degrade",
        ] {
            assert_fixture(fixture);
        }
    }
}
