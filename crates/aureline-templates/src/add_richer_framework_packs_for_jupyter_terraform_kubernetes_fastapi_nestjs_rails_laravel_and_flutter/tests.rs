use super::*;

const PACKET_ID: &str = "richer-framework-pack:stable:0001";
const PACKET_LABEL: &str =
    "Richer Framework Packs: Jupyter Adjacency, Terraform/Kubernetes, FastAPI, Nest, Rails, Laravel, and Flutter";

const JUPYTER_ROW: &str = "framework-lane-row:jupyter_adjacency.partner_bridge:2026.06";
const TERRAFORM_ROW: &str = "framework-lane-row:terraform.first_party:2026.06";
const KUBERNETES_ROW: &str = "framework-lane-row:kubernetes.first_party_update:2026.06";
const NEST_ROW: &str = "framework-lane-row:nest.community_update:2026.05";
const LARAVEL_ROW: &str = "framework-lane-row:laravel.heuristic_degraded:2026.04";
const FLUTTER_ROW: &str = "framework-lane-row:flutter.mirror_unknown:2026.03";

fn proof_freshness() -> FrameworkLaneProofFreshness {
    FrameworkLaneProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> FrameworkLanePacket {
    canonical_richer_framework_pack(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        "2026-06-09T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(packet: &'a FrameworkLanePacket, row_id: &str) -> &'a FrameworkLaneRow {
    packet
        .rows
        .iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing row {row_id}"))
}

#[test]
fn richer_framework_pack_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_rows_cover_all_named_lanes() {
    let packet = packet();
    let domains: Vec<LaneDomainClass> = packet
        .rows
        .iter()
        .map(|row| row.lane_domain_class)
        .collect();
    for required in [
        LaneDomainClass::NotebookAdjacency,
        LaneDomainClass::InfrastructureProvisioning,
        LaneDomainClass::WebApiService,
        LaneDomainClass::MobileApp,
    ] {
        assert!(
            domains.contains(&required),
            "missing lane domain {}",
            required.as_str()
        );
    }
    // The eight named lanes are all present as labeled frameworks.
    let frameworks: Vec<&str> = packet
        .rows
        .iter()
        .map(|row| row.framework_label.as_str())
        .collect();
    for required in [
        "Jupyter",
        "Terraform",
        "Kubernetes",
        "FastAPI",
        "Nest",
        "Rails",
        "Laravel",
        "Flutter",
    ] {
        assert!(
            frameworks.contains(&required),
            "missing framework {required}"
        );
    }
}

#[test]
fn canonical_rows_cover_provenance_capability_origin_and_health_spectrum() {
    let packet = packet();
    let provenance: Vec<LanePackProvenanceClass> =
        packet.rows.iter().map(|row| row.provenance_class).collect();
    for required in [
        LanePackProvenanceClass::FirstParty,
        LanePackProvenanceClass::PartnerCertified,
        LanePackProvenanceClass::Community,
        LanePackProvenanceClass::ProvenanceUnknown,
    ] {
        assert!(
            provenance.contains(&required),
            "missing provenance {}",
            required.as_str()
        );
    }
    let capability: Vec<LanePackCapabilityClass> =
        packet.rows.iter().map(|row| row.capability_class).collect();
    assert!(capability.contains(&LanePackCapabilityClass::FullCapability));
    assert!(capability.contains(&LanePackCapabilityClass::PartialCapability));
    assert!(capability.contains(&LanePackCapabilityClass::HeuristicCapability));
    assert!(capability.contains(&LanePackCapabilityClass::CapabilityUnknown));

    let origin: Vec<LanePackOriginTruthClass> = packet
        .rows
        .iter()
        .map(|row| row.origin_truth_class)
        .collect();
    assert!(origin.contains(&LanePackOriginTruthClass::GeneratedManaged));
    assert!(origin.contains(&LanePackOriginTruthClass::BridgedAdjacent));
    assert!(origin.contains(&LanePackOriginTruthClass::RuntimeObserved));
    assert!(origin.contains(&LanePackOriginTruthClass::OriginUnknown));

    let health: Vec<LaneArchetypeHealthClass> = packet
        .rows
        .iter()
        .map(|row| row.archetype_health_class)
        .collect();
    assert!(health.contains(&LaneArchetypeHealthClass::CertifiedHealthy));
    assert!(health.contains(&LaneArchetypeHealthClass::HealthyUncertified));
    assert!(health.contains(&LaneArchetypeHealthClass::Degraded));
    assert!(health.contains(&LaneArchetypeHealthClass::HealthUnknown));
}

#[test]
fn jupyter_adjacency_discloses_bridge_and_is_offered_with_banner() {
    let packet = packet();
    let jupyter = row(&packet, JUPYTER_ROW);
    assert_eq!(
        jupyter.lane_domain_class,
        LaneDomainClass::NotebookAdjacency
    );
    assert!(jupyter.support_class.requires_disclosure());
    assert_eq!(
        jupyter.origin_truth_class,
        LanePackOriginTruthClass::BridgedAdjacent
    );
    assert!(jupyter.downgrade_banner_class.is_present());
    assert!(jupyter
        .downgrade_triggers
        .contains(&FrameworkLaneDowngradeTrigger::BridgeBehaviorDisclosed));
    // A disclosed bridge is allowed to be offered behind its banner.
    assert!(jupyter.admitted_for_offer);
    assert!(!jupyter.is_blocked());
}

#[test]
fn laravel_heuristic_is_held_with_degraded_health() {
    let packet = packet();
    let laravel = row(&packet, LARAVEL_ROW);
    assert_eq!(
        laravel.capability_class,
        LanePackCapabilityClass::HeuristicCapability
    );
    assert_eq!(
        laravel.archetype_health_class,
        LaneArchetypeHealthClass::Degraded
    );
    assert!(!laravel.known_issue_refs.is_empty());
    assert!(laravel.downgrade_banner_class.is_present());
    assert!(laravel
        .downgrade_triggers
        .contains(&FrameworkLaneDowngradeTrigger::HeuristicMappingDisclosed));
    assert!(!laravel.admitted_for_offer);
}

#[test]
fn flutter_mirror_is_provenance_unknown_and_blocked() {
    let packet = packet();
    let flutter = row(&packet, FLUTTER_ROW);
    assert!(flutter.provenance_class.is_unknown());
    assert_eq!(
        flutter.downgrade_banner_class,
        LanePackDowngradeBannerClass::ProvenanceUnknownBanner
    );
    assert_eq!(
        flutter.origin_truth_class,
        LanePackOriginTruthClass::OriginUnknown
    );
    assert!(flutter.is_blocked());
    assert!(!flutter.admitted_for_offer);
}

#[test]
fn first_party_infra_and_api_packs_are_offered_cleanly() {
    let packet = packet();
    for clean in [TERRAFORM_ROW, KUBERNETES_ROW] {
        let row = row(&packet, clean);
        assert_eq!(row.provenance_class, LanePackProvenanceClass::FirstParty);
        assert_eq!(
            row.capability_class,
            LanePackCapabilityClass::FullCapability
        );
        assert!(row.admitted_for_offer);
        assert!(!row.is_blocked());
    }
}

#[test]
fn rows_for_domain_filters_by_lane() {
    let packet = packet();
    let web: Vec<&str> = packet
        .rows_for_domain(LaneDomainClass::WebApiService)
        .map(|row| row.framework_label.as_str())
        .collect();
    assert!(web.contains(&"FastAPI"));
    assert!(web.contains(&"Nest"));
    assert!(web.contains(&"Rails"));
    assert!(web.contains(&"Laravel"));
    assert!(!web.contains(&"Terraform"));
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::RowsEmpty));
}

