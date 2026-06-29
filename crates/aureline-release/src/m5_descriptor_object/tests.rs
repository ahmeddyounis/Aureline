//! Inline tests for the M5 descriptor-object lane.

use super::*;

fn registry() -> M5DescriptorObjectRegistry {
    seeded_m5_descriptor_object_registry()
}

#[test]
fn canonical_registry_validates() {
    let registry = registry();
    assert!(registry.validate().is_empty(), "{:?}", registry.validate());
    assert_eq!(registry.registry_id, M5_DESCRIPTOR_OBJECT_REGISTRY_ID);
    assert_eq!(
        registry.record_kind,
        M5_DESCRIPTOR_OBJECT_REGISTRY_RECORD_KIND
    );
    assert_eq!(registry.objects.len(), 3);
    assert!(registry.conformance.all_hold());
    assert!(registry.vocabulary.matches_canonical());
}

#[test]
fn every_object_validates() {
    for object in registry().objects {
        assert_eq!(object.record_kind, M5_DESCRIPTOR_OBJECT_RECORD_KIND);
        assert!(object.validate().is_empty(), "{:?}", object.validate());
    }
}

#[test]
fn controlled_enums_freeze_every_facet_vocabulary() {
    let vocab = DescriptorObjectVocabulary::canonical();
    // Every facet vocabulary is non-empty and carries its frozen token count.
    assert_eq!(vocab.source_classes.len(), ProvenanceClass::ALL.len());
    assert_eq!(vocab.signature_states.len(), SignatureState::ALL.len());
    assert_eq!(vocab.freshness_states.len(), FreshnessState::ALL.len());
    assert_eq!(vocab.evidence_states.len(), EvidenceState::ALL.len());
    assert_eq!(vocab.support_classes.len(), QualificationClass::ALL.len());
    assert_eq!(vocab.client_kinds.len(), ClientScope::ALL.len());
    assert_eq!(vocab.authority_classes.len(), AuthorityClass::ALL.len());
    assert_eq!(
        vocab.handoff_requirements.len(),
        HandoffRequirement::ALL.len()
    );
    assert_eq!(vocab.facets.len(), DescriptorFacet::ALL.len());
}

#[test]
fn partial_evidence_states_are_first_class_tokens() {
    let states: Vec<&str> = EvidenceState::ALL.iter().map(|s| s.as_str()).collect();
    for needle in [
        "not_provided",
        "partial",
        "evidence_stale",
        "retest_pending",
        "limited",
    ] {
        assert!(
            states.contains(&needle),
            "evidence vocabulary dropped `{needle}`"
        );
    }
}

#[test]
fn signature_authority_handoff_enums_present() {
    // Signature/attestation state.
    assert!(SignatureState::ALL
        .iter()
        .any(|s| matches!(s, SignatureState::SignatureInvalid)));
    // Authority class.
    assert!(AuthorityClass::ALL
        .iter()
        .any(|a| matches!(a, AuthorityClass::FullAuthority)));
    // Handoff requirement.
    assert!(HandoffRequirement::ALL
        .iter()
        .any(|h| matches!(h, HandoffRequirement::DesktopHandoffRequired)));
}

#[test]
fn clean_object_stands_at_stable() {
    let object = seeded_stable_descriptor_object();
    assert!(object.narrowings.is_empty());
    assert!(object.is_stable());
    assert!(!object.blocks_stable_promotion());
    assert_eq!(object.effective_qualification, QualificationClass::Stable);
}

#[test]
fn weaker_but_present_evidence_auto_narrows_to_beta() {
    let object = seeded_narrowed_descriptor_object();
    assert!(!object.is_stable());
    assert!(!object.blocks_stable_promotion());
    assert_eq!(object.effective_qualification, QualificationClass::Beta);
    // Every weaker value is named, never omitted.
    let facets: Vec<DescriptorFacet> = object.narrowings.iter().map(|n| n.facet).collect();
    for facet in [
        DescriptorFacet::SourceClass,
        DescriptorFacet::SignatureState,
        DescriptorFacet::FreshnessState,
        DescriptorFacet::FreshnessEvidence,
        DescriptorFacet::QualificationEvidence,
        DescriptorFacet::ClientKind,
        DescriptorFacet::AuthorityClass,
        DescriptorFacet::HandoffRequirement,
    ] {
        assert!(facets.contains(&facet), "missing narrowing for {facet:?}");
    }
    assert!(object
        .narrowings
        .iter()
        .all(|n| matches!(n.effect, DowngradeEffect::Narrow)));
}

