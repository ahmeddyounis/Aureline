# Settings schema registry seed

This document is the human-readable seed for the Aureline settings
**schema registry**: the published family of JSON Schema boundary
records every settings-aware surface reads when it resolves, inspects,
edits, imports, syncs, migrates, previews, approves, applies, rolls
back, exports, or explains a setting. The ADR
[`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
freezes the decision; the vocabulary doc
[`docs/settings/settings_vocabulary.md`](./settings_vocabulary.md)
names the shared tokens; this document freezes the **publishing and
consumption contract** for the boundary schemas themselves so the
settings UI, CLI `settings` command, command palette, docs renderer,
policy explainer, AI "Explain why" affordance, search panel,
mutation-journal renderer, profile import / export, optional sync,
support-bundle exporter, and the eventual schema-registry distribution
service can all point to the same rows, the same field names, and the
same evolution posture.

If this document and the ADR disagree, the ADR wins and this document
must be updated in the same change.

## Why a registry seed now

The ADR freezes the vocabulary. Without a registry seed that names the
**publishing conventions** — how a setting-definition row declares its
schema identity, how unknown fields are preserved, how JSONC comments
survive a round trip, how legacy aliases retire, how row-level preview
semantics map to in-product row anatomy, and how release-to-release
diffs are produced — each lane would invent its own answer:

- the settings UI would render source labels without a stable pill
  contract;
- the CLI `settings inspect` and the support exporter would serialise
  rows with different unknown-field policies;
- the profile importer would discard user comments because it did not
  know JSONC round-trip was part of the contract;
- the migration lane would retire alias rows on different schedules
  from what import-preview expects;
- the schema-registry distribution service would ship setting payloads
  that do not diff cleanly release-to-release;
- settings search would surface a row without being able to land on
  the exact definition and winning source without reinventing a
  parallel row model.

This seed closes those gaps before the distribution service, the
generated settings UI, and the live resolver land.

## Schema family (boundary of record)

The cross-tool schema family is the files below. The eventual settings
crate's Rust types are the schema of record; the JSON Schemas are the
cross-tool boundary every non-resolver consumer reads.

| Schema | Envelope | Purpose |
|---|---|---|
| [`/schemas/settings/setting_definition.schema.json`](../../schemas/settings/setting_definition.schema.json) | `setting_definition_row` | Stable `setting_id`, declared `value_type`, `allowed_scopes`, `alias_set`, `migration_table`, `capability_dependencies`, `preview_class`, `restart_posture`, `lifecycle_label`, `redaction_class`, `is_machine_specific`, `is_synced_by_default`, `is_policy_narrowable`, `summary`, `description`, `help_doc_ref`, `change_guidance`, `decision_row_ref`. One canonical row per setting. |
| [`/schemas/settings/effective_setting.schema.json`](../../schemas/settings/effective_setting.schema.json) | `effective_setting_record` / `settings_audit_event_record` | Winning `value`, `resolved_scope`, `source_label`, `lifecycle_label`, `shadow_chain`, `lock_state`, `lock_reason`, `write_intent`, `write_denial_reason`, `restart_posture`, `preview_class`, `capability_dependencies`, `control_stack`, `last_written`, `schema_version`, `redaction_class`, `alias_redirected_from`, `migration_applied`, `preview_state`, `remote_scope_context`. Plus the frozen settings audit event shape. |
| [`/schemas/settings/settings_row_state.schema.json`](../../schemas/settings/settings_row_state.schema.json) | `field_row_record` (settings specialization) | Settings-row projection for settings UI and settings search. Specializes the shared UX field-row contract to `surface_family=settings_form` and `canonical_field_path` matching the canonical `setting_id`. Carries row anatomy, source pill, effective-value inspector, deep-link, and optional search-hit highlighting so surfaces can land on an exact row without inventing parallel models. See [`docs/settings/settings_row_contract.md`](./settings_row_contract.md). |
| [`/schemas/settings/write_intent.schema.json`](../../schemas/settings/write_intent.schema.json) | `write_intent_packet` / `change_preview_packet` | Typed `reason_class` (`user_edit`, `profile_apply`, `import`, `sync`, `policy`, `automation`), target scope, proposed value, `write_intent` verdict, `write_denial_reason`, `scope_broadening_verdict`, `checkpoint` block (checkpoint / approval refs), preview state, apply state, redaction posture, and the structured change-preview delta. |
| [`/schemas/settings/precedence_resolution.schema.json`](../../schemas/settings/precedence_resolution.schema.json) | `precedence_resolution_packet` / `write_scope_review_packet` | Inspector-facing resolution chain across product defaults, packages, templates, imports, profile, user, device, workspace, folder, environment, remote target, session override, policy, and emergency override scopes. Also names write-scope fan-out targets, exact file or authority refs, blocked scopes, review requirements, checkpoint refs, approval refs, and downgrade state. See [`docs/settings/precedence_lock_and_write_scope_contract.md`](./precedence_lock_and_write_scope_contract.md). |
| [`/schemas/settings/lock_state_reason.schema.json`](../../schemas/settings/lock_state_reason.schema.json) | `lock_state_reason_packet` | Shared lock-state reason rows for inspector and write-scope packets, including inherited, policy-locked, unsupported-scope, wrong-target, secret-required, missing-dependency, degraded-read-only, mixed-version, stale-read, and migration-alias-only explanations. See [`docs/settings/precedence_lock_and_write_scope_contract.md`](./precedence_lock_and_write_scope_contract.md). |
| [`/schemas/settings/sync_device_record.schema.json`](../../schemas/settings/sync_device_record.schema.json) | `sync_device_record` | Opaque `device_id`, `device_label`, `device_class`, `os_family_class`, `identity_mode`, `trust_state`, revocation lifecycle (`active` / `paused` / `revoked` / `forgotten`), `revocation_reason`, `revoked_at`, `revoked_by_actor_class`, `capability_states`, `device_secret_binding` (class + broker alias only), `export_safe_lineage`, `redaction_class`. See [`docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md). |
| [`/schemas/settings/sync_scope_bundle.schema.json`](../../schemas/settings/sync_scope_bundle.schema.json) | `sync_scope_bundle` / `sync_session_envelope` | Single-scope bundle entries with `scope_broadening_verdict`, the frozen `omitted_classes` denylist, `producer_device_id`, monotonic `bundle_epoch`, and the session-level envelope carrying `session_state`, `transport_state`, `degrade_reasons`, and the `manual_continuity` block. See [`docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md). |
| [`/schemas/settings/sync_conflict_packet.schema.json`](../../schemas/settings/sync_conflict_packet.schema.json) | `sync_conflict_packet` | Typed `conflict_class`, field-aware `conflict_delta`, `scope_broadening_verdict`, offered `resolution_paths` (keep-local, keep-synced, merge-preview, rollback-friendly-review, decline), `resolution_state`, `rollback_checkpoint_ref`, `approval_ticket_ref`, `mutation_journal_ref`, `change_preview_ref`, and `remote_origin` lineage. See [`docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md). |

