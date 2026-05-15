# External alpha schema and record registry proof packet

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T17:24:31Z
stale_after: P14D
source_revision: git:7ef49d38b543d94113d56e1b3aa289eea9e62c2e
trigger_revision: alpha_schema_record_registry_contract_set@2026-05-15
validator: ci/check_schema_record_registry_alpha.py
validation_capture: artifacts/milestones/m2/captures/schema_record_registry_alpha_validation_capture.json
claim_change_state: no_claim_widening
```

This packet registers the alpha schema registry, record-class registry,
managed-copy/local-truth reviewer page, validator, fixture manifest, and
support/export projection as the current proof root for state,
support/export, portable-package, managed-copy, retention, delete, hold,
and receipt truth.

## Canonical Artifacts

- Schema registry: `artifacts/governance/schema_registry_alpha.yaml`
- Record registry: `artifacts/governance/record_class_registry_alpha.yaml`
- Reviewer page: `docs/governance/managed_copy_vs_local_truth.md`
- Protected fixture manifest:
  `fixtures/governance/schema_record_registry_alpha_cases/manifest.yaml`
- Validator and first consumer: `ci/check_schema_record_registry_alpha.py`
- Latest capture:
  `artifacts/milestones/m2/captures/schema_record_registry_alpha_validation_capture.json`
- Alpha scope row: `scoreboard_row:alpha_scope.schema_record_registry`

## Acceptance Coverage

The registries cover:

- durable workspace and restore state;
- support bundle archives;
- portable state packages;
- managed-copy index and managed-workspace lifecycle copies;
- usage and offboarding export packets;
- destruction receipts; and
- AI retained evidence packets where managed retention is enabled.

Every alpha record row carries an owner, schema row refs, local truth
authority, managed-copy posture, retention label, hold semantics, delete
semantics, export semantics, and consumer refs.

## First Consumer

Run the support/export projection:

```sh
python3 ci/check_schema_record_registry_alpha.py --repo-root . --render-support-export-projection
```

Refresh the checked-in validation capture:

```sh
python3 ci/check_schema_record_registry_alpha.py --repo-root . --report artifacts/milestones/m2/captures/schema_record_registry_alpha_validation_capture.json
```

## Current Posture

This is a governed alpha proof scaffold. It does not implement runtime
storage, support upload, managed retention, delete jobs, or offboarding
assemblers. Downstream support, product-boundary, docs, CLI, and export
lanes should cite these registries before widening local-only,
managed-copy, support/export, or delete-completion claims.
