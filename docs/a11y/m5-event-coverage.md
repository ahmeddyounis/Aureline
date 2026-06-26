# M5 Event-Class Non-Visual Coverage

This document is the contract for the M5 event-class coverage catalog that maps the
concrete dynamic events of each high-churn workflow to the assistive channel. Where
the [live-announcement grammar](./m5-announcement-grammar.md) governs *how* a dynamic
event is narrated (its message template, channel, coalescing budget, and durable
fallback) and the frozen dynamic-surface matrix governs *which* controlled
vocabularies an accessibility object may carry, this catalog supplies *which concrete
events* a professional user must be able to follow non-visually — diagnostics,
completion/snippet/session changes, run/debug/test transitions, terminal command
boundaries, collaboration control/recording changes, AI patch/review milestones, and
stale/degraded-truth transitions.

- Record kind: `m5_event_class_coverage_catalog`
- Schema: [`schemas/a11y/m5-event-coverage.schema.json`](../../schemas/a11y/m5-event-coverage.schema.json)
- Canonical support export: [`artifacts/a11y/m5-event-coverage-proof/support_export.json`](../../artifacts/a11y/m5-event-coverage-proof/support_export.json)
- Governance summary artifact: [`artifacts/a11y/m5-event-coverage-proof/event-coverage-proof.md`](../../artifacts/a11y/m5-event-coverage-proof/event-coverage-proof.md)
- Fixtures: [`fixtures/a11y/m5-event-coverage/`](../../fixtures/a11y/m5-event-coverage/)
- Producer: `aureline_shell::accessibility::events::current_stable_m5_event_coverage_export`
- Headless emitter: `aureline_shell_m5_event_coverage`
- Announcement grammar this lane routes through: [`schemas/a11y/m5-announcement-grammar.schema.json`](../../schemas/a11y/m5-announcement-grammar.schema.json)

## Why this catalog exists

Static screen-reader labels are not enough on dynamic IDE surfaces. A professional
follows work by its *state transitions*: a diagnostic appears, a completion list
opens, a test run starts and finishes, a terminal command exits, a collaborator's
role changes, an AI patch is proposed, a panel's data goes stale. Before this catalog,
which of those transitions actually reached an assistive user — and with what identity
and reason — was implicit per surface. This catalog makes event coverage a single
governed packet: one family per workflow, each enumerating the meaning-changing events
that must narrate, every event bound to a grammar class, a concise-identity message
id, a blocked/degraded-reason disclosure, and a reopenable durable fallback. The same
mappings are reused by support exports, docs/help, and assistive-tech conformance
packets, so a dynamic-narration regression is debuggable from the export alone.

## Governed event families

The catalog carries one coverage row for each governed event family:

| Family | Row | Producers | Events |
| --- | --- | --- | --- |
| `diagnostics` | `event-family:diagnostics` | editor, notebook | published, cleared, blocking-error |
| `completion_and_session` | `event-family:completion-and-session` | editor | list-opened, snippet-session, assist-unavailable |
| `run_debug_test` | `event-family:run-debug-test` | debug, notebook | started, completed, debug-paused, blocked |
| `terminal_boundary` | `event-family:terminal-boundary` | terminal | command-started, command-exited, boundary-unavailable |
| `collaboration_control` | `event-family:collaboration-control` | collab | role-changed, recording-changed, control-restricted |
| `ai_patch_review` | `event-family:ai-patch-review` | ai, review | generation-started, patch-proposed, milestone-reached, generation-blocked |
| `stale_degraded_truth` | `event-family:stale-degraded-truth` | shell | went-stale, bridge-degraded, refreshed |

## What each event binds

Each `dynamic_event` inside a family binds a stable `event_id` to:

- **A concise-identity message id** — `identity_message_id` (prefixed `event.`) is the
  stable handle for the meaning the announcement and the durable fallback both carry,
  so what the user heard can be reconstructed later.
