use super::*;

fn packet() -> DeploymentContinuityComponentMatrix {
    seeded_deployment_continuity_component_matrix()
}

fn row_mut<'a>(
    packet: &'a mut DeploymentContinuityComponentMatrix,
    component_id: &str,
) -> &'a mut ComponentRow {
    packet
        .components
        .iter_mut()
        .find(|r| r.component_id == component_id)
        .unwrap_or_else(|| panic!("component {component_id}"))
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_family_is_defined() {
    let families = packet().represented_families();
    for family in M5DeploymentComponentFamily::ALL {
        assert!(
            families.contains(&family),
            "missing family: {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_carries_degraded_rows() {
    assert!(packet().degraded_row_count() >= 1);
}

#[test]
fn payload_is_present_only_for_its_family() {
    for row in &packet().components {
        assert!(
            row.payload_matches_family(),
            "payload mismatch for {}",
            row.component_id
        );
    }
}

#[test]
fn missing_family_fails() {
    let mut packet = packet();
    packet
        .components
        .retain(|r| r.family != M5DeploymentComponentFamily::MirrorOfflineArtifactRow);
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::RequiredFamilyMissing));
}

#[test]
fn no_degraded_row_fails() {
    let mut packet = packet();
    for row in &mut packet.components {
        row.degraded = None;
    }
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::DegradedCaseMissing));
}

#[test]
fn wrong_payload_for_family_fails() {
    let mut packet = packet();
    // Attach a stray install-profile-card payload to a rollout-ring row.
    let row = row_mut(&mut packet, "component:rollout-ring-row:0001");
    row.install_profile_card = Some(InstallProfileCardDescriptor {
        install_id_ref: "install:leak:0001".to_owned(),
        install_mode: M5DeploymentMode::Managed,
        channel_ref: "channel:stable".to_owned(),
        updater_owner_ref: "updater_owner:managed_admin".to_owned(),
        state_root_ref: "state_root:leak:0001".to_owned(),
        discloses_state_roots: true,
        discloses_updater_owner: true,
    });
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadFamilyMismatch));
}

#[test]
fn install_card_hiding_state_roots_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:install-profile-card:0001")
        .install_profile_card
        .as_mut()
        .expect("card")
        .discloses_state_roots = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn install_card_disclosing_different_mode_than_row_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:install-profile-card:0001")
        .install_profile_card
        .as_mut()
        .expect("card")
        .install_mode = M5DeploymentMode::Desktop;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::DescriptorRowMismatch));
}

#[test]
fn side_by_side_capturing_handler_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:side-by-side-import-sheet:0001")
        .side_by_side_import_sheet
        .as_mut()
        .expect("sheet")
        .last_writer_wins_capture = true;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn channel_association_capturing_handler_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:channel-association-review-row:0001")
        .channel_association_review_row
        .as_mut()
        .expect("row")
        .last_writer_wins_capture = true;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn channel_association_without_review_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:channel-association-review-row:0001")
        .channel_association_review_row
        .as_mut()
        .expect("row")
        .reviewed_before_apply = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn rollout_ring_hiding_ring_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:rollout-ring-row:0001")
        .rollout_ring_row
        .as_mut()
        .expect("ring")
        .discloses_ring = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn summary_card_hiding_a_plane_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:deployment-summary-card:0001")
        .deployment_summary_card
        .as_mut()
        .expect("card")
        .control_plane_visible = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn summary_card_disclosing_different_mode_than_row_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:deployment-summary-card:0001")
        .deployment_summary_card
        .as_mut()
        .expect("card")
        .operating_mode = M5DeploymentMode::Desktop;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::DescriptorRowMismatch));
}

