//! Fixture replay for the M5 dense-surface i18n qualification harness.
//!
//! Confirms the checked-in qualification, review, and narrowing fixtures match
//! the seeded truth, that the gate derives from harness results, and that an
//! IME, RTL/bidi, font-fallback, or localized-format regression blocks or
//! narrows a claimed localized profile.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_i18n::{
    locale_text_direction, seeded_m5_dense_surface_i18n_qualification,
    seeded_m5_dense_surface_i18n_review_packet, seeded_m5_dense_surface_narrowing_scenarios,
    DenseI18nFailureClass, M5DenseClaimNarrowReason, M5DenseHarnessKind,
    M5DenseNarrowingScenarioSet, M5DenseSurfaceFamily, M5DenseSurfaceI18nQualification,
    M5DenseSurfaceI18nReviewPacket, M5HarnessResultState, MatrixGateState, ProfileClaimClass,
    TextDirection,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/i18n/pseudoloc-rtl-ime-cjk")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn qualification_fixture_matches_seeded_packet() {
    let from_file: M5DenseSurfaceI18nQualification = load_json("qualification.json");
    let from_code = seeded_m5_dense_surface_i18n_qualification();
    assert_eq!(from_file, from_code);
    from_file.validate().expect("qualification validates");
}

#[test]
fn review_fixture_matches_seeded_packet() {
    let from_file: M5DenseSurfaceI18nReviewPacket = load_json("review_export.json");
    let from_code = seeded_m5_dense_surface_i18n_review_packet();
    assert_eq!(from_file, from_code);
    assert_eq!(from_file.promotion_state, "green");
}

#[test]
fn narrowing_fixture_matches_seeded_scenarios() {
    let from_file: M5DenseNarrowingScenarioSet = load_json("narrowing_cases.json");
    let from_code = seeded_m5_dense_surface_narrowing_scenarios();
    assert_eq!(from_file, from_code);

    let packet = seeded_m5_dense_surface_i18n_qualification();
    from_file
        .validate_against(&packet)
        .expect("every narrowing scenario derives its expected gate");
}

#[test]
fn dense_m5_surfaces_are_covered_with_proof() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let families = packet
        .surfaces
        .iter()
        .map(|surface| surface.surface_family)
        .collect::<BTreeSet<_>>();
    for required in M5DenseSurfaceFamily::all() {
        assert!(families.contains(&required), "missing {required:?}");
    }
    // Each surface must have at least one harness result per claimed locale.
    for surface in &packet.surfaces {
        let result_count = packet
            .harness_results
            .iter()
            .filter(|row| row.surface_family == surface.surface_family)
            .count();
        assert!(
            result_count >= 3,
            "{} lacks dense-surface proof",
            surface.surface_id
        );
    }
}

#[test]
fn all_seven_harnesses_run_on_claimed_locales() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    for kind in M5DenseHarnessKind::all() {
        for locale in ["es-MX", "ja-JP", "ar-SA"] {
            let present = packet
                .harness_results
                .iter()
                .any(|row| row.harness_kind == kind && row.requested_locale == locale);
            assert!(present, "missing {kind:?} for {locale}");
        }
    }
}

#[test]
fn ime_regression_blocks_promotion_on_the_claimed_row() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let injected = packet.with_injected_result(
        "ja-JP",
        M5DenseSurfaceFamily::Notebook,
        M5DenseHarnessKind::ImeComposition,
        M5HarnessResultState::Failed,
        Some(DenseI18nFailureClass::ImePreeditLoss),
    );
    injected
        .validate()
        .expect("injected packet stays consistent");
    let row = injected
        .profile_qualification("ja-JP")
        .expect("row present");
    assert_eq!(row.gate_state, MatrixGateState::Blocked);
    assert!(row.blocks_promotion);
    assert_eq!(
        row.effective_claim_class,
        ProfileClaimClass::SourceLanguageFallbackOnly
    );
    assert!(row
        .narrow_reasons
        .contains(&M5DenseClaimNarrowReason::ImeCompositionRegression));
    assert_eq!(injected.summary.promotion_state, "blocked");
}

#[test]
fn rtl_font_and_format_regressions_each_block_promotion() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let cases = [
        (
            "ar-SA",
            M5DenseSurfaceFamily::PipelineLogView,
            M5DenseHarnessKind::RtlBidi,
            DenseI18nFailureClass::LiteralTechnicalStringMirrored,
            M5DenseClaimNarrowReason::RtlBidiMirrorRegression,
        ),
        (
            "ja-JP",
            M5DenseSurfaceFamily::DataGrid,
            M5DenseHarnessKind::FontFallback,
            DenseI18nFailureClass::MissingGlyphOrWrongFontFallback,
            M5DenseClaimNarrowReason::FontFallbackRegression,
        ),
        (
            "es-MX",
            M5DenseSurfaceFamily::DataGrid,
            M5DenseHarnessKind::LocalizedDateNumber,
            DenseI18nFailureClass::LocalizedDateNumberDrift,
            M5DenseClaimNarrowReason::LocalizedFormatRegression,
        ),
    ];
    for (locale, family, harness, failure, reason) in cases {
        let injected = packet.with_injected_result(
            locale,
            family,
            harness,
            M5HarnessResultState::Failed,
            Some(failure),
        );
        let row = injected.profile_qualification(locale).expect("row present");
        assert_eq!(
            row.gate_state,
            MatrixGateState::Blocked,
            "{locale}/{harness:?}"
        );
        assert!(row.blocks_promotion);
        assert!(row.narrow_reasons.contains(&reason));
    }
}

#[test]
fn rtl_locale_results_render_right_to_left() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    assert_eq!(locale_text_direction("ar-SA"), TextDirection::RightToLeft);
    assert_eq!(locale_text_direction("es-MX"), TextDirection::LeftToRight);
    assert!(packet
        .harness_results
        .iter()
        .filter(|row| row.requested_locale == "ar-SA")
        .all(|row| row.text_direction == TextDirection::RightToLeft));
}
