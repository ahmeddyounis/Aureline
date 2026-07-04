// Canonical seed for the M5 run/attempt-header primitive. Included from `mod.rs`
// so the seeded builder, its worked cases, the fixture-emitting example, and the
// on-disk support export all stay byte-aligned.

/// A live task run on its second attempt, with a failed first attempt in the
/// selector — one run with multiple attempts, distinguishable from separate runs.
fn task_running_multi_attempt_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:task:build-and-test:0001".to_owned(),
        run_ref: "run:build-and-test:0001".to_owned(),
        attempt_ref: "attempt:build-and-test:0001#2".to_owned(),
        attempt_ordinal: 2,
        run_label: "build-and-test".to_owned(),
        initiator: M5RunInitiatorClass::UserManual,
        initiator_label: Some("initiator:workspace-user".to_owned()),
        target_ref: "target:workspace/root".to_owned(),
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "cargo build then cargo test on the workspace root".to_owned(),
        age_label: "just now".to_owned(),
        outcome: M5RunOutcome::Running,
        truth_mode: M5ExecutionTruthMode::Live,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![M5SiblingAttempt {
            attempt_ref: "attempt:build-and-test:0001#1".to_owned(),
            attempt_ordinal: 1,
            outcome: M5RunOutcome::Failed,
            is_current: false,
        }],
        degraded: None,
    }
}

/// A live task run preparing its environment (covers the Preparing outcome).
fn task_preparing_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:task:lint:0002".to_owned(),
        run_ref: "run:lint:0002".to_owned(),
        attempt_ref: "attempt:lint:0002#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "lint".to_owned(),
        initiator: M5RunInitiatorClass::UserManual,
        initiator_label: None,
        target_ref: "target:workspace/root".to_owned(),
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "resolving toolchain before running clippy".to_owned(),
        age_label: "5s ago".to_owned(),
        outcome: M5RunOutcome::Preparing,
        truth_mode: M5ExecutionTruthMode::Live,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: None,
    }
}

/// A live test run (shares the Running outcome with the task pane — AC2).
fn test_running_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:test:integration:0007".to_owned(),
        run_ref: "run:integration:0007".to_owned(),
        attempt_ref: "attempt:integration:0007#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "integration suite".to_owned(),
        initiator: M5RunInitiatorClass::UserManual,
        initiator_label: None,
        target_ref: "target:tests/integration".to_owned(),
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "running the integration test suite".to_owned(),
        age_label: "12s ago".to_owned(),
        outcome: M5RunOutcome::Running,
        truth_mode: M5ExecutionTruthMode::Live,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: None,
    }
}

/// A request run waiting for user input before it can continue.
fn request_waiting_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:request:deploy-approval:0003".to_owned(),
        run_ref: "run:deploy-approval:0003".to_owned(),
        attempt_ref: "attempt:deploy-approval:0003#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "deploy approval request".to_owned(),
        initiator: M5RunInitiatorClass::UserManual,
        initiator_label: None,
        target_ref: "target:api/deploy".to_owned(),
        target_boundary: M5ExecutionLocality::Remote,
        context_summary: "POST deploy request awaiting an approval before dispatch".to_owned(),
        age_label: "30s ago".to_owned(),
        outcome: M5RunOutcome::WaitingInput,
        truth_mode: M5ExecutionTruthMode::Live,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: None,
    }
}

/// A notebook execution partially complete: some cells done, some still pending.
fn notebook_partial_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:notebook:analysis:0004".to_owned(),
        run_ref: "run:notebook-analysis:0004".to_owned(),
        attempt_ref: "attempt:notebook-analysis:0004#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "analysis notebook".to_owned(),
        initiator: M5RunInitiatorClass::UserManual,
        initiator_label: None,
        target_ref: "target:notebooks/analysis.ipynb".to_owned(),
        target_boundary: M5ExecutionLocality::Container,
        context_summary: "run-all in progress; 6 of 14 cells executed".to_owned(),
        age_label: "1m ago".to_owned(),
        outcome: M5RunOutcome::PartiallyComplete,
        truth_mode: M5ExecutionTruthMode::Live,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: None,
    }
}

