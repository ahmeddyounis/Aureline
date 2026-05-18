//! Shared marketplace fact-grid contract for catalog and install-review truth.
//!
//! Marketplace rows, package details, install/update/rollback sheets, headless
//! diagnostics, and support exports read this record instead of assembling
//! package trust from local row copy. The grid deliberately joins catalog
//! source, client scope, compatibility, permission delta, script/native-build
//! risk, lockfile churn, revocation posture, rollback posture, and activation
//! budget before any workspace mutation can commit.

use serde::{Deserialize, Serialize};

use crate::install_review::{
    ActivationBudget, InstallReviewAlphaPacketRecord, InstallReviewDecisionClass,
    InstallReviewDecisionReasonClass, RuntimeCostClass, RuntimeCostEvidenceClass,
};
use crate::manifest_baseline::{DeclaredVsEffectiveDiffEntry, RedactionClass};
use crate::marketplace_truth::{
    MarketplaceCompatibilityLabelClass, MarketplaceSupportChipClass, MarketplaceTrustChipClass,
    MarketplaceTruthBadgeClass, MarketplaceTruthRowRecord,
};
use crate::registry::{
    CatalogDescriptorDecisionClass, CatalogDescriptorRecord, CatalogLifecycleStateClass,
    CatalogMirrorabilityClass, CatalogRegistrySourceClass, CatalogRevocationSnapshotAgeClass,
};
use crate::review_alpha::RevocationStateClass;

#[cfg(test)]
mod tests;

/// Record-kind tag carried by [`MarketplaceFactGridRecord`] payloads.
pub const MARKETPLACE_FACT_GRID_RECORD_KIND: &str = "marketplace_fact_grid_record";

/// Record-kind tag carried by [`MarketplaceFactGridSupportExportRecord`] payloads.
pub const MARKETPLACE_FACT_GRID_SUPPORT_EXPORT_RECORD_KIND: &str =
    "marketplace_fact_grid_support_export_record";

/// Schema version for marketplace fact-grid payloads.
pub const MARKETPLACE_FACT_GRID_SCHEMA_VERSION: u32 = 1;

/// Surface consuming the shared marketplace fact grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceFactGridSurfaceClass {
    /// Compact discovery row in a public, mirrored, private, offline, or local catalog.
    ResultRow,
    /// Package detail page shown before install or update.
    DetailPage,
    /// Product-owned install review sheet.
    InstallReviewSheet,
    /// Product-owned update review sheet.
    UpdateReviewSheet,
    /// Product-owned rollback review sheet.
    RollbackReviewSheet,
    /// CLI or support-safe diagnostic projection.
    SupportExport,
    /// Compatibility report row or exported compatibility packet.
    CompatibilityReport,
    /// Manual artifact import review.
    ManualImportReview,
    /// Offline registry row.
    OfflineRegistryRow,
}

/// Client scope declared for a package or workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientScopeClass {
    /// Supported in the desktop client.
    Desktop,
    /// Supported only in a browser companion or lightweight companion client.
    BrowserCompanion,
    /// Supported in desktop and browser companion clients, with authority still scoped per client.
    DesktopPlusBrowserCompanion,
    /// Supported only through CLI, automation, or headless execution.
    HeadlessOnly,
    /// Not supported in the current client scope.
    Unsupported,
}

impl ClientScopeClass {
    /// Returns the stable schema token for this client-scope class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::BrowserCompanion => "browser_companion",
            Self::DesktopPlusBrowserCompanion => "desktop_plus_browser_companion",
            Self::HeadlessOnly => "headless_only",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns the short display label for this client-scope class.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::BrowserCompanion => "Browser companion",
            Self::DesktopPlusBrowserCompanion => "Desktop + browser companion",
            Self::HeadlessOnly => "Headless only",
            Self::Unsupported => "Unsupported",
        }
    }

    /// Returns `true` when the current client scope cannot perform the workflow.
    pub const fn blocks_current_client(self) -> bool {
        matches!(self, Self::Unsupported)
    }
}

/// Script or native-build risk shown before a package mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptRiskClass {
    /// The package declares no lifecycle scripts and no native build step.
    NoScriptsOrNativeBuild,
    /// The package declares lifecycle scripts that must be reviewed before mutation.
    LifecycleScriptsDeclared,
    /// The package needs a native build, compiler, toolchain, or platform-specific helper.
    NativeBuildRequired,
    /// The package starts an external helper or extension host process.
    ExternalHelperOrHost,
    /// Script or native-build risk is unknown and therefore blocks mutation.
    UnknownScriptRiskBlocked,
}

