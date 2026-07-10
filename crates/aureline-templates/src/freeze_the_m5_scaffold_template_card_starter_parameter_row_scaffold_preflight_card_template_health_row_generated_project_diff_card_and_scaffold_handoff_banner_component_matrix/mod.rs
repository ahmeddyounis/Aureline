//! Frozen M5 scaffold-template-card, starter-parameter-row, scaffold-preflight-card,
//! template-health-row, generated-project-diff-card, and scaffold-handoff-banner component
//! matrix.
//!
//! This module locks Aureline's reusable scaffold / project-entry components into one
//! export-safe packet. Every starter-generation and project-entry subcomponent M5 claims that
//! still drifts too easily by start-center, template-gallery, preflight, generation-diff, or
//! workspace-handoff surface — the scaffold template card, the starter parameter row, the
//! scaffold preflight card, the template health row, the generated-project diff card, and the
//! scaffold handoff banner — is named once here and constrained by the same starter source
//! class, support class, host boundary, parameter source layer, immediate-versus-deferred
//! action timing, file / dependency / task / extension impact, health freshness,
//! generated-versus-user-owned boundary, and delete-generated or continue-without-starter
//! recovery language regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families; the one controlled disposition vocabulary every consumer binds
//! (`first_party`, `team_managed`, `community`, `local_only`, `create_empty`,
//! `continue_without_starter`, `blocked`, `warning`, `optional`); the starter source classes
//! and template support classes the template card binds; the parameter source layers and
//! action timings the parameter row binds; the preflight check classes and result states the
//! preflight card binds; the health signal classes and freshness states the template health
//! row binds; the generated-zone classes and diff-review states the generated-project diff
//! card binds; the handoff outcome classes and recovery actions the scaffold handoff banner
//! binds; the deployment lines every component must survive; the non-visual accessibility
//! routes; and the mandatory labels every component must be able to show. It does not
//! re-architect the signed template registry, scaffold planner, framework-pack, generation
//! diff / recovery, or project-entry contracts that already own those records — it is the
//! shared scaffold-component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 start-center, gallery,
//! preflight, diff-review, or handoff surface may publish a scaffold template card, a starter
//! parameter row, a scaffold preflight card, a template health row, a generated-project diff
//! card, or a scaffold handoff banner. Every consumer reads this packet so one template card
//! names where a starter came from and how it is supported, one parameter row names where a
//! value came from and whether it is applied immediately or deferred, one preflight card names
//! which checks are current and never hides a network, dependency-install, remote-provisioning,
//! trust, or managed-workspace side effect behind a generic Create, one health row names its
//! signal and freshness, one generated-project diff card names what is generated versus
//! user-owned and never blurs the boundary, and one handoff banner keeps Continue without
//! starter, Create empty, and delete-generated recovery paths explicit. No M5 lane invents a
//! second scaffold grammar or an alternate label for a governed source, support, timing,
//! ownership, or recovery state.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5ScaffoldComponentVocabularySet`] rather than minted per surface. Raw file bodies, raw
//! diffs, raw local paths, repository URLs, credentials, and secrets stay outside the export
//! boundary.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ScaffoldComponentMatrixPacket`].
pub const M5_SCAFFOLD_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix";

/// Schema version for M5 scaffold component-matrix records.
pub const M5_SCAFFOLD_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined scaffold-component boundary schema.
pub const M5_SCAFFOLD_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SCAFFOLD_COMPONENT_DOC_REF: &str = "docs/templates/m5_scaffold_component_matrix.md";

/// Repo-relative path of the per-component scaffold-template-card schema.
pub const M5_SCAFFOLD_TEMPLATE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-template-card.schema.json";

/// Repo-relative path of the per-component starter-parameter-row schema.
pub const M5_STARTER_PARAMETER_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-starter-parameter-row.schema.json";

/// Repo-relative path of the per-component scaffold-preflight-card schema.
pub const M5_SCAFFOLD_PREFLIGHT_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-preflight-card.schema.json";

/// Repo-relative path of the per-component template-health-row schema.
pub const M5_TEMPLATE_HEALTH_ROW_SCHEMA_REF: &str = "schemas/ui/m5-template-health-row.schema.json";

/// Repo-relative path of the per-component generated-project-diff-card schema.
pub const M5_GENERATED_PROJECT_DIFF_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-generated-project-diff-card.schema.json";

/// Repo-relative path of the per-component scaffold-handoff-banner schema.
pub const M5_SCAFFOLD_HANDOFF_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-scaffold-handoff-banner.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SCAFFOLD_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-scaffold-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SCAFFOLD_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-scaffold-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SCAFFOLD_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-scaffold-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SCAFFOLD_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-scaffold-component-matrix.md";

/// One of the six governed scaffold-component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldComponentFamily {
    /// A scaffold template card carrying its starter source class and support class.
    ScaffoldTemplateCard,
    /// A starter parameter row carrying its parameter source layer and action timing.
    StarterParameterRow,
    /// A scaffold preflight card carrying its check classes and result states.
    ScaffoldPreflightCard,
    /// A template health row carrying its health signal class and freshness state.
    TemplateHealthRow,
    /// A generated-project diff card carrying its generated-zone class and diff-review state.
    GeneratedProjectDiffCard,
    /// A scaffold handoff banner carrying its handoff outcome class and recovery action.
    ScaffoldHandoffBanner,
}

impl M5ScaffoldComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ScaffoldTemplateCard,
        Self::StarterParameterRow,
        Self::ScaffoldPreflightCard,
        Self::TemplateHealthRow,
        Self::GeneratedProjectDiffCard,
        Self::ScaffoldHandoffBanner,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScaffoldTemplateCard => "scaffold_template_card",
            Self::StarterParameterRow => "starter_parameter_row",
            Self::ScaffoldPreflightCard => "scaffold_preflight_card",
            Self::TemplateHealthRow => "template_health_row",
            Self::GeneratedProjectDiffCard => "generated_project_diff_card",
            Self::ScaffoldHandoffBanner => "scaffold_handoff_banner",
        }
    }

    /// `true` when this family is a scaffold template card and must therefore declare its
    /// starter source classes and support classes.
    pub const fn is_scaffold_template_card(self) -> bool {
        matches!(self, Self::ScaffoldTemplateCard)
    }

    /// `true` when this family is a starter parameter row and must therefore declare its
    /// parameter source layers and action timings.
    pub const fn is_starter_parameter_row(self) -> bool {
        matches!(self, Self::StarterParameterRow)
    }

    /// `true` when this family is a scaffold preflight card and must therefore declare its
    /// preflight check classes and result states.
    pub const fn is_scaffold_preflight_card(self) -> bool {
        matches!(self, Self::ScaffoldPreflightCard)
    }

    /// `true` when this family is a template health row and must therefore declare its health
    /// signal classes and freshness states.
    pub const fn is_template_health_row(self) -> bool {
        matches!(self, Self::TemplateHealthRow)
    }

    /// `true` when this family is a generated-project diff card and must therefore declare its
    /// generated-zone classes and diff-review states.
    pub const fn is_generated_project_diff_card(self) -> bool {
        matches!(self, Self::GeneratedProjectDiffCard)
    }

    /// `true` when this family is a scaffold handoff banner and must therefore declare its
    /// handoff outcome classes and recovery actions.
    pub const fn is_scaffold_handoff_banner(self) -> bool {
        matches!(self, Self::ScaffoldHandoffBanner)
    }
}

