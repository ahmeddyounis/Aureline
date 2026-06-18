# Recipe-builder, parameter-review, dry-run/explain, run-history, macro-recorder, and safety-label contract

This document freezes the canonical M5 automation **authoring, preview, history,
and macro** object model. The recorded-macro / declarative-recipe manifest and
the run-record contracts already freeze *what a stored recipe or macro is* and
*what evidence a dispatch mints*
([`docs/automation/recipe_and_macro_contract.md`](../automation/recipe_and_macro_contract.md)).
This contract closes the remaining UI/runtime gap: the object model for actually
**building, reviewing, previewing, rerunning, and exporting** automation across
M5 surfaces, so later palette, docs/help, export, and support work inherits one
inspected builder / preview / history / macro / safety contract rather than
inventing feature-local runners.

The central rule is unchanged from the recorded-macro / recipe contract:
automation is **declarative first**. The recipe builder emits declarative recipe
manifests only; the macro recorder is constrained to explicit UI or editor state;
reruns re-resolve current context and never replay stale authority; and no M5
surface widens automation authority through a hidden UI shortcut or stale
history.

## Companion artifacts

- [`/schemas/automation/recipe-builder.schema.json`](../../schemas/automation/recipe-builder.schema.json)
  — boundary schema for the `recipe_builder_session_record`,
  `parameter_review_sheet_record`, `dry_run_explain_packet_record`, and
  `automation_safety_label_manifest_record` shapes.
- [`/schemas/automation/macro-session.schema.json`](../../schemas/automation/macro-session.schema.json)
  — boundary schema for the `macro_session_record` shape.
- [`/schemas/automation/automation-contract-baseline.schema.json`](../../schemas/automation/automation-contract-baseline.schema.json)
  — boundary schema for the canonical baseline packet that binds the six object
  families, the reused safety-label vocabulary, and the freeze invariants.
- [`/artifacts/m5/automation/automation-contract-baseline/`](../../artifacts/m5/automation/automation-contract-baseline/)
  — the checked-in baseline packet, support export, CLI/headless view,
  safety-label manifest, and compact projection.
- [`/fixtures/automation/m5/recipe-macro/`](../../fixtures/automation/m5/recipe-macro/)
  — worked example records for a preview-ready and a blocked builder session, a
  parameter-review sheet, a dry-run/explain packet, and a stopped/promotable and
  a discarded macro session.
- [`/fixtures/automation/m5/automation-contract-baseline/`](../../fixtures/automation/m5/automation-contract-baseline/)
  — baseline mutation cases that prove the fail-closed gate.
- [`/tools/ci/m5/automation_contract_baseline_check.py`](../../tools/ci/m5/automation_contract_baseline_check.py)
  — the fail-closed CI gate over the artifacts and fixtures.

The Rust types in
`crates/aureline-runtime/src/m5_automation_contract_baseline` are the schema of
record; the headless inspector
`crates/aureline-runtime/examples/dump_m5_automation_contract_baseline.rs`
regenerates every artifact and fixture from the seed so they are bit-for-bit
derivable.

## Cross-linked contracts already in the repository

- [`docs/automation/recipe_and_macro_contract.md`](../automation/recipe_and_macro_contract.md)
  and
  [`schemas/automation/recipe_manifest.schema.json`](../../schemas/automation/recipe_manifest.schema.json)
  — the builder emits a `recipe_manifest_record` and the recorder emits a
  `recorded_macro_manifest_record` against this contract. The authoring-language,
  capability, step-kind, surface-class, and posture vocabularies originate there.
- [`docs/automation/run_history_contract.md`](../automation/run_history_contract.md)
  and
  [`schemas/automation/run_history_row.schema.json`](../../schemas/automation/run_history_row.schema.json)
  — the run-history family binds this existing row contract by reference; it is
  not re-invented here.
- [`schemas/automation/run_record.schema.json`](../../schemas/automation/run_record.schema.json)
  — every dispatch and dry-run preview projects through the canonical run record.
- [`docs/automation/preview-and-lifecycle.md`](../automation/preview-and-lifecycle.md)
  and
  [`schemas/automation/automation-manifest.schema.json`](../../schemas/automation/automation-manifest.schema.json)
  — the `controlled_automation_label` vocabulary the safety labels reuse
  originates there. This contract reuses it; it does not mint parallel labels.
