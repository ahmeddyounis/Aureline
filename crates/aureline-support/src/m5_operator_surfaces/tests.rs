//! Unit tests for the operator-surface matrix builder, invariants, and
//! export-safety rules.

use super::*;

#[test]
fn matrix_validates_and_all_invariants_hold() {
    let matrix = operator_surface_matrix();
    matrix.validate().expect("canonical matrix validates");
    assert!(matrix.all_invariants_hold());
    assert!(!matrix.invariants.is_empty());
}

#[test]
fn matrix_is_deterministic() {
    assert_eq!(operator_surface_matrix(), operator_surface_matrix());
}

#[test]
fn matrix_is_support_export_safe() {
    let matrix = operator_surface_matrix();
    assert!(matrix.raw_payload_excluded);
    assert!(matrix.is_support_export_safe());
}

#[test]
fn every_surface_family_is_present_once() {
    let matrix = operator_surface_matrix();
    assert_eq!(matrix.surfaces.len(), OperatorSurfaceClass::ALL.len());
    for class in OperatorSurfaceClass::ALL {
        let entry = matrix.surface(class).expect("surface present");
        assert_eq!(entry.surface_id, class.surface_id());
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.applicable_states.is_empty());
        assert!(entry.ownership_fields.iter().any(|f| f.required));
    }
}

#[test]
fn every_operator_path_is_present_once() {
    let matrix = operator_surface_matrix();
    assert_eq!(matrix.operator_paths.len(), OperatorPathClass::ALL.len());
    for class in OperatorPathClass::ALL {
        let entry = matrix.path(class).expect("path present");
        assert_eq!(entry.path_id, class.path_id());
        assert!(!entry.deployment_profiles.is_empty());
    }
}

#[test]
fn state_vocabulary_is_complete_and_unique() {
    let matrix = operator_surface_matrix();
    assert_eq!(matrix.state_vocabulary.len(), OperatorStateClass::ALL.len());
    let mut tokens = std::collections::BTreeSet::new();
    for term in &matrix.state_vocabulary {
        assert_eq!(term.token, term.state.as_str());
        assert!(!term.derived_from_refs.is_empty());
        assert!(tokens.insert(term.token.clone()), "duplicate state token");
    }
}

#[test]
fn every_applicable_state_is_a_defined_vocabulary_term() {
    let matrix = operator_surface_matrix();
    for surface in &matrix.surfaces {
        for state in &surface.applicable_states {
            assert!(
                matrix.state_term(*state).is_some(),
                "surface {} references undefined state {}",
                surface.surface.as_str(),
                state.as_str()
            );
        }
    }
}

#[test]
fn freshness_headlined_surfaces_downgrade_green() {
    let matrix = operator_surface_matrix();
    for class in [
        OperatorSurfaceClass::OperationalOverviewBoard,
        OperatorSurfaceClass::TriageInbox,
        OperatorSurfaceClass::ShiftDigest,
        OperatorSurfaceClass::ServiceOwnershipStrip,
    ] {
        let surface = matrix.surface(class).expect("surface present");
        assert!(
            surface
                .applicable_states
                .contains(&OperatorStateClass::Unconfirmed),
            "{} must carry the unconfirmed downgrade",
            class.as_str()
        );
        assert!(
            surface.freshness_rule.downgrades_green,
            "{} must downgrade green on stale evidence",
            class.as_str()
        );
    }
}

#[test]
fn window_surfaces_keep_local_safe_actions_and_publish_later() {
    let matrix = operator_surface_matrix();
    for surface in &matrix.surfaces {
        let in_window = surface
            .applicable_states
            .contains(&OperatorStateClass::ReadOnlyWindow)
            || surface
                .applicable_states
                .contains(&OperatorStateClass::DrainWindow);
        if !in_window {
            continue;
        }
        assert!(
            !surface.local_safe_actions.is_empty(),
            "{} must keep local-safe actions during a window",
            surface.surface.as_str()
        );
        if surface.captures_user_writes {
            assert!(
                surface.publish_later_capture,
                "{} must offer publish-later capture during a window",
                surface.surface.as_str()
            );
        }
    }
}

#[test]
fn embedded_handoff_surfaces_are_boundary_honest() {
    let matrix = operator_surface_matrix();
    for surface in &matrix.surfaces {
        if surface
            .applicable_states
            .contains(&OperatorStateClass::EmbeddedBoundaryHandoff)
        {
            assert!(surface.boundary_honest);
            assert!(!surface.boundary_note.is_empty());
        }
    }
}

#[test]
fn handoff_bundle_preserves_truth_fields() {
    let matrix = operator_surface_matrix();
    let bundle = matrix
        .surface(OperatorSurfaceClass::HandoffBundle)
        .expect("handoff bundle present");
    for required in [
        "bundle_scope",
        "retention_owner",
        "redaction_class",
        "live_vs_snapshot",
    ] {
        assert!(
            bundle
                .ownership_fields
                .iter()
                .any(|f| f.field_id == required && f.required),
            "handoff bundle must carry required field {required}"
        );
    }
    assert!(bundle
        .applicable_states
        .contains(&OperatorStateClass::ImportedSnapshotNoLive));
}

#[test]
fn validate_rejects_a_raw_payload_flag_flip() {
    let mut matrix = operator_surface_matrix();
    matrix.raw_payload_excluded = false;
    assert!(matrix.validate().is_err());
    assert!(!matrix.is_support_export_safe());
}

#[test]
fn validate_rejects_an_unsafe_ref() {
    let mut matrix = operator_surface_matrix();
    matrix.surfaces[0]
        .produced_by_refs
        .push("https://internal.example.com/secret".to_owned());
    assert!(!matrix.is_support_export_safe());
    assert!(matrix.validate().is_err());
}

#[test]
fn human_readable_projection_renders() {
    let matrix = operator_surface_matrix();
    let lines = operator_surface_lines(&matrix);
    assert!(lines.iter().any(|l| l.contains("Operator-surface matrix")));
    assert!(lines.iter().any(|l| l.contains("Surfaces:")));
    assert!(lines.iter().any(|l| l.contains("Paths:")));
    for class in OperatorSurfaceClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(class.as_str())),
            "projection must mention surface {}",
            class.as_str()
        );
    }
}
