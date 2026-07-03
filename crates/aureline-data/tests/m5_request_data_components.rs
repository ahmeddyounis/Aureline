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

fn i64_field(value: &Value, field: &str) -> i64 {
    value
        .get(field)
        .and_then(Value::as_i64)
        .unwrap_or_else(|| panic!("missing integer field {field}"))
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

fn assert_reduced_capability_disclosure(value: &Value, fixture_name: &str) {
    let banner = value
        .get("reduced_capability_banner")
        .unwrap_or_else(|| panic!("{fixture_name} missing reduced capability banner"));
    assert!(
        banner.get("banner_id").and_then(Value::as_str).is_some(),
        "{fixture_name} banner must carry a stable id"
    );
    assert!(
        banner
            .get("visible_label")
            .and_then(Value::as_str)
            .is_some(),
        "{fixture_name} banner must carry visible reduced-capability copy"
    );
    assert!(
        banner
            .get("missing_capabilities")
            .and_then(Value::as_array)
            .is_some_and(|items| !items.is_empty()),
        "{fixture_name} banner must disclose missing capabilities"
    );
    assert!(
        banner
            .get("preserved_fields")
            .and_then(Value::as_array)
            .is_some_and(|items| !items.is_empty()),
        "{fixture_name} banner must list fields preserved across consumers"
    );

    let action_policy = banner
        .get("action_policy")
        .unwrap_or_else(|| panic!("{fixture_name} missing reduced action policy"));
    for action in [
        "live_send_available",
        "replay_available",
        "mutate_available",
        "export_available",
    ] {
        assert!(
            action_policy.get(action).and_then(Value::as_bool).is_some(),
            "{fixture_name} action policy must disclose {action}"
        );
    }

    let notes = value
        .get("provider_handoff_notes")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("{fixture_name} missing provider handoff notes"));
    assert!(
        !notes.is_empty(),
        "{fixture_name} must carry at least one provider handoff note"
    );
    for note in notes {
        assert!(
            note.get("note_id").and_then(Value::as_str).is_some(),
            "{fixture_name} provider handoff note must carry a stable id"
        );
        assert!(
            note.get("provider_surface")
                .and_then(Value::as_str)
                .is_some(),
            "{fixture_name} provider handoff note must name the provider surface"
        );
        assert!(
            note.get("handoff_state").and_then(Value::as_str).is_some(),
            "{fixture_name} provider handoff note must name the handoff state"
        );
        assert!(
            note.get("truth_preserved")
                .and_then(Value::as_array)
                .is_some_and(|items| !items.is_empty()),
            "{fixture_name} provider handoff note must list preserved truth fields"
        );
        assert_eq!(
            note.get("raw_material_exported").and_then(Value::as_bool),
            Some(false),
            "{fixture_name} provider handoff note must stay raw-material safe"
        );
    }
}

