use super::*;

const PACKET_ID: &str = "app-topology:stable:0001";
const PACKET_LABEL: &str =
    "Route Explorers, Component Trees, and App-Topology Views with Authored/Generated/Runtime-Only Truth";

const AUTHORED_ROUTE: &str = "app-topology-row:route.authored.dashboard:2026.06";
const GENERATED_ROUTE: &str = "app-topology-row:route.generated.users_api:2026.06";
const ZONE_COMPONENT: &str = "app-topology-row:component.authored_in_zone.user_card:2026.06";
const HEURISTIC_COMPONENT: &str = "app-topology-row:component.heuristic.legacy_widget:2026.05";
const RUNTIME_NODE: &str = "app-topology-row:topology.runtime_only.webhook:2026.06";
const UNKNOWN_NODE: &str = "app-topology-row:topology.unknown_origin.orphan:2026.04";

fn proof_freshness() -> AppTopologyProofFreshness {
    AppTopologyProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> AppTopologyPacket {
    canonical_app_topology(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        "2026-06-08T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(packet: &'a AppTopologyPacket, row_id: &str) -> &'a AppTopologyRow {
    packet
        .rows
        .iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing row {row_id}"))
}

#[test]
fn app_topology_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_rows_cover_origin_and_view_spectrum() {
    let packet = packet();
    let origin: Vec<NodeOriginClass> = packet.rows.iter().map(|row| row.origin_class).collect();
    for required in [
        NodeOriginClass::Authored,
        NodeOriginClass::Generated,
        NodeOriginClass::AuthoredInGeneratedZone,
        NodeOriginClass::RuntimeOnly,
        NodeOriginClass::OriginUnknown,
    ] {
        assert!(
            origin.contains(&required),
            "missing origin {}",
            required.as_str()
        );
    }
    let views: Vec<TopologyViewKind> = packet.rows.iter().map(|row| row.view_kind).collect();
    assert!(views.contains(&TopologyViewKind::RouteExplorer));
    assert!(views.contains(&TopologyViewKind::ComponentTree));
    assert!(views.contains(&TopologyViewKind::AppTopology));
}

#[test]
fn generated_route_shows_generator_version() {
    let packet = packet();
    let generated = row(&packet, GENERATED_ROUTE);
    assert!(generated.origin_class.is_generated());
    assert_eq!(generated.generator_version, "1.8.0");
}

#[test]
fn authored_in_generated_zone_is_shown_with_managed_zone_honesty() {
    let packet = packet();
    let zone = row(&packet, ZONE_COMPONENT);
    assert_eq!(zone.origin_class, NodeOriginClass::AuthoredInGeneratedZone);
    assert!(zone.origin_class.is_generated());
    assert!(zone.admitted_for_display);
    assert!(!zone.is_blocked());
}

#[test]
fn runtime_only_node_is_shown_but_disclosed() {
    let packet = packet();
    let runtime = row(&packet, RUNTIME_NODE);
    assert!(runtime.origin_class.is_runtime_only());
    assert!(runtime
        .downgrade_triggers
        .contains(&TopologyDowngradeTrigger::RuntimeOnlyDisclosed));
    assert!(runtime.admitted_for_display);
}

#[test]
fn heuristic_node_discloses_known_issue_banner_and_is_held() {
    let packet = packet();
    let heuristic = row(&packet, HEURISTIC_COMPONENT);
    assert!(heuristic.support_class.requires_disclosure());
    assert!(!heuristic.known_issue_refs.is_empty());
    assert!(heuristic.downgrade_banner_class.is_present());
    assert!(heuristic
        .downgrade_triggers
        .contains(&TopologyDowngradeTrigger::HeuristicMappingDisclosed));
    assert!(!heuristic.admitted_for_display);
}

#[test]
fn origin_unknown_node_is_blocked_in_canonical_packet() {
    let packet = packet();
    let unknown = row(&packet, UNKNOWN_NODE);
    assert!(unknown.origin_class.is_unknown());
    assert_eq!(
        unknown.downgrade_banner_class,
        TopologyDowngradeBannerClass::OriginUnknownBanner
    );
    assert!(unknown.is_blocked());
    assert!(!unknown.admitted_for_display);
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet.validate().contains(&AppTopologyViolation::RowsEmpty));
}

#[test]
fn non_exact_derivation_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == HEURISTIC_COMPONENT)
        .unwrap()
        .downgrade_banner_class = TopologyDowngradeBannerClass::NoBanner;
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::DerivationBannerMissing));
}

#[test]
fn heuristic_node_without_disclosure_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == HEURISTIC_COMPONENT)
        .unwrap()
        .known_issue_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::BridgeBehaviorUndisclosed));
}

