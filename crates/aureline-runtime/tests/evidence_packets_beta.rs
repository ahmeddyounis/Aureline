//! End-to-end coverage for the runtime evidence packet contract.
//!
//! Each checked-in fixture under
//! [`fixtures/runtime/m3/evidence_packets/`] pins one seeded scenario,
//! the expected lane / evidence kind, and the expected replay comparator
//! outcome. The test replays the seed builder, asserts the comparator
//! decision matches the fixture, and round-trips the support-export
//! bundle through serde with the pinned redaction class.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    seeded_runtime_evidence_packet, seeded_runtime_evidence_packet_support_export,
    ExecutionProvenanceRedactionClass, ReplayCompatibilityClass, ReplayIncompatibilityReason,
    RuntimeEvidenceKind, RuntimeEvidenceLane, RuntimeEvidencePacketSeededScenario,
    RuntimeEvidencePacketSupportExport, RUNTIME_EVIDENCE_PACKET_RECORD_KIND,
    RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION, RUNTIME_EVIDENCE_PACKET_SUPPORT_EXPORT_RECORD_KIND,
    RUNTIME_EVIDENCE_REPLAY_COMPARISON_RECORD_KIND,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("m3")
        .join("evidence_packets")
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
    lane: RuntimeEvidenceLane,
    evidence_kind: RuntimeEvidenceKind,
    compatibility: ReplayCompatibilityClass,
    permits_replay_without_review: bool,
    blocks_replay: bool,
    required_incompatibility_reasons: Vec<ReplayIncompatibilityReason>,
}

fn scenario_for(name: &str) -> RuntimeEvidencePacketSeededScenario {
    match name {
        "local_task_compatible" => RuntimeEvidencePacketSeededScenario::LocalTaskCompatible,
        "local_test_policy_advanced_clean" => {
            RuntimeEvidencePacketSeededScenario::LocalTestPolicyAdvancedClean
        }
        "container_debug_capsule_drift" => {
            RuntimeEvidencePacketSeededScenario::ContainerDebugCapsuleDrift
        }
        "managed_runtime_trust_downgraded" => {
            RuntimeEvidencePacketSeededScenario::ManagedRuntimeTrustDowngraded
        }
        other => panic!("unknown evidence packet scenario: {other}"),
    }
}

#[test]
fn every_seeded_scenario_fixture_replays_through_the_seed_builder() {
    for fixture_name in [
        "local_task_compatible.json",
        "local_test_policy_advanced_clean.json",
        "container_debug_capsule_drift.json",
        "managed_runtime_trust_downgraded.json",
    ] {
        let path = fixture_root().join(fixture_name);
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {fixture_name}: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse fixture {fixture_name}: {err}"));
        assert_eq!(fixture.record_kind, "runtime_evidence_packet_case");
        assert_eq!(
            fixture.schema_version,
            RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION
        );

        let scenario = scenario_for(&fixture.scenario);
        let (packet, comparison) = seeded_runtime_evidence_packet(scenario);

        assert_eq!(packet.record_kind, RUNTIME_EVIDENCE_PACKET_RECORD_KIND);
        assert_eq!(
            packet.lane, fixture.expect.lane,
            "{fixture_name}: lane mismatch"
        );
        assert_eq!(
            packet.evidence_kind, fixture.expect.evidence_kind,
            "{fixture_name}: evidence_kind mismatch"
        );
        assert!(packet.redaction_safe, "{fixture_name}: redaction_safe");
        assert!(matches!(
            packet.redaction_class,
            ExecutionProvenanceRedactionClass::MetadataSafeDefault
        ));

        assert_eq!(
            comparison.record_kind,
            RUNTIME_EVIDENCE_REPLAY_COMPARISON_RECORD_KIND
        );
        assert_eq!(
            comparison.compatibility, fixture.expect.compatibility,
            "{fixture_name}: compatibility class mismatch"
        );
        assert_eq!(
            comparison.permits_replay_without_review,
            fixture.expect.permits_replay_without_review,
            "{fixture_name}: permits_replay_without_review"
        );
        assert_eq!(
            comparison.blocks_replay, fixture.expect.blocks_replay,
            "{fixture_name}: blocks_replay"
        );
        for reason in &fixture.expect.required_incompatibility_reasons {
            assert!(
                comparison.incompatibility_reasons.contains(reason),
                "{fixture_name}: comparator missing reason {reason:?}; got {:?}",
                comparison.incompatibility_reasons
            );
        }
    }
}

#[test]
fn support_export_round_trips_and_pins_redaction_class() {
    let export = seeded_runtime_evidence_packet_support_export(
        "evpkt-support:integration",
        "2026-05-15T19:02:00Z",
    );
    assert_eq!(
        export.record_kind,
        RUNTIME_EVIDENCE_PACKET_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(
        export.packets.len(),
        RuntimeEvidencePacketSeededScenario::ALL.len()
    );
    assert_eq!(
        export.comparisons.len(),
        RuntimeEvidencePacketSeededScenario::ALL.len()
    );
    assert!(matches!(
        export.redaction_class,
        ExecutionProvenanceRedactionClass::MetadataSafeDefault
    ));
    assert!(export.redaction_safe);
    assert!(export.any_comparison_blocks_replay);
    assert!(export.any_minor_drift);

    let json = serde_json::to_string(&export).expect("serialize export");
    let round: RuntimeEvidencePacketSupportExport =
        serde_json::from_str(&json).expect("deserialize export");
    assert_eq!(round, export);

    // The serialised export must NOT carry raw secret markers; this guards
    // against accidentally adding a downstream field that copies env
    // bodies or credentials into the evidence packet.
    assert!(!json.contains("BEARER"));
    assert!(!json.contains("AWS_SECRET_ACCESS_KEY"));
    assert!(!json.contains("SSH_PRIVATE_KEY"));
    assert!(!json.contains("LD_LIBRARY_PATH"));
}

#[test]
fn support_export_bundles_one_packet_per_lane() {
    let export = seeded_runtime_evidence_packet_support_export(
        "evpkt-support:lane-coverage",
        "2026-05-15T19:02:00Z",
    );
    let mut lanes: Vec<RuntimeEvidenceLane> = export.packets.iter().map(|p| p.lane).collect();
    lanes.sort();
    lanes.dedup();
    for expected in RuntimeEvidenceLane::ALL {
        assert!(
            lanes.contains(&expected),
            "support export must cover lane {expected:?}"
        );
    }
}

#[test]
fn support_export_plaintext_quotes_closed_vocabularies() {
    let export = seeded_runtime_evidence_packet_support_export(
        "evpkt-support:plaintext",
        "2026-05-15T19:02:00Z",
    );
    let text = export.render_plaintext();
    for token in [
        "compatible_replay",
        "compatible_minor_drift",
        "incompatible_capsule_drift",
        "incompatible_trust_state_downgraded",
        "metadata_safe_default",
        "task_event",
        "test_attempt",
        "debug_session",
        "runtime_trace_evidence",
    ] {
        assert!(
            text.contains(token),
            "plaintext must quote closed token '{token}'"
        );
    }
}

#[test]
fn seeded_export_is_deterministic_across_calls() {
    let first = serde_json::to_string(&seeded_runtime_evidence_packet_support_export(
        "evpkt-support:deterministic",
        "2026-05-15T19:02:00Z",
    ))
    .expect("first");
    let second = serde_json::to_string(&seeded_runtime_evidence_packet_support_export(
        "evpkt-support:deterministic",
        "2026-05-15T19:02:00Z",
    ))
    .expect("second");
    assert_eq!(first, second);
}
