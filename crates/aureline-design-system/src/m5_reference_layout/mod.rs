//! Versioned, machine-readable reference-layout descriptors and shell-slot conformance packets for
//! the dominant M5 workspaces.
//!
//! Where [`crate::m5_design_system_contract`] freezes the *governance matrix* — which design-system
//! objects exist and whether each claimed surface maps them — [`crate::m5_foundation_package`]
//! ships the versioned *foundations*, and [`crate::m5_component_manifest`] ships the durable
//! *component contracts*, this module ships the *reference layouts*: a versioned
//! [`M5ReferenceLayoutPackage`] carrying one [`M5WorkspaceReferenceLayout`] per launch-critical M5
//! workspace family — notebooks, data grids, the profiler, pipelines, docs, preview, incident, and
//! companion surfaces.
//!
//! Each descriptor is the single, cite-able truth for how a workspace occupies the governed shell
//! zones instead of placing panes ad hoc. It records, for one workspace:
//!
//! - **zone occupancy** — which [shell zone](M5ShellZone) the workspace claims, the governed
//!   [slot id](M5ZoneOccupancy::slot_id) it fills, the [surface kind](M5SurfaceKind) that fills it,
//!   whether the zone is mandatory, and the placeholder behavior the zone shows before content
//!   resolves.
//! - **responsive collapse** — per [adaptive class](M5AdaptiveClass), which zones collapse and the
//!   [fallback placement](M5FallbackPlacement) (sheet, overflow, or in-slot placeholder) they
//!   collapse to, so the sheet fallbacks are declared rather than improvised.
//! - **missing-dependency placeholders** — when a dependency is absent, the
//!   [placeholder class](M5PlaceholderClass), governed message id, and degraded behavior the
//!   affected zone shows, so a missing kernel, remote, provider, or extension degrades to the
//!   declared placeholder instead of a blank pane.
//! - **reopen / reset routes** — the [routes](M5LayoutRoute) that reopen a closed surface or reset
//!   the workspace to its reference layout, each with its governed command message id and key chord.
//!
//! The package projects two derived records:
//!
//! - [`M5ReferenceLayoutPackage::release_packet`] mints a [`M5ReferenceLayoutReleasePacket`] with
//!   one lifecycle-and-shape summary per workspace, so a release record names the layout revision
//!   QA and support exports cite.
//! - [`M5ReferenceLayoutPackage::conformance_packet`] mints a [`M5ShellSlotConformancePacket`] — the
//!   flattened, slot-keyed layout truth a feature implementation tests against, so a notebook,
//!   profiler, or pipeline surface can be checked against the same descriptor the design system
//!   ships rather than a hand-written assertion list.
//!
//! The governed zone, slot, fallback-placement, and placeholder-class tokens this module publishes
//! match the canonical shell vocabulary, so shell code, docs/help, and support exports name the
//! same layout identities and collapse states users actually see.
//!
//! The records are metadata-only truth packets: they carry semantic slot *ids* and message *ids*,
//! never raw geometry payloads, credential bodies, or provider payloads.
//!
//! - Schema:
//!   [`schemas/design-system/m5-reference-layout-package.schema.json`](../../../../../schemas/design-system/m5-reference-layout-package.schema.json)
//! - Doc:
//!   [`docs/design-system/m5-reference-layout-package.md`](../../../../../docs/design-system/m5-reference-layout-package.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_reference_layout_package, M5_REFERENCE_LAYOUT_PACKAGE_ID,
    M5_REFERENCE_LAYOUT_PACKAGE_VERSION,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Record-kind tag carried by [`M5ReferenceLayoutPackage`].
pub const M5_REFERENCE_LAYOUT_PACKAGE_RECORD_KIND: &str =
    "m5_design_system_reference_layout_package";

/// Record-kind tag carried by [`M5ReferenceLayoutReleasePacket`].
pub const M5_REFERENCE_LAYOUT_RELEASE_RECORD_KIND: &str =
    "m5_design_system_reference_layout_release";

/// Record-kind tag carried by [`M5ShellSlotConformancePacket`].
pub const M5_SHELL_SLOT_CONFORMANCE_RECORD_KIND: &str =
    "m5_design_system_shell_slot_conformance_packet";

/// Schema version shared by the reference-layout records.
pub const M5_REFERENCE_LAYOUT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the reference-layout-package boundary schema.
pub const M5_REFERENCE_LAYOUT_SCHEMA_REF: &str =
    "schemas/design-system/m5-reference-layout-package.schema.json";

/// Repo-relative path of the reference-layout contract doc.
pub const M5_REFERENCE_LAYOUT_DOC_REF: &str = "docs/design-system/m5-reference-layout-package.md";

/// Repo-relative path of the release-grade reference-layout proof packet — the proof lane that
/// blocks drift for the package.
pub const M5_REFERENCE_LAYOUT_PROOF_REF: &str =
    "artifacts/release/m5-design-system-proof/reference-layout-release.json";

/// Repo-relative path of the shell-slot conformance packet feature implementations test against.
pub const M5_REFERENCE_LAYOUT_CONFORMANCE_REF: &str =
    "artifacts/release/m5-design-system-proof/reference-layout-conformance.json";

/// Release packet that keeps the reference layouts current (shared with the foundation package,
/// component manifests, and contract matrix).
pub const M5_REFERENCE_LAYOUT_RELEASE_PACKET_REF: &str = "evidence:m5-design-system-release-packet";

