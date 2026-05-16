//! Integration coverage for the request-workspace alpha contract.
//!
//! The test replays the checked-in fixtures under
//! [`fixtures/runtime/request_workspace_alpha/`] end-to-end. Each fixture pins
//! one seeded scenario (the same scenarios the headless CLI / inspector
//! binary `aureline_shell_request_workspace` emits) and the expected truth
//! the runtime record, the canonical send-inspector report, the shell panel
//! projection, and the headless CLI all surface.
//!
//! The test enforces the three acceptance bullets from the request-workspace
//! alpha spec:
//!
//! 1. Requests, environments, assertions, and artifacts are represented by
//!    one schema shared by UI and CLI/headless lanes. The canonical
//!    [`RequestWorkspaceAlphaRecord`] bundles all four; the shell panel
//!    projection and the integration test consume the same record verbatim.
//! 2. A send inspector can explain target, credential class, execution
//!    context, and expected side effects before execution. The fixture's
//!    `expect` block pins those fields and the test asserts the canonical
//!    [`SendInspectorReport`] surfaces them character-identically.
//! 3. Support and artifact exports can reopen or compare request-workspace
//!    runs truthfully. The [`RequestWorkspaceSupportExport`] wrapper
//!    round-trips through serde, carries every record verbatim, and carries
//!    one [`SendInspectorReport`] per record so reviewer / support flows do
//!    not re-derive readiness locally.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    seeded_request_workspace_record, seeded_request_workspace_support_export,
    CredentialClass, AuthStrategyKind, RequestWorkspaceAlphaRecord, RequestWorkspaceSeededScenario,
    RequestWorkspaceSupportExport, SendInspectorReadiness, TargetClass,
    REQUEST_WORKSPACE_ALPHA_RECORD_KIND, REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
    REQUEST_WORKSPACE_SEND_INSPECTOR_RECORD_KIND, REQUEST_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("request_workspace_alpha")
}

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    scenario: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    target_class: TargetClass,
    method_token: String,
    credential_class: CredentialClass,
    auth_strategy: AuthStrategyKind,
    boundary_cue_visible: bool,
    readiness: SendInspectorReadiness,
    requires_review_before_dispatch: bool,
    blocks_dispatch: bool,
    expected_side_effect_tokens: Vec<String>,
    expected_banner_kinds: Vec<String>,
    schema_freshness_token: String,
    any_secret_handle: bool,
}

fn scenario_for(name: &str) -> RequestWorkspaceSeededScenario {
    match name {
        "local_read_only_get" => RequestWorkspaceSeededScenario::LocalReadOnlyGet,
        "remote_mutating_post_stale_schema" => {
            RequestWorkspaceSeededScenario::RemoteMutatingPostStaleSchema
        }
        "managed_delete_missing_schema" => {
            RequestWorkspaceSeededScenario::ManagedDeleteMissingSchema
        }
        "remote_graphql_no_auth" => RequestWorkspaceSeededScenario::RemoteGraphqlNoAuth,
        other => panic!("unknown request-workspace scenario: {other}"),
    }
}

#[test]
fn every_seeded_scenario_fixture_replays_through_canonical_send_inspector() {
    for fixture_name in [
        "local_read_only_get.json",
        "remote_mutating_post_stale_schema.json",
        "managed_delete_missing_schema.json",
        "remote_graphql_no_auth.json",
    ] {
        let path = fixture_root().join(fixture_name);
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {fixture_name}: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse fixture {fixture_name}: {err}"));
        assert_eq!(fixture.record_kind, "request_workspace_alpha_case");
        assert_eq!(fixture.schema_version, REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION);

        let scenario = scenario_for(&fixture.scenario);
        let record = seeded_request_workspace_record(scenario);
        assert_eq!(record.record_kind, REQUEST_WORKSPACE_ALPHA_RECORD_KIND);
        assert!(
            record.validation_issues().is_empty(),
            "{fixture_name}: record must not overclaim truth: {:?}",
            record.validation_issues()
        );

        assert_eq!(
            record.target_class, fixture.expect.target_class,
            "{fixture_name}: target_class"
        );
        assert_eq!(
            record.boundary_cue_visible, fixture.expect.boundary_cue_visible,
            "{fixture_name}: boundary_cue_visible"
        );
        assert_eq!(
            record.environment.has_secret_handle(),
            fixture.expect.any_secret_handle,
            "{fixture_name}: any_secret_handle"
        );
        assert_eq!(
            record.schema_snapshot.freshness_token, fixture.expect.schema_freshness_token,
            "{fixture_name}: schema_freshness_token"
        );

        let report = record.send_inspector_report();
        assert_eq!(report.record_kind, REQUEST_WORKSPACE_SEND_INSPECTOR_RECORD_KIND);
        assert_eq!(report.method_token, fixture.expect.method_token, "{fixture_name}: method");
        assert_eq!(
            report.credential_class, fixture.expect.credential_class,
            "{fixture_name}: credential_class"
        );
        assert_eq!(
            report.auth_strategy, fixture.expect.auth_strategy,
            "{fixture_name}: auth_strategy"
        );
        assert_eq!(report.readiness, fixture.expect.readiness, "{fixture_name}: readiness");
        assert_eq!(
            report.requires_review_before_dispatch,
            fixture.expect.requires_review_before_dispatch,
            "{fixture_name}: requires_review_before_dispatch"
        );
        assert_eq!(
            report.blocks_dispatch, fixture.expect.blocks_dispatch,
            "{fixture_name}: blocks_dispatch"
        );

        let actual_side_effect_tokens: Vec<String> = report
            .expected_side_effects
            .iter()
            .map(|row| row.class_token.clone())
            .collect();
        assert_eq!(
            actual_side_effect_tokens, fixture.expect.expected_side_effect_tokens,
            "{fixture_name}: expected_side_effect_tokens"
        );

        let actual_banner_kinds: Vec<String> = report
            .banners
            .iter()
            .map(|banner| banner.banner_kind.clone())
            .collect();
        assert_eq!(
            actual_banner_kinds, fixture.expect.expected_banner_kinds,
            "{fixture_name}: expected_banner_kinds"
        );
    }
}

