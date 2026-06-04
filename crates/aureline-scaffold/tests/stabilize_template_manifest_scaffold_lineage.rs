use aureline_scaffold::stabilize_template_manifest_scaffold_lineage::{
    DeclaredAction, DeclaredFileClass, DivergenceState, FileImpactSummary,
    GeneratedProjectLineageRecord, HealthFreshnessState, HealthSeverity, HookClass, ParameterKind,
    ParameterSource, PublisherSignature, ResolvedParameter, RollbackBoundary, ScaffoldActor,
    ScaffoldOutcome, ScaffoldPlan, ScaffoldRunRecord, ScaffoldScope, SecretResolution,
    StableScaffoldError, StableScaffoldPacket, SupportClass, SupportedEcosystem, SupportedPlatform,
    TaskClass, TemplateArchetype, TemplateHealthReport, TemplateHealthRow, TemplateManifest,
    TemplateParameter, TemplateSourceClass, TrustEgressPosture, UpdateRebaseCompatibility,
    GENERATED_PROJECT_LINEAGE_SCHEMA_VERSION, SCAFFOLD_PLAN_SCHEMA_VERSION,
    SCAFFOLD_RUN_SCHEMA_VERSION, STABLE_SCAFFOLD_PACKET_RECORD_KIND,
    TEMPLATE_HEALTH_REPORT_SCHEMA_VERSION, TEMPLATE_MANIFEST_SCHEMA_VERSION,
};

#[test]
fn fixture_packet_replays_stable_and_narrowed_cases() {
    let stable: StableScaffoldPacket = serde_json::from_str(include_str!(
        "../../../fixtures/scaffold/stabilize-template-manifest-scaffold-lineage/first_party_rust_service_stable.json"
    ))
    .unwrap();
    stable.validate_stable().unwrap();

    let narrowed: StableScaffoldPacket = serde_json::from_str(include_str!(
        "../../../fixtures/scaffold/stabilize-template-manifest-scaffold-lineage/undeclared_action_narrowed.json"
    ))
    .unwrap();
    assert_eq!(
        narrowed.validate_stable(),
        Err(StableScaffoldError::UndeclaredAction(
            "hidden-bootstrap".to_string()
        ))
    );
}

