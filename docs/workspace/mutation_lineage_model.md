# Mutation-journal and generated-artifact lineage model

This document seeds one lineage model that the editor-edit, refactor,
AI-apply, save, build, preview, and review lanes all reuse. It is the
shared vocabulary the mutation journal, local history, AI evidence
packets, review packs, recovery exports, support bundles, and the
eventual replay lane read when they have to describe *what changed*,
*who changed it*, *how to get it back*, and *which bytes are
authoritative*.

The machine-readable schemas live at:

- [`/schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json)
- [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)
- [`/schemas/workspace/mutation_journal_alpha.schema.json`](../../schemas/workspace/mutation_journal_alpha.schema.json)
  — the export-safe alpha projection consumed by support/review surfaces.

This document does not restate ADR 0003, ADR 0005, or ADR 0006. It
pins the vocabulary every non-buffer / non-VFS surface must use when
it renders, logs, exports, or explains a mutation and its consequences.
The ADRs win on any disagreement; this document and the schemas are
updated in the same change.

## Why freeze this now

An IDE that promises "previewable, attributable, and undoable" cannot
land honest UI until the mutation record is one shape. Left implicit,
each surface answers "what was this mutation, really?" slightly
differently — local history records one set of fields, the AI evidence
packet records another, the refactor preview records a third, the
support bundle exports a fourth, and the replay lane reconstructs a
fifth. The goal here is one frozen lineage model so every surface
tells the same story about the same change.

Generated artifacts are the other half of the honesty contract. A
build output, codegen sibling, lockfile, notebook output, preview
snapshot, or mirrored pack is not a hand-authored file even though it
appears as one. The writable-boundary policy, the regeneration
affordance, and the drift state need one vocabulary so search, review,
AI, and save flows never overclaim which bytes are authoritative.

## Scope

- Freeze one mutation-journal entry shape that covers keystrokes,
  multi-cursor edits, structural edits, refactors, save-participant
  groups, AI apply, imports, migrations, external reloads, decode
  recovery, filesystem renames, build runs, codegen runs, preview
  regenerations, and audit-only external effects.
- Freeze one generated-artifact lineage record that covers build
  outputs, codegen siblings, structured lockfiles, notebook outputs,
  preview snapshots, and mirrored pack artifacts.
- Reuse the ADR-0003 undo-class taxonomy unchanged and carry it on
  every mutation entry.
- Freeze an orthogonal **reversal-class** axis (exact undo,
  compensating undo, regenerate / recompute, restore from
  checkpoint, audit-only) so the UI can say what users can actually
  get back.
- Freeze the **durable vs disposable** state-class axis so
  clear-caches flows never blow away user-authored durable state
  and so the mutation journal separates user-authored settings from
  derived index rebuilds.
- Seed example fixtures for the required originator classes: typing,
  format-on-save, repair (decode recovery), AI patch proposal, build
  output, and generated preview artifact.
- Seed an export-safe alpha packet proving formatter, lockfile,
  build-output, preview-regeneration, and AI-apply paths share one
  support-safe envelope without raw bodies or secret material.

## Out of scope

- The full history UI or recovery engine. The spec for M0 is the
  vocabulary, not the surfaces that render it.
- The concrete journal on-disk encoding. The buffer's recovery
  journal (ADR 0003) is a private implementation detail; this
  document freezes the cross-surface *record* shape, not the bytes
  on disk.
- Final replay / timeline UI design. The replay lane reads the
  records defined here; the UI is a later decision row.
- Per-artifact-class regeneration playbooks beyond the hint kinds
  enumerated here. The playbooks live with the generator lane.

## 1. Mutation-journal record

Every mutating command writes one `mutation_journal_entry`. Named
operations also write one `mutation_group_record` that lists member
`mutation_id`s. Support bundles, review packs, AI evidence packets,
and local-history entries reference records by `mutation_id` or
`group_id` rather than duplicating the bodies.

### 1.1 Object identity

Each entry carries the identity of the thing that changed.

- `target_refs[].target_kind` — one of `filesystem_object`, `buffer`,
  `workspace_setting`, `workspace_manifest`, `task_config`,
  `launch_config`, `policy_document`, `generated_artifact`,
  `external_service`.
- `target_refs[].filesystem_identity` — full four-layer
  filesystem-identity record (ADR 0006 layers 1–4) for filesystem,
  buffer, and generated-artifact targets. Present by reference to
  `schemas/filesystem/save_target_token.schema.json#filesystem_identity_record`;
  not re-invented here.
- `target_refs[].logical_ref` — opaque logical reference for
  non-filesystem targets (setting key, task id, service endpoint).
- `target_refs[].affected_range` — optional byte / line range inside
  the target; coordinates follow ADR 0003.
- `scope_ref` — one of `workspace`, `root`, `workset`, `slice`,
  `window`, `buffer`, `file`, `review_workspace`, `remote_session`,
  `companion_surface`, `settings_scope`.

### 1.2 Actor / source classification

- `actor_class` — what the editor called the originator:
  `user_keystroke`, `user_command`, `multi_cursor_command`,
  `refactor_engine`, `formatter`, `save_participant`, `ai_apply`,
  `code_action`, `scaffolding`, `settings_import`,
  `workspace_migration`, `external_reload`, `decode_recovery`,
  `build_runner`, `codegen_runner`, `preview_regenerator`,
  `review_apply`, `replay_import`, `vendor_import`.
- `source_class` — provenance class: `human_local`,
  `human_remote_session`, `machine_local`, `machine_remote_agent`,
  `ai_local_model`, `ai_hosted_provider`, `imported_external`,
  `replayed_capture`, `policy_driven`.
- `actor_ref` — `display_name`, optional `stable_id`, optional
  `role`. Redaction-aware.
- `command_id` — canonical command id from the command plane.

### 1.3 Undo class and reversal class (orthogonal axes)

- `undo_class` — frozen ADR-0003 taxonomy: `text_edit`,
  `multi_cursor_text_edit`, `structural_edit`, `refactor_single_file`,
  `refactor_multi_file`, `formatter_run`, `save_participant_group`,
  `imported_change`, `machine_generated_change`, `migration_change`,
  `external_reload`, `decode_recovery_change`. The compensation
  posture of each class lives in
  [`artifacts/architecture/undo_class_rows.yaml`](../../artifacts/architecture/undo_class_rows.yaml)
  and is invariant.
- `reversal_class` — what a user can actually get back:
  `exact_undo`, `compensating_undo`, `regenerate_or_recompute`,
  `restore_from_checkpoint`, `audit_only`. The UI MUST surface this
  value verbatim; a plain "Undo" label is forbidden on compensating
  / regenerate / audit-only mutations.
- `reversibility.reversible` — convenience boolean for badges.
- `reversibility.declared_at_commit` — true when the reversal class
  was recorded at commit time. If authority drifts after commit and
  the originally exact mutation becomes compensating, an amendment
  record flips this flag to false and cites
  `reversibility.downgrade_reason`.

### 1.4 Checkpoint linkage

- `checkpoint_refs[]` — each ref carries `checkpoint_kind` and
  `checkpoint_id` from one of `recovery_journal`, `save_manifest`,
  `workspace_migration`, `settings_backup`, `mutation_group_preview`,
  `local_history_snapshot`, `review_checkpoint`, and a
  `durability_class` from `{durable, disposable}` mirroring ADR 0006.
- `save_manifest_ref` — opaque pointer to an ADR-0006 save manifest
  for mutations that reached the save pipeline.
- `generated_artifact_lineage_ref` — opaque pointer to a generated-
  artifact lineage record for mutations on generated artifacts.
- `preview_ref` — pointer to the preview record for any grouped
  apply path required to be previewable by policy.
- `approval_ref` — pointer to the approval record for any mutation
  gated by review-before-save / review-before-rename / AI-evidence
  policy.

### 1.5 Durable versus disposable

`durable_vs_disposable` is mandatory on every entry:

- `durable_user_authored` — buffers, settings, profiles, snippets,
  keybindings.
- `durable_workspace_authored` — workspace manifests, task configs,
  launch configs, AI policy files, extension locks.
- `disposable_derived` — indexes, symbol caches, search shards,
  render caches, preview bundles.

This separation is how the product keeps "clear caches" safe: no
`disposable_derived` clear command may traverse `durable_user_authored`
entries.

### 1.6 Policy / redaction / side-effect summary

- `policy_context.policy_epoch` — the policy epoch in effect at
  commit. Replay consumers compare this against the current epoch
  before re-running any compensating command.
- `redaction_class` — `metadata_only`, `environment_adjacent`,
  `code_adjacent`, or `high_risk`. Drives how support bundles, AI
  evidence packets, and replay captures export the body.
- `side_effect_summary.summary` — short redaction-aware free text
  (≤ 1024 graphemes). Product, CLI, and support render the same
  text.

### 1.7 Group records

One named operation emits one `mutation_group_record`:

- `group_kind` — `multi_cursor_keystroke`, `refactor_single_file`,
  `refactor_multi_file`, `format_on_save`, `save_participant_group`,
  `ai_patch`, `bulk_replace`, `multi_file_rename`, `scaffolding_run`,
  `migration_import`, `settings_import`,
  `generated_artifact_refresh`, `preview_regeneration`,
  `external_reload_group`, `review_apply_group`.
- `member_mutation_ids` — every member entry.
- `resolution` — `applied`, `aborted`, `reverted`, or
  `partially_applied_and_rolled_back`.
- The group's `reversal_class` is the most restrictive of its
  members (any `audit_only` member forces the group to
  `audit_only`; otherwise the group takes the strongest posture
  the members all meet).

## 2. Generated-artifact lineage record

Every generated or mirrored artifact that can appear in the shell,
search, graph, AI context, review, or support flow carries exactly
one `generated_artifact_lineage_record`.

### 2.1 Source inputs

- `canonical_source_refs[]` — each ref names a `source_kind` from
  `filesystem_object`, `declarative_manifest`, `schema_or_idl`,
  `template`, `notebook_cell_group`, `data_snapshot`,
  `preview_runtime`, `annotations`, `upstream_signed_artifact`,
  `registry_entry`, or `dependency_manifest`.
- Workspace sources carry a full filesystem-identity record by
  reference; non-workspace sources carry an `opaque_ref`.
- `role` tags the input's relationship to the artifact (primary,
  template, sibling, data, config, toolchain).

