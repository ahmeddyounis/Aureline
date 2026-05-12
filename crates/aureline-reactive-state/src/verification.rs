//! Verification helpers layered on top of the reactive-state
//! scenario table.
//!
//! This module adds two seed lanes the higher-level verification
//! packet needs:
//!
//! - snapshot-vs-delta parity for one derived/materialized view;
//! - concise invalidation-order audits extracted from the same
//!   scenario reports the main harness already emits.

use std::fmt::Write as _;

use crate::envelope::{Completeness, Freshness};
use crate::harness::{run_harness, ScenarioReport};
use crate::trace::TraceEvent;

/// Schema version for snapshot-vs-delta parity cases.
pub const SNAPSHOT_DELTA_PARITY_SCHEMA_VERSION: u32 = 1;

/// Schema version for invalidation-order audits.
pub const INVALIDATION_ORDER_AUDIT_SCHEMA_VERSION: u32 = 1;

/// Human-reviewable parity case. Each step records the materialized
/// view observed when the same logical revision is reached via the
/// snapshot path and the delta path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotDeltaParityCase {
    pub schema_version: u32,
    pub case_id: &'static str,
    pub query_family: &'static str,
    pub upstream_query_family: &'static str,
    pub view_class: &'static str,
    pub scope_class: &'static str,
    pub scope_id: &'static str,
    pub steps: Vec<SnapshotDeltaParityStep>,
}

/// One parity step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotDeltaParityStep {
    pub step_id: &'static str,
    pub revision_ref: &'static str,
    pub snapshot_view: DiagnosticsSummaryView,
    pub delta_view: DiagnosticsSummaryView,
}

/// Materialized diagnostics summary used by the seed parity corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticsSummaryView {
    pub freshness: Freshness,
    pub completeness: Completeness,
    pub upstream_digest: &'static str,
    pub total_files_with_diagnostics: u64,
    pub total_errors: u64,
    pub total_warnings: u64,
    pub affected_files: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiagnosticsProjectionState {
    freshness: Freshness,
    completeness: Completeness,
    upstream_digest: &'static str,
    files: Vec<DiagnosticsFileState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiagnosticsFileState {
    file: &'static str,
    errors: u64,
    warnings: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagnosticsDeltaOp {
    UpsertFile {
        file: &'static str,
        errors: u64,
        warnings: u64,
    },
    RemoveFile {
        file: &'static str,
    },
    AdvanceUpstreamDigest {
        digest: &'static str,
    },
}

/// Condensed audit of one invalidation-order contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidationOrderAudit {
    pub schema_version: u32,
    pub file_stem: &'static str,
    pub audit_id: &'static str,
    pub scenario_label: &'static str,
    pub summary: &'static str,
    pub query_families: Vec<&'static str>,
    pub order_contract: Vec<InvalidationOrderStep>,
    pub drift_detected_if: Vec<&'static str>,
    pub recovery_behavior: &'static str,
}

/// One ordered event inside an invalidation-order audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidationOrderStep {
    pub ordinal: u64,
    pub tick: u64,
    pub event_kind: &'static str,
    pub hook_id: &'static str,
    pub query_family: Option<String>,
    pub frame_class: Option<String>,
    pub snapshot_epoch: Option<u64>,
    pub delta_seq: Option<u64>,
    pub freshness: Option<String>,
    pub completeness: Option<String>,
    pub stale_reason: Option<String>,
    pub message: Option<String>,
    pub expectation: &'static str,
}

/// Run the seed snapshot-vs-delta parity corpus.
pub fn run_snapshot_delta_parity_cases() -> Vec<SnapshotDeltaParityCase> {
    vec![diagnostics_summary_parity_case()]
}

/// Run the invalidation-order audits extracted from the scenario table.
pub fn run_invalidation_order_audits() -> Vec<InvalidationOrderAudit> {
    let harness = run_harness();
    let scenarios = harness.scenarios;
    vec![
        refresh_ordering_audit(find_scenario(
            &scenarios,
            "refresh_ordering_authority_before_derived",
        )),
        delta_gap_resync_audit(find_scenario(
            &scenarios,
            "file_identity_delta_gap_triggers_resync",
        )),
        snapshot_required_switch_audit(find_scenario(
            &scenarios,
            "backpressure_snapshot_required_switch",
        )),
    ]
}

