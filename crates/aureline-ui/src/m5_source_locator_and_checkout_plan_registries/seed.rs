//! Canonical seed builders for the M5 source-locator and checkout-plan registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean source-locator and checkout-plan entries are built
//! so the one stable source-locator object resolving per entry flow, open and clone staying distinct verbs, the
//! literal target preserved verbatim, the bootstrap credential posture disclosed before any network or mirror
//! fetch, the canonical / accessible / audit resolution forms, and the complete ref-selection / depth-filter /
//! submodule-mode / LFS-posture / destination-path / cost-band checkout-plan object are proven across the
//! acquisition-engine, shell, workspace, git, diagnostics, and support surfaces without any hand-copied
//! per-entry assumption, verb rewrite, incomplete object, implicit bootstrap, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_PACKET_ID: &str =
    "m5-source-locator-and-checkout-plan-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn locator(input: M5SourceLocatorEntryResolutionInput) -> M5ResolvedSourceLocatorEntry {
    resolve_source_locator_entry(input).expect("seed source-locator entry resolves")
}

fn plan(input: M5CheckoutPlanEntryResolutionInput) -> M5ResolvedCheckoutPlanEntry {
    resolve_checkout_plan_entry(input).expect("seed checkout-plan entry resolves")
}

fn all_forms() -> Vec<M5AcquisitionResolutionForm> {
    M5AcquisitionResolutionForm::ALL.to_vec()
}

// -- Clean source-locator entries (stable object, verb-preserving, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_locator_base(
    entry_id: &str,
    entry_flow_id: &str,
    token_name: &str,
    semantic_role: M5RepositoryBootstrapRole,
    source_locator_kind: M5SourceLocatorKind,
    surface_context: M5AcquisitionSurfaceContext,
    literal_target: &str,
    resolved_root_or_container: &str,
    trust_stage_metadata: &str,
    credential_posture: &str,
    signer_or_mirror_provenance: &str,
    mirror_or_air_gap_hint: &str,
) -> M5SourceLocatorEntryResolutionInput {
    M5SourceLocatorEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        entry_flow_id: entry_flow_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        source_locator_kind,
        surface_context,
        resolution_form_coverage: all_forms(),
        literal_target: literal_target.to_owned(),
        resolved_root_or_container: resolved_root_or_container.to_owned(),
        trust_stage_metadata: trust_stage_metadata.to_owned(),
        credential_posture: credential_posture.to_owned(),
        signer_or_mirror_provenance: signer_or_mirror_provenance.to_owned(),
        mirror_or_air_gap_hint: mirror_or_air_gap_hint.to_owned(),
        bound_to_registry: true,
        literal_target_preserved: true,
        touches_network_or_mirror: false,
        credential_posture_disclosed: true,
        proof_fresh: true,
    }
}

fn locator_shell_localpath_clean() -> M5ResolvedSourceLocatorEntry {
    locator(clean_locator_base(
        "locator:shell:local-path",
        "entry.acme.open-local",
        "source.locator.local_path",
        M5RepositoryBootstrapRole::SourceLocator,
        M5SourceLocatorKind::LocalPathSource,
        M5AcquisitionSurfaceContext::ShellSurface,
        "local-path.acme/repo",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    ))
}

