use std::fs;
use std::path::PathBuf;

use aureline_input::text_input::{ImeEvent, TextInputAction, TextInputSession, TextKeyEvent};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TextInputFixture {
    steps: Vec<TextInputStep>,
}

#[derive(Debug, Deserialize)]
struct TextInputStep {
    #[serde(flatten)]
    event: TextInputStepEvent,
    expected: Option<TextInputAction>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TextInputStepEvent {
    Ime { ime: ImeEvent },
    Key { key: TextKeyEvent },
}

#[test]
fn ime_and_dead_key_fixtures_stay_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/input/ime_cases");

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
        let fixture: TextInputFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        let mut session = TextInputSession::new();
        for (idx, step) in fixture.steps.into_iter().enumerate() {
            let actual = match step.event {
                TextInputStepEvent::Ime { ime } => session.handle_ime_event(ime),
                TextInputStepEvent::Key { key } => session.handle_key_event(&key),
            };

            assert_eq!(
                actual, step.expected,
                "step {idx} mismatch for {path:?}"
            );
        }
    }
}

