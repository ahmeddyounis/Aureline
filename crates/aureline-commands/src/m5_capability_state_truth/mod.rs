//! M5 capability-state truth and lifecycle-dependency projection packet.
//!
//! The command-governance packet proves preview, approval, disabled-reason,
//! result, and route parity for the M5 command families. This companion packet
//! answers the next rollout question: when those capabilities surface in
//! settings, Help/About, docs packs, workflow bundles, profile exports, support
//! packets, and browser- or extension-adjacent metadata, does the non-stable or
//! narrowed lifecycle truth remain visible instead of being flattened into
//! stable-looking copy?
//!
//! The packet is seeded from [`crate::m5_command_governance`] so it does not
//! invent a second command graph. It projects the same command-owned lifecycle,
//! rollout, and route refs into lifecycle-state rows plus per-surface projection
//! rows that enforce dependency-marker disclosure, support narrowing, and
//! freshness-driven retest posture.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_command_governance::{
    seeded_m5_command_governance_packet, M5CommandGovernancePacket, M5CommandGovernanceRow,
    M5GovernanceSurfaceClass,
};
use crate::m5_rollout_inventory::{M5RolloutStateClass, M5_ROLLOUT_INVENTORY_PACKET_REF};
use crate::registry::seeded_registry;

#[cfg(test)]
mod tests;

/// Stable record-kind tag carried by [`M5CapabilityStateTruthPacket`].
pub const M5_CAPABILITY_STATE_TRUTH_RECORD_KIND: &str = "m5_capability_state_truth_packet";

/// Schema version for M5 capability-state truth packets.
pub const M5_CAPABILITY_STATE_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the M5 capability-state truth schema.
pub const M5_CAPABILITY_STATE_TRUTH_SCHEMA_REF: &str =
    "schemas/commands/m5_capability_state_truth.schema.json";

/// Repo-relative path of the companion doc.
pub const M5_CAPABILITY_STATE_TRUTH_DOC_REF: &str = "docs/commands/m5_capability_state_truth.md";

/// Repo-relative path of the checked fixture directory.
pub const M5_CAPABILITY_STATE_TRUTH_FIXTURE_DIR: &str =
    "fixtures/commands/m5_capability_state_truth";

/// Repo-relative path of the checked support export.
pub const M5_CAPABILITY_STATE_TRUTH_SUPPORT_EXPORT_REF: &str =
    "artifacts/commands/m5_capability_state_truth/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CAPABILITY_STATE_TRUTH_SUMMARY_REF: &str =
    "artifacts/commands/m5_capability_state_truth/summary.md";

/// Stable packet id used by the seeded export.
pub const M5_CAPABILITY_STATE_TRUTH_PACKET_ID: &str = "m5-capability-state-truth:stable:0001";

/// Stable support-export id used by [`M5CapabilityStateTruthSupportExport`].
pub const M5_CAPABILITY_STATE_TRUTH_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-capability-state-truth:0001";

const GENERATED_AT: &str = "2026-06-12T00:00:00Z";
const SOURCE_PACKET_REF: &str = "artifacts/commands/m5_command_governance/support_export.json";
const SOURCE_ROLLOUT_INVENTORY_REF: &str = M5_ROLLOUT_INVENTORY_PACKET_REF;
const SOURCE_LIFECYCLE_VOCAB_REF: &str =
    "docs/adr/0011-capability-lifecycle-and-dependency-markers.md";
const SOURCE_PROJECTION_MATRIX_REF: &str =
    "docs/governance/capability_lifecycle_projection_matrix.md";

/// Canonical capability-state class exported by M5 lifecycle projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityStateClass {
    /// Off-by-default exploratory state.
    Labs,
    /// Visible but still unstable.
    Preview,
    /// Supported but not yet general-availability stable.
    Beta,
    /// General-availability stable.
    Stable,
    /// Still present but on a visible sunset path.
    Deprecated,
    /// Present but blocked by policy or kill switch.
    DisabledByPolicy,
    /// Evidence or certification freshness has lapsed and claims must narrow.
    RetestPending,
    /// Removed and preserved only as a tombstone/reference.
    Removed,
}

impl M5CapabilityStateClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Labs => "labs",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::RetestPending => "retest_pending",
            Self::Removed => "removed",
        }
    }

    /// Human-facing label shared by badge-bearing surfaces.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Labs => "Labs",
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::Stable => "Stable",
            Self::Deprecated => "Deprecated",
            Self::DisabledByPolicy => "DisabledByPolicy",
            Self::RetestPending => "RetestPending",
            Self::Removed => "Removed",
        }
    }

    /// Required coverage for packet-level state definitions.
    pub const fn required_coverage() -> [Self; 8] {
        [
            Self::Labs,
            Self::Preview,
            Self::Beta,
            Self::Stable,
            Self::Deprecated,
            Self::DisabledByPolicy,
            Self::RetestPending,
            Self::Removed,
        ]
    }

    /// Whether stable-facing wording may publish with this state.
    pub const fn allows_stable_wording(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Whether support badges may remain fully green.
    pub const fn allows_full_support_badge(self) -> bool {
        !matches!(
            self,
            Self::Labs | Self::DisabledByPolicy | Self::RetestPending | Self::Removed
        )
    }
}

