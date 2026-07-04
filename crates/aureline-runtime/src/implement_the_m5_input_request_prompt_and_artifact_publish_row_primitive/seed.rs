// Canonical seed for the M5 input-request / artifact-publish primitive. Included from
// `mod.rs` so the seeded builder, its worked cases, the fixture-emitting example, and
// the on-disk support export all stay byte-aligned.

/// A live task run whose plain-text input was answered and that is streaming a live
/// trace plus a buffered report.
fn task_running_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:task:build-and-test:0001".to_owned(),
        run_ref: "run:build-and-test:0001".to_owned(),
        attempt_ref: "attempt:build-and-test:0001#2".to_owned(),
        attempt_ordinal: 2,
        run_label: "build-and-test".to_owned(),
        run_outcome: M5RunOutcome::Running,
        truth_mode: M5ExecutionTruthMode::Live,
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "cargo build then cargo test on the workspace root".to_owned(),
        age_label: "just now".to_owned(),
        input_request: Some(M5InputRequestInput {
            prompt_ref: "input_request:confirm-target:0001".to_owned(),
            kind: M5InputRequestKind::PlainText,
            prompt_label: "name the build target to run".to_owned(),
            consequence: M5InputConsequence::BlocksUntilAnswered,
            disposition: M5InputRequestDisposition::Continued,
            has_deadline: false,
            deadline_label: None,
            default_label: None,
        }),
        artifacts: vec![
            M5ArtifactPublishInput {
                artifact_ref: "artifact:build-trace:0001".to_owned(),
                producing_run_ref: "run:build-and-test:0001".to_owned(),
                producing_attempt_ref: "attempt:build-and-test:0001#2".to_owned(),
                producing_step_label: "cargo build".to_owned(),
                artifact_label: "live build trace".to_owned(),
                kind: M5ArtifactKind::Trace,
                freshness: M5ArtifactFreshness::Live,
                retention: M5RetentionClass::EphemeralSessionOnly,
                trust: M5ArtifactTrustClass::FirstPartyUnverified,
                open_action_ref: Some("action:open:build-trace:0001".to_owned()),
                export_action_ref: Some("action:export:build-trace:0001".to_owned()),
            },
            M5ArtifactPublishInput {
                artifact_ref: "artifact:test-report:0001".to_owned(),
                producing_run_ref: "run:build-and-test:0001".to_owned(),
                producing_attempt_ref: "attempt:build-and-test:0001#2".to_owned(),
                producing_step_label: "cargo test".to_owned(),
                artifact_label: "buffered test report".to_owned(),
                kind: M5ArtifactKind::Report,
                freshness: M5ArtifactFreshness::Buffered,
                retention: M5RetentionClass::RetainedDurable,
                trust: M5ArtifactTrustClass::FirstPartyVerified,
                open_action_ref: Some("action:open:test-report:0001".to_owned()),
                export_action_ref: Some("action:export:test-report:0001".to_owned()),
            },
        ],
        degraded: None,
    }
}

/// A live test run producing a buffered report and a sampled trace.
fn test_running_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:test:integration:0002".to_owned(),
        run_ref: "run:integration:0002".to_owned(),
        attempt_ref: "attempt:integration:0002#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "integration suite".to_owned(),
        run_outcome: M5RunOutcome::Running,
        truth_mode: M5ExecutionTruthMode::Live,
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "running the integration test suite".to_owned(),
        age_label: "12s ago".to_owned(),
        input_request: None,
        artifacts: vec![
            M5ArtifactPublishInput {
                artifact_ref: "artifact:integration-report:0002".to_owned(),
                producing_run_ref: "run:integration:0002".to_owned(),
                producing_attempt_ref: "attempt:integration:0002#1".to_owned(),
                producing_step_label: "test runner".to_owned(),
                artifact_label: "buffered integration report".to_owned(),
                kind: M5ArtifactKind::Report,
                freshness: M5ArtifactFreshness::Buffered,
                retention: M5RetentionClass::RetainedDurable,
                trust: M5ArtifactTrustClass::FirstPartyVerified,
                open_action_ref: Some("action:open:integration-report:0002".to_owned()),
                export_action_ref: Some("action:export:integration-report:0002".to_owned()),
            },
            M5ArtifactPublishInput {
                artifact_ref: "artifact:integration-trace:0002".to_owned(),
                producing_run_ref: "run:integration:0002".to_owned(),
                producing_attempt_ref: "attempt:integration:0002#1".to_owned(),
                producing_step_label: "sampling profiler".to_owned(),
                artifact_label: "sampled execution trace".to_owned(),
                kind: M5ArtifactKind::Trace,
                freshness: M5ArtifactFreshness::Sampled,
                retention: M5RetentionClass::ExpiresScheduled,
                trust: M5ArtifactTrustClass::FirstPartyUnverified,
                open_action_ref: Some("action:open:integration-trace:0002".to_owned()),
                export_action_ref: Some("action:export:integration-trace:0002".to_owned()),
            },
        ],
        degraded: None,
    }
}

