//! Typed record model for declared shell slots and placeholder-preserving fallback.

use serde::{Deserialize, Serialize};

/// Schema version for shell-zoning stabilization packets.
pub const SHELL_ZONING_SCHEMA_VERSION: u32 = 1;

/// Record-kind tag for the packet emitted by this lane.
pub const SHELL_ZONING_PACKET_RECORD_KIND: &str = "shell_zoning_responsive_fallback_packet";

/// Canonical shell zones that may host stable shell occupants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoneId {
    /// Workspace, trust, target, profile, and route identity.
    TitleContextBar,
    /// Durable top-level route rail.
    ActivityRail,
    /// Structural navigation and query collections.
    LeftSidebar,
    /// Editor groups, review surfaces, and primary working sets.
    MainWorkspace,
    /// Contextual detail and inspectable evidence.
    RightInspector,
    /// Execution, output, problems, terminal, and timeline panels.
    BottomPanel,
    /// Persistent instrumentation and compact recovery/status truth.
    StatusBar,
    /// Window-local command palettes, dialogs, sheets, and overlays.
    TransientOverlay,
}

impl ZoneId {
    /// Returns the stable shell-zone id used by schemas and fixtures.
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
}

/// Declared slot ids exported by the stable shell registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellSlotId {
    /// Title/context identity slot.
    TitleContextBarIdentity,
    /// Activity rail route slot.
    ActivityRailPrimaryRoutes,
    /// Left sidebar section host.
    LeftSidebarSectionSurface,
    /// Main workspace editor/workset host.
    MainWorkspaceWorkingSet,
    /// Main workspace review/approval host.
    MainWorkspaceReviewSurface,
    /// Right inspector contextual detail host.
    RightInspectorContextualDetail,
    /// Bottom-panel tool panel host.
    BottomPanelToolPanels,
    /// Status-bar protected recovery/status host.
    StatusBarRecoveryPrimary,
    /// Status-bar scoped extension item host.
    StatusBarExtensionScoped,
    /// Command-palette overlay host.
    TransientOverlayCommandPalette,
    /// Dialog and sheet overlay host.
    TransientOverlayDialogOrSheet,
}

impl ShellSlotId {
    /// Returns the stable slot id used by schemas and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TitleContextBarIdentity => "slot.title_context_bar.identity",
            Self::ActivityRailPrimaryRoutes => "slot.activity_rail.primary_routes",
            Self::LeftSidebarSectionSurface => "slot.sidebar.section_surface",
            Self::MainWorkspaceWorkingSet => "slot.main_workspace.working_set",
            Self::MainWorkspaceReviewSurface => "slot.main_workspace.review_surface",
            Self::RightInspectorContextualDetail => "slot.right_inspector.contextual_detail",
            Self::BottomPanelToolPanels => "slot.bottom_panel.tool_panels",
            Self::StatusBarRecoveryPrimary => "status.slot.recovery.primary",
            Self::StatusBarExtensionScoped => "status.slot.extension.scoped",
            Self::TransientOverlayCommandPalette => "slot.overlay.command_palette",
            Self::TransientOverlayDialogOrSheet => "slot.overlay.dialog_or_sheet",
        }
    }

    /// Returns the owning zone for this slot.
    pub const fn zone(self) -> ZoneId {
        match self {
            Self::TitleContextBarIdentity => ZoneId::TitleContextBar,
            Self::ActivityRailPrimaryRoutes => ZoneId::ActivityRail,
            Self::LeftSidebarSectionSurface => ZoneId::LeftSidebar,
            Self::MainWorkspaceWorkingSet | Self::MainWorkspaceReviewSurface => {
                ZoneId::MainWorkspace
            }
            Self::RightInspectorContextualDetail => ZoneId::RightInspector,
            Self::BottomPanelToolPanels => ZoneId::BottomPanel,
            Self::StatusBarRecoveryPrimary | Self::StatusBarExtensionScoped => ZoneId::StatusBar,
            Self::TransientOverlayCommandPalette | Self::TransientOverlayDialogOrSheet => {
                ZoneId::TransientOverlay
            }
        }
    }
}

/// Surface contribution classes accepted by stable shell slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// Required host chrome owned by the shell.
    HostChrome,
    /// First-party core product surface.
    FirstParty,
    /// Provider-backed surface admitted through a first-party contract.
    ProviderBacked,
    /// Extension-contributed surface admitted through an explicit slot row.
    ExtensionContributed,
}

impl SurfaceKind {
    /// Returns the stable surface-kind id.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostChrome => "host_chrome",
            Self::FirstParty => "first_party",
            Self::ProviderBacked => "provider_backed",
            Self::ExtensionContributed => "extension_contributed",
        }
    }
}

/// Responsive class resolved before applying a fallback ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveClass {
    /// Compact desktop width.
    CompactDesktop,
    /// Standard desktop width.
    StandardDesktop,
    /// Expanded desktop width.
    ExpandedDesktop,
}

impl AdaptiveClass {
    /// Returns the stable adaptive-class id.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactDesktop => "compact_desktop",
            Self::StandardDesktop => "standard_desktop",
            Self::ExpandedDesktop => "expanded_desktop",
        }
    }
}

