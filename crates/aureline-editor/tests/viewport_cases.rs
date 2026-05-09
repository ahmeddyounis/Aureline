use std::fs;
use std::path::PathBuf;

use aureline_editor::{EditorAction, EditorViewport, EditorViewportSnapshot};
use aureline_render::draw_queue::DamageRegion;
use aureline_render::PixelRect;
use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation as _;

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
struct ViewportFixture {
    width_px: u32,
    height_px: u32,
}

#[derive(Debug, Deserialize)]
struct ExpectedDamageFixture {
    layer_id: String,
    class_id: String,
    hook_id: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    snapshot: EditorViewportSnapshot,
    #[serde(default)]
    damage: Option<ExpectedDamageFixture>,
}

#[derive(Debug, Deserialize)]
struct ViewportCaseFixture {
    #[serde(rename = "__fixture__")]
    meta: Meta,
    document: DocumentFixture,
    viewport: ViewportFixture,
    initial: EditorViewportSnapshot,
    action: EditorAction,
    expected: ExpectedFixture,
}

#[test]
fn viewport_cases_fixture_set_stays_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/editor/viewport_cases");

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
        let fixture: ViewportCaseFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        let mut viewport = EditorViewport::new();
        viewport.set_caret(fixture.initial.caret);
        viewport.set_selection_anchor(fixture.initial.selection_anchor);
        viewport.set_ime_composition(fixture.initial.ime_composition.clone());

        let mut lines: Vec<String> = fixture
            .document
            .text
            .split('\n')
            .map(|s| s.to_string())
            .collect();
        if lines.is_empty() {
            lines.push(String::new());
        }
        let line_graphemes: Vec<usize> = lines
            .iter()
            .map(|line| line.graphemes(true).count())
            .collect();
        let max_scroll_line = lines.len().saturating_sub(1);

        let _ = viewport.scroll_by_lines(fixture.initial.scroll_line as i32, max_scroll_line);

        match &fixture.action {
            EditorAction::MoveCaret {
                movement,
                extend_selection,
            } => {
                let _ = viewport.move_caret(*movement, &line_graphemes, *extend_selection);
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
            EditorAction::InsertText { .. }
            | EditorAction::DeleteBackward
            | EditorAction::DeleteForward => {}
            EditorAction::ScrollLines { .. } | EditorAction::ScaleChange => {}
        }

        let viewport_rect =
            PixelRect::new(0, 0, fixture.viewport.width_px, fixture.viewport.height_px);
        let damage = viewport.apply_action(&fixture.action, viewport_rect, max_scroll_line);
        match (&fixture.expected.damage, damage) {
            (None, None) => {}
            (Some(expected), Some(damage)) => {
                assert_eq!(
                    damage.event.layer.id(),
                    expected.layer_id,
                    "layer id mismatch for {:?}",
                    path
                );
                assert_eq!(
                    damage.event.class.id(),
                    expected.class_id,
                    "class id mismatch for {:?}",
                    path
                );
                assert_eq!(
                    damage.hook.id(),
                    expected.hook_id,
                    "hook id mismatch for {:?}",
                    path
                );

                match damage.event.region {
                    DamageRegion::Rect(rect) => assert_eq!(
                        rect, viewport_rect,
                        "expected viewport-bounded damage region for {:?}",
                        path
                    ),
                    DamageRegion::Unspecified => {
                        panic!("expected concrete damage region for {:?}", path)
                    }
                }
            }
            (None, Some(_)) => panic!("expected action to be a no-op (no damage) for {:?}", path),
            (Some(_), None) => panic!("expected fixture action to produce damage for {:?}", path),
        }

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
