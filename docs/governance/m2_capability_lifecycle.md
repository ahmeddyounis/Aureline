# External Alpha Capability Lifecycle Registry

This page is the human entry point for the external alpha capability
lifecycle registry. The machine-readable source is
[`/artifacts/governance/capability_lifecycle_registry.yaml`](../../artifacts/governance/capability_lifecycle_registry.yaml);
surfaces that render alpha readiness must consume that registry rather
than copying status text.

## Canonical Sources

- Registry:
  [`/artifacts/governance/capability_lifecycle_registry.yaml`](../../artifacts/governance/capability_lifecycle_registry.yaml)
- Dependency-marker schema:
  [`/schemas/governance/dependency_marker.schema.json`](../../schemas/governance/dependency_marker.schema.json)
- Protected fixture manifest:
  [`/fixtures/governance/capability_lifecycle_registry_cases/manifest.yaml`](../../fixtures/governance/capability_lifecycle_registry_cases/manifest.yaml)
- First consumer and validator:
  [`/ci/check_capability_lifecycle_registry.py`](../../ci/check_capability_lifecycle_registry.py)

## Registry Contract

Every row carries:

- `declared_lifecycle_state` and `effective_lifecycle_state`;
- `owner`, `target_persona_or_workflow`, and `default_posture`;
- `migration_note`, `support_promise`, and `review_or_expiry_date`;
- `kill_switch_or_policy_disable_ref`;
- `source_scope_refs`, `scoreboard_row_refs`, and
  `dependency_marker_refs`;
- the surfaces that must render the row.

The controlled lifecycle vocabulary for this registry is:
`Labs`, `Preview`, `Beta`, `Stable`, `Deprecated`,
`DisabledByPolicy`, and `Retired`. The registry projects these values
to the existing schema vocabulary in
[`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
without adding local synonyms.

## Dependency Markers

Dependency markers make non-stable or policy-blocked dependencies
visible before a surface can imply readiness. The baseline covers:

- profile artifacts;
- workspace manifests;
- saved views;
- support exports;
- migration packets;
- launch-bundle claims;
- archetype claims.

Rows that depend on helper services or hosted integrations also carry
markers so Help/About, settings, diagnostics, marketplace, docs, and
support exports cannot render them as stable by omission.

## First Consumer

The validator is also the first Help/About and diagnostics projection
consumer. It renders export-safe JSON directly from the registry:

```bash
python3 ci/check_capability_lifecycle_registry.py --repo-root . --render-help-projection
```

Each projected row includes lifecycle state, owner, review or expiry
date, support promise, migration note, policy or kill-switch reference,
and dependency markers.

## Validation

Run the protected lane:

```bash
python3 ci/check_capability_lifecycle_registry.py --repo-root .
```

Refresh the checked-in validation capture:

```bash
python3 ci/check_capability_lifecycle_registry.py \
  --repo-root . \
  --report artifacts/milestones/m2/captures/capability_lifecycle_registry_validation_capture.json
```

The lane fails when:

- a claimed alpha wedge, workflow, deployment row, launch bundle,
  archetype row, or marketed scoreboard row is missing lifecycle
  coverage;
- a row or marker uses a lifecycle value outside the controlled
  vocabulary;
- a non-stable dependency marker is hidden from its parent row;
- a `DisabledByPolicy` row lacks a policy-disable marker and repair or
  scope-review path;
- the first consumer cannot render owner, expiry, lifecycle state, and
  dependency markers directly from the registry.
