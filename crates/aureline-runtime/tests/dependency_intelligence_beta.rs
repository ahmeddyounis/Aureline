use std::path::{Path, PathBuf};

use aureline_runtime::{
    AdvisoryAffectedRange, AdvisoryLifecycleClass, AdvisorySeverityClass, AdvisorySourceClass,
    AdvisoryTruthClass, DebtReleaseVisibilityClass, DependencyAdvisoryRecord,
    DependencyAdvisoryRecordSeed, DependencyDebtKindClass, DependencyDebtPacket,
    DependencyDebtPacketSeed, DependencyDebtRow, DependencyEdgeRecord, DependencyFreshnessClass,
    DependencyGraphRecord, DependencyIntelligenceViolation, DependencyProvenanceClass,
    DependencyRecord, DependencyRecordSeed, DependencyRelationshipClass, DependencyResolutionClass,
    DependencySection, DependencySourceClass, LicenseDecisionClass, LockfileMutationPreview,
    LockfilePreviewActionClass, ManifestScopeClass, MirrorOrOfflineStateClass,
    NodePackageMutationReviewRequest, NodePackageMutationReviewer,
    NodePackageMutationReviewerConfig, PackageManagerFamily, PackageOperationClass,
    RegistryAuthModeClass, RegistryFreshnessClass, RegistryRevocationStateClass,
    RegistrySourceAlphaDescriptor, RegistrySourceClass, ScriptRiskAlphaDescriptor, SuppressionRef,
    SuppressionStateClass, ValidationTaskClass,
};
use serde::Deserialize;

fn repo_fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn workspace_fixture_root() -> PathBuf {
    repo_fixture_root().join("runtime/packages/node_pnpm_workspace")
}

fn dependency_fixture_path() -> PathBuf {
    repo_fixture_root()
        .join("dependencies/advisory_and_lockfile_review/enterprise_mirror_security_patch.json")
}

fn reviewer() -> NodePackageMutationReviewer {
    NodePackageMutationReviewer::new(NodePackageMutationReviewerConfig {
        workspace_id: "workspace:dependency-review".into(),
        workspace_root_ref: "workspace:root:node-pnpm".into(),
        execution_context_ref: "execution-context:dependency-review:node-pnpm".into(),
        target_identity_ref: "target:local-host:node-pnpm".into(),
        workspace_scope_ref: "workspace-scope:apps-web".into(),
        ..NodePackageMutationReviewerConfig::default()
    })
}

fn update_vite_request() -> NodePackageMutationReviewRequest {
    let registry_source = RegistrySourceAlphaDescriptor::new(
        "registry-source:enterprise-npm-mirror",
        PackageManagerFamily::Pnpm,
        RegistrySourceClass::CustomerOperatedMirror,
        RegistryAuthModeClass::MirrorOrOfflineNoAuthRequired,
        RegistryFreshnessClass::MirrorSnapshotWithinPolicyWindow,
        RegistryRevocationStateClass::RevocationCheckedFromMirror,
        MirrorOrOfflineStateClass::OnlineMirrorPinnedNoDirectOrigin,
        "Enterprise mirror snapshot is current for review; direct origin is not used.",
    );

    NodePackageMutationReviewRequest {
        operation_class: PackageOperationClass::UpgradeExistingDependency,
        dependency_section: DependencySection::DevDependencies,
        package_name: "vite".into(),
        package_coordinate_ref: "pkg:npm:vite".into(),
        requested_requirement: "5.4.3".into(),
        active_manifest_path: "apps/web/package.json".into(),
        manifest_scope_class: ManifestScopeClass::WorkspaceMemberManifest,
        workspace_member_ref: Some("workspace-member:apps-web".into()),
        module_identity_ref: Some("module:apps-web".into()),
        actor_ref: "actor:local-user".into(),
        command_id_ref: "command:dependency-advisory.review-fix".into(),
        issuing_surface: "dependency_security_lane".into(),
        policy_epoch_ref: "policy-epoch:dependency-review:2026-05-18".into(),
        registry_source,
        script_risk: ScriptRiskAlphaDescriptor::no_scripts(),
        validation_tasks: vec![
            ValidationTaskClass::Build,
            ValidationTaskClass::UnitTest,
            ValidationTaskClass::SecurityAudit,
            ValidationTaskClass::DependencyAudit,
            ValidationTaskClass::LicenseAudit,
        ],
    }
}

