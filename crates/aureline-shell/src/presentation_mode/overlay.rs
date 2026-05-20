//! The reversible overlay projection over a [`PresentationSession`].
//!
//! [`project_overlay`] turns one session into a [`PresentationOverlay`]: the six
//! presentation surfaces from the design system — presenter bar, agenda /
//! waypoint rail, spotlight frame, speaker-notes tray, audience strip / follow
//! chip, and breakaway banner — plus a provenance strip and a restore
//! affordance that keep the layer thin and reversible.
//!
//! Two guarantees are baked into the projection rather than left to the caller:
//!
//! 1. **Keyboard-complete, never pointer-only.** Every actionable control is a
//!    [`KeyboardAction`] carrying a stable command id, a key-binding ref, and an
//!    accessible label. The overlay reports `keyboard_complete = true`,
//!    `pointer_only = false`, and `screen_reader_reachable = true`, and the
//!    validation corpus checks every action backs those claims.
//! 2. **No hidden authority.** Presentation actions guide attention only; none
//!    is destructive and none mutates the workspace. Mutation-capable teaching
//!    still flows through the ordinary command graph, never through a
//!    presentation shortcut.

use serde::{Deserialize, Serialize};

use super::session::{
    AudienceParticipant, BoundaryLabel, FollowWaypoint, LeaderFollowState, PresentationSession,
    SpeakerNoteScope, WalkthroughSurfaceKind, WaypointCompletionState,
};

/// Stable record kind for [`PresentationOverlay`] payloads.
pub const PRESENTATION_OVERLAY_RECORD_KIND: &str = "presentation_overlay_record";

/// Zoom preset shown on the presenter bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoomPreset {
    /// Fit the current surface to the viewport.
    Fit,
    /// The presenter's standard zoom.
    Standard,
    /// A magnified zoom for audience legibility.
    Magnified,
}

impl ZoomPreset {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fit => "fit",
            Self::Standard => "standard",
            Self::Magnified => "magnified",
        }
    }
}

/// A single keyboard-reachable control on a presentation surface.
///
/// Presentation actions guide attention; they never mutate the workspace and
/// are never destructive. Mutation-capable teaching uses the ordinary command
/// graph instead, so `mutates_workspace` and `is_destructive` are always
/// `false` here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardAction {
    pub action_id: String,
    /// Short visible label.
    pub label: String,
    /// Stable command id this action invokes.
    pub command_id: String,
    /// Stable key-binding id so the action is reachable without a pointer.
    pub key_binding_ref: String,
    /// Accessible name announced to assistive technology.
    pub accessible_label: String,
    /// Always `false`: presentation controls never destroy data.
    pub is_destructive: bool,
    /// Always `false`: presentation controls never mutate the workspace.
    pub mutates_workspace: bool,
}

impl KeyboardAction {
    /// Build an attention-only keyboard action with the safety flags fixed.
    pub fn new(
        action_id: impl Into<String>,
        label: impl Into<String>,
        command_id: impl Into<String>,
        key_binding_ref: impl Into<String>,
        accessible_label: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            label: label.into(),
            command_id: command_id.into(),
            key_binding_ref: key_binding_ref.into(),
            accessible_label: accessible_label.into(),
            is_destructive: false,
            mutates_workspace: false,
        }
    }
}

/// Presenter bar: switch into guided mode without losing workspace context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresenterBar {
    /// Current file/workspace or tour-step title (provenance, not chrome).
    pub context_title: String,
    pub zoom_preset: ZoomPreset,
    pub spotlight_enabled: bool,
    pub leader_follow_state: LeaderFollowState,
    pub notes_tray_open: bool,
    pub actions: Vec<KeyboardAction>,
}

/// One row in the agenda / waypoint rail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaypointRailRow {
    pub waypoint_ref: String,
    pub ordinal: u32,
    pub step_title: String,
    pub surface_kind: WalkthroughSurfaceKind,
    pub completion_state: WaypointCompletionState,
    pub is_current: bool,
    pub jump_action: KeyboardAction,
}

