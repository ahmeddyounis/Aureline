//! Unit, lifecycle, and fixture-equality coverage for the dictation-edit lane.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_buffer::Buffer;

use super::seed::seeded_dictation_edit_parity_packet;
use super::*;
use crate::selection::SelectionState;
use crate::viewport::TextPoint;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/voice/dictation-edit-parity")
}

fn doc_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/ux/dictation-edit-contract.md")
}

fn supported_editor() -> DictationSurface {
    DictationSurface {
        surface_class: DictationSurfaceClass::MainEditor,
        surface_id: "editor.main".to_owned(),
        support: DictationSurfaceSupport::Supported,
        support_reason_ref: "label:dictation:editor_main_full".to_owned(),
        accessibility_label_ref: "a11y:dictation:editor_main".to_owned(),
    }
}

fn buffer_text(buffer: &Buffer) -> String {
    String::from_utf8(buffer.contents()).expect("buffer is utf-8")
}

#[test]
fn seed_validates_and_marks_every_invariant_satisfied() {
    let packet = seeded_dictation_edit_parity_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "seed must validate: {violations:?}");
    assert!(packet.is_well_formed());
    assert!(packet.invariants.all_satisfied());
    assert!(packet.raw_audio_or_transcript_bytes_excluded);
    assert!(packet.no_hidden_speech_buffer);
    assert!(packet.routes_through_shared_edit_model);
}

#[test]
fn seed_envelope_is_stable() {
    let packet = seeded_dictation_edit_parity_packet();
    assert_eq!(packet.record_kind, DICTATION_EDIT_PARITY_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, DICTATION_EDIT_PARITY_SCHEMA_VERSION);
    assert_eq!(packet.packet_id, DICTATION_EDIT_PARITY_PACKET_ID);
    assert_eq!(packet.doc_ref, DICTATION_EDIT_PARITY_DOC_REF);
    assert_eq!(
        packet.fixtures_dir_ref,
        DICTATION_EDIT_PARITY_FIXTURES_DIR_REF
    );
    assert_eq!(packet.redaction_class, REDACTION_CLASS);
}

#[test]
fn seed_covers_supported_unsupported_and_degraded_surfaces() {
    let packet = seeded_dictation_edit_parity_packet();
    let supports: Vec<DictationSurfaceSupport> =
        packet.surfaces.iter().map(|s| s.support).collect();
    assert!(supports.contains(&DictationSurfaceSupport::Supported));
    assert!(supports.contains(&DictationSurfaceSupport::DegradedTextOnly));
    assert!(supports.contains(&DictationSurfaceSupport::Unsupported));
    // No unsupported / degraded surface drops its explicit reason.
    for surface in &packet.surfaces {
        if surface.support != DictationSurfaceSupport::Supported {
            assert!(
                !surface.support_reason_ref.trim().is_empty(),
                "{} has no reason",
                surface.surface_id
            );
        }
    }
}

#[test]
fn checked_in_fixtures_match_seed() {
    let packet = seeded_dictation_edit_parity_packet();
    let dir = fixtures_dir();

    let packet_json = fs::read_to_string(dir.join("packet.json")).expect("packet.json present");
    assert_eq!(
        packet_json,
        fixture_json(&packet).expect("serialize packet"),
        "packet.json drifted; regenerate fixtures"
    );

    for scenario in &packet.scenarios {
        let name = scenario_fixture_file_name(scenario);
        let json = fs::read_to_string(dir.join(&name)).unwrap_or_else(|_| panic!("missing {name}"));
        assert_eq!(
            json,
            fixture_json(scenario).expect("serialize scenario"),
            "{name} drifted; regenerate fixtures"
        );
    }

    let compact = fs::read_to_string(dir.join("compact.txt")).expect("compact.txt present");
    let mut expected = packet.compact_lines().join("\n");
    expected.push('\n');
    assert_eq!(
        compact, expected,
        "compact.txt drifted; regenerate fixtures"
    );
}

#[test]
fn checked_in_doc_matches_seed() {
    let packet = seeded_dictation_edit_parity_packet();
    let doc = fs::read_to_string(doc_path()).expect("doc present");
    assert_eq!(doc, packet.render_markdown(), "doc drifted; regenerate it");
}

