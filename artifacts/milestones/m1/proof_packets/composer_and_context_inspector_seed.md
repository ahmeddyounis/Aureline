# Proof packet: AI composer and context-inspector seed

Purpose: anchor proof captures for the M1 bounded prototype wedge that lands
the first-pass AI composer and context inspector on the launch AI wedge.
The seed is read-only for mutation, never dispatches a model, and projects
its addressable axes through the shared canonical command registry and the
shared execution-context object rather than forking AI-only truth.

Reviewer landing page:
[`docs/ai/m1_composer_and_context_inspector_seed.md`](../../../../docs/ai/m1_composer_and_context_inspector_seed.md).

Canonical sources:

- Crate: `crates/aureline-ai/`
  - `src/lib.rs` — public re-exports
  - `src/composer/mod.rs` — `ComposerDraft` record, mention / attachment /
    slash-command / route-placeholder vocabulary, `validate()` outcome,
    block-reason enum.
- Crate (consumer): `crates/aureline-shell/`
  - `src/ai_context_inspector/mod.rs` — `AiContextInspectorSnapshot`
    projection, section / row / action vocabulary, deterministic plaintext
    render.
- Reviewer landing page:
  `docs/ai/m1_composer_and_context_inspector_seed.md`
- Failure-drill fixture:
  `fixtures/ai/m1_composer_and_context_inspector_seed_cases/tainted_attachment_outside_fence.json`
- Tests (named-consumer wiring):
  - `crates/aureline-ai/src/composer/tests.rs`
  - `crates/aureline-shell/src/ai_context_inspector/tests.rs`

Upstream contracts the seed projects against (without forking):

- `docs/commands/command_descriptor_contract.md` /
  `crates/aureline-commands/` — slash commands resolve into the canonical
  registry.
- `docs/runtime/execution_context_seed.md` /
  `crates/aureline-runtime/` — execution-context identity is reused; the
  composer does not re-derive target / cwd / toolchain truth.
- `docs/ai/prompt_composer_contract.md` /
  `docs/ai/context_assembly_contract.md` — frozen mention / attachment /
  trust-posture / source-class / block-reason vocabularies. The M1 seed
  covers a small, honest subset and grows additively without forking.

## Protected walk

Open the launch AI wedge against a draft with one resolved workspace
mention, one Live workspace-slice attachment, and one `/open-folder` slash
command that resolves to the canonical `cmd:workspace.open_folder`
registry entry. Confirm:

- the prototype label chip reads "M1 prototype seed — read-only, no model
  dispatch";
- the route placeholder rows each carry the `dispatch_disabled` status and
  quote the typed `disabled_no_provider_in_m1_seed`,
  `denied_by_policy_in_m1_seed`, and `disabled_no_dispatch_in_m1_seed`
  tokens verbatim;
- the draft state row reads `dispatch_disabled_in_m1_seed`;
- the composer and inspector quote the same `composer_draft_id`,
  `composer_session_id`, and `request_workspace_id`;
- the slash-command row quotes the canonical `cmd:workspace.open_folder`
  registry id rather than a bespoke AI-only alias.

Evidence:
`crates/aureline-shell/src/ai_context_inspector/tests.rs::snapshot_shape_is_stable_for_a_clean_draft`,
`crates/aureline-shell/src/ai_context_inspector/tests.rs::resolved_slash_command_quotes_canonical_command_id_from_seeded_registry`,
`crates/aureline-shell/src/ai_context_inspector/tests.rs::route_placeholder_renders_dispatch_disabled_marker_on_every_row`,
`crates/aureline-shell/src/ai_context_inspector/tests.rs::draft_state_section_quotes_dispatch_disabled_label_for_m1_seed`,
`crates/aureline-ai/src/composer/tests.rs::happy_path_draft_keeps_route_marker_but_no_actionable_blocks`,
`crates/aureline-ai/src/composer/tests.rs::slash_command_resolves_against_seeded_registry_or_records_unresolved_state`.

## Failure drill

Add a pasted-text attachment with `trust_posture = untrusted_user_supplied`
that is not placed under a fenced-tainted-data role. The composer's
`validate()` outcome surfaces the typed
`TaintedAttachmentOutsideFencedSection` block reason against the offending
attachment id; the inspector renders both the attachment row and the
block-reason row as `Blocked` and points the `has_tainted_attachments`
chip on the snapshot so the wedge cannot silently include the bytes.

Evidence:
`fixtures/ai/m1_composer_and_context_inspector_seed_cases/tainted_attachment_outside_fence.json`,
`crates/aureline-ai/src/composer/tests.rs::fixture_failure_drill_replays_the_tainted_outside_fence_block_reason`,
`crates/aureline-ai/src/composer/tests.rs::tainted_attachment_outside_fence_is_blocked_even_when_status_lies`,
`crates/aureline-shell/src/ai_context_inspector/tests.rs::tainted_attachment_failure_drill_lights_chip_and_addresses_the_attachment`.

Adjacent failure drills covered by the same suite:

- `crates/aureline-ai/src/composer/tests.rs::unresolved_mention_blocks_the_draft`
- `crates/aureline-ai/src/composer/tests.rs::out_of_scope_attachment_records_block_reason_addressable_by_id`
- `crates/aureline-ai/src/composer/tests.rs::over_budget_aggregate_attributes_to_the_last_attachment`
- `crates/aureline-ai/src/composer/tests.rs::remove_attachment_clears_block_and_drops_the_row`
- `crates/aureline-shell/src/ai_context_inspector/tests.rs::unresolved_mention_renders_with_blocked_status_and_routes_to_mention_id`
- `crates/aureline-shell/src/ai_context_inspector/tests.rs::unresolved_slash_command_renders_blocked_with_typed_reason`

## Validation command

```
cargo test -p aureline-ai && cargo test -p aureline-shell --lib ai_context_inspector
```

## Evidence storage

- Crate sources: `crates/aureline-ai/`,
  `crates/aureline-shell/src/ai_context_inspector/`
- Reviewer doc: `docs/ai/m1_composer_and_context_inspector_seed.md`
- Failure-drill fixture:
  `fixtures/ai/m1_composer_and_context_inspector_seed_cases/`
- Tests: `crates/aureline-ai/src/composer/tests.rs`,
  `crates/aureline-shell/src/ai_context_inspector/tests.rs`
