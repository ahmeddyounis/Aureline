# M5 Execution-Lifecycle Component Matrix

- Packet: `m5-execution-lifecycle-component-matrix:stable:0001`
- Label: `M5 Execution-Lifecycle Component Matrix`
- Components: 12 across 7 / 7 families (5 degraded)

## Components

- **component:run-attempt-header:0001** (run_attempt_header) — Run/attempt header for a live task run
  - A run/attempt header keeps run and attempt identity distinct and shows an actively running attempt as live truth
  - family=run_attempt_header truth=live locality=local export_safe=true assistive=true
- **component:run-attempt-header:0002** (run_attempt_header) — Run/attempt header for a superseded run with stale output
  - A run/attempt header discloses stale output as captured evidence rather than let it read as a current live result
  - family=run_attempt_header truth=captured locality=remote export_safe=true assistive=true
  - Degraded: trigger=captured_evidence_only — The source changed since this run; its output is marked stale and shown as captured evidence rather than a live result
- **component:input-request-prompt:0001** (input_request_prompt) — Input-request prompt awaiting an approval with a cancel-on-timeout
  - An input-request prompt discloses that it needs approval and that a timeout will cancel the run, with a visible deadline
  - family=input_request_prompt truth=live locality=local export_safe=true assistive=true
- **component:artifact-publish-row:0001** (artifact_publish_row) — Artifact-publish row for a durably retained build artifact
  - An artifact-publish row names the run that produced the artifact and discloses durable retention
  - family=artifact_publish_row truth=captured locality=container export_safe=true assistive=true
- **component:artifact-publish-row:0002** (artifact_publish_row) — Artifact-publish row for an evicted-but-recoverable artifact
  - An artifact-publish row discloses that the artifact was evicted but keeps producing-run lineage so it can be rebuilt
  - family=artifact_publish_row truth=captured locality=local export_safe=true assistive=true
  - Degraded: trigger=artifact_retention_expired — The artifact was evicted from cache; its producing run is still known and the artifact is rebuildable from lineage
- **component:rerun-comparison-sheet:0001** (rerun_comparison_sheet) — Rerun comparison sheet for an exact replay
  - A rerun comparison sheet confirms an exact replay of the baseline run's selection, environment, and inputs
  - family=rerun_comparison_sheet truth=planned locality=local export_safe=true assistive=true
- **component:rerun-comparison-sheet:0002** (rerun_comparison_sheet) — Rerun comparison sheet for a current-context rerun
  - A rerun comparison sheet shows the exact-versus-current-context difference before dispatch rather than after
  - family=rerun_comparison_sheet truth=planned locality=remote export_safe=true assistive=true
  - Degraded: trigger=rerun_context_drift — This rerun uses the current context, which differs from the baseline; the changed selection and environment are shown before dispatch
- **component:debug-session-header:0001** (debug_session_header) — Debug session header for a live attach session
  - A debug session header names an attach session with live control on the local machine and keeps the boundary explicit
  - family=debug_session_header truth=live locality=local export_safe=true assistive=true
- **component:debug-session-header:0002** (debug_session_header) — Debug session header for a captured replay session
  - A debug session header discloses a replay session as captured evidence, never as live control
  - family=debug_session_header truth=captured locality=managed export_safe=true assistive=true
  - Degraded: trigger=captured_evidence_only — This is a recorded replay in a managed environment; stepping navigates captured evidence and never controls a live process
- **component:thread-process-tree:0001** (thread_process_tree) — Thread/process tree for a live containerized run
  - A thread/process tree marks itself as a live container hierarchy and names its execution boundary explicitly
  - family=thread_process_tree truth=live locality=container export_safe=true assistive=true
- **component:dump-crash-artifact-card:0001** (dump_crash_artifact_card) — Dump/crash artifact card for a symbolicated crash dump
  - A dump/crash artifact card names the run that produced the dump, shows it fully symbolicated, and marks it captured evidence
  - family=dump_crash_artifact_card truth=captured locality=remote export_safe=true assistive=true
- **component:dump-crash-artifact-card:0002** (dump_crash_artifact_card) — Dump/crash artifact card for a dump with unavailable symbols
  - A dump/crash artifact card discloses that symbols are unavailable rather than present raw addresses as a resolved stack
  - family=dump_crash_artifact_card truth=captured locality=container export_safe=true assistive=true
  - Degraded: trigger=symbols_unavailable — No symbols resolved for this dump; frames are shown as raw addresses and the card offers a symbol-upload route
