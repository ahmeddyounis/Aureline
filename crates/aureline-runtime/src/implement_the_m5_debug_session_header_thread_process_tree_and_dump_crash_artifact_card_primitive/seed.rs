// Canonical seed for the M5 debug-session-hierarchy primitive. Included from `mod.rs` so
// the seeded builder, its worked cases, the fixture-emitting example, and the on-disk
// support export all stay byte-aligned.

/// A task debug session launched under live attached control: the debugger launched the
/// target and it is actively running, so the header reads as live control against live
/// truth.
fn task_launch_live_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:task:build-runner:0001".to_owned(),
        session_ref: "session:build-runner:0001".to_owned(),
        target_ref: "process:build-runner:0001".to_owned(),
        session_label: "build-runner debug".to_owned(),
        context_summary: "launch the build runner under the debugger and step into it".to_owned(),
        age_label: "just now".to_owned(),
        session_mode: M5DebugSessionMode::Launch,
        truth_mode: M5ExecutionTruthMode::Live,
        locality: M5ExecutionLocality::Local,
        adapter_state: M5DebugAdapterState::Connected,
        stop_reason: M5DebugStopReason::Running,
        session_outcome: M5RunOutcome::Running,
        restored: false,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:build-runner:0001".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "build-runner (pid 4821)".to_owned(),
                thread_count: 2,
                run_state: M5ThreadRunState::Running,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::PauseExecution],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:build-runner:0001#main".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:build-runner:0001".to_owned()),
                label: "main".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Running,
                is_selected: true,
                available_actions: vec![
                    M5DebugActionKind::SwitchThread,
                    M5DebugActionKind::PauseExecution,
                ],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:build-runner:0001#worker".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:build-runner:0001".to_owned()),
                label: "worker".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Waiting,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::SwitchThread],
            },
        ],
        selected_thread_ref: Some("thread:build-runner:0001#main".to_owned()),
        dump_cards: vec![],
        degraded: None,
    }
}

/// A test debug session attached to a running process and stopped at a breakpoint: live
/// attached control, threads paused.
fn test_attach_breakpoint_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:test:integration:0002".to_owned(),
        session_ref: "session:integration:0002".to_owned(),
        target_ref: "process:integration:0002".to_owned(),
        session_label: "integration test debug".to_owned(),
        context_summary: "attach to the integration test process stopped at a breakpoint"
            .to_owned(),
        age_label: "1m ago".to_owned(),
        session_mode: M5DebugSessionMode::Attach,
        truth_mode: M5ExecutionTruthMode::Live,
        locality: M5ExecutionLocality::Local,
        adapter_state: M5DebugAdapterState::Connected,
        stop_reason: M5DebugStopReason::Breakpoint,
        session_outcome: M5RunOutcome::Running,
        restored: false,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:integration:0002".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "integration-tests (pid 5120)".to_owned(),
                thread_count: 3,
                run_state: M5ThreadRunState::Paused,
                is_selected: false,
                available_actions: vec![
                    M5DebugActionKind::ContinueExecution,
                    M5DebugActionKind::DetachSession,
                ],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:integration:0002#main".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:integration:0002".to_owned()),
                label: "main".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Paused,
                is_selected: true,
                available_actions: vec![
                    M5DebugActionKind::SwitchThread,
                    M5DebugActionKind::ContinueExecution,
                ],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:integration:0002#pool-1".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:integration:0002".to_owned()),
                label: "pool-1".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Running,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::SwitchThread],
            },
        ],
        selected_thread_ref: Some("thread:integration:0002#main".to_owned()),
        dump_cards: vec![],
        degraded: None,
    }
}

/// An inspect-only view of an imported request-execution debug snapshot: no live control,
/// no adapter, so the header reads as an inspect-only view (AC1 narrowed).
fn request_inspect_only_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:request:webhook:0003".to_owned(),
        session_ref: "session:webhook:0003".to_owned(),
        target_ref: "process:webhook:0003".to_owned(),
        session_label: "webhook request inspect".to_owned(),
        context_summary: "inspect an imported request-execution snapshot with no live control"
            .to_owned(),
        age_label: "20m ago".to_owned(),
        session_mode: M5DebugSessionMode::InspectOnly,
        truth_mode: M5ExecutionTruthMode::Imported,
        locality: M5ExecutionLocality::Remote,
        adapter_state: M5DebugAdapterState::Disconnected,
        stop_reason: M5DebugStopReason::Exception,
        session_outcome: M5RunOutcome::Failed,
        restored: false,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:webhook:0003".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "webhook-handler (imported)".to_owned(),
                thread_count: 1,
                run_state: M5ThreadRunState::Unknown,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::CopyReference],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:webhook:0003#handler".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:webhook:0003".to_owned()),
                label: "handler".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Unknown,
                is_selected: true,
                available_actions: vec![
                    M5DebugActionKind::SwitchThread,
                    M5DebugActionKind::OpenInEditor,
                ],
            },
        ],
        selected_thread_ref: Some("thread:webhook:0003#handler".to_owned()),
        dump_cards: vec![],
        degraded: None,
    }
}

