//! Unit and fixture-equality coverage for the voice-command-bridge lane.

use std::path::{Path, PathBuf};

use super::seed::seeded_voice_command_bridge_packet;
use super::*;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/voice/disambiguation")
}

#[test]
fn seed_validates_and_marks_every_invariant_satisfied() {
    let packet = seeded_voice_command_bridge_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "seed must validate: {violations:?}");
    assert!(packet.is_well_formed());
    assert_eq!(packet.invariants, VoiceBridgeInvariantManifest::all_true());
    assert!(packet.raw_audio_or_transcript_bytes_excluded);
}

#[test]
fn seed_envelope_is_stable() {
    let packet = seeded_voice_command_bridge_packet();
    assert_eq!(packet.record_kind, VOICE_COMMAND_BRIDGE_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, VOICE_COMMAND_BRIDGE_SCHEMA_VERSION);
    assert_eq!(packet.packet_id, VOICE_COMMAND_BRIDGE_PACKET_ID);
    assert_eq!(packet.doc_ref, VOICE_COMMAND_BRIDGE_DOC_REF);
    assert_eq!(
        packet.fixtures_dir_ref,
        VOICE_COMMAND_BRIDGE_FIXTURES_DIR_REF
    );
    for row in &packet.rows {
        assert_eq!(row.record_kind, VOICE_COMMAND_BRIDGE_ROW_RECORD_KIND);
        assert_eq!(row.schema_version, VOICE_COMMAND_BRIDGE_SCHEMA_VERSION);
        assert_eq!(row.redaction_class, REDACTION_CLASS);
    }
}

#[test]
fn seed_covers_every_intent_class() {
    let packet = seeded_voice_command_bridge_packet();
    let intents: Vec<VoiceIntentResolutionClass> =
        packet.rows.iter().map(|r| r.intent_class).collect();
    for expected in [
        VoiceIntentResolutionClass::ResolvesToSingleCommand,
        VoiceIntentResolutionClass::AmbiguousRequiresDisambiguation,
        VoiceIntentResolutionClass::ResolvesToDictationText,
        VoiceIntentResolutionClass::DeniedNoCanonicalCommand,
    ] {
        assert!(intents.contains(&expected), "missing intent {expected:?}");
    }
}

#[test]
fn ambiguous_row_exposes_candidates_and_commits_nothing() {
    let packet = seeded_voice_command_bridge_packet();
    let row = packet
        .row("voice:bridge:rename_symbol_ambiguous")
        .expect("ambiguous row");
    assert!(
        row.candidates.len() >= 2,
        "ambiguity needs a candidate list"
    );
    assert!(row.selected_command_id.is_none(), "must not auto-select");
    assert!(
        row.grouped_undo_lineage.is_none(),
        "ambiguity must not commit silently"
    );
    // The disabled candidate keeps its reason visible, not hidden.
    assert!(row
        .candidates
        .iter()
        .any(|c| !c.is_enabled() && c.disabled_reason_code.is_some()));
}

#[test]
fn high_impact_rows_require_confirmation_and_correction() {
    let packet = seeded_voice_command_bridge_packet();
    let mut saw_high_impact = false;
    for row in &packet.rows {
        if row
            .selected_impact()
            .map(CommandImpactClass::is_high_impact)
            .unwrap_or(false)
        {
            saw_high_impact = true;
            assert_eq!(
                row.confirmation_gate,
                ConfirmationGateClass::ConfirmationRequiredBeforeCommit,
                "{}",
                row.row_id
            );
            assert!(row.transcript_strip.correction_availability.is_available());
            assert!(row.transcript_strip.shown_before_commit);
        }
    }
    assert!(saw_high_impact, "seed must exercise a high-impact command");
}

#[test]
fn committed_rows_keep_keyboard_lineage_parity() {
    let packet = seeded_voice_command_bridge_packet();
    let mut saw_commit = false;
    for row in &packet.rows {
        if let Some(undo) = &row.grouped_undo_lineage {
            saw_commit = true;
            assert!(undo.commits_through_canonical_session);
            assert!(undo.joins_shared_undo_history);
            assert_eq!(
                Some(undo.keyboard_equivalent_command_id.as_str()),
                row.selected_command_id.as_deref()
            );
            assert_eq!(undo.lineage.command_id, undo.keyboard_equivalent_command_id);
        }
    }
    assert!(saw_commit, "seed must exercise a committed command");
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_voice_command_bridge_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let parsed: VoiceCommandBridgePacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(packet, parsed);
}

#[test]
fn injected_ambiguous_auto_execution_is_caught() {
    let mut packet = seeded_voice_command_bridge_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.intent_class == VoiceIntentResolutionClass::AmbiguousRequiresDisambiguation)
        .expect("ambiguous row");
    row.selected_command_id = Some("cmd:edit.rename_symbol_across_project".to_owned());
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceBridgeViolation::AmbiguousIntentAutoExecuted { .. })));
}

#[test]
fn injected_silent_high_impact_is_caught() {
    let packet = seeded_voice_command_bridge_packet();
    let mut row = packet
        .row("voice:bridge:rename_symbol_across_project_confirm")
        .cloned()
        .expect("rename row");
    row.confirmation_gate = ConfirmationGateClass::DirectCommitLowImpact;
    let id_set = packet.canonical_id_set();
    let violations = row.check(&id_set);
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceBridgeViolation::SilentHighImpactWithoutConfirmation { .. }
    )));
}

