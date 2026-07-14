//! Implemented M5 source-locator and checkout-plan registries.
//!
//! The frozen [repository-bootstrap matrix][matrix] names Aureline's five project-entry acquisition families
//! and locks their controlled vocabulary. This module is the first implement lane for the concrete
//! source-resolution and checkout-planning flows: it turns the *source-locator* grammar (open-local /
//! open-archive) and the *checkout-plan* grammar (clone-remote) into registry resolvers that produce
//! export-safe, honest projections. Every claimed M5 entry flow then resolves to one stable source-locator
//! object — the source-locator kind, the literal target it preserves verbatim, the checkout root or archive
//! container it resolves, the staged-trust metadata, the disclosed bootstrap credential posture, the signer /
//! mirror provenance, and the mirror / air-gap hint kept distinct from authoritative provenance — and to one
//! checkout-plan object — the ref selection, full / partial / sparse mode, depth / filter, submodule mode, LFS
//! posture, destination path, and expected disk / network cost band — that the shell, entry, diagnostics,
//! admin, and support / export surfaces can inspect without manual reconstruction, so open and clone stay
//! distinct verbs (never a silently rewritten clone-into-open), checkout cost and credential posture stay
//! visible before any network or disk mutation, and an entry flow that cannot explain the literal target and
//! checkout posture it chose degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one stable source-locator object per entry flow.** [`resolve_source_locator_entry`] refuses to
//!   read as a clean, registry-bound locator entry unless it names a canonical registry token, a classified
//!   [source-locator kind][M5SourceLocatorKind], a repository-bootstrap role, covers every
//!   [resolution form][M5AcquisitionResolutionForm] (the canonical object, the accessible summary, and the
//!   audit record), publishes every locator field (literal target, resolved checkout root or archive
//!   container, staged-trust metadata, disclosed credential posture, signer / mirror provenance, and the
//!   distinct mirror / air-gap hint), preserves the literal target as a verb-faithful locator, and discloses
//!   the bootstrap credential posture before a network or mirror fetch; otherwise it degrades.
//! * **Keep the source locator from rewriting one acquisition verb into another.**
//!   [`literal_target_stays_verb_preserving`] rejects a locator entry whose literal target was rewritten into a
//!   different verb (for example a clone silently reopened over an existing local checkout) so it degrades to
//!   [`M5SourceLocatorEntryDegradeReason::SourceLocatorRewritesVerbOrHidesCredentialPosture`], and a
//!   network- or mirror-touching locator that hides its credential posture behind generic connected-state copy
//!   degrades the same way.
//! * **Keep the checkout plan from running repo-owned actions implicitly or hiding checkout cost.**
//!   [`resolve_checkout_plan_entry`] names a classified [checkout mode][M5CheckoutMode], requires the full
//!   ref-selection / depth-filter / submodule-mode / LFS-posture / destination-path / cost-band checkout-plan
//!   object, covers every resolution form, and degrades to
//!   [`M5CheckoutPlanEntryDegradeReason::CheckoutPlanRunsRepoOwnedActionOrHidesCost`] when the plan would run a
//!   repo-owned action (hook, task, extension, package restore, submodule or LFS hydration, generator install)
//!   without staging it, hides checkout cost or topology before mutation, or asserts an implicit mutation it
//!   cannot explain, so a checkout plan can never read as safe when it has quietly become an implicit
//!   bootstrap.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5RepositoryBootstrapRole`] role
//! vocabulary and the [`M5RepositoryBootstrapConsumerSurface`] consumer-surface taxonomy — so the shell,
//! entry, diagnostics, admin, workspace, git, trust, docs, CLI, and support surfaces can never fork their own
//! acquisition meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_repository_bootstrap_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_source_locator_and_checkout_plan_registries,
    seeded_m5_source_locator_and_checkout_plan_registries_local_path_source_beta_narrowed,
    seeded_m5_source_locator_and_checkout_plan_registries_sparse_checkout_preview_narrowed,
    M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_PACKET_ID,
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
    M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF, M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
    M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF, M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SourceLocatorCheckoutPlanRegistriesPacket`].
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_source_locator_and_checkout_plan_registries";

/// Schema version for M5 source-locator / checkout-plan registry records.
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_SCHEMA_REF: &str =
    "schemas/workspaces/m5-source-locator-and-checkout-plan-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_DOC_REF: &str =
    "docs/workspaces/m5_source_locator_and_checkout_plan_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-source-locator-and-checkout-plan-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-source-locator-and-checkout-plan-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-source-locator-and-checkout-plan-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/workspaces/m5-source-locator-and-checkout-plan-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5SourceLocatorCheckoutPlanRegistriesConsumerSurface =
    M5RepositoryBootstrapConsumerSurface;

/// One of the three resolution forms every source-locator or checkout-plan entry must hold across so its truth
/// keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or written
/// to the audit / support record. Minted by this lane because the frozen matrix names the source-locator and
/// checkout-plan *families* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AcquisitionResolutionForm {
    /// The canonical resolved source-locator / checkout-plan object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved acquisition discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved acquisition inspectable off-renderer.
    AuditRecord,
}

impl M5AcquisitionResolutionForm {
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

/// Controlled source-locator kind a source-locator entry resolves, so the canonical locator model shares one
/// registry rather than a hand-copied per-entry assumption. Minted by this lane because the frozen matrix
/// carries the acquisition families but not the concrete local-path / remote-forge / archive / mirror / managed
/// snapshot kind a locator entry resolves against. Every classified kind carries its canonical locator mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceLocatorKind {
    /// A local path source (open a local checkout that already exists on disk).
    LocalPathSource,
    /// A remote forge / URL source (clone a remote repository).
    RemoteForgeUrlSource,
    /// An archive / import-bundle source (open an archive container or import a bundle).
    ArchiveImportBundleSource,
    /// A mirror source (an offline or mirrored fetch that preserves signer / mirror provenance).
    MirrorSource,
    /// A managed-snapshot source (a resumable managed snapshot).
    ManagedSnapshotSource,
    /// The source-locator kind is unclassified, which is disallowed.
    KindUnclassified,
}

impl M5SourceLocatorKind {
    /// Every source-locator kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalPathSource,
        Self::RemoteForgeUrlSource,
        Self::ArchiveImportBundleSource,
        Self::MirrorSource,
        Self::ManagedSnapshotSource,
        Self::KindUnclassified,
    ];

    /// The five canonical source-locator kinds every claimed M5 entry flow resolves against.
    pub const CANONICAL_KINDS: [Self; 5] = [
        Self::LocalPathSource,
        Self::RemoteForgeUrlSource,
        Self::ArchiveImportBundleSource,
        Self::MirrorSource,
        Self::ManagedSnapshotSource,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPathSource => "local_path_source",
            Self::RemoteForgeUrlSource => "remote_forge_url_source",
            Self::ArchiveImportBundleSource => "archive_import_bundle_source",
            Self::MirrorSource => "mirror_source",
            Self::ManagedSnapshotSource => "managed_snapshot_source",
            Self::KindUnclassified => "kind_unclassified",
        }
    }

    /// Whether the kind is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::KindUnclassified)
    }

    /// The canonical locator mode for this kind.
    pub const fn canonical_locator_mode(self) -> &'static str {
        match self {
            Self::LocalPathSource => "local_path_locator",
            Self::RemoteForgeUrlSource => "remote_forge_url_locator",
            Self::ArchiveImportBundleSource => "archive_import_bundle_locator",
            Self::MirrorSource => "mirror_source_locator",
            Self::ManagedSnapshotSource => "managed_snapshot_locator",
            Self::KindUnclassified => "",
        }
    }

    /// Whether this kind touches the network or a mirror and so must disclose the bootstrap credential posture
    /// before the fetch.
    pub const fn touches_network_or_mirror(self) -> bool {
        matches!(self, Self::RemoteForgeUrlSource | Self::MirrorSource)
    }
}

/// Controlled checkout mode a checkout-plan entry must resolve its plan from, so a checkout plan shares one
/// registry rather than a hand-copied per-entry plan. Minted by this lane, tracking the full / partial / sparse
/// modes the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckoutMode {
    /// The full checkout plan.
    FullCheckoutPlan,
    /// The partial / shallow checkout plan (depth or filter bounded).
    PartialOrShallowCheckoutPlan,
    /// The sparse checkout plan (path-scoped working tree).
    SparseCheckoutPlan,
    /// The checkout mode is unclassified, which is disallowed.
    ModeUnclassified,
}

impl M5CheckoutMode {
    /// Every checkout mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullCheckoutPlan,
        Self::PartialOrShallowCheckoutPlan,
        Self::SparseCheckoutPlan,
        Self::ModeUnclassified,
    ];

    /// The three canonical modes every checkout plan must stay distinct across.
    pub const CANONICAL_MODES: [Self; 3] = [
        Self::FullCheckoutPlan,
        Self::PartialOrShallowCheckoutPlan,
        Self::SparseCheckoutPlan,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullCheckoutPlan => "full_checkout_plan",
            Self::PartialOrShallowCheckoutPlan => "partial_or_shallow_checkout_plan",
            Self::SparseCheckoutPlan => "sparse_checkout_plan",
            Self::ModeUnclassified => "mode_unclassified",
        }
    }

    /// Whether the checkout mode is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ModeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a source-locator or
/// checkout-plan token's meaning stays stable whether it appears in the shell, entry, diagnostics, admin, or a
/// support / export form. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AcquisitionSurfaceContext {
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

impl M5AcquisitionSurfaceContext {
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

/// One mandatory rendered part a source-locator or checkout-plan entry must be able to show, so no source
/// kind, literal target, resolved root, trust-stage metadata, credential posture, checkout-plan field, or
/// registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AcquisitionAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The source-locator kind the entry resolves (source-locator entry).
    SourceLocatorKind,
    /// The literal target and resolved checkout root / archive container the entry publishes (source-locator
    /// entry).
    LiteralTargetAndResolvedRoot,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The staged-trust metadata and disclosed credential posture the entry publishes (source-locator entry).
    TrustStageAndCredentialPosture,
    /// The checkout-plan fields (ref selection, mode, depth / filter, submodule mode, LFS posture, destination)
    /// the entry publishes (checkout-plan entry).
    CheckoutPlanFields,
    /// The expected disk / network cost band the entry publishes (checkout-plan entry).
    CostBandHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved source locator or checkout plan (both entries).
    PlainLanguageMeaning,
}

impl M5AcquisitionAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::SourceLocatorKind,
        Self::LiteralTargetAndResolvedRoot,
        Self::ResolutionFormCoverage,
        Self::TrustStageAndCredentialPosture,
        Self::CheckoutPlanFields,
        Self::CostBandHint,
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
            Self::SourceLocatorKind => "source_locator_kind",
            Self::LiteralTargetAndResolvedRoot => "literal_target_and_resolved_root",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::TrustStageAndCredentialPosture => "trust_stage_and_credential_posture",
            Self::CheckoutPlanFields => "checkout_plan_fields",
            Self::CostBandHint => "cost_band_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// source locator, a checkout plan, or a degraded source-locator / checkout-plan entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AcquisitionNextAction {
    /// Expand the resolved source locator's or checkout plan's plain-language meaning.
    ExpandAcquisitionMeaning,
    /// Inspect the source-locator kind or checkout mode the entry resolves.
    InspectKindOrMode,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5AcquisitionNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandAcquisitionMeaning,
        Self::InspectKindOrMode,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandAcquisitionMeaning => "expand_acquisition_meaning",
            Self::InspectKindOrMode => "inspect_kind_or_mode",
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
pub enum M5AcquisitionExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The repository-bootstrap families covered.
    RepositoryBootstrapFamilies,
    /// The source-locator kinds carried.
    SourceLocatorKinds,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The checkout modes carried.
    CheckoutModes,
    /// The render / surface context.
    SurfaceContext,
    /// The locator modes carried.
    LocatorModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5AcquisitionExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::RepositoryBootstrapFamilies,
        Self::SourceLocatorKinds,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::CheckoutModes,
        Self::SurfaceContext,
        Self::LocatorModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::RepositoryBootstrapFamilies,
        Self::SourceLocatorKinds,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::RepositoryBootstrapFamilies => "repository_bootstrap_families",
            Self::SourceLocatorKinds => "source_locator_kinds",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::CheckoutModes => "checkout_modes",
            Self::SurfaceContext => "surface_context",
            Self::LocatorModes => "locator_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a source-locator entry degraded below a clean, registry-bound state. The degrade-first ladder returns
/// one of these instead of ever letting a hand-copied, verb-rewriting, field-incomplete, or form-incomplete
/// entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceLocatorEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the locator means.
    LocatorTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The source-locator kind is unclassified (not in the resolved taxonomy).
    SourceLocatorKindUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    LocatorNotBoundToRegistry,
    /// The resolved source-locator object is incomplete: the literal target, resolved checkout root / archive
    /// container, staged-trust metadata, disclosed credential posture, signer / mirror provenance, or the
    /// distinct mirror / air-gap hint is unstated.
    SourceLocatorObjectIncomplete,
    /// The literal target was rewritten into a different acquisition verb (for example clone silently reopened
    /// over an existing local checkout), or a network / mirror locator hid its credential posture behind
    /// generic connected-state copy.
    SourceLocatorRewritesVerbOrHidesCredentialPosture,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A network- or mirror-touching locator did not disclose the credential posture before the fetch.
    CredentialPostureNotDisclosedBeforeNetwork,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SourceLocatorEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::LocatorTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::SourceLocatorKindUnclassified,
        Self::LocatorNotBoundToRegistry,
        Self::SourceLocatorObjectIncomplete,
        Self::SourceLocatorRewritesVerbOrHidesCredentialPosture,
        Self::ResolutionFormCoverageIncomplete,
        Self::CredentialPostureNotDisclosedBeforeNetwork,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocatorTokenUnstated => "locator_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::SourceLocatorKindUnclassified => "source_locator_kind_unclassified",
            Self::LocatorNotBoundToRegistry => "locator_not_bound_to_registry",
            Self::SourceLocatorObjectIncomplete => "source_locator_object_incomplete",
            Self::SourceLocatorRewritesVerbOrHidesCredentialPosture => {
                "source_locator_rewrites_verb_or_hides_credential_posture"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::CredentialPostureNotDisclosedBeforeNetwork => {
                "credential_posture_not_disclosed_before_network"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AcquisitionNextAction {
        match self {
            Self::LocatorTokenUnstated | Self::LocatorNotBoundToRegistry => {
                M5AcquisitionNextAction::TraceCanonicalRegistry
            }
            Self::SourceLocatorKindUnclassified
            | Self::SourceLocatorObjectIncomplete
            | Self::SourceLocatorRewritesVerbOrHidesCredentialPosture => {
                M5AcquisitionNextAction::InspectKindOrMode
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5AcquisitionNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::CredentialPostureNotDisclosedBeforeNetwork
            | Self::ProofStale => M5AcquisitionNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            Self::LocatorTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SourceLocatorKindUnclassified | Self::SourceLocatorObjectIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::SourceLocatorUnstated
            }
            Self::LocatorNotBoundToRegistry => {
                M5RepositoryBootstrapDowngradeTrigger::CheckoutPlanBoundaryDriftedBySurface
            }
            Self::SourceLocatorRewritesVerbOrHidesCredentialPosture => {
                M5RepositoryBootstrapDowngradeTrigger::RewroteCloneIntoOpenWhenLocalCheckoutAlreadyExists
            }
            Self::CredentialPostureNotDisclosedBeforeNetwork => {
                M5RepositoryBootstrapDowngradeTrigger::HidBootstrapCredentialPostureBehindGenericConnectedStateCopy
            }
            Self::ProofStale => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a checkout-plan entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckoutPlanEntryDegradeReason {
    /// The canonical registry token name is unstated.
    PlanTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The checkout mode is unclassified (not in the resolved taxonomy).
    CheckoutModeUnclassified,
    /// The checkout plan would run a repo-owned action implicitly, hides checkout cost or topology before
    /// mutation, or asserts an implicit mutation it cannot explain, or it dropped one of the required
    /// checkout-plan fields (ref selection, depth / filter, submodule mode, LFS posture, destination, cost
    /// band).
    CheckoutPlanRunsRepoOwnedActionOrHidesCost,
    /// The canonical / accessible / audit resolution-form coverage of the checkout plan is incomplete.
    PlanFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CheckoutPlanEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PlanTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CheckoutModeUnclassified,
        Self::CheckoutPlanRunsRepoOwnedActionOrHidesCost,
        Self::PlanFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlanTokenUnstated => "plan_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CheckoutModeUnclassified => "checkout_mode_unclassified",
            Self::CheckoutPlanRunsRepoOwnedActionOrHidesCost => {
                "checkout_plan_runs_repo_owned_action_or_hides_cost"
            }
            Self::PlanFormCoverageIncomplete => "plan_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5AcquisitionNextAction {
        match self {
            Self::PlanTokenUnstated => M5AcquisitionNextAction::TraceCanonicalRegistry,
            Self::CheckoutModeUnclassified | Self::CheckoutPlanRunsRepoOwnedActionOrHidesCost => {
                M5AcquisitionNextAction::InspectKindOrMode
            }
            Self::PlanFormCoverageIncomplete => {
                M5AcquisitionNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5AcquisitionNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            Self::PlanTokenUnstated => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::CheckoutModeUnclassified => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::CheckoutPlanRunsRepoOwnedActionOrHidesCost => {
                M5RepositoryBootstrapDowngradeTrigger::RanRepoOwnedActionsImplicitlyDuringAcquisition
            }
            Self::PlanFormCoverageIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::CheckoutPlanBoundaryDriftedBySurface
            }
            Self::ProofStale => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_source_locator_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SourceLocatorEntryResolutionInput {
    /// Stable identity of the source-locator-registry entry.
    pub entry_id: String,
    /// The stable entry-flow ID this locator binds to (e.g. `entry.acme.open-local`); empty means unstated.
    pub entry_flow_id: String,
    /// The canonical registry token name (e.g. `source.locator.local_path`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5RepositoryBootstrapRole,
    /// The source-locator kind this entry resolves.
    pub source_locator_kind: M5SourceLocatorKind,
    /// The render / surface context.
    pub surface_context: M5AcquisitionSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5AcquisitionResolutionForm>,
    /// The published literal target preserved verbatim; empty means unstated.
    pub literal_target: String,
    /// The published resolved checkout root or archive container; empty means unstated.
    pub resolved_root_or_container: String,
    /// The published staged-trust metadata reference; empty means unstated.
    pub trust_stage_metadata: String,
    /// The published disclosed credential posture reference; empty means unstated.
    pub credential_posture: String,
    /// The published signer / mirror provenance reference; empty means unstated.
    pub signer_or_mirror_provenance: String,
    /// The published mirror / air-gap hint kept distinct from authoritative provenance; empty means unstated.
    pub mirror_or_air_gap_hint: String,
    /// True when the behavior traces to the source-locator registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the literal target stays a verb-faithful locator and is never rewritten into a different verb
    /// (a hard invariant when `false`).
    pub literal_target_preserved: bool,
    /// True when this locator touches the network or a mirror.
    pub touches_network_or_mirror: bool,
    /// True when the bootstrap credential posture is disclosed before the network or mirror fetch.
    pub credential_posture_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe source-locator-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSourceLocatorEntry {
    /// Stable identity of the source-locator-registry entry.
    pub entry_id: String,
    /// The stable entry-flow ID this locator binds to.
    pub entry_flow_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must stage trust and disclose provenance before bootstrap.
    pub semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: bool,
    /// The source-locator-kind token named by the entry.
    pub source_locator_kind: String,
    /// Whether the source-locator kind is classified into the resolved taxonomy.
    pub source_locator_kind_is_classified: bool,
    /// The canonical locator mode for the entry's kind.
    pub canonical_locator_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published literal target.
    pub literal_target: String,
    /// The published resolved checkout root or archive container.
    pub resolved_root_or_container: String,
    /// The published staged-trust metadata.
    pub trust_stage_metadata: String,
    /// The published disclosed credential posture.
    pub credential_posture: String,
    /// The published signer / mirror provenance.
    pub signer_or_mirror_provenance: String,
    /// The published mirror / air-gap hint.
    pub mirror_or_air_gap_hint: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved source-locator object publishes every required field.
    pub source_locator_object_complete: bool,
    /// Whether the entry traces to the source-locator registry.
    pub bound_to_registry: bool,
    /// Whether the literal target stays a verb-faithful locator.
    pub literal_target_preserved: bool,
    /// Whether this locator touches the network or a mirror.
    pub touches_network_or_mirror: bool,
    /// Whether the bootstrap credential posture is disclosed before the network or mirror fetch.
    pub credential_posture_disclosed: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5SourceLocatorEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AcquisitionNextAction,
    /// Whether the locator resolves to one stable object across every claimed entry flow (clean entry naming
    /// every fact).
    pub locator_resolves_across_entry_flows: bool,
}

impl M5ResolvedSourceLocatorEntry {
    /// Whether this source-locator entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_checkout_plan_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CheckoutPlanEntryResolutionInput {
    /// Stable identity of the checkout-plan entry.
    pub entry_id: String,
    /// The stable source-ref this plan binds to; empty means unstated.
    pub source_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5RepositoryBootstrapRole,
    /// The checkout mode this entry must resolve its plan from.
    pub checkout_mode: M5CheckoutMode,
    /// The render / surface context.
    pub surface_context: M5AcquisitionSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5AcquisitionResolutionForm>,
    /// The published ref selection; empty means missing.
    pub ref_selection: String,
    /// The published depth / filter; empty means missing.
    pub depth_filter: String,
    /// The published submodule mode; empty means missing.
    pub submodule_mode: String,
    /// The published LFS posture; empty means missing.
    pub lfs_posture: String,
    /// The published destination path; empty means missing.
    pub destination_path: String,
    /// The published expected disk / network cost band; empty means missing.
    pub cost_band: String,
    /// True when the plan keeps checkout cost and topology visible before any network or disk mutation.
    pub keeps_cost_visible_before_mutation: bool,
    /// True when the plan is truthful (never claims a safe plan over an implicit bootstrap).
    pub plan_is_truthful: bool,
    /// True when the plan schedules a repo-owned action (hook, task, extension, restore, hydration, generator).
    pub repo_owned_action_scheduled: bool,
    /// True when any scheduled repo-owned action is staged (never run implicitly during acquisition).
    pub repo_owned_action_staged_not_auto_run: bool,
    /// True when the plan asserts an implicit mutation.
    pub implicit_mutation_asserted: bool,
    /// True when an asserted implicit mutation is explained rather than left implicit.
    pub implicit_mutation_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe checkout-plan projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCheckoutPlanEntry {
    /// Stable identity of the checkout-plan entry.
    pub entry_id: String,
    /// The stable source-ref this plan binds to.
    pub source_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must stage trust and disclose provenance before bootstrap.
    pub semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: bool,
    /// The checkout-mode token named by the entry.
    pub checkout_mode: String,
    /// Whether the checkout mode is classified into the resolved taxonomy.
    pub checkout_mode_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published ref selection.
    pub ref_selection: String,
    /// The published depth / filter.
    pub depth_filter: String,
    /// The published submodule mode.
    pub submodule_mode: String,
    /// The published LFS posture.
    pub lfs_posture: String,
    /// The published destination path.
    pub destination_path: String,
    /// The published expected disk / network cost band.
    pub cost_band: String,
    /// Whether the plan keeps checkout cost and topology visible before mutation.
    pub keeps_cost_visible_before_mutation: bool,
    /// Whether the plan is truthful.
    pub plan_is_truthful: bool,
    /// Whether the plan schedules a repo-owned action.
    pub repo_owned_action_scheduled: bool,
    /// Whether any scheduled repo-owned action is staged.
    pub repo_owned_action_staged_not_auto_run: bool,
    /// Whether the plan asserts an implicit mutation.
    pub implicit_mutation_asserted: bool,
    /// Whether an asserted implicit mutation is explained.
    pub implicit_mutation_explained: bool,
    /// Whether the plan stays honest (cost visible, no implicit repo-owned action, explained mutations).
    pub checkout_plan_stays_honest: bool,
    /// Whether the entry provides the complete checkout-plan object (ref selection, depth / filter, submodule
    /// mode, LFS posture, destination, cost band).
    pub provides_complete_checkout_plan: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5CheckoutPlanEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5AcquisitionNextAction,
    /// Whether the checkout plan is safe on every claimed source (clean entry naming every fact).
    pub plan_safe_on_every_source: bool,
}

impl M5ResolvedCheckoutPlanEntry {
    /// Whether this checkout-plan entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5AcquisitionResolutionError {
    /// The source-locator-entry id was empty.
    EmptySourceLocatorEntryId,
    /// The checkout-plan-entry id was empty.
    EmptyCheckoutPlanEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5AcquisitionResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySourceLocatorEntryId => "empty_source_locator_entry_id",
            Self::EmptyCheckoutPlanEntryId => "empty_checkout_plan_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5AcquisitionResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 source-locator / checkout-plan registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AcquisitionResolutionError {}

fn form_tokens(forms: &[M5AcquisitionResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5AcquisitionResolutionForm]) -> bool {
    let present: BTreeSet<M5AcquisitionResolutionForm> = forms.iter().copied().collect();
    M5AcquisitionResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved source-locator object publishes every required field: locator mode (via a classified
/// kind), literal target, resolved checkout root or archive container, staged-trust metadata, disclosed
/// credential posture, signer / mirror provenance, and the distinct mirror / air-gap hint. An unclassified
/// kind or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn source_locator_object_is_complete(
    kind: M5SourceLocatorKind,
    literal_target: &str,
    resolved_root_or_container: &str,
    trust_stage_metadata: &str,
    credential_posture: &str,
    signer_or_mirror_provenance: &str,
    mirror_or_air_gap_hint: &str,
) -> bool {
    kind.is_classified()
        && !literal_target.trim().is_empty()
        && !resolved_root_or_container.trim().is_empty()
        && !trust_stage_metadata.trim().is_empty()
        && !credential_posture.trim().is_empty()
        && !signer_or_mirror_provenance.trim().is_empty()
        && !mirror_or_air_gap_hint.trim().is_empty()
}

/// Whether the source locator stays a verb-faithful, credential-disclosing locator: the kind must be
/// classified, the literal target must be preserved (never rewritten into a different acquisition verb), and a
/// network- or mirror-touching locator must disclose the bootstrap credential posture before the fetch. An
/// unclassified kind, a rewritten literal target, or a hidden credential posture never matches.
pub fn literal_target_stays_verb_preserving(
    kind: M5SourceLocatorKind,
    literal_target_preserved: bool,
    touches_network_or_mirror: bool,
    credential_posture_disclosed: bool,
) -> bool {
    kind.is_classified()
        && literal_target_preserved
        && (!touches_network_or_mirror || credential_posture_disclosed)
}

/// Whether a checkout plan stays honest: the mode must be classified, the plan must be truthful, it must keep
/// checkout cost and topology visible before mutation, any scheduled repo-owned action must be staged rather
/// than run implicitly, and any asserted implicit mutation must be explained.
pub fn checkout_plan_stays_honest(
    mode: M5CheckoutMode,
    plan_is_truthful: bool,
    keeps_cost_visible_before_mutation: bool,
    repo_owned_action_scheduled: bool,
    repo_owned_action_staged_not_auto_run: bool,
    implicit_mutation_asserted: bool,
    implicit_mutation_explained: bool,
) -> bool {
    mode.is_classified()
        && plan_is_truthful
        && keeps_cost_visible_before_mutation
        && (!repo_owned_action_scheduled || repo_owned_action_staged_not_auto_run)
        && (!implicit_mutation_asserted || implicit_mutation_explained)
}

/// Resolves a source-locator-registry entry so it stays bound to the source-locator registry: the entry names
/// its canonical token, semantic role, and source-locator kind, covers all three resolution forms, publishes a
/// complete source-locator object (literal target, resolved root / container, staged-trust metadata, disclosed
/// credential posture, signer / mirror provenance, distinct mirror / air-gap hint), preserves the literal
/// target as a verb-faithful locator, and discloses the credential posture before a network or mirror fetch.
pub fn resolve_source_locator_entry(
    input: M5SourceLocatorEntryResolutionInput,
) -> Result<M5ResolvedSourceLocatorEntry, M5AcquisitionResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5AcquisitionResolutionError::EmptySourceLocatorEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.entry_flow_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.literal_target)
        || string_is_forbidden(&input.resolved_root_or_container)
        || string_is_forbidden(&input.trust_stage_metadata)
        || string_is_forbidden(&input.credential_posture)
        || string_is_forbidden(&input.signer_or_mirror_provenance)
        || string_is_forbidden(&input.mirror_or_air_gap_hint)
    {
        return Err(M5AcquisitionResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = source_locator_object_is_complete(
        input.source_locator_kind,
        &input.literal_target,
        &input.resolved_root_or_container,
        &input.trust_stage_metadata,
        &input.credential_posture,
        &input.signer_or_mirror_provenance,
        &input.mirror_or_air_gap_hint,
    );
    let verb_preserving_ok = literal_target_stays_verb_preserving(
        input.source_locator_kind,
        input.literal_target_preserved,
        input.touches_network_or_mirror,
        input.credential_posture_disclosed,
    );
    let credential_undisclosed =
        input.touches_network_or_mirror && !input.credential_posture_disclosed;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SourceLocatorEntryDegradeReason::LocatorTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SourceLocatorEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.source_locator_kind.is_classified() {
        Some(M5SourceLocatorEntryDegradeReason::SourceLocatorKindUnclassified)
    } else if !input.bound_to_registry {
        Some(M5SourceLocatorEntryDegradeReason::LocatorNotBoundToRegistry)
    } else if !object_complete {
        Some(M5SourceLocatorEntryDegradeReason::SourceLocatorObjectIncomplete)
    } else if !verb_preserving_ok {
        Some(M5SourceLocatorEntryDegradeReason::SourceLocatorRewritesVerbOrHidesCredentialPosture)
    } else if !all_forms {
        Some(M5SourceLocatorEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if credential_undisclosed {
        Some(M5SourceLocatorEntryDegradeReason::CredentialPostureNotDisclosedBeforeNetwork)
    } else if !input.proof_fresh {
        Some(M5SourceLocatorEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AcquisitionNextAction::ExpandAcquisitionMeaning,
    };

    Ok(M5ResolvedSourceLocatorEntry {
        entry_id: input.entry_id,
        entry_flow_id: input.entry_flow_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: input
            .semantic_role
            .must_stage_trust_and_disclose_provenance_before_bootstrap(),
        source_locator_kind: input.source_locator_kind.as_str().to_owned(),
        source_locator_kind_is_classified: input.source_locator_kind.is_classified(),
        canonical_locator_mode: input
            .source_locator_kind
            .canonical_locator_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        literal_target: input.literal_target,
        resolved_root_or_container: input.resolved_root_or_container,
        trust_stage_metadata: input.trust_stage_metadata,
        credential_posture: input.credential_posture,
        signer_or_mirror_provenance: input.signer_or_mirror_provenance,
        mirror_or_air_gap_hint: input.mirror_or_air_gap_hint,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        source_locator_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        literal_target_preserved: input.literal_target_preserved,
        touches_network_or_mirror: input.touches_network_or_mirror,
        credential_posture_disclosed: input.credential_posture_disclosed,
        degrade_reason,
        next_action,
        locator_resolves_across_entry_flows: degrade_reason.is_none(),
    })
}

/// Resolves a checkout-plan entry so its plan stays safe: the entry names its canonical token, semantic role,
/// and checkout mode, covers all three resolution forms, provides the complete ref-selection / depth-filter /
/// submodule-mode / LFS-posture / destination-path / cost-band checkout-plan object, and degrades honestly when
/// the plan would run a repo-owned action implicitly, hides checkout cost before mutation, or asserts an
/// implicit mutation it cannot explain.
pub fn resolve_checkout_plan_entry(
    input: M5CheckoutPlanEntryResolutionInput,
) -> Result<M5ResolvedCheckoutPlanEntry, M5AcquisitionResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5AcquisitionResolutionError::EmptyCheckoutPlanEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.source_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.ref_selection)
        || string_is_forbidden(&input.depth_filter)
        || string_is_forbidden(&input.submodule_mode)
        || string_is_forbidden(&input.lfs_posture)
        || string_is_forbidden(&input.destination_path)
        || string_is_forbidden(&input.cost_band)
    {
        return Err(M5AcquisitionResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let plan_stays_honest = checkout_plan_stays_honest(
        input.checkout_mode,
        input.plan_is_truthful,
        input.keeps_cost_visible_before_mutation,
        input.repo_owned_action_scheduled,
        input.repo_owned_action_staged_not_auto_run,
        input.implicit_mutation_asserted,
        input.implicit_mutation_explained,
    );
    let provides_plan = input.checkout_mode.is_classified()
        && !input.ref_selection.trim().is_empty()
        && !input.depth_filter.trim().is_empty()
        && !input.submodule_mode.trim().is_empty()
        && !input.lfs_posture.trim().is_empty()
        && !input.destination_path.trim().is_empty()
        && !input.cost_band.trim().is_empty()
        && plan_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CheckoutPlanEntryDegradeReason::PlanTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CheckoutPlanEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.checkout_mode.is_classified() {
        Some(M5CheckoutPlanEntryDegradeReason::CheckoutModeUnclassified)
    } else if !provides_plan {
        Some(M5CheckoutPlanEntryDegradeReason::CheckoutPlanRunsRepoOwnedActionOrHidesCost)
    } else if !all_forms {
        Some(M5CheckoutPlanEntryDegradeReason::PlanFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5CheckoutPlanEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5AcquisitionNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedCheckoutPlanEntry {
        entry_id: input.entry_id,
        source_ref: input.source_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: input
            .semantic_role
            .must_stage_trust_and_disclose_provenance_before_bootstrap(),
        checkout_mode: input.checkout_mode.as_str().to_owned(),
        checkout_mode_is_classified: input.checkout_mode.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        ref_selection: input.ref_selection,
        depth_filter: input.depth_filter,
        submodule_mode: input.submodule_mode,
        lfs_posture: input.lfs_posture,
        destination_path: input.destination_path,
        cost_band: input.cost_band,
        keeps_cost_visible_before_mutation: input.keeps_cost_visible_before_mutation,
        plan_is_truthful: input.plan_is_truthful,
        repo_owned_action_scheduled: input.repo_owned_action_scheduled,
        repo_owned_action_staged_not_auto_run: input.repo_owned_action_staged_not_auto_run,
        implicit_mutation_asserted: input.implicit_mutation_asserted,
        implicit_mutation_explained: input.implicit_mutation_explained,
        checkout_plan_stays_honest: plan_stays_honest,
        provides_complete_checkout_plan: provides_plan,
        degrade_reason,
        next_action,
        plan_safe_on_every_source: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved source-locator and checkout-plan entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceLocatorCheckoutPlanRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SourceLocatorCheckoutPlanRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5AcquisitionAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5AcquisitionExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    /// Resolved source-locator-registry examples.
    pub source_locator_entries: Vec<M5ResolvedSourceLocatorEntry>,
    /// Resolved checkout-plan examples.
    pub checkout_plan_entries: Vec<M5ResolvedCheckoutPlanEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the source-locator and checkout-plan domain
    /// schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never rewrites clone into open because a local checkout already exists. MUST be
    /// `false`.
    pub rewrites_clone_into_open_when_local_checkout_already_exists: bool,
    /// Hard invariant: this row never runs repo-owned actions implicitly during acquisition. MUST be `false`.
    pub runs_repo_owned_actions_implicitly_during_acquisition: bool,
    /// Hard invariant: this row never hides checkout cost, topology, or credential posture before mutation.
    /// MUST be `false`.
    pub hides_checkout_cost_topology_or_credential_posture_before_mutation: bool,
    /// Hard invariant: this row never collapses distinct acquisition verbs into one runtime path. MUST be
    /// `false`.
    pub collapses_distinct_acquisition_verbs_into_one_runtime_path: bool,
}

impl M5SourceLocatorCheckoutPlanRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5AcquisitionAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5AcquisitionAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5AcquisitionExportField> =
            self.export_fields.iter().copied().collect();
        M5AcquisitionExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.rewrites_clone_into_open_when_local_checkout_already_exists
            && !self.runs_repo_owned_actions_implicitly_during_acquisition
            && !self.hides_checkout_cost_topology_or_credential_posture_before_mutation
            && !self.collapses_distinct_acquisition_verbs_into_one_runtime_path
    }

    /// True when a clean source-locator entry preserves registry-bound truth: it traces to the registry, keeps
    /// a classified source-locator kind, publishes a complete locator object, preserves the literal target,
    /// covers all three resolution forms, and discloses the credential posture before a network or mirror
    /// fetch.
    fn locator_is_honest(ex: &M5ResolvedSourceLocatorEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.source_locator_kind_is_classified
                && ex.source_locator_object_complete
                && ex.literal_target_preserved
                && ex.covers_all_resolution_forms
                && (!ex.touches_network_or_mirror || ex.credential_posture_disclosed))
    }

    /// True when a clean checkout-plan entry preserves a safe plan: it keeps a classified mode, provides the
    /// complete checkout-plan object, stays honest, and covers all three resolution forms.
    fn plan_is_honest(ex: &M5ResolvedCheckoutPlanEntry) -> bool {
        !ex.is_clean()
            || (ex.checkout_mode_is_classified
                && ex.provides_complete_checkout_plan
                && ex.checkout_plan_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.source_locator_entries
            .iter()
            .all(Self::locator_is_honest)
            && self.checkout_plan_entries.iter().all(Self::plan_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceLocatorCheckoutPlanRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Source-locator-kind tokens (minted by this lane).
    pub source_locator_kinds: Vec<String>,
    /// Checkout-mode tokens (minted by this lane).
    pub checkout_modes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Source-locator-entry degrade-reason tokens.
    pub source_locator_degrade_reasons: Vec<String>,
    /// Checkout-plan-entry degrade-reason tokens.
    pub checkout_plan_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SourceLocatorCheckoutPlanRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5RepositoryBootstrapRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5AcquisitionResolutionForm::ALL, |v| v.as_str()),
            source_locator_kinds: tokens(&M5SourceLocatorKind::ALL, |v| v.as_str()),
            checkout_modes: tokens(&M5CheckoutMode::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5AcquisitionSurfaceContext::ALL, |v| v.as_str()),
            source_locator_degrade_reasons: tokens(&M5SourceLocatorEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            checkout_plan_degrade_reasons: tokens(&M5CheckoutPlanEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5AcquisitionAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5AcquisitionNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5AcquisitionExportField::ALL, |v| v.as_str()),
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
pub struct M5SourceLocatorCheckoutPlanRegistriesGovernanceReview {
    /// The locator registry names a canonical token, semantic role, and source-locator kind for every entry.
    pub locator_registry_names_token_role_and_kind: bool,
    /// Every claimed entry flow resolves to one stable source-locator object from the shared registry, not
    /// per-entry reconstruction.
    pub entry_flow_resolves_to_stable_object_from_shared_registry: bool,
    /// The literal target, resolved root / container, staged-trust metadata, disclosed credential posture,
    /// signer / mirror provenance, and distinct mirror / air-gap hint are published for every resolved locator.
    pub literal_target_root_trust_and_provenance_published: bool,
    /// Open and clone stay distinct verbs; the literal target is never rewritten into a different verb.
    pub open_and_clone_stay_distinct_verbs: bool,
    /// The checkout plan keeps checkout cost and topology visible and never runs a repo-owned action
    /// implicitly.
    pub checkout_plan_keeps_cost_visible_and_stages_trust: bool,
    /// The bootstrap credential posture is disclosed before any network or mirror fetch.
    pub credential_posture_disclosed_before_network: bool,
    /// Every source-locator and checkout-plan entry covers the canonical / accessible / audit resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Source-locator and checkout-plan behavior stay bound to the shared registries rather than hand-copied
    /// per entry flow.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shell, entry, diagnostics, and admin read a single acquisition source.
    pub shell_entry_diagnostics_admin_read_single_source: bool,
    /// A verb rewrite, an incomplete object, or an implicit bootstrap is caught by fixtures before release
    /// evidence turns green.
    pub locator_or_plan_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceLocatorCheckoutPlanRegistriesConsumerProjection {
    /// Shell and entry consume the shared source-locator registry.
    pub shell_and_entry_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared checkout-plan registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Git and workspace services consume the shared registries.
    pub git_and_workspace_services_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical source-locator and checkout-plan domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical source-locator / checkout-plan registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceLocatorCheckoutPlanRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceLocatorCheckoutPlanRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting repository-bootstrap audit for the lane.
    pub repository_bootstrap_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SourceLocatorCheckoutPlanRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SourceLocatorCheckoutPlanRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SourceLocatorCheckoutPlanRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SourceLocatorCheckoutPlanRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SourceLocatorCheckoutPlanRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SourceLocatorCheckoutPlanRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SourceLocatorCheckoutPlanRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SourceLocatorCheckoutPlanRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 source-locator and checkout-plan registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceLocatorCheckoutPlanRegistriesPacket {
    /// Record kind; must equal [`M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SourceLocatorCheckoutPlanRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SourceLocatorCheckoutPlanRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SourceLocatorCheckoutPlanRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SourceLocatorCheckoutPlanRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SourceLocatorCheckoutPlanRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SourceLocatorCheckoutPlanRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SourceLocatorCheckoutPlanRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SourceLocatorCheckoutPlanRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5SourceLocatorCheckoutPlanRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_RECORD_KIND {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 source-locator / checkout-plan registries packet serializes"),
        ) {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 source-locator / checkout-plan registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,source_locator_entries,checkout_plan_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .source_locator_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.checkout_plan_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.source_locator_entries.len(),
                row.checkout_plan_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Source-Locator and Checkout-Plan Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Source-locator kinds: {}\n",
            self.vocabulary_set.source_locator_kinds.join(", ")
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
                "  - Source-locator entries: {} / checkout-plan entries: {}\n",
                row.source_locator_entries.len(),
                row.checkout_plan_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry acquisition reference table generated from the registry, so docs and admin
    /// runbooks render the same locator-mode / literal-target / resolved-root / trust-stage / credential-posture
    /// / signer-provenance truth the resolvers produced rather than a hand-copied acquisition table. Only clean,
    /// registry-bound source-locator entries are listed.
    pub fn render_source_acquisition_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| entry_flow_id | locator_mode | literal_target | resolved_root_or_container | trust_stage_metadata | credential_posture | signer_or_mirror_provenance |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.source_locator_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.entry_flow_id,
                    ex.canonical_locator_mode,
                    ex.literal_target,
                    ex.resolved_root_or_container,
                    ex.trust_stage_metadata,
                    ex.credential_posture,
                    ex.signer_or_mirror_provenance
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SourceLocatorCheckoutPlanRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SourceLocatorCheckoutPlanRegistriesViolation>),
}