/// Repo-relative directory of the checked-in reference-layout fixtures.
pub const M5_REFERENCE_LAYOUT_DIR: &str = "fixtures/ui/m5-reference-layout/";

/// Repo-relative extension-SDK guidance an extension author reads to claim shell zones.
pub const M5_REFERENCE_LAYOUT_EXTENSION_GUIDANCE_REF: &str =
    "docs/sdk/extension-ui-design-system.md";

/// Prefix every governed message id in this lane carries so consumers can route them.
pub const M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX: &str = "design_system_reference_layout.";

/// One launch-critical M5 workspace family the package publishes a reference layout for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceKind {
    /// Notebook workspace: cell working set, kernel runtime, and outputs.
    Notebook,
    /// Data-grid workspace: a dense, virtualizable result grid with a query/source surface.
    DataGrid,
    /// Profiler workspace: a capture working set with a flame/detail inspector.
    Profiler,
    /// Pipeline workspace: stage cards and a job/run detail surface.
    Pipeline,
    /// Docs workspace: an embedded docs/browser reading pane with navigation.
    Docs,
    /// Preview workspace: a live preview canvas with route and trust truth.
    Preview,
    /// Incident workspace: an incident timeline with linked evidence and actions.
    Incident,
    /// Companion workspace: a cross-device companion surface that mirrors a primary session.
    Companion,
}

impl M5WorkspaceKind {
    /// Every workspace kind, in declaration order. The package must publish one layout per kind.
    pub const ALL: [Self; 8] = [
        Self::Notebook,
        Self::DataGrid,
        Self::Profiler,
        Self::Pipeline,
        Self::Docs,
        Self::Preview,
        Self::Incident,
        Self::Companion,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataGrid => "data_grid",
            Self::Profiler => "profiler",
            Self::Pipeline => "pipeline",
            Self::Docs => "docs",
            Self::Preview => "preview",
            Self::Incident => "incident",
            Self::Companion => "companion",
        }
    }
}

/// One governed shell zone a workspace can claim. The tokens match the canonical shell zone
/// vocabulary so shell code consumes the same identities the descriptor names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellZone {
    /// Top identity strip (workspace, trust, and execution-target identity).
    TitleContextBar,
    /// Durable top-level route rail.
    ActivityRail,
    /// Structural navigation sidebar.
    LeftSidebar,
    /// Primary work surface (editor groups, review, primary worksets).
    MainWorkspace,
    /// Contextual detail inspector.
    RightInspector,
    /// Execution / output / terminal panel.
    BottomPanel,
    /// Persistent status strip.
    StatusBar,
    /// Transient overlay (dialogs, sheets, command palette).
    TransientOverlay,
}

impl M5ShellZone {
    /// Every shell zone, in canonical order.
    pub const ALL: [Self; 8] = [
        Self::TitleContextBar,
        Self::ActivityRail,
        Self::LeftSidebar,
        Self::MainWorkspace,
        Self::RightInspector,
        Self::BottomPanel,
        Self::StatusBar,
        Self::TransientOverlay,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TitleContextBar => "title_context_bar",
            Self::ActivityRail => "activity_rail",
            Self::LeftSidebar => "left_sidebar",
            Self::MainWorkspace => "main_workspace",
            Self::RightInspector => "right_inspector",
            Self::BottomPanel => "bottom_panel",
            Self::StatusBar => "status_bar",
            Self::TransientOverlay => "transient_overlay",
        }
    }

    /// The canonical default slot id for this zone, matching the shell's declared slot vocabulary.
    pub const fn canonical_slot_id(self) -> &'static str {
        match self {
            Self::TitleContextBar => "slot.title_context_bar.identity",
            Self::ActivityRail => "slot.activity_rail.primary_routes",
            Self::LeftSidebar => "slot.sidebar.section_surface",
            Self::MainWorkspace => "slot.main_workspace.working_set",
            Self::RightInspector => "slot.right_inspector.contextual_detail",
            Self::BottomPanel => "slot.bottom_panel.tool_panels",
            Self::StatusBar => "status.slot.recovery.primary",
            Self::TransientOverlay => "slot.overlay.dialog_or_sheet",
        }
    }

    /// True when this zone is persistent: it never collapses out of a workspace, because losing it
    /// would erase the work surface or the status truth users rely on.
    pub const fn is_persistent(self) -> bool {
        matches!(self, Self::MainWorkspace | Self::StatusBar)
    }
}

/// The contribution class that fills a zone. Tokens match the canonical shell surface-kind
/// vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceKind {
    /// Required host chrome.
    HostChrome,
    /// First-party core product surface.
    FirstParty,
    /// Provider-backed surface reached through a contract.
    ProviderBacked,
    /// Extension-contributed surface.
    ExtensionContributed,
}

impl M5SurfaceKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostChrome => "host_chrome",
            Self::FirstParty => "first_party",
            Self::ProviderBacked => "provider_backed",
            Self::ExtensionContributed => "extension_contributed",
        }
    }
}

/// A responsive breakpoint class. Tokens match the canonical shell adaptive-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdaptiveClass {
    /// Compact desktop (narrow windows).
    CompactDesktop,
    /// Standard desktop.
    StandardDesktop,
    /// Expanded desktop (wide windows).
    ExpandedDesktop,
}

