# Interaction-safety worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ux/shell_interaction_safety_contract.md`](../../../docs/ux/shell_interaction_safety_contract.md)
and the schema at
[`/schemas/ux/interaction_safety.schema.json`](../../../schemas/ux/interaction_safety.schema.json).

Every file is a multi-document YAML stream. The first document is
a `__fixture__` prelude summarising the scenario, the ADR /
contract sections it exercises, and the packet / record kinds it
produces. The remaining documents are individual
`interaction_safety_packet_record`,
`preview_apply_revert_record`,
`batch_scope_record`,
`permission_prompt_record`,
`copy_export_representation_record`,
`focus_return_record`,
`responsive_fallback_record`, and
`interaction_safety_audit_event_record` instances that conform to
the schema.

No fixture embeds raw bodies, raw paths, raw URLs, raw prompt
text, raw credential material, or raw artifact bytes. Every such
field is an opaque ref.

## Cases

- [`destructive_bulk_rename_apply.yaml`](./destructive_bulk_rename_apply.yaml)
  — destructive core path. A review / diff canvas apply of a
  rename across 42 files with basis-drift detection, explicit
  included / blocked / hidden / query-derived batch summary,
  compact-shell responsive fallback that preserves every
  required visible field, focus return to the invoking review
  row, and a sanitized copy-export of the preview summary.
- [`extension_install_publish_capable.yaml`](./extension_install_publish_capable.yaml)
  — ecosystem / publish-capable / externally-mutating path. An
  install-update-attach canvas install of a verified-publisher
  extension with a workspace-scoped grant, a typed permission
  prompt that names the delta over a prior profile grant, a
  browser-handoff step-up renewal when the workspace trust
  state changes, a sanitized export of the publisher-identity
  row, and a returned-exact focus trajectory back to the
  install invoker.
- [`compact_shell_chrome_hid_required_field_denial.yaml`](./compact_shell_chrome_hid_required_field_denial.yaml)
  — responsive-fallback denial. A compact-shell rehearsal in
  which the narrow-width-sheet fallback dropped the
  `blocked_or_hidden_member_count` and `policy_source_label`
  required-visible-field classes on an `external_shared`
  consequence class; the surface denies the interaction with
  `chrome_hid_required_field` rather than committing against
  a collapsed chrome.