#[test]
fn non_full_capability_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == NEST_ROW)
        .unwrap()
        .downgrade_banner_class = LanePackDowngradeBannerClass::NoBanner;
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::CapabilityBannerMissing));
}

#[test]
fn degraded_health_without_banner_fails() {
    let mut packet = packet();
    let laravel = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == LARAVEL_ROW)
        .unwrap();
    // Resolve the capability and support so only the health invariant trips.
    laravel.capability_class = LanePackCapabilityClass::FullCapability;
    laravel.support_class = LanePackSupportClass::CommunitySupported;
    laravel.freshness_class = LanePackFreshnessClass::Fresh;
    laravel.downgrade_banner_class = LanePackDowngradeBannerClass::NoBanner;
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::HealthBannerMissing));
}

#[test]
fn origin_unknown_without_banner_fails() {
    let mut packet = packet();
    let flutter = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == FLUTTER_ROW)
        .unwrap();
    flutter.provenance_class = LanePackProvenanceClass::Mirror;
    flutter.freshness_class = LanePackFreshnessClass::Aging;
    flutter.capability_class = LanePackCapabilityClass::PartialCapability;
    flutter.archetype_health_class = LaneArchetypeHealthClass::HealthyUncertified;
    flutter.downgrade_banner_class = LanePackDowngradeBannerClass::NoBanner;
    let violations = packet.validate();
    assert!(violations.contains(&FrameworkLaneViolation::OriginTruthBannerMissing));
}

#[test]
fn bridge_row_without_disclosure_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == JUPYTER_ROW)
        .unwrap()
        .known_issue_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::BridgeBehaviorUndisclosed));
}

#[test]
fn provenance_unknown_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == FLUTTER_ROW)
        .unwrap()
        .downgrade_banner_class = LanePackDowngradeBannerClass::FreshnessBanner;
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::ProvenanceUnknownBannerMissing));
}

