# M5 glossary chips/cards and safe explanation banners

The glossary chip or card and the safe explanation banner are two of the six governed learning
components frozen by the
[M5 learning-component matrix](m5_learning_component_matrix.md). This lane implements those two
families as two co-equal control vectors in one export-safe packet,
[`GlossaryChipCardSafeExplanationBannerControlsPacket`](../../crates/aureline-learning/src/implement_glossary_chips_or_cards_and_safe_explanation_banners_with_cited_file_symbol_doc_truth_freshness_source_class_labels_and_explain_versus_do_separation_across_claimed_m5_learning_surfaces/mod.rs),
so a claimed M5 onboarding, guided-tour, glossary, learning-mode, or inline-help surface can
explain what a term means or why a result is suggested **without letting educational prose drift
away from cited source truth or quietly blur into an apply-capable action**: a learner can always
tell where a definition is cited from, how current that citation is, and whether an explanation
only explains or also offers a governed do.

## What the resolvers decide

The module has two derived resolvers so the honesty of each control is computed, never asserted.

### `resolve_glossary_citation`

Given a glossary control's citation state, the resolver derives a **citation class**:

- `citation_current` / `citation_versioned` → `cited_current`
- `citation_stale` → `cited_stale` (must carry an explicit stale note), never current
- `citation_cached` → `cited_cached`
- `citation_offline_unavailable` → `offline_unavailable` (must carry an explicit offline note)
- `citation_missing` → `uncited` (must carry an explicit missing-citation note)

A learner can therefore always tell **how grounded a definition is**; a stale, offline, or
missing citation can never read as current.

### `resolve_explanation_apply`

Given a banner's apply state, the resolver derives an **apply disposition**:

- `no_apply` → `explain_only`
- `preview_available` → `preview_offered`
- `approval_pending` → `approval_pending`
- `applied_with_undo` → `applied_reversible` (must carry an explicit undo note)
- `blocked_apply` / `mutation_declined` → `apply_withheld` (must carry an explicit withheld note)

An explanation that has not actually applied anything can never read as having done so, and any
real apply stays **reversible and governed** rather than a hidden mutation.

## Cited file / symbol / docs truth and source-class labels

- **Term meaning** — every glossary control names what the term means, and every banner names a
  grounded explanation body ("what this term means" / "why this result is suggested").
- **Cited source** — every control points at a stable cited source: a `command_reference`,
  `file_location`, `symbol_location`, or `docs_anchor` reference with a human-readable citation
  label, so a definition is backed by source truth rather than free-floating prose.
- **Source class and freshness** — every glossary control shows its source class (`cited_docs`,
  `cited_spec`, `cited_help_pack`, `community_note`, `uncited_draft`, `unknown_source`) and its
  citation freshness. A control drawing on a non-cited source can never claim to rest on cited
  source truth, and it must disclose that it is not cited.
- **Open related concept** — every glossary control offers the mandatory `open_related_concept`
  action so a learner can always follow the concept graph.

## Explain-versus-do separation

- **Explicit boundary** — every banner names its explain-versus-do boundary, so a learner always
  knows whether it only explains or also offers a governed do.
- **Explain-only banners never do** — an `explain_only` banner offers no `preview_change` or
  `request_approval` action, and its apply state can never run ahead of what the boundary permits.
- **Governed apply** — when a banner offers a do, it stays behind the ordinary preview / approval
  / undo model; nothing is applied by a hidden authority. `Why this result is suggested` and
  `What this term means` never imply an apply-capable action.
- **Distinct from apply-capable actions** — educational glossary and explanation surfaces stay
  visibly distinct from apply-capable AI or command actions across every claimed M5 consumer.

## Hard invariants

Both control vectors share five hard invariants, each of which MUST be `false`:

- `masks_privacy_or_offline_state`
- `hides_citation_source_or_freshness`
- `implies_apply_capable_action_or_hidden_authority`
- `invents_alternate_state_label`
- `drifts_prose_from_cited_source_truth`

Any control that trips one of these is rejected by
`GlossaryChipCardSafeExplanationBannerControlsPacket::validate`.

## Coverage

The canonical packet covers, across its six glossary chips/cards, every derived citation class,
every glossary source class, and every glossary citation state; and, across its six safe
explanation banners, every derived apply disposition, every explanation boundary class, and every
explanation apply state. Two narrowed scenario fixtures spotlight an uncited glossary chip that
never reads as cited and an explain-only banner that never implies an apply-capable action.

## Emitting and validating

The headless emitter
`aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive` is the single
mint-from-truth path for the checked-in support export, matrix CSV, design report, and fixtures:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive -- support-export
cargo run -q -p aureline-learning --bin aureline_learning_m5_glossary_chip_card_safe_explanation_banner_primitive -- validate
```

The export, CSV, report, and fixtures live under
`artifacts/release/m5-glossary-chip-card-safe-explanation-banner-proof/`,
`artifacts/design/m5-glossary-chip-card-safe-explanation-banner.md`, and
`fixtures/ui/m5-glossary-chip-card-safe-explanation-banner-controls/`. All of them validate
against
[`schemas/ui/m5-glossary-chip-card-safe-explanation-banner-controls.schema.json`](../../schemas/ui/m5-glossary-chip-card-safe-explanation-banner-controls.schema.json).
