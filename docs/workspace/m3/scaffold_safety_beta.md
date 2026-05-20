# Scaffold-safety beta truth

Beta layer that turns template manifests, scaffold plans, and
scaffold-run records into explicit, reviewable objects so project
generation stops being a privileged black box. Aureline can create or
update projects through templates and generators only after showing
provenance, parameters, file and dependency impact, hook / egress
posture, validation tasks, and the rollback boundary — and the generated
output stays plain, attributable workspace content rather than hidden
IDE-owned state.

The lane binds three boundary records into one cross-surface projection
so Start Center starter rows, the command palette, the generator-preview
sheet, AI-assisted generation, extension-provided generators, the CLI /
headless creation path, and support exporters always agree, **before any
file is written**, on:

- **what is being generated** — the signed template / generator
  identity, provider, signature state, generation kind, supported
  ecosystems / archetypes, and the distinct generation verb (create
  project, generate into existing, update / regenerate) quoted from the
  descriptor and plan;
- **what the scaffold plan will do** — the target scope, resolved
  parameter sources, file / directory impact, dependency and task plan,
  and remote / bootstrap implications;
- **which side effects are declared before execution** — the closed
  hook / network / registry / remote-image / dependency side-effect set,
  each declared before execution and attributable after rollback;
- **which create-empty / set-up-later / rollback handoffs are visible** —
  the explicit setup choices and the rollback boundary the plan plants
  before any write; and
- **whether the run kept generated output as plain workspace content** —
  the optional run outcome and lineage ref plus the guardrails that block
  undeclared hooks and any hidden project database.

The exit-gate condition the surfaces guard together is the scaffold
anchor: **Aureline can create or update projects through templates and
generators only after showing provenance, parameters, file and
dependency impact, hook / egress posture, validation tasks, and rollback
boundary instead of treating project generation as a privileged black
box.**

The machine-readable boundaries are:

- [`/schemas/workspace/template_generator_descriptor.schema.json`](../../../schemas/workspace/template_generator_descriptor.schema.json)
- [`/schemas/workspace/scaffold_plan.schema.json`](../../../schemas/workspace/scaffold_plan.schema.json)
- [`/schemas/workspace/scaffold_run.schema.json`](../../../schemas/workspace/scaffold_run.schema.json)

The projection record itself is:

- [`/schemas/workspace/scaffold_safety.schema.json`](../../../schemas/workspace/scaffold_safety.schema.json)

The worked fixtures live under:

- [`/fixtures/workspace/m3/scaffold_preflight_and_generation/`](../../../fixtures/workspace/m3/scaffold_preflight_and_generation/)

The Rust types are exported from `aureline_workspace::scaffold`. The
integration test
[`crates/aureline-workspace/tests/scaffold_safety_beta.rs`](../../../crates/aureline-workspace/tests/scaffold_safety_beta.rs)
replays every scenario fixture, proves the closed acceptance states, and
round-trips each record through the descriptors. This beta layer builds
on the alpha template/scaffold packet (`generated_projects`) and the
alpha template-bundle / scaffold schemas; when they disagree, the
boundary records here win and the alpha projection updates with them.

## 1 Beta truth contract

Every scaffold surface reads exactly one
`ScaffoldSafetyBetaProjection`. The projection is derived from one
`TemplateGeneratorDescriptor`, one `ScaffoldPlanRecord`, and zero-or-one
`ScaffoldRunRecord`. The projection refuses to assemble when the plan
does not reference the supplied descriptor, when the run does not
reference the supplied plan and descriptor, when the plan plans a task it
claims is descriptor-declared but no such hook / task exists, or when the
run invokes a hook / task the descriptor never declared. This guarantees
that no surface reads a plan or run that bound to a sibling or stale
template, and that AI / extension generation cannot smuggle an
undeclared hook or hidden bootstrap step through the plan or run.

## 2 Template / generator descriptor

The signed descriptor names identity / version, provider class,
signature state, generation kind, supported ecosystems / archetypes,
required parameters, declared hooks, declared validation tasks, the trust
and egress expectations, policy constraints, and provenance. A scaffold
run may invoke only hooks and validation tasks that appear here. AI-
assisted (`ai_assisted`) and extension-provided (`extension_provided`)
generation are first-class providers, not a privileged bypass: they
carry the same descriptor and reuse the same governed scaffold-plan /
diff-review surface. A descriptor whose `signature_state` is not
`signed_verified` is never rendered as fully trusted.

## 3 Scaffold plan (preflight / generator review sheet)

The plan is the preflight the user reviews before any write. It names:

| Field | Meaning |
|---|---|
| `generation_verb` | `create_project`, `generate_into_existing`, or `update_regenerate` — distinct verbs with distinct review semantics. |
| `target` | The scope (`new_project_root` / `subdirectory` / `existing_project_root` / `single_file_set`), an opaque path ref, and whether it writes into existing content. |
| `resolved_parameters` | Each parameter and its source (`user_input`, `default`, `inferred`, `policy_pinned`, `ai_suggested`, `extension_supplied`) — resolved sources, not just values. |
| `file_impact` | Create / modify / delete / directory counts disclosed before any write. |
| `dependency_plan` | Per-entry package action and registry class; `restore_now` / `restore_deferred` reach a registry and must declare a dependency side effect. |
| `task_plan` | Planned setup / validation tasks; a `declared_in_descriptor` task must actually exist on the descriptor. |
| `remote_bootstrap_implications` | Network fetch / registry access / remote-image pull / devcontainer bootstrap / prebuild attach / credential provisioning / remote-workspace create — each declared before execution. |
| `side_effect_declarations` | The hook / network / registry / remote-image / dependency families, each declared before execution and attributable after rollback. |
| `setup_choices` | Explicit `create_empty` / `set_up_later` / `full_scaffold` / `scaffold_without_dependency_restore` handoffs where the source artifact supports them. |
| `rollback_boundary` | `checkpoint` / `delete_generated_files` / `git_initial_commit` / `manual_only` planted before any write. |
| `review_state` | `no_writes_before_review` and an exportable preflight summary. |