/// A request run paused on an approval gate, awaiting a response before dispatch.
fn request_awaiting_approval_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:request:deploy-approval:0003".to_owned(),
        run_ref: "run:deploy-approval:0003".to_owned(),
        attempt_ref: "attempt:deploy-approval:0003#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "deploy approval request".to_owned(),
        run_outcome: M5RunOutcome::WaitingInput,
        truth_mode: M5ExecutionTruthMode::Live,
        target_boundary: M5ExecutionLocality::Remote,
        context_summary: "POST deploy request awaiting an approval before dispatch".to_owned(),
        age_label: "30s ago".to_owned(),
        input_request: Some(M5InputRequestInput {
            prompt_ref: "input_request:approve-deploy:0003".to_owned(),
            kind: M5InputRequestKind::Approval,
            prompt_label: "approve the deploy request before it is dispatched".to_owned(),
            consequence: M5InputConsequence::RequiresApproval,
            disposition: M5InputRequestDisposition::AwaitingResponse,
            has_deadline: false,
            deadline_label: None,
            default_label: None,
        }),
        artifacts: vec![],
        degraded: None,
    }
}

/// A notebook run whose file-selection prompt was answered before its deadline, with a
/// declared default; producing a buffered preview endpoint.
fn notebook_partial_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:notebook:analysis:0004".to_owned(),
        run_ref: "run:notebook-analysis:0004".to_owned(),
        attempt_ref: "attempt:notebook-analysis:0004#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "analysis notebook".to_owned(),
        run_outcome: M5RunOutcome::PartiallyComplete,
        truth_mode: M5ExecutionTruthMode::Live,
        target_boundary: M5ExecutionLocality::Container,
        context_summary: "run-all in progress; 6 of 14 cells executed".to_owned(),
        age_label: "1m ago".to_owned(),
        input_request: Some(M5InputRequestInput {
            prompt_ref: "input_request:select-dataset:0004".to_owned(),
            kind: M5InputRequestKind::FilePathSelection,
            prompt_label: "select the dataset file for the analysis cells".to_owned(),
            consequence: M5InputConsequence::TimeoutAppliesDefault,
            disposition: M5InputRequestDisposition::Continued,
            has_deadline: true,
            deadline_label: Some("30s before the sample dataset is used".to_owned()),
            default_label: Some("the bundled sample dataset".to_owned()),
        }),
        artifacts: vec![M5ArtifactPublishInput {
            artifact_ref: "artifact:notebook-preview:0004".to_owned(),
            producing_run_ref: "run:notebook-analysis:0004".to_owned(),
            producing_attempt_ref: "attempt:notebook-analysis:0004#1".to_owned(),
            producing_step_label: "render cell output".to_owned(),
            artifact_label: "buffered analysis preview".to_owned(),
            kind: M5ArtifactKind::PreviewEndpoint,
            freshness: M5ArtifactFreshness::Buffered,
            retention: M5RetentionClass::EphemeralSessionOnly,
            trust: M5ArtifactTrustClass::FirstPartyUnverified,
            open_action_ref: Some("action:open:notebook-preview:0004".to_owned()),
            export_action_ref: Some("action:export:notebook-preview:0004".to_owned()),
        }],
        degraded: None,
    }
}

