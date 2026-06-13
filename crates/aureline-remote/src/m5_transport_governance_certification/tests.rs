//! Unit coverage for the M5 transport-governance certification packet.

use super::*;

fn seeded() -> M5TransportGovernanceCertificationPage {
    seeded_m5_transport_governance_certification_page()
}

#[test]
fn seeded_page_is_certified_and_clean() {
    let page = seeded();
    assert!(page.is_certified());
    assert!(page.no_withdrawn_rows());
    assert!(page.defects.is_empty());
    assert!(validate_certification_page(&page).is_ok());
}

#[test]
fn seeded_page_covers_every_required_profile_and_dimension() {
    let page = seeded();
    assert!(page.covers_all_required_profiles());
    assert!(page.covers_all_dimensions());
    assert_eq!(page.rows.len(), REQUIRED_PROFILES.len());
    assert_eq!(
        page.certification_snapshot.profiles.len(),
        REQUIRED_PROFILES.len()
    );
    for row in &page.rows {
        assert_eq!(row.total_dimension_count, REQUIRED_DIMENSIONS.len());
    }
}

#[test]
fn seeded_page_binds_every_dimension_to_a_sibling_lane() {
    let page = seeded();
    assert!(page.binds_all_dimensions());
    assert_eq!(page.dimension_bindings.len(), REQUIRED_DIMENSIONS.len());
    for binding in &page.dimension_bindings {
        assert_eq!(
            binding.evidence_contract_ref,
            binding.dimension.contract_ref()
        );
        assert!(!binding.evidence_doc_ref.is_empty());
    }
}

#[test]
fn seeded_page_exposes_full_denial_vocabulary() {
    let page = seeded();
    for code in REQUIRED_DENIAL_CODES {
        assert!(page
            .summary
            .denial_vocabulary
            .iter()
            .any(|t| t == code.as_str()));
    }
}

#[test]
fn local_oss_waives_proxy_and_host_proof_but_is_certified() {
    let page = seeded();
    let local = page
        .rows
        .iter()
        .find(|r| r.profile_token == "local_oss")
        .expect("local_oss row present");
    assert_eq!(
        local.verdict_token,
        CertificationVerdictClass::Certified.as_str()
    );
    assert_eq!(
        local
            .cell_states
            .get("proxy_resolution")
            .map(String::as_str),
        Some("waived")
    );
    assert_eq!(
        local.cell_states.get("host_proof").map(String::as_str),
        Some("waived")
    );
}

#[test]
fn cells_render_at_field_parity() {
    let page = seeded();
    assert!(page.all_cells_at_field_parity());
    let cell = &page.certification_snapshot.profiles[0].cells[0];
    let names: Vec<String> = cell.render_fields().into_iter().map(|(n, _)| n).collect();
    assert_eq!(names, CELL_FIELD_NAMES.to_vec());
}

#[test]
fn stale_dimension_narrows_profile() {
    let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
    let profile = &mut snapshot.profiles[1]; // self_hosted
    let cell = profile
        .cells
        .iter_mut()
        .find(|c| c.dimension == CertificationDimensionClass::TrustStore)
        .unwrap();
    cell.state = CertificationCellStateClass::Stale;
    cell.state_token = CertificationCellStateClass::Stale.as_str().to_owned();
    cell.freshness = ProofFreshnessClass::ExpiredBeyondWindow;
    cell.freshness_token = ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
    let page = M5TransportGovernanceCertificationPage::new(
        "drill",
        "drill",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    let row = page
        .rows
        .iter()
        .find(|r| r.profile_token == "self_hosted")
        .unwrap();
    assert_eq!(
        row.verdict_token,
        CertificationVerdictClass::Narrowed.as_str()
    );
    assert_eq!(row.narrow_reason_token, "transport_proof_stale");
    assert_eq!(page.summary.overall_verdict_token, "narrowed");
}

#[test]
fn missing_continuity_holds_profile_back() {
    let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
    let profile = &mut snapshot.profiles[2]; // managed
    profile
        .cells
        .retain(|c| c.dimension != CertificationDimensionClass::MirrorOffline);
    let page = M5TransportGovernanceCertificationPage::new(
        "drill",
        "drill",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    let row = page
        .rows
        .iter()
        .find(|r| r.profile_token == "managed")
        .unwrap();
    assert_eq!(
        row.verdict_token,
        CertificationVerdictClass::HeldBack.as_str()
    );
    assert_eq!(row.narrow_reason_token, "continuity_coverage_missing");
    assert_eq!(page.summary.overall_verdict_token, "held_back");
}

#[test]
fn raw_material_withdraws_whole_packet() {
    let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
    snapshot.profiles[0].raw_private_material_excluded = false;
    let page = M5TransportGovernanceCertificationPage::new(
        "drill",
        "drill",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(page.summary.overall_verdict_token, "withdrawn");
    // Every row is tainted by the withdrawal.
    for row in &page.rows {
        assert_eq!(
            row.verdict_token,
            CertificationVerdictClass::Withdrawn.as_str()
        );
    }
    assert!(validate_certification_page(&page).is_err());
}

#[test]
fn silent_public_fallthrough_withdraws_packet() {
    let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
    snapshot.profiles[3].no_silent_public_fallthrough = false; // air_gapped
    let page = M5TransportGovernanceCertificationPage::new(
        "drill",
        "drill",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(page.summary.overall_verdict_token, "withdrawn");
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == CertificationNarrowReasonClass::SilentPublicFallthrough));
}

#[test]
fn missing_required_profile_holds_page_back() {
    let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
    snapshot
        .profiles
        .retain(|p| p.profile != CertificationProfileClass::AirGapped);
    let page = M5TransportGovernanceCertificationPage::new(
        "drill",
        "drill",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.covers_all_required_profiles());
    assert_eq!(page.summary.overall_verdict_token, "held_back");
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == CertificationNarrowReasonClass::RequiredProfileMissing));
}

#[test]
fn page_round_trips_through_serde() {
    let page = seeded();
    let json = serde_json::to_string(&page).expect("serialize");
    let back: M5TransportGovernanceCertificationPage =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, back);
}

#[test]
fn support_export_rolls_up_defects_safely() {
    let mut snapshot = seeded_m5_transport_governance_certification_snapshot();
    let cell = snapshot.profiles[1]
        .cells
        .iter_mut()
        .find(|c| c.dimension == CertificationDimensionClass::ProxyResolution)
        .unwrap();
    cell.state = CertificationCellStateClass::Partial;
    cell.state_token = CertificationCellStateClass::Partial.as_str().to_owned();
    let page = M5TransportGovernanceCertificationPage::new(
        "drill",
        "drill",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    let export = CertificationSupportExport::from_page("export-1", "2026-06-01T00:00:00Z", page);
    assert!(export.raw_private_material_excluded);
    assert!(export
        .narrow_reasons_present
        .contains(&CertificationNarrowReasonClass::TransportProofPartial));
    assert_eq!(
        export
            .defect_counts_by_narrow_reason
            .get("transport_proof_partial"),
        Some(&1)
    );
}
