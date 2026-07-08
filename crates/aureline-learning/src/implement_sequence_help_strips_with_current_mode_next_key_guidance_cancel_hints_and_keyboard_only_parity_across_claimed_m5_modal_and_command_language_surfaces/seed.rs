//! Canonical seed builders for the M5 sequence-help-strip primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical sequence-help-strip primitive packet.
pub const M5_SEQUENCE_HELP_STRIP_PACKET_ID: &str = "m5-sequence-help-strip-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked sequence-help-strip resolution case from a full sequence state.
#[allow(clippy::too_many_arguments)]
fn sequence_case(
    help_state: M5SequenceHelpState,
    step_kind: M5SequenceStepKind,
    command_backing: M5CommandBackingState,
    current_mode_or_leader_ref: &str,
    valid_next_keys: &[&str],
    cancel_key: &str,
    example_command_ref: Option<&str>,
    screen_reader_announcement: &str,
    cheat_sheet_ref: &str,
    strip_identity_ref: &str,
) -> M5SequenceHelpStripResolutionCase {
    M5SequenceHelpStripResolutionCase::resolved(M5SequenceHelpStripResolutionInput {
        help_state,
        step_kind,
        command_backing,
        current_mode_or_leader_ref: current_mode_or_leader_ref.to_owned(),
        valid_next_keys: valid_next_keys.iter().map(|s| (*s).to_owned()).collect(),
        cancel_key: cancel_key.to_owned(),
        example_command_ref: example_command_ref.map(str::to_owned),
        screen_reader_announcement: screen_reader_announcement.to_owned(),
        cheat_sheet_ref: cheat_sheet_ref.to_owned(),
        strip_identity_ref: strip_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full sequence-help-strip anatomy,
/// help-state, step-kind, command-backing, help-posture, action, export-field, and accessibility
/// parity every consumer carries.
fn base_row(
    consumer_surface: M5SequenceHelpConsumerSurface,
    qualification: M5TeachingQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    sequence_examples: Vec<M5SequenceHelpStripResolutionCase>,
) -> M5SequenceHelpConsumerRow {
    M5SequenceHelpConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TeachingSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TeachingDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5SequenceHelpAnatomyPart::ALL.to_vec(),
        help_states: M5SequenceHelpState::ALL.to_vec(),
        step_kinds: M5SequenceStepKind::ALL.to_vec(),
        command_backing_states: M5CommandBackingState::ALL.to_vec(),
        help_postures: M5SequenceHelpPosture::ALL.to_vec(),
        help_actions: M5SequenceHelpAction::ALL.to_vec(),
        export_fields: M5SequenceHelpExportField::ALL.to_vec(),
        accessibility_routes: M5TeachingAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TeachingConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TeachingDowngradeTrigger::SequenceHelpStateUnstated,
            M5TeachingDowngradeTrigger::CommandBackingHidden,
            M5TeachingDowngradeTrigger::AlternateStateLabelInvented,
            M5TeachingDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SEQUENCE_HELP_STRIP_SCHEMA_REF,
            M5_SEQUENCE_HELP_STRIP_KEYBINDING_RESOLVER_REF,
            M5_SEQUENCE_HELP_STRIP_COMMAND_DESCRIPTOR_REF,
        ]),
        sequence_examples,
        masks_current_mode_or_next_keys: false,
        fails_silently_on_partial_or_ambiguous: false,
        requires_pointer_hover: false,
        severs_command_backing_or_cheat_sheet: false,
    }
}