/// Render one audit as canonical JSON.
pub fn invalidation_order_audit_to_json(audit: &InvalidationOrderAudit) -> String {
    let mut out = String::new();
    out.push('{');
    let ind = 1;
    write_kv_u64(
        &mut out,
        "schema_version",
        audit.schema_version as u64,
        ind,
        false,
    );
    write_kv_str(&mut out, "audit_id", audit.audit_id, ind, true);
    write_kv_str(&mut out, "scenario_label", audit.scenario_label, ind, true);
    write_kv_str(&mut out, "summary", audit.summary, ind, true);
    write_key(&mut out, "query_families", ind, true);
    write_string_array(&mut out, &audit.query_families, ind);
    write_key(&mut out, "order_contract", ind, true);
    write_order_contract(&mut out, &audit.order_contract, ind);
    write_key(&mut out, "drift_detected_if", ind, true);
    write_string_array(&mut out, &audit.drift_detected_if, ind);
    write_kv_str(
        &mut out,
        "recovery_behavior",
        audit.recovery_behavior,
        ind,
        true,
    );
    push_indent(&mut out, 0);
    out.push('}');
    out
}

/// Render the full audit set as one canonical JSON blob.
pub fn invalidation_order_audits_to_json(audits: &[InvalidationOrderAudit]) -> String {
    let mut out = String::new();
    out.push('{');
    let ind = 1;
    write_kv_u64(
        &mut out,
        "schema_version",
        INVALIDATION_ORDER_AUDIT_SCHEMA_VERSION as u64,
        ind,
        false,
    );
    write_key(&mut out, "audits", ind, true);
    out.push('[');
    for (idx, audit) in audits.iter().enumerate() {
        push_indent(&mut out, ind + 1);
        out.push_str(&invalidation_order_audit_to_json(audit));
        if idx + 1 != audits.len() {
            out.push(',');
        }
    }
    push_indent(&mut out, ind);
    out.push(']');
    push_indent(&mut out, 0);
    out.push('}');
    out
}

fn diagnostics_summary_parity_case() -> SnapshotDeltaParityCase {
    let snapshot_states = [
        DiagnosticsProjectionState {
            freshness: Freshness::Cached,
            completeness: Completeness::Full,
            upstream_digest:
                "sha256:00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff",
            files: vec![
                DiagnosticsFileState {
                    file: "src/lib.rs",
                    errors: 0,
                    warnings: 1,
                },
                DiagnosticsFileState {
                    file: "src/store.rs",
                    errors: 1,
                    warnings: 0,
                },
            ],
        },
        DiagnosticsProjectionState {
            freshness: Freshness::Cached,
            completeness: Completeness::Full,
            upstream_digest:
                "sha256:111122223333444455556666777788889999aaaabbbbccccddddeeeeffff0000",
            files: vec![
                DiagnosticsFileState {
                    file: "src/harness.rs",
                    errors: 0,
                    warnings: 1,
                },
                DiagnosticsFileState {
                    file: "src/lib.rs",
                    errors: 0,
                    warnings: 1,
                },
                DiagnosticsFileState {
                    file: "src/store.rs",
                    errors: 2,
                    warnings: 0,
                },
            ],
        },
        DiagnosticsProjectionState {
            freshness: Freshness::Cached,
            completeness: Completeness::Full,
            upstream_digest:
                "sha256:9999aaaabbbbccccddddeeeeffff000011112222333344445555666677778888",
            files: vec![
                DiagnosticsFileState {
                    file: "src/bin/reactive_proto.rs",
                    errors: 1,
                    warnings: 0,
                },
                DiagnosticsFileState {
                    file: "src/harness.rs",
                    errors: 0,
                    warnings: 1,
                },
                DiagnosticsFileState {
                    file: "src/store.rs",
                    errors: 2,
                    warnings: 0,
                },
            ],
        },
    ];
    let delta_steps = [
        vec![
            DiagnosticsDeltaOp::UpsertFile {
                file: "src/store.rs",
                errors: 2,
                warnings: 0,
            },
            DiagnosticsDeltaOp::UpsertFile {
                file: "src/harness.rs",
                errors: 0,
                warnings: 1,
            },
            DiagnosticsDeltaOp::AdvanceUpstreamDigest {
                digest: "sha256:111122223333444455556666777788889999aaaabbbbccccddddeeeeffff0000",
            },
        ],
        vec![
            DiagnosticsDeltaOp::RemoveFile { file: "src/lib.rs" },
            DiagnosticsDeltaOp::UpsertFile {
                file: "src/bin/reactive_proto.rs",
                errors: 1,
                warnings: 0,
            },
            DiagnosticsDeltaOp::AdvanceUpstreamDigest {
                digest: "sha256:9999aaaabbbbccccddddeeeeffff000011112222333344445555666677778888",
            },
        ],
    ];

    let mut delta_state = snapshot_states[0].clone();
    let mut steps = Vec::new();
    steps.push(SnapshotDeltaParityStep {
        step_id: "base_snapshot",
        revision_ref: "diag_rev_1",
        snapshot_view: materialize_diagnostics_summary(&snapshot_states[0]),
        delta_view: materialize_diagnostics_summary(&delta_state),
    });

    for (idx, ops) in delta_steps.iter().enumerate() {
        for op in ops {
            apply_delta(&mut delta_state, *op);
        }
        steps.push(SnapshotDeltaParityStep {
            step_id: if idx == 0 {
                "delta_batch_1"
            } else {
                "delta_batch_2"
            },
            revision_ref: if idx == 0 { "diag_rev_2" } else { "diag_rev_3" },
            snapshot_view: materialize_diagnostics_summary(&snapshot_states[idx + 1]),
            delta_view: materialize_diagnostics_summary(&delta_state),
        });
    }

    SnapshotDeltaParityCase {
        schema_version: SNAPSHOT_DELTA_PARITY_SCHEMA_VERSION,
        case_id: "state.snapshot_delta_parity.language_diagnostics_summary",
        query_family: "language.diagnostics",
        upstream_query_family: "vfs.file_identity",
        view_class: "durable_local_materialization",
        scope_class: "workspace",
        scope_id: "ws-aureline-primary",
        steps,
    }
}

