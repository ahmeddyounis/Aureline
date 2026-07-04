//! Tests for the M5 deployment-summary primitive: the resolver, the parity matrix, and
//! the checked-in support export.

use super::*;

// --- resolver: AC1 boundary honestly scoped ---

#[test]
fn resolver_preserves_deployment_identity_across_surfaces() {
    let input = self_hosted_admin_input();
    let resolved = resolve_deployment_summary(&input).expect("resolves");
    assert_eq!(resolved.deployment_id, input.deployment_id);
    assert_eq!(resolved.summary_card.deployment_id, input.deployment_id);
    assert_eq!(resolved.status_strip.deployment_id, input.deployment_id);
    assert!(resolved
        .residual_rows
        .iter()
        .all(|row| row.deployment_id == input.deployment_id));
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_keeps_self_hosted_boundary_honest_about_residual_dependency() {
    let resolved = resolve_deployment_summary(&self_hosted_admin_input()).expect("resolves");
    assert!(resolved.scope_claims_reduced_vendor_dependency());
    assert!(resolved.has_required_residual());
    assert!(resolved.boundary_not_overclaimed());
    assert!(resolved.summary_card.boundary_honestly_scoped);
    // The required residual dependency is named with its consequence and mitigation.
    let required = resolved
        .residual_rows
        .iter()
        .find(|row| row.required_for_operation)
        .expect("a required residual row exists");
    assert_eq!(
        required.dependency_class,
        M5ResidualDependencyClass::LicenseActivation
    );
    assert_eq!(
        required.failure_consequence,
        M5ResidualFailureConsequence::BlocksActivation
    );
    assert!(required.names_failure_and_path);
}

#[test]
fn resolver_rejects_overclaimed_boundary() {
    let mut input = self_hosted_admin_input();
    // A self-hosted boundary that hides a required residual vendor dependency overclaims.
    input.residual_dependencies[0].disclosed = false;
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::BoundaryOverclaimed)
    );
}

#[test]
fn resolver_rejects_undisclosed_optional_residual_on_managed_scope() {
    let mut input = shared_managed_about_input();
    // A shared-managed scope does not claim a reduced boundary, so an undisclosed
    // residual row is a plain disclosure failure rather than a boundary overclaim.
    input.residual_dependencies[1].disclosed = false;
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::ResidualDependencyUndisclosed)
    );
}

// --- resolver: AC2 control-plane distinct from local runtime ---

#[test]
fn resolver_keeps_control_plane_distinct_from_local_runtime() {
    let resolved = resolve_deployment_summary(&self_hosted_admin_input()).expect("resolves");
    assert!(resolved.status_strip.control_plane_state.is_impaired());
    assert!(!resolved.status_strip.local_runtime_impaired);
    assert!(resolved.status_strip.planes_distinct);
    assert!(resolved
        .status_strip
        .control_impairment_not_masked_as_local);
    assert!(resolved.status_strip.local_safe_next_step_visible);
    assert!(resolved.planes_distinguishable());
}

#[test]
fn resolver_rejects_control_plane_masked_as_local() {
    let mut input = self_hosted_admin_input();
    input.control_plane_impairment_flagged_as_local = true;
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::ControlPlaneMaskedAsLocal)
    );
}

#[test]
fn resolver_keeps_local_safe_next_step_visible() {
    let resolved = resolve_deployment_summary(&sovereign_service_health_input()).expect("resolves");
    assert_eq!(
        resolved.status_strip.control_plane_state,
        M5PlaneState::Unavailable
    );
    assert_eq!(
        resolved.status_strip.data_plane_state,
        M5PlaneState::Operational
    );
    assert_eq!(
        resolved.status_strip.local_safe_next_step,
        M5LocalSafeNextStep::WorkOfflineCached
    );
    assert!(resolved.status_strip.local_safe_next_step_visible);
}

// --- resolver: AC3 residual dependency explicit and exportable ---