fn generated_fixture() -> DependencyIntelligenceFixture {
    let vite_ref = "dependency:pnpm:apps-web:vite";
    let react_ref = "dependency:pnpm:apps-web:react";
    let advisory_ref = "advisory:dependency.vite_mirror_snapshot";
    let suppression_ref = "suppression:dependency.vite.false-positive-window";

    let vite = DependencyRecord::new(DependencyRecordSeed {
        dependency_id: vite_ref.into(),
        package_manager_family_token: "pnpm".into(),
        package_coordinate_ref: "pkg:npm:vite".into(),
        package_name: "vite".into(),
        relationship_class: DependencyRelationshipClass::Direct,
        source_class: DependencySourceClass::MirrorRegistryDependency,
        freshness_class: DependencyFreshnessClass::MirroredData,
        provenance_class: DependencyProvenanceClass::MirrorSnapshot,
        resolution_class: DependencyResolutionClass::RequestedAndResolved,
        requested_requirement: Some("5.4.0".into()),
        resolved_version: Some("5.4.0".into()),
        manifest_ref: Some("manifest:apps_web_package_json".into()),
        manifest_path: Some("apps/web/package.json".into()),
        lockfile_ref: Some("lockfile:pnpm_lock_yaml".into()),
        lockfile_path: Some("pnpm-lock.yaml".into()),
        workspace_member_ref: Some("workspace-member:apps-web".into()),
        registry_source_ref: Some("registry-source:enterprise-npm-mirror".into()),
        advisory_refs: vec![advisory_ref.into()],
        license_decision_class: LicenseDecisionClass::AllowedWithNotice,
        notice_implication_refs: vec!["notice:third_party:vite".into()],
        evidence_refs: vec![
            "manifest:apps_web_package_json".into(),
            "lockfile:pnpm_lock_yaml".into(),
            "mirror-snapshot:npm:2026-05-18T08:30:00Z".into(),
        ],
    });
    let react = DependencyRecord::new(DependencyRecordSeed {
        dependency_id: react_ref.into(),
        package_manager_family_token: "pnpm".into(),
        package_coordinate_ref: "pkg:npm:react".into(),
        package_name: "react".into(),
        relationship_class: DependencyRelationshipClass::Direct,
        source_class: DependencySourceClass::LockfileNode,
        freshness_class: DependencyFreshnessClass::FreshLocalAnalysis,
        provenance_class: DependencyProvenanceClass::LockfileResolved,
        resolution_class: DependencyResolutionClass::ResolvedExactLockfile,
        requested_requirement: Some("18.3.1".into()),
        resolved_version: Some("18.3.1".into()),
        manifest_ref: Some("manifest:apps_web_package_json".into()),
        manifest_path: Some("apps/web/package.json".into()),
        lockfile_ref: Some("lockfile:pnpm_lock_yaml".into()),
        lockfile_path: Some("pnpm-lock.yaml".into()),
        workspace_member_ref: Some("workspace-member:apps-web".into()),
        registry_source_ref: Some("registry-source:enterprise-npm-mirror".into()),
        advisory_refs: Vec::new(),
        license_decision_class: LicenseDecisionClass::AllowedWithNotice,
        notice_implication_refs: vec!["notice:third_party:react".into()],
        evidence_refs: vec![
            "manifest:apps_web_package_json".into(),
            "lockfile:pnpm_lock_yaml".into(),
        ],
    });
    let graph = DependencyGraphRecord::new(
        "dependency-graph:workspace-scope:apps-web:pnpm-lock",
        "workspace-scope:apps-web",
        "2026-05-18T09:00:00Z",
        DependencyFreshnessClass::MirroredData,
        MirrorOrOfflineStateClass::OnlineMirrorPinnedNoDirectOrigin,
        vec![vite, react],
        vec![DependencyEdgeRecord::new(
            "dependency-edge:apps-web:vite",
            "manifest:apps_web_package_json",
            vite_ref,
            DependencyRelationshipClass::Direct,
            vec!["lockfile:pnpm_lock_yaml".into()],
        )],
        vec![
            "manifest:apps_web_package_json".into(),
            "lockfile:pnpm_lock_yaml".into(),
            "mirror-snapshot:npm:2026-05-18T08:30:00Z".into(),
        ],
    );

    let suppression = SuppressionRef::new(
        suppression_ref,
        SuppressionStateClass::ActiveTimeBound,
        "suppression-reason:false-positive-on-test-fixture",
        "actor:security-reviewer",
        "workspace-scope:apps-web",
        Some("2026-06-30T00:00:00Z".into()),
        vec!["review-note:dependency.vite.suppression".into()],
    );
    let advisory_record = DependencyAdvisoryRecord::new(DependencyAdvisoryRecordSeed {
        advisory_id: advisory_ref.into(),
        advisory_source_ref: "advisory-source:enterprise-npm-mirror".into(),
        source_class: AdvisorySourceClass::EnterpriseMirror,
        truth_class: AdvisoryTruthClass::MirroredData,
        severity_class: AdvisorySeverityClass::High,
        lifecycle_class: AdvisoryLifecycleClass::SuppressedUntilExpiry,
        affected_dependency_refs: vec![vite_ref.into()],
        affected_ranges: vec![AdvisoryAffectedRange::new(
            "pkg:npm:vite",
            ">=5.0.0 <5.4.3",
            Some("5.4.3".into()),
        )],
        feed_epoch_ref: Some("advisory-feed-epoch:npm-mirror:2026-05-18T08:30:00Z".into()),
        mirror_snapshot_ref: Some("mirror-snapshot:npm:2026-05-18T08:30:00Z".into()),
        offline_bundle_ref: None,
        imported_report_ref: None,
        stale_reason: None,
        suppression_refs: vec![suppression],
        evidence_refs: vec![
            "dependency:pnpm:apps-web:vite".into(),
            "mirror-snapshot:npm:2026-05-18T08:30:00Z".into(),
        ],
        export_object_refs: vec![
            "schemas/dependencies/advisory_record.schema.json".into(),
            "schemas/dependencies/dependency_debt_packet.schema.json".into(),
        ],
        matched_at: "2026-05-18T09:00:00Z".into(),
    });

    let package_packet = reviewer().review_workspace(
        &workspace_fixture_root(),
        update_vite_request(),
        "2026-05-18T09:00:30Z",
    );
    let lockfile_preview = LockfileMutationPreview::from_package_operation(
        "lockfile-preview:dependency.vite.mirror-advisory-fix",
        LockfilePreviewActionClass::AdvisoryRemediation,
        &package_packet,
        "2026-05-18T09:01:00Z",
    );

    let mut advisory_debt = DependencyDebtRow::new(
        "debt:dependency.vite.active-advisory",
        DependencyDebtKindClass::ActiveAdvisory,
        vite_ref,
        "mirrored_data",
        DebtReleaseVisibilityClass::BetaReleaseVisible,
        "owner:security",
    );
    advisory_debt.advisory_ref = Some(advisory_ref.into());
    advisory_debt.severity_token = Some("high".into());
    advisory_debt.evidence_refs = vec!["mirror-snapshot:npm:2026-05-18T08:30:00Z".into()];

    let mut suppression_debt = DependencyDebtRow::new(
        "debt:dependency.vite.active-suppression",
        DependencyDebtKindClass::ActiveSuppression,
        vite_ref,
        "mirrored_data",
        DebtReleaseVisibilityClass::SupportPacketVisible,
        "owner:security",
    );
    suppression_debt.advisory_ref = Some(advisory_ref.into());
    suppression_debt.suppression_ref = Some(suppression_ref.into());
    suppression_debt.due_at = Some("2026-06-30T00:00:00Z".into());

    let mut notice_debt = DependencyDebtRow::new(
        "debt:dependency.vite.notice-implication",
        DependencyDebtKindClass::UnresolvedLicenseNotice,
        vite_ref,
        "mirrored_data",
        DebtReleaseVisibilityClass::BetaReleaseVisible,
        "owner:oss-compliance",
    );
    notice_debt.license_notice_ref = Some("notice:third_party:vite".into());
    notice_debt.evidence_refs = vec!["notice:third_party:vite".into()];

    let debt_packet = DependencyDebtPacket::new(DependencyDebtPacketSeed {
        debt_packet_id: "dependency-debt:beta:apps-web".into(),
        generated_at: "2026-05-18T09:02:00Z".into(),
        release_candidate_ref: "release-candidate:beta.apps-web".into(),
        artifact_family_refs: vec![
            "artifact-family:ide-binary".into(),
            "artifact-family:sbom-document".into(),
        ],
        dependency_graph_ref: "dependency-graph:workspace-scope:apps-web:pnpm-lock".into(),
        advisory_refs: vec![advisory_ref.into()],
        suppression_refs: vec![suppression_ref.into()],
        lockfile_preview_refs: vec!["lockfile-preview:dependency.vite.mirror-advisory-fix".into()],
        notice_license_implication_refs: vec!["notice:third_party:vite".into()],
        rows: vec![advisory_debt, suppression_debt, notice_debt],
        mirror_or_offline_state_class: MirrorOrOfflineStateClass::OnlineMirrorPinnedNoDirectOrigin,
    });

    DependencyIntelligenceFixture {
        record_kind: "dependency_intelligence_fixture".into(),
        schema_version: 1,
        dependency_graph: graph,
        advisory_record,
        lockfile_mutation_preview: lockfile_preview,
        debt_packet,
        expect: FixtureExpect {
            advisory_truth_token: "mirrored_data".into(),
            lockfile_mutation_mode_token: "regenerate_and_review".into(),
            debt_release_visible_count: 3,
        },
    }
}

