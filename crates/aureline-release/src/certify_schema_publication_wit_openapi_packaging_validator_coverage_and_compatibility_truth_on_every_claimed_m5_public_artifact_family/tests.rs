//! Inline unit tests for the typed M5 public-contract certification register.

use super::*;

fn register() -> M5PublicContractCertificationRegister {
    current_m5_public_contract_certification_register().expect("checked-in register parses")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_PUBLIC_CONTRACT_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_PUBLIC_CONTRACT_CERTIFICATION_RECORD_KIND);
    assert_eq!(r.register_id, M5_PUBLIC_CONTRACT_CERTIFICATION_REGISTER_ID);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_recomputes_from_rows() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert!(r.summary.total_families > 0);
    assert_eq!(
        r.summary.certified_families + r.summary.narrowed_families + r.summary.withheld_families,
        r.rows.len(),
        "every family is certified, narrowed, or withheld"
    );
}

#[test]
fn every_family_carries_one_pillar_per_kind() {
    // The acceptance anchor: every claimed family is certified on the published contract
    // form, lifecycle metadata, example corpus, validator coverage, compatibility report, and
    // release-graph linkage.
    let r = register();
    for row in &r.rows {
        let kinds: Vec<PillarKind> = row.pillars.iter().map(|p| p.pillar_kind).collect();
        assert_eq!(
            kinds,
            PillarKind::ALL.to_vec(),
            "{} must carry one pillar per kind in order",
            row.family_id
        );
        for pillar in &row.pillars {
            assert!(
                !pillar.certifying_artifact_ref.is_empty(),
                "{}/{:?} names its certifying artifact",
                row.family_id,
                pillar.pillar_kind
            );
        }
    }
}

#[test]
fn certification_state_and_blocker_follow_the_pillars() {
    let r = register();
    for row in &r.rows {
        assert_eq!(
            row.certification_state,
            row.computed_certification_state(),
            "{} certification state must follow its pillars",
            row.family_id
        );
        assert_eq!(
            row.blocker.decision,
            row.computed_blocker_decision(),
            "{} blocker decision must follow its state",
            row.family_id
        );
    }
}

#[test]
fn certified_label_never_greener_than_the_public_claim() {
    // The guardrail: a family may never certify a greener label than its public claim.
    let r = register();
    for row in &r.rows {
        assert!(
            !row.certified_label_greener_than_claim(),
            "{} certifies a greener label ({:?}) than its public claim ({:?})",
            row.family_id,
            row.certified_label,
            row.claim_label
        );
    }
}

#[test]
fn release_blocking_family_missing_a_required_pillar_withholds_and_holds() {
    // The acceptance anchor: any release-blocking family missing a required contract pillar
    // withholds certification and holds promotion.
    let r = register();
    let task_event = r
        .row("task_event_envelope")
        .expect("task_event_envelope is registered");
    assert_eq!(
        task_event.certification_state,
        CertificationState::Withheld,
        "a release-blocking family with a missing required pillar withholds certification"
    );
    assert!(task_event.release_blocking);
    // The marketed claim is stable; certification narrows it to beta.
    assert_eq!(task_event.claim_label, LifecycleLabel::Stable);
    assert_eq!(task_event.certified_label, LifecycleLabel::Beta);
    assert_eq!(task_event.blocker.decision, BlockerDecision::Hold);
    assert!(task_event
        .blocker
        .blocking_pillar_kinds
        .contains(&PillarKind::CompatibilityReport));
    assert!(r.holds_promotion());
    assert_eq!(r.promotion.decision, DecisionState::Hold);
    assert!(r
        .promotion
        .blocking_family_ids
        .contains(&"task_event_envelope".to_string()));
}

#[test]
fn a_clean_family_certifies_its_public_claim() {
    let r = register();
    let cmd = r
        .row("command_descriptors")
        .expect("command_descriptors is registered");
    assert_eq!(cmd.certification_state, CertificationState::Certified);
    assert_eq!(cmd.certified_label, cmd.claim_label);
    assert!(cmd.active_certification_reasons.is_empty());
    assert!(cmd.stop_actions.is_empty());
    assert!(cmd
        .pillars
        .iter()
        .all(|p| p.evidence_state == EvidenceState::Current));
    assert_eq!(cmd.blocker.decision, BlockerDecision::Clear);
}

#[test]
fn at_least_one_family_narrows_below_its_marketed_claim() {
    // The closeout requires automatic narrowing: a marketed claim whose contract packages are
    // missing must certify below the marketed label.
    let r = register();
    assert!(
        r.summary.families_narrowed_below_claim >= 1,
        "the closeout must demonstrate at least one narrowed claim"
    );
    let narrowed: Vec<&Row> = r
        .rows
        .iter()
        .filter(|row| row.certified_label.rank() > row.claim_label.rank())
        .collect();
    assert!(narrowed
        .iter()
        .any(|row| row.family_id == "task_event_envelope"));
}

#[test]
fn resolve_and_project_round_trip() {
    let r = register();
    let (label, state, decision) = r
        .resolve_certification("service_optional_api")
        .expect("service_optional_api resolves");
    assert_eq!(label, LifecycleLabel::Stable);
    assert_eq!(state, CertificationState::Certified);
    assert_eq!(decision, BlockerDecision::Clear);
    assert!(r.resolve_certification("not_a_family").is_none());

    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    assert_eq!(projection.decision, r.promotion.decision);
    for prow in &projection.rows {
        let src = r
            .row(&prow.family_id)
            .expect("projection family is in the model");
        assert_eq!(prow.certified_label, src.certified_label);
        assert_eq!(prow.certification_state, src.certification_state);
    }
}

#[test]
fn upstream_join_refs_are_present_for_every_row() {
    let r = register();
    for row in &r.rows {
        assert!(!row.proof.health_row_ref.is_empty());
        assert!(!row.proof.matrix_row_ref.is_empty());
        assert!(!row.proof.catalog_entry_ref.is_empty());
        assert!(!row.proof.contract_form_catalog_ref.is_empty());
    }
}

#[test]
fn duplicate_family_id_is_rejected() {
    let mut r = register();
    let dup = r.rows[0].clone();
    r.rows.push(dup);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.duplicate_family_id"));
}

#[test]
fn missing_pillar_is_rejected() {
    let mut r = register();
    r.rows[0].pillars.pop();
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.pillar_coverage"));
}

#[test]
fn greener_certified_label_is_rejected() {
    let mut r = register();
    r.rows[0].claim_label = LifecycleLabel::Beta;
    r.rows[0].certified_label = LifecycleLabel::Stable;
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.claim_parity"));
}

#[test]
fn relabeling_a_withheld_family_certified_is_rejected() {
    let mut r = register();
    let idx = r
        .rows
        .iter()
        .position(|row| row.certification_state == CertificationState::Withheld)
        .expect("a withheld row");
    r.rows[idx].certification_state = CertificationState::Certified;
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.certification_state"));
}

#[test]
fn summary_drift_is_rejected() {
    let mut r = register();
    r.summary.total_families += 1;
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "summary.count_mismatch"));
}
