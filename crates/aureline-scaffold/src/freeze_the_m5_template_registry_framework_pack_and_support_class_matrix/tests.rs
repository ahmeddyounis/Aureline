use super::*;

const PACKET_ID: &str = "template-framework-matrix:stable:0001";
const MATRIX_LABEL: &str = "Template Registry, Framework-Pack, and Support-Class Matrix";

fn proof_freshness() -> TemplateFrameworkMatrixProofFreshness {
    TemplateFrameworkMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> TemplateFrameworkMatrixPacket {
    frozen_template_framework_matrix(
        PACKET_ID.to_owned(),
        MATRIX_LABEL.to_owned(),
        "2026-06-07T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

#[test]
fn template_framework_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_matrix_covers_every_lane() {
    let present: Vec<TemplateFrameworkLane> =
        packet().lane_rows.iter().map(|row| row.lane).collect();
    for lane in TemplateFrameworkLane::ALL {
        assert!(present.contains(&lane), "missing lane {}", lane.as_str());
    }
}

#[test]
fn missing_lane_fails_validation() {
    let mut packet = packet();
    packet
        .lane_rows
        .retain(|row| row.lane != TemplateFrameworkLane::FrameworkPack);
    assert!(packet
        .validate()
        .contains(&TemplateFrameworkMatrixViolation::RequiredLaneMissing));
}

#[test]
fn stable_lane_missing_evidence_fails() {
    let mut packet = packet();
    packet.lane_rows[0].required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&TemplateFrameworkMatrixViolation::StableLaneMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.lane_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&TemplateFrameworkMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.lane_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&TemplateFrameworkMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&TemplateFrameworkMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.heuristic_never_presented_as_exact = false;
    assert!(packet
        .validate()
        .contains(&TemplateFrameworkMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .diff_preview_shows_authored_generated_truth = false;
    assert!(packet
        .validate()
        .contains(&TemplateFrameworkMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&TemplateFrameworkMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn stale_proof_narrows_a_stable_lane() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[TemplateFrameworkLaneObservation {
        lane: TemplateFrameworkLane::SignedTemplateRegistry,
        evidence_valid: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let row = packet
        .lane_rows
        .iter()
        .find(|row| row.lane == TemplateFrameworkLane::SignedTemplateRegistry)
        .unwrap();
    assert_eq!(row.qualification, TemplateFrameworkQualificationClass::Beta);
    assert_eq!(
        row.support_class,
        TemplateFrameworkSupportClass::NarrowedBelowStable
    );
    // The narrowed packet remains a valid, export-safe matrix.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn invalid_evidence_holds_a_lane() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[TemplateFrameworkLaneObservation {
        lane: TemplateFrameworkLane::ArchetypeHealthBundle,
        evidence_valid: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let row = packet
        .lane_rows
        .iter()
        .find(|row| row.lane == TemplateFrameworkLane::ArchetypeHealthBundle)
        .unwrap();
    assert_eq!(row.qualification, TemplateFrameworkQualificationClass::Held);
    assert_eq!(
        row.evidence_requirement,
        TemplateFrameworkEvidenceRequirement::NotApplicable
    );
    assert!(row.required_evidence_packet_refs.is_empty());
    // A held lane carries no Stable evidence obligation, so the packet stays valid.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn markdown_summary_lists_every_lane_with_support_class() {
    let summary = packet().render_markdown_summary();
    for lane in TemplateFrameworkLane::ALL {
        assert!(
            summary.contains(lane.as_str()),
            "summary missing lane {}",
            lane.as_str()
        );
    }
    assert!(summary.contains("community_supported"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_template_framework_matrix_export()
        .expect("checked template/framework matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_template_framework_matrix_export()
        .expect("checked template/framework matrix export validates");
    let built = packet();
    assert_eq!(checked, built);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/framework_pack_support_class_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/archetype_health_held.json"
        )),
    ] {
        let packet: TemplateFrameworkMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
