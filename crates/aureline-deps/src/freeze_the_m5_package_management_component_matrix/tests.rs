use super::*;

const PACKET_ID: &str = "m5-package-component-matrix:stable:0001";

fn component_rows() -> Vec<M5PackageComponentMatrixRow> {
    vec![
        M5PackageComponentMatrixRow {
            component: M5PackageComponent::PackageExplorerRow,
            maturity: M5PackageComponentMaturityClass::Stable,
            scope_summary: "Package explorer row naming one package, its owning manifest, and its direct/transitive/workspace-local relation while browsing a package set".to_owned(),
            manifest_scope_disclosure: "Names the owning manifest and whether the package is a direct, transitive, or workspace-local dependency rather than presenting a flat unscoped list".to_owned(),
            registry_source_disclosure: "Names the registry, mirror, or offline snapshot the resolved identity came from so a mirror answer is never shown as an upstream fact".to_owned(),
            auth_posture_disclosure: "Marks whether the row's resolution needed authentication and whether it was satisfied, never hiding an auth-required state behind a blank version".to_owned(),
            script_native_build_disclosure: "Carries no mutation itself but surfaces whether the package is known to run install scripts or a native build when a mutation is proposed from the row".to_owned(),
            lockfile_churn_disclosure: "Reflects whether the package is exactly pinned by the lockfile or only range-governed so an unpinned dependency is never read as fixed".to_owned(),
            rollback_checkpoint_disclosure: "Read-only browse row; it references the durable operation history entry that would carry any rollback rather than implying an inline revert".to_owned(),
            degradation_narrowing_vocab: vec![
                M5PackageComponentDegradationState::ResolvedExact,
                M5PackageComponentDegradationState::ManifestRangeOnly,
                M5PackageComponentDegradationState::MirrorBacked,
                M5PackageComponentDegradationState::UnknownOrStale,
            ],
            evidence_requirement: M5PackageComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:package-explorer-row-manifest-and-relation:m5".to_owned(),
                "evidence:package-explorer-row-source-and-pin-state:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5PackageComponentDowngradeTrigger::ProofStale,
                M5PackageComponentDowngradeTrigger::MirrorBackedOnly,
                M5PackageComponentDowngradeTrigger::RegistryUnreachable,
                M5PackageComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5PackageComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5PackageComponentConsumerSurface::PackageWorkspace,
                M5PackageComponentConsumerSurface::DependencyExplorer,
                M5PackageComponentConsumerSurface::CliHeadless,
                M5PackageComponentConsumerSurface::SupportExport,
            ],
        },
        M5PackageComponentMatrixRow {
            component: M5PackageComponent::ManifestScopeSwitcher,
            maturity: M5PackageComponentMaturityClass::Stable,
            scope_summary: "Manifest-scope switcher selecting the target manifest (whole workspace, selected manifest, workset slice, or workspace member) before a mutation leaves review".to_owned(),
            manifest_scope_disclosure: "This is the primary carrier of manifest-scope truth; it names the exact target manifest and requires explicit confirmation before a whole-workspace mutation".to_owned(),
            registry_source_disclosure: "Names the registry/mirror the selected scope resolves against so switching scope never silently changes the source of truth".to_owned(),
            auth_posture_disclosure: "Surfaces whether the selected scope requires auth that is not yet satisfied so a member switch never masks an auth-required target".to_owned(),
            script_native_build_disclosure: "Marks whether the selected scope contains packages known to run scripts or native builds so scope selection carries the risk forward".to_owned(),
            lockfile_churn_disclosure: "Names whether the scope shares a lockfile with siblings so a member operation can never silently widen lockfile churn to the wrong manifest".to_owned(),
            rollback_checkpoint_disclosure: "Selecting scope is not a mutation; it references the checkpoint the eventual mutation on this scope would create rather than reverting anything".to_owned(),
            degradation_narrowing_vocab: vec![
                M5PackageComponentDegradationState::ResolvedExact,
                M5PackageComponentDegradationState::ManifestRangeOnly,
                M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
                M5PackageComponentDegradationState::UnknownOrStale,
            ],
            evidence_requirement: M5PackageComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:manifest-scope-switcher-target-and-confirmation:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5PackageComponentDowngradeTrigger::ProofStale,
                M5PackageComponentDowngradeTrigger::ScopeExpansionUnqualified,
                M5PackageComponentDowngradeTrigger::AuthRequired,
                M5PackageComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5PackageComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5PackageComponentConsumerSurface::PackageWorkspace,
                M5PackageComponentConsumerSurface::InstallUpdateReview,
                M5PackageComponentConsumerSurface::CliHeadless,
                M5PackageComponentConsumerSurface::SupportExport,
            ],
        },
        M5PackageComponentMatrixRow {
            component: M5PackageComponent::InstallReviewSheet,
            maturity: M5PackageComponentMaturityClass::Stable,
            scope_summary: "Install-review sheet previewing one install, update, remove, or regenerate with manifest scope, script risk, resolver identity, and lockfile churn explicit before commit".to_owned(),
            manifest_scope_disclosure: "Names the exact target manifest and whether the change adds a direct, transitive, or workspace-local dependency so an install is never applied to an ambiguous target".to_owned(),
            registry_source_disclosure: "Names the registry or mirror the packages resolve from and marks an offline-snapshot resolution so review never overstates upstream freshness".to_owned(),
            auth_posture_disclosure: "States whether the resolve/publish path needs auth and whether it is satisfied so the commit gate blocks while an auth-required state holds".to_owned(),
            script_native_build_disclosure: "Surfaces the script/native-build label (no scripts, known install scripts, native build required, unknown hook risk, or policy blocked) so a one-click install never hides code execution".to_owned(),
            lockfile_churn_disclosure: "Shows the lockfile diff class and quantified blast radius so a broad regeneration is never presented as a small pin change".to_owned(),
            rollback_checkpoint_disclosure: "Pins the durable rollback checkpoint the commit will create so a failed or partial mutation leaves a recorded receipt rather than a transient toast".to_owned(),
            degradation_narrowing_vocab: vec![
                M5PackageComponentDegradationState::ResolvedExact,
                M5PackageComponentDegradationState::ManifestRangeOnly,
                M5PackageComponentDegradationState::OfflineSnapshotOnly,
                M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
            ],
            evidence_requirement: M5PackageComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:install-review-sheet-scope-and-script-risk:m5".to_owned(),
                "evidence:install-review-sheet-lockfile-churn-and-rollback:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5PackageComponentDowngradeTrigger::ProofStale,
                M5PackageComponentDowngradeTrigger::ScriptOrNativeBuildRisk,
                M5PackageComponentDowngradeTrigger::LockfileDivergent,
                M5PackageComponentDowngradeTrigger::AuthRequired,
            ],
            rollback_posture: M5PackageComponentRollbackPosture::WriteBackCheckpointed,
            source_contract_refs: vec![
                M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5PackageComponentConsumerSurface::InstallUpdateReview,
                M5PackageComponentConsumerSurface::PackageWorkspace,
                M5PackageComponentConsumerSurface::CliHeadless,
                M5PackageComponentConsumerSurface::SupportExport,
            ],
        },
        M5PackageComponentMatrixRow {
            component: M5PackageComponent::RegistryOrMirrorRow,
            maturity: M5PackageComponentMaturityClass::Stable,
            scope_summary: "Registry or mirror row naming the source a package resolves from (public registry, private registry, enterprise mirror, cache, or offline snapshot) and the auth posture used to reach it".to_owned(),
            manifest_scope_disclosure: "Names which manifests and scopes route through this source so a mirror is never applied to a manifest the user did not intend".to_owned(),
            registry_source_disclosure: "This is the primary carrier of registry-source truth; it distinguishes public, private, mirror, cache-only, and offline-snapshot sources and never collapses them into a generic connected state".to_owned(),
            auth_posture_disclosure: "This is the primary carrier of auth posture; it names the credential mode (anonymous, OS-store, token, browser/device sign-in, or policy) and marks an auth-required-unsatisfied state without ever showing a token body or private URL".to_owned(),
            script_native_build_disclosure: "Source selection carries no script execution itself, and the row states that install-script risk is disclosed on the install-review sheet, not masked here".to_owned(),
            lockfile_churn_disclosure: "Notes whether switching source can change resolved identities and therefore lockfile churn so a mirror swap never silently rewrites the lockfile".to_owned(),
            rollback_checkpoint_disclosure: "Read-only source posture row; it references the auth-flow revoke/switch-account recovery rather than a lockfile rollback".to_owned(),
            degradation_narrowing_vocab: vec![
                M5PackageComponentDegradationState::ResolvedExact,
                M5PackageComponentDegradationState::MirrorBacked,
                M5PackageComponentDegradationState::OfflineSnapshotOnly,
                M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
            ],
            evidence_requirement: M5PackageComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:registry-or-mirror-row-source-and-auth:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5PackageComponentDowngradeTrigger::ProofStale,
                M5PackageComponentDowngradeTrigger::MirrorBackedOnly,
                M5PackageComponentDowngradeTrigger::OfflineSnapshotOnly,
                M5PackageComponentDowngradeTrigger::AuthRequired,
            ],
            rollback_posture: M5PackageComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5PackageComponentConsumerSurface::RegistryAuthWorkspace,
                M5PackageComponentConsumerSurface::InstallUpdateReview,
                M5PackageComponentConsumerSurface::SupportExport,
                M5PackageComponentConsumerSurface::HelpAbout,
            ],
        },
        M5PackageComponentMatrixRow {
            component: M5PackageComponent::ScriptRiskNotice,
            maturity: M5PackageComponentMaturityClass::Beta,
            scope_summary: "Script-risk notice naming whether a proposed mutation may run install/post-install scripts or a native build so code execution is disclosed before, not after, install".to_owned(),
            manifest_scope_disclosure: "Names which manifest and packages carry the script or native-build risk so the notice maps to a real target rather than a blanket warning".to_owned(),
            registry_source_disclosure: "Notes the source of the risky package so a mirror-provided package's post-install behavior is attributed to its real origin".to_owned(),
            auth_posture_disclosure: "States that disclosing script risk never requires or reveals credentials; auth posture is inherited from the registry row rather than duplicated here".to_owned(),
            script_native_build_disclosure: "This is the primary carrier of script/native-build risk; it keeps no-scripts, known install scripts, native build required, unknown hook risk, and policy-blocked distinct and never downgrades unknown risk to none".to_owned(),
            lockfile_churn_disclosure: "Notes that running a build or script does not itself change the lockfile, keeping execution risk and dependency churn as separate facts".to_owned(),
            rollback_checkpoint_disclosure: "States that side effects of scripts or native builds may be only compensating-reversible, so the notice names when a checkpoint cannot fully undo execution".to_owned(),
            degradation_narrowing_vocab: vec![
                M5PackageComponentDegradationState::ResolvedExact,
                M5PackageComponentDegradationState::MirrorBacked,
                M5PackageComponentDegradationState::UnknownOrStale,
                M5PackageComponentDegradationState::OfflineSnapshotOnly,
            ],
            evidence_requirement: M5PackageComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:script-risk-notice-execution-disclosure:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5PackageComponentDowngradeTrigger::ProofStale,
                M5PackageComponentDowngradeTrigger::ScriptOrNativeBuildRisk,
                M5PackageComponentDowngradeTrigger::PolicyBlocked,
                M5PackageComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5PackageComponentRollbackPosture::CompensatingOnlyNoCleanRevert,
            source_contract_refs: vec![
                M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5PackageComponentConsumerSurface::InstallUpdateReview,
                M5PackageComponentConsumerSurface::PackageWorkspace,
                M5PackageComponentConsumerSurface::SupportExport,
                M5PackageComponentConsumerSurface::HelpAbout,
            ],
        },
        M5PackageComponentMatrixRow {
            component: M5PackageComponent::LockfileImpactCard,
            maturity: M5PackageComponentMaturityClass::Stable,
            scope_summary: "Lockfile-impact card quantifying the lockfile diff (added/removed/changed resolutions and blast radius) for a proposed mutation without understating churn".to_owned(),
            manifest_scope_disclosure: "Names which manifest's lockfile is affected and whether siblings share it so a member change never hides workspace-wide lockfile impact".to_owned(),
            registry_source_disclosure: "Marks whether the new resolutions came from upstream, a mirror, or an offline snapshot so lockfile churn is attributed to a real source".to_owned(),
            auth_posture_disclosure: "Notes when a divergent or blocked auth state prevented a full resolution so an incomplete lockfile diff is never shown as complete".to_owned(),
            script_native_build_disclosure: "Cross-references the script-risk notice for any newly added package rather than presenting lockfile churn as risk-free".to_owned(),
            lockfile_churn_disclosure: "This is the primary carrier of lockfile-churn truth; it quantifies added/removed/changed resolutions and marks a broad regeneration explicitly rather than as a single-line change".to_owned(),
            rollback_checkpoint_disclosure: "Names the manifest/lockfile checkpoint that a revert would restore so lockfile impact and its rollback stay bound together".to_owned(),
            degradation_narrowing_vocab: vec![
                M5PackageComponentDegradationState::ResolvedExact,
                M5PackageComponentDegradationState::ManifestRangeOnly,
                M5PackageComponentDegradationState::OfflineSnapshotOnly,
                M5PackageComponentDegradationState::UnknownOrStale,
            ],
            evidence_requirement: M5PackageComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:lockfile-impact-card-churn-quantified:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5PackageComponentDowngradeTrigger::ProofStale,
                M5PackageComponentDowngradeTrigger::BroadLockfileRegeneration,
                M5PackageComponentDowngradeTrigger::LockfileDivergent,
                M5PackageComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5PackageComponentRollbackPosture::RegenerateOnlyNoManualEdit,
            source_contract_refs: vec![
                M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5PackageComponentConsumerSurface::InstallUpdateReview,
                M5PackageComponentConsumerSurface::DependencyExplorer,
                M5PackageComponentConsumerSurface::CliHeadless,
                M5PackageComponentConsumerSurface::SupportExport,
            ],
        },
        M5PackageComponentMatrixRow {
            component: M5PackageComponent::GroupedUpdatePlanner,
            maturity: M5PackageComponentMaturityClass::Preview,
            scope_summary: "Grouped-update planner naming the grouped-update reason (security, major, minor/patch, pinned, deduped, or workspace-aligned) with constraint and conflict cards before any batch update applies".to_owned(),
            manifest_scope_disclosure: "Names every manifest the grouped plan touches so a batch update never widens beyond the manifests the user reviewed".to_owned(),
            registry_source_disclosure: "Names the registry/mirror each grouped candidate resolves from so a grouped plan never mixes upstream and mirror answers silently".to_owned(),
            auth_posture_disclosure: "Marks any grouped candidate whose source needs unsatisfied auth so the plan cannot claim readiness it cannot reach".to_owned(),
            script_native_build_disclosure: "Aggregates the script/native-build risk across grouped candidates so a batch update never hides that some members run code on install".to_owned(),
            lockfile_churn_disclosure: "Rolls up per-candidate lockfile churn into the plan's total blast radius without averaging away a single large regeneration".to_owned(),
            rollback_checkpoint_disclosure: "Binds the grouped plan to a single durable rollback checkpoint with revert, open-diff, and export-patch recovery so a partial batch failure stays recoverable".to_owned(),
            degradation_narrowing_vocab: vec![
                M5PackageComponentDegradationState::ResolvedExact,
                M5PackageComponentDegradationState::ManifestRangeOnly,
                M5PackageComponentDegradationState::MirrorBacked,
                M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
            ],
            evidence_requirement: M5PackageComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:grouped-update-planner-reason-and-constraints:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5PackageComponentDowngradeTrigger::ProofStale,
                M5PackageComponentDowngradeTrigger::BroadLockfileRegeneration,
                M5PackageComponentDowngradeTrigger::ScopeExpansionUnqualified,
                M5PackageComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5PackageComponentRollbackPosture::WriteBackCheckpointed,
            source_contract_refs: vec![
                M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5PackageComponentConsumerSurface::InstallUpdateReview,
                M5PackageComponentConsumerSurface::PackageWorkspace,
                M5PackageComponentConsumerSurface::CliHeadless,
                M5PackageComponentConsumerSurface::SupportExport,
            ],
        },
        M5PackageComponentMatrixRow {
            component: M5PackageComponent::RollbackCheckpointStrip,
            maturity: M5PackageComponentMaturityClass::Stable,
            scope_summary: "Rollback / checkpoint strip naming the durable checkpoint identity for a completed or in-flight package mutation with revert, open-diff, and export-patch recovery actions".to_owned(),
            manifest_scope_disclosure: "Names the exact manifest and scope the checkpoint would restore so a revert never silently touches manifests outside the recorded operation".to_owned(),
            registry_source_disclosure: "Records the source the mutation resolved from so a revert against a stale mirror is disclosed rather than assumed clean".to_owned(),
            auth_posture_disclosure: "Notes when re-resolving during a revert would need auth so an unreachable rollback is disclosed instead of implied instant".to_owned(),
            script_native_build_disclosure: "Marks when the original mutation ran scripts or a native build so the strip states that reverting the manifest may not undo those side effects".to_owned(),
            lockfile_churn_disclosure: "Names the manifest/lockfile identity before and after as redacted digests so the exact lockfile the checkpoint restores is unambiguous".to_owned(),
            rollback_checkpoint_disclosure: "This is the primary carrier of rollback/checkpoint identity; it keeps the checkpoint id, reachability, and recovery actions explicit and never reduces to a generic undo".to_owned(),
            degradation_narrowing_vocab: vec![
                M5PackageComponentDegradationState::ResolvedExact,
                M5PackageComponentDegradationState::OfflineSnapshotOnly,
                M5PackageComponentDegradationState::AuthRequiredUnsatisfied,
                M5PackageComponentDegradationState::UnknownOrStale,
            ],
            evidence_requirement: M5PackageComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:rollback-checkpoint-strip-identity-and-recovery:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5PackageComponentDowngradeTrigger::ProofStale,
                M5PackageComponentDowngradeTrigger::RollbackUnavailable,
                M5PackageComponentDowngradeTrigger::PolicyBlocked,
                M5PackageComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5PackageComponentRollbackPosture::WriteBackCheckpointed,
            source_contract_refs: vec![
                M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5PackageComponentConsumerSurface::RollbackRecovery,
                M5PackageComponentConsumerSurface::InstallUpdateReview,
                M5PackageComponentConsumerSurface::CliHeadless,
                M5PackageComponentConsumerSurface::SupportExport,
            ],
        },
    ]
}

