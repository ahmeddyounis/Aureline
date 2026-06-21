//! Deterministic seed for the dictation-edit-parity lane.
//!
//! The seed runs the real [`DictationCaptureSession`] over throwaway buffers so
//! the checked-in fixtures, the published companion doc, and any surface that
//! ingests dictation parity are minted from the same wiring the editor uses at
//! runtime. Every id, label, and seed string is stable, and every buffer is
//! fresh, so the deterministic undo-group counter keeps the fixtures bit-for-bit
//! equal across regenerations.

use aureline_buffer::Buffer;
use aureline_history::voice_groups::ORDINARY_TEXT_EDIT_UNDO_CLASS_IDS;

use crate::selection::SelectionState;
use crate::viewport::TextPoint;

use super::{
    CaptureStatus, CorrectionGesture, DictationCaptureSession, DictationEditParityPacket,
    DictationError, DictationIntent, DictationParityScenario, DictationRecognitionLocality,
    DictationScenarioOutcomeClass, DictationSurface, DictationSurfaceClass,
    DictationSurfaceCoverageRow, DictationSurfaceSupport, FormattingIntent, PunctuationMark,
    UndoRedoRoundtrip,
};

fn surface(
    surface_class: DictationSurfaceClass,
    surface_id: &str,
    support: DictationSurfaceSupport,
    support_reason_ref: &str,
    accessibility_label_ref: &str,
) -> DictationSurface {
    DictationSurface {
        surface_class,
        surface_id: surface_id.to_owned(),
        support,
        support_reason_ref: support_reason_ref.to_owned(),
        accessibility_label_ref: accessibility_label_ref.to_owned(),
    }
}

fn coverage_row(surface: &DictationSurface) -> DictationSurfaceCoverageRow {
    DictationSurfaceCoverageRow {
        surface_class: surface.surface_class,
        surface_id: surface.surface_id.clone(),
        support: surface.support,
        support_reason_ref: surface.support_reason_ref.clone(),
        accessibility_label_ref: surface.accessibility_label_ref.clone(),
    }
}

fn buffer_text(buffer: &Buffer) -> String {
    String::from_utf8(buffer.contents()).expect("seed buffer is utf-8")
}

fn selection_at_end(buffer: &mut Buffer) -> SelectionState {
    let snapshot = buffer.snapshot();
    let last_line = snapshot.line_count().saturating_sub(1);
    let grapheme = snapshot.grapheme_count_in_line(last_line).unwrap_or(0);
    SelectionState::new(TextPoint {
        line: last_line,
        grapheme,
    })
}

fn undo_n(buffer: &mut Buffer, n: usize) {
    for _ in 0..n {
        let _ = buffer.undo();
    }
}

fn redo_n(buffer: &mut Buffer, n: usize) {
    for _ in 0..n {
        let _ = buffer.redo();
    }
}

/// Runs an all-applied scenario: every intent is expected to commit or move the
/// shared undo/redo stack, then the buffer is fully undone and redone to prove
/// predictable grouped history.
fn applied_scenario(
    scenario_id: &str,
    surface: &DictationSurface,
    locality: DictationRecognitionLocality,
    seed_text: &str,
    intents: &[DictationIntent],
    note_ref: &str,
    with_history_group: bool,
) -> DictationParityScenario {
    let mut buffer = Buffer::from_str(seed_text);
    let mut selection = selection_at_end(&mut buffer);
    let mut session = DictationCaptureSession::begin(scenario_id, surface.clone(), locality);

    for intent in intents {
        session
            .apply(&mut buffer, &mut selection, intent)
            .expect("seed intent applies");
    }

    let final_text = buffer_text(&buffer);
    let depth = buffer.journal_len();
    let content_edits = session
        .records()
        .iter()
        .filter(|record| record.effect.is_content_edit())
        .count() as u32;
    let all_groups_text_edit_class = session.records().iter().all(|record| {
        record.undo_group_id.is_none()
            || ORDINARY_TEXT_EDIT_UNDO_CLASS_IDS.contains(&record.undo_class_id.as_str())
    });

    undo_n(&mut buffer, depth);
    let returns_to_seed_after_full_undo = buffer_text(&buffer) == seed_text;
    redo_n(&mut buffer, depth);
    let returns_to_final_after_full_redo = buffer_text(&buffer) == final_text;

    let history_group = if with_history_group {
        session.history_group()
    } else {
        None
    };

    DictationParityScenario {
        scenario_id: scenario_id.to_owned(),
        surface_id: surface.surface_id.clone(),
        surface_class: surface.surface_class,
        support: surface.support,
        outcome_class: DictationScenarioOutcomeClass::Applied,
        edits: session.records().to_vec(),
        history_group,
        undo_redo: Some(UndoRedoRoundtrip {
            content_edits,
            returns_to_seed_after_full_undo,
            returns_to_final_after_full_redo,
            all_groups_text_edit_class,
        }),
        restore_point_preserved: None,
        explicit_rejection: false,
        note_ref: note_ref.to_owned(),
    }
}

