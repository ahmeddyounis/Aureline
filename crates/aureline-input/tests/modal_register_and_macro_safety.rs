use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_input::modal::{
    ClipboardRouteKind, EditTargetScope, MacroReplayDecisionClass, MacroReplayPreviewRecord,
    MacroReplayRiskClass, ModalRecoveryActionClass, ModalSequenceState, ModalStateSnapshot,
    RegisterScopeKind,
};
use serde_json::Value;

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/input/m3/modal_register_and_macro_safety")
}

fn fixture_paths() -> Vec<PathBuf> {
    let mut paths = fs::read_dir(fixture_dir())
        .expect("fixture directory exists")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn load_records() -> (Vec<ModalStateSnapshot>, Vec<MacroReplayPreviewRecord>) {
    let mut states = Vec::new();
    let mut previews = Vec::new();
    for path in fixture_paths() {
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let value: Value = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
        match value["record_kind"].as_str() {
            Some("modal_state_snapshot_record") => {
                states.push(
                    serde_json::from_value(value)
                        .unwrap_or_else(|err| panic!("invalid modal state fixture: {err}")),
                );
            }
            Some("macro_replay_preview_record") => {
                previews.push(
                    serde_json::from_value(value)
                        .unwrap_or_else(|err| panic!("invalid macro replay fixture: {err}")),
                );
            }
            other => panic!("unknown record_kind in {}: {other:?}", path.display()),
        }
    }
    (states, previews)
}

#[test]
fn modal_state_and_macro_replay_fixtures_validate() {
    let (states, previews) = load_records();
    assert!(!states.is_empty(), "expected modal state fixtures");
    assert!(!previews.is_empty(), "expected macro replay fixtures");

    for state in &states {
        let report = state.validate();
        assert!(
            report.passed,
            "modal state fixture {} failed validation: {:#?}",
            state.state_id, report.findings
        );
    }
    for preview in &previews {
        let report = preview.validate();
        assert!(
            report.passed,
            "macro replay fixture {} failed validation: {:#?}",
            preview.preview_id, report.findings
        );
    }
}

#[test]
fn fixtures_cover_register_clipboard_and_surface_truth() {
    let (states, previews) = load_records();
    let mut register_scopes = BTreeSet::new();
    let mut clipboard_routes = BTreeSet::new();
    let mut sequence_states = BTreeSet::new();
    let mut recovery_actions = BTreeSet::new();

    for state in &states {
        register_scopes.insert(state.register_route.active_register.scope_kind);
        clipboard_routes.insert(state.register_route.clipboard_route);
        if let Some(guide) = &state.sequence_guide {
            sequence_states.insert(guide.sequence_state);
        }
        for action in &state.recovery_actions {
            recovery_actions.insert(action.action_class);
        }
    }
    for preview in &previews {
        register_scopes.insert(preview.source_register.scope_kind);
        register_scopes.insert(preview.register_route.active_register.scope_kind);
        clipboard_routes.insert(preview.register_route.clipboard_route);
    }

    for expected in [
        RegisterScopeKind::EditorLocal,
        RegisterScopeKind::SystemClipboard,
        RegisterScopeKind::RemoteClipboardBridge,
        RegisterScopeKind::NamedRegister,
        RegisterScopeKind::SearchHistory,
        RegisterScopeKind::Macro,
    ] {
        assert!(
            register_scopes.contains(&expected),
            "missing register scope coverage: {expected:?}"
        );
    }

    for expected in [
        ClipboardRouteKind::LocalEditorRegister,
        ClipboardRouteKind::LocalSystemClipboard,
        ClipboardRouteKind::RemoteClipboardBridge,
        ClipboardRouteKind::RemoteBridgeSuppressed,
        ClipboardRouteKind::AdminBlocked,
        ClipboardRouteKind::UnsupportedOnSurface,
    ] {
        assert!(
            clipboard_routes.contains(&expected),
            "missing clipboard route coverage: {expected:?}"
        );
    }

    assert!(sequence_states.contains(&ModalSequenceState::PartialWaiting));
    assert!(sequence_states.contains(&ModalSequenceState::UnsupportedSurface));
    assert!(recovery_actions.contains(&ModalRecoveryActionClass::ReviewRegisterRoute));
    assert!(recovery_actions.contains(&ModalRecoveryActionClass::RetryOnSupportedSurface));
}

#[test]
fn macro_replay_never_silently_widens_scope() {
    let (_, previews) = load_records();
    for preview in &previews {
        let widened = preview.target_scope != EditTargetScope::CurrentBuffer
            || !preview.risks.is_empty()
            || preview.register_route.route_changes_result
            || preview
                .command_steps
                .iter()
                .any(|step| step.crosses_files || step.mutates_settings || step.run_capable);
        if widened {
            assert!(
                preview.review_required,
                "{} must require review",
                preview.preview_id
            );
            assert!(
                !preview.can_silently_proceed,
                "{} must not silently proceed",
                preview.preview_id
            );
            assert_ne!(
                preview.decision,
                MacroReplayDecisionClass::ProceedLocalEditorOnly,
                "{} must leave the silent proceed lane",
                preview.preview_id
            );
        }
    }
}

#[test]
fn unsupported_imported_macro_sequences_fail_closed() {
    let (_, previews) = load_records();
    let denied = previews
        .iter()
        .find(|preview| {
            preview
                .risks
                .iter()
                .any(|risk| risk.risk_class == MacroReplayRiskClass::UnsupportedImportedSequence)
        })
        .expect("fixture with unsupported imported sequence");

    assert_eq!(
        denied.decision,
        MacroReplayDecisionClass::DeniedUnsafeReplay
    );
    assert!(denied.review_required);
    assert!(!denied.can_silently_proceed);
    assert!(!denied.diagnostics.is_empty());
}
