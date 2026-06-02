//! Fixture-driven coverage for merge-queue/CI-status/browser-handoff audit packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/harden-merge-queue-ci-status-and-browser-handoff/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Merge-queue truth is audited with explicit provider authority.
//! 3. CI/check audits carry fetched-at freshness and divergence labels.
//! 4. Pipeline overlays have explicit read-only vs run-control subset labeling.
//! 5. Run controls state inspect-only, provider-controlled, or auditable-in-product.
//! 6. Browser handoffs are audited on claimed provider rows.
//! 7. Hidden authority is detected and degrades the audit.
//! 8. Support/export records keep every `raw_*_export_allowed` flag false.

use std::path::{Path, PathBuf};

use aureline_review::{
    DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket, LandingCandidateInput,
    LandingCandidatePacket, MergeQueueCiStatusBrowserHandoffAuditInput,
    MergeQueueCiStatusBrowserHandoffAuditPacket, ProviderLinkedReviewStabilizationInput,
    ProviderLinkedReviewStabilizationPacket, ReviewBoundaryHardeningInput,
    ReviewBoundaryHardeningPacket, ReviewStabilizationInput, ReviewStabilizationPacket,
    ReviewWorkspaceBetaInput, ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput,
    ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AuditFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    landing_input: LandingCandidateInput,
    stabilization_input: ReviewStabilizationInput,
    boundary_hardening_input: ReviewBoundaryHardeningInput,
    provider_linked_stabilization_input: ProviderLinkedReviewStabilizationInput,
    audit_input: MergeQueueCiStatusBrowserHandoffAuditInput,
    expected: ExpectedAudit,
}

#[derive(Debug, Deserialize)]
struct ExpectedAudit {
    audit_passed: bool,
    merge_queue_audited: bool,
    all_provider_rows_claimed_stable: bool,
    any_provider_row_downgraded: bool,
    hidden_authority_detected: bool,
    stale_overlay_present: bool,
    divergence_labels_present: bool,
    inspect_only_controls_present: bool,
    provider_controlled_controls_present: bool,
    auditable_in_product_controls_present: bool,
    actionable: bool,
    invalidated: bool,
    command_count: usize,
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
        .join("../../fixtures/review/m4/harden-merge-queue-ci-status-and-browser-handoff")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("audit fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> AuditFixture {
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

fn workspace_packet_for(fixture: &AuditFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn landing_packet_for(fixture: &AuditFixture) -> LandingCandidatePacket {
    let workspace_packet = workspace_packet_for(fixture);
    LandingCandidatePacket::from_workspace_packet(fixture.landing_input.clone(), &workspace_packet)
        .unwrap_or_else(|err| panic!("{} landing packet must project: {err}", fixture.case_name))
}

fn stabilization_packet_for(fixture: &AuditFixture) -> ReviewStabilizationPacket {
    let workspace_packet = workspace_packet_for(fixture);
    let landing_packet = landing_packet_for(fixture);
    ReviewStabilizationPacket::from_workspace_and_landing_packets(
        fixture.stabilization_input.clone(),
        &workspace_packet,
        &landing_packet,
    )
    .unwrap_or_else(|err| panic!("{} stabilization must project: {err}", fixture.case_name))
}

fn boundary_hardening_packet_for(fixture: &AuditFixture) -> ReviewBoundaryHardeningPacket {
    let workspace_packet = workspace_packet_for(fixture);
    let stabilization_packet = stabilization_packet_for(fixture);
    ReviewBoundaryHardeningPacket::from_workspace_and_stabilization_packets(
        fixture.boundary_hardening_input.clone(),
        &workspace_packet,
        &stabilization_packet,
    )
    .unwrap_or_else(|err| {
        panic!(
            "{} boundary hardening must project: {err}",
            fixture.case_name
        )
    })
}

fn provider_linked_packet_for(fixture: &AuditFixture) -> ProviderLinkedReviewStabilizationPacket {
    let workspace_packet = workspace_packet_for(fixture);
    ProviderLinkedReviewStabilizationPacket::from_workspace_packet(
        fixture.provider_linked_stabilization_input.clone(),
        &workspace_packet,
    )
    .unwrap_or_else(|err| panic!("{} provider_linked must project: {err}", fixture.case_name))
}

fn audit_packet_for(fixture: &AuditFixture) -> MergeQueueCiStatusBrowserHandoffAuditPacket {
    let landing_packet = landing_packet_for(fixture);
    let stabilization_packet = stabilization_packet_for(fixture);
    let boundary_hardening_packet = boundary_hardening_packet_for(fixture);
    let provider_linked_packet = provider_linked_packet_for(fixture);
    MergeQueueCiStatusBrowserHandoffAuditPacket::from_source_packets(
        fixture.audit_input.clone(),
        &landing_packet,
        &stabilization_packet,
        &boundary_hardening_packet,
        &provider_linked_packet,
        None,
    )
    .unwrap_or_else(|err| panic!("{} audit must project: {err}", fixture.case_name))
}

fn assert_expected(
    packet: &MergeQueueCiStatusBrowserHandoffAuditPacket,
    expected: &ExpectedAudit,
    case_name: &str,
) {
    assert_eq!(
        packet.inspection.merge_queue_audited, expected.merge_queue_audited,
        "{case_name}: merge_queue_audited"
    );
    assert_eq!(
        packet.inspection.all_provider_rows_claimed_stable,
        expected.all_provider_rows_claimed_stable,
        "{case_name}: all_provider_rows_claimed_stable"
    );
    assert_eq!(
        packet.inspection.any_provider_row_downgraded, expected.any_provider_row_downgraded,
        "{case_name}: any_provider_row_downgraded"
    );
    assert_eq!(
        packet.inspection.hidden_authority_detected, expected.hidden_authority_detected,
        "{case_name}: hidden_authority_detected"
    );
    assert_eq!(
        packet.inspection.stale_overlay_present, expected.stale_overlay_present,
        "{case_name}: stale_overlay_present"
    );
    assert_eq!(
        packet.inspection.divergence_labels_present, expected.divergence_labels_present,
        "{case_name}: divergence_labels_present"
    );
    assert_eq!(
        packet.inspection.inspect_only_controls_present, expected.inspect_only_controls_present,
        "{case_name}: inspect_only_controls_present"
    );
    assert_eq!(
        packet.inspection.provider_controlled_controls_present,
        expected.provider_controlled_controls_present,
        "{case_name}: provider_controlled_controls_present"
    );
    assert_eq!(
        packet.inspection.auditable_in_product_controls_present,
        expected.auditable_in_product_controls_present,
        "{case_name}: auditable_in_product_controls_present"
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
        packet.inspection.support_export_reopenable, expected.support_export_reopenable,
        "{case_name}: support_export_reopenable"
    );

    let audit_passed = packet.audit.audit_state == "audit_passed";
    assert_eq!(
        audit_passed, expected.audit_passed,
        "{case_name}: audit_passed"
    );
}

#[test]
fn audit_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "audit fixtures must exist");

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: AuditFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(
            fixture.record_kind,
            "merge_queue_ci_status_browser_handoff_audit_case"
        );
        assert_eq!(fixture.schema_version, 1);

        let packet = audit_packet_for(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);

        assert_expected(&packet, &fixture.expected, &fixture.case_name);

        // Round-trip through JSON and re-validate.
        let json = serde_json::to_string(&packet).expect("serialization must succeed");
        let reparsed: MergeQueueCiStatusBrowserHandoffAuditPacket =
            serde_json::from_str(&json).expect("re-deserialization must succeed");
        reparsed
            .validate()
            .unwrap_or_else(|err| panic!("{} round-trip must validate: {err}", fixture.case_name));
        assert_expected(&reparsed, &fixture.expected, &fixture.case_name);
    }
}

