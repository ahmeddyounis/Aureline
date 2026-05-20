# dashboard & queue truth drill corpus

These fixtures back the M3 freshness-honesty and queue-explainability lane for
the operator-facing beta dashboards and queues. They are read by every surface
that displays dashboard or queue truth — desktop shell, CLI / headless inspect,
diagnostics, and support exports — so a single regression in the no-silent-green
downgrade rule, the order-reason vocabulary, the hidden-scope counters, or the
canonical-object routing fails the corpus instead of shipping silently.

Each fixture is a complete `dashboard_truth_view_record` validated against
`schemas/ops/dashboard_freshness_card.schema.json` (its embedded
`queue_order` is validated against
`schemas/ops/queue_order_reason.schema.json`). The fixtures are minted by the
`aureline_shell_dashboard_truth_corpus` emitter from the in-code corpus at
`crates/aureline-shell/src/dashboard_truth/corpus.rs`, and the fixture-replay
test at `crates/aureline-shell/tests/dashboard_truth_fixtures.rs` asserts the
disk content matches the in-code projection bit-for-bit.

Regenerate after any change to the corpus or model:

```sh
cargo run -q -p aureline-shell \
  --bin aureline_shell_dashboard_truth_corpus -- emit-fixtures \
  fixtures/ops/m3/dashboard_and_queue_truth
```

## Drills

| Scenario id | Surface | Overall state | Overall freshness | Hidden | What it proves |
| ----------- | ------- | ------------- | ----------------- | ------ | -------------- |
| `service_health_all_clear` | `service_health` | `clear` | `fresh` | 0 | A dashboard may render an all-clear headline with no honesty chip only while every card is fresh and evidence-current. |
| `service_health_stale_green` | `service_health` | `unconfirmed` | `stale` | 0 | A clear card whose probe expired and a clear card serving cached data both downgrade to `unconfirmed` and name why; a fresh card stays clear. |
| `service_health_partial_offline` | `service_health` | `blocked` | `unavailable` | 0 | Partial, offline (a green that downgrades), and policy-blocked cards each carry a distinct downgrade reason. |
| `review_inbox_ordered_narrowed` | `review_inbox` | `blocked` | `stale` | 21 | Blocking, assigned-to-you, and oldest-unresolved order reasons; reviews hidden by scope, assignee, and resolved filters; a review with expired check evidence downgrades from green. |
| `incident_queue_severity_sla` | `incident_queue` | `blocked` | `cached` | 5 | Severity-descending and SLA-deadline ordering; a monitoring incident serving cached data downgrades from green; incidents below the severity filter are disclosed. |
| `support_queue_policy_scoped` | `support_queue` | `blocked` | `policy_blocked` | 9 | Blocking-dependency, SLA, and default order; a policy-blocked case; cases hidden by policy scope are disclosed as unknown (incomplete knowledge), not silently dropped. |
| `admin_queue_offline_partial` | `admin_queue` | `unconfirmed` | `partial` | 27 | A manual pin and recency order; an audit follow-up loaded only partially offline downgrades from green; an incomplete offline list and archived items are disclosed. |

## Invariants the replay test enforces

- **No silent green.** A card is `effective_state == clear` only when its
  declared state is `clear`, its freshness is `fresh`, and its evidence age is
  current (`fresh` or `recent`). Any declared-clear card that is stale, cached,
  partial, policy-blocked, offline, or evidence-expired downgrades to
  `unconfirmed`, sets `green_downgraded`, and lights the honesty marker.
- **Always an inspectable path.** Every card's `evidence_ref`, every queue
  row's `open_details_ref`, and every hidden bucket's `reveal_ref` is a
  canonical durable object ref (`aureline://<class>/<id>`), never a generic
  landing page.
- **Order and narrowing are explainable.** Each queue surface carries one order
  reason per visible row (rank `1..=N`) and one hidden-scope counter per
  narrowing reason; the service-health dashboard carries no queue-order record.
- **Cross-surface agreement.** Desktop, CLI / headless inspect, diagnostics,
  and support exports replay the same record, so they agree on freshness, order
  reason, and hidden-scope state for the same object.
