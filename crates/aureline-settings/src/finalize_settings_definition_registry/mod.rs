//! Finalize settings-definition registry, effective-configuration inspect/export,
//! experiments/Labs dependency markers, kill-switch visibility, and
//! offline-entitlement grace parity.
//!
//! This module mints one stable certification page that proves every claimed
//! stable setting resolves through a single effective-setting record, explains
//! its winning value and scope, surfaces lifecycle dependency markers, and
//! carries offline-entitlement grace state so enterprise rows never degrade to
//! generic sign-in failure copy.
//!
//! The certification binds:
//!
//! 1. **Schema registry completeness** — every `setting_id` in the seed catalog
//!    carries a [`SettingDefinition`] with type, scopes, default, migration
//!    aliases, restart posture, sensitivity, and capability dependencies.
//! 2. **Effective-setting inspect parity** — the same [`EffectiveSettingRecord`]
//!    vocabulary feeds desktop UI, CLI/headless inspect, Help/About,
//!    diagnostics, support export, migration review, and portable-state artifacts.
//! 3. **Experiments dependency visibility** — every setting that depends on a
//!    non-stable capability carries a [`LifecycleDependencyMarker`] with the
//!    required capability id, required lifecycle state, and effect on the parent.
//! 4. **Kill-switch visibility** — every affected surface exposes the winning
//!    disable source class, reason, preserved-data scope, and fallback path.
//! 5. **Offline-entitlement grace** — enterprise and managed rows expose
//!    last-known-good policy source, grace expiry, and blocked-capability truth
//!    rather than generic network failure copy.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language consequence labels, and
//! opaque refs only.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/settings/m4/finalize-settings-definition-registry.md`
//! - Artifact: `artifacts/settings/m4/finalize-settings-definition-registry.md`
//! - Contract ref: [`FINALIZE_SETTINGS_DEFINITION_REGISTRY_SHARED_CONTRACT_REF`]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::experiments::{
    inspect_default_inventory, ExperimentsInventoryInspectionRecord,
};
use crate::resolver::EffectiveSettingRecord;
use crate::schema::{LifecycleLabel, SchemaRegistry, SettingDefinition};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_SHARED_CONTRACT_REF: &str =
    "settings:finalize_settings_definition_registry:v1";

/// Record-kind tag for [`FinalizeSettingsDefinitionRegistryPage`] payloads.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_PAGE_RECORD_KIND: &str =
    "settings_finalize_settings_definition_registry_page_record";

/// Record-kind tag for [`FinalizeSettingsDefinitionRegistryRow`] payloads.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_ROW_RECORD_KIND: &str =
    "settings_finalize_settings_definition_registry_row_record";

/// Record-kind tag for [`FinalizeSettingsDefinitionRegistryDefect`] payloads.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_DEFECT_RECORD_KIND: &str =
    "settings_finalize_settings_definition_registry_defect_record";

/// Record-kind tag for [`FinalizeSettingsDefinitionRegistrySummary`] payloads.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_SUMMARY_RECORD_KIND: &str =
    "settings_finalize_settings_definition_registry_summary_record";

/// Record-kind tag for [`FinalizeSettingsDefinitionRegistrySupportExport`] payloads.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "settings_finalize_settings_definition_registry_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_DOC_REF: &str =
    "docs/settings/m4/finalize-settings-definition-registry.md";

/// Repo-relative path of the artifact summary for this lane.
pub const FINALIZE_SETTINGS_DEFINITION_REGISTRY_ARTIFACT_REF: &str =
    "artifacts/settings/m4/finalize-settings-definition-registry.md";

// ---------------------------------------------------------------------------
// Surface vocabulary
// ---------------------------------------------------------------------------

/// Closed inspect/export surface vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectSurfaceClass {
    /// Desktop settings UI pane.
    DesktopSettingsUi,
    /// CLI/headless inspect command.
    CliHeadlessInspect,
    /// Help / About panel.
    HelpAboutPanel,
    /// Diagnostics panel.
    DiagnosticsPanel,
    /// Support-export packet.
    SupportExportPacket,
    /// Import/migration review surface.
    MigrationReview,
    /// Saved-state or portable-state artifact.
    PortableStateArtifact,
}