#[test]
fn residual_dependency_hiding_dependency_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:residual-dependency-row:0001")
        .residual_dependency_row
        .as_mut()
        .expect("row")
        .discloses_residual = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn status_strip_masking_impairment_as_local_fails() {
    let mut packet = packet();
    row_mut(
        &mut packet,
        "component:control-plane-data-plane-status-strip:0001",
    )
    .control_plane_data_plane_status_strip
    .as_mut()
    .expect("strip")
    .impairment_not_masked_as_local_failure = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn mirror_row_showing_stale_as_current_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:mirror-offline-artifact-row:0001")
        .mirror_offline_artifact_row
        .as_mut()
        .expect("row")
        .stale_not_shown_as_current = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn mirror_row_disclosing_different_freshness_than_row_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:mirror-offline-artifact-row:0001")
        .mirror_offline_artifact_row
        .as_mut()
        .expect("row")
        .freshness = M5DeploymentTruthMode::Live;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::DescriptorRowMismatch));
}

#[test]
fn mode_change_without_review_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:mode-change-review-sheet:0001")
        .mode_change_review_sheet
        .as_mut()
        .expect("sheet")
        .reviewed_before_durable_change = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn mode_change_hiding_cache_and_rollback_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:mode-change-review-sheet:0001")
        .mode_change_review_sheet
        .as_mut()
        .expect("sheet")
        .discloses_cache_and_rollback = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::PayloadDishonest));
}

#[test]
fn missing_operating_context_ref_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:install-profile-card:0001").operating_context_ref =
        "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::RowIncomplete));
}

#[test]
fn missing_mandatory_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:install-profile-card:0001")
        .required_labels
        .retain(|l| *l != M5DeploymentRequiredLabel::KeyboardRoute);
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::MandatoryLabelMissing));
}

#[test]
fn not_export_safe_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:install-profile-card:0001").export_safe = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::ParityMissing));
}

#[test]
fn not_assistive_ready_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:install-profile-card:0001").assistive_ready = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::ParityMissing));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:install-profile-card:0002")
        .degraded
        .as_mut()
        .expect("degraded")
        .degraded_label = "offline".to_owned();
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::DegradedLabelGeneric));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.components[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_DOC_REF);
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet
        .guardrails
        .control_plane_impairment_never_masked_as_local = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .later_rows_reference_one_canonical_family = false;
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::WrongRecordKind));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:install-profile-card:0001").label_summary =
        "leaked Bearer abc123 token".to_owned();
    assert!(packet
        .validate()
        .contains(&DeploymentContinuityComponentViolation::RawBoundaryMaterialInExport));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: DeploymentContinuityComponentMatrix =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let row = &packet().components[0];
    let chips = row.chip_tokens();
    assert!(chips.contains("family=install_profile_card"));
    assert!(chips.contains("truth=live"));
    assert!(chips.contains("mode=managed"));
    assert!(chips.contains("export_safe=true"));
    assert!(chips.contains("assistive=true"));
}

#[test]
fn csv_names_every_component() {
    let csv = packet().render_matrix_csv();
    assert!(csv.contains("component_id,family,truth_mode"));
    assert!(csv.contains("component:mirror-offline-artifact-row:0001"));
    assert!(csv.contains("mirror_stale"));
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Deployment/Continuity Component Matrix"));
    assert!(summary.contains("component:control-plane-data-plane-status-strip:0001"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_deployment_continuity_component_matrix_export()
        .expect("checked deployment/continuity component export validates");
    assert_eq!(checked, packet());
}

fn certification() -> M5DeploymentContinuitySurfaceCertificationPacket {
    seeded_deployment_continuity_surface_certification()
}

fn certification_row_mut(
    packet: &mut M5DeploymentContinuitySurfaceCertificationPacket,
    surface: M5ClaimedDeploymentSurface,
) -> &mut M5DeploymentContinuitySurfaceCertificationRow {
    packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface == surface)
        .unwrap_or_else(|| panic!("surface {}", surface.as_str()))
}

#[test]
fn surface_certification_validates() {
    let packet = certification();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn surface_certification_covers_every_claimed_surface() {
    let packet = certification();
    let surfaces = packet.represented_surfaces();
    for surface in M5ClaimedDeploymentSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "missing surface: {}",
            surface.as_str()
        );
    }
}

