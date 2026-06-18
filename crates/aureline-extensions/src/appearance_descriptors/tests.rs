//! Unit coverage for governed extension appearance descriptors.

use super::{
    build_extension_appearance_audit, evaluate_extension_appearance_descriptor,
    project_extension_appearance_support_export, seeded_extension_appearance_audit,
    seeded_extension_appearance_inputs, seeded_extension_appearance_support_export,
    validate_extension_appearance_audit, validate_extension_appearance_descriptor,
    validate_extension_appearance_support_export, AppearanceAxisClass,
    AppearanceDescriptorDefectKind, AppearanceGap, ExtensionAppearanceInput, InheritanceBadgeClass,
    ParityClaimStateClass, SurfaceKindClass, EXTENSION_APPEARANCE_AUDIT_ID,
    EXTENSION_APPEARANCE_AUDIT_RECORD_KIND, EXTENSION_APPEARANCE_DESCRIPTOR_RECORD_KIND,
    EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF, RENDERED_SURFACE_TOKENS,
};
use crate::webview_boundary::ExtensionInheritanceClass;

fn first_input() -> ExtensionAppearanceInput {
    seeded_extension_appearance_inputs()
        .into_iter()
        .next()
        .expect("seeded inputs are non-empty")
}

fn input_named(descriptor_id: &str) -> ExtensionAppearanceInput {
    seeded_extension_appearance_inputs()
        .into_iter()
        .find(|input| input.descriptor_id == descriptor_id)
        .unwrap_or_else(|| panic!("seeded input {descriptor_id} must exist"))
}

#[test]
fn seeded_audit_validates_and_is_clean() {
    let audit = seeded_extension_appearance_audit();
    validate_extension_appearance_audit(&audit).expect("seeded audit must validate");
    assert!(audit.is_clean());
    assert_eq!(audit.record_kind, EXTENSION_APPEARANCE_AUDIT_RECORD_KIND);
    assert_eq!(audit.audit_id, EXTENSION_APPEARANCE_AUDIT_ID);
    assert_eq!(
        audit.shared_contract_ref,
        EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF
    );
}

#[test]
fn seeded_audit_is_deterministic() {
    assert_eq!(
        seeded_extension_appearance_audit(),
        seeded_extension_appearance_audit()
    );
}

#[test]
fn seeded_summary_recomputes_from_descriptors() {
    let audit = seeded_extension_appearance_audit();
    assert_eq!(audit.summary.descriptor_count, 5);
    assert_eq!(audit.summary.full_inheritance_count, 1);
    assert_eq!(audit.summary.does_not_inherit_count, 1);
    assert!(audit.summary.partial_inheritance_count >= 1);
    assert_eq!(audit.summary.undisclosed_count, 0);
    assert_eq!(audit.summary.host_parity_claim_count, 1);
    assert_eq!(audit.summary.partial_parity_claim_count, 1);
    assert_eq!(audit.summary.denied_parity_claim_count, 0);
    assert_eq!(audit.summary.defect_count, 0);
}

#[test]
fn every_descriptor_carries_five_axes() {
    let audit = seeded_extension_appearance_audit();
    for descriptor in &audit.descriptors {
        assert_eq!(
            descriptor.axes.len(),
            5,
            "descriptor {} must carry five governed axes",
            descriptor.descriptor_id
        );
        // partial / does-not-inherit / undisclosed axes always require disclosure.
        for posture in &descriptor.axes {
            if posture.posture != ExtensionInheritanceClass::Inherits {
                assert!(
                    posture.user_visible_disclosure_required,
                    "non-inheriting axis {} must require disclosure",
                    posture.axis.as_str()
                );
            }
        }
    }
}

#[test]
fn descriptor_records_carry_envelope_and_rendered_surfaces() {
    let audit = seeded_extension_appearance_audit();
    for descriptor in &audit.descriptors {
        assert_eq!(
            descriptor.record_kind,
            EXTENSION_APPEARANCE_DESCRIPTOR_RECORD_KIND
        );
        for token in RENDERED_SURFACE_TOKENS {
            assert!(
                descriptor.rendered_on_surfaces.iter().any(|s| s == token),
                "descriptor {} must render its badge on {token}",
                descriptor.descriptor_id
            );
        }
    }
}

#[test]
fn full_inheritance_with_evidence_grants_host_parity() {
    let input = input_named("extension-appearance:dev.aureline.samples/markdown-lens:preview-pane");
    let descriptor = evaluate_extension_appearance_descriptor(input);
    assert_eq!(
        descriptor.badge.badge_class,
        InheritanceBadgeClass::FullInheritance
    );
    assert_eq!(
        descriptor.parity_claim_state,
        ParityClaimStateClass::ClaimsHostParity
    );
    assert!(descriptor.badge.implies_host_parity);
    assert!(descriptor.is_clean());
}

#[test]
fn honest_partial_claim_with_disclosed_gaps_is_accepted() {
    let input = input_named("extension-appearance:dev.aureline.samples/api-docs:help-pane");
    let descriptor = evaluate_extension_appearance_descriptor(input);
    assert_eq!(
        descriptor.badge.badge_class,
        InheritanceBadgeClass::PartialInheritance
    );
    assert_eq!(
        descriptor.parity_claim_state,
        ParityClaimStateClass::PartialClaimWithGaps
    );
    assert!(descriptor.is_clean());
}

