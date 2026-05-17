//! Fixture-driven coverage for the alpha review-pack DSL family.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_review_pack, ReviewPackError, ReviewPackRecord, REVIEW_PACK_ALPHA_DSL_VERSION,
    REVIEW_PACK_ALPHA_RECORD_KIND, REVIEW_PACK_ALPHA_SCHEMA_VERSION,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/m3/review_pack_dsl")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("review-pack DSL fixtures dir must exist")
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
        "review-pack DSL fixtures dir must contain at least one fixture"
    );
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_review_pack(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));

        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "review_pack_inspector"),
            "fixture {path:?} must wire the review_pack_inspector consumer",
        );
        assert!(
            !projection.raw_path_export_allowed,
            "fixture {path:?} must keep raw paths closed",
        );
        assert!(
            !projection.raw_glob_body_export_allowed,
            "fixture {path:?} must keep raw glob bodies closed",
        );
        assert!(
            !projection.raw_command_export_allowed,
            "fixture {path:?} must keep raw command lines closed",
        );
        assert!(
            !projection.raw_check_output_export_allowed,
            "fixture {path:?} must keep raw check outputs closed",
        );
        assert_eq!(projection.dsl_version, REVIEW_PACK_ALPHA_DSL_VERSION);
        assert_eq!(projection.schema_version, REVIEW_PACK_ALPHA_SCHEMA_VERSION);

        let record: ReviewPackRecord =
            serde_json::from_str(&payload).expect("fixture must parse for review-invariant probe");
        assert!(record.review_invariants.repo_anchor_pinned);
        assert!(record.review_invariants.checks_pinned);
        assert!(record.review_invariants.ownership_hints_pinned);
        assert!(record.review_invariants.local_ci_parity_declared);
        assert!(record.review_invariants.unsupported_fields_declared);
        assert!(record.review_invariants.no_hidden_writes);
    }
}

#[test]
fn fixtures_cover_all_four_authority_classes() {
    let paths = load_fixture_paths();
    let mut authorities = std::collections::BTreeSet::new();
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_review_pack(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        authorities.insert(projection.pack_authority_class);
    }
    for class in [
        "repo_first_party",
        "repo_team_shared",
        "repo_partner_signed",
        "repo_uncertified_community",
    ] {
        assert!(
            authorities.contains(class),
            "fixtures must cover pack_authority_class={class}"
        );
    }
}

#[test]
fn fixtures_cover_parity_class_spread() {
    let paths = load_fixture_paths();
    let mut classes = std::collections::BTreeSet::new();
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_review_pack(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        for class in projection.local_ci_parity_classes {
            classes.insert(class);
        }
    }
    for class in [
        "local_and_ci_parity",
        "ci_only_documented",
        "local_only_documented",
        "parity_unknown_requires_review",
    ] {
        assert!(
            classes.contains(class),
            "fixtures must cover parity_class={class}"
        );
    }
}

