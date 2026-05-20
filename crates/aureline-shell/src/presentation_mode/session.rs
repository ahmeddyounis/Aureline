//! The governed presentation-session object.
//!
//! A [`PresentationSession`] is the single boundary object the shell uses to
//! drive a guided walkthrough as a *thin, reversible layer* over existing
//! Aureline surfaces — editors, diffs, docs, topology graphs, and notebooks —
//! rather than a parallel product. Every field on it exists to keep that layer
//! honest:
//!
//! - the [`leader_follow_state`](PresentationSession::leader_follow_state) and
//!   the per-participant [`AudienceParticipant`] states keep follow / breakaway
//!   / return explicit and attributable instead of inferred from cursor motion;
//! - each [`FollowWaypoint`] binds to a stable object on an *existing* surface
//!   and carries the file path, symbol anchor, branch/workspace context, and
//!   local/remote/shared boundary label so provenance is never erased by
//!   decorative full-screen chrome;
//! - [`SpeakerNote`]s default to [`SpeakerNoteScope::Local`] and only become
//!   shared through an explicit, separately recorded promotion;
//! - the [`RestoreCheckpoint`] captures the prior layout, focus, panel
//!   visibility, and accessibility posture so [`restore_from_checkpoint`] can
//!   put the user back exactly where they were on exit, cancel, or crash
//!   recovery — never in an improvised shell.
//!
//! The session never widens authority: it grants no mutation shortcut, no
//! shared-control escalation, and no private data-ownership semantics. Those
//! guardrails are encoded as inspectable flags so a reviewer (or the validation
//! corpus) can prove them rather than trust them.

use serde::{Deserialize, Serialize};

/// Schema version exported by presentation-mode beta records.
pub const PRESENTATION_MODE_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref carried by every presentation-mode beta record so shell
/// rows, the headless CLI rows, and support-export rows pivot to the same case.
pub const PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF: &str = "shell:presentation_mode_beta:v1";

/// Stable record kind for [`PresentationSession`] payloads.
pub const PRESENTATION_SESSION_RECORD_KIND: &str = "presentation_session_record";

/// Stable record kind for [`RestoreOutcome`] payloads.
pub const PRESENTATION_RESTORE_OUTCOME_RECORD_KIND: &str = "presentation_restore_outcome_record";

/// Where the session sits in its enter → active → restored lifecycle.
///
/// Entering checkpoints the prior layout first; every terminal state restores
/// that checkpoint, so a presentation can never strand the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionLifecycleState {
    /// The prior layout was checkpointed and the overlay is attaching.
    Entering,
    /// The overlay is active over the existing surfaces.
    Active,
    /// The user exited cleanly; the prior layout was restored.
    ExitedRestored,
    /// The user cancelled; the prior layout was restored.
    CancelledRestored,
    /// Crash recovery restored the prior layout from the checkpoint.
    CrashRecoveredRestored,
}

impl SessionLifecycleState {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entering => "entering",
            Self::Active => "active",
            Self::ExitedRestored => "exited_restored",
            Self::CancelledRestored => "cancelled_restored",
            Self::CrashRecoveredRestored => "crash_recovered_restored",
        }
    }
}

/// The local user's leader / follow posture in the session.
///
/// `Follow`, `Break away`, and `Request follow` are distinct states; the shell
/// never infers them from cursor movement alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaderFollowState {
    /// This user is the presenter driving the walkthrough.
    Presenting,
    /// Following the presenter's anchor; navigation tracks the leader.
    FollowingPresenter,
    /// Browsing independently while the presenter's anchor stays visible.
    BrokenAway,
    /// Asked to (re)join the presenter; not yet synced.
    RequestingFollow,
}

impl LeaderFollowState {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presenting => "presenting",
            Self::FollowingPresenter => "following_presenter",
            Self::BrokenAway => "broken_away",
            Self::RequestingFollow => "requesting_follow",
        }
    }

    /// True when the user is navigating away from the presenter's anchor and a
    /// breakaway banner must be shown.
    pub const fn is_broken_away(self) -> bool {
        matches!(self, Self::BrokenAway)
    }
}

/// Who can see the session, and therefore how careful the boundary labels and
/// note scopes have to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudienceScope {
    /// Solo rehearsal: no audience. Notes never leave the machine.
    SoloRehearsal,
    /// A shared session inside the same workspace / trust boundary.
    SharedWorkspace,
    /// Invited external guests are present; boundary labels must say so.
    InvitedGuests,
}