/// Lifecycle-dependency marker class exported to stable- and saved-artifact surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleDependencyMarkerClass {
    /// The consuming surface still depends on a Labs capability.
    LabsDependency,
    /// The consuming surface still depends on a Preview capability.
    PreviewDependency,
    /// The consuming surface still depends on a Beta capability.
    BetaDependency,
    /// A policy-disabled capability or kill switch narrows the consumer.
    PolicyDisabledDependency,
    /// Retest-pending evidence narrows the consumer.
    RetestPendingDependency,
    /// The dependency is deprecated and should remain visibly sunsetted.
    DeprecatedDependency,
    /// The dependency has been removed and must remain a tombstone.
    RemovedDependency,
    /// Evidence freshness has lapsed and stable/support claims must narrow.
    StaleEvidenceDependency,
}

impl M5LifecycleDependencyMarkerClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabsDependency => "labs_dependency",
            Self::PreviewDependency => "preview_dependency",
            Self::BetaDependency => "beta_dependency",
            Self::PolicyDisabledDependency => "policy_disabled_dependency",
            Self::RetestPendingDependency => "retest_pending_dependency",
            Self::DeprecatedDependency => "deprecated_dependency",
            Self::RemovedDependency => "removed_dependency",
            Self::StaleEvidenceDependency => "stale_evidence_dependency",
        }
    }
}

/// Projection surface that consumes lifecycle-state truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityProjectionSurfaceClass {
    /// Settings or effective-configuration row.
    SettingsRow,
    /// Help or About surface.
    HelpAbout,
    /// Docs-pack or command-reference surface.
    DocsPack,
    /// Release-facing claim row.
    ReleaseRow,
    /// Workflow-bundle manifest or review row.
    WorkflowBundle,
    /// Profile export or portable-state projection.
    ProfileExport,
    /// Support packet or diagnostics export.
    SupportPacket,
    /// Desktop inspector or in-product details surface.
    DesktopInspector,
    /// CLI or headless inspect output.
    CliInspect,
    /// Extension metadata surface.
    ExtensionMetadata,
    /// Browser or companion details surface.
    BrowserCompanion,
}

impl M5CapabilityProjectionSurfaceClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsRow => "settings_row",
            Self::HelpAbout => "help_about",
            Self::DocsPack => "docs_pack",
            Self::ReleaseRow => "release_row",
            Self::WorkflowBundle => "workflow_bundle",
            Self::ProfileExport => "profile_export",
            Self::SupportPacket => "support_packet",
            Self::DesktopInspector => "desktop_inspector",
            Self::CliInspect => "cli_inspect",
            Self::ExtensionMetadata => "extension_metadata",
            Self::BrowserCompanion => "browser_companion",
        }
    }

    /// Required projection-surface coverage per capability row.
    pub const fn required_coverage() -> [Self; 11] {
        [
            Self::SettingsRow,
            Self::HelpAbout,
            Self::DocsPack,
            Self::ReleaseRow,
            Self::WorkflowBundle,
            Self::ProfileExport,
            Self::SupportPacket,
            Self::DesktopInspector,
            Self::CliInspect,
            Self::ExtensionMetadata,
            Self::BrowserCompanion,
        ]
    }

    fn claim_surface(self) -> bool {
        matches!(
            self,
            Self::SettingsRow
                | Self::HelpAbout
                | Self::DocsPack
                | Self::ReleaseRow
                | Self::WorkflowBundle
                | Self::ProfileExport
                | Self::SupportPacket
        )
    }

    fn stable_facing(self) -> bool {
        matches!(
            self,
            Self::HelpAbout
                | Self::DocsPack
                | Self::ReleaseRow
                | Self::WorkflowBundle
                | Self::ProfileExport
        )
    }

    fn saved_artifact(self) -> bool {
        matches!(
            self,
            Self::WorkflowBundle | Self::ProfileExport | Self::SupportPacket
        )
    }

    fn inspection_surface(self) -> bool {
        matches!(
            self,
            Self::DesktopInspector
                | Self::CliInspect
                | Self::ExtensionMetadata
                | Self::BrowserCompanion
        )
    }
}

/// Canonical state-definition row exported with the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityStateDefinitionRow {
    /// State class this definition covers.
    pub state_class: M5CapabilityStateClass,
    /// Human-facing disclosure label.
    pub display_label: String,
    /// Detail ref that help/docs/inspect surfaces resolve through.
    pub detail_ref: String,
    /// Whether stable-facing wording may publish for this state.
    pub stable_wording_allowed: bool,
    /// Whether full-support badges may remain green for this state.
    pub full_support_badge_allowed: bool,
    /// Whether saved artifacts must carry dependency markers for this state.
    pub dependency_markers_required_on_saved_artifacts: bool,
    /// Whether the state is a tombstone rather than a runnable surface.
    pub tombstone_state: bool,
}

/// Lifecycle-dependency marker that must survive across claim and export surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleDependencyMarkerRecord {
    /// Opaque marker ref.
    pub marker_ref: String,
    /// Marker class.
    pub marker_class: M5LifecycleDependencyMarkerClass,
    /// Dependent capability or proof ref.
    pub dependency_ref: String,
    /// Disclosure label shown to users and support.
    pub disclosure_label: String,
    /// Stable detail ref for inspection.
    pub detail_ref: String,
    /// Effective state the marker narrows the projection to.
    pub narrows_to_state_class: M5CapabilityStateClass,
    /// Surfaces that must preserve this marker.
    pub affects_surfaces: Vec<M5CapabilityProjectionSurfaceClass>,
}

