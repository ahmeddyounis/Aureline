use std::fs;
use std::path::PathBuf;

use aureline_buffer::Buffer;
use aureline_editor::find_replace::FindReplaceDegradedReason;
use aureline_editor::{
    FindOptions, FindReplaceError, FindReplaceMode, FindReplaceState, TextPoint,
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
struct BudgetsFixture {
    max_scan_bytes: usize,
    max_match_count: usize,
}

#[derive(Debug, Deserialize)]
struct InitialFixture {
    mode: FindReplaceMode,
    query: String,
    #[serde(default)]
    replacement: String,
    #[serde(default)]
    options: FindOptions,
    caret: TextPoint,
    #[serde(default)]
    budgets: Option<BudgetsFixture>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum FindReplaceActionFixture {
    Sync,
    SelectNext,
    SelectPrev,
    ReplaceActive,
    ReplaceAll,
}

#[derive(Debug, Deserialize)]
struct HighlightSpanFixture {
    start: TextPoint,
    end: TextPoint,
}

#[derive(Debug, Deserialize)]
struct HighlightsFixture {
    matches: Vec<HighlightSpanFixture>,
    #[serde(default)]
    active_match: Option<HighlightSpanFixture>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct ByteRangeFixture {
    start: usize,
    end: usize,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    match_count: usize,
    #[serde(default)]
    active_match_index: Option<usize>,
    #[serde(default)]
    active_match_range: Option<ByteRangeFixture>,
    #[serde(default)]
    highlights: Option<HighlightsFixture>,
    #[serde(default)]
    degraded_reason: Option<FindReplaceDegradedReason>,
    #[serde(default)]
    after_text: Option<String>,
    #[serde(default)]
    replaced_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct FindReplaceCaseFixture {
    #[serde(rename = "__fixture__")]
    meta: Meta,
    document: DocumentFixture,
    initial: InitialFixture,
    action: FindReplaceActionFixture,
    expected: ExpectedFixture,
}

fn apply_fixture_options(state: &mut FindReplaceState, desired: FindOptions) {
    let current = state.options();
    if current.case_sensitive != desired.case_sensitive {
        state.toggle_case_sensitive();
    }
    let current = state.options();
    if current.whole_word != desired.whole_word {
        state.toggle_whole_word();
    }
}

fn byte_range(state: &FindReplaceState) -> Option<ByteRangeFixture> {
    let range = state.active_match_range()?;
    Some(ByteRangeFixture {
        start: range.start,
        end: range.end,
    })
}

#[test]
fn find_replace_cases_fixture_set_stays_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/editor/find_replace_cases");

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
        let fixture: FindReplaceCaseFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        let before_text = fixture.document.text.clone();
        let mut buffer = Buffer::from_str(&fixture.document.text);
        let mut snapshot = buffer.snapshot();

        let mut state = FindReplaceState::new();
        if let Some(budgets) = fixture.initial.budgets.as_ref() {
            state.configure_budgets(budgets.max_scan_bytes, budgets.max_match_count);
        }

        state.set_mode(fixture.initial.mode);
        state.set_query(fixture.initial.query.clone());
        state.set_replacement(fixture.initial.replacement.clone());
        apply_fixture_options(&mut state, fixture.initial.options);

        let caret = fixture.initial.caret;

        match fixture.action {
            FindReplaceActionFixture::Sync => {
                state
                    .sync_for_view(&snapshot, caret)
                    .expect("fixture sync must succeed");
            }
            FindReplaceActionFixture::SelectNext => {
                let _ = state
                    .select_next(&snapshot, caret)
                    .expect("fixture select_next must succeed");
            }
            FindReplaceActionFixture::SelectPrev => {
                let _ = state
                    .select_prev(&snapshot, caret)
                    .expect("fixture select_prev must succeed");
            }
            FindReplaceActionFixture::ReplaceActive => {
                let outcome = state
                    .replace_active(
                        &mut buffer,
                        &snapshot,
                        caret,
                        "fixture:editor.find_replace.replace_active",
                    )
                    .expect("fixture replace_active must succeed");

                if let Some(outcome) = outcome {
                    snapshot = outcome.snapshot;
                    if let Some(expected) = fixture.expected.replaced_count {
                        assert_eq!(
                            outcome.replaced_count, expected,
                            "replaced_count mismatch for {:?} ({})",
                            path, fixture.meta.scenario
                        );
                    }
                }

                state
                    .sync_for_view(&snapshot, caret)
                    .expect("fixture sync after replace_active must succeed");
            }
            FindReplaceActionFixture::ReplaceAll => {
                let outcome = state
                    .replace_all(
                        &mut buffer,
                        &snapshot,
                        caret,
                        "fixture:editor.find_replace.replace_all",
                    )
                    .expect("fixture replace_all must succeed");

                if let Some(outcome) = outcome {
                    snapshot = outcome.snapshot;
                    if let Some(expected) = fixture.expected.replaced_count {
                        assert_eq!(
                            outcome.replaced_count, expected,
                            "replaced_count mismatch for {:?} ({})",
                            path, fixture.meta.scenario
                        );
                    }
                }

                state
                    .sync_for_view(&snapshot, caret)
                    .expect("fixture sync after replace_all must succeed");
            }
        }

        if let Some(expected) = fixture.expected.after_text.as_deref() {
            let contents = buffer.contents();
            let contents = std::str::from_utf8(&contents).expect("fixture buffer must be utf-8");
            assert_eq!(
                contents, expected,
                "buffer text mismatch for {:?} ({})",
                path, fixture.meta.scenario
            );

            assert_eq!(
                buffer.journal_len(),
                1,
                "expected one undo group for {:?} ({})",
                path,
                fixture.meta.scenario
            );

            buffer.undo().expect("undo must exist");
            let contents = std::str::from_utf8(&buffer.contents())
                .expect("undo snapshot must remain utf-8")
                .to_string();
            assert_eq!(
                contents, before_text,
                "undo mismatch for {:?} ({})",
                path, fixture.meta.scenario
            );
            buffer.redo().expect("redo must exist");
            let contents = std::str::from_utf8(&buffer.contents())
                .expect("redo snapshot must remain utf-8")
                .to_string();
            assert_eq!(
                contents, expected,
                "redo mismatch for {:?} ({})",
                path, fixture.meta.scenario
            );
        }

        assert_eq!(
            state.match_count(),
            fixture.expected.match_count,
            "match_count mismatch for {:?} ({})",
            path,
            fixture.meta.scenario
        );
        assert_eq!(
            state.active_match_index(),
            fixture.expected.active_match_index,
            "active_match_index mismatch for {:?} ({})",
            path,
            fixture.meta.scenario
        );

        assert_eq!(
            byte_range(&state),
            fixture.expected.active_match_range,
            "active_match_range mismatch for {:?} ({})",
            path,
            fixture.meta.scenario
        );

        assert_eq!(
            state.degraded_reason(),
            fixture.expected.degraded_reason.as_ref(),
            "degraded_reason mismatch for {:?} ({})",
            path,
            fixture.meta.scenario
        );

        match (&fixture.expected.highlights, state.highlight_overlays()) {
            (None, None) => {}
            (Some(expected), Some(actual)) => {
                let expected_matches: Vec<_> = expected
                    .matches
                    .iter()
                    .map(|span| aureline_editor::HighlightSpan {
                        start: span.start,
                        end: span.end,
                    })
                    .collect();
                assert_eq!(
                    actual.matches, expected_matches,
                    "highlight matches mismatch for {:?} ({})",
                    path, fixture.meta.scenario
                );

                let expected_active =
                    expected
                        .active_match
                        .as_ref()
                        .map(|span| aureline_editor::HighlightSpan {
                            start: span.start,
                            end: span.end,
                        });
                assert_eq!(
                    actual.active_match, expected_active,
                    "active highlight mismatch for {:?} ({})",
                    path, fixture.meta.scenario
                );
            }
            (None, Some(_)) => panic!(
                "expected no highlights but state returned overlays for {:?} ({})",
                path, fixture.meta.scenario
            ),
            (Some(_), None) => panic!(
                "expected highlights but state returned none for {:?} ({})",
                path, fixture.meta.scenario
            ),
        }

        let _ = fixture.meta.name;
    }
}

#[test]
fn find_replace_sync_reports_non_utf8_snapshots() {
    let mut buffer = Buffer::from_bytes(&[0xff, 0xfe, 0xfd]);
    let snapshot = buffer.snapshot();

    let mut state = FindReplaceState::new();
    state.set_mode(FindReplaceMode::Find);
    state.set_query("foo");

    let err = state
        .sync_for_view(
            &snapshot,
            TextPoint {
                line: 0,
                grapheme: 0,
            },
        )
        .expect_err("expected error for non-utf8 snapshot");

    assert_eq!(err, FindReplaceError::NonUtf8Snapshot);
    assert_eq!(
        state.degraded_reason(),
        Some(&FindReplaceDegradedReason::NonUtf8Snapshot)
    );
    assert_eq!(state.match_count(), 0);
    assert!(state.highlight_overlays().is_none());
}

#[test]
fn find_replace_blocks_replace_all_when_match_budget_exceeded() {
    let mut buffer = Buffer::from_str("foo foo foo");
    let snapshot = buffer.snapshot();

    let mut state = FindReplaceState::new();
    state.configure_budgets(64, 2);
    state.set_mode(FindReplaceMode::Replace);
    state.set_query("foo");
    state.set_replacement("bar");
    state
        .sync_for_view(
            &snapshot,
            TextPoint {
                line: 0,
                grapheme: 0,
            },
        )
        .expect("sync must succeed");

    assert_eq!(
        state.degraded_reason(),
        Some(&FindReplaceDegradedReason::MatchBudgetExceeded { match_cap: 2 })
    );

    let err = state
        .replace_all(
            &mut buffer,
            &snapshot,
            TextPoint {
                line: 0,
                grapheme: 0,
            },
            "fixture:editor.find_replace.replace_all_blocked",
        )
        .expect_err("replace_all must refuse limited match set");

    assert_eq!(err, FindReplaceError::LimitedMatchSet);
}
