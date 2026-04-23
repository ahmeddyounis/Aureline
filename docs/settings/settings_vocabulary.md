# Settings vocabulary

This document is the cross-surface companion to
[`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md).
The ADR freezes the decision; this document names the vocabulary every
non-resolver surface (settings UI, CLI `settings` command, command
palette, docs renderer, policy explainer, AI "Explain why" affordance,
search panel, mutation-journal renderer, profile import / export,
optional sync, schema registry, support-bundle exporter) uses when it
renders, logs, exports, or explains a setting.

If this document and the ADR disagree, the ADR wins and this document
must be updated in the same change.

## Artifacts this vocabulary points at

- [`/schemas/settings/setting_definition.schema.json`](../../schemas/settings/setting_definition.schema.json)
  — boundary schema for one setting-definition row. Names the
  canonical `setting_id`, `value_type`, `allowed_scopes`, `alias_set`,
  `migration_table`, `capability_dependencies`, `preview_class`,
  `restart_posture`, `lifecycle_label`, `redaction_class`, and docs
  strings. The eventual settings crate's Rust types are the schema of
  record; this file is the cross-tool boundary.
- [`/schemas/settings/effective_setting.schema.json`](../../schemas/settings/effective_setting.schema.json)
  — boundary schema for the effective-setting record and the
  structured settings audit event. One record flows between the
  resolver and every consumer surface.
- [`/schemas/settings/write_intent.schema.json`](../../schemas/settings/write_intent.schema.json)
  — boundary schema for the write-intent packet and the
  change-preview packet. Names the typed `reason_class`
  (`user_edit`, `profile_apply`, `import`, `sync`, `policy`,
  `automation`), the `scope_broadening_verdict`, the
  `checkpoint` block (rollback checkpoint and approval ticket
  refs), and the structured change-preview delta every
  preview, apply, and rollback surface reads.
- [`/docs/settings/schema_registry_seed.md`](./schema_registry_seed.md)
  — schema registry publishing contract. Names `$id` / version URI
  conventions, the unknown-field policy, the JSONC
  comment-preservation posture, the migration-alias retention
  window, the in-product row-anatomy mapping (source pill, lock /
  explain state, reset / diff affordance, deep-link and
  search-highlight behaviour), and the release-diff rules the
  eventual distribution service reads.
- [`/docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md)
  — optional-sync scope bundle and device-registry publishing
  contract. Names the device-record lifecycle (active / paused /
  revoked / forgotten), the scope-bundle shape and its
  omitted-classes denylist, the sync-session envelope, the
  field-aware sync-conflict packet, the local-authoritative degrade
  reasons, and the support / diagnostics projection rules every
  sync-aware surface reads.
- [`/artifacts/architecture/settings_tradeoff_rows.yaml`](../../artifacts/architecture/settings_tradeoff_rows.yaml)
  — machine-readable tradeoff register (nine axes, per-row reopen
  triggers) backing the ADR.
- [`/artifacts/settings/scope_precedence_rows.yaml`](../../artifacts/settings/scope_precedence_rows.yaml)
  — machine-readable matrix that binds each frozen scope to its
  precedence rank, intended content, writable-by posture,
  synchronisability, narrowability, and conformance tests.
- [`/fixtures/settings/setting_examples/`](../../fixtures/settings/setting_examples/)
  — six short fixtures exercising the scope set, the precedence order,
  the effective-setting record shape, the lock states, the denial
  reasons, the preview classes, and the control-stack fields.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — class-level registry the schema-registry and support-bundle
  exporter quote when a settings-related export, sync packet, or
  offboarding packet needs explicit retention/export posture instead of
  a private "retention class" label.

## Vocabulary surfaces share

Every settings-aware surface renders and logs the fields below using
the exact names the schemas export. Aliases, cute labels, or private
renames on protected surfaces are forbidden.

- `setting_id` — canonical id. Always shown in machine surfaces;
  human surfaces may render `source_label` + summary instead but MUST
  be able to surface the id on demand.
- `resolved_scope` — one of the ten frozen scopes. Drives the
  "show source" affordance.
- `source_label` — short source tag. Copied into the control-stack
  block and into every audit event.
- `lifecycle_label` — `experimental`, `beta`, `stable`,
  `deprecated`, or `retired`. Surfaces MUST render at least a
  lifecycle badge on non-`stable` values.
- `shadow_chain` — ordered list of contributing scopes. Rendered by
  the "show source" affordance and by support-bundle exports.
- `lock_state` / `lock_reason` — typed lock state and typed reason.
  Drives disabled-control affordances.
- `write_intent` / `write_denial_reason` — typed write intent and
  typed denial reason. Drives preview / apply UX and error copy.
- `restart_posture` — declared restart posture. Drives
  "restart required" prompts. A surface that silently requires a
  restart that was not declared is a bug.
- `preview_class` — `safe_apply`, `preview_required`,
  `rollback_checkpoint_required`,
  `rollback_checkpoint_and_approval_required`, or
  `managed_action_only`. Drives preview / rollback / approval flows.