/// An AI-mediated run queued behind an upstream dependency — the queue reason and
/// admission-control class are disclosed in the shared header vocabulary.
fn ai_queued_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:ai:refactor-agent:0005".to_owned(),
        run_ref: "run:refactor-agent:0005".to_owned(),
        attempt_ref: "attempt:refactor-agent:0005#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "refactor agent run".to_owned(),
        initiator: M5RunInitiatorClass::AgentAi,
        initiator_label: Some("initiator:refactor-agent".to_owned()),
        target_ref: "target:workspace/src".to_owned(),
        target_boundary: M5ExecutionLocality::Managed,
        context_summary: "agent refactor run queued behind an upstream build".to_owned(),
        age_label: "2m ago".to_owned(),
        outcome: M5RunOutcome::Queued,
        truth_mode: M5ExecutionTruthMode::Planned,
        admission_control: M5AdmissionControlClass::DependencyQueued,
        queue_reason: Some("waiting on the upstream build run to finish".to_owned()),
        relative_ordering: Some(3),
        sibling_attempts: vec![],
        degraded: None,
    }
}

/// A publish flow that passed (shares the Passed outcome with preview — AC2).
fn publish_passed_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:publish:release-bundle:0006".to_owned(),
        run_ref: "run:release-bundle:0006".to_owned(),
        attempt_ref: "attempt:release-bundle:0006#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "release bundle publish".to_owned(),
        initiator: M5RunInitiatorClass::CiTriggered,
        initiator_label: Some("initiator:release-pipeline".to_owned()),
        target_ref: "target:registry/release".to_owned(),
        target_boundary: M5ExecutionLocality::Remote,
        context_summary: "published the release bundle to the registry".to_owned(),
        age_label: "8m ago".to_owned(),
        outcome: M5RunOutcome::Passed,
        truth_mode: M5ExecutionTruthMode::Captured,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: None,
    }
}

/// A preview flow that passed (shares the Passed outcome with publish — AC2).
fn preview_passed_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:preview:render:0008".to_owned(),
        run_ref: "run:preview-render:0008".to_owned(),
        attempt_ref: "attempt:preview-render:0008#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "preview render".to_owned(),
        initiator: M5RunInitiatorClass::WatchAuto,
        initiator_label: None,
        target_ref: "target:preview/index".to_owned(),
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "rendered the preview after a source change".to_owned(),
        age_label: "3m ago".to_owned(),
        outcome: M5RunOutcome::Passed,
        truth_mode: M5ExecutionTruthMode::Captured,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: None,
    }
}

/// A history / activity-center row for a run whose output went stale after a source
/// change — disclosed as captured evidence, never as a live result.
fn history_stale_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:history:nightly:0009".to_owned(),
        run_ref: "run:nightly:0009".to_owned(),
        attempt_ref: "attempt:nightly:0009#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "nightly build".to_owned(),
        initiator: M5RunInitiatorClass::Scheduled,
        initiator_label: Some("initiator:nightly-schedule".to_owned()),
        target_ref: "target:workspace/root".to_owned(),
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "nightly build superseded by a later source change".to_owned(),
        age_label: "9h ago".to_owned(),
        outcome: M5RunOutcome::StaleOutput,
        truth_mode: M5ExecutionTruthMode::Captured,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: Some(DegradedState {
            trigger: M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
            degraded_label:
                "the source changed since this run; its output is shown as captured evidence rather than a live result"
                    .to_owned(),
        }),
    }
}

/// A support / export replay reconstructed from an imported CI run.
fn support_replay_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:support:imported-ci:0010".to_owned(),
        run_ref: "run:imported-ci:0010".to_owned(),
        attempt_ref: "attempt:imported-ci:0010#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "imported CI run".to_owned(),
        initiator: M5RunInitiatorClass::CiTriggered,
        initiator_label: None,
        target_ref: "target:ci/pipeline".to_owned(),
        target_boundary: M5ExecutionLocality::Remote,
        context_summary: "offline replay of an imported CI run for support".to_owned(),
        age_label: "2d ago".to_owned(),
        outcome: M5RunOutcome::Failed,
        truth_mode: M5ExecutionTruthMode::Imported,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: None,
    }
}

