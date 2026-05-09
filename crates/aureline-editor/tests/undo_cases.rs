use aureline_buffer::{Buffer, CompensationPosture, TransactionSpec, UndoClass};
use aureline_editor::undo::{next_redo, next_undo, originator};
use aureline_editor::{SelectionState, TextEditScope, TextPoint};

#[test]
fn next_undo_reports_typing_as_text_edit() {
    let mut buffer = Buffer::from_str("hello");
    let snapshot = buffer.snapshot();
    let mut selections = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 5,
    });

    selections
        .apply_insert_text(
            &mut buffer,
            &snapshot,
            "!",
            originator::USER_KEYSTROKE,
            TextEditScope::AllCarets,
        )
        .expect("typing edit should apply")
        .expect("typing edit should commit");

    let summary = next_undo(&buffer).expect("expected undo summary");
    assert_eq!(summary.class, UndoClass::TextEdit);
    assert_eq!(summary.class_id, "text_edit");
    assert_eq!(
        summary.compensation_posture,
        CompensationPosture::Compensatable
    );
    assert_eq!(summary.originator, originator::USER_KEYSTROKE);
    assert_eq!(summary.label, None);

    buffer.undo().expect("undo should succeed");
    let summary = next_redo(&buffer).expect("expected redo summary");
    assert_eq!(summary.class, UndoClass::TextEdit);
    assert_eq!(summary.originator, originator::USER_KEYSTROKE);
}

#[test]
fn next_undo_reports_paste_originator() {
    let mut buffer = Buffer::from_str("hello");
    let snapshot = buffer.snapshot();
    let mut selections = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 5,
    });

    selections
        .apply_insert_text(
            &mut buffer,
            &snapshot,
            " world",
            originator::PASTE,
            TextEditScope::AllCarets,
        )
        .expect("paste edit should apply")
        .expect("paste edit should commit");

    let summary = next_undo(&buffer).expect("expected undo summary");
    assert_eq!(summary.class, UndoClass::TextEdit);
    assert_eq!(summary.originator, originator::PASTE);
}

#[test]
fn multi_cursor_edits_get_class_and_originator_suffix() {
    let mut buffer = Buffer::from_str("hello");
    let snapshot = buffer.snapshot();
    let mut selections = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 0,
    });
    selections.add_secondary_caret(TextPoint {
        line: 0,
        grapheme: 5,
    });

    selections
        .apply_insert_text(
            &mut buffer,
            &snapshot,
            "!",
            originator::USER_KEYSTROKE,
            TextEditScope::AllCarets,
        )
        .expect("multi-cursor edit should apply")
        .expect("multi-cursor edit should commit");

    let summary = next_undo(&buffer).expect("expected undo summary");
    assert_eq!(summary.class, UndoClass::MultiCursorTextEdit);
    assert_eq!(summary.originator, "user_keystroke:multi_cursor");
}

#[test]
fn external_reload_is_undoable_and_reports_label() {
    let mut buffer = Buffer::from_str("hello");
    let len = buffer.len();
    let spec = TransactionSpec::new(
        UndoClass::ExternalReload,
        originator::EXTERNAL_CHANGE_RELOAD,
    )
    .with_label("Reloaded from disk");

    let mut tx = buffer.begin(spec).expect("reload tx should begin");
    tx.replace(0..len, "goodbye")
        .expect("reload tx should replace");
    tx.commit().expect("reload tx should commit");

    let summary = next_undo(&buffer).expect("expected undo summary");
    assert_eq!(summary.class, UndoClass::ExternalReload);
    assert_eq!(summary.originator, originator::EXTERNAL_CHANGE_RELOAD);
    assert_eq!(summary.label.as_deref(), Some("Reloaded from disk"));
    assert_eq!(summary.label_or_class_id(), "Reloaded from disk");
}
