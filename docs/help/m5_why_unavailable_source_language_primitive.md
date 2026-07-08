# M5 Why-Unavailable / Source-Language Primitive

Two reusable M5 blocked-action / localized-help primitives implemented as one governed matrix:
the **why-unavailable explanation row** and the **source-language fallback surface**. Together they
close the gap between broader onboarding/help systems and the reusable rows a user actually hits
when an action is blocked or localized help is behind the canonical source.

- Crate module:
  `aureline_learning::implement_why_unavailable_explanation_rows_and_source_language_fallback_surfaces_with_owner_reason_next_safe_action_truth_and_citation_preserving_help_parity_across_claimed_m5_blocked_action_and_localized_surfaces`
- Schema: `schemas/ui/m5-why-unavailable-source-language.schema.json`
- Support export: `artifacts/release/m5-why-unavailable-source-language-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-why-unavailable-source-language-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-why-unavailable-source-language-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-why-unavailable-source-language-primitive/`

The two families narrow the frozen contextual-teaching / migration-bridge component matrix
(`freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`).
All acceptance vocabulary — blocked-action owners, unavailable reason classes, next-safe-action
classes, source-language classes, and fallback-state classes — is reused verbatim from that matrix
so no surface invents a parallel grammar.

## Family 1 — why-unavailable explanation row

`resolve_why_unavailable_explanation_row` takes a blocked action's reason class, owning boundary,
next-safe-action class, an optional next-safe-action target, a deeper-docs reference, a
screen-reader announcement, and a stable row identity, and produces the resolved row.

- **Posture** is derived one-to-one from the unavailable reason class
  (`blocked_by_policy` / `missing_permission` / `precondition_unmet` / `feature_disabled` /
  `offline_unavailable` / `unsupported_target`).
- **Failure domain** (`policy` / `trust` / `context` / `runtime`) is derived from the posture so
  context, trust, policy, and runtime failures never collapse into one generic disabled state
  (AC1).
- **Actions**: `take_next_safe_action` when the row names a concrete next step,
  `contact_blocking_owner` when the owner is reachable, `retry_when_resolved` when the block is
  transient (unmet precondition / offline), and always `open_deeper_docs` and
  `export_unavailable_evidence`.
- An actionable row with no concrete next-safe-action target is rejected
  (`MissingNextActionRefForActionableRow`); a `no_safe_action` row with a target is rejected
  (`NextActionRefOnNoSafeAction`) — so the row always names the next safe action or honestly states
  there is none.

## Family 2 — source-language fallback surface

`resolve_source_language_fallback` takes a surface's source-language class, fallback-state class,
display locale, stable ID, canonical citation, optional preserved source-language text, a
screen-reader announcement, and a stable row identity, and produces the resolved surface.

- **Posture** is derived one-to-one from the fallback-state class
  (`fully_localized` / `showing_source_language` / `partially_localized` / `stale_localization` /
  `citation_preserved_fallback` / `no_localization`).
- **Actions**: `view_source_language_text` when the source is shown, `report_translation_gap` when
  there is a gap, `request_localization` when there is no localization, and always
  `open_canonical_citation` and `export_locale_evidence`.
- Any surface that is not fully localized must carry the preserved source-language text; a fallback
  with none is rejected (`MissingSourceTextForFallback`). The stable ID and canonical citation are
  always preserved so localized flows stay aligned with canonical IDs and cited source material
  instead of drifting into unsourced paraphrase (AC2).

## Consumers and invariants

One row per claimed blocked-action / localized consumer — command-help row, menu-and-action row,
inline-status row, settings-and-docs row, and support explanation export — binds both shared
anatomies, the frozen vocabularies, the derived postures, the bounded actions, the export fields,
and the non-visual accessibility routes. Each row holds four hard invariants (all `false`):
`collapses_into_generic_disabled_state`, `hides_blocking_owner_or_reason`,
`severs_canonical_citation_or_id`, and `drifts_into_unsourced_paraphrase`.

Raw error dumps, stack traces, credentials, and private endpoints never cross the export boundary;
every reference is carried as an opaque, export-safe representation.

## Regenerating artifacts

```sh
BIN=aureline_learning_m5_why_unavailable_source_language_primitive
cargo run -q -p aureline-learning --bin $BIN -- support-export > artifacts/release/m5-why-unavailable-source-language-primitive-proof/support_export.json
cargo run -q -p aureline-learning --bin $BIN -- csv          > artifacts/release/m5-why-unavailable-source-language-primitive-proof/matrix.csv
cargo run -q -p aureline-learning --bin $BIN -- report       > artifacts/design/m5-why-unavailable-source-language-primitive.md
cargo run -q -p aureline-learning --bin $BIN -- fixture-menu-and-action-row-beta-narrowed              > fixtures/ui/m5-why-unavailable-source-language-primitive/menu_and_action_row_beta_narrowed.json
cargo run -q -p aureline-learning --bin $BIN -- fixture-support-explanation-export-preview-narrowed    > fixtures/ui/m5-why-unavailable-source-language-primitive/support_explanation_export_preview_narrowed.json
cargo run -q -p aureline-learning --bin $BIN -- validate
```
