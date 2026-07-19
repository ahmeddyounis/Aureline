//! M5 project-entry component matrix validation.
//!
//! This module consumes the shared component fixture used by Start Center,
//! entry review, CLI/headless, deep links, support export, and release proof.
//! M05-839 tightens the entry chooser and entry review rows so `open`,
//! `clone`, `import`, `restore`, and `resume` remain distinct inspectable
//! verbs, with source-locator, write-scope, host/auth, side-effect, and
//! retained-input diagnostics visible before execution.
//!
//! M05-841 adds admission-checkpoint cards (root identity, trust class,
//! archetype/bundle recommendation source, blocked-vs-optional readiness
//! tasks, and `Continue without`/`Set up later` choices), archetype/readiness
//! rows across certified, probable, mixed, generic, restricted, and
//! missing-prerequisite outcomes with confidence and evidence source, and
//! first-useful-work routing that stays attributable to the entry source while
//! preserving a same-weight plain-open path.

use std::collections::{BTreeMap, BTreeSet};

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

/// One follow-up vocabulary shared by deferred setup, non-durable staging, and
/// safe reuse/add/clone-elsewhere paths across local, remote, template,
/// prebuild, and import flows (M05-840).
pub const M5_FOLLOW_UP_STATE_VOCABULARY: &[&str] = &[
    "setup_deferred_durable",
    "non_durable_staging",
    "safe_reuse_available",
    "safe_add_existing_available",
    "safe_clone_elsewhere_available",
    "open_minimal_available",
];

/// Safe destination-collision actions offered instead of generic overwrite or
/// retry copy (M05-840).
pub const M5_COLLISION_SAFE_ACTION_VOCABULARY: &[&str] = &[
    "reuse_existing",
    "add_existing_to_workspace",
    "clone_elsewhere",
    "reveal_in_filesystem",
    "inspect_only",
    "cancel_no_change",
];

/// Archetype/readiness outcomes an admission checkpoint must be able to state
/// without pretending certainty: certified match, probable match,
/// mixed/ambiguous, unknown/generic, restricted/policy-blocked, and
/// missing-toolchain/remote-prerequisite (M05-841).
pub const M5_ARCHETYPE_OUTCOME_VOCABULARY: &[&str] = &[
    "certified",
    "probable",
    "mixed",
    "generic",
    "restricted",
    "missing_prerequisite",
];

/// Entry sources whose first-useful-work cards must route differently while
/// preserving a same-weight plain-open path (M05-841).
pub const M5_FIRST_USEFUL_WORK_ENTRY_SOURCES: &[&str] = &[
    "single_file_open",
    "folder_or_repo_open",
    "repo_clone",
    "restore",
    "review_link_open",
    "imported_handoff_packet",
];

/// Entry sources whose plain-open path must route to ordinary editing rather
/// than a generic welcome tab (M05-841).
pub const M5_PLAIN_OPEN_ENTRY_SOURCES: &[&str] = &["single_file_open", "folder_or_repo_open"];

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
    validate_destination_collision_sheets(components, &mut errors);
    validate_post_entry_handoff_cards(components, &mut errors);
    validate_admission_checkpoint_cards(components, &mut errors);
    validate_archetype_readiness_rows(components, &mut errors);
    validate_first_useful_work_routing(components, &mut errors);

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

