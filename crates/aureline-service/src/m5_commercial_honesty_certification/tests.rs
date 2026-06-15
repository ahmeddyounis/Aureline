//! Tests for the frozen commercial honesty-certification packet.

use super::*;

fn packet() -> HonestyCertificationPacket {
    canonical_stable_honesty_certification_packet()
}

#[test]
fn canonical_packet_validates_clean() {
    let p = packet();
    let violations = p.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn checked_in_packet_matches_canonical_builder() {
    let stable = current_stable_honesty_certification_packet()
        .expect("checked-in packet parses and validates");
    assert_eq!(
        stable,
        packet(),
        "the checked-in artifact drifted from the canonical builder; regenerate it with the dump example"
    );
}

#[test]
fn certification_rides_real_backing_consumers() {
    let p = packet();
    let violations = p.cross_check_backing_consumers();
    assert!(
        violations.is_empty(),
        "backing-consumer cross-check failed: {violations:?}"
    );
}

#[test]
fn every_dimension_and_drill_is_covered() {
    let p = packet();
    assert_eq!(p.rows.len(), HonestyDimension::ALL.len());
    assert!(p.inspection.dimension_vocab_complete);
    assert!(p.inspection.drill_vocab_complete);
    assert_eq!(p.inspection.drills_exercised, CertificationDrill::ALL.len());
    for dim in HonestyDimension::ALL {
        let r = p.row(dim).expect("dimension has a row");
        assert_eq!(r.backing_consumer, dim.backing_consumer());
        let present: std::collections::BTreeSet<_> = r.drills.iter().map(|d| d.drill).collect();
        let required: std::collections::BTreeSet<_> =
            required_drills(dim).iter().copied().collect();
        assert_eq!(
            present, required,
            "{dim:?} drills must match its required set"
        );
    }
}

#[test]
fn baseline_packet_is_fully_certified() {
    let p = packet();
    assert!(p.inspection.fully_certified);
    assert_eq!(p.inspection.narrowed_row_count, 0);
    assert_eq!(
        p.inspection.certified_row_count,
        HonestyDimension::ALL.len()
    );
    assert_eq!(p.inspection.local_safe_only_row_count, 0);
    for r in &p.rows {
        assert_eq!(r.effective_certified_claim, MarketedClaim::ManagedFull);
        assert!(r.recovery_cue.is_none());
        assert!(r.narrowing_reasons.is_empty());
    }
}

#[test]
fn every_row_keeps_a_non_empty_local_safe_baseline() {
    let p = packet();
    assert!(p.inspection.all_rows_local_safe_backed);
    for r in &p.rows {
        assert!(!r.local_safe_baseline.is_empty());
        assert!(r.local_safe_baseline.iter().all(|s| !s.trim().is_empty()));
    }
}

#[test]
fn certification_is_not_vendor_managed_online_only() {
    let p = packet();
    assert!(p.inspection.certifies_self_host_or_offline);
    assert!(p.inspection.addresses_all_deployment_profiles);
    // Every row partitions all five deployment profiles between certified and
    // not-offered, so the self-host, air-gapped, and mirror profiles are always
    // addressed.
    for r in &p.rows {
        let mut addressed: Vec<_> = r
            .certified_profiles
            .iter()
            .chain(r.not_offered_profiles.iter())
            .copied()
            .collect();
        addressed.sort();
        addressed.dedup();
        assert_eq!(addressed.len(), DeploymentProfile::ALL.len());
    }
    // At least one dimension certifies in a self-host profile.
    assert!(p
        .rows
        .iter()
        .any(|r| r.certifies_in(DeploymentProfile::SelfHosted)));
}

#[test]
fn commercial_boundary_certifies_air_gapped() {
    let p = packet();
    let boundary = p
        .row(HonestyDimension::CommercialBoundaryHonesty)
        .expect("boundary row present");
    assert!(boundary.certifies_in(DeploymentProfile::AirGapped));
    assert!(boundary.not_offered_profiles.is_empty());
}

#[test]
fn a_failed_drill_narrows_only_its_row() {
    let mut p = packet();
    let narrowed = p.narrow_for_drill_failure(
        HonestyDimension::MeteringHonesty,
        CertificationDrill::FailClosedManagedAction,
    );
    assert!(narrowed);
    // The metering row narrows; its claim drops below the declared full claim.
    let metering = p.row(HonestyDimension::MeteringHonesty).unwrap();
    assert_eq!(
        metering.effective_certified_claim,
        MarketedClaim::ManagedNarrowed
    );
    assert!(metering
        .narrowing_reasons
        .contains(&CertificationDrill::FailClosedManagedAction));
    assert!(metering.recovery_cue.is_some());
    // The local-safe baseline is preserved even when narrowed.
    assert!(!metering.local_safe_baseline.is_empty());
    // Other rows are untouched.
    let entitlement = p.row(HonestyDimension::EntitlementHonesty).unwrap();
    assert_eq!(
        entitlement.effective_certified_claim,
        MarketedClaim::ManagedFull
    );
    // Inspection reflects the single narrowing.
    assert!(!p.inspection.fully_certified);
    assert_eq!(p.inspection.narrowed_row_count, 1);
    // The narrowed packet still validates (derived values recomputed).
    assert!(p.validate().is_empty());
}

#[test]
fn stale_evidence_narrows_even_a_certified_drill() {
    let mut p = packet();
    // Manually mark a forecast drill's evidence stale and recompute.
    {
        let row = p
            .rows
            .iter_mut()
            .find(|r| r.dimension == HonestyDimension::ForecastHonesty)
            .unwrap();
        let drill = row
            .drills
            .iter_mut()
            .find(|d| d.drill == CertificationDrill::ExportRightsValidation)
            .unwrap();
        assert_eq!(drill.grade, DrillGrade::Certified);
        drill.evidence_status = BoundaryEvidenceStatus::Stale;
    }
    p.recompute();
    let forecast = p.row(HonestyDimension::ForecastHonesty).unwrap();
    assert_eq!(
        forecast.effective_certified_claim,
        MarketedClaim::ManagedNarrowed,
        "stale evidence must narrow the row even though the drill graded certified"
    );
    assert!(p.validate().is_empty());
}

#[test]
fn missing_evidence_drops_to_local_safe_only() {
    let mut p = packet();
    {
        let row = p
            .rows
            .iter_mut()
            .find(|r| r.dimension == HonestyDimension::ChargebackHonesty)
            .unwrap();
        let drill = row
            .drills
            .iter_mut()
            .find(|d| d.drill == CertificationDrill::ChargebackScopeExportCheck)
            .unwrap();
        drill.evidence_status = BoundaryEvidenceStatus::Missing;
    }
    p.recompute();
    let chargeback = p.row(HonestyDimension::ChargebackHonesty).unwrap();
    assert_eq!(
        chargeback.effective_certified_claim,
        MarketedClaim::LocalSafeOnly
    );
    assert_eq!(p.inspection.local_safe_only_row_count, 1);
    assert!(p.validate().is_empty());
}

#[test]
fn validate_flags_a_tampered_effective_claim() {
    let mut p = packet();
    p.rows[0].effective_certified_claim = MarketedClaim::LocalSafeOnly;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| v.field == "row.effective_certified_claim"));
}

