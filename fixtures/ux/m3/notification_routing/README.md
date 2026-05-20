# Attention-routing (beta) fixture corpus

Reviewable fixtures for the beta attention router that lives in
[`crates/aureline-shell/src/attention_router/`](../../../../crates/aureline-shell/src/attention_router/).

Each JSON file is a literal projection of the seeded
`AttentionRoutingCorpus` produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_attention_router.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_attention_router.rs)).
The inspector is the only mint-from-truth path for these fixtures, so the
checked-in JSON cannot drift from the Rust types. Every routed outcome is a
`notification_route_outcome_record` that conforms to the boundary schema at
[`schemas/ux/notification_route_outcome.schema.json`](../../../../schemas/ux/notification_route_outcome.schema.json),
which is itself a governed projection over the
[`notification_envelope`](../../../../schemas/ux/notification_envelope.schema.json)
and [`fanout_receipt`](../../../../schemas/ux/fanout_receipt.schema.json)
contracts.

All records carry the shared contract ref `shell:attention_router_beta:v1`
so shell UI rows, the headless CLI rows, and the support-export rows pivot to
the same `case_id` and `route_outcome_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`corpus.json`](./corpus.json) | Full corpus: aggregate coverage summary plus one routing case per scenario (foreground-focused in-app, background OS delivery, locked lock-screen summary, quiet-hours companion held, admin-suppressed security, dedupe burst repeat, companion-available fanout, companion policy blocked, screen-reader navigable, and placeholder reopen). |
| [`cases.json`](./cases.json) | The case vector embedded in the corpus, broken out for row-level review. |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes each case's route outcome through support-safe enums and per-surface resolution rows. Raw user-facing message text is excluded by construction. |

## What each case proves

Every case routes one `notification_envelope_record` through the
`AttentionRouter` under one live `ChannelContext` and captures the single
`NotificationRouteOutcome`. Across the corpus the fixtures prove:

- **One alert, every surface.** The same envelope resolves consistently
  across `durable_job_row`, `status_item`, `activity_center_digest_card`,
  `toast`, `contextual_banner`, `os_notification`, `lock_screen_summary`,
  and `companion_push`.
- **Live-channel narrowing, never widening.** A focused foreground window
  drops the redundant OS notification (`suppressed_foreground_redundant`); an
  unlocked device drops the lock-screen summary
  (`lock_screen_not_applicable`); an unreachable companion drops the push
  (`companion_unavailable`); a managed policy blocks the push
  (`companion_policy_blocked`). No held / suppressed / deduped surface is
  ever upgraded back to delivered.
- **Exact reopen, no generic home.** Every resolved surface carries the
  outcome's single `reopen_target_ref`. The placeholder case proves a
  truthful placeholder reopen rather than a generic home view.
- **Durable truth survives the hold.** Quiet hours and admin suppression
  delay interruption while the durable surface still delivers (or coalesces
  into durable truth a prior emission delivered).
- **Privacy-safe handoff.** OS and companion fanout is summary-first, with
  durable reopen required for privileged detail and a fixed list of forbidden
  shortcut classes the summary refuses to complete.

## Fixture rules

- The fixtures are regenerated only by the headless inspector:

  ```sh
  cargo run -q -p aureline-shell --bin aureline_shell_attention_router -- corpus > fixtures/ux/m3/notification_routing/corpus.json
  cargo run -q -p aureline-shell --bin aureline_shell_attention_router -- cases > fixtures/ux/m3/notification_routing/cases.json
  cargo run -q -p aureline-shell --bin aureline_shell_attention_router -- support-export > fixtures/ux/m3/notification_routing/support_export.json
  ```

- The replay test
  [`crates/aureline-shell/tests/attention_router_fixtures.rs`](../../../../crates/aureline-shell/tests/attention_router_fixtures.rs)
  fails if the JSON drifts from the seeded corpus, if any outcome loses its
  reopen target or durable truth, if the surface / resolution-class coverage
  shrinks, or if the support export leaks summary copy.
