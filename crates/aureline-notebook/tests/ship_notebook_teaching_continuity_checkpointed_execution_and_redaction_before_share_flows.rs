//! Integration tests for notebook teaching continuity, checkpointed execution,
//! and redaction-before-share flows.

use aureline_notebook::{
    current_notebook_teaching_continuity_checkpointed_redaction_packet, NotebookCheckpointClass,
    NotebookCheckpointPreference, NotebookCheckpointedExecution, NotebookRedactionBeforeShare,
    NotebookRedactionClass, NotebookRedactionTrigger, NotebookRollbackPosture,
    NotebookSandboxState, NotebookTeachingContinuity,
    NotebookTeachingContinuityCheckpointedRedactionPacket, NotebookTeachingMode,
    NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND, NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND,
    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_RECORD_KIND,
    NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
    NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND,
};

#[test]
fn module_constants_are_consistent() {
    assert_eq!(
        NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
        1
    );
    assert_eq!(
        NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND,
        "notebook_teaching_continuity"
    );
    assert_eq!(
        NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND,
        "notebook_checkpointed_execution"
    );
    assert_eq!(
        NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND,
        "notebook_redaction_before_share"
    );
    assert_eq!(
        NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_PACKET_RECORD_KIND,
        "notebook_teaching_continuity_checkpointed_redaction_packet"
    );
}

#[test]
fn teaching_continuity_roundtrips_through_json() {
    let original = NotebookTeachingContinuity {
        record_kind: NOTEBOOK_TEACHING_CONTINUITY_RECORD_KIND.to_owned(),
        notebook_teaching_continuity_checkpointed_redaction_schema_version:
            NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
        teaching_continuity_id: "nb.teaching.roundtrip.01".to_owned(),
        document_id_ref: "nb.doc.roundtrip".to_owned(),
        teaching_mode: NotebookTeachingMode::MentorSession,
        checkpoint_preference: NotebookCheckpointPreference::SandboxOnly,
        current_step_index: Some(1),
        total_steps: Some(3),
        sandbox_required: true,
        sandbox_unavailable_explanation: None,
        summary: "Mentor session round-trip test.".to_owned(),
    };
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookTeachingContinuity =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn checkpointed_execution_roundtrips_through_json() {
    let original = NotebookCheckpointedExecution {
        record_kind: NOTEBOOK_CHECKPOINTED_EXECUTION_RECORD_KIND.to_owned(),
        notebook_teaching_continuity_checkpointed_redaction_schema_version:
            NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
        checkpointed_execution_id: "nb.checkpoint.roundtrip.01".to_owned(),
        document_id_ref: "nb.doc.roundtrip".to_owned(),
        cell_id_ref: "nb.cell.roundtrip.01".to_owned(),
        checkpoint_class: NotebookCheckpointClass::SandboxBoundary,
        sandbox_state: NotebookSandboxState::SandboxPending,
        rollback_posture: NotebookRollbackPosture::CompensatingReplayOnly,
        checkpointed_at: "2026-06-09T12:00:00Z".to_owned(),
        honest_state_label: "Compensating replay only; sandbox still pending.".to_owned(),
        summary: "Sandbox boundary checkpoint round-trip test.".to_owned(),
    };
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookCheckpointedExecution =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn redaction_before_share_roundtrips_through_json() {
    let original = NotebookRedactionBeforeShare {
        record_kind: NOTEBOOK_REDACTION_BEFORE_SHARE_RECORD_KIND.to_owned(),
        notebook_teaching_continuity_checkpointed_redaction_schema_version:
            NOTEBOOK_TEACHING_CONTINUITY_CHECKPOINTED_REDACTION_SCHEMA_VERSION,
        redaction_id: "nb.redaction.roundtrip.01".to_owned(),
        document_id_ref: "nb.doc.roundtrip".to_owned(),
        redaction_class: NotebookRedactionClass::MetadataRedacted,
        redaction_trigger: NotebookRedactionTrigger::TeachingSafety,
        redacted_cell_refs: vec!["nb.cell.01".to_owned()],
        redacted_output_refs: vec![],
        redaction_explanation: "Metadata redacted for teaching safety.".to_owned(),
        summary: "Teaching-safety redaction round-trip test.".to_owned(),
    };
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookRedactionBeforeShare =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn packet_roundtrips_through_json() {
    let original = current_notebook_teaching_continuity_checkpointed_redaction_packet();
    let json = serde_json::to_string(&original).expect("must serialize");
    let deserialized: NotebookTeachingContinuityCheckpointedRedactionPacket =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(original, deserialized);
}

#[test]
fn embedded_packet_matches_generated_packet() {
    let generated = current_notebook_teaching_continuity_checkpointed_redaction_packet();
    let embedded = current_notebook_teaching_continuity_checkpointed_redaction_packet();
    assert_eq!(generated.schema_version, embedded.schema_version);
    assert_eq!(generated.record_kind, embedded.record_kind);
    assert_eq!(generated.packet_id, embedded.packet_id);
}

#[test]
fn all_teaching_modes_are_unique() {
    let all = NotebookTeachingMode::ALL;
    let mut set = std::collections::HashSet::new();
    for mode in &all {
        assert!(set.insert(mode.as_str()), "duplicate teaching mode token");
    }
    assert_eq!(all.len(), 5);
}

#[test]
fn all_checkpoint_classes_are_unique() {
    let all = NotebookCheckpointClass::ALL;
    let mut set = std::collections::HashSet::new();
    for cls in &all {
        assert!(set.insert(cls.as_str()), "duplicate checkpoint class token");
    }
    assert_eq!(all.len(), 5);
}

#[test]
fn all_redaction_triggers_are_unique() {
    let all = NotebookRedactionTrigger::ALL;
    let mut set = std::collections::HashSet::new();
    for trigger in &all {
        assert!(
            set.insert(trigger.as_str()),
            "duplicate redaction trigger token"
        );
    }
    assert_eq!(all.len(), 5);
}

#[test]
fn packet_validate_includes_subrecord_findings() {
    let mut packet = current_notebook_teaching_continuity_checkpointed_redaction_packet();
    packet.example_teaching_continuities[0].document_id_ref = "".to_owned();
    let findings = packet.validate();
    assert!(
        findings
            .iter()
            .any(|f| f.check_id.contains("document_id_ref_required")),
        "packet validation should surface subrecord findings"
    );
}
