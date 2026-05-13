use std::collections::BTreeSet;
use std::path::Path;

use aureline_shell::onboarding::{
    build_onboarding_alpha_surface, EntryVerbClass, LearningDigestAvailability,
    OnboardingStateKind, PackInstallState, RecommendationActionClass, SourceLanguageFallbackClass,
    ONBOARDING_ALPHA_FIXTURE_GENERATED_AT,
};

fn fixture(path: &str) -> serde_json::Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    serde_json::from_str(&payload).expect("fixture must parse")
}

#[test]
fn first_run_fixture_matches_generated_onboarding_contract() {
    let expected = fixture("../../fixtures/ux/onboarding_alpha/first_run_no_account_surface.json");
    let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);

    assert_eq!(
        expected["requires_no_account_local_work"].as_bool(),
        Some(surface.no_account_path.local_work_available)
    );

    let expected_verbs = expected["expected_entry_verb_classes"]
        .as_array()
        .expect("expected verbs")
        .iter()
        .map(|value| value.as_str().expect("verb string").to_string())
        .collect::<BTreeSet<_>>();
    let actual_verbs = surface
        .entry_verbs
        .iter()
        .map(|row| row.entry_verb_class.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_verbs, expected_verbs);

    let expected_actions = expected["expected_recommendation_actions"]
        .as_array()
        .expect("expected actions")
        .iter()
        .map(|value| value.as_str().expect("action string").to_string())
        .collect::<Vec<_>>();
    let actual_actions = surface.recommendation_cards[0]
        .actions
        .iter()
        .map(|action| action.action_class.as_str().to_string())
        .collect::<Vec<_>>();
    assert_eq!(actual_actions, expected_actions);

    assert!(surface.entry_verbs.iter().all(|row| {
        row.command.command_id.starts_with("cmd:") && !row.command.keyboard_route.is_empty()
    }));
}

#[test]
fn state_fixture_matches_portable_progress_distinctions() {
    let expected = fixture("../../fixtures/ux/onboarding_alpha/first_run_no_account_surface.json");
    let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);

    let expected_kinds = expected["expected_state_kinds"]
        .as_array()
        .expect("expected state kinds")
        .iter()
        .map(|value| value.as_str().expect("state string").to_string())
        .collect::<BTreeSet<_>>();
    let actual_kinds = surface
        .portable_state
        .items
        .iter()
        .map(|item| item.state_kind.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_kinds, expected_kinds);
    assert!(!surface.portable_state.any_workspace_local_hidden_store);
    assert!(surface
        .portable_state
        .items
        .iter()
        .all(|item| item.storage_lane.as_str() == "portable_user_profile_state"));

    let must_include = [
        OnboardingStateKind::DismissedHint,
        OnboardingStateKind::CompletedTask,
        OnboardingStateKind::DeferredSetup,
        OnboardingStateKind::ProtectedRecoveryRecommendation,
        OnboardingStateKind::ImportedProfileHistory,
    ];
    for kind in must_include {
        assert!(surface
            .portable_state
            .items
            .iter()
            .any(|item| item.state_kind == kind));
    }
}

#[test]
fn help_search_fixture_proves_locale_fallback_and_pack_posture() {
    let expected =
        fixture("../../fixtures/ux/onboarding_help_search_alpha/source_language_fallback.json");
    let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);

    let fallback = surface
        .help_search
        .items
        .iter()
        .find(|item| {
            item.source_language_fallback_class
                == SourceLanguageFallbackClass::FallbackToSourceLanguageDisclosed
        })
        .expect("source-language fallback item");
    assert_eq!(
        fallback.requested_locale,
        expected["requested_locale"]
            .as_str()
            .expect("requested locale")
    );
    assert_eq!(
        fallback.effective_locale,
        expected["effective_locale"]
            .as_str()
            .expect("effective locale")
    );
    assert_eq!(
        fallback.command.command_id,
        expected["expected_command_id"]
            .as_str()
            .expect("command id")
    );
    assert!(!fallback.citation_refs.is_empty());
    assert_eq!(
        fallback.docs_node_id.as_deref(),
        Some("docs-node:onboarding.keymap-bridge")
    );
    assert_eq!(
        fallback.source_pack_revision_ref.as_deref(),
        Some("pack-rev:project:aureline:2026.05.13-01")
    );
    assert_eq!(
        fallback.citation_drawer_ref.as_deref(),
        Some("citation-drawer:docs-pack:onboarding.keymap-bridge")
    );

    let pack_states = surface
        .help_search
        .pack_states
        .iter()
        .map(|pack| pack.install_state)
        .collect::<Vec<_>>();
    assert!(pack_states.contains(&PackInstallState::LocalOnlyStarter));
    assert!(pack_states.contains(&PackInstallState::CachedSnapshotCurrent));
    assert!(pack_states.contains(&PackInstallState::NotInstalled));
}

#[test]
fn learning_digest_placeholder_preserves_local_continuity() {
    let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
    assert_eq!(
        surface.learning_digest.availability_class,
        LearningDigestAvailability::NotInstalledPlaceholder
    );
    assert!(surface.learning_digest.no_account_continuity_preserved);
    assert!(surface.learning_digest.exact_reopen_preserves_anchors);
}

#[test]
fn recommendation_card_has_same_weight_safe_paths() {
    let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
    let card = surface
        .recommendation_cards
        .iter()
        .find(|card| card.recommendation_ref == "launch_bundle:typescript_web_app.seed")
        .expect("launch bundle recommendation");
    let actions = card
        .actions
        .iter()
        .map(|action| action.action_class)
        .collect::<Vec<_>>();
    assert_eq!(
        actions,
        vec![
            RecommendationActionClass::Apply,
            RecommendationActionClass::Compare,
            RecommendationActionClass::Dismiss,
            RecommendationActionClass::OpenMinimal,
            RecommendationActionClass::SetUpLater,
        ]
    );
    assert!(card.review_required_on_later_open);
    assert!(!card.can_silently_install);
    assert!(!card.can_silently_widen_trust);
}

#[test]
fn render_plaintext_exports_reviewable_summary() {
    let surface = build_onboarding_alpha_surface(ONBOARDING_ALPHA_FIXTURE_GENERATED_AT);
    let text = surface.render_plaintext();
    assert!(text.contains("Onboarding alpha"));
    assert!(text.contains("entry_verbs:"));
    assert!(text.contains("help_search:"));
    assert!(text.contains("portable_state:"));
    assert!(text.contains(EntryVerbClass::RecentWork.as_str()));
}
