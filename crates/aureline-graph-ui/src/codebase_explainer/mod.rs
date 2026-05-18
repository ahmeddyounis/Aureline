//! Cited codebase-explainer projections.
//!
//! A [`CodebaseExplainerSurface`] is intentionally claim-row first. Summary
//! prose is derived from cited claims and every non-trivial claim keeps its
//! citations, generated-vs-curated label, scope disclosure, and omissions in
//! the exported packet.

use aureline_graph::{EvidenceCitation, ExplainerSourceKind, ImpactExplainerPacket};
use serde::{Deserialize, Serialize};

use crate::topology::{GraphDisclosureState, ScopeVocabularyClass, SurfaceAction};

/// Schema version for codebase explainer surfaces.
pub const CODEBASE_EXPLAINER_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for codebase explainer surfaces.
pub const CODEBASE_EXPLAINER_SURFACE_RECORD_KIND: &str = "codebase_explainer_surface_record";

/// Whether explainer text is curated or generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainerTextSource {
    /// Text copied from curated source artifacts.
    Curated,
    /// Text generated from cited evidence.
    Generated,
}

impl ExplainerTextSource {
    /// Returns the stable token used in serialized surface packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Curated => "curated",
            Self::Generated => "generated",
        }
    }

    /// Maps graph explainer source labels into the surface vocabulary.
    pub const fn from_graph_source(source: ExplainerSourceKind) -> Self {
        match source {
            ExplainerSourceKind::Curated => Self::Curated,
            ExplainerSourceKind::Generated => Self::Generated,
        }
    }
}

/// Citation reference carried by a claim row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationRef {
    /// Stable citation id.
    pub citation_id: String,
    /// Cited subject kind.
    pub subject_kind: String,
    /// Cited graph or source ref.
    pub subject_ref: String,
    /// Generated-vs-curated source label.
    pub source_kind: ExplainerTextSource,
    /// Redaction-aware display label.
    pub display_label: String,
    /// Freshness token.
    pub freshness: String,
    /// Confidence token.
    pub confidence: String,
}

/// One claim row in the codebase explainer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainerClaim {
    /// Stable claim id.
    pub claim_id: String,
    /// Whether the claim text is curated or generated.
    pub source: ExplainerTextSource,
    /// Redaction-aware statement rendered by the product.
    pub statement: String,
    /// Citation refs that support the statement.
    pub citations: Vec<CitationRef>,
    /// Shared scope vocabulary class.
    pub scope_class: ScopeVocabularyClass,
    /// Omission disclosures attached to this claim.
    pub omission_disclosures: Vec<GraphDisclosureState>,
}

impl ExplainerClaim {
    /// Returns true when this claim asserts non-trivial prose and must cite evidence.
    pub fn requires_citation(&self) -> bool {
        !self.statement.trim().is_empty()
            && !self
                .omission_disclosures
                .iter()
                .any(|state| *state == GraphDisclosureState::PolicyHidden)
    }
}

/// Export packet declaration for cited explainers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplainerExportPacket {
    /// Stable packet id.
    pub packet_id: String,
    /// Export format token.
    pub export_format: String,
    /// Whether citation ids and anchors survive the export.
    pub preserves_citations: bool,
    /// Whether omission disclosures survive the export.
    pub preserves_omissions: bool,
    /// Whether generated-vs-curated labels survive the export.
    pub preserves_source_labels: bool,
}

/// Exportable codebase explainer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodebaseExplainerSurface {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable surface id.
    pub surface_id: String,
    /// Subject graph node id.
    pub subject_node_id: String,
    /// Shared scope vocabulary class.
    pub scope_class: ScopeVocabularyClass,
    /// Top-of-pane generated-vs-curated label.
    pub generated_vs_curated_label: ExplainerTextSource,
    /// Explicit scope and omission disclosures.
    pub scope_disclosures: Vec<GraphDisclosureState>,
    /// Claim rows backing all explainer prose.
    pub claims: Vec<ExplainerClaim>,
    /// Packet-level omission disclosures.
    pub omissions: Vec<GraphDisclosureState>,
    /// Open-evidence actions.
    pub evidence_actions: Vec<SurfaceAction>,
    /// Export packet declarations.
    pub export_packets: Vec<ExplainerExportPacket>,
}

