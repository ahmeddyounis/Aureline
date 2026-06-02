//! Role-aware control affordances projected from a teaching session.
//!
//! Where [`super::session`] holds the governed boundary object, this module
//! projects the *controls* each seat actually sees. The projection is the proof
//! surface for two acceptance clauses that are easy to claim and hard to trust:
//!
//! - **Teaching roles stay separate from terminal / debug control and never
//!   imply broader authority.** No projected affordance is ever a terminal or
//!   debug control; only a [`TeachingRole::Moderator`] sees drive controls; and
//!   any mutation affordance routes through the ordinary command path
//!   (`routes_through_ordinary_command_path = true`) rather than a teaching
//!   shortcut.
//! - **Limited and low-bandwidth clients join cleanly as observers or
//!   note-takers without broken or misleading control affordances.** A
//!   constrained client is never handed a drive or mutation control it cannot
//!   use; heavy live affordances are *omitted* rather than rendered disabled, so
//!   nothing on screen lies about what it can do.
//!
//! The projection generates only affordances that are actually actionable for a
//! given (role, client) pair, so a reviewer can read "no broken controls" off
//! the absence of disabled rows rather than trusting a flag.

use serde::{Deserialize, Serialize};

use super::session::{
    ClientClass, DemonstratedAction, TeachingParticipant, TeachingRole, TeachingSession,
    TEACHING_SESSION_BETA_SCHEMA_VERSION, TEACHING_SESSION_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`TeachingAffordanceProjection`] payloads.
pub const TEACHING_AFFORDANCE_PROJECTION_RECORD_KIND: &str =
    "teaching_affordance_projection_record";

/// Synthetic participant id used for the local user's own affordance view.
pub const LOCAL_PARTICIPANT_ID: &str = "teaching:participant:local";

/// The kind of control an affordance exposes. The set is deliberately small and
/// closed: there is no terminal, debug, or authority-granting control kind,
/// because a teaching role never grants one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffordanceKind {
    /// Advance to the next teaching segment. Drive control; moderator only.
    AdvanceSegment,
    /// Spotlight the current segment's surface. Drive control; moderator only.
    SpotlightSegment,
    /// Open the cited docs source. Read-only; available to every seat.
    OpenCitedDocs,
    /// Take a shared note. Note-taking; moderator, participant, or scribe.
    TakeNote,
    /// Prepare a mutation preview through the ordinary command path. Mutation
    /// affordance; moderator, participant, or approver, full clients only.
    PrepareMutationPreview,
    /// Approve a demonstrated mutation through the ordinary approval fence.
    /// Mutation affordance; approver, full clients only.
    ApproveMutation,
    /// Exit the teaching session (moderator) and restore the prior workspace.
    ExitSession,
    /// Leave the session (non-moderator) and restore the prior workspace.
    LeaveSession,
}

impl AffordanceKind {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvanceSegment => "advance_segment",
            Self::SpotlightSegment => "spotlight_segment",
            Self::OpenCitedDocs => "open_cited_docs",
            Self::TakeNote => "take_note",
            Self::PrepareMutationPreview => "prepare_mutation_preview",
            Self::ApproveMutation => "approve_mutation",
            Self::ExitSession => "exit_session",
            Self::LeaveSession => "leave_session",
        }
    }

    /// True when the affordance drives the session (only the moderator drives).
    pub const fn is_drive(self) -> bool {
        matches!(self, Self::AdvanceSegment | Self::SpotlightSegment)
    }

    /// True when the affordance authorizes or applies a workspace mutation.
    pub const fn is_mutation(self) -> bool {
        matches!(self, Self::PrepareMutationPreview | Self::ApproveMutation)
    }

    /// Always `false`: no affordance kind is a terminal or debug control.
    pub const fn is_terminal_or_debug_control(self) -> bool {
        false
    }
}

/// One projected control. Generated only when it is actionable for the seat, so
/// every projected affordance is enabled and honest by construction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingControlAffordance {
    pub affordance_id: String,
    pub kind: AffordanceKind,
    /// Ordinary command id this control invokes (never a teaching shortcut).
    pub command_id: String,
    /// Keyboard binding ref so the control is reachable without a pointer.
    pub key_binding_ref: String,
    /// Accessible label ref so the control is reachable by a screen reader.
    pub accessible_label_ref: String,
    /// Preview sheet ref reused from ordinary work (mutation affordances only).
    pub preview_sheet_ref: Option<String>,
    /// Approval path ref reused from ordinary work (mutation affordances only).
    pub approval_path_ref: Option<String>,
    /// Rollback semantics ref reused from ordinary work (mutation affordances
    /// only).
    pub rollback_semantics_ref: Option<String>,
    /// True when the control authorizes or applies a workspace mutation.
    pub mutates_workspace: bool,
    /// Always `true`: the control routes through the ordinary command path.
    pub routes_through_ordinary_command_path: bool,
    /// Always `false`: the control is never a terminal or debug control.
    pub is_terminal_or_debug_control: bool,
    /// Always `true`: only actionable controls are projected, so none is broken.
    pub actionable: bool,
}

