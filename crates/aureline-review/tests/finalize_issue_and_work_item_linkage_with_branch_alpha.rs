//! Fixture-driven coverage for work-item linkage finalization packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/finalize-issue-and-work-item-linkage-with-branch/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Work-item detail surfaces disclose their write mode on every surface.
//! 3. Status-transition sheets preserve local draft on failure and preview side effects.
//! 4. Offline-handoff continuities survive restart, reconnect, and export/import.
//! 5. Branch and review links are previewable before publish.
//! 6. Publish-later continuities cite queue items and survive restart/reconnect.
//! 7. Support/export records keep every `raw_*_export_allowed` flag false.

use std::path::{Path, PathBuf};

use aureline_review::{
    DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket, LandingCandidateInput,
    LandingCandidatePacket, ReviewStabilizationInput, ReviewStabilizationPacket,
    ReviewWorkspaceBetaInput, ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput,
    ReviewWorkspaceSeedPacket, WorkItemLinkageFinalizationInput, WorkItemLinkageFinalizationPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LinkageFinalizationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    landing_input: LandingCandidateInput,
    stabilization_input: ReviewStabilizationInput,
    linkage_finalization_input: WorkItemLinkageFinalizationInput,
    expected: ExpectedLinkageFinalization,
}

#[derive(Debug, Deserialize)]
struct ExpectedLinkageFinalization {
    finalized_current: bool,
    finalized_stale_provider_overlay: bool,
    finalized_partial_work_item_scope: bool,
    finalized_diverged_requires_review: bool,
    finalized_offline_handoff_only: bool,
    provider_authoritative_surface_present: bool,
    local_draft_surface_present: bool,
    sync_pending_surface_present: bool,
    offline_captured_surface_present: bool,
    write_mode_disclosed_on_all_surfaces: bool,
    transition_sheet_present: bool,
    local_draft_preserved_on_failure: bool,
    offline_handoff_survives_restart: bool,
    offline_handoff_survives_reconnect: bool,
    branch_link_previewable: bool,
    review_link_previewable: bool,
    publish_later_survives_restart: bool,
    publish_later_survives_reconnect: bool,
    actionable: bool,
    invalidated: bool,
    command_count: usize,
    detail_surface_count: usize,
    transition_sheet_count: usize,
    offline_handoff_continuity_count: usize,
    branch_link_count: usize,
    review_link_count: usize,
    publish_later_continuity_count: usize,
    preview_capable: bool,
    support_export_reopenable: bool,
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
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/review/m4/finalize-issue-and-work-item-linkage-with-branch")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("linkage finalization fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> LinkageFinalizationFixture {
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

fn workspace_packet_for(fixture: &LinkageFinalizationFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn landing_packet_for(fixture: &LinkageFinalizationFixture) -> LandingCandidatePacket {
    let workspace_packet = workspace_packet_for(fixture);
    LandingCandidatePacket::from_workspace_packet(fixture.landing_input.clone(), &workspace_packet)
        .unwrap_or_else(|err| panic!("{} landing packet must project: {err}", fixture.case_name))
}

fn stabilization_packet_for(fixture: &LinkageFinalizationFixture) -> ReviewStabilizationPacket {
    let workspace_packet = workspace_packet_for(fixture);
    let landing_packet = landing_packet_for(fixture);
    ReviewStabilizationPacket::from_workspace_and_landing_packets(
        fixture.stabilization_input.clone(),
        &workspace_packet,
        &landing_packet,
    )
    .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn linkage_finalization_packet_for(
    fixture: &LinkageFinalizationFixture,
) -> WorkItemLinkageFinalizationPacket {
    let workspace_packet = workspace_packet_for(fixture);
    let stabilization_packet = stabilization_packet_for(fixture);
    WorkItemLinkageFinalizationPacket::from_workspace_and_stabilization_packets(
        fixture.linkage_finalization_input.clone(),
        &workspace_packet,
        &stabilization_packet,
    )
    .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(
    packet: &WorkItemLinkageFinalizationPacket,
    expected: &ExpectedLinkageFinalization,
    case_name: &str,
) {
    assert_eq!(
        packet.inspection.finalized_current, expected.finalized_current,
        "{case_name}: finalized_current"
    );
    assert_eq!(
        packet.inspection.finalized_stale_provider_overlay,
        expected.finalized_stale_provider_overlay,
        "{case_name}: finalized_stale_provider_overlay"
    );
    assert_eq!(
        packet.inspection.finalized_partial_work_item_scope,
        expected.finalized_partial_work_item_scope,
        "{case_name}: finalized_partial_work_item_scope"
    );
    assert_eq!(
        packet.inspection.finalized_diverged_requires_review,
        expected.finalized_diverged_requires_review,
        "{case_name}: finalized_diverged_requires_review"
    );
    assert_eq!(
        packet.inspection.finalized_offline_handoff_only, expected.finalized_offline_handoff_only,
        "{case_name}: finalized_offline_handoff_only"
    );
    assert_eq!(
        packet.inspection.provider_authoritative_surface_present,
        expected.provider_authoritative_surface_present,
        "{case_name}: provider_authoritative_surface_present"
    );
    assert_eq!(
        packet.inspection.local_draft_surface_present, expected.local_draft_surface_present,
        "{case_name}: local_draft_surface_present"
    );
    assert_eq!(
        packet.inspection.sync_pending_surface_present, expected.sync_pending_surface_present,
        "{case_name}: sync_pending_surface_present"
    );
    assert_eq!(
        packet.inspection.offline_captured_surface_present,
        expected.offline_captured_surface_present,
        "{case_name}: offline_captured_surface_present"
    );
    assert_eq!(
        packet.inspection.write_mode_disclosed_on_all_surfaces,
        expected.write_mode_disclosed_on_all_surfaces,
        "{case_name}: write_mode_disclosed_on_all_surfaces"
    );
    assert_eq!(
        packet.inspection.transition_sheet_present, expected.transition_sheet_present,
        "{case_name}: transition_sheet_present"
    );
    assert_eq!(
        packet.inspection.local_draft_preserved_on_failure,
        expected.local_draft_preserved_on_failure,
        "{case_name}: local_draft_preserved_on_failure"
    );
    assert_eq!(
        packet.inspection.offline_handoff_survives_restart,
        expected.offline_handoff_survives_restart,
        "{case_name}: offline_handoff_survives_restart"
    );
    assert_eq!(
        packet.inspection.offline_handoff_survives_reconnect,
        expected.offline_handoff_survives_reconnect,
        "{case_name}: offline_handoff_survives_reconnect"
    );
    assert_eq!(
        packet.inspection.branch_link_previewable, expected.branch_link_previewable,
        "{case_name}: branch_link_previewable"
    );
    assert_eq!(
        packet.inspection.review_link_previewable, expected.review_link_previewable,
        "{case_name}: review_link_previewable"
    );
    assert_eq!(
        packet.inspection.publish_later_survives_restart, expected.publish_later_survives_restart,
        "{case_name}: publish_later_survives_restart"
    );
    assert_eq!(
        packet.inspection.publish_later_survives_reconnect,
        expected.publish_later_survives_reconnect,
        "{case_name}: publish_later_survives_reconnect"
    );
    assert_eq!(
        packet.inspection.actionable, expected.actionable,
        "{case_name}: actionable"
    );
    assert_eq!(
        packet.inspection.invalidated, expected.invalidated,
        "{case_name}: invalidated"
    );
    assert_eq!(
        packet.inspection.command_count, expected.command_count,
        "{case_name}: command_count"
    );
    assert_eq!(
        packet.inspection.detail_surface_count, expected.detail_surface_count,
        "{case_name}: detail_surface_count"
    );
    assert_eq!(
        packet.inspection.transition_sheet_count, expected.transition_sheet_count,
        "{case_name}: transition_sheet_count"
    );
    assert_eq!(
        packet.inspection.offline_handoff_continuity_count,
        expected.offline_handoff_continuity_count,
        "{case_name}: offline_handoff_continuity_count"
    );
    assert_eq!(
        packet.inspection.branch_link_count, expected.branch_link_count,
        "{case_name}: branch_link_count"
    );
    assert_eq!(
        packet.inspection.review_link_count, expected.review_link_count,
        "{case_name}: review_link_count"
    );
    assert_eq!(
        packet.inspection.publish_later_continuity_count, expected.publish_later_continuity_count,
        "{case_name}: publish_later_continuity_count"
    );
    assert_eq!(
        packet.inspection.preview_capable, expected.preview_capable,
        "{case_name}: preview_capable"
    );
    assert_eq!(
        packet.inspection.support_export_reopenable, expected.support_export_reopenable,
        "{case_name}: support_export_reopenable"
    );
}

#[test]
fn all_fixtures_parse_validate_and_project() {
    for path in load_fixture_paths() {
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        let fixture = load_fixture(&format!("{name}.json"));
        let packet = linkage_finalization_packet_for(&fixture);
        assert_expected(&packet, &fixture.expected, &fixture.case_name);
    }
}

#[test]
fn write_modes_disclosed_on_all_surfaces() {
    let fixture = load_fixture("finalized_current_all_surfaces_present.json");
    let packet = linkage_finalization_packet_for(&fixture);
    assert!(
        packet.write_modes_disclosed(),
        "write modes must be disclosed"
    );
}

#[test]
fn offline_handoff_continuity_preserved() {
    let fixture = load_fixture("finalized_offline_handoff_only.json");
    let packet = linkage_finalization_packet_for(&fixture);
    assert!(
        packet.offline_handoff_continuity_preserved(),
        "offline handoff must survive restart and reconnect"
    );
}

#[test]
fn mutations_previewable_before_publish() {
    let fixture = load_fixture("finalized_current_all_surfaces_present.json");
    let packet = linkage_finalization_packet_for(&fixture);
    assert!(
        packet.mutations_previewable_before_publish(),
        "mutations must be previewable before publish"
    );
}

#[test]
fn support_export_keeps_raw_flags_false() {
    let fixture = load_fixture("finalized_current_all_surfaces_present.json");
    let packet = linkage_finalization_packet_for(&fixture);
    assert!(
        !packet.support_export.raw_url_export_allowed,
        "raw_url_export_allowed must be false"
    );
    assert!(
        !packet.support_export.raw_provider_payload_export_allowed,
        "raw_provider_payload_export_allowed must be false"
    );
}