/// A notebook time-travel replay session: captured analysis of a recorded execution, no
/// live control.
fn notebook_replay_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:notebook:analysis:0004".to_owned(),
        session_ref: "session:notebook-analysis:0004".to_owned(),
        target_ref: "process:notebook-analysis:0004".to_owned(),
        session_label: "analysis notebook replay".to_owned(),
        context_summary: "replay the recorded notebook kernel execution to inspect a step"
            .to_owned(),
        age_label: "40m ago".to_owned(),
        session_mode: M5DebugSessionMode::Replay,
        truth_mode: M5ExecutionTruthMode::Captured,
        locality: M5ExecutionLocality::Container,
        adapter_state: M5DebugAdapterState::Unavailable,
        stop_reason: M5DebugStopReason::StepComplete,
        session_outcome: M5RunOutcome::PartiallyComplete,
        restored: false,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:notebook-analysis:0004".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "python-kernel (recorded)".to_owned(),
                thread_count: 1,
                run_state: M5ThreadRunState::Paused,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::CopyReference],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:notebook-analysis:0004#kernel".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:notebook-analysis:0004".to_owned()),
                label: "kernel".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Paused,
                is_selected: true,
                available_actions: vec![
                    M5DebugActionKind::SwitchThread,
                    M5DebugActionKind::OpenInEditor,
                ],
            },
        ],
        selected_thread_ref: Some("thread:notebook-analysis:0004#kernel".to_owned()),
        dump_cards: vec![],
        degraded: None,
    }
}

/// An AI-agent debug session attached to a managed runtime with a restored adapter: live
/// attached control even after a reconnect.
fn ai_attach_restored_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:ai:refactor-agent:0005".to_owned(),
        session_ref: "session:refactor-agent:0005".to_owned(),
        target_ref: "process:refactor-agent:0005".to_owned(),
        session_label: "refactor agent debug".to_owned(),
        context_summary: "attach to the managed agent runtime after the adapter reconnected"
            .to_owned(),
        age_label: "3m ago".to_owned(),
        session_mode: M5DebugSessionMode::Attach,
        truth_mode: M5ExecutionTruthMode::Live,
        locality: M5ExecutionLocality::Managed,
        adapter_state: M5DebugAdapterState::Restored,
        stop_reason: M5DebugStopReason::Signal,
        session_outcome: M5RunOutcome::Running,
        restored: true,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:refactor-agent:0005".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "agent-runtime (managed)".to_owned(),
                thread_count: 2,
                run_state: M5ThreadRunState::Running,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::DetachSession],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:refactor-agent:0005#driver".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:refactor-agent:0005".to_owned()),
                label: "driver".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Running,
                is_selected: true,
                available_actions: vec![
                    M5DebugActionKind::SwitchThread,
                    M5DebugActionKind::ContinueExecution,
                ],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:refactor-agent:0005#tool".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:refactor-agent:0005".to_owned()),
                label: "tool-worker".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Paused,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::SwitchThread],
            },
        ],
        selected_thread_ref: Some("thread:refactor-agent:0005#driver".to_owned()),
        dump_cards: vec![],
        degraded: None,
    }
}

