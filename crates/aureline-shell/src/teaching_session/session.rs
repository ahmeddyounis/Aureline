//! The governed teaching / classroom session object.
//!
//! A [`TeachingSession`] is the single boundary object the shell uses to run a
//! teaching or classroom walkthrough as a *thin, reversible layer* over the
//! existing learnability surfaces — guided tours, exercise packs, glossary
//! cards, and speaker notes — rather than a parallel collaboration product.
//! Every field exists to keep the layer honest:
//!
//! - the [`local_role`](TeachingSession::local_role) and per-participant
//!   [`TeachingParticipant`] roles describe *participation*, never control:
//!   teaching roles stay separate from terminal / debug authority and never
//!   imply broader rights than the underlying workspace already grants;
//! - each [`TeachingSegment`] cites the *same* docs / graph objects learning
//!   mode uses (its `learning_object_ref`, `docs_node_refs`, `graph_node_refs`,
//!   and `citation_refs` reuse the seeded learning-mode ids) and stays resumable
//!   across restart and reconnect;
//! - each segment carries an explicit [`DocsPackState`] so offline, cached,
//!   mirrored, and not-installed packs stay visible instead of pretending remote
//!   enrichment is available;
//! - a [`DemonstratedAction`] is non-mutating by default; any mutation
//!   demonstrated during teaching reuses the ordinary command id, preview sheet,
//!   approval fence, and rollback semantics — never a teaching shortcut;
//! - the [`RestoreCheckpoint`] captures the prior layout, focus, panel
//!   visibility, and accessibility posture so [`restore_from_checkpoint`] puts
//!   the user back exactly where they were on exit, leave, or crash recovery.
//!
//! The session never widens authority and never mints a hidden progress model,
//! cohort dashboard, or grading flow. Those guardrails are encoded as
//! inspectable flags so a reviewer (or the conformance corpus) can prove them
//! rather than trust them.

use serde::{Deserialize, Serialize};

/// Schema version exported by teaching-session beta records.
pub const TEACHING_SESSION_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref carried by every teaching-session beta record so shell
/// rows, the headless CLI rows, and support-export rows pivot to the same case.
pub const TEACHING_SESSION_BETA_SHARED_CONTRACT_REF: &str = "shell:teaching_classroom_beta:v1";

/// Stable record kind for [`TeachingSession`] payloads.
pub const TEACHING_SESSION_RECORD_KIND: &str = "teaching_session_record";

/// Stable record kind for [`TeachingRestoreOutcome`] payloads.
pub const TEACHING_RESTORE_OUTCOME_RECORD_KIND: &str = "teaching_restore_outcome_record";

/// Whether the session is a one-presenter teaching walkthrough or a multi-seat
/// classroom. Both are optional, cited, and reversible; neither widens
/// authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionKind {
    /// A teaching walkthrough (one presenter guiding attention).
    Teaching,
    /// A classroom session with multiple seated participants.
    Classroom,
}

impl SessionKind {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Teaching => "teaching",
            Self::Classroom => "classroom",
        }
    }
}

/// Where the session sits in its enter → active → restored lifecycle.
///
/// Entering checkpoints the prior layout first; every terminal state restores
/// that checkpoint, so a teaching session can never strand the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionLifecycleState {
    /// The prior layout was checkpointed and the session is attaching.
    Entering,
    /// The session is active over the existing learnability surfaces.
    Active,
    /// The user exited cleanly; the prior layout was restored.
    ExitedRestored,
    /// The user left the classroom; the prior layout was restored.
    LeftRestored,
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
            Self::LeftRestored => "left_restored",
            Self::CrashRecoveredRestored => "crash_recovered_restored",
        }
    }
}

/// A typed teaching role. Roles describe participation in the session; they are
/// deliberately *not* a control or authority grant. No role grants terminal or
/// debug control, and no role implies broader authority than the underlying
/// workspace already permits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeachingRole {
    /// Drives the session: advances segments, spotlights, narrates.
    Moderator,
    /// Works the exercises along with the moderator.
    Participant,
    /// Watches only; no drive, mutation, or note affordance.
    Observer,
    /// Approves a demonstrated mutation through the ordinary approval fence.
    Approver,
    /// Takes shared notes for the session; no drive or mutation affordance.
    Scribe,
}

