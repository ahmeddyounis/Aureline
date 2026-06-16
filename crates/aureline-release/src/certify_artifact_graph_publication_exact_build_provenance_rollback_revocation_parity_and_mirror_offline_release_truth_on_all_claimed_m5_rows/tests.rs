use super::*;

fn register() -> M5PublicationCertRegister {
    current_m5_publication_cert_register().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_PUBLICATION_CERT_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_PUBLICATION_CERT_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.rows.is_empty());
}

#[test]
fn covers_every_artifact_family_kind() {
    let r = register();
    for kind in M5ArtifactFamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "artifact family kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn every_family_covers_every_dimension() {
    let r = register();
    for row in &r.rows {
        for dimension in PublicationDimension::ALL {
            assert!(
                row.cell(dimension).is_some(),
                "family {} must cover dimension {}",
                row.entry_id,
                dimension.as_str()
            );
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_family() {
    let r = register();
    assert!(!r.release_blocking_family_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .iter()
        .map(|row| row.family_ref.as_str())
        .collect();
    for declared in &r.release_blocking_family_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking family"
        );
    }
}

#[test]
fn every_held_family_is_scoped_and_proves_mirror_parity() {
    let r = register();
    for row in r.rows_published_stable() {
        assert!(
            row.publish_target.is_scoped(),
            "held family {} must publish through a scoped publish target",
            row.entry_id
        );
        assert!(
            !row.publish_target.inherits_ambient_credentials,
            "held family {} must not inherit ambient credentials",
            row.entry_id
        );
        assert!(
            row.mirror_offline.fully_proven(),
            "held family {} must prove mirror/offline parity with current drill evidence",
            row.entry_id
        );
        assert!(
            row.disclosure.redaction_disclosed,
            "held family {} must disclose its redaction posture",
            row.entry_id
        );
        assert!(
            row.downgrade_automation.state.holds() && row.downgrade_automation.rollback_verified,
            "held family {} must carry defined+verified downgrade automation",
            row.entry_id
        );
    }
}

#[test]
fn summary_counts_match_families() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.entries_certified + r.summary.entries_narrowed,
        r.rows.len()
    );
}

#[test]
fn promotion_decision_matches_computed() {
    let r = register();
    assert_eq!(r.promotion.decision, r.computed_promotion_decision());
    assert_eq!(
        r.promotion.blocking_rule_ids,
        r.computed_blocking_rule_ids()
    );
    assert_eq!(
        r.promotion.blocking_claim_ids,
        r.computed_blocking_entry_ids()
    );
}

#[test]
fn every_narrowing_reason_has_a_stop_rule() {
    let r = register();
    let covered: BTreeSet<NarrowingReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn release_truth_evidence_narrows_real_rows() {
    // The canonical register exercises the guardrails on real rows: at least one
    // family inherits ambient credentials and at least one lacks current mirror
    // parity, and both must narrow below the cutline.
    let r = register();
    let ambient = r
        .rows
        .iter()
        .find(|row| row.has_active_reason(NarrowingReason::AmbientCredentialInherited))
        .expect("a family that inherits ambient credentials");
    assert!(!ambient.publishes_stable());

    let mirror = r
        .rows
        .iter()
        .find(|row| row.has_active_reason(NarrowingReason::MirrorOfflineDrillStale))
        .expect("a family without current mirror parity");
    assert!(!mirror.publishes_stable());
}

#[test]
fn validate_flags_a_held_family_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held family exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::RollbackPlanUnverified);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, M5PublicationCertViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_held_family_inheriting_ambient_credentials() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held family exists");
    row.publish_target.inherits_ambient_credentials = true;
    r.summary = r.computed_summary();
    let violations = r.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5PublicationCertViolation::HeldWithoutScopedPublishTarget { .. }
    )));
}

#[test]
fn validate_flags_a_held_family_without_mirror_parity() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held family exists");
    row.mirror_offline.offline_parity = false;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5PublicationCertViolation::HeldWithoutMirrorParity { .. }
    )));
}

#[test]
fn validate_flags_a_missing_dimension_cell() {
    let mut r = register();
    r.rows[0]
        .scorecard
        .retain(|cell| cell.dimension != PublicationDimension::MirrorOfflineParity);
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5PublicationCertViolation::DimensionIncompleteCoverage { .. }
    )));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = match r.promotion.decision {
        PromotionDecision::Hold => PromotionDecision::Proceed,
        PromotionDecision::Proceed => PromotionDecision::Hold,
    };
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5PublicationCertViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn export_projection_mirrors_families() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, proj) in r.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.disclosure.trust_tier, proj.trust_tier);
        assert_eq!(
            row.publish_target.inherits_ambient_credentials,
            proj.inherits_ambient_credentials
        );
        assert_eq!(row.mirror_offline.drill_state, proj.mirror_drill_state);
        assert_eq!(row.downgrade_automation.state, proj.automation_state);
    }
}
