# Configuration and State Path Map

This document is the normative path-level map for Aureline
configuration, durable state, recovery state, policy/control artifacts,
derived caches, and evidence bundles. It turns the architecture
appendix's path seed into a repository-owned contract that support,
backup, portability, recovery, low-disk, and clear-data tooling can
read without guessing semantics from filenames or implementation code.

The machine-readable companion artifacts are:

- [`/artifacts/state/path_level_seed_map.yaml`](../../artifacts/state/path_level_seed_map.yaml)
  - one row per known artifact or directory family, with scope,
  location concept, authority, portability, retention, clear, secret,
  export, backup, low-disk, and support posture.
- [`/artifacts/state/durable_artifact_inventory.yaml`](../../artifacts/state/durable_artifact_inventory.yaml)
  - class-level rules for user-authored durable truth, recovery state,
  admin or tenant control artifacts, disposable derived state, and
  evidence bundles.
- [`/fixtures/state/path_map_examples/`](../../fixtures/state/path_map_examples/)
  - selector examples showing support, backup, clear-data, and
  portability tools reading the same row ids.

This contract composes with, and does not replace,
[`profile_and_state_map.md`](./profile_and_state_map.md) and
[`state_object_inventory.md`](./state_object_inventory.md). The profile
map freezes portability vocabulary and state-map row ids. The object
inventory freezes schema-evolution and corruption-routing posture. This
path map adds the operational path-level selectors those contracts
leave implicit.

If this document disagrees with the product, architecture, or technical
design sources, those source documents win and this document plus both
machine-readable artifacts MUST be updated in the same change. If this
document disagrees with a downstream support, backup, restore,
portability, policy, or cache-clear surface, this document wins and the
surface is non-conforming.

## Scope

The map covers every launch-relevant state family named by the design
sources:

- user settings, keybindings, snippets, themes, command aliases,
  terminal preferences, machine settings, extension selections, profile
  exports, and the profile library;
- workspace manifests, workset manifests, tasks, launch configs,
  extension recommendations, and extension lockfiles;
- sync metadata, sync conflict journals, terminal scrollback and
  restore metadata, session restore state, dirty-buffer recovery,
  local history, AI memory or cache metadata, and deferred-intent
  outbox state;
- admin policy, tenant policy or entitlement snapshots, policy bundle
  cache, trust approvals, auth/session secrets, long-lived credentials,
  and admin audit metadata;
- execution-context cache, index/cache store, shared object store,
  knowledge or prebuild caches, logs, traces, support bundles, crash
  envelopes, review packets, incident bundles, benchmark results, and
  release evidence packs.

Out of scope: final per-OS concrete paths, storage-engine selection,
quota math, encryption envelope implementation, and UI copy. The row
ids and classes are stable selectors; concrete path expansion belongs
to the eventual platform path resolver.

## Closed Class Vocabulary

Every row in the path map binds to exactly one `artifact_class`.

| Class | Meaning | Delete rule |
|---|---|---|
| `user_authored_durable_truth` | State the user or workspace author wrote, accepted, or intentionally imported. | Never delete silently. Destructive changes require preview and rollback where the row declares it. |
| `user_owned_recovery_state` | Product-held state that protects the user's continuity after crash, restart, reconnect, or offline reconciliation. | Never treat as cache. Clear requires preview unless a class-specific retained-evidence rule is stricter. |
| `admin_or_tenant_control_artifact` | Signed policy, entitlement, audit, trust, or tenant control state whose authority is outside ordinary user preference editing. | Generic clear is denied; refresh, replace, or revoke through the signed authority path. |
| `disposable_derived_state` | Fully regenerable indexes, caches, object-store entries, or derived metadata. | Clear and low-disk eviction are allowed only when no retained-evidence pin or control authority override applies. |
| `evidence_bundle` | Support, crash, incident, benchmark, review, or release evidence packet. | Retention and hold policy govern; ordinary cache reset must not remove active evidence. |

Class confusion is forbidden:

1. `local_history`, `dirty_buffer_recovery_journal`,
   `terminal_scrollback_restore`, `sync_metadata`,
   `conflict_journal`, and `deferred_intent_outbox` are recovery
   state. They are not cache even when their retention is bounded.
2. `admin_policy`, `tenant_entitlement_snapshot`,
   `policy_bundle_cache`, `trust_approvals`, and `admin_audit_log` are
   control artifacts. A reset or low-disk flow may not downgrade them
   to disposable derived state.
3. `execution_context_cache`, `index_cache_store`,
   `knowledge_cache`, and unpinned `shared_object_store` entries are
   disposable derived state. They may not be used as proof that a user
   edit, external side effect, or policy decision happened.
4. `support_bundles`, `crash_envelopes`, `incident_bundles`,
   `benchmark_results`, and `release_evidence_packs` are evidence
   bundles. They may be content-addressed, but that does not make them
   ordinary cache while a retention window, hold, or release/support
   decision references them.

