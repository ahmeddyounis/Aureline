use std::path::Path;

use aureline_language::{
    navigation_target_model::{AccessKind, RelationKind},
    python_navigation_target, python_reference_occurrences, python_rename_preview_set,
    CodeActionRefactorScopeAdmissionClass, CodeActionScopeWideningReviewTriggerClass,
    LanguageServerHostIdentity, LanguageServerHostStatus, PythonHoverRecord,
    PythonInterpreterReadinessClass, PythonInterpreterSelectionStateClass, PythonLaunchWedge,
    PythonLaunchWedgeSnapshot, PythonReferenceSetRecord, PythonRenamePreviewRecord,
    PythonSemanticResultRecord, RouterCapabilityClass, RouterCompletenessClass,
    RouterFallbackClass, RouterFaultDomainId, RouterFreshnessClass, RouterHealthState,
    RouterLocalityClass, RouterScopeClaimClass, ScopeLimitClass,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    snapshot: PythonLaunchWedgeSnapshot,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct Case {
    case_id: String,
    provider_id: String,
    server_label: String,
    host_health: String,
    host_freshness: String,
    host_completeness: String,
    scope_claim: String,
    scope_limits: Vec<String>,
    host_execution_context_id: Option<String>,
    interpreter_selection_state: Option<String>,
    interpreter_readiness: Option<String>,
    expect: Expect,
}

#[derive(Debug, Deserialize)]
struct Expect {
    hover_layer: String,
    definition_confidence: String,
    definition_requires_disclosure: bool,
    references_materialized_count: usize,
    references_requires_disclosure: bool,
    rename_preview_completeness: String,
    rename_apply_posture: String,
    rename_changed_count: usize,
    rename_skipped_count: usize,
    rename_generated_count: usize,
    rename_protected_count: usize,
    rename_requires_disclosure: bool,
    interpreter_requires_disclosure: bool,
}

#[test]
fn python_wedge_emits_hover_definition_references_and_rename_preview() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "python_nav_alpha_cases");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(
        fixture.snapshot.record_kind,
        PythonLaunchWedgeSnapshot::RECORD_KIND
    );
    assert_eq!(fixture.snapshot.schema_version, 1);

    for case in &fixture.cases {
        let snapshot = snapshot_for_case(&fixture.snapshot, case);
        let wedge = PythonLaunchWedge::new(snapshot.clone());
        let host_status = host_status(&snapshot, case);
        let host_statuses = [host_status];

        let hover = wedge
            .hover("symbol:python:calculate_discount", &host_statuses)
            .unwrap_or_else(|err| panic!("hover should build for {}: {err}", case.case_id));
        assert_eq!(
            enum_token(hover.answer_layer_class),
            case.expect.hover_layer,
            "hover layer mismatch for {}",
            case.case_id
        );
        assert_hover_round_trips(&hover);
        assert_eq!(
            hover
                .provider_snapshot
                .interpreter_context
                .requires_disclosure(),
            case.expect.interpreter_requires_disclosure,
            "hover interpreter disclosure mismatch for {}",
            case.case_id
        );

        let definition = wedge
            .definition("symbol:python:calculate_discount", &host_statuses)
            .unwrap_or_else(|err| panic!("definition should build for {}: {err}", case.case_id));
        assert_eq!(
            enum_token(definition.result_confidence_class),
            case.expect.definition_confidence,
            "definition confidence mismatch for {}",
            case.case_id
        );
        assert_eq!(
            definition.requires_degraded_disclosure(),
            case.expect.definition_requires_disclosure,
            "definition disclosure mismatch for {}",
            case.case_id
        );
        assert_semantic_result_round_trips(&definition);
        let shared_target = python_navigation_target(&definition);
        assert_eq!(shared_target.relation_kind, RelationKind::Definition);
        assert_eq!(
            shared_target.requires_downgrade_disclosure(),
            case.expect.definition_requires_disclosure,
            "shared target disclosure mismatch for {}",
            case.case_id
        );
        assert_eq!(
            definition
                .provider_snapshot
                .interpreter_context
                .environment_ref,
            snapshot
                .workspace_context
                .interpreter_context
                .environment_ref,
            "definition must preserve interpreter environment for {}",
            case.case_id
        );

        let references = wedge
            .references("symbol:python:calculate_discount", &host_statuses)
            .unwrap_or_else(|err| panic!("references should build for {}: {err}", case.case_id));
        assert_eq!(
            references.count_summary.materialized_count, case.expect.references_materialized_count,
            "reference materialized count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            references.requires_degraded_disclosure(),
            case.expect.references_requires_disclosure,
            "references disclosure mismatch for {}",
            case.case_id
        );
        assert_reference_set_round_trips(&references);
        let shared_occurrences = python_reference_occurrences(&references);
        assert_eq!(
            shared_occurrences.len(),
            case.expect.references_materialized_count,
            "shared reference count mismatch for {}",
            case.case_id
        );
        if case.expect.references_materialized_count > 1 {
            assert!(
                shared_occurrences
                    .iter()
                    .any(|occurrence| occurrence.access_kind == AccessKind::Call),
                "shared references must preserve call access for {}",
                case.case_id
            );
        }

        let rename_preview = wedge
            .rename_preview(
                "symbol:python:calculate_discount",
                "symbol:name:compute_discount",
                &host_statuses,
            )
            .unwrap_or_else(|err| {
                panic!("rename preview should build for {}: {err}", case.case_id)
            });
        assert_eq!(
            enum_token(rename_preview.preview_completeness_class),
            case.expect.rename_preview_completeness,
            "rename completeness mismatch for {}",
            case.case_id
        );
        assert_eq!(
            enum_token(rename_preview.apply_posture_class),
            case.expect.rename_apply_posture,
            "rename apply posture mismatch for {}",
            case.case_id
        );
        assert_eq!(
            rename_preview.count_summary.changed_count, case.expect.rename_changed_count,
            "rename changed count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            rename_preview.count_summary.skipped_count, case.expect.rename_skipped_count,
            "rename skipped count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            rename_preview.count_summary.generated_count, case.expect.rename_generated_count,
            "rename generated count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            rename_preview.count_summary.protected_count, case.expect.rename_protected_count,
            "rename protected count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            rename_preview.requires_degraded_disclosure(),
            case.expect.rename_requires_disclosure,
            "rename disclosure mismatch for {}",
            case.case_id
        );
        assert!(
            !rename_preview.affected_scope_rows.is_empty(),
            "rename preview must always label scope for {}",
            case.case_id
        );
        let outside_test_ref =
            "nav:semantic:result:python:reference:symbol-python-calculate_discount:occ-python-calculate_discount-test-call"
                .to_string();
        assert!(
            !rename_preview
                .refactor_scope_binding
                .admitted_target_refs
                .contains(&outside_test_ref),
            "rename preview must not admit outside-workset test occurrence for {}",
            case.case_id
        );
        assert!(
            rename_preview
                .refactor_scope_binding
                .refused_target_refs
                .contains(&outside_test_ref),
            "rename preview must refuse outside-workset test occurrence for {}",
            case.case_id
        );
        assert_rename_preview_round_trips(&rename_preview);
        let shared_preview = python_rename_preview_set(&rename_preview);
        assert_eq!(
            shared_preview.count_summary.changed_count, case.expect.rename_changed_count,
            "shared rename changed count mismatch for {}",
            case.case_id
        );
        assert_eq!(
            shared_preview.requires_downgrade_disclosure(),
            case.expect.rename_requires_disclosure,
            "shared rename disclosure mismatch for {}",
            case.case_id
        );
        if case.expect.rename_generated_count > 0 {
            assert!(
                !shared_preview.generated_scope_notes.is_empty(),
                "shared rename preview must preserve generated notes for {}",
                case.case_id
            );
        }
        assert_eq!(
            rename_preview
                .provider_snapshot
                .interpreter_context
                .interpreter_ref,
            snapshot
                .workspace_context
                .interpreter_context
                .interpreter_ref,
            "rename preview must stay bound to selected interpreter for {}",
            case.case_id
        );
    }

    let ready_case = fixture
        .cases
        .iter()
        .find(|case| case.case_id == "python-nav-ready-semantic")
        .expect("ready semantic case");
    let ready_snapshot = snapshot_for_case(&fixture.snapshot, ready_case);
    let ready_wedge = PythonLaunchWedge::new(ready_snapshot.clone());
    let ready_host = host_status(&ready_snapshot, ready_case);
    let widening_preview = ready_wedge
        .rename_preview_with_scope_widening_review(
            "symbol:python:calculate_discount",
            "symbol:name:compute_discount",
            &[ready_host],
            true,
        )
        .expect("scope-widening rename preview");
    assert_eq!(
        widening_preview.refactor_scope_binding.admission_class,
        CodeActionRefactorScopeAdmissionClass::BlockedPendingScopeWideningReview
    );
    let review = widening_preview
        .refactor_scope_binding
        .scope_widening_review
        .as_ref()
        .expect("widening attempt prompts review");
    assert_eq!(
        review.trigger_class,
        CodeActionScopeWideningReviewTriggerClass::RefactorWiden
    );
    assert!(review.typed_confirmation_required);
    assert!(review
        .requested_scope_refs
        .contains(&"scope:root:payments-tests".to_string()));
}

