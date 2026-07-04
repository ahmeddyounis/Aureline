// Canonical seed for the M5 rerun-comparison-sheet primitive. Included from `mod.rs` so
// the seeded builder, its worked cases, the fixture-emitting example, and the on-disk
// support export all stay byte-aligned.

/// A task rerun against the current context: the inputs changed since the prior attempt
/// while the runtime is unchanged, so `Rerun exactly` and `Rerun with current context`
/// stay distinct reviewed actions.
fn task_current_context_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:task:build-and-test:0001".to_owned(),
        prior_run_ref: "run:build-and-test:0001".to_owned(),
        prior_attempt_ref: "attempt:build-and-test:0001#2".to_owned(),
        prior_attempt_ordinal: 2,
        new_attempt_ordinal: 3,
        prior_run_outcome: M5RunOutcome::Passed,
        baseline_run_ref: "run:build-and-test:0001".to_owned(),
        run_label: "build-and-test".to_owned(),
        context_summary: "rerun the workspace build-and-test after the source changed".to_owned(),
        age_label: "4m ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Local,
        rerun_mode: M5RerunMode::RerunWithCurrentContext,
        rerun_context: M5RerunContext::CurrentContext,
        retry_scope: M5RetryScope::WholeRun,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
        ],
        prior_side_effect_class: M5SideEffectClass::None,
        rerun_side_effect_class: M5SideEffectClass::None,
        difference_reason:
            "the source tree changed since attempt #2, so a current-context rerun compiles different inputs than an exact replay"
                .to_owned(),
        changed_dimensions: vec![
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::Input,
                state: M5RerunChangeState::Changed,
                before_label: Some("workspace at commit a1b2c3".to_owned()),
                after_label: Some("workspace at commit d4e5f6".to_owned()),
                detail: "the tracked source changed between attempts".to_owned(),
            },
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::Runtime,
                state: M5RerunChangeState::Unchanged,
                before_label: None,
                after_label: None,
                detail: "same stable toolchain as the prior attempt".to_owned(),
            },
        ],
        degraded: None,
    }
}

/// A test retry of only the failed step: the prior run failed and the reviewed action is
/// a failed-step-only retry, distinct from a whole-run rerun.
fn test_retry_failed_step_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:test:integration:0002".to_owned(),
        prior_run_ref: "run:integration:0002".to_owned(),
        prior_attempt_ref: "attempt:integration:0002#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::Failed,
        baseline_run_ref: "run:integration:0002".to_owned(),
        run_label: "integration suite".to_owned(),
        context_summary: "retry only the failed integration tests from attempt #1".to_owned(),
        age_label: "9m ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Local,
        rerun_mode: M5RerunMode::RetryFailedStepOnly,
        rerun_context: M5RerunContext::ModifiedSelection,
        retry_scope: M5RetryScope::FailedStepOnly,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
            M5RerunMode::RetryFailedStepOnly,
        ],
        prior_side_effect_class: M5SideEffectClass::None,
        rerun_side_effect_class: M5SideEffectClass::None,
        difference_reason:
            "attempt #1 failed 3 of 48 tests; the retry re-runs only that failed selection, not the whole suite"
                .to_owned(),
        changed_dimensions: vec![M5RerunChangeInput {
            dimension: M5RerunChangeDimension::Input,
            state: M5RerunChangeState::Changed,
            before_label: Some("48 tests selected".to_owned()),
            after_label: Some("3 failed tests selected".to_owned()),
            detail: "the retry narrows the selection to the failed units".to_owned(),
        }],
        degraded: None,
    }
}

