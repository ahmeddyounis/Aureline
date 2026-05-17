//! Fixture-driven coverage for the alpha change-lineage family.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_change_lineage, ChangeLineageError, ChangeLineageRecord,
    CHANGE_LINEAGE_ALPHA_RECORD_KIND, CHANGE_LINEAGE_ALPHA_SCHEMA_VERSION,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/m3/change_lineage")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("change-lineage fixtures dir must exist")
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
        "change-lineage fixtures dir must contain at least one fixture"
    );
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_change_lineage(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));

        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "change_inspector"),
            "fixture {path:?} must wire the change_inspector consumer",
        );
        assert!(
            !projection.raw_path_export_allowed,
            "fixture {path:?} must keep raw paths closed",
        );
        assert!(
            !projection.raw_branch_name_export_allowed,
            "fixture {path:?} must keep raw branch names closed",
        );
        assert!(
            !projection.raw_remote_url_export_allowed,
            "fixture {path:?} must keep raw remote URLs closed",
        );
        assert!(
            !projection.raw_diff_body_export_allowed,
            "fixture {path:?} must keep raw diff bodies closed",
        );

        let record: ChangeLineageRecord =
            serde_json::from_str(&payload).expect("fixture must parse for review-invariant probe");
        assert!(record.review_invariants.target_ref_pinned);
        assert!(record.review_invariants.ancestry_pinned);
        assert!(record.review_invariants.conflict_state_inspectable);
        assert!(record.review_invariants.publish_readiness_inspectable);
        assert!(record.review_invariants.no_hidden_target_mutation);
    }
}

#[test]
fn fixtures_cover_all_three_kinds_and_diverse_scopes() {
    let paths = load_fixture_paths();
    let mut kinds = std::collections::BTreeSet::new();
    let mut scopes = std::collections::BTreeSet::new();
    let mut readiness = std::collections::BTreeSet::new();
    let mut conflict_states = std::collections::BTreeSet::new();
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_change_lineage(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        kinds.insert(projection.change_object_kind);
        scopes.insert(projection.active_scope_class);
        readiness.insert(projection.publish_readiness_class);
        conflict_states.insert(projection.conflict_state_class);
    }
    for kind in ["branch", "worktree", "patch_stack"] {
        assert!(
            kinds.contains(kind),
            "fixtures must cover change_object_kind={kind}"
        );
    }
    for scope in ["main_worktree", "side_worktree", "stacked_patch_set"] {
        assert!(
            scopes.contains(scope),
            "fixtures must cover active_scope_class={scope} so users can tell where they are"
        );
    }
    for class in [
        "ready_to_publish",
        "blocked_by_review_required",
        "blocked_by_conflicts",
        "not_applicable_inspect_only",
    ] {
        assert!(
            readiness.contains(class),
            "fixtures must cover publish_readiness_class={class}"
        );
    }
    assert!(
        conflict_states.contains("no_conflicts_detected"),
        "at least one fixture must report no_conflicts_detected"
    );
    assert!(
        conflict_states
            .iter()
            .any(|state| state != "no_conflicts_detected"),
        "at least one fixture must report a non-clean conflict state"
    );
}

