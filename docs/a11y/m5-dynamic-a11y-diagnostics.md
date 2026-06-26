# M5 Dynamic-Surface Assistive-Tech Diagnostics

This document is the contract for the M5 assistive-technology diagnostics report that
turns AT health on the claimed custom-rendered surfaces into a diagnosable, exportable,
and release-gated system. Where the
[per-surface descriptors](./m5-surface-descriptors.md) bind a custom surface to its
semantic roles, label model, and OS bridge mapping, the
[announcement grammar](./m5-announcement-grammar.md) bounds how often a live region may
speak, and the [non-visual summaries](./m5-custom-surface-summaries.md) expose each
surface's structure, this report says *whether each surface is currently healthy* and
*whether it may ship* — bridge probes, the full diagnostic battery, announcement-spam
budgets, high-zoom/high-contrast/reduced-motion conformance, and a per-surface release
gate the release/public-truth automation reads.

- Record kind: `m5_dynamic_a11y_diagnostics_report`
- Schema: [`schemas/a11y/m5-dynamic-a11y-report.schema.json`](../../schemas/a11y/m5-dynamic-a11y-report.schema.json)
- Canonical support export: [`artifacts/a11y/m5-dynamic-a11y-diagnostics/support_export.json`](../../artifacts/a11y/m5-dynamic-a11y-diagnostics/support_export.json)
- Governance summary artifact: [`artifacts/a11y/m5-dynamic-a11y-diagnostics/dynamic-a11y-diagnostics-proof.md`](../../artifacts/a11y/m5-dynamic-a11y-diagnostics/dynamic-a11y-diagnostics-proof.md)
- Fixtures: [`fixtures/a11y/m5-bridge-and-announcement-drills/`](../../fixtures/a11y/m5-bridge-and-announcement-drills/)
- Producer: `aureline_shell::accessibility::diagnostics::current_stable_m5_dynamic_a11y_diagnostics_export`
- Headless emitter: `aureline_shell_m5_dynamic_a11y_diagnostics`

## Why this report exists

Dynamic-surface accessibility stays fragile unless Aureline can inspect bridge health,
over-announcement, zoom/contrast/motion regressions, and AT-specific failures through the
same support/export system used for other protected paths. Before this report, whether a
custom surface's OS bridge was connected, whether its live region was flooding, whether
its semantic tree was fully mapped, and whether it broke under high zoom, forced colors,
or reduced motion were implicit and reproduced by hand. This report makes AT health a
single governed packet: one diagnostics row per protected surface family, each carrying a
bridge probe, the full diagnostic battery, a published announcement-spam budget, the
visual-conformance verdicts, the current degraded state, and a release gate.

## Diagnosed surfaces

The report carries one diagnostics row for each protected custom-rendered surface family,
reusing the surface families and OS bridge taxonomy from the surface descriptors:

| Surface family | Row | Bridge | Object identity |
| --- | --- | --- | --- |
| `shell_region` | `diagnostics:shell_region` | ui_automation | `shell:zone-root` |
| `editor_canvas` | `diagnostics:editor_canvas` | ns_accessibility | `editor:active-buffer` |
| `terminal_canvas` | `diagnostics:terminal_canvas` | at_spi | `terminal:active-session` |
| `dense_collection` | `diagnostics:dense_collection` | ui_automation | `data-grid:active-view` |
| `notebook_cell` | `diagnostics:notebook_cell` | ns_accessibility | `notebook:active-cell` |
| `data_cell` | `diagnostics:data_cell` | at_spi | `data:active-cell` |
| `review_diff` | `diagnostics:review_diff` | ui_automation | `review:active-diff` |
| `overlay_sheet` | `diagnostics:overlay_sheet` | headless_inspector | `overlay:active-sheet` |

Each row binds to the **same object identity the descriptor and the visual surface carry**,
so the diagnostics can never drift from the object the user saw in-product.

## What each diagnostics row carries

- **A bridge probe** — `bridge_probe` records the OS accessibility `bridge_kind`, the
  current `bridge_state` (`bridged_active`, `partial`, `stale`, `unavailable`), the
  delivered `non_visual_fidelity`, an expected/present/missing `semantic_node_coverage`
  accounting, and a disclosed `degradation_reason`. A healthy bridge cannot carry a
  degradation reason or missing nodes; a degraded bridge must disclose both.
- **The full diagnostic battery** — `checks` is exactly one `diagnostic_check` per
  diagnostic class: `bridge_health`, `missing_semantic_node`, `announcement_rate`,
  `coalescing_violation`, `focus_return_failure`, `high_zoom_regression`,
  `high_contrast_regression`, `reduced_motion_regression`, and `label_or_role_drift`. Each
  check has an `outcome` (`pass`, `regressed`, `auto_narrowed`, `not_applicable`), a
  `severity` (`blocking` or `advisory`), a stable `diagnostic.`-prefixed
  `detail_message_id`, and an export-safe `evidence_ref`. The `focus_return_failure` check
  additionally exports the `focus_return_disposition` the surface fell back to, so a
  support bundle can read the focus-contract outcome exactly where it applies.