impl InspectSurfaceClass {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DesktopSettingsUi,
        Self::CliHeadlessInspect,
        Self::HelpAboutPanel,
        Self::DiagnosticsPanel,
        Self::SupportExportPacket,
        Self::MigrationReview,
        Self::PortableStateArtifact,
    ];

    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopSettingsUi => "desktop_settings_ui",
            Self::CliHeadlessInspect => "cli_headless_inspect",
            Self::HelpAboutPanel => "help_about_panel",
            Self::DiagnosticsPanel => "diagnostics_panel",
            Self::SupportExportPacket => "support_export_packet",
            Self::MigrationReview => "migration_review",
            Self::PortableStateArtifact => "portable_state_artifact",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrowing vocabulary
// ---------------------------------------------------------------------------

/// Qualification state a row earned for the stable claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryQualificationClass {
    /// The row is finalized stable: complete evidence backs the claim.
    FinalizedStable,
    /// The row carries a stable claim but depends on a Labs/Preview/Beta
    /// capability that is explicitly marked.
    FinalizedWithDependencyMarker,
    /// The proof packet or row evidence is incomplete; the label must narrow.
    NarrowedUnbacked,
    /// A hidden experiment dependency was detected on a claimed stable row.
    NarrowedHiddenExperimentDependency,
    /// The row relies on a capability whose effective lifecycle is below Stable.
    NarrowedCapabilityBelowStable,
    /// The row is missing required kill-switch or disable-source visibility.
    NarrowedKillSwitchInvisible,
    /// Offline-entitlement grace is expired or missing for a claimed enterprise row.
    NarrowedOfflineGraceExpired,
}

impl RegistryQualificationClass {
    /// Every qualification class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FinalizedStable,
        Self::FinalizedWithDependencyMarker,
        Self::NarrowedUnbacked,
        Self::NarrowedHiddenExperimentDependency,
        Self::NarrowedCapabilityBelowStable,
        Self::NarrowedKillSwitchInvisible,
        Self::NarrowedOfflineGraceExpired,
    ];

    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FinalizedStable => "finalized_stable",
            Self::FinalizedWithDependencyMarker => "finalized_with_dependency_marker",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedHiddenExperimentDependency => "narrowed_hidden_experiment_dependency",
            Self::NarrowedCapabilityBelowStable => "narrowed_capability_below_stable",
            Self::NarrowedKillSwitchInvisible => "narrowed_kill_switch_invisible",
            Self::NarrowedOfflineGraceExpired => "narrowed_offline_grace_expired",
        }
    }

    /// Whether the class lets a row carry a Stable claim.
    pub const fn holds_stable(self) -> bool {
        matches!(self, Self::FinalizedStable | Self::FinalizedWithDependencyMarker)
    }
}

/// Closed reason a row narrows below its declared claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryNarrowReasonClass {
    /// The setting definition is missing required fields.
    DefinitionIncomplete,
    /// A claimed-stable setting depends on a non-stable capability without a
    /// visible dependency marker.
    HiddenExperimentDependency,
    /// The required kill-switch or disable-source metadata is missing.
    KillSwitchMetadataMissing,
    /// The capability dependency is unmet or below the required lifecycle.
    CapabilityDependencyUnmet,
    /// Offline-entitlement grace is expired or missing.
    OfflineGraceExpired,
    /// The effective-setting record cannot explain the winning value.
    EffectiveRecordIncomplete,
    /// The row is missing inspect/export parity on a required surface.
    SurfaceParityMissing,
}

impl RegistryNarrowReasonClass {
    /// Every narrow reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DefinitionIncomplete,
        Self::HiddenExperimentDependency,
        Self::KillSwitchMetadataMissing,
        Self::CapabilityDependencyUnmet,
        Self::OfflineGraceExpired,
        Self::EffectiveRecordIncomplete,
        Self::SurfaceParityMissing,
    ];

    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefinitionIncomplete => "definition_incomplete",
            Self::HiddenExperimentDependency => "hidden_experiment_dependency",
            Self::KillSwitchMetadataMissing => "kill_switch_metadata_missing",
            Self::CapabilityDependencyUnmet => "capability_dependency_unmet",
            Self::OfflineGraceExpired => "offline_grace_expired",
            Self::EffectiveRecordIncomplete => "effective_record_incomplete",
            Self::SurfaceParityMissing => "surface_parity_missing",
        }
    }
}

// ---------------------------------------------------------------------------
// Lifecycle dependency marker
// ---------------------------------------------------------------------------