/// One lifecycle-state projection row for a consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityProjectionRow {
    /// Surface this projection covers.
    pub surface_class: M5CapabilityProjectionSurfaceClass,
    /// Opaque projection ref consumed by docs/help/export tooling.
    pub projection_ref: String,
    /// Declared state class projected for details.
    pub declared_state_class: M5CapabilityStateClass,
    /// Effective state class surfaces must show after narrowing.
    pub effective_state_class: M5CapabilityStateClass,
    /// Whether the projection still claims stable-looking wording.
    pub stable_wording_visible: bool,
    /// Whether the projection still implies ordinary support.
    pub support_wording_visible: bool,
    /// Marker refs disclosed on the projection.
    pub dependency_marker_refs: Vec<String>,
    /// Whether dependency-marker detail remains inspectable on the projection.
    pub dependency_markers_visible: bool,
    /// Badge labels rendered by the projection.
    pub badge_labels: Vec<String>,
    /// State/detail ref exposed by the projection.
    pub inspect_detail_ref: String,
    /// Exact route, handoff, or metadata ref used by inspection surfaces.
    pub route_or_metadata_ref: String,
    /// Evidence freshness shown on the projection.
    pub evidence_freshness_class: String,
    /// Whether the projection is export-safe.
    pub export_safe: bool,
}

/// One capability-state row for an M5 command family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityStateTruthRow {
    /// Stable capability row id.
    pub capability_row_id: String,
    /// Canonical command id anchoring the family.
    pub command_id: String,
    /// Display label shared across settings/help/docs/export surfaces.
    pub display_label: String,
    /// Command-family canonical verb.
    pub canonical_verb: String,
    /// Declared command-owned state class.
    pub declared_state_class: M5CapabilityStateClass,
    /// Effective state class after dependency or freshness narrowing.
    pub effective_state_class: M5CapabilityStateClass,
    /// Support class preserved from the command descriptor.
    pub support_class: String,
    /// Release channel preserved from the command descriptor.
    pub release_channel: String,
    /// Freshness class preserved from the command descriptor.
    pub freshness_class: String,
    /// Origin class for the capability.
    pub origin_class: String,
    /// Current owner ref for rollout/retest review.
    pub owner_ref: String,
    /// Command lifecycle detail ref.
    pub lifecycle_ref: String,
    /// Command rollout-state ref.
    pub rollout_state_ref: String,
    /// Marker rows that must survive across consumers.
    pub dependency_markers: Vec<M5LifecycleDependencyMarkerRecord>,
    /// Surface projections backed by this row.
    pub projection_rows: Vec<M5CapabilityProjectionRow>,
    /// Machine-readable findings. Empty means conforming.
    pub finding_codes: Vec<String>,
}

/// Packet summary for release/help/support consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityStateTruthSummary {
    /// Number of capability rows under audit.
    pub capability_count: usize,
    /// Number of packet-level state definitions.
    pub state_definition_count: usize,
    /// Number of dependency markers carried across rows.
    pub dependency_marker_count: usize,
    /// Number of per-surface projection rows.
    pub projection_row_count: usize,
    /// Number of claim-bearing projections.
    pub claim_surface_projection_count: usize,
    /// Number of saved-artifact projections.
    pub saved_artifact_projection_count: usize,
    /// Number of rows narrowed to Labs.
    pub labs_row_count: usize,
    /// Number of rows narrowed to Preview.
    pub preview_row_count: usize,
    /// Number of rows staying at Beta.
    pub beta_row_count: usize,
    /// Number of rows staying at Stable.
    pub stable_row_count: usize,
    /// Number of rows narrowed to Deprecated.
    pub deprecated_row_count: usize,
    /// Number of rows narrowed to DisabledByPolicy.
    pub disabled_by_policy_row_count: usize,
    /// Number of rows narrowed to RetestPending.
    pub retest_pending_row_count: usize,
    /// Number of rows narrowed to Removed.
    pub removed_row_count: usize,
    /// Number of projections still allowed to render stable wording.
    pub stable_wording_projection_count: usize,
    /// Number of total findings.
    pub finding_count: usize,
}

/// Canonical M5 capability-state truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityStateTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Schema ref for this packet.
    pub schema_ref: String,
    /// Companion doc ref.
    pub doc_ref: String,
    /// Upstream command-governance source ref.
    pub source_packet_ref: String,
    /// Upstream rollout inventory source ref.
    pub source_rollout_inventory_ref: String,
    /// Lifecycle vocabulary source ref.
    pub lifecycle_vocabulary_ref: String,
    /// Projection-matrix source ref.
    pub projection_matrix_ref: String,
    /// Packet-level capability-state definitions.
    pub state_definitions: Vec<M5CapabilityStateDefinitionRow>,
    /// Ordered capability rows.
    pub rows: Vec<M5CapabilityStateTruthRow>,
    /// Roll-up counts.
    pub summary: M5CapabilityStateTruthSummary,
}

