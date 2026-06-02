//! Validators for parameter-form-state, invocation-review-sheet, and the
//! catalog that pairs them.
//!
//! The validators enforce the acceptance invariants from the spec:
//!
//! 1. Every parameter form state must declare a field state record for
//!    every typed argument it advertises, with a controlled
//!    source-layer label and a typed restart/reload class.
//! 2. Secret-bearing fields must be handle-first: value visibility is
//!    `value_handle_only` (or masked / omitted) and the redaction class
//!    is at least `operator_only_restricted`.
//! 3. Field state classes that imply a typed source layer must agree
//!    with their declared source layer class.
//! 4. The form-level validation rollup must accurately reflect the
//!    field-level findings.
//! 5. Invocation review sheets must list at least one side-effect class,
//!    quote the same command id / revision / form-state id as the form
//!    they project from, and offer a preview / dry-run path unless the
//!    descriptor declares preview is unavailable by design.
//! 6. Blocked prerequisites pair with an execution intent that cannot
//!    apply (propose / simulate / cancel only); a sheet with blocked
//!    prerequisites whose intent is apply-class is non-conforming.

use serde::{Deserialize, Serialize};

use super::{
    ApprovalPostureClass, CommandFormBundle, CommandFormsCatalog, ExecutionIntentClass,
    FieldStateClass, FieldStateRecord, InvocationReviewSheetRecord, ParameterFormStateRecord,
    RedactionClass, SourceLayerClass, ValidationSeverity, ValueVisibilityClass,
    COMMAND_FORMS_CATALOG_RECORD_KIND, COMMAND_FORMS_SHARED_CONTRACT_REF,
    INVOCATION_REVIEW_SHEET_RECORD_KIND, INVOCATION_REVIEW_SHEET_SCHEMA_VERSION,
    PARAMETER_FORM_STATE_RECORD_KIND, PARAMETER_FORM_STATE_SCHEMA_VERSION,
};

/// One validation error returned by the catalog validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CommandFormsValidationError {
    EmptyCatalog,
    CatalogRecordKindMismatch {
        actual: String,
    },
    CatalogSharedContractRefMismatch {
        actual: String,
    },
    DuplicateScenarioId {
        scenario_id: String,
    },

    FormRecordKindMismatch {
        form_state_id: String,
        actual: String,
    },
    FormSchemaVersionMismatch {
        form_state_id: String,
        actual: u32,
    },
    FormMissingCommandRevision {
        form_state_id: String,
    },
    FormFieldDuplicateArgumentName {
        form_state_id: String,
        argument_name: String,
    },
    FormFieldMissingNarrationLabel {
        form_state_id: String,
        argument_name: String,
    },
    FormFieldSourceLayerMismatch {
        form_state_id: String,
        argument_name: String,
        field_state_class: String,
        source_layer_class: String,
    },
    FormFieldSecretValueVisibilityViolation {
        form_state_id: String,
        argument_name: String,
        value_visibility: String,
    },
    FormFieldSecretRedactionTooLoose {
        form_state_id: String,
        argument_name: String,
        redaction_class: String,
    },
    FormFieldUnsupportedMissingClass {
        form_state_id: String,
        argument_name: String,
    },
    FormFieldRuntimePromptHasResolvedValue {
        form_state_id: String,
        argument_name: String,
    },
    FormRollupMismatch {
        form_state_id: String,
        expected_severity: String,
        actual_severity: String,
        expected_blocking_count: u32,
        actual_blocking_count: u32,
    },

    ReviewRecordKindMismatch {
        review_sheet_id: String,
        actual: String,
    },
    ReviewSchemaVersionMismatch {
        review_sheet_id: String,
        actual: u32,
    },
    ReviewCommandIdMismatch {
        review_sheet_id: String,
        form_command_id: String,
        review_command_id: String,
    },
    ReviewCommandRevisionMismatch {
        review_sheet_id: String,
        form_revision: String,
        review_revision: String,
    },
    ReviewFormStateIdMismatch {
        review_sheet_id: String,
        form_state_id: String,
        review_form_state_id: String,
    },
    ReviewSideEffectsEmpty {
        review_sheet_id: String,
    },
    ReviewPreviewUnavailableButDryRunAdvertised {
        review_sheet_id: String,
    },
    ReviewPreviewRequiredButNotAvailable {
        review_sheet_id: String,
    },
    ReviewBlockedPrerequisitesWithApplyIntent {
        review_sheet_id: String,
        execution_intent: String,
    },
    ReviewSecretHandlingMissing {
        review_sheet_id: String,
    },
    ReviewSecretHandlingHandleOnlyDrift {
        review_sheet_id: String,
    },
    ReviewProvenanceSummaryFieldMismatch {
        review_sheet_id: String,
        argument_name: String,
    },
    ReviewBlockedReasonMissingRepairHook {
        review_sheet_id: String,
        blocked_class: String,
    },
    ReviewApprovalApplyMismatch {
        review_sheet_id: String,
        approval_posture: String,
        execution_intent: String,
    },
}

