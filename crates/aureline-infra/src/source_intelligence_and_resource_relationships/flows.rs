//! Shared cross-surface infrastructure relation journeys and environment-slice explainers.
//!
//! These records let code/infra, review, docs, and incident surfaces resolve
//! the same `show live counterpart`, `show applied-by`, `show owned-by`,
//! `show impacts`, and `explain this environment slice` flows from the shared
//! [`SourceIntelligenceObjectPacket`] rather than rebuilding private caches.

use serde::{Deserialize, Serialize};

use crate::{
    ActionPosture, FreshnessLabel, InfrastructureConsumerProjection, InfrastructureConsumerSurface,
    InfrastructureFamily, InfrastructureObjectRelationRecord, RelationEdgeClass,
    SourceIntelligenceObjectPacket, TruthLayer,
};

/// Surface vocabulary used by the shared infrastructure relation journeys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureJourneySurface {
    /// Code editor, infra editor, or relation-graph view.
    CodeInfraView,
    /// Review workspace or approval surface.
    ReviewWorkspace,
    /// Incident timeline or incident workspace.
    IncidentTimeline,
    /// Docs card or codebase-understanding surface.
    DocsCards,
}

impl InfrastructureJourneySurface {
    /// Returns the stable token used by exported journey records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeInfraView => "code_infra_view",
            Self::ReviewWorkspace => "review_workspace",
            Self::IncidentTimeline => "incident_timeline",
            Self::DocsCards => "docs_cards",
        }
    }

    pub(super) const fn consumer_surface(self) -> InfrastructureConsumerSurface {
        match self {
            Self::CodeInfraView => InfrastructureConsumerSurface::Graph,
            Self::ReviewWorkspace => InfrastructureConsumerSurface::Review,
            Self::IncidentTimeline => InfrastructureConsumerSurface::Incident,
            Self::DocsCards => InfrastructureConsumerSurface::Docs,
        }
    }
}

/// Journey action resolved from the shared infrastructure packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureJourneyKind {
    /// Resolve the live resource or object counterpart for the subject.
    ShowLiveCounterpart,
    /// Resolve the apply, reconcile, or provider-owned handoff evidence.
    ShowAppliedBy,
    /// Resolve the ownership or controller edge for the subject.
    ShowOwnedBy,
    /// Resolve the impacted environment or runtime slice for the subject.
    ShowImpacts,
}

impl InfrastructureJourneyKind {
    /// Returns the stable token used by exported journey records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShowLiveCounterpart => "show_live_counterpart",
            Self::ShowAppliedBy => "show_applied_by",
            Self::ShowOwnedBy => "show_owned_by",
            Self::ShowImpacts => "show_impacts",
        }
    }
}

/// Labeled result state for a journey or environment-slice explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureJourneyStatus {
    /// The requested flow resolved from explicit packet relations.
    Resolved,
    /// The subject exists, but the claimed surface did not project the needed rows.
    OutOfScope,
    /// The subject object id is unknown to the packet.
    UnknownSubjectObject,
    /// No explicit live-counterpart relation exists for the subject.
    MissingLiveCounterpart,
    /// No explicit applied-by relation exists for the subject.
    MissingAppliedBy,
    /// No explicit owned-by relation exists for the subject.
    MissingOwnedBy,
    /// No explicit impacts relation exists for the subject.
    MissingImpacts,
    /// The selected context has no projected objects or relations on this surface.
    MissingEnvironmentSliceCoverage,
    /// The requested flow resolved, but freshness is below current/live posture.
    FreshnessReviewRequired,
}

/// One resolved or labeled infrastructure relation journey.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureRelationJourney {
    /// Stable synthetic journey ref.
    pub journey_ref: String,
    /// Flow kind this journey resolved.
    pub journey_kind: InfrastructureJourneyKind,
    /// Consumer surface that requested the journey.
    pub surface: InfrastructureJourneySurface,
    /// Surface projection ref that backed the journey.
    pub source_projection_ref: String,
    /// Subject object id.
    pub subject_object_ref: String,
    /// Subject context id.
    pub subject_context_ref: Option<String>,
    /// Resolution status for the journey.
    pub status: InfrastructureJourneyStatus,
    /// Relation ids that backed the result.
    pub relation_refs: Vec<String>,
    /// Target object ids reached by the journey.
    pub target_object_refs: Vec<String>,
    /// Truth layers used by the journey.
    pub truth_layers_used: Vec<TruthLayer>,
    /// Freshness labels preserved by the journey.
    pub freshness_labels: Vec<FreshnessLabel>,
    /// Authority postures preserved by the journey.
    pub authority_postures: Vec<ActionPosture>,
    /// Export-safe journey summary.
    pub support_summary: String,
}

