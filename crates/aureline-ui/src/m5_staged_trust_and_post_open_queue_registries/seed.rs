//! Canonical seed builders for the M5 staged-trust and post-open-queue registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean staged-trust and post-open-queue entries are built so
//! the one stable staged-trust object resolving per acquisition path, the staged trust staying browse-safe with
//! no repo-owned action running implicitly, an explicit approval recorded before any trust-widening stage, the
//! canonical / accessible / audit resolution forms, and the complete queue-item-kind / execution-site /
//! trust-consequence / network-consequence / approval-requirement / attribution post-open-queue object are proven
//! across the acquisition-engine, git, trust, diagnostics, CLI, and support surfaces without any hand-copied
//! per-entry assumption, implicitly-executing queue item, widened-early trust, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_PACKET_ID: &str =
    "m5-staged-trust-and-post-open-queue-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn staging(input: M5StagedTrustEntryResolutionInput) -> M5ResolvedStagedTrustEntry {
    resolve_staged_trust_entry(input).expect("seed staged-trust entry resolves")
}

fn queue(input: M5PostOpenQueueEntryResolutionInput) -> M5ResolvedPostOpenQueueEntry {
    resolve_post_open_queue_entry(input).expect("seed post-open-queue entry resolves")
}

fn all_forms() -> Vec<M5StagingResolutionForm> {
    M5StagingResolutionForm::ALL.to_vec()
}

// -- Clean staged-trust entries (stable object, browse-safe, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_staging_base(
    entry_id: &str,
    acquisition_path_id: &str,
    token_name: &str,
    semantic_role: M5RepositoryBootstrapRole,
    trust_stage_kind: M5TrustStageKind,
    surface_context: M5StagingSurfaceContext,
    browse_scope_ref: &str,
    computed_metadata_ref: &str,
    deferred_repo_action_set: &str,
    trust_prompt_policy: &str,
    explicit_approval_reference: &str,
    staged_trust_provenance: &str,
) -> M5StagedTrustEntryResolutionInput {
    M5StagedTrustEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        acquisition_path_id: acquisition_path_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        trust_stage_kind,
        surface_context,
        resolution_form_coverage: all_forms(),
        browse_scope_ref: browse_scope_ref.to_owned(),
        computed_metadata_ref: computed_metadata_ref.to_owned(),
        deferred_repo_action_set: deferred_repo_action_set.to_owned(),
        trust_prompt_policy: trust_prompt_policy.to_owned(),
        explicit_approval_reference: explicit_approval_reference.to_owned(),
        staged_trust_provenance: staged_trust_provenance.to_owned(),
        bound_to_registry: true,
        browse_safe_before_widening: true,
        widens_trust_or_runs_code: false,
        explicit_approval_recorded: true,
        proof_fresh: true,
    }
}

