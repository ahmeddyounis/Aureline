//! Protected tests binding the typed accessibility surface signoff register to the
//! checked-in artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves the typed
//! model and the Python gate agree on the promotion verdict, the surface-kind coverage
//! counts, the packet-freshness counts, and the qualified/narrowed counts; the negative
//! cases mutate a parsed copy and the checked-in fixtures to prove that a signoff which
//! fails to narrow, a backed row with a blocked dimension, a row carried wider than its
//! public claim's ceiling, and a promotion verdict that disagrees with the firing rules all
//! fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery::{
    current_accessibility_surface_signoffs, DimensionKind, DimensionState, GapReason, SignoffState,
    SurfaceKind, AccessibilitySurfaceSignoffs, AccessibilitySurfaceSignoffsViolation,
    ACCESSIBILITY_SURFACE_SIGNOFFS_RECORD_KIND, ACCESSIBILITY_SURFACE_SIGNOFFS_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery_validation_capture.json"
));

fn register() -> AccessibilitySurfaceSignoffs {
    current_accessibility_surface_signoffs()
        .expect("checked-in accessibility register parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let reg = register();
    assert_eq!(
        reg.schema_version,
        ACCESSIBILITY_SURFACE_SIGNOFFS_SCHEMA_VERSION
    );
    assert_eq!(reg.record_kind, ACCESSIBILITY_SURFACE_SIGNOFFS_RECORD_KIND);
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_surface_kind() {
    let reg = register();
    for kind in SurfaceKind::ALL {
        assert!(
            !reg.rows_for_kind(kind).is_empty(),
            "surface kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_surface() {
    let reg = register();
    assert!(!reg.release_blocking_surface_refs.is_empty());
    let covered: Vec<&str> = reg
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.surface_ref.as_str())
        .collect();
    for declared in &reg.release_blocking_surface_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let reg = register();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(reg.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_entries"].as_u64().unwrap() as usize,
        reg.rows.len(),
        "capture entry count must match the model"
    );
    assert_eq!(
        summary["entries_qualified"].as_u64().unwrap() as usize,
        reg.rows
            .iter()
            .filter(|r| r.signoff_state == SignoffState::Qualified)
            .count(),
        "capture qualified count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed"].as_u64().unwrap() as usize,
        reg.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    for (key, kind) in [
        ("shell_entries", SurfaceKind::Shell),
        ("tree_entries", SurfaceKind::Tree),
        ("palette_entries", SurfaceKind::Palette),
        ("diff_entries", SurfaceKind::Diff),
        ("terminal_entries", SurfaceKind::Terminal),
        ("debugger_entries", SurfaceKind::Debugger),
        ("settings_entries", SurfaceKind::Settings),
        ("auth_entries", SurfaceKind::Auth),
        ("recovery_entries", SurfaceKind::Recovery),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            reg.rows_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        reg.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["promotion"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        reg.promotion.decision.as_str(),
        "capture promotion decision must match the model"
    );
    assert_eq!(reg.promotion.decision, reg.computed_promotion_decision());

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
fn register_narrows_a_row_under_a_still_stable_claim() {
    let reg = register();
    let narrowed = reg.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.publishes_stable()
            && row.signoff_state != SignoffState::Qualified
    });
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking row under a still-stable claim"
    );
}

#[test]
fn register_shows_a_blocked_or_pending_dimension() {
    let reg = register();
    let blocked = reg
        .rows
        .iter()
        .find(|row| row.has_blocked_or_pending_dimension());
    assert!(
        blocked.is_some(),
        "the register must show at least one row with a blocked or pending dimension"
    );
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| {
            row.signoff_state == SignoffState::EvidenceStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("register has a stale row under a stable ceiling");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    reg.promotion.decision = reg.computed_promotion_decision();
    reg.promotion.blocking_rule_ids = reg.computed_blocking_rule_ids();
    reg.promotion.blocking_entry_ids = reg.computed_blocking_entry_ids();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            AccessibilitySurfaceSignoffsViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_row_with_blocked_dimension_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.signoff_state == SignoffState::Qualified)
        .expect("register has a qualified row");
    for check in &mut row.dimension_checks {
        if check.dimension == DimensionKind::ScreenReader {
            check.dimension_state = DimensionState::Blocked;
            check.evidence_ref = None;
            break;
        }
    }
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            AccessibilitySurfaceSignoffsViolation::HeldWithBlockedDimension { .. }
        )),
        "a backed row may not carry a blocked or pending dimension"
    );
}

#[test]
fn backed_row_on_a_breached_packet_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("register has a backed row");
    row.proof_packet.slo_state =
        aureline_release::stable_claim_manifest::FreshnessSloState::Breached;
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            AccessibilitySurfaceSignoffsViolation::HeldOnStalePacket { .. }
        )),
        "a backed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn promotion_proceed_while_a_rule_fires_fails() {
    let mut reg = register();
    reg.promotion.decision = PromotionDecision::Proceed;

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            AccessibilitySurfaceSignoffsViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/m4/stabilize-accessibility-signoff-across-shell-tree-palette-diff-terminal-debugger-settings-auth-and-recovery");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let expected = case["expected_check_id"].as_str().unwrap_or_default();
        if expected.starts_with("ceiling.") {
            continue;
        }
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: AccessibilitySurfaceSignoffs =
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
