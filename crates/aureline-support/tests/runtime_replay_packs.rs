//! End-to-end coverage for the runtime replay-pack contract.
//!
//! Each checked-in fixture under
//! [`fixtures/runtime/m3/replay_packets/`] pins one seeded scenario, the
//! expected fidelity / privilege / reopen-decision outcome, and whether the
//! pack covers the required artefact classes. The test replays the seeded
//! builder, asserts the outcome matches the fixture, and round-trips the
//! support-export bundle through serde with the pinned redaction posture.

use std::path::{Path, PathBuf};

use aureline_support::runtime_evidence::{
    seeded_runtime_replay_pack, seeded_runtime_replay_pack_support_export, ReplayFidelityClass,
    ReplayReopenDecisionClass, ReplaySubjectPrivilegeClass, RuntimeReplayPackSeededScenario,
    RuntimeReplayPackSupportExport, RUNTIME_REPLAY_PACK_RECORD_KIND,
    RUNTIME_REPLAY_PACK_SCHEMA_VERSION, RUNTIME_REPLAY_PACK_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("m3")
        .join("replay_packets")
}

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    #[allow(dead_code)]
    case_id: String,
    scenario: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    fidelity: ReplayFidelityClass,
    subject_privilege: ReplaySubjectPrivilegeClass,
    reopen_decision: ReplayReopenDecisionClass,
    forbids_live_rerun: bool,
    comparator_blocks_replay: bool,
    covers_required_artefact_classes: bool,
}

fn scenario_for(name: &str) -> RuntimeReplayPackSeededScenario {
    match name {
        "local_task_exact_read_only" => RuntimeReplayPackSeededScenario::LocalTaskExactReadOnly,
        "local_test_compatible_read_only" => {
            RuntimeReplayPackSeededScenario::LocalTestCompatibleReadOnly
        }
        "container_debug_layout_only_mutating" => {
            RuntimeReplayPackSeededScenario::ContainerDebugLayoutOnlyMutating
        }
        "managed_runtime_layout_only_privileged" => {
            RuntimeReplayPackSeededScenario::ManagedRuntimeLayoutOnlyPrivileged
        }
        other => panic!("unknown replay-pack scenario: {other}"),
    }
}

#[test]
fn every_seeded_replay_pack_fixture_replays_through_the_seed_builder() {
    for fixture_name in [
        "local_task_exact_read_only.json",
        "local_test_compatible_read_only.json",
        "container_debug_layout_only_mutating.json",
        "managed_runtime_layout_only_privileged.json",
    ] {
        let path = fixture_root().join(fixture_name);
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {fixture_name}: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse fixture {fixture_name}: {err}"));
        assert_eq!(fixture.record_kind, "runtime_replay_pack_case");
        assert_eq!(fixture.schema_version, RUNTIME_REPLAY_PACK_SCHEMA_VERSION);

        let scenario = scenario_for(&fixture.scenario);
        let pack = seeded_runtime_replay_pack(scenario);

        assert_eq!(pack.record_kind, RUNTIME_REPLAY_PACK_RECORD_KIND);
        assert_eq!(
            pack.fidelity, fixture.expect.fidelity,
            "{fixture_name}: fidelity mismatch",
        );
        assert_eq!(
            pack.subject_privilege, fixture.expect.subject_privilege,
            "{fixture_name}: subject_privilege mismatch",
        );
        assert_eq!(
            pack.reopen_decision, fixture.expect.reopen_decision,
            "{fixture_name}: reopen_decision mismatch",
        );
        assert_eq!(
            pack.forbids_live_rerun, fixture.expect.forbids_live_rerun,
            "{fixture_name}: forbids_live_rerun mismatch",
        );
        assert_eq!(
            pack.comparator_blocks_replay, fixture.expect.comparator_blocks_replay,
            "{fixture_name}: comparator_blocks_replay mismatch",
        );
        assert_eq!(
            pack.covers_required_artefact_classes(),
            fixture.expect.covers_required_artefact_classes,
            "{fixture_name}: covers_required_artefact_classes mismatch",
        );
    }
}