fn locator_entry_remote_clean() -> M5ResolvedSourceLocatorEntry {
    // A remote forge clone touches the network and discloses its credential posture before the fetch.
    let mut base = clean_locator_base(
        "locator:entry:remote",
        "entry.acme.clone-remote",
        "source.locator.remote_forge",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5SourceLocatorKind::RemoteForgeUrlSource,
        M5AcquisitionSurfaceContext::EntrySurface,
        "remote-forge.acme/org/repo",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.disclosed",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    );
    base.touches_network_or_mirror = true;
    base.credential_posture_disclosed = true;
    locator(base)
}

fn locator_diagnostics_archive_clean() -> M5ResolvedSourceLocatorEntry {
    locator(clean_locator_base(
        "locator:diagnostics:archive",
        "entry.acme.open-archive",
        "source.locator.archive_bundle",
        M5RepositoryBootstrapRole::SourceLocator,
        M5SourceLocatorKind::ArchiveImportBundleSource,
        M5AcquisitionSurfaceContext::DiagnosticsSurface,
        "archive-bundle.acme/pack.bundle",
        "archive-container.acme/pack",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "air-gap-hint.offline",
    ))
}

fn locator_admin_mirror_clean() -> M5ResolvedSourceLocatorEntry {
    // A mirrored fetch preserves signer / mirror provenance and discloses its credential posture.
    let mut base = clean_locator_base(
        "locator:admin:mirror",
        "entry.acme.mirrored",
        "source.locator.mirror",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5SourceLocatorKind::MirrorSource,
        M5AcquisitionSurfaceContext::AdminSurface,
        "mirror.acme/org/repo",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.disclosed",
        "mirror-provenance.acme.v3",
        "mirror-hint.airgap",
    );
    base.touches_network_or_mirror = true;
    base.credential_posture_disclosed = true;
    locator(base)
}

fn locator_support_snapshot_clean() -> M5ResolvedSourceLocatorEntry {
    locator(clean_locator_base(
        "locator:support:snapshot",
        "entry.acme.managed-snapshot",
        "source.locator.managed_snapshot",
        M5RepositoryBootstrapRole::SourceLocator,
        M5SourceLocatorKind::ManagedSnapshotSource,
        M5AcquisitionSurfaceContext::SupportOrExportForm,
        "managed-snapshot.acme/snap-0007",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    ))
}

// -- Degraded source-locator entries ------------------------------------------------------------

/// Degraded locator entry: the resolved locator object is incomplete — the resolved checkout root / archive
/// container is unstated.
fn locator_object_incomplete() -> M5ResolvedSourceLocatorEntry {
    let mut base = clean_locator_base(
        "locator:shell:incomplete",
        "entry.acme.open-local",
        "source.locator.local_path",
        M5RepositoryBootstrapRole::SourceLocator,
        M5SourceLocatorKind::LocalPathSource,
        M5AcquisitionSurfaceContext::ShellSurface,
        "local-path.acme/repo",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    );
    base.resolved_root_or_container = "   ".to_owned();
    locator(base)
}

/// Degraded locator entry: the literal target was rewritten into a different acquisition verb (a clone silently
/// reopened over an existing local checkout).
fn locator_verb_rewritten() -> M5ResolvedSourceLocatorEntry {
    let mut base = clean_locator_base(
        "locator:workspace:verb-rewritten",
        "entry.acme.clone-remote",
        "source.locator.remote_forge",
        M5RepositoryBootstrapRole::SourceLocator,
        M5SourceLocatorKind::RemoteForgeUrlSource,
        M5AcquisitionSurfaceContext::DiagnosticsSurface,
        "remote-forge.acme/org/repo",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.disclosed",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    );
    base.touches_network_or_mirror = true;
    base.literal_target_preserved = false;
    locator(base)
}

/// Degraded locator entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn locator_unbound() -> M5ResolvedSourceLocatorEntry {
    let mut base = clean_locator_base(
        "locator:git:unbound",
        "entry.acme.mirrored",
        "source.locator.mirror",
        M5RepositoryBootstrapRole::SourceLocator,
        M5SourceLocatorKind::MirrorSource,
        M5AcquisitionSurfaceContext::AdminSurface,
        "mirror.acme/org/repo",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.disclosed",
        "mirror-provenance.acme.v3",
        "mirror-hint.airgap",
    );
    base.touches_network_or_mirror = true;
    base.bound_to_registry = false;
    locator(base)
}

/// Degraded locator entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn locator_form_incomplete() -> M5ResolvedSourceLocatorEntry {
    let mut base = clean_locator_base(
        "locator:entry:form-incomplete",
        "entry.acme.open-archive",
        "source.locator.archive_bundle",
        M5RepositoryBootstrapRole::SourceLocator,
        M5SourceLocatorKind::ArchiveImportBundleSource,
        M5AcquisitionSurfaceContext::EntrySurface,
        "archive-bundle.acme/pack.bundle",
        "archive-container.acme/pack",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "air-gap-hint.offline",
    );
    base.resolution_form_coverage = vec![M5AcquisitionResolutionForm::CanonicalObject];
    locator(base)
}