impl AudienceScope {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SoloRehearsal => "solo_rehearsal",
            Self::SharedWorkspace => "shared_workspace",
            Self::InvitedGuests => "invited_guests",
        }
    }
}

/// A layout preset the session asks the shell to adopt. `InheritCurrent` keeps
/// the user's existing layout and only adds overlay chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutPreset {
    /// Keep the current layout; only the overlay chrome is added.
    InheritCurrent,
    /// A focused single-surface preset for a code or diff walkthrough.
    FocusedSingle,
    /// A split preset (e.g. diff beside notes) for compare-driven walkthroughs.
    SplitCompare,
    /// A docs / graph oriented narrative preset.
    NarrativeWide,
}

impl LayoutPreset {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InheritCurrent => "inherit_current",
            Self::FocusedSingle => "focused_single",
            Self::SplitCompare => "split_compare",
            Self::NarrativeWide => "narrative_wide",
        }
    }
}

/// The kind of *existing* surface a waypoint targets. Presentation mode reuses
/// these surfaces; it never mints a new artifact type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalkthroughSurfaceKind {
    /// A code editor buffer.
    Editor,
    /// A diff / review surface.
    Diff,
    /// A docs / knowledge surface.
    Docs,
    /// A topology / dependency graph map.
    Graph,
    /// A notebook document.
    Notebook,
}

impl WalkthroughSurfaceKind {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::Docs => "docs",
            Self::Graph => "graph",
            Self::Notebook => "notebook",
        }
    }

    /// Every walkthrough surface is an existing Aureline surface, by
    /// construction. Kept as a method so the invariant reads explicitly in the
    /// validation corpus.
    pub const fn is_existing_surface(self) -> bool {
        true
    }
}

/// The local / remote / shared boundary a waypoint's target lives on. Preserved
/// so the overlay can label provenance instead of replacing it with chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryLabel {
    /// A local workspace object.
    Local,
    /// A remote / managed workspace object.
    Remote,
    /// A shared collaboration object.
    Shared,
}

impl BoundaryLabel {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Shared => "shared",
        }
    }
}

/// Where a waypoint sits relative to the presenter's progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaypointCompletionState {
    /// Not yet visited.
    Pending,
    /// The presenter's current anchor.
    Current,
    /// Already visited this session.
    Visited,
}

impl WaypointCompletionState {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Current => "current",
            Self::Visited => "visited",
        }
    }
}

/// Visibility scope of a speaker note. Notes default to [`Self::Local`] and
/// only become shared through an explicit, separately recorded promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeakerNoteScope {
    /// Presenter-only, never leaves the local machine.
    Local,
    /// Explicitly promoted to the shared session. Auditable.
    Shared,
}

impl SpeakerNoteScope {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Shared => "shared",
        }
    }

    /// True when the note is visible only to the presenter.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::Local)
    }
}

/// A presenter-only prompt attached to a waypoint. The human-readable
/// `body_label` lives on the in-memory model and the reviewable fixtures, but
/// the support export carries only [`SpeakerNote::has_body`] and posture flags
/// so notes cannot leak through diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeakerNote {
    pub note_id: String,
    pub linked_waypoint_ref: String,
    pub scope: SpeakerNoteScope,
    /// Short presenter-facing reminder. `None` for an empty note slot.
    pub body_label: Option<String>,
    /// A short "next step" cue shown under the note, if present.
    pub next_step_cue_label: Option<String>,
    /// Docs / graph citation refs the note points at, reusing learning-mode
    /// citation objects so explanation surfaces stay coherent.
    pub citation_refs: Vec<String>,
    /// True only when the note was deliberately promoted to a shared scope.
    /// Always `false` for [`SpeakerNoteScope::Local`].
    pub shared_promotion_explicit: bool,
}

impl SpeakerNote {
    /// A local-only note. Cannot leak because it is never shared.
    pub fn local(
        note_id: impl Into<String>,
        linked_waypoint_ref: impl Into<String>,
        body_label: impl Into<String>,
    ) -> Self {
        Self {
            note_id: note_id.into(),
            linked_waypoint_ref: linked_waypoint_ref.into(),
            scope: SpeakerNoteScope::Local,
            body_label: Some(body_label.into()),
            next_step_cue_label: None,
            citation_refs: Vec::new(),
            shared_promotion_explicit: false,
        }
    }

