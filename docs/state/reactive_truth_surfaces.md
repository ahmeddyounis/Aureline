# Reactive-truth surfaces — shipped cues for the derived M5 views

The reactive-truth-surfaces cue layer is implemented in
[`crates/aureline-reactive-state/src/reactive_truth_surfaces/mod.rs`](../../crates/aureline-reactive-state/src/reactive_truth_surfaces/mod.rs)
and serialized to
[`artifacts/state/reactive_truth_surfaces.json`](../../artifacts/state/reactive_truth_surfaces.json).

Where the
[reactive-governance matrix](./m5_reactive_governance.md) freezes the
*rules* — which authority owns each surface, which states it can
present, and how the canonical engine narrows a presented claim — this
layer ships the *rendered truth* every derived surface actually shows.
For one surface and one observed subscription state it produces a single
reactive-truth cue, so a search panel, a graph surface, an AI inspector,
a review pane, a docs view, and a support summary describe the same
degraded state identically instead of each inventing local stale-state
prose.

## What the cue carries

For a surface and an observed subscription state, the cue answers, in one
shared grammar:

- **Where the truth came from** — the `authority_class`, the
  materialized-view class, the scope, and the `epoch_parity_group_id`
  the surface must stay level with.
- **How fresh and complete it is** — the observed freshness,
  completeness, and backpressure echoed straight from the subscription
  envelope.
- **What invalidation changed it** — the dominant `invalidation_reason`
  behind the current narrowed claim (e.g. a stale snapshot reports
  `upstream_input_stale`; a cache-served projection reports
  `cache_served`).
- **Whether it is keeping up** — a coalesced or snapshot-required cue
  when the stream lags the producer.
- **What the surface may now claim** — the canonically narrowed
  `truth_claim`. A derived surface never claims `exact_current_truth`;
  its ceiling is `consistent_snapshot`.
- **Whether dangerous derived actions stay live** — an `action_gate`
  plus a resubscribe-required flag.

## The action gate

Dangerous (mutating) derived actions narrow as the surface loses the
ability to prove a consistent snapshot:

| claim | action gate | dangerous action |
| --- | --- | --- |
| `consistent_snapshot` | `enabled` | live (revalidates against authority on apply) |
| `coalesced_stream`, `cached_projection` | `revalidate_before_act` | must revalidate first |
| `partial_projection`, `warming_no_truth_yet` | `narrowed_to_read_only` | narrowed to read-only |
| `stale_snapshot`, `replayed_snapshot`, `imported_snapshot`, `policy_limited_projection`, `provider_unavailable` | `blocked` | disabled until refresh / reconnect |

A terminal stream, snapshot-required backpressure, or an unavailable
scope additionally sets `resubscribe_required`, so the surface asks for a
fresh subscription instead of hiding behind a generic "updating"
spinner.

## Cross-channel parity

The same cue renders through every channel — shell truth strip,
CLI/headless line, activity-center row, keyboard-help line, accessibility
narration, diagnostics export, and support summary. Every channel carries
the same claim, gate, invalidation, and resubscribe tokens; only the
framing differs. Consumers:

- shell truth strips and action gating in
  [`crates/aureline-shell/src/reactive_truth_surfaces/mod.rs`](../../crates/aureline-shell/src/reactive_truth_surfaces/mod.rs);
- metadata-safe support export in
  [`crates/aureline-support/src/reactive_truth_surfaces/mod.rs`](../../crates/aureline-support/src/reactive_truth_surfaces/mod.rs);
- fixture replay in
  [`crates/aureline-reactive-state/tests/reactive_truth_surfaces.rs`](../../crates/aureline-reactive-state/tests/reactive_truth_surfaces.rs).

## No drift

The gate, dominant invalidation reason, and resubscribe cue are derived
from the canonical narrowing engine in
[`crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs`](../../crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs).
The audit packet projects every governed surface; fixtures pin the
rendered cue for representative observed states. Both are validated
against the [boundary schema](../../schemas/state/reactive_truth_surfaces.schema.json)
and replayed in the crate's test gate, so the cue layer can never fork
the engine.