impl M5CapabilityStateTruthPacket {
    /// Renders a compact Markdown summary for checked artifacts.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Capability State Truth\n\n");
        out.push_str("| Metric | Value |\n|---|---:|\n");
        out.push_str(&format!(
            "| Capability rows | {} |\n",
            self.summary.capability_count
        ));
        out.push_str(&format!(
            "| State definitions | {} |\n",
            self.summary.state_definition_count
        ));
        out.push_str(&format!(
            "| Dependency markers | {} |\n",
            self.summary.dependency_marker_count
        ));
        out.push_str(&format!(
            "| Projection rows | {} |\n",
            self.summary.projection_row_count
        ));
        out.push_str(&format!(
            "| Claim surfaces | {} |\n",
            self.summary.claim_surface_projection_count
        ));
        out.push_str(&format!(
            "| Saved-artifact projections | {} |\n",
            self.summary.saved_artifact_projection_count
        ));
        out.push_str(&format!(
            "| Labs rows | {} |\n",
            self.summary.labs_row_count
        ));
        out.push_str(&format!(
            "| Preview rows | {} |\n",
            self.summary.preview_row_count
        ));
        out.push_str(&format!(
            "| Beta rows | {} |\n",
            self.summary.beta_row_count
        ));
        out.push_str(&format!(
            "| Stable rows | {} |\n",
            self.summary.stable_row_count
        ));
        out.push_str(&format!(
            "| Deprecated rows | {} |\n",
            self.summary.deprecated_row_count
        ));
        out.push_str(&format!(
            "| DisabledByPolicy rows | {} |\n",
            self.summary.disabled_by_policy_row_count
        ));
        out.push_str(&format!(
            "| RetestPending rows | {} |\n",
            self.summary.retest_pending_row_count
        ));
        out.push_str(&format!(
            "| Removed rows | {} |\n",
            self.summary.removed_row_count
        ));
        out.push_str(&format!(
            "| Stable wording projections | {} |\n",
            self.summary.stable_wording_projection_count
        ));
        out.push_str(&format!(
            "| Findings | {} |\n\n",
            self.summary.finding_count
        ));

        out.push_str(
            "| Command | Effective state | Markers | Claim surfaces with markers | Saved artifacts | Inspection surfaces |\n",
        );
        out.push_str("|---|---|---|---:|---:|---:|\n");
        for row in &self.rows {
            let claim_count = row
                .projection_rows
                .iter()
                .filter(|projection| {
                    projection.surface_class.claim_surface()
                        && !projection.dependency_marker_refs.is_empty()
                })
                .count();
            let saved_count = row
                .projection_rows
                .iter()
                .filter(|projection| projection.surface_class.saved_artifact())
                .count();
            let inspection_count = row
                .projection_rows
                .iter()
                .filter(|projection| projection.surface_class.inspection_surface())
                .count();
            let markers = if row.dependency_markers.is_empty() {
                "none".to_string()
            } else {
                row.dependency_markers
                    .iter()
                    .map(|marker| marker.marker_class.as_str().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | {} | {} |\n",
                row.command_id,
                row.effective_state_class.display_label(),
                markers,
                claim_count,
                saved_count,
                inspection_count
            ));
        }
        out.push('\n');
        out
    }
}

/// Support-export wrapper for the M5 capability-state truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityStateTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet schema ref.
    pub schema_ref: String,
    /// Case ids useful for support joins.
    pub case_ids: Vec<String>,
    /// Quoted truth packet.
    pub packet: M5CapabilityStateTruthPacket,
}

impl M5CapabilityStateTruthSupportExport {
    /// Builds a deterministic support-export wrapper from a packet.
    pub fn from_packet(support_export_id: String, packet: M5CapabilityStateTruthPacket) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.source_rollout_inventory_ref.clone(),
        ];
        for definition in &packet.state_definitions {
            case_ids.push(definition.detail_ref.clone());
        }
        for row in &packet.rows {
            case_ids.push(row.capability_row_id.clone());
            case_ids.push(row.command_id.clone());
            case_ids.push(row.lifecycle_ref.clone());
            case_ids.push(row.rollout_state_ref.clone());
            case_ids.push(row.owner_ref.clone());
            for marker in &row.dependency_markers {
                case_ids.push(marker.marker_ref.clone());
                case_ids.push(marker.dependency_ref.clone());
                case_ids.push(marker.detail_ref.clone());
            }
            for projection in &row.projection_rows {
                case_ids.push(projection.projection_ref.clone());
                case_ids.push(projection.inspect_detail_ref.clone());
                case_ids.push(projection.route_or_metadata_ref.clone());
            }
        }
        case_ids.sort();
        case_ids.dedup();
        Self {
            record_kind: "m5_capability_state_truth_support_export".to_string(),
            schema_version: 1,
            support_export_id,
            schema_ref: M5_CAPABILITY_STATE_TRUTH_SCHEMA_REF.to_string(),
            case_ids,
            packet,
        }
    }
}