## Path Roots

The map deliberately names location concepts, not final platform
paths.

| Root | Meaning |
|---|---|
| `AURELINE_CONFIG` | User-readable configuration and profile files. |
| `AURELINE_STATE` | Local state, recovery journals, caches, logs, evidence, and policy cache. |
| `AURELINE_POLICY` | Signed policy bundle import root or managed policy drop location. |
| `workspace_tree` | Repository or workspace-owned files intended to be reviewed with the project. |
| `os_credential_store` | Platform credential store, keychain, vault, or enterprise secret broker handle store. |
| `export_destination` | User-selected export target; never an implicit source of authority. |

## Required Row Fields

Every `path_map_rows[]` entry in the seed map MUST include:

- `path_row_id`
- `artifact_family`
- `artifact_class`
- `scope_tags`
- `location_root`
- `location_concept`
- `authority_owner`
- `portability_class`
- `retention_class`
- `clear_class`
- `secret_posture`
- `exportability`
- `backup_posture`
- `low_disk_posture`
- `supportability`
- `never_delete_silently`

Downstream tooling MUST select by row id, class, and posture fields.
It MUST NOT infer deletion, export, backup, or redaction behavior from
path strings alone.

## Operational Selectors

### Support Bundles

Support tooling reads `supportability` and `secret_posture` from the
path map. Metadata-only rows stay metadata-only. Rows marked
`excluded_always` never contribute raw payload. Rows marked
`support_opt_in_redacted` require a user or policy-selected redaction
profile before body capture.

### Backup And Restore

Backup tooling reads `backup_posture` and `never_delete_silently`.
User-authored durable truth is backed up by default. Recovery state is
backed up when it materially protects continuity. Disposable derived
state is excluded unless a retained-evidence pin or explicit support
case references it.

### Clear Data And Low Disk

Clear-data tooling reads `clear_class`; low-disk tooling reads
`low_disk_posture`. Generic cache reset may target only rows whose
clear class admits derived-state clearing. It must not include local
history, dirty-buffer recovery, deferred intents, policy bundles,
entitlement snapshots, credential handles, or active evidence.

### Portability

Portability tooling reads `portability_class` and `exportability`.
Portable profiles include portable user-authored rows and declare
excluded rows explicitly. Workspace exports include workspace-shared
truth. Admin or tenant control artifacts travel only through signed
bundle, entitlement, audit, or offboarding paths. Raw credential
material never appears in a portable profile, support bundle, or
workspace export.

## Seed Row Index

The full machine-readable map is in
[`/artifacts/state/path_level_seed_map.yaml`](../../artifacts/state/path_level_seed_map.yaml).
This index lists the row ids downstream tools should quote:

