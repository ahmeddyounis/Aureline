//! Binding the reversible presentation overlay onto the pane-and-navigation system.
//!
//! [`project_overlay_navigation_binding`] takes the canonical
//! [`PresentationSession`] (and its reversible [`PresentationOverlay`]
//! projection) plus a live [`ZoneRegistryLayout`] and produces a
//! [`PresentationOverlayNavigationBinding`]: an inspectable truth packet proving
//! that the presenter bar, agenda / waypoint rail, spotlight frame, zoom
//! presets, speaker-notes tray, audience strip, and breakaway banner are placed
//! **on top of the existing shell zones and navigation provenance**, never as a
//! second workspace shell.
//!
//! The binding makes the spec's contract checkable rather than asserted:
//!
//! - every overlay surface attaches to a canonical [`ShellZoneId`] and never
//!   replaces the underlying pane (the spotlight frame is a strict inset within
//!   the main workspace pane);
//! - the file / symbol / branch / workspace context and the local/remote/shared
//!   boundary label flow from the current waypoint's navigation anchor into the
//!   provenance strip and stay visible under the overlay chrome;
//! - every actionable surface is command-backed and keyboard reachable, carrying
//!   the stable command id and key-binding ref from the projected overlay;
//! - entering checkpoints the prior layout and exit / cancel / crash recovery all
//!   restore it, proven by replaying [`restore_from_checkpoint`].

use serde::{Deserialize, Serialize};

use crate::layout::presentation_overlays::{
    main_workspace_preserved, plan_presentation_overlays, PresentationOverlayPlacement,
    PresentationOverlaySurface,
};
use crate::layout::zone_registry::{Rect, ShellZoneId, ZoneRegistryLayout};
use crate::presentation_mode::{
    project_overlay, restore_from_checkpoint, AudienceScope, BoundaryLabel, KeyboardAction,
    LayoutPreset, LeaderFollowState, PresentationOverlay, PresentationSession, RestoreTrigger,
    WalkthroughSurfaceKind, PRESENTATION_MODE_BETA_SCHEMA_VERSION,
    PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`PresentationOverlayNavigationBinding`] payloads.
pub const PRESENTATION_OVERLAY_BINDING_RECORD_KIND: &str =
    "shell_presentation_overlay_navigation_binding_record";

/// Repo-relative path of the human-readable overlay/navigation contract doc.
pub const PRESENTATION_OVERLAYS_AND_NAVIGATION_DOC_REF: &str =
    "docs/ux/presentation-overlays-and-navigation.md";

/// Repo-relative directory holding the checked-in overlay/waypoint fixtures.
pub const PRESENTATION_OVERLAY_FIXTURE_DIR: &str = "fixtures/presentation/overlay-and-waypoint";

/// Serde mirror of [`ShellZoneId`] so a binding can name the host zone in an
/// export without leaking the geometry type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellZoneTag {
    /// Title / context bar.
    TitleContextBar,
    /// Primary activity rail.
    ActivityRail,
    /// Left sidebar.
    LeftSidebar,
    /// Main workspace pane.
    MainWorkspace,
    /// Right inspector.
    RightInspector,
    /// Bottom panel.
    BottomPanel,
    /// Status bar.
    StatusBar,
    /// Transient overlay zone (floats above the layout).
    TransientOverlay,
}

impl From<ShellZoneId> for ShellZoneTag {
    fn from(zone: ShellZoneId) -> Self {
        match zone {
            ShellZoneId::TitleContextBar => Self::TitleContextBar,
            ShellZoneId::ActivityRail => Self::ActivityRail,
            ShellZoneId::LeftSidebar => Self::LeftSidebar,
            ShellZoneId::MainWorkspace => Self::MainWorkspace,
            ShellZoneId::RightInspector => Self::RightInspector,
            ShellZoneId::BottomPanel => Self::BottomPanel,
            ShellZoneId::StatusBar => Self::StatusBar,
            ShellZoneId::TransientOverlay => Self::TransientOverlay,
        }
    }
}