fn snapshot_for_case(base: &PythonLaunchWedgeSnapshot, case: &Case) -> PythonLaunchWedgeSnapshot {
    let mut snapshot = base.clone();
    if let Some(selection_state) = &case.interpreter_selection_state {
        snapshot
            .workspace_context
            .interpreter_context
            .selection_state_class = interpreter_selection_state(selection_state);
    }
    if let Some(readiness) = &case.interpreter_readiness {
        snapshot
            .workspace_context
            .interpreter_context
            .readiness_class = interpreter_readiness(readiness);
    }
    snapshot
}

fn host_status(snapshot: &PythonLaunchWedgeSnapshot, case: &Case) -> LanguageServerHostStatus {
    LanguageServerHostStatus {
        identity: LanguageServerHostIdentity {
            host_instance_id: format!("host:lsp:{}", case.case_id),
            provider_id: case.provider_id.clone(),
            workspace_id: snapshot.workspace_context.workspace_id.clone(),
            root_ref: snapshot.workspace_context.subject_root_ref.clone(),
            language_id: snapshot.language_id.clone(),
            server_label: case.server_label.clone(),
            execution_context_id: case
                .host_execution_context_id
                .clone()
                .unwrap_or_else(|| snapshot.workspace_context.execution_context_id.clone()),
            locality_class: RouterLocalityClass::LocalSidecar,
            fault_domain_id: RouterFaultDomainId::SessionScopedExecutionHosts,
            restart_budget_ref: "restart_budget:session_scoped_execution_hosts:language:python:01"
                .into(),
        },
        health_state: health(&case.host_health),
        freshness_class: freshness(&case.host_freshness),
        scope_claim_class: scope_claim(&case.scope_claim),
        completeness_class: completeness(&case.host_completeness),
        scope_limit_classes: case
            .scope_limits
            .iter()
            .map(|limit| scope_limit(limit))
            .collect(),
        supported_capability_classes: vec![
            RouterCapabilityClass::Hover,
            RouterCapabilityClass::Definition,
            RouterCapabilityClass::Reference,
            RouterCapabilityClass::Rename,
        ],
        restart_strike_count: usize::from(case.host_health == "unavailable") as u32,
        quarantine_ref: None,
        fallback_class: RouterFallbackClass::ProtocolToText,
        health_summary: format!("{} is {}.", case.server_label, case.host_health),
    }
}

