use std::fs;
use std::path::PathBuf;

use aureline_buffer::Buffer;
use aureline_editor::{
    EditorAction, EditorViewport, EditorViewportSnapshot, SecondarySelectionSnapshot, TextEditScope,
};
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
struct ExpectedCommittedFixture {
    class_id: String,
    operation_count: usize,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    text: String,
    committed: ExpectedCommittedFixture,
    snapshot: EditorViewportSnapshot,
}

#[derive(Debug, Deserialize)]
struct MulticursorCaseFixture {
    #[serde(rename = "__fixture__")]
    meta: Meta,
    document: DocumentFixture,
    initial: EditorViewportSnapshot,
    action: EditorAction,
    expected: ExpectedFixture,
}

#[test]
fn multicursor_cases_fixture_set_stays_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/editor/multicursor_cases");

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
        let fixture: MulticursorCaseFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        let before = fixture.document.text.clone();
        let expected = fixture.expected.text.clone();

        let mut buffer = Buffer::from_str(&fixture.document.text);
        let snapshot = buffer.snapshot();

        let mut viewport = EditorViewport::new();
        viewport.set_caret(fixture.initial.caret);
        viewport.set_selection_anchor(fixture.initial.selection_anchor);
        viewport.set_ime_composition(fixture.initial.ime_composition.clone());
        viewport.clear_secondary_carets();
        for SecondarySelectionSnapshot {
            caret,
            selection_anchor,
        } in &fixture.initial.secondary_selections
        {
            assert!(
                selection_anchor.is_none(),
                "secondary selection anchors are not yet supported by this fixture harness: {:?}",
                path
            );
            viewport.add_secondary_caret(*caret);
        }

        let scope_for_insert = if viewport.caret_count() > 1 && viewport.ime_composition().is_some()
        {
            TextEditScope::PrimaryOnly
        } else {
            TextEditScope::AllCarets
        };

        let outcome = match &fixture.action {
            EditorAction::InsertText { text } => viewport
                .selections_mut()
                .apply_insert_text(&mut buffer, &snapshot, text, "user_keystroke", scope_for_insert)
                .expect("insert should succeed"),
            EditorAction::DeleteBackward => viewport
                .selections_mut()
                .apply_delete_backward(
                    &mut buffer,
                    &snapshot,
                    "user_keystroke",
                    TextEditScope::AllCarets,
                )
                .expect("delete_backward should succeed"),
            _ => panic!(
                "unsupported action in {:?}: {:?}",
                path, fixture.action
            ),
        };

        let Some(outcome) = outcome else {
            panic!(
                "expected fixture action to mutate the buffer for {:?} ({})",
                path, fixture.meta.scenario
            );
        };

        viewport.set_ime_composition(None);

        assert_eq!(
            outcome.committed.class_id,
            fixture.expected.committed.class_id,
            "class_id mismatch for {:?}",
            path
        );
        assert_eq!(
            outcome.committed.operation_count,
            fixture.expected.committed.operation_count,
            "operation_count mismatch for {:?}",
            path
        );

        assert_eq!(
            buffer.contents(),
            expected.as_bytes(),
            "buffer contents mismatch for {:?} ({})",
            path,
            fixture.meta.scenario
        );

        assert_eq!(
            viewport.snapshot(),
            fixture.expected.snapshot,
            "viewport snapshot mismatch for {:?} ({})",
            path,
            fixture.meta.scenario
        );

        assert_eq!(buffer.journal_len(), 1, "expected one undo group for {:?}", path);
        buffer.undo().expect("undo should exist");
        assert_eq!(
            buffer.contents(),
            before.as_bytes(),
            "undo mismatch for {:?} ({})",
            path,
            fixture.meta.scenario
        );
        buffer.redo().expect("redo should exist");
        assert_eq!(
            buffer.contents(),
            expected.as_bytes(),
            "redo mismatch for {:?} ({})",
            path,
            fixture.meta.scenario
        );

        let _ = fixture.meta.name;
    }
}

