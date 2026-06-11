use super::*;

const PACKET_ID: &str = "archetype-health:stable:0001";
const PACKET_LABEL: &str =
    "Certified-Archetype Health-Check Bundles, Stack Diagnostics, and Fix-Forward Guidance";

const SERVICE_HEALTHY: &str = "archetype-health-row:rust.axum.service.certified.healthy:2026.06";
const WEBAPP_ADVISORIES: &str = "archetype-health-row:ts.next.webapp.certified.advisories:2026.06";
const FULLSTACK_HEURISTIC: &str =
    "archetype-health-row:py.fastapi.fullstack.heuristic.degraded:2026.05";
const CLI_PROVISIONAL_FAILING: &str = "archetype-health-row:go.cli.provisional.failing:2026.06";
const LIB_HEALTH_UNKNOWN: &str =
    "archetype-health-row:rust.lib.uncertified.health_unknown.blocked:2026.04";
const SERVICE_BRIDGE: &str = "archetype-health-row:node.express.community.bridge.degraded:2026.06";

fn proof_freshness() -> ArchetypeHealthProofFreshness {
    ArchetypeHealthProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> ArchetypeHealthBundlePacket {
    canonical_archetype_health_bundles(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        "2026-06-08T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(packet: &'a ArchetypeHealthBundlePacket, row_id: &str) -> &'a ArchetypeHealthBundleRow {
    packet
        .rows
        .iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing row {row_id}"))
}

#[test]
fn archetype_health_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_rows_cover_health_spectrum() {
    let packet = packet();
    let health: Vec<HealthCheckClass> = packet
        .rows
        .iter()
        .map(|row| row.health_check_class)
        .collect();
    for required in [
        HealthCheckClass::Healthy,
        HealthCheckClass::HealthyWithAdvisories,
        HealthCheckClass::Degraded,
        HealthCheckClass::Failing,
        HealthCheckClass::HealthUnknown,
    ] {
        assert!(
            health.contains(&required),
            "missing health {}",
            required.as_str()
        );
    }
}

#[test]
fn canonical_rows_cover_diagnostic_and_fix_spectrum() {
    let packet = packet();
    let diagnostics: Vec<StackDiagnosticClass> = packet
        .rows
        .iter()
        .map(|row| row.stack_diagnostic_class)
        .collect();
    for required in [
        StackDiagnosticClass::NoDiagnostics,
        StackDiagnosticClass::Advisory,
        StackDiagnosticClass::Warning,
        StackDiagnosticClass::Error,
        StackDiagnosticClass::Blocking,
        StackDiagnosticClass::DiagnosticsUnavailable,
    ] {
        assert!(
            diagnostics.contains(&required),
            "missing diagnostic {}",
            required.as_str()
        );
    }

    let fixes: Vec<FixForwardClass> = packet
        .rows
        .iter()
        .map(|row| row.fix_forward_class)
        .collect();
    for required in [
        FixForwardClass::NoFixNeeded,
        FixForwardClass::FixAutomatic,
        FixForwardClass::FixGuided,
        FixForwardClass::FixAdvisoryOnly,
        FixForwardClass::FixUnavailable,
    ] {
        assert!(
            fixes.contains(&required),
            "missing fix-forward {}",
            required.as_str()
        );
    }
}

#[test]
fn canonical_rows_cover_certification_spectrum() {
    let packet = packet();
    let certs: Vec<ArchetypeCertificationClass> = packet
        .rows
        .iter()
        .map(|row| row.certification_class)
        .collect();
    for required in [
        ArchetypeCertificationClass::CertifiedArchetype,
        ArchetypeCertificationClass::ProvisionalArchetype,
        ArchetypeCertificationClass::CommunityArchetype,
        ArchetypeCertificationClass::UncertifiedArchetype,
    ] {
        assert!(
            certs.contains(&required),
            "missing certification {}",
            required.as_str()
        );
    }
}

#[test]
fn certified_healthy_bundle_is_active() {
    let packet = packet();
    let service = row(&packet, SERVICE_HEALTHY);
    assert_eq!(service.health_check_class, HealthCheckClass::Healthy);
    assert_eq!(
        service.stack_diagnostic_class,
        StackDiagnosticClass::NoDiagnostics
    );
    assert_eq!(service.fix_forward_class, FixForwardClass::NoFixNeeded);
    assert_eq!(
        service.certification_class,
        ArchetypeCertificationClass::CertifiedArchetype
    );
    assert!(service.admitted_for_display);
    assert!(!service.is_blocked());
}

#[test]
fn certified_advisories_bundle_offers_guided_fix() {
    let packet = packet();
    let webapp = row(&packet, WEBAPP_ADVISORIES);
    assert_eq!(
        webapp.health_check_class,
        HealthCheckClass::HealthyWithAdvisories
    );
    assert_eq!(webapp.fix_forward_class, FixForwardClass::FixGuided);
    assert!(!webapp.fix_forward_refs.is_empty());
    assert!(webapp.admitted_for_display);
}

#[test]
fn heuristic_bundle_discloses_banner_and_is_held() {
    let packet = packet();
    let fullstack = row(&packet, FULLSTACK_HEURISTIC);
    assert!(fullstack.support_class.requires_disclosure());
    assert!(!fullstack.known_issue_refs.is_empty());
    assert!(fullstack.downgrade_banner_class.is_present());
    assert!(fullstack
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::HeuristicMappingDisclosed));
    assert!(!fullstack.admitted_for_display);
}

#[test]
fn provisional_archetype_is_held_behind_certification_banner() {
    let packet = packet();
    let cli = row(&packet, CLI_PROVISIONAL_FAILING);
    assert_eq!(
        cli.certification_class,
        ArchetypeCertificationClass::ProvisionalArchetype
    );
    assert_eq!(cli.health_check_class, HealthCheckClass::Failing);
    assert_eq!(cli.fix_forward_class, FixForwardClass::FixAutomatic);
    assert!(cli.certification_class.requires_disclosure());
    assert!(cli
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::UncertifiedArchetypeDisclosed));
    // Failing health is a determinable verdict; it is not a structural block.
    assert!(!cli.is_blocked());
    assert!(!cli.admitted_for_display);
}

