# Mutation class matrix and reversal-policy contract

This document publishes the mutation-class matrix and reversal-policy
contract Aureline uses to keep preview, approval, journaling, reversal,
and audit behavior coherent across every write path.

It exists so the editor, VFS, refactor engine, AI apply flow, build and
preview lanes, and publish/external integrations do not each invent
bespoke semantics for “what kind of write was this?” or “what does undo
mean here?”.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this document
must be updated in the same change.

## Companion artifacts

- [`/artifacts/change/mutation_classes.yaml`](../../artifacts/change/mutation_classes.yaml)
  — machine-readable mutation-class matrix plus reversal-policy rules.
- [`/fixtures/change/mutation_class_examples/`](../../fixtures/change/mutation_class_examples/)
  — worked examples covering AI apply, refactor apply, save-as, generated
  output refresh, publish actions, and external writes.
- [`/schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json)
  — mutation-journal entry schema (`reversal_class`, `preview_ref`,
  `approval_ref`, checkpoints, and target identity vocabulary).
- [`/artifacts/architecture/undo_class_rows.yaml`](../../artifacts/architecture/undo_class_rows.yaml)
  — ADR-0003 undo-class taxonomy and compensation posture.
- [`/docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md)
  — mutation-journal and generated-artifact lineage model that consumes
  the same reversal-class axis.

## Scope

In scope:

- the closed set of mutation classes used to classify every mutation;
- per-class expectations for:
  - required journal fields and identity expression;
  - preview minimum (what must be shown before apply);
  - approval posture expectations (when approvals are required);
  - reversal posture and downgrade rules; and
  - support-export metadata requirements under redaction.

Out of scope:

- implementing mutation engines, preview renderers, or journaling backends;
- defining UI copy; UI wording belongs to the UI / UX spec and style guide;
- defining new schemas. This contract uses existing journal, command, and
  identity schemas and freezes a shared matrix over them.

## Shared vocabulary

### Mutation class (`mutation_class`)

Mutation classes are a *high-level family label* answering “what kind of
authority changed?”. A single user command may emit multiple mutation
journal entries (for example, a semantic refactor that produces buffer
edits plus filesystem renames); each entry is classified independently
and then grouped under a `group_id`.

Closed set (stable ids):

- `buffer_text`
- `filesystem`
- `semantic_tooling`
- `generated_state`
- `external_effect`

These ids are defined in
[`/artifacts/change/mutation_classes.yaml`](../../artifacts/change/mutation_classes.yaml).

### Reversal class (`reversal_class`)

Reversal class answers “what can the user actually get back?”. It is
orthogonal to ADR-0003 `undo_class`.

Closed set:

- `exact_undo`
- `compensating_undo`
- `regenerate_or_recompute`
- `restore_from_checkpoint`
- `audit_only`

This vocabulary is exported by
[`/schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json).

## Summary matrix (human-readable)

| Mutation class | Typical examples | Default reversal posture |
|---|---|---|
| `buffer_text` | typing, paste, multi-cursor edit, in-buffer formatting | `exact_undo` |
| `filesystem` | rename/move/create/delete, save-as, VFS save coordination | `exact_undo` (downgrades on drift) |
| `semantic_tooling` | refactor apply, AI patch apply, scaffolding step, review apply | `restore_from_checkpoint` |
| `generated_state` | lock refresh, codegen, notebook outputs, preview refresh | `regenerate_or_recompute` |
| `external_effect` | push/publish, remote apply, external API write | `audit_only` |

The detailed, machine-readable contract lives in
[`/artifacts/change/mutation_classes.yaml`](../../artifacts/change/mutation_classes.yaml).

## Per-class contract highlights

This section is a *summary* of the full matrix. Producers and reviewers
should treat the artifact as the source of truth.

### `buffer_text` — Buffer/text mutation

- **Target identity:** `target_refs[]` includes a `buffer` target with a
  filesystem identity record. `affected_range` is emitted when available.
- **Preview minimum:** no preview required for interactive input; bulk
  operations still follow command preview policy and must surface
  `reversal_class`.
- **Approval posture:** not required by default; policy-gated buffers may
  require approvals.
- **Reversal posture:** `exact_undo` by default; downgrade visibly if
  authority drift or identity mismatch is detected.

### `filesystem` — Filesystem mutation

- **Target identity:** `target_refs[]` includes `filesystem_object` with
  canonical filesystem identity; `save_manifest_ref` is emitted when the
  durable write reaches the save pipeline.
- **Preview minimum:** destructive or broad-scope filesystem actions
  require preview showing canonical identity, affected scope, drift/conflict
  state (when applicable), and `reversal_class`.
- **Approval posture:** policy-driven; trust boundaries and destructive
  scope commonly require approvals.
- **Reversal posture:** `exact_undo` when identity + authority state match;
  downgrade to `compensating_undo` or `restore_from_checkpoint` on drift.

### `semantic_tooling` — Semantic/tooling mutation

- **Target identity:** `target_refs[]` covers every touched object; multi-
  target operations use `group_id`.
- **Preview minimum:** preview required. Must show scope summary, diffs or
  structured change summary, producer attribution, and declared reversal.
- **Approval posture:** policy-driven; AI apply and privileged scope are
  common approval gates.
- **Reversal posture:** defaults to `restore_from_checkpoint` (especially
  for only-revertible undo classes); exact reversal is allowed only when
  the inverse is deterministic and the authority state has not drifted.

### `generated_state` — Generated-state mutation

- **Target identity:** `target_refs[]` includes `generated_artifact` with
  filesystem identity; `generated_artifact_lineage_ref` is mandatory for
  durable generated writes.
- **Preview minimum:** when user-visible or durable, preview must disclose
  generator identity/version (by reference), input digest set, drift state,
  and reversal posture.
- **Approval posture:** not required for local regeneration by default, but
  required under trust boundary crossings or when overwriting user-authored
  overrides.
- **Reversal posture:** defaults to `regenerate_or_recompute`. Do not claim
  `exact_undo` unless a checkpoint-based restore is the declared posture.

### `external_effect` — External-effect mutation

- **Target identity:** `target_refs[]` includes `external_service` with a
  logical ref; `side_effect_summary.external_targets` is present and
  matches the target identity.
- **Preview minimum:** explicit target confirmation and side-effect scope
  disclosure are mandatory; approval posture must be explicit.
- **Approval posture:** required by default (policy may allow explicit
  narrow exceptions for inert metadata-only operations).
- **Reversal posture:** defaults to `audit_only`. Compensating reversals are
  allowed only when an explicit compensating flow exists and is declared.

## Reversal-policy contract (selection rules)

The reversal class is a *claim*. It must remain honest under drift:

- Producers MAY claim `exact_undo` only when the inverse is deterministic
  against the same authority state and target identity.
- Producers MAY claim `compensating_undo` only when a bounded compensating
  command exists and drift/partial effect can be detected and disclosed.
  Compensating reversals must not auto-run under drift or policy-epoch
  change.
- Producers MUST use `regenerate_or_recompute` for derived artifacts whose
  canonical authority is generator + inputs, not the current bytes.
- Producers MUST use `restore_from_checkpoint` when the honest reversal
  posture is snapshot-based (for example, only-revertible multi-file
  applies) and a durable checkpoint exists.
- Producers MUST use `audit_only` when the primary effect is external and
  no compensating flow is declared.

If authority state drifts after commit and the original reversal posture
is no longer true, the producer must downgrade visibly before executing a
reversal (and record the downgrade reason in the mutation journal).
