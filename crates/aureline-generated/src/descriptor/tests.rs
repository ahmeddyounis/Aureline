use super::*;

fn generator() -> GeneratorIdentity {
    GeneratorIdentity {
        kind: GeneratorKind::Composer,
        name: "scoped-composer".to_owned(),
        version: "1.0.0".to_owned(),
    }
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_generated_artifact_descriptor_packet();
    validate_generated_artifact_descriptor_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_describes_every_class() {
    let packet = seeded_generated_artifact_descriptor_packet();
    assert_eq!(packet.descriptors.len(), ArtifactClass::ALL.len());
    let classes: BTreeSet<_> = packet
        .descriptors
        .iter()
        .map(|d| d.artifact_class)
        .collect();
    for required in ArtifactClass::ALL {
        assert!(classes.contains(&required), "missing class {required:?}");
    }
}

#[test]
fn canonical_authoritative_in_sync_presents_as_ordinary_source() {
    let presentation = derive_descriptor_presentation(
        ArtifactClass::ScaffoldedProject,
        AuthorityClass::CanonicalAuthoritative,
        &generator(),
        CanonicalSourceState::Linked,
        DriftState::InSync,
        EditPosture::DirectEditAllowed,
    );
    assert_eq!(
        presentation.presented_authority,
        PresentedAuthority::OrdinarySource
    );
    assert!(presentation.ordinary_source_claim_allowed);
    assert_eq!(
        presentation.effective_edit_posture,
        EditPosture::DirectEditAllowed
    );
    assert!(!presentation.edit_posture_downgraded);
    assert!(presentation.block_reason_tokens.is_empty());
}

#[test]
fn hidden_canonical_source_blocks_ordinary_source_claim() {
    // The marquee guardrail: a hidden canonical source can never be ordinary
    // source, and a direct-edit boundary narrows to a reviewed override.
    let presentation = derive_descriptor_presentation(
        ArtifactClass::ScaffoldedProject,
        AuthorityClass::CanonicalAuthoritative,
        &generator(),
        CanonicalSourceState::Hidden,
        DriftState::Unknown,
        EditPosture::DirectEditAllowed,
    );
    assert_eq!(
        presentation.presented_authority,
        PresentedAuthority::ProvenanceWithheld
    );
    assert!(!presentation.ordinary_source_claim_allowed);
    assert_eq!(
        presentation.effective_edit_posture,
        EditPosture::ReviewedOverrideRequired
    );
    assert!(presentation.edit_posture_downgraded);
    assert!(presentation
        .block_reason_tokens
        .contains(&"canonical_source_hidden".to_owned()));
}

#[test]
fn missing_canonical_source_blocks_ordinary_source_and_forces_regenerate_only() {
    let presentation = derive_descriptor_presentation(
        ArtifactClass::AiAssistedEdit,
        AuthorityClass::CanonicalAuthoritative,
        &generator(),
        CanonicalSourceState::Missing,
        DriftState::SourceMissing,
        EditPosture::DirectEditAllowed,
    );
    assert_eq!(
        presentation.presented_authority,
        PresentedAuthority::ProvenanceWithheld
    );
    assert!(!presentation.ordinary_source_claim_allowed);
    assert_eq!(
        presentation.effective_edit_posture,
        EditPosture::RegenerateOnly
    );
    assert_eq!(
        presentation.block_reason_tokens,
        vec![
            "canonical_source_missing".to_owned(),
            "drift_source_missing".to_owned(),
        ]
    );
}

#[test]
fn drifting_canonical_authoritative_withholds_and_downgrades() {
    let presentation = derive_descriptor_presentation(
        ArtifactClass::AiAssistedEdit,
        AuthorityClass::CanonicalAuthoritative,
        &generator(),
        CanonicalSourceState::Linked,
        DriftState::Drifting,
        EditPosture::DirectEditAllowed,
    );
    assert_eq!(
        presentation.presented_authority,
        PresentedAuthority::ProvenanceWithheld
    );
    assert!(!presentation.ordinary_source_claim_allowed);
    assert_eq!(
        presentation.effective_edit_posture,
        EditPosture::ReviewedOverrideRequired
    );
    assert!(presentation.edit_posture_downgraded);
    assert_eq!(
        presentation.block_reason_tokens,
        vec!["drift_drifting".to_owned()]
    );
}

#[test]
fn derived_linked_in_sync_presents_as_annotated_derived() {
    let presentation = derive_descriptor_presentation(
        ArtifactClass::NotebookOutput,
        AuthorityClass::DerivedReadonly,
        &generator(),
        CanonicalSourceState::Linked,
        DriftState::InSync,
        EditPosture::RegenerateOnly,
    );
    assert_eq!(
        presentation.presented_authority,
        PresentedAuthority::DerivedAnnotated
    );
    assert!(!presentation.ordinary_source_claim_allowed);
    assert_eq!(
        presentation.effective_edit_posture,
        EditPosture::RegenerateOnly
    );
}

#[test]
fn derived_drifting_stays_annotated_and_posture_does_not_widen() {
    // A regenerate-only artifact stays regenerate-only even when drift would
    // only impose a reviewed-override floor; narrowing never widens.
    let presentation = derive_descriptor_presentation(
        ArtifactClass::NotebookOutput,
        AuthorityClass::DerivedReadonly,
        &generator(),
        CanonicalSourceState::Linked,
        DriftState::Drifting,
        EditPosture::RegenerateOnly,
    );
    assert_eq!(
        presentation.presented_authority,
        PresentedAuthority::DerivedAnnotated
    );
    assert_eq!(
        presentation.effective_edit_posture,
        EditPosture::RegenerateOnly
    );
    assert!(!presentation.edit_posture_downgraded);
}

#[test]
fn unknown_drift_withholds_annotated_derived_presentation() {
    let presentation = derive_descriptor_presentation(
        ArtifactClass::FrameworkCodegen,
        AuthorityClass::DerivedEditable,
        &generator(),
        CanonicalSourceState::Linked,
        DriftState::Unknown,
        EditPosture::ReviewedOverrideRequired,
    );
    assert_eq!(
        presentation.presented_authority,
        PresentedAuthority::ProvenanceWithheld
    );
    assert_eq!(
        presentation.block_reason_tokens,
        vec!["drift_unknown".to_owned()]
    );
}

#[test]
fn every_surface_projects_identical_identity_fields() {
    let packet = seeded_generated_artifact_descriptor_packet();
    for descriptor in &packet.descriptors {
        let identity = descriptor.identity_fields();
        let projections = descriptor.project_all();
        assert_eq!(projections.len(), SurfaceKind::ALL.len());
        for projection in &projections {
            assert_eq!(
                projection.identity,
                identity,
                "surface {} drifted from the descriptor identity",
                projection.surface.as_str()
            );
            assert_eq!(projection.copy_line, descriptor.presentation.copy_line);
            assert!(!projection.badge.is_empty());
            assert!(!projection.headline.is_empty());
            assert!(!projection.detail.is_empty());
        }
    }
}

#[test]
fn copy_line_is_stable_and_self_consistent() {
    let descriptor = healthy_descriptor(ArtifactClass::FrameworkCodegen);
    let expected = "generated-artifact class=framework_codegen authority=derived_editable generator=framework/openapi-codegen@5.0.0 source=linked drift=in_sync presented=derived_annotated edit=reviewed_override_required ordinary_source=false";
    assert_eq!(descriptor.copy_line(), expected);
    assert_eq!(descriptor.presentation.copy_line, expected);
}

#[test]
fn seeded_fixtures_validate_and_cover_every_presentation() {
    let fixtures = seeded_generated_artifact_descriptor_fixtures();
    assert!(!fixtures.is_empty());
    let mut presentations = BTreeSet::new();
    let mut saw_blocked_ordinary_source = false;
    let mut saw_edit_downgrade = false;
    for fixture in &fixtures {
        validate_generated_artifact_descriptor_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        presentations.insert(fixture.expected_presented_authority);
        if fixture
            .descriptor
            .canonical_source
            .state
            .blocks_ordinary_source()
        {
            assert!(
                !fixture.expected_ordinary_source_claim_allowed,
                "fixture {} must block ordinary source with hidden/missing canonical source",
                fixture.fixture_id
            );
            saw_blocked_ordinary_source = true;
        }
        if fixture.descriptor.presentation.edit_posture_downgraded {
            saw_edit_downgrade = true;
        }
    }
    for required in [
        PresentedAuthority::OrdinarySource,
        PresentedAuthority::DerivedAnnotated,
        PresentedAuthority::ProvenanceWithheld,
    ] {
        assert!(
            presentations.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    assert!(
        saw_blocked_ordinary_source,
        "fixtures must cover a blocked ordinary-source claim"
    );
    assert!(
        saw_edit_downgrade,
        "fixtures must cover an edit-posture downgrade"
    );
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_generated_artifact_descriptor_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: GeneratedArtifactDescriptorPacket =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}

#[test]
fn fixtures_round_trip_through_json() {
    for fixture in seeded_generated_artifact_descriptor_fixtures() {
        let json = serde_json::to_string(&fixture).expect("fixture serializes");
        let back: GeneratedArtifactDescriptorFixture =
            serde_json::from_str(&json).expect("fixture deserializes");
        assert_eq!(fixture, back);
    }
}
