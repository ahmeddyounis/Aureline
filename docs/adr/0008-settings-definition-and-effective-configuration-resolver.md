# ADR 0008 — Settings definition, effective-configuration resolver, and control-stack vocabulary

- **Decision id:** D-0014 (see `artifacts/governance/decision_index.yaml#D-0014`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-08-15
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council
- **Related requirement ids:** none

## Context

Every surface that exposes, edits, explains, imports, exports, syncs,
or supports a setting has to agree on the same answer to four
questions: what is the active value, which source won it, why can a
write be denied, and what happens after apply or restart. If the UI
settings panel, the JSONC file, the CLI `settings inspect`, the
command palette, the policy explainer, the import / migration flow,
the optional sync path, the support-bundle exporter, and the
extension SDK each invent their own idea of a setting's identity, its
declared type, its allowed scopes, its precedence order, or its
provenance fields, the product cannot honestly promise that two
surfaces are describing the same configuration. The PRD
(`.t2/docs/Aureline_PRD.md:1125`) and the TAD
(`.t2/docs/Aureline_Technical_Architecture_Document.md:1699`) both
freeze a layered configuration model as a first-order product
contract; this ADR closes the stable vocabulary for that contract
before settings logic spreads across any of those surfaces.

The freeze matters now, ahead of the settings UI, the settings CLI,
the profile import and export lanes, the optional sync path, the
schema-registry lane, and the support-export lane landing: if those
lanes proliferate before a shared settings vocabulary is frozen, each
will invent its own `setting_id` shape, its own scope list, its own
precedence rule, and its own provenance record; import, migration,
sync, and support work would then land with incompatible assumptions
about which scope wins, when a lock blocks a write, how a restart is
signalled, how an alias redirects a legacy key, and which fields
reach the control-stack surfaces that the feature-flag / experiment
lane and the schema-governance lane later consume. This ADR closes
`D-0014` (settings definition, effective-configuration resolver, and
control-stack vocabulary) so those lanes can instrument against one
contract.

The settings resolver rides alongside the ADR-0001 identity modes
(managed, self-hosted, account-free local) whose trust state caps
write intent, the ADR-0004 RPC transport (no raw secret bodies cross
the wire; setting values that carry references do so through
`credential_handle` aliases only), the ADR-0005 subscription envelope
(the effective-settings view is a subscription whose authority class
is `policy_entitlement` and whose frames carry the shared freshness /
completeness vocabulary), the ADR-0006 VFS save contract (the text
artifacts that hold settings are saved through the same compare-
before-write pipeline, and durable-state migration uses the same
mutation-journal record shape as other workspace writes), and the
ADR-0007 secret broker (setting values that look like secrets are
alias references resolved by the broker; raw bytes never appear in
any settings artifact). This ADR does not redefine those contracts;
it defines the settings-specific fields they refer to. Full
settings UI, live resolver implementation, the managed sync service,
and the schema-registry distribution service are explicitly out of
scope; this freeze establishes the vocabulary and invariants those
later lanes will honour.

## Decision

Aureline freezes a single **settings contract**: every setting has
one stable **setting id**, a declared **value type**, a frozen
**allowed-scope set**, a frozen **precedence order**, a stable
**alias and migration table**, and one **effective-setting record**
shape that every surface (UI, JSONC files, CLI, palette, import,
migration, optional sync, schema registry, docs, policy explainer,
AI "explain why" affordance, support-bundle exporter, mutation
journal) reads. The effective-setting record carries the **shadow
chain** (every scope that contributed, in order, with the winning
source labelled), the **lock / constraint state**, the
**write-intent denial reason** when a write is refused, the
**restart posture**, the **capability dependencies**, and the
**control-stack** fields (source label, lifecycle label,
last-refresh stamp, expiry, local-offline fallback, explain-why
affordance). Preview-class settings and rollback-checkpoint settings
declare those postures in their definition row; the resolver enforces
them uniformly. No surface may invent a "copy-only shadow setting"
that duplicates a resolved setting with a different key or a
different precedence.

All rules below are stated in terms of contract, vocabulary, and
record names rather than specific crates so adapter changes are
hygiene, not re-litigation.

### Stable setting identity

Every setting lives under one canonical id. Aliases redirect; they
do not mint new identities.

- **`setting_id`** — canonical, stable, dotted lower-snake path
  (for example, `editor.tab_size`, `terminal.cursor_blink`,
  `ai.default_provider.alias`). Pattern:
  `^[a-z][a-z0-9_]*(?:\.[a-z][a-z0-9_]*)+$`. Stable once the setting
  leaves `experimental` lifecycle; may not be repurposed after that
  point.
- **`alias_set`** — ordered list of prior ids that resolve to this
  setting. Every alias carries `from_id`, `since_version`,
  `deprecated_in_version`, `removal_target_version`, and an
  `alias_direction` that is always `redirect_to_canonical`. Aliases
  are additive-minor; repurposing an alias to mean a different
  setting is breaking and requires a new decision row.
- **`migration_table`** — per-version records that describe how a
  prior value shape translates into the current one
  (`from_version`, `to_version`, `transform_class`: `identity`,
  `narrow_enum`, `widen_enum_additive`, `split_field`, `merge_field`,
  `rename_field`, `change_default_only`, `type_change_breaking`).
  `type_change_breaking` requires a new `setting_id`, not a
  migration row, so rename-in-place is always safe.
- Stable setting ids flow identically through the UI panel, JSONC
  files, CLI output, command palette, docs, profile export, optional
  sync, mutation journal, and support bundle. Surfaces MUST NOT
  invent display-only ids that diverge from the canonical id.

### Declared value type

Every setting row declares one typed value shape. Surfaces read the
shape from the setting-definition registry, not from the value.

| Type class            | Frozen constraints                                                             |
|-----------------------|--------------------------------------------------------------------------------|
| `bool`                | `true` / `false`. No truthy-string coercion.                                   |
| `integer`             | Inclusive `min` / `max` where declared; overflow fails validation.             |
| `float`               | Inclusive `min` / `max` where declared; NaN forbidden.                         |
| `string`              | Optional regex pattern, optional length bounds.                                |
| `enum`                | Closed set declared in the definition row. Widening the enum is additive-minor.|
| `duration`            | ISO-8601-style string; resolver canonicalises to monotonic units.              |
| `path`                | Path string. Paths referencing workspace roots use the ADR-0006 identity layers.|
| `uri`                 | Stable URI string. Resolver does not auto-rewrite.                             |
| `object`              | Closed property map declared in the definition row.                            |
| `array_of<T>`         | Ordered list of a declared element type. `dedup_mode` is declared per row.     |
| `tagged_union`        | Discriminated variant set; discriminator field is declared.                    |
| `credential_alias`    | Alias string resolved by the ADR-0007 secret broker; no raw secret material ever. |
| `feature_flag_state`  | One of `off`, `on`, `off_by_policy`, `on_by_policy`. Resolver binds to the control stack. |
| `color_token`         | Design-token id resolved against the design-system registry.                   |

Unknown fields inside `object` and `tagged_union` values are
preserved where possible (per PRD `.t2/docs/Aureline_PRD.md:1209`);
unknown top-level settings are preserved on read, surfaced as a
typed `unknown_setting` validation event, and never silently
discarded.

### Allowed scopes (frozen)

Every setting row declares the set of scopes in which the setting
MAY be written. The resolver rejects writes from scopes outside the
declared set with the frozen denial reason
`scope_not_allowed_for_setting`. Broadening a setting's allowed-scope
set is additive-minor; narrowing a previously allowed scope is
breaking and requires a new decision row. Silent scope broadening
(for example, letting a user-global value win a setting whose
allowed-scope set is `workspace`-only) is forbidden.

| Scope id                         | Meaning                                                                                       |
|----------------------------------|-----------------------------------------------------------------------------------------------|
| `built_in_default`               | Ship-time default compiled into the product. Always readable; never user-writable.            |
| `channel_or_experiment_default`  | Release-channel or experiment default declared in the control stack.                          |
| `imported_profile_default`       | Default carried by an imported profile artifact.                                              |
| `user_global`                    | User-level value in `$AURELINE_CONFIG/settings.jsonc` or equivalent.                          |
| `machine_specific`               | Machine-bound value held locally and never synced by default.                                 |
| `workspace`                      | Value held in a workspace manifest or workspace settings file.                                |
| `folder_or_module_override`      | Override scoped to a directory or module inside the workspace.                                |
| `language_override`              | Override keyed on the active language / semantic domain.                                      |
| `session_or_command_override`    | Session-scoped or command-scoped override; discarded when the scope ends unless promoted.     |
| `admin_policy_narrowing`         | Admin-policy scope. Narrows lower layers; never merged as user preference. See below.         |

### Precedence order (frozen)

The resolver evaluates scopes in the order below. Later entries
override earlier ones for the setting's **value**; the
`admin_policy_narrowing` scope is orthogonal and caps the effective
value rather than replacing it (see "Lock and constraint state").

1. `built_in_default`
2. `channel_or_experiment_default`
3. `imported_profile_default`
4. `user_global`
5. `machine_specific`
6. `workspace`
7. `folder_or_module_override`
8. `language_override`
9. `session_or_command_override`

Admin policy is evaluated as a **narrowing ceiling** after the nine
value layers resolve: it MAY enforce a constrained value set, mark
the setting locked, or reduce a capability, but it MAY NOT widen
trust, permissions, AI egress, network egress, or workspace-scope
beyond the layered value. This mirrors the PRD
(`.t2/docs/Aureline_PRD.md:1147`) and TAD
(`.t2/docs/Aureline_Technical_Architecture_Document.md:1747`) rule
that workspace and imported values may narrow but not silently widen
trust and that admin policy never merges as user preference. The
resolver records the narrowing explicitly; surfaces render it as a
`policy_locked` or `policy_constrained` state with the
admin-policy source label.

### Effective-setting record (frozen)

Every surface that renders or quotes a setting reads one record
shape. The record is the only truth that flows between the resolver
and any consumer (UI, CLI, command palette, docs, migration,
import, optional sync, support-bundle exporter, mutation journal,
schema registry).

Required fields:

- `setting_id` — canonical id.
- `value` — current effective value (typed per the definition row;
  `credential_alias` values never carry raw secret bytes).
- `resolved_scope` — the scope that won (one of the ten frozen
  scopes; `admin_policy_narrowing` only appears here when the
  narrowing replaced the layered value with the declared floor).
- `source_label` — human-readable source tag used by `show source`
  affordances. Surfaces MUST NOT infer this from the raw path.
- `lifecycle_label` — one of `experimental`, `beta`, `stable`,
  `deprecated`, `retired`.
- `shadow_chain` — ordered list of contributing scopes, in
  precedence order, each carrying:
  - `scope` — scope id.
  - `value_present` — whether a value existed in that scope.
  - `value_preview` — preview of that scope's value (redacted per
    the setting's declared redaction class; never raw secret
    material).
  - `source_label` — that scope's source tag.
  - `winner` — boolean; exactly one entry is `true` unless the
    resolver returned a degraded default.