#[test]
fn health_unknown_bundle_is_blocked() {
    let packet = packet();
    let lib = row(&packet, LIB_HEALTH_UNKNOWN);
    assert!(lib.health_check_class.is_unknown());
    assert!(lib.stack_diagnostic_class.is_unavailable());
    assert_eq!(
        lib.downgrade_banner_class,
        ArchetypeHealthDowngradeBannerClass::HealthUnknownBanner
    );
    assert!(lib.is_blocked());
    assert!(!lib.admitted_for_display);
}

#[test]
fn bridged_bundle_discloses_known_issue_and_is_held() {
    let packet = packet();
    let bridge = row(&packet, SERVICE_BRIDGE);
    assert_eq!(
        bridge.support_class,
        ArchetypeHealthSupportClass::BridgeBehavior
    );
    assert_eq!(
        bridge.stack_diagnostic_class,
        StackDiagnosticClass::Blocking
    );
    assert!(!bridge.known_issue_refs.is_empty());
    assert!(bridge
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::BridgeBehaviorDisclosed));
    assert!(!bridge.admitted_for_display);
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::RowsEmpty));
}

#[test]
fn health_unknown_without_banner_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == LIB_HEALTH_UNKNOWN)
        .unwrap()
        .downgrade_banner_class = ArchetypeHealthDowngradeBannerClass::FreshnessBanner;
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::HealthUnknownBannerMissing));
}

#[test]
fn grounded_diagnostic_without_refs_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == FULLSTACK_HEURISTIC)
        .unwrap()
        .diagnostic_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::DiagnosticRefsMissing));
}

#[test]
fn guidance_without_refs_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == WEBAPP_ADVISORIES)
        .unwrap()
        .fix_forward_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::FixForwardRefsMissing));
}

#[test]
fn non_certified_without_disclosure_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == CLI_PROVISIONAL_FAILING)
        .unwrap()
        .downgrade_triggers
        .retain(|trigger| {
            *trigger != ArchetypeHealthDowngradeTrigger::UncertifiedArchetypeDisclosed
        });
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::CertificationUndisclosed));
}

#[test]
fn bridge_run_without_disclosure_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == SERVICE_BRIDGE)
        .unwrap()
        .known_issue_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::SupportClassUndisclosed));
}

#[test]
fn blocked_bundle_admitted_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == LIB_HEALTH_UNKNOWN)
        .unwrap()
        .admitted_for_display = true;
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::BlockedDisplayAdmitted));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet.review.health_unknown_blocks_confident_verdict = false;
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .uncertified_bundles_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ArchetypeHealthViolation::ProofFreshnessIncomplete));
}