/// Agenda / waypoint rail: step through prepared anchors in a predictable order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaypointRail {
    pub rows: Vec<WaypointRailRow>,
    /// Whether reorder / edit affordances are offered (presenter only).
    pub reorder_allowed: bool,
    pub actions: Vec<KeyboardAction>,
}

/// Spotlight frame: direct attention without hiding the rest of the layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpotlightFrame {
    pub enabled: bool,
    pub highlighted_region_ref: Option<String>,
    pub dimmed_surroundings: bool,
    /// Accessible textual label for the highlighted region.
    pub accessible_region_label: Option<String>,
    /// Always `true`: the spotlight preserves keyboard order and focus.
    pub preserves_keyboard_order: bool,
    /// Always `true`: the spotlight respects reduced-motion preferences.
    pub respects_reduced_motion: bool,
    pub clear_spotlight_action: KeyboardAction,
}

/// One row in the speaker-notes tray. Carries posture, never the body, so the
/// projection itself cannot leak a note off the presenter's machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNotesTrayRow {
    pub note_ref: String,
    pub linked_waypoint_ref: String,
    pub scope: SpeakerNoteScope,
    pub has_body: bool,
    pub next_step_cue_present: bool,
    pub shared_promotion_explicit: bool,
}

/// Speaker-notes tray: presenter-only prompts kept available without leaking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNotesTray {
    pub open: bool,
    /// Always `true`: notes default local-only.
    pub default_local_only: bool,
    /// True when at least one note has been explicitly promoted to shared.
    pub any_shared_notes: bool,
    pub rows: Vec<SpeakerNotesTrayRow>,
    pub actions: Vec<KeyboardAction>,
}

/// Follow chip: separate shared navigation from mutating control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowChip {
    pub leader_follow_state: LeaderFollowState,
    /// The presenter's current anchor, so a follower can always return to it.
    pub presenter_anchor_ref: Option<String>,
    pub actions: Vec<KeyboardAction>,
}

/// Audience strip: show who is following and who has broken away.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudienceStrip {
    pub participant_count: u32,
    pub following_count: u32,
    pub broken_away_count: u32,
    pub requesting_follow_count: u32,
    pub external_guest_count: u32,
    pub follow_chip: FollowChip,
    pub participants: Vec<AudienceParticipant>,
    pub actions: Vec<KeyboardAction>,
}

/// Breakaway banner: keep independent navigation honest during a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakawayBanner {
    /// The explicit "you are browsing independently" state label.
    pub state_label: String,
    pub presenter_anchor_ref: String,
    pub return_to_presenter_action: KeyboardAction,
}

/// Provenance strip: keep source identity visible under the overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceStrip {
    pub file_path_ref: Option<String>,
    pub symbol_anchor_ref: Option<String>,
    pub branch_workspace_ref: String,
    pub boundary_label: BoundaryLabel,
    /// Always `true`: decorative chrome never erases provenance.
    pub source_identity_preserved: bool,
}

/// Restore affordance: the explicit, keyboard-reachable way back out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreAffordance {
    pub checkpoint_ref: String,
    pub exit_action: KeyboardAction,
    pub cancel_action: KeyboardAction,
    /// Always `true`: exit/cancel restore the prior layout.
    pub restores_prior_layout: bool,
    /// Always `true`: exit/cancel restore the prior focus.
    pub restores_prior_focus: bool,
    /// Always `true`: crash recovery restores the prior layout too.
    pub restores_on_crash_recovery: bool,
}

/// The full reversible overlay projected over one session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationOverlay {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub session_id: String,
    pub presenter_bar: PresenterBar,
    pub waypoint_rail: WaypointRail,
    pub spotlight_frame: SpotlightFrame,
    pub speaker_notes_tray: SpeakerNotesTray,
    pub audience_strip: AudienceStrip,
    /// Present only while the local user is broken away.
    pub breakaway_banner: Option<BreakawayBanner>,
    pub provenance_strip: ProvenanceStrip,
    pub restore_affordance: RestoreAffordance,
    /// Always `true`: every control is reachable by keyboard.
    pub keyboard_complete: bool,
    /// Always `false`: nothing is pointer-only.
    pub pointer_only: bool,
    /// Always `true`: every surface is reachable by a screen reader.
    pub screen_reader_reachable: bool,
}