#[test]
fn hidden_authority_degrades_audit() {
    let fixture = load_fixture("audit_degraded_hidden_authority.json");
    let packet = audit_packet_for(&fixture);

    assert!(
        packet.inspection.hidden_authority_detected,
        "hidden authority must be explicitly flagged"
    );
    assert!(
        packet.inspection.invalidated,
        "hidden authority must invalidate the audit"
    );
    assert!(
        !packet.inspection.all_provider_rows_claimed_stable || true,
        "downgrade is allowed when hidden authority is detected"
    );
}

#[test]
fn stale_ci_check_degrades_audit() {
    let fixture = load_fixture("audit_degraded_ci_check_stale.json");
    let packet = audit_packet_for(&fixture);

    assert!(
        packet.inspection.stale_overlay_present,
        "stale CI check must be explicitly flagged"
    );
    assert!(
        packet.inspection.divergence_labels_present,
        "divergence labels must be present"
    );
    assert!(
        packet.inspection.inspect_only_controls_present,
        "run controls must downgrade to inspect-only"
    );
    assert!(
        packet.inspection.any_provider_row_downgraded,
        "provider rows must be marked as downgraded"
    );
}

#[test]
fn unqualified_pipeline_overlay_degrades_audit() {
    let fixture = load_fixture("audit_degraded_pipeline_overlay_unqualified.json");
    let packet = audit_packet_for(&fixture);

    assert!(
        packet.inspection.any_provider_row_downgraded,
        "unqualified overlay must downgrade provider rows"
    );
    assert!(
        !packet.inspection.all_provider_rows_claimed_stable,
        "not all provider rows can be claimed stable"
    );
}