/// Degraded locator entry: the canonical registry token name is unstated.
fn locator_token_unstated() -> M5ResolvedSourceLocatorEntry {
    let mut base = clean_locator_base(
        "locator:support:token-unstated",
        "entry.acme.managed-snapshot",
        "  ",
        M5RepositoryBootstrapRole::SourceLocator,
        M5SourceLocatorKind::ManagedSnapshotSource,
        M5AcquisitionSurfaceContext::SupportOrExportForm,
        "managed-snapshot.acme/snap-0007",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    );
    base.token_name = "  ".to_owned();
    locator(base)
}

// -- Clean checkout-plan entries ----------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_plan_base(
    entry_id: &str,
    source_ref: &str,
    token_name: &str,
    semantic_role: M5RepositoryBootstrapRole,
    checkout_mode: M5CheckoutMode,
    surface_context: M5AcquisitionSurfaceContext,
    ref_selection: &str,
    depth_filter: &str,
    submodule_mode: &str,
    lfs_posture: &str,
    destination_path: &str,
    cost_band: &str,
) -> M5CheckoutPlanEntryResolutionInput {
    M5CheckoutPlanEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        source_ref: source_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        checkout_mode,
        surface_context,
        resolution_form_coverage: all_forms(),
        ref_selection: ref_selection.to_owned(),
        depth_filter: depth_filter.to_owned(),
        submodule_mode: submodule_mode.to_owned(),
        lfs_posture: lfs_posture.to_owned(),
        destination_path: destination_path.to_owned(),
        cost_band: cost_band.to_owned(),
        keeps_cost_visible_before_mutation: true,
        plan_is_truthful: true,
        repo_owned_action_scheduled: false,
        repo_owned_action_staged_not_auto_run: false,
        implicit_mutation_asserted: false,
        implicit_mutation_explained: false,
        proof_fresh: true,
    }
}

fn plan_full_shell_clean() -> M5ResolvedCheckoutPlanEntry {
    plan(clean_plan_base(
        "plan:shell:full",
        "entry.acme.open-local",
        "checkout.plan.full",
        M5RepositoryBootstrapRole::CheckoutPlan,
        M5CheckoutMode::FullCheckoutPlan,
        M5AcquisitionSurfaceContext::ShellSurface,
        "ref.main",
        "depth.full",
        "submodule.recursive-staged",
        "lfs.deferred",
        "destination.acme/repo",
        "cost.small",
    ))
}

fn plan_partial_entry_clean() -> M5ResolvedCheckoutPlanEntry {
    // A repo-owned action (a package restore) is scheduled but staged, never run implicitly during acquisition.
    let mut base = clean_plan_base(
        "plan:entry:partial",
        "entry.acme.clone-remote",
        "checkout.plan.partial",
        M5RepositoryBootstrapRole::StagedTrust,
        M5CheckoutMode::PartialOrShallowCheckoutPlan,
        M5AcquisitionSurfaceContext::EntrySurface,
        "ref.main",
        "depth.shallow-1",
        "submodule.none",
        "lfs.deferred",
        "destination.acme/repo",
        "cost.medium",
    );
    base.repo_owned_action_scheduled = true;
    base.repo_owned_action_staged_not_auto_run = true;
    plan(base)
}

