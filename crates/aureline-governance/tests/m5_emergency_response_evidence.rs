//! Protected tests binding the typed emergency-response evidence register to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the coverage check proves every packet kind is
//! exercised and every narrowing reason is wired; the capture cross-check proves the typed model and
//! the CI gate agree on the promotion verdict, the scan/surface parity, and the cleared/narrowed
//! counts; the no-mask check proves a green emergency-response surface still narrows on a mirror or
//! offline reach gap and that scan and surface agree on every record; the narrowing check proves a
//! response failure on a still-stable subject holds promotion while inherited and waived narrowings
//! stay gated upstream; the negative cases mutate a parsed copy and read the checked-in fixtures to
//! prove that a hidden distribution gap, a green surface over a gapped scan, a narrowed record that
//! stays above the cutline, a break-glass action without audit markers, and a proceed verdict while
//! a rule fires all fail validation.

use std::path::{Path, PathBuf};

use aureline_governance::m5_boundary_and_upstream_durability::{FreshnessSloState, LifecycleLabel};
use aureline_governance::m5_emergency_response_evidence::{
    current_m5_emergency_response_evidence, ControlDimension, EmergencyResponseEvidenceRegister,
    PacketKind, Posture, PublicationDecision, RegisterViolation, ResponseReason, ResponseState,
    M5_EMERGENCY_RESPONSE_EVIDENCE_RECORD_KIND, M5_EMERGENCY_RESPONSE_EVIDENCE_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/captures/m5-emergency-response-evidence_validation_capture.json"
));