impl fmt::Display for M5SourceLocatorCheckoutPlanRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 source-locator / checkout-plan registries export parse failed: {error}"
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
                    "m5 source-locator / checkout-plan registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SourceLocatorCheckoutPlanRegistriesArtifactError {}

/// Validation failures emitted by [`M5SourceLocatorCheckoutPlanRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SourceLocatorCheckoutPlanRegistriesViolation {
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
    /// A registry row does not point at both the source-locator and checkout-plan domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, verb-rewriting, field-incomplete,
    /// form-incomplete, or a checkout-plan entry missing the complete plan object).
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
    /// Source-locator-resolution is not proven: clean source-locator entries do not cover the canonical
    /// source-locator kinds or the first shell / entry / diagnostics / admin / support surfaces, no
    /// object-incomplete example degrades, or a clean locator entry published an incomplete object.
    SourceLocatorResolutionNotProven,
    /// Literal-target-preservation is not proven: no verb-rewrite example and no unbound example degrade, no
    /// clean verb-preserving locator entry is present, or a clean locator entry lost the literal target or is
    /// unbound.
    LiteralTargetPreservationNotProven,
    /// Checkout-plan-integrity is not proven: clean checkout-plan entries do not cover the canonical full /
    /// partial / sparse modes with full resolution-form coverage while providing the complete plan object, no
    /// implicit-bootstrap or form-incomplete example degrades, or a clean checkout-plan entry is missing the
    /// complete plan object.
    CheckoutPlanIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SourceLocatorCheckoutPlanRegistriesViolation {
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
            Self::SourceLocatorResolutionNotProven => "source_locator_resolution_not_proven",
            Self::LiteralTargetPreservationNotProven => "literal_target_preservation_not_proven",
            Self::CheckoutPlanIntegrityNotProven => "checkout_plan_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_source_locator_and_checkout_plan_registries_export() -> Result<
    M5SourceLocatorCheckoutPlanRegistriesPacket,
    M5SourceLocatorCheckoutPlanRegistriesArtifactError,
> {
    let packet: M5SourceLocatorCheckoutPlanRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-source-locator-and-checkout-plan-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SourceLocatorCheckoutPlanRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SourceLocatorCheckoutPlanRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SourceLocatorCheckoutPlanRegistriesPacket,
    violations: &mut Vec<M5SourceLocatorCheckoutPlanRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_SCHEMA_REF,
        M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SourceLocatorCheckoutPlanRegistriesPacket,
    violations: &mut Vec<M5SourceLocatorCheckoutPlanRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::NoRegistryRows);
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
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5SourceLocatorCheckoutPlanRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5SourceLocatorCheckoutPlanRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF)
        {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.source_locator_entries.is_empty() || row.checkout_plan_entries.is_empty() {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SourceLocatorCheckoutPlanRegistriesPacket,
    violations: &mut Vec<M5SourceLocatorCheckoutPlanRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.locator_registry_names_token_role_and_kind,
        review.entry_flow_resolves_to_stable_object_from_shared_registry,
        review.literal_target_root_trust_and_provenance_published,
        review.open_and_clone_stay_distinct_verbs,
        review.checkout_plan_keeps_cost_visible_and_stages_trust,
        review.credential_posture_disclosed_before_network,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shell_entry_diagnostics_admin_read_single_source,
        review.locator_or_plan_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5SourceLocatorCheckoutPlanRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SourceLocatorCheckoutPlanRegistriesPacket,
    violations: &mut Vec<M5SourceLocatorCheckoutPlanRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_entry_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.git_and_workspace_services_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations
                .push(M5SourceLocatorCheckoutPlanRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SourceLocatorCheckoutPlanRegistriesPacket,
    violations: &mut Vec<M5SourceLocatorCheckoutPlanRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SourceLocatorCheckoutPlanRegistriesPacket,
    violations: &mut Vec<M5SourceLocatorCheckoutPlanRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.repository_bootstrap_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SourceLocatorCheckoutPlanRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SourceLocatorCheckoutPlanRegistriesPacket,
    violations: &mut Vec<M5SourceLocatorCheckoutPlanRegistriesViolation>,
) {
    let locators = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.source_locator_entries.iter())
    };
    let plans = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.checkout_plan_entries.iter())
    };

    // AC1: every claimed entry flow resolves to one stable source-locator object with literal-target /
    // resolved-root / trust-stage / credential-posture / provenance fields. Clean locator entries cover the
    // canonical source-locator kinds and the first shell / entry / diagnostics / admin / support surfaces, an
    // object-incomplete example degrades, and no clean locator entry published an incomplete object.
    let clean_kinds: BTreeSet<String> = locators()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.source_locator_kind.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = locators()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let kinds_covered = M5SourceLocatorKind::CANONICAL_KINDS
        .iter()
        .all(|k| clean_kinds.contains(k.as_str()));
    let first_surfaces_covered = M5AcquisitionSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = locators().any(|ex| {
        ex.degrade_reason == Some(M5SourceLocatorEntryDegradeReason::SourceLocatorObjectIncomplete)
    });
    let no_clean_incomplete =
        !locators().any(|ex| ex.is_clean() && !ex.source_locator_object_complete);
    if !(kinds_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations
            .push(M5SourceLocatorCheckoutPlanRegistriesViolation::SourceLocatorResolutionNotProven);
    }

    // AC2: the literal target stays verb-faithful — open and clone stay distinct verbs. A verb-rewrite example
    // degrades, an unbound example degrades, at least one clean verb-preserving locator entry is present, and no
    // clean locator entry lost the literal target or is unbound.
    let rewrite_degrades = locators().any(|ex| {
        ex.degrade_reason
            == Some(M5SourceLocatorEntryDegradeReason::SourceLocatorRewritesVerbOrHidesCredentialPosture)
    });
    let unbound_degrades = locators().any(|ex| {
        ex.degrade_reason == Some(M5SourceLocatorEntryDegradeReason::LocatorNotBoundToRegistry)
    });
    let preserving_clean_locator =
        locators().any(|ex| ex.is_clean() && ex.literal_target_preserved);
    let no_clean_unbound = !locators().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_rewritten = !locators().any(|ex| ex.is_clean() && !ex.literal_target_preserved);
    if !(rewrite_degrades
        && unbound_degrades
        && preserving_clean_locator
        && no_clean_unbound
        && no_clean_rewritten)
    {
        violations.push(
            M5SourceLocatorCheckoutPlanRegistriesViolation::LiteralTargetPreservationNotProven,
        );
    }

    // AC3: the suite fails when a checkout plan collapses into an implicit bootstrap. Clean checkout-plan entries
    // cover every canonical full / partial / sparse mode with full resolution-form coverage while providing the
    // complete plan object, an implicit-bootstrap example degrades, a form-incomplete example degrades, and no
    // clean checkout-plan entry is missing the complete plan object.
    let clean_plan_modes: BTreeSet<String> = plans()
        .filter(|ex| {
            ex.is_clean()
                && ex.checkout_mode_is_classified
                && ex.provides_complete_checkout_plan
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.checkout_mode.clone())
        .collect();
    let plan_modes_covered = M5CheckoutMode::CANONICAL_MODES
        .iter()
        .all(|m| clean_plan_modes.contains(m.as_str()));
    let implicit_degrades = plans().any(|ex| {
        ex.degrade_reason
            == Some(M5CheckoutPlanEntryDegradeReason::CheckoutPlanRunsRepoOwnedActionOrHidesCost)
    });
    let form_incomplete_degrades = plans().any(|ex| {
        ex.degrade_reason == Some(M5CheckoutPlanEntryDegradeReason::PlanFormCoverageIncomplete)
    });
    let no_clean_missing_plan =
        !plans().any(|ex| ex.is_clean() && !ex.provides_complete_checkout_plan);
    if !(plan_modes_covered
        && implicit_degrades
        && form_incomplete_degrades
        && no_clean_missing_plan)
    {
        violations
            .push(M5SourceLocatorCheckoutPlanRegistriesViolation::CheckoutPlanIntegrityNotProven);
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

/// The repository-bootstrap families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5RepositoryBootstrapFamily; 3] = [
    M5RepositoryBootstrapFamily::OpenLocal,
    M5RepositoryBootstrapFamily::CloneRemote,
    M5RepositoryBootstrapFamily::OpenArchive,
];