fn interpreter_selection_state(value: &str) -> PythonInterpreterSelectionStateClass {
    match value {
        "selected" => PythonInterpreterSelectionStateClass::Selected,
        "ambiguous" => PythonInterpreterSelectionStateClass::Ambiguous,
        "missing" => PythonInterpreterSelectionStateClass::Missing,
        "drifted" => PythonInterpreterSelectionStateClass::Drifted,
        other => panic!("unknown interpreter selection state {other}"),
    }
}

fn interpreter_readiness(value: &str) -> PythonInterpreterReadinessClass {
    match value {
        "ready_for_analysis" => PythonInterpreterReadinessClass::ReadyForAnalysis,
        "partial_environment" => PythonInterpreterReadinessClass::PartialEnvironment,
        "unavailable" => PythonInterpreterReadinessClass::Unavailable,
        other => panic!("unknown interpreter readiness {other}"),
    }
}

fn health(value: &str) -> RouterHealthState {
    match value {
        "ready" => RouterHealthState::Ready,
        "unavailable" => RouterHealthState::Unavailable,
        other => panic!("unknown health {other}"),
    }
}

fn freshness(value: &str) -> RouterFreshnessClass {
    match value {
        "authoritative_live" => RouterFreshnessClass::AuthoritativeLive,
        "unverified" => RouterFreshnessClass::Unverified,
        other => panic!("unknown freshness {other}"),
    }
}