- **An announcement grammar class** — `announcement_event_class` is one of the six
  governed grammar classes. The live-region channel is *derived* from it: only an event
  that narrates through `blocker_raised` interrupts assertively; every other event
  stays polite. Events route through the one governed grammar rather than per-surface
  prose.
- **A blocked/degraded-reason disclosure** — `degraded_disclosure` pairs an
  `announces_reason` flag with a `reason_class` (`not_applicable`, `blocked`,
  `degraded`, `stale`, `unavailable`, or `policy_restricted`). A `blocked` reason must
  narrate through `blocker_raised`; a `degraded`/`stale`/`unavailable`/
  `policy_restricted` reason must narrate through `degraded_or_stale_truth`; a normal
  (`not_applicable`) transition discloses no reason and never claims either reserved
  class. Every family carries at least one event that can announce a blocked/degraded
  reason.
- **A reopenable durable fallback** — `durable_fallback` names the surface
  (`activity_row`, `run_header`, `patch_review_header`, `banner_detail`,
  `selection_summary`, `notification_center_entry`, or `status_detail`) the user can
  reopen to recover the same event identity, never relying on ephemeral narration
  alone.
- **A meaning-changing guard** — `meaning_changing` is always `true`. Only
  meaning-changing dynamic events belong in the assistive channel; a producer that can
  emit a low-value repaint tick must not seed it here.

## Controlled vocabulary reuse

The event-class and durable-fallback-surface tokens are reused verbatim from the
announcement grammar through the `announcement_vocabulary_set` block, which must match
the grammar's canonical token lists. The shared state vocabularies
(`announcement_politeness`, `coalescing_strategy`, `fallback_durability`, …) are reused
from the frozen dynamic-surface matrix through the `shared_vocabulary_set` block. The
coverage-shaped vocabularies this lane adds — `event_family`, `event_producer`, and
`reason_class` — are frozen in the `coverage_vocabulary_set` block. No surface mints a
parallel synonym for a governed event family or reason.

## Auto-narrowing on degraded bridge or stale proof

A family whose assistive-tech proof has gone stale narrows its qualification (for
example Stable to Beta) while keeping its events, identities, durable fallbacks, and
`proof_stale` downgrade trigger intact. A family whose OS accessibility bridge is
unavailable narrows (for example Stable to Preview) and drops its
`non_visual_fidelity` to `degraded_accessible`, while keeping its events and its
`bridge_unavailable` trigger — its boundary-unavailable event still narrates the
unavailable reason rather than disappearing. The `proof_stale_narrowed.json` and
`bridge_unavailable_narrowed.json` fixtures exercise both paths: the narrowing is
always a disclosed claim change, never a hidden family.

## Consumers

`editor` routes diagnostics and assist events; `terminal` routes command boundaries;
`debug` and notebook route run/test transitions; `review` and AI surfaces route
patch/review milestones; `collab` routes control/recording changes; and `shell` routes
cross-surface stale/degraded truth. Support exports, docs/help, and assistive-tech
conformance packets reuse the same coverage. The `consumer_projection` block records
that every one of those consumers routes through the coverage catalog rather than
improvising per-surface narration.

## Regenerating the catalog

The seed builders in `aureline_shell::accessibility::events` are the single producer of
the checked-in support export and fixtures. Regenerate with the headless emitter:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- support-export \
  > artifacts/a11y/m5-event-coverage-proof/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- markdown \
  > artifacts/a11y/m5-event-coverage-proof/event-coverage-proof.md
cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- fixture-proof-stale-narrowed \
  > fixtures/a11y/m5-event-coverage/proof_stale_narrowed.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_event_coverage -- fixture-bridge-unavailable-narrowed \
  > fixtures/a11y/m5-event-coverage/bridge_unavailable_narrowed.json
```

The `checked_support_export_matches_seed` test fails if the checked-in export drifts
from the seed builder, so the artifact, the fixtures, and the in-code coverage stay in
lockstep.