#[test]
fn absent_provenance_and_evidence_block_stable() {
    let object = seeded_not_provided_descriptor_object();
    assert!(object.blocks_stable_promotion());
    assert_eq!(
        object.effective_qualification,
        QualificationClass::Unavailable
    );
    // The absent values survive as explicit blocking narrowings.
    let blockers: Vec<&DescriptorNarrowing> = object
        .narrowings
        .iter()
        .filter(|n| matches!(n.effect, DowngradeEffect::Block))
        .collect();
    assert!(blockers
        .iter()
        .any(|n| n.facet == DescriptorFacet::SourceClass && n.token == "not_provided"));
    assert!(blockers
        .iter()
        .any(|n| n.facet == DescriptorFacet::FreshnessState && n.token == "missing"));
    assert!(blockers
        .iter()
        .any(|n| n.facet == DescriptorFacet::FreshnessEvidence && n.token == "not_provided"));
}

#[test]
fn identity_and_binding_survive_round_trip() {
    for object in registry().objects {
        let json = object.export_safe_json();
        let parsed: DescriptorObject = serde_json::from_str(&json).expect("object deserializes");
        assert_eq!(parsed, object);
        // The binding stays structured — not flattened to a single string.
        assert_eq!(parsed.descriptor_id, object.descriptor_id);
        assert_eq!(parsed.artifact_ref, object.artifact_ref);
        assert!(!parsed.artifact_ref.artifact_id.is_empty());
        assert!(!parsed.artifact_ref.content_digest_ref.is_empty());
    }
}

#[test]
fn registry_round_trips() {
    let registry = registry();
    let json = registry.export_safe_json();
    let parsed: M5DescriptorObjectRegistry =
        serde_json::from_str(&json).expect("registry deserializes");
    assert_eq!(parsed, registry);
    assert!(parsed.validate().is_empty());
}

#[test]
fn diff_names_changed_facets() {
    let stable = seeded_stable_descriptor_object();
    let narrowed = seeded_narrowed_descriptor_object();
    let diff = stable.diff(&narrowed);
    let facets: Vec<&str> = diff.iter().map(|d| d.facet.as_str()).collect();
    assert!(facets.contains(&"source_class"));
    assert!(facets.contains(&"signature_state"));
    assert!(facets.contains(&"effective_qualification"));
    // No change between an object and itself.
    assert!(stable.diff(&stable).is_empty());
}

#[test]
fn tampered_effective_qualification_is_rejected() {
    let mut object = seeded_narrowed_descriptor_object();
    object.effective_qualification = QualificationClass::Stable;
    assert!(object
        .validate()
        .contains(&M5DescriptorObjectViolation::EffectiveQualificationDrift));
}

#[test]
fn tampered_narrowings_are_rejected() {
    let mut object = seeded_narrowed_descriptor_object();
    object.narrowings.clear();
    let violations = object.validate();
    assert!(violations.contains(&M5DescriptorObjectViolation::NarrowingDrift));
}

#[test]
fn missing_artifact_binding_is_rejected() {
    let mut object = seeded_stable_descriptor_object();
    object.artifact_ref.artifact_id = String::new();
    assert!(object
        .validate()
        .contains(&M5DescriptorObjectViolation::MissingArtifactBinding));
}

#[test]
fn markdown_render_names_objects_and_narrowings() {
    let md = registry().render_markdown_summary();
    assert!(md.contains("# M5 public-truth descriptor objects"));
    assert!(md.contains("stable"));
    assert!(md.contains("→ `beta`") || md.contains("`beta`"));
    assert!(md.contains("`unavailable`"));
    // The blocked object's absent provenance is rendered, never hidden.
    assert!(md.contains("not_provided"));
}

#[test]
fn registry_consumes_one_runtime_across_consumers() {
    let registry = registry();
    let expected: Vec<String> = PublicTruthConsumer::ALL
        .iter()
        .map(|c| c.as_str().to_owned())
        .collect();
    assert_eq!(registry.consumers, expected);
    assert!(registry.conformance.shared_across_consumers);
}