- `lock_state` — one of `unlocked`, `policy_locked`,
  `policy_constrained`, `capability_locked`, `read_only_surface`.
- `lock_reason` — typed; see the lock / constraint table below.
- `write_intent` — for surfaces that preview a pending write:
  `allowed`, `allowed_with_restart`, `allowed_with_preview`,
  `allowed_with_rollback_checkpoint`,
  `allowed_requires_approval_ticket`, `denied`.
- `write_denial_reason` — typed; see the denial-reason table below.
- `restart_posture` — `no_restart`, `reload_view`,
  `reload_workspace`, `restart_extensions`, `restart_process`,
  `reopen_workspace`. Preview-class settings that require restart
  declare it here so the UI surfaces the prompt before apply.
- `preview_class` — `safe_apply`, `preview_required`,
  `rollback_checkpoint_required`,
  `rollback_checkpoint_and_approval_required`,
  `managed_action_only`.
- `capability_dependencies` — declared in the definition row;
  copied into the effective record when the setting is gated
  (ordered list of `feature_flag_state` references, identity-mode
  requirements, trust-state minima, policy-epoch minima).
- `control_stack` — frozen control-stack block (see below).
- `last_written` — monotonic stamp of the most recent write that
  produced this value, plus the actor class
  (`user_keystroke`, `user_command`, `imported_profile`,
  `workspace_migration`, `admin_policy_injector`,
  `experiment_rollout`, `session_override`, `extension_api`,
  `ai_apply`).
