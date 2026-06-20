# Protected-threshold change policy — normative

This document is the **normative** policy for changing a protected benchmark
metric's threshold. It makes threshold easings reviewable, evidence-backed,
time-bounded, and promotion-blocking instead of implicit, so a protected
performance or certification bar cannot move silently through an unrelated PR or
an informal shiproom decision.

If this document disagrees with the machine-readable schema or ledger, the schema
and ledger win and this document is updated in the same change.

Companion artifacts:

- [`/schemas/benchmarks/threshold-change-record.schema.json`](../../schemas/benchmarks/threshold-change-record.schema.json)
  — boundary schema for a threshold-change record and the change ledger.
- [`/artifacts/benchmarks/threshold-change-ledger.json`](../../artifacts/benchmarks/threshold-change-ledger.json)
  — canonical machine-readable change ledger (the truth source).
- [`/artifacts/benchmarks/threshold-change-workflow.md`](../../artifacts/benchmarks/threshold-change-workflow.md)
  — human-readable workflow rendering and the current change ledger.
- [`/fixtures/benchmarks/threshold-change/`](../../fixtures/benchmarks/threshold-change/)
  — workflow fixtures proving each fail-closed path.
- [`/ci/check_benchmark_threshold_change.py`](../../ci/check_benchmark_threshold_change.py)
  — the validator that enforces this policy.

This policy sits **underneath** the
[benchmark-governance matrix](./m5-benchmark-governance.md), which holds each
protected metric's *current* threshold state and waiver binding. This document
governs the *transition* between states: the typed change record that carries the
rationale, before/after evidence, owner, approvals, and waiver expiry behind every
move. It does not restate the corpus change-control policy in
[`corpus_governance.md`](./corpus_governance.md) or the public-comparison rules in
[`public_comparison_rules.md`](./public_comparison_rules.md).

## 1. Why a typed change record exists

The matrix records *where* a protected bar sits today. What it left implicit was
*how it got there*: which evidence, which rationale, which owner, which approvals,
and — for a waiver — when the exception expires. Without that record an easing can
hide inside the feature PR that benefits from it, and a time-boxed waiver can
outlive its expiry unnoticed. The threshold-change ledger closes that gap: every
protected metric's threshold history is a list of reviewable records, and the
in-force record for each metric is kept in lockstep with the matrix.

## 2. The change record

Each record is a single self-describing object validated by its boundary schema.
It carries no raw run logs, raw machine labels, or raw provider payloads — only
stable ids and reviewable sentences. The governed fields are:

- **`change_kind`** — one of `set_calibrated`, `tightened`, `eased_with_evidence`,
  `provisional_hold`, `recalibration_reset`. Each kind maps to exactly one
  `resulting_threshold_state`; a mismatch is rejected.
- **`loosens_protected_bar`** — true only for an `eased_with_evidence` change; an
  easing carries the stricter approval and evidence requirements below.
- **`rationale`** and **`comparability_note`** — the structured reason for the
  change and the comparability conditions it preserves.
- **`before_after_evidence`** — a `before` and an `after` side, each naming the
  evidence packet that backs it and the date it was captured. The after capture
  must be no earlier than the before capture.
- **`owner_ref`** and **`approvals`** — the governing owner and the approval
  lineage (authority, approver, and date).
- **`waiver`** — the waiver grant: its class, an owner-resolvable ref, the grant
  date, and a hard expiry. The default class is `none` and carries no grant.
- **`release_evidence_ref`** — the release-evidence packet that carries the
  change; required for an easing.
- **`status`** — `proposed`, `approved`, `active`, `superseded`, or `withdrawn`.
  Exactly one `active` record is in force per metric.

## 3. Easing a protected bar

A move to `eased_with_evidence` is the only change that may loosen a protected
bar, and it is the most strictly governed. Before it lands it MUST carry:

1. a structured `rationale` and a `comparability_note`;
2. `before_after_evidence` on both sides with capture dates in order;
3. a `release_evidence_ref` linking the release-evidence packet; and
4. **both** a performance authority (`performance_owner` or `performance_council`)
   **and** an architecture authority (`architecture_board` or
   `architecture_council`) in its approvals.

These mirror the `eased_with_evidence` requirements in the governance matrix and
the `threshold_easing_requirements` in
[`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml).
An easing missing either approval, the release evidence, or consistent before/after
evidence is rejected. **Threshold easing never rides hidden inside the feature PR
that benefits from it** — it lands as its own reviewable record.

## 4. Waivers are time-boxed

A non-default waiver is a time-boxed exception, not a standing one. Every
non-`none` waiver MUST carry an owner-resolvable `waiver_ref`, a hard `expires_on`
date, and the approval of its granting authority:

| Waiver class | Granting approval(s) required |
| --- | --- |
| `performance_council_time_boxed` | a performance authority |
| `architecture_council_protected_path` | an architecture authority **and** a performance authority |
| `release_council_launch_scope` | the release council |
| `shiproom_executive_scope` | shiproom executive-scope review |

A waiver with no expiry, no ref, or no granting approval is rejected. A `none`
waiver that nevertheless carries a ref or a date is rejected.

## 5. Expired waivers block promotion

An **open, in-force** waiver (a record with `status: active`) whose `expires_on`
is in the past **blocks promotion**. The validator fails closed on it; the only
ways out are to renew the waiver with fresh approval, close the record
(`superseded` or `withdrawn`), or remediate the underlying metric so it no longer
needs the exception. A waiver on a closed (`superseded` / `withdrawn`) record is
history and does not block.

This is the same obligation the governance matrix expresses through the
`expired_waiver` narrowing rule, made enforceable at the change-record level: an
expired exception cannot quietly keep a claim alive.

## 6. Shiproom and release surface the active waivers

The validator projects, for every in-force record, the active waivers and their
expiry dates — the waiver class, the owner-resolvable ref, the days remaining, and
whether any is expired-open. This projection is what a shiproom or release packet
shows: a reviewer sees, before promotion, exactly which protected metrics ride on
a live exception and when each expires. An expired-open waiver appears flagged and
fails the gate.

## 7. The in-force record matches the matrix

For each protected metric there is exactly one `active` record, and its
`resulting_threshold_state` and waiver binding MUST equal the matrix's threshold
state and waiver for that metric. This keeps the threshold from drifting in the
ledger without the matrix, or vice versa: a change to a protected bar shows up in
both, reviewably, or the gate fails.

## 8. Procedure for changing a protected threshold

1. Land the threshold value change in
   [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml).
2. Add a threshold-change record to the ledger with the correct change kind,
   rationale, comparability note, before/after evidence, owner, and — for an
   easing — both approvals and the release-evidence link.
3. For a waiver, set its class, ref, grant date, and a hard expiry, and record the
   granting authority's approval.
4. Mark the new record `active` and supersede the prior in-force record for that
   metric.
5. Update the matrix's threshold state and waiver binding to match.
6. Run [`ci/check_benchmark_threshold_change.py`](../../ci/check_benchmark_threshold_change.py)
   and the [governance gate](../../ci/check_m5_benchmark_governance.py).

## 9. What this policy is not

- It is **not** the corpus change-control policy; that stays in
  [`corpus_governance.md`](./corpus_governance.md).
- It is **not** the public-comparison rule set; that stays in
  [`public_comparison_rules.md`](./public_comparison_rules.md).
- It does **not** hold the threshold *values*; those stay in
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml).
- It does **not** govern non-protected dashboard thresholds; only the named
  protected metrics in the governance matrix.