fn staging_acq_browse_clean() -> M5ResolvedStagedTrustEntry {
    staging(clean_staging_base(
        "staging:acquisition:browse-tree-and-manifests",
        "entry.acme.open-local",
        "staged.trust.browse_tree_and_manifests",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::BrowseTreeAndManifests,
        M5StagingSurfaceContext::ShellSurface,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/hooks-tasks-extensions",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    ))
}

fn staging_git_metadata_clean() -> M5ResolvedStagedTrustEntry {
    staging(clean_staging_base(
        "staging:git:compute-safe-metadata",
        "entry.acme.clone-remote",
        "staged.trust.compute_safe_metadata",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::ComputeSafeMetadata,
        M5StagingSurfaceContext::EntrySurface,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/restore-submodule-lfs",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    ))
}

fn staging_diagnostics_review_clean() -> M5ResolvedStagedTrustEntry {
    staging(clean_staging_base(
        "staging:diagnostics:review-deferred-repo-actions",
        "entry.acme.open-archive",
        "staged.trust.review_deferred_repo_actions",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::ReviewDeferredRepoActions,
        M5StagingSurfaceContext::DiagnosticsSurface,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/generator-install",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    ))
}

fn staging_admin_run_clean() -> M5ResolvedStagedTrustEntry {
    // A run-repo-owned-action stage widens trust and records an explicit approval before it may run.
    let mut base = clean_staging_base(
        "staging:admin:run-repo-owned-action",
        "entry.acme.import-bundle",
        "staged.trust.run_repo_owned_action_after_approval",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::RunRepoOwnedActionAfterApproval,
        M5StagingSurfaceContext::AdminSurface,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/hooks-tasks",
        "trust-prompt.explicit-approval.v3",
        "approval-ref.acme/handle-0007",
        "staged-trust-provenance.acme.v3",
    );
    base.widens_trust_or_runs_code = true;
    base.explicit_approval_recorded = true;
    staging(base)
}

fn staging_support_hydrate_clean() -> M5ResolvedStagedTrustEntry {
    // A hydrate-network-content stage widens trust and records an explicit approval before it may run.
    let mut base = clean_staging_base(
        "staging:support:hydrate-network-content",
        "entry.acme.resume-snapshot",
        "staged.trust.hydrate_network_content_after_approval",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::HydrateNetworkContentAfterApproval,
        M5StagingSurfaceContext::SupportOrExportForm,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/submodule-lfs-restore",
        "trust-prompt.explicit-approval.v3",
        "approval-ref.acme/handle-0042",
        "staged-trust-provenance.acme.v3",
    );
    base.widens_trust_or_runs_code = true;
    base.explicit_approval_recorded = true;
    staging(base)
}

// -- Degraded staged-trust entries --------------------------------------------------------------

/// Degraded staging entry: the resolved staged-trust object is incomplete — the deferred repo-owned action set
/// is unstated.
fn staging_object_incomplete() -> M5ResolvedStagedTrustEntry {
    let mut base = clean_staging_base(
        "staging:acquisition:incomplete",
        "entry.acme.open-local",
        "staged.trust.browse_tree_and_manifests",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::BrowseTreeAndManifests,
        M5StagingSurfaceContext::ShellSurface,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/hooks-tasks-extensions",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    );
    base.deferred_repo_action_set = "   ".to_owned();
    staging(base)
}

/// Degraded staging entry: a trust-widening stage would run a repo-owned action before an explicit approval is
/// recorded.
fn staging_widen_early() -> M5ResolvedStagedTrustEntry {
    let mut base = clean_staging_base(
        "staging:trust:widen-early",
        "entry.acme.import-bundle",
        "staged.trust.run_repo_owned_action_after_approval",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::RunRepoOwnedActionAfterApproval,
        M5StagingSurfaceContext::DiagnosticsSurface,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/hooks-tasks",
        "trust-prompt.explicit-approval.v3",
        "approval-ref.acme/handle-0007",
        "staged-trust-provenance.acme.v3",
    );
    base.widens_trust_or_runs_code = true;
    base.explicit_approval_recorded = false;
    staging(base)
}

/// Degraded staging entry: the behavior is a hand-copied per-entry assumption instead of tracing to the registry.
fn staging_unbound() -> M5ResolvedStagedTrustEntry {
    let mut base = clean_staging_base(
        "staging:diagnostics:unbound",
        "entry.acme.open-archive",
        "staged.trust.review_deferred_repo_actions",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::ReviewDeferredRepoActions,
        M5StagingSurfaceContext::AdminSurface,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/generator-install",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    );
    base.bound_to_registry = false;
    staging(base)
}

/// Degraded staging entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn staging_form_incomplete() -> M5ResolvedStagedTrustEntry {
    let mut base = clean_staging_base(
        "staging:git:form-incomplete",
        "entry.acme.clone-remote",
        "staged.trust.compute_safe_metadata",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::ComputeSafeMetadata,
        M5StagingSurfaceContext::EntrySurface,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/restore-submodule-lfs",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    );
    base.resolution_form_coverage = vec![M5StagingResolutionForm::CanonicalObject];
    staging(base)
}

/// Degraded staging entry: the canonical registry token name is unstated.
fn staging_token_unstated() -> M5ResolvedStagedTrustEntry {
    let mut base = clean_staging_base(
        "staging:support:token-unstated",
        "entry.acme.resume-snapshot",
        "  ",
        M5RepositoryBootstrapRole::StagedTrust,
        M5TrustStageKind::HydrateNetworkContentAfterApproval,
        M5StagingSurfaceContext::SupportOrExportForm,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/submodule-lfs-restore",
        "trust-prompt.explicit-approval.v3",
        "approval-ref.acme/handle-0042",
        "staged-trust-provenance.acme.v3",
    );
    base.token_name = "  ".to_owned();
    base.widens_trust_or_runs_code = true;
    base.explicit_approval_recorded = true;
    staging(base)
}

// -- Clean post-open-queue entries --------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_queue_base(
    entry_id: &str,
    source_ref: &str,
    token_name: &str,
    semantic_role: M5RepositoryBootstrapRole,
    queue_class: M5PostOpenQueueClass,
    surface_context: M5StagingSurfaceContext,
    queue_item_kind: &str,
    execution_site: &str,
    trust_consequence: &str,
    network_consequence: &str,
    approval_requirement: &str,
    attribution_ref: &str,
) -> M5PostOpenQueueEntryResolutionInput {
    M5PostOpenQueueEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        source_ref: source_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        queue_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        queue_item_kind: queue_item_kind.to_owned(),
        execution_site: execution_site.to_owned(),
        trust_consequence: trust_consequence.to_owned(),
        network_consequence: network_consequence.to_owned(),
        approval_requirement: approval_requirement.to_owned(),
        attribution_ref: attribution_ref.to_owned(),
        identifies_run_site_and_consequence: true,
        item_is_truthfully_typed: true,
        is_protected_item: false,
        explicit_approval_or_policy_gated: false,
        schedules_deferred_followup: false,
        followup_is_disclosed: false,
        auto_executes_during_acquisition: false,
        proof_fresh: true,
    }
}

fn queue_runs_code_shell_clean() -> M5ResolvedPostOpenQueueEntry {
    // A runs-repo-owned-code item is protected and gated behind an explicit approval.
    let mut base = clean_queue_base(
        "queue:acquisition:runs-repo-owned-code",
        "entry.acme.clone-remote",
        "post.open.queue.runs_repo_owned_code",
        M5RepositoryBootstrapRole::PostOpenQueue,
        M5PostOpenQueueClass::RunsRepoOwnedCode,
        M5StagingSurfaceContext::ShellSurface,
        "queue-item.repo-hook-or-task",
        "site.worktree",
        "consequence.widens-trust-runs-code",
        "consequence.offline",
        "approval.explicit-required",
        "attribution.acquisition-engine",
    );
    base.is_protected_item = true;
    base.explicit_approval_or_policy_gated = true;
    queue(base)
}

fn queue_hydrates_entry_clean() -> M5ResolvedPostOpenQueueEntry {
    // A hydrates-network-backed-content item is protected, gated, and schedules a disclosed follow-up.
    let mut base = clean_queue_base(
        "queue:git:hydrates-network-backed-content",
        "entry.acme.clone-remote",
        "post.open.queue.hydrates_network_backed_content",
        M5RepositoryBootstrapRole::PostOpenQueue,
        M5PostOpenQueueClass::HydratesNetworkBackedContent,
        M5StagingSurfaceContext::EntrySurface,
        "queue-item.submodule-init-or-lfs-hydrate",
        "site.network",
        "consequence.widens-trust",
        "consequence.hydrates-network",
        "approval.explicit-required",
        "attribution.git-service",
    );
    base.is_protected_item = true;
    base.explicit_approval_or_policy_gated = true;
    base.schedules_deferred_followup = true;
    base.followup_is_disclosed = true;
    queue(base)
}

fn queue_mutates_diag_clean() -> M5ResolvedPostOpenQueueEntry {
    // A mutates-reviewed-checkout item is protected and gated behind an explicit policy.
    let mut base = clean_queue_base(
        "queue:trust:mutates-reviewed-checkout",
        "entry.acme.open-archive",
        "post.open.queue.mutates_reviewed_checkout",
        M5RepositoryBootstrapRole::PostOpenQueue,
        M5PostOpenQueueClass::MutatesReviewedCheckout,
        M5StagingSurfaceContext::DiagnosticsSurface,
        "queue-item.index-warmup-or-docs-import",
        "site.git-dir",
        "consequence.mutates-checkout",
        "consequence.offline",
        "approval.policy-allowed",
        "attribution.trust-service",
    );
    base.is_protected_item = true;
    base.explicit_approval_or_policy_gated = true;
    queue(base)
}

fn queue_inert_admin_clean() -> M5ResolvedPostOpenQueueEntry {
    // An inert recommendation presents only; it is not protected and needs no approval.
    queue(clean_queue_base(
        "queue:diagnostics:inert-recommendation",
        "entry.acme.import-bundle",
        "post.open.queue.inert_recommendation",
        M5RepositoryBootstrapRole::PostOpenQueue,
        M5PostOpenQueueClass::InertRecommendation,
        M5StagingSurfaceContext::AdminSurface,
        "queue-item.bundle-recommendation-or-trust-prompt",
        "site.presentation-only",
        "consequence.no-trust-change",
        "consequence.offline",
        "approval.none-inert",
        "attribution.diagnostics",
    ))
}

fn queue_runs_code_support_clean() -> M5ResolvedPostOpenQueueEntry {
    let mut base = clean_queue_base(
        "queue:support:runs-repo-owned-code",
        "entry.acme.clone-remote",
        "post.open.queue.runs_repo_owned_code",
        M5RepositoryBootstrapRole::PostOpenQueue,
        M5PostOpenQueueClass::RunsRepoOwnedCode,
        M5StagingSurfaceContext::SupportOrExportForm,
        "queue-item.generator-install",
        "site.extension-host",
        "consequence.widens-trust-runs-code",
        "consequence.offline",
        "approval.explicit-required",
        "attribution.support-export",
    );
    base.is_protected_item = true;
    base.explicit_approval_or_policy_gated = true;
    queue(base)
}

// -- Degraded post-open-queue entries -----------------------------------------------------------

/// Degraded queue entry: a protected item would auto-execute during acquisition — a hook, task, or hydration
/// step runs merely because a path was opened or cloned, so the item reads as unsafe.
fn queue_implicit_execution() -> M5ResolvedPostOpenQueueEntry {
    let mut base = clean_queue_base(
        "queue:acquisition:implicit-execution",
        "entry.acme.clone-remote",
        "post.open.queue.runs_repo_owned_code",
        M5RepositoryBootstrapRole::PostOpenQueue,
        M5PostOpenQueueClass::RunsRepoOwnedCode,
        M5StagingSurfaceContext::ShellSurface,
        "queue-item.repo-hook-or-task",
        "site.worktree",
        "consequence.widens-trust-runs-code",
        "consequence.offline",
        "approval.explicit-required",
        "attribution.acquisition-engine",
    );
    base.is_protected_item = true;
    base.explicit_approval_or_policy_gated = true;
    base.auto_executes_during_acquisition = true;
    queue(base)
}

/// Degraded queue entry: the canonical / accessible / audit resolution-form coverage of the queue item is
/// incomplete.
fn queue_form_incomplete() -> M5ResolvedPostOpenQueueEntry {
    let mut base = clean_queue_base(
        "queue:git:form-incomplete",
        "entry.acme.clone-remote",
        "post.open.queue.hydrates_network_backed_content",
        M5RepositoryBootstrapRole::PostOpenQueue,
        M5PostOpenQueueClass::HydratesNetworkBackedContent,
        M5StagingSurfaceContext::EntrySurface,
        "queue-item.submodule-init-or-lfs-hydrate",
        "site.network",
        "consequence.widens-trust",
        "consequence.hydrates-network",
        "approval.explicit-required",
        "attribution.git-service",
    );
    base.is_protected_item = true;
    base.explicit_approval_or_policy_gated = true;
    base.resolution_form_coverage = vec![M5StagingResolutionForm::CanonicalObject];
    queue(base)
}

/// Degraded queue entry: the post-open-queue class is unclassified.
fn queue_class_unclassified() -> M5ResolvedPostOpenQueueEntry {
    queue(clean_queue_base(
        "queue:diagnostics:class-unclassified",
        "entry.acme.import-bundle",
        "post.open.queue.unknown",
        M5RepositoryBootstrapRole::PostOpenQueue,
        M5PostOpenQueueClass::ClassUnclassified,
        M5StagingSurfaceContext::AdminSurface,
        "queue-item.bundle-recommendation-or-trust-prompt",
        "site.presentation-only",
        "consequence.no-trust-change",
        "consequence.offline",
        "approval.none-inert",
        "attribution.diagnostics",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5StagedTrustPostOpenQueueRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    staged_trust_entries: Vec<M5ResolvedStagedTrustEntry>,
    post_open_queue_entries: Vec<M5ResolvedPostOpenQueueEntry>,
) -> M5StagedTrustPostOpenQueueRegistriesRow {
    M5StagedTrustPostOpenQueueRegistriesRow {
        consumer_surface,
        qualification: M5RepositoryBootstrapQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5RepositoryBootstrapDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5RepositoryBootstrapRequiredLabel::Identity,
            M5RepositoryBootstrapRequiredLabel::SemanticRole,
            M5RepositoryBootstrapRequiredLabel::RegistryReference,
            M5RepositoryBootstrapRequiredLabel::CredentialPosture,
            M5RepositoryBootstrapRequiredLabel::CheckoutPlan,
        ],
        accessibility_routes: M5RepositoryBootstrapAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5StagingAnatomyPart::ALL.to_vec(),
        export_fields: M5StagingExportField::ALL.to_vec(),
        downgrade_triggers,
        staged_trust_entries,
        post_open_queue_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_SCHEMA_REF,
            M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
            M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
        ]),
        runs_repo_owned_actions_implicitly_during_acquisition: false,
        auto_executes_post_open_bootstrap_queue_without_explicit_approval: false,
        hides_what_a_queue_item_would_run_or_its_trust_or_network_consequence: false,
        widens_trust_before_browse_safe_metadata_is_computed: false,
    }
}

fn registry_rows() -> Vec<M5StagedTrustPostOpenQueueRegistriesRow> {
    use M5RepositoryBootstrapConsumerSurface as C;
    use M5RepositoryBootstrapDowngradeTrigger as D;

    vec![
        base_row(
            C::AcquisitionEngine,
            "Acquisition-engine owner",
            "The acquisition engine resolves the browse-tree-and-manifests trust stage to one stable object — browse scope, computed metadata, deferred repo-owned action set, trust-prompt policy, explicit-approval reference, and staged-trust provenance — from the shared registry and derives the runs-repo-owned-code post-open queue item gated behind an explicit approval; a staged-trust object missing its deferred action set and a queue item that would auto-execute a hook merely because a path was cloned degrade honestly instead of reading as a clean pass",
            "evidence:m5-repository-bootstrap-acquisition-engine:001",
            vec![
                D::StagedTrustRuleUnstated,
                D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
                D::ProofStale,
            ],
            vec![staging_acq_browse_clean(), staging_object_incomplete()],
            vec![queue_runs_code_shell_clean(), queue_implicit_execution()],
        ),
        base_row(
            C::GitService,
            "Git-service owner",
            "The git service resolves the compute-safe-metadata trust stage while keeping the tree browse-safe before any hydration, and renders the hydrates-network-backed-content post-open queue item gated behind an explicit approval with a disclosed follow-up; a resolution-form gap on a staging entry and on a queue item is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-repository-bootstrap-git-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![staging_git_metadata_clean(), staging_form_incomplete()],
            vec![queue_hydrates_entry_clean(), queue_form_incomplete()],
        ),
        base_row(
            C::TrustService,
            "Trust-service owner",
            "The trust service reports the review-deferred-repo-actions trust stage and the mutates-reviewed-checkout post-open queue item without manual reconstruction; a run-repo-owned-action stage that would widen trust before an explicit approval is recorded is caught as an early trust widening",
            "evidence:m5-repository-bootstrap-trust-service:001",
            vec![
                D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![staging_diagnostics_review_clean(), staging_widen_early()],
            vec![queue_mutates_diag_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics resolves the run-repo-owned-action-after-approval trust stage while keeping it browse-safe and bound to the registry, and renders the inert-recommendation post-open queue item; a staging entry that is a hand-copied per-entry assumption and a queue item on an unclassified class degrade honestly",
            "evidence:m5-repository-bootstrap-diagnostics:001",
            vec![
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![staging_admin_run_clean(), staging_unbound()],
            vec![queue_inert_admin_clean(), queue_class_unclassified()],
        ),
        base_row(
            C::CliExport,
            "CLI-export owner",
            "The CLI export renders the same resolved staged-trust and post-open-queue truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied queue table",
            "evidence:m5-repository-bootstrap-cli-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![staging_diagnostics_review_clean(), staging_form_incomplete()],
            vec![queue_hydrates_entry_clean(), queue_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved staged-trust and post-open-queue truth without embedding raw secrets, so a hand-copied constant, an unstated registry token, an implicitly-executing queue item, or a trust widened before browse-safe metadata is computed is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-repository-bootstrap-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::RanRepoOwnedActionsImplicitlyDuringAcquisition,
                D::ProofStale,
            ],
            vec![staging_support_hydrate_clean(), staging_token_unstated()],
            vec![queue_runs_code_support_clean()],
        ),
    ]
}

fn governance_review() -> M5StagedTrustPostOpenQueueRegistriesGovernanceReview {
    M5StagedTrustPostOpenQueueRegistriesGovernanceReview {
        staged_trust_registry_names_token_role_and_stage: true,
        entry_flow_resolves_to_stable_staging_from_shared_registry: true,
        browse_scope_metadata_deferred_actions_and_provenance_published: true,
        staged_trust_stays_browse_safe_no_implicit_repo_action: true,
        post_open_queue_identifies_run_site_and_consequence: true,
        protected_queue_item_gated_behind_explicit_approval_or_policy: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        acquisition_git_trust_diagnostics_read_single_source: true,
        staging_or_queue_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5StagedTrustPostOpenQueueRegistriesConsumerProjection {
    M5StagedTrustPostOpenQueueRegistriesConsumerProjection {
        acquisition_and_git_consume_shared_registries: true,
        trust_and_diagnostics_consume_shared_registries: true,
        cli_and_support_export_consume_shared_registries: true,
        docs_help_and_workspace_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5StagedTrustPostOpenQueueRegistriesProofFreshness {
    M5StagedTrustPostOpenQueueRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5StagedTrustPostOpenQueueRegistriesReleasePosture {
    M5StagedTrustPostOpenQueueRegistriesReleasePosture {
        proof_packet_ref: M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_ARTIFACT_REF.to_owned(),
        repository_bootstrap_audit_ref: M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_SCHEMA_REF,
        M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 staged-trust and post-open-queue registries packet.
pub fn seeded_m5_staged_trust_and_post_open_queue_registries(
) -> M5StagedTrustPostOpenQueueRegistriesPacket {
    M5StagedTrustPostOpenQueueRegistriesPacket::new(M5StagedTrustPostOpenQueueRegistriesPacketInput {
        packet_id: M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 staged-trust and post-open bootstrap-queue registries with one stable staged-trust object resolving per acquisition path, the staged trust staying browse-safe with no repo-owned action running implicitly and an explicit approval recorded before any trust-widening stage, canonical / accessible / audit resolution-form coverage, and the complete queue-item-kind / execution-site / trust-consequence / network-consequence / approval-requirement / attribution post-open-queue object across acquisition-engine, git, trust, diagnostics, CLI, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5StagedTrustPostOpenQueueRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the trust-service row is held at Beta pending deferred-hydrate follow-up parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_staged_trust_and_post_open_queue_registries_deferred_hydrate_beta_narrowed(
) -> M5StagedTrustPostOpenQueueRegistriesPacket {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.packet_id =
        "m5-staged-trust-and-post-open-queue-registries:deferred-hydrate-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepositoryBootstrapConsumerSurface::TrustService)
        .expect("trust-service row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics row is narrowed to Preview pending trust-prompt scheduling parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_staged_trust_and_post_open_queue_registries_trust_prompt_preview_narrowed(
) -> M5StagedTrustPostOpenQueueRegistriesPacket {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.packet_id =
        "m5-staged-trust-and-post-open-queue-registries:trust-prompt-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepositoryBootstrapConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Preview;
    packet
}
