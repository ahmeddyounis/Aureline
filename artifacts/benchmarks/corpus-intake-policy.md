# Corpus-intake policy — human-readable rendering

Human-readable rendering of the benchmark corpus-intake gate and the current
intake ledger. The machine-readable truth is at
[`artifacts/benchmarks/corpus-intake-ledger.json`](./corpus-intake-ledger.json),
validated by
[`schemas/benchmarks/corpus-intake-record.schema.json`](../../schemas/benchmarks/corpus-intake-record.schema.json).
The normative policy companion is
[`docs/benchmarks/corpus-intake-and-redaction.md`](../../docs/benchmarks/corpus-intake-and-redaction.md).
The ledger keeps every corpus the
[benchmark-governance matrix](./m5-benchmark-governance.json) binds admissible.

## Why this gate exists

The governance matrix records *which* corpus each protected metric measures. This
gate governs whether that corpus is *allowed in*: its licensing, sensitivity,
redaction posture, retention, and — when sensitive data cannot enter CI — its
synthetic fallback. Partner- or customer-derived material cannot ride into a
protected lane on benchmark convenience: it enters only with an approved intake
decision, a cleared license, a verified redaction posture, a data-steward and
privacy review, and a time-boxed retention. Where the real data cannot enter CI,
an identified synthetic fallback keeps the metric reproducible.

## Admitting a sensitive corpus — the steps

1. Record source lineage before inspecting or transforming bytes.
2. Add an intake record with the origin, sensitivity, license, redaction,
   retention, and approved use classes.
3. Capture the `data_steward`, `privacy_review`, and `legal_review` approvals and
   set the intake decision `approved`.
4. If the real data cannot enter CI, set `ci_admissibility: admitted_synthetic_only`,
   mark the synthetic fallback required, and name an available fallback corpus.
5. Set `retention_class: sensitive_time_boxed` with an owner and a future purge
   date.
6. Run the [intake gate](../../ci/check_benchmark_corpus_intake.py) and the
   [governance gate](../../ci/check_m5_benchmark_governance.py).

## CI admissibility

| Admissibility | Real data in CI | Use |
| --- | --- | --- |
| `admitted_real_data` | yes | materialised corpus gates CI directly |
| `admitted_synthetic_only` | no | only the named synthetic fallback runs |
| `blocked_pending_intake` | no | cannot back any protected lane |
| `blocked_unredactable` | no | cannot back any protected lane |

## Current intake ledger

Every corpus a protected metric binds carries an approved intake record. Sensitive
corpora run synthetic-only with a named, available fallback.

| Corpus | Origin | Sensitivity | Redaction | Admissibility | Synthetic fallback | Purge due |
| --- | --- | --- | --- | --- | --- | --- |
| `corpus.workflow.startup_warm_to_first_paint` | synthetic | non_sensitive | none | `admitted_real_data` | — | — |
| `corpus.micro.shaping_smoke_cases` | original | non_sensitive | none | `admitted_real_data` | — | — |
| `corpus.workflow.first_useful_edit_rust_self_host` | original | non_sensitive | none | `admitted_real_data` | — | — |
| `corpus.micro.interaction_safety_cases` | synthetic | non_sensitive | none | `admitted_real_data` | — | — |
| `corpus.partner.editor_session_traces` | partner | partner_confidential | unredactable | `admitted_synthetic_only` | `corpus.micro.interaction_safety_cases` (available) | `2026-12-05` |
| `corpus.customer.large_repository_capture` | customer | customer_confidential | pending | `admitted_synthetic_only` | `corpus.workflow.startup_warm_to_first_paint` (available) | `2026-09-30` |

## Redaction-safe sensitivity projection (release / support / evaluation)

The gate projects, for every corpus, a redaction-safe view — sensitivity class,
redaction class, CI admissibility, approved use classes, synthetic-fallback
status, and purge window — and nothing else. Release, support, and evaluation
packets read this projection to explain *what kind* of corpus backs a claim
without surfacing raw content, raw partner or customer names, or raw run logs.

| Corpus | Sensitivity | Redaction | Admissibility | Fallback status |
| --- | --- | --- | --- | --- |
| `corpus.partner.editor_session_traces` | partner_confidential | unredactable_use_synthetic | `admitted_synthetic_only` | available |
| `corpus.customer.large_repository_capture` | customer_confidential | redaction_pending | `admitted_synthetic_only` | available |

## Fixtures

[`fixtures/benchmarks/redaction/`](../../fixtures/benchmarks/redaction/) holds the
records the gate replays each run: a conforming non-sensitive corpus and a
conforming synthetic-fallback corpus plus fail-closed negatives — unmaterialised
real data, an origin/sensitivity mismatch, an uncleared license, a rejected
decision left admitted, a real-data admission with no approval, a blocked corpus
claiming a protected use, a sensitive corpus missing its privacy review, a
licensed corpus missing legal clearance, pending or impossible redaction in
real-data CI, an unmarked, unavailable, or unresolved synthetic fallback, a
sensitive corpus that is not time-boxed, an overdue retention window, and a record
missing a required field.
