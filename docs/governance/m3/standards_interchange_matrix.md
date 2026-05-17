# Beta standards and interchange matrix

This page is the beta publication view of Aureline's standards and
interchange posture. The canonical cross-milestone register remains
[`artifacts/governance/standards_matrix.yaml`](../../../artifacts/governance/standards_matrix.yaml);
this page narrows that register into the rows beta docs, Help/About,
support exports, release packets, and public-proof reviews may cite.

If this page and the machine-readable register disagree, the register
wins and this page must be updated in the same change set. If the
open-project beta packet disagrees with either source, the packet
validator fails closed:
[`artifacts/milestones/m3/open_project_beta_packet.md`](../../../artifacts/milestones/m3/open_project_beta_packet.md).

## Rules for beta claims

- Cite exact row ids from `standards_matrix.yaml`; do not invent a
  beta-only standard name.
- Use the existing support classes verbatim:
  `standard_shaped_import_and_export`,
  `standard_shaped_export_only`, `standard_shaped_import_only`,
  `custom_but_mirrorable`, `custom_with_bridge_planned`, and
  `standard_deferred_placeholder`.
- A deferred placeholder is not a live interoperability claim. It may
  be described only as reserved or planned.
- A bridge posture is not a conformance claim. It may be described only
  as custom, mirrorable, or bridge-planned until the evidence gate is
  lifted.
- Any widening of import/export support must update the standards
  register, deviation ledger when applicable, this beta page, and the
  open-project beta packet together.

## Beta publication matrix

| Row id | Standard / surface | Beta support posture | Import | Export | Public claim ceiling |
|---|---|---|---|---|---|
| `standard.opentelemetry` | OpenTelemetry OTLP traces, metrics, and logs | `custom_with_bridge_planned` | `none_planned` | `placeholder_stub_only` | OTLP export lane reserved; no live OTLP export claim. |
| `standard.sarif` | SARIF 2.1.0 static-analysis findings | `standard_shaped_import_and_export` | `supported` | `supported` | SARIF import/export may be claimed only with the validator and example bundle cited from the matrix row. |
| `standard.spdx` | SPDX license identifiers and release SBOM inventory | `standard_shaped_export_only` | `supported` | `required` | SPDX per-file identifiers are required; release-grade SBOM wording stays tied to the conformance packet and may not treat the stub as conformance. |
| `standard.cyclonedx` | CycloneDX alternate SBOM serialization | `standard_deferred_placeholder` | `best_effort` | `deferred_to_later_milestone` | Reserved only; no CycloneDX export-support claim. |
| `standard.reuse` | REUSE / SPDX per-file licensing hygiene | `standard_shaped_import_and_export` | `required` | `required` | New source-bearing files follow REUSE-shaped SPDX metadata. |
| `standard.commonmark` | CommonMark docs authoring and rendering baseline | `standard_shaped_import_and_export` | `required` | `required` | Docs rendering may claim CommonMark baseline with declared extensions; any semantic break needs a deviation ADR. |
| `standard.oidc` | OpenID Connect human authentication | `standard_shaped_import_only` | `required` | `none_planned` | Managed and self-hosted auth may claim OIDC consumption only; no OIDC export or hosted identity lock-in claim. |
| `standard.scim` | SCIM user and group provisioning | `standard_shaped_import_only` | `required` | `none_planned` | Managed and self-hosted provisioning may claim SCIM consumption only. |
| `standard.oci_distribution` | OCI-compatible artifact transport | `standard_deferred_placeholder` | `best_effort` | `deferred_to_later_milestone` | Candidate/reserved transport only; no OCI push/pull compatibility claim. |
| `standard.semver` | Semantic Versioning for public surfaces | `standard_shaped_import_and_export` | `required` | `required` | Public schemas, SDKs, and contract surfaces use SemVer once declared; pre-stable surfaces must say they are pre-release. |
| `standard.openfeature` | OpenFeature-shaped feature-flag evaluation | `custom_but_mirrorable` | `none_planned` | `none_planned` | Custom flag governance with a mirrorable bridge; no OpenFeature API-compatibility claim. |
| `standard.opa_rego` | OPA/Rego-style policy bundles and evaluation | `custom_but_mirrorable` | `none_planned` | `none_planned` | Custom policy bundle with a mirrorable bridge; no Rego language support claim. |
| `standard.json_schema_2020_12` | JSON Schema Draft 2020-12 | `standard_shaped_import_and_export` | `required` | `required` | Repo schemas under `/schemas/**` use Draft 2020-12 unless a deviation ADR narrows the row. |
| `standard.openapi_3_2` | OpenAPI service description | `standard_deferred_placeholder` | `supported` | `deferred_to_later_milestone` | Reserved for future HTTP/service APIs; no published OpenAPI document claim in beta. |
| `standard.wasm_component_model_wit` | WebAssembly Component Model and WIT extension ABI | `standard_shaped_import_and_export` | `required` | `required` | Extension ABI and host worlds may claim WIT/component-model shape within the pinned pre-1.0 range. |

## Evidence gates

Evidence minimums are defined by
[`docs/governance/standards_adoption_evidence_gate.md`](../standards_adoption_evidence_gate.md)
and worked fixtures under
[`fixtures/governance/standards_evidence_cases/`](../../../fixtures/governance/standards_evidence_cases/).
The open-project beta packet cites the exact evidence refs accepted for
each row.

Rows with `standard_deferred_placeholder`,
`custom_but_mirrorable`, or `custom_with_bridge_planned` remain narrow
even when their fixture exists. The fixture proves the current ceiling,
not the wider standard conformance claim.

## How to verify

Run the open-project beta packet validator:

```sh
python3 ci/check_m3_open_project_beta_packet.py --repo-root .
```

Use `--check` in CI to fail if the validation capture would drift:

```sh
python3 ci/check_m3_open_project_beta_packet.py --repo-root . --check
```