impl CodebaseExplainerSurface {
    /// Builds a cited codebase explainer from an impact explainer packet.
    pub fn from_impact_packet(packet: &ImpactExplainerPacket) -> Self {
        let scope_class =
            ScopeVocabularyClass::from_graph_scope_token(&packet.workset_scope.scope_class);
        let source = ExplainerTextSource::from_graph_source(
            packet.evidence_card.generated_vs_curated_source,
        );
        let citations = packet
            .evidence_card
            .citations
            .iter()
            .map(citation_ref)
            .collect::<Vec<_>>();
        let omissions = omission_disclosures(packet);
        let claim = ExplainerClaim {
            claim_id: format!("claim:explainer:{}", packet.query_request_id),
            source,
            statement: format!(
                "This area has {} visible impact edge(s) in {} and {} impact edge(s) outside the active scope.",
                packet.impact_summary.visible_impact_edge_count,
                scope_class.label(),
                packet.impact_summary.out_of_scope_count
            ),
            citations,
            scope_class,
            omission_disclosures: omissions.clone(),
        };
        let evidence_actions = packet
            .evidence_card
            .open_detail_actions
            .iter()
            .map(|action| SurfaceAction {
                action_id: action.action_id.clone(),
                action_class: action.action_class.as_str().to_owned(),
                subject_ref: action.subject_ref.clone(),
                preserves_scope: action.preserves_scope,
            })
            .collect::<Vec<_>>();

        Self {
            record_kind: CODEBASE_EXPLAINER_SURFACE_RECORD_KIND.to_owned(),
            schema_version: CODEBASE_EXPLAINER_SURFACE_SCHEMA_VERSION,
            surface_id: format!("surface:codebase_explainer:{}", packet.query_request_id),
            subject_node_id: packet.subject_node_id.clone(),
            scope_class,
            generated_vs_curated_label: source,
            scope_disclosures: omissions.clone(),
            claims: vec![claim],
            omissions,
            evidence_actions,
            export_packets: vec![
                ExplainerExportPacket {
                    packet_id: format!("export:explainer:{}:json", packet.query_request_id),
                    export_format: "json_explainer_snapshot".to_owned(),
                    preserves_citations: true,
                    preserves_omissions: true,
                    preserves_source_labels: true,
                },
                ExplainerExportPacket {
                    packet_id: format!("export:explainer:{}:markdown", packet.query_request_id),
                    export_format: "markdown_explainer_brief".to_owned(),
                    preserves_citations: false,
                    preserves_omissions: false,
                    preserves_source_labels: true,
                },
            ],
        }
    }

    /// Verifies every non-trivial claim is cited.
    pub fn validate_cited_claims(&self) -> Result<(), CitationValidationError> {
        for claim in &self.claims {
            if claim.requires_citation() && claim.citations.is_empty() {
                return Err(CitationValidationError::UncitedClaim {
                    claim_id: claim.claim_id.clone(),
                });
            }
        }
        Ok(())
    }
}

/// Error returned when an explainer claim drops required citations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CitationValidationError {
    /// A non-trivial claim did not carry citations.
    UncitedClaim {
        /// Claim id that failed validation.
        claim_id: String,
    },
}

fn citation_ref(citation: &EvidenceCitation) -> CitationRef {
    CitationRef {
        citation_id: citation.citation_id.clone(),
        subject_kind: citation.subject_kind.clone(),
        subject_ref: citation.subject_ref.clone(),
        source_kind: ExplainerTextSource::from_graph_source(citation.source_kind),
        display_label: citation.display_label.clone(),
        freshness: citation.freshness.clone(),
        confidence: citation.confidence.clone(),
    }
}

fn omission_disclosures(packet: &ImpactExplainerPacket) -> Vec<GraphDisclosureState> {
    let mut omissions = Vec::new();
    if packet.impact_summary.out_of_scope_count > 0 || packet.workset_scope.hidden_result_count > 0
    {
        omissions.push(GraphDisclosureState::OutsideCurrentScope);
    }
    if packet.workset_scope.index_coverage.not_loaded_count > 0 {
        omissions.push(GraphDisclosureState::PartialGraph);
    }
    for cause in &packet.partial_truth_causes {
        match cause.as_str() {
            "imported" | "replayed" => omissions.push(GraphDisclosureState::ImportedFact),
            "policy_hidden" => omissions.push(GraphDisclosureState::PolicyHidden),
            "derived" | "missing_anchor" | "low_confidence" => {
                omissions.push(GraphDisclosureState::GeneratedOrHeuristic);
            }
            "partial_scope" | "warming" | "stale" => {
                omissions.push(GraphDisclosureState::PartialGraph)
            }
            _ => {}
        }
    }
    omissions.sort_by_key(|state| state.as_str());
    omissions.dedup();
    omissions
}