#[test]
fn preflight_without_create_empty_is_not_stable() {
    let mut packet = StableScaffoldPacket {
        record_kind: STABLE_SCAFFOLD_PACKET_RECORD_KIND.to_string(),
        manifest: TemplateManifest {
            schema_version: TEMPLATE_MANIFEST_SCHEMA_VERSION,
            template_id: "template:extension-panel".to_string(),
            template_version: "2.0.0".to_string(),
            source_class: TemplateSourceClass::ExtensionProvided,
            publisher_signature: PublisherSignature {
                publisher_ref: "publisher:extension:ui-kit".to_string(),
                signature_ref: "signature:extension-panel:2.0.0".to_string(),
                verified: true,
            },
            archetype: TemplateArchetype::ExtensionOrPlugin,
            supported_ecosystems: vec![SupportedEcosystem::NodePnpm],
            supported_platforms: vec![SupportedPlatform::MacosArm64],
            required_parameters: vec![TemplateParameter {
                parameter_id: "panel_name".to_string(),
                kind: ParameterKind::Identifier,
                required: true,
                secret_bearing: false,
            }],
            declared_file_classes: vec![DeclaredFileClass::SourceFile],
            declared_actions: vec![DeclaredAction {
                action_id: "pnpm-install".to_string(),
                hook_class: Some(HookClass::PostCreate),
                task_class: Some(TaskClass::PackageRestore),
                posture: TrustEgressPosture::PackageRegistryEgress,
            }],
            trust_egress_notes: "Restores dependencies only after explicit admission.".to_string(),
            support_class: SupportClass::Experimental,
        },
        plan: ScaffoldPlan {
            schema_version: SCAFFOLD_PLAN_SCHEMA_VERSION,
            plan_id: "plan:extension-panel".to_string(),
            manifest_template_id_ref: "template:extension-panel".to_string(),
            manifest_version_ref: "2.0.0".to_string(),
            target_ref: "target:module:extension-panel".to_string(),
            scope: ScaffoldScope::NewModule,
            resolved_parameters: vec![ResolvedParameter {
                parameter_id: "panel_name".to_string(),
                source: ParameterSource::UserInput,
                secret_resolution: SecretResolution::NotSecret,
            }],
            file_impact: FileImpactSummary {
                create_count: 4,
                modify_count: 1,
                delete_count: 0,
                directory_create_count: 1,
            },
            planned_action_ids: vec!["pnpm-install".to_string()],
            rollback_boundary: RollbackBoundary::LocalHistoryCheckpoint,
            create_empty_alternative: true,
            reviewed_or_exported_before_write: true,
        },
        run: Some(ScaffoldRunRecord {
            schema_version: SCAFFOLD_RUN_SCHEMA_VERSION,
            run_id: "run:extension-panel".to_string(),
            plan_id_ref: "plan:extension-panel".to_string(),
            manifest_template_id_ref: "template:extension-panel".to_string(),
            manifest_version_ref: "2.0.0".to_string(),
            workspace_workset_ref: "workset:plugin-dev".to_string(),
            created_artifact_refs: vec!["artifact:panel".to_string()],
            modified_artifact_refs: vec!["artifact:manifest".to_string()],
            invoked_action_ids: vec!["pnpm-install".to_string()],
            checkpoint_ref: Some("checkpoint:extension-panel".to_string()),
            outcome: ScaffoldOutcome::AppliedWithWarnings,
            actor: ScaffoldActor::Extension,
        }),
        health_report: TemplateHealthReport {
            schema_version: TEMPLATE_HEALTH_REPORT_SCHEMA_VERSION,
            report_id: "health:extension-panel".to_string(),
            manifest_template_id_ref: "template:extension-panel".to_string(),
            manifest_version_ref: "2.0.0".to_string(),
            rows: vec![
                TemplateHealthRow {
                    check_id: "node".to_string(),
                    label: "Node runtime".to_string(),
                    severity: HealthSeverity::BlockedPrerequisite,
                    freshness_state: HealthFreshnessState::Live,
                    scope: vec![SupportedPlatform::MacosArm64],
                    skipped: false,
                    fix_guidance: "Install the supported Node runtime.".to_string(),
                },
                TemplateHealthRow {
                    check_id: "signature".to_string(),
                    label: "Extension signature".to_string(),
                    severity: HealthSeverity::Warning,
                    freshness_state: HealthFreshnessState::PolicyEvaluated,
                    scope: vec![SupportedPlatform::MacosArm64],
                    skipped: false,
                    fix_guidance: "Review extension publisher continuity.".to_string(),
                },
                TemplateHealthRow {
                    check_id: "cache".to_string(),
                    label: "Template cache".to_string(),
                    severity: HealthSeverity::Warning,
                    freshness_state: HealthFreshnessState::Cached,
                    scope: vec![SupportedPlatform::MacosArm64],
                    skipped: false,
                    fix_guidance: "Refresh the template cache when online.".to_string(),
                },
                TemplateHealthRow {
                    check_id: "smoke".to_string(),
                    label: "Optional smoke".to_string(),
                    severity: HealthSeverity::OptionalOptimization,
                    freshness_state: HealthFreshnessState::Unchecked,
                    scope: vec![SupportedPlatform::MacosArm64],
                    skipped: true,
                    fix_guidance: "Run the extension smoke test after generation.".to_string(),
                },
            ],
        },
        lineage: Some(GeneratedProjectLineageRecord {
            schema_version: GENERATED_PROJECT_LINEAGE_SCHEMA_VERSION,
            lineage_id: "lineage:extension-panel".to_string(),
            generated_root_ref: "root:extension-panel".to_string(),
            originating_run_id_ref: "run:extension-panel".to_string(),
            manifest_template_id_ref: "template:extension-panel".to_string(),
            manifest_version_ref: "2.0.0".to_string(),
            workspace_workset_ref: "workset:plugin-dev".to_string(),
            divergence_state: DivergenceState::InSync,
            update_rebase_compatibility: UpdateRebaseCompatibility::InSync,
            latest_health_report_ref: "health:extension-panel".to_string(),
            plain_reviewable_metadata: true,
        }),
    };

    packet.plan.create_empty_alternative = false;

    assert_eq!(
        packet.validate_stable(),
        Err(StableScaffoldError::MissingCreateEmptyAlternative)
    );
}