/// Shared explanation of one environment slice on a claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfrastructureEnvironmentSliceExplanation {
    /// Stable synthetic slice ref.
    pub slice_ref: String,
    /// Consumer surface that requested the explanation.
    pub surface: InfrastructureJourneySurface,
    /// Surface projection ref that backed the explanation.
    pub source_projection_ref: String,
    /// Context id being explained.
    pub context_ref: String,
    /// Resolution status for the slice explanation.
    pub status: InfrastructureJourneyStatus,
    /// Infrastructure families covered by the slice.
    pub families: Vec<InfrastructureFamily>,
    /// Object ids participating in the slice.
    pub object_refs: Vec<String>,
    /// Relation ids participating in the slice.
    pub relation_refs: Vec<String>,
    /// Truth layers visible in the slice.
    pub truth_layers: Vec<TruthLayer>,
    /// Freshness labels visible in the slice.
    pub freshness_labels: Vec<FreshnessLabel>,
    /// `live_counterpart_of` relation ids in the slice.
    pub live_counterpart_relation_refs: Vec<String>,
    /// `applied_by` relation ids in the slice.
    pub applied_by_relation_refs: Vec<String>,
    /// `owned_by` relation ids in the slice.
    pub owned_by_relation_refs: Vec<String>,
    /// `impacts` relation ids in the slice.
    pub impact_relation_refs: Vec<String>,
    /// `runbook_reference` relation ids in the slice.
    pub runbook_relation_refs: Vec<String>,
    /// `review_anchor` relation ids in the slice.
    pub review_anchor_relation_refs: Vec<String>,
    /// Export-safe explanation summary.
    pub support_summary: String,
}

/// Surface-scoped view into one shared infrastructure object packet.
#[derive(Debug, Clone, Copy)]
pub struct InfrastructureSurfaceView<'a> {
    packet: &'a SourceIntelligenceObjectPacket,
    projection: &'a InfrastructureConsumerProjection,
    surface: InfrastructureJourneySurface,
}

impl<'a> InfrastructureSurfaceView<'a> {
    /// Returns the stable surface vocabulary this view uses.
    pub const fn surface(&self) -> InfrastructureJourneySurface {
        self.surface
    }

    /// Returns the shared projection id used by this view.
    pub fn projection_id(&self) -> &str {
        &self.projection.projection_id
    }

    /// Resolves `show live counterpart` from the shared packet.
    pub fn show_live_counterpart(&self, subject_object_ref: &str) -> InfrastructureRelationJourney {
        self.resolve_relation_journey(
            subject_object_ref,
            InfrastructureJourneyKind::ShowLiveCounterpart,
            RelationEdgeClass::LiveCounterpartOf,
            InfrastructureJourneyStatus::MissingLiveCounterpart,
        )
    }

    /// Resolves `show applied-by` from the shared packet.
    pub fn show_applied_by(&self, subject_object_ref: &str) -> InfrastructureRelationJourney {
        self.resolve_relation_journey(
            subject_object_ref,
            InfrastructureJourneyKind::ShowAppliedBy,
            RelationEdgeClass::AppliedBy,
            InfrastructureJourneyStatus::MissingAppliedBy,
        )
    }

    /// Resolves `show owned-by` from the shared packet.
    pub fn show_owned_by(&self, subject_object_ref: &str) -> InfrastructureRelationJourney {
        self.resolve_relation_journey(
            subject_object_ref,
            InfrastructureJourneyKind::ShowOwnedBy,
            RelationEdgeClass::OwnedBy,
            InfrastructureJourneyStatus::MissingOwnedBy,
        )
    }