#[test]
fn support_export_round_trips_and_pins_metadata_safe_default() {
    let export = seeded_runtime_replay_pack_support_export(
        "replay-pack-support:integration",
        "2026-05-15T19:03:00Z",
    );
    assert_eq!(
        export.record_kind,
        RUNTIME_REPLAY_PACK_SUPPORT_EXPORT_RECORD_KIND,
    );
    assert_eq!(
        export.packs.len(),
        RuntimeReplayPackSeededScenario::ALL.len(),
    );
    assert!(export.every_pack_covers_required_artefact_classes);
    assert!(export.any_pack_forbids_live_rerun);
    assert!(export.any_pack_comparator_blocks_replay);

    let json = serde_json::to_string(&export).expect("serialize export");
    let round: RuntimeReplayPackSupportExport =
        serde_json::from_str(&json).expect("deserialize export");
    assert_eq!(round, export);

    // The bundled evidence packets pin metadata_safe_default; the artefact
    // refs are opaque digests. Guard against accidental field additions that
    // would leak raw env or credentials into the export.
    assert!(!json.contains("BEARER"));
    assert!(!json.contains("AWS_SECRET_ACCESS_KEY"));
    assert!(!json.contains("SSH_PRIVATE_KEY"));
    assert!(!json.contains("LD_LIBRARY_PATH"));
}

#[test]
fn support_export_quotes_every_closed_fidelity_and_reopen_token() {
    let export = seeded_runtime_replay_pack_support_export(
        "replay-pack-support:lane-coverage",
        "2026-05-15T19:03:00Z",
    );
    for token in ["exact", "compatible", "layout_only"] {
        assert!(
            export
                .fidelity_class_tokens_present
                .contains(&token.to_owned()),
            "fidelity coverage missing token '{token}'",
        );
    }
    for token in ["allow_replay", "allow_inspect_no_rerun"] {
        assert!(
            export
                .reopen_decision_tokens_present
                .contains(&token.to_owned()),
            "reopen coverage missing token '{token}'",
        );
    }
}

#[test]
fn mutating_and_privileged_subjects_are_gated_to_inspect_only() {
    let export = seeded_runtime_replay_pack_support_export(
        "replay-pack-support:privilege-gating",
        "2026-05-15T19:03:00Z",
    );
    for pack in &export.packs {
        if matches!(
            pack.subject_privilege,
            ReplaySubjectPrivilegeClass::Mutating | ReplaySubjectPrivilegeClass::Privileged
        ) {
            assert_ne!(
                pack.reopen_decision,
                ReplayReopenDecisionClass::AllowReplay,
                "mutating/privileged subject must NEVER resolve to allow_replay (pack={})",
                pack.replay_pack_id,
            );
            assert!(
                pack.forbids_live_rerun,
                "mutating/privileged subject must forbid live rerun (pack={})",
                pack.replay_pack_id,
            );
        }
    }
}

#[test]
fn plaintext_quotes_every_seeded_scenario_id() {
    let export = seeded_runtime_replay_pack_support_export(
        "replay-pack-support:plaintext",
        "2026-05-15T19:03:00Z",
    );
    let text = export.render_plaintext();
    for scenario in RuntimeReplayPackSeededScenario::ALL {
        assert!(
            text.contains(scenario.as_str()),
            "plaintext must quote scenario id '{}'",
            scenario.as_str(),
        );
    }
}

#[test]
fn seeded_export_is_deterministic_across_calls() {
    let first = serde_json::to_string(&seeded_runtime_replay_pack_support_export(
        "replay-pack-support:deterministic",
        "2026-05-15T19:03:00Z",
    ))
    .expect("first");
    let second = serde_json::to_string(&seeded_runtime_replay_pack_support_export(
        "replay-pack-support:deterministic",
        "2026-05-15T19:03:00Z",
    ))
    .expect("second");
    assert_eq!(first, second);
}