fn materialize_diagnostics_summary(state: &DiagnosticsProjectionState) -> DiagnosticsSummaryView {
    let mut affected_files = state
        .files
        .iter()
        .filter(|file| file.errors > 0 || file.warnings > 0)
        .map(|file| file.file)
        .collect::<Vec<_>>();
    affected_files.sort_unstable();
    DiagnosticsSummaryView {
        freshness: state.freshness,
        completeness: state.completeness,
        upstream_digest: state.upstream_digest,
        total_files_with_diagnostics: affected_files.len() as u64,
        total_errors: state.files.iter().map(|file| file.errors).sum(),
        total_warnings: state.files.iter().map(|file| file.warnings).sum(),
        affected_files,
    }
}

fn apply_delta(state: &mut DiagnosticsProjectionState, op: DiagnosticsDeltaOp) {
    match op {
        DiagnosticsDeltaOp::UpsertFile {
            file,
            errors,
            warnings,
        } => {
            if let Some(existing) = state.files.iter_mut().find(|entry| entry.file == file) {
                existing.errors = errors;
                existing.warnings = warnings;
            } else {
                state.files.push(DiagnosticsFileState {
                    file,
                    errors,
                    warnings,
                });
                state.files.sort_unstable_by_key(|entry| entry.file);
            }
        }
        DiagnosticsDeltaOp::RemoveFile { file } => {
            state.files.retain(|entry| entry.file != file);
        }
        DiagnosticsDeltaOp::AdvanceUpstreamDigest { digest } => {
            state.upstream_digest = digest;
        }
    }
}

