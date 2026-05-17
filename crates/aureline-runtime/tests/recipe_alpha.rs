//! Fixture-driven coverage for the declarative recipe alpha page.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    RecipeAlphaPage, RecipeApprovalClass, RecipeAttributionSurfaceClass, RecipeAuditEventClass,
    RecipePreviewRequirementClass, RecipeRunDispositionClass, RecipeStepDispositionClass,
    RecipeTrustGateClass, RecipeWriteClass, StepCommandLineageClass,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/runtime/recipe_alpha/page.json")
}

fn load_page() -> RecipeAlphaPage {
    let text = fs::read_to_string(fixture_path()).expect("read recipe alpha fixture");
    serde_json::from_str(&text).expect("parse recipe alpha fixture")
}

#[test]
fn alpha_fixture_validates() {
    let page = load_page();
    let report = page.validate();
    assert!(
        report.passed,
        "recipe alpha fixture failed validation: {:#?}",
        report.findings
    );
}

#[test]
fn fixture_covers_run_dispositions() {
    let page = load_page();
    let report = page.validate();
    for disposition in [
        RecipeRunDispositionClass::ProceedLocalEditorOnly,
        RecipeRunDispositionClass::ProceedAfterRecipeApproval,
        RecipeRunDispositionClass::PreviewRequiredBeforeApply,
        RecipeRunDispositionClass::DowngradedToObserverNoMutation,
        RecipeRunDispositionClass::PromotedToFullRecipeRun,
        RecipeRunDispositionClass::DeniedUnsafeRecipe,
    ] {
        assert!(
            report.coverage.run_dispositions.contains(&disposition),
            "missing run-disposition coverage: {disposition:?}"
        );
    }
}

#[test]
fn fixture_covers_audit_event_classes() {
    let page = load_page();
    let report = page.validate();
    for class in [
        RecipeAuditEventClass::RecipeRunStarted,
        RecipeAuditEventClass::RecipeRunCompleted,
        RecipeAuditEventClass::RecipeDenied,
        RecipeAuditEventClass::RecipeRunPromotedToApprovedRun,
        RecipeAuditEventClass::AttributionMinted,
    ] {
        assert!(
            report.coverage.audit_event_classes.contains(&class),
            "missing audit-event coverage: {class:?}"
        );
    }
}

#[test]
fn fixture_covers_attribution_surfaces() {
    let page = load_page();
    let report = page.validate();
    for surface in [
        RecipeAttributionSurfaceClass::SupportExport,
        RecipeAttributionSurfaceClass::ActivityHistory,
        RecipeAttributionSurfaceClass::AdminAuditExport,
    ] {
        assert!(
            report.coverage.attribution_surfaces.contains(&surface),
            "missing attribution-surface coverage: {surface:?}"
        );
    }
}

#[test]
fn fixture_covers_step_lineages() {
    let page = load_page();
    let report = page.validate();
    for lineage in [
        StepCommandLineageClass::CoreCommand,
        StepCommandLineageClass::CliVerb,
        StepCommandLineageClass::ProviderAction,
    ] {
        assert!(
            report.coverage.step_lineages.contains(&lineage),
            "missing step-lineage coverage: {lineage:?}"
        );
    }
}

#[test]
fn every_mutating_step_requires_preview_and_approval() {
    let page = load_page();
    for definition in &page.definitions {
        for step in &definition.steps {
            let has_mutation = step.write_classes.iter().any(|w| w.is_mutating());
            if has_mutation {
                assert!(
                    step.preview_requirement.requires_preview(),
                    "step {} mutates but does not require preview",
                    step.step_id
                );
                assert!(
                    !matches!(step.approval_class, RecipeApprovalClass::NoApprovalRequired),
                    "step {} mutates but does not require approval",
                    step.step_id
                );
            }
            let has_provider = step.write_classes.iter().any(|w| w.is_provider_facing());
            if has_provider {
                assert!(
                    matches!(
                        step.preview_requirement,
                        RecipePreviewRequirementClass::PreviewRequiredProviderMutation
                            | RecipePreviewRequirementClass::PreviewRequiredBeforeApply
                    ),
                    "provider step {} missing provider-preview",
                    step.step_id
                );
                assert!(
                    matches!(
                        step.approval_class,
                        RecipeApprovalClass::RecipeApprovalRequired
                            | RecipeApprovalClass::AdminSignedApprovalRequired
                            | RecipeApprovalClass::SingleStepApprovalRequired
                    ),
                    "provider step {} missing approval",
                    step.step_id
                );
            }
        }
    }
}