fn trust_review() -> M5PackageComponentMatrixTrustReview {
    M5PackageComponentMatrixTrustReview {
        manifest_scope_always_explicit: true,
        direct_transitive_state_explicit: true,
        registry_source_always_explicit: true,
        auth_posture_never_hidden: true,
        script_native_build_risk_explicit: true,
        lockfile_churn_never_understated: true,
        grouped_update_reason_explicit: true,
        rollback_checkpoint_identity_explicit: true,
        mirror_offline_continuity_explicit: true,
        one_click_never_conceals_scope_or_risk: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5PackageComponentMatrixConsumerProjection {
    M5PackageComponentMatrixConsumerProjection {
        package_explorer_row_shows_manifest_and_relation: true,
        manifest_scope_switcher_shows_target_and_confirmation: true,
        install_review_sheet_shows_scope_script_and_lockfile: true,
        registry_or_mirror_row_shows_source_and_auth: true,
        script_risk_notice_shows_script_native_build_risk: true,
        lockfile_impact_card_shows_churn_without_understating: true,
        grouped_update_planner_shows_reason_and_constraints: true,
        rollback_checkpoint_strip_shows_checkpoint_identity: true,
        cli_headless_shows_component_truth: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> M5PackageComponentMatrixProofFreshness {
    M5PackageComponentMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_EXPLORER_ROW_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_MANIFEST_SCOPE_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_INSTALL_REVIEW_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_REGISTRY_MIRROR_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_LOCKFILE_IMPACT_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5PackageComponentMatrixPacket {
    M5PackageComponentMatrixPacket::new(M5PackageComponentMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Package-Management Component Matrix".to_owned(),
        component_rows: component_rows(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_package_component_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn matrix_has_eight_components() {
    assert_eq!(packet().component_rows.len(), 8);
    assert_eq!(M5PackageComponent::ALL.len(), 8);
}

#[test]
fn missing_component_fails_validation() {
    let mut packet = packet();
    packet
        .component_rows
        .retain(|row| row.component != M5PackageComponent::InstallReviewSheet);
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn component_source_contract_mismatch_fails() {
    let mut packet = packet();
    packet.component_rows[0].source_contract_refs =
        vec![M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::ComponentSourceContractMismatch));
}

#[test]
fn each_component_lists_its_canonical_contract() {
    for row in packet().component_rows {
        assert!(
            row.source_contract_refs
                .contains(&row.component.canonical_source_contract_ref().to_owned()),
            "component {} missing canonical contract",
            row.component.as_str()
        );
    }
}

#[test]
fn every_canonical_contract_is_distinct() {
    let refs: BTreeSet<&str> = M5PackageComponent::ALL
        .iter()
        .map(|component| component.canonical_source_contract_ref())
        .collect();
    assert_eq!(refs.len(), M5PackageComponent::ALL.len());
}

#[test]
fn stable_component_missing_evidence_fails() {
    let mut packet = packet();
    packet.component_rows[0]
        .required_evidence_packet_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::StableComponentMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.component_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.component_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_manifest_scope_disclosure_fails() {
    let mut packet = packet();
    packet.component_rows[0].manifest_scope_disclosure = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::ManifestScopeDisclosureMissing));
}

#[test]
fn missing_registry_source_disclosure_fails() {
    let mut packet = packet();
    packet.component_rows[3].registry_source_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::RegistrySourceDisclosureMissing));
}

#[test]
fn missing_auth_posture_disclosure_fails() {
    let mut packet = packet();
    packet.component_rows[3].auth_posture_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::AuthPostureDisclosureMissing));
}

#[test]
fn missing_script_native_build_disclosure_fails() {
    let mut packet = packet();
    packet.component_rows[4].script_native_build_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::ScriptNativeBuildDisclosureMissing));
}

#[test]
fn missing_lockfile_churn_disclosure_fails() {
    let mut packet = packet();
    packet.component_rows[5].lockfile_churn_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::LockfileChurnDisclosureMissing));
}

