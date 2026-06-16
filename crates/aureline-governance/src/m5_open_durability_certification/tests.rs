//! Inline unit tests binding the typed register to the checked-in artifact and exercising
//! scan/surface parity, per-axis narrowing, the no-mask invariant, the headline guardrails, and the
//! promotion verdict against mutated copies.

use super::*;

fn register() -> OpenDurabilityCertificationRegister {
    current_m5_open_durability_certification().expect("checked-in register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_OPEN_DURABILITY_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_OPEN_DURABILITY_CERTIFICATION_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.records.is_empty());
}

#[test]
fn every_row_kind_is_exercised() {
    let r = register();
    for kind in RowKind::ALL {
        assert!(
            !r.records_of_kind(kind).is_empty(),
            "row kind {} must have at least one record",
            kind.as_str()
        );
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
    let states: BTreeSet<CertificationState> =
        r.records.iter().map(|x| x.certification_state).collect();
    assert!(states.contains(&CertificationState::Certified));
    // Distinct narrowing axes coexist instead of collapsing into one pass/fail flag.
    assert!(states.contains(&CertificationState::NarrowedBoundary));
    assert!(states.contains(&CertificationState::NarrowedCompliance));
    assert!(states.contains(&CertificationState::NarrowedImport));
    assert!(states.contains(&CertificationState::NarrowedAuthority));
    assert!(states.contains(&CertificationState::NarrowedEmergency));
    assert!(states.contains(&CertificationState::NarrowedUpstream));
    assert!(states.contains(&CertificationState::NarrowedStale));
    let reasons: BTreeSet<CertificationReason> = r
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
fn the_three_guardrails_are_first_class_not_masked() {
    let r = register();
    // A hidden proprietary baseline narrows on the boundary axis and surfaces gaps.
    let hidden = r
        .records
        .iter()
        .find(|rec| rec.has_active_reason(CertificationReason::HiddenProprietaryBaseline))
        .expect("a hidden proprietary baseline exists");
    assert_eq!(
        hidden.certification_state,
        CertificationState::NarrowedBoundary
    );
    assert_eq!(hidden.surface_posture, Posture::GapsFound);
    // An ownerless critical import narrows on the import axis.
    let ownerless = r
        .records
        .iter()
        .find(|rec| rec.has_active_reason(CertificationReason::OwnerlessCriticalImport))
        .expect("an ownerless critical import exists");
    assert_eq!(
        ownerless.certification_state,
        CertificationState::NarrowedImport
    );
    // A single-person emergency authority narrows on the authority axis.
    let single = r
        .records
        .iter()
        .find(|rec| rec.has_active_reason(CertificationReason::SinglePersonEmergencyAuthority))
        .expect("a single-person emergency authority exists");
    assert_eq!(
        single.certification_state,
        CertificationState::NarrowedAuthority
    );
    // The guardrail counts are tracked.
    assert!(r.summary.hidden_proprietary_baseline_gaps > 0);
    assert!(r.summary.ownerless_critical_import_gaps > 0);
    assert!(r.summary.single_person_authority_gaps > 0);
}

#[test]
fn every_axis_carries_a_gap_somewhere() {
    let r = register();
    assert!(r.summary.boundary_gaps > 0, "must record a boundary gap");
    assert!(
        r.summary.compliance_gaps > 0,
        "must record a compliance gap"
    );
    assert!(r.summary.import_gaps > 0, "must record an import gap");
    assert!(r.summary.authority_gaps > 0, "must record an authority gap");
    assert!(r.summary.emergency_gaps > 0, "must record an emergency gap");
    assert!(r.summary.upstream_gaps > 0, "must record an upstream gap");
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
        if rec.certification_state.is_narrowed() {
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
        r.summary.records_certified + r.summary.records_narrowed + r.summary.state_withdrawn,
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
fn certification_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_decision());
    let blocking = r.computed_blocking_record_ids();
    assert!(
        !blocking.is_empty(),
        "a certification failure on a still-stable row must hold promotion"
    );
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
        assert!(rec.certification_state.is_narrowed());
    }
    // An inherited (below-cutline) or waived narrowing is gated upstream.
    for rec in &r.records {
        if rec.certification_state.is_narrowed()
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
fn waived_and_inherited_narrowings_stay_visible() {
    let r = register();
    let blocking = r.computed_blocking_record_ids();
    // The waived boundary gap is narrowed and visible, but not held.
    let waived = r
        .record("cert-review-managed-release")
        .expect("waived record exists");
    assert_eq!(
        waived.certification_state,
        CertificationState::NarrowedBoundary
    );
    assert!(waived.is_waived());
    assert!(!blocking.contains(&waived.record_id));
    // The companion preview row already sits below the cutline (Beta): inherited.
    let beta = r
        .record("cert-companion-preview-ecosystem")
        .expect("beta record exists");
    assert!(beta.certification_state.is_narrowed());
    assert!(!beta.declares_at_or_above_cutline());
    assert!(!blocking.contains(&beta.record_id));
}

#[test]
fn stale_and_missing_proof_narrow_on_the_stale_axis() {
    let r = register();
    let stale = r
        .record("cert-managed_depth-ecosystem")
        .expect("stale-proof record exists");
    assert_eq!(stale.certification_state, CertificationState::NarrowedStale);
    assert_eq!(stale.proof_packet.slo_state, FreshnessSloState::Breached);
    let missing = r
        .record("cert-companion-release")
        .expect("missing-proof record exists");
    assert_eq!(
        missing.certification_state,
        CertificationState::NarrowedStale
    );
    assert_eq!(missing.proof_packet.slo_state, FreshnessSloState::Missing);
}

#[test]
fn validate_flags_a_certified_record_with_a_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_certified())
        .expect("a certified record exists");
    rec.active_reasons
        .push(CertificationReason::OwnerSignoffMissing);
    assert!(r
        .validate()
        .iter()
        .any(|x| matches!(x, RegisterViolation::CertifiedWithActiveReason { .. })));
}

#[test]
fn validate_flags_a_hidden_boundary_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_certified())
        .expect("a certified record exists");
    // Open a boundary gap without narrowing on it: the gap must surface its reason.
    rec.boundary.state = BoundaryEvidenceState::Unpublished;
    rec.boundary.manifest_published = false;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: CertificationReason::BoundaryManifestMissing,
            ..
        } | RegisterViolation::ControlStateInconsistent { .. }
    )));
}

#[test]
fn validate_flags_a_single_person_authority_hidden() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_certified())
        .expect("a certified record exists");
    // The headline guardrail: a single-person emergency authority must surface its reason.
    rec.authority.state = AuthorityEvidenceState::SinglePersonAuthority;
    rec.authority.available_distinct_humans = 1;
    rec.authority.backup_present = false;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: CertificationReason::SinglePersonEmergencyAuthority,
            ..
        } | RegisterViolation::ControlStateInconsistent { .. }
    )));
}

#[test]
fn validate_flags_an_axis_fact_inconsistency() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_certified())
        .expect("a certified record exists");
    // A "published" state over an unpublished fact is a fact inconsistency.
    rec.boundary.manifest_published = false;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::AxisFactInconsistent {
            dimension: ControlDimension::BoundaryManifest,
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
        .find(|x| x.certification_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.surface_posture = Posture::Clear;
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
        .find(|x| x.certification_state.is_narrowed())
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