impl M5AdaptiveClass {
    /// Every adaptive class, in widening order. A layout publishes one collapse rule per class.
    pub const ALL: [Self; 3] = [
        Self::CompactDesktop,
        Self::StandardDesktop,
        Self::ExpandedDesktop,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactDesktop => "compact_desktop",
            Self::StandardDesktop => "standard_desktop",
            Self::ExpandedDesktop => "expanded_desktop",
        }
    }
}

/// Where a collapsed zone goes when a workspace narrows. Tokens match the canonical shell
/// fallback-placement vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackPlacement {
    /// The zone stays docked in place (no collapse).
    Docked,
    /// The zone collapses to an attached sheet.
    Sheet,
    /// The zone collapses to a keyboard-reachable overflow route.
    Overflow,
    /// The zone collapses to an in-slot placeholder that preserves its identity.
    Placeholder,
}

impl M5FallbackPlacement {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docked => "docked",
            Self::Sheet => "sheet",
            Self::Overflow => "overflow",
            Self::Placeholder => "placeholder",
        }
    }
}

/// Why a missing-dependency placeholder appears. Tokens match the canonical shell placeholder-class
/// vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlaceholderClass {
    /// Required shell chrome that cannot be closed.
    RequiredChrome,
    /// A removed or disabled extension surface.
    MissingExtension,
    /// An unavailable session or remote.
    MissingRemote,
    /// An unavailable provider-backed detail.
    MissingProvider,
    /// A capability that is no longer granted.
    CapabilityLoss,
    /// A display-topology change moved the surface.
    TopologyDrift,
}

impl M5PlaceholderClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredChrome => "required_chrome",
            Self::MissingExtension => "missing_extension",
            Self::MissingRemote => "missing_remote",
            Self::MissingProvider => "missing_provider",
            Self::CapabilityLoss => "capability_loss",
            Self::TopologyDrift => "topology_drift",
        }
    }
}

/// Whether a layout route reopens a closed surface or resets the workspace to its reference layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayoutRouteKind {
    /// Reopens a surface the user closed, restoring its zone occupancy.
    Reopen,
    /// Resets the workspace to its reference layout.
    Reset,
}

impl M5LayoutRouteKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reopen => "reopen",
            Self::Reset => "reset",
        }
    }
}

/// Lifecycle state of a published reference layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceLifecycleState {
    /// In active design; the layout may still change.
    Experimental,
    /// Shape is settling and consumable, ahead of a stable commitment.
    Preview,
    /// Committed layout; changes are versioned and reviewed.
    Stable,
    /// On a removal path; consumers should migrate.
    Deprecated,
}

impl M5WorkspaceLifecycleState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Experimental => "experimental",
            Self::Preview => "preview",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Versioned lifecycle and owner metadata for one reference layout, so design QA, support exports,
/// and release packets point at the same layout revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceLifecycle {
    /// Owner role accountable for the layout.
    pub owner_role: String,
    /// Lifecycle state of the layout.
    pub lifecycle_state: M5WorkspaceLifecycleState,
    /// Monotonic layout version; bumps when this workspace's layout changes.
    pub layout_version: u32,
    /// Package version (semver) the layout was introduced in.
    pub introduced_in_package_version: String,
}

/// One zone a workspace occupies, with the slot it fills, the surface kind that fills it, whether
/// the zone is mandatory, and the placeholder behavior the zone shows before content resolves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ZoneOccupancy {
    /// The governed shell zone.
    pub zone: M5ShellZone,
    /// The governed slot id the workspace fills in the zone.
    pub slot_id: String,
    /// What the workspace renders in the zone.
    pub surface_role: String,
    /// The contribution class that fills the zone.
    pub surface_kind: M5SurfaceKind,
    /// True when the workspace cannot render without this zone.
    pub required: bool,
    /// What the zone shows before its content resolves, so it never goes blank.
    pub placeholder_behavior: String,
}

/// One workspace's responsive collapse rule at a single adaptive class: which zones collapse and
/// where they go.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResponsiveCollapseRule {
    /// The adaptive class this rule applies at.
    pub adaptive_class: M5AdaptiveClass,
    /// The zones that collapse at this class (empty when nothing collapses).
    pub collapsed_zones: Vec<M5ShellZone>,
    /// Where the collapsed zones go.
    pub placement: M5FallbackPlacement,
    /// True when the collapse preserves each zone's slot identity and reopen route.
    pub preserves_zone_identity: bool,
    /// Human-readable description of the collapse behavior.
    pub behavior: String,
}

/// One workspace's missing-dependency placeholder rule: when a dependency is absent, what the
/// affected zone shows instead of a blank pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MissingDependencyRule {
    /// Stable id of the missing dependency.
    pub dependency_id: String,
    /// The zone whose content the dependency feeds.
    pub affected_zone: M5ShellZone,
    /// Why the placeholder appears.
    pub placeholder_class: M5PlaceholderClass,
    /// Governed message id the placeholder announces; prefixed
    /// [`M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX`].
    pub placeholder_message_id: String,
    /// What still works while the dependency is missing.
    pub degraded_behavior: String,
}

/// One route that reopens a closed surface or resets the workspace to its reference layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LayoutRoute {
    /// Stable route id, unique within the layout.
    pub route_id: String,
    /// Whether the route reopens a surface or resets the workspace.
    pub route_kind: M5LayoutRouteKind,
    /// Governed command label message id; prefixed [`M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX`].
    pub command_message_id: String,
    /// Default key chord that invokes the route.
    pub keys: String,
    /// What the route restores.
    pub description: String,
}