/// A publish crash analyzed from a full core dump: captured analysis with a fully
/// symbolicated dump card.
fn publish_core_symbolicated_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:publish:release-bundle:0006".to_owned(),
        session_ref: "session:release-bundle:0006".to_owned(),
        target_ref: "process:release-bundle:0006".to_owned(),
        session_label: "release publish crash".to_owned(),
        context_summary: "analyze the full core dump captured when the publish worker crashed"
            .to_owned(),
        age_label: "15m ago".to_owned(),
        session_mode: M5DebugSessionMode::Core,
        truth_mode: M5ExecutionTruthMode::Captured,
        locality: M5ExecutionLocality::Remote,
        adapter_state: M5DebugAdapterState::Unavailable,
        stop_reason: M5DebugStopReason::CrashCapture,
        session_outcome: M5RunOutcome::Failed,
        restored: false,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:release-bundle:0006".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "publish-worker (crashed)".to_owned(),
                thread_count: 1,
                run_state: M5ThreadRunState::Exited,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::CopyReference],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:release-bundle:0006#faulting".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:release-bundle:0006".to_owned()),
                label: "faulting thread".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Exited,
                is_selected: true,
                available_actions: vec![
                    M5DebugActionKind::SwitchThread,
                    M5DebugActionKind::OpenInEditor,
                ],
            },
        ],
        selected_thread_ref: Some("thread:release-bundle:0006#faulting".to_owned()),
        dump_cards: vec![M5DumpCardInput {
            dump_ref: "dump:release-bundle:0006#core".to_owned(),
            producing_run_ref: "run:release-bundle:0006".to_owned(),
            artifact_kind: M5DumpArtifactKind::FullCore,
            symbolication: M5SymbolicationState::Symbolicated,
            capture_time_label: "captured 15m ago".to_owned(),
            build_provenance_label: "exact build release-bundle 1.4.2+a1b2c3".to_owned(),
            symbol_provenance_label: "symbols matched from the release symbol store".to_owned(),
            retention: M5RetentionClass::RetainedDurable,
            available_actions: vec![
                M5DebugActionKind::OpenRawDump,
                M5DebugActionKind::ExportEvidence,
                M5DebugActionKind::CopyReference,
            ],
        }],
        degraded: None,
    }
}

/// A preview crash analyzed from a crash report with partial symbols: captured analysis
/// where the symbolication state is partial and disclosed.
fn preview_core_partial_symbols_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:preview:render:0007".to_owned(),
        session_ref: "session:preview-render:0007".to_owned(),
        target_ref: "process:preview-render:0007".to_owned(),
        session_label: "preview render crash".to_owned(),
        context_summary: "analyze the crash report captured when the preview renderer faulted"
            .to_owned(),
        age_label: "8m ago".to_owned(),
        session_mode: M5DebugSessionMode::Core,
        truth_mode: M5ExecutionTruthMode::Captured,
        locality: M5ExecutionLocality::Local,
        adapter_state: M5DebugAdapterState::Disconnected,
        stop_reason: M5DebugStopReason::CrashCapture,
        session_outcome: M5RunOutcome::Failed,
        restored: false,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:preview-render:0007".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "preview-renderer (crashed)".to_owned(),
                thread_count: 1,
                run_state: M5ThreadRunState::Exited,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::CopyReference],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:preview-render:0007#render".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:preview-render:0007".to_owned()),
                label: "render thread".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Exited,
                is_selected: true,
                available_actions: vec![M5DebugActionKind::SwitchThread],
            },
        ],
        selected_thread_ref: Some("thread:preview-render:0007#render".to_owned()),
        dump_cards: vec![M5DumpCardInput {
            dump_ref: "dump:preview-render:0007#crash".to_owned(),
            producing_run_ref: "run:preview-render:0007".to_owned(),
            artifact_kind: M5DumpArtifactKind::CrashReport,
            symbolication: M5SymbolicationState::PartialSymbols,
            capture_time_label: "captured 8m ago".to_owned(),
            build_provenance_label: "local build preview-render debug+d4e5f6".to_owned(),
            symbol_provenance_label: "some frames resolved; third-party frames unresolved"
                .to_owned(),
            retention: M5RetentionClass::ExpiresScheduled,
            available_actions: vec![
                M5DebugActionKind::OpenRawDump,
                M5DebugActionKind::ExportEvidence,
            ],
        }],
        degraded: None,
    }
}