impl PresentationOverlay {
    /// All keyboard actions across every surface, in a stable order.
    pub fn all_actions(&self) -> Vec<&KeyboardAction> {
        let mut actions: Vec<&KeyboardAction> = Vec::new();
        actions.extend(self.presenter_bar.actions.iter());
        actions.extend(self.waypoint_rail.actions.iter());
        for row in &self.waypoint_rail.rows {
            actions.push(&row.jump_action);
        }
        actions.push(&self.spotlight_frame.clear_spotlight_action);
        actions.extend(self.speaker_notes_tray.actions.iter());
        actions.extend(self.audience_strip.actions.iter());
        actions.extend(self.audience_strip.follow_chip.actions.iter());
        if let Some(banner) = &self.breakaway_banner {
            actions.push(&banner.return_to_presenter_action);
        }
        actions.push(&self.restore_affordance.exit_action);
        actions.push(&self.restore_affordance.cancel_action);
        actions
    }
}

/// Project the reversible overlay surfaces for `session`.
pub fn project_overlay(session: &PresentationSession) -> PresentationOverlay {
    let sid = &session.session_id;

    let current_waypoint = session
        .current_focus_waypoint_ref
        .as_ref()
        .and_then(|r| session.waypoints.iter().find(|w| &w.waypoint_id == r))
        .or_else(|| session.waypoints.first());

    let presenter_bar = build_presenter_bar(session, current_waypoint);
    let waypoint_rail = build_waypoint_rail(session);
    let spotlight_frame = build_spotlight_frame(current_waypoint);
    let speaker_notes_tray = build_speaker_notes_tray(session);
    let audience_strip = build_audience_strip(session, current_waypoint);
    let breakaway_banner = build_breakaway_banner(session, current_waypoint);
    let provenance_strip = build_provenance_strip(current_waypoint);
    let restore_affordance = build_restore_affordance(session);

    PresentationOverlay {
        record_kind: PRESENTATION_OVERLAY_RECORD_KIND.to_owned(),
        schema_version: session.schema_version,
        shared_contract_ref: session.shared_contract_ref.clone(),
        session_id: sid.clone(),
        presenter_bar,
        waypoint_rail,
        spotlight_frame,
        speaker_notes_tray,
        audience_strip,
        breakaway_banner,
        provenance_strip,
        restore_affordance,
        keyboard_complete: true,
        pointer_only: false,
        screen_reader_reachable: true,
    }
}

fn build_presenter_bar(
    session: &PresentationSession,
    current: Option<&FollowWaypoint>,
) -> PresenterBar {
    let context_title = current
        .map(|w| w.step_title.clone())
        .unwrap_or_else(|| "Presentation".to_owned());
    let actions = vec![
        KeyboardAction::new(
            "act:presenter-bar:toggle-spotlight",
            "Spotlight",
            "cmd:presentation.toggle_spotlight",
            "key:presentation.toggle_spotlight",
            "Toggle the spotlight frame",
        ),
        KeyboardAction::new(
            "act:presenter-bar:toggle-notes",
            "Notes",
            "cmd:presentation.toggle_notes_tray",
            "key:presentation.toggle_notes_tray",
            "Toggle the speaker-notes tray",
        ),
        KeyboardAction::new(
            "act:presenter-bar:cycle-zoom",
            "Zoom",
            "cmd:presentation.cycle_zoom_preset",
            "key:presentation.cycle_zoom_preset",
            "Cycle the zoom preset",
        ),
        KeyboardAction::new(
            "act:presenter-bar:exit",
            "Exit",
            "cmd:presentation.exit",
            "key:presentation.exit",
            "Exit presentation mode and restore the prior layout",
        ),
    ];
    PresenterBar {
        context_title,
        zoom_preset: ZoomPreset::Standard,
        spotlight_enabled: session
            .current_focus_waypoint_ref
            .as_ref()
            .map(|_| true)
            .unwrap_or(false),
        leader_follow_state: session.leader_follow_state,
        notes_tray_open: false,
        actions,
    }
}

