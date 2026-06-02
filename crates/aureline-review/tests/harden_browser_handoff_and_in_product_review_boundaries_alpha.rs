//! Fixture-driven coverage for review-boundary-hardening packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/harden-browser-handoff-and-in-product-review-boundaries/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Boundary-truth axes are surfaced as separable inspectable truths.
//! 3. Browser handoff boundaries are reversible and typed when claimed.
//! 4. Provider identity is fully disclosed at the boundary.
//! 5. In-product review boundaries do not hide hosted authority behind local chrome.
//! 6. Return paths are explicit, typed, and not expired when required.
//! 7. Boundary freshness observations block mutation when stale.
//! 8. Boundary ownership signals remain split between advisory and enforceable classes.
//! 9. Support/export records keep every `raw_*_export_allowed` flag false.

use std::path::{Path, PathBuf};

use aureline_review::{
    DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket, LandingCandidateInput,
    LandingCandidatePacket, ReviewBoundaryHardeningInput, ReviewBoundaryHardeningPacket,
    ReviewStabilizationInput, ReviewStabilizationPacket, ReviewWorkspaceBetaInput,
    ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BoundaryHardeningFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    landing_input: LandingCandidateInput,
    stabilization_input: ReviewStabilizationInput,
    boundary_hardening_input: ReviewBoundaryHardeningInput,
    expected: ExpectedBoundaryHardening,
}