#[test]
fn missing_rollback_checkpoint_disclosure_fails() {
    let mut packet = packet();
    packet.component_rows[7].rollback_checkpoint_disclosure = String::new();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::RollbackCheckpointDisclosureMissing));
}

#[test]
fn missing_degradation_narrowing_vocab_fails() {
    let mut packet = packet();
    packet.component_rows[4].degradation_narrowing_vocab.clear();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::DegradationNarrowingVocabMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.one_click_never_conceals_scope_or_risk = false;
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .script_risk_notice_shows_script_native_build_risk = false;
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::WrongRecordKind));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.component_rows[0].scope_summary = "leaked password value".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PackageComponentMatrixViolation::RawBoundaryMaterialInExport));
}

#[test]
fn only_two_components_are_narrowed() {
    let narrowed = packet()
        .component_rows
        .iter()
        .filter(|row| !row.maturity.is_stable())
        .count();
    assert_eq!(narrowed, 2);
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = packet().render_markdown_summary();
    for component in M5PackageComponent::ALL {
        assert!(
            summary.contains(component.as_str()),
            "summary missing component {}",
            component.as_str()
        );
    }
}

#[test]
fn export_safe_json_roundtrips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5PackageComponentMatrixPacket =
        serde_json::from_str(&json).expect("export json roundtrips");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_package_component_matrix_export()
        .expect("checked M5 package-component matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let checked = current_stable_m5_package_component_matrix_export()
        .expect("checked M5 package-component matrix export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-management-components/script_risk_notice_unknown_hook.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-package-management-components/grouped_update_planner_offline_snapshot.json"
        )),
    ] {
        let packet: M5PackageComponentMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_PACKAGE_MANAGEMENT_COMPONENT_ARTIFACTS` so ordinary test
/// runs never touch the working tree. Run with the env var set to refresh the
/// checked-in support export, summary, and fixtures from the seed builder.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_PACKAGE_MANAGEMENT_COMPONENT_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-package-management-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-package-management-components");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let mut script = packet.clone();
    script.packet_id = "m5-package-component-matrix:fixture:script-risk-unknown-hook".to_owned();
    if let Some(row) = script
        .component_rows
        .iter_mut()
        .find(|row| row.component == M5PackageComponent::ScriptRiskNotice)
    {
        row.degradation_narrowing_vocab = vec![
            M5PackageComponentDegradationState::UnknownOrStale,
            M5PackageComponentDegradationState::MirrorBacked,
        ];
    }
    assert!(script.validate().is_empty(), "{:?}", script.validate());
    std::fs::write(
        fixture_dir.join("script_risk_notice_unknown_hook.json"),
        format!("{}\n", script.export_safe_json()),
    )
    .expect("write script-risk fixture");

    let mut grouped = packet.clone();
    grouped.packet_id =
        "m5-package-component-matrix:fixture:grouped-update-offline-snapshot".to_owned();
    if let Some(row) = grouped
        .component_rows
        .iter_mut()
        .find(|row| row.component == M5PackageComponent::GroupedUpdatePlanner)
    {
        row.degradation_narrowing_vocab = vec![
            M5PackageComponentDegradationState::OfflineSnapshotOnly,
            M5PackageComponentDegradationState::UnknownOrStale,
        ];
    }
    assert!(grouped.validate().is_empty(), "{:?}", grouped.validate());
    std::fs::write(
        fixture_dir.join("grouped_update_planner_offline_snapshot.json"),
        format!("{}\n", grouped.export_safe_json()),
    )
    .expect("write grouped-update fixture");
}
