# Command shareability, deep-link, and automation-safety fixtures

Worked fixtures for the shareability-metadata contract frozen in
[`/docs/commands/shareability_and_automation_contract.md`](../../../docs/commands/shareability_and_automation_contract.md).
Every fixture here conforms to
[`/schemas/commands/shareability_metadata.schema.json`](../../../schemas/commands/shareability_metadata.schema.json).

The fixtures exist so the command palette, application and
context menus, toolbar buttons, keybinding editor, keybinding
help overlay, CLI help output, AI-tool surface, automation-
recipe surface, docs reference page, and About / service-health
panel can project the same copy affordances, the same deep-link
gating contract, the same CLI / headless posture, the same
argument-inspection form, the same automation-safety cues, and
the same typed why-unavailable reasons across surfaces without
inventing parallel per-surface dialects. Each file carries a
`__fixture__` section summarising the scenario, the axes it
exercises, and the contract sections it illustrates. The
top-level record itself conforms to the schema so tooling can
validate the file as an integration check.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/commands/shareability_metadata.schema.json`](../../../schemas/commands/shareability_metadata.schema.json).
  A fixture that fails validation is a bug in the fixture, not
  in the schema.
- **Descriptor pairing.** Every shareability fixture MUST quote
  the `command_id`, `command_revision_ref`, and `canonical_verb`
  of a live command descriptor frozen in
  [`/fixtures/commands/command_descriptor_examples/`](../command_descriptor_examples/)
  (or of a descriptor that will ship with the extension / policy
  registry it belongs to). A shareability fixture whose identity
  does not resolve to a descriptor is a bug in the fixture.
- **Parity-audit corpus.** A later parity audit between palette,
  menu, toolbar, keybinding editor, CLI help, AI-tool surface,
  automation-recipe response, docs reference page, and About /
  service-health panel compares emitted rows for the same
  `command_id` / `command_revision_ref` field-for-field against
  the shareability record's `unavailability_disclosure.reason_set`
  and `automation_safety_cues`. These fixtures are the reference
  projections that audit reads.

## Required fields (per the contract)

A shareability record MUST carry:

- `command_id`, `command_revision_ref`, and `canonical_verb`
  re-exported from the command descriptor,
- a non-empty `copy_forms` list with at least
  `copy_canonical_command_id` and `copy_canonical_verb`,
- a `deep_link` block when `copy_forms` contains
  `copy_deep_link_ref`; every gating-requirement list is
  non-empty; the three `revalidates_*` constants are `true`,
- an `invocation_ref` block when the command is automation-
  callable, or `null` for `ui_only_interactive` commands,
- a `cli_equivalent` block (always present) with a
  `cli_equivalent_presence_class` and a `headless_mode` block,
- an `argument_inspection` block whose `inspectable_arguments`
  mirror the descriptor's `typed_arguments` order 1:1,
- a non-empty `automation_safety_cues` list,
- an `unavailability_disclosure` block whose `reason_set` pairs
  every reason with a `repair_hook_ref` and whose
  `parity_surfaces` list is non-empty,
- a `gating_contract` block with all eight constants set to
  `true`,
- a `policy_context`, a `redaction_class`, and a `minted_at`
  timestamp.

## When a command is unavailable, restricted, or UI-only

- **UI-only interactive.** `automation_safety_cues` contains
  `ui_only_interactive`; `invocation_ref` is `null`; the
  `unavailability_disclosure.reason_set` MUST contain
  `ui_only_command_not_representable_as_invocation`.
- **Approval-required.** `automation_safety_cues` contains
  `approval_required`; the `unavailability_disclosure.reason_set`
  MUST contain `approval_denial_no_approval_path`.
- **CLI-denied.** `cli_equivalent.presence_class =
  no_cli_equivalent_ui_only` forces `cli_verb_ref = null`.
- **Policy-blocked.** `automation_safety_cues` contains
  `policy_restricted` and / or `managed_only`; the
  `unavailability_disclosure.reason_set` typically surfaces
  `capability_disabled_by_policy`,
  `client_scope_excludes_surface`, and
  `managed_only_channel_required`.
- **Extension-provided.** `automation_safety_cues` contains
  `extension_invocation_only`; `deep_link.gating_requirements`
  includes `publisher_admission_required`.

## Fixtures

- [`workspace_open_folder.shareability.json`](./workspace_open_folder.shareability.json)
  — **Core command.** Baseline first-party `workspace.open_folder`
  shareability with every copy form, a `reveal_command_in_palette`
  deep link, a full invocation ref, a CLI-on-every-permitted-scope
  equivalent, `fully_headless_safe` headless mode, and the
  `[macro_safe, recipe_safe, headless_safe]` cue set. Parity
  surfaces cover every surface in the vocabulary.
- [`extension_markdown_render_preview.shareability.json`](./extension_markdown_render_preview.shareability.json)
  — **Extension command.** Extension-supplied command whose deep
  link re-runs `publisher_admission_required` and
  `kill_switch_revalidation_required`; invocation ref seeds an
  `extension_supplied_argument_ref`; cue set carries
  `extension_invocation_only`; reason set surfaces
  `publisher_not_permitted` alongside the standard lifecycle
  reasons.
- [`settings_edit_managed_policy.shareability.json`](./settings_edit_managed_policy.shareability.json)
  — **Policy-blocked command.** `client_scopes =
  [managed_admin_surface]` with `ai_tool_surfacing_class =
  not_ai_callable`; `invocation_ref = null`;
  `cli_equivalent.presence_class =
  cli_equivalent_denied_by_client_scope`;
  `headless_mode_class = denied_in_headless_mode`; cue set carries
  `[managed_only, policy_restricted, requires_trusted_workspace,
  preview_required_before_apply, approval_required,
  never_ai_callable]`; reason set surfaces
  `capability_disabled_by_policy`, `client_scope_excludes_surface`,
  `cli_surface_not_in_client_scopes`,
  `ai_tool_surface_not_in_client_scopes`,
  `managed_only_channel_required`, and
  `approval_denial_no_approval_path`.
- [`ui_show_welcome_tour.shareability.json`](./ui_show_welcome_tour.shareability.json)
  — **UI-only interactive command.** `invocation_ref = null`;
  `cli_equivalent.presence_class = no_cli_equivalent_ui_only`;
  `headless_mode_class = denied_in_headless_mode`; cue set carries
  `[ui_only_interactive, never_ai_callable,
  requires_trusted_workspace]`; reason set surfaces
  `ui_only_command_not_representable_as_invocation`,
  `cli_surface_not_in_client_scopes`,
  `ai_tool_surface_not_in_client_scopes`, and
  `argument_inspection_requires_interactive_prompt`.
- [`git_push_branch.shareability.json`](./git_push_branch.shareability.json)
  — **Approval-gated remote / network command.** Deep link
  resolves into `prefill_command_invocation_preview` with
  `preview_path_required`, `approval_path_required`, and
  `credential_broker_handshake_required` in the gating set;
  invocation ref carries a `policy_pinned_argument_ref` template
  entry for `force_with_lease`; headless mode is
  `headless_safe_with_declared_prompt_elision` with two declared
  elisions; cue set carries `[recipe_safe, headless_safe,
  approval_required, preview_required_before_apply,
  requires_trusted_workspace]`; reason set surfaces
  `approval_denial_no_approval_path`,
  `preview_required_not_shown`,
  `deep_link_gating_requirement_failed`,
  `required_provider_unlinked`, `required_credential_missing`,
  and `workspace_trust_restricted`.

## Related schemas and artifacts

- [`/schemas/commands/command_descriptor.schema.json`](../../../schemas/commands/command_descriptor.schema.json)
  — the descriptor every shareability record extends.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../../schemas/governance/capability_lifecycle.schema.json)
  — `repair_hook_ref`, `redaction_class`, and `freshness_class`
  vocabularies re-exported on the shareability record.
- [`/schemas/commands/ui_slot_taxonomy.schema.json`](../../../schemas/commands/ui_slot_taxonomy.schema.json)
  plus
  [`/fixtures/commands/ui_slot_taxonomy_examples/`](../ui_slot_taxonomy_examples/)
  — the UI slot taxonomy the `unavailability_parity_surface`
  vocabulary projects against.
- [`/docs/commands/command_parity_diff.md`](../../../docs/commands/command_parity_diff.md)
  — consumes `unavailability_disclosure.parity_surfaces` and
  `automation_safety_cues` mechanically.
