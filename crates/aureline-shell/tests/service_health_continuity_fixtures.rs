//! Fixture-replay for the service-health continuity-drill corpus. The
//! drills live under `fixtures/ops/m3/service_health_continuity/` and are
//! minted by the `aureline_shell_service_health_continuity_corpus`
//! emitter so the checked-in JSON stays a literal projection of the
//! in-code corpus.
//!
//! What this guards:
//!
//! - Each drill's aggregator on disk matches the in-code projection
//!   bit-for-bit. Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_service_health_continuity_corpus -- emit-fixtures \
//!     fixtures/ops/m3/service_health_continuity
//!   ```
//!
//! - The pinned `overall_contract_state`, `overall_local_continuity`, and
//!   `honesty_marker_present` per drill match the surface's render.
//! - Stale-status bans hold: a card whose state is `stale` (or whose age
//!   bucket is `stale` / `very_stale` / `never_checked`) lights the
//!   honesty marker — it cannot masquerade as current ready truth.
//! - Hosted-only and vendor-provider outages do not drag overall local
//!   continuity below `local_safe`.
//! - Data-plane outages drag overall continuity at least to
//!   `local_safe_read_only` so external writes are visibly paused.
//! - Mirror-only fallback drills carry the `fallback_mode:mirror_only`
//!   detail token and surface `local_only` contract state with
//!   `local_safe` continuity.
//! - Recovery drills replace cached "down" state with a fresh probe and
//!   stay honest about probes that have not yet returned post-restart.
//! - Every card's `contract_state_token` and `affected_workflow_tokens`
//!   resolve to the closed vocabulary. No surface-local "service down" /
//!   "service degraded" / "broken" / "error occurred" copy is admitted.

use aureline_shell::service_health::aggregator::{
    LastCheckedAgeClass, LocalContinuityClass, ServiceContractStateClass, ServiceHealthAggregator,
};
use aureline_shell::service_health::continuity_corpus::{continuity_corpus, ContinuityScenario};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ops/m3/service_health_continuity",
);

