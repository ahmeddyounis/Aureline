//! Fixture-driven coverage for landing-candidate and merge-queue packets.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_landing_candidate_packet, DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket,
    LandingCandidateInput, LandingCandidatePacket, ReviewWorkspaceBetaInput,
    ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LandingCandidateFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    landing_input: LandingCandidateInput,
    expected: ExpectedLandingCandidate,
}

#[derive(Debug, Deserialize)]
struct ExpectedLandingCandidate {
    mergeable: bool,
    queue_eligible: bool,
    queued: bool,
    stale_base_blocks_landing: bool,
    checks_stale_blocks_landing: bool,
    policy_blocks_landing: bool,
    approval_invalidated: bool,
    provider_authoritative: bool,
    queue_state_is_local_estimate_only: bool,
    candidate_invalidated: bool,
    command_count: usize,
    preview_capable: bool,
    support_export_reopenable: bool,
    eligibility_state: String,
    queue_state: String,
    invalidation_reasons: Vec<String>,
    blocked_reasons: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceSeedFixture {
    change_list_row: ChangeListRowFixture,
    workspace_seed: ReviewWorkspaceSeedInput,
    diff: DiffFileInput,
}

#[derive(Debug, Deserialize)]
struct ChangeListRowFixture {
    row_ref: String,
    file_state_token: String,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/m3/merge_queue_and_landing")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("landing candidate fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> LandingCandidateFixture {
    let path = fixtures_dir().join(name);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"))
}

fn seed_packet_for(seed_fixture_ref: &str) -> ReviewWorkspaceSeedPacket {
    let path = repo_root().join(seed_fixture_ref);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("seed fixture {path:?}: {err}"));
    let fixture: ReviewWorkspaceSeedFixture =
        serde_yaml::from_str(&text).unwrap_or_else(|err| panic!("seed fixture {path:?}: {err}"));
    let open_target = DiffOpenTarget::from_change_list_row_parts(
        &fixture.diff.workspace_ref,
        &fixture.diff.truth_source_ref,
        &fixture.change_list_row.row_ref,
        &fixture.diff.group_token,
        fixture.diff.path.clone(),
        fixture.diff.original_path.clone(),
        &fixture.diff.status_code,
        &fixture.change_list_row.file_state_token,
    );
    let diff_packet = DiffViewSurfacePacket::from_file_input(open_target, fixture.diff);
    ReviewWorkspaceSeedPacket::from_diff_packet(fixture.workspace_seed, &diff_packet)
}

fn workspace_packet_for(fixture: &LandingCandidateFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn packet_for_fixture(fixture: &LandingCandidateFixture) -> LandingCandidatePacket {
    let workspace_packet = workspace_packet_for(fixture);
    LandingCandidatePacket::from_workspace_packet(fixture.landing_input.clone(), &workspace_packet)
        .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(packet: &LandingCandidatePacket, expected: &ExpectedLandingCandidate) {
    assert_eq!(packet.inspection.mergeable, expected.mergeable);
    assert_eq!(packet.inspection.queue_eligible, expected.queue_eligible);
    assert_eq!(packet.inspection.queued, expected.queued);
    assert_eq!(
        packet.inspection.stale_base_blocks_landing,
        expected.stale_base_blocks_landing
    );
    assert_eq!(
        packet.inspection.checks_stale_blocks_landing,
        expected.checks_stale_blocks_landing
    );
    assert_eq!(
        packet.inspection.policy_blocks_landing,
        expected.policy_blocks_landing
    );
    assert_eq!(
        packet.inspection.approval_invalidated,
        expected.approval_invalidated
    );
    assert_eq!(
        packet.inspection.provider_authoritative,
        expected.provider_authoritative
    );
    assert_eq!(
        packet.inspection.queue_state_is_local_estimate_only,
        expected.queue_state_is_local_estimate_only
    );
    assert_eq!(
        packet.inspection.candidate_invalidated,
        expected.candidate_invalidated
    );
    assert_eq!(packet.inspection.command_count, expected.command_count);
    assert_eq!(packet.inspection.preview_capable, expected.preview_capable);
    assert_eq!(
        packet.inspection.support_export_reopenable,
        expected.support_export_reopenable
    );
    assert_eq!(
        packet.landing_candidate.eligibility_state,
        expected.eligibility_state
    );
    let queue_state = packet
        .merge_queue_entry
        .as_ref()
        .map(|entry| entry.queue_state.clone())
        .unwrap_or_else(|| "not_queued".to_string());
    assert_eq!(queue_state, expected.queue_state);
    assert_eq!(
        packet.landing_candidate.invalidation_reasons,
        expected.invalidation_reasons
    );
    assert_eq!(
        packet.landing_candidate.blocked_reasons,
        expected.blocked_reasons
    );
}

#[test]
fn landing_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "landing fixtures must exist");

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: LandingCandidateFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(fixture.record_kind, "review_landing_candidate_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = packet_for_fixture(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.truths_are_separable(), "{}", fixture.case_name);
        assert!(
            packet.landing_requires_explicit_candidate(),
            "{}",
            fixture.case_name
        );
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);
        assert_expected(&packet, &fixture.expected);

        let serialized = serde_json::to_string_pretty(&packet).expect("packet serializes");
        let projection = project_landing_candidate_packet(&serialized)
            .unwrap_or_else(|err| panic!("{} must re-project: {err}", fixture.case_name));
        assert_eq!(projection.packet_id, packet.packet_id);
        assert_eq!(
            projection.eligibility_state,
            fixture.expected.eligibility_state
        );
        assert_eq!(projection.queue_state, fixture.expected.queue_state);
        assert_eq!(projection.command_count, fixture.expected.command_count);
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "support_export"));
    }
}

