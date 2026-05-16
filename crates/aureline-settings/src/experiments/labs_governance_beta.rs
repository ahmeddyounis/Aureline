//! Beta experiments / flags / Labs governance UI projection.
//!
//! This module is the page-level projection that promotes the
//! settings-owned experiments inventory to a beta governance UI surface.
//! It does NOT mint a parallel inventory, kill-switch precedence, or
//! lifecycle vocabulary — those still live in
//! [`crate::experiments`]. It pins, on every claimed beta governance
//! row, that:
//!
//! 1. **Alignment.** Every row carries an owner, cohort/ring, expiry or
//!    review date, and a kill-switch path summary. The validator rejects
//!    a row that loses any one of those fields. The kill-switch path is
//!    the highest-precedence non-empty source recorded by the inventory
//!    (emergency → admin policy → release-channel → cohort/ring → user
//!    toggle).
//! 2. **Visible markers on stable surfaces.** Every host surface
//!    enumerates whether it claims `claims_stable_posture` truth. A
//!    stable-claiming host that renders a non-stable row MUST carry a
//!    `visible_marker_token` AND a `visible_marker_disclosure` quoting
//!    the lifecycle state so the user is never silently inside an
//!    experiment behind a stable label.
//! 3. **Shared vocabulary.** UI badges, CLI rows, support-export rows,
//!    and reviewer docs all read the same closed
//!    [`CapabilityLifecycleState`] tokens. The validator rejects a row
//!    whose UI badge token disagrees with the CLI / export token or the
//!    inventory's effective lifecycle state.
//!
//! The same projection feeds the live shell governance card, the
//! `aureline_shell_experiments_governance` headless inspector, the
//! support-export wrapper, and the reviewer-facing companion doc under
//! [`docs/governance/m3/experiments_labs_beta.md`]. UI rows, CLI rows,
//! and support-export rows always come from the same inventory record,
//! so the live shell, the diagnostics panel, and the support export
//! cannot drift their lifecycle, owner, cohort, expiry, or kill-switch
//! truth.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::{
    inspect_inventory, load_default_inventory, CapabilityLifecycleState, ExperimentsInventory,
    ExperimentsInventoryError, ExperimentsInventoryInspectionRecord,
    ExperimentsInventoryRowInspection,
};

#[cfg(test)]
use super::EXPERIMENTS_INVENTORY_SCHEMA_VERSION;

/// Beta schema version exported with every record.
pub const LABS_GOVERNANCE_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record.
pub const LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF: &str =
    "settings:experiments_labs_governance_beta:v1";

/// Stable record kind for [`LabsGovernanceBetaPage`] payloads.
pub const LABS_GOVERNANCE_BETA_PAGE_RECORD_KIND: &str =
    "settings_experiments_labs_governance_beta_page_record";

/// Stable record kind for [`LabsGovernanceBetaRow`] payloads.
pub const LABS_GOVERNANCE_BETA_ROW_RECORD_KIND: &str =
    "settings_experiments_labs_governance_beta_row_record";

/// Stable record kind for [`LabsGovernanceBetaBadge`] payloads.
pub const LABS_GOVERNANCE_BETA_BADGE_RECORD_KIND: &str =
    "settings_experiments_labs_governance_beta_badge_record";

/// Stable record kind for [`LabsGovernanceBetaCliProjection`] payloads.
pub const LABS_GOVERNANCE_BETA_CLI_RECORD_KIND: &str =
    "settings_experiments_labs_governance_beta_cli_record";

/// Stable record kind for [`LabsGovernanceBetaSupportExport`] payloads.
pub const LABS_GOVERNANCE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "settings_experiments_labs_governance_beta_support_export_record";

/// Closed host-surface vocabulary the beta projection enumerates.
///
/// Each variant maps to a real shell consumer that already renders the
/// inventory (or claims to). Adding a host surface to this enum is how
/// the projection grows; downstream the validator enforces the visible
/// marker rule for every claimed `claims_stable_posture` surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostSurfaceClass {
    /// Settings root pane. Claims stable posture; any non-stable row
    /// MUST carry a visible marker.
    SettingsRoot,
    /// Settings → Labs tab. Does NOT claim stable posture; the tab itself
    /// is the visible marker for its rows.
    SettingsLabsTab,
    /// Help / About panel. Claims stable posture; any non-stable
    /// dependency carries a visible marker on the row.
    HelpAboutPanel,
    /// Diagnostics panel rendered to operators. Claims stable posture;
    /// the panel surfaces lifecycle and kill-switch state inline.
    DiagnosticsPanel,
    /// Support-export packet attached to a support bundle. Claims stable
    /// posture; export rows carry the lifecycle state inline.
    SupportExportPacket,
    /// Release-center notice. Claims stable posture; release notices
    /// must quote the lifecycle state for any non-stable feature.
    ReleaseCenter,
    /// Command palette surface. Claims stable posture; the palette row
    /// must carry the lifecycle marker for any non-stable command.
    CommandPalette,
}