/// A companion-surface summary of a cancelled watch-triggered run.
fn companion_cancelled_input() -> M5RunAttemptHeaderInput {
    M5RunAttemptHeaderInput {
        header_id: "header:companion:watch-run:0011".to_owned(),
        run_ref: "run:watch-run:0011".to_owned(),
        attempt_ref: "attempt:watch-run:0011#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "watch-triggered run".to_owned(),
        initiator: M5RunInitiatorClass::WatchAuto,
        initiator_label: None,
        target_ref: "target:workspace/watched".to_owned(),
        target_boundary: M5ExecutionLocality::Managed,
        context_summary: "watch-triggered run cancelled when a newer change arrived".to_owned(),
        age_label: "20m ago".to_owned(),
        outcome: M5RunOutcome::Cancelled,
        truth_mode: M5ExecutionTruthMode::Captured,
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        relative_ordering: None,
        sibling_attempts: vec![],
        degraded: None,
    }
}

fn case(input: M5RunAttemptHeaderInput) -> M5RunAttemptCase {
    M5RunAttemptCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5RunAttemptSurfaceRow> {
    let base_source_refs = vec![
        M5_RUN_ATTEMPT_HEADER_SCHEMA_REF.to_owned(),
        M5_RUN_ATTEMPT_HEADER_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5RunAttemptExportField::ALL.to_vec();

    vec![
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::TaskRunPane,
            owner_role: "Task-run guild".to_owned(),
            scope_summary:
                "Run/attempt header and attempt selector for task runs, keeping run and attempt distinct across retries"
                    .to_owned(),
            outcomes: vec![M5RunOutcome::Running, M5RunOutcome::Preparing],
            truth_modes: vec![M5ExecutionTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::RunAttemptIdentityUnresolved,
                M5ExecutionDowngradeTrigger::ConnectorLost,
            ],
            consumer_surfaces: vec!["task_pane".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![
                case(task_running_multi_attempt_input()),
                case(task_preparing_input()),
            ],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::TestRunPane,
            owner_role: "Test-run guild".to_owned(),
            scope_summary: "Run/attempt header for test runs with the shared outcome vocabulary"
                .to_owned(),
            outcomes: vec![M5RunOutcome::Running, M5RunOutcome::Passed, M5RunOutcome::Failed],
            truth_modes: vec![M5ExecutionTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["test_pane".to_owned(), "history".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![case(test_running_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::RequestRunPane,
            owner_role: "Request-execution guild".to_owned(),
            scope_summary: "Run/attempt header for API requests awaiting input before dispatch"
                .to_owned(),
            outcomes: vec![M5RunOutcome::WaitingInput, M5RunOutcome::Passed],
            truth_modes: vec![M5ExecutionTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::ConnectorLost],
            consumer_surfaces: vec!["request_pane".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![case(request_waiting_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::NotebookExecution,
            owner_role: "Notebook-execution guild".to_owned(),
            scope_summary: "Run/attempt header for notebook run-all keeping partial progress honest"
                .to_owned(),
            outcomes: vec![M5RunOutcome::PartiallyComplete, M5RunOutcome::Running],
            truth_modes: vec![M5ExecutionTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["notebook_pane".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![case(notebook_partial_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::AiMediatedExecution,
            owner_role: "AI-execution guild".to_owned(),
            scope_summary:
                "Run/attempt header for agent runs disclosing queue reason and admission-control class"
                    .to_owned(),
            outcomes: vec![M5RunOutcome::Queued, M5RunOutcome::Running],
            truth_modes: vec![M5ExecutionTruthMode::Planned, M5ExecutionTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::RunAttemptIdentityUnresolved,
                M5ExecutionDowngradeTrigger::ConnectorLost,
            ],
            consumer_surfaces: vec!["ai_pane".to_owned(), "companion".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![case(ai_queued_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::PublishFlow,
            owner_role: "Publish guild".to_owned(),
            scope_summary: "Run/attempt header for publish runs with captured outcome truth"
                .to_owned(),
            outcomes: vec![M5RunOutcome::Passed, M5RunOutcome::Failed],
            truth_modes: vec![M5ExecutionTruthMode::Captured],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["publish_pane".to_owned(), "release_control".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![case(publish_passed_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::PreviewFlow,
            owner_role: "Preview guild".to_owned(),
            scope_summary: "Run/attempt header for preview renders with the shared outcome vocabulary"
                .to_owned(),
            outcomes: vec![M5RunOutcome::Passed, M5RunOutcome::Failed],
            truth_modes: vec![M5ExecutionTruthMode::Captured],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["preview_pane".to_owned(), "companion".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![case(preview_passed_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::HistoryActivityCenter,
            owner_role: "History / activity-center guild".to_owned(),
            scope_summary:
                "Run/attempt history rows keeping stale output as captured evidence, never a live result"
                    .to_owned(),
            outcomes: vec![M5RunOutcome::StaleOutput, M5RunOutcome::Passed, M5RunOutcome::Failed],
            truth_modes: vec![M5ExecutionTruthMode::Captured],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
                M5ExecutionDowngradeTrigger::RunAttemptIdentityUnresolved,
            ],
            consumer_surfaces: vec!["history".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![case(history_stale_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::SupportExportReplay,
            owner_role: "Support / diagnostics guild".to_owned(),
            scope_summary:
                "Offline replay reconstructing run/attempt truth from an imported CI run"
                    .to_owned(),
            outcomes: vec![M5RunOutcome::Failed, M5RunOutcome::Passed],
            truth_modes: vec![M5ExecutionTruthMode::Imported],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_headers: vec![case(support_replay_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
        M5RunAttemptSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::CompanionSummary,
            owner_role: "Companion-surface guild".to_owned(),
            scope_summary: "Companion run summary keeping run/attempt identity and outcome explicit"
                .to_owned(),
            outcomes: vec![M5RunOutcome::Cancelled, M5RunOutcome::Passed],
            truth_modes: vec![M5ExecutionTruthMode::Captured],
            export_fields: all_export_fields,
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::ConnectorLost],
            consumer_surfaces: vec!["companion".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs,
            example_headers: vec![case(companion_cancelled_input())],
            hides_run_or_attempt_identity: false,
            blurs_run_and_attempt: false,
            drops_state_label_parity: false,
            drops_export_ids_or_states: false,
        },
    ]
}

fn seeded_governance_review() -> M5RunAttemptGovernanceReview {
    M5RunAttemptGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        run_and_attempt_identity_kept_distinct: true,
        state_labels_consistent_across_surfaces: true,
        queue_and_admission_preserved_in_shared_vocabulary: true,
        exported_evidence_preserves_ids_and_states: true,
        support_export_reconstructs_run_attempt: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5RunAttemptConsumerProjection {
    M5RunAttemptConsumerProjection {
        execution_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        attempt_selector_reads_single_run_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5RunAttemptReleasePosture {
    M5RunAttemptReleasePosture {
        release_packet_ref: M5_RUN_ATTEMPT_HEADER_ARTIFACT_REF.to_owned(),
        header_audit_ref: M5_RUN_ATTEMPT_HEADER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 run/attempt-header primitive packet. This is
/// the one source of truth shared by the tests, the fixture-emitting example, and the
/// on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_run_attempt_header_packet() -> M5RunAttemptHeaderPrimitivePacket {
    M5RunAttemptHeaderPrimitivePacket::new(M5RunAttemptHeaderPrimitivePacketInput {
        packet_id: "m5-run-attempt-header-primitive:stable:0001".to_owned(),
        matrix_label: "M5 Run/Attempt-Header Primitive: Header and Attempt Selector".to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5RunAttemptVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_RUN_ATTEMPT_HEADER_SCHEMA_REF.to_owned(),
            M5_RUN_ATTEMPT_HEADER_DOC_REF.to_owned(),
            M5_RUN_ATTEMPT_HEADER_COMPONENT_MATRIX_REF.to_owned(),
            M5_RUN_ATTEMPT_HEADER_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
