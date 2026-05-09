use super::*;

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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
struct EditFixture {
    offset: usize,
    insert: String,
    expected_text: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    caret_offset: usize,
}

#[derive(Debug, Deserialize)]
struct CloseReopenCase {
    edit: EditFixture,
    expected: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct SplitMoveCase {
    edit: EditFixture,
    expected: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct RestoreHandoffCase {
    edit: EditFixture,
    expected: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "case", rename_all = "snake_case")]
enum ContinuityCase {
    CloseReopen(CloseReopenCase),
    SplitMove(SplitMoveCase),
    RestoreHandoff(RestoreHandoffCase),
}

#[derive(Debug, Deserialize)]
struct ContinuityFixture {
    #[serde(rename = "__fixture__")]
    meta: Meta,
    document: DocumentFixture,
    #[serde(flatten)]
    case: ContinuityCase,
}

#[test]
fn editor_continuity_fixtures_stay_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/editor/continuity_cases");

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

    for fixture_path in fixture_paths {
        let raw = fs::read_to_string(&fixture_path).expect("fixture should be readable");
        let fixture: ContinuityFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        match &fixture.case {
            ContinuityCase::CloseReopen(case) => run_close_reopen_case(&fixture, case),
            ContinuityCase::SplitMove(case) => run_split_move_case(&fixture, case),
            ContinuityCase::RestoreHandoff(case) => run_restore_handoff_case(&fixture, case),
        }

        let _ = fixture.meta.name;
    }
}

fn run_close_reopen_case(fixture: &ContinuityFixture, case: &CloseReopenCase) {
    let tmp_path = write_temp_file(&fixture.meta.name, &fixture.document.text, false);

    let mut frame = DesktopFrame::new(1280, 720);
    let group = frame.focused_editor_group();
    let tab1 = frame.open_tab().expect("open first tab");
    let tab2 = frame.open_tab().expect("open second tab");

    let mut editor_runtime = EditorWorkspaceRuntimeState::new();
    editor_runtime
        .open_file(group, tab1, &tmp_path)
        .expect("open file in tab1");
    editor_runtime
        .open_file(group, tab2, &tmp_path)
        .expect("open file in tab2");

    let viewport_rect = PixelRect::new(0, 0, 800, 600);
    apply_insert_action(
        &mut editor_runtime,
        group,
        tab1,
        &case.edit.insert,
        case.edit.offset,
        viewport_rect,
    );

    let expected_caret_offset = {
        let session = editor_runtime
            .tab_session_mut(group, tab1)
            .expect("tab1 session");
        caret_offset(session)
    };

    {
        let session = editor_runtime
            .tab_session_mut(group, tab2)
            .expect("tab2 session");
        session.ensure_fresh_snapshot();
        assert_eq!(
            session.snapshot.as_str().unwrap_or_default(),
            case.edit.expected_text.as_str(),
            "expected shared buffer content to reach tab2 ({})",
            fixture.meta.scenario
        );
    }

    frame.set_active_tab(group, tab1);
    let closed = frame.close_active_tab(group).expect("close tab1");
    assert_eq!(closed, tab1);
    editor_runtime.close_tab(group, tab1);

    let tab3 = frame.open_tab().expect("open third tab");
    editor_runtime
        .open_file(group, tab3, &tmp_path)
        .expect("reopen file in tab3");

    let auth2 = editor_runtime
        .tab_session_mut(group, tab2)
        .expect("tab2 session")
        .authority
        .clone();
    let auth3 = editor_runtime
        .tab_session_mut(group, tab3)
        .expect("tab3 session")
        .authority
        .clone();
    assert!(
        Rc::ptr_eq(&auth2, &auth3),
        "expected reopen to preserve one shared buffer authority ({})",
        fixture.meta.scenario
    );

    let info2 = editor_runtime
        .tab_render_info(group, tab2)
        .expect("tab2 render info");
    let info3 = editor_runtime
        .tab_render_info(group, tab3)
        .expect("tab3 render info");
    assert!(info2.dirty, "expected tab2 to be Modified after reopen");
    assert!(info3.dirty, "expected tab3 to be Modified after reopen");

    {
        let session = editor_runtime
            .tab_session_mut(group, tab3)
            .expect("tab3 session");
        session.ensure_fresh_snapshot();
        assert_eq!(
            session.snapshot.as_str().unwrap_or_default(),
            case.edit.expected_text.as_str(),
            "expected reopened tab to surface shared buffer content ({})",
            fixture.meta.scenario
        );
        assert_eq!(
            caret_offset(session),
            case.expected.caret_offset,
            "expected caret continuity across reopen ({})",
            fixture.meta.scenario
        );
        assert_eq!(
            expected_caret_offset, case.expected.caret_offset,
            "expected fixture caret offset to match observed state ({})",
            fixture.meta.scenario
        );
    }

    cleanup_temp_file(&tmp_path);
}