    /// Resolves `show impacts` from the shared packet.
    pub fn show_impacts(&self, subject_object_ref: &str) -> InfrastructureRelationJourney {
        self.resolve_relation_journey(
            subject_object_ref,
            InfrastructureJourneyKind::ShowImpacts,
            RelationEdgeClass::Impacts,
            InfrastructureJourneyStatus::MissingImpacts,
        )
    }

    /// Explains one environment slice by shared context id.
    pub fn explain_environment_slice(
        &self,
        context_ref: &str,
    ) -> InfrastructureEnvironmentSliceExplanation {
        let slice_ref = format!(
            "slice:{}:{}:{}",
            self.surface.as_str(),
            self.projection.projection_id,
            context_ref
        );
        let objects = self
            .projection
            .object_refs
            .iter()
            .filter_map(|object_ref| self.packet.object(object_ref))
            .filter(|object| object.context_ref == context_ref)
            .collect::<Vec<_>>();
        if objects.is_empty() {
            return InfrastructureEnvironmentSliceExplanation {
                slice_ref,
                surface: self.surface,
                source_projection_ref: self.projection.projection_id.clone(),
                context_ref: context_ref.to_string(),
                status: InfrastructureJourneyStatus::MissingEnvironmentSliceCoverage,
                families: Vec::new(),
                object_refs: Vec::new(),
                relation_refs: Vec::new(),
                truth_layers: Vec::new(),
                freshness_labels: Vec::new(),
                live_counterpart_relation_refs: Vec::new(),
                applied_by_relation_refs: Vec::new(),
                owned_by_relation_refs: Vec::new(),
                impact_relation_refs: Vec::new(),
                runbook_relation_refs: Vec::new(),
                review_anchor_relation_refs: Vec::new(),
                support_summary: format!(
                    "{} does not project environment slice {}.",
                    self.surface.as_str(),
                    context_ref
                ),
            };
        }

        let object_refs = objects
            .iter()
            .map(|object| object.object_id.clone())
            .collect::<Vec<_>>();
        let relations = self
            .projection
            .relation_refs
            .iter()
            .filter_map(|relation_ref| self.packet.relation(relation_ref))
            .filter(|relation| {
                object_refs.contains(&relation.from_object_ref)
                    && object_refs.contains(&relation.to_object_ref)
            })
            .collect::<Vec<_>>();

        let status = if objects
            .iter()
            .any(|object| freshness_requires_review(object.freshness))
            || relations
                .iter()
                .any(|relation| freshness_requires_review(relation.freshness))
        {
            InfrastructureJourneyStatus::FreshnessReviewRequired
        } else {
            InfrastructureJourneyStatus::Resolved
        };

        InfrastructureEnvironmentSliceExplanation {
            slice_ref,
            surface: self.surface,
            source_projection_ref: self.projection.projection_id.clone(),
            context_ref: context_ref.to_string(),
            status,
            families: collect_unique(objects.iter().map(|object| object.family)),
            object_refs,
            relation_refs: relations
                .iter()
                .map(|relation| relation.relation_id.clone())
                .collect(),
            truth_layers: collect_unique(objects.iter().map(|object| object.truth_layer)),
            freshness_labels: collect_unique(
                objects
                    .iter()
                    .map(|object| object.freshness)
                    .chain(relations.iter().map(|relation| relation.freshness)),
            ),
            live_counterpart_relation_refs: collect_relation_refs(
                &relations,
                RelationEdgeClass::LiveCounterpartOf,
            ),
            applied_by_relation_refs: collect_relation_refs(
                &relations,
                RelationEdgeClass::AppliedBy,
            ),
            owned_by_relation_refs: collect_relation_refs(&relations, RelationEdgeClass::OwnedBy),
            impact_relation_refs: collect_relation_refs(&relations, RelationEdgeClass::Impacts),
            runbook_relation_refs: collect_relation_refs(
                &relations,
                RelationEdgeClass::RunbookReference,
            ),
            review_anchor_relation_refs: collect_relation_refs(
                &relations,
                RelationEdgeClass::ReviewAnchor,
            ),
            support_summary: format!(
                "{} explains {} with {} objects, {} relations, and explicit truth layers.",
                self.surface.as_str(),
                context_ref,
                objects.len(),
                relations.len()
            ),
        }
    }

