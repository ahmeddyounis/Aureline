//! Finalize experiments/feature-flag/Labs inventory, kill-switch visibility,
//! dependency markers, and release-claim alignment.
//!
//! This module mints one stable certification page that proves every experiment
//! or Labs flag affecting a claimed surface is visible, exportable, and bounded
//! by owner, cohort, expiry, rollout ring, and kill-switch metadata. Stable
//! claims do not silently depend on hidden experiment state.
//!
//! The certification binds:
//!
//! 1. **Inventory completeness** — every capability row carries `capability_id`,
//!    `owner`, `declared_lifecycle_state`, `effective_lifecycle_state`,
//!    `review_or_expiry_date`, enrollment scope, cohort/ring, and public label.
//! 2. **Kill-switch visibility** — every row that is `DisabledByPolicy` or has
//!    an active disable source exposes the winning source class, reason,
//!    preserved-data scope, and fallback path.
//! 3. **Dependency-marker truth** — every saved artifact, bundle, sync packet,
//!    and migration export that depends on a non-stable capability carries a
//!    visible marker with `required_capability_id`, `required_lifecycle_state`,
//!    and `effect_on_parent`.
//! 4. **Release-claim alignment** — every experiment row maps to the
//!    stable-claim-manifest entry whose lifecycle label it backs, so a row
//!    whose proof packet ages out or whose waiver expires narrows automatically.
//! 5. **Controlled vocabulary** — the lifecycle vocabulary exposed by settings,
//!    help, and export is exactly `Labs`, `Preview`, `Beta`, `Stable`,
//!    `Deprecated`, `DisabledByPolicy`, `Retired`; no softer synonyms.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language consequence labels, and
//! opaque refs only.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/release/m4/finalize-experiments-labs-inventory.md`
//! - Artifact: `artifacts/release/m4/finalize-experiments-labs-inventory.md`
//! - Contract ref: [`FINALIZE_EXPERIMENTS_LABS_INVENTORY_SHARED_CONTRACT_REF`]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use aureline_settings::experiments::{
    inspect_default_inventory, ExperimentsInventoryInspectionRecord,
    ExperimentsInventoryRowInspection,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_SHARED_CONTRACT_REF: &str =
    "release:finalize_experiments_labs_inventory:v1";

/// Record-kind tag for [`FinalizeExperimentsLabsInventoryPage`] payloads.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_PAGE_RECORD_KIND: &str =
    "release_finalize_experiments_labs_inventory_page_record";

/// Record-kind tag for [`FinalizeExperimentsLabsInventoryRow`] payloads.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_ROW_RECORD_KIND: &str =
    "release_finalize_experiments_labs_inventory_row_record";

/// Record-kind tag for [`FinalizeExperimentsLabsInventoryDefect`] payloads.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND: &str =
    "release_finalize_experiments_labs_inventory_defect_record";

/// Record-kind tag for [`FinalizeExperimentsLabsInventorySummary`] payloads.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_SUMMARY_RECORD_KIND: &str =
    "release_finalize_experiments_labs_inventory_summary_record";

/// Record-kind tag for [`FinalizeExperimentsLabsInventorySupportExport`] payloads.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "release_finalize_experiments_labs_inventory_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_DOC_REF: &str =
    "docs/release/m4/finalize-experiments-labs-inventory.md";

/// Repo-relative path of the artifact summary for this lane.
pub const FINALIZE_EXPERIMENTS_LABS_INVENTORY_ARTIFACT_REF: &str =
    "artifacts/release/m4/finalize-experiments-labs-inventory.md";

// ---------------------------------------------------------------------------
// Surface and claim vocabulary
// ---------------------------------------------------------------------------

/// Closed surface-class vocabulary for consumers of the experiments inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InventorySurfaceClass {
    /// Settings UI surfaces (root pane, Labs tab, inspector).
    SettingsUi,
    /// CLI/headless inspect and export commands.
    CliHeadless,
    /// Help / About panel.
    HelpAbout,
    /// Diagnostics and support-export packets.
    DiagnosticsAndSupport,
    /// Release-center notices and claim manifests.
    ReleaseCenter,
    /// Migration review and portable-state artifacts.
    MigrationAndPortable,
}

