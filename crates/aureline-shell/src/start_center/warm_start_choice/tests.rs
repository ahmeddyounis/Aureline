use std::path::Path;

use super::*;

fn fixture_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m3/template_and_resume_cards")
        .join(name)
}

#[test]
fn seeded_page_validates() {
    let page = seeded_warm_start_choice_page();
    validate_warm_start_choice_page(&page).expect("seeded page must validate");
    assert_eq!(page.cards.len(), 5);
    assert!(page.honesty_marker_present);
    assert!(page.open_minimal_same_weight_on_local_first);
    assert!(page.set_up_later_same_weight_on_local_first);
}

#[test]
fn validate_card_accepts_every_seeded_card_and_rejects_a_stale_resume() {
    let page = seeded_warm_start_choice_page();
    for card in &page.cards {
        validate_warm_start_choice_card(card)
            .unwrap_or_else(|errors| panic!("seeded card {} must validate: {errors:?}", card.card_id));
    }

    // A stale snapshot whose live-resume lane is suddenly takeable must be
    // rejected by the per-card validator, not just the page validator.
    let mut tampered = page
        .cards
        .iter()
        .find(|card| {
            card.snapshot
                .as_ref()
                .map(|snapshot| snapshot.freshness.is_stale_or_invalidated())
                .unwrap_or(false)
        })
        .expect("seed carries a stale snapshot card")
        .clone();
    if let Some(lane) = tampered
        .choice_lanes
        .iter_mut()
        .find(|lane| lane.path_class == WarmStartPathClass::ResumeLiveWorkspace)
    {
        lane.availability = WarmStartLaneAvailability::Available;
        lane.availability_token = WarmStartLaneAvailability::Available.as_str().to_string();
    }
    let errors = validate_warm_start_choice_card(&tampered)
        .expect_err("a stale snapshot with a takeable live resume must be rejected");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("stale_resume_still_takeable")),
        "expected a stale-resume finding, got {errors:?}"
    );
}

#[test]
fn seeded_page_is_deterministic() {
    let one = seeded_warm_start_choice_page();
    let two = seeded_warm_start_choice_page();
    assert_eq!(one, two);
}

#[test]
fn support_export_validates_against_embedded_page() {
    let export = seeded_warm_start_choice_support_export();
    validate_warm_start_choice_support_export(&export).expect("support export must validate");
    assert!(export.raw_secret_material_excluded);
    assert_eq!(export.card_ids.len(), export.page.cards.len());
}

#[test]
fn every_path_class_is_exercised_by_the_seed() {
    let page = seeded_warm_start_choice_page();
    let mut seen = std::collections::HashSet::new();
    for card in &page.cards {
        for lane in &card.choice_lanes {
            seen.insert(lane.path_class);
        }
    }
    for required in [
        WarmStartPathClass::ResumeLiveWorkspace,
        WarmStartPathClass::StartFromSnapshot,
        WarmStartPathClass::CloneFresh,
        WarmStartPathClass::OpenMinimal,
        WarmStartPathClass::SetUpLater,
        WarmStartPathClass::UseTemplate,
    ] {
        assert!(seen.contains(&required), "missing lane {}", required.as_str());
    }
}

#[test]
fn safest_next_action_is_always_local_safe() {
    let page = seeded_warm_start_choice_page();
    for card in &page.cards {
        let lane = card
            .lane(card.safest_next_action)
            .expect("safest lane present");
        assert!(
            lane.is_local_safe(),
            "card {} default lane is not local-safe",
            card.card_id
        );
        assert!(!card.default_widens_trust);
        assert!(!card.default_runs_networked_work);
    }
}

#[test]
fn stale_snapshot_card_disables_live_resume() {
    let page = seeded_warm_start_choice_page();
    let stale = page
        .cards
        .iter()
        .find(|card| card.card_id == "warm_start_card:snapshot.python_devcontainer.stale")
        .expect("stale snapshot card present");
    let snapshot = stale.snapshot.as_ref().expect("snapshot facts present");
    assert!(snapshot.freshness.is_stale_or_invalidated());
    assert_eq!(snapshot.invalidation_reason.as_deref(), Some("capsule_drift"));

    let resume = stale
        .lane(WarmStartPathClass::ResumeLiveWorkspace)
        .expect("resume lane present");
    assert_eq!(
        resume.availability,
        WarmStartLaneAvailability::UnavailableStaleSnapshot
    );
    assert!(!resume.availability.is_takeable());
    assert!(stale.honesty_marker_present);
}

#[test]
fn managed_live_resume_requires_reauth_and_stays_off_default() {
    let page = seeded_warm_start_choice_page();
    let managed = page
        .cards
        .iter()
        .find(|card| card.card_id == "warm_start_card:live_resume.managed_data_workspace")
        .expect("managed live-resume card present");
    assert!(!managed.local_first);
    let resume = managed
        .lane(WarmStartPathClass::ResumeLiveWorkspace)
        .expect("resume lane present");
    assert_eq!(resume.availability, WarmStartLaneAvailability::RequiresReauth);
    assert!(resume.requires_trust_grant);
    assert!(resume.requires_network);
    assert!(resume.materializes_remote_work);
    // The default still stays local-safe even for a managed card.
    assert_eq!(managed.safest_next_action, WarmStartPathClass::OpenMinimal);
    assert!(managed.honesty_marker_present);
}

#[test]
fn local_first_cards_keep_escape_hatches_same_weight() {
    let page = seeded_warm_start_choice_page();
    for card in page.cards.iter().filter(|card| card.local_first) {
        for path in [WarmStartPathClass::OpenMinimal, WarmStartPathClass::SetUpLater] {
            let lane = card.lane(path).expect("escape hatch present");
            assert!(lane.same_weight_local_path);
            assert!(lane.is_local_safe());
        }
    }
}