    /// Explains the environment slice containing the given subject object.
    pub fn explain_environment_slice_for_object(
        &self,
        subject_object_ref: &str,
    ) -> InfrastructureEnvironmentSliceExplanation {
        match self.packet.object(subject_object_ref) {
            Some(object) => self.explain_environment_slice(&object.context_ref),
            None => InfrastructureEnvironmentSliceExplanation {
                slice_ref: format!(
                    "slice:{}:{}:{}",
                    self.surface.as_str(),
                    self.projection.projection_id,
                    subject_object_ref
                ),
                surface: self.surface,
                source_projection_ref: self.projection.projection_id.clone(),
                context_ref: subject_object_ref.to_string(),
                status: InfrastructureJourneyStatus::UnknownSubjectObject,
                families: Vec::new(),
                object_refs: Vec::new(),
                relation_refs: Vec::new(),
                truth_layers: Vec::new(),
                freshness_labels: Vec::new(),
                live_counterpart_relation_refs: Vec::new(),
                applied_by_relation_refs: Vec::new(),
                owned_by_relation_refs: Vec::new(),
                impact_relation_refs: Vec::new(),
                runbook_relation_refs: Vec::new(),
                review_anchor_relation_refs: Vec::new(),
                support_summary: format!(
                    "subject object {} is unknown to the infrastructure packet",
                    subject_object_ref
                ),
            },
        }
    }

    fn resolve_relation_journey(
        &self,
        subject_object_ref: &str,
        journey_kind: InfrastructureJourneyKind,
        edge_class: RelationEdgeClass,
        missing_status: InfrastructureJourneyStatus,
    ) -> InfrastructureRelationJourney {
        let journey_ref = format!(
            "journey:{}:{}:{}",
            self.surface.as_str(),
            journey_kind.as_str(),
            subject_object_ref
        );
        let Some(subject) = self.packet.object(subject_object_ref) else {
            return InfrastructureRelationJourney {
                journey_ref,
                journey_kind,
                surface: self.surface,
                source_projection_ref: self.projection.projection_id.clone(),
                subject_object_ref: subject_object_ref.to_string(),
                subject_context_ref: None,
                status: InfrastructureJourneyStatus::UnknownSubjectObject,
                relation_refs: Vec::new(),
                target_object_refs: Vec::new(),
                truth_layers_used: Vec::new(),
                freshness_labels: Vec::new(),
                authority_postures: Vec::new(),
                support_summary: format!(
                    "subject object {} is unknown to the infrastructure packet",
                    subject_object_ref
                ),
            };
        };

        if !self.projection.object_refs.contains(&subject.object_id) {
            return InfrastructureRelationJourney {
                journey_ref,
                journey_kind,
                surface: self.surface,
                source_projection_ref: self.projection.projection_id.clone(),
                subject_object_ref: subject.object_id.clone(),
                subject_context_ref: Some(subject.context_ref.clone()),
                status: InfrastructureJourneyStatus::OutOfScope,
                relation_refs: Vec::new(),
                target_object_refs: Vec::new(),
                truth_layers_used: vec![subject.truth_layer],
                freshness_labels: vec![subject.freshness],
                authority_postures: vec![subject.authority_posture],
                support_summary: format!(
                    "{} keeps {} out of scope on this surface projection.",
                    self.surface.as_str(),
                    subject.object_id
                ),
            };
        }

        let packet_relations = self
            .packet
            .relation_records
            .iter()
            .filter(|relation| {
                relation.edge_class == edge_class && relation.from_object_ref == subject.object_id
            })
            .collect::<Vec<_>>();
        let projected_relations = self
            .projection
            .relation_refs
            .iter()
            .filter_map(|relation_ref| self.packet.relation(relation_ref))
            .filter(|relation| {
                relation.edge_class == edge_class && relation.from_object_ref == subject.object_id
            })
            .filter(|relation| {
                self.projection
                    .object_refs
                    .contains(&relation.to_object_ref)
            })
            .collect::<Vec<_>>();

        if projected_relations.is_empty() {
            let status = if packet_relations.is_empty() {
                missing_status
            } else {
                InfrastructureJourneyStatus::OutOfScope
            };
            return InfrastructureRelationJourney {
                journey_ref,
                journey_kind,
                surface: self.surface,
                source_projection_ref: self.projection.projection_id.clone(),
                subject_object_ref: subject.object_id.clone(),
                subject_context_ref: Some(subject.context_ref.clone()),
                status,
                relation_refs: Vec::new(),
                target_object_refs: Vec::new(),
                truth_layers_used: vec![subject.truth_layer],
                freshness_labels: vec![subject.freshness],
                authority_postures: vec![subject.authority_posture],
                support_summary: format!(
                    "{} could not resolve {} for {} on {}.",
                    self.surface.as_str(),
                    journey_kind.as_str(),
                    subject.object_id,
                    subject.context_ref
                ),
            };
        }

        let target_objects = projected_relations
            .iter()
            .filter_map(|relation| self.packet.object(&relation.to_object_ref))
            .collect::<Vec<_>>();
        let status = if freshness_requires_review(subject.freshness)
            || projected_relations
                .iter()
                .any(|relation| freshness_requires_review(relation.freshness))
            || target_objects
                .iter()
                .any(|object| freshness_requires_review(object.freshness))
        {
            InfrastructureJourneyStatus::FreshnessReviewRequired
        } else {
            InfrastructureJourneyStatus::Resolved
        };

        InfrastructureRelationJourney {
            journey_ref,
            journey_kind,
            surface: self.surface,
            source_projection_ref: self.projection.projection_id.clone(),
            subject_object_ref: subject.object_id.clone(),
            subject_context_ref: Some(subject.context_ref.clone()),
            status,
            relation_refs: projected_relations
                .iter()
                .map(|relation| relation.relation_id.clone())
                .collect(),
            target_object_refs: target_objects
                .iter()
                .map(|object| object.object_id.clone())
                .collect(),
            truth_layers_used: collect_unique(
                std::iter::once(subject.truth_layer)
                    .chain(target_objects.iter().map(|object| object.truth_layer)),
            ),
            freshness_labels: collect_unique(
                std::iter::once(subject.freshness)
                    .chain(
                        projected_relations
                            .iter()
                            .map(|relation| relation.freshness),
                    )
                    .chain(target_objects.iter().map(|object| object.freshness)),
            ),
            authority_postures: collect_unique(
                std::iter::once(subject.authority_posture)
                    .chain(
                        projected_relations
                            .iter()
                            .map(|relation| relation.authority_posture),
                    )
                    .chain(target_objects.iter().map(|object| object.authority_posture)),
            ),
            support_summary: format!(
                "{} resolved {} from {} through {} explicit relation(s).",
                self.surface.as_str(),
                journey_kind.as_str(),
                subject.object_id,
                projected_relations.len()
            ),
        }
    }
}