#[test]
fn ambient_branch_state_cannot_land_without_candidate() {
    let fixture = load_fixture("provider_authoritative_mergeable_queued.json");
    let packet = packet_for_fixture(&fixture);
    assert!(packet.landing_requires_explicit_candidate());
    assert!(packet.landing_candidate.landing_requires_explicit_candidate);
}

#[test]
fn support_export_eligibility_snapshot_mirrors_candidate() {
    let fixture = load_fixture("policy_blocked_with_invalidated_approval.json");
    let packet = packet_for_fixture(&fixture);
    let snapshot = &packet.support_export.eligibility_snapshot;
    assert_eq!(
        snapshot.mergeable_state,
        packet.landing_candidate.mergeable_state
    );
    assert_eq!(
        snapshot.eligibility_state,
        packet.landing_candidate.eligibility_state
    );
    assert_eq!(
        snapshot.approval_state,
        packet.landing_candidate.approval_state
    );
    assert_eq!(
        snapshot.policy_block_state,
        packet.landing_candidate.policy_block_state
    );
    assert!(snapshot
        .invalidation_reasons
        .contains(&"approval_invalidated".to_string()));
    assert!(snapshot
        .invalidation_reasons
        .contains(&"policy_blocked".to_string()));
    assert_eq!(snapshot.queue_state, "dequeued_by_provider".to_string());
    assert_eq!(
        snapshot.queue_authority_class.as_deref(),
        Some("repo_policy_managed_queue_state")
    );
}

#[test]
fn local_estimate_omits_merge_queue_entry() {
    let fixture = load_fixture("local_estimate_no_provider_overlay.json");
    let packet = packet_for_fixture(&fixture);
    assert!(packet.merge_queue_entry.is_none());
    assert_eq!(packet.landing_candidate.eligibility_state, "queue_eligible");
    assert!(!packet.inspection.queued);
    assert!(!packet.inspection.provider_authoritative);
}

#[test]
fn provider_eligible_candidate_without_queue_entry_is_rejected() {
    let fixture = load_fixture("provider_authoritative_mergeable_queued.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.landing_input;
    input.merge_queue_entry = None;
    let err = LandingCandidatePacket::from_workspace_packet(input, &workspace_packet)
        .expect_err("provider-authoritative queue_eligible without entry must fail");
    assert!(err.message().contains("merge_queue_entry"));
}
