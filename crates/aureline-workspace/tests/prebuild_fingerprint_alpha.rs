//! Fixture-driven coverage for the alpha prebuild fingerprint, reuse
//! decision, and disclosure records.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    parse_prebuild_alpha_record, project_prebuild_fingerprint_alpha, PrebuildAlphaRecord,
    PrebuildDisclosureRecord, PrebuildFingerprintRecord, PrebuildReuseDecisionRecord,
    PREBUILD_DISCLOSURE_RECORD_KIND, PREBUILD_FINGERPRINT_RECORD_KIND,
    PREBUILD_REUSE_DECISION_RECORD_KIND,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m3/prebuild_fingerprint")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("prebuild_fingerprint fixtures dir must exist")
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
        "prebuild_fingerprint fixtures dir must contain at least one fixture"
    );
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_prebuild_fingerprint_alpha(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        assert!(
            projection.stale_snapshot_must_not_be_labeled_live_resume,
            "fixture {path:?} must keep the resume-live invariant true",
        );
    }
}

#[test]
fn fingerprint_fixture_carries_full_coverage() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("valid_cached_prebuild_fingerprint.json"))
            .expect("fingerprint fixture must read");
    let record = parse_prebuild_alpha_record(&payload).expect("fingerprint must parse");
    let fingerprint = match record {
        PrebuildAlphaRecord::Fingerprint(record) => record,
        other => panic!("expected fingerprint record, got {other:?}"),
    };
    assert_eq!(fingerprint.record_kind, PREBUILD_FINGERPRINT_RECORD_KIND);
    assert_eq!(
        fingerprint.freshness.freshness_age_class,
        "fresh_under_window"
    );
    assert!(fingerprint
        .redaction_and_portability
        .excluded_residue_classes
        .iter()
        .any(|c| c == "machine_unique_trust_anchors"));
    assert!(!fingerprint.cache_artifacts.is_empty());
}

#[test]
fn reuse_allowed_fixture_clears_invalidation_set() {
    let payload = std::fs::read_to_string(fixtures_dir().join("reuse_allowed_decision.json"))
        .expect("reuse-allowed fixture must read");
    let record = parse_prebuild_alpha_record(&payload).expect("reuse decision must parse");
    let decision = match record {
        PrebuildAlphaRecord::ReuseDecision(record) => record,
        other => panic!("expected reuse decision record, got {other:?}"),
    };
    assert_eq!(decision.record_kind, PREBUILD_REUSE_DECISION_RECORD_KIND);
    assert_eq!(decision.reuse_outcome, "reuse_allowed");
    assert!(decision.invalidation_bundle_refs.is_empty());
    assert!(decision.required_revalidations.is_empty());
}

#[test]
fn stale_snapshot_resume_request_is_denied() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("stale_snapshot_resume_denied_decision.json"))
            .expect("stale-snapshot fixture must read");
    let record = parse_prebuild_alpha_record(&payload).expect("stale-snapshot must parse");
    let decision = match record {
        PrebuildAlphaRecord::ReuseDecision(record) => record,
        other => panic!("expected reuse decision record, got {other:?}"),
    };
    assert_eq!(decision.requested_path, "resume_live_workspace");
    assert_eq!(
        decision.source_materialization_class,
        "stale_prebuild_snapshot"
    );
    assert_eq!(decision.reuse_outcome, "resume_live_denied");
}

#[test]
fn local_override_disclosure_requires_rebuild() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("local_override_rebuild_disclosure.json"))
            .expect("local-override fixture must read");
    let record = parse_prebuild_alpha_record(&payload).expect("disclosure must parse");
    let disclosure = match record {
        PrebuildAlphaRecord::Disclosure(record) => record,
        other => panic!("expected disclosure record, got {other:?}"),
    };
    assert_eq!(disclosure.record_kind, PREBUILD_DISCLOSURE_RECORD_KIND);
    assert!(disclosure.rebuild_required);
    assert!(disclosure.local_override_disclosed);
    assert_eq!(
        disclosure.disclosure_state,
        "local_override_rebuild_required"
    );
    assert!(disclosure
        .excluded_residue_classes
        .iter()
        .any(|c| c == "uncommitted_workspace_edits"));
}

#[test]
fn fresh_clone_disclosure_stays_distinct() {
    let payload = std::fs::read_to_string(fixtures_dir().join("fresh_clone_disclosure.json"))
        .expect("fresh-clone fixture must read");
    let record = parse_prebuild_alpha_record(&payload).expect("disclosure must parse");
    let disclosure = match record {
        PrebuildAlphaRecord::Disclosure(record) => record,
        other => panic!("expected disclosure record, got {other:?}"),
    };
    assert_eq!(disclosure.disclosure_state, "fresh_clone");
    assert!(disclosure.fresh_clone_required);
    assert!(!disclosure.rebuild_required);
    assert!(disclosure
        .alternative_lane_refs
        .iter()
        .any(|c| c == "reuse_cached_prebuild"));
}

#[test]
fn rejects_resume_live_on_snapshot_with_widened_outcome() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("stale_snapshot_resume_denied_decision.json"))
            .expect("stale-snapshot fixture must read");
    let mut decision: PrebuildReuseDecisionRecord =
        serde_json::from_str(&payload).expect("decision must parse");
    decision.reuse_outcome = "reuse_after_revalidation".to_string();
    let err = decision
        .validate()
        .expect_err("resume on snapshot cannot be widened");
    assert!(err.message().to_lowercase().contains("resume_live_denied"));
}

#[test]
fn rejects_fingerprint_with_widened_capture() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("valid_cached_prebuild_fingerprint.json"))
            .expect("fingerprint fixture must read");
    let mut fingerprint: PrebuildFingerprintRecord =
        serde_json::from_str(&payload).expect("fingerprint must parse");
    fingerprint
        .redaction_and_portability
        .broadened_capture_approved = true;
    let err = fingerprint
        .validate()
        .expect_err("broadened capture must be rejected");
    assert!(err
        .message()
        .to_lowercase()
        .contains("broadened_capture_approved"));
}

#[test]
fn rejects_disclosure_with_missing_residue_exclusion() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("local_override_rebuild_disclosure.json"))
            .expect("disclosure fixture must read");
    let mut disclosure: PrebuildDisclosureRecord =
        serde_json::from_str(&payload).expect("disclosure must parse");
    disclosure
        .excluded_residue_classes
        .retain(|c| c != "raw_secret_material");
    let err = disclosure
        .validate()
        .expect_err("disclosure must keep raw_secret_material excluded");
    assert!(err.message().contains("raw_secret_material"));
}