#[test]
fn validate_flags_a_collapsed_deployment_profile() {
    let mut p = packet();
    // Drop the air-gapped profile from the boundary row so it is no longer addressed.
    {
        let row = p
            .rows
            .iter_mut()
            .find(|r| r.dimension == HonestyDimension::CommercialBoundaryHonesty)
            .unwrap();
        row.certified_profiles
            .retain(|pf| *pf != DeploymentProfile::AirGapped);
    }
    p.inspection = HonestyCertificationInspection::derive(&p.rows, &p.surface_bindings);
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| v.field == "row.certified_profiles"));
}

#[test]
fn validate_flags_a_not_applicable_drill_with_stale_evidence() {
    let mut p = packet();
    {
        let drill = &mut p.rows[0].drills[0];
        drill.grade = DrillGrade::NotApplicable;
        drill.evidence_status = BoundaryEvidenceStatus::Stale;
        drill.claim_cap = CertificationDrillResult::derive_cap(drill.grade, drill.evidence_status);
    }
    p.recompute();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| v.field == "drill.evidence_status"));
}

#[test]
fn all_surfaces_bind_and_release_center_narrows() {
    let p = packet();
    assert!(p.inspection.surface_coverage_complete);
    assert_eq!(p.surface_bindings.len(), CertificationSurface::ALL.len());
    let release = p
        .surface_bindings
        .iter()
        .find(|b| b.surface == CertificationSurface::ReleaseCenter)
        .unwrap();
    assert!(release.narrows_on_failure);
    assert!(release.projects_effective_claim);
    // Diagnostics is read-only on the verdict; it does not narrow.
    let diagnostics = p
        .surface_bindings
        .iter()
        .find(|b| b.surface == CertificationSurface::Diagnostics)
        .unwrap();
    assert!(!diagnostics.narrows_on_failure);
}

#[test]
fn source_refs_cite_schema_and_every_backing_artifact() {
    let p = packet();
    assert!(p
        .source_refs
        .contains(&HONESTY_CERTIFICATION_SCHEMA_REF.to_owned()));
    for consumer in [
        BackingConsumer::CommercialControlPlane,
        BackingConsumer::EntitlementSummary,
        BackingConsumer::UsageForecastViews,
        BackingConsumer::ChargebackScopeViews,
        BackingConsumer::MeteringDegradationRules,
        BackingConsumer::OffboardingCards,
        BackingConsumer::CommercialBoundaryCards,
    ] {
        assert!(
            p.source_refs.contains(&consumer.artifact_path().to_owned()),
            "source_refs must cite {}",
            consumer.artifact_path()
        );
    }
}