Every row across the family shares the same frozen token sets
(`scope_id`, `lifecycle_label`, `preview_class`, `restart_posture`,
`redaction_class`, `write_intent`, `write_denial_reason`,
`actor_class`, `capability_dependency.kind`,
`control_authority`) plus the precedence packet's user-visible
resolution scopes and lock-state reason codes. The vocabulary doc
lists each token; the JSON Schemas are the authoritative enum set and
bind together by the shared `settings_schema_version` or
`settings_precedence_schema_version` integer.

## Publishing conventions

### `$id` and `$schema` URIs

- Each schema declares a stable absolute `$id` under
  `https://aureline.dev/schemas/settings/<name>.schema.json`. The
  published URI MUST match the on-disk path so offline consumers and
  the distribution service resolve identically.
- Each schema declares `$schema:
  "https://json-schema.org/draft/2020-12/schema"`. Later drafts MUST
  NOT land without a new decision row (mirrors ADR 0004, ADR 0006).
- The `$id` is the stable identity across releases. Payloads carry
  an integer `settings_schema_version` (currently `1`) that surfaces
  may diff release-to-release without re-parsing the `$id`.
- Every setting-definition row also carries a string `schema_version`
  (currently `"1"`) copied into the matching effective-setting record
  and write-intent packet; it tracks the **row** shape separately from
  the envelope `settings_schema_version`.

### Version URI and payload version rules

- Additive-minor changes (new scope, lifecycle label, denial reason,
  write intent, restart posture, preview class, control-authority
  class, capability-dependency kind, transform class, reason class,
  widening vector, preview delta kind) MUST NOT bump the `$id`; they
  MUST bump `settings_schema_version` and add a row here.
