# M5 Execution-Lifecycle Component Accessibility Fallback & Auto-Narrowing (M05-826)

This lane is the accessibility-and-auto-narrowing capstone over the frozen
[M5 execution-lifecycle component matrix](m5_execution_lifecycle_component_matrix.md).
Where the freeze matrix defines the reusable run/attempt-header,
input-request-prompt, artifact-publish-row, rerun-comparison-sheet,
debug-session-header, thread/process-tree, and dump/crash-artifact-card primitives
and the 821–824 implementation lanes resolve their per-surface truth, M05-826
certifies — per component family — that execution-lifecycle claims stay
**keyboard-complete, screen-reader-reachable, CLI/export-safe, and self-narrowing**
rather than presenting a stale or partial lane as fully current, fully controllable,
or fully attributable.

- Module: `crates/aureline-runtime/src/implement_keyboard_screen_reader_cli_export_parity_and_execution_lifecycle_auto_narrowing/`
- Boundary schema: `schemas/ui/m5-execution-lifecycle-accessibility-fallback.schema.json`
- Release proof: `artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- Fixtures: `fixtures/ui/m5-execution-lifecycle-accessibility-fallback/`

## What each row certifies

Every [`ExecutionAccessibilityRow`] keys on one frozen
`M5ExecutionComponentFamily` and reuses the frozen `M5ExecutionRequiredLabel` and
`M5ExecutionDowngradeTrigger` and the shared `M5RunAttemptSurfaceFamily` consumer
vocabulary rather than minting synonyms, so certified labels stay byte-identical to
the matrix and the sibling primitive packets.

### Reach & export parity

- **`keyboard_reach` / `screen_reader_reach` / `cli_reach`** — each is a tri-state
  `reachable_and_labeled` / `disclosed_reduced_but_reachable` / `view_only_trap`.
  A `view_only_trap` on any axis strands the row (red).
- **Hierarchy-heavy families** (the thread/process tree) must render a `structured`
  modality *and* a non-visual (`list` / `textual` / `cli`) fallback so the tree is
  navigable non-visually.
- **`export_summary` + `copy_export`** — the support/release export reconstructs the
  component from typed tokens and opaque refs without a screenshot, offering text /
  JSON / Markdown copy of the same run/attempt IDs, target boundaries, and
  artifact/mapping states shown in-product.

### Interactive-claim auto-narrowing

Each family declares a `full_interactive_claim` — the strongest control it asserts
when every dimension is intact — from strongest to weakest:

| Claim | Meaning |
| --- | --- |
| `full_interactive` | live control: dispatch, answer input, continue / pause, re-open live |
| `review_required` | action allowed only behind an explicit review step |
| `read_only` | copy / export allowed, action is not |
| `inspect_only` | captured evidence may be viewed; nothing acted on |

Each row models the observed condition of its execution dimensions
(`attempt_lineage`, `input_state`, `artifact_freshness`, `mapping_quality`,
`target_identity`) as `intact` / `partial` / `stale` / `unavailable` /
`policy_blocked`. A weakened dimension imposes a ceiling — `partial` →
`review_required`, `stale` → `read_only`, `unavailable` / `policy_blocked` →
`inspect_only` — and the effective claim is narrowed to the weakest ceiling across
all modeled dimensions, capped at the family's full claim.

When a dimension narrows the claim below the full claim, the row carries a
`claim_narrow` block naming the binding dimension, its frozen downgrade trigger, a
precise (non-generic) label, and `preserves_canonical_identity: true`. A row whose
dimensions are all intact must **not** carry a spurious narrow block.

## Acceptance criteria mapping

- **AC1 — a stale or partial lane can no longer present as fully current,
  controllable, or attributable.** `claim_is_honest()` requires the effective claim
  never to exceed the permitted ceiling, an honest `claim_narrow` whenever a
  dimension narrows below the full claim (matching the ceiling, binding dimension,
  and trigger), and no spurious narrow when everything is intact.
- **AC2 — accessibility and export surfaces preserve the same run/attempt IDs,
  target boundaries, and artifact/mapping states.** `reaches_canonical_truth_via_at()`
  requires a non-trapping keyboard / screen-reader / CLI path to the same canonical
  truth (execution context always visible), a non-visual fallback for hierarchy-heavy
  families, and `export_preserves_meaning()` (never screenshot-only; complete
  text/JSON/Markdown copy parity).
- **AC3 — claim publication and field triage stay aligned on downgrade behavior.**
  `narrowing_disclosed()` requires every narrowed rendering surface to carry a
  disclosure that preserves labels and never silently drops state; the same narrowed
  state surfaces in UI, docs/help, release packets, and the support export (which is
  this packet).

Coverage lints additionally require every frozen family, every claim dimension, and
every interactive-claim tier (`full_interactive` → `review_required` → `read_only` →
`inspect_only`) to be exercised across the packet, so the full narrowing spectrum is
proven end-to-end.

## Seeded proof

The checked-in packet certifies all 7 families: **2 green / 5 yellow / 0 red**.

| Row | Family | Weakened dimension | Full → effective claim |
| --- | --- | --- | --- |
| `a11y:run-attempt-header` | run/attempt header | — (intact) | full_interactive → full_interactive |
| `a11y:input-request-prompt` | input-request prompt | input_state (policy_blocked) | full_interactive → inspect_only |
| `a11y:artifact-publish-row` | artifact-publish row | artifact_freshness (stale) | full_interactive → read_only |
| `a11y:rerun-comparison-sheet` | rerun comparison sheet | — (intact) | review_required → review_required |
| `a11y:debug-session-header` | debug session header | target_identity (partial) | full_interactive → review_required |
| `a11y:thread-process-tree` | thread/process tree | target_identity (unavailable) | full_interactive → inspect_only |
| `a11y:dump-crash-artifact-card` | dump/crash artifact card | mapping_quality (unavailable) | read_only → inspect_only |

## Regenerating the artifacts

The seeded builder, the tests, and the on-disk export share one source of truth
(`seeded_m5_execution_a11y_fallback_packet()`). Regenerate the checked artifacts
with:

```sh
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_accessibility_fallback -- support \
  > artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_accessibility_fallback -- csv \
  > artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/matrix.csv
cargo run -p aureline-runtime --example dump_m5_execution_lifecycle_accessibility_fallback -- summary \
  > artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/report.md
```

`current_m5_execution_a11y_fallback_export()` (`include_str!`) re-reads and
validates the checked-in export; the `on_disk_export_matches_builder` test fails if
the artifact drifts from the seeded builder.