fn plan_sparse_diagnostics_clean() -> M5ResolvedCheckoutPlanEntry {
    // A justified implicit mutation stays honest: it is explained on this diagnostics surface.
    let mut base = clean_plan_base(
        "plan:diagnostics:sparse",
        "entry.acme.open-archive",
        "checkout.plan.sparse",
        M5RepositoryBootstrapRole::CheckoutPlan,
        M5CheckoutMode::SparseCheckoutPlan,
        M5AcquisitionSurfaceContext::DiagnosticsSurface,
        "ref.tag-v1",
        "filter.blob-none",
        "submodule.none",
        "lfs.eager-staged",
        "destination.acme/repo",
        "cost.small",
    );
    base.implicit_mutation_asserted = true;
    base.implicit_mutation_explained = true;
    plan(base)
}

fn plan_partial_admin_clean() -> M5ResolvedCheckoutPlanEntry {
    plan(clean_plan_base(
        "plan:admin:partial",
        "entry.acme.mirrored",
        "checkout.plan.partial",
        M5RepositoryBootstrapRole::CheckoutPlan,
        M5CheckoutMode::PartialOrShallowCheckoutPlan,
        M5AcquisitionSurfaceContext::AdminSurface,
        "ref.main",
        "depth.shallow-1",
        "submodule.recursive-staged",
        "lfs.deferred",
        "destination.acme/repo",
        "cost.medium",
    ))
}

fn plan_full_support_clean() -> M5ResolvedCheckoutPlanEntry {
    plan(clean_plan_base(
        "plan:support:full",
        "entry.acme.managed-snapshot",
        "checkout.plan.full",
        M5RepositoryBootstrapRole::CheckoutPlan,
        M5CheckoutMode::FullCheckoutPlan,
        M5AcquisitionSurfaceContext::SupportOrExportForm,
        "ref.main",
        "depth.full",
        "submodule.recursive-staged",
        "lfs.deferred",
        "destination.acme/repo",
        "cost.small",
    ))
}

// -- Degraded checkout-plan entries -------------------------------------------------------------

/// Degraded plan entry: the plan would run a repo-owned action implicitly during acquisition — the checkout
/// reads as safe when it has quietly become an implicit bootstrap.
fn plan_implicit_bootstrap() -> M5ResolvedCheckoutPlanEntry {
    let mut base = clean_plan_base(
        "plan:shell:implicit",
        "entry.acme.open-local",
        "checkout.plan.full",
        M5RepositoryBootstrapRole::CheckoutPlan,
        M5CheckoutMode::FullCheckoutPlan,
        M5AcquisitionSurfaceContext::ShellSurface,
        "ref.main",
        "depth.full",
        "submodule.recursive-staged",
        "lfs.deferred",
        "destination.acme/repo",
        "cost.small",
    );
    base.repo_owned_action_scheduled = true;
    base.repo_owned_action_staged_not_auto_run = false;
    plan(base)
}

/// Degraded plan entry: the canonical / accessible / audit resolution-form coverage of the plan is incomplete.
fn plan_form_incomplete() -> M5ResolvedCheckoutPlanEntry {
    let mut base = clean_plan_base(
        "plan:entry:form-incomplete",
        "entry.acme.clone-remote",
        "checkout.plan.partial",
        M5RepositoryBootstrapRole::CheckoutPlan,
        M5CheckoutMode::PartialOrShallowCheckoutPlan,
        M5AcquisitionSurfaceContext::EntrySurface,
        "ref.main",
        "depth.shallow-1",
        "submodule.none",
        "lfs.deferred",
        "destination.acme/repo",
        "cost.medium",
    );
    base.resolution_form_coverage = vec![M5AcquisitionResolutionForm::CanonicalObject];
    plan(base)
}

