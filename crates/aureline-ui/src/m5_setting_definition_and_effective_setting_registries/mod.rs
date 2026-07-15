//! Implemented M5 setting-definition and effective-setting registries.
//!
//! The frozen [settings-governance matrix][matrix] names Aureline's five configuration-runtime families and
//! locks their controlled vocabulary. This module is the first implement lane for the concrete
//! setting-resolution flows: it turns the *setting-definition* grammar (how a stable setting is declared) and
//! the *effective-setting* grammar (how its live value is resolved from the winning scope) into registry
//! resolvers that produce export-safe, honest projections. Every claimed M5 configuration surface then
//! resolves to one stable setting-definition object — the declared type it classifies, the stable setting ID
//! it preserves verbatim (never recycled), the allowed scopes, the declared default, the migration aliases,
//! the restart posture, the sensitivity class, and the capability dependencies kept distinct — and to one
//! effective-setting object — the resolved value or redacted summary, the shadow chain of scopes that lost,
//! the lock or constraint state, the validation status, the restart state, the capability availability, and
//! the last-applied revision — that the settings, shell, diagnostics, admin, and support / export surfaces
//! can inspect without manual reconstruction, so a stable setting ID is never recycled into a different
//! meaning, the shadow chain and restart posture stay visible before the resolution is trusted, and a
//! configuration surface that cannot explain the setting it declared or the scope that won degrades honestly
//! instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one stable setting-definition object per setting.** [`resolve_setting_definition_entry`]
//!   refuses to read as a clean, registry-bound definition entry unless it names a canonical registry token, a
//!   classified [setting-definition type][M5SettingDefinitionKind], a settings-governance role, covers every
//!   [resolution form][M5SettingResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every definition field (stable setting ID, allowed scopes, declared default, migration
//!   aliases, restart posture, sensitivity class, and capability dependencies), preserves the stable setting
//!   ID as a non-recycled identity, and discloses the redaction posture before a sensitive setting is
//!   surfaced; otherwise it degrades.
//! * **Keep the setting definition from recycling a stable setting ID.**
//!   [`stable_setting_id_stays_non_recycled`] rejects a definition entry whose stable setting ID was recycled
//!   into a different meaning so it degrades to
//!   [`M5SettingDefinitionEntryDegradeReason::SettingDefinitionRecyclesIdOrHidesSensitivity`], and a sensitive
//!   setting that hides its redaction / sensitivity posture behind generic copy degrades the same way.
//! * **Keep the effective setting from hiding the shadow chain or masking locked / machine-only state.**
//!   [`resolve_effective_setting_entry`] names a classified [winning scope][M5EffectiveSettingScope], requires
//!   the full resolved-value / shadow-chain / lock-state / validation-status / restart-state /
//!   capability-availability / last-applied-revision effective-setting object, covers every resolution form,
//!   and degrades to
//!   [`M5EffectiveSettingEntryDegradeReason::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState`] when
//!   the record would hide the shadow chain of scopes that lost, mask a locked value without disclosing its
//!   lock source, or let machine-only state masquerade as portable, so an effective setting can never read as
//!   trustworthy when it has quietly dropped the reason another scope lost.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5SettingsGovernanceRole`] role
//! vocabulary and the [`M5SettingsGovernanceConsumerSurface`] consumer-surface taxonomy — so the settings,
//! shell, diagnostics, admin, sync, policy, capability, docs, CLI, and support surfaces can never fork their
//! own configuration meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_settings_governance_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_setting_definition_and_effective_setting_registries,
    seeded_m5_setting_definition_and_effective_setting_registries_effective_setting_preview_narrowed,
    seeded_m5_setting_definition_and_effective_setting_registries_setting_definition_beta_narrowed,
    M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_settings_governance_matrix::{
    M5SettingsGovernanceAccessibilityRoute, M5SettingsGovernanceConsumerSurface,
    M5SettingsGovernanceDeploymentLine, M5SettingsGovernanceDowngradeTrigger,
    M5SettingsGovernanceFamily, M5SettingsGovernanceQualificationClass,
    M5SettingsGovernanceRequiredLabel, M5SettingsGovernanceRole, M5_EFFECTIVE_SETTING_SCHEMA_REF,
    M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF, M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
    M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SettingDefinitionEffectiveSettingRegistriesPacket`].
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_setting_definition_and_effective_setting_registries";

/// Schema version for M5 setting-definition / effective-setting registry records.
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_SCHEMA_REF: &str =
    "schemas/config/m5-setting-definition-and-effective-setting-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_DOC_REF: &str =
    "docs/settings/m5_setting_definition_and_effective_setting_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-setting-definition-and-effective-setting-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-setting-definition-and-effective-setting-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-setting-definition-and-effective-setting-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/config/m5-setting-definition-and-effective-setting-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5SettingDefinitionEffectiveSettingRegistriesConsumerSurface =
    M5SettingsGovernanceConsumerSurface;

/// One of the three resolution forms every setting-definition or effective-setting entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary,
/// or written to the audit / support record. Minted by this lane because the frozen matrix names the
/// setting-definition and effective-setting *families* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingResolutionForm {
    /// The canonical resolved setting-definition / effective-setting object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved setting discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved setting inspectable off-renderer.
    AuditRecord,
}

impl M5SettingResolutionForm {
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

/// Controlled setting-definition type a setting-definition entry declares, so the canonical definition model
/// shares one registry rather than a hand-copied per-setting assumption. Minted by this lane because the
/// frozen matrix carries the configuration families but not the concrete boolean / enum / number / string /
/// path / secret-reference declared type a definition entry classifies against. Every classified type carries
/// its canonical type mode, and the path and secret-reference types are sensitivity-bearing so their live
/// value must be disclosed as a redacted summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingDefinitionKind {
    /// A boolean setting.
    BooleanSetting,
    /// An enumerated setting (a fixed set of allowed variants).
    EnumSetting,
    /// A numeric setting.
    NumberSetting,
    /// A filesystem-path setting (sensitivity-bearing; a path may leak location).
    PathSetting,
    /// A secret-reference setting (sensitivity-bearing; references a stored handle, never the secret itself).
    SecretReferenceSetting,
    /// The setting-definition type is unclassified, which is disallowed.
    TypeUnclassified,
}

impl M5SettingDefinitionKind {
    /// Every setting-definition type, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BooleanSetting,
        Self::EnumSetting,
        Self::NumberSetting,
        Self::PathSetting,
        Self::SecretReferenceSetting,
        Self::TypeUnclassified,
    ];

    /// The five canonical setting-definition types every claimed M5 setting classifies against.
    pub const CANONICAL_TYPES: [Self; 5] = [
        Self::BooleanSetting,
        Self::EnumSetting,
        Self::NumberSetting,
        Self::PathSetting,
        Self::SecretReferenceSetting,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BooleanSetting => "boolean_setting",
            Self::EnumSetting => "enum_setting",
            Self::NumberSetting => "number_setting",
            Self::PathSetting => "path_setting",
            Self::SecretReferenceSetting => "secret_reference_setting",
            Self::TypeUnclassified => "type_unclassified",
        }
    }

    /// Whether the type is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::TypeUnclassified)
    }

    /// The canonical type mode for this kind.
    pub const fn canonical_type_mode(self) -> &'static str {
        match self {
            Self::BooleanSetting => "boolean_setting_type",
            Self::EnumSetting => "enum_setting_type",
            Self::NumberSetting => "number_setting_type",
            Self::PathSetting => "path_setting_type",
            Self::SecretReferenceSetting => "secret_reference_setting_type",
            Self::TypeUnclassified => "",
        }
    }

    /// Whether this type is sensitivity-bearing and so must disclose the redaction / sensitivity posture
    /// before the live value is surfaced.
    pub const fn is_sensitive_type(self) -> bool {
        matches!(self, Self::PathSetting | Self::SecretReferenceSetting)
    }
}

/// Controlled winning scope an effective-setting entry must resolve its value from, so an effective setting
/// shares one registry rather than a hand-copied per-record assumption. Minted by this lane, tracking the
/// machine / user / workspace scopes the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EffectiveSettingScope {
    /// The machine scope won the resolution.
    MachineScope,
    /// The user scope won the resolution.
    UserScope,
    /// The workspace scope won the resolution.
    WorkspaceScope,
    /// The winning scope is unclassified, which is disallowed.
    ScopeUnclassified,
}

impl M5EffectiveSettingScope {
    /// Every winning scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MachineScope,
        Self::UserScope,
        Self::WorkspaceScope,
        Self::ScopeUnclassified,
    ];

    /// The three canonical scopes every effective setting must stay distinct across.
    pub const CANONICAL_SCOPES: [Self; 3] =
        [Self::MachineScope, Self::UserScope, Self::WorkspaceScope];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MachineScope => "machine_scope",
            Self::UserScope => "user_scope",
            Self::WorkspaceScope => "workspace_scope",
            Self::ScopeUnclassified => "scope_unclassified",
        }
    }

    /// Whether the winning scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ScopeUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a setting-definition or
/// effective-setting token's meaning stays stable whether it appears in the settings, shell, diagnostics,
/// admin, or a support / export form. Minted by this lane, tracking the first-consumer surfaces the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingSurfaceContext {
    /// The settings surface.
    SettingsSurface,
    /// The shell surface.
    ShellSurface,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5SettingSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SettingsSurface,
        Self::ShellSurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::SettingsSurface,
        Self::ShellSurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsSurface => "settings_surface",
            Self::ShellSurface => "shell_surface",
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

/// One mandatory rendered part a setting-definition or effective-setting entry must be able to show, so no
/// setting type, stable ID, allowed scopes, restart posture, sensitivity class, effective-setting field, or
/// registry fact is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The setting-definition type the entry classifies (setting-definition entry).
    SettingDefinitionType,
    /// The stable setting ID, allowed scopes, and declared default the entry publishes (setting-definition
    /// entry).
    SettingIdAndAllowedScopes,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The restart posture and sensitivity class the entry publishes (setting-definition entry).
    RestartPostureAndSensitivityClass,
    /// The effective-setting fields (resolved value, shadow chain, lock state, validation, restart state,
    /// capability availability) the entry publishes (effective-setting entry).
    EffectiveSettingFields,
    /// The capability-dependency hint the entry publishes (effective-setting entry).
    CapabilityDependencyHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved setting definition or effective setting (both entries).
    PlainLanguageMeaning,
}

impl M5SettingAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::SettingDefinitionType,
        Self::SettingIdAndAllowedScopes,
        Self::ResolutionFormCoverage,
        Self::RestartPostureAndSensitivityClass,
        Self::EffectiveSettingFields,
        Self::CapabilityDependencyHint,
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
            Self::SettingDefinitionType => "setting_definition_type",
            Self::SettingIdAndAllowedScopes => "setting_id_and_allowed_scopes",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::RestartPostureAndSensitivityClass => "restart_posture_and_sensitivity_class",
            Self::EffectiveSettingFields => "effective_setting_fields",
            Self::CapabilityDependencyHint => "capability_dependency_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// setting definition, an effective setting, or a degraded setting-definition / effective-setting entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingNextAction {
    /// Expand the resolved setting definition's or effective setting's plain-language meaning.
    ExpandSettingMeaning,
    /// Inspect the setting-definition type or winning scope the entry resolves.
    InspectTypeOrScope,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5SettingNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandSettingMeaning,
        Self::InspectTypeOrScope,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandSettingMeaning => "expand_setting_meaning",
            Self::InspectTypeOrScope => "inspect_type_or_scope",
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
pub enum M5SettingExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The settings-governance families covered.
    SettingsGovernanceFamilies,
    /// The setting-definition types carried.
    SettingDefinitionTypes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The winning scopes carried.
    EffectiveSettingScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The type modes carried.
    DefinitionTypeModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5SettingExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::SettingDefinitionTypes,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::EffectiveSettingScopes,
        Self::SurfaceContext,
        Self::DefinitionTypeModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::SettingDefinitionTypes,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::SettingsGovernanceFamilies => "settings_governance_families",
            Self::SettingDefinitionTypes => "setting_definition_types",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::EffectiveSettingScopes => "effective_setting_scopes",
            Self::SurfaceContext => "surface_context",
            Self::DefinitionTypeModes => "definition_type_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a setting-definition entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, ID-recycling, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingDefinitionEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the definition means.
    DefinitionTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The setting-definition type is unclassified (not in the resolved taxonomy).
    SettingDefinitionTypeUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    DefinitionNotBoundToRegistry,
    /// The resolved setting-definition object is incomplete: the stable setting ID, allowed scopes, declared
    /// default, migration aliases, restart posture, sensitivity class, or capability dependencies is unstated.
    SettingDefinitionObjectIncomplete,
    /// The stable setting ID was recycled into a different meaning, or a sensitive setting hid its redaction /
    /// sensitivity posture behind generic copy.
    SettingDefinitionRecyclesIdOrHidesSensitivity,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A sensitivity-bearing setting did not disclose the redaction / sensitivity posture before it was
    /// surfaced.
    SensitivityNotDisclosedForSensitiveSetting,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SettingDefinitionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::DefinitionTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::SettingDefinitionTypeUnclassified,
        Self::DefinitionNotBoundToRegistry,
        Self::SettingDefinitionObjectIncomplete,
        Self::SettingDefinitionRecyclesIdOrHidesSensitivity,
        Self::ResolutionFormCoverageIncomplete,
        Self::SensitivityNotDisclosedForSensitiveSetting,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefinitionTokenUnstated => "definition_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::SettingDefinitionTypeUnclassified => "setting_definition_type_unclassified",
            Self::DefinitionNotBoundToRegistry => "definition_not_bound_to_registry",
            Self::SettingDefinitionObjectIncomplete => "setting_definition_object_incomplete",
            Self::SettingDefinitionRecyclesIdOrHidesSensitivity => {
                "setting_definition_recycles_id_or_hides_sensitivity"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::SensitivityNotDisclosedForSensitiveSetting => {
                "sensitivity_not_disclosed_for_sensitive_setting"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SettingNextAction {
        match self {
            Self::DefinitionTokenUnstated | Self::DefinitionNotBoundToRegistry => {
                M5SettingNextAction::TraceCanonicalRegistry
            }
            Self::SettingDefinitionTypeUnclassified
            | Self::SettingDefinitionObjectIncomplete
            | Self::SettingDefinitionRecyclesIdOrHidesSensitivity => {
                M5SettingNextAction::InspectTypeOrScope
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5SettingNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::SensitivityNotDisclosedForSensitiveSetting
            | Self::ProofStale => M5SettingNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::DefinitionTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SettingDefinitionTypeUnclassified | Self::SettingDefinitionObjectIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::DefinitionNotBoundToRegistry => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::SettingDefinitionRecyclesIdOrHidesSensitivity
            | Self::SensitivityNotDisclosedForSensitiveSetting => {
                M5SettingsGovernanceDowngradeTrigger::RecycledARetiredSettingId
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an effective-setting entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EffectiveSettingEntryDegradeReason {
    /// The canonical registry token name is unstated.
    RecordTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The winning scope is unclassified (not in the resolved taxonomy).
    WinningScopeUnclassified,
    /// The effective setting would hide the shadow chain of scopes that lost, mask a locked value without
    /// disclosing its lock source, let machine-only state masquerade as portable, or it dropped one of the
    /// required effective-setting fields (resolved value, shadow chain, lock state, validation, restart state,
    /// capability availability, last-applied revision).
    EffectiveSettingHidesShadowChainOrMasksLockOrMachineState,
    /// The canonical / accessible / audit resolution-form coverage of the record is incomplete.
    RecordFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5EffectiveSettingEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RecordTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::WinningScopeUnclassified,
        Self::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState,
        Self::RecordFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordTokenUnstated => "record_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::WinningScopeUnclassified => "winning_scope_unclassified",
            Self::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState => {
                "effective_setting_hides_shadow_chain_or_masks_lock_or_machine_state"
            }
            Self::RecordFormCoverageIncomplete => "record_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SettingNextAction {
        match self {
            Self::RecordTokenUnstated => M5SettingNextAction::TraceCanonicalRegistry,
            Self::WinningScopeUnclassified
            | Self::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState => {
                M5SettingNextAction::InspectTypeOrScope
            }
            Self::RecordFormCoverageIncomplete => {
                M5SettingNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5SettingNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::RecordTokenUnstated => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::WinningScopeUnclassified => {
                M5SettingsGovernanceDowngradeTrigger::WinningScopeUnstated
            }
            Self::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState => {
                M5SettingsGovernanceDowngradeTrigger::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync
            }
            Self::RecordFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_setting_definition_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingDefinitionEntryResolutionInput {
    /// Stable identity of the setting-definition-registry entry.
    pub entry_id: String,
    /// The stable setting-binding ID this definition binds to (e.g. `settings.acme.editor.font-size`); empty
    /// means unstated.
    pub setting_binding_id: String,
    /// The canonical registry token name (e.g. `setting.definition.editor.font_size`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The setting-definition type this entry classifies.
    pub setting_definition_type: M5SettingDefinitionKind,
    /// The render / surface context.
    pub surface_context: M5SettingSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5SettingResolutionForm>,
    /// The published stable setting ID preserved verbatim; empty means unstated.
    pub stable_setting_id: String,
    /// The published allowed scopes descriptor; empty means unstated.
    pub allowed_scopes: String,
    /// The published declared default; empty means unstated.
    pub declared_default: String,
    /// The published migration aliases; empty means unstated.
    pub migration_aliases: String,
    /// The published restart posture; empty means unstated.
    pub restart_posture: String,
    /// The published sensitivity class kept distinct; empty means unstated.
    pub sensitivity_class: String,
    /// The published capability dependencies; empty means unstated.
    pub capability_dependencies: String,
    /// True when the behavior traces to the setting-definition registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the stable setting ID is preserved and never recycled into a different meaning (a hard
    /// invariant when `false`).
    pub setting_id_preserved: bool,
    /// True when this setting is sensitivity-bearing.
    pub is_sensitive_setting: bool,
    /// True when the redaction / sensitivity posture is disclosed before the sensitive setting is surfaced.
    pub sensitivity_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe setting-definition-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSettingDefinitionEntry {
    /// Stable identity of the setting-definition-registry entry.
    pub entry_id: String,
    /// The stable setting-binding ID this definition binds to.
    pub setting_binding_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The setting-definition-type token named by the entry.
    pub setting_definition_type: String,
    /// Whether the setting-definition type is classified into the resolved taxonomy.
    pub setting_definition_type_is_classified: bool,
    /// The canonical type mode for the entry's type.
    pub canonical_type_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published stable setting ID.
    pub stable_setting_id: String,
    /// The published allowed scopes descriptor.
    pub allowed_scopes: String,
    /// The published declared default.
    pub declared_default: String,
    /// The published migration aliases.
    pub migration_aliases: String,
    /// The published restart posture.
    pub restart_posture: String,
    /// The published sensitivity class.
    pub sensitivity_class: String,
    /// The published capability dependencies.
    pub capability_dependencies: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved setting-definition object publishes every required field.
    pub setting_definition_object_complete: bool,
    /// Whether the entry traces to the setting-definition registry.
    pub bound_to_registry: bool,
    /// Whether the stable setting ID stays preserved (never recycled).
    pub setting_id_preserved: bool,
    /// Whether this setting is sensitivity-bearing.
    pub is_sensitive_setting: bool,
    /// Whether the redaction / sensitivity posture is disclosed before the sensitive setting is surfaced.
    pub sensitivity_disclosed: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5SettingDefinitionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SettingNextAction,
    /// Whether the definition resolves to one stable object across every claimed setting (clean entry naming
    /// every fact).
    pub definition_resolves_across_settings: bool,
}

impl M5ResolvedSettingDefinitionEntry {
    /// Whether this setting-definition entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_effective_setting_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EffectiveSettingEntryResolutionInput {
    /// Stable identity of the effective-setting entry.
    pub entry_id: String,
    /// The stable setting-ref this record binds to; empty means unstated.
    pub setting_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The winning scope this record must resolve its value from.
    pub winning_scope: M5EffectiveSettingScope,
    /// The render / surface context.
    pub surface_context: M5SettingSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5SettingResolutionForm>,
    /// The published resolved value or redacted summary; empty means missing.
    pub resolved_value_summary: String,
    /// The published shadow chain of scopes that lost; empty means missing.
    pub shadow_chain: String,
    /// The published lock or constraint state; empty means missing.
    pub lock_or_constraint_state: String,
    /// The published validation status; empty means missing.
    pub validation_status: String,
    /// The published restart state; empty means missing.
    pub restart_state: String,
    /// The published capability availability; empty means missing.
    pub capability_availability: String,
    /// The published last-applied revision; empty means missing.
    pub last_applied_revision: String,
    /// True when the record keeps the shadow chain of scopes that lost visible.
    pub keeps_shadow_chain_visible: bool,
    /// True when the resolution is truthful (never claims a clean resolution over a hidden shadow chain).
    pub resolution_is_truthful: bool,
    /// True when the resolved value is locked or constrained.
    pub lock_present: bool,
    /// True when a locked value discloses its lock source (never masks the lock).
    pub lock_source_disclosed: bool,
    /// True when the resolved value carries machine-only state.
    pub machine_only_value_present: bool,
    /// True when machine-only state is flagged non-portable rather than masquerading as portable.
    pub machine_only_flagged_not_portable: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe effective-setting projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEffectiveSettingEntry {
    /// Stable identity of the effective-setting entry.
    pub entry_id: String,
    /// The stable setting-ref this record binds to.
    pub setting_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The winning-scope token named by the entry.
    pub winning_scope: String,
    /// Whether the winning scope is classified into the resolved taxonomy.
    pub winning_scope_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published resolved value or redacted summary.
    pub resolved_value_summary: String,
    /// The published shadow chain of scopes that lost.
    pub shadow_chain: String,
    /// The published lock or constraint state.
    pub lock_or_constraint_state: String,
    /// The published validation status.
    pub validation_status: String,
    /// The published restart state.
    pub restart_state: String,
    /// The published capability availability.
    pub capability_availability: String,
    /// The published last-applied revision.
    pub last_applied_revision: String,
    /// Whether the record keeps the shadow chain visible.
    pub keeps_shadow_chain_visible: bool,
    /// Whether the resolution is truthful.
    pub resolution_is_truthful: bool,
    /// Whether the resolved value is locked or constrained.
    pub lock_present: bool,
    /// Whether a locked value discloses its lock source.
    pub lock_source_disclosed: bool,
    /// Whether the resolved value carries machine-only state.
    pub machine_only_value_present: bool,
    /// Whether machine-only state is flagged non-portable.
    pub machine_only_flagged_not_portable: bool,
    /// Whether the record stays honest (shadow chain visible, lock source disclosed, machine-only flagged).
    pub effective_setting_stays_honest: bool,
    /// Whether the entry provides the complete effective-setting object (resolved value, shadow chain, lock
    /// state, validation, restart state, capability availability, last-applied revision).
    pub provides_complete_effective_setting: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5EffectiveSettingEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SettingNextAction,
    /// Whether the effective setting is safe on every claimed setting (clean entry naming every fact).
    pub record_safe_on_every_setting: bool,
}

impl M5ResolvedEffectiveSettingEntry {
    /// Whether this effective-setting entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5SettingResolutionError {
    /// The setting-definition-entry id was empty.
    EmptySettingDefinitionEntryId,
    /// The effective-setting-entry id was empty.
    EmptyEffectiveSettingEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5SettingResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySettingDefinitionEntryId => "empty_setting_definition_entry_id",
            Self::EmptyEffectiveSettingEntryId => "empty_effective_setting_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5SettingResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 setting-definition / effective-setting registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SettingResolutionError {}

fn form_tokens(forms: &[M5SettingResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5SettingResolutionForm]) -> bool {
    let present: BTreeSet<M5SettingResolutionForm> = forms.iter().copied().collect();
    M5SettingResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved setting-definition object publishes every required field: declared type (via a
/// classified kind), stable setting ID, allowed scopes, declared default, migration aliases, restart posture,
/// sensitivity class, and capability dependencies. An unclassified type or any empty field never resolves to a
/// complete object.
#[allow(clippy::too_many_arguments)]
pub fn setting_definition_object_is_complete(
    kind: M5SettingDefinitionKind,
    stable_setting_id: &str,
    allowed_scopes: &str,
    declared_default: &str,
    migration_aliases: &str,
    restart_posture: &str,
    sensitivity_class: &str,
    capability_dependencies: &str,
) -> bool {
    kind.is_classified()
        && !stable_setting_id.trim().is_empty()
        && !allowed_scopes.trim().is_empty()
        && !declared_default.trim().is_empty()
        && !migration_aliases.trim().is_empty()
        && !restart_posture.trim().is_empty()
        && !sensitivity_class.trim().is_empty()
        && !capability_dependencies.trim().is_empty()
}

/// Whether the setting definition keeps a stable, non-recycled, sensitivity-disclosing identity: the type must
/// be classified, the stable setting ID must be preserved (never recycled into a different meaning), and a
/// sensitivity-bearing setting must disclose the redaction / sensitivity posture before it is surfaced. An
/// unclassified type, a recycled ID, or a hidden sensitivity posture never matches.
pub fn stable_setting_id_stays_non_recycled(
    kind: M5SettingDefinitionKind,
    setting_id_preserved: bool,
    is_sensitive_setting: bool,
    sensitivity_disclosed: bool,
) -> bool {
    kind.is_classified() && setting_id_preserved && (!is_sensitive_setting || sensitivity_disclosed)
}

/// Whether an effective setting stays honest: the scope must be classified, the resolution must be truthful,
/// it must keep the shadow chain of scopes that lost visible, any locked value must disclose its lock source
/// rather than mask it, and any machine-only state must be flagged non-portable rather than masquerade as
/// portable.
pub fn effective_setting_stays_honest(
    scope: M5EffectiveSettingScope,
    resolution_is_truthful: bool,
    keeps_shadow_chain_visible: bool,
    lock_present: bool,
    lock_source_disclosed: bool,
    machine_only_value_present: bool,
    machine_only_flagged_not_portable: bool,
) -> bool {
    scope.is_classified()
        && resolution_is_truthful
        && keeps_shadow_chain_visible
        && (!lock_present || lock_source_disclosed)
        && (!machine_only_value_present || machine_only_flagged_not_portable)
}

/// Resolves a setting-definition-registry entry so it stays bound to the setting-definition registry: the
/// entry names its canonical token, semantic role, and setting-definition type, covers all three resolution
/// forms, publishes a complete setting-definition object (stable setting ID, allowed scopes, declared default,
/// migration aliases, restart posture, sensitivity class, capability dependencies), preserves the stable
/// setting ID as a non-recycled identity, and discloses the sensitivity posture before a sensitive setting is
/// surfaced.
pub fn resolve_setting_definition_entry(
    input: M5SettingDefinitionEntryResolutionInput,
) -> Result<M5ResolvedSettingDefinitionEntry, M5SettingResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SettingResolutionError::EmptySettingDefinitionEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.setting_binding_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.stable_setting_id)
        || string_is_forbidden(&input.allowed_scopes)
        || string_is_forbidden(&input.declared_default)
        || string_is_forbidden(&input.migration_aliases)
        || string_is_forbidden(&input.restart_posture)
        || string_is_forbidden(&input.sensitivity_class)
        || string_is_forbidden(&input.capability_dependencies)
    {
        return Err(M5SettingResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = setting_definition_object_is_complete(
        input.setting_definition_type,
        &input.stable_setting_id,
        &input.allowed_scopes,
        &input.declared_default,
        &input.migration_aliases,
        &input.restart_posture,
        &input.sensitivity_class,
        &input.capability_dependencies,
    );
    let non_recycled_ok = stable_setting_id_stays_non_recycled(
        input.setting_definition_type,
        input.setting_id_preserved,
        input.is_sensitive_setting,
        input.sensitivity_disclosed,
    );
    let sensitivity_undisclosed = input.is_sensitive_setting && !input.sensitivity_disclosed;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SettingDefinitionEntryDegradeReason::DefinitionTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SettingDefinitionEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.setting_definition_type.is_classified() {
        Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionTypeUnclassified)
    } else if !input.bound_to_registry {
        Some(M5SettingDefinitionEntryDegradeReason::DefinitionNotBoundToRegistry)
    } else if !object_complete {
        Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionObjectIncomplete)
    } else if !non_recycled_ok {
        Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionRecyclesIdOrHidesSensitivity)
    } else if !all_forms {
        Some(M5SettingDefinitionEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if sensitivity_undisclosed {
        Some(M5SettingDefinitionEntryDegradeReason::SensitivityNotDisclosedForSensitiveSetting)
    } else if !input.proof_fresh {
        Some(M5SettingDefinitionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SettingNextAction::ExpandSettingMeaning,
    };

    Ok(M5ResolvedSettingDefinitionEntry {
        entry_id: input.entry_id,
        setting_binding_id: input.setting_binding_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        setting_definition_type: input.setting_definition_type.as_str().to_owned(),
        setting_definition_type_is_classified: input.setting_definition_type.is_classified(),
        canonical_type_mode: input
            .setting_definition_type
            .canonical_type_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        stable_setting_id: input.stable_setting_id,
        allowed_scopes: input.allowed_scopes,
        declared_default: input.declared_default,
        migration_aliases: input.migration_aliases,
        restart_posture: input.restart_posture,
        sensitivity_class: input.sensitivity_class,
        capability_dependencies: input.capability_dependencies,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        setting_definition_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        setting_id_preserved: input.setting_id_preserved,
        is_sensitive_setting: input.is_sensitive_setting,
        sensitivity_disclosed: input.sensitivity_disclosed,
        degrade_reason,
        next_action,
        definition_resolves_across_settings: degrade_reason.is_none(),
    })
}

/// Resolves an effective-setting entry so its resolution stays safe: the entry names its canonical token,
/// semantic role, and winning scope, covers all three resolution forms, provides the complete resolved-value /
/// shadow-chain / lock-state / validation-status / restart-state / capability-availability /
/// last-applied-revision effective-setting object, and degrades honestly when the record would hide the shadow
/// chain of scopes that lost, mask a locked value, or let machine-only state masquerade as portable.
pub fn resolve_effective_setting_entry(
    input: M5EffectiveSettingEntryResolutionInput,
) -> Result<M5ResolvedEffectiveSettingEntry, M5SettingResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SettingResolutionError::EmptyEffectiveSettingEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.setting_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.resolved_value_summary)
        || string_is_forbidden(&input.shadow_chain)
        || string_is_forbidden(&input.lock_or_constraint_state)
        || string_is_forbidden(&input.validation_status)
        || string_is_forbidden(&input.restart_state)
        || string_is_forbidden(&input.capability_availability)
        || string_is_forbidden(&input.last_applied_revision)
    {
        return Err(M5SettingResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = effective_setting_stays_honest(
        input.winning_scope,
        input.resolution_is_truthful,
        input.keeps_shadow_chain_visible,
        input.lock_present,
        input.lock_source_disclosed,
        input.machine_only_value_present,
        input.machine_only_flagged_not_portable,
    );
    let provides_record = input.winning_scope.is_classified()
        && !input.resolved_value_summary.trim().is_empty()
        && !input.shadow_chain.trim().is_empty()
        && !input.lock_or_constraint_state.trim().is_empty()
        && !input.validation_status.trim().is_empty()
        && !input.restart_state.trim().is_empty()
        && !input.capability_availability.trim().is_empty()
        && !input.last_applied_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5EffectiveSettingEntryDegradeReason::RecordTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5EffectiveSettingEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.winning_scope.is_classified() {
        Some(M5EffectiveSettingEntryDegradeReason::WinningScopeUnclassified)
    } else if !provides_record {
        Some(M5EffectiveSettingEntryDegradeReason::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState)
    } else if !all_forms {
        Some(M5EffectiveSettingEntryDegradeReason::RecordFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5EffectiveSettingEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SettingNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedEffectiveSettingEntry {
        entry_id: input.entry_id,
        setting_ref: input.setting_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        winning_scope: input.winning_scope.as_str().to_owned(),
        winning_scope_is_classified: input.winning_scope.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        resolved_value_summary: input.resolved_value_summary,
        shadow_chain: input.shadow_chain,
        lock_or_constraint_state: input.lock_or_constraint_state,
        validation_status: input.validation_status,
        restart_state: input.restart_state,
        capability_availability: input.capability_availability,
        last_applied_revision: input.last_applied_revision,
        keeps_shadow_chain_visible: input.keeps_shadow_chain_visible,
        resolution_is_truthful: input.resolution_is_truthful,
        lock_present: input.lock_present,
        lock_source_disclosed: input.lock_source_disclosed,
        machine_only_value_present: input.machine_only_value_present,
        machine_only_flagged_not_portable: input.machine_only_flagged_not_portable,
        effective_setting_stays_honest: record_stays_honest,
        provides_complete_effective_setting: provides_record,
        degrade_reason,
        next_action,
        record_safe_on_every_setting: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved setting-definition and effective-setting
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingDefinitionEffectiveSettingRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SettingDefinitionEffectiveSettingRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5SettingAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5SettingExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    /// Resolved setting-definition-registry examples.
    pub setting_definition_entries: Vec<M5ResolvedSettingDefinitionEntry>,
    /// Resolved effective-setting examples.
    pub effective_setting_entries: Vec<M5ResolvedEffectiveSettingEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the setting-definition and
    /// effective-setting domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never recycles a retired setting ID. MUST be `false`.
    pub recycles_a_retired_setting_id: bool,
    /// Hard invariant: this row never resolves an effective value without an inspectable shadow chain. MUST be
    /// `false`.
    pub resolves_an_effective_value_without_an_inspectable_shadow_chain: bool,
    /// Hard invariant: this row never hides restart posture, lock source, or sensitivity before the
    /// resolution. MUST be `false`.
    pub hides_restart_posture_lock_source_or_sensitivity_before_resolution: bool,
    /// Hard invariant: this row never collapses distinct settings scopes into one resolution path. MUST be
    /// `false`.
    pub collapses_distinct_settings_scopes_into_one_resolution_path: bool,
}

impl M5SettingDefinitionEffectiveSettingRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SettingAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5SettingAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5SettingExportField> = self.export_fields.iter().copied().collect();
        M5SettingExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.recycles_a_retired_setting_id
            && !self.resolves_an_effective_value_without_an_inspectable_shadow_chain
            && !self.hides_restart_posture_lock_source_or_sensitivity_before_resolution
            && !self.collapses_distinct_settings_scopes_into_one_resolution_path
    }

    /// True when a clean setting-definition entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified setting-definition type, publishes a complete definition object, preserves the
    /// stable setting ID, covers all three resolution forms, and discloses the sensitivity posture for a
    /// sensitive setting.
    fn definition_is_honest(ex: &M5ResolvedSettingDefinitionEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.setting_definition_type_is_classified
                && ex.setting_definition_object_complete
                && ex.setting_id_preserved
                && ex.covers_all_resolution_forms
                && (!ex.is_sensitive_setting || ex.sensitivity_disclosed))
    }

    /// True when a clean effective-setting entry preserves a safe record: it keeps a classified scope,
    /// provides the complete effective-setting object, stays honest, and covers all three resolution forms.
    fn record_is_honest(ex: &M5ResolvedEffectiveSettingEntry) -> bool {
        !ex.is_clean()
            || (ex.winning_scope_is_classified
                && ex.provides_complete_effective_setting
                && ex.effective_setting_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.setting_definition_entries
            .iter()
            .all(Self::definition_is_honest)
            && self
                .effective_setting_entries
                .iter()
                .all(Self::record_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingDefinitionEffectiveSettingRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Setting-definition-type tokens (minted by this lane).
    pub setting_definition_types: Vec<String>,
    /// Winning-scope tokens (minted by this lane).
    pub effective_setting_scopes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Setting-definition-entry degrade-reason tokens.
    pub setting_definition_degrade_reasons: Vec<String>,
    /// Effective-setting-entry degrade-reason tokens.
    pub effective_setting_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SettingDefinitionEffectiveSettingRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5SettingsGovernanceRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5SettingResolutionForm::ALL, |v| v.as_str()),
            setting_definition_types: tokens(&M5SettingDefinitionKind::ALL, |v| v.as_str()),
            effective_setting_scopes: tokens(&M5EffectiveSettingScope::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5SettingSurfaceContext::ALL, |v| v.as_str()),
            setting_definition_degrade_reasons: tokens(
                &M5SettingDefinitionEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            effective_setting_degrade_reasons: tokens(
                &M5EffectiveSettingEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5SettingAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5SettingNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5SettingExportField::ALL, |v| v.as_str()),
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
pub struct M5SettingDefinitionEffectiveSettingRegistriesGovernanceReview {
    /// The definition registry names a canonical token, semantic role, and setting-definition type for every
    /// entry.
    pub definition_registry_names_token_role_and_type: bool,
    /// Every claimed setting resolves to one stable setting-definition object from the shared registry, not
    /// per-entry reconstruction.
    pub setting_resolves_to_stable_object_from_shared_registry: bool,
    /// The stable setting ID, allowed scopes, declared default, migration aliases, restart posture,
    /// sensitivity class, and capability dependencies are published for every resolved definition.
    pub setting_id_type_scopes_default_and_sensitivity_published: bool,
    /// Stable setting IDs stay non-recycled; a stable setting ID is never recycled into a different meaning.
    pub stable_setting_ids_stay_non_recycled: bool,
    /// The effective record keeps the shadow chain of scopes that lost visible and discloses the lock source.
    pub effective_record_keeps_shadow_chain_visible_and_discloses_lock_source: bool,
    /// The redaction / sensitivity posture is disclosed before any sensitive setting is surfaced.
    pub sensitivity_disclosed_for_sensitive_settings: bool,
    /// Every setting-definition and effective-setting entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Setting-definition and effective-setting behavior stay bound to the shared registries rather than
    /// hand-copied per setting.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Settings, shell, diagnostics, and admin read a single configuration source.
    pub settings_shell_diagnostics_admin_read_single_source: bool,
    /// A recycled ID, an incomplete object, or a hidden shadow chain is caught by fixtures before release
    /// evidence turns green.
    pub definition_or_record_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingDefinitionEffectiveSettingRegistriesConsumerProjection {
    /// Settings and shell consume the shared setting-definition registry.
    pub settings_and_shell_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared effective-setting registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Sync and capability services consume the shared registries.
    pub sync_and_capability_services_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical setting-definition and effective-setting domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical setting-definition / effective-setting registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingDefinitionEffectiveSettingRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingDefinitionEffectiveSettingRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting settings-governance audit for the lane.
    pub settings_governance_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SettingDefinitionEffectiveSettingRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingDefinitionEffectiveSettingRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingDefinitionEffectiveSettingRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingDefinitionEffectiveSettingRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingDefinitionEffectiveSettingRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingDefinitionEffectiveSettingRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingDefinitionEffectiveSettingRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingDefinitionEffectiveSettingRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 setting-definition and effective-setting registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingDefinitionEffectiveSettingRegistriesPacket {
    /// Record kind; must equal [`M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingDefinitionEffectiveSettingRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingDefinitionEffectiveSettingRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingDefinitionEffectiveSettingRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingDefinitionEffectiveSettingRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingDefinitionEffectiveSettingRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingDefinitionEffectiveSettingRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SettingDefinitionEffectiveSettingRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SettingDefinitionEffectiveSettingRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_RECORD_KIND {
            violations
                .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_SCHEMA_VERSION
        {
            violations
                .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations
                .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 setting-definition / effective-setting registries packet serializes"),
        ) {
            violations
                .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 setting-definition / effective-setting registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,setting_definition_entries,effective_setting_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .setting_definition_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.effective_setting_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.setting_definition_entries.len(),
                row.effective_setting_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Setting-Definition and Effective-Setting Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Setting-definition types: {}\n",
            self.vocabulary_set.setting_definition_types.join(", ")
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
                "  - Setting-definition entries: {} / effective-setting entries: {}\n",
                row.setting_definition_entries.len(),
                row.effective_setting_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry settings reference table generated from the registry, so docs and admin
    /// runbooks render the same type-mode / stable-setting-id / allowed-scopes / default / restart-posture /
    /// sensitivity-class truth the resolvers produced rather than a hand-copied settings table. Only clean,
    /// registry-bound setting-definition entries are listed.
    pub fn render_setting_definition_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| setting_binding_id | type_mode | stable_setting_id | allowed_scopes | declared_default | restart_posture | sensitivity_class |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.setting_definition_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.setting_binding_id,
                    ex.canonical_type_mode,
                    ex.stable_setting_id,
                    ex.allowed_scopes,
                    ex.declared_default,
                    ex.restart_posture,
                    ex.sensitivity_class
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SettingDefinitionEffectiveSettingRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation>),
}

impl fmt::Display for M5SettingDefinitionEffectiveSettingRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 setting-definition / effective-setting registries export parse failed: {error}"
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
                    "m5 setting-definition / effective-setting registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SettingDefinitionEffectiveSettingRegistriesArtifactError {}

/// Validation failures emitted by [`M5SettingDefinitionEffectiveSettingRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SettingDefinitionEffectiveSettingRegistriesViolation {
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
    /// A registry row does not point at both the setting-definition and effective-setting domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, ID-recycling, field-incomplete,
    /// form-incomplete, or an effective-setting entry missing the complete record object).
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
    /// Setting-definition-resolution is not proven: clean setting-definition entries do not cover the canonical
    /// setting-definition types or the first settings / shell / diagnostics / admin / support surfaces, no
    /// object-incomplete example degrades, or a clean definition entry published an incomplete object.
    SettingDefinitionResolutionNotProven,
    /// Stable-setting-id-preservation is not proven: no ID-recycle example and no unbound example degrade, no
    /// clean non-recycled definition entry is present, or a clean definition entry recycled the ID or is
    /// unbound.
    StableSettingIdPreservationNotProven,
    /// Effective-setting-integrity is not proven: clean effective-setting entries do not cover the canonical
    /// machine / user / workspace scopes with full resolution-form coverage while providing the complete record
    /// object, no hidden-shadow-chain or form-incomplete example degrades, or a clean effective-setting entry
    /// is missing the complete record object.
    EffectiveSettingIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SettingDefinitionEffectiveSettingRegistriesViolation {
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
            Self::SettingDefinitionResolutionNotProven => {
                "setting_definition_resolution_not_proven"
            }
            Self::StableSettingIdPreservationNotProven => {
                "stable_setting_id_preservation_not_proven"
            }
            Self::EffectiveSettingIntegrityNotProven => "effective_setting_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_setting_definition_and_effective_setting_registries_export() -> Result<
    M5SettingDefinitionEffectiveSettingRegistriesPacket,
    M5SettingDefinitionEffectiveSettingRegistriesArtifactError,
> {
    let packet: M5SettingDefinitionEffectiveSettingRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-setting-definition-and-effective-setting-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SettingDefinitionEffectiveSettingRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SettingDefinitionEffectiveSettingRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SettingDefinitionEffectiveSettingRegistriesPacket,
    violations: &mut Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_SCHEMA_REF,
        M5_SETTING_DEFINITION_EFFECTIVE_SETTING_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
        M5_EFFECTIVE_SETTING_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5SettingDefinitionEffectiveSettingRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SettingDefinitionEffectiveSettingRegistriesPacket,
    violations: &mut Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SettingDefinitionEffectiveSettingRegistriesViolation::NoRegistryRows);
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
                M5SettingDefinitionEffectiveSettingRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5SettingDefinitionEffectiveSettingRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5SettingDefinitionEffectiveSettingRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_EFFECTIVE_SETTING_SCHEMA_REF)
        {
            violations.push(
                M5SettingDefinitionEffectiveSettingRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.setting_definition_entries.is_empty() || row.effective_setting_entries.is_empty() {
            violations
                .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations
                .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SettingDefinitionEffectiveSettingRegistriesPacket,
    violations: &mut Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.definition_registry_names_token_role_and_type,
        review.setting_resolves_to_stable_object_from_shared_registry,
        review.setting_id_type_scopes_default_and_sensitivity_published,
        review.stable_setting_ids_stay_non_recycled,
        review.effective_record_keeps_shadow_chain_visible_and_discloses_lock_source,
        review.sensitivity_disclosed_for_sensitive_settings,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.settings_shell_diagnostics_admin_read_single_source,
        review.definition_or_record_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5SettingDefinitionEffectiveSettingRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SettingDefinitionEffectiveSettingRegistriesPacket,
    violations: &mut Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.settings_and_shell_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.sync_and_capability_services_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5SettingDefinitionEffectiveSettingRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SettingDefinitionEffectiveSettingRegistriesPacket,
    violations: &mut Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations
            .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SettingDefinitionEffectiveSettingRegistriesPacket,
    violations: &mut Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.settings_governance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations
            .push(M5SettingDefinitionEffectiveSettingRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SettingDefinitionEffectiveSettingRegistriesPacket,
    violations: &mut Vec<M5SettingDefinitionEffectiveSettingRegistriesViolation>,
) {
    let definitions = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.setting_definition_entries.iter())
    };
    let records = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.effective_setting_entries.iter())
    };

    // AC1: every claimed setting resolves to one stable setting-definition object with stable-setting-id /
    // allowed-scopes / default / restart-posture / sensitivity fields. Clean definition entries cover the
    // canonical setting-definition types and the first settings / shell / diagnostics / admin / support
    // surfaces, an object-incomplete example degrades, and no clean definition entry published an incomplete
    // object.
    let clean_types: BTreeSet<String> = definitions()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.setting_definition_type.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = definitions()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let types_covered = M5SettingDefinitionKind::CANONICAL_TYPES
        .iter()
        .all(|k| clean_types.contains(k.as_str()));
    let first_surfaces_covered = M5SettingSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = definitions().any(|ex| {
        ex.degrade_reason
            == Some(M5SettingDefinitionEntryDegradeReason::SettingDefinitionObjectIncomplete)
    });
    let no_clean_incomplete =
        !definitions().any(|ex| ex.is_clean() && !ex.setting_definition_object_complete);
    if !(types_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5SettingDefinitionEffectiveSettingRegistriesViolation::SettingDefinitionResolutionNotProven,
        );
    }

    // AC2: the stable setting ID stays non-recycled. An ID-recycle example degrades, an unbound example
    // degrades, at least one clean non-recycled definition entry is present, and no clean definition entry
    // recycled the ID or is unbound.
    let recycle_degrades = definitions().any(|ex| {
        ex.degrade_reason
            == Some(
                M5SettingDefinitionEntryDegradeReason::SettingDefinitionRecyclesIdOrHidesSensitivity,
            )
    });
    let unbound_degrades = definitions().any(|ex| {
        ex.degrade_reason
            == Some(M5SettingDefinitionEntryDegradeReason::DefinitionNotBoundToRegistry)
    });
    let preserving_clean_definition =
        definitions().any(|ex| ex.is_clean() && ex.setting_id_preserved);
    let no_clean_unbound = !definitions().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_recycled = !definitions().any(|ex| ex.is_clean() && !ex.setting_id_preserved);
    if !(recycle_degrades
        && unbound_degrades
        && preserving_clean_definition
        && no_clean_unbound
        && no_clean_recycled)
    {
        violations.push(
            M5SettingDefinitionEffectiveSettingRegistriesViolation::StableSettingIdPreservationNotProven,
        );
    }

    // AC3: the suite fails when an effective setting collapses into a hidden shadow chain. Clean
    // effective-setting entries cover every canonical machine / user / workspace scope with full
    // resolution-form coverage while providing the complete record object, a hidden-shadow-chain example
    // degrades, a form-incomplete example degrades, and no clean effective-setting entry is missing the
    // complete record object.
    let clean_record_scopes: BTreeSet<String> = records()
        .filter(|ex| {
            ex.is_clean()
                && ex.winning_scope_is_classified
                && ex.provides_complete_effective_setting
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.winning_scope.clone())
        .collect();
    let record_scopes_covered = M5EffectiveSettingScope::CANONICAL_SCOPES
        .iter()
        .all(|m| clean_record_scopes.contains(m.as_str()));
    let hidden_shadow_degrades = records().any(|ex| {
        ex.degrade_reason
            == Some(
                M5EffectiveSettingEntryDegradeReason::EffectiveSettingHidesShadowChainOrMasksLockOrMachineState,
            )
    });
    let form_incomplete_degrades = records().any(|ex| {
        ex.degrade_reason
            == Some(M5EffectiveSettingEntryDegradeReason::RecordFormCoverageIncomplete)
    });
    let no_clean_missing_record =
        !records().any(|ex| ex.is_clean() && !ex.provides_complete_effective_setting);
    if !(record_scopes_covered
        && hidden_shadow_degrades
        && form_incomplete_degrades
        && no_clean_missing_record)
    {
        violations.push(
            M5SettingDefinitionEffectiveSettingRegistriesViolation::EffectiveSettingIntegrityNotProven,
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
pub const IMPLEMENTED_FAMILIES: [M5SettingsGovernanceFamily; 2] = [
    M5SettingsGovernanceFamily::ResolveSetting,
    M5SettingsGovernanceFamily::MigrateSchema,
];