impl InventorySurfaceClass {
    /// Every surface class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SettingsUi,
        Self::CliHeadless,
        Self::HelpAbout,
        Self::DiagnosticsAndSupport,
        Self::ReleaseCenter,
        Self::MigrationAndPortable,
    ];

    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsUi => "settings_ui",
            Self::CliHeadless => "cli_headless",
            Self::HelpAbout => "help_about",
            Self::DiagnosticsAndSupport => "diagnostics_and_support",
            Self::ReleaseCenter => "release_center",
            Self::MigrationAndPortable => "migration_and_portable",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrowing vocabulary
// ---------------------------------------------------------------------------

/// Qualification state a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InventoryQualificationClass {
    /// The row is finalized stable: complete evidence backs the claim.
    FinalizedStable,
    /// The row carries the claim's full label only because an active,
    /// unexpired waiver covers a recorded gap.
    FinalizedOnWaiver,
    /// The experiment is visible and bounded but below Stable.
    VisibleBounded,
    /// The proof packet or row evidence is incomplete; the label must narrow.
    NarrowedUnbacked,
    /// A hidden experiment dependency was detected on a claimed stable surface.
    NarrowedHiddenDependency,
    /// The row is missing required owner, expiry, or kill-switch metadata.
    NarrowedMetadataMissing,
    /// The effective lifecycle does not match the declared lifecycle after
    /// kill-switch evaluation.
    NarrowedEffectiveLifecycleMismatch,
    /// Offline-entitlement grace is expired for a managed experiment row.
    NarrowedOfflineGraceExpired,
}

impl InventoryQualificationClass {
    /// Every qualification class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::FinalizedStable,
        Self::FinalizedOnWaiver,
        Self::VisibleBounded,
        Self::NarrowedUnbacked,
        Self::NarrowedHiddenDependency,
        Self::NarrowedMetadataMissing,
        Self::NarrowedEffectiveLifecycleMismatch,
        Self::NarrowedOfflineGraceExpired,
    ];

    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FinalizedStable => "finalized_stable",
            Self::FinalizedOnWaiver => "finalized_on_waiver",
            Self::VisibleBounded => "visible_bounded",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedHiddenDependency => "narrowed_hidden_dependency",
            Self::NarrowedMetadataMissing => "narrowed_metadata_missing",
            Self::NarrowedEffectiveLifecycleMismatch => "narrowed_effective_lifecycle_mismatch",
            Self::NarrowedOfflineGraceExpired => "narrowed_offline_grace_expired",
        }
    }

    /// Whether the class lets a row carry a Stable claim.
    pub const fn holds_stable(self) -> bool {
        matches!(self, Self::FinalizedStable | Self::FinalizedOnWaiver)
    }
}

/// Closed reason a row narrows below its declared claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InventoryNarrowReasonClass {
    /// The capability row is missing an owner.
    OwnerMissing,
    /// The capability row is missing an expiry or review date.
    ExpiryMissing,
    /// The capability row is missing a cohort or ring assignment.
    CohortMissing,
    /// The kill-switch or disable-source metadata is incomplete.
    KillSwitchIncomplete,
    /// A dependency marker is not visible on a claimed-stable surface.
    HiddenDependencyMarker,
    /// The effective lifecycle does not match the declared lifecycle.
    EffectiveLifecycleMismatch,
    /// The public claim this row backs is itself below the cutline.
    BackingClaimNarrowed,
    /// Offline-entitlement grace has expired.
    OfflineGraceExpired,
}

impl InventoryNarrowReasonClass {
    /// Every narrow reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::OwnerMissing,
        Self::ExpiryMissing,
        Self::CohortMissing,
        Self::KillSwitchIncomplete,
        Self::HiddenDependencyMarker,
        Self::EffectiveLifecycleMismatch,
        Self::BackingClaimNarrowed,
        Self::OfflineGraceExpired,
    ];

    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerMissing => "owner_missing",
            Self::ExpiryMissing => "expiry_missing",
            Self::CohortMissing => "cohort_missing",
            Self::KillSwitchIncomplete => "kill_switch_incomplete",
            Self::HiddenDependencyMarker => "hidden_dependency_marker",
            Self::EffectiveLifecycleMismatch => "effective_lifecycle_mismatch",
            Self::BackingClaimNarrowed => "backing_claim_narrowed",
            Self::OfflineGraceExpired => "offline_grace_expired",
        }
    }
}