#[test]
fn every_run_resolves_to_a_known_definition() {
    let page = load_page();
    let definition_ids: std::collections::HashSet<&str> = page
        .definitions
        .iter()
        .map(|d| d.definition_id.as_str())
        .collect();
    for run in &page.runs {
        assert!(
            definition_ids.contains(run.definition_ref.as_str()),
            "run {} references unknown definition {}",
            run.run_id,
            run.definition_ref
        );
    }
}

#[test]
fn no_definition_or_run_masks_remote_attach_degraded_state() {
    let page = load_page();
    for definition in &page.definitions {
        assert!(
            !definition.remote_attach_degraded_state_masked,
            "definition {} masks remote-attach degraded state",
            definition.definition_id
        );
    }
    for run in &page.runs {
        assert!(
            !run.remote_attach_degraded_state_masked,
            "run {} masks remote-attach degraded state",
            run.run_id
        );
    }
}

#[test]
fn no_step_or_run_silently_widens_authority() {
    let page = load_page();
    for definition in &page.definitions {
        assert!(!definition.silent_authority_widening_taken);
    }
    for run in &page.runs {
        assert!(!run.silent_authority_widening_taken);
    }
}

#[test]
fn trust_gate_is_never_managed_only_denied() {
    let page = load_page();
    for definition in &page.definitions {
        assert!(!matches!(
            definition.trust_gate,
            RecipeTrustGateClass::ManagedOnlyDenied
        ));
    }
    for run in &page.runs {
        assert!(!matches!(
            run.trust_gate_observed,
            RecipeTrustGateClass::ManagedOnlyDenied
        ));
    }
}

#[test]
fn no_step_uses_credential_mutation_denied_write_class() {
    let page = load_page();
    for definition in &page.definitions {
        for step in &definition.steps {
            for write_class in &step.write_classes {
                assert!(
                    !matches!(write_class, RecipeWriteClass::CredentialMutationDenied),
                    "step {} declares credential_mutation_denied",
                    step.step_id
                );
            }
        }
    }
}

#[test]
fn support_export_projection_redacts_raw_and_internal_fields() {
    let page = load_page();
    let projection = page.support_export_projection();
    let json = serde_json::to_string(&projection).expect("projection serializes");
    assert_eq!(projection.record_kind, "recipe_alpha_support_export");
    assert!(!json.contains("raw_branch_mutation"));
    assert!(!json.contains("raw_worktree_mutation"));
    assert!(!json.contains("raw_provider_payload"));
    assert!(!json.contains("raw_credential"));
    assert!(!json.contains("silent_authority_widening"));
    assert!(!json.contains("remote_attach_degraded_state"));
    assert_eq!(
        projection.definition_summaries.len(),
        page.definitions.len()
    );
    assert_eq!(projection.run_summaries.len(), page.runs.len());
    assert_eq!(projection.audit_summaries.len(), page.audit_events.len());
    assert_eq!(
        projection.attribution_summaries.len(),
        page.attributions.len()
    );
}

#[test]
fn editing_a_provider_step_to_drop_approval_is_rejected_after_edit() {
    let mut page = load_page();
    let definition = page
        .definitions
        .iter_mut()
        .find(|d| d.definition_id == "recipe_alpha.def.publish_draft_pr.01")
        .expect("publish-draft-pr definition present");
    let step = definition
        .steps
        .iter_mut()
        .find(|s| s.step_id == "recipe_alpha.def.publish_draft_pr.01.step.open_draft")
        .expect("open-draft step present");
    step.approval_class = RecipeApprovalClass::NoApprovalRequired;
    definition
        .declared_approval_classes
        .push(RecipeApprovalClass::NoApprovalRequired);
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "recipe_alpha.step_provider_mutation_missing_approval"));
}

