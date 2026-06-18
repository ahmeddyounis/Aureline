//! Fixture-driven coverage for the M5 macro-recorder session and replay object and
//! its first consumers: the checked-in packet matches the seed bit-for-bit, every
//! entrypoint binds a panel whose replay resolutions quote the recomputed replay
//! action, the worked-example export round-trips into an equal session, the
//! cross-scope macro fails closed pending promotion, the unsupported-command
//! recording blocks save, replay fails closed on a context mismatch, and every
//! mutation fixture reproduces the fail-closed promotion state the gate enforces.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_macro_recorder_first_consumers_input, seeded_cross_scope_promotion_session,
    seeded_macro_recorder_panel, seeded_macro_recorder_session, AutomationBaselinePromotionState,
    AutomationSafetyLabelId, CapturedCommand, CapturedCommandSupportClass, ContentAddress,
    MacroRecorderConsumerBinding, MacroRecorderFirstConsumersInput,
    MacroRecorderFirstConsumersPacket, MacroRecorderSession, MacroReplayActionClass,
    MacroReplayBlocker, MacroSessionExport, MacroStorageScopeClass, RecipeBuilderEntrypoint,
    RecordedSurfaceClass, ReplayPostureClass, MACRO_RECORDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
};
use serde::Deserialize;

const FIXTURE_DIR: &str = "fixtures/automation/m5/macro-recorder";

/// Each mutation fixture is (file-name, mutation).
const CASES: [(&str, &str); 11] = [
    ("macro_recorder_stable.json", "none"),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
    ),
    (
        "replay_implies_stale_context_blocks_stable.json",
        "replay_implies_stale_context",
    ),
    (
        "repository_content_defines_macro_blocks_stable.json",
        "repository_content_defines_macro",
    ),
    (
        "unsupported_command_not_blocked_blocks_stable.json",
        "unsupported_command_not_blocked",
    ),
    (
        "promotion_not_explicit_blocks_stable.json",
        "promotion_not_explicit",
    ),
    (
        "profile_local_default_violated_blocks_stable.json",
        "profile_local_default_violated",
    ),
    (
        "ambient_or_managed_only_capture_blocks_stable.json",
        "ambient_or_managed_only_capture",
    ),
    (
        "replay_resolution_projection_inconsistent_blocks_stable.json",
        "replay_resolution_projection_inconsistent",
    ),
    (
        "raw_secret_material_in_session_blocks_stable.json",
        "raw_secret_material_in_session",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
    ),
];

