//! Inline unit tests binding the typed register to the checked-in artifact and exercising
//! scan/surface parity, per-axis narrowing, the no-mask invariant, and the promotion verdict against
//! mutated copies.

use super::*;

fn register() -> EmergencyResponseEvidenceRegister {
    current_m5_emergency_response_evidence().expect("checked-in register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_EMERGENCY_RESPONSE_EVIDENCE_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_EMERGENCY_RESPONSE_EVIDENCE_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.records.is_empty());
}

#[test]
fn every_packet_kind_is_exercised() {
    let r = register();
    for kind in PacketKind::ALL {
        assert!(
            !r.records_of_kind(kind).is_empty(),
            "packet kind {} must have at least one record",
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
    let states: BTreeSet<ResponseState> = r.records.iter().map(|x| x.continuity_state).collect();
    assert!(states.contains(&ResponseState::Cleared));
    assert!(
        states.len() >= 6,
        "expected several distinct response states, not one global flag"
    );
    let reasons: BTreeSet<ResponseReason> = r
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
fn mirror_and_offline_reach_gaps_are_first_class_not_masked() {
    let r = register();
    // A mirror that never received the evidence narrows on the distribution axis even with hosted
    // propagated, proving a green card can't mask a hosted-only delivery.
    let mirror = r.records.iter().any(|rec| {
        rec.has_active_reason(ResponseReason::MirrorPropagationIncomplete)
            && rec.continuity_state == ResponseState::NarrowedDistribution
    });
    assert!(
        mirror,
        "expected a mirror reach gap narrowing on the distribution axis"
    );
    let offline = r.records.iter().any(|rec| {
        rec.has_active_reason(ResponseReason::OfflineImportResponseMissing)
            && rec.continuity_state == ResponseState::NarrowedDistribution
    });
    assert!(
        offline,
        "expected an offline reach gap narrowing on the distribution axis"
    );
    for rec in &r.records {
        if rec.has_distribution_gap() {
            assert_eq!(rec.surface_posture, Posture::GapsFound);
        }
    }
}

#[test]
fn break_glass_actions_require_audit_and_reconciliation() {
    let r = register();
    let break_glass: Vec<&EmergencyResponseRecord> =
        r.records.iter().filter(|x| x.is_break_glass).collect();
    assert!(
        !break_glass.is_empty(),
        "expected at least one break-glass record"
    );
    for rec in &r.records {
        // Break-glass and high-severity actions must require reconciliation.
        if rec.is_break_glass || rec.is_high_severity() {
            assert!(rec.requires_reconciliation());
        }
        if rec.audit_markers_missing() {
            assert!(rec.has_active_reason(ResponseReason::AuditMarkersMissing));
            assert_eq!(rec.continuity_state, ResponseState::NarrowedAudit);
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
        if rec.continuity_state.is_narrowed() {
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
fn response_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_decision());
    let blocking = r.computed_blocking_record_ids();
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
        assert!(rec.continuity_state.is_narrowed());
    }
    // An inherited (below-cutline) or waived narrowing is gated upstream.
    for rec in &r.records {
        if rec.continuity_state.is_narrowed()
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
    rec.active_reasons.push(ResponseReason::OwnerSignoffMissing);
    assert!(r
        .validate()
        .iter()
        .any(|x| matches!(x, RegisterViolation::ClearedWithActiveReason { .. })));
}

#[test]
fn validate_flags_a_hidden_distribution_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    // Open a mirror reach gap without narrowing on it: the gap must surface its reason.
    if let Some(mirror) = rec
        .distribution_reach
        .channels
        .iter_mut()
        .find(|c| c.channel == DistributionChannel::Mirror)
    {
        mirror.claimed = true;
        mirror.state = ChannelState::Pending;
    }
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: ResponseReason::MirrorPropagationIncomplete,
            ..
        } | RegisterViolation::ControlStateInconsistent { .. }
    )));
}

#[test]
fn validate_flags_a_green_surface_over_a_gapped_scan() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.continuity_state.is_narrowed())
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
        .find(|x| x.continuity_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_break_glass_action_without_audit_markers() {
    let mut r = register();
    if let Some(rec) = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared() && x.is_break_glass)
    {
        rec.audit_trail.audit_markers_present = false;
        rec.audit_trail.audit_marker_ref = String::new();
        assert!(r.validate().iter().any(|x| matches!(
            x,
            RegisterViolation::GapWithoutReason {
                reason: ResponseReason::AuditMarkersMissing,
                ..
            } | RegisterViolation::ControlStateInconsistent { .. }
        )));
    }
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