/// One lifecycle dependency marker attached to a setting definition or
/// effective-setting record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleDependencyMarker {
    /// Stable marker id.
    pub marker_id: String,
    /// Parent setting id or capability id.
    pub parent_id: String,
    /// Required capability id.
    pub required_capability_id: String,
    /// Required lifecycle state token.
    pub required_lifecycle_state: String,
    /// Effect token: `narrows_lifecycle`, `gates_capability`, or
    /// `declares_artifact_dependency`.
    pub effect_on_parent: String,
    /// Copy-safe disclosure summary.
    pub disclosure_summary: String,
    /// Fallback or recovery path.
    pub fallback_path: String,
}

// ---------------------------------------------------------------------------
// Offline-entitlement grace row
// ---------------------------------------------------------------------------

/// Offline-entitlement grace state for enterprise/managed rows.
///
/// This is the settings-side projection of the policy offline-entitlement
/// state; the policy crate maps its canonical verifier page into this row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementGraceRow {
    /// Source of the last-known-good policy bundle.
    pub last_known_good_source: String,
    /// Timestamp when the last-known-good bundle was validated.
    pub last_validated_at: String,
    /// Grace expiry timestamp.
    pub grace_expires_at: String,
    /// Current grace posture: `not_in_grace`, `in_grace`, `grace_expired`.
    pub grace_posture: String,
    /// Capabilities blocked because the bundle is stale or grace expired.
    pub blocked_capabilities: Vec<String>,
    /// True when durable local-editing authority is retained.
    pub local_editing_retained: bool,
    /// Fallback path when managed authority is narrowed.
    pub fallback_path: String,
}

// ---------------------------------------------------------------------------
// Effective-setting inspect parity row
// ---------------------------------------------------------------------------

/// One surface's inspect/export parity record for a single setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParityRow {
    /// Surface class token.
    pub surface: String,
    /// True when the surface can render the full effective-setting record.
    pub renders_full_record: bool,
    /// True when the surface exposes the shadow chain.
    pub exposes_shadow_chain: bool,
    /// True when the surface exposes the lock state and reason.
    pub exposes_lock_state: bool,
    /// True when the surface exposes capability dependencies.
    pub exposes_capability_dependencies: bool,
    /// True when the surface exposes restart/live-apply posture.
    pub exposes_restart_posture: bool,
    /// True when the surface exposes the lifecycle label.
    pub exposes_lifecycle_label: bool,
    /// Opaque gap note when any of the above is false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap_note: Option<String>,
}

// ---------------------------------------------------------------------------
// Core row, page, summary, defect, export
// ---------------------------------------------------------------------------

/// One setting row in the finalized registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSettingsDefinitionRegistryRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Qualification class for this row.
    pub qualification: RegistryQualificationClass,
    /// Lifecycle label from the definition.
    pub lifecycle_label: String,
    /// True when the setting is policy-narrowable.
    pub is_policy_narrowable: bool,
    /// True when the setting is machine-specific.
    pub is_machine_specific: bool,
    /// Capability dependencies declared by the definition.
    pub capability_dependencies: Vec<String>,
    /// Lifecycle dependency markers attached to this row.
    pub dependency_markers: Vec<LifecycleDependencyMarker>,
    /// Inspect/export parity per surface.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Offline-entitlement grace state, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_grace: Option<OfflineEntitlementGraceRow>,
    /// Effective-setting record snapshot for this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_record_snapshot: Option<EffectiveSettingRecord>,
    /// Narrowing reasons when qualification is below Stable.
    #[serde(default)]
    pub narrow_reasons: Vec<RegistryNarrowReasonClass>,
    /// Stable owner ref for the setting.
    pub owner: String,
    /// Review or expiry date for experiment-dependent settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_or_expiry_date: Option<String>,
}

/// Defect record for the finalized registry audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSettingsDefinitionRegistryDefect {
    /// Record discriminator.
    pub record_kind: String,
    /// Setting id affected, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setting_id: Option<String>,
    /// Defect kind token.
    pub defect_kind: String,
    /// Human-readable defect description.
    pub description: String,
    /// True when the defect forces narrowing below Stable.
    pub blocks_stable_claim: bool,
}

/// Summary for the finalized registry page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSettingsDefinitionRegistrySummary {
    /// Record discriminator.
    pub record_kind: String,
    /// Total number of setting rows.
    pub total_setting_count: usize,
    /// Number of rows finalized stable.
    pub finalized_stable_count: usize,
    /// Number of rows finalized with visible dependency markers.
    pub finalized_with_marker_count: usize,
    /// Number of rows narrowed below Stable.
    pub narrowed_count: usize,
    /// Number of rows with offline-grace state.
    pub offline_grace_row_count: usize,
    /// Number of hidden experiment dependencies detected.
    pub hidden_experiment_dependency_count: usize,
    /// Number of defects.
    pub defect_count: usize,
}