#[test]
fn injected_correction_removal_is_caught() {
    let mut packet = seeded_voice_command_bridge_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "voice:bridge:push_current_branch_confirm")
        .expect("push row");
    row.transcript_strip.correction_availability = TranscriptCorrectionAvailability::Unavailable;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceBridgeViolation::CorrectionUnavailableBeforeHighImpactCommit { .. }
    )));
}

#[test]
fn injected_invented_command_is_caught() {
    let mut packet = seeded_voice_command_bridge_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "voice:bridge:go_to_definition_direct")
        .expect("go-to-def row");
    row.selected_command_id = Some("cmd:edit.invented_speech_only_macro".to_owned());
    if let Some(candidate) = row.candidates.first_mut() {
        candidate.candidate_command_id = "cmd:edit.invented_speech_only_macro".to_owned();
    }
    if let Some(undo) = row.grouped_undo_lineage.as_mut() {
        undo.keyboard_equivalent_command_id = "cmd:edit.invented_speech_only_macro".to_owned();
        undo.lineage.command_id = "cmd:edit.invented_speech_only_macro".to_owned();
    }
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceBridgeViolation::InventedNonCanonicalCommand { .. })));
}

#[test]
fn injected_disabled_candidate_without_reason_is_caught() {
    let mut packet = seeded_voice_command_bridge_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.intent_class == VoiceIntentResolutionClass::AmbiguousRequiresDisambiguation)
        .expect("ambiguous row");
    let candidate = row
        .candidates
        .iter_mut()
        .find(|c| !c.is_enabled())
        .expect("disabled candidate");
    candidate.disabled_reason_code = None;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceBridgeViolation::CandidateMissingDisabledReason { .. }
    )));
}

#[test]
fn injected_side_path_commit_is_caught() {
    let mut packet = seeded_voice_command_bridge_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "voice:bridge:insert_dictated_text")
        .expect("dictation row");
    row.grouped_undo_lineage
        .as_mut()
        .expect("undo")
        .commits_through_canonical_session = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceBridgeViolation::CommitsThroughSidePath { .. })));
}

#[test]
fn injected_weakened_guard_is_caught() {
    let mut packet = seeded_voice_command_bridge_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == "voice:bridge:rename_symbol_across_project_confirm")
        .expect("rename row");
    row.grouped_undo_lineage
        .as_mut()
        .expect("undo")
        .no_bypass_guards
        .preview_path_preserved = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceBridgeViolation::HighImpactGuardsWeakened { .. })));
}

#[test]
fn injected_missing_keyboard_fallback_is_caught() {
    let mut packet = seeded_voice_command_bridge_packet();
    packet.rows[0].keyboard_fallback_command_id.clear();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceBridgeViolation::MissingKeyboardFallback { .. })));
}

#[test]
fn denied_row_binds_no_command() {
    let packet = seeded_voice_command_bridge_packet();
    let row = packet
        .row("voice:bridge:denied_uncanonical_verb")
        .expect("denied row");
    assert!(row.selected_command_id.is_none());
    assert!(row.grouped_undo_lineage.is_none());
    assert!(row.candidates.is_empty());
    assert!(!row.keyboard_fallback_command_id.trim().is_empty());
}

#[test]
fn render_markdown_lists_every_row() {
    let packet = seeded_voice_command_bridge_packet();
    let md = packet.render_markdown();
    assert!(md.starts_with("# Voice disambiguation and confirmation"));
    for row in &packet.rows {
        assert!(md.contains(&row.row_id), "markdown missing {}", row.row_id);
    }
}

#[test]
fn compact_lines_summarize_each_row() {
    let packet = seeded_voice_command_bridge_packet();
    let lines = packet.compact_lines();
    assert_eq!(lines.len(), packet.rows.len() + 1);
}

#[test]
fn on_disk_fixtures_match_seed_bit_for_bit() {
    let packet = seeded_voice_command_bridge_packet();
    let dir = fixtures_dir();

    let expected_packet = fixture_json(&packet).expect("serialize packet");
    let actual_packet =
        std::fs::read_to_string(dir.join("packet.json")).expect("read packet.json fixture");
    assert_eq!(
        actual_packet, expected_packet,
        "packet.json drifted from seed; regenerate with the dump_voice_command_bridge example"
    );

    for row in &packet.rows {
        let file = row_fixture_file_name(row);
        let expected = fixture_json(row).expect("serialize row");
        let actual =
            std::fs::read_to_string(dir.join(&file)).unwrap_or_else(|e| panic!("read {file}: {e}"));
        assert_eq!(actual, expected, "{file} drifted from seed");
    }

    let mut expected_compact = packet.compact_lines().join("\n");
    expected_compact.push('\n');
    let actual_compact =
        std::fs::read_to_string(dir.join("compact.txt")).expect("read compact.txt fixture");
    assert_eq!(
        actual_compact, expected_compact,
        "compact.txt drifted from seed"
    );
}