/// One launch-critical workspace's reference layout: how it occupies the governed shell zones,
/// collapses responsively, degrades when a dependency is missing, and reopens or resets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceReferenceLayout {
    /// The governed workspace kind.
    pub workspace_kind: M5WorkspaceKind,
    /// Stable workspace id, unique within the package.
    pub workspace_id: String,
    /// Human-readable workspace name.
    pub display_name: String,
    /// Versioned lifecycle and owner metadata.
    pub lifecycle: M5WorkspaceLifecycle,
    /// The zones the workspace occupies.
    pub zone_occupancy: Vec<M5ZoneOccupancy>,
    /// The responsive collapse rules (one per [`M5AdaptiveClass`]).
    pub responsive_collapse: Vec<M5ResponsiveCollapseRule>,
    /// The missing-dependency placeholder rules.
    pub missing_dependency_rules: Vec<M5MissingDependencyRule>,
    /// The reopen / reset routes.
    pub reopen_routes: Vec<M5LayoutRoute>,
    /// Stable summary message id; prefixed [`M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5WorkspaceReferenceLayout {
    /// Finds the occupancy for a zone.
    pub fn zone(&self, zone: M5ShellZone) -> Option<&M5ZoneOccupancy> {
        self.zone_occupancy.iter().find(|o| o.zone == zone)
    }

    /// The zones the workspace occupies, in declared order.
    pub fn occupied_zones(&self) -> Vec<M5ShellZone> {
        self.zone_occupancy.iter().map(|o| o.zone).collect()
    }

    /// The occupancies the workspace cannot render without.
    pub fn required_zones(&self) -> Vec<&M5ZoneOccupancy> {
        self.zone_occupancy.iter().filter(|o| o.required).collect()
    }

    /// The collapse rule for an adaptive class.
    pub fn collapse_rule(&self, class: M5AdaptiveClass) -> Option<&M5ResponsiveCollapseRule> {
        self.responsive_collapse
            .iter()
            .find(|r| r.adaptive_class == class)
    }

    /// The reopen / reset routes of a kind.
    pub fn routes_of_kind(&self, kind: M5LayoutRouteKind) -> Vec<&M5LayoutRoute> {
        self.reopen_routes
            .iter()
            .filter(|r| r.route_kind == kind)
            .collect()
    }
}

