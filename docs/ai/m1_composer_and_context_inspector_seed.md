# M1 AI composer and context-inspector seed

Reviewer-facing landing page for the bounded launch AI wedge that lands the
first-pass AI composer and context inspector on a protected dogfood path.

The seed is intentionally a **bounded prototype**:

- the composer carries no mutation authority and never dispatches a model;
- the context inspector reads the same record the composer mints — it never
  re-derives mention, attachment, or route truth locally;
- every row carries a typed honesty marker rather than a free-form warning;
- every attachment is individually inspectable and removable, and every
  mention / slash-command invocation resolves through the shared registry
  rather than a bespoke AI-only vocabulary.

## Reuse, not reinvention

The seed projects against three upstream contracts and does not fork their
vocabularies:

- [`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  and the canonical [`aureline_commands::CommandRegistry`] supply the
  resolution target for slash-command invocations. The composer never mints
  ad-hoc AI-only command ids; `ComposerSlashCommandInvocation::resolve_in_registry`
  returns either a registry-resolved canonical [`CommandId`] or a typed
  `UnresolvedNoMatch` state.
- [`docs/runtime/execution_context_seed.md`](../runtime/execution_context_seed.md)
  and the [`aureline_runtime::ExecutionContext`] object supply the
  execution-context identity the composer references when a draft mentions
  the current target / scope. The wedge never re-derives target / cwd /
  toolchain truth.
- [`docs/ai/prompt_composer_contract.md`](./prompt_composer_contract.md) and
  [`docs/ai/context_assembly_contract.md`](./context_assembly_contract.md)
  freeze the broader mention / attachment / trust-posture / source-class /
  block-reason vocabularies. The M1 seed covers a small, honest subset of
  those vocabularies and grows additively without forking truth.

## What the seed owns

| Artifact | Role |
| --- | --- |
| `crates/aureline-ai/src/composer/mod.rs` | One inspectable `ComposerDraft` record plus the typed mention / attachment / slash-command / route-placeholder vocabularies and the `validate()` outcome. |
| `crates/aureline-shell/src/ai_context_inspector/mod.rs` | One `AiContextInspectorSnapshot` projection the shell renders. Every row carries a stable id, a status class, and an addressable target. |
| `fixtures/ai/m1_composer_and_context_inspector_seed_cases/` | Failure-drill fixtures the composer and inspector replay; each fixture is owned by this lane and quoted by the proof packet. |
| `docs/ai/m1_composer_and_context_inspector_seed.md` | This reviewer landing page. |
| `artifacts/milestones/m1/proof_packets/composer_and_context_inspector_seed.md` | Proof packet anchored from the artifact index. |

## What the seed deliberately does **not** own

- Live model routing, dispatch, or spend. The route placeholder is pinned to
  `provider_class = disabled_no_provider_in_m1_seed` /
  `route_path_class = denied_by_policy_in_m1_seed` /
  `dispatch_target_class = disabled_no_dispatch_in_m1_seed`, and the
  inspector chrome shows the typed `dispatch_disabled` honesty marker on
  every route row.
- Autonomous apply flows. The composer has no apply method; the
  `remove_attachment` API is the only mutation the seed exposes, and it
  mutates the draft, not the workspace.
- The full ordered-section composition described in the prompt-composer
  contract. The M1 seed does not emit an
  `ai_context_assembly_record`; it captures the inputs that record would
  consume.
- Evidence-packet minting, route receipts, spend receipts. Those land
  later milestones; the M1 seed surface labels them as reserved rather
  than fabricating them.

## Protected walk

1. Open the launch AI wedge surface (the shell renders the prototype-label
   chip verbatim; the chip reads "M1 prototype seed — read-only, no model
   dispatch").
2. Add one workspace mention (resolves to a stable id) and one workspace
   slice attachment (Live). The composer's `validate()` outcome returns
   `state = dispatch_disabled_in_m1_seed` with the always-on
   `PolicyBlockedRoute` honesty marker as the only block reason.
3. Type a slash command (`/open-folder`) that resolves into the canonical
   `cmd:workspace.open_folder` registry entry. The inspector quotes the
   canonical command id verbatim instead of forking a bespoke AI-only
   alias.
4. Open the AI context inspector. Confirm every section renders with the
   stable ids:
   - `prototype_label`, `intent`, `mentions`, `attachments`,
     `slash_commands`, `route_placeholder`, `block_reasons`, `draft_state`.
   - The `route_placeholder` rows each carry the
     `dispatch_disabled` status.
   - The `draft_state` row reads `dispatch_disabled_in_m1_seed`.
5. Verify the composer and inspector quote the same `composer_draft_id`,
   `composer_session_id`, and `request_workspace_id`. Neither surface
   forks the addressing scheme.

Evidence:
`crates/aureline-shell/src/ai_context_inspector/tests.rs::snapshot_shape_is_stable_for_a_clean_draft`,
`crates/aureline-shell/src/ai_context_inspector/tests.rs::resolved_slash_command_quotes_canonical_command_id_from_seeded_registry`,
`crates/aureline-shell/src/ai_context_inspector/tests.rs::route_placeholder_renders_dispatch_disabled_marker_on_every_row`,
`crates/aureline-ai/src/composer/tests.rs::happy_path_draft_keeps_route_marker_but_no_actionable_blocks`.

## Failure drill

A draft accumulates one resolved workspace mention plus a pasted-text
attachment whose `trust_posture = untrusted_user_supplied`. The seed has no
fenced-tainted-data role yet, so the composer surfaces a typed
`TaintedAttachmentOutsideFencedSection` block reason against the offending
attachment id and the inspector renders both the attachment row and the
block-reason row as `Blocked` with the typed token.

Evidence:
`fixtures/ai/m1_composer_and_context_inspector_seed_cases/tainted_attachment_outside_fence.json`,
`crates/aureline-ai/src/composer/tests.rs::fixture_failure_drill_replays_the_tainted_outside_fence_block_reason`,
`crates/aureline-ai/src/composer/tests.rs::tainted_attachment_outside_fence_is_blocked_even_when_status_lies`,
`crates/aureline-shell/src/ai_context_inspector/tests.rs::tainted_attachment_failure_drill_lights_chip_and_addresses_the_attachment`.

Additional drills covered by the same suite:

- An unresolved mention surfaces with a typed
  `unresolved_not_found` block reason on the mention row.
- A stale workspace-slice attachment surfaces with a typed
  `stale_attachment` block reason; calling
  `ComposerDraft::remove_attachment` clears the row and the block.
- An over-budget aggregate attributes the typed `over_budget_context`
  block reason to the last attachment, giving the inspector a single
  addressable row to route the user to.
- A `/does-not-exist` slash-command invocation lands with the typed
  `unresolved_no_match` block reason and the slash-command row labels its
  status accordingly.
- An out-of-scope attachment surfaces with `out_of_scope_attachment`.

## Validation command

```
cargo test -p aureline-ai && cargo test -p aureline-shell --lib ai_context_inspector
```

## Closure recipe

The bounded wedge is live, the authority / lineage / claim limits stay
visible on every row, the failure drill catches regressions without
widening scope, and the typed honesty markers prevent silent dispatch
through future surfaces.

## Out of scope

Model-routing backends, autonomous apply, broad AI execution depth, and
product-policy work. Those land their own milestones; the M1 seed's job is
to make the composer / inspector legible without overstating what the
product can do today.
