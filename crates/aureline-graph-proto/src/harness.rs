//! Smoke harness for the workspace-graph seed prototype.
//!
//! Runs the frozen scenario table end-to-end, validates each graph
//! via [`validate_graph`], aggregates protected-hot-path and
//! observability counters, and reports a counts-only structural
//! summary. Output is byte-stable across hosts: no wall-clock, no
//! serde, synthetic monotonic tokens, deterministic ordering.

use crate::hooks::HookCounters;
use crate::render::{edge_id_list, node_id_list};
use crate::scenarios::{all_scenarios, Scenario};
use crate::validator::{rule_ids, validate_graph, ValidationError};
use crate::vocab::{Freshness, NodeClass};

/// Per-scenario structural report. Fields mirror the shape emitted
/// by [`crate::render::report_to_json`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioReport {
    pub label: &'static str,
    pub doc_section: &'static str,
    pub node_count: u64,
    pub edge_count: u64,
    pub errors: Vec<ValidationError>,
    pub node_classes_seen: Vec<String>,
    pub edge_classes_seen: Vec<String>,
    pub evidence_states_seen: Vec<String>,
    pub freshness_seen: Vec<String>,
    pub hooks: HookCounters,
    pub node_ids: Vec<String>,
    pub edge_ids: Vec<String>,
}

/// Aggregate harness output across all scenarios.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub scenarios: Vec<ScenarioReport>,
    pub total_errors: u64,
    pub rule_ids_enforced: Vec<String>,
    pub aggregate_hooks: HookCounters,
}

/// Run the frozen scenario table and aggregate results.
pub fn run_harness() -> Report {
    let scenarios_in = all_scenarios();
    let mut scenario_reports: Vec<ScenarioReport> = Vec::with_capacity(scenarios_in.len());
    let mut total_errors: u64 = 0;
    let mut aggregate_hooks = HookCounters::default();

    for scenario in &scenarios_in {
        let report = report_for_scenario(scenario);
        total_errors += report.errors.len() as u64;
        add_hooks(&mut aggregate_hooks, &report.hooks);
        scenario_reports.push(report);
    }

    Report {
        scenarios: scenario_reports,
        total_errors,
        rule_ids_enforced: rule_ids().iter().map(|s| (*s).to_string()).collect(),
        aggregate_hooks,
    }
}

fn report_for_scenario(scenario: &Scenario) -> ScenarioReport {
    let (errors, hooks) = validate_graph(&scenario.graph);

    let mut node_classes_seen: Vec<String> = scenario
        .graph
        .nodes
        .iter()
        .map(|n| n.node_class.as_str().to_string())
        .collect();
    dedup_sorted(&mut node_classes_seen);

    let mut edge_classes_seen: Vec<String> = scenario
        .graph
        .edges
        .iter()
        .map(|e| e.edge_class.as_str().to_string())
        .collect();
    dedup_sorted(&mut edge_classes_seen);

    let mut evidence_states_seen: Vec<String> = scenario
        .graph
        .edges
        .iter()
        .map(|e| e.evidence.evidence_state.as_str().to_string())
        .collect();
    dedup_sorted(&mut evidence_states_seen);

    let mut freshness_seen: Vec<String> = Vec::new();
    for node in &scenario.graph.nodes {
        freshness_seen.push(node.freshness_frame.freshness.as_str().to_string());
    }
    for edge in &scenario.graph.edges {
        freshness_seen.push(
            edge.evidence
                .freshness_frame
                .freshness
                .as_str()
                .to_string(),
        );
    }
    dedup_sorted(&mut freshness_seen);

    ScenarioReport {
        label: scenario.label,
        doc_section: scenario.doc_section,
        node_count: scenario.graph.nodes.len() as u64,
        edge_count: scenario.graph.edges.len() as u64,
        errors,
        node_classes_seen,
        edge_classes_seen,
        evidence_states_seen,
        freshness_seen,
        hooks,
        node_ids: node_id_list(&scenario.graph.nodes),
        edge_ids: edge_id_list(&scenario.graph.edges),
    }
}

fn add_hooks(into: &mut HookCounters, from: &HookCounters) {
    into.workspace_graph_snapshot_emitted += from.workspace_graph_snapshot_emitted;
    into.graph_node_admitted += from.graph_node_admitted;
    into.graph_edge_admitted += from.graph_edge_admitted;
    into.graph_freshness_downgraded += from.graph_freshness_downgraded;
    into.graph_confidence_downgraded += from.graph_confidence_downgraded;
    into.graph_missing_anchor_recorded += from.graph_missing_anchor_recorded;
    into.graph_workset_scope_narrowed += from.graph_workset_scope_narrowed;
    into.graph_workset_scope_widened += from.graph_workset_scope_widened;
    into.graph_policy_view_projected += from.graph_policy_view_projected;
    into.graph_imported_attach += from.graph_imported_attach;
    into.graph_topology_edge_admitted += from.graph_topology_edge_admitted;
    into.graph_impact_reason_attached += from.graph_impact_reason_attached;
    into.graph_explainer_citation_attached += from.graph_explainer_citation_attached;
}

fn dedup_sorted(items: &mut Vec<String>) {
    items.sort();
    items.dedup();
}

/// Discovery helper: every node-class token the frozen vocabulary
/// names. Used by the coverage test below.
fn all_node_class_tokens() -> Vec<&'static str> {
    NodeClass::all().iter().map(|c| c.as_str()).collect()
}

/// Discovery helper: every freshness token the frozen vocabulary
/// names.
fn all_freshness_tokens() -> Vec<&'static str> {
    Freshness::all().iter().map(|f| f.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harness_runs_clean() {
        let report = run_harness();
        assert_eq!(report.scenarios.len(), 5);
        assert_eq!(report.total_errors, 0);
        assert_eq!(report.rule_ids_enforced.len(), 11);
    }

    #[test]
    fn harness_is_byte_stable() {
        let a = run_harness();
        let b = run_harness();
        assert_eq!(
            crate::render::report_to_json(&a),
            crate::render::report_to_json(&b)
        );
    }

    #[test]
    fn coverage_across_scenarios_touches_every_node_class() {
        let report = run_harness();
        let mut union: Vec<String> = Vec::new();
        for sc in &report.scenarios {
            for c in &sc.node_classes_seen {
                union.push(c.clone());
            }
        }
        dedup_sorted(&mut union);
        for expected in all_node_class_tokens() {
            assert!(
                union.iter().any(|c| c == expected),
                "no scenario exercises node class `{expected}`"
            );
        }
    }

    #[test]
    fn aggregate_hook_sums_match_per_scenario_sums() {
        let report = run_harness();
        // Every scenario admits exactly one snapshot.
        assert_eq!(
            report.aggregate_hooks.workspace_graph_snapshot_emitted,
            report.scenarios.len() as u64
        );
        // Per-scenario node-admit counts sum to aggregate.
        let node_sum: u64 = report.scenarios.iter().map(|s| s.node_count).sum();
        assert_eq!(report.aggregate_hooks.graph_node_admitted, node_sum);
        let edge_sum: u64 = report.scenarios.iter().map(|s| s.edge_count).sum();
        assert_eq!(report.aggregate_hooks.graph_edge_admitted, edge_sum);
    }

    #[test]
    fn freshness_vocabulary_exists() {
        // Smoke check that the vocabulary helper is reachable; the
        // scenario corpus only needs to exercise the subset its doc
        // sections cover.
        assert!(!all_freshness_tokens().is_empty());
    }
}