impl TeachingRole {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Moderator => "moderator",
            Self::Participant => "participant",
            Self::Observer => "observer",
            Self::Approver => "approver",
            Self::Scribe => "scribe",
        }
    }

    /// True when the role may drive the session (advance segments, spotlight).
    /// Only the moderator drives.
    pub const fn can_drive_session(self) -> bool {
        matches!(self, Self::Moderator)
    }

    /// True when the role exposes a mutation affordance at all. Mutation still
    /// flows through the ordinary preview/approval/rollback fence regardless;
    /// the role never *grants* the authority. Observers and scribes never see a
    /// mutation affordance.
    pub const fn may_expose_mutation_affordance(self) -> bool {
        matches!(self, Self::Moderator | Self::Participant | Self::Approver)
    }

    /// True when the role may take notes. Note-taking is low-bandwidth friendly
    /// and available to the moderator, participants, and the scribe.
    pub const fn can_take_notes(self) -> bool {
        matches!(self, Self::Moderator | Self::Participant | Self::Scribe)
    }

    /// Always `false`: a teaching role never grants terminal or debug control.
    /// Kept as a method so the invariant reads explicitly in the corpus.
    pub const fn grants_terminal_or_debug_control(self) -> bool {
        false
    }

    /// Always `false`: a teaching role never implies broader authority than the
    /// underlying workspace already permits.
    pub const fn implies_broader_authority(self) -> bool {
        false
    }
}

/// The capability class of a participant's client. Limited and low-bandwidth
/// clients still join cleanly — as observers or note-taking participants —
/// without ever being shown broken or misleading control affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientClass {
    /// A full-capability client.
    Full,
    /// A reduced-capability client (e.g. a constrained or remote viewer).
    Limited,
    /// A low-bandwidth client that must avoid heavy live affordances.
    LowBandwidth,
}

impl ClientClass {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Limited => "limited",
            Self::LowBandwidth => "low_bandwidth",
        }
    }

    /// True for a constrained (limited or low-bandwidth) client.
    pub const fn is_constrained(self) -> bool {
        matches!(self, Self::Limited | Self::LowBandwidth)
    }
}

/// The availability state of the docs pack a segment relies on.
///
/// The state is always rendered so a guided session stays explicit about what
/// is locally available, what is stale, and what requires reconnect or install
/// — it never pretends remote enrichment is available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPackState {
    /// Locally installed and current.
    Installed,
    /// Locally available from a cached snapshot; stale, disclosed.
    Cached,
    /// Locally available from an offline mirror; stale, disclosed.
    Mirrored,
    /// Remote enrichment is unavailable; local content shows, fresh enrichment
    /// requires reconnect.
    Offline,
    /// The pack is not installed; content is blocked behind an explicit install.
    NotInstalled,
}

impl DocsPackState {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Cached => "cached",
            Self::Mirrored => "mirrored",
            Self::Offline => "offline",
            Self::NotInstalled => "not_installed",
        }
    }

    /// True when the pack content is available on the local machine.
    pub const fn is_locally_available(self) -> bool {
        matches!(self, Self::Installed | Self::Cached | Self::Mirrored | Self::Offline)
    }

    /// True when the pack content is stale relative to the live source.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Cached | Self::Mirrored | Self::Offline)
    }

    /// True when fresh enrichment requires reconnecting to the live source.
    pub const fn requires_reconnect(self) -> bool {
        matches!(self, Self::Offline)
    }

    /// True when the pack must be installed before its content is available.
    pub const fn requires_install(self) -> bool {
        matches!(self, Self::NotInstalled)
    }

    /// True when the state must carry a user-visible disclosure (anything other
    /// than a current, installed pack).
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Installed)
    }
}

