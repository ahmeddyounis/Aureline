//! Governed docs knowledge-surface evidence records.
//!
//! The records in this module are the beta evidence layer shared by docs
//! browser rows, glossary cards, architecture maps, codebase explainers, quick
//! help, docs-backed search, AI evidence packets, and support exports. They
//! preserve source, version, build-date, freshness, locale, locality,
//! mirror/offline, citation, downgrade, and external-open truth without
//! carrying raw document bodies, raw URLs, raw prompt text, or provider
//! payloads.

use serde::{Deserialize, Serialize};

use crate::{
    CitationAnchorAvailability, CitationConfidenceClass, CitationInferenceMarker,
    CitationLocalityClass, CitationSourceClass, CitationTruthViolation, DocsFreshnessClass,
    DocsNodeIdentity, DocsNodeKind, VersionMatchState,
};

/// Schema version shared by docs knowledge-surface evidence records.
pub const DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DocsNodeProvenance`] payloads.
pub const DOCS_NODE_PROVENANCE_RECORD_KIND: &str = "docs_node_provenance_record";

/// Stable record-kind tag carried by [`DocsKnowledgeSurfaceProjection`] payloads.
pub const DOCS_KNOWLEDGE_SURFACE_PROJECTION_RECORD_KIND: &str =
    "docs_knowledge_surface_projection_record";

/// Stable record-kind tag carried by [`DocsDerivedExplanation`] payloads.
pub const DOCS_DERIVED_EXPLANATION_RECORD_KIND: &str = "docs_derived_explanation_record";

/// Stable record-kind tag carried by [`DocsKnowledgeSurfaceEvidencePacket`] payloads.
pub const DOCS_KNOWLEDGE_SURFACE_EVIDENCE_PACKET_RECORD_KIND: &str =
    "docs_knowledge_surface_evidence_packet";

/// Knowledge object rendered by a docs-backed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsKnowledgeObjectKind {
    /// Ordinary documentation node or docs browser row.
    DocsNode,
    /// Glossary entry or terminology card.
    GlossaryEntry,
    /// Curated knowledge card such as a tutorial or architecture card.
    CuratedKnowledgeCard,
    /// Derived explanation that cites upstream evidence or marks inference.
    DerivedExplanation,
}

impl DocsKnowledgeObjectKind {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsNode => "docs_node",
            Self::GlossaryEntry => "glossary_entry",
            Self::CuratedKnowledgeCard => "curated_knowledge_card",
            Self::DerivedExplanation => "derived_explanation",
        }
    }
}

impl From<DocsNodeKind> for DocsKnowledgeObjectKind {
    fn from(value: DocsNodeKind) -> Self {
        match value {
            DocsNodeKind::GlossaryItem => Self::GlossaryEntry,
            DocsNodeKind::OnboardingCard | DocsNodeKind::GuidedTourStep => {
                Self::CuratedKnowledgeCard
            }
            DocsNodeKind::DerivedExplainer | DocsNodeKind::AiEvidenceSource => {
                Self::DerivedExplanation
            }
            DocsNodeKind::ProductHelp
            | DocsNodeKind::ReferencePage
            | DocsNodeKind::SupportRunbook => Self::DocsNode,
        }
    }
}

/// Surface family that renders docs evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsKnowledgeSurfaceKind {
    /// In-product docs browser row.
    DocsBrowser,
    /// Glossary card or glossary details drawer.
    GlossaryCard,
    /// Architecture-map node or edge detail.
    ArchitectureMap,
    /// Topology or impact explainer card.
    TopologyExplainer,
    /// Codebase explainer prose or evidence card.
    CodebaseExplainer,
    /// Quick help or contextual help surface.
    QuickHelp,
    /// Docs-backed search result.
    DocsBackedSearch,
    /// Docs-backed AI answer or evidence packet.
    DocsBackedAi,
    /// Support or operator export packet.
    SupportExport,
}

impl DocsKnowledgeSurfaceKind {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::GlossaryCard => "glossary_card",
            Self::ArchitectureMap => "architecture_map",
            Self::TopologyExplainer => "topology_explainer",
            Self::CodebaseExplainer => "codebase_explainer",
            Self::QuickHelp => "quick_help",
            Self::DocsBackedSearch => "docs_backed_search",
            Self::DocsBackedAi => "docs_backed_ai",
            Self::SupportExport => "support_export",
        }
    }
}

/// Mirror, cache, or offline posture for docs knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsMirrorOfflinePosture {
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

impl DocsMirrorOfflinePosture {
    /// Returns the stable string token.
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

    /// Maps the citation locality vocabulary to the knowledge-surface posture.
    pub const fn from_locality(locality: CitationLocalityClass) -> Self {
        match locality {
            CitationLocalityClass::LocalProjectPack => Self::LocalProjectPack,
            CitationLocalityClass::GeneratedLocal => Self::GeneratedLocal,
            CitationLocalityClass::MirroredOffline => Self::OfflinePinnedPack,
            CitationLocalityClass::CachedLocal => Self::CachedLocal,
            CitationLocalityClass::VendorLive => Self::LiveOnline,
            CitationLocalityClass::NotInstalled => Self::NotInstalled,
            CitationLocalityClass::SupportPack => Self::SupportPack,
        }
    }

    /// Returns true when this posture depends on a mirror or offline copy.
    pub const fn is_mirror_or_offline(self) -> bool {
        matches!(self, Self::MirroredPack | Self::OfflinePinnedPack)
    }
}

/// External-open or browser fallback posture for a docs source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsExternalOpenState {
    /// Opening an external source is available through an explicit action.
    Available,
    /// Policy blocks external opening, and the surface must disclose it.
    BlockedByPolicy,
    /// External opening is not required for this source.
    NotRequired,
    /// External opening would be useful but is unavailable.
    Unavailable,
}

impl DocsExternalOpenState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::NotRequired => "not_required",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns true when the state requires a visible disclosure note.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::BlockedByPolicy | Self::Unavailable)
    }
}

/// Browser or source-opening fallback for a docs source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsExternalOpenFallback {
    /// External-open state.
    pub state: DocsExternalOpenState,
    /// Stable action ref used by keyboard, command palette, and exports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_ref: Option<String>,
    /// User-visible action label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_label: Option<String>,
    /// Browser-handoff or source-open packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Disclosure for blocked or unavailable states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
}

impl DocsExternalOpenFallback {
    /// Builds an available external-open fallback.
    pub fn available(
        action_ref: impl Into<String>,
        action_label: impl Into<String>,
        packet_ref: impl Into<String>,
    ) -> Self {
        Self {
            state: DocsExternalOpenState::Available,
            action_ref: Some(action_ref.into()),
            action_label: Some(action_label.into()),
            browser_handoff_packet_ref: Some(packet_ref.into()),
            disclosure_note: None,
        }
    }

    /// Builds a local source posture where external opening is not required.
    pub fn not_required() -> Self {
        Self {
            state: DocsExternalOpenState::NotRequired,
            action_ref: None,
            action_label: None,
            browser_handoff_packet_ref: None,
            disclosure_note: None,
        }
    }

    /// Builds a policy-blocked external-open posture.
    pub fn blocked_by_policy(disclosure_note: impl Into<String>) -> Self {
        Self {
            state: DocsExternalOpenState::BlockedByPolicy,
            action_ref: None,
            action_label: None,
            browser_handoff_packet_ref: None,
            disclosure_note: Some(disclosure_note.into()),
        }
    }

    /// Validates the external-open fallback.
    pub fn validate(&self) -> Vec<DocsEvidenceModelViolation> {
        let mut violations = Vec::new();
        if self.state == DocsExternalOpenState::Available {
            let has_action = self
                .action_ref
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
                && self
                    .action_label
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty())
                && self
                    .browser_handoff_packet_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty());
            if !has_action {
                violations.push(DocsEvidenceModelViolation::ExternalOpenActionMissing);
            }
        }
        if self.state.requires_disclosure()
            && self
                .disclosure_note
                .as_deref()
                .map_or(true, |note| note.trim().is_empty())
        {
            violations.push(DocsEvidenceModelViolation::MissingExternalOpenDisclosure);
        }
        violations
    }
}

/// Example-validation posture for documentation examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsExampleValidationClass {
    /// Examples were validated for the declared version and scope.
    Verified,
    /// Prior validation exists but must be rerun before stable claims continue.
    RetestPending,
    /// Example is illustrative only and must not imply executable guidance.
    IllustrativeOnly,
    /// Validation failed because the example is stale or drifted.
    FailedStale,
    /// The docs node does not contain a validation-bearing example.
    NotApplicable,
}

impl DocsExampleValidationClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::RetestPending => "retest_pending",
            Self::IllustrativeOnly => "illustrative_only",
            Self::FailedStale => "failed_stale",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// User-visible truth label after stale or illustrative downgrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsTruthLabelClass {
    /// Source can be treated as current for the declared scope.
    CurrentDocs,
    /// Source needs a retest or refresh before stable language can continue.
    RetestPending,
    /// Source is useful background or illustrative guidance, not exact truth.
    Illustrative,
}

impl DocsTruthLabelClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentDocs => "current_docs",
            Self::RetestPending => "retest_pending",
            Self::Illustrative => "illustrative",
        }
    }

    /// Returns the visible label used by UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CurrentDocs => "Current docs",
            Self::RetestPending => "Retest pending",
            Self::Illustrative => "Illustrative",
        }
    }
}

/// Downgrade state attached to provenance and export rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsTruthDowngrade {
    /// Truth-label class after downgrade calculation.
    pub label_class: DocsTruthLabelClass,
    /// Visible label.
    pub label: String,
    /// Stable reason tokens explaining why a downgrade occurred.
    pub reason_tokens: Vec<String>,
    /// Repair or refresh action ref, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_action_ref: Option<String>,
}

impl DocsTruthDowngrade {
    fn for_provenance(
        node: &DocsNodeIdentity,
        example_validation: DocsExampleValidationClass,
        derived_kind: Option<DocsDerivedExplanationKind>,
    ) -> Self {
        let mut reasons = Vec::new();
        if node.freshness_class.lowers_certainty() {
            push_unique(&mut reasons, node.freshness_class.as_str());
        }
        if node.version_match_state != VersionMatchState::ExactBuildMatch {
            push_unique(&mut reasons, node.version_match_state.as_str());
        }
        if matches!(
            example_validation,
            DocsExampleValidationClass::RetestPending | DocsExampleValidationClass::FailedStale
        ) {
            push_unique(&mut reasons, example_validation.as_str());
        }
        if node.locality_class == CitationLocalityClass::NotInstalled {
            push_unique(&mut reasons, node.locality_class.as_str());
        }
        let retest_required = !reasons.is_empty();

        if retest_required {
            return Self {
                label_class: DocsTruthLabelClass::RetestPending,
                label: DocsTruthLabelClass::RetestPending.label().to_owned(),
                reason_tokens: reasons,
                repair_action_ref: Some(format!(
                    "action:refresh-docs-evidence:{}",
                    sanitize_ref(&node.docs_node_id)
                )),
            };
        }

        let mut illustrative_reasons = Vec::new();
        if node.citation_availability != CitationAnchorAvailability::ExactAnchorAvailable {
            push_unique(
                &mut illustrative_reasons,
                node.citation_availability.as_str(),
            );
        }
        if example_validation == DocsExampleValidationClass::IllustrativeOnly {
            push_unique(&mut illustrative_reasons, example_validation.as_str());
        }
        if node.source_class == CitationSourceClass::DerivedExplanation {
            push_unique(&mut illustrative_reasons, node.source_class.as_str());
        }
        if matches!(derived_kind, Some(DocsDerivedExplanationKind::Generated)) {
            push_unique(&mut illustrative_reasons, "generated_derived_explanation");
        }
        if !illustrative_reasons.is_empty() {
            return Self {
                label_class: DocsTruthLabelClass::Illustrative,
                label: DocsTruthLabelClass::Illustrative.label().to_owned(),
                reason_tokens: illustrative_reasons,
                repair_action_ref: None,
            };
        }

        Self {
            label_class: DocsTruthLabelClass::CurrentDocs,
            label: DocsTruthLabelClass::CurrentDocs.label().to_owned(),
            reason_tokens: Vec::new(),
            repair_action_ref: None,
        }
    }
}

/// Export-safe infrastructure truth-layer token shared by docs, retrieval-debug, and AI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsInfraTruthLayer {
    /// Repo-authored desired state.
    AuthoredDesired,
    /// Rendered, expanded, or otherwise derived output.
    RenderedExpanded,
    /// Planned or validated output such as dry runs and policy checks.
    PlannedValidated,
    /// Observed or live state from a scoped target.
    ObservedLive,
    /// Provider-owned overlay or vendor-console metadata.
    ProviderOverlay,
}

impl DocsInfraTruthLayer {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredDesired => "authored_desired",
            Self::RenderedExpanded => "rendered_expanded",
            Self::PlannedValidated => "planned_validated",
            Self::ObservedLive => "observed_live",
            Self::ProviderOverlay => "provider_overlay",
        }
    }
}

/// Export-safe infrastructure explanation lineage carried by docs-backed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsInfrastructureLineage {
    /// Stable lineage id.
    pub lineage_ref: String,
    /// Stable infra object or environment-slice subject ref.
    pub subject_ref: String,
    /// Stable context or environment-slice ref when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_ref: Option<String>,
    /// Truth layers explicitly used by the explanation.
    pub truth_layers_used: Vec<DocsInfraTruthLayer>,
    /// Truth layers that were unavailable and therefore surfaced as limits.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unavailable_truth_layers: Vec<DocsInfraTruthLayer>,
    /// Relation or journey refs that make the lineage navigable.
    pub relationship_refs: Vec<String>,
    /// Visible explanation of any limit or fallback posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_limit_summary: Option<String>,
    /// Export-safe explanation summary.
    pub support_summary: String,
}

impl DocsInfrastructureLineage {
    /// Returns the stable truth-layer tokens rendered on source strips and exports.
    pub fn truth_layer_tokens(&self) -> Vec<String> {
        self.truth_layers_used
            .iter()
            .map(|layer| layer.as_str().to_owned())
            .collect()
    }

    /// Returns the unavailable truth-layer tokens rendered as visible limits.
    pub fn unavailable_truth_layer_tokens(&self) -> Vec<String> {
        self.unavailable_truth_layers
            .iter()
            .map(|layer| layer.as_str().to_owned())
            .collect()
    }

    fn validate(&self) -> Vec<DocsEvidenceModelViolation> {
        let mut violations = Vec::new();
        if self.lineage_ref.trim().is_empty()
            || self.subject_ref.trim().is_empty()
            || self.support_summary.trim().is_empty()
            || self.truth_layers_used.is_empty()
            || self.relationship_refs.is_empty()
        {
            violations.push(DocsEvidenceModelViolation::InfrastructureLineageInvalid);
            return violations;
        }

        let mut seen_truth_layers = Vec::new();
        for layer in &self.truth_layers_used {
            if seen_truth_layers.contains(layer) {
                violations.push(DocsEvidenceModelViolation::InfrastructureLineageInvalid);
                break;
            }
            seen_truth_layers.push(*layer);
        }

        let mut seen_unavailable = Vec::new();
        for layer in &self.unavailable_truth_layers {
            if seen_unavailable.contains(layer) || self.truth_layers_used.contains(layer) {
                violations.push(DocsEvidenceModelViolation::InfrastructureLineageInvalid);
                break;
            }
            seen_unavailable.push(*layer);
        }

        let needs_visible_limit = self.unavailable_truth_layers.iter().any(|layer| {
            matches!(
                layer,
                DocsInfraTruthLayer::ObservedLive | DocsInfraTruthLayer::ProviderOverlay
            )
        });
        if needs_visible_limit
            && self
                .visible_limit_summary
                .as_deref()
                .map_or(true, |summary| summary.trim().is_empty())
        {
            violations.push(DocsEvidenceModelViolation::InfrastructureLineageInvalid);
        }
        if needs_visible_limit
            && !self.truth_layers_used.iter().any(|layer| {
                matches!(
                    layer,
                    DocsInfraTruthLayer::AuthoredDesired
                        | DocsInfraTruthLayer::RenderedExpanded
                        | DocsInfraTruthLayer::PlannedValidated
                )
            })
        {
            violations.push(DocsEvidenceModelViolation::InfrastructureLineageInvalid);
        }

        violations
    }
}

/// Derived explanation origin label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsDerivedExplanationKind {
    /// Curated explanation produced by a reviewed knowledge pack or operator.
    Curated,
    /// Generated explanation produced by AI or graph summarization.
    Generated,
}

impl DocsDerivedExplanationKind {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Curated => "curated",
            Self::Generated => "generated",
        }
    }

    /// Returns the visible label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Curated => "Curated explanation",
            Self::Generated => "Generated explanation",
        }
    }
}

/// Provenance record shared by docs knowledge surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsNodeProvenance {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable provenance id.
    pub provenance_id: String,
    /// Stable docs-node identity.
    pub docs_node: DocsNodeIdentity,
    /// Knowledge object class rendered by the surface.
    pub knowledge_object_kind: DocsKnowledgeObjectKind,
    /// Curated or generated label for derived explanations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_explanation_kind: Option<DocsDerivedExplanationKind>,
    /// Source build date or deterministic build stamp.
    pub source_build_at: String,
    /// Running build identity the source was checked against.
    pub running_build_identity_ref: String,
    /// Mirror, cache, or offline posture.
    pub mirror_offline_posture: DocsMirrorOfflinePosture,
    /// External-open or browser fallback state.
    pub external_open: DocsExternalOpenFallback,
    /// Example validation posture for this docs node.
    pub example_validation: DocsExampleValidationClass,
    /// Citation drawer or evidence-view ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_drawer_ref: Option<String>,
    /// Infrastructure explanation lineage when the docs object is infra-aware.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_lineage: Option<DocsInfrastructureLineage>,
    /// Surface refs that consume this provenance record.
    pub surface_refs: Vec<String>,
    /// Truth label after stale or illustrative downgrade calculation.
    pub truth_label: DocsTruthDowngrade,
}

impl DocsNodeProvenance {
    /// Builds provenance with stable record metadata and computed downgrade.
    pub fn new(input: DocsNodeProvenanceInput) -> Self {
        let truth_label = DocsTruthDowngrade::for_provenance(
            &input.docs_node,
            input.example_validation,
            input.derived_explanation_kind,
        );
        Self {
            record_kind: DOCS_NODE_PROVENANCE_RECORD_KIND.to_owned(),
            schema_version: DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION,
            provenance_id: input.provenance_id,
            docs_node: input.docs_node,
            knowledge_object_kind: input.knowledge_object_kind,
            derived_explanation_kind: input.derived_explanation_kind,
            source_build_at: input.source_build_at,
            running_build_identity_ref: input.running_build_identity_ref,
            mirror_offline_posture: input.mirror_offline_posture,
            external_open: input.external_open,
            example_validation: input.example_validation,
            citation_drawer_ref: input.citation_drawer_ref,
            infrastructure_lineage: input.infrastructure_lineage,
            surface_refs: input.surface_refs,
            truth_label,
        }
    }

    /// Returns the source strip rendered by docs/search/help rows.
    pub fn source_strip(&self) -> DocsKnowledgeSourceStrip {
        DocsKnowledgeSourceStrip {
            knowledge_object_kind_token: self.knowledge_object_kind.as_str().to_owned(),
            source_class_token: self.docs_node.source_class.as_str().to_owned(),
            source_pack_ref: self.docs_node.source_pack_ref.clone(),
            source_pack_revision_ref: self.docs_node.source_pack_revision_ref.clone(),
            version_or_revision_ref: self.docs_node.version_or_revision_ref.clone(),
            source_build_at: self.source_build_at.clone(),
            running_build_identity_ref: self.running_build_identity_ref.clone(),
            freshness_class_token: self.docs_node.freshness_class.as_str().to_owned(),
            version_match_state_token: self.docs_node.version_match_state.as_str().to_owned(),
            locality_class_token: self.docs_node.locality_class.as_str().to_owned(),
            mirror_offline_posture_token: self.mirror_offline_posture.as_str().to_owned(),
            source_locale: self.docs_node.source_locale.clone(),
            requested_locale: self.docs_node.requested_locale.clone(),
            effective_locale: self.docs_node.effective_locale.clone(),
            locale_overlay_state_token: self.docs_node.locale_overlay_state.as_str().to_owned(),
            citation_availability_token: self.docs_node.citation_availability.as_str().to_owned(),
            citation_anchor_refs: self.docs_node.citation_anchor_refs.clone(),
            external_open_state_token: self.external_open.state.as_str().to_owned(),
            truth_label_token: self.truth_label.label_class.as_str().to_owned(),
            truth_label: self.truth_label.label.clone(),
            infrastructure_lineage_ref: self
                .infrastructure_lineage
                .as_ref()
                .map(|lineage| lineage.lineage_ref.clone()),
            infrastructure_truth_layer_tokens: self
                .infrastructure_lineage
                .as_ref()
                .map(DocsInfrastructureLineage::truth_layer_tokens)
                .unwrap_or_default(),
            infrastructure_unavailable_truth_layer_tokens: self
                .infrastructure_lineage
                .as_ref()
                .map(DocsInfrastructureLineage::unavailable_truth_layer_tokens)
                .unwrap_or_default(),
            infrastructure_limit_summary: self
                .infrastructure_lineage
                .as_ref()
                .and_then(|lineage| lineage.visible_limit_summary.clone()),
        }
    }

    /// Returns true when source truth degraded to a non-current label.
    pub fn is_downgraded(&self) -> bool {
        self.truth_label.label_class != DocsTruthLabelClass::CurrentDocs
    }

    /// Validates provenance and downgrade invariants.
    pub fn validate(&self) -> Vec<DocsEvidenceModelViolation> {
        let mut violations = Vec::new();
        if self.record_kind != DOCS_NODE_PROVENANCE_RECORD_KIND {
            violations.push(DocsEvidenceModelViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION {
            violations.push(DocsEvidenceModelViolation::WrongSchemaVersion);
        }
        if self.provenance_id.trim().is_empty() || self.surface_refs.is_empty() {
            violations.push(DocsEvidenceModelViolation::MissingStableIdentity);
        }
        if self.source_build_at.trim().is_empty() {
            violations.push(DocsEvidenceModelViolation::MissingSourceBuildDate);
        }
        if self.running_build_identity_ref.trim().is_empty() {
            violations.push(DocsEvidenceModelViolation::MissingRunningBuildIdentityRef);
        }
        for citation_violation in self.docs_node.validate() {
            violations.push(DocsEvidenceModelViolation::from(citation_violation));
        }
        violations.extend(self.external_open.validate());
        if let Some(lineage) = &self.infrastructure_lineage {
            violations.extend(lineage.validate());
        }
        if self.docs_node.source_class == CitationSourceClass::DerivedExplanation
            && self.derived_explanation_kind.is_none()
        {
            violations.push(DocsEvidenceModelViolation::DerivedExplanationUnmarked);
        }
        if self.knowledge_object_kind == DocsKnowledgeObjectKind::DerivedExplanation
            && self.derived_explanation_kind.is_none()
        {
            violations.push(DocsEvidenceModelViolation::DerivedExplanationUnmarked);
        }
        if stale_or_unverified_requires_retest(
            self.docs_node.freshness_class,
            self.docs_node.version_match_state,
            self.example_validation,
        ) && self.truth_label.label_class != DocsTruthLabelClass::RetestPending
        {
            violations.push(DocsEvidenceModelViolation::StaleTruthNotDowngraded);
        }
        if self.docs_node.citation_availability != CitationAnchorAvailability::ExactAnchorAvailable
            && self.truth_label.label_class == DocsTruthLabelClass::CurrentDocs
        {
            violations.push(DocsEvidenceModelViolation::CitationUnavailableNotDowngraded);
        }
        if self.truth_label.label_class != DocsTruthLabelClass::CurrentDocs
            && self.truth_label.reason_tokens.is_empty()
        {
            violations.push(DocsEvidenceModelViolation::MissingDowngradeReason);
        }
        violations
    }
}

/// Constructor fields for [`DocsNodeProvenance`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsNodeProvenanceInput {
    /// Stable provenance id.
    pub provenance_id: String,
    /// Stable docs-node identity.
    pub docs_node: DocsNodeIdentity,
    /// Knowledge object kind.
    pub knowledge_object_kind: DocsKnowledgeObjectKind,
    /// Curated or generated label for derived explanations.
    pub derived_explanation_kind: Option<DocsDerivedExplanationKind>,
    /// Source build date or deterministic build stamp.
    pub source_build_at: String,
    /// Running build identity the source was checked against.
    pub running_build_identity_ref: String,
    /// Mirror, cache, or offline posture.
    pub mirror_offline_posture: DocsMirrorOfflinePosture,
    /// External-open or browser fallback state.
    pub external_open: DocsExternalOpenFallback,
    /// Example validation posture.
    pub example_validation: DocsExampleValidationClass,
    /// Citation drawer or evidence-view ref.
    pub citation_drawer_ref: Option<String>,
    /// Infrastructure explanation lineage when the docs object is infra-aware.
    pub infrastructure_lineage: Option<DocsInfrastructureLineage>,
    /// Surface refs that consume this provenance record.
    pub surface_refs: Vec<String>,
}

/// Compact source strip shared by docs/browser/search/AI rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsKnowledgeSourceStrip {
    /// Knowledge object kind token.
    pub knowledge_object_kind_token: String,
    /// Source class token.
    pub source_class_token: String,
    /// Source pack ref.
    pub source_pack_ref: String,
    /// Source pack revision ref.
    pub source_pack_revision_ref: String,
    /// Version or revision ref.
    pub version_or_revision_ref: String,
    /// Source build date or deterministic stamp.
    pub source_build_at: String,
    /// Running build identity ref.
    pub running_build_identity_ref: String,
    /// Freshness token.
    pub freshness_class_token: String,
    /// Version-match token.
    pub version_match_state_token: String,
    /// Locality token.
    pub locality_class_token: String,
    /// Mirror/offline posture token.
    pub mirror_offline_posture_token: String,
    /// Canonical source locale.
    pub source_locale: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective locale.
    pub effective_locale: String,
    /// Locale overlay token.
    pub locale_overlay_state_token: String,
    /// Citation availability token.
    pub citation_availability_token: String,
    /// Citation anchor refs.
    pub citation_anchor_refs: Vec<String>,
    /// External-open state token.
    pub external_open_state_token: String,
    /// Truth-label token.
    pub truth_label_token: String,
    /// Visible truth label.
    pub truth_label: String,
    /// Infrastructure lineage ref when the row is infra-aware.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_lineage_ref: Option<String>,
    /// Infrastructure truth-layer tokens cited by the row.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub infrastructure_truth_layer_tokens: Vec<String>,
    /// Infrastructure truth-layer tokens that were unavailable and surfaced as limits.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub infrastructure_unavailable_truth_layer_tokens: Vec<String>,
    /// Visible infrastructure limit summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_limit_summary: Option<String>,
}

/// Render projection for a docs-backed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsKnowledgeSurfaceProjection {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Surface family.
    pub surface_kind: DocsKnowledgeSurfaceKind,
    /// Stable surface row/card id.
    pub surface_id_ref: String,
    /// Provenance consumed by this surface.
    pub provenance: DocsNodeProvenance,
    /// Source strip rendered by the surface.
    pub source_strip: DocsKnowledgeSourceStrip,
    /// Keyboard-accessible citation inspection action.
    pub citation_inspection_action_ref: String,
    /// Open supporting source action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_supporting_source_action_ref: Option<String>,
    /// True when the citation and source actions are keyboard reachable.
    pub keyboard_accessible_actions: bool,
    /// Export packet refs preserving this projection.
    pub export_packet_refs: Vec<String>,
}

impl DocsKnowledgeSurfaceProjection {
    /// Builds a surface projection from provenance.
    pub fn new(input: DocsKnowledgeSurfaceProjectionInput) -> Self {
        let source_strip = input.provenance.source_strip();
        Self {
            record_kind: DOCS_KNOWLEDGE_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION,
            surface_kind: input.surface_kind,
            surface_id_ref: input.surface_id_ref,
            provenance: input.provenance,
            source_strip,
            citation_inspection_action_ref: input.citation_inspection_action_ref,
            open_supporting_source_action_ref: input.open_supporting_source_action_ref,
            keyboard_accessible_actions: input.keyboard_accessible_actions,
            export_packet_refs: input.export_packet_refs,
        }
    }

    /// Validates projection action and provenance invariants.
    pub fn validate(&self) -> Vec<DocsEvidenceModelViolation> {
        let mut violations = Vec::new();
        if self.record_kind != DOCS_KNOWLEDGE_SURFACE_PROJECTION_RECORD_KIND {
            violations.push(DocsEvidenceModelViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION {
            violations.push(DocsEvidenceModelViolation::WrongSchemaVersion);
        }
        if self.surface_id_ref.trim().is_empty() {
            violations.push(DocsEvidenceModelViolation::MissingStableIdentity);
        }
        violations.extend(self.provenance.validate());
        if self.citation_inspection_action_ref.trim().is_empty() {
            violations.push(DocsEvidenceModelViolation::MissingCitationInspectionAction);
        }
        if !self.keyboard_accessible_actions {
            violations.push(DocsEvidenceModelViolation::KeyboardInspectionUnavailable);
        }
        if self.provenance.external_open.state == DocsExternalOpenState::Available
            && self
                .open_supporting_source_action_ref
                .as_deref()
                .map_or(true, |value| value.trim().is_empty())
        {
            violations.push(DocsEvidenceModelViolation::ExternalOpenActionMissing);
        }
        violations
    }
}

/// Constructor fields for [`DocsKnowledgeSurfaceProjection`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsKnowledgeSurfaceProjectionInput {
    /// Surface family.
    pub surface_kind: DocsKnowledgeSurfaceKind,
    /// Stable surface row/card id.
    pub surface_id_ref: String,
    /// Provenance consumed by this surface.
    pub provenance: DocsNodeProvenance,
    /// Keyboard-accessible citation inspection action.
    pub citation_inspection_action_ref: String,
    /// Open supporting source action.
    pub open_supporting_source_action_ref: Option<String>,
    /// True when the citation and source actions are keyboard reachable.
    pub keyboard_accessible_actions: bool,
    /// Export packet refs preserving this projection.
    pub export_packet_refs: Vec<String>,
}

/// Claim kind inside a derived explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsDerivedClaimKind {
    /// Direct fact claim based on cited source material.
    FactClaim,
    /// Inference that bridges sources.
    InferenceClaim,
    /// Procedure, step, or task guidance.
    ProcedureOrStep,
    /// Definition or glossary text.
    DefinitionOrGlossary,
    /// Architecture, topology, or relation summary.
    ArchitectureRelation,
}

impl DocsDerivedClaimKind {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FactClaim => "fact_claim",
            Self::InferenceClaim => "inference_claim",
            Self::ProcedureOrStep => "procedure_or_step",
            Self::DefinitionOrGlossary => "definition_or_glossary",
            Self::ArchitectureRelation => "architecture_relation",
        }
    }
}

/// One claim rendered by a derived explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsDerivedExplanationClaim {
    /// Stable claim id.
    pub claim_id: String,
    /// Claim kind.
    pub claim_kind: DocsDerivedClaimKind,
    /// Export-safe claim summary.
    pub claim_summary: String,
    /// Supporting citation anchor refs.
    pub supporting_citation_anchor_refs: Vec<String>,
    /// Inference marker for uncited or synthesized claims.
    pub inference_marker: CitationInferenceMarker,
    /// Confidence class for this claim.
    pub confidence_class: CitationConfidenceClass,
}

impl DocsDerivedExplanationClaim {
    /// Returns true when the claim is explicitly supported by citations.
    pub fn has_supporting_citation(&self) -> bool {
        !self.supporting_citation_anchor_refs.is_empty()
    }

    fn validate(&self) -> Vec<DocsEvidenceModelViolation> {
        let mut violations = Vec::new();
        if self.claim_id.trim().is_empty() || self.claim_summary.trim().is_empty() {
            violations.push(DocsEvidenceModelViolation::MissingStableIdentity);
        }
        if !self.has_supporting_citation()
            && self.inference_marker == CitationInferenceMarker::RawSource
        {
            violations.push(DocsEvidenceModelViolation::DerivedClaimUncited);
        }
        if !self.has_supporting_citation()
            && self.confidence_class == CitationConfidenceClass::EvidenceBacked
        {
            violations.push(DocsEvidenceModelViolation::DerivedClaimUncited);
        }
        violations
    }
}

/// Derived explanation record with provenance, claims, and upstream citations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsDerivedExplanation {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable explanation id.
    pub explanation_id: String,
    /// Surface family that renders the explanation.
    pub surface_kind: DocsKnowledgeSurfaceKind,
    /// Curated or generated label.
    pub derived_explanation_kind: DocsDerivedExplanationKind,
    /// Export-safe summary label.
    pub summary_label: String,
    /// Provenance for the explanation node.
    pub provenance: DocsNodeProvenance,
    /// Upstream citation anchors retained for reconstruction.
    pub upstream_citation_anchor_refs: Vec<String>,
    /// Upstream docs-node refs used by the explanation.
    pub cited_docs_node_refs: Vec<String>,
    /// Claim rows rendered or exported by the explanation.
    pub claims: Vec<DocsDerivedExplanationClaim>,
    /// Graph epoch or docs query epoch used by the explanation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derivation_epoch_ref: Option<String>,
    /// Docs pack refs used as input.
    pub docs_pack_refs: Vec<String>,
}

impl DocsDerivedExplanation {
    /// Builds a derived explanation with stable record metadata.
    pub fn new(input: DocsDerivedExplanationInput) -> Self {
        Self {
            record_kind: DOCS_DERIVED_EXPLANATION_RECORD_KIND.to_owned(),
            schema_version: DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION,
            explanation_id: input.explanation_id,
            surface_kind: input.surface_kind,
            derived_explanation_kind: input.derived_explanation_kind,
            summary_label: input.summary_label,
            provenance: input.provenance,
            upstream_citation_anchor_refs: input.upstream_citation_anchor_refs,
            cited_docs_node_refs: input.cited_docs_node_refs,
            claims: input.claims,
            derivation_epoch_ref: input.derivation_epoch_ref,
            docs_pack_refs: input.docs_pack_refs,
        }
    }

    /// Validates derived-explanation citation and labeling invariants.
    pub fn validate(&self) -> Vec<DocsEvidenceModelViolation> {
        let mut violations = Vec::new();
        if self.record_kind != DOCS_DERIVED_EXPLANATION_RECORD_KIND {
            violations.push(DocsEvidenceModelViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION {
            violations.push(DocsEvidenceModelViolation::WrongSchemaVersion);
        }
        if self.explanation_id.trim().is_empty()
            || self.summary_label.trim().is_empty()
            || self.claims.is_empty()
            || self.upstream_citation_anchor_refs.is_empty()
        {
            violations.push(DocsEvidenceModelViolation::MissingStableIdentity);
        }
        if self.provenance.derived_explanation_kind != Some(self.derived_explanation_kind) {
            violations.push(DocsEvidenceModelViolation::DerivedExplanationUnmarked);
        }
        violations.extend(self.provenance.validate());
        for claim in &self.claims {
            violations.extend(claim.validate());
        }
        violations
    }
}

/// Constructor fields for [`DocsDerivedExplanation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsDerivedExplanationInput {
    /// Stable explanation id.
    pub explanation_id: String,
    /// Surface family that renders the explanation.
    pub surface_kind: DocsKnowledgeSurfaceKind,
    /// Curated or generated label.
    pub derived_explanation_kind: DocsDerivedExplanationKind,
    /// Export-safe summary label.
    pub summary_label: String,
    /// Provenance for the explanation node.
    pub provenance: DocsNodeProvenance,
    /// Upstream citation anchors retained for reconstruction.
    pub upstream_citation_anchor_refs: Vec<String>,
    /// Upstream docs-node refs used by the explanation.
    pub cited_docs_node_refs: Vec<String>,
    /// Claim rows rendered or exported by the explanation.
    pub claims: Vec<DocsDerivedExplanationClaim>,
    /// Graph epoch or docs query epoch used by the explanation.
    pub derivation_epoch_ref: Option<String>,
    /// Docs pack refs used as input.
    pub docs_pack_refs: Vec<String>,
}

