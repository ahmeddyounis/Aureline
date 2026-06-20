# Corpus-intake and redaction policy — normative

This document is the **normative** policy for admitting a benchmark corpus into
the protected lanes. It makes corpus licensing, privacy/redaction posture,
retention, and synthetic-fallback substitution reviewable and enforceable, so
partner- or customer-derived material cannot drift into a protected metric or a
public-proof pack on benchmark convenience alone.

If this document disagrees with the machine-readable schema or ledger, the schema
and ledger win and this document is updated in the same change.

Companion artifacts:

- [`/schemas/benchmarks/corpus-intake-record.schema.json`](../../schemas/benchmarks/corpus-intake-record.schema.json)
  — boundary schema for a corpus-intake record and the intake ledger.
- [`/artifacts/benchmarks/corpus-intake-ledger.json`](../../artifacts/benchmarks/corpus-intake-ledger.json)
  — canonical machine-readable intake ledger (the truth source).
- [`/artifacts/benchmarks/corpus-intake-policy.md`](../../artifacts/benchmarks/corpus-intake-policy.md)
  — human-readable policy rendering and the current intake ledger.
- [`/fixtures/benchmarks/redaction/`](../../fixtures/benchmarks/redaction/)
  — intake fixtures proving each fail-closed path.
- [`/ci/check_benchmark_corpus_intake.py`](../../ci/check_benchmark_corpus_intake.py)
  — the validator that enforces this policy.

This policy sits **alongside** the
[benchmark-governance matrix](./m5-benchmark-governance.md), which binds each
protected metric to a named corpus. This document governs whether that corpus is
*allowed in* at all: the licensing, privacy, redaction, retention, and
synthetic-fallback intake behind every corpus a protected metric or a
public-proof pack rests on. It extends, and does not restate, the corpus
change-control rules in [`corpus_governance.md`](./corpus_governance.md), the
reviewer workflow in [`privacy_cleared_corpus_workflow.md`](./privacy_cleared_corpus_workflow.md),
or the public-comparison rules in [`public_comparison_rules.md`](./public_comparison_rules.md).

## 1. Why an intake gate exists

The governance matrix records *which* corpus each protected metric measures. What
it left implicit was whether that corpus was *legally and privately safe to use*:
its license, its sensitivity class, whether it was redacted, who owns its
retention, and — when the real data cannot enter CI — what synthetic substitute
keeps the metric reproducible. Without that record, a partner or customer corpus
can ride into a protected lane on benchmark convenience, and a sensitive capture
can outlive its retention window unnoticed. The intake ledger closes that gap:
every corpus a protected metric binds carries a reviewable intake record, and a
corpus with no approved intake fails closed.

## 2. The intake record

Each record is a single self-describing object validated by its boundary schema.
It carries no raw fixture bytes, no raw partner or customer names, and no raw run
logs — only stable ids and reviewable sentences. The governed fields are:

- **`corpus_origin_class`** — what the content is and where it came from:
  `original_project_authored`, `synthetic_generated`, `vendored_third_party`,
  `partner_provided`, `customer_provided`, or `field_collected`. The last four
  are *licensed origins* and need a cleared license.
- **`sensitivity_class`** — the corpus sensitivity surfaced downstream:
  `non_sensitive`, `internal_only`, `partner_confidential`,
  `customer_confidential`, or `regulated_personal_data`. The last three are
  *sensitive* and need an approved privacy review.
- **`license`** — its status, whether it is `cleared` for the approved uses, and
  whether attribution is required.
- **`redaction`** — its `redaction_class` (`no_redaction_needed`,
  `redaction_applied_verified`, `redaction_pending`, or
  `unredactable_use_synthetic`), the method, and the verification date.
- **`retention`** — its `retention_class`, the accountable `retention_owner_ref`,
  and a `purge_due_on` date for time-boxed material.
- **`approved_use_classes`** — the lanes the corpus may back: `protected_ci_gate`,
  `public_head_to_head_proof`, `aureline_only_proof`, `methodology_only`, or
  `internal_exploration_only`.
- **`ci_admissibility`** — `admitted_real_data`, `admitted_synthetic_only`,
  `blocked_pending_intake`, or `blocked_unredactable`.
- **`synthetic_fallback`** — whether a synthetic substitute is required, the
  fallback corpus id, its availability, and a fidelity note.
- **`intake_decision`** and **`approvals`** — the decision status and the
  approval lineage (authority, approver, date).