/// The affordances one seat (the local user or one participant) actually sees.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParticipantAffordanceView {
    pub participant_id: String,
    pub role: TeachingRole,
    pub client_class: ClientClass,
    pub is_local_user: bool,
    pub affordances: Vec<TeachingControlAffordance>,
    /// True when the seat exposes any drive control (moderator only).
    pub exposes_drive_control: bool,
    /// True when the seat exposes any mutation affordance.
    pub exposes_mutation_affordance: bool,
    /// True when the seat can take notes.
    pub can_take_notes: bool,
    /// Always `false`: no seat exposes a terminal or debug control.
    pub exposes_terminal_or_debug_control: bool,
    /// True when a constrained client is kept safe: no drive or mutation control
    /// is exposed to it, so it can join as an observer or note-taker without a
    /// broken or misleading affordance. Always `true` for a full client.
    pub constrained_client_join_safe: bool,
}

/// The full role-aware affordance projection for a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingAffordanceProjection {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub session_id: String,
    pub participant_views: Vec<ParticipantAffordanceView>,
    /// Always `true`: every projected control is keyboard-reachable.
    pub keyboard_complete: bool,
    /// Always `false`: no control is pointer-only.
    pub pointer_only: bool,
    /// Always `true`: every projected control is screen-reader reachable.
    pub screen_reader_reachable: bool,
    /// Always `false`: no seat exposes a terminal or debug control.
    pub exposes_terminal_or_debug_control: bool,
    /// Always `false`: no projected control is broken or misleading.
    pub exposes_misleading_control: bool,
    /// True when every constrained-client seat joins safely.
    pub all_constrained_clients_join_safe: bool,
}

impl TeachingAffordanceProjection {
    /// Iterate every projected affordance across every seat.
    pub fn all_affordances(&self) -> impl Iterator<Item = &TeachingControlAffordance> {
        self.participant_views
            .iter()
            .flat_map(|v| v.affordances.iter())
    }
}

/// The first mutating demonstration in the session, if any. Its ordinary-work
/// refs back the mutation affordances so they reuse the same command path.
fn mutating_demonstration(session: &TeachingSession) -> Option<&DemonstratedAction> {
    session
        .segments
        .iter()
        .filter_map(|s| s.demonstrated_action.as_ref())
        .find(|a| a.mutates_workspace)
}

fn affordance(seat: &str, kind: AffordanceKind, command_id: &str) -> TeachingControlAffordance {
    TeachingControlAffordance {
        affordance_id: format!("aff:{seat}:{}", kind.as_str()),
        kind,
        command_id: command_id.to_owned(),
        key_binding_ref: format!("keybinding:teaching.{}", kind.as_str()),
        accessible_label_ref: format!("msg:teaching.affordance.{}", kind.as_str()),
        preview_sheet_ref: None,
        approval_path_ref: None,
        rollback_semantics_ref: None,
        mutates_workspace: false,
        routes_through_ordinary_command_path: true,
        is_terminal_or_debug_control: false,
        actionable: true,
    }
}