/// Export-safe docs evidence packet shared by operators, support, and AI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsKnowledgeSurfaceEvidencePacket {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Surface or workflow that produced the packet.
    pub surface_ref: String,
    /// Export generation timestamp or deterministic fixture stamp.
    pub generated_at: String,
    /// Surface projections preserved by the packet.
    pub surface_projections: Vec<DocsKnowledgeSurfaceProjection>,
    /// Derived explanations preserved by the packet.
    pub derived_explanations: Vec<DocsDerivedExplanation>,
    /// AI evidence packet refs that used this docs evidence.
    pub ai_evidence_packet_refs: Vec<String>,
    /// Graph or explainer packet refs that used this docs evidence.
    pub explainer_packet_refs: Vec<String>,
    /// Operator or support packet refs that copied this evidence.
    pub operator_support_packet_refs: Vec<String>,
}

impl DocsKnowledgeSurfaceEvidencePacket {
    /// Builds an evidence packet with stable record metadata.
    pub fn new(input: DocsKnowledgeSurfaceEvidencePacketInput) -> Self {
        Self {
            record_kind: DOCS_KNOWLEDGE_SURFACE_EVIDENCE_PACKET_RECORD_KIND.to_owned(),
            schema_version: DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_ref: input.surface_ref,
            generated_at: input.generated_at,
            surface_projections: input.surface_projections,
            derived_explanations: input.derived_explanations,
            ai_evidence_packet_refs: input.ai_evidence_packet_refs,
            explainer_packet_refs: input.explainer_packet_refs,
            operator_support_packet_refs: input.operator_support_packet_refs,
        }
    }