/// A restored history debug session over a captured minidump with unsymbolicated frames:
/// AC1 narrowed via both restored and degraded, symbolication unsymbolicated.
fn history_restored_unsymbolicated_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:history:nightly:0008".to_owned(),
        session_ref: "session:nightly:0008".to_owned(),
        target_ref: "process:nightly:0008".to_owned(),
        session_label: "nightly crash minidump".to_owned(),
        context_summary: "reopen a restored nightly crash from history over its minidump"
            .to_owned(),
        age_label: "9h ago".to_owned(),
        session_mode: M5DebugSessionMode::Core,
        truth_mode: M5ExecutionTruthMode::Captured,
        locality: M5ExecutionLocality::Local,
        adapter_state: M5DebugAdapterState::Unavailable,
        stop_reason: M5DebugStopReason::CrashCapture,
        session_outcome: M5RunOutcome::Failed,
        restored: true,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:nightly:0008".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "nightly-build (crashed)".to_owned(),
                thread_count: 1,
                run_state: M5ThreadRunState::Exited,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::CopyReference],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:nightly:0008#main".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:nightly:0008".to_owned()),
                label: "main".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Exited,
                is_selected: true,
                available_actions: vec![M5DebugActionKind::SwitchThread],
            },
        ],
        selected_thread_ref: Some("thread:nightly:0008#main".to_owned()),
        dump_cards: vec![M5DumpCardInput {
            dump_ref: "dump:nightly:0008#minidump".to_owned(),
            producing_run_ref: "run:nightly:0008".to_owned(),
            artifact_kind: M5DumpArtifactKind::Minidump,
            symbolication: M5SymbolicationState::Unsymbolicated,
            capture_time_label: "captured 9h ago".to_owned(),
            build_provenance_label: "build id nightly-2026-07-03 recorded".to_owned(),
            symbol_provenance_label: "no symbols loaded; raw addresses only".to_owned(),
            retention: M5RetentionClass::EvictedRecoverable,
            available_actions: vec![
                M5DebugActionKind::OpenRawDump,
                M5DebugActionKind::CopyReference,
            ],
        }],
        degraded: Some(DegradedState {
            trigger: M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
            degraded_label:
                "only the captured minidump remains; there is no live process to control"
                    .to_owned(),
        }),
    }
}

/// A support / export replay over an imported heap snapshot whose symbols are unavailable:
/// captured analysis, symbolication symbols-unavailable, degraded.
fn support_replay_symbols_unavailable_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:support:imported-ci:0009".to_owned(),
        session_ref: "session:imported-ci:0009".to_owned(),
        target_ref: "process:imported-ci:0009".to_owned(),
        session_label: "imported CI heap analysis".to_owned(),
        context_summary: "replay an imported CI heap snapshot for support triage".to_owned(),
        age_label: "2d ago".to_owned(),
        session_mode: M5DebugSessionMode::Replay,
        truth_mode: M5ExecutionTruthMode::Imported,
        locality: M5ExecutionLocality::Remote,
        adapter_state: M5DebugAdapterState::Unavailable,
        stop_reason: M5DebugStopReason::Exception,
        session_outcome: M5RunOutcome::Failed,
        restored: false,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:imported-ci:0009".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "ci-worker (imported)".to_owned(),
                thread_count: 1,
                run_state: M5ThreadRunState::Unknown,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::CopyReference],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:imported-ci:0009#worker".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:imported-ci:0009".to_owned()),
                label: "worker".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Unknown,
                is_selected: true,
                available_actions: vec![
                    M5DebugActionKind::SwitchThread,
                    M5DebugActionKind::OpenInEditor,
                ],
            },
        ],
        selected_thread_ref: Some("thread:imported-ci:0009#worker".to_owned()),
        dump_cards: vec![M5DumpCardInput {
            dump_ref: "dump:imported-ci:0009#heap".to_owned(),
            producing_run_ref: "run:imported-ci:0009".to_owned(),
            artifact_kind: M5DumpArtifactKind::HeapSnapshot,
            symbolication: M5SymbolicationState::SymbolsUnavailable,
            capture_time_label: "captured 2d ago on the CI host".to_owned(),
            build_provenance_label: "imported build ci-9f8e7d with no symbol bundle".to_owned(),
            symbol_provenance_label: "symbols could not be resolved for the imported build"
                .to_owned(),
            retention: M5RetentionClass::EvictedGone,
            available_actions: vec![
                M5DebugActionKind::OpenRawDump,
                M5DebugActionKind::ExportEvidence,
                M5DebugActionKind::CopyReference,
            ],
        }],
        degraded: Some(DegradedState {
            trigger: M5ExecutionDowngradeTrigger::SymbolsUnavailable,
            degraded_label:
                "the imported build shipped no symbols, so frames stay at raw addresses"
                    .to_owned(),
        }),
    }
}

