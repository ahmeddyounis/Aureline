//! Fixture-replay and invariant tests for the stable interaction-parity corpus.
//!
//! The records live under
//! `fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state/`
//! and are minted by the `aureline_shell_interaction_parity_stable` emitter so
//! the checked-in JSON stays a literal projection of the in-code corpus, which
//! is itself a projection of the live interaction-integrity packet.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_interaction_parity_stable -- emit-fixtures \
//!     fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state
//!   ```
//!
//! - All five dense-surface families (tree, virtualized list, grid, palette-like,
//!   inspector/detail) are covered.
//! - Focus, current item, selection, anchor, and activation are modeled as
//!   distinct stable-id states on Stable rows; the collapse drill narrows.
//! - Asynchronous updates preserve focus and selection by stable id and never
//!   steal focus from the active task; the focus-theft drill narrows.
//! - Every required focus-return drill is present and returns to a safe target;
//!   the drop-to-body drill narrows.
//! - The keyboard model is single-tab-stop / roving-tabindex with the full key
//!   bar and no silent destructive activation on Stable rows.
//! - Selected-count, position-in-set, and blocked/read-only cues hold, and the
//!   accessibility block holds across normal / high-contrast / zoomed layouts.
//! - Per-OS conformance covers macOS, Windows, and Linux with current proof.
//! - The shell collection surface, keyboard help, CLI inspect, Help/About, and
//!   support export all bind the shared record.
//! - The matrix spans Stable and narrowed rows; a posture that cannot prove a
//!   pillar or sits on a below-Stable binding surface is narrowed with a named
//!   reason.
//! - The same posture opens from the activity center, command palette, status
//!   bar, and a menu command, keyboard-first, across normal / high-contrast /
//!   zoomed layouts, without an account or managed services.

use aureline_shell::interaction_integrity_stable::{
    interaction_parity_corpus, AsyncUpdateClass, DisappearanceResolution, FocusReturnTrigger,
    InteractionParityRecord, InteractionRecoveryAction, InteractionSurfaceClass,
    InteractionTruthSurface, PlatformProfileClass,
};
use aureline_shell::notification_attention_stable::{
    AttentionRouteSurface, LayoutMode, StableClaimClass,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state",
);

