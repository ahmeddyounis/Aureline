use std::fs;

use super::{
    audit_structured_config_policy_bundle_and_entitlement_matrix,
    parse_structured_config_policy_bundle_and_entitlement_matrix,
    seeded_structured_config_policy_bundle_and_entitlement_matrix, ArtifactFamilyKind, BundleClass,
    DeploymentProfileKind, DistributionPath, DowngradeLabelClass, EnvelopeFieldClass,
    ManagedAuthDependencyClass, STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_PATH,
};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/config/structured_config_policy_bundle_and_entitlement_matrix/canonical.json",
);

#[test]
fn seeded_matrix_passes_validation() {
    let packet = seeded_structured_config_policy_bundle_and_entitlement_matrix();
    let defects = audit_structured_config_policy_bundle_and_entitlement_matrix(&packet);
    assert!(defects.is_empty(), "validation defects: {defects:?}");
}

#[test]
fn checked_in_artifact_matches_seeded_packet() {
    let path = format!(
        "{}/../../{}",
        env!("CARGO_MANIFEST_DIR"),
        STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_PATH
    );
    let body = fs::read_to_string(path).expect("artifact exists");
    let artifact = parse_structured_config_policy_bundle_and_entitlement_matrix(&body)
        .expect("artifact parses");
    assert_eq!(
        artifact,
        seeded_structured_config_policy_bundle_and_entitlement_matrix()
    );
}

#[test]
fn checked_in_fixture_matches_seeded_packet() {
    let body = fs::read_to_string(FIXTURE_PATH).expect("fixture exists");
    let fixture = parse_structured_config_policy_bundle_and_entitlement_matrix(&body)
        .expect("fixture parses");
    assert_eq!(
        fixture,
        seeded_structured_config_policy_bundle_and_entitlement_matrix()
    );
}

#[test]
fn required_artifact_families_bundle_classes_and_profiles_are_present() {
    let packet = seeded_structured_config_policy_bundle_and_entitlement_matrix();

    let families: Vec<_> = packet
        .artifact_families
        .iter()
        .map(|row| row.family)
        .collect();
    assert_eq!(families.len(), ArtifactFamilyKind::ALL.len());
    for required in ArtifactFamilyKind::ALL {
        assert!(families.contains(&required), "missing family {required:?}");
    }

    let bundles: Vec<_> = packet
        .bundle_taxonomy
        .iter()
        .map(|row| row.bundle_class)
        .collect();
    assert_eq!(bundles.len(), BundleClass::ALL.len());
    for required in BundleClass::ALL {
        assert!(bundles.contains(&required), "missing bundle {required:?}");
    }

    let profiles: Vec<_> = packet
        .profile_qualifications
        .iter()
        .map(|row| row.profile)
        .collect();
    assert_eq!(profiles.len(), DeploymentProfileKind::ALL.len());
    for required in DeploymentProfileKind::ALL {
        assert!(profiles.contains(&required), "missing profile {required:?}");
    }
}

#[test]
fn preview_rows_are_explicit_and_air_gapped_stays_offline() {
    let packet = seeded_structured_config_policy_bundle_and_entitlement_matrix();

    let preview_row = packet
        .artifact_families
        .iter()
        .find(|row| row.family == ArtifactFamilyKind::PreviewRuntimeConfig)
        .expect("preview runtime row present");
    assert!(
        preview_row
            .downgrade_labels
            .contains(&DowngradeLabelClass::PreviewDependencyDisclosed),
        "preview rows must carry explicit disclosure"
    );

    let air_gapped = packet
        .profile_qualifications
        .iter()
        .find(|row| row.profile == DeploymentProfileKind::FullyAirGapped)
        .expect("air-gapped profile present");
    assert_eq!(
        air_gapped.managed_auth_dependency,
        ManagedAuthDependencyClass::OfflineSnapshotAndLocalAdminCache
    );
    assert!(
        !air_gapped
            .distribution_paths
            .contains(&DistributionPath::SignedOrigin),
        "air-gapped profile must not allow live origin fetch"
    );
}

#[test]
fn bundle_taxonomy_keeps_core_envelope_fields_and_relations() {
    let packet = seeded_structured_config_policy_bundle_and_entitlement_matrix();

    for row in &packet.bundle_taxonomy {
        for required in [
            EnvelopeFieldClass::BundleId,
            EnvelopeFieldClass::BundleClass,
            EnvelopeFieldClass::SchemaVersion,
            EnvelopeFieldClass::IssuedAt,
            EnvelopeFieldClass::SignerRef,
            EnvelopeFieldClass::ScopeRef,
            EnvelopeFieldClass::DigestRef,
            EnvelopeFieldClass::PayloadRef,
            EnvelopeFieldClass::DistributionPath,
        ] {
            assert!(
                row.required_envelope_fields.contains(&required),
                "{:?} missing {:?}",
                row.bundle_class,
                required
            );
        }
    }

    let emergency = packet
        .bundle_taxonomy
        .iter()
        .find(|row| row.bundle_class == BundleClass::EmergencyDisableBundle)
        .expect("emergency bundle row present");
    assert!(emergency.supports_supersedes);
    assert!(emergency.supports_revokes);
}
