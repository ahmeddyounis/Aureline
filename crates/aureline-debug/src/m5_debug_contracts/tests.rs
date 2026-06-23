//! Unit tests for the M5 debug-contracts matrix builder, invariants, controlled
//! vocabulary, and export-safety rules.

use super::*;

#[test]
fn matrix_validates_and_all_invariants_hold() {
    let matrix = m5_debug_contracts_matrix();
    matrix.validate().expect("canonical matrix validates");
    assert!(matrix.all_invariants_hold());
    assert!(!matrix.invariants.is_empty());
}

#[test]
fn matrix_is_deterministic() {
    assert_eq!(m5_debug_contracts_matrix(), m5_debug_contracts_matrix());
}

#[test]
fn matrix_is_support_export_safe() {
    let matrix = m5_debug_contracts_matrix();
    assert!(matrix.raw_payload_excluded);
    assert!(matrix.is_support_export_safe());
}

#[test]
fn every_object_family_is_present_once() {
    let matrix = m5_debug_contracts_matrix();
    assert_eq!(matrix.objects.len(), DebugObjectClass::ALL.len());
    for class in DebugObjectClass::ALL {
        let entry = matrix.object(class).expect("object present");
        assert_eq!(entry.object_id, class.object_id());
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.proof_packet_ref.is_empty());
        assert!(!entry.applicable_states.is_empty());
        assert!(!entry.controlled_vocabularies.is_empty());
        assert!(!entry.consumed_by.is_empty());
        assert!(entry.required_fields.iter().any(|f| f.required));
    }
}

#[test]
fn state_vocabulary_is_complete_and_unique() {
    let matrix = m5_debug_contracts_matrix();
    assert_eq!(matrix.state_vocabulary.len(), DebugStateClass::ALL.len());
    for state in DebugStateClass::ALL {
        let term = matrix.state_term(state).expect("state present");
        assert_eq!(term.token, state.as_str());
        assert_eq!(term.vocabulary, state.vocabulary());
    }
    assert!(all_unique(
        matrix.state_vocabulary.iter().map(|t| t.token.as_str())
    ));
}

#[test]
fn every_named_controlled_vocabulary_is_bound() {
    let matrix = m5_debug_contracts_matrix();
    for vocab in DebugVocabulary::ALL {
        assert!(
            matrix.objects.iter().any(|o| o.binds(vocab)),
            "controlled vocabulary {} bound by no object",
            vocab.as_str()
        );
    }
}

#[test]
fn session_modes_are_distinct() {
    assert_eq!(DebugStateClass::SESSION_MODES.len(), 5);
    assert!(all_unique(
        DebugStateClass::SESSION_MODES.iter().map(|s| s.as_str())
    ));
}

#[test]
fn inspect_only_modes_carry_no_live_authority() {
    for state in [
        DebugStateClass::SessionCoreFile,
        DebugStateClass::SessionReplay,
        DebugStateClass::SessionInspectOnly,
    ] {
        assert!(!state.implies_live_authority());
        assert!(state.requires_disclosure());
    }
}

#[test]
fn stale_variable_never_implies_live() {
    assert!(!DebugStateClass::VariableStaleSinceResume.implies_live_authority());
    assert!(DebugStateClass::VariableStaleSinceResume.requires_disclosure());
    assert!(DebugStateClass::VariableLiveAtStop.implies_live_authority());
}

#[test]
fn mutating_evaluate_discloses_side_effects() {
    assert!(DebugStateClass::EvaluateMutating.discloses_side_effect_risk());
    assert!(DebugStateClass::EvaluateUnknownSideEffects.discloses_side_effect_risk());
    assert!(!DebugStateClass::EvaluateSideEffectFree.discloses_side_effect_risk());
}

#[test]
fn restore_layout_only_never_reacquires_authority() {
    assert!(!DebugStateClass::RestoreLayoutOnlyNotReattached.implies_live_authority());
    assert!(!DebugStateClass::RestoreReattachRequired.implies_live_authority());
    assert!(DebugStateClass::RestoreReacquiredAuthority.implies_live_authority());
}

#[test]
fn shared_support_vocabulary_objects_bind_shared_axes_and_support_export() {
    let matrix = m5_debug_contracts_matrix();
    for class in DebugObjectClass::SHARED_SUPPORT_VOCABULARY_OBJECTS {
        let entry = matrix.object(class).expect("object present");
        assert!(entry.binds(DebugVocabulary::SessionMode));
        assert!(entry.binds(DebugVocabulary::MappingFidelity));
        assert!(entry.consumed_by.contains(&DebugConsumer::SupportExport));
    }
}

#[test]
fn named_required_consumers_each_render_an_object() {
    let matrix = m5_debug_contracts_matrix();
    for consumer in DebugConsumer::NAMED_REQUIRED {
        assert!(
            matrix
                .objects
                .iter()
                .any(|o| o.consumed_by.contains(&consumer)),
            "consumer {} renders no object",
            consumer.as_str()
        );
    }
}

#[test]
fn applicable_states_match_bound_vocabularies() {
    let matrix = m5_debug_contracts_matrix();
    for object in &matrix.objects {
        for state in &object.applicable_states {
            assert!(
                object.binds(state.vocabulary()),
                "object {} shows state {} without binding axis {}",
                object.object.as_str(),
                state.as_str(),
                state.vocabulary().as_str()
            );
        }
    }
}

#[test]
fn objects_with_non_live_states_disclose_authority_posture() {
    let matrix = m5_debug_contracts_matrix();
    for object in &matrix.objects {
        if object.can_show_non_live_authority_state() {
            assert!(
                object.discloses_authority_posture,
                "object {} can be non-live but does not disclose authority posture",
                object.object.as_str()
            );
        }
    }
}

#[test]
fn lines_projection_is_non_empty() {
    let matrix = m5_debug_contracts_matrix();
    let lines = m5_debug_contracts_lines(&matrix);
    assert!(lines.len() > matrix.objects.len());
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
}

#[test]
fn rejects_empty_proof_packet() {
    let mut matrix = m5_debug_contracts_matrix();
    matrix.objects[0].proof_packet_ref.clear();
    assert!(matrix.validate().is_err());
}