#[derive(Debug, Deserialize)]
struct CaseFixture {
    case_name: String,
    mutation: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    promotion_state: String,
    validation_finding_count: usize,
    expected_finding_kinds: Vec<String>,
    entrypoint_tokens: Vec<String>,
    is_stable: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let body = std::fs::read_to_string(path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn rebuild_binding(
    input: &mut MacroRecorderFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
    sessions: Vec<MacroRecorderSession>,
) {
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != entrypoint);
    input
        .consumer_bindings
        .push(MacroRecorderConsumerBinding::from_sessions(
            entrypoint,
            sessions,
            "mutated panel",
        ));
}

fn binding_mut(
    input: &mut MacroRecorderFirstConsumersInput,
    entrypoint: RecipeBuilderEntrypoint,
) -> &mut MacroRecorderConsumerBinding {
    input
        .consumer_bindings
        .iter_mut()
        .find(|binding| binding.entrypoint == entrypoint)
        .expect("entrypoint present")
}

fn unsupported_process_command() -> CapturedCommand {
    CapturedCommand {
        command_id: "capture:launch-process".to_owned(),
        surface_class: RecordedSurfaceClass::UiPanelOpenCloseState,
        support_class: CapturedCommandSupportClass::UnsupportedRunsProcess,
        replay_posture_class: ReplayPostureClass::ReplayUiOrEditorStateOnly,
        state_digest: ContentAddress {
            digest_algorithm: "sha256".to_owned(),
            digest_hex: "abababababababababababababababababababababababababababababababab00"
                .to_owned(),
            digest_size_bytes: 32,
        },
        captured_at: "2026-06-18T00:00:09Z".to_owned(),
        label: "Launch a process (unsupported in a macro)".to_owned(),
    }
}

fn mutated(mutation: &str) -> MacroRecorderFirstConsumersPacket {
    let mut input = current_macro_recorder_first_consumers_input();
    match mutation {
        "none" => {}
        "missing_entrypoint" => {
            input
                .consumer_bindings
                .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
        }
        "replay_implies_stale_context" => {
            let mut session = seeded_macro_recorder_session(RecipeBuilderEntrypoint::RequestApi);
            session
                .current_replay_blockers
                .push(MacroReplayBlocker::NoBlockerPresent);
            rebuild_binding(
                &mut input,
                RecipeBuilderEntrypoint::RequestApi,
                vec![session],
            );
        }
        "repository_content_defines_macro" => {
            let mut sessions = seeded_macro_recorder_panel(RecipeBuilderEntrypoint::Notebook);
            sessions[0].imported_from_repository_content = true;
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::Notebook, sessions);
        }
        "unsupported_command_not_blocked" => {
            let mut sessions = seeded_macro_recorder_panel(RecipeBuilderEntrypoint::Notebook);
            sessions[0]
                .captured_commands
                .push(unsupported_process_command());
            sessions[0].current_replay_blockers =
                vec![MacroReplayBlocker::UnsupportedCommandCaptured];
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::Notebook, sessions);
        }
        "promotion_not_explicit" => {
            let mut session = seeded_cross_scope_promotion_session();
            session.current_replay_blockers = vec![MacroReplayBlocker::NoBlockerPresent];
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::Package, vec![session]);
        }
        "profile_local_default_violated" => {
            let mut session = seeded_macro_recorder_session(RecipeBuilderEntrypoint::Notebook);
            session.storage_scope_class = MacroStorageScopeClass::PortableProfileExportOnly;
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::Notebook, vec![session]);
        }
        "ambient_or_managed_only_capture" => {
            let mut session = seeded_macro_recorder_session(RecipeBuilderEntrypoint::Notebook);
            session
                .projected_safety_labels
                .push(AutomationSafetyLabelId::ApprovalRequired);
            rebuild_binding(&mut input, RecipeBuilderEntrypoint::Notebook, vec![session]);
        }
        "replay_resolution_projection_inconsistent" => {
            binding_mut(&mut input, RecipeBuilderEntrypoint::Notebook).replay_resolutions[0]
                .replay_action_class = MacroReplayActionClass::BlockedKillSwitch;
        }
        "raw_secret_material_in_session" => {
            let mut session = seeded_macro_recorder_session(RecipeBuilderEntrypoint::RequestApi);
            session.captured_commands[0].command_id = "raw:plaintext-token".to_owned();
            rebuild_binding(
                &mut input,
                RecipeBuilderEntrypoint::RequestApi,
                vec![session],
            );
        }
        "invariant_violated" => {
            input
                .invariants
                .replay_fails_closed_when_context_or_scope_no_longer_matches = false;
        }
        other => panic!("unknown mutation {other}"),
    }
    MacroRecorderFirstConsumersPacket::materialize(input)
}

#[test]
fn checked_in_packet_matches_the_seed() {
    let seeded = aureline_runtime::seeded_macro_recorder_first_consumers_packet();
    let artifact: MacroRecorderFirstConsumersPacket =
        read_json(&repo_root().join(MACRO_RECORDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF));
    assert_eq!(
        artifact, seeded,
        "checked-in packet must be bit-for-bit derivable from the seed; regenerate the dump"
    );
    assert!(seeded.is_stable());
}

#[test]
fn every_entrypoint_binds_a_reviewable_panel() {
    let packet = aureline_runtime::seeded_macro_recorder_first_consumers_packet();
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let binding = packet
            .binding(entrypoint)
            .unwrap_or_else(|| panic!("missing binding for {}", entrypoint.as_str()));
        assert!(!binding.sessions.is_empty());
        assert_eq!(binding.replay_resolutions.len(), binding.sessions.len());
        for (session, resolution) in binding.sessions.iter().zip(&binding.replay_resolutions) {
            assert!(!session.session_id.is_empty());
            assert!(session.replay_consistent());
            assert!(session.promotion_consistent());
            assert!(session.profile_local_default_consistent());
            assert!(session.safety_labels_constrained());
            assert!(session.disposition_consistent());
            assert!(session.captures_are_opaque());
            // The replay resolution quotes the recomputed replay action.
            assert_eq!(
                resolution.replay_action_class,
                session.resolved_replay_class()
            );
            assert!(resolution.is_fail_closed_safe());
            // A macro is never imported from repository content.
            assert!(!session.imported_from_repository_content);
        }
    }
}