impl ShellZoneTag {
    /// Stable token recorded in the binding.
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

/// Serde mirror of [`PresentationOverlaySurface`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlaySurfaceTag {
    /// Presenter bar.
    PresenterBar,
    /// Provenance strip.
    ProvenanceStrip,
    /// Agenda / waypoint rail.
    WaypointRail,
    /// Spotlight frame.
    SpotlightFrame,
    /// Speaker-notes tray.
    SpeakerNotesTray,
    /// Audience strip / follow chip.
    AudienceStrip,
    /// Breakaway banner.
    BreakawayBanner,
}

impl From<PresentationOverlaySurface> for OverlaySurfaceTag {
    fn from(surface: PresentationOverlaySurface) -> Self {
        match surface {
            PresentationOverlaySurface::PresenterBar => Self::PresenterBar,
            PresentationOverlaySurface::ProvenanceStrip => Self::ProvenanceStrip,
            PresentationOverlaySurface::WaypointRail => Self::WaypointRail,
            PresentationOverlaySurface::SpotlightFrame => Self::SpotlightFrame,
            PresentationOverlaySurface::SpeakerNotesTray => Self::SpeakerNotesTray,
            PresentationOverlaySurface::AudienceStrip => Self::AudienceStrip,
            PresentationOverlaySurface::BreakawayBanner => Self::BreakawayBanner,
        }
    }
}

impl OverlaySurfaceTag {
    /// Stable token recorded in the binding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PresenterBar => "presenter_bar",
            Self::ProvenanceStrip => "provenance_strip",
            Self::WaypointRail => "waypoint_rail",
            Self::SpotlightFrame => "spotlight_frame",
            Self::SpeakerNotesTray => "speaker_notes_tray",
            Self::AudienceStrip => "audience_strip",
            Self::BreakawayBanner => "breakaway_banner",
        }
    }

    /// A display-only surface (the provenance strip) carries no command; every
    /// other surface is actionable and must be command-backed.
    pub const fn is_actionable(self) -> bool {
        !matches!(self, Self::ProvenanceStrip)
    }
}

/// Serde mirror of [`Rect`] in window-local logical px.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlayRect {
    /// Left edge.
    pub x: u32,
    /// Top edge.
    pub y: u32,
    /// Width.
    pub width: u32,
    /// Height.
    pub height: u32,
}

impl From<Rect> for OverlayRect {
    fn from(rect: Rect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }
}

impl OverlayRect {
    /// Whether the rect has positive area.
    pub const fn is_non_empty(self) -> bool {
        self.width > 0 && self.height > 0
    }
}

/// One overlay surface bound to a host shell zone, with the command and
/// key-binding that reach it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlayPlacementBinding {
    /// The overlay surface placed.
    pub surface: OverlaySurfaceTag,
    /// The canonical shell zone the surface rides.
    pub host_zone: ShellZoneTag,
    /// The rect the surface occupies.
    pub rect: OverlayRect,
    /// True when the preferred host zone was collapsed and the surface floated.
    pub is_fallback_placement: bool,
    /// Always `false`: an overlay never replaces the underlying pane.
    pub replaces_underlying_pane: bool,
    /// Always `true`: the underlying pane stays visible beneath the overlay.
    pub underlying_pane_visible: bool,
    /// Whether this surface carries an action (false for the provenance strip).
    pub is_actionable: bool,
    /// Stable command id reaching this surface, present for actionable surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// Stable key-binding ref, present for actionable surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_binding_ref: Option<String>,
    /// Accessible label announced to assistive technology.
    pub accessible_label: String,
}

impl OverlayPlacementBinding {
    /// Whether an actionable surface carries a command id and key-binding ref,
    /// and a display-only surface carries neither but stays announced.
    pub fn command_and_keyboard_ok(&self) -> bool {
        if self.accessible_label.trim().is_empty() {
            return false;
        }
        if self.is_actionable {
            self.command_id
                .as_ref()
                .is_some_and(|c| !c.trim().is_empty())
                && self
                    .key_binding_ref
                    .as_ref()
                    .is_some_and(|k| !k.trim().is_empty())
        } else {
            self.command_id.is_none() && self.key_binding_ref.is_none()
        }
    }