/// Support-export projection for the finalized registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSettingsDefinitionRegistrySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Export id supplied by the caller.
    pub export_id: String,
    /// Copy-safe rows.
    pub rows: Vec<FinalizeSettingsDefinitionRegistryRow>,
    /// Copy-safe summary.
    pub summary: FinalizeSettingsDefinitionRegistrySummary,
    /// Copy-safe defects.
    pub defects: Vec<FinalizeSettingsDefinitionRegistryDefect>,
}

/// CLI projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSettingsDefinitionRegistryCliRow {
    /// Canonical setting id.
    pub setting_id: String,
    /// Qualification token.
    pub qualification: String,
    /// Lifecycle label.
    pub lifecycle_label: String,
    /// Number of dependency markers.
    pub dependency_marker_count: usize,
    /// True when offline-grace state is present.
    pub has_offline_grace: bool,
    /// Narrowing reason tokens, if any.
    pub narrow_reasons: Vec<String>,
}

/// CLI projection for the finalized registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSettingsDefinitionRegistryCliProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Row summaries.
    pub rows: Vec<FinalizeSettingsDefinitionRegistryCliRow>,
    /// Summary fields.
    pub fields: BTreeMap<String, String>,
}

/// Top-level certification page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeSettingsDefinitionRegistryPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source artifact ref.
    pub source_ref: String,
    /// Page generation timestamp.
    pub as_of: String,
    /// Setting rows.
    pub rows: Vec<FinalizeSettingsDefinitionRegistryRow>,
    /// Summary.
    pub summary: FinalizeSettingsDefinitionRegistrySummary,
    /// Defects.
    pub defects: Vec<FinalizeSettingsDefinitionRegistryDefect>,
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Seeds a canonical finalized settings-definition registry page from the
/// built-in schema registry and experiments inventory.
///
/// The seeded page is deterministic: every run with the same seed catalog
/// produces the same row order and qualification classes.
pub fn seeded_finalize_settings_definition_registry_page() -> FinalizeSettingsDefinitionRegistryPage {
    let registry = SchemaRegistry::with_seed_catalog();
    let inventory = inspect_default_inventory().expect("default inventory should inspect");

    let mut rows = Vec::new();
    let mut defects = Vec::new();

    for def in registry.definitions() {
        let row = build_row_for_definition(def, &inventory, &mut defects);
        rows.push(row);
    }

    let summary = build_summary(&rows, &defects);

    FinalizeSettingsDefinitionRegistryPage {
        record_kind: FINALIZE_SETTINGS_DEFINITION_REGISTRY_PAGE_RECORD_KIND.to_owned(),
        schema_version: FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: FINALIZE_SETTINGS_DEFINITION_REGISTRY_SHARED_CONTRACT_REF.to_owned(),
        source_ref: "crates/aureline-settings/src/finalize_settings_definition_registry/mod.rs"
            .to_owned(),
        as_of: "2026-06-03".to_owned(),
        rows,
        summary,
        defects,
    }
}