/// A versioned, machine-readable package of M5 workspace reference layouts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReferenceLayoutPackage {
    /// Record kind; must equal [`M5_REFERENCE_LAYOUT_PACKAGE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REFERENCE_LAYOUT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable package id.
    pub package_id: String,
    /// Package version (semver `MAJOR.MINOR.PATCH`).
    pub package_version: String,
    /// Owner role accountable for the package.
    pub owner_role: String,
    /// The governed workspace reference layouts (one per [`M5WorkspaceKind`]).
    pub layouts: Vec<M5WorkspaceReferenceLayout>,
    /// Repo-relative proof lane that blocks drift.
    pub proof_lane_ref: String,
    /// Repo-relative release packet that keeps the package current.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable summary message id; prefixed [`M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5ReferenceLayoutPackage {
    /// Finds the layout for a workspace kind.
    pub fn layout(&self, kind: M5WorkspaceKind) -> Option<&M5WorkspaceReferenceLayout> {
        self.layouts.iter().find(|l| l.workspace_kind == kind)
    }

    /// Total layout count.
    pub fn total_layouts(&self) -> usize {
        self.layouts.len()
    }

    /// Validates the package invariants, returning the violations (empty when valid).
    pub fn validate(&self) -> Vec<M5ReferenceLayoutViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REFERENCE_LAYOUT_PACKAGE_RECORD_KIND {
            violations.push(M5ReferenceLayoutViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REFERENCE_LAYOUT_SCHEMA_VERSION {
            violations.push(M5ReferenceLayoutViolation::WrongSchemaVersion);
        }
        if self.package_id.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.proof_lane_ref.trim().is_empty()
            || self.release_packet_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReferenceLayoutViolation::MissingIdentity);
        }
        if !is_semver(&self.package_version) {
            violations.push(M5ReferenceLayoutViolation::BadPackageVersion);
        }
        if !self
            .summary_message_id
            .starts_with(M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX)
        {
            violations.push(M5ReferenceLayoutViolation::MessageIdPrefixMissing);
        }

        for required in [
            M5_REFERENCE_LAYOUT_SCHEMA_REF,
            M5_REFERENCE_LAYOUT_DOC_REF,
            M5_REFERENCE_LAYOUT_PROOF_REF,
        ] {
            if !self.source_contract_refs.iter().any(|r| r == required) {
                violations.push(M5ReferenceLayoutViolation::MissingSourceContracts);
                break;
            }
        }

        validate_layout_set(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 reference layout package serializes"),
        ) {
            violations.push(M5ReferenceLayoutViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// True when the package validates with no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Deterministic export-safe JSON for the package.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 reference layout package serializes")
    }

    /// Imports a package from JSON. The caller validates the returned package with
    /// [`Self::validate`].
    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    /// Projects the release-packet inclusion: one lifecycle-and-shape summary per layout, so a
    /// release record names the layout revision QA and support exports cite.
    pub fn release_packet(&self) -> M5ReferenceLayoutReleasePacket {
        let layout_summaries: Vec<M5WorkspaceLayoutSummary> = self
            .layouts
            .iter()
            .map(|l| M5WorkspaceLayoutSummary {
                workspace_kind: l.workspace_kind,
                workspace_id: l.workspace_id.clone(),
                lifecycle_state: l.lifecycle.lifecycle_state,
                layout_version: l.lifecycle.layout_version,
                zone_count: l.zone_occupancy.len() as u32,
                required_zone_count: l.required_zones().len() as u32,
                collapse_rule_count: l.responsive_collapse.len() as u32,
                missing_dependency_count: l.missing_dependency_rules.len() as u32,
                reopen_route_count: l.reopen_routes.len() as u32,
            })
            .collect();

        M5ReferenceLayoutReleasePacket {
            record_kind: M5_REFERENCE_LAYOUT_RELEASE_RECORD_KIND.to_owned(),
            schema_version: M5_REFERENCE_LAYOUT_SCHEMA_VERSION,
            package_id: self.package_id.clone(),
            package_version: self.package_version.clone(),
            total_layouts: self.total_layouts() as u32,
            layout_summaries,
            proof_lane_ref: self.proof_lane_ref.clone(),
            release_packet_ref: self.release_packet_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            redaction_class_token: self.redaction_class_token.clone(),
            summary_message_id: format!(
                "{}{}.release",
                M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX, self.package_id
            ),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Projects the shell-slot conformance packet: the flattened, slot-keyed layout truth a feature
    /// implementation tests against, so a notebook, profiler, or pipeline surface is checked against
    /// the same descriptor the design system ships. Collapse and missing-dependency expectations are
    /// resolved to the exact slot ids the zones occupy, so a feature test names slots rather than
    /// zones.
    pub fn conformance_packet(&self) -> M5ShellSlotConformancePacket {
        let workspace_conformance: Vec<M5WorkspaceSlotConformance> =
            self.layouts.iter().map(workspace_conformance).collect();

        let total_slot_expectations: u32 = workspace_conformance
            .iter()
            .map(|w| w.slot_expectations.len() as u32)
            .sum();

        M5ShellSlotConformancePacket {
            record_kind: M5_SHELL_SLOT_CONFORMANCE_RECORD_KIND.to_owned(),
            schema_version: M5_REFERENCE_LAYOUT_SCHEMA_VERSION,
            package_id: self.package_id.clone(),
            package_version: self.package_version.clone(),
            total_workspaces: self.total_layouts() as u32,
            total_slot_expectations,
            workspace_conformance,
            proof_lane_ref: self.proof_lane_ref.clone(),
            conformance_ref: M5_REFERENCE_LAYOUT_CONFORMANCE_REF.to_owned(),
            release_packet_ref: self.release_packet_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            redaction_class_token: self.redaction_class_token.clone(),
            summary_message_id: format!(
                "{}{}.conformance",
                M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX, self.package_id
            ),
            minted_at: self.minted_at.clone(),
        }
    }
}

/// Builds the conformance projection for one layout: per-slot, per-collapse, per-missing-dependency,
/// and per-route expectations, with collapse and missing-dependency expectations resolved to the
/// slot ids the zones occupy.
fn workspace_conformance(layout: &M5WorkspaceReferenceLayout) -> M5WorkspaceSlotConformance {
    let slot_for_zone =
        |zone: M5ShellZone| -> Option<String> { layout.zone(zone).map(|o| o.slot_id.clone()) };

    let slot_expectations: Vec<M5SlotExpectation> = layout
        .zone_occupancy
        .iter()
        .map(|o| M5SlotExpectation {
            zone: o.zone,
            slot_id: o.slot_id.clone(),
            surface_kind: o.surface_kind,
            required: o.required,
            expected_placeholder_behavior: o.placeholder_behavior.clone(),
        })
        .collect();

    let collapse_expectations: Vec<M5CollapseExpectation> = layout
        .responsive_collapse
        .iter()
        .map(|rule| {
            let collapsed_slot_ids: Vec<String> = rule
                .collapsed_zones
                .iter()
                .filter_map(|z| slot_for_zone(*z))
                .collect();
            M5CollapseExpectation {
                adaptive_class: rule.adaptive_class,
                collapsed_slot_ids,
                placement: rule.placement,
                preserves_zone_identity: rule.preserves_zone_identity,
            }
        })
        .collect();

    let missing_dependency_expectations: Vec<M5MissingDependencyExpectation> = layout
        .missing_dependency_rules
        .iter()
        .map(|rule| M5MissingDependencyExpectation {
            dependency_id: rule.dependency_id.clone(),
            affected_slot_id: slot_for_zone(rule.affected_zone),
            affected_zone: rule.affected_zone,
            placeholder_class: rule.placeholder_class,
            placeholder_message_id: rule.placeholder_message_id.clone(),
        })
        .collect();

    let reopen_route_expectations: Vec<M5ReopenRouteExpectation> = layout
        .reopen_routes
        .iter()
        .map(|route| M5ReopenRouteExpectation {
            route_id: route.route_id.clone(),
            route_kind: route.route_kind,
            command_message_id: route.command_message_id.clone(),
            keys: route.keys.clone(),
        })
        .collect();

    M5WorkspaceSlotConformance {
        workspace_kind: layout.workspace_kind,
        workspace_id: layout.workspace_id.clone(),
        display_name: layout.display_name.clone(),
        slot_expectations,
        collapse_expectations,
        missing_dependency_expectations,
        reopen_route_expectations,
    }
}

/// Reads and validates the checked-in canonical reference-layout package fixture.
pub fn current_stable_m5_reference_layout_package(
) -> Result<M5ReferenceLayoutPackage, M5ReferenceLayoutArtifactError> {
    let package: M5ReferenceLayoutPackage = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-reference-layout/reference-layout-package.json"
    )))
    .map_err(M5ReferenceLayoutArtifactError::Parse)?;
    let violations = package.validate();
    if violations.is_empty() {
        Ok(package)
    } else {
        Err(M5ReferenceLayoutArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading a checked-in reference-layout-package export.
#[derive(Debug)]
pub enum M5ReferenceLayoutArtifactError {
    /// The export failed to parse.
    Parse(serde_json::Error),
    /// The export failed validation.
    Validation(Vec<M5ReferenceLayoutViolation>),
}

impl fmt::Display for M5ReferenceLayoutArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => {
                write!(
                    formatter,
                    "m5 reference layout package parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                write!(
                    formatter,
                    "m5 reference layout package failed validation: {}",
                    tokens.join(",")
                )
            }
        }
    }
}

