# Disabled-reason vocabulary (canonical)

This page is the reviewer-facing entrypoint for the canonical
`disabled_reason_code` vocabulary used by the command enablement engine and by
invocation packets when a command is denied.

The authoritative contract is
`docs/commands/command_descriptor_contract.md` (the “Disabled-reason vocabulary”
section) and the schema source of truth is
`schemas/commands/command_descriptor.schema.json#/$defs/disabled_reason_code`.

## What the code represents

`disabled_reason_code` is a *machine reason*:

- Shell surfaces (palette, menus, keybinding help/editor, diagnostics) render it
  as a compact “why unavailable” chip and route to a typed repair hook.
- Exports and parity audits use it as the stable join key; surfaces MUST NOT
  invent per-surface reason strings.

## Vocabulary

The admissible values are:

- `workspace_trust_restricted`
- `capability_lifecycle_retired`
- `capability_disabled_by_policy`
- `kill_switch_tripped`
- `client_scope_excludes_surface`
- `freshness_floor_unmet`
- `required_provider_unlinked`
- `required_credential_missing`
- `required_argument_unresolved`
- `execution_context_unavailable`
- `managed_only_channel_required`
- `dependency_state_below_command_ceiling`
- `command_deprecated_within_window`
- `command_retired`
- `command_version_unknown`
- `preview_denial_no_safe_preview`
- `approval_denial_no_approval_path`
- `publisher_not_permitted`
- `policy_blocked_in_context`
- `authority_class_unresolved`
- `issuing_surface_unresolved`
- `scope_class_drifted_from_descriptor`
- `preview_required_not_shown`
- `basis_snapshot_drifted`

## Repair hook rule

When an enablement decision is not `enabled`, the decision MUST carry both the
typed `disabled_reason_code` and a `repair_hook_ref`. A surface that renders
“disabled” with no typed reason or no repair hook is non-conforming.

## Alpha cause-family mapping

The alpha command-descriptor baseline adds a reviewer-facing cause-family layer
without minting a second machine vocabulary. The manifest at
`fixtures/commands/disabled_reason_alpha/manifest.json` maps:

- focus and selection gaps to `required_argument_unresolved`
- lifecycle state gaps to `command_retired`
- missing dependency gaps to `required_provider_unlinked`
- policy gaps to `policy_blocked_in_context`
- entitlement gaps to `managed_only_channel_required`
- remote or host mismatch gaps to `client_scope_excludes_surface`

Surfaces may localize the cause-family copy, but exports and parity packets
continue to join on the canonical `disabled_reason_code`.
