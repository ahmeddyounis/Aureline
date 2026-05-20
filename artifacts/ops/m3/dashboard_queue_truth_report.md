# Dashboard & queue truth — audit report

This report is the deterministic, support- and release-facing summary of the
operator dashboard & queue truth audit lane. It is generated from the checked-in
view fixtures and validated by
[`ci/check_dashboard_queue_truth.py`](../../../ci/check_dashboard_queue_truth.py),
run from
[`scripts/ci/run_dashboard_queue_truth.sh`](../../../scripts/ci/run_dashboard_queue_truth.sh).

It draws on:

- the Rust-pinned runtime fixtures under
  [`fixtures/ops/m3/dashboard_and_queue_truth/`](../../../fixtures/ops/m3/dashboard_and_queue_truth/),
  minted by the shell emitter and replay-pinned by
  `crates/aureline-shell/tests/dashboard_truth_fixtures.rs`;
- the audit drills and companions under
  [`fixtures/ops/m3/dashboard_queue_truth_corpus/`](../../../fixtures/ops/m3/dashboard_queue_truth_corpus/);
- the boundary schemas
  [`schemas/ops/dashboard_freshness_card.schema.json`](../../../schemas/ops/dashboard_freshness_card.schema.json)
  and
  [`schemas/ops/queue_order_reason.schema.json`](../../../schemas/ops/queue_order_reason.schema.json);
- the beta contract at
  [`docs/ops/m3/dashboard_queue_truth_beta.md`](../../../docs/ops/m3/dashboard_queue_truth_beta.md).

## Why this lane exists

The dashboard/queue beta made operator surfaces *truthful*: a row carries its
freshness, the age of its last-successful evidence, the canonical object behind
its open-details path, and — for queues — why each row is sorted where it is and
what is hidden by scope. This lane keeps that truthful under the failure modes
that quietly creep back in during beta hardening, so every marketed
operator-facing row carries current evidence that it does not lie:

- **stale success** — a green that no longer has fresh data or current evidence
  behind it;
- **unexplained ordering** — a queue whose sort cannot be explained to a user;
- **hidden narrowing** — a list that drops rows without disclosing how many or
  why;
- **broken evidence reopen** — an open-details / reveal path that no longer
  routes to the durable object behind the row, including after a restart or a
  reconnect.

The lane runs in CI and nightly, and it produces stable failures for these
regressions instead of relying on screenshots or tribal knowledge.

## How truth is enforced

The validator is a second, independent implementation of the freshness/order
model (`crates/aureline-shell/src/dashboard_truth/model.rs`). For every view
fixture it re-derives the honest record from the raw inputs and asserts the
stored record matches, then applies the lane-failing invariants:

1. **No silent green.** `effective_state == clear` is reachable only from a
   declared-clear, fresh, evidence-current row. Any other declared-clear row
   downgrades to `unconfirmed`, sets `green_downgraded`, lights the honesty
   marker, and names the downgrade reasons.
2. **Order is explainable.** Each visible queue row carries one closed-vocabulary
   order reason, a reviewable explanation, a canonical `open_details_ref`, and a
   rank `1..=N`.
3. **Narrowing is explainable.** Each hidden bucket carries a closed narrowing
   reason, a positive count, a reviewable explanation, a canonical `reveal_ref`,
   and the correct `incomplete_knowledge` flag.
4. **Canonical routing.** Every evidence / open-details / reveal ref is
   `aureline://<class>/<id>`, never a generic landing page.
5. **Export parity.** The support-export plaintext and the CLI / headless index
   projection preserve the same freshness / order / hidden semantics as the
   product UI record.

## Scenario packet

This is the current dashboard/queue packet a beta scorecard can attach to each
marketed operator-facing surface. Every row reflects a fixture that passes the
lane today.