impl HostSurfaceClass {
    /// Returns the stable snake_case token recorded by the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsRoot => "settings_root",
            Self::SettingsLabsTab => "settings_labs_tab",
            Self::HelpAboutPanel => "help_about_panel",
            Self::DiagnosticsPanel => "diagnostics_panel",
            Self::SupportExportPacket => "support_export_packet",
            Self::ReleaseCenter => "release_center",
            Self::CommandPalette => "command_palette",
        }
    }

    /// True when this surface claims a stable visual posture. A
    /// stable-claiming surface that renders a non-stable row MUST carry
    /// a visible marker on the row.
    pub const fn claims_stable_posture(self) -> bool {
        match self {
            Self::SettingsLabsTab => false,
            Self::SettingsRoot
            | Self::HelpAboutPanel
            | Self::DiagnosticsPanel
            | Self::SupportExportPacket
            | Self::ReleaseCenter
            | Self::CommandPalette => true,
        }
    }
}

/// Closed visible-marker token vocabulary mirrored by every projection
/// row, badge, CLI row, and support-export row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibleMarkerToken {
    /// Labs chip — renders next to the row title on every host surface.
    LabsChip,
    /// Preview chip — renders next to the row title on every host
    /// surface.
    PreviewChip,
    /// Beta chip — renders next to the row title on every host surface.
    BetaChip,
    /// Deprecated chip — renders with a replacement hint.
    DeprecatedChip,
    /// Disabled-by-policy chip — renders with the disable source and
    /// fallback path.
    PolicyDisabledChip,
    /// Retired tombstone — renders without an opt-in affordance.
    RetiredTombstone,
}

impl VisibleMarkerToken {
    /// Returns the stable snake_case token recorded by the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabsChip => "labs_chip",
            Self::PreviewChip => "preview_chip",
            Self::BetaChip => "beta_chip",
            Self::DeprecatedChip => "deprecated_chip",
            Self::PolicyDisabledChip => "policy_disabled_chip",
            Self::RetiredTombstone => "retired_tombstone",
        }
    }

    /// Returns the visible marker token for the given effective lifecycle
    /// state. Returns `None` only when the lifecycle is Stable, since a
    /// stable row needs no marker.
    pub const fn from_state(state: CapabilityLifecycleState) -> Option<Self> {
        match state {
            CapabilityLifecycleState::Labs => Some(Self::LabsChip),
            CapabilityLifecycleState::Preview => Some(Self::PreviewChip),
            CapabilityLifecycleState::Beta => Some(Self::BetaChip),
            CapabilityLifecycleState::Deprecated => Some(Self::DeprecatedChip),
            CapabilityLifecycleState::DisabledByPolicy => Some(Self::PolicyDisabledChip),
            CapabilityLifecycleState::Retired => Some(Self::RetiredTombstone),
            CapabilityLifecycleState::Stable => None,
        }
    }
}

/// One host-surface assignment for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSurfaceAssignment {
    /// Host surface class.
    pub host_surface: HostSurfaceClass,
    /// True when the host renders the row at all on this surface.
    pub renders_row: bool,
    /// Reviewer-facing copy summarising why the row appears here.
    pub disclosure: String,
}

/// Kill-switch path summary projected for one row.
///
/// The beta promise: every row carries the highest-precedence non-empty
/// source recorded by the inventory plus a copy-safe summary of how a
/// stop is triggered. The validator rejects a row whose summary is
/// empty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KillSwitchPathProjection {
    /// True when the inventory recorded at least one disable-source
    /// path (active or inactive). Beta rejects rows that record no
    /// possible disable path.
    pub disable_path_declared: bool,
    /// Source class token of the winning disable source, when one is
    /// active. None when no disable source is currently active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_source_class: Option<String>,
    /// Stable ref of the highest-precedence inventory source, whether
    /// active or not.
    pub highest_precedence_source_ref: String,
    /// True when the highest-precedence source preserves durable
    /// user-authored data.
    pub preserves_user_data: bool,
    /// Bounded fallback or recovery path quoted from the inventory.
    pub fallback_path: String,
    /// Reviewer-facing summary that quotes the precedence ladder this
    /// row resolves through. Must be non-empty.
    pub summary: String,
}

/// One beta governance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabsGovernanceBetaRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable capability id from the inventory.
    pub capability_id: String,
    /// Human-readable capability title.
    pub title: String,
    /// Target workflow / persona for the capability.
    pub target_workflow: String,
    /// Owner ref. Must be non-empty.
    pub owner: String,
    /// Cohort or ring label. Must be non-empty.
    pub cohort_or_ring: String,
    /// Public-facing label rendered on the row.
    pub public_label: String,
    /// Declared lifecycle state token (pre-disable).
    pub declared_lifecycle_state: String,
    /// Effective lifecycle state token (post-disable).
    pub effective_lifecycle_state: String,
    /// Review or expiry date in `YYYY-MM-DD`. Must be non-empty.
    pub review_or_expiry_date: String,
    /// Visible marker token rendered next to the row title. None when
    /// the effective lifecycle is Stable (no marker required).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_marker_token: Option<String>,
    /// Reviewer-facing copy that quotes the lifecycle and any active
    /// disable source. None for Stable rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_marker_disclosure: Option<String>,
    /// Kill-switch path summary.
    pub kill_switch_path: KillSwitchPathProjection,
    /// Host-surface assignments for this row.
    pub host_surfaces: Vec<HostSurfaceAssignment>,
    /// True when at least one saved-artifact dependency marker still
    /// renders for the row.
    pub saved_artifact_dependency_present: bool,
    /// Source inventory record ref this row was projected from.
    pub source_inventory_row_ref: String,
}