#[test]
fn editing_a_mutating_step_to_drop_preview_is_rejected_after_edit() {
    let mut page = load_page();
    let definition = page
        .definitions
        .iter_mut()
        .find(|d| d.definition_id == "recipe_alpha.def.normalize_imports.01")
        .expect("normalize-imports definition present");
    let step = definition
        .steps
        .iter_mut()
        .find(|s| s.step_id == "recipe_alpha.def.normalize_imports.01.step.sort")
        .expect("sort step present");
    step.preview_requirement = RecipePreviewRequirementClass::NoPreviewRequired;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "recipe_alpha.step_mutation_missing_preview"));
}

#[test]
fn editing_a_denied_run_to_drop_reason_is_rejected_after_edit() {
    let mut page = load_page();
    let run = page
        .runs
        .iter_mut()
        .find(|r| r.run_id == "recipe_alpha.run.run_test_suite.denied.01")
        .expect("denied run present");
    run.denial_reason = None;
    run.denial_reason_label = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "recipe_alpha.run_denial_reason_missing"));
}

#[test]
fn editing_a_promoted_run_to_drop_promoted_run_ref_is_rejected_after_edit() {
    let mut page = load_page();
    let run = page
        .runs
        .iter_mut()
        .find(|r| r.run_id == "recipe_alpha.run.publish_draft_pr.promoted.01")
        .expect("promoted run present");
    run.promoted_run_ref = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "recipe_alpha.run_promoted_run_ref_missing"));
}

#[test]
fn editing_a_downgraded_run_to_drop_target_label_is_rejected_after_edit() {
    let mut page = load_page();
    let run = page
        .runs
        .iter_mut()
        .find(|r| r.run_id == "recipe_alpha.run.normalize_imports.downgraded.01")
        .expect("downgraded run present");
    run.downgrade_target_label = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "recipe_alpha.run_downgrade_target_label_missing"));
}

#[test]
fn editing_a_run_to_mask_remote_attach_degraded_state_is_rejected_after_edit() {
    let mut page = load_page();
    let run = page
        .runs
        .iter_mut()
        .next()
        .expect("at least one run present");
    run.remote_attach_degraded_state_masked = true;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "recipe_alpha.run_remote_attach_degraded_masked"));
}

#[test]
fn dropping_attribution_surface_breaks_required_coverage_after_edit() {
    let mut page = load_page();
    page.attributions.retain(|a| {
        !matches!(
            a.attribution_surface,
            RecipeAttributionSurfaceClass::ActivityHistory
        )
    });
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "recipe_alpha.coverage_attribution_surface_missing"));
}

#[test]
fn audit_event_referencing_unknown_run_is_rejected_after_edit() {
    let mut page = load_page();
    let event = page
        .audit_events
        .iter_mut()
        .find(|e| e.event_class == RecipeAuditEventClass::RecipeRunCompleted)
        .expect("completed audit event present");
    event.run_ref = Some("recipe_alpha.run.does_not_exist".to_string());
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|f| f.check_id == "recipe_alpha.audit_event_run_ref_unknown"));
}

#[test]
fn step_disposition_resolves_to_a_known_step() {
    let page = load_page();
    let step_ids: std::collections::HashSet<&str> = page
        .definitions
        .iter()
        .flat_map(|d| d.steps.iter().map(|s| s.step_id.as_str()))
        .collect();
    for run in &page.runs {
        for disposition in &run.step_dispositions {
            assert!(
                step_ids.contains(disposition.step_ref.as_str()),
                "run {} references unknown step {}",
                run.run_id,
                disposition.step_ref
            );
        }
    }
}

#[test]
fn denied_step_dispositions_cite_a_denial_reason() {
    let page = load_page();
    for run in &page.runs {
        for disposition in &run.step_dispositions {
            if matches!(
                disposition.step_disposition,
                RecipeStepDispositionClass::DeniedUnsafeStep
            ) {
                assert!(
                    disposition.denial_reason.is_some(),
                    "denied step disposition on run {} must cite a denial_reason",
                    run.run_id
                );
            }
        }
    }
}