fn assert_accessibility_contract(value: &Value, fixture_name: &str) {
    let contract = value
        .get("accessibility_contract")
        .unwrap_or_else(|| panic!("{fixture_name} missing accessibility contract"));

    let keyboard = contract
        .get("keyboard_traversal")
        .unwrap_or_else(|| panic!("{fixture_name} missing keyboard traversal contract"));
    for flag in [
        "roving_tabindex",
        "arrow_key_navigation",
        "home_end_navigation",
        "escape_returns_focus",
    ] {
        assert!(
            bool_field(keyboard, flag),
            "{fixture_name} keyboard traversal must enable {flag}"
        );
    }
    assert!(
        keyboard
            .get("tab_stop_order")
            .and_then(Value::as_array)
            .is_some_and(|items| items.len() >= 4),
        "{fixture_name} must declare a stable tab stop order"
    );

    let screen_reader = contract
        .get("screen_reader")
        .unwrap_or_else(|| panic!("{fixture_name} missing screen-reader contract"));
    assert!(
        screen_reader
            .get("label_ref")
            .and_then(Value::as_str)
            .is_some_and(|label| label.starts_with("a11y:")),
        "{fixture_name} must expose a stable screen-reader label ref"
    );
    assert!(
        screen_reader
            .get("described_by_refs")
            .and_then(Value::as_array)
            .is_some_and(|items| items.len() >= 2),
        "{fixture_name} must expose described-by refs for state and degraded reasons"
    );
    assert!(
        screen_reader
            .get("state_announced_fields")
            .and_then(Value::as_array)
            .is_some_and(|items| items.len() >= 4),
        "{fixture_name} must name fields announced to screen readers"
    );
    assert!(
        !bool_field(screen_reader, "raw_secret_announced"),
        "{fixture_name} must never announce raw secrets"
    );

    let fallback = contract
        .get("fallback")
        .unwrap_or_else(|| panic!("{fixture_name} missing accessible fallback contract"));
    assert!(bool_field(fallback, "accessible_text_summary_required"));
    assert!(bool_field(fallback, "accessible_table_fallback_required"));
    for format in ["text", "json", "markdown"] {
        assert!(
            contains_string(fallback, "fallback_export_formats", format),
            "{fixture_name} fallback must include {format}"
        );
    }
    assert!(bool_field(fallback, "screenshot_only_prohibited"));

    let zoom_density = contract
        .get("zoom_density")
        .unwrap_or_else(|| panic!("{fixture_name} missing zoom/density contract"));
    for flag in [
        "supports_200_percent_zoom",
        "supports_high_density",
        "stable_row_height_under_density",
        "no_horizontal_content_loss",
    ] {
        assert!(
            bool_field(zoom_density, flag),
            "{fixture_name} zoom/density contract must enable {flag}"
        );
    }
}

fn assert_support_export_join(value: &Value, fixture_name: &str) -> Vec<String> {
    let join = value
        .get("support_export_join")
        .unwrap_or_else(|| panic!("{fixture_name} missing support-export join"));
    assert!(
        str_field(join, "join_id").starts_with("support-join:"),
        "{fixture_name} join id must be stable"
    );
    assert!(
        str_field(join, "schema_ref").starts_with("schemas/ui/"),
        "{fixture_name} join must reference the machine-readable UI schema"
    );
    let object_kinds = array_field(join, "joined_object_kinds")
        .iter()
        .map(|kind| {
            kind.as_str()
                .unwrap_or_else(|| panic!("{fixture_name} joined object kind must be a string"))
                .to_owned()
        })
        .collect::<Vec<_>>();
    assert!(
        !object_kinds.is_empty(),
        "{fixture_name} join must declare at least one object kind"
    );
    assert!(bool_field(join, "preserve_gui_labels_in_cli_and_support"));
    assert!(!bool_field(join, "raw_secrets_exported"));
    assert!(!bool_field(join, "raw_rows_exported_by_default"));
    assert!(!bool_field(join, "raw_response_bodies_exported_by_default"));
    object_kinds
}

fn assert_auto_narrowing_contract(value: &Value, fixture_name: &str) {
    let contract = value
        .get("auto_narrowing_contract")
        .unwrap_or_else(|| panic!("{fixture_name} missing auto-narrowing contract"));
    for trigger in [
        "auth_source_class",
        "origin_boundary",
        "schema_freshness",
        "plan_freshness",
        "export_redaction_posture",
    ] {
        assert!(
            contains_string(contract, "narrow_on_missing_or_stale", trigger),
            "{fixture_name} must narrow on missing or stale {trigger}"
        );
    }
    assert_eq!(
        str_field(contract, "stale_or_missing_effect"),
        "narrow_claim"
    );
    assert_eq!(
        str_field(contract, "policy_blocked_effect"),
        "block_export_or_narrow_claim"
    );
    assert_eq!(
        str_field(contract, "degraded_state_reason_field"),
        "narrowed_claim_reasons"
    );
    assert!(bool_field(contract, "cli_support_reason_parity"));
    assert_eq!(
        str_field(contract, "release_help_claim_ceiling"),
        "available_evidence_only"
    );
}