/// The one controlled disposition vocabulary every scaffold-component consumer binds. These
/// are the exact acceptance-criteria labels so no surface invents a parallel word for a
/// first-party, team-managed, community, or local-only starter, for a create-empty or
/// continue-without-starter recovery path, or for a blocked, warning, or optional state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldDisposition {
    /// A first-party starter.
    FirstParty,
    /// A team-managed starter.
    TeamManaged,
    /// A community starter.
    Community,
    /// A local-only starter.
    LocalOnly,
    /// Create empty (no starter writes files).
    CreateEmpty,
    /// Continue without starter.
    ContinueWithoutStarter,
    /// Blocked.
    Blocked,
    /// Warning.
    Warning,
    /// Optional.
    Optional,
}

impl M5ScaffoldDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::FirstParty,
        Self::TeamManaged,
        Self::Community,
        Self::LocalOnly,
        Self::CreateEmpty,
        Self::ContinueWithoutStarter,
        Self::Blocked,
        Self::Warning,
        Self::Optional,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::TeamManaged => "team_managed",
            Self::Community => "community",
            Self::LocalOnly => "local_only",
            Self::CreateEmpty => "create_empty",
            Self::ContinueWithoutStarter => "continue_without_starter",
            Self::Blocked => "blocked",
            Self::Warning => "warning",
            Self::Optional => "optional",
        }
    }
}

/// Controlled starter source class — where a scaffold template card's starter comes from, so a
/// card never leaves its first-party / team-managed / community / local origin implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StarterSourceClass {
    /// A first-party starter shipped by Aureline.
    FirstPartyStarter,
    /// A team-managed starter from a governed registry.
    TeamManagedStarter,
    /// A community starter.
    CommunityStarter,
    /// A local-only starter on this machine.
    LocalOnlyStarter,
    /// A mirrored / offline starter.
    MirroredStarter,
    /// A starter of unknown source.
    UnknownSourceStarter,
}

impl M5StarterSourceClass {
    /// Every starter source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstPartyStarter,
        Self::TeamManagedStarter,
        Self::CommunityStarter,
        Self::LocalOnlyStarter,
        Self::MirroredStarter,
        Self::UnknownSourceStarter,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyStarter => "first_party_starter",
            Self::TeamManagedStarter => "team_managed_starter",
            Self::CommunityStarter => "community_starter",
            Self::LocalOnlyStarter => "local_only_starter",
            Self::MirroredStarter => "mirrored_starter",
            Self::UnknownSourceStarter => "unknown_source_starter",
        }
    }
}

/// Controlled template support class — how a scaffold template card's starter is supported, so
/// bridge or heuristic behavior never reads as exact first-party support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TemplateSupportClass {
    /// Officially supported.
    OfficiallySupported,
    /// Community-supported, best effort.
    CommunitySupported,
    /// Experimental.
    Experimental,
    /// Bridge behavior, not exact first-party generation.
    BridgeBehavior,
    /// Deprecated.
    Deprecated,
    /// Unsupported.
    Unsupported,
}

impl M5TemplateSupportClass {
    /// Every template support class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OfficiallySupported,
        Self::CommunitySupported,
        Self::Experimental,
        Self::BridgeBehavior,
        Self::Deprecated,
        Self::Unsupported,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficiallySupported => "officially_supported",
            Self::CommunitySupported => "community_supported",
            Self::Experimental => "experimental",
            Self::BridgeBehavior => "bridge_behavior",
            Self::Deprecated => "deprecated",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Controlled parameter source layer — where a starter parameter row's value comes from, so a
/// row never leaves whether a value is a default, user-provided, or derived implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ParameterSourceLayer {
    /// A default value from the starter.
    DefaultValue,
    /// A user-provided value.
    UserProvided,
    /// A value inherited from a profile.
    ProfileInherited,
    /// A value derived from the environment.
    EnvironmentDerived,
    /// A computed / derived value.
    ComputedDerived,
    /// An unset required value.
    UnsetRequired,
}

impl M5ParameterSourceLayer {
    /// Every parameter source layer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DefaultValue,
        Self::UserProvided,
        Self::ProfileInherited,
        Self::EnvironmentDerived,
        Self::ComputedDerived,
        Self::UnsetRequired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultValue => "default_value",
            Self::UserProvided => "user_provided",
            Self::ProfileInherited => "profile_inherited",
            Self::EnvironmentDerived => "environment_derived",
            Self::ComputedDerived => "computed_derived",
            Self::UnsetRequired => "unset_required",
        }
    }
}

/// Controlled parameter action timing — whether a starter parameter row's action runs
/// immediately or is deferred, so a row never leaves the immediate-versus-deferred boundary
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ParameterActionTiming {
    /// Applied immediately.
    AppliedImmediately,
    /// Deferred until after create.
    DeferredAfterCreate,
    /// Requires explicit confirmation.
    RequiresConfirmation,
    /// Blocked because the value is invalid.
    BlockedInvalid,
    /// Optional and skippable.
    OptionalSkippable,
    /// Not applicable.
    NotApplicable,
}

impl M5ParameterActionTiming {
    /// Every parameter action timing, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AppliedImmediately,
        Self::DeferredAfterCreate,
        Self::RequiresConfirmation,
        Self::BlockedInvalid,
        Self::OptionalSkippable,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppliedImmediately => "applied_immediately",
            Self::DeferredAfterCreate => "deferred_after_create",
            Self::RequiresConfirmation => "requires_confirmation",
            Self::BlockedInvalid => "blocked_invalid",
            Self::OptionalSkippable => "optional_skippable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Controlled preflight check class — what a scaffold preflight card checks before a starter
/// writes files, so a generic Create never hides a network, dependency, host-boundary, or
/// credential side effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PreflightCheckClass {
    /// Required tooling is present.
    ToolingPresent,
    /// Dependency availability / install reach.
    DependencyAvailability,
    /// Network access needed by the starter.
    NetworkAccess,
    /// Whether the target workspace is writable.
    WorkspaceWritable,
    /// The host / managed-workspace boundary the starter runs against.
    HostBoundary,
    /// The credential scope the starter requires.
    CredentialScope,
}

impl M5PreflightCheckClass {
    /// Every preflight check class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ToolingPresent,
        Self::DependencyAvailability,
        Self::NetworkAccess,
        Self::WorkspaceWritable,
        Self::HostBoundary,
        Self::CredentialScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToolingPresent => "tooling_present",
            Self::DependencyAvailability => "dependency_availability",
            Self::NetworkAccess => "network_access",
            Self::WorkspaceWritable => "workspace_writable",
            Self::HostBoundary => "host_boundary",
            Self::CredentialScope => "credential_scope",
        }
    }
}

/// Controlled preflight result state — the outcome of a scaffold preflight check, so a card
/// never hides a blocked or skipped check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PreflightResultState {
    /// Passed.
    Passed,
    /// Warning.
    Warning,
    /// Blocked.
    Blocked,
    /// Skipped because optional.
    SkippedOptional,
    /// Not run.
    NotRun,
    /// Unknown.
    Unknown,
}