/// A request rerun whose approval / auth posture and target endpoint both changed, so
/// the changed authority is reviewed before the request is re-dispatched.
fn request_authority_changed_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:request:deploy:0003".to_owned(),
        prior_run_ref: "run:deploy-request:0003".to_owned(),
        prior_attempt_ref: "attempt:deploy-request:0003#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::Passed,
        baseline_run_ref: "run:deploy-request:0003".to_owned(),
        run_label: "deploy request".to_owned(),
        context_summary: "re-dispatch the deploy request against the current endpoint".to_owned(),
        age_label: "22m ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Remote,
        rerun_mode: M5RerunMode::RerunWithCurrentContext,
        rerun_context: M5RerunContext::CurrentContext,
        retry_scope: M5RetryScope::WholeRun,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
        ],
        prior_side_effect_class: M5SideEffectClass::ExternalWrite,
        rerun_side_effect_class: M5SideEffectClass::ExternalWrite,
        difference_reason:
            "the deploy token was rotated and the target endpoint moved, so the current-context rerun runs under different authority than attempt #1"
                .to_owned(),
        changed_dimensions: vec![
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::ApprovalAuthority,
                state: M5RerunChangeState::Changed,
                before_label: Some("deploy token minted 2026-06-30".to_owned()),
                after_label: Some("deploy token rotated 2026-07-03".to_owned()),
                detail: "the authorizing credential was rotated between attempts".to_owned(),
            },
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::Target,
                state: M5RerunChangeState::Changed,
                before_label: Some("deploy.blue.internal".to_owned()),
                after_label: Some("deploy.green.internal".to_owned()),
                detail: "the deploy endpoint host changed".to_owned(),
            },
        ],
        degraded: None,
    }
}

/// A notebook rerun whose runtime and profile both changed, reviewed against a modified
/// environment before dispatch.
fn notebook_environment_changed_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:notebook:analysis:0004".to_owned(),
        prior_run_ref: "run:notebook-analysis:0004".to_owned(),
        prior_attempt_ref: "attempt:notebook-analysis:0004#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::PartiallyComplete,
        baseline_run_ref: "run:notebook-analysis:0004".to_owned(),
        run_label: "analysis notebook".to_owned(),
        context_summary: "re-run the analysis notebook after the kernel was upgraded".to_owned(),
        age_label: "1h ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Container,
        rerun_mode: M5RerunMode::RerunWithCurrentContext,
        rerun_context: M5RerunContext::ModifiedEnvironment,
        retry_scope: M5RetryScope::WholeRun,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
        ],
        prior_side_effect_class: M5SideEffectClass::LocalWrite,
        rerun_side_effect_class: M5SideEffectClass::LocalWrite,
        difference_reason:
            "the notebook kernel and memory profile changed, so a current-context rerun executes on a different environment than the recorded attempt"
                .to_owned(),
        changed_dimensions: vec![
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::Runtime,
                state: M5RerunChangeState::Changed,
                before_label: Some("python 3.11 kernel".to_owned()),
                after_label: Some("python 3.12 kernel".to_owned()),
                detail: "the notebook kernel was upgraded between attempts".to_owned(),
            },
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::Profile,
                state: M5RerunChangeState::Changed,
                before_label: Some("2 GiB memory profile".to_owned()),
                after_label: Some("4 GiB memory profile".to_owned()),
                detail: "the launch profile memory limit was raised".to_owned(),
            },
        ],
        degraded: None,
    }
}

/// An AI-mediated rerun whose side-effect class escalates from read-only to an external
/// write; the escalation is disclosed as a reviewable change before dispatch.
fn ai_side_effect_escalates_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:ai:refactor-agent:0005".to_owned(),
        prior_run_ref: "run:refactor-agent:0005".to_owned(),
        prior_attempt_ref: "attempt:refactor-agent:0005#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::Failed,
        baseline_run_ref: "run:refactor-agent:0005".to_owned(),
        run_label: "refactor agent run".to_owned(),
        context_summary: "re-run the refactor agent, now permitted to write the changes".to_owned(),
        age_label: "35m ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Managed,
        rerun_mode: M5RerunMode::RerunWithCurrentContext,
        rerun_context: M5RerunContext::CurrentContext,
        retry_scope: M5RetryScope::WholeRun,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
            M5RerunMode::RetryFailedStepOnly,
        ],
        prior_side_effect_class: M5SideEffectClass::ReadOnly,
        rerun_side_effect_class: M5SideEffectClass::ExternalWrite,
        difference_reason:
            "attempt #1 ran read-only in dry-run; this rerun is authorized to write, so it escalates the side-effect class"
                .to_owned(),
        changed_dimensions: vec![M5RerunChangeInput {
            dimension: M5RerunChangeDimension::SideEffectClass,
            state: M5RerunChangeState::Changed,
            before_label: Some("read-only dry run".to_owned()),
            after_label: Some("external write to the repository".to_owned()),
            detail: "the rerun escalates from a dry run to an applied write".to_owned(),
        }],
        degraded: None,
    }
}

