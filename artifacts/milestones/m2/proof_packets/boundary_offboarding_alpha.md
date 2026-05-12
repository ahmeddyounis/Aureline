# Proof packet: boundary and offboarding baseline

## Scope

This packet covers the alpha boundary manifest, entitlement snapshot,
org-switch posture, usage-export baseline, and offboarding packet skeleton.
It proves that managed-boundary labels, quota/meter rows, entitlement states,
and exit-package references can be rendered from one canonical artifact set.

## Canonical artifacts

- `schemas/governance/boundary_manifest_v1.schema.json`
- `artifacts/governance/boundary_manifest_alpha.yaml`
- `artifacts/governance/entitlement_snapshot_alpha.yaml`
- `artifacts/governance/org_switch_posture_alpha.yaml`
- `artifacts/governance/usage_export_baseline_alpha.yaml`
- `docs/governance/boundary_offboarding_packet_skeleton.md`
- `fixtures/governance/boundary_offboarding_alpha_cases/manifest.yaml`
- `ci/check_boundary_offboarding_alpha.py`

## First consumer

The first consumer is the CLI/support-export projection rendered by:

```sh
python3 ci/check_boundary_offboarding_alpha.py --repo-root . --render-consumer-projection
```

The projection emits the fields that docs/help, admin, support-export,
CLI/headless, and release-evidence surfaces need: capability id, boundary
class, local/self-host/mirror alternatives, identity and network boundary,
entitlement posture, quota meter ids, usage-export refs, and offboarding refs.

## Protected states

The protected fixture manifest exercises:

- boundary rows for claimed managed/support/export lanes;
- local-core rows with no managed prerequisite;
- signed-out, grace, restricted-managed-only, and offboarded entitlement states;
- org-switch reauth and revocation;
- usage-export meter reconciliation;
- offboarding packets that reference usage exports without embedding them;
- the consumer projection.

## Validation

Run:

```sh
python3 ci/check_boundary_offboarding_alpha.py --repo-root .
python3 ci/check_boundary_offboarding_alpha.py --repo-root . --render-consumer-projection
python3 ci/check_boundary_offboarding_alpha.py --repo-root . --report artifacts/milestones/m2/captures/boundary_offboarding_alpha_validation_capture.json
```

The capture is expected at
`artifacts/milestones/m2/captures/boundary_offboarding_alpha_validation_capture.json`.

## Closure checks

- Local-core capabilities keep editing, command invocation, local Git, local
  profiles, local/BYOK AI where configured, and local support bundles free of
  managed identity, quota, usage-export, or offboarding gates.
- Managed rows name a local alternative and a self-hosted or mirrored path.
- Entitlement and org-switch states narrow managed actions without widening or
  blocking local core.
- Usage-export samples use the same meter ids, quota families, and units as
  the boundary manifest.
- Offboarding packets reference sibling export families by opaque id and keep
  held or policy-retained data distinct from destroyed data.
