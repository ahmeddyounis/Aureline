use super::*;

fn packet() -> WorkflowBundleComponentMatrix {
    seeded_workflow_bundle_component_matrix()
}

fn row_of(
    packet: &mut WorkflowBundleComponentMatrix,
    family: M5WorkflowBundleComponentFamily,
) -> &mut ComponentRow {
    packet
        .components
        .iter_mut()
        .find(|r| r.family == family)
        .expect("seeded matrix has a row for every family")
}

#[test]
fn packet_validates() {
    assert!(packet().validate().is_empty());
}

#[test]
fn every_family_is_defined() {
    let families = packet().represented_families();
    for required in M5WorkflowBundleComponentFamily::ALL {
        assert!(families.contains(&required), "missing family {required:?}");
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
            "row {} has a stray or missing payload",
            row.component_id
        );
    }
}

#[test]
fn missing_family_fails() {
    let mut p = packet();
    p.components
        .retain(|r| r.family != M5WorkflowBundleComponentFamily::BundleDetailPage);
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::RequiredFamilyMissing));
}

#[test]
fn no_degraded_row_fails() {
    let mut p = packet();
    for row in &mut p.components {
        row.degraded = None;
    }
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::DegradedCaseMissing));
}

#[test]
fn wrong_payload_for_family_fails() {
    let mut p = packet();
    let row = row_of(&mut p, M5WorkflowBundleComponentFamily::BundleDetailPage);
    // Attach a second, foreign payload alongside the detail page.
    row.bundle_drift_banner = Some(BundleDriftBannerDescriptor {
        bundle_id_ref: "bundle:launch:0001".to_owned(),
        drift_state: DriftState::Diverged,
        local_override_state: AssetOwnership::LocallyOverridden,
        reads_like_generic_package_update: false,
        discloses_override_state: true,
    });
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadFamilyMismatch));
}

#[test]
fn start_center_card_hiding_signer_source_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::StartCenterBundleCard,
    );
    row.start_center_bundle_card
        .as_mut()
        .expect("card")
        .discloses_signer_source = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn start_center_card_disclosing_different_class_than_row_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::StartCenterBundleCard,
    );
    row.start_center_bundle_card
        .as_mut()
        .expect("card")
        .bundle_class = BundleClass::OrgManagedBundle;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::DescriptorRowMismatch));
}

#[test]
fn badge_group_inventing_private_meaning_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::CertifiedArchetypeBadgeGroup,
    );
    row.certified_archetype_badge_group
        .as_mut()
        .expect("badges")
        .invents_private_badge_meaning = true;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn detail_page_hiding_entitlement_deps_fails() {
    let mut p = packet();
    let row = row_of(&mut p, M5WorkflowBundleComponentFamily::BundleDetailPage);
    row.bundle_detail_page
        .as_mut()
        .expect("detail")
        .lists_entitlement_dependencies = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn review_sheet_without_review_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::BundleInstallUpdateReviewSheet,
    );
    row.bundle_install_update_review_sheet
        .as_mut()
        .expect("sheet")
        .reviewed_before_apply = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn drift_banner_reading_like_package_update_fails() {
    let mut p = packet();
    let row = row_of(&mut p, M5WorkflowBundleComponentFamily::BundleDriftBanner);
    row.bundle_drift_banner
        .as_mut()
        .expect("banner")
        .reads_like_generic_package_update = true;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn override_row_discarding_local_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::BundleLocalOverrideRow,
    );
    row.bundle_local_override_row
        .as_mut()
        .expect("override")
        .preserves_local_override = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn rollback_card_hiding_side_effects_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::BundleRollbackRemoveCard,
    );
    row.bundle_rollback_remove_card
        .as_mut()
        .expect("rollback")
        .discloses_side_effects = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn class_disclosure_inventing_private_class_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::BundleClassDisclosureCard,
    );
    row.bundle_class_disclosure_card
        .as_mut()
        .expect("disclosure")
        .invents_private_class_meaning = true;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn class_disclosure_disclosing_different_class_than_row_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::BundleClassDisclosureCard,
    );
    row.bundle_class_disclosure_card
        .as_mut()
        .expect("disclosure")
        .bundle_class = BundleClass::LaunchBundle;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::DescriptorRowMismatch));
}