### 2.2 Generator / signer identity

- `generator_or_signer_ref.kind` — `local_generator`,
  `build_toolchain`, `package_resolver`, `notebook_kernel`,
  `preview_runtime`, `codegen_tool`, `formatter_as_generator`,
  `upstream_signer`, `ai_assisted_generator`, `migration_runner`.
- `generator_or_signer_ref.id` and `.version` are mandatory.
- `toolchain_identity` is required on build-output class when the
  toolchain is distinct from the generator id (target triple,
  profile, cross-compile key).
- `signature_chain[]` is required on `mirrored_pack_artifact` class
  records.

### 2.3 Input digest set and output digest

Mirrors ADR-0006 cache-identity exactly:

- `input_digest_set[]` — each entry names an input by `name` plus
  `digest_kind` and `digest_value`. Legal `digest_kind` values:
  `content_hash`, `file_id_generation`, `device_inode_generation`,
  `windows_object_id`, `provider_object_id_revision`,
  `remote_revision_token`, `registry_coordinate`,
  `upstream_artifact_digest`.
- `output_digest` — `content_hash` only. A support bundle that
  proves "these are the bytes the generator wrote" quotes this
  value.

### 2.4 Drift state

Mirrors ADR-0006 and TAD Appendix DE.2:

- `in_sync` — output matches the declared inputs and generator.
- `stale_inputs` — one or more inputs changed since last run.
- `generator_changed` — generator or toolchain changed.
- `manually_diverged` — the artifact was edited directly, past the
  writable boundary.