impl Error for M5ReferenceLayoutArtifactError {}

/// Validation failures emitted by [`M5ReferenceLayoutPackage::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReferenceLayoutViolation {
    /// Package record kind is wrong.
    WrongRecordKind,
    /// Package schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The package version is not `MAJOR.MINOR.PATCH`.
    BadPackageVersion,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A governed workspace kind has no published layout.
    RequiredWorkspaceKindMissing,
    /// Two layouts share a kind.
    DuplicateWorkspaceKind,
    /// Two layouts share a workspace id.
    DuplicateWorkspaceId,
    /// A layout is missing an identity field (workspace id, display name, or summary id).
    LayoutIncomplete,
    /// A layout's lifecycle metadata is incomplete (empty owner, zero version, or bad introduced
    /// version).
    LifecycleIncomplete,
    /// A layout's zone occupancy is empty, has duplicate zones, declares no required zone, omits the
    /// main work surface, or has an incomplete occupancy.
    ZoneOccupancyIncomplete,
    /// A layout's responsive collapse rules do not cover exactly the canonical adaptive classes, or
    /// collapse a zone the workspace does not occupy or that is persistent.
    CollapseRulesIncomplete,
    /// A layout's missing-dependency rules are empty, reference an unoccupied zone, or carry an
    /// unprefixed / empty field.
    MissingDependencyRulesIncomplete,
    /// A layout's reopen routes are empty, lack a reopen or reset route, have duplicate ids, or
    /// carry an unprefixed message id / empty key chord.
    ReopenRoutesIncomplete,
    /// A message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ReferenceLayoutViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::BadPackageVersion => "bad_package_version",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredWorkspaceKindMissing => "required_workspace_kind_missing",
            Self::DuplicateWorkspaceKind => "duplicate_workspace_kind",
            Self::DuplicateWorkspaceId => "duplicate_workspace_id",
            Self::LayoutIncomplete => "layout_incomplete",
            Self::LifecycleIncomplete => "lifecycle_incomplete",
            Self::ZoneOccupancyIncomplete => "zone_occupancy_incomplete",
            Self::CollapseRulesIncomplete => "collapse_rules_incomplete",
            Self::MissingDependencyRulesIncomplete => "missing_dependency_rules_incomplete",
            Self::ReopenRoutesIncomplete => "reopen_routes_incomplete",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Release-packet records.
// ---------------------------------------------------------------------------

/// Release-packet projection of a reference-layout package: one lifecycle-and-shape summary per
/// layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReferenceLayoutReleasePacket {
    /// Record kind; must equal [`M5_REFERENCE_LAYOUT_RELEASE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The package id this release record projects.
    pub package_id: String,
    /// The package version.
    pub package_version: String,
    /// Total layouts across the package.
    pub total_layouts: u32,
    /// Per-layout lifecycle and shape summaries, in package order.
    pub layout_summaries: Vec<M5WorkspaceLayoutSummary>,
    /// Repo-relative proof lane.
    pub proof_lane_ref: String,
    /// Repo-relative release packet.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable message id; prefixed [`M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5ReferenceLayoutReleasePacket {
    /// Deterministic export-safe JSON for the release packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 reference layout release packet serializes")
    }
}

/// One layout's lifecycle and shape summary inside a release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceLayoutSummary {
    /// The governed workspace kind.
    pub workspace_kind: M5WorkspaceKind,
    /// The workspace id.
    pub workspace_id: String,
    /// The layout's lifecycle state.
    pub lifecycle_state: M5WorkspaceLifecycleState,
    /// The layout version.
    pub layout_version: u32,
    /// Occupied zone count.
    pub zone_count: u32,
    /// Required zone count.
    pub required_zone_count: u32,
    /// Responsive collapse rule count.
    pub collapse_rule_count: u32,
    /// Missing-dependency rule count.
    pub missing_dependency_count: u32,
    /// Reopen / reset route count.
    pub reopen_route_count: u32,
}

// ---------------------------------------------------------------------------
// Shell-slot conformance records.
// ---------------------------------------------------------------------------

