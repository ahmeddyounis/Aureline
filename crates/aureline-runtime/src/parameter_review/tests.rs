//! Unit coverage for the parameter-review object and its first-consumers packet.

use super::*;
use crate::m5_automation_contract_baseline::{ArgumentInspectionKind, ParameterReviewVerdictClass};

fn request_sheet() -> ParameterReviewBuilder {
    seeded_consumer_sheet(RecipeBuilderEntrypoint::RequestApi)
}

#[test]
fn adds_parameters_in_order() {
    let sheet = request_sheet();
    let names: Vec<&str> = sheet
        .parameters
        .iter()
        .map(|parameter| parameter.parameter_name.as_str())
        .collect();
    assert_eq!(
        names,
        vec![
            "environment_profile",
            "request_url",
            "bearer_token",
            "body_variable"
        ]
    );
}

#[test]
fn rejects_duplicate_parameter_names() {
    let mut sheet = request_sheet();
    let duplicate = ReviewedParameter {
        parameter_name: "request_url".to_owned(),
        field_type: ParameterFieldType::UrlReference,
        source_layer: ParameterSourceLayer::RecipeSupplied,
        value_state: ParameterValueState::DefaultValue,
        required: true,
        sensitivity_class: "metadata_safe_default".to_owned(),
        secret_reference: None,
        chosen_save_scope: SaveToScope::RunOnly,
        available_save_scopes: vec![SaveToScope::RunOnly],
        validation: ParameterValidation::satisfied(
            ParameterConstraintKind::UrlScheme,
            "an https URL",
        ),
        summary: "duplicate".to_owned(),
    };
    let err = sheet.add_parameter(duplicate).unwrap_err();
    assert_eq!(
        err,
        ParameterReviewError::DuplicateParameterName("request_url".into())
    );
}

#[test]
fn every_parameter_is_typed_with_an_explicit_source_layer() {
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        let sheet = seeded_consumer_sheet(entrypoint);
        assert!(!sheet.parameters.is_empty());
        for parameter in &sheet.parameters {
            assert!(parameter.source_layer.explicit_inspection_kind().is_some());
        }
    }
}

#[test]
fn secret_values_are_held_as_references_never_raw() {
    let sheet = request_sheet();
    let token = sheet.parameter("bearer_token").expect("bearer_token");
    assert_eq!(token.field_type, ParameterFieldType::SecretReference);
    assert!(token.secret_reference.is_some());
    assert!(token.secret_posture_consistent());
    assert_eq!(
        token.verdict_class(),
        ParameterReviewVerdictClass::SensitiveHeldForReview
    );
    // A secret reference maps to the credential-handle inspection kind.
    assert_eq!(
        token.source_layer.inspection_kind(),
        ArgumentInspectionKind::CredentialHandleArgumentRef
    );
}

#[test]
fn override_keeps_state_visible_and_preserves_provenance() {
    let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Notebook);
    sheet.override_parameter("kernel_profile").unwrap();
    let parameter = sheet.parameter("kernel_profile").expect("kernel_profile");
    assert!(parameter.value_state.is_override());
    // The override does not change the source layer or save scope.
    assert_eq!(
        parameter.source_layer,
        ParameterSourceLayer::DescriptorDefault
    );
    assert_eq!(parameter.chosen_save_scope, SaveToScope::Workspace);
}

#[test]
fn policy_pinned_parameter_reads_as_policy_pinned() {
    let sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Package);
    let channel = sheet.parameter("update_channel").expect("update_channel");
    assert_eq!(channel.value_state, ParameterValueState::PolicyPinned);
    assert_eq!(
        channel.verdict_class(),
        ParameterReviewVerdictClass::PolicyPinned
    );
    assert_eq!(channel.chosen_save_scope, SaveToScope::OrganizationPolicy);
}

#[test]
fn awaiting_required_input_counts_as_unresolved() {
    let sheet = request_sheet();
    let body = sheet.parameter("body_variable").expect("body_variable");
    assert_eq!(
        body.verdict_class(),
        ParameterReviewVerdictClass::NeedsInput
    );
    assert!(body.is_unresolved_required());
    assert_eq!(sheet.unresolved_required_count(), 1);
    assert!(!sheet.is_apply_ready());
}

#[test]
fn fully_resolved_sheet_is_apply_ready() {
    let sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Notebook);
    assert_eq!(sheet.unresolved_required_count(), 0);
    assert!(sheet.is_apply_ready());
}

#[test]
fn sheet_record_reuses_verdict_truth() {
    let sheet = request_sheet();
    let record = sheet.to_sheet_record();
    assert_eq!(record.record_kind, PARAMETER_REVIEW_SHEET_RECORD_KIND);
    assert_eq!(record.rows.len(), sheet.parameters.len());
    for (row, parameter) in record.rows.iter().zip(&sheet.parameters) {
        assert_eq!(row.parameter_name, parameter.parameter_name);
        assert_eq!(row.verdict_class, parameter.verdict_class());
        assert_eq!(
            row.inspection_kind,
            parameter.source_layer.inspection_kind()
        );
    }
    assert_eq!(record.unresolved_required_count, 1);
}

#[test]
fn export_round_trips_to_an_equal_sheet() {
    let mut sheet = request_sheet();
    sheet.override_parameter("request_url").unwrap();
    let export = sheet.export("export:test", "2026-06-18T00:01:00Z");
    let imported = export.import();
    assert_eq!(imported, sheet);
    assert!(export.provenance_preserved());
    assert_eq!(export.record_kind, PARAMETER_REVIEW_EXPORT_RECORD_KIND);
}

