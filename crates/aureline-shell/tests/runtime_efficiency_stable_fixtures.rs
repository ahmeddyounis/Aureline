//! Fixture-replay and invariant tests for the stable runtime-efficiency corpus.
//!
//! The records live under
//! `fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible/`
//! and are minted by the `aureline_shell_runtime_efficiency_stable` emitter so
//! the checked-in JSON stays a literal projection of the in-code corpus, which
//! is itself a projection of the live efficiency runtime and the suspend-resume
//! / power-posture page.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_runtime_efficiency_stable -- emit-fixtures \
//!     fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible
//!   ```
//!
//! - All five runtime-efficiency states (Nominal, EfficiencyAware,
//!   ThermalConstrained, ProtectCore, Recovery) are materialized.
//! - Background work is shed before any foreground regression, and protected
//!   foreground paths stay within published latency bands on Stable rows.
//! - Hidden panes stay quiescent on Stable rows; the render-leak drill narrows.
//! - The queue-governor reason, paused lanes, and resume owner are surfaced for
//!   every pressured posture so a transition never masquerades as generic
//!   slowness or stale data.
//! - Save durability and local durable state are preserved everywhere.
//! - Per-OS conformance covers macOS, Windows, and Linux with current proof.
//! - The shell status strip, diagnostics review, CLI inspect, Help/About, and
//!   support export all bind the shared record.
//! - The matrix spans Stable and narrowed rows; a posture that cannot prove a
//!   pillar or sits on a below-Stable binding surface is narrowed with a named
//!   reason.
//! - The same posture opens from the activity center, command palette, status
//!   bar, and a menu command, keyboard-first, across normal / high-contrast /
//!   zoomed layouts, without an account or managed services.

use aureline_shell::notification_attention_stable::{
    is_canonical_object_ref, AttentionRouteSurface, LayoutMode, StableClaimClass,
};
use aureline_shell::runtime_efficiency_stable::{
    runtime_efficiency_corpus, EfficiencyRecoveryAction, EfficiencyTruthSurface,
    PlatformProfileClass, ProtectedForegroundPath, RuntimeEfficiencyRecord, ALL_EFFICIENCY_STATES,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible",
);