impl ScriptRiskClass {
    /// Returns the stable schema token for this risk class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoScriptsOrNativeBuild => "no_scripts_or_native_build",
            Self::LifecycleScriptsDeclared => "lifecycle_scripts_declared",
            Self::NativeBuildRequired => "native_build_required",
            Self::ExternalHelperOrHost => "external_helper_or_host",
            Self::UnknownScriptRiskBlocked => "unknown_script_risk_blocked",
        }
    }

    /// Returns `true` when the risk class must narrow or block mutation.
    pub const fn blocks_mutation(self) -> bool {
        matches!(self, Self::UnknownScriptRiskBlocked)
    }
}

/// Manifest change class shown in install, update, and rollback review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestChangeClass {
    /// A manifest field or file is added.
    Added,
    /// A manifest field or file is removed.
    Removed,
    /// A manifest field is changed.
    Changed,
    /// A permission-manifest or effective-permission delta is part of the change.
    PermissionDelta,
    /// Only metadata such as rationale, docs, or lifecycle copy changed.
    MetadataOnly,
    /// No manifest change is expected.
    NoChangeDeclared,
}

/// Lockfile or generated-file impact class for a package mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileImpactClass {
    /// No lockfile or generated-file mutation is expected.
    NoLockfileChange,
    /// One or more lockfiles are expected to change.
    LockfileChurnExpected,
    /// Generated files are expected to change alongside manifests or lockfiles.
    GeneratedFilesExpected,
    /// The safe path is resolver regeneration followed by review.
    RegenerateAndReview,
    /// Lockfile impact is unknown and therefore blocks mutation.
    UnknownBlocked,
}

impl LockfileImpactClass {
    /// Returns the stable schema token for this impact class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLockfileChange => "no_lockfile_change",
            Self::LockfileChurnExpected => "lockfile_churn_expected",
            Self::GeneratedFilesExpected => "generated_files_expected",
            Self::RegenerateAndReview => "regenerate_and_review",
            Self::UnknownBlocked => "unknown_blocked",
        }
    }

    /// Returns `true` when the class must include lockfile or generated-file refs.
    pub const fn requires_artifact_refs(self) -> bool {
        matches!(
            self,
            Self::LockfileChurnExpected | Self::GeneratedFilesExpected | Self::RegenerateAndReview
        )
    }

    /// Returns `true` when the class blocks mutation until better evidence exists.
    pub const fn blocks_mutation(self) -> bool {
        matches!(self, Self::UnknownBlocked)
    }
}

/// Script and native-build risk disclosure shown in every fact grid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptRiskDisclosure {
    /// Controlled script/native-build risk class.
    pub script_risk_class: ScriptRiskClass,
    /// Registry, manifest, SBOM, or package-manager refs that justify the class.
    pub risk_source_refs: Vec<String>,
    /// Native build or toolchain refs, when relevant.
    pub native_build_requirement_refs: Vec<String>,
    /// Policy refs that block or narrow script execution.
    pub policy_block_refs: Vec<String>,
    /// Metadata-safe explanation for UI, CLI, and support export.
    pub summary: String,
}

impl ScriptRiskDisclosure {
    /// Returns a no-script/no-native-build disclosure with explicit evidence refs.
    pub fn no_scripts(risk_source_refs: Vec<String>, summary: impl Into<String>) -> Self {
        Self {
            script_risk_class: ScriptRiskClass::NoScriptsOrNativeBuild,
            risk_source_refs,
            native_build_requirement_refs: Vec::new(),
            policy_block_refs: Vec::new(),
            summary: summary.into(),
        }
    }
}

/// One manifest-change row in the shared fact grid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestChangeRow {
    /// Stable row id.
    pub change_id: String,
    /// Class of manifest change.
    pub change_class: ManifestChangeClass,
    /// Manifest or generated-manifest ref affected by the change.
    pub manifest_ref: String,
    /// Field path or structured member affected by the change.
    pub field_path: String,
    /// Redaction-safe previous value summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_summary: Option<String>,
    /// Redaction-safe candidate value summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_summary: Option<String>,
    /// True when the row must be included in native review before commit.
    pub review_required: bool,
    /// Metadata-safe explanation for UI, CLI, and support export.
    pub summary: String,
}

