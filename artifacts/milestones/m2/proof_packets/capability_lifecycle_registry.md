# Proof Packet: External Alpha Capability Lifecycle Registry

## Scope

This packet proves that launch-wedge lifecycle state and dependency
markers are represented as a canonical registry, schema, validator, and
first consumer projection.

## Canonical Artifacts

- Registry:
  [`artifacts/governance/capability_lifecycle_registry.yaml`](../../../governance/capability_lifecycle_registry.yaml)
- Dependency-marker schema:
  [`schemas/governance/dependency_marker.schema.json`](../../../../schemas/governance/dependency_marker.schema.json)
- Human entry point:
  [`docs/governance/m2_capability_lifecycle.md`](../../../../docs/governance/m2_capability_lifecycle.md)
- Protected fixture manifest:
  [`fixtures/governance/capability_lifecycle_registry_cases/manifest.yaml`](../../../../fixtures/governance/capability_lifecycle_registry_cases/manifest.yaml)
- Validator and first consumer:
  [`ci/check_capability_lifecycle_registry.py`](../../../../ci/check_capability_lifecycle_registry.py)

## Acceptance Coverage

The lane exercises these states:

- claimed alpha surfaces have lifecycle rows;
- non-stable dependencies render dependency markers;
- marketed alpha rows cannot remain lifecycle-unknown;
- policy-disabled claims remain blocked until scope review;
- Help/About and diagnostics projection renders lifecycle state, owner,
  review or expiry, and dependency markers directly from the registry.

## Latest Local Validation

Refresh command:

```bash
python3 ci/check_capability_lifecycle_registry.py \
  --repo-root . \
  --report artifacts/milestones/m2/captures/capability_lifecycle_registry_validation_capture.json
```

Render the first consumer projection:

```bash
python3 ci/check_capability_lifecycle_registry.py --repo-root . --render-help-projection
```

The checked-in capture is
[`artifacts/milestones/m2/captures/capability_lifecycle_registry_validation_capture.json`](../captures/capability_lifecycle_registry_validation_capture.json).
