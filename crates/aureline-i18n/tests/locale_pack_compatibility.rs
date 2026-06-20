//! Fixture replay and skew-handling checks for the locale-pack delivery lane.

use std::path::{Path, PathBuf};

use aureline_i18n::{
    decide_application, seeded_core_locale_pack_artifacts,
    seeded_locale_pack_compatibility_report, LocalePackArtifact, LocalePackCompatibilityReport,
    LocalePackSignatureState, PackApplicationDecision, PackEvaluationInput, SkewDegradeReason,
    VersionMatchState,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_json<T: serde::de::DeserializeOwned>(rel: &str) -> T {
    let path = repo_root().join(rel);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn report_fixture_matches_seeded_report_and_validates() {
    let from_file: LocalePackCompatibilityReport =
        load_json("fixtures/i18n/pack-skew-and-signature/compatibility_report.json");
    let from_code = seeded_locale_pack_compatibility_report();

    assert_eq!(from_file, from_code);
    from_file
        .validate()
        .expect("compatibility report validates");
}

#[test]
fn checked_in_core_artifacts_match_seeded_artifacts() {
    let cases = [
        ("locale-packs/core/en-US/pack.json", "locale-pack:core:source:en-us"),
        ("locale-packs/core/es-MX/pack.json", "locale-pack:core:es-mx"),
        ("locale-packs/core/fr-FR/pack.json", "locale-pack:core:fr-fr"),
        ("locale-packs/core/ja-JP/pack.json", "locale-pack:core:ja-jp"),
        ("locale-packs/core/de-DE/pack.json", "locale-pack:core:de-de"),
    ];
    let seeded = seeded_core_locale_pack_artifacts();
    assert_eq!(seeded.len(), cases.len());

    for (rel, pack_id) in cases {
        let from_file: LocalePackArtifact = load_json(rel);
        from_file.validate().expect("core pack artifact validates");
        assert_eq!(from_file.pack_id, pack_id);
        let from_code = seeded
            .iter()
            .find(|artifact| artifact.pack_id == pack_id)
            .expect("seeded artifact exists");
        assert_eq!(&from_file, from_code, "{rel} drifted from seeded artifact");
    }
}

#[test]
fn signature_failure_degrades_fully_without_applying_translations() {
    let report = seeded_locale_pack_compatibility_report();
    let ja = report.row("locale-pack:core:ja-jp").expect("ja-jp row");

    // The on-disk pack is fully translated, yet a signature failure must drop
    // every key to source language rather than apply a single stale string.
    let artifact = report.artifact("locale-pack:core:ja-jp").expect("ja-jp artifact");
    assert_eq!(artifact.declared_missing_key_count(), 0);

    assert_eq!(
        ja.application_decision,
        PackApplicationDecision::DegradeToSourceLanguageOnly
    );
    assert_eq!(ja.skew_degrade_reason, SkewDegradeReason::SignatureFailed);
    assert_eq!(ja.effective_locale, "en-US");
    assert_eq!(ja.missing_key_count, ja.total_key_count);
    assert!(!ja.claimed_localized_profile);
    assert!(ja.non_blocking_core_use);
}

#[test]
fn version_skew_degrades_fully_to_source_language() {
    let report = seeded_locale_pack_compatibility_report();
    let de = report.row("locale-pack:core:de-de").expect("de-de row");

    assert_eq!(
        de.application_decision,
        PackApplicationDecision::DegradeToSourceLanguageOnly
    );
    assert_eq!(
        de.skew_degrade_reason,
        SkewDegradeReason::BuildOutsideCompatibilityRange
    );
    assert!(!de.target_build_in_compatibility_range);
    assert_eq!(de.missing_key_count, de.total_key_count);
    assert!(!de.claimed_localized_profile);
}

#[test]
fn renderable_partial_pack_applies_and_discloses_missing_keys() {
    let report = seeded_locale_pack_compatibility_report();
    let fr = report.row("locale-pack:core:fr-fr").expect("fr-fr row");

    assert_eq!(
        fr.application_decision,
        PackApplicationDecision::ApplyLocalizedWithDisclosedMissingKeys
    );
    assert_eq!(fr.skew_degrade_reason, SkewDegradeReason::NotDegraded);
    assert_eq!(fr.effective_locale, "fr-FR");
    assert!(fr.missing_key_count > 0 && fr.missing_key_count < fr.total_key_count);
    assert!(fr.claimed_localized_profile);
    assert_eq!(
        fr.missing_key_count_by_surface.get("docs_tour_or_auth_text"),
        Some(&3)
    );
}

#[test]
fn unsigned_accepted_pack_applies_but_is_never_claimed() {
    let report = seeded_locale_pack_compatibility_report();
    let pt = report.row("locale-pack:community:pt-br").expect("pt-br row");

    assert_eq!(
        pt.signature_state,
        LocalePackSignatureState::UnsignedExplicitAcceptance
    );
    assert!(pt.application_decision.applies());
    assert!(!pt.claimed_localized_profile);
    assert!(pt.explicit_acceptance_decision_row_ref.is_some());
}

#[test]
fn decide_application_is_conservative_about_skew() {
    let base = PackEvaluationInput {
        target_build_identity_ref: "build:test".to_owned(),
        target_build_in_compatibility_range: true,
        signature_state: LocalePackSignatureState::SignedVerified,
        version_match_state: VersionMatchState::ExactBuildMatch,
        integrity_digest_matches: true,
        pack_present: true,
        policy_locale_enabled: true,
    };
    assert_eq!(
        decide_application(&base),
        (
            PackApplicationDecision::ApplyLocalizedWithDisclosedMissingKeys,
            SkewDegradeReason::NotDegraded
        )
    );

    let cases = [
        (
            PackEvaluationInput { pack_present: false, ..base.clone() },
            SkewDegradeReason::PackMissing,
        ),
        (
            PackEvaluationInput { policy_locale_enabled: false, ..base.clone() },
            SkewDegradeReason::PolicyDisabledLocale,
        ),
        (
            PackEvaluationInput {
                signature_state: LocalePackSignatureState::SignatureFailedBlocked,
                ..base.clone()
            },
            SkewDegradeReason::SignatureFailed,
        ),
        (
            PackEvaluationInput {
                signature_state: LocalePackSignatureState::SignedUnverified,
                ..base.clone()
            },
            SkewDegradeReason::SignatureUnverifiedNotAccepted,
        ),
        (
            PackEvaluationInput { integrity_digest_matches: false, ..base.clone() },
            SkewDegradeReason::IntegrityDigestMismatch,
        ),
        (
            PackEvaluationInput {
                target_build_in_compatibility_range: false,
                version_match_state: VersionMatchState::IncompatibleDriftDetected,
                ..base.clone()
            },
            SkewDegradeReason::BuildOutsideCompatibilityRange,
        ),
        (
            PackEvaluationInput {
                version_match_state: VersionMatchState::UnknownTargetBuild,
                ..base.clone()
            },
            SkewDegradeReason::UnknownTargetBuild,
        ),
    ];

    for (input, expected_reason) in cases {
        let (decision, reason) = decide_application(&input);
        assert_eq!(
            decision,
            PackApplicationDecision::DegradeToSourceLanguageOnly,
            "expected degrade for {expected_reason:?}"
        );
        assert_eq!(reason, expected_reason);
    }
}

#[test]
fn unsigned_explicit_acceptance_still_renders() {
    let input = PackEvaluationInput {
        target_build_identity_ref: "build:test".to_owned(),
        target_build_in_compatibility_range: true,
        signature_state: LocalePackSignatureState::UnsignedExplicitAcceptance,
        version_match_state: VersionMatchState::CompatibleMinorDrift,
        integrity_digest_matches: true,
        pack_present: true,
        policy_locale_enabled: true,
    };
    let (decision, reason) = decide_application(&input);
    assert!(decision.applies());
    assert_eq!(reason, SkewDegradeReason::NotDegraded);
}
