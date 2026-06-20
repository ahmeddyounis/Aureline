use super::*;
use crate::m5_env_governance::{ClaimMaturity, EvidenceState, RowVerdict, WarmStartPosture};

fn first_party() -> WorkspaceTemplate {
    seeded_workspace_templates()
        .into_iter()
        .find(|t| t.identity.template_id == "env.template.first_party")
        .expect("first-party template exists")
}

fn community() -> WorkspaceTemplate {
    seeded_workspace_templates()
        .into_iter()
        .find(|t| t.identity.template_id == "env.template.community")
        .expect("community template exists")
}

fn local_draft() -> WorkspaceTemplate {
    seeded_workspace_templates()
        .into_iter()
        .find(|t| t.identity.template_id == "env.template.local_draft")
        .expect("local-draft template exists")
}

#[test]
fn every_seeded_template_validates() {
    for template in seeded_workspace_templates() {
        validate_workspace_template(&template).unwrap_or_else(|err| {
            panic!(
                "template {} must validate: {err}",
                template.identity.template_id
            )
        });
    }
}

#[test]
fn seeded_corpus_covers_every_source_class() {
    let fixtures = seeded_workspace_template_fixtures();
    let mut classes = BTreeSet::new();
    for fixture in &fixtures {
        classes.insert(fixture.source_class);
    }
    for required in TemplateSourceClass::ALL {
        assert!(
            classes.contains(&required),
            "fixtures must cover source class {}",
            required.as_str()
        );
    }
}

