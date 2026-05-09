use std::fs;
use std::path::PathBuf;

use aureline_buffer::Buffer;
use aureline_editor::clipboard::{
    plan_copy_default, plan_cut_default, ClipboardError, CopyPayload,
};
use aureline_editor::{SelectionState, TextPoint};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Meta {
    name: String,
    scenario: String,
}

#[derive(Debug, Deserialize)]
struct DocumentFixture {
    text: String,
}

#[derive(Debug, Deserialize)]
struct CaretSelectionFixture {
    caret: TextPoint,
    #[serde(default)]
    anchor: Option<TextPoint>,
}

#[derive(Debug, Deserialize)]
struct SelectionFixture {
    primary: CaretSelectionFixture,
    #[serde(default)]
    secondary: Vec<CaretSelectionFixture>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ClipboardOperationFixture {
    CopyDefault,
    CutDefault,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    payload: CopyPayload,
    #[serde(default)]
    after_cut_text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClipboardCaseFixture {
    #[serde(rename = "__fixture__")]
    meta: Meta,
    document: DocumentFixture,
    selection: SelectionFixture,
    operation: ClipboardOperationFixture,
    expected: ExpectedFixture,
}

fn selection_state_from_fixture(fixture: &SelectionFixture) -> SelectionState {
    let mut selections = SelectionState::new(fixture.primary.caret);
    selections.set_primary_anchor(fixture.primary.anchor);

    for secondary in &fixture.secondary {
        selections.add_secondary_caret(secondary.caret);
        if let Some(anchor) = secondary.anchor {
            if let Some(entry) = selections
                .secondary_mut()
                .iter_mut()
                .find(|entry| entry.caret() == secondary.caret)
            {
                entry.set_anchor(Some(anchor));
            }
        }
    }

    selections
}

#[test]
fn clipboard_cases_fixture_set_stays_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/editor/clipboard_cases");

    let mut fixture_paths: Vec<PathBuf> = fs::read_dir(&fixtures_dir)
        .expect("fixture directory must exist")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixture_paths.sort();

    assert!(
        !fixture_paths.is_empty(),
        "expected at least one fixture under {fixtures_dir:?}"
    );

    for path in fixture_paths {
        let raw = fs::read_to_string(&path).expect("fixture should be readable");
        let fixture: ClipboardCaseFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        let mut buffer = Buffer::from_str(&fixture.document.text);
        let snapshot = buffer.snapshot();
        let mut selections = selection_state_from_fixture(&fixture.selection);

        match fixture.operation {
            ClipboardOperationFixture::CopyDefault => {
                let payload = plan_copy_default(&snapshot, &selections).expect("copy should plan");
                assert_eq!(
                    payload, fixture.expected.payload,
                    "payload mismatch for {:?} ({})",
                    path, fixture.meta.scenario
                );
                assert!(
                    fixture.expected.after_cut_text.is_none(),
                    "copy fixtures must not define after_cut_text: {:?}",
                    path
                );
            }
            ClipboardOperationFixture::CutDefault => {
                let plan = plan_cut_default(&snapshot, &selections).expect("cut should plan");
                assert_eq!(
                    plan.payload, fixture.expected.payload,
                    "payload mismatch for {:?} ({})",
                    path, fixture.meta.scenario
                );

                let after = fixture
                    .expected
                    .after_cut_text
                    .as_deref()
                    .expect("cut fixtures must define after_cut_text");

                let outcome = selections
                    .apply_delete_byte_ranges(
                        &mut buffer,
                        &snapshot,
                        plan.delete_ranges,
                        "fixture_clipboard_cut",
                    )
                    .expect("planned delete should apply");
                assert!(
                    outcome.is_some(),
                    "expected planned cut to produce a committed edit for {:?}",
                    path
                );

                let contents = buffer.contents();
                let contents =
                    std::str::from_utf8(&contents).expect("fixture buffer must remain utf-8");
                assert_eq!(
                    contents, after,
                    "buffer mismatch for {:?} ({})",
                    path, fixture.meta.scenario
                );
            }
        }

        let _ = fixture.meta.name;
    }
}

#[test]
fn clipboard_planning_refuses_non_utf8_snapshots() {
    let mut buffer = Buffer::from_bytes(&[0xff, 0xfe, 0xfd]);
    let snapshot = buffer.snapshot();
    let selections = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 0,
    });

    let err = plan_copy_default(&snapshot, &selections).expect_err("expected error");
    assert_eq!(err, ClipboardError::NonUtf8Snapshot);
}

#[test]
fn clipboard_planning_reports_out_of_bounds_selections() {
    let mut buffer = Buffer::from_str("hello");
    let snapshot = buffer.snapshot();
    let mut selections = SelectionState::new(TextPoint {
        line: 99,
        grapheme: 0,
    });
    selections.set_primary_anchor(Some(TextPoint {
        line: 0,
        grapheme: 0,
    }));

    let err = plan_copy_default(&snapshot, &selections).expect_err("expected error");
    assert_eq!(err, ClipboardError::SelectionOutOfBounds);
}