| Row id | Class | Location concept |
|---|---|---|
| `user_global_settings` | `user_authored_durable_truth` | `$AURELINE_CONFIG/settings.jsonc` |
| `keybindings` | `user_authored_durable_truth` | `$AURELINE_CONFIG/keybindings.jsonc` |
| `snippets` | `user_authored_durable_truth` | `$AURELINE_CONFIG/snippets/*` |
| `themes_and_design_tokens` | `user_authored_durable_truth` | `$AURELINE_CONFIG/themes/*` |
| `command_aliases` | `user_authored_durable_truth` | `$AURELINE_CONFIG/aliases.jsonc` |
| `terminal_preferences` | `user_authored_durable_truth` | `$AURELINE_CONFIG/terminal/preferences.jsonc` |
| `machine_specific_settings` | `user_authored_durable_truth` | `$AURELINE_CONFIG/machine.settings.jsonc` |
| `profile_export` | `user_authored_durable_truth` | user-selected `*.aureprofile.json` |
| `profile_library` | `user_authored_durable_truth` | `$AURELINE_CONFIG/profiles/*.aureprofile.json` |
| `workspace_manifest` | `user_authored_durable_truth` | `aureline.workspace.jsonc` |
| `workset_manifest` | `user_authored_durable_truth` | `.aureline/worksets/*.jsonc` |
| `tasks_and_launch_configs` | `user_authored_durable_truth` | `.aureline/tasks.jsonc`, `.aureline/launch.jsonc` |
| `extension_selection_inventory` | `user_authored_durable_truth` | `$AURELINE_CONFIG/extensions.selected.jsonc` |
| `extension_recommendations` | `user_authored_durable_truth` | `.aureline/extensions.recommend.jsonc` |
| `extension_lockfile` | `user_authored_durable_truth` | `.aureline/extensions.lock.json` |
| `sync_metadata` | `user_owned_recovery_state` | `$AURELINE_STATE/sync/*` |
| `conflict_journal` | `user_owned_recovery_state` | `$AURELINE_STATE/sync/conflict_journal/*` |
| `terminal_scrollback_restore` | `user_owned_recovery_state` | `$AURELINE_STATE/terminal/*` |
| `session_restore_state` | `user_owned_recovery_state` | `$AURELINE_STATE/session/*` |
| `dirty_buffer_recovery_journal` | `user_owned_recovery_state` | `$AURELINE_STATE/history/recovery_journal/*` |
| `local_history` | `user_owned_recovery_state` | `$AURELINE_STATE/history/*` |
| `ai_memory_cache_metadata` | `user_owned_recovery_state` | `$AURELINE_STATE/ai/*` |
| `deferred_intent_outbox` | `user_owned_recovery_state` | `$AURELINE_STATE/outbox/*` |
| `admin_policy` | `admin_or_tenant_control_artifact` | `$AURELINE_POLICY/aureline.policy.signed` |
| `policy_bundle_cache` | `admin_or_tenant_control_artifact` | `$AURELINE_STATE/policy/*` |
| `tenant_entitlement_snapshot` | `admin_or_tenant_control_artifact` | `$AURELINE_STATE/entitlements/*` |
| `admin_audit_log` | `admin_or_tenant_control_artifact` | `$AURELINE_STATE/audit/*` |
| `trust_approvals` | `admin_or_tenant_control_artifact` | `$AURELINE_STATE/trust/approvals/*` |
| `auth_session_secrets` | `admin_or_tenant_control_artifact` | OS credential store handles |
| `long_lived_credentials` | `admin_or_tenant_control_artifact` | OS credential store handles |
| `execution_context_cache` | `disposable_derived_state` | `$AURELINE_STATE/contexts/*` |
| `index_cache_store` | `disposable_derived_state` | `$AURELINE_STATE/cache/*` |
| `knowledge_cache` | `disposable_derived_state` | `$AURELINE_STATE/knowledge/*` |
| `prebuild_environment_cache` | `disposable_derived_state` | `$AURELINE_STATE/prebuilds/*` |
| `shared_object_store` | `disposable_derived_state` | `$AURELINE_STATE/objects/*` |
| `logs_and_traces` | `disposable_derived_state` | `$AURELINE_STATE/logs/*` |
| `support_bundles` | `evidence_bundle` | `$AURELINE_STATE/support/bundles/*` |
| `crash_envelopes` | `evidence_bundle` | `$AURELINE_STATE/support/crash/*` |
| `review_packets` | `evidence_bundle` | `$AURELINE_STATE/support/review/*` |
| `incident_bundles` | `evidence_bundle` | `$AURELINE_STATE/support/incidents/*` |
| `benchmark_results` | `evidence_bundle` | `$AURELINE_STATE/benchmarks/results/*` |
| `release_evidence_packs` | `evidence_bundle` | `$AURELINE_STATE/release/evidence/*` |

## Invariants

1. Every launch-relevant state family has exactly one path row id,
   one artifact class, and at least one scope tag.
2. Supportability, portability, backup, recovery, and cache-clear
   flows MUST quote this row id and use the same class vocabulary.
3. A row whose `never_delete_silently` value is `true` may not be
   targeted by low-disk reclamation, generic reset, or ordinary cache
   clear without the row's declared preview or authority-specific
   path.
4. A path row whose `secret_posture` is
   `raw_secret_external_store_only` or `credential_handle_only` may
   export only opaque handles, omission records, or redacted metadata.
5. A row whose `artifact_class` is `admin_or_tenant_control_artifact`
   may be refreshed or replaced only through signed policy,
   entitlement, trust, or audit authority. User preference import does
   not modify it.
6. A disposable derived row may be cleared or rebuilt, but a consumer
   must not infer durable truth from its presence. Source truth always
   comes from the durable, recovery, control, or evidence row that
   produced it.

## Source Anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix F:
  configuration and state map.
- `.t2/docs/Aureline_Technical_Design_Document.md` durable-state
  classes, canonical human-readable state, execution-context
  provenance, and recovery reset postures.
- `.t2/docs/Aureline_PRD.md` resource governance, data portability,
  execution-context, policy, and cache/low-disk rules.
- `.t2/docs/Aureline_Milestones_Document.md` durable-state,
  migration, local-history, recovery, and enterprise policy bars.
- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
  for state-map row vocabulary.
- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md)
  for schema-evolution and corruption-routing posture.
- [`/docs/identity/offline_entitlement_and_policy_seed.md`](../identity/offline_entitlement_and_policy_seed.md)
  for policy, entitlement, and admin-audit objects.
- [`/docs/ai/memory_and_reconciliation_contract.md`](../ai/memory_and_reconciliation_contract.md)
  for AI memory and deferred-intent per-class delete/export posture.
- [`/docs/execution/terminal_truth_contract.md`](../execution/terminal_truth_contract.md)
  for terminal scrollback, restore metadata, and no-rerun truth.
- [`/docs/reliability/local_history_contract.md`](../reliability/local_history_contract.md)
  for local-history clear and recovery rules.
