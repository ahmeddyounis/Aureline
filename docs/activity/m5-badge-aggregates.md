# Badge aggregates

This document describes the working engine that turns a corpus of **durable attention items**
into **deduped, per-scope badge counts**, coalesces **repeated failures** from one root cause
into a single durable object, projects **one shared count** across every badge-bearing
surface, and emits **stable telemetry enums**. It makes a badge number *governed, deduped
truth* rather than a surface-local accumulation of raw events.

Where the [attention-routing matrix](./m5-attention-routing.md) *names and freezes the object
model* — including the `BadgeAggregate` object and its required `scope_key` / `count` /
`count_class` / `freshness` / `deduped_objects` / `muted_reasons` / `suppressed_reasons` /
`active_quiet_hours_modes` fields — and the [fanout-receipts lane](./m5-fanout-receipts.md)
*records per-destination delivery truth*, this lane decides **what number a badge shows, why,
and where opening it lands**.

The track invariant this lane protects: **badges derive from deduped durable items.** A badge
count cannot be trusted until it is modeled as the count of distinct durable objects a user
can open — never a tally of duplicate raw events, and never inflated by repeated failures or
muted, suppressed, or deferred noise.

If this document, the companion schema, and the worked fixture disagree, the normative
sources in `.t2/docs/` win and this document plus its companions update in the same change.

## Counts derive from deduped durable items

[`aggregate_badges`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) is a pure
function of a slice of
[`DurableAttentionItem`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) — the
badge-relevant projection of a durable activity object or notification envelope. It returns
one
[`BadgeAggregate`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) per scope
that has items, so the same corpus yields the same counts byte-for-byte in support export and
CLI / headless diagnostics.

A scope's badge `count` equals the number of **distinct durable objects** pending attention
after dedupe: counted items sharing a `dedupe_key` collapse to one
[`BadgeObjectRef`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs). The
aggregate also reports `raw_event_count`, so the dedupe ratio is visible — a badge that shows
`3` from `5` raw events, or `10` from `11`, proves repeats are **deduped, not added**.

## What counts, and what is excluded but named

The engine derives a
[`BadgeContributionClass`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) for
each item; it never trusts a pre-counted total.

| Contribution | Counts? | Meaning |
| --- | --- | --- |
| `counted` | yes | A durable item in an active, pending state, not muted/suppressed/deferred |
| `muted` | no | Muted by the user or a focus/scope rule |
| `suppressed` | no | Suppressed by policy or rate-limiting |
| `quiet_hours_deferred` | no | Deferred by an active quiet-hours mode |
| `settled` | no | Already acknowledged, resolved, dismissed, archived, or completed |

An excluded item never increments the count, yet the aggregate names its reason in
`muted_reasons`, `suppressed_reasons`, or `active_quiet_hours_modes` — so the number is
**auditable**, not a silent drop. A zero-count badge still explains why eligible items were
not counted.

**A security advisory is never silenced.** An active-state `security_advisory` item is always
`counted`, regardless of any mute, suppression, or quiet-hours signal it carries.

## The count class

Every count is also classified into a coarse
[`CountClass`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) so governance and
telemetry reason about magnitude without echoing an exact, spammy number.

| Count class | Count | Display |
| --- | --- | --- |
| `none` | 0 | `0` |
| `single` | 1 | `1` |
| `few` | 2–9 | the number |
| `many` | 10–98 | the number |
| `saturated` | 99+ | `99+` |

## One count across every surface

[`surface_badges`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) projects each
scope aggregate onto the five governed badge-bearing surfaces. The shell activity center is
the **authoritative durable record**; the dock/taskbar badge, browser and mobile companions,
and operator dashboard project the *same* aggregate.

| Surface | Base redaction | Authoritative |
| --- | --- | --- |
| `in_app_activity_center` | `summary_only` | yes |
| `dock_taskbar_badge` | `count_only` | no |
| `browser_companion` | `summary_only` | no |
| `mobile_companion` | `summary_only` | no |
| `operator_dashboard` | `summary_only` | no |