- **An announcement-spam budget** — `announcement_budget` pairs the grammar-owned
  coalescing budget (strategy, max announcements per window, window seconds, minimum
  interval) with the observed traffic and a `within_budget` verdict. The verdict must
  match the observed numbers, and an over-budget surface cannot show passing
  announcement/coalescing checks.
- **Visual-adaptation conformance** — `visual_conformance` declares the `high_zoom`,
  `high_contrast`, and `reduced_motion` outcomes; each mirrors the matching diagnostic
  check so a zoom/contrast/motion regression always lands in the gate-bearing check.
- **The current degraded state** — `current_degraded_state` mirrors the bridge probe
  (state, fidelity, reason) and a derived `is_degraded` flag, so support/export can
  disclose *why* a surface is degraded in the same vocabulary the user saw.
- **A per-surface release gate** — `gate` is a deterministic function of the blocking
  regressions: it `blocks` iff a `regressed` check with `blocking` severity is present,
  and it names exactly those classes. An advisory regression is recorded but never gates.
- **A reopenable durable fallback** — `durable_fallback` names the grammar-owned surface a
  user can reopen to recover the diagnostics, never relying on ephemeral narration alone.

## Announcement-spam budgets and the release gate

Each protected surface publishes an announcement-spam budget with a real coalescing
strategy and positive caps. When the observed traffic exceeds the budget, the
`announcement_rate` and `coalescing_violation` checks regress and the surface's gate
blocks. The report-level `release_gate` aggregates the per-surface gates: `blocks_release`
is true iff at least one surface is blocked, and `blocked_surface_ids` lists exactly those
surfaces. The release/public-truth automation reads `blocks_release` to fail rows for
bridge regressions, announcement spam, or zoom/contrast/motion breakage on protected
surfaces. Per the guardrail, this report never collapses the per-surface bridge and
announcement diagnostics into a single aggregate pass/fail dashboard — the gate is derived
*from* the per-surface findings, which stay individually exportable.

## Auto-narrowing on degraded bridge or stale proof

A surface whose OS accessibility bridge becomes unavailable auto-narrows: its
`bridge_health` and `missing_semantic_node` checks become `auto_narrowed`, its
qualification drops below Stable (for example to Beta), its non-visual fidelity drops to
`degraded_accessible`, the degraded state is disclosed, and it carries the matching
downgrade trigger (`bridge_unavailable` / `bridge_partial_or_stale`). The narrowing is a
disclosed claim change — the surface keeps every check and still ships at the narrowed
claim, so the release gate stays green. The
`bridge_unavailable_narrowed.json` fixture exercises this path; the three
`*_blocked.json` fixtures exercise unhandled blocking regressions that fail the gate.

## Controlled vocabulary reuse

The shared state vocabularies (`bridge_states`, `non_visual_fidelities`,
`coalescing_strategies`, `focus_return_dispositions`, `semantic_role_classes`, …) are
reused verbatim from the frozen dynamic-surface matrix through the
`shared_vocabulary_set` block; the protected surface families, OS bridge kinds, and
bridge-degradation reasons are reused from the surface descriptors; and the
coalescing-budget shape and durable-fallback surface tokens come from the announcement
grammar. The diagnostics-shaped vocabularies this lane adds — `diagnostic_class`,
`diagnostic_outcome`, `diagnostic_severity`, `visual_adaptation_mode`, and
`release_gate_decision` — are frozen in the `diagnostics_vocabulary_set` block.

## Support and export safety

Every diagnostic finding is carried by a stable `diagnostic.`-prefixed message id and an
export-safe evidence ref (an id, never a raw payload). The support export and AT
conformance packets can disclose bridge health, message ids, the exported focus-contract
disposition, and the current degraded state without leaking raw provider payloads,
credentials, secret material, screenshots, or untranslated free-text prose.

## Consumers

The shell surfaces the diagnostics health to the user; the editor, terminal, dense
collection, notebook, data, and review surfaces are diagnosed; support export reuses the
report; help/docs document the diagnostics packet; the release/public-truth automation
gates on it; and assistive-tech conformance packets reuse it. The `consumer_projection`
block records that every one of those consumers reads the same report.

## Regenerating the report

The seed builders in `aureline_shell::accessibility::diagnostics` are the single producer
of the checked-in support export and fixtures. Regenerate with the headless emitter:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- support-export \
  > artifacts/a11y/m5-dynamic-a11y-diagnostics/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- markdown \
  > artifacts/a11y/m5-dynamic-a11y-diagnostics/dynamic-a11y-diagnostics-proof.md
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- fixture-bridge-unavailable-narrowed \
  > fixtures/a11y/m5-bridge-and-announcement-drills/bridge_unavailable_narrowed.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- fixture-bridge-regression-blocked \
  > fixtures/a11y/m5-bridge-and-announcement-drills/bridge_regression_blocked.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- fixture-announcement-spam-blocked \
  > fixtures/a11y/m5-bridge-and-announcement-drills/announcement_spam_blocked.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_diagnostics -- fixture-visual-regression-blocked \
  > fixtures/a11y/m5-bridge-and-announcement-drills/visual_regression_blocked.json
```

The `checked_support_export_matches_seed` test fails if the checked-in export drifts from
the seed builder, so the artifact, the fixtures, and the in-code diagnostics stay in
lockstep.