#[derive(Debug, Deserialize)]
struct ExpectedBoundaryHardening {
    boundary_hardened: bool,
    boundary_degraded_provider_overlay_stale: bool,
    boundary_degraded_missing_return_path: bool,
    boundary_degraded_hidden_authority: bool,
    boundary_degraded_freshness_unknown: bool,
    boundary_degraded_ownership_ambiguous: bool,
    handoff_reversible_typed: bool,
    in_product_local_authoritative: bool,
    in_product_provider_authoritative: bool,
    local_provider_agree: bool,
    hidden_authority_detected: bool,
    return_path_present_and_valid: bool,
    boundary_fresh_or_within_grace: bool,
    boundary_freshness_blocks_mutation: bool,
    enforceable_ownership_at_boundary: bool,
    advisory_ownership_at_boundary: bool,
    ownership_conflict_at_boundary: bool,
    actionable: bool,
    invalidated: bool,
    command_count: usize,
    boundary_ownership_signal_count: usize,
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
        .join("../../fixtures/review/m4/harden-browser-handoff-and-in-product-review-boundaries")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("boundary hardening fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> BoundaryHardeningFixture {
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

fn workspace_packet_for(fixture: &BoundaryHardeningFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn landing_packet_for(fixture: &BoundaryHardeningFixture) -> LandingCandidatePacket {
    let workspace_packet = workspace_packet_for(fixture);
    LandingCandidatePacket::from_workspace_packet(fixture.landing_input.clone(), &workspace_packet)
        .unwrap_or_else(|err| panic!("{} landing packet must project: {err}", fixture.case_name))
}

fn stabilization_packet_for(fixture: &BoundaryHardeningFixture) -> ReviewStabilizationPacket {
    let workspace_packet = workspace_packet_for(fixture);
    let landing_packet = landing_packet_for(fixture);
    ReviewStabilizationPacket::from_workspace_and_landing_packets(
        fixture.stabilization_input.clone(),
        &workspace_packet,
        &landing_packet,
    )
    .unwrap_or_else(|err| panic!("{} stabilization must project: {err}", fixture.case_name))
}

fn boundary_hardening_packet_for(
    fixture: &BoundaryHardeningFixture,
) -> ReviewBoundaryHardeningPacket {
    let workspace_packet = workspace_packet_for(fixture);
    let stabilization_packet = stabilization_packet_for(fixture);
    ReviewBoundaryHardeningPacket::from_workspace_and_stabilization_packets(
        fixture.boundary_hardening_input.clone(),
        &workspace_packet,
        &stabilization_packet,
    )
    .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(
    packet: &ReviewBoundaryHardeningPacket,
    expected: &ExpectedBoundaryHardening,
    case_name: &str,
) {
    assert_eq!(
        packet.inspection.boundary_hardened, expected.boundary_hardened,
        "{case_name}: boundary_hardened"
    );
    assert_eq!(
        packet.inspection.boundary_degraded_provider_overlay_stale,
        expected.boundary_degraded_provider_overlay_stale,
        "{case_name}: boundary_degraded_provider_overlay_stale"
    );
    assert_eq!(
        packet.inspection.boundary_degraded_missing_return_path,
        expected.boundary_degraded_missing_return_path,
        "{case_name}: boundary_degraded_missing_return_path"
    );
    assert_eq!(
        packet.inspection.boundary_degraded_hidden_authority,
        expected.boundary_degraded_hidden_authority,
        "{case_name}: boundary_degraded_hidden_authority"
    );
    assert_eq!(
        packet.inspection.boundary_degraded_freshness_unknown,
        expected.boundary_degraded_freshness_unknown,
        "{case_name}: boundary_degraded_freshness_unknown"
    );
    assert_eq!(
        packet.inspection.boundary_degraded_ownership_ambiguous,
        expected.boundary_degraded_ownership_ambiguous,
        "{case_name}: boundary_degraded_ownership_ambiguous"
    );
    assert_eq!(
        packet.inspection.handoff_reversible_typed, expected.handoff_reversible_typed,
        "{case_name}: handoff_reversible_typed"
    );
    assert_eq!(
        packet.inspection.in_product_local_authoritative, expected.in_product_local_authoritative,
        "{case_name}: in_product_local_authoritative"
    );
    assert_eq!(
        packet.inspection.in_product_provider_authoritative,
        expected.in_product_provider_authoritative,
        "{case_name}: in_product_provider_authoritative"
    );
    assert_eq!(
        packet.inspection.local_provider_agree, expected.local_provider_agree,
        "{case_name}: local_provider_agree"
    );
    assert_eq!(
        packet.inspection.hidden_authority_detected, expected.hidden_authority_detected,
        "{case_name}: hidden_authority_detected"
    );
    assert_eq!(
        packet.inspection.return_path_present_and_valid, expected.return_path_present_and_valid,
        "{case_name}: return_path_present_and_valid"
    );
    assert_eq!(
        packet.inspection.boundary_fresh_or_within_grace, expected.boundary_fresh_or_within_grace,
        "{case_name}: boundary_fresh_or_within_grace"
    );
    assert_eq!(
        packet.inspection.boundary_freshness_blocks_mutation,
        expected.boundary_freshness_blocks_mutation,
        "{case_name}: boundary_freshness_blocks_mutation"
    );
    assert_eq!(
        packet.inspection.enforceable_ownership_at_boundary,
        expected.enforceable_ownership_at_boundary,
        "{case_name}: enforceable_ownership_at_boundary"
    );
    assert_eq!(
        packet.inspection.advisory_ownership_at_boundary, expected.advisory_ownership_at_boundary,
        "{case_name}: advisory_ownership_at_boundary"
    );
    assert_eq!(
        packet.inspection.ownership_conflict_at_boundary, expected.ownership_conflict_at_boundary,
        "{case_name}: ownership_conflict_at_boundary"
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
        packet.inspection.boundary_ownership_signal_count, expected.boundary_ownership_signal_count,
        "{case_name}: boundary_ownership_signal_count"
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
fn boundary_hardening_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "boundary hardening fixtures must exist");

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: BoundaryHardeningFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(fixture.record_kind, "review_boundary_hardening_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = boundary_hardening_packet_for(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.truths_are_separable(), "{}", fixture.case_name);
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);
        assert!(
            packet.provider_identity_disclosed(),
            "{}",
            fixture.case_name
        );
        assert!(
            packet.ownership_signals_properly_split(),
            "{}",
            fixture.case_name
        );

        assert_expected(&packet, &fixture.expected, &fixture.case_name);

        // Round-trip through JSON and re-validate.
        let json = serde_json::to_string(&packet).expect("serialization must succeed");
        let reparsed: ReviewBoundaryHardeningPacket =
            serde_json::from_str(&json).expect("re-deserialization must succeed");
        reparsed
            .validate()
            .unwrap_or_else(|err| panic!("{} round-trip must validate: {err}", fixture.case_name));
        assert_expected(&reparsed, &fixture.expected, &fixture.case_name);
    }
}

#[test]
fn hidden_authority_rejects_boundary_hardened() {
    let fixture = load_fixture("boundary_hardened_with_reversible_handoff.json");
    let mut fixture = fixture;
    fixture
        .boundary_hardening_input
        .in_product_boundary
        .hidden_authority_detected = true;
    fixture.boundary_hardening_input.boundary_hardening_state = "boundary_hardened".to_string();

    let workspace_packet = workspace_packet_for(&fixture);
    let stabilization_packet = stabilization_packet_for(&fixture);
    let result = ReviewBoundaryHardeningPacket::from_workspace_and_stabilization_packets(
        fixture.boundary_hardening_input,
        &workspace_packet,
        &stabilization_packet,
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err
        .message()
        .contains("boundary_hardened state is incompatible with hidden_authority_detected"));
}

#[test]
fn expired_return_path_degrades_boundary() {
    let fixture = load_fixture("boundary_degraded_missing_return_path.json");
    let packet = boundary_hardening_packet_for(&fixture);

    assert!(
        packet.inspection.boundary_degraded_missing_return_path,
        "missing return path must be explicitly flagged"
    );
    assert!(
        !packet.inspection.return_path_present_and_valid,
        "expired return path must not claim validity"
    );
}

#[test]
fn provider_identity_disclosed_on_every_fixture() {
    let paths = load_fixture_paths();
    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: BoundaryHardeningFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let packet = boundary_hardening_packet_for(&fixture);
        assert!(
            packet.provider_identity_disclosed(),
            "{}: provider identity must be disclosed",
            fixture.case_name
        );
    }
}

#[test]
fn ownership_conflict_detected_at_boundary() {
    let fixture = load_fixture("boundary_degraded_ownership_ambiguous.json");
    let packet = boundary_hardening_packet_for(&fixture);

    assert!(packet.inspection.ownership_conflict_at_boundary);
    assert!(packet.inspection.enforceable_ownership_at_boundary);
    assert!(packet.inspection.advisory_ownership_at_boundary);
    assert!(packet.inspection.boundary_degraded_ownership_ambiguous);
}
