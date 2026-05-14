use std::path::{Path, PathBuf};

use aureline_runtime::{
    DependencySection, ManifestScopeClass, MirrorOrOfflineStateClass,
    NodePackageMutationReviewRequest, NodePackageMutationReviewer,
    NodePackageMutationReviewerConfig, PackageAuditResultClass, PackageManagerFamily,
    PackageOperationAlphaPacket, PackageOperationAuditPacket, PackageOperationClass,
    PackageOperationSupportExport, PackageReviewOutcomeClass, RegistryAuthModeClass,
    RegistryFreshnessClass, RegistryRevocationStateClass, RegistrySourceAlphaDescriptor,
    RegistrySourceClass, RollbackPostureClass, ScriptRiskAlphaDescriptor, ScriptRiskClass,
    ValidationTaskClass,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/runtime/packages")
}

fn workspace_fixture_root() -> PathBuf {
    fixture_root().join("node_pnpm_workspace")
}

fn package_mutation_fixture_root() -> PathBuf {
    fixture_root().join("package_mutation_alpha")
}

fn reviewer() -> NodePackageMutationReviewer {
    NodePackageMutationReviewer::new(NodePackageMutationReviewerConfig {
        workspace_id: "workspace:package-alpha".into(),
        workspace_root_ref: "workspace:root:node-pnpm".into(),
        execution_context_ref: "execution-context:pkg-review:node-pnpm".into(),
        target_identity_ref: "target:local-host:node-pnpm".into(),
        workspace_scope_ref: "workspace-scope:apps-web".into(),
        ..NodePackageMutationReviewerConfig::default()
    })
}

fn add_dev_dependency_request() -> NodePackageMutationReviewRequest {
    NodePackageMutationReviewRequest {
        operation_class: PackageOperationClass::InstallNewDependency,
        dependency_section: DependencySection::DevDependencies,
        package_name: "eslint-plugin-unused-imports".into(),
        package_coordinate_ref: "pkg:npm:eslint_plugin_unused_imports".into(),
        requested_requirement: "^4.1.0".into(),
        active_manifest_path: "apps/web/package.json".into(),
        manifest_scope_class: ManifestScopeClass::WorkspaceMemberManifest,
        workspace_member_ref: Some("workspace-member:apps-web".into()),
        module_identity_ref: Some("module:apps-web".into()),
        actor_ref: "actor:local-user".into(),
        command_id_ref: "command:package-install.review".into(),
        issuing_surface: "desktop_review_surface".into(),
        policy_epoch_ref: "policy-epoch:local:2026-05-14".into(),
        registry_source: RegistrySourceAlphaDescriptor::public_default(PackageManagerFamily::Pnpm),
        script_risk: ScriptRiskAlphaDescriptor::no_scripts(),
        validation_tasks: vec![
            ValidationTaskClass::Typecheck,
            ValidationTaskClass::Build,
            ValidationTaskClass::UnitTest,
            ValidationTaskClass::DependencyAudit,
        ],
    }
}

