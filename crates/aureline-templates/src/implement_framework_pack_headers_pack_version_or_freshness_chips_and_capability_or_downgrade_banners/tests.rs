use super::*;

const PACKET_ID: &str = "framework-pack:stable:0001";
const PACKET_LABEL: &str =
    "Framework-Pack Headers, Pack Version/Freshness Chips, and Capability/Downgrade Banners";

const CLEAN_ROW: &str = "framework-pack-row:rust_axum.first_party:2026.05";
const PARTIAL_ROW: &str = "framework-pack-row:node_nest.community_update:2026.05";
const BRIDGE_ROW: &str = "framework-pack-row:py_flask.bridge_heuristic:2026.04";
const UNKNOWN_ROW: &str = "framework-pack-row:mirror.unknown_provenance:2026.03";

fn proof_freshness() -> FrameworkPackProofFreshness {
    FrameworkPackProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> FrameworkPackPacket {
    canonical_framework_pack(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        "2026-06-07T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(packet: &'a FrameworkPackPacket, row_id: &str) -> &'a FrameworkPackRow {
    packet
        .rows
        .iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing row {row_id}"))
}

#[test]
fn framework_pack_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_rows_cover_provenance_and_capability_spectrum() {
    let packet = packet();
    let provenance: Vec<PackProvenanceClass> =
        packet.rows.iter().map(|row| row.provenance_class).collect();
    for required in [
        PackProvenanceClass::FirstParty,
        PackProvenanceClass::Community,
        PackProvenanceClass::BridgedFromOtherFramework,
        PackProvenanceClass::ProvenanceUnknown,
    ] {
        assert!(
            provenance.contains(&required),
            "missing provenance {}",
            required.as_str()
        );
    }
    let capability: Vec<PackCapabilityClass> =
        packet.rows.iter().map(|row| row.capability_class).collect();
    assert!(capability.contains(&PackCapabilityClass::FullCapability));
    assert!(capability.contains(&PackCapabilityClass::PartialCapability));
    assert!(capability.contains(&PackCapabilityClass::HeuristicCapability));
    assert!(capability.contains(&PackCapabilityClass::CapabilityUnknown));
}

#[test]
fn provenance_unknown_row_is_blocked_in_canonical_packet() {
    let packet = packet();
    let unknown = row(&packet, UNKNOWN_ROW);
    assert!(unknown.provenance_class.is_unknown());
    assert_eq!(
        unknown.downgrade_banner_class,
        PackDowngradeBannerClass::ProvenanceUnknownBanner
    );
    assert!(unknown.is_blocked());
    assert!(!unknown.admitted_for_offer);
}

#[test]
fn bridge_row_discloses_known_issue_banner_and_is_held() {
    let packet = packet();
    let bridge = row(&packet, BRIDGE_ROW);
    assert!(bridge.support_class.requires_disclosure());
    assert!(!bridge.known_issue_refs.is_empty());
    assert!(bridge.downgrade_banner_class.is_present());
    assert!(bridge
        .downgrade_triggers
        .contains(&FrameworkPackDowngradeTrigger::HeuristicMappingDisclosed));
    assert!(!bridge.admitted_for_offer);
}

#[test]
fn partial_capability_row_is_offered_with_a_banner() {
    let packet = packet();
    let partial = row(&packet, PARTIAL_ROW);
    assert_eq!(
        partial.capability_class,
        PackCapabilityClass::PartialCapability
    );
    assert!(partial.capability_class.requires_banner());
    assert!(partial.downgrade_banner_class.is_present());
    assert!(!partial.is_blocked());
    assert!(partial.admitted_for_offer);
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::RowsEmpty));
}

#[test]
fn non_full_capability_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == PARTIAL_ROW)
        .unwrap()
        .downgrade_banner_class = PackDowngradeBannerClass::NoBanner;
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::CapabilityBannerMissing));
}

#[test]
fn bridge_row_without_disclosure_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == BRIDGE_ROW)
        .unwrap()
        .known_issue_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::BridgeBehaviorUndisclosed));
}

#[test]
fn provenance_unknown_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == UNKNOWN_ROW)
        .unwrap()
        .downgrade_banner_class = PackDowngradeBannerClass::FreshnessBanner;
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::ProvenanceUnknownBannerMissing));
}