fn validate_destination_collision_sheets(components: &[Value], errors: &mut Vec<String>) {
    let sheets: Vec<&Value> = components
        .iter()
        .filter(|component| {
            component.get("component_family").and_then(Value::as_str)
                == Some("destination_collision_sheet")
        })
        .collect();

    let sources = sheets
        .iter()
        .filter_map(|sheet| sheet.get("collision_source_class").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();
    for required in [
        "existing_local_root",
        "prior_workspace_state",
        "duplicate_clone_target",
    ] {
        if !sources.contains(required) {
            errors.push(format!(
                "destination collision sheets do not distinguish {required}"
            ));
        }
    }

    for sheet in sheets {
        let id = component_id(sheet);
        for field in [
            "collision_class",
            "collision_source_class",
            "existing_target_identity_ref",
            "existing_target_label",
        ] {
            require_non_empty_string(sheet, field, &id, errors);
        }
        require_non_empty_array(sheet, "safe_actions", &id, errors);
        require_non_empty_array(sheet, "safe_action_labels", &id, errors);

        if sheet.get("blocks_until_choice").and_then(Value::as_bool) != Some(true) {
            errors.push(format!("{id} does not block until an explicit choice"));
        }
        if sheet
            .get("overwrite_or_retry_copy_forbidden")
            .and_then(Value::as_bool)
            != Some(true)
        {
            errors.push(format!(
                "{id} can fall back to generic overwrite or retry copy"
            ));
        }

        let safe_actions = string_set(sheet, "safe_actions");
        for action in &safe_actions {
            if !M5_COLLISION_SAFE_ACTION_VOCABULARY.contains(&action.as_str()) {
                errors.push(format!(
                    "{id} offers non-canonical collision action {action}"
                ));
            }
        }
        let source = sheet
            .get("collision_source_class")
            .and_then(Value::as_str)
            .unwrap_or("");
        if source != "policy_blocked_destination" {
            if !safe_actions.iter().any(|action| {
                matches!(
                    action.as_str(),
                    "reuse_existing" | "add_existing_to_workspace" | "clone_elsewhere"
                )
            }) {
                errors.push(format!(
                    "{id} offers no safe reuse, add-existing, or clone-elsewhere choice"
                ));
            }
            if !safe_actions.contains("reveal_in_filesystem") {
                errors.push(format!("{id} does not offer reveal in filesystem"));
            }
        }
    }
}

fn validate_post_entry_handoff_cards(components: &[Value], errors: &mut Vec<String>) {
    let cards: Vec<&Value> = components
        .iter()
        .filter(|component| {
            component.get("component_family").and_then(Value::as_str)
                == Some("post_entry_handoff_card")
        })
        .collect();

    let follow_up_states = cards
        .iter()
        .filter_map(|card| card.get("follow_up_state_class").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();
    for required in [
        "setup_deferred_durable",
        "non_durable_staging",
        "open_minimal_available",
    ] {
        if !follow_up_states.contains(required) {
            errors.push(format!(
                "post-entry handoff cards do not cover follow-up state {required}"
            ));
        }
    }

    for card in cards {
        let id = component_id(card);
        for field in [
            "entry_verb",
            "opened_object_ref",
            "opened_object_label",
            "recommended_next_action",
            "follow_up_state_class",
            "export_or_share_state",
        ] {
            require_non_empty_string(card, field, &id, errors);
        }
        require_non_empty_array(card, "pending_setup_or_trust_tasks", &id, errors);
        require_non_empty_array(card, "intentionally_not_done", &id, errors);
        require_non_empty_array(card, "handoff_actions", &id, errors);

        if card.get("set_up_later_available").and_then(Value::as_bool) != Some(true) {
            errors.push(format!("{id} does not keep set-up-later available"));
        }
        if card.get("open_minimal_available").and_then(Value::as_bool) != Some(true) {
            errors.push(format!("{id} does not keep open-minimal available"));
        }

        let follow_up = card
            .get("follow_up_state_class")
            .and_then(Value::as_str)
            .unwrap_or("");
        if !follow_up.is_empty() && !M5_FOLLOW_UP_STATE_VOCABULARY.contains(&follow_up) {
            errors.push(format!(
                "{id} uses non-canonical follow-up state {follow_up}"
            ));
        }

        let handoff_actions = string_set(card, "handoff_actions");
        for required in ["set_up_later", "open_minimal"] {
            if !handoff_actions.contains(required) {
                errors.push(format!("{id} handoff actions do not offer {required}"));
            }
        }
    }
}

fn validate_admission_checkpoint_cards(components: &[Value], errors: &mut Vec<String>) {
    let cards: Vec<&Value> = components
        .iter()
        .filter(|component| {
            component.get("component_family").and_then(Value::as_str)
                == Some("admission_checkpoint_card")
        })
        .collect();

    for card in cards {
        let id = component_id(card);
        for field in [
            "admission_class",
            "root_identity_ref",
            "root_identity_label",
            "recommendation_source",
        ] {
            require_non_empty_string(card, field, &id, errors);
        }
        require_surfaces(
            card,
            &["admission_checkpoint", "cli_headless", "support_export"],
            &id,
            errors,
        );

        // `Continue without` and `Set up later` must both stay reachable so an
        // admission checkpoint never monopolizes plain editing.
        if card
            .get("continue_without_available")
            .and_then(Value::as_bool)
            != Some(true)
        {
            errors.push(format!("{id} does not keep continue-without available"));
        }
        if card.get("set_up_later_available").and_then(Value::as_bool) != Some(true) {
            errors.push(format!("{id} does not keep set-up-later available"));
        }
        let checkpoint_actions = string_set(card, "checkpoint_actions");
        if checkpoint_actions.is_empty() {
            errors.push(format!("{id} exposes no checkpoint actions"));
        }
        for required in ["continue_without", "set_up_later"] {
            if !checkpoint_actions.contains(required) {
                errors.push(format!("{id} checkpoint actions do not offer {required}"));
            }
        }

        // Blocked-vs-optional readiness tasks stay explicit and reconcile with
        // the summary totals rather than collapsing into one urgency.
        let summary = card
            .get("readiness_bucket_summary")
            .and_then(Value::as_object);
        let Some(summary) = summary else {
            errors.push(format!("{id} is missing readiness_bucket_summary"));
            continue;
        };
        let tasks = card
            .get("readiness_tasks")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let mut bucket_counts: BTreeMap<&str, u64> = BTreeMap::new();
        for task in &tasks {
            let bucket = task
                .get("readiness_bucket")
                .and_then(Value::as_str)
                .unwrap_or("");
            if bucket.is_empty() {
                errors.push(format!("{id} has a readiness task with no bucket"));
                continue;
            }
            *bucket_counts.entry(bucket).or_default() += 1;
            match bucket {
                "blocking_now" => {
                    if !task
                        .get("blocked_reason_class")
                        .and_then(Value::as_str)
                        .is_some_and(|value| !value.trim().is_empty())
                    {
                        errors.push(format!("{id} blocking task states no blocked reason"));
                    }
                }
                "optional_later" => {
                    if !task
                        .get("optional_reason_class")
                        .and_then(Value::as_str)
                        .is_some_and(|value| !value.trim().is_empty())
                    {
                        errors.push(format!("{id} optional task states no optional reason"));
                    }
                }
                _ => {}
            }
        }
        for (bucket, field) in [
            ("blocking_now", "blocking_now_total"),
            ("recommended_soon", "recommended_soon_total"),
            ("optional_later", "optional_later_total"),
        ] {
            let total = summary.get(field).and_then(Value::as_u64).unwrap_or(0);
            let counted = bucket_counts.get(bucket).copied().unwrap_or(0);
            if total != counted {
                errors.push(format!(
                    "{id} {field} ({total}) does not match its {bucket} tasks ({counted})"
                ));
            }
        }

        let blocking_total = summary
            .get("blocking_now_total")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        let blocked_reasons = card
            .get("blocked_reason_classes")
            .and_then(Value::as_array)
            .map(|values| values.iter().filter_map(Value::as_str).count())
            .unwrap_or(0);
        if blocking_total > 0 && blocked_reasons == 0 {
            errors.push(format!(
                "{id} has blocking work but no blocked reason class"
            ));
        }
    }
}

fn validate_archetype_readiness_rows(components: &[Value], errors: &mut Vec<String>) {
    let rows: Vec<&Value> = components
        .iter()
        .filter(|component| {
            component.get("component_family").and_then(Value::as_str)
                == Some("archetype_readiness_row")
        })
        .collect();

    let outcomes = rows
        .iter()
        .filter_map(|row| row.get("detected_archetype_class").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();
    for required in M5_ARCHETYPE_OUTCOME_VOCABULARY {
        if !outcomes.contains(required) {
            errors.push(format!(
                "archetype readiness rows do not cover the {required} outcome"
            ));
        }
    }

    for row in rows {
        let id = component_id(row);
        for field in [
            "detected_archetype_class",
            "readiness_bucket",
            "setup_location_class",
            "confidence_class",
            "evidence_source_class",
        ] {
            require_non_empty_string(row, field, &id, errors);
        }
        require_surfaces(
            row,
            &["admission_checkpoint", "docs_help", "support_export"],
            &id,
            errors,
        );

        let outcome = row
            .get("detected_archetype_class")
            .and_then(Value::as_str)
            .unwrap_or("");
        let readiness = row
            .get("readiness_bucket")
            .and_then(Value::as_str)
            .unwrap_or("");
        let confidence = row
            .get("confidence_class")
            .and_then(Value::as_str)
            .unwrap_or("");
        let has_blocked_reason = row
            .get("blocked_reason_class")
            .and_then(Value::as_str)
            .is_some_and(|value| !value.trim().is_empty());

        // Restricted/policy-blocked and missing-prerequisite outcomes are
        // blocking-now and must attribute the blocked reason.
        if matches!(outcome, "restricted" | "missing_prerequisite") {
            if !has_blocked_reason {
                errors.push(format!("{id} blocked outcome states no blocked reason"));
            }
            if readiness != "blocking_now" {
                errors.push(format!(
                    "{id} blocked outcome must sit in blocking_now, not {readiness}"
                ));
            }
        }

        // Generic/unknown outcomes must not overclaim confidence.
        if outcome == "generic" && !matches!(confidence, "none" | "low") {
            errors.push(format!(
                "{id} generic outcome overclaims {confidence} confidence"
            ));
        }
    }
}

fn validate_first_useful_work_routing(components: &[Value], errors: &mut Vec<String>) {
    let cards: Vec<&Value> = components
        .iter()
        .filter(|component| {
            component.get("component_family").and_then(Value::as_str)
                == Some("post_entry_handoff_card")
        })
        .collect();

    let sources = cards
        .iter()
        .filter_map(|card| card.get("entry_source_class").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();
    for required in M5_FIRST_USEFUL_WORK_ENTRY_SOURCES {
        if !sources.contains(required) {
            errors.push(format!(
                "first-useful-work routing does not cover entry source {required}"
            ));
        }
    }

    let routes = cards
        .iter()
        .filter_map(|card| card.get("first_useful_work_route").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();
    if routes.len() < 3 {
        errors.push(
            "first-useful-work routing collapses into fewer than three distinct routes".into(),
        );
    }

    for card in cards {
        let id = component_id(card);
        require_non_empty_string(card, "entry_source_class", &id, errors);
        require_non_empty_string(card, "first_useful_work_route", &id, errors);

        // The same-weight plain-open path must stay available for every entry
        // source instead of routing users into a universal welcome tab.
        if card.get("plain_open_same_weight").and_then(Value::as_bool) != Some(true) {
            errors.push(format!("{id} drops the same-weight plain-open path"));
        }
        if card.get("open_minimal_available").and_then(Value::as_bool) != Some(true) {
            errors.push(format!("{id} does not keep plain open-minimal available"));
        }

        let source = card
            .get("entry_source_class")
            .and_then(Value::as_str)
            .unwrap_or("");
        let route = card
            .get("first_useful_work_route")
            .and_then(Value::as_str)
            .unwrap_or("");
        if M5_PLAIN_OPEN_ENTRY_SOURCES.contains(&source) && route != "ordinary_editing" {
            errors.push(format!(
                "{id} plain-open source {source} routes to {route} instead of ordinary_editing"
            ));
        }
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

fn string_set(row: &Value, field: &str) -> BTreeSet<String> {
    row.get(field)
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<BTreeSet<String>>()
        })
        .unwrap_or_default()
}

fn component_id(component: &Value) -> String {
    component
        .get("component_id")
        .and_then(Value::as_str)
        .unwrap_or("<unknown-component>")
        .to_string()
}