#[test]
fn node_pnpm_fixture_matches_runtime_review_packet() {
    let manifest_payload =
        std::fs::read_to_string(package_mutation_fixture_root().join("manifest.json"))
            .expect("manifest fixture reads");
    let manifest: FixtureManifest =
        serde_json::from_str(&manifest_payload).expect("manifest fixture parses");
    assert_eq!(manifest.status, "protected");
    assert_eq!(manifest.fixture_family, "package_mutation_alpha");

    let case_path = package_mutation_fixture_root().join(&manifest.case_files[0].file);
    let payload = std::fs::read_to_string(&case_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", case_path.display()));
    let fixture: PackageOperationFixture =
        serde_json::from_str(&payload).expect("package operation fixture parses");

    let generated = reviewer().review_workspace(
        &workspace_fixture_root(),
        add_dev_dependency_request(),
        "2026-05-14T17:00:00Z",
    );
    assert_eq!(generated, fixture.packet);
    assert!(generated.validation_issues().is_empty());
    assert!(!generated.blocks_apply());

    assert_eq!(
        generated.manifest_scope.active_manifest_path,
        fixture.expect.manifest_path
    );
    assert_eq!(
        generated.manifest_scope.lockfile_refs[0].lockfile_path,
        fixture.expect.lockfile_path
    );
    assert_eq!(
        generated.registry_source.registry_source_token,
        fixture.expect.registry_source_token
    );
    assert_eq!(
        generated.registry_source.auth_mode_token,
        fixture.expect.auth_mode_token
    );
    assert_eq!(
        generated.script_risk.script_risk_token,
        fixture.expect.script_risk_token
    );
    assert_eq!(
        generated.lockfile_impact.mutation_mode_token,
        fixture.expect.lockfile_mutation_mode_token
    );
    assert_eq!(
        generated.review_outcome_token,
        fixture.expect.review_outcome_token
    );

    let audit = generated.audit_packet(
        "pkg-audit:node-pnpm-add-dev-dep:preview",
        PackageAuditResultClass::PreviewCreated,
        "2026-05-14T17:00:01Z",
    );
    assert_eq!(audit, fixture.audit_packet);

    let export = PackageOperationSupportExport::from_packets(
        "support:package-operation:node-pnpm-add-dev-dep",
        "2026-05-14T17:00:02Z",
        &[generated],
    );
    assert_eq!(export, fixture.support_export);
    assert!(export.redaction_safe);

    let encoded = serde_json::to_string(&export).expect("support export serializes");
    assert!(!encoded.contains("react"));
    assert!(!encoded.contains("vite"));
    assert!(!encoded.contains("registry.npmjs"));
    assert!(!encoded.contains("secret_value"));
    assert!(!encoded.contains("package-lock body"));
}

#[test]
fn native_build_risk_blocks_apply_until_consent_is_present() {
    let mut request = add_dev_dependency_request();
    request.package_name = "sharp".into();
    request.package_coordinate_ref = "pkg:npm:sharp".into();
    request.requested_requirement = "^0.33.5".into();
    request.script_risk = ScriptRiskAlphaDescriptor::new(
        ScriptRiskClass::NativeCompilationRequiredLocalToolchain,
        "Native compilation requires the local toolchain; user consent is required before apply.",
    );
    request.script_risk.native_toolchain_ref = Some("native-toolchain:node-gyp:local".into());

    let packet =
        reviewer().review_workspace(&workspace_fixture_root(), request, "2026-05-14T17:10:00Z");
    assert_eq!(
        packet.review_outcome_class,
        PackageReviewOutcomeClass::ReviewBlockedPendingNativeBuildConsent
    );
    assert_eq!(
        packet.rollback.rollback_posture_class,
        RollbackPostureClass::RollbackBlockedNativeArtifactsMustBeRecompiled
    );
    assert!(packet.blocks_apply());
    assert!(packet
        .blocked_reason_tokens
        .contains(&"script_risk_requires_consent".to_owned()));

    let export = PackageOperationSupportExport::from_packets(
        "support:package-operation:native-build",
        "2026-05-14T17:10:01Z",
        &[packet],
    );
    assert!(export.redaction_safe);
    assert_eq!(
        export.rows[0].script_risk_token,
        "native_compilation_required_local_toolchain"
    );
}

#[test]
fn raw_registry_secret_observation_blocks_mutating_review() {
    let mut registry_source = RegistrySourceAlphaDescriptor::new(
        "registry-source:private-raw-secret-observed",
        PackageManagerFamily::Pnpm,
        RegistrySourceClass::PrivateInternalRegistry,
        RegistryAuthModeClass::RegistryAuthUnknownRequiresReview,
        RegistryFreshnessClass::StaleRequiresReview,
        RegistryRevocationStateClass::RevocationCheckUnavailableStale,
        MirrorOrOfflineStateClass::OnlineDefaultOriginAdmissible,
        "Private registry auth is unresolved because raw workspace secret material was observed.",
    );
    registry_source.raw_secret_observed = true;

    let mut request = add_dev_dependency_request();
    request.registry_source = registry_source;
    let packet =
        reviewer().review_workspace(&workspace_fixture_root(), request, "2026-05-14T17:20:00Z");

    assert_eq!(
        packet.review_outcome_class,
        PackageReviewOutcomeClass::ReviewBlockedPendingPolicy
    );
    assert!(packet.blocks_apply());
    assert!(packet
        .blocked_reason_tokens
        .contains(&"raw_registry_secret_observed".to_owned()));
    assert!(packet
        .blocked_reason_tokens
        .contains(&"registry_auth_blocked".to_owned()));

    let audit = packet.audit_packet(
        "pkg-audit:raw-secret-blocked:preview",
        PackageAuditResultClass::ApplyBlockedByReview,
        "2026-05-14T17:20:01Z",
    );
    assert_eq!(audit.result_token, "apply_blocked_by_review");
    assert!(audit.export_safe);
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    status: String,
    fixture_family: String,
    case_files: Vec<FixtureCaseFile>,
}

#[derive(Debug, Deserialize)]
struct FixtureCaseFile {
    file: String,
}

#[derive(Debug, Deserialize)]
struct PackageOperationFixture {
    packet: PackageOperationAlphaPacket,
    audit_packet: PackageOperationAuditPacket,
    support_export: PackageOperationSupportExport,
    expect: PackageOperationFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct PackageOperationFixtureExpect {
    manifest_path: String,
    lockfile_path: String,
    registry_source_token: String,
    auth_mode_token: String,
    script_risk_token: String,
    lockfile_mutation_mode_token: String,
    review_outcome_token: String,
}