- [`schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json)
  and
  [`schemas/commands/shareability_metadata.schema.json`](../../schemas/commands/shareability_metadata.schema.json)
  — every builder step and parameter row cites a command descriptor and reuses
  the argument-inspection and shareability vocabularies.

## The six object families

The baseline binds exactly six families. Each carries a boundary schema, a doc
anchor, a closed state vocabulary, the evidence hooks its records resolve
through, and the consumer surfaces that read it.

| Family | Record | Schema |
|---|---|---|
| Recipe builder | `recipe_builder_session_record` | `recipe-builder.schema.json` |
| Parameter review | `parameter_review_sheet_record` | `recipe-builder.schema.json` |
| Dry-run / explain | `dry_run_explain_packet_record` | `recipe-builder.schema.json` |
| Run history | `automation_run_history_row` | `run_history_row.schema.json` |
| Macro recorder | `macro_session_record` | `macro-session.schema.json` |
| Safety labels | `automation_safety_label_manifest_record` | `recipe-builder.schema.json` |

### Recipe builder

The recipe-builder session is the live authoring state. Its
`builder_state_class` is one of `draft`, `validation_failed`, `preview_ready`,
`approval_required`, or `blocked`. Every step draft cites a `command_id` and a
`command_revision_ref` resolvable against the command descriptor; the builder
emits a **declarative** manifest only (`manifest_target_schema_ref` is the recipe
manifest schema). A draft that cites a `ui_only` command is blocked with a
blocking validation finding rather than silently producing an inadmissible
recipe.

### Parameter review

The parameter-review sheet resolves every argument's provenance before apply.
Each row carries an `inspection_kind` re-exported from the shareability contract,
a `verdict_class` (`resolved`, `needs_input`, `policy_pinned`,
`sensitive_held_for_review`, `blocked`), a `required` flag, and a
`sensitivity_class`. Apply is blocked while `unresolved_required_count` is
greater than zero. The sheet carries reviewable summaries, never raw values.

### Dry-run and explain

The dry-run / explain packet explains each step before any apply. Its
`dry_run_outcome_class` is one of `would_apply`, `would_apply_under_approval`,
`would_be_denied_at_gate`, or `no_safe_preview`. Each step explanation names the
capabilities it touches, the safety labels it projects, whether the effect is
reversible, and a blast-radius summary. Every dispatch mints a run record against
`run_record.schema.json`.

### Run history

The run-history family binds the existing
[`automation_run_history_row`](../automation/run_history_contract.md) contract by
reference. Its state vocabulary is the automation-layer vocabulary
(`recorded_macro_layer`, `declarative_recipe_layer`,
`managed_only_template_layer`, `extension_or_external_automation_layer`,
`headless_safe_run_layer`). Rerun-under-current-policy re-resolves current
context; it never replays a cached approval or a stale environment.

### Macro recorder

The macro-recorder session is the deliberately narrow surface. Its
`recorder_state_class` is one of `recording`, `paused`, `stopped`, `discarded`,
or `promoted_to_recipe`. Every captured step is strictly constrained to a
`recorded_macro_surface_class` (UI or editor state); the projected safety labels
are limited to `macro_safe` and `ui_only`; ambient network / process / secret
capture is forbidden mechanically; and the macro is never admissible on the
managed-only channel. A stopped session mints a recorded-macro manifest; a
discarded session mints none.

### Automation safety labels

The safety-label vocabulary is the single reuse surface every M5 builder,
preview, history, palette, CLI, AI, and support projection reads. It reuses the
`controlled_automation_label` vocabulary frozen in
[`automation-manifest.schema.json`](../../schemas/automation/automation-manifest.schema.json);
it does not mint parallel labels.

| Label | Display | Kind |
|---|---|---|
| `macro_safe` | Macro-safe | admissibility cue |
| `recipe_safe` | Recipe-safe | admissibility cue |
| `headless_safe` | Headless-safe | admissibility cue |
| `ui_only` | UI-only | admissibility cue |
| `approval_required` | Approval required | admissibility cue |
| `writes_files` | Writes files | effect disclosure |
| `runs_process` | Runs process | effect disclosure |
| `network_call` | Network call | effect disclosure |
| `remote_mutation` | Remote mutation | effect disclosure |

The first five are **admissibility cues** (where automation may run); the last
four are **effect disclosures** (what automation does). Each label projects from
the controlled-automation-label axis and carries a stable display token and a
reviewable meaning sentence.

## Freeze invariants

The baseline packet pins these invariants as schema-level constants in its
`invariants` block. A false value is non-conforming; the block is how this
contract freezes its MUST rules mechanically.

1. `recipe_builder_emits_declarative_manifests_only`
2. `macro_recorder_constrained_to_ui_or_editor_state`
3. `dry_run_explain_required_before_irreversible_apply`
4. `parameter_review_resolves_provenance_before_apply`
5. `one_safety_label_vocabulary_reused_across_surfaces`
6. `safety_labels_project_from_existing_axes_not_minted`
7. `run_history_reuses_the_canonical_run_record`
8. `no_hidden_ui_shortcut_widens_automation_authority`
9. `reruns_reresolve_current_context_never_replay_stale_authority`

## How the freeze is enforced

[`/tools/ci/m5/automation_contract_baseline_check.py`](../../tools/ci/m5/automation_contract_baseline_check.py)
is the fail-closed gate. It blocks stable when a family is dropped, a family
declares no schema / evidence hook / consumer surface / state vocabulary, the
safety-label set is incomplete or miscategorized, the reused-contract refs are
dropped, or an invariant is violated. The baseline mutation fixtures under
[`/fixtures/automation/m5/recipe-macro/`](../../fixtures/automation/m5/recipe-macro/)'s
sibling `automation-contract-baseline` directory each reproduce one of those
blocking states, and the typed Rust consumer mints the identical packet so
`cargo test -p aureline-runtime --test m5_automation_contract_baseline` enforces
the same invariants.

## Schema of record

The Aureline automation runtime's Rust types are the schema of record. The JSON
Schema exports are the cross-tool boundary every non-owning surface reads. Adding
a new enum value to a frozen vocabulary is additive-minor and bumps the relevant
`_schema_version` const; repurposing an existing value is breaking and requires a
new decision row.

## Source anchors

- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md) — power-user
  automation requirements, declarative-first posture, CLI/headless rules.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md)
  — safe-automation matrix, recipe / macro architecture.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  — command invocation / session / result contracts, recipe and macro objects.
- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — recipe builder, parameter review, dry-run/explain, run history, and macro
  recorder UX.
