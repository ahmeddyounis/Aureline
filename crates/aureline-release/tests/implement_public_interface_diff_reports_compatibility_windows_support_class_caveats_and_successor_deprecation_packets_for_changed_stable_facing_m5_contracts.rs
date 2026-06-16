//! Protected tests binding the typed M5 public-interface diff-report register to
//! the checked-in artifact, the frozen CI validation capture, and the negative
//! fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves
//! the typed model and the CI gate agree on the promotion verdict, the
//! breaking-change and deprecation-packet counts, the closed-window count, and the
//! packet-freshness counts; the negative cases mutate a parsed copy and the
//! checked-in fixtures to prove that a contract that fails to narrow, a held report
//! with an active gap, a breaking change held without a deprecation packet, a
//! held report whose reader/writer review is missing, and a promotion verdict that
//! disagrees with the firing rules all fail validation.

use std::path::{Path, PathBuf};

use aureline_release::implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts::{
    current_m5_public_interface_diff_reports, ChangeClass, CompatibilityPosture,
    ContractDiffReportRegister, ContractDiffReportViolation, ContractKind, DeprecationStatus,
    ReportState, ReviewPosture, SupportClass, WindowSupportState,
    M5_PUBLIC_INTERFACE_DIFF_REPORTS_RECORD_KIND, M5_PUBLIC_INTERFACE_DIFF_REPORTS_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts_validation_capture.json"
));