/// Lockfile and generated-file impact shown before workspace mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockfileImpact {
    /// Controlled impact class.
    pub impact_class: LockfileImpactClass,
    /// Resolver identity and version ref.
    pub resolver_ref: String,
    /// Affected lockfile refs.
    pub affected_lockfile_refs: Vec<String>,
    /// Generated-file refs expected to change.
    pub generated_file_refs: Vec<String>,
    /// Platform, environment, or feature-factor refs affecting the resolver.
    pub environment_factor_refs: Vec<String>,
    /// Rollback or checkpoint ref that can restore prior lockfile state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Metadata-safe explanation for UI, CLI, and support export.
    pub summary: String,
}

/// Quarantine, revocation, and rollback state copied from the catalog descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantineRevocationState {
    /// Catalog revocation state.
    pub revocation_state_class: RevocationStateClass,
    /// Age and verifiability of the revocation snapshot.
    pub revocation_snapshot_age_class: CatalogRevocationSnapshotAgeClass,
    /// Last known good version, when catalog metadata carries one.
    pub last_known_good_version: String,
    /// Rollback manifest ref.
    pub rollback_manifest_ref: String,
    /// Emergency disable refs.
    pub emergency_disable_refs: Vec<String>,
    /// True when quarantine or revocation blocks install/update.
    pub install_or_update_blocked: bool,
}

/// Inputs supplied to build one marketplace fact-grid record.
pub struct MarketplaceFactGridInput<'a> {
    /// Stable fact-grid id.
    pub fact_grid_id: &'a str,
    /// Surface consuming the grid.
    pub surface_class: MarketplaceFactGridSurfaceClass,
    /// Marketplace row shown before install review.
    pub marketplace_row: &'a MarketplaceTruthRowRecord,
    /// Catalog descriptor backing the row.
    pub catalog: &'a CatalogDescriptorRecord,
    /// Native install-review packet opened by the row.
    pub install_review: &'a InstallReviewAlphaPacketRecord,
    /// Client scope declared for the row.
    pub client_scope_class: ClientScopeClass,
    /// Metadata-safe client-scope explanation.
    pub client_scope_summary: &'a str,
    /// Script or native-build risk disclosure.
    pub script_risk: ScriptRiskDisclosure,
    /// Manifest changes expected before mutation.
    pub manifest_changes: Vec<ManifestChangeRow>,
    /// Lockfile or generated-file impact disclosure.
    pub lockfile_impact: LockfileImpact,
    /// Projection timestamp.
    pub generated_at: &'a str,
}

