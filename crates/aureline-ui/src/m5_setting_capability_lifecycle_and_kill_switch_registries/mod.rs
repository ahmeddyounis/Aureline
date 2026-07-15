//! Implemented M5 capability-record and kill-switch-record registries.
//!
//! The frozen [settings-governance matrix][matrix] names Aureline's five configuration-runtime families and
//! locks their controlled vocabulary. This is the capability-lifecycle / kill-switch implement lane over the
//! `rollout_capability` family: it turns the *capability-record* grammar (how a capability record, Labs
//! enrollment, rollout plan, and dependency marker declare the lifecycle state, the accountable owner, the
//! scope, the review / expiry window, the enabled posture, the artifact dependency marker, the fallback, and the
//! rollback note a capability carries) and the *kill-switch-record* grammar (how a kill-switch or policy-disable
//! record names the disabling source, the disabled timestamp, the preserved user-authored data, the
//! self-explanation, the capability dependency, the fallback, and the last ledger revision for a kill-switch,
//! policy-disable, dependency-unavailable, review-expired, or manual-opt-out disable) into registry resolvers
//! that produce export-safe, honest projections. Every claimed M5 capability then resolves to one
//! capability-record object — the lifecycle state it classifies (Labs / Preview / Beta / generally-available /
//! graduated / deprecated), the owner, the scope, the review / expiry window, the enabled posture, the
//! dependency marker, the fallback, and the rollback note — and every claimed disable resolves to one
//! kill-switch-record object — the disabling source, the disabled timestamp, the preserved-data reference, the
//! explanation reference, the capability dependency, the fallback reference, and the last ledger revision — that
//! the settings, docs / help, bundle, import-apply, and support / export flows can inspect before a claim
//! publishes without manual reconstruction, so a stable-facing surface never depends on a hidden Labs / Preview
//! capability without an explicit dependency marker and fallback, a lifecycle or experiment dependency never
//! disappears behind unpublished markers, a kill switch or policy disable always preserves user-authored data
//! and explains its cause, and a lifecycle flow that cannot explain a capability state or a disable cause
//! degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one capability-record object per capability.** [`resolve_capability_record_entry`] refuses to read
//!   as a clean, registry-bound capability entry unless it names a canonical registry token, a classified
//!   [lifecycle class][M5CapabilityLifecycleClass], a settings-governance role, covers every
//!   [resolution form][M5ConfigCapabilityResolutionForm] (the canonical object, the accessible summary, and the
//!   audit record), publishes every record field (owner, scope, review / expiry, enabled posture, dependency
//!   marker, fallback, and rollback note), keeps its dependency marker published, and publishes a fallback before
//!   a protected (Labs / Preview / Beta) capability is claimed; otherwise it degrades.
//! * **Keep a capability from hiding its dependency behind unpublished markers or lacking a fallback.**
//!   [`capability_does_not_hide_dependency`] rejects a capability entry whose dependency marker is not published so
//!   it degrades to [`M5CapabilityRecordEntryDegradeReason::CapabilityHidesDependencyOrLacksFallback`], and a
//!   protected capability that has not published a fallback degrades the same way.
//! * **Keep the kill-switch ledger from hiding its cause or dropping user-data preservation.**
//!   [`resolve_kill_switch_record_entry`] names a classified [kill-switch class][M5KillSwitchClass], requires the
//!   full disabling-source / disabled-timestamp / preserved-data-reference / explanation-reference /
//!   capability-dependency / fallback-reference / last-ledger-revision kill-switch-record object, covers every
//!   resolution form, and degrades to
//!   [`M5KillSwitchRecordEntryDegradeReason::KillSwitchHidesCauseOrDropsDataPreservation`] when the record would
//!   hide a kill-switch / policy-disable cause without disclosing its reason or leave preserved user-authored data
//!   without disclosing that it is preserved, so a disable can never read as trustworthy when it has quietly
//!   dropped the cause it fired or the user data it still holds.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5SettingsGovernanceRole`] role vocabulary
//! and the [`M5SettingsGovernanceConsumerSurface`] consumer-surface taxonomy — so the settings, shell,
//! diagnostics, admin, sync, policy, capability, docs, CLI, and support surfaces can never fork their own
//! capability-lifecycle or kill-switch meaning. Raw secret values and private endpoints stay outside the export
//! boundary.
//!
//! [matrix]: crate::m5_settings_governance_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_setting_capability_lifecycle_and_kill_switch_registries,
    seeded_m5_setting_capability_lifecycle_and_kill_switch_registries_capability_lifecycle_beta_narrowed,
    seeded_m5_setting_capability_lifecycle_and_kill_switch_registries_kill_switch_preview_narrowed,
    M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_settings_governance_matrix::{
    M5SettingsGovernanceAccessibilityRoute, M5SettingsGovernanceConsumerSurface,
    M5SettingsGovernanceDeploymentLine, M5SettingsGovernanceDowngradeTrigger,
    M5SettingsGovernanceFamily, M5SettingsGovernanceQualificationClass,
    M5SettingsGovernanceRequiredLabel, M5SettingsGovernanceRole,
    M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF, M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF,
    M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF, M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SettingCapabilityLifecycleKillSwitchRegistriesPacket`].
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_setting_capability_lifecycle_and_kill_switch_registries";

/// Schema version for M5 capability-record / kill-switch-record registry records.
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_SCHEMA_REF: &str =
    "schemas/config/m5-setting-capability-lifecycle-and-kill-switch-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_DOC_REF: &str =
    "docs/settings/m5_setting_capability_lifecycle_and_kill_switch_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-setting-capability-lifecycle-and-kill-switch-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-setting-capability-lifecycle-and-kill-switch-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-setting-capability-lifecycle-and-kill-switch-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/config/m5-setting-capability-lifecycle-and-kill-switch-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5SettingCapabilityLifecycleKillSwitchRegistriesConsumerSurface =
    M5SettingsGovernanceConsumerSurface;

/// One of the three resolution forms every capability or kill-switch entry must hold across so its truth keeps
/// whether it is shown as the canonical resolved object, announced as an accessible summary, or written to the
/// audit / support record. Minted by this lane because the frozen matrix names the rollout-capability *family*
/// but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigCapabilityResolutionForm {
    /// The canonical resolved capability-record / kill-switch-record object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved capability discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved capability inspectable off-renderer.
    AuditRecord,
}

impl M5ConfigCapabilityResolutionForm {
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

/// Controlled capability-lifecycle state a capability-record entry declares, so the lifecycle model shares one
/// registry rather than scattering Labs / Preview / Stable labels across surfaces or hiding capability state
/// behind an ad-hoc flag. Minted by this lane because the frozen matrix carries the configuration families but
/// not the concrete Labs / Preview / Beta / generally-available / graduated / deprecated lifecycle state a
/// capability classifies against. Every classified state carries its canonical state mode, and the Labs,
/// Preview, and Beta states are pre-stable and so must publish an explicit dependency marker and fallback before
/// a stable-facing surface may depend on them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityLifecycleClass {
    /// The capability is an opt-in Labs experiment; a stable surface may depend on it only with a marker.
    Labs,
    /// The capability is in Preview; a stable surface may depend on it only with a marker and fallback.
    Preview,
    /// The capability is in Beta; a stable surface may depend on it only with a marker and fallback.
    Beta,
    /// The capability is generally available on the Stable channel.
    GenerallyAvailable,
    /// The capability has graduated from an experiment into a permanent capability.
    Graduated,
    /// The capability is deprecated and scheduled for removal.
    Deprecated,
    /// The lifecycle class is unclassified, which is disallowed.
    LifecycleClassUnclassified,
}

impl M5CapabilityLifecycleClass {
    /// Every lifecycle class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Labs,
        Self::Preview,
        Self::Beta,
        Self::GenerallyAvailable,
        Self::Graduated,
        Self::Deprecated,
        Self::LifecycleClassUnclassified,
    ];

    /// The six canonical lifecycle classes every claimed M5 capability classifies against.
    pub const CANONICAL_CLASSES: [Self; 6] = [
        Self::Labs,
        Self::Preview,
        Self::Beta,
        Self::GenerallyAvailable,
        Self::Graduated,
        Self::Deprecated,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Labs => "labs",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::GenerallyAvailable => "generally_available",
            Self::Graduated => "graduated",
            Self::Deprecated => "deprecated",
            Self::LifecycleClassUnclassified => "lifecycle_class_unclassified",
        }
    }

    /// Whether the class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::LifecycleClassUnclassified)
    }

    /// The canonical state mode for this lifecycle class.
    pub const fn canonical_state_mode(self) -> &'static str {
        match self {
            Self::Labs => "labs_capability",
            Self::Preview => "preview_capability",
            Self::Beta => "beta_capability",
            Self::GenerallyAvailable => "generally_available_capability",
            Self::Graduated => "graduated_capability",
            Self::Deprecated => "deprecated_capability",
            Self::LifecycleClassUnclassified => "",
        }
    }

    /// Whether this class is a pre-stable (Labs / Preview / Beta) capability and so must publish an explicit
    /// dependency marker and fallback before a stable-facing surface may depend on it.
    pub const fn requires_dependency_marker_and_fallback(self) -> bool {
        matches!(self, Self::Labs | Self::Preview | Self::Beta)
    }
}

/// Controlled kill-switch class a kill-switch-record entry must resolve, so a kill-switch / policy-disable /
/// dependency-unavailable / review-expired / manual-opt-out disable shares one registry rather than a hand-copied
/// per-record assumption. Minted by this lane, tracking the disable dispositions the acceptance criteria require
/// by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KillSwitchClass {
    /// A remote kill switch disabled the capability.
    KillSwitch,
    /// A policy disabled the capability (DisabledByPolicy).
    PolicyDisabled,
    /// A required dependency became unavailable, disabling the capability.
    DependencyUnavailable,
    /// The capability's review / expiry window lapsed, disabling it.
    ReviewExpired,
    /// The user manually opted out of the capability.
    ManualOptOut,
    /// The kill-switch class is unclassified, which is disallowed.
    KillSwitchClassUnclassified,
}

impl M5KillSwitchClass {
    /// Every kill-switch class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KillSwitch,
        Self::PolicyDisabled,
        Self::DependencyUnavailable,
        Self::ReviewExpired,
        Self::ManualOptOut,
        Self::KillSwitchClassUnclassified,
    ];

    /// The five canonical kill-switch classes every kill-switch ledger must stay distinct across.
    pub const CANONICAL_CLASSES: [Self; 5] = [
        Self::KillSwitch,
        Self::PolicyDisabled,
        Self::DependencyUnavailable,
        Self::ReviewExpired,
        Self::ManualOptOut,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KillSwitch => "kill_switch",
            Self::PolicyDisabled => "policy_disabled",
            Self::DependencyUnavailable => "dependency_unavailable",
            Self::ReviewExpired => "review_expired",
            Self::ManualOptOut => "manual_opt_out",
            Self::KillSwitchClassUnclassified => "kill_switch_class_unclassified",
        }
    }

    /// Whether the kill-switch class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::KillSwitchClassUnclassified)
    }
}

/// Controlled render context — which claimed M5 flow renders the registry entry, so a capability or kill-switch
/// token's meaning stays stable whether it appears in a settings, docs / help, bundle, or import-apply flow, or
/// in a support / export form. Minted by this lane, tracking the first-consumer flows the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigCapabilitySurfaceContext {
    /// The settings surface flow.
    SettingsSurfaceFlow,
    /// The docs / help flow.
    DocsHelpFlow,
    /// The bundle flow (a signed capability bundle disclosing its lifecycle dependencies).
    BundleFlow,
    /// The import-apply flow.
    ImportApplyFlow,
    /// The support / export form surface (including claim publication).
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ConfigCapabilitySurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SettingsSurfaceFlow,
        Self::DocsHelpFlow,
        Self::BundleFlow,
        Self::ImportApplyFlow,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer flows the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::SettingsSurfaceFlow,
        Self::DocsHelpFlow,
        Self::BundleFlow,
        Self::ImportApplyFlow,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsSurfaceFlow => "settings_surface_flow",
            Self::DocsHelpFlow => "docs_help_flow",
            Self::BundleFlow => "bundle_flow",
            Self::ImportApplyFlow => "import_apply_flow",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a capability or kill-switch entry must be able to show, so no lifecycle class,
/// dependency marker, kill-switch cause, kill-switch-record field, or registry fact is left implicit behind a
/// hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigCapabilityAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The lifecycle class the entry classifies (capability entry).
    LifecycleClassLabel,
    /// The owner, scope, and review / expiry the capability carries (capability entry).
    OwnerScopeAndReview,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The enabled posture, dependency marker, fallback, and rollback note the entry publishes (capability entry).
    DependencyMarkerFallbackAndRollback,
    /// The kill-switch-record fields (disabling source, disabled timestamp, preserved data, explanation,
    /// capability dependency) the entry publishes (kill-switch entry).
    KillSwitchLedgerFields,
    /// The disable-cause / data-preservation hint the entry publishes.
    DisableCauseAndDataPreservationHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved capability or disable (both entries).
    PlainLanguageMeaning,
}

impl M5ConfigCapabilityAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::LifecycleClassLabel,
        Self::OwnerScopeAndReview,
        Self::ResolutionFormCoverage,
        Self::DependencyMarkerFallbackAndRollback,
        Self::KillSwitchLedgerFields,
        Self::DisableCauseAndDataPreservationHint,
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
            Self::LifecycleClassLabel => "lifecycle_class_label",
            Self::OwnerScopeAndReview => "owner_scope_and_review",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::DependencyMarkerFallbackAndRollback => "dependency_marker_fallback_and_rollback",
            Self::KillSwitchLedgerFields => "kill_switch_ledger_fields",
            Self::DisableCauseAndDataPreservationHint => "disable_cause_and_data_preservation_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// capability, a disable, or a degraded capability / kill-switch entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigCapabilityNextAction {
    /// Expand the resolved capability's or disable's plain-language meaning.
    ExpandCapabilityMeaning,
    /// Inspect the lifecycle class or kill-switch ledger the entry resolves.
    InspectClassOrLedger,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ConfigCapabilityNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandCapabilityMeaning,
        Self::InspectClassOrLedger,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandCapabilityMeaning => "expand_capability_meaning",
            Self::InspectClassOrLedger => "inspect_class_or_ledger",
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
pub enum M5ConfigCapabilityExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The settings-governance families covered.
    SettingsGovernanceFamilies,
    /// The capability-lifecycle classes carried.
    CapabilityLifecycleClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The kill-switch classes carried.
    KillSwitchClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The state modes carried.
    LifecycleStateModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ConfigCapabilityExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::CapabilityLifecycleClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::KillSwitchClasses,
        Self::SurfaceContext,
        Self::LifecycleStateModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::CapabilityLifecycleClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::SettingsGovernanceFamilies => "settings_governance_families",
            Self::CapabilityLifecycleClasses => "capability_lifecycle_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::KillSwitchClasses => "kill_switch_classes",
            Self::SurfaceContext => "surface_context",
            Self::LifecycleStateModes => "lifecycle_state_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a capability-record entry degraded below a clean, registry-bound state. The degrade-first ladder returns
/// one of these instead of ever letting a hand-copied, dependency-hiding, record-incomplete, or form-incomplete
/// entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityRecordEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the capability means.
    CapabilityTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The lifecycle class is unclassified (not in the resolved taxonomy).
    CapabilityLifecycleClassUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    CapabilityNotBoundToRegistry,
    /// The resolved capability-record object is incomplete: the owner, scope, review / expiry, enabled posture,
    /// dependency marker, fallback, or rollback note is unstated.
    CapabilityRecordIncomplete,
    /// The dependency marker is not published (a stable surface could depend on a hidden capability), or a
    /// protected (Labs / Preview / Beta) capability published no fallback.
    CapabilityHidesDependencyOrLacksFallback,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A protected capability did not publish a fallback before it was claimed.
    FallbackNotPublishedForProtectedCapability,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CapabilityRecordEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::CapabilityTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CapabilityLifecycleClassUnclassified,
        Self::CapabilityNotBoundToRegistry,
        Self::CapabilityRecordIncomplete,
        Self::CapabilityHidesDependencyOrLacksFallback,
        Self::ResolutionFormCoverageIncomplete,
        Self::FallbackNotPublishedForProtectedCapability,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityTokenUnstated => "capability_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CapabilityLifecycleClassUnclassified => "capability_lifecycle_class_unclassified",
            Self::CapabilityNotBoundToRegistry => "capability_not_bound_to_registry",
            Self::CapabilityRecordIncomplete => "capability_record_incomplete",
            Self::CapabilityHidesDependencyOrLacksFallback => {
                "capability_hides_dependency_or_lacks_fallback"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::FallbackNotPublishedForProtectedCapability => {
                "fallback_not_published_for_protected_capability"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ConfigCapabilityNextAction {
        match self {
            Self::CapabilityTokenUnstated | Self::CapabilityNotBoundToRegistry => {
                M5ConfigCapabilityNextAction::TraceCanonicalRegistry
            }
            Self::CapabilityLifecycleClassUnclassified
            | Self::CapabilityRecordIncomplete
            | Self::CapabilityHidesDependencyOrLacksFallback => {
                M5ConfigCapabilityNextAction::InspectClassOrLedger
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5ConfigCapabilityNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::FallbackNotPublishedForProtectedCapability
            | Self::ProofStale => M5ConfigCapabilityNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::CapabilityTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::CapabilityLifecycleClassUnclassified | Self::CapabilityRecordIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::LifecycleStateUnstated
            }
            Self::CapabilityNotBoundToRegistry => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::CapabilityHidesDependencyOrLacksFallback
            | Self::FallbackNotPublishedForProtectedCapability => {
                M5SettingsGovernanceDowngradeTrigger::HidLifecycleOrExperimentDependencyBehindUnpublishedMarkers
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a kill-switch-record entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KillSwitchRecordEntryDegradeReason {
    /// The canonical registry token name is unstated.
    KillSwitchTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The kill-switch class is unclassified (not in the resolved taxonomy).
    KillSwitchClassUnclassified,
    /// The kill-switch record would hide a kill-switch / policy-disable cause without disclosing its reason, leave
    /// preserved user-authored data without disclosing that it is preserved, or it dropped one of the required
    /// kill-switch-record fields (disabling source, disabled timestamp, preserved-data reference, explanation
    /// reference, capability dependency, fallback reference, last ledger revision).
    KillSwitchHidesCauseOrDropsDataPreservation,
    /// The canonical / accessible / audit resolution-form coverage of the record is incomplete.
    LedgerFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5KillSwitchRecordEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KillSwitchTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::KillSwitchClassUnclassified,
        Self::KillSwitchHidesCauseOrDropsDataPreservation,
        Self::LedgerFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KillSwitchTokenUnstated => "kill_switch_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::KillSwitchClassUnclassified => "kill_switch_class_unclassified",
            Self::KillSwitchHidesCauseOrDropsDataPreservation => {
                "kill_switch_hides_cause_or_drops_data_preservation"
            }
            Self::LedgerFormCoverageIncomplete => "ledger_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ConfigCapabilityNextAction {
        match self {
            Self::KillSwitchTokenUnstated => M5ConfigCapabilityNextAction::TraceCanonicalRegistry,
            Self::KillSwitchClassUnclassified
            | Self::KillSwitchHidesCauseOrDropsDataPreservation => {
                M5ConfigCapabilityNextAction::InspectClassOrLedger
            }
            Self::LedgerFormCoverageIncomplete => {
                M5ConfigCapabilityNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ConfigCapabilityNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::KillSwitchTokenUnstated => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::KillSwitchClassUnclassified => {
                M5SettingsGovernanceDowngradeTrigger::LifecycleStateUnstated
            }
            Self::KillSwitchHidesCauseOrDropsDataPreservation => {
                M5SettingsGovernanceDowngradeTrigger::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy
            }
            Self::LedgerFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_capability_record_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CapabilityRecordEntryResolutionInput {
    /// Stable identity of the capability-registry entry.
    pub entry_id: String,
    /// The stable capability-target ID this record binds to (e.g. `capability.acme.ai.inline-assist@labs`);
    /// empty means unstated.
    pub capability_ref: String,
    /// The canonical registry token name (e.g. `capability.ai.inline_assist`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The lifecycle class this entry classifies.
    pub lifecycle_class: M5CapabilityLifecycleClass,
    /// The render / surface context.
    pub surface_context: M5ConfigCapabilitySurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5ConfigCapabilityResolutionForm>,
    /// The published accountable owner; empty means unstated.
    pub owner: String,
    /// The published capability scope; empty means unstated.
    pub scope: String,
    /// The published review / expiry window; empty means unstated.
    pub review_or_expiry: String,
    /// The published enabled posture; empty means unstated.
    pub enabled_posture: String,
    /// The published artifact dependency marker; empty means unstated.
    pub dependency_marker: String,
    /// The published fallback; empty means unstated.
    pub fallback: String,
    /// The published rollback note; empty means unstated.
    pub rollback_note: String,
    /// True when the behavior traces to the capability-lifecycle registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the dependency marker is published (explicit) rather than hidden behind an unpublished flag (a
    /// hard invariant when `false`).
    pub dependency_marker_published: bool,
    /// True when this lifecycle class is a pre-stable capability that must publish a dependency marker and
    /// fallback before a stable-facing surface may depend on it.
    pub requires_dependency_marker_and_fallback: bool,
    /// True when a fallback is published before a protected capability is claimed.
    pub fallback_published: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe capability-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCapabilityRecordEntry {
    /// Stable identity of the capability-registry entry.
    pub entry_id: String,
    /// The stable capability-target ID this record binds to.
    pub capability_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The lifecycle-class token named by the entry.
    pub lifecycle_class: String,
    /// Whether the lifecycle class is classified into the resolved taxonomy.
    pub lifecycle_class_is_classified: bool,
    /// The canonical state mode for the entry's lifecycle class.
    pub canonical_state_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published owner.
    pub owner: String,
    /// The published scope.
    pub scope: String,
    /// The published review / expiry window.
    pub review_or_expiry: String,
    /// The published enabled posture.
    pub enabled_posture: String,
    /// The published dependency marker.
    pub dependency_marker: String,
    /// The published fallback.
    pub fallback: String,
    /// The published rollback note.
    pub rollback_note: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved capability-record object publishes every required field.
    pub capability_record_complete: bool,
    /// Whether the entry traces to the capability-lifecycle registry.
    pub bound_to_registry: bool,
    /// Whether the dependency marker is published (never hidden behind an unpublished flag).
    pub dependency_marker_published: bool,
    /// Whether this capability requires a dependency marker and fallback.
    pub requires_dependency_marker_and_fallback: bool,
    /// Whether a fallback is published before the capability is claimed.
    pub fallback_published: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5CapabilityRecordEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ConfigCapabilityNextAction,
    /// Whether the capability resolves to one object across every claimed route (clean entry naming every fact).
    pub capability_resolves_across_routes: bool,
}

impl M5ResolvedCapabilityRecordEntry {
    /// Whether this capability entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_kill_switch_record_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5KillSwitchRecordEntryResolutionInput {
    /// Stable identity of the kill-switch entry.
    pub entry_id: String,
    /// The stable capability-ref this record binds to; empty means unstated.
    pub capability_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The kill-switch class this record must resolve.
    pub kill_switch_class: M5KillSwitchClass,
    /// The render / surface context.
    pub surface_context: M5ConfigCapabilitySurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5ConfigCapabilityResolutionForm>,
    /// The published disabling source; empty means missing.
    pub disabling_source: String,
    /// The published disabled timestamp; empty means missing.
    pub disabled_timestamp: String,
    /// The published preserved-data reference; empty means missing.
    pub preserved_data_reference: String,
    /// The published explanation reference; empty means missing.
    pub explanation_reference: String,
    /// The published capability dependency; empty means missing.
    pub capability_dependency: String,
    /// The published fallback reference; empty means missing.
    pub fallback_reference: String,
    /// The published last ledger revision; empty means missing.
    pub last_ledger_revision: String,
    /// True when the record keeps the disabling source visible.
    pub keeps_disabling_source_visible: bool,
    /// True when the ledger is truthful (never claims a clean resolution over a hidden cause).
    pub ledger_is_truthful: bool,
    /// True when the disable is a kill switch / policy disable (its cause must be disclosed).
    pub policy_disable_present: bool,
    /// True when a kill-switch / policy-disable discloses its reason (never hides the cause).
    pub disable_cause_disclosed: bool,
    /// True when the capability held user-authored data at disable time.
    pub user_data_present: bool,
    /// True when a disable that held user-authored data discloses that the data stays preserved.
    pub user_data_preservation_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe kill-switch projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedKillSwitchRecordEntry {
    /// Stable identity of the kill-switch entry.
    pub entry_id: String,
    /// The stable capability-ref this record binds to.
    pub capability_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The kill-switch-class token named by the entry.
    pub kill_switch_class: String,
    /// Whether the kill-switch class is classified into the resolved taxonomy.
    pub kill_switch_class_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published disabling source.
    pub disabling_source: String,
    /// The published disabled timestamp.
    pub disabled_timestamp: String,
    /// The published preserved-data reference.
    pub preserved_data_reference: String,
    /// The published explanation reference.
    pub explanation_reference: String,
    /// The published capability dependency.
    pub capability_dependency: String,
    /// The published fallback reference.
    pub fallback_reference: String,
    /// The published last ledger revision.
    pub last_ledger_revision: String,
    /// Whether the record keeps the disabling source visible.
    pub keeps_disabling_source_visible: bool,
    /// Whether the ledger is truthful.
    pub ledger_is_truthful: bool,
    /// Whether the disable is a kill switch / policy disable.
    pub policy_disable_present: bool,
    /// Whether a kill-switch / policy-disable discloses its reason.
    pub disable_cause_disclosed: bool,
    /// Whether the capability held user-authored data at disable time.
    pub user_data_present: bool,
    /// Whether a disable that held user-authored data discloses that the data stays preserved.
    pub user_data_preservation_disclosed: bool,
    /// Whether the record preserves user data and explains itself (disabling source visible, cause disclosed,
    /// data-preservation disclosed).
    pub kill_switch_record_preserves_data_and_explains: bool,
    /// Whether the entry provides the complete kill-switch-record object (disabling source, disabled timestamp,
    /// preserved-data reference, explanation reference, capability dependency, fallback reference, last ledger
    /// revision).
    pub provides_complete_kill_switch_ledger: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5KillSwitchRecordEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ConfigCapabilityNextAction,
    /// Whether the kill-switch ledger is safe on every claimed route (clean entry naming every fact).
    pub ledger_safe_on_every_route: bool,
}

impl M5ResolvedKillSwitchRecordEntry {
    /// Whether this kill-switch entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ConfigCapabilityResolutionError {
    /// The capability-entry id was empty.
    EmptyCapabilityEntryId,
    /// The kill-switch-entry id was empty.
    EmptyKillSwitchEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ConfigCapabilityResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCapabilityEntryId => "empty_capability_entry_id",
            Self::EmptyKillSwitchEntryId => "empty_kill_switch_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ConfigCapabilityResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 capability-record / kill-switch registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ConfigCapabilityResolutionError {}

fn form_tokens(forms: &[M5ConfigCapabilityResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5ConfigCapabilityResolutionForm]) -> bool {
    let present: BTreeSet<M5ConfigCapabilityResolutionForm> = forms.iter().copied().collect();
    M5ConfigCapabilityResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved capability-record object publishes every required field: declared lifecycle class (via a
/// classified class), owner, scope, review / expiry, enabled posture, dependency marker, fallback, and rollback
/// note. An unclassified class or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn capability_record_is_complete(
    class: M5CapabilityLifecycleClass,
    owner: &str,
    scope: &str,
    review_or_expiry: &str,
    enabled_posture: &str,
    dependency_marker: &str,
    fallback: &str,
    rollback_note: &str,
) -> bool {
    class.is_classified()
        && !owner.trim().is_empty()
        && !scope.trim().is_empty()
        && !review_or_expiry.trim().is_empty()
        && !enabled_posture.trim().is_empty()
        && !dependency_marker.trim().is_empty()
        && !fallback.trim().is_empty()
        && !rollback_note.trim().is_empty()
}

/// Whether the capability keeps its dependency marker published and fallback-safe: the class must be classified,
/// the dependency marker must be published (never hidden behind an unpublished flag), and a protected (Labs /
/// Preview / Beta) capability must publish a fallback before a stable-facing surface may depend on it. An
/// unclassified class, a hidden dependency marker, or a protected capability without a fallback never matches.
pub fn capability_does_not_hide_dependency(
    class: M5CapabilityLifecycleClass,
    dependency_marker_published: bool,
    requires_dependency_marker_and_fallback: bool,
    fallback_published: bool,
) -> bool {
    class.is_classified()
        && dependency_marker_published
        && (!requires_dependency_marker_and_fallback || fallback_published)
}

/// Whether a kill-switch ledger preserves user data and explains itself: the class must be classified, the ledger
/// must be truthful, it must keep the disabling source visible, any kill-switch / policy-disable must disclose its
/// cause rather than hide it, and any disable that held user-authored data must disclose that the data stays
/// preserved rather than read as an ambiguous loss.
pub fn kill_switch_record_preserves_data_and_explains(
    class: M5KillSwitchClass,
    ledger_is_truthful: bool,
    keeps_disabling_source_visible: bool,
    policy_disable_present: bool,
    disable_cause_disclosed: bool,
    user_data_present: bool,
    user_data_preservation_disclosed: bool,
) -> bool {
    class.is_classified()
        && ledger_is_truthful
        && keeps_disabling_source_visible
        && (!policy_disable_present || disable_cause_disclosed)
        && (!user_data_present || user_data_preservation_disclosed)
}

/// Resolves a capability-registry entry so it stays bound to the capability-lifecycle registry: the entry names
/// its canonical token, semantic role, and lifecycle class, covers all three resolution forms, publishes a
/// complete capability-record object (owner, scope, review / expiry, enabled posture, dependency marker,
/// fallback, rollback note), keeps its dependency marker published, and publishes a fallback before a protected
/// capability is claimed.
pub fn resolve_capability_record_entry(
    input: M5CapabilityRecordEntryResolutionInput,
) -> Result<M5ResolvedCapabilityRecordEntry, M5ConfigCapabilityResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ConfigCapabilityResolutionError::EmptyCapabilityEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.capability_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.owner)
        || string_is_forbidden(&input.scope)
        || string_is_forbidden(&input.review_or_expiry)
        || string_is_forbidden(&input.enabled_posture)
        || string_is_forbidden(&input.dependency_marker)
        || string_is_forbidden(&input.fallback)
        || string_is_forbidden(&input.rollback_note)
    {
        return Err(M5ConfigCapabilityResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = capability_record_is_complete(
        input.lifecycle_class,
        &input.owner,
        &input.scope,
        &input.review_or_expiry,
        &input.enabled_posture,
        &input.dependency_marker,
        &input.fallback,
        &input.rollback_note,
    );
    let dependency_ok = capability_does_not_hide_dependency(
        input.lifecycle_class,
        input.dependency_marker_published,
        input.requires_dependency_marker_and_fallback,
        input.fallback_published,
    );
    let fallback_unpublished =
        input.requires_dependency_marker_and_fallback && !input.fallback_published;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CapabilityRecordEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.lifecycle_class.is_classified() {
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityLifecycleClassUnclassified)
    } else if !input.bound_to_registry {
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityNotBoundToRegistry)
    } else if !object_complete {
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityRecordIncomplete)
    } else if !dependency_ok {
        Some(M5CapabilityRecordEntryDegradeReason::CapabilityHidesDependencyOrLacksFallback)
    } else if !all_forms {
        Some(M5CapabilityRecordEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if fallback_unpublished {
        Some(M5CapabilityRecordEntryDegradeReason::FallbackNotPublishedForProtectedCapability)
    } else if !input.proof_fresh {
        Some(M5CapabilityRecordEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ConfigCapabilityNextAction::ExpandCapabilityMeaning,
    };

    Ok(M5ResolvedCapabilityRecordEntry {
        entry_id: input.entry_id,
        capability_ref: input.capability_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        lifecycle_class: input.lifecycle_class.as_str().to_owned(),
        lifecycle_class_is_classified: input.lifecycle_class.is_classified(),
        canonical_state_mode: input.lifecycle_class.canonical_state_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        owner: input.owner,
        scope: input.scope,
        review_or_expiry: input.review_or_expiry,
        enabled_posture: input.enabled_posture,
        dependency_marker: input.dependency_marker,
        fallback: input.fallback,
        rollback_note: input.rollback_note,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        capability_record_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        dependency_marker_published: input.dependency_marker_published,
        requires_dependency_marker_and_fallback: input.requires_dependency_marker_and_fallback,
        fallback_published: input.fallback_published,
        degrade_reason,
        next_action,
        capability_resolves_across_routes: degrade_reason.is_none(),
    })
}

/// Resolves a kill-switch entry so its resolution stays safe: the entry names its canonical token, semantic role,
/// and kill-switch class, covers all three resolution forms, provides the complete disabling-source /
/// disabled-timestamp / preserved-data-reference / explanation-reference / capability-dependency /
/// fallback-reference / last-ledger-revision kill-switch-record object, and degrades honestly when the record
/// would hide a kill-switch / policy-disable cause without disclosing its reason or leave preserved user-authored
/// data without disclosing that it stays preserved.
pub fn resolve_kill_switch_record_entry(
    input: M5KillSwitchRecordEntryResolutionInput,
) -> Result<M5ResolvedKillSwitchRecordEntry, M5ConfigCapabilityResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ConfigCapabilityResolutionError::EmptyKillSwitchEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.capability_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.disabling_source)
        || string_is_forbidden(&input.disabled_timestamp)
        || string_is_forbidden(&input.preserved_data_reference)
        || string_is_forbidden(&input.explanation_reference)
        || string_is_forbidden(&input.capability_dependency)
        || string_is_forbidden(&input.fallback_reference)
        || string_is_forbidden(&input.last_ledger_revision)
    {
        return Err(M5ConfigCapabilityResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_preserves = kill_switch_record_preserves_data_and_explains(
        input.kill_switch_class,
        input.ledger_is_truthful,
        input.keeps_disabling_source_visible,
        input.policy_disable_present,
        input.disable_cause_disclosed,
        input.user_data_present,
        input.user_data_preservation_disclosed,
    );
    let provides_record = input.kill_switch_class.is_classified()
        && !input.disabling_source.trim().is_empty()
        && !input.disabled_timestamp.trim().is_empty()
        && !input.preserved_data_reference.trim().is_empty()
        && !input.explanation_reference.trim().is_empty()
        && !input.capability_dependency.trim().is_empty()
        && !input.fallback_reference.trim().is_empty()
        && !input.last_ledger_revision.trim().is_empty()
        && record_preserves;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5KillSwitchRecordEntryDegradeReason::KillSwitchTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5KillSwitchRecordEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.kill_switch_class.is_classified() {
        Some(M5KillSwitchRecordEntryDegradeReason::KillSwitchClassUnclassified)
    } else if !provides_record {
        Some(M5KillSwitchRecordEntryDegradeReason::KillSwitchHidesCauseOrDropsDataPreservation)
    } else if !all_forms {
        Some(M5KillSwitchRecordEntryDegradeReason::LedgerFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5KillSwitchRecordEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ConfigCapabilityNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedKillSwitchRecordEntry {
        entry_id: input.entry_id,
        capability_ref: input.capability_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        kill_switch_class: input.kill_switch_class.as_str().to_owned(),
        kill_switch_class_is_classified: input.kill_switch_class.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        disabling_source: input.disabling_source,
        disabled_timestamp: input.disabled_timestamp,
        preserved_data_reference: input.preserved_data_reference,
        explanation_reference: input.explanation_reference,
        capability_dependency: input.capability_dependency,
        fallback_reference: input.fallback_reference,
        last_ledger_revision: input.last_ledger_revision,
        keeps_disabling_source_visible: input.keeps_disabling_source_visible,
        ledger_is_truthful: input.ledger_is_truthful,
        policy_disable_present: input.policy_disable_present,
        disable_cause_disclosed: input.disable_cause_disclosed,
        user_data_present: input.user_data_present,
        user_data_preservation_disclosed: input.user_data_preservation_disclosed,
        kill_switch_record_preserves_data_and_explains: record_preserves,
        provides_complete_kill_switch_ledger: provides_record,
        degrade_reason,
        next_action,
        ledger_safe_on_every_route: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved capability and kill-switch entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingCapabilityLifecycleKillSwitchRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SettingCapabilityLifecycleKillSwitchRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5SettingsGovernanceQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Configuration contexts this row keeps the same truth across.
    pub deployment_lines: Vec<M5SettingsGovernanceDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5SettingsGovernanceRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5SettingsGovernanceAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ConfigCapabilityAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ConfigCapabilityExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    /// Resolved capability-registry examples.
    pub capability_entries: Vec<M5ResolvedCapabilityRecordEntry>,
    /// Resolved kill-switch examples.
    pub kill_switch_entries: Vec<M5ResolvedKillSwitchRecordEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the capability-lifecycle domain and the
    /// landed capability-lifecycle schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never hides a lifecycle or experiment dependency behind unpublished markers. MUST
    /// be `false`.
    pub hides_lifecycle_or_experiment_dependency_behind_unpublished_markers: bool,
    /// Hard invariant: this row never hides a kill-switch or policy-disable cause behind generic unavailable copy.
    /// MUST be `false`.
    pub hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy: bool,
    /// Hard invariant: this row never lets a stable-facing surface depend on a hidden Labs or Preview capability.
    /// MUST be `false`.
    pub lets_a_stable_surface_depend_on_a_hidden_labs_or_preview_capability: bool,
    /// Hard invariant: this row never loses user-authored data when a kill switch or policy disable fires. MUST be
    /// `false`.
    pub loses_user_authored_data_when_a_kill_switch_or_policy_disable_fires: bool,
}

impl M5SettingCapabilityLifecycleKillSwitchRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ConfigCapabilityAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ConfigCapabilityAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ConfigCapabilityExportField> =
            self.export_fields.iter().copied().collect();
        M5ConfigCapabilityExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.hides_lifecycle_or_experiment_dependency_behind_unpublished_markers
            && !self.hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy
            && !self.lets_a_stable_surface_depend_on_a_hidden_labs_or_preview_capability
            && !self.loses_user_authored_data_when_a_kill_switch_or_policy_disable_fires
    }

    /// True when a clean capability entry preserves registry-bound truth: it traces to the registry, keeps a
    /// classified lifecycle class, publishes a complete capability record, keeps its dependency marker published,
    /// covers all three resolution forms, and publishes a fallback for a protected capability.
    fn capability_is_honest(ex: &M5ResolvedCapabilityRecordEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.lifecycle_class_is_classified
                && ex.capability_record_complete
                && ex.dependency_marker_published
                && ex.covers_all_resolution_forms
                && (!ex.requires_dependency_marker_and_fallback || ex.fallback_published))
    }

    /// True when a clean kill-switch entry preserves a safe record: it keeps a classified class, provides the
    /// complete kill-switch-record object, preserves user data and explains itself, and covers all three
    /// resolution forms.
    fn kill_switch_is_honest(ex: &M5ResolvedKillSwitchRecordEntry) -> bool {
        !ex.is_clean()
            || (ex.kill_switch_class_is_classified
                && ex.provides_complete_kill_switch_ledger
                && ex.kill_switch_record_preserves_data_and_explains
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.capability_entries
            .iter()
            .all(Self::capability_is_honest)
            && self
                .kill_switch_entries
                .iter()
                .all(Self::kill_switch_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingCapabilityLifecycleKillSwitchRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Capability-lifecycle-class tokens (minted by this lane).
    pub capability_lifecycle_classes: Vec<String>,
    /// Kill-switch-class tokens (minted by this lane).
    pub kill_switch_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Capability-entry degrade-reason tokens.
    pub capability_degrade_reasons: Vec<String>,
    /// Kill-switch-entry degrade-reason tokens.
    pub kill_switch_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SettingCapabilityLifecycleKillSwitchRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5SettingsGovernanceRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5ConfigCapabilityResolutionForm::ALL, |v| v.as_str()),
            capability_lifecycle_classes: tokens(&M5CapabilityLifecycleClass::ALL, |v| v.as_str()),
            kill_switch_classes: tokens(&M5KillSwitchClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ConfigCapabilitySurfaceContext::ALL, |v| v.as_str()),
            capability_degrade_reasons: tokens(&M5CapabilityRecordEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            kill_switch_degrade_reasons: tokens(&M5KillSwitchRecordEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5ConfigCapabilityAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ConfigCapabilityNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ConfigCapabilityExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5SettingsGovernanceConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5SettingCapabilityLifecycleKillSwitchRegistriesGovernanceReview {
    /// The capability registry names a canonical token, semantic role, and lifecycle class for every entry.
    pub capability_registry_names_token_role_and_class: bool,
    /// Every claimed capability resolves to one capability record from the shared registry, not per-entry
    /// reconstruction.
    pub capability_resolves_to_one_record_from_shared_registry: bool,
    /// The owner, scope, review / expiry, enabled posture, dependency marker, fallback, and rollback note are
    /// published for every resolved capability.
    pub owner_scope_review_dependency_marker_fallback_and_rollback_published: bool,
    /// No stable-facing surface depends on a hidden Labs / Preview capability without a marker and fallback.
    pub no_stable_surface_depends_on_a_hidden_labs_or_preview_capability: bool,
    /// The kill-switch record keeps the disabling source visible and discloses the kill-switch / policy-disable
    /// cause and the user-data-preservation posture.
    pub kill_switch_record_keeps_source_visible_and_discloses_cause: bool,
    /// User-authored data is preserved before any kill switch or policy disable fires.
    pub user_authored_data_preserved_before_kill_switch_fires: bool,
    /// Every capability and kill-switch entry covers the canonical / accessible / audit resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Capability and kill-switch behavior stay bound to the shared registries rather than hand-copied per
    /// capability.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Settings, docs / help, bundle, and import-apply flows read a single configuration source.
    pub settings_docs_bundle_and_import_read_single_source: bool,
    /// A hidden dependency, an incomplete record, or a hidden kill-switch ledger is caught by fixtures before
    /// release evidence turns green.
    pub capability_or_ledger_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingCapabilityLifecycleKillSwitchRegistriesConsumerProjection {
    /// Settings and docs / help flows consume the shared capability registry.
    pub settings_and_docs_consume_shared_registries: bool,
    /// Bundle and import-apply flows consume the shared kill-switch registry.
    pub bundle_and_import_consume_shared_registries: bool,
    /// Sync and policy services consume the shared registries.
    pub sync_and_policy_services_consume_shared_registries: bool,
    /// Docs, admin, and CLI export consume the shared registries.
    pub docs_admin_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical capability-lifecycle domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export and claim publication read a single canonical capability / kill-switch registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingCapabilityLifecycleKillSwitchRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingCapabilityLifecycleKillSwitchRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting settings-governance audit for the lane.
    pub settings_governance_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SettingCapabilityLifecycleKillSwitchRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingCapabilityLifecycleKillSwitchRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingCapabilityLifecycleKillSwitchRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingCapabilityLifecycleKillSwitchRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingCapabilityLifecycleKillSwitchRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingCapabilityLifecycleKillSwitchRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingCapabilityLifecycleKillSwitchRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 capability-record and kill-switch-record registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingCapabilityLifecycleKillSwitchRegistriesPacket {
    /// Record kind; must equal [`M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingCapabilityLifecycleKillSwitchRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingCapabilityLifecycleKillSwitchRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingCapabilityLifecycleKillSwitchRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingCapabilityLifecycleKillSwitchRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingCapabilityLifecycleKillSwitchRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SettingCapabilityLifecycleKillSwitchRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SettingCapabilityLifecycleKillSwitchRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version: M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_RECORD_KIND {
            violations
                .push(M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version
            != M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 capability-record / kill-switch registries packet serializes"),
        ) {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::RawMaterialInExport,
            );
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
            .expect("m5 capability-record / kill-switch registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,capability_entries,kill_switch_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .capability_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.kill_switch_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.capability_entries.len(),
                row.kill_switch_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Setting-Capability-Lifecycle and Kill-Switch Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Capability-lifecycle classes: {}\n",
            self.vocabulary_set.capability_lifecycle_classes.join(", ")
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
                "  - Capability entries: {} / kill-switch entries: {}\n",
                row.capability_entries.len(),
                row.kill_switch_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry capability reference table generated from the registry, so docs and lifecycle
    /// runbooks render the same state-mode / owner / dependency-marker / fallback / rollback-note truth the
    /// resolvers produced rather than a hand-copied capability table. Only clean, registry-bound capability
    /// entries are listed.
    pub fn render_capability_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| capability_ref | state_mode | owner | enabled_posture | dependency_marker | fallback | rollback_note |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.capability_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.capability_ref,
                    ex.canonical_state_mode,
                    ex.owner,
                    ex.enabled_posture,
                    ex.dependency_marker,
                    ex.fallback,
                    ex.rollback_note
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SettingCapabilityLifecycleKillSwitchRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation>),
}

impl fmt::Display for M5SettingCapabilityLifecycleKillSwitchRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 capability-record / kill-switch registries export parse failed: {error}"
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
                    "m5 capability-record / kill-switch registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SettingCapabilityLifecycleKillSwitchRegistriesArtifactError {}

/// Validation failures emitted by [`M5SettingCapabilityLifecycleKillSwitchRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SettingCapabilityLifecycleKillSwitchRegistriesViolation {
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
    /// A registry row does not point at both the capability-lifecycle domain and the landed capability-lifecycle
    /// schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, dependency-hiding, record-incomplete,
    /// form-incomplete, or a kill-switch entry missing the complete record object).
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
    /// Capability-lifecycle-resolution is not proven: clean capability entries do not cover the canonical lifecycle
    /// classes or the first settings / docs-help / bundle / import-apply / support flows, no record-incomplete
    /// example degrades, or a clean capability entry published an incomplete record.
    CapabilityLifecycleResolutionNotProven,
    /// Dependency-marker-honesty is not proven: no dependency-hide example and no unbound example degrade, no clean
    /// dependency-published capability entry is present, or a clean capability entry hid its dependency marker or
    /// is unbound.
    DependencyMarkerHonestyNotProven,
    /// Kill-switch-data-preservation is not proven: clean kill-switch entries do not cover the canonical
    /// kill-switch / policy-disabled / dependency-unavailable / review-expired / manual-opt-out classes with full
    /// resolution-form coverage while providing the complete record object, no hidden-cause or form-incomplete
    /// example degrades, or a clean kill-switch entry is missing the complete record object.
    KillSwitchDataPreservationNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SettingCapabilityLifecycleKillSwitchRegistriesViolation {
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
            Self::CapabilityLifecycleResolutionNotProven => {
                "capability_lifecycle_resolution_not_proven"
            }
            Self::DependencyMarkerHonestyNotProven => "dependency_marker_honesty_not_proven",
            Self::KillSwitchDataPreservationNotProven => "kill_switch_data_preservation_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_setting_capability_lifecycle_and_kill_switch_registries_export() -> Result<
    M5SettingCapabilityLifecycleKillSwitchRegistriesPacket,
    M5SettingCapabilityLifecycleKillSwitchRegistriesArtifactError,
> {
    let packet: M5SettingCapabilityLifecycleKillSwitchRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-setting-capability-lifecycle-and-kill-switch-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SettingCapabilityLifecycleKillSwitchRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SettingCapabilityLifecycleKillSwitchRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SettingCapabilityLifecycleKillSwitchRegistriesPacket,
    violations: &mut Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_SCHEMA_REF,
        M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF,
        M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SettingCapabilityLifecycleKillSwitchRegistriesPacket,
    violations: &mut Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::NoRegistryRows);
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
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF)
        {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.capability_entries.is_empty() || row.kill_switch_entries.is_empty() {
            violations
                .push(M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5SettingCapabilityLifecycleKillSwitchRegistriesPacket,
    violations: &mut Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.capability_registry_names_token_role_and_class,
        review.capability_resolves_to_one_record_from_shared_registry,
        review.owner_scope_review_dependency_marker_fallback_and_rollback_published,
        review.no_stable_surface_depends_on_a_hidden_labs_or_preview_capability,
        review.kill_switch_record_keeps_source_visible_and_discloses_cause,
        review.user_authored_data_preserved_before_kill_switch_fires,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.settings_docs_bundle_and_import_read_single_source,
        review.capability_or_ledger_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SettingCapabilityLifecycleKillSwitchRegistriesPacket,
    violations: &mut Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.settings_and_docs_consume_shared_registries,
        projection.bundle_and_import_consume_shared_registries,
        projection.sync_and_policy_services_consume_shared_registries,
        projection.docs_admin_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SettingCapabilityLifecycleKillSwitchRegistriesPacket,
    violations: &mut Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5SettingCapabilityLifecycleKillSwitchRegistriesPacket,
    violations: &mut Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.settings_governance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SettingCapabilityLifecycleKillSwitchRegistriesPacket,
    violations: &mut Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesViolation>,
) {
    let capabilities = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.capability_entries.iter())
    };
    let kill_switches = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.kill_switch_entries.iter())
    };

    // AC1: lifecycle states, dependency markers, and kill-switch posture stay canonical across surfaces. Clean
    // capability entries cover the canonical lifecycle classes and the first settings / docs-help / bundle /
    // import-apply / support flows, a record-incomplete example degrades, and no clean capability entry published
    // an incomplete record.
    let clean_classes: BTreeSet<String> = capabilities()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.lifecycle_class.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = capabilities()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let classes_covered = M5CapabilityLifecycleClass::CANONICAL_CLASSES
        .iter()
        .all(|k| clean_classes.contains(k.as_str()));
    let first_surfaces_covered = M5ConfigCapabilitySurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let record_incomplete_degrades = capabilities().any(|ex| {
        ex.degrade_reason == Some(M5CapabilityRecordEntryDegradeReason::CapabilityRecordIncomplete)
    });
    let no_clean_incomplete =
        !capabilities().any(|ex| ex.is_clean() && !ex.capability_record_complete);
    if !(classes_covered
        && first_surfaces_covered
        && record_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::CapabilityLifecycleResolutionNotProven,
        );
    }

    // AC2: no stable-facing surface depends on a hidden Labs / Preview capability without a marker and fallback. A
    // dependency-hide example degrades, an unbound example degrades, at least one clean dependency-published
    // capability entry is present, and no clean capability entry hid its dependency marker or is unbound.
    let dependency_hide_degrades = capabilities().any(|ex| {
        ex.degrade_reason
            == Some(M5CapabilityRecordEntryDegradeReason::CapabilityHidesDependencyOrLacksFallback)
    });
    let unbound_degrades = capabilities().any(|ex| {
        ex.degrade_reason
            == Some(M5CapabilityRecordEntryDegradeReason::CapabilityNotBoundToRegistry)
    });
    let dependency_published_clean =
        capabilities().any(|ex| ex.is_clean() && ex.dependency_marker_published);
    let no_clean_unbound = !capabilities().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_dependency_hidden =
        !capabilities().any(|ex| ex.is_clean() && !ex.dependency_marker_published);
    if !(dependency_hide_degrades
        && unbound_degrades
        && dependency_published_clean
        && no_clean_unbound
        && no_clean_dependency_hidden)
    {
        violations.push(
            M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::DependencyMarkerHonestyNotProven,
        );
    }

    // AC3: regression suites fail when consumers describe the same capability state, dependency, or kill-switch
    // outcome differently. Clean kill-switch entries cover every canonical kill-switch / policy-disabled /
    // dependency-unavailable / review-expired / manual-opt-out class with full resolution-form coverage while
    // providing the complete record object, a hidden-cause example degrades, a form-incomplete example degrades,
    // and no clean kill-switch entry is missing the complete record object.
    let clean_record_classes: BTreeSet<String> = kill_switches()
        .filter(|ex| {
            ex.is_clean()
                && ex.kill_switch_class_is_classified
                && ex.provides_complete_kill_switch_ledger
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.kill_switch_class.clone())
        .collect();
    let record_classes_covered = M5KillSwitchClass::CANONICAL_CLASSES
        .iter()
        .all(|m| clean_record_classes.contains(m.as_str()));
    let hidden_ledger_degrades = kill_switches().any(|ex| {
        ex.degrade_reason
            == Some(
                M5KillSwitchRecordEntryDegradeReason::KillSwitchHidesCauseOrDropsDataPreservation,
            )
    });
    let form_incomplete_degrades = kill_switches().any(|ex| {
        ex.degrade_reason
            == Some(M5KillSwitchRecordEntryDegradeReason::LedgerFormCoverageIncomplete)
    });
    let no_clean_missing_record =
        !kill_switches().any(|ex| ex.is_clean() && !ex.provides_complete_kill_switch_ledger);
    if !(record_classes_covered
        && hidden_ledger_degrades
        && form_incomplete_degrades
        && no_clean_missing_record)
    {
        violations.push(
            M5SettingCapabilityLifecycleKillSwitchRegistriesViolation::KillSwitchDataPreservationNotProven,
        );
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

/// The settings-governance families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5SettingsGovernanceFamily; 1] =
    [M5SettingsGovernanceFamily::RolloutCapability];
