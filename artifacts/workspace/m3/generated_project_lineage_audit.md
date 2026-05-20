# Generated-project lineage audit

This artifact audits the **generated-project lineage** the M3 scaffold-safety
corpus pins: how each run-bearing drill ties its `ScaffoldRunRecord` back to
the signed descriptor, the reviewed scaffold plan, the created / modified
artifacts, the optional setup tasks, the checkpoint, and the cleanup /
rollback decision — and how that lineage stays reconstructable after rollback,
retry, import / export, and support-safe capture.

The lineage is reconstructed by
[`crates/aureline-qe/src/scaffold_safety/`](../../../crates/aureline-qe/src/scaffold_safety/)
(the `assert_lineage` step of the drill runner) and replayed by
`cargo test -p aureline-qe --test scaffold_safety_conformance`. Every
run-bearing drill below is asserted to reconstruct its descriptor / plan / run
chain and to carry a non-empty `generated_lineage_ref`.

## Lineage chain

For each run-bearing drill, `ScaffoldSafetyBetaProjection::project` refuses to
assemble unless the run binds the supplied plan and descriptor, and the plan
binds the descriptor. The runner then asserts the projection echoes the
fixture identifiers and a non-empty lineage ref:

```
TemplateGeneratorDescriptor.descriptor_id
  └─ ScaffoldPlanRecord.descriptor_ref            (plan binds descriptor)
       └─ ScaffoldRunRecord.scaffold_plan_ref      (run binds plan)
            ├─ created_artifact_refs[]             (plain workspace files)
            ├─ modified_artifact_refs[]
            ├─ invoked_declared_hook_ids[]         (⊆ descriptor.declared_hooks)
            ├─ invoked_declared_task_ids[]         (⊆ descriptor.declared_validation_tasks)
            ├─ checkpoint_ref                       (rollback boundary)
            ├─ rollback_state                       (cleanup decision)
            └─ generated_lineage_ref                (plain-file lineage, non-empty)
```

`plain_file_authority` and `no_hidden_project_database` on the run say whether
a plain workspace file remains the authoritative lineage record. The
`caught.hidden_project_database` drill is the negative control: it sets
`no_hidden_project_database = false`, flips the
`generated_output_is_plain_workspace_content` guardrail to `false`, and is
still required to keep a reconstructable lineage chain — proving the audit
catches a hidden database without losing attribution.

## Run lineage rows

| Drill id | Outcome | Created / modified | Invoked hooks / tasks | Checkpoint | Rollback / cleanup decision | Lineage ref | Plain-file authority |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `template.first_party_signed_create` | succeeded | 2 created | `hook.post_create.git_init` | bound | rollback available | non-empty | yes |
| `template.extension_starter_governed` | succeeded | 2 created / 1 modified | `hook.post_generate.format`, `task.lint` | bound | rollback available (delete-generated) | non-empty | yes |
| `template.ai_assisted_governed` | succeeded | 2 created | `hook.post_generate.format` | bound | rollback available | non-empty | yes |
| `import.scaffold_pack_offline_signed` | succeeded | 2 created | none | bound | rollback available (git-initial) | non-empty | yes |
| `mirror.template_signed_registry` | succeeded | 2 created | `hook.post_create.git_init` | bound | rollback available | non-empty | yes |
| `handoff.create_empty_parity` | succeeded | 0 created | none | bound | rollback available (delete-generated) | non-empty | yes |
| `handoff.setup_later_deferred` | succeeded | 1 created | `hook.post_create.git_init` | bound | rollback available; restore deferred | non-empty | yes |
| `remote_image.devcontainer_pull` | succeeded | 2 created | `hook.post_create.devcontainer_up` | bound | rollback available | non-empty | yes |
| `update.regenerate_managed_zone_partial` | partially_applied | 1 created / 2 modified | `hook.post_generate.codemod` | bound | rollback available (`rollback:…`); `partial_rollback` honesty | non-empty | yes |
| `support.export_run_lineage` | succeeded | 2 created | `hook.post_create.git_init` | bound | rollback available; export-safe capture | non-empty | yes |
| `failure.partial_generation_left_in_place` | failed_left_in_place | 1 created (partial) | `hook.post_create.install` | bound | **manual cleanup** (`unavailable_manual`, named ref) | non-empty | yes |
| `failure.missing_toolchain_rolled_back` | failed_rolled_back | none (rolled back) | `hook.post_create.bootstrap`, `task.build` | bound | rollback **performed** | non-empty | yes |
| `failure.mirror_outage_cancelled` | cancelled | none | none | bound | rollback **not needed** (no writes) | non-empty | yes |
| `failure.remote_image_unavailable_rolled_back` | failed_rolled_back | none (rolled back) | `hook.post_create.devcontainer_up` | bound | rollback **performed** | non-empty | yes |
| `caught.hidden_project_database` | succeeded | 1 created | `hook.post_create.git_init` | bound | rollback available | non-empty | **no — caught** |

The six preflight-only drills (`policy.generator_blocked_allowlist`,
`diff_review.generate_into_existing`, `caught.writes_before_review`,
`caught.hidden_side_effect`) carry no run; their lineage is the
descriptor → plan chain, which the runner still reconstructs.

## Recovery / replay reconstruction

- **After rollback.** `failure.missing_toolchain_rolled_back` and
  `failure.remote_image_unavailable_rolled_back` perform a rollback yet still
  bind a non-empty lineage ref, so the cleared workspace remains attributable
  to its descriptor / plan and is replay-safe for a retry.
- **After partial application.** `update.regenerate_managed_zone_partial`
  carries `partially_applied` with an available rollback ref and the
  `partial_rollback` honesty label, so the managed-zone regeneration is never
  left in an ambiguous half-trust state.
- **After left-in-place failure.** `failure.partial_generation_left_in_place`
  names a manual cleanup path (`unavailable_manual` with a rollback ref)
  rather than `not_needed`, so partial output is explicitly owned and
  cleanable.
- **After cancellation.** `failure.mirror_outage_cancelled` records the
  cancelled attempt with `not_needed` rollback and zero created artifacts, so
  a mirror outage never strands partial state.
- **For import / export and support capture.**
  `support.export_run_lineage` reconstructs the same chain on the support
  surface using only opaque refs and typed labels; the runner's raw-export
  scan guarantees no raw path / credential / byte leaks into the capture.

## Hidden-database control

`caught.hidden_project_database` is the deliberate negative: lineage stays
reconstructable, but because `no_hidden_project_database = false` the
`generated_output_is_plain_workspace_content` guardrail and
`guardrails_all_hold` are `false`. A hidden project database can never be
laundered into a "plain workspace content" claim while still being audited for
lineage.

## Replay

```
cargo test -p aureline-qe --test scaffold_safety_conformance
```
