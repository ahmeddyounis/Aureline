# dashboard & queue truth — audit corpus

This is the audit lane for the marketed beta operator dashboards and queues. It
turns the freshness / order / evidence truth model into a durable, repeatable
proof that release and support teams can attach to every operator-facing row,
catching the quiet ways a dashboard or queue lies:

- **Stale green** — a row keeps an all-clear headline after its freshness or
  last-successful evidence expired.
- **Unexplained order / silent narrowing** — a queue sorts rows with no stated
  reason, or hides rows without disclosing how many or why.
- **Broken evidence routing** — an open-details / reveal path points at a
  generic landing page instead of the durable object behind the row.
- **Export drift** — a support bundle or CLI / headless projection loses the
  semantics the product UI shows.

## What lives here

This directory is the audit-lane corpus. The view records it draws from are the
Rust-pinned runtime fixtures; this lane joins them with the two drills the audit
lane adds, an enum-only matrix, and an export-parity packet.

| File | What it is |
| ---- | ---------- |
| `review_inbox_order_ambiguity.json` | Audit drill: a review inbox that could be sorted by risk, by time, or by owner discloses which principle placed each row. |
| `support_queue_restart_evidence_break.json` | Audit drill: after restart / reconnect, a row whose evidence link broke (never re-confirmed) or aged out loses its green headline and keeps a canonical reopen path. |
| `corpus_matrix.json` | Enum-only matrix: one row per scenario with its surface, roll-ups, order / narrowing vocabulary, and the lane properties it proves. |
| `export_parity_packet.json` | Per scenario, the support-export plaintext and CLI / headless index projections plus the semantic digest each one must preserve. |

The view records the lane validates come from two places:

- the Rust-pinned runtime fixtures under
  `fixtures/ops/m3/dashboard_and_queue_truth/` (minted by the shell emitter and
  replay-pinned bit-for-bit by
  `crates/aureline-shell/tests/dashboard_truth_fixtures.rs`); and
- the two audit drills in this directory.

Each view is a `dashboard_truth_view_record` validated against
`schemas/ops/dashboard_freshness_card.schema.json` (its embedded `queue_order`
against `schemas/ops/queue_order_reason.schema.json`).

## Scenarios

| Scenario | Surface | Overall | Proves |
| -------- | ------- | ------- | ------ |
| `service_health_all_clear` | service_health | clear / fresh | An all-clear headline is allowed only while every card is fresh and evidence-current. |
| `service_health_stale_green` | service_health | unconfirmed / stale | An expired probe and a cached card both downgrade from green and name why. |
| `service_health_partial_offline` | service_health | blocked / unavailable | Partial, offline, and policy-blocked cards each carry a distinct downgrade reason. |
| `review_inbox_ordered_narrowed` | review_inbox | blocked / stale | Blocking / assigned / oldest order, scope+assignee+resolved narrowing, and a green that downgrades from expired evidence. |
| `review_inbox_order_ambiguity` | review_inbox | blocked / fresh | Risk vs time vs owner ordering is disambiguated per row with a distinct explanation. |
| `incident_queue_severity_sla` | incident_queue | blocked / cached | Severity + SLA ordering and a cached monitoring incident that downgrades from green. |
| `support_queue_policy_scoped` | support_queue | blocked / policy_blocked | Blocking / SLA / default order; policy-scoped cases disclosed as unknown, not dropped. |
| `support_queue_restart_evidence_break` | support_queue | unconfirmed / unavailable | After restart/reconnect, a broken or aged-out evidence link cannot stay green and keeps a canonical reopen path. |
| `admin_queue_offline_partial` | admin_queue | unconfirmed / partial | A manual pin + recency order, an offline-partial list, and a partially loaded green that downgrades. |

## Lane-failing invariants

The validator `ci/check_dashboard_queue_truth.py` independently re-derives the
model (a second implementation of
`crates/aureline-shell/src/dashboard_truth/model.rs`) and fails the lane when:

- **No silent green.** A card is `effective_state == clear` only when it is
  declared clear, `freshness == fresh`, and `evidence_age` is current
  (`fresh` or `recent`). Any other declared-clear row must downgrade to
  `unconfirmed`, set `green_downgraded`, and light the honesty marker.
- **Order is explainable.** Every visible queue row carries one closed-vocabulary
  `order_reason`, a reviewable explanation, a canonical `open_details_ref`, and a
  rank `1..=N` in render order.
- **Narrowing is explainable.** Every hidden bucket carries a closed
  `narrowing_reason`, a positive count, a reviewable explanation, a canonical
  `reveal_ref`, and the correct `incomplete_knowledge` flag (policy scope and
  offline-partial lists are unknown rows, not deliberate filters).
- **Canonical routing.** Every `evidence_ref`, `open_details_ref`, and
  `reveal_ref` is `aureline://<class>/<id>`, never a generic landing page.
- **Coverage.** The corpus exercises every surface, freshness, evidence-age,
  order, narrowing, evidence-kind, and downgrade-reason token, plus the two
  audit properties: risk-vs-time-vs-owner order ambiguity and a restart/reconnect
  broken-evidence downgrade.
- **Export parity.** The support-export plaintext and the CLI / headless index
  projection preserve the same freshness / order / hidden semantics as the
  product UI record.

The matrix and parity packet are regenerated and drift-checked on every run, so
a regression in any roll-up, order, narrowing, or export projection fails the
lane instead of shipping silently.

## Regenerate

```sh
python3 ci/check_dashboard_queue_truth.py --write
```

This re-mints the two audit drill fixtures, `corpus_matrix.json`, and
`export_parity_packet.json` from the model. Run the full gate with
`scripts/ci/run_dashboard_queue_truth.sh`.
