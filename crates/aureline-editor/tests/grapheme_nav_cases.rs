use std::fs;
use std::path::PathBuf;

use aureline_buffer::Buffer;
use aureline_editor::text_nav::{move_point_by_word, WordMotion};
use aureline_editor::{CaretMove, EditorAction, EditorViewport, EditorViewportSnapshot, TextPoint};
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
struct ExpectedFixture {
    text: String,
    snapshot: EditorViewportSnapshot,
}

#[derive(Debug, Deserialize)]
struct GraphemeNavCaseFixture {
    #[serde(rename = "__fixture__")]
    meta: Meta,
    document: DocumentFixture,
    initial: EditorViewportSnapshot,
    action: EditorAction,
    expected: ExpectedFixture,
}

fn line_graphemes_for_snapshot(snapshot: &aureline_buffer::Snapshot) -> Vec<usize> {
    let line_count = snapshot.line_count().max(1);
    (0..line_count)
        .map(|line| snapshot.grapheme_count_in_line(line).unwrap_or(0))
        .collect()
}

fn apply_word_move(
    snapshot: &aureline_buffer::Snapshot,
    viewport: &mut EditorViewport,
    movement: CaretMove,
    extend_selection: bool,
) -> bool {
    let before = viewport.caret();

    if !extend_selection {
        viewport.clear_selection();
    } else if viewport.selection_anchor().is_none() {
        viewport.set_selection_anchor(Some(before));
    }

    let direction = match movement {
        CaretMove::WordLeft => WordMotion::Left,
        CaretMove::WordRight => WordMotion::Right,
        _ => return false,
    };

    let Some(next) = move_point_by_word(snapshot, before, direction) else {
        return false;
    };

    if next == before {
        return false;
    }

    viewport.set_caret(next);
    true
}

fn apply_action(buffer: &mut Buffer, viewport: &mut EditorViewport, action: &EditorAction) {
    let snapshot = buffer.snapshot();
    let mut line_graphemes = line_graphemes_for_snapshot(&snapshot);
    if line_graphemes.is_empty() {
        line_graphemes.push(0);
    }

    match action {
        EditorAction::InsertText { text } => {
            let scope = aureline_editor::TextEditScope::AllCarets;
            let outcome = viewport
                .selections_mut()
                .apply_insert_text(buffer, &snapshot, text, "fixture_user_keystroke", scope)
                .expect("insert should succeed");
            assert!(outcome.is_some(), "expected insert to mutate buffer");
            viewport.set_ime_composition(None);
            let next_snapshot = buffer.snapshot();
            viewport.clamp_to_document(&line_graphemes_for_snapshot(&next_snapshot));
        }
        EditorAction::DeleteBackward => {
            let outcome = viewport
                .selections_mut()
                .apply_delete_backward(
                    buffer,
                    &snapshot,
                    "fixture_user_keystroke",
                    aureline_editor::TextEditScope::AllCarets,
                )
                .expect("delete_backward should succeed");
            assert!(
                outcome.is_some(),
                "expected delete_backward to mutate buffer"
            );
            viewport.set_ime_composition(None);
            let next_snapshot = buffer.snapshot();
            viewport.clamp_to_document(&line_graphemes_for_snapshot(&next_snapshot));
        }
        EditorAction::DeleteForward => {
            let outcome = viewport
                .selections_mut()
                .apply_delete_forward(
                    buffer,
                    &snapshot,
                    "fixture_user_keystroke",
                    aureline_editor::TextEditScope::AllCarets,
                )
                .expect("delete_forward should succeed");
            assert!(
                outcome.is_some(),
                "expected delete_forward to mutate buffer"
            );
            viewport.set_ime_composition(None);
            let next_snapshot = buffer.snapshot();
            viewport.clamp_to_document(&line_graphemes_for_snapshot(&next_snapshot));
        }
        EditorAction::MoveCaret {
            movement,
            extend_selection,
        } => {
            if matches!(movement, CaretMove::WordLeft | CaretMove::WordRight) {
                let _ = apply_word_move(&snapshot, viewport, *movement, *extend_selection);
            } else {
                let _ = viewport.move_caret(*movement, &line_graphemes, *extend_selection);
            }
        }
        EditorAction::ChangeSelection { delta } => {
            viewport.apply_selection_delta(*delta, &line_graphemes);
        }
        EditorAction::UpdateComposition { composition } => {
            viewport.set_ime_composition(Some(composition.clone()));
        }
        EditorAction::ClearComposition => {
            viewport.set_ime_composition(None);
        }
        EditorAction::ScrollLines { .. } | EditorAction::ScaleChange => {}
    }
}

#[test]
fn grapheme_nav_fixtures_stay_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/editor/grapheme_nav_cases");

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
        let fixture: GraphemeNavCaseFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        let mut buffer = Buffer::from_str(&fixture.document.text);

        let mut viewport = EditorViewport::new();
        viewport.set_caret(fixture.initial.caret);
        viewport.set_selection_anchor(fixture.initial.selection_anchor);
        viewport.set_ime_composition(fixture.initial.ime_composition.clone());
        viewport.clear_secondary_carets();
        for secondary in &fixture.initial.secondary_selections {
            assert!(
                secondary.selection_anchor.is_none(),
                "secondary selection anchors are not yet supported by this fixture harness: {:?}",
                path
            );
            viewport.add_secondary_caret(secondary.caret);
        }

        apply_action(&mut buffer, &mut viewport, &fixture.action);

        assert_eq!(
            buffer.contents(),
            fixture.expected.text.as_bytes(),
            "buffer mismatch for {:?} ({})",
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

        let _ = fixture.meta.name;
    }
}

#[test]
fn byte_offset_translation_roundtrips_grapheme_points() {
    let text = "a\u{0301} 👍🏽\n👩\u{200d}💻";
    let mut buffer = Buffer::from_str(text);
    let snapshot = buffer.snapshot();

    let points = [
        TextPoint {
            line: 0,
            grapheme: 0,
        },
        TextPoint {
            line: 0,
            grapheme: 1,
        },
        TextPoint {
            line: 0,
            grapheme: 2,
        },
        TextPoint {
            line: 0,
            grapheme: 3,
        },
        TextPoint {
            line: 1,
            grapheme: 0,
        },
        TextPoint {
            line: 1,
            grapheme: 1,
        },
    ];

    for point in points {
        let Some(offset) = aureline_editor::text_nav::byte_offset_for_text_point(&snapshot, point)
        else {
            panic!("expected {point:?} to map into snapshot");
        };
        let mapped = aureline_editor::text_nav::text_point_for_byte_offset(&snapshot, offset)
            .expect("expected offset to map back into snapshot");
        assert_eq!(mapped, point, "roundtrip mismatch for {point:?}");
    }
}

#[test]
fn grapheme_helpers_refuse_non_utf8_snapshots() {
    let mut buffer = Buffer::from_bytes(&[0xff, 0xfe, 0xfd]);
    let snapshot = buffer.snapshot();

    let origin = TextPoint {
        line: 0,
        grapheme: 0,
    };
    assert!(
        aureline_editor::text_nav::byte_offset_for_text_point(&snapshot, origin).is_none(),
        "expected non-utf8 snapshot to refuse byte offset translation"
    );
    assert!(
        aureline_editor::text_nav::text_point_for_byte_offset(&snapshot, 0).is_none(),
        "expected non-utf8 snapshot to refuse reverse translation"
    );
    assert!(
        move_point_by_word(&snapshot, origin, WordMotion::Right).is_none(),
        "expected non-utf8 snapshot to refuse word navigation"
    );
}