    /// Validates every nested projection and derived explanation.
    pub fn validate(&self) -> Vec<DocsEvidenceModelViolation> {
        let mut violations = Vec::new();
        if self.record_kind != DOCS_KNOWLEDGE_SURFACE_EVIDENCE_PACKET_RECORD_KIND {
            violations.push(DocsEvidenceModelViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_KNOWLEDGE_SURFACE_SCHEMA_VERSION {
            violations.push(DocsEvidenceModelViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_ref.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            violations.push(DocsEvidenceModelViolation::MissingStableIdentity);
        }
        if self.surface_projections.is_empty() {
            violations.push(DocsEvidenceModelViolation::ExportMissingProvenance);
        }
        for projection in &self.surface_projections {
            violations.extend(projection.validate());
        }
        for explanation in &self.derived_explanations {
            violations.extend(explanation.validate());
        }
        violations
    }

    /// Deterministic JSON serialization for support/export fixtures.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs evidence packet serializes")
    }
}

/// Constructor fields for [`DocsKnowledgeSurfaceEvidencePacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsKnowledgeSurfaceEvidencePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Surface or workflow that produced the packet.
    pub surface_ref: String,
    /// Export generation timestamp or deterministic fixture stamp.
    pub generated_at: String,
    /// Surface projections preserved by the packet.
    pub surface_projections: Vec<DocsKnowledgeSurfaceProjection>,
    /// Derived explanations preserved by the packet.
    pub derived_explanations: Vec<DocsDerivedExplanation>,
    /// AI evidence packet refs that used this docs evidence.
    pub ai_evidence_packet_refs: Vec<String>,
    /// Graph or explainer packet refs that used this docs evidence.
    pub explainer_packet_refs: Vec<String>,
    /// Operator or support packet refs that copied this evidence.
    pub operator_support_packet_refs: Vec<String>,
}

/// Validation failure emitted by the docs knowledge-surface evidence model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsEvidenceModelViolation {
    /// Record kind did not match the expected discriminator.
    WrongRecordKind,
    /// Schema version did not match the supported version.
    WrongSchemaVersion,
    /// A required stable identity field was empty.
    MissingStableIdentity,
    /// Source build date or deterministic build stamp was missing.
    MissingSourceBuildDate,
    /// Running build identity ref was missing.
    MissingRunningBuildIdentityRef,
    /// External-open fallback lacked a required disclosure note.
    MissingExternalOpenDisclosure,
    /// Available external-open fallback lacked an action or packet ref.
    ExternalOpenActionMissing,
    /// Citation inspection action was missing.
    MissingCitationInspectionAction,
    /// Citation/source actions were not keyboard reachable.
    KeyboardInspectionUnavailable,
    /// Stale or unverified source truth did not downgrade.
    StaleTruthNotDowngraded,
    /// Missing or unavailable citation truth did not downgrade.
    CitationUnavailableNotDowngraded,
    /// Downgraded truth lacked a stable reason token.
    MissingDowngradeReason,
    /// Infrastructure lineage was incomplete, contradictory, or hid its fallback posture.
    InfrastructureLineageInvalid,
    /// A derived explanation lacked curated/generated labeling.
    DerivedExplanationUnmarked,
    /// A derived claim lacked citations and inference labeling.
    DerivedClaimUncited,
    /// Export packet contained no provenance rows.
    ExportMissingProvenance,
    /// Nested citation truth validation failed.
    CitationTruthInvalid,
}