fn load_record(filename: &str) -> InteractionParityRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in interaction_parity_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_interaction_parity_stable -- emit-fixtures fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.posture_id, scenario.expected_posture,
            "{} posture",
            scenario.scenario_id
        );
        assert_eq!(
            record.surface_class, scenario.expected_surface_class,
            "{} surface class",
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
fn matrix_covers_every_surface_family() {
    let corpus = interaction_parity_corpus();
    for required in InteractionSurfaceClass::REQUIRED {
        assert!(
            corpus
                .iter()
                .any(|scenario| scenario.expected_surface_class == required),
            "matrix must cover surface family {}",
            required.as_str(),
        );
    }
}

#[test]
fn coordination_states_distinct_on_stable() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.coordination.states_modeled_distinctly
                    && record.coordination.activation_preserves_selection
                    && record.coordination.identity_by_stable_id_not_index,
                "{} Stable but coordination states not distinct",
                scenario.scenario_id,
            );
            assert!(
                record.pillars.coordination_states_distinct,
                "{} Stable but coordination pillar false",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn collapse_drill_actually_collapses() {
    let record = load_record("coordination_collapse_drill.json");
    assert!(!record.coordination.states_modeled_distinctly);
    assert!(!record.pillars.coordination_states_distinct);
    assert!(record
        .stable_qualification
        .narrowing_reasons
        .iter()
        .any(|r| r.as_str() == "coordination_states_collapsed"));
}

#[test]
fn async_updates_complete_and_never_steal_on_stable() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in AsyncUpdateClass::REQUIRED {
            assert!(
                record
                    .async_updates
                    .iter()
                    .any(|row| row.update_class == required),
                "{} missing async-update class {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        for row in &record.async_updates {
            // No row may resolve a disappearing focus to the document body.
            if row.focused_object_can_disappear {
                assert!(
                    matches!(
                        row.disappearance_resolution,
                        DisappearanceResolution::NearestSafeSibling
                            | DisappearanceResolution::ParentNode
                    ) && row.announces_focus_move_reason,
                    "{} async row {} drops a disappearing focus unsafely",
                    scenario.scenario_id,
                    row.update_class.as_str(),
                );
            }
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            for row in &record.async_updates {
                assert!(
                    !row.steals_focus_from_active_task,
                    "{} Stable but async row {} steals focus",
                    scenario.scenario_id,
                    row.update_class.as_str(),
                );
                assert!(
                    row.preserves_focus_by_stable_id
                        && row.preserves_selection_by_stable_id
                        && row.preserves_anchor,
                    "{} Stable but async row {} loses identity",
                    scenario.scenario_id,
                    row.update_class.as_str(),
                );
            }
            assert!(
                record.pillars.async_never_steals_focus
                    && record.pillars.identity_survives_async_updates,
                "{} Stable but async pillars false",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn focus_theft_drill_actually_steals() {
    let record = load_record("async_update_focus_theft_drill.json");
    assert!(record
        .async_updates
        .iter()
        .any(|row| row.steals_focus_from_active_task));
    assert!(!record.pillars.async_never_steals_focus);
    assert!(record
        .stable_qualification
        .narrowing_reasons
        .iter()
        .any(|r| r.as_str() == "async_update_steals_focus"));
}

#[test]
fn focus_return_drills_complete_and_safe_on_stable() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in FocusReturnTrigger::REQUIRED {
            assert!(
                record
                    .focus_returns
                    .iter()
                    .any(|row| row.trigger == required),
                "{} missing focus-return trigger {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            for row in &record.focus_returns {
                assert!(
                    row.returns_to_invoker_or_safe_ancestor
                        && row.never_returns_to_document_body
                        && row.never_returns_to_offscreen_surface
                        && row.never_warps_across_windows
                        && row.preserves_selection_or_cursor_state,
                    "{} Stable but focus-return {} is unsafe",
                    scenario.scenario_id,
                    row.trigger.as_str(),
                );
            }
            assert!(
                record.pillars.focus_return_complete,
                "{} Stable but focus-return pillar false",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn drop_to_body_drill_actually_drops() {
    let record = load_record("focus_return_drop_to_body_drill.json");
    assert!(record
        .focus_returns
        .iter()
        .any(|row| !row.never_returns_to_document_body));
    assert!(!record.pillars.focus_return_complete);
    assert!(record
        .stable_qualification
        .narrowing_reasons
        .iter()
        .any(|r| r.as_str() == "focus_return_incomplete"));
}

#[test]
fn keyboard_model_complete_on_stable() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            let km = &record.keyboard_model;
            let selection_ok = !km.selection_supported || km.space_toggles_selection;
            assert!(
                km.single_tab_stop
                    && km.arrow_moves_current_item
                    && selection_ok
                    && km.enter_triggers_default_action
                    && km.default_action_discoverable
                    && km.home_end_page_preserves_anchor
                    && km.no_silent_destructive_activation,
                "{} Stable but keyboard model incomplete",
                scenario.scenario_id,
            );
            assert!(
                record.pillars.keyboard_model_complete,
                "{} Stable but keyboard pillar false",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn accessibility_cues_complete_on_stable() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            let cues = &record.a11y_cues;
            assert!(
                cues.selected_count_narrated
                    && cues.position_in_set_narrated
                    && cues.blocked_row_cue_present
                    && cues.read_only_row_cue_present
                    && cues.roving_tabindex_narrated,
                "{} Stable but accessibility cues incomplete",
                scenario.scenario_id,
            );
            assert!(
                record.pillars.accessibility_cues_complete,
                "{} Stable but a11y pillar false",
                scenario.scenario_id,
            );
        }
        // Layout-mode parity holds for every row.
        for required in LayoutMode::REQUIRED {
            let disclosure = record
                .accessibility
                .layout_modes
                .iter()
                .find(|m| m.mode == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing layout mode {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                disclosure.row_narration_available && disclosure.recovery_affordances_reachable,
                "{} layout mode {} unreachable",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        // Accessibility action labels mirror the recovery routes.
        assert_eq!(
            record.accessibility.action_labels.len(),
            record.recovery_routes.len(),
            "{} action-label / recovery-route count drift",
            scenario.scenario_id,
        );
    }
}

#[test]
fn per_os_conformance_is_complete_with_proof() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in PlatformProfileClass::REQUIRED {
            let row = record
                .platform_conformance
                .iter()
                .find(|row| row.profile == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing per-OS profile {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                row.covered && !row.proof_ref.trim().is_empty(),
                "{} profile {} lacks current proof",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                !row.named_behaviors.is_empty(),
                "{} profile {} names no behavior",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn surfaces_bind_the_shared_record() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in InteractionTruthSurface::REQUIRED {
            let projection = record
                .surface_projections
                .iter()
                .find(|p| p.surface == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing binding surface {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                projection.reads_shared_record,
                "{} surface {} clones prose",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                !projection.summary_line.trim().is_empty(),
                "{} surface {} empty summary",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn entry_routes_reach_the_same_posture_keyboard_first() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in AttentionRouteSurface::REQUIRED {
            let route = record
                .routes
                .iter()
                .find(|r| r.surface == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing entry route {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                route.keyboard_reachable && route.activates_same_item,
                "{} route {} not keyboard-first or targets a different item",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        for required in InteractionRecoveryAction::REQUIRED {
            assert!(
                record
                    .recovery_routes
                    .iter()
                    .any(|r| r.action_id == required.as_str()),
                "{} missing recovery action {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        assert!(
            record.available_without_account && record.available_without_managed_services,
            "{} not available without an account or managed services",
            scenario.scenario_id,
        );
    }
}

#[test]
fn matrix_spans_stable_and_narrowed_rows() {
    let corpus = interaction_parity_corpus();
    let stable = corpus
        .iter()
        .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
        .count();
    let narrowed = corpus.len() - stable;
    assert!(stable >= 1, "matrix must include a Stable row");
    assert!(narrowed >= 1, "matrix must include a narrowed row");
}

#[test]
fn narrowed_rows_drop_below_cutline_and_name_a_reason() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let qualification = &record.stable_qualification;
        if qualification.claim_class == StableClaimClass::Stable {
            assert!(
                qualification.qualifies_stable && qualification.narrowing_reasons.is_empty(),
                "{} Stable row carries a narrowing reason",
                scenario.scenario_id,
            );
        } else {
            assert!(
                !qualification.qualifies_stable,
                "{} narrowed row still qualifies",
                scenario.scenario_id
            );
            assert!(
                !qualification.claim_class.at_or_above_cutline(),
                "{} narrowed claim sits at or above the cutline",
                scenario.scenario_id,
            );
            assert!(
                !qualification.narrowing_reasons.is_empty(),
                "{} narrowed without a named reason",
                scenario.scenario_id,
            );
            assert!(
                record.honesty_marker_present,
                "{} hides the honesty marker",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn claim_ceiling_never_overclaims() {
    for scenario in interaction_parity_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        let pillars = record.pillars;
        if ceiling.asserts_coordination_states_distinct {
            assert!(
                pillars.coordination_states_distinct,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_identity_survives_async_updates {
            assert!(
                pillars.identity_survives_async_updates,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_focus_return_complete {
            assert!(pillars.focus_return_complete, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_keyboard_model_complete {
            assert!(pillars.keyboard_model_complete, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_async_never_steals_focus {
            assert!(pillars.async_never_steals_focus, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_accessibility_cues_complete {
            assert!(
                pillars.accessibility_cues_complete,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_platform_conformance_complete {
            assert!(
                pillars.platform_conformance_complete,
                "{}",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn preview_surface_inspector_narrows_to_preview() {
    let record = load_record("preview_surface_inspector.json");
    assert_eq!(
        record.stable_qualification.claim_class,
        StableClaimClass::Preview
    );
    // Every pillar holds; the only narrowing is the below-Stable binding surface.
    assert!(record.pillars.coordination_states_distinct);
    assert!(record.pillars.keyboard_model_complete);
    assert_eq!(
        record.stable_qualification.narrowing_reasons.len(),
        1,
        "preview row should narrow only on the surface marker",
    );
    assert_eq!(
        record.stable_qualification.narrowing_reasons[0].as_str(),
        "surface_not_yet_stable"
    );
}