/// The shell-slot conformance packet a feature implementation tests against: per-workspace,
/// slot-keyed expectations derived from the reference layouts, so a feature surface is checked
/// against the same descriptor the design system ships rather than a hand-written assertion list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellSlotConformancePacket {
    /// Record kind; must equal [`M5_SHELL_SLOT_CONFORMANCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The package id this conformance packet projects.
    pub package_id: String,
    /// The package version.
    pub package_version: String,
    /// Total workspaces covered.
    pub total_workspaces: u32,
    /// Total slot expectations across all workspaces.
    pub total_slot_expectations: u32,
    /// Per-workspace conformance, in package order.
    pub workspace_conformance: Vec<M5WorkspaceSlotConformance>,
    /// Repo-relative proof lane.
    pub proof_lane_ref: String,
    /// Repo-relative path of this conformance packet.
    pub conformance_ref: String,
    /// Repo-relative release packet.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable message id; prefixed [`M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5ShellSlotConformancePacket {
    /// Deterministic export-safe JSON for the conformance packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 shell slot conformance packet serializes")
    }

    /// Finds the conformance for a workspace kind.
    pub fn workspace(&self, kind: M5WorkspaceKind) -> Option<&M5WorkspaceSlotConformance> {
        self.workspace_conformance
            .iter()
            .find(|w| w.workspace_kind == kind)
    }
}

/// One workspace's slot-keyed conformance expectations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceSlotConformance {
    /// The governed workspace kind.
    pub workspace_kind: M5WorkspaceKind,
    /// The workspace id.
    pub workspace_id: String,
    /// Human-readable workspace name.
    pub display_name: String,
    /// Per-slot expectations: the slots the workspace claims and their placeholder behavior.
    pub slot_expectations: Vec<M5SlotExpectation>,
    /// Per-adaptive-class collapse expectations, resolved to the slot ids that collapse.
    pub collapse_expectations: Vec<M5CollapseExpectation>,
    /// Per-missing-dependency expectations.
    pub missing_dependency_expectations: Vec<M5MissingDependencyExpectation>,
    /// Reopen / reset route expectations.
    pub reopen_route_expectations: Vec<M5ReopenRouteExpectation>,
}

impl M5WorkspaceSlotConformance {
    /// Finds the slot expectation for a zone.
    pub fn slot(&self, zone: M5ShellZone) -> Option<&M5SlotExpectation> {
        self.slot_expectations.iter().find(|s| s.zone == zone)
    }
}

/// One slot a workspace claims, with the placeholder behavior the slot is expected to show.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SlotExpectation {
    /// The governed shell zone.
    pub zone: M5ShellZone,
    /// The governed slot id the workspace fills.
    pub slot_id: String,
    /// The contribution class that fills the slot.
    pub surface_kind: M5SurfaceKind,
    /// True when the workspace cannot render without the slot.
    pub required: bool,
    /// The placeholder behavior the slot is expected to show before content resolves.
    pub expected_placeholder_behavior: String,
}

/// One adaptive class's collapse expectation, resolved to the slot ids that collapse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollapseExpectation {
    /// The adaptive class.
    pub adaptive_class: M5AdaptiveClass,
    /// The slot ids expected to collapse at this class.
    pub collapsed_slot_ids: Vec<String>,
    /// Where the collapsed slots go.
    pub placement: M5FallbackPlacement,
    /// True when the collapse preserves each slot's identity and reopen route.
    pub preserves_zone_identity: bool,
}

/// One missing-dependency expectation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MissingDependencyExpectation {
    /// The missing dependency id.
    pub dependency_id: String,
    /// The slot id whose content the dependency feeds (`None` when the affected zone is not
    /// occupied).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_slot_id: Option<String>,
    /// The zone whose content the dependency feeds.
    pub affected_zone: M5ShellZone,
    /// Why the placeholder appears.
    pub placeholder_class: M5PlaceholderClass,
    /// The governed placeholder message id.
    pub placeholder_message_id: String,
}

/// One reopen / reset route expectation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReopenRouteExpectation {
    /// The route id.
    pub route_id: String,
    /// Whether the route reopens a surface or resets the workspace.
    pub route_kind: M5LayoutRouteKind,
    /// The governed command label message id.
    pub command_message_id: String,
    /// The default key chord.
    pub keys: String,
}

// ---------------------------------------------------------------------------
// Validation helpers.
// ---------------------------------------------------------------------------