#[test]
fn undeterminable_health_blocks_a_bundle() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ArchetypeHealthRowObservation {
        row_id: SERVICE_HEALTHY.to_owned(),
        certification_verified: true,
        health_determinable: false,
        diagnostics_available: true,
        fix_guidance_available: true,
        bundle_fresh: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let service = row(&packet, SERVICE_HEALTHY);
    assert_eq!(service.health_check_class, HealthCheckClass::HealthUnknown);
    assert_eq!(
        service.stack_diagnostic_class,
        StackDiagnosticClass::DiagnosticsUnavailable
    );
    assert_eq!(service.fix_forward_class, FixForwardClass::FixUnavailable);
    assert_eq!(
        service.downgrade_banner_class,
        ArchetypeHealthDowngradeBannerClass::HealthUnknownBanner
    );
    assert!(!service.admitted_for_display);
    assert!(service
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::HealthUndeterminable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unverified_certification_narrows_and_withdraws() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ArchetypeHealthRowObservation {
        row_id: SERVICE_HEALTHY.to_owned(),
        certification_verified: false,
        health_determinable: true,
        diagnostics_available: true,
        fix_guidance_available: true,
        bundle_fresh: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let service = row(&packet, SERVICE_HEALTHY);
    assert_eq!(
        service.certification_class,
        ArchetypeCertificationClass::CertificationUnknown
    );
    assert_eq!(
        service.downgrade_banner_class,
        ArchetypeHealthDowngradeBannerClass::CertificationBanner
    );
    assert!(!service.admitted_for_display);
    assert!(service
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::CertificationUnverified));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unavailable_diagnostics_blocks_a_bundle() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ArchetypeHealthRowObservation {
        row_id: WEBAPP_ADVISORIES.to_owned(),
        certification_verified: true,
        health_determinable: true,
        diagnostics_available: false,
        fix_guidance_available: true,
        bundle_fresh: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let webapp = row(&packet, WEBAPP_ADVISORIES);
    assert_eq!(
        webapp.stack_diagnostic_class,
        StackDiagnosticClass::DiagnosticsUnavailable
    );
    assert_eq!(
        webapp.downgrade_banner_class,
        ArchetypeHealthDowngradeBannerClass::DiagnosticsUnavailableBanner
    );
    assert!(!webapp.admitted_for_display);
    assert!(webapp
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::DiagnosticsUnavailable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unavailable_fix_guidance_is_labeled_not_withdrawn() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ArchetypeHealthRowObservation {
        row_id: WEBAPP_ADVISORIES.to_owned(),
        certification_verified: true,
        health_determinable: true,
        diagnostics_available: true,
        fix_guidance_available: false,
        bundle_fresh: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let webapp = row(&packet, WEBAPP_ADVISORIES);
    assert_eq!(webapp.fix_forward_class, FixForwardClass::FixUnavailable);
    assert_eq!(
        webapp.downgrade_banner_class,
        ArchetypeHealthDowngradeBannerClass::FixUnavailableBanner
    );
    assert!(webapp
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::FixGuidanceUnavailable));
    // A missing fix-forward path is honest, not a block: the bundle stays offered.
    assert!(webapp.admitted_for_display);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_scan_record_raises_banner_and_withdraws_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ArchetypeHealthRowObservation {
        row_id: WEBAPP_ADVISORIES.to_owned(),
        certification_verified: true,
        health_determinable: true,
        diagnostics_available: true,
        fix_guidance_available: true,
        bundle_fresh: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let webapp = row(&packet, WEBAPP_ADVISORIES);
    assert_eq!(webapp.freshness_class, ArchetypeHealthFreshnessClass::Stale);
    assert!(webapp.downgrade_banner_class.is_present());
    assert!(!webapp.admitted_for_display);
    assert!(webapp
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::BundleRecordStale));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_proof_withholds_display() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ArchetypeHealthRowObservation {
        row_id: SERVICE_HEALTHY.to_owned(),
        certification_verified: true,
        health_determinable: true,
        diagnostics_available: true,
        fix_guidance_available: true,
        bundle_fresh: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let service = row(&packet, SERVICE_HEALTHY);
    assert!(!service.admitted_for_display);
    assert!(service
        .downgrade_triggers
        .contains(&ArchetypeHealthDowngradeTrigger::ProofStale));
}

#[test]
fn markdown_summary_lists_every_archetype() {
    let summary = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(
            summary.contains(&row.archetype_label),
            "summary missing archetype {}",
            row.archetype_label
        );
    }
    assert!(summary.contains("health_unknown_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_archetype_health_export().expect("checked health-bundle export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked =
        current_archetype_health_export().expect("checked health-bundle export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/health_unknown_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/fix_forward_unavailable_labeled.json"
        )),
    ] {
        let packet: ArchetypeHealthBundlePacket =
            serde_json::from_str(raw).expect("fixture parses as health-bundle packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