impl M5PreflightResultState {
    /// Every preflight result state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Passed,
        Self::Warning,
        Self::Blocked,
        Self::SkippedOptional,
        Self::NotRun,
        Self::Unknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Warning => "warning",
            Self::Blocked => "blocked",
            Self::SkippedOptional => "skipped_optional",
            Self::NotRun => "not_run",
            Self::Unknown => "unknown",
        }
    }
}

/// Controlled health signal class — which facet of template health a template health row
/// reports, so a row never leaves what it is asserting implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HealthSignalClass {
    /// Build health.
    BuildHealth,
    /// Dependency freshness.
    DependencyFreshness,
    /// Security advisories.
    SecurityAdvisories,
    /// Test status.
    TestStatus,
    /// Maintenance cadence.
    MaintenanceCadence,
    /// Compatibility.
    Compatibility,
}

impl M5HealthSignalClass {
    /// Every health signal class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BuildHealth,
        Self::DependencyFreshness,
        Self::SecurityAdvisories,
        Self::TestStatus,
        Self::MaintenanceCadence,
        Self::Compatibility,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildHealth => "build_health",
            Self::DependencyFreshness => "dependency_freshness",
            Self::SecurityAdvisories => "security_advisories",
            Self::TestStatus => "test_status",
            Self::MaintenanceCadence => "maintenance_cadence",
            Self::Compatibility => "compatibility",
        }
    }
}

/// Controlled health freshness state — how current a template health signal is, so a row never
/// presents a stale or never-checked signal as fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HealthFreshnessState {
    /// Fresh.
    Fresh,
    /// Aging.
    Aging,
    /// Stale.
    Stale,
    /// Expired.
    Expired,
    /// Never checked.
    NeverChecked,
    /// Unavailable.
    Unavailable,
}

impl M5HealthFreshnessState {
    /// Every health freshness state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Fresh,
        Self::Aging,
        Self::Stale,
        Self::Expired,
        Self::NeverChecked,
        Self::Unavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::NeverChecked => "never_checked",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Controlled generated-zone class — whether a generated-project diff card's scope is
/// generated or user-owned, so a card never blurs the generated-versus-user-owned boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeneratedZoneClass {
    /// Generated only.
    GeneratedOnly,
    /// User-owned.
    UserOwned,
    /// Generated and then hand-edited.
    GeneratedThenEdited,
    /// Runtime-only (caches, build output).
    RuntimeOnly,
    /// A mixed generated / user-owned zone.
    MixedZone,
    /// Zone unknown; review required.
    ZoneUnknown,
}

impl M5GeneratedZoneClass {
    /// Every generated-zone class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GeneratedOnly,
        Self::UserOwned,
        Self::GeneratedThenEdited,
        Self::RuntimeOnly,
        Self::MixedZone,
        Self::ZoneUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratedOnly => "generated_only",
            Self::UserOwned => "user_owned",
            Self::GeneratedThenEdited => "generated_then_edited",
            Self::RuntimeOnly => "runtime_only",
            Self::MixedZone => "mixed_zone",
            Self::ZoneUnknown => "zone_unknown",
        }
    }
}

/// Controlled diff-review state — what a generated-project diff card permits, so no write
/// happens silently before review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffReviewState {
    /// A reviewable diff preview is ready.
    PreviewReady,
    /// Review is required before any write.
    ReviewRequired,
    /// No changes; nothing to review.
    NoChanges,
    /// A conflict was detected.
    ConflictDetected,
    /// The diff could not be computed.
    DiffUnavailable,
    /// Blocked.
    Blocked,
}

impl M5DiffReviewState {
    /// Every diff-review state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreviewReady,
        Self::ReviewRequired,
        Self::NoChanges,
        Self::ConflictDetected,
        Self::DiffUnavailable,
        Self::Blocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewReady => "preview_ready",
            Self::ReviewRequired => "review_required",
            Self::NoChanges => "no_changes",
            Self::ConflictDetected => "conflict_detected",
            Self::DiffUnavailable => "diff_unavailable",
            Self::Blocked => "blocked",
        }
    }
}

/// Controlled handoff outcome class — how a scaffold handoff banner reports the result of a
/// bootstrap, so a partial or failed bootstrap is never presented as a clean create.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffOutcomeClass {
    /// Create succeeded.
    CreateSucceeded,
    /// A partial bootstrap.
    PartialBootstrap,
    /// Create failed.
    CreateFailed,
    /// Continued without a starter.
    ContinuedWithoutStarter,
    /// Created empty.
    CreatedEmpty,
    /// Remote provisioning is pending.
    ProvisioningPending,
}

impl M5HandoffOutcomeClass {
    /// Every handoff outcome class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CreateSucceeded,
        Self::PartialBootstrap,
        Self::CreateFailed,
        Self::ContinuedWithoutStarter,
        Self::CreatedEmpty,
        Self::ProvisioningPending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreateSucceeded => "create_succeeded",
            Self::PartialBootstrap => "partial_bootstrap",
            Self::CreateFailed => "create_failed",
            Self::ContinuedWithoutStarter => "continued_without_starter",
            Self::CreatedEmpty => "created_empty",
            Self::ProvisioningPending => "provisioning_pending",
        }
    }
}

/// Controlled handoff recovery action — the recovery path a scaffold handoff banner keeps
/// explicit, so delete-generated and continue-without-starter recovery are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffRecoveryAction {
    /// Open the new workspace.
    OpenWorkspace,
    /// Retry the bootstrap.
    RetryBootstrap,
    /// Delete the generated output.
    DeleteGenerated,
    /// Continue without the starter.
    ContinueWithoutStarter,
    /// Keep the partial output for review.
    KeepPartialReview,
    /// No recovery is needed.
    NoRecoveryNeeded,
}

impl M5HandoffRecoveryAction {
    /// Every handoff recovery action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenWorkspace,
        Self::RetryBootstrap,
        Self::DeleteGenerated,
        Self::ContinueWithoutStarter,
        Self::KeepPartialReview,
        Self::NoRecoveryNeeded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenWorkspace => "open_workspace",
            Self::RetryBootstrap => "retry_bootstrap",
            Self::DeleteGenerated => "delete_generated",
            Self::ContinueWithoutStarter => "continue_without_starter",
            Self::KeepPartialReview => "keep_partial_review",
            Self::NoRecoveryNeeded => "no_recovery_needed",
        }
    }
}

/// Claimed M5 project-entry / starter-generation surface family that renders / consumes a
/// scaffold component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldSurfaceFamily {
    /// The start-center surface.
    StartCenter,
    /// The template-gallery surface.
    TemplateGallery,
    /// The scaffold-preflight surface.
    ScaffoldPreflight,
    /// The generation diff-review surface.
    GenerationDiffReview,
    /// The workspace-handoff surface.
    WorkspaceHandoff,
    /// The CLI surface.
    CliSurface,
}

impl M5ScaffoldSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StartCenter,
        Self::TemplateGallery,
        Self::ScaffoldPreflight,
        Self::GenerationDiffReview,
        Self::WorkspaceHandoff,
        Self::CliSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::TemplateGallery => "template_gallery",
            Self::ScaffoldPreflight => "scaffold_preflight",
            Self::GenerationDiffReview => "generation_diff_review",
            Self::WorkspaceHandoff => "workspace_handoff",
            Self::CliSurface => "cli_surface",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's source,