#[test]
fn active_recording_strip_and_review_are_consistent() {
    let packet = aureline_runtime::seeded_macro_recorder_first_consumers_packet();
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        for session in &packet.binding(entrypoint).unwrap().sessions {
            let strip = session.active_recording_strip();
            assert_eq!(
                strip.captured_command_count,
                session.captured_commands.len() as u32
            );
            assert_eq!(
                strip.supported_command_count + strip.unsupported_command_count,
                strip.captured_command_count
            );
            let review = session.captured_command_review();
            assert_eq!(review.command_rows.len(), session.captured_commands.len());
            assert_eq!(
                review.has_unsupported_command,
                session.has_unsupported_command()
            );
            // An unsupported command blocks save.
            assert_eq!(review.save_admissible, !review.has_unsupported_command);
            // A saved macro never carries an unsupported command.
            if session.disposition_class.mints_manifest() {
                assert!(!session.has_unsupported_command());
            }
        }
    }
}

#[test]
fn worked_example_export_round_trips() {
    let export: MacroSessionExport = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("macro_session_export_roundtrip.json"),
    );
    let imported = export.import();
    let reexported = imported.export(export.export_id.clone(), export.exported_at.clone());
    assert_eq!(
        reexported, export,
        "import then export must reproduce the checked-in export verbatim"
    );
    assert!(export.replay_and_scope_preserved());
}

#[test]
fn cross_scope_macro_requires_promotion() {
    #[derive(Debug, Deserialize)]
    struct CrossScope {
        crosses_scope: bool,
        replay_admissible: bool,
        replay_fails_closed_pending_promotion: bool,
        replay_and_scope_preserved: bool,
    }
    let demo: CrossScope = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("cross_scope_macro_requires_promotion.json"),
    );
    assert!(demo.crosses_scope);
    assert!(!demo.replay_admissible);
    assert!(demo.replay_fails_closed_pending_promotion);
    assert!(demo.replay_and_scope_preserved);
}

#[test]
fn unsupported_command_blocks_save() {
    #[derive(Debug, Deserialize)]
    struct Unsupported {
        has_unsupported_command: bool,
        save_admissible: bool,
        replay_fails_closed: bool,
        minted_no_manifest: bool,
    }
    let demo: Unsupported = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("unsupported_command_blocks_save.json"),
    );
    assert!(demo.has_unsupported_command);
    assert!(!demo.save_admissible);
    assert!(demo.replay_fails_closed);
    assert!(demo.minted_no_manifest);
}

#[test]
fn replay_fails_closed_on_context_mismatch() {
    #[derive(Debug, Deserialize)]
    struct FailClosed {
        admissible: bool,
        fails_closed: bool,
        declares_target_scope: bool,
        refuses_on_context_mismatch: bool,
        reresolves_supported_command_set: bool,
        replay_action_class: String,
    }
    let demo: FailClosed = read_json(
        &repo_root()
            .join(FIXTURE_DIR)
            .join("replay_fails_closed_on_context_mismatch.json"),
    );
    assert!(!demo.admissible);
    assert!(demo.fails_closed);
    assert!(demo.declares_target_scope);
    assert!(demo.refuses_on_context_mismatch);
    assert!(demo.reresolves_supported_command_set);
    assert_eq!(
        demo.replay_action_class,
        "macro_replay_blocked_supported_command_set_changed"
    );
}

#[test]
fn repository_imported_macro_never_replays() {
    let mut session = seeded_macro_recorder_session(RecipeBuilderEntrypoint::Notebook);
    session.imported_from_repository_content = true;
    assert_eq!(
        session.resolved_replay_class(),
        MacroReplayActionClass::BlockedImportedFromRepositoryContent
    );
    assert!(!session.replay_admissible());
}

#[test]
fn mutation_fixtures_reproduce_promotion_states() {
    for (file_name, mutation) in CASES {
        let fixture: CaseFixture = read_json(&repo_root().join(FIXTURE_DIR).join(file_name));
        assert_eq!(fixture.mutation, mutation);
        let packet = mutated(mutation);
        assert_eq!(
            packet.promotion_state.as_str(),
            fixture.expect.promotion_state,
            "{} promotion mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.validation_findings.len(),
            fixture.expect.validation_finding_count,
            "{} finding count mismatch",
            fixture.case_name
        );
        let kinds: Vec<String> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str().to_owned())
            .collect();
        assert_eq!(
            kinds, fixture.expect.expected_finding_kinds,
            "{} finding kinds mismatch",
            fixture.case_name
        );
        assert_eq!(packet.entrypoint_tokens(), fixture.expect.entrypoint_tokens);
        assert_eq!(packet.is_stable(), fixture.expect.is_stable);
        if file_name == "macro_recorder_stable.json" {
            assert_eq!(
                packet.promotion_state,
                AutomationBaselinePromotionState::Stable
            );
        } else {
            assert_eq!(
                packet.promotion_state,
                AutomationBaselinePromotionState::BlocksStable
            );
        }
    }
}
