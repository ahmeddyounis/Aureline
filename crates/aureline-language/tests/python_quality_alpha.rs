use std::path::Path;

use aureline_language::{
    DiagnosticBusAggregateCounts, DiagnosticEvidencePlaneClass, DiagnosticSourceFamily,
    DiagnosticSurfaceClass, PythonQualityAggregateCounts, PythonQualityExecutionPlaneProjection,
    PythonQualityRerunPostureClass, PythonQualitySeedSnapshot, PythonQualitySnapshot,
    PythonQualitySnapshotRequest, PythonQualityWedge, RouterHealthState,
    DIAGNOSTIC_BUS_SCHEMA_VERSION, PYTHON_QUALITY_ALPHA_SCHEMA_VERSION,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct Case {
    case_id: String,
    snapshot_id: String,
    collection_id: String,
    captured_at: String,
    seed: PythonQualitySeedSnapshot,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Expected {
    total_count: usize,
    error_count: usize,
    warning_count: usize,
    notice_count: usize,
    hint_count: usize,
    partial_count: usize,
    stale_or_unverified_count: usize,
    editor_inline_count: usize,
    support_export_visible_count: usize,
    unavailable_provider_count: usize,
    degraded_tool_count: usize,
    unavailable_tool_count: usize,
    runnable_hook_count: usize,
    blocked_hook_count: usize,
    interpreter_blocked_hook_count: usize,
    preview_required_hook_count: usize,
    requires_degraded_disclosure: bool,
}

#[test]
fn python_quality_wedge_normalizes_diagnostics_and_task_hooks() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "python_quality_alpha_cases");
    assert_eq!(fixture.schema_version, PYTHON_QUALITY_ALPHA_SCHEMA_VERSION);

    for case in &fixture.cases {
        assert_eq!(
            case.seed.record_kind,
            PythonQualitySeedSnapshot::RECORD_KIND
        );
        assert_eq!(
            case.seed.schema_version,
            PYTHON_QUALITY_ALPHA_SCHEMA_VERSION
        );

        let wedge = PythonQualityWedge::new(case.seed.clone());
        let snapshot = wedge.snapshot(PythonQualitySnapshotRequest {
            snapshot_id: case.snapshot_id.clone(),
            collection_id: case.collection_id.clone(),
            captured_at: case.captured_at.clone(),
        });

        assert_eq!(snapshot.record_kind, PythonQualitySnapshot::RECORD_KIND);
        assert_eq!(snapshot.schema_version, PYTHON_QUALITY_ALPHA_SCHEMA_VERSION);
        assert_eq!(
            snapshot
                .diagnostic_bus_snapshot
                .diagnostic_bus_schema_version,
            DIAGNOSTIC_BUS_SCHEMA_VERSION
        );
        assert_eq!(
            snapshot.diagnostic_bus_snapshot.aggregate_counts,
            DiagnosticBusAggregateCounts {
                total_count: case.expected.total_count,
                error_count: case.expected.error_count,
                warning_count: case.expected.warning_count,
                notice_count: case.expected.notice_count,
                hint_count: case.expected.hint_count,
                local_count: case.expected.total_count,
                imported_count: 0,
                cached_count: 0,
                partial_count: case.expected.partial_count,
                stale_or_unverified_count: case.expected.stale_or_unverified_count,
                unavailable_provider_count: case.expected.unavailable_provider_count,
            },
            "diagnostic counts mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.aggregate_counts,
            PythonQualityAggregateCounts {
                normalized_diagnostic_count: case.expected.total_count,
                tool_count: 3,
                degraded_tool_count: case.expected.degraded_tool_count,
                unavailable_tool_count: case.expected.unavailable_tool_count,
                task_hook_count: 4,
                runnable_hook_count: case.expected.runnable_hook_count,
                blocked_hook_count: case.expected.blocked_hook_count,
                interpreter_blocked_hook_count: case.expected.interpreter_blocked_hook_count,
                preview_required_hook_count: case.expected.preview_required_hook_count,
            },
            "quality counts mismatch for {}",
            case.case_id
        );
        assert_eq!(
            snapshot.requires_degraded_disclosure(),
            case.expected.requires_degraded_disclosure,
            "degraded disclosure mismatch for {}",
            case.case_id
        );

        assert_quality_sources_are_preserved(&snapshot, &case.case_id);
        assert_task_hooks_are_execution_plane_ready(&snapshot, case);
        assert_projection_consumes_diagnostics_and_hooks(&snapshot, case);
        assert_snapshot_round_trips(&snapshot);
    }
}