fn register() -> EmergencyResponseEvidenceRegister {
    current_m5_emergency_response_evidence().expect("checked-in register parses into the model")
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
        M5_EMERGENCY_RESPONSE_EVIDENCE_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_EMERGENCY_RESPONSE_EVIDENCE_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_packet_kind_and_every_reason_has_a_rule() {
    let r = register();
    for kind in PacketKind::ALL {
        assert!(
            !r.records_of_kind(kind).is_empty(),
            "packet kind {} must have at least one record",
            kind.as_str()
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
    for reason in ResponseReason::ALL {
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
    let states: std::collections::BTreeSet<ResponseState> =
        r.records.iter().map(|x| x.continuity_state).collect();
    assert!(states.contains(&ResponseState::Cleared));
    // Distinct narrowing axes coexist instead of collapsing into one pass/fail flag.
    assert!(states.contains(&ResponseState::NarrowedTemplate));
    assert!(states.contains(&ResponseState::NarrowedDistribution));
    assert!(states.contains(&ResponseState::NarrowedAttribution));
    assert!(states.contains(&ResponseState::NarrowedReversibility));
    assert!(states.contains(&ResponseState::NarrowedAudit));
    assert!(states.contains(&ResponseState::NarrowedLinkage));
    assert!(states.contains(&ResponseState::NarrowedStale));

    let reasons: std::collections::BTreeSet<ResponseReason> = r
        .records
        .iter()
        .flat_map(|x| x.active_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&ResponseReason::MirrorPropagationIncomplete));
    assert!(reasons.contains(&ResponseReason::OfflineImportResponseMissing));
    assert!(reasons.contains(&ResponseReason::ChannelEvidenceStale));
    assert!(reasons.contains(&ResponseReason::AuditMarkersMissing));
    assert!(reasons.contains(&ResponseReason::ActionUnattributable));
    assert!(reasons.contains(&ResponseReason::PacketTemplateUnbound));
}

#[test]
fn green_surface_never_masks_a_mirror_or_offline_reach_gap() {
    let r = register();
    // Every record's scan and surface agree, so a green surface can never sit over a scan that found
    // gaps.
    for rec in &r.records {
        assert!(
            rec.scan_surface_agree(),
            "record {} scan and surface must agree",
            rec.record_id
        );
        assert_eq!(rec.surface_posture, rec.computed_posture());
    }
    // A mirror reach gap still narrows on the distribution axis and reports gaps on its surface.
    let mirror = r
        .records
        .iter()
        .find(|rec| {
            rec.has_active_reason(ResponseReason::MirrorPropagationIncomplete) && !rec.is_waived()
        })
        .expect("a mirror reach gap exists");
    assert_eq!(mirror.continuity_state, ResponseState::NarrowedDistribution);
    assert_eq!(mirror.surface_posture, Posture::GapsFound);
    // An offline reach gap still narrows on the distribution axis.
    let offline = r
        .records
        .iter()
        .find(|rec| rec.has_active_reason(ResponseReason::OfflineImportResponseMissing))
        .expect("an offline reach gap exists");
    assert_eq!(
        offline.continuity_state,
        ResponseState::NarrowedDistribution
    );
    assert_eq!(offline.surface_posture, Posture::GapsFound);
}

#[test]
fn distribution_attribution_and_audit_truth_is_recorded() {
    let r = register();
    // The distribution, attribution, audit, reversibility, linkage, and template axes carry gaps.
    assert!(r.summary.template_gaps > 0, "must record a template gap");
    assert!(
        r.summary.distribution_gaps > 0,
        "must record a distribution gap"
    );
    assert!(
        r.summary.attribution_gaps > 0,
        "must record an attribution gap"
    );
    assert!(
        r.summary.reversibility_gaps > 0,
        "must record a reversibility gap"
    );
    assert!(r.summary.audit_gaps > 0, "must record an audit gap");
    assert!(r.summary.linkage_gaps > 0, "must record a linkage gap");
    // The hosted/mirror/offline reach is tracked at the channel level.
    assert!(r.summary.mirror_reach_gaps > 0);
    assert!(r.summary.offline_reach_gaps > 0);
    // Break-glass actions and reconciliation are tracked.
    assert!(r.summary.break_glass_total > 0);
    assert!(r.summary.reconciliation_required > 0);
    assert!(r.summary.reconciliation_complete > 0);
    for rec in &r.records {
        if rec.unattributable() {
            assert!(rec.has_active_reason(ResponseReason::ActionUnattributable));
        }
        if rec.linkage_missing() {
            assert!(rec.has_active_reason(ResponseReason::EvidenceLinkageMissing));
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
        u(&summary["state_narrowed_template"]),
        computed.state_narrowed_template
    );
    assert_eq!(
        u(&summary["state_narrowed_distribution"]),
        computed.state_narrowed_distribution
    );
    assert_eq!(
        u(&summary["state_narrowed_attribution"]),
        computed.state_narrowed_attribution
    );
    assert_eq!(
        u(&summary["state_narrowed_reversibility"]),
        computed.state_narrowed_reversibility
    );
    assert_eq!(
        u(&summary["state_narrowed_audit"]),
        computed.state_narrowed_audit
    );
    assert_eq!(
        u(&summary["state_narrowed_linkage"]),
        computed.state_narrowed_linkage
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
    assert_eq!(u(&summary["template_gaps"]), computed.template_gaps);
    assert_eq!(u(&summary["distribution_gaps"]), computed.distribution_gaps);
    assert_eq!(u(&summary["attribution_gaps"]), computed.attribution_gaps);
    assert_eq!(
        u(&summary["reversibility_gaps"]),
        computed.reversibility_gaps
    );
    assert_eq!(u(&summary["audit_gaps"]), computed.audit_gaps);
    assert_eq!(u(&summary["linkage_gaps"]), computed.linkage_gaps);
    assert_eq!(u(&summary["mirror_reach_gaps"]), computed.mirror_reach_gaps);
    assert_eq!(
        u(&summary["offline_reach_gaps"]),
        computed.offline_reach_gaps
    );
    assert_eq!(u(&summary["break_glass_total"]), computed.break_glass_total);
    assert_eq!(
        u(&summary["reconciliation_required"]),
        computed.reconciliation_required
    );
    assert_eq!(
        u(&summary["reconciliation_complete"]),
        computed.reconciliation_complete
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
fn response_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, PublicationDecision::Hold);
    let blocking = r.computed_blocking_record_ids();
    assert!(
        !blocking.is_empty(),
        "a response failure on a still-stable subject must hold promotion"
    );
    for id in &blocking {
        let rec = r.record(id).expect("blocking record exists");
        assert!(rec.release_blocking);
        assert!(rec.declares_at_or_above_cutline());
        assert!(!rec.is_waived());
    }
    // The notebook revocation that never reached the mirror holds promotion on the distribution axis.
    let mirror = r
        .record("response-notebook-extension-provider-revocation")
        .expect("notebook revocation record exists");
    assert_eq!(mirror.continuity_state, ResponseState::NarrowedDistribution);
    assert!(mirror.has_active_reason(ResponseReason::MirrorPropagationIncomplete));
    assert!(blocking.contains(&mirror.record_id));
    // The managed-depth disable bundle that never reached offline holds promotion on the same axis.
    let offline = r
        .record("response-managed_depth-emergency-disable-bundle")
        .expect("managed-depth disable record exists");
    assert_eq!(
        offline.continuity_state,
        ResponseState::NarrowedDistribution
    );
    assert!(offline.has_active_reason(ResponseReason::OfflineImportResponseMissing));
    assert!(blocking.contains(&offline.record_id));
    // The break-glass advisory with no audit markers is the headline break-glass guardrail.
    let audit = r
        .record("response-ai_adjacent-security-advisory")
        .expect("audit record exists");
    assert_eq!(audit.continuity_state, ResponseState::NarrowedAudit);
    assert!(audit.has_active_reason(ResponseReason::AuditMarkersMissing));
    assert!(audit.is_break_glass);
    assert!(blocking.contains(&audit.record_id));
    // The side-channel-only disable bundle holds promotion on the linkage axis.
    let linkage = r
        .record("response-data_rich-emergency-disable-bundle")
        .expect("linkage record exists");
    assert_eq!(linkage.continuity_state, ResponseState::NarrowedLinkage);
    assert!(blocking.contains(&linkage.record_id));
    // The mirror gap held by an unexpired waiver is narrowed and visible, but not held.
    let waived = r
        .record("response-review-extension-provider-revocation")
        .expect("waived record exists");
    assert_eq!(waived.continuity_state, ResponseState::NarrowedDistribution);
    assert!(waived.is_waived());
    assert!(!blocking.contains(&waived.record_id));
    // The companion disable bundle already sits below the cutline (Beta): inherited.
    let beta = r
        .record("response-companion-emergency-disable-bundle")
        .expect("beta record exists");
    assert!(beta.continuity_state.is_narrowed());
    assert!(!beta.declares_at_or_above_cutline());
    assert!(!blocking.contains(&beta.record_id));
}

#[test]
fn stale_and_missing_proof_narrow_on_the_stale_axis() {
    let r = register();
    let stale = r
        .record("response-managed_depth-high-severity-postmortem")
        .expect("stale-proof record exists");
    assert_eq!(stale.continuity_state, ResponseState::NarrowedStale);
    assert_eq!(stale.proof_packet.slo_state, FreshnessSloState::Breached);
    let missing = r
        .record("response-data_rich-security-advisory")
        .expect("missing-proof record exists");
    assert_eq!(missing.continuity_state, ResponseState::NarrowedStale);
    assert_eq!(missing.proof_packet.slo_state, FreshnessSloState::Missing);
}

#[test]
fn hidden_distribution_gap_fails() {
    let mut r = register();
    let rec = r
        .records
        .iter_mut()
        .find(|x| x.is_cleared())
        .expect("a cleared record exists");
    if let Some(mirror) = rec.distribution_reach.channels.iter_mut().find(|c| {
        c.channel
            == aureline_governance::m5_emergency_response_evidence::DistributionChannel::Mirror
    }) {
        mirror.claimed = true;
        mirror.state = aureline_governance::m5_emergency_response_evidence::ChannelState::Pending;
    }
    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            RegisterViolation::GapWithoutReason {
                reason: ResponseReason::MirrorPropagationIncomplete,
                ..
            } | RegisterViolation::ControlStateInconsistent { .. }
        )),
        "a hidden distribution gap must fail validation"
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
    let fixtures_dir = repo_root().join("fixtures/governance/m5-emergency-response-evidence");
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
        let candidate: EmergencyResponseEvidenceRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        checked += 1;
    }
    assert!(checked > 0, "at least one fixture must be exercised");
}
