# M5 Adapter-Confidence-Chip and Discovery-Diff-Card Controls

- Packet: `m5-adapter-confidence-chip-discovery-diff-card-controls:stable:0001`
- Label: `M5 adapter-confidence-chip and discovery-diff-card controls with adapter/source class, confidence band, heuristic-vs-structured-vs-imported discovery mode, current downgrade reason, previous-vs-current target identity, changed certainty, review-before-switch, and no-higher-confidence-overwrite truth`
- Consumer surfaces: 5
- Certainties: exact, compatible, heuristic, imported, downgraded, stale
- Proof freshness SLO: 168 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **run_test_debug_ui**: `stable`
  - Owner: Run/test/debug surface owner
  - Scope: Every run, test, and debug target renders an adapter-confidence chip naming the adapter/source class, confidence band, and heuristic-vs-structured-vs-imported discovery mode before the user invokes; a discovery-diff card names the previous and current target with an attributable review state and never silently relabels a material change
  - Chip examples: 3 / card examples: 2
- **preview_ui**: `stable`
  - Owner: Preview surface owner
  - Scope: Preview targets reuse the same confidence chip vocabulary, naming the compatible or imported discovery mode and degrading honestly when the confidence band is unstated; the discovery-diff card names the changed certainty rather than hiding a material change
  - Chip examples: 3 / card examples: 2
- **companion_ui**: `stable`
  - Owner: AI tool-routing owner
  - Scope: AI tool routing reads the same adapter-confidence chip so a downgraded target attributes its current downgrade reason before the model runs, debugs, or hands off work; the discovery-diff card keeps a higher-confidence resolved target instead of letting a weaker heuristic overwrite it without review
  - Chip examples: 2 / card examples: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved chip and card truth, so a stale target's attributed downgrade reason, an unattributed downgrade, or an undisclosed target identity is visible in evidence rather than hidden behind feature-local prose
  - Chip examples: 2 / card examples: 1
- **product_ui**: `stable`
  - Owner: In-product surface owner
  - Scope: In-product surfaces reuse the same confidence and discovery-drift vocabulary the run/test/debug surface shows, keeping the language consistent across shell, notebook, and companion so an exact target reads as exact and a reviewed switch reads as reviewed everywhere
  - Chip examples: 1 / card examples: 1
