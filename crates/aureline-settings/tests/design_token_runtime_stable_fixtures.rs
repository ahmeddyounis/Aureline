//! Fixture-replay and invariant tests for the stable design-token runtime
//! certification corpus.
//!
//! The records live under
//! `fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light/` and are
//! minted by the `aureline_settings_design_token_runtime_stable` emitter so the
//! checked-in JSON stays a literal projection of the in-code corpus, which is
//! itself a projection of the live appearance runtime.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-settings \
//!     --bin aureline_settings_design_token_runtime_stable -- emit-fixtures \
//!     fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light
//!   ```
//!
//! - Dark, light, high-contrast (dark and light), reduced-motion, and density
//!   rows are all exercised and conform on the Stable postures.
//! - Diagnostics, policy locks, trust warnings, execution targets, selection,
//!   and focus never rely on hue alone; each carries a non-color carrier.
//! - Golden captures and accessibility packets are attributable to one
//!   appearance-session value.
//! - Every appearance axis applies live or discloses a reload/restart — never a
//!   silent lag.
//! - Reduced-motion / power-saver suppression is modeled in the token runtime.
//! - No posture over-claims; a posture that cannot prove a pillar or sits on a
//!   below-Stable surface is narrowed below Stable with a named reason.
//! - The same record opens from the settings appearance panel, command palette,
//!   status bar, and a menu command, keyboard-first, across normal /
//!   high-contrast / zoomed layouts.
//! - Every record stays available without an account or managed services.

use std::collections::BTreeSet;

use aureline_settings::design_token_runtime_stable::{
    design_token_runtime_corpus, is_canonical_object_ref, AppearanceModeClass,
    DesignTokenRuntimeCertification, LayoutMode, LiveApplyClass, ProtectedCueClass, RouteSurface,
    StableClaimClass, DESIGN_TOKEN_RUNTIME_RECORD_KIND, DESIGN_TOKEN_RUNTIME_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light",
);

fn load_record(filename: &str) -> DesignTokenRuntimeCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in design_token_runtime_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-settings --bin aureline_settings_design_token_runtime_stable -- emit-fixtures fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.posture_id, scenario.expected_posture,
            "{} posture",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.claim_class, scenario.expected_claim_class,
            "{} claim_class",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.qualifies_stable, scenario.expected_qualifies_stable,
            "{} qualifies_stable",
            scenario.scenario_id
        );
        assert_eq!(
            record.surface_lifecycle_marker, scenario.expected_surface_marker,
            "{} surface marker",
            scenario.scenario_id
        );
    }
}

#[test]
fn record_identity_and_contract_are_stable() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, DESIGN_TOKEN_RUNTIME_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            DESIGN_TOKEN_RUNTIME_SHARED_CONTRACT_REF
        );
        assert!(is_canonical_object_ref(&record.diagnostics_export_ref));
        assert!(is_canonical_object_ref(&record.support_export_ref));
        assert!(is_canonical_object_ref(&record.appearance_session.value_ref));
    }
}

#[test]
fn every_required_appearance_mode_is_present_and_resolves() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<AppearanceModeClass> =
            record.mode_rows.iter().map(|row| row.mode_class).collect();
        for required in AppearanceModeClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing mode {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.mode_rows {
            assert!(
                row.token_registry_resolves,
                "{} mode {} token registry must resolve",
                scenario.scenario_id,
                row.mode_class.as_str()
            );
            assert!(
                !row.certified_token_refs.is_empty(),
                "{} mode {} must certify at least one token",
                scenario.scenario_id,
                row.mode_class.as_str()
            );
        }
    }
}

#[test]
fn captures_attribute_to_one_appearance_session() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let value_ref = &record.appearance_session.value_ref;
        for row in &record.mode_rows {
            assert_eq!(
                &row.appearance_session_value_ref, value_ref,
                "{} mode {} capture must cite the one appearance-session value",
                scenario.scenario_id,
                row.mode_class.as_str()
            );
            assert!(is_canonical_object_ref(&row.golden_capture_ref));
            assert!(is_canonical_object_ref(&row.accessibility_packet_ref));
        }
        assert!(record.pillars.captures_share_one_session);
    }
}

#[test]
fn protected_cues_never_rely_on_hue_alone() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<ProtectedCueClass> =
            record.protected_cues.iter().map(|row| row.cue_class).collect();
        for required in ProtectedCueClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing protected cue {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.protected_cues {
            assert!(
                !row.non_color_cues.is_empty() && row.hue_only_forbidden,
                "{} cue {} must carry a non-color cue",
                scenario.scenario_id,
                row.cue_class.as_str()
            );
        }
    }
}

#[test]
fn live_apply_axes_never_silently_lag() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.live_apply_axes.len(),
            7,
            "{} must cover every appearance axis",
            scenario.scenario_id
        );
        for row in &record.live_apply_axes {
            assert!(
                !row.silently_lags_system,
                "{} axis must not silently lag",
                scenario.scenario_id
            );
            if matches!(
                row.live_apply_class,
                LiveApplyClass::ReloadRequired | LiveApplyClass::RestartRequired
            ) {
                assert!(
                    row.disclosure_required,
                    "{} reload/restart axis must disclose",
                    scenario.scenario_id
                );
            }
        }
    }
}

#[test]
fn motion_suppression_is_modeled_in_the_runtime() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.motion_suppression.len(), 5);
        for row in &record.motion_suppression {
            assert!(row.modeled_in_token_runtime);
            assert!(row.per_surface_improvisation_absent);
        }
        assert!(record.pillars.motion_suppression_in_runtime);
    }
}

#[test]
fn stable_postures_qualify_and_narrowed_postures_name_a_reason() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.stable_qualification.qualifies_stable {
            assert_eq!(
                record.stable_qualification.claim_class,
                StableClaimClass::Stable
            );
            assert!(record.stable_qualification.narrowing_reasons.is_empty());
        } else {
            assert_ne!(
                record.stable_qualification.claim_class,
                StableClaimClass::Stable
            );
            assert!(
                !record.stable_qualification.narrowing_reasons.is_empty(),
                "{} narrowed posture must name a reason",
                scenario.scenario_id
            );
            assert!(record.honesty_marker_present);
        }
    }
}

#[test]
fn at_least_one_posture_qualifies_and_one_narrows() {
    let records: Vec<_> = design_token_runtime_corpus()
        .into_iter()
        .map(|scenario| scenario.record())
        .collect();
    assert!(
        records
            .iter()
            .any(|record| record.stable_qualification.qualifies_stable),
        "matrix must include a Stable posture"
    );
    assert!(
        records
            .iter()
            .any(|record| !record.stable_qualification.qualifies_stable),
        "matrix must exercise the narrowing path"
    );
}

#[test]
fn record_is_reachable_keyboard_first_across_layouts() {
    for scenario in design_token_runtime_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<RouteSurface> =
            record.routes.iter().map(|route| route.surface).collect();
        for required in RouteSurface::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing entry surface {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for route in &record.routes {
            assert!(route.keyboard_reachable && route.activates_same_record);
        }
        for required in LayoutMode::REQUIRED {
            let disclosure = record
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
                .unwrap_or_else(|| {
                    panic!("{} missing layout mode {}", scenario.scenario_id, required.as_str())
                });
            assert!(
                disclosure.row_narration_available && disclosure.recovery_affordances_reachable
            );
        }
        assert!(record.available_without_account && record.available_without_managed_services);
    }
}
