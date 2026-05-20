//! Functional and failure-drill fixtures for the restore hydrator.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    AuthorityRebindResult, Bounds, LayoutRestoreProvenanceRecordKind, PlaceholderActionClass,
    PlaceholderReasonClass, RestoreDisplayAdjustmentClass, RestoreHydrationError,
    RestoreHydrationOutcome, RestoreHydrationRequest, RestoreLevel, RestoreNoRerunGuardrail,
    RestorePhase, RestoreSurfaceRestorePosture,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_request(name: &str) -> RestoreHydrationRequest {
    let path = repo_root()
        .join("fixtures/workspace/m3/restore_hydration")
        .join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

fn bounds_contains(outer: &Bounds, inner: &Bounds) -> bool {
    inner.width > 0
        && inner.height > 0
        && inner.x >= outer.x
        && inner.y >= outer.y
        && inner.x + inner.width <= outer.x + outer.width
        && inner.y + inner.height <= outer.y + outer.height
}

#[test]
fn all_ready_request_yields_exact_restore_with_no_placeholders() {
    let request = read_request("all_ready_request.json");
    let outcome = request.hydrate().expect("all-ready request hydrates");

    assert_eq!(outcome.windows.len(), 1);
    let window = &outcome.windows[0];
    assert!(window.shell_restored);
    assert_eq!(
        window.preserved_pane_ids,
        vec![
            "pane-editor-left-0001".to_string(),
            "pane-explorer-right-0001".to_string()
        ]
    );
    assert_eq!(window.provenance.restore_level, RestoreLevel::ExactRestore);
    assert!(window.provenance.placeholder_results.is_empty());
    assert!(window.provenance.live_surface_outcomes.is_empty());
    assert!(window.provenance.display_adjustments.is_empty());
    assert_eq!(
        window.provenance.authority_rebind_result,
        AuthorityRebindResult::BoundExistingAuthority
    );
    assert_eq!(
        outcome.summary.aggregate_restore_level,
        RestoreLevel::ExactRestore
    );
    assert_eq!(outcome.summary.pane_count, 2);
    assert!(outcome.summary.missing_dependency_classes.is_empty());
}

#[test]
fn skeleton_phase_completes_before_hydrate_phase() {
    let request = read_request("all_ready_request.json");
    let outcome = request.hydrate().expect("hydrates");
    let phases: Vec<RestorePhase> = outcome.windows[0]
        .provenance
        .phase_trace
        .iter()
        .map(|phase| phase.phase)
        .collect();
    assert_eq!(
        phases,
        vec![
            RestorePhase::Chooser,
            RestorePhase::Skeleton,
            RestorePhase::Hydrate,
            RestorePhase::Rebind,
            RestorePhase::EvidenceOnlyFallback,
        ]
    );
}

#[test]
fn degraded_request_preserves_every_pane_and_recovers_safely() {
    let request = read_request("multi_window_degraded_request.json");
    let outcome = request.hydrate().expect("degraded request hydrates");

    // Both window shells are restored and all four panes survive.
    assert_eq!(outcome.windows.len(), 2);
    let all_panes: Vec<&str> = outcome
        .windows
        .iter()
        .flat_map(|window| window.preserved_pane_ids.iter().map(String::as_str))
        .collect();
    for required in [
        "pane-editor-main-0001",
        "pane-terminal-0001",
        "pane-preview-ext-0001",
        "pane-notebook-0001",
    ] {
        assert!(all_panes.contains(&required), "missing pane {required}");
    }

    // No window is trapped off-screen: every applied bound sits in a connected display.
    let display = Bounds {
        x: 0,
        y: 0,
        width: 1440,
        height: 900,
    };
    for window in &outcome.windows {
        assert!(window.shell_restored);
        assert!(
            bounds_contains(&display, &window.applied_bounds),
            "window {} bounds {:?} escaped the display",
            window.window_id,
            window.applied_bounds
        );
    }

    // Aggregate restore class is honest: a heavy pane fell back to evidence only.
    assert_eq!(
        outcome.summary.aggregate_restore_level,
        RestoreLevel::EvidenceOnly
    );

    // Missing-dependency classes surface in the in-product vocabulary.
    for required in [
        PlaceholderReasonClass::NonReentrantLiveSurface,
        PlaceholderReasonClass::MissingExtension,
        PlaceholderReasonClass::MissingRemote,
    ] {
        assert!(
            outcome
                .summary
                .missing_dependency_classes
                .contains(&required),
            "missing reason {required:?}"
        );
    }
    assert!(outcome
        .summary
        .remaining_manual_actions
        .contains(&PlaceholderActionClass::RerunExplicitly));
}

#[test]
fn vanished_display_and_offscreen_bounds_are_remapped_and_recorded() {
    let request = read_request("multi_window_degraded_request.json");
    let outcome = request.hydrate().expect("hydrates");
    let primary = outcome
        .windows
        .iter()
        .find(|window| window.window_id == "window-primary-0001")
        .expect("primary window");

    let classes: Vec<RestoreDisplayAdjustmentClass> = primary
        .provenance
        .display_adjustments
        .iter()
        .map(|adjustment| adjustment.adjustment_class)
        .collect();
    for required in [
        RestoreDisplayAdjustmentClass::MovedToPrimaryDisplay,
        RestoreDisplayAdjustmentClass::ScaleNormalized,
        RestoreDisplayAdjustmentClass::SnappedToSafeBounds,
        RestoreDisplayAdjustmentClass::FullscreenCleared,
    ] {
        assert!(
            classes.contains(&required),
            "missing adjustment {required:?}"
        );
    }
    // Each adjustment keeps pane-id provenance.
    for adjustment in &primary.provenance.display_adjustments {
        assert!(!adjustment.affected_pane_ids.is_empty());
    }
    assert_eq!(primary.chosen_display_ref, "display-primary-0001");

    // The auxiliary window kept valid bounds, so it needs no adjustment.
    let aux = outcome
        .windows
        .iter()
        .find(|window| window.window_id == "window-auxiliary-0001")
        .expect("auxiliary window");
    assert!(aux.provenance.display_adjustments.is_empty());
}

#[test]
fn mutating_sessions_are_never_replayed_automatically() {
    let request = read_request("multi_window_degraded_request.json");
    let outcome = request.hydrate().expect("hydrates");

    let mut saw_terminal = false;
    let mut saw_notebook = false;
    for window in &outcome.windows {
        for outcome_row in &window.provenance.live_surface_outcomes {
            // Every live surface requires an explicit user action to resume.
            assert!(
                outcome_row
                    .no_rerun_guardrails
                    .contains(&RestoreNoRerunGuardrail::ExplicitUserActionRequired),
                "pane {} lacks explicit-user-action guardrail",
                outcome_row.pane_id
            );
            // A placeholder posture must never imply a live, ready surface.
            assert_ne!(
                outcome_row.restore_posture,
                RestoreSurfaceRestorePosture::LiveAttachVisible
            );
            if outcome_row.pane_id == "pane-terminal-0001" {
                saw_terminal = true;
                assert!(outcome_row
                    .no_rerun_guardrails
                    .contains(&RestoreNoRerunGuardrail::NoCommandRerun));
            }
            if outcome_row.pane_id == "pane-notebook-0001" {
                saw_notebook = true;
                assert!(outcome_row
                    .no_rerun_guardrails
                    .contains(&RestoreNoRerunGuardrail::NoCommandRerun));
            }
        }
    }
    assert!(saw_terminal, "terminal live outcome missing");
    assert!(saw_notebook, "notebook live outcome missing");
}

#[test]
fn missing_surfaces_reopen_as_placeholders_with_provenance() {
    let request = read_request("multi_window_degraded_request.json");
    let outcome = request.hydrate().expect("hydrates");

    let preview = outcome
        .windows
        .iter()
        .flat_map(|window| window.provenance.placeholder_results.iter())
        .find(|placeholder| placeholder.pane_id == "pane-preview-ext-0001")
        .expect("preview placeholder");
    assert_eq!(
        preview.placeholder_reason,
        PlaceholderReasonClass::MissingExtension
    );
    assert!(preview
        .safe_actions
        .contains(&PlaceholderActionClass::InstallExtension));
    assert!(preview.last_known_provenance_label.is_some());
    assert!(preview.evidence_retained);
}

#[test]
fn outcome_is_deterministic_and_round_trips_through_serde() {
    let request = read_request("multi_window_degraded_request.json");
    let first = request.hydrate().expect("hydrates");
    let second = request.hydrate().expect("hydrates again");
    assert_eq!(first, second, "hydration must be deterministic");

    let json = serde_json::to_string(&first).expect("serialize");
    let parsed: RestoreHydrationOutcome = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(first, parsed);
    parsed.validate().expect("round-tripped outcome validates");

    // The emitted provenance is a layout-restore provenance record.
    assert_eq!(
        first.windows[0].provenance.record_kind,
        LayoutRestoreProvenanceRecordKind::LayoutRestoreProvenanceRecord
    );
}

#[test]
fn support_plaintext_uses_in_product_vocabulary() {
    let request = read_request("multi_window_degraded_request.json");
    let outcome = request.hydrate().expect("hydrates");
    let rendered = outcome.summary.render_plaintext();
    assert!(rendered.contains("Restore Hydration Summary"));
    assert!(rendered.contains("Evidence only"));
    assert!(rendered.contains("Missing extension"));
    assert!(rendered.contains("Missing remote"));
    assert!(rendered.contains("diagnostics_ref=diagnostics:restore:"));
    assert!(rendered.contains("support_export_ref=support-export:restore:"));
}

#[test]
fn validator_rejects_primary_display_not_connected() {
    let mut request = read_request("all_ready_request.json");
    request.environment.primary_display_ref = "display-not-here".to_string();
    assert!(matches!(
        request.hydrate(),
        Err(RestoreHydrationError::PrimaryDisplayNotConnected { .. })
    ));
}

#[test]
fn validator_rejects_exact_window_carrying_a_placeholder() {
    let request = read_request("all_ready_request.json");
    let mut outcome = request.hydrate().expect("hydrates");
    // Force an exact window to carry a placeholder: this must be rejected.
    outcome.windows[0].provenance.placeholder_results.push(
        aureline_workspace::PlaceholderResultRecord {
            pane_id: "pane-editor-left-0001".to_string(),
            surface_role: aureline_workspace::SurfaceRole::Editor,
            surface_class: aureline_workspace::SurfaceClass::TextEditor,
            placeholder_reason: PlaceholderReasonClass::ManualRecoveryRequired,
            safe_actions: vec![PlaceholderActionClass::RetryHydrate],
            evidence_retained: false,
            last_known_provenance_label: None,
            note: None,
        },
    );
    assert!(matches!(
        outcome.validate(),
        Err(RestoreHydrationError::ExactWindowHasPlaceholder { .. })
    ));
}

#[test]
fn validator_rejects_live_outcome_without_no_rerun_guardrail() {
    let request = read_request("multi_window_degraded_request.json");
    let mut outcome = request.hydrate().expect("hydrates");
    let live = outcome
        .windows
        .iter_mut()
        .flat_map(|window| window.provenance.live_surface_outcomes.iter_mut())
        .next()
        .expect("a live outcome exists");
    live.no_rerun_guardrails = vec![RestoreNoRerunGuardrail::PlaceholderPreserved];
    assert!(matches!(
        outcome.validate(),
        Err(RestoreHydrationError::LiveOutcomeMissingNoRerun { .. })
    ));
}

#[test]
fn validator_rejects_outcome_referencing_an_unknown_pane() {
    let request = read_request("multi_window_degraded_request.json");
    let mut outcome = request.hydrate().expect("hydrates");
    outcome.windows[0].provenance.placeholder_results[0].pane_id = "pane-ghost".to_string();
    assert!(matches!(
        outcome.validate(),
        Err(RestoreHydrationError::OutcomeForUnknownPane { .. })
    ));
}
