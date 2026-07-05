//! M5 project-entry component matrix validation.
//!
//! This module consumes the shared component fixture used by Start Center,
//! entry review, CLI/headless, deep links, support export, and release proof.
//! M05-839 tightens the entry chooser and entry review rows so `open`,
//! `clone`, `import`, `restore`, and `resume` remain distinct inspectable
//! verbs, with source-locator, write-scope, host/auth, side-effect, and
//! retained-input diagnostics visible before execution.

use std::collections::BTreeSet;

use serde_json::Value;

/// Schema version exported by the M5 project-entry component fixture.
pub const M5_PROJECT_ENTRY_COMPONENT_SCHEMA_VERSION: u64 = 1;

/// Stable fixture record kind.
pub const M5_PROJECT_ENTRY_COMPONENT_RECORD_KIND: &str =
    "m5_project_entry_component_matrix_fixture";

/// Stable packet id shared by matrix, fixtures, and release proof.
pub const M5_PROJECT_ENTRY_COMPONENT_PACKET_ID: &str = "m5-project-entry-components:stable:0001";

/// Repo-relative path to the checked-in component matrix fixture.
pub const M5_PROJECT_ENTRY_COMPONENT_FIXTURE_REF: &str =
    "fixtures/ui/m5-project-entry-components/component_matrix.json";

/// Embedded checked-in fixture JSON.
pub const M5_PROJECT_ENTRY_COMPONENT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ui/m5-project-entry-components/component_matrix.json"
));

/// Parse the embedded project-entry component matrix fixture.
pub fn current_m5_project_entry_component_matrix() -> Result<Value, serde_json::Error> {
    serde_json::from_str(M5_PROJECT_ENTRY_COMPONENT_JSON)
}

