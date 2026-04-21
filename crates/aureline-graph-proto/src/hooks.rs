//! Protected-hot-path and observability hook counters for the
//! workspace-graph seed.
//!
//! Hook ids match the names the design doc enumerates. The prototype
//! counts; a production graph engine replaces the struct with a
//! telemetry seam behind the same names so the benchmark lab, the
//! support-export lane, and the eventual subscription / replay lane
//! never have to translate vocabulary.

/// Per-graph hook-fire counters. Counts only; no wall-clock timing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HookCounters {
    /// Protected: one snapshot emitted.
    pub workspace_graph_snapshot_emitted: u64,
    /// Protected: a graph node admitted to the snapshot.
    pub graph_node_admitted: u64,
    /// Protected: a graph edge admitted to the snapshot.
    pub graph_edge_admitted: u64,
    /// Observability: a node projected a non-authoritative freshness.
    pub graph_freshness_downgraded: u64,
    /// Observability: a node projected a low/unknown confidence.
    pub graph_confidence_downgraded: u64,
    /// Observability: a missing-anchor node admitted.
    pub graph_missing_anchor_recorded: u64,
    /// Observability: a workset-scope narrowing edge admitted.
    pub graph_workset_scope_narrowed: u64,
    /// Observability: a workset-scope widening edge admitted.
    pub graph_workset_scope_widened: u64,
    /// Observability: a policy-view projection admitted.
    pub graph_policy_view_projected: u64,
    /// Observability: an imported-bundle node admitted.
    pub graph_imported_attach: u64,
    /// Observability: a topology edge admitted (carries
    /// topology_edge_slot).
    pub graph_topology_edge_admitted: u64,
    /// Observability: an impact-reason slot attached.
    pub graph_impact_reason_attached: u64,
    /// Observability: an explainer-citation slot attached.
    pub graph_explainer_citation_attached: u64,
}

impl HookCounters {
    /// Ordered `(hook_id, protected_hot_path, count)` rows.
    /// Deterministic iteration order so harness JSON stays byte-stable
    /// across hosts.
    pub fn entries(&self) -> [(&'static str, bool, u64); 13] {
        [
            (
                "workspace_graph_snapshot_emitted",
                true,
                self.workspace_graph_snapshot_emitted,
            ),
            ("graph_node_admitted", true, self.graph_node_admitted),
            ("graph_edge_admitted", true, self.graph_edge_admitted),
            (
                "graph_freshness_downgraded",
                false,
                self.graph_freshness_downgraded,
            ),
            (
                "graph_confidence_downgraded",
                false,
                self.graph_confidence_downgraded,
            ),
            (
                "graph_missing_anchor_recorded",
                false,
                self.graph_missing_anchor_recorded,
            ),
            (
                "graph_workset_scope_narrowed",
                false,
                self.graph_workset_scope_narrowed,
            ),
            (
                "graph_workset_scope_widened",
                false,
                self.graph_workset_scope_widened,
            ),
            (
                "graph_policy_view_projected",
                false,
                self.graph_policy_view_projected,
            ),
            ("graph_imported_attach", false, self.graph_imported_attach),
            (
                "graph_topology_edge_admitted",
                false,
                self.graph_topology_edge_admitted,
            ),
            (
                "graph_impact_reason_attached",
                false,
                self.graph_impact_reason_attached,
            ),
            (
                "graph_explainer_citation_attached",
                false,
                self.graph_explainer_citation_attached,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_are_unique_and_complete() {
        let entries = HookCounters::default().entries();
        assert_eq!(entries.len(), 13);
        let mut labels: Vec<&'static str> = entries.iter().map(|(id, _, _)| *id).collect();
        labels.sort();
        labels.dedup();
        assert_eq!(labels.len(), 13, "hook ids must be unique");
    }

    #[test]
    fn protected_hooks_match_doc() {
        let entries = HookCounters::default().entries();
        let protected: Vec<&'static str> = entries
            .iter()
            .filter(|(_, protected, _)| *protected)
            .map(|(id, _, _)| *id)
            .collect();
        assert_eq!(protected.len(), 3);
        assert!(protected.contains(&"workspace_graph_snapshot_emitted"));
        assert!(protected.contains(&"graph_node_admitted"));
        assert!(protected.contains(&"graph_edge_admitted"));
    }
}