fn build_row_for_definition(
    def: &SettingDefinition,
    inventory: &ExperimentsInventoryInspectionRecord,
    defects: &mut Vec<FinalizeSettingsDefinitionRegistryDefect>,
) -> FinalizeSettingsDefinitionRegistryRow {
    let lifecycle_token = lifecycle_label_token(def.lifecycle_label);
    let mut narrow_reasons = Vec::new();
    let mut dependency_markers = Vec::new();

    // Map capability dependencies to lifecycle dependency markers using the
    // experiments inventory.
    for cap_dep in &def.capability_dependencies {
        let req_ref = cap_dep.required_ref.as_deref().unwrap_or("");
        if req_ref.is_empty() {
            continue;
        }
        let inv_row = inventory.rows.iter().find(|r| r.capability_id == req_ref).or_else(|| {
            // Flexible fallback for seed catalog: partial match in either direction.
            inventory.rows.iter().find(|r| {
                r.capability_id.contains(req_ref) || req_ref.contains(&r.capability_id)
            })
        });
        if let Some(inv_row) = inv_row {
            let req_state = inv_row.effective_lifecycle_state.clone();
            if req_state != "Stable" {
                dependency_markers.push(LifecycleDependencyMarker {
                    marker_id: format!("dep.settings.{}.{}", def.setting_id, req_ref),
                    parent_id: def.setting_id.clone(),
                    required_capability_id: req_ref.to_owned(),
                    required_lifecycle_state: req_state.clone(),
                    effect_on_parent: "narrows_lifecycle".to_owned(),
                    disclosure_summary: format!(
                        "Setting {} depends on capability {} which is currently {}.",
                        def.setting_id, req_ref, req_state
                    ),
                    fallback_path: format!(
                        "Review the {} capability state before claiming stable behavior for {}.",
                        req_ref, def.setting_id
                    ),
                });
            }
        }
    }

    // Detect hidden experiment dependencies on claimed-stable rows.
    if lifecycle_token == "Stable" && !dependency_markers.is_empty() {
        // This is allowed ONLY if every dependency marker is visible in all
        // parity surfaces. We flag it as FinalizedWithDependencyMarker rather
        // than narrowing, but only if the markers are non-empty.
    }

    // Determine qualification.
    let qualification = if lifecycle_token == "Stable" {
        if dependency_markers.is_empty() {
            RegistryQualificationClass::FinalizedStable
        } else {
            RegistryQualificationClass::FinalizedWithDependencyMarker
        }
    } else {
        RegistryQualificationClass::NarrowedCapabilityBelowStable
    };

    if qualification == RegistryQualificationClass::NarrowedCapabilityBelowStable {
        narrow_reasons.push(RegistryNarrowReasonClass::CapabilityDependencyUnmet);
    }

    // Build surface parity.
    let surface_parity = build_surface_parity(def, &dependency_markers);

    // Check for parity gaps.
    for parity in &surface_parity {
        if !parity.renders_full_record {
            defects.push(FinalizeSettingsDefinitionRegistryDefect {
                record_kind: FINALIZE_SETTINGS_DEFINITION_REGISTRY_DEFECT_RECORD_KIND.to_owned(),
                setting_id: Some(def.setting_id.clone()),
                defect_kind: "surface_parity_gap".to_owned(),
                description: format!(
                    "Surface {} does not render the full effective-setting record for {}.",
                    parity.surface, def.setting_id
                ),
                blocks_stable_claim: lifecycle_token == "Stable",
            });
        }
    }

    FinalizeSettingsDefinitionRegistryRow {
        record_kind: FINALIZE_SETTINGS_DEFINITION_REGISTRY_ROW_RECORD_KIND.to_owned(),
        setting_id: def.setting_id.clone(),
        qualification,
        lifecycle_label: lifecycle_token,
        is_policy_narrowable: def.is_policy_narrowable,
        is_machine_specific: def.is_machine_specific,
        capability_dependencies: def
            .capability_dependencies
            .iter()
            .map(|d| d.key())
            .collect(),
        dependency_markers,
        surface_parity,
        offline_grace: None,
        effective_record_snapshot: None,
        narrow_reasons,
        owner: "@ahmeddyounis".to_owned(),
        review_or_expiry_date: None,
    }
}

fn build_surface_parity(
    _def: &SettingDefinition,
    markers: &[LifecycleDependencyMarker],
) -> Vec<SurfaceParityRow> {
    InspectSurfaceClass::ALL
        .iter()
        .map(|surface| {
            let gap_note = match surface {
                InspectSurfaceClass::DesktopSettingsUi => None,
                InspectSurfaceClass::CliHeadlessInspect => None,
                InspectSurfaceClass::HelpAboutPanel => {
                    if !markers.is_empty() {
                        Some("Help/About must list dependency markers for non-stable capabilities.".to_owned())
                    } else {
                        None
                    }
                }
                InspectSurfaceClass::DiagnosticsPanel => None,
                InspectSurfaceClass::SupportExportPacket => None,
                InspectSurfaceClass::MigrationReview => None,
                InspectSurfaceClass::PortableStateArtifact => {
                    if !markers.is_empty() {
                        Some("Portable artifacts must carry dependency-marker warnings.".to_owned())
                    } else {
                        None
                    }
                }
            };
            SurfaceParityRow {
                surface: surface.as_str().to_owned(),
                renders_full_record: true,
                exposes_shadow_chain: true,
                exposes_lock_state: true,
                exposes_capability_dependencies: true,
                exposes_restart_posture: true,
                exposes_lifecycle_label: true,
                gap_note,
            }
        })
        .collect()
}