fn register() -> ContractDiffReportRegister {
    current_m5_public_interface_diff_reports().expect("checked-in register parses into the model")
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
        M5_PUBLIC_INTERFACE_DIFF_REPORTS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_PUBLIC_INTERFACE_DIFF_REPORTS_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_contract_kind_and_change_class() {
    let r = register();
    for kind in ContractKind::ALL {
        assert!(
            !r.reports_for_kind(kind).is_empty(),
            "contract kind {} must have at least one report",
            kind.as_str()
        );
    }
    for class in ChangeClass::ALL {
        assert!(
            !r.reports_for_change_class(class).is_empty(),
            "change class {} must have at least one report",
            class.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_contract() {
    let r = register();
    assert!(!r.release_blocking_contract_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_reports()
        .into_iter()
        .map(|row| row.contract_ref.as_str())
        .collect();
    for declared in &r.release_blocking_contract_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking report"
        );
    }
}

#[test]
fn exercises_the_diff_and_deprecation_vocabulary() {
    let r = register();

    // Every narrowing scenario the register must demonstrate.
    let states: std::collections::BTreeSet<ReportState> =
        r.reports.iter().map(|row| row.report_state).collect();
    for state in [
        ReportState::BreakingUnpacketed,
        ReportState::DeprecationIncomplete,
        ReportState::CompatReviewPending,
        ReportState::RemovalOverdue,
        ReportState::SupportWindowEnded,
        ReportState::EvidenceStale,
        ReportState::Incomplete,
    ] {
        assert!(
            states.contains(&state),
            "the register must exercise the {} state",
            state.as_str()
        );
    }

    // A breaking change governed by a complete deprecation packet still holds.
    assert!(
        r.reports.iter().any(|row| row.holds_label()
            && row.change_class == ChangeClass::Breaking
            && row
                .deprecation_packet
                .as_ref()
                .is_some_and(|p| p.is_complete())),
        "a managed breaking change with a complete deprecation packet must hold"
    );

    // A breaking change with no packet must narrow.
    assert!(
        r.reports
            .iter()
            .any(|row| row.change_class == ChangeClass::Breaking
                && row.deprecation_packet.is_none()
                && !row.publishes_stable()),
        "an unpacketed breaking change must narrow"
    );

    // The compatibility/support window vocabulary is exercised.
    let support_states: std::collections::BTreeSet<WindowSupportState> = r
        .reports
        .iter()
        .map(|row| row.compatibility_window.support_state)
        .collect();
    assert!(support_states.contains(&WindowSupportState::WithinWindow));
    assert!(support_states.contains(&WindowSupportState::SupportEnded));

    // Breaking and unreviewed review postures both appear.
    assert!(
        r.reports
            .iter()
            .any(|row| row.interface_diff.found_breaking()),
        "the register must show a reader/writer review that found a breaking change"
    );
    assert!(
        r.reports.iter().any(
            |row| row.interface_diff.writer_posture == ReviewPosture::Unreviewed
                || row.interface_diff.reader_posture == ReviewPosture::Unreviewed
        ),
        "the register must show an unreviewed reader/writer side"
    );

    // The deprecation-status and compatibility-posture vocabularies are exercised.
    let statuses: std::collections::BTreeSet<DeprecationStatus> = r
        .reports
        .iter()
        .filter_map(|row| row.deprecation_packet.as_ref().map(|p| p.status))
        .collect();
    assert!(statuses.contains(&DeprecationStatus::Deprecated));
    assert!(statuses.contains(&DeprecationStatus::RemovalScheduled));
    assert!(r
        .reports
        .iter()
        .any(|row| row.compatibility_window.posture == CompatibilityPosture::Breaking));
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
    assert_eq!(
        summary["total_reports"].as_u64().unwrap() as usize,
        r.reports.len(),
        "capture report count must match the model"
    );
    assert_eq!(
        summary["reports_publishing_stable"].as_u64().unwrap() as usize,
        r.reports_publishing_stable().len(),
        "capture publishing-stable count must match the model"
    );
    assert_eq!(
        summary["reports_narrowed"].as_u64().unwrap() as usize,
        r.reports_narrowed().len(),
        "capture narrowed count must match the model"
    );
    assert_eq!(
        summary["breaking_changes"].as_u64().unwrap() as usize,
        computed.breaking_changes,
        "capture breaking-change count must match the model"
    );
    assert_eq!(
        summary["complete_deprecation_packets"].as_u64().unwrap() as usize,
        computed.complete_deprecation_packets,
        "capture complete-packet count must match the model"
    );
    assert_eq!(
        summary["reports_support_window_ended"].as_u64().unwrap() as usize,
        computed.reports_support_window_ended,
        "capture window-ended count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        computed.packets_breached,
        "capture breached-packet count must match the model"
    );
    assert_eq!(
        summary["rules_firing"].as_u64().unwrap() as usize,
        computed.rules_firing,
        "capture firing-rule count must match the model"
    );

    let captured_decision = capture["promotion"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        r.promotion.decision.as_str(),
        "capture promotion decision must match the model"
    );
    assert_eq!(r.promotion.decision, r.computed_promotion_decision());

    let captured_rules: Vec<&str> = capture["promotion"]["blocking_rule_ids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(
        captured_rules,
        r.computed_blocking_rule_ids(),
        "capture blocking rule ids must match the model"
    );

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
fn register_narrows_a_release_blocking_contract() {
    let r = register();
    let narrowed = r
        .reports
        .iter()
        .find(|row| row.release_blocking && row.claim_holds_stable() && !row.publishes_stable());
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking contract under a still-stable claim"
    );
}

#[test]
fn register_shows_a_contract_on_waiver() {
    let r = register();
    let on_waiver = r
        .reports
        .iter()
        .find(|row| row.report_state == ReportState::OnWaiver)
        .expect("the register must show a contract on waiver");
    assert!(on_waiver.waiver.is_some());
    assert!(on_waiver.publishes_stable());
}

#[test]
fn register_shows_a_compatible_change_that_narrows_on_evidence() {
    // A backward-compatible additive change keeps its compatibility, yet its
    // support claim still narrows on stale or missing evidence.
    let r = register();
    let narrowed_but_compatible = r.reports.iter().find(|row| {
        row.change_class == ChangeClass::Additive
            && row.compatibility_window.support_state == WindowSupportState::WithinWindow
            && !row.publishes_stable()
    });
    assert!(
        narrowed_but_compatible.is_some(),
        "the register must show a compatible change whose claim narrows on evidence"
    );
}

#[test]
fn narrowing_contract_that_does_not_narrow_fails() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| !row.holds_label() && row.claim_label == StableClaimLevel::Stable)
        .expect("register has a narrowed report under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    r.summary = r.computed_summary();
    r.promotion.decision = r.computed_promotion_decision();
    r.promotion.blocking_rule_ids = r.computed_blocking_rule_ids();
    r.promotion.blocking_claim_ids = r.computed_blocking_entry_ids();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            ContractDiffReportViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a contract that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_contract_with_active_gap_fails() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed report");
    row.active_narrowing_reasons
        .push(aureline_release::implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts::NarrowingReason::EvidenceStale);
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ContractDiffReportViolation::HeldWithActiveGap { .. })),
        "a backed contract may not carry an active narrowing reason"
    );
}

#[test]
fn breaking_change_held_without_packet_fails() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| row.change_class == ChangeClass::Breaking && row.deprecation_packet.is_none())
        .expect("register has an unpacketed breaking change");
    row.report_state = ReportState::Published;
    row.published_label = StableClaimLevel::Stable;
    row.active_narrowing_reasons.clear();
    row.support_caveat.support_class = SupportClass::SupportedWithCaveats;
    row.support_caveat.caveats = vec!["forced".to_owned()];
    r.summary = r.computed_summary();

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            ContractDiffReportViolation::BreakingHeldWithoutPacket { .. }
        )),
        "a breaking change without a deprecation packet may not hold a stable claim"
    );
}

#[test]
fn backed_contract_on_a_breached_packet_fails() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("register has a backed report");
    row.proof_packet.slo_state = FreshnessSloState::Breached;
    r.summary = r.computed_summary();

    assert!(
        r.validate()
            .iter()
            .any(|v| matches!(v, ContractDiffReportViolation::HeldOnStalePacket { .. })),
        "a backed contract may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;

    assert!(
        r.validate().iter().any(|v| matches!(
            v,
            ContractDiffReportViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/compat/m5-public-interface-diff-reports");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: ContractDiffReportRegister =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
        model_checked += 1;
    }
    assert!(
        model_checked > 0,
        "at least one fixture must exercise a typed-model structural invariant"
    );
}