- `schema_version` — the setting-definition schema version the
  record conforms to (copied from the registry).
- `redaction_class` — the ADR-0007 redaction class the setting's
  value defaults to when it reaches an exportable surface.

No surface may invent additional top-level fields. Adding a new
top-level field is an additive-minor schema change; repurposing
an existing field is breaking.

### Lock and constraint state (frozen)

Writes can be denied for reasons other than validation. Every
denial is typed, visible, auditable, and repairable.

| Lock state              | Meaning                                                                                                               |
|-------------------------|-----------------------------------------------------------------------------------------------------------------------|
| `unlocked`              | Normal; layered precedence governs the value.                                                                         |
| `policy_locked`         | Admin policy pins the value; the user may inspect but not override.                                                   |
| `policy_constrained`    | Admin policy declares an allowed value set that caps user overrides; user values outside the set are denied.         |
| `capability_locked`     | The setting's declared capability dependency is unmet (missing feature flag, insufficient trust, wrong identity mode, stale policy epoch). |
| `read_only_surface`     | The current surface cannot write this setting (for example, the CLI renders a workspace-only setting from a user-global surface). |

Denial reasons (frozen):

| Denial reason                             | Fires when                                                                                             |
|-------------------------------------------|--------------------------------------------------------------------------------------------------------|
| `scope_not_allowed_for_setting`           | Write targets a scope outside the setting's declared allowed-scope set.                                |
| `scope_broadening_would_widen_trust`      | Write would silently widen workspace trust, AI egress, network egress, or permissions.                 |
| `policy_locked_value`                     | Admin policy pins the value; user write is refused.                                                    |
| `policy_constrained_value`                | Admin policy forbids the proposed value (not in the allowed set).                                      |
| `capability_dependency_unmet`             | Required feature flag, identity mode, trust state, or policy epoch is not satisfied.                   |
| `preview_required_not_acknowledged`       | Setting is `preview_required` and no preview has been confirmed.                                       |
| `rollback_checkpoint_not_created`         | Setting is `rollback_checkpoint_required` and no checkpoint exists.                                    |
| `approval_ticket_missing`                 | Setting requires a runtime approval ticket (see ADR-0007) that is not present.                         |
| `restart_required_not_acknowledged`       | Setting is `restart_required` and the user has not confirmed the restart.                              |
| `validation_failed`                       | Proposed value fails type, enum, regex, range, or object-shape validation.                             |
| `setting_unknown_at_registry`             | No setting-definition row matches the id (and no alias redirects).                                     |
| `setting_retired`                         | Setting has been retired; writes are refused and reads return the frozen retired-value notice.         |
| `managed_mode_only`                       | Setting is only settable under the managed identity mode.                                              |
| `read_only_surface`                       | Current surface is not permitted to write this setting.                                                |

Denials fail closed. The resolver MUST NOT silently downgrade to
writing into a different scope, MUST NOT silently accept a widened
value, and MUST emit a typed `settings_write_denied` audit event.

### Validation posture (frozen)

Validation runs on read and write. Rules:

- The declared type, range, regex, and enum set are authoritative.
  Surfaces MUST NOT invent additional type coercions.
- Unknown fields inside `object` and `tagged_union` values are
  preserved where possible; they surface as `unknown_field`
  diagnostics but do not block other fields.
- Unknown top-level settings are preserved on read, flagged with a
  typed `unknown_setting` validation event, and may be removed only
  with an explicit user or migration action.
