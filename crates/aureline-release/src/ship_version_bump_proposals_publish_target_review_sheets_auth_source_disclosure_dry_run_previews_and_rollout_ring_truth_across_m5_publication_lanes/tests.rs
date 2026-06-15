use super::*;

fn register() -> PublicationReviewRegister {
    current_publication_review_register().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, PUBLICATION_REVIEW_SCHEMA_VERSION);
    assert_eq!(r.record_kind, PUBLICATION_REVIEW_RECORD_KIND);
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
    assert_eq!(register(), build_publication_review_register());
}

#[test]
fn builder_validates_cleanly() {
    assert_eq!(build_publication_review_register().validate(), Vec::new());
}

#[test]
fn covers_every_lane_kind() {
    let r = register();
    for kind in M5ArtifactFamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "lane kind {} must have at least one sheet",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_lane() {
    let r = register();
    assert!(!r.release_blocking_lane_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .iter()
        .map(|row| row.lane_ref.as_str())
        .collect();
    for declared in &r.release_blocking_lane_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking sheet"
        );
    }
}

#[test]
fn register_narrows_at_least_one_lane() {
    let r = register();
    assert!(
        !r.rows_narrowed().is_empty(),
        "the register must narrow at least one lane below the cutline"
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
fn cleared_lanes_share_descriptor_and_diff_payload() {
    // Acceptance: human review and headless publication share the same
    // publish-target descriptor and diff payload on every published lane.
    let r = register();
    for row in r.rows_cleared() {
        assert_eq!(row.review_parity.parity_state, ParityState::Matched);
        assert!(row.review_parity.descriptor_digests_match());
        assert!(row.review_parity.diff_payload_digests_match());
    }
}

#[test]
fn cleared_lanes_disclose_non_ambient_auth_before_mutation() {
    // Acceptance: publication actions disclose auth source and target scope
    // before any mutation, and never inherit ambient credentials.
    let r = register();
    for row in r.rows_cleared() {
        let auth = &row.publish_target.auth_disclosure;
        assert_eq!(auth.state, AuthDisclosureState::ExplicitDisclosed);
        assert!(auth.disclosed_before_mutation);
        assert!(auth.target_scope_disclosed);
    }
}

#[test]
fn summary_counts_match_rows() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.entries_cleared + r.summary.entries_narrowed,
        r.rows.len()
    );
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
fn narrowed_lane_surfaces_its_gaps_in_export() {
    // The narrowed lane must expose its auth/parity/dry-run/rollback gaps rather
    // than dropping them from the view.
    let r = register();
    let projection = r.support_export_projection();
    let narrowed = projection
        .rows
        .iter()
        .find(|row| !row.publishes_stable)
        .expect("a narrowed lane exists");
    assert!(
        !narrowed.active_narrowing_reasons.is_empty(),
        "a narrowed lane must surface its narrowing reasons"
    );
    assert!(
        narrowed.auth_disclosure_state == AuthDisclosureState::AmbientInherited
            || narrowed.parity_state != ParityState::Matched
            || narrowed.rollback_target_ref.is_empty(),
        "a narrowed lane must surface its concrete disclosure gap"
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
        assert_eq!(
            row.version_bump.proposal.target_version,
            proj.target_version
        );
    }
}

#[test]
fn validate_flags_a_cleared_lane_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a cleared lane exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::ProofPacketMissing);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, PublicationReviewViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_an_ambient_auth_lane_without_reason() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a cleared lane exists");
    row.publish_target.auth_disclosure.state = AuthDisclosureState::AmbientInherited;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        PublicationReviewViolation::DisclosureGapWithoutReason {
            reason: NarrowingReason::AmbientCredentialInheritance,
            ..
        }
    )));
}

#[test]
fn validate_flags_a_broken_descriptor_parity_on_a_cleared_lane() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a cleared lane exists");
    row.review_parity.headless_descriptor_digest = "sha256/tampered".to_owned();
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        PublicationReviewViolation::HeldWithoutParity { .. }
            | PublicationReviewViolation::DisclosureGapWithoutReason {
                reason: NarrowingReason::DescriptorParityBroken,
                ..
            }
    )));
}

#[test]
fn validate_flags_a_cleared_lane_without_signoff() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a cleared lane exists");
    row.owner_signoff.signed_off = false;
    row.owner_signoff.signed_at = None;
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, PublicationReviewViolation::HeldWithoutSignoff { .. })));
}
