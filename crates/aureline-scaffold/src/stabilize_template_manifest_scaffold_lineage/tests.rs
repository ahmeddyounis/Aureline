use super::*;

fn stable_packet() -> StableScaffoldPacket {
    StableScaffoldPacket {
        record_kind: STABLE_SCAFFOLD_PACKET_RECORD_KIND.to_string(),
        manifest: TemplateManifest {
            schema_version: TEMPLATE_MANIFEST_SCHEMA_VERSION,
            template_id: "template:rust-service".to_string(),
            template_version: "1.4.0".to_string(),
            source_class: TemplateSourceClass::FirstParty,
            publisher_signature: PublisherSignature {
                publisher_ref: "publisher:aureline:first-party".to_string(),
                signature_ref: "signature:template:rust-service:1.4.0".to_string(),
                verified: true,
            },
            archetype: TemplateArchetype::BackendService,
            supported_ecosystems: vec![SupportedEcosystem::RustCargo],
            supported_platforms: vec![
                SupportedPlatform::MacosArm64,
                SupportedPlatform::LinuxX86_64,
            ],
            required_parameters: vec![
                TemplateParameter {
                    parameter_id: "service_name".to_string(),
                    kind: ParameterKind::Identifier,
                    required: true,
                    secret_bearing: false,
                },
                TemplateParameter {
                    parameter_id: "registry_token".to_string(),
                    kind: ParameterKind::SecretHandle,
                    required: true,
                    secret_bearing: true,
                },
            ],
            declared_file_classes: vec![
                DeclaredFileClass::SourceFile,
                DeclaredFileClass::DependencyManifest,
                DeclaredFileClass::LineageMetadata,
            ],
            declared_actions: vec![
                DeclaredAction {
                    action_id: "cargo-fetch".to_string(),
                    hook_class: None,
                    task_class: Some(TaskClass::PackageRestore),
                    posture: TrustEgressPosture::PackageRegistryEgress,
                },
                DeclaredAction {
                    action_id: "cargo-test".to_string(),
                    hook_class: Some(HookClass::HealthCheck),
                    task_class: Some(TaskClass::Test),
                    posture: TrustEgressPosture::LocalNoEgress,
                },
            ],
            trust_egress_notes:
                "Uses Cargo registry access only when dependency restore is admitted.".to_string(),
            support_class: SupportClass::OfficiallySupported,
        },
        plan: ScaffoldPlan {
            schema_version: SCAFFOLD_PLAN_SCHEMA_VERSION,
            plan_id: "plan:rust-service:preview".to_string(),
            manifest_template_id_ref: "template:rust-service".to_string(),
            manifest_version_ref: "1.4.0".to_string(),
            target_ref: "target:new-workspace:service".to_string(),
            scope: ScaffoldScope::NewProject,
            resolved_parameters: vec![
                ResolvedParameter {
                    parameter_id: "service_name".to_string(),
                    source: ParameterSource::UserInput,
                    secret_resolution: SecretResolution::NotSecret,
                },
                ResolvedParameter {
                    parameter_id: "registry_token".to_string(),
                    source: ParameterSource::SecretBrokerHandle,
                    secret_resolution: SecretResolution::BrokerHandleOnly,
                },
            ],
            file_impact: FileImpactSummary {
                create_count: 12,
                modify_count: 0,
                delete_count: 0,
                directory_create_count: 4,
            },
            planned_action_ids: vec!["cargo-fetch".to_string(), "cargo-test".to_string()],
            rollback_boundary: RollbackBoundary::WorkspaceCheckpoint,
            create_empty_alternative: true,
            reviewed_or_exported_before_write: true,
        },
        run: Some(ScaffoldRunRecord {
            schema_version: SCAFFOLD_RUN_SCHEMA_VERSION,
            run_id: "run:rust-service:001".to_string(),
            plan_id_ref: "plan:rust-service:preview".to_string(),
            manifest_template_id_ref: "template:rust-service".to_string(),
            manifest_version_ref: "1.4.0".to_string(),
            workspace_workset_ref: "workset:local:service".to_string(),
            created_artifact_refs: vec!["artifact:src-main".to_string()],
            modified_artifact_refs: Vec::new(),
            invoked_action_ids: vec!["cargo-fetch".to_string(), "cargo-test".to_string()],
            checkpoint_ref: Some("checkpoint:scaffold:001".to_string()),
            outcome: ScaffoldOutcome::Applied,
            actor: ScaffoldActor::User,
        }),
        health_report: TemplateHealthReport {
            schema_version: TEMPLATE_HEALTH_REPORT_SCHEMA_VERSION,
            report_id: "health:rust-service:001".to_string(),
            manifest_template_id_ref: "template:rust-service".to_string(),
            manifest_version_ref: "1.4.0".to_string(),
            rows: vec![
                TemplateHealthRow {
                    check_id: "toolchain".to_string(),
                    label: "Rust toolchain".to_string(),
                    severity: HealthSeverity::BlockedPrerequisite,
                    freshness_state: HealthFreshnessState::Live,
                    scope: vec![SupportedPlatform::MacosArm64],
                    skipped: false,
                    fix_guidance: "Install the selected Rust toolchain before apply.".to_string(),
                },
                TemplateHealthRow {
                    check_id: "template-freshness".to_string(),
                    label: "Template freshness".to_string(),
                    severity: HealthSeverity::Warning,
                    freshness_state: HealthFreshnessState::Cached,
                    scope: vec![SupportedPlatform::MacosArm64],
                    skipped: false,
                    fix_guidance: "Refresh the signed template bundle when online.".to_string(),
                },
                TemplateHealthRow {
                    check_id: "policy-egress".to_string(),
                    label: "Registry egress policy".to_string(),
                    severity: HealthSeverity::BlockedPrerequisite,
                    freshness_state: HealthFreshnessState::PolicyEvaluated,
                    scope: vec![SupportedPlatform::MacosArm64],
                    skipped: false,
                    fix_guidance: "Ask policy to admit Cargo registry egress or skip setup."
                        .to_string(),
                },
                TemplateHealthRow {
                    check_id: "optional-smoke".to_string(),
                    label: "Optional smoke run".to_string(),
                    severity: HealthSeverity::OptionalOptimization,
                    freshness_state: HealthFreshnessState::Unchecked,
                    scope: vec![SupportedPlatform::MacosArm64],
                    skipped: true,
                    fix_guidance: "Run smoke test after generation.".to_string(),
                },
            ],
        },
        lineage: Some(GeneratedProjectLineageRecord {
            schema_version: GENERATED_PROJECT_LINEAGE_SCHEMA_VERSION,
            lineage_id: "lineage:rust-service:001".to_string(),
            generated_root_ref: "workspace-root:service".to_string(),
            originating_run_id_ref: "run:rust-service:001".to_string(),
            manifest_template_id_ref: "template:rust-service".to_string(),
            manifest_version_ref: "1.4.0".to_string(),
            workspace_workset_ref: "workset:local:service".to_string(),
            divergence_state: DivergenceState::LocalOverrides,
            update_rebase_compatibility: UpdateRebaseCompatibility::ThreeWayUpdateAvailable,
            latest_health_report_ref: "health:rust-service:001".to_string(),
            plain_reviewable_metadata: true,
        }),
    }
}

