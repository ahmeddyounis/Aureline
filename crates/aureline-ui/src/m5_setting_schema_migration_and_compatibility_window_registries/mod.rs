//! Implemented M5 schema-migration-record and compatibility-window registries.
//!
//! The frozen [settings-governance matrix][matrix] names Aureline's five configuration-runtime families and
//! locks their controlled vocabulary. This is the schema-migration + downgrade implement lane over the
//! `migrate_schema` family: it turns the *schema-migration-record* grammar (how a configuration artifact
//! declares the old key / alias, new key, transform, lossy fidelity, compatibility window, and rollback note a
//! version change carries) and the *compatibility-window* grammar (how an upgrade, import, restore, or
//! downgrade flow discloses whether the stored meaning is inside its compatibility window, deprecated but
//! supported, or outside the window before it applies) into registry resolvers that produce export-safe, honest
//! projections. Every claimed M5 configuration migration then resolves to one schema-migration-record object —
//! the fidelity label it classifies (exact / compatible / lossy / manual-review), the old key / alias, the new
//! key, the transform, the compatibility window, the rollback note, the compare-before-apply reference, and the
//! migration provenance reference — and to one compatibility-window object — the window source, the supported
//! version range, the deprecation review, the validation status, the review state, the docs pointer, and the
//! last review revision — that the upgrade, import, restore, downgrade, and support / export flows can inspect
//! before apply without manual reconstruction, so a migration never implies full fidelity when it is lossy or
//! requires manual review, a schema change never alters stored meaning without a checked-in migration record and
//! compare surface, a compatibility window always names its window source and downgrade guidance, and a
//! configuration flow that cannot explain what a migration changes degrades honestly instead of reading as a
//! clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one schema-migration-record object per version change.** [`resolve_schema_migration_record_entry`]
//!   refuses to read as a clean, registry-bound migration entry unless it names a canonical registry token, a
//!   classified [fidelity label][M5SchemaMigrationFidelityClass], a settings-governance role, covers every
//!   [resolution form][M5ConfigMigrationResolutionForm] (the canonical object, the accessible summary, and the
//!   audit record), publishes every record field (old key / alias, new key, transform, compatibility window,
//!   rollback note, compare-before-apply reference, and migration provenance reference), keeps its fidelity
//!   label honest, and materializes the compare-before-apply surface before a lossy or manual-review migration
//!   applies; otherwise it degrades.
//! * **Keep the migration from overstating fidelity or hiding its compare surface.**
//!   [`migration_does_not_overstate_fidelity`] rejects a migration entry whose fidelity label overstates what the
//!   transform actually preserves so it degrades to
//!   [`M5SchemaMigrationRecordEntryDegradeReason::MigrationOverstatesFidelityOrHidesCompareSurface`], and a lossy
//!   or manual-review migration that has not materialized its compare-before-apply surface degrades the same
//!   way.
//! * **Keep the compatibility window from masking its window source or hiding the downgrade guidance.**
//!   [`resolve_compatibility_window_entry`] names a classified [window class][M5CompatibilityWindowClass],
//!   requires the full window-source / supported-version-range / deprecation-review / validation-status /
//!   review-state / docs-pointer / last-review-revision compatibility-window object, covers every resolution
//!   form, and degrades to
//!   [`M5CompatibilityWindowEntryDegradeReason::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance`]
//!   when the record would mask a deprecated window without disclosing its window source or leave an
//!   outside-window migration without disclosing the downgrade guidance, so a deprecated or unsupported
//!   migration can never read as trustworthy when it has quietly dropped the reason it is out of window or the
//!   downgrade route the user still has.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5SettingsGovernanceRole`] role vocabulary
//! and the [`M5SettingsGovernanceConsumerSurface`] consumer-surface taxonomy — so the settings, shell,
//! diagnostics, admin, sync, policy, capability, docs, CLI, and support surfaces can never fork their own
//! migration or compatibility-window meaning. Raw secret values and private endpoints stay outside the export
//! boundary.
//!
//! [matrix]: crate::m5_settings_governance_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_setting_schema_migration_and_compatibility_window_registries,
    seeded_m5_setting_schema_migration_and_compatibility_window_registries_compatibility_window_preview_narrowed,
    seeded_m5_setting_schema_migration_and_compatibility_window_registries_schema_migration_beta_narrowed,
    M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_PACKET_ID,
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
    M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF, M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
    M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket`].
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_setting_schema_migration_and_compatibility_window_registries";

/// Schema version for M5 schema-migration / compatibility-window registry records.
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_SCHEMA_REF: &str =
    "schemas/config/m5-setting-schema-migration-and-compatibility-window-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_DOC_REF: &str =
    "docs/settings/m5_setting_schema_migration_and_compatibility_window_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-setting-schema-migration-and-compatibility-window-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-setting-schema-migration-and-compatibility-window-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-setting-schema-migration-and-compatibility-window-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/config/m5-setting-schema-migration-and-compatibility-window-registries";

/// Repo-relative path of the already-landed schema-migration-record schema the migration registry binds back
/// to, so a version change's old key / alias, new key, transform, lossy flag, compatibility window, and rollback
/// note trace to one canonical migration contract rather than a lane-local invention.
pub const M5_SCHEMA_MIGRATION_LANDED_SCHEMA_REF: &str =
    "schemas/governance/schema_migration_record.schema.json";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5SettingSchemaMigrationCompatibilityWindowRegistriesConsumerSurface =
    M5SettingsGovernanceConsumerSurface;

/// One of the three resolution forms every schema-migration or compatibility-window entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the migrate-schema
/// *family* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigMigrationResolutionForm {
    /// The canonical resolved schema-migration / compatibility-window object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved migration discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved migration inspectable off-renderer.
    AuditRecord,
}

impl M5ConfigMigrationResolutionForm {
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

/// Controlled migration fidelity label a schema-migration-record entry declares, so the migration model shares
/// one registry rather than a hand-copied per-version assumption of how faithfully a change preserves stored
/// meaning. Minted by this lane because the frozen matrix carries the configuration families but not the
/// concrete exact / compatible / lossy / manual-review fidelity label a migration classifies against. Every
/// classified label carries its canonical label mode, and the lossy / manual-review labels lose or need review
/// of stored meaning so they must materialize a compare-before-apply surface before the migration applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SchemaMigrationFidelityClass {
    /// An exact migration: the stored meaning is preserved verbatim.
    ExactMigration,
    /// A compatible migration: the stored meaning is preserved under a safe, reversible transform.
    CompatibleMigration,
    /// A lossy migration: some stored meaning cannot be preserved (compare-surface-bearing).
    LossyMigration,
    /// A manual-review migration: the change needs explicit human review before it can apply
    /// (compare-surface-bearing).
    ManualReviewMigration,
    /// The fidelity label is unclassified, which is disallowed.
    FidelityClassUnclassified,
}

impl M5SchemaMigrationFidelityClass {
    /// Every fidelity label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExactMigration,
        Self::CompatibleMigration,
        Self::LossyMigration,
        Self::ManualReviewMigration,
        Self::FidelityClassUnclassified,
    ];

    /// The four canonical fidelity labels every claimed M5 migration classifies against.
    pub const CANONICAL_CLASSES: [Self; 4] = [
        Self::ExactMigration,
        Self::CompatibleMigration,
        Self::LossyMigration,
        Self::ManualReviewMigration,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactMigration => "exact_migration",
            Self::CompatibleMigration => "compatible_migration",
            Self::LossyMigration => "lossy_migration",
            Self::ManualReviewMigration => "manual_review_migration",
            Self::FidelityClassUnclassified => "fidelity_class_unclassified",
        }
    }

    /// Whether the label is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::FidelityClassUnclassified)
    }

    /// The canonical label mode for this fidelity label.
    pub const fn canonical_class_mode(self) -> &'static str {
        match self {
            Self::ExactMigration => "exact_migration_label",
            Self::CompatibleMigration => "compatible_migration_label",
            Self::LossyMigration => "lossy_migration_label",
            Self::ManualReviewMigration => "manual_review_migration_label",
            Self::FidelityClassUnclassified => "",
        }
    }

    /// Whether this label loses or defers stored meaning and so must materialize a compare-before-apply surface
    /// before the migration applies.
    pub const fn is_lossy_or_manual_review(self) -> bool {
        matches!(self, Self::LossyMigration | Self::ManualReviewMigration)
    }
}

/// Controlled compatibility-window class a compatibility-window entry must resolve, so a deprecated or
/// unsupported migration shares one registry rather than a hand-copied per-record assumption. Minted by this
/// lane, tracking the within-window / deprecated-but-supported / outside-window dispositions the acceptance
/// criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityWindowClass {
    /// The migration is inside its compatibility window and fully supported.
    WithinCompatibilityWindow,
    /// The migration is deprecated but still supported with disclosed window source.
    DeprecatedButSupported,
    /// The migration is outside its compatibility window and needs disclosed downgrade guidance.
    OutsideCompatibilityWindow,
    /// The window class is unclassified, which is disallowed.
    WindowClassUnclassified,
}

impl M5CompatibilityWindowClass {
    /// Every window class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::WithinCompatibilityWindow,
        Self::DeprecatedButSupported,
        Self::OutsideCompatibilityWindow,
        Self::WindowClassUnclassified,
    ];

    /// The three canonical window classes every compatibility window must stay distinct across.
    pub const CANONICAL_CLASSES: [Self; 3] = [
        Self::WithinCompatibilityWindow,
        Self::DeprecatedButSupported,
        Self::OutsideCompatibilityWindow,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinCompatibilityWindow => "within_compatibility_window",
            Self::DeprecatedButSupported => "deprecated_but_supported",
            Self::OutsideCompatibilityWindow => "outside_compatibility_window",
            Self::WindowClassUnclassified => "window_class_unclassified",
        }
    }

    /// Whether the window class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::WindowClassUnclassified)
    }
}

/// Controlled render context — which claimed M5 flow renders the registry entry, so a schema-migration or
/// compatibility-window token's meaning stays stable whether it appears before apply in an upgrade, import,
/// restore, or downgrade flow, or in a support / export form. Minted by this lane, tracking the first-consumer
/// flows the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigMigrationSurfaceContext {
    /// The upgrade flow.
    UpgradeFlow,
    /// The import flow.
    ImportFlow,
    /// The restore flow.
    RestoreFlow,
    /// The downgrade flow.
    DowngradeFlow,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ConfigMigrationSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UpgradeFlow,
        Self::ImportFlow,
        Self::RestoreFlow,
        Self::DowngradeFlow,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer flows the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::UpgradeFlow,
        Self::ImportFlow,
        Self::RestoreFlow,
        Self::DowngradeFlow,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpgradeFlow => "upgrade_flow",
            Self::ImportFlow => "import_flow",
            Self::RestoreFlow => "restore_flow",
            Self::DowngradeFlow => "downgrade_flow",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a schema-migration or compatibility-window entry must be able to show, so no
/// fidelity label, old / new key, compare surface, compatibility-window field, or registry fact is left implicit
/// behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigMigrationAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The fidelity label the entry classifies (schema-migration entry).
    MigrationFidelityLabel,
    /// The old key / alias and new key the migration carries (schema-migration entry).
    OldAndNewKey,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The compare-before-apply reference and migration provenance reference the entry publishes
    /// (schema-migration entry).
    CompareSurfaceAndProvenance,
    /// The compatibility-window fields (window source, supported version range, deprecation review, validation,
    /// review state, docs pointer) the entry publishes (compatibility-window entry).
    CompatibilityWindowFields,
    /// The downgrade-guidance hint the entry publishes (compatibility-window entry).
    DowngradeGuidanceHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved migration or compatibility window (both entries).
    PlainLanguageMeaning,
}

impl M5ConfigMigrationAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::MigrationFidelityLabel,
        Self::OldAndNewKey,
        Self::ResolutionFormCoverage,
        Self::CompareSurfaceAndProvenance,
        Self::CompatibilityWindowFields,
        Self::DowngradeGuidanceHint,
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
            Self::MigrationFidelityLabel => "migration_fidelity_label",
            Self::OldAndNewKey => "old_and_new_key",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::CompareSurfaceAndProvenance => "compare_surface_and_provenance",
            Self::CompatibilityWindowFields => "compatibility_window_fields",
            Self::DowngradeGuidanceHint => "downgrade_guidance_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// migration, a compatibility window, or a degraded migration / compatibility-window entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigMigrationNextAction {
    /// Expand the resolved migration's or compatibility window's plain-language meaning.
    ExpandMigrationMeaning,
    /// Inspect the fidelity label or window class the entry resolves.
    InspectLabelOrWindow,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ConfigMigrationNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandMigrationMeaning,
        Self::InspectLabelOrWindow,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandMigrationMeaning => "expand_migration_meaning",
            Self::InspectLabelOrWindow => "inspect_label_or_window",
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
pub enum M5ConfigMigrationExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The settings-governance families covered.
    SettingsGovernanceFamilies,
    /// The migration fidelity labels carried.
    MigrationFidelityLabels,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The compatibility-window classes carried.
    CompatibilityWindowClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The label modes carried.
    FidelityLabelModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ConfigMigrationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::MigrationFidelityLabels,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::CompatibilityWindowClasses,
        Self::SurfaceContext,
        Self::FidelityLabelModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::MigrationFidelityLabels,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::SettingsGovernanceFamilies => "settings_governance_families",
            Self::MigrationFidelityLabels => "migration_fidelity_labels",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::CompatibilityWindowClasses => "compatibility_window_classes",
            Self::SurfaceContext => "surface_context",
            Self::FidelityLabelModes => "fidelity_label_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a schema-migration-record entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, fidelity-overstating, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SchemaMigrationRecordEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the migration means.
    SchemaMigrationTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The fidelity label is unclassified (not in the resolved taxonomy).
    FidelityClassUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    SchemaMigrationNotBoundToRegistry,
    /// The resolved schema-migration-record object is incomplete: the old key / alias, new key, transform,
    /// compatibility window, rollback note, compare-before-apply reference, or migration provenance reference is
    /// unstated.
    SchemaMigrationRecordIncomplete,
    /// The fidelity label overstates what the transform preserves, or a lossy / manual-review migration hid its
    /// compare-before-apply surface behind generic copy.
    MigrationOverstatesFidelityOrHidesCompareSurface,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A lossy or manual-review migration did not materialize the compare-before-apply surface before it
    /// applied.
    CompareSurfaceNotMaterializedForLossyMigration,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SchemaMigrationRecordEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SchemaMigrationTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::FidelityClassUnclassified,
        Self::SchemaMigrationNotBoundToRegistry,
        Self::SchemaMigrationRecordIncomplete,
        Self::MigrationOverstatesFidelityOrHidesCompareSurface,
        Self::ResolutionFormCoverageIncomplete,
        Self::CompareSurfaceNotMaterializedForLossyMigration,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaMigrationTokenUnstated => "schema_migration_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::FidelityClassUnclassified => "fidelity_class_unclassified",
            Self::SchemaMigrationNotBoundToRegistry => "schema_migration_not_bound_to_registry",
            Self::SchemaMigrationRecordIncomplete => "schema_migration_record_incomplete",
            Self::MigrationOverstatesFidelityOrHidesCompareSurface => {
                "migration_overstates_fidelity_or_hides_compare_surface"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::CompareSurfaceNotMaterializedForLossyMigration => {
                "compare_surface_not_materialized_for_lossy_migration"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ConfigMigrationNextAction {
        match self {
            Self::SchemaMigrationTokenUnstated | Self::SchemaMigrationNotBoundToRegistry => {
                M5ConfigMigrationNextAction::TraceCanonicalRegistry
            }
            Self::FidelityClassUnclassified
            | Self::SchemaMigrationRecordIncomplete
            | Self::MigrationOverstatesFidelityOrHidesCompareSurface => {
                M5ConfigMigrationNextAction::InspectLabelOrWindow
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5ConfigMigrationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::CompareSurfaceNotMaterializedForLossyMigration
            | Self::ProofStale => M5ConfigMigrationNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::SchemaMigrationTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::FidelityClassUnclassified | Self::SchemaMigrationRecordIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::SchemaMigrationNotBoundToRegistry => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::MigrationOverstatesFidelityOrHidesCompareSurface
            | Self::CompareSurfaceNotMaterializedForLossyMigration => {
                M5SettingsGovernanceDowngradeTrigger::RewroteAScopedWriteIntoABroaderScope
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a compatibility-window entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityWindowEntryDegradeReason {
    /// The canonical registry token name is unstated.
    WindowTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The window class is unclassified (not in the resolved taxonomy).
    WindowClassUnclassified,
    /// The compatibility window would mask a deprecated window without disclosing its window source, leave an
    /// outside-window migration without disclosing the downgrade guidance, or it dropped one of the required
    /// compatibility-window fields (window source, supported version range, deprecation review, validation,
    /// review state, docs pointer, last review revision).
    CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance,
    /// The canonical / accessible / audit resolution-form coverage of the record is incomplete.
    WindowFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CompatibilityWindowEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WindowTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::WindowClassUnclassified,
        Self::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance,
        Self::WindowFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowTokenUnstated => "window_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::WindowClassUnclassified => "window_class_unclassified",
            Self::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance => {
                "compatibility_window_masks_window_source_or_hides_downgrade_guidance"
            }
            Self::WindowFormCoverageIncomplete => "window_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ConfigMigrationNextAction {
        match self {
            Self::WindowTokenUnstated => M5ConfigMigrationNextAction::TraceCanonicalRegistry,
            Self::WindowClassUnclassified
            | Self::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance => {
                M5ConfigMigrationNextAction::InspectLabelOrWindow
            }
            Self::WindowFormCoverageIncomplete => {
                M5ConfigMigrationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ConfigMigrationNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::WindowTokenUnstated => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::WindowClassUnclassified => {
                M5SettingsGovernanceDowngradeTrigger::LifecycleStateUnstated
            }
            Self::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance => {
                M5SettingsGovernanceDowngradeTrigger::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy
            }
            Self::WindowFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_schema_migration_record_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SchemaMigrationRecordEntryResolutionInput {
    /// Stable identity of the schema-migration-registry entry.
    pub entry_id: String,
    /// The stable migration-target ID this record binds to (e.g. `settings.acme.editor.font-size@v1-to-v2`);
    /// empty means unstated.
    pub migration_ref: String,
    /// The canonical registry token name (e.g. `migration.editor.font_size`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The fidelity label this entry classifies.
    pub fidelity_class: M5SchemaMigrationFidelityClass,
    /// The render / surface context.
    pub surface_context: M5ConfigMigrationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5ConfigMigrationResolutionForm>,
    /// The published old key / alias the migration carries; empty means unstated.
    pub old_key_or_alias: String,
    /// The published new key the migration carries; empty means unstated.
    pub new_key: String,
    /// The published transform the migration applies; empty means unstated.
    pub transform: String,
    /// The published compatibility window the migration is valid across; empty means unstated.
    pub compatibility_window: String,
    /// The published rollback note; empty means unstated.
    pub rollback_note: String,
    /// The published compare-before-apply reference; empty means unstated.
    pub compare_before_apply_reference: String,
    /// The published migration provenance reference; empty means unstated.
    pub migration_provenance_reference: String,
    /// True when the behavior traces to the schema-migration registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the fidelity label honestly matches what the transform preserves (never overstates fidelity)
    /// (a hard invariant when `false`).
    pub fidelity_label_honest: bool,
    /// True when this migration is lossy or requires manual review.
    pub is_lossy_or_manual_review: bool,
    /// True when the compare-before-apply surface is materialized before a lossy / manual-review migration
    /// applies.
    pub compare_surface_materialized: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe schema-migration-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSchemaMigrationRecordEntry {
    /// Stable identity of the schema-migration-registry entry.
    pub entry_id: String,
    /// The stable migration-target ID this record binds to.
    pub migration_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The fidelity-label token named by the entry.
    pub fidelity_class: String,
    /// Whether the fidelity label is classified into the resolved taxonomy.
    pub fidelity_class_is_classified: bool,
    /// The canonical label mode for the entry's fidelity label.
    pub canonical_class_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published old key / alias.
    pub old_key_or_alias: String,
    /// The published new key.
    pub new_key: String,
    /// The published transform.
    pub transform: String,
    /// The published compatibility window.
    pub compatibility_window: String,
    /// The published rollback note.
    pub rollback_note: String,
    /// The published compare-before-apply reference.
    pub compare_before_apply_reference: String,
    /// The published migration provenance reference.
    pub migration_provenance_reference: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved schema-migration-record object publishes every required field.
    pub schema_migration_record_complete: bool,
    /// Whether the entry traces to the schema-migration registry.
    pub bound_to_registry: bool,
    /// Whether the fidelity label honestly matches the transform (never overstates fidelity).
    pub fidelity_label_honest: bool,
    /// Whether this migration is lossy or requires manual review.
    pub is_lossy_or_manual_review: bool,
    /// Whether the compare-before-apply surface is materialized before the migration applies.
    pub compare_surface_materialized: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5SchemaMigrationRecordEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ConfigMigrationNextAction,
    /// Whether the migration resolves to one object across every claimed route (clean entry naming every fact).
    pub migration_resolves_across_routes: bool,
}

impl M5ResolvedSchemaMigrationRecordEntry {
    /// Whether this schema-migration entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_compatibility_window_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompatibilityWindowEntryResolutionInput {
    /// Stable identity of the compatibility-window entry.
    pub entry_id: String,
    /// The stable window-ref this record binds to; empty means unstated.
    pub window_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The window class this record must resolve.
    pub window_class: M5CompatibilityWindowClass,
    /// The render / surface context.
    pub surface_context: M5ConfigMigrationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5ConfigMigrationResolutionForm>,
    /// The published window source; empty means missing.
    pub window_source: String,
    /// The published supported version range; empty means missing.
    pub supported_version_range: String,
    /// The published deprecation review window; empty means missing.
    pub deprecation_review: String,
    /// The published validation status; empty means missing.
    pub validation_status: String,
    /// The published review state; empty means missing.
    pub review_state: String,
    /// The published docs pointer; empty means missing.
    pub docs_pointer: String,
    /// The published last review revision; empty means missing.
    pub last_review_revision: String,
    /// True when the record keeps the window source visible.
    pub keeps_window_source_visible: bool,
    /// True when the window is truthful (never claims a clean resolution over a masked window).
    pub window_is_truthful: bool,
    /// True when the migration is deprecated.
    pub deprecation_present: bool,
    /// True when a deprecated window discloses its window source (never masks the window).
    pub deprecation_source_disclosed: bool,
    /// True when the migration is outside its compatibility window (unsupported).
    pub unsupported_present: bool,
    /// True when an outside-window migration discloses its downgrade guidance rather than ambiguous failure
    /// copy.
    pub downgrade_guidance_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe compatibility-window projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCompatibilityWindowEntry {
    /// Stable identity of the compatibility-window entry.
    pub entry_id: String,
    /// The stable window-ref this record binds to.
    pub window_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The window-class token named by the entry.
    pub window_class: String,
    /// Whether the window class is classified into the resolved taxonomy.
    pub window_class_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published window source.
    pub window_source: String,
    /// The published supported version range.
    pub supported_version_range: String,
    /// The published deprecation review window.
    pub deprecation_review: String,
    /// The published validation status.
    pub validation_status: String,
    /// The published review state.
    pub review_state: String,
    /// The published docs pointer.
    pub docs_pointer: String,
    /// The published last review revision.
    pub last_review_revision: String,
    /// Whether the record keeps the window source visible.
    pub keeps_window_source_visible: bool,
    /// Whether the window is truthful.
    pub window_is_truthful: bool,
    /// Whether the migration is deprecated.
    pub deprecation_present: bool,
    /// Whether a deprecated window discloses its window source.
    pub deprecation_source_disclosed: bool,
    /// Whether the migration is outside its compatibility window.
    pub unsupported_present: bool,
    /// Whether an outside-window migration discloses its downgrade guidance.
    pub downgrade_guidance_disclosed: bool,
    /// Whether the record stays honest (window source visible, window source disclosed, downgrade guidance
    /// disclosed).
    pub compatibility_window_stays_honest: bool,
    /// Whether the entry provides the complete compatibility-window object (window source, supported version
    /// range, deprecation review, validation, review state, docs pointer, last review revision).
    pub provides_complete_compatibility_window: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5CompatibilityWindowEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ConfigMigrationNextAction,
    /// Whether the compatibility window is safe on every claimed route (clean entry naming every fact).
    pub window_safe_on_every_route: bool,
}

impl M5ResolvedCompatibilityWindowEntry {
    /// Whether this compatibility-window entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ConfigMigrationResolutionError {
    /// The schema-migration-entry id was empty.
    EmptySchemaMigrationEntryId,
    /// The compatibility-window-entry id was empty.
    EmptyCompatibilityWindowEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ConfigMigrationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySchemaMigrationEntryId => "empty_schema_migration_entry_id",
            Self::EmptyCompatibilityWindowEntryId => "empty_compatibility_window_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ConfigMigrationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 schema-migration / compatibility-window registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ConfigMigrationResolutionError {}

fn form_tokens(forms: &[M5ConfigMigrationResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5ConfigMigrationResolutionForm]) -> bool {
    let present: BTreeSet<M5ConfigMigrationResolutionForm> = forms.iter().copied().collect();
    M5ConfigMigrationResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved schema-migration-record object publishes every required field: declared fidelity label
/// (via a classified label), old key / alias, new key, transform, compatibility window, rollback note,
/// compare-before-apply reference, and migration provenance reference. An unclassified label or any empty field
/// never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn schema_migration_record_is_complete(
    class: M5SchemaMigrationFidelityClass,
    old_key_or_alias: &str,
    new_key: &str,
    transform: &str,
    compatibility_window: &str,
    rollback_note: &str,
    compare_before_apply_reference: &str,
    migration_provenance_reference: &str,
) -> bool {
    class.is_classified()
        && !old_key_or_alias.trim().is_empty()
        && !new_key.trim().is_empty()
        && !transform.trim().is_empty()
        && !compatibility_window.trim().is_empty()
        && !rollback_note.trim().is_empty()
        && !compare_before_apply_reference.trim().is_empty()
        && !migration_provenance_reference.trim().is_empty()
}

/// Whether the migration keeps its fidelity honest: the label must be classified, the fidelity label must
/// honestly match what the transform preserves (never overstate fidelity), and a lossy or manual-review
/// migration must materialize the compare-before-apply surface before it applies. An unclassified label, an
/// overstated fidelity label, or a hidden compare surface never matches.
pub fn migration_does_not_overstate_fidelity(
    class: M5SchemaMigrationFidelityClass,
    fidelity_label_honest: bool,
    is_lossy_or_manual_review: bool,
    compare_surface_materialized: bool,
) -> bool {
    class.is_classified()
        && fidelity_label_honest
        && (!is_lossy_or_manual_review || compare_surface_materialized)
}

/// Whether a compatibility window stays honest: the window class must be classified, the window must be
/// truthful, it must keep the window source visible, any deprecated window must disclose its window source
/// rather than mask it, and any outside-window migration must disclose its downgrade guidance rather than read
/// as ambiguous failure copy.
pub fn compatibility_window_stays_honest(
    class: M5CompatibilityWindowClass,
    window_is_truthful: bool,
    keeps_window_source_visible: bool,
    deprecation_present: bool,
    deprecation_source_disclosed: bool,
    unsupported_present: bool,
    downgrade_guidance_disclosed: bool,
) -> bool {
    class.is_classified()
        && window_is_truthful
        && keeps_window_source_visible
        && (!deprecation_present || deprecation_source_disclosed)
        && (!unsupported_present || downgrade_guidance_disclosed)
}

/// Resolves a schema-migration-registry entry so it stays bound to the schema-migration registry: the entry
/// names its canonical token, semantic role, and fidelity label, covers all three resolution forms, publishes a
/// complete schema-migration-record object (old key / alias, new key, transform, compatibility window, rollback
/// note, compare-before-apply reference, migration provenance reference), keeps its fidelity label honest, and
/// materializes the compare-before-apply surface before a lossy / manual-review migration applies.
pub fn resolve_schema_migration_record_entry(
    input: M5SchemaMigrationRecordEntryResolutionInput,
) -> Result<M5ResolvedSchemaMigrationRecordEntry, M5ConfigMigrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ConfigMigrationResolutionError::EmptySchemaMigrationEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.migration_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.old_key_or_alias)
        || string_is_forbidden(&input.new_key)
        || string_is_forbidden(&input.transform)
        || string_is_forbidden(&input.compatibility_window)
        || string_is_forbidden(&input.rollback_note)
        || string_is_forbidden(&input.compare_before_apply_reference)
        || string_is_forbidden(&input.migration_provenance_reference)
    {
        return Err(M5ConfigMigrationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = schema_migration_record_is_complete(
        input.fidelity_class,
        &input.old_key_or_alias,
        &input.new_key,
        &input.transform,
        &input.compatibility_window,
        &input.rollback_note,
        &input.compare_before_apply_reference,
        &input.migration_provenance_reference,
    );
    let fidelity_ok = migration_does_not_overstate_fidelity(
        input.fidelity_class,
        input.fidelity_label_honest,
        input.is_lossy_or_manual_review,
        input.compare_surface_materialized,
    );
    let compare_unmaterialized =
        input.is_lossy_or_manual_review && !input.compare_surface_materialized;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SchemaMigrationRecordEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.fidelity_class.is_classified() {
        Some(M5SchemaMigrationRecordEntryDegradeReason::FidelityClassUnclassified)
    } else if !input.bound_to_registry {
        Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationNotBoundToRegistry)
    } else if !object_complete {
        Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationRecordIncomplete)
    } else if !fidelity_ok {
        Some(M5SchemaMigrationRecordEntryDegradeReason::MigrationOverstatesFidelityOrHidesCompareSurface)
    } else if !all_forms {
        Some(M5SchemaMigrationRecordEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if compare_unmaterialized {
        Some(M5SchemaMigrationRecordEntryDegradeReason::CompareSurfaceNotMaterializedForLossyMigration)
    } else if !input.proof_fresh {
        Some(M5SchemaMigrationRecordEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ConfigMigrationNextAction::ExpandMigrationMeaning,
    };

    Ok(M5ResolvedSchemaMigrationRecordEntry {
        entry_id: input.entry_id,
        migration_ref: input.migration_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        fidelity_class: input.fidelity_class.as_str().to_owned(),
        fidelity_class_is_classified: input.fidelity_class.is_classified(),
        canonical_class_mode: input.fidelity_class.canonical_class_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        old_key_or_alias: input.old_key_or_alias,
        new_key: input.new_key,
        transform: input.transform,
        compatibility_window: input.compatibility_window,
        rollback_note: input.rollback_note,
        compare_before_apply_reference: input.compare_before_apply_reference,
        migration_provenance_reference: input.migration_provenance_reference,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        schema_migration_record_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        fidelity_label_honest: input.fidelity_label_honest,
        is_lossy_or_manual_review: input.is_lossy_or_manual_review,
        compare_surface_materialized: input.compare_surface_materialized,
        degrade_reason,
        next_action,
        migration_resolves_across_routes: degrade_reason.is_none(),
    })
}

/// Resolves a compatibility-window entry so its resolution stays safe: the entry names its canonical token,
/// semantic role, and window class, covers all three resolution forms, provides the complete window-source /
/// supported-version-range / deprecation-review / validation-status / review-state / docs-pointer /
/// last-review-revision compatibility-window object, and degrades honestly when the record would mask a
/// deprecated window without disclosing its window source or leave an outside-window migration without
/// disclosing the downgrade guidance.
pub fn resolve_compatibility_window_entry(
    input: M5CompatibilityWindowEntryResolutionInput,
) -> Result<M5ResolvedCompatibilityWindowEntry, M5ConfigMigrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ConfigMigrationResolutionError::EmptyCompatibilityWindowEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.window_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.window_source)
        || string_is_forbidden(&input.supported_version_range)
        || string_is_forbidden(&input.deprecation_review)
        || string_is_forbidden(&input.validation_status)
        || string_is_forbidden(&input.review_state)
        || string_is_forbidden(&input.docs_pointer)
        || string_is_forbidden(&input.last_review_revision)
    {
        return Err(M5ConfigMigrationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = compatibility_window_stays_honest(
        input.window_class,
        input.window_is_truthful,
        input.keeps_window_source_visible,
        input.deprecation_present,
        input.deprecation_source_disclosed,
        input.unsupported_present,
        input.downgrade_guidance_disclosed,
    );
    let provides_record = input.window_class.is_classified()
        && !input.window_source.trim().is_empty()
        && !input.supported_version_range.trim().is_empty()
        && !input.deprecation_review.trim().is_empty()
        && !input.validation_status.trim().is_empty()
        && !input.review_state.trim().is_empty()
        && !input.docs_pointer.trim().is_empty()
        && !input.last_review_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CompatibilityWindowEntryDegradeReason::WindowTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CompatibilityWindowEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.window_class.is_classified() {
        Some(M5CompatibilityWindowEntryDegradeReason::WindowClassUnclassified)
    } else if !provides_record {
        Some(M5CompatibilityWindowEntryDegradeReason::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance)
    } else if !all_forms {
        Some(M5CompatibilityWindowEntryDegradeReason::WindowFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5CompatibilityWindowEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ConfigMigrationNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedCompatibilityWindowEntry {
        entry_id: input.entry_id,
        window_ref: input.window_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        window_class: input.window_class.as_str().to_owned(),
        window_class_is_classified: input.window_class.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        window_source: input.window_source,
        supported_version_range: input.supported_version_range,
        deprecation_review: input.deprecation_review,
        validation_status: input.validation_status,
        review_state: input.review_state,
        docs_pointer: input.docs_pointer,
        last_review_revision: input.last_review_revision,
        keeps_window_source_visible: input.keeps_window_source_visible,
        window_is_truthful: input.window_is_truthful,
        deprecation_present: input.deprecation_present,
        deprecation_source_disclosed: input.deprecation_source_disclosed,
        unsupported_present: input.unsupported_present,
        downgrade_guidance_disclosed: input.downgrade_guidance_disclosed,
        compatibility_window_stays_honest: record_stays_honest,
        provides_complete_compatibility_window: provides_record,
        degrade_reason,
        next_action,
        window_safe_on_every_route: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved schema-migration and compatibility-window
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSchemaMigrationCompatibilityWindowRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SettingSchemaMigrationCompatibilityWindowRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5ConfigMigrationAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ConfigMigrationExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    /// Resolved schema-migration-registry examples.
    pub schema_migration_entries: Vec<M5ResolvedSchemaMigrationRecordEntry>,
    /// Resolved compatibility-window examples.
    pub compatibility_window_entries: Vec<M5ResolvedCompatibilityWindowEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the setting-definition domain and the
    /// schema-migration landed schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never implies full fidelity when the migration is lossy. MUST be `false`.
    pub implies_full_fidelity_when_migration_is_lossy: bool,
    /// Hard invariant: this row never alters stored meaning without a checked-in migration record. MUST be
    /// `false`.
    pub alters_stored_meaning_without_a_checked_in_migration_record: bool,
    /// Hard invariant: this row never applies a lossy migration without a compare-before-apply surface. MUST be
    /// `false`.
    pub applies_a_lossy_migration_without_a_compare_before_apply_surface: bool,
    /// Hard invariant: this row never hides the compatibility window or downgrade cause behind generic copy.
    /// MUST be `false`.
    pub hides_the_compatibility_window_or_downgrade_cause_behind_generic_copy: bool,
}

impl M5SettingSchemaMigrationCompatibilityWindowRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ConfigMigrationAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ConfigMigrationAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ConfigMigrationExportField> =
            self.export_fields.iter().copied().collect();
        M5ConfigMigrationExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.implies_full_fidelity_when_migration_is_lossy
            && !self.alters_stored_meaning_without_a_checked_in_migration_record
            && !self.applies_a_lossy_migration_without_a_compare_before_apply_surface
            && !self.hides_the_compatibility_window_or_downgrade_cause_behind_generic_copy
    }

    /// True when a clean schema-migration entry preserves registry-bound truth: it traces to the registry, keeps
    /// a classified fidelity label, publishes a complete migration record, keeps its fidelity label honest,
    /// covers all three resolution forms, and materializes the compare surface for a lossy / manual-review
    /// migration.
    fn migration_is_honest(ex: &M5ResolvedSchemaMigrationRecordEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.fidelity_class_is_classified
                && ex.schema_migration_record_complete
                && ex.fidelity_label_honest
                && ex.covers_all_resolution_forms
                && (!ex.is_lossy_or_manual_review || ex.compare_surface_materialized))
    }

    /// True when a clean compatibility-window entry preserves a safe record: it keeps a classified window class,
    /// provides the complete compatibility-window object, stays honest, and covers all three resolution forms.
    fn window_is_honest(ex: &M5ResolvedCompatibilityWindowEntry) -> bool {
        !ex.is_clean()
            || (ex.window_class_is_classified
                && ex.provides_complete_compatibility_window
                && ex.compatibility_window_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.schema_migration_entries
            .iter()
            .all(Self::migration_is_honest)
            && self
                .compatibility_window_entries
                .iter()
                .all(Self::window_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSchemaMigrationCompatibilityWindowRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Migration fidelity-label tokens (minted by this lane).
    pub migration_fidelity_classes: Vec<String>,
    /// Compatibility-window-class tokens (minted by this lane).
    pub compatibility_window_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Schema-migration-entry degrade-reason tokens.
    pub schema_migration_degrade_reasons: Vec<String>,
    /// Compatibility-window-entry degrade-reason tokens.
    pub compatibility_window_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SettingSchemaMigrationCompatibilityWindowRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5SettingsGovernanceRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5ConfigMigrationResolutionForm::ALL, |v| v.as_str()),
            migration_fidelity_classes: tokens(&M5SchemaMigrationFidelityClass::ALL, |v| {
                v.as_str()
            }),
            compatibility_window_classes: tokens(&M5CompatibilityWindowClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ConfigMigrationSurfaceContext::ALL, |v| v.as_str()),
            schema_migration_degrade_reasons: tokens(
                &M5SchemaMigrationRecordEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            compatibility_window_degrade_reasons: tokens(
                &M5CompatibilityWindowEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5ConfigMigrationAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ConfigMigrationNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ConfigMigrationExportField::ALL, |v| v.as_str()),
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
pub struct M5SettingSchemaMigrationCompatibilityWindowRegistriesGovernanceReview {
    /// The schema-migration registry names a canonical token, semantic role, and fidelity label for every
    /// entry.
    pub schema_migration_registry_names_token_role_and_label: bool,
    /// Every claimed version change resolves to one migration record from the shared registry, not per-entry
    /// reconstruction.
    pub migration_resolves_to_one_record_from_shared_registry: bool,
    /// The old key / alias, new key, transform, compatibility window, rollback note, compare-before-apply
    /// reference, and migration provenance reference are published for every resolved migration.
    pub old_key_new_key_transform_window_rollback_and_compare_surface_published: bool,
    /// Migration labels never overstate fidelity; a lossy or manual-review migration never implies full
    /// fidelity.
    pub migration_labels_never_overstate_fidelity: bool,
    /// The compatibility-window record keeps the window source visible and discloses the downgrade guidance.
    pub compatibility_window_keeps_window_source_visible_and_discloses_downgrade_guidance: bool,
    /// The compare-before-apply surface is materialized before any lossy or manual-review migration applies.
    pub compare_before_apply_surface_materialized_for_lossy_or_manual_migrations: bool,
    /// Every schema-migration and compatibility-window entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Schema-migration and compatibility-window behavior stay bound to the shared registries rather than
    /// hand-copied per version change.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Upgrade, import, restore, and downgrade flows read a single configuration source.
    pub upgrade_import_restore_downgrade_read_single_source: bool,
    /// An overstated fidelity label, an incomplete record, or a masked window is caught by fixtures before
    /// release evidence turns green.
    pub migration_or_window_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSchemaMigrationCompatibilityWindowRegistriesConsumerProjection {
    /// Upgrade and import flows consume the shared schema-migration registry.
    pub upgrade_and_import_consume_shared_registries: bool,
    /// Restore and downgrade flows consume the shared compatibility-window registry.
    pub restore_and_downgrade_consume_shared_registries: bool,
    /// Migration and compatibility services consume the shared registries.
    pub migration_and_compat_services_consume_shared_registries: bool,
    /// Docs, migration guides, and CLI export consume the shared registries.
    pub docs_migration_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical schema-migration and compatibility-window domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical schema-migration / compatibility-window registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSchemaMigrationCompatibilityWindowRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSchemaMigrationCompatibilityWindowRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting settings-governance audit for the lane.
    pub settings_governance_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingSchemaMigrationCompatibilityWindowRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingSchemaMigrationCompatibilityWindowRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingSchemaMigrationCompatibilityWindowRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection:
        M5SettingSchemaMigrationCompatibilityWindowRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingSchemaMigrationCompatibilityWindowRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingSchemaMigrationCompatibilityWindowRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 schema-migration and compatibility-window registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket {
    /// Record kind; must equal [`M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingSchemaMigrationCompatibilityWindowRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingSchemaMigrationCompatibilityWindowRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection:
        M5SettingSchemaMigrationCompatibilityWindowRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingSchemaMigrationCompatibilityWindowRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingSchemaMigrationCompatibilityWindowRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SettingSchemaMigrationCompatibilityWindowRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version:
                M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind
            != M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_RECORD_KIND
        {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::VocabularySetDrift,
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
                .expect("m5 schema-migration / compatibility-window registries packet serializes"),
        ) {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::RawMaterialInExport,
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
            .expect("m5 schema-migration / compatibility-window registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,schema_migration_entries,compatibility_window_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .schema_migration_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.compatibility_window_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.schema_migration_entries.len(),
                row.compatibility_window_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Setting-Schema-Migration and Compatibility-Window Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Migration fidelity labels: {}\n",
            self.vocabulary_set.migration_fidelity_classes.join(", ")
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
                "  - Schema-migration entries: {} / compatibility-window entries: {}\n",
                row.schema_migration_entries.len(),
                row.compatibility_window_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry schema-migration reference table generated from the registry, so docs and
    /// migration runbooks render the same label-mode / old-key / new-key / transform / rollback-note /
    /// compare-reference truth the resolvers produced rather than a hand-copied migration table. Only clean,
    /// registry-bound schema-migration entries are listed.
    pub fn render_migration_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| migration_ref | label_mode | old_key | new_key | transform | rollback_note | compare_reference |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.schema_migration_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.migration_ref,
                    ex.canonical_class_mode,
                    ex.old_key_or_alias,
                    ex.new_key,
                    ex.transform,
                    ex.rollback_note,
                    ex.compare_before_apply_reference
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SettingSchemaMigrationCompatibilityWindowRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation>),
}