fn run_split_move_case(fixture: &ContinuityFixture, case: &SplitMoveCase) {
    let tmp_path = write_temp_file(&fixture.meta.name, &fixture.document.text, false);

    let mut frame = DesktopFrame::new(1280, 720);
    let source_group = frame.focused_editor_group();
    let source_tab = frame.open_tab().expect("open source tab");

    let mut editor_runtime = EditorWorkspaceRuntimeState::new();
    editor_runtime
        .open_file(source_group, source_tab, &tmp_path)
        .expect("open file in source tab");

    let viewport_rect = PixelRect::new(0, 0, 800, 600);
    apply_insert_action(
        &mut editor_runtime,
        source_group,
        source_tab,
        &case.edit.insert,
        case.edit.offset,
        viewport_rect,
    );

    let auth_before = editor_runtime
        .tab_session_mut(source_group, source_tab)
        .expect("source session")
        .authority
        .clone();
    let caret_before = {
        let session = editor_runtime
            .tab_session_mut(source_group, source_tab)
            .expect("source session");
        caret_offset(session)
    };
    assert_eq!(
        caret_before, case.expected.caret_offset,
        "expected fixture caret offset to match source view ({})",
        fixture.meta.scenario
    );

    let new_group = match frame.request_split_focused_editor_group() {
        NewEditorGroupOutcome::Created { new_group } => new_group,
        NewEditorGroupOutcome::WouldViolateMinimum(_) => {
            panic!("split should succeed in test frame")
        }
    };
    let moved_tab = frame.open_tab_in_group(new_group).expect("open moved tab");
    assert!(
        editor_runtime.clone_tab_view(source_group, source_tab, new_group, moved_tab),
        "clone_tab_view should succeed ({})",
        fixture.meta.scenario
    );

    frame.set_active_tab(source_group, source_tab);
    let closed = frame
        .close_active_tab(source_group)
        .expect("close source tab");
    assert_eq!(closed, source_tab);
    editor_runtime.close_tab(source_group, source_tab);

    let moved_auth = editor_runtime
        .tab_session_mut(new_group, moved_tab)
        .expect("moved session")
        .authority
        .clone();
    assert!(
        Rc::ptr_eq(&auth_before, &moved_auth),
        "expected split move to preserve one shared buffer authority ({})",
        fixture.meta.scenario
    );

    let moved_info = editor_runtime
        .tab_render_info(new_group, moved_tab)
        .expect("moved render info");
    assert!(moved_info.dirty, "expected moved tab to remain Modified");

    let moved_caret = {
        let session = editor_runtime
            .tab_session_mut(new_group, moved_tab)
            .expect("moved session");
        caret_offset(session)
    };
    assert_eq!(
        moved_caret, case.expected.caret_offset,
        "expected caret continuity across split move ({})",
        fixture.meta.scenario
    );

    cleanup_temp_file(&tmp_path);
}

fn run_restore_handoff_case(fixture: &ContinuityFixture, case: &RestoreHandoffCase) {
    let tmp_path = write_temp_file(&fixture.meta.name, &fixture.document.text, false);

    let mut frame = DesktopFrame::new(1280, 720);
    let group = frame.focused_editor_group();
    let tab1 = frame.open_tab().expect("open tab1");

    let mut editor_runtime = EditorWorkspaceRuntimeState::new();
    editor_runtime
        .open_file(group, tab1, &tmp_path)
        .expect("open file in tab1");

    let viewport_rect = PixelRect::new(0, 0, 800, 600);
    apply_insert_action(
        &mut editor_runtime,
        group,
        tab1,
        &case.edit.insert,
        case.edit.offset,
        viewport_rect,
    );

    frame.set_active_tab(group, tab1);
    let closed = frame.close_active_tab(group).expect("close tab1");
    assert_eq!(closed, tab1);
    editor_runtime.close_tab(group, tab1);

    let buffers = editor_runtime.take_buffer_store();
    let mut restored_runtime = EditorWorkspaceRuntimeState::with_buffer_store(buffers);

    let tab2 = frame.open_tab().expect("open tab2");
    restored_runtime
        .open_file(group, tab2, &tmp_path)
        .expect("reopen file after handoff");

    let restored_info = restored_runtime
        .tab_render_info(group, tab2)
        .expect("render info");
    assert!(
        restored_info.dirty,
        "expected dirty buffer continuity across restore handoff ({})",
        fixture.meta.scenario
    );

    let restored_session = restored_runtime
        .tab_session_mut(group, tab2)
        .expect("restored session");
    restored_session.ensure_fresh_snapshot();
    assert_eq!(
        restored_session.snapshot.as_str().unwrap_or_default(),
        case.edit.expected_text.as_str(),
        "expected restored buffer contents to match ({})",
        fixture.meta.scenario
    );
    assert_eq!(
        caret_offset(restored_session),
        case.expected.caret_offset,
        "expected caret continuity across restore handoff ({})",
        fixture.meta.scenario
    );

    cleanup_temp_file(&tmp_path);
}

fn apply_insert_action(
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    group: PaneId,
    tab: EditorTabId,
    text: &str,
    offset: usize,
    viewport_rect: PixelRect,
) {
    let session = editor_runtime
        .tab_session_mut(group, tab)
        .expect("tab session");
    let (line, grapheme) = session
        .snapshot
        .line_grapheme_for_byte_offset(offset)
        .expect("offset should map into document");
    session
        .viewport
        .set_caret(aureline_editor::TextPoint { line, grapheme });
    session.viewport.clear_selection();
    let _ = session.apply_action(
        &EditorAction::InsertText {
            text: text.to_string(),
        },
        viewport_rect,
    );
}

fn caret_offset(session: &mut EditorTabSession) -> usize {
    session.ensure_fresh_snapshot();
    let caret = session.viewport.caret();
    session
        .snapshot
        .byte_offset_for_line_grapheme(caret.line, caret.grapheme)
        .unwrap_or_default()
}

fn write_temp_file(name: &str, text: &str, read_only: bool) -> PathBuf {
    let tmp_dir = std::env::temp_dir();
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let tmp_path = tmp_dir.join(format!("aureline_continuity_{}_{}.txt", name, suffix));
    fs::write(&tmp_path, text.as_bytes()).expect("write temp file");
    if read_only {
        let mut perms = fs::metadata(&tmp_path)
            .expect("read temp metadata")
            .permissions();
        perms.set_readonly(true);
        fs::set_permissions(&tmp_path, perms).expect("set read-only permissions");
    }
    tmp_path
}

fn cleanup_temp_file(path: &Path) {
    let _ = fs::remove_file(path);
}
