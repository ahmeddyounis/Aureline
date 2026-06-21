# M5 Problem Records — source-task correlation and rerun/jump parity

- Packet: `m5-problem-records:stable:0001`
- Label: `M5 Problems records — source-task correlation and rerun/jump parity`
- As of: `2026-06-21T00:00:00Z`
- Rows: 11
- Effective: 4 actionable, 3 narrowed, 2 read-only imported, 1 raw-evidence-only, 1 labs

| Row | Origin | Parse | Claimed | Effective | Confidence | Jump | Output | Rerun |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| problem:local-structured-diagnostic:0001 | local_task | structured_language_diagnostic | actionable | actionable | structured_full | available | not_applicable | available |
| problem:local-test-normalized-event:0001 | local_test | normalized_task_event | actionable | actionable | structured_full | available | available | available |
| problem:local-heuristic-parse:0001 | local_task | heuristic_output_parse | actionable | actionable | heuristic_medium | available | available | available |
| problem:imported-provider-annotation:0001 | imported_provider_evidence | imported_provider_annotation | read_only_imported | read_only_imported | provider_mapped | available | available | read_only_inspect_only |
| problem:pipeline-provider-run:0001 | pipeline_provider_run | normalized_task_event | read_only_imported | read_only_imported | provider_mapped | available | available | read_only_inspect_only |
| problem:notebook-superseded:0001 | notebook_run | normalized_task_event | actionable | narrowed_actionable | structured_full | available | available | available |
| problem:headless-stale-run:0001 | headless_automation | heuristic_output_parse | actionable | narrowed_actionable | heuristic_low | available | available | available |
| problem:local-downgraded-mapping:0001 | local_task | heuristic_output_parse | actionable | narrowed_actionable | heuristic_low | available | available | available |
| problem:extension-gated-rerun:0001 | extension_owned_run | normalized_task_event | actionable | actionable | structured_full | available | available | gated_requires_authority |
| problem:local-lineage-lost-floored:0001 | local_task | heuristic_output_parse | actionable | raw_evidence_only | unmapped_requires_review | available | available | not_applicable |
| problem:labs-cross-run-correlation:0001 | ai_triggered_run | normalized_task_event | labs_not_claimed | labs_not_claimed | unmapped_requires_review | unavailable | not_applicable | not_applicable |

- Narrowed: `problem:notebook-superseded:0001` — Held at narrowed_actionable below the actionable row: superseded by newer run; the finding stays jumpable and inspectable until current evidence replaces it
- Narrowed: `problem:headless-stale-run:0001` — Held at narrowed_actionable below the actionable row: stale run; the finding stays jumpable and inspectable until current evidence replaces it
- Narrowed: `problem:local-downgraded-mapping:0001` — Held at narrowed_actionable below the actionable row: downgraded mapping; the finding stays jumpable and inspectable until current evidence replaces it
- Narrowed: `problem:local-lineage-lost-floored:0001` — Floored to raw_evidence_only below the actionable row: source ref missing; the raw-output backlink stays reopenable rather than rendering a clean-but-false actionable row