fn refresh_ordering_audit(report: &ScenarioReport) -> InvalidationOrderAudit {
    let authority_refresh = first_frame(report, |query_family, frame_class, snapshot_epoch, _| {
        query_family == "vfs.file_identity" && frame_class == "snapshot" && snapshot_epoch == 2
    });
    let derived_resync = first_frame(report, |query_family, frame_class, _, _| {
        query_family == "language.diagnostics" && frame_class == "resync_required"
    });
    let derived_refresh = first_frame(report, |query_family, frame_class, snapshot_epoch, _| {
        query_family == "language.diagnostics" && frame_class == "snapshot" && snapshot_epoch == 2
    });
    InvalidationOrderAudit {
        schema_version: INVALIDATION_ORDER_AUDIT_SCHEMA_VERSION,
        file_stem: "authority_before_derived_refresh",
        audit_id: "state.invalidation_order.authority_before_derived_refresh",
        scenario_label: report.label,
        summary:
            "Authority refresh must land before the derived view invalidates and republishes.",
        query_families: vec!["vfs.file_identity", "language.diagnostics"],
        order_contract: vec![
            step_from_frame(
                1,
                authority_refresh,
                "authoritative workspace truth republishes before any derived stale or refresh frame",
            ),
            step_from_frame(
                2,
                derived_resync,
                "derived consumer marks itself stale only after authority state has advanced",
            ),
            step_from_frame(
                3,
                derived_refresh,
                "derived refresh completes only after stale labeling has already been made explicit",
            ),
        ],
        drift_detected_if: vec![
            "language.diagnostics emits resync_required before vfs.file_identity publishes snapshot_epoch=2",
            "language.diagnostics emits its fresh snapshot before the stale resync_required frame",
        ],
        recovery_behavior:
            "Keep exact derived actions disabled until the authoritative refresh and the post-resync derived snapshot both land.",
    }
}

fn delta_gap_resync_audit(report: &ScenarioReport) -> InvalidationOrderAudit {
    let gap_note = first_note(report, "delta_gap_detected");
    let resync = first_frame(report, |query_family, frame_class, _, _| {
        query_family == "vfs.file_identity" && frame_class == "resync_required"
    });
    let fresh_snapshot = first_frame(report, |query_family, frame_class, snapshot_epoch, _| {
        query_family == "vfs.file_identity" && frame_class == "snapshot" && snapshot_epoch == 2
    });
    InvalidationOrderAudit {
        schema_version: INVALIDATION_ORDER_AUDIT_SCHEMA_VERSION,
        file_stem: "delta_gap_requires_resync",
        audit_id: "state.invalidation_order.delta_gap_requires_resync",
        scenario_label: report.label,
        summary:
            "A detected delta gap must surface as an explicit stale transition before the replacement snapshot lands.",
        query_families: vec!["vfs.file_identity"],
        order_contract: vec![
            step_from_note(
                1,
                gap_note,
                "consumer records the causality break before any replacement state is trusted",
            ),
            step_from_frame(
                2,
                resync,
                "stale labeling is emitted immediately after the gap is detected",
            ),
            step_from_frame(
                3,
                fresh_snapshot,
                "fresh snapshot_epoch=2 replaces the stale lineage only after resync_required was observed",
            ),
        ],
        drift_detected_if: vec![
            "snapshot_epoch=2 arrives without a preceding delta_gap_detected note and resync_required frame",
            "consumer continues to accept epoch-1 deltas after the resync_required frame",
        ],
        recovery_behavior:
            "Discard epoch-1 deltas, preserve the stale snapshot only for review, and wait for the replacement snapshot before resuming exact actions.",
    }
}

fn snapshot_required_switch_audit(report: &ScenarioReport) -> InvalidationOrderAudit {
    let coalesce_note = first_note(report, "subscription_backpressure_coalesce");
    let switch_note = first_note(report, "subscription_snapshot_required_switch");
    let resync = first_frame(report, |query_family, frame_class, _, _| {
        query_family == "vfs.workspace_readiness" && frame_class == "resync_required"
    });
    let post_switch_snapshot =
        first_frame(report, |query_family, frame_class, snapshot_epoch, _| {
            query_family == "vfs.workspace_readiness"
                && frame_class == "snapshot"
                && snapshot_epoch == 2
        });
    InvalidationOrderAudit {
        schema_version: INVALIDATION_ORDER_AUDIT_SCHEMA_VERSION,
        file_stem: "snapshot_required_switch",
        audit_id: "state.invalidation_order.snapshot_required_switch",
        scenario_label: report.label,
        summary:
            "Coalesced delivery that can no longer preserve parity must escalate to snapshot_required before the replacement snapshot is trusted.",
        query_families: vec!["vfs.workspace_readiness"],
        order_contract: vec![
            step_from_note(
                1,
                coalesce_note,
                "producer discloses the coalesced backlog instead of silently pretending no lag exists",
            ),
            step_from_note(
                2,
                switch_note,
                "consumer escalation to snapshot_required is durable and reviewable",
            ),
            step_from_frame(
                3,
                resync,
                "causal continuity is explicitly abandoned before the next fresh snapshot",
            ),
            step_from_frame(
                4,
                post_switch_snapshot,
                "the replacement snapshot is trusted only after the snapshot_required resync has already landed",
            ),
        ],
        drift_detected_if: vec![
            "coalesced delivery switches to snapshot_required without recording the switch note",
            "a new snapshot lands before the stale resync_required frame for the same subscription",
        ],
        recovery_behavior:
            "Surface coalesced backlog, label the projection stale, and replace it with a new snapshot lineage under backpressure_mode=snapshot_required.",
    }
}