fn build_waypoint_rail(session: &PresentationSession) -> WaypointRail {
    let current_ref = session.current_focus_waypoint_ref.as_deref();
    let rows = session
        .waypoints
        .iter()
        .map(|w| {
            let is_current = Some(w.waypoint_id.as_str()) == current_ref
                || w.completion_state == WaypointCompletionState::Current;
            WaypointRailRow {
                waypoint_ref: w.waypoint_id.clone(),
                ordinal: w.ordinal,
                step_title: w.step_title.clone(),
                surface_kind: w.surface_kind,
                completion_state: w.completion_state,
                is_current,
                jump_action: KeyboardAction::new(
                    format!("act:waypoint-rail:jump:{}", w.waypoint_id),
                    "Jump",
                    "cmd:presentation.jump_to_waypoint",
                    "key:presentation.jump_to_waypoint",
                    format!("Jump to step {}: {}", w.ordinal, w.step_title),
                ),
            }
        })
        .collect();
    let actions = vec![
        KeyboardAction::new(
            "act:waypoint-rail:next",
            "Next",
            "cmd:presentation.next_waypoint",
            "key:presentation.next_waypoint",
            "Go to the next waypoint",
        ),
        KeyboardAction::new(
            "act:waypoint-rail:previous",
            "Previous",
            "cmd:presentation.previous_waypoint",
            "key:presentation.previous_waypoint",
            "Go to the previous waypoint",
        ),
    ];
    WaypointRail {
        rows,
        reorder_allowed: matches!(session.leader_follow_state, LeaderFollowState::Presenting),
        actions,
    }
}

fn build_spotlight_frame(current: Option<&FollowWaypoint>) -> SpotlightFrame {
    let enabled = current.is_some();
    SpotlightFrame {
        enabled,
        highlighted_region_ref: current.map(|w| w.target_object_ref.clone()),
        dimmed_surroundings: enabled,
        accessible_region_label: current.map(|w| format!("Spotlight on {}", w.step_title)),
        preserves_keyboard_order: true,
        respects_reduced_motion: true,
        clear_spotlight_action: KeyboardAction::new(
            "act:spotlight:clear",
            "Clear spotlight",
            "cmd:presentation.clear_spotlight",
            "key:presentation.clear_spotlight",
            "Clear the spotlight and reveal the full layout",
        ),
    }
}

fn build_speaker_notes_tray(session: &PresentationSession) -> SpeakerNotesTray {
    let rows: Vec<SpeakerNotesTrayRow> = session
        .waypoints
        .iter()
        .filter_map(|w| {
            w.speaker_note.as_ref().map(|n| SpeakerNotesTrayRow {
                note_ref: n.note_id.clone(),
                linked_waypoint_ref: n.linked_waypoint_ref.clone(),
                scope: n.scope,
                has_body: n.has_body(),
                next_step_cue_present: n.next_step_cue_label.is_some(),
                shared_promotion_explicit: n.shared_promotion_explicit,
            })
        })
        .collect();
    let any_shared_notes = rows.iter().any(|r| !r.scope.is_local_only());
    let actions = vec![
        KeyboardAction::new(
            "act:notes-tray:collapse",
            "Collapse notes",
            "cmd:presentation.collapse_notes_tray",
            "key:presentation.collapse_notes_tray",
            "Collapse the speaker-notes tray",
        ),
        KeyboardAction::new(
            "act:notes-tray:promote",
            "Share note",
            "cmd:presentation.promote_note_to_shared",
            "key:presentation.promote_note_to_shared",
            "Explicitly share the selected note with the session",
        ),
    ];
    SpeakerNotesTray {
        open: false,
        default_local_only: true,
        any_shared_notes,
        rows,
        actions,
    }
}

