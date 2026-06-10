//! Notebook teaching continuity, checkpointed execution, and redaction-before-share flows.
//!
//! This module materializes the typed records that keep notebook teaching flows
//! honest about checkpoint preference, sandbox posture, and rollback truth;
//! checkpointed execution honest about save-point identity, sandbox state, and
//! replay posture; and redaction-before-share honest about what was redacted,
//! why, and under what trigger so the sharing surface never silently leaks
//! sensitive outputs.
//!
//! The module exposes:
//!
//! - the [`NotebookTeachingContinuity`] record that carries a teaching flow’s
//!   mode, checkpoint preference, guided-exercise step state, and sandbox
//!   requirement so teaching surfaces never run destructive cells without an
//!   explicit safe path;
//! - the [`NotebookCheckpointedExecution`] record that carries an execution
//!   checkpoint’s class, sandbox state, rollback posture, and honest replay
//!   label so users always know whether a checkpoint is available, expired,
//!   orphaned, or replayable;
//! - the [`NotebookRedactionBeforeShare`] record that carries redaction class,
//!   trigger, scope, and explanation so collaboration and presentation surfaces
//!   expose collapse/redaction controls before broad sharing when outputs may
//!   contain sensitive data;
//! - the [`NotebookTeachingContinuityCheckpointedRedactionPacket`] checked-in
//!   artifact that downstream docs, help, support, and CI surfaces ingest
//!   instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookTeachingContinuity`] payloads.
pub const NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND: &str = "notebook_teaching_continuity";

/// Stable record-kind tag for serialized [`NotebookCheckpointedExecution`] payloads.
pub const NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND: &str = "notebook_checkpointed_execution";

/// Stable record-kind tag for serialized [`NotebookRedactionBeforeShare`] payloads.
pub const NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND: &str = "notebook_redaction_before_share";

/// Stable record-kind tag for the checked-in [`NotebookTeachingContinuityCheckpointedRedactionPacket`].
pub const NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_RECORD_KIND: &str =
    "notebook_teaching_continuity_checkpointed_redaction_packet";

/// Repo-relative path to the checked-in teaching-continuity/checkpointed-redaction packet JSON.
pub const NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_PATH: &str =
    "artifacts/notebook/m5/ship_notebook_teaching_continuity_checkpointed_execution_and_redaction_before_share_flows.json";

/// Embedded checked-in teaching-continuity/checkpointed-redaction packet JSON.
pub const NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/ship_notebook_teaching_continuity_checkpointed_execution_and_redaction_before_share_flows.json"
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
    /// Teaching-mode class. Names the pedagogical mode so the teaching
    /// surface never silently degrades a guided exercise into unrestricted
    /// execution.
    NotebookTeachingMode {
        GuidedExercise => "guided_exercise",
        Demo => "demo",
        SoloExploration => "solo_exploration",
        Classroom => "classroom",
        MentorSession => "mentor_session",
    }
);

closed_vocab!(
    /// Checkpoint-preference class. Names whether the teaching flow prefers
    /// auto-checkpoints, manual checkpoints, or no checkpointing so the
    /// runtime can align sandbox and save-point behavior with pedagogy.
    NotebookCheckpointPreference {
        AutoCheckpoint => "auto_checkpoint",
        ManualCheckpoint => "manual_checkpoint",
        NoCheckpoint => "no_checkpoint",
        SandboxOnly => "sandbox_only",
    }
);

closed_vocab!(
    /// Checkpoint class. Names the kind of execution checkpoint so consumers
    /// know whether the save point was created automatically, manually, or
    /// at a specific boundary.
    NotebookCheckpointClass {
        AutoCheckpoint => "auto_checkpoint",
        ManualCheckpoint => "manual_checkpoint",
        PreExecution => "pre_execution",
        PreDestructive => "pre_destructive",
        SandboxBoundary => "sandbox_boundary",
    }
);

closed_vocab!(
    /// Sandbox-state class. Names whether the execution is sandboxed,
    /// unsandboxed, pending sandbox creation, or failed to enter a sandbox.
    NotebookSandboxState {
        Sandboxed => "sandboxed",
        Unsandboxed => "unsandboxed",
        SandboxPending => "sandbox_pending",
        SandboxFailed => "sandbox_failed",
    }
);