- `unknown_lineage` — provenance is incomplete.

### 2.5 Writable boundary

- `writable_boundary.boundary_kind` — `never_writable`,
  `canonical_source_only`, `structured_round_trip_region`,
  `full_with_divergence_marker`, or `replace_via_mirror_promotion`.
- `writable_boundary.allowed_ranges[]` — enumerated safe-edit
  regions for structured artifacts (lockfile user-preference
  region, notebook code cells, config user overlay, annotated
  round-trip blocks).
- `writable_boundary.override_requires_divergence_marker` — when
  true (default), any write outside the boundary forces the save
  pipeline to record `drift_state = manually_diverged` and surface
  a visible divergence badge.

### 2.6 Regeneration hints

- `regeneration_hints[]` — each hint names a `hint_kind`
  (`regeneration_command`, `rebuild_command`, `resolver_command`,
  `kernel_reexecute`, `preview_runtime_refresh`,
  `mirror_promotion`, `manual_instructions`), an optional
  `command_ref`, an optional `documentation_ref`, and a coarse
  `expected_cost_class` (`instant`, `fast`, `moderate`,
  `expensive`, `network_dependent`).
- Hints may be empty only when `drift_state` is `unknown_lineage`;
  otherwise at least one hint is required.

### 2.7 Provenance

