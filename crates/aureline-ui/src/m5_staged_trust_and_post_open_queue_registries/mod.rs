//! Implemented M5 staged-trust and post-open bootstrap-queue registries.
//!
//! The frozen [repository-bootstrap matrix][matrix] names Aureline's five project-entry acquisition families
//! and locks their controlled vocabulary. This is the staged-trust + post-open-queue implement lane: it turns
//! the *staged-trust* grammar (how Aureline browses the tree, manifests, and docs and computes safe metadata
//! before any repo-owned hook, task, extension recommendation, package restore, submodule init, LFS hydrate,
//! or generator install can run) and the *post-open bootstrap queue* grammar (typed, attributable work objects
//! that run repo-owned code, hydrate network-backed content, mutate the reviewed checkout, or merely recommend)
//! into registry resolvers that produce export-safe, honest projections. Every claimed M5 acquisition path then
//! resolves to one stable staged-trust object — the trust-stage kind and canonical trust mode, the browse-scope
//! reference, the computed-metadata reference, the deferred repo-owned action set, the trust-prompt policy, the
//! explicit-approval reference, and the staged-trust provenance — and to one post-open-queue object — the
//! queue-item kind, the execution site, the trust consequence, the network consequence, the approval
//! requirement, and the attribution reference — that the acquisition, git, trust, diagnostics, CLI, and support
//! / export surfaces can inspect without manual reconstruction, so repository open stays useful before any
//! repo-owned action runs, a protected post-open queue item never auto-executes implicitly during acquisition,
//! trust is never widened before browse-safe metadata is computed, every queue row identifies exactly what
//! would run, where it would run, and what trust or network consequence it carries, and an acquisition path
//! that cannot explain its staged trust or its post-open queue degrades honestly instead of reading as a clean
//! pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one stable staged-trust object per acquisition path.** [`resolve_staged_trust_entry`] refuses to
//!   read as a clean, registry-bound staging entry unless it names a canonical registry token, a classified
//!   [trust stage][M5TrustStageKind], a repository-bootstrap role, covers every
//!   [resolution form][M5StagingResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every staging field (browse-scope reference, computed-metadata reference, deferred
//!   repo-owned action set, trust-prompt policy, explicit-approval reference, and staged-trust provenance),
//!   keeps the stage browse-safe before it widens trust, and records an explicit approval before any
//!   trust-widening or code-running stage; otherwise it degrades.
//! * **Keep the staged trust from running a repo-owned action implicitly or widening trust early.**
//!   [`staged_trust_stays_browse_safe`] rejects a stage that would run a repo-owned action or widen trust before
//!   browse-safe metadata is computed and an explicit approval is recorded, so it degrades to
//!   [`M5StagedTrustEntryDegradeReason::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly`].
//! * **Keep the post-open queue from executing implicitly or hiding its consequence.**
//!   [`resolve_post_open_queue_entry`] names a classified [queue class][M5PostOpenQueueClass], requires the full
//!   queue-item-kind / execution-site / trust-consequence / network-consequence / approval-requirement /
//!   attribution post-open-queue object, covers every resolution form, and degrades to
//!   [`M5PostOpenQueueEntryDegradeReason::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence`] when a
//!   protected item would auto-execute during acquisition, run ungated without explicit approval or policy, or
//!   hide what it would run and where, so a queue item can never read as safe when it has quietly executed a
//!   hook, task, extension, or hydration step merely because a path was opened or cloned.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5RepositoryBootstrapRole`] role
//! vocabulary and the [`M5RepositoryBootstrapConsumerSurface`] consumer-surface taxonomy — so the acquisition,
//! shell, git, trust, diagnostics, docs, CLI, and support surfaces can never fork their own staging or queue
//! meaning. Raw secret values, tokens, and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_repository_bootstrap_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_staged_trust_and_post_open_queue_registries,
    seeded_m5_staged_trust_and_post_open_queue_registries_deferred_hydrate_beta_narrowed,
    seeded_m5_staged_trust_and_post_open_queue_registries_trust_prompt_preview_narrowed,
    M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_repository_bootstrap_matrix::{
    M5RepositoryBootstrapAccessibilityRoute, M5RepositoryBootstrapConsumerSurface,
    M5RepositoryBootstrapDeploymentLine, M5RepositoryBootstrapDowngradeTrigger,
    M5RepositoryBootstrapFamily, M5RepositoryBootstrapQualificationClass,
    M5RepositoryBootstrapRequiredLabel, M5RepositoryBootstrapRole,
    M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF, M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
    M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF, M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5StagedTrustPostOpenQueueRegistriesPacket`].
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_staged_trust_and_post_open_queue_registries";

/// Schema version for M5 staged-trust / post-open-queue registry records.
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_SCHEMA_REF: &str =
    "schemas/workspaces/m5-staged-trust-and-post-open-queue-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_DOC_REF: &str =
    "docs/workspaces/m5_staged_trust_and_post_open_queue_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-staged-trust-and-post-open-queue-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-staged-trust-and-post-open-queue-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-staged-trust-and-post-open-queue-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/workspaces/m5-staged-trust-and-post-open-queue-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5StagedTrustPostOpenQueueRegistriesConsumerSurface = M5RepositoryBootstrapConsumerSurface;

/// One of the three resolution forms every staged-trust or post-open-queue entry must hold across so its truth
/// keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or written
/// to the audit / support record. Minted by this lane because the frozen matrix names the staged-trust and
/// post-open-queue *roles* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StagingResolutionForm {
    /// The canonical resolved staged-trust / post-open-queue object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved stage / queue discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved stage / queue inspectable off-renderer.
    AuditRecord,
}

impl M5StagingResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled trust-stage kind a staged-trust entry resolves, so the canonical staging model shares one
/// registry rather than a hand-copied per-entry assumption. Minted by this lane because the frozen matrix
/// carries the acquisition families but not the concrete browse-tree-and-manifests / compute-safe-metadata /
/// review-deferred-repo-actions / run-repo-owned-action-after-approval / hydrate-network-content-after-approval
/// stage a path resolves at. Every classified kind carries its canonical trust mode; the two trust-widening
/// stages carry code-running or network-hydrating consequence and so require an explicit approval before they
/// may run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustStageKind {
    /// Browse the repository tree, manifests, and docs (read-only; no repo-owned action).
    BrowseTreeAndManifests,
    /// Compute safe metadata from the parsed tree and manifests (no repo-owned action).
    ComputeSafeMetadata,
    /// Review the deferred repo-owned actions that would run, staged rather than executed.
    ReviewDeferredRepoActions,
    /// Run a repo-owned action (hook, task, extension, or generator install) after an explicit approval.
    RunRepoOwnedActionAfterApproval,
    /// Hydrate network-backed content (submodule init, LFS hydrate, or package restore) after an explicit
    /// approval.
    HydrateNetworkContentAfterApproval,
    /// The trust stage is unclassified, which is disallowed.
    StageUnclassified,
}

impl M5TrustStageKind {
    /// Every trust-stage kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BrowseTreeAndManifests,
        Self::ComputeSafeMetadata,
        Self::ReviewDeferredRepoActions,
        Self::RunRepoOwnedActionAfterApproval,
        Self::HydrateNetworkContentAfterApproval,
        Self::StageUnclassified,
    ];

    /// The five canonical trust stages every claimed M5 acquisition path resolves at.
    pub const CANONICAL_KINDS: [Self; 5] = [
        Self::BrowseTreeAndManifests,
        Self::ComputeSafeMetadata,
        Self::ReviewDeferredRepoActions,
        Self::RunRepoOwnedActionAfterApproval,
        Self::HydrateNetworkContentAfterApproval,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowseTreeAndManifests => "browse_tree_and_manifests",
            Self::ComputeSafeMetadata => "compute_safe_metadata",
            Self::ReviewDeferredRepoActions => "review_deferred_repo_actions",
            Self::RunRepoOwnedActionAfterApproval => "run_repo_owned_action_after_approval",
            Self::HydrateNetworkContentAfterApproval => "hydrate_network_content_after_approval",
            Self::StageUnclassified => "stage_unclassified",
        }
    }

    /// Whether the kind is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::StageUnclassified)
    }

    /// The canonical trust mode for this kind.
    pub const fn canonical_trust_mode(self) -> &'static str {
        match self {
            Self::BrowseTreeAndManifests => "browse_tree_and_manifests_stage",
            Self::ComputeSafeMetadata => "compute_safe_metadata_stage",
            Self::ReviewDeferredRepoActions => "review_deferred_repo_actions_stage",
            Self::RunRepoOwnedActionAfterApproval => "run_repo_owned_action_after_approval_stage",
            Self::HydrateNetworkContentAfterApproval => {
                "hydrate_network_content_after_approval_stage"
            }
            Self::StageUnclassified => "",
        }
    }

    /// Whether this trust stage widens trust or runs repo-owned code / network hydration and so must record an
    /// explicit approval before it may run, never widening trust implicitly during acquisition.
    pub const fn widens_trust_or_runs_code(self) -> bool {
        matches!(
            self,
            Self::RunRepoOwnedActionAfterApproval | Self::HydrateNetworkContentAfterApproval
        )
    }
}

/// Controlled post-open-queue class a queue entry must resolve its item from, so a queue item shares one
/// registry rather than a hand-copied per-entry item. Minted by this lane, tracking the runs-repo-owned-code /
/// hydrates-network-backed-content / mutates-reviewed-checkout / inert-recommendation consequence classes the
/// implementation requirement differentiates by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PostOpenQueueClass {
    /// The item runs repo-owned code (hook, task, extension activation, or generator install).
    RunsRepoOwnedCode,
    /// The item hydrates network-backed content (submodule init, LFS hydrate, or package restore).
    HydratesNetworkBackedContent,
    /// The item mutates the reviewed checkout beyond the acquisition plan (index warm-up or docs import).
    MutatesReviewedCheckout,
    /// The item is an inert recommendation (bundle recommendation or trust-prompt scheduling; presents only).
    InertRecommendation,
    /// The post-open-queue class is unclassified, which is disallowed.
    ClassUnclassified,
}

impl M5PostOpenQueueClass {
    /// Every post-open-queue class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RunsRepoOwnedCode,
        Self::HydratesNetworkBackedContent,
        Self::MutatesReviewedCheckout,
        Self::InertRecommendation,
        Self::ClassUnclassified,
    ];

    /// The four canonical classes every queue item must stay distinct across.
    pub const CANONICAL_CLASSES: [Self; 4] = [
        Self::RunsRepoOwnedCode,
        Self::HydratesNetworkBackedContent,
        Self::MutatesReviewedCheckout,
        Self::InertRecommendation,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunsRepoOwnedCode => "runs_repo_owned_code",
            Self::HydratesNetworkBackedContent => "hydrates_network_backed_content",
            Self::MutatesReviewedCheckout => "mutates_reviewed_checkout",
            Self::InertRecommendation => "inert_recommendation",
            Self::ClassUnclassified => "class_unclassified",
        }
    }

    /// Whether the post-open-queue class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ClassUnclassified)
    }

    /// Whether this class is a protected item that widens trust, runs code, hydrates network-backed content, or
    /// mutates the reviewed checkout, and so must be gated behind an explicit approval or policy. An inert
    /// recommendation is not protected.
    pub const fn is_protected(self) -> bool {
        matches!(
            self,
            Self::RunsRepoOwnedCode
                | Self::HydratesNetworkBackedContent
                | Self::MutatesReviewedCheckout
        )
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a staged-trust or
/// post-open-queue token's meaning stays stable whether it appears in the shell, entry, diagnostics, admin, or a
/// support / export form. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StagingSurfaceContext {
    /// The shell surface.
    ShellSurface,
    /// The project-entry surface.
    EntrySurface,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5StagingSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShellSurface,
        Self::EntrySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ShellSurface,
        Self::EntrySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellSurface => "shell_surface",
            Self::EntrySurface => "entry_surface",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::AdminSurface => "admin_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a staged-trust or post-open-queue entry must be able to show, so no trust stage,
/// queue item, browse-safety fact, approval gate, consequence, or registry fact is left implicit behind a
/// hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StagingAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The trust-stage kind the entry resolves (staged-trust entry).
    TrustStageKind,
    /// The browse-scope, computed-metadata, and deferred-action fields the entry publishes (staged-trust entry).
    StagedTrustFields,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The browse-safety and explicit-approval facts the entry publishes (staged-trust entry).
    BrowseSafetyAndApproval,
    /// The post-open-queue fields (queue class, execution site, trust / network consequence, approval
    /// requirement) the entry publishes (post-open-queue entry).
    PostOpenQueueFields,
    /// The attribution reference the entry publishes (post-open-queue entry).
    QueueAttributionHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved staged trust or post-open queue (both entries).
    PlainLanguageMeaning,
}

impl M5StagingAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::TrustStageKind,
        Self::StagedTrustFields,
        Self::ResolutionFormCoverage,
        Self::BrowseSafetyAndApproval,
        Self::PostOpenQueueFields,
        Self::QueueAttributionHint,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::TrustStageKind => "trust_stage_kind",
            Self::StagedTrustFields => "staged_trust_fields",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::BrowseSafetyAndApproval => "browse_safety_and_approval",
            Self::PostOpenQueueFields => "post_open_queue_fields",
            Self::QueueAttributionHint => "queue_attribution_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// staged trust, a post-open queue, or a degraded staged-trust / post-open-queue entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StagingNextAction {
    /// Expand the resolved stage's or queue's plain-language meaning.
    ExpandStagingMeaning,
    /// Inspect the trust stage or queue class the entry resolves.
    InspectStageOrQueue,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5StagingNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandStagingMeaning,
        Self::InspectStageOrQueue,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandStagingMeaning => "expand_staging_meaning",
            Self::InspectStageOrQueue => "inspect_stage_or_queue",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StagingExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The repository-bootstrap families covered.
    RepositoryBootstrapFamilies,
    /// The trust-stage kinds carried.
    TrustStageKinds,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The post-open-queue classes carried.
    PostOpenQueueClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The trust modes carried.
    TrustModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5StagingExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::RepositoryBootstrapFamilies,
        Self::TrustStageKinds,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::PostOpenQueueClasses,
        Self::SurfaceContext,
        Self::TrustModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::RepositoryBootstrapFamilies,
        Self::TrustStageKinds,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::RepositoryBootstrapFamilies => "repository_bootstrap_families",
            Self::TrustStageKinds => "trust_stage_kinds",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::PostOpenQueueClasses => "post_open_queue_classes",
            Self::SurfaceContext => "surface_context",
            Self::TrustModes => "trust_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a staged-trust entry degraded below a clean, registry-bound state. The degrade-first ladder returns
/// one of these instead of ever letting a hand-copied, implicitly-executing, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StagedTrustEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the stage means.
    StageTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The trust stage is unclassified (not in the resolved taxonomy).
    TrustStageUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    StagingNotBoundToRegistry,
    /// The resolved staged-trust object is incomplete: the browse-scope reference, computed-metadata reference,
    /// deferred repo-owned action set, trust-prompt policy, explicit-approval reference, or staged-trust
    /// provenance is unstated.
    StagedTrustObjectIncomplete,
    /// The stage would run a repo-owned action or widen trust before browse-safe metadata is computed and an
    /// explicit approval is recorded.
    StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A trust-widening stage executed a repo-owned action during acquisition without an explicit approval.
    RepoOwnedActionExecutedDuringAcquisition,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5StagedTrustEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::StageTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::TrustStageUnclassified,
        Self::StagingNotBoundToRegistry,
        Self::StagedTrustObjectIncomplete,
        Self::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly,
        Self::ResolutionFormCoverageIncomplete,
        Self::RepoOwnedActionExecutedDuringAcquisition,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StageTokenUnstated => "stage_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::TrustStageUnclassified => "trust_stage_unclassified",
            Self::StagingNotBoundToRegistry => "staging_not_bound_to_registry",
            Self::StagedTrustObjectIncomplete => "staged_trust_object_incomplete",
            Self::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly => {
                "staged_trust_runs_repo_owned_action_implicitly_or_widens_trust_early"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RepoOwnedActionExecutedDuringAcquisition => {
                "repo_owned_action_executed_during_acquisition"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5StagingNextAction {
        match self {
            Self::StageTokenUnstated | Self::StagingNotBoundToRegistry => {
                M5StagingNextAction::TraceCanonicalRegistry
            }
            Self::TrustStageUnclassified
            | Self::StagedTrustObjectIncomplete
            | Self::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly => {
                M5StagingNextAction::InspectStageOrQueue
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5StagingNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RepoOwnedActionExecutedDuringAcquisition
            | Self::ProofStale => M5StagingNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            Self::StageTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::TrustStageUnclassified | Self::StagedTrustObjectIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::StagedTrustRuleUnstated
            }
            Self::StagingNotBoundToRegistry => {
                M5RepositoryBootstrapDowngradeTrigger::CheckoutPlanBoundaryDriftedBySurface
            }
            Self::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly
            | Self::RepoOwnedActionExecutedDuringAcquisition => {
                M5RepositoryBootstrapDowngradeTrigger::RanRepoOwnedActionsImplicitlyDuringAcquisition
            }
            Self::ProofStale => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a post-open-queue entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PostOpenQueueEntryDegradeReason {
    /// The canonical registry token name is unstated.
    QueueTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The post-open-queue class is unclassified (not in the resolved taxonomy).
    PostOpenQueueClassUnclassified,
    /// A protected queue item would auto-execute during acquisition, run ungated without explicit approval or
    /// policy, or hide what it would run, where it would run, and its trust / network consequence, or it dropped
    /// one of the required queue-item fields (queue item kind, execution site, trust consequence, network
    /// consequence, approval requirement, attribution).
    PostOpenQueueItemExecutesImplicitlyOrHidesConsequence,
    /// The canonical / accessible / audit resolution-form coverage of the queue item is incomplete.
    QueueFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PostOpenQueueEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::QueueTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::PostOpenQueueClassUnclassified,
        Self::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence,
        Self::QueueFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueueTokenUnstated => "queue_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::PostOpenQueueClassUnclassified => "post_open_queue_class_unclassified",
            Self::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence => {
                "post_open_queue_item_executes_implicitly_or_hides_consequence"
            }
            Self::QueueFormCoverageIncomplete => "queue_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5StagingNextAction {
        match self {
            Self::QueueTokenUnstated => M5StagingNextAction::TraceCanonicalRegistry,
            Self::PostOpenQueueClassUnclassified
            | Self::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence => {
                M5StagingNextAction::InspectStageOrQueue
            }
            Self::QueueFormCoverageIncomplete => {
                M5StagingNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5StagingNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            Self::QueueTokenUnstated => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::PostOpenQueueClassUnclassified => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence => {
                M5RepositoryBootstrapDowngradeTrigger::RanRepoOwnedActionsImplicitlyDuringAcquisition
            }
            Self::QueueFormCoverageIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::CheckoutPlanBoundaryDriftedBySurface
            }
            Self::ProofStale => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_staged_trust_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StagedTrustEntryResolutionInput {
    /// Stable identity of the staged-trust-registry entry.
    pub entry_id: String,
    /// The stable acquisition-path ID this stage binds to (e.g. `entry.acme.clone-remote`); empty means
    /// unstated.
    pub acquisition_path_id: String,
    /// The canonical registry token name (e.g. `staged.trust.browse_tree_and_manifests`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5RepositoryBootstrapRole,
    /// The trust stage this entry resolves.
    pub trust_stage_kind: M5TrustStageKind,
    /// The render / surface context.
    pub surface_context: M5StagingSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5StagingResolutionForm>,
    /// The published browse-scope reference (what tree / manifests / docs are browsable); empty means unstated.
    pub browse_scope_ref: String,
    /// The published computed-metadata reference (the safe metadata computed); empty means unstated.
    pub computed_metadata_ref: String,
    /// The published deferred repo-owned action set (hooks / tasks / extensions / restore / submodule / LFS /
    /// generator); empty means unstated.
    pub deferred_repo_action_set: String,
    /// The published trust-prompt policy; empty means unstated.
    pub trust_prompt_policy: String,
    /// The published explicit-approval reference (a handle to the recorded approval / policy); empty means
    /// unstated.
    pub explicit_approval_reference: String,
    /// The published staged-trust provenance; empty means unstated.
    pub staged_trust_provenance: String,
    /// True when the behavior traces to the staged-trust registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the stage keeps repository open browse-safe before it widens trust — the tree, manifests, and
    /// docs are browsable and safe metadata is computed before any repo-owned action can run (a hard invariant
    /// when `false`).
    pub browse_safe_before_widening: bool,
    /// True when this stage widens trust or runs repo-owned code / network hydration.
    pub widens_trust_or_runs_code: bool,
    /// True when an explicit approval is recorded before this stage widens trust or runs any repo-owned action.
    pub explicit_approval_recorded: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe staged-trust-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedStagedTrustEntry {
    /// Stable identity of the staged-trust-registry entry.
    pub entry_id: String,
    /// The stable acquisition-path ID this stage binds to.
    pub acquisition_path_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must stage trust and disclose provenance before bootstrap.
    pub semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: bool,
    /// The trust-stage-kind token named by the entry.
    pub trust_stage_kind: String,
    /// Whether the trust-stage kind is classified into the resolved taxonomy.
    pub trust_stage_kind_is_classified: bool,
    /// The canonical trust mode for the entry's kind.
    pub canonical_trust_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published browse-scope reference.
    pub browse_scope_ref: String,
    /// The published computed-metadata reference.
    pub computed_metadata_ref: String,
    /// The published deferred repo-owned action set.
    pub deferred_repo_action_set: String,
    /// The published trust-prompt policy.
    pub trust_prompt_policy: String,
    /// The published explicit-approval reference.
    pub explicit_approval_reference: String,
    /// The published staged-trust provenance.
    pub staged_trust_provenance: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved staged-trust object publishes every required field.
    pub staged_trust_object_complete: bool,
    /// Whether the entry traces to the staged-trust registry.
    pub bound_to_registry: bool,
    /// Whether repository open stays browse-safe before the stage widens trust.
    pub browse_safe_before_widening: bool,
    /// Whether this stage widens trust or runs repo-owned code.
    pub widens_trust_or_runs_code: bool,
    /// Whether an explicit approval is recorded before any trust-widening.
    pub explicit_approval_recorded: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5StagedTrustEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5StagingNextAction,
    /// Whether the stage resolves to one stable object across every claimed acquisition path (clean entry
    /// naming every fact).
    pub staging_resolves_across_entry_flows: bool,
}

impl M5ResolvedStagedTrustEntry {
    /// Whether this staged-trust entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_post_open_queue_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PostOpenQueueEntryResolutionInput {
    /// Stable identity of the post-open-queue entry.
    pub entry_id: String,
    /// The stable source-ref this queue item binds to; empty means unstated.
    pub source_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5RepositoryBootstrapRole,
    /// The post-open-queue class this entry must resolve its item from.
    pub queue_class: M5PostOpenQueueClass,
    /// The render / surface context.
    pub surface_context: M5StagingSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5StagingResolutionForm>,
    /// The published queue-item kind (what would run); empty means missing.
    pub queue_item_kind: String,
    /// The published execution site (where it would run); empty means missing.
    pub execution_site: String,
    /// The published trust consequence; empty means missing.
    pub trust_consequence: String,
    /// The published network consequence; empty means missing.
    pub network_consequence: String,
    /// The published approval requirement; empty means missing.
    pub approval_requirement: String,
    /// The published attribution reference (who / what attributes this work object); empty means missing.
    pub attribution_ref: String,
    /// True when the queue item identifies exactly what would run, where, and its trust / network consequence.
    pub identifies_run_site_and_consequence: bool,
    /// True when the queue item is truthfully typed (never claims a safe class over an executing item).
    pub item_is_truthfully_typed: bool,
    /// True when the queue item is a protected item (widens trust, runs code, hydrates network-backed content,
    /// or mutates the reviewed checkout).
    pub is_protected_item: bool,
    /// True when a protected item is gated behind an explicit approval or policy.
    pub explicit_approval_or_policy_gated: bool,
    /// True when the item schedules deferred follow-up bootstrap work.
    pub schedules_deferred_followup: bool,
    /// True when scheduled follow-up bootstrap work is disclosed rather than left implicit.
    pub followup_is_disclosed: bool,
    /// True when the item would auto-execute during acquisition (a hard invariant when `true`).
    pub auto_executes_during_acquisition: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe post-open-queue projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPostOpenQueueEntry {
    /// Stable identity of the post-open-queue entry.
    pub entry_id: String,
    /// The stable source-ref this queue item binds to.
    pub source_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must stage trust and disclose provenance before bootstrap.
    pub semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: bool,
    /// The post-open-queue-class token named by the entry.
    pub queue_class: String,
    /// Whether the post-open-queue class is classified into the resolved taxonomy.
    pub queue_class_is_classified: bool,
    /// Whether the post-open-queue class is a protected item.
    pub queue_class_is_protected: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published queue-item kind.
    pub queue_item_kind: String,
    /// The published execution site.
    pub execution_site: String,
    /// The published trust consequence.
    pub trust_consequence: String,
    /// The published network consequence.
    pub network_consequence: String,
    /// The published approval requirement.
    pub approval_requirement: String,
    /// The published attribution reference.
    pub attribution_ref: String,
    /// Whether the queue item identifies what would run, where, and its consequence.
    pub identifies_run_site_and_consequence: bool,
    /// Whether the queue item is truthfully typed.
    pub item_is_truthfully_typed: bool,
    /// Whether the queue item is a protected item.
    pub is_protected_item: bool,
    /// Whether a protected item is gated behind an explicit approval or policy.
    pub explicit_approval_or_policy_gated: bool,
    /// Whether the item schedules deferred follow-up bootstrap work.
    pub schedules_deferred_followup: bool,
    /// Whether scheduled follow-up bootstrap work is disclosed.
    pub followup_is_disclosed: bool,
    /// Whether the item would auto-execute during acquisition.
    pub auto_executes_during_acquisition: bool,
    /// Whether the queue item holds for approval (protected items gated, disclosed follow-up, never
    /// auto-executing during acquisition).
    pub post_open_queue_item_holds_for_approval: bool,
    /// Whether the entry provides the complete post-open-queue object (queue item kind, execution site, trust /
    /// network consequence, approval requirement, attribution).
    pub provides_complete_post_open_queue: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5PostOpenQueueEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5StagingNextAction,
    /// Whether the queue item is safe on every claimed source (clean entry naming every fact).
    pub queue_safe_on_every_source: bool,
}

impl M5ResolvedPostOpenQueueEntry {
    /// Whether this post-open-queue entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5StagingResolutionError {
    /// The staged-trust-entry id was empty.
    EmptyStagedTrustEntryId,
    /// The post-open-queue-entry id was empty.
    EmptyPostOpenQueueEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5StagingResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStagedTrustEntryId => "empty_staged_trust_entry_id",
            Self::EmptyPostOpenQueueEntryId => "empty_post_open_queue_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5StagingResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 staged-trust / post-open-queue registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5StagingResolutionError {}

fn form_tokens(forms: &[M5StagingResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5StagingResolutionForm]) -> bool {
    let present: BTreeSet<M5StagingResolutionForm> = forms.iter().copied().collect();
    M5StagingResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved staged-trust object publishes every required field: trust mode (via a classified kind),
/// browse-scope reference, computed-metadata reference, deferred repo-owned action set, trust-prompt policy,
/// explicit-approval reference, and staged-trust provenance. An unclassified kind or any empty field never
/// resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn staged_trust_object_is_complete(
    kind: M5TrustStageKind,
    browse_scope_ref: &str,
    computed_metadata_ref: &str,
    deferred_repo_action_set: &str,
    trust_prompt_policy: &str,
    explicit_approval_reference: &str,
    staged_trust_provenance: &str,
) -> bool {
    kind.is_classified()
        && !browse_scope_ref.trim().is_empty()
        && !computed_metadata_ref.trim().is_empty()
        && !deferred_repo_action_set.trim().is_empty()
        && !trust_prompt_policy.trim().is_empty()
        && !explicit_approval_reference.trim().is_empty()
        && !staged_trust_provenance.trim().is_empty()
}

/// Whether the staged trust stays browse-safe: the kind must be classified, repository open must stay browse-safe
/// before the stage widens trust (the tree, manifests, and docs are browsable and safe metadata is computed
/// before any repo-owned action can run), and a stage that widens trust or runs repo-owned code must record an
/// explicit approval before it may run (never widening trust implicitly during acquisition). An unclassified
/// kind, an unsafe browse before widening, or a trust-widening stage with no recorded approval never matches.
pub fn staged_trust_stays_browse_safe(
    kind: M5TrustStageKind,
    browse_safe_before_widening: bool,
    widens_trust_or_runs_code: bool,
    explicit_approval_recorded: bool,
) -> bool {
    kind.is_classified()
        && browse_safe_before_widening
        && (!widens_trust_or_runs_code || explicit_approval_recorded)
}

/// Whether a post-open-queue item holds for approval: the class must be classified, the item must be truthfully
/// typed, it must identify what would run, where, and its trust / network consequence, it must never auto-execute
/// during acquisition, any protected item must be gated behind an explicit approval or policy, and any scheduled
/// deferred follow-up must be disclosed.
#[allow(clippy::too_many_arguments)]
pub fn post_open_queue_item_holds_for_approval(
    class: M5PostOpenQueueClass,
    item_is_truthfully_typed: bool,
    identifies_run_site_and_consequence: bool,
    is_protected_item: bool,
    explicit_approval_or_policy_gated: bool,
    schedules_deferred_followup: bool,
    followup_is_disclosed: bool,
    auto_executes_during_acquisition: bool,
) -> bool {
    class.is_classified()
        && item_is_truthfully_typed
        && identifies_run_site_and_consequence
        && !auto_executes_during_acquisition
        && (!is_protected_item || explicit_approval_or_policy_gated)
        && (!schedules_deferred_followup || followup_is_disclosed)
}

/// Resolves a staged-trust-registry entry so it stays bound to the staged-trust registry: the entry names its
/// canonical token, semantic role, and trust stage, covers all three resolution forms, publishes a complete
/// staged-trust object (browse-scope reference, computed-metadata reference, deferred repo-owned action set,
/// trust-prompt policy, explicit-approval reference, staged-trust provenance), keeps repository open browse-safe
/// before it widens trust, and records an explicit approval before any trust-widening stage.
pub fn resolve_staged_trust_entry(
    input: M5StagedTrustEntryResolutionInput,
) -> Result<M5ResolvedStagedTrustEntry, M5StagingResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5StagingResolutionError::EmptyStagedTrustEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.acquisition_path_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.browse_scope_ref)
        || string_is_forbidden(&input.computed_metadata_ref)
        || string_is_forbidden(&input.deferred_repo_action_set)
        || string_is_forbidden(&input.trust_prompt_policy)
        || string_is_forbidden(&input.explicit_approval_reference)
        || string_is_forbidden(&input.staged_trust_provenance)
    {
        return Err(M5StagingResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = staged_trust_object_is_complete(
        input.trust_stage_kind,
        &input.browse_scope_ref,
        &input.computed_metadata_ref,
        &input.deferred_repo_action_set,
        &input.trust_prompt_policy,
        &input.explicit_approval_reference,
        &input.staged_trust_provenance,
    );
    let browse_safe_ok = staged_trust_stays_browse_safe(
        input.trust_stage_kind,
        input.browse_safe_before_widening,
        input.widens_trust_or_runs_code,
        input.explicit_approval_recorded,
    );
    let repo_action_executed_early =
        input.widens_trust_or_runs_code && !input.explicit_approval_recorded;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5StagedTrustEntryDegradeReason::StageTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5StagedTrustEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.trust_stage_kind.is_classified() {
        Some(M5StagedTrustEntryDegradeReason::TrustStageUnclassified)
    } else if !input.bound_to_registry {
        Some(M5StagedTrustEntryDegradeReason::StagingNotBoundToRegistry)
    } else if !object_complete {
        Some(M5StagedTrustEntryDegradeReason::StagedTrustObjectIncomplete)
    } else if !browse_safe_ok {
        Some(M5StagedTrustEntryDegradeReason::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly)
    } else if !all_forms {
        Some(M5StagedTrustEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if repo_action_executed_early {
        Some(M5StagedTrustEntryDegradeReason::RepoOwnedActionExecutedDuringAcquisition)
    } else if !input.proof_fresh {
        Some(M5StagedTrustEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5StagingNextAction::ExpandStagingMeaning,
    };

    Ok(M5ResolvedStagedTrustEntry {
        entry_id: input.entry_id,
        acquisition_path_id: input.acquisition_path_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: input
            .semantic_role
            .must_stage_trust_and_disclose_provenance_before_bootstrap(),
        trust_stage_kind: input.trust_stage_kind.as_str().to_owned(),
        trust_stage_kind_is_classified: input.trust_stage_kind.is_classified(),
        canonical_trust_mode: input.trust_stage_kind.canonical_trust_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        browse_scope_ref: input.browse_scope_ref,
        computed_metadata_ref: input.computed_metadata_ref,
        deferred_repo_action_set: input.deferred_repo_action_set,
        trust_prompt_policy: input.trust_prompt_policy,
        explicit_approval_reference: input.explicit_approval_reference,
        staged_trust_provenance: input.staged_trust_provenance,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        staged_trust_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        browse_safe_before_widening: input.browse_safe_before_widening,
        widens_trust_or_runs_code: input.widens_trust_or_runs_code,
        explicit_approval_recorded: input.explicit_approval_recorded,
        degrade_reason,
        next_action,
        staging_resolves_across_entry_flows: degrade_reason.is_none(),
    })
}

/// Resolves a post-open-queue entry so its item stays safe: the entry names its canonical token, semantic role,
/// and post-open-queue class, covers all three resolution forms, provides the complete queue-item-kind /
/// execution-site / trust-consequence / network-consequence / approval-requirement / attribution queue object,
/// and degrades honestly when a protected item would auto-execute during acquisition, run ungated without an
/// explicit approval or policy, or hide what it would run and where.
pub fn resolve_post_open_queue_entry(
    input: M5PostOpenQueueEntryResolutionInput,
) -> Result<M5ResolvedPostOpenQueueEntry, M5StagingResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5StagingResolutionError::EmptyPostOpenQueueEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.source_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.queue_item_kind)
        || string_is_forbidden(&input.execution_site)
        || string_is_forbidden(&input.trust_consequence)
        || string_is_forbidden(&input.network_consequence)
        || string_is_forbidden(&input.approval_requirement)
        || string_is_forbidden(&input.attribution_ref)
    {
        return Err(M5StagingResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let holds_for_approval = post_open_queue_item_holds_for_approval(
        input.queue_class,
        input.item_is_truthfully_typed,
        input.identifies_run_site_and_consequence,
        input.is_protected_item,
        input.explicit_approval_or_policy_gated,
        input.schedules_deferred_followup,
        input.followup_is_disclosed,
        input.auto_executes_during_acquisition,
    );
    let provides_queue = input.queue_class.is_classified()
        && !input.queue_item_kind.trim().is_empty()
        && !input.execution_site.trim().is_empty()
        && !input.trust_consequence.trim().is_empty()
        && !input.network_consequence.trim().is_empty()
        && !input.approval_requirement.trim().is_empty()
        && !input.attribution_ref.trim().is_empty()
        && holds_for_approval;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5PostOpenQueueEntryDegradeReason::QueueTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PostOpenQueueEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.queue_class.is_classified() {
        Some(M5PostOpenQueueEntryDegradeReason::PostOpenQueueClassUnclassified)
    } else if !provides_queue {
        Some(M5PostOpenQueueEntryDegradeReason::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence)
    } else if !all_forms {
        Some(M5PostOpenQueueEntryDegradeReason::QueueFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5PostOpenQueueEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5StagingNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedPostOpenQueueEntry {
        entry_id: input.entry_id,
        source_ref: input.source_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: input
            .semantic_role
            .must_stage_trust_and_disclose_provenance_before_bootstrap(),
        queue_class: input.queue_class.as_str().to_owned(),
        queue_class_is_classified: input.queue_class.is_classified(),
        queue_class_is_protected: input.queue_class.is_protected(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        queue_item_kind: input.queue_item_kind,
        execution_site: input.execution_site,
        trust_consequence: input.trust_consequence,
        network_consequence: input.network_consequence,
        approval_requirement: input.approval_requirement,
        attribution_ref: input.attribution_ref,
        identifies_run_site_and_consequence: input.identifies_run_site_and_consequence,
        item_is_truthfully_typed: input.item_is_truthfully_typed,
        is_protected_item: input.is_protected_item,
        explicit_approval_or_policy_gated: input.explicit_approval_or_policy_gated,
        schedules_deferred_followup: input.schedules_deferred_followup,
        followup_is_disclosed: input.followup_is_disclosed,
        auto_executes_during_acquisition: input.auto_executes_during_acquisition,
        post_open_queue_item_holds_for_approval: holds_for_approval,
        provides_complete_post_open_queue: provides_queue,
        degrade_reason,
        next_action,
        queue_safe_on_every_source: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved staged-trust and post-open-queue entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedTrustPostOpenQueueRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5StagedTrustPostOpenQueueRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5RepositoryBootstrapQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Acquisition contexts this row keeps the same truth across.
    pub deployment_lines: Vec<M5RepositoryBootstrapDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5RepositoryBootstrapRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5RepositoryBootstrapAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5StagingAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5StagingExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    /// Resolved staged-trust-registry examples.
    pub staged_trust_entries: Vec<M5ResolvedStagedTrustEntry>,
    /// Resolved post-open-queue examples.
    pub post_open_queue_entries: Vec<M5ResolvedPostOpenQueueEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the checkout-plan and bootstrap-evidence
    /// domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never runs repo-owned actions implicitly during acquisition. MUST be `false`.
    pub runs_repo_owned_actions_implicitly_during_acquisition: bool,
    /// Hard invariant: this row never auto-executes a post-open bootstrap queue item without explicit approval.
    /// MUST be `false`.
    pub auto_executes_post_open_bootstrap_queue_without_explicit_approval: bool,
    /// Hard invariant: this row never hides what a queue item would run or its trust / network consequence. MUST
    /// be `false`.
    pub hides_what_a_queue_item_would_run_or_its_trust_or_network_consequence: bool,
    /// Hard invariant: this row never widens trust before browse-safe metadata is computed. MUST be `false`.
    pub widens_trust_before_browse_safe_metadata_is_computed: bool,
}

impl M5StagedTrustPostOpenQueueRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5StagingAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5StagingAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5StagingExportField> = self.export_fields.iter().copied().collect();
        M5StagingExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.runs_repo_owned_actions_implicitly_during_acquisition
            && !self.auto_executes_post_open_bootstrap_queue_without_explicit_approval
            && !self.hides_what_a_queue_item_would_run_or_its_trust_or_network_consequence
            && !self.widens_trust_before_browse_safe_metadata_is_computed
    }

    /// True when a clean staged-trust entry preserves registry-bound truth: it traces to the registry, keeps a
    /// classified trust stage, publishes a complete staged-trust object, stays browse-safe before widening,
    /// covers all three resolution forms, and records an explicit approval before any trust-widening stage.
    fn staging_is_honest(ex: &M5ResolvedStagedTrustEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.trust_stage_kind_is_classified
                && ex.staged_trust_object_complete
                && ex.browse_safe_before_widening
                && ex.covers_all_resolution_forms
                && (!ex.widens_trust_or_runs_code || ex.explicit_approval_recorded))
    }

    /// True when a clean post-open-queue entry preserves a safe item: it keeps a classified class, provides the
    /// complete queue object, holds for approval, and covers all three resolution forms.
    fn queue_is_honest(ex: &M5ResolvedPostOpenQueueEntry) -> bool {
        !ex.is_clean()
            || (ex.queue_class_is_classified
                && ex.provides_complete_post_open_queue
                && ex.post_open_queue_item_holds_for_approval
                && !ex.auto_executes_during_acquisition
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.staged_trust_entries
            .iter()
            .all(Self::staging_is_honest)
            && self
                .post_open_queue_entries
                .iter()
                .all(Self::queue_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedTrustPostOpenQueueRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Trust-stage tokens (minted by this lane).
    pub trust_stage_kinds: Vec<String>,
    /// Post-open-queue-class tokens (minted by this lane).
    pub post_open_queue_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Staged-trust-entry degrade-reason tokens.
    pub staged_trust_degrade_reasons: Vec<String>,
    /// Post-open-queue-entry degrade-reason tokens.
    pub post_open_queue_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5StagedTrustPostOpenQueueRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5RepositoryBootstrapRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5StagingResolutionForm::ALL, |v| v.as_str()),
            trust_stage_kinds: tokens(&M5TrustStageKind::ALL, |v| v.as_str()),
            post_open_queue_classes: tokens(&M5PostOpenQueueClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5StagingSurfaceContext::ALL, |v| v.as_str()),
            staged_trust_degrade_reasons: tokens(&M5StagedTrustEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            post_open_queue_degrade_reasons: tokens(&M5PostOpenQueueEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5StagingAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5StagingNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5StagingExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5RepositoryBootstrapConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedTrustPostOpenQueueRegistriesGovernanceReview {
    /// The staged-trust registry names a canonical token, semantic role, and trust stage for every entry.
    pub staged_trust_registry_names_token_role_and_stage: bool,
    /// Every claimed acquisition path resolves to one stable staged-trust object from the shared registry, not
    /// per-entry reconstruction.
    pub entry_flow_resolves_to_stable_staging_from_shared_registry: bool,
    /// The browse scope, computed metadata, deferred repo-owned action set, trust-prompt policy, explicit
    /// approval, and provenance are published for every resolved stage.
    pub browse_scope_metadata_deferred_actions_and_provenance_published: bool,
    /// The staged trust stays browse-safe; no repo-owned action runs implicitly during acquisition.
    pub staged_trust_stays_browse_safe_no_implicit_repo_action: bool,
    /// The post-open queue identifies exactly what would run, where, and what trust / network consequence it
    /// carries.
    pub post_open_queue_identifies_run_site_and_consequence: bool,
    /// A protected queue item is gated behind an explicit approval or policy and never auto-executes.
    pub protected_queue_item_gated_behind_explicit_approval_or_policy: bool,
    /// Every staged-trust and post-open-queue entry covers the canonical / accessible / audit resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Staged-trust and post-open-queue behavior stay bound to the shared registries rather than hand-copied per
    /// acquisition path.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Acquisition, git, trust, and diagnostics read a single staging / queue source.
    pub acquisition_git_trust_diagnostics_read_single_source: bool,
    /// An implicit repo-owned action, an auto-executing queue item, or a hidden consequence is caught by
    /// fixtures before release evidence turns green.
    pub staging_or_queue_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedTrustPostOpenQueueRegistriesConsumerProjection {
    /// Acquisition engine and git service consume the shared staged-trust registry.
    pub acquisition_and_git_consume_shared_registries: bool,
    /// Trust service and diagnostics consume the shared post-open-queue registry.
    pub trust_and_diagnostics_consume_shared_registries: bool,
    /// CLI export and support export consume the shared registries.
    pub cli_and_support_export_consume_shared_registries: bool,
    /// Docs, help, and workspace services consume the shared registries.
    pub docs_help_and_workspace_consume_shared_registries: bool,
    /// Behavior traces back to the canonical checkout-plan and bootstrap-evidence domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical staged-trust / post-open-queue registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedTrustPostOpenQueueRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedTrustPostOpenQueueRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting repository-bootstrap audit for the lane.
    pub repository_bootstrap_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5StagedTrustPostOpenQueueRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StagedTrustPostOpenQueueRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5StagedTrustPostOpenQueueRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5StagedTrustPostOpenQueueRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5StagedTrustPostOpenQueueRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5StagedTrustPostOpenQueueRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5StagedTrustPostOpenQueueRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5StagedTrustPostOpenQueueRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 staged-trust and post-open-queue registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StagedTrustPostOpenQueueRegistriesPacket {
    /// Record kind; must equal [`M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5StagedTrustPostOpenQueueRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5StagedTrustPostOpenQueueRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5StagedTrustPostOpenQueueRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5StagedTrustPostOpenQueueRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5StagedTrustPostOpenQueueRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5StagedTrustPostOpenQueueRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5StagedTrustPostOpenQueueRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5StagedTrustPostOpenQueueRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5StagedTrustPostOpenQueueRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_RECORD_KIND {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 staged-trust / post-open-queue registries packet serializes"),
        ) {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 staged-trust / post-open-queue registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,staged_trust_entries,post_open_queue_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .staged_trust_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.post_open_queue_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.staged_trust_entries.len(),
                row.post_open_queue_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Staged-Trust and Post-Open Bootstrap-Queue Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Trust stages: {}\n",
            self.vocabulary_set.trust_stage_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Staged-trust entries: {} / post-open-queue entries: {}\n",
                row.staged_trust_entries.len(),
                row.post_open_queue_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-item post-open-queue reference table generated from the registry, so docs and admin
    /// runbooks render the same queue-item / execution-site / trust-consequence / network-consequence /
    /// approval-requirement truth the resolvers produced rather than a hand-copied queue table. Only clean,
    /// registry-bound post-open-queue entries are listed.
    pub fn render_post_open_queue_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| source_ref | queue_class | queue_item_kind | execution_site | trust_consequence | network_consequence | approval_requirement |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.post_open_queue_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.source_ref,
                    ex.queue_class,
                    ex.queue_item_kind,
                    ex.execution_site,
                    ex.trust_consequence,
                    ex.network_consequence,
                    ex.approval_requirement
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5StagedTrustPostOpenQueueRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5StagedTrustPostOpenQueueRegistriesViolation>),
}