- A value that fails validation does not overwrite the previously
  resolved value. The resolver records a `validation_failed`
  diagnostic, retains the last-known-valid value, and surfaces a
  repair affordance.
- Import and optional sync paths run the same validation pipeline
  as interactive writes; they MAY NOT bypass validation.
- Schema-registry evolution is additive-preferred; type changes
  that cannot be expressed as a migration row require a new
  `setting_id`.

### Write-intent reasons (frozen)

Write-intent reasons gate preview / apply UX. They sit alongside the
denial reasons and describe the non-denial path.

| Write intent                                    | Meaning                                                                                |
|-------------------------------------------------|----------------------------------------------------------------------------------------|
| `allowed`                                       | Write is safe to apply immediately.                                                    |
| `allowed_with_restart`                          | Write is allowed; restart posture names which restart fires on apply.                  |
| `allowed_with_preview`                          | Write is allowed only after a preview is acknowledged.                                 |
| `allowed_with_rollback_checkpoint`              | Write is allowed; resolver creates a rollback checkpoint before apply.                 |
| `allowed_with_rollback_checkpoint_and_approval` | Write is allowed; both a rollback checkpoint and an ADR-0007 approval ticket are required. |
| `allowed_requires_approval_ticket`              | Write requires an ADR-0007 approval ticket but no rollback checkpoint.                 |
| `denied`                                        | Write is denied; the denial reason describes why.                                      |

### Restart posture (frozen)

The resolver declares one restart posture per setting. Surfaces
render it before apply; the write intent repeats it for consumers
who read only the effective-setting record.

- `no_restart` — apply is immediate.
- `reload_view` — affected views re-render; no process restart.
- `reload_workspace` — workspace re-opens; buffers and indexes
  re-warm with the ADR-0005 freshness vocabulary.
- `restart_extensions` — extension hosts bounce; core editing
  survives.
- `restart_process` — app process restarts; session state is
  preserved per ADR-0003 durability rules.
- `reopen_workspace` — workspace must be explicitly reopened
  (for example, root-set change).

A setting that silently requires a restart it did not declare is a
bug; the resolver emits a `restart_posture_mismatch` audit event
and the conformance corpus catches the regression.

### Capability dependencies (frozen)

Every setting row MAY declare capability dependencies. The resolver
evaluates them at read time and sets `capability_locked` when any
dependency is unmet. Dependencies are typed:

- `feature_flag_required` — named flag must be `on` or `on_by_policy`.
- `identity_mode_required` — one of `account_free_local`,
  `self_hosted_org`, `managed_convenience` (ADR-0001).
- `trust_state_minimum` — one of `restricted`, `trusted`
  (ADR-0001).
- `policy_epoch_minimum` — integer policy epoch; earlier epochs
  lock the setting until the epoch rolls forward.
- `workspace_capability` — declared capability in the ADR-0006
  root-capability envelope (for example,
  `supports_atomic_replace`).
- `extension_capability` — declared capability from the
  extension-host registry (for example,
  `language_server_available`).
- `credential_handle_class` — ADR-0007 secret-class whose handle
  MUST resolve for the setting to apply (used for
  `credential_alias` settings).

Resolver behaviour: the effective-setting record's `value` reflects
the last-known-valid value, but `lock_state` is set to
`capability_locked` and `write_denial_reason` is set to
`capability_dependency_unmet` until the dependency resolves. This
avoids the silent-downgrade failure mode where a setting appears
active but the underlying capability is missing.

### Migration alias tables (frozen)

Migration rows describe how a prior setting shape resolves today.
The resolver applies migrations on read without rewriting the
user's artifact unless the user (or a migration action) explicitly
saves. This preserves round-trip fidelity (unknown fields, user
comments) per the PRD's rule that settings files must not churn
(`.t2/docs/Aureline_PRD.md:1209`).

Migration rules:

- Every migration row carries `from_version`, `to_version`,
  `from_shape`, `to_shape`, `transform_class`, `is_lossy`,
  `rollback_supported`, and an optional `divergence_marker` for
  values that cannot be represented in the current shape.
- Lossy migrations MUST create a rollback checkpoint before apply
  (ADR-0006 mutation-journal record); the effective-setting record
  notes `last_written.actor_class = workspace_migration` until a
  subsequent user write re-labels it.
- A migration row that would change a setting's type in a way that
  no transform can describe (for example, `string` to
  `array_of<string>`) is forbidden; mint a new `setting_id` and
  add a `redirect_to_canonical` alias pointing the old id at the
  new one.
- Import flows (VS Code, JetBrains, Vim, Emacs) translate legacy
  ids into canonical ids through the same migration table; they
  MAY NOT invent side-channel keys.

### Control-stack fields (frozen)

