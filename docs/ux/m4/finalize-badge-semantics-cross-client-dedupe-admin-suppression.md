# Badge aggregate: count-class semantics, cross-client dedupe, admin suppression, and persistent attention summaries — contract

This is the reviewer-facing companion for the stable lane that locks the
**badge aggregate** to Aureline's durable truth model: one governed record per
whole-shell snapshot that binds **typed count classes**, **one durable object
set** behind every badge surface, **cross-client / cross-window dedupe**,
**export-safe admin / quiet-hours suppression lineage**, a **`0`-means-none**
guarantee, and a **persistent, inspectable attention summary** — all to a public
claim ceiling and an automatic narrow-below-Stable verdict.

This lane sits one level above the per-class durable-attention lock
([`/docs/ux/m4/lock-notification-routing-durable-activity-center-truth-quiet.md`](./lock-notification-routing-durable-activity-center-truth-quiet.md)).
Where that lane mints one record per durable attention class, this lane
reconciles the **whole shell's badge state** from the same durable object set
the activity center reads, so the dock/taskbar badge, the title-bar badge, the
in-shell badge, and the companion badge can never drift from durable truth.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression/`](../../../fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression/)
- Schema:
  [`/schemas/ux/finalize-badge-semantics-cross-client-dedupe-admin-suppression.schema.json`](../../../schemas/ux/finalize-badge-semantics-cross-client-dedupe-admin-suppression.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression.md`](../../../artifacts/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression.md)
- Typed source: `aureline_shell::badge_aggregate_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_badge_aggregate_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/badge_aggregate_stable_fixtures.rs`

## Why one governed aggregate record

When each badge surface counts on its own, the badge stops being trustworthy. A
dock badge outpaces the durable model; the same underlying object delivered to
desktop, companion, and a native notification multiplies a badge threefold; an
admin policy or quiet-hours window silences an item and the badge silently drops
to zero with no way for support to explain *why* no alert fired. The result is a
badge whose `0` might mean "nothing is wrong" or might mean "a fanout path
failed" — and nobody can tell which.

This lane mints one governed `badge_aggregate_record` per shell snapshot. It does
**not** reinvent the dedupe core, the per-item badge reconciliation
(`aureline_shell::notifications::actions`), or the count classes: each record is
a genuine projection of the live attention stack
(`aureline_shell::notifications`, `aureline_shell::attention_router`), routed
through `aureline_shell::notification_envelope_corpus`. The record binds, for one
snapshot:

- **Typed count classes.** Every count is keyed by an `AggregateCountClass`, not
  by an arbitrary surface. The taxonomy keeps the required classes explicit where
  they are exposed: `pending_review_approval`, `failed_runs`,
  `queued_publish_later`, `provider_auth_attention`, `managed_advisories`, and
  `muted_informational_backlog`, plus `session_requests`, `durable_running`, and
  `completion_unread`.
- **One durable object set.** The activity-center, dock/taskbar, title-bar,
  in-shell, and companion projections are reconciled against the same deduped
  durable objects. The activity center is authoritative; no surface may inflate a
  class above its authoritative active count, and any shortfall must be an
  explained per-class disablement.
- **Cross-client / cross-window dedupe.** Raw appearances are collapsed by
  canonical object identity (`cross_client_canonical_event_id`). The same object
  reported from desktop, companion, a native notification, or a second window
  becomes one durable object and counts once per class. A repeat that disagreed
  on its count class is rejected outright — dedupe may never lose class integrity.
- **Export-safe suppression lineage.** Every admin-suppressed,
  quiet-hours-muted, or per-class-disabled badge difference carries a lineage
  entry that names the reason, the affected surfaces, and proves the durable
  object and its reopen target stayed reachable. Support and shiproom read this
  to explain a missing alert.
- **`0` means none.** An active badge count of zero for a class means there are
  no current durable objects of that class — counts are *derived* from durable
  object state, never from toast history or per-surface counters. Held items are
  tracked in a separate held count and explained by lineage.
- **A persistent, inspectable attention summary** that survives restart and is
  inspectable in-product.
- **A public claim ceiling** and **automatic narrowing**: a snapshot that cannot
  prove a pillar, or whose lowest badge surface marker is below Stable, narrows
  below Stable with a named reason instead of inheriting an adjacent green row.

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Demonstrates |
| --- | --- | --- | --- | --- |
| `nominal.json` | nominal | **stable** | stable | typed classes; cross-client + cross-window dedupe; a muted backlog item with a zero active badge, a held count, and lineage |
| `quiet_and_admin_suppression.json` | quiet hours + admin suppression | **stable** | stable | admin-suppressed advisory, quiet-hours-muted run, companion per-class disablement — all with export-safe lineage |
| `companion_preview_surface.json` | companion badge surface in preview | preview (narrowed) | preview | narrow-below-Stable by the lowest badge surface marker |
| `cross_client_inflation_drill.json` | cross-client inflation drill | beta (narrowed) | stable | the lane detects a companion surface that multiplies cross-client copies and narrows with `one_durable_set_not_proven` |

Coverage verdict: **2 Stable, 2 narrowed**. The narrowed rows each name a reason
(`surface_not_yet_stable`; `one_durable_set_not_proven`) and drop below the
launch cutline instead of inheriting an adjacent green row.

## How the pillars are derived (not asserted)

The builder *computes* every pillar from the durable object set, so a record can
never publish a claim wider than its proof:

- **Dedupe** collapses raw appearances by `object_ref`. Disagreement on count
  class or disposition is a hard error. `cross_client_collapsed = raw − deduped`.
- **Per-class aggregates** count Active objects into `active_count` and
  Held/Suppressed objects into `held_or_suppressed_count`; Cleared/Resolved
  objects count nowhere. `0` active is therefore exactly "no Active durable
  objects of that class".
- **Surface projections** are reconciled against the authoritative active counts.
  A surface that reports more than the authoritative count *inflates*; a surface
  that reports less without a recorded disablement *diverges*. Either drops
  `one_durable_set_holds`.
- **Suppression lineage** must cover every held object (object-scoped) and every
  per-class disablement (surface-class-scoped), and every entry must preserve the
  durable object and reopen target, or `suppression_lineage_export_safe` drops.
- **Summary persistence** requires every durable object to be preserved.

The **claim ceiling** is then checked against the derived pillars: a snapshot may
not assert a pillar it cannot prove (a build error), and any pillar that is false
adds a named narrowing reason and drops the claim below Stable.

## Surfaces that ingest this record

The activity center, dock/taskbar badge, title bar, status bar, companion
fanout, command palette, diagnostics packet, support export, and Help/About read
this record verbatim. The same aggregate is reachable, keyboard-first, from the
activity center, command palette, status bar, and a menu command, across normal,
high-contrast, and zoomed layouts, and stays available without an account or
managed services.

## Regenerating the fixtures

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_badge_aggregate_stable -- emit-fixtures \
  fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression
```

The replay gate
(`crates/aureline-shell/tests/badge_aggregate_stable_fixtures.rs`) fails if the
on-disk JSON drifts from the in-code projection.
