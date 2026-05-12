//! Protected fixture tests for alpha palette discoverability.

use std::collections::HashMap;
use std::path::Path;

use aureline_commands::registry::seeded_registry;
use aureline_shell::commands::CommandReviewRuntimeInputs;
use aureline_shell::palette::{
    materialize_alpha_palette_query, materialize_alpha_palette_support_export,
    materialize_command_deep_link_review, AlphaFileCandidate, AlphaPaletteQueryInputs,
    AlphaPaletteResultRow, AlphaRecentActionCandidate, AlphaSymbolCandidate,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AlphaPaletteFixture {
    case_id: String,
    query: String,
    runtime: RuntimeFixture,
    #[serde(default)]
    shortcuts: Vec<ShortcutFixture>,
    #[serde(default)]
    recent_actions: Vec<AlphaRecentActionCandidate>,
    #[serde(default)]
    symbols: Vec<AlphaSymbolCandidate>,
    #[serde(default)]
    files: Vec<AlphaFileCandidate>,
    expectations: Expectations,
}

#[derive(Debug, Deserialize)]
struct RuntimeFixture {
    client_scope: String,
    workspace_trust_state: String,
    execution_context_available: bool,
    provider_linked: Option<bool>,
    credential_available: Option<bool>,
    policy_disabled: bool,
    policy_blocked_in_context: bool,
    labs_enabled: bool,
}

#[derive(Debug, Deserialize)]
struct ShortcutFixture {
    command_id: String,
    shortcuts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Expectations {
    required_row_kinds_in_order: Vec<String>,
    required_rows: Vec<ExpectedRow>,
    deep_link_review: Option<ExpectedDeepLinkReview>,
}

#[derive(Debug, Deserialize)]
struct ExpectedRow {
    row_kind: String,
    label: Option<String>,
    command_id: Option<String>,
    availability_class: Option<String>,
    disabled_reason_code: Option<String>,
    origin_source_badge: Option<String>,
    category_contains: Option<String>,
    winning_keybinding: Option<String>,
    default_action_class: Option<String>,
    default_action_enabled: Option<bool>,
    split_or_alternate_open_enabled: Option<bool>,
    copy_command_id_enabled: Option<bool>,
    copy_cli_enabled: Option<bool>,
    add_to_recipe_enabled: Option<bool>,
    diagnostics_sheet_available: Option<bool>,
    invocation_preview_required: Option<bool>,
    preview_side_effect_class: Option<String>,
    rollback_or_checkpoint_posture: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedDeepLinkReview {
    command_id: String,
    route_outcome_class: String,
    preflight_decision_class: String,
    sheet_kind: String,
    diagnostics_reason: Option<String>,
    preview_side_effect_class: Option<String>,
    rollback_evidence_ref: Option<String>,
    preview_shown: Option<bool>,
}

fn runtime_inputs(runtime: &RuntimeFixture) -> CommandReviewRuntimeInputs<'_> {
    CommandReviewRuntimeInputs {
        client_scope: &runtime.client_scope,
        workspace_trust_state: &runtime.workspace_trust_state,
        execution_context_available: runtime.execution_context_available,
        provider_linked: runtime.provider_linked,
        credential_available: runtime.credential_available,
        policy_disabled: runtime.policy_disabled,
        policy_blocked_in_context: runtime.policy_blocked_in_context,
        labs_enabled: runtime.labs_enabled,
    }
}

fn shortcuts(fixtures: &[ShortcutFixture]) -> HashMap<String, Vec<String>> {
    fixtures
        .iter()
        .map(|row| (row.command_id.clone(), row.shortcuts.clone()))
        .collect()
}

fn fixture_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/commands/alpha_palette_queries")
}

fn load_fixture(path: &Path) -> AlphaPaletteFixture {
    let payload = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn run_fixture(path: &Path) {
    let fixture = load_fixture(path);
    let registry = seeded_registry();
    let shortcuts = shortcuts(&fixture.shortcuts);
    let runtime = runtime_inputs(&fixture.runtime);
    let snapshot = materialize_alpha_palette_query(AlphaPaletteQueryInputs {
        registry,
        query: &fixture.query,
        shortcuts_by_command_id: &shortcuts,
        runtime,
        recent_actions: &fixture.recent_actions,
        symbols: &fixture.symbols,
        files: &fixture.files,
    });

    assert!(
        !snapshot.rows.is_empty(),
        "case {} should produce inspectable rows",
        fixture.case_id
    );
    assert_row_contract_is_complete(&fixture.case_id, &snapshot.rows);
    assert_support_export_is_redacted(&fixture.case_id, &snapshot);
    assert_kind_order(
        &fixture.case_id,
        &snapshot.rows,
        &fixture.expectations.required_row_kinds_in_order,
    );

    for expected in &fixture.expectations.required_rows {
        let row = find_expected_row(&snapshot.rows, expected).unwrap_or_else(|| {
            panic!(
                "case {} missing expected row {:?}; observed rows: {:#?}",
                fixture.case_id, expected, snapshot.rows
            )
        });
        assert_expected_row(&fixture.case_id, row, expected);
    }

    if let Some(expected) = &fixture.expectations.deep_link_review {
        let review = materialize_command_deep_link_review(
            registry,
            &expected.command_id,
            runtime_inputs(&fixture.runtime),
        )
        .unwrap_or_else(|| {
            panic!(
                "case {} could not materialize deep-link review for {}",
                fixture.case_id, expected.command_id
            )
        });
        assert_eq!(
            review.route_outcome_class, expected.route_outcome_class,
            "case {} route outcome mismatch",
            fixture.case_id
        );
        assert_eq!(
            review.preflight_decision_class, expected.preflight_decision_class,
            "case {} preflight mismatch",
            fixture.case_id
        );
        assert!(review.no_bypass_guards.preview_path_preserved);
        assert!(review.no_bypass_guards.policy_revalidation_required);
        assert_deep_link_sheet(&fixture.case_id, &review, expected);
    }
}

fn assert_support_export_is_redacted(
    case_id: &str,
    snapshot: &aureline_shell::palette::AlphaPaletteDiscoverabilitySnapshot,
) {
    let export = materialize_alpha_palette_support_export(snapshot);
    assert_eq!(export.source_record_kind, snapshot.record_kind);
    assert_eq!(export.row_count, snapshot.rows.len(), "case {case_id}");
    assert_eq!(export.redaction_class, "metadata_safe_no_query_text");
    assert!(
        export
            .omitted_material
            .iter()
            .any(|value| value == "raw_query_text"),
        "case {case_id} support export must omit raw query text"
    );
}

fn assert_row_contract_is_complete(case_id: &str, rows: &[AlphaPaletteResultRow]) {
    for row in rows {
        assert!(
            !row.origin_source_badge.trim().is_empty(),
            "case {case_id} row {} missing origin/source badge",
            row.row_id
        );
        assert!(
            !row.category_or_path.trim().is_empty(),
            "case {case_id} row {} missing category/path",
            row.row_id
        );
        assert!(
            !row.winning_keybinding.trim().is_empty(),
            "case {case_id} row {} missing keybinding state",
            row.row_id
        );
        assert!(
            !row.dominant_side_effect_class.trim().is_empty(),
            "case {case_id} row {} missing side-effect cue",
            row.row_id
        );
        if row.row_kind.as_str() == "command" || row.command_id.is_some() {
            assert!(
                row.action_footer.copy_command_id.enabled,
                "case {case_id} command row {} must allow copy command id",
                row.row_id
            );
            assert_eq!(
                row.action_footer.copy_command_id.copy_payload,
                row.command_id.clone()
            );
        }
    }
}

fn assert_kind_order(case_id: &str, rows: &[AlphaPaletteResultRow], expected_order: &[String]) {
    let mut previous_index = None;
    for expected_kind in expected_order {
        let index = rows
            .iter()
            .position(|row| row.row_kind.as_str() == expected_kind)
            .unwrap_or_else(|| {
                panic!("case {case_id} missing row kind {expected_kind}; rows: {rows:#?}")
            });
        if let Some(previous) = previous_index {
            assert!(
                previous < index,
                "case {case_id} row kind {expected_kind} appeared out of order"
            );
        }
        previous_index = Some(index);
    }
}

fn find_expected_row<'a>(
    rows: &'a [AlphaPaletteResultRow],
    expected: &ExpectedRow,
) -> Option<&'a AlphaPaletteResultRow> {
    rows.iter().find(|row| {
        row.row_kind.as_str() == expected.row_kind
            && expected
                .label
                .as_ref()
                .map_or(true, |label| row.label == *label)
            && expected.command_id.as_ref().map_or(true, |command_id| {
                row.command_id.as_ref() == Some(command_id)
            })
    })
}