fn build_audience_strip(
    session: &PresentationSession,
    current: Option<&FollowWaypoint>,
) -> AudienceStrip {
    use super::session::ParticipantFollowState;
    let participants = session.audience_participants.clone();
    let participant_count = participants.len() as u32;
    let following_count = participants
        .iter()
        .filter(|p| p.follow_state == ParticipantFollowState::Following)
        .count() as u32;
    let broken_away_count = participants
        .iter()
        .filter(|p| p.follow_state == ParticipantFollowState::BrokenAway)
        .count() as u32;
    let requesting_follow_count = participants
        .iter()
        .filter(|p| p.follow_state == ParticipantFollowState::RequestingFollow)
        .count() as u32;
    let external_guest_count = participants.iter().filter(|p| p.is_external_guest).count() as u32;

    let follow_chip = FollowChip {
        leader_follow_state: session.leader_follow_state,
        presenter_anchor_ref: current.map(|w| w.target_object_ref.clone()),
        actions: vec![
            KeyboardAction::new(
                "act:follow-chip:follow",
                "Follow",
                "cmd:presentation.follow_presenter",
                "key:presentation.follow_presenter",
                "Follow the presenter's anchor",
            ),
            KeyboardAction::new(
                "act:follow-chip:break-away",
                "Break away",
                "cmd:presentation.break_away",
                "key:presentation.break_away",
                "Break away to browse independently",
            ),
            KeyboardAction::new(
                "act:follow-chip:return",
                "Return to presenter",
                "cmd:presentation.return_to_presenter",
                "key:presentation.return_to_presenter",
                "Return to the presenter's current anchor",
            ),
        ],
    };

    let actions = vec![
        KeyboardAction::new(
            "act:audience-strip:request-follow",
            "Request follow",
            "cmd:presentation.request_follow",
            "key:presentation.request_follow",
            "Ask the presenter to bring everyone to your anchor",
        ),
        KeyboardAction::new(
            "act:audience-strip:take-over",
            "Take over",
            "cmd:presentation.request_take_over",
            "key:presentation.request_take_over",
            "Request to take over as presenter",
        ),
    ];

    AudienceStrip {
        participant_count,
        following_count,
        broken_away_count,
        requesting_follow_count,
        external_guest_count,
        follow_chip,
        participants,
        actions,
    }
}

fn build_breakaway_banner(
    session: &PresentationSession,
    current: Option<&FollowWaypoint>,
) -> Option<BreakawayBanner> {
    if !session.leader_follow_state.is_broken_away() {
        return None;
    }
    let presenter_anchor_ref = current
        .map(|w| w.target_object_ref.clone())
        .unwrap_or_else(|| "presentation:anchor:unknown".to_owned());
    Some(BreakawayBanner {
        state_label: "You are browsing independently".to_owned(),
        presenter_anchor_ref,
        return_to_presenter_action: KeyboardAction::new(
            "act:breakaway-banner:return",
            "Return to presenter",
            "cmd:presentation.return_to_presenter",
            "key:presentation.return_to_presenter",
            "Return to the presenter's current anchor",
        ),
    })
}

fn build_provenance_strip(current: Option<&FollowWaypoint>) -> ProvenanceStrip {
    match current {
        Some(w) => ProvenanceStrip {
            file_path_ref: w.file_path_ref.clone(),
            symbol_anchor_ref: w.symbol_anchor_ref.clone(),
            branch_workspace_ref: w.branch_workspace_ref.clone(),
            boundary_label: w.boundary_label,
            source_identity_preserved: true,
        },
        None => ProvenanceStrip {
            file_path_ref: None,
            symbol_anchor_ref: None,
            branch_workspace_ref: "workspace:unknown".to_owned(),
            boundary_label: BoundaryLabel::Local,
            source_identity_preserved: true,
        },
    }
}

fn build_restore_affordance(session: &PresentationSession) -> RestoreAffordance {
    RestoreAffordance {
        checkpoint_ref: session.restore_checkpoint.checkpoint_id.clone(),
        exit_action: KeyboardAction::new(
            "act:restore:exit",
            "Exit",
            "cmd:presentation.exit",
            "key:presentation.exit",
            "Exit presentation mode and restore the prior layout",
        ),
        cancel_action: KeyboardAction::new(
            "act:restore:cancel",
            "Cancel",
            "cmd:presentation.cancel",
            "key:presentation.cancel",
            "Cancel presentation mode and restore the prior layout",
        ),
        restores_prior_layout: true,
        restores_prior_focus: true,
        restores_on_crash_recovery: true,
    }
}