/// A publish rerun that is an exact replay: no reviewable context change, so the
/// reviewed actions are semantically equivalent and may be offered as one action.
fn publish_exact_replay_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:publish:release-bundle:0006".to_owned(),
        prior_run_ref: "run:release-bundle:0006".to_owned(),
        prior_attempt_ref: "attempt:release-bundle:0006#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::Failed,
        baseline_run_ref: "run:release-bundle:0006".to_owned(),
        run_label: "release bundle publish".to_owned(),
        context_summary: "re-publish the exact recorded release bundle after a transient failure"
            .to_owned(),
        age_label: "12m ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Remote,
        rerun_mode: M5RerunMode::RerunExactly,
        rerun_context: M5RerunContext::ExactReplay,
        retry_scope: M5RetryScope::WholeRun,
        available_modes: vec![M5RerunMode::RerunExactly],
        prior_side_effect_class: M5SideEffectClass::Irreversible,
        rerun_side_effect_class: M5SideEffectClass::Irreversible,
        difference_reason:
            "the prior publish failed transiently; the exact replay reuses the recorded selection, environment, and inputs unchanged"
                .to_owned(),
        changed_dimensions: vec![
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::Target,
                state: M5RerunChangeState::NotApplicable,
                before_label: None,
                after_label: None,
                detail: "an exact replay reuses the recorded target".to_owned(),
            },
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::SideEffectClass,
                state: M5RerunChangeState::Unchanged,
                before_label: None,
                after_label: None,
                detail: "the irreversible publish class is unchanged from the prior attempt"
                    .to_owned(),
            },
        ],
        degraded: None,
    }
}

/// A preview rerun whose target viewport changed while its profile is unchanged, rerun
/// against the current context over a selected subset.
fn preview_target_changed_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:preview:render:0007".to_owned(),
        prior_run_ref: "run:preview-render:0007".to_owned(),
        prior_attempt_ref: "attempt:preview-render:0007#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::Passed,
        baseline_run_ref: "run:preview-render:0007".to_owned(),
        run_label: "preview render".to_owned(),
        context_summary: "re-render the preview at a changed viewport".to_owned(),
        age_label: "6m ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Local,
        rerun_mode: M5RerunMode::RerunWithCurrentContext,
        rerun_context: M5RerunContext::CurrentContext,
        retry_scope: M5RetryScope::SelectedSubset,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
        ],
        prior_side_effect_class: M5SideEffectClass::None,
        rerun_side_effect_class: M5SideEffectClass::None,
        difference_reason:
            "the preview viewport changed from desktop to mobile width, so the current-context render targets a different layout than attempt #1"
                .to_owned(),
        changed_dimensions: vec![
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::Target,
                state: M5RerunChangeState::Changed,
                before_label: Some("1440px desktop viewport".to_owned()),
                after_label: Some("768px mobile viewport".to_owned()),
                detail: "the preview viewport width changed between attempts".to_owned(),
            },
            M5RerunChangeInput {
                dimension: M5RerunChangeDimension::Profile,
                state: M5RerunChangeState::Unchanged,
                before_label: None,
                after_label: None,
                detail: "the render profile is unchanged".to_owned(),
            },
        ],
        degraded: None,
    }
}