- `provenance.recorded_at` — producer-local monotonic timestamp.
- `provenance.source_ref` — opaque pointer to the originating
  build-run id, codegen invocation id, notebook execution id, or
  mirrored-pack manifest id.
- `provenance.mutation_group_id` — set when the artifact was
  produced inside a named mutation group, completing the round
  trip between the mutation journal and the lineage record.
- `provenance.support_ref` — optional pointer to a support-bundle
  entry that carries the full forensic packet.

## 3. User-authored durable versus disposable / generated state

ADR 0003 freezes that the recovery journal never silently discards
user bytes. ADR 0006 freezes that durable caches never promote over
authoritative state. This document finishes the rule set so every
surface knows which axis it is on:

| Surface authority class            | Mutation-journal `durable_vs_disposable` | Typical targets                                           | Default survives clear-caches? |
|------------------------------------|------------------------------------------|-----------------------------------------------------------|--------------------------------|
| **Buffer / editor authority**      | `durable_user_authored`                  | Buffers, settings, profiles, snippets, keybindings        | yes                            |
| **Workspace authority**            | `durable_workspace_authored`             | Workspace manifests, task configs, launch configs, policy | yes                            |
| **Derived knowledge / caches**     | `disposable_derived`                     | Indexes, symbol caches, search shards, render caches      | no                             |
| **Generated artifacts (authored)** | `durable_workspace_authored`             | Codegen output under `manually_diverged`                  | yes (until regenerated)        |
| **Generated artifacts (in_sync)**  | `disposable_derived`                     | Build outputs, preview snapshots, `in_sync` codegen       | no                             |

The rule is simple: user-authored or workspace-authored durable
entries survive cache sweeps and survive producer restart. Disposable
derived entries do not. A generated artifact whose `drift_state` is
`manually_diverged` is carried into `durable_workspace_authored`
until the user regenerates; otherwise a `clear caches / rebuild`
flow would silently discard the override.

## 4. Worked examples

Each example references a companion fixture under
[`/fixtures/workspace/mutation_lineage_examples/`](../../fixtures/workspace/mutation_lineage_examples/).

### 4.1 Typing (single-character keystroke)

The user types a single character. One `mutation_journal_entry`
with `actor_class = user_keystroke`, `source_class = human_local`,
`undo_class = text_edit`, `reversal_class = exact_undo`,
`durable_vs_disposable = durable_user_authored`, and one target
that is the active buffer. No generated-artifact lineage.

See [`typing_single_keystroke.json`](../../fixtures/workspace/mutation_lineage_examples/typing_single_keystroke.json).

### 4.2 Format-on-save (save-participant group)