#[test]
fn seeded_packet_is_stable() {
    let packet = seeded_parameter_review_first_consumers_packet();
    assert!(packet.is_stable());
    assert!(packet.validation_findings.is_empty());
    assert_eq!(
        packet.consumer_bindings.len(),
        RecipeBuilderEntrypoint::ALL.len()
    );
    for entrypoint in RecipeBuilderEntrypoint::ALL {
        assert!(packet.binding(entrypoint).is_some());
    }
}

#[test]
fn missing_entrypoint_blocks_stable() {
    let mut input = current_parameter_review_first_consumers_input();
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Package);
    let packet = ParameterReviewFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert_eq!(
        packet.promotion_state,
        AutomationBaselinePromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == ParameterReviewFindingKind::MissingEntrypoint));
}

#[test]
fn raw_secret_blocks_stable() {
    let mut sheet = request_sheet();
    // Forge a secret field that carries no broker handle but claims a resolved value.
    let token = sheet
        .parameters
        .iter_mut()
        .find(|parameter| parameter.parameter_name == "bearer_token")
        .expect("bearer_token");
    token.secret_reference = None;
    token.value_state = ParameterValueState::DefaultValue;
    assert!(!token.secret_posture_consistent());

    let mut input = current_parameter_review_first_consumers_input();
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::RequestApi);
    input
        .consumer_bindings
        .push(ParameterReviewConsumerBinding::from_builder(&sheet));
    let packet = ParameterReviewFirstConsumersPacket::materialize(input);
    assert!(packet.validation_findings.iter().any(
        |finding| finding.finding_kind == ParameterReviewFindingKind::SecretValueNotReferenced
    ));
}

#[test]
fn disallowed_save_scope_blocks_stable() {
    let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Notebook);
    let parameter = sheet
        .parameters
        .iter_mut()
        .find(|parameter| parameter.parameter_name == "output_dir")
        .expect("output_dir");
    // output_dir does not allow user scope.
    parameter.chosen_save_scope = SaveToScope::User;
    assert!(!parameter.save_scope_allowed());

    let mut input = current_parameter_review_first_consumers_input();
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Notebook);
    input
        .consumer_bindings
        .push(ParameterReviewConsumerBinding::from_builder(&sheet));
    let packet = ParameterReviewFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == ParameterReviewFindingKind::SaveScopeNotAllowed));
}

#[test]
fn unspecified_source_layer_blocks_stable() {
    let mut sheet = seeded_consumer_sheet(RecipeBuilderEntrypoint::Incident);
    let parameter = sheet
        .parameters
        .iter_mut()
        .find(|parameter| parameter.parameter_name == "incident_ref")
        .expect("incident_ref");
    parameter.source_layer = ParameterSourceLayer::UnspecifiedGenericControl;

    let mut input = current_parameter_review_first_consumers_input();
    input
        .consumer_bindings
        .retain(|binding| binding.entrypoint != RecipeBuilderEntrypoint::Incident);
    input
        .consumer_bindings
        .push(ParameterReviewConsumerBinding::from_builder(&sheet));
    let packet = ParameterReviewFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == ParameterReviewFindingKind::SourceLayerUnspecified));
}

#[test]
fn inconsistent_projection_blocks_stable() {
    let mut input = current_parameter_review_first_consumers_input();
    let binding = input
        .consumer_bindings
        .iter_mut()
        .find(|binding| binding.entrypoint == RecipeBuilderEntrypoint::Notebook)
        .expect("notebook binding");
    binding.sheet_record.rows[0].verdict_class = ParameterReviewVerdictClass::Blocked;
    let packet = ParameterReviewFirstConsumersPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == ParameterReviewFindingKind::SheetProjectionInconsistent));
}

#[test]
fn violated_invariant_blocks_stable() {
    let mut input = current_parameter_review_first_consumers_input();
    input.invariants.secret_values_are_references_not_raw = false;
    let packet = ParameterReviewFirstConsumersPacket::materialize(input);
    assert!(!packet.is_stable());
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == ParameterReviewFindingKind::InvariantViolated));
}

#[test]
fn support_export_and_cli_view_are_consistent() {
    let packet = seeded_parameter_review_first_consumers_packet();
    let export = packet.support_export("support:test", "2026-06-18T00:02:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id, packet.packet_id);
    assert_eq!(export.consumer_rows.len(), packet.consumer_bindings.len());
    // No support row carries a raw value; secret-bearing rows expose only the flag.
    for row in &export.consumer_rows {
        for parameter_row in &row.parameter_rows {
            if parameter_row.field_type == ParameterFieldType::SecretReference {
                assert!(parameter_row.held_as_secret_reference);
            }
        }
    }

    let view = packet.cli_headless_view("cli:test", "2026-06-18T00:02:00Z");
    assert!(view.every_entrypoint_explained());
}

#[test]
fn digest_is_order_invariant() {
    let packet = seeded_parameter_review_first_consumers_packet();
    let mut shuffled = current_parameter_review_first_consumers_input();
    shuffled.consumer_bindings.reverse();
    let reshuffled = ParameterReviewFirstConsumersPacket::materialize(shuffled);
    assert_eq!(packet.packet_digest, reshuffled.packet_digest);
}
