use super::*;

fn page() -> BackupRestoreFailoverPage {
    seeded_backup_restore_failover_page()
}

fn packet_mut<'a>(
    input: &'a mut BackupRestoreFailoverInput,
    packet_id: &str,
) -> &'a mut BackupRestoreFailoverPacketEntry {
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
    assert!(validate_backup_restore_failover_page(&page).is_ok());
}

#[test]
fn seeded_page_covers_every_managed_continuity_family() {
    let page = page();
    assert_eq!(page.summary.packet_count, 5);
    // backup, failover, restore, snapshot_replication, local_core_continuity
    assert_eq!(page.summary.family_count, 5);
    assert_eq!(page.summary.managed_family_packet_count, 4);
    assert!(page
        .descriptor("continuity-brf:managed-workspace-backup")
        .is_some());
    assert!(page
        .descriptor("continuity-brf:sovereign-snapshot-replication")
        .is_some());
}

#[test]
fn seeded_page_records_cadence_owner_scope_identity_and_partial_loss() {
    let page = page();
    let descriptor = page
        .descriptor("continuity-brf:managed-workspace-backup")
        .expect("backup descriptor");
    assert_eq!(descriptor.cadence_token, "per_release");
    assert_eq!(descriptor.scope_exercised_token, "fully_exercised");
    assert_eq!(descriptor.restore_identity_token, "same_identity_restore");
    assert_eq!(descriptor.partial_loss_token, "bounded_recent_window_loss");
    assert!(descriptor
        .restore_identity_line
        .contains("reproduces the same durable identity"));
    assert!(descriptor
        .partial_loss_line
        .contains("bounded recent window"));
    assert!(descriptor.drill_line.contains("Managed platform on-call"));
    assert!(descriptor.drill_line.contains("Reliability guild"));
}

#[test]
fn partially_exercised_packet_discloses_what_was_not_exercised() {
    let page = page();
    let descriptor = page
        .descriptor("continuity-brf:self-hosted-restore")
        .expect("restore descriptor");
    assert_eq!(descriptor.scope_exercised_token, "partially_exercised");
    assert!(descriptor.scope_line.contains("Not exercised:"));
    assert!(descriptor
        .scope_line
        .contains("cross-region failover cutover"));
    // The partially-exercised packet with a disclosure still stays stable.
    let outcome = page
        .outcome("continuity-brf:self-hosted-restore")
        .expect("outcome");
    assert!(!outcome.narrowed);
}

#[test]
fn seeded_summary_flags_export_safety_and_coverage() {
    let page = page();
    assert!(page.summary.no_generic_dr_text);
    assert!(page.summary.all_managed_packets_disclose_restore_identity);
    assert!(page.summary.all_packets_disclose_partial_loss);
    assert!(page.summary.all_expected_claims_covered);
    assert!(page.summary.restore_identity_and_partial_loss_export_safe);
    assert!(page.summary.raw_payloads_excluded);
    assert_eq!(page.summary.covered_claim_count, 5);
    assert_eq!(page.summary.uncovered_claim_count, 0);
}

#[test]
fn registry_reports_every_resilience_row_covered() {
    let page = page();
    let registry = DrillPacketRegistry::from_page(&page);
    assert_eq!(registry, page.registry);
    assert!(registry.all_claims_covered());
    assert!(registry.is_claim_row_covered("continuity:row:managed-workspace-sync-backup"));
    let row = registry
        .coverage_for_claim_row("continuity:row:sovereign-airgapped-snapshot")
        .expect("coverage row");
    assert_eq!(row.coverage_class, ClaimCoverageClass::CurrentPacket);
    assert!(row.covered);
}

#[test]
fn every_surface_renders_identical_vocabulary_for_a_packet() {
    let page = page();
    let packet_id = "continuity-brf:managed-relay-failover";
    let descriptor = page.descriptor(packet_id).expect("descriptor");
    let projections: Vec<&BackupRestoreFailoverSurfaceProjection> = page
        .surface_projections
        .iter()
        .filter(|projection| projection.packet_id == packet_id)
        .collect();
    assert_eq!(projections.len(), PacketSurfaceClass::ALL.len());
    for projection in projections {
        assert_eq!(projection.scope_line, descriptor.scope_line);
        assert_eq!(
            projection.restore_identity_line,
            descriptor.restore_identity_line
        );
        assert_eq!(projection.partial_loss_line, descriptor.partial_loss_line);
        assert_eq!(projection.drill_line, descriptor.drill_line);
    }
}