    /// An explicitly shared note. The `shared_promotion_explicit` flag is set
    /// so the share decision stays attributable.
    pub fn shared(
        note_id: impl Into<String>,
        linked_waypoint_ref: impl Into<String>,
        body_label: impl Into<String>,
    ) -> Self {
        Self {
            note_id: note_id.into(),
            linked_waypoint_ref: linked_waypoint_ref.into(),
            scope: SpeakerNoteScope::Shared,
            body_label: Some(body_label.into()),
            next_step_cue_label: None,
            citation_refs: Vec::new(),
            shared_promotion_explicit: true,
        }
    }

    /// Attach a next-step cue.
    pub fn with_next_step(mut self, cue: impl Into<String>) -> Self {
        self.next_step_cue_label = Some(cue.into());
        self
    }

    /// Attach citation refs.
    pub fn with_citations(mut self, refs: Vec<String>) -> Self {
        self.citation_refs = refs;
        self
    }

    /// True when the note carries a presenter-facing body.
    pub fn has_body(&self) -> bool {
        self.body_label.is_some()
    }
}

/// One prepared anchor in the walkthrough. Binds to a stable object on an
/// existing surface and carries the provenance that must survive the overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowWaypoint {
    pub waypoint_id: String,
    pub ordinal: u32,
    /// Short step title shown in the rail and presenter bar.
    pub step_title: String,
    pub surface_kind: WalkthroughSurfaceKind,
    /// Stable id of the target object on the existing surface.
    pub target_object_ref: String,
    /// File path the target lives at (preserved, not erased).
    pub file_path_ref: Option<String>,
    /// Symbol anchor within the file, when applicable.
    pub symbol_anchor_ref: Option<String>,
    /// Branch / workspace context for the target.
    pub branch_workspace_ref: String,
    /// Local / remote / shared boundary the target lives on.
    pub boundary_label: BoundaryLabel,
    /// Stable id of the zoom / layout hint for this step.
    pub zoom_layout_hint_ref: Option<String>,
    /// Stable id of the reveal action played when arriving at this step.
    pub reveal_action_ref: Option<String>,
    pub completion_state: WaypointCompletionState,
    pub speaker_note: Option<SpeakerNote>,
    /// Always `true`: a waypoint targets an existing surface.
    pub reuses_existing_surface: bool,
    /// Always `false`: a waypoint never creates a parallel artifact type.
    pub creates_parallel_artifact: bool,
}

/// Role badge for an audience participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantRole {
    /// The presenter driving the session.
    Presenter,
    /// A co-presenter who may request to take over (still explicit).
    CoPresenter,
    /// A viewer following along.
    Viewer,
}

impl ParticipantRole {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presenter => "presenter",
            Self::CoPresenter => "co_presenter",
            Self::Viewer => "viewer",
        }
    }
}

/// A participant's follow / breakaway posture, shown in the audience strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantFollowState {
    /// Following the presenter's anchor.
    Following,
    /// Browsing independently.
    BrokenAway,
    /// Asked to (re)join the presenter.
    RequestingFollow,
}

impl ParticipantFollowState {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Following => "following",
            Self::BrokenAway => "broken_away",
            Self::RequestingFollow => "requesting_follow",
        }
    }
}

/// One audience member, summarized for the audience strip / follow chip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudienceParticipant {
    pub participant_id: String,
    pub role_badge: ParticipantRole,
    pub follow_state: ParticipantFollowState,
    /// True for an invited external guest (drives a distinct badge).
    pub is_external_guest: bool,
}

/// The checkpoint captured on enter so the prior environment can be restored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreCheckpoint {
    pub checkpoint_id: String,
    /// Window-topology snapshot ref captured before the overlay attached.
    pub prior_layout_ref: String,
    /// Focus-chain / selection ref captured before the overlay attached.
    pub prior_focus_ref: String,
    /// Panel-visibility ref captured before the overlay attached.
    pub prior_panel_visibility_ref: String,
    /// Accessibility posture (screen-reader / reduced-motion) preserved.
    pub accessibility_posture_ref: String,
    pub captured_at: String,
}