/// Build the affordances a single (role, client) seat sees in `session`.
fn affordances_for(
    participant_id: &str,
    role: TeachingRole,
    client_class: ClientClass,
    session: &TeachingSession,
) -> Vec<TeachingControlAffordance> {
    let mut out = Vec::new();
    let constrained = client_class.is_constrained();
    let mutation = mutating_demonstration(session);

    // Drive controls: moderator only, and only on a full client. A constrained
    // client never gets heavy live drive controls — they are omitted, not shown
    // disabled.
    if role.can_drive_session() && !constrained {
        out.push(affordance(
            participant_id,
            AffordanceKind::AdvanceSegment,
            "cmd:teaching.advance_segment",
        ));
        out.push(affordance(
            participant_id,
            AffordanceKind::SpotlightSegment,
            "cmd:teaching.spotlight_segment",
        ));
    }

    // Open cited docs: read-only, available to every seat including observers
    // and constrained clients.
    out.push(affordance(
        participant_id,
        AffordanceKind::OpenCitedDocs,
        "cmd:docs.open_in_browser",
    ));

    // Note-taking: moderator, participant, or scribe. Low-bandwidth friendly, so
    // it stays available on constrained clients.
    if role.can_take_notes() {
        out.push(affordance(
            participant_id,
            AffordanceKind::TakeNote,
            "cmd:teaching.take_note",
        ));
    }

    // Mutation affordances exist only when a mutation is actually demonstrated,
    // for roles that may expose one, and only on full clients. They reuse the
    // demonstration's ordinary command path — never a teaching shortcut.
    if let Some(demo) = mutation {
        if !constrained && role.may_expose_mutation_affordance() {
            let command_id = demo
                .command_id
                .clone()
                .unwrap_or_else(|| "cmd:workspace.import_profile".to_owned());
            if role == TeachingRole::Approver {
                let mut approve =
                    affordance(participant_id, AffordanceKind::ApproveMutation, &command_id);
                approve.preview_sheet_ref = demo.preview_sheet_ref.clone();
                approve.approval_path_ref = demo.approval_path_ref.clone();
                approve.rollback_semantics_ref = demo.rollback_semantics_ref.clone();
                approve.mutates_workspace = true;
                out.push(approve);
            } else {
                let mut prepare = affordance(
                    participant_id,
                    AffordanceKind::PrepareMutationPreview,
                    &command_id,
                );
                prepare.preview_sheet_ref = demo.preview_sheet_ref.clone();
                prepare.approval_path_ref = demo.approval_path_ref.clone();
                prepare.rollback_semantics_ref = demo.rollback_semantics_ref.clone();
                prepare.mutates_workspace = true;
                out.push(prepare);
            }
        }
    }

    // Exit / leave: the moderator exits, everyone else leaves. Either way the
    // prior workspace is restored, and the control is available to every client.
    let exit_kind = if role.can_drive_session() {
        AffordanceKind::ExitSession
    } else {
        AffordanceKind::LeaveSession
    };
    let exit_command = if role.can_drive_session() {
        "cmd:teaching.exit_session"
    } else {
        "cmd:teaching.leave_session"
    };
    out.push(affordance(participant_id, exit_kind, exit_command));

    out
}

fn build_view(
    participant_id: &str,
    role: TeachingRole,
    client_class: ClientClass,
    is_local_user: bool,
    session: &TeachingSession,
) -> ParticipantAffordanceView {
    let affordances = affordances_for(participant_id, role, client_class, session);
    let exposes_drive_control = affordances.iter().any(|a| a.kind.is_drive());
    let exposes_mutation_affordance = affordances.iter().any(|a| a.mutates_workspace);
    let can_take_notes = affordances
        .iter()
        .any(|a| a.kind == AffordanceKind::TakeNote);

    // A constrained client is safe when it is never handed a drive or mutation
    // control it cannot use. A full client is always considered safe.
    let constrained_client_join_safe = if client_class.is_constrained() {
        !exposes_drive_control && !exposes_mutation_affordance
    } else {
        true
    };

    ParticipantAffordanceView {
        participant_id: participant_id.to_owned(),
        role,
        client_class,
        is_local_user,
        affordances,
        exposes_drive_control,
        exposes_mutation_affordance,
        can_take_notes,
        exposes_terminal_or_debug_control: false,
        constrained_client_join_safe,
    }
}

/// Project the role-aware control affordances for `session`.
///
/// The local user is projected as a full-client seat under
/// [`TeachingSession::local_role`]; each [`TeachingParticipant`] is projected
/// under its own role and client class.
pub fn project_affordances(session: &TeachingSession) -> TeachingAffordanceProjection {
    let mut participant_views = Vec::with_capacity(session.participants.len() + 1);
    participant_views.push(build_view(
        LOCAL_PARTICIPANT_ID,
        session.local_role,
        ClientClass::Full,
        true,
        session,
    ));
    for p in &session.participants {
        let TeachingParticipant {
            participant_id,
            role,
            client_class,
            ..
        } = p;
        participant_views.push(build_view(
            participant_id,
            *role,
            *client_class,
            false,
            session,
        ));
    }

    let all_constrained_clients_join_safe = participant_views
        .iter()
        .all(|v| v.constrained_client_join_safe);

    TeachingAffordanceProjection {
        record_kind: TEACHING_AFFORDANCE_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: TEACHING_SESSION_BETA_SCHEMA_VERSION,
        shared_contract_ref: TEACHING_SESSION_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session.session_id.clone(),
        participant_views,
        keyboard_complete: true,
        pointer_only: false,
        screen_reader_reachable: true,
        exposes_terminal_or_debug_control: false,
        exposes_misleading_control: false,
        all_constrained_clients_join_safe,
    }
}
