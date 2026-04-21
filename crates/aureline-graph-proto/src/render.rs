//! Hand-rolled canonical JSON renderer for the workspace-graph seed.
//!
//! Counts only, no wall-clock. No serde dependency so the committed
//! aggregate report stays byte-stable across hosts. The fixtures
//! under `/fixtures/graph/example_workspace_graphs/` are the
//! authoring format; this renderer targets the *bench aggregate*
//! report so the prototype's emitted artifact is reviewable
//! alongside the fixture seed.

use crate::harness::{Report, ScenarioReport};
use crate::hooks::HookCounters;
use crate::model::{GraphEdge, GraphNode, WorkspaceGraph};
use crate::validator::{rule_id_for, ValidationError};
use crate::vocab::WORKSPACE_GRAPH_SCHEMA_VERSION;

/// Render the aggregate report as canonical JSON.
pub fn report_to_json(report: &Report) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    field_raw(&mut out, 1, "bench_version", &json_str("aureline-graph-proto/0"));
    comma(&mut out);
    field_raw(
        &mut out,
        1,
        "workspace_graph_schema_version",
        &WORKSPACE_GRAPH_SCHEMA_VERSION.to_string(),
    );
    comma(&mut out);
    field_raw(&mut out, 1, "scenario_count", &report.scenarios.len().to_string());
    comma(&mut out);
    field_raw(
        &mut out,
        1,
        "total_errors",
        &report.total_errors.to_string(),
    );
    comma(&mut out);
    indent(&mut out, 1);
    out.push_str("\"rule_ids_enforced\": ");
    string_array(&mut out, 1, &report.rule_ids_enforced);
    comma(&mut out);
    indent(&mut out, 1);
    out.push_str("\"aggregate_hook_counts\": ");
    hook_counters_json(&mut out, 1, &report.aggregate_hooks);
    comma(&mut out);
    indent(&mut out, 1);
    out.push_str("\"scenarios\": [\n");
    for (idx, scenario) in report.scenarios.iter().enumerate() {
        scenario_json(&mut out, 2, scenario);
        if idx + 1 < report.scenarios.len() {
            out.push_str(",\n");
        } else {
            out.push('\n');
        }
    }
    indent(&mut out, 1);
    out.push_str("]\n");
    out.push_str("}\n");
    out
}

fn scenario_json(out: &mut String, depth: usize, scenario: &ScenarioReport) {
    indent(out, depth);
    out.push_str("{\n");
    field_raw(out, depth + 1, "label", &json_str(scenario.label));
    comma(out);
    field_raw(out, depth + 1, "doc_section", &json_str(scenario.doc_section));
    comma(out);
    field_raw(out, depth + 1, "node_count", &scenario.node_count.to_string());
    comma(out);
    field_raw(out, depth + 1, "edge_count", &scenario.edge_count.to_string());
    comma(out);
    field_raw(
        out,
        depth + 1,
        "error_count",
        &scenario.errors.len().to_string(),
    );
    comma(out);
    indent(out, depth + 1);
    out.push_str("\"node_classes_seen\": ");
    string_array(out, depth + 1, &scenario.node_classes_seen);
    comma(out);
    indent(out, depth + 1);
    out.push_str("\"edge_classes_seen\": ");
    string_array(out, depth + 1, &scenario.edge_classes_seen);
    comma(out);
    indent(out, depth + 1);
    out.push_str("\"evidence_states_seen\": ");
    string_array(out, depth + 1, &scenario.evidence_states_seen);
    comma(out);
    indent(out, depth + 1);
    out.push_str("\"freshness_seen\": ");
    string_array(out, depth + 1, &scenario.freshness_seen);
    comma(out);
    indent(out, depth + 1);
    out.push_str("\"hook_counts\": ");
    hook_counters_json(out, depth + 1, &scenario.hooks);
    comma(out);
    indent(out, depth + 1);
    out.push_str("\"errors\": ");
    errors_json(out, depth + 1, &scenario.errors);
    out.push('\n');
    indent(out, depth);
    out.push('}');
}

fn hook_counters_json(out: &mut String, depth: usize, hooks: &HookCounters) {
    out.push_str("{\n");
    let entries = hooks.entries();
    for (idx, (id, protected, count)) in entries.iter().enumerate() {
        indent(out, depth + 1);
        out.push_str(&json_str(id));
        out.push_str(": {\n");
        field_raw(out, depth + 2, "protected_hot_path", &protected.to_string());
        comma(out);
        field_raw(out, depth + 2, "count", &count.to_string());
        out.push('\n');
        indent(out, depth + 1);
        out.push('}');
        if idx + 1 < entries.len() {
            out.push_str(",\n");
        } else {
            out.push('\n');
        }
    }
    indent(out, depth);
    out.push('}');
}