/// Degraded plan entry: the checkout mode is unclassified.
fn plan_mode_unclassified() -> M5ResolvedCheckoutPlanEntry {
    plan(clean_plan_base(
        "plan:git:mode-unclassified",
        "entry.acme.mirrored",
        "checkout.plan.unknown",
        M5RepositoryBootstrapRole::CheckoutPlan,
        M5CheckoutMode::ModeUnclassified,
        M5AcquisitionSurfaceContext::AdminSurface,
        "ref.main",
        "depth.full",
        "submodule.none",
        "lfs.deferred",
        "destination.acme/repo",
        "cost.small",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SourceLocatorCheckoutPlanRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    source_locator_entries: Vec<M5ResolvedSourceLocatorEntry>,
    checkout_plan_entries: Vec<M5ResolvedCheckoutPlanEntry>,
) -> M5SourceLocatorCheckoutPlanRegistriesRow {
    M5SourceLocatorCheckoutPlanRegistriesRow {
        consumer_surface,
        qualification: M5RepositoryBootstrapQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5RepositoryBootstrapDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5RepositoryBootstrapRequiredLabel::Identity,
            M5RepositoryBootstrapRequiredLabel::SemanticRole,
            M5RepositoryBootstrapRequiredLabel::RegistryReference,
            M5RepositoryBootstrapRequiredLabel::SourceLocator,
            M5RepositoryBootstrapRequiredLabel::CheckoutPlan,
        ],
        accessibility_routes: M5RepositoryBootstrapAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5AcquisitionAnatomyPart::ALL.to_vec(),
        export_fields: M5AcquisitionExportField::ALL.to_vec(),
        downgrade_triggers,
        source_locator_entries,
        checkout_plan_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_SCHEMA_REF,
            M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
            M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        ]),
        rewrites_clone_into_open_when_local_checkout_already_exists: false,
        runs_repo_owned_actions_implicitly_during_acquisition: false,
        hides_checkout_cost_topology_or_credential_posture_before_mutation: false,
        collapses_distinct_acquisition_verbs_into_one_runtime_path: false,
    }
}

fn registry_rows() -> Vec<M5SourceLocatorCheckoutPlanRegistriesRow> {
    use M5RepositoryBootstrapConsumerSurface as C;
    use M5RepositoryBootstrapDowngradeTrigger as D;

    vec![
        base_row(
            C::AcquisitionEngine,
            "Acquisition-engine owner",
            "The acquisition engine resolves the local-path source locator to one stable object — literal target, resolved checkout root, staged-trust metadata, disclosed credential posture, signer provenance, and the distinct mirror / air-gap hint — from the shared registry and derives the full checkout plan; a locator object missing its resolved root and a checkout plan that would run a repo-owned action implicitly degrade honestly instead of reading as a clean pass",
            "evidence:m5-repository-bootstrap-acquisition-engine:001",
            vec![
                D::SourceLocatorUnstated,
                D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
                D::ProofStale,
            ],
            vec![locator_shell_localpath_clean(), locator_object_incomplete()],
            vec![plan_full_shell_clean(), plan_implicit_bootstrap()],
        ),
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the remote-forge source locator while disclosing its credential posture before the fetch, and renders the partial checkout plan; a resolution-form gap on a locator entry and on a checkout plan is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-repository-bootstrap-shell-ui:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![locator_entry_remote_clean(), locator_form_incomplete()],
            vec![plan_partial_entry_clean(), plan_form_incomplete()],
        ),
        base_row(
            C::WorkspaceService,
            "Workspace-service owner",
            "The workspace service reports the archive / import-bundle source locator and the sparse checkout plan without manual reconstruction; a clone whose literal target was silently reopened over an existing local checkout is caught as a verb rewrite",
            "evidence:m5-repository-bootstrap-workspace-service:001",
            vec![
                D::RewroteCloneIntoOpenWhenLocalCheckoutAlreadyExists,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                locator_diagnostics_archive_clean(),
                locator_verb_rewritten(),
            ],
            vec![plan_sparse_diagnostics_clean()],
        ),
        base_row(
            C::GitService,
            "Git-service owner",
            "The git service resolves the mirror source locator while keeping signer / mirror provenance continuous and bound to the registry; a locator that is a hand-copied per-entry assumption and a checkout plan on an unclassified mode degrade honestly",
            "evidence:m5-repository-bootstrap-git-service:001",
            vec![
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![locator_admin_mirror_clean(), locator_unbound()],
            vec![plan_partial_admin_clean(), plan_mode_unclassified()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved source-locator and checkout-plan truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied acquisition table",
            "evidence:m5-repository-bootstrap-diagnostics:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                locator_diagnostics_archive_clean(),
                locator_form_incomplete(),
            ],
            vec![plan_partial_entry_clean(), plan_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved source-locator and checkout-plan truth, so a hand-copied constant, an unstated registry token, a verb rewrite, or an implicit bootstrap is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-repository-bootstrap-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::HidBootstrapCredentialPostureBehindGenericConnectedStateCopy,
                D::ProofStale,
            ],
            vec![locator_support_snapshot_clean(), locator_token_unstated()],
            vec![plan_full_support_clean()],
        ),
    ]
}

