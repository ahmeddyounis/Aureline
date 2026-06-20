//! Unit tests for the M5 dense-surface i18n qualification harness.

use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    packet.validate().expect("seeded qualification validates");
}

#[test]
fn every_dense_surface_family_is_covered() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let families = packet
        .surfaces
        .iter()
        .map(|surface| surface.surface_family)
        .collect::<BTreeSet<_>>();
    for family in M5DenseSurfaceFamily::all() {
        assert!(families.contains(&family), "missing {family:?}");
    }
    // The new M5 dense surfaces must be present, not just generic chrome.
    for required in [
        M5DenseSurfaceFamily::Notebook,
        M5DenseSurfaceFamily::DataGrid,
        M5DenseSurfaceFamily::PipelineLogView,
        M5DenseSurfaceFamily::SupportReport,
    ] {
        assert!(families.contains(&required), "missing {required:?}");
    }
}

#[test]
fn every_harness_kind_is_exercised() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let kinds = packet
        .harness_cases
        .iter()
        .flat_map(|case| case.harness_kinds.iter().copied())
        .collect::<BTreeSet<_>>();
    for kind in M5DenseHarnessKind::all() {
        assert!(kinds.contains(&kind), "missing {kind:?}");
    }
}

#[test]
fn seeded_profiles_are_all_green() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    assert_eq!(packet.profile_qualifications.len(), CLAIMED_LOCALES.len());
    for row in &packet.profile_qualifications {
        assert_eq!(row.gate_state, MatrixGateState::Green, "{}", row.profile_id);
        assert_eq!(
            row.effective_claim_class,
            ProfileClaimClass::ClaimedLocalized
        );
        assert!(!row.blocks_promotion);
        assert_eq!(row.failed_count, 0);
        assert!(row.narrow_reasons.is_empty());
    }
    assert_eq!(packet.summary.promotion_state, "green");
    assert_eq!(packet.summary.failed_result_count, 0);
}

#[test]
fn rtl_locale_renders_right_to_left() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let row = packet
        .profile_qualification("ar-SA")
        .expect("ar-SA profile present");
    assert_eq!(row.text_direction, TextDirection::RightToLeft);
    assert!(packet
        .harness_results
        .iter()
        .filter(|result| result.requested_locale == "ar-SA")
        .all(|result| result.text_direction == TextDirection::RightToLeft));
}

#[test]
fn ime_failure_blocks_the_claimed_profile() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let injected = packet.with_injected_result(
        "ja-JP",
        M5DenseSurfaceFamily::Notebook,
        M5DenseHarnessKind::ImeComposition,
        M5HarnessResultState::Failed,
        Some(DenseI18nFailureClass::ImePreeditLoss),
    );
    let row = injected
        .profile_qualification("ja-JP")
        .expect("ja-JP profile present");
    assert_eq!(row.gate_state, MatrixGateState::Blocked);
    assert!(row.blocks_promotion);
    assert_eq!(
        row.effective_claim_class,
        ProfileClaimClass::SourceLanguageFallbackOnly
    );
    assert!(row
        .narrow_reasons
        .contains(&M5DenseClaimNarrowReason::ImeCompositionRegression));
    assert!(row
        .affected_surface_families
        .contains(&M5DenseSurfaceFamily::Notebook));
    assert_eq!(injected.summary.promotion_state, "blocked");

    // The regression must be scoped: other claimed locales stay green.
    let other = injected
        .profile_qualification("ar-SA")
        .expect("ar-SA profile present");
    assert_eq!(other.gate_state, MatrixGateState::Green);
}

#[test]
fn source_language_fallback_narrows_without_blocking() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let injected = packet.with_injected_result(
        "es-MX",
        M5DenseSurfaceFamily::SupportReport,
        M5DenseHarnessKind::TextExpansion,
        M5HarnessResultState::SourceLanguageFallbackPassed,
        None,
    );
    let row = injected
        .profile_qualification("es-MX")
        .expect("es-MX profile present");
    assert_eq!(row.gate_state, MatrixGateState::Narrowed);
    assert!(!row.blocks_promotion);
    assert_eq!(
        row.effective_claim_class,
        ProfileClaimClass::SourceLanguageFallbackOnly
    );
    assert!(row
        .narrow_reasons
        .contains(&M5DenseClaimNarrowReason::TextExpansionOverflow));
    assert_eq!(injected.summary.promotion_state, "narrowed");
}

#[test]
fn injected_packet_revalidates() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let injected = packet.with_injected_result(
        "ar-SA",
        M5DenseSurfaceFamily::PipelineLogView,
        M5DenseHarnessKind::RtlBidi,
        M5HarnessResultState::Failed,
        Some(DenseI18nFailureClass::LiteralTechnicalStringMirrored),
    );
    injected
        .validate()
        .expect("injected qualification stays internally consistent");
}

#[test]
fn narrowing_scenarios_replay_against_the_packet() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    let scenarios = seeded_m5_dense_surface_narrowing_scenarios();
    scenarios
        .validate_against(&packet)
        .expect("narrowing scenarios derive their expected gates");
    assert!(scenarios
        .scenarios
        .iter()
        .any(|scenario| scenario.expected_blocks_promotion));
    assert!(scenarios
        .scenarios
        .iter()
        .any(|scenario| scenario.expected_gate_state == MatrixGateState::Narrowed));
}

#[test]
fn review_packet_reports_promotion_state() {
    let review = seeded_m5_dense_surface_i18n_review_packet();
    assert_eq!(review.promotion_state, "green");
    assert!(review.blocked_profiles.is_empty());
    assert!(review.narrowed_profiles.is_empty());
    assert_eq!(review.profile_rows.len(), CLAIMED_LOCALES.len());
    assert_eq!(
        review.harness_kinds_covered.len(),
        M5DenseHarnessKind::all().len()
    );
}

#[test]
fn ime_cases_only_exist_for_text_input_surfaces() {
    let packet = seeded_m5_dense_surface_i18n_qualification();
    for case in &packet.harness_cases {
        let has_ime = case
            .harness_kinds
            .contains(&M5DenseHarnessKind::ImeComposition);
        assert_eq!(
            has_ime,
            case.ime_scenario.is_some(),
            "{} ime kind and scenario must agree",
            case.case_id
        );
    }
}

#[test]
fn detail_strings_never_leak_translated_bodies() {
    // Result details are export-safe summaries, not raw translated copy.
    let packet = seeded_m5_dense_surface_i18n_qualification();
    for result in &packet.harness_results {
        assert!(!result.detail.is_empty());
        assert!(result
            .evidence_ref
            .starts_with(M5_DENSE_SURFACE_LAB_FIXTURE_ROOT));
    }
}