/// An AI-mediated run whose choice prompt was dismissed; the dismissal leaves the run
/// blocked and waiting rather than silently failing.
fn ai_dismissed_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:ai:refactor-agent:0005".to_owned(),
        run_ref: "run:refactor-agent:0005".to_owned(),
        attempt_ref: "attempt:refactor-agent:0005#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "refactor agent run".to_owned(),
        run_outcome: M5RunOutcome::WaitingInput,
        truth_mode: M5ExecutionTruthMode::Live,
        target_boundary: M5ExecutionLocality::Managed,
        context_summary: "agent refactor run waiting on a strategy choice that was dismissed"
            .to_owned(),
        age_label: "2m ago".to_owned(),
        input_request: Some(M5InputRequestInput {
            prompt_ref: "input_request:choose-strategy:0005".to_owned(),
            kind: M5InputRequestKind::Choice,
            prompt_label: "choose the refactor strategy for the agent to apply".to_owned(),
            consequence: M5InputConsequence::DismissLeavesWaiting,
            disposition: M5InputRequestDisposition::Dismissed,
            has_deadline: false,
            deadline_label: None,
            default_label: None,
        }),
        artifacts: vec![M5ArtifactPublishInput {
            artifact_ref: "artifact:agent-plan-log:0005".to_owned(),
            producing_run_ref: "run:refactor-agent:0005".to_owned(),
            producing_attempt_ref: "attempt:refactor-agent:0005#1".to_owned(),
            producing_step_label: "agent planning".to_owned(),
            artifact_label: "buffered agent plan log".to_owned(),
            kind: M5ArtifactKind::DiagnosticLog,
            freshness: M5ArtifactFreshness::Buffered,
            retention: M5RetentionClass::ExpiresScheduled,
            trust: M5ArtifactTrustClass::FirstPartyUnverified,
            open_action_ref: Some("action:open:agent-plan-log:0005".to_owned()),
            export_action_ref: Some("action:export:agent-plan-log:0005".to_owned()),
        }],
        degraded: None,
    }
}

/// A publish run that passed, producing a durably-retained release bundle.
fn publish_passed_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:publish:release-bundle:0006".to_owned(),
        run_ref: "run:release-bundle:0006".to_owned(),
        attempt_ref: "attempt:release-bundle:0006#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "release bundle publish".to_owned(),
        run_outcome: M5RunOutcome::Passed,
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Remote,
        context_summary: "published the release bundle to the registry".to_owned(),
        age_label: "8m ago".to_owned(),
        input_request: None,
        artifacts: vec![M5ArtifactPublishInput {
            artifact_ref: "artifact:release-bundle:0006".to_owned(),
            producing_run_ref: "run:release-bundle:0006".to_owned(),
            producing_attempt_ref: "attempt:release-bundle:0006#1".to_owned(),
            producing_step_label: "package and sign".to_owned(),
            artifact_label: "signed release bundle".to_owned(),
            kind: M5ArtifactKind::Bundle,
            freshness: M5ArtifactFreshness::Buffered,
            retention: M5RetentionClass::RetainedDurable,
            trust: M5ArtifactTrustClass::FirstPartyVerified,
            open_action_ref: Some("action:open:release-bundle:0006".to_owned()),
            export_action_ref: Some("action:export:release-bundle:0006".to_owned()),
        }],
        degraded: None,
    }
}

/// A preview run that passed, producing a buffered preview endpoint.
fn preview_passed_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:preview:render:0007".to_owned(),
        run_ref: "run:preview-render:0007".to_owned(),
        attempt_ref: "attempt:preview-render:0007#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "preview render".to_owned(),
        run_outcome: M5RunOutcome::Passed,
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "rendered the preview after a source change".to_owned(),
        age_label: "3m ago".to_owned(),
        input_request: None,
        artifacts: vec![M5ArtifactPublishInput {
            artifact_ref: "artifact:preview-endpoint:0007".to_owned(),
            producing_run_ref: "run:preview-render:0007".to_owned(),
            producing_attempt_ref: "attempt:preview-render:0007#1".to_owned(),
            producing_step_label: "render preview".to_owned(),
            artifact_label: "buffered preview endpoint".to_owned(),
            kind: M5ArtifactKind::PreviewEndpoint,
            freshness: M5ArtifactFreshness::Buffered,
            retention: M5RetentionClass::ExpiresScheduled,
            trust: M5ArtifactTrustClass::FirstPartyVerified,
            open_action_ref: Some("action:open:preview-endpoint:0007".to_owned()),
            export_action_ref: Some("action:export:preview-endpoint:0007".to_owned()),
        }],
        degraded: None,
    }
}

