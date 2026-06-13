use super::*;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_evidence_chronology_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_packet_covers_every_workflow() {
    let packet = seeded_m5_evidence_chronology_packet();
    for workflow in M5EvidenceWorkflowClass::ALL {
        assert!(
            packet.rows.iter().any(|row| row.workflow_class == workflow),
            "missing workflow: {workflow:?}"
        );
    }
}

#[test]
fn every_row_carries_absolute_time_zone_source_and_imported_class() {
    let packet = seeded_m5_evidence_chronology_packet();
    for row in &packet.rows {
        assert!(!row.time_posture.absolute_timestamp.trim().is_empty());
        assert!(!row.time_posture.timezone_iana.trim().is_empty());
        assert!(!row.time_posture.utc_offset.trim().is_empty());
        assert!(!row.provenance_badges.is_empty());
        // The source and imported classes are typed enums, always present.
        let _ = row.source_class;
        let _ = row.imported_class;
    }
}

#[test]
fn lineage_steps_are_ordered_and_originator_first() {
    let packet = seeded_m5_evidence_chronology_packet();
    for row in &packet.rows {
        assert!(!row.actor_lineage.steps.is_empty());
        for (position, step) in row.actor_lineage.steps.iter().enumerate() {
            assert_eq!(step.step_index as usize, position, "{row:?}");
        }
        let originator = row.actor_lineage.originator().expect("originator exists");
        assert_eq!(originator.role, ActorLineageRole::Originator);
        assert_eq!(originator.actor_label, row.actor_label);
    }
}

#[test]
fn local_only_rows_never_claim_remote_control() {
    let packet = seeded_m5_evidence_chronology_packet();
    for row in &packet.rows {
        if row.residency.is_local_only() {
            assert!(!row.claims_remote_hold, "{row:?}");
            assert!(!row.claims_remote_export, "{row:?}");
            assert!(!row.claims_remote_delete, "{row:?}");
            assert!(row.local_only_boundary_note.is_some(), "{row:?}");
        }
    }
}

#[test]
fn projections_cover_every_row() {
    let packet = seeded_m5_evidence_chronology_packet();
    assert_eq!(packet.product_projection().len(), packet.rows.len());
    assert_eq!(packet.admin_projection().len(), packet.rows.len());
    assert_eq!(packet.support_export_projection().len(), packet.rows.len());
}

#[test]
fn admin_and_support_projections_reconstruct_lineage_and_chronology() {
    let packet = seeded_m5_evidence_chronology_packet();
    let admin = packet.admin_projection();
    let support = packet.support_export_projection();
    for ((row, a), s) in packet.rows.iter().zip(admin.iter()).zip(support.iter()) {
        // Admin reconstructs the full lineage and absolute chronology.
        assert_eq!(a.canonical_event_id, row.canonical_event_id);
        assert_eq!(a.absolute_timestamp, row.time_posture.absolute_timestamp);
        assert_eq!(a.timezone_iana, row.time_posture.timezone_iana);
        assert_eq!(a.actor_lineage, row.actor_lineage);
        assert_eq!(a.source_class, row.source_class);
        assert_eq!(a.imported_class, row.imported_class);
        // Support preserves the same chronology and lineage length.
        assert_eq!(s.canonical_event_id, row.canonical_event_id);
        assert_eq!(s.absolute_timestamp, row.time_posture.absolute_timestamp);
        assert_eq!(s.source_class, row.source_class);
        assert_eq!(s.imported_class, row.imported_class);
        assert_eq!(s.lineage_step_count, row.actor_lineage.steps.len());
        assert_eq!(s.lineage_actor_refs, row.actor_lineage.ordered_actor_refs());
    }
}

#[test]
fn local_only_remote_hold_claim_is_rejected() {
    let mut packet = seeded_m5_evidence_chronology_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.residency.is_local_only())
        .expect("a local-only row exists");
    row.claims_remote_hold = true;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5EvidenceChronologyViolation::LocalOnlyClaimsRemoteHold { .. }
    )));
}

#[test]
fn missing_absolute_timestamp_is_rejected() {
    let mut packet = seeded_m5_evidence_chronology_packet();
    packet.rows[0].time_posture.absolute_timestamp = String::new();

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5EvidenceChronologyViolation::RequiredFieldEmpty { field, .. } if field == "absolute_timestamp"
    )));
}

#[test]
fn lineage_order_gap_is_rejected() {
    let mut packet = seeded_m5_evidence_chronology_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.actor_lineage.steps.len() > 1)
        .expect("a multi-step lineage exists");
    row.actor_lineage.steps[1].step_index = 9;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5EvidenceChronologyViolation::LineageStepOrderBroken { .. }
    )));
}

#[test]
fn grammar_sentence_drift_is_rejected() {
    let mut packet = seeded_m5_evidence_chronology_packet();
    packet.rows[0].grammar_sentence = "tampered sentence".to_owned();

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5EvidenceChronologyViolation::GrammarSentenceDrift { .. }
    )));
}

#[test]
fn stale_row_without_reason_is_rejected() {
    let mut packet = seeded_m5_evidence_chronology_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            matches!(
                row.time_posture.relative_age.freshness_class,
                ChronologyFreshnessClass::Stale | ChronologyFreshnessClass::Expired
            )
        })
        .expect("a stale row exists");
    row.time_posture.relative_age.stale_reason_label = None;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5EvidenceChronologyViolation::StaleReasonMissing { .. }
    )));
}

#[test]
fn checked_in_canonical_fixture_matches_seeded_packet() {
    let fixture = repo_root()
        .join("fixtures/governance/m5_evidence_chronology_lineage/canonical_packet.yaml");
    let raw = std::fs::read_to_string(&fixture).expect("canonical fixture is readable");
    let parsed: M5EvidenceChronologyPacket =
        serde_yaml::from_str(&raw).expect("canonical fixture parses");

    assert!(
        parsed.validate().is_empty(),
        "canonical fixture must validate cleanly: {:?}",
        parsed.validate()
    );
    assert_eq!(
        parsed,
        seeded_m5_evidence_chronology_packet(),
        "canonical fixture drifted from the seeded packet; regenerate it"
    );
}
