//! Protected tests binding the typed release-authority continuity register to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the coverage check proves every authority lane is
//! exercised and every narrowing reason is wired; the capture cross-check proves the typed model
//! and the CI gate agree on the promotion verdict, the scan/surface parity, and the
//! cleared/narrowed counts; the no-mask check proves a green authority surface still narrows on a
//! single-owner or owner-vacant lane and that scan and surface agree on every record; the narrowing
//! check proves a continuity failure on a still-stable subject holds promotion while inherited and
//! waived narrowings stay gated upstream; the negative cases mutate a parsed copy and read the
//! checked-in fixtures to prove that a hidden backup gap, a green surface over a gapped scan, a
//! narrowed record that stays above the cutline, and a proceed verdict while a rule fires all fail
//! validation.

use std::path::{Path, PathBuf};

use aureline_governance::m5_boundary_and_upstream_durability::{FreshnessSloState, LifecycleLabel};
use aureline_governance::m5_release_authority_continuity::{
    current_m5_release_authority_continuity, AuthorityLane, BackupState, ContinuityReason,
    ContinuityState, ControlDimension, Posture, PublicationDecision, RegisterViolation,
    ReleaseAuthorityContinuityRegister, M5_RELEASE_AUTHORITY_CONTINUITY_RECORD_KIND,
    M5_RELEASE_AUTHORITY_CONTINUITY_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/captures/m5-release-authority-continuity_validation_capture.json"
));

fn register() -> ReleaseAuthorityContinuityRegister {
    current_m5_release_authority_continuity().expect("checked-in register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_RELEASE_AUTHORITY_CONTINUITY_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_RELEASE_AUTHORITY_CONTINUITY_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_authority_lane_and_every_reason_has_a_rule() {
    let r = register();
    for lane in AuthorityLane::ALL {
        assert!(
            !r.records_of_lane(lane).is_empty(),
            "authority lane {} must have at least one record",
            lane.as_str()
        );
    }
    for rec in &r.records {
        for dimension in ControlDimension::ALL {
            assert_eq!(
                rec.controls
                    .iter()
                    .filter(|c| c.dimension == dimension)
                    .count(),
                1,
                "record {} must declare control {} exactly once",
                rec.record_id,
                dimension.as_str()
            );
        }
    }
    for reason in ContinuityReason::ALL {
        assert!(
            r.rules.iter().any(|rule| rule.trigger_reason == reason),
            "reason {} must be watched by a rule",
            reason.as_str()
        );
    }
}

#[test]
fn keeps_per_axis_state_not_one_global_flag() {
    let r = register();
    let states: std::collections::BTreeSet<ContinuityState> =
        r.records.iter().map(|x| x.continuity_state).collect();
    assert!(states.contains(&ContinuityState::Cleared));
    // Distinct narrowing axes coexist instead of collapsing into one pass/fail flag.
    assert!(states.contains(&ContinuityState::NarrowedOwner));
    assert!(states.contains(&ContinuityState::NarrowedBackup));
    assert!(states.contains(&ContinuityState::NarrowedQuorum));
    assert!(states.contains(&ContinuityState::NarrowedRunbook));
    assert!(states.contains(&ContinuityState::NarrowedAuthority));
    assert!(states.contains(&ContinuityState::NarrowedStale));

    let reasons: std::collections::BTreeSet<ContinuityReason> = r
        .records
        .iter()
        .flat_map(|x| x.active_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&ContinuityReason::BackupOwnerMissing));
    assert!(reasons.contains(&ContinuityReason::RosterQuorumBelowThreshold));
    assert!(reasons.contains(&ContinuityReason::SplitAuthorityUnmet));
    assert!(reasons.contains(&ContinuityReason::ShiproomEscalationMissing));
    assert!(reasons.contains(&ContinuityReason::PrimaryOwnerVacant));
    assert!(reasons.contains(&ContinuityReason::RunbookMissing));
}

#[test]
fn green_surface_never_masks_a_single_owner_or_vacant_lane() {
    let r = register();
    // Every record's scan and surface agree, so a green surface can never sit over a scan that
    // found gaps.
    for rec in &r.records {
        assert!(
            rec.scan_surface_agree(),
            "record {} scan and surface must agree",
            rec.record_id
        );
        assert_eq!(rec.surface_posture, rec.computed_posture());
    }
    // A single-owner lane still narrows on the backup axis and reports gaps on its surface.
    let single = r
        .records
        .iter()
        .find(|rec| rec.is_single_owner() && !rec.is_waived())
        .expect("a single-owner lane exists");
    assert_eq!(single.continuity_state, ContinuityState::NarrowedBackup);
    assert_eq!(single.surface_posture, Posture::GapsFound);
    // A primary-owner-vacant lane still narrows on the owner axis.
    let vacant = r
        .records
        .iter()
        .find(|rec| rec.owner_vacant())
        .expect("an owner-vacant lane exists");
    assert_eq!(vacant.continuity_state, ContinuityState::NarrowedOwner);
    assert_eq!(vacant.surface_posture, Posture::GapsFound);
}

