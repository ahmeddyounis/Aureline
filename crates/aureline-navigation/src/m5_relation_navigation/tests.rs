//! Unit tests for the relation-navigation matrix builder, invariants, controlled
//! vocabulary, and export-safety rules.

use super::*;
use crate::target_model::{
    AmbiguityClass, FreshnessClass, GeneratedOrExternalState, ScopeCompleteness,
};

#[test]
fn matrix_validates_and_all_invariants_hold() {
    let matrix = relation_navigation_matrix();
    matrix.validate().expect("canonical matrix validates");
    assert!(matrix.all_invariants_hold());
    assert!(!matrix.invariants.is_empty());
}

#[test]
fn matrix_is_deterministic() {
    assert_eq!(relation_navigation_matrix(), relation_navigation_matrix());
}

#[test]
fn matrix_is_support_export_safe() {
    let matrix = relation_navigation_matrix();
    assert!(matrix.raw_payload_excluded);
    assert!(matrix.is_support_export_safe());
}

#[test]
fn every_object_family_is_present_once() {
    let matrix = relation_navigation_matrix();
    assert_eq!(matrix.objects.len(), RelationNavObjectClass::ALL.len());
    for class in RelationNavObjectClass::ALL {
        let entry = matrix.object(class).expect("object present");
        assert_eq!(entry.object_id, class.object_id());
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.proof_packet_ref.is_empty());
        assert!(!entry.applicable_states.is_empty());
        assert!(!entry.controlled_vocabularies.is_empty());
        assert!(!entry.relation_kinds.is_empty());
        assert!(entry.required_fields.iter().any(|f| f.required));
        assert!(entry.proof_class_required);
        assert!(entry.binds(RelationNavVocabulary::ProofClassAxis));
    }
}

#[test]
fn state_vocabulary_is_complete_and_unique() {
    let matrix = relation_navigation_matrix();
    assert_eq!(
        matrix.state_vocabulary.len(),
        RelationNavStateClass::ALL.len()
    );
    for state in RelationNavStateClass::ALL {
        let term = matrix.state_term(state).expect("state present");
        assert_eq!(term.token, state.as_str());
    }
    assert!(all_unique(
        matrix.state_vocabulary.iter().map(|t| t.token.as_str())
    ));
}

#[test]
fn definition_is_not_declaration() {
    let matrix = relation_navigation_matrix();
    assert_ne!(
        RelationKind::Definition.as_str(),
        RelationKind::Declaration.as_str()
    );
    let target = matrix
        .object(RelationNavObjectClass::NavigationTarget)
        .expect("navigation target present");
    assert!(target.represents(RelationKind::Definition));
    assert!(target.represents(RelationKind::Declaration));
}

#[test]
fn fallback_states_always_require_disclosure() {
    let matrix = relation_navigation_matrix();
    for term in &matrix.state_vocabulary {
        if term.is_fallback_proof {
            assert!(
                term.requires_disclosure,
                "fallback state {} must require disclosure",
                term.token
            );
        }
    }
    // Only exact and indexed semantic proof may render without a caveat.
    let no_disclosure: Vec<&str> = matrix
        .state_vocabulary
        .iter()
        .filter(|t| !t.requires_disclosure)
        .map(|t| t.token.as_str())
        .collect();
    assert_eq!(no_disclosure, vec!["exact_semantic", "indexed_semantic"]);
}

#[test]
fn navigable_objects_that_show_fallback_bind_proof_class() {
    let matrix = relation_navigation_matrix();
    for entry in matrix
        .objects
        .iter()
        .filter(|o| o.object.is_navigable_object())
    {
        if entry
            .applicable_states
            .iter()
            .any(|s| s.is_fallback_proof())
        {
            assert!(
                entry.binds(RelationNavVocabulary::ProofClassAxis),
                "object {} shows a fallback state but does not bind proof_class",
                entry.object.as_str()
            );
        }
    }
}