fn find_scenario<'a>(scenarios: &'a [ScenarioReport], label: &str) -> &'a ScenarioReport {
    scenarios
        .iter()
        .find(|scenario| scenario.label == label)
        .unwrap_or_else(|| panic!("missing scenario: {label}"))
}

fn first_frame<F>(report: &ScenarioReport, predicate: F) -> &TraceEvent
where
    F: Fn(&str, &str, u64, u64) -> bool,
{
    report
        .trace
        .iter()
        .find(|event| match event {
            TraceEvent::FrameEmit { envelope, .. } => predicate(
                &envelope.query_family,
                envelope.frame_class.as_str(),
                envelope.snapshot_epoch,
                envelope.delta_seq,
            ),
            _ => false,
        })
        .unwrap_or_else(|| panic!("missing frame in scenario {}", report.label))
}

fn first_note<'a>(report: &'a ScenarioReport, hook_id: &str) -> &'a TraceEvent {
    report
        .trace
        .iter()
        .find(|event| matches!(event, TraceEvent::Note { hook_id: current, .. } if *current == hook_id))
        .unwrap_or_else(|| panic!("missing note {hook_id} in scenario {}", report.label))
}

fn step_from_frame(
    ordinal: u64,
    event: &TraceEvent,
    expectation: &'static str,
) -> InvalidationOrderStep {
    match event {
        TraceEvent::FrameEmit {
            tick,
            hook_id,
            envelope,
            observation: _,
        } => InvalidationOrderStep {
            ordinal,
            tick: *tick,
            event_kind: "frame_emit",
            hook_id,
            query_family: Some(envelope.query_family.clone()),
            frame_class: Some(envelope.frame_class.as_str().to_owned()),
            snapshot_epoch: Some(envelope.snapshot_epoch),
            delta_seq: Some(envelope.delta_seq),
            freshness: Some(envelope.freshness.as_str().to_owned()),
            completeness: Some(envelope.completeness.as_str().to_owned()),
            stale_reason: envelope
                .invalidation
                .as_ref()
                .map(|invalidation| invalidation.stale_reason.as_str().to_owned()),
            message: None,
            expectation,
        },
        _ => panic!("expected frame event"),
    }
}

fn step_from_note(
    ordinal: u64,
    event: &TraceEvent,
    expectation: &'static str,
) -> InvalidationOrderStep {
    match event {
        TraceEvent::Note {
            tick,
            hook_id,
            message,
        } => InvalidationOrderStep {
            ordinal,
            tick: *tick,
            event_kind: "note",
            hook_id,
            query_family: None,
            frame_class: None,
            snapshot_epoch: None,
            delta_seq: None,
            freshness: None,
            completeness: None,
            stale_reason: None,
            message: Some(message.clone()),
            expectation,
        },
        _ => panic!("expected note event"),
    }
}

fn push_indent(out: &mut String, indent: usize) {
    out.push('\n');
    for _ in 0..indent {
        out.push_str("  ");
    }
}

fn write_key(out: &mut String, key: &str, indent: usize, leading_comma: bool) {
    if leading_comma {
        out.push(',');
    }
    push_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": ");
}

fn write_kv_str(out: &mut String, key: &str, value: &str, indent: usize, leading_comma: bool) {
    write_key(out, key, indent, leading_comma);
    write_json_string(out, value);
}

fn write_kv_u64(out: &mut String, key: &str, value: u64, indent: usize, leading_comma: bool) {
    write_key(out, key, indent, leading_comma);
    let _ = write!(out, "{value}");
}

fn write_kv_optional_string(
    out: &mut String,
    key: &str,
    value: Option<&str>,
    indent: usize,
    leading_comma: bool,
) {
    write_key(out, key, indent, leading_comma);
    match value {
        Some(value) => write_json_string(out, value),
        None => out.push_str("null"),
    }
}

