//! Headless inspector and regenerator for the M5 macro-recorder session and
//! replay object and its first consumers.
//!
//! Running the example with no argument regenerates the checked-in macro-recorder
//! artifacts and the worked-example macro-recorder fixtures from the frozen seed:
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_macro_recorder
//! ```
//!
//! With a subcommand it prints one projection to stdout (used for review and
//! support drills):
//!
//! ```sh
//! cargo run -q -p aureline-runtime --example dump_m5_macro_recorder -- packet
//! cargo run -q -p aureline-runtime --example dump_m5_macro_recorder -- support-export
//! cargo run -q -p aureline-runtime --example dump_m5_macro_recorder -- cli-headless
//! cargo run -q -p aureline-runtime --example dump_m5_macro_recorder -- compact
//! cargo run -q -p aureline-runtime --example dump_m5_macro_recorder -- validate
//! ```

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_macro_recorder_first_consumers_input, seeded_cross_scope_promotion_session,
    seeded_macro_recorder_first_consumers_packet, seeded_macro_recorder_panel,
    seeded_macro_recorder_session, seeded_macro_session_export_roundtrip,
    seeded_unsupported_command_session, AutomationSafetyLabelId, CapturedCommand,
    CapturedCommandSupportClass, MacroRecorderConsumerBinding, MacroRecorderFirstConsumersInput,
    MacroRecorderFirstConsumersPacket, MacroRecorderSession, MacroReplayActionClass,
    MacroReplayBlocker, MacroStorageScopeClass, RecipeBuilderEntrypoint, RecordedSurfaceClass,
    ReplayPostureClass, MACRO_RECORDER_FIRST_CONSUMERS_CLI_HEADLESS_ID,
    MACRO_RECORDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF,
    MACRO_RECORDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
};
use serde_json::json;

const ARTIFACT_DIR: &str = "artifacts/m5/automation/macro-recorder";
const FIXTURE_DIR: &str = "fixtures/automation/m5/macro-recorder";

/// Each mutation case is (file-name, mutation, scenario).
const CASES: [(&str, &str, &str); 11] = [
    (
        "macro_recorder_stable.json",
        "none",
        "Every entrypoint binds a session panel whose sessions declare target and storage scope, recorded macros stay profile-local, unsupported commands are flagged and block save, replay fails closed on a context or scope mismatch, repository content defines no macro, cross-scope promotion is explicit, and captures stay UI-only, so the packet is stable.",
    ),
    (
        "missing_entrypoint_blocks_stable.json",
        "missing_entrypoint",
        "The package entrypoint is dropped, so a later surface could render a package macro panel with no canonical macro-recorder object; the packet blocks stable.",
    ),
    (
        "replay_implies_stale_context_blocks_stable.json",
        "replay_implies_stale_context",
        "The request macro's blockers add no_blocker_present while the supported-command set still changed, implying the captured context is still authoritative; the packet blocks stable.",
    ),
    (
        "repository_content_defines_macro_blocks_stable.json",
        "repository_content_defines_macro",
        "The notebook macro is marked imported from repository content, so repository content would silently define an executable macro in the user profile; the packet blocks stable.",
    ),
    (
        "unsupported_command_not_blocked_blocks_stable.json",
        "unsupported_command_not_blocked",
        "The notebook macro is saved with an unsupported process-launch command captured into it, so an unsupported command did not block save; the packet blocks stable.",
    ),
    (
        "promotion_not_explicit_blocks_stable.json",
        "promotion_not_explicit",
        "The package macro crosses files but drops its cross-scope promotion blocker, so a cross-file macro would replay with a widened reach instead of being promoted to a recipe; the packet blocks stable.",
    ),
    (
        "profile_local_default_violated_blocks_stable.json",
        "profile_local_default_violated",
        "The notebook macro's resident storage is an export scope instead of profile-local, so a macro would not be profile-local by default; the packet blocks stable.",
    ),
    (
        "ambient_or_managed_only_capture_blocks_stable.json",
        "ambient_or_managed_only_capture",
        "The notebook macro projects an approval_required label outside the macro_safe / ui_only subset, so the macro would carry ambient authority; the packet blocks stable.",
    ),
    (
        "replay_resolution_projection_inconsistent_blocks_stable.json",
        "replay_resolution_projection_inconsistent",
        "The notebook panel's projected replay resolution quotes a replay action that disagrees with the live session, so a reviewer could trust a stale resolution; the packet blocks stable.",
    ),
    (
        "raw_secret_material_in_session_blocks_stable.json",
        "raw_secret_material_in_session",
        "The request macro carries a raw command reference instead of an opaque handle, turning the recording into a shadow secret store; the packet blocks stable.",
    ),
    (
        "invariant_violated_blocks_stable.json",
        "invariant_violated",
        "The replay-fails-closed invariant is set false, so a surface could replay a macro whose context no longer matches; the packet blocks stable.",
    ),
];

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let packet = seeded_macro_recorder_first_consumers_packet();

    match std::env::args().nth(1).as_deref() {
        None => regenerate(&root, &packet),
        Some("packet") => print_json(&packet),
        Some("support-export") => print_json(&packet.support_export(
            MACRO_RECORDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
            exported_at(),
        )),
        Some("cli-headless") => print_json(&packet.cli_headless_view(
            MACRO_RECORDER_FIRST_CONSUMERS_CLI_HEADLESS_ID,
            exported_at(),
        )),
        Some("compact") => {
            for line in packet.compact_lines() {
                println!("{line}");
            }
        }
        Some("validate") => match packet.validate() {
            findings if findings.is_empty() => println!("ok"),
            findings => {
                for finding in &findings {
                    eprintln!("error: {}", finding.finding_kind.as_str());
                }
                std::process::exit(3);
            }
        },
        Some(other) => {
            eprintln!("unknown subcommand: {other}");
            std::process::exit(2);
        }
    }
}

