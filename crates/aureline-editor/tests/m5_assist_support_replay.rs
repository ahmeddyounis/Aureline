//! Freeze gate for the canonical assist-support packet.
//!
//! The checked-in fixture `fixtures/editor/m5-assist-support/canonical_packet.json`
//! is the published packet. This gate rebuilds the packet in code and asserts it
//! equals the fixture byte-for-byte after a serialize round-trip, so the in-code
//! packet cannot drift from the published artifact without failing CI. It also
//! re-proves every frozen invariant, support-export safety, full drift-class
//! coverage, and that every decision carries stable ids and a redaction-safe flag.

use std::path::{Path, PathBuf};

use aureline_editor::{
    assist_support_packet, assist_support_packet_lines, AssistDecisionKind, AssistDriftClass,
    AssistSupportPacket, EditorSurfaceClass, M5_ASSIST_SUPPORT_RECORD_KIND,
    M5_ASSIST_SUPPORT_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-assist-support/canonical_packet.json")
}

fn load_fixture() -> AssistSupportPacket {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_packet_matches_checked_in_fixture() {
    let built = assist_support_packet();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code assist-support packet drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_assist_support`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ASSIST_SUPPORT_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ASSIST_SUPPORT_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: AssistSupportPacket =
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
fn packet_covers_every_drift_class_and_decision_kind() {
    let fixture = load_fixture();
    for class in AssistDriftClass::ALL {
        assert!(
            fixture
                .decisions
                .iter()
                .any(|decision| decision.drift_class == class),
            "missing decision for drift class {}",
            class.as_str()
        );
        assert!(
            fixture.drift_rollup(class).is_some(),
            "missing rollup for drift class {}",
            class.as_str()
        );
    }
    for kind in AssistDecisionKind::ALL {
        assert!(
            fixture.decisions_for_kind(kind).next().is_some(),
            "missing decision for kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn packet_covers_constrained_surfaces() {
    let fixture = load_fixture();
    for surface in [
        EditorSurfaceClass::NotebookCell,
        EditorSurfaceClass::RequestEditor,
        EditorSurfaceClass::SqlEditor,
        EditorSurfaceClass::DocsCodeBlock,
        EditorSurfaceClass::GeneratedFile,
        EditorSurfaceClass::ProtectedFile,
        EditorSurfaceClass::PartialIndexState,
        EditorSurfaceClass::LargeFileRestricted,
    ] {
        assert!(
            fixture.decisions_for_surface(surface).next().is_some(),
            "missing constrained surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_decision_is_stable_and_redaction_safe() {
    let fixture = load_fixture();
    for decision in &fixture.decisions {
        assert!(decision.decision_id.starts_with(decision.kind.id_prefix()));
        assert!(!decision.field_id.is_empty());
        assert!(!decision.subject_ref.is_empty());
        assert!(decision.redaction_safe);
        if !decision.is_clean() {
            assert!(decision.next_safe_action.is_some());
            assert!(!decision.explanation.is_empty());
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = assist_support_packet_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Assist-support packet")));
    assert!(lines.iter().any(|line| line.contains("Support export:")));
    for class in AssistDriftClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(class.as_str())),
            "projection must mention drift class {}",
            class.as_str()
        );
    }
}