impl fmt::Display for M5StagedTrustPostOpenQueueRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 staged-trust / post-open-queue registries export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 staged-trust / post-open-queue registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5StagedTrustPostOpenQueueRegistriesArtifactError {}

/// Validation failures emitted by [`M5StagedTrustPostOpenQueueRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5StagedTrustPostOpenQueueRegistriesViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the checkout-plan and bootstrap-evidence domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, implicitly-executing, field-incomplete,
    /// form-incomplete, or a post-open-queue entry missing the complete queue object).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Staged-trust-resolution is not proven: clean staging entries do not cover the canonical trust stages or
    /// the first shell / entry / diagnostics / admin / support surfaces, no object-incomplete example degrades,
    /// or a clean staging entry published an incomplete object.
    StagedTrustResolutionNotProven,
    /// Browse-safe-staging is not proven: no implicit-repo-action example and no unbound example degrade, no
    /// clean browse-safe staging entry is present, or a clean staging entry widened trust early or is unbound.
    BrowseSafeStagingNotProven,
    /// Post-open-queue-gating is not proven: clean queue entries do not cover the canonical runs-code /
    /// hydrates-network / mutates-checkout / inert-recommendation classes with full resolution-form coverage
    /// while providing the complete queue object, no implicit-execution or form-incomplete example degrades, or
    /// a clean queue entry auto-executes or is missing the complete queue object.
    PostOpenQueueGatingNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5StagedTrustPostOpenQueueRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::StagedTrustResolutionNotProven => "staged_trust_resolution_not_proven",
            Self::BrowseSafeStagingNotProven => "browse_safe_staging_not_proven",
            Self::PostOpenQueueGatingNotProven => "post_open_queue_gating_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_staged_trust_and_post_open_queue_registries_export() -> Result<
    M5StagedTrustPostOpenQueueRegistriesPacket,
    M5StagedTrustPostOpenQueueRegistriesArtifactError,