/// Validation error raised by [`validate_m5_capability_state_truth_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CapabilityStateTruthValidationError {
    /// The packet has no rows.
    NoRows,
    /// A required state definition is missing.
    MissingStateDefinition {
        /// Missing state token.
        state_class: String,
    },
    /// A required projection surface is missing from a row.
    MissingProjectionSurface {
        /// Command id that regressed.
        command_id: String,
        /// Missing surface token.
        surface_class: String,
    },
    /// A claim-bearing surface lost dependency-marker disclosure.
    MissingDependencyMarkerDisclosure {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// A stable-facing surface still claimed stable wording after narrowing.
    StableWordingOverclaim {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// A retest-pending or policy-disabled row failed to narrow support wording.
    SupportNarrowingMissing {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// An inspection surface stopped carrying an inspectable detail ref.
    LifecycleNotInspectable {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// A projection stopped being export-safe.
    ProjectionNotExportSafe {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
}

impl fmt::Display for M5CapabilityStateTruthValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRows => write!(f, "m5 capability-state truth packet has no rows"),
            Self::MissingStateDefinition { state_class } => {
                write!(f, "missing capability-state definition for {state_class}")
            }
            Self::MissingProjectionSurface {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} is missing lifecycle projection for {surface_class}"
            ),
            Self::MissingDependencyMarkerDisclosure {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} lost dependency-marker disclosure on {surface_class}"
            ),
            Self::StableWordingOverclaim {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} still claims stable wording on {surface_class} after narrowing"
            ),
            Self::SupportNarrowingMissing {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} still implies ordinary support on {surface_class} after policy or retest narrowing"
            ),
            Self::LifecycleNotInspectable {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} stopped exposing inspectable lifecycle truth on {surface_class}"
            ),
            Self::ProjectionNotExportSafe {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} stopped being export-safe on {surface_class}"
            ),
        }
    }
}

impl Error for M5CapabilityStateTruthValidationError {}

fn state_definitions() -> Vec<M5CapabilityStateDefinitionRow> {
    M5CapabilityStateClass::required_coverage()
        .into_iter()
        .map(|state_class| M5CapabilityStateDefinitionRow {
            state_class,
            display_label: state_class.display_label().to_string(),
            detail_ref: format!("capability-state:{}", state_class.as_str()),
            stable_wording_allowed: state_class.allows_stable_wording(),
            full_support_badge_allowed: state_class.allows_full_support_badge(),
            dependency_markers_required_on_saved_artifacts: !matches!(
                state_class,
                M5CapabilityStateClass::Stable
            ),
            tombstone_state: matches!(state_class, M5CapabilityStateClass::Removed),
        })
        .collect()
}

fn declared_state_class(row: &M5CommandGovernanceRow) -> M5CapabilityStateClass {
    match row.rollout_governance.declared_state_class {
        M5RolloutStateClass::Labs => M5CapabilityStateClass::Labs,
        M5RolloutStateClass::Preview => M5CapabilityStateClass::Preview,
        M5RolloutStateClass::Beta => M5CapabilityStateClass::Beta,
        M5RolloutStateClass::Stable => M5CapabilityStateClass::Stable,
        M5RolloutStateClass::Deprecated => M5CapabilityStateClass::Deprecated,
        M5RolloutStateClass::DisabledByPolicy => M5CapabilityStateClass::DisabledByPolicy,
        M5RolloutStateClass::RetestPending => M5CapabilityStateClass::RetestPending,
        M5RolloutStateClass::Removed => M5CapabilityStateClass::Removed,
    }
}

fn effective_state_class(row: &M5CommandGovernanceRow) -> M5CapabilityStateClass {
    match row.rollout_governance.effective_state_class {
        M5RolloutStateClass::Labs => M5CapabilityStateClass::Labs,
        M5RolloutStateClass::Preview => M5CapabilityStateClass::Preview,
        M5RolloutStateClass::Beta => M5CapabilityStateClass::Beta,
        M5RolloutStateClass::Stable => M5CapabilityStateClass::Stable,
        M5RolloutStateClass::Deprecated => M5CapabilityStateClass::Deprecated,
        M5RolloutStateClass::DisabledByPolicy => M5CapabilityStateClass::DisabledByPolicy,
        M5RolloutStateClass::RetestPending => M5CapabilityStateClass::RetestPending,
        M5RolloutStateClass::Removed => M5CapabilityStateClass::Removed,
    }
}

fn dependency_marker_class_for_state(
    state_class: M5CapabilityStateClass,
) -> Option<M5LifecycleDependencyMarkerClass> {
    match state_class {
        M5CapabilityStateClass::Labs => Some(M5LifecycleDependencyMarkerClass::LabsDependency),
        M5CapabilityStateClass::Preview => {
            Some(M5LifecycleDependencyMarkerClass::PreviewDependency)
        }
        M5CapabilityStateClass::Beta => Some(M5LifecycleDependencyMarkerClass::BetaDependency),
        M5CapabilityStateClass::Stable => None,
        M5CapabilityStateClass::Deprecated => {
            Some(M5LifecycleDependencyMarkerClass::DeprecatedDependency)
        }
        M5CapabilityStateClass::DisabledByPolicy => {
            Some(M5LifecycleDependencyMarkerClass::PolicyDisabledDependency)
        }
        M5CapabilityStateClass::RetestPending => {
            Some(M5LifecycleDependencyMarkerClass::RetestPendingDependency)
        }
        M5CapabilityStateClass::Removed => {
            Some(M5LifecycleDependencyMarkerClass::RemovedDependency)
        }
    }
}

fn marker_label(marker_class: M5LifecycleDependencyMarkerClass) -> &'static str {
    match marker_class {
        M5LifecycleDependencyMarkerClass::LabsDependency => {
            "Depends on a Labs-only capability state"
        }
        M5LifecycleDependencyMarkerClass::PreviewDependency => {
            "Depends on a Preview capability state"
        }
        M5LifecycleDependencyMarkerClass::BetaDependency => "Depends on a Beta capability state",
        M5LifecycleDependencyMarkerClass::PolicyDisabledDependency => {
            "Blocked by policy or kill-switch state"
        }
        M5LifecycleDependencyMarkerClass::RetestPendingDependency => {
            "RetestPending because freshness or qualification lapsed"
        }
        M5LifecycleDependencyMarkerClass::DeprecatedDependency => {
            "Depends on a deprecated capability state"
        }
        M5LifecycleDependencyMarkerClass::RemovedDependency => {
            "Depends on a removed capability state"
        }
        M5LifecycleDependencyMarkerClass::StaleEvidenceDependency => {
            "Evidence freshness lapsed and narrows support claims"
        }
    }
}

