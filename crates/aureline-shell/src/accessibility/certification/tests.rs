//! Inline tests for the M5 dynamic-surface assistive-tech certification capstone.

use super::*;

use crate::accessibility::M5SurfaceFamily;

fn canonical() -> M5DynamicA11yCertificationPacket {
    seeded_m5_dynamic_a11y_certification()
}

#[test]
fn canonical_packet_validates() {
    let packet = canonical();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DYNAMIC_A11Y_CERTIFICATION_PACKET_ID);
    assert_eq!(
        packet.record_kind,
        M5_DYNAMIC_A11Y_CERTIFICATION_RECORD_KIND
    );
}

#[test]
fn canonical_covers_every_dynamic_surface_across_six_dimensions() {
    let packet = canonical();
    assert_eq!(packet.surfaces.len(), M5SurfaceFamily::ALL.len());
    let present: std::collections::BTreeSet<M5SurfaceFamily> =
        packet.surfaces.iter().map(|s| s.surface_family).collect();
    for family in M5SurfaceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing family {}",
            family.as_str()
        );
    }
    for surface in &packet.surfaces {
        let dims: std::collections::BTreeSet<M5A11yProofDimension> =
            surface.dimensions.iter().map(|d| d.dimension).collect();
        assert_eq!(
            dims.len(),
            M5A11yProofDimension::ALL.len(),
            "surface {} missing dimensions",
            surface.surface_id
        );
        for dimension in M5A11yProofDimension::ALL {
            assert!(
                dims.contains(&dimension),
                "surface {} missing dimension {}",
                surface.surface_id,
                dimension.as_str()
            );
        }
    }
}

#[test]
fn canonical_is_all_certified_green() {
    let packet = canonical();
    for surface in &packet.surfaces {
        assert_eq!(
            surface.certification_status,
            M5A11yCertificationStatus::Certified,
            "surface {} not certified",
            surface.surface_id
        );
        assert_eq!(surface.signal, M5A11yCertificationSignal::Green);
        assert!(surface.is_certified());
        assert_eq!(
            surface.effective_qualification,
            M5DynamicSurfaceA11yQualificationClass::Stable
        );
        assert!(surface.stale_proof_causes.is_empty());
    }
    assert!(!packet.blocks_stable_promotion());
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.green_count, M5SurfaceFamily::ALL.len() as u32);
    assert_eq!(dashboard.yellow_count, 0);
    assert_eq!(dashboard.red_count, 0);
}

#[test]
fn certification_reuses_diagnostics_object_identity() {
    use crate::accessibility::diagnostics::seeded_m5_dynamic_a11y_diagnostics_report;
    let diagnostics = seeded_m5_dynamic_a11y_diagnostics_report();
    let packet = canonical();
    for diag in &diagnostics.surfaces {
        let cert = packet
            .surfaces
            .iter()
            .find(|s| s.surface_family == diag.surface_family)
            .expect("certification row for family");
        assert_eq!(
            cert.object_identity_ref,
            diag.object_identity_ref,
            "object identity drifted for {}",
            diag.surface_family.as_str()
        );
    }
}

#[test]
fn every_dimension_binds_to_its_backing_proof() {
    let packet = canonical();
    for surface in &packet.surfaces {
        for dimension in &surface.dimensions {
            assert_eq!(
                dimension.backing_proof_ref,
                dimension.dimension.backing_proof_ref(),
                "dimension {} backing proof drifted",
                dimension.dimension.as_str()
            );
        }
    }
}

#[test]
fn stale_proof_drill_auto_narrows_without_blocking() {
    let packet = seeded_m5_dynamic_a11y_certification_stale_proof_retest_pending();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // Stale proof narrows but never blocks: the lane keeps shipping at a reduced claim.
    assert!(!packet.blocks_stable_promotion());
    let dense = packet
        .surfaces
        .iter()
        .find(|s| s.surface_family == M5SurfaceFamily::DenseCollection)
        .expect("dense surface present");
    assert_eq!(
        dense.certification_status,
        M5A11yCertificationStatus::RetestPending
    );
    assert_eq!(dense.signal, M5A11yCertificationSignal::Yellow);
    assert!(dense.is_auto_narrowed());
    assert_eq!(
        dense.effective_qualification,
        M5DynamicSurfaceA11yQualificationClass::Beta
    );
    // The exact stale-proof cause is named for the stale dimension.
    let cause = dense
        .stale_proof_causes
        .iter()
        .find(|c| c.dimension == M5A11yProofDimension::StaleProofDowngrade)
        .expect("stale-proof cause named");
    assert_eq!(cause.freshness, M5A11yProofFreshness::Stale);
    assert_eq!(
        cause.trigger,
        M5DynamicSurfaceA11yDowngradeTrigger::ProofStale
    );
    assert!(!cause.waived);
}

