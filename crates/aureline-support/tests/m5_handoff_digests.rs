//! Freeze gate for the M5 handoff/digest set.
//!
//! The checked-in fixture `fixtures/ops/m5-handoff-digests/canonical_handoff_digests.json`
//! is the published continuity-packet set. This gate rebuilds the set in code and
//! asserts it equals the fixture after a serialize round-trip, so the handoff/digest
//! contract cannot drift from the published artifact without failing CI. It also
//! re-proves support-export safety, full packet coverage, both matrix surfaces, the
//! storage-class distinction, the reopen-safe-continuity rule, severity-before-
//! chronology grouping, the scope/boundary truth, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_support::m5_action_plans::SharePosture;
use aureline_support::m5_handoff_digests::{
    export_packet, handoff_digest_lines, handoff_digest_set, ContinuityPacketKind,
    HandoffDigestSet, PacketClass, ReopenAnchorClass, StorageClass, M5_HANDOFF_DIGESTS_MATRIX_REF,
    M5_HANDOFF_DIGESTS_RECORD_KIND, M5_HANDOFF_DIGESTS_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-handoff-digests/canonical_handoff_digests.json")
}

fn load_fixture() -> HandoffDigestSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = handoff_digest_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code handoff/digest set drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_handoff_digests`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_HANDOFF_DIGESTS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_HANDOFF_DIGESTS_SCHEMA_REF);
    assert_eq!(fixture.matrix_ref, M5_HANDOFF_DIGESTS_MATRIX_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: HandoffDigestSet =
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
fn set_covers_every_packet_and_both_surfaces() {
    let fixture = load_fixture();
    assert_eq!(fixture.packets.len(), PacketClass::ALL.len());
    for packet in PacketClass::ALL {
        let entry = fixture.packet(packet).expect("packet present");
        assert!(!entry.object_groups.is_empty());
        assert_eq!(entry.surface_id, packet.surface().surface_id());
    }
    for kind in ContinuityPacketKind::ALL {
        assert!(fixture.packets.iter().any(|p| p.kind == kind));
    }
}

#[test]
fn fixture_proves_storage_and_anchor_vocabularies() {
    let fixture = load_fixture();
    let evidence: Vec<_> = fixture
        .packets
        .iter()
        .flat_map(|p| p.object_groups.iter())
        .flat_map(|g| g.evidence.iter())
        .collect();
    for sc in StorageClass::ALL {
        assert!(evidence.iter().any(|e| e.storage_class == sc));
    }
    let anchors: Vec<_> = fixture
        .packets
        .iter()
        .flat_map(|p| {
            std::iter::once(&p.reopen_anchor)
                .chain(p.object_groups.iter().map(|g| &g.reopen_anchor))
        })
        .collect();
    for class in ReopenAnchorClass::ALL {
        assert!(anchors.iter().any(|a| a.anchor_class == class));
    }
    for posture in SharePosture::ALL {
        assert!(fixture.packets.iter().any(|p| p.share_posture == posture));
    }
}

#[test]
fn fixture_reopen_never_lands_on_a_generic_dashboard() {
    let fixture = load_fixture();
    let anchors: Vec<_> = fixture
        .packets
        .iter()
        .flat_map(|p| {
            std::iter::once(&p.reopen_anchor)
                .chain(p.object_groups.iter().map(|g| &g.reopen_anchor))
        })
        .collect();
    for anchor in &anchors {
        if anchor.anchor_class == ReopenAnchorClass::TruthfulPlaceholder {
            assert!(anchor.target_ref.is_empty());
            assert!(!anchor.placeholder_label.is_empty());
            assert!(!anchor.resolves_object);
        } else {
            assert!(anchor.target_ref.starts_with("aureline://"));
            assert!(anchor.resolves_object);
        }
    }
}

#[test]
fn fixture_digests_group_by_severity_before_chronology() {
    let fixture = load_fixture();
    for packet in fixture
        .packets
        .iter()
        .filter(|p| p.kind == ContinuityPacketKind::ShiftDigest)
    {
        for w in packet.object_groups.windows(2) {
            assert!(w[0].severity.rank() >= w[1].severity.rank());
        }
        for g in &packet.object_groups {
            for w in g.events.windows(2) {
                assert!(w[0].at <= w[1].at);
            }
        }
    }
}

#[test]
fn fixture_export_parity_holds() {
    let fixture = load_fixture();
    for packet in &fixture.packets {
        let exported = export_packet(packet);
        assert_eq!(
            exported,
            packet.export,
            "{} frozen export must equal re-exporting it",
            packet.packet.as_str()
        );
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = handoff_digest_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Operator handoff bundles & shift digests")));
    for packet in PacketClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(packet.as_str())),
            "projection must mention packet {}",
            packet.as_str()
        );
    }
}
