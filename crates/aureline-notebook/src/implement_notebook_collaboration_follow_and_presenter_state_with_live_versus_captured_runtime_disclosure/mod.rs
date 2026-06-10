//! Notebook collaboration follow and presenter state with live-versus-captured runtime disclosure.
//!
//! This module materializes the typed records that keep notebook collaboration
//! honest about follow state, presenter authority, and the boundary between live
//! runtime state and captured output. The records and closed vocabularies here
//! mirror the boundary schema at
//! `/schemas/notebook/implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure.schema.json`
//! and reuse the session-role and admission vocabulary already frozen in
//! `/schemas/collab/session_role_admission_and_retention_qualification.schema.json`.
//!
//! The module exposes:
//!
//! - the [`NotebookCollaborationFollowState`] record that carries per-participant
//!   follow posture — who they follow, what cell or output is in view, whether
//!   they are independent or breakaway, and why follow degraded — so the
//!   collaboration surface never silently loses track of a participant;
//! - the [`NotebookPresenterState`] record that carries presenter identity,
//!   presenter mode, shared cell or output refs, and available presenter actions
//!   so the audience always knows who is driving and what scope is shared;
//! - the [`NotebookRuntimeDisclosure`] record that makes the live-versus-captured
//!   boundary explicit for every collaborative view, with disclosure class,
//!   kernel session ref, capture timestamp, and available disclosure actions so
//!   participants never mistake a saved notebook for a live runtime session;
//! - the [`NotebookCollaborationFollowPresenterPacket`] checked-in artifact that
//!   downstream docs, help, support, and CI surfaces ingest instead of cloning
//!   status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every collaboration follow/presenter/runtime-
/// disclosure record carried by this module. Bumped only on breaking payload
/// changes; additive-optional fields do not bump this value.
pub const NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookCollaborationFollowState`] payloads.
pub const NOTEBOOK_COLLABORATION_FOLLOW_STATE_RECORD_KIND: &str =
    "notebook_collaboration_follow_state";

/// Stable record-kind tag for serialized [`NotebookPresenterState`] payloads.
pub const NOTEBOOK_PRESENTER_STATE_RECORD_KIND: &str = "notebook_presenter_state";

/// Stable record-kind tag for serialized [`NotebookRuntimeDisclosure`] payloads.
pub const NOTEBOOK_RUNTIME_DISCLOSURE_RECORD_KIND: &str = "notebook_runtime_disclosure";

/// Stable record-kind tag for the checked-in [`NotebookCollaborationFollowPresenterPacket`].
pub const NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_PACKET_RECORD_KIND: &str =
    "notebook_collaboration_follow_presenter_packet";

/// Repo-relative path to the checked-in collaboration follow/presenter/runtime-
/// disclosure packet JSON.
pub const NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure.json";

/// Embedded checked-in collaboration follow/presenter/runtime-disclosure packet JSON.
pub const NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Follow-mode class. Names the participant's follow posture so the
    /// collaboration surface never silently loses track of a participant.
    NotebookFollowMode {
        FollowingPresenter => "following_presenter",
        Independent => "independent",
        Breakaway => "breakaway",
        ReturnAvailable => "return_available",
        FollowDegraded => "follow_degraded",
    }
);

closed_vocab!(
    /// Follow-target class. Names what kind of entity a participant is
    /// following so consumers know the scope of the follow binding.
    NotebookFollowTargetClass {
        Presenter => "presenter",
        SpecificCell => "specific_cell",
        SpecificOutput => "specific_output",
        SpecificParticipant => "specific_participant",
    }
);

closed_vocab!(
    /// Presenter-mode class. Names the lifecycle state of the presenter
    /// role so the audience always knows who is driving and whether a
    /// handoff is in progress.
    NotebookPresenterMode {
        ActivePresenter => "active_presenter",
        CoPresenter => "co_presenter",
        Idle => "idle",
        HandoffPending => "handoff_pending",
    }
);

closed_vocab!(
    /// Presenter-action class. Names the actions a presenter may take so
    /// the UI can show safe, scoped affordances instead of broad authority.
    NotebookPresenterActionClass {
        ShareScreen => "share_screen",
        ShareCell => "share_cell",
        ShareOutput => "share_output",
        Handoff => "handoff",
        Pause => "pause",
        Resume => "resume",
    }
);

closed_vocab!(
    /// Runtime-disclosure class. Names whether a collaborative view is
    /// showing live runtime state, captured output, a mix, stale runtime,
    /// or no kernel so participants never mistake a saved notebook for a
    /// live runtime session.
    NotebookRuntimeDisclosureClass {
        LiveRuntime => "live_runtime",
        CapturedOutput => "captured_output",
        MixedState => "mixed_state",
        StaleRuntime => "stale_runtime",
        NoKernel => "no_kernel",
    }
);

closed_vocab!(
    /// Runtime-disclosure-action class. Names the actions available when
    /// the live-versus-captured boundary changes so the UI can offer safe
    /// transitions instead of silent swaps.
    NotebookRuntimeDisclosureActionClass {
        RefreshRuntime => "refresh_runtime",
        AcknowledgeCaptured => "acknowledge_captured",
        SwitchToLive => "switch_to_live",
        SwitchToCaptured => "switch_to_captured",
        RequestKernel => "request_kernel",
    }
);

/// Generic finding shape used by every collaboration follow/presenter/runtime-
/// disclosure validator. Mirrors the finding shapes other Aureline crates
/// expose so a single review/audit/support pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationFollowPresenterFinding {
    /// Stable check id (e.g. `notebook_collaboration_follow_state.mode_required`).
    pub check_id: String,
    /// Subject row id (record id, session id, participant id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl CollaborationFollowPresenterFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookCollaborationFollowState`].