/// Badge mirror for one beta governance row.
///
/// Beta requires the badge, row, CLI projection, and support-export
/// row to agree on the closed lifecycle vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabsGovernanceBetaBadge {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable capability id this badge mirrors.
    pub capability_id: String,
    /// Mirrored effective lifecycle state token.
    pub effective_lifecycle_state: String,
    /// Visible marker token rendered on the badge. None for Stable rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_marker_token: Option<String>,
    /// Short reviewer-facing badge label.
    pub badge_label: String,
    /// True when the badge counts toward the "experiments in flight"
    /// attention chip in the diagnostics or settings root surfaces.
    pub counts_toward_attention_chip: bool,
}

/// CLI / headless projection of one beta governance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabsGovernanceBetaCliRow {
    /// Stable capability id from the inventory.
    pub capability_id: String,
    /// Effective lifecycle state token.
    pub effective_lifecycle_state: String,
    /// Owner ref.
    pub owner: String,
    /// Cohort or ring label.
    pub cohort_or_ring: String,
    /// Review or expiry date.
    pub review_or_expiry_date: String,
    /// Visible marker token. None for Stable rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_marker_token: Option<String>,
    /// Winning disable source class, when one is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_disable_source: Option<String>,
    /// Reviewer-facing one-line summary.
    pub kill_switch_summary: String,
    /// Bounded fallback path.
    pub fallback_path: String,
}

/// CLI projection envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabsGovernanceBetaCliProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Source inventory artifact ref.
    pub source_inventory_ref: String,
    /// Deterministic field map.
    pub fields: BTreeMap<String, String>,
    /// CLI rows in stable page order.
    pub rows: Vec<LabsGovernanceBetaCliRow>,
}

/// One support-export row.
///
/// The export row carries the same lifecycle, owner, cohort, expiry,
/// and kill-switch fields as the page row and the CLI row. Beta forbids
/// support-export rows that quote raw private material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabsGovernanceBetaSupportExportRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable capability id.
    pub capability_id: String,
    /// Effective lifecycle state token.
    pub effective_lifecycle_state: String,
    /// Owner ref.
    pub owner: String,
    /// Cohort or ring label.
    pub cohort_or_ring: String,
    /// Review or expiry date.
    pub review_or_expiry_date: String,
    /// Visible marker token. None for Stable rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_marker_token: Option<String>,
    /// Kill-switch path summary copy.
    pub kill_switch_summary: String,
    /// Bounded fallback path.
    pub fallback_path: String,
    /// True when no raw private material crosses the export boundary.
    pub raw_private_material_excluded: bool,
}

/// Top-level beta governance page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabsGovernanceBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Reviewer-facing page label.
    pub page_label: String,
    /// Source inventory artifact ref.
    pub source_inventory_ref: String,
    /// Source inventory id.
    pub source_inventory_id: String,
    /// Inventory as-of date.
    pub as_of: String,
    /// Aggregate lifecycle counts keyed by controlled token.
    pub lifecycle_counts: BTreeMap<String, usize>,
    /// Beta rows in stable order.
    pub rows: Vec<LabsGovernanceBetaRow>,
    /// Badge mirror for each row.
    pub badges: Vec<LabsGovernanceBetaBadge>,
}

/// Support-export wrapper around a beta governance page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabsGovernanceBetaSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Generated-at timestamp.
    pub generated_at: String,
    /// Embedded beta page.
    pub page: LabsGovernanceBetaPage,
    /// Per-row export rows in stable page order.
    pub rows: Vec<LabsGovernanceBetaSupportExportRow>,
    /// True when no raw private material crosses the export boundary.
    pub raw_private_material_excluded: bool,
}

/// Beta governance validation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabsGovernanceBetaValidationError {
    /// Required alignment field was empty on a row.
    AlignmentFieldMissing {
        /// Capability id.
        capability_id: String,
        /// Missing field name.
        field: String,
    },
    /// Lifecycle token disagreed across page, badge, CLI, or export
    /// projections for the same capability.
    LifecycleTokenDrift {
        /// Capability id.
        capability_id: String,
        /// Field that drifted.
        field: String,
    },
    /// Visible marker token was missing on a non-stable row.
    VisibleMarkerMissing {
        /// Capability id.
        capability_id: String,
    },
    /// Visible marker disclosure was empty on a non-stable row.
    VisibleMarkerDisclosureMissing {
        /// Capability id.
        capability_id: String,
    },
    /// A stable-claiming host surface rendered a non-stable row without
    /// a visible marker token. This is the protected promise: a stable
    /// surface cannot silently depend on hidden experiment state.
    StableHostRendersHiddenExperiment {
        /// Capability id.
        capability_id: String,
        /// Stable-claiming host surface token.
        host_surface: String,
    },
    /// Badge mirror was missing for a page row.
    BadgeMissingForRow {
        /// Capability id.
        capability_id: String,
    },
    /// Kill-switch summary was empty on a row.
    KillSwitchSummaryMissing {
        /// Capability id.
        capability_id: String,
    },
    /// A row carried no disable-source path at all.
    KillSwitchPathUndeclared {
        /// Capability id.
        capability_id: String,
    },
    /// A row was disabled by policy but the projection carries no
    /// active source class.
    PolicyDisabledRowMissingActiveSource {
        /// Capability id.
        capability_id: String,
    },
    /// Page did not cover the required lifecycle states.
    LifecycleCoverageIncomplete {
        /// Missing lifecycle state token.
        missing_state: String,
    },
}