fn dependency_markers(row: &M5CommandGovernanceRow) -> Vec<M5LifecycleDependencyMarkerRecord> {
    let effective_state = effective_state_class(row);
    let mut markers = Vec::new();
    if let Some(marker_class) = dependency_marker_class_for_state(effective_state) {
        markers.push(M5LifecycleDependencyMarkerRecord {
            marker_ref: format!("marker:{}:{}", row.canonical_verb, marker_class.as_str()),
            marker_class,
            dependency_ref: row.rollout_state_ref(),
            disclosure_label: marker_label(marker_class).to_string(),
            detail_ref: format!(
                "dependency:{}:{}",
                row.canonical_verb,
                marker_class.as_str()
            ),
            narrows_to_state_class: effective_state,
            affects_surfaces: M5CapabilityProjectionSurfaceClass::required_coverage().to_vec(),
        });
    }

    if effective_state == M5CapabilityStateClass::RetestPending {
        let marker_class = M5LifecycleDependencyMarkerClass::StaleEvidenceDependency;
        markers.push(M5LifecycleDependencyMarkerRecord {
            marker_ref: format!("marker:{}:{}", row.canonical_verb, marker_class.as_str()),
            marker_class,
            dependency_ref: format!("evidence:{}:proof_freshness", row.canonical_verb),
            disclosure_label: marker_label(marker_class).to_string(),
            detail_ref: format!(
                "dependency:{}:{}",
                row.canonical_verb,
                marker_class.as_str()
            ),
            narrows_to_state_class: M5CapabilityStateClass::RetestPending,
            affects_surfaces: M5CapabilityProjectionSurfaceClass::required_coverage().to_vec(),
        });
    }

    markers
}

trait CommandGovernanceRowExt {
    fn rollout_state_ref(&self) -> String;
}

impl CommandGovernanceRowExt for M5CommandGovernanceRow {
    fn rollout_state_ref(&self) -> String {
        self.lifecycle_disclosure.rollout_state_ref.clone()
    }
}

fn owner_ref(row: &M5CommandGovernanceRow) -> String {
    row.rollout_governance.owner_ref.clone()
}

fn capability_row_id(row: &M5CommandGovernanceRow) -> String {
    format!("capability-state:{}", row.canonical_verb)
}

fn command_display_label(row: &M5CommandGovernanceRow) -> String {
    seeded_registry()
        .get(&row.command_id)
        .map(|entry| entry.title.clone())
        .unwrap_or_else(|| row.command_id.clone())
}

fn route_or_metadata_ref(
    row: &M5CommandGovernanceRow,
    surface_class: M5CapabilityProjectionSurfaceClass,
) -> String {
    match surface_class {
        M5CapabilityProjectionSurfaceClass::DesktopInspector => {
            surface_route_ref(row, M5GovernanceSurfaceClass::Desktop, "desktop-inspector")
        }
        M5CapabilityProjectionSurfaceClass::CliInspect => {
            surface_route_ref(row, M5GovernanceSurfaceClass::Cli, "cli-inspect")
        }
        M5CapabilityProjectionSurfaceClass::ExtensionMetadata => surface_route_ref(
            row,
            M5GovernanceSurfaceClass::Extension,
            "extension-metadata",
        ),
        M5CapabilityProjectionSurfaceClass::BrowserCompanion => surface_route_ref(
            row,
            M5GovernanceSurfaceClass::BrowserCompanion,
            "browser-companion",
        ),
        _ => format!(
            "projection:{}:{}",
            row.canonical_verb,
            surface_class.as_str()
        ),
    }
}

fn surface_route_ref(
    row: &M5CommandGovernanceRow,
    required_surface: M5GovernanceSurfaceClass,
    fallback_prefix: &str,
) -> String {
    row.surface_rows
        .iter()
        .find(|surface| surface.surface_class == required_surface)
        .and_then(|surface| {
            surface
                .route_provenance
                .handoff_packet_ref
                .clone()
                .or_else(|| Some(surface.route_provenance.authority_boundary_ref.clone()))
        })
        .unwrap_or_else(|| format!("{fallback_prefix}:{}", row.canonical_verb))
}

