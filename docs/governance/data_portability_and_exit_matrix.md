# Data-portability, export, and customer-exit matrix

This document is the human-readable companion to the data-portability
artifact matrix. It exists so Aureline's "local-first" and "open"
claims resolve to an auditable per-domain contract — minimum export
format, scriptability, admin-vs-user scope, deletion story,
import unknown-field preservation, and the rules that keep local-core
workflows and export APIs truthful after staged migration, self-host
move, air-gap, or SaaS cancellation — *before* any managed or hosted
row hardens.

Companion artifacts:

- [`/artifacts/governance/portability_artifact_matrix.yaml`](../../artifacts/governance/portability_artifact_matrix.yaml)
  — machine-readable row register. Every row conforms to the row
  schema below.
- [`/schemas/governance/portability_row.schema.json`](../../schemas/governance/portability_row.schema.json)
  — boundary schema for one `portability_row`.
- [`/fixtures/governance/portability_cases/`](../../fixtures/governance/portability_cases/)
  — fixture cases the matrix is structurally challenged against.

Adjacent artifacts the matrix composes over and never replaces:

- [`./record_class_governance.md`](./record_class_governance.md) and
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — class-level retention, hold, delete, and offboarding posture. The
  registry is the ceiling; this matrix narrows the user-facing
  artifact contract under it.
- [`./storage_and_retention_vocabulary.md`](./storage_and_retention_vocabulary.md)
  and [`/artifacts/governance/storage_modes.yaml`](../../artifacts/governance/storage_modes.yaml)
  — storage / retention / redaction vocabulary every export row reuses
  verbatim. Free-text substitutes are non-conforming.
- [`../service/managed_service_seed.md`](../service/managed_service_seed.md),
  [`/artifacts/service/retention_rows.yaml`](../../artifacts/service/retention_rows.yaml),
  and [`/schemas/service/deletion_job_record.schema.json`](../../schemas/service/deletion_job_record.schema.json)
  — managed-service SLO, retention, and deletion semantics that
  withdrawal rules cite when the managed surface withdraws.
