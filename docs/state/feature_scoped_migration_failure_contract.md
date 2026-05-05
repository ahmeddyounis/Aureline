# Feature-scoped migration-failure, unknown-field preservation, and partial-open contract

This document freezes the contract Aureline uses when schema drift,
migration, or integrity failures affect persisted workspace artifacts.
The goal is to prevent failures from collapsing into destructive
all-or-nothing resets. Instead, the product:

- keeps the workspace open when safe;
- preserves unknown fields where the artifact family promises it;
- disables or degrades only the feature families whose correctness
  depends on the failing artifact; and
- emits a typed, export-safe explanation and next actions.

Machine-readable companions:

- [`/schemas/state/migration_failure_state.schema.json`](../../schemas/state/migration_failure_state.schema.json)
  — boundary schema for the emitted state record.
- [`/artifacts/state/unknown_field_preservation_rules.yaml`](../../artifacts/state/unknown_field_preservation_rules.yaml)
  — frozen unknown-field posture by artifact family class.

Worked fixtures live under:

- [`/fixtures/state/migration_failure_cases/`](../../fixtures/state/migration_failure_cases/)

This contract composes with, and does not replace:

- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md)
- [`/artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml)
- [`/artifacts/state/corruption_routing_matrix.yaml`](../../artifacts/state/corruption_routing_matrix.yaml)
- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
- [`/docs/state/durable_state_compatibility_contract.md`](./durable_state_compatibility_contract.md)
- [`/docs/config/human_edited_artifact_contract.md`](../config/human_edited_artifact_contract.md)
- [`/artifacts/config/format_selection_matrix.yaml`](../../artifacts/config/format_selection_matrix.yaml)
- [`/docs/ux/degraded_mode_pattern.md`](../ux/degraded_mode_pattern.md)
- [`/docs/ux/state_and_recovery_taxonomy.md`](../ux/state_and_recovery_taxonomy.md)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, the state-object inventory, or the format-selection matrix,
those upstream sources win and this contract plus its schema, fixtures,
and companion artifact MUST be updated in the same change. Where a
downstream surface (workspace open, Start Center, Project Doctor, repair
flow, Migration Center, support export) invents parallel “reset”
behavior, “best effort” unknown-field handling, or feature-disable copy,
this contract wins and the surface is non-conforming.

## Why freeze this now

Schema drift and migration failures are inevitable: users pin versions,
teams roll back, extensions introduce `x-*` blocks, and generated
artifacts evolve. Drift becomes destructive when the product treats
“cannot fully interpret this artifact” as permission to:

- silently rewrite a human-edited file and drop unknown fields;
- perform a full workspace reset because one feature’s config is
  unreadable; or
- hide a schema-version mismatch behind a generic “corrupt / reset”
  prompt.

This contract forecloses those patterns by freezing: which artifact
families preserve unknown fields, which ones refuse unknown fields, how
unsupported schema versions route, which features disable, and which
partial-open posture the workspace must present.

## Scope

- Freeze the feature-scoped migration-failure decision contract for
  persisted artifacts that gate core workflows: tasks/launch,
  extension lockfiles, policy artifacts, AI instruction/policy files,
  portability bundles, support bundles, and cache/index shards.
- Freeze unknown-field preservation rules by artifact family class,
  explicitly varying the rule by family rather than assuming one
  universal posture.
- Freeze the partial-open rules: when the workspace stays open, which
  feature families must be disabled/degraded, which actions remain safe,
  and which next actions are offered.
- Provide seed cases for:
  1) recoverable unknown fields,
  2) unsupported new schema version,
  3) lossy downgrade refused,
  4) corruption that falls back to read-only / compare-only.

## Out of scope

- Implementing migration code, schema translators, lockfile resolvers,
  signature verification, or recovery-ladder execution.
- Final UI copy strings. This contract freezes the machine vocabulary
  and required explanation fields; surfaces render localized copy over
  them.
- Exhaustively enumerating every future artifact family. New families
  add rows to the state-object inventory and the format-selection matrix
  and extend the unknown-field rules artifact when needed.

## 1. Artifact classes and ownership

This contract discusses failures in terms of *artifact family classes*.
The classes align with the state-object inventory and the durable-state
compatibility contract:

- `user_owned_durable` — user-authored durable truth (settings,
  keybindings, snippets, themes, etc).
- `workspace_owned_durable` — workspace-authored durable truth (workspace
  manifests, worksets, tasks/launch, extension lockfiles, etc).
- `admin_or_control` — signed policy, trust, and control artifacts whose
  authority is not the user (policy bundle epochs, trust approvals).
- `cache_or_index` — derived disposable caches and indexes (rebuildable
  by definition).
- `generated_or_structured` — generated structured artifacts that may be
  inspectable and comparable but are not authoritative truth.
- `evidence_or_support` — evidence packets and support bundles that are
  content-addressed and never “migrated in place”.

The class determines two things:

1. Whether unknown-field preservation is promised.
2. Whether a failure disables only a feature or blocks privileged
   operations / workspace open.

## 2. Unknown-field preservation rules

Unknown-field handling is governed by a closed posture vocabulary shared
with schema-migration records:

- `preserve_verbatim` — readers accept additive fields and writers
  preserve them verbatim.
- `preserve_under_namespaced_key` — unknown fields are preserved under a
  declared namespaced extension block (for example, `x-preserved/*`).
- `drop_with_disclosure` — unknown fields may be dropped only when the
  writer routes through an explicit preview + rollback checkpoint and
  stamps the disclosure surfaces declared by the format policy.
- `refuse_read` — the artifact refuses unknown fields; the product must
  not guess. The dependent feature is disabled until the artifact is
  regenerated or the reader is upgraded.

The frozen posture by artifact family class lives in
[`/artifacts/state/unknown_field_preservation_rules.yaml`](../../artifacts/state/unknown_field_preservation_rules.yaml).

Rules (frozen):

1. When `unknown_field_posture = preserve_verbatim`, the product MUST
   keep the workspace open and MUST NOT offer a “reset” action as the
   first response. Unknown-field presence is a degraded detail, not a
   corruption verdict.
2. When `unknown_field_posture = refuse_read`, the product MUST treat
   the artifact as unreadable for semantic operations and MUST disable
   only the dependent feature family by default (for example, extension
   resolution when the lockfile refuses unknown fields).
3. A writer MUST NOT switch postures ad hoc. Changing an artifact
   family’s unknown-field posture is a governance change: it requires an
   explicit format-matrix row update plus migration-row disclosure.

## 3. Feature-scoped migration-failure routing

When an artifact cannot be read or cannot be migrated, the product
emits a `migration_failure_state_record` and routes through one of these
outcomes:

- **Feature-disabled, workspace continues** — default for user/workspace
  durable artifacts. The workspace opens; the dependent feature family
  is disabled or degraded, and the artifact is left unmodified.
- **Privileged-ops blocked, local continuity preserved** — default for
  admin/control artifacts. Local editing continues under the last-known-
  good snapshot; privileged operations refuse until authority refreshes.
- **Rebuild automatically** — default for cache/index artifacts. The
  derived store is discarded and rebuilt; the workspace stays open.
- **Read-only / compare-only fallback** — default for generated or
  structured artifacts. The product may inspect or compare, but it must
  not imply safe mutation.
- **Workspace open blocked** — reserved for failures that prevent
  establishing an authoritative root set or trust boundary. If used, the
  record MUST explain why no partial-open posture is safe.

Rules (frozen):

1. A failure in `tasks_and_launch_configs` disables task run/debug
   actions only; editing and navigation remain available.
2. A failure in `extension_lockfile` disables lockfile-based extension
   pinning and privileged publish/apply actions; editing continues.
3. A failure in signed policy artifacts (`admin_policy_bundle`,
   `policy_bundle_cache`, trust approvals) MUST fall back to last-known-
   good and MUST refuse privileged operations; non-privileged editing
   continues with explicit warning.
4. A failure in `cache_or_index` artifacts MUST NOT delete or rewrite
   user/workspace durable truth; it rebuilds derived state only.

## 4. Partial-open and messaging contract

When any feature family is disabled due to this contract, the workspace
is considered partial-open. Surfaces MUST follow the UI/UX degraded-mode
pattern:

- name the affected domain (tasks/debug, policy, extensions, indexing,
  etc);
- state what still works;
- state what is disabled/degraded/read-only/compare-only;
- provide the narrowest next-safe action (upgrade, regenerate through
  preview, repair flow, rebuild cache, refetch signed epoch, open in text
  mode, export for support).

The `migration_failure_state_record` is the machine carrier for those
fields; surfaces render copy and controls over it. A generic “Reset
workspace” prompt is non-conforming when a narrower, feature-scoped
posture exists.

## 5. Seed cases

| Fixture | Primary point |
|---|---|
| [`unknown_fields_preserved_tasks.yaml`](../../fixtures/state/migration_failure_cases/unknown_fields_preserved_tasks.yaml) | Unknown fields in a human-edited tasks file are preserved; tasks remain runnable. |
| [`unsupported_schema_version_tasks.yaml`](../../fixtures/state/migration_failure_cases/unsupported_schema_version_tasks.yaml) | Unsupported schema version disables tasks only; workspace remains open. |
| [`lossy_downgrade_refused_lockfile.yaml`](../../fixtures/state/migration_failure_cases/lossy_downgrade_refused_lockfile.yaml) | Generator-owned lockfile refuses unknown fields; lossy downgrade is refused; extension pinning disables without reset. |
| [`corruption_generated_artifact_compare_only.yaml`](../../fixtures/state/migration_failure_cases/corruption_generated_artifact_compare_only.yaml) | Generated structured artifact corruption routes to compare-only/read-only fallback; rebuild is optional and explicit. |

