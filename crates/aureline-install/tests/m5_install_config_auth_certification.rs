//! Fixture-driven tests for the M5 install/config/auth certification packet.

use std::path::{Path, PathBuf};

use aureline_install::{
    current_m5_install_config_auth_certification, CertificationConsumer, CertificationDomain,
    CertificationDrill, CertificationDrillClass, CertificationProfile,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/install/m5/m5-install-config-auth-certification")
}

fn packet() -> aureline_install::M5InstallConfigAuthCertification {
    current_m5_install_config_auth_certification().expect("embedded certification packet parses")
}

#[test]
fn embedded_packet_validates_clean() {
    let packet = packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "certification packet failed validation: {violations:#?}"
    );
}

#[test]
fn every_profile_is_covered_once() {
    let packet = packet();
    for profile in CertificationProfile::ALL {
        assert!(
            packet.row(profile).is_some(),
            "missing certification row for {}",
            profile.as_str()
        );
    }
    assert_eq!(packet.rows.len(), CertificationProfile::ALL.len());
}

#[test]
fn every_row_covers_all_required_domains() {
    let packet = packet();
    for row in &packet.rows {
        for domain in CertificationDomain::REQUIRED {
            assert!(
                row.domains.iter().any(|qual| qual.domain == domain),
                "{} missing {}",
                row.row_id,
                domain.as_str()
            );
        }
    }
}

#[test]
fn drill_fixtures_match_the_embedded_packet() {
    let packet = packet();
    let dir = fixture_dir();
    let cases = [
        (
            "drill-install-topology.json",
            CertificationDrillClass::InstallTopology,
        ),
        (
            "drill-side-by-side.json",
            CertificationDrillClass::SideBySide,
        ),
        ("drill-portable.json", CertificationDrillClass::Portable),
        (
            "drill-mirror-offline.json",
            CertificationDrillClass::MirrorOffline,
        ),
        (
            "drill-settings-portability.json",
            CertificationDrillClass::SettingsPortability,
        ),
        (
            "drill-sync-device.json",
            CertificationDrillClass::SyncDevice,
        ),
        (
            "drill-passkey-recovery.json",
            CertificationDrillClass::PasskeyRecovery,
        ),
        (
            "drill-accessibility.json",
            CertificationDrillClass::Accessibility,
        ),
        ("drill-downgrade.json", CertificationDrillClass::Downgrade),
    ];
    for (file, drill_class) in cases {
        let bytes = std::fs::read(dir.join(file)).unwrap_or_else(|_| panic!("read {file}"));
        let drill: CertificationDrill =
            serde_json::from_slice(&bytes).unwrap_or_else(|_| panic!("parse {file}"));
        assert_eq!(drill.drill_class, drill_class, "{file} has the wrong class");
        assert!(drill.detected, "{file} must detect its scenario");
        assert!(
            packet.row_by_id(&drill.target_ref).is_some(),
            "{file} targets an unknown certification row"
        );
        let embedded = packet
            .drills
            .iter()
            .find(|candidate| candidate.drill_id == drill.drill_id)
            .unwrap_or_else(|| panic!("embedded packet missing drill {}", drill.drill_id));
        assert_eq!(embedded, &drill, "{file} drifted from the embedded packet");
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for consumer in CertificationConsumer::REQUIRED {
        assert!(
            packet.has_binding_for(consumer),
            "missing binding for {}",
            consumer.as_str()
        );
    }
}

#[test]
fn support_export_round_trips_clean() {
    let packet = packet();
    let export = packet.support_export("support-export-fixture", "2026-06-12T00:00:00Z");
    assert!(export.is_export_safe());
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.certification_packet_id_ref, packet.packet_id);
    serde_json::to_string(&export).expect("serialize support export");
    assert!(export.certification.validate().is_empty());
}
