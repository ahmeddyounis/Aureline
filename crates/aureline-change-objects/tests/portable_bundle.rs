//! Fixture-driven coverage for portable bundles and shelf entries.

use std::path::{Path, PathBuf};

use aureline_change_objects::{
    current_portable_bundle_fixture_projections, project_portable_bundle, PortableBundleRecord,
    PORTABLE_BUNDLE_RECORD_KIND, PORTABLE_BUNDLE_SCHEMA_VERSION,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/m3/portable_bundle")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("portable bundle fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

#[test]
fn every_fixture_projects_through_the_portable_bundle_contract() {
    let paths = load_fixture_paths();
    assert!(
        !paths.is_empty(),
        "portable bundle fixture dir must contain fixtures"
    );
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_portable_bundle(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));

        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "portable_bundle_inspector"),
            "fixture {path:?} must wire portable_bundle_inspector",
        );
        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "support_export"),
            "fixture {path:?} must wire support_export",
        );
        assert!(
            projection.no_live_provider_authority,
            "fixture {path:?} must exclude live provider authority",
        );
        assert!(
            !projection.raw_path_export_allowed,
            "fixture {path:?} must keep raw paths closed",
        );
        assert!(
            !projection.raw_remote_url_export_allowed,
            "fixture {path:?} must keep raw remote URLs closed",
        );
        assert!(
            !projection.raw_secret_export_allowed,
            "fixture {path:?} must keep raw secrets closed",
        );
        assert!(
            !projection.raw_credential_export_allowed,
            "fixture {path:?} must keep raw credentials closed",
        );
    }
}

#[test]
fn fixtures_cover_handoff_purposes_and_reopen_modes() {
    let projections = current_portable_bundle_fixture_projections().expect("fixtures must project");
    let purposes: std::collections::BTreeSet<_> = projections
        .iter()
        .map(|projection| projection.handoff_purpose_class.as_str())
        .collect();
    for purpose in [
        "offline_review_handoff",
        "browser_companion_handoff",
        "incident_follow_up",
        "support_export",
    ] {
        assert!(purposes.contains(purpose), "missing purpose {purpose}");
    }

    assert!(
        projections
            .iter()
            .any(|projection| projection.compare_only_reopen_available),
        "fixtures must cover compare-only reopen",
    );
    assert!(
        projections
            .iter()
            .any(|projection| projection.desktop_resume_available),
        "fixtures must cover desktop resume",
    );
    assert!(
        projections
            .iter()
            .any(|projection| projection.browser_companion_read_only_available),
        "fixtures must cover browser companion read-only reopen",
    );
    assert!(
        projections
            .iter()
            .any(|projection| !projection.staleness_labels.is_empty()),
        "fixtures must cover stale-validation labels",
    );
}

#[test]
fn shelf_fixture_requires_revalidation_before_desktop_resume() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("support_export_shelf_desktop_resume.json"))
            .expect("shelf fixture must read");
    let projection = project_portable_bundle(&payload).expect("shelf fixture must project");
    assert_eq!(projection.object_class, "shelf_entry");
    assert_eq!(
        projection.bundle_state_class,
        "desktop_resume_pending_revalidation"
    );
    assert!(projection.desktop_resume_available);
    assert_eq!(
        projection.validation_freshness_class,
        "stale_review_pack_version_changed"
    );
    assert!(projection
        .staleness_labels
        .iter()
        .any(|label| label == "review_pack_version_changed"));
}

#[test]
fn rejects_live_bearer_authority() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("offline_review_handoff.json")).unwrap();
    let mut record: PortableBundleRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.authority_state.live_bearer_authority_included = true;
    let err = record
        .validate()
        .expect_err("live bearer authority must fail validation");
    assert!(err.message().contains("live bearer authority"));
}

#[test]
fn rejects_stale_validation_without_label() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("incident_follow_up_stale_validation.json"))
            .unwrap();
    let mut record: PortableBundleRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.validation_state.staleness_labels.clear();
    let err = record
        .validate()
        .expect_err("stale validation without labels must fail");
    assert!(err.message().contains("staleness label"));
}

#[test]
fn rejects_raw_diff_body() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("offline_review_handoff.json")).unwrap();
    let mut record: PortableBundleRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.diff_refs[0].raw_diff_body_included = true;
    let err = record
        .validate()
        .expect_err("raw diff bodies must not cross the record boundary");
    assert!(err.message().contains("raw_diff_body_included"));
}

#[test]
fn rejects_raw_secret_export() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("offline_review_handoff.json")).unwrap();
    let mut record: PortableBundleRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.redaction_profile.raw_secret_export_allowed = true;
    let err = record.validate().expect_err("raw secret export must fail");
    assert!(err.message().contains("redaction_profile"));
}

#[test]
fn schema_version_constants_match_record_kind() {
    assert_eq!(PORTABLE_BUNDLE_SCHEMA_VERSION, 1);
    assert_eq!(PORTABLE_BUNDLE_RECORD_KIND, "portable_change_bundle_record");
}