#[test]
fn owner_backup_and_authority_truth_is_recorded() {
    let r = register();
    // The owner, backup, quorum, runbook, and authority axes actually carry gaps.
    assert!(r.summary.owner_gaps > 0, "must record an owner gap");
    assert!(r.summary.backup_gaps > 0, "must record a backup gap");
    assert!(r.summary.quorum_gaps > 0, "must record a quorum gap");
    assert!(r.summary.runbook_gaps > 0, "must record a runbook gap");
    assert!(r.summary.authority_gaps > 0, "must record an authority gap");
    // Critical lanes, escalations, and split-authority enforcement are tracked.
    assert!(r.summary.critical_total > 0);
    assert!(r.summary.single_owner_total > 0);
    assert!(r.summary.escalations_required > 0);
    assert!(r.summary.escalations_raised > 0);
    assert!(r.summary.split_authority_enforced > 0);
    for rec in &r.records {
        if rec.is_single_owner() {
            assert!(rec.has_active_reason(ContinuityReason::BackupOwnerMissing));
        }
        if rec.escalation_missing() {
            assert!(rec.has_active_reason(ContinuityReason::ShiproomEscalationMissing));
        }
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let r = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(r.as_of.as_str()));

    let summary = &capture["summary"];
    let computed = r.computed_summary();
    let u = |v: &serde_json::Value| v.as_u64().unwrap() as usize;
    assert_eq!(u(&summary["total_records"]), computed.total_records);
    assert_eq!(u(&summary["records_cleared"]), computed.records_cleared);
    assert_eq!(u(&summary["records_narrowed"]), computed.records_narrowed);
    assert_eq!(u(&summary["state_cleared"]), computed.state_cleared);
    assert_eq!(
        u(&summary["state_narrowed_owner"]),
        computed.state_narrowed_owner
    );
    assert_eq!(
        u(&summary["state_narrowed_backup"]),
        computed.state_narrowed_backup
    );
    assert_eq!(
        u(&summary["state_narrowed_quorum"]),
        computed.state_narrowed_quorum
    );
    assert_eq!(
        u(&summary["state_narrowed_runbook"]),
        computed.state_narrowed_runbook
    );
    assert_eq!(
        u(&summary["state_narrowed_authority"]),
        computed.state_narrowed_authority
    );
    assert_eq!(
        u(&summary["state_narrowed_stale"]),
        computed.state_narrowed_stale
    );
    assert_eq!(
        u(&summary["release_blocking_narrowed"]),
        computed.release_blocking_narrowed
    );
    assert_eq!(
        u(&summary["records_on_active_waiver"]),
        computed.records_on_active_waiver
    );
    assert_eq!(u(&summary["owner_gaps"]), computed.owner_gaps);
    assert_eq!(u(&summary["backup_gaps"]), computed.backup_gaps);
    assert_eq!(u(&summary["quorum_gaps"]), computed.quorum_gaps);
    assert_eq!(u(&summary["runbook_gaps"]), computed.runbook_gaps);
    assert_eq!(u(&summary["authority_gaps"]), computed.authority_gaps);
    assert_eq!(u(&summary["critical_total"]), computed.critical_total);
    assert_eq!(
        u(&summary["single_owner_total"]),
        computed.single_owner_total
    );
    assert_eq!(
        u(&summary["escalations_required"]),
        computed.escalations_required
    );
    assert_eq!(
        u(&summary["escalations_raised"]),
        computed.escalations_raised
    );
    assert_eq!(
        u(&summary["split_authority_enforced"]),
        computed.split_authority_enforced
    );
    assert_eq!(
        u(&summary["total_active_reasons"]),
        computed.total_active_reasons
    );
    assert_eq!(u(&summary["rules_firing"]), computed.rules_firing);

    let parity = &capture["scan_surface_parity"];
    let computed_parity = r.computed_scan_surface_parity();
    assert_eq!(
        u(&parity["subjects_in_agreement"]),
        computed_parity.subjects_in_agreement
    );
    assert_eq!(
        u(&parity["subjects_in_disagreement"]),
        computed_parity.subjects_in_disagreement
    );
    assert_eq!(
        u(&parity["subjects_with_gaps"]),
        computed_parity.subjects_with_gaps
    );
    assert_eq!(
        parity["all_subjects_agree"].as_bool(),
        Some(computed_parity.all_subjects_agree)
    );

    assert_eq!(
        capture["publication"]["decision"].as_str().unwrap(),
        r.publication.decision.as_str()
    );
    assert_eq!(r.publication.decision, r.computed_decision());

    let captured_rules: Vec<&str> = capture["publication"]["blocking_rule_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_rules, r.computed_blocking_rule_ids());

    let captured_records: Vec<&str> = capture["publication"]["blocking_record_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(captured_records, r.computed_blocking_record_ids());

    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
    let fixtures = capture["fixture_cases"].as_array().unwrap();
    assert!(!fixtures.is_empty(), "capture must record fixture cases");
    for case in fixtures {
        assert_eq!(
            case["status"].as_str(),
            Some("passed"),
            "frozen capture fixture case {} must have passed",
            case["case_id"]
        );
    }
}

#[test]
fn continuity_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, PublicationDecision::Hold);
    let blocking = r.computed_blocking_record_ids();
    assert!(
        !blocking.is_empty(),
        "a continuity failure on a still-stable subject must hold promotion"
    );
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
    }
    // The single-owner notebook promotion lane holds promotion on the backup axis.
    let notebook = r
        .record("authority-notebook-promotion-approval")
        .expect("notebook record exists");
    assert_eq!(notebook.continuity_state, ContinuityState::NarrowedBackup);
    assert!(blocking.contains(&notebook.record_id));
    // The under-quorum managed-depth signing lane holds promotion on the quorum axis.
    let quorum = r
        .record("authority-managed_depth-release-signing")
        .expect("quorum record exists");
    assert_eq!(quorum.continuity_state, ContinuityState::NarrowedQuorum);
    assert!(blocking.contains(&quorum.record_id));
    // The unmet-split, unescalated security lane is the headline guardrail.
    let security = r
        .record("authority-ai_adjacent-security-response")
        .expect("authority record exists");
    assert_eq!(
        security.continuity_state,
        ContinuityState::NarrowedAuthority
    );
    assert!(security.has_active_reason(ContinuityReason::SplitAuthorityUnmet));
    assert!(security.has_active_reason(ContinuityReason::ShiproomEscalationMissing));
    assert!(blocking.contains(&security.record_id));
    // The runbook-less companion moderation lane holds promotion on the runbook axis.
    let runbook = r
        .record("authority-companion-registry-moderation")
        .expect("runbook record exists");
    assert_eq!(runbook.continuity_state, ContinuityState::NarrowedRunbook);
    assert!(blocking.contains(&runbook.record_id));
    // The single-owner review promotion lane is narrowed and visible, but held by a waiver.
    let waived = r
        .record("authority-review-promotion-approval")
        .expect("waived record exists");
    assert_eq!(waived.continuity_state, ContinuityState::NarrowedBackup);
    assert!(waived.is_waived());
    assert_eq!(
        waived.backup_coverage.backup_state,
        BackupState::SingleOwner
    );
    assert!(!blocking.contains(&waived.record_id));
    // The data-rich moderation lane already sits below the cutline (Beta): inherited.
    let beta = r
        .record("authority-data_rich-registry-moderation")
        .expect("beta record exists");
    assert!(beta.continuity_state.is_narrowed());
    assert!(!beta.declares_at_or_above_cutline());
    assert!(!blocking.contains(&beta.record_id));
}