The user hits save. A `mutation_group_record` with
`group_kind = format_on_save` opens; its members are one
`formatter_run` mutation and one `save_participant_group` mutation.
The group resolves with `resolution = applied` and
`reversal_class = compensating_undo` because ADR 0003 marks
`save_participant_group` as only-revertible. The group record links
the ADR-0006 save manifest via `save_manifest_ref`.

See [`format_on_save_group.json`](../../fixtures/workspace/mutation_lineage_examples/format_on_save_group.json).

### 4.3 Decode recovery (repair)

A file opens under decode-recovery state and the user accepts a
recovered decoding. One mutation entry with
`actor_class = decode_recovery`, `source_class = machine_local`,
`undo_class = decode_recovery_change`,
`reversal_class = restore_from_checkpoint` (the raw bytes are
preserved via the recovery-journal checkpoint per ADR 0003), and
one checkpoint ref of kind `recovery_journal`.

See [`decode_recovery_repair.json`](../../fixtures/workspace/mutation_lineage_examples/decode_recovery_repair.json).

### 4.4 AI patch proposal applied

An AI apply flow opens a named group and emits multiple
`machine_generated_change` entries across two buffers. One
`mutation_group_record` with `group_kind = ai_patch`,
`source_class = ai_hosted_provider`, `reversal_class =
compensating_undo`, a `preview_ref` pointing at the AI-evidence
packet, and an `approval_ref` citing the AI-evidence gate.

See [`ai_patch_applied.json`](../../fixtures/workspace/mutation_lineage_examples/ai_patch_applied.json).

### 4.5 Build output

A build run writes a compiled binary. The mutation-journal entry is
an `actor_class = build_runner`, `reversal_class =
regenerate_or_recompute`, `durable_vs_disposable =
disposable_derived` record whose single target is a generated
artifact. The matching `generated_artifact_lineage_record` lives in
a sibling fixture with `generation_class = build_output`,
`edit_policy = block_direct_edit`,
`writable_boundary.boundary_kind = never_writable`, and a
`regeneration_hints[]` list naming the rebuild command.

See [`build_output_mutation.json`](../../fixtures/workspace/mutation_lineage_examples/build_output_mutation.json)
and [`build_output_lineage.json`](../../fixtures/workspace/mutation_lineage_examples/build_output_lineage.json).

### 4.6 Generated preview artifact

A preview regeneration refreshes a design snapshot. Mutation entry
with `actor_class = preview_regenerator`, `reversal_class =
regenerate_or_recompute`, `durable_vs_disposable =
disposable_derived`, and a `generated_artifact_lineage_ref`
resolving into a record with `generation_class =
preview_render_snapshot`, `writable_boundary.boundary_kind =
canonical_source_only`, and `regeneration_hints[]` naming the
preview-runtime refresh.

See [`preview_regeneration.json`](../../fixtures/workspace/mutation_lineage_examples/preview_regeneration.json)
and [`preview_snapshot_lineage.json`](../../fixtures/workspace/mutation_lineage_examples/preview_snapshot_lineage.json).

## 5. Surface rules

These rules apply to every surface that renders, logs, exports, or
reasons about the records defined in §1 or §2.

1. **No surface invents a private mutation field.** Every consumer
   reads `mutation_id`, `group_id`, `actor_class`, `source_class`,
   `undo_class`, `reversal_class`, `durable_vs_disposable`, and
   `redaction_class` from the journal record; surfaces do not add
   parallel fields when they render.
2. **Reversal-class language is canonical.** "Undo" is rendered
   only for `exact_undo`. Compensating / regenerate / restore /
   audit-only mutations render their respective verbs (`Revert
   with compensation`, `Regenerate from source`, `Restore
   checkpoint`, no undo affordance).
3. **Generated artifacts always carry lineage.** A generated
   artifact that appears in the shell, search, graph, AI context,
   review, or support flow without a lineage record is a contract
   violation. The surface renders a degraded / unknown-lineage
   badge rather than hiding the artifact.
4. **Durable user-authored entries survive cache sweeps.** Any
   clear-caches flow filters by `durable_vs_disposable`; it never
   touches `durable_user_authored` or `durable_workspace_authored`.
