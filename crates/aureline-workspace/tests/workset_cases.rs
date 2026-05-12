//! Fixture-driven coverage for the multi-root workspace + workset artifact
//! seed. Each case asserts both nominal multi-root behavior and at least one
//! degraded path (outside-scope cross-repo result, partial-index sparse slice,
//! admin-policy narrowing).

use std::path::Path;

use serde::Deserialize;

use aureline_workspace::{
    ChipAction, ChipPresentationState, ChipSurfaceClass, MembershipDecision, MultiRootWorkspace,
    NarrowingCause, PartialTruthLabel, ScopeClass, WorksetArtifactRecord,
};

#[derive(Debug, Clone, Deserialize)]
struct WorksetCaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    workspace: MultiRootWorkspace,
    artifact: WorksetArtifactRecord,
    expect: ExpectBlock,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectBlock {
    workspace_root_count: u32,
    workspace_is_multi_root: bool,
    artifact_root_count: u32,
    artifact_is_multi_root: bool,
    artifact_is_full_workspace: bool,
    artifact_is_narrowed_scope: bool,
    membership_probes: Vec<MembershipProbe>,
    chip: ChipExpect,
    #[serde(default)]
    outside_scope_chip: Option<OutsideScopeChipExpect>,
}

#[derive(Debug, Clone, Deserialize)]
struct MembershipProbe {
    root_ref: String,
    decision: String,
    #[serde(default)]
    expected_partial_truth: Option<String>,
    #[serde(default)]
    expected_cause: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChipExpect {
    presentation_state: String,
    label_starts_with: String,
    root_count: u32,
    outside_current_scope_marker_visible: bool,
    must_offer_actions: Vec<String>,
    #[serde(default)]
    must_not_offer_actions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct OutsideScopeChipExpect {
    expect_outside_marker: bool,
    expected_label: String,
    must_offer_actions: Vec<String>,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/workset_cases")
}

fn load_fixtures() -> Vec<(std::path::PathBuf, WorksetCaseFixture)> {
    let dir = fixtures_dir();
    let mut fixtures: Vec<_> = std::fs::read_dir(&dir)
        .expect("fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    fixtures
        .into_iter()
        .map(|path| {
            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let parsed: WorksetCaseFixture = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("failed to parse fixture {path:?}: {err}"));
            (path, parsed)
        })
        .collect()
}

fn presentation_state_token(state: ChipPresentationState) -> &'static str {
    match state {
        ChipPresentationState::ActiveNarrowSafe => "active_narrow_safe",
        ChipPresentationState::ActivePartial => "active_partial",
        ChipPresentationState::ActivePolicyLimited => "active_policy_limited",
        ChipPresentationState::ActiveWidened => "active_widened",
        ChipPresentationState::OutsideCurrentScope => "outside_current_scope",
    }
}

fn chip_action_token(action: ChipAction) -> &'static str {
    match action {
        ChipAction::WidenToFullWorkspace => "widen_to_full_workspace",
        ChipAction::WidenWithReview => "widen_with_review",
        ChipAction::NarrowToCurrentRepo => "narrow_to_current_repo",
        ChipAction::OpenScopeDiff => "open_scope_diff",
        ChipAction::BuildMissingIndexes => "build_missing_indexes",
        ChipAction::KeepCurrentScope => "keep_current_scope",
        ChipAction::RevealHiddenResultsPolicyAdminOnly => "reveal_hidden_results_policy_admin_only",
        ChipAction::OpenInNewPane => "open_in_new_pane",
        ChipAction::CopyWorksetId => "copy_workset_id",
        ChipAction::ExportWorksetArtifact => "export_workset_artifact",
    }
}

fn partial_truth_token(label: PartialTruthLabel) -> &'static str {
    label.as_str()
}

