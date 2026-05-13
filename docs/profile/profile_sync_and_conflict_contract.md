# Profile library, sync metadata, and conflict-resolution contract

This contract freezes the profile-level boundary Aureline uses for
profile-library rows, optional sync metadata, conflict journals, and
machine-binding addenda. It exists so profile convenience features
cannot silently absorb machine bindings, workspace trust approvals,
admin policy, delegated credentials, or broader AI / network authority.

Companion artifacts:

- [`/schemas/profile/profile_library_entry.schema.json`](../../schemas/profile/profile_library_entry.schema.json)
  defines one `profile_library_entry` row.
- [`/schemas/profile/sync_conflict_record.schema.json`](../../schemas/profile/sync_conflict_record.schema.json)
  defines one `profile_sync_conflict_record` row.
- [`/fixtures/profile/scope_class_cases/`](../../fixtures/profile/scope_class_cases/)
  contains worked rows for scope classes, addenda, managed-sync
  exclusions, and conflict review.

This contract composes with, and does not replace:

- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  for portable profile bodies, state-map rows, export manifests, and
  restore-provenance records.
- [`/docs/settings/sync_and_device_registry_seed.md`](../settings/sync_and_device_registry_seed.md)
  for settings-specific device records, scope bundles, session
  envelopes, and conflict packets.
- [`/docs/config/artifact_format_and_migration_policy.md`](../config/artifact_format_and_migration_policy.md)
  for JSON / JSONC format, schema-version, unknown-field, and
  comment-preservation policy.
- [`/docs/governance/data_portability_and_exit_matrix.md`](../governance/data_portability_and_exit_matrix.md)
  for export, deletion, offboarding, and managed-withdrawal posture.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  for approval authority. Profile import and sync may cite approval
  refs but may not mint approval authority.

## Scope

Frozen here:

- the six profile scope classes every profile library row must expose;
- the portable profile library entry shape, including revision
  lineage, included / excluded scope classes, sync posture, and
  rollback hooks;
- the sync metadata and conflict-journal minimum needed to make
  conflicts previewable and supportable;
- the machine-binding addendum rules that keep machine-local state
  out of portable profiles and ordinary sync;
- the non-widening rules for import, sync, profile overwrite, and
  scope toggles;
- the minimum posture required before a future managed sync claim may
  mention encrypted or customer-managed storage.

Out of scope:

- implementing a sync backend, key-management system, profile UI,
  storage service, or merge engine;
- defining final product copy for review sheets or badges;
- making managed sync part of the open-source baseline. File export
  and import remain the complete baseline.

## Core invariants

1. **Every library entry names all six scope classes.** A row that
   says only "synced profile" is non-conforming because reviewers
   cannot tell what was excluded.
2. **Local durable state remains authoritative.** Stale remote data,
   missing keys, policy blocks, or managed-service outages may create
   a conflict row; they do not overwrite local state.
3. **Sync and import are non-widening.** They may narrow behavior or
   preserve inert references, but they may not silently grant
   workspace trust, extension permissions, entitlements, AI egress,
   network egress, credential exposure, or admin control.
4. **Machine bindings require an addendum.** Machine-local paths,
   display bindings, hardware toggles, trust-store pointers, and
   credential-store handles never ride in the portable profile body
   or ordinary sync payload.
5. **Conflicts are field-aware and previewable.** Whole-document
   last-writer-wins is forbidden for profile library entries and sync
   records.
6. **Rollback is part of apply.** Profile overwrite, scope toggles,
   and conflict resolutions that change durable profile state require
   a change preview and rollback checkpoint before apply.
7. **No-service portability is complete.** A user can export, carry,
   inspect, import, preview, decline, and roll back a profile without
   an account or live service.

## Scope class matrix

| Scope class | Authority | Export posture | Sync posture | Conflict / safety rule |
|---|---|---|---|---|
| `profile_defaults` | User-authored profile artifact or imported baseline. | Portable profile body. | Opt-in when sync is enabled. | Schema-aware merge preview before overwriting another profile revision. User-global values win over imported defaults. |
| `user_global_preferences` | User. | Portable profile body or user-scope settings export. | Opt-in per scope. | Field-aware conflict review; high-risk effective changes route through preview and rollback. |
| `machine_specific_state` | Current machine. | Local-only machine-binding addendum when explicitly created. | Never ordinary sync. | Local value is authoritative. Import may display an inert hint, but durable apply on a destination requires explicit Save. |
| `workspace_settings` | Workspace owner or local user, narrowed by policy. | Separate workspace export, not user profile export. | Not part of user-profile sync. | Import or sync may narrow behavior but must block or strip any trust, permission, entitlement, AI egress, or network egress widening. |
| `administrative_defaults_or_policy` | Admin or organization signed bundle. | Signed policy bundle or admin export only. | Never user sync. | Replaced only by a newer valid signed bundle. User profile import cannot create, edit, or supersede admin policy. |
| `ephemeral_session_state` | Current session. | Not exported except support metadata where allowed. | Never synced. | Discard on switch or restart unless the user explicitly promotes a safe subset through a durable write intent. |

## Profile library entry

A `profile_library_entry` is the profile picker / import / support row,
not the whole profile body. It points at the portable profile artifact
and records the policy around that artifact.

Required groups:

| Group | Purpose |
|---|---|
| `revision` | Current logical revision, parent revision, content digest, schema version, source device ref, and emitted time. |
| `scope_class_entries` | One row for each class in the scope matrix, with authority, export posture, sync posture, conflict rule, and excluded data classes. |
| `library_artifacts` | Opaque refs for the portable profile body, export manifest, sync metadata, conflict journal, machine-binding addenda, rollback checkpoints, and preserved prior artifacts. |
| `managed_sync` | Whether sync is enabled, what storage claim is made, what classes are excluded, and what degraded posture is used when sync cannot run. |
| `non_widening_guarantees` | The denied widening vectors and the audit / approval / blocked-resolution requirements for import and sync. |
| `merge_preview_and_rollback` | The preview and checkpoint floor for overwrite, import, scope-toggle, and conflict-resolution flows. |
| `oss_portability_fallback` | The file export / import path that remains available with no service, account, or managed storage. |

