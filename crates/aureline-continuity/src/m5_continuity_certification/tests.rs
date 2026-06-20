use super::*;

fn report() -> ContinuityCertificationReport {
    seeded_continuity_certification_report()
}

fn row<'a>(input: &'a mut ContinuityCertificationInput, row_id: &str) -> &'a mut CertifiedRow {
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing seeded row: {row_id}"))
}

fn set_state(
    input: &mut ContinuityCertificationInput,
    row_id: &str,
    dimension: CertificationDimension,
    state: CertificationEvidenceState,
) {
    let row = row(input, row_id);
    let cell = row
        .evidence
        .iter_mut()
        .find(|cell| cell.dimension == dimension)
        .unwrap_or_else(|| panic!("missing dimension {dimension:?} on {row_id}"));
    cell.state = state;
    cell.state_token = state.as_str().to_owned();
    if !state.requires_evidence_ref() {
        cell.evidence_ref.clear();
    } else if cell.evidence_ref.is_empty() {
        cell.evidence_ref = format!("{row_id}:{}", dimension.as_str());
    }
}

fn rebuilt(input: ContinuityCertificationInput) -> ContinuityCertificationReport {
    ContinuityCertificationReport::new(
        "continuity:certification:case",
        "case",
        "2026-06-19T00:00:00Z",
        input,
    )
}

#[test]
fn seeded_report_is_fully_certified() {
    let report = report();
    assert!(
        report.is_structurally_clean(),
        "defects: {:?}",
        report.defects
    );
    assert!(report.is_fully_certified());
    assert!(validate_continuity_certification_report(&report).is_ok());
    assert_eq!(audit_continuity_certification_report(&report).len(), 0);
    assert_eq!(report.summary.overall_decision_token, "certified");
}

#[test]
fn seeded_summary_counts_match_input() {
    let report = report();
    assert_eq!(report.summary.row_count, 5);
    assert_eq!(report.summary.certification_scope_row_count, 4);
    assert_eq!(report.summary.local_core_row_count, 1);
    assert_eq!(report.summary.certified_row_count, 5);
    assert_eq!(report.summary.narrowed_row_count, 0);
    assert_eq!(report.summary.withdrawn_row_count, 0);
    assert_eq!(report.summary.stale_or_missing_evidence_row_count, 0);
    assert_eq!(
        report.summary.backup_restore_failover_uncertified_row_count,
        0
    );
    assert_eq!(report.summary.drill_freshness_uncertified_row_count, 0);
    assert_eq!(report.summary.defect_count, 0);
}

#[test]
fn every_scope_row_declares_every_required_dimension() {
    let report = report();
    for row in report
        .input
        .rows
        .iter()
        .filter(|r| r.in_certification_scope())
    {
        for dimension in row.required_dimensions() {
            assert!(
                row.evidence_for(dimension).is_some(),
                "{} missing dimension {:?}",
                row.row_id,
                dimension
            );
        }
    }
}

#[test]
fn stale_backup_drill_narrows_to_beta() {
    let mut input = seeded_continuity_certification_input();
    set_state(
        &mut input,
        "continuity-row:managed-cloud-sync",
        CertificationDimension::BackupRestoreFailover,
        CertificationEvidenceState::Stale,
    );
    let report = rebuilt(input);
    let outcome = report
        .row_outcome("continuity-row:managed-cloud-sync")
        .expect("row present");
    assert!(outcome.narrowed);
    assert_eq!(outcome.verdict_token, "narrowed");
    assert_eq!(outcome.effective_qualification_token, "beta");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"backup_restore_failover_uncertified".to_owned()));
    assert_eq!(report.summary.overall_decision_token, "narrowed");
    assert!(report.is_structurally_clean());
}

#[test]
fn missing_restore_identity_narrows_to_preview() {
    let mut input = seeded_continuity_certification_input();
    set_state(
        &mut input,
        "continuity-row:self-hosted-restore",
        CertificationDimension::RestoreIdentityPartialLoss,
        CertificationEvidenceState::Missing,
    );
    let report = rebuilt(input);
    let outcome = report
        .row_outcome("continuity-row:self-hosted-restore")
        .expect("row present");
    assert_eq!(outcome.effective_qualification_token, "preview");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"restore_identity_partial_loss_uncertified".to_owned()));
}

#[test]
fn breached_freshness_narrows_the_row() {
    let mut input = seeded_continuity_certification_input();
    set_state(
        &mut input,
        "continuity-row:managed-relay-failover",
        CertificationDimension::DrillFreshnessSlo,
        CertificationEvidenceState::Stale,
    );
    let report = rebuilt(input);
    let outcome = report
        .row_outcome("continuity-row:managed-relay-failover")
        .expect("row present");
    assert!(outcome.narrowed);
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"drill_freshness_uncertified".to_owned()));
    assert_eq!(report.summary.drill_freshness_uncertified_row_count, 1);
}

#[test]
fn profile_mismatch_withdraws_the_claim() {
    let mut input = seeded_continuity_certification_input();
    set_state(
        &mut input,
        "continuity-row:sovereign-airgap-snapshot",
        CertificationDimension::LocalityTenantKey,
        CertificationEvidenceState::ProfileMismatched,
    );
    let report = rebuilt(input);
    let outcome = report
        .row_outcome("continuity-row:sovereign-airgap-snapshot")
        .expect("row present");
    assert_eq!(outcome.verdict_token, "withdrawn");
    assert_eq!(outcome.effective_qualification_token, "withdrawn");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"continuity_profile_mismatch".to_owned()));
    assert_eq!(report.summary.overall_decision_token, "withdrawn");
    assert_eq!(report.summary.withdrawn_row_count, 1);
}