/// A history rerun whose input equivalence cannot be confirmed from the activity center,
/// so the input dimension is reviewed as unknown rather than assumed unchanged.
fn history_unknown_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:history:nightly:0008".to_owned(),
        prior_run_ref: "run:nightly:0008".to_owned(),
        prior_attempt_ref: "attempt:nightly:0008#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::StaleOutput,
        baseline_run_ref: "run:nightly:0008".to_owned(),
        run_label: "nightly build".to_owned(),
        context_summary: "re-run a nightly build from the activity-center history".to_owned(),
        age_label: "9h ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Local,
        rerun_mode: M5RerunMode::RerunWithCurrentContext,
        rerun_context: M5RerunContext::CurrentContext,
        retry_scope: M5RetryScope::WholeRun,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
        ],
        prior_side_effect_class: M5SideEffectClass::LocalWrite,
        rerun_side_effect_class: M5SideEffectClass::LocalWrite,
        difference_reason:
            "the nightly output went stale and history no longer retains the recorded inputs, so the product cannot confirm the inputs are unchanged"
                .to_owned(),
        changed_dimensions: vec![M5RerunChangeInput {
            dimension: M5RerunChangeDimension::Input,
            state: M5RerunChangeState::Unknown,
            before_label: None,
            after_label: None,
            detail: "the recorded inputs were evicted from history and cannot be compared"
                .to_owned(),
        }],
        degraded: Some(DegradedState {
            trigger: M5ExecutionDowngradeTrigger::RerunContextDrift,
            degraded_label:
                "the recorded inputs are no longer retained, so this rerun runs against the current context and cannot claim an exact replay"
                    .to_owned(),
        }),
    }
}

/// A support / export replay reconstructing a reviewed rerun of an imported run whose
/// toolchain differs from the pinned baseline.
fn support_imported_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:support:imported-ci:0009".to_owned(),
        prior_run_ref: "run:imported-ci:0009".to_owned(),
        prior_attempt_ref: "attempt:imported-ci:0009#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::Failed,
        baseline_run_ref: "run:imported-ci:0009".to_owned(),
        run_label: "imported CI run".to_owned(),
        context_summary: "reconstruct the reviewed rerun of an imported CI failure".to_owned(),
        age_label: "2d ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Imported,
        target_boundary: M5ExecutionLocality::Remote,
        rerun_mode: M5RerunMode::RerunWithCurrentContext,
        rerun_context: M5RerunContext::ModifiedEnvironment,
        retry_scope: M5RetryScope::WholeRun,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
            M5RerunMode::RetryFailedStepOnly,
        ],
        prior_side_effect_class: M5SideEffectClass::None,
        rerun_side_effect_class: M5SideEffectClass::None,
        difference_reason:
            "the imported CI run pinned a toolchain that differs from the current environment, so a local rerun would use a different runtime"
                .to_owned(),
        changed_dimensions: vec![M5RerunChangeInput {
            dimension: M5RerunChangeDimension::Runtime,
            state: M5RerunChangeState::Changed,
            before_label: Some("imported toolchain 1.74".to_owned()),
            after_label: Some("current toolchain 1.78".to_owned()),
            detail: "the imported run pinned an older toolchain than the current environment"
                .to_owned(),
        }],
        degraded: None,
    }
}