#[test]
fn surface_certification_has_narrowed_rows() {
    assert!(certification().narrowed_row_count() >= 1);
}

#[test]
fn missing_surface_fails_certification() {
    let mut packet = certification();
    packet
        .surface_rows
        .retain(|row| row.surface != M5ClaimedDeploymentSurface::AirGapped);
    assert!(packet
        .validate()
        .contains(&M5DeploymentContinuitySurfaceCertificationViolation::RequiredSurfaceMissing));
}

#[test]
fn missing_certification_drill_fails() {
    let mut packet = certification();
    certification_row_mut(&mut packet, M5ClaimedDeploymentSurface::Managed)
        .drills
        .retain(|drill| drill.drill_kind != M5DeploymentCertificationDrillKind::Degradation);
    let violations = packet.validate();
    assert!(violations
        .contains(&M5DeploymentContinuitySurfaceCertificationViolation::DrillCoverageMissing));
    assert!(violations
        .contains(&M5DeploymentContinuitySurfaceCertificationViolation::SurfaceRowIncomplete));
}

#[test]
fn missing_compatibility_dimension_fails() {
    let mut packet = certification();
    certification_row_mut(&mut packet, M5ClaimedDeploymentSurface::Managed)
        .compatibility_notes
        .retain(|note| {
            note.dimension != M5DeploymentCompatibilityDimension::ControlPlaneDataPlaneContinuity
        });
    let violations = packet.validate();
    assert!(violations.contains(
        &M5DeploymentContinuitySurfaceCertificationViolation::CompatibilityCoverageMissing
    ));
    assert!(violations
        .contains(&M5DeploymentContinuitySurfaceCertificationViolation::SurfaceRowIncomplete));
}

#[test]
fn degraded_surface_without_visible_narrowing_fails() {
    let mut packet = certification();
    let row = certification_row_mut(&mut packet, M5ClaimedDeploymentSurface::Managed);
    row.effective_label = M5DeploymentClaimLabel::FullTruth;
    row.auto_narrowed = false;
    row.narrowing_reasons.clear();
    let violations = packet.validate();
    assert!(violations.contains(
        &M5DeploymentContinuitySurfaceCertificationViolation::ClaimNarrowingInconsistent
    ));
    assert!(violations
        .contains(&M5DeploymentContinuitySurfaceCertificationViolation::SurfaceRowIncomplete));
}

#[test]
fn healthy_surface_with_spurious_narrowing_fails() {
    let mut packet = certification();
    let row = certification_row_mut(&mut packet, M5ClaimedDeploymentSurface::LocalOnly);
    row.effective_label = M5DeploymentClaimLabel::DegradedNarrowed;
    row.auto_narrowed = true;
    row.narrowing_reasons
        .push("Healthy local-only surface was incorrectly narrowed".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(
        &M5DeploymentContinuitySurfaceCertificationViolation::ClaimNarrowingInconsistent
    ));
}

#[test]
fn certification_release_proof_incomplete_fails() {
    let mut packet = certification();
    packet.release_support_proof.export_drills_complete = false;
    assert!(packet.validate().contains(
        &M5DeploymentContinuitySurfaceCertificationViolation::ReleaseSupportProofIncomplete
    ));
}

#[test]
fn certification_csv_names_surface_labels() {
    let csv = certification().render_certification_csv();
    assert!(csv.contains("surface,claimed_label,effective_label"));
    assert!(csv.contains("air_gapped,full_truth,local_safe_only,true"));
    assert!(csv.contains("side_by_side,full_truth,full_truth,false"));
}

#[test]
fn certification_report_names_narrowing() {
    let report = certification().render_certification_report();
    assert!(report.contains("M5 Deployment/Continuity Surface Certification"));
    assert!(report.contains("fleet_rollout"));
    assert!(report.contains("auto_narrowed=true"));
}

#[test]
fn checked_surface_certification_matches_builder() {
    let checked = current_m5_deployment_continuity_surface_certification_export()
        .expect("checked deployment/continuity surface certification validates");
    assert_eq!(checked, certification());
}