fn badge_labels(
    effective_state: M5CapabilityStateClass,
    markers: &[M5LifecycleDependencyMarkerRecord],
) -> Vec<String> {
    let mut labels = vec![effective_state.display_label().to_string()];
    if effective_state == M5CapabilityStateClass::DisabledByPolicy {
        labels.push("PolicyBlocked".to_string());
    }
    if effective_state == M5CapabilityStateClass::RetestPending {
        labels.push("EvidenceStale".to_string());
    }
    for marker in markers {
        let label = match marker.marker_class {
            M5LifecycleDependencyMarkerClass::LabsDependency => "DependsOnLabs",
            M5LifecycleDependencyMarkerClass::PreviewDependency => "DependsOnPreview",
            M5LifecycleDependencyMarkerClass::BetaDependency => "DependsOnBeta",
            M5LifecycleDependencyMarkerClass::PolicyDisabledDependency => "PolicyBlocked",
            M5LifecycleDependencyMarkerClass::RetestPendingDependency => "RetestPending",
            M5LifecycleDependencyMarkerClass::DeprecatedDependency => "DependsOnDeprecated",
            M5LifecycleDependencyMarkerClass::RemovedDependency => "DependsOnRemoved",
            M5LifecycleDependencyMarkerClass::StaleEvidenceDependency => "EvidenceStale",
        };
        if !labels.iter().any(|existing| existing == label) {
            labels.push(label.to_string());
        }
    }
    labels
}

fn evidence_freshness_class(
    row: &M5CommandGovernanceRow,
    effective_state: M5CapabilityStateClass,
) -> String {
    if effective_state == M5CapabilityStateClass::RetestPending {
        "proof_stale".to_string()
    } else {
        row.lifecycle_disclosure.freshness_class.clone()
    }
}

fn stable_wording_visible(
    effective_state: M5CapabilityStateClass,
    markers: &[M5LifecycleDependencyMarkerRecord],
    surface_class: M5CapabilityProjectionSurfaceClass,
) -> bool {
    surface_class.stable_facing()
        && effective_state == M5CapabilityStateClass::Stable
        && markers.is_empty()
}

fn support_wording_visible(
    effective_state: M5CapabilityStateClass,
    surface_class: M5CapabilityProjectionSurfaceClass,
) -> bool {
    if !surface_class.claim_surface() {
        return true;
    }
    effective_state.allows_full_support_badge()
}

fn projection_rows(row: &M5CommandGovernanceRow) -> Vec<M5CapabilityProjectionRow> {
    let declared_state = declared_state_class(row);
    let effective_state = effective_state_class(row);
    let markers = dependency_markers(row);
    let marker_refs = markers
        .iter()
        .map(|marker| marker.marker_ref.clone())
        .collect::<Vec<_>>();
    let evidence_freshness = evidence_freshness_class(row, effective_state);
    let badges = badge_labels(effective_state, &markers);
    M5CapabilityProjectionSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| M5CapabilityProjectionRow {
            surface_class,
            projection_ref: format!(
                "projection:{}:{}",
                row.canonical_verb,
                surface_class.as_str()
            ),
            declared_state_class: declared_state,
            effective_state_class: effective_state,
            stable_wording_visible: stable_wording_visible(
                effective_state,
                &markers,
                surface_class,
            ),
            support_wording_visible: support_wording_visible(effective_state, surface_class),
            dependency_marker_refs: if surface_class.claim_surface()
                || surface_class.inspection_surface()
            {
                marker_refs.clone()
            } else {
                Vec::new()
            },
            dependency_markers_visible: true,
            badge_labels: badges.clone(),
            inspect_detail_ref: row.lifecycle_disclosure.lifecycle_ref.clone(),
            route_or_metadata_ref: route_or_metadata_ref(row, surface_class),
            evidence_freshness_class: evidence_freshness.clone(),
            export_safe: true,
        })
        .collect()
}

fn build_row(row: &M5CommandGovernanceRow) -> M5CapabilityStateTruthRow {
    M5CapabilityStateTruthRow {
        capability_row_id: capability_row_id(row),
        command_id: row.command_id.clone(),
        display_label: command_display_label(row),
        canonical_verb: row.canonical_verb.clone(),
        declared_state_class: declared_state_class(row),
        effective_state_class: effective_state_class(row),
        support_class: row.lifecycle_disclosure.support_class.clone(),
        release_channel: row.lifecycle_disclosure.release_channel.clone(),
        freshness_class: evidence_freshness_class(row, effective_state_class(row)),
        origin_class: row.origin_disclosure.origin_class.clone(),
        owner_ref: owner_ref(row),
        lifecycle_ref: row.lifecycle_disclosure.lifecycle_ref.clone(),
        rollout_state_ref: row.lifecycle_disclosure.rollout_state_ref.clone(),
        dependency_markers: dependency_markers(row),
        projection_rows: projection_rows(row),
        finding_codes: Vec::new(),
    }
}