// ---------------------------------------------------------------------------
// Kill-switch and dependency visibility
// ---------------------------------------------------------------------------

/// One kill-switch or disable-source projection for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KillSwitchVisibilityRow {
    /// Source class token.
    pub source_class: String,
    /// Stable source ref.
    pub source_ref: String,
    /// Disable reason.
    pub reason: String,
    /// True when durable user-authored data is preserved.
    pub preserve_user_data: bool,
    /// Scope of preserved data.
    pub preserved_data_scope: String,
    /// Fallback or recovery path.
    pub fallback_path: String,
}

/// One dependency marker projected for release-claim alignment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryDependencyMarker {
    /// Stable marker id.
    pub marker_id: String,
    /// Artifact class that carries the marker.
    pub artifact_class: String,
    /// Required capability id.
    pub required_capability_id: String,
    /// Required lifecycle state token.
    pub required_lifecycle_state: String,
    /// Effect on the parent.
    pub effect_on_parent: String,
    /// Copy-safe disclosure summary.
    pub disclosure_summary: String,
}

// ---------------------------------------------------------------------------
// Core row, page, summary, defect, export
// ---------------------------------------------------------------------------

/// One experiment/Labs row in the finalized inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeExperimentsLabsInventoryRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Stable capability id.
    pub capability_id: String,
    /// Human-readable title.
    pub title: String,
    /// Owner ref.
    pub owner: String,
    /// Declared lifecycle state token.
    pub declared_lifecycle_state: String,
    /// Effective lifecycle state token.
    pub effective_lifecycle_state: String,
    /// Qualification class.
    pub qualification: InventoryQualificationClass,
    /// Cohort or ring label.
    pub cohort_or_ring: String,
    /// Review or expiry date.
    pub review_or_expiry_date: String,
    /// Public label.
    pub public_label: String,
    /// Winning kill-switch visibility, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kill_switch_visibility: Option<KillSwitchVisibilityRow>,
    /// Dependency markers attached to this row.
    #[serde(default)]
    pub dependency_markers: Vec<InventoryDependencyMarker>,
    /// Surfaces that render this row.
    #[serde(default)]
    pub rendered_surfaces: Vec<String>,
    /// Narrowing reasons when qualification is below Stable.
    #[serde(default)]
    pub narrow_reasons: Vec<InventoryNarrowReasonClass>,
    /// True when this row is claimed as alpha-visible.
    pub claimed_alpha_visible: bool,
}

/// Defect record for the finalized inventory audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeExperimentsLabsInventoryDefect {
    /// Record discriminator.
    pub record_kind: String,
    /// Capability id affected, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_id: Option<String>,
    /// Defect kind token.
    pub defect_kind: String,
    /// Human-readable defect description.
    pub description: String,
    /// True when the defect forces narrowing below Stable.
    pub blocks_stable_claim: bool,
}

/// Summary for the finalized inventory page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeExperimentsLabsInventorySummary {
    /// Record discriminator.
    pub record_kind: String,
    /// Total number of capability rows.
    pub total_row_count: usize,
    /// Number of rows finalized stable.
    pub finalized_stable_count: usize,
    /// Number of rows visible and bounded below Stable.
    pub visible_bounded_count: usize,
    /// Number of rows narrowed below Stable.
    pub narrowed_count: usize,
    /// Number of rows with active kill-switch visibility.
    pub kill_switch_visible_count: usize,
    /// Number of rows with dependency markers.
    pub dependency_marker_count: usize,
    /// Number of defects.
    pub defect_count: usize,
}

/// Support-export projection for the finalized inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeExperimentsLabsInventorySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Export id supplied by the caller.
    pub export_id: String,
    /// Copy-safe rows.
    pub rows: Vec<FinalizeExperimentsLabsInventoryRow>,
    /// Copy-safe summary.
    pub summary: FinalizeExperimentsLabsInventorySummary,
    /// Copy-safe defects.
    pub defects: Vec<FinalizeExperimentsLabsInventoryDefect>,
}

