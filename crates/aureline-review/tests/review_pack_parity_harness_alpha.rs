//! Fixture-driven coverage for the alpha review-pack parity-harness family.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_review_pack_parity_harness, ReviewPackParityHarnessDriftDowngrade,
    ReviewPackParityHarnessError, ReviewPackParityHarnessRecord,
    REVIEW_PACK_PARITY_HARNESS_ALPHA_HARNESS_VERSION,
    REVIEW_PACK_PARITY_HARNESS_ALPHA_RECORD_KIND,
    REVIEW_PACK_PARITY_HARNESS_ALPHA_SCHEMA_VERSION,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/m3/review_pack_harness")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("review-pack parity-harness fixtures dir must exist")
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
        "review-pack parity-harness fixtures dir must contain at least one fixture"
    );
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_review_pack_parity_harness(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));

        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "parity_harness_inspector"),
            "fixture {path:?} must wire the parity_harness_inspector consumer",
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
        assert_eq!(
            projection.harness_version,
            REVIEW_PACK_PARITY_HARNESS_ALPHA_HARNESS_VERSION
        );
        assert_eq!(
            projection.schema_version,
            REVIEW_PACK_PARITY_HARNESS_ALPHA_SCHEMA_VERSION
        );
        assert!(
            projection.review_pack_ref.starts_with("review_pack_alpha:"),
            "fixture {path:?} must reference a review_pack_alpha id"
        );
        assert!(
            projection
                .parity_harness_id
                .starts_with("review_pack_parity_harness_alpha:"),
            "fixture {path:?} must mint a review_pack_parity_harness_alpha id"
        );

        let record: ReviewPackParityHarnessRecord = serde_json::from_str(&payload)
            .expect("fixture must parse for review-invariant probe");
        assert!(record.review_invariants.review_pack_ref_pinned);
        assert!(record.review_invariants.harness_lanes_pinned);
        assert!(record.review_invariants.check_findings_pinned);
        assert!(record.review_invariants.drift_downgrades_pinned);
        assert!(record.review_invariants.overall_verdict_pinned);
        assert!(record.review_invariants.no_hidden_writes);
    }
}

#[test]
fn fixtures_cover_all_four_authority_classes() {
    let paths = load_fixture_paths();
    let mut authorities = std::collections::BTreeSet::new();
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_review_pack_parity_harness(&payload)
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
fn fixtures_cover_drift_downgrade_path() {
    let paths = load_fixture_paths();
    let mut saw_drift = false;
    let mut saw_full_parity = false;
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_review_pack_parity_harness(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        if projection.overall_verdict_class == "drift_downgraded" {
            saw_drift = true;
            assert!(
                projection.row_downgrade_class != "no_downgrade",
                "drift_downgraded verdict must downgrade the row in {path:?}"
            );
            assert!(
                projection.downgrade_count >= 1,
                "drift_downgraded verdict must list at least one drift_downgrades entry in {path:?}"
            );
        }
        if projection.overall_verdict_class == "full_parity" {
            saw_full_parity = true;
            assert_eq!(
                projection.row_downgrade_class, "no_downgrade",
                "full_parity verdict must not downgrade the row in {path:?}"
            );
            assert_eq!(
                projection.downgrade_count, 0,
                "full_parity verdict must carry no downgrades in {path:?}"
            );
        }
    }
    assert!(
        saw_drift,
        "fixtures must include at least one drift_downgraded run"
    );
    assert!(
        saw_full_parity,
        "fixtures must include at least one full_parity run"
    );
}

#[test]
fn fixtures_cover_parity_finding_spread() {
    let paths = load_fixture_paths();
    let mut classes = std::collections::BTreeSet::new();
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_review_pack_parity_harness(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        for finding in projection.check_findings {
            classes.insert(finding.parity_finding_class);
        }
    }
    for class in [
        "full_parity",
        "local_only_documented_match",
        "ci_only_documented_match",
        "drift_detected",
    ] {
        assert!(
            classes.contains(class),
            "fixtures must cover parity_finding_class={class}"
        );
    }
}

#[test]
fn community_fixture_drift_downgrades_the_row() {
    let path = fixtures_dir().join("uncertified_community_drift_downgrade.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    let projection = project_review_pack_parity_harness(&payload)
        .expect("community fixture must project");
    assert_eq!(projection.overall_verdict_class, "drift_downgraded");
    assert_eq!(
        projection.row_downgrade_class,
        "downgraded_to_review_required"
    );
    assert!(projection.drift_detected_count >= 1);
    assert!(projection.downgrade_count >= 1);
}

#[test]
fn rejects_drift_finding_without_downgrade() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("uncertified_community_drift_downgrade.json"))
            .unwrap();
    let mut record: ReviewPackParityHarnessRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.drift_downgrades.clear();
    let err = record
        .validate()
        .expect_err("drift finding without downgrade must fail");
    assert!(err.message().contains("drift"));
}