/// Validates one parameter-form-state record.
pub fn validate_parameter_form_state(
    form: &ParameterFormStateRecord,
) -> Result<(), Vec<CommandFormsValidationError>> {
    let mut errors = Vec::new();
    if form.record_kind != PARAMETER_FORM_STATE_RECORD_KIND {
        errors.push(CommandFormsValidationError::FormRecordKindMismatch {
            form_state_id: form.form_state_id.clone(),
            actual: form.record_kind.clone(),
        });
    }
    if form.parameter_form_state_schema_version != PARAMETER_FORM_STATE_SCHEMA_VERSION {
        errors.push(CommandFormsValidationError::FormSchemaVersionMismatch {
            form_state_id: form.form_state_id.clone(),
            actual: form.parameter_form_state_schema_version,
        });
    }
    if form.command_revision_ref.is_empty() {
        errors.push(CommandFormsValidationError::FormMissingCommandRevision {
            form_state_id: form.form_state_id.clone(),
        });
    }

    let mut seen = std::collections::HashSet::new();
    let mut counted_blocking = 0u32;
    let mut counted_warning = 0u32;
    let mut counted_informational = 0u32;

    for field in &form.fields {
        if !seen.insert(field.argument_name.clone()) {
            errors.push(
                CommandFormsValidationError::FormFieldDuplicateArgumentName {
                    form_state_id: form.form_state_id.clone(),
                    argument_name: field.argument_name.clone(),
                },
            );
        }
        if field.narration_label_ref.is_empty() {
            errors.push(
                CommandFormsValidationError::FormFieldMissingNarrationLabel {
                    form_state_id: form.form_state_id.clone(),
                    argument_name: field.argument_name.clone(),
                },
            );
        }
        validate_field_constraints(form, field, &mut errors);

        for finding in &field.validation_findings {
            match finding.severity {
                ValidationSeverity::Blocking => counted_blocking += 1,
                ValidationSeverity::Warning => counted_warning += 1,
                ValidationSeverity::Informational => counted_informational += 1,
            }
        }
    }

    let expected_severity = if counted_blocking > 0 {
        ValidationSeverity::Blocking
    } else if counted_warning > 0 {
        ValidationSeverity::Warning
    } else {
        ValidationSeverity::Informational
    };

    if form.validation_rollup.overall_severity != expected_severity
        || form.validation_rollup.blocking_finding_count != counted_blocking
        || form.validation_rollup.warning_finding_count != counted_warning
        || form.validation_rollup.informational_finding_count != counted_informational
    {
        errors.push(CommandFormsValidationError::FormRollupMismatch {
            form_state_id: form.form_state_id.clone(),
            expected_severity: expected_severity.as_str().to_owned(),
            actual_severity: form.validation_rollup.overall_severity.as_str().to_owned(),
            expected_blocking_count: counted_blocking,
            actual_blocking_count: form.validation_rollup.blocking_finding_count,
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_field_constraints(
    form: &ParameterFormStateRecord,
    field: &FieldStateRecord,
    errors: &mut Vec<CommandFormsValidationError>,
) {
    // Field-state to source-layer constraints.
    match field.field_state_class {
        FieldStateClass::PolicyPinnedReadOnly => {
            if field.source_layer_class != SourceLayerClass::OrgPolicyPinned {
                errors.push(CommandFormsValidationError::FormFieldSourceLayerMismatch {
                    form_state_id: form.form_state_id.clone(),
                    argument_name: field.argument_name.clone(),
                    field_state_class: field.field_state_class.as_str().to_owned(),
                    source_layer_class: field.source_layer_class.as_str().to_owned(),
                });
            }
        }
        FieldStateClass::SecretMaskedReadOnly | FieldStateClass::SecretHandleSwapOnly => {
            if field.source_layer_class != SourceLayerClass::SecretHandleReference {
                errors.push(CommandFormsValidationError::FormFieldSourceLayerMismatch {
                    form_state_id: form.form_state_id.clone(),
                    argument_name: field.argument_name.clone(),
                    field_state_class: field.field_state_class.as_str().to_owned(),
                    source_layer_class: field.source_layer_class.as_str().to_owned(),
                });
            }
            // Secret-bearing fields must be handle-first.
            if !matches!(
                field.value_visibility,
                ValueVisibilityClass::ValueHandleOnly
                    | ValueVisibilityClass::ValueMasked
                    | ValueVisibilityClass::ValueOmittedForRedaction
            ) {
                errors.push(
                    CommandFormsValidationError::FormFieldSecretValueVisibilityViolation {
                        form_state_id: form.form_state_id.clone(),
                        argument_name: field.argument_name.clone(),
                        value_visibility: field.value_visibility.as_str().to_owned(),
                    },
                );
            }
            if matches!(field.redaction_class, RedactionClass::MetadataSafeDefault) {
                errors.push(
                    CommandFormsValidationError::FormFieldSecretRedactionTooLoose {
                        form_state_id: form.form_state_id.clone(),
                        argument_name: field.argument_name.clone(),
                        redaction_class: field.redaction_class.as_str().to_owned(),
                    },
                );
            }
        }
        FieldStateClass::RuntimePromptRequired => {
            if field.source_layer_class != SourceLayerClass::RuntimePrompt {
                errors.push(CommandFormsValidationError::FormFieldSourceLayerMismatch {
                    form_state_id: form.form_state_id.clone(),
                    argument_name: field.argument_name.clone(),
                    field_state_class: field.field_state_class.as_str().to_owned(),
                    source_layer_class: field.source_layer_class.as_str().to_owned(),
                });
            }
            if field.resolved_value_ref.is_some() {
                errors.push(
                    CommandFormsValidationError::FormFieldRuntimePromptHasResolvedValue {
                        form_state_id: form.form_state_id.clone(),
                        argument_name: field.argument_name.clone(),
                    },
                );
            }
        }
        FieldStateClass::UnsupportedInClientScope
        | FieldStateClass::UnsupportedInRestrictedTrust => {
            if field.unsupported_field_class.is_none() {
                errors.push(
                    CommandFormsValidationError::FormFieldUnsupportedMissingClass {
                        form_state_id: form.form_state_id.clone(),
                        argument_name: field.argument_name.clone(),
                    },
                );
            }
        }
        _ => {}
    }

    // CredentialHandleRef kinds must be handle-first regardless of state.
    if matches!(
        field.argument_kind,
        super::ArgumentKind::CredentialHandleRef
    ) {
        if !matches!(
            field.value_visibility,
            ValueVisibilityClass::ValueHandleOnly
                | ValueVisibilityClass::ValueMasked
                | ValueVisibilityClass::ValueOmittedForRedaction
        ) {
            errors.push(
                CommandFormsValidationError::FormFieldSecretValueVisibilityViolation {
                    form_state_id: form.form_state_id.clone(),
                    argument_name: field.argument_name.clone(),
                    value_visibility: field.value_visibility.as_str().to_owned(),
                },
            );
        }
        if matches!(field.redaction_class, RedactionClass::MetadataSafeDefault) {
            errors.push(
                CommandFormsValidationError::FormFieldSecretRedactionTooLoose {
                    form_state_id: form.form_state_id.clone(),
                    argument_name: field.argument_name.clone(),
                    redaction_class: field.redaction_class.as_str().to_owned(),
                },
            );
        }
    }
}

/// Validates one invocation-review-sheet record against its paired form.
pub fn validate_invocation_review_sheet(
    form: &ParameterFormStateRecord,
    sheet: &InvocationReviewSheetRecord,
) -> Result<(), Vec<CommandFormsValidationError>> {
    let mut errors = Vec::new();
    if sheet.record_kind != INVOCATION_REVIEW_SHEET_RECORD_KIND {
        errors.push(CommandFormsValidationError::ReviewRecordKindMismatch {
            review_sheet_id: sheet.review_sheet_id.clone(),
            actual: sheet.record_kind.clone(),
        });
    }
    if sheet.invocation_review_sheet_schema_version != INVOCATION_REVIEW_SHEET_SCHEMA_VERSION {
        errors.push(CommandFormsValidationError::ReviewSchemaVersionMismatch {
            review_sheet_id: sheet.review_sheet_id.clone(),
            actual: sheet.invocation_review_sheet_schema_version,
        });
    }
    if sheet.command_id != form.command_id {
        errors.push(CommandFormsValidationError::ReviewCommandIdMismatch {
            review_sheet_id: sheet.review_sheet_id.clone(),
            form_command_id: form.command_id.clone(),
            review_command_id: sheet.command_id.clone(),
        });
    }
    if sheet.command_revision_ref != form.command_revision_ref {
        errors.push(CommandFormsValidationError::ReviewCommandRevisionMismatch {
            review_sheet_id: sheet.review_sheet_id.clone(),
            form_revision: form.command_revision_ref.clone(),
            review_revision: sheet.command_revision_ref.clone(),
        });
    }
    if sheet.form_state_id != form.form_state_id {
        errors.push(CommandFormsValidationError::ReviewFormStateIdMismatch {
            review_sheet_id: sheet.review_sheet_id.clone(),
            form_state_id: form.form_state_id.clone(),
            review_form_state_id: sheet.form_state_id.clone(),
        });
    }

    if sheet.side_effects.is_empty() {
        errors.push(CommandFormsValidationError::ReviewSideEffectsEmpty {
            review_sheet_id: sheet.review_sheet_id.clone(),
        });
    }
    if sheet.preview_class_declared.requires_preview() && !sheet.preview_or_dry_run_available {
        errors.push(
            CommandFormsValidationError::ReviewPreviewRequiredButNotAvailable {
                review_sheet_id: sheet.review_sheet_id.clone(),
            },
        );
    }
    if matches!(
        sheet.preview_or_dry_run_class,
        super::PreviewOrDryRunClass::PreviewUnavailableByDesign
    ) && sheet.preview_or_dry_run_available
    {
        errors.push(
            CommandFormsValidationError::ReviewPreviewUnavailableButDryRunAdvertised {
                review_sheet_id: sheet.review_sheet_id.clone(),
            },
        );
    }
    if !sheet.blocked_prerequisites.is_empty()
        && !matches!(
            sheet.execution_intent,
            ExecutionIntentClass::ProposePreviewOnly
                | ExecutionIntentClass::SimulateOrDryRun
                | ExecutionIntentClass::CancelPendingInvocation
        )
    {
        errors.push(
            CommandFormsValidationError::ReviewBlockedPrerequisitesWithApplyIntent {
                review_sheet_id: sheet.review_sheet_id.clone(),
                execution_intent: sheet.execution_intent.as_str().to_owned(),
            },
        );
    }

    for prereq in &sheet.blocked_prerequisites {
        if prereq.repair_hook_ref.hook_id.is_empty() {
            errors.push(
                CommandFormsValidationError::ReviewBlockedReasonMissingRepairHook {
                    review_sheet_id: sheet.review_sheet_id.clone(),
                    blocked_class: prereq.class.as_str().to_owned(),
                },
            );
        }
    }

    // Secret-handling summary required when any field is credential-bearing.
    let any_secret_field = form.fields.iter().any(|f| {
        matches!(f.argument_kind, super::ArgumentKind::CredentialHandleRef)
            || matches!(
                f.field_state_class,
                FieldStateClass::SecretMaskedReadOnly | FieldStateClass::SecretHandleSwapOnly
            )
    });
    if any_secret_field {
        match &sheet.secret_handling_summary {
            None => {
                errors.push(CommandFormsValidationError::ReviewSecretHandlingMissing {
                    review_sheet_id: sheet.review_sheet_id.clone(),
                });
            }
            Some(summary) => {
                let all_handle_only_on_form = form.fields.iter().all(|f| {
                    if matches!(f.argument_kind, super::ArgumentKind::CredentialHandleRef)
                        || matches!(
                            f.field_state_class,
                            FieldStateClass::SecretMaskedReadOnly
                                | FieldStateClass::SecretHandleSwapOnly
                        )
                    {
                        matches!(
                            f.value_visibility,
                            ValueVisibilityClass::ValueHandleOnly
                                | ValueVisibilityClass::ValueMasked
                                | ValueVisibilityClass::ValueOmittedForRedaction
                        )
                    } else {
                        true
                    }
                });
                if !summary.any_secret_bearing_field {
                    errors.push(
                        CommandFormsValidationError::ReviewSecretHandlingHandleOnlyDrift {
                            review_sheet_id: sheet.review_sheet_id.clone(),
                        },
                    );
                }
                if summary.all_handle_only != all_handle_only_on_form {
                    errors.push(
                        CommandFormsValidationError::ReviewSecretHandlingHandleOnlyDrift {
                            review_sheet_id: sheet.review_sheet_id.clone(),
                        },
                    );
                }
            }
        }
    }

    // Provenance summary must cover every field by argument name and label_ref.
    for field in &form.fields {
        let provenance_entry = sheet
            .field_provenance_summary
            .iter()
            .find(|p| p.argument_name == field.argument_name);
        match provenance_entry {
            None => {
                errors.push(
                    CommandFormsValidationError::ReviewProvenanceSummaryFieldMismatch {
                        review_sheet_id: sheet.review_sheet_id.clone(),
                        argument_name: field.argument_name.clone(),
                    },
                );
            }
            Some(entry) => {
                if entry.source_layer_label_ref != field.source_layer_label_ref {
                    errors.push(
                        CommandFormsValidationError::ReviewProvenanceSummaryFieldMismatch {
                            review_sheet_id: sheet.review_sheet_id.clone(),
                            argument_name: field.argument_name.clone(),
                        },
                    );
                }
            }
        }
    }

    // Approval / execution-intent consistency.
    let apply_intents = matches!(
        sheet.execution_intent,
        ExecutionIntentClass::ApplyAfterPreview
            | ExecutionIntentClass::ApplyWithApproval
            | ExecutionIntentClass::ApplyDirectTrustedPath
    );
    if apply_intents
        && matches!(
            sheet.approval_posture_class,
            ApprovalPostureClass::AdminPolicyApprovalRequired
                | ApprovalPostureClass::ManagedOnlyApprovalRequired
        )
        && !matches!(
            sheet.execution_intent,
            ExecutionIntentClass::ApplyWithApproval
        )
    {
        errors.push(CommandFormsValidationError::ReviewApprovalApplyMismatch {
            review_sheet_id: sheet.review_sheet_id.clone(),
            approval_posture: sheet.approval_posture_class.as_str().to_owned(),
            execution_intent: sheet.execution_intent.as_str().to_owned(),
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates a bundle of form + review sheet.
fn validate_bundle(bundle: &CommandFormBundle, errors: &mut Vec<CommandFormsValidationError>) {
    if let Err(form_errs) = validate_parameter_form_state(&bundle.form_state) {
        errors.extend(form_errs);
    }
    if let Err(review_errs) =
        validate_invocation_review_sheet(&bundle.form_state, &bundle.review_sheet)
    {
        errors.extend(review_errs);
    }
}

/// Validates the full catalog.
pub fn validate_command_forms_catalog(
    catalog: &CommandFormsCatalog,
) -> Result<(), Vec<CommandFormsValidationError>> {
    let mut errors = Vec::new();
    if catalog.bundles.is_empty() {
        errors.push(CommandFormsValidationError::EmptyCatalog);
    }
    if catalog.record_kind != COMMAND_FORMS_CATALOG_RECORD_KIND {
        errors.push(CommandFormsValidationError::CatalogRecordKindMismatch {
            actual: catalog.record_kind.clone(),
        });
    }
    if catalog.shared_contract_ref != COMMAND_FORMS_SHARED_CONTRACT_REF {
        errors.push(
            CommandFormsValidationError::CatalogSharedContractRefMismatch {
                actual: catalog.shared_contract_ref.clone(),
            },
        );
    }
    let mut seen = std::collections::HashSet::new();
    for bundle in &catalog.bundles {
        if !seen.insert(bundle.scenario_id.clone()) {
            errors.push(CommandFormsValidationError::DuplicateScenarioId {
                scenario_id: bundle.scenario_id.clone(),
            });
        }
        validate_bundle(bundle, &mut errors);
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_forms::seed::seeded_command_forms_catalog;

    #[test]
    fn seeded_catalog_validates() {
        let catalog = seeded_command_forms_catalog();
        match validate_command_forms_catalog(&catalog) {
            Ok(()) => {}
            Err(errs) => panic!("seeded catalog failed validation: {errs:#?}"),
        }
    }

    #[test]
    fn seeded_bundles_have_paired_ids() {
        let catalog = seeded_command_forms_catalog();
        for bundle in &catalog.bundles {
            assert_eq!(bundle.form_state.command_id, bundle.review_sheet.command_id);
            assert_eq!(
                bundle.form_state.command_revision_ref,
                bundle.review_sheet.command_revision_ref
            );
            assert_eq!(
                bundle.form_state.form_state_id,
                bundle.review_sheet.form_state_id
            );
        }
    }

    #[test]
    fn invocability_map_reflects_blocked_scenarios() {
        let catalog = seeded_command_forms_catalog();
        let map = catalog.invocability_map();
        assert_eq!(
            map.get("recipe_run_blocked_by_policy"),
            Some(&false),
            "policy-pinned recipe step must not be invocable"
        );
        assert_eq!(
            map.get("repair_workspace_blocked_on_runtime_prompt"),
            Some(&false),
            "repair blocked on runtime prompt must not be invocable"
        );
        assert_eq!(
            map.get("bulk_replace_in_files_desktop_apply"),
            Some(&true),
            "clean bulk replace bundle must be invocable"
        );
    }
}