/// A companion-surface inspect-only view of a provider-reported debug snapshot: no live
/// control, no adapter (AC1 narrowed via inspect-only).
fn companion_inspect_only_input() -> M5DebugHierarchyInput {
    M5DebugHierarchyInput {
        session_id: "debug:companion:batch-job:0010".to_owned(),
        session_ref: "session:batch-job:0010".to_owned(),
        target_ref: "process:batch-job:0010".to_owned(),
        session_label: "batch job inspect".to_owned(),
        context_summary: "inspect a provider-reported batch-job debug snapshot on the companion"
            .to_owned(),
        age_label: "30m ago".to_owned(),
        session_mode: M5DebugSessionMode::InspectOnly,
        truth_mode: M5ExecutionTruthMode::ProviderReported,
        locality: M5ExecutionLocality::Managed,
        adapter_state: M5DebugAdapterState::Disconnected,
        stop_reason: M5DebugStopReason::EntryPoint,
        session_outcome: M5RunOutcome::PartiallyComplete,
        restored: false,
        tree_nodes: vec![
            M5DebugTreeNodeInput {
                node_ref: "process:batch-job:0010".to_owned(),
                node_kind: M5DebugNodeKind::Process,
                parent_ref: None,
                label: "batch-job (provider-reported)".to_owned(),
                thread_count: 1,
                run_state: M5ThreadRunState::Unknown,
                is_selected: false,
                available_actions: vec![M5DebugActionKind::CopyReference],
            },
            M5DebugTreeNodeInput {
                node_ref: "thread:batch-job:0010#shard-3".to_owned(),
                node_kind: M5DebugNodeKind::Thread,
                parent_ref: Some("process:batch-job:0010".to_owned()),
                label: "shard-3".to_owned(),
                thread_count: 0,
                run_state: M5ThreadRunState::Unknown,
                is_selected: true,
                available_actions: vec![M5DebugActionKind::SwitchThread],
            },
        ],
        selected_thread_ref: Some("thread:batch-job:0010#shard-3".to_owned()),
        dump_cards: vec![],
        degraded: None,
    }
}