#[test]
fn stable_packet_validates_full_manifest_plan_run_health_and_lineage() {
    stable_packet().validate_stable().unwrap();
}

#[test]
fn undeclared_run_actions_are_rejected() {
    let mut packet = stable_packet();
    packet
        .run
        .as_mut()
        .unwrap()
        .invoked_action_ids
        .push("hidden-bootstrap".to_string());

    assert_eq!(
        packet.validate_stable(),
        Err(StableScaffoldError::UndeclaredAction(
            "hidden-bootstrap".to_string()
        ))
    );
}

#[test]
fn secret_parameters_must_resolve_through_broker_handles() {
    let mut packet = stable_packet();
    let secret = packet
        .plan
        .resolved_parameters
        .iter_mut()
        .find(|parameter| parameter.parameter_id == "registry_token")
        .unwrap();
    secret.secret_resolution = SecretResolution::RawSecretRejected;

    assert_eq!(
        packet.validate_stable(),
        Err(StableScaffoldError::RawSecretOrMissingSecretHandle(
            "registry_token".to_string()
        ))
    );
}

#[test]
fn health_rows_must_preserve_freshness_states() {
    let mut packet = stable_packet();
    let optional = packet
        .health_report
        .rows
        .iter_mut()
        .find(|row| row.severity == HealthSeverity::OptionalOptimization)
        .unwrap();
    optional.freshness_state = HealthFreshnessState::Cached;

    assert_eq!(
        packet.validate_stable(),
        Err(StableScaffoldError::HealthFreshnessNotPreserved)
    );
}

#[test]
fn lineage_update_truth_requires_plain_three_way_state() {
    let mut packet = stable_packet();
    let lineage = packet.lineage.as_mut().unwrap();
    lineage.update_rebase_compatibility = UpdateRebaseCompatibility::NoUpdatePath;

    assert_eq!(
        packet.validate_stable(),
        Err(StableScaffoldError::MissingThreeWayUpdateRebaseTruth)
    );
}
