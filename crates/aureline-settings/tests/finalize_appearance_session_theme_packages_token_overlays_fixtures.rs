//! Fixture-replay and invariant tests for the stable appearance-session
//! finalization certification corpus.
//!
//! The records live under
//! `fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays/`
//! and are minted by the
//! `aureline_settings_finalize_appearance_session_theme_packages_token_overlays`
//! emitter so the checked-in JSON stays a literal projection of the in-code
//! corpus, which is itself a projection of the live appearance runtime.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-settings \
//!     --bin aureline_settings_finalize_appearance_session_theme_packages_token_overlays -- emit-fixtures \
//!     fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays
//!   ```
//!
//! - Theme package manifests are versioned and declare provenance, supported
//!   modes, density defaults, motion flags, and minimum contrast metadata.
//! - Appearance session summaries are exportable, cite one package source, and
//!   carry checkpoint/rollback information.
//! - Token overlays are validated by scope; unknown or unsupported tokens are
//!   preserved inert or downgraded, never silently dropped.
//! - Imported-theme mapping reports name translated, unsupported, unresolved,
//!   and fallback slots, and block full-fidelity claims without evidence.
//! - Extension/embedded surfaces declare inheritance or surface visible gaps in
//!   product, exported appearance packets, and migration/support diagnostics.
//! - Live OS appearance changes apply coherently or disclose reload/restart —
//!   never a silent lag.
//! - Appearance provenance survives import/export/sync without flattening.
//! - No posture over-claims; a posture that cannot prove a pillar or sits on a
//!   below-Stable row is narrowed below Stable with a named reason.
//! - The same record opens from the settings appearance panel, command palette,
//!   status bar, and a menu command, keyboard-first, across normal /
//!   high-contrast / zoomed layouts.
//! - Every record stays available without an account or managed services.

use std::collections::BTreeSet;

use aureline_settings::finalize_appearance_session_theme_packages_token_overlays::{
    appearance_session_finalization_corpus, is_canonical_object_ref,
    AppearanceSessionFinalizationCertification, LayoutMode, LiveApplyClass, OverlayScopeClass,
    ProvenanceDimensionClass, RouteSurface, StableClaimClass,
    APPEARANCE_SESSION_FINALIZATION_RECORD_KIND,
    APPEARANCE_SESSION_FINALIZATION_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays",
);