impl std::fmt::Display for LabsGovernanceBetaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlignmentFieldMissing { capability_id, field } => write!(
                f,
                "beta governance row {capability_id:?} missing required alignment field {field}"
            ),
            Self::LifecycleTokenDrift { capability_id, field } => write!(
                f,
                "beta governance row {capability_id:?} lifecycle token drift on field {field}"
            ),
            Self::VisibleMarkerMissing { capability_id } => write!(
                f,
                "beta governance row {capability_id:?} is non-stable but carries no visible_marker_token"
            ),
            Self::VisibleMarkerDisclosureMissing { capability_id } => write!(
                f,
                "beta governance row {capability_id:?} is non-stable but carries no visible_marker_disclosure"
            ),
            Self::StableHostRendersHiddenExperiment { capability_id, host_surface } => write!(
                f,
                "stable host surface {host_surface} renders non-stable capability {capability_id:?} without a visible marker"
            ),
            Self::BadgeMissingForRow { capability_id } => write!(
                f,
                "beta governance page row {capability_id:?} has no badge mirror"
            ),
            Self::KillSwitchSummaryMissing { capability_id } => write!(
                f,
                "beta governance row {capability_id:?} missing kill_switch_path.summary"
            ),
            Self::KillSwitchPathUndeclared { capability_id } => write!(
                f,
                "beta governance row {capability_id:?} declared no kill-switch path at all"
            ),
            Self::PolicyDisabledRowMissingActiveSource { capability_id } => write!(
                f,
                "beta governance row {capability_id:?} is policy-disabled but carries no active source class"
            ),
            Self::LifecycleCoverageIncomplete { missing_state } => write!(
                f,
                "beta governance page does not cover lifecycle state {missing_state}"
            ),
        }
    }
}

impl std::error::Error for LabsGovernanceBetaValidationError {}

/// Build the beta governance page from the checked-in inventory.
pub fn build_default_labs_governance_beta_page(
) -> Result<LabsGovernanceBetaPage, ExperimentsInventoryError> {
    let inventory = load_default_inventory()?;
    let inspection = inspect_inventory(&inventory)?;
    Ok(build_labs_governance_beta_page_from_records(
        &inventory,
        &inspection,
    ))
}

/// Build the beta governance page from explicit inventory records.
pub fn build_labs_governance_beta_page_from_records(
    inventory: &ExperimentsInventory,
    inspection: &ExperimentsInventoryInspectionRecord,
) -> LabsGovernanceBetaPage {
    let mut rows = Vec::with_capacity(inspection.rows.len());
    let mut badges = Vec::with_capacity(inspection.rows.len());

    for row in &inspection.rows {
        let projection_row = build_row(inventory, row);
        let badge = build_badge(&projection_row);
        rows.push(projection_row);
        badges.push(badge);
    }

    let lifecycle_counts = inspection.lifecycle_counts.clone();
    LabsGovernanceBetaPage {
        record_kind: LABS_GOVERNANCE_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
        shared_contract_ref: LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF.to_owned(),
        page_id: "settings:experiments_labs_governance_beta:default".to_owned(),
        page_label: "Experiments, flags, and Labs governance (beta)".to_owned(),
        source_inventory_ref: inventory.source_inventory_ref.clone(),
        source_inventory_id: inventory.inventory_id.clone(),
        as_of: inventory.as_of.clone(),
        lifecycle_counts,
        rows,
        badges,
    }
}

fn build_row(
    inventory: &ExperimentsInventory,
    inspection_row: &ExperimentsInventoryRowInspection,
) -> LabsGovernanceBetaRow {
    let state = lifecycle_state_from_token(&inspection_row.effective_lifecycle_state);
    let marker = state.and_then(VisibleMarkerToken::from_state);
    let visible_marker_token = marker.map(|t| t.as_str().to_owned());
    let visible_marker_disclosure = marker.map(|t| disclosure_for_marker(t, inspection_row));

    let kill_switch_path = build_kill_switch(inventory, inspection_row);
    let host_surfaces = host_assignments_for(inspection_row);

    LabsGovernanceBetaRow {
        record_kind: LABS_GOVERNANCE_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
        shared_contract_ref: LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF.to_owned(),
        capability_id: inspection_row.capability_id.clone(),
        title: inspection_row.title.clone(),
        target_workflow: inspection_row.target_workflow.clone(),
        owner: inspection_row.owner.clone(),
        cohort_or_ring: inspection_row.cohort_or_ring.clone(),
        public_label: inspection_row.public_label.clone(),
        declared_lifecycle_state: inspection_row.declared_lifecycle_state.clone(),
        effective_lifecycle_state: inspection_row.effective_lifecycle_state.clone(),
        review_or_expiry_date: inspection_row.review_or_expiry_date.clone(),
        visible_marker_token,
        visible_marker_disclosure,
        kill_switch_path,
        host_surfaces,
        saved_artifact_dependency_present: inspection_row.saved_artifact_dependency_present,
        source_inventory_row_ref: format!(
            "{}#{}",
            inventory.source_inventory_ref, inspection_row.capability_id
        ),
    }
}