#[test]
fn resolver_makes_residual_dependency_exportable() {
    let resolved = resolve_deployment_summary(&shared_managed_about_input()).expect("resolves");
    assert!(!resolved.residual_rows.is_empty());
    assert!(resolved.residual_dependency_exportable());
    assert!(resolved.residual_rows.iter().all(|row| row.exportable));
    assert!(resolved.residual_rows.iter().all(|row| row.disclosed));
}

#[test]
fn resolver_allows_local_only_with_no_residual() {
    let resolved = resolve_deployment_summary(&local_only_docs_input()).expect("resolves");
    assert!(resolved.residual_rows.is_empty());
    assert!(!resolved.has_required_residual());
    // A truly local-only boundary with no residual dependency is not an overclaim.
    assert!(resolved.boundary_not_overclaimed());
    assert!(resolved.residual_dependency_exportable());
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_empty_deployment_id() {
    let input = M5DeploymentSummaryInput {
        deployment_id: "  ".to_owned(),
        ..shared_managed_about_input()
    };
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::EmptyDeploymentId)
    );
}

#[test]
fn resolver_rejects_empty_tenant_ref() {
    let input = M5DeploymentSummaryInput {
        tenant_org_ref: String::new(),
        ..shared_managed_about_input()
    };
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::EmptyTenantOrgRef)
    );
}

#[test]
fn resolver_rejects_empty_region_ref() {
    let input = M5DeploymentSummaryInput {
        region_ref: "  ".to_owned(),
        ..shared_managed_about_input()
    };
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::EmptyRegionRef)
    );
}

#[test]
fn resolver_rejects_empty_sync_ref() {
    let input = M5DeploymentSummaryInput {
        last_control_plane_sync_ref: String::new(),
        ..shared_managed_about_input()
    };
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::EmptySyncRef)
    );
}

#[test]
fn resolver_rejects_empty_residual_ref() {
    let mut input = shared_managed_about_input();
    input.residual_dependencies[0].vendor_dependency_ref = "   ".to_owned();
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::EmptyResidualRef)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5DeploymentSummaryInput {
        last_control_plane_sync_ref: "sync://control-plane".to_owned(),
        ..shared_managed_about_input()
    };
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5DeploymentSummaryInput {
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
            degraded_label: "unavailable".to_owned(),
        }),
        ..shared_managed_about_input()
    };
    assert_eq!(
        resolve_deployment_summary(&input),
        Err(M5DeploymentSummaryResolutionError::DegradedLabelGeneric)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_deployment_summary_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_deployment_summary_packet();
    let present: BTreeSet<M5DeploymentSummarySurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5DeploymentSummarySurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_deployment_summary_packet();
    for row in &packet.surface_rows {
        for case in &row.example_summaries {
            assert!(
                case.is_self_consistent(),
                "case drifted on {:?}",
                row.surface_family
            );
        }
    }
}

#[test]
fn vocabulary_set_matches_canonical() {
    assert!(M5DeploymentSummaryVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_deployment_summary_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_deployment_summary_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentSummaryViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_deployment_summary_packet();
    packet.surface_rows[0].masks_control_plane_as_local = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentSummaryViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_deployment_summary_packet();
    packet.surface_rows[0].example_summaries[0]
        .resolved
        .boundary_not_overclaimed = !packet.surface_rows[0].example_summaries[0]
        .resolved
        .boundary_not_overclaimed;
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentSummaryViolation::ExampleSummaryDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_deployment_summary_packet();
    packet
        .vocabulary_set
        .deployment_scopes
        .push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentSummaryViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_deployment_summary_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5DeploymentSummaryExportField::ResidualDependencies);
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentSummaryViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_deployment_summary_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_deployment_summary_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_deployment_summary_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-deployment-summary-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_deployment_summary_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_deployment_summary_packet();
    assert_eq!(packet.record_kind, M5_DEPLOYMENT_SUMMARY_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_DEPLOYMENT_SUMMARY_SCHEMA_VERSION);
}