- Repurposing an existing enum value, relabeling a token, or removing
  a field is breaking. Breaking evolution mints a new `$id` on a
  superseding schema file, ships a deprecation row under the existing
  `$id`, and requires a new decision row.
- The same payload never mixes envelope versions. A consumer reading
  an `effective_setting_record` with `settings_schema_version=1`
  reads the matching `setting_definition_row` with
  `settings_schema_version=1` and the matching
  `write_intent_packet` / `change_preview_packet` with
  `settings_schema_version=1`.
- The distribution service MUST be able to render a machine diff
  between any two published versions of the registry. The diff
  surfaces added rows, deprecated rows, added aliases, added
  migrations, added enum values, and any change to preview / restart /
  redaction / capability posture. No surface may invent a "settings
  changelog" that bypasses the machine diff.

### Unknown-field policy

- Settings artifacts (user `settings.jsonc`, workspace settings,
  imported profile, optional-sync payload, admin-policy bundle) are
  **round-trip preserving**. Unknown top-level settings are kept
  on read, surfaced as a typed `setting_unknown_preserved` audit
  event, and may be removed only with an explicit user or migration
  action (mirrors the ADR Section *Validation posture (frozen)*).
- Unknown fields inside `object` and `tagged_union` values are
  preserved when the shape allows it; they render as `unknown_field`
  diagnostics but do not block other fields from applying.
- Boundary schemas declare `additionalProperties: false` on every
  `*_row` / `*_record` / `*_packet` object so the resolver and every
  consumer fail loudly on a typo, a renamed field, or a drifted
  enum. Unknown-field preservation applies to **settings values**,
  not to the **envelope** record shape; envelope fields are closed.
- The optional-sync lane and the profile importer run the same
  validation pipeline as interactive writes; they MAY NOT bypass the
  unknown-field policy.

### Comment-preservation posture for JSONC-like artifacts

- User `settings.jsonc`, workspace settings, and imported profiles
  accept line and block comments. The resolver reads the JSON value
  alongside a side-channel that tracks leading and trailing comments
  per key; on save, surfaces MUST round-trip the comments alongside
  the ADR-0006 compare-before-write pipeline.
- A save that would drop a user's comment, re-order map keys, or
  re-flow whitespace beyond documented canonicalisation MUST be
  treated as a bug; the save pipeline emits a typed write-intent
  diagnostic instead of silently churning the artifact.
- Profile export reproduces the imported artifact byte-for-byte
  modulo documented canonicalisation (per the scope-precedence
  matrix conformance test `round_trip_fidelity`).
- Admin-policy bundles and signed release-channel bundles are
  **not** JSONC; they MAY NOT embed user comments and MUST serialise
  canonically so signatures remain stable.

### Migration-alias retention windows

- Every `alias_row` carries `from_id`, `since_version`,
  `deprecated_in_version`, `removal_target_version`, and
  `alias_direction: redirect_to_canonical`. Aliases never mint new
  identities; they redirect onto a canonical `setting_id`.
- **Retention window** for an alias has three required stages:
  1. **Active**: `since_version` set, `deprecated_in_version` null,
     `removal_target_version` null. Reads and writes redirect
     silently; the resolver emits `setting_alias_redirected` on hit.
  2. **Deprecated**: `deprecated_in_version` set. Reads and writes
     still redirect; the UI, CLI, docs renderer, and migration
     center MUST surface the deprecation and name the canonical id.
  3. **Scheduled-for-removal**: `removal_target_version` set. The
     alias continues to redirect until the named version; on that
     version the alias row is retired and the legacy id begins
     returning `setting_unknown_at_registry` with a one-release
     grace message that names the canonical id.
- Default retention: an alias stays **active** for at least **two
  minor releases** after `since_version`, is **deprecated** for at
  least **two minor releases** before `removal_target_version`, and
  moves through removal on a **major release**. Shorter windows
  require a decision row that names the affected id.
- Admin-policy, optional-sync, and profile-import paths honour the
  same window. A profile that carries a deprecated id is accepted
  silently; a profile that carries a removed id is accepted with a
  visible migration-center row and never silently dropped.
- A retired setting id (lifecycle `retired`) is not an alias; it
  refuses writes and returns the frozen retired-value notice on
  read. An alias onto a retired id is forbidden.

### Sensitivity / redaction declarations