closed_vocab!(
    /// Rollback-posture class. Names the honest state of a checkpoint so
    /// the UI never implies a rollback is exact when it is only approximate
    /// or no longer available.
    NotebookRollbackPosture {
        RollbackAvailable => "rollback_available",
        RollbackExpired => "rollback_expired",
        CheckpointOrphaned => "checkpoint_orphaned",
        ExactReplayAvailable => "exact_replay_available",
        CompensatingReplayOnly => "compensating_replay_only",
    }
);

closed_vocab!(
    /// Redaction-class class. Names what category of content was redacted
    /// before share so the recipient knows the scope of omission.
    NotebookRedactionClass {
        OutputRedacted => "output_redacted",
        CellSourceRedacted => "cell_source_redacted",
        MetadataRedacted => "metadata_redacted",
        VariableRedacted => "variable_redacted",
        None => "none",
    }
);

closed_vocab!(
    /// Redaction-trigger class. Names why redaction was applied so the UI
    /// can show truthful labels instead of generic "content hidden" language.
    NotebookRedactionTrigger {
        ManualReview => "manual_review",
        PolicyAuto => "policy_auto",
        SensitivityScan => "sensitivity_scan",
        RecipientMismatch => "recipient_mismatch",
        TeachingSafety => "teaching_safety",
    }
);

/// Generic finding shape used by every validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingContinuityCheckpointedRedactionFinding {
    /// Stable check id (e.g. `notebook_teaching_continuity.mode_required`).
    pub check_id: String,
    /// Subject row id (record id, checkpoint id, redaction id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl TeachingContinuityCheckpointedRedactionFinding {
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

/// Typed validation finding for a [`NotebookTeachingContinuity`].
pub type NotebookTeachingContinuityFinding = TeachingContinuityCheckpointedRedactionFinding;

/// Typed validation finding for a [`NotebookCheckpointedExecution`].
pub type NotebookCheckpointedExecutionFinding = TeachingContinuityCheckpointedRedactionFinding;

/// Typed validation finding for a [`NotebookRedactionBeforeShare`].
pub type NotebookRedactionBeforeShareFinding = TeachingContinuityCheckpointedRedactionFinding;

/// Typed validation finding for a [`NotebookTeachingContinuityCheckpointedRedactionPacket`].
pub type NotebookTeachingContinuityCheckpointedRedactionPacketFinding =
    TeachingContinuityCheckpointedRedactionFinding;

