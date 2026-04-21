# Command-descriptor fixtures

Worked fixtures for the command-descriptor contract frozen in
[`/docs/commands/command_descriptor_contract.md`](../../../docs/commands/command_descriptor_contract.md).
Every fixture here conforms to
[`/schemas/commands/command_descriptor.schema.json`](../../../schemas/commands/command_descriptor.schema.json).

The fixtures exist so the palette, application and context menus,
keybinding / shortcut-help layer, CLI help output, AI-tool surface,
automation-recipe surface, invocation-session audit, rollback, and
parity-diff lanes can write against a shared corpus without
inventing their own command dialect. Each file carries a
`__fixture__` section summarising the scenario, the axes it
exercises, and the contract sections it illustrates. The top-level
record itself conforms to the schema so tooling can validate the
file as an integration check.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/commands/command_descriptor.schema.json`](../../../schemas/commands/command_descriptor.schema.json).
  A fixture that fails validation is a bug in the fixture, not in
  the schema.
- **Projection-parity corpus.** A later parity audit between
  palette, menu, keybinding / shortcut help, CLI help, and AI-tool
  surfaces compares emitted rows for the same
  `command_id` / `command_revision_ref` field-for-field. These
  fixtures are the reference descriptors that audit reads.
- **Invocation-session replay.** Replay, audit, rollback, and
  support-export tooling reads the invocation-session packet
  fixtures as the envelope every call site is expected to mint.

## Required fields (per the contract)

A command descriptor MUST carry:

- `command_id`, `command_revision_ref`, `canonical_verb`,
  `primary_label_ref`,
- a complete `accessibility_label_path` (primary label,
  accessibility role, keyboard-shortcut narration ref),
- a `docs_help_anchor_ref` pinning a docs-pack anchor,
- a `shortcut_narration_hint` with bound / unbound narration refs,
- `aliases` (may be empty),
- an ordered `typed_arguments` list (may be empty),
- `capability_scope_class`, `preview_class`,
  `approval_posture_class`, `ai_tool_surfacing_class`,
  `palette_visibility`,
- `ui_slot_hints` (may be empty),
- `lifecycle_state`, `support_class`, `release_channel`,
  `declared_freshness_class`, a non-empty `client_scopes`,
- a `result_contract` block,
- `policy_context`, `redaction_class`, and `minted_at`.

An invocation-session packet MUST carry:

- `invocation_session_id`, `command_id`, `command_revision_ref`,
- `issuing_surface`, `authority_class`,
- a complete `argument_provenance_map` (one row per typed-argument
  slot, or the enablement decision is `disabled_with_reason /
  required_argument_unresolved`),
- a `context_snapshot`,
- an `enablement_decision` (typed `decision_class`; non-enabled
  decisions MUST carry a `disabled_reason_code` and a
  `repair_hook_ref`),
- a `preview_posture` (if the descriptor declared a non-
  `no_preview_required` class, `preview_shown` MUST be true before
  any non-denied outcome),
- an `approval_posture` (approval-required descriptors MUST carry
  an `approval_ticket_ref` once `approval_state` leaves
  `not_required`),
- `execution_intent`, `outcome`, `created_artifact_refs`,
  `evidence_refs`,
- `policy_context`, `redaction_class`, and `minted_at`.

## When a command is disabled, previewed, approved, or evidenced

The closed set of rules is frozen in the
[contract](../../../docs/commands/command_descriptor_contract.md).
In summary:

- **Preview-required.** `preview_class` other than
  `no_preview_required` forces the invocation-session packet to
  ship a preview before any non-denied outcome. Apply-direct denies
  with `preview_required_not_shown`.
- **Approval-required.** `approval_posture_class` other than
  `no_approval_required` forces an approval ticket; the packet
  walks through `approval_pending` → `approval_granted` /
  `approval_denied` / `approval_expired`. Denials emit
  `outcome_class = denied_by_approval` with
  `approval_denial_no_approval_path`.
- **Disabled-with-reason.** `enablement_decision.decision_class =
  disabled_with_reason` / `hidden_with_reason` carries a typed
  `disabled_reason_code` and a `repair_hook_ref`.
- **Result-evidence.** Successful invocations on non-read-only
  scopes MUST carry every `evidence_ref_class_required` entry
  from the descriptor's `result_contract`. Silent apply with no
  evidence is non-conforming.

## Fixtures

### Command descriptors (first-party)

- [`workspace_open_folder.json`](./workspace_open_folder.json) — the
  baseline reversible workspace command. Exercises five UI slot
  hints (palette, global application menu, explorer context menu,
  keybinding help, CLI help) and projects consistently across all
  of them.
- [`editor_format_document.json`](./editor_format_document.json) —
  reversible editor command with `preview_class =
  inline_summary_preview` and a mutation-journal result contract.
  Exercises the result-evidence rule on reversible-local mutations.
- [`search_find_in_workspace.json`](./search_find_in_workspace.json)
  — read-only query with `ai_tool_surfacing_class =
  ai_callable_read_only` and a `typed_value_returned` result
  contract. Exercises the AI-callable read-only path and argument
  kinds (`string_free_form`, `glob_expression`,
  `multi_value_list`, `integer_bounded`).
- [`git_push_branch.json`](./git_push_branch.json) — externally
  visible publish command with `preview_class =
  irreversible_publish_preview`, `approval_posture_class =
  second_party_review_required`, and `ai_tool_surfacing_class =
  ai_callable_externally_visible_mutation_requires_approval`.
  Exercises the AI-callable-externally-visible-mutation-requires-
  approval pairing rule.
- [`settings_edit_managed_policy.json`](./settings_edit_managed_policy.json)
  — policy-authoring command with `preview_class =
  policy_authoring_or_waiver_preview`, `approval_posture_class =
  admin_policy_approval_required`, `ai_tool_surfacing_class =
  not_ai_callable`, `palette_visibility = policy_restricted`, and
  `client_scopes = [managed_admin_surface]`. Exercises the
  preview-required + approval-required combination and
  trust-state-triggered argument pinning.
- [`workspace_reset_to_snapshot.json`](./workspace_reset_to_snapshot.json)
  — destructive bulk-mutation command with `preview_class =
  destructive_bulk_mutation_preview`, `capability_scope_class =
  irreversible_high_blast_mutation`, and `ai_tool_surfacing_class =
  ai_callable_irreversible_high_blast_denied`. Exercises the
  AI-denied posture on irreversible / high-blast scopes.

### Invocation-session packets

- [`invocation_session_open_folder_succeeded.json`](./invocation_session_open_folder_succeeded.json)
  — a succeeded invocation of `cmd:workspace.open_folder`. Shows
  the result-evidence state: the packet carries both a
  `mutation_journal_entry_ref` (required by the descriptor's
  `result_contract`) and an additional `audit_event_ref`.
- [`invocation_session_push_branch_approval_pending.json`](./invocation_session_push_branch_approval_pending.json)
  — an AI-initiated invocation of `cmd:git.push_branch` that
  rendered the preview, opened a second-party approval ticket,
  and minted the packet with `outcome_class =
  scheduled_pending_background`. Shows the approval-required
  state.
- [`invocation_session_edit_policy_disabled_by_trust.json`](./invocation_session_edit_policy_disabled_by_trust.json)
  — an invocation of `cmd:settings.edit_managed_policy` that
  denied with `decision_class = disabled_with_reason`,
  `disabled_reason_code = workspace_trust_restricted`, and a
  `request_admin_policy_change` repair hook. Shows the
  disabled-with-reason state.

## Related schemas and artifacts

- [`/schemas/governance/capability_lifecycle.schema.json`](../../../schemas/governance/capability_lifecycle.schema.json)
  — `lifecycle_state`, `support_class`, `release_channel`,
  `freshness_class`, `client_scope`, `redaction_class`, and
  `repair_hook_ref` vocabularies re-exported on the descriptor.
- [`/schemas/ux/interaction_safety.schema.json`](../../../schemas/ux/interaction_safety.schema.json)
  — `authority_class`, preview / apply / revert record shapes,
  permission prompt, and focus-return envelope the invocation-
  session packet cites via `preview_record_ref`,
  `permission_prompt_ref`, and `interaction_safety_packet_ref`.
- [`/schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json)
  — docs-pack manifest the `docs_help_anchor_ref` resolves into.