impl fmt::Display for M5SettingSchemaMigrationCompatibilityWindowRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 schema-migration / compatibility-window registries export parse failed: {error}"
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
                    "m5 schema-migration / compatibility-window registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SettingSchemaMigrationCompatibilityWindowRegistriesArtifactError {}

/// Validation failures emitted by [`M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation {
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
    /// A registry row does not point at both the setting-definition domain and the schema-migration landed
    /// schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, fidelity-overstating, field-incomplete,
    /// form-incomplete, or a compatibility-window entry missing the complete record object).
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
    /// Schema-migration-resolution is not proven: clean migration entries do not cover the canonical fidelity
    /// labels or the first upgrade / import / restore / downgrade / support flows, no record-incomplete example
    /// degrades, or a clean migration entry published an incomplete record.
    SchemaMigrationResolutionNotProven,
    /// Migration-fidelity-honesty is not proven: no fidelity-overstate example and no unbound example degrade,
    /// no clean fidelity-honest migration entry is present, or a clean migration entry overstated fidelity or is
    /// unbound.
    MigrationFidelityHonestyNotProven,
    /// Compatibility-window-integrity is not proven: clean compatibility-window entries do not cover the
    /// canonical within-window / deprecated / outside-window classes with full resolution-form coverage while
    /// providing the complete record object, no masked-window or form-incomplete example degrades, or a clean
    /// compatibility-window entry is missing the complete record object.
    CompatibilityWindowIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation {
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
            Self::SchemaMigrationResolutionNotProven => "schema_migration_resolution_not_proven",
            Self::MigrationFidelityHonestyNotProven => "migration_fidelity_honesty_not_proven",
            Self::CompatibilityWindowIntegrityNotProven => {
                "compatibility_window_integrity_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_setting_schema_migration_and_compatibility_window_registries_export(
) -> Result<
    M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket,
    M5SettingSchemaMigrationCompatibilityWindowRegistriesArtifactError,
> {
    let packet: M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-setting-schema-migration-and-compatibility-window-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SettingSchemaMigrationCompatibilityWindowRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5SettingSchemaMigrationCompatibilityWindowRegistriesArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket,
    violations: &mut Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_SCHEMA_REF,
        M5_SETTING_SCHEMA_MIGRATION_COMPATIBILITY_WINDOW_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
        M5_SCHEMA_MIGRATION_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket,
    violations: &mut Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations
            .push(M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::NoRegistryRows);
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
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_SCHEMA_MIGRATION_LANDED_SCHEMA_REF)
        {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.schema_migration_entries.is_empty() || row.compatibility_window_entries.is_empty() {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket,
    violations: &mut Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.schema_migration_registry_names_token_role_and_label,
        review.migration_resolves_to_one_record_from_shared_registry,
        review.old_key_new_key_transform_window_rollback_and_compare_surface_published,
        review.migration_labels_never_overstate_fidelity,
        review.compatibility_window_keeps_window_source_visible_and_discloses_downgrade_guidance,
        review.compare_before_apply_surface_materialized_for_lossy_or_manual_migrations,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.upgrade_import_restore_downgrade_read_single_source,
        review.migration_or_window_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket,
    violations: &mut Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.upgrade_and_import_consume_shared_registries,
        projection.restore_and_downgrade_consume_shared_registries,
        projection.migration_and_compat_services_consume_shared_registries,
        projection.docs_migration_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket,
    violations: &mut Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket,
    violations: &mut Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.settings_governance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket,
    violations: &mut Vec<M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation>,
) {
    let migrations = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.schema_migration_entries.iter())
    };
    let windows = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.compatibility_window_entries.iter())
    };

    // AC1: configuration artifacts carry explicit migration provenance and compatibility-window truth. Clean
    // migration entries cover the canonical fidelity labels and the first upgrade / import / restore / downgrade
    // / support flows, a record-incomplete example degrades, and no clean migration entry published an
    // incomplete record (an incomplete record is a missing compare / provenance reference).
    let clean_classes: BTreeSet<String> = migrations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.fidelity_class.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = migrations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let classes_covered = M5SchemaMigrationFidelityClass::CANONICAL_CLASSES
        .iter()
        .all(|k| clean_classes.contains(k.as_str()));
    let first_surfaces_covered = M5ConfigMigrationSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let record_incomplete_degrades = migrations().any(|ex| {
        ex.degrade_reason
            == Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationRecordIncomplete)
    });
    let no_clean_incomplete =
        !migrations().any(|ex| ex.is_clean() && !ex.schema_migration_record_complete);
    if !(classes_covered
        && first_surfaces_covered
        && record_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::SchemaMigrationResolutionNotProven,
        );
    }

    // AC2: no downgrade / import path implies full fidelity when the migration is lossy or requires manual
    // review. A fidelity-overstate example degrades, an unbound example degrades, at least one clean
    // fidelity-honest migration entry is present, and no clean migration entry overstated fidelity or is
    // unbound.
    let overstate_degrades = migrations().any(|ex| {
        ex.degrade_reason
            == Some(
                M5SchemaMigrationRecordEntryDegradeReason::MigrationOverstatesFidelityOrHidesCompareSurface,
            )
    });
    let unbound_degrades = migrations().any(|ex| {
        ex.degrade_reason
            == Some(M5SchemaMigrationRecordEntryDegradeReason::SchemaMigrationNotBoundToRegistry)
    });
    let honest_clean_migration = migrations().any(|ex| ex.is_clean() && ex.fidelity_label_honest);
    let no_clean_unbound = !migrations().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_overstated = !migrations().any(|ex| ex.is_clean() && !ex.fidelity_label_honest);
    if !(overstate_degrades
        && unbound_degrades
        && honest_clean_migration
        && no_clean_unbound
        && no_clean_overstated)
    {
        violations.push(
            M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::MigrationFidelityHonestyNotProven,
        );
    }

    // AC3: schema changes never alter stored meaning without a checked-in migration record and compare surface.
    // Clean compatibility-window entries cover every canonical within-window / deprecated / outside-window class
    // with full resolution-form coverage while providing the complete record object, a masked-window example
    // degrades, a form-incomplete example degrades, and no clean compatibility-window entry is missing the
    // complete record object.
    let clean_record_classes: BTreeSet<String> = windows()
        .filter(|ex| {
            ex.is_clean()
                && ex.window_class_is_classified
                && ex.provides_complete_compatibility_window
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.window_class.clone())
        .collect();
    let record_classes_covered = M5CompatibilityWindowClass::CANONICAL_CLASSES
        .iter()
        .all(|m| clean_record_classes.contains(m.as_str()));
    let masked_window_degrades = windows().any(|ex| {
        ex.degrade_reason
            == Some(
                M5CompatibilityWindowEntryDegradeReason::CompatibilityWindowMasksWindowSourceOrHidesDowngradeGuidance,
            )
    });
    let form_incomplete_degrades = windows().any(|ex| {
        ex.degrade_reason
            == Some(M5CompatibilityWindowEntryDegradeReason::WindowFormCoverageIncomplete)
    });
    let no_clean_missing_record =
        !windows().any(|ex| ex.is_clean() && !ex.provides_complete_compatibility_window);
    if !(record_classes_covered
        && masked_window_degrades
        && form_incomplete_degrades
        && no_clean_missing_record)
    {
        violations.push(
            M5SettingSchemaMigrationCompatibilityWindowRegistriesViolation::CompatibilityWindowIntegrityNotProven,
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
    [M5SettingsGovernanceFamily::MigrateSchema];