> {
    let packet: M5StagedTrustPostOpenQueueRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-staged-trust-and-post-open-queue-registries-proof/support_export.json"
        )
    ))
    .map_err(M5StagedTrustPostOpenQueueRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5StagedTrustPostOpenQueueRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5StagedTrustPostOpenQueueRegistriesPacket,
    violations: &mut Vec<M5StagedTrustPostOpenQueueRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_SCHEMA_REF,
        M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5StagedTrustPostOpenQueueRegistriesPacket,
    violations: &mut Vec<M5StagedTrustPostOpenQueueRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5StagedTrustPostOpenQueueRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF)
        {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.staged_trust_entries.is_empty() || row.post_open_queue_entries.is_empty() {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5StagedTrustPostOpenQueueRegistriesPacket,
    violations: &mut Vec<M5StagedTrustPostOpenQueueRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.staged_trust_registry_names_token_role_and_stage,
        review.entry_flow_resolves_to_stable_staging_from_shared_registry,
        review.browse_scope_metadata_deferred_actions_and_provenance_published,
        review.staged_trust_stays_browse_safe_no_implicit_repo_action,
        review.post_open_queue_identifies_run_site_and_consequence,
        review.protected_queue_item_gated_behind_explicit_approval_or_policy,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.acquisition_git_trust_diagnostics_read_single_source,
        review.staging_or_queue_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5StagedTrustPostOpenQueueRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5StagedTrustPostOpenQueueRegistriesPacket,
    violations: &mut Vec<M5StagedTrustPostOpenQueueRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.acquisition_and_git_consume_shared_registries,
        projection.trust_and_diagnostics_consume_shared_registries,
        projection.cli_and_support_export_consume_shared_registries,
        projection.docs_help_and_workspace_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations
                .push(M5StagedTrustPostOpenQueueRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5StagedTrustPostOpenQueueRegistriesPacket,
    violations: &mut Vec<M5StagedTrustPostOpenQueueRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5StagedTrustPostOpenQueueRegistriesPacket,
    violations: &mut Vec<M5StagedTrustPostOpenQueueRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.repository_bootstrap_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5StagedTrustPostOpenQueueRegistriesPacket,
    violations: &mut Vec<M5StagedTrustPostOpenQueueRegistriesViolation>,
) {
    let stagings = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.staged_trust_entries.iter())
    };
    let queues = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.post_open_queue_entries.iter())
    };

    // AC1: repository open remains useful before repo-owned actions run — every claimed acquisition path
    // resolves to one stable staged-trust object with browse-scope / metadata / deferred-action / provenance
    // fields. Clean staging entries cover the canonical trust stages and the first shell / entry / diagnostics /
    // admin / support surfaces, an object-incomplete example degrades, and no clean staging entry published an
    // incomplete object.
    let clean_kinds: BTreeSet<String> = stagings()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.trust_stage_kind.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = stagings()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let kinds_covered = M5TrustStageKind::CANONICAL_KINDS
        .iter()
        .all(|k| clean_kinds.contains(k.as_str()));
    let first_surfaces_covered = M5StagingSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = stagings().any(|ex| {
        ex.degrade_reason == Some(M5StagedTrustEntryDegradeReason::StagedTrustObjectIncomplete)
    });
    let no_clean_incomplete =
        !stagings().any(|ex| ex.is_clean() && !ex.staged_trust_object_complete);
    if !(kinds_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations
            .push(M5StagedTrustPostOpenQueueRegistriesViolation::StagedTrustResolutionNotProven);
    }

    // AC1/AC3: the staged trust stays browse-safe — no repo-owned action runs implicitly and trust is never
    // widened before browse-safe metadata is computed. An implicit-repo-action example degrades, an unbound
    // example degrades, at least one clean browse-safe staging entry is present, and no clean staging entry
    // widened trust early or is unbound.
    let implicit_action_degrades = stagings().any(|ex| {
        ex.degrade_reason
            == Some(
                M5StagedTrustEntryDegradeReason::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly,
            )
    });
    let unbound_degrades = stagings().any(|ex| {
        ex.degrade_reason == Some(M5StagedTrustEntryDegradeReason::StagingNotBoundToRegistry)
    });
    let browse_safe_clean_staging = stagings().any(|ex| {
        ex.is_clean()
            && ex.browse_safe_before_widening
            && (!ex.widens_trust_or_runs_code || ex.explicit_approval_recorded)
    });
    let no_clean_unbound = !stagings().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_widen_early = !stagings()
        .any(|ex| ex.is_clean() && ex.widens_trust_or_runs_code && !ex.explicit_approval_recorded);
    if !(implicit_action_degrades
        && unbound_degrades
        && browse_safe_clean_staging
        && no_clean_unbound
        && no_clean_widen_early)
    {
        violations.push(M5StagedTrustPostOpenQueueRegistriesViolation::BrowseSafeStagingNotProven);
    }

    // AC2/AC3: the suite fails when a protected post-open queue item auto-executes during acquisition. Clean
    // queue entries cover every canonical runs-code / hydrates-network / mutates-checkout / inert-recommendation
    // class with full resolution-form coverage while providing the complete queue object, an implicit-execution
    // example degrades, a form-incomplete example degrades, and no clean queue entry auto-executes or is missing
    // the complete queue object.
    let clean_queue_classes: BTreeSet<String> = queues()
        .filter(|ex| {
            ex.is_clean()
                && ex.queue_class_is_classified
                && ex.provides_complete_post_open_queue
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.queue_class.clone())
        .collect();
    let queue_classes_covered = M5PostOpenQueueClass::CANONICAL_CLASSES
        .iter()
        .all(|c| clean_queue_classes.contains(c.as_str()));
    let implicit_execution_degrades = queues().any(|ex| {
        ex.degrade_reason
            == Some(
                M5PostOpenQueueEntryDegradeReason::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence,
            )
    });
    let form_incomplete_degrades = queues().any(|ex| {
        ex.degrade_reason == Some(M5PostOpenQueueEntryDegradeReason::QueueFormCoverageIncomplete)
    });
    let no_clean_auto_executing =
        !queues().any(|ex| ex.is_clean() && ex.auto_executes_during_acquisition);
    let no_clean_missing_queue =
        !queues().any(|ex| ex.is_clean() && !ex.provides_complete_post_open_queue);
    if !(queue_classes_covered
        && implicit_execution_degrades
        && form_incomplete_degrades
        && no_clean_auto_executing
        && no_clean_missing_queue)
    {
        violations
            .push(M5StagedTrustPostOpenQueueRegistriesViolation::PostOpenQueueGatingNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The repository-bootstrap families this lane implements, for downstream reference. Staged trust and the
/// post-open bootstrap queue apply to every acquisition verb, so this lane covers all five families.
pub const IMPLEMENTED_FAMILIES: [M5RepositoryBootstrapFamily; 5] = [
    M5RepositoryBootstrapFamily::OpenLocal,
    M5RepositoryBootstrapFamily::CloneRemote,
    M5RepositoryBootstrapFamily::OpenArchive,
    M5RepositoryBootstrapFamily::ImportBundle,
    M5RepositoryBootstrapFamily::ResumeSnapshot,
];