/// support, side-effect, ownership, or recovery truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5ScaffoldDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldConsumerSurface {
    /// The start-center UI.
    StartCenterUi,
    /// The template-gallery UI.
    TemplateGalleryUi,
    /// The parameter-form UI.
    ParameterFormUi,
    /// The preflight UI.
    PreflightUi,
    /// The diff-review UI.
    DiffReviewUi,
    /// The workspace UI.
    WorkspaceUi,
    /// The template-health dashboard UI.
    HealthDashboardUi,
    /// The CLI surface.
    CliSurface,
    /// The support export.
    SupportExport,
}

impl M5ScaffoldConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::StartCenterUi,
        Self::TemplateGalleryUi,
        Self::ParameterFormUi,
        Self::PreflightUi,
        Self::DiffReviewUi,
        Self::WorkspaceUi,
        Self::HealthDashboardUi,
        Self::CliSurface,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenterUi => "start_center_ui",
            Self::TemplateGalleryUi => "template_gallery_ui",
            Self::ParameterFormUi => "parameter_form_ui",
            Self::PreflightUi => "preflight_ui",
            Self::DiffReviewUi => "diff_review_ui",
            Self::WorkspaceUi => "workspace_ui",
            Self::HealthDashboardUi => "health_dashboard_ui",
            Self::CliSurface => "cli_surface",
            Self::SupportExport => "support_export",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no scaffold truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5ScaffoldAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed scaffold component must be able to show. The first three are hard
/// requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about starter source / support, side-effect disclosure, and recovery / ownership
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The starter source class and support class behind the component.
    StarterSourceAndSupport,
    /// The network / dependency / provisioning / trust side effects the component discloses.
    SideEffectDisclosure,
    /// The generated-versus-user-owned boundary and the recovery path the component keeps.
    RecoveryAndOwnershipBoundary,
}

impl M5ScaffoldRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::StarterSourceAndSupport,
        Self::SideEffectDisclosure,
        Self::RecoveryAndOwnershipBoundary,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::StarterSourceAndSupport => "starter_source_and_support",
            Self::SideEffectDisclosure => "side_effect_disclosure",
            Self::RecoveryAndOwnershipBoundary => "recovery_and_ownership_boundary",
        }
    }
}

/// Qualification class for an M5 scaffold-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5ScaffoldQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a scaffold component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScaffoldDowngradeTrigger {
    /// A template card left its starter source unstated.
    StarterSourceUnstated,
    /// A template card left its support class unstated.
    SupportClassUnstated,
    /// A component hid a network / dependency / provisioning / trust side effect.
    SideEffectUndisclosed,
    /// A preflight card left its host / managed-workspace boundary unstated.
    HostBoundaryUnstated,
    /// A parameter row left its parameter source layer unstated.
    ParameterSourceUnstated,
    /// A parameter row left its immediate-versus-deferred action timing unstated.
    ActionTimingUnstated,
    /// A component left its file / dependency / task / extension impact undisclosed.
    ImpactUndisclosed,
    /// A template health row's freshness went stale.
    HealthFreshnessStale,
    /// A diff card blurred the generated-versus-user-owned boundary.
    GeneratedBoundaryBlurred,
    /// A handoff banner omitted its recovery or continue-without-starter path.
    RecoveryPathOmitted,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5ScaffoldDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::StarterSourceUnstated,
        Self::SupportClassUnstated,
        Self::SideEffectUndisclosed,
        Self::HostBoundaryUnstated,
        Self::ParameterSourceUnstated,
        Self::ActionTimingUnstated,
        Self::ImpactUndisclosed,
        Self::HealthFreshnessStale,
        Self::GeneratedBoundaryBlurred,
        Self::RecoveryPathOmitted,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StarterSourceUnstated => "starter_source_unstated",
            Self::SupportClassUnstated => "support_class_unstated",
            Self::SideEffectUndisclosed => "side_effect_undisclosed",
            Self::HostBoundaryUnstated => "host_boundary_unstated",
            Self::ParameterSourceUnstated => "parameter_source_unstated",
            Self::ActionTimingUnstated => "action_timing_unstated",
            Self::ImpactUndisclosed => "impact_undisclosed",
            Self::HealthFreshnessStale => "health_freshness_stale",
            Self::GeneratedBoundaryBlurred => "generated_boundary_blurred",
            Self::RecoveryPathOmitted => "recovery_path_omitted",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed scaffold-component family bound to the surface-specific
/// truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentRow {
    /// Governed component family.
    pub component_family: M5ScaffoldComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5ScaffoldQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 project-entry / starter-generation surface families that render / consume
    /// this component.
    pub surface_families: Vec<M5ScaffoldSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5ScaffoldDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5ScaffoldRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ScaffoldRequiredLabel>,
    /// Controlled dispositions this component binds (must be non-empty; drawn from the one
    /// shared [`M5ScaffoldDisposition`] vocabulary).
    pub dispositions: Vec<M5ScaffoldDisposition>,
    /// Starter source classes this component names (scaffold-template-card only).
    pub starter_source_classes: Vec<M5StarterSourceClass>,
    /// Template support classes this component names (scaffold-template-card only).
    pub template_support_classes: Vec<M5TemplateSupportClass>,
    /// Parameter source layers this component names (starter-parameter-row only).
    pub parameter_source_layers: Vec<M5ParameterSourceLayer>,
    /// Parameter action timings this component names (starter-parameter-row only).
    pub parameter_action_timings: Vec<M5ParameterActionTiming>,
    /// Preflight check classes this component names (scaffold-preflight-card only).
    pub preflight_check_classes: Vec<M5PreflightCheckClass>,
    /// Preflight result states this component names (scaffold-preflight-card only).
    pub preflight_result_states: Vec<M5PreflightResultState>,
    /// Health signal classes this component names (template-health-row only).
    pub health_signal_classes: Vec<M5HealthSignalClass>,
    /// Health freshness states this component names (template-health-row only).
    pub health_freshness_states: Vec<M5HealthFreshnessState>,
    /// Generated-zone classes this component names (generated-project-diff-card only).
    pub generated_zone_classes: Vec<M5GeneratedZoneClass>,
    /// Diff-review states this component names (generated-project-diff-card only).
    pub diff_review_states: Vec<M5DiffReviewState>,
    /// Handoff outcome classes this component names (scaffold-handoff-banner only).
    pub handoff_outcome_classes: Vec<M5HandoffOutcomeClass>,
    /// Handoff recovery actions this component names (scaffold-handoff-banner only).
    pub handoff_recovery_actions: Vec<M5HandoffRecoveryAction>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5ScaffoldAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ScaffoldConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5ScaffoldDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never hides its starter source or support class. MUST be
    /// `false`.
    pub hides_starter_source_or_support_class: bool,
    /// Hard invariant: this component never hides a network / dependency / provisioning /
    /// trust side effect behind a generic Create. MUST be `false`.
    pub hides_side_effect_behind_generic_create: bool,
    /// Hard invariant: this component never blurs the generated-versus-user-owned boundary.
    /// MUST be `false`.
    pub hides_generated_versus_user_owned_boundary: bool,
    /// Hard invariant: this component never omits a delete-generated, continue-without-starter,
    /// or create-empty recovery path. MUST be `false`.
    pub omits_recovery_or_continue_without_starter_path: bool,
    /// Hard invariant: this component never invents an alternate label for a governed state.
    /// MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl M5ScaffoldComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ScaffoldRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ScaffoldRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_starter_source_or_support_class
            && !self.hides_side_effect_behind_generic_create
            && !self.hides_generated_versus_user_owned_boundary
            && !self.omits_recovery_or_continue_without_starter_path
            && !self.invents_alternate_state_label
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Disposition tokens (the one shared consumer vocabulary).
    pub dispositions: Vec<String>,
    /// Starter-source-class tokens.
    pub starter_source_classes: Vec<String>,
    /// Template-support-class tokens.
    pub template_support_classes: Vec<String>,
    /// Parameter-source-layer tokens.
    pub parameter_source_layers: Vec<String>,
    /// Parameter-action-timing tokens.
    pub parameter_action_timings: Vec<String>,
    /// Preflight-check-class tokens.
    pub preflight_check_classes: Vec<String>,
    /// Preflight-result-state tokens.
    pub preflight_result_states: Vec<String>,
    /// Health-signal-class tokens.
    pub health_signal_classes: Vec<String>,
    /// Health-freshness-state tokens.
    pub health_freshness_states: Vec<String>,
    /// Generated-zone-class tokens.
    pub generated_zone_classes: Vec<String>,
    /// Diff-review-state tokens.
    pub diff_review_states: Vec<String>,
    /// Handoff-outcome-class tokens.
    pub handoff_outcome_classes: Vec<String>,
    /// Handoff-recovery-action tokens.
    pub handoff_recovery_actions: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5ScaffoldComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5ScaffoldComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5ScaffoldDisposition::ALL, |v| v.as_str()),
            starter_source_classes: tokens(&M5StarterSourceClass::ALL, |v| v.as_str()),
            template_support_classes: tokens(&M5TemplateSupportClass::ALL, |v| v.as_str()),
            parameter_source_layers: tokens(&M5ParameterSourceLayer::ALL, |v| v.as_str()),
            parameter_action_timings: tokens(&M5ParameterActionTiming::ALL, |v| v.as_str()),
            preflight_check_classes: tokens(&M5PreflightCheckClass::ALL, |v| v.as_str()),
            preflight_result_states: tokens(&M5PreflightResultState::ALL, |v| v.as_str()),
            health_signal_classes: tokens(&M5HealthSignalClass::ALL, |v| v.as_str()),
            health_freshness_states: tokens(&M5HealthFreshnessState::ALL, |v| v.as_str()),
            generated_zone_classes: tokens(&M5GeneratedZoneClass::ALL, |v| v.as_str()),
            diff_review_states: tokens(&M5DiffReviewState::ALL, |v| v.as_str()),
            handoff_outcome_classes: tokens(&M5HandoffOutcomeClass::ALL, |v| v.as_str()),
            handoff_recovery_actions: tokens(&M5HandoffRecoveryAction::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ScaffoldSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ScaffoldDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ScaffoldConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ScaffoldAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ScaffoldRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5ScaffoldComponentGovernanceReview {
    /// The scaffold template card shows its starter source and support class.
    pub template_card_shows_source_and_support: bool,
    /// The starter parameter row shows its source layer and action timing.
    pub parameter_row_shows_source_and_timing: bool,
    /// The scaffold preflight card shows its checks and their side effects.
    pub preflight_card_shows_checks_and_side_effects: bool,
    /// The template health row shows its signal and freshness.
    pub template_health_row_shows_signal_and_freshness: bool,
    /// The generated-project diff card shows the generated-versus-user-owned boundary.
    pub generated_diff_card_shows_generated_versus_user_owned: bool,
    /// The scaffold handoff banner shows its outcome and recovery.
    pub handoff_banner_shows_outcome_and_recovery: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// No generic Create hides a network / dependency / provisioning / trust side effect.
    pub no_generic_create_hides_side_effects: bool,
    /// The generated-versus-user-owned boundary stays explicit.
    pub generated_versus_user_owned_always_explicit: bool,
    /// Continue without starter and Create empty are always offered.
    pub continue_without_starter_and_create_empty_always_offered: bool,
    /// The delete-generated recovery path stays explicit.
    pub delete_generated_recovery_always_explicit: bool,
    /// The host boundary and trust source stay visible.
    pub host_boundary_and_trust_always_visible: bool,
    /// The starter source and support class stay explicit.
    pub starter_source_and_support_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel scaffold vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentConsumerProjection {
    /// Start-center surfaces consume the template-card and parameter vocabulary.
    pub start_center_surfaces_consume_template_card_and_parameter_vocabulary: bool,
    /// Preflight surfaces consume the check and side-effect vocabulary.
    pub preflight_surfaces_consume_check_and_side_effect_vocabulary: bool,
    /// Diff surfaces consume the generated-boundary vocabulary.
    pub diff_surfaces_consume_generated_boundary_vocabulary: bool,
    /// Health surfaces consume the signal and freshness vocabulary.
    pub health_surfaces_consume_signal_and_freshness_vocabulary: bool,
    /// Handoff surfaces consume the outcome and recovery vocabulary.
    pub handoff_surfaces_consume_outcome_and_recovery_vocabulary: bool,
    /// Support / export reads a single canonical scaffold source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the scaffold-component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting scaffold-component audit for the lane.
    pub scaffold_component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ScaffoldComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ScaffoldComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ScaffoldComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ScaffoldComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ScaffoldComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ScaffoldComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ScaffoldComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ScaffoldComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 scaffold-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ScaffoldComponentMatrixPacket {
    /// Record kind; must equal [`M5_SCAFFOLD_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SCAFFOLD_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ScaffoldComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ScaffoldComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ScaffoldComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ScaffoldComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ScaffoldComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ScaffoldComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ScaffoldComponentMatrixPacket {
    /// Builds an M5 scaffold-component matrix packet from stable-lane input.
    pub fn new(input: M5ScaffoldComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_SCAFFOLD_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_SCAFFOLD_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 scaffold-component matrix invariants.
    pub fn validate(&self) -> Vec<M5ScaffoldComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SCAFFOLD_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5ScaffoldComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SCAFFOLD_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5ScaffoldComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ScaffoldComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 scaffold component matrix packet serializes"),
        ) {
            violations.push(M5ScaffoldComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 scaffold component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,dispositions,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.dispositions, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Scaffold-Template-Card, Starter-Parameter-Row, Scaffold-Preflight-Card, Template-Health-Row, Generated-Project-Diff-Card, and Scaffold-Handoff-Banner Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Dispositions: {}\n",
                row.dispositions
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 scaffold matrix export.
#[derive(Debug)]
pub enum M5ScaffoldComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ScaffoldComponentMatrixViolation>),
}

impl fmt::Display for M5ScaffoldComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 scaffold component matrix export parse failed: {error}"
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
                    "m5 scaffold component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ScaffoldComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5ScaffoldComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ScaffoldComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row declares no dispositions.
    DispositionsMissing,
    /// A scaffold-template-card component declares no starter source classes.
    StarterSourceClassMissing,
    /// A scaffold-template-card component declares no template support classes.
    TemplateSupportClassMissing,
    /// A starter-parameter-row component declares no parameter source layers.
    ParameterSourceLayerMissing,
    /// A starter-parameter-row component declares no parameter action timings.
    ParameterActionTimingMissing,
    /// A scaffold-preflight-card component declares no preflight check classes.
    PreflightCheckClassMissing,
    /// A scaffold-preflight-card component declares no preflight result states.
    PreflightResultStateMissing,
    /// A template-health-row component declares no health signal classes.
    HealthSignalClassMissing,
    /// A template-health-row component declares no health freshness states.
    HealthFreshnessStateMissing,
    /// A generated-project-diff-card component declares no generated-zone classes.
    GeneratedZoneClassMissing,
    /// A generated-project-diff-card component declares no diff-review states.
    DiffReviewStateMissing,
    /// A scaffold-handoff-banner component declares no handoff outcome classes.
    HandoffOutcomeClassMissing,
    /// A scaffold-handoff-banner component declares no handoff recovery actions.
    HandoffRecoveryActionMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (hidden starter source / support, side effect
    /// hidden behind a generic Create, blurred generated-versus-user-owned boundary, omitted
    /// recovery path, or invented alternate state label).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ScaffoldComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::StarterSourceClassMissing => "starter_source_class_missing",
            Self::TemplateSupportClassMissing => "template_support_class_missing",
            Self::ParameterSourceLayerMissing => "parameter_source_layer_missing",
            Self::ParameterActionTimingMissing => "parameter_action_timing_missing",
            Self::PreflightCheckClassMissing => "preflight_check_class_missing",
            Self::PreflightResultStateMissing => "preflight_result_state_missing",
            Self::HealthSignalClassMissing => "health_signal_class_missing",
            Self::HealthFreshnessStateMissing => "health_freshness_state_missing",
            Self::GeneratedZoneClassMissing => "generated_zone_class_missing",
            Self::DiffReviewStateMissing => "diff_review_state_missing",
            Self::HandoffOutcomeClassMissing => "handoff_outcome_class_missing",
            Self::HandoffRecoveryActionMissing => "handoff_recovery_action_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 scaffold matrix export.
///
/// This is the first real consumer of the scaffold-component lane: a start-center, gallery,
/// preflight, diff-review, handoff, or support-export surface calls it to ingest the canonical
/// matrix rather than cloning status text.
///
/// # Errors
///
/// Returns [`M5ScaffoldComponentMatrixArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_stable_m5_scaffold_component_matrix_export(
) -> Result<M5ScaffoldComponentMatrixPacket, M5ScaffoldComponentMatrixArtifactError> {
    let packet: M5ScaffoldComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-scaffold-component-proof/support_export.json"
    )))
    .map_err(M5ScaffoldComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ScaffoldComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ScaffoldComponentMatrixPacket,
    violations: &mut Vec<M5ScaffoldComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
        M5_SCAFFOLD_TEMPLATE_CARD_SCHEMA_REF,
        M5_STARTER_PARAMETER_ROW_SCHEMA_REF,
        M5_SCAFFOLD_PREFLIGHT_CARD_SCHEMA_REF,
        M5_TEMPLATE_HEALTH_ROW_SCHEMA_REF,
        M5_GENERATED_PROJECT_DIFF_CARD_SCHEMA_REF,
        M5_SCAFFOLD_HANDOFF_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ScaffoldComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ScaffoldComponentMatrixPacket,
    violations: &mut Vec<M5ScaffoldComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ScaffoldComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5ScaffoldComponentMatrixPacket,
    violations: &mut Vec<M5ScaffoldComponentMatrixViolation>,
) {
    let present: BTreeSet<M5ScaffoldComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5ScaffoldComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ScaffoldComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ScaffoldComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ScaffoldComponentMatrixViolation::MandatoryLabelMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::DispositionsMissing);
        }
        if family.is_scaffold_template_card() && row.starter_source_classes.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::StarterSourceClassMissing);
        }
        if family.is_scaffold_template_card() && row.template_support_classes.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::TemplateSupportClassMissing);
        }
        if family.is_starter_parameter_row() && row.parameter_source_layers.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::ParameterSourceLayerMissing);
        }
        if family.is_starter_parameter_row() && row.parameter_action_timings.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::ParameterActionTimingMissing);
        }
        if family.is_scaffold_preflight_card() && row.preflight_check_classes.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::PreflightCheckClassMissing);
        }
        if family.is_scaffold_preflight_card() && row.preflight_result_states.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::PreflightResultStateMissing);
        }
        if family.is_template_health_row() && row.health_signal_classes.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::HealthSignalClassMissing);
        }
        if family.is_template_health_row() && row.health_freshness_states.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::HealthFreshnessStateMissing);
        }
        if family.is_generated_project_diff_card() && row.generated_zone_classes.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::GeneratedZoneClassMissing);
        }
        if family.is_generated_project_diff_card() && row.diff_review_states.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::DiffReviewStateMissing);
        }
        if family.is_scaffold_handoff_banner() && row.handoff_outcome_classes.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::HandoffOutcomeClassMissing);
        }
        if family.is_scaffold_handoff_banner() && row.handoff_recovery_actions.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::HandoffRecoveryActionMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ScaffoldComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ScaffoldComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ScaffoldComponentMatrixPacket,
    violations: &mut Vec<M5ScaffoldComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.template_card_shows_source_and_support,
        review.parameter_row_shows_source_and_timing,
        review.preflight_card_shows_checks_and_side_effects,
        review.template_health_row_shows_signal_and_freshness,
        review.generated_diff_card_shows_generated_versus_user_owned,
        review.handoff_banner_shows_outcome_and_recovery,
        review.no_surface_invents_alternate_state_label,
        review.no_generic_create_hides_side_effects,
        review.generated_versus_user_owned_always_explicit,
        review.continue_without_starter_and_create_empty_always_offered,
        review.delete_generated_recovery_always_explicit,
        review.host_boundary_and_trust_always_visible,
        review.starter_source_and_support_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ScaffoldComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ScaffoldComponentMatrixPacket,
    violations: &mut Vec<M5ScaffoldComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.start_center_surfaces_consume_template_card_and_parameter_vocabulary,
        projection.preflight_surfaces_consume_check_and_side_effect_vocabulary,
        projection.diff_surfaces_consume_generated_boundary_vocabulary,
        projection.health_surfaces_consume_signal_and_freshness_vocabulary,
        projection.handoff_surfaces_consume_outcome_and_recovery_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ScaffoldComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ScaffoldComponentMatrixPacket,
    violations: &mut Vec<M5ScaffoldComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ScaffoldComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ScaffoldComponentMatrixPacket,
    violations: &mut Vec<M5ScaffoldComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.scaffold_component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ScaffoldComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Canonical seed builders
