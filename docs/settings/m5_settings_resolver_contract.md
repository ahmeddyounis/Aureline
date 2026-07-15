# M5 Setting-Definition, Write-Intent, Sync-Conflict, and Capability-Lifecycle Contract

Status: frozen (B143 opening matrix)

This contract freezes Aureline's concrete settings-resolver, sync-conflict, and capability-lifecycle
runtime behavior into one export-safe matrix. It is the canonical source of configuration-runtime truth for
M5: later settings UI, sync/device flows, policy and capability services, Doctor/support diagnostics, claim
publication, and release-evidence tooling consume it directly rather than copying settings-row prose or
admin copy by hand.

- Matrix schema: `schemas/config/m5-settings-resolver-matrix.schema.json`
- Setting-definition domain schema (resolve-setting / migrate-schema): `schemas/config/m5-setting-definition.schema.json`
- Setting-write-intent domain schema (write-setting): `schemas/config/m5-setting-write-intent.schema.json`
- Sync-conflict-packet domain schema (sync-scope): `schemas/config/m5-sync-conflict-packet.schema.json`
- Capability-lifecycle domain schema (rollout-capability): `schemas/config/m5-capability-lifecycle.schema.json`
- Support export: `artifacts/release/m5-settings-governance-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-settings-governance-proof/matrix.csv`
- Design report: `artifacts/config/m5-settings-resolver-matrix.md`
- Narrowed fixtures: `fixtures/config/m5-settings-runtime/`
- Authoritative validator: `crates/aureline-ui` (`m5_settings_governance_matrix`)
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_settings_governance_matrix`

## Governed configuration-runtime families

The matrix freezes **five** configuration-runtime families, each qualified independently and each pointing
at one canonical domain schema:

| Family | Configuration concern | Owner | Domain schema |
| --- | --- | --- | --- |
| `resolve_setting` | Resolve the effective value from the winning scope; never recycle a stable setting ID | Settings-resolver owner | setting-definition |
| `write_setting` | Land the write in the chosen artifact and scope with preview / checkpoint / rollback evidence | Settings-write owner | setting-write-intent |
| `sync_scope` | Surface conflicts rather than silently overwrite local authoritative state | Sync-service owner | sync-conflict-packet |
| `migrate_schema` | Preserve setting-ID continuity across versions with a reversible checkpoint | Migration-service owner | setting-definition |
| `rollout_capability` | Keep lifecycle / kill-switch causes visible and self-explaining | Capability-lifecycle owner | capability-lifecycle |

## Shared settings-governance-role vocabulary

Every consumer binds to one controlled role vocabulary; no surface invents a parallel word:

`setting_definition`, `effective_resolution`, `write_intent`, `policy_constraint`, `sync_conflict`,
`schema_migration`, `capability_lifecycle`.

The write-intent / policy-constraint / sync-conflict / capability-lifecycle roles (`write_intent`,
`policy_constraint`, `sync_conflict`, `capability_lifecycle`) must preserve evidence and disclose cause
before applying — a write may never widen a scope, sync may never silently overwrite local authoritative
state, and a lifecycle may never hide a dependency behind unpublished markers or a kill-switch / policy-
disable cause behind generic unavailable copy. The descriptive structure roles (`setting_definition`,
`effective_resolution`, `schema_migration`) are inspectable descriptors.

## Hard invariants

Every row carries five hard-invariant booleans that must be `false`, and the governance-review block
asserts the corresponding fleet-level guarantees:

1. A retired setting ID is never recycled.
2. A scoped (Workspace/Profile) write is never rewritten into a broader (User/Machine) scope because it is
   easier downstream.
3. Locked or machine-only state is never silently overwritten during sync.
4. A lifecycle or experiment dependency is never hidden behind unpublished markers.
5. A kill-switch or policy-disable cause is never hidden behind generic unavailable copy.

## Automatic narrowing

Claim publication and support/export narrow configuration claims automatically when the B143 registry is
missing, stale, or not yet qualified. Two narrowed fixtures demonstrate honest narrowing while keeping every
family visible:

- `sync_scope_beta_narrowed.json` — sync scope held at **Beta** pending local-authoritative-state continuity
  across every configuration context.
- `rollout_capability_preview_narrowed.json` — rollout capability narrowed to **Preview** pending complete
  capability-lifecycle evidence.

## Bound source contracts

The matrix binds back to already-landed truth so configuration truth is never split across scattered
settings notes: the effective-setting schema (`schemas/config/effective_setting.schema.json`) and the
capability-lifecycle schema (`schemas/governance/capability_lifecycle.schema.json`).
