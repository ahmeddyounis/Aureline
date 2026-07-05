use aureline_shell::m5_project_entry_components::{
    current_m5_project_entry_component_matrix, validate_m5_project_entry_component_matrix,
    M5_PROJECT_ENTRY_COMPONENT_PACKET_ID, M5_PROJECT_ENTRY_COMPONENT_RECORD_KIND,
    M5_PROJECT_ENTRY_COMPONENT_SCHEMA_VERSION,
};
use serde_json::Value;

#[test]
fn project_entry_component_fixture_parses_and_validates() {
    let packet = current_m5_project_entry_component_matrix().expect("fixture parses");

    assert_eq!(
        packet.get("record_kind").and_then(Value::as_str),
        Some(M5_PROJECT_ENTRY_COMPONENT_RECORD_KIND)
    );
    assert_eq!(
        packet.get("schema_version").and_then(Value::as_u64),
        Some(M5_PROJECT_ENTRY_COMPONENT_SCHEMA_VERSION)
    );
    assert_eq!(
        packet.get("packet_id").and_then(Value::as_str),
        Some(M5_PROJECT_ENTRY_COMPONENT_PACKET_ID)
    );
    validate_m5_project_entry_component_matrix(&packet).expect("fixture validates");
}

#[test]
fn entry_chooser_rows_cover_literal_m5_verbs_and_destinations() {
    let packet = current_m5_project_entry_component_matrix().expect("fixture parses");
    let components = packet
        .get("components")
        .and_then(Value::as_array)
        .expect("components array");
    let chooser_rows = components
        .iter()
        .filter(|row| {
            row.get("component_family").and_then(Value::as_str) == Some("entry_chooser_row")
        })
        .collect::<Vec<_>>();

    for verb in ["open", "clone", "import", "restore"] {
        let row = chooser_rows
            .iter()
            .find(|row| row.get("entry_verb_candidate").and_then(Value::as_str) == Some(verb))
            .unwrap_or_else(|| panic!("{verb} chooser row exists"));
        assert!(row
            .get("target_kind_candidates")
            .and_then(Value::as_array)
            .is_some_and(|values| !values.is_empty()));
        assert!(row
            .get("resulting_mode_candidates")
            .and_then(Value::as_array)
            .is_some_and(|values| !values.is_empty()));
        assert!(row
            .get("last_used_or_recommended_destination")
            .and_then(Value::as_object)
            .is_some());
        assert!(row
            .get("keyboard_equivalent")
            .and_then(Value::as_str)
            .is_some_and(|value| !value.trim().is_empty()));
    }
}

#[test]
fn review_sheets_expose_locator_write_scope_and_retained_diagnostics() {
    let packet = current_m5_project_entry_component_matrix().expect("fixture parses");
    let components = packet
        .get("components")
        .and_then(Value::as_array)
        .expect("components array");
    let review_sheets = components
        .iter()
        .filter(|row| {
            row.get("component_family").and_then(Value::as_str) == Some("entry_review_sheet")
        })
        .collect::<Vec<_>>();

    for verb in ["open", "clone", "import", "resume"] {
        let sheet = review_sheets
            .iter()
            .find(|sheet| sheet.get("entry_verb_candidate").and_then(Value::as_str) == Some(verb))
            .unwrap_or_else(|| panic!("{verb} review sheet exists"));
        assert!(sheet
            .get("literal_target")
            .and_then(Value::as_str)
            .is_some_and(|value| !value.trim().is_empty()));
        assert!(sheet
            .get("normalized_source_locator")
            .and_then(Value::as_str)
            .is_some_and(|value| value.starts_with("locator:")));
        assert!(sheet
            .get("write_scope")
            .and_then(Value::as_str)
            .is_some_and(|value| !value.trim().is_empty()));
        assert_eq!(
            sheet
                .get("no_hidden_hook_or_trust_widening_truth")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert!(sheet
            .get("retained_input_diagnostics")
            .and_then(Value::as_object)
            .and_then(|diagnostics| diagnostics.get("retained_input_refs"))
            .and_then(Value::as_array)
            .is_some_and(|values| !values.is_empty()));
    }
}

#[test]
fn validator_flags_hidden_side_effect_and_lost_retained_inputs() {
    let mut packet = current_m5_project_entry_component_matrix().expect("fixture parses");
    let components = packet
        .get_mut("components")
        .and_then(Value::as_array_mut)
        .expect("components array");
    let clone_sheet = components
        .iter_mut()
        .find(|row| {
            row.get("component_family").and_then(Value::as_str) == Some("entry_review_sheet")
                && row.get("entry_verb_candidate").and_then(Value::as_str) == Some("clone")
        })
        .expect("clone sheet");

    clone_sheet["side_effect_truth"]["hidden_repo_hooks_blocked"] = Value::Bool(false);
    clone_sheet["retained_input_diagnostics"]["retained_input_refs"] = Value::Array(Vec::new());

    let errors =
        validate_m5_project_entry_component_matrix(&packet).expect_err("mutated packet fails");
    assert!(errors
        .iter()
        .any(|error| error.contains("hidden_repo_hooks_blocked")));
    assert!(errors
        .iter()
        .any(|error| error.contains("keep no typed inputs")));
}