- `capability_dependencies` — typed dependencies copied from the
  definition row with their observed state. Drives
  "this setting is unavailable because ..." affordances.
- `control_stack` — `source_label`, `lifecycle_label`,
  `last_refresh_at`, `expires_at`, `offline_fallback`,
  `explain_why_ref`, `control_authority`, `narrowing_ceiling_active`.
  Reserved for the feature-flag / experiment / schema-governance /
  admin-policy surfaces.
- `last_written` — monotonic stamp plus `actor_class`. Drives
  "last changed ..." affordances and audit records.
- `redaction_class` — ADR-0007 redaction class applied on export.
- `schema_version` — the setting-definition schema version the
  record conforms to. Distinct from `settings_schema_version`, which
  versions the envelope.

## Rules surfaces follow

1. **One registry.** Every surface reads from the setting-definition
   registry and the effective-setting records emitted by the
   resolver. Copy-only shadow settings are forbidden. If a new name
   is needed, add an alias row; do not mint a parallel row.
2. **No silent scope broadening.** Imported profiles, optional sync,
   and extension-API writes MAY narrow a workspace or user value;
   they MAY NOT silently widen trust, workspace permissions, AI
   egress, network egress, or managed entitlements. Widening writes
   MUST be denied with `scope_broadening_would_widen_trust`.
3. **Fail closed.** Denials are typed and visible. The resolver never
   silently writes a rejected value into a different scope and never
   silently accepts a widened value.
4. **Round-trip fidelity.** The resolver preserves unknown fields and
   user comments when round-tripping human-edited files. Unknown
   top-level settings are preserved on read, flagged with a typed
   `unknown_setting` validation event, and never silently discarded.
5. **Admin policy narrows, it does not widen.** Admin-policy
   narrowing is evaluated as an orthogonal ceiling after the nine
   value layers resolve. Policy MAY enforce an allowed value set,
   pin a value, or reduce a capability; policy MAY NOT widen beyond
   the layered value. When narrowing is active,
   `control_stack.narrowing_ceiling_active` is `true`.
6. **Session overrides never auto-promote.** A
   `session_or_command_override` value is discarded when the scope
   ends unless the user (or a migration action) explicitly promotes
   it to a durable scope.
7. **Show source everywhere.** Every surface that displays a setting
   MUST be able to render the `shadow_chain` and the
   `control_stack.explain_why_ref`. The "Explain why" affordance
   reads from these fields; it does not invent private reasoning.
8. **Redaction on export.** Every surface that exports a setting to
   a log, trace, support bundle, evidence packet, profile export,
   optional-sync payload, crash dump, mutation-journal entry, save
   manifest, replay / timeline capture, terminal transcript, or
   clipboard projection applies the setting's `redaction_class`
   per ADR-0007. `credential_alias` values carry aliases only.
9. **Schema-registry and support-export views quote record class.**
   Any generated-reference or support/export surface that emits a
   settings-related export, sync packet, delete packet, or offboarding
   packet MUST name the corresponding `record_class_id` from
   `record_class_registry.yaml`. Those surfaces MAY NOT invent a
   parallel "retention class", "export safe", or "offboarding class"
   vocabulary of their own.
10. **Mutation journal carries setting writes.** Every applied
   settings write records `setting_id`, `resolved_scope`,
   `source_label`, `write_intent`, `preview_class`, and
   `rollback_checkpoint_ref` (when the preview class required one).
   The mutation journal MUST NOT embed raw secret material or raw
   `credential_alias` secret payloads.
11. **Audit every observable action.** Every resolver action the
    user, an administrator, a support engineer, or a governance
    reviewer could ask about emits one of the frozen audit events.
    Audit events never carry raw secret material.

## Where related decisions live

- Identity modes and workspace trust:
  [`docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md).
- RPC transport across which effective-setting records and write
  intents cross:
  [`docs/adr/0004-rpc-transport-and-schema-toolchain.md`](../adr/0004-rpc-transport-and-schema-toolchain.md).
- Subscription envelope the effective-settings view rides on:
  [`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- Save pipeline, mutation-journal, and rollback-checkpoint plumbing:
  [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- Secret broker and `credential_alias` resolution:
  [`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md).
- Decision register row tracking `D-0014`:
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## Change management

- Adding a new scope, lifecycle label, denial reason, write intent,
  restart posture, preview class, control-authority class, or
  capability-dependency kind is additive-minor: bump
  `settings_schema_version`, add a row here, and extend the schemas.
- Repurposing any existing value (for example, reusing an existing
  denial reason for a different fail path) is breaking and requires
  a new decision row.
- Renaming a setting is done through an `alias_row` that redirects
  the legacy id to the canonical id; it is never done by mutating
  the existing definition row's `setting_id`.
- A lossy migration MUST create a rollback checkpoint before apply;
  a breaking type change MUST mint a new `setting_id` with a
  redirect alias rather than trying to migrate in place.