/// Shared marketplace fact grid consumed by rows, details, reviews, and exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceFactGridRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this record.
    pub marketplace_fact_grid_schema_version: u32,
    /// Stable fact-grid id.
    pub fact_grid_id: String,
    /// Surface consuming the grid.
    pub surface_class: MarketplaceFactGridSurfaceClass,
    /// Source marketplace row ref.
    pub marketplace_row_ref: String,
    /// Source catalog descriptor ref.
    pub catalog_descriptor_ref: String,
    /// Native install-review ref opened by the row.
    pub install_review_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Package id.
    pub package_id: String,
    /// Display name.
    pub display_name: String,
    /// Publisher display label.
    pub publisher_display_label: String,
    /// Client scope class shown beside support and freshness truth.
    pub client_scope_class: ClientScopeClass,
    /// Client-scope token for copy/export consumers.
    pub client_scope_token: String,
    /// Metadata-safe client-scope explanation.
    pub client_scope_summary: String,
    /// Registry source class.
    pub registry_source_class: CatalogRegistrySourceClass,
    /// Mirrorability class.
    pub mirrorability_class: CatalogMirrorabilityClass,
    /// Catalog lifecycle state.
    pub catalog_lifecycle_state_class: CatalogLifecycleStateClass,
    /// Catalog decision class.
    pub catalog_decision_class: CatalogDescriptorDecisionClass,
    /// Controlled lifecycle badges copied from the marketplace row.
    pub lifecycle_badges: Vec<MarketplaceTruthBadgeClass>,
    /// Compatibility label copied from the marketplace row.
    pub compatibility_label_class: MarketplaceCompatibilityLabelClass,
    /// Compatibility report row consumed by the marketplace row.
    pub compatibility_report_row_id: String,
    /// Compatibility report generated-at timestamp consumed by the row.
    pub compatibility_report_generated_at: String,
    /// Support chips copied from the marketplace row.
    pub support_chips: Vec<MarketplaceSupportChipClass>,
    /// Trust chips copied from the marketplace row.
    pub trust_chips: Vec<MarketplaceTrustChipClass>,
    /// Script or native-build risk disclosure.
    pub script_risk: ScriptRiskDisclosure,
    /// Manifest changes expected before mutation.
    pub manifest_changes: Vec<ManifestChangeRow>,
    /// Permission deltas copied from native install review.
    pub permission_delta_entries: Vec<DeclaredVsEffectiveDiffEntry>,
    /// Number of permission deltas copied from native install review.
    pub permission_delta_count: usize,
    /// Lockfile or generated-file impact disclosure.
    pub lockfile_impact: LockfileImpact,
    /// Activation budget copied from native install review.
    pub activation_budget: ActivationBudget,
    /// Runtime-cost class copied from native install review.
    pub runtime_cost_class: RuntimeCostClass,
    /// Runtime-cost evidence class copied from native install review.
    pub runtime_cost_evidence_class: RuntimeCostEvidenceClass,
    /// Activation trigger refs copied from native install review.
    pub activation_trigger_refs: Vec<String>,
    /// Budget axis refs copied from native install review.
    pub budget_axis_refs: Vec<String>,
    /// Quarantine, revocation, and rollback state copied from the catalog descriptor.
    pub quarantine_revocation: QuarantineRevocationState,
    /// Native install-review decision.
    pub install_review_decision_class: InstallReviewDecisionClass,
    /// Native install-review decision reason.
    pub install_review_decision_reason_class: InstallReviewDecisionReasonClass,
    /// True when the native install review allows mutation.
    pub mutation_allowed_by_install_review: bool,
    /// True when the shared fact grid blocks install or update before mutation.
    pub blocks_install_or_update: bool,
    /// Metadata-safe workspace-change summary.
    pub workspace_change_summary: String,
    /// Metadata-safe fact-grid summary.
    pub fact_grid_summary: String,
    /// Projection timestamp.
    pub generated_at: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Metadata-safe support export derived from a marketplace fact grid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceFactGridSupportExportRecord {
    /// Discriminator for this record family.
    pub record_kind: String,
    /// Schema version for this record.
    pub marketplace_fact_grid_schema_version: u32,
    /// Stable support-export id.
    pub export_id: String,
    /// Source fact-grid ref.
    pub fact_grid_ref: String,
    /// Source marketplace row ref.
    pub marketplace_row_ref: String,
    /// Source catalog descriptor ref.
    pub catalog_descriptor_ref: String,
    /// Native install-review ref.
    pub install_review_ref: String,
    /// Extension identity.
    pub extension_identity: String,
    /// Extension version.
    pub extension_version: String,
    /// Client scope class.
    pub client_scope_class: ClientScopeClass,
    /// Registry source class.
    pub registry_source_class: CatalogRegistrySourceClass,
    /// Compatibility label rendered on the grid.
    pub compatibility_label_class: MarketplaceCompatibilityLabelClass,
    /// Runtime-cost class rendered on the grid.
    pub runtime_cost_class: RuntimeCostClass,
    /// Script/native-build risk class rendered on the grid.
    pub script_risk_class: ScriptRiskClass,
    /// Lockfile impact class rendered on the grid.
    pub lockfile_impact_class: LockfileImpactClass,
    /// Manifest-change count.
    pub manifest_change_count: usize,
    /// Permission-delta count.
    pub permission_delta_count: usize,
    /// Revocation state.
    pub revocation_state_class: RevocationStateClass,
    /// True when install or update is blocked before mutation.
    pub blocks_install_or_update: bool,
    /// Metadata-safe summary.
    pub export_safe_summary: String,
    /// Redaction class for emitted records.
    pub redaction_class: RedactionClass,
}

/// Typed validation finding emitted by marketplace fact-grid validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceFactGridFinding {
    /// Stable validation check id.
    pub check_id: String,
    /// Human-readable validation message.
    pub message: String,
}

impl MarketplaceFactGridFinding {
    fn new(check_id: &str, message: impl Into<String>) -> Self {
        Self {
            check_id: check_id.to_string(),
            message: message.into(),
        }
    }
}

