//! Fixture-driven coverage for local Git diff-view packets.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_review::diff::{
    DiffFileInput, DiffOpenTarget, DiffScrollAnchor, DiffViewSurfacePacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DiffViewFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    change_list_row: ChangeListRowFixture,
    input: DiffFileInput,
    expected: ExpectedDiffView,
}

#[derive(Debug, Deserialize)]
struct ChangeListRowFixture {
    row_ref: String,
    file_state_token: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedDiffView {
    syntax_class_token: String,
    compare_target_kind: String,
    required_copy_representations: Vec<String>,
    required_warning_classes: Vec<String>,
    rows_have_path_and_target_truth: bool,
    safe_copy_visible_on_suspicious_rows: bool,
    reopen: Option<ExpectedReopen>,
}

#[derive(Debug, Deserialize)]
struct ExpectedReopen {
    selected_hunk_index: usize,
    scroll_offset: u32,
    fallback_open_file_used: bool,
    restores_compare_target: bool,
    restores_path_truth: bool,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/diff_view_alpha")
}

fn open_target(fixture: &DiffViewFixture) -> DiffOpenTarget {
    DiffOpenTarget::from_change_list_row_parts(
        &fixture.input.workspace_ref,
        &fixture.input.truth_source_ref,
        &fixture.change_list_row.row_ref,
        &fixture.input.group_token,
        fixture.input.path.clone(),
        fixture.input.original_path.clone(),
        &fixture.input.status_code,
        &fixture.change_list_row.file_state_token,
    )
}

fn all_copy_representations(packet: &DiffViewSurfacePacket) -> BTreeSet<String> {
    packet
        .hunks
        .iter()
        .flat_map(|hunk| hunk.rows.iter())
        .flat_map(|row| row.copy_actions.iter())
        .map(|action| action.representation_token.clone())
        .collect()
}

#[test]
fn protected_diff_view_fixtures_preserve_syntax_suspicion_and_reopen_truth() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "diff-view fixtures must exist");

    for path in fixtures {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: DiffViewFixture = serde_yaml::from_str(&text)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(fixture.record_kind, "git_diff_view_alpha_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = DiffViewSurfacePacket::from_file_input(open_target(&fixture), fixture.input);
        assert_eq!(
            packet.syntax.syntax_class_token, fixture.expected.syntax_class_token,
            "{}: syntax class mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.compare_target.target_kind_token, fixture.expected.compare_target_kind,
            "{}: compare target mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.all_rows_have_exact_path_and_target_truth(),
            fixture.expected.rows_have_path_and_target_truth,
            "{}: path/target truth mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.all_suspicious_rows_offer_safe_copy(),
            fixture.expected.safe_copy_visible_on_suspicious_rows,
            "{}: safe copy mismatch",
            fixture.case_name
        );
        assert!(
            packet.all_rows_offer_raw_plain_and_context_copy(),
            "{}: missing raw/plain/context copy choices",
            fixture.case_name
        );

        let representations = all_copy_representations(&packet);
        for required in &fixture.expected.required_copy_representations {
            assert!(
                representations.contains(required),
                "{}: missing copy representation {required}",
                fixture.case_name
            );
        }

        let warning_classes = packet
            .suspicious_class_tokens()
            .into_iter()
            .collect::<BTreeSet<_>>();
        for required in &fixture.expected.required_warning_classes {
            assert!(
                warning_classes.contains(required),
                "{}: missing warning class {required}",
                fixture.case_name
            );
        }
        if !fixture.expected.required_warning_classes.is_empty() {
            assert!(
                packet
                    .hunks
                    .iter()
                    .flat_map(|hunk| hunk.rows.iter())
                    .filter(|row| !row.suspicious_cues.is_empty())
                    .all(|row| row.escaped_text.contains("\\u{")),
                "{}: suspicious rows must expose escaped safe-copy text",
                fixture.case_name
            );
        }

        if let Some(expected) = fixture.expected.reopen {
            let selected_hunk_ref = packet
                .hunk_ref_at(expected.selected_hunk_index)
                .unwrap_or_else(|| panic!("{}: selected hunk missing", fixture.case_name))
                .to_string();
            let first_row_ref = packet
                .first_row_ref()
                .unwrap_or_else(|| panic!("{}: first row missing", fixture.case_name))
                .to_string();
            let closed = packet.close_for_reopen(
                DiffScrollAnchor {
                    first_visible_row_ref: first_row_ref,
                    scroll_offset: expected.scroll_offset,
                },
                Some(selected_hunk_ref.clone()),
                None,
                "2026-05-13T00:00:00Z",
            );
            let reopened = closed.reopen();

            assert_eq!(
                reopened.fallback_open_file_used, expected.fallback_open_file_used,
                "{}: fallback open-file mismatch",
                fixture.case_name
            );
            assert_eq!(
                reopened.restored_compare_target_ref == packet.compare_target.compare_target_ref,
                expected.restores_compare_target,
                "{}: compare target restore mismatch",
                fixture.case_name
            );
            assert_eq!(
                reopened.restored_path_truth_ref == packet.path_truth.path_truth_ref,
                expected.restores_path_truth,
                "{}: path truth restore mismatch",
                fixture.case_name
            );
            assert_eq!(reopened.restored_selected_hunk_ref, Some(selected_hunk_ref));
            assert_eq!(
                reopened.restored_scroll_anchor.scroll_offset,
                expected.scroll_offset
            );
        }
    }
}