## 3. Partner or customer data cannot enter on convenience

A sensitive corpus (`partner_confidential`, `customer_confidential`, or
`regulated_personal_data`) MUST NOT enter a protected lane without **all** of:

1. an **approved** `intake_decision`;
2. a **cleared** license — and, for any licensed origin, a `legal_review`
   approval;
3. both a `data_steward` **and** a `privacy_review` approval;
4. a verified redaction posture (see §4); and
5. a time-boxed retention with a named owner and an unexpired purge date (see §5).

A corpus that is `blocked_pending_intake` or `blocked_unredactable` may not claim
a `protected_ci_gate` or public-proof use. A real-data admission whose corpus id
is not materialised in the
[corpus register](../../fixtures/benchmarks/corpus_manifest.yaml) is rejected:
real data that gates CI lives in the repository under the register, never as an
unresolved id.

## 4. Redaction posture gates real-data CI

Redaction posture decides whether *real* bytes may enter CI:

| Redaction class | Real-data CI | Meaning |
| --- | --- | --- |
| `no_redaction_needed` | allowed | synthetic or original content |
| `redaction_applied_verified` | allowed | a verified redaction pass cleared the bytes |
| `redaction_pending` | **blocked** | redaction is drafted but not verified; use synthetic until it is |
| `unredactable_use_synthetic` | **blocked** | the content cannot be safely materialised; a synthetic fallback must back the metric |

A `redaction_pending` or `unredactable_use_synthetic` corpus marked
`admitted_real_data` is rejected. It may still be `admitted_synthetic_only`, where
the real data stays out of CI and an identified synthetic fallback runs in its
place.

## 5. Sensitive corpora are time-boxed

A sensitive corpus MUST carry `retention_class: sensitive_time_boxed`, a named
`retention_owner_ref`, and a hard `purge_due_on` date. An **admitted** sensitive
corpus whose `purge_due_on` is in the past **fails closed**: the data should
already have been purged or re-cleared. This keeps a partner or customer capture
from lingering in a benchmark lane past the window it was admitted for.

## 6. Synthetic fallback keeps metrics reproducible

When real data cannot enter CI, the metric does not lose its bar — it runs against
an **identified synthetic fallback**. A corpus marked `admitted_synthetic_only`
MUST mark its `synthetic_fallback` required, name a `fallback_corpus_ref` that
resolves in the corpus register, and show its `status` as `available`. The
fallback is named, not implied: a reviewer can see exactly which synthetic corpus
stands in for the sensitive source and how faithfully it reproduces the workload.
A required fallback that is unavailable or unresolved is rejected.

## 7. Downstream packets explain sensitivity without leaking content

The validator projects a **redaction-safe sensitivity view**: for each corpus, its
sensitivity class, redaction class, CI admissibility, approved use classes,
synthetic-fallback status, and purge window — and nothing else. Release, support,
and evaluation packets read this projection to explain *what kind* of corpus backs
a claim without ever surfacing the raw content, raw partner or customer names, or
raw run logs behind it.

## 8. Procedure for admitting a corpus

1. Record source lineage before inspecting or transforming bytes (see the
   [privacy-cleared corpus workflow](./privacy_cleared_corpus_workflow.md)).
2. Add an intake record to the ledger with the origin, sensitivity, license,
   redaction, retention, and approved use classes.
3. For a sensitive or licensed corpus, capture the `data_steward`,
   `privacy_review`, and `legal_review` approvals and set the intake decision
   `approved`.
4. If the real data cannot enter CI, set `ci_admissibility: admitted_synthetic_only`,
   mark the synthetic fallback required, and name an available fallback corpus.
5. For sensitive material, set `retention_class: sensitive_time_boxed` with an
   owner and a future purge date.
6. Run [`ci/check_benchmark_corpus_intake.py`](../../ci/check_benchmark_corpus_intake.py)
   and the [governance gate](../../ci/check_m5_benchmark_governance.py).

## 9. What this policy is not

- It is **not** the corpus change-control policy; that stays in
  [`corpus_governance.md`](./corpus_governance.md).
- It is **not** the reviewer intake workflow for reference workspaces; that stays
  in [`privacy_cleared_corpus_workflow.md`](./privacy_cleared_corpus_workflow.md).
- It is **not** a general data-governance policy; it governs only benchmark and
  certification corpora.
- It does **not** hold the threshold history of a metric; that stays in
  [`threshold-change-policy.md`](./threshold-change-policy.md).
