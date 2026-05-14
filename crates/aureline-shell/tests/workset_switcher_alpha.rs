//! Fixture-backed coverage for workset switching and scope-diff review truth.

use std::path::{Path, PathBuf};

use aureline_shell::workset_switcher::{
    project_scope_banner, project_workset_switcher, BannerAction, BannerState, ReviewAction,
    ScopeBannerRecord, ScopeDiffReviewRecord, SwitcherAction, SwitcherRowClass, TrustClass,
    TrustPolicyNote, WorksetSwitcherRecord,
};
use aureline_workspace::{
    ReadinessState, ScopeClass, ScopeWidenDiffRecord, SourceClass, WorksetArtifactRecord,
    WorksetScopeConsumerClass,
};

fn repo_fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{rel}"))
}

fn read_json_fixture<T: serde::de::DeserializeOwned>(rel: &str) -> T {
    let path = repo_fixture(rel);
    let payload = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{rel}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("{rel}: {err}"))
}

fn read_yaml_fixture<T: serde::de::DeserializeOwned>(rel: &str) -> T {
    let path = repo_fixture(rel);
    let payload = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{rel}: {err}"));
    serde_yaml::from_str(&payload).unwrap_or_else(|err| panic!("{rel}: {err}"))
}

fn workset_artifact(rel: &str) -> WorksetArtifactRecord {
    let artifact: WorksetArtifactRecord = read_json_fixture(rel);
    artifact
        .validate()
        .unwrap_or_else(|err| panic!("{rel} artifact must validate: {err}"));
    artifact
}

#[test]
fn checked_in_switcher_and_banner_fixtures_validate() {
    let switcher: WorksetSwitcherRecord = read_yaml_fixture(
        "fixtures/workspace/workset_cross_repo_cases/multi_repo_workset_switcher.yaml",
    );
    switcher.validate().expect("switcher fixture validates");
    assert_eq!(switcher.rows.len(), 5);
    let active = switcher
        .rows
        .iter()
        .find(|row| row.is_active)
        .expect("active row");
    assert_eq!(active.workset_ref, switcher.active_workset_ref);
    assert_eq!(active.switcher_row_class, SwitcherRowClass::NamedWorksetRow);
    assert_eq!(active.scope_class, ScopeClass::SelectedWorkset);
    assert_eq!(active.repo_count, 3);
    assert!(active
        .offered_actions
        .contains(&SwitcherAction::OpenScopeDiff));

    for rel in [
        "fixtures/workspace/workset_cross_repo_cases/warm_vs_cold_scope_switch.yaml",
        "fixtures/workspace/workset_cross_repo_cases/policy_limited_workset_banner.yaml",
    ] {
        let banner: ScopeBannerRecord = read_yaml_fixture(rel);
        banner
            .validate()
            .unwrap_or_else(|err| panic!("{rel} banner must validate: {err}"));
    }
}

#[test]
fn shell_projects_switcher_rows_from_workset_artifacts() {
    let selected =
        workset_artifact("fixtures/workspace/workset_examples/selected_workset_multi_root.json");
    let current =
        workset_artifact("fixtures/workspace/workset_examples/current_repo_fallback.json");
    let full =
        workset_artifact("fixtures/workspace/workset_examples/full_workspace_multi_root.json");
    let sparse =
        workset_artifact("fixtures/workspace/workset_examples/sparse_slice_pattern_narrowed.json");
    let policy =
        workset_artifact("fixtures/workspace/workset_examples/policy_limited_admin_hidden.json");
    let artifacts = vec![selected.clone(), current, full, sparse, policy];

    let switcher = project_workset_switcher(
        "switcher:payments:projected",
        "wksp:payments:monorepo",
        &selected.workset_id,
        &artifacts,
        "mono:test",
    );
    switcher.validate().expect("projected switcher validates");
    let active = switcher
        .rows
        .iter()
        .find(|row| row.is_active)
        .expect("active row");
    assert_eq!(active.workset_ref, selected.workset_id);
    assert_eq!(active.switcher_row_class, SwitcherRowClass::NamedWorksetRow);
    assert_eq!(active.repo_count, selected.root_refs.len() as u32);
    assert!(active
        .offered_actions
        .contains(&SwitcherAction::ExportWorksetArtifact));
    assert!(active
        .offered_actions
        .contains(&SwitcherAction::OpenScopeDiff));

    let sparse_row = switcher
        .rows
        .iter()
        .find(|row| row.switcher_row_class == SwitcherRowClass::SparseSliceRow)
        .expect("sparse row");
    assert_eq!(sparse_row.source_class, SourceClass::LocalOnly);
    assert_eq!(sparse_row.readiness_state, ReadinessState::Partial);
    assert!(sparse_row
        .offered_actions
        .contains(&SwitcherAction::BuildMissingIndexes));

    let policy_row = switcher
        .rows
        .iter()
        .find(|row| row.switcher_row_class == SwitcherRowClass::PolicyLimitedOverlayRow)
        .expect("policy row");
    assert!(policy_row.policy_overlay.is_some());
    assert!(!policy_row
        .offered_actions
        .contains(&SwitcherAction::ExportWorksetArtifact));
}