/// A history row for a stale run whose report has been evicted but stays attributable
/// to its producing run via lineage.
fn history_evicted_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:history:nightly:0008".to_owned(),
        run_ref: "run:nightly:0008".to_owned(),
        attempt_ref: "attempt:nightly:0008#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "nightly build".to_owned(),
        run_outcome: M5RunOutcome::StaleOutput,
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Local,
        context_summary: "nightly build superseded by a later source change".to_owned(),
        age_label: "9h ago".to_owned(),
        input_request: None,
        artifacts: vec![M5ArtifactPublishInput {
            artifact_ref: "artifact:nightly-report:0008".to_owned(),
            producing_run_ref: "run:nightly:0008".to_owned(),
            producing_attempt_ref: "attempt:nightly:0008#1".to_owned(),
            producing_step_label: "nightly report".to_owned(),
            artifact_label: "evicted nightly report (recoverable from lineage)".to_owned(),
            kind: M5ArtifactKind::Report,
            freshness: M5ArtifactFreshness::Buffered,
            retention: M5RetentionClass::EvictedRecoverable,
            trust: M5ArtifactTrustClass::FirstPartyVerified,
            open_action_ref: None,
            export_action_ref: Some("action:export:nightly-report:0008".to_owned()),
        }],
        degraded: Some(DegradedState {
            trigger: M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
            degraded_label:
                "the report bytes were evicted after the run went stale; it stays attributable to its producing run and can be rebuilt from lineage"
                    .to_owned(),
        }),
    }
}

/// A support / export replay reconstructing an imported provider artifact from an
/// imported CI run.
fn support_imported_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:support:imported-ci:0009".to_owned(),
        run_ref: "run:imported-ci:0009".to_owned(),
        attempt_ref: "attempt:imported-ci:0009#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "imported CI run".to_owned(),
        run_outcome: M5RunOutcome::Failed,
        truth_mode: M5ExecutionTruthMode::Imported,
        target_boundary: M5ExecutionLocality::Remote,
        context_summary: "offline replay of an imported CI run for support".to_owned(),
        age_label: "2d ago".to_owned(),
        input_request: None,
        artifacts: vec![M5ArtifactPublishInput {
            artifact_ref: "artifact:imported-ci-log:0009".to_owned(),
            producing_run_ref: "run:imported-ci:0009".to_owned(),
            producing_attempt_ref: "attempt:imported-ci:0009#1".to_owned(),
            producing_step_label: "imported CI pipeline".to_owned(),
            artifact_label: "imported CI failure log".to_owned(),
            kind: M5ArtifactKind::ImportedProviderArtifact,
            freshness: M5ArtifactFreshness::Imported,
            retention: M5RetentionClass::ExpiresScheduled,
            trust: M5ArtifactTrustClass::ProviderAttested,
            open_action_ref: Some("action:open:imported-ci-log:0009".to_owned()),
            export_action_ref: Some("action:export:imported-ci-log:0009".to_owned()),
        }],
        degraded: None,
    }
}