- Every setting row declares a `redaction_class` from the frozen
  set (`none`, `ui_string_only`, `redact_value_preserve_shape`,
  `redact_to_class_label`, `exclude_from_export`). The effective-
  setting record and the write-intent / change-preview packet copy
  the class and apply it on every exportable surface.
- `credential_alias` settings MUST declare a
  `credential_secret_class` in the value-type block
  (`ai_provider_token`, `code_host_token`, `package_registry_token`,
  `database_credential`, `ssh_key_material`, `client_certificate`,
  `signing_key_material`, `provider_session`, `device_secret`,
  `ephemeral_operation_token`). Raw secret bytes never appear in
  any settings artifact; only the broker alias ever crosses a
  boundary (ADR-0007).
- Support bundles, telemetry payloads, audit events, and mutation-
  journal entries apply the declared redaction class before export;
  schema-registry exports MUST quote the class rather than inventing
  a private "sensitive" chip.

### Evidence and docs refs

- `help_doc_ref` points at the docs lane via a stable handle; never
  at a mutable URL. The docs renderer resolves the handle.
- `decision_row_ref` points at
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and is **required** for settings whose `preview_class` is
  `rollback_checkpoint_and_approval_required` and for settings whose
  `allowed_scopes` include `admin_policy_narrowing` as the intended
  authority.