#[test]
fn missing_required_dimension_emits_defect_and_narrows() {
    let mut input = seeded_continuity_certification_input();
    let row = row(&mut input, "continuity-row:managed-cloud-sync");
    row.evidence
        .retain(|cell| cell.dimension != CertificationDimension::ControlDataPlaneDegradation);
    let report = rebuilt(input);
    assert!(report.defects.iter().any(|d| {
        d.defect_kind == CertificationDefectKind::RequiredDimensionMissing
            && d.source.contains("control_data_plane_degradation")
    }));
    let outcome = report
        .row_outcome("continuity-row:managed-cloud-sync")
        .expect("row present");
    assert!(outcome.narrowed);
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"required_evidence_missing".to_owned()));
}

#[test]
fn shared_reference_drill_narrows_both_rows() {
    let mut input = seeded_continuity_certification_input();
    let shared = "drill:shared-reference-env".to_owned();
    for row_id in [
        "continuity-row:managed-cloud-sync",
        "continuity-row:managed-relay-failover",
    ] {
        let row = row(&mut input, row_id);
        let cell = row
            .evidence
            .iter_mut()
            .find(|c| c.dimension == CertificationDimension::BackupRestoreFailover)
            .expect("drill cell");
        cell.evidence_ref = shared.clone();
    }
    let report = rebuilt(input);
    assert!(report
        .defects
        .iter()
        .any(|d| d.defect_kind == CertificationDefectKind::SharedReferenceDrillEvidence));
    for row_id in [
        "continuity-row:managed-cloud-sync",
        "continuity-row:managed-relay-failover",
    ] {
        let outcome = report.row_outcome(row_id).expect("row present");
        assert!(outcome.narrowed, "{row_id} should narrow");
        assert!(outcome
            .narrow_reason_tokens
            .contains(&"shared_reference_drill_reused".to_owned()));
    }
}

#[test]
fn local_core_row_stays_certified_when_managed_rows_go_stale() {
    let mut input = seeded_continuity_certification_input();
    set_state(
        &mut input,
        "continuity-row:managed-cloud-sync",
        CertificationDimension::BackupRestoreFailover,
        CertificationEvidenceState::Missing,
    );
    let report = rebuilt(input);
    let local = report
        .row_outcome("continuity-row:local-desktop-core")
        .expect("row present");
    assert!(local.certified);
    assert!(!local.narrowed);
    assert!(!local.in_certification_scope);
    assert!(local.narrow_reason_tokens.is_empty());
    // The managed row still narrowed.
    let managed = report
        .row_outcome("continuity-row:managed-cloud-sync")
        .expect("row present");
    assert!(managed.narrowed);
}

#[test]
fn surface_reuse_incomplete_emits_defect_and_narrows() {
    let mut input = seeded_continuity_certification_input();
    row(&mut input, "continuity-row:managed-cloud-sync")
        .surface_visibility
        .partner_qualification = false;
    let report = rebuilt(input);
    assert!(report
        .defects
        .iter()
        .any(|d| d.defect_kind == CertificationDefectKind::SurfaceReuseIncomplete));
    let outcome = report
        .row_outcome("continuity-row:managed-cloud-sync")
        .expect("row present");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"surface_reuse_incomplete".to_owned()));
}

#[test]
fn evidence_ref_incoherent_when_present_state_lacks_ref() {
    let mut input = seeded_continuity_certification_input();
    let row = row(&mut input, "continuity-row:managed-cloud-sync");
    let cell = row
        .evidence
        .iter_mut()
        .find(|c| c.dimension == CertificationDimension::LocalityTenantKey)
        .expect("cell");
    cell.evidence_ref.clear();
    let report = rebuilt(input);
    assert!(report
        .defects
        .iter()
        .any(|d| d.defect_kind == CertificationDefectKind::EvidenceRefIncoherent));
}

#[test]
fn support_export_excludes_raw_material_and_lists_reasons() {
    let mut input = seeded_continuity_certification_input();
    set_state(
        &mut input,
        "continuity-row:managed-cloud-sync",
        CertificationDimension::BackupRestoreFailover,
        CertificationEvidenceState::Stale,
    );
    let report = rebuilt(input);
    let export = ContinuityCertificationSupportExport::from_report(
        "continuity:certification:support-export:test",
        "2026-06-19T00:00:00Z",
        report,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export
        .narrow_reasons_present
        .contains(&"backup_restore_failover_uncertified".to_owned()));
}

#[test]
fn audit_matches_recorded_defects() {
    let mut input = seeded_continuity_certification_input();
    row(&mut input, "continuity-row:managed-cloud-sync")
        .evidence
        .retain(|cell| cell.dimension != CertificationDimension::DrillFreshnessSlo);
    let report = rebuilt(input);
    assert_eq!(
        audit_continuity_certification_report(&report),
        report.defects
    );
    assert!(validate_continuity_certification_report(&report).is_err());
}

#[test]
fn every_dimension_token_round_trips() {
    for dimension in CertificationDimension::ALL {
        let token = dimension.as_str();
        let json = serde_json::to_string(&dimension).expect("serialize");
        assert_eq!(json, format!("\"{token}\""));
    }
}