    /// Whether the placement keeps the underlying pane (never a replacement).
    pub const fn pane_preserved(&self) -> bool {
        !self.replaces_underlying_pane && self.underlying_pane_visible
    }
}

/// The navigation provenance bound under the overlay: file / symbol / branch /
/// workspace context and the boundary label, all kept visible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationProvenanceBinding {
    /// The waypoint that currently holds focus, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_waypoint_ref: Option<String>,
    /// The kind of existing surface the current anchor lives on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_kind: Option<WalkthroughSurfaceKind>,
    /// File path of the current anchor (kept, not erased).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_path_ref: Option<String>,
    /// Symbol anchor within the file, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_anchor_ref: Option<String>,
    /// Branch / workspace context for the anchor.
    pub branch_workspace_ref: String,
    /// Local / remote / shared boundary label.
    pub boundary_label: BoundaryLabel,
    /// Always `true`: provenance stays visible under the overlay chrome.
    pub provenance_visible_under_overlay: bool,
    /// Always `true`: the anchor reuses an existing navigation surface.
    pub reuses_existing_surface: bool,
}

impl NavigationProvenanceBinding {
    /// Whether the provenance binding is honest: it stays visible, reuses an
    /// existing surface, and carries branch/workspace context.
    pub fn is_preserved(&self) -> bool {
        self.provenance_visible_under_overlay
            && self.reuses_existing_surface
            && !self.branch_workspace_ref.trim().is_empty()
    }
}

/// The layout checkpoint bound on enter and replayed on exit / cancel / crash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutCheckpointBinding {
    /// Id of the checkpoint captured before the overlay attached.
    pub checkpoint_ref: String,
    /// Prior window-topology snapshot ref.
    pub prior_layout_ref: String,
    /// Prior focus-chain ref.
    pub prior_focus_ref: String,
    /// Always `true`: entering presentation checkpoints the prior layout first.
    pub enter_checkpoints_prior_layout: bool,
    /// True when exit, cancel, and crash recovery all restore the checkpoint.
    pub restores_under_all_triggers: bool,
    /// Always `true`: restore never strands the user in an improvised shell.
    pub no_improvised_shell_on_restore: bool,
}

impl LayoutCheckpointBinding {
    /// Whether the checkpoint binding holds for a reversible overlay.
    pub fn restore_holds(&self) -> bool {
        self.enter_checkpoints_prior_layout
            && self.restores_under_all_triggers
            && self.no_improvised_shell_on_restore
            && !self.checkpoint_ref.trim().is_empty()
            && !self.prior_layout_ref.trim().is_empty()
            && !self.prior_focus_ref.trim().is_empty()
    }
}

/// An inspectable truth packet binding one presentation overlay onto the live
/// pane-and-navigation system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationOverlayNavigationBinding {
    /// Record kind; must equal [`PRESENTATION_OVERLAY_BINDING_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version shared with presentation-mode beta records.
    pub schema_version: u32,
    /// Shared contract ref for cross-surface pivoting.
    pub shared_contract_ref: String,
    /// The session this binding projects.
    pub session_id: String,
    /// The local user's leader / follow posture.
    pub leader_follow_state: LeaderFollowState,
    /// The session's layout preset.
    pub layout_preset: LayoutPreset,
    /// The session's audience scope.
    pub audience_scope: AudienceScope,
    /// The responsive adaptive class of the host window.
    pub adaptive_class: String,
    /// The host window bounds.
    pub window: OverlayRect,
    /// Per-surface placement bindings.
    pub placements: Vec<OverlayPlacementBinding>,
    /// The navigation provenance bound under the overlay.
    pub provenance: NavigationProvenanceBinding,
    /// The layout checkpoint / restore binding.
    pub checkpoint: LayoutCheckpointBinding,
    /// Always `true`: every overlay control is keyboard reachable.
    pub keyboard_complete: bool,
    /// Always `false`: nothing is pointer-only.
    pub pointer_only: bool,
    /// Always `true`: every surface is reachable by a screen reader.
    pub screen_reader_reachable: bool,
    /// True when the overlay sits on the existing panes without replacing them.
    pub preserves_pane_and_navigation_system: bool,
    /// Always `true`: presentation is a thin overlay, not a second shell.
    pub thin_overlay_not_second_shell: bool,
    /// Always `false`: presentation opens no mutation shortcut.
    pub grants_mutation_authority: bool,
    /// Always `false`: following is not control.
    pub grants_control_authority: bool,
}

