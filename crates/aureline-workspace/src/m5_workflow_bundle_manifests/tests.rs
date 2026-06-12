use super::*;

fn packet() -> M5WorkflowBundleManifestsPacket {
    current_m5_workflow_bundle_manifests_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_WORKFLOW_BUNDLE_MANIFESTS_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_WORKFLOW_BUNDLE_MANIFESTS_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_matches_recomputed() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_wedge_has_a_manifest() {
    // Every claimed M5 launch wedge is composed as a real workflow bundle, not an ad hoc pile.
    let packet = packet();
    assert!(packet.covers_every_wedge());
    for wedge in M5Wedge::ALL {
        assert!(
            packet.manifests_for_wedge(wedge).next().is_some(),
            "wedge {} has no manifest",
            wedge.as_str()
        );
    }
}

#[test]
fn every_certification_target_is_exercised() {
    let packet = packet();
    for target in CertificationTarget::ALL {
        assert!(
            packet.manifests_with_target(target).next().is_some(),
            "certification target {} is not exercised",
            target.as_str()
        );
    }
}

#[test]
fn every_component_kind_is_exercised() {
    let packet = packet();
    for kind in BundleComponentKind::ALL {
        let exercised = packet
            .manifests
            .iter()
            .flat_map(|m| m.components.iter())
            .any(|c| c.component_kind == kind);
        assert!(
            exercised,
            "component kind {} is not exercised",
            kind.as_str()
        );
    }
}

#[test]
fn every_lifecycle_stage_is_exercised() {
    let packet = packet();
    for stage in LifecycleStage::ALL {
        let exercised = packet
            .manifests
            .iter()
            .flat_map(|m| m.components.iter())
            .any(|c| c.lifecycle_stage == stage);
        assert!(
            exercised,
            "lifecycle stage {} is not exercised",
            stage.as_str()
        );
    }
}

#[test]
fn every_manifest_is_diffable_and_never_opaque() {
    // Opaque binary bundle state is forbidden on these claimed paths.
    let packet = packet();
    for m in &packet.manifests {
        assert!(
            m.guards_correct(),
            "manifest {} is opaque or non-diffable",
            m.bundle_id
        );
        assert!(m.diffable && m.mirrorable && m.export_safe);
        assert!(!m.opaque_binary_state);
        for c in &m.components {
            assert!(c.diffable, "component {} is opaque", c.component_id);
        }
    }
}

#[test]
fn every_manifest_captures_a_minimum_cohesive_experience() {
    let packet = packet();
    for m in &packet.manifests {
        assert!(
            m.has_minimum_cohesive_experience(),
            "manifest {} lacks an extension set or runnable recipe",
            m.bundle_id
        );
    }
}

#[test]
fn non_stable_dependencies_are_always_disclosed() {
    // A bundle may depend on a non-stable capability, but never hides it.
    let packet = packet();
    for m in &packet.manifests {
        assert!(
            m.disclosure_consistent(),
            "manifest {} hides or mislabels a non-stable dependency",
            m.bundle_id
        );
        assert_eq!(
            m.discloses_non_stable_dependencies,
            m.has_non_stable_components(),
            "manifest {} disclosure flag diverges from its components",
            m.bundle_id
        );
        assert_eq!(m.dependency_markers, m.computed_dependency_markers());
        for c in &m.components {
            if c.lifecycle_stage.is_non_stable() {
                assert!(
                    c.requires_review,
                    "non-stable component {} must be review-gated",
                    c.component_id
                );
            }
        }
    }
    // At least one bundle exercises a disclosed non-stable dependency.
    assert!(packet
        .manifests
        .iter()
        .any(|m| m.has_non_stable_components()));
}

#[test]
fn migration_mappings_carry_from_and_to() {
    let packet = packet();
    for m in &packet.manifests {
        for c in m.components_of_kind(BundleComponentKind::MigrationMapping) {
            assert!(c.migration_fields_consistent());
            assert!(c.migration_from.as_deref().is_some_and(|s| !s.is_empty()));
            assert!(c.migration_to.as_deref().is_some_and(|s| !s.is_empty()));
        }
        for c in m
            .components
            .iter()
            .filter(|c| c.component_kind != BundleComponentKind::MigrationMapping)
        {
            assert!(c.migration_from.is_none() && c.migration_to.is_none());
        }
    }
}

#[test]
fn draft_manifests_only_claim_local_draft() {
    let packet = packet();
    for m in &packet.manifests {
        assert!(
            m.target_within_publication(),
            "manifest {} certification target exceeds its publication state",
            m.bundle_id
        );
    }
}

#[test]
fn only_certified_targets_present_as_certified() {
    let packet = packet();
    for m in &packet.manifests {
        assert_eq!(
            m.presents_as_certified(),
            m.certification_target == CertificationTarget::Certified,
            "manifest {} certified presentation diverges from its target",
            m.bundle_id
        );
    }
}

#[test]
fn every_manifest_carries_provenance() {
    let packet = packet();
    for m in &packet.manifests {
        assert!(
            m.provenance_complete(),
            "manifest {} lacks manifest, certification, or migration provenance",
            m.bundle_id
        );
    }
}

#[test]
fn export_projection_reflects_manifests() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.manifests.len(), packet.manifests.len());
    assert!(projection.all_manifests_consistent);
    assert_eq!(
        projection.certified_target_manifests,
        packet.summary.certified_target_manifests
    );
    for (row, m) in projection.manifests.iter().zip(&packet.manifests) {
        assert_eq!(row.bundle_id, m.bundle_id);
        assert_eq!(row.certification_target, m.certification_target);
        assert_eq!(row.dependency_markers, m.dependency_markers);
    }
}