#[test]
fn surface_projection_count_matches_five_packets_across_five_surfaces() {
    let page = page();
    assert_eq!(
        page.summary.surface_projection_count,
        5 * PacketSurfaceClass::ALL.len()
    );
}

#[test]
fn generic_dr_text_fails_closed_and_is_withdrawn() {
    let mut input = seeded_backup_restore_failover_input();
    packet_mut(&mut input, "continuity-brf:managed-workspace-backup").generic_dr_text_only = true;

    let page = BackupRestoreFailoverPage::new("t:dr", "dr", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.summary.no_generic_dr_text);
    let outcome = page
        .outcome("continuity-brf:managed-workspace-backup")
        .expect("outcome");
    assert!(outcome.claim_withheld);
    assert!(outcome.generic_dr_text_only);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::GenericDrTextOnly));
    // The withheld packet leaves its claim row uncovered.
    let row = page
        .registry
        .coverage_for_claim_row("continuity:row:managed-workspace-sync-backup")
        .expect("row");
    assert_eq!(row.coverage_class, ClaimCoverageClass::PacketWithheld);
    assert!(!page.summary.all_expected_claims_covered);
}

#[test]
fn partial_drill_without_disclosure_narrows_to_beta() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:self-hosted-restore");
    packet.restore_scope.not_exercised_note = String::new();

    let page = BackupRestoreFailoverPage::new("t:nd", "nd", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::NotExercisedDisclosureMissing));
}

#[test]
fn managed_packet_that_exercised_nothing_holds_at_preview() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-relay-failover");
    packet.restore_scope = RestoreScope::new(ScopeExercisedClass::NotExercised, Vec::new(), "");

    let page = BackupRestoreFailoverPage::new("t:ne", "ne", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::ScopeNotExercised));
}

#[test]
fn undeclared_restore_identity_narrows_to_beta() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-workspace-backup");
    packet.restore_identity = RestoreIdentityClass::NotApplicable;
    packet.restore_identity_token = RestoreIdentityClass::NotApplicable.as_str().to_owned();

    let page = BackupRestoreFailoverPage::new("t:ri", "ri", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(!page.summary.all_managed_packets_disclose_restore_identity);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::RestoreIdentityUndeclared));
}

#[test]
fn undisclosed_partial_loss_narrows_to_beta() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-relay-failover");
    packet.partial_loss = PartialLossClass::Undisclosed;
    packet.partial_loss_token = PartialLossClass::Undisclosed.as_str().to_owned();

    let page = BackupRestoreFailoverPage::new("t:pl", "pl", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::PartialLossUndisclosed));
}

#[test]
fn stale_drill_evidence_narrows_to_beta() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:sovereign-snapshot-replication");
    packet.drill.evidence_state = DrillEvidenceStateClass::StaleNeedsDrill;
    packet.drill.evidence_state_token =
        DrillEvidenceStateClass::StaleNeedsDrill.as_str().to_owned();

    let page = BackupRestoreFailoverPage::new("t:stale", "stale", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::DrillEvidenceStale));
    // A stale packet leaves its claim row needing a refresh.
    let row = page
        .registry
        .coverage_for_claim_row("continuity:row:sovereign-airgapped-snapshot")
        .expect("row");
    assert_eq!(
        row.coverage_class,
        ClaimCoverageClass::StalePacketNeedsRefresh
    );
    assert!(!row.covered);
}

#[test]
fn never_run_drill_holds_at_preview() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-workspace-backup");
    packet.drill.evidence_state = DrillEvidenceStateClass::NeverRun;
    packet.drill.evidence_state_token = DrillEvidenceStateClass::NeverRun.as_str().to_owned();
    packet.drill.last_drill_at = String::new();

    let page = BackupRestoreFailoverPage::new("t:never", "never", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::DrillNeverRun));
}

#[test]
fn missing_freshness_window_narrows_to_beta() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-workspace-backup");
    packet.drill.evidence_expires_at = String::new();

    let page = BackupRestoreFailoverPage::new("t:fresh", "fresh", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::DrillEvidenceStale));
}