/// A companion-surface retry of a partially-complete run's failed step, reviewing the
/// narrowed input selection before dispatch.
fn companion_retry_failed_step_input() -> M5RerunReviewInput {
    M5RerunReviewInput {
        sheet_id: "rerun:companion:batch-job:0010".to_owned(),
        prior_run_ref: "run:batch-job:0010".to_owned(),
        prior_attempt_ref: "attempt:batch-job:0010#1".to_owned(),
        prior_attempt_ordinal: 1,
        new_attempt_ordinal: 2,
        prior_run_outcome: M5RunOutcome::PartiallyComplete,
        baseline_run_ref: "run:batch-job:0010".to_owned(),
        run_label: "batch job".to_owned(),
        context_summary: "retry only the failed shard of a partially-complete batch job".to_owned(),
        age_label: "18m ago".to_owned(),
        truth_mode: M5ExecutionTruthMode::Captured,
        target_boundary: M5ExecutionLocality::Managed,
        rerun_mode: M5RerunMode::RetryFailedStepOnly,
        rerun_context: M5RerunContext::ModifiedSelection,
        retry_scope: M5RetryScope::FailedStepOnly,
        available_modes: vec![
            M5RerunMode::RerunExactly,
            M5RerunMode::RerunWithCurrentContext,
            M5RerunMode::RetryFailedStepOnly,
        ],
        prior_side_effect_class: M5SideEffectClass::LocalWrite,
        rerun_side_effect_class: M5SideEffectClass::LocalWrite,
        difference_reason:
            "attempt #1 completed 7 of 8 shards; the retry re-runs only the one failed shard, not the completed work"
                .to_owned(),
        changed_dimensions: vec![M5RerunChangeInput {
            dimension: M5RerunChangeDimension::Input,
            state: M5RerunChangeState::Changed,
            before_label: Some("8 shards selected".to_owned()),
            after_label: Some("1 failed shard selected".to_owned()),
            detail: "the retry narrows the selection to the failed shard".to_owned(),
        }],
        degraded: None,
    }
}