/// Where a slot occupant may move during responsive fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackPlacement {
    /// The surface remains docked in its declared zone.
    Docked,
    /// The surface opens as a sheet attached to its declared zone.
    Sheet,
    /// The surface is reachable through a keyboard-accessible overflow route.
    Overflow,
    /// The surface becomes an in-slot placeholder preserving identity.
    Placeholder,
}

impl FallbackPlacement {
    /// Returns the stable placement id.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docked => "docked",
            Self::Sheet => "sheet",
            Self::Overflow => "overflow",
            Self::Placeholder => "placeholder",
        }
    }
}

/// Placeholder class used when hydration cannot rebind a live surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderClass {
    /// Required shell chrome is not closable and has no dependency placeholder.
    RequiredChrome,
    /// Extension surface is missing, disabled, or no longer admitted.
    MissingExtension,
    /// Remote/session dependency must be reconnected or reauthorized.
    MissingRemote,
    /// Provider-backed detail is unavailable.
    MissingProvider,
    /// The needed capability is no longer granted or supported.
    CapabilityLoss,
    /// Window/display topology changed and the surface is safely recentered.
    TopologyDrift,
}

impl PlaceholderClass {
    /// Returns the stable placeholder-class id.
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

/// Dependency-loss condition that can replace a live surface with a placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyLossClass {
    /// Warm restore can hydrate structure without live authority.
    WarmRestorePendingRebind,
    /// Extension contribution is missing after restore or update.
    ExtensionRemoved,
    /// Remote target/session is unavailable.
    RemoteUnavailable,
    /// Provider capability was revoked or narrowed.
    ProviderCapabilityLost,
    /// Monitor, DPI, or window topology drifted.
    DisplayTopologyChanged,
}

impl DependencyLossClass {
    /// Returns the stable dependency-loss id.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarmRestorePendingRebind => "warm_restore_pending_rebind",
            Self::ExtensionRemoved => "extension_removed",
            Self::RemoteUnavailable => "remote_unavailable",
            Self::ProviderCapabilityLost => "provider_capability_lost",
            Self::DisplayTopologyChanged => "display_topology_changed",
        }
    }
}

/// Registry row for one declared shell slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeclaredSlotRecord {
    /// Slot id.
    pub slot_id: ShellSlotId,
    /// Shell zone that owns the slot.
    pub shell_zone: ZoneId,
    /// Review-safe label for evidence packets.
    pub slot_label: String,
    /// Surface classes permitted in this slot.
    pub permitted_surface_kinds: Vec<SurfaceKind>,
    /// Ordered fallback placements used under responsive pressure or loss.
    pub fallback_order: Vec<FallbackPlacement>,
    /// Canonical command that closes, hides, or dismisses the slot occupant.
    pub close_command_id: String,
    /// Canonical command that reopens the slot occupant.
    pub reopen_command_id: String,
    /// Placeholder class rendered when the occupant cannot hydrate.
    pub placeholder_class: PlaceholderClass,
    /// Whether the slot is stable-required for claimed stable rows.
    pub stable_required: bool,
    /// Owning contract or schema reference.
    pub owner_ref: String,
    /// Whether top-level private chrome is forbidden for occupants.
    pub private_top_level_chrome_forbidden: bool,
}

impl DeclaredSlotRecord {
    /// Returns true when the row is internally consistent with the canonical slot map.
    pub fn validates(&self) -> bool {
        self.shell_zone == self.slot_id.zone()
            && !self.permitted_surface_kinds.is_empty()
            && !self.fallback_order.is_empty()
            && self
                .fallback_order
                .contains(&FallbackPlacement::Placeholder)
            && self.close_command_id.starts_with("cmd:")
            && self.reopen_command_id.starts_with("cmd:")
            && self.private_top_level_chrome_forbidden
    }
}

/// Responsive fallback ladder for one slot and adaptive class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveFallbackLadder {
    /// Slot governed by this ladder.
    pub slot_id: ShellSlotId,
    /// Adaptive class the ladder applies to.
    pub adaptive_class: AdaptiveClass,
    /// Minimum editor width protected before this slot collapses further.
    pub minimum_editor_width_px: u32,
    /// Ordered placements for this class.
    pub placements_in_order: Vec<FallbackPlacement>,
    /// Whether the slot id is preserved through every placement.
    pub preserves_slot_identity: bool,
    /// Whether breadcrumb truth remains visible or recoverable.
    pub preserves_breadcrumb_truth: bool,
    /// Whether trust truth remains visible or recoverable.
    pub preserves_trust_truth: bool,
    /// Whether execution-target truth remains visible or recoverable.
    pub preserves_execution_target_truth: bool,
    /// Whether overflow or sheet recovery is keyboard reachable.
    pub keyboard_reachable_recovery: bool,
}

impl ResponsiveFallbackLadder {
    /// Returns true when the ladder preserves stable shell truth.
    pub fn protects_truth_cues(&self) -> bool {
        self.minimum_editor_width_px >= 420
            && !self.placements_in_order.is_empty()
            && self.preserves_slot_identity
            && self.preserves_breadcrumb_truth
            && self.preserves_trust_truth
            && self.preserves_execution_target_truth
            && self.keyboard_reachable_recovery
    }
}