fn assert_quality_sources_are_preserved(snapshot: &PythonQualitySnapshot, case_id: &str) {
    assert!(
        snapshot
            .diagnostic_bus_snapshot
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.source.source_family
                == DiagnosticSourceFamily::LinterFormatterStyle
                && diagnostic.source.evidence_plane_class
                    == DiagnosticEvidencePlaneClass::StaticAnalysis),
        "formatter/linter diagnostic source missing for {case_id}"
    );
    assert!(
        snapshot
            .diagnostic_bus_snapshot
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.source.source_family
                == DiagnosticSourceFamily::RuntimeTestOrDebug
                && diagnostic.source.evidence_plane_class
                    == DiagnosticEvidencePlaneClass::RuntimeOrTestExecution),
        "runtime/test diagnostic source missing for {case_id}"
    );
    assert!(
        snapshot.task_hooks.iter().all(|hook| hook.interpreter_ref
            == snapshot
                .workspace_context
                .interpreter_context
                .interpreter_ref),
        "task hooks must preserve interpreter ref for {case_id}"
    );
    assert!(
        snapshot
            .tool_rows
            .iter()
            .any(|tool| tool.health_state != RouterHealthState::Ready),
        "fixture must include one missing interpreter, linter, or test tool for {case_id}"
    );
}

fn assert_task_hooks_are_execution_plane_ready(snapshot: &PythonQualitySnapshot, case: &Case) {
    for hook in &snapshot.task_hooks {
        assert_eq!(
            hook.execution_context_id, snapshot.workspace_context.execution_context_id,
            "hook execution context mismatch for {}",
            case.case_id
        );
        assert!(
            !hook.source_diagnostic_refs.is_empty(),
            "hook must cite diagnostics for {}",
            case.case_id
        );
        assert!(
            !hook.normalized_task_event_refs.is_empty(),
            "hook must cite normalized task events for {}",
            case.case_id
        );
    }

    if case.case_id.contains("interpreter-missing") {
        assert!(snapshot.task_hooks.iter().all(|hook| {
            hook.rerun_posture_class
                == PythonQualityRerunPostureClass::BlockedInterpreterUnavailable
        }));
    }
    if case.case_id.contains("linter-missing") {
        assert!(snapshot.task_hooks.iter().any(|hook| {
            hook.rerun_posture_class == PythonQualityRerunPostureClass::BlockedToolUnavailable
        }));
    }
    if case.case_id.contains("pytest-missing") {
        assert!(snapshot.task_hooks.iter().any(|hook| {
            hook.rerun_posture_class == PythonQualityRerunPostureClass::BlockedToolUnavailable
        }));
    }
}

fn assert_projection_consumes_diagnostics_and_hooks(snapshot: &PythonQualitySnapshot, case: &Case) {
    let editor_projection = snapshot
        .diagnostic_bus_snapshot
        .surface_projection(DiagnosticSurfaceClass::EditorInline, &case.captured_at);
    assert_eq!(
        editor_projection.visible_count, case.expected.editor_inline_count,
        "editor inline count mismatch for {}",
        case.case_id
    );

    let support_projection = snapshot
        .diagnostic_bus_snapshot
        .surface_projection(DiagnosticSurfaceClass::SupportExport, &case.captured_at);
    assert_eq!(
        support_projection.visible_count, case.expected.support_export_visible_count,
        "support projection count mismatch for {}",
        case.case_id
    );

    let execution_projection = snapshot
        .execution_plane_projection(DiagnosticSurfaceClass::SupportExport, &case.captured_at);
    assert_eq!(
        execution_projection.record_kind,
        PythonQualityExecutionPlaneProjection::RECORD_KIND
    );
    assert_eq!(
        execution_projection.runnable_task_hook_ids.len(),
        case.expected.runnable_hook_count,
        "runnable hook projection mismatch for {}",
        case.case_id
    );
    assert_eq!(
        execution_projection.blocked_task_hook_ids.len(),
        case.expected.blocked_hook_count,
        "blocked hook projection mismatch for {}",
        case.case_id
    );
    assert_eq!(
        execution_projection.preview_required_task_hook_ids.len(),
        case.expected.preview_required_hook_count,
        "preview-required hook projection mismatch for {}",
        case.case_id
    );
    assert_eq!(
        execution_projection.interpreter_ref,
        snapshot
            .workspace_context
            .interpreter_context
            .interpreter_ref
    );
    assert_eq!(
        execution_projection.disclosure_required,
        case.expected.requires_degraded_disclosure
    );
    assert_projection_round_trips(&execution_projection);
}

fn assert_snapshot_round_trips(snapshot: &PythonQualitySnapshot) {
    let serialized = serde_json::to_string(snapshot).expect("quality snapshot serializes");
    let round_trip: PythonQualitySnapshot =
        serde_json::from_str(&serialized).expect("quality snapshot deserializes");
    assert_eq!(round_trip, *snapshot);
}

fn assert_projection_round_trips(projection: &PythonQualityExecutionPlaneProjection) {
    let serialized = serde_json::to_string(projection).expect("quality projection serializes");
    let round_trip: PythonQualityExecutionPlaneProjection =
        serde_json::from_str(&serialized).expect("quality projection deserializes");
    assert_eq!(round_trip, *projection);
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/python_quality_alpha/quality_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