/// Replay / archive posture for the session. Replay and archive are optional and
/// reversible; sharing an archive is an explicit, separately recorded decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPolicy {
    /// Nothing is replayed or archived; the session is ephemeral.
    Ephemeral,
    /// Replay is kept locally and owned by the user.
    LocalReplayUserOwned,
    /// The session is archived to the shared workspace only on explicit opt-in.
    SharedArchiveOptIn,
}

impl ReplayPolicy {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ephemeral => "ephemeral",
            Self::LocalReplayUserOwned => "local_replay_user_owned",
            Self::SharedArchiveOptIn => "shared_archive_opt_in",
        }
    }

    /// True when this policy archives to a shared scope and therefore requires
    /// an explicit opt-in marker.
    pub const fn requires_explicit_opt_in(self) -> bool {
        matches!(self, Self::SharedArchiveOptIn)
    }
}

/// Retention class for any artifact the session keeps. Defaults to the most
/// private, reversible class; sharing or retaining beyond the local machine is
/// an explicit decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Nothing is retained; artifacts are discarded on exit.
    DiscardOnExit,
    /// Artifacts are kept locally and owned by the user.
    LocalUserOwned,
    /// Artifacts are retained in the shared workspace on explicit opt-in.
    SharedWorkspaceRetained,
}

impl RetentionClass {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscardOnExit => "discard_on_exit",
            Self::LocalUserOwned => "local_user_owned",
            Self::SharedWorkspaceRetained => "shared_workspace_retained",
        }
    }

    /// True when this class retains to a shared scope and therefore requires an
    /// explicit opt-in marker.
    pub const fn requires_explicit_opt_in(self) -> bool {
        matches!(self, Self::SharedWorkspaceRetained)
    }
}

/// The kind of learnability content a segment carries. Each kind cites the same
/// docs / graph objects learning mode uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentKind {
    /// A guided tour reused from learning mode.
    Tour,
    /// A guided exercise pack reused from learning mode.
    ExercisePack,
    /// A glossary card backed by a docs node.
    GlossaryCard,
    /// A presenter-facing speaker note.
    SpeakerNote,
}

impl SegmentKind {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tour => "tour",
            Self::ExercisePack => "exercise_pack",
            Self::GlossaryCard => "glossary_card",
            Self::SpeakerNote => "speaker_note",
        }
    }
}

/// What a demonstrated action does. Demonstrations are non-mutating by default;
/// only [`Self::MutationThroughFences`] mutates, and only by reusing the
/// ordinary command path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DemonstrationKind {
    /// Explain only; reads nothing destructive and writes nothing.
    Explain,
    /// Open the cited docs source; read-only.
    OpenDocs,
    /// Prepare a preview without applying it; read-only until approved.
    PreviewOnly,
    /// Apply a mutation through the ordinary preview / approval / rollback fence.
    MutationThroughFences,
}

impl DemonstrationKind {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Explain => "explain",
            Self::OpenDocs => "open_docs",
            Self::PreviewOnly => "preview_only",
            Self::MutationThroughFences => "mutation_through_fences",
        }
    }

    /// True when the demonstration mutates the workspace.
    pub const fn mutates(self) -> bool {
        matches!(self, Self::MutationThroughFences)
    }
}

/// A demonstrated action attached to a teaching segment.
///
/// Non-mutating by default. A mutating demonstration is allowed only when it
/// reuses the ordinary command id, preview sheet, approval fence, and rollback
/// semantics — never a teaching-only shortcut.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DemonstratedAction {
    pub action_id: String,
    pub kind: DemonstrationKind,
    /// Command id this action invokes, reusing the command graph.
    pub command_id: Option<String>,
    /// Preview sheet ref reused from ordinary work (required for mutation).
    pub preview_sheet_ref: Option<String>,
    /// Approval path ref reused from ordinary work (required for mutation).
    pub approval_path_ref: Option<String>,
    /// Rollback / reset semantics ref reused from ordinary work (required for
    /// mutation).
    pub rollback_semantics_ref: Option<String>,
    /// Evidence packet rule ref reused from ordinary work (required for
    /// mutation).
    pub evidence_packet_rule_ref: Option<String>,
    /// Whether the demonstration mutates the workspace.
    pub mutates_workspace: bool,
    /// Always `true`: a demonstration reuses the ordinary command path rather
    /// than a teaching-only shortcut.
    pub reuses_ordinary_command_path: bool,
}