#[test]
fn blocked_row_admitted_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == FLUTTER_ROW)
        .unwrap()
        .admitted_for_offer = true;
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::BlockedOfferAdmitted));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet
        .review
        .bridge_or_heuristic_never_presented_as_first_party = false;
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.blocked_rows_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&FrameworkLaneViolation::ProofFreshnessIncomplete));
}

#[test]
fn unknown_provenance_blocks_a_row() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkLaneRowObservation {
        row_id: TERRAFORM_ROW.to_owned(),
        provenance_resolved: false,
        pack_version_current: true,
        generator_version_current: true,
        freshness_fresh: true,
        capability_verified: true,
        archetype_health_ok: true,
        origin_truth_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let terraform = row(&packet, TERRAFORM_ROW);
    assert_eq!(
        terraform.provenance_class,
        LanePackProvenanceClass::ProvenanceUnknown
    );
    assert_eq!(
        terraform.origin_truth_class,
        LanePackOriginTruthClass::OriginUnknown
    );
    assert_eq!(
        terraform.archetype_health_class,
        LaneArchetypeHealthClass::HealthUnknown
    );
    assert_eq!(
        terraform.downgrade_banner_class,
        LanePackDowngradeBannerClass::ProvenanceUnknownBanner
    );
    assert!(!terraform.admitted_for_offer);
    assert!(terraform
        .downgrade_triggers
        .contains(&FrameworkLaneDowngradeTrigger::ProvenanceUnknown));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn yanked_generator_version_withdraws_offer() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkLaneRowObservation {
        row_id: TERRAFORM_ROW.to_owned(),
        provenance_resolved: true,
        pack_version_current: true,
        generator_version_current: false,
        freshness_fresh: true,
        capability_verified: true,
        archetype_health_ok: true,
        origin_truth_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let terraform = row(&packet, TERRAFORM_ROW);
    assert_eq!(terraform.freshness_class, LanePackFreshnessClass::Stale);
    assert_eq!(
        terraform.downgrade_banner_class,
        LanePackDowngradeBannerClass::FreshnessBanner
    );
    assert!(!terraform.admitted_for_offer);
    assert!(terraform
        .downgrade_triggers
        .contains(&FrameworkLaneDowngradeTrigger::GeneratorVersionYanked));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn failed_health_check_degrades_and_withdraws_offer() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkLaneRowObservation {
        row_id: TERRAFORM_ROW.to_owned(),
        provenance_resolved: true,
        pack_version_current: true,
        generator_version_current: true,
        freshness_fresh: true,
        capability_verified: true,
        archetype_health_ok: false,
        origin_truth_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let terraform = row(&packet, TERRAFORM_ROW);
    assert_eq!(
        terraform.archetype_health_class,
        LaneArchetypeHealthClass::Degraded
    );
    assert!(terraform.downgrade_banner_class.is_present());
    assert!(!terraform.admitted_for_offer);
    assert!(terraform
        .downgrade_triggers
        .contains(&FrameworkLaneDowngradeTrigger::ArchetypeHealthDegraded));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unverified_origin_truth_narrows_and_withdraws_offer() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkLaneRowObservation {
        row_id: NEST_ROW.to_owned(),
        provenance_resolved: true,
        pack_version_current: true,
        generator_version_current: true,
        freshness_fresh: true,
        capability_verified: true,
        archetype_health_ok: true,
        origin_truth_verified: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let nest = row(&packet, NEST_ROW);
    assert_eq!(
        nest.origin_truth_class,
        LanePackOriginTruthClass::OriginUnknown
    );
    assert!(nest.downgrade_banner_class.is_present());
    assert!(!nest.admitted_for_offer);
    assert!(nest
        .downgrade_triggers
        .contains(&FrameworkLaneDowngradeTrigger::OriginTruthUnverified));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_proof_withholds_offer() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[FrameworkLaneRowObservation {
        row_id: TERRAFORM_ROW.to_owned(),
        provenance_resolved: true,
        pack_version_current: true,
        generator_version_current: true,
        freshness_fresh: true,
        capability_verified: true,
        archetype_health_ok: true,
        origin_truth_verified: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let terraform = row(&packet, TERRAFORM_ROW);
    assert!(!terraform.admitted_for_offer);
    assert!(terraform
        .downgrade_triggers
        .contains(&FrameworkLaneDowngradeTrigger::ProofStale));
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
    let packet = current_richer_framework_pack_export()
        .expect("checked richer framework-pack export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_richer_framework_pack_export()
        .expect("checked richer framework-pack export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/health_degraded_withheld.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/generator_version_yanked_blocked.json"
        )),
    ] {
        let packet: FrameworkLanePacket =
            serde_json::from_str(raw).expect("fixture parses as richer framework-pack packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