fn regenerate(root: &Path, packet: &MacroRecorderFirstConsumersPacket) {
    // First-consumers packet and its projections.
    write_json(
        &root.join(MACRO_RECORDER_FIRST_CONSUMERS_PACKET_ARTIFACT_REF),
        packet,
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("support_export.json"),
        &packet.support_export(
            MACRO_RECORDER_FIRST_CONSUMERS_SUPPORT_EXPORT_ID,
            exported_at(),
        ),
    );
    write_json(
        &root.join(ARTIFACT_DIR).join("cli_headless.json"),
        &packet.cli_headless_view(
            MACRO_RECORDER_FIRST_CONSUMERS_CLI_HEADLESS_ID,
            exported_at(),
        ),
    );
    let compact = packet.compact_lines().join("\n");
    write_text(&root.join(ARTIFACT_DIR).join("compact.txt"), &compact);

    // Worked-example fixtures.
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("macro_session_export_roundtrip.json"),
        &seeded_macro_session_export_roundtrip(),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("cross_scope_macro_requires_promotion.json"),
        &cross_scope_demonstration(),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("unsupported_command_blocks_save.json"),
        &unsupported_demonstration(),
    );
    write_json(
        &root
            .join(FIXTURE_DIR)
            .join("replay_fails_closed_on_context_mismatch.json"),
        &fail_closed_demonstration(),
    );

    // Mutation fixtures (the fail-closed gate cases).
    for (file_name, mutation, scenario) in CASES {
        let case_name = file_name.trim_end_matches(".json");
        let mutated = MacroRecorderFirstConsumersPacket::materialize(mutated_input(mutation));
        let fixture = json!({
            "record_kind": "m5_macro_recorder_case",
            "schema_version": 1,
            "case_name": case_name,
            "scenario": scenario,
            "mutation": mutation,
            "expect": {
                "promotion_state": mutated.promotion_state.as_str(),
                "validation_finding_count": mutated.validation_findings.len(),
                "expected_finding_kinds": mutated
                    .validation_findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>(),
                "entrypoint_tokens": mutated.entrypoint_tokens(),
                "is_stable": mutated.is_stable(),
            }
        });
        write_json(&root.join(FIXTURE_DIR).join(file_name), &fixture);
    }
}

