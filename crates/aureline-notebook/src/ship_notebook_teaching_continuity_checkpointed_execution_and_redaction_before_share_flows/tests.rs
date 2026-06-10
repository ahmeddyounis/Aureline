use super::*;

fn sample_teaching_continuity_guided() -> NotebookTeachingContinuity {
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
    }
}

fn sample_teaching_continuity_demo() -> NotebookTeachingContinuity {
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
    }
}

fn sample_checkpointed_execution_sandboxed() -> NotebookCheckpointedExecution {
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
    }
}

fn sample_checkpointed_execution_unsandboxed() -> NotebookCheckpointedExecution {
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
    }
}

fn sample_checkpointed_execution_orphaned() -> NotebookCheckpointedExecution {
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
    }
}

fn sample_redaction_output() -> NotebookRedactionBeforeShare {
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
        redaction_explanation: "Output contained PII detected by sensitivity scan.".to_owned(),
        summary: "Output redacted before share due to detected PII.".to_owned(),
    }
}

fn sample_redaction_none() -> NotebookRedactionBeforeShare {
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
    }
}

#[test]
fn teaching_continuity_guided_validates_clean() {
    let t = sample_teaching_continuity_guided();
    assert!(
        t.validate().is_empty(),
        "guided teaching_continuity should be clean: {:?}",
        t.validate()
    );
}

#[test]
fn teaching_continuity_demo_validates_clean() {
    let t = sample_teaching_continuity_demo();
    assert!(
        t.validate().is_empty(),
        "demo teaching_continuity should be clean: {:?}",
        t.validate()
    );
}