/// Projects marketplace, catalog, and install-review truth into one fact grid.
pub fn project_marketplace_fact_grid(
    input: MarketplaceFactGridInput<'_>,
) -> Result<MarketplaceFactGridRecord, MarketplaceFactGridFinding> {
    if input.marketplace_row.catalog_descriptor_ref != input.catalog.descriptor_id {
        return Err(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.catalog_descriptor_drift",
            "marketplace row and catalog descriptor refs must match",
        ));
    }
    if input.marketplace_row.install_review_ref != input.install_review.review_id {
        return Err(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.install_review_ref_drift",
            "marketplace row and install review refs must match",
        ));
    }
    if input.marketplace_row.registry_source_class != input.catalog.lifecycle.source_registry_class
    {
        return Err(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.registry_source_drift",
            "marketplace row and catalog descriptor registry source classes must match",
        ));
    }
    if input.manifest_changes.is_empty() {
        return Err(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.manifest_changes_missing",
            "fact grid must describe exact manifest changes or explicitly declare no change",
        ));
    }
    if input.client_scope_summary.trim().is_empty() {
        return Err(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.client_scope_summary_missing",
            "fact grid must explain client scope before mutation",
        ));
    }

    let quarantine_revocation = quarantine_revocation_state(input.catalog);
    let permission_delta_entries = input.install_review.permission_delta_entries.clone();
    let permission_delta_count = permission_delta_entries.len();
    let script_blocks = input.script_risk.script_risk_class.blocks_mutation();
    let lockfile_blocks = input.lockfile_impact.impact_class.blocks_mutation();
    let client_blocks = input.client_scope_class.blocks_current_client();
    let activation_unknown = activation_budget_has_unknown_axis(
        &input.install_review.activation_budget.activation_budget,
    );
    let blocks_install_or_update = input.marketplace_row.blocks_install_or_update
        || quarantine_revocation.install_or_update_blocked
        || script_blocks
        || lockfile_blocks
        || client_blocks
        || activation_unknown
        || !input.install_review.mutation_allowed;

    let workspace_change_summary = format!(
        "Manifest changes: {}; permission deltas: {}; lockfile impact: {}; script/native risk: {}; activation cost: {}.",
        input.manifest_changes.len(),
        permission_delta_count,
        input.lockfile_impact.impact_class.as_str(),
        input.script_risk.script_risk_class.as_str(),
        runtime_cost_token(input.install_review.activation_budget.runtime_cost_class)
    );
    let fact_grid_summary = format!(
        "{} {} scope={} source={:?} lifecycle={:?} compatibility={:?} blocked={}",
        input.marketplace_row.extension_identity,
        input.marketplace_row.extension_version,
        input.client_scope_class.label(),
        input.catalog.lifecycle.source_registry_class,
        input.catalog.lifecycle.lifecycle_state_class,
        input.marketplace_row.compatibility_label_class,
        blocks_install_or_update
    );

    Ok(MarketplaceFactGridRecord {
        record_kind: MARKETPLACE_FACT_GRID_RECORD_KIND.to_string(),
        marketplace_fact_grid_schema_version: MARKETPLACE_FACT_GRID_SCHEMA_VERSION,
        fact_grid_id: input.fact_grid_id.to_string(),
        surface_class: input.surface_class,
        marketplace_row_ref: input.marketplace_row.row_id.clone(),
        catalog_descriptor_ref: input.catalog.descriptor_id.clone(),
        install_review_ref: input.install_review.review_id.clone(),
        extension_identity: input.marketplace_row.extension_identity.clone(),
        extension_version: input.marketplace_row.extension_version.clone(),
        package_id: input.marketplace_row.package_id.clone(),
        display_name: input.marketplace_row.display_name.clone(),
        publisher_display_label: input.marketplace_row.publisher_display_label.clone(),
        client_scope_class: input.client_scope_class,
        client_scope_token: input.client_scope_class.as_str().to_string(),
        client_scope_summary: input.client_scope_summary.to_string(),
        registry_source_class: input.catalog.lifecycle.source_registry_class,
        mirrorability_class: input.catalog.mirror.mirrorability_class,
        catalog_lifecycle_state_class: input.catalog.lifecycle.lifecycle_state_class,
        catalog_decision_class: input.catalog.decision_class,
        lifecycle_badges: input.marketplace_row.lifecycle_badges.clone(),
        compatibility_label_class: input.marketplace_row.compatibility_label_class,
        compatibility_report_row_id: input.marketplace_row.compatibility_report_row_id.clone(),
        compatibility_report_generated_at: input
            .marketplace_row
            .compatibility_report_generated_at
            .clone(),
        support_chips: input.marketplace_row.support_chips.clone(),
        trust_chips: input.marketplace_row.trust_chips.clone(),
        script_risk: input.script_risk,
        manifest_changes: input.manifest_changes,
        permission_delta_entries,
        permission_delta_count,
        lockfile_impact: input.lockfile_impact,
        activation_budget: input
            .install_review
            .activation_budget
            .activation_budget
            .clone(),
        runtime_cost_class: input.install_review.activation_budget.runtime_cost_class,
        runtime_cost_evidence_class: input
            .install_review
            .activation_budget
            .runtime_cost_evidence_class,
        activation_trigger_refs: input
            .install_review
            .activation_budget
            .activation_trigger_refs
            .clone(),
        budget_axis_refs: input
            .install_review
            .activation_budget
            .budget_axis_refs
            .clone(),
        quarantine_revocation,
        install_review_decision_class: input.install_review.decision_class,
        install_review_decision_reason_class: input.install_review.decision_reason_class,
        mutation_allowed_by_install_review: input.install_review.mutation_allowed,
        blocks_install_or_update,
        workspace_change_summary,
        fact_grid_summary,
        generated_at: input.generated_at.to_string(),
        redaction_class: RedactionClass::MetadataSafeDefault,
    })
}