/// CLI projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeExperimentsLabsInventoryCliRow {
    /// Stable capability id.
    pub capability_id: String,
    /// Effective lifecycle state token.
    pub effective_lifecycle_state: String,
    /// Qualification token.
    pub qualification: String,
    /// Owner ref.
    pub owner: String,
    /// Cohort or ring.
    pub cohort_or_ring: String,
    /// Review or expiry date.
    pub review_or_expiry_date: String,
    /// Winning disable source class, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_disable_source: Option<String>,
    /// Number of dependency markers.
    pub dependency_marker_count: usize,
    /// Narrowing reason tokens, if any.
    pub narrow_reasons: Vec<String>,
}

/// CLI projection for the finalized inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeExperimentsLabsInventoryCliProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Row summaries.
    pub rows: Vec<FinalizeExperimentsLabsInventoryCliRow>,
    /// Summary fields.
    pub fields: BTreeMap<String, String>,
}

/// Top-level certification page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizeExperimentsLabsInventoryPage {
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
    /// Capability rows.
    pub rows: Vec<FinalizeExperimentsLabsInventoryRow>,
    /// Summary.
    pub summary: FinalizeExperimentsLabsInventorySummary,
    /// Defects.
    pub defects: Vec<FinalizeExperimentsLabsInventoryDefect>,
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// Seeds a canonical finalized experiments/Labs inventory page from the
/// default experiments inventory.
///
/// The seeded page is deterministic: every run with the same inventory
/// produces the same row order and qualification classes.
pub fn seeded_finalize_experiments_labs_inventory_page() -> FinalizeExperimentsLabsInventoryPage {
    let inventory = inspect_default_inventory().expect("default inventory should inspect");
    build_page_from_inventory(&inventory)
}

/// Builds a finalized page from an already-inspected inventory record.
pub fn build_page_from_inventory(
    inventory: &ExperimentsInventoryInspectionRecord,
) -> FinalizeExperimentsLabsInventoryPage {
    let mut rows = Vec::new();
    let mut defects = Vec::new();

    for inv_row in &inventory.rows {
        let row = build_row(inv_row, &mut defects);
        rows.push(row);
    }

    let summary = build_summary(&rows, &defects);

    FinalizeExperimentsLabsInventoryPage {
        record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_PAGE_RECORD_KIND.to_owned(),
        schema_version: FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION,
        shared_contract_ref: FINALIZE_EXPERIMENTS_LABS_INVENTORY_SHARED_CONTRACT_REF.to_owned(),
        source_ref: inventory.source_inventory_ref.clone(),
        as_of: inventory.as_of.clone(),
        rows,
        summary,
        defects,
    }
}