fn assert_expected_row(case_id: &str, row: &AlphaPaletteResultRow, expected: &ExpectedRow) {
    if let Some(value) = expected.availability_class.as_deref() {
        assert_eq!(row.availability_class, value, "case {case_id}");
    }
    if let Some(value) = expected.disabled_reason_code.as_deref() {
        assert_eq!(
            row.disabled_reason_code.as_deref(),
            Some(value),
            "case {case_id}"
        );
    }
    if let Some(value) = expected.origin_source_badge.as_deref() {
        assert_eq!(row.origin_source_badge, value, "case {case_id}");
    }
    if let Some(value) = expected.category_contains.as_deref() {
        assert!(
            row.category_or_path.contains(value),
            "case {case_id} expected category/path {:?} to contain {:?}",
            row.category_or_path,
            value
        );
    }
    if let Some(value) = expected.winning_keybinding.as_deref() {
        assert_eq!(row.winning_keybinding, value, "case {case_id}");
    }
    if let Some(value) = expected.default_action_class.as_deref() {
        assert_eq!(
            row.action_footer.default_action.action_class, value,
            "case {case_id}"
        );
    }
    if let Some(value) = expected.default_action_enabled {
        assert_eq!(
            row.action_footer.default_action.enabled, value,
            "case {case_id}"
        );
    }
    if let Some(value) = expected.split_or_alternate_open_enabled {
        assert_eq!(
            row.action_footer.split_or_alternate_open.enabled, value,
            "case {case_id}"
        );
    }
    if let Some(value) = expected.copy_command_id_enabled {
        assert_eq!(
            row.action_footer.copy_command_id.enabled, value,
            "case {case_id}"
        );
    }
    if let Some(value) = expected.copy_cli_enabled {
        assert_eq!(
            row.action_footer.copy_cli_headless_form.enabled, value,
            "case {case_id}"
        );
    }
    if let Some(value) = expected.add_to_recipe_enabled {
        assert_eq!(
            row.action_footer.add_to_recipe.enabled, value,
            "case {case_id}"
        );
    }
    if let Some(value) = expected.diagnostics_sheet_available {
        assert_eq!(
            row.action_footer.command_diagnostics_sheet_available, value,
            "case {case_id}"
        );
    }
    if let Some(value) = expected.invocation_preview_required {
        assert_eq!(
            row.action_footer.invocation_preview_required, value,
            "case {case_id}"
        );
    }
    if let Some(value) = expected.preview_side_effect_class.as_deref() {
        assert_eq!(row.preview.side_effect_class, value, "case {case_id}");
    }
    if let Some(value) = expected.rollback_or_checkpoint_posture.as_deref() {
        assert_eq!(
            row.preview.rollback_or_checkpoint_posture, value,
            "case {case_id}"
        );
    }
}