/// Builds the seeded M5 capability-state truth packet from the M5 command-governance packet.
pub fn seeded_m5_capability_state_truth_packet() -> M5CapabilityStateTruthPacket {
    let source_packet: M5CommandGovernancePacket = seeded_m5_command_governance_packet();
    let state_definitions = state_definitions();
    let rows = source_packet.rows.iter().map(build_row).collect::<Vec<_>>();

    let summary = M5CapabilityStateTruthSummary {
        capability_count: rows.len(),
        state_definition_count: state_definitions.len(),
        dependency_marker_count: rows.iter().map(|row| row.dependency_markers.len()).sum(),
        projection_row_count: rows.iter().map(|row| row.projection_rows.len()).sum(),
        claim_surface_projection_count: rows
            .iter()
            .flat_map(|row| row.projection_rows.iter())
            .filter(|projection| projection.surface_class.claim_surface())
            .count(),
        saved_artifact_projection_count: rows
            .iter()
            .flat_map(|row| row.projection_rows.iter())
            .filter(|projection| projection.surface_class.saved_artifact())
            .count(),
        labs_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5CapabilityStateClass::Labs)
            .count(),
        preview_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5CapabilityStateClass::Preview)
            .count(),
        beta_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5CapabilityStateClass::Beta)
            .count(),
        stable_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5CapabilityStateClass::Stable)
            .count(),
        deprecated_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5CapabilityStateClass::Deprecated)
            .count(),
        disabled_by_policy_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5CapabilityStateClass::DisabledByPolicy)
            .count(),
        retest_pending_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5CapabilityStateClass::RetestPending)
            .count(),
        removed_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5CapabilityStateClass::Removed)
            .count(),
        stable_wording_projection_count: rows
            .iter()
            .flat_map(|row| row.projection_rows.iter())
            .filter(|projection| projection.stable_wording_visible)
            .count(),
        finding_count: rows.iter().map(|row| row.finding_codes.len()).sum(),
    };

    M5CapabilityStateTruthPacket {
        record_kind: M5_CAPABILITY_STATE_TRUTH_RECORD_KIND.to_string(),
        schema_version: M5_CAPABILITY_STATE_TRUTH_SCHEMA_VERSION,
        packet_id: M5_CAPABILITY_STATE_TRUTH_PACKET_ID.to_string(),
        generated_at: GENERATED_AT.to_string(),
        schema_ref: M5_CAPABILITY_STATE_TRUTH_SCHEMA_REF.to_string(),
        doc_ref: M5_CAPABILITY_STATE_TRUTH_DOC_REF.to_string(),
        source_packet_ref: SOURCE_PACKET_REF.to_string(),
        source_rollout_inventory_ref: SOURCE_ROLLOUT_INVENTORY_REF.to_string(),
        lifecycle_vocabulary_ref: SOURCE_LIFECYCLE_VOCAB_REF.to_string(),
        projection_matrix_ref: SOURCE_PROJECTION_MATRIX_REF.to_string(),
        state_definitions,
        rows,
        summary,
    }
}

/// Returns the current seeded packet after validating it.
pub fn current_m5_capability_state_truth_export(
) -> Result<M5CapabilityStateTruthPacket, Vec<M5CapabilityStateTruthValidationError>> {
    let packet = seeded_m5_capability_state_truth_packet();
    validate_m5_capability_state_truth_packet(&packet)?;
    Ok(packet)
}

/// Validates the canonical M5 capability-state truth packet.
pub fn validate_m5_capability_state_truth_packet(
    packet: &M5CapabilityStateTruthPacket,
) -> Result<(), Vec<M5CapabilityStateTruthValidationError>> {
    let mut errors = Vec::new();
    if packet.rows.is_empty() {
        errors.push(M5CapabilityStateTruthValidationError::NoRows);
    }

    for required_state in M5CapabilityStateClass::required_coverage() {
        if !packet
            .state_definitions
            .iter()
            .any(|definition| definition.state_class == required_state)
        {
            errors.push(
                M5CapabilityStateTruthValidationError::MissingStateDefinition {
                    state_class: required_state.as_str().to_string(),
                },
            );
        }
    }

    for row in &packet.rows {
        for required_surface in M5CapabilityProjectionSurfaceClass::required_coverage() {
            let Some(projection) = row
                .projection_rows
                .iter()
                .find(|projection| projection.surface_class == required_surface)
            else {
                errors.push(
                    M5CapabilityStateTruthValidationError::MissingProjectionSurface {
                        command_id: row.command_id.clone(),
                        surface_class: required_surface.as_str().to_string(),
                    },
                );
                continue;
            };

            if required_surface.claim_surface()
                && !row.dependency_markers.is_empty()
                && (!projection.dependency_markers_visible
                    || projection.dependency_marker_refs.is_empty())
            {
                errors.push(
                    M5CapabilityStateTruthValidationError::MissingDependencyMarkerDisclosure {
                        command_id: row.command_id.clone(),
                        surface_class: required_surface.as_str().to_string(),
                    },
                );
            }

            if required_surface.stable_facing()
                && projection.stable_wording_visible
                && (!projection.effective_state_class.allows_stable_wording()
                    || !projection.dependency_marker_refs.is_empty())
            {
                errors.push(
                    M5CapabilityStateTruthValidationError::StableWordingOverclaim {
                        command_id: row.command_id.clone(),
                        surface_class: required_surface.as_str().to_string(),
                    },
                );
            }

            if matches!(
                projection.effective_state_class,
                M5CapabilityStateClass::DisabledByPolicy | M5CapabilityStateClass::RetestPending
            ) && projection.support_wording_visible
                && required_surface.claim_surface()
            {
                errors.push(
                    M5CapabilityStateTruthValidationError::SupportNarrowingMissing {
                        command_id: row.command_id.clone(),
                        surface_class: required_surface.as_str().to_string(),
                    },
                );
            }

            if required_surface.inspection_surface()
                && (projection.inspect_detail_ref.trim().is_empty()
                    || projection.route_or_metadata_ref.trim().is_empty())
            {
                errors.push(
                    M5CapabilityStateTruthValidationError::LifecycleNotInspectable {
                        command_id: row.command_id.clone(),
                        surface_class: required_surface.as_str().to_string(),
                    },
                );
            }

            if !projection.export_safe {
                errors.push(
                    M5CapabilityStateTruthValidationError::ProjectionNotExportSafe {
                        command_id: row.command_id.clone(),
                        surface_class: required_surface.as_str().to_string(),
                    },
                );
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