/// A companion summary of a cancelled run whose device/browser handoff timed out; the
/// timeout cancels the run as its disclosed consequence, and a provider-supplied
/// artifact is shown as such.
fn companion_timed_out_input() -> M5ExecutionInteractionInput {
    M5ExecutionInteractionInput {
        interaction_id: "interaction:companion:handoff-run:0010".to_owned(),
        run_ref: "run:handoff-run:0010".to_owned(),
        attempt_ref: "attempt:handoff-run:0010#1".to_owned(),
        attempt_ordinal: 1,
        run_label: "device-handoff run".to_owned(),
        run_outcome: M5RunOutcome::Cancelled,
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Managed,
        context_summary: "browser handoff timed out; the run was cancelled".to_owned(),
        age_label: "20m ago".to_owned(),
        input_request: Some(M5InputRequestInput {
            prompt_ref: "input_request:browser-handoff:0010".to_owned(),
            kind: M5InputRequestKind::DeviceBrowserHandoff,
            prompt_label: "complete the sign-in in the opened browser to continue".to_owned(),
            consequence: M5InputConsequence::TimeoutCancelsRun,
            disposition: M5InputRequestDisposition::TimedOut,
            has_deadline: true,
            deadline_label: Some("2m before the handoff is abandoned".to_owned()),
            default_label: None,
        }),
        artifacts: vec![M5ArtifactPublishInput {
            artifact_ref: "artifact:provider-session:0010".to_owned(),
            producing_run_ref: "run:handoff-run:0010".to_owned(),
            producing_attempt_ref: "attempt:handoff-run:0010#1".to_owned(),
            producing_step_label: "provider session record".to_owned(),
            artifact_label: "provider-supplied session record".to_owned(),
            kind: M5ArtifactKind::ImportedProviderArtifact,
            freshness: M5ArtifactFreshness::ProviderSupplied,
            retention: M5RetentionClass::RetainedDurable,
            trust: M5ArtifactTrustClass::ProviderAttested,
            open_action_ref: Some("action:open:provider-session:0010".to_owned()),
            export_action_ref: Some("action:export:provider-session:0010".to_owned()),
        }],
        degraded: None,
    }
}

