//! Fixture-replay and invariant tests for the stable desktop
//! handoff-conformance corpus.
//!
//! The records live under
//! `fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded/`
//! and are minted by the `aureline_shell_desktop_handoff_conformance_stable`
//! emitter so the checked-in JSON stays a literal projection of the in-code
//! corpus, which is itself a projection of the live native desktop contract
//! packet and the system-browser return-paths page.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_desktop_handoff_conformance_stable -- emit-fixtures \
//!     fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded
//!   ```
//!
//! - Every required entry path (file association, protocol handler, system open,
//!   default-browser auth callback, reveal-in-shell, recent item, jump list,
//!   removable volume, network share, native open/save) is covered.
//! - Handler ownership enumerates every side-by-side channel and a Stable row
//!   never degrades into last-writer-wins.
//! - Claimed-identity / auth rows default to system-browser handoff or disclose
//!   an explicit exception with scope, return path, and recovery.
//! - Moved / removable / network / missing targets render recoverable
//!   placeholders with last-seen identity and explicit Locate / Open cached
//!   context / Close placeholder actions.
//! - Per-OS conformance covers macOS, Windows, and Linux with current proof.
//! - The matrix spans Stable and narrowed rows; a posture that cannot prove a
//!   pillar or sits on a below-Stable binding surface is narrowed below Stable
//!   with a named reason.
//! - The same posture opens from the activity center, command palette, status
//!   bar, and a menu command, keyboard-first, across normal / high-contrast /
//!   zoomed layouts.
//! - Every posture stays available without an account or managed services.

use aureline_shell::desktop_handoff_conformance_stable::{
    desktop_handoff_conformance_corpus, ChannelClass, DesktopHandoffConformanceRecord,
    EntryPathClass, HandoffRecoveryAction, HandoffTruthSurface, PlatformProfileClass,
};
use aureline_shell::notification_attention_stable::{
    is_canonical_object_ref, AttentionRouteSurface, LayoutMode, StableClaimClass,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded",
);

fn load_record(filename: &str) -> DesktopHandoffConformanceRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in desktop_handoff_conformance_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_desktop_handoff_conformance_stable -- emit-fixtures fixtures/ux/m4/finalize-desktop-handoff-file-association-protocol-handler-embedded`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in desktop_handoff_conformance_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.posture_id, scenario.expected_posture,
            "{} posture",
            scenario.scenario_id
        );
        assert_eq!(
            record.entry_path, scenario.expected_entry_path,
            "{} entry path",
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
fn matrix_covers_every_entry_path() {
    let corpus = desktop_handoff_conformance_corpus();
    for required in EntryPathClass::REQUIRED {
        assert!(
            corpus
                .iter()
                .any(|scenario| scenario.expected_entry_path == required),
            "matrix must cover entry path {}",
            required.as_str(),
        );
    }
}

#[test]
fn handler_ownership_enumerates_side_by_side_channels() {
    for scenario in desktop_handoff_conformance_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for channel in ChannelClass::REQUIRED {
            assert!(
                record
                    .handler_ownership
                    .side_by_side_channels
                    .contains(&channel),
                "{} missing side-by-side channel {}",
                scenario.scenario_id,
                channel.as_str(),
            );
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.pillars.handler_ownership_explicit,
                "{} Stable but ownership not explicit",
                scenario.scenario_id
            );
            assert!(
                record.handler_ownership.no_last_writer_wins,
                "{} Stable but allows last-writer-wins",
                scenario.scenario_id
            );
            assert!(
                record.handler_ownership.spoof_resistant,
                "{} Stable but not spoof-resistant",
                scenario.scenario_id
            );
        }
        assert!(
            is_canonical_object_ref(&record.handler_ownership.owning_channel_ref),
            "{} owning channel ref not canonical",
            scenario.scenario_id,
        );
    }
}

#[test]
fn auth_rows_default_to_system_browser_or_disclose_exception() {
    for scenario in desktop_handoff_conformance_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if !record.auth_default.applies {
            continue;
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record
                    .pillars
                    .system_browser_default_or_explicit_exception,
                "{} Stable auth row not system-browser-default or disclosed exception",
                scenario.scenario_id,
            );
            // A non-default auth row must carry scope, return path, and recovery.
            if !record.auth_default.default_to_system_browser {
                assert!(
                    record.auth_default.exception_scope_ref.is_some()
                        && record.auth_default.return_path_ref.is_some()
                        && record.auth_default.recovery_on_exception_ref.is_some(),
                    "{} disclosed exception missing scope/return/recovery",
                    scenario.scenario_id,
                );
            }
            // An embedded browser is never allowed on a Stable auth row.
            assert!(
                !record.auth_default.embedded_browser_used,
                "{} Stable auth row uses an embedded browser",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn moved_or_missing_targets_render_truthful_placeholders() {
    for scenario in desktop_handoff_conformance_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.recovery.availability.requires_placeholder() {
            assert!(
                record.recovery.placeholder_required,
                "{} unavailable target without a placeholder",
                scenario.scenario_id,
            );
            assert!(
                record.recovery.last_seen_identity_ref.is_some(),
                "{} placeholder without a last-seen identity",
                scenario.scenario_id,
            );
            for required in [
                HandoffRecoveryAction::LocateTarget,
                HandoffRecoveryAction::OpenCachedContext,
                HandoffRecoveryAction::ClosePlaceholder,
            ] {
                assert!(
                    record.recovery.recovery_actions.contains(&required),
                    "{} placeholder missing recovery action {}",
                    scenario.scenario_id,
                    required.as_str(),
                );
            }
        }
        assert!(
            record.recovery.no_silent_replay_or_authority_reuse,
            "{} permits silent replay or authority reuse",
            scenario.scenario_id,
        );
    }
}

#[test]
fn per_os_conformance_is_complete_with_proof() {
    for scenario in desktop_handoff_conformance_corpus() {
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
        }
    }
}

#[test]
fn surfaces_bind_the_shared_record() {
    for scenario in desktop_handoff_conformance_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in HandoffTruthSurface::REQUIRED {
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
    let corpus = desktop_handoff_conformance_corpus();
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
    for scenario in desktop_handoff_conformance_corpus() {
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
    for scenario in desktop_handoff_conformance_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        let pillars = record.pillars;
        if ceiling.asserts_typed_intent_preserved {
            assert!(pillars.typed_intent_preserved, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_handler_ownership_explicit {
            assert!(pillars.handler_ownership_explicit, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_system_browser_default {
            assert!(
                pillars.system_browser_default_or_explicit_exception,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_trust_review_enforced {
            assert!(pillars.trust_review_enforced, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_recovery_truthful {
            assert!(pillars.recovery_truthful, "{}", scenario.scenario_id);
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
    for scenario in desktop_handoff_conformance_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let route_ids: Vec<&str> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in HandoffRecoveryAction::REQUIRED {
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
    for scenario in desktop_handoff_conformance_corpus() {
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
    for scenario in desktop_handoff_conformance_corpus() {
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
    for scenario in desktop_handoff_conformance_corpus() {
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
    for scenario in desktop_handoff_conformance_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for (label, value) in [
            ("diagnostics_export_ref", &record.diagnostics_export_ref),
            ("support_export_ref", &record.support_export_ref),
            ("intent.canonical_target_ref", &record.intent.canonical_target_ref),
            (
                "handler_ownership.owning_channel_ref",
                &record.handler_ownership.owning_channel_ref,
            ),
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
