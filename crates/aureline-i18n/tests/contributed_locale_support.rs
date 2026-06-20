//! Fixture replay and host-protection checks for the contributed-locale lane.

use std::path::{Path, PathBuf};

use aureline_i18n::{
    decide_contributed_support, seeded_contributed_locale_support_report, ContributedDegradeReason,
    ContributedEvaluationInput, ContributedLocaleSupportReport, ContributedPackOwnerClass,
    LocalePackSignatureState, LocalizationIssueSourceClass, PackApplicationDecision,
    VersionMatchState, ALL_HOST_STABLE_LABEL_CLASSES,
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
fn fixture_matches_seeded_report_and_validates() {
    let from_file: ContributedLocaleSupportReport =
        load_json("fixtures/i18n/extension-companion-pack-compat/support_report.json");
    let from_code = seeded_contributed_locale_support_report();

    assert_eq!(from_file, from_code);
    from_file.validate().expect("contributed report validates");
}

#[test]
fn host_stable_labels_stay_canonical_on_every_row() {
    let report = seeded_contributed_locale_support_report();
    for row in &report.support_rows {
        assert_eq!(
            row.host_stable_labels_preserved,
            ALL_HOST_STABLE_LABEL_CLASSES.to_vec(),
            "row {} dropped a host-stable label class",
            row.row_id
        );
    }
    for manifest in &report.manifests {
        assert!(!manifest.may_override_host_stable_labels);
    }
}

#[test]
fn issue_source_attribution_covers_extension_and_companion() {
    let report = seeded_contributed_locale_support_report();
    let has_extension = report
        .support_rows
        .iter()
        .any(|r| r.issue_source_class == LocalizationIssueSourceClass::ExtensionPack);
    let has_companion = report
        .support_rows
        .iter()
        .any(|r| r.issue_source_class == LocalizationIssueSourceClass::CompanionOverlay);
    assert!(has_extension && has_companion);
    // The packet names the first-party report it joins against, so support can
    // attribute the third source class too.
    assert_eq!(
        report.first_party_compatibility_report_ref,
        "i18n:m5-locale-pack-compatibility:v1"
    );
}

#[test]
fn decision_matches_each_seeded_row() {
    // Re-deriving the decision from the row inputs must reproduce the stored
    // decision for at least the deterministic, pack-shipping rows.
    let report = seeded_contributed_locale_support_report();
    let row = report
        .row("contributed-support:companion:browser-handoff:ja-jp")
        .expect("companion narrower row");
    let (decision, reason) = decide_contributed_support(&ContributedEvaluationInput {
        owner_class: ContributedPackOwnerClass::CompanionOverlayPack,
        pack_present_for_locale: true,
        target_build_in_compatibility_range: true,
        signature_state: LocalePackSignatureState::SignedVerified,
        version_match_state: VersionMatchState::ExactBuildMatch,
        policy_locale_enabled: true,
        companion_scope_narrower_than_desktop: true,
    });
    assert_eq!(decision, row.application_decision);
    assert_eq!(reason, row.degrade_reason);
    assert_eq!(
        decision,
        PackApplicationDecision::DegradeToSourceLanguageOnly
    );
    assert_eq!(
        reason,
        ContributedDegradeReason::CompanionScopeNarrowerThanDesktop
    );
}
