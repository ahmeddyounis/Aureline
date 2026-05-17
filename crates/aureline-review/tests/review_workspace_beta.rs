//! Fixture-driven coverage for beta review-workspace packets.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_review_workspace_beta_packet, DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket,
    ReviewWorkspaceBetaInput, ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput,
    ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceBetaFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_input: ReviewWorkspaceBetaInput,
    expected: ExpectedReviewWorkspaceBeta,
}

#[derive(Debug, Deserialize)]
struct ExpectedReviewWorkspaceBeta {
    durable_comment_anchor_count: usize,
    check_freshness_count: usize,
    object_lineage_count: usize,
    anchor_identity_preserved: bool,
    check_freshness_browser_independent: bool,
    typed_reversible_browser_handoff_present: bool,
    support_export_reopenable: bool,
    raw_escape_hatches_absent: bool,
    operator_truth_current: bool,
    stale_check_blocks_operator_truth: bool,
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

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceBetaProjectionArtifact {
    record_kind: String,
    schema_version: u32,
    projection_rows: Vec<ReviewWorkspaceBetaProjectionArtifactRow>,
}

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceBetaProjectionArtifactRow {
    packet_id: String,
    review_workspace_id: String,
    durable_comment_anchor_count: usize,
    check_freshness_count: usize,
    object_lineage_count: usize,
    typed_reversible_browser_handoff_present: bool,
    support_export_reopenable: bool,
    operator_truth_current: bool,
    stale_check_blocks_operator_truth: bool,
    support_export_ref: String,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/m3/review_workspace_beta")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("review workspace beta fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> ReviewWorkspaceBetaFixture {
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

fn artifact_path() -> PathBuf {
    repo_root().join("artifacts/review/m3/review_workspace_beta_projection.json")
}

fn packet_for_fixture(fixture: &ReviewWorkspaceBetaFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(packet: &ReviewWorkspaceBetaPacket, expected: &ExpectedReviewWorkspaceBeta) {
    assert_eq!(
        packet.inspection.durable_comment_anchor_count,
        expected.durable_comment_anchor_count
    );
    assert_eq!(
        packet.inspection.check_freshness_count,
        expected.check_freshness_count
    );
    assert_eq!(
        packet.inspection.object_lineage_count,
        expected.object_lineage_count
    );
    assert_eq!(
        packet.inspection.anchor_identity_preserved,
        expected.anchor_identity_preserved
    );
    assert_eq!(
        packet.inspection.check_freshness_browser_independent,
        expected.check_freshness_browser_independent
    );
    assert_eq!(
        packet.inspection.typed_reversible_browser_handoff_present,
        expected.typed_reversible_browser_handoff_present
    );
    assert_eq!(
        packet.inspection.support_export_reopenable,
        expected.support_export_reopenable
    );
    assert_eq!(
        packet.inspection.raw_escape_hatches_absent,
        expected.raw_escape_hatches_absent
    );
    assert_eq!(
        packet.inspection.operator_truth_current,
        expected.operator_truth_current
    );
    assert_eq!(
        packet.inspection.stale_check_blocks_operator_truth,
        expected.stale_check_blocks_operator_truth
    );
}

#[test]
fn beta_fixtures_project_from_workspace_seed() {
    let paths = load_fixture_paths();
    assert!(
        !paths.is_empty(),
        "review workspace beta fixtures must exist"
    );

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: ReviewWorkspaceBetaFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(fixture.record_kind, "review_workspace_beta_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = packet_for_fixture(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert_expected(&packet, &fixture.expected);
        assert!(packet.preserves_anchor_identity());
        assert!(packet.check_freshness_is_browser_independent());
        assert!(packet.support_export_can_reopen_context());
        assert!(packet.raw_escape_hatches_absent());

        let materialized = serde_json::to_string_pretty(&packet).expect("packet serializes");
        let projection = project_review_workspace_beta_packet(&materialized)
            .unwrap_or_else(|err| panic!("{} must re-project: {err}", fixture.case_name));
        assert_eq!(projection.packet_id, fixture.beta_input.packet_id);
        assert_eq!(
            projection.durable_comment_anchor_count,
            fixture.expected.durable_comment_anchor_count
        );
        assert_eq!(
            projection.typed_reversible_browser_handoff_present,
            fixture.expected.typed_reversible_browser_handoff_present
        );
        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "support_export"),
            "{} must keep support_export wired",
            fixture.case_name
        );
    }
}

#[test]
fn browser_handoff_rejects_raw_url_escape_hatch() {
    let fixture = load_fixture("local_workspace_with_reversible_browser_handoff.json");
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    let mut input = fixture.beta_input;
    input
        .browser_handoff
        .as_mut()
        .expect("fixture has browser handoff")
        .destination_ref = "https://example.invalid/review/1842".to_string();

    let err = ReviewWorkspaceBetaPacket::from_seed_packet(input, &seed_packet)
        .expect_err("raw URL handoff must be rejected");
    assert!(err.message().contains("raw URL"));
}

#[test]
fn stale_check_must_block_operator_truth_claims() {
    let fixture = load_fixture("stale_check_blocks_operator_truth.json");
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    let mut input = fixture.beta_input;
    input.check_freshness[0].blocks_operator_truth_claim_when_stale = false;

    let err = ReviewWorkspaceBetaPacket::from_seed_packet(input, &seed_packet)
        .expect_err("stale check without operator-truth block must fail");
    assert!(err.message().contains("operator-truth"));
}

#[test]
fn projection_artifact_matches_fixture_packets() {
    let text = std::fs::read_to_string(artifact_path()).expect("projection artifact reads");
    let artifact: ReviewWorkspaceBetaProjectionArtifact =
        serde_json::from_str(&text).expect("projection artifact parses");
    assert_eq!(
        artifact.record_kind,
        "review_workspace_beta_projection_artifact"
    );
    assert_eq!(artifact.schema_version, 1);

    let mut expected_rows = artifact.projection_rows;
    expected_rows.sort_by(|a, b| a.packet_id.cmp(&b.packet_id));

    let mut actual_packets = load_fixture_paths()
        .into_iter()
        .map(|path| {
            let text = std::fs::read_to_string(&path).expect("fixture reads");
            let fixture: ReviewWorkspaceBetaFixture =
                serde_json::from_str(&text).expect("fixture parses");
            packet_for_fixture(&fixture)
        })
        .collect::<Vec<_>>();
    actual_packets.sort_by(|a, b| a.packet_id.cmp(&b.packet_id));

    assert_eq!(expected_rows.len(), actual_packets.len());
    for (expected, packet) in expected_rows.iter().zip(actual_packets.iter()) {
        assert_eq!(expected.packet_id, packet.packet_id);
        assert_eq!(
            expected.review_workspace_id,
            packet.review_workspace.review_workspace_id
        );
        assert_eq!(
            expected.durable_comment_anchor_count,
            packet.inspection.durable_comment_anchor_count
        );
        assert_eq!(
            expected.check_freshness_count,
            packet.inspection.check_freshness_count
        );
        assert_eq!(
            expected.object_lineage_count,
            packet.inspection.object_lineage_count
        );
        assert_eq!(
            expected.typed_reversible_browser_handoff_present,
            packet.inspection.typed_reversible_browser_handoff_present
        );
        assert_eq!(
            expected.support_export_reopenable,
            packet.inspection.support_export_reopenable
        );
        assert_eq!(
            expected.operator_truth_current,
            packet.inspection.operator_truth_current
        );
        assert_eq!(
            expected.stale_check_blocks_operator_truth,
            packet.inspection.stale_check_blocks_operator_truth
        );
        assert_eq!(
            expected.support_export_ref,
            packet.support_export.support_export_id
        );
    }
}
