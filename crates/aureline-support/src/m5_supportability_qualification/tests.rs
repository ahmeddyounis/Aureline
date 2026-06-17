use std::fs;
use std::path::{Path, PathBuf};

use super::*;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(M5_SUPPORTABILITY_QUALIFICATION_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> M5SupportabilityQualificationPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_has_expected_envelope_and_coverage() {
    let packet = seeded_m5_supportability_qualification_packet();
    assert_eq!(
        packet.record_kind,
        M5_SUPPORTABILITY_QUALIFICATION_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(packet.packet_id, M5_SUPPORTABILITY_QUALIFICATION_PACKET_ID);
    assert_eq!(packet.doc_ref, M5_SUPPORTABILITY_QUALIFICATION_DOC_REF);
    assert_eq!(
        packet.schema_ref,
        M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_REF
    );
    assert_eq!(
        packet.artifact_ref,
        M5_SUPPORTABILITY_QUALIFICATION_ARTIFACT_REF
    );

    for profile in ClaimedM5Profile::ALL {
        assert!(
            packet.claimed_profiles.contains(&profile),
            "missing profile {}",
            profile.as_str()
        );
    }
    for mode in DeploymentMode::ALL {
        assert!(
            packet.deployment_modes.contains(&mode),
            "missing deployment mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_surface_on_every_profile() {
    let packet = seeded_m5_supportability_qualification_packet();
    for surface in SupportabilitySurfaceClass::ALL {
        for profile in ClaimedM5Profile::ALL {
            assert!(
                packet
                    .qualification_rows
                    .iter()
                    .any(|row| row.surface == surface && row.profile == profile),
                "missing row for {} on {}",
                surface.as_str(),
                profile.as_str()
            );
        }
    }
    let expected = SupportabilitySurfaceClass::ALL.len() * ClaimedM5Profile::ALL.len();
    assert_eq!(packet.qualification_rows.len(), expected);
}

#[test]
fn canonical_packet_validates_and_is_export_safe() {
    let packet = seeded_m5_supportability_qualification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_export_safe());
    assert!(packet
        .qualification_rows
        .iter()
        .all(|row| row.published_state == QualificationStateClass::Qualified));
}

#[test]
fn every_row_cites_only_its_own_surface_proof() {
    // Guardrail: a row may not borrow a neighboring crash/Doctor surface's proof.
    let packet = seeded_m5_supportability_qualification_packet();
    for row in &packet.qualification_rows {
        let refs = row.surface.backing_refs();
        assert_eq!(row.backing_schema_ref, refs.schema_ref);
        assert_eq!(row.backing_artifact_ref, refs.artifact_ref);
        assert_eq!(row.backing_fixture_ref, refs.fixture_ref);
        assert_eq!(row.backing_record_kind, refs.record_kind);
    }
    // The six surfaces must cite six distinct boundary schemas.
    let mut schemas: Vec<&str> = packet
        .qualification_rows
        .iter()
        .map(|row| row.backing_schema_ref.as_str())
        .collect();
    schemas.sort_unstable();
    schemas.dedup();
    assert_eq!(schemas.len(), SupportabilitySurfaceClass::ALL.len());
}

#[test]
fn send_capable_surfaces_keep_local_save_first() {
    let packet = seeded_m5_supportability_qualification_packet();
    for row in &packet.qualification_rows {
        if row.share_capable {
            assert!(
                row.local_save_first,
                "{} demotes local-save below a send path",
                row.qualification_row_id
            );
        }
        assert_eq!(row.share_capable, row.surface.share_capable());
    }
}

#[test]
fn every_drill_class_is_bound_and_covers_a_surface() {
    let packet = seeded_m5_supportability_qualification_packet();
    for class in SupportabilityDrillClass::ALL {
        let drill = packet
            .drill_catalog
            .iter()
            .find(|drill| drill.drill_class == class)
            .unwrap_or_else(|| panic!("missing drill {}", class.as_str()));
        assert!(!drill.covered_surfaces.is_empty());
        assert!(!drill.evidence_refs.is_empty());
        for surface in &drill.covered_surfaces {
            assert!(
                surface.drills().contains(&class),
                "{} claims surface {} that does not bind it",
                class.as_str(),
                surface.as_str()
            );
        }
    }
}

#[test]
fn consumer_bindings_all_ingest_the_same_index() {
    let packet = seeded_m5_supportability_qualification_packet();
    for consumer in QualificationConsumerClass::ALL {
        let binding = packet
            .consumer_bindings
            .iter()
            .find(|binding| binding.consumer == consumer)
            .unwrap_or_else(|| panic!("missing consumer binding {}", consumer.as_str()));
        assert_eq!(binding.ingested_packet_id, packet.packet_id);
        assert_eq!(
            binding.qualification_row_count,
            packet.qualification_rows.len()
        );
        assert!(binding.narrow_on_stale_proof);
        for field in [
            "qualification_row_id",
            "surface",
            "profile",
            "published_state",
            "deployment_mode_coverage",
            "stale_proof_tokens",
            "downgrade_rule_ids",
        ] {
            assert!(
                binding
                    .required_verbatim_fields
                    .iter()
                    .any(|item| item == field),
                "{} binding must preserve {field}",
                consumer.as_str()
            );
        }
    }
}

#[test]
fn consent_drill_stale_fixture_narrows_send_capable_surfaces() {
    let packet = seeded_consent_drill_stale_m5_supportability_qualification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The consent-sheet-accuracy drill is marked stale.
    assert!(packet.drill_catalog.iter().any(|drill| drill.drill_class
        == SupportabilityDrillClass::ConsentSheetAccuracy
        && !drill.is_fresh));
    for row in &packet.qualification_rows {
        let binds_consent = row
            .surface
            .drills()
            .contains(&SupportabilityDrillClass::ConsentSheetAccuracy);
        if binds_consent {
            assert_eq!(
                row.published_state,
                QualificationStateClass::LocalSelfDiagnosisOnly,
                "{} must narrow when consent-sheet accuracy is stale",
                row.qualification_row_id
            );
            assert!(row
                .stale_proof_tokens
                .iter()
                .any(|token| token == "consent_sheet_accuracy_drill_stale"));
            // Local-save stays first-class even while the send claim narrows.
            assert!(row.local_save_first);
        } else {
            assert_eq!(row.published_state, QualificationStateClass::Qualified);
        }
    }
}

#[test]
fn environment_evidence_stale_fixture_blocks_owner_and_narrows_support_center() {
    let packet = seeded_environment_evidence_stale_m5_supportability_qualification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for row in &packet.qualification_rows {
        match row.surface {
            SupportabilitySurfaceClass::EnvironmentExplainability => {
                assert_eq!(
                    row.published_state,
                    QualificationStateClass::BlockedUnverified,
                    "{} must block when its own evidence is stale",
                    row.qualification_row_id
                );
            }
            SupportabilitySurfaceClass::SupportCenter => {
                assert_eq!(
                    row.published_state,
                    QualificationStateClass::LimitedProfileScoped,
                    "{} narrows because it binds stale environment evidence",
                    row.qualification_row_id
                );
            }
            // Adjacent surfaces keep their own fresh proof and stay green.
            _ => assert_eq!(row.published_state, QualificationStateClass::Qualified),
        }
    }
}

#[test]
fn degraded_fixtures_are_not_green_and_cite_real_rules() {
    for packet in [
        seeded_consent_drill_stale_m5_supportability_qualification_packet(),
        seeded_environment_evidence_stale_m5_supportability_qualification_packet(),
    ] {
        assert!(packet
            .qualification_rows
            .iter()
            .any(|row| row.published_state != QualificationStateClass::Qualified));
        let rule_ids: Vec<&str> = packet
            .downgrade_rules
            .iter()
            .map(|rule| rule.rule_id.as_str())
            .collect();
        for row in &packet.qualification_rows {
            if row.published_state != QualificationStateClass::Qualified {
                assert!(!row.downgrade_rule_ids.is_empty());
                for id in &row.downgrade_rule_ids {
                    assert!(rule_ids.contains(&id.as_str()), "unknown rule {id}");
                }
            }
        }
    }
}

#[test]
fn state_counts_track_published_rows() {
    let canonical = seeded_m5_supportability_qualification_packet();
    let counts = canonical.state_counts();
    assert_eq!(counts.qualified, canonical.qualification_rows.len());
    assert_eq!(counts.blocked_unverified, 0);

    let blocked = seeded_environment_evidence_stale_m5_supportability_qualification_packet();
    let blocked_counts = blocked.state_counts();
    assert_eq!(
        blocked_counts.blocked_unverified,
        ClaimedM5Profile::ALL.len()
    );
    assert_eq!(
        blocked_counts.limited_profile_scoped,
        ClaimedM5Profile::ALL.len()
    );
}

#[test]
fn checked_in_docs_schema_artifact_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        M5_SUPPORTABILITY_QUALIFICATION_SCHEMA_REF,
        M5_SUPPORTABILITY_QUALIFICATION_DOC_REF,
        M5_SUPPORTABILITY_QUALIFICATION_ARTIFACT_REF,
        M5_SUPPORTABILITY_QUALIFICATION_CLAIM_PACKET_REF,
        "fixtures/support/m5/m5-supportability-qualification/manifest.yaml",
        "fixtures/support/m5/m5-supportability-qualification/README.md",
        "fixtures/support/m5/m5-supportability-qualification/packet.json",
        "fixtures/support/m5/m5-supportability-qualification/consent_drill_stale_narrowed.json",
        "fixtures/support/m5/m5-supportability-qualification/environment_evidence_stale_blocked.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn canonical_fixture_matches_seeded_packet() {
    assert_eq!(
        load_fixture("packet.json"),
        seeded_m5_supportability_qualification_packet()
    );
}

#[test]
fn degraded_fixtures_match_seeded_variants() {
    assert_eq!(
        load_fixture("consent_drill_stale_narrowed.json"),
        seeded_consent_drill_stale_m5_supportability_qualification_packet()
    );
    assert_eq!(
        load_fixture("environment_evidence_stale_blocked.json"),
        seeded_environment_evidence_stale_m5_supportability_qualification_packet()
    );
}