fn case(input: M5ExecutionInteractionInput) -> M5ExecutionInteractionCase {
    M5ExecutionInteractionCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5InteractionSurfaceRow> {
    let base_source_refs = vec![
        M5_EXECUTION_INTERACTION_SCHEMA_REF.to_owned(),
        M5_EXECUTION_INTERACTION_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5InteractionExportField::ALL.to_vec();

    vec![
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::TaskRunPane,
            owner_role: "Task-run guild".to_owned(),
            scope_summary:
                "Input prompts and artifact rows for task runs, keeping produced objects attributed while the run is live"
                    .to_owned(),
            input_kinds: vec![M5InputRequestKind::PlainText],
            artifact_kinds: vec![M5ArtifactKind::Trace, M5ArtifactKind::Report],
            freshness_classes: vec![M5ArtifactFreshness::Live, M5ArtifactFreshness::Buffered],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::InputConsequenceUnknown,
                M5ExecutionDowngradeTrigger::ArtifactLineageLost,
            ],
            consumer_surfaces: vec!["task_pane".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(task_running_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::TestRunPane,
            owner_role: "Test-run guild".to_owned(),
            scope_summary: "Artifact rows for test runs disclosing buffered and sampled freshness"
                .to_owned(),
            input_kinds: vec![],
            artifact_kinds: vec![M5ArtifactKind::Report, M5ArtifactKind::Trace],
            freshness_classes: vec![M5ArtifactFreshness::Buffered, M5ArtifactFreshness::Sampled],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::ArtifactRetentionExpired],
            consumer_surfaces: vec!["test_pane".to_owned(), "history".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(test_running_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::RequestRunPane,
            owner_role: "Request-execution guild".to_owned(),
            scope_summary: "Approval input prompts for API requests, awaiting a response before dispatch"
                .to_owned(),
            input_kinds: vec![M5InputRequestKind::Approval],
            artifact_kinds: vec![],
            freshness_classes: vec![],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::InputConsequenceUnknown],
            consumer_surfaces: vec!["request_pane".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(request_awaiting_approval_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::NotebookExecution,
            owner_role: "Notebook-execution guild".to_owned(),
            scope_summary: "File-selection prompts with a declared default and buffered notebook previews"
                .to_owned(),
            input_kinds: vec![M5InputRequestKind::FilePathSelection],
            artifact_kinds: vec![M5ArtifactKind::PreviewEndpoint],
            freshness_classes: vec![M5ArtifactFreshness::Buffered],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["notebook_pane".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(notebook_partial_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::AiMediatedExecution,
            owner_role: "AI-execution guild".to_owned(),
            scope_summary:
                "Choice prompts for agent runs where a dismissal leaves the run visibly blocked, never silently failed"
                    .to_owned(),
            input_kinds: vec![M5InputRequestKind::Choice],
            artifact_kinds: vec![M5ArtifactKind::DiagnosticLog],
            freshness_classes: vec![M5ArtifactFreshness::Buffered],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::InputConsequenceUnknown,
                M5ExecutionDowngradeTrigger::ConnectorLost,
            ],
            consumer_surfaces: vec!["ai_pane".to_owned(), "companion".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(ai_dismissed_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::PublishFlow,
            owner_role: "Publish guild".to_owned(),
            scope_summary: "Artifact rows for publish runs, keeping bundle lineage and retention visible"
                .to_owned(),
            input_kinds: vec![],
            artifact_kinds: vec![M5ArtifactKind::Bundle],
            freshness_classes: vec![M5ArtifactFreshness::Buffered],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::ArtifactLineageLost],
            consumer_surfaces: vec!["publish_pane".to_owned(), "release_control".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(publish_passed_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::PreviewFlow,
            owner_role: "Preview guild".to_owned(),
            scope_summary: "Artifact rows for preview renders, disclosing buffered preview endpoints"
                .to_owned(),
            input_kinds: vec![],
            artifact_kinds: vec![M5ArtifactKind::PreviewEndpoint],
            freshness_classes: vec![M5ArtifactFreshness::Buffered],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["preview_pane".to_owned(), "companion".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(preview_passed_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::HistoryActivityCenter,
            owner_role: "History / activity-center guild".to_owned(),
            scope_summary:
                "History rows keeping evicted artifacts attributable to their producing run via lineage"
                    .to_owned(),
            input_kinds: vec![],
            artifact_kinds: vec![M5ArtifactKind::Report],
            freshness_classes: vec![M5ArtifactFreshness::Buffered],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
                M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
            ],
            consumer_surfaces: vec!["history".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(history_evicted_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::SupportExportReplay,
            owner_role: "Support / diagnostics guild".to_owned(),
            scope_summary:
                "Offline replay reconstructing imported provider artifacts, disclosed as imported"
                    .to_owned(),
            input_kinds: vec![],
            artifact_kinds: vec![M5ArtifactKind::ImportedProviderArtifact],
            freshness_classes: vec![M5ArtifactFreshness::Imported],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_interactions: vec![case(support_imported_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
        M5InteractionSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::CompanionSummary,
            owner_role: "Companion-surface guild".to_owned(),
            scope_summary:
                "Device/browser handoff prompts whose timeout cancels the run, and provider-supplied artifacts shown as such"
                    .to_owned(),
            input_kinds: vec![M5InputRequestKind::DeviceBrowserHandoff],
            artifact_kinds: vec![M5ArtifactKind::ImportedProviderArtifact],
            freshness_classes: vec![M5ArtifactFreshness::ProviderSupplied],
            export_fields: all_export_fields,
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::ConnectorLost],
            consumer_surfaces: vec!["companion".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs,
            example_interactions: vec![case(companion_timed_out_input())],
            hides_input_consequence: false,
            drops_artifact_lineage: false,
            hides_artifact_freshness: false,
            drops_export_ids_or_states: false,
        },
    ]
}

fn seeded_governance_review() -> M5InteractionGovernanceReview {
    M5InteractionGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        input_consequences_never_silent: true,
        artifact_lineage_preserved_after_pane_clears: true,
        artifact_freshness_disclosed_before_action: true,
        exported_evidence_preserves_ids_and_states: true,
        support_export_reconstructs_interaction: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5InteractionConsumerProjection {
    M5InteractionConsumerProjection {
        execution_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        artifact_rows_read_single_run_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5InteractionReleasePosture {
    M5InteractionReleasePosture {
        release_packet_ref: M5_EXECUTION_INTERACTION_ARTIFACT_REF.to_owned(),
        interaction_audit_ref: M5_EXECUTION_INTERACTION_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 interaction primitive packet. This is the one
/// source of truth shared by the tests, the fixture-emitting example, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_execution_interaction_packet() -> M5ExecutionInteractionPrimitivePacket {
    M5ExecutionInteractionPrimitivePacket::new(M5ExecutionInteractionPrimitivePacketInput {
        packet_id: "m5-input-request-artifact-publish-primitive:stable:0001".to_owned(),
        matrix_label: "M5 Input-Request / Artifact-Publish Primitive".to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5InteractionVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_EXECUTION_INTERACTION_SCHEMA_REF.to_owned(),
            M5_EXECUTION_INTERACTION_DOC_REF.to_owned(),
            M5_EXECUTION_INTERACTION_COMPONENT_MATRIX_REF.to_owned(),
            M5_EXECUTION_INTERACTION_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