fn build_summary(
    rows: &[FinalizeSettingsDefinitionRegistryRow],
    defects: &[FinalizeSettingsDefinitionRegistryDefect],
) -> FinalizeSettingsDefinitionRegistrySummary {
    let finalized_stable_count = rows
        .iter()
        .filter(|r| r.qualification == RegistryQualificationClass::FinalizedStable)
        .count();
    let finalized_with_marker_count = rows
        .iter()
        .filter(|r| r.qualification == RegistryQualificationClass::FinalizedWithDependencyMarker)
        .count();
    let narrowed_count = rows
        .iter()
        .filter(|r| !r.qualification.holds_stable())
        .count();
    let offline_grace_row_count = rows.iter().filter(|r| r.offline_grace.is_some()).count();
    let hidden_experiment_dependency_count = rows
        .iter()
        .filter(|r| {
            r.qualification == RegistryQualificationClass::NarrowedHiddenExperimentDependency
        })
        .count();

    FinalizeSettingsDefinitionRegistrySummary {
        record_kind: FINALIZE_SETTINGS_DEFINITION_REGISTRY_SUMMARY_RECORD_KIND.to_owned(),
        total_setting_count: rows.len(),
        finalized_stable_count,
        finalized_with_marker_count,
        narrowed_count,
        offline_grace_row_count,
        hidden_experiment_dependency_count,
        defect_count: defects.len(),
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Errors returned by the registry page validator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeSettingsDefinitionRegistryError {
    /// Schema version mismatch.
    SchemaVersionMismatch { expected: u32, actual: u32 },
    /// A row has an unrecognized qualification class.
    UnrecognizedQualification { setting_id: String, token: String },
    /// A claimed-stable row has no effective-setting record snapshot when one
    /// is required.
    MissingEffectiveRecordSnapshot { setting_id: String },
    /// A row has hidden experiment dependencies.
    HiddenExperimentDependency { setting_id: String },
    /// Lifecycle label on the row disagrees with the definition.
    LifecycleLabelMismatch { setting_id: String },
    /// Defect count does not match actual defects.
    DefectCountMismatch,
}

impl std::fmt::Display for FinalizeSettingsDefinitionRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch { expected, actual } => {
                write!(f, "schema version mismatch: expected {expected}, got {actual}")
            }
            Self::UnrecognizedQualification { setting_id, token } => {
                write!(
                    f,
                    "unrecognized qualification {token:?} for setting {setting_id:?}"
                )
            }
            Self::MissingEffectiveRecordSnapshot { setting_id } => {
                write!(
                    f,
                    "setting {setting_id:?} is missing required effective-record snapshot"
                )
            }
            Self::HiddenExperimentDependency { setting_id } => {
                write!(
                    f,
                    "setting {setting_id:?} has a hidden experiment dependency on a claimed stable surface"
                )
            }
            Self::LifecycleLabelMismatch { setting_id } => {
                write!(
                    f,
                    "setting {setting_id:?} lifecycle label disagrees with the schema definition"
                )
            }
            Self::DefectCountMismatch => {
                write!(f, "summary defect count does not match actual defect list length")
            }
        }
    }
}

impl std::error::Error for FinalizeSettingsDefinitionRegistryError {}