fn write_kv_optional_u64(
    out: &mut String,
    key: &str,
    value: Option<u64>,
    indent: usize,
    leading_comma: bool,
) {
    write_key(out, key, indent, leading_comma);
    match value {
        Some(value) => {
            let _ = write!(out, "{value}");
        }
        None => out.push_str("null"),
    }
}

fn write_string_array(out: &mut String, values: &[&str], indent: usize) {
    out.push('[');
    for (idx, value) in values.iter().enumerate() {
        push_indent(out, indent + 1);
        write_json_string(out, value);
        if idx + 1 != values.len() {
            out.push(',');
        }
    }
    push_indent(out, indent);
    out.push(']');
}

fn write_order_contract(out: &mut String, steps: &[InvalidationOrderStep], indent: usize) {
    out.push('[');
    for (idx, step) in steps.iter().enumerate() {
        push_indent(out, indent + 1);
        out.push('{');
        write_kv_u64(out, "ordinal", step.ordinal, indent + 2, false);
        write_kv_u64(out, "tick", step.tick, indent + 2, true);
        write_kv_str(out, "event_kind", step.event_kind, indent + 2, true);
        write_kv_str(out, "hook_id", step.hook_id, indent + 2, true);
        write_kv_optional_string(
            out,
            "query_family",
            step.query_family.as_deref(),
            indent + 2,
            true,
        );
        write_kv_optional_string(
            out,
            "frame_class",
            step.frame_class.as_deref(),
            indent + 2,
            true,
        );
        write_kv_optional_u64(out, "snapshot_epoch", step.snapshot_epoch, indent + 2, true);
        write_kv_optional_u64(out, "delta_seq", step.delta_seq, indent + 2, true);
        write_kv_optional_string(
            out,
            "freshness",
            step.freshness.as_deref(),
            indent + 2,
            true,
        );
        write_kv_optional_string(
            out,
            "completeness",
            step.completeness.as_deref(),
            indent + 2,
            true,
        );
        write_kv_optional_string(
            out,
            "stale_reason",
            step.stale_reason.as_deref(),
            indent + 2,
            true,
        );
        write_kv_optional_string(out, "message", step.message.as_deref(), indent + 2, true);
        write_kv_str(out, "expectation", step.expectation, indent + 2, true);
        push_indent(out, indent + 1);
        out.push('}');
        if idx + 1 != steps.len() {
            out.push(',');
        }
    }
    push_indent(out, indent);
    out.push(']');
}

fn write_json_string(out: &mut String, value: &str) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_summary_is_equivalent_under_snapshot_and_delta_paths() {
        let cases = run_snapshot_delta_parity_cases();
        assert_eq!(cases.len(), 1);
        let case = &cases[0];
        for step in &case.steps {
            assert_eq!(
                step.snapshot_view, step.delta_view,
                "parity mismatch at {}",
                step.step_id
            );
        }
        let final_view = &case.steps.last().expect("final parity step").snapshot_view;
        assert_eq!(final_view.total_errors, 3);
        assert_eq!(final_view.total_warnings, 1);
        assert_eq!(
            final_view.affected_files,
            vec![
                "src/bin/reactive_proto.rs",
                "src/harness.rs",
                "src/store.rs"
            ]
        );
    }

    #[test]
    fn invalidation_order_audits_have_strictly_increasing_ticks() {
        for audit in run_invalidation_order_audits() {
            let mut last_tick = 0;
            for step in &audit.order_contract {
                assert!(
                    step.tick > last_tick,
                    "ticks must increase for audit {}",
                    audit.audit_id
                );
                last_tick = step.tick;
            }
        }
    }

    #[test]
    fn invalidation_order_audits_are_byte_stable() {
        let first = run_invalidation_order_audits();
        let second = run_invalidation_order_audits();
        assert_eq!(
            invalidation_order_audits_to_json(&first),
            invalidation_order_audits_to_json(&second)
        );
    }

    #[test]
    fn invalidation_order_audits_cover_required_paths() {
        let audits = run_invalidation_order_audits();
        let ids = audits
            .iter()
            .map(|audit| audit.audit_id)
            .collect::<Vec<_>>();
        assert!(ids.contains(&"state.invalidation_order.authority_before_derived_refresh"));
        assert!(ids.contains(&"state.invalidation_order.delta_gap_requires_resync"));
        assert!(ids.contains(&"state.invalidation_order.snapshot_required_switch"));
    }
}
