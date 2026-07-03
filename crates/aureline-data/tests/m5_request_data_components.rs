//! Fixture replay for M5 request/data component primitives.

use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture(file_name: &str) -> Value {
    let path = repo_root()
        .join("fixtures/ui/m5-request-data-components")
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn str_field<'a>(value: &'a Value, field: &str) -> &'a str {
    value
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("missing string field {field}"))
}

fn bool_field(value: &Value, field: &str) -> bool {
    value
        .get(field)
        .and_then(Value::as_bool)
        .unwrap_or_else(|| panic!("missing boolean field {field}"))
}

fn array_field<'a>(value: &'a Value, field: &str) -> &'a Vec<Value> {
    value
        .get(field)
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("missing array field {field}"))
}

fn contains_string(value: &Value, field: &str, expected: &str) -> bool {
    array_field(value, field)
        .iter()
        .any(|entry| entry.as_str() == Some(expected))
}

#[test]
fn request_header_exposes_origin_auth_run_control_and_last_run_summary() {
    let header = load_fixture("request_editor_header.json");

    assert_eq!(
        str_field(&header, "record_kind"),
        "m5_request_editor_header"
    );
    assert_eq!(str_field(&header, "execution_origin"), "browser_runtime");
    assert_eq!(
        str_field(&header, "capability_state"),
        "mutation_review_required"
    );
    assert_eq!(
        str_field(&header, "auth_sheet_ref"),
        "auth-sheet:project-api-device-code"
    );
    assert!(contains_string(
        &header,
        "contract_source_badges",
        "contract-badge:project-api-openapi:get-users"
    ));

    let environment_picker = header
        .get("environment_picker")
        .expect("environment picker is present");
    assert_eq!(
        str_field(environment_picker, "selected_layer"),
        "managed_policy"
    );
    assert_eq!(
        str_field(environment_picker, "override_scope"),
        "managed_policy_only"
    );
    assert_eq!(
        str_field(environment_picker, "export_scope"),
        "metadata_only"
    );
    assert!(
        environment_picker
            .get("source_layers")
            .and_then(Value::as_array)
            .is_some_and(|layers| layers.len() >= 3),
        "environment picker must expose source layers"
    );

    let auth = header.get("auth").expect("auth summary is present");
    assert_eq!(str_field(auth, "scheme"), "oauth2_device_code");
    assert_eq!(
        str_field(auth, "secret_source_class"),
        "browser_device_code"
    );
    assert_eq!(str_field(auth, "token_lifetime"), "refreshable");
    assert_eq!(
        str_field(auth, "handoff_state"),
        "awaiting_user_authorization"
    );
    assert!(!bool_field(auth, "raw_secret_exposed"));

    let run_control = header.get("run_control").expect("run control is present");
    assert_eq!(str_field(run_control, "state"), "blocked");
    assert!(!bool_field(run_control, "run_enabled"));
    assert!(!bool_field(run_control, "cancel_enabled"));

    let last_run_summary = header
        .get("last_run_summary")
        .expect("last run summary is present");
    assert_eq!(str_field(last_run_summary, "status"), "stale");
    assert!(last_run_summary.get("duration_ms").is_some());
    assert!(last_run_summary.get("result_summary_ref").is_some());
}

#[test]
fn response_tabset_keeps_evidence_tabs_and_actions_distinct() {
    let tabset = load_fixture("response_tabset.json");

    assert_eq!(str_field(&tabset, "record_kind"), "m5_response_tabset");
    for tab in [
        "summary",
        "body",
        "headers_cookies",
        "assertions",
        "timeline",
    ] {
        assert!(
            contains_string(&tabset, "tabs", tab),
            "response tabset must include {tab}"
        );
    }

    let headers = tabset
        .get("headers_cookies_summary")
        .expect("headers/cookies summary is present");
    assert!(bool_field(headers, "cookie_values_redacted"));

    let assertions = tabset
        .get("assertions_summary")
        .expect("assertions summary is present");
    assert_eq!(assertions.get("fail").and_then(Value::as_i64), Some(1));

    let actions = tabset.get("actions").expect("actions are present");
    assert!(contains_string(
        actions,
        "export_actions",
        "export_assertions"
    ));
    assert!(contains_string(
        actions,
        "export_actions",
        "export_timeline_transport"
    ));
    assert!(contains_string(
        actions,
        "compare_actions",
        "compare_assertions"
    ));
    assert!(contains_string(
        actions,
        "compare_actions",
        "compare_timeline_transport"
    ));
    assert!(bool_field(actions, "action_label_parity"));
}