Every surface shows the **same count and count class** for a scope; only the applied redaction
differs, and it is layered as the stronger of the surface base and the aggregate **privacy
floor** (`summary_safe → summary_only`, `workspace_sensitive` / `security_critical →
redacted_payload`, `managed_sensitive → count_only`). The dock/taskbar badge is always
`count_only`. No surface ever widens privacy below the floor.

## Repeated failures coalesce

[`coalesce_failures`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) groups
failures sharing one `root_cause_key` into one
[`CoalescedFailure`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) per cause.
Instead of spamming the OS notification, the badge, both companions, and the operator
dashboard with one alert per failure, the failure rises **once**: `occurrence_count` records
how many raw failures collapsed, and the object reopens the representative item's **exact
authoritative object**. The same root cause is counted exactly once in the scope badge, so
repeated failures never inflate the number.

## Reopen to the authoritative object

Every badge reopens via the activity row (`activity_job_row`) — anchored on the **exact
object** when the count is one, or the **scope's pending list** when the count is more than
one — never an ambiguous generic shell. A coalesced failure reopens its representative item's
exact authoritative anchor.

## Telemetry — stable enums and counts, no message text

[`badge_telemetry`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) rolls the
corpus up into a
[`BadgeTelemetry`](../../crates/aureline-activity/src/m5_badge_aggregates/mod.rs) packet that
records the notification class (producing subsystem), the route (badge-bearing surface), and
the outcome by **stable token plus a count only**. `captures_message_text` is always `false`:
no message body, payload, or secret-bearing field is captured. The outcome rows reconcile to
the total item count, and every route reports the same total badge count — parity at the
telemetry level.

| Outcome | Meaning |
| --- | --- |
| `counted_in_badge` | A distinct durable object counted in the badge |
| `deduped_repeat` | A repeated raw event coalesced into an already-counted object |
| `muted` | Muted out of the count |
| `suppressed` | Suppressed out of the count |
| `quiet_hours_deferred` | Deferred out of the count by quiet hours |
| `settled` | Already settled, so not counted |

## Matrix binding and export safety

Every scope, privacy class, redaction class, reopen target, severity, dedupe scheme,
suppression reason, and quiet-hours mode the bundle uses is one the
[attention-routing matrix](./m5-attention-routing.md) defines, and the `BadgeAggregate` object
can show the badge states (`shown`, `acknowledged`, `dismissed`, `suppressed`,
`quiet_hours_deferred`, `unknown_requires_review`). The record carries no message bodies,
credentials, raw provider payloads, hostnames, device identifiers, or absolute paths — only
opaque object refs, stable tokens, short reviewable sentences, and counts — so it is safe to
embed in a support export verbatim.

## Worked fixture and freeze gate

The canonical bundle is checked in at
[`/fixtures/activity/m5-badge-aggregates/canonical_bundle.json`](../../fixtures/activity/m5-badge-aggregates/canonical_bundle.json),
with an evidence companion at
[`/artifacts/activity/m5-badge-aggregates.md`](../../artifacts/activity/m5-badge-aggregates.md)
and a boundary schema at
[`/schemas/activity/m5-badge-aggregates.schema.json`](../../schemas/activity/m5-badge-aggregates.schema.json).
The freeze gate
[`crates/aureline-activity/tests/m5_badge_aggregates.rs`](../../crates/aureline-activity/tests/m5_badge_aggregates.rs)
rebuilds the bundle in code and asserts it equals the fixture byte-for-byte; an inconsistent
edit flips an invariant or fails the round-trip. Regenerate the fixture with:

```sh
cargo run -p aureline-activity --example dump_m5_badge_aggregates > \
  fixtures/activity/m5-badge-aggregates/canonical_bundle.json
```
