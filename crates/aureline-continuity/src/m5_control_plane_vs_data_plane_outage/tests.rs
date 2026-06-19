use super::*;

fn page() -> ServiceOutageTaxonomyPage {
    seeded_service_outage_taxonomy_page()
}

fn entry_mut<'a>(
    input: &'a mut ServiceOutageTaxonomyInput,
    packet_id: &str,
) -> &'a mut ServiceOutageEntry {
    input
        .entries
        .iter_mut()
        .find(|entry| entry.packet_id == packet_id)
        .unwrap_or_else(|| panic!("missing seeded packet: {packet_id}"))
}

#[test]
fn seeded_page_qualifies_stable_with_zero_defects() {
    let page = page();
    assert!(page.qualifies_stable());
    assert!(
        page.defects.is_empty(),
        "seeded defects: {:?}",
        page.defects
    );
    assert!(validate_service_outage_taxonomy_page(&page).is_ok());
}

#[test]
fn seeded_page_covers_every_optional_service_family() {
    let page = page();
    assert!(page.covers_all_families());
    assert_eq!(page.summary.family_count, 6);
    assert_eq!(page.summary.packet_count, 6);
    for family in OptionalServiceFamily::ALL {
        assert!(
            page.descriptor_for_family(family).is_some(),
            "missing family: {family:?}"
        );
    }
}

#[test]
fn seeded_page_classifies_both_planes_and_a_mix_of_severities() {
    let page = page();
    assert!(page.distinguishes_control_and_data_plane());
    assert!(page.summary.plane_distinction_present);
    assert!(page.summary.control_plane_impaired_count >= 1);
    assert!(page.summary.data_plane_impaired_count >= 1);
    assert_eq!(page.summary.degraded_count, 3);
    assert_eq!(page.summary.unavailable_count, 2);
    assert_eq!(page.summary.recovering_count, 1);
    assert_eq!(page.summary.operational_count, 0);
}

#[test]
fn every_seeded_packet_preserves_local_core_and_avoids_ide_down() {
    let page = page();
    assert!(page.summary.all_local_core_preserved);
    assert!(page.summary.no_global_ide_down_misclaim);
    assert_eq!(page.summary.local_core_preserved_count, 6);
    assert!(page.summary.raw_payloads_excluded);
    for descriptor in &page.descriptors {
        assert!(descriptor.local_core_preserved);
        assert!(!descriptor.sets_global_ide_down);
        assert!(!descriptor.conflates_local_core);
        assert!(descriptor
            .local_core_line
            .contains("Local editing, save, search, and version control all keep working"));
    }
}

#[test]
fn descriptor_carries_plain_language_outage_and_fallback() {
    let page = page();
    let descriptor = page
        .descriptor("continuity-outage:ai-gateway")
        .expect("ai-gateway descriptor");
    assert_eq!(descriptor.family_plain, "AI gateway");
    assert_eq!(descriptor.impaired_plane_plain, "managed data plane");
    assert_eq!(descriptor.severity_plain, "degraded");
    assert_eq!(
        descriptor.fallback_plain,
        "fall back to a local model or manual editing"
    );
    assert_eq!(
        descriptor.degraded_state_token,
        "managed_data_plane_impaired_local_core_preserved"
    );
}

#[test]
fn control_plane_outage_gets_a_control_plane_degraded_state() {
    let page = page();
    let descriptor = page
        .descriptor("continuity-outage:remote-control-plane")
        .expect("remote descriptor");
    assert_eq!(descriptor.impaired_plane_token, "control_plane_impairment");
    assert_eq!(
        descriptor.degraded_state_token,
        "control_plane_impaired_local_core_preserved"
    );
    assert_eq!(descriptor.fallback_token, "fail_closed_local_core_only");
}

#[test]
fn every_surface_renders_identical_vocabulary_for_a_packet() {
    let page = page();
    let packet_id = "continuity-outage:collaboration";
    let descriptor = page.descriptor(packet_id).expect("descriptor");
    let projections: Vec<&ServiceOutageSurfaceProjection> = page
        .surface_projections
        .iter()
        .filter(|projection| projection.packet_id == packet_id)
        .collect();
    assert_eq!(projections.len(), OutageSurfaceClass::ALL.len());
    for projection in projections {
        assert_eq!(
            projection.outage_summary_line,
            descriptor.outage_summary_line
        );
        assert_eq!(projection.local_core_line, descriptor.local_core_line);
    }
}

#[test]
fn surface_projection_count_matches_six_packets_across_six_surfaces() {
    let page = page();
    assert_eq!(
        page.summary.surface_projection_count,
        6 * OutageSurfaceClass::ALL.len()
    );
}

#[test]
fn ide_down_conflation_fails_closed_and_is_withdrawn() {
    let mut input = seeded_service_outage_taxonomy_input();
    let entry = entry_mut(&mut input, "continuity-outage:collaboration");
    entry.sets_global_ide_down = true;

    let page = ServiceOutageTaxonomyPage::new("t:ide", "ide", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert_eq!(page.summary.withdrawn_count, 1);
    assert!(!page.summary.no_global_ide_down_misclaim);

    let outcome = page
        .outcome("continuity-outage:collaboration")
        .expect("outcome");
    assert!(outcome.claim_withheld);
    assert!(outcome.conflates_local_core);
    assert_eq!(
        outcome.degraded_state_token,
        "local_core_conflated_misclaim"
    );

    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::LocalCoreConflated));
    assert_eq!(page.conflating_descriptors().len(), 1);

    // The guardrail isolates the misclaim: other packets stay stable.
    let other = page.outcome("continuity-outage:ai-gateway").expect("other");
    assert!(!other.narrowed);
}