/// Runs a cancel scenario: an in-flight interim is discarded and the prior
/// insertion point is restored, with no buffer mutation.
fn cancel_scenario(surface: &DictationSurface) -> DictationParityScenario {
    let scenario_id = "dictation:scenario:cancel_restores_insertion_point";
    let mut buffer = Buffer::from_str("oldName");
    let mut selection = selection_at_end(&mut buffer);
    let insertion_point = selection.clone();
    let mut session = DictationCaptureSession::begin(
        scenario_id,
        surface.clone(),
        DictationRecognitionLocality::OnDeviceLocal,
    );

    session.set_interim("newName", &selection);
    // The recognizer previews a different caret while the interim is in flight.
    selection.set_primary_caret(TextPoint {
        line: 0,
        grapheme: 3,
    });
    let _summary = session.cancel(&mut selection);

    let restored = selection == insertion_point
        && session.status() == CaptureStatus::Cancelled
        && buffer_text(&buffer) == "oldName";

    DictationParityScenario {
        scenario_id: scenario_id.to_owned(),
        surface_id: surface.surface_id.clone(),
        surface_class: surface.surface_class,
        support: surface.support,
        outcome_class: DictationScenarioOutcomeClass::CancelledRestored,
        edits: session.records().to_vec(),
        history_group: None,
        undo_redo: None,
        restore_point_preserved: Some(restored),
        explicit_rejection: false,
        note_ref: "label:dictation:scenario_cancel_restores".to_owned(),
    }
}

/// Runs an unsupported-surface scenario: dictation is explicitly rejected and
/// nothing is applied.
fn unsupported_scenario(surface: &DictationSurface) -> DictationParityScenario {
    let scenario_id = "dictation:scenario:terminal_unsupported";
    let mut buffer = Buffer::from_str("");
    let mut selection = selection_at_end(&mut buffer);
    let mut session = DictationCaptureSession::begin(
        scenario_id,
        surface.clone(),
        DictationRecognitionLocality::OnDeviceLocal,
    );

    let result = session.apply(
        &mut buffer,
        &mut selection,
        &DictationIntent::InsertText {
            text: "list files".to_owned(),
        },
    );
    let explicit_rejection = matches!(result, Err(DictationError::SurfaceUnsupported { .. }))
        && buffer_text(&buffer).is_empty()
        && session.records().is_empty();

    DictationParityScenario {
        scenario_id: scenario_id.to_owned(),
        surface_id: surface.surface_id.clone(),
        surface_class: surface.surface_class,
        support: surface.support,
        outcome_class: DictationScenarioOutcomeClass::RejectedUnsupported,
        edits: session.records().to_vec(),
        history_group: None,
        undo_redo: None,
        restore_point_preserved: None,
        explicit_rejection,
        note_ref: "label:dictation:scenario_terminal_unsupported".to_owned(),
    }
}

/// Runs a degraded-surface scenario: plain dictated text applies, but a richer
/// formatting intent is explicitly rejected rather than half-applied.
fn degraded_scenario(surface: &DictationSurface) -> DictationParityScenario {
    let scenario_id = "dictation:scenario:find_field_degraded";
    let mut buffer = Buffer::from_str("");
    let mut selection = selection_at_end(&mut buffer);
    let mut session = DictationCaptureSession::begin(
        scenario_id,
        surface.clone(),
        DictationRecognitionLocality::OnDeviceLocal,
    );

    session
        .apply(
            &mut buffer,
            &mut selection,
            &DictationIntent::InsertText {
                text: "query".to_owned(),
            },
        )
        .expect("plain text applies on a text-only surface");
    let formatting = session.apply(
        &mut buffer,
        &mut selection,
        &DictationIntent::Formatting(FormattingIntent::NewLine),
    );
    let explicit_rejection = matches!(
        formatting,
        Err(DictationError::IntentUnsupportedOnSurface { .. })
    );

    DictationParityScenario {
        scenario_id: scenario_id.to_owned(),
        surface_id: surface.surface_id.clone(),
        surface_class: surface.surface_class,
        support: surface.support,
        outcome_class: DictationScenarioOutcomeClass::RejectedDegraded,
        edits: session.records().to_vec(),
        history_group: None,
        undo_redo: None,
        restore_point_preserved: None,
        explicit_rejection,
        note_ref: "label:dictation:scenario_find_field_degraded".to_owned(),
    }
}