fn case(input: M5RerunReviewInput) -> M5RerunReviewCase {
    M5RerunReviewCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5RerunSurfaceRow> {
    let base_source_refs = vec![
        M5_RERUN_REVIEW_SCHEMA_REF.to_owned(),
        M5_RERUN_REVIEW_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5RerunExportField::ALL.to_vec();

    vec![
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::TaskRunPane,
            owner_role: "Task-run guild".to_owned(),
            scope_summary:
                "Rerun sheets for task runs, keeping exact replay and current-context reruns distinct when the source has changed"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
            ],
            change_dimensions: vec![
                M5RerunChangeDimension::Input,
                M5RerunChangeDimension::Runtime,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::RerunContextDrift,
                M5ExecutionDowngradeTrigger::RunAttemptIdentityUnresolved,
            ],
            consumer_surfaces: vec!["task_pane".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(task_current_context_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::TestRunPane,
            owner_role: "Test-run guild".to_owned(),
            scope_summary:
                "Rerun sheets for test runs offering a failed-step-only retry distinct from a whole-run rerun"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
                M5RerunMode::RetryFailedStepOnly,
            ],
            change_dimensions: vec![M5RerunChangeDimension::Input],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::RerunContextDrift],
            consumer_surfaces: vec!["test_pane".to_owned(), "history".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(test_retry_failed_step_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::RequestRunPane,
            owner_role: "Request-execution guild".to_owned(),
            scope_summary:
                "Rerun sheets for API requests, reviewing changed approval / authority and target before re-dispatch"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
            ],
            change_dimensions: vec![
                M5RerunChangeDimension::ApprovalAuthority,
                M5RerunChangeDimension::Target,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::RerunContextDrift,
                M5ExecutionDowngradeTrigger::ConnectorLost,
            ],
            consumer_surfaces: vec!["request_pane".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(request_authority_changed_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::NotebookExecution,
            owner_role: "Notebook-execution guild".to_owned(),
            scope_summary:
                "Rerun sheets for notebooks reviewing a changed runtime and profile before re-running"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
            ],
            change_dimensions: vec![
                M5RerunChangeDimension::Runtime,
                M5RerunChangeDimension::Profile,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::RerunContextDrift],
            consumer_surfaces: vec!["notebook_pane".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(notebook_environment_changed_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::AiMediatedExecution,
            owner_role: "AI-execution guild".to_owned(),
            scope_summary:
                "Rerun sheets for agent runs disclosing a side-effect escalation before a write-authorized rerun"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
                M5RerunMode::RetryFailedStepOnly,
            ],
            change_dimensions: vec![M5RerunChangeDimension::SideEffectClass],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::RerunContextDrift,
                M5ExecutionDowngradeTrigger::ConnectorLost,
            ],
            consumer_surfaces: vec!["ai_pane".to_owned(), "companion".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(ai_side_effect_escalates_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::PublishFlow,
            owner_role: "Publish guild".to_owned(),
            scope_summary:
                "Rerun sheets for publish runs, offering one action for an exact replay when nothing has changed"
                    .to_owned(),
            rerun_modes: vec![M5RerunMode::RerunExactly],
            change_dimensions: vec![
                M5RerunChangeDimension::Target,
                M5RerunChangeDimension::SideEffectClass,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["publish_pane".to_owned(), "release_control".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(publish_exact_replay_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::PreviewFlow,
            owner_role: "Preview guild".to_owned(),
            scope_summary:
                "Rerun sheets for preview renders reviewing a changed target viewport before re-rendering"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
            ],
            change_dimensions: vec![
                M5RerunChangeDimension::Target,
                M5RerunChangeDimension::Profile,
            ],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["preview_pane".to_owned(), "companion".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(preview_target_changed_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::HistoryActivityCenter,
            owner_role: "History / activity-center guild".to_owned(),
            scope_summary:
                "Rerun sheets from history flagging inputs that cannot be confirmed unchanged as unknown, not assumed exact"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
            ],
            change_dimensions: vec![M5RerunChangeDimension::Input],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::RerunContextDrift,
                M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
            ],
            consumer_surfaces: vec!["history".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(history_unknown_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::SupportExportReplay,
            owner_role: "Support / diagnostics guild".to_owned(),
            scope_summary:
                "Offline replay reconstructing a reviewed rerun of an imported run with a changed toolchain"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
                M5RerunMode::RetryFailedStepOnly,
            ],
            change_dimensions: vec![M5RerunChangeDimension::Runtime],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reruns: vec![case(support_imported_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
        M5RerunSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::CompanionSummary,
            owner_role: "Companion-surface guild".to_owned(),
            scope_summary:
                "Rerun sheets on the companion surface retrying only the failed step of a partially-complete run"
                    .to_owned(),
            rerun_modes: vec![
                M5RerunMode::RerunExactly,
                M5RerunMode::RerunWithCurrentContext,
                M5RerunMode::RetryFailedStepOnly,
            ],
            change_dimensions: vec![M5RerunChangeDimension::Input],
            export_fields: all_export_fields,
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::ConnectorLost],
            consumer_surfaces: vec!["companion".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs,
            example_reruns: vec![case(companion_retry_failed_step_input())],
            collapses_distinct_actions: false,
            hides_changed_context: false,
            drops_prior_lineage: false,
            drops_export_mode_or_summary: false,
        },
    ]
}

fn seeded_governance_review() -> M5RerunGovernanceReview {
    M5RerunGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        distinct_rerun_actions_never_collapsed: true,
        changed_context_reviewable_before_dispatch: true,
        prior_attempt_lineage_preserved: true,
        reviewed_mode_and_summary_survive_export: true,
        support_export_reconstructs_rerun: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5RerunConsumerProjection {
    M5RerunConsumerProjection {
        execution_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        change_rows_read_single_diff_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5RerunReleasePosture {
    M5RerunReleasePosture {
        release_packet_ref: M5_RERUN_REVIEW_ARTIFACT_REF.to_owned(),
        rerun_audit_ref: M5_RERUN_REVIEW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 rerun primitive packet. This is the one source of
/// truth shared by the tests, the fixture-emitting example, and the on-disk support
/// export so all three stay byte-aligned.
pub fn seeded_m5_rerun_review_packet() -> M5RerunReviewPrimitivePacket {
    M5RerunReviewPrimitivePacket::new(M5RerunReviewPrimitivePacketInput {
        packet_id: "m5-rerun-comparison-sheet-primitive:stable:0001".to_owned(),
        matrix_label: "M5 Rerun-Comparison-Sheet Primitive".to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5RerunVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_RERUN_REVIEW_SCHEMA_REF.to_owned(),
            M5_RERUN_REVIEW_DOC_REF.to_owned(),
            M5_RERUN_REVIEW_COMPONENT_MATRIX_REF.to_owned(),
            M5_RERUN_REVIEW_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