/// Demonstrates that a macro's replay and scope truth survive export, history, and
/// support: the resolved replay comes through the resolution and a re-import
/// unchanged, the resolution declares scope and refuses unsafe reuse, and the macro
/// stays comparable across surfaces.
fn cross_scope_demonstration() -> serde_json::Value {
    let session = seeded_cross_scope_promotion_session();
    let resolved = session.resolved_replay_class();
    let export = session.export("export:bump-versions:v1", "2026-06-18T00:03:00Z");
    json!({
        "record_kind": "macro_cross_scope_promotion_demonstration",
        "schema_version": 1,
        "session_id": session.session_id,
        "declared_target_scope_class": session.declared_target_scope_class.as_str(),
        "crosses_scope": session.declared_target_scope_class.requires_promotion(),
        "promotion_affordance_class": session.promotion_affordance_class.as_str(),
        "replay_action_class": resolved.as_str(),
        "replay_admissible": resolved.is_admissible(),
        "replay_fails_closed_pending_promotion": resolved == MacroReplayActionClass::BlockedPromotionRequired,
        "replay_and_scope_preserved": export.replay_and_scope_preserved(),
        "export_digest": export.export_digest,
    })
}

/// Demonstrates that an unsupported command is flagged and blocks save.
fn unsupported_demonstration() -> serde_json::Value {
    let session = seeded_unsupported_command_session();
    let review = session.captured_command_review();
    let resolved = session.resolved_replay_class();
    json!({
        "record_kind": "macro_unsupported_command_demonstration",
        "schema_version": 1,
        "session_id": session.session_id,
        "disposition_class": session.disposition_class.as_str(),
        "has_unsupported_command": review.has_unsupported_command,
        "unsupported_command_count": review.unsupported_command_count,
        "save_admissible": review.save_admissible,
        "unsupported_warnings": review
            .unsupported_command_warnings
            .iter()
            .map(|warning| warning.support_class.as_str())
            .collect::<Vec<_>>(),
        "replay_action_class": resolved.as_str(),
        "replay_fails_closed": resolved.is_fail_closed(),
        "minted_no_manifest": session.resulting_macro_manifest_ref.is_none(),
    })
}

/// Demonstrates that replay fails closed when the supported-command set drifts.
fn fail_closed_demonstration() -> serde_json::Value {
    let session = seeded_macro_recorder_session(RecipeBuilderEntrypoint::RequestApi);
    let resolution = session.resolve_replay("2026-06-18T00:04:00Z");
    json!({
        "record_kind": "macro_replay_fail_closed_demonstration",
        "schema_version": 1,
        "session_id": session.session_id,
        "declared_target_scope_class": resolution.declared_target_scope_class.as_str(),
        "replay_action_class": resolution.replay_action_class.as_str(),
        "admissible": resolution.admissible,
        "fails_closed": resolution.fails_closed,
        "declares_target_scope": resolution.declares_target_scope,
        "refuses_on_context_mismatch": resolution.refuses_on_context_mismatch,
        "reresolves_supported_command_set": resolution.reresolves_supported_command_set,
        "current_replay_blockers": resolution
            .current_replay_blockers
            .iter()
            .map(|blocker| blocker.as_str())
            .collect::<Vec<_>>(),
    })
}

fn mutated_input(mutation: &str) -> MacroRecorderFirstConsumersInput {
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
    input
}

fn unsupported_process_command() -> CapturedCommand {
    CapturedCommand {
        command_id: "capture:launch-process".to_owned(),
        surface_class: RecordedSurfaceClass::UiPanelOpenCloseState,
        support_class: CapturedCommandSupportClass::UnsupportedRunsProcess,
        replay_posture_class: ReplayPostureClass::ReplayUiOrEditorStateOnly,
        state_digest: aureline_runtime::ContentAddress {
            digest_algorithm: "sha256".to_owned(),
            digest_hex: "abababababababababababababababababababababababababababababababab00"
                .to_owned(),
            digest_size_bytes: 32,
        },
        captured_at: "2026-06-18T00:00:09Z".to_owned(),
        label: "Launch a process (unsupported in a macro)".to_owned(),
    }
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

fn exported_at() -> &'static str {
    "2026-06-18T00:01:00Z"
}

fn print_json(value: &impl serde::Serialize) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).expect("serialize JSON")
    );
}

fn write_json(path: &PathBuf, value: &impl serde::Serialize) {
    ensure_parent(path);
    let payload = serde_json::to_string_pretty(value).expect("serialize JSON");
    std::fs::write(path, format!("{payload}\n")).expect("write JSON");
}

fn write_text(path: &PathBuf, body: &str) {
    ensure_parent(path);
    std::fs::write(path, format!("{body}\n")).expect("write text");
}

fn ensure_parent(path: &Path) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create artifact directory");
    }
}