fn build_badge(row: &LabsGovernanceBetaRow) -> LabsGovernanceBetaBadge {
    let counts_toward_attention_chip =
        !matches!(row.effective_lifecycle_state.as_str(), "Stable" | "Retired");
    let badge_label = match row.effective_lifecycle_state.as_str() {
        "Labs" => "Labs",
        "Preview" => "Preview",
        "Beta" => "Beta",
        "Stable" => "Stable",
        "Deprecated" => "Deprecated · review",
        "DisabledByPolicy" => "Disabled by policy",
        "Retired" => "Retired",
        _ => "Lifecycle",
    };
    LabsGovernanceBetaBadge {
        record_kind: LABS_GOVERNANCE_BETA_BADGE_RECORD_KIND.to_owned(),
        schema_version: LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
        shared_contract_ref: LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF.to_owned(),
        capability_id: row.capability_id.clone(),
        effective_lifecycle_state: row.effective_lifecycle_state.clone(),
        visible_marker_token: row.visible_marker_token.clone(),
        badge_label: badge_label.to_owned(),
        counts_toward_attention_chip,
    }
}

fn build_kill_switch(
    inventory: &ExperimentsInventory,
    inspection_row: &ExperimentsInventoryRowInspection,
) -> KillSwitchPathProjection {
    let raw_row = inventory
        .rows
        .iter()
        .find(|r| r.capability_id == inspection_row.capability_id);

    let any_path = raw_row
        .map(|r| !r.disable_sources.is_empty())
        .unwrap_or(false);

    let active_source_class = inspection_row
        .winning_disable_source
        .as_ref()
        .map(|s| s.source_class.clone());

    let (highest_ref, preserves_user_data, fallback_path) =
        highest_precedence_source(raw_row, inventory, inspection_row);

    let summary = build_summary(inspection_row);

    KillSwitchPathProjection {
        disable_path_declared: any_path,
        active_source_class,
        highest_precedence_source_ref: highest_ref,
        preserves_user_data,
        fallback_path,
        summary,
    }
}

fn highest_precedence_source(
    raw_row: Option<&super::ExperimentCapabilityRow>,
    inventory: &ExperimentsInventory,
    inspection_row: &ExperimentsInventoryRowInspection,
) -> (String, bool, String) {
    if let Some(active) = inspection_row.winning_disable_source.as_ref() {
        return (
            active.source_ref.clone(),
            active.preserve_user_data,
            active.fallback_path.clone(),
        );
    }
    if let Some(row) = raw_row {
        let mut best: Option<&super::DisableSource> = None;
        for source in &row.disable_sources {
            let idx = inventory
                .kill_switch_precedence
                .iter()
                .position(|class| *class == source.source_class)
                .unwrap_or(usize::MAX);
            let best_idx = best
                .map(|b| {
                    inventory
                        .kill_switch_precedence
                        .iter()
                        .position(|class| *class == b.source_class)
                        .unwrap_or(usize::MAX)
                })
                .unwrap_or(usize::MAX);
            if idx < best_idx {
                best = Some(source);
            }
        }
        if let Some(source) = best {
            return (
                source.source_ref.clone(),
                source.preserve_user_data,
                source.fallback_path.clone(),
            );
        }
    }
    (String::new(), false, String::new())
}

fn build_summary(inspection_row: &ExperimentsInventoryRowInspection) -> String {
    if let Some(source) = inspection_row.winning_disable_source.as_ref() {
        format!(
            "Active stop: {} ({}). Fallback: {}",
            source.source_class, source.source_ref, source.fallback_path
        )
    } else {
        format!(
            "Stops resolve through emergency → admin policy → release-channel → cohort/ring → user toggle for {}.",
            inspection_row.capability_id
        )
    }
}

fn lifecycle_state_from_token(token: &str) -> Option<CapabilityLifecycleState> {
    match token {
        "Labs" => Some(CapabilityLifecycleState::Labs),
        "Preview" => Some(CapabilityLifecycleState::Preview),
        "Beta" => Some(CapabilityLifecycleState::Beta),
        "Stable" => Some(CapabilityLifecycleState::Stable),
        "Deprecated" => Some(CapabilityLifecycleState::Deprecated),
        "DisabledByPolicy" => Some(CapabilityLifecycleState::DisabledByPolicy),
        "Retired" => Some(CapabilityLifecycleState::Retired),
        _ => None,
    }
}

fn disclosure_for_marker(
    marker: VisibleMarkerToken,
    inspection_row: &ExperimentsInventoryRowInspection,
) -> String {
    match marker {
        VisibleMarkerToken::LabsChip => format!(
            "Labs surface. Cohort {} • opt-in until {}.",
            inspection_row.cohort_or_ring, inspection_row.review_or_expiry_date
        ),
        VisibleMarkerToken::PreviewChip => format!(
            "Preview surface for cohort {} • review by {}.",
            inspection_row.cohort_or_ring, inspection_row.review_or_expiry_date
        ),
        VisibleMarkerToken::BetaChip => format!(
            "Beta surface • cohort {} • next review {}.",
            inspection_row.cohort_or_ring, inspection_row.review_or_expiry_date
        ),
        VisibleMarkerToken::DeprecatedChip => format!(
            "Deprecated. Visible only for migration; remove by {}.",
            inspection_row.review_or_expiry_date
        ),
        VisibleMarkerToken::PolicyDisabledChip => {
            let reason = inspection_row
                .winning_disable_source
                .as_ref()
                .map(|s| s.reason.clone())
                .unwrap_or_else(|| "Disabled by policy.".to_owned());
            format!("Disabled by policy. {reason}")
        }
        VisibleMarkerToken::RetiredTombstone => format!(
            "Retired. Visible only for migration disclosure until {}.",
            inspection_row.review_or_expiry_date
        ),
    }
}