#[test]
fn first_party_fixture_reports_blocking_checks() {
    let path = fixtures_dir().join("first_party_local_and_ci_parity.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    let projection = project_review_pack(&payload).expect("first-party fixture must project");
    assert_eq!(projection.pack_authority_class, "repo_first_party");
    assert!(projection.blocking_check_count >= 1);
    assert_eq!(projection.local_only_check_count, 0);
    assert_eq!(projection.ci_only_check_count, 0);
}

#[test]
fn team_shared_fixture_reports_local_only_and_ci_only_counts() {
    let path = fixtures_dir().join("team_shared_mixed_parity.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    let projection = project_review_pack(&payload).expect("team-shared fixture must project");
    assert_eq!(projection.pack_authority_class, "repo_team_shared");
    assert_eq!(projection.local_only_check_count, 1);
    assert_eq!(projection.ci_only_check_count, 1);
    assert!(!projection.unsupported_fields.is_empty());
}

#[test]
fn partner_signed_fixture_reports_ci_only_audit() {
    let path = fixtures_dir().join("partner_signed_ci_only_lane.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    let projection = project_review_pack(&payload).expect("partner-signed fixture must project");
    assert_eq!(projection.pack_authority_class, "repo_partner_signed");
    assert_eq!(projection.ci_only_check_count, 1);
}

#[test]
fn community_fixture_carries_unsupported_field_declarations() {
    let path = fixtures_dir().join("uncertified_community_local_only_lane.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    let projection = project_review_pack(&payload).expect("community fixture must project");
    assert_eq!(
        projection.pack_authority_class,
        "repo_uncertified_community"
    );
    assert!(projection.local_only_check_count >= 1);
    assert!(
        projection
            .unsupported_fields
            .iter()
            .any(|f| f.unsupported_class == "experimental_local_only"),
        "community fixture must declare an experimental_local_only unsupported field"
    );
    assert!(
        projection
            .unsupported_fields
            .iter()
            .any(|f| f.unsupported_class == "deprecated_pending_removal"),
        "community fixture must declare a deprecated_pending_removal unsupported field"
    );
}

#[test]
fn rejects_check_with_undeclared_ownership_scope() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_local_and_ci_parity.json"))
            .unwrap();
    let mut record: ReviewPackRecord = serde_json::from_str(&payload).expect("fixture must parse");
    record.checks[0]
        .ownership_scope_refs
        .push("ownership_scope:phantom".to_string());
    let err = record
        .validate()
        .expect_err("undeclared ownership scope refs must fail");
    assert!(err.message().contains("not declared"));
}

#[test]
fn rejects_check_parity_class_without_observation() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_local_and_ci_parity.json"))
            .unwrap();
    let mut record: ReviewPackRecord = serde_json::from_str(&payload).expect("fixture must parse");
    record.checks[0].parity_class = "ci_only_documented".to_string();
    record
        .parity_observations
        .retain(|o| o.parity_class != "ci_only_documented");
    let err = record
        .validate()
        .expect_err("check parity_class without an observation must fail");
    assert!(err.message().contains("parity_observation"));
}

#[test]
fn rejects_raw_command_export() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_local_and_ci_parity.json"))
            .unwrap();
    let mut record: ReviewPackRecord = serde_json::from_str(&payload).expect("fixture must parse");
    record.support_export.raw_command_export_allowed = true;
    let err = record
        .validate()
        .expect_err("must reject raw command export");
    assert!(err.message().contains("raw_"));
}

#[test]
fn rejects_missing_review_pack_inspector_consumer() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("team_shared_mixed_parity.json")).unwrap();
    let mut record: ReviewPackRecord = serde_json::from_str(&payload).expect("fixture must parse");
    record
        .consumer_surfaces
        .retain(|surface| surface != "review_pack_inspector");
    let err = record
        .validate()
        .expect_err("consumer_surfaces without review_pack_inspector must fail");
    assert!(err.message().contains("review_pack_inspector"));
}

#[test]
fn rejects_invalid_record_kind_via_project() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_local_and_ci_parity.json"))
            .unwrap();
    let tampered = payload.replace(
        "\"record_kind\": \"review_pack_alpha_record\"",
        "\"record_kind\": \"some_other_record_kind\"",
    );
    match project_review_pack(&tampered) {
        Err(ReviewPackError::Validation(err)) => {
            assert!(err.message().contains("record_kind"));
        }
        other => panic!("expected validation failure, got {other:?}"),
    }
}

#[test]
fn schema_version_constants_match_record_kind() {
    assert_eq!(REVIEW_PACK_ALPHA_SCHEMA_VERSION, 1);
    assert_eq!(REVIEW_PACK_ALPHA_DSL_VERSION, 1);
    assert_eq!(REVIEW_PACK_ALPHA_RECORD_KIND, "review_pack_alpha_record");
}

#[test]
fn fixture_ids_use_review_pack_alpha_prefix() {
    let paths = load_fixture_paths();
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_review_pack(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        assert!(
            projection.review_pack_id.starts_with("review_pack_alpha:"),
            "fixture {path:?} must mint a review_pack_alpha id"
        );
        assert!(
            projection.repo_anchor_ref.starts_with("repo_anchor:"),
            "fixture {path:?} must mint a repo_anchor id"
        );
    }
}
