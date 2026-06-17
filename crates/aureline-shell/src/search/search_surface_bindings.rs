//! Desktop bindings onto the durable query-session and result-identity substrate.
//!
//! This module is the desktop first consumer of
//! [`QuerySessionFirstConsumersPacket`]. It projects a compact, inspectable
//! per-surface binding that reuses the durable query-session and result ids
//! verbatim, so quick open, symbol search, full-text search, references, docs
//! search, and recent-navigation panes never re-mint result identity from
//! rendered row text across virtualization, preview toggles, reason-chip
//! toggles, or pane restore.

use aureline_search::{
    ConsumerSurfaceKind, QuerySessionFirstConsumersPacket, SessionConsumerClass,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SearchSurfaceBindingSet`].
pub const SEARCH_SURFACE_BINDING_SET_RECORD_KIND: &str = "search_surface_binding_set";

/// Schema version for [`SearchSurfaceBindingSet`].
pub const SEARCH_SURFACE_BINDING_SET_SCHEMA_VERSION: u32 = 1;

/// One desktop surface bound to its durable session and result identities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchSurfaceBinding {
    /// Surface token reused from the substrate.
    pub surface: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Durable query-session id the pane binds to.
    pub query_session_id: String,
    /// Durable result ids the pane renders, in substrate order.
    pub result_ids: Vec<String>,
    /// Number of materialized rows the pane renders.
    pub row_count: usize,
}

/// Desktop projection of the durable substrate across all bound surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchSurfaceBindingSet {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Packet id the desktop ingests verbatim.
    pub ingested_packet_id: String,
    /// Per-surface bindings, in substrate order.
    pub bindings: Vec<SearchSurfaceBinding>,
}

impl SearchSurfaceBindingSet {
    /// Returns the binding for one surface, if present.
    pub fn binding_for(&self, surface: ConsumerSurfaceKind) -> Option<&SearchSurfaceBinding> {
        self.bindings
            .iter()
            .find(|binding| binding.surface == surface.as_str())
    }

    /// True when every covered surface has a binding and the desktop consumer
    /// reuse contract in the packet is intact (no UI-text reconstruction, no
    /// private candidate list).
    pub fn reuses_substrate(&self) -> bool {
        self.bindings.len() == ConsumerSurfaceKind::ALL.len()
            && self.bindings.iter().all(|binding| {
                !binding.query_session_id.is_empty() && !binding.result_ids.is_empty()
            })
    }
}

/// Projects the desktop surface bindings from the durable substrate packet.
///
/// The projection reuses the durable query-session and result ids verbatim; it
/// mints no new identity, so the desktop search panes share one truth with the
/// CLI/headless, AI-context, and support-export consumers.
pub fn project_search_surface_bindings(
    packet: &QuerySessionFirstConsumersPacket,
) -> SearchSurfaceBindingSet {
    let bindings = packet
        .durable_sessions
        .iter()
        .map(|session| SearchSurfaceBinding {
            surface: session.surface.as_str().to_string(),
            surface_label: session.surface_label.clone(),
            query_session_id: session.query_session.query_session_id.clone(),
            result_ids: session
                .result_rows
                .iter()
                .map(|row| row.result_ref.result_id.clone())
                .collect(),
            row_count: session.result_rows.len(),
        })
        .collect();

    SearchSurfaceBindingSet {
        record_kind: SEARCH_SURFACE_BINDING_SET_RECORD_KIND.to_string(),
        schema_version: SEARCH_SURFACE_BINDING_SET_SCHEMA_VERSION,
        ingested_packet_id: packet.packet_id.clone(),
        bindings,
    }
}

/// True when the substrate names the desktop a first consumer that reuses the
/// session and result ids without reconstructing state from rendered UI text.
pub fn desktop_is_first_consumer(packet: &QuerySessionFirstConsumersPacket) -> bool {
    packet.consumer_bindings.iter().any(|binding| {
        binding.consumer == SessionConsumerClass::Desktop
            && !binding.reconstructs_from_ui_text
            && !binding.invents_private_candidate_list
            && binding.preserves_source_stratum_lineage
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_search::seeded_query_session_first_consumers_packet;

    #[test]
    fn projects_a_binding_for_every_surface() {
        let packet = seeded_query_session_first_consumers_packet();
        let set = project_search_surface_bindings(&packet);
        assert_eq!(set.record_kind, SEARCH_SURFACE_BINDING_SET_RECORD_KIND);
        assert_eq!(set.ingested_packet_id, packet.packet_id);
        assert!(set.reuses_substrate());
        for surface in ConsumerSurfaceKind::ALL {
            assert!(set.binding_for(surface).is_some(), "missing {surface:?}");
        }
    }

    #[test]
    fn reuses_durable_ids_without_reminting() {
        let packet = seeded_query_session_first_consumers_packet();
        let set = project_search_surface_bindings(&packet);
        // Quick open and full-text search bind to the same file identity.
        let quick = set.binding_for(ConsumerSurfaceKind::QuickOpen).unwrap();
        let full_text = set
            .binding_for(ConsumerSurfaceKind::FullTextSearch)
            .unwrap();
        let shared = packet
            .cross_surface_reuse
            .iter()
            .map(|reuse| reuse.shared_result_id.clone())
            .collect::<Vec<_>>();
        assert!(shared
            .iter()
            .any(|id| quick.result_ids.contains(id) && full_text.result_ids.contains(id)));
        assert!(desktop_is_first_consumer(&packet));
    }
}