fn contains_string(value: &Value, field: &str, expected: &str) -> bool {
    array_field(value, field)
        .iter()
        .any(|entry| entry.as_str() == Some(expected))
}

fn array_contains_str(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .is_some_and(|values| values.iter().any(|entry| entry.as_str() == Some(expected)))
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
            enum_contains(&schema, "/$defs/target_location_class/enum", location),
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
    assert_eq!(
        str_field(&bar, "write_risk_state"),
        "read_only_no_write_risk"
    );
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

#[test]
fn result_grid_preserves_virtualized_range_type_rendering_and_export_review_truth() {
    let grid = load_fixture("result_grid.json");

    assert_eq!(str_field(&grid, "record_kind"), "m5_result_grid");
    assert_eq!(
        str_field(&grid, "row_count_scope"),
        "exact_returned_only_total_unknown"
    );
    assert_eq!(i64_field(&grid, "returned_row_count"), 1000);
    assert!(grid.get("total_row_count").is_some_and(Value::is_null));
    assert_eq!(
        str_field(&grid, "truncation_state"),
        "row_truncated_user_limit"
    );
    assert_eq!(
        str_field(&grid, "virtualization_state"),
        "row_and_column_virtualized"
    );

    let loaded = grid
        .get("loaded_range_truth")
        .expect("loaded range truth is present");
    assert_eq!(
        str_field(loaded, "range_scope"),
        "partial_returned_rows_loaded"
    );
    assert_eq!(i64_field(loaded, "loaded_row_count"), 120);
    assert!(bool_field(loaded, "unloaded_rows_known"));
    let ranges = array_field(loaded, "loaded_ranges");
    assert_eq!(i64_field(&ranges[0], "start_row_inclusive"), 0);
    assert_eq!(i64_field(&ranges[0], "end_row_exclusive"), 120);

    let columns = array_field(&grid, "columns");
    for type_identity in ["decimal_or_numeric", "binary_bytes", "json_document"] {
        assert!(
            columns
                .iter()
                .any(|column| column.get("type_identity").and_then(Value::as_str)
                    == Some(type_identity)),
            "grid must preserve {type_identity} column identity"
        );
    }
    assert!(columns.iter().any(|column| {
        column.get("nullable").and_then(Value::as_bool) == Some(true)
            && column.get("rendering_rule").and_then(Value::as_str) == Some("binary_size_digest")
    }));
    assert!(columns.iter().any(|column| {
        column.get("type_identity").and_then(Value::as_str) == Some("json_document")
            && column.get("rendering_rule").and_then(Value::as_str) == Some("json_collapsed_tree")
    }));

    let rendering = grid
        .get("value_rendering_rules")
        .expect("value rendering rules are present");
    assert_eq!(
        str_field(rendering, "null_display"),
        "null_glyph_with_type_tooltip"
    );
    assert_eq!(
        str_field(rendering, "binary_display"),
        "size_and_digest_only"
    );
    assert_eq!(str_field(rendering, "json_display"), "collapsed_tree_typed");
    assert!(!bool_field(rendering, "raw_binary_copy_default"));
    assert!(bool_field(rendering, "json_preserves_type_identity"));

    let export_review = grid.get("export_review").expect("export review is present");
    assert!(bool_field(export_review, "review_required"));
    assert!(bool_field(export_review, "raw_values_blocked_by_default"));
    for action in [
        "review_visible_row_scope",
        "review_binary_payloads",
        "approve_notebook_handoff",
        "approve_support_export",
    ] {
        assert!(
            contains_string(export_review, "review_actions", action),
            "result grid export review must expose {action}"
        );
    }
    for surface in [
        "desktop_database_tool",
        "notebook_handoff",
        "chart_handoff",
        "ai_context_handoff",
        "support_export",
    ] {
        assert!(
            contains_string(&grid, "consumer_surfaces", surface),
            "result grid must project to {surface}"
        );
    }
    assert!(
        contains_string(&grid, "copy_export_actions", "chart_handoff_typed"),
        "result grid must preserve the typed chart handoff action"
    );
    assert!(
        contains_string(&grid, "copy_export_actions", "ai_context_metadata_only"),
        "result grid must preserve the metadata-only AI handoff action"
    );
}

#[test]
fn query_history_row_carries_origin_statement_counts_outcome_and_retention() {
    let row = load_fixture("query_history_row.json");

    assert_eq!(str_field(&row, "record_kind"), "m5_query_history_row");
    assert_eq!(str_field(&row, "connection_label"), "Prod analytics");
    assert_eq!(str_field(&row, "service_label"), "Snowflake prod analytics");
    assert_eq!(str_field(&row, "execution_origin"), "managed_workspace");
    assert_eq!(str_field(&row, "target_location_class"), "managed");
    assert_eq!(str_field(&row, "statement_class"), "read_only_query");
    assert_eq!(i64_field(&row, "duration_ms"), 842);
    assert_eq!(str_field(&row, "retention_mode"), "metadata_only");
    assert_eq!(str_field(&row, "auth_storage_mode"), "policy_injected");
    assert_eq!(
        str_field(&row, "result_grid_ref"),
        "result-grid:snowflake:recent-orders"
    );
    assert_eq!(
        str_field(&row, "explain_plan_ref"),
        "explain-pane:imported-estimated-orders"
    );

    let counts = row.get("result_counts").expect("result counts are present");
    assert_eq!(
        str_field(counts, "row_count_scope"),
        "exact_returned_only_total_unknown"
    );
    assert_eq!(i64_field(counts, "returned_row_count"), 1000);
    assert_eq!(i64_field(counts, "affected_row_count"), 0);
    assert!(counts.get("total_row_count").is_some_and(Value::is_null));

    let outcome = row.get("outcome").expect("outcome is present");
    assert_eq!(str_field(outcome, "status"), "success");
    assert!(outcome.get("error_class").is_some_and(Value::is_null));
    assert!(outcome.get("error_ref").is_some_and(Value::is_null));

    let replay = row.get("replay").expect("replay is present");
    assert_eq!(str_field(replay, "replay_mode"), "current_context_only");
    assert!(!bool_field(replay, "exact_rerun_available"));
    assert!(bool_field(replay, "current_context_replay_available"));

    for surface in [
        "desktop_database_tool",
        "notebook_handoff",
        "chart_handoff",
        "support_export",
    ] {
        assert!(
            contains_string(&row, "consumer_surfaces", surface),
            "query history row must project to {surface}"
        );
    }

    let export = row.get("copy_export").expect("copy/export is present");
    for field in [
        "service_label",
        "execution_origin",
        "statement_class",
        "duration_ms",
        "result_counts",
        "outcome",
        "retention_mode",
    ] {
        assert!(
            contains_string(export, "export_fields", field),
            "query history export must preserve {field}"
        );
    }
}

#[test]
fn explain_plan_pane_keeps_estimated_actual_freshness_and_source_query_truth() {
    let pane = load_fixture("explain_plan_pane.json");

    assert_eq!(str_field(&pane, "record_kind"), "m5_explain_plan_pane");
    assert_eq!(
        str_field(&pane, "statement_identity_ref"),
        "statement:sql:recent-orders"
    );
    assert_eq!(str_field(&pane, "engine_family"), "postgresql");
    assert_eq!(str_field(&pane, "engine_version_ref"), "postgresql:15.1");
    assert_eq!(str_field(&pane, "plan_capture_kind"), "imported_estimated");
    assert_eq!(
        str_field(&pane, "estimated_vs_actual_truth"),
        "imported_estimated_not_replayed"
    );
    assert!(!bool_field(&pane, "actual_execution_disclosed"));
    assert_eq!(str_field(&pane, "freshness_state"), "stale");
    assert_eq!(str_field(&pane, "execution_origin"), "imported_snapshot");
    assert!(array_contains_str(
        pane.get("warnings").expect("warnings are present"),
        "estimated_plan_did_not_execute"
    ));

    let source = pane
        .get("source_query_link")
        .expect("source query link is present");
    assert_eq!(str_field(source, "action"), "open_source_query");
    assert_eq!(
        str_field(source, "statement_identity_ref"),
        str_field(&pane, "statement_identity_ref")
    );
    assert!(bool_field(source, "safe_to_open"));
    assert!(!bool_field(source, "raw_statement_exported"));

    let comparison = pane.get("comparison").expect("comparison is present");
    assert_eq!(
        str_field(comparison, "comparison_basis"),
        "imported_vs_live"
    );
    assert!(bool_field(comparison, "diff_visible"));

    assert!(
        pane.get("result_identity_ref").is_none(),
        "explain plan panes must not masquerade as result data"
    );
    let export = pane.get("copy_export").expect("copy/export is present");
    assert!(
        contains_string(export, "export_fields", "source_query_link"),
        "explain exports must preserve the safe path back to source query text"
    );
    assert!(
        !contains_string(export, "export_fields", "result_identity_ref"),
        "explain exports must not export result-grid identity as plan truth"
    );
}

#[test]
fn result_query_and_plan_schemas_cover_required_truth_vocabularies() {
    let result_schema = load_schema("m5-result-grid.schema.json");
    assert!(enum_contains(
        &result_schema,
        "/properties/loaded_range_truth/properties/range_scope/enum",
        "partial_returned_rows_loaded"
    ));
    assert!(enum_contains(
        &result_schema,
        "/properties/columns/items/properties/rendering_rule/enum",
        "binary_size_digest"
    ));
    assert!(enum_contains(
        &result_schema,
        "/properties/copy_export_actions/items/enum",
        "notebook_handoff_typed"
    ));

    let history_schema = load_schema("m5-query-history-row.schema.json");
    assert!(enum_contains(
        &history_schema,
        "/properties/statement_class/enum",
        "read_only_query"
    ));
    assert!(enum_contains(
        &history_schema,
        "/properties/outcome/properties/status/enum",
        "error"
    ));
    assert!(enum_contains(
        &history_schema,
        "/properties/retention_mode/enum",
        "metadata_only"
    ));

    let plan_schema = load_schema("m5-explain-plan-pane.schema.json");
    for plan_kind in [
        "estimated",
        "actual",
        "imported_estimated",
        "imported_actual",
    ] {
        assert!(
            enum_contains(
                &plan_schema,
                "/properties/plan_capture_kind/enum",
                plan_kind
            ),
            "plan schema must keep {plan_kind} distinct"
        );
    }
    assert!(enum_contains(
        &plan_schema,
        "/properties/source_query_link/properties/action/enum",
        "open_source_query"
    ));
}

#[test]
fn reusable_component_fixtures_carry_reduced_capability_and_provider_handoff_disclosures() {
    let manifest = load_fixture("component_manifest.json");
    let fixtures = array_field(&manifest, "fixtures");
    assert!(fixtures.len() >= 12);

    let mut all_consumers = Vec::new();
    let mut joined_object_kinds = Vec::new();
    for entry in fixtures {
        assert!(
            entry
                .get("reduced_capability_banner_ref")
                .and_then(Value::as_str)
                .is_some(),
            "manifest entry must point at a reduced-capability banner"
        );
        assert!(
            entry
                .get("provider_handoff_note_refs")
                .and_then(Value::as_array)
                .is_some_and(|refs| !refs.is_empty()),
            "manifest entry must point at provider handoff notes"
        );
        assert_accessibility_contract(entry, str_field(entry, "family"));
        joined_object_kinds.extend(assert_support_export_join(
            entry,
            str_field(entry, "family"),
        ));
        assert_auto_narrowing_contract(entry, str_field(entry, "family"));

        let fixture_ref = str_field(entry, "fixture_ref");
        let fixture_name = fixture_ref
            .rsplit('/')
            .next()
            .expect("fixture ref has a filename");
        if fixture_name != "component_manifest.json" {
            let fixture = load_fixture(fixture_name);
            assert_reduced_capability_disclosure(&fixture, fixture_name);
        }

        for consumer in array_field(entry, "claimed_consumers") {
            if let Some(consumer) = consumer.as_str() {
                all_consumers.push(consumer.to_owned());
            }
        }
    }

    for required_consumer in [
        "desktop_request_workspace",
        "desktop_database_tool",
        "notebook_handoff",
        "chart_handoff",
        "browser_runtime_panel",
        "support_export",
    ] {
        assert!(
            all_consumers
                .iter()
                .any(|consumer| consumer == required_consumer),
            "component manifest must include first consumer {required_consumer}"
        );
    }

    for object_kind in ["request_run", "result_set", "history_row", "plan_object"] {
        assert!(
            joined_object_kinds.iter().any(|kind| kind == object_kind),
            "component manifest support joins must cover {object_kind}"
        );
    }
}

#[test]
fn support_export_preserves_accessibility_join_and_narrowing_reason_parity() {
    let export =
        load_repo_json("artifacts/release/m5-request-data-component-proof/support_export.json");
    let manifest = load_fixture("component_manifest.json");
    let manifest_families = array_field(&manifest, "fixtures")
        .iter()
        .map(|entry| str_field(entry, "family").to_owned())
        .collect::<Vec<_>>();
    let export_families = array_field(&export, "rows")
        .iter()
        .map(|row| str_field(row, "family").to_owned())
        .collect::<Vec<_>>();
    for family in manifest_families {
        assert!(
            export_families
                .iter()
                .any(|export_family| export_family == &family),
            "support export must carry a row for manifest family {family}"
        );
    }

    let contract = export
        .get("support_export_contract")
        .expect("support export contract is present");
    assert!(bool_field(
        contract,
        "preserve_gui_labels_in_cli_and_support"
    ));
    assert!(bool_field(
        contract,
        "joins_request_runs_result_sets_history_rows_and_plan_objects"
    ));
    assert!(!bool_field(contract, "raw_secret_export_default"));
    assert!(!bool_field(contract, "raw_result_row_export_default"));
    assert!(!bool_field(contract, "raw_response_body_export_default"));
    assert!(bool_field(contract, "degraded_state_reasons_match_gui"));
    assert_eq!(
        str_field(contract, "release_help_claim_ceiling"),
        "available_evidence_only"
    );

    let freshness = export
        .get("freshness_and_parity")
        .expect("freshness and parity block is present");
    for check in [
        "accessibility_parity",
        "machine_readable_support_join_parity",
        "truth_auto_narrowing",
    ] {
        assert!(
            contains_string(freshness, "narrowing_checks", check),
            "support export must enforce {check}"
        );
    }

    for row in array_field(&export, "rows") {
        assert!(
            str_field(row, "support_export_join_id").starts_with("support-join:"),
            "support export rows must carry join ids"
        );
        assert!(
            str_field(row, "machine_readable_schema_ref").starts_with("schemas/ui/"),
            "support export rows must carry schema refs"
        );

        let a11y = row
            .get("accessibility_summary")
            .expect("accessibility summary is present");
        for field in [
            "keyboard_traversal_exported",
            "screen_reader_label_exported",
            "accessible_text_or_table_fallback_exported",
            "zoom_and_high_density_reviewed",
        ] {
            assert!(bool_field(a11y, field), "row must export {field}");
        }

        for reason in [
            "auth_source_class_missing_or_stale",
            "origin_boundary_missing_or_stale",
            "schema_freshness_missing_or_stale",
            "plan_freshness_missing_or_stale",
            "export_redaction_posture_missing_or_stale",
        ] {
            assert!(
                contains_string(row, "narrowed_claim_reasons", reason),
                "support export row must include narrowed reason {reason}"
            );
        }
        assert!(!bool_field(row, "raw_secret_exported"));
        assert!(!bool_field(row, "raw_rows_exported_by_default"));
        assert!(bool_field(row, "gui_cli_support_label_parity"));
    }
}

#[test]
fn proof_packet_consumer_claims_match_fixture_projection_surfaces() {
    let proof =
        load_repo_json("artifacts/release/m5-request-data-component-proof/proof_packet.json");
    let contract = proof
        .get("qualification_contract")
        .expect("qualification contract is present");
    assert!(bool_field(contract, "accessibility_review_required"));
    assert!(bool_field(contract, "cli_headless_label_parity_required"));
    assert!(bool_field(contract, "support_export_join_required"));
    assert!(!bool_field(contract, "raw_secret_export_default"));
    assert!(!bool_field(contract, "raw_result_row_export_default"));
    assert!(!bool_field(contract, "raw_response_body_export_default"));
    assert_eq!(
        str_field(contract, "release_help_claim_ceiling"),
        "available_evidence_only"
    );
    for trigger in [
        "auth_source_class",
        "origin_boundary",
        "schema_freshness",
        "plan_freshness",
        "export_redaction_posture",
    ] {
        assert!(
            contains_string(contract, "auto_narrowing_triggers", trigger),
            "proof packet must narrow on {trigger}"
        );
    }

    for family in array_field(&proof, "component_families") {
        let fixture_ref = str_field(family, "fixture_ref");
        let fixture = load_repo_json(fixture_ref);
        assert!(
            str_field(family, "accessibility_contract_ref").starts_with("accessibility:"),
            "proof family must carry accessibility contract ref"
        );
        assert!(
            str_field(family, "support_export_join_id").starts_with("support-join:"),
            "proof family must carry support-export join id"
        );
        assert_eq!(
            str_field(family, "machine_readable_schema_ref"),
            str_field(family, "schema_ref"),
            "proof family machine-readable schema ref must match schema ref"
        );
        for trigger in [
            "auth_source_class",
            "origin_boundary",
            "schema_freshness",
            "plan_freshness",
            "export_redaction_posture",
        ] {
            assert!(
                contains_string(family, "auto_narrowing_triggers", trigger),
                "{} proof family must narrow on {trigger}",
                str_field(family, "family")
            );
        }
        for projection in ["gui", "cli_headless", "support_export", "release_proof"] {
            assert!(
                contains_string(family, "narrowed_claim_reason_projection", projection),
                "{} must project narrowed reasons to {projection}",
                str_field(family, "family")
            );
        }

        let mut fixture_surfaces = Vec::new();
        if fixture_ref.ends_with("schema_object_rows.json") {
            for row in array_field(&fixture, "rows") {
                for surface in array_field(row, "consumer_surfaces") {
                    if let Some(surface) = surface.as_str() {
                        fixture_surfaces.push(surface.to_owned());
                    }
                }
            }
        } else {
            for surface in array_field(&fixture, "consumer_surfaces") {
                if let Some(surface) = surface.as_str() {
                    fixture_surfaces.push(surface.to_owned());
                }
            }
        }

        for consumer in array_field(family, "consumer_surfaces") {
            let consumer = consumer
                .as_str()
                .expect("proof consumer surface must be a string");
            assert!(
                fixture_surfaces
                    .iter()
                    .any(|fixture_surface| fixture_surface == consumer),
                "{} proof claim is missing fixture projection surface {consumer}",
                str_field(family, "family")
            );
        }
    }
}