#[test]
fn dependency_intelligence_fixture_matches_runtime_records() {
    let payload = std::fs::read_to_string(dependency_fixture_path()).expect("fixture reads");
    let fixture: DependencyIntelligenceFixture =
        serde_json::from_str(&payload).expect("fixture parses");
    let generated = generated_fixture();

    assert_eq!(generated, fixture);
    assert!(generated.dependency_graph.validation_issues().is_empty());
    assert!(generated.advisory_record.validation_issues().is_empty());
    assert!(generated
        .lockfile_mutation_preview
        .validation_issues()
        .is_empty());
    assert!(generated.debt_packet.validation_issues().is_empty());
    assert_eq!(generated.expect.advisory_truth_token, "mirrored_data");
    assert_eq!(
        generated.expect.lockfile_mutation_mode_token,
        "regenerate_and_review"
    );
    assert_eq!(generated.expect.debt_release_visible_count, 3);
}

#[test]
fn stale_offline_advisory_keeps_explicit_truth_class() {
    let advisory = DependencyAdvisoryRecord::new(DependencyAdvisoryRecordSeed {
        advisory_id: "advisory:dependency.cache.only".into(),
        advisory_source_ref: "advisory-source:stale-cache".into(),
        source_class: AdvisorySourceClass::StaleCache,
        truth_class: AdvisoryTruthClass::StaleOfflineState,
        severity_class: AdvisorySeverityClass::Unknown,
        lifecycle_class: AdvisoryLifecycleClass::StaleFeedNeedsReview,
        affected_dependency_refs: vec!["dependency:pnpm:apps-web:vite".into()],
        affected_ranges: vec![AdvisoryAffectedRange::new(
            "pkg:npm:vite",
            "unknown_from_stale_cache",
            None,
        )],
        feed_epoch_ref: None,
        mirror_snapshot_ref: None,
        offline_bundle_ref: Some("offline-bundle:advisories:last-known-good".into()),
        imported_report_ref: None,
        stale_reason: Some(
            "Public and mirror feeds are unavailable; last-known offline bundle is being shown."
                .into(),
        ),
        suppression_refs: Vec::new(),
        evidence_refs: vec!["offline-bundle:advisories:last-known-good".into()],
        export_object_refs: vec!["schemas/dependencies/advisory_record.schema.json".into()],
        matched_at: "2026-05-18T09:10:00Z".into(),
    });

    assert!(advisory.validation_issues().is_empty());
    assert_eq!(advisory.truth_token, "stale_offline_state");
    assert!(advisory.is_release_visible_debt());
}

#[test]
fn mutating_lockfile_preview_requires_checkpoint_and_attribution() {
    let generated = generated_fixture();
    let mut preview = generated.lockfile_mutation_preview;
    preview.rollback_checkpoint_refs.clear();
    preview.attributable_preview_object = false;

    let issues = preview.validation_issues();
    assert!(issues.contains(&DependencyIntelligenceViolation::LockfilePreviewMissingCheckpoint));
    assert!(issues.contains(&DependencyIntelligenceViolation::LockfilePreviewNotAttributable));
    assert!(preview.blocks_write());
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct DependencyIntelligenceFixture {
    record_kind: String,
    schema_version: u32,
    dependency_graph: DependencyGraphRecord,
    advisory_record: DependencyAdvisoryRecord,
    lockfile_mutation_preview: LockfileMutationPreview,
    debt_packet: DependencyDebtPacket,
    expect: FixtureExpect,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct FixtureExpect {
    advisory_truth_token: String,
    lockfile_mutation_mode_token: String,
    debt_release_visible_count: u32,
}