The profile library entry may cite the portable profile schema, the
settings conflict packet, or migration / restore records, but it does
not duplicate their payload bodies. It carries refs, summaries, class
labels, and decisions safe for support export.

## Sync metadata and conflict journal

Sync metadata is local recovery state for a sync attempt. A conflict
journal is the durable record of conflicts and chosen resolutions. Both
are kept separate from the portable profile body.

Minimum sync metadata:

- participating profile id and library entry ref;
- selected scope classes and omitted scope classes;
- local revision set, incoming revision set, and last common revision
  when available;
- transport state, storage claim, encryption / key availability, and
  policy state;
- last successful apply, last refused apply, and the reason classes;
- conflict journal refs and rollback checkpoint refs.

Minimum conflict journal:

- one `profile_sync_conflict_record` per unresolved or resolved
  disagreement;
- local and incoming candidates, redacted value previews or class
  labels, and source revision refs;
- typed conflict class and field diffs;
- non-widening verdict;
- offered resolution paths and selected resolution;
- preview, rollback, approval, mutation-journal, and support-export
  refs.

Keymap and saved-view conflicts use the alpha review packet documented at
[`docs/settings/sync_conflict_review_alpha.md`](../settings/sync_conflict_review_alpha.md).
Those packets keep the same local-authoritative and non-widening rules while
adding artifact owner scope, privacy class, portability label, per-device
revision attribution, and explicit `Keep local`, `Keep synced`, and `Compare`
actions. Saved-view packets must not serialize transient selection, stale
provider cursors, or secret-bearing parameters as portable state.

Sync metadata and conflict journals are not themselves portable
profile content. Support bundles may include their metadata under the
declared redaction class, but raw secret material, raw machine paths,
workspace code, trust approvals, and admin bundle payloads remain
excluded.

## Machine-binding addendum

A machine-binding addendum is a local-only companion to a portable
profile. It may record redacted hints or handles for values that only
make sense on the current machine, such as:

- local toolchain path hints;
- display, window, or input-device bindings;
- OS trust-store pointer summaries;
- platform credential-store handle classes;
- local hardware acceleration or terminal integration toggles.

Rules:

1. The addendum is never emitted by ordinary profile export or sync.
2. If a user explicitly exports an addendum, import renders it as an
   inert review aid. The destination machine must mint its own durable
   machine-specific state through an explicit Save.
3. Raw secret material, delegated credentials, approval tickets, and
   admin policy never appear in an addendum.
4. Support export may include addendum metadata only when the active
   redaction profile allows it.

## Non-widening import and sync

The following vectors are denied by default for profile import and
sync:

- workspace trust;
- extension permissions;
- managed entitlements;
- AI egress;
- network egress;
- credential exposure;
- admin control or policy authority;
- approval tickets or delegated credentials.

A profile import or synced value that would widen one of those vectors
must be refused or converted into a conflict record. The only safe
default resolution is `keep_local` or `decline`. A user may later
perform a separate explicit approval, trust, permission, or admin flow
through the owning subsystem; the profile lane may not bundle that
authority into profile apply.

Workspace-related values are allowed to narrow behavior. Examples:
disabling AI provider routing, reducing network egress classes,
turning off an extension recommendation, or preserving a workspace
setting as inert pending review. Narrowing still goes through preview
when it materially changes effective behavior.

## Merge preview and rollback

Profile apply surfaces must offer the same decision quality whether
the source is manual file import, managed sync, support recovery, or a
future encrypted storage provider.

Required preview cases:

- profile overwrite or delete-vs-modify;
- adopting a synced value over a local value;
- merging profile defaults or user-global preferences;
- toggling a scope on or off for sync or export;
- accepting a workspace-related value that narrows behavior;
- importing a profile whose schema version needs migration;
- applying a machine-binding addendum as a new local setting.

Every preview names the source, target, scope class, revision refs,
redaction posture, non-widening verdict, and rollback checkpoint. The
rollback checkpoint is retained through the declared recovery window;
partial apply or manual review keeps the checkpoint until the user
resolves or expires it.

## Managed sync and encrypted storage claims

Future managed sync rows may claim encrypted or customer-managed
storage only when the profile library entry can prove:

- sync is opt-in and can be disabled without harming local editing;
- the OSS file export / import path remains complete;
- payload encryption mode and storage owner are declared;
- key unavailability degrades to local-authoritative state;
- customer-managed storage or self-hosted storage has an export path
  that does not require support intervention;
- excluded classes remain excluded even when encryption is present;
- conflict records and rollback checkpoints stay locally inspectable.

Encryption is not a license to sync otherwise excluded classes.
Machine bindings, trust approvals, live credentials, delegated
credentials, admin policy payloads, and ephemeral session state remain
excluded unless a separate governed contract explicitly admits a
narrow, reviewed substitute.

## Verification fixtures

The fixture corpus in
[`/fixtures/profile/scope_class_cases/`](../../fixtures/profile/scope_class_cases/)
keeps one worked row for each scope class and conflict posture. A new
fixture must:

- point at either `profile_library_entry.schema.json` or
  `sync_conflict_record.schema.json`;
- state the focus scope class in `__fixture__`;
- cite the contract section it exercises;
- use opaque refs instead of raw paths, URLs, hostnames, or secrets;
- preserve local-authoritative behavior when conflict or degradation is
  present.