#[test]
fn regression_drill_blocks_stable_promotion_and_names_the_cause() {
    let packet = seeded_m5_dynamic_a11y_certification_regression_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());
    let terminal = packet
        .surfaces
        .iter()
        .find(|s| s.surface_family == M5SurfaceFamily::TerminalCanvas)
        .expect("terminal surface present");
    assert_eq!(
        terminal.certification_status,
        M5A11yCertificationStatus::Degraded
    );
    assert_eq!(terminal.signal, M5A11yCertificationSignal::Red);
    assert!(terminal.is_blocked());
    assert_eq!(
        terminal.effective_qualification,
        M5DynamicSurfaceA11yQualificationClass::Held
    );
    assert_eq!(
        packet.blocked_surface_ids(),
        vec![terminal.surface_id.as_str()]
    );
    let cause = terminal
        .stale_proof_causes
        .iter()
        .find(|c| c.dimension == M5A11yProofDimension::BridgeHealth)
        .expect("bridge-health cause named");
    assert_eq!(cause.conformance, M5DiagnosticOutcome::Regressed);
    assert!(!cause.waived);
}

#[test]
fn waiver_drill_ships_narrowed_but_stays_red() {
    let packet = seeded_m5_dynamic_a11y_certification_waived_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The waived regression no longer blocks promotion, but the true status stays degraded.
    assert!(!packet.blocks_stable_promotion());
    let dense = packet
        .surfaces
        .iter()
        .find(|s| s.surface_family == M5SurfaceFamily::DenseCollection)
        .expect("dense surface present");
    assert_eq!(
        dense.certification_status,
        M5A11yCertificationStatus::Degraded
    );
    assert_eq!(dense.signal, M5A11yCertificationSignal::Red);
    assert!(dense.is_auto_narrowed());
    assert_eq!(
        dense.effective_qualification,
        M5DynamicSurfaceA11yQualificationClass::Preview
    );
    assert_eq!(dense.waivers.len(), 1);
    let waiver = &dense.waivers[0];
    assert_eq!(waiver.dimension, M5A11yProofDimension::AnnouncementCoverage);
    assert_eq!(
        waiver.narrowed_to,
        M5DynamicSurfaceA11yQualificationClass::Preview
    );
    assert!(!waiver.expires_at.trim().is_empty());
    // The cause is still named, but disclosed as waived.
    let cause = dense
        .stale_proof_causes
        .iter()
        .find(|c| c.dimension == M5A11yProofDimension::AnnouncementCoverage)
        .expect("announcement-coverage cause named");
    assert!(cause.waived);
    // The dashboard names the active waiver.
    let dashboard = packet.dashboard();
    assert!(dashboard.active_waiver_ids.contains(&waiver.waiver_id));
    assert!(dashboard.waived_surface_ids.contains(&dense.surface_id));
}

#[test]
fn dashboard_traffic_light_matches_rows() {
    for packet in [
        seeded_m5_dynamic_a11y_certification(),
        seeded_m5_dynamic_a11y_certification_stale_proof_retest_pending(),
        seeded_m5_dynamic_a11y_certification_regression_blocked(),
        seeded_m5_dynamic_a11y_certification_waived_narrowed(),
    ] {
        let dashboard = packet.dashboard();
        assert_eq!(dashboard.total_surfaces, packet.surfaces.len() as u32);
        assert_eq!(
            dashboard.green_count + dashboard.yellow_count + dashboard.red_count,
            dashboard.total_surfaces
        );
        assert_eq!(dashboard.record_kind, M5_DYNAMIC_A11Y_DASHBOARD_RECORD_KIND);
    }
}

#[test]
fn detects_tampered_derived_fields() {
    let mut packet = canonical();
    // Flip a derived status without changing the underlying dimensions; the validator must
    // catch the inconsistency.
    packet.surfaces[0].certification_status = M5A11yCertificationStatus::Degraded;
    let violations = packet.validate();
    assert!(violations.contains(&M5CertificationViolation::DerivedRowInconsistent));
}

#[test]
fn detects_vocabulary_drift() {
    let mut packet = canonical();
    packet
        .vocabulary_set
        .proof_dimensions
        .push("bogus".to_owned());
    assert!(packet
        .validate()
        .contains(&M5CertificationViolation::VocabularySetDrift));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_dynamic_a11y_certification_export()
        .expect("checked certification export validates");
    assert_eq!(
        from_disk,
        canonical(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_dashboard_matches_seed() {
    let from_disk = current_stable_m5_dynamic_a11y_dashboard().expect("checked dashboard parses");
    assert_eq!(
        from_disk,
        canonical().dashboard(),
        "checked dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_drill_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-dynamic-a11y-certification/stale_proof_retest_pending.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-dynamic-a11y-certification/regression_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-dynamic-a11y-certification/waived_narrowed.json"
        )),
    ] {
        let packet: M5DynamicA11yCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = canonical().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}

#[test]
fn markdown_summary_names_surfaces_and_waivers() {
    let summary = seeded_m5_dynamic_a11y_certification_waived_narrowed().render_markdown_summary();
    assert!(summary.contains("Assistive-Tech Certification"));
    assert!(summary.contains("waiver:dense-collection-announcement-coverage"));
    assert!(summary.contains("certification:dense_collection"));
}