fn governance_review() -> M5SourceLocatorCheckoutPlanRegistriesGovernanceReview {
    M5SourceLocatorCheckoutPlanRegistriesGovernanceReview {
        locator_registry_names_token_role_and_kind: true,
        entry_flow_resolves_to_stable_object_from_shared_registry: true,
        literal_target_root_trust_and_provenance_published: true,
        open_and_clone_stay_distinct_verbs: true,
        checkout_plan_keeps_cost_visible_and_stages_trust: true,
        credential_posture_disclosed_before_network: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shell_entry_diagnostics_admin_read_single_source: true,
        locator_or_plan_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SourceLocatorCheckoutPlanRegistriesConsumerProjection {
    M5SourceLocatorCheckoutPlanRegistriesConsumerProjection {
        shell_and_entry_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        git_and_workspace_services_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SourceLocatorCheckoutPlanRegistriesProofFreshness {
    M5SourceLocatorCheckoutPlanRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SourceLocatorCheckoutPlanRegistriesReleasePosture {
    M5SourceLocatorCheckoutPlanRegistriesReleasePosture {
        proof_packet_ref: M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_ARTIFACT_REF.to_owned(),
        repository_bootstrap_audit_ref: M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_SCHEMA_REF,
        M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 source-locator and checkout-plan registries packet.
pub fn seeded_m5_source_locator_and_checkout_plan_registries(
) -> M5SourceLocatorCheckoutPlanRegistriesPacket {
    M5SourceLocatorCheckoutPlanRegistriesPacket::new(
        M5SourceLocatorCheckoutPlanRegistriesPacketInput {
            packet_id: M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 source-locator and checkout-plan registries with one stable source-locator object resolving per entry flow, open and clone staying distinct verbs with a preserved literal target, the bootstrap credential posture disclosed before any network or mirror fetch, canonical / accessible / audit resolution-form coverage, and the complete ref-selection / depth-filter / submodule-mode / LFS-posture / destination-path / cost-band checkout-plan object across acquisition-engine, shell, workspace, git, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5SourceLocatorCheckoutPlanRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the acquisition-engine row is held at Beta pending local-path source-locator parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_source_locator_and_checkout_plan_registries_local_path_source_beta_narrowed(
) -> M5SourceLocatorCheckoutPlanRegistriesPacket {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.packet_id =
        "m5-source-locator-and-checkout-plan-registries:local-path-source-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepositoryBootstrapConsumerSurface::AcquisitionEngine)
        .expect("acquisition-engine row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Beta;
    packet
}

/// Narrowed variant: the workspace-service row is narrowed to Preview pending sparse checkout-plan parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_source_locator_and_checkout_plan_registries_sparse_checkout_preview_narrowed(
) -> M5SourceLocatorCheckoutPlanRegistriesPacket {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.packet_id =
        "m5-source-locator-and-checkout-plan-registries:sparse-checkout-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepositoryBootstrapConsumerSurface::WorkspaceService)
        .expect("workspace-service row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Preview;
    packet
}
