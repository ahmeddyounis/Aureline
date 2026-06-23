//! Unit tests for the consolidated locale diagnostics packet.

use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_locale_diagnostics_packet();
    packet.validate().expect("seeded packet validates");
}

#[test]
fn active_locale_is_a_profiled_locale() {
    let packet = seeded_locale_diagnostics_packet();
    assert_eq!(packet.requested_locale, SEEDED_ACTIVE_LOCALE);
    assert!(
        packet.active_profile().is_some(),
        "active locale must have a diagnostics profile"
    );
}

#[test]
fn every_profile_carries_a_problem_origin_and_chain() {
    let packet = seeded_locale_diagnostics_packet();
    for profile in &packet.locale_profiles {
        assert_eq!(
            profile.fallback_chain.first(),
            Some(&profile.requested_locale)
        );
        assert_eq!(
            profile.fallback_chain.last(),
            Some(&packet.source_language_locale)
        );
        // Route activity tracks degradation exactly.
        assert_eq!(
            profile.source_language_route_active,
            profile.problem_origin.is_degraded()
        );
    }
}

#[test]
fn support_export_is_metadata_only_and_origin_bearing() {
    let export = seeded_locale_diagnostics_support_export();
    assert!(!export.raw_translated_bodies_exported);
    assert!(export
        .installed_pack_rows
        .iter()
        .all(|row| row.raw_translated_body_omitted));
    assert!(export
        .profile_rows
        .iter()
        .all(|row| row.raw_translated_body_omitted));
    assert!(!export.preserved_stable_anchor_refs.is_empty());
    assert!(export
        .omitted_material_classes
        .contains(&"locale_pack_signing_keys".to_owned()));
    // Every of the five problem-origin buckets is representable and assigned.
    assert!(export
        .profile_rows
        .iter()
        .all(|row| LocaleProblemOrigin::ALL.contains(&row.problem_origin)));
}

#[test]
fn support_export_distinguishes_pack_skew_from_source_language_fallback() {
    let export = seeded_locale_diagnostics_support_export();
    let origins: std::collections::BTreeSet<LocaleProblemOrigin> = export
        .profile_rows
        .iter()
        .map(|row| row.problem_origin)
        .collect();
    assert!(
        origins.contains(&LocaleProblemOrigin::PackSkew),
        "seeded spread should include an incompatible pack (de-DE)"
    );
    assert!(
        origins.contains(&LocaleProblemOrigin::SourceLanguageFallback),
        "seeded spread should include a signature-failure source-language fallback (ja-JP)"
    );
    assert!(
        origins.contains(&LocaleProblemOrigin::RequestedLocale),
        "seeded spread should include a fully localized locale (es-MX)"
    );
}

#[test]
fn release_gate_narrows_incompatible_and_degraded_claims() {
    let packet = seeded_locale_diagnostics_packet();
    let gate = &packet.release_gate;
    assert!(gate.any_claim_narrowed);
    assert!(gate.any_claim_blocked);

    // A pack-skew locale is blocked and not publishable; a source-language
    // fallback is narrowed and not publishable.
    let blocked = gate
        .rows
        .iter()
        .find(|row| row.problem_origin == LocaleProblemOrigin::PackSkew)
        .expect("a pack-skew claim is present");
    assert_eq!(
        blocked.gate_state,
        LocaleClaimGateState::ClaimBlockedIncompatiblePack
    );
    assert!(!blocked.publishable_localized_claim);

    let source_fallback = gate
        .rows
        .iter()
        .find(|row| row.problem_origin == LocaleProblemOrigin::SourceLanguageFallback)
        .expect("a source-language claim is present");
    assert!(source_fallback.narrowed);
    assert!(!source_fallback.publishable_localized_claim);

    // The source language itself is not gated as a claim row.
    assert!(gate
        .rows
        .iter()
        .all(|row| row.claimed_locale != packet.source_language_locale));
}

#[test]
fn help_about_card_lights_honesty_marker_when_active_locale_degrades() {
    let packet = seeded_locale_diagnostics_packet();
    // The seeded active locale (de-DE) ships an incompatible pack.
    assert!(packet.problem_origin.is_degraded());
    assert!(packet.help_about_card.honesty_marker_present);
    assert!(packet.help_about_card.incompatible_pack_count >= 1);
}

#[test]
fn summary_problem_origin_counts_sum_to_profiles() {
    let packet = seeded_locale_diagnostics_packet();
    let total: usize = packet.summary.problem_origin_counts.values().sum();
    assert_eq!(total, packet.locale_profiles.len());
    assert_eq!(
        packet.summary.profiled_locale_count,
        packet.locale_profiles.len()
    );
}

#[test]
fn tampering_with_summary_is_caught_by_validate() {
    let mut packet = seeded_locale_diagnostics_packet();
    packet.summary.total_missing_key_count += 1;
    assert!(packet.validate().is_err());
}

#[test]
fn tampering_with_a_blocked_claim_publishability_is_caught() {
    let mut packet = seeded_locale_diagnostics_packet();
    if let Some(row) = packet
        .release_gate
        .rows
        .iter_mut()
        .find(|row| row.problem_origin == LocaleProblemOrigin::PackSkew)
    {
        row.publishable_localized_claim = true;
    }
    assert!(packet.validate().is_err());
}
