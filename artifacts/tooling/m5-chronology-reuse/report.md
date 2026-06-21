# M5 Task/Problem/Output Chronology Reuse

- Packet: `m5-chronology-reuse:stable:0001`
- Label: `M5 task/problem/output chronology reuse — one durable run-lifecycle grammar across activity, history, issue, support, and AI evidence`
- As of: `2026-06-21T00:00:00Z`
- Entries: 9
- Effective: 6 reused, 1 narrowed, 1 read-only overlay, 0 unreconstructable, 1 labs

| Entry | Phase | Outcome | Origin | Claimed | Effective | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| chronology:run-started-local-task:0001 | run_started | in_progress | local_task | chronology_reused | chronology_reused | structured_full |
| chronology:run-progress-local-test:0001 | run_progress | in_progress | local_test | chronology_reused | chronology_reused | structured_full |
| chronology:run-retried-local-task:0001 | run_retried | retried | local_task | chronology_reused | chronology_reused | structured_full |
| chronology:run-cancelled-local-task:0001 | run_cancelled | cancelled | local_task | chronology_reused | chronology_reused | structured_full |
| chronology:run-failed-local-test:0001 | run_failed | failed | local_test | chronology_reused | chronology_reused | structured_full |
| chronology:run-completed-notebook:0001 | run_completed | succeeded | notebook_run | chronology_reused | chronology_reused | structured_full |
| chronology:run-failed-pipeline-provider:0001 | run_failed | failed | pipeline_provider_run | chronology_read_only_overlay | chronology_read_only_overlay | provider_mapped |
| chronology:run-completed-perf-local:0001 | run_completed | succeeded | local_task | chronology_reused | chronology_narrowed | heuristic_high |
| chronology:run-progress-labs:0001 | run_progress | in_progress | notebook_run | chronology_labs_not_claimed | chronology_labs_not_claimed | heuristic_medium |

- Narrowed: `chronology:run-completed-perf-local:0001` — Held at chronology_narrowed below the chronology_reused claim: verification proof stale; lineage stays reopenable via the generated artifact until re-verified
