//! Desktop navigation-target chrome bound to the search action-binding matrix.
//!
//! This module is the desktop first consumer of
//! [`SearchActionBindingPacket`](aureline_search::SearchActionBindingPacket). It
//! projects a compact, inspectable per-flow set of navigation-target cues that
//! reuse the canonical action bindings verbatim, so the shell's preview, open,
//! split, peek, and external-handoff launchers land on the same target
//! semantics that the keyboard, mouse, AI, automation, and support-replay
//! consumers see.
//!
//! Crucially, the projection does **not** flatten a wrong-target fallback into a
//! plain "open": when a binding fell back under narrowed scope, trust, or
//! freshness, the cue carries the visible reason and recovery action so the
//! chrome renders an explicit, recoverable cue instead of silently jumping to a
//! nearby declaration or browser page. The shell mints no new target identity;
//! it preserves the durable result id, the requested and resolved relation
//! kinds, and the return anchor.

use aureline_navigation::target_model::RelationKind;
use aureline_search::{
    ActionConsumerClass, ActionFlowClass, SearchActionBindingPacket, SearchActionKind,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`NavigationTargetProjectionSet`].
pub const NAVIGATION_TARGET_PROJECTION_SET_RECORD_KIND: &str = "navigation_target_projection_set";

/// Schema version for [`NavigationTargetProjectionSet`].
pub const NAVIGATION_TARGET_PROJECTION_SET_SCHEMA_VERSION: u32 = 1;

/// One desktop navigation-target cue bound to a single action binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetCue {
    /// Binding id reused from the substrate.
    pub binding_id: String,
    /// Action launched by the cue.
    pub action_kind: SearchActionKind,
    /// Durable, surface-independent result identity the cue targets.
    pub result_id: String,
    /// Display title preserved verbatim.
    pub display_title: String,
    /// Relation kind the user requested.
    pub requested_relation_kind: RelationKind,
    /// Relation kind the action actually resolved to.
    pub resolved_relation_kind: RelationKind,
    /// Open-target ref reused from the canonical binding.
    pub open_target_ref: String,
    /// Return anchor focus comes back to after the action.
    pub return_anchor_ref: String,
    /// True when the cue resolved to a wrong-target-safe fallback.
    pub degraded: bool,
    /// True when the resolved relation kind differs from the requested one.
    pub relation_kind_changed: bool,
    /// True when the action crossed to an external handoff.
    pub crosses_to_external_handoff: bool,
    /// User-visible fallback reason; present iff the cue is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<String>,
    /// User-visible recovery hint; present iff the cue is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_hint: Option<String>,
    /// True when the cue preserves authority (never widened by routing).
    pub authority_preserved: bool,
}

impl NavigationTargetCue {
    /// True when the cue keeps the attributable target refs and, if degraded, a
    /// visible and recoverable fallback reason — never a silent jump.
    pub fn is_attributable(&self) -> bool {
        !self.open_target_ref.is_empty()
            && !self.return_anchor_ref.is_empty()
            && self.authority_preserved
            && (!self.degraded
                || self
                    .fallback_reason
                    .as_ref()
                    .is_some_and(|reason| !reason.trim().is_empty()))
    }
}

/// Per-flow group of navigation-target cues.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetFlowProjection {
    /// Flow token reused from the substrate.
    pub flow: ActionFlowClass,
    /// Human-readable flow label.
    pub flow_label: String,
    /// Navigation-target cues, in binding order.
    pub cues: Vec<NavigationTargetCue>,
}

impl NavigationTargetFlowProjection {
    /// Number of cues in this flow that render a wrong-target-safe fallback.
    pub fn degraded_cue_count(&self) -> usize {
        self.cues.iter().filter(|cue| cue.degraded).count()
    }
}

/// Desktop projection of the action-binding matrix across all flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetProjectionSet {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet id the desktop ingests verbatim.
    pub ingested_packet_id: String,
    /// Per-flow projections, in substrate order.
    pub flows: Vec<NavigationTargetFlowProjection>,
}

impl NavigationTargetProjectionSet {
    /// Returns the projection for one flow, if present.
    pub fn flow_for(&self, flow: ActionFlowClass) -> Option<&NavigationTargetFlowProjection> {
        self.flows.iter().find(|projection| projection.flow == flow)
    }

    /// True when every flow is bound and every cue keeps attributable target
    /// refs and visible, recoverable fallbacks.
    pub fn reuses_substrate(&self) -> bool {
        self.flows.len() == ActionFlowClass::ALL.len()
            && self.flows.iter().all(|flow| {
                !flow.cues.is_empty() && flow.cues.iter().all(NavigationTargetCue::is_attributable)
            })
    }
}