impl DemonstratedAction {
    /// An explain-only demonstration.
    pub fn explain(action_id: impl Into<String>) -> Self {
        Self {
            action_id: action_id.into(),
            kind: DemonstrationKind::Explain,
            command_id: None,
            preview_sheet_ref: None,
            approval_path_ref: None,
            rollback_semantics_ref: None,
            evidence_packet_rule_ref: None,
            mutates_workspace: false,
            reuses_ordinary_command_path: true,
        }
    }

    /// An open-docs demonstration that invokes a read-only command.
    pub fn open_docs(action_id: impl Into<String>, command_id: impl Into<String>) -> Self {
        Self {
            action_id: action_id.into(),
            kind: DemonstrationKind::OpenDocs,
            command_id: Some(command_id.into()),
            preview_sheet_ref: None,
            approval_path_ref: None,
            rollback_semantics_ref: None,
            evidence_packet_rule_ref: None,
            mutates_workspace: false,
            reuses_ordinary_command_path: true,
        }
    }

    /// A preview-only demonstration: prepares a preview but applies nothing.
    pub fn preview_only(
        action_id: impl Into<String>,
        command_id: impl Into<String>,
        preview_sheet_ref: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            kind: DemonstrationKind::PreviewOnly,
            command_id: Some(command_id.into()),
            preview_sheet_ref: Some(preview_sheet_ref.into()),
            approval_path_ref: None,
            rollback_semantics_ref: None,
            evidence_packet_rule_ref: None,
            mutates_workspace: false,
            reuses_ordinary_command_path: true,
        }
    }

    /// A mutating demonstration that reuses the ordinary command path: command
    /// id, preview sheet, approval fence, rollback semantics, and evidence rule.
    #[allow(clippy::too_many_arguments)]
    pub fn mutation_through_fences(
        action_id: impl Into<String>,
        command_id: impl Into<String>,
        preview_sheet_ref: impl Into<String>,
        approval_path_ref: impl Into<String>,
        rollback_semantics_ref: impl Into<String>,
        evidence_packet_rule_ref: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            kind: DemonstrationKind::MutationThroughFences,
            command_id: Some(command_id.into()),
            preview_sheet_ref: Some(preview_sheet_ref.into()),
            approval_path_ref: Some(approval_path_ref.into()),
            rollback_semantics_ref: Some(rollback_semantics_ref.into()),
            evidence_packet_rule_ref: Some(evidence_packet_rule_ref.into()),
            mutates_workspace: true,
            reuses_ordinary_command_path: true,
        }
    }

    /// True when a mutating demonstration carries every ordinary-work fence:
    /// command id, preview sheet, approval path, and rollback semantics. A
    /// non-mutating demonstration is always fenced by construction.
    pub fn is_properly_fenced(&self) -> bool {
        if !self.mutates_workspace {
            return true;
        }
        self.kind == DemonstrationKind::MutationThroughFences
            && self.command_id.is_some()
            && self.preview_sheet_ref.is_some()
            && self.approval_path_ref.is_some()
            && self.rollback_semantics_ref.is_some()
            && self.reuses_ordinary_command_path
    }
}