#[test]
fn validator_rejects_remote_lane_masquerading_as_local() {
    let mut page = seeded_warm_start_choice_page();
    // Flip the clone-fresh lane to claim it has no side effect.
    let card = page
        .cards
        .iter_mut()
        .find(|card| card.card_id == "warm_start_card:clone_fresh.platform_repository")
        .expect("clone-fresh card present");
    let lane = card
        .choice_lanes
        .iter_mut()
        .find(|lane| lane.path_class == WarmStartPathClass::CloneFresh)
        .expect("clone-fresh lane present");
    lane.side_effect_class = WarmStartSideEffectClass::NoSideEffect;
    lane.side_effect_token = WarmStartSideEffectClass::NoSideEffect.as_str().to_string();

    let errors = validate_warm_start_choice_page(&page)
        .expect_err("masquerading remote lane must be rejected");
    assert!(errors
        .iter()
        .any(|error| error.contains("remote_masquerades_as_local")));
}

#[test]
fn validator_rejects_stale_snapshot_with_takeable_resume() {
    let mut page = seeded_warm_start_choice_page();
    let card = page
        .cards
        .iter_mut()
        .find(|card| card.card_id == "warm_start_card:snapshot.python_devcontainer.stale")
        .expect("stale card present");
    let resume = card
        .choice_lanes
        .iter_mut()
        .find(|lane| lane.path_class == WarmStartPathClass::ResumeLiveWorkspace)
        .expect("resume lane present");
    resume.availability = WarmStartLaneAvailability::Available;
    resume.availability_token = WarmStartLaneAvailability::Available.as_str().to_string();
    // Recompute honesty marker so the inconsistency error does not mask this one.
    card.honesty_marker_present = true;

    let errors =
        validate_warm_start_choice_page(&page).expect_err("stale resume must be rejected");
    assert!(errors
        .iter()
        .any(|error| error.contains("stale_resume_still_takeable")));
}

#[test]
fn validator_rejects_non_local_safe_default() {
    let mut page = seeded_warm_start_choice_page();
    let card = page
        .cards
        .iter_mut()
        .find(|card| card.card_id == "warm_start_card:clone_fresh.platform_repository")
        .expect("clone-fresh card present");
    card.safest_next_action = WarmStartPathClass::CloneFresh;
    card.safest_next_action_token = WarmStartPathClass::CloneFresh.as_str().to_string();

    let errors = validate_warm_start_choice_page(&page)
        .expect_err("non-local-safe default must be rejected");
    assert!(errors
        .iter()
        .any(|error| error.contains("safest_next_action.not_local_safe")));
}

#[test]
fn plaintext_is_deterministic_and_lists_every_card() {
    let page = seeded_warm_start_choice_page();
    let first = render_warm_start_choice_plaintext(&page);
    let second = render_warm_start_choice_plaintext(&page);
    assert_eq!(first, second);
    assert!(first.contains("Warm-start choices beta"));
    for card in &page.cards {
        assert!(first.contains(&card.card_id), "missing {} in plaintext", card.card_id);
    }
}

#[test]
fn vocabulary_lists_the_full_path_taxonomy() {
    let vocabulary = warm_start_choice_vocabulary();
    let paths = vocabulary
        .iter()
        .find(|(dimension, _)| *dimension == "path_class")
        .map(|(_, values)| values.clone())
        .expect("path_class dimension present");
    for required in [
        "resume_live_workspace",
        "start_from_snapshot",
        "clone_fresh",
        "open_minimal",
        "set_up_later",
        "use_template",
    ] {
        assert!(paths.contains(&required), "missing path token {required}");
    }
}

#[test]
fn seeded_page_matches_checked_in_fixture() {
    let page = seeded_warm_start_choice_page();
    let actual = serde_json::to_string_pretty(&page).expect("page serializes");
    let expected = std::fs::read_to_string(fixture_path("seeded_warm_start_choice_page.json"))
        .expect("seeded page fixture must read");
    assert_eq!(
        actual.trim_end(),
        expected.trim_end(),
        "seeded warm-start choice page drifted from the checked-in fixture; regenerate it with \
         `cargo run -q -p aureline-shell --bin aureline_shell_warm_start_choice -- page`"
    );

    // The fixture must round-trip back into a valid page.
    let parsed: WarmStartChoicePage =
        serde_json::from_str(&expected).expect("fixture must deserialize");
    validate_warm_start_choice_page(&parsed).expect("fixture page must validate");
    assert_eq!(parsed, page);
}

#[test]
fn checked_in_card_fixtures_match_seeded_cards() {
    let page = seeded_warm_start_choice_page();
    let cases = [
        ("template_card.json", "warm_start_card:template.ts_web.local"),
        (
            "live_resume_card.json",
            "warm_start_card:live_resume.managed_data_workspace",
        ),
        (
            "valid_snapshot_card.json",
            "warm_start_card:snapshot.ts_web.local_fresh",
        ),
        (
            "stale_snapshot_card.json",
            "warm_start_card:snapshot.python_devcontainer.stale",
        ),
        (
            "clone_fresh_card.json",
            "warm_start_card:clone_fresh.platform_repository",
        ),
    ];
    for (file, card_id) in cases {
        let card = page
            .cards
            .iter()
            .find(|card| card.card_id == card_id)
            .unwrap_or_else(|| panic!("seeded card {card_id} present"));
        let actual = serde_json::to_string_pretty(card).expect("card serializes");
        let expected = std::fs::read_to_string(fixture_path(file))
            .unwrap_or_else(|_| panic!("card fixture {file} must read"));
        assert_eq!(
            actual.trim_end(),
            expected.trim_end(),
            "card fixture {file} drifted from the seeded card"
        );
    }
}
