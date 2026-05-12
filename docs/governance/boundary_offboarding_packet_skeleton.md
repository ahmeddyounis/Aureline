# Boundary and offboarding packet skeleton

This skeleton is the reviewer entry point for alpha managed-boundary,
entitlement, org-switch, usage-export, and offboarding truth. It exists so
docs/help, admin, CLI/headless, and support-export surfaces render the same
labels instead of inventing local wording for open/local, self-hostable,
mirrored, managed, quota-gated, restricted, and offboarded states.

## Canonical artifacts

- [`/schemas/governance/boundary_manifest_v1.schema.json`](../../schemas/governance/boundary_manifest_v1.schema.json)
  defines the capability-row contract.
- [`/artifacts/governance/boundary_manifest_alpha.yaml`](../../artifacts/governance/boundary_manifest_alpha.yaml)
  names capability ids, boundary classes, local/open alternatives,
  self-host alternatives, identity/network boundaries, quota meters, and
  lifecycle metadata.
- [`/artifacts/governance/entitlement_snapshot_alpha.yaml`](../../artifacts/governance/entitlement_snapshot_alpha.yaml)
  freezes active, signed-out, grace, restricted-managed-only, and offboarded
  posture.
- [`/artifacts/governance/org_switch_posture_alpha.yaml`](../../artifacts/governance/org_switch_posture_alpha.yaml)
  applies the entitlement states to org switching and credential invalidation.
- [`/artifacts/governance/usage_export_baseline_alpha.yaml`](../../artifacts/governance/usage_export_baseline_alpha.yaml)
  reconciles usage-export rows, meter definitions, quota families, and
  offboarding packet refs.
- [`/fixtures/governance/boundary_offboarding_alpha_cases/manifest.yaml`](../../fixtures/governance/boundary_offboarding_alpha_cases/manifest.yaml)
  lists the protected proof states.
- [`/ci/check_boundary_offboarding_alpha.py`](../../ci/check_boundary_offboarding_alpha.py)
  validates the artifacts and renders the first consumer projection.

## Inherited contracts

This packet consumes, rather than restates:

- [`/artifacts/milestones/m2/alpha_wedge_matrix.yaml`](../../artifacts/milestones/m2/alpha_wedge_matrix.yaml)
  for alpha claim scope and the managed-cloud daily-driver exclusion.
- [`/artifacts/governance/schema_registry_alpha.yaml`](../../artifacts/governance/schema_registry_alpha.yaml)
  and [`/artifacts/governance/record_class_registry_alpha.yaml`](../../artifacts/governance/record_class_registry_alpha.yaml)
  for export, managed-copy, and record-class truth.
- [`/docs/governance/usage_export_and_offboarding_contract.md`](./usage_export_and_offboarding_contract.md)
  for usage-export and exit-packet linkage rules.
- [`/docs/managed/metering_and_usage_export_contract.md`](../managed/metering_and_usage_export_contract.md)
  for meter authority, quota family, unit, time basis, and caveat vocabulary.

## Required rendering model

Every consuming surface should render from the validator projection:

```sh
python3 ci/check_boundary_offboarding_alpha.py --repo-root . --render-consumer-projection
```

The projection carries:

- capability id, boundary class, and claim status;
- open/local, self-hosted, and mirrored/offline alternatives;
- identity and network boundary requirements;
- entitlement-state posture and org-switch behavior;
- quota meter ids, quota family, quota unit, and usage-export packet refs;
- offboarding packet refs and held-data notes.

Surfaces must not replace these fields with ad hoc labels such as “paid”,
“cloud”, “disabled”, or “deleted” when a more precise state exists.

## Non-widening rules

- Local-core rows must have no managed identity requirement, no live network
  requirement, no quota meter, and no usage-export prerequisite.
- Managed rows must name an open/local alternative and, where applicable, a
  self-hosted or mirrored alternative.
- Signed-out, grace, restricted-managed-only, and offboarded states keep local
  editing, command invocation, local Git, local profile files, local/BYOK AI
  where configured, and local support-bundle generation available.
- Usage exports reference the same meter ids, quota families, and units that
  the boundary manifest exposes.
- Offboarding packets reference usage-export packets by opaque id. They do not
  embed sibling payloads or report held/policy-retained data as destroyed.

## Proof path

The protected proof path is data-driven:

1. Validate the artifact set:

   ```sh
   python3 ci/check_boundary_offboarding_alpha.py --repo-root .
   ```

2. Render the consumer projection and confirm docs/help, admin, support-export,
   and CLI fields all come from the same source:

   ```sh
   python3 ci/check_boundary_offboarding_alpha.py --repo-root . --render-consumer-projection
   ```

3. Refresh the release-evidence capture when the lane changes:

   ```sh
   python3 ci/check_boundary_offboarding_alpha.py --repo-root . --report artifacts/milestones/m2/captures/boundary_offboarding_alpha_validation_capture.json
   ```

The validator fails closed if a local-core row gains managed prerequisites, a
managed row loses its local or self-host/mirror alternative, an entitlement
state fails to preserve local-core availability, or a usage/offboarding sample
does not reconcile to the manifest meter definitions.