/// Projects a marketplace fact grid into a metadata-safe support export.
pub fn project_marketplace_fact_grid_support_export(
    grid: &MarketplaceFactGridRecord,
    export_id: &str,
) -> MarketplaceFactGridSupportExportRecord {
    MarketplaceFactGridSupportExportRecord {
        record_kind: MARKETPLACE_FACT_GRID_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        marketplace_fact_grid_schema_version: MARKETPLACE_FACT_GRID_SCHEMA_VERSION,
        export_id: export_id.to_string(),
        fact_grid_ref: grid.fact_grid_id.clone(),
        marketplace_row_ref: grid.marketplace_row_ref.clone(),
        catalog_descriptor_ref: grid.catalog_descriptor_ref.clone(),
        install_review_ref: grid.install_review_ref.clone(),
        extension_identity: grid.extension_identity.clone(),
        extension_version: grid.extension_version.clone(),
        client_scope_class: grid.client_scope_class,
        registry_source_class: grid.registry_source_class,
        compatibility_label_class: grid.compatibility_label_class,
        runtime_cost_class: grid.runtime_cost_class,
        script_risk_class: grid.script_risk.script_risk_class,
        lockfile_impact_class: grid.lockfile_impact.impact_class,
        manifest_change_count: grid.manifest_changes.len(),
        permission_delta_count: grid.permission_delta_count,
        revocation_state_class: grid.quarantine_revocation.revocation_state_class,
        blocks_install_or_update: grid.blocks_install_or_update,
        export_safe_summary: format!(
            "{} {} fact_grid={} scope={} source={:?} compatibility={:?} runtime={} script={} lockfile={} manifest_changes={} permission_deltas={} blocked={}",
            grid.extension_identity,
            grid.extension_version,
            grid.fact_grid_id,
            grid.client_scope_class.label(),
            grid.registry_source_class,
            grid.compatibility_label_class,
            runtime_cost_token(grid.runtime_cost_class),
            grid.script_risk.script_risk_class.as_str(),
            grid.lockfile_impact.impact_class.as_str(),
            grid.manifest_changes.len(),
            grid.permission_delta_count,
            grid.blocks_install_or_update
        ),
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validates structural invariants for a marketplace fact-grid record.
pub fn validate_marketplace_fact_grid(
    grid: &MarketplaceFactGridRecord,
) -> Vec<MarketplaceFactGridFinding> {
    let mut findings = Vec::new();

    if grid.record_kind != MARKETPLACE_FACT_GRID_RECORD_KIND {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.record_kind_wrong",
            format!(
                "record_kind must be '{MARKETPLACE_FACT_GRID_RECORD_KIND}'; got {:?}",
                grid.record_kind
            ),
        ));
    }
    if grid.marketplace_fact_grid_schema_version != MARKETPLACE_FACT_GRID_SCHEMA_VERSION {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.schema_version_wrong",
            format!(
                "marketplace_fact_grid_schema_version must be {MARKETPLACE_FACT_GRID_SCHEMA_VERSION}; got {}",
                grid.marketplace_fact_grid_schema_version
            ),
        ));
    }
    if !grid.fact_grid_id.starts_with("marketplace_fact_grid:") {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.id_unprefixed",
            "fact_grid_id must start with 'marketplace_fact_grid:'",
        ));
    }
    if !grid
        .marketplace_row_ref
        .starts_with("marketplace_truth_row:")
    {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.row_ref_unprefixed",
            "marketplace_row_ref must cite a marketplace truth row",
        ));
    }
    if !grid
        .catalog_descriptor_ref
        .starts_with("catalog_descriptor:")
    {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.catalog_descriptor_ref_unprefixed",
            "catalog_descriptor_ref must cite a catalog descriptor",
        ));
    }
    if !grid.install_review_ref.starts_with("install_review_alpha:") {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.install_review_ref_unprefixed",
            "install_review_ref must cite the native install-review packet",
        ));
    }
    if grid.client_scope_summary.trim().is_empty() {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.client_scope_summary_missing",
            "client scope must be explained anywhere package support is claimed",
        ));
    }
    if grid.lifecycle_badges.is_empty()
        || grid.support_chips.is_empty()
        || grid.trust_chips.is_empty()
    {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.marketplace_truth_missing",
            "fact grid must carry lifecycle badges, support chips, and trust chips",
        ));
    }
    if grid.manifest_changes.is_empty() {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.manifest_changes_missing",
            "fact grid must show manifest changes or an explicit no-change row",
        ));
    }
    for change in &grid.manifest_changes {
        if change.change_id.trim().is_empty()
            || change.manifest_ref.trim().is_empty()
            || change.field_path.trim().is_empty()
            || change.summary.trim().is_empty()
        {
            findings.push(MarketplaceFactGridFinding::new(
                "marketplace_fact_grid.manifest_change_incomplete",
                "manifest change rows must carry id, manifest ref, field path, and summary",
            ));
        }
    }
    if grid.script_risk.summary.trim().is_empty() || grid.script_risk.risk_source_refs.is_empty() {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.script_risk_incomplete",
            "script/native-build risk must carry source refs and a summary",
        ));
    }
    if grid.lockfile_impact.summary.trim().is_empty()
        || grid.lockfile_impact.resolver_ref.trim().is_empty()
    {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.lockfile_impact_incomplete",
            "lockfile impact must carry a resolver ref and summary",
        ));
    }
    if grid.lockfile_impact.impact_class.requires_artifact_refs()
        && grid.lockfile_impact.affected_lockfile_refs.is_empty()
        && grid.lockfile_impact.generated_file_refs.is_empty()
    {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.lockfile_refs_missing",
            "lockfile or generated-file churn must name affected refs before mutation",
        ));
    }
    if grid.permission_delta_count != grid.permission_delta_entries.len() {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.permission_delta_count_drift",
            "permission_delta_count must equal permission_delta_entries length",
        ));
    }
    if activation_budget_has_unknown_axis(&grid.activation_budget)
        && grid.install_review_decision_class == InstallReviewDecisionClass::AdmitAfterNativeReview
    {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.activation_budget_unknown_but_admitted",
            "unknown activation budget axes must not be paired with admitted native review",
        ));
    }
    if grid.client_scope_class.blocks_current_client() && !grid.blocks_install_or_update {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.unsupported_client_not_blocked",
            "unsupported client scope must block install or update",
        ));
    }
    if grid.script_risk.script_risk_class.blocks_mutation() && !grid.blocks_install_or_update {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.unknown_script_risk_not_blocked",
            "unknown script/native-build risk must block install or update",
        ));
    }
    if grid.lockfile_impact.impact_class.blocks_mutation() && !grid.blocks_install_or_update {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.unknown_lockfile_impact_not_blocked",
            "unknown lockfile impact must block install or update",
        ));
    }
    if grid.quarantine_revocation.install_or_update_blocked && !grid.blocks_install_or_update {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.revocation_not_blocked",
            "quarantine or revocation state must block install or update",
        ));
    }
    if grid.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid.redaction_not_metadata_safe",
            "marketplace fact grids must be metadata-safe by default",
        ));
    }

    findings
}

