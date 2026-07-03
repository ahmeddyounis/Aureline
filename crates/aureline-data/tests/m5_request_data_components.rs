//! Fixture replay for M5 request/data component primitives.

use std::path::{Path, PathBuf};

use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture(file_name: &str) -> Value {
    load_repo_json(&format!(
        "fixtures/ui/m5-request-data-components/{file_name}"
    ))
}

fn load_schema(file_name: &str) -> Value {
    load_repo_json(&format!("schemas/ui/{file_name}"))
}

fn load_repo_json(repo_relative_path: &str) -> Value {
    let path = repo_root().join(repo_relative_path);
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

fn object_field<'a>(value: &'a Value, field: &str) -> &'a serde_json::Map<String, Value> {
    value
        .get(field)
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("missing object field {field}"))
}

fn contains_string(value: &Value, field: &str, expected: &str) -> bool {
    array_field(value, field)
        .iter()
        .any(|entry| entry.as_str() == Some(expected))
}

fn enum_contains(schema: &Value, pointer: &str, expected: &str) -> bool {
    schema
        .pointer(pointer)
        .and_then(Value::as_array)
        .is_some_and(|values| values.iter().any(|value| value.as_str() == Some(expected)))
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

#[test]
fn connection_picker_row_exposes_location_access_policy_and_current_scope() {
    let row = load_fixture("connection_picker_row.json");

    assert_eq!(str_field(&row, "record_kind"), "m5_connection_picker_row");
    assert_eq!(
        str_field(&row, "service_identity_ref"),
        "service:snowflake:prod-analytics"
    );
    assert_eq!(str_field(&row, "engine_family"), "snowflake");
    assert_eq!(str_field(&row, "execution_origin"), "managed_workspace");
    assert_eq!(str_field(&row, "target_location_class"), "managed");
    assert_eq!(str_field(&row, "access_mode"), "read_only");
    assert_eq!(str_field(&row, "capability_state"), "read_only");
    assert_eq!(
        str_field(&row, "current_database_ref"),
        "database:snowflake:prod_analytics"
    );
    assert_eq!(
        str_field(&row, "current_schema_ref"),
        "schema:snowflake:prod_analytics:analytics"
    );
    assert_eq!(str_field(&row, "online_state"), "online");
    assert_eq!(str_field(&row, "policy_state"), "read_only_enforced");
    assert_eq!(str_field(&row, "auth_storage_mode"), "policy_injected");
    assert!(bool_field(&row, "permission_limited"));

    let export = object_field(&row, "copy_export");
    for field in [
        "service_identity_ref",
        "target_location_class",
        "access_mode",
        "current_database_ref",
        "current_schema_ref",
        "online_state",
        "policy_state",
    ] {
        assert!(
            contains_string(&Value::Object(export.clone()), "export_fields", field),
            "connection export must preserve {field}"
        );
    }
}

#[test]
fn connection_schema_covers_local_tunnel_container_remote_and_managed_targets() {
    let schema = load_schema("m5-connection-picker-row.schema.json");

    for location in ["local", "tunneled", "container_local", "remote", "managed"] {
        assert!(
            enum_contains(
                &schema,
                "/$defs/target_location_class/enum",
                location
            ),
            "target_location_class must include {location}"
        );
    }
    for mode in ["read_only", "write_capable", "policy_blocked"] {
        assert!(
            enum_contains(&schema, "/$defs/access_mode/enum", mode),
            "access_mode must include {mode}"
        );
    }
    for state in ["online", "offline", "policy_blocked"] {
        assert!(
            enum_contains(&schema, "/$defs/online_state/enum", state),
            "online_state must include {state}"
        );
    }
}

#[test]
fn schema_object_rows_preserve_fresh_stale_permission_limited_and_offline_truth() {
    let fixture = load_fixture("schema_object_rows.json");
    let rows = array_field(&fixture, "rows");
    assert!(rows.len() >= 5);

    for state in ["fresh", "cached", "stale", "permission_limited", "offline"] {
        assert!(
            rows.iter()
                .any(|row| row.get("freshness_state").and_then(Value::as_str) == Some(state)),
            "schema object rows must include {state}"
        );
    }

    for row in rows {
        assert_eq!(str_field(row, "record_kind"), "m5_schema_object_row");
        assert!(row.get("object_type").is_some());
        assert!(row.get("object_name_ref").is_some());
        assert!(
            row.get("object_path_refs")
                .and_then(Value::as_array)
                .is_some_and(|path| !path.is_empty()),
            "object path must be present"
        );

        let permissions = row
            .get("permission_summary")
            .expect("permission summary is present");
        assert!(permissions.get("permission_state").is_some());
        assert!(permissions.get("read_allowed").is_some());
        assert!(permissions.get("write_allowed").is_some());
        assert!(permissions.get("permission_limited").is_some());

        let actions = row.get("actions").expect("actions are present");
        assert!(actions.get("open_enabled").is_some());
        assert!(actions.get("query_enabled").is_some());
        assert!(actions.get("copy_identifier_enabled").is_some());
        for action in ["open", "query", "copy_identifier"] {
            assert!(
                contains_string(actions, "action_labels", action),
                "schema object row must expose {action}"
            );
        }

        if str_field(row, "freshness_state") == "offline" {
            assert_eq!(str_field(row, "online_state"), "offline");
            assert!(!bool_field(actions, "query_enabled"));
            assert!(actions.get("disabled_reason_ref").is_some());
        }
    }
}

#[test]
fn sql_run_bar_preserves_selected_connection_transaction_and_actions() {
    let bar = load_fixture("sql_run_bar.json");

    assert_eq!(str_field(&bar, "record_kind"), "m5_sql_run_bar");
    assert_eq!(str_field(&bar, "access_mode"), "read_only");
    assert_eq!(str_field(&bar, "write_risk_state"), "read_only_no_write_risk");
    assert_eq!(str_field(&bar, "autocommit_state"), "not_executable");
    assert_eq!(str_field(&bar, "transaction_state"), "explain_only");
    assert_eq!(
        bar.get("selected_statement_count").and_then(Value::as_i64),
        Some(1)
    );

    let selected_connection = bar
        .get("selected_connection")
        .expect("selected connection is present");
    assert_eq!(
        str_field(selected_connection, "connection_identity_ref"),
        "connection:managed:snowflake:prod-analytics"
    );
    assert_eq!(
        str_field(selected_connection, "service_identity_ref"),
        "service:snowflake:prod-analytics"
    );
    assert_eq!(
        str_field(selected_connection, "target_location_class"),
        "managed"
    );
    assert_eq!(
        str_field(selected_connection, "policy_state"),
        "read_only_enforced"
    );

    let safety = bar
        .get("statement_safety_summary")
        .expect("statement safety summary is present");
    assert_eq!(str_field(safety, "safety_class"), "read_only_query");
    assert!(!bool_field(safety, "mutation_review_required"));
    assert!(!bool_field(safety, "protected_target_step_up_required"));

    let actions = bar.get("actions").expect("actions are present");
    assert!(bool_field(actions, "run_enabled"));
    assert!(!bool_field(actions, "cancel_enabled"));
    assert!(bool_field(actions, "explain_enabled"));
    for action in ["run", "cancel", "explain"] {
        assert!(
            contains_string(actions, "action_labels", action),
            "SQL run bar must expose {action}"
        );
    }
}
