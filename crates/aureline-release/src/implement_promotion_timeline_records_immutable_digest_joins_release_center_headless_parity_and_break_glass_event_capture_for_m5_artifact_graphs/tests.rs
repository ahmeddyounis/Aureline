use super::*;

use crate::release_center_model::BreakGlassStateClass;

fn register() -> M5ArtifactGraphPromotionRegister {
    current_m5_artifact_graph_promotion_ledger().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_ARTIFACT_GRAPH_PROMOTION_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_ARTIFACT_GRAPH_PROMOTION_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "register must validate cleanly: {violations:#?}"
    );
    assert!(!r.rows.is_empty());
}

#[test]
fn embedded_json_matches_builder() {
    // The checked-in JSON must be exactly what the in-code builder produces, so
    // the embedded consumer and the artifact never drift.
    assert_eq!(register(), build_m5_artifact_graph_promotion_ledger());
}

#[test]
fn builder_validates_cleanly() {
    assert_eq!(
        build_m5_artifact_graph_promotion_ledger().validate(),
        Vec::new()
    );
}

#[test]
fn covers_every_family_kind() {
    let r = register();
    for kind in M5ArtifactFamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one ledger",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_candidate() {
    let r = register();
    assert!(!r.release_blocking_candidate_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .iter()
        .map(|row| row.candidate_ref.as_str())
        .collect();
    for declared in &r.release_blocking_candidate_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking ledger"
        );
    }
}

#[test]
fn register_narrows_at_least_one_family() {
    let r = register();
    assert!(
        !r.rows_narrowed().is_empty(),
        "the register must narrow at least one family below the cutline"
    );
}

#[test]
fn every_narrowing_reason_has_a_stop_rule() {
    let r = register();
    let covered: std::collections::BTreeSet<NarrowingReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn reconstructable_families_replay_the_same_history_across_flows() {
    // Acceptance: release-center and headless flows reconstruct the same promotion
    // history for a given M5 artifact graph.
    let r = register();
    for row in r.rows_reconstructable() {
        assert_eq!(row.reconstruction.parity_state, ParityState::Matched);
        assert!(row.reconstruction.history_digests_match());
        assert!(row.reconstruction.audit_replay_available());
        assert_eq!(
            row.reconstruction.reconstructed_step_ids,
            row.timeline_step_ids(),
            "the reconstructed history must equal the ordered timeline"
        );
    }
}

#[test]
fn every_promotion_step_binds_an_immutable_digest_in_the_node_set() {
    // Acceptance/guardrail: ordinary and break-glass steps alike are joined to the
    // affected node set by immutable digest; no mutable pointer stands in.
    let r = register();
    for row in r.rows_reconstructable() {
        assert_eq!(
            row.history_pointer_class,
            HistoryPointerClass::ImmutableGraphHistory
        );
        for step in &row.timeline {
            assert!(
                row.step_digests_resolve(step),
                "ledger {} step {} must bind digests in its node set",
                row.entry_id,
                step.timeline_step_id
            );
        }
    }
}

#[test]
fn break_glass_events_ride_the_same_timeline_model() {
    // Break-glass freezes, emergency publications, and out-of-band corrections are
    // captured in the same step model as ordinary promotions.
    let r = register();
    let total_break_glass: usize = r.rows.iter().map(|row| row.break_glass_steps().len()).sum();
    assert!(
        total_break_glass > 0,
        "the register must capture at least one break-glass step"
    );
    // A held ledger may carry a break-glass step only when it is reconciled and
    // still digest-bound (it never bypasses capture).
    for row in r.rows_reconstructable() {
        for step in row.break_glass_steps() {
            assert!(FamilyPromotionLedger::break_glass_step_reconciled(step));
            assert!(row.step_digests_resolve(step));
            assert!(FamilyPromotionLedger::step_capture_complete(step));
        }
    }
}

#[test]
fn audit_export_replays_who_what_when_evidence_and_window() {
    // Acceptance: audit and postmortem exports can replay who promoted what, when,
    // on which evidence, and with which reversible window.
    let r = register();
    let projection = r.support_export_projection();
    let reconstructable = projection
        .rows
        .iter()
        .find(|row| row.publishes_stable)
        .expect("a reconstructable ledger exists");
    assert!(!reconstructable.replay.is_empty());
    for entry in &reconstructable.replay {
        assert!(
            !entry.approving_actor_refs.is_empty(),
            "replay must record who promoted"
        );
        assert!(
            !entry.evidence_refs.is_empty(),
            "replay must record on which evidence"
        );
        assert!(
            !entry.digest_refs.is_empty(),
            "replay must record the immutable digests"
        );
        assert!(
            entry
                .reversible_window
                .as_ref()
                .map(|w| !w.trim().is_empty())
                .unwrap_or(false)
                || !entry.rollback_target_ref.is_empty(),
            "replay must record the reversible window or rollback target"
        );
    }
}

#[test]
fn summary_counts_match_rows() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.entries_reconstructable + r.summary.entries_narrowed,
        r.rows.len()
    );
    assert!(r.summary.total_break_glass_steps > 0);
}