fn rows() -> Vec<M5SequenceHelpConsumerRow> {
    use M5CommandBackingState as Backing;
    use M5SequenceHelpConsumerSurface as Surface;
    use M5SequenceHelpState as State;
    use M5SequenceStepKind as Step;
    use M5TeachingQualificationClass as Qual;

    vec![
        // 1. Leader-sequence overlay — a leader key ready for its first key, and a chord awaiting
        //    its next key. Both are open sequences that show their valid next keys, the cancel
        //    key, and an example command.
        base_row(
            Surface::LeaderSequenceOverlay,
            Qual::Stable,
            "Leader-sequence overlay owner",
            "The leader-sequence overlay renders the shared sequence-help strip so a leader key ready for its first key shows the current leader, the valid next keys, the cancel key, and an example command, and a chord awaiting its next key shows the same current-mode / next-keys / cancel / example guidance — every open leader sequence is inspectable before it completes, with a screen-reader announcement and no reliance on pointer hover",
            "evidence:m5-sequence-help-strip-leader-sequence-overlay:001",
            vec![
                sequence_case(
                    State::Ready,
                    Step::LeaderKey,
                    Backing::KeybindingRoute,
                    "Leader (Space)",
                    &["f", "g", "s"],
                    "Esc",
                    Some("command:leader.find-file"),
                    "Leader active. Press f to find a file, g for git, s to search. Press Escape to cancel.",
                    "cheatsheet:leader-keys",
                    "strip:leader-overlay:leader-root",
                ),
                sequence_case(
                    State::AwaitingNextKey,
                    Step::Chord,
                    Backing::BoundCommand,
                    "Leader g (git)",
                    &["s", "c", "p"],
                    "Esc",
                    Some("command:git.status"),
                    "Leader g pending. Press s for status, c to commit, p to push. Press Escape to cancel.",
                    "cheatsheet:leader-keys",
                    "strip:leader-overlay:leader-git",
                ),
            ],
        ),
        // 2. Modal-operator strip — a partial operator+motion match still awaiting its motion, and
        //    a terminal action disabled in the current context. The disabled sequence still shows
        //    the current mode, the cancel key, and an example command, and can open the full
        //    cheat sheet — it never fails silently.
        base_row(
            Surface::ModalOperatorStrip,
            Qual::Stable,
            "Modal-operator strip owner",
            "The modal-operator strip renders the shared sequence-help strip so a partial operator awaiting its motion shows the operator mode, the valid motion keys, the cancel key, and an example command, and a terminal action disabled in the current context is shown honestly as disabled — still naming the current mode, the cancel key, and the example command and keeping the full cheat sheet reachable — so a keyboard-first user always knows why the operator will not complete",
            "evidence:m5-sequence-help-strip-modal-operator-strip:001",
            vec![
                sequence_case(
                    State::PartialMatch,
                    Step::Operator,
                    Backing::BoundCommand,
                    "Normal — operator d (delete)",
                    &["w", "$", "}"],
                    "Esc",
                    Some("command:editor.delete-motion"),
                    "Delete operator pending. Press w to delete a word, $ to end of line, } to paragraph. Press Escape to cancel.",
                    "cheatsheet:modal-operators",
                    "strip:modal-operator:delete-motion",
                ),
                sequence_case(
                    State::DisabledInContext,
                    Step::TerminalAction,
                    Backing::UnboundHint,
                    "Normal — record macro q",
                    &[],
                    "Esc",
                    Some("command:editor.record-macro"),
                    "Macro recording is disabled while a modal dialog is open. Press Escape to dismiss this hint, or open the cheat sheet to learn more.",
                    "cheatsheet:modal-operators",
                    "strip:modal-operator:record-macro-disabled",
                ),
            ],
        ),
        // 3. Partial-command hint — a no-binding dead end (no next keys, no command backing) that
        //    still keeps cancel and the full cheat sheet reachable, and a partial motion match
        //    that shows its valid next keys and an example command.
        base_row(
            Surface::PartialCommandHint,
            Qual::Stable,
            "Partial-command hint owner",
            "The partial-command hint renders the shared sequence-help strip so a keystroke run that resolves to no binding is shown honestly as an unbound dead end — naming the current mode, keeping the cancel key and the full cheat sheet reachable, and never failing silently — and a partial motion match shows the valid next keys, the cancel key, and an example command so an ambiguous partial command is always interpretable without external docs",
            "evidence:m5-sequence-help-strip-partial-command-hint:001",
            vec![
                sequence_case(
                    State::NoBinding,
                    Step::PrefixArgument,
                    Backing::NoCommandBacking,
                    "Normal — prefix 12",
                    &[],
                    "Esc",
                    None,
                    "No command is bound to the keys you entered. Press Escape to clear, or open the cheat sheet to see valid commands.",
                    "cheatsheet:command-language",
                    "strip:partial-command:unbound-prefix",
                ),
                sequence_case(
                    State::PartialMatch,
                    Step::Motion,
                    Backing::DeepLinkCommand,
                    "Normal — motion g",
                    &["g", "j", "k"],
                    "Esc",
                    Some("command:editor.go-to-top"),
                    "Motion g pending. Press g to go to the top, j down, k up. Press Escape to cancel.",
                    "cheatsheet:command-language",
                    "strip:partial-command:go-motion",
                ),
            ],
        ),
        // 4. Command-palette sequence hint — a conflicting binding that offers a
        //    resolve-conflicting-binding action alongside its next keys and example command, and a
        //    leader ready for its first palette-prefix key.
        base_row(
            Surface::CommandPaletteSequenceHint,
            Qual::Stable,
            "Command-palette sequence hint owner",
            "The command-palette sequence hint renders the shared sequence-help strip so a conflicting binding is shown honestly as ambiguous — offering a resolve-conflicting-binding action, showing the conflicting next keys, the cancel key, and an example command — and a leader ready for its first palette-prefix key shows the valid prefix keys, the cancel key, and an example command, so an ambiguous command-language sequence is always resolvable in-product",
            "evidence:m5-sequence-help-strip-command-palette-sequence-hint:001",
            vec![
                sequence_case(
                    State::ConflictingBinding,
                    Step::Chord,
                    Backing::PaletteEntry,
                    "Palette — prefix Ctrl+K",
                    &["1", "2"],
                    "Esc",
                    Some("command:palette.resolve-binding"),
                    "Two commands share Ctrl+K. Press 1 or 2 to choose, resolve the conflict, or press Escape to cancel.",
                    "cheatsheet:command-palette-sequences",
                    "strip:command-palette:ctrl-k-conflict",
                ),
                sequence_case(
                    State::Ready,
                    Step::LeaderKey,
                    Backing::PaletteEntry,
                    "Palette — prefix >",
                    &[">", "@", "#"],
                    "Esc",
                    Some("command:palette.run-command"),
                    "Palette prefix ready. Press > to run a command, @ for symbols, # for lines. Press Escape to cancel.",
                    "cheatsheet:command-palette-sequences",
                    "strip:command-palette:prefix-ready",
                ),
            ],
        ),
        // 5. Support sequence export — an awaiting-next-key motion whose guidance survives the
        //    export, and a disabled operator with no command backing whose current mode, cancel
        //    key, and cheat-sheet route survive the export without leaking raw keystroke material.
        base_row(
            Surface::SupportSequenceExport,
            Qual::Stable,
            "Support sequence export owner",
            "The support sequence export renders the shared sequence-help strip so an awaiting-next-key motion exports its current mode, valid next keys, cancel key, and example command intact, and a disabled operator with no command backing exports honestly as disabled — keeping its current mode, cancel key, and cheat-sheet route — so support can reconstruct exactly what a keyboard-first user saw, with no raw keystroke log or buffer leaking across the boundary",
            "evidence:m5-sequence-help-strip-support-sequence-export:001",
            vec![
                sequence_case(
                    State::AwaitingNextKey,
                    Step::Motion,
                    Backing::KeybindingRoute,
                    "Normal — motion g",
                    &["gg", "G"],
                    "Esc",
                    Some("command:editor.go-to-line"),
                    "Motion g pending. Press gg to go to the top, G to the bottom. Press Escape to cancel.",
                    "cheatsheet:command-language",
                    "strip:support-export:go-line-motion",
                ),
                sequence_case(
                    State::DisabledInContext,
                    Step::Operator,
                    Backing::NoCommandBacking,
                    "Normal — operator > (indent)",
                    &[],
                    "Esc",
                    None,
                    "The indent operator is disabled in a read-only buffer. Press Escape to dismiss, or open the cheat sheet to learn more.",
                    "cheatsheet:command-language",
                    "strip:support-export:indent-disabled",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5SequenceHelpStripGovernanceReview {
    M5SequenceHelpStripGovernanceReview {
        sequence_strip_shows_current_mode_or_leader: true,
        sequence_strip_shows_valid_next_keys: true,
        sequence_strip_shows_cancel_key: true,
        sequence_strip_shows_example_command: true,
        sequence_strip_opens_full_cheat_sheet: true,
        partial_or_ambiguous_sequences_never_fail_silently: true,
        keyboard_first_users_learn_pathways_in_product: true,
        sequence_strip_never_requires_pointer_hover: true,
        sequence_strip_provides_screen_reader_announcement: true,
        sequence_strip_preserves_command_backing: true,
        sequence_strips_stable_across_deployment_lines: true,
        sequence_strips_stable_across_consumer_surfaces: true,
        every_sequence_strip_declares_accessibility_route: true,
        support_export_reconstructs_sequence_truth: true,
        later_rows_cannot_invent_parallel_sequence_vocabulary: true,
    }
}

fn consumer_projection() -> M5SequenceHelpStripConsumerProjection {
    M5SequenceHelpStripConsumerProjection {
        command_language_surfaces_consume_sequence_vocabulary: true,
        help_posture_reads_single_source: true,
        action_set_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5SequenceHelpStripProofFreshness {
    M5SequenceHelpStripProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SequenceHelpStripReleasePosture {
    M5SequenceHelpStripReleasePosture {
        release_packet_ref: M5_SEQUENCE_HELP_STRIP_ARTIFACT_REF.to_owned(),
        sequence_help_audit_ref: M5_SEQUENCE_HELP_STRIP_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SEQUENCE_HELP_STRIP_SCHEMA_REF,
        M5_SEQUENCE_HELP_STRIP_DOC_REF,
        M5_SEQUENCE_HELP_STRIP_COMPONENT_MATRIX_REF,
        M5_SEQUENCE_HELP_STRIP_KEYBINDING_RESOLVER_REF,
        M5_SEQUENCE_HELP_STRIP_COMMAND_DESCRIPTOR_REF,
    ])
}

/// Builds the canonical M5 sequence-help-strip packet.
pub fn seeded_m5_sequence_help_strip_packet() -> M5SequenceHelpStripPacket {
    M5SequenceHelpStripPacket::new(M5SequenceHelpStripPacketInput {
        packet_id: M5_SEQUENCE_HELP_STRIP_PACKET_ID.to_owned(),
        matrix_label:
            "M5 sequence-help-strip primitive: sequence-help state, sequence step kind, command-backing state, current-mode-or-leader reference, valid next keys, cancel key, example-command reference, screen-reader announcement, full-cheat-sheet reference, derived help posture (ready-for-input/awaiting-next-key/partial-sequence/unbound-dead-end/conflicting-binding/disabled-in-context), and bounded show-valid-next-keys/run-example-command/resolve-conflicting-binding/cancel-sequence/open-full-cheat-sheet actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5SequenceHelpStripVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the command-palette sequence hint consumer is held at Beta because a slice
/// of palette sequences does not yet render the step-kind cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_sequence_help_strip_command_palette_sequence_hint_beta_narrowed(
) -> M5SequenceHelpStripPacket {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.packet_id =
        "m5-sequence-help-strip-primitive:command-palette-sequence-hint-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.consumer_surface == M5SequenceHelpConsumerSurface::CommandPaletteSequenceHint
        })
        .expect("command-palette-sequence-hint row present");
    row.qualification = M5TeachingQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support sequence export consumer is narrowed to Preview pending
/// screen-reader-announcement parity proof across every deployment; every consumer stays visible.
pub fn seeded_m5_sequence_help_strip_support_sequence_export_preview_narrowed(
) -> M5SequenceHelpStripPacket {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.packet_id =
        "m5-sequence-help-strip-primitive:support-sequence-export-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SequenceHelpConsumerSurface::SupportSequenceExport)
        .expect("support-sequence-export row present");
    row.qualification = M5TeachingQualificationClass::Preview;
    packet
}
