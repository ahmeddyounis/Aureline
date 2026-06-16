//! Inline unit tests binding the typed register to the checked-in artifact and exercising
//! scan/surface parity, per-axis narrowing, the no-mask invariant, and the promotion verdict
//! against mutated copies.

use super::*;

fn register() -> ReleaseAuthorityContinuityRegister {
    current_m5_release_authority_continuity().expect("checked-in register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_RELEASE_AUTHORITY_CONTINUITY_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_RELEASE_AUTHORITY_CONTINUITY_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.records.is_empty());
}

#[test]
fn every_authority_lane_is_exercised() {
    let r = register();
    for lane in AuthorityLane::ALL {
        assert!(
            !r.records_of_lane(lane).is_empty(),
            "authority lane {} must have at least one record",
            lane.as_str()
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
    let states: BTreeSet<ContinuityState> = r.records.iter().map(|x| x.continuity_state).collect();
    assert!(states.contains(&ContinuityState::Cleared));
    assert!(
        states.len() >= 6,
        "expected several distinct continuity states, not one global flag"
    );
    let reasons: BTreeSet<ContinuityReason> = r
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
fn single_owner_and_owner_gaps_are_first_class_not_masked() {
    let r = register();
    // A single-owner lane narrows on the backup axis even with a raised escalation, proving a green
    // authority card can't mask the single-person system.
    let single = r.records.iter().any(|rec| {
        rec.is_single_owner() && rec.continuity_state == ContinuityState::NarrowedBackup
    });
    assert!(
        single,
        "expected a single-owner lane narrowing on the backup axis"
    );
    // A lane left without a named primary owner still narrows.
    let vacant = r.records.iter().any(|rec| rec.owner_vacant());
    assert!(
        vacant,
        "expected a primary-owner-vacant lane narrowing on the owner axis"
    );
    for rec in &r.records {
        if rec.is_single_owner() {
            assert!(rec.has_active_reason(ContinuityReason::BackupOwnerMissing));
        }
        if rec.owner_vacant() {
            assert!(rec.has_active_reason(ContinuityReason::PrimaryOwnerVacant));
        }
    }
}

#[test]
fn critical_lanes_require_split_authority_and_escalation() {
    let r = register();
    let critical: Vec<&AuthorityContinuityRecord> =
        r.records.iter().filter(|x| x.is_critical()).collect();
    assert!(!critical.is_empty(), "expected at least one critical lane");
    for rec in critical {
        assert!(rec.requires_split_authority());
        assert!(rec.requires_escalation());
        if rec.split_authority_unmet() || rec.escalation_missing() {
            assert_eq!(rec.continuity_state, ContinuityState::NarrowedAuthority);
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
fn continuity_failure_holds_promotion_inherited_does_not() {
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
    rec.active_reasons
        .push(ContinuityReason::OwnerSignoffMissing);
    assert!(r
        .validate()
        .iter()
        .any(|x| matches!(x, RegisterViolation::ClearedWithActiveReason { .. })));
}

#[test]
fn validate_flags_a_hidden_backup_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared() && x.is_critical())
        .expect("a cleared critical record exists");
    // Open a single-owner gap without narrowing on it: the gap must surface its reason.
    rec.backup_coverage.backup_state = BackupState::SingleOwner;
    rec.backup_coverage.backup_owner_count = 0;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: ContinuityReason::BackupOwnerMissing,
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
        .find(|x| x.continuity_state.is_narrowed())
        .expect("a narrowed record exists");
    // Pretend the governance surface is green over a scan that found gaps.
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
fn validate_flags_a_critical_lane_without_split_authority() {
    let mut r = register();
    // Find a critical record whose split authority is enforced and drop it to unmet without
    // narrowing.
    if let Some(rec) = r.records.iter_mut().find(|x| {
        x.is_critical() && x.split_authority.split_state == SplitAuthorityState::Satisfied
    }) {
        rec.split_authority.split_state = SplitAuthorityState::Unmet;
        rec.split_authority.distinct_authorities = 1;
        assert!(r.validate().iter().any(|x| matches!(
            x,
            RegisterViolation::GapWithoutReason {
                reason: ContinuityReason::SplitAuthorityUnmet,
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
