use super::*;
use crate::m5_install_and_portability_governance::current_m5_install_portability_governance_matrix;

fn packet() -> M5CoexistenceFleetRollout {
    current_m5_coexistence_and_fleet_rollout().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_COEXISTENCE_FLEET_ROLLOUT_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_COEXISTENCE_FLEET_ROLLOUT_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_family_has_exactly_one_lane() {
    let packet = packet();
    assert_eq!(packet.lanes.len(), packet.families.len());
    for &family in &packet.families {
        assert!(
            packet.lane(family).is_some(),
            "missing lane for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_lane_ring_and_import_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_lanes_gate_consistent());
    for lane in &packet.lanes {
        assert_eq!(
            lane.published_support,
            lane.effective_support(),
            "lane {} publishes beyond the gate",
            lane.lane_id
        );
        assert_eq!(
            lane.narrow_reasons,
            lane.computed_narrow_reasons(),
            "lane {} narrow reasons diverge from the gate",
            lane.lane_id
        );
        assert_eq!(
            lane.recovery_path,
            lane.computed_recovery_path(),
            "lane {} recovery path diverges from the gate",
            lane.lane_id
        );
    }
    for ring in &packet.rings {
        assert_eq!(ring.published_support, ring.effective_support());
    }
    for import in &packet.mirror_imports {
        assert_eq!(import.published_support, import.effective_support());
    }
}

#[test]
fn lanes_never_publish_beyond_their_governance_lane() {
    let packet = packet();
    let matrix = current_m5_install_portability_governance_matrix().expect("governance parses");
    assert_eq!(packet.governance_packet_id_ref, matrix.packet_id);
    for lane in &packet.lanes {
        let gov_row = matrix
            .lane_row(lane.governs_lane)
            .expect("governance lane exists");
        assert_eq!(
            lane.governs_assurance, gov_row.published_assurance,
            "lane {} snapshots a stale governance assurance",
            lane.lane_id
        );
        assert!(
            lane.published_support.rank() <= gov_row.published_assurance.rank(),
            "lane {} claims support beyond its governance lane",
            lane.lane_id
        );
    }
}

#[test]
fn each_family_pins_to_its_canonical_lane() {
    let packet = packet();
    for lane in &packet.lanes {
        assert_eq!(
            lane.governs_lane,
            lane.family.governs_lane(),
            "lane {} is not pinned to its canonical lane",
            lane.lane_id
        );
    }
}

#[test]
fn every_lane_records_all_three_handler_surfaces() {
    let packet = packet();
    for lane in &packet.lanes {
        for surface in HandlerSurface::REQUIRED {
            assert!(
                lane.handler_precedence.iter().any(|h| h.surface == surface),
                "lane {} is missing handler surface {}",
                lane.lane_id,
                surface.as_str()
            );
        }
    }
}

#[test]
fn the_gate_admits_and_narrows_in_every_direction() {
    let packet = packet();
    let s = &packet.summary;
    assert!(s.verified_lanes >= 1, "no verified lane");
    assert!(s.bounded_lanes >= 1, "no bounded lane");
    assert!(s.retest_pending_lanes >= 1, "no retest-pending lane");
    assert!(s.withheld_lanes >= 1, "no withheld lane");
    assert!(s.downgraded_lanes >= 1, "no downgraded lane");
}

#[test]
fn every_required_ring_has_a_row() {
    let packet = packet();
    for ring in RolloutRing::REQUIRED {
        assert!(
            packet.ring_row(ring).is_some(),
            "missing ring row for {}",
            ring.as_str()
        );
    }
}

#[test]
fn at_least_one_import_proves_detached_signature_verification() {
    let packet = packet();
    assert!(packet.mirror_imports.iter().any(|m| {
        m.signature_verification == MirrorSignatureVerification::DetachedSignatureVerified
    }));
}

#[test]
fn every_required_incident_has_a_detected_drill() {
    let packet = packet();
    for incident in RolloutIncident::REQUIRED {
        let drill = packet
            .drills
            .iter()
            .find(|d| d.incident == incident)
            .unwrap_or_else(|| panic!("missing drill for incident {}", incident.as_str()));
        assert!(drill.detected, "drill {} does not detect", drill.drill_id);
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for consumer in RolloutConsumer::REQUIRED {
        assert!(
            packet.has_binding_for(consumer),
            "missing binding for consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn export_projection_preserves_published_support() {
    let packet = packet();
    let projection = packet.export_projection();
    assert!(projection.all_lanes_gate_consistent);
    assert_eq!(projection.lanes.len(), packet.lanes.len());
    for (lane, export) in packet.lanes.iter().zip(projection.lanes.iter()) {
        assert_eq!(export.published_support, lane.published_support.as_str());
        assert_eq!(export.recovery_path, lane.recovery_path.as_str());
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("export-test", "2026-06-12T00:00:00Z");
    assert!(export.is_export_safe());
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.rollout_packet_id_ref, packet.packet_id);
}

#[test]
fn detects_overstated_lane_support() {
    let mut packet = packet();
    let lane = packet
        .lanes
        .iter_mut()
        .find(|l| l.published_support == InstallAssurance::Withheld)
        .expect("a withheld lane exists");
    lane.published_support = InstallAssurance::Verified;
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5CoexistenceFleetViolation::OverstatedSupport { .. })));
}

#[test]
fn detects_last_writer_wins_handler_takeover() {
    let mut packet = packet();
    // Flip a sole-owner handler to last-writer-wins on a currently-verified lane: the gate must
    // withhold it and flag the overstated support.
    let lane = packet
        .lanes
        .iter_mut()
        .find(|l| l.is_verified())
        .expect("a verified lane exists");
    lane.handler_precedence[0].precedence = HandlerPrecedenceClass::LastWriterWins;
    assert_eq!(lane.effective_support(), InstallAssurance::Withheld);
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5CoexistenceFleetViolation::OverstatedSupport { .. })));
}

#[test]
fn detects_missing_handler_surface() {
    let mut packet = packet();
    packet.lanes[0].handler_precedence.remove(0);
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5CoexistenceFleetViolation::MissingHandlerSurface { .. })));
}