fn validate_layout_set(
    package: &M5ReferenceLayoutPackage,
    violations: &mut Vec<M5ReferenceLayoutViolation>,
) {
    let present: BTreeSet<M5WorkspaceKind> =
        package.layouts.iter().map(|l| l.workspace_kind).collect();
    for required in M5WorkspaceKind::ALL {
        if !present.contains(&required) {
            violations.push(M5ReferenceLayoutViolation::RequiredWorkspaceKindMissing);
            break;
        }
    }
    if present.len() != package.layouts.len() {
        violations.push(M5ReferenceLayoutViolation::DuplicateWorkspaceKind);
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for layout in &package.layouts {
        if !seen_ids.insert(layout.workspace_id.as_str()) {
            violations.push(M5ReferenceLayoutViolation::DuplicateWorkspaceId);
        }
        validate_layout(layout, violations);
    }
}

fn validate_layout(
    layout: &M5WorkspaceReferenceLayout,
    violations: &mut Vec<M5ReferenceLayoutViolation>,
) {
    if layout.workspace_id.trim().is_empty()
        || layout.display_name.trim().is_empty()
        || layout.summary_message_id.trim().is_empty()
    {
        violations.push(M5ReferenceLayoutViolation::LayoutIncomplete);
    }
    if !layout
        .summary_message_id
        .starts_with(M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX)
    {
        violations.push(M5ReferenceLayoutViolation::MessageIdPrefixMissing);
    }

    validate_lifecycle(&layout.lifecycle, violations);
    let occupied = validate_zone_occupancy(layout, violations);
    validate_collapse_rules(layout, &occupied, violations);
    validate_missing_dependency_rules(layout, &occupied, violations);
    validate_reopen_routes(layout, violations);
}

fn validate_lifecycle(
    lifecycle: &M5WorkspaceLifecycle,
    violations: &mut Vec<M5ReferenceLayoutViolation>,
) {
    if lifecycle.owner_role.trim().is_empty()
        || lifecycle.layout_version == 0
        || !is_semver(&lifecycle.introduced_in_package_version)
    {
        violations.push(M5ReferenceLayoutViolation::LifecycleIncomplete);
    }
}

/// Validates zone occupancy and returns the set of occupied zones for downstream checks.
fn validate_zone_occupancy(
    layout: &M5WorkspaceReferenceLayout,
    violations: &mut Vec<M5ReferenceLayoutViolation>,
) -> BTreeSet<M5ShellZone> {
    let mut seen: BTreeSet<M5ShellZone> = BTreeSet::new();
    let mut duplicate = false;
    let mut incomplete = layout.zone_occupancy.is_empty();
    for occ in &layout.zone_occupancy {
        if !seen.insert(occ.zone) {
            duplicate = true;
        }
        if occ.slot_id.trim().is_empty()
            || occ.surface_role.trim().is_empty()
            || occ.placeholder_behavior.trim().is_empty()
        {
            incomplete = true;
        }
    }
    // Every workspace must claim the main work surface and mark it required, and must declare at
    // least one required zone.
    let main_required = layout
        .zone(M5ShellZone::MainWorkspace)
        .map(|o| o.required)
        .unwrap_or(false);
    let any_required = layout.zone_occupancy.iter().any(|o| o.required);
    if incomplete || duplicate || !main_required || !any_required {
        violations.push(M5ReferenceLayoutViolation::ZoneOccupancyIncomplete);
    }
    seen
}

fn validate_collapse_rules(
    layout: &M5WorkspaceReferenceLayout,
    occupied: &BTreeSet<M5ShellZone>,
    violations: &mut Vec<M5ReferenceLayoutViolation>,
) {
    let classes: BTreeSet<M5AdaptiveClass> = layout
        .responsive_collapse
        .iter()
        .map(|r| r.adaptive_class)
        .collect();
    let canonical: BTreeSet<M5AdaptiveClass> = M5AdaptiveClass::ALL.iter().copied().collect();
    let mut bad = classes != canonical || classes.len() != layout.responsive_collapse.len();
    for rule in &layout.responsive_collapse {
        if rule.behavior.trim().is_empty() {
            bad = true;
        }
        for zone in &rule.collapsed_zones {
            // A collapsed zone must be one the workspace occupies, and must never be a persistent
            // zone (the main work surface or status strip never collapse out of a workspace).
            if !occupied.contains(zone) || zone.is_persistent() {
                bad = true;
            }
        }
    }
    if bad {
        violations.push(M5ReferenceLayoutViolation::CollapseRulesIncomplete);
    }
}

fn validate_missing_dependency_rules(
    layout: &M5WorkspaceReferenceLayout,
    occupied: &BTreeSet<M5ShellZone>,
    violations: &mut Vec<M5ReferenceLayoutViolation>,
) {
    let mut bad = layout.missing_dependency_rules.is_empty();
    for rule in &layout.missing_dependency_rules {
        if rule.dependency_id.trim().is_empty()
            || rule.degraded_behavior.trim().is_empty()
            || !rule
                .placeholder_message_id
                .starts_with(M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX)
            || !occupied.contains(&rule.affected_zone)
        {
            bad = true;
        }
    }
    if bad {
        violations.push(M5ReferenceLayoutViolation::MissingDependencyRulesIncomplete);
    }
}

fn validate_reopen_routes(
    layout: &M5WorkspaceReferenceLayout,
    violations: &mut Vec<M5ReferenceLayoutViolation>,
) {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut bad = layout.reopen_routes.is_empty();
    for route in &layout.reopen_routes {
        if !seen.insert(route.route_id.as_str()) {
            bad = true;
        }
        if route.route_id.trim().is_empty()
            || route.keys.trim().is_empty()
            || route.description.trim().is_empty()
            || !route
                .command_message_id
                .starts_with(M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX)
        {
            bad = true;
        }
    }
    // A layout must offer both a reopen route and a reset route.
    let has_reopen = layout
        .reopen_routes
        .iter()
        .any(|r| r.route_kind == M5LayoutRouteKind::Reopen);
    let has_reset = layout
        .reopen_routes
        .iter()
        .any(|r| r.route_kind == M5LayoutRouteKind::Reset);
    if bad || !has_reopen || !has_reset {
        violations.push(M5ReferenceLayoutViolation::ReopenRoutesIncomplete);
    }
}

/// True when `value` is a `MAJOR.MINOR.PATCH` numeric semver.
fn is_semver(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Returns true when the JSON tree carries any forbidden raw-boundary material (credential bodies,
/// raw provider payloads). Reference layouts are metadata-only by construction; this is a
/// defense-in-depth scan over the serialized export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    const FORBIDDEN_KEYS: [&str; 6] = [
        "api_key",
        "authorization",
        "password",
        "secret",
        "access_token",
        "raw_payload",
    ];
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if FORBIDDEN_KEYS.contains(&key.to_lowercase().as_str()) {
                    return true;
                }
                if json_contains_forbidden_boundary_material(child) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
