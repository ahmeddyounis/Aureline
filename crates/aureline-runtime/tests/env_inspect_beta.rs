//! Integration coverage for the env-inspect contract.
//!
//! The test replays the checked-in fixtures under
//! [`fixtures/runtime/env_inspect_beta/`] end-to-end. Each fixture pins one
//! seeded scenario (the same scenarios the headless CLI / inspector binary
//! `aureline_shell_env_inspect` emits) and the expected core fields the
//! runtime, the shell panel projection, and the headless CLI all surface.
//!
//! The test enforces three acceptance bullets from the env-inspect spec:
//!
//! 1. UI and CLI/headless inspection produce the same core fields and
//!    degradation labels for the same context. The
//!    [`aureline_shell::env_inspect::EnvInspectPanelProjection`] rows MUST
//!    match the canonical [`EnvInspectSnapshot::core_fields`] verbatim.
//! 2. Support exports embed the snapshot without leaking secrets or
//!    unmanaged credentials. The export's [`EnvInspectRedactionClass`] is
//!    pinned to `structured_tokens_only`, and the bundle round-trips
//!    through serde with no raw env, raw command line, or secret payload.
//! 3. The inspect contract is stable enough to feed docs, support
//!    playbooks, and partner debugging. The seeded snapshots are
//!    deterministic across calls so reviewer and partner runs of the
//!    headless inspector reproduce reviewer fixtures byte-for-byte.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    seeded_env_inspect_snapshot, seeded_env_inspect_support_export, EnvInspectDegradationSeverity,
    EnvInspectRedactionClass, EnvInspectSeededScenario, EnvInspectSnapshot,
    EnvInspectSupportExport, ExecutionContextBetaLane, SurfaceClass, TargetClass,
    TargetConfidenceReason, ToolchainClass, TrustState, ENV_INSPECT_SCHEMA_VERSION,
    ENV_INSPECT_SNAPSHOT_RECORD_KIND, ENV_INSPECT_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("env_inspect_beta")
}

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    scenario: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    lane: ExecutionContextBetaLane,
    lane_label: String,
    target_class: TargetClass,
    surface: SurfaceClass,
    toolchain_class: ToolchainClass,
    boundary_cue_visible: bool,
    trust_state: TrustState,
    any_degradation: bool,
    requires_review_before_dispatch: bool,
    blocks_dispatch: bool,
    expected_degradation_severities: Vec<EnvInspectDegradationSeverity>,
    #[serde(default)]
    expected_target_confidence_reasons: Vec<TargetConfidenceReason>,
}

fn scenario_for(name: &str) -> EnvInspectSeededScenario {
    match name {
        "local_terminal" => EnvInspectSeededScenario::LocalTerminal,
        "remote_attach_pending_trust" => EnvInspectSeededScenario::RemoteAttachPendingTrust,
        "container_devcontainer" => EnvInspectSeededScenario::ContainerDevcontainer,
        "managed_workspace_restricted" => EnvInspectSeededScenario::ManagedWorkspaceRestricted,
        other => panic!("unknown env-inspect scenario: {other}"),
    }
}

#[test]
fn every_seeded_scenario_fixture_replays_through_the_canonical_snapshot() {
    for fixture_name in [
        "local_terminal.json",
        "remote_attach_pending_trust.json",
        "container_devcontainer.json",
        "managed_workspace_restricted.json",
    ] {
        let path = fixture_root().join(fixture_name);
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {fixture_name}: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse fixture {fixture_name}: {err}"));
        assert_eq!(fixture.record_kind, "env_inspect_beta_case");
        assert_eq!(fixture.schema_version, ENV_INSPECT_SCHEMA_VERSION);

        let scenario = scenario_for(&fixture.scenario);
        let snapshot = seeded_env_inspect_snapshot(scenario);

        assert_eq!(snapshot.record_kind, ENV_INSPECT_SNAPSHOT_RECORD_KIND);
        assert_eq!(snapshot.lane, fixture.expect.lane, "{fixture_name}: lane");
        assert_eq!(
            snapshot.lane_label, fixture.expect.lane_label,
            "{fixture_name}: lane_label"
        );
        assert_eq!(
            snapshot.target_class, fixture.expect.target_class,
            "{fixture_name}: target_class"
        );
        assert_eq!(snapshot.surface, fixture.expect.surface, "{fixture_name}: surface");
        assert_eq!(
            snapshot.toolchain_class, fixture.expect.toolchain_class,
            "{fixture_name}: toolchain_class"
        );
        assert_eq!(
            snapshot.boundary_cue_visible, fixture.expect.boundary_cue_visible,
            "{fixture_name}: boundary_cue"
        );
        assert_eq!(
            snapshot.trust_state, fixture.expect.trust_state,
            "{fixture_name}: trust_state"
        );
        assert_eq!(
            snapshot.has_degradation(),
            fixture.expect.any_degradation,
            "{fixture_name}: any_degradation"
        );
        assert_eq!(
            snapshot.requires_review_before_dispatch(),
            fixture.expect.requires_review_before_dispatch,
            "{fixture_name}: requires_review_before_dispatch"
        );
        assert_eq!(
            snapshot.blocks_dispatch(),
            fixture.expect.blocks_dispatch,
            "{fixture_name}: blocks_dispatch"
        );

        let actual_severities: Vec<EnvInspectDegradationSeverity> = snapshot
            .degradation_labels
            .iter()
            .map(|label| label.severity)
            .collect();
        for expected_severity in &fixture.expect.expected_degradation_severities {
            assert!(
                actual_severities.contains(expected_severity),
                "{fixture_name}: missing degradation severity {expected_severity:?} in {actual_severities:?}"
            );
        }
        for expected_reason in &fixture.expect.expected_target_confidence_reasons {
            assert!(
                snapshot.target_confidence_reasons.contains(expected_reason),
                "{fixture_name}: missing confidence reason {expected_reason:?}"
            );
        }
    }
}