/// Validates structural invariants for a marketplace fact-grid support export.
pub fn validate_marketplace_fact_grid_support_export(
    export: &MarketplaceFactGridSupportExportRecord,
) -> Vec<MarketplaceFactGridFinding> {
    let mut findings = Vec::new();

    if export.record_kind != MARKETPLACE_FACT_GRID_SUPPORT_EXPORT_RECORD_KIND {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid_export.record_kind_wrong",
            format!(
                "record_kind must be '{MARKETPLACE_FACT_GRID_SUPPORT_EXPORT_RECORD_KIND}'; got {:?}",
                export.record_kind
            ),
        ));
    }
    if export.marketplace_fact_grid_schema_version != MARKETPLACE_FACT_GRID_SCHEMA_VERSION {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid_export.schema_version_wrong",
            format!(
                "marketplace_fact_grid_schema_version must be {MARKETPLACE_FACT_GRID_SCHEMA_VERSION}; got {}",
                export.marketplace_fact_grid_schema_version
            ),
        ));
    }
    if !export
        .export_id
        .starts_with("marketplace_fact_grid_support_export:")
    {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid_export.id_unprefixed",
            "export_id must start with 'marketplace_fact_grid_support_export:'",
        ));
    }
    if !export.fact_grid_ref.starts_with("marketplace_fact_grid:") {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid_export.fact_grid_ref_unprefixed",
            "fact_grid_ref must cite a marketplace fact grid",
        ));
    }
    if export.export_safe_summary.trim().is_empty() {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid_export.summary_missing",
            "support export must carry a metadata-safe summary",
        ));
    }
    if export.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(MarketplaceFactGridFinding::new(
            "marketplace_fact_grid_export.redaction_not_metadata_safe",
            "marketplace fact-grid support exports must be metadata-safe by default",
        ));
    }

    findings
}

