# Add Memory-Analysis Views, Snapshot Pairs, Retained or Allocation Diffs, and Leak-Hint Confidence

This document is the reviewer-facing landing page for the M5 memory-analysis lane.

## Scope

This lane governs how profiler and trace surfaces:

- present memory-analysis views that show retained sizes, allocation counts, class
  histograms, instance lists, and dominator trees;
- manage snapshot pairs with explicit baseline and comparison refs, snapshot kind,
  and comparison basis so users always know what is being compared;
- compute retained diffs between snapshots with honest mapping quality and
  degraded-state labels;
- compute allocation diffs between snapshots with honest mapping quality and
  degraded-state labels;
- surface leak hints with explicit confidence levels so users are not misled by
  weak or uncertain evidence.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/add_memory_analysis_views_snapshot_pairs_retained_or_allocation_diffs_and_leak_hint_confidence/`
- **Packet:** `artifacts/perf/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.json`
- **Schema:** `schemas/perf/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.schema.json`
- **Fixtures:** `fixtures/performance/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Retained size view | Stable | Shows snapshot pairs, retained diffs, mapping quality, and degraded-state labels. |
| Allocation count view | Stable | Shows snapshot pairs, allocation diffs, mapping quality, and degraded-state labels. |
| Class histogram view | Stable | Shows snapshot pairs, mapping quality, and degraded-state labels. |
| Diff view | Stable | Shows retained and allocation diffs, snapshot pairs, mapping quality, and degraded-state labels. |
| Leak hint view | Stable | Shows leak hints with confidence levels, snapshot pairs, mapping quality, and degraded-state labels. |
| Snapshot pair browser | Stable | Shows snapshot pairs with comparison basis, mapping quality, and degraded-state labels. |
| Export review | Preview | Redaction-safe export flows for memory-analysis evidence are still under qualification. |
| Support export | Preview | Support-bundle redaction for memory-analysis payloads is still under qualification. |

## Snapshot Pairs

Snapshot pairs carry a closed snapshot-kind vocabulary:

- `heap_dump` — full heap dump;
- `allocation_trace` — allocation trace with call stacks;
- `live_object_set` — live object set snapshot.

Every pair MUST show its comparison basis (`retained_diff`, `allocation_diff`,
`shallow_diff`, or `object_count_diff`) and MUST show a degraded-state label when
mapping quality is incomplete.

## Retained and Allocation Diffs

Diff rows bind a snapshot pair to a type or class and show baseline and comparison
values with deltas. Every diff row MUST show its mapping quality so users never
trust false precision.

## Leak Hints

Leak hints carry a closed confidence vocabulary:

- `high` — strong evidence of a leak;
- `medium` — moderate evidence of a leak;
- `low` — weak evidence of a leak;
- `uncertain` — insufficient evidence to claim a leak.

Every leak hint MUST show its confidence level and MUST include a rationale
explaining the evidence. Hints with `low` or `uncertain` confidence MUST NOT be
presented as actionable without additional context.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Memory-analysis views MUST show mapping quality and degraded-state labels;
  missing labels trigger validation violations.
- Snapshot pairs MUST show comparison basis and degraded-state labels; missing
  labels trigger validation violations.
- Retained and allocation diffs MUST show mapping quality; missing labels trigger
  validation violations.
- Leak hints MUST show confidence levels; missing confidence triggers a validation
  violation.
- Cross-reference failures (unknown snapshot pair refs) trigger validation
  violations.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Every memory-analysis view that references a snapshot pair points to a known
  pair.
- Every diff and leak hint points to a known snapshot pair.
- Every leak hint carries a confidence level and rationale.