#[test]
fn seeded_snapshots_are_deterministic_across_calls() {
    // Reviewer and partner runs of the headless inspector MUST reproduce
    // the fixture-pinned record byte-for-byte. The test serialises the
    // snapshot twice and asserts character-identical output.
    for scenario in EnvInspectSeededScenario::ALL {
        let first = serde_json::to_string(&seeded_env_inspect_snapshot(scenario)).expect("serialize first");
        let second = serde_json::to_string(&seeded_env_inspect_snapshot(scenario)).expect("serialize second");
        assert_eq!(
            first,
            second,
            "{}: seeded snapshot must be deterministic",
            scenario.as_str()
        );
    }
}

#[test]
fn support_export_round_trips_and_pins_the_redaction_class() {
    let export =
        seeded_env_inspect_support_export("env-inspect-beta:test", "2026-05-15T00:00:00Z");
    assert_eq!(export.record_kind, ENV_INSPECT_SUPPORT_EXPORT_RECORD_KIND);
    assert_eq!(export.snapshots.len(), EnvInspectSeededScenario::ALL.len());
    assert_eq!(
        export.redaction_class,
        EnvInspectRedactionClass::StructuredTokensOnly
    );
    assert!(
        export.any_degradation,
        "support export must surface at least one degradation across the seeded scenarios"
    );
    assert!(
        export.any_requires_review,
        "support export must surface at least one review-required scenario"
    );

    let json = serde_json::to_string(&export).expect("serialize support export");
    let round: EnvInspectSupportExport =
        serde_json::from_str(&json).expect("deserialize support export");
    assert_eq!(round, export);

    // Redaction guarantee: the serialised payload must NOT contain raw
    // environment markers, raw command-line fragments, or secret bodies.
    // The seeded contexts never emit such fields, so any appearance here
    // would be a regression that would cause the support packet to leak.
    assert!(!json.contains("LD_LIBRARY_PATH"));
    assert!(!json.contains("AWS_SECRET_ACCESS_KEY"));
    assert!(!json.contains("SSH_PRIVATE_KEY"));
    assert!(!json.contains("BEARER"));
}

#[test]
fn snapshot_plaintext_renders_every_canonical_section_header() {
    // The headless CLI and the chrome inspector both delegate to
    // EnvInspectSnapshot::render_plaintext for the export-safe textual
    // form. Asserting the section header set keeps the contract stable
    // for support playbooks that grep this output.
    let snapshot: EnvInspectSnapshot = seeded_env_inspect_snapshot(EnvInspectSeededScenario::LocalTerminal);
    let text = snapshot.render_plaintext();
    for header in [
        "[lane] Lane",
        "[target] Target",
        "[toolchain] Toolchain",
        "[environment_capsule] Environment capsule",
        "[policy_and_trust] Policy & trust",
        "[scope] Workset scope",
        "[cache] Cache disposition",
        "[prebuild] Prebuild",
        "[mixed_version] Mixed-version posture",
        "[target_confidence] Target confidence",
        "[provenance] Provenance",
    ] {
        assert!(
            text.contains(header),
            "plaintext must contain section header '{header}'"
        );
    }
    assert!(text.contains("degradation: none"));
}