impl SourceIntelligenceObjectPacket {
    /// Returns a surface-scoped view backed by the shared consumer projection.
    pub fn surface_view(
        &self,
        surface: InfrastructureJourneySurface,
    ) -> Option<InfrastructureSurfaceView<'_>> {
        let projection = self.consumer_projection(surface.consumer_surface())?;
        Some(InfrastructureSurfaceView {
            packet: self,
            projection,
            surface,
        })
    }
}

pub(super) fn projection_covers_context_slice(
    packet: &SourceIntelligenceObjectPacket,
    projection: &InfrastructureConsumerProjection,
    context_ref: &str,
) -> bool {
    packet
        .object_records
        .iter()
        .filter(|object| object.context_ref == context_ref)
        .all(|object| projection.object_refs.contains(&object.object_id))
}

pub(super) fn projection_covers_relation_flow(
    packet: &SourceIntelligenceObjectPacket,
    projection: &InfrastructureConsumerProjection,
    edge_class: RelationEdgeClass,
) -> bool {
    packet
        .relation_records
        .iter()
        .filter(|relation| {
            relation.edge_class == edge_class
                && projection.object_refs.contains(&relation.from_object_ref)
                && projection.object_refs.contains(&relation.to_object_ref)
        })
        .all(|relation| projection.relation_refs.contains(&relation.relation_id))
}

fn collect_relation_refs(
    relations: &[&InfrastructureObjectRelationRecord],
    edge_class: RelationEdgeClass,
) -> Vec<String> {
    relations
        .iter()
        .filter(|relation| relation.edge_class == edge_class)
        .map(|relation| relation.relation_id.clone())
        .collect()
}