fn narrowing_cause_token(cause: NarrowingCause) -> &'static str {
    match cause {
        NarrowingCause::AdminPolicy => "admin_policy",
        NarrowingCause::TrustPolicy => "trust_policy",
        NarrowingCause::LicenseOrExportControl => "license_or_export_control",
        NarrowingCause::RemoteUnavailable => "remote_unavailable",
        NarrowingCause::IndexNotBuilt => "index_not_built",
        NarrowingCause::UserMuted => "user_muted",
    }
}

#[test]
fn fixtures_round_trip_through_serde() {
    for (path, fixture) in load_fixtures() {
        assert_eq!(
            fixture.record_kind, "workset_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        let payload = serde_json::to_string(&fixture.artifact).expect("artifact must serialize");
        let parsed: WorksetArtifactRecord =
            serde_json::from_str(&payload).expect("artifact must round-trip");
        assert_eq!(parsed, fixture.artifact, "artifact mismatch in {path:?}");

        let payload = serde_json::to_string(&fixture.workspace).expect("workspace must serialize");
        let parsed: MultiRootWorkspace =
            serde_json::from_str(&payload).expect("workspace must round-trip");
        assert_eq!(parsed, fixture.workspace, "workspace mismatch in {path:?}");
    }
}

#[test]
fn fixtures_validate_against_seed_invariants() {
    for (path, fixture) in load_fixtures() {
        fixture
            .artifact
            .validate()
            .unwrap_or_else(|err| panic!("artifact in {path:?} must validate: {err}"));
        for root_ref in &fixture.artifact.root_refs {
            assert!(
                fixture.workspace.contains_root(root_ref),
                "artifact references root {root_ref} not in workspace at {path:?}"
            );
        }
    }
}

#[test]
fn workspace_and_artifact_membership_match_expectations() {
    for (path, fixture) in load_fixtures() {
        assert_eq!(
            fixture.workspace.root_count() as u32,
            fixture.expect.workspace_root_count,
            "workspace_root_count mismatch in {path:?}"
        );
        assert_eq!(
            fixture.workspace.is_multi_root(),
            fixture.expect.workspace_is_multi_root,
            "workspace_is_multi_root mismatch in {path:?}"
        );
        assert_eq!(
            fixture.artifact.root_count() as u32,
            fixture.expect.artifact_root_count,
            "artifact_root_count mismatch in {path:?}"
        );
        assert_eq!(
            fixture.artifact.is_multi_root(),
            fixture.expect.artifact_is_multi_root,
            "artifact_is_multi_root mismatch in {path:?}"
        );
        assert_eq!(
            fixture.artifact.is_full_workspace(),
            fixture.expect.artifact_is_full_workspace,
            "artifact_is_full_workspace mismatch in {path:?}"
        );
        assert_eq!(
            fixture.artifact.is_narrowed_scope(),
            fixture.expect.artifact_is_narrowed_scope,
            "artifact_is_narrowed_scope mismatch in {path:?}"
        );

        for probe in &fixture.expect.membership_probes {
            let decision = fixture.artifact.root_membership_decision(&probe.root_ref);
            match (probe.decision.as_str(), decision) {
                ("in_scope", MembershipDecision::InScope { partial_truth }) => {
                    if let Some(expected) = probe.expected_partial_truth.as_deref() {
                        assert_eq!(
                            partial_truth_token(partial_truth),
                            expected,
                            "partial_truth mismatch for {} in {path:?}",
                            probe.root_ref
                        );
                    }
                }
                ("policy_hidden", MembershipDecision::PolicyHidden { cause }) => {
                    if let Some(expected) = probe.expected_cause.as_deref() {
                        assert_eq!(
                            narrowing_cause_token(cause),
                            expected,
                            "policy cause mismatch for {} in {path:?}",
                            probe.root_ref
                        );
                    }
                }
                ("outside_current_scope", MembershipDecision::OutsideCurrentScope) => {}
                (expected, actual) => panic!(
                    "membership decision mismatch for {} in {path:?}: expected={expected} actual={actual:?}",
                    probe.root_ref
                ),
            }
        }
    }
}

#[test]
fn projected_chips_match_expectations() {
    for (path, fixture) in load_fixtures() {
        let chip = fixture.artifact.project_chip(
            format!("chip:{}", fixture.case_name),
            ChipSurfaceClass::WorksetSwitcher,
            "mono:test",
        );
        assert_eq!(
            presentation_state_token(chip.chip_presentation_state),
            fixture.expect.chip.presentation_state,
            "chip presentation_state mismatch in {path:?}"
        );
        assert!(
            chip.chip_label
                .starts_with(&fixture.expect.chip.label_starts_with),
            "chip label '{}' does not start with '{}' in {path:?}",
            chip.chip_label,
            fixture.expect.chip.label_starts_with
        );
        assert_eq!(
            chip.root_count,
            Some(fixture.expect.chip.root_count),
            "chip root_count mismatch in {path:?}"
        );
        assert_eq!(
            chip.outside_current_scope_marker_visible,
            fixture.expect.chip.outside_current_scope_marker_visible,
            "outside_current_scope_marker_visible mismatch in {path:?}"
        );
        let offered: Vec<&str> = chip
            .offered_actions
            .iter()
            .copied()
            .map(chip_action_token)
            .collect();
        for must in &fixture.expect.chip.must_offer_actions {
            assert!(
                offered.iter().any(|a| a == must),
                "chip must offer action {must} but offered {offered:?} in {path:?}"
            );
        }
        for forbidden in &fixture.expect.chip.must_not_offer_actions {
            assert!(
                !offered.iter().any(|a| a == forbidden),
                "chip must NOT offer action {forbidden} but offered {offered:?} in {path:?}"
            );
        }

        if let Some(outside) = fixture.expect.outside_scope_chip.as_ref() {
            let outside_chip = fixture.artifact.project_outside_scope_chip(
                format!("chip:outside:{}", fixture.case_name),
                ChipSurfaceClass::SearchResultRowMarker,
                "mono:test",
            );
            assert_eq!(
                outside_chip.outside_current_scope_marker_visible, outside.expect_outside_marker,
                "outside-scope marker mismatch in {path:?}"
            );
            assert_eq!(
                outside_chip.chip_label, outside.expected_label,
                "outside-scope label mismatch in {path:?}"
            );
            let offered: Vec<&str> = outside_chip
                .offered_actions
                .iter()
                .copied()
                .map(chip_action_token)
                .collect();
            for must in &outside.must_offer_actions {
                assert!(
                    offered.iter().any(|a| a == must),
                    "outside-scope chip must offer action {must} but offered {offered:?} in {path:?}"
                );
            }
            assert_eq!(
                presentation_state_token(outside_chip.chip_presentation_state),
                "outside_current_scope",
                "outside-scope chip presentation_state mismatch in {path:?}"
            );
        }
    }
}

#[test]
fn at_least_one_fixture_exercises_each_seed_class() {
    let fixtures = load_fixtures();
    assert!(
        !fixtures.is_empty(),
        "workset_cases must seed at least one fixture"
    );
    let scope_classes: std::collections::BTreeSet<&'static str> = fixtures
        .iter()
        .map(|(_, f)| match f.artifact.scope_class {
            ScopeClass::CurrentRepo => "current_repo",
            ScopeClass::SelectedWorkset => "selected_workset",
            ScopeClass::SparseSlice => "sparse_slice",
            ScopeClass::FullWorkspace => "full_workspace",
            ScopeClass::PolicyLimitedView => "policy_limited_view",
        })
        .collect();
    for required in [
        "full_workspace",
        "selected_workset",
        "sparse_slice",
        "policy_limited_view",
    ] {
        assert!(
            scope_classes.contains(required),
            "no fixture covers scope_class {required}; got {scope_classes:?}"
        );
    }
}
