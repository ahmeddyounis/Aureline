//! Fixture-replay and invariant tests for the stable window-topology restore
//! corpus.
//!
//! The records live under
//! `fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed/` and are
//! minted by the `aureline_shell_window_topology_restore_stable` emitter so the
//! checked-in JSON stays a literal projection of the in-code corpus, which is
//! itself a projection of the live windows page and the restore-provenance
//! contract.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_window_topology_restore_stable -- emit-fixtures \
//!     fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed
//!   ```
//!
//! - Workspace authority (centralized) and window topology (window-local) are
//!   separated in the persisted model.
//! - The pane tree is versioned with stable pane IDs, and every leaf maps to
//!   exactly one slot.
//! - Restore is skeleton-first / hydrate-second: no session-scoped pane hydrates
//!   live, and every placeholder-backed pane forbids command rerun and authority
//!   reacquire.
//! - Restore fidelity and display-topology adjustments are carried in provenance
//!   so Exact / Compatible / Layout-only / placeholder-backed reopens are
//!   explainable without scraping localized UI copy.
//! - The matrix spans Stable and narrowed rows; a reopen that cannot prove a
//!   pillar or sits on a below-Stable binding surface is narrowed below Stable
//!   with a named reason.
//! - The same reopen opens from the activity center, command palette, status
//!   bar, and a menu command, keyboard-first, across normal / high-contrast /
//!   zoomed layouts.
//! - Every reopen stays available without an account or managed services.

use aureline_shell::notification_attention_stable::{
    is_canonical_object_ref, AttentionRouteSurface, LayoutMode, StableClaimClass,
};
use aureline_shell::window_topology_restore_stable::{
    window_topology_restore_corpus, PaneHydrationClass, RestoreFidelityClass, RestoreTruthSurface,
    WindowLocalTopologyClass, WindowRestoreRecoveryAction, WindowTopologyRestoreRecord,
    WorkspaceAuthorityClass, PANE_TREE_SCHEMA_VERSION,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed",
);

