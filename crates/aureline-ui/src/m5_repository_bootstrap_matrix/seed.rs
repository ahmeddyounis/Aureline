//! Canonical seed builders for the frozen M5 repository-bootstrap matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical repository-bootstrap matrix.
pub const M5_REPOSITORY_BOOTSTRAP_MATRIX_PACKET_ID: &str = "m5-repository-bootstrap:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every family must be able to show.
fn mandatory_labels() -> Vec<M5RepositoryBootstrapRequiredLabel> {
    M5RepositoryBootstrapRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a family carries.
fn labels_with(
    extra: &[M5RepositoryBootstrapRequiredLabel],
) -> Vec<M5RepositoryBootstrapRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every family filled in and every family-specific vocabulary left
/// empty for the caller to populate.
fn base_row(
    repository_bootstrap_family: M5RepositoryBootstrapFamily,
    qualification: M5RepositoryBootstrapQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5RepositoryBootstrapRow {
    M5RepositoryBootstrapRow {
        repository_bootstrap_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5RepositoryBootstrapSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5RepositoryBootstrapDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        open_local_roles: vec![],
        clone_remote_roles: vec![],
        open_archive_roles: vec![],
        import_bundle_roles: vec![],
        resume_snapshot_roles: vec![],
        degraded_reasons: M5RepositoryBootstrapDegradedReason::ALL.to_vec(),
        accessibility_routes: M5RepositoryBootstrapAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5RepositoryBootstrapConsumerSurface::SupportExport,
            M5RepositoryBootstrapConsumerSurface::DocsHelp,
        ],
        downgrade_triggers: vec![M5RepositoryBootstrapDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        rewrites_clone_into_open_when_local_checkout_already_exists: false,
        runs_repo_owned_actions_implicitly_during_acquisition: false,
        loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches: false,
        strands_partial_acquisition_without_resume_discard_or_readonly_choices: false,
        hides_bootstrap_credential_posture_behind_generic_connected_state_copy: false,
    }
}

fn repository_bootstrap_rows() -> Vec<M5RepositoryBootstrapRow> {
    use M5RepositoryBootstrapConsumerSurface as C;
    use M5RepositoryBootstrapDowngradeTrigger as D;
    use M5RepositoryBootstrapFamily as F;
    use M5RepositoryBootstrapQualificationClass as Q;
    use M5RepositoryBootstrapRequiredLabel as L;
    use M5RepositoryBootstrapRole as R;

    let mut rows = Vec::new();

    // 1. Open local.
    let mut row = base_row(
        F::OpenLocal,
        Q::Stable,
        "Repository-acquisition owner",
        "One open-local profile naming the located local checkout root, the existing checkout detected rather than recloned, the working-tree-versus-git-dir distinction, and the read-only partial root offered when incomplete so opening a local checkout stays a distinct verb and never rewrites clone into open because a local checkout already exists",
        "evidence:m5-open-local-acquisition-parity:001",
        &[
            M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
            M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
            M5_REPOSITORY_ACQUISITION_SCHEMA_REF,
        ],
    );
    row.open_local_roles = M5OpenLocalRole::ALL.to_vec();
    row.semantic_roles = vec![R::SourceLocator, R::CheckoutPlan];
    row.required_labels = labels_with(&[L::SourceLocator]);
    row.consumer_surfaces = vec![
        C::AcquisitionEngine,
        C::ShellUi,
        C::WorkspaceService,
        C::Diagnostics,
        C::SupportExport,
        C::DocsHelp,
    ];
    row.downgrade_triggers = vec![
        D::RewroteCloneIntoOpenWhenLocalCheckoutAlreadyExists,
        D::SourceLocatorUnstated,
        D::CheckoutPlanBoundaryDriftedBySurface,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Clone remote.
    let mut row = base_row(
        F::CloneRemote,
        Q::Stable,
        "Git-service owner",
        "One clone-remote profile naming the resolved remote source locator, the checkout cost and topology shown before the fetch, the credential posture disclosed before network access, and the declared sparse or partial checkout plan so a remote clone shows checkout cost, topology, and credential posture before any network or disk mutation and never runs a repo-owned action implicitly during the clone",
        "evidence:m5-clone-remote-acquisition-parity:001",
        &[
            M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
            M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
            M5_REPOSITORY_ACQUISITION_SCHEMA_REF,
        ],
    );
    row.clone_remote_roles = M5CloneRemoteRole::ALL.to_vec();
    row.semantic_roles = vec![R::SourceLocator, R::CheckoutPlan, R::CredentialPosture];
    row.required_labels = labels_with(&[L::SourceLocator, L::CheckoutPlan, L::CredentialPosture]);
    row.consumer_surfaces = vec![
        C::AcquisitionEngine,
        C::ShellUi,
        C::GitService,
        C::TrustService,
        C::Diagnostics,
        C::SupportExport,
    ];
    row.downgrade_triggers = vec![
        D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
        D::HidBootstrapCredentialPostureBehindGenericConnectedStateCopy,
        D::CredentialPostureUnstated,
        D::CheckoutPlanUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Open archive.
    let mut row = base_row(
        F::OpenArchive,
        Q::Stable,
        "Repository-acquisition owner",
        "One open-archive profile naming the located archive container, the archive digest verified before extract, the extraction plan shown before disk mutation, and the disclosed nested-archive topology so opening an archive stays a distinct verb, shows its extraction plan before disk mutation, and never silently overwrites a working tree",
        "evidence:m5-open-archive-acquisition-parity:001",
        &[
            M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
            M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
            M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_REF,
        ],
    );
    row.open_archive_roles = M5OpenArchiveRole::ALL.to_vec();
    row.semantic_roles = vec![R::SourceLocator, R::EvidencePacket];
    row.required_labels = labels_with(&[L::SourceLocator]);
    row.consumer_surfaces = vec![
        C::AcquisitionEngine,
        C::ShellUi,
        C::GitService,
        C::Diagnostics,
        C::SupportExport,
        C::DocsHelp,
    ];
    row.downgrade_triggers = vec![
        D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
        D::SourceLocatorUnstated,
        D::CheckoutPlanBoundaryDriftedBySurface,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Import bundle.
    let mut row = base_row(
        F::ImportBundle,
        Q::Stable,
        "Trust-service owner",
        "One import-bundle profile naming the verified bundle signer continuity, the preserved mirror and air-gap provenance, the bundle digest verified before import, and the recorded offline-import evidence so importing a bundle preserves signer and mirror provenance across offline or mirrored fetches and stages trust rather than running repo-owned actions implicitly",
        "evidence:m5-import-bundle-acquisition-parity:001",
        &[
            M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
            M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
            M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_REF,
        ],
    );
    row.import_bundle_roles = M5ImportBundleRole::ALL.to_vec();
    row.semantic_roles = vec![R::EvidencePacket, R::StagedTrust];
    row.required_labels = labels_with(&[L::CredentialPosture]);
    row.consumer_surfaces = vec![
        C::AcquisitionEngine,
        C::GitService,
        C::TrustService,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
        D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
        D::StagedTrustRuleUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Resume snapshot.
    let mut row = base_row(
        F::ResumeSnapshot,
        Q::Stable,
        "Workspace-service owner",
        "One resume-snapshot profile naming the resumable partial-acquisition state, the offered Resume / Discard / Open-read-only-partial-root choice, the typed post-open bootstrap queue, and the preserved resume evidence so an interrupted or partial acquisition stays resumable or discardable with evidence and never strands partial state without a choice or auto-executes a post-open queue",
        "evidence:m5-resume-snapshot-acquisition-parity:001",
        &[
            M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
            M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
            M5_REPOSITORY_ACQUISITION_SCHEMA_REF,
        ],
    );
    row.resume_snapshot_roles = M5ResumeSnapshotRole::ALL.to_vec();
    row.semantic_roles = vec![R::ResumableAcquisition, R::PostOpenQueue];
    row.required_labels = labels_with(&[L::CheckoutPlan]);
    row.consumer_surfaces = vec![
        C::AcquisitionEngine,
        C::WorkspaceService,
        C::TrustService,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::StrandedPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices,
        D::StagedTrustRuleUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5RepositoryBootstrapGovernanceReview {
    M5RepositoryBootstrapGovernanceReview {
        source_locator_and_checkout_plan_stay_separately_inspectable: true,
        repo_owned_actions_never_run_implicitly_during_acquisition: true,
        clone_is_never_rewritten_into_open_when_local_checkout_exists: true,
        checkout_cost_topology_and_credential_posture_shown_before_mutation: true,
        signer_and_mirror_provenance_preserved_across_offline_or_mirrored_fetches: true,
        interrupted_acquisition_stays_resumable_or_discardable_with_evidence: true,
        trust_is_staged_so_repo_owned_actions_never_run_implicitly: true,
        bootstrap_credential_posture_never_hidden_behind_generic_connected_state_copy: true,
        post_open_bootstrap_queue_is_typed_and_never_auto_executed: true,
        every_family_declares_acquisition_contexts: true,
        every_family_declares_accessibility_route: true,
        support_export_reads_single_repository_bootstrap_source: true,
        shell_entry_diagnostics_admin_bind_to_single_repository_bootstrap_source: true,
        later_rows_cannot_invent_parallel_repository_bootstrap_vocabulary: true,
        bootstrap_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_registry_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5RepositoryBootstrapConsumerProjection {
    M5RepositoryBootstrapConsumerProjection {
        shell_and_entry_consume_shared_repository_bootstrap_truth: true,
        diagnostics_and_admin_consume_shared_trust_stage_boundaries: true,
        git_and_workspace_services_consume_shared_source_locator_and_checkout_plan: true,
        docs_help_and_screenshots_read_single_repository_bootstrap_source: true,
        hooks_tasks_extensions_and_generators_bind_to_shared_staged_trust_rule: true,
        support_export_reads_single_repository_bootstrap_source: true,
    }
}

fn proof_freshness() -> M5RepositoryBootstrapProofFreshness {
    M5RepositoryBootstrapProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RepositoryBootstrapReleasePosture {
    M5RepositoryBootstrapReleasePosture {
        proof_packet_ref: M5_REPOSITORY_BOOTSTRAP_ARTIFACT_REF.to_owned(),
        repository_bootstrap_audit_ref: M5_REPOSITORY_BOOTSTRAP_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
        M5_REPOSITORY_ACQUISITION_SCHEMA_REF,
        M5_SOURCE_ACQUISITION_REVIEW_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 repository-bootstrap matrix packet.
pub fn seeded_m5_repository_bootstrap_matrix() -> M5RepositoryBootstrapMatrixPacket {
    M5RepositoryBootstrapMatrixPacket::new(M5RepositoryBootstrapMatrixPacketInput {
        packet_id: M5_REPOSITORY_BOOTSTRAP_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 repository-bootstrap, checkout-plan, trust-stage, and post-open-queue matrix"
                .to_owned(),
        repository_bootstrap_rows: repository_bootstrap_rows(),
        vocabulary_set: M5RepositoryBootstrapVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: import bundle is held at Beta because mirror / air-gap signer continuity is not yet
/// proven across every acquisition context; every family stays visible.
pub fn seeded_m5_repository_bootstrap_matrix_import_bundle_beta_narrowed(
) -> M5RepositoryBootstrapMatrixPacket {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.packet_id = "m5-repository-bootstrap:import-bundle-beta:0001".to_owned();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::ImportBundle)
        .expect("import-bundle row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Beta;
    packet
}

/// Narrowed variant: resume snapshot is narrowed to Preview pending complete resumable-partial-acquisition
/// evidence across every acquisition context; every family stays visible.
pub fn seeded_m5_repository_bootstrap_matrix_resume_snapshot_preview_narrowed(
) -> M5RepositoryBootstrapMatrixPacket {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.packet_id = "m5-repository-bootstrap:resume-snapshot-preview:0001".to_owned();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::ResumeSnapshot)
        .expect("resume-snapshot row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Preview;
    packet
}
