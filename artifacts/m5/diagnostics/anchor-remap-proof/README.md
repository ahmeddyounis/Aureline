# M5 Anchor-Remap Proof

`support_export.json` is the checked support export of the M5 anchor-remap history
set (`AnchorRemapHistorySetPacket`). It is the canonical artifact downstream
editor, Problems, review, CLI, and support surfaces ingest through
`aureline_runtime::record_anchor_remap_history_with_revision_pairs_and_drift_states_across_m5_lanes::current_m5_anchor_remap_history_set_export`
instead of forking per-surface drift state.

The set proves that anchor drift is made **explicit** instead of being silently
dropped, "fixed", or relabeled when a finding's artifact moves. It carries five
append-only histories that span every drift lane and every remap state:

- **File edit** — a finding is anchored exact, then a live edit moves it so it only
  *contextually* survives.
- **Notebook cell identity change** — a cell is re-keyed; with no fresh mapping the
  finding is retained against a *stale* epoch.
- **Generated-artifact churn** — a generated region is regenerated and the anchor
  can no longer be located, so the finding becomes *unmapped* — recorded, not
  discarded.
- **Imported snapshot comparison** — an imported scan carries an *imported_static*
  location that is then mapped onto a later local revision from surrounding
  context.
- **Imported replay comparison** — a replayed support bundle carries an
  *imported_static* snapshot-only location that has not been locally revalidated.

Each history is an **append-only** sequence of entries. Every entry pairs an old
anchor ref with a new anchor ref, a resulting remap state derived from a typed
evidence basis, a from/to revision pair, the actor/tool that produced the remap,
and the drift lane. The sequence numbers are contiguous, the revision pairs are
continuous, and the anchor chain is continuous, so support and review flows get a
causal trail for every moved finding rather than a single overwritten "current"
state. A remap state must match its evidence basis: a row cannot jump back to
`exact` without `exact_range_preserved` evidence, which is the no-silent-repair
guarantee.

The editor, Problems, review, CLI, and support surfaces each receive a projection
that exposes the current remap state and the full append-only trail, and the
support export preserves each history's ordered entry trail rather than a lossy
display-only row.

`support_export.md` is the deterministic Markdown summary of the same packet.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_anchor_remap_history > \
  artifacts/m5/diagnostics/anchor-remap-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_anchor_remap_history summary > \
  artifacts/m5/diagnostics/anchor-remap-proof/support_export.md
cp artifacts/m5/diagnostics/anchor-remap-proof/support_export.json \
  fixtures/quality/m5/anchor-remap/anchor_remap_history_set.json
```

The artifact validates against
[`schemas/quality/anchor-remap-record.schema.json`](../../../../schemas/quality/anchor-remap-record.schema.json)
and is byte-identical to the protected fixture at
[`fixtures/quality/m5/anchor-remap/anchor_remap_history_set.json`](../../../../fixtures/quality/m5/anchor-remap/anchor_remap_history_set.json).