fn quarantine_revocation_state(catalog: &CatalogDescriptorRecord) -> QuarantineRevocationState {
    let install_or_update_blocked = matches!(
        catalog.revocation.revocation_state_class,
        RevocationStateClass::Revoked
            | RevocationStateClass::Quarantined
            | RevocationStateClass::EmergencyDisabled
            | RevocationStateClass::MirrorPromotionRevoked
            | RevocationStateClass::PendingReverify
    ) || matches!(
        catalog.revocation.revocation_snapshot_age_class,
        CatalogRevocationSnapshotAgeClass::Stale
            | CatalogRevocationSnapshotAgeClass::UnverifiedNoSnapshot
    ) || matches!(
        catalog.decision_class,
        CatalogDescriptorDecisionClass::Refused
    );

    QuarantineRevocationState {
        revocation_state_class: catalog.revocation.revocation_state_class,
        revocation_snapshot_age_class: catalog.revocation.revocation_snapshot_age_class,
        last_known_good_version: catalog.revocation.last_known_good_version.clone(),
        rollback_manifest_ref: catalog.revocation.rollback_manifest_ref.clone(),
        emergency_disable_refs: catalog.revocation.emergency_disable_refs.clone(),
        install_or_update_blocked,
    }
}

fn activation_budget_has_unknown_axis(budget: &ActivationBudget) -> bool {
    value_unknown(&budget.cpu)
        || value_unknown(&budget.memory)
        || value_unknown(&budget.startup_cost_ceiling)
        || budget.opt_in_feature_gates.is_empty()
        || budget
            .opt_in_feature_gates
            .iter()
            .any(|gate| value_unknown(gate))
}

fn value_unknown(value: &str) -> bool {
    let value = value.trim();
    value.is_empty() || value.eq_ignore_ascii_case("unknown")
}

const fn runtime_cost_token(class: RuntimeCostClass) -> &'static str {
    match class {
        RuntimeCostClass::RuntimeCostLowNominal => "runtime_cost_low_nominal",
        RuntimeCostClass::RuntimeCostNominal => "runtime_cost_nominal",
        RuntimeCostClass::RuntimeCostElevatedWarmOrIdlePollingBreach => {
            "runtime_cost_elevated_warm_or_idle_polling_breach"
        }
        RuntimeCostClass::RuntimeCostUnknownPendingEvidence => {
            "runtime_cost_unknown_pending_evidence"
        }
        RuntimeCostClass::RuntimeCostQuarantinedUnderCrashLoopOrEgressBreach => {
            "runtime_cost_quarantined_under_crash_loop_or_egress_breach"
        }
    }
}