impl DocsEvidenceModelViolation {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingStableIdentity => "missing_stable_identity",
            Self::MissingSourceBuildDate => "missing_source_build_date",
            Self::MissingRunningBuildIdentityRef => "missing_running_build_identity_ref",
            Self::MissingExternalOpenDisclosure => "missing_external_open_disclosure",
            Self::ExternalOpenActionMissing => "external_open_action_missing",
            Self::MissingCitationInspectionAction => "missing_citation_inspection_action",
            Self::KeyboardInspectionUnavailable => "keyboard_inspection_unavailable",
            Self::StaleTruthNotDowngraded => "stale_truth_not_downgraded",
            Self::CitationUnavailableNotDowngraded => "citation_unavailable_not_downgraded",
            Self::MissingDowngradeReason => "missing_downgrade_reason",
            Self::InfrastructureLineageInvalid => "infrastructure_lineage_invalid",
            Self::DerivedExplanationUnmarked => "derived_explanation_unmarked",
            Self::DerivedClaimUncited => "derived_claim_uncited",
            Self::ExportMissingProvenance => "export_missing_provenance",
            Self::CitationTruthInvalid => "citation_truth_invalid",
        }
    }
}

impl From<CitationTruthViolation> for DocsEvidenceModelViolation {
    fn from(_: CitationTruthViolation) -> Self {
        Self::CitationTruthInvalid
    }
}

fn stale_or_unverified_requires_retest(
    freshness: DocsFreshnessClass,
    version_match: VersionMatchState,
    example_validation: DocsExampleValidationClass,
) -> bool {
    freshness.lowers_certainty()
        || version_match != VersionMatchState::ExactBuildMatch
        || matches!(
            example_validation,
            DocsExampleValidationClass::RetestPending | DocsExampleValidationClass::FailedStale
        )
}

fn push_unique(target: &mut Vec<String>, value: &str) {
    if !target.iter().any(|existing| existing == value) {
        target.push(value.to_owned());
    }
}

fn sanitize_ref(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect()
}