#[test]
fn private_styling_badges_does_not_inherit() {
    let input = input_named("extension-appearance:io.devtools.legacy/console:panel");
    let descriptor = evaluate_extension_appearance_descriptor(input);
    assert_eq!(
        descriptor.badge.badge_class,
        InheritanceBadgeClass::DoesNotInherit
    );
    assert_eq!(
        descriptor.parity_claim_state,
        ParityClaimStateClass::NoParityClaim
    );
}

#[test]
fn embedded_webview_with_mixed_axes_badges_partial() {
    let input = input_named("extension-appearance:com.acme.insights/analytics:dashboard");
    let descriptor = evaluate_extension_appearance_descriptor(input);
    assert_eq!(
        descriptor.badge.badge_class,
        InheritanceBadgeClass::PartialInheritance
    );
    assert!(!descriptor.badge.implies_host_parity);
    assert_eq!(descriptor.surface_kind, SurfaceKindClass::EmbeddedWebview);
}

#[test]
fn parity_claim_without_full_inheritance_is_denied() {
    let mut input = input_named("extension-appearance:com.acme.insights/analytics:dashboard");
    input.claims_first_party_parity = true;
    let descriptor = evaluate_extension_appearance_descriptor(input);
    assert_eq!(
        descriptor.parity_claim_state,
        ParityClaimStateClass::DeniedClaim
    );
    assert!(descriptor.defect_kind_tokens.contains(
        &AppearanceDescriptorDefectKind::OverclaimedParity
            .as_str()
            .to_owned()
    ));
}

#[test]
fn parity_claim_without_accessibility_evidence_is_denied() {
    let mut input =
        input_named("extension-appearance:dev.aureline.samples/markdown-lens:preview-pane");
    input.accessibility_evidence_refs.clear();
    let descriptor = evaluate_extension_appearance_descriptor(input);
    assert_eq!(
        descriptor.parity_claim_state,
        ParityClaimStateClass::DeniedClaim
    );
    assert!(descriptor.defect_kind_tokens.contains(
        &AppearanceDescriptorDefectKind::OverclaimedParity
            .as_str()
            .to_owned()
    ));
}

#[test]
fn undisclosed_axis_is_a_defect_and_badges_undisclosed() {
    let mut input = first_input();
    input.inherit_contrast = ExtensionInheritanceClass::NotDisclosed;
    let descriptor = evaluate_extension_appearance_descriptor(input);
    assert_eq!(
        descriptor.badge.badge_class,
        InheritanceBadgeClass::Undisclosed
    );
    let defects = validate_extension_appearance_descriptor(&descriptor);
    assert!(defects
        .iter()
        .any(|d| d.defect_kind == AppearanceDescriptorDefectKind::UndisclosedAxis));
}

#[test]
fn suppressed_host_badge_is_a_defect() {
    let mut input = first_input();
    input.host_rendered_appearance_badge = false;
    let descriptor = evaluate_extension_appearance_descriptor(input);
    assert!(descriptor.defect_kind_tokens.contains(
        &AppearanceDescriptorDefectKind::HostBadgeChromeHidden
            .as_str()
            .to_owned()
    ));
}

#[test]
fn full_badge_with_gap_is_a_hidden_gap_defect() {
    let mut input =
        input_named("extension-appearance:dev.aureline.samples/markdown-lens:preview-pane");
    input.known_gaps.push(AppearanceGap {
        axis: AppearanceAxisClass::Density,
        summary: "Undisclosed in the badge but present.".to_owned(),
    });
    let descriptor = evaluate_extension_appearance_descriptor(input);
    let defects = validate_extension_appearance_descriptor(&descriptor);
    assert!(defects
        .iter()
        .any(|d| d.defect_kind == AppearanceDescriptorDefectKind::HiddenInheritanceGap));
}

#[test]
fn audit_validation_collects_defects_for_overclaim() {
    let mut inputs = seeded_extension_appearance_inputs();
    // Force the embedded webview to overclaim parity.
    for input in &mut inputs {
        if input.surface_kind == SurfaceKindClass::EmbeddedWebview {
            input.claims_first_party_parity = true;
        }
    }
    let descriptors = inputs
        .into_iter()
        .map(evaluate_extension_appearance_descriptor)
        .collect();
    let audit = build_extension_appearance_audit(descriptors);
    let err = validate_extension_appearance_audit(&audit)
        .expect_err("overclaiming audit must fail validation");
    assert!(err
        .iter()
        .any(|d| d.defect_kind == AppearanceDescriptorDefectKind::OverclaimedParity));
}

#[test]
fn support_export_round_trips_and_validates() {
    let audit = seeded_extension_appearance_audit();
    let export = seeded_extension_appearance_support_export();
    let projected = project_extension_appearance_support_export(&audit, export.export_id.clone());
    assert_eq!(export, projected);
    validate_extension_appearance_support_export(&audit, &export)
        .expect("seeded support export must validate");
    assert!(export.raw_appearance_material_excluded);
    assert!(export.case_ids.contains(&audit.audit_id));
    for descriptor in &audit.descriptors {
        assert!(
            export.case_ids.contains(&descriptor.descriptor_id),
            "support export must quote descriptor {}",
            descriptor.descriptor_id
        );
    }
}

#[test]
fn descriptors_serde_round_trip() {
    let audit = seeded_extension_appearance_audit();
    let json = serde_json::to_string(&audit).expect("serialize");
    let back: super::ExtensionAppearanceAudit = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(audit, back);
}
