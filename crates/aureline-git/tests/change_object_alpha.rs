//! Fixture-driven coverage for the alpha change-object family.

use std::path::{Path, PathBuf};

use aureline_git::{
    project_change_object, ChangeObjectError, ChangeObjectRecord, CHANGE_OBJECT_ALPHA_RECORD_KIND,
    CHANGE_OBJECT_ALPHA_SCHEMA_VERSION,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m3/change_objects")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("change-object fixtures dir must exist")
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
        "change-object fixtures dir must contain at least one fixture"
    );
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_change_object(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));

        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "change_object_inspector"),
            "fixture {path:?} must wire the change_object_inspector consumer",
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

        let record: ChangeObjectRecord =
            serde_json::from_str(&payload).expect("fixture must parse for review-invariant probe");
        assert!(record.review_invariants.inspectable_before_publish);
        assert!(record.review_invariants.inspectable_before_merge);
        assert!(record.review_invariants.inspectable_before_apply);
        assert!(record.review_invariants.no_hidden_target_mutation);
    }
}

#[test]
fn fixtures_cover_all_three_kinds_and_diverse_landing_states() {
    let paths = load_fixture_paths();
    let mut kinds = std::collections::BTreeSet::new();
    let mut landing_states = std::collections::BTreeSet::new();
    for path in paths {
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let projection = project_change_object(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must project: {err}"));
        kinds.insert(projection.change_object_kind);
        landing_states.insert(projection.landing_state_class);
    }
    for kind in ["branch", "worktree", "patch_stack"] {
        assert!(
            kinds.contains(kind),
            "fixtures must cover change_object_kind={kind}"
        );
    }
    for state in [
        "local_only_no_remote_yet",
        "pending_publish_to_remote",
        "pending_merge_into_base",
        "pending_patch_apply",
        "landed_publicly",
    ] {
        assert!(
            landing_states.contains(state),
            "fixtures must cover landing_state_class={state}"
        );
    }
}

#[test]
fn branch_fixture_reports_publish_action_class() {
    let payload = std::fs::read_to_string(fixtures_dir().join("branch_local_pending_publish.json"))
        .expect("branch fixture must read");
    let projection = project_change_object(&payload).expect("branch publish fixture must project");
    assert_eq!(projection.change_object_kind, "branch");
    assert_eq!(projection.landing_state_class, "pending_publish_to_remote");
    assert_eq!(projection.landing_action_class, "publish");
    assert_eq!(projection.mutation_authority_class, "provider_bound");
    assert_ne!(
        projection.required_network_egress_class, "no_network_egress_required",
        "publish must declare a network egress class"
    );
}

#[test]
fn worktree_fixture_stays_local_only() {
    let payload = std::fs::read_to_string(fixtures_dir().join("worktree_linked_local_only.json"))
        .expect("worktree fixture must read");
    let projection = project_change_object(&payload).expect("worktree fixture must project");
    assert_eq!(projection.change_object_kind, "worktree");
    assert_eq!(projection.landing_state_class, "local_only_no_remote_yet");
    assert_eq!(projection.remote_visibility_class, "no_remote_attached");
    assert_eq!(
        projection.required_network_egress_class,
        "no_network_egress_required"
    );
    assert_eq!(projection.mutation_authority_class, "local_only");
}

#[test]
fn patch_stack_fixture_reports_apply_action_class() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("patch_stack_provider_pull_request.json"))
            .expect("patch-stack fixture must read");
    let projection = project_change_object(&payload).expect("patch-stack fixture must project");
    assert_eq!(projection.change_object_kind, "patch_stack");
    assert_eq!(projection.landing_state_class, "pending_patch_apply");
    assert_eq!(projection.landing_action_class, "apply");
}

#[test]
fn rejects_pending_publish_without_remote_attached() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("branch_local_pending_publish.json")).unwrap();
    let mut record: ChangeObjectRecord =
        serde_json::from_str(&payload).expect("branch fixture must parse");
    record.landing_state.remote_visibility_class = "no_remote_attached".to_string();
    let err = record
        .validate()
        .expect_err("publish without remote must fail");
    assert!(err.message().contains("remote-attached"));
}

#[test]
fn rejects_local_only_with_remote_egress() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("worktree_linked_local_only.json")).unwrap();
    let mut record: ChangeObjectRecord =
        serde_json::from_str(&payload).expect("worktree fixture must parse");
    record.landing_state.required_network_egress_class = "first_party_origin_only".to_string();
    let err = record
        .validate()
        .expect_err("local_only_no_remote_yet must not declare a remote egress envelope");
    assert!(err.message().contains("required_network_egress_class"));
}

#[test]
fn rejects_branch_with_patch_stack_variant() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("branch_local_pending_publish.json")).unwrap();
    let mut record: ChangeObjectRecord =
        serde_json::from_str(&payload).expect("branch fixture must parse");
    record.patch_stack = Some(aureline_git::ChangeObjectPatchStackVariant {
        patch_stack_target_class: "current_branch".to_string(),
        patch_state_class: "drafted".to_string(),
        patch_count: 1,
        top_patch_label: "ref:patch:phantom".to_string(),
        review_class: None,
        review_handle_label: None,
    });
    let err = record
        .validate()
        .expect_err("branch must not carry a patch-stack variant block");
    assert!(err.message().contains("patch_stack"));
}

#[test]
fn rejects_missing_inspector_consumer() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("patch_stack_provider_pull_request.json"))
            .unwrap();
    let mut record: ChangeObjectRecord =
        serde_json::from_str(&payload).expect("patch-stack fixture must parse");
    record
        .consumer_surfaces
        .retain(|surface| surface != "change_object_inspector");
    let err = record
        .validate()
        .expect_err("consumer_surfaces without change_object_inspector must fail");
    assert!(err.message().contains("change_object_inspector"));
}

#[test]
fn rejects_raw_branch_name_export() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("branch_local_pending_publish.json")).unwrap();
    let mut record: ChangeObjectRecord =
        serde_json::from_str(&payload).expect("branch fixture must parse");
    record.support_export.raw_branch_name_export_allowed = true;
    let err = record
        .validate()
        .expect_err("must reject raw branch-name export");
    assert!(err.message().contains("raw_"));
}

#[test]
fn rejects_invalid_record_kind_via_project() {
    let payload =
        std::fs::read_to_string(fixtures_dir().join("branch_local_pending_publish.json")).unwrap();
    let tampered = payload.replace(
        "\"record_kind\": \"change_object_alpha_record\"",
        "\"record_kind\": \"some_other_record_kind\"",
    );
    match project_change_object(&tampered) {
        Err(ChangeObjectError::Validation(err)) => {
            assert!(err.message().contains("record_kind"));
        }
        other => panic!("expected validation failure, got {other:?}"),
    }
}

#[test]
fn schema_version_constants_match_record_kind() {
    assert_eq!(CHANGE_OBJECT_ALPHA_SCHEMA_VERSION, 1);
    assert_eq!(
        CHANGE_OBJECT_ALPHA_RECORD_KIND,
        "change_object_alpha_record"
    );
}
