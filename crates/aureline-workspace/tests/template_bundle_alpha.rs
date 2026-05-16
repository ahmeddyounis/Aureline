//! Fixture-driven coverage for the alpha workspace-template bundle.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_workspace_template_bundle, WorkspaceTemplateBundleError, WorkspaceTemplateBundleRecord,
    TEMPLATE_BUNDLE_BYPASS_CONTINUITY_CLASS_EQUAL_WEIGHT,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m3/template_bundle")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("template-bundle fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

#[test]
fn every_fixture_projects_through_the_alpha_contract() {
    let paths = load_fixture_paths();
    assert!(
        !paths.is_empty(),
        "template-bundle fixtures dir must contain at least one fixture"
    );
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_workspace_template_bundle(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));

        assert_eq!(
            projection.bypass_continuity_class,
            TEMPLATE_BUNDLE_BYPASS_CONTINUITY_CLASS_EQUAL_WEIGHT,
            "fixture {path:?} must keep bypass continuity at equal weight",
        );
        assert!(
            !projection.open_without_starter_route_ids.is_empty(),
            "fixture {path:?} must offer at least one bypass route",
        );
        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "start_center"),
            "fixture {path:?} must wire Start Center as a consumer",
        );
        assert!(
            !projection.raw_secret_export_allowed,
            "fixture {path:?} must not allow raw secret export",
        );
        assert!(
            !projection.raw_command_export_allowed,
            "fixture {path:?} must not allow raw command export",
        );
        assert!(
            !projection.raw_url_export_allowed,
            "fixture {path:?} must not allow raw URL export",
        );
    }
}

#[test]
fn first_party_fixture_exposes_review_disclosure() {
    let payload = std::fs::read_to_string(fixtures_dir().join("first_party_local_starter.json"))
        .expect("first-party fixture must read");
    let projection =
        project_workspace_template_bundle(&payload).expect("first-party fixture must project");
    assert_eq!(projection.source_class, "first_party");
    assert_eq!(projection.support_class, "experimental");
    assert_eq!(projection.runtime_scope_class, "local_only");
    assert_eq!(projection.host_boundary_class, "host_local_device_only");
    assert_eq!(
        projection.required_remote_provisioning_class,
        "no_remote_provisioning_required"
    );
    assert!(projection
        .open_without_starter_route_ids
        .iter()
        .any(|r| r == "bypass.create_empty_workspace"));
}

#[test]
fn community_fixture_carries_trust_notes() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("community_uncertified_starter.json"))
            .expect("community fixture must read");
    let projection =
        project_workspace_template_bundle(&payload).expect("community fixture must project");
    assert_eq!(projection.source_class, "community");
    assert!(
        !projection.trust_notes.is_empty(),
        "community bundle must carry at least one trust note",
    );
    assert!(projection
        .required_network_egress_class
        .contains("community"));
}

#[test]
fn managed_cloud_fixture_requires_managed_workspace_provisioning() {
    let payload = std::fs::read_to_string(fixtures_dir().join("managed_cloud_starter.json"))
        .expect("managed-cloud fixture must read");
    let projection =
        project_workspace_template_bundle(&payload).expect("managed-cloud fixture must project");
    assert_eq!(projection.runtime_scope_class, "managed_cloud_required");
    assert_eq!(
        projection.required_remote_provisioning_class,
        "managed_workspace_required"
    );
    assert_ne!(
        projection.required_managed_service_class,
        "no_managed_service_required"
    );
    assert_ne!(
        projection.required_network_egress_class,
        "no_network_egress_required"
    );
}

#[test]
fn rejects_managed_cloud_without_managed_service() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("managed_cloud_starter.json")).unwrap();
    let mut record: WorkspaceTemplateBundleRecord =
        serde_json::from_str(&payload).expect("managed-cloud fixture must parse");
    record.side_effect_review.required_managed_service_class =
        "no_managed_service_required".to_string();
    let err = record
        .validate()
        .expect_err("managed cloud cannot drop managed-service class");
    assert!(err.message().to_lowercase().contains("managed-service"));
}

#[test]
fn rejects_empty_bypass_route_list() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_local_starter.json")).unwrap();
    let mut record: WorkspaceTemplateBundleRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.bypass_review.open_without_starter_route_ids.clear();
    let err = record.validate().expect_err("must reject empty bypass");
    assert!(err.message().contains("open_without_starter_route_ids"));
}

#[test]
fn rejects_consumer_surfaces_without_start_center() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_local_starter.json")).unwrap();
    let mut record: WorkspaceTemplateBundleRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record
        .consumer_surfaces
        .retain(|surface| surface != "start_center");
    let err = record
        .validate()
        .expect_err("must reject consumer list without start_center");
    assert!(err.message().contains("start_center"));
}

#[test]
fn rejects_raw_secret_export() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_local_starter.json")).unwrap();
    let mut record: WorkspaceTemplateBundleRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.support_export.raw_secret_export_allowed = true;
    let err = record
        .validate()
        .expect_err("must reject raw secret export");
    assert!(err.message().contains("raw_"));
}

#[test]
fn rejects_invalid_record_kind_via_project() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_local_starter.json")).unwrap();
    let tampered = payload.replace(
        "\"record_kind\": \"workspace_template_bundle_alpha_record\"",
        "\"record_kind\": \"some_other_record_kind\"",
    );
    match project_workspace_template_bundle(&tampered) {
        Err(WorkspaceTemplateBundleError::Validation(err)) => {
            assert!(err.message().contains("record_kind"));
        }
        other => panic!("expected validation failure, got {other:?}"),
    }
}
