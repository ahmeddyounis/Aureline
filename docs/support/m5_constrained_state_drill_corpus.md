# M5 Constrained-State Drill Corpus (Mixed-State Fixtures and Regression Drills)

Task: **M05-1262** — batch **B150** (constrained-file-state, canonical-source relation, and write-target-review
truth across claimed M5 editor, review, save, AI, repair, and export surfaces).

This lane is the **fixture-corpus and regression-drill** lane over the six constrained-current-object classes frozen
in the [constrained-file-state matrix](../../artifacts/program/m5-constrained-file-state-matrix.md). Where the
state-descriptor, badge-group, canonical-source-relation, write-review-sheet, and cross-actor-gate lanes make one
honest constrained-object *loop* real, this lane seeds the reusable corpus that proves those loops stay honest under
failure so that constrained-state truth regressions are **detectable instead of anecdotal**.

## What it seeds

Six fixture families — one per constrained-object class — are exercised by nine drills across the nine shared
consumer surfaces (tab chrome, breadcrumb trail, status bar, command palette, editor banner, diff / review header,
write-review sheet, AI / automation path, and support / export packet):

| Fixture family | Object class | Drills (problematic transitions) |
| --- | --- | --- |
| Read-only symlink / alias path | `read_only` | symlink / alias save, read-only + generated overlay |
| Generated / derived artifact | `generated` | generated-artifact drift, generated + policy-locked regenerate |
| Policy-locked managed mirror | `policy_locked` | policy-locked managed mirror |
| Projection / virtual view | `projection` | projection export |
| Managed, externally-owned source | `managed` | managed-mirror round trip, managed + captured-snapshot restore |
| Captured snapshot in workspace | `captured_snapshot` | captured snapshot in workspace |

Every drill attempts a direct write, watches it be **denied**, and routes to exactly the reviewed fallback path keyed
to the object class through the shared pure functions:

- `read_only` / `captured_snapshot` → **duplicate to an editable copy** (`read_only_blocked`)
- `generated` → **regenerate with preview** (`regenerate_only`)
- `policy_locked` → **request approval** (`approval_gated`)
- `managed` → **detach from the managed source** (`detach_required`)
- `projection` → **create an overlay patch** (`detach_required`)

## Acceptance criteria

1. **Every state class plus five mixed-state combinations.** The corpus covers all six classes as a primary and the
   five mixed-state combinations `read_only + generated`, `generated + policy_locked`, `policy_locked + managed`,
   `projection + captured_snapshot`, and `managed + captured_snapshot`. When two state classes materially affect
   behaviour, both stay visible instead of one badge hiding another.
2. **Drills catch lossy fallback, hidden second-state, or cross-surface disagreement regressions.** Each binding
   derives its blocked-write reason, chosen fallback path, required write disposition, and checkpoint / undo class
   from its object class through the shared pure functions, so a lossy direct write, a masked second state, a fallback
   that does not match its reason, or a grammar that drifts across surfaces is mechanically rejected.
3. **The support / export packet can replay a denial and chosen fallback.** Every binding records a denial expectation
   naming the exact blocked-write reason, the chosen reviewed fallback, the required write disposition, the
   checkpoint / undo class, and the reviewed-fallback ref, and binds back to screenshots, an accessibility check, the
   CLI / support export, and the health dashboard.

## Guardrails (hard invariants)

Every binding keeps its constrained state explicitly classified and, when mixed-state, keeps both facets visible.
None of the following may ever be true:

- lets one constrained state class hide another;
- silently falls back to a lossy direct write;
- gives AI / automation / import / repair a hidden bypass;
- leaves the canonical source or exact write target unstated;
- presents the object as directly writable or hides the recovery / regenerate path.

## Checked-in artifacts

- Schema: `schemas/program/m5-constrained-state-drill-corpus.schema.json`
- Support export: `artifacts/support/m5-constrained-state-drills/support_export.json`
- Matrix CSV: `artifacts/support/m5-constrained-state-drills/matrix.csv`
- Markdown summary: `artifacts/support/m5-constrained-state-drills/summary.md`
- Health dashboard: `dashboards/m5-constrained-state-drill-health.json`
- Narrowed fixtures: `fixtures/editor/m5-constrained-state-drills/{mixed_state_narrowed,read_only_generated_narrowed}.json`

## Regenerating

The example emitter is the only mint-from-truth path:

```text
cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- support-export
cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- report
cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- csv
cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- dashboard
cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- fixture-mixed-state-narrowed
cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- fixture-read-only-generated-narrowed
cargo run -p aureline-ui --example dump_m5_constrained_state_drill_corpus -- validate
```

Tests live in `crates/aureline-ui/src/m5_constrained_state_drill_corpus/tests.rs`.