fn load_record(filename: &str) -> AppearanceSessionFinalizationCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in appearance_session_finalization_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-settings --bin aureline_settings_finalize_appearance_session_theme_packages_token_overlays -- emit-fixtures fixtures/ux/m4/finalize-appearance-session-theme-packages-token-overlays`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in appearance_session_finalization_corpus() {
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
    for scenario in appearance_session_finalization_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.record_kind,
            APPEARANCE_SESSION_FINALIZATION_RECORD_KIND
        );
        assert_eq!(
            record.shared_contract_ref,
            APPEARANCE_SESSION_FINALIZATION_SHARED_CONTRACT_REF
        );
        assert!(is_canonical_object_ref(&record.diagnostics_export_ref));
        assert!(is_canonical_object_ref(&record.support_export_ref));
        assert!(is_canonical_object_ref(
            &record.appearance_session.value_ref
        ));
    }
}

#[test]
fn every_theme_package_is_versioned_and_declares_provenance() {
    for scenario in appearance_session_finalization_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            !record.theme_packages.is_empty(),
            "{} must declare at least one theme package",
            scenario.scenario_id
        );
        for row in &record.theme_packages {
            assert!(
                row.manifest_versioned,
                "{} package {} must be versioned",
                scenario.scenario_id, row.package_ref
            );
            assert!(
                row.provenance_declared,
                "{} package {} must declare provenance",
                scenario.scenario_id, row.package_ref
            );
            assert!(
                row.trust_severity_semantics_preserved,
                "{} package {} must preserve trust/severity semantics",
                scenario.scenario_id, row.package_ref
            );
            assert!(
                !row.supported_theme_classes.is_empty(),
                "{} package {} must declare supported theme classes",
                scenario.scenario_id,
                row.package_ref
            );
            assert!(
                !row.supported_density_classes.is_empty(),
                "{} package {} must declare supported density classes",
                scenario.scenario_id,
                row.package_ref
            );
            assert!(
                !row.supported_motion_postures.is_empty(),
                "{} package {} must declare supported motion postures",
                scenario.scenario_id,
                row.package_ref
            );
        }
    }
}

#[test]
fn appearance_session_summaries_are_exportable_and_cite_one_package() {
    for scenario in appearance_session_finalization_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            !record.session_summaries.is_empty(),
            "{} must include at least one session summary",
            scenario.scenario_id
        );
        for row in &record.session_summaries {
            assert!(
                row.summary_exportable,
                "{} session {} must be exportable",
                scenario.scenario_id, row.appearance_session_id
            );
            assert!(
                row.cites_one_package_source,
                "{} session {} must cite one package source",
                scenario.scenario_id, row.appearance_session_id
            );
            assert_eq!(
                row.appearance_session_id, record.appearance_session.appearance_session_id,
                "{} session must cite the binding session",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn token_overlay_scopes_are_present_and_no_tokens_silently_dropped() {
    for scenario in appearance_session_finalization_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<OverlayScopeClass> =
            record.token_overlays.iter().map(|row| row.scope).collect();
        for required in OverlayScopeClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing overlay scope {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.token_overlays {
            assert!(
                row.scope_lineage_recorded,
                "{} overlay {} must record scope lineage",
                scenario.scenario_id,
                row.scope.as_str()
            );
        }
        if record.stable_qualification.qualifies_stable {
            assert!(record.pillars.token_overlays_validated);
        }
    }
}

#[test]
fn imported_theme_reports_are_honest_and_block_overclaim() {
    for scenario in appearance_session_finalization_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            !record.import_reports.is_empty(),
            "{} must include at least one import report",
            scenario.scenario_id
        );
        for row in &record.import_reports {
            assert!(
                row.syntax_coverage_reported,
                "{} import {} must report syntax coverage",
                scenario.scenario_id, row.report_ref
            );
            assert!(
                row.parity_notes_visible,
                "{} import {} must show parity notes",
                scenario.scenario_id, row.report_ref
            );
            assert!(
                row.fallback_behavior_documented,
                "{} import {} must document fallback behavior",
                scenario.scenario_id, row.report_ref
            );
        }
    }
}

#[test]
fn extension_gaps_are_visible_or_inheritance_is_full() {
    for scenario in appearance_session_finalization_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            !record.extension_descriptors.is_empty(),
            "{} must include at least one extension descriptor",
            scenario.scenario_id
        );
        for row in &record.extension_descriptors {
            let all_inherit = row.theme_inheritance.inherits_fully()
                && row.density_inheritance.inherits_fully()
                && row.high_contrast_inheritance.inherits_fully()
                && row.focus_inheritance.inherits_fully()
                && row.reduced_motion_inheritance.inherits_fully();
            if !all_inherit {
                assert!(
                    row.gap_visible_in_product
                        && row.gap_visible_in_export
                        && row.gap_visible_in_diagnostics,
                    "{} extension {} gap must be visible in product, export, and diagnostics",
                    scenario.scenario_id,
                    row.surface_id
                );
            }
        }
    }
}

#[test]
fn live_appearance_changes_never_silently_lag() {
    for scenario in appearance_session_finalization_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.live_changes.len(),
            6,
            "{} must cover every live-appearance axis",
            scenario.scenario_id
        );
        for row in &record.live_changes {
            assert!(
                !row.silently_lags_system,
                "{} axis {} must not silently lag",
                scenario.scenario_id,
                row.axis.as_str()
            );
            assert!(
                row.applies_coherently_or_discloses,
                "{} axis {} must apply coherently or disclose",
                scenario.scenario_id,
                row.axis.as_str()
            );
            if matches!(
                row.live_apply_class,
                LiveApplyClass::ReloadRequired | LiveApplyClass::RestartRequired
            ) {
                assert!(
                    row.disclosure_required,
                    "{} reload/restart axis {} must disclose",
                    scenario.scenario_id,
                    row.axis.as_str()
                );
            }
        }
    }
}

#[test]
fn provenance_dimensions_survive_export_and_sync() {
    for scenario in appearance_session_finalization_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<ProvenanceDimensionClass> =
            record.provenance.iter().map(|row| row.dimension).collect();
        for required in ProvenanceDimensionClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing provenance dimension {}",
                scenario.scenario_id,
                required.as_str()
            );
        }
        for row in &record.provenance {
            assert!(
                row.survives_sync_without_flattening,
                "{} provenance {} must survive sync without flattening",
                scenario.scenario_id,
                row.dimension.as_str()
            );
        }
        assert!(record.pillars.provenance_intact);
    }
}

#[test]
fn stable_postures_qualify_and_narrowed_postures_name_a_reason() {
    for scenario in appearance_session_finalization_corpus() {
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
    let records: Vec<_> = appearance_session_finalization_corpus()
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
    for scenario in appearance_session_finalization_corpus() {
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