#[test]
fn all_manifests_consistent_holds() {
    assert!(packet().all_manifests_consistent());
}

#[test]
fn validate_flags_an_undisclosed_non_stable_dependency() {
    let mut packet = packet();
    let m = packet
        .manifests
        .iter_mut()
        .find(|m| m.has_non_stable_components())
        .expect("a manifest with a non-stable dependency exists");
    m.discloses_non_stable_dependencies = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5WorkflowBundleManifestsViolation::UndisclosedNonStableDependency { .. }
    )));
}

#[test]
fn validate_flags_opaque_binary_state() {
    let mut packet = packet();
    packet.manifests[0].opaque_binary_state = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5WorkflowBundleManifestsViolation::OpaqueOrNonDiffable { .. }
    )));
}

#[test]
fn validate_flags_a_draft_claiming_certified() {
    let mut packet = packet();
    let m = packet
        .manifests
        .iter_mut()
        .find(|m| m.publication_state == ManifestPublicationState::Draft)
        .expect("a draft manifest exists");
    m.certification_target = CertificationTarget::Certified;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5WorkflowBundleManifestsViolation::TargetExceedsPublication { .. }
    )));
}

#[test]
fn validate_flags_a_non_cohesive_bundle() {
    let mut packet = packet();
    packet.manifests[0]
        .components
        .retain(|c| c.component_kind != BundleComponentKind::Extension);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5WorkflowBundleManifestsViolation::NotCohesive { .. })));
}

#[test]
fn validate_flags_missing_wedge_coverage() {
    let mut packet = packet();
    packet
        .manifests
        .retain(|m| m.wedge != M5Wedge::NotebookWorkspace);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5WorkflowBundleManifestsViolation::MissingWedgeCoverage {
            wedge: M5Wedge::NotebookWorkspace
        }
    )));
}

#[test]
fn constants_point_at_checked_in_paths() {
    assert_eq!(
        M5_WORKFLOW_BUNDLE_MANIFESTS_PATH,
        "artifacts/workspace/m5/m5-workflow-bundle-manifests.json"
    );
    assert_eq!(
        M5_WORKFLOW_BUNDLE_MANIFESTS_SCHEMA_REF,
        "schemas/workspace/m5-workflow-bundle-manifests.schema.json"
    );
    assert_eq!(
        M5_WORKFLOW_BUNDLE_MANIFESTS_DOC_REF,
        "docs/workspace/m5/m5-workflow-bundle-manifests.md"
    );
}