#[test]
fn rejects_drift_finding_with_no_downgrade_row() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("uncertified_community_drift_downgrade.json"))
            .unwrap();
    let mut record: ReviewPackParityHarnessRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.row_downgrade_class = "no_downgrade".to_string();
    record.overall_verdict_class = "full_parity".to_string();
    let err = record
        .validate()
        .expect_err("drift finding with no row downgrade must fail");
    assert!(
        err.message().contains("downgrade") || err.message().contains("full_parity"),
        "unexpected message: {}",
        err.message()
    );
}

#[test]
fn rejects_downgrade_referencing_unknown_check() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("uncertified_community_drift_downgrade.json"))
            .unwrap();
    let mut record: ReviewPackParityHarnessRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record
        .drift_downgrades
        .push(ReviewPackParityHarnessDriftDowngrade {
            check_ref: "review_pack_check:phantom".to_string(),
            downgrade_class: "downgraded_to_advisory".to_string(),
            summary: "Phantom downgrade.".to_string(),
        });
    let err = record
        .validate()
        .expect_err("phantom check_ref in drift_downgrades must fail");
    assert!(err.message().contains("phantom") || err.message().contains("not present"));
}

#[test]
fn rejects_raw_command_export() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_full_parity_run.json")).unwrap();
    let mut record: ReviewPackParityHarnessRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.support_export.raw_command_export_allowed = true;
    let err = record
        .validate()
        .expect_err("raw command export must fail");
    assert!(err.message().contains("raw_"));
}

#[test]
fn rejects_missing_parity_harness_inspector_consumer() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("team_shared_mixed_parity_documented.json"))
            .unwrap();
    let mut record: ReviewPackParityHarnessRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record
        .consumer_surfaces
        .retain(|surface| surface != "parity_harness_inspector");
    let err = record
        .validate()
        .expect_err("missing parity_harness_inspector must fail");
    assert!(err.message().contains("parity_harness_inspector"));
}

#[test]
fn rejects_invalid_record_kind_via_project() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("first_party_full_parity_run.json")).unwrap();
    let tampered = payload.replace(
        "\"record_kind\": \"review_pack_parity_harness_alpha_record\"",
        "\"record_kind\": \"some_other_record_kind\"",
    );
    match project_review_pack_parity_harness(&tampered) {
        Err(ReviewPackParityHarnessError::Validation(err)) => {
            assert!(err.message().contains("record_kind"));
        }
        other => panic!("expected validation failure, got {other:?}"),
    }
}

#[test]
fn schema_version_constants_match_record_kind() {
    assert_eq!(REVIEW_PACK_PARITY_HARNESS_ALPHA_SCHEMA_VERSION, 1);
    assert_eq!(REVIEW_PACK_PARITY_HARNESS_ALPHA_HARNESS_VERSION, 1);
    assert_eq!(
        REVIEW_PACK_PARITY_HARNESS_ALPHA_RECORD_KIND,
        "review_pack_parity_harness_alpha_record"
    );
}

#[test]
fn fixture_review_pack_refs_align_with_dsl_fixtures() {
    use std::collections::BTreeSet;
    let parity_dir = fixtures_dir();
    let parity_paths: Vec<PathBuf> = std::fs::read_dir(&parity_dir)
        .expect("parity fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    let mut referenced: BTreeSet<String> = BTreeSet::new();
    for path in parity_paths {
        let payload = std::fs::read_to_string(&path).expect("parity fixture must read");
        let projection = project_review_pack_parity_harness(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        referenced.insert(projection.review_pack_ref);
    }
    let dsl_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/m3/review_pack_dsl");
    let mut known: BTreeSet<String> = BTreeSet::new();
    for entry in std::fs::read_dir(&dsl_dir).expect("review-pack DSL fixtures dir must exist") {
        let path = entry.expect("dir entry must read").path();
        if path.extension().is_some_and(|ext| ext == "json") {
            let payload = std::fs::read_to_string(&path).expect("DSL fixture must read");
            let value: serde_json::Value =
                serde_json::from_str(&payload).expect("DSL fixture must parse");
            if let Some(id) = value.get("review_pack_id").and_then(|v| v.as_str()) {
                known.insert(id.to_string());
            }
        }
    }
    for reference in &referenced {
        assert!(
            known.contains(reference),
            "parity fixture references review_pack_ref {reference} that no DSL fixture defines"
        );
    }
}