/// Projects the desktop navigation-target cues from the action-binding packet.
///
/// The projection reuses the durable result ids, relation kinds, and return
/// anchors verbatim; it mints no new target identity, so the desktop launchers
/// share one truth with the history/back-forward and support-replay consumers.
pub fn project_navigation_targets(
    packet: &SearchActionBindingPacket,
) -> NavigationTargetProjectionSet {
    let flows = packet
        .flows
        .iter()
        .map(|flow| NavigationTargetFlowProjection {
            flow: flow.flow,
            flow_label: flow.flow_label.clone(),
            cues: flow
                .bindings
                .iter()
                .map(|binding| NavigationTargetCue {
                    binding_id: binding.binding_id.clone(),
                    action_kind: binding.action_kind,
                    result_id: binding.result_id.clone(),
                    display_title: binding.display_title.clone(),
                    requested_relation_kind: binding.requested_relation_kind,
                    resolved_relation_kind: binding.resolved_relation_kind,
                    open_target_ref: binding.action_binding.open_target_ref.clone(),
                    return_anchor_ref: binding.return_anchor_ref.clone(),
                    degraded: binding.fallback.is_some(),
                    relation_kind_changed: binding
                        .fallback
                        .as_ref()
                        .is_some_and(|fallback| fallback.relation_kind_changed),
                    crosses_to_external_handoff: binding
                        .fallback
                        .as_ref()
                        .is_some_and(|fallback| fallback.crosses_to_external_handoff),
                    fallback_reason: binding
                        .fallback
                        .as_ref()
                        .map(|fallback| fallback.visible_reason.clone()),
                    recovery_hint: binding
                        .fallback
                        .as_ref()
                        .map(|fallback| fallback.recovery_action.clone()),
                    authority_preserved: binding.authority_not_widened,
                })
                .collect(),
        })
        .collect();

    NavigationTargetProjectionSet {
        record_kind: NAVIGATION_TARGET_PROJECTION_SET_RECORD_KIND.to_string(),
        schema_version: NAVIGATION_TARGET_PROJECTION_SET_SCHEMA_VERSION,
        ingested_packet_id: packet.packet_id.clone(),
        flows,
    }
}

/// True when the packet names the desktop a first consumer that reuses the same
/// binding objects without widening authority.
pub fn shell_is_action_binding_first_consumer(packet: &SearchActionBindingPacket) -> bool {
    packet.consumer_projections.iter().any(|projection| {
        projection.consumer == ActionConsumerClass::ProductUi
            && projection.reuses_same_binding_objects
            && projection.preserves_relation_kinds
            && projection.preserves_return_anchors
            && projection.preserves_fallback_reasons
            && !projection.widens_authority
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_search::seeded_search_action_binding_packet;

    #[test]
    fn projects_a_flow_for_every_action_flow() {
        let packet = seeded_search_action_binding_packet();
        let set = project_navigation_targets(&packet);
        assert_eq!(
            set.record_kind,
            NAVIGATION_TARGET_PROJECTION_SET_RECORD_KIND
        );
        assert_eq!(set.ingested_packet_id, packet.packet_id);
        assert!(set.reuses_substrate());
        for flow in ActionFlowClass::ALL {
            assert!(set.flow_for(flow).is_some(), "missing {flow:?}");
        }
        assert!(shell_is_action_binding_first_consumer(&packet));
    }

    #[test]
    fn preserves_relation_kinds_and_return_anchors() {
        let packet = seeded_search_action_binding_packet();
        let set = project_navigation_targets(&packet);
        for flow in &set.flows {
            for cue in &flow.cues {
                assert!(!cue.return_anchor_ref.is_empty());
                assert_ne!(cue.return_anchor_ref, cue.open_target_ref);
                assert!(cue.authority_preserved);
            }
        }
    }

    #[test]
    fn degraded_cues_render_a_visible_recoverable_reason() {
        // A wrong-target fallback never flattens into a plain open: the cue keeps
        // the visible reason and recovery hint the chrome must render.
        let packet = seeded_search_action_binding_packet();
        let set = project_navigation_targets(&packet);
        let search = set.flow_for(ActionFlowClass::SearchResults).unwrap();
        let degraded = search
            .cues
            .iter()
            .find(|cue| cue.relation_kind_changed)
            .expect("a definition-to-declaration degrade cue");
        assert_eq!(degraded.requested_relation_kind, RelationKind::Definition);
        assert_eq!(degraded.resolved_relation_kind, RelationKind::Declaration);
        assert!(degraded.degraded);
        assert!(degraded.fallback_reason.is_some());
        assert!(degraded.recovery_hint.is_some());

        let docs = set.flow_for(ActionFlowClass::DocsResults).unwrap();
        assert!(docs
            .cues
            .iter()
            .any(|cue| cue.crosses_to_external_handoff && cue.fallback_reason.is_some()));
    }

    #[test]
    fn round_trips_through_json() {
        let packet = seeded_search_action_binding_packet();
        let set = project_navigation_targets(&packet);
        let json = serde_json::to_string(&set).expect("projection serializes");
        let round_trip: NavigationTargetProjectionSet =
            serde_json::from_str(&json).expect("projection deserializes");
        assert_eq!(round_trip, set);
    }
}