pub type NotebookCollaborationFollowStateFinding = CollaborationFollowPresenterFinding;

/// Typed validation finding for a [`NotebookPresenterState`].
pub type NotebookPresenterStateFinding = CollaborationFollowPresenterFinding;

/// Typed validation finding for a [`NotebookRuntimeDisclosure`].
pub type NotebookRuntimeDisclosureFinding = CollaborationFollowPresenterFinding;

/// Typed validation finding for a [`NotebookCollaborationFollowPresenterPacket`].
pub type NotebookCollaborationFollowPresenterPacketFinding = CollaborationFollowPresenterFinding;

/// Notebook collaboration follow-state record. Carries per-participant follow
/// posture — who they follow, what cell or output is in view, whether they are
/// independent or breakaway, and why follow degraded — so the collaboration
/// surface never silently loses track of a participant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCollaborationFollowState {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_collaboration_follow_presenter_schema_version: u32,
    /// Stable opaque follow-state id.
    pub follow_state_id: String,
    /// Opaque ref to the notebook document this follow state belongs to.
    pub document_id_ref: String,
    /// Opaque ref to the collaboration session envelope.
    pub session_envelope_ref: String,
    /// Opaque ref to the participant actor this record describes.
    pub participant_ref: String,
    /// Follow-mode class.
    pub follow_mode: NotebookFollowMode,
    /// Follow-target class.
    pub follow_target_class: NotebookFollowTargetClass,
    /// Opaque ref to the target being followed (presenter, cell, output, or participant).
    pub follow_target_ref: String,
    /// Opaque ref to the current cell id in the participant's view.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_cell_id_ref: Option<String>,
    /// Opaque ref to the current output handle in the participant's view.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_output_handle_ref: Option<String>,
    /// Export-safe explanation when follow_mode is [`NotebookFollowMode::Breakaway`] or [`NotebookFollowMode::FollowDegraded`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_explanation: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCollaborationFollowState {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookCollaborationFollowStateFinding> {
        let mut findings = Vec::new();
        let subject = self.follow_state_id.as_str();

        if self.record_kind != NOTEBOOK_COLLABORATION_FOLLOW_STATE_RECORD_KIND {
            findings.push(NotebookCollaborationFollowStateFinding::new(
                "notebook_collaboration_follow_state.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_COLLABORATION_FOLLOW_STATE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_collaboration_follow_presenter_schema_version
            != NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION
        {
            findings.push(NotebookCollaborationFollowStateFinding::new(
                "notebook_collaboration_follow_state.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION}, found {}",
                    self.notebook_collaboration_follow_presenter_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookCollaborationFollowStateFinding::new(
                "notebook_collaboration_follow_state.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.session_envelope_ref.trim().is_empty() {
            findings.push(NotebookCollaborationFollowStateFinding::new(
                "notebook_collaboration_follow_state.session_envelope_ref_required",
                subject,
                "session_envelope_ref must be non-empty",
            ));
        }
        if self.participant_ref.trim().is_empty() {
            findings.push(NotebookCollaborationFollowStateFinding::new(
                "notebook_collaboration_follow_state.participant_ref_required",
                subject,
                "participant_ref must be non-empty",
            ));
        }
        if self.follow_target_ref.trim().is_empty() {
            findings.push(NotebookCollaborationFollowStateFinding::new(
                "notebook_collaboration_follow_state.follow_target_ref_required",
                subject,
                "follow_target_ref must be non-empty",
            ));
        }

        if (self.follow_mode == NotebookFollowMode::Breakaway
            || self.follow_mode == NotebookFollowMode::FollowDegraded)
            && self.follow_explanation.is_none()
        {
            findings.push(NotebookCollaborationFollowStateFinding::new(
                "notebook_collaboration_follow_state.follow_explanation_required",
                subject,
                "follow_explanation must be Some when follow_mode is breakaway or follow_degraded",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookCollaborationFollowStateFinding::new(
                "notebook_collaboration_follow_state.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook presenter-state record. Carries presenter identity, presenter mode,
/// shared cell or output refs, and available presenter actions so the audience
/// always knows who is driving and what scope is shared.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookPresenterState {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_collaboration_follow_presenter_schema_version: u32,
    /// Stable opaque presenter-state id.
    pub presenter_state_id: String,
    /// Opaque ref to the notebook document this presenter state belongs to.
    pub document_id_ref: String,
    /// Opaque ref to the collaboration session envelope.
    pub session_envelope_ref: String,
    /// Presenter-mode class.
    pub presenter_mode: NotebookPresenterMode,
    /// Opaque ref to the actor who holds the presenter role.
    pub presenter_actor_ref: String,
    /// Opaque ref to the cell currently shared by the presenter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_cell_id_ref: Option<String>,
    /// Opaque ref to the output currently shared by the presenter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_output_handle_ref: Option<String>,
    /// Available presenter actions for the current mode.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presenter_actions: Vec<NotebookPresenterActionClass>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookPresenterState {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookPresenterStateFinding> {
        let mut findings = Vec::new();
        let subject = self.presenter_state_id.as_str();

        if self.record_kind != NOTEBOOK_PRESENTER_STATE_RECORD_KIND {
            findings.push(NotebookPresenterStateFinding::new(
                "notebook_presenter_state.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_PRESENTER_STATE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_collaboration_follow_presenter_schema_version
            != NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION
        {
            findings.push(NotebookPresenterStateFinding::new(
                "notebook_presenter_state.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION}, found {}",
                    self.notebook_collaboration_follow_presenter_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookPresenterStateFinding::new(
                "notebook_presenter_state.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.session_envelope_ref.trim().is_empty() {
            findings.push(NotebookPresenterStateFinding::new(
                "notebook_presenter_state.session_envelope_ref_required",
                subject,
                "session_envelope_ref must be non-empty",
            ));
        }
        if self.presenter_actor_ref.trim().is_empty() {
            findings.push(NotebookPresenterStateFinding::new(
                "notebook_presenter_state.presenter_actor_ref_required",
                subject,
                "presenter_actor_ref must be non-empty",
            ));
        }

        if self.presenter_mode == NotebookPresenterMode::ActivePresenter
            && self.shared_cell_id_ref.is_none()
            && self.shared_output_handle_ref.is_none()
        {
            findings.push(NotebookPresenterStateFinding::new(
                "notebook_presenter_state.shared_ref_required_when_active",
                subject,
                "shared_cell_id_ref or shared_output_handle_ref must be Some when presenter_mode is active_presenter",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookPresenterStateFinding::new(
                "notebook_presenter_state.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook runtime-disclosure record. Makes the live-versus-captured boundary
/// explicit for every collaborative view, with disclosure class, kernel session
/// ref, capture timestamp, and available disclosure actions so participants
/// never mistake a saved notebook for a live runtime session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRuntimeDisclosure {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_collaboration_follow_presenter_schema_version: u32,
    /// Stable opaque runtime-disclosure id.
    pub runtime_disclosure_id: String,
    /// Opaque ref to the notebook document this disclosure belongs to.
    pub document_id_ref: String,
    /// Runtime-disclosure class.
    pub disclosure_class: NotebookRuntimeDisclosureClass,
    /// Opaque ref to the live kernel session, when [`NotebookRuntimeDisclosureClass::LiveRuntime`] or [`NotebookRuntimeDisclosureClass::StaleRuntime`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_ref: Option<String>,
    /// UTC timestamp when the captured state was frozen, when [`NotebookRuntimeDisclosureClass::CapturedOutput`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_at: Option<String>,
    /// Available disclosure actions for the current class.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disclosure_actions: Vec<NotebookRuntimeDisclosureActionClass>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookRuntimeDisclosure {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookRuntimeDisclosureFinding> {
        let mut findings = Vec::new();
        let subject = self.runtime_disclosure_id.as_str();

        if self.record_kind != NOTEBOOK_RUNTIME_DISCLOSURE_RECORD_KIND {
            findings.push(NotebookRuntimeDisclosureFinding::new(
                "notebook_runtime_disclosure.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_RUNTIME_DISCLOSURE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_collaboration_follow_presenter_schema_version
            != NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION
        {
            findings.push(NotebookRuntimeDisclosureFinding::new(
                "notebook_runtime_disclosure.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION}, found {}",
                    self.notebook_collaboration_follow_presenter_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookRuntimeDisclosureFinding::new(
                "notebook_runtime_disclosure.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }

        if (self.disclosure_class == NotebookRuntimeDisclosureClass::LiveRuntime
            || self.disclosure_class == NotebookRuntimeDisclosureClass::StaleRuntime)
            && self.kernel_session_ref.is_none()
        {
            findings.push(NotebookRuntimeDisclosureFinding::new(
                "notebook_runtime_disclosure.kernel_session_ref_required",
                subject,
                "kernel_session_ref must be Some when disclosure_class is live_runtime or stale_runtime",
            ));
        }

        if self.disclosure_class == NotebookRuntimeDisclosureClass::CapturedOutput
            && self.captured_at.is_none()
        {
            findings.push(NotebookRuntimeDisclosureFinding::new(
                "notebook_runtime_disclosure.captured_at_required",
                subject,
                "captured_at must be Some when disclosure_class is captured_output",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookRuntimeDisclosureFinding::new(
                "notebook_runtime_disclosure.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Checked-in collaboration follow/presenter/runtime-disclosure packet that
/// downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCollaborationFollowPresenterPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: follow modes.
    pub follow_modes: Vec<NotebookFollowMode>,
    /// Closed vocabulary: follow target classes.
    pub follow_target_classes: Vec<NotebookFollowTargetClass>,
    /// Closed vocabulary: presenter modes.
    pub presenter_modes: Vec<NotebookPresenterMode>,
    /// Closed vocabulary: presenter actions.
    pub presenter_actions: Vec<NotebookPresenterActionClass>,
    /// Closed vocabulary: runtime disclosure classes.
    pub runtime_disclosure_classes: Vec<NotebookRuntimeDisclosureClass>,
    /// Closed vocabulary: runtime disclosure actions.
    pub runtime_disclosure_actions: Vec<NotebookRuntimeDisclosureActionClass>,
    /// Worked example follow states.
    pub example_follow_states: Vec<NotebookCollaborationFollowState>,
    /// Worked example presenter states.
    pub example_presenter_states: Vec<NotebookPresenterState>,
    /// Worked example runtime disclosures.
    pub example_runtime_disclosures: Vec<NotebookRuntimeDisclosure>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCollaborationFollowPresenterPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookCollaborationFollowPresenterPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION {
            findings.push(NotebookCollaborationFollowPresenterPacketFinding::new(
                "notebook_collaboration_follow_presenter_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_PACKET_RECORD_KIND {
            findings.push(NotebookCollaborationFollowPresenterPacketFinding::new(
                "notebook_collaboration_follow_presenter_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.follow_modes.len() != NotebookFollowMode::ALL.len() {
            findings.push(NotebookCollaborationFollowPresenterPacketFinding::new(
                "notebook_collaboration_follow_presenter_packet.follow_modes_coverage",
                subject,
                "follow_modes must list every variant",
            ));
        }
        if self.follow_target_classes.len() != NotebookFollowTargetClass::ALL.len() {
            findings.push(NotebookCollaborationFollowPresenterPacketFinding::new(
                "notebook_collaboration_follow_presenter_packet.follow_target_classes_coverage",
                subject,
                "follow_target_classes must list every variant",
            ));
        }
        if self.presenter_modes.len() != NotebookPresenterMode::ALL.len() {
            findings.push(NotebookCollaborationFollowPresenterPacketFinding::new(
                "notebook_collaboration_follow_presenter_packet.presenter_modes_coverage",
                subject,
                "presenter_modes must list every variant",
            ));
        }
        if self.presenter_actions.len() != NotebookPresenterActionClass::ALL.len() {
            findings.push(NotebookCollaborationFollowPresenterPacketFinding::new(
                "notebook_collaboration_follow_presenter_packet.presenter_actions_coverage",
                subject,
                "presenter_actions must list every variant",
            ));
        }
        if self.runtime_disclosure_classes.len() != NotebookRuntimeDisclosureClass::ALL.len() {
            findings.push(NotebookCollaborationFollowPresenterPacketFinding::new(
                "notebook_collaboration_follow_presenter_packet.runtime_disclosure_classes_coverage",
                subject,
                "runtime_disclosure_classes must list every variant",
            ));
        }
        if self.runtime_disclosure_actions.len() != NotebookRuntimeDisclosureActionClass::ALL.len()
        {
            findings.push(NotebookCollaborationFollowPresenterPacketFinding::new(
                "notebook_collaboration_follow_presenter_packet.runtime_disclosure_actions_coverage",
                subject,
                "runtime_disclosure_actions must list every variant",
            ));
        }

        for follow_state in &self.example_follow_states {
            findings.extend(follow_state.validate().into_iter().map(|f| {
                NotebookCollaborationFollowPresenterPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for presenter_state in &self.example_presenter_states {
            findings.extend(presenter_state.validate().into_iter().map(|f| {
                NotebookCollaborationFollowPresenterPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for disclosure in &self.example_runtime_disclosures {
            findings.extend(disclosure.validate().into_iter().map(|f| {
                NotebookCollaborationFollowPresenterPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }

        findings
    }
}

/// Parses the checked-in collaboration follow/presenter/runtime-disclosure packet JSON.
pub fn current_notebook_collaboration_follow_presenter_packet(
) -> Result<NotebookCollaborationFollowPresenterPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_COLLABORATION_FOLLOW_PRESENTER_PACKET_JSON)
}

impl NotebookFollowMode {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FollowingPresenter,
        Self::Independent,
        Self::Breakaway,
        Self::ReturnAvailable,
        Self::FollowDegraded,
    ];
}

impl NotebookFollowTargetClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Presenter,
        Self::SpecificCell,
        Self::SpecificOutput,
        Self::SpecificParticipant,
    ];
}

impl NotebookPresenterMode {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ActivePresenter,
        Self::CoPresenter,
        Self::Idle,
        Self::HandoffPending,
    ];
}

impl NotebookPresenterActionClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShareScreen,
        Self::ShareCell,
        Self::ShareOutput,
        Self::Handoff,
        Self::Pause,
        Self::Resume,
    ];
}

impl NotebookRuntimeDisclosureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveRuntime,
        Self::CapturedOutput,
        Self::MixedState,
        Self::StaleRuntime,
        Self::NoKernel,
    ];
}

impl NotebookRuntimeDisclosureActionClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RefreshRuntime,
        Self::AcknowledgeCaptured,
        Self::SwitchToLive,
        Self::SwitchToCaptured,
        Self::RequestKernel,
    ];
}

#[cfg(test)]
mod tests;