No claimed beta scaffold flow writes files before the user can review
this plan or its exported preflight summary.

## 4 Scaffold run

When the plan executes, a `ScaffoldRunRecord` keeps the result
replay-safe for support and migration flows. It cites the plan and
descriptor, lists created / modified artifacts, names the invoked
declared hooks / tasks, binds the checkpoint and the plain-file
generated-project lineage ref, and reports the outcome
(`succeeded` / `partially_applied` / `failed_rolled_back` /
`failed_left_in_place` / `cancelled`), the actor lineage
(`user` / `ai_assistant` / `extension` / `automation` /
`support_replay`), and the rollback state. `plain_file_authority` and
`no_hidden_project_database` must be true: no hidden project database and
no silent post-create package restore is the authoritative result of
generation.

## 5 AI-assisted and extension-provided generation

AI-assisted and extension-provided generation reuse this same projection
and the same diff-review surface. The projection guarantees they cannot
invent undeclared hooks, hidden bootstrap steps, or IDE-only authority:
the run invokes only descriptor-declared hooks / tasks (enforced at
projection time), blocks undeclared actions, and still honours
`no_writes_before_review`. The `ai_extension_uses_governed_surface`
guardrail makes this explicit whenever the descriptor provider or the run
actor is an AI assistant or extension.

## 6 Guardrails

The projection guarantees seven typed guardrails, each mapping to a
guardrail or acceptance criterion in the scaffold-safety spec:

| Guardrail | Holds when |
|---|---|
| `no_writes_before_review` | The plan guards no-writes-before-review and exposes an exportable preflight summary, and any run agrees it wrote nothing before review. |
| `side_effects_declared_before_execution` | Every declared side effect and every remote / bootstrap implication is declared before execution. |
| `side_effects_attributable_after_rollback` | Every declared side effect remains attributable to the plan / run after failure or rollback. |
| `no_undeclared_hooks_or_bootstrap` | A run invokes only descriptor-declared hooks / tasks and blocks undeclared actions. |
| `generated_output_is_plain_workspace_content` | A run keeps plain-file authority and no hidden project database. |
| `rollback_boundary_visible` | A rollback / delete-generated boundary is visible and a failed run names an attributable (non-`not_needed`) rollback state. |
| `ai_extension_uses_governed_surface` | AI / extension generation still enforces no-writes-before-review and undeclared-action blocking. |

`ScaffoldSafetyGuardrails::all_hold()` is true only when every guardrail
holds.

## 7 Honesty labels

Honesty labels are the closed vocabulary a surface renders verbatim
alongside a scaffold / generation row: `unsigned_template`,
`signature_mismatch`, `ai_assisted_generation`,
`extension_provided_generation`, `hooks_declared`,
`network_egress_declared`, `registry_access_declared`,
`remote_image_pull_declared`, `dependency_restore_declared`,
`writes_into_existing_project`, `set_up_later_available`,
`create_empty_available`, `policy_constrained`, and `partial_rollback`.
`surface_must_disclose_generation()` is true when the descriptor is
unsigned / mismatched, AI / extension provided, network-bearing, writes
into an existing project, declares any side effect, or its run failed and
left attributable artifacts.

## 8 Fixture coverage

The scenario fixture suite covers, at minimum:

- `create_project_first_party_signed` — first-party signed template,
  deferred registry restore, checkpoint rollback, succeeding run.
- `generate_component_into_existing` — preflight-only component
  generator into an existing project with no egress.
- `ai_assisted_scaffold_governed` — AI-assisted, unsigned, governed
  surface, run blocking undeclared hooks.
- `extension_provided_generator` — extension-provided, signed-unverified,
  registry side effect, extension actor run.
- `update_regenerate_migration` — template update / migration with a
  partially-applied run and plain replay-safe lineage.
- `offline_signed_template_no_egress` — offline signed bundle with
  create-empty / set-up-later handoffs and no egress.
- `remote_image_devcontainer_scaffold` — devcontainer template declaring
  a remote-image pull and devcontainer bootstrap before execution.
- `failed_rolled_back_run` — a run that fails mid-apply and rolls back to
  its checkpoint while staying attributable.
- `policy_constrained_registry_allowlist` — fleet-pinned version and a
  registry allowlist surfaced in preflight.

Removing any of these scenarios without a replacement fixture is a
breaking contract change.

## 9 Scope and out-of-scope

This lane seeds the boundary records, the cross-surface projection, the
schemas, the fixtures, and the conformance test. It deliberately does not
build the post-v1 signed template registry or a full cloud-control
creation plane, and it does not introduce a hidden project database or a
silent post-create package restore as the authoritative result of
generation. Downstream consumers (shell starter rows, the command lane,
the security / trust review, and support export) read this projection;
they do not mint a parallel scaffold-safety vocabulary.