#[test]
fn checkpointed_execution_sandboxed_validates_clean() {
    let c = sample_checkpointed_execution_sandboxed();
    assert!(
        c.validate().is_empty(),
        "sandboxed checkpointed_execution should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn checkpointed_execution_unsandboxed_validates_clean() {
    let c = sample_checkpointed_execution_unsandboxed();
    assert!(
        c.validate().is_empty(),
        "unsandboxed checkpointed_execution should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn checkpointed_execution_orphaned_validates_clean() {
    let c = sample_checkpointed_execution_orphaned();
    assert!(
        c.validate().is_empty(),
        "orphaned checkpointed_execution should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn redaction_output_validates_clean() {
    let r = sample_redaction_output();
    assert!(
        r.validate().is_empty(),
        "output redaction_before_share should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn redaction_none_validates_clean() {
    let r = sample_redaction_none();
    assert!(
        r.validate().is_empty(),
        "none redaction_before_share should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn teaching_continuity_rejects_empty_document_id_ref() {
    let mut t = sample_teaching_continuity_guided();
    t.document_id_ref = "".to_owned();
    let findings = t.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_teaching_continuity.document_id_ref_required"));
}

#[test]
fn teaching_continuity_rejects_empty_summary() {
    let mut t = sample_teaching_continuity_guided();
    t.summary = "".to_owned();
    let findings = t.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_teaching_continuity.summary_required"));
}

#[test]
fn checkpointed_execution_rejects_empty_document_id_ref() {
    let mut c = sample_checkpointed_execution_sandboxed();
    c.document_id_ref = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_checkpointed_execution.document_id_ref_required"));
}

#[test]
fn checkpointed_execution_rejects_empty_cell_id_ref() {
    let mut c = sample_checkpointed_execution_sandboxed();
    c.cell_id_ref = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_checkpointed_execution.cell_id_ref_required"));
}

#[test]
fn checkpointed_execution_rejects_empty_checkpointed_at() {
    let mut c = sample_checkpointed_execution_sandboxed();
    c.checkpointed_at = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_checkpointed_execution.checkpointed_at_required"));
}

#[test]
fn checkpointed_execution_rejects_rollback_available_with_sandbox_failed() {
    let mut c = sample_checkpointed_execution_sandboxed();
    c.rollback_posture = NotebookRollbackPosture::RollbackAvailable;
    c.sandbox_state = NotebookSandboxState::SandboxFailed;
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_checkpointed_execution.sandbox_failed_invariant"));
}

#[test]
fn checkpointed_execution_rejects_exact_replay_with_sandbox_failed() {
    let mut c = sample_checkpointed_execution_sandboxed();
    c.rollback_posture = NotebookRollbackPosture::ExactReplayAvailable;
    c.sandbox_state = NotebookSandboxState::SandboxFailed;
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_checkpointed_execution.sandbox_failed_invariant"));
}

#[test]
fn redaction_rejects_empty_document_id_ref() {
    let mut r = sample_redaction_output();
    r.document_id_ref = "".to_owned();
    let findings = r.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_redaction_before_share.document_id_ref_required"));
}

#[test]
fn redaction_rejects_missing_refs_when_redacted() {
    let mut r = sample_redaction_output();
    r.redacted_cell_refs = vec![];
    r.redacted_output_refs = vec![];
    let findings = r.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_redaction_before_share.redacted_refs_required"));
}

#[test]
fn redaction_rejects_empty_explanation_when_redacted() {
    let mut r = sample_redaction_output();
    r.redaction_explanation = "".to_owned();
    let findings = r.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_redaction_before_share.redaction_explanation_required"));
}

#[test]
fn redaction_rejects_empty_summary() {
    let mut r = sample_redaction_output();
    r.summary = "".to_owned();
    let findings = r.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_redaction_before_share.summary_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookTeachingMode::GuidedExercise.as_str(), "guided_exercise");
    assert_eq!(NotebookTeachingMode::Demo.as_str(), "demo");
    assert_eq!(NotebookTeachingMode::SoloExploration.as_str(), "solo_exploration");
    assert_eq!(NotebookTeachingMode::Classroom.as_str(), "classroom");
    assert_eq!(NotebookTeachingMode::MentorSession.as_str(), "mentor_session");

    assert_eq!(
        NotebookCheckpointPreference::AutoCheckpoint.as_str(),
        "auto_checkpoint"
    );
    assert_eq!(
        NotebookCheckpointPreference::ManualCheckpoint.as_str(),
        "manual_checkpoint"
    );
    assert_eq!(NotebookCheckpointPreference::NoCheckpoint.as_str(), "no_checkpoint");
    assert_eq!(NotebookCheckpointPreference::SandboxOnly.as_str(), "sandbox_only");

    assert_eq!(NotebookCheckpointClass::AutoCheckpoint.as_str(), "auto_checkpoint");
    assert_eq!(NotebookCheckpointClass::ManualCheckpoint.as_str(), "manual_checkpoint");
    assert_eq!(NotebookCheckpointClass::PreExecution.as_str(), "pre_execution");
    assert_eq!(
        NotebookCheckpointClass::PreDestructive.as_str(),
        "pre_destructive"
    );
    assert_eq!(
        NotebookCheckpointClass::SandboxBoundary.as_str(),
        "sandbox_boundary"
    );

    assert_eq!(NotebookSandboxState::Sandboxed.as_str(), "sandboxed");
    assert_eq!(NotebookSandboxState::Unsandboxed.as_str(), "unsandboxed");
    assert_eq!(NotebookSandboxState::SandboxPending.as_str(), "sandbox_pending");
    assert_eq!(NotebookSandboxState::SandboxFailed.as_str(), "sandbox_failed");

    assert_eq!(
        NotebookRollbackPosture::RollbackAvailable.as_str(),
        "rollback_available"
    );
    assert_eq!(
        NotebookRollbackPosture::RollbackExpired.as_str(),
        "rollback_expired"
    );
    assert_eq!(
        NotebookRollbackPosture::CheckpointOrphaned.as_str(),
        "checkpoint_orphaned"
    );
    assert_eq!(
        NotebookRollbackPosture::ExactReplayAvailable.as_str(),
        "exact_replay_available"
    );
    assert_eq!(
        NotebookRollbackPosture::CompensatingReplayOnly.as_str(),
        "compensating_replay_only"
    );

    assert_eq!(
        NotebookRedactionClass::OutputRedacted.as_str(),
        "output_redacted"
    );
    assert_eq!(
        NotebookRedactionClass::CellSourceRedacted.as_str(),
        "cell_source_redacted"
    );
    assert_eq!(
        NotebookRedactionClass::MetadataRedacted.as_str(),
        "metadata_redacted"
    );
    assert_eq!(
        NotebookRedactionClass::VariableRedacted.as_str(),
        "variable_redacted"
    );
    assert_eq!(NotebookRedactionClass::None.as_str(), "none");

    assert_eq!(
        NotebookRedactionTrigger::ManualReview.as_str(),
        "manual_review"
    );
    assert_eq!(NotebookRedactionTrigger::PolicyAuto.as_str(), "policy_auto");
    assert_eq!(
        NotebookRedactionTrigger::SensitivityScan.as_str(),
        "sensitivity_scan"
    );
    assert_eq!(
        NotebookRedactionTrigger::RecipientMismatch.as_str(),
        "recipient_mismatch"
    );
    assert_eq!(
        NotebookRedactionTrigger::TeachingSafety.as_str(),
        "teaching_safety"
    );
}

#[test]
fn packet_validates_clean() {
    let packet = current_notebook_teaching_continuity_checkpointed_redaction_packet();
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet =
        current_notebook_teaching_continuity_checkpointed_redaction_packet();
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_RECORD_KIND
    );
}
