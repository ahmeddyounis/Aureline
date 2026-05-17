# Recorded macro alpha — bound to the command graph, mode state, and trust policy

This page is the reviewer-facing landing for the recorded-macro alpha
record family. It freezes how recorded macros project into the shell as
**governed automation** instead of "keystroke folklore": every macro
step resolves to a stable `command_id` on the command graph, every step
pins the editor / palette / shell mode the replay MUST observe, and
every definition projects the workspace-trust posture it requires.

Companion artifacts:

- [`/schemas/commands/recorded_macro.schema.json`](../../../schemas/commands/recorded_macro.schema.json)
  — boundary schema for the alpha record family. Every page, definition,
  replay disposition, audit event, and attribution row is one record
  against this schema.
- [`/crates/aureline-shell/src/macros/mod.rs`](../../../crates/aureline-shell/src/macros/mod.rs)
  — Rust types, validator, and redaction-safe support-export
  projection.
- [`/fixtures/commands/recorded_macro_alpha/page.json`](../../../fixtures/commands/recorded_macro_alpha/page.json)
  — protected reviewer fixture covering proceed, preview-required,
  downgrade, promote-to-recipe, and denied replay lanes plus the
  matching audit-event stream and support / activity / admin-audit
  attribution rows.
- [`/crates/aureline-shell/tests/recorded_macro_alpha.rs`](../../../crates/aureline-shell/tests/recorded_macro_alpha.rs)
  — integration test that loads the fixture and asserts the validator
  invariants and required coverage.

Cross-linked contracts already in the repository:

- [`/docs/automation/recipe_and_macro_contract.md`](../../automation/recipe_and_macro_contract.md)
  — the upstream boundary that already names recorded macros as the
  deliberately narrow, UI / editor-state-only authoring shape and
  declarative recipes as the only form admitted on the managed-only
  channel. This alpha is the shell-side projection of that contract —
  it never forks the recipe / macro vocabulary.
- [`/schemas/commands/command_descriptor.schema.json`](../../../schemas/commands/command_descriptor.schema.json)
  — every step in a definition cites a stable `command_id` and
  `command_revision_ref` resolvable against this descriptor.
- [`/schemas/commands/shareability_metadata.schema.json`](../../../schemas/commands/shareability_metadata.schema.json)
  — `macro_safe`, `recipe_safe`, `ui_only_interactive`, and
  `irreversible_high_blast_denied_for_automation` cues; a step whose
  shareability row denies macro admission is non-conforming.
- [`/schemas/automation/recipe_manifest.schema.json`](../../../schemas/automation/recipe_manifest.schema.json)
  — promoted-to-recipe dispositions cite a `promoted_recipe_ref` on the
  recipe-manifest contract.
- [`/schemas/automation/run_record.schema.json`](../../../schemas/automation/run_record.schema.json)
  — every replay attempt mints a run record there; the disposition row
  on this alpha is the boundary the shell consumes before the run
  record is published.
- [`/artifacts/security/trust_state_matrix.yaml`](../../../artifacts/security/trust_state_matrix.yaml)
  — the trust-state matrix this alpha's `trust_gate_class` projects
  through.

## What the lane freezes

| Axis                              | Closed vocabulary                                                                                                                                       |
|-----------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| Step command-graph lineage        | `core_command`, `imported_command`, `extension_command`, `ai_tool_handle`, `cli_verb`, `unmapped_keystroke_denied`                                       |
| Step mode requirement             | `editor_normal_mode_required`, `editor_insert_mode_required`, `editor_visual_mode_required`, `editor_any_mode_admissible`, `palette_mode_required`, `terminal_mode_denied` |
| Step write class                  | `read_only`, `editor_buffer_mutation`, `editor_multi_file_mutation`, `settings_mutation_denied`, `network_mutation_denied`, `process_mutation_denied`, `credential_mutation_denied` |
| Definition replay limitation      | `single_buffer_safe`, `multi_buffer_requires_preview`, `crosses_workspace_boundary_requires_recipe_promotion`, `non_replayable_unstable_timing`, `denied_unmapped_command_present` |
| Definition trust gate             | `restricted_workspace_admissible`, `trusted_workspace_required`, `admin_policy_observed`, `managed_only_denied`                                          |
| Replay disposition                | `proceed_local_editor_only`, `preview_required_before_apply`, `downgraded_to_observer_no_mutation`, `promoted_to_declarative_recipe`, `denied_unsafe_replay` |
| Audit event                       | `macro_recorded`, `macro_replay_requested`, `macro_replay_admitted`, `macro_replay_denied`, `macro_replay_preview_required`, `macro_replay_downgraded`, `macro_replay_promoted_to_recipe`, `macro_attribution_minted` |
| Attribution surface               | `support_export`, `activity_history`, `admin_audit_export`                                                                                                |

