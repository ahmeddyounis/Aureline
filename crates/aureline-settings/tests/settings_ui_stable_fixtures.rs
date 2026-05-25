//! Fixture-replay and invariant tests for the stable settings-UI certification
//! corpus.
//!
//! The records live under
//! `fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector/` and
//! are minted by the `aureline_settings_finalize_settings_ui_stable` emitter so
//! the checked-in JSON stays a literal projection of the in-code corpus, which is
//! itself a projection of the live settings runtime.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-settings \
//!     --bin aureline_settings_finalize_settings_ui_stable -- emit-fixtures \
//!     fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector
//!   ```
//!
//! - Every visible setting resolves through one effective-setting record that can
//!   explain value, winning scope, lock state/reason, and restart posture.
//! - The shadow chain exposes the active profile, temporary profile,
//!   machine-local, synced, workspace, and policy-owned contributors rather than
//!   implying one flat value.
//! - Every previewable write is scope-explicit and, when denied, names a typed
//!   blocked-write reason and a Diagnostics Center entry point.
//! - The desktop UI, CLI inspect, Help/About, diagnostics/support export, and
//!   migration/import review all consume the shared record.
//! - The profile-switch review summarizes immediate changes, restart-required
//!   deltas, excluded machine-local state, narrowing effects, and rollback
//!   posture.
//! - No posture over-claims; a posture that cannot prove a pillar or sits on a
//!   below-Stable surface is narrowed below Stable with a named reason.
//! - The same record opens from the settings inspector, command palette, status
//!   bar, and a menu command, keyboard-first, across normal / high-contrast /
//!   zoomed layouts, available without an account or managed services.

use std::collections::BTreeSet;

use aureline_settings::settings_ui_stable::{
    is_canonical_object_ref, settings_ui_corpus, ContributorClass, LayoutMode, RouteSurface,
    SettingsUiCertification, StableClaimClass, SurfaceClass, SETTINGS_UI_RECORD_KIND,
    SETTINGS_UI_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector",
);

