//! Protected tests binding the typed final go/no-go rehearsal to the checked-in artifact,
//! the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in rehearsal; the capture cross-check proves the typed
//! model and the Python gate agree on the go/no-go verdict, the stage-kind coverage counts,
//! the rollback-checkpoint counts, and the packet-freshness counts; the negative cases
//! mutate a parsed copy and the checked-in fixtures to prove that a stage which fails to
//! narrow, a Go stage riding a breached packet, a stage carried wider than its public claim's
//! ceiling, and a go/no-go verdict that disagrees with the firing rules all fail validation.
//!
//! Cross-artifact fixtures whose `expected_check_id` is a `ceiling.*` check are skipped in
//! the typed-model fixture loop: those flaws (a claim label that disagrees with the stable
//! claim manifest) are only observable by reading the neighbouring manifest, which the CI
//! gate does and the metadata-only typed model deliberately does not.

use std::path::{Path, PathBuf};

use aureline_release::go_no_go_rehearsal::{
    current_go_no_go_rehearsal, GoNoGoRehearsal, GoNoGoRehearsalViolation, RehearsalState,
    StageKind, GO_NO_GO_REHEARSAL_RECORD_KIND, GO_NO_GO_REHEARSAL_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/go_no_go_rehearsal_validation_capture.json"
));

fn rehearsal() -> GoNoGoRehearsal {
    current_go_no_go_rehearsal()
        .expect("checked-in go/no-go rehearsal parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_rehearsal_parses_and_validates() {
    let rehearsal = rehearsal();
    assert_eq!(rehearsal.schema_version, GO_NO_GO_REHEARSAL_SCHEMA_VERSION);
    assert_eq!(rehearsal.record_kind, GO_NO_GO_REHEARSAL_RECORD_KIND);
    let violations = rehearsal.validate();
    assert!(
        violations.is_empty(),
        "checked-in rehearsal must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_stage_kind() {
    let rehearsal = rehearsal();
    for kind in StageKind::ALL {
        assert!(
            !rehearsal.rows_for_kind(kind).is_empty(),
            "stage kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_stage() {
    let rehearsal = rehearsal();
    assert!(!rehearsal.release_blocking_stage_refs.is_empty());
    let covered: Vec<&str> = rehearsal
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.entry_id.as_str())
        .collect();
    for declared in &rehearsal.release_blocking_stage_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let rehearsal = rehearsal();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(rehearsal.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_rows"].as_u64().unwrap() as usize,
        rehearsal.rows.len(),
        "capture row count must match the model"
    );
    assert_eq!(
        summary["rows_go"].as_u64().unwrap() as usize,
        rehearsal.rows_go().len(),
        "capture go count must match the model"
    );
    assert_eq!(
        summary["total_checkpoints"].as_u64().unwrap() as usize,
        rehearsal.computed_summary().total_checkpoints,
        "capture checkpoint total must match the model"
    );
    assert_eq!(
        summary["checkpoints_unverified"].as_u64().unwrap() as usize,
        rehearsal.computed_summary().checkpoints_unverified,
        "capture unverified-checkpoint count must match the model"
    );
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        rehearsal.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        rehearsal.publication.decision.as_str(),
        "capture go/no-go decision must match the model"
    );
    assert_eq!(
        rehearsal.publication.decision,
        rehearsal.computed_publication_decision()
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
fn rehearsal_narrows_a_row_under_a_still_stable_claim() {
    let rehearsal = rehearsal();
    // A release-blocking stage whose public claim is still published Stable but is itself
    // unrehearsed — the launch-rehearsal truth beneath an optimistic claim.
    let narrowed = rehearsal.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.holds_stable()
            && row.rehearsal_state != RehearsalState::NoGoClaimNarrowed
    });
    assert!(
        narrowed.is_some(),
        "the rehearsal must narrow at least one release-blocking row under a still-stable claim"
    );
}

#[test]
fn rehearsal_narrows_a_row_for_an_unverified_checkpoint() {
    let rehearsal = rehearsal();
    let narrowed = rehearsal
        .rows
        .iter()
        .find(|row| row.rehearsal_state == RehearsalState::NoGoUnrehearsed)
        .expect("the rehearsal must show a checkpoint-narrowed row");
    assert!(narrowed.unverified_checkpoint_count() > 0);
    assert!(!narrowed.holds_stable());
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut rehearsal = rehearsal();
    let row = rehearsal
        .rows
        .iter_mut()
        .find(|row| {
            row.rehearsal_state == RehearsalState::NoGoStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("rehearsal has a no-go-stale row under a stable ceiling");
    row.effective_label = StableClaimLevel::Stable;
    rehearsal.summary = rehearsal.computed_summary();
    rehearsal.publication.decision = rehearsal.computed_publication_decision();
    rehearsal.publication.blocking_rule_ids = rehearsal.computed_blocking_rule_ids();
    rehearsal.publication.blocking_entry_ids = rehearsal.computed_blocking_entry_ids();

    assert!(
        rehearsal.validate().iter().any(|v| matches!(
            v,
            GoNoGoRehearsalViolation::EffectiveLabelNotNarrowed { .. }
        )),
        "a row that is not a Go must narrow below the cutline"
    );
}

#[test]
fn go_row_with_unverified_checkpoint_fails() {
    let mut rehearsal = rehearsal();
    let row = rehearsal
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("rehearsal has a Go row");
    row.rollback_checkpoints[0].verified = false;
    rehearsal.summary = rehearsal.computed_summary();

    assert!(
        rehearsal.validate().iter().any(|v| matches!(
            v,
            GoNoGoRehearsalViolation::HeldWithUnverifiedCheckpoint { .. }
        )),
        "a Go row may not carry an unverified required rollback checkpoint"
    );
}

#[test]
fn go_row_on_a_breached_packet_fails() {
    let mut rehearsal = rehearsal();
    let row = rehearsal
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("rehearsal has a Go row");
    row.rehearsal_packet.slo_state = FreshnessSloState::Breached;
    rehearsal.summary = rehearsal.computed_summary();

    assert!(
        rehearsal
            .validate()
            .iter()
            .any(|v| matches!(v, GoNoGoRehearsalViolation::HeldOnStalePacket { .. })),
        "a Go row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn go_no_go_proceed_while_a_rule_fires_fails() {
    let mut rehearsal = rehearsal();
    rehearsal.publication.decision = PromotionDecision::Proceed;

    assert!(
        rehearsal.validate().iter().any(|v| matches!(
            v,
            GoNoGoRehearsalViolation::PublicationDecisionInconsistent { .. }
        )),
        "the go/no-go must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/go_no_go_rehearsal");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        // Cross-artifact (ceiling) flaws are only observable against the neighbouring claim
        // manifest, which the CI gate reads and the typed model does not.
        let expected = case["expected_check_id"].as_str().unwrap_or_default();
        if expected.starts_with("ceiling.") {
            continue;
        }
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: GoNoGoRehearsal =
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