#[test]
fn marking_local_editing_unavailable_fails_closed() {
    let mut input = seeded_service_outage_taxonomy_input();
    let entry = entry_mut(&mut input, "continuity-outage:remote-control-plane");
    entry.local_core.editing_available = false;

    let page = ServiceOutageTaxonomyPage::new("t:edit", "edit", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.summary.all_local_core_preserved);
    let outcome = page
        .outcome("continuity-outage:remote-control-plane")
        .expect("outcome");
    assert!(!outcome.local_core_preserved);
    assert!(outcome.conflates_local_core);
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::LocalCoreConflated));
}

#[test]
fn impaired_lane_without_a_fallback_narrows_to_beta() {
    let mut input = seeded_service_outage_taxonomy_input();
    let entry = entry_mut(&mut input, "continuity-outage:ai-gateway");
    entry.fallback = DegradedFallbackClass::NotDeclared;
    entry.fallback_token = DegradedFallbackClass::NotDeclared.as_str().to_owned();

    let page = ServiceOutageTaxonomyPage::new("t:fb", "fb", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::FallbackUndeclared));
}

#[test]
fn operational_lane_claiming_a_fallback_holds_at_preview() {
    let mut input = seeded_service_outage_taxonomy_input();
    let entry = entry_mut(&mut input, "continuity-outage:identity-policy");
    entry.severity = ImpairmentSeverityClass::Operational;
    entry.severity_token = ImpairmentSeverityClass::Operational.as_str().to_owned();
    // fallback stays active (cached_policy_read_only) while severity is operational.

    let page = ServiceOutageTaxonomyPage::new("t:op", "op", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(
        |defect| defect.narrow_reason == OutageNarrowReasonClass::OperationalStateInconsistent
    ));
}

#[test]
fn stale_outage_evidence_holds_at_preview() {
    let mut input = seeded_service_outage_taxonomy_input();
    let entry = entry_mut(&mut input, "continuity-outage:registry-updates-docs");
    entry.evidence_state = OutageEvidenceStateClass::StaleNeedsRefresh;
    entry.evidence_state_token = OutageEvidenceStateClass::StaleNeedsRefresh
        .as_str()
        .to_owned();

    let page = ServiceOutageTaxonomyPage::new("t:stale", "stale", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::OutageEvidenceStale));
}

#[test]
fn missing_outage_evidence_narrows_to_beta() {
    let mut input = seeded_service_outage_taxonomy_input();
    let entry = entry_mut(&mut input, "continuity-outage:registry-updates-docs");
    entry.evidence_state = OutageEvidenceStateClass::Missing;
    entry.evidence_state_token = OutageEvidenceStateClass::Missing.as_str().to_owned();
    entry.outage_evidence_ref = String::new();

    let page = ServiceOutageTaxonomyPage::new("t:miss", "miss", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::OutageEvidenceMissing));
}

#[test]
fn incomplete_surface_projection_narrows_to_beta() {
    let mut input = seeded_service_outage_taxonomy_input();
    let entry = entry_mut(&mut input, "continuity-outage:identity-policy");
    entry
        .projected_surfaces
        .retain(|surface| *surface != OutageSurfaceClass::Shiproom);

    let page = ServiceOutageTaxonomyPage::new("t:surf", "surf", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::SurfaceReuseIncomplete));
}

#[test]
fn dropping_a_family_flags_incomplete_coverage() {
    let mut input = seeded_service_outage_taxonomy_input();
    input
        .entries
        .retain(|entry| entry.family != OptionalServiceFamily::TelemetrySupport);

    let page = ServiceOutageTaxonomyPage::new("t:cov", "cov", "2026-06-01T00:00:00Z", input);
    assert!(!page.covers_all_families());
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::FamilyCoverageIncomplete));
}

#[test]
fn a_single_plane_taxonomy_flags_missing_distinction() {
    let mut input = seeded_service_outage_taxonomy_input();
    for entry in &mut input.entries {
        entry.impaired_plane = PlaneImpairmentClass::ControlPlaneImpairment;
        entry.impaired_plane_token = PlaneImpairmentClass::ControlPlaneImpairment
            .as_str()
            .to_owned();
    }

    let page = ServiceOutageTaxonomyPage::new("t:plane", "plane", "2026-06-01T00:00:00Z", input);
    assert!(!page.distinguishes_control_and_data_plane());
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::PlaneDistinctionMissing));
}

#[test]
fn tampered_projection_vocabulary_is_detected_on_reaudit() {
    let mut page = page();
    page.surface_projections[0].outage_summary_line = "drifted vocabulary".to_owned();
    let defects = audit_service_outage_taxonomy_page(&page);
    assert!(defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::OutageVocabularyDrift));
    assert!(validate_service_outage_taxonomy_page(&page).is_err());
}

#[test]
fn support_export_wraps_seeded_page_without_raw_payloads() {
    let export = ServiceOutageTaxonomySupportExport::from_page(
        "continuity:outage-taxonomy:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page(),
    );
    assert!(export.raw_payloads_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.page.qualifies_stable());
}

#[test]
fn reauditing_seeded_page_returns_zero_defects() {
    let defects = audit_service_outage_taxonomy_page(&page());
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: ServiceOutageTaxonomyPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
