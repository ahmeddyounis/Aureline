//! Protected tests for the M5 fault/crash certification packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::{
    seeded_m5_fault_crash_certification_packet,
    seeded_stale_schema_m5_fault_crash_certification_packet,
    seeded_stale_symbolication_m5_fault_crash_certification_packet, CertificationStateClass,
    CertificationSurfaceClass, ClaimedM5Profile, M5FaultCrashCertificationPacket,
    M5_FAULT_CRASH_CERTIFICATION_ARTIFACT_REF, M5_FAULT_CRASH_CERTIFICATION_DOC_REF,
    M5_FAULT_CRASH_CERTIFICATION_FIXTURE_DIR, M5_FAULT_CRASH_CERTIFICATION_PACKET_RECORD_KIND,
    M5_FAULT_CRASH_CERTIFICATION_SCHEMA_REF, M5_FAULT_CRASH_CERTIFICATION_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(M5_FAULT_CRASH_CERTIFICATION_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5FaultCrashCertificationPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_has_expected_envelope_and_profile_coverage() {
    let packet = seeded_m5_fault_crash_certification_packet();
    assert_eq!(
        packet.record_kind,
        M5_FAULT_CRASH_CERTIFICATION_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_FAULT_CRASH_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(packet.doc_ref, M5_FAULT_CRASH_CERTIFICATION_DOC_REF);
    assert_eq!(packet.schema_ref, M5_FAULT_CRASH_CERTIFICATION_SCHEMA_REF);

    for profile in ClaimedM5Profile::ALL {
        assert!(
            packet.claimed_profiles.contains(&profile),
            "missing claimed profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_required_host_on_every_profile() {
    let packet = seeded_m5_fault_crash_certification_packet();
    for host_id in [
        "notebook_kernel_host",
        "data_api_connector_host",
        "preview_dev_server_host",
        "provider_run_session_host",
        "profiler_replay_session_host",
        "pipeline_viewer_host",
        "query_runtime_host",
        "docs_browser_bridge_host",
        "registry_database_connector_host",
        "infra_helper_job",
    ] {
        for profile in ClaimedM5Profile::ALL {
            assert!(
                packet
                    .certification_rows
                    .iter()
                    .any(|row| row.host_family_id == host_id && row.profile == profile),
                "missing row for {host_id} on {}",
                profile.as_str()
            );
        }
    }
}

#[test]
fn canonical_packet_validates_and_is_export_safe() {
    let packet = seeded_m5_fault_crash_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_export_safe());
}

#[test]
fn air_gapped_rows_narrow_instead_of_inheriting_broad_claims() {
    let packet = seeded_m5_fault_crash_certification_packet();
    for host_id in ["provider_run_session_host", "docs_browser_bridge_host"] {
        let row = packet
            .certification_rows
            .iter()
            .find(|row| {
                row.host_family_id == host_id
                    && row.profile == ClaimedM5Profile::AirGappedMirrorOnly
            })
            .expect("air-gapped row");
        assert_eq!(row.published_state, CertificationStateClass::NotMarketed);
        assert!(!row.stale_proof_tokens.is_empty());
    }
}

#[test]
fn surface_bindings_all_ingest_the_same_index() {
    let packet = seeded_m5_fault_crash_certification_packet();
    for surface in CertificationSurfaceClass::ALL {
        let binding = packet
            .surface_bindings
            .iter()
            .find(|binding| binding.surface == surface)
            .expect("surface binding");
        assert_eq!(binding.ingested_packet_id, packet.packet_id);
        assert_eq!(
            binding.certification_row_count,
            packet.certification_rows.len()
        );
        for field in [
            "certification_row_id",
            "host_family_id",
            "profile",
            "published_state",
            "stale_proof_tokens",
            "downgrade_rule_ids",
        ] {
            assert!(
                binding
                    .required_verbatim_fields
                    .iter()
                    .any(|item| item == field),
                "{} binding must preserve {field}",
                surface.as_str()
            );
        }
    }
}

#[test]
fn stale_symbolication_fixture_narrows_shared_forensics_claims() {
    let packet = seeded_stale_symbolication_m5_fault_crash_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for host_id in ["provider_run_session_host", "profiler_replay_session_host"] {
        for profile in [
            ClaimedM5Profile::DesktopLocalFirst,
            ClaimedM5Profile::HybridRemoteAttach,
            ClaimedM5Profile::ManagedCloud,
            ClaimedM5Profile::SelfHostedSovereign,
        ] {
            let row = packet
                .certification_rows
                .iter()
                .find(|row| row.host_family_id == host_id && row.profile == profile)
                .expect("stale symbolication row");
            assert_eq!(
                row.published_state,
                CertificationStateClass::ExperimentalLocalOnly
            );
            assert!(row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "stale_symbolication_proof"));
        }
    }
}

#[test]
fn stale_schema_fixture_blocks_managed_export_claims() {
    let packet = seeded_stale_schema_m5_fault_crash_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for host_id in [
        "provider_run_session_host",
        "pipeline_viewer_host",
        "registry_database_connector_host",
    ] {
        for profile in [
            ClaimedM5Profile::ManagedCloud,
            ClaimedM5Profile::SelfHostedSovereign,
        ] {
            let row = packet
                .certification_rows
                .iter()
                .find(|row| row.host_family_id == host_id && row.profile == profile)
                .expect("stale schema row");
            assert_eq!(
                row.published_state,
                CertificationStateClass::BlockedUnverified
            );
            assert!(row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "stale_diagnostic_schema_review"));
        }
    }
}

#[test]
fn checked_in_docs_schema_artifact_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        M5_FAULT_CRASH_CERTIFICATION_SCHEMA_REF,
        M5_FAULT_CRASH_CERTIFICATION_DOC_REF,
        M5_FAULT_CRASH_CERTIFICATION_ARTIFACT_REF,
        "fixtures/support/m5/fault_crash_certification/manifest.yaml",
        "fixtures/support/m5/fault_crash_certification/README.md",
        "fixtures/support/m5/fault_crash_certification/packet.json",
        "fixtures/support/m5/fault_crash_certification/stale_symbolication_narrowed.json",
        "fixtures/support/m5/fault_crash_certification/stale_schema_blocked.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    let fixture = load_fixture("packet.json");
    let packet = seeded_m5_fault_crash_certification_packet();
    assert_eq!(fixture, packet);
}

#[test]
fn degraded_fixtures_match_seeded_variants() {
    assert_eq!(
        load_fixture("stale_symbolication_narrowed.json"),
        seeded_stale_symbolication_m5_fault_crash_certification_packet()
    );
    assert_eq!(
        load_fixture("stale_schema_blocked.json"),
        seeded_stale_schema_m5_fault_crash_certification_packet()
    );
}