#[test]
fn request_history_row_preserves_debug_metadata_without_default_payload_retention() {
    let row = load_fixture("request_history_row.json");

    assert_eq!(str_field(&row, "record_kind"), "m5_request_history_row");
    assert_eq!(str_field(&row, "execution_origin"), "browser_runtime");
    assert_eq!(str_field(&row, "origin_scope"), "browser_runtime");
    assert_eq!(str_field(&row, "status_result_class"), "assertion_failed");
    assert_eq!(str_field(&row, "executed_at"), "2026-07-03T11:50:00Z");

    let environment = row.get("environment").expect("environment is present");
    assert_eq!(
        str_field(environment, "environment_ref"),
        "env:managed-project-api:production"
    );

    let assertions = row.get("assertions").expect("assertions are present");
    assert_eq!(str_field(assertions, "assertion_state"), "failed");
    assert_eq!(assertions.get("pass").and_then(Value::as_i64), Some(3));
    assert_eq!(assertions.get("fail").and_then(Value::as_i64), Some(1));

    let redaction = row
        .get("redaction_retention")
        .expect("redaction/retention is present");
    assert_eq!(str_field(redaction, "retention_mode"), "metadata_only");
    assert!(!bool_field(redaction, "raw_secret_retained"));
    assert!(!bool_field(redaction, "unsafe_payload_retained_by_default"));
    assert!(!bool_field(redaction, "cookie_values_retained"));

    let replay = row.get("replay").expect("replay is present");
    assert_eq!(str_field(replay, "replay_mode"), "current_context_only");
    assert!(!bool_field(replay, "exact_rerun_available"));
    assert!(bool_field(replay, "current_context_replay_available"));

    let actions = row.get("actions").expect("actions are present");
    assert!(contains_string(
        actions,
        "compare_actions",
        "compare_body_metadata"
    ));
    assert!(contains_string(
        actions,
        "export_actions",
        "export_support_packet"
    ));
    assert!(contains_string(
        &row,
        "contract_source_badges",
        "contract-badge:project-api-openapi:get-users"
    ));
}

#[test]
fn contract_source_badge_has_version_freshness_and_label_parity() {
    let badge = load_fixture("contract_source_badge.json");

    assert_eq!(str_field(&badge, "record_kind"), "m5_contract_source_badge");
    assert_eq!(str_field(&badge, "contract_kind"), "openapi");
    assert_eq!(str_field(&badge, "display_label"), "OpenAPI");
    assert_eq!(
        str_field(&badge, "version_or_snapshot_ref"),
        "snapshot:project-api-openapi:2026-07-03"
    );
    assert_eq!(str_field(&badge, "freshness_state"), "current");
    assert_eq!(str_field(&badge, "drift_state"), "current");
    assert!(bool_field(&badge, "generated_from_contract"));
    assert!(!bool_field(&badge, "raw_contract_payload_exposed"));

    for context in [
        "full_request_editor",
        "history_row",
        "handoff_surface",
        "compare_surface",
        "response_tabset",
        "cli_headless",
        "support_export",
    ] {
        assert!(
            contains_string(&badge, "surface_contexts", context),
            "badge must project to {context}"
        );
    }

    let actions = badge.get("actions").expect("badge actions are present");
    assert!(contains_string(actions, "badge_actions", "open_contract"));
    assert!(contains_string(actions, "badge_actions", "diff_snapshot"));
    assert!(bool_field(actions, "label_parity_across_surfaces"));
}

#[test]
fn variable_resolution_inspector_is_secret_safe_and_scope_explicit() {
    let inspector = load_fixture("variable_resolution_inspector.json");

    assert_eq!(
        str_field(&inspector, "record_kind"),
        "m5_variable_resolution_inspector"
    );
    assert_eq!(
        str_field(&inspector, "effective_source_layer"),
        "secret_broker"
    );
    assert_eq!(
        str_field(&inspector, "effective_value_state"),
        "secret_handle"
    );
    assert_eq!(str_field(&inspector, "override_scope"), "not_overridable");
    assert_eq!(str_field(&inspector, "export_scope"), "secret_handle_ref");
    assert!(!bool_field(&inspector, "raw_secret_exposed"));

    let preview = inspector
        .get("effective_redacted_preview")
        .expect("effective preview is present");
    assert_eq!(str_field(preview, "preview_state"), "withheld_secret");
    assert_eq!(str_field(preview, "display"), "secret handle only");

    let source_layers = inspector
        .get("source_layers")
        .and_then(Value::as_array)
        .expect("source layers are present");
    assert_eq!(source_layers.len(), 3);
    for layer in source_layers {
        assert!(layer.get("resolution_state").is_some());
        assert!(layer.get("redacted_preview").is_some());
        assert!(layer.get("override_scope").is_some());
        assert!(layer.get("export_scope").is_some());
    }
}

#[test]
fn auth_sheet_exposes_scheme_lifetime_handoff_and_policy_without_secrets() {
    let auth_sheet = load_fixture("auth_sheet.json");

    assert_eq!(str_field(&auth_sheet, "record_kind"), "m5_auth_sheet");
    assert_eq!(
        str_field(&auth_sheet, "execution_origin"),
        "browser_runtime"
    );
    assert_eq!(str_field(&auth_sheet, "scheme"), "oauth2_device_code");
    assert_eq!(
        str_field(&auth_sheet, "secret_source_class"),
        "browser_device_code"
    );
    assert_eq!(
        str_field(&auth_sheet, "auth_storage_mode"),
        "browser_device_code"
    );
    assert_eq!(str_field(&auth_sheet, "token_lifetime"), "refreshable");
    assert!(!bool_field(&auth_sheet, "raw_secret_exposed"));

    let expiry = auth_sheet.get("expiry").expect("expiry is present");
    assert_eq!(str_field(expiry, "freshness_state"), "current");

    let handoff = auth_sheet.get("handoff").expect("handoff is present");
    assert_eq!(str_field(handoff, "state"), "awaiting_user_authorization");
    assert!(!bool_field(handoff, "raw_verification_code_exposed"));

    let policy_notes = auth_sheet
        .get("policy_notes")
        .and_then(Value::as_array)
        .expect("policy notes are present");
    assert!(policy_notes.len() >= 2);
}