#[test]
fn hierarchy_edge_preserves_proof_and_ambiguity() {
    let matrix = relation_navigation_matrix();
    let edge = matrix
        .object(RelationNavObjectClass::HierarchyEdge)
        .expect("hierarchy edge present");
    assert!(edge.binds(RelationNavVocabulary::ProofClassAxis));
    assert!(edge.binds(RelationNavVocabulary::Ambiguity));
    assert!(edge.can_show(RelationNavStateClass::FrameworkDerivedDisclosed));
    assert!(edge.can_show(RelationNavStateClass::RuntimeObservedDisclosed));
    assert!(edge.can_show(RelationNavStateClass::AmbiguousNeedsSelection));
}

#[test]
fn related_object_relation_is_source_attributed() {
    let matrix = relation_navigation_matrix();
    let related = matrix
        .object(RelationNavObjectClass::RelatedObjectRelation)
        .expect("related-object relation present");
    assert!(related.carries_source_attribution);
    assert!(related.source_attribution_field.is_some());
    assert!(related.binds(RelationNavVocabulary::ProofClassAxis));
}

#[test]
fn rename_preview_exposes_blocked_and_partial_candidates() {
    let matrix = relation_navigation_matrix();
    let rename = matrix
        .object(RelationNavObjectClass::RenamePreviewSet)
        .expect("rename-preview set present");
    assert!(rename.binds(RelationNavVocabulary::RenameOmissionReason));
    assert!(rename.binds(RelationNavVocabulary::GeneratedRuntimeLabel));
    assert!(rename.can_show(RelationNavStateClass::RenameBlockedPendingReview));
    assert!(rename.can_show(RelationNavStateClass::GeneratedBoundaryDisclosed));
    assert!(rename.can_show(RelationNavStateClass::ReadOnlyProtected));
    assert!(rename.can_show(RelationNavStateClass::PartialScope));
    for f in ["blocked_refs", "generated_scope_notes", "count_summary"] {
        assert!(
            rename.required_fields.iter().any(|rf| rf.field_id == f),
            "rename preview must declare field {f}"
        );
    }
}

#[test]
fn relation_fallback_vocabulary_enumerates_all_relation_kinds() {
    let matrix = relation_navigation_matrix();
    let vocab = matrix
        .object(RelationNavObjectClass::RelationFallbackVocabulary)
        .expect("relation/fallback vocabulary present");
    assert_eq!(vocab.relation_kinds.len(), REQUIRED_RELATION_KINDS.len());
    for kind in REQUIRED_RELATION_KINDS {
        assert!(vocab.represents(kind));
    }
}

