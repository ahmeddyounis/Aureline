# Generated-artifact write boundary

This document describes the *write boundary* for generated artifacts: what
happens when a user attempts a direct edit to a generated file. The canonical
packet is implemented in
[`crates/aureline-generated/src/write_boundary/mod.rs`](../../crates/aureline-generated/src/write_boundary/mod.rs)
and serialized to
[`artifacts/generated/write-boundary-packet.json`](../../artifacts/generated/write-boundary-packet.json).

The sibling
[`generated-artifact governance`](./m5-generated-governance.md) matrix
certifies generated-artifact truth one row per *class*, and the
[`generated-artifact descriptor`](./generated-artifact-descriptor.md) models
the per-*artifact* identity object the surfaces render. This lane models the
boundary itself — the decision the file-tree save gate, the review override
sheet, the diverged-state lineage, the AI context, and the support export all
render when an edit reaches a generated file. The durable diverged-from-generator
record an admitted override leaves is the same one described by the
[diverged-from-generator contract](./diverged_from_generator_contract.md).

## Why this exists

A generated file looks like any other file on disk, so a save can reach it the
same way it reaches a hand-authored source file. Without one typed decision,
each surface can guess differently about whether that edit is safe, whether it
needs review, or whether the file should be regenerated instead — and a block
can decay into a generic save failure buried in a log. This lane makes the
decision a first-class object: a direct edit is **admitted**, **held for a
reviewed override**, or **blocked in favor of regeneration**, always with a
visible reason and a recovery path.

## The five boundary states

Every decision names one [`BoundaryState`] — the writable condition of the
artifact:

| State | Meaning |
| --- | --- |
| `in_sync` | The derived bytes match the canonical source. |
| `drift_detected` | The derived bytes have diverged from the canonical source. |
| `source_missing` | The canonical source is absent; the artifact cannot be compared or regenerated against it. |
| `generator_unavailable` | The generator that rebuilds the artifact cannot run. |
| `regeneration_blocked_by_policy` | A policy forbids regenerating the artifact. |

## The decision engine

One engine — `decide_write_boundary` — folds the artifact subject into a
single [`WriteBoundaryDecision`]. The effective edit gate starts at the
declared writable-boundary posture and is **floored** by the boundary state;
it only narrows, never widens.

| Boundary state | Edit-gate floor |
| --- | --- |
| `in_sync` | — |
| `drift_detected` | `reviewed_override_required` |
| `source_missing` | `regenerate_only` |
| `generator_unavailable` | `regenerate_only` |
| `regeneration_blocked_by_policy` | `regenerate_only` |

The attempt outcome then follows from the gate and any recorded reviewed
override:

| Effective gate | Recorded override | Outcome |
| --- | --- | --- |
| `direct_edit_allowed` | — | `direct_edit_admitted` |
| `reviewed_override_required` | none | `blocked_pending_review` |
| `reviewed_override_required` | recorded | `override_admitted_with_divergence` |
| `regenerate_only` | any | `blocked_regenerate_first` |

A reviewed override is honored **only** on a `reviewed_override_required`
gate. It is never a force-write past a `regenerate_only` block, so there is no
escape hatch beyond the reviewed override model.

## Blocked, but never silent

Every blocked or escalated decision carries:

- **`why_blocked_tokens`** — stable tokens naming each input that blocked or
  escalated the edit (e.g. `declared_reviewed_override_required`,
  `boundary_drift_detected`). Empty only when the edit is directly admitted.
- **`guidance_line`** — a user-visible regenerate-first / override line. The
  block is never reduced to a generic save failure or hidden in a toast.
- **`canonical_source_jump`** — a jump action to the canonical source,
  present whenever the source is linkable.
- **`recovery`** — the recovery path: a reviewed override, a regeneration, or
  the precondition to restore first (the missing source, the unavailable
  generator, or the blocking policy).

## Reviewed override and the divergence it leaves

When a `reviewed_override_required` edit is admitted through a recorded
review, the decision leaves a durable [`DivergedFromGenerator`] state:

- it cites the recorded `override_review_ref`;
- it is flagged `diverged`;
- it carries a recovery path with both `regenerate_from_source` (discard the
  divergence) and `reconcile_into_source` (promote the change into the
  canonical source).

This is the one record the diverged-state lineage persists, so the divergence
survives the session and is never silently re-absorbed on the next
regeneration.

## Three-way compare without lost provenance

Every decision carries a [`ThreeWayCompare`] over three legs — the canonical
source, the current artifact, and the regenerated candidate. Each leg keeps
its `provenance_ref` even when the leg cannot be produced right now (a missing
source, or a regeneration that is blocked), so a compare always shows what the
artifact derives from and what a regeneration would yield. The
current-artifact leg is always available.

## One decision for every surface

Real consumers bind to the packet:

- `file_tree_save_gate` — `crates/aureline-vfs/src/save_conflict_suite/mod.rs`
- `review_override_sheet` — `crates/aureline-review/src/change_inspector/mod.rs`
- `diverged_state_lineage` — `crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs`
- `ai_context` — `crates/aureline-ai/src/context_inspector/mod.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`

## Regeneration

The proof packet and fixtures are projections of the seeded packet:

```bash
cargo run -q -p aureline-generated --example dump_write_boundary -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/generated/write-boundary-packet.json
```

The fixture corpus under
[`fixtures/generated/write-boundary/`](../../fixtures/generated/write-boundary/)
is generated the same way from the `fixtures` mode and split one file per
fixture. The replay gate in
[`crates/aureline-generated/tests/write_boundary.rs`](../../crates/aureline-generated/tests/write_boundary.rs)
fails CI if the artifact or fixtures drift from the seeded packet.