#[test]
fn every_seeded_fixture_validates() {
    for fixture in seeded_workspace_template_fixtures() {
        validate_workspace_template_fixture(&fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn current_template_certifies_on_inspection() {
    let inspection = inspect_template(&first_party());
    assert_eq!(inspection.verdict, RowVerdict::Certified);
    assert_eq!(inspection.effective_maturity, ClaimMaturity::Stable);
    assert!(inspection.narrow_reason_tokens.is_empty());
    assert!(inspection.composition_narrow_tokens.is_empty());
    assert!(inspection.guardrails_clean);
}

#[test]
fn inspection_wraps_the_same_capsule_inspection() {
    // No forking: the template inspection embeds the exact capsule inspection.
    let template = first_party();
    let template_inspection = inspect_template(&template);
    let capsule_inspection = crate::inspect_environment(&template.environment_capsule);
    assert_eq!(template_inspection.capsule_inspection, capsule_inspection);
}

#[test]
fn partial_attestation_narrows_through_composition() {
    let mut template = community();
    template.trust.attestation_state = EvidenceState::Partial;
    let inspection = inspect_template(&template);
    assert_eq!(inspection.verdict, RowVerdict::Narrowed);
    assert_eq!(inspection.effective_maturity, ClaimMaturity::Beta);
    assert!(
        inspection
            .composition_narrow_tokens
            .iter()
            .any(|t| t.starts_with("trust_attestation_")),
        "composition tokens must name the trust attestation layer"
    );
    // Composition narrowing does not touch warm start.
    assert!(!inspection.warm_start_downgraded);
}

#[test]
fn missing_attestation_withholds_the_template() {
    let mut template = local_draft();
    template.trust.attestation_state = EvidenceState::Missing;
    let inspection = inspect_template(&template);
    assert_eq!(inspection.verdict, RowVerdict::Withheld);
    assert_eq!(inspection.effective_maturity, ClaimMaturity::Withdrawn);
}

#[test]
fn template_inherits_capsule_warm_start_downgrade() {
    // A stale embedded-capsule fingerprint must downgrade warm start on the
    // template exactly as it does on the capsule — no forking.
    let mut template = community();
    template.environment_capsule = crate::seeded_environment_capsules()
        .into_iter()
        .find(|c| c.identity.capsule_id == "env.capsule.container")
        .expect("container capsule");
    template
        .environment_capsule
        .compatibility_fingerprint
        .coverage = EvidenceState::Stale;
    template.claimed_maturity = template.environment_capsule.claimed_maturity;
    template.claimed_warm_start_posture = template.environment_capsule.claimed_warm_start_posture;
    let inspection = inspect_template(&template);
    assert_eq!(inspection.verdict, RowVerdict::Narrowed);
    assert_eq!(
        inspection.effective_warm_start_posture,
        WarmStartPosture::ColdBuild
    );
    assert!(inspection.warm_start_downgraded);
    assert_eq!(
        inspection.warm_start_downgrade_tokens,
        vec!["prebuild_fingerprint_stale".to_owned()]
    );
}

#[test]
fn guardrail_violation_fails_validation() {
    let mut template = first_party();
    template.guardrails.widens_bundle_or_runtime_scope = true;
    let err = validate_workspace_template(&template).expect_err("guardrail must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "template.guardrails"));
}

#[test]
fn widening_bundle_fails_validation() {
    let mut template = first_party();
    template.workflow_bundle_refs[0].widens_execution_scope = true;
    let err = validate_workspace_template(&template).expect_err("widening bundle must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "template.bundle_widens_scope"));
}

#[test]
fn inconsistent_signer_fails_validation() {
    let mut template = first_party();
    template.trust.signer_class = SignerClass::Unsigned;
    let err = validate_workspace_template(&template).expect_err("signer mismatch must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "template.signer_matches_source"));
}

#[test]
fn claim_widened_above_capsule_fails_validation() {
    let mut template = first_party();
    template.claimed_maturity = ClaimMaturity::Beta;
    let err = validate_workspace_template(&template).expect_err("widened claim must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "template.claimed_maturity_matches_capsule"));
}

#[test]
fn desktop_headless_and_support_share_one_object() {
    let template = community();
    let desktop = desktop_template_inspection(&template);
    let headless = headless_template_inspection(&template);
    let support = support_template_inspection(&template);
    assert_eq!(desktop, headless, "desktop and headless must be identical");
    assert_eq!(
        support.inspection, desktop,
        "support export must wrap the same inspection object"
    );
}

#[test]
fn export_is_metadata_first() {
    let template = first_party();
    let export = export_template_metadata(&template);
    assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
    assert_eq!(export.template_digest.value.len(), 64);
    assert!(export.guardrails_clean);
    // The export wraps the capsule's own metadata export.
    assert_eq!(
        export.capsule_export.capsule_id,
        template.environment_capsule.identity.capsule_id
    );
}

#[test]
fn diff_detects_a_version_and_layer_change() {
    let base = first_party();
    let mut target = base.clone();
    target.identity.template_version = 2;
    target.workflow_bundle_refs[0].digest = CapsuleDigest {
        algorithm: "sha256".to_owned(),
        value: "a".repeat(64),
    };
    let diff = diff_templates(&base, &target);
    assert!(!diff.identical);
    let paths: Vec<&str> = diff.changes.iter().map(|c| c.path.as_str()).collect();
    assert!(paths.contains(&"identity.template_version"));
    assert!(paths.iter().any(|p| p.starts_with("workflow_bundle_refs.")));
}

#[test]
fn diff_of_identical_templates_is_empty() {
    let template = community();
    let diff = diff_templates(&template, &template);
    assert!(diff.identical);
    assert!(diff.changes.is_empty());
    assert!(diff.capsule_diff.identical);
}

#[test]
fn install_plan_lists_composed_layers_and_rollback() {
    let template = first_party();
    let plan = plan_template_change(TemplateLifecycleOp::Install, None, Some(&template));
    assert_eq!(plan.op, TemplateLifecycleOp::Install);
    assert!(plan.reviewable);
    assert!(plan.diff.is_none());
    assert!(!plan.composed_layers.is_empty());
    assert_eq!(plan.effective_maturity, Some(ClaimMaturity::Stable));
    assert_eq!(plan.before_version, None);
    assert_eq!(plan.after_version, Some(1));
}

#[test]
fn update_plan_carries_a_diff() {
    let base = first_party();
    let mut target = base.clone();
    target.identity.template_version = 2;
    let plan = plan_template_change(TemplateLifecycleOp::Update, Some(&base), Some(&target));
    assert_eq!(plan.op, TemplateLifecycleOp::Update);
    assert!(plan.diff.is_some());
    assert!(plan.rollback_summary.contains("version 1"));
}

#[test]
fn remove_plan_has_no_resulting_claim() {
    let base = first_party();
    let plan = plan_template_change(TemplateLifecycleOp::Remove, Some(&base), None);
    assert_eq!(plan.op, TemplateLifecycleOp::Remove);
    assert!(plan.composed_layers.is_empty());
    assert_eq!(plan.effective_maturity, None);
    assert!(
        plan.rollback_summary.contains("Reinstall") || plan.rollback_summary.contains("reinstall")
    );
}

#[test]
fn template_round_trips_through_json() {
    let template = community();
    let json = serde_json::to_string(&template).expect("template serializes");
    let back: WorkspaceTemplate = serde_json::from_str(&json).expect("template deserializes");
    assert_eq!(template, back);
}

#[test]
fn fixtures_cover_certified_narrowed_and_withheld() {
    let fixtures = seeded_workspace_template_fixtures();
    let mut verdicts = BTreeSet::new();
    let mut saw_warm_downgrade = false;
    let mut saw_composition_narrow = false;
    for fixture in &fixtures {
        verdicts.insert(fixture.expected_verdict);
        if !fixture.expected_warm_start_downgrade_tokens.is_empty() {
            saw_warm_downgrade = true;
        }
        if !fixture.expected_composition_narrow_tokens.is_empty() {
            saw_composition_narrow = true;
        }
    }
    for required in [
        RowVerdict::Certified,
        RowVerdict::Narrowed,
        RowVerdict::Withheld,
    ] {
        assert!(
            verdicts.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    assert!(
        saw_warm_downgrade,
        "fixtures must cover a warm-start downgrade"
    );
    assert!(
        saw_composition_narrow,
        "fixtures must cover a composition-layer narrowing"
    );
}