fn freshness_requires_review(label: FreshnessLabel) -> bool {
    !matches!(
        label,
        FreshnessLabel::Live | FreshnessLabel::CurrentSnapshot
    )
}

fn collect_unique<T>(values: impl IntoIterator<Item = T>) -> Vec<T>
where
    T: PartialEq,
{
    let mut collected = Vec::new();
    for value in values {
        if !collected.contains(&value) {
            collected.push(value);
        }
    }
    collected
}

#[cfg(test)]
mod tests {
    use crate::seeded_source_intelligence_object_packet;

    use super::{InfrastructureJourneyStatus, InfrastructureJourneySurface, RelationEdgeClass};

    #[test]
    fn code_surface_resolves_live_counterpart_flow() {
        let packet = seeded_source_intelligence_object_packet();
        let journey = packet
            .surface_view(InfrastructureJourneySurface::CodeInfraView)
            .expect("code surface view")
            .show_live_counterpart("obj:k8s:rendered");

        assert_eq!(journey.status, InfrastructureJourneyStatus::Resolved);
        assert_eq!(journey.relation_refs, vec!["rel:k8s:live_counterpart"]);
        assert_eq!(journey.target_object_refs, vec!["obj:k8s:observed"]);
    }

    #[test]
    fn review_surface_resolves_applied_by_flow() {
        let packet = seeded_source_intelligence_object_packet();
        let journey = packet
            .surface_view(InfrastructureJourneySurface::ReviewWorkspace)
            .expect("review surface view")
            .show_applied_by("obj:tf:observed");

        assert_eq!(journey.status, InfrastructureJourneyStatus::Resolved);
        assert_eq!(journey.relation_refs, vec!["rel:tf:applied_by"]);
        assert_eq!(journey.target_object_refs, vec!["obj:tf:overlay"]);
    }

    #[test]
    fn docs_surface_resolves_impact_flow() {
        let packet = seeded_source_intelligence_object_packet();
        let journey = packet
            .surface_view(InfrastructureJourneySurface::DocsCards)
            .expect("docs surface view")
            .show_impacts("obj:ci:planned");

        assert_eq!(journey.status, InfrastructureJourneyStatus::Resolved);
        assert_eq!(journey.relation_refs, vec!["rel:ci:impacts"]);
        assert_eq!(journey.target_object_refs, vec!["obj:ci:observed"]);
    }

    #[test]
    fn incident_surface_explains_environment_slice() {
        let packet = seeded_source_intelligence_object_packet();
        let explanation = packet
            .surface_view(InfrastructureJourneySurface::IncidentTimeline)
            .expect("incident surface view")
            .explain_environment_slice("ctx:kubernetes");

        assert_eq!(explanation.status, InfrastructureJourneyStatus::Resolved);
        assert!(explanation
            .relation_refs
            .contains(&"rel:k8s:live_counterpart".to_string()));
        assert!(explanation
            .relation_refs
            .contains(&"rel:k8s:impacts".to_string()));
    }

    #[test]
    fn narrowed_projection_labels_out_of_scope_flow() {
        let mut packet = seeded_source_intelligence_object_packet();
        let docs_projection = packet
            .consumer_projections
            .iter_mut()
            .find(|projection| projection.surface == crate::InfrastructureConsumerSurface::Docs)
            .expect("docs projection");
        docs_projection
            .relation_refs
            .retain(|relation_ref| relation_ref != "rel:ci:impacts");

        let journey = packet
            .surface_view(InfrastructureJourneySurface::DocsCards)
            .expect("docs surface view")
            .show_impacts("obj:ci:planned");

        assert_eq!(journey.status, InfrastructureJourneyStatus::OutOfScope);
    }

    #[test]
    fn helper_checks_require_projected_flow_edges() {
        let packet = seeded_source_intelligence_object_packet();
        let projection = packet
            .consumer_projection(crate::InfrastructureConsumerSurface::Review)
            .expect("review projection");

        assert!(super::projection_covers_relation_flow(
            &packet,
            projection,
            RelationEdgeClass::AppliedBy
        ));
        assert!(super::projection_covers_context_slice(
            &packet,
            projection,
            "ctx:terraform"
        ));
    }
}