fn load_record(filename: &str) -> WindowTopologyRestoreRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in window_topology_restore_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_window_topology_restore_stable -- emit-fixtures fixtures/ux/m4/harden-multi-window-pane-detach-split-layout-mixed`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in window_topology_restore_corpus() {
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
        assert_eq!(
            record.restore_provenance.resulting_fidelity, scenario.expected_fidelity,
            "{} fidelity",
            scenario.scenario_id
        );
    }
}

#[test]
fn authority_and_topology_are_separated_in_the_model() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        // Every required authority and topology class is enumerated.
        for class in WorkspaceAuthorityClass::REQUIRED {
            assert!(
                record
                    .authority
                    .workspace_authority_classes
                    .contains(&class),
                "{} missing authority class {}",
                scenario.scenario_id,
                class.as_str(),
            );
        }
        for class in WindowLocalTopologyClass::REQUIRED {
            assert!(
                record
                    .authority
                    .window_local_topology_classes
                    .contains(&class),
                "{} missing topology class {}",
                scenario.scenario_id,
                class.as_str(),
            );
        }
        // A Stable reopen proves the separation pillar; a narrowed one names it.
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.pillars.authority_topology_separated,
                "{} Stable but not separated",
                scenario.scenario_id
            );
            assert!(record.authority.workspace_authority_centralized);
            assert!(record.authority.topology_window_local);
        }
    }
}

#[test]
fn pane_tree_is_versioned_with_stable_ids() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let tree = &record.pane_tree;
        assert!(
            tree.pane_tree_schema_version >= 1,
            "{} pane tree not versioned",
            scenario.scenario_id
        );
        // Every leaf maps to exactly one slot, and every slot is in the tree.
        let leaf_ids: std::collections::BTreeSet<String> =
            tree.root.leaf_ids().into_iter().collect();
        let slot_ids: std::collections::BTreeSet<String> =
            tree.slots.iter().map(|s| s.pane_id.clone()).collect();
        assert_eq!(
            leaf_ids, slot_ids,
            "{} pane tree leaves and slots disagree",
            scenario.scenario_id
        );
        assert_eq!(
            tree.slots.len(),
            slot_ids.len(),
            "{} duplicate pane ids",
            scenario.scenario_id
        );
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert_eq!(
                tree.pane_tree_schema_version, PANE_TREE_SCHEMA_VERSION,
                "{} Stable but pane tree schema not current",
                scenario.scenario_id,
            );
            assert!(record.pillars.pane_tree_versioned_stable_ids);
        }
    }
}

#[test]
fn restore_is_skeleton_first_with_no_silent_rerun() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for slot in &record.pane_tree.slots {
            // No session-scoped pane silently reacquires authority on restore.
            if slot.surface_class.is_session_scoped() {
                assert_ne!(
                    slot.hydration,
                    PaneHydrationClass::HydratedLive,
                    "{} session pane {} hydrated live",
                    scenario.scenario_id,
                    slot.pane_id,
                );
            }
            // Every placeholder-backed pane keeps the slot and forbids rerun.
            if slot.is_placeholder() {
                assert!(
                    slot.placeholder_state.is_some(),
                    "{} placeholder {} has no state",
                    scenario.scenario_id,
                    slot.pane_id,
                );
                assert!(
                    slot.command_rerun_forbidden && slot.authority_reacquire_forbidden,
                    "{} placeholder {} allows rerun/reacquire",
                    scenario.scenario_id,
                    slot.pane_id,
                );
                assert!(
                    !slot.runtime_survived,
                    "{} placeholder {} claims runtime survived",
                    scenario.scenario_id, slot.pane_id,
                );
                assert!(
                    !slot.recovery_actions.is_empty(),
                    "{} placeholder {} has no recovery actions",
                    scenario.scenario_id,
                    slot.pane_id,
                );
            }
        }
        assert!(
            record.pillars.skeleton_first_hydrate_second,
            "{} not skeleton-first",
            scenario.scenario_id
        );
        assert!(
            record.pillars.no_silent_rerun_or_reacquire,
            "{} permits silent rerun",
            scenario.scenario_id
        );
    }
}

#[test]
fn restore_fidelity_and_topology_provenance_is_consistent() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let provenance = &record.restore_provenance;
        match provenance.resulting_fidelity {
            RestoreFidelityClass::Exact => {
                assert!(
                    provenance.downgrade.is_none(),
                    "{} exact carries a downgrade",
                    scenario.scenario_id
                );
                assert!(
                    record.display_topology.topology_change_classes.is_empty()
                        && record.display_topology.adjustments.is_empty(),
                    "{} exact carries adjustments",
                    scenario.scenario_id,
                );
                assert_eq!(
                    record.placeholder_pane_count(),
                    0,
                    "{} exact carries placeholders",
                    scenario.scenario_id
                );
            }
            other => {
                let downgrade = provenance.downgrade.as_ref().unwrap_or_else(|| {
                    panic!("{} non-exact without downgrade", scenario.scenario_id)
                });
                assert_eq!(
                    downgrade.to_fidelity, other,
                    "{} downgrade target mismatch",
                    scenario.scenario_id
                );
                assert!(
                    is_canonical_object_ref(provenance.compare_ref.as_deref().unwrap_or("")),
                    "{} non-exact missing compare ref",
                    scenario.scenario_id
                );
                assert!(
                    is_canonical_object_ref(provenance.export_ref.as_deref().unwrap_or("")),
                    "{} non-exact missing export ref",
                    scenario.scenario_id
                );
            }
        }
        // A reopen with placeholders must be layout-only or placeholder-backed.
        if record.placeholder_pane_count() > 0 {
            assert!(
                matches!(
                    provenance.resulting_fidelity,
                    RestoreFidelityClass::LayoutOnly | RestoreFidelityClass::PlaceholderBacked
                ),
                "{} placeholder reopen claims too-high fidelity",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn surfaces_bind_the_shared_record() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in RestoreTruthSurface::REQUIRED {
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
fn matrix_spans_stable_and_narrowed_rows() {
    let corpus = window_topology_restore_corpus();
    let stable = corpus
        .iter()
        .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
        .count();
    let narrowed = corpus.len() - stable;
    assert!(stable >= 1, "matrix must include a Stable row");
    assert!(narrowed >= 1, "matrix must include a narrowed row");
    // Every restore fidelity class is exercised somewhere in the matrix.
    let mut seen_fidelity = std::collections::BTreeSet::new();
    for scenario in &corpus {
        seen_fidelity.insert(scenario.expected_fidelity);
    }
    for required in [
        RestoreFidelityClass::Exact,
        RestoreFidelityClass::Compatible,
        RestoreFidelityClass::PlaceholderBacked,
    ] {
        assert!(
            seen_fidelity.contains(&required),
            "missing fidelity {required:?}"
        );
    }
}

#[test]
fn narrowed_rows_drop_below_cutline_and_name_a_reason() {
    for scenario in window_topology_restore_corpus() {
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
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        let pillars = record.pillars;
        if ceiling.asserts_authority_topology_separated {
            assert!(
                pillars.authority_topology_separated,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_pane_tree_versioned {
            assert!(
                pillars.pane_tree_versioned_stable_ids,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_skeleton_first_hydrate_second {
            assert!(
                pillars.skeleton_first_hydrate_second,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_no_silent_rerun {
            assert!(
                pillars.no_silent_rerun_or_reacquire,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_provenance_export_safe {
            assert!(
                pillars.restore_provenance_export_safe,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_recovery_chrome_reachable {
            assert!(
                pillars.recovery_chrome_reachable,
                "{}",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn recovery_routes_are_complete_and_keyboard_reachable() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let route_ids: Vec<&str> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in WindowRestoreRecoveryAction::REQUIRED {
            assert!(
                route_ids.contains(&required.as_str()),
                "{} missing recovery route {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        for route in &record.recovery_routes {
            assert!(
                route.keyboard_reachable,
                "{} recovery route {} not keyboard reachable",
                scenario.scenario_id, route.action_id,
            );
        }
    }
}

#[test]
fn routes_reach_every_surface_keyboard_first() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in AttentionRouteSurface::REQUIRED {
            let route = record
                .routes
                .iter()
                .find(|route| route.surface == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing route surface {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                route.keyboard_reachable,
                "{} route {} not keyboard reachable",
                scenario.scenario_id,
                required.as_str()
            );
            assert!(
                route.activates_same_item,
                "{} route {} activates a different item",
                scenario.scenario_id,
                required.as_str()
            );
            assert!(
                is_canonical_object_ref(&route.route_ref),
                "{} route {} ref {:?} not canonical",
                scenario.scenario_id,
                required.as_str(),
                route.route_ref,
            );
        }
    }
}

#[test]
fn accessibility_holds_in_every_layout() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.accessibility.action_labels.len(),
            record.recovery_routes.len(),
            "{} action labels drift from recovery routes",
            scenario.scenario_id,
        );
        for (label, route) in record
            .accessibility
            .action_labels
            .iter()
            .zip(record.recovery_routes.iter())
        {
            assert_eq!(
                label, &route.action_label,
                "{} action label drift",
                scenario.scenario_id
            );
        }
        for required in LayoutMode::REQUIRED {
            let mode = record
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
                mode.row_narration_available && mode.recovery_affordances_reachable,
                "{} layout mode {} unreachable",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn rows_stay_available_without_account_or_managed_services() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.available_without_account,
            "{} hidden without an account",
            scenario.scenario_id
        );
        assert!(
            record.available_without_managed_services,
            "{} hidden without managed services",
            scenario.scenario_id,
        );
    }
}

#[test]
fn minted_refs_are_canonical_durable_objects() {
    for scenario in window_topology_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for (label, value) in [
            ("diagnostics_export_ref", &record.diagnostics_export_ref),
            ("support_export_ref", &record.support_export_ref),
            (
                "authority.workspace_authority_ref",
                &record.authority.workspace_authority_ref,
            ),
            (
                "restore_provenance.restore_provenance_ref",
                &record.restore_provenance.restore_provenance_ref,
            ),
        ] {
            assert!(
                is_canonical_object_ref(value),
                "{} {label} {value:?} not canonical",
                scenario.scenario_id,
            );
        }
        for slot in &record.pane_tree.slots {
            assert!(
                is_canonical_object_ref(&slot.reopen_target_ref),
                "{} pane {} reopen ref not canonical",
                scenario.scenario_id,
                slot.pane_id,
            );
        }
        for value in record
            .evidence_refs
            .iter()
            .chain(record.narrative_refs.iter())
        {
            assert!(
                is_canonical_object_ref(value),
                "{} ref {value:?} not canonical",
                scenario.scenario_id,
            );
        }
    }
}