#[test]
fn blocked_row_admitted_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == UNKNOWN_ROW)
        .unwrap()
        .admitted_for_offer = true;
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::BlockedOfferAdmitted));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet
        .review
        .bridge_or_heuristic_never_presented_as_first_party = false;
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.blocked_rows_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&FrameworkPackViolation::ProofFreshnessIncomplete));
}

#[test]
fn unknown_provenance_blocks_a_row() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkPackRowObservation {
        row_id: CLEAN_ROW.to_owned(),
        provenance_resolved: false,
        pack_version_current: true,
        freshness_fresh: true,
        capability_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let clean = row(&packet, CLEAN_ROW);
    assert_eq!(
        clean.provenance_class,
        PackProvenanceClass::ProvenanceUnknown
    );
    assert_eq!(
        clean.capability_class,
        PackCapabilityClass::CapabilityUnknown
    );
    assert_eq!(
        clean.downgrade_banner_class,
        PackDowngradeBannerClass::ProvenanceUnknownBanner
    );
    assert!(!clean.admitted_for_offer);
    assert!(clean
        .downgrade_triggers
        .contains(&FrameworkPackDowngradeTrigger::ProvenanceUnknown));
    // A blocked-but-labeled row keeps the export valid.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn yanked_pack_version_withdraws_offer() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkPackRowObservation {
        row_id: CLEAN_ROW.to_owned(),
        provenance_resolved: true,
        pack_version_current: false,
        freshness_fresh: true,
        capability_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let clean = row(&packet, CLEAN_ROW);
    assert_eq!(clean.freshness_class, PackFreshnessClass::Stale);
    assert_eq!(
        clean.downgrade_banner_class,
        PackDowngradeBannerClass::FreshnessBanner
    );
    assert!(!clean.admitted_for_offer);
    assert!(clean
        .downgrade_triggers
        .contains(&FrameworkPackDowngradeTrigger::PackVersionYanked));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_freshness_raises_banner_and_withdraws_offer() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkPackRowObservation {
        row_id: CLEAN_ROW.to_owned(),
        provenance_resolved: true,
        pack_version_current: true,
        freshness_fresh: false,
        capability_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let clean = row(&packet, CLEAN_ROW);
    assert_eq!(clean.freshness_class, PackFreshnessClass::Stale);
    assert!(clean.downgrade_banner_class.is_present());
    assert!(!clean.admitted_for_offer);
    assert!(clean
        .downgrade_triggers
        .contains(&FrameworkPackDowngradeTrigger::FreshnessStale));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unverified_capability_degrades_and_withdraws_offer() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkPackRowObservation {
        row_id: PARTIAL_ROW.to_owned(),
        provenance_resolved: true,
        pack_version_current: true,
        freshness_fresh: true,
        capability_verified: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let partial = row(&packet, PARTIAL_ROW);
    assert_eq!(
        partial.capability_class,
        PackCapabilityClass::CapabilityDegraded
    );
    assert!(partial.downgrade_banner_class.is_present());
    assert!(!partial.admitted_for_offer);
    assert!(partial
        .downgrade_triggers
        .contains(&FrameworkPackDowngradeTrigger::CapabilityDegraded));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_proof_withholds_offer() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkPackRowObservation {
        row_id: CLEAN_ROW.to_owned(),
        provenance_resolved: true,
        pack_version_current: true,
        freshness_fresh: true,
        capability_verified: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let clean = row(&packet, CLEAN_ROW);
    assert!(!clean.admitted_for_offer);
    assert!(clean
        .downgrade_triggers
        .contains(&FrameworkPackDowngradeTrigger::ProofStale));
}

#[test]
fn markdown_summary_lists_every_row() {
    let summary = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(
            summary.contains(&row.pack_label),
            "summary missing pack {}",
            row.pack_label
        );
    }
    assert!(summary.contains("provenance_unknown_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_framework_pack_export().expect("checked framework-pack export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_framework_pack_export().expect("checked framework-pack export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/provenance_unknown_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/capability_degraded_withheld.json"
        )),
    ] {
        let packet: FrameworkPackPacket =
            serde_json::from_str(raw).expect("fixture parses as framework-pack packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
