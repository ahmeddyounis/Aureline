# Dashboard & queue truth — beta contract

This document is the contract that ships behind the M3 beta operator
dashboards and queues. It is the shared truth that the service-health
dashboard, the review inbox, the incident queue, the support queue, and the
admin queue MUST all read, and that desktop shell, CLI / headless inspect,
diagnostics, and support exports MUST agree on for the same object.

## Why one freshness-and-order model

Every dashboard and queue answers the same operator question when something
looks calm:

1. Is this row actually current, or is it serving stale or cached data?
2. Why is it sorted where it is?
3. What is hidden from me by scope, and where is the evidence behind the
   displayed state?

Without a shared model, each surface invents its own status chip and sort copy,
and two failures follow:

- **Stale green.** A row keeps a confident all-clear headline long after the
  probe that justified it expired or the last-successful evidence aged out. The
  operator trusts a green that no longer has support behind it.
- **Unexplained order and silent narrowing.** A queue sorts rows with no stated
  reason and quietly drops rows outside the active scope, so the operator
  cannot tell whether the list is empty because nothing is wrong or because
  everything that is wrong is hidden.

The model at `crates/aureline-shell/src/dashboard_truth/model.rs` fixes this by
minting one `dashboard_truth_view_record` per surface. Each row is a
`dashboard_freshness_card_record`; queue surfaces additionally carry a
`queue_order_reason_record`.

## What a freshness card carries

| Field | Meaning |
| ----- | ------- |
| `card_id` | Stable object identity. Support exports and diagnostics quote this so a row shows up under one ref across surfaces. |
| `surface` | Closed vocabulary: `service_health`, `review_inbox`, `incident_queue`, `support_queue`, `admin_queue`. |
| `displayed_state` | Declared headline reported by the source: `clear`, `attention`, `blocked`. |
| `effective_state` | The honest, derived headline the chrome MUST render: `clear`, `unconfirmed`, `attention`, `blocked`. |
| `freshness` | How current the data is: `fresh`, `cached`, `stale`, `partial`, `policy_blocked`, `unavailable`. |
| `last_successful_evidence_at` / `evidence_age` | Timestamp and age bucket (`fresh`, `recent`, `stale`, `very_stale`, `never`) of the last-successful evidence. |
| `evidence_kind` / `evidence_ref` | The kind and canonical ref (`aureline://<class>/<id>`) of the durable object the open-details / inspect-evidence affordance routes to. |
| `downgrade_reasons` | Why a would-be-green row was withdrawn: `cached_fallback`, `freshness_expired`, `evidence_aged_out`, `source_partial`, `policy_blocked`, `source_offline`. |
| `green_downgraded` | True when `displayed_state` was `clear` but `effective_state` was withdrawn to `unconfirmed`. |
| `honesty_marker_present` | The single bit the chrome reads to light a yellow chip on the row. |

## The no-silent-green invariant

A card's `effective_state` is `clear` **only** when all three hold:

1. `displayed_state == clear`,
2. `freshness == fresh`, and
3. `evidence_age` is current (`fresh` or `recent`).

Any other combination on a would-be-green row downgrades it to `unconfirmed`,
sets `green_downgraded`, lights `honesty_marker_present`, and records the
precise `downgrade_reasons`. This is what keeps a seemingly healthy row from
masquerading as confirmed truth once freshness or evidence has expired — the
green is withdrawn and the row says why, never silently.

`attention` and `blocked` declared states always carry through to the effective
state, and still surface the freshness chip and any downgrade reasons.

The view rolls up `overall_effective_state` as the worst card state and
`overall_freshness` as the worst card freshness, and lights
`honesty_marker_present` when any card warns or any queue list is narrowed.

## What a queue-order record carries

For the four queue surfaces, the `queue_order_reason_record` answers "why this
order, and what is hidden?":

| Field | Meaning |
| ----- | ------- |
| `rows[]` | One entry per visible row, in render order, each with `order_rank` (1-based), an `order_reason`, a reviewable `order_explanation`, and a canonical `open_details_ref`. |
| `order_reason` | Closed vocabulary: `severity_descending`, `sla_deadline`, `oldest_unresolved_first`, `recently_updated`, `assigned_to_you`, `blocking_dependency`, `manual_pin`, `default_recency`. |
| `hidden_scope[]` | One counter per narrowing reason, each with `hidden_count`, a reviewable `narrowing_explanation`, a canonical `reveal_ref`, and an `incomplete_knowledge` flag. |
| `narrowing_reason` | Closed vocabulary: `scope_filter`, `policy_scope`, `assignee_filter`, `resolved_hidden`, `archived_hidden`, `severity_filter`, `offline_partial_list`. |
| `visible_row_count` / `hidden_total` / `total_in_scope_count` | The overview counters so the operator can see how much of the list is hidden. |
| `narrowing_present` / `incomplete_knowledge_present` | Whether the list is narrowed at all, and whether any hidden rows are unknown (policy scope or an incomplete offline list) rather than deliberately filtered. |

Every visible card on a queue surface MUST have exactly one ordering row; the
builder rejects a queue whose rows do not cover its cards.

## Canonical routing, never a landing page

Every `evidence_ref`, `open_details_ref`, and `reveal_ref` MUST be a canonical
durable object ref of the form `aureline://<class>/<id>`. The `<class>` segment
MUST be a specific object class; the builder rejects generic landing
destinations (`home`, `dashboard`, `landing`, `index`, `overview`, `start`,
`root`). When the operator chooses "open details", "inspect evidence", "resolve
follow-up", or "reveal hidden", the chrome routes to the durable object behind
the row, not to a dashboard home.

## Cross-surface agreement

The same `dashboard_truth_view_record` is read verbatim by the desktop shell,
the CLI / headless inspector, diagnostics, and support exports. The headless
emitter `aureline_shell_dashboard_truth_corpus` renders the JSON and the
plaintext block; support exports quote the plaintext. Because all surfaces read
one record, they cannot disagree on freshness, order reason, or hidden-scope
state for the same object.

## What never crosses this boundary

Raw endpoint URLs, hostnames, credentials, raw payloads, raw stack frames, raw
operator identity strings, and absolute paths never appear on these records.
Surfaces carry opaque object refs, stable tokens, and short reviewable
sentences only.

## Conformance corpus

The drill corpus at `crates/aureline-shell/src/dashboard_truth/corpus.rs` mints
one scenario per surface and downgrade/order/narrowing drill, pinned bit-for-bit
on disk under `fixtures/ops/m3/dashboard_and_queue_truth/`. The fixture-replay
test at `crates/aureline-shell/tests/dashboard_truth_fixtures.rs` enforces:

- the on-disk view matches the in-code projection bit-for-bit;
- no `effective_state == clear` card is stale or evidence-expired, and every
  declared-clear-but-not-current card is downgraded;
- every ref is a canonical durable object;
- queue presence matches the surface, and ordering rows cover every card; and
- the fixture directory carries exactly the corpus scenarios.

Regenerate the fixtures after any change:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_dashboard_truth_corpus -- emit-fixtures \
  fixtures/ops/m3/dashboard_and_queue_truth
```

## Scope

This contract is deliberately narrow: it adds freshness, evidence, order, and
hidden-scope truth to the already-claimed beta operator surfaces. It does not
introduce business analytics or vanity dashboards, and it does not redesign the
review, incident, support, or admin information architectures beyond the minimal
metadata needed for freshness and order explainability.
