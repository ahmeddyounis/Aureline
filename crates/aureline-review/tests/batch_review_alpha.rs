//! Fixture-driven coverage for review batch-review sheets.
//!
//! The cases under `fixtures/review/batch_review_alpha/` prove review
//! collections consume the shared filter, counter, selection, and batch-review
//! vocabulary instead of creating local scope semantics.

use std::path::{Path, PathBuf};

use aureline_review::diff::{DiffFileInput, DiffOpenTarget};
use aureline_review::workspace::{ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket};
use aureline_review::{ReviewCollectionAlphaInput, ReviewCollectionAlphaPacket};
use aureline_search::{
    BatchActionClass, BatchExecutionOriginClass, CollectionSurfaceFamily, SelectionScopeClass,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ReviewBatchFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    workspace_seed_fixture: String,
    input: ReviewBatchInputFixture,
    expected: ReviewBatchExpected,
}

#[derive(Debug, Deserialize)]
struct ReviewBatchInputFixture {
    collection_view_id: String,
    batch_review_id: String,
    action_id: String,
    action_label: String,
    action_class: BatchActionClass,
    selection_scope_class: SelectionScopeClass,
    execution_origin_class: BatchExecutionOriginClass,
    selected_anchor_indexes: Vec<usize>,
    blocked_anchor_indexes: Vec<usize>,
    hidden_anchor_indexes: Vec<usize>,
    stale_anchor_indexes: Vec<usize>,
    generated_at: String,
}

#[derive(Debug, Deserialize)]
struct ReviewBatchExpected {
    surface_family: CollectionSurfaceFamily,
    hidden_narrowing_count: usize,
    selected_count: u64,
    blocked_count: u64,
    hidden_count: u64,
    included_count: usize,
    excluded_count: usize,
    review_required: bool,
    validation_findings: usize,
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review/batch_review_alpha")
}

fn workspace_fixtures_dir() -> PathBuf {
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

fn seed_packet_for(path: &Path) -> ReviewWorkspaceSeedPacket {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("workspace fixture {path:?} must read: {err}"));
    let fixture: ReviewWorkspaceSeedFixture = serde_yaml::from_str(&text)
        .unwrap_or_else(|err| panic!("workspace fixture {path:?} must parse: {err}"));
    let diff_packet = aureline_review::DiffViewSurfacePacket::from_file_input(
        open_target(&fixture),
        fixture.diff.clone(),
    );
    ReviewWorkspaceSeedPacket::from_diff_packet(fixture.workspace_seed, &diff_packet)
}

fn anchor_ids(seed: &ReviewWorkspaceSeedPacket, indexes: &[usize]) -> Vec<String> {
    indexes
        .iter()
        .map(|index| {
            seed.anchors
                .get(*index)
                .unwrap_or_else(|| panic!("anchor index {index} must exist"))
                .anchor_id
                .clone()
        })
        .collect()
}

fn collection_input(
    fixture: &ReviewBatchFixture,
    seed: &ReviewWorkspaceSeedPacket,
) -> ReviewCollectionAlphaInput {
    ReviewCollectionAlphaInput {
        collection_view_id: fixture.input.collection_view_id.clone(),
        batch_review_id: fixture.input.batch_review_id.clone(),
        action_id: fixture.input.action_id.clone(),
        action_label: fixture.input.action_label.clone(),
        action_class: fixture.input.action_class,
        selection_scope_class: fixture.input.selection_scope_class,
        execution_origin_class: fixture.input.execution_origin_class,
        selected_anchor_id_refs: anchor_ids(seed, &fixture.input.selected_anchor_indexes),
        blocked_anchor_id_refs: anchor_ids(seed, &fixture.input.blocked_anchor_indexes),
        hidden_anchor_id_refs: anchor_ids(seed, &fixture.input.hidden_anchor_indexes),
        stale_anchor_id_refs: anchor_ids(seed, &fixture.input.stale_anchor_indexes),
        generated_at: fixture.input.generated_at.clone(),
    }
}

#[test]
fn protected_review_batch_fixtures_emit_review_sheets() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "review batch fixtures must exist");

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: ReviewBatchFixture = serde_yaml::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(fixture.record_kind, "review_batch_review_alpha_case");
        assert_eq!(fixture.schema_version, 1);

        let seed = seed_packet_for(&workspace_fixtures_dir().join(&fixture.workspace_seed_fixture));
        let packet = ReviewCollectionAlphaPacket::from_workspace_seed(
            collection_input(&fixture, &seed),
            &seed,
        );
        assert_eq!(
            packet.collection_view.surface_family, fixture.expected.surface_family,
            "{}: surface family mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.collection_view.hidden_narrowing_labels.len(),
            fixture.expected.hidden_narrowing_count,
            "{}: hidden narrowing count mismatch",
            fixture.case_name
        );
        assert!(packet.collection_view.surfaces_hidden_narrowing());
        assert!(packet.preserves_anchor_identity_selection());
        assert_eq!(
            packet.collection_view.counters.selected.value,
            Some(fixture.expected.selected_count),
            "{}: selected count mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.batch_review_sheet.blocked_item_id_refs.len() as u64,
            fixture.expected.blocked_count,
            "{}: blocked count mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.batch_review_sheet.hidden_item_id_refs.len() as u64,
            fixture.expected.hidden_count,
            "{}: hidden count mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.batch_review_sheet.included_item_id_refs.len(),
            fixture.expected.included_count,
            "{}: included count mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.batch_review_sheet.excluded_item_id_refs.len(),
            fixture.expected.excluded_count,
            "{}: excluded count mismatch",
            fixture.case_name
        );
        assert_eq!(
            packet.batch_review_sheet.review_required, fixture.expected.review_required,
            "{}: review-required mismatch",
            fixture.case_name
        );
        let findings = packet.batch_review_sheet.validate();
        assert_eq!(
            findings.len(),
            fixture.expected.validation_findings,
            "{}: unexpected batch-review findings: {findings:?}",
            fixture.case_name
        );
    }
}