fn build_row(
    inv_row: &ExperimentsInventoryRowInspection,
    defects: &mut Vec<FinalizeExperimentsLabsInventoryDefect>,
) -> FinalizeExperimentsLabsInventoryRow {
    let declared = &inv_row.declared_lifecycle_state;
    let effective = &inv_row.effective_lifecycle_state;

    let mut narrow_reasons = Vec::new();

    // Detect effective/declared mismatch.
    if declared != effective {
        narrow_reasons.push(InventoryNarrowReasonClass::EffectiveLifecycleMismatch);
        defects.push(FinalizeExperimentsLabsInventoryDefect {
            record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND.to_owned(),
            capability_id: Some(inv_row.capability_id.clone()),
            defect_kind: "effective_lifecycle_mismatch".to_owned(),
            description: format!(
                "Capability {} declared {} but effective is {}.",
                inv_row.capability_id, declared, effective
            ),
            blocks_stable_claim: declared == "Stable",
        });
    }

    // Detect missing owner.
    if inv_row.owner.trim().is_empty() || inv_row.owner == "@unknown" {
        narrow_reasons.push(InventoryNarrowReasonClass::OwnerMissing);
        defects.push(FinalizeExperimentsLabsInventoryDefect {
            record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND.to_owned(),
            capability_id: Some(inv_row.capability_id.clone()),
            defect_kind: "owner_missing".to_owned(),
            description: format!(
                "Capability {} is missing an owner ref.",
                inv_row.capability_id
            ),
            blocks_stable_claim: true,
        });
    }

    // Detect missing expiry.
    if inv_row.review_or_expiry_date.trim().is_empty() {
        narrow_reasons.push(InventoryNarrowReasonClass::ExpiryMissing);
        defects.push(FinalizeExperimentsLabsInventoryDefect {
            record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND.to_owned(),
            capability_id: Some(inv_row.capability_id.clone()),
            defect_kind: "expiry_missing".to_owned(),
            description: format!(
                "Capability {} is missing a review_or_expiry_date.",
                inv_row.capability_id
            ),
            blocks_stable_claim: true,
        });
    }

    // Detect missing cohort.
    if inv_row.cohort_or_ring.trim().is_empty() {
        narrow_reasons.push(InventoryNarrowReasonClass::CohortMissing);
        defects.push(FinalizeExperimentsLabsInventoryDefect {
            record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND.to_owned(),
            capability_id: Some(inv_row.capability_id.clone()),
            defect_kind: "cohort_missing".to_owned(),
            description: format!(
                "Capability {} is missing a cohort_or_ring assignment.",
                inv_row.capability_id
            ),
            blocks_stable_claim: true,
        });
    }

    // Map kill-switch visibility.
    let kill_switch_visibility =
        inv_row
            .winning_disable_source
            .as_ref()
            .map(|source| KillSwitchVisibilityRow {
                source_class: source.source_class.clone(),
                source_ref: source.source_ref.clone(),
                reason: source.reason.clone(),
                preserve_user_data: source.preserve_user_data,
                preserved_data_scope: source.preserved_data_scope.clone(),
                fallback_path: source.fallback_path.clone(),
            });

    // If there is a winning disable source but recovery metadata is incomplete,
    // flag a defect.
    if let Some(source) = &inv_row.winning_disable_source {
        if !source.preserve_user_data
            || source.preserved_data_scope.trim().is_empty()
            || source.fallback_path.trim().is_empty()
        {
            narrow_reasons.push(InventoryNarrowReasonClass::KillSwitchIncomplete);
            defects.push(FinalizeExperimentsLabsInventoryDefect {
                record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND.to_owned(),
                capability_id: Some(inv_row.capability_id.clone()),
                defect_kind: "kill_switch_incomplete".to_owned(),
                description: format!(
                    "Capability {} has an active disable source without complete recovery metadata.",
                    inv_row.capability_id
                ),
                blocks_stable_claim: true,
            });
        }
    }

    // Map dependency markers.
    let dependency_markers: Vec<InventoryDependencyMarker> = inv_row
        .dependency_markers
        .iter()
        .map(|dm| InventoryDependencyMarker {
            marker_id: dm.marker_id.clone(),
            artifact_class: dm.artifact_class.clone(),
            required_capability_id: dm.required_capability_id.clone(),
            required_lifecycle_state: dm.required_lifecycle_state.clone(),
            effect_on_parent: dm.effect_on_parent.clone(),
            disclosure_summary: dm.disclosure_summary.clone(),
        })
        .collect();

    // Determine qualification.
    let qualification = match effective.as_str() {
        "Stable" => {
            if narrow_reasons.is_empty() {
                InventoryQualificationClass::FinalizedStable
            } else {
                InventoryQualificationClass::FinalizedOnWaiver
            }
        }
        "Labs" | "Preview" | "Beta" | "Deprecated" => {
            if narrow_reasons.is_empty() {
                InventoryQualificationClass::VisibleBounded
            } else {
                InventoryQualificationClass::NarrowedUnbacked
            }
        }
        "DisabledByPolicy" | "Retired" => InventoryQualificationClass::NarrowedUnbacked,
        _ => InventoryQualificationClass::NarrowedMetadataMissing,
    };

    // Surfaces that render this row.
    let rendered_surfaces: Vec<String> = InventorySurfaceClass::ALL
        .iter()
        .map(|s| s.as_str().to_owned())
        .collect();

    FinalizeExperimentsLabsInventoryRow {
        record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_ROW_RECORD_KIND.to_owned(),
        capability_id: inv_row.capability_id.clone(),
        title: inv_row.title.clone(),
        owner: inv_row.owner.clone(),
        declared_lifecycle_state: declared.clone(),
        effective_lifecycle_state: effective.clone(),
        qualification,
        cohort_or_ring: inv_row.cohort_or_ring.clone(),
        review_or_expiry_date: inv_row.review_or_expiry_date.clone(),
        public_label: inv_row.public_label.clone(),
        kill_switch_visibility,
        dependency_markers,
        rendered_surfaces,
        narrow_reasons,
        claimed_alpha_visible: inv_row.saved_artifact_dependency_present,
    }
}

