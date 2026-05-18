use std::collections::HashSet;
use std::path::{Path, PathBuf};

use aureline_remote::{
    AudienceClass, AuthSourceClass, ControlledExposureLabel, ExposureReview, LifecycleState,
    RevocationSummary, RevokePostureClass, RouteObject, TeardownState, EXPOSURE_REVIEW_RECORD_KIND,
    ROUTE_OBJECT_RECORD_KIND,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join("fixtures/remote/m3/route_exposure_and_revocation")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_controlled_exposure_labels: Vec<String>,
    expected_lifecycle_states: Vec<String>,
    expected_revocation_postures: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    __fixture__: FixtureMeta,
    route: RouteObject,
    #[serde(default)]
    review: Option<ExposureReview>,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    controlled_exposure_label: ControlledExposureLabel,
    audience_class: AudienceClass,
    auth_source_class: AuthSourceClass,
    teardown_state: TeardownState,
    revoke_posture_class: RevokePostureClass,
    findings: Vec<String>,
}

#[test]
fn route_exposure_and_revocation_fixtures_validate_and_match_manifest() {
    let manifest_path = fixture_root().join("manifest.yaml");
    let manifest_payload =
        std::fs::read_to_string(&manifest_path).expect("manifest must read from disk");
    let manifest: Manifest = serde_yaml::from_str(&manifest_payload).expect("manifest must parse");

    assert_eq!(manifest.schema_version, 1);
    assert!(!manifest.case_refs.is_empty());

    let mut observed_labels: HashSet<String> = HashSet::new();
    let mut observed_lifecycle: HashSet<String> = HashSet::new();
    let mut observed_postures: HashSet<String> = HashSet::new();

    for case_rel in &manifest.case_refs {
        let case_path = repo_root().join(case_rel);
        let payload = std::fs::read_to_string(&case_path)
            .unwrap_or_else(|err| panic!("read {case_path:?}: {err}"));
        let case: FixtureCase = serde_yaml::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse {case_path:?}: {err}"));

        let scope = format!("{case_rel} ({})", case.__fixture__.name);
        assert_eq!(
            case.route.record_kind, ROUTE_OBJECT_RECORD_KIND,
            "{scope}: route carries canonical record kind"
        );

        let findings = case.route.validate();
        assert!(
            findings.is_empty(),
            "{scope}: route must validate clean, found {findings:?}"
        );
        assert_eq!(
            case.__fixture__.expected.findings,
            Vec::<String>::new(),
            "{scope}: fixture must declare zero expected findings"
        );

        assert_eq!(
            case.route.controlled_exposure_label,
            case.__fixture__.expected.controlled_exposure_label,
            "{scope}: controlled exposure label"
        );
        assert_eq!(
            case.route.audience.audience_class, case.__fixture__.expected.audience_class,
            "{scope}: audience class"
        );
        assert_eq!(
            case.route.auth.auth_source_class, case.__fixture__.expected.auth_source_class,
            "{scope}: auth source class"
        );
        assert_eq!(
            case.route.revocation.teardown_state, case.__fixture__.expected.teardown_state,
            "{scope}: teardown state"
        );
        assert_eq!(
            case.route.revocation.revoke_posture_class,
            case.__fixture__.expected.revoke_posture_class,
            "{scope}: revoke posture class"
        );

        let summary = RevocationSummary::from_route(&case.route);
        assert_eq!(summary.route_id, case.route.route_id);
        assert_eq!(
            summary.controlled_exposure_label,
            case.route.controlled_exposure_label
        );
        assert_eq!(summary.audience_class, case.route.audience.audience_class);
        assert_eq!(summary.teardown_state, case.route.revocation.teardown_state);

        observed_labels.insert(case.route.controlled_exposure_label.as_str().to_owned());
        observed_lifecycle.insert(serialize_lifecycle(case.route.lifecycle_state));
        observed_postures.insert(
            case.route
                .revocation
                .revoke_posture_class
                .as_str()
                .to_owned(),
        );

        if let Some(review) = case.review.as_ref() {
            assert_eq!(
                review.record_kind, EXPOSURE_REVIEW_RECORD_KIND,
                "{scope}: review carries canonical record kind"
            );
            assert_eq!(
                review.route_id, case.route.route_id,
                "{scope}: review binds the same route id as the route object"
            );
            let review_findings = review.validate();
            assert!(
                review_findings.is_empty(),
                "{scope}: review must validate clean, found {review_findings:?}"
            );
        }
    }

    for label in &manifest.expected_controlled_exposure_labels {
        assert!(
            observed_labels.contains(label),
            "manifest declares controlled exposure label {label} but no fixture covers it"
        );
    }
    for state in &manifest.expected_lifecycle_states {
        assert!(
            observed_lifecycle.contains(state),
            "manifest declares lifecycle state {state} but no fixture covers it"
        );
    }
    for posture in &manifest.expected_revocation_postures {
        assert!(
            observed_postures.contains(posture),
            "manifest declares revoke posture {posture} but no fixture covers it"
        );
    }
}

fn serialize_lifecycle(state: LifecycleState) -> String {
    match state {
        LifecycleState::Proposed => "proposed",
        LifecycleState::PendingReview => "pending_review",
        LifecycleState::Active => "active",
        LifecycleState::SuspendedReconnect => "suspended_reconnect",
        LifecycleState::Paused => "paused",
        LifecycleState::Degraded => "degraded",
        LifecycleState::StaleTarget => "stale_target",
        LifecycleState::PolicyDenied => "policy_denied",
        LifecycleState::ApprovalExpired => "approval_expired",
        LifecycleState::CapabilityNarrowed => "capability_narrowed",
        LifecycleState::ProviderUnavailable => "provider_unavailable",
        LifecycleState::Revoked => "revoked",
        LifecycleState::Expired => "expired",
        LifecycleState::Closed => "closed",
        LifecycleState::Blocked => "blocked",
    }
    .to_owned()
}