/// One segment of teaching content. Cites the same docs / graph objects learning
/// mode uses and stays resumable across restart and reconnect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingSegment {
    pub segment_id: String,
    pub ordinal: u32,
    pub segment_kind: SegmentKind,
    /// Short reviewable title shown in the agenda.
    pub title: String,
    /// The learning-mode object this segment reuses (tour, step, exercise pack,
    /// or docs node id), so teaching content never forks from learning mode.
    pub learning_object_ref: String,
    /// Docs node refs cited by the segment (shared with learning mode).
    pub docs_node_refs: Vec<String>,
    /// Graph node refs cited by the segment (shared with learning mode).
    pub graph_node_refs: Vec<String>,
    /// Citation refs backing the segment (shared with learning mode).
    pub citation_refs: Vec<String>,
    /// The docs pack this segment relies on.
    pub docs_pack_ref: String,
    /// Availability state of that docs pack.
    pub docs_pack_state: DocsPackState,
    /// User-visible disclosure ref for a degraded docs-pack state. Required for
    /// any state other than installed; `None` only when installed and current.
    pub docs_pack_disclosure_ref: Option<String>,
    /// Stable resume ref so the segment can be reopened exactly.
    pub resume_ref: String,
    /// Always `true`: the segment resumes across an app restart.
    pub resumable_across_restart: bool,
    /// Always `true`: the segment resumes across a reconnect.
    pub resumable_across_reconnect: bool,
    /// An optional demonstrated action. Non-mutating by default.
    pub demonstrated_action: Option<DemonstratedAction>,
    /// Always `true`: the segment cites a learning-mode object.
    pub cites_learning_mode_object: bool,
}

impl TeachingSegment {
    /// True when the segment carries at least one docs / graph / citation
    /// reference so its claims trace back to a cited source.
    pub fn has_citation(&self) -> bool {
        !self.docs_node_refs.is_empty()
            || !self.graph_node_refs.is_empty()
            || !self.citation_refs.is_empty()
    }

    /// True when a degraded docs-pack state carries the disclosure it requires.
    /// An installed pack needs no disclosure.
    pub fn docs_pack_disclosure_ok(&self) -> bool {
        if self.docs_pack_state.requires_disclosure() {
            self.docs_pack_disclosure_ref.is_some()
        } else {
            true
        }
    }

    /// True when the segment is resumable across both restart and reconnect and
    /// carries a non-empty resume ref.
    pub fn is_resumable(&self) -> bool {
        self.resumable_across_restart
            && self.resumable_across_reconnect
            && !self.resume_ref.is_empty()
    }
}

/// One seated participant in a teaching / classroom session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingParticipant {
    pub participant_id: String,
    pub role: TeachingRole,
    pub client_class: ClientClass,
    /// True for an invited external guest (drives a distinct badge).
    pub is_external_guest: bool,
}

/// The checkpoint captured on enter so the prior environment can be restored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreCheckpoint {
    pub checkpoint_id: String,
    /// Window-topology snapshot ref captured before the session attached.
    pub prior_layout_ref: String,
    /// Focus-chain / selection ref captured before the session attached.
    pub prior_focus_ref: String,
    /// Panel-visibility ref captured before the session attached.
    pub prior_panel_visibility_ref: String,
    /// Accessibility posture (screen-reader / reduced-motion) preserved.
    pub accessibility_posture_ref: String,
    pub captured_at: String,
}