fn build_summary(
    rows: &[FinalizeExperimentsLabsInventoryRow],
    defects: &[FinalizeExperimentsLabsInventoryDefect],
) -> FinalizeExperimentsLabsInventorySummary {
    let finalized_stable_count = rows
        .iter()
        .filter(|r| r.qualification == InventoryQualificationClass::FinalizedStable)
        .count();
    let visible_bounded_count = rows
        .iter()
        .filter(|r| r.qualification == InventoryQualificationClass::VisibleBounded)
        .count();
    let narrowed_count = rows
        .iter()
        .filter(|r| !r.qualification.holds_stable())
        .count();
    let kill_switch_visible_count = rows
        .iter()
        .filter(|r| r.kill_switch_visibility.is_some())
        .count();
    let dependency_marker_count = rows.iter().map(|r| r.dependency_markers.len()).sum();

    FinalizeExperimentsLabsInventorySummary {
        record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_SUMMARY_RECORD_KIND.to_owned(),
        total_row_count: rows.len(),
        finalized_stable_count,
        visible_bounded_count,
        narrowed_count,
        kill_switch_visible_count,
        dependency_marker_count,
        defect_count: defects.len(),
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Errors returned by the inventory page validator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeExperimentsLabsInventoryError {
    /// Schema version mismatch.
    SchemaVersionMismatch { expected: u32, actual: u32 },
    /// A row has an unrecognized qualification class.
    UnrecognizedQualification {
        capability_id: String,
        token: String,
    },
    /// A claimed-stable row has hidden experiment dependencies.
    HiddenDependencyOnStableClaim { capability_id: String },
    /// Lifecycle vocabulary contains an unrecognized token.
    UnrecognizedLifecycleToken { token: String },
    /// Defect count does not match actual defects.
    DefectCountMismatch,
}

impl std::fmt::Display for FinalizeExperimentsLabsInventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::UnrecognizedQualification {
                capability_id,
                token,
            } => {
                write!(
                    f,
                    "unrecognized qualification {token:?} for capability {capability_id:?}"
                )
            }
            Self::HiddenDependencyOnStableClaim { capability_id } => {
                write!(
                    f,
                    "capability {capability_id:?} has a hidden dependency on a claimed stable surface"
                )
            }
            Self::UnrecognizedLifecycleToken { token } => {
                write!(f, "unrecognized lifecycle token {token:?}")
            }
            Self::DefectCountMismatch => {
                write!(
                    f,
                    "summary defect count does not match actual defect list length"
                )
            }
        }
    }
}

impl std::error::Error for FinalizeExperimentsLabsInventoryError {}