/// The single governed presentation-session object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationSession {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub session_id: String,
    pub lifecycle_state: SessionLifecycleState,
    pub leader_follow_state: LeaderFollowState,
    pub layout_preset: LayoutPreset,
    pub audience_scope: AudienceScope,
    /// The waypoint that currently holds focus, if any.
    pub current_focus_waypoint_ref: Option<String>,
    pub waypoints: Vec<FollowWaypoint>,
    pub audience_participants: Vec<AudienceParticipant>,
    pub restore_checkpoint: RestoreCheckpoint,
    // ---- guardrail flags (always the safe value; encoded for inspection) ----
    /// Presentation guides attention; it never opens a mutation shortcut.
    pub grants_mutation_authority: bool,
    /// Following is not control: no shared editing / debug authority is granted.
    pub grants_control_authority: bool,
    /// The session owns no private data class; notes are user-owned local state.
    pub establishes_private_data_ownership: bool,
    /// Speaker notes are local-only unless explicitly promoted.
    pub speaker_notes_default_local_only: bool,
    /// File path, symbol anchor, branch/workspace, and boundary labels survive.
    pub preserves_source_provenance: bool,
    /// Every waypoint targets an existing surface; none duplicate one.
    pub reuses_existing_surfaces_only: bool,
}

/// Builder for assembling a [`PresentationSession`] with the guardrail flags
/// fixed to their safe values.
pub struct PresentationSessionBuilder {
    session_id: String,
    lifecycle_state: SessionLifecycleState,
    leader_follow_state: LeaderFollowState,
    layout_preset: LayoutPreset,
    audience_scope: AudienceScope,
    current_focus_waypoint_ref: Option<String>,
    waypoints: Vec<FollowWaypoint>,
    audience_participants: Vec<AudienceParticipant>,
    restore_checkpoint: RestoreCheckpoint,
}

impl PresentationSessionBuilder {
    /// Start a builder. The checkpoint is required up front because entering
    /// presentation mode must checkpoint the prior layout first.
    pub fn new(
        session_id: impl Into<String>,
        leader_follow_state: LeaderFollowState,
        audience_scope: AudienceScope,
        restore_checkpoint: RestoreCheckpoint,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            lifecycle_state: SessionLifecycleState::Active,
            leader_follow_state,
            layout_preset: LayoutPreset::InheritCurrent,
            audience_scope,
            current_focus_waypoint_ref: None,
            waypoints: Vec::new(),
            audience_participants: Vec::new(),
            restore_checkpoint,
        }
    }

    /// Set the lifecycle state (defaults to [`SessionLifecycleState::Active`]).
    pub fn lifecycle(mut self, state: SessionLifecycleState) -> Self {
        self.lifecycle_state = state;
        self
    }

    /// Set the layout preset (defaults to [`LayoutPreset::InheritCurrent`]).
    pub fn layout(mut self, preset: LayoutPreset) -> Self {
        self.layout_preset = preset;
        self
    }

    /// Set the current focus waypoint ref.
    pub fn focus(mut self, waypoint_ref: impl Into<String>) -> Self {
        self.current_focus_waypoint_ref = Some(waypoint_ref.into());
        self
    }

    /// Append a waypoint.
    pub fn waypoint(mut self, waypoint: FollowWaypoint) -> Self {
        self.waypoints.push(waypoint);
        self
    }

    /// Append an audience participant.
    pub fn participant(mut self, participant: AudienceParticipant) -> Self {
        self.audience_participants.push(participant);
        self
    }

    /// Finish the session with the guardrail flags fixed safe.
    pub fn build(self) -> PresentationSession {
        PresentationSession {
            record_kind: PRESENTATION_SESSION_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            session_id: self.session_id,
            lifecycle_state: self.lifecycle_state,
            leader_follow_state: self.leader_follow_state,
            layout_preset: self.layout_preset,
            audience_scope: self.audience_scope,
            current_focus_waypoint_ref: self.current_focus_waypoint_ref,
            waypoints: self.waypoints,
            audience_participants: self.audience_participants,
            restore_checkpoint: self.restore_checkpoint,
            grants_mutation_authority: false,
            grants_control_authority: false,
            establishes_private_data_ownership: false,
            speaker_notes_default_local_only: true,
            preserves_source_provenance: true,
            reuses_existing_surfaces_only: true,
        }
    }
}