fn errors_json(out: &mut String, depth: usize, errors: &[ValidationError]) {
    if errors.is_empty() {
        out.push_str("[]");
        return;
    }
    out.push_str("[\n");
    for (idx, error) in errors.iter().enumerate() {
        indent(out, depth + 1);
        out.push_str("{\n");
        field_raw(out, depth + 2, "rule_id", &json_str(rule_id_for(error)));
        comma(out);
        field_raw(out, depth + 2, "detail", &json_str(&format!("{error:?}")));
        out.push('\n');
        indent(out, depth + 1);
        out.push('}');
        if idx + 1 < errors.len() {
            out.push_str(",\n");
        } else {
            out.push('\n');
        }
    }
    indent(out, depth);
    out.push(']');
}

/// Render a single per-scenario structural report as canonical
/// JSON. Used for `--emit-scenarios` mode so each scenario's
/// counts-only report lands in its own reviewable file.
pub fn scenario_to_json(scenario: &ScenarioReport) -> String {
    let mut out = String::new();
    scenario_json(&mut out, 0, scenario);
    out.push('\n');
    out
}

/// Render a single workspace graph as canonical JSON for diffing
/// against the fixture. Not all fixture metadata round-trips (for
/// example, `__fixture__` is ignored); the renderer emits the core
/// record shape only.
pub fn graph_to_json(graph: &WorkspaceGraph) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    field_raw(&mut out, 1, "record_kind", &json_str("workspace_graph_record"));
    comma(&mut out);
    field_raw(
        &mut out,
        1,
        "workspace_graph_schema_version",
        &WORKSPACE_GRAPH_SCHEMA_VERSION.to_string(),
    );
    comma(&mut out);
    field_raw(
        &mut out,
        1,
        "workspace_graph_id",
        &json_str(&graph.workspace_graph_id),
    );
    comma(&mut out);
    field_raw(&mut out, 1, "workspace_id", &json_str(&graph.workspace_id));
    comma(&mut out);
    field_raw(&mut out, 1, "recorded_at", &json_str(&graph.recorded_at));
    comma(&mut out);
    field_raw(
        &mut out,
        1,
        "node_count",
        &graph.nodes.len().to_string(),
    );
    comma(&mut out);
    field_raw(
        &mut out,
        1,
        "edge_count",
        &graph.edges.len().to_string(),
    );
    comma(&mut out);
    indent(&mut out, 1);
    out.push_str("\"node_ids\": ");
    string_array(
        &mut out,
        1,
        &graph
            .nodes
            .iter()
            .map(|n| n.node_id.clone())
            .collect::<Vec<_>>(),
    );
    comma(&mut out);
    indent(&mut out, 1);
    out.push_str("\"edge_ids\": ");
    string_array(
        &mut out,
        1,
        &graph
            .edges
            .iter()
            .map(|e| e.edge_id.clone())
            .collect::<Vec<_>>(),
    );
    out.push('\n');
    out.push_str("}\n");
    out
}

// ----- low-level helpers -----

fn field_raw(out: &mut String, depth: usize, name: &str, value_json: &str) {
    indent(out, depth);
    out.push_str(&json_str(name));
    out.push_str(": ");
    out.push_str(value_json);
}

fn comma(out: &mut String) {
    out.push_str(",\n");
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn string_array(out: &mut String, depth: usize, items: &[String]) {
    if items.is_empty() {
        out.push_str("[]");
        return;
    }
    out.push_str("[\n");
    for (idx, item) in items.iter().enumerate() {
        indent(out, depth + 1);
        out.push_str(&json_str(item));
        if idx + 1 < items.len() {
            out.push_str(",\n");
        } else {
            out.push('\n');
        }
    }
    indent(out, depth);
    out.push(']');
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// Utility helpers consumed by harness::report_to_scenario.
pub(crate) fn node_id_list(nodes: &[GraphNode]) -> Vec<String> {
    nodes.iter().map(|n| n.node_id.clone()).collect()
}

pub(crate) fn edge_id_list(edges: &[GraphEdge]) -> Vec<String> {
    edges.iter().map(|e| e.edge_id.clone()).collect()
}