/// The single governed teaching / classroom session object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingSession {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub session_id: String,
    pub session_kind: SessionKind,
    pub lifecycle_state: SessionLifecycleState,
    /// The local user's role in the session.
    pub local_role: TeachingRole,
    pub replay_policy: ReplayPolicy,
    pub retention_class: RetentionClass,
    /// Exercise-pack ids the session draws its segments from (learning-mode
    /// package ids).
    pub exercise_pack_refs: Vec<String>,
    /// The segment that currently holds focus, if any.
    pub current_focus_segment_ref: Option<String>,
    pub segments: Vec<TeachingSegment>,
    pub participants: Vec<TeachingParticipant>,
    pub restore_checkpoint: RestoreCheckpoint,
    /// True when an archived/shared replay was explicitly opted into. Must be
    /// true iff `replay_policy` archives to a shared scope.
    pub replay_archive_opt_in_explicit: bool,
    /// True when shared retention was explicitly opted into. Must be true iff
    /// `retention_class` retains to a shared scope.
    pub shared_retention_opt_in_explicit: bool,
    // ---- guardrail flags (always the safe value; encoded for inspection) ----
    /// Teaching guides attention; it never opens a mutation shortcut.
    pub grants_mutation_authority: bool,
    /// A teaching role is not control: no terminal or debug authority is granted.
    pub grants_terminal_or_debug_control: bool,
    /// A teaching role never widens authority beyond the workspace's own grants.
    pub grants_broader_authority_than_workspace: bool,
    /// The session owns no new private data class; artifacts are user-owned.
    pub establishes_private_data_ownership: bool,
    /// No hidden telemetry-only progress model is created.
    pub creates_hidden_progress_model: bool,
    /// No cohort dashboard or grading flow is created.
    pub creates_cohort_or_grading_flow: bool,
    /// Demonstrated actions are non-mutating unless explicitly fenced.
    pub demonstrations_non_mutating_by_default: bool,
    /// Every segment cites the source objects learning mode cites.
    pub preserves_source_citations: bool,
    /// Every segment reuses a learning-mode object; none forks a new artifact.
    pub reuses_learning_mode_objects_only: bool,
    /// Exit, leave, and crash recovery all restore the prior environment.
    pub restore_on_exit_guaranteed: bool,
}

/// Builder for assembling a [`TeachingSession`] with the guardrail flags fixed
/// to their safe values and the opt-in markers derived from the policy/class.
pub struct TeachingSessionBuilder {
    session_id: String,
    session_kind: SessionKind,
    lifecycle_state: SessionLifecycleState,
    local_role: TeachingRole,
    replay_policy: ReplayPolicy,
    retention_class: RetentionClass,
    exercise_pack_refs: Vec<String>,
    current_focus_segment_ref: Option<String>,
    segments: Vec<TeachingSegment>,
    participants: Vec<TeachingParticipant>,
    restore_checkpoint: RestoreCheckpoint,
}

impl TeachingSessionBuilder {
    /// Start a builder. The checkpoint is required up front because entering a
    /// teaching session must checkpoint the prior layout first.
    pub fn new(
        session_id: impl Into<String>,
        session_kind: SessionKind,
        local_role: TeachingRole,
        replay_policy: ReplayPolicy,
        retention_class: RetentionClass,
        restore_checkpoint: RestoreCheckpoint,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            session_kind,
            lifecycle_state: SessionLifecycleState::Active,
            local_role,
            replay_policy,
            retention_class,
            exercise_pack_refs: Vec::new(),
            current_focus_segment_ref: None,
            segments: Vec::new(),
            participants: Vec::new(),
            restore_checkpoint,
        }
    }

    /// Set the lifecycle state (defaults to [`SessionLifecycleState::Active`]).
    pub fn lifecycle(mut self, state: SessionLifecycleState) -> Self {
        self.lifecycle_state = state;
        self
    }

    /// Set the current focus segment ref.
    pub fn focus(mut self, segment_ref: impl Into<String>) -> Self {
        self.current_focus_segment_ref = Some(segment_ref.into());
        self
    }

    /// Append an exercise-pack ref.
    pub fn exercise_pack(mut self, pack_ref: impl Into<String>) -> Self {
        self.exercise_pack_refs.push(pack_ref.into());
        self
    }

    /// Append a teaching segment.
    pub fn segment(mut self, segment: TeachingSegment) -> Self {
        self.segments.push(segment);
        self
    }

    /// Append a participant.
    pub fn participant(mut self, participant: TeachingParticipant) -> Self {
        self.participants.push(participant);
        self
    }

    /// Finish the session with the guardrail flags fixed safe and the opt-in
    /// markers derived from the replay policy and retention class.
    pub fn build(self) -> TeachingSession {
        TeachingSession {
            record_kind: TEACHING_SESSION_RECORD_KIND.to_owned(),
            schema_version: TEACHING_SESSION_BETA_SCHEMA_VERSION,
            shared_contract_ref: TEACHING_SESSION_BETA_SHARED_CONTRACT_REF.to_owned(),
            session_id: self.session_id,
            session_kind: self.session_kind,
            lifecycle_state: self.lifecycle_state,
            local_role: self.local_role,
            replay_policy: self.replay_policy,
            retention_class: self.retention_class,
            exercise_pack_refs: self.exercise_pack_refs,
            current_focus_segment_ref: self.current_focus_segment_ref,
            segments: self.segments,
            participants: self.participants,
            restore_checkpoint: self.restore_checkpoint,
            replay_archive_opt_in_explicit: self.replay_policy.requires_explicit_opt_in(),
            shared_retention_opt_in_explicit: self.retention_class.requires_explicit_opt_in(),
            grants_mutation_authority: false,
            grants_terminal_or_debug_control: false,
            grants_broader_authority_than_workspace: false,
            establishes_private_data_ownership: false,
            creates_hidden_progress_model: false,
            creates_cohort_or_grading_flow: false,
            demonstrations_non_mutating_by_default: true,
            preserves_source_citations: true,
            reuses_learning_mode_objects_only: true,
            restore_on_exit_guaranteed: true,
        }
    }
}

