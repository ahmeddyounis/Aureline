//! Deterministic seed for the voice-command-bridge lane.
//!
//! The seed is the single mint-from-truth source for the checked-in fixtures,
//! the published companion doc, and any surface that ingests voice command
//! parity. Every id, ref, and label is stable so the fixtures stay bit-for-bit
//! equal across regenerations.

use super::{
    CommandImpactClass, ConfidenceCue, ConfirmationGateClass, DisabledReasonCode,
    EnablementDecisionClass, GroupedUndoLineage, InvocationLineageRecord, NoBypassGuards,
    ShortcutNarrationHint, TranscriptCorrectionAvailability, TranscriptStripState,
    VoiceBridgeCandidate, VoiceCommandBridgePacket, VoiceCommandBridgeRow,
    VoiceIntentResolutionClass, REDACTION_CLASS, VOICE_COMMAND_BRIDGE_ROW_RECORD_KIND,
    VOICE_COMMAND_BRIDGE_SCHEMA_VERSION, VOICE_COMMAND_BRIDGE_SHARED_CONTRACT_REF,
};

/// Canonical keyboard-first entry point every row falls back to.
const KEYBOARD_FALLBACK: &str = "cmd:command_palette.open";

fn shortcut(verb: &str) -> ShortcutNarrationHint {
    ShortcutNarrationHint {
        when_bound_narration_ref: format!("label:{verb}:shortcut_bound"),
        when_unbound_narration_ref: format!("label:{verb}:shortcut_unbound"),
        chord_class_hint: "modifier_plus_key".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn candidate(
    command_id: &str,
    verb: &str,
    enablement: EnablementDecisionClass,
    disabled_reason: Option<DisabledReasonCode>,
    impact: CommandImpactClass,
    preview_required: bool,
    approval_required: bool,
    confidence: ConfidenceCue,
) -> VoiceBridgeCandidate {
    VoiceBridgeCandidate {
        candidate_command_id: command_id.to_owned(),
        command_revision_ref: format!("cmd-rev:{verb}:2026.05.20-01"),
        canonical_verb: verb.to_owned(),
        primary_label_ref: format!("label:{verb}:primary"),
        shortcut: shortcut(verb),
        enablement_decision_class: enablement,
        disabled_reason_code: disabled_reason,
        impact_class: impact,
        preview_required,
        approval_required,
        confidence_cue: confidence,
    }
}

fn transcript_strip(
    heard: &str,
    confidence: ConfidenceCue,
    correction: TranscriptCorrectionAvailability,
) -> TranscriptStripState {
    TranscriptStripState {
        heard_text_label_ref: heard.to_owned(),
        confidence_cue: confidence,
        correction_availability: correction,
        shown_before_commit: true,
        edit_command_id: "cmd:voice.edit_transcript".to_owned(),
        correct_command_id: "cmd:voice.correct_transcript".to_owned(),
        confirm_command_id: "cmd:voice.confirm_transcript".to_owned(),
        cancel_command_id: "cmd:voice.cancel_transcript".to_owned(),
        accessibility_label_ref: "a11y:voice:transcript_strip".to_owned(),
    }
}

fn lineage(command_id: &str, verb: &str, reversible: bool) -> InvocationLineageRecord {
    InvocationLineageRecord {
        command_id: command_id.to_owned(),
        invocation_session_id: format!("inv:{verb}:voice:01"),
        result_packet_id: format!("result:{verb}:voice:01"),
        result_outcome_code: "succeeded".to_owned(),
        evidence_refs: vec![format!("evidence:{verb}:voice:01")],
        notification_refs: vec![format!("notification:{verb}:voice:01")],
        activity_refs: vec![format!("activity:{verb}:voice:01")],
        rollback_handle_posture: if reversible {
            "reversible_handle".to_owned()
        } else {
            "not_reversible_by_contract".to_owned()
        },
        rollback_handle_id: reversible.then(|| format!("rollback:{verb}:voice:01")),
        support_bundle_ref: Some("support-bundle:voice-bridge:01".to_owned()),
    }
}

fn grouped_undo(command_id: &str, verb: &str, reversible: bool) -> GroupedUndoLineage {
    GroupedUndoLineage {
        undo_group_id: format!("undo-group:{verb}:voice:01"),
        joins_shared_undo_history: true,
        history_entry_ref: format!("history:{verb}:voice:01"),
        commits_through_canonical_session: true,
        keyboard_equivalent_command_id: command_id.to_owned(),
        lineage: lineage(command_id, verb, reversible),
        no_bypass_guards: NoBypassGuards::strict(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    phrase: &str,
    intent: VoiceIntentResolutionClass,
    gate: ConfirmationGateClass,
    transcript: TranscriptStripState,
    candidates: Vec<VoiceBridgeCandidate>,
    selected: Option<&str>,
    undo: Option<GroupedUndoLineage>,
    docs_anchor: &str,
) -> VoiceCommandBridgeRow {
    VoiceCommandBridgeRow {
        record_kind: VOICE_COMMAND_BRIDGE_ROW_RECORD_KIND.to_owned(),
        schema_version: VOICE_COMMAND_BRIDGE_SCHEMA_VERSION,
        shared_contract_ref: VOICE_COMMAND_BRIDGE_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        spoken_phrase_label_ref: phrase.to_owned(),
        intent_class: intent,
        confirmation_gate: gate,
        transcript_strip: transcript,
        candidates,
        selected_command_id: selected.map(str::to_owned),
        grouped_undo_lineage: undo,
        keyboard_fallback_command_id: KEYBOARD_FALLBACK.to_owned(),
        docs_help_anchor_ref: docs_anchor.to_owned(),
        redaction_class: REDACTION_CLASS.to_owned(),
    }
}

/// Builds the deterministic seeded voice-command-bridge packet.
pub fn seeded_voice_command_bridge_packet() -> VoiceCommandBridgePacket {
    let canonical_command_ids = vec![
        "cmd:command_palette.open".to_owned(),
        "cmd:edit.rename_symbol_across_project".to_owned(),
        "cmd:edit.rename_symbol_in_file".to_owned(),
        "cmd:editor.insert_dictated_text".to_owned(),
        "cmd:git.push_current_branch".to_owned(),
        "cmd:navigation.go_to_definition".to_owned(),
    ];

    // 1. Unambiguous high-impact command: a project-wide rename routed through a
    //    confirmation gate with a required transcript correction before commit.
    let rename_confirm = row(
        "voice:bridge:rename_symbol_across_project_confirm",
        "label:voice:phrase:rename_symbol_across_project",
        VoiceIntentResolutionClass::ResolvesToSingleCommand,
        ConfirmationGateClass::ConfirmationRequiredBeforeCommit,
        transcript_strip(
            "label:voice:transcript:rename_symbol_across_project",
            ConfidenceCue::High,
            TranscriptCorrectionAvailability::RequiredBeforeCommit,
        ),
        vec![candidate(
            "cmd:edit.rename_symbol_across_project",
            "edit.rename_symbol_across_project",
            EnablementDecisionClass::Enabled,
            None,
            CommandImpactClass::RecoverableDurableMutation,
            true,
            false,
            ConfidenceCue::High,
        )],
        Some("cmd:edit.rename_symbol_across_project"),
        Some(grouped_undo(
            "cmd:edit.rename_symbol_across_project",
            "edit.rename_symbol_across_project",
            true,
        )),
        "docs:anchor:voice:confirmation_before_high_impact",
    );

    // 2. Ambiguous "rename symbol": a candidate sheet with two canonical
    //    commands — one disabled with a visible reason — and nothing committed.
    let rename_ambiguous = row(
        "voice:bridge:rename_symbol_ambiguous",
        "label:voice:phrase:rename_symbol_ambiguous",
        VoiceIntentResolutionClass::AmbiguousRequiresDisambiguation,
        ConfirmationGateClass::DisambiguationRequiredBeforeCommit,
        transcript_strip(
            "label:voice:transcript:rename_symbol_ambiguous",
            ConfidenceCue::Medium,
            TranscriptCorrectionAvailability::OfferedBeforeCommit,
        ),
        vec![
            candidate(
                "cmd:edit.rename_symbol_across_project",
                "edit.rename_symbol_across_project",
                EnablementDecisionClass::Enabled,
                None,
                CommandImpactClass::RecoverableDurableMutation,
                true,
                false,
                ConfidenceCue::High,
            ),
            candidate(
                "cmd:edit.rename_symbol_in_file",
                "edit.rename_symbol_in_file",
                EnablementDecisionClass::DisabledWithReason,
                Some(DisabledReasonCode::WorkspaceTrustRestricted),
                CommandImpactClass::ReversibleLocalMutation,
                false,
                false,
                ConfidenceCue::Medium,
            ),
        ],
        None,
        None,
        "docs:anchor:voice:disambiguation_sheet",
    );

    // 3. Irreversible publish: a branch push that keeps preview + approval and a
    //    required correction, with audit lineage but no rollback handle.
    let push_confirm = row(
        "voice:bridge:push_current_branch_confirm",
        "label:voice:phrase:push_current_branch",
        VoiceIntentResolutionClass::ResolvesToSingleCommand,
        ConfirmationGateClass::ConfirmationRequiredBeforeCommit,
        transcript_strip(
            "label:voice:transcript:push_current_branch",
            ConfidenceCue::High,
            TranscriptCorrectionAvailability::RequiredBeforeCommit,
        ),
        vec![candidate(
            "cmd:git.push_current_branch",
            "git.push_current_branch",
            EnablementDecisionClass::Enabled,
            None,
            CommandImpactClass::IrreversiblePublish,
            true,
            true,
            ConfidenceCue::High,
        )],
        Some("cmd:git.push_current_branch"),
        Some(grouped_undo(
            "cmd:git.push_current_branch",
            "git.push_current_branch",
            false,
        )),
        "docs:anchor:voice:irreversible_publish_confirmation",
    );

    // 4. Dictation text: routed through the shared edit model and grouped undo,
    //    committing directly with an offered correction.
    let dictation = row(
        "voice:bridge:insert_dictated_text",
        "label:voice:phrase:dictated_segment",
        VoiceIntentResolutionClass::ResolvesToDictationText,
        ConfirmationGateClass::DirectCommitLowImpact,
        transcript_strip(
            "label:voice:transcript:dictated_segment",
            ConfidenceCue::Medium,
            TranscriptCorrectionAvailability::OfferedBeforeCommit,
        ),
        vec![candidate(
            "cmd:editor.insert_dictated_text",
            "editor.insert_dictated_text",
            EnablementDecisionClass::Enabled,
            None,
            CommandImpactClass::ReversibleLocalMutation,
            false,
            false,
            ConfidenceCue::Medium,
        )],
        Some("cmd:editor.insert_dictated_text"),
        Some(grouped_undo(
            "cmd:editor.insert_dictated_text",
            "editor.insert_dictated_text",
            true,
        )),
        "docs:anchor:voice:dictation_grouped_undo",
    );

    // 5. Reversible command-mode read: go-to-definition commits directly while
    //    still producing keyboard-parity lineage in the shared history.
    let go_to_definition = row(
        "voice:bridge:go_to_definition_direct",
        "label:voice:phrase:go_to_definition",
        VoiceIntentResolutionClass::ResolvesToSingleCommand,
        ConfirmationGateClass::DirectCommitLowImpact,
        transcript_strip(
            "label:voice:transcript:go_to_definition",
            ConfidenceCue::High,
            TranscriptCorrectionAvailability::OfferedBeforeCommit,
        ),
        vec![candidate(
            "cmd:navigation.go_to_definition",
            "navigation.go_to_definition",
            EnablementDecisionClass::Enabled,
            None,
            CommandImpactClass::ReversibleLocalRead,
            false,
            false,
            ConfidenceCue::High,
        )],
        Some("cmd:navigation.go_to_definition"),
        Some(grouped_undo(
            "cmd:navigation.go_to_definition",
            "navigation.go_to_definition",
            false,
        )),
        "docs:anchor:voice:direct_commit_low_impact",
    );

    // 6. Denied utterance: the verb is not on the stable command graph, so the
    //    bridge invents nothing and offers a keyboard-first fallback.
    let denied = row(
        "voice:bridge:denied_uncanonical_verb",
        "label:voice:phrase:uncanonical_verb",
        VoiceIntentResolutionClass::DeniedNoCanonicalCommand,
        ConfirmationGateClass::BlockedNoCanonicalCommand,
        transcript_strip(
            "label:voice:transcript:uncanonical_verb",
            ConfidenceCue::Low,
            TranscriptCorrectionAvailability::OfferedBeforeCommit,
        ),
        Vec::new(),
        None,
        None,
        "docs:anchor:voice:denied_uncanonical_verb",
    );

    VoiceCommandBridgePacket::new(
        vec![
            rename_confirm,
            rename_ambiguous,
            push_confirm,
            dictation,
            go_to_definition,
            denied,
        ],
        canonical_command_ids,
    )
}