#[test]
fn stale_and_missing_proof_narrow_on_the_stale_axis() {
    let r = register();
    let stale = r
        .record("authority-managed_depth-security-response")
        .expect("stale-proof record exists");
    assert_eq!(stale.continuity_state, ContinuityState::NarrowedStale);
    assert_eq!(stale.proof_packet.slo_state, FreshnessSloState::Breached);
    let missing = r
        .record("authority-managed_depth-registry-moderation")
        .expect("missing-proof record exists");
    assert_eq!(missing.continuity_state, ContinuityState::NarrowedStale);
    assert_eq!(missing.proof_packet.slo_state, FreshnessSloState::Missing);
}

#[test]
fn hidden_backup_gap_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared() && x.is_critical())
        .expect("a cleared critical record exists");
    rec.backup_coverage.backup_state = BackupState::SingleOwner;
    rec.backup_coverage.backup_owner_count = 0;
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            RegisterViolation::GapWithoutReason {
                reason: ContinuityReason::BackupOwnerMissing,
                ..
            }
        )),
        "a hidden backup gap must fail validation"
    );
}

#[test]
fn green_surface_over_a_gapped_scan_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.continuity_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.surface_posture = Posture::Clear;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::ScanSurfaceDisagreement { .. }
            | RegisterViolation::PostureMismatch { .. }
    )));
}

#[test]
fn narrowed_record_above_the_cutline_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.continuity_state.is_narrowed())
        .expect("a narrowed record exists");
    rec.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.publication.decision = PublicationDecision::Proceed;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, RegisterViolation::PublicationDecisionInconsistent)));
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/governance/m5-release-authority-continuity");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: ReleaseAuthorityContinuityRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        checked += 1;
    }
    assert!(checked > 0, "at least one fixture must be exercised");
}
