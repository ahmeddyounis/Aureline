//! Fixture and invariant tests for the stable service-health destination truth descriptor.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_service_health::{
    canonical_service_health_destination_truth_descriptor, ContinuityDrillScenario,
    DestinationTrustClass, PublicProofSurface, ServiceContractState,
    ServiceHealthDestinationTruthDescriptor, SERVICE_HEALTH_DESTINATION_RECORD_KIND,
    SERVICE_HEALTH_DESTINATION_SCHEMA_VERSION,
};

const FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/help/m4/finalize-service-health-destination-truth/canonical_descriptor.json"
));

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn canonical_descriptor_validates_cleanly() {
    let descriptor = canonical_service_health_destination_truth_descriptor();
    assert_eq!(
        descriptor.record_kind,
        SERVICE_HEALTH_DESTINATION_RECORD_KIND
    );
    assert_eq!(
        descriptor.schema_version,
        SERVICE_HEALTH_DESTINATION_SCHEMA_VERSION
    );
    let report = descriptor.validate();
    assert!(
        report.passed,
        "canonical descriptor must validate cleanly: {:#?}",
        report.findings
    );
}

#[test]
fn checked_in_fixture_matches_canonical_descriptor() {
    let fixture: ServiceHealthDestinationTruthDescriptor =
        serde_json::from_str(FIXTURE).expect("canonical fixture parses");
    assert_eq!(
        fixture,
        canonical_service_health_destination_truth_descriptor()
    );
}

#[test]
fn descriptor_covers_required_vocabularies_and_surfaces() {
    let descriptor = canonical_service_health_destination_truth_descriptor();
    let report = descriptor.validate();
    let state_tokens: BTreeSet<_> = report.coverage.service_contract_states;
    for state in ServiceContractState::ALL {
        assert!(
            state_tokens.contains(&state),
            "missing service contract state {}",
            state.as_str()
        );
    }

    let destination_classes: BTreeSet<_> = report.coverage.destination_classes;
    for class in DestinationTrustClass::ALL {
        assert!(
            destination_classes.contains(&class),
            "missing destination trust class {}",
            class.as_str()
        );
    }

    let surfaces: BTreeSet<_> = report.coverage.surfaces;
    for surface in PublicProofSurface::REQUIRED {
        assert!(
            surfaces.contains(&surface),
            "missing public-proof surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn continuity_drills_cover_offline_browser_blocked_and_partial_outage() {
    let descriptor = canonical_service_health_destination_truth_descriptor();
    let scenarios: BTreeSet<_> = descriptor
        .continuity_drills
        .iter()
        .map(|drill| drill.scenario)
        .collect();
    for scenario in [
        ContinuityDrillScenario::Offline,
        ContinuityDrillScenario::Mirrored,
        ContinuityDrillScenario::BrowserBlocked,
        ContinuityDrillScenario::DegradedService,
        ContinuityDrillScenario::PartialServiceOutage,
    ] {
        assert!(scenarios.contains(&scenario), "missing drill {scenario:?}");
    }
    for drill in &descriptor.continuity_drills {
        assert!(drill.stale_or_cached_label_visible);
        assert!(drill.destination_classes_preserved_before_exit);
        assert!(drill.local_only_continuity_visible);
        assert!(drill.support_save_later_verified);
        assert!(drill.no_implicit_upload);
    }
}

#[test]
fn support_export_is_local_first_save_later_only_until_explicit_submit() {
    let descriptor = canonical_service_health_destination_truth_descriptor();
    let export = descriptor.support_export_projection();
    assert_eq!(
        export.support_save_later.destination_class,
        DestinationTrustClass::LocalOnly
    );
    assert!(export.support_save_later.local_first);
    assert!(!export.support_save_later.implicit_upload_allowed);
    assert!(export.support_save_later.inspect_before_submit_required);
    assert!(!export
        .support_save_later
        .explicit_submit_action_refs
        .is_empty());
}

#[test]
fn schema_artifact_exists_at_declared_path() {
    let path = repo_root().join("schemas/help/service-health-destination.schema.json");
    assert!(path.exists(), "{} must exist", path.display());
}
