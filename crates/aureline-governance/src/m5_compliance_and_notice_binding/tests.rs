//! Inline unit tests binding the typed register to the checked-in artifact and
//! exercising scan/surface parity, per-axis narrowing, the no-mask invariant, and the
//! promotion verdict against mutated copies.

use super::*;

fn register() -> ComplianceRegister {
    current_m5_compliance_and_notice_binding().expect("checked-in register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_COMPLIANCE_AND_NOTICE_BINDING_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_COMPLIANCE_AND_NOTICE_BINDING_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.records.is_empty());
}

#[test]
fn every_family_has_an_artifact_family_record() {
    let r = register();
    for family in M5Family::ALL {
        let rec = r.artifact_family_record(family).unwrap_or_else(|| {
            panic!(
                "family {} must have an artifact-family record",
                family.as_str()
            )
        });
        assert_eq!(rec.scope_kind, ScopeKind::ArtifactFamily);
    }
}

#[test]
fn every_record_declares_all_control_dimensions() {
    let r = register();
    for rec in &r.records {
        for dimension in ControlDimension::ALL {
            let count = rec
                .controls
                .iter()
                .filter(|c| c.dimension == dimension)
                .count();
            assert_eq!(
                count,
                1,
                "record {} must declare control {} exactly once",
                rec.record_id,
                dimension.as_str()
            );
        }
    }
}

#[test]
fn states_are_per_axis_not_one_global_flag() {
    let r = register();
    let states: BTreeSet<ComplianceState> = r.records.iter().map(|x| x.compliance_state).collect();
    assert!(states.contains(&ComplianceState::Cleared));
    assert!(
        states.len() >= 4,
        "expected several distinct compliance states"
    );
    let reasons: BTreeSet<ComplianceReason> = r
        .records
        .iter()
        .flat_map(|x| x.active_reasons.iter().copied())
        .collect();
    assert!(!reasons.is_empty(), "narrowed records must name reasons");
}

#[test]
fn scan_and_surface_agree_on_every_record() {
    let r = register();
    for rec in &r.records {
        assert!(
            rec.scan_surface_agree(),
            "record {} scan and surface postures must agree",
            rec.record_id
        );
        // A green surface may never sit over a gap.
        assert_eq!(
            rec.surface_posture,
            rec.computed_posture(),
            "record {} surface posture must reflect its gaps",
            rec.record_id
        );
    }
}

#[test]
fn notice_and_licensing_gaps_are_first_class_not_masked_by_sbom() {
    let r = register();
    // Some record carries a green SBOM (present + bound) while still narrowing on a
    // notice or licensing gap — proving the SBOM badge cannot mask the gap.
    let masked = r.records.iter().any(|rec| {
        rec.sbom.spdx_primary_present
            && rec.sbom.binding_state == SbomBindingState::Bound
            && (rec.notices.is_partial()
                || rec.notices.is_missing()
                || rec.licensing.coverage_incomplete()
                || rec.licensing.exception_undocumented())
    });
    assert!(
        masked,
        "expected a record with a present, bound SBOM that still narrows on a notice/licensing gap"
    );
    for rec in &r.records {
        if rec.notices.is_partial() {
            assert!(rec.has_active_reason(ComplianceReason::NoticeInventoryPartial));
        }
        if rec.licensing.coverage_incomplete() {
            assert!(rec.has_active_reason(ComplianceReason::LicensingCoverageIncomplete));
        }
    }
}

#[test]
fn no_record_publishes_wider_than_it_declares() {
    let r = register();
    for rec in &r.records {
        assert!(
            rec.effective_label.rank() <= rec.declared_label.rank(),
            "record {} effective label is wider than declared",
            rec.record_id
        );
        if rec.compliance_state.is_narrowed() {
            assert!(
                !rec.effective_label.is_at_or_above_cutline(),
                "narrowed record {} must drop below the cutline",
                rec.record_id
            );
        }
    }
}

#[test]
fn summary_and_parity_match_records() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(r.scan_surface_parity, r.computed_scan_surface_parity());
    assert_eq!(
        r.summary.records_cleared + r.summary.records_narrowed + r.summary.state_withdrawn,
        r.records.len()
    );
}

#[test]
fn reuse_projection_covers_every_record() {
    let r = register();
    let projection = r.reuse_projection();
    assert_eq!(projection.len(), r.records.len());
    for projected in &projection {
        assert!(
            !projected.surfaces.is_empty(),
            "projected record {} must carry reuse surfaces",
            projected.record_id
        );
    }
}

#[test]
fn compliance_layer_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_decision());
    let blocking = r.computed_blocking_record_ids();
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
        assert!(rec.compliance_state.is_narrowed());
    }
    // An inherited (below-cutline) or waived narrowing is gated upstream.
    for rec in &r.records {
        if rec.compliance_state.is_narrowed()
            && (!rec.declares_at_or_above_cutline() || rec.is_waived())
        {
            assert!(
                !blocking.contains(&rec.record_id),
                "inherited/waived narrowing on {} must not hold promotion",
                rec.record_id
            );
        }
    }
}

#[test]
fn validate_flags_a_cleared_record_with_a_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    rec.active_reasons
        .push(ComplianceReason::OwnerSignoffMissing);
    assert!(r
        .validate()
        .iter()
        .any(|x| matches!(x, RegisterViolation::ClearedWithActiveReason { .. })));
}

#[test]
fn validate_flags_a_hidden_licensing_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    // Open a licensing gap without narrowing on it: the gap must surface its reason.
    rec.licensing.files_spdx_covered = rec.licensing.files_spdx_covered.saturating_sub(1);
    rec.licensing.files_total += 1;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: ComplianceReason::LicensingCoverageIncomplete,
            ..
        }
    )));
}

#[test]
fn validate_flags_a_green_surface_over_a_gapped_scan() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.compliance_state.is_narrowed())
        .expect("a narrowed record exists");
    // Pretend the user/admin surface is clean over a scan that found gaps.
    rec.surface_posture = CompliancePosture::Clear;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::ScanSurfaceDisagreement { .. }
            | RegisterViolation::PostureMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_narrowed_record_above_the_cutline() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.compliance_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_proceed_verdict_while_a_rule_fires() {
    let mut r = register();
    if r.computed_decision() == PublicationDecision::Hold {
        r.publication.decision = PublicationDecision::Proceed;
        assert!(r
            .validate()
            .iter()
            .any(|x| matches!(x, RegisterViolation::PublicationDecisionInconsistent)));
    }
}