#[test]
fn dictated_text_lands_in_text_edit_group_and_undoes() {
    let mut buffer = Buffer::from_str("start ");
    let mut selection = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 6,
    });
    let mut session = DictationCaptureSession::begin(
        "cap:text",
        supported_editor(),
        DictationRecognitionLocality::OnDeviceLocal,
    );

    let outcome = session
        .apply(
            &mut buffer,
            &mut selection,
            &DictationIntent::InsertText {
                text: "done".to_owned(),
            },
        )
        .expect("dictated insert applies");

    assert_eq!(outcome.record.undo_class_id, "text_edit");
    assert_eq!(outcome.record.effect, DictationEffectClass::InsertText);
    assert!(outcome.record.undo_group_id.is_some());
    assert!(outcome.mutated_buffer);
    assert_eq!(buffer_text(&buffer), "start done");

    // The dictated group reverses through the ordinary shared undo stack.
    assert!(buffer.undo().is_some());
    assert_eq!(buffer_text(&buffer), "start ");
}

#[test]
fn punctuation_and_formatting_route_through_text_edits() {
    let mut buffer = Buffer::from_str("hi");
    let mut selection = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 2,
    });
    let mut session = DictationCaptureSession::begin(
        "cap:mix",
        supported_editor(),
        DictationRecognitionLocality::OnDeviceLocal,
    );

    session
        .apply(
            &mut buffer,
            &mut selection,
            &DictationIntent::Punctuation(PunctuationMark::ExclamationMark),
        )
        .expect("punctuation applies");
    session
        .apply(
            &mut buffer,
            &mut selection,
            &DictationIntent::Formatting(FormattingIntent::NewLine),
        )
        .expect("formatting applies");

    assert_eq!(buffer_text(&buffer), "hi!\n");
    let group = session.history_group().expect("history group");
    assert!(group.is_well_formed(), "{:?}", group.check());
    assert_eq!(group.members.len(), 2);
    assert!(group
        .members
        .iter()
        .all(|m| m.uses_ordinary_text_edit_class()));
}

#[test]
fn scratch_that_reverses_through_shared_undo_stack() {
    let mut buffer = Buffer::from_str("value = ");
    let mut selection = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 8,
    });
    let mut session = DictationCaptureSession::begin(
        "cap:scratch",
        supported_editor(),
        DictationRecognitionLocality::OnDeviceLocal,
    );

    session
        .apply(
            &mut buffer,
            &mut selection,
            &DictationIntent::InsertText {
                text: "teh".to_owned(),
            },
        )
        .expect("insert applies");
    assert_eq!(buffer_text(&buffer), "value = teh");

    let scratch = session
        .apply(
            &mut buffer,
            &mut selection,
            &DictationIntent::Correction(CorrectionGesture::ScratchThat),
        )
        .expect("scratch applies");
    assert_eq!(scratch.record.effect, DictationEffectClass::UndoLastGroup);
    assert_eq!(buffer_text(&buffer), "value = ");
}

#[test]
fn cancel_restores_prior_insertion_point() {
    let buffer = Buffer::from_str("oldName");
    let mut selection = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 7,
    });
    let insertion_point = selection.clone();
    let mut session = DictationCaptureSession::begin(
        "cap:cancel",
        DictationSurface {
            surface_class: DictationSurfaceClass::SingleLineTextField,
            surface_id: "field.rename".to_owned(),
            support: DictationSurfaceSupport::Supported,
            support_reason_ref: "label:dictation:rename_field_full".to_owned(),
            accessibility_label_ref: "a11y:dictation:rename_field".to_owned(),
        },
        DictationRecognitionLocality::OnDeviceLocal,
    );

    session.set_interim("newName", &selection);
    selection.set_primary_caret(TextPoint {
        line: 0,
        grapheme: 2,
    });
    let summary = session.cancel(&mut selection);

    assert_eq!(summary.status, CaptureStatus::Cancelled);
    assert_eq!(selection, insertion_point);
    assert!(session.interim().is_none());
    // Cancelling discards the interim with no buffer mutation.
    assert_eq!(buffer_text(&buffer), "oldName");
}