- Every row that exports to a support bundle, offboarding packet, or
  schema-governance surface MUST also quote the matching
  `record_class_id` from
  [`artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  instead of minting a parallel "retention class" label.

## In-product row anatomy (schema → pixel contract)

Settings surfaces are not built yet, but the schema family already
pins the **row anatomy** those surfaces render. The mapping below is
normative: any future settings row MUST read these semantics from the
schema family and MUST NOT invent parallel fields.
The row-level interaction contract (source pill visibility, lock
inspection, exact-row deep links, and search highlight) is frozen in
[`docs/settings/settings_row_contract.md`](./settings_row_contract.md).

| Row element | Reads from | Field(s) | Contract |
|---|---|---|---|
| **Source pill** | `effective_setting_record` | `resolved_scope`, `source_label`, `control_stack.control_authority` | Render the frozen scope label plus the source tag. Never render a raw path; never infer a pill from the value. Pill icon follows the scope: built-in default, channel / experiment, imported profile, user, this machine, workspace, folder override, language override, session override, admin policy. |
| **Lock / explain state** | `effective_setting_record` | `lock_state`, `lock_reason`, `control_stack.narrowing_ceiling_active`, `control_stack.explain_why_ref`, `capability_dependencies[*].satisfied` | Disabled controls MUST name the typed `lock_reason` in the copy and MUST expose `explain_why_ref` to the "Explain why" affordance. Silent disabling without a reason is a bug. |
| **Lifecycle badge** | `effective_setting_record` | `lifecycle_label`, `control_stack.expires_at` | Non-`stable` lifecycles MUST render a visible badge; experiment / channel scopes MUST render `expires_at` when set. |
| **Reset affordance** | `setting_definition_row` + `effective_setting_record` | `default_values[*].value_preview`, `resolved_scope`, `shadow_chain` | "Reset to default" targets the next lower non-empty scope in the shadow chain; surfaces name the target scope and invoke the same write-intent packet path as any other edit. |
| **Diff affordance** | `change_preview_packet` | `old_value`, `new_value`, `change_preview_delta`, `redaction_class` | High-risk settings render the frozen `delta_kind` diff; redacted classes fall through to `redacted_structural`. Surfaces MUST NOT invent a parallel diff format. |
| **Deep-link / exact-row URL** | `setting_definition_row` | `setting_id`, `alias_set[*].from_id` | The canonical `setting_id` is the deep-link anchor. An incoming link to a legacy id hits the alias table and lands on the canonical row; the surface records the redirect via `setting_alias_redirected`. |
| **Search highlight** | `effective_setting_record` | `setting_id`, `source_label`, `shadow_chain`, `lock_state`, `lock_reason`, `write_intent`, `preview_class` | A settings-search hit lands on **one** row and reveals the winning source, the shadow chain, and the lock reason without re-resolving. The row is the same projection the settings UI renders in-place; there is no parallel "search view" row model. |
| **Pending-write banner** | `write_intent_packet` | `write_intent`, `write_denial_reason`, `scope_broadening_verdict`, `preview_state`, `apply_state`, `restart_posture`, `checkpoint.checkpoint_required`, `checkpoint.approval_required` | Surfaces that render a pending write MUST name the typed verdict, the typed denial reason, the widening vector (if any), the preview state, and the checkpoint / approval requirement before apply. Silent acceptance of a widening write is a bug. |
| **Apply / rollback buttons** | `write_intent_packet` + `change_preview_packet` | `checkpoint.rollback_checkpoint_ref`, `checkpoint.approval_ticket_ref`, `apply_state`, `preview_class` | Apply is gated on the declared preview class: `preview_required` needs `preview_state=acknowledged`; `rollback_checkpoint_required` needs `rollback_checkpoint_ref` non-null; `rollback_checkpoint_and_approval_required` needs both plus `approval_ticket_ref`. Rollback routes to the checkpoint handle; surfaces do not mint their own. |
| **Restart prompt** | `effective_setting_record` + `write_intent_packet` | `restart_posture`, `checkpoint.restart_required_acknowledged` | A non-`no_restart` posture MUST prompt before apply; unacknowledged restarts produce `restart_required_not_acknowledged`. A silent restart that was not declared in the definition row is a bug (`setting_restart_posture_mismatch`). |
| **Last-changed line** | `effective_setting_record` | `last_written.at`, `last_written.actor_class`, `last_written.mutation_journal_ref` | "Last changed" copy reads the monotonic stamp plus the typed actor class; the mutation-journal handle is the deep link into the history. |

## Reason classes for write-intent packets

Every write-intent packet declares one `reason_class`. Surfaces MUST
use the packet path even when the write is silent (admin-policy
injection, optional-sync pull, profile-apply). No surface may bypass
the packet for a "quick write".

| Reason class | Typical initiators | Required scope / authority posture |
|---|---|---|
| `user_edit` | Settings UI keystroke / command, command palette, JSONC save, CLI `settings edit`, AI `apply` acknowledged by the user. | Any scope in the setting's `allowed_scopes` that the current surface is permitted to write. |
| `profile_apply` | Profile switch action. | `user_global`, `workspace`, `folder_or_module_override`, `language_override`; narrowing only. |
| `import` | Profile import (Aureline, VS Code, JetBrains, Vim, Emacs). | `imported_profile_default`; narrowing only. A writing import that widens trust / permissions / egress MUST be denied with `scope_broadening_would_widen_trust`. |
| `sync` | Optional-sync pull / push. | `user_global`, `language_override`; never `machine_specific`. Sync runs the same validation / migration pipeline as interactive writes. |
| `policy` | Admin-policy injector. | `admin_policy_narrowing`. Admin policy MAY NOT target any other scope. Admin policy narrows; it does not widen (ADR 0008 Section *Precedence order (frozen)*). |
| `automation` | Extension API, CI, agent action that was not a direct user keystroke. | Any scope the extension / agent has capability for. Automation writes are still gated by preview class and approval posture. |

## No-silent-scope-broadening invariants

The write-intent / change-preview packet carries a frozen
`scope_broadening_verdict` block so every lane stamps the same
answer to three questions:

1. Would the proposed write silently widen workspace trust, AI
   egress, network egress, extension permissions, managed
   entitlement, credential exposure, or the declared allowed-scope
   set? The `widening_vector` enum names the vector; `none` pairs
   with `would_widen_trust=false`.
2. Did an active admin-policy narrowing ceiling intersect the
   proposal? `narrowing_ceiling_intersected=true` is informational;
   it never flips the verdict on its own but surfaces the trace so
   the support lane and the "Explain why" affordance read one
   record.
3. Was the verdict a denial? Any `would_widen_trust=true` MUST pair
   with `write_intent=denied` and
   `write_denial_reason=scope_broadening_would_widen_trust`; the
   conformance corpus catches mismatches.

Silent widening is a bug. The optional-sync lane, profile importer,
extension API, and automation harness MUST respect the verdict;
they MAY NOT drop the packet and write the value anyway.

## Checkpoint and approval linkage

The `checkpoint` block in the write-intent packet is the **only**
path to a rollback handle or an approval ticket for settings. The
distribution service, the support exporter, and the migration
center read the handles directly; no surface may invent its own
checkpoint store.

- `checkpoint_required` is `true` for every
  `rollback_checkpoint_required` and
  `rollback_checkpoint_and_approval_required` setting. The
  `rollback_checkpoint_ref` points at an ADR-0006 mutation-journal
  record; `Undo` for the setting routes to that record.
- `approval_required` is `true` for every
  `rollback_checkpoint_and_approval_required` setting and for every
  `managed_action_only` setting proposed under a non-managed
  identity. The `approval_ticket_ref` points at an ADR-0007 ticket
  whose class matches `approval_ticket_class`; the resolver refuses
  apply on class mismatch.
- A preview flow that expires (`preview_state=expired`) MUST be
  re-presented before apply; the packet's `apply_state` falls back
  to `awaiting_preview`. The mutation journal never contains an
  applied write without a matching acknowledgement.

## Release diffing

The registry is explicit enough that a release pipeline can compute
a diff mechanically without reading prose:

- **Added** — new `setting_definition_row`, new enum value, new
  migration row, new alias row in `active` state.
- **Deprecated** — alias row flipped to `deprecated`, lifecycle
  label moved to `deprecated`, preview_class demoted (requires a
  new decision row), or capability_dependency added that narrows
  the value.
- **Removed** — alias row removed on its `removal_target_version`,
  lifecycle `retired`, deleted `setting_definition_row` (breaking;
  requires a new decision row and a superseding `$id`).
- **Reshape** — any change that repurposes an existing enum value,
  narrows `allowed_scopes`, relabels a field, or tightens the
  `redaction_class`. Reshape is breaking.

The distribution service publishes these diffs alongside the
registry; release notes reference the diff rather than the schema
file byte count. The machine diff is the source of truth; prose
summaries may render on top of it.

## Consumer checklist

Before a settings-aware surface ships, it confirms:

1. It reads the **canonical** setting id and never a display-only
   id. Legacy ids arrive through the alias table; the surface
   renders the redirect.
2. It renders the row anatomy mapping above from the schema family;
   it does not invent private fields for source pill, lock state,
   reset, diff, deep-link, or search highlight.
3. It emits a `write_intent_packet` for every settings write, even
   when the write is silent (sync pull, profile apply,
   admin-policy injection).
4. It applies the declared `redaction_class` before any export;
   `credential_alias` values carry aliases only.
5. It honours the `scope_broadening_verdict` and never drops a
   packet to write the value anyway.
6. It routes `Undo` for rollback-class settings to the
   `rollback_checkpoint_ref`; it never invents its own checkpoint.
7. It honours the `restart_posture`; a silent restart that was not
   declared is a bug.
8. It renders the `lifecycle_label` badge and the
   `control_stack.expires_at` for non-`stable` lifecycles.
9. It resolves `setting_id` to the same row the settings UI would
   render; search hits land on exactly one row and reveal the
   winning source, the shadow chain, and the lock reason.
10. It quotes `record_class_id` when the export is class-bearing
    (support bundle, offboarding packet, usage export); it does
    not invent a private "retention class" label.

## Where related artifacts live

- Decision register row `D-0014`:
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- Scope / precedence matrix:
  [`artifacts/settings/scope_precedence_rows.yaml`](../../artifacts/settings/scope_precedence_rows.yaml).
- Tradeoff register:
  [`artifacts/architecture/settings_tradeoff_rows.yaml`](../../artifacts/architecture/settings_tradeoff_rows.yaml).
- Setting-definition, effective-setting, and write-intent fixtures:
  [`fixtures/settings/setting_examples/`](../../fixtures/settings/setting_examples/).
- Record-class registry for export / retention posture:
  [`artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml).
- Companion vocabulary document:
  [`docs/settings/settings_vocabulary.md`](./settings_vocabulary.md).

## Change management

- Adding a new field, enum value, reason class, widening vector,
  preview delta kind, or preview state is additive-minor: bump
  `settings_schema_version`, add a row in the companion vocabulary
  doc, and extend the schemas.
- Repurposing any existing value (for example, reusing a widening
  vector for a different vector) is breaking and requires a new
  decision row.
- Renaming a setting is done through an `alias_row` that redirects
  the legacy id to the canonical id; it is never done by mutating
  the existing definition row's `setting_id`.
- A lossy migration MUST create a rollback checkpoint before apply;
  a breaking type change MUST mint a new `setting_id` with a
  redirect alias rather than trying to migrate in place.
- Publishing a new release of the registry MUST emit the machine
  diff described above; release notes quote the diff rather than
  inventing a parallel "settings changelog".
