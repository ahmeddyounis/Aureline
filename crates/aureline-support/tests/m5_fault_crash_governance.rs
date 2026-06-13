//! Protected tests for the M5 fault/crash governance packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::{
    seeded_m5_fault_crash_governance_packet, ClaimStateClass, CrashArtifactClass,
    DiagnosticSignalClass, DowngradeTriggerClass, FaultDomainClass, M5FaultCrashGovernancePacket,
    RestartClass, CRASH_STORE_VIEWER_SCHEMA_REF, M5_FAULT_CRASH_GOVERNANCE_ARTIFACT_REF,
    M5_FAULT_CRASH_GOVERNANCE_DOC_REF, M5_FAULT_CRASH_GOVERNANCE_FIXTURE_DIR,
    M5_FAULT_CRASH_GOVERNANCE_PACKET_RECORD_KIND, M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF,
    M5_FAULT_CRASH_GOVERNANCE_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(M5_FAULT_CRASH_GOVERNANCE_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5FaultCrashGovernancePacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_covers_all_fault_domains_and_restart_classes() {
    let packet = seeded_m5_fault_crash_governance_packet();
    assert_eq!(
        packet.record_kind,
        M5_FAULT_CRASH_GOVERNANCE_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_FAULT_CRASH_GOVERNANCE_SCHEMA_VERSION
    );
    assert_eq!(packet.doc_ref, M5_FAULT_CRASH_GOVERNANCE_DOC_REF);
    assert_eq!(packet.schema_ref, M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF);
    assert!(
        packet
            .supporting_contract_refs
            .contains(&CRASH_STORE_VIEWER_SCHEMA_REF.to_owned()),
        "governance packet must cite the crash-store viewer contract"
    );

    let fault_domains = packet
        .fault_domains
        .iter()
        .map(|row| row.fault_domain_class)
        .collect::<Vec<_>>();
    for required in FaultDomainClass::ALL {
        assert!(
            fault_domains.contains(&required),
            "missing fault domain {}",
            required.as_str()
        );
    }

    let restart_classes = packet
        .restart_classes
        .iter()
        .map(|row| row.restart_class)
        .collect::<Vec<_>>();
    for required in RestartClass::ALL {
        assert!(
            restart_classes.contains(&required),
            "missing restart class {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_required_m5_host_families() {
    let packet = seeded_m5_fault_crash_governance_packet();
    for required in [
        "notebook_kernel_host",
        "data_api_connector_host",
        "preview_dev_server_host",
        "provider_run_session_host",
        "profiler_replay_session_host",
        "pipeline_viewer_host",
    ] {
        assert!(
            packet
                .host_families
                .iter()
                .any(|row| row.host_family_id == required),
            "missing required host family {required}",
        );
    }
}

#[test]
fn crash_artifact_rows_enforce_local_first_exact_build_and_no_auto_upload() {
    let packet = seeded_m5_fault_crash_governance_packet();
    for row in &packet.crash_artifacts {
        assert!(
            row.auto_upload_forbidden,
            "{} must forbid automatic upload",
            row.artifact_class.as_str()
        );
        assert!(
            row.exact_build_only,
            "{} must require exact-build identity",
            row.artifact_class.as_str()
        );
        if row.artifact_class != CrashArtifactClass::MirroredSymbolService {
            assert!(
                row.local_first_by_default,
                "{} should stay local-first by default",
                row.artifact_class.as_str()
            );
        }
    }
}

#[test]
fn diagnostic_schema_rows_cover_crash_performance_usage_and_support() {
    let packet = seeded_m5_fault_crash_governance_packet();
    let signal_classes = packet
        .diagnostic_schemas
        .iter()
        .map(|row| row.signal_class)
        .collect::<Vec<_>>();
    for required in DiagnosticSignalClass::ALL {
        assert!(
            signal_classes.contains(&required),
            "missing diagnostic signal {}",
            required.as_str()
        );
    }

    let crash = packet
        .diagnostic_schemas
        .iter()
        .find(|row| row.signal_class == DiagnosticSignalClass::Crash)
        .expect("crash schema row");
    assert_eq!(crash.schema_id, "diagnostics.crash_payload");

    let usage = packet
        .diagnostic_schemas
        .iter()
        .find(|row| row.signal_class == DiagnosticSignalClass::Usage)
        .expect("usage schema row");
    assert_eq!(usage.schema_id, "usage.metering_export_packet");

    let support = packet
        .diagnostic_schemas
        .iter()
        .find(|row| row.signal_class == DiagnosticSignalClass::Support)
        .expect("support schema row");
    assert_eq!(support.schema_id, "support.bundle_manifest");
}

#[test]
fn downgrade_rules_cover_restart_crash_and_schema_staleness() {
    let packet = seeded_m5_fault_crash_governance_packet();
    let triggers = packet
        .downgrade_rules
        .iter()
        .map(|row| row.trigger_class)
        .collect::<Vec<_>>();
    for required in [
        DowngradeTriggerClass::RestartEvidenceStale,
        DowngradeTriggerClass::CrashArtifactProofStale,
        DowngradeTriggerClass::SymbolicationNotExactBuild,
        DowngradeTriggerClass::DiagnosticSchemaStale,
    ] {
        assert!(
            triggers.contains(&required),
            "missing downgrade trigger {}",
            required.as_str()
        );
    }
}

#[test]
fn validation_accepts_seeded_packet_and_rejects_qualified_rows_with_stale_proof() {
    let packet = seeded_m5_fault_crash_governance_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let mut invalid = packet.clone();
    let provider = invalid
        .host_families
        .iter_mut()
        .find(|row| row.host_family_id == "provider_run_session_host")
        .expect("provider host");
    provider
        .stale_proof_tokens
        .push("stale_symbolication".to_owned());
    let violations = invalid.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.path.contains("provider_run_session_host")),
        "qualified row with stale proof must be rejected: {violations:?}",
    );
}

#[test]
fn packet_round_trips_through_json_and_is_export_safe() {
    let packet = seeded_m5_fault_crash_governance_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let round: M5FaultCrashGovernancePacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(round, packet);
    assert!(packet.is_export_safe());
    assert!(!json.contains("SSH_PRIVATE_KEY"));
    assert!(!json.contains("/Users/"));
}

#[test]
fn checked_in_docs_schema_artifact_and_fixture_manifest_exist() {
    let root = repo_root();
    for rel in [
        M5_FAULT_CRASH_GOVERNANCE_SCHEMA_REF,
        M5_FAULT_CRASH_GOVERNANCE_DOC_REF,
        M5_FAULT_CRASH_GOVERNANCE_ARTIFACT_REF,
        "fixtures/support/m5/fault_crash_governance/manifest.yaml",
        "fixtures/support/m5/fault_crash_governance/README.md",
        "fixtures/support/m5/fault_crash_governance/packet.json",
        "fixtures/support/m5/fault_crash_governance/stale_symbolication_narrowed.json",
        "fixtures/support/m5/fault_crash_governance/stale_diagnostic_schema_blocked.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    let fixture = load_fixture("packet.json");
    let packet = seeded_m5_fault_crash_governance_packet();
    assert_eq!(fixture, packet);
}

#[test]
fn narrowed_fixtures_keep_claims_below_qualified() {
    let symbolication_fixture = load_fixture("stale_symbolication_narrowed.json");
    assert!(symbolication_fixture.validate().is_empty());
    for host_id in ["provider_run_session_host", "profiler_replay_session_host"] {
        let row = symbolication_fixture
            .host_families
            .iter()
            .find(|row| row.host_family_id == host_id)
            .expect("row present");
        assert_eq!(row.claim_state, ClaimStateClass::NarrowedLocalOnly);
        assert!(!row.stale_proof_tokens.is_empty());
    }

    let schema_fixture = load_fixture("stale_diagnostic_schema_blocked.json");
    assert!(schema_fixture.validate().is_empty());
    for host_id in [
        "provider_run_session_host",
        "registry_database_connector_host",
    ] {
        let row = schema_fixture
            .host_families
            .iter()
            .find(|row| row.host_family_id == host_id)
            .expect("row present");
        assert_eq!(row.claim_state, ClaimStateClass::BlockedUnverified);
        assert!(!row.stale_proof_tokens.is_empty());
    }
}
