//! Conformance tests binding the shell efficiency vocabulary to the frozen
//! M5 efficiency-state governance matrix on disk.

use std::path::PathBuf;

use serde_json::Value;

use super::*;
use crate::efficiency::{
    EfficiencyState, EfficiencyStateSnapshot, HiddenPaneRenderAudit, RenderVisibilitySample,
};

/// Resolves the frozen governance matrix relative to this crate.
fn matrix_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(M5_EFFICIENCY_GOVERNANCE_MATRIX_REF)
}

fn load_matrix() -> Value {
    let path = matrix_path();
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("frozen governance matrix {path:?} is readable: {err}"));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("frozen governance matrix {path:?} parses: {err}"))
}

#[test]
fn matrix_record_kind_is_canonical() {
    let matrix = load_matrix();
    assert_eq!(
        matrix["record_kind"].as_str(),
        Some(M5_EFFICIENCY_GOVERNANCE_RECORD_KIND),
        "frozen matrix record_kind must equal the canonical constant",
    );
}

#[test]
fn closed_vocabularies_match_the_shell_tokens() {
    let matrix = load_matrix();
    let vocab = &matrix["closed_vocabularies"];
    for (name, expected) in canonical_vocabularies() {
        let found: Vec<String> = vocab[name]
            .as_array()
            .unwrap_or_else(|| panic!("frozen matrix declares closed vocabulary {name}"))
            .iter()
            .map(|token| {
                token
                    .as_str()
                    .expect("vocabulary token is a string")
                    .to_owned()
            })
            .collect();
        let expected: Vec<String> = expected.into_iter().map(str::to_owned).collect();
        assert_eq!(
            found, expected,
            "closed vocabulary {name} in the frozen matrix drifted from the shell tokens",
        );
    }
}

#[test]
fn new_vocabularies_round_trip_through_serde() {
    for behavior in HiddenPaneBehavior::ALL {
        let json = serde_json::to_value(behavior).unwrap();
        assert_eq!(json.as_str(), Some(behavior.as_str()));
        let back: HiddenPaneBehavior = serde_json::from_value(json).unwrap();
        assert_eq!(back, behavior);
    }
    for posture in OverridePosture::ALL {
        let json = serde_json::to_value(posture).unwrap();
        assert_eq!(json.as_str(), Some(posture.as_str()));
        let back: OverridePosture = serde_json::from_value(json).unwrap();
        assert_eq!(back, posture);
    }
    for state in EfficiencyRecoveryState::ALL {
        let json = serde_json::to_value(state).unwrap();
        assert_eq!(json.as_str(), Some(state.as_str()));
        let back: EfficiencyRecoveryState = serde_json::from_value(json).unwrap();
        assert_eq!(back, state);
    }
}

#[test]
fn override_posture_user_overridable_classification() {
    assert!(OverridePosture::UserOverrideSessionOnly.is_user_overridable());
    assert!(OverridePosture::UserOverridePersistent.is_user_overridable());
    assert!(!OverridePosture::NotOverridable.is_user_overridable());
    assert!(!OverridePosture::PolicyBlocked.is_user_overridable());
    assert!(!OverridePosture::AdminControlled.is_user_overridable());
}

#[test]
fn projection_stamps_the_matrix_reference_from_a_real_snapshot() {
    // A real shell surface: build a thermal-pressure snapshot from the live
    // efficiency runtime, then project it through the governance binding.
    let audit = HiddenPaneRenderAudit::from_samples(vec![RenderVisibilitySample {
        surface_id: "preview-1".to_owned(),
        surface_class: "preview_viewport".to_owned(),
        visibility_state: "hidden_tab".to_owned(),
        committed_paint_count: 0,
        hidden_pane_work: 0,
        offscreen_suppression_eligible: 1,
    }]);
    let snapshot = EfficiencyStateSnapshot::from_decisions(
        "ws:1",
        EfficiencyState::ThermalConstrained,
        vec![super::EfficiencyPressureSource::ThermalPressure],
        true,
        Vec::new(),
        audit,
        "2026-06-20T14:00:00Z",
    );

    let projection = EfficiencyGovernanceProjection::from_snapshot(
        &snapshot,
        &[
            HiddenPaneBehavior::RenderSuppressed,
            HiddenPaneBehavior::FullyQuiescent,
        ],
        OverridePosture::UserOverrideSessionOnly,
        EfficiencyRecoveryState::NotInRecovery,
    );

    assert_eq!(projection.matrix_ref, M5_EFFICIENCY_GOVERNANCE_MATRIX_REF);
    assert_eq!(projection.schema_ref, M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF);
    assert_eq!(
        projection.active_state,
        EfficiencyState::ThermalConstrained.as_str()
    );
    assert_eq!(
        projection.source_of_change,
        vec![super::EfficiencyPressureSource::ThermalPressure
            .as_str()
            .to_owned()],
    );
    assert_eq!(
        projection.hidden_pane_behaviors,
        vec!["render_suppressed".to_owned(), "fully_quiescent".to_owned()],
    );
    assert_eq!(projection.override_posture, "user_override_session_only");
    assert_eq!(projection.recovery_state, "not_in_recovery");
}