fn load_record(filename: &str) -> SettingsUiCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in settings_ui_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-settings --bin aureline_settings_finalize_settings_ui_stable -- emit-fixtures fixtures/ux/m4/finalize-the-settings-ui-with-effective-value-inspector`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in settings_ui_corpus() {
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
    for scenario in settings_ui_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, SETTINGS_UI_RECORD_KIND);
        assert_eq!(record.shared_contract_ref, SETTINGS_UI_SHARED_CONTRACT_REF);
        assert!(is_canonical_object_ref(&record.diagnostics_export_ref));
        assert!(is_canonical_object_ref(&record.support_export_ref));
    }
}

#[test]
fn every_setting_resolves_through_one_record() {
    for scenario in settings_ui_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            !record.effective_settings.is_empty(),
            "{} must certify at least one setting",
            scenario.scenario_id
        );
        for row in &record.effective_settings {
            assert!(
                row.resolves_through_one_record,
                "{} setting {} must resolve through one record",
                scenario.scenario_id, row.setting_id
            );
            assert!(
                !row.winning_scope.is_empty()
                    && !row.lock_state.is_empty()
                    && !row.lock_reason.is_empty()
                    && !row.restart_posture.is_empty()
                    && !row.shadow_chain.is_empty(),
                "{} setting {} must explain value, scope, lock, restart, and source chain",
                scenario.scenario_id,
                row.setting_id
            );
            assert!(is_canonical_object_ref(&row.effective_record_ref));
            assert!(is_canonical_object_ref(&row.escalation_path_ref));
        }
        assert!(record.pillars.every_setting_resolves_one_record);
    }
}

#[test]
fn shadow_chain_exposes_required_contributors() {
    for scenario in settings_ui_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let exposed: BTreeSet<ContributorClass> = record
            .effective_settings
            .iter()
            .flat_map(|row| row.shadow_chain.iter().map(|c| c.contributor_class))
            .collect();
        for required in ContributorClass::REQUIRED_COVERAGE {
            assert!(
                exposed.contains(&required),
                "{} shadow chain must expose contributor {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        assert!(record.pillars.shadow_chain_exposes_contributors);
        // The certification's contributor_coverage rollup must match what the
        // chains actually expose.
        let rollup: BTreeSet<ContributorClass> =
            record.contributor_coverage.iter().copied().collect();
        assert_eq!(
            rollup, exposed,
            "{} contributor rollup",
            scenario.scenario_id
        );
    }
}

#[test]
fn previewable_writes_are_scope_explicit_and_honest_when_denied() {
    for scenario in settings_ui_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            !record.previewable_writes.is_empty(),
            "{} must exercise at least one previewable write",
            scenario.scenario_id
        );
        for write in &record.previewable_writes {
            assert!(
                write.scope_explicit,
                "{} write {} must be scope-explicit",
                scenario.scenario_id, write.setting_id
            );
            assert!(is_canonical_object_ref(&write.target_artifact_ref));
            assert!(is_canonical_object_ref(&write.diagnostics_entry_ref));
            if write.denied {
                assert!(
                    write.blocked_write_reason.is_some(),
                    "{} denied write {} must name a blocked reason",
                    scenario.scenario_id,
                    write.setting_id
                );
            }
        }
        // The matrix must exercise both an allowed and a denied write.
        assert!(
            record.previewable_writes.iter().any(|w| !w.denied),
            "{} must include an allowed write",
            scenario.scenario_id
        );
        assert!(
            record.previewable_writes.iter().any(|w| w.denied),
            "{} must include a denied write with a typed reason",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_required_surface_shares_one_truth() {
    for scenario in settings_ui_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<SurfaceClass> = record
            .surface_parity
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for required in SurfaceClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing surface {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.surface_parity {
            assert!(is_canonical_object_ref(&row.record_ref));
        }
    }
}

#[test]
fn profile_switch_review_summarizes_the_switch() {
    for scenario in settings_ui_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let review = &record.profile_switch_review;
        assert!(!review.summary.trim().is_empty());
        assert!(review.conforms);
        assert_eq!(
            review.creates_rollback_checkpoint,
            review.rollback_checkpoint_ref.is_some(),
            "{} checkpoint ref must accompany a created checkpoint",
            scenario.scenario_id
        );
        if let Some(checkpoint) = &review.rollback_checkpoint_ref {
            assert!(is_canonical_object_ref(checkpoint));
        }
        // The review must demonstrate immediate, restart-required, excluded
        // machine-local, and narrowing facets.
        assert!(
            !review.immediate_changes.is_empty(),
            "{} review must list immediate changes",
            scenario.scenario_id
        );
        assert!(
            !review.restart_required_changes.is_empty(),
            "{} review must list restart-required deltas",
            scenario.scenario_id
        );
        assert!(
            !review.excluded_machine_specific.is_empty(),
            "{} review must exclude machine-local state",
            scenario.scenario_id
        );
        assert!(
            !review.narrowing_effects.is_empty(),
            "{} review must name narrowing effects",
            scenario.scenario_id
        );
        assert!(record.pillars.profile_switch_review_complete);
    }
}

#[test]
fn stable_postures_qualify_and_narrowed_postures_name_a_reason() {
    for scenario in settings_ui_corpus() {
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
    let records: Vec<_> = settings_ui_corpus()
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
fn setting_ids_stay_canonical_in_exports() {
    for scenario in settings_ui_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for row in &record.effective_settings {
            assert!(
                row.setting_id_canonical && !row.setting_id.is_empty(),
                "{} setting id {:?} must stay canonical",
                scenario.scenario_id,
                row.setting_id
            );
            // The certified id appears verbatim in the upstream rollup.
            assert!(
                record
                    .upstream
                    .certified_setting_ids
                    .contains(&row.setting_id),
                "{} upstream must carry setting id {}",
                scenario.scenario_id,
                row.setting_id
            );
        }
        assert!(record.pillars.setting_ids_canonical_in_exports);
    }
}

#[test]
fn record_is_reachable_keyboard_first_across_layouts() {
    for scenario in settings_ui_corpus() {
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