impl PresentationOverlayNavigationBinding {
    /// A placement by its surface tag.
    pub fn placement(&self, surface: OverlaySurfaceTag) -> Option<&OverlayPlacementBinding> {
        self.placements.iter().find(|p| p.surface == surface)
    }

    /// Validate the overlay/navigation binding invariants.
    pub fn validate(&self) -> Vec<PresentationBindingViolation> {
        use PresentationBindingViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != PRESENTATION_OVERLAY_BINDING_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.session_id.trim().is_empty() {
            violations.push(V::MissingIdentity);
        }
        if self.placements.is_empty() {
            violations.push(V::NoPlacements);
        }

        // A presenter bar, waypoint rail, and provenance strip are always part of
        // a presentation overlay; their absence means the overlay was not bound.
        for required in [
            OverlaySurfaceTag::PresenterBar,
            OverlaySurfaceTag::WaypointRail,
            OverlaySurfaceTag::ProvenanceStrip,
        ] {
            if self.placement(required).is_none() {
                violations.push(V::RequiredSurfaceMissing);
            }
        }

        for placement in &self.placements {
            if !placement.pane_preserved() {
                violations.push(V::PlacementReplacesPane);
            }
            if !placement.command_and_keyboard_ok() {
                violations.push(V::PlacementNotCommandBacked);
            }
            if !placement.rect.is_non_empty() {
                violations.push(V::PlacementRectEmpty);
            }
            // The spotlight frame is the only surface allowed to ride the main
            // workspace, and only as an inset (never the full pane rect).
            if placement.host_zone == ShellZoneTag::MainWorkspace {
                let is_inset_spotlight = placement.surface == OverlaySurfaceTag::SpotlightFrame
                    && OverlayRect::from(Rect::new(
                        self.window.x,
                        self.window.y,
                        self.window.width,
                        self.window.height,
                    )) != placement.rect;
                if !is_inset_spotlight {
                    violations.push(V::PlacementCoversMainWorkspace);
                }
            }
        }

        if !self.provenance.is_preserved() {
            violations.push(V::ProvenanceNotPreserved);
        }
        if !self.checkpoint.restore_holds() {
            violations.push(V::CheckpointDoesNotRestore);
        }
        if !self.keyboard_complete || self.pointer_only || !self.screen_reader_reachable {
            violations.push(V::AccessibilityIncomplete);
        }
        if !self.preserves_pane_and_navigation_system || !self.thin_overlay_not_second_shell {
            violations.push(V::NotAThinOverlay);
        }
        if self.grants_mutation_authority || self.grants_control_authority {
            violations.push(V::AuthorityWidened);
        }

        violations
    }
}

/// Validation failures emitted by [`PresentationOverlayNavigationBinding::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationBindingViolation {
    /// Wrong record kind.
    WrongRecordKind,
    /// Wrong schema version.
    WrongSchemaVersion,
    /// Missing session identity.
    MissingIdentity,
    /// No overlay placements were bound.
    NoPlacements,
    /// A surface that must always be present is missing.
    RequiredSurfaceMissing,
    /// A placement replaces or hides the underlying pane.
    PlacementReplacesPane,
    /// An actionable placement is not command-backed and keyboard reachable.
    PlacementNotCommandBacked,
    /// A placement rect has no area.
    PlacementRectEmpty,
    /// A placement covers the whole main workspace instead of insetting.
    PlacementCoversMainWorkspace,
    /// Source provenance is not preserved under the overlay.
    ProvenanceNotPreserved,
    /// The layout checkpoint does not restore under all triggers.
    CheckpointDoesNotRestore,
    /// The overlay is not fully keyboard / screen-reader reachable.
    AccessibilityIncomplete,
    /// The overlay is not a thin layer over the existing panes.
    NotAThinOverlay,
    /// The binding widens mutation or control authority.
    AuthorityWidened,
}

