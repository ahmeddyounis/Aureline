# Refactor Preview Beta Contract

This document records the launch-language refactor evidence lane in
`aureline-language::refactor_preview`.

## Runtime Records

`RefactorPreviewRecord` captures:

- the refactor class, launch-language row, runtime condition, confidence tier,
  and semantic source class;
- the affected file, symbol, reference, generated-file, dependency, and omitted
  scope counts;
- an explicit fallback or source label whenever the target set is partial,
  cached, remote-assisted, textual, policy-limited, or not qualified;
- generated-artifact and dependency-impact notes;
- a validation summary with rerun hooks; and
- rollback refs that reuse local-history checkpoints and mutation-journal
  grouped entries for any applyable multi-file preview.

`RefactorValidationResult` states whether the row is `green`, `downgraded`, or
`unsupported`, and separately validates preview truth, fallback labeling,
rollback lineage, support-export visibility, and rollback-drill outcome.

## Corpus

The protected corpus lives at
`fixtures/language/refactor_preview_and_rollback/` and covers:

- warm semantic rename;
- partial-index extract;
- cached semantic move;
- remote-assisted import update;
- generated-limited cross-file signature change; and
- policy-limited text fallback for an unsupported semantic move.

Green rows may only use current or qualified remote semantic evidence. Textual
fallback rows are never green and must stay visibly downgraded or unsupported.

## Verification

Run:

```sh
cargo test -p aureline-language --test refactor_preview_beta
cargo test -p aureline-support --test refactor_preview_support_export
python3 -m json.tool schemas/language/refactor_preview.schema.json >/dev/null
python3 -m json.tool schemas/language/refactor_validation_result.schema.json >/dev/null
```
