# M5 Output-Channel Virtualization, Trust, and Freshness

- Packet: `m5-output-channels:stable:0001`
- Label: `M5 output channels — stream-first virtualization, trust classes, pin/export, and stale/live truth`
- As of: `2026-06-21T00:00:00Z`
- Channels: 9
- Effective: 5 certified, 1 narrowed, 2 read-only overlay, 0 unreconstructable, 1 labs

| Channel | Payload | Trust | Origin | Claimed | Effective | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| channel:raw-log-local-test:0001 | raw_log_stream | raw | local_test | channel_certified | channel_certified | structured_full |
| channel:structured-report-local-test:0001 | structured_report | trusted_structured | local_test | channel_certified | channel_certified | structured_full |
| channel:html-bundle-local-task:0001 | html_report_bundle | untrusted_active | local_task | channel_certified | channel_certified | structured_full |
| channel:trace-profile-local-task:0001 | trace_profile_output | safe_preview | local_task | channel_certified | channel_certified | structured_full |
| channel:artifact-local-test:0001 | generated_artifact | safe_preview | local_test | channel_certified | channel_certified | structured_full |
| channel:raw-log-local-task:0001 | raw_log_stream | raw | local_task | channel_certified | channel_narrowed | structured_full |
| channel:raw-log-pipeline-provider:0001 | raw_log_stream | raw | pipeline_provider_run | channel_read_only_overlay | channel_read_only_overlay | provider_mapped |
| channel:structured-report-imported-provider:0001 | structured_report | trusted_structured | imported_provider_evidence | channel_read_only_overlay | channel_read_only_overlay | provider_mapped |
| channel:html-bundle-labs:0001 | html_report_bundle | untrusted_active | local_task | channel_labs_not_claimed | channel_labs_not_claimed | heuristic_medium |

- Narrowed: `channel:raw-log-local-task:0001` — Held at channel_narrowed below the channel_certified claim: verification proof stale; lineage stays reopenable via the output channel until re-verified