/// Builds the canonical dictation-edit-parity packet from the deterministic
/// seed. This is the single mint-from-truth source for the checked-in fixtures
/// and the published companion doc.
pub fn seeded_dictation_edit_parity_packet() -> DictationEditParityPacket {
    let editor_main = surface(
        DictationSurfaceClass::MainEditor,
        "editor.main",
        DictationSurfaceSupport::Supported,
        "label:dictation:editor_main_full",
        "a11y:dictation:editor_main",
    );
    let rename_field = surface(
        DictationSurfaceClass::SingleLineTextField,
        "field.rename",
        DictationSurfaceSupport::Supported,
        "label:dictation:rename_field_full",
        "a11y:dictation:rename_field",
    );
    let commit_area = surface(
        DictationSurfaceClass::MultiLineTextArea,
        "scm.commit_message",
        DictationSurfaceSupport::Supported,
        "label:dictation:commit_message_full",
        "a11y:dictation:commit_message",
    );
    let find_field = surface(
        DictationSurfaceClass::SingleLineTextField,
        "field.find",
        DictationSurfaceSupport::DegradedTextOnly,
        "label:dictation:find_field_text_only",
        "a11y:dictation:find_field",
    );
    let terminal = surface(
        DictationSurfaceClass::Terminal,
        "terminal.integrated",
        DictationSurfaceSupport::Unsupported,
        "label:dictation:terminal_not_wired",
        "a11y:dictation:terminal",
    );
    let notebook = surface(
        DictationSurfaceClass::NotebookCell,
        "notebook.cell",
        DictationSurfaceSupport::Unsupported,
        "label:dictation:notebook_not_wired",
        "a11y:dictation:notebook",
    );
    let custom = surface(
        DictationSurfaceClass::CustomWidget,
        "extension.custom",
        DictationSurfaceSupport::Unsupported,
        "label:dictation:custom_widget_not_wired",
        "a11y:dictation:custom_widget",
    );

    let surfaces = vec![
        coverage_row(&editor_main),
        coverage_row(&rename_field),
        coverage_row(&commit_area),
        coverage_row(&find_field),
        coverage_row(&terminal),
        coverage_row(&notebook),
        coverage_row(&custom),
    ];

    let scenarios = vec![
        applied_scenario(
            "dictation:scenario:dictate_sentence_main_editor",
            &editor_main,
            DictationRecognitionLocality::OnDeviceLocal,
            "// notes:",
            &[
                DictationIntent::InsertText {
                    text: " capture totals".to_owned(),
                },
                DictationIntent::Punctuation(PunctuationMark::Period),
                DictationIntent::Formatting(FormattingIntent::NewLine),
                DictationIntent::InsertText {
                    text: "// then reconcile".to_owned(),
                },
            ],
            "label:dictation:scenario_dictate_sentence",
            true,
        ),
        applied_scenario(
            "dictation:scenario:scratch_that_then_redictate",
            &editor_main,
            DictationRecognitionLocality::OnDeviceLocal,
            "value = ",
            &[
                DictationIntent::InsertText {
                    text: "teh".to_owned(),
                },
                DictationIntent::Correction(CorrectionGesture::ScratchThat),
                DictationIntent::InsertText {
                    text: "the result".to_owned(),
                },
            ],
            "label:dictation:scenario_scratch_that",
            false,
        ),
        applied_scenario(
            "dictation:scenario:commit_message_hosted_provider",
            &commit_area,
            DictationRecognitionLocality::HostedProvider,
            "",
            &[
                DictationIntent::InsertText {
                    text: "Add dictation bridge".to_owned(),
                },
                DictationIntent::Formatting(FormattingIntent::NewParagraph),
                DictationIntent::InsertText {
                    text: "Routes speech through the shared edit model".to_owned(),
                },
            ],
            "label:dictation:scenario_commit_message",
            true,
        ),
        cancel_scenario(&rename_field),
        unsupported_scenario(&terminal),
        degraded_scenario(&find_field),
    ];

    DictationEditParityPacket::new(surfaces, scenarios)
}
