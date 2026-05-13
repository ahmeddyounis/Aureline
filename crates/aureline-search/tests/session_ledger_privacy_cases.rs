//! Fixture-driven coverage for saved-query privacy and search export packets.
//!
//! Each fixture under `fixtures/search/saved_query_privacy/` starts from a
//! planner-backed query session, writes the session ledger, saves the query,
//! optionally mints a support/docs/local packet, and checks that raw query
//! text, scope labels, hidden counts, and reopen behavior stay honest.

use std::path::Path;

use serde::Deserialize;

use aureline_search::{
    QuerySessionLedgerRecord, SavedQueryPrivacyClass, SavedQueryRecord, SavedQueryRecordInputs,
    SavedQueryReopenContext, SavedQuerySharePolicy, SavedQuerySourceClass, SearchExportDestination,
    SearchExportPacket, SearchExportPacketInputs, SearchPacketCountSummary, SearchPlannerAlpha,
    SearchPlannerInputs, SearchScopeCountsRecord,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: CaseInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct CaseInput {
    ledger_id: String,
    saved_query_id: String,
    source_class: SavedQuerySourceClass,
    privacy_class: SavedQueryPrivacyClass,
    share_policy: SavedQuerySharePolicy,
    #[serde(default)]
    policy_epoch: Option<String>,
    created_at: String,
    planner: SearchPlannerInputs,
    #[serde(default)]
    export: Option<ExportInput>,
    #[serde(default)]
    reopen_context: Option<SavedQueryReopenContext>,
}

#[derive(Debug, Deserialize)]
struct ExportInput {
    packet_id: String,
    destination: SearchExportDestination,
    #[serde(default)]
    selected_result_ids: Vec<String>,
    #[serde(default)]
    scope_counts: Option<SearchScopeCountsRecord>,
    exported_at: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    ledger_query_text_mode: String,
    ledger_has_raw_query_text: bool,
    saved_query_text_mode: String,
    saved_has_raw_query_text: bool,
    saved_has_query_hash: bool,
    saved_query_material_disposition: String,
    saved_privacy_findings: Vec<String>,
    saved_scope_label: String,
    export_destination: String,
    export_privacy_class: String,
    export_redaction_state: String,
    export_has_raw_query_text: bool,
    export_query_text_mode: String,
    export_scope_label: String,
    export_readiness_state: String,
    export_selected_result_refs: Vec<String>,
    export_included_result_refs: Vec<String>,
    export_result_source_labels: Vec<String>,
    export_count_summary: SearchPacketCountSummary,
    export_findings: Vec<String>,
    reopen_scope_honesty_state: String,
    reopen_effective_scope_class: String,
    reopen_effective_stable_scope_id: String,
    reopen_rerun_allowed: bool,
    reopen_denial_reason: Option<String>,
    reopen_readiness_changed: bool,
    reopen_index_epoch_changed: bool,
}

#[test]
fn saved_query_privacy_cases_match_expected_projection() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/saved_query_privacy");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "at least one saved_query_privacy fixture must exist"
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(
            fixture.record_kind, "saved_query_privacy_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let output = SearchPlannerAlpha::plan(fixture.input.planner.clone());
        let mut ledger = QuerySessionLedgerRecord::from_planner_output(
            fixture.input.ledger_id.clone(),
            &output,
            fixture.input.privacy_class,
            fixture.input.created_at.clone(),
        );
        let saved_query = SavedQueryRecord::from_session(SavedQueryRecordInputs {
            saved_query_id: fixture.input.saved_query_id.clone(),
            source_class: fixture.input.source_class,
            privacy_class: fixture.input.privacy_class,
            share_policy: fixture.input.share_policy,
            query_session: output.query_session.clone(),
            policy_epoch: fixture.input.policy_epoch.clone(),
            created_at: fixture.input.created_at.clone(),
        });

        assert!(
            ledger.link_saved_query(
                &output.query_session.query_session_id,
                saved_query.saved_query_id.clone(),
                fixture.input.created_at.clone()
            ),
            "ledger must link saved query in {path:?}"
        );

        let entry = ledger
            .entry(&output.query_session.query_session_id)
            .unwrap_or_else(|| panic!("ledger missing entry in {path:?}"));
        assert_eq!(
            entry.query_session.query_text_mode.as_str(),
            fixture.expect.ledger_query_text_mode,
            "ledger query text mode mismatch in {path:?}"
        );
        assert_eq!(
            entry.query_session.query_text.is_some(),
            fixture.expect.ledger_has_raw_query_text,
            "ledger raw query text presence mismatch in {path:?}"
        );

        assert_eq!(
            saved_query.query_text_mode.as_str(),
            fixture.expect.saved_query_text_mode,
            "saved query text mode mismatch in {path:?}"
        );
        assert_eq!(
            saved_query.contains_raw_query_text(),
            fixture.expect.saved_has_raw_query_text,
            "saved raw query text presence mismatch in {path:?}"
        );
        assert_eq!(
            saved_query.query_hash.is_some(),
            fixture.expect.saved_has_query_hash,
            "saved query hash presence mismatch in {path:?}"
        );
        assert_eq!(
            saved_query.query_material_disposition.as_str(),
            fixture.expect.saved_query_material_disposition,
            "saved query material disposition mismatch in {path:?}"
        );
        assert_eq!(
            saved_query.scope_label, fixture.expect.saved_scope_label,
            "saved scope label mismatch in {path:?}"
        );
        assert_eq!(
            finding_tokens(&saved_query.validate_privacy()),
            fixture.expect.saved_privacy_findings,
            "saved privacy findings mismatch in {path:?}"
        );

        let export_input = fixture
            .input
            .export
            .as_ref()
            .unwrap_or_else(|| panic!("fixture {path:?} must include an export block"));
        let packet = SearchExportPacket::from_planned_result_set(SearchExportPacketInputs {
            packet_id: export_input.packet_id.clone(),
            destination: export_input.destination,
            privacy_class: fixture.input.privacy_class,
            query_session: output.query_session.clone(),
            result_set: output.result_set.clone(),
            selected_result_ids: export_input.selected_result_ids.clone(),
            scope_counts: export_input.scope_counts.clone(),
            exported_at: export_input.exported_at.clone(),
        })
        .unwrap_or_else(|err| panic!("fixture {path:?} export packet must build: {err}"));

        assert!(
            ledger.link_export_packet(
                &output.query_session.query_session_id,
                packet.packet_id.clone(),
                export_input.exported_at.clone()
            ),
            "ledger must link export packet in {path:?}"
        );
        assert_eq!(
            packet.destination.as_str(),
            fixture.expect.export_destination,
            "export destination mismatch in {path:?}"
        );
        assert_eq!(
            packet.privacy_class.as_str(),
            fixture.expect.export_privacy_class,
            "export privacy class mismatch in {path:?}"
        );
        assert_eq!(
            packet.redaction_state.as_str(),
            fixture.expect.export_redaction_state,
            "export redaction state mismatch in {path:?}"
        );
        assert_eq!(
            packet.query_text.is_some(),
            fixture.expect.export_has_raw_query_text,
            "export raw query text presence mismatch in {path:?}"
        );
        assert_eq!(
            packet.query_text_mode.as_str(),
            fixture.expect.export_query_text_mode,
            "export query text mode mismatch in {path:?}"
        );
        assert_eq!(
            packet.scope_label, fixture.expect.export_scope_label,
            "export scope label mismatch in {path:?}"
        );
        assert_eq!(
            packet.readiness_state, fixture.expect.export_readiness_state,
            "export readiness state mismatch in {path:?}"
        );
        assert_eq!(
            packet.selected_result_refs, fixture.expect.export_selected_result_refs,
            "selected result refs mismatch in {path:?}"
        );
        assert_eq!(
            packet.included_result_refs, fixture.expect.export_included_result_refs,
            "included result refs mismatch in {path:?}"
        );
        assert_eq!(
            packet.result_source_labels, fixture.expect.export_result_source_labels,
            "source labels mismatch in {path:?}"
        );
        assert_eq!(
            packet.count_summary, fixture.expect.export_count_summary,
            "export count summary mismatch in {path:?}"
        );
        assert_eq!(
            finding_tokens(&packet.validate_export_safe()),
            fixture.expect.export_findings,
            "export safety findings mismatch in {path:?}"
        );

        let reopen_context = fixture
            .input
            .reopen_context
            .clone()
            .unwrap_or_else(|| panic!("fixture {path:?} must include a reopen context"));
        let reopen = saved_query.reopen_against(reopen_context);
        assert_eq!(
            reopen.scope_honesty_state.as_str(),
            fixture.expect.reopen_scope_honesty_state,
            "reopen scope honesty state mismatch in {path:?}"
        );
        assert_eq!(
            reopen.effective_scope_class.as_str(),
            fixture.expect.reopen_effective_scope_class,
            "reopen effective scope class mismatch in {path:?}"
        );
        assert_eq!(
            reopen.effective_stable_scope_id, fixture.expect.reopen_effective_stable_scope_id,
            "reopen effective scope id mismatch in {path:?}"
        );
        assert_eq!(
            reopen.rerun_allowed, fixture.expect.reopen_rerun_allowed,
            "reopen rerun_allowed mismatch in {path:?}"
        );
        assert_eq!(
            reopen
                .denial_reason
                .map(|reason| reason.as_str().to_string()),
            fixture.expect.reopen_denial_reason,
            "reopen denial reason mismatch in {path:?}"
        );
        assert_eq!(
            reopen.readiness_changed, fixture.expect.reopen_readiness_changed,
            "reopen readiness changed mismatch in {path:?}"
        );
        assert_eq!(
            reopen.index_epoch_changed, fixture.expect.reopen_index_epoch_changed,
            "reopen index epoch changed mismatch in {path:?}"
        );
    }
}

fn finding_tokens(findings: &[aureline_search::SavedQueryValidationFinding]) -> Vec<String> {
    findings
        .iter()
        .map(|finding| finding.finding_kind.as_str().to_string())
        .collect()
}