/// Validate the M05-839 entry chooser and review-sheet invariants.
pub fn validate_m5_project_entry_component_matrix(packet: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if packet.get("record_kind").and_then(Value::as_str)
        != Some(M5_PROJECT_ENTRY_COMPONENT_RECORD_KIND)
    {
        errors.push("packet record_kind is not the project-entry component matrix fixture".into());
    }
    if packet.get("schema_version").and_then(Value::as_u64)
        != Some(M5_PROJECT_ENTRY_COMPONENT_SCHEMA_VERSION)
    {
        errors.push("packet schema_version is not 1".into());
    }
    if packet.get("packet_id").and_then(Value::as_str) != Some(M5_PROJECT_ENTRY_COMPONENT_PACKET_ID)
    {
        errors.push("packet_id does not match the stable project-entry component packet".into());
    }

    let components = match packet.get("components").and_then(Value::as_array) {
        Some(components) => components,
        None => {
            errors.push("components array is missing".into());
            return Err(errors);
        }
    };

    validate_no_generic_get_started(components, &mut errors);
    validate_entry_chooser_rows(components, &mut errors);
    validate_entry_review_sheets(components, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_no_generic_get_started(components: &[Value], errors: &mut Vec<String>) {
    for component in components {
        let id = component_id(component);
        for field in ["component_label", "card_label", "helper_text"] {
            if let Some(text) = component.get(field).and_then(Value::as_str) {
                if text.to_ascii_lowercase().contains("get started") {
                    errors.push(format!("{id} uses generic Get started copy in {field}"));
                }
            }
        }
    }
}

fn validate_entry_chooser_rows(components: &[Value], errors: &mut Vec<String>) {
    let chooser_rows: Vec<&Value> = components
        .iter()
        .filter(|component| {
            component.get("component_family").and_then(Value::as_str) == Some("entry_chooser_row")
        })
        .collect();
    let verbs = chooser_rows
        .iter()
        .filter_map(|row| row.get("entry_verb_candidate").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();

    for required in ["open", "clone", "import", "restore"] {
        if !verbs.contains(required) {
            errors.push(format!("entry chooser rows do not cover {required}"));
        }
    }

    for row in chooser_rows {
        let id = component_id(row);
        require_non_empty_string(row, "entry_verb_candidate", &id, errors);
        require_non_empty_array(row, "target_kind_candidates", &id, errors);
        require_non_empty_array(row, "resulting_mode_candidates", &id, errors);
        require_non_empty_string(row, "keyboard_equivalent", &id, errors);
        require_destination_hint(row, &id, errors);
        require_surfaces(
            row,
            &["start_center", "cli_headless", "deep_link"],
            &id,
            errors,
        );
    }
}

fn validate_entry_review_sheets(components: &[Value], errors: &mut Vec<String>) {
    let review_sheets: Vec<&Value> = components
        .iter()
        .filter(|component| {
            component.get("component_family").and_then(Value::as_str) == Some("entry_review_sheet")
        })
        .collect();
    let verbs = review_sheets
        .iter()
        .filter_map(|sheet| sheet.get("entry_verb_candidate").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();

    for required in ["open", "clone", "import", "resume"] {
        if !verbs.contains(required) {
            errors.push(format!("entry review sheets do not cover {required}"));
        }
    }

    for sheet in review_sheets {
        let id = component_id(sheet);
        for field in [
            "entry_verb_candidate",
            "literal_target",
            "normalized_source_locator",
            "source_locator_kind",
            "protocol_class",
            "host_class",
            "auth_posture",
            "resulting_mode",
            "write_scope",
            "post_open_action",
        ] {
            require_non_empty_string(sheet, field, &id, errors);
        }
        require_non_empty_array(sheet, "target_kind_candidates", &id, errors);
        require_non_empty_array(sheet, "resulting_mode_candidates", &id, errors);
        require_surfaces(
            sheet,
            &["entry_review", "cli_headless", "support_export"],
            &id,
            errors,
        );

        if !sheet
            .get("normalized_source_locator")
            .and_then(Value::as_str)
            .is_some_and(|locator| locator.starts_with("locator:"))
        {
            errors.push(format!("{id} has no normalized source locator"));
        }
        if sheet.get("reviewed_before_write").and_then(Value::as_bool) != Some(true) {
            errors.push(format!("{id} is not reviewed before write"));
        }
        if sheet
            .get("no_hidden_hook_or_trust_widening_truth")
            .and_then(Value::as_bool)
            != Some(true)
        {
            errors.push(format!(
                "{id} does not assert no-hidden-hook/trust-widening truth"
            ));
        }
        require_side_effect_truth(sheet, &id, errors);
        require_retained_input_diagnostics(sheet, &id, errors);
        require_write_and_remote_truth(sheet, &id, errors);
    }
}

fn require_write_and_remote_truth(sheet: &Value, id: &str, errors: &mut Vec<String>) {
    let verb = sheet
        .get("entry_verb_candidate")
        .and_then(Value::as_str)
        .unwrap_or("");
    let write_scope = sheet
        .get("write_scope")
        .and_then(Value::as_str)
        .unwrap_or("");
    if matches!(verb, "clone" | "import" | "resume")
        && matches!(write_scope, "" | "inspect_only" | "no_write")
    {
        errors.push(format!("{id} hides write/session scope for {verb}"));
    }

    let protocol = sheet
        .get("protocol_class")
        .and_then(Value::as_str)
        .unwrap_or("");
    let auth = sheet
        .get("auth_posture")
        .and_then(Value::as_str)
        .unwrap_or("");
    if matches!(protocol, "git_https" | "git_ssh" | "managed_session")
        && matches!(auth, "" | "not_applicable")
    {
        errors.push(format!(
            "{id} can contact a remote host without auth posture"
        ));
    }
}

fn require_destination_hint(row: &Value, id: &str, errors: &mut Vec<String>) {
    let Some(destination) = row
        .get("last_used_or_recommended_destination")
        .and_then(Value::as_object)
    else {
        errors.push(format!("{id} is missing last-used/recommended destination"));
        return;
    };
    for field in ["destination_ref", "destination_label", "destination_source"] {
        if !destination
            .get(field)
            .and_then(Value::as_str)
            .is_some_and(|value| !value.trim().is_empty())
        {
            errors.push(format!("{id} destination hint is missing {field}"));
        }
    }
}

fn require_side_effect_truth(sheet: &Value, id: &str, errors: &mut Vec<String>) {
    let Some(truth) = sheet.get("side_effect_truth").and_then(Value::as_object) else {
        errors.push(format!("{id} is missing side_effect_truth"));
        return;
    };
    for field in [
        "hidden_repo_hooks_blocked",
        "hidden_dependency_restore_blocked",
        "hidden_trust_widening_blocked",
    ] {
        if truth.get(field).and_then(Value::as_bool) != Some(true) {
            errors.push(format!("{id} side_effect_truth does not keep {field} true"));
        }
    }
}

fn require_retained_input_diagnostics(sheet: &Value, id: &str, errors: &mut Vec<String>) {
    let Some(diagnostics) = sheet
        .get("retained_input_diagnostics")
        .and_then(Value::as_object)
    else {
        errors.push(format!("{id} is missing retained_input_diagnostics"));
        return;
    };
    for field in ["diagnostic_class", "redacted_error_context"] {
        if !diagnostics
            .get(field)
            .and_then(Value::as_str)
            .is_some_and(|value| !value.trim().is_empty())
        {
            errors.push(format!("{id} retained diagnostics missing {field}"));
        }
    }
    if !diagnostics
        .get("retained_input_refs")
        .and_then(Value::as_array)
        .is_some_and(|refs| !refs.is_empty())
    {
        errors.push(format!("{id} retained diagnostics keep no typed inputs"));
    }
    if !diagnostics
        .get("repair_actions")
        .and_then(Value::as_array)
        .is_some_and(|actions| !actions.is_empty())
    {
        errors.push(format!(
            "{id} retained diagnostics expose no repair actions"
        ));
    }
    if diagnostics
        .get("retry_uses_retained_input")
        .and_then(Value::as_bool)
        != Some(true)
    {
        errors.push(format!("{id} retry does not use retained input"));
    }
}

fn require_surfaces(row: &Value, required: &[&str], id: &str, errors: &mut Vec<String>) {
    let surfaces = row
        .get("consumer_surfaces")
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    for surface in required {
        if !surfaces.contains(surface) {
            errors.push(format!("{id} is missing consumer surface {surface}"));
        }
    }
}

fn require_non_empty_string(row: &Value, field: &str, id: &str, errors: &mut Vec<String>) {
    if !row
        .get(field)
        .and_then(Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
    {
        errors.push(format!("{id} is missing {field}"));
    }
}

fn require_non_empty_array(row: &Value, field: &str, id: &str, errors: &mut Vec<String>) {
    if !row
        .get(field)
        .and_then(Value::as_array)
        .is_some_and(|value| !value.is_empty())
    {
        errors.push(format!("{id} is missing {field}"));
    }
}

fn component_id(component: &Value) -> String {
    component
        .get("component_id")
        .and_then(Value::as_str)
        .unwrap_or("<unknown-component>")
        .to_string()
}