#[test]
fn claim_narrowing_inventing_stale_wording_fails() {
    let mut p = packet();
    let row = row_of(
        &mut p,
        M5WorkflowBundleComponentFamily::BundleClaimNarrowingRow,
    );
    row.bundle_claim_narrowing_row
        .as_mut()
        .expect("narrowing")
        .invents_stale_wording = true;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::PayloadDishonest));
}

#[test]
fn missing_bundle_context_ref_fails() {
    let mut p = packet();
    p.components[0].bundle_context_ref = String::new();
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::RowIncomplete));
}

#[test]
fn missing_mandatory_label_fails() {
    let mut p = packet();
    p.components[0]
        .required_labels
        .retain(|l| *l != M5BundleRequiredLabel::KeyboardRoute);
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::MandatoryLabelMissing));
}

#[test]
fn not_export_safe_fails() {
    let mut p = packet();
    p.components[0].export_safe = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::ParityMissing));
}

#[test]
fn not_assistive_ready_fails() {
    let mut p = packet();
    p.components[0].assistive_ready = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::ParityMissing));
}

#[test]
fn generic_degraded_label_fails() {
    let mut p = packet();
    let row = row_of(&mut p, M5WorkflowBundleComponentFamily::BundleDriftBanner);
    row.degraded = Some(DegradedState {
        trigger: M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
        degraded_label: "drift".to_owned(),
    });
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::DegradedLabelGeneric));
}

#[test]
fn row_without_evidence_fails() {
    let mut p = packet();
    p.components[0].evidence_refs.clear();
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut p = packet();
    p.source_contract_refs
        .retain(|r| r != WORKFLOW_BUNDLE_COMPONENT_MATRIX_DOC_REF);
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut p = packet();
    p.guardrails.drift_never_reads_like_generic_package_update = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut p = packet();
    p.consumer_projection
        .later_rows_reference_one_canonical_family = false;
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut p = packet();
    p.record_kind = "something_else".to_owned();
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::WrongRecordKind));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut p = packet();
    p.components[0].label_summary = "Bearer abc123 leaked into the label".to_owned();
    assert!(p
        .validate()
        .contains(&WorkflowBundleComponentViolation::RawBoundaryMaterialInExport));
}

#[test]
fn export_safe_json_round_trips() {
    let p = packet();
    let json = p.export_safe_json();
    let back: WorkflowBundleComponentMatrix = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(p, back);
    assert!(back.validate().is_empty());
}

#[test]
fn chip_tokens_name_governed_chips() {
    let chips = packet().components[0].chip_tokens();
    assert!(chips.contains("family="));
    assert!(chips.contains("truth="));
    assert!(chips.contains("class="));
    assert!(chips.contains("export_safe="));
    assert!(chips.contains("assistive="));
}

#[test]
fn csv_names_every_component() {
    let csv = packet().render_matrix_csv();
    for row in &packet().components {
        assert!(
            csv.contains(&row.component_id),
            "csv missing {}",
            row.component_id
        );
    }
    assert!(csv.contains("mirror_stale"));
}

#[test]
fn markdown_summary_names_rows() {
    let md = packet().render_markdown_summary();
    assert!(md.contains("# M5 Workflow-Bundle Component Matrix"));
    assert!(md.contains("component:start-center-bundle-card:0001"));
    assert!(md.contains("Degraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked =
        current_m5_workflow_bundle_component_matrix_export().expect("checked export is valid");
    assert_eq!(checked, seeded_workflow_bundle_component_matrix());
}

#[test]
fn truth_mode_current_source_only_live() {
    for mode in M5BundleTruthMode::ALL {
        assert_eq!(mode.is_current_source(), mode == M5BundleTruthMode::Live);
    }
}

#[test]
fn required_label_mandatory_subset() {
    let all: BTreeSet<_> = M5BundleRequiredLabel::ALL.into_iter().collect();
    for label in M5BundleRequiredLabel::MANDATORY {
        assert!(all.contains(&label));
    }
}

#[test]
fn family_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5WorkflowBundleComponentFamily::ALL
        .iter()
        .map(|f| f.as_str())
        .collect();
    assert_eq!(tokens.len(), M5WorkflowBundleComponentFamily::ALL.len());
}