impl PresentationBindingViolation {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::NoPlacements => "no_placements",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::PlacementReplacesPane => "placement_replaces_pane",
            Self::PlacementNotCommandBacked => "placement_not_command_backed",
            Self::PlacementRectEmpty => "placement_rect_empty",
            Self::PlacementCoversMainWorkspace => "placement_covers_main_workspace",
            Self::ProvenanceNotPreserved => "provenance_not_preserved",
            Self::CheckpointDoesNotRestore => "checkpoint_does_not_restore",
            Self::AccessibilityIncomplete => "accessibility_incomplete",
            Self::NotAThinOverlay => "not_a_thin_overlay",
            Self::AuthorityWidened => "authority_widened",
        }
    }
}

/// Project the overlay/navigation binding for `session` over `layout`.
pub fn project_overlay_navigation_binding(
    session: &PresentationSession,
    layout: &ZoneRegistryLayout,
) -> PresentationOverlayNavigationBinding {
    let overlay = project_overlay(session);
    let active = active_surfaces(&overlay);
    let placements = plan_presentation_overlays(layout, &active);
    let placement_bindings = placements
        .iter()
        .map(|placement| bind_placement(*placement, &overlay))
        .collect();

    let provenance = bind_provenance(session, &overlay);
    let checkpoint = bind_checkpoint(session);
    let preserves = main_workspace_preserved(layout, &placements)
        && placements.iter().all(|p| p.within_window(layout))
        && provenance.is_preserved();

    PresentationOverlayNavigationBinding {
        record_kind: PRESENTATION_OVERLAY_BINDING_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session.session_id.clone(),
        leader_follow_state: session.leader_follow_state,
        layout_preset: session.layout_preset,
        audience_scope: session.audience_scope,
        adaptive_class: layout.adaptive_class.name().to_owned(),
        window: layout.window.into(),
        placements: placement_bindings,
        provenance,
        checkpoint,
        keyboard_complete: overlay.keyboard_complete,
        pointer_only: overlay.pointer_only,
        screen_reader_reachable: overlay.screen_reader_reachable,
        preserves_pane_and_navigation_system: preserves,
        thin_overlay_not_second_shell: true,
        grants_mutation_authority: session.grants_mutation_authority,
        grants_control_authority: session.grants_control_authority,
    }
}

fn active_surfaces(overlay: &PresentationOverlay) -> Vec<PresentationOverlaySurface> {
    let mut active = vec![
        PresentationOverlaySurface::PresenterBar,
        PresentationOverlaySurface::ProvenanceStrip,
        PresentationOverlaySurface::WaypointRail,
    ];
    if overlay.spotlight_frame.enabled {
        active.push(PresentationOverlaySurface::SpotlightFrame);
    }
    active.push(PresentationOverlaySurface::SpeakerNotesTray);
    active.push(PresentationOverlaySurface::AudienceStrip);
    if overlay.breakaway_banner.is_some() {
        active.push(PresentationOverlaySurface::BreakawayBanner);
    }
    active
}

fn bind_placement(
    placement: PresentationOverlayPlacement,
    overlay: &PresentationOverlay,
) -> OverlayPlacementBinding {
    let surface_tag: OverlaySurfaceTag = placement.surface.into();
    let action = representative_action(placement.surface, overlay);
    let (command_id, key_binding_ref, accessible_label) = match action {
        Some(action) => (
            Some(action.command_id.clone()),
            Some(action.key_binding_ref.clone()),
            action.accessible_label.clone(),
        ),
        None => (None, None, provenance_label(overlay)),
    };
    OverlayPlacementBinding {
        surface: surface_tag,
        host_zone: placement.host_zone.into(),
        rect: placement.rect.into(),
        is_fallback_placement: placement.is_fallback_placement,
        replaces_underlying_pane: placement.replaces_underlying_pane(),
        underlying_pane_visible: placement.underlying_pane_visible(),
        is_actionable: surface_tag.is_actionable(),
        command_id,
        key_binding_ref,
        accessible_label,
    }
}

