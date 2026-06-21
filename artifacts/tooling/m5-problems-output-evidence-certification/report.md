# M5 Problems / Output / Execution-Evidence Qualification

- Packet: `m5-problems-output-evidence-certification:stable:0001`
- Label: `M5 Problems, output-channel, and execution-evidence qualification with automatic claim-narrowing on stale or failing causal-link and confidence proof`
- As of: `2026-06-21T00:00:00Z`
- Profiles: 8 (8 claimed, 1 overlay, 1 narrowed)
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-21T00:00:00Z, auto-narrow on stale: true)

## Profiles

| Profile | Origin | Claimed | Effective | Confidence | Freshness |
| --- | --- | --- | --- | --- | --- |
| problems_panel | local_task | qualified | qualified | structured_full | live |
| output_channel | local_task | qualified | qualified | structured_full | live |
| terminal_runner | local_task | qualified | qualified | heuristic_high | live |
| debug_console | local_debug_session | qualified | qualified | structured_full | live |
| notebook_output | notebook_run | qualified | retest_pending | structured_full | stale_expired |
| pipeline_overlay | pipeline_provider_run | limited | limited | provider_mapped | cached_within_window |
| ai_tool_evidence | ai_triggered_run | qualified | qualified | structured_full | live |
| support_export | headless_automation | qualified | qualified | structured_full | live |

## Release-evidence rows

| Axis | Dimension | Holding | Worst grade |
| --- | --- | --- | --- |
| causal_link_integrity | causal_link_integrity | 8/8 | retest_pending |
| confidence_honesty | confidence_honesty | 8/8 | retest_pending |
| stale_superseded_handling | stale_superseded_handling | 7/8 | retest_pending |
| reopenable_evidence_parity | reopen_to_origin_parity | 8/8 | retest_pending |

## Narrowed profiles

- `notebook_output`: claim `qualified` -> effective `retest_pending` — Held at retest_pending below the qualified claim: stale/superseded handling proof aged out; reopen-to-origin stays available until re-verified