//
// These builders are the single producer of the checked-in support export and the narrowed
// fixtures. The headless emitter example and the inline tests both call them so the in-code
// matrix, the artifact, and the fixtures never drift.
// ---------------------------------------------------------------------------

/// Stable packet id for the canonical scaffold-component matrix.
pub const M5_SCAFFOLD_COMPONENT_MATRIX_PACKET_ID: &str = "m5-scaffold-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5ScaffoldRequiredLabel> {
    M5ScaffoldRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5ScaffoldRequiredLabel]) -> Vec<M5ScaffoldRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5ScaffoldComponentFamily,
    qualification: M5ScaffoldQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5ScaffoldComponentRow {
    M5ScaffoldComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ScaffoldSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ScaffoldDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        dispositions: vec![],
        starter_source_classes: vec![],
        template_support_classes: vec![],
        parameter_source_layers: vec![],
        parameter_action_timings: vec![],
        preflight_check_classes: vec![],
        preflight_result_states: vec![],
        health_signal_classes: vec![],
        health_freshness_states: vec![],
        generated_zone_classes: vec![],
        diff_review_states: vec![],
        handoff_outcome_classes: vec![],
        handoff_recovery_actions: vec![],
        accessibility_routes: M5ScaffoldAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ScaffoldConsumerSurface::StartCenterUi,
            M5ScaffoldConsumerSurface::SupportExport,
        ],
        downgrade_triggers: vec![M5ScaffoldDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        hides_starter_source_or_support_class: false,
        hides_side_effect_behind_generic_create: false,
        hides_generated_versus_user_owned_boundary: false,
        omits_recovery_or_continue_without_starter_path: false,
        invents_alternate_state_label: false,
    }
}