Adding a new enum value to any of these vocabularies is additive-minor
and bumps the schema version constant; repurposing an existing value
is breaking and requires a new contract row.

## Reviewer invariants

The validator enforces — and the integration test confirms — the
following invariants:

1. **Command lineage is never inferred from a raw key chord.** Every
   step resolves to one `step_command_lineage_class`; an unmapped step
   resolves to `unmapped_keystroke_denied` and forces the bound
   definition off the silent `proceed_local_editor_only` lane.
2. **Mode state is named up front.** Every step pins the editor /
   palette mode the replay MUST observe; `terminal_mode_denied` is a
   closed denial class because recorded macros never replay raw
   terminal input.
3. **Write classes are the union of every step's writes.** A definition
   that contains any non-`read_only` / non-`editor_buffer_mutation`
   write class MUST NOT keep the `single_buffer_safe` replay
   limitation; the validator refuses the row when it does.
4. **Trust policy is observed.** Definitions never record a
   `managed_only_denied` trust gate (recorded macros are not admissible
   on the managed-only channel), and dispositions never observe one
   either.
5. **Unsafe replays never silently dispatch.** The only disposition
   that silently dispatches is `proceed_local_editor_only`. Every other
   disposition (preview, downgrade, recipe promotion, denial) carries
   the corresponding reason / target / promoted-recipe ref / denial
   label as a closed precondition; the validator refuses the row when
   the reason or target is missing.
6. **Limitation-to-disposition admissibility is enforced.** A
   disposition's class MUST match the disposition the definition's
   replay limitation forces (see the table below); the validator
   refuses the row when it does not.
7. **Attribution is closed.** Every definition mints
   `support_attribution_minted` and `activity_attribution_minted` at
   recording time; every disposition cites both a
   `support_attribution_ref` and an `activity_attribution_ref`; the
   page MUST cover at least `support_export` and `activity_history`
   attribution surfaces.
8. **Raw material never crosses the boundary.** Raw keystroke bytes,
   raw editor buffer bytes, raw shell fragments, and raw credential
   material are pinned absent on every row; the support-export
   projection retains only opaque refs and reviewer-safe summaries.

### Limitation → forced disposition

| Replay limitation                                            | Forced disposition                       |
|--------------------------------------------------------------|------------------------------------------|
| `single_buffer_safe`                                         | none (admissible on `proceed_local_editor_only`) |
| `multi_buffer_requires_preview`                              | `preview_required_before_apply`          |
| `crosses_workspace_boundary_requires_recipe_promotion`       | `promoted_to_declarative_recipe`         |
| `non_replayable_unstable_timing`                             | `denied_unsafe_replay`                   |
| `denied_unmapped_command_present`                            | `denied_unsafe_replay`                   |

## What the fixture exercises

The protected fixture covers five definitions and matching
dispositions:

1. **Normalize imports (proceed).** Three core editor commands; single
   buffer; admissible on the silent `proceed_local_editor_only` lane.
2. **Rename symbol across open files (preview).** Multi-file mutation
   forces `preview_required_before_apply`.
3. **Apply scaffold template across workspace (promote).** Extension
   command crosses the workspace boundary; replay
   `promoted_to_declarative_recipe`; admin-audit attribution row
   minted alongside support / activity rows.
4. **AI-assist rename (downgrade).** AI-tool handle step under admin
   policy; replay `downgraded_to_observer_no_mutation` with a named
   downgrade target.
5. **Terminal-key-chord attempt (deny).** Mixed lineage with an
   unmapped keystroke targeting terminal mode and a process-mutation
   write class; replay `denied_unsafe_replay` with a typed
   denial-reason label.

The audit-event stream covers `macro_recorded`,
`macro_replay_requested`, `macro_replay_admitted`,
`macro_replay_denied`, `macro_replay_preview_required`,
`macro_replay_downgraded`, `macro_replay_promoted_to_recipe`, and
`macro_attribution_minted` so every disposition class is observable on
the audit lane.

## What this lane does not claim

- It does not ship the live macro recorder, the live macro replayer,
  or the run-record publisher; the runner lands in its own lane and
  consumes this alpha as a typed boundary.
- It does not broaden recorded macros into a scripting surface; the
  declarative-first posture from
  [`/docs/automation/recipe_and_macro_contract.md`](../../automation/recipe_and_macro_contract.md)
  is the only path admitted on the managed-only channel and the only
  path that may declare ambient network, process, or secret
  capabilities.
- It does not silently widen mutation authority: every disposition is
  named, every guardrail is closed, and silent authority widening is
  pinned absent on every row.
