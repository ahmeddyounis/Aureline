use super::*;

fn page() -> MirrorAirgapPage {
    seeded_mirror_airgap_page()
}

fn packet_mut<'a>(
    input: &'a mut MirrorAirgapInput,
    packet_id: &str,
) -> &'a mut MirrorAirgapPacketEntry {
    input
        .packets
        .iter_mut()
        .find(|packet| packet.packet_id == packet_id)
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
    assert!(validate_mirror_airgap_page(&page).is_ok());
}

#[test]
fn seeded_page_exercises_mirror_only_and_air_gapped_rows() {
    let page = page();
    assert_eq!(page.summary.packet_count, 4);
    assert_eq!(page.summary.mirror_only_count, 1);
    assert_eq!(page.summary.air_gapped_count, 1);
    assert_eq!(page.summary.self_hosted_restricted_count, 1);
    assert!(page.summary.exercises_mirror_only_and_air_gapped);
    assert!(page
        .descriptor("continuity-mirror:mirror-only-self-hosted")
        .is_some());
    assert!(page
        .descriptor("continuity-mirror:air-gapped-sovereign")
        .is_some());
}

#[test]
fn seeded_rows_show_trust_root_offline_advisory_and_public_fallback() {
    let page = page();
    let mirror = page
        .descriptor("continuity-mirror:mirror-only-self-hosted")
        .expect("mirror-only descriptor");
    assert_eq!(
        mirror.trust_root_posture_token,
        "customer_managed_trust_root"
    );
    assert!(mirror.trust_root_survives_offline);
    assert_eq!(mirror.public_fallback_policy_token, "prohibited");
    assert!(mirror.public_fallback_governed);
    assert!(mirror.trust_root_line.contains("survives offline"));
    assert!(mirror
        .advisory_line
        .contains("replicated through the approved mirror"));
    assert!(mirror.public_fallback_line.contains("prohibited"));

    let air = page
        .descriptor("continuity-mirror:air-gapped-sovereign")
        .expect("air-gapped descriptor");
    assert_eq!(air.trust_root_posture_token, "offline_trust_root");
    assert_eq!(air.public_fallback_policy_token, "unavailable");
    assert!(air.offline_exchange_line.contains("signed offline bundle"));
    assert!(air
        .offline_exchange_line
        .contains("physical media transfer"));
}

#[test]
fn seeded_summary_flags_governance_and_export_safety() {
    let page = page();
    assert!(page.summary.no_silent_public_fallback);
    assert!(page.summary.no_advisory_live_public_fetch_on_isolated);
    assert!(page.summary.all_offline_rows_declare_trust_root_continuity);
    assert!(page.summary.all_offline_rows_state_public_fallback_policy);
    assert!(page.summary.all_expected_claims_covered);
    assert!(page.summary.fallback_and_trust_root_export_safe);
    assert!(page.summary.raw_payloads_excluded);
    assert_eq!(page.summary.public_fallback_governed_count, 3);
    // Three offline rows plus the exempt local-only row are all covered.
    assert_eq!(page.summary.covered_claim_count, 4);
    assert_eq!(page.summary.uncovered_claim_count, 0);
}

#[test]
fn registry_reports_every_offline_row_covered() {
    let page = page();
    let registry = OfflineContinuityRegistry::from_page(&page);
    assert_eq!(registry, page.registry);
    assert!(registry.all_claims_covered());
    assert!(registry.is_claim_row_covered("continuity:row:mirror-only-self-hosted-registry"));
    let row = registry
        .coverage_for_claim_row("continuity:row:air-gapped-sovereign-boundary")
        .expect("coverage row");
    assert_eq!(row.coverage_class, OfflineCoverageClass::CurrentPacket);
    assert!(row.covered);
}

#[test]
fn every_surface_renders_identical_vocabulary_for_a_packet() {
    let page = page();
    let packet_id = "continuity-mirror:air-gapped-sovereign";
    let descriptor = page.descriptor(packet_id).expect("descriptor");
    let projections: Vec<&MirrorAirgapSurfaceProjection> = page
        .surface_projections
        .iter()
        .filter(|projection| projection.packet_id == packet_id)
        .collect();
    assert_eq!(projections.len(), OfflineSurfaceClass::ALL.len());
    for projection in projections {
        assert_eq!(projection.trust_root_line, descriptor.trust_root_line);
        assert_eq!(
            projection.mirror_freshness_line,
            descriptor.mirror_freshness_line
        );
        assert_eq!(
            projection.offline_exchange_line,
            descriptor.offline_exchange_line
        );
        assert_eq!(projection.advisory_line, descriptor.advisory_line);
        assert_eq!(
            projection.public_fallback_line,
            descriptor.public_fallback_line
        );
    }
}