#[test]
fn runtime_only_without_disclosure_trigger_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == RUNTIME_NODE)
        .unwrap()
        .downgrade_triggers
        .retain(|trigger| *trigger != TopologyDowngradeTrigger::RuntimeOnlyDisclosed);
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::RuntimeOnlyUndisclosed));
}

#[test]
fn origin_unknown_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == UNKNOWN_NODE)
        .unwrap()
        .downgrade_banner_class = TopologyDowngradeBannerClass::FreshnessBanner;
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::OriginUnknownBannerMissing));
}

#[test]
fn blocked_node_admitted_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == UNKNOWN_NODE)
        .unwrap()
        .admitted_for_display = true;
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::BlockedDisplayAdmitted));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet
        .review
        .runtime_only_never_presented_as_authored_or_generated = false;
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.blocked_nodes_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&AppTopologyViolation::ProofFreshnessIncomplete));
}

#[test]
fn unresolved_origin_blocks_a_node() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[AppTopologyRowObservation {
        row_id: AUTHORED_ROUTE.to_owned(),
        origin_resolved: false,
        generator_version_current: true,
        scan_fresh: true,
        derivation_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let authored = row(&packet, AUTHORED_ROUTE);
    assert_eq!(authored.origin_class, NodeOriginClass::OriginUnknown);
    assert_eq!(
        authored.derivation_class,
        NodeDerivationClass::DerivationUnknown
    );
    assert_eq!(
        authored.downgrade_banner_class,
        TopologyDowngradeBannerClass::OriginUnknownBanner
    );
    assert!(!authored.admitted_for_display);
    assert!(authored
        .downgrade_triggers
        .contains(&TopologyDowngradeTrigger::OriginUnknown));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn yanked_generator_version_withdraws_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[AppTopologyRowObservation {
        row_id: GENERATED_ROUTE.to_owned(),
        origin_resolved: true,
        generator_version_current: false,
        scan_fresh: true,
        derivation_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let generated = row(&packet, GENERATED_ROUTE);
    assert_eq!(generated.freshness_class, ViewFreshnessClass::Stale);
    assert_eq!(
        generated.downgrade_banner_class,
        TopologyDowngradeBannerClass::FreshnessBanner
    );
    assert!(!generated.admitted_for_display);
    assert!(generated
        .downgrade_triggers
        .contains(&TopologyDowngradeTrigger::GeneratorVersionYanked));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_scan_raises_banner_and_withdraws_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[AppTopologyRowObservation {
        row_id: AUTHORED_ROUTE.to_owned(),
        origin_resolved: true,
        generator_version_current: true,
        scan_fresh: false,
        derivation_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let authored = row(&packet, AUTHORED_ROUTE);
    assert_eq!(authored.freshness_class, ViewFreshnessClass::Stale);
    assert!(authored.downgrade_banner_class.is_present());
    assert!(!authored.admitted_for_display);
    assert!(authored
        .downgrade_triggers
        .contains(&TopologyDowngradeTrigger::ScanStale));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unverified_derivation_degrades_and_withdraws_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[AppTopologyRowObservation {
        row_id: ZONE_COMPONENT.to_owned(),
        origin_resolved: true,
        generator_version_current: true,
        scan_fresh: true,
        derivation_verified: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let zone = row(&packet, ZONE_COMPONENT);
    assert_eq!(
        zone.derivation_class,
        NodeDerivationClass::DerivationDegraded
    );
    assert!(zone.downgrade_banner_class.is_present());
    assert!(!zone.admitted_for_display);
    assert!(zone
        .downgrade_triggers
        .contains(&TopologyDowngradeTrigger::DerivationDegraded));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_proof_withholds_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[AppTopologyRowObservation {
        row_id: AUTHORED_ROUTE.to_owned(),
        origin_resolved: true,
        generator_version_current: true,
        scan_fresh: true,
        derivation_verified: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let authored = row(&packet, AUTHORED_ROUTE);
    assert!(!authored.admitted_for_display);
    assert!(authored
        .downgrade_triggers
        .contains(&TopologyDowngradeTrigger::ProofStale));
}

#[test]
fn markdown_summary_lists_every_node() {
    let summary = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(
            summary.contains(&row.node_label),
            "summary missing node {}",
            row.node_label
        );
    }
    assert!(summary.contains("origin_unknown_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_app_topology_export().expect("checked app-topology export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_app_topology_export().expect("checked app-topology export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/origin_unknown_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/derivation_degraded_withheld.json"
        )),
    ] {
        let packet: AppTopologyPacket =
            serde_json::from_str(raw).expect("fixture parses as app-topology packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
