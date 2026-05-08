# Standards adoption evidence gate

This policy makes standards adoption an auditable operational commitment.
No surface may claim a format, protocol, or contract is “standards-based”
unless the claim cites (a) a named evidence bundle or (b) an explicit
deviation record that states the compatibility cost and re-adoption work.

Companions:

- [`/artifacts/governance/standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  — canonical register of preferred standards, posture, owners, and evidence path
  classes.
- [`/artifacts/governance/standards_deviation_ledger.yaml`](../../artifacts/governance/standards_deviation_ledger.yaml)
  — machine-readable ledger of deviations (narrow/extend/wrap/custom) with
  migration burden and re-adoption requirements.
- [`/fixtures/governance/standards_evidence_cases/`](../../fixtures/governance/standards_evidence_cases/)
  — worked “minimum evidence bundle” fixtures that name the concrete artifacts a
  reviewer (or a future CI gate) checks.
- [`/docs/governance/deviation_adr_template.md`](./deviation_adr_template.md)
  — required narrative fields for a deviation ADR when a standard is narrowed,
  extended, bridged, or temporarily diverged from.
- [`/docs/governance/evidence_freshness_policy.md`](./evidence_freshness_policy.md)
  — how proof artifacts age and when stale evidence must narrow claims.

## Scope

This gate applies to any claim that uses standards language (e.g. “SARIF
export”, “SPDX SBOM”, “OpenTelemetry OTLP”, “OpenAPI-described service”, “WIT
ABI”) on any discoverable surface:

- docs pages and generated docs,
- CLI `--json` output and exported packets/bundles,
- release notes, compatibility notes, and public-proof packets,
- registries (extension registry, schema registry, artifact registries).

It is explicitly in scope for the standards rows seeded in the matrix:
OpenTelemetry, SARIF, SPDX, CycloneDX, CommonMark, OIDC, SCIM, OpenFeature-style
flag evaluation, OPA/Rego-style policy bundles, JSON Schema Draft 2020-12,
OpenAPI, WIT/component model, OCI-compatible artifact transport, SemVer, and
REUSE/SPDX per-file licensing hygiene.

## Gate rule (fail closed)

If a PR introduces or widens a standards-based claim, reviewers MUST be able to
trace the claim to one of:

1. A standards-matrix row whose `evidence_refs` include a minimum-evidence bundle
   fixture under `fixtures/governance/standards_evidence_cases/`, or
2. A standards-deviation ledger row that declares why the standard is not met
   and what proof is admissible for the narrower/bridged posture.

A claim that cannot be traced to (1) or (2) is treated as not-yet-ratified and
MUST narrow (wording, badges, metadata) rather than survive as implied
compatibility.

## Evidence bundle vocabulary

Minimum evidence bundles reuse the evidence-path classes from
`standards_matrix.yaml` and bind them to concrete artifacts:

- `upstream_validator_harness` — a documented command/workflow that runs an
  upstream validator (or a validator-grade library) and records a pass/fail
  report.
- `example_export_fixture` — a committed example export payload that is
  standards-shaped, paired with the schema/validator used to check it.
- `import_round_trip_fixture` — a committed fixture that proves parse → emit
  round-trip for a representative sample, with a stated equality criterion.
- `bridge_disclaimer_doc` — a narrative disclosure page that explains fidelity,
  lossy/lossless mapping, and what is *not* claimed.
- `compatibility_note` — a pinned version note that states the adopted version
  range and the window in which it will be kept compatible.
- `conformance_report_artifact` — a release-evidence-grade report/packet that
  vouches for conformance for a given release train.

## Conformance minimums (by preferred standard)

The minimum bundle below is the smallest admissible proof set before a surface
may claim “standards-based” posture for the named family. The concrete artifacts
live in the matching fixture file(s).

- **OpenTelemetry (OTLP)** — `standard.opentelemetry`
  - Minimum: `bridge_disclaimer_doc` + `compatibility_note`.
  - Required before claiming *live* OTLP export: `example_export_fixture` +
    `upstream_validator_harness`.
- **SARIF** — `standard.sarif`
  - Minimum: `example_export_fixture` + `upstream_validator_harness`.
  - Required before claiming import+export parity: add `import_round_trip_fixture`.
- **SPDX** — `standard.spdx`
  - Minimum (per-file hygiene): `upstream_validator_harness` (REUSE/SPDX header
    lint) + `compatibility_note`.
  - Required before claiming SBOM conformance: `example_export_fixture` +
    `conformance_report_artifact`.
- **CycloneDX** — `standard.cyclonedx`
  - Minimum: `compatibility_note`.
  - Required before claiming export support: `example_export_fixture` +
    `upstream_validator_harness`.
- **CommonMark** — `standard.commonmark`
  - Minimum: `example_export_fixture` + `compatibility_note`.
  - Required before claiming renderer conformance: `upstream_validator_harness`
    against a pinned CommonMark test corpus.
- **OIDC** — `standard.oidc`
  - Minimum: `bridge_disclaimer_doc` + `compatibility_note` naming the pinned
    profile set.
  - Required before claiming enterprise-grade interoperability: an
    `import_round_trip_fixture` (auth flow + token validation transcript) plus a
    conformance/compat report packet in release evidence.
- **SCIM** — `standard.scim`
  - Minimum: `bridge_disclaimer_doc` + `compatibility_note` naming the pinned
    schema/profile set.
  - Required before claiming provisioning parity: `upstream_validator_harness`
    (schema/contract validation) + `import_round_trip_fixture` (provision →
    readback diff criteria).
- **OpenFeature-style flags** — `standard.openfeature`
  - Minimum: `bridge_disclaimer_doc` + `compatibility_note`.
  - Required before claiming OpenFeature API compatibility: `example_export_fixture`
    (API surface docs/IDL) + `upstream_validator_harness` (conformance tests or
    compile-time contract check).
- **OPA/Rego-style policy bundles** — `standard.opa_rego`
  - Minimum: `bridge_disclaimer_doc` + `compatibility_note`.
  - Required before claiming Rego/bundle compatibility: `example_export_fixture`
    + `upstream_validator_harness` (OPA tooling pass).
- **JSON Schema Draft 2020-12** — `standard.json_schema_2020_12`
  - Minimum: `upstream_validator_harness` (Draft 2020-12 validator) +
    `example_export_fixture` (schema + instance fixtures).
- **OpenAPI** — `standard.openapi_3_2`
  - Minimum: `compatibility_note`.
  - Required before claiming OpenAPI-described service: `example_export_fixture`
    (OpenAPI document) + `upstream_validator_harness` (OpenAPI validation).
- **WIT / Component Model** — `standard.wasm_component_model_wit`
  - Minimum: `upstream_validator_harness` (WIT lint/format) +
    `example_export_fixture` (a pinned WIT package/world snapshot) +
    `compatibility_note`.
- **OCI artifact transport** — `standard.oci_distribution`
  - Minimum: `compatibility_note`.
  - Required before claiming OCI push/pull: `example_export_fixture` +
    `import_round_trip_fixture` (push → pull → digest match) + a release-evidence
    conformance report.
- **SemVer** — `standard.semver`
  - Minimum: `compatibility_note` (policy + version-advertisement anchors) +
    `example_export_fixture` (a contract surface demonstrating the version rule).
- **REUSE / SPDX file licensing hygiene** — `standard.reuse`
  - Minimum: `upstream_validator_harness` (REUSE/SPDX lint) +
    `compatibility_note` (policy anchor).

## How to use this gate

When a change introduces or widens a standards claim:

1. Update (or add) the relevant row in `artifacts/governance/standards_matrix.yaml`.
2. Add/refresh a minimum-evidence bundle fixture under
   `fixtures/governance/standards_evidence_cases/` and cite it from the matrix
   row’s `evidence_refs`.
3. If the standard is not met verbatim, add a deviation row in
   `artifacts/governance/standards_deviation_ledger.yaml` and (when required by
   the matrix row) land a deviation ADR using
   `docs/governance/deviation_adr_template.md`.

Evidence should be treated as time-bounded when it reflects external tooling,
protocol interoperability, or ecosystem expectations. If an evidence bundle
would go stale under `docs/governance/evidence_freshness_policy.md`, surfaces
MUST narrow the claim rather than continue to present it as current.

