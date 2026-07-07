# M5 local-history / write-scope component consumers (M05-897)

The **adoption lane** over the frozen M5 local-history / write-scope component matrix. It
proves the seven governed component families are reusable components — not one local-history
timeline plus a few isolated recovery objects — by binding every claimed M5 mutation /
recovery consumer to the same canonical component schemas and the same descriptor vocabulary,
so **checkpoint, rollback, restore, and export** language stays aligned across surfaces even
when the surrounding workflow differs.

This closes the B105 consumer-adoption lane over the frozen local-history / write-scope
component matrix (`schemas/ui/m5-local-history-write-scope-component-matrix.schema.json`), layered
on top of the four `implement_*` / `ship_*` primitive lanes (M05-893 … M05-896) that narrowed the
matrix families into working resolvers.

- Module:
  `crates/aureline-history/src/add_shared_rename_refactor_replace_import_repair_generated_artifact_and_ai_review_consumers_so_local_history_and_write_scope_components_keep_checkpoint_rollback_language_aligned_across_claimed_m5_mutation_surfaces`
- Emitter bin: `aureline_history_local_history_write_scope_component_consumers`
- Schema: `schemas/ui/m5-local-history-write-scope-component-consumer.schema.json`
- Support export: `artifacts/release/m5-local-history-write-scope-component-consumer-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-local-history-write-scope-component-consumer-proof/matrix.csv`
- Report: `artifacts/release/m5-local-history-write-scope-component-consumer-proof/report.md`
- Fixtures: `fixtures/ui/m5-local-history-write-scope-component-consumers/`

## Consumers

Seven claimed M5 mutation / recovery consumers adopt the shared components:

| Consumer | Adopted families (examples) |
| --- | --- |
| **Editor Rename / Refactor** | write-scope preview tree, local-history row, checkpoint-group card, restore-preview card |
| **Replace In Files** | write-scope preview tree, checkpoint-group card, restore-granularity selector |
| **Import / Migration Session** | local-history row, write-scope preview tree, restore-granularity selector |
| **Repair Transaction** | checkpoint-group card, restore-granularity selector, restore-preview card |
| **Generated-Artifact Provenance** | local-history row, retention/export card, write-scope preview tree |
| **AI Apply / Review** | checkpoint-group card, write-scope preview tree, restore-preview card, history-export manifest |
| **Support / Export Desk** | retention/export card, history-export manifest, local-history row, restore-preview card |

Each family is adopted by **at least two** distinct consumers (the acceptance-criterion proof
of reuse). The support / export desk is singled out for a canonical-schema reference so a
support / export lane's prose can never drift from the product truth.

## The shared descriptor vocabulary

Every binding surfaces the four required descriptors — **checkpoint**, **rollback**,
**restore**, and **export** — read from a single canonical source. A consumer never re-words
these per surface and never invents a second recovery grammar.

## Resolver — `resolve_history_binding`

Takes one consumer's adoption of one component family, the descriptor set it surfaces, the
parity-health mode it renders under, and any export caveats. It:

1. Rejects an empty descriptor set, a missing required descriptor, or a note that carries
   forbidden material.
2. Derives the **claim-parity state**: `claims_preserved` at full parity, `claims_auto_narrowed`
   under any weakened parity-health mode.
3. Whenever parity is weakened, emits a **self-contained auto-narrow banner** naming the exact
   reason, the descriptors that stay preserved, the export caveats, and the recovery action —
   never a generic "degraded" note.

### Parity-health modes → reasons → recovery actions

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `full_parity` | — | — | — |
| `preview_only_narrowed` | `preview_only_workflow` | `return_to_recovery_center_to_commit` | `restore_commit_disabled_preview_only` |
| `external_drift_narrowed` | `external_drift_unreconciled` | `reconcile_external_drift_first` | `scope_uncertain_until_drift_reconciled` |
| `generated_managed_narrowed` | `generated_or_managed_scope` | `regenerate_from_source_instead` | `generated_file_restore_caveated` |
| `export_redacted_narrowed` | `export_redaction_applied` | `request_unredacted_export` | `export_redacted_not_full_evidence` |

The narrowed rendering keeps the full descriptor vocabulary; only the claim is narrowed, so a
consumer that cannot preserve parity is **visibly narrowed rather than inheriting stronger
labels from healthier recovery lanes**.

## Canonical family → primitive mapping

Each family points at the narrowed primitive that owns it, never a local re-description:

| Family | Canonical schema |
| --- | --- |
| `local_history_row`, `checkpoint_group_card` | `schemas/ui/m5-local-history-row-and-checkpoint-group-card.schema.json` (M05-893) |
| `restore_preview_card`, `restore_granularity_selector` | `schemas/ui/m5-restore-preview-card-and-restore-granularity-selector.schema.json` (M05-894) |
| `write_scope_preview_tree` | `schemas/ui/m5-write-scope-preview-tree.schema.json` (M05-895) |
| `retention_export_card` | `schemas/ui/m5-retention-export-card.schema.json` (M05-896) |
| `history_export_manifest` | `schemas/ui/m5-history-export-manifest.schema.json` (M05-896) |

## First-consumer compatibility notes

- **Editor rename / refactor**, **replace-in-files**: full parity on the write-scope preview
  tree and checkpoint-group card; the restore-granularity selector auto-narrows under
  unreconciled external drift.
- **Import / migration session**: auto-narrows the restore-granularity selector under external
  drift; held at Preview in the narrowed fixture pending drift-reconciliation parity across
  every imported migration scope.
- **Repair transaction**, **AI apply / review**: the restore-preview card auto-narrows to
  preview-only because the review cannot commit the restore in place.
- **Generated-artifact provenance**: the write-scope preview tree auto-narrows under a
  generated / managed scope — a generated file's restore is caveated (regenerate from source).
- **Support / export desk**, **AI apply / review**: the history-export manifest auto-narrows
  under an applied export redaction (a redacted share, not full recovery evidence); the AI
  apply / review surface is held at Beta in the narrowed fixture pending banner parity on every
  export-redacted path.

## Governance

Every consumer adopts the shared primitives, references the canonical schema, keeps the
descriptor vocabulary shared (never re-worded), invents no new recovery grammar, and declares a
non-visual accessibility route. Later M5 rows cannot invent parallel consumer-adoption
vocabulary. Raw file bodies, raw paths, credentials, and external endpoints never cross the
support boundary; every label is carried only as an opaque, export-safe representation.
