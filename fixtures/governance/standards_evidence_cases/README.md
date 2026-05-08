# Standards adoption evidence fixtures

Worked “minimum evidence bundle” fixtures for the standards-adoption evidence
gate defined in:

- `docs/governance/standards_adoption_evidence_gate.md`

These fixtures exist so reviewers (and future CI gates) can answer one question
without hunting through architecture prose:

> What concrete proof artifacts must exist before a surface may claim a given
> standard as “adopted” or “standards-based”?

The authoritative standards posture lives in:

- `artifacts/governance/standards_matrix.yaml`

The deviation cost (when the preferred standard is narrowed/extended/wrapped or
only bridged/deferred) lives in:

- `artifacts/governance/standards_deviation_ledger.yaml`

## Fixture shape (conventions)

Each fixture is a small YAML record with:

- `standard_row_ref` — the `standards_matrix.yaml` row this evidence bundle is
  for.
- `claim_vectors[]` — the claim scopes the bundle covers (bridge-only, export,
  import+export, hygiene enforcement).
- `minimum_evidence_bundle[]` per claim vector — required evidence-path classes
  and the concrete artifact refs a reviewer expects to exist.

Artifact refs may include:

- repo paths (committed evidence, docs, seeds, schemas),
- `target/...` paths as *generated outputs* that are expected once an
  implementation exists (these are not committed).

## Index

| Standard | Fixture |
| --- | --- |
| OpenTelemetry / OTLP | `opentelemetry_otlp_minimum.yaml` |
| SARIF | `sarif_minimum.yaml` |
| SPDX | `spdx_sbom_minimum.yaml` |
| CycloneDX | `cyclonedx_minimum.yaml` |
| REUSE / per-file SPDX hygiene | `reuse_file_hygiene_minimum.yaml` |
| CommonMark | `commonmark_minimum.yaml` |
| OIDC | `oidc_minimum.yaml` |
| SCIM | `scim_minimum.yaml` |
| OCI distribution | `oci_distribution_minimum.yaml` |
| SemVer | `semver_minimum.yaml` |
| OpenFeature bridge | `openfeature_bridge_minimum.yaml` |
| OPA/Rego bridge | `opa_rego_bridge_minimum.yaml` |
| JSON Schema Draft 2020-12 | `json_schema_2020_12_minimum.yaml` |
| OpenAPI | `openapi_minimum.yaml` |
| WIT / Component Model | `wit_component_model_minimum.yaml` |