#[test]
fn every_controlled_vocabulary_axis_is_bound() {
    let matrix = relation_navigation_matrix();
    for axis in RelationNavVocabulary::ALL {
        assert!(
            matrix.objects.iter().any(|o| o.binds(axis)),
            "controlled vocabulary {} is bound by no object",
            axis.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_renders_some_object() {
    let matrix = relation_navigation_matrix();
    for consumer in RelationNavConsumer::ALL {
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

/// The controlled-vocabulary tokens the matrix publishes must equal the serde
/// tokens the upstream `target_model` enums serialize to, so the matrix can never
/// silently diverge from the object model it governs.
#[test]
fn published_vocab_tokens_match_target_model_serde_tokens() {
    let matrix = relation_navigation_matrix();
    let vocab = &matrix.shared_vocabulary;

    fn token<T: serde::Serialize>(value: &T) -> String {
        serde_json::to_value(value)
            .expect("serializes")
            .as_str()
            .expect("enum serializes to a string")
            .to_owned()
    }

    // Relation kinds.
    let got: Vec<&str> = vocab
        .relation_kinds
        .iter()
        .map(|d| d.token.as_str())
        .collect();
    let want: Vec<String> = REQUIRED_RELATION_KINDS.iter().map(token).collect();
    assert_eq!(got, want.iter().map(String::as_str).collect::<Vec<_>>());

    // Proof classes.
    let got: Vec<&str> = vocab
        .proof_classes
        .iter()
        .map(|d| d.token.as_str())
        .collect();
    let want: Vec<String> = PROOF_CLASS_ORDER.iter().map(token).collect();
    assert_eq!(got, want.iter().map(String::as_str).collect::<Vec<_>>());

    // Access kinds.
    let got: Vec<&str> = vocab
        .access_kinds
        .iter()
        .map(|d| d.token.as_str())
        .collect();
    let want: Vec<String> = ACCESS_KIND_ORDER.iter().map(token).collect();
    assert_eq!(got, want.iter().map(String::as_str).collect::<Vec<_>>());

    // Ambiguity classes.
    let want: Vec<String> = [
        AmbiguityClass::Unambiguous,
        AmbiguityClass::AmbiguousNeedsSelection,
        AmbiguityClass::MultipleCandidatesRanked,
        AmbiguityClass::DriftedNeedsReview,
        AmbiguityClass::MissingTarget,
        AmbiguityClass::ScopeUnavailable,
    ]
    .iter()
    .map(token)
    .collect();
    let got: Vec<&str> = vocab
        .ambiguity_classes
        .iter()
        .map(|d| d.token.as_str())
        .collect();
    assert_eq!(got, want.iter().map(String::as_str).collect::<Vec<_>>());

    // Freshness classes.
    let want: Vec<String> = [
        FreshnessClass::AuthoritativeLive,
        FreshnessClass::WarmCached,
        FreshnessClass::DegradedCached,
        FreshnessClass::Stale,
        FreshnessClass::Unverified,
    ]
    .iter()
    .map(token)
    .collect();
    let got: Vec<&str> = vocab
        .freshness_classes
        .iter()
        .map(|d| d.token.as_str())
        .collect();
    assert_eq!(got, want.iter().map(String::as_str).collect::<Vec<_>>());

    // Partiality / scope-completeness classes.
    let want: Vec<String> = [
        ScopeCompleteness::CompleteForDeclaredScope,
        ScopeCompleteness::PartialForDeclaredScope,
        ScopeCompleteness::StaleForDeclaredScope,
        ScopeCompleteness::UnavailableForDeclaredScope,
    ]
    .iter()
    .map(token)
    .collect();
    let got: Vec<&str> = vocab
        .partiality_classes
        .iter()
        .map(|d| d.token.as_str())
        .collect();
    assert_eq!(got, want.iter().map(String::as_str).collect::<Vec<_>>());

    // Generated / runtime / external labels.
    let want: Vec<String> = [
        GeneratedOrExternalState::AuthoredSource,
        GeneratedOrExternalState::GeneratedSource,
        GeneratedOrExternalState::ExternalDependency,
        GeneratedOrExternalState::ReadOnlySource,
        GeneratedOrExternalState::ImportedSnapshot,
    ]
    .iter()
    .map(token)
    .collect();
    let got: Vec<&str> = vocab
        .generated_runtime_labels
        .iter()
        .map(|d| d.token.as_str())
        .collect();
    assert_eq!(got, want.iter().map(String::as_str).collect::<Vec<_>>());
}

#[test]
fn lines_projection_is_non_empty_and_lists_objects() {
    let matrix = relation_navigation_matrix();
    let lines = relation_navigation_lines(&matrix);
    assert!(lines
        .iter()
        .any(|l| l.contains("Relation-navigation matrix")));
    for class in RelationNavObjectClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(class.as_str())),
            "projection should mention {}",
            class.as_str()
        );
    }
}

#[test]
fn roundtrips_through_json() {
    let matrix = relation_navigation_matrix();
    let json = serde_json::to_string(&matrix).expect("serializes");
    let back: RelationNavigationMatrix = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(matrix, back);
}

#[test]
fn validation_rejects_a_dropped_proof_packet() {
    let mut matrix = relation_navigation_matrix();
    matrix.objects[0].proof_packet_ref.clear();
    // Recompute the invariant that watches proof packets so the tampered record is
    // internally consistent about its own failure, then confirm validate() rejects.
    let err = matrix
        .validate()
        .expect_err("must reject a dropped proof packet");
    assert!(err.to_string().contains("proof packet") || err.to_string().contains("not support"));
}