fn load_record(filename: &str) -> RuntimeEfficiencyRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in runtime_efficiency_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_runtime_efficiency_stable -- emit-fixtures fixtures/ux/m4/stabilize-battery-thermal-suspend-resume-and-user-visible`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.posture_id, scenario.expected_posture,
            "{} posture",
            scenario.scenario_id
        );
        assert_eq!(
            record.efficiency_state, scenario.expected_state,
            "{} efficiency state",
            scenario.scenario_id
        );
        assert_eq!(
            record.governor.reason, scenario.expected_governor,
            "{} governor reason",
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
fn matrix_materializes_every_efficiency_state() {
    let corpus = runtime_efficiency_corpus();
    for required in ALL_EFFICIENCY_STATES {
        assert!(
            corpus
                .iter()
                .any(|scenario| scenario.expected_state == required),
            "matrix must materialize efficiency state {}",
            required.as_str(),
        );
    }
}

#[test]
fn stable_rows_shed_background_before_foreground() {
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        // Every behavior-changing shed-work row sheds before foreground regresses.
        for row in &record.shed_work {
            if row.changes_behavior {
                assert!(
                    row.shed_before_foreground,
                    "{} sheds {} without protecting foreground first",
                    scenario.scenario_id, row.workload_id,
                );
            }
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.pillars.background_shed_before_foreground,
                "{} Stable but background not shed before foreground",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn protected_paths_complete_and_within_band_on_stable() {
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in ProtectedForegroundPath::REQUIRED {
            assert!(
                record
                    .protected_paths
                    .iter()
                    .any(|row| row.path == required),
                "{} missing protected path {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            for row in &record.protected_paths {
                assert!(
                    row.holds() && row.observed_p99_ms <= row.published_band_ms,
                    "{} Stable but protected path {} exceeds its band ({} > {})",
                    scenario.scenario_id,
                    row.path.as_str(),
                    row.observed_p99_ms,
                    row.published_band_ms,
                );
            }
            assert!(
                record.pillars.foreground_within_latency_bands,
                "{} Stable but foreground latency pillar is false",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn hidden_panes_quiescent_on_stable() {
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.hidden_pane_audit.passes_hidden_pane_policy,
                "{} Stable but hidden-pane audit fails",
                scenario.scenario_id,
            );
            assert_eq!(
                record.hidden_pane_audit.hidden_pane_render_violation_count, 0,
                "{} Stable but hidden-pane violations present",
                scenario.scenario_id,
            );
            assert!(
                record.pillars.hidden_panes_quiescent,
                "{} Stable but hidden-pane pillar is false",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn leak_drill_actually_leaks() {
    let record = load_record("hidden_pane_render_leak_drill.json");
    assert!(
        !record.hidden_pane_audit.passes_hidden_pane_policy,
        "leak drill must fail the hidden-pane audit",
    );
    assert!(
        record.hidden_pane_audit.hidden_pane_render_violation_count >= 1,
        "leak drill must record a hidden-pane render violation",
    );
    assert!(!record.pillars.hidden_panes_quiescent);
}

#[test]
fn governor_reason_surfaced_for_pressure_postures() {
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.governor.not_generic_slowness && record.governor.not_stale_masquerade,
            "{} governor masquerades as generic slowness or stale data",
            scenario.scenario_id,
        );
        if record.governor.reason.requires_surfacing() {
            assert!(
                record.governor.surfaced_in_status_strip && record.governor.surfaced_in_diagnostics,
                "{} pressured posture does not surface the governor reason",
                scenario.scenario_id,
            );
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.pillars.governor_reason_surfaced,
                "{} Stable but governor reason not surfaced",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn durable_state_preserved_everywhere() {
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.durability.save_durability_preserved
                && record.durability.dirty_buffers_preserved
                && record.durability.user_owned_artifacts_preserved,
            "{} loses durable state",
            scenario.scenario_id,
        );
        assert!(
            record.pillars.durable_state_preserved,
            "{} durable-state pillar false",
            scenario.scenario_id,
        );
    }
}

#[test]
fn per_os_conformance_is_complete_with_proof() {
    for scenario in runtime_efficiency_corpus() {
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
                !row.named_downgrade_behaviors.is_empty(),
                "{} profile {} names no downgrade behavior",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn surfaces_bind_the_shared_record() {
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in EfficiencyTruthSurface::REQUIRED {
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
    let corpus = runtime_efficiency_corpus();
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
    for scenario in runtime_efficiency_corpus() {
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
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        let pillars = record.pillars;
        if ceiling.asserts_efficiency_state_materialized {
            assert!(
                pillars.efficiency_state_materialized,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_background_shed_before_foreground {
            assert!(
                pillars.background_shed_before_foreground,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_foreground_within_latency_bands {
            assert!(
                pillars.foreground_within_latency_bands,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_hidden_panes_quiescent {
            assert!(pillars.hidden_panes_quiescent, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_governor_reason_surfaced {
            assert!(pillars.governor_reason_surfaced, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_durable_state_preserved {
            assert!(pillars.durable_state_preserved, "{}", scenario.scenario_id);
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
fn recovery_routes_are_complete_and_keyboard_reachable() {
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let route_ids: Vec<&str> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in EfficiencyRecoveryAction::REQUIRED {
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
    for scenario in runtime_efficiency_corpus() {
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
    for scenario in runtime_efficiency_corpus() {
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
fn suspend_resume_continuity_admits_no_silent_rerun() {
    let record = load_record("suspend_resume_recovery_stable.json");
    let continuity = record
        .suspend_resume
        .as_ref()
        .expect("recovery posture must carry suspend-resume continuity");
    assert!(
        continuity.local_work_continues,
        "local work must continue across resume"
    );
    assert!(
        continuity.privileged_or_mutating_work_paused,
        "privileged work must pause at the boundary",
    );
    assert!(
        continuity.no_silent_rerun_or_authority_reuse,
        "resume must not silently rerun or reuse authority",
    );
    assert!(
        continuity.user_visible_resume_summary_required,
        "resume must require a user-visible summary",
    );
}

#[test]
fn rows_stay_available_without_account_or_managed_services() {
    for scenario in runtime_efficiency_corpus() {
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
    for scenario in runtime_efficiency_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for (label, value) in [
            ("diagnostics_export_ref", &record.diagnostics_export_ref),
            ("support_export_ref", &record.support_export_ref),
        ] {
            assert!(
                is_canonical_object_ref(value),
                "{} {label} {value:?} not canonical",
                scenario.scenario_id,
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