#[test]
fn surface_projection_count_matches_four_packets_across_five_surfaces() {
    let page = page();
    assert_eq!(
        page.summary.surface_projection_count,
        4 * OfflineSurfaceClass::ALL.len()
    );
}

#[test]
fn silent_public_fallback_fails_closed_and_is_withdrawn() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet.public_fallback_policy = PublicFallbackPolicyClass::SilentPublicFallback;
    packet.public_fallback_policy_token = PublicFallbackPolicyClass::SilentPublicFallback
        .as_str()
        .to_owned();

    let page = MirrorAirgapPage::new("t:silent", "silent", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.summary.no_silent_public_fallback);
    let outcome = page
        .outcome("continuity-mirror:mirror-only-self-hosted")
        .expect("outcome");
    assert!(outcome.claim_withheld);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::SilentPublicFallback));
    let row = page
        .registry
        .coverage_for_claim_row("continuity:row:mirror-only-self-hosted-registry")
        .expect("row");
    assert_eq!(row.coverage_class, OfflineCoverageClass::PacketWithheld);
    assert!(!page.summary.all_expected_claims_covered);
}

#[test]
fn advisory_live_public_fetch_on_isolated_row_is_withdrawn() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:air-gapped-sovereign");
    packet.advisory_revocation_source = AdvisoryRevocationSourceClass::LivePublicFetch;
    packet.advisory_revocation_source_token = AdvisoryRevocationSourceClass::LivePublicFetch
        .as_str()
        .to_owned();

    let page = MirrorAirgapPage::new("t:adv", "adv", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.summary.no_advisory_live_public_fetch_on_isolated);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::AdvisoryImpliesLivePublicFetch));
}

#[test]
fn self_hosted_restricted_may_fetch_advisories_publicly() {
    // A self-hosted-restricted row is not isolated, so a live public advisory
    // fetch is allowed and does not narrow.
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:self-hosted-restricted");
    packet.advisory_revocation_source = AdvisoryRevocationSourceClass::LivePublicFetch;
    packet.advisory_revocation_source_token = AdvisoryRevocationSourceClass::LivePublicFetch
        .as_str()
        .to_owned();

    let page = MirrorAirgapPage::new("t:srf", "srf", "2026-06-01T00:00:00Z", input);
    assert!(page.qualifies_stable(), "defects: {:?}", page.defects);
}

#[test]
fn isolated_trust_root_needing_public_reissue_is_withdrawn() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet.trust_root.renewal = TrustRootRenewalClass::PublicReissueRequired;
    packet.trust_root.renewal_token = TrustRootRenewalClass::PublicReissueRequired
        .as_str()
        .to_owned();

    let page = MirrorAirgapPage::new("t:tr", "tr", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::TrustRootBreaksOffline));
}

#[test]
fn undisclosed_trust_root_narrows_to_preview() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet.trust_root.posture = TrustRootPostureClass::TrustRootUndisclosed;
    packet.trust_root.posture_token = TrustRootPostureClass::TrustRootUndisclosed
        .as_str()
        .to_owned();

    let page = MirrorAirgapPage::new("t:tru", "tru", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(!page.summary.all_offline_rows_declare_trust_root_continuity);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::TrustRootContinuityUndeclared));
}

#[test]
fn undisclosed_public_fallback_narrows_to_preview() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:self-hosted-restricted");
    packet.public_fallback_policy = PublicFallbackPolicyClass::Undisclosed;
    packet.public_fallback_policy_token =
        PublicFallbackPolicyClass::Undisclosed.as_str().to_owned();

    let page = MirrorAirgapPage::new("t:pf", "pf", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(!page.summary.all_offline_rows_state_public_fallback_policy);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::PublicFallbackUndisclosed));
}

#[test]
fn undisclosed_offline_exchange_narrows_to_preview() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:air-gapped-sovereign");
    packet.offline_import = OfflineExchangeClass::Undisclosed;
    packet.offline_import_token = OfflineExchangeClass::Undisclosed.as_str().to_owned();

    let page = MirrorAirgapPage::new("t:ox", "ox", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::OfflineExchangeUndisclosed));
}

#[test]
fn undisclosed_advisory_source_narrows_to_preview() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet.advisory_revocation_source = AdvisoryRevocationSourceClass::Undisclosed;
    packet.advisory_revocation_source_token = AdvisoryRevocationSourceClass::Undisclosed
        .as_str()
        .to_owned();

    let page = MirrorAirgapPage::new("t:ad", "ad", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::AdvisoryRevocationUndisclosed));
}

#[test]
fn stale_mirror_needing_sync_narrows_to_beta() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet.mirror_freshness.state = MirrorFreshnessStateClass::StaleNeedsSync;
    packet.mirror_freshness.state_token = MirrorFreshnessStateClass::StaleNeedsSync
        .as_str()
        .to_owned();

    let page = MirrorAirgapPage::new("t:ms", "ms", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::MirrorFreshnessStale));
    let row = page
        .registry
        .coverage_for_claim_row("continuity:row:mirror-only-self-hosted-registry")
        .expect("row");
    assert_eq!(
        row.coverage_class,
        OfflineCoverageClass::StalePacketNeedsRefresh
    );
    assert!(!row.covered);
}