#[test]
fn ready_to_publish_fixture_reports_publish_action() {
    let path = fixtures_dir().join("branch_main_worktree_ready_to_publish.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    let projection =
        project_change_lineage(&payload).expect("ready_to_publish fixture must project");
    assert_eq!(projection.active_scope_class, "main_worktree");
    assert_eq!(projection.publish_readiness_class, "ready_to_publish");
    assert_eq!(projection.landing_action_class, "publish");
    assert_eq!(projection.conflict_state_class, "no_conflicts_detected");
    assert_eq!(
        projection.change_object_ref, "change_object_alpha:branch:feature_landing_inspector",
        "lineage record must quote the underlying change-object id verbatim"
    );
}

#[test]
fn side_worktree_fixture_keeps_local_only() {
    let path = fixtures_dir().join("worktree_side_worktree_inspect_only.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    let projection =
        project_change_lineage(&payload).expect("worktree inspect-only fixture must project");
    assert_eq!(projection.active_scope_class, "side_worktree");
    assert_eq!(
        projection.publish_readiness_class,
        "not_applicable_inspect_only"
    );
    assert_eq!(projection.remote_visibility_class, "no_remote_attached");
    assert_eq!(
        projection.required_network_egress_class,
        "no_network_egress_required"
    );
}

#[test]
fn patch_stack_fixture_reports_rebase_blocker() {
    let path = fixtures_dir().join("patch_stack_blocked_by_conflicts.json");
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    let projection = project_change_lineage(&payload).expect("patch-stack fixture must project");
    assert_eq!(projection.active_scope_class, "stacked_patch_set");
    assert_eq!(projection.publish_readiness_class, "blocked_by_conflicts");
    assert!(
        projection
            .readiness_blockers
            .iter()
            .any(|blocker| blocker == "rebase_required"),
        "patch-stack fixture must declare a rebase blocker explicitly"
    );
}

#[test]
fn rejects_branch_with_stacked_patch_scope() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("branch_main_worktree_ready_to_publish.json"))
            .unwrap();
    let mut record: ChangeLineageRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.active_scope_class = "stacked_patch_set".to_string();
    let err = record
        .validate()
        .expect_err("branch must not open the stacked_patch_set scope");
    assert!(err.message().contains("stacked_patch_set"));
}

#[test]
fn rejects_ready_to_apply_with_pending_conflicts() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("patch_stack_blocked_by_conflicts.json"))
            .unwrap();
    let mut record: ChangeLineageRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.publish_readiness.publish_readiness_class = "ready_to_apply".to_string();
    record.publish_readiness.blockers = vec!["no_blockers".to_string()];
    let err = record
        .validate()
        .expect_err("ready_to_apply must not pair with pending conflicts");
    assert!(err.message().contains("conflict") || err.message().contains("no_conflicts_detected"));
}

#[test]
fn rejects_missing_change_inspector_consumer() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("branch_landed_publicly_inspect_only.json"))
            .unwrap();
    let mut record: ChangeLineageRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record
        .consumer_surfaces
        .retain(|surface| surface != "change_inspector");
    let err = record
        .validate()
        .expect_err("consumer_surfaces without change_inspector must fail");
    assert!(err.message().contains("change_inspector"));
}

#[test]
fn rejects_raw_remote_url_export() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("branch_main_worktree_ready_to_publish.json"))
            .unwrap();
    let mut record: ChangeLineageRecord =
        serde_json::from_str(&payload).expect("fixture must parse");
    record.support_export.raw_remote_url_export_allowed = true;
    let err = record
        .validate()
        .expect_err("must reject raw remote URL export");
    assert!(err.message().contains("raw_"));
}

#[test]
fn rejects_invalid_record_kind_via_project() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("branch_main_worktree_ready_to_publish.json"))
            .unwrap();
    let tampered = payload.replace(
        "\"record_kind\": \"change_lineage_alpha_record\"",
        "\"record_kind\": \"some_other_record_kind\"",
    );
    match project_change_lineage(&tampered) {
        Err(ChangeLineageError::Validation(err)) => {
            assert!(err.message().contains("record_kind"));
        }
        other => panic!("expected validation failure, got {other:?}"),
    }
}

#[test]
fn lineage_record_quotes_matching_change_object_id() {
    let paths = load_fixture_paths();
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_change_lineage(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        assert!(
            projection
                .change_object_ref
                .starts_with("change_object_alpha:"),
            "fixture {path:?} must quote a change-object alpha id"
        );
        assert!(
            projection
                .change_lineage_id
                .starts_with("change_lineage_alpha:"),
            "fixture {path:?} must mint a change-lineage alpha id"
        );
    }
}

#[test]
fn schema_version_constants_match_record_kind() {
    assert_eq!(CHANGE_LINEAGE_ALPHA_SCHEMA_VERSION, 1);
    assert_eq!(
        CHANGE_LINEAGE_ALPHA_RECORD_KIND,
        "change_lineage_alpha_record"
    );
}