/// Pick the representative command-backed action for a surface from the overlay.
/// The provenance strip is display-only and returns `None`.
fn representative_action<'a>(
    surface: PresentationOverlaySurface,
    overlay: &'a PresentationOverlay,
) -> Option<&'a KeyboardAction> {
    use PresentationOverlaySurface as S;
    match surface {
        // The presenter bar hosts the zoom presets; surface the zoom cycle as
        // its representative action.
        S::PresenterBar => overlay
            .presenter_bar
            .actions
            .iter()
            .find(|a| a.command_id == "cmd:presentation.cycle_zoom_preset")
            .or_else(|| overlay.presenter_bar.actions.first()),
        S::WaypointRail => overlay
            .waypoint_rail
            .actions
            .iter()
            .find(|a| a.command_id == "cmd:presentation.next_waypoint")
            .or_else(|| overlay.waypoint_rail.actions.first()),
        S::SpotlightFrame => Some(&overlay.spotlight_frame.clear_spotlight_action),
        S::SpeakerNotesTray => overlay.speaker_notes_tray.actions.first(),
        S::AudienceStrip => overlay
            .audience_strip
            .follow_chip
            .actions
            .first()
            .or_else(|| overlay.audience_strip.actions.first()),
        S::BreakawayBanner => overlay
            .breakaway_banner
            .as_ref()
            .map(|b| &b.return_to_presenter_action),
        S::ProvenanceStrip => None,
    }
}

fn provenance_label(overlay: &PresentationOverlay) -> String {
    let strip = &overlay.provenance_strip;
    let mut parts = Vec::new();
    if let Some(path) = &strip.file_path_ref {
        parts.push(path.clone());
    }
    if let Some(symbol) = &strip.symbol_anchor_ref {
        parts.push(symbol.clone());
    }
    parts.push(strip.branch_workspace_ref.clone());
    parts.push(format!("boundary: {}", strip.boundary_label.as_str()));
    format!("Source provenance — {}", parts.join(" · "))
}

fn bind_provenance(
    session: &PresentationSession,
    overlay: &PresentationOverlay,
) -> NavigationProvenanceBinding {
    let current = session
        .current_focus_waypoint_ref
        .as_ref()
        .and_then(|r| session.waypoints.iter().find(|w| &w.waypoint_id == r))
        .or_else(|| session.waypoints.first());
    let strip = &overlay.provenance_strip;
    NavigationProvenanceBinding {
        current_waypoint_ref: current.map(|w| w.waypoint_id.clone()),
        surface_kind: current.map(|w| w.surface_kind),
        file_path_ref: strip.file_path_ref.clone(),
        symbol_anchor_ref: strip.symbol_anchor_ref.clone(),
        branch_workspace_ref: strip.branch_workspace_ref.clone(),
        boundary_label: strip.boundary_label,
        provenance_visible_under_overlay: strip.source_identity_preserved,
        reuses_existing_surface: current.map(|w| w.reuses_existing_surface).unwrap_or(true),
    }
}

fn bind_checkpoint(session: &PresentationSession) -> LayoutCheckpointBinding {
    let cp = &session.restore_checkpoint;
    let restores_under_all_triggers = [
        RestoreTrigger::Exit,
        RestoreTrigger::Cancel,
        RestoreTrigger::CrashRecovery,
    ]
    .into_iter()
    .all(|trigger| {
        let outcome = restore_from_checkpoint(session, trigger);
        outcome.matches_checkpoint
            && !outcome.left_in_improvised_shell
            && outcome.restored_layout_ref == cp.prior_layout_ref
            && outcome.restored_focus_ref == cp.prior_focus_ref
    });
    LayoutCheckpointBinding {
        checkpoint_ref: cp.checkpoint_id.clone(),
        prior_layout_ref: cp.prior_layout_ref.clone(),
        prior_focus_ref: cp.prior_focus_ref.clone(),
        enter_checkpoints_prior_layout: true,
        restores_under_all_triggers,
        no_improvised_shell_on_restore: true,
    }
}