fn completeness(value: &str) -> RouterCompletenessClass {
    match value {
        "complete_for_claimed_scope" => RouterCompletenessClass::CompleteForClaimedScope,
        "partial_for_claimed_scope" => RouterCompletenessClass::PartialForClaimedScope,
        "unavailable_for_claimed_scope" => RouterCompletenessClass::UnavailableForClaimedScope,
        other => panic!("unknown completeness {other}"),
    }
}

fn scope_claim(value: &str) -> RouterScopeClaimClass {
    match value {
        "active_workset" => RouterScopeClaimClass::ActiveWorkset,
        "single_file" => RouterScopeClaimClass::SingleFile,
        "whole_workspace" => RouterScopeClaimClass::WholeWorkspace,
        other => panic!("unknown scope claim {other}"),
    }
}

fn scope_limit(value: &str) -> ScopeLimitClass {
    match value {
        "active_workset_only" => ScopeLimitClass::ActiveWorksetOnly,
        "unloaded_roots_omitted" => ScopeLimitClass::UnloadedRootsOmitted,
        "generated_candidates_omitted" => ScopeLimitClass::GeneratedCandidatesOmitted,
        other => panic!("unknown scope limit {other}"),
    }
}

fn assert_hover_round_trips(record: &PythonHoverRecord) {
    let serialized = serde_json::to_string(record).expect("hover serializes");
    let round_trip: PythonHoverRecord =
        serde_json::from_str(&serialized).expect("hover deserializes");
    assert_eq!(round_trip, *record);
}

fn assert_semantic_result_round_trips(record: &PythonSemanticResultRecord) {
    let serialized = serde_json::to_string(record).expect("semantic result serializes");
    let round_trip: PythonSemanticResultRecord =
        serde_json::from_str(&serialized).expect("semantic result deserializes");
    assert_eq!(round_trip, *record);
}

fn assert_reference_set_round_trips(record: &PythonReferenceSetRecord) {
    let serialized = serde_json::to_string(record).expect("reference set serializes");
    let round_trip: PythonReferenceSetRecord =
        serde_json::from_str(&serialized).expect("reference set deserializes");
    assert_eq!(round_trip, *record);
}

fn assert_rename_preview_round_trips(record: &PythonRenamePreviewRecord) {
    let serialized = serde_json::to_string(record).expect("rename preview serializes");
    let round_trip: PythonRenamePreviewRecord =
        serde_json::from_str(&serialized).expect("rename preview deserializes");
    assert_eq!(round_trip, *record);
}

fn enum_token<T: serde::Serialize>(value: T) -> String {
    serde_json::to_value(value)
        .expect("enum serializes")
        .as_str()
        .expect("enum serializes as string")
        .to_owned()
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/python_nav_alpha/wedge_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
