//! Unit tests for the admin-plane matrix builder, invariants, and export-safety
//! rules.

use super::*;

#[test]
fn matrix_validates_and_all_invariants_hold() {
    let matrix = admin_plane_matrix();
    matrix.validate().expect("canonical matrix validates");
    assert!(matrix.all_invariants_hold());
    assert!(!matrix.invariants.is_empty());
}

#[test]
fn matrix_is_deterministic() {
    assert_eq!(admin_plane_matrix(), admin_plane_matrix());
}

#[test]
fn matrix_is_support_export_safe() {
    let matrix = admin_plane_matrix();
    assert!(matrix.raw_payload_excluded);
    assert!(matrix.is_support_export_safe());
}

#[test]
fn every_surface_family_is_present_once() {
    let matrix = admin_plane_matrix();
    assert_eq!(matrix.surfaces.len(), AdminSurfaceClass::ALL.len());
    for class in AdminSurfaceClass::ALL {
        let entry = matrix.surface(class).expect("surface present");
        assert_eq!(entry.surface_id, class.surface_id());
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.proof_packet_ref.is_empty());
        assert!(!entry.applicable_states.is_empty());
        assert!(!entry.controlled_vocabularies.is_empty());
        assert!(entry.ownership_fields.iter().any(|f| f.required));
    }
}

#[test]
fn every_admin_path_is_present_once() {
    let matrix = admin_plane_matrix();
    assert_eq!(matrix.admin_paths.len(), AdminPathClass::ALL.len());
    for class in AdminPathClass::ALL {
        let entry = matrix.path(class).expect("path present");
        assert_eq!(entry.path_id, class.path_id());
        assert!(!entry.deployment_profiles.is_empty());
        assert!(!entry.local_safe_baseline_ref.is_empty());
    }
}

#[test]
fn state_vocabulary_is_complete_and_unique() {
    let matrix = admin_plane_matrix();
    assert_eq!(matrix.state_vocabulary.len(), AdminStateClass::ALL.len());
    let mut tokens = std::collections::BTreeSet::new();
    for term in &matrix.state_vocabulary {
        assert_eq!(term.token, term.state.as_str());
        assert!(!term.derived_from_refs.is_empty());
        assert!(tokens.insert(term.token.clone()), "duplicate state token");
    }
}

#[test]
fn every_applicable_state_is_a_defined_vocabulary_term() {
    let matrix = admin_plane_matrix();
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
fn every_controlled_vocabulary_is_bound_by_some_surface() {
    let matrix = admin_plane_matrix();
    for vocab in ControlledVocabulary::ALL {
        assert!(
            matrix.surfaces.iter().any(|s| s.binds(vocab)),
            "controlled vocabulary {} bound by no surface",
            vocab.as_str()
        );
    }
}

#[test]
fn freshness_headlined_surfaces_downgrade_green() {
    let matrix = admin_plane_matrix();
    for class in [
        AdminSurfaceClass::EffectivePolicyView,
        AdminSurfaceClass::PolicyDiff,
        AdminSurfaceClass::RetentionDeletionMatrix,
        AdminSurfaceClass::ProcurementVerificationPacket,
        AdminSurfaceClass::EndpointPostureCard,
        AdminSurfaceClass::DecisionHistoryTimeline,
    ] {
        let surface = matrix.surface(class).expect("surface present");
        assert!(
            surface.can_show(AdminStateClass::UnconfirmedStale),
            "{} must carry the unconfirmed_stale downgrade",
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
fn locked_state_is_always_explained() {
    let matrix = admin_plane_matrix();
    for surface in &matrix.surfaces {
        if surface.can_show(AdminStateClass::LockedByPolicy) {
            assert!(
                surface.binds(ControlledVocabulary::PolicySourceState),
                "{} must bind policy_source_state to explain a lock",
                surface.surface.as_str()
            );
            assert!(
                surface.binds(ControlledVocabulary::OwnerEscalation),
                "{} must bind owner_escalation to explain a lock",
                surface.surface.as_str()
            );
        }
    }
}

#[test]
fn delete_capable_surfaces_show_receipt_or_hold() {
    let matrix = admin_plane_matrix();
    for surface in &matrix.surfaces {
        if surface.can_show(AdminStateClass::DeletePending) {
            assert!(
                surface.binds(ControlledVocabulary::DeleteExportState),
                "{} must bind delete_export_state",
                surface.surface.as_str()
            );
            assert!(
                surface.can_show(AdminStateClass::DeleteReceipted)
                    || surface.can_show(AdminStateClass::DeleteBlockedByHold),
                "{} must expose a receipt or blocked-by-hold path",
                surface.surface.as_str()
            );
        }
    }
}

#[test]
fn unverified_signature_surfaces_bind_verification_vocabulary() {
    let matrix = admin_plane_matrix();
    for surface in &matrix.surfaces {
        if surface.can_show(AdminStateClass::SignatureUnverified) {
            assert!(
                surface.binds(ControlledVocabulary::VerificationSignaturePosture),
                "{} must bind verification_signature_posture",
                surface.surface.as_str()
            );
        }
    }
}

#[test]
fn validate_rejects_a_raw_payload_flag_flip() {
    let mut matrix = admin_plane_matrix();
    matrix.raw_payload_excluded = false;
    assert!(matrix.validate().is_err());
    assert!(!matrix.is_support_export_safe());
}

#[test]
fn validate_rejects_an_unsafe_ref() {
    let mut matrix = admin_plane_matrix();
    matrix.surfaces[0]
        .produced_by_refs
        .push("https://internal.example.com/secret".to_owned());
    assert!(!matrix.is_support_export_safe());
    assert!(matrix.validate().is_err());
}

#[test]
fn validate_rejects_a_missing_proof_packet() {
    let mut matrix = admin_plane_matrix();
    matrix.surfaces[0].proof_packet_ref = String::new();
    // The proof-packet-mapped invariant is recomputed by the builder, but a
    // post-hoc flip must still be caught by structural validation.
    assert!(matrix.validate().is_err());
}

#[test]
fn human_readable_projection_renders() {
    let matrix = admin_plane_matrix();
    let lines = admin_plane_lines(&matrix);
    assert!(lines.iter().any(|l| l.contains("Admin-plane matrix")));
    assert!(lines.iter().any(|l| l.contains("Surfaces:")));
    assert!(lines.iter().any(|l| l.contains("Paths:")));
    for class in AdminSurfaceClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(class.as_str())),
            "projection must mention surface {}",
            class.as_str()
        );
    }
}
