//! Fixture-replay and invariant tests for the stable component-state registry
//! certification corpus.
//!
//! The records live under
//! `fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity/` and
//! are minted by the `aureline_settings_component_state_registry_stable` emitter
//! so the checked-in JSON stays a literal projection of the in-code corpus,
//! which is itself a projection of the live design-system registry, the live
//! extension appearance-conformance packet, and the live screenshot-diff packet.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-settings \
//!     --bin aureline_settings_component_state_registry_stable -- emit-fixtures \
//!     fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity
//!   ```
//!
//! - Every required component family is present, token-driven, and the union of
//!   supported states covers the shared vocabulary.
//! - Disabled, blocked, policy-locked, reconnecting, warming, partial, stale, and
//!   recovering states stay consistent across shell, review, settings, and
//!   support, each carrying a non-color cue and never hue or animation alone.
//! - Every extension appearance axis inherits fully or discloses its gap in
//!   review, diagnostics, and support; an undisclosed gap is refused and narrowed.
//! - Shell-zoning metrics and placeholders are token-driven.
//! - Every launch-critical surface/state permutation has a conforming fixture
//!   with focus visibility and screen-reader semantics preserved.
//! - No posture over-claims; a posture that cannot prove a pillar or sits on a
//!   below-Stable family is narrowed below Stable with a named reason.
//! - The same record opens from the settings/design-system panel, command
//!   palette, status bar, and a menu command, keyboard-first, across normal /
//!   high-contrast / zoomed layouts, without an account or managed services.

use std::collections::BTreeSet;

use aureline_extensions::appearance_conformance::{AppearanceSupportClass, APPEARANCE_AXES};
use aureline_settings::component_state_registry_stable::{
    component_state_registry_corpus, is_canonical_object_ref, CanonicalStateClass,
    ComponentFamilyClass, ComponentStateName, ComponentStateRegistryCertification,
    LaunchSurfaceClass, LayoutMode, RegistrySurfaceClass, RouteSurface, StableClaimClass,
    COMPONENT_STATE_REGISTRY_RECORD_KIND, COMPONENT_STATE_REGISTRY_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity",
);

fn load_record(filename: &str) -> ComponentStateRegistryCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in component_state_registry_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-settings --bin aureline_settings_component_state_registry_stable -- emit-fixtures fixtures/ux/m4/harden-component-state-registry-and-theme-token-parity`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in component_state_registry_corpus() {
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
    for scenario in component_state_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, COMPONENT_STATE_REGISTRY_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            COMPONENT_STATE_REGISTRY_SHARED_CONTRACT_REF
        );
        assert!(is_canonical_object_ref(&record.diagnostics_export_ref));
        assert!(is_canonical_object_ref(&record.support_export_ref));
        assert!(is_canonical_object_ref(&record.registry_binding.value_ref));
    }
}

#[test]
fn every_family_is_present_and_covers_the_vocabulary() {
    for scenario in component_state_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<ComponentFamilyClass> = record
            .component_families
            .iter()
            .map(|row| row.family_class)
            .collect();
        for required in ComponentFamilyClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing family {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        let mut covered: BTreeSet<ComponentStateName> = BTreeSet::new();
        for row in &record.component_families {
            for state in &row.supported_states {
                covered.insert(*state);
            }
        }
        for required in ComponentStateName::REQUIRED {
            assert!(
                covered.contains(&required),
                "{} vocabulary does not cover {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn normalized_states_are_consistent_and_never_hue_or_animation_only() {
    for scenario in component_state_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<ComponentStateName> = record
            .normalized_states
            .iter()
            .map(|row| row.state_name)
            .collect();
        for required in ComponentStateName::NORMALIZED_REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing normalized state {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.normalized_states {
            assert_eq!(
                row.taxonomy_ref,
                row.state_name.taxonomy(),
                "{} normalized state {} must bind to the shared taxonomy",
                scenario.scenario_id,
                row.state_name.as_str()
            );
            assert!(
                !row.non_color_cues.is_empty()
                    && row.hue_only_forbidden
                    && row.animation_only_forbidden,
                "{} normalized state {} must carry a non-color cue",
                scenario.scenario_id,
                row.state_name.as_str()
            );
            for surface in RegistrySurfaceClass::REQUIRED {
                assert!(
                    row.consistent_across_surfaces.contains(&surface),
                    "{} normalized state {} must be consistent on {}",
                    scenario.scenario_id,
                    row.state_name.as_str(),
                    surface.as_str()
                );
            }
            assert!(row.conforms);
        }
    }
}

#[test]
fn extension_inheritance_covers_every_axis_and_discloses_gaps() {
    for scenario in component_state_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<_> = record
            .extension_inheritance
            .iter()
            .map(|row| row.axis)
            .collect();
        for required in APPEARANCE_AXES {
            assert!(
                present.contains(&required),
                "{} missing extension axis {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.extension_inheritance {
            // A conforming axis either inherits fully or discloses its gap in
            // review, diagnostics, and support export.
            if row.conforms {
                let inherits_fully =
                    matches!(row.support_class, AppearanceSupportClass::FullInheritance);
                assert!(
                    inherits_fully
                        || (row.gap_disclosed_in_review
                            && row.gap_surfaced_in_diagnostics
                            && row.gap_surfaced_in_support_export),
                    "{} axis {} claims conformance without disclosing its gap",
                    scenario.scenario_id,
                    row.axis.as_str()
                );
            }
        }
    }
}

#[test]
fn shell_zoning_metrics_are_token_driven_or_waived() {
    for scenario in component_state_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for row in &record.shell_zones {
            assert!(
                row.min_chrome_px <= row.max_chrome_px,
                "{} zone {} has inverted chrome metrics",
                scenario.scenario_id,
                row.zone_class.as_str()
            );
            if !row.conforms {
                assert!(
                    row.waiver_ref.is_some(),
                    "{} non-conforming zone {} must carry a waiver",
                    scenario.scenario_id,
                    row.zone_class.as_str()
                );
            }
        }
    }
}

#[test]
fn state_fixtures_cover_every_launch_permutation() {
    for scenario in component_state_registry_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let surfaces: BTreeSet<LaunchSurfaceClass> = record
            .state_fixtures
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for required in LaunchSurfaceClass::required() {
            assert!(
                surfaces.contains(required),
                "{} missing fixture surface {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        let states: BTreeSet<CanonicalStateClass> = record
            .state_fixtures
            .iter()
            .map(|row| row.state_class)
            .collect();
        for required in CanonicalStateClass::required() {
            assert!(
                states.contains(required),
                "{} missing fixture state {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.state_fixtures {
            assert!(is_canonical_object_ref(&row.screenshot_ref));
            assert!(is_canonical_object_ref(&row.fixture_ref));
            assert!(
                row.focus_visible_preserved && row.screen_reader_semantics_preserved,
                "{} fixture {}/{} regressed focus or screen-reader semantics",
                scenario.scenario_id,
                row.surface_class.as_str(),
                row.state_class.as_str()
            );
        }
        assert!(record.pillars.focus_and_screen_reader_preserved);
    }
}

#[test]
fn stable_postures_qualify_and_narrowed_postures_name_a_reason() {
    for scenario in component_state_registry_corpus() {
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
    let records: Vec<_> = component_state_registry_corpus()
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
    for scenario in component_state_registry_corpus() {
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
                    panic!(
                        "{} missing layout mode {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                disclosure.row_narration_available && disclosure.recovery_affordances_reachable
            );
        }
        assert!(record.available_without_account && record.available_without_managed_services);
    }
}