fn host_assignments_for(
    inspection_row: &ExperimentsInventoryRowInspection,
) -> Vec<HostSurfaceAssignment> {
    let is_stable = inspection_row.effective_lifecycle_state == "Stable";
    let is_retired = inspection_row.effective_lifecycle_state == "Retired";
    let mut out = Vec::new();

    out.push(HostSurfaceAssignment {
        host_surface: HostSurfaceClass::SettingsRoot,
        renders_row: !is_retired,
        disclosure: if is_stable {
            "Stable row · no visible marker required.".to_owned()
        } else {
            "Non-stable row · the settings root renders the lifecycle chip inline so the surface never claims stable behind a hidden experiment.".to_owned()
        },
    });

    out.push(HostSurfaceAssignment {
        host_surface: HostSurfaceClass::SettingsLabsTab,
        renders_row: matches!(
            inspection_row.effective_lifecycle_state.as_str(),
            "Labs" | "Preview" | "Beta"
        ),
        disclosure: "Labs tab opt-in surface; the tab itself is the visible marker.".to_owned(),
    });

    out.push(HostSurfaceAssignment {
        host_surface: HostSurfaceClass::HelpAboutPanel,
        renders_row: !is_retired,
        disclosure:
            "Help / About cards quote the lifecycle chip for any non-stable capability used by the build."
                .to_owned(),
    });

    out.push(HostSurfaceAssignment {
        host_surface: HostSurfaceClass::DiagnosticsPanel,
        renders_row: true,
        disclosure: "Diagnostics panel renders every inventory row with its lifecycle and kill-switch context.".to_owned(),
    });

    out.push(HostSurfaceAssignment {
        host_surface: HostSurfaceClass::SupportExportPacket,
        renders_row: true,
        disclosure:
            "Support-export packet quotes the same lifecycle and kill-switch fields as the UI row."
                .to_owned(),
    });

    out
}