/// Claimed stable surface and its admitted slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSurfaceClaim {
    /// Stable surface id.
    pub surface_id: String,
    /// Owning subsystem.
    pub owned_by: String,
    /// Slot occupied by the surface.
    pub declared_slot_id: ShellSlotId,
    /// Contribution class.
    pub surface_kind: SurfaceKind,
    /// Whether the surface is claimed stable.
    pub stable_claim: bool,
    /// Whether the surface attempts private top-level chrome.
    pub has_private_top_level_chrome: bool,
    /// Whether the surface creates a duplicate sidebar.
    pub creates_duplicate_sidebar: bool,
    /// Whether the surface creates a floating global button.
    pub creates_floating_global_button: bool,
    /// Review-safe reason diagnostics/support can report.
    pub occupancy_reason: String,
}

impl StableSurfaceClaim {
    /// Returns true when the stable surface obeys shell-slot admission.
    pub fn is_admitted_by(&self, registry: &[DeclaredSlotRecord]) -> bool {
        let Some(slot) = registry
            .iter()
            .find(|slot| slot.slot_id == self.declared_slot_id)
        else {
            return false;
        };

        (!self.stable_claim || slot.stable_required)
            && slot.permitted_surface_kinds.contains(&self.surface_kind)
            && !self.has_private_top_level_chrome
            && !self.creates_duplicate_sidebar
            && !self.creates_floating_global_button
    }
}

/// Fixture row proving placeholder-in-place hydration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotHydrationCase {
    /// Scenario id.
    pub scenario_id: String,
    /// Surface that could not hydrate as live.
    pub surface_id: String,
    /// Declared slot retained by the placeholder.
    pub slot_id: ShellSlotId,
    /// Declared zone retained by the placeholder.
    pub shell_zone: ZoneId,
    /// Dependency-loss condition.
    pub dependency_loss: DependencyLossClass,
    /// Placeholder class rendered in the same slot.
    pub placeholder_class: PlaceholderClass,
    /// Breadcrumb route or target truth is preserved.
    pub preserves_breadcrumb_truth: bool,
    /// Trust, policy, or authority truth is preserved.
    pub preserves_trust_truth: bool,
    /// Target identity remains inspectable.
    pub preserves_target_truth: bool,
    /// Reopen command path remains exact.
    pub preserves_reopen_path: bool,
    /// Adjacent layout did not collapse silently.
    pub adjacent_layout_collapsed: bool,
    /// Multi-window or display topology identity remains honest.
    pub topology_identity_preserved: bool,
}

impl SlotHydrationCase {
    /// Returns true when hydration preserves in-slot truth.
    pub fn preserves_layout_truth(&self) -> bool {
        self.shell_zone == self.slot_id.zone()
            && self.preserves_breadcrumb_truth
            && self.preserves_trust_truth
            && self.preserves_target_truth
            && self.preserves_reopen_path
            && !self.adjacent_layout_collapsed
            && self.topology_identity_preserved
    }
}

/// Audit result consumed by diagnostics and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellZoningAuditRecord {
    /// Every stable surface occupies a declared slot.
    pub every_stable_surface_declared: bool,
    /// Every slot row validates against the canonical zone map.
    pub every_slot_valid: bool,
    /// No claimed stable surface uses private top-level chrome.
    pub no_private_top_level_chrome: bool,
    /// No claimed stable surface creates duplicate sidebars.
    pub no_duplicate_sidebars: bool,
    /// No claimed stable surface creates floating global buttons.
    pub no_floating_global_buttons: bool,
    /// Every responsive ladder preserves stable identity cues.
    pub responsive_ladders_preserve_truth: bool,
    /// Every loss fixture renders a placeholder in the declared slot.
    pub placeholders_preserve_declared_slot: bool,
}

impl ShellZoningAuditRecord {
    /// Returns true when the packet satisfies stable shell-zoning acceptance.
    pub fn passes(&self) -> bool {
        self.every_stable_surface_declared
            && self.every_slot_valid
            && self.no_private_top_level_chrome
            && self.no_duplicate_sidebars
            && self.no_floating_global_buttons
            && self.responsive_ladders_preserve_truth
            && self.placeholders_preserve_declared_slot
    }
}

/// Canonical packet emitted for docs, diagnostics, support, and fixture replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellZoningPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub shell_zoning_schema_version: u32,
    /// Shared contract reference.
    pub contract_ref: String,
    /// Declared shell-slot registry.
    pub declared_slots: Vec<DeclaredSlotRecord>,
    /// Responsive fallback ladders.
    pub responsive_ladders: Vec<ResponsiveFallbackLadder>,
    /// Claimed stable surface admissions.
    pub stable_surface_claims: Vec<StableSurfaceClaim>,
    /// Placeholder-preserving hydration cases.
    pub placeholder_hydration_cases: Vec<SlotHydrationCase>,
    /// Computed conformance audit.
    pub audit: ShellZoningAuditRecord,
}
