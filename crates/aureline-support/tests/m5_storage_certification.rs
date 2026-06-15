//! Protected tests for the M5 storage certification packet.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::m5_storage_governance::{
    current_m5_artifact_family_storage_matrix, M5ArtifactFamilyStorageMatrix,
};
use aureline_support::{
    seeded_blurred_cache_authority_m5_storage_certification_packet,
    seeded_m5_storage_certification_packet,
    seeded_stale_pin_retention_m5_storage_certification_packet, ClaimedM5Profile,
    M5StorageCertificationPacket, PressureSourcePostureClass, StorageCertificationStateClass,
    StorageCertificationSurfaceClass, M5_STORAGE_CERTIFICATION_ARTIFACT_REF,
    M5_STORAGE_CERTIFICATION_DOC_REF, M5_STORAGE_CERTIFICATION_FIXTURE_DIR,
    M5_STORAGE_CERTIFICATION_PACKET_ID, M5_STORAGE_CERTIFICATION_PACKET_RECORD_KIND,
    M5_STORAGE_CERTIFICATION_SCHEMA_REF, M5_STORAGE_CERTIFICATION_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(M5_STORAGE_CERTIFICATION_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5StorageCertificationPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_has_expected_envelope_and_profile_coverage() {
    let packet = seeded_m5_storage_certification_packet();
    assert_eq!(
        packet.record_kind,
        M5_STORAGE_CERTIFICATION_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_STORAGE_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(packet.doc_ref, M5_STORAGE_CERTIFICATION_DOC_REF);
    assert_eq!(packet.schema_ref, M5_STORAGE_CERTIFICATION_SCHEMA_REF);
    assert_eq!(packet.artifact_ref, M5_STORAGE_CERTIFICATION_ARTIFACT_REF);
    assert_eq!(packet.packet_id, M5_STORAGE_CERTIFICATION_PACKET_ID);

    for profile in ClaimedM5Profile::ALL {
        assert!(
            packet.claimed_profiles.contains(&profile),
            "missing claimed profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_family_on_every_profile() {
    let packet = seeded_m5_storage_certification_packet();
    for family in M5ArtifactFamilyStorageMatrix::required_families() {
        for profile in ClaimedM5Profile::ALL {
            assert!(
                packet
                    .certification_rows
                    .iter()
                    .any(|row| row.family_id == *family && row.profile == profile),
                "missing row for {} on {}",
                family.as_str(),
                profile.as_str()
            );
        }
    }
    let expected =
        M5ArtifactFamilyStorageMatrix::required_families().len() * ClaimedM5Profile::ALL.len();
    assert_eq!(packet.certification_rows.len(), expected);
}

#[test]
fn canonical_packet_validates_and_is_export_safe() {
    let packet = seeded_m5_storage_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_export_safe());
    assert!(packet
        .certification_rows
        .iter()
        .all(|row| row.published_state == StorageCertificationStateClass::Qualified));
}

#[test]
fn rows_stay_consistent_with_the_storage_governance_matrix() {
    let packet = seeded_m5_storage_certification_packet();
    let matrix = current_m5_artifact_family_storage_matrix().expect("load matrix");
    for row in &packet.certification_rows {
        let matrix_row = matrix.family(row.family_id).expect("matrix row");
        assert_eq!(row.storage_class_id, matrix_row.storage_class_id);
        assert_eq!(row.authority_class, matrix_row.authority_class);
        assert_eq!(row.protected_continuity, matrix_row.protected_continuity);
    }
}

#[test]
fn managed_cloud_protects_user_state_from_quota_deletion() {
    let packet = seeded_m5_storage_certification_packet();
    for row in &packet.certification_rows {
        match (row.profile, row.protected_continuity) {
            (ClaimedM5Profile::ManagedCloud, true) => assert_eq!(
                row.pressure_source_posture,
                PressureSourcePostureClass::ManagedQuotaProtectedExcluded,
                "{} must be excluded from managed-quota deletion",
                row.certification_row_id
            ),
            (ClaimedM5Profile::ManagedCloud, false) => assert_eq!(
                row.pressure_source_posture,
                PressureSourcePostureClass::DiskAndManagedQuota,
                "{} faces disk + managed quota",
                row.certification_row_id
            ),
            (_, _) => assert_eq!(
                row.pressure_source_posture,
                PressureSourcePostureClass::LocalDiskOnly,
                "{} has no managed quota off managed_cloud",
                row.certification_row_id
            ),
        }
    }
}

#[test]
fn surface_bindings_all_ingest_the_same_index() {
    let packet = seeded_m5_storage_certification_packet();
    for surface in StorageCertificationSurfaceClass::ALL {
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
        assert!(binding.narrow_on_stale_proof);
        for field in [
            "certification_row_id",
            "family_id",
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
fn stale_pin_retention_fixture_gates_every_protected_family() {
    let packet = seeded_stale_pin_retention_m5_storage_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for row in &packet.certification_rows {
        if row.protected_continuity {
            assert_eq!(
                row.published_state,
                StorageCertificationStateClass::ProtectedReviewGatedOnly,
                "{} must be review-gated when pin/retention is stale",
                row.certification_row_id
            );
            assert!(row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "stale_pin_retention_audit"));
        } else {
            assert_eq!(
                row.published_state,
                StorageCertificationStateClass::Qualified
            );
        }
    }
}

#[test]
fn blurred_cache_authority_fixture_blocks_authoritative_and_narrows_disposable() {
    let packet = seeded_blurred_cache_authority_m5_storage_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // Protected families block; cache versus authoritative state can't be told apart.
    let blocked = packet
        .certification_rows
        .iter()
        .filter(|row| row.published_state == StorageCertificationStateClass::BlockedUnverified)
        .count();
    assert!(blocked > 0, "expected blocked authoritative rows");
    for row in &packet.certification_rows {
        if row.protected_continuity {
            assert_eq!(
                row.published_state,
                StorageCertificationStateClass::BlockedUnverified
            );
            assert!(row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "blurred_cache_versus_authoritative_state"));
        }
    }
    // At least one disposable family narrows because pressure behavior is hidden.
    assert!(packet.certification_rows.iter().any(|row| {
        row.published_state == StorageCertificationStateClass::LimitedClassScoped
            && row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "hidden_low_disk_pressure_behavior")
    }));
}

#[test]
fn degraded_fixtures_are_not_green() {
    for packet in [
        seeded_stale_pin_retention_m5_storage_certification_packet(),
        seeded_blurred_cache_authority_m5_storage_certification_packet(),
    ] {
        assert!(packet
            .certification_rows
            .iter()
            .any(|row| row.published_state != StorageCertificationStateClass::Qualified));
        // Every non-qualified row cites a downgrade rule that exists.
        let rule_ids: Vec<&str> = packet
            .downgrade_rules
            .iter()
            .map(|rule| rule.rule_id.as_str())
            .collect();
        for row in &packet.certification_rows {
            if row.published_state != StorageCertificationStateClass::Qualified {
                assert!(!row.downgrade_rule_ids.is_empty());
                for id in &row.downgrade_rule_ids {
                    assert!(rule_ids.contains(&id.as_str()), "unknown rule {id}");
                }
            }
        }
    }
}

#[test]
fn checked_in_docs_schema_artifact_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        M5_STORAGE_CERTIFICATION_SCHEMA_REF,
        M5_STORAGE_CERTIFICATION_DOC_REF,
        M5_STORAGE_CERTIFICATION_ARTIFACT_REF,
        "fixtures/storage/m5_storage_certification/manifest.yaml",
        "fixtures/storage/m5_storage_certification/README.md",
        "fixtures/storage/m5_storage_certification/packet.json",
        "fixtures/storage/m5_storage_certification/stale_pin_retention_gated.json",
        "fixtures/storage/m5_storage_certification/blurred_cache_authority_blocked.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    assert_eq!(
        load_fixture("packet.json"),
        seeded_m5_storage_certification_packet()
    );
}

#[test]
fn degraded_fixtures_match_seeded_variants() {
    assert_eq!(
        load_fixture("stale_pin_retention_gated.json"),
        seeded_stale_pin_retention_m5_storage_certification_packet()
    );
    assert_eq!(
        load_fixture("blurred_cache_authority_blocked.json"),
        seeded_blurred_cache_authority_m5_storage_certification_packet()
    );
}
