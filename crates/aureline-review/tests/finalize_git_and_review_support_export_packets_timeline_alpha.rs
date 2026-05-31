//! Fixture-driven coverage for finalized Git/review support-export timeline
//! packets, chronology truth, and operator playbooks.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/finalize-git-and-review-support-export-packets-timeline/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Timeline events strictly increase by sequence index (chronology truth).
//! 3. Every event is attributed and carries a clock source from the closed
//!    vocabulary; hosted/provider events disclose hosted authority.
//! 4. Operator playbook steps that mutate are previewable and reversible or
//!    checkpoint-backed, and authority-broadening steps are never actionable.
//! 5. Support/export records keep every `raw_*_export_allowed` flag false and
//!    cite the source schema.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_git_review_timeline_packet, DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket,
    GitReviewSupportExportTimelinePacket, GitReviewTimelineInput, ReviewWorkspaceBetaInput,
    ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TimelineFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    timeline_input: GitReviewTimelineInput,
    expected: ExpectedTimeline,
}

#[derive(Debug, Deserialize)]
struct ExpectedTimeline {
    chronology_current: bool,
    chronology_stale: bool,
    chronology_reconstructed: bool,
    chronology_gap_detected: bool,
    monotonic_ordering_preserved: bool,
    all_events_attributed: bool,
    all_events_have_clock_source: bool,
    all_hosted_events_disclosed: bool,
    lineage_resolves: bool,
    all_mutating_steps_reversible_or_checkpointed: bool,
    all_mutating_steps_previewable: bool,
    all_hosted_steps_disclosed: bool,
    no_authority_broadening: bool,
    actionable: bool,
    invalidated: bool,
    timeline_event_count: usize,
    playbook_count: usize,
    playbook_step_count: usize,
    support_export_reopenable: bool,
    raw_escape_hatches_absent: bool,
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
        .join("../../fixtures/review/m4/finalize-git-and-review-support-export-packets-timeline")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("timeline fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> TimelineFixture {
    let path = fixtures_dir().join(name);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"))
}

fn seed_packet_for(seed_fixture_ref: &str) -> ReviewWorkspaceSeedPacket {
    let path = repo_root().join(seed_fixture_ref);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("seed fixture {path:?}: {err}"));
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

fn workspace_packet_for(fixture: &TimelineFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn timeline_packet_for(fixture: &TimelineFixture) -> GitReviewSupportExportTimelinePacket {
    let workspace_packet = workspace_packet_for(fixture);
    GitReviewSupportExportTimelinePacket::from_workspace_packet(
        fixture.timeline_input.clone(),
        &workspace_packet,
    )
    .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(
    packet: &GitReviewSupportExportTimelinePacket,
    expected: &ExpectedTimeline,
    case_name: &str,
) {
    let i = &packet.inspection;
    assert_eq!(i.chronology_current, expected.chronology_current, "{case_name}: chronology_current");
    assert_eq!(i.chronology_stale, expected.chronology_stale, "{case_name}: chronology_stale");
    assert_eq!(
        i.chronology_reconstructed, expected.chronology_reconstructed,
        "{case_name}: chronology_reconstructed"
    );
    assert_eq!(
        i.chronology_gap_detected, expected.chronology_gap_detected,
        "{case_name}: chronology_gap_detected"
    );
    assert_eq!(
        i.monotonic_ordering_preserved, expected.monotonic_ordering_preserved,
        "{case_name}: monotonic_ordering_preserved"
    );
    assert_eq!(
        i.all_events_attributed, expected.all_events_attributed,
        "{case_name}: all_events_attributed"
    );
    assert_eq!(
        i.all_events_have_clock_source, expected.all_events_have_clock_source,
        "{case_name}: all_events_have_clock_source"
    );
    assert_eq!(
        i.all_hosted_events_disclosed, expected.all_hosted_events_disclosed,
        "{case_name}: all_hosted_events_disclosed"
    );
    assert_eq!(i.lineage_resolves, expected.lineage_resolves, "{case_name}: lineage_resolves");
    assert_eq!(
        i.all_mutating_steps_reversible_or_checkpointed,
        expected.all_mutating_steps_reversible_or_checkpointed,
        "{case_name}: all_mutating_steps_reversible_or_checkpointed"
    );
    assert_eq!(
        i.all_mutating_steps_previewable, expected.all_mutating_steps_previewable,
        "{case_name}: all_mutating_steps_previewable"
    );
    assert_eq!(
        i.all_hosted_steps_disclosed, expected.all_hosted_steps_disclosed,
        "{case_name}: all_hosted_steps_disclosed"
    );
    assert_eq!(
        i.no_authority_broadening, expected.no_authority_broadening,
        "{case_name}: no_authority_broadening"
    );
    assert_eq!(i.actionable, expected.actionable, "{case_name}: actionable");
    assert_eq!(i.invalidated, expected.invalidated, "{case_name}: invalidated");
    assert_eq!(
        i.timeline_event_count, expected.timeline_event_count,
        "{case_name}: timeline_event_count"
    );
    assert_eq!(i.playbook_count, expected.playbook_count, "{case_name}: playbook_count");
    assert_eq!(
        i.playbook_step_count, expected.playbook_step_count,
        "{case_name}: playbook_step_count"
    );
    assert_eq!(
        i.support_export_reopenable, expected.support_export_reopenable,
        "{case_name}: support_export_reopenable"
    );
    assert_eq!(
        i.raw_escape_hatches_absent, expected.raw_escape_hatches_absent,
        "{case_name}: raw_escape_hatches_absent"
    );
}

#[test]
fn timeline_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "timeline fixtures must exist");

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: TimelineFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(fixture.record_kind, "git_review_timeline_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = timeline_packet_for(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);
        assert!(packet.monotonic_ordering_preserved(), "{}", fixture.case_name);
        assert!(packet.all_events_attributed(), "{}", fixture.case_name);
        assert!(packet.hosted_events_disclosed(), "{}", fixture.case_name);

        assert_expected(&packet, &fixture.expected, &fixture.case_name);

        // Round-trip through JSON and re-validate.
        let json = serde_json::to_string(&packet).expect("serialization must succeed");
        let reparsed: GitReviewSupportExportTimelinePacket =
            serde_json::from_str(&json).expect("re-deserialization must succeed");
        reparsed
            .validate()
            .unwrap_or_else(|err| panic!("{} round-trip must validate: {err}", fixture.case_name));
        assert_expected(&reparsed, &fixture.expected, &fixture.case_name);
    }
}

#[test]
fn chronology_current_with_playbook_is_actionable() {
    let fixture = load_fixture("chronology_current_with_playbook.json");
    let packet = timeline_packet_for(&fixture);

    assert!(packet.inspection.chronology_current, "must be chronology_current");
    assert!(packet.inspection.actionable, "must be actionable");
    assert!(packet.no_authority_broadening(), "must not broaden authority");
    assert!(
        packet.inspection.all_mutating_steps_reversible_or_checkpointed,
        "mutating steps must be reversible or checkpointed"
    );
}

#[test]
fn reconstructed_timeline_resolves_lineage() {
    let fixture = load_fixture("reconstructed_timeline_from_lineage.json");
    let packet = timeline_packet_for(&fixture);

    assert!(packet.inspection.chronology_reconstructed, "must be reconstructed");
    assert!(packet.inspection.lineage_resolves, "lineage must resolve");
    assert!(packet.monotonic_ordering_preserved(), "ordering must be monotonic");
}

#[test]
fn authority_broadening_step_is_blocked() {
    let fixture = load_fixture("blocked_authority_broadening_step.json");
    let packet = timeline_packet_for(&fixture);

    assert!(
        !packet.no_authority_broadening(),
        "fixture must flag an authority-broadening step"
    );
    assert!(!packet.timeline_truth.actionable, "timeline must be blocked");
    assert!(
        packet.playbook_steps.iter().any(|s| s.would_broaden_authority && !s.actionable),
        "authority-broadening step must be non-actionable"
    );
}

#[test]
fn projection_from_json_payload() {
    let fixture = load_fixture("chronology_current_with_playbook.json");
    let packet = timeline_packet_for(&fixture);
    let json = serde_json::to_string(&packet).expect("serialization must succeed");

    let projection = project_git_review_timeline_packet(&json).expect("projection must succeed");
    assert_eq!(projection.timeline_id, packet.timeline_truth.timeline_id);
    assert_eq!(
        projection.review_workspace_id,
        packet.review_workspace.review_workspace_id
    );
    assert_eq!(projection.chronology_state, packet.timeline_truth.chronology_state);
}