#[test]
fn on_demand_only_cadence_narrows_managed_packet() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-relay-failover");
    packet.drill.cadence = DrillCadenceClass::OnDemandOnly;
    packet.drill.cadence_token = DrillCadenceClass::OnDemandOnly.as_str().to_owned();

    let page = BackupRestoreFailoverPage::new("t:cad", "cad", "2026-06-01T00:00:00Z", input);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::DrillCadenceMissing));
}

#[test]
fn missing_drill_owner_narrows_managed_packet() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-workspace-backup");
    packet.drill.future_owner_label = String::new();

    let page = BackupRestoreFailoverPage::new("t:own", "own", "2026-06-01T00:00:00Z", input);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::DrillOwnerMissing));
}

#[test]
fn sovereign_hidden_vendor_failover_is_withdrawn() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:sovereign-snapshot-replication");
    packet.restore_failover_hosting = RestoreFailoverHostingClass::VendorOperated;
    packet.restore_failover_hosting_token = RestoreFailoverHostingClass::VendorOperated
        .as_str()
        .to_owned();
    packet.external_dependency_disclosed = false;

    let page = BackupRestoreFailoverPage::new("t:sov", "sov", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::SovereignContinuityOverclaimed));
}

#[test]
fn managed_local_core_hosting_is_a_profile_mismatch() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-workspace-backup");
    packet.restore_failover_hosting = RestoreFailoverHostingClass::LocalCore;
    packet.restore_failover_hosting_token =
        RestoreFailoverHostingClass::LocalCore.as_str().to_owned();

    let page = BackupRestoreFailoverPage::new("t:pm", "pm", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::ProfileMismatch));
}

#[test]
fn incomplete_surface_projection_narrows_to_beta() {
    let mut input = seeded_backup_restore_failover_input();
    let packet = packet_mut(&mut input, "continuity-brf:managed-workspace-backup");
    packet
        .projected_surfaces
        .retain(|surface| *surface != PacketSurfaceClass::Shiproom);

    let page = BackupRestoreFailoverPage::new("t:surf", "surf", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::SurfaceReuseIncomplete));
}

#[test]
fn missing_packet_for_claimed_row_narrows_at_preview() {
    let mut input = seeded_backup_restore_failover_input();
    input
        .packets
        .retain(|packet| packet.packet_id != "continuity-brf:managed-relay-failover");

    let page = BackupRestoreFailoverPage::new("t:miss", "miss", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(!page.summary.all_expected_claims_covered);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::PacketEvidenceMissing));
    let row = page
        .registry
        .coverage_for_claim_row("continuity:row:managed-relay-collaboration-failover")
        .expect("row");
    assert_eq!(row.coverage_class, ClaimCoverageClass::NoPacket);
}

#[test]
fn tampered_projection_vocabulary_is_detected_on_reaudit() {
    let mut page = page();
    page.surface_projections[0].restore_identity_line = "drifted vocabulary".to_owned();
    let defects = audit_backup_restore_failover_page(&page);
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason == PacketNarrowReasonClass::PacketVocabularyDrift));
    assert!(validate_backup_restore_failover_page(&page).is_err());
}

#[test]
fn local_core_packet_is_exempt_from_managed_requirements() {
    let page = page();
    // The local-core packet uses on-demand cadence, no restore identity, and no
    // freshness window, yet it does not narrow because it is not a managed family.
    let outcome = page
        .outcome("continuity-brf:local-core-continuity")
        .expect("outcome");
    assert!(!outcome.narrowed);
    assert_eq!(outcome.restore_identity_token, "not_applicable");
}

#[test]
fn support_export_wraps_seeded_page_without_raw_payloads() {
    let export = BackupRestoreFailoverSupportExport::from_page(
        "continuity:backup-restore-failover:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page(),
    );
    assert!(export.raw_payloads_excluded);
    assert!(export.restore_identity_and_partial_loss_export_safe);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.page.qualifies_stable());
}

#[test]
fn reauditing_seeded_page_returns_zero_defects() {
    let defects = audit_backup_restore_failover_page(&page());
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: BackupRestoreFailoverPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
