use super::*;

fn packet() -> OrientationAidPacket {
    seeded_orientation_aid_packet()
}

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn fixture_packet_validates() {
    let violations = fixture_orientation_aid_packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected fixture violations: {violations:?}"
    );
}

#[test]
fn seeded_packet_covers_required_surface_kinds() {
    let kinds = packet().represented_surface_kinds();
    for required in REQUIRED_SURFACE_KINDS {
        assert!(
            kinds.contains(&required),
            "missing surface kind {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_required_object_classes() {
    let classes = packet().represented_object_classes();
    for required in REQUIRED_OBJECT_CLASSES {
        assert!(
            classes.contains(&required),
            "missing object class {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_aid_kind() {
    let kinds = packet().represented_aid_kinds();
    for required in OrientationAidKind::ALL {
        assert!(
            kinds.contains(&required),
            "missing aid kind {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_disclosure_class() {
    let resolutions = packet().represented_resolutions();
    for required in OrientationDisclosureClass::ALL {
        assert!(
            resolutions.contains(&required),
            "missing disclosure class {}",
            required.as_str()
        );
    }
}

#[test]
fn disclosure_ranks_are_strictly_ordered() {
    let ranks: Vec<u8> = OrientationDisclosureClass::ALL
        .iter()
        .map(|class| class.disclosure_rank())
        .collect();
    for window in ranks.windows(2) {
        assert!(window[0] < window[1], "ranks must be strictly increasing");
    }
}

#[test]
fn high_cardinality_aid_cannot_flatten_to_fully_active() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "orientation:notebook:0001")
        .expect("notebook record");
    assert!(record.must_not_flatten());
    // Force the high-cardinality aid back onto the flat fully-active lane.
    record.resolution = OrientationDisclosureClass::AidFullyActive;
    record.aid_posture = OrientationAidClass::FullOrientationAids;
    record.count_summary_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&OrientationAidViolation::SilentFlatteningOfAid));
}

#[test]
fn constrained_viewport_requires_reduced_detail_floor() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "orientation:data-api:0001")
        .expect("data/api record");
    assert_eq!(
        record.required_floor_rank(),
        OrientationDisclosureClass::ReducedDetailDisclosed.disclosure_rank()
    );
    // A mere count summary is below the constrained-viewport floor.
    record.resolution = OrientationDisclosureClass::CountSummaryPreserved;
    record.reduced_detail_label = None;
    record.count_summary_label = Some("Some count".to_owned());
    record.aid_posture = OrientationDisclosureClass::CountSummaryPreserved.canonical_aid_class();
    let violations = packet.validate();
    assert!(violations.contains(&OrientationAidViolation::ResolutionBelowRequiredFloor));
}

#[test]
fn limited_profile_requires_unavailable_disclosure() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "orientation:runtime:0001")
        .expect("runtime record");
    assert_eq!(
        record.required_floor_rank(),
        OrientationDisclosureClass::UnavailableDisclosed.disclosure_rank()
    );
}

#[test]
fn silent_removal_posture_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "orientation:runtime:0001")
        .expect("runtime record");
    // Collapse the aid to nothing instead of disclosing it.
    record.aid_posture = OrientationAidClass::OrientationAidsAbsentDowngraded;
    let violations = packet.validate();
    assert!(violations.contains(&OrientationAidViolation::AidPostureInconsistent));
}

#[test]
fn stale_marker_flag_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "orientation:review:0001")
        .expect("review record");
    record.stale_markers_shown = true;
    let violations = packet.validate();
    assert!(violations.contains(&OrientationAidViolation::RawBoundaryMaterialPresent));
}

#[test]
fn provider_record_cannot_read_as_local() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "orientation:companion:0001")
        .expect("companion record");
    assert!(record.provider_or_imported());
    record.verification.proof_currency = AxisProofCurrency::VerifiedCurrent;
    let violations = packet.validate();
    assert!(violations.contains(&OrientationAidViolation::ImportedReadsAsLocal));
}

#[test]
fn stale_proof_forces_aid_off_flat_lane() {
    let packet = fixture_orientation_aid_packet();
    let record = packet
        .record("orientation:editor-core:stale-proof:0001")
        .expect("stale-proof drill record");
    assert!(record.must_not_flatten());
    assert!(record
        .fired_triggers
        .contains(&OrientationContractTrigger::StaleOrMissingOrientationProof));
    assert_eq!(
        record.resolution,
        OrientationDisclosureClass::DegradedDisclosed
    );
}

#[test]
fn generic_degrade_label_is_rejected() {
    let mut packet = packet();
    let record = packet
        .records
        .iter_mut()
        .find(|record| record.record_id == "orientation:review:0001")
        .expect("review record");
    record.degrade_reason_label = Some("degraded".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&OrientationAidViolation::ResolutionDetailInconsistent));
}

#[test]
fn missing_aid_kind_coverage_is_flagged() {
    let mut packet = packet();
    // Drop every overview-ruler record.
    packet
        .records
        .retain(|record| record.aid_kind != OrientationAidKind::OverviewRuler);
    let violations = packet.validate();
    assert!(violations.contains(&OrientationAidViolation::AidKindCoverageMissing));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: OrientationAidPacket = serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_records_and_disclosures() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Orientation Aids"));
    assert!(summary.contains("multi_cursor"));
    assert!(summary.contains("Count summary:"));
    assert!(summary.contains("Identity alignment:"));
    assert!(summary.contains("Reduced detail:"));
    assert!(summary.contains("Motion reduced:"));
    assert!(summary.contains("Degraded:"));
    assert!(summary.contains("Unavailable:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked =
        current_orientation_aid_export().expect("checked orientation aid export validates");
    assert_eq!(checked, packet());
}
