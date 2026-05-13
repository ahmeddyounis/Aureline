//! Fixture-driven coverage for review-workspace seed packets.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_review::diff::{DiffFileInput, DiffOpenTarget};
use aureline_review::workspace::{
    ReviewProviderOverlayInput, ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceSeedFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    change_list_row: ChangeListRowFixture,
    workspace_seed: ReviewWorkspaceSeedInput,
    diff: DiffFileInput,
    expected: ExpectedReviewWorkspaceSeed,
}

#[derive(Debug, Deserialize)]
struct ChangeListRowFixture {
    row_ref: String,
    file_state_token: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedReviewWorkspaceSeed {
    review_workspace_source_class: String,
    provider_authority_class: String,
    anchor_count: usize,
    all_diff_entries_have_anchors: bool,
    provider_ready_anchor_semantics: bool,
    stable_identity_fields: Vec<String>,
    work_item_linkage: ExpectedWorkItemLinkage,
}

#[derive(Debug, Deserialize)]
struct ExpectedWorkItemLinkage {
    work_item_detail_record_id_ref: String,
    target_object_identity_ref: String,
    linked_review_class: String,
    issue_to_branch_link_class: String,
    actor_ref: String,
    command_id_ref: String,
    source_schema_refs: Vec<String>,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/workspace_seed_alpha")
}

fn open_target(fixture: &ReviewWorkspaceSeedFixture) -> DiffOpenTarget {
    DiffOpenTarget::from_change_list_row_parts(
        &fixture.diff.workspace_ref,
        &fixture.diff.truth_source_ref,
        &fixture.change_list_row.row_ref,
        &fixture.diff.group_token,
        fixture.diff.path.clone(),
        fixture.diff.original_path.clone(),
        &fixture.diff.status_code,
        &fixture.change_list_row.file_state_token,
    )
}

fn packet_for_fixture(fixture: &ReviewWorkspaceSeedFixture) -> ReviewWorkspaceSeedPacket {
    let diff_packet = aureline_review::DiffViewSurfacePacket::from_file_input(
        open_target(fixture),
        fixture.diff.clone(),
    );
    ReviewWorkspaceSeedPacket::from_diff_packet(fixture.workspace_seed.clone(), &diff_packet)
}

#[test]
fn protected_workspace_seed_fixture_materializes_anchors_and_work_item_linkage() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "workspace-seed fixtures must exist");

    for path in fixtures {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: ReviewWorkspaceSeedFixture = serde_yaml::from_str(&text)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(fixture.record_kind, "review_workspace_seed_alpha_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = packet_for_fixture(&fixture);
        assert_eq!(
            packet.review_workspace.review_workspace_source_class,
            fixture.expected.review_workspace_source_class,
            "{}: source class mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.review_workspace.provider_authority_class,
            fixture.expected.provider_authority_class,
            "{}: provider authority mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.anchors.len(),
            fixture.expected.anchor_count,
            "{}: anchor count mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.every_diff_entry_has_stable_anchors(),
            fixture.expected.all_diff_entries_have_anchors,
            "{}: diff entry anchor coverage mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.provider_ready_anchor_semantics(),
            fixture.expected.provider_ready_anchor_semantics,
            "{}: provider-neutral anchor semantics mismatch",
            fixture.case_name
        );
        assert!(
            packet
                .anchors
                .iter()
                .all(|anchor| anchor.stable_identity_fields
                    == fixture.expected.stable_identity_fields),
            "{}: stable identity fields mismatch",
            fixture.case_name
        );
        assert!(
            packet
                .anchors
                .iter()
                .all(|anchor| anchor.target_ref.starts_with("git.diff.hunk.")),
            "{}: anchors must target diff rows",
            fixture.case_name
        );

        let linkage = packet
            .work_item_linkages
            .first()
            .unwrap_or_else(|| panic!("{}: expected a work-item linkage", fixture.case_name));
        assert_eq!(
            linkage.work_item_detail_record_id_ref,
            fixture
                .expected
                .work_item_linkage
                .work_item_detail_record_id_ref
        );
        assert_eq!(
            linkage.target_object_identity_ref,
            fixture
                .expected
                .work_item_linkage
                .target_object_identity_ref
        );
        assert_eq!(
            linkage.linked_review_class,
            fixture.expected.work_item_linkage.linked_review_class
        );
        assert_eq!(
            linkage.issue_to_branch_link_class,
            fixture
                .expected
                .work_item_linkage
                .issue_to_branch_link_class
        );
        assert_eq!(
            linkage.actor_ref,
            fixture.expected.work_item_linkage.actor_ref
        );
        assert_eq!(
            linkage.command_id_ref,
            fixture.expected.work_item_linkage.command_id_ref
        );

        let expected_sources = fixture
            .expected
            .work_item_linkage
            .source_schema_refs
            .into_iter()
            .collect::<BTreeSet<_>>();
        let actual_sources = linkage
            .source_schema_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_eq!(actual_sources, expected_sources);
        assert!(packet.has_attributable_work_item_linkage());
        assert!(packet.inspection.attributable_work_item_linkage_present);
    }
}

#[test]
fn hosted_provider_overlay_preserves_local_anchor_ids() {
    let path = fixtures_dir().join("local_diff_with_work_item_link.yaml");
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
    let fixture: ReviewWorkspaceSeedFixture =
        serde_yaml::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
    let local_packet = packet_for_fixture(&fixture);
    let diff_packet = aureline_review::DiffViewSurfacePacket::from_file_input(
        open_target(&fixture),
        fixture.diff.clone(),
    );
    let hosted_input = fixture
        .workspace_seed
        .with_provider_overlay(ReviewProviderOverlayInput {
            provider_class: "review_or_code_host".to_string(),
            connected_provider_record_id_ref: "connected_provider.fixture.github".to_string(),
            provider_object_identity_ref: "provider.review.fixture.pr-1842".to_string(),
            provider_overlay_freshness_class: "provider_overlay_fresh".to_string(),
            last_fetched_at: "2026-05-13T00:00:00Z".to_string(),
            grace_window_seconds: Some(600),
        });
    let hosted_packet = ReviewWorkspaceSeedPacket::from_diff_packet(hosted_input, &diff_packet);

    let local_anchor_ids = local_packet
        .anchors
        .iter()
        .map(|anchor| anchor.anchor_id.as_str())
        .collect::<Vec<_>>();
    let hosted_anchor_ids = hosted_packet
        .anchors
        .iter()
        .map(|anchor| anchor.anchor_id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(local_anchor_ids, hosted_anchor_ids);
    assert_eq!(
        hosted_packet.review_workspace.review_workspace_source_class,
        "composite_local_with_provider_overlay"
    );
    assert_eq!(
        hosted_packet.work_item_linkages[0].linked_review_class,
        "linked_review_workspace_with_provider_overlay"
    );
    assert!(hosted_packet.provider_ready_anchor_semantics());
}
