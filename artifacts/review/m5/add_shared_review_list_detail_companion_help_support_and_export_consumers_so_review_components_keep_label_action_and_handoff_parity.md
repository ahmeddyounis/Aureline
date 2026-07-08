# Shared Review-Component Consumers: Label, Action, and Handoff Parity

- Packet: `review-component-consumer:stable:0001`
- Surface: `Shared review-component consumers`
- Consumer bindings: 16 (9 narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Consumer bindings

- **PR #4821** [`bind:rr-4821:list`]: component `review_request_row` on `desktop_list`, mode `full_parity`
- **PR #4821** [`bind:rr-4821:detail`]: component `review_request_row` on `detail_pane`, mode `full_parity`
- **PR #4821** [`bind:rr-4821:export`]: component `review_request_row` on `exported_evidence`, mode `full_parity`
- **Run #3310** [`bind:cs-3310:list`]: component `checks_summary_card` on `desktop_list`, mode `freshness_narrowed`
- **Run #3310** [`bind:cs-3310:detail`]: component `checks_summary_card` on `detail_pane`, mode `freshness_narrowed`
- **Review #771** [`bind:pt-771:companion`]: component `pending_review_tray` on `companion_triage`, mode `local_continue_fallback`
- **Review #771** [`bind:pt-771:detail`]: component `pending_review_tray` on `detail_pane`, mode `local_continue_fallback`
- **Candidate #559** [`bind:mr-559:detail`]: component `merge_readiness_panel` on `detail_pane`, mode `full_parity`
- **Candidate #559** [`bind:mr-559:support`]: component `merge_readiness_panel` on `support_export`, mode `full_parity`
- **Queue entry #88** [`bind:mq-88:list`]: component `merge_queue_entry` on `desktop_list`, mode `freshness_narrowed`
- **Queue entry #88** [`bind:mq-88:companion`]: component `merge_queue_entry` on `companion_triage`, mode `freshness_narrowed`
- **Stack #12** [`bind:sd-12:detail`]: component `stack_dependency_chip` on `detail_pane`, mode `full_parity`
- **Stack #12** [`bind:sd-12:help`]: component `stack_dependency_chip` on `help_surface`, mode `full_parity`
- **PR #90** [`bind:ai-90:list`]: component `approval_invalidation_banner` on `desktop_list`, mode `handoff_required`
- **PR #90** [`bind:ai-90:detail`]: component `approval_invalidation_banner` on `detail_pane`, mode `handoff_required`
- **PR #90** [`bind:ai-90:support`]: component `approval_invalidation_banner` on `support_export`, mode `handoff_required`