#[test]
fn seeded_records_are_deterministic_across_calls() {
    // Reviewer and partner runs of the headless inspector MUST reproduce
    // the fixture-pinned record byte-for-byte. The test serialises the
    // record twice and asserts character-identical output.
    for scenario in RequestWorkspaceSeededScenario::ALL {
        let first =
            serde_json::to_string(&seeded_request_workspace_record(scenario)).expect("serialize first");
        let second =
            serde_json::to_string(&seeded_request_workspace_record(scenario)).expect("serialize second");
        assert_eq!(
            first,
            second,
            "{}: seeded record must be deterministic",
            scenario.as_str()
        );
    }
}

#[test]
fn support_export_round_trips_and_bundles_every_scenario() {
    let export = seeded_request_workspace_support_export(
        "request-workspace-alpha:test",
        "2026-05-15T00:00:00Z",
    );
    assert_eq!(export.record_kind, REQUEST_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND);
    assert_eq!(export.records.len(), RequestWorkspaceSeededScenario::ALL.len());
    assert_eq!(
        export.send_inspector_reports.len(),
        RequestWorkspaceSeededScenario::ALL.len()
    );
    assert!(
        export.any_requires_review,
        "support export must surface at least one review-required scenario"
    );
    assert!(
        export.any_blocks_dispatch,
        "support export must surface at least one blocked scenario"
    );

    let json = serde_json::to_string(&export).expect("serialize support export");
    let round: RequestWorkspaceSupportExport =
        serde_json::from_str(&json).expect("deserialize support export");
    assert_eq!(round, export);

    // Redaction guarantee: the seeded scenarios never embed raw credential
    // bodies, raw command lines, or raw secret material. Any appearance
    // here would be a regression that would cause the support packet to
    // leak.
    assert!(!json.contains("AWS_SECRET_ACCESS_KEY"));
    assert!(!json.contains("Bearer "));
    assert!(!json.contains("password"));
}

#[test]
fn support_export_send_inspector_reports_match_canonical_records() {
    // Reviewer / support consumers receive one report per record in
    // canonical order. The integration test asserts the bundle does not
    // diverge from re-deriving the report locally; if it does, the
    // wrapper has invented or dropped truth.
    let export = seeded_request_workspace_support_export(
        "request-workspace-alpha:parity",
        "2026-05-15T00:00:00Z",
    );
    for (record, report) in export
        .records
        .iter()
        .zip(export.send_inspector_reports.iter())
    {
        let local_report = record.send_inspector_report();
        assert_eq!(report, &local_report);
        assert_eq!(report.request_workspace_ref, record.request_workspace_ref);
    }
}

#[test]
fn support_export_plaintext_lists_every_seeded_scenario() {
    let export = seeded_request_workspace_support_export(
        "request-workspace-alpha:plaintext",
        "2026-05-15T00:00:00Z",
    );
    let text = export.render_plaintext();
    assert!(text.contains("request-workspace support export:"));
    for record in &export.records {
        assert!(
            text.contains(&record.request_workspace_ref),
            "plaintext must list request workspace ref {}",
            record.request_workspace_ref
        );
    }
}

#[test]
fn seeded_records_emit_canonical_lane_and_record_kind() {
    for scenario in RequestWorkspaceSeededScenario::ALL {
        let record: RequestWorkspaceAlphaRecord = seeded_request_workspace_record(scenario);
        assert_eq!(record.lane_id, aureline_runtime::REQUEST_WORKSPACE_ALPHA_LANE_ID);
        assert_eq!(record.schema_version, REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION);
        assert_eq!(record.record_kind, REQUEST_WORKSPACE_ALPHA_RECORD_KIND);
    }
}