| Scenario | Surface | Effective state | Freshness | Honesty | Green downgrades | Hidden |
| -------- | ------- | --------------- | --------- | ------- | ---------------- | ------ |
| `service_health_all_clear` | service_health | clear | fresh | none | 0 | 0 |
| `service_health_stale_green` | service_health | unconfirmed | stale | present | 2 | 0 |
| `service_health_partial_offline` | service_health | blocked | unavailable | present | 1 | 0 |
| `review_inbox_ordered_narrowed` | review_inbox | blocked | stale | present | 1 | 21 |
| `review_inbox_order_ambiguity` | review_inbox | blocked | fresh | present | 0 | 6 |
| `incident_queue_severity_sla` | incident_queue | blocked | cached | present | 1 | 5 |
| `support_queue_policy_scoped` | support_queue | blocked | policy_blocked | present | 0 | 9 |
| `support_queue_restart_evidence_break` | support_queue | unconfirmed | unavailable | present | 2 | 4 |
| `admin_queue_offline_partial` | admin_queue | unconfirmed | partial | present | 1 | 27 |

### Order and narrowing vocabulary per queue

| Scenario | Order reasons present | Narrowing reasons present |
| -------- | --------------------- | ------------------------- |
| `review_inbox_ordered_narrowed` | assigned_to_you, oldest_unresolved_first, severity_descending | assignee_filter, resolved_hidden, scope_filter |
| `review_inbox_order_ambiguity` | assigned_to_you, blocking_dependency, default_recency, oldest_unresolved_first | assignee_filter, scope_filter |
| `incident_queue_severity_sla` | recently_updated, severity_descending, sla_deadline | severity_filter |
| `support_queue_policy_scoped` | blocking_dependency, default_recency, sla_deadline | policy_scope |
| `support_queue_restart_evidence_break` | default_recency, oldest_unresolved_first | offline_partial_list |
| `admin_queue_offline_partial` | manual_pin, recently_updated | archived_hidden, offline_partial_list |

## The two audit drills

### Risk vs time vs owner ordering — `review_inbox_order_ambiguity`

The same inbox could be sorted by risk, by time, or by owner. The drill proves
each visible row names which principle placed it: a blocking review leads on
**risk** (`blocking_dependency`), a review assigned to you follows on **owner**
(`assigned_to_you`), the oldest unresolved review follows on **time**
(`oldest_unresolved_first`), and a recently opened review is placed on the
**default** recency order. An operator never has to guess why one review outranks
another. (`review_inbox_ordered_narrowed` independently spans the same three
principles.)

### Broken evidence after restart / reconnect — `support_queue_restart_evidence_break`

After a restart and a reconnect, a case restored from a snapshot has a broken
evidence link (`evidence_age = never`) and a reconnecting case has evidence that
aged out (`evidence_age = very_stale`). Both declared themselves clear; both
downgrade to `unconfirmed`, set `green_downgraded`, and name the downgrade
reasons (`source_offline` / `freshness_expired` + `evidence_aged_out`). A third
case that re-confirmed within the review window goes clear again. Every row keeps
a canonical `aureline://support_case/<id>` reopen path even when its evidence is
not current — so the open-evidence affordance survives the restart even though the
green claim does not.

## Export parity

For every scenario the lane re-renders the support-export plaintext (the block
support bundles ship) and the CLI / headless index line, then re-derives the
freshness / order / hidden semantics from each projection and asserts they match
the product UI record. The full per-scenario projections and digests are in
[`fixtures/ops/m3/dashboard_queue_truth_corpus/export_parity_packet.json`](../../../fixtures/ops/m3/dashboard_queue_truth_corpus/export_parity_packet.json).

A support bundle and a headless inspect therefore cannot disagree with the
desktop UI about whether a row is stale, why it is sorted where it is, or how
much of a list is hidden.

## How to run

```sh
# Fast, deterministic gate (no toolchain needed):
scripts/ci/run_dashboard_queue_truth.sh --no-cargo

# Full gate (also re-emits the runtime fixtures and runs the Rust replay test
# when a Cargo toolchain is available):
scripts/ci/run_dashboard_queue_truth.sh

# Machine-readable findings:
python3 ci/check_dashboard_queue_truth.py --report-json artifacts/ops/m3/dashboard_queue_truth_corpus_report.json

# Regenerate the audit fixtures, matrix, and parity packet after a model change:
python3 ci/check_dashboard_queue_truth.py --write
```

## Scope

This lane is proof scaffolding over the already-claimed beta operator surfaces.
It does not build new dashboard products or non-operator analytics, and it
favors deterministic synthetic fixtures over live-service timing so it stays fast
enough to run repeatedly during beta hardening.