/// Notebook teaching-continuity record. Carries a teaching flow’s mode,
/// checkpoint preference, guided-exercise step state, and sandbox requirement
/// so teaching surfaces never run destructive cells without an explicit safe
/// path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookTeachingContinuity {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_teaching_continuity_checkpointed_redaction_schema_version: u32,
    /// Stable opaque teaching-continuity id.
    pub teaching_continuity_id: String,
    /// Opaque ref to the notebook document this teaching flow belongs to.
    pub document_id_ref: String,
    /// Teaching-mode class.
    pub teaching_mode: NotebookTeachingMode,
    /// Checkpoint-preference class.
    pub checkpoint_preference: NotebookCheckpointPreference,
    /// Current step index in a guided exercise; null when not applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_step_index: Option<u32>,
    /// Total step count in a guided exercise; null when not applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_steps: Option<u32>,
    /// Whether the teaching flow requires sandboxed execution.
    pub sandbox_required: bool,
    /// Export-safe explanation when sandbox is required but unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandbox_unavailable_explanation: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookTeachingContinuity {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookTeachingContinuityFinding> {
        let mut findings = Vec::new();
        let subject = self.teaching_continuity_id.as_str();

        if self.record_kind != NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND {
            findings.push(NotebookTeachingContinuityFinding::new(
                "notebook_teaching_continuity.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_teaching_continuity_checkpointed_redaction_schema_version
            != NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION
        {
            findings.push(NotebookTeachingContinuityFinding::new(
                "notebook_teaching_continuity.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION}, found {}",
                    self.notebook_teaching_continuity_checkpointed_redaction_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookTeachingContinuityFinding::new(
                "notebook_teaching_continuity.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookTeachingContinuityFinding::new(
                "notebook_teaching_continuity.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook checkpointed-execution record. Carries an execution checkpoint’s
/// class, sandbox state, rollback posture, and honest replay label so users
/// always know whether a checkpoint is available, expired, orphaned, or
/// replayable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCheckpointedExecution {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_teaching_continuity_checkpointed_redaction_schema_version: u32,
    /// Stable opaque checkpointed-execution id.
    pub checkpointed_execution_id: String,
    /// Opaque ref to the notebook document this checkpoint belongs to.
    pub document_id_ref: String,
    /// Opaque ref to the cell that created or owns this checkpoint.
    pub cell_id_ref: String,
    /// Checkpoint class.
    pub checkpoint_class: NotebookCheckpointClass,
    /// Sandbox state at the time of checkpoint creation.
    pub sandbox_state: NotebookSandboxState,
    /// Rollback posture that describes the honest recoverability of this checkpoint.
    pub rollback_posture: NotebookRollbackPosture,
    /// UTC timestamp when the checkpoint was created.
    pub checkpointed_at: String,
    /// Human-readable honest-state label rendered for this checkpoint.
    pub honest_state_label: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCheckpointedExecution {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookCheckpointedExecutionFinding> {
        let mut findings = Vec::new();
        let subject = self.checkpointed_execution_id.as_str();

        if self.record_kind != NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND {
            findings.push(NotebookCheckpointedExecutionFinding::new(
                "notebook_checkpointed_execution.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_teaching_continuity_checkpointed_redaction_schema_version
            != NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION
        {
            findings.push(NotebookCheckpointedExecutionFinding::new(
                "notebook_checkpointed_execution.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION}, found {}",
                    self.notebook_teaching_continuity_checkpointed_redaction_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookCheckpointedExecutionFinding::new(
                "notebook_checkpointed_execution.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookCheckpointedExecutionFinding::new(
                "notebook_checkpointed_execution.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }
        if self.checkpointed_at.trim().is_empty() {
            findings.push(NotebookCheckpointedExecutionFinding::new(
                "notebook_checkpointed_execution.checkpointed_at_required",
                subject,
                "checkpointed_at must be non-empty",
            ));
        }
        if self.honest_state_label.trim().is_empty() {
            findings.push(NotebookCheckpointedExecutionFinding::new(
                "notebook_checkpointed_execution.honest_state_label_required",
                subject,
                "honest_state_label must be non-empty",
            ));
        }

        if matches!(
            self.rollback_posture,
            NotebookRollbackPosture::ExactReplayAvailable
                | NotebookRollbackPosture::RollbackAvailable
        ) && matches!(self.sandbox_state, NotebookSandboxState::SandboxFailed)
        {
            findings.push(NotebookCheckpointedExecutionFinding::new(
                "notebook_checkpointed_execution.sandbox_failed_invariant",
                subject,
                "rollback_available or exact_replay_available must not pair with sandbox_failed",
            ));
        }

        findings
    }
}

/// Notebook redaction-before-share record. Carries redaction class, trigger,
/// scope, and explanation so collaboration and presentation surfaces expose
/// collapse/redaction controls before broad sharing when outputs may contain
/// sensitive data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRedactionBeforeShare {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_teaching_continuity_checkpointed_redaction_schema_version: u32,
    /// Stable opaque redaction id.
    pub redaction_id: String,
    /// Opaque ref to the notebook document this redaction belongs to.
    pub document_id_ref: String,
    /// Redaction class.
    pub redaction_class: NotebookRedactionClass,
    /// Redaction trigger.
    pub redaction_trigger: NotebookRedactionTrigger,
    /// Opaque refs to the cells whose content was redacted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub redacted_cell_refs: Vec<String>,
    /// Opaque refs to the outputs that were redacted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub redacted_output_refs: Vec<String>,
    /// Export-safe explanation of what was redacted and why.
    pub redaction_explanation: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookRedactionBeforeShare {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookRedactionBeforeShareFinding> {
        let mut findings = Vec::new();
        let subject = self.redaction_id.as_str();

        if self.record_kind != NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND {
            findings.push(NotebookRedactionBeforeShareFinding::new(
                "notebook_redaction_before_share.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_teaching_continuity_checkpointed_redaction_schema_version
            != NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION
        {
            findings.push(NotebookRedactionBeforeShareFinding::new(
                "notebook_redaction_before_share.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION}, found {}",
                    self.notebook_teaching_continuity_checkpointed_redaction_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookRedactionBeforeShareFinding::new(
                "notebook_redaction_before_share.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }

        if self.redaction_class != NotebookRedactionClass::None {
            if self.redacted_cell_refs.is_empty() && self.redacted_output_refs.is_empty() {
                findings.push(NotebookRedactionBeforeShareFinding::new(
                    "notebook_redaction_before_share.redacted_refs_required",
                    subject,
                    "redacted_cell_refs or redacted_output_refs must not be empty when redaction_class is not none",
                ));
            }
            if self.redaction_explanation.trim().is_empty() {
                findings.push(NotebookRedactionBeforeShareFinding::new(
                    "notebook_redaction_before_share.redaction_explanation_required",
                    subject,
                    "redaction_explanation must be non-empty when redaction_class is not none",
                ));
            }
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookRedactionBeforeShareFinding::new(
                "notebook_redaction_before_share.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Checked-in teaching-continuity, checkpointed-execution, and
/// redaction-before-share packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookTeachingContinuityCheckpointedRedactionPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: teaching modes.
    pub teaching_modes: Vec<NotebookTeachingMode>,
    /// Closed vocabulary: checkpoint preferences.
    pub checkpoint_preferences: Vec<NotebookCheckpointPreference>,
    /// Closed vocabulary: checkpoint classes.
    pub checkpoint_classes: Vec<NotebookCheckpointClass>,
    /// Closed vocabulary: sandbox states.
    pub sandbox_states: Vec<NotebookSandboxState>,
    /// Closed vocabulary: rollback postures.
    pub rollback_postures: Vec<NotebookRollbackPosture>,
    /// Closed vocabulary: redaction classes.
    pub redaction_classes: Vec<NotebookRedactionClass>,
    /// Closed vocabulary: redaction triggers.
    pub redaction_triggers: Vec<NotebookRedactionTrigger>,
    /// Worked example teaching-continuity records.
    pub example_teaching_continuities: Vec<NotebookTeachingContinuity>,
    /// Worked example checkpointed-execution records.
    pub example_checkpointed_executions: Vec<NotebookCheckpointedExecution>,
    /// Worked example redaction-before-share records.
    pub example_redaction_before_shares: Vec<NotebookRedactionBeforeShare>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookTeachingContinuityCheckpointedRedactionPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookTeachingContinuityCheckpointedRedactionPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION
        {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind
            != NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_RECORD_KIND
        {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.teaching_modes.len() != NotebookTeachingMode::ALL.len() {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.teaching_modes_coverage",
                subject,
                "teaching_modes must list every variant",
            ));
        }
        if self.checkpoint_preferences.len() != NotebookCheckpointPreference::ALL.len() {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.checkpoint_preferences_coverage",
                subject,
                "checkpoint_preferences must list every variant",
            ));
        }
        if self.checkpoint_classes.len() != NotebookCheckpointClass::ALL.len() {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.checkpoint_classes_coverage",
                subject,
                "checkpoint_classes must list every variant",
            ));
        }
        if self.sandbox_states.len() != NotebookSandboxState::ALL.len() {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.sandbox_states_coverage",
                subject,
                "sandbox_states must list every variant",
            ));
        }
        if self.rollback_postures.len() != NotebookRollbackPosture::ALL.len() {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.rollback_postures_coverage",
                subject,
                "rollback_postures must list every variant",
            ));
        }
        if self.redaction_classes.len() != NotebookRedactionClass::ALL.len() {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.redaction_classes_coverage",
                subject,
                "redaction_classes must list every variant",
            ));
        }
        if self.redaction_triggers.len() != NotebookRedactionTrigger::ALL.len() {
            findings.push(NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                "notebook_teaching_continuity_checkpointed_redaction_packet.redaction_triggers_coverage",
                subject,
                "redaction_triggers must list every variant",
            ));
        }

        for tc in &self.example_teaching_continuities {
            findings.extend(tc.validate().into_iter().map(|f| {
                NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for ce in &self.example_checkpointed_executions {
            findings.extend(ce.validate().into_iter().map(|f| {
                NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for rb in &self.example_redaction_before_shares {
            findings.extend(rb.validate().into_iter().map(|f| {
                NotebookTeachingContinuityCheckpointedRedactionPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }

        findings
    }
}

/// Builds the canonical checked-in [`NotebookTeachingContinuityCheckpointedRedactionPacket`].
pub fn current_notebook_teaching_continuity_checkpointed_redaction_packet(
) -> NotebookTeachingContinuityCheckpointedRedactionPacket {
    NotebookTeachingContinuityCheckpointedRedactionPacket {
        schema_version: NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
        record_kind: NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.teaching_continuity_checkpointed_redaction.packet.m5.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        teaching_modes: NotebookTeachingMode::ALL.to_vec(),
        checkpoint_preferences: NotebookCheckpointPreference::ALL.to_vec(),
        checkpoint_classes: NotebookCheckpointClass::ALL.to_vec(),
        sandbox_states: NotebookSandboxState::ALL.to_vec(),
        rollback_postures: NotebookRollbackPosture::ALL.to_vec(),
        redaction_classes: NotebookRedactionClass::ALL.to_vec(),
        redaction_triggers: NotebookRedactionTrigger::ALL.to_vec(),
        example_teaching_continuities: vec![
            NotebookTeachingContinuity {
                record_kind: NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                teaching_continuity_id: "nb.teaching.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                teaching_mode: NotebookTeachingMode::GuidedExercise,
                checkpoint_preference: NotebookCheckpointPreference::AutoCheckpoint,
                current_step_index: Some(2),
                total_steps: Some(5),
                sandbox_required: true,
                sandbox_unavailable_explanation: Some(
                    "Sandbox environment is provisioning; execution paused.".to_owned(),
                ),
                summary: "Guided exercise with auto-checkpoints and sandbox requirement.".to_owned(),
            },
            NotebookTeachingContinuity {
                record_kind: NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                teaching_continuity_id: "nb.teaching.02".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                teaching_mode: NotebookTeachingMode::Demo,
                checkpoint_preference: NotebookCheckpointPreference::ManualCheckpoint,
                current_step_index: None,
                total_steps: None,
                sandbox_required: false,
                sandbox_unavailable_explanation: None,
                summary: "Demo mode with manual checkpoint preference.".to_owned(),
            },
            NotebookTeachingContinuity {
                record_kind: NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                teaching_continuity_id: "nb.teaching.03".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                teaching_mode: NotebookTeachingMode::Classroom,
                checkpoint_preference: NotebookCheckpointPreference::SandboxOnly,
                current_step_index: None,
                total_steps: None,
                sandbox_required: true,
                sandbox_unavailable_explanation: None,
                summary: "Classroom mode with sandbox-only execution.".to_owned(),
            },
        ],
        example_checkpointed_executions: vec![
            NotebookCheckpointedExecution {
                record_kind: NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                checkpointed_execution_id: "nb.checkpoint.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.01".to_owned(),
                checkpoint_class: NotebookCheckpointClass::PreExecution,
                sandbox_state: NotebookSandboxState::Sandboxed,
                rollback_posture: NotebookRollbackPosture::ExactReplayAvailable,
                checkpointed_at: "2026-06-09T10:00:00Z".to_owned(),
                honest_state_label: "Exact replay available from sandboxed pre-execution checkpoint."
                    .to_owned(),
                summary: "Pre-execution checkpoint created in sandbox.".to_owned(),
            },
            NotebookCheckpointedExecution {
                record_kind: NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                checkpointed_execution_id: "nb.checkpoint.02".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.02".to_owned(),
                checkpoint_class: NotebookCheckpointClass::PreDestructive,
                sandbox_state: NotebookSandboxState::Unsandboxed,
                rollback_posture: NotebookRollbackPosture::RollbackAvailable,
                checkpointed_at: "2026-06-09T10:05:00Z".to_owned(),
                honest_state_label: "Rollback available; unsandboxed pre-destructive checkpoint."
                    .to_owned(),
                summary: "Pre-destructive checkpoint before package installation.".to_owned(),
            },
            NotebookCheckpointedExecution {
                record_kind: NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                checkpointed_execution_id: "nb.checkpoint.03".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                cell_id_ref: "nb.cell.03".to_owned(),
                checkpoint_class: NotebookCheckpointClass::AutoCheckpoint,
                sandbox_state: NotebookSandboxState::SandboxFailed,
                rollback_posture: NotebookRollbackPosture::CheckpointOrphaned,
                checkpointed_at: "2026-06-09T10:10:00Z".to_owned(),
                honest_state_label: "Checkpoint orphaned after sandbox failure.".to_owned(),
                summary: "Auto-checkpoint orphaned when sandbox creation failed.".to_owned(),
            },
        ],
        example_redaction_before_shares: vec![
            NotebookRedactionBeforeShare {
                record_kind: NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                redaction_id: "nb.redaction.01".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                redaction_class: NotebookRedactionClass::OutputRedacted,
                redaction_trigger: NotebookRedactionTrigger::SensitivityScan,
                redacted_cell_refs: vec!["nb.cell.04".to_owned()],
                redacted_output_refs: vec!["nb.output.04.01".to_owned()],
                redaction_explanation: "Output contained PII detected by sensitivity scan."
                    .to_owned(),
                summary: "Output redacted before share due to detected PII.".to_owned(),
            },
            NotebookRedactionBeforeShare {
                record_kind: NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                redaction_id: "nb.redaction.02".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                redaction_class: NotebookRedactionClass::VariableRedacted,
                redaction_trigger: NotebookRedactionTrigger::ManualReview,
                redacted_cell_refs: vec!["nb.cell.05".to_owned()],
                redacted_output_refs: vec![],
                redaction_explanation: "Variable values redacted per manual review before external share."
                    .to_owned(),
                summary: "Variable redaction applied before external share.".to_owned(),
            },
            NotebookRedactionBeforeShare {
                record_kind: NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND.to_owned(),
                notebook_teaching_continuity_checkpointed_redaction_schema_version:
                    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
                redaction_id: "nb.redaction.03".to_owned(),
                document_id_ref: "nb.doc.example".to_owned(),
                redaction_class: NotebookRedactionClass::None,
                redaction_trigger: NotebookRedactionTrigger::PolicyAuto,
                redacted_cell_refs: vec![],
                redacted_output_refs: vec![],
                redaction_explanation: "No redaction required by policy.".to_owned(),
                summary: "Policy cleared the notebook for unredacted share.".to_owned(),
            },
        ],
        summary: "Notebook teaching continuity, checkpointed execution, and redaction-before-share flows packet v1.".to_owned(),
    }
}

impl NotebookTeachingMode {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::GuidedExercise,
        Self::Demo,
        Self::SoloExploration,
        Self::Classroom,
        Self::MentorSession,
    ];
}

impl NotebookCheckpointPreference {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AutoCheckpoint,
        Self::ManualCheckpoint,
        Self::NoCheckpoint,
        Self::SandboxOnly,
    ];
}

impl NotebookCheckpointClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AutoCheckpoint,
        Self::ManualCheckpoint,
        Self::PreExecution,
        Self::PreDestructive,
        Self::SandboxBoundary,
    ];
}

impl NotebookSandboxState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Sandboxed,
        Self::Unsandboxed,
        Self::SandboxPending,
        Self::SandboxFailed,
    ];
}

impl NotebookRollbackPosture {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RollbackAvailable,
        Self::RollbackExpired,
        Self::CheckpointOrphaned,
        Self::ExactReplayAvailable,
        Self::CompensatingReplayOnly,
    ];
}

impl NotebookRedactionClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OutputRedacted,
        Self::CellSourceRedacted,
        Self::MetadataRedacted,
        Self::VariableRedacted,
        Self::None,
    ];
}

impl NotebookRedactionTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ManualReview,
        Self::PolicyAuto,
        Self::SensitivityScan,
        Self::RecipientMismatch,
        Self::TeachingSafety,
    ];
}

#[cfg(test)]
mod tests;