fn component_rows() -> Vec<M5ScaffoldComponentRow> {
    use M5ScaffoldComponentFamily as F;
    use M5ScaffoldConsumerSurface as C;
    use M5ScaffoldDisposition as DI;
    use M5ScaffoldDowngradeTrigger as D;
    use M5ScaffoldQualificationClass as Q;
    use M5ScaffoldRequiredLabel as L;

    let mut rows = Vec::new();

    // 1. Scaffold template card.
    let mut row = base_row(
        F::ScaffoldTemplateCard,
        Q::Stable,
        "Scaffold template card owner",
        "One scaffold-template-card model naming where a starter comes from (a first-party starter, a team-managed starter, a community starter, a local-only starter, a mirrored starter, or an unknown source) and how it is supported (officially supported, community supported, experimental, bridge behavior, deprecated, or unsupported), so a card never leaves its starter source or support class implicit and never presents bridge or heuristic behavior as exact first-party support",
        "evidence:m5-scaffold-template-card-parity:001",
        &[M5_SCAFFOLD_COMPONENT_SCHEMA_REF, M5_SCAFFOLD_TEMPLATE_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![
        DI::FirstParty,
        DI::TeamManaged,
        DI::Community,
        DI::LocalOnly,
    ];
    row.starter_source_classes = M5StarterSourceClass::ALL.to_vec();
    row.template_support_classes = M5TemplateSupportClass::ALL.to_vec();
    row.required_labels = labels_with(&[L::StarterSourceAndSupport]);
    row.surface_families = vec![
        M5ScaffoldSurfaceFamily::StartCenter,
        M5ScaffoldSurfaceFamily::TemplateGallery,
        M5ScaffoldSurfaceFamily::CliSurface,
    ];
    row.consumer_surfaces = vec![C::StartCenterUi, C::TemplateGalleryUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::StarterSourceUnstated,
        D::SupportClassUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Starter parameter row.
    let mut row = base_row(
        F::StarterParameterRow,
        Q::Stable,
        "Starter parameter row owner",
        "One starter-parameter-row model naming where a parameter value comes from (a default value, a user-provided value, a profile-inherited value, an environment-derived value, a computed value, or an unset required value) and whether its action is applied immediately or deferred (applied immediately, deferred after create, requires confirmation, blocked because invalid, optional and skippable, or not applicable), so a row never leaves the parameter source layer or the immediate-versus-deferred boundary implicit",
        "evidence:m5-starter-parameter-row-parity:001",
        &[M5_SCAFFOLD_COMPONENT_SCHEMA_REF, M5_STARTER_PARAMETER_ROW_SCHEMA_REF],
    );
    row.dispositions = vec![DI::Optional, DI::Warning, DI::Blocked];
    row.parameter_source_layers = M5ParameterSourceLayer::ALL.to_vec();
    row.parameter_action_timings = M5ParameterActionTiming::ALL.to_vec();
    row.required_labels = labels_with(&[L::SideEffectDisclosure]);
    row.surface_families = vec![
        M5ScaffoldSurfaceFamily::StartCenter,
        M5ScaffoldSurfaceFamily::TemplateGallery,
        M5ScaffoldSurfaceFamily::CliSurface,
    ];
    row.consumer_surfaces = vec![C::ParameterFormUi, C::StartCenterUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::ParameterSourceUnstated,
        D::ActionTimingUnstated,
        D::SideEffectUndisclosed,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Scaffold preflight card.
    let mut row = base_row(
        F::ScaffoldPreflightCard,
        Q::Stable,
        "Scaffold preflight card owner",
        "One scaffold-preflight-card model naming what is checked before a starter writes files (required tooling present, dependency availability, network access, workspace writable, the host or managed-workspace boundary, or the credential scope) and each check's outcome (passed, warning, blocked, skipped because optional, not run, or unknown), so a generic Create never hides a network, dependency-install, remote-provisioning, trust, or managed-workspace side effect",
        "evidence:m5-scaffold-preflight-card-parity:001",
        &[M5_SCAFFOLD_COMPONENT_SCHEMA_REF, M5_SCAFFOLD_PREFLIGHT_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![DI::Blocked, DI::Warning, DI::Optional];
    row.preflight_check_classes = M5PreflightCheckClass::ALL.to_vec();
    row.preflight_result_states = M5PreflightResultState::ALL.to_vec();
    row.required_labels = labels_with(&[L::SideEffectDisclosure]);
    row.surface_families = vec![
        M5ScaffoldSurfaceFamily::ScaffoldPreflight,
        M5ScaffoldSurfaceFamily::StartCenter,
        M5ScaffoldSurfaceFamily::CliSurface,
    ];
    row.consumer_surfaces = vec![C::PreflightUi, C::StartCenterUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::SideEffectUndisclosed,
        D::HostBoundaryUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Template health row.
    let mut row = base_row(
        F::TemplateHealthRow,
        Q::Stable,
        "Template health row owner",
        "One template-health-row model naming which health facet it reports (build health, dependency freshness, security advisories, test status, maintenance cadence, or compatibility) and how current the signal is (fresh, aging, stale, expired, never checked, or unavailable), so a row never presents a stale or never-checked health signal as fresh and always names what it is asserting",
        "evidence:m5-template-health-row-parity:001",
        &[M5_SCAFFOLD_COMPONENT_SCHEMA_REF, M5_TEMPLATE_HEALTH_ROW_SCHEMA_REF],
    );
    row.dispositions = vec![DI::Warning, DI::Optional];
    row.health_signal_classes = M5HealthSignalClass::ALL.to_vec();
    row.health_freshness_states = M5HealthFreshnessState::ALL.to_vec();
    row.required_labels = labels_with(&[L::StarterSourceAndSupport]);
    row.surface_families = vec![
        M5ScaffoldSurfaceFamily::TemplateGallery,
        M5ScaffoldSurfaceFamily::StartCenter,
        M5ScaffoldSurfaceFamily::CliSurface,
    ];
    row.consumer_surfaces = vec![C::HealthDashboardUi, C::TemplateGalleryUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::HealthFreshnessStale,
        D::ImpactUndisclosed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Generated-project diff card.
    let mut row = base_row(
        F::GeneratedProjectDiffCard,
        Q::Stable,
        "Generated-project diff card owner",
        "One generated-project-diff-card model naming what a starter wrote versus what the user owns (generated only, user-owned, generated then hand-edited, runtime-only, a mixed zone, or zone unknown) and its diff-review state (a preview ready, review required before any write, no changes, a conflict detected, the diff unavailable, or blocked), so a card never blurs the generated-versus-user-owned boundary and never overwrites or deletes user-owned work silently",
        "evidence:m5-generated-project-diff-card-parity:001",
        &[M5_SCAFFOLD_COMPONENT_SCHEMA_REF, M5_GENERATED_PROJECT_DIFF_CARD_SCHEMA_REF],
    );
    row.dispositions = vec![DI::CreateEmpty, DI::ContinueWithoutStarter, DI::Blocked];
    row.generated_zone_classes = M5GeneratedZoneClass::ALL.to_vec();
    row.diff_review_states = M5DiffReviewState::ALL.to_vec();
    row.required_labels = labels_with(&[L::RecoveryAndOwnershipBoundary]);
    row.surface_families = vec![
        M5ScaffoldSurfaceFamily::GenerationDiffReview,
        M5ScaffoldSurfaceFamily::WorkspaceHandoff,
        M5ScaffoldSurfaceFamily::CliSurface,
    ];
    row.consumer_surfaces = vec![C::DiffReviewUi, C::WorkspaceUi, C::SupportExport];
    row.downgrade_triggers = vec![
        D::GeneratedBoundaryBlurred,
        D::RecoveryPathOmitted,
        D::ImpactUndisclosed,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Scaffold handoff banner.
    let mut row = base_row(
        F::ScaffoldHandoffBanner,
        Q::Stable,
        "Scaffold handoff banner owner",
        "One scaffold-handoff-banner model naming the bootstrap outcome (create succeeded, a partial bootstrap, create failed, continued without a starter, created empty, or remote provisioning pending) and the recovery path it keeps explicit (open the workspace, retry the bootstrap, delete the generated output, continue without the starter, keep the partial output for review, or no recovery needed), so a partial or failed bootstrap is never presented as a clean create and delete-generated or continue-without-starter recovery is never hidden",
        "evidence:m5-scaffold-handoff-banner-parity:001",
        &[M5_SCAFFOLD_COMPONENT_SCHEMA_REF, M5_SCAFFOLD_HANDOFF_BANNER_SCHEMA_REF],
    );
    row.dispositions = vec![DI::CreateEmpty, DI::ContinueWithoutStarter, DI::FirstParty];
    row.handoff_outcome_classes = M5HandoffOutcomeClass::ALL.to_vec();
    row.handoff_recovery_actions = M5HandoffRecoveryAction::ALL.to_vec();
    row.required_labels = labels_with(&[L::RecoveryAndOwnershipBoundary, L::SideEffectDisclosure]);
    row.surface_families = vec![
        M5ScaffoldSurfaceFamily::WorkspaceHandoff,
        M5ScaffoldSurfaceFamily::StartCenter,
        M5ScaffoldSurfaceFamily::CliSurface,
    ];
    row.consumer_surfaces = vec![
        C::WorkspaceUi,
        C::StartCenterUi,
        C::CliSurface,
        C::SupportExport,
    ];
    row.downgrade_triggers = vec![
        D::RecoveryPathOmitted,
        D::SideEffectUndisclosed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ScaffoldComponentGovernanceReview {
    M5ScaffoldComponentGovernanceReview {
        template_card_shows_source_and_support: true,
        parameter_row_shows_source_and_timing: true,
        preflight_card_shows_checks_and_side_effects: true,
        template_health_row_shows_signal_and_freshness: true,
        generated_diff_card_shows_generated_versus_user_owned: true,
        handoff_banner_shows_outcome_and_recovery: true,
        no_surface_invents_alternate_state_label: true,
        no_generic_create_hides_side_effects: true,
        generated_versus_user_owned_always_explicit: true,
        continue_without_starter_and_create_empty_always_offered: true,
        delete_generated_recovery_always_explicit: true,
        host_boundary_and_trust_always_visible: true,
        starter_source_and_support_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ScaffoldComponentConsumerProjection {
    M5ScaffoldComponentConsumerProjection {
        start_center_surfaces_consume_template_card_and_parameter_vocabulary: true,
        preflight_surfaces_consume_check_and_side_effect_vocabulary: true,
        diff_surfaces_consume_generated_boundary_vocabulary: true,
        health_surfaces_consume_signal_and_freshness_vocabulary: true,
        handoff_surfaces_consume_outcome_and_recovery_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ScaffoldComponentProofFreshness {
    M5ScaffoldComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ScaffoldComponentReleasePosture {
    M5ScaffoldComponentReleasePosture {
        proof_packet_ref: M5_SCAFFOLD_COMPONENT_ARTIFACT_REF.to_owned(),
        scaffold_component_audit_ref: M5_SCAFFOLD_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SCAFFOLD_COMPONENT_SCHEMA_REF,
        M5_SCAFFOLD_COMPONENT_DOC_REF,
        M5_SCAFFOLD_TEMPLATE_CARD_SCHEMA_REF,
        M5_STARTER_PARAMETER_ROW_SCHEMA_REF,
        M5_SCAFFOLD_PREFLIGHT_CARD_SCHEMA_REF,
        M5_TEMPLATE_HEALTH_ROW_SCHEMA_REF,
        M5_GENERATED_PROJECT_DIFF_CARD_SCHEMA_REF,
        M5_SCAFFOLD_HANDOFF_BANNER_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 scaffold-component matrix packet.
pub fn seeded_m5_scaffold_component_matrix() -> M5ScaffoldComponentMatrixPacket {
    M5ScaffoldComponentMatrixPacket::new(M5ScaffoldComponentMatrixPacketInput {
        packet_id: M5_SCAFFOLD_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 scaffold-template-card, starter-parameter-row, scaffold-preflight-card, template-health-row, generated-project-diff-card, and scaffold-handoff-banner component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5ScaffoldComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the scaffold preflight card is held at Beta because network, dependency,
/// and host-boundary checks are environment-dependent and parity evidence for a slice of the
/// side-effect disclosures does not yet round-trip across every preflight surface; every
/// component stays visible.
pub fn seeded_m5_scaffold_component_matrix_scaffold_preflight_card_beta_narrowed(
) -> M5ScaffoldComponentMatrixPacket {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.packet_id = "m5-scaffold-components:scaffold-preflight-card-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ScaffoldComponentFamily::ScaffoldPreflightCard)
        .expect("scaffold-preflight-card row present");
    row.qualification = M5ScaffoldQualificationClass::Beta;
    packet
}

/// Narrowed variant: the scaffold handoff banner is narrowed to Preview pending
/// remote-provisioning and delete-generated recovery parity proof across every handoff surface;
/// every component stays visible.
pub fn seeded_m5_scaffold_component_matrix_scaffold_handoff_banner_preview_narrowed(
) -> M5ScaffoldComponentMatrixPacket {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.packet_id = "m5-scaffold-components:scaffold-handoff-banner-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ScaffoldComponentFamily::ScaffoldHandoffBanner)
        .expect("scaffold-handoff-banner row present");
    row.qualification = M5ScaffoldQualificationClass::Preview;
    packet
}