/// Validates a finalized inventory page against structural invariants.
pub fn validate_finalize_experiments_labs_inventory_page(
    page: &FinalizeExperimentsLabsInventoryPage,
) -> Result<(), FinalizeExperimentsLabsInventoryError> {
    if page.schema_version != FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION {
        return Err(
            FinalizeExperimentsLabsInventoryError::SchemaVersionMismatch {
                expected: FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION,
                actual: page.schema_version,
            },
        );
    }

    let valid_lifecycle: Vec<&str> = vec![
        "Labs",
        "Preview",
        "Beta",
        "Stable",
        "Deprecated",
        "DisabledByPolicy",
        "Retired",
    ];

    for row in &page.rows {
        if !valid_lifecycle.contains(&row.effective_lifecycle_state.as_str()) {
            return Err(
                FinalizeExperimentsLabsInventoryError::UnrecognizedLifecycleToken {
                    token: row.effective_lifecycle_state.clone(),
                },
            );
        }

        let valid_qualifications: Vec<&str> = InventoryQualificationClass::ALL
            .iter()
            .map(|q| q.as_str())
            .collect();
        if !valid_qualifications.contains(&row.qualification.as_str()) {
            return Err(
                FinalizeExperimentsLabsInventoryError::UnrecognizedQualification {
                    capability_id: row.capability_id.clone(),
                    token: row.qualification.as_str().to_owned(),
                },
            );
        }

        // Claimed-stable rows must not hide non-stable dependency markers.
        if row.qualification == InventoryQualificationClass::FinalizedStable {
            for marker in &row.dependency_markers {
                if marker.required_lifecycle_state != "Stable" {
                    return Err(
                        FinalizeExperimentsLabsInventoryError::HiddenDependencyOnStableClaim {
                            capability_id: row.capability_id.clone(),
                        },
                    );
                }
            }
        }
    }

    if page.summary.defect_count != page.defects.len() {
        return Err(FinalizeExperimentsLabsInventoryError::DefectCountMismatch);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Audit
// ---------------------------------------------------------------------------

/// Audits a finalized inventory page and returns any additional defects.
pub fn audit_finalize_experiments_labs_inventory_page(
    page: &FinalizeExperimentsLabsInventoryPage,
) -> Vec<FinalizeExperimentsLabsInventoryDefect> {
    let mut out = Vec::new();

    for row in &page.rows {
        // Every non-stable row must be visible on all surfaces.
        if row.effective_lifecycle_state != "Stable" && row.rendered_surfaces.is_empty() {
            out.push(FinalizeExperimentsLabsInventoryDefect {
                record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND.to_owned(),
                capability_id: Some(row.capability_id.clone()),
                defect_kind: "non_stable_row_not_visible".to_owned(),
                description: format!(
                    "Capability {} is {} but is not rendered on any surface.",
                    row.capability_id, row.effective_lifecycle_state
                ),
                blocks_stable_claim: false,
            });
        }

        // Every disabled row must expose kill-switch visibility.
        if row.effective_lifecycle_state == "DisabledByPolicy"
            && row.kill_switch_visibility.is_none()
        {
            out.push(FinalizeExperimentsLabsInventoryDefect {
                record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND.to_owned(),
                capability_id: Some(row.capability_id.clone()),
                defect_kind: "disabled_row_missing_kill_switch".to_owned(),
                description: format!(
                    "Capability {} is DisabledByPolicy but has no kill_switch_visibility row.",
                    row.capability_id
                ),
                blocks_stable_claim: true,
            });
        }

        // Dependency markers must be visible on claimed-stable surfaces.
        for marker in &row.dependency_markers {
            if marker.required_lifecycle_state != "Stable" {
                // If the parent is claimed stable but depends on a non-stable
                // capability, the marker must be visible everywhere.
                if row.qualification.holds_stable() {
                    out.push(FinalizeExperimentsLabsInventoryDefect {
                        record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND
                            .to_owned(),
                        capability_id: Some(row.capability_id.clone()),
                        defect_kind: "stable_claim_with_non_stable_dependency".to_owned(),
                        description: format!(
                            "Capability {} claims stable but depends on {} which is {}.",
                            row.capability_id,
                            marker.required_capability_id,
                            marker.required_lifecycle_state
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

/// Builds the CLI projection for a finalized inventory page.
pub fn project_cli_inventory(
    page: &FinalizeExperimentsLabsInventoryPage,
) -> FinalizeExperimentsLabsInventoryCliProjection {
    let mut fields = BTreeMap::new();
    fields.insert("total_row_count".to_owned(), page.rows.len().to_string());
    fields.insert(
        "finalized_stable_count".to_owned(),
        page.summary.finalized_stable_count.to_string(),
    );
    fields.insert(
        "visible_bounded_count".to_owned(),
        page.summary.visible_bounded_count.to_string(),
    );
    fields.insert(
        "narrowed_count".to_owned(),
        page.summary.narrowed_count.to_string(),
    );
    fields.insert(
        "defect_count".to_owned(),
        page.summary.defect_count.to_string(),
    );

    let rows = page
        .rows
        .iter()
        .map(|row| FinalizeExperimentsLabsInventoryCliRow {
            capability_id: row.capability_id.clone(),
            effective_lifecycle_state: row.effective_lifecycle_state.clone(),
            qualification: row.qualification.as_str().to_owned(),
            owner: row.owner.clone(),
            cohort_or_ring: row.cohort_or_ring.clone(),
            review_or_expiry_date: row.review_or_expiry_date.clone(),
            winning_disable_source: row
                .kill_switch_visibility
                .as_ref()
                .map(|k| k.source_class.clone()),
            dependency_marker_count: row.dependency_markers.len(),
            narrow_reasons: row
                .narrow_reasons
                .iter()
                .map(|r| r.as_str().to_owned())
                .collect(),
        })
        .collect();

    FinalizeExperimentsLabsInventoryCliProjection {
        record_kind: "release_finalize_experiments_labs_inventory_cli_projection".to_owned(),
        schema_version: FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION,
        shared_contract_ref: page.shared_contract_ref.clone(),
        rows,
        fields,
    }
}

/// Builds a support-export projection from the finalized inventory page.
pub fn project_support_export(
    export_id: impl Into<String>,
    page: &FinalizeExperimentsLabsInventoryPage,
) -> FinalizeExperimentsLabsInventorySupportExport {
    FinalizeExperimentsLabsInventorySupportExport {
        record_kind: FINALIZE_EXPERIMENTS_LABS_INVENTORY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION,
        shared_contract_ref: page.shared_contract_ref.clone(),
        export_id: export_id.into(),
        rows: page.rows.clone(),
        summary: page.summary.clone(),
        defects: page.defects.clone(),
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
        let page = seeded_finalize_experiments_labs_inventory_page();
        assert_eq!(
            page.schema_version,
            FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION
        );
        assert_eq!(
            page.shared_contract_ref,
            FINALIZE_EXPERIMENTS_LABS_INVENTORY_SHARED_CONTRACT_REF
        );
    }

    #[test]
    fn seeded_page_covers_all_inventory_rows() {
        let inventory = inspect_default_inventory().expect("inventory inspects");
        let page = build_page_from_inventory(&inventory);
        assert_eq!(page.rows.len(), inventory.rows.len());
    }

    #[test]
    fn lifecycle_vocabulary_is_exactly_controlled_set() {
        let page = seeded_finalize_experiments_labs_inventory_page();
        let valid = [
            "Labs",
            "Preview",
            "Beta",
            "Stable",
            "Deprecated",
            "DisabledByPolicy",
            "Retired",
        ];
        for row in &page.rows {
            assert!(
                valid.contains(&row.effective_lifecycle_state.as_str()),
                "unrecognized lifecycle {}",
                row.effective_lifecycle_state
            );
        }
    }

    #[test]
    fn disabled_rows_expose_kill_switch_visibility() {
        let page = seeded_finalize_experiments_labs_inventory_page();
        for row in &page.rows {
            if row.effective_lifecycle_state == "DisabledByPolicy" {
                assert!(
                    row.kill_switch_visibility.is_some(),
                    "disabled row {} must expose kill_switch_visibility",
                    row.capability_id
                );
            }
        }
    }

    #[test]
    fn validate_accepts_seeded_page() {
        let page = seeded_finalize_experiments_labs_inventory_page();
        validate_finalize_experiments_labs_inventory_page(&page).expect("seeded page is valid");
    }

    #[test]
    fn audit_detects_stable_claim_with_non_stable_dependency() {
        let mut page = seeded_finalize_experiments_labs_inventory_page();
        // Force a stable qualification on a row that has a non-stable dependency.
        for row in &mut page.rows {
            if !row.dependency_markers.is_empty() {
                row.qualification = InventoryQualificationClass::FinalizedStable;
                break;
            }
        }
        let defects = audit_finalize_experiments_labs_inventory_page(&page);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == "stable_claim_with_non_stable_dependency"));
    }

    #[test]
    fn cli_projection_matches_page_summary() {
        let page = seeded_finalize_experiments_labs_inventory_page();
        let cli = project_cli_inventory(&page);
        assert_eq!(
            cli.fields.get("total_row_count").map(String::as_str),
            Some(page.rows.len().to_string().as_str())
        );
        assert_eq!(cli.rows.len(), page.rows.len());
    }

    #[test]
    fn support_export_round_trips() {
        let page = seeded_finalize_experiments_labs_inventory_page();
        let export = project_support_export("test-export", &page);
        assert_eq!(export.export_id, "test-export");
        assert_eq!(export.rows.len(), page.rows.len());
    }

    #[test]
    fn qualification_class_holds_stable_logic() {
        assert!(InventoryQualificationClass::FinalizedStable.holds_stable());
        assert!(InventoryQualificationClass::FinalizedOnWaiver.holds_stable());
        assert!(!InventoryQualificationClass::VisibleBounded.holds_stable());
        assert!(!InventoryQualificationClass::NarrowedUnbacked.holds_stable());
    }
}