fn case(input: M5DebugHierarchyInput) -> M5DebugHierarchyCase {
    M5DebugHierarchyCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5DebugSurfaceRow> {
    let base_source_refs = vec![
        M5_DEBUG_HIERARCHY_SCHEMA_REF.to_owned(),
        M5_DEBUG_HIERARCHY_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5DebugExportField::ALL.to_vec();

    vec![
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::TaskRunPane,
            owner_role: "Task-run debug guild".to_owned(),
            scope_summary:
                "Debug headers for launched task runs, holding live attached control over the target process"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::Launch, M5DebugSessionMode::Attach],
            control_postures: vec![M5DebugControlPosture::LiveAttachedControl],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::DebugAdapterUnavailable,
                M5ExecutionDowngradeTrigger::ConnectorLost,
            ],
            consumer_surfaces: vec!["task_pane".to_owned(), "debug_view".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(task_launch_live_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::TestRunPane,
            owner_role: "Test-run debug guild".to_owned(),
            scope_summary:
                "Debug headers for attached test runs stopped at a breakpoint, keeping thread state explicit"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::Attach],
            control_postures: vec![M5DebugControlPosture::LiveAttachedControl],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::ConnectorLost],
            consumer_surfaces: vec!["test_pane".to_owned(), "debug_view".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(test_attach_breakpoint_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::RequestRunPane,
            owner_role: "Request-execution debug guild".to_owned(),
            scope_summary:
                "Inspect-only views of imported request-execution snapshots with no live control"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::InspectOnly],
            control_postures: vec![M5DebugControlPosture::InspectOnlyView],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
                M5ExecutionDowngradeTrigger::DebugAdapterUnavailable,
            ],
            consumer_surfaces: vec!["request_pane".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(request_inspect_only_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::NotebookExecution,
            owner_role: "Notebook-execution debug guild".to_owned(),
            scope_summary:
                "Time-travel replay sessions for notebook kernels, captured analysis without live control"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::Replay],
            control_postures: vec![M5DebugControlPosture::CapturedAnalysis],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["notebook_pane".to_owned(), "debug_view".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(notebook_replay_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::AiMediatedExecution,
            owner_role: "AI-execution debug guild".to_owned(),
            scope_summary:
                "Debug headers for attached agent runtimes, keeping live control explicit even after a reconnect"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::Attach],
            control_postures: vec![M5DebugControlPosture::LiveAttachedControl],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::ConnectorLost,
                M5ExecutionDowngradeTrigger::DebugAdapterUnavailable,
            ],
            consumer_surfaces: vec!["ai_pane".to_owned(), "companion".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(ai_attach_restored_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::PublishFlow,
            owner_role: "Publish debug guild".to_owned(),
            scope_summary:
                "Crash analysis for publish runs over full core dumps, captured analysis with symbolicated frames"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::Core],
            control_postures: vec![M5DebugControlPosture::CapturedAnalysis],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["publish_pane".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(publish_core_symbolicated_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::PreviewFlow,
            owner_role: "Preview debug guild".to_owned(),
            scope_summary:
                "Crash analysis for preview renders over crash reports, disclosing partial symbolication"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::Core],
            control_postures: vec![M5DebugControlPosture::CapturedAnalysis],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
                M5ExecutionDowngradeTrigger::SymbolsUnavailable,
            ],
            consumer_surfaces: vec!["preview_pane".to_owned(), "debug_view".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(preview_core_partial_symbols_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::HistoryActivityCenter,
            owner_role: "History / activity-center debug guild".to_owned(),
            scope_summary:
                "Restored crash sessions from history over minidumps, keeping the hierarchy understandable when degraded"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::Core],
            control_postures: vec![M5DebugControlPosture::CapturedAnalysis],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
                M5ExecutionDowngradeTrigger::SymbolsUnavailable,
            ],
            consumer_surfaces: vec!["history".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(history_restored_unsymbolicated_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::SupportExportReplay,
            owner_role: "Support / diagnostics debug guild".to_owned(),
            scope_summary:
                "Offline replay of imported heap snapshots for support triage, disclosing unavailable symbols"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::Replay],
            control_postures: vec![M5DebugControlPosture::CapturedAnalysis],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::SymbolsUnavailable],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_sessions: vec![case(support_replay_symbols_unavailable_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
        M5DebugSurfaceRow {
            surface_family: M5RunAttemptSurfaceFamily::CompanionSummary,
            owner_role: "Companion-surface debug guild".to_owned(),
            scope_summary:
                "Inspect-only companion views of provider-reported debug snapshots with no live control"
                    .to_owned(),
            session_modes: vec![M5DebugSessionMode::InspectOnly],
            control_postures: vec![M5DebugControlPosture::InspectOnlyView],
            export_fields: all_export_fields,
            downgrade_triggers: vec![M5ExecutionDowngradeTrigger::CapturedEvidenceOnly],
            consumer_surfaces: vec!["companion".to_owned(), "activity_center".to_owned()],
            source_contract_refs: base_source_refs,
            example_sessions: vec![case(companion_inspect_only_input())],
            blurs_live_and_captured: false,
            flattens_hierarchy: false,
            drops_provenance: false,
            dump_implies_live_control: false,
        },
    ]
}

fn seeded_governance_review() -> M5DebugGovernanceReview {
    M5DebugGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        live_control_never_blurs_with_captured: true,
        hierarchy_never_flattened: true,
        provenance_and_symbolication_preserved: true,
        dump_cards_never_imply_live_control: true,
        support_export_reconstructs_debug_hierarchy: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5DebugConsumerProjection {
    M5DebugConsumerProjection {
        execution_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        tree_rows_read_single_hierarchy_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5DebugReleasePosture {
    M5DebugReleasePosture {
        release_packet_ref: M5_DEBUG_HIERARCHY_ARTIFACT_REF.to_owned(),
        debug_audit_ref: M5_DEBUG_HIERARCHY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 debug primitive packet. This is the one source of
/// truth shared by the tests, the fixture-emitting example, and the on-disk support export
/// so all three stay byte-aligned.
pub fn seeded_m5_debug_hierarchy_packet() -> M5DebugHierarchyPrimitivePacket {
    M5DebugHierarchyPrimitivePacket::new(M5DebugHierarchyPrimitivePacketInput {
        packet_id: "m5-debug-session-hierarchy-primitive:stable:0001".to_owned(),
        matrix_label: "M5 Debug-Session-Hierarchy Primitive".to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5DebugVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_DEBUG_HIERARCHY_SCHEMA_REF.to_owned(),
            M5_DEBUG_HIERARCHY_DOC_REF.to_owned(),
            M5_DEBUG_HIERARCHY_COMPONENT_MATRIX_REF.to_owned(),
            M5_DEBUG_HIERARCHY_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