#[test]
fn scope_banner_projection_discloses_partial_and_widened_states() {
    let selected =
        workset_artifact("fixtures/workspace/workset_examples/selected_workset_multi_root.json");
    let banner = project_scope_banner(
        "scope_banner:projected:hot_path",
        "wksp:payments:monorepo",
        &selected,
        Some(TrustPolicyNote {
            trust_class: TrustClass::FullyTrusted,
            label: "Workspace-shared workset.".to_string(),
        }),
        None,
        "mono:test",
    );
    banner.validate().expect("partial banner validates");
    assert_eq!(banner.banner_state, BannerState::ActivePartial);
    assert!(banner
        .offered_actions
        .contains(&BannerAction::WidenWithReview));
    assert!(banner
        .hidden_result_summary
        .as_ref()
        .is_some_and(|summary| summary.count == Some(4)));

    let widened = project_scope_banner(
        "scope_banner:projected:widened",
        "wksp:payments:monorepo",
        &selected,
        None,
        Some("scope_widen_diff:payments:hot_path_to_full_workspace:01".to_string()),
        "mono:test",
    );
    widened.validate().expect("widened banner validates");
    assert_eq!(widened.banner_state, BannerState::ActiveWidened);
    assert!(widened
        .offered_actions
        .contains(&BannerAction::OpenScopeDiff));
}

#[test]
fn scope_diff_review_fixtures_block_or_gate_widening_explicitly() {
    let review: ScopeDiffReviewRecord = read_yaml_fixture(
        "fixtures/workspace/scope_widening_cases/widen_current_repo_to_selected_workset.yaml",
    );
    review.validate().expect("widen review validates");
    assert!(review.offered_actions.contains(&ReviewAction::ConfirmWiden));
    assert!(review
        .offered_actions
        .contains(&ReviewAction::CancelWidenKeepCurrentScope));
    assert!(review.has_remember_choice_option());
    assert!(!review.remote_fetch_required);

    let remote_review: ScopeDiffReviewRecord = read_yaml_fixture(
        "fixtures/workspace/scope_widening_cases/widen_selected_workset_to_full_workspace.yaml",
    );
    remote_review
        .validate()
        .expect("remote widen review validates");
    assert!(remote_review.remote_fetch_required);
    assert!(
        remote_review
            .support_availability_impact
            .includes_managed_provider_refs_after_widen
    );

    let blocked: ScopeDiffReviewRecord = read_yaml_fixture(
        "fixtures/workspace/scope_widening_cases/blocked_widening_by_trust_or_policy.yaml",
    );
    blocked.validate().expect("blocked review validates");
    assert!(!blocked
        .offered_actions
        .contains(&ReviewAction::ConfirmWiden));
    assert!(blocked
        .offered_actions
        .contains(&ReviewAction::RequestTrustReview));
    assert!(blocked
        .offered_actions
        .contains(&ReviewAction::RequestPolicyReviewAdminOnly));
}

#[test]
fn scope_widen_diff_and_non_ui_bindings_preserve_reopen_identity() {
    let diff: ScopeWidenDiffRecord = read_json_fixture(
        "fixtures/workspace/workset_examples/scope_widen_diff_selected_to_full.json",
    );
    diff.validate().expect("scope widen diff validates");
    assert!(diff.widens_scope);
    assert_ne!(diff.base_workset_ref, diff.candidate_workset_ref);

    let artifact =
        workset_artifact("fixtures/workspace/workset_scope_alpha/aureline.workset.jsonc");
    let local = artifact.project_consumer_binding(
        WorksetScopeConsumerClass::LocalUi,
        aureline_workspace::ScopeReopenPosture::Exact,
        "mono:local",
    );
    let headless = artifact.project_consumer_binding(
        WorksetScopeConsumerClass::Headless,
        aureline_workspace::ScopeReopenPosture::Degraded(
            aureline_workspace::ScopeDegradedReason::RebindingRequired,
        ),
        "mono:headless",
    );
    assert_eq!(local.stable_scope_id, artifact.stable_scope_id());
    assert_eq!(headless.stable_scope_id, artifact.stable_scope_id());
    assert_eq!(
        headless.reopen_state,
        aureline_workspace::ScopeReopenState::Degraded
    );
    assert!(headless.degraded_reason.is_some());
}
