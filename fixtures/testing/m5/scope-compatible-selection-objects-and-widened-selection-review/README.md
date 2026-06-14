# Scope-compatible selection-object fixtures

Proof fixtures for the portable test-selection packet
(`portable_test_selection_packet`). Each fixture is an export-safe packet that
`PortableSelectionPacket::validate` accepts and that exercises a specific truth
the contract guarantees.

- `widened_selection_review_resolves_without_silent_expansion.json` — a packet
  whose selection objects span the UI, CLI, AI, and support channels with
  rerun-all, rerun-failed, changed-since, snapshot-scoped, and explicit-items
  intents (a parameterized template kept distinct from its concrete invocations).
  Its compatibility assessments show a CLI rerun-failed selection that stays
  **compatible** and dispatches, an AI changed-since selection whose re-resolution
  would **widen** the scope and so opened a review the operator resolved by
  keeping the original scope (`rejected_keep_original` — preserves origin, never
  silently expanded), a UI selection whose **snapshot drifted** and is held
  pending review, and an imported CI overlay that is **blocked** from
  re-dispatching as a local rerun.

The boundary schema is
`schemas/testing/scope-compatible-selection-objects-and-widened-selection-review.schema.json`;
the contract doc is
`docs/testing/m5/scope-compatible-selection-objects-and-widened-selection-review.md`.
Regenerate the canonical export with:

```bash
cargo run -p aureline-runtime --example dump_scope_compatible_selection
```