#[test]
fn publication_decision_matches_computed() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_publication_decision());
    assert_eq!(
        r.publication.blocking_rule_ids,
        r.computed_blocking_rule_ids()
    );
    assert_eq!(
        r.publication.blocking_claim_ids,
        r.computed_blocking_entry_ids()
    );
}

#[test]
fn narrowed_family_surfaces_its_gaps_in_export() {
    let r = register();
    let projection = r.support_export_projection();
    let narrowed = projection
        .rows
        .iter()
        .find(|row| !row.publishes_stable)
        .expect("a narrowed family exists");
    assert!(
        !narrowed.active_narrowing_reasons.is_empty(),
        "a narrowed family must surface its narrowing reasons"
    );
    assert!(
        narrowed.history_pointer_class == HistoryPointerClass::MutableLatestPointer
            || narrowed.parity_state != ParityState::Matched
            || !narrowed.audit_replay_available,
        "a narrowed family must surface its concrete history gap"
    );
}

#[test]
fn export_projection_mirrors_rows() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, proj) in r.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.timeline.len(), proj.timeline_step_count);
        assert_eq!(row.break_glass_steps().len(), proj.break_glass_step_count);
    }
}

#[test]
fn validate_flags_a_reconstructable_family_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a reconstructable family exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::ProofPacketMissing);
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphPromotionViolation::HeldWithActiveGap { .. }
    )));
}

#[test]
fn validate_flags_a_mutable_pointer_without_reason() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a reconstructable family exists");
    row.history_pointer_class = HistoryPointerClass::MutableLatestPointer;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphPromotionViolation::HistoryGapWithoutReason {
            reason: NarrowingReason::MutableLatestPointer,
            ..
        }
    )));
}

#[test]
fn validate_flags_a_broken_reconstruction_on_a_held_family() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a reconstructable family exists");
    row.reconstruction.headless_history_digest = "sha256/tampered".to_owned();
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphPromotionViolation::HeldWithoutReconstruction { .. }
            | M5ArtifactGraphPromotionViolation::HistoryGapWithoutReason {
                reason: NarrowingReason::ReconstructionDivergent,
                ..
            }
    )));
}

#[test]
fn validate_flags_an_emergency_step_that_bypasses_digest_binding() {
    // Guardrail: an emergency flow may not bypass timeline capture or digest
    // binding. Stripping a break-glass step's digests must trip the guardrail.
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| {
            row.timeline
                .iter()
                .any(|step| step.break_glass.state_class == BreakGlassStateClass::Reconciled)
        })
        .expect("a ledger with a break-glass step exists");
    for step in row.timeline.iter_mut() {
        if step.break_glass.state_class == BreakGlassStateClass::Reconciled {
            step.digest_refs.clear();
        }
    }
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphPromotionViolation::BreakGlassBypassedCapture { .. }
    )));
}

#[test]
fn validate_flags_a_reconstructable_family_without_signoff() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a reconstructable family exists");
    row.owner_signoff.signed_off = false;
    row.owner_signoff.signed_at = None;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphPromotionViolation::HeldWithoutSignoff { .. }
    )));
}