5. **Preview and approval refs are not optional when policy
   requires them.** A policy-gated mutation without a
   `preview_ref` or `approval_ref` is a contract violation; the
   save pipeline refuses the commit.
6. **Lineage identity reuses ADR 0006.** The artifact's identity
   record, its source inputs' identities, and its writable-
   boundary decisions all cite the five-layer identity model; no
   surface invents a parallel identity.
7. **Support parity.** Every mutation-journal entry and
   generated-artifact lineage record exports through the support
   bundle, replay capture, and AI evidence packet families with
   the same fields it shows in chrome. Redaction is the only way
   to hide a field.

## 6. Changing this vocabulary

- **Additive-minor** changes (new `actor_class`, new `group_kind`,
  new `checkpoint_kind`, new `generation_class`, new
  `regeneration_hint.hint_kind`, new `edit_policy` slot, new
  `canonical_source_ref.source_kind`) land here and in the
  companion schemas in the same change. The change must cite the
  motivating fixture or packet.
- **Repurposing** an existing state, class, or reversal value is
  breaking. It opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The ADR wins on any disagreement with ADR 0003 or ADR 0006; this
  document and the schemas are updated in the same change when that
  happens.

## 7. Acceptance

- The mutation-journal entry and generated-artifact lineage record
  shapes are reused by every mutation-producing lane (editor, VFS,
  refactor, AI apply, save participants, build, codegen, preview,
  review, migration, import) and by every mutation-consuming lane
  (local history, AI evidence, review packs, support bundles,
  replay). No lane invents its own.
- The schemas at
  [`/schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json)
  and
  [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)
  validate the six worked-example fixtures under
  [`/fixtures/workspace/mutation_lineage_examples/`](../../fixtures/workspace/mutation_lineage_examples/).
- The six required originator classes named in the spec — typing,
  format-on-save, repair (decode recovery), AI patch proposal,
  build output, and generated preview artifact — each have a
  fixture demonstrating actor / source classes and checkpoint
  linkage.
- This document explicitly separates user-authored durable state
  from disposable / generated state (§3) and the mutation journal
  carries the axis on every entry.

## Source anchors

- `.t2/docs/Aureline_PRD.md:707` — "every automated edit path —
  refactorings, quick fixes, AI changes, formatter actions — must
  be previewable, attributable, and undoable".
- `.t2/docs/Aureline_PRD.md:854` — large-file mode backing store.
- `.t2/docs/Aureline_PRD.md:856` — "undo/redo: operation log with
  coalescing and transaction grouping".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1648` —
  "support bundles, recovery manifests, and mutation-journal
  entries must include enough identity metadata".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1797` —
  "12.4.3 Unified mutation journal, undo classes, and compensation
  architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1810` —
  "every mutating command receives a `mutation_id` before commit".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1816` —
  "the mutation journal is the shared lineage source for local
  history, AI evidence packets, review packs, recovery exports,
  and support bundles".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:3920` —
  "21.4.1 Generated-artifact provenance, regeneration, and
  writable-boundary architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9701` —
  "Appendix DD — Mutation Journal, Undo Class, and Compensation
  Matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9727` —
  "Appendix DE — Generated Artifact Provenance, Regeneration, and
  Writable-Boundary Matrix".

## Linked artifacts

- ADR (undo classes, buffer, recovery journal):
  [`docs/adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md).
- ADR (filesystem identity, save pipeline, cache identity):
  [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- Undo-class rows (machine form):
  [`artifacts/architecture/undo_class_rows.yaml`](../../artifacts/architecture/undo_class_rows.yaml).
- Filesystem-identity vocabulary (cross-surface):
  [`docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md).
- Mutation-journal schema:
  [`schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json).
- Generated-artifact lineage schema:
  [`schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json).
- Worked-example fixtures:
  [`fixtures/workspace/mutation_lineage_examples/`](../../fixtures/workspace/mutation_lineage_examples/).