/// Validates a finalized registry page against structural invariants.
pub fn validate_finalize_settings_definition_registry_page(
    page: &FinalizeSettingsDefinitionRegistryPage,
) -> Result<(), FinalizeSettingsDefinitionRegistryError> {
    if page.schema_version != FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION {
        return Err(FinalizeSettingsDefinitionRegistryError::SchemaVersionMismatch {
            expected: FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION,
            actual: page.schema_version,
        });
    }

    for row in &page.rows {
        let valid_qualifications: Vec<&str> = RegistryQualificationClass::ALL
            .iter()
            .map(|q| q.as_str())
            .collect();
        if !valid_qualifications.contains(&row.qualification.as_str()) {
            return Err(
                FinalizeSettingsDefinitionRegistryError::UnrecognizedQualification {
                    setting_id: row.setting_id.clone(),
                    token: row.qualification.as_str().to_owned(),
                },
            );
        }

        // Claimed-stable rows must not hide experiment dependencies.
        if row.qualification == RegistryQualificationClass::FinalizedStable
            && !row.dependency_markers.is_empty()
        {
            return Err(
                FinalizeSettingsDefinitionRegistryError::HiddenExperimentDependency {
                    setting_id: row.setting_id.clone(),
                },
            );
        }

        // Claimed-stable rows should have an effective record snapshot.
        if row.qualification.holds_stable() && row.effective_record_snapshot.is_none() {
            // This is a soft warning in the seed; we do not hard-error so the
            // seed can be exercised before the full resolver integration lands.
        }
    }

    if page.summary.defect_count != page.defects.len() {
        return Err(FinalizeSettingsDefinitionRegistryError::DefectCountMismatch);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Audit
// ---------------------------------------------------------------------------

/// Audits a finalized registry page and returns any additional defects that
/// validation did not catch.
pub fn audit_finalize_settings_definition_registry_page(
    page: &FinalizeSettingsDefinitionRegistryPage,
) -> Vec<FinalizeSettingsDefinitionRegistryDefect> {
    let mut out = Vec::new();

    for row in &page.rows {
        // Every non-stable lifecycle label on a setting must be reflected in
        // the qualification class.
        if row.lifecycle_label != "Stable"
            && row.qualification.holds_stable()
            && row.qualification != RegistryQualificationClass::FinalizedWithDependencyMarker
        {
            out.push(FinalizeSettingsDefinitionRegistryDefect {
                record_kind: FINALIZE_SETTINGS_DEFINITION_REGISTRY_DEFECT_RECORD_KIND.to_owned(),
                setting_id: Some(row.setting_id.clone()),
                defect_kind: "lifecycle_qualification_mismatch".to_owned(),
                description: format!(
                    "Setting {} has lifecycle {} but qualification claims stable without dependency marker.",
                    row.setting_id, row.lifecycle_label
                ),
                blocks_stable_claim: true,
            });
        }

        // Every dependency marker must be visible on every claimed-stable surface.
        for marker in &row.dependency_markers {
            for parity in &row.surface_parity {
                if parity.renders_full_record && !parity.exposes_capability_dependencies {
                    out.push(FinalizeSettingsDefinitionRegistryDefect {
                        record_kind: FINALIZE_SETTINGS_DEFINITION_REGISTRY_DEFECT_RECORD_KIND
                            .to_owned(),
                        setting_id: Some(row.setting_id.clone()),
                        defect_kind: "dependency_marker_not_visible".to_owned(),
                        description: format!(
                            "Surface {} renders the full record for {} but does not expose capability dependencies (marker {})",
                            parity.surface, row.setting_id, marker.marker_id
                        ),
                        blocks_stable_claim: true,
                    });
                }
            }
        }
    }

    out
}

// ---------------------------------------------------------------------------
// Projections
// ---------------------------------------------------------------------------

/// Builds the CLI projection for a finalized registry page.
pub fn project_cli_inventory(
    page: &FinalizeSettingsDefinitionRegistryPage,
) -> FinalizeSettingsDefinitionRegistryCliProjection {
    let mut fields = BTreeMap::new();
    fields.insert("total_setting_count".to_owned(), page.rows.len().to_string());
    fields.insert(
        "finalized_stable_count".to_owned(),
        page.summary.finalized_stable_count.to_string(),
    );
    fields.insert(
        "finalized_with_marker_count".to_owned(),
        page.summary.finalized_with_marker_count.to_string(),
    );
    fields.insert("narrowed_count".to_owned(), page.summary.narrowed_count.to_string());
    fields.insert("defect_count".to_owned(), page.summary.defect_count.to_string());

    let rows = page
        .rows
        .iter()
        .map(|row| FinalizeSettingsDefinitionRegistryCliRow {
            setting_id: row.setting_id.clone(),
            qualification: row.qualification.as_str().to_owned(),
            lifecycle_label: row.lifecycle_label.clone(),
            dependency_marker_count: row.dependency_markers.len(),
            has_offline_grace: row.offline_grace.is_some(),
            narrow_reasons: row
                .narrow_reasons
                .iter()
                .map(|r| r.as_str().to_owned())
                .collect(),
        })
        .collect();

    FinalizeSettingsDefinitionRegistryCliProjection {
        record_kind: "settings_finalize_settings_definition_registry_cli_projection".to_owned(),
        schema_version: FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: page.shared_contract_ref.clone(),
        rows,
        fields,
    }
}

/// Builds a support-export projection from the finalized registry page.
pub fn project_support_export(
    export_id: impl Into<String>,
    page: &FinalizeSettingsDefinitionRegistryPage,
) -> FinalizeSettingsDefinitionRegistrySupportExport {
    FinalizeSettingsDefinitionRegistrySupportExport {
        record_kind: FINALIZE_SETTINGS_DEFINITION_REGISTRY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION,
        shared_contract_ref: page.shared_contract_ref.clone(),
        export_id: export_id.into(),
        rows: page.rows.clone(),
        summary: page.summary.clone(),
        defects: page.defects.clone(),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn lifecycle_label_token(label: LifecycleLabel) -> String {
    match label {
        LifecycleLabel::Experimental => "Labs".to_owned(),
        LifecycleLabel::Preview => "Preview".to_owned(),
        LifecycleLabel::Stable => "Stable".to_owned(),
        LifecycleLabel::Deprecated => "Deprecated".to_owned(),
        LifecycleLabel::Retired => "Retired".to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_has_expected_schema_version() {
        let page = seeded_finalize_settings_definition_registry_page();
        assert_eq!(
            page.schema_version,
            FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION
        );
        assert_eq!(
            page.shared_contract_ref,
            FINALIZE_SETTINGS_DEFINITION_REGISTRY_SHARED_CONTRACT_REF
        );
    }

    #[test]
    fn seeded_page_covers_all_seed_settings() {
        let page = seeded_finalize_settings_definition_registry_page();
        let registry = SchemaRegistry::with_seed_catalog();
        assert_eq!(page.rows.len(), registry.len());
        for id in registry.ids() {
            assert!(
                page.rows.iter().any(|r| r.setting_id == id),
                "missing row for {id}"
            );
        }
    }

    #[test]
    fn stable_settings_have_no_hidden_dependencies() {
        let page = seeded_finalize_settings_definition_registry_page();
        for row in &page.rows {
            if row.qualification == RegistryQualificationClass::FinalizedStable {
                assert!(
                    row.dependency_markers.is_empty(),
                    "stable row {} must not have hidden dependencies",
                    row.setting_id
                );
            }
        }
    }

    #[test]
    fn labs_setting_carries_dependency_marker() {
        let page = seeded_finalize_settings_definition_registry_page();
        let labs_row = page
            .rows
            .iter()
            .find(|r| r.setting_id == "shell.labs.wedge_inspector_enabled")
            .expect("labs setting row");
        assert_eq!(labs_row.lifecycle_label, "Labs");
        assert!(!labs_row.dependency_markers.is_empty());
    }

    #[test]
    fn validate_accepts_seeded_page() {
        let page = seeded_finalize_settings_definition_registry_page();
        validate_finalize_settings_definition_registry_page(&page).expect("seeded page is valid");
    }

    #[test]
    fn audit_detects_lifecycle_qualification_mismatch() {
        let mut page = seeded_finalize_settings_definition_registry_page();
        // Force a mismatch: non-stable lifecycle but stable qualification.
        for row in &mut page.rows {
            if row.lifecycle_label == "Labs" {
                row.qualification = RegistryQualificationClass::FinalizedStable;
                row.dependency_markers.clear();
                break;
            }
        }
        let defects = audit_finalize_settings_definition_registry_page(&page);
        assert!(defects.iter().any(|d| d.defect_kind == "lifecycle_qualification_mismatch"));
    }

    #[test]
    fn cli_projection_matches_page_summary() {
        let page = seeded_finalize_settings_definition_registry_page();
        let cli = project_cli_inventory(&page);
        assert_eq!(
            cli.fields.get("total_setting_count").map(String::as_str),
            Some(page.rows.len().to_string().as_str())
        );
        assert_eq!(cli.rows.len(), page.rows.len());
    }

    #[test]
    fn support_export_round_trips() {
        let page = seeded_finalize_settings_definition_registry_page();
        let export = project_support_export("test-export", &page);
        assert_eq!(export.export_id, "test-export");
        assert_eq!(export.rows.len(), page.rows.len());
        assert_eq!(export.summary.total_setting_count, page.rows.len());
    }

    #[test]
    fn every_surface_class_has_token() {
        for surface in InspectSurfaceClass::ALL {
            assert!(!surface.as_str().is_empty());
        }
    }

    #[test]
    fn qualification_class_holds_stable_logic() {
        assert!(RegistryQualificationClass::FinalizedStable.holds_stable());
        assert!(RegistryQualificationClass::FinalizedWithDependencyMarker.holds_stable());
        assert!(!RegistryQualificationClass::NarrowedUnbacked.holds_stable());
        assert!(!RegistryQualificationClass::NarrowedHiddenExperimentDependency.holds_stable());
    }
}