#[test]
fn never_synced_mirror_holds_at_preview() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet.mirror_freshness.state = MirrorFreshnessStateClass::NeverSynced;
    packet.mirror_freshness.state_token =
        MirrorFreshnessStateClass::NeverSynced.as_str().to_owned();
    packet.mirror_freshness.last_synced_at = String::new();

    let page = MirrorAirgapPage::new("t:nv", "nv", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::MirrorNeverSynced));
}

#[test]
fn mirror_only_row_without_a_mirror_narrows() {
    // A mirror-only row that names no live mirror (not_applicable) is incomplete.
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet.mirror_freshness.state = MirrorFreshnessStateClass::NotApplicable;
    packet.mirror_freshness.state_token =
        MirrorFreshnessStateClass::NotApplicable.as_str().to_owned();
    packet.mirror_freshness.last_synced_at = String::new();
    packet.mirror_freshness.freshness_expires_at = String::new();

    let page = MirrorAirgapPage::new("t:nm", "nm", "2026-06-01T00:00:00Z", input);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::MirrorFreshnessStale));
}

#[test]
fn air_gapped_row_without_a_mirror_is_acceptable() {
    // The seeded air-gapped row uses offline bundles and a not_applicable mirror;
    // it qualifies stable. Confirm that posture exemption holds in isolation.
    let page = page();
    let outcome = page
        .outcome("continuity-mirror:air-gapped-sovereign")
        .expect("outcome");
    assert!(!outcome.narrowed);
    assert_eq!(outcome.mirror_freshness_token, "not_applicable");
}

#[test]
fn missing_freshness_window_narrows_to_beta() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet.mirror_freshness.freshness_expires_at = String::new();

    let page = MirrorAirgapPage::new("t:fw", "fw", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::MirrorFreshnessStale));
}

#[test]
fn profile_posture_mismatch_narrows_to_preview() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:air-gapped-sovereign");
    packet.profile_class = ContinuityProfileClass::Managed;
    packet.profile_class_token = ContinuityProfileClass::Managed.as_str().to_owned();

    let page = MirrorAirgapPage::new("t:mm", "mm", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::ProfilePostureMismatch));
}

#[test]
fn incomplete_surface_projection_narrows_to_beta() {
    let mut input = seeded_mirror_airgap_input();
    let packet = packet_mut(&mut input, "continuity-mirror:mirror-only-self-hosted");
    packet
        .projected_surfaces
        .retain(|surface| *surface != OfflineSurfaceClass::Shiproom);

    let page = MirrorAirgapPage::new("t:surf", "surf", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::SurfaceReuseIncomplete));
}

#[test]
fn missing_packet_for_claimed_row_narrows_at_preview() {
    let mut input = seeded_mirror_airgap_input();
    input
        .packets
        .retain(|packet| packet.packet_id != "continuity-mirror:air-gapped-sovereign");

    let page = MirrorAirgapPage::new("t:miss", "miss", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(!page.summary.all_expected_claims_covered);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::PacketEvidenceMissing));
    let row = page
        .registry
        .coverage_for_claim_row("continuity:row:air-gapped-sovereign-boundary")
        .expect("row");
    assert_eq!(row.coverage_class, OfflineCoverageClass::NoPacket);
}

#[test]
fn tampered_projection_vocabulary_is_detected_on_reaudit() {
    let mut page = page();
    page.surface_projections[0].public_fallback_line = "drifted vocabulary".to_owned();
    let defects = audit_mirror_airgap_page(&page);
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason == MirrorAirgapNarrowReasonClass::PacketVocabularyDrift));
    assert!(validate_mirror_airgap_page(&page).is_err());
}

#[test]
fn local_only_packet_is_exempt_from_offline_requirements() {
    let page = page();
    // The local-only packet declares not-applicable exchange, advisory, and
    // public-fallback values, yet does not narrow because it is exempt.
    let outcome = page
        .outcome("continuity-mirror:local-only-core")
        .expect("outcome");
    assert!(!outcome.narrowed);
    assert_eq!(outcome.public_fallback_policy_token, "not_applicable");
}

#[test]
fn support_export_wraps_seeded_page_without_raw_payloads() {
    let export = MirrorAirgapSupportExport::from_page(
        "continuity:mirror-airgap:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page(),
    );
    assert!(export.raw_payloads_excluded);
    assert!(export.fallback_and_trust_root_export_safe);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.page.qualifies_stable());
}

#[test]
fn reauditing_seeded_page_returns_zero_defects() {
    let defects = audit_mirror_airgap_page(&page());
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: MirrorAirgapPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