fn load_aggregator(filename: &str) -> ServiceHealthAggregator {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

fn find_scenario(drill_id: &str) -> ContinuityScenario {
    continuity_corpus()
        .into_iter()
        .find(|s| s.drill_id == drill_id)
        .unwrap_or_else(|| panic!("missing continuity scenario {drill_id}"))
}

#[test]
fn every_continuity_drill_fixture_matches_in_code_projection() {
    for scenario in continuity_corpus() {
        let on_disk = load_aggregator(scenario.fixture_filename);
        let in_code = scenario.aggregator();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_service_health_continuity_corpus -- emit-fixtures fixtures/ops/m3/service_health_continuity`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn every_drill_renders_the_pinned_overall_state_and_continuity() {
    for scenario in continuity_corpus() {
        let agg = scenario.aggregator();
        assert_eq!(
            agg.overall_contract_state, scenario.expected_overall_contract_state,
            "{} overall_contract_state drift",
            scenario.drill_id,
        );
        assert_eq!(
            agg.overall_local_continuity, scenario.expected_overall_local_continuity,
            "{} overall_local_continuity drift",
            scenario.drill_id,
        );
        assert_eq!(
            agg.honesty_marker_present, scenario.expected_honesty_marker_present,
            "{} honesty_marker_present drift",
            scenario.drill_id,
        );
    }
}

#[test]
fn no_card_can_render_an_open_ended_contract_state_token() {
    for scenario in continuity_corpus() {
        let agg = scenario.aggregator();
        for card in &agg.cards {
            assert!(
                ServiceContractStateClass::from_token(&card.contract_state_token).is_some(),
                "{} card {} carries contract_state_token {} outside the closed vocabulary",
                scenario.drill_id,
                card.card_id,
                card.contract_state_token,
            );
        }
    }
}

#[test]
fn no_card_leaks_generic_outage_copy() {
    // These phrases describe outages in terms that collapse the closed
    // contract-state vocabulary into generic copy. The corpus bans them so
    // chrome that hand-rolls outage strings fails the replay.
    const FORBIDDEN: &[&str] = &[
        "service down",
        "service is down",
        "service degraded",
        "broken",
        "error occurred",
        "we hit an error",
        "something went wrong",
    ];
    for scenario in continuity_corpus() {
        let agg = scenario.aggregator();
        for card in &agg.cards {
            let lower = card.state_explanation.to_lowercase();
            for phrase in FORBIDDEN {
                assert!(
                    !lower.contains(phrase),
                    "{} card {} state_explanation contains banned phrase '{}': {}",
                    scenario.drill_id,
                    card.card_id,
                    phrase,
                    card.state_explanation,
                );
            }
        }
    }
}

#[test]
fn stale_status_cannot_masquerade_as_current_online_truth() {
    for scenario in continuity_corpus() {
        let agg = scenario.aggregator();
        for card in &agg.cards {
            // A `stale` contract state MUST report an aged or absent probe.
            if card.contract_state == ServiceContractStateClass::Stale {
                assert!(
                    matches!(
                        card.last_checked_age,
                        LastCheckedAgeClass::Stale
                            | LastCheckedAgeClass::VeryStale
                            | LastCheckedAgeClass::NeverChecked
                    ),
                    "{} card {} contract_state=stale but last_checked_age={:?}",
                    scenario.drill_id,
                    card.card_id,
                    card.last_checked_age,
                );
            }
            // An aged or never-checked probe MUST light the honesty marker so
            // the chrome cannot paint the card as "current online truth".
            if matches!(
                card.last_checked_age,
                LastCheckedAgeClass::Stale
                    | LastCheckedAgeClass::VeryStale
                    | LastCheckedAgeClass::NeverChecked
            ) {
                assert!(
                    card.honesty_marker_present,
                    "{} card {} has aged probe ({:?}) but does not light the honesty marker",
                    scenario.drill_id, card.card_id, card.last_checked_age,
                );
            }
        }
    }
}

#[test]
fn single_service_hosted_outage_does_not_downgrade_overall_continuity() {
    let scenario = find_scenario("single_service_outage");
    let agg = scenario.aggregator();
    assert_eq!(
        agg.overall_local_continuity,
        LocalContinuityClass::LocalSafe,
        "hosted-only outage must not downgrade overall local continuity",
    );
    // The hosted marketplace card MUST be visibly unavailable.
    let marketplace = agg
        .cards
        .iter()
        .find(|c| c.card_id == "card:marketplace")
        .expect("single-service drill must include the marketplace card");
    assert_eq!(
        marketplace.contract_state,
        ServiceContractStateClass::Unavailable
    );
    assert!(marketplace.honesty_marker_present);
}

#[test]
fn control_plane_drill_keeps_data_plane_local_safe() {
    let agg = find_scenario("control_plane_unavailable").aggregator();
    assert_eq!(
        agg.overall_local_continuity,
        LocalContinuityClass::LocalSafe,
        "control-plane impairment must not drag overall continuity below local_safe",
    );
    // Data-plane families remain ready in the same render.
    for card_id in [
        "card:language_services",
        "card:ai_assist",
        "card:workspace_sync",
    ] {
        let card = agg
            .cards
            .iter()
            .find(|c| c.card_id == card_id)
            .unwrap_or_else(|| panic!("control-plane drill must include {card_id}"));
        assert_eq!(
            card.contract_state,
            ServiceContractStateClass::Ready,
            "{} should be ready in a control-plane drill",
            card_id,
        );
    }
}

#[test]
fn data_plane_drill_downgrades_overall_continuity_to_read_only() {
    let agg = find_scenario("data_plane_unavailable").aggregator();
    assert!(
        agg.overall_local_continuity <= LocalContinuityClass::LocalSafeReadOnly,
        "data-plane outage must downgrade overall continuity to at most local_safe_read_only",
    );
    // Workspace sync MUST surface the local_only state.
    let sync = agg
        .cards
        .iter()
        .find(|c| c.card_id == "card:workspace_sync")
        .expect("data-plane drill must include the workspace_sync card");
    assert_eq!(sync.contract_state, ServiceContractStateClass::LocalOnly);
    assert_eq!(
        sync.local_continuity,
        LocalContinuityClass::LocalSafeReadOnly
    );
}

#[test]
fn mirror_only_fallback_drill_carries_fallback_token_and_keeps_local_safe() {
    let agg = find_scenario("mirror_only_fallback").aggregator();
    assert_eq!(
        agg.overall_local_continuity,
        LocalContinuityClass::LocalSafe,
        "mirror fallback must keep overall continuity local_safe",
    );
    for card_id in ["card:marketplace", "card:docs_knowledge"] {
        let card = agg
            .cards
            .iter()
            .find(|c| c.card_id == card_id)
            .unwrap_or_else(|| panic!("mirror drill must include {card_id}"));
        assert_eq!(
            card.contract_state,
            ServiceContractStateClass::LocalOnly,
            "{} should sit in local_only on a mirror fallback",
            card_id,
        );
        assert!(
            card.detail_tokens
                .iter()
                .any(|t| t == "fallback_mode:mirror_only"),
            "{} should carry the fallback_mode:mirror_only detail token",
            card_id,
        );
    }
}

#[test]
fn contract_mismatch_drill_is_distinct_from_generic_degraded_state() {
    let agg = find_scenario("contract_mismatch").aggregator();
    assert_eq!(
        agg.overall_contract_state,
        ServiceContractStateClass::ContractMismatch,
        "contract_mismatch must not collapse into degraded",
    );
    let release = agg
        .cards
        .iter()
        .find(|c| c.card_id == "card:release_channel")
        .expect("contract_mismatch drill must include the release_channel card");
    assert_eq!(
        release.contract_state,
        ServiceContractStateClass::ContractMismatch
    );
    assert!(release
        .detail_tokens
        .iter()
        .any(|t| t == "schema_skew:claim_manifest"));
}

#[test]
fn policy_block_drill_keeps_local_work_safe_and_uses_policy_blocked_token() {
    let agg = find_scenario("policy_block").aggregator();
    assert_eq!(
        agg.overall_contract_state,
        ServiceContractStateClass::PolicyBlocked,
        "policy_blocked must not collapse into unavailable",
    );
    assert_eq!(
        agg.overall_local_continuity,
        LocalContinuityClass::LocalSafe,
    );
    for card_id in ["card:telemetry", "card:ai_assist"] {
        let card = agg
            .cards
            .iter()
            .find(|c| c.card_id == card_id)
            .unwrap_or_else(|| panic!("policy drill must include {card_id}"));
        assert_eq!(
            card.contract_state,
            ServiceContractStateClass::PolicyBlocked
        );
    }
}

#[test]
fn auth_loss_drill_cascades_into_read_only_continuity() {
    let agg = find_scenario("auth_loss").aggregator();
    assert_eq!(
        agg.overall_local_continuity,
        LocalContinuityClass::LocalSafeReadOnly,
        "auth loss must cascade into read-only overall continuity",
    );
    let license = agg
        .cards
        .iter()
        .find(|c| c.card_id == "card:license_entitlement")
        .expect("auth_loss drill must include the license_entitlement card");
    assert_eq!(
        license.contract_state,
        ServiceContractStateClass::Unavailable
    );
    let sync = agg
        .cards
        .iter()
        .find(|c| c.card_id == "card:workspace_sync")
        .expect("auth_loss drill must include the workspace_sync card");
    assert_eq!(sync.contract_state, ServiceContractStateClass::LocalOnly);
}

#[test]
fn recovery_drill_replaces_pre_restart_cache_with_fresh_probes() {
    let agg = find_scenario("recovery_after_restart").aggregator();
    let sync = agg
        .cards
        .iter()
        .find(|c| c.card_id == "card:workspace_sync")
        .expect("recovery drill must include the workspace_sync card");
    assert_eq!(
        sync.contract_state,
        ServiceContractStateClass::Ready,
        "post-restart sync must read ready, not the pre-restart cached status",
    );
    assert_eq!(sync.last_checked_age, LastCheckedAgeClass::Fresh);
    let ai = agg
        .cards
        .iter()
        .find(|c| c.card_id == "card:ai_assist")
        .expect("recovery drill must include the ai_assist card");
    assert_eq!(ai.last_checked_age, LastCheckedAgeClass::NeverChecked);
    assert!(
        ai.honesty_marker_present,
        "a never-checked probe must light the honesty marker; the chrome cannot reuse the pre-restart status",
    );
}

#[test]
fn render_plaintext_quotes_each_drill_card() {
    // Surfaces (CLI/headless inspect, support export) render the same
    // plaintext block; this guards that the per-drill plaintext mentions
    // every focused card so support export readers see all named families.
    for scenario in continuity_corpus() {
        let plaintext = scenario.aggregator().render_plaintext();
        for card_id in scenario.focused_card_ids {
            assert!(
                plaintext.contains(card_id),
                "{} render_plaintext must mention card {}",
                scenario.drill_id,
                card_id,
            );
        }
    }
}
