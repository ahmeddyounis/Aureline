use std::collections::BTreeSet;

use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_artifact_save_truth_packet();
    validate_artifact_save_truth_packet(&packet).expect("seeded packet must validate");
}

#[test]
fn seeded_fixtures_validate() {
    let packet = seeded_artifact_save_truth_packet();
    let fixtures = seeded_artifact_save_truth_fixtures();
    for fixture in &fixtures {
        validate_artifact_save_truth_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn every_surface_row_has_exactly_one_fixture() {
    let packet = seeded_artifact_save_truth_packet();
    let fixtures = seeded_artifact_save_truth_fixtures();
    let row_ids: BTreeSet<_> = packet.rows.iter().map(|row| row.row_id.as_str()).collect();
    let fixture_rows: BTreeSet<_> = fixtures
        .iter()
        .map(|fixture| fixture.expected_row_id.as_str())
        .collect();
    assert_eq!(row_ids, fixture_rows);
}

#[test]
fn required_evidence_classes_are_covered() {
    let fixtures = seeded_artifact_save_truth_fixtures();
    let evidence: BTreeSet<_> = fixtures
        .iter()
        .flat_map(|fixture| fixture.evidence_classes.iter().copied())
        .collect();
    for required in [
        SaveTruthEvidenceClass::LossyDecodeRisk,
        SaveTruthEvidenceClass::MetadataPreservationDisclosure,
        SaveTruthEvidenceClass::ExecuteBitRetention,
        SaveTruthEvidenceClass::ExportOrRegenerateNotExactSave,
        SaveTruthEvidenceClass::LogicalTargetAmbiguityDisclosure,
        SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
    ] {
        assert!(
            evidence.contains(&required),
            "fixture corpus must cover evidence class {}",
            required.as_str()
        );
    }
}

#[test]
fn packet_json_round_trips_after_artifact_generation() {
    let packet = seeded_artifact_save_truth_packet();
    let json = serde_json::to_string_pretty(&packet).expect("packet serializes");
    let reparsed: ArtifactSaveTruthPacket = serde_json::from_str(&json).expect("packet reparses");
    assert_eq!(reparsed, packet);
}