Every effective-setting record carries a frozen control-stack block
so the feature-flag / experiment / schema-governance / admin-policy
surfaces share one decision trace. The block is orthogonal to the
scope precedence: it describes **which external control authority
is currently influencing the setting**, not which user-editable
layer won. (UI reference:
`.t2/docs/Aureline_UI_UX_Spec_Document.md:7880` "Feature flags,
experiment, kill switches, and schema-governance UX".)

| Field                        | Meaning                                                                                                 |
|------------------------------|---------------------------------------------------------------------------------------------------------|
| `source_label`               | Short source tag for the winning layer (the same label surfaces render on "show source").               |
| `lifecycle_label`            | `experimental`, `beta`, `stable`, `deprecated`, `retired`.                                              |
| `last_refresh_at`            | Monotonic stamp of the last time the control stack evaluated this setting.                              |
| `expires_at`                 | When the current control-stack binding expires (experiment expiry, policy TTL); `null` if not bounded.  |
| `offline_fallback`           | `authoritative_local`, `last_known_good_signed`, `cache_only`, `unavailable_offline`.                   |
| `explain_why_ref`            | Opaque handle an `Explain why` affordance resolves against the resolver (links to the shadow chain plus any control-stack reasoning). |
| `control_authority`          | One of `embedded_default`, `signed_admin_bundle`, `user_profile_workspace`, `managed_override`, `experiment_rollout`, `kill_switch`. |
| `narrowing_ceiling_active`   | Boolean; `true` when admin-policy narrowing is capping the value.                                       |

Surfaces MUST NOT render the control-stack block alongside a
separate "copy-only shadow setting" that duplicates the underlying
setting; the explain-why affordance reads from this block only.

### Preview-class and rollback-checkpoint expectations

High-risk settings declare a `preview_class` in their definition
row. The resolver enforces the class uniformly across every
write surface.

| Preview class                                    | Required affordance                                                                                         |
|--------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
| `safe_apply`                                     | Apply directly. Most settings.                                                                              |
| `preview_required`                               | Show a visible preview; apply blocked until acknowledged.                                                   |
| `rollback_checkpoint_required`                   | Create a mutation-journal checkpoint (ADR-0006) before apply; `Undo` routes to the checkpoint.              |
| `rollback_checkpoint_and_approval_required`      | As above, plus an ADR-0007 approval ticket bound to the setting, the target scope, and the operation.       |
| `managed_action_only`                            | Apply is available only under the managed identity mode; self-hosted and account-free surface
the state but cannot write.                                                                                  |

High-risk examples (named here so later lanes do not invent new
classes): workspace trust toggles, AI egress policy, network egress
policy, admin-policy mirror endpoint, extension allow / deny lists,
provider selection for a credential-backed setting, default shell
and default task runner for a workspace, schema version pins for
durable artifacts, and policy-epoch minimums. Preview-class
declarations are additive-minor; demoting a previously
`rollback_checkpoint_required` setting to `safe_apply` is breaking
and requires a new decision row.

### One registry, no copy-only shadows

- The generated settings UI, the CLI `settings inspect` and
  `settings edit`, the command palette, the docs, the policy
  explainer, the AI `Explain why` affordance, the search panel, the
  mutation-journal renderer, the support-bundle exporter, the
  profile-import preview, and the optional-sync conflict preview
  MUST all read from the same registry and the same
  effective-setting record shape.
- Copy-only shadow settings (a setting whose only job is to mirror
  another setting's value under a different key) are forbidden on
  protected surfaces. If a new name is needed, add an alias row
  that redirects to the canonical id; do not mint a parallel row.
- Docs strings for a setting are stored on the definition row
  (`summary`, `description`, `help_doc_ref`, `change_guidance`);
  surfaces render those strings rather than embedding private
  copies.
- Reserve fields for later control-stack, experiment-expiry, and
  schema-governance packets (see the control-stack block above)
  are additive-minor extensions to the effective-setting record;
  they do not repurpose existing fields.

### Scope-broadening rule (frozen)

The resolver applies three invariants that together forbid silent
scope broadening:

1. A setting MUST NOT widen its declared allowed-scope set without
   an additive-minor schema change, a new decision row, and a
   migration row describing the expansion.
2. Imported profiles and optional-sync payloads MAY narrow a
   workspace or user value, but they MAY NOT silently widen trust,
   workspace permissions, AI egress, network egress, or
   managed-entitlement scope (per PRD
   `.t2/docs/Aureline_PRD.md:1147` and TAD
   `.t2/docs/Aureline_Technical_Architecture_Document.md:1764`).
   The resolver denies any such write with
   `scope_broadening_would_widen_trust`.
3. A `session_or_command_override` never promotes to a durable
   scope unless the user (or a migration action) explicitly
   promotes it. The effective-setting record names the ephemeral
   source with a `session` source-label so the user can tell
   whether the value is durable.

### Audit events (frozen)

Every observable resolver action emits a structured event on the
`settings` audit stream. Events carry setting id, resolved scope,
source label, lifecycle label, write intent, denial reason where
relevant, actor class, workspace scope, policy epoch, and trust
state. Events MUST NOT carry raw secret material (ADR-0007
redaction applies).

| Event id                              | Fires when                                                                          |
|---------------------------------------|-------------------------------------------------------------------------------------|
| `setting_read`                        | A surface resolves an effective-setting record.                                     |
| `setting_write_proposed`              | A write intent is computed; includes preview / approval / restart posture.          |
| `setting_write_applied`               | A write commits; includes the resolved scope and mutation-journal ref.              |
| `setting_write_denied`                | A write is denied with a typed denial reason.                                       |
| `setting_validation_failed`           | A value fails validation on read or write.                                          |
| `setting_preview_presented` / `_dismissed` / `_acknowledged` | Preview-class settings move through the preview flow.            |
| `setting_rollback_checkpoint_created` / `_restored` | A rollback-class setting creates / restores its checkpoint.               |
| `setting_migration_applied`           | A migration row translates a prior value shape.                                     |
| `setting_alias_redirected`            | A read or write arrived at a legacy id and was redirected to the canonical id.      |
| `setting_capability_locked_changed`   | A capability dependency's state changes (flag flips, trust degrades).               |
| `setting_policy_narrowing_applied`    | Admin-policy narrowing caps the effective value.                                    |
| `setting_restart_posture_mismatch`    | A surface would require a restart the definition row did not declare.               |
| `setting_unknown_preserved`           | A read preserved an unknown-id entry without promoting it.                          |
| `setting_retired_read`                | A read encountered a retired setting id.                                            |
| `setting_schema_version_bumped`       | The setting-definition registry's schema version advances.                          |

### Process-boundary constraints (frozen)

1. The settings resolver runs in the host process alongside the
   workspace authority. Extensions reach settings through the
   extension-SDK surface; they MUST NOT crawl JSONC files
   directly.
2. Effective-setting records cross the RPC boundary; raw secret
   material does not. `credential_alias` values carry aliases only
   (ADR-0007).
3. Remote-agent workspaces surface a remote-scoped settings view
   whose authority class is `workspace_vfs` on the remote and
   `policy_entitlement` on the host; the effective-setting record
   names both in the `source_label` when a remote setting is in
   effect.
4. Crash dumps MUST NOT inherit in-flight write-intent buffers that
   carry `credential_alias` values; redaction applies per ADR-0007.
5. Mutation-journal entries for setting writes name
   `setting_id`, `resolved_scope`, `source_label`, `write_intent`,
   `preview_class`, and `rollback_checkpoint_ref`; they MUST NOT
   embed raw secret material or raw `credential_alias` secret
   payloads (the alias reference is safe).

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- The full generated settings UI and the CLI `settings` command
  surface. This ADR freezes the vocabulary those surfaces will
  render; the surfaces themselves land under later rows.
- The live resolver implementation (crate, cache strategy,
  incremental re-resolution, notification plumbing). This ADR
  freezes the contract; the crate lands under a later row.
- The managed sync service. This ADR freezes the scope posture and
  the non-widening rule; the service itself lands under a later
  row.
- The schema-registry distribution service. This ADR freezes the
  setting-definition record shape; the registry landing lands
  under a later row.
- The experiment-expiry UX and per-experiment rollout fabric. This
  ADR reserves the control-stack fields (`expires_at`,
  `control_authority`, `explain_why_ref`, `narrowing_ceiling_active`);
  the rollout fabric lands under a later row.
- Per-language override grammar for `language_override` (how the
  language key is resolved). This ADR freezes that the scope
  exists and that its precedence is above `folder_or_module_override`;
  the grammar lands under the language-platform lane.

These lines move only by opening a new decision row, not by
editing this ADR.

### Tradeoff table

The structured tradeoff rows live in
`artifacts/architecture/settings_tradeoff_rows.yaml`. Headline
summary:

| Axis                                   | Chosen stack                                                                                                   | Best rejected alternative                                              | Why chosen wins                                                                                                        |
|----------------------------------------|----------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------|
| **Identity model**                     | One canonical `setting_id` with `alias_set` redirecting legacy ids.                                            | Free-form ids per surface.                                             | Stable ids let keymaps, docs, policy, and automation survive renames without hidden breakage.                          |
| **Allowed scopes**                     | Frozen scope set with additive-minor expansion and an admin-policy narrowing ceiling.                          | Let every setting declare an ad-hoc scope list.                        | Unbounded scope lists make precedence unprovable; the frozen set keeps the resolver deterministic.                     |
| **Precedence order**                   | Nine-layer precedence per PRD 5.18 / TAD 12.4, plus admin narrowing as an orthogonal ceiling.                  | Collapse layers into "user" / "workspace" / "admin" only.              | Collapsing hides the imported-profile, machine, folder, and language overrides that the migration journeys need.       |
| **Effective-setting record**           | One frozen record shape with a shadow chain, lock state, write intent, restart posture, and control-stack block.| Per-surface view structs.                                              | Per-surface structs guarantee drift between UI, CLI, palette, docs, policy, and support.                               |
| **Lock / constraint state**            | Typed lock states with typed denial reasons; fail-closed writes.                                               | Silent scope downgrade on denial.                                      | Silent downgrade hides the policy ceiling and produces surprise leaks (trust widening, AI egress changes).             |
| **Shadow settings**                    | One registry; copy-only shadow settings forbidden.                                                             | Let surfaces define private shadow keys for UX convenience.            | Shadow keys desync docs, policy, migration, and sync exactly where users need them to agree.                           |
| **Restart posture**                    | Declared on the definition row; mismatch emits a typed audit event.                                            | Leave restart-required as a UI toast.                                  | Without a declared posture, imported profiles and optional sync cannot promise which changes require restart.          |
| **Migration posture**                  | Additive-preferred migration rows; breaking type changes require a new `setting_id`.                           | Rewrite in place when the new shape arrives.                           | Rewrite-in-place breaks profile round-trip and loses user comments; rename-via-alias preserves both.                   |
| **Schema of record**                   | Rust types in the eventual settings crate; JSON Schema export at `schemas/settings/*.schema.json`.             | External IDL + codegen at this milestone.                              | No second-language consumer yet; the JSON Schema export reserves a clean integration point.                            |

Each row carries reopen triggers in the YAML. A migration finding
that a legacy alias no longer resolves, a support finding that a
denial reason is not typed, a conformance finding that two surfaces
disagree on the winning scope, or a benchmark finding that
effective-setting resolution cannot meet its budget reopens the
relevant row.

### Setting-definition fixtures

A small corpus of setting-definition fixtures lives under
`fixtures/settings/setting_examples/`. They are short, reviewable
scenarios (user-global value wins, workspace value wins over
imported profile, machine-specific value excluded from sync,
admin policy narrows a user value, session-scoped override with
explicit promotion, preview-class and rollback-checkpoint
high-risk change) used by the UI, CLI, docs, policy explainer,
migration, and support-export lanes to anchor the scope names, the
precedence order, the effective-setting shape, the lock states, the
denial reasons, and the control-stack fields to concrete inputs and
observable outcomes. They are not a test suite; they are the
language the ADR's scope table and record shape refer to.

## Consequences

- **Frozen:** the allowed-scope set (`built_in_default`,
  `channel_or_experiment_default`, `imported_profile_default`,
  `user_global`, `machine_specific`, `workspace`,
  `folder_or_module_override`, `language_override`,
  `session_or_command_override`, `admin_policy_narrowing`), the
  nine-layer precedence order, the admin-policy narrowing ceiling,
  and the non-widening rule.
- **Frozen:** the effective-setting record shape, including
  `setting_id`, `value`, `resolved_scope`, `source_label`,
  `lifecycle_label`, `shadow_chain`, `lock_state`, `lock_reason`,
  `write_intent`, `write_denial_reason`, `restart_posture`,
  `preview_class`, `capability_dependencies`, `control_stack`,
  `last_written`, `schema_version`, `redaction_class`.
- **Frozen:** the lock-state set (`unlocked`, `policy_locked`,
  `policy_constrained`, `capability_locked`, `read_only_surface`),
  the denial-reason set, the write-intent set, the restart-posture
  set, and the preview-class set.
- **Frozen:** the control-stack block fields (`source_label`,
  `lifecycle_label`, `last_refresh_at`, `expires_at`,
  `offline_fallback`, `explain_why_ref`, `control_authority`,
  `narrowing_ceiling_active`). Reserved for later control-stack,
  experiment-expiry, and schema-governance packets.
- **Frozen:** stable `setting_id` rule, `alias_set` semantics, the
  migration-row shape, and the rule that type-change-breaking
  migrations require a new `setting_id`.
- **Frozen:** no copy-only shadow settings. One registry.
- **Frozen:** the schema of record is Rust types in the eventual
  settings crate; the boundary schemas live at
  `schemas/settings/setting_definition.schema.json` and
  `schemas/settings/effective_setting.schema.json`; no external IDL
  or codegen toolchain at this milestone. This mirrors ADR 0004,
  ADR 0005, ADR 0006, and ADR 0007.
- **Permitted:** adding a new allowed scope, a new lifecycle
  label, a new denial reason, a new write intent, a new restart
  posture, a new preview class, or a new control-authority class
  is an additive-minor change with a schema bump and a row in the
  registry; repurposing an existing value is breaking and requires
  a new decision row.
- **Permitted:** admin policy MAY narrow collection further on any
  setting. Policy MAY NOT silently widen collection beyond a
  setting's declared scope posture; any widening lands through a
  declared policy bundle that itself redacts to its class.
- **Follow-up:** the settings UI, the settings CLI, the profile
  import / export lanes, the optional-sync lane, the schema-
  registry lane, the experiment-rollout lane, and the support-
  export lane instrument every resolver event and respect every
  frozen scope / precedence / lock / write-intent / restart /
  preview rule before claiming settings-handling guarantees.
- **Follow-up:** the eventual managed sync service and schema-
  registry distribution service open follow-on decision rows that
  ride this contract rather than reshape it.
- **Follow-up:** the mutation-journal lane adds `settings_write`
  as a first-class actor class once the live resolver lands; the
  record fields named above are already reserved.
- **Ratifies:** the ADR-0005 subscription envelope's authority
  class `policy_entitlement` now refers to the setting-definition
  and effective-setting records frozen here. The ADR-0004 frozen
  error taxonomy's `policy` class absorbs the denial-reason set
  above as typed subcodes. The ADR-0006 save manifest's durable-
  state fields cover the JSONC artifacts that hold settings;
  settings writes use the same compare-before-write pipeline. The
  ADR-0007 secret broker owns every `credential_alias` value; no
  raw secret ever appears in a settings artifact.

## Alternatives considered

- **Free-form ids per surface.** Rejected: without a canonical
  `setting_id`, keymaps, docs, policy, migration, sync, and the
  mutation journal drift into incompatible id spaces. Renames
  become breakage events instead of alias rows.
- **Collapsed scope model (user / workspace / admin only).**
  Rejected: collapsing hides the imported-profile scope, the
  machine-specific scope, the folder / module override scope, and
  the language-override scope — exactly the scopes migration,
  portable profiles, and per-language behaviour depend on.
- **Per-surface effective-setting view structs.** Rejected: each
  surface would drift on provenance, lock state, and restart
  posture; `show source`, `Explain why`, and support-bundle
  exports would all tell different stories.
- **Silent scope downgrade on denial.** Rejected: downgrade turns
  a refusal into a leak (for example, silently writing a
  workspace-only trust toggle into user-global storage). Typed
  denial with a visible repair path is the only auditable posture.
- **Copy-only shadow settings.** Rejected: shadow keys desync
  docs, policy, migration, and sync exactly where users need
  those surfaces to agree. Aliases redirecting to the canonical
  id cover every rename / duplication use case.
- **Rewrite-in-place migration.** Rejected: rewrite-in-place loses
  user comments, churns diffs, and breaks profile round-trip.
  Alias + additive migration preserves the user's artifact.
- **Leave restart-required as a UI toast.** Rejected: without a
  declared restart posture, imported profiles and optional sync
  cannot promise which changes require restart. The conformance
  corpus needs a typed field, not scraped toast copy.
- **External IDL + generator for the setting envelopes.**
  Rejected: same argument ADR 0004, ADR 0005, ADR 0006, and
  ADR 0007 make — an IDL without a second-language consumer costs
  more than it buys; the JSON Schema export reserves the
  integration point.
- **Defer to a later milestone.** Rejected: the default-if-
  unresolved narrowing on `D-0014` ("freeze the product to a
  two-layer scope model — user-global and workspace only — with
  no admin narrowing ceiling, no imported-profile scope, no
  language override, and no preview / rollback posture") would
  block the migration, profile, sync, admin-policy, experiment,
  and support-export lanes exactly when later work needs the
  frozen vocabulary.

The `D-0014` `narrow` default-if-unresolved posture would have
locked the resolver to a two-layer user-global / workspace model
with no admin-policy narrowing, no imported-profile scope, no
language override, and no preview / rollback posture until an ADR
landed. Accepting this ADR replaces that narrowing with the frozen
allowed-scope set, nine-layer precedence order, admin-policy
narrowing ceiling, effective-setting record shape, lock-state set,
denial-reason set, write-intent set, restart-posture set,
preview-class set, and control-stack block above; the narrowing
default does not apply.

## Source anchors

- `.t2/docs/Aureline_PRD.md:1125` — "5.18 Workspace, configuration,
  profiles, and sync model".
- `.t2/docs/Aureline_PRD.md:1129` — "Configuration precedence" list.
- `.t2/docs/Aureline_PRD.md:1141` — "Profiles bundle keymaps,
  themes, fonts, extensions, tasks, snippets, AI policy presets,
  and layout preferences".
- `.t2/docs/Aureline_PRD.md:1147` — "workspace settings may never
  silently widen permissions, trust state, or AI egress policy".
- `.t2/docs/Aureline_PRD.md:1172` — "Property and model tests ...
  settings precedence".
- `.t2/docs/Aureline_PRD.md:1209` — "Aureline must preserve unknown
  fields and, where practical, user comments when round-tripping
  human-edited files".
- `.t2/docs/Aureline_PRD.md:1211` — "Policy files and machine
  overrides must be stored separately from user preferences so
  administrators can lock settings without rewriting personal
  files".
- `.t2/docs/Aureline_PRD.md:1213` — "Every effective setting shown
  in the UI should provide 'show source' information".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1699` —
  "12.4 Configuration and profile model".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1732` —
  "effective settings expose source attribution in the UI".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1747` —
  "Administrative defaults / policy ... narrows or overrides lower
  layers and is never merged as user preference".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1753` —
  "profile switching should apply the majority of visual,
  keybinding, command-surface, snippet, and extension-selection
  changes without restart; any restart-required delta must be
  surfaced before apply".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1764` —
  "imported or synced workspace-related settings may narrow
  behavior, but may never silently widen workspace trust,
  extension permissions, AI egress, network egress, or managed
  entitlements".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:7545` —
  "User and workspace settings ... JSON Schema Draft 2020-12".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:7550` —
  "Control-plane precedence" (embedded default -> signed local
  admin bundle -> user/workspace settings -> optional managed
  override).
- `.t2/docs/Aureline_Technical_Architecture_Document.md:8360` —
  "Appendix BD — Profile, Settings Sync, and Conflict Matrix".
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:7880` — "Feature flags,
  experiment, kill switches, and schema-governance UX".
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:7895` — "Feature flag /
  experiment ... last refresh".
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:3121` —
  "Policy-locked / managed ... the user can inspect but not
  override the effective value".

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0014`
- RFC: none.
- Tradeoff register (machine form):
  `artifacts/architecture/settings_tradeoff_rows.yaml`.
- Scope / precedence matrix (machine form):
  `artifacts/settings/scope_precedence_rows.yaml`.
- Boundary schemas (machine form):
  `schemas/settings/setting_definition.schema.json`,
  `schemas/settings/effective_setting.schema.json`.
- Setting-definition and effective-setting fixtures:
  `fixtures/settings/setting_examples/`.
- Companion vocabulary document:
  `docs/settings/settings_vocabulary.md`.
- Identity-mode envelope this contract rides:
  `docs/adr/0001-identity-modes.md`.
- Transport boundary effective-setting records cross:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`.
- Reactive-truth contract the effective-settings subscription rides:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`.
- Save-manifest contract settings writes flow through:
  `docs/adr/0006-vfs-save-cache-identity.md`.
- Secret broker every `credential_alias` value resolves against:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`.
- Affected lanes: `governance_lane:docs_public_truth`,
  `governance_lane:shell_command_system`,
  `governance_lane:support_export`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance. No supersession.
