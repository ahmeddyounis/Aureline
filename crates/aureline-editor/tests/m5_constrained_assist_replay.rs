//! Freeze gate for the canonical constrained-file and degraded-provider
//! assist-narrowing model.
//!
//! The checked-in fixture `fixtures/editor/m5-constrained-assist/canonical_model.json`
//! is the published model. This gate rebuilds the model in code and asserts it
//! equals the fixture after a serialize round-trip, so the in-code model cannot
//! drift from the published artifact without failing CI. It also re-proves every
//! frozen invariant, support-export safety, the inspectable-reason guardrail, the
//! blocked-apply next-safe-action contract, and the consumer-surface reuse
//! contract.

use std::path::{Path, PathBuf};

use aureline_editor::{
    constrained_assist_model, constrained_assist_model_lines, AssistChannelClass,
    AssistDegradeClass, ConstrainedAssistModel, ConstrainedFileStateClass, EditorSurfaceClass,
    NextSafeActionClass, M5_CONSTRAINED_ASSIST_RECORD_KIND, M5_CONSTRAINED_ASSIST_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-constrained-assist/canonical_model.json")
}

fn load_fixture() -> ConstrainedAssistModel {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_model_matches_checked_in_fixture() {
    let built = constrained_assist_model();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code constrained-assist model drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_constrained_assist`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_CONSTRAINED_ASSIST_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_CONSTRAINED_ASSIST_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: ConstrainedAssistModel =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn every_constrained_state_present_with_full_channel_coverage() {
    let fixture = load_fixture();
    assert_eq!(
        fixture.state_profiles.len(),
        ConstrainedFileStateClass::ALL.len()
    );
    for state in ConstrainedFileStateClass::ALL {
        let profile = fixture
            .profile(state)
            .unwrap_or_else(|| panic!("missing profile for {}", state.as_str()));
        assert_eq!(profile.cells.len(), AssistChannelClass::ALL.len());
    }
}

#[test]
fn narrowed_cells_disclose_and_blocked_apply_offers_route() {
    let fixture = load_fixture();
    for cell in fixture.all_cells() {
        assert!(
            cell.reason_inspectable(),
            "channel {} must disclose its narrowing reason",
            cell.channel.as_str()
        );
        assert!(
            cell.apply_block_offers_route(),
            "channel {} blocks apply without a route",
            cell.channel.as_str()
        );
        assert!(
            cell.no_silent_hidden_side_effect(),
            "channel {} silently hides a side-effectful assist",
            cell.channel.as_str()
        );
    }
}

#[test]
fn large_file_suppresses_and_partial_index_pends() {
    let fixture = load_fixture();
    let large = fixture
        .profile(ConstrainedFileStateClass::LargeFile)
        .expect("large file");
    for cell in &large.cells {
        if cell.channel.is_semantic() || cell.channel.is_apply_capable() {
            assert_eq!(cell.degrade_class, AssistDegradeClass::SuppressedLargeFile);
            assert_eq!(
                cell.next_safe_action,
                Some(NextSafeActionClass::OpenInFullEditor)
            );
        }
    }

    let partial = fixture
        .profile(ConstrainedFileStateClass::PartialIndex)
        .expect("partial index");
    for cell in partial.cells.iter().filter(|c| c.channel.is_semantic()) {
        assert_eq!(cell.degrade_class, AssistDegradeClass::PendingPartialIndex);
        assert!(!cell.apply_blocked);
    }
}

#[test]
fn degraded_provider_cases_are_honest() {
    let fixture = load_fixture();
    assert!(!fixture.degraded_provider_cases.is_empty());
    for case in &fixture.degraded_provider_cases {
        assert!(
            case.is_honest(),
            "degraded-provider case {} must be source-labeled, routed, and disclosed",
            case.case_id
        );
    }
}

#[test]
fn claimed_consumer_surfaces_reuse_shared_vocabulary() {
    let fixture = load_fixture();
    for surface in [
        EditorSurfaceClass::NotebookCell,
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::RequestEditor,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::ProtectedFile,
    ] {
        let proof = fixture
            .consumer_proofs
            .iter()
            .find(|p| p.base_editor_surface == Some(surface))
            .unwrap_or_else(|| panic!("missing consumer proof for {}", surface.as_str()));
        assert!(proof.reuses_shared_vocabulary);
        let profile = fixture.profile(proof.exhibited_state).expect("profile");
        let cell = profile.cell(proof.representative_channel).expect("cell");
        assert_eq!(cell.degrade_class, proof.resolved_degrade);
        assert_eq!(cell.next_safe_action, proof.next_safe_action);
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = constrained_assist_model_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Constrained-assist model")));
    assert!(lines.iter().any(|line| line.contains("State profiles:")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Degraded-provider cases:")));
    assert!(lines.iter().any(|line| line.contains("Consumer proofs:")));
    for profile in &fixture.state_profiles {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(profile.state_class.as_str())),
            "projection must mention state {}",
            profile.state_class.as_str()
        );
    }
}