/// Validates one beta governance page against the alignment, visible
/// marker, and shared vocabulary promises.
pub fn validate_labs_governance_beta_page(
    page: &LabsGovernanceBetaPage,
) -> Result<(), Vec<LabsGovernanceBetaValidationError>> {
    let mut errors = Vec::new();
    let mut seen_ids = BTreeSet::new();

    for row in &page.rows {
        seen_ids.insert(row.capability_id.clone());

        check_field(&row.capability_id, "owner", &row.owner, &mut errors);
        check_field(
            &row.capability_id,
            "cohort_or_ring",
            &row.cohort_or_ring,
            &mut errors,
        );
        check_field(
            &row.capability_id,
            "review_or_expiry_date",
            &row.review_or_expiry_date,
            &mut errors,
        );
        check_field(
            &row.capability_id,
            "public_label",
            &row.public_label,
            &mut errors,
        );

        if !row.kill_switch_path.disable_path_declared {
            errors.push(
                LabsGovernanceBetaValidationError::KillSwitchPathUndeclared {
                    capability_id: row.capability_id.clone(),
                },
            );
        }
        if row.kill_switch_path.summary.trim().is_empty() {
            errors.push(
                LabsGovernanceBetaValidationError::KillSwitchSummaryMissing {
                    capability_id: row.capability_id.clone(),
                },
            );
        }
        if row.effective_lifecycle_state == "DisabledByPolicy"
            && row.kill_switch_path.active_source_class.is_none()
        {
            errors.push(
                LabsGovernanceBetaValidationError::PolicyDisabledRowMissingActiveSource {
                    capability_id: row.capability_id.clone(),
                },
            );
        }

        let is_stable = row.effective_lifecycle_state == "Stable";
        if !is_stable {
            if row.visible_marker_token.is_none() {
                errors.push(LabsGovernanceBetaValidationError::VisibleMarkerMissing {
                    capability_id: row.capability_id.clone(),
                });
            }
            if row
                .visible_marker_disclosure
                .as_deref()
                .map_or(true, |s| s.trim().is_empty())
            {
                errors.push(
                    LabsGovernanceBetaValidationError::VisibleMarkerDisclosureMissing {
                        capability_id: row.capability_id.clone(),
                    },
                );
            }
            for assignment in &row.host_surfaces {
                if assignment.renders_row
                    && assignment.host_surface.claims_stable_posture()
                    && row.visible_marker_token.is_none()
                {
                    errors.push(
                        LabsGovernanceBetaValidationError::StableHostRendersHiddenExperiment {
                            capability_id: row.capability_id.clone(),
                            host_surface: assignment.host_surface.as_str().to_owned(),
                        },
                    );
                }
            }
        }

        let badge = page
            .badges
            .iter()
            .find(|b| b.capability_id == row.capability_id);
        match badge {
            None => errors.push(LabsGovernanceBetaValidationError::BadgeMissingForRow {
                capability_id: row.capability_id.clone(),
            }),
            Some(badge) => {
                if badge.effective_lifecycle_state != row.effective_lifecycle_state {
                    errors.push(LabsGovernanceBetaValidationError::LifecycleTokenDrift {
                        capability_id: row.capability_id.clone(),
                        field: "badge.effective_lifecycle_state".to_owned(),
                    });
                }
                if badge.visible_marker_token != row.visible_marker_token {
                    errors.push(LabsGovernanceBetaValidationError::LifecycleTokenDrift {
                        capability_id: row.capability_id.clone(),
                        field: "badge.visible_marker_token".to_owned(),
                    });
                }
            }
        }
    }

    for state in [
        "Labs",
        "Preview",
        "Beta",
        "Stable",
        "Deprecated",
        "DisabledByPolicy",
        "Retired",
    ] {
        if !page.lifecycle_counts.contains_key(state) {
            errors.push(
                LabsGovernanceBetaValidationError::LifecycleCoverageIncomplete {
                    missing_state: state.to_owned(),
                },
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Projects the beta governance page to the CLI / headless inspection
/// surface.
pub fn project_labs_governance_beta_cli(
    page: &LabsGovernanceBetaPage,
) -> LabsGovernanceBetaCliProjection {
    let mut fields = BTreeMap::new();
    fields.insert("page_id".to_owned(), page.page_id.clone());
    fields.insert("as_of".to_owned(), page.as_of.clone());
    fields.insert(
        "source_inventory_id".to_owned(),
        page.source_inventory_id.clone(),
    );
    fields.insert("row_count".to_owned(), page.rows.len().to_string());
    let non_stable_count = page
        .rows
        .iter()
        .filter(|r| r.effective_lifecycle_state != "Stable")
        .count();
    fields.insert(
        "non_stable_row_count".to_owned(),
        non_stable_count.to_string(),
    );

    let rows = page
        .rows
        .iter()
        .map(|row| LabsGovernanceBetaCliRow {
            capability_id: row.capability_id.clone(),
            effective_lifecycle_state: row.effective_lifecycle_state.clone(),
            owner: row.owner.clone(),
            cohort_or_ring: row.cohort_or_ring.clone(),
            review_or_expiry_date: row.review_or_expiry_date.clone(),
            visible_marker_token: row.visible_marker_token.clone(),
            winning_disable_source: row.kill_switch_path.active_source_class.clone(),
            kill_switch_summary: row.kill_switch_path.summary.clone(),
            fallback_path: row.kill_switch_path.fallback_path.clone(),
        })
        .collect();

    LabsGovernanceBetaCliProjection {
        record_kind: LABS_GOVERNANCE_BETA_CLI_RECORD_KIND.to_owned(),
        schema_version: LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
        shared_contract_ref: LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_inventory_ref: page.source_inventory_ref.clone(),
        fields,
        rows,
    }
}

/// Projects the beta governance page to a support-export wrapper.
pub fn project_labs_governance_beta_support_export(
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
    page: LabsGovernanceBetaPage,
) -> LabsGovernanceBetaSupportExport {
    let rows = page
        .rows
        .iter()
        .map(|row| LabsGovernanceBetaSupportExportRow {
            record_kind: "settings_experiments_labs_governance_beta_support_export_row_record"
                .to_owned(),
            schema_version: LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
            shared_contract_ref: LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF.to_owned(),
            capability_id: row.capability_id.clone(),
            effective_lifecycle_state: row.effective_lifecycle_state.clone(),
            owner: row.owner.clone(),
            cohort_or_ring: row.cohort_or_ring.clone(),
            review_or_expiry_date: row.review_or_expiry_date.clone(),
            visible_marker_token: row.visible_marker_token.clone(),
            kill_switch_summary: row.kill_switch_path.summary.clone(),
            fallback_path: row.kill_switch_path.fallback_path.clone(),
            raw_private_material_excluded: true,
        })
        .collect();

    LabsGovernanceBetaSupportExport {
        record_kind: LABS_GOVERNANCE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
        shared_contract_ref: LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF.to_owned(),
        export_id: export_id.into(),
        generated_at: generated_at.into(),
        page,
        rows,
        raw_private_material_excluded: true,
    }
}

/// Validates a support-export wrapper for parity with its embedded
/// page.
pub fn validate_labs_governance_beta_support_export(
    export: &LabsGovernanceBetaSupportExport,
) -> Result<(), Vec<LabsGovernanceBetaValidationError>> {
    let mut errors = Vec::new();
    for row in &export.page.rows {
        let export_row = export
            .rows
            .iter()
            .find(|r| r.capability_id == row.capability_id);
        match export_row {
            None => errors.push(LabsGovernanceBetaValidationError::BadgeMissingForRow {
                capability_id: row.capability_id.clone(),
            }),
            Some(export_row) => {
                if export_row.effective_lifecycle_state != row.effective_lifecycle_state {
                    errors.push(LabsGovernanceBetaValidationError::LifecycleTokenDrift {
                        capability_id: row.capability_id.clone(),
                        field: "support_export.effective_lifecycle_state".to_owned(),
                    });
                }
                if export_row.visible_marker_token != row.visible_marker_token {
                    errors.push(LabsGovernanceBetaValidationError::LifecycleTokenDrift {
                        capability_id: row.capability_id.clone(),
                        field: "support_export.visible_marker_token".to_owned(),
                    });
                }
                if export_row.owner != row.owner
                    || export_row.cohort_or_ring != row.cohort_or_ring
                    || export_row.review_or_expiry_date != row.review_or_expiry_date
                {
                    errors.push(LabsGovernanceBetaValidationError::LifecycleTokenDrift {
                        capability_id: row.capability_id.clone(),
                        field: "support_export.alignment".to_owned(),
                    });
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn check_field(
    capability_id: &str,
    field_name: &str,
    value: &str,
    errors: &mut Vec<LabsGovernanceBetaValidationError>,
) {
    if value.trim().is_empty() {
        errors.push(LabsGovernanceBetaValidationError::AlignmentFieldMissing {
            capability_id: capability_id.to_owned(),
            field: field_name.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_page_passes_validation_and_covers_lifecycle_states() {
        let page = build_default_labs_governance_beta_page().expect("page builds");
        validate_labs_governance_beta_page(&page).expect("page validates");
        assert_eq!(page.schema_version, LABS_GOVERNANCE_BETA_SCHEMA_VERSION);
        assert_eq!(
            page.shared_contract_ref,
            LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF
        );
        for state in [
            "Labs",
            "Preview",
            "Beta",
            "Stable",
            "Deprecated",
            "DisabledByPolicy",
            "Retired",
        ] {
            assert!(
                page.lifecycle_counts.contains_key(state),
                "missing lifecycle state {state}"
            );
        }
    }

    #[test]
    fn non_stable_rows_carry_visible_marker_and_disclosure() {
        let page = build_default_labs_governance_beta_page().expect("page builds");
        for row in &page.rows {
            if row.effective_lifecycle_state == "Stable" {
                assert!(row.visible_marker_token.is_none());
            } else {
                assert!(
                    row.visible_marker_token.is_some(),
                    "{} missing marker",
                    row.capability_id
                );
                assert!(
                    row.visible_marker_disclosure
                        .as_deref()
                        .map(|s| !s.trim().is_empty())
                        .unwrap_or(false),
                    "{} empty disclosure",
                    row.capability_id
                );
            }
        }
    }

    #[test]
    fn validator_flags_stable_host_with_hidden_marker() {
        let mut page = build_default_labs_governance_beta_page().expect("page builds");
        let labs_row = page
            .rows
            .iter_mut()
            .find(|r| r.effective_lifecycle_state == "Labs")
            .expect("labs row");
        labs_row.visible_marker_token = None;
        let badge = page
            .badges
            .iter_mut()
            .find(|b| b.capability_id == labs_row.capability_id)
            .expect("badge for labs row");
        badge.visible_marker_token = None;

        let errors =
            validate_labs_governance_beta_page(&page).expect_err("must flag hidden experiment");
        assert!(errors.iter().any(|e| matches!(
            e,
            LabsGovernanceBetaValidationError::VisibleMarkerMissing { .. }
        )));
        assert!(errors.iter().any(|e| matches!(
            e,
            LabsGovernanceBetaValidationError::StableHostRendersHiddenExperiment { .. }
        )));
    }

    #[test]
    fn validator_flags_missing_owner_or_cohort() {
        let mut page = build_default_labs_governance_beta_page().expect("page builds");
        page.rows[0].owner = String::new();
        page.rows[1].cohort_or_ring = String::new();

        let errors = validate_labs_governance_beta_page(&page).expect_err("must flag missing");
        assert!(errors.iter().any(|e| matches!(
            e,
            LabsGovernanceBetaValidationError::AlignmentFieldMissing { field, .. }
                if field == "owner"
        )));
        assert!(errors.iter().any(|e| matches!(
            e,
            LabsGovernanceBetaValidationError::AlignmentFieldMissing { field, .. }
                if field == "cohort_or_ring"
        )));
    }

    #[test]
    fn cli_and_support_export_share_lifecycle_vocabulary() {
        let page = build_default_labs_governance_beta_page().expect("page builds");
        let cli = project_labs_governance_beta_cli(&page);
        let export = project_labs_governance_beta_support_export(
            "support-export:experiments-labs-governance:test",
            "2026-05-15T00:00:00Z",
            page.clone(),
        );
        validate_labs_governance_beta_support_export(&export).expect("export validates");

        for cli_row in &cli.rows {
            let page_row = page
                .rows
                .iter()
                .find(|r| r.capability_id == cli_row.capability_id)
                .expect("page row exists for cli row");
            assert_eq!(
                cli_row.effective_lifecycle_state,
                page_row.effective_lifecycle_state
            );
            assert_eq!(cli_row.visible_marker_token, page_row.visible_marker_token);
            let export_row = export
                .rows
                .iter()
                .find(|r| r.capability_id == cli_row.capability_id)
                .expect("export row exists");
            assert_eq!(
                export_row.effective_lifecycle_state,
                cli_row.effective_lifecycle_state
            );
            assert_eq!(
                export_row.visible_marker_token,
                cli_row.visible_marker_token
            );
        }
    }

    #[test]
    fn schema_version_matches_inventory_schema_version() {
        assert_eq!(
            LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
            EXPERIMENTS_INVENTORY_SCHEMA_VERSION
        );
    }
}