- [`../state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  and [`/schemas/profile/portable_profile.schema.json`](../../schemas/profile/portable_profile.schema.json)
  — portable-profile artifact, state map, and restore-provenance
  vocabulary user-scoped exports project onto.

## Why the matrix exists

"Local-first" and "open" become marketing language unless every
artifact domain that influences user exit, self-host move, or
migration carries an explicit export, deletion, and offboarding
posture. Without that contract:

- managed surfaces silently take ownership of artifacts the user
  reasonably expects to be local;
- offboarding stories collapse into a vague "contact support" promise
  that nothing in the product enforces;
- export APIs drift from the documented schema once the managed
  control plane changes shape;
- import surfaces silently drop fields they do not recognise, so
  round-tripping a profile across versions becomes lossy in ways the
  user can't detect.

The matrix forecloses these patterns by treating data-portability as
a contract: every artifact domain resolves to one row, every row
seeds at least one rule per applicable withdrawal scenario, and every
managed-only or hybrid artifact is structurally visible and
challengeable rather than hidden behind a vendor-only store.

## Seeded artifact domains

The seed register intentionally starts with the artifact domains most
likely to drift into managed-only behavior first. Every domain
resolves to one `artifact_domain_class` value; adding a new value is
additive-minor and bumps `portability_row_schema_version`.

| Artifact domain | Local vs managed posture | Admin / user scope | Offboarding story |
|---|---|---|---|
| `settings_keybindings_profiles_snippets` | local-authoritative, optional managed mirror | user-scope only | user self-serve export |
| `workspaces_tasks_launch_policy_files` | local-authoritative, optional managed mirror | user with admin visibility | user self-serve export |
| `extension_inventory_and_registry_policy` | local-authoritative, optional managed mirror | user and admin distinct | admin-packaged export |
| `local_history_and_ai_evidence` | local-authoritative for history, hybrid for AI evidence | user and admin distinct | exit-packet required |
| `collaboration_comments_and_audit` | managed-authoritative, local cache | admin scope only | admin-packaged export |
| `managed_templates_and_prebuilds` | managed-authoritative, local cache | admin scope only | admin-packaged export |
| `usage_and_billing_exports` | generated packet only | admin scope only | user self-serve export |
| `profile_library_artifacts` | local-authoritative, optional managed mirror | user-scope only | user self-serve export |
| `sync_metadata_and_conflict_journals` | managed-authoritative, local cache | user-scope only | user self-serve export |
| `policy_bundles` | managed-authoritative, local cache | admin scope only | admin-packaged export |
| `extension_lockfiles_and_recommendations` | local-only | user-scope only | user self-serve export |
| `execution_context_summaries` | local-authoritative, optional managed mirror | user with admin visibility | user self-serve export |

The seed is intentionally narrow. Every additional artifact class
that influences exit, self-host move, or migration MUST land a row
here in the same change that introduces the class. Vague export or
deletion stories for hybrid or managed-bearing classes are
non-conforming once the matrix is seeded.

## Row shape

Every `portability_row` keeps the contract axes separate:

- `artifact_domain` resolves to one closed-set domain value so the
  row schema and consumer surfaces share a vocabulary.
- `scope_posture` names admin-vs-user scope and managed-vs-local
  dependency separately so admin-scoped exports never silently widen
  a user-scoped request.
- `export_posture` names the minimum export format, additional
  formats, scriptability requirement, manifest expectation, and
  raw-secret-exclusion expectation. The minimum format is
  contractual; additional formats are optional.
- `import_posture` names the unknown-field rule and round-trip
  expectation so import surfaces never silently drop fields they do
  not recognise.
- `deletion_posture` keeps local-vs-managed deletion actions, hold
  blocking, and completion evidence separate from each other.
- `offboarding_posture` names whether the artifact is user
  self-serve, admin packaged, exit-packet required, or manual local
  capture, and whether obtaining the export currently depends on a
  support ticket. The acceptance bar is `support_ticket_required:
  false` for at least one fixture per major export class.
- `service_withdrawal_rules` enumerates one rule per applicable
  scenario (`staged_migration`, `self_host_move`,
  `air_gap_continuity`, `saas_cancellation`,
  `managed_only_withdrawal`). Hybrid or managed-bearing rows MUST
  cover the first four; rows whose live capability cannot survive
  managed withdrawal MUST surface the loss explicitly with a
  `managed_only_withdrawal` rule rather than implying silent
  equivalence.
- `governance_links` cites at least one record-class id and one
  storage-mode consumer id so retention and redaction posture is
  inherited rather than re-declared.

These objects are deliberately not merged into one `portability`
field. Export, import, deletion, offboarding, and withdrawal-rule
behavior are distinct contracts and the matrix keeps them distinct.

## How withdrawal rules stay honest

Each row seeds explicit rules for the four service-withdrawal
scenarios so local-core workflows and export APIs never quietly drift
once a managed surface withdraws:

- **Staged migration.** Schemas, signing chains, and importer rules
  must remain interpretable across migration windows. A rule with
  `local_core_workflow_remains_truthful: true` and
  `export_api_remains_truthful: true` is a contract; reviewers can
  challenge a row that drops to `false` without naming the surface
  that takes over.
- **Self-host move.** Every hybrid or managed-bearing row must
  describe how the artifact survives a move to a customer-managed
  control plane without managed-cloud dependence. Self-host parity
  rows in adjacent matrices (qualification, locality, claim manifest)
  read the same row.
- **Air-gap continuity.** Air-gap deployments cannot reach a managed
  control plane; the rule must name what carries the artifact (a
  mirrored bundle, a local journal, a previously emitted packet) or
  flag the loss with a `managed_only_withdrawal` rule.
- **SaaS cancellation.** Cancellation must trigger the contractually
  promised exports before access ends. The matrix forecloses
  cancellation flows that depend on filing a support ticket; the
  row's `support_ticket_required` value is the canonical answer.
- **Managed-only withdrawal.** Reserved for artifacts that cannot
  survive managed withdrawal at all. Rather than hiding the loss
  behind a vague offline mode, the matrix surfaces it with
  `local_core_workflow_remains_truthful: false` and (when
  applicable) `manual_capture_required: true` so reviewers see the
  cost of the dependency.

Every rule that sets `local_core_workflow_remains_truthful: false`
or `export_api_remains_truthful: false` MUST cite a manual-capture
note or the surface that takes over; silent loss is non-conforming.

## How other lanes use it

- **Boundary reviews** consult the matrix when a managed claim moves
  bytes off device, retains evidence, or promises an exit packet, so
  the user-visible export and deletion contract is named at the same
  time as the managed claim.
- **Supportability** uses the row to keep export and offboarding
  surfaces honest about what runs without a support ticket, which
  fixture demonstrates the path, and which artifact partial-result
  causes apply.
- **Settings, profile, sync, and policy lanes** project their UI
  copy onto the row's storage / retention / redaction vocabulary so
  generated docs, manifests, and admin explainers never invent
  parallel labels.
- **Schema-registry and openapi work** quote `portability_row_id`
  when rendering an export endpoint, instead of inventing a second
  "portability class" vocabulary in generated docs.

## Change discipline

Adding or changing a portability row requires all of the following in
the same change:

1. Add or update the row in
   [`portability_artifact_matrix.yaml`](../../artifacts/governance/portability_artifact_matrix.yaml).
2. If the change introduces new vocabulary, extend
   [`portability_row.schema.json`](../../schemas/governance/portability_row.schema.json)
   and bump `portability_row_schema_version`.
3. Cite at least one `record_class_id` from the record-class registry
   and one consumer id from the storage-mode register so retention
   and redaction posture is inherited rather than re-declared.
4. Seed at least one rule per applicable withdrawal scenario. Hybrid
   or managed-bearing rows MUST cover staged-migration, self-host
   move, air-gap, and SaaS cancellation; rows whose live capability
   cannot survive managed withdrawal MUST add a
   `managed_only_withdrawal` rule.
5. Add or refresh a fixture under
   [`/fixtures/governance/portability_cases/`](../../fixtures/governance/portability_cases/)
   when the row introduces a new export class so the contract stays
   structurally challengeable.

## Versioning rules

- Adding a new row is additive.
- Adding a new enum value to the schema is additive-minor and
  requires a `portability_row_schema_version` bump plus a doc update
  here.
- Repurposing an existing enum value or reusing a `portability_row_id`
  for a meaningfully different artifact domain is breaking and
  requires a new decision row plus a superseding registry row.

## What this matrix is not

- It is **not** the per-record state machine — that remains in the
  record-class registry and the record-state-and-policy simulation
  models.
- It is **not** the export-API implementation. The matrix names the
  contract a future implementation must satisfy; the implementation
  itself lands in later milestones.
- It is **not** a replacement for the boundary manifest. The
  boundary manifest says *which capability* is local-core or
  managed; this matrix says *which artifact contracts* every
  capability creates and how those artifacts behave under export,
  deletion, and offboarding.
