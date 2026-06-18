# Recipe builder object and first consumers

This document describes the **live recipe-builder object** and the first M5
surfaces that consume it. The
[recipe-builder / parameter-review / dry-run contract](recipe-builder-and-macro-contract.md)
already froze *what* a recipe-builder authoring session is
(`recipe_builder_session_record`) and the reused safety-label vocabulary every
surface reads. This contract closes the runtime gap it left open: the concrete,
mutable builder that **authors, reorders, previews, and exports** declarative
recipes, and the first consumers that bind it instead of inventing feature-local
wizards.

The central rule is unchanged: automation is **declarative first**. The builder
emits declarative recipe manifests only, it cites stable command identities
rather than embedding raw shell or paths, and it keeps blocked and unresolved
steps visible before the user trusts the recipe.

## Companion artifacts

- [`/schemas/automation/recipe-builder-first-consumers.schema.json`](../../schemas/automation/recipe-builder-first-consumers.schema.json)
  — boundary schema for the `m5_recipe_builder_first_consumers_packet`,
  `recipe_builder_export_record`, support-export, and CLI/headless shapes.
- [`/schemas/automation/recipe-builder.schema.json`](../../schemas/automation/recipe-builder.schema.json)
  — the frozen `recipe_builder_session_record` the builder projects onto.
- [`/artifacts/m5/automation/recipe-builder-first-consumers/`](../../artifacts/m5/automation/recipe-builder-first-consumers/)
  — the checked-in first-consumers packet, support export, CLI/headless view, and
  compact projection.
- [`/fixtures/automation/m5/recipe-builder/`](../../fixtures/automation/m5/recipe-builder/)
  — worked-example builder export, blocked builder session, reorder
  demonstration, and the mutation cases that prove the fail-closed gate.
- [`/tools/ci/m5/recipe_builder_first_consumers_check.py`](../../tools/ci/m5/recipe_builder_first_consumers_check.py)
  — the fail-closed CI gate over the artifacts and fixtures.

The Rust types in `crates/aureline-runtime/src/recipe_builder` are the schema of
record; the headless inspector
`crates/aureline-runtime/examples/dump_m5_recipe_builder_first_consumers.rs`
regenerates every artifact and fixture from the seed so they are bit-for-bit
derivable.

## The recipe-builder object

A `RecipeBuilder` is the live authoring state for one recipe. It owns an
**ordered** list of steps and a reorder log; it derives its authoring state, its
validation findings, and the reused safety-label union from those steps; and it
projects the frozen `recipe_builder_session_record` on demand. It holds no
private form state — every projection reads back through the step drafts.

### Ordered, declarative steps

Each step wraps a reused **command-truth draft**: a `command_id`, a
`command_revision_ref`, a `canonical_verb`, the capabilities the step declares,
and the safety labels it projects, all quoted from a command descriptor. The
builder never rewrites a draft; appending, inserting, and removing steps only
change the order and membership of the recipe, so a step's command identity is
preserved for its whole life.

### Drag-or-keyboard reorder

A step is reordered by a drag handle (`drag_to_index`) or a keyboard shortcut
(`keyboard_move_up` / `keyboard_move_down`). Both gestures resolve to **one
canonical target index**, so a recipe reordered by drag and the same recipe
reordered by the equivalent keyboard moves converge on the identical step order.
Every move is recorded in the reorder log with the step id, the gesture kind, and
the from/to indices, so the reorder history survives export.

### Unresolved and blocked state stays visible

The builder's `builder_state_class` is derived, never asserted:

- a step that cites a UI-only command (or is denied by policy or a trust gate)
  drives `blocked` and raises a blocker finding;
- a step with no capability declaration drives `validation_failed`;
- a required argument slot still needing input keeps the recipe in `draft`;
- an `approval_required` label drives `approval_required`; and
- otherwise the recipe is `preview_ready`.

A UI-only command is blocked **on authoring** rather than silently producing an
inadmissible recipe.

### Copy-CLI and open-docs parity

Each step exposes a `copy_cli` string and an `open_docs` anchor that both project
from the same `canonical_verb`: the CLI string contains the verb and the docs
anchor ends with the slugified verb fragment. Parity is mechanically checkable —
the CLI and the docs always point at the same command. Neither ever embeds raw
argv, paths, URLs, or secrets.

### Export and import

`RecipeBuilder::export` nests the whole builder verbatim alongside the derived
authoring-session projection and an order-stable digest. The export preserves
step order, command provenance, unresolved and blocked state, the copy-CLI /
open-docs actions, and the reorder log, so `import` reconstructs the identical
builder. This is what makes a recipe a trustworthy shareable artifact across
share, rerun-review, and support-export flows.

## First consumers

The first-consumers packet binds the six M5 surfaces that now suggest or save
recipes, each to a seeded builder:

| Entrypoint | Recipe authored |
|---|---|
| `notebook` | Run notebook and export results |
| `task_test_debug` | Run tests and rerun failures |
| `request_api` | Send request and save response |
| `package` | Audit and update dependencies (under approval) |
| `incident` | Capture incident evidence bundle |
| `ai_assistant` | Apply AI-proposed fix under review |

Each binding carries the builder's projected `recipe_builder_session_record`, its
state, its per-step copy-CLI lines and open-docs anchors, and the unresolved
count — proving the surface reuses the canonical builder rather than a private
runner.

## Freeze invariants

The packet pins these invariants as schema-level constants. A false value is
non-conforming.

1. `builder_reuses_command_truth_not_private_form_state`
2. `every_entrypoint_binds_the_canonical_builder`
3. `steps_are_ordered_and_reorder_preserves_identity`
4. `blocked_or_unresolved_steps_remain_visible`
5. `copy_cli_and_open_docs_cite_the_same_command`
6. `builder_emits_declarative_manifests_only`
7. `builder_state_survives_export_import`

## How the freeze is enforced

[`/tools/ci/m5/recipe_builder_first_consumers_check.py`](../../tools/ci/m5/recipe_builder_first_consumers_check.py)
is the fail-closed gate. It blocks stable when an entrypoint is dropped, a builder
is empty, a step loses its command identity, a builder targets a non-declarative
manifest, a UI-only step is not blocked, copy-CLI / open-docs parity breaks, or an
invariant is violated. The mutation fixtures under
[`/fixtures/automation/m5/recipe-builder/`](../../fixtures/automation/m5/recipe-builder/)
each reproduce one blocking state, and the typed Rust consumer mints the identical
packet so `cargo test -p aureline-runtime --test m5_recipe_builder_first_consumers`
enforces the same invariants.

## Source anchors

- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md) — power-user
  automation requirements, declarative-first posture, CLI/headless rules.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md)
  — safe-automation matrix, recipe and command architecture.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  — command invocation / session / result contracts and recipe objects.
- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — recipe builder, parameter review, and dry-run/explain UX.
- [`.t2/docs/Aureline_UX_Design_System_Style_Guide.md`](../../.t2/docs/Aureline_UX_Design_System_Style_Guide.md)
  — recipe builder, drag-or-keyboard reorder, and copy-CLI / open-docs rules.