impl PresentationSession {
    /// True when no speaker note has been promoted to a shared scope.
    pub fn all_notes_local_only(&self) -> bool {
        self.waypoints
            .iter()
            .filter_map(|w| w.speaker_note.as_ref())
            .all(|n| n.scope.is_local_only())
    }

    /// True when every shared note carries an explicit promotion marker.
    pub fn shared_notes_are_explicit(&self) -> bool {
        self.waypoints
            .iter()
            .filter_map(|w| w.speaker_note.as_ref())
            .filter(|n| !n.scope.is_local_only())
            .all(|n| n.shared_promotion_explicit)
    }

    /// True when every waypoint reuses an existing surface and creates no
    /// parallel artifact.
    pub fn waypoints_reuse_existing_surfaces(&self) -> bool {
        self.waypoints
            .iter()
            .all(|w| w.reuses_existing_surface && !w.creates_parallel_artifact)
    }

    /// True when every waypoint preserves the source provenance the overlay
    /// must not erase: a stable target identity and branch/workspace context
    /// always, plus a file path for file-backed surfaces (editor, diff,
    /// notebook). Docs and graph targets are addressed by their stable object
    /// ref rather than a file path, so a path is not required for them.
    pub fn waypoints_preserve_provenance(&self) -> bool {
        self.waypoints.iter().all(|w| {
            let has_target = !w.target_object_ref.is_empty();
            let has_context = !w.branch_workspace_ref.is_empty();
            let file_backed = matches!(
                w.surface_kind,
                WalkthroughSurfaceKind::Editor
                    | WalkthroughSurfaceKind::Diff
                    | WalkthroughSurfaceKind::Notebook
            );
            let file_ok = !file_backed || w.file_path_ref.is_some();
            has_target && has_context && file_ok
        })
    }
}

/// What triggered a restore. Each trigger restores the same checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreTrigger {
    /// The user exited presentation mode cleanly.
    Exit,
    /// The user cancelled while entering or mid-session.
    Cancel,
    /// Crash recovery rehydrated the session and restored the prior layout.
    CrashRecovery,
}

impl RestoreTrigger {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exit => "exit",
            Self::Cancel => "cancel",
            Self::CrashRecovery => "crash_recovery",
        }
    }

    /// The lifecycle state a session lands in after this trigger.
    pub const fn restored_lifecycle(self) -> SessionLifecycleState {
        match self {
            Self::Exit => SessionLifecycleState::ExitedRestored,
            Self::Cancel => SessionLifecycleState::CancelledRestored,
            Self::CrashRecovery => SessionLifecycleState::CrashRecoveredRestored,
        }
    }
}

/// The result of restoring the prior environment from a session's checkpoint.
///
/// Every field but the trigger is derived from the checkpoint, so the outcome
/// proves the prior layout / focus / panels / accessibility posture come back
/// exactly and the user is never left in an improvised shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreOutcome {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub session_id: String,
    pub trigger: RestoreTrigger,
    pub resulting_lifecycle_state: SessionLifecycleState,
    pub restored_layout_ref: String,
    pub restored_focus_ref: String,
    pub restored_panel_visibility_ref: String,
    pub restored_accessibility_posture_ref: String,
    /// True when the restored refs match the checkpoint exactly.
    pub matches_checkpoint: bool,
    /// Always `false`: the user is never left in an improvised shell.
    pub left_in_improvised_shell: bool,
}

/// Restore the prior environment for `session` under `trigger`.
///
/// Exit, cancel, and crash recovery all replay the same checkpoint, so the
/// resulting environment is identical regardless of how the session ended.
pub fn restore_from_checkpoint(
    session: &PresentationSession,
    trigger: RestoreTrigger,
) -> RestoreOutcome {
    let cp = &session.restore_checkpoint;
    RestoreOutcome {
        record_kind: PRESENTATION_RESTORE_OUTCOME_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session.session_id.clone(),
        trigger,
        resulting_lifecycle_state: trigger.restored_lifecycle(),
        restored_layout_ref: cp.prior_layout_ref.clone(),
        restored_focus_ref: cp.prior_focus_ref.clone(),
        restored_panel_visibility_ref: cp.prior_panel_visibility_ref.clone(),
        restored_accessibility_posture_ref: cp.accessibility_posture_ref.clone(),
        matches_checkpoint: true,
        left_in_improvised_shell: false,
    }
}
