//! Protected tests binding the typed desktop-platform conformance register to the
//! checked-in artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in register; the capture cross-check proves the typed
//! model and the Python gate agree on the promotion verdict, the domain-kind coverage
//! counts, the packet-freshness counts, and the qualified/narrowed counts; the negative
//! cases mutate a parsed copy and the checked-in fixtures to prove that a conformance which
//! fails to narrow, a backed row with a blocked check, a row carried wider than its
//! public claim's ceiling, and a promotion verdict that disagrees with the firing rules all
//! fail validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance::{
    current_desktop_platform_conformance, CheckKind, CheckState, ConformanceState,
    ConformanceDomain, DesktopPlatformConformance, DesktopPlatformConformanceViolation,
    DESKTOP_PLATFORM_CONFORMANCE_RECORD_KIND, DESKTOP_PLATFORM_CONFORMANCE_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance_validation_capture.json"
));

fn register() -> DesktopPlatformConformance {
    current_desktop_platform_conformance()
        .expect("checked-in desktop-platform conformance register parses into the model")
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
        DESKTOP_PLATFORM_CONFORMANCE_SCHEMA_VERSION
    );
    assert_eq!(reg.record_kind, DESKTOP_PLATFORM_CONFORMANCE_RECORD_KIND);
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_domain_kind() {
    let reg = register();
    for kind in ConformanceDomain::ALL {
        assert!(
            !reg.rows_for_kind(kind).is_empty(),
            "domain kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_domain() {
    let reg = register();
    assert!(!reg.release_blocking_domain_refs.is_empty());
    let covered: Vec<&str> = reg
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.domain_ref.as_str())
        .collect();
    for declared in &reg.release_blocking_domain_refs {
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
            .filter(|r| r.signoff_state == ConformanceState::Qualified)
            .count(),
        "capture qualified count must match the model"
    );
    assert_eq!(
        summary["entries_narrowed"].as_u64().unwrap() as usize,
        reg.rows_narrowed().len(),
        "capture narrowed count must match the model"
    );
    for (key, kind) in [
        ("ime_entries", ConformanceDomain::ImeGraphemeBidiUnicode),
        ("high_contrast_entries", ConformanceDomain::HighContrast),
        ("zoom_density_entries", ConformanceDomain::ZoomDensity),
        ("pseudoloc_rtl_entries", ConformanceDomain::PseudolocRtl),
        ("locale_pack_entries", ConformanceDomain::LocalePack),
        (
            "desktop_platform_entries",
            ConformanceDomain::DesktopPlatform,
        ),
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
            && !row.publishes_stable()
            && row.signoff_state != ConformanceState::Qualified
    });
    assert!(
        narrowed.is_some(),
        "the register must narrow at least one release-blocking row"
    );
}

#[test]
fn register_shows_a_blocked_or_pending_check() {
    let reg = register();
    let blocked = reg
        .rows
        .iter()
        .find(|row| row.has_blocked_or_pending_check());
    assert!(
        blocked.is_some(),
        "the register must show at least one row with a blocked or pending check"
    );
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.signoff_state == ConformanceState::NotQualified && !row.publishes_stable())
        .expect("register has a narrowed row");
    row.published_label = StableClaimLevel::Stable;
    reg.summary = reg.computed_summary();
    reg.promotion.decision = reg.computed_promotion_decision();
    reg.promotion.blocking_rule_ids = reg.computed_blocking_rule_ids();
    reg.promotion.blocking_entry_ids = reg.computed_blocking_entry_ids();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            DesktopPlatformConformanceViolation::PublishedLabelNotNarrowed { .. }
        )),
        "a row that is not backed must narrow below the cutline"
    );
}

#[test]
fn backed_row_with_blocked_check_fails() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|row| row.signoff_state == ConformanceState::Qualified)
        .expect("register has a qualified row");
    for check in &mut row.conformance_checks {
        if check.check_kind == CheckKind::GraphemeClustering {
            check.check_state = CheckState::Blocked;
            check.evidence_ref = None;
            break;
        }
    }
    reg.summary = reg.computed_summary();

    assert!(
        reg.validate().iter().any(|v| matches!(
            v,
            DesktopPlatformConformanceViolation::HeldWithBlockedCheck { .. }
        )),
        "a backed row may not carry a blocked or pending check"
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
            DesktopPlatformConformanceViolation::HeldOnStalePacket { .. }
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
            DesktopPlatformConformanceViolation::PromotionDecisionInconsistent { .. }
        )),
        "promotion must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/m4/finalize-ime-grapheme-bidi-unicode-high-contrast-zoom-density-pseudoloc-rtl-locale-pack-and-desktop-platform-conformance");
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
        let candidate: DesktopPlatformConformance =
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
