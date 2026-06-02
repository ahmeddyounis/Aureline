//! Fixture-driven coverage for AI review evidence attachment and safe
//! suggestion apply packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/stabilize-review-side-ai-evidence-attachment-and-safe/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. AI evidence sources are disclosed and from the closed vocabulary.
//! 3. Safe suggestion apply never broadens authority.
//! 4. Applied suggestions always carry checkpoints.
//! 5. Support/export records keep every `raw_*_export_allowed` flag false and
//!    consumer-surface lists include both `support_export` and `audit_lane`.
//! 6. Raw escape hatches are absent from the support boundary.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_ai_review_evidence_packet, AiReviewEvidenceInput, AiReviewEvidencePacket,
    DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket, ReviewWorkspaceBetaInput,
    ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EvidenceFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    evidence_input: AiReviewEvidenceInput,
    expected: ExpectedEvidence,
}

#[derive(Debug, Deserialize)]
struct ExpectedEvidence {
    evidence_state_attached_current: bool,
    evidence_state_attached_stale: bool,
    evidence_state_detached_invalidated: bool,
    evidence_state_pending_verification: bool,
    ai_model_source_present: bool,
    human_curated_source_present: bool,
    suggestion_preview_ready: bool,
    suggestion_applied_with_checkpoint: bool,
    suggestion_reverted: bool,
    suggestion_blocked_pending_review: bool,
    suggestion_blocked_scope_exceeded: bool,
    suggestion_would_broaden_authority: bool,
    all_applied_have_checkpoints: bool,
    all_checkpoints_recoverable: bool,
    actionable: bool,
    invalidated: bool,
    evidence_attachment_count: usize,
    suggestion_apply_count: usize,
    checkpoint_count: usize,
    command_count: usize,
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
        .join("../../fixtures/review/m4/stabilize-review-side-ai-evidence-attachment-and-safe")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("evidence fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> EvidenceFixture {
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

fn workspace_packet_for(fixture: &EvidenceFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn evidence_packet_for(fixture: &EvidenceFixture) -> AiReviewEvidencePacket {
    let workspace_packet = workspace_packet_for(fixture);
    AiReviewEvidencePacket::from_workspace_packet(fixture.evidence_input.clone(), &workspace_packet)
        .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(packet: &AiReviewEvidencePacket, expected: &ExpectedEvidence, case_name: &str) {
    assert_eq!(
        packet.inspection.evidence_state_attached_current, expected.evidence_state_attached_current,
        "{case_name}: evidence_state_attached_current"
    );
    assert_eq!(
        packet.inspection.evidence_state_attached_stale, expected.evidence_state_attached_stale,
        "{case_name}: evidence_state_attached_stale"
    );
    assert_eq!(
        packet.inspection.evidence_state_detached_invalidated,
        expected.evidence_state_detached_invalidated,
        "{case_name}: evidence_state_detached_invalidated"
    );
    assert_eq!(
        packet.inspection.evidence_state_pending_verification,
        expected.evidence_state_pending_verification,
        "{case_name}: evidence_state_pending_verification"
    );
    assert_eq!(
        packet.inspection.ai_model_source_present, expected.ai_model_source_present,
        "{case_name}: ai_model_source_present"
    );
    assert_eq!(
        packet.inspection.human_curated_source_present, expected.human_curated_source_present,
        "{case_name}: human_curated_source_present"
    );
    assert_eq!(
        packet.inspection.suggestion_preview_ready, expected.suggestion_preview_ready,
        "{case_name}: suggestion_preview_ready"
    );
    assert_eq!(
        packet.inspection.suggestion_applied_with_checkpoint,
        expected.suggestion_applied_with_checkpoint,
        "{case_name}: suggestion_applied_with_checkpoint"
    );
    assert_eq!(
        packet.inspection.suggestion_reverted, expected.suggestion_reverted,
        "{case_name}: suggestion_reverted"
    );
    assert_eq!(
        packet.inspection.suggestion_blocked_pending_review,
        expected.suggestion_blocked_pending_review,
        "{case_name}: suggestion_blocked_pending_review"
    );
    assert_eq!(
        packet.inspection.suggestion_blocked_scope_exceeded,
        expected.suggestion_blocked_scope_exceeded,
        "{case_name}: suggestion_blocked_scope_exceeded"
    );
    assert_eq!(
        packet.inspection.suggestion_would_broaden_authority,
        expected.suggestion_would_broaden_authority,
        "{case_name}: suggestion_would_broaden_authority"
    );
    assert_eq!(
        packet.inspection.all_applied_have_checkpoints, expected.all_applied_have_checkpoints,
        "{case_name}: all_applied_have_checkpoints"
    );
    assert_eq!(
        packet.inspection.all_checkpoints_recoverable, expected.all_checkpoints_recoverable,
        "{case_name}: all_checkpoints_recoverable"
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
        packet.inspection.evidence_attachment_count, expected.evidence_attachment_count,
        "{case_name}: evidence_attachment_count"
    );
    assert_eq!(
        packet.inspection.suggestion_apply_count, expected.suggestion_apply_count,
        "{case_name}: suggestion_apply_count"
    );
    assert_eq!(
        packet.inspection.checkpoint_count, expected.checkpoint_count,
        "{case_name}: checkpoint_count"
    );
    assert_eq!(
        packet.inspection.command_count, expected.command_count,
        "{case_name}: command_count"
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
fn evidence_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "evidence fixtures must exist");

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: EvidenceFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(fixture.record_kind, "ai_review_evidence_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = evidence_packet_for(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);
        assert!(packet.evidence_sources_disclosed(), "{}", fixture.case_name);

        assert_expected(&packet, &fixture.expected, &fixture.case_name);

        // Round-trip through JSON and re-validate.
        let json = serde_json::to_string(&packet).expect("serialization must succeed");
        let reparsed: AiReviewEvidencePacket =
            serde_json::from_str(&json).expect("re-deserialization must succeed");
        reparsed
            .validate()
            .unwrap_or_else(|err| panic!("{} round-trip must validate: {err}", fixture.case_name));
        assert_expected(&reparsed, &fixture.expected, &fixture.case_name);
    }
}

#[test]
fn attached_current_has_no_authority_broadening() {
    let fixture = load_fixture("attached_current_with_preview.json");
    let packet = evidence_packet_for(&fixture);

    assert!(
        packet.inspection.evidence_state_attached_current,
        "attached_current must be true"
    );
    assert!(
        !packet.inspection.suggestion_would_broaden_authority,
        "must not have authority-broadening suggestions"
    );
    assert!(
        packet.evidence_sources_disclosed(),
        "evidence sources must be disclosed"
    );
}

#[test]
fn applied_suggestion_carries_checkpoint() {
    let fixture = load_fixture("applied_with_checkpoint.json");
    let packet = evidence_packet_for(&fixture);

    assert!(
        packet.inspection.suggestion_applied_with_checkpoint,
        "must have applied suggestion"
    );
    assert!(
        packet.inspection.all_applied_have_checkpoints,
        "all applied suggestions must have checkpoints"
    );
    assert!(
        packet.applied_suggestions_have_checkpoints(),
        "packet must confirm applied suggestions have checkpoints"
    );
}

#[test]
fn authority_broadening_rejects_apply() {
    let fixture = load_fixture("blocked_authority_broadening.json");
    let packet = evidence_packet_for(&fixture);

    assert!(
        packet.inspection.suggestion_would_broaden_authority,
        "must flag authority-broadening suggestion"
    );
    assert!(
        packet.inspection.suggestion_blocked_pending_review
            || packet.inspection.suggestion_blocked_scope_exceeded,
        "must be blocked"
    );
    assert!(
        !packet.no_authority_broadening(),
        "packet must detect authority broadening"
    );
}

#[test]
fn projection_from_json_payload() {
    let fixture = load_fixture("attached_current_with_preview.json");
    let packet = evidence_packet_for(&fixture);
    let json = serde_json::to_string(&packet).expect("serialization must succeed");

    let projection = project_ai_review_evidence_packet(&json).expect("projection must succeed");
    assert_eq!(projection.evidence_id, packet.evidence.evidence_id);
    assert_eq!(
        projection.review_workspace_id,
        packet.review_workspace.review_workspace_id
    );
    assert_eq!(projection.evidence_state, packet.evidence.evidence_state);
}
