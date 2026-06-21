# M5 Structured-vs-Heuristic Fallback Evidence Drills

- Packet: `m5-fallback-evidence-drills:stable:0001`
- Label: `M5 structured-native versus heuristic-fallback proof corpus across local, remote, notebook, extension, AI-tool, and provider channels`
- As of: `2026-06-21T00:00:00Z`
- Cases: 14
- Effective: 8 certified, 2 narrowed, 3 read-only overlay, 0 unreconstructable, 1 labs

| Case | Source | Drill | Channel | Origin | Claimed | Effective | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fallback:native-structured-language-problems:0001 | structured_language_diagnostic | native_structured | task_test_debug_output | local_test | fallback_certified | fallback_certified | structured_full |
| fallback:normalized-task-event-terminal:0001 | normalized_task_event | normalized_task_event | task_test_debug_output | local_task | fallback_certified | fallback_certified | structured_full |
| fallback:heuristic-parse-terminal:0001 | heuristic_output_parse | heuristic_text_parse | task_test_debug_output | local_task | fallback_certified | fallback_certified | heuristic_high |
| fallback:malformed-output-heuristic:0001 | heuristic_output_parse | malformed_output | task_test_debug_output | local_task | fallback_certified | fallback_certified | heuristic_medium |
| fallback:imported-provider-annotation:0001 | imported_provider_annotation | imported_evidence | remote_provider_imported_output | imported_provider_evidence | fallback_read_only_overlay | fallback_read_only_overlay | provider_mapped |
| fallback:pipeline-reconnect:0001 | normalized_task_event | reconnect | remote_provider_imported_output | pipeline_provider_run | fallback_read_only_overlay | fallback_read_only_overlay | provider_mapped |
| fallback:notebook-heuristic-stale:0001 | heuristic_output_parse | stale_run | task_test_debug_output | notebook_run | fallback_certified | fallback_narrowed | heuristic_high |
| fallback:superseded-retry-marked:0001 | normalized_task_event | superseded_retry | task_test_debug_output | local_test | fallback_certified | fallback_certified | structured_full |
| fallback:extension-ai-tool-heuristic:0001 | heuristic_output_parse | heuristic_text_parse | extension_ai_tool_output | extension_owned_run | fallback_certified | fallback_certified | heuristic_medium |
| fallback:channel-virtualization-large-log:0001 | structured_language_diagnostic | channel_virtualization | task_test_debug_output | local_task | fallback_certified | fallback_certified | structured_full |
| fallback:partial-export-support-bundle:0001 | structured_language_diagnostic | partial_export | evidence_bundle | local_test | fallback_certified | fallback_certified | structured_full |
| fallback:heuristic-stale-proof:0001 | heuristic_output_parse | heuristic_text_parse | task_test_debug_output | local_task | fallback_certified | fallback_narrowed | heuristic_high |
| fallback:lost-channel-remote:0001 | normalized_task_event | lost_channel | remote_provider_imported_output | remote_linked_run | fallback_read_only_overlay | fallback_read_only_overlay | provider_mapped |
| fallback:labs-heuristic-notebook:0001 | heuristic_output_parse | heuristic_text_parse | task_test_debug_output | notebook_run | fallback_labs_not_claimed | fallback_labs_not_claimed | heuristic_medium |

- Narrowed: `fallback:notebook-heuristic-stale:0001` — Held at fallback_narrowed below the fallback_certified claim: evidence stale; lineage stays reopenable via the raw output backlink until re-verified
- Narrowed: `fallback:heuristic-stale-proof:0001` — Held at fallback_narrowed below the fallback_certified claim: verification proof stale; lineage stays reopenable via the raw output backlink until re-verified
