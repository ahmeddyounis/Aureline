use std::path::Path;

use aureline_language::{
    LanguageServerHostIdentity, LanguageServerHostStatus, RouterCapabilityClass,
    RouterCompletenessClass, RouterFallbackClass, RouterFaultDomainId, RouterFreshnessClass,
    RouterHealthState, RouterLocalityClass, RouterScopeClaimClass, ScopeLimitClass,
    TsJsHoverRecord, TsJsLaunchWedge, TsJsLaunchWedgeSnapshot, TsJsReferenceSetRecord,
    TsJsRenamePreviewRecord, TsJsSemanticResultRecord,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    snapshot: TsJsLaunchWedgeSnapshot,
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
}

#[test]
fn tsjs_wedge_emits_hover_definition_references_and_rename_preview() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "tsjs_nav_alpha_cases");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(
        fixture.snapshot.record_kind,
        TsJsLaunchWedgeSnapshot::RECORD_KIND
    );
    assert_eq!(fixture.snapshot.schema_version, 1);

    let wedge = TsJsLaunchWedge::new(fixture.snapshot.clone());
    for case in &fixture.cases {
        let host_status = host_status(&fixture.snapshot, case);
        let host_statuses = [host_status];

        let hover = wedge
            .hover("symbol:tsjs:formatPrice", &host_statuses)
            .unwrap_or_else(|err| panic!("hover should build for {}: {err}", case.case_id));
        assert_eq!(
            enum_token(hover.answer_layer_class),
            case.expect.hover_layer,
            "hover layer mismatch for {}",
            case.case_id
        );
        assert_hover_round_trips(&hover);

        let definition = wedge
            .definition("symbol:tsjs:formatPrice", &host_statuses)
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

        let references = wedge
            .references("symbol:tsjs:formatPrice", &host_statuses)
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

        let rename_preview = wedge
            .rename_preview(
                "symbol:tsjs:formatPrice",
                "symbol:name:formatCurrency",
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
        assert_rename_preview_round_trips(&rename_preview);
    }
}

fn host_status(snapshot: &TsJsLaunchWedgeSnapshot, case: &Case) -> LanguageServerHostStatus {
    LanguageServerHostStatus {
        identity: LanguageServerHostIdentity {
            host_instance_id: format!("host:lsp:{}", case.case_id),
            provider_id: case.provider_id.clone(),
            workspace_id: snapshot.workspace_context.workspace_id.clone(),
            root_ref: snapshot.workspace_context.subject_root_ref.clone(),
            language_id: snapshot.language_id.clone(),
            server_label: case.server_label.clone(),
            execution_context_id: snapshot.workspace_context.execution_context_id.clone(),
            locality_class: RouterLocalityClass::LocalSidecar,
            fault_domain_id: RouterFaultDomainId::SessionScopedExecutionHosts,
            restart_budget_ref: "restart_budget:session_scoped_execution_hosts:language:ts:01"
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

fn assert_hover_round_trips(record: &TsJsHoverRecord) {
    let serialized = serde_json::to_string(record).expect("hover serializes");
    let round_trip: TsJsHoverRecord =
        serde_json::from_str(&serialized).expect("hover deserializes");
    assert_eq!(round_trip, *record);
}

fn assert_semantic_result_round_trips(record: &TsJsSemanticResultRecord) {
    let serialized = serde_json::to_string(record).expect("semantic result serializes");
    let round_trip: TsJsSemanticResultRecord =
        serde_json::from_str(&serialized).expect("semantic result deserializes");
    assert_eq!(round_trip, *record);
}

fn assert_reference_set_round_trips(record: &TsJsReferenceSetRecord) {
    let serialized = serde_json::to_string(record).expect("reference set serializes");
    let round_trip: TsJsReferenceSetRecord =
        serde_json::from_str(&serialized).expect("reference set deserializes");
    assert_eq!(round_trip, *record);
}

fn assert_rename_preview_round_trips(record: &TsJsRenamePreviewRecord) {
    let serialized = serde_json::to_string(record).expect("rename preview serializes");
    let round_trip: TsJsRenamePreviewRecord =
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
        .join("../../fixtures/language/tsjs_nav_alpha/wedge_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
