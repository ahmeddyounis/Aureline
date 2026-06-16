//! Inline unit tests binding the typed register to the checked-in artifact and
//! exercising manifest/surface parity, per-axis narrowing, the no-mask invariant, and the
//! promotion verdict against mutated copies.

use super::*;

fn register() -> ImportRegister {
    current_m5_import_provenance_and_fork_review().expect("checked-in register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_SCHEMA_VERSION
    );
    assert_eq!(
        r.record_kind,
        M5_IMPORT_PROVENANCE_AND_FORK_REVIEW_RECORD_KIND
    );
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.records.is_empty());
}

#[test]
fn every_import_kind_is_exercised() {
    let r = register();
    for kind in ImportKind::ALL {
        assert!(
            !r.records_of_kind(kind).is_empty(),
            "import kind {} must have at least one record",
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
    let states: BTreeSet<ImportState> = r.records.iter().map(|x| x.import_state).collect();
    assert!(states.contains(&ImportState::Cleared));
    assert!(
        states.len() >= 5,
        "expected several distinct import states, not one global flag"
    );
    let reasons: BTreeSet<ImportReason> = r
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
        // A clean surface may never sit over a gap.
        assert_eq!(
            rec.surface_posture,
            rec.computed_posture(),
            "record {} surface posture must reflect its gaps",
            rec.record_id
        );
    }
}

#[test]
fn generator_and_ownership_gaps_are_first_class_not_masked() {
    let r = register();
    // A checked-in generated artifact still narrows when its generator identity is buried,
    // proving a clean import card can't mask the gap.
    let gen_gap = r.records.iter().any(|rec| {
        rec.import_kind == ImportKind::GeneratedArtifact && rec.generator.identity_missing()
    });
    assert!(
        gen_gap,
        "expected a generated artifact narrowing on a buried generator identity"
    );
    // An import treated as "just build-time" still narrows when left ownerless.
    let ownerless = r.records.iter().any(|rec| rec.ownership.owner_missing());
    assert!(
        ownerless,
        "expected an ownerless import narrowing on the ownership axis"
    );
    for rec in &r.records {
        if rec.generator.identity_missing() {
            assert!(rec.has_active_reason(ImportReason::GeneratorIdentityMissing));
        }
        if rec.ownership.owner_missing() {
            assert!(rec.has_active_reason(ImportReason::UpdateOwnerMissing));
        }
    }
}

#[test]
fn long_lived_imports_require_a_decision() {
    let r = register();
    // Long-lived forks and single-source imports require a recorded sponsor/fork/replace
    // decision; a pending decision narrows on the divergence axis.
    let long_lived: Vec<&ImportRecord> =
        r.records.iter().filter(|x| x.requires_decision()).collect();
    assert!(
        !long_lived.is_empty(),
        "expected at least one long-lived fork or single-source import"
    );
    for rec in long_lived {
        if rec.decision_missing() {
            assert!(rec.has_active_reason(ImportReason::DecisionRecordMissing));
            assert_eq!(rec.import_state, ImportState::NarrowedDivergence);
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
        if rec.import_state.is_narrowed() {
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
    assert_eq!(
        r.manifest_surface_parity,
        r.computed_manifest_surface_parity()
    );
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
fn import_layer_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_decision());
    let blocking = r.computed_blocking_record_ids();
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
        assert!(rec.import_state.is_narrowed());
    }
    // An inherited (below-cutline) or waived narrowing is gated upstream.
    for rec in &r.records {
        if rec.import_state.is_narrowed()
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
    rec.active_reasons.push(ImportReason::OwnerSignoffMissing);
    assert!(r
        .validate()
        .iter()
        .any(|x| matches!(x, RegisterViolation::ClearedWithActiveReason { .. })));
}

#[test]
fn validate_flags_a_hidden_ownership_gap() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    // Open an ownership gap without narrowing on it: the gap must surface its reason.
    rec.ownership.ownership_state = OwnershipState::Ownerless;
    rec.ownership.update_owner_ref = String::new();
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: ImportReason::UpdateOwnerMissing,
            ..
        }
    )));
}

#[test]
fn validate_flags_a_clean_surface_over_a_gapped_scan() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.import_state.is_narrowed())
        .expect("a narrowed record exists");
    // Pretend the user/admin surface is clean over a scan that found gaps.
    rec.surface_posture = Posture::Clear;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::ManifestScanSurfaceDisagreement { .. }
            | RegisterViolation::PostureMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_narrowed_record_above_the_cutline() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.import_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_buried_generator_identity() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared() && x.generator.applies)
        .expect("a cleared generated record exists");
    // Bury the generator identity without narrowing: the gap must surface its reason.
    rec.generator.generator_identity_present = false;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: ImportReason::GeneratorIdentityMissing,
            ..
        } | RegisterViolation::ControlStateInconsistent { .. }
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