fn assert_deep_link_sheet(
    case_id: &str,
    review: &aureline_shell::palette::CommandDeepLinkReviewRecord,
    expected: &ExpectedDeepLinkReview,
) {
    match expected.sheet_kind.as_str() {
        "diagnostics_sheet" => {
            let sheet = review
                .diagnostics_sheet
                .as_ref()
                .unwrap_or_else(|| panic!("case {case_id} missing diagnostics sheet"));
            assert!(review.invocation_preview_sheet.is_none());
            if let Some(reason) = expected.diagnostics_reason.as_deref() {
                assert_eq!(
                    sheet
                        .disabled_reason
                        .as_ref()
                        .map(|details| details.disabled_reason_code.as_str()),
                    Some(reason),
                    "case {case_id}"
                );
            }
        }
        "invocation_preview_sheet" => {
            let sheet = review
                .invocation_preview_sheet
                .as_ref()
                .unwrap_or_else(|| panic!("case {case_id} missing invocation preview sheet"));
            assert!(review.diagnostics_sheet.is_none());
            if let Some(side_effect) = expected.preview_side_effect_class.as_deref() {
                assert_eq!(sheet.packet.dominant_side_effect_class, side_effect);
            }
            if let Some(evidence) = expected.rollback_evidence_ref.as_deref() {
                assert!(
                    sheet
                        .packet
                        .evidence_ref_class_required
                        .iter()
                        .any(|value| value == evidence),
                    "case {case_id} missing evidence ref {evidence}"
                );
            }
            if let Some(preview_shown) = expected.preview_shown {
                assert_eq!(
                    sheet.invocation_session.preview_posture.preview_shown,
                    preview_shown
                );
            }
        }
        other => panic!("case {case_id} unknown expected sheet kind {other}"),
    }
}

#[test]
fn open_query_surfaces_recent_commands_symbols_and_files() {
    run_fixture(&fixture_dir().join("wedge_query_open.json"));
}

#[test]
fn blocked_command_remains_discoverable_with_diagnostics() {
    run_fixture(&fixture_dir().join("blocked_clone_discoverable.json"));
}

#[test]
fn preview_required_deeplink_uses_invocation_preview() {
    run_fixture(&fixture_dir().join("import_preview_deeplink.json"));
}