#[test]
fn end_finalizes_interim_through_shared_model() {
    let mut buffer = Buffer::from_str("x = ");
    let mut selection = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 4,
    });
    let mut session = DictationCaptureSession::begin(
        "cap:end",
        supported_editor(),
        DictationRecognitionLocality::OnDeviceLocal,
    );

    session.set_interim("42", &selection);
    let summary = session
        .end(&mut buffer, &mut selection)
        .expect("end applies");

    assert_eq!(summary.status, CaptureStatus::Ended);
    assert_eq!(buffer_text(&buffer), "x = 42");
    assert_eq!(summary.content_edits, 1);
    let group = summary.history_group.expect("history group");
    assert!(group.is_well_formed());
}

#[test]
fn unsupported_surface_rejects_without_mutation() {
    let mut buffer = Buffer::from_str("");
    let mut selection = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 0,
    });
    let mut session = DictationCaptureSession::begin(
        "cap:terminal",
        DictationSurface {
            surface_class: DictationSurfaceClass::Terminal,
            surface_id: "terminal.integrated".to_owned(),
            support: DictationSurfaceSupport::Unsupported,
            support_reason_ref: "label:dictation:terminal_not_wired".to_owned(),
            accessibility_label_ref: "a11y:dictation:terminal".to_owned(),
        },
        DictationRecognitionLocality::OnDeviceLocal,
    );

    let result = session.apply(
        &mut buffer,
        &mut selection,
        &DictationIntent::InsertText {
            text: "ls".to_owned(),
        },
    );

    assert!(matches!(
        result,
        Err(DictationError::SurfaceUnsupported { .. })
    ));
    assert!(session.records().is_empty());
    assert!(buffer_text(&buffer).is_empty());
}

#[test]
fn degraded_surface_rejects_richer_intents() {
    let mut buffer = Buffer::from_str("");
    let mut selection = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 0,
    });
    let mut session = DictationCaptureSession::begin(
        "cap:find",
        DictationSurface {
            surface_class: DictationSurfaceClass::SingleLineTextField,
            surface_id: "field.find".to_owned(),
            support: DictationSurfaceSupport::DegradedTextOnly,
            support_reason_ref: "label:dictation:find_field_text_only".to_owned(),
            accessibility_label_ref: "a11y:dictation:find_field".to_owned(),
        },
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
        .expect("plain text applies");
    assert_eq!(buffer_text(&buffer), "query");

    let formatting = session.apply(
        &mut buffer,
        &mut selection,
        &DictationIntent::Formatting(FormattingIntent::NewLine),
    );
    assert!(matches!(
        formatting,
        Err(DictationError::IntentUnsupportedOnSurface { .. })
    ));
    // The rejected intent left the buffer untouched.
    assert_eq!(buffer_text(&buffer), "query");
}

#[test]
fn applying_after_end_is_rejected() {
    let mut buffer = Buffer::from_str("");
    let mut selection = SelectionState::new(TextPoint {
        line: 0,
        grapheme: 0,
    });
    let mut session = DictationCaptureSession::begin(
        "cap:closed",
        supported_editor(),
        DictationRecognitionLocality::OnDeviceLocal,
    );
    session.end(&mut buffer, &mut selection).expect("end");

    let result = session.apply(
        &mut buffer,
        &mut selection,
        &DictationIntent::InsertText {
            text: "late".to_owned(),
        },
    );
    assert!(matches!(result, Err(DictationError::NotCapturing { .. })));
}

#[test]
fn scenario_history_groups_are_parity_clean() {
    let packet = seeded_dictation_edit_parity_packet();
    for scenario in &packet.scenarios {
        if let Some(group) = &scenario.history_group {
            assert!(
                group.is_well_formed(),
                "{}: {:?}",
                scenario.scenario_id,
                group.check()
            );
        }
        if let Some(roundtrip) = scenario.undo_redo {
            assert!(
                roundtrip.returns_to_seed_after_full_undo
                    && roundtrip.returns_to_final_after_full_redo
                    && roundtrip.all_groups_text_edit_class,
                "{} round-trip drift",
                scenario.scenario_id
            );
        }
    }
}