impl TeachingSession {
    /// True when every segment cites a learning-mode object and at least one
    /// docs / graph / citation reference.
    pub fn segments_cite_learning_mode(&self) -> bool {
        self.segments
            .iter()
            .all(|s| s.cites_learning_mode_object && !s.learning_object_ref.is_empty() && s.has_citation())
    }

    /// True when every segment resumes across restart and reconnect.
    pub fn segments_are_resumable(&self) -> bool {
        self.segments.iter().all(|s| s.is_resumable())
    }

    /// True when every degraded docs-pack state carries its disclosure.
    pub fn docs_pack_states_disclosed(&self) -> bool {
        self.segments.iter().all(|s| s.docs_pack_disclosure_ok())
    }

    /// True when every demonstrated action is non-mutating, or a properly fenced
    /// mutation that reuses the ordinary command path.
    pub fn demonstrations_are_fenced(&self) -> bool {
        self.segments
            .iter()
            .filter_map(|s| s.demonstrated_action.as_ref())
            .all(|a| a.is_properly_fenced())
    }

    /// True when the replay/retention opt-in markers agree with the policy and
    /// class: a shared archive or shared retention must be explicitly opted in,
    /// and a non-shared one must not carry an opt-in marker.
    pub fn opt_in_markers_consistent(&self) -> bool {
        self.replay_archive_opt_in_explicit == self.replay_policy.requires_explicit_opt_in()
            && self.shared_retention_opt_in_explicit
                == self.retention_class.requires_explicit_opt_in()
    }
}

/// What triggered a restore. Each trigger restores the same checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreTrigger {
    /// The moderator exited the teaching session cleanly.
    Exit,
    /// A participant left the classroom.
    Leave,
    /// Crash recovery rehydrated the session and restored the prior layout.
    CrashRecovery,
}

impl RestoreTrigger {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exit => "exit",
            Self::Leave => "leave",
            Self::CrashRecovery => "crash_recovery",
        }
    }

    /// The lifecycle state a session lands in after this trigger.
    pub const fn restored_lifecycle(self) -> SessionLifecycleState {
        match self {
            Self::Exit => SessionLifecycleState::ExitedRestored,
            Self::Leave => SessionLifecycleState::LeftRestored,
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
pub struct TeachingRestoreOutcome {
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
/// Exit, leave, and crash recovery all replay the same checkpoint, so the
/// resulting environment is identical regardless of how the session ended.
pub fn restore_from_checkpoint(
    session: &TeachingSession,
    trigger: RestoreTrigger,
) -> TeachingRestoreOutcome {
    let cp = &session.restore_checkpoint;
    TeachingRestoreOutcome {
        record_kind: TEACHING_RESTORE_OUTCOME_RECORD_KIND.to_owned(),
        schema_version: TEACHING_SESSION_BETA_SCHEMA_VERSION,
        shared_contract_ref: TEACHING_SESSION_BETA_SHARED_CONTRACT_REF.to_owned(),
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
