# Standards / interchange matrix

This document is the human-readable companion to the canonical
machine-readable register in
[`/artifacts/governance/standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml).
It freezes where Aureline reuses industry standards verbatim, where
it wraps them with a custom contract that still mirrors the
standard, and where it intentionally owns a format with no claim of
standard conformance.

Its purpose is narrow: one reviewer must be able to see, in a single
page, **which Aureline artefacts should be standard-shaped, which
are custom-but-mirrorable, and who owns each choice**. Security,
release, ecosystem, and docs work cite the same matrix when
selecting a format, so format decisions stop being tribal knowledge
spread across ADRs, PRDs, and chat threads.

Companion artifacts:

- [`/artifacts/governance/standards_matrix.yaml`](../../artifacts/governance/standards_matrix.yaml)
  — canonical machine-readable register.
- [`/docs/governance/standards_adoption_evidence_gate.md`](../governance/standards_adoption_evidence_gate.md)
  — standards-adoption evidence gate and conformance minimums for standards-based
  claims.
- [`/artifacts/governance/standards_deviation_ledger.yaml`](../../artifacts/governance/standards_deviation_ledger.yaml)
  — deviation burden ledger (owner, reason, migration impact, re-adoption plan).
- [`/fixtures/governance/standards_evidence_cases/`](../../fixtures/governance/standards_evidence_cases/)
  — worked “minimum evidence bundle” fixtures reviewers can cite.
- [`/docs/governance/deviation_adr_template.md`](../governance/deviation_adr_template.md)
  — narrative template used whenever Aureline narrows, extends,
  bridges, or temporarily diverges from a preferred standard.
- [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  — every deviation appears as a `D-NNNN` row here and points back
  at the matrix row it modifies.
- [`/artifacts/governance/build_vs_buy_register.yaml`](../../artifacts/governance/build_vs_buy_register.yaml)
  — sibling register for subsystem-level build-vs-buy posture.
- [`/artifacts/governance/dependency_register.yaml`](../../artifacts/governance/dependency_register.yaml)
  — upstream implementations that carry a standard into the
  workspace.
- [`/docs/governance/provenance_and_compliance_baseline.md`](../governance/provenance_and_compliance_baseline.md)
  — governing baseline for DCO, REUSE / SPDX, SBOM, and provenance
  hygiene this matrix composes with.

## 1. When to use this register

Use the matrix **before** selecting any on-disk, on-wire, or
publication format on a protected lane. The register applies
whenever a change would:

- add, remove, or rename an externally consumable artefact
  (telemetry payload, SBOM, SARIF report, flag-evaluation API,
  OpenAPI document, WIT world, saved manifest, CLI JSON record);
- bind an Aureline contract to an open standard by claim, banner,
  or release note;
- narrow, extend, bridge, or temporarily diverge from a preferred
  standard.

The gate is simple: **no major open-format or standard-compatibility
claim may survive the current milestone without either a preferred-
standard row in the matrix or a declared deviation path in the
decision register.** A PR that would publish such a claim without
those linkages is not ready to merge; open the matrix row (and the
deviation ADR, if one applies) in the same change.

## 2. How to use the matrix

### 2.1 Scoring a new format admission

1. Identify the domain from `domain_classes` in
   `standards_matrix.yaml` (or extend the enum through a schema /
   matrix change if the domain is genuinely new — do not coerce an
   unrelated domain).
2. Find or add the row for the preferred standard. Record:
   - `preferred_standard.family`, `preferred_standard.surfaces`,
     and `preferred_standard.version_range`;
   - `import_expectation` and `export_expectation` chosen from the
     allowed enums;
   - `support_class` that plainly describes how Aureline relates to
     the standard today;
   - a named surface owner (a DRI and the owning lane from
     [`dri_map.md`](../governance/dri_map.md));
   - the minimum evidence path(s) that will prove the posture;
   - `compatibility_window_class` plus a one-sentence note on how
     the pinned version moves.
3. Cross-link related registers where the same selection already
   has a home: `build_vs_buy_refs`, `dependency_ids`, and
   `decision_ids` for open / closed decision rows.
4. If the admission implies any narrowing, extension, bridge, or
   temporary divergence from the preferred standard, open a
   deviation ADR (see §3) and record its decision id in
   `related_registers.decision_ids`.

### 2.2 Reading a row

A matrix row answers six questions in order:

1. **Domain** — what kind of artefact / surface this governs.
2. **Preferred standard** — the family, named surfaces, and
   version range Aureline treats as canonical for that domain.
3. **Import / export expectation** — what Aureline will consume
   and what it will emit.
4. **Support class** — the plain label for how Aureline relates to
   the standard today (verbatim, export-only, custom-but-mirrorable,
   deferred, declined-with-rationale).
5. **Deviation policy and notes** — what counts as a deviation, and
   the rules that apply when one is proposed.
6. **Evidence and ownership** — who owns the posture, and what
   artefacts prove the claim.

If any of the six is missing, the row is not yet usable by
reviewers and the PR that adds it is not ready to merge.

### 2.3 Support-class legend

| Class                              | Meaning |
|------------------------------------|---|
| `standard_shaped_import_and_export` | Aureline consumes and emits the standard verbatim, within the pinned version range. |
| `standard_shaped_export_only`      | Aureline emits the standard but does not consume it. Upgrading requires a matrix change. |
| `standard_shaped_import_only`      | Aureline consumes the standard but does not emit it. Upgrading requires a matrix change. |
| `custom_but_mirrorable`            | Aureline owns a custom contract today; a documented bridge can project it onto the standard when needed. |
| `custom_with_bridge_planned`       | Aureline owns a custom contract today; a bridge lane is reserved in a named companion artifact. |
| `standard_deferred_placeholder`    | Seat reserved; the posture is a stub / placeholder for this milestone and cannot be cited as a live claim. |
| `standard_declined_with_rationale` | Aureline has decided not to adopt the standard at this milestone; the row records the audit trail instead of deleting itself. |

### 2.4 Evidence legend

Every row whose `support_class` is not a placeholder MUST list at
least one evidence path:

- `upstream_validator_harness` — a conformance lint or validator
  invocation against Aureline-produced or Aureline-consumed
  payloads.
- `example_export_fixture` — a committed, worked example of an
  Aureline emission in the standard's shape.
- `import_round_trip_fixture` — a parse-then-emit-then-compare
  fixture proving Aureline can consume and re-emit the standard
  without loss.
- `bridge_disclaimer_doc` — a narrative document that describes
  the bridge between Aureline's custom contract and the standard.
- `compatibility_note` — a release-note or docs entry naming the
  pinned version and compatibility window.
- `mirror_manifest_entry` — a row in
  `third_party_import_register.yaml` for mirrored bytes.
- `conformance_report_artifact` — a report checked into release
  evidence that vouches for conformance on a given release train.

## 3. Deviation workflow

A deviation is any posture that:

- narrows the preferred standard (emit only a subset of keywords,
  refuse a feature the standard permits);
- extends the preferred standard (add custom properties, custom
  keywords, private protocol extensions);
- bridges the preferred standard with a custom contract instead of
  adopting it directly;
- temporarily diverges from the preferred standard (use an older
  version, a non-preferred alternative, or a placeholder) with a
  named migration target.

Deviations are **not** silent implementation notes. A deviation
that touches a matrix row MUST be tracked as follows:

1. **Open a decision row** in
   `artifacts/governance/decision_index.yaml` with a fresh
   `D-NNNN` id, `commitment_class`, forum, freeze date, and
   `default_if_unresolved` posture, per
   [`decision_workflow.md`](../governance/decision_workflow.md).
2. **Author the deviation ADR** from
   [`/docs/governance/deviation_adr_template.md`](../governance/deviation_adr_template.md).
   The ADR MUST name:
   - the matrix row being modified;
   - the deviation class
     (`narrow_with_adr`, `extend_with_adr`, `bridge_with_adr`, or
     `temporarily_diverge_with_adr`);
   - the scope, the rationale, and the rollback or re-adoption
     plan;
   - the evidence paths that will prove the deviation is honoured
     (validator, round-trip, bridge note, compatibility note, or
     conformance report).
3. **Update the matrix row** in
   `standards_matrix.yaml` to:
   - add the decision id to `related_registers.decision_ids`;
   - record the deviation in `deviation_notes`;
   - adjust `support_class`, `import_expectation`,
     `export_expectation`, and `evidence_paths` as needed.
4. **Land everything in a single change.** The matrix row, the
   decision row, and the ADR MUST land together; partial landings
   break the cross-reference invariant.

Deviation policy classes the matrix admits:

- `no_deviation_permitted` — the standard is non-negotiable at
  this milestone (e.g. REUSE / SPDX per-file hygiene on new
  source-bearing files).
- `narrow_with_adr` — narrowing requires a deviation ADR.
- `extend_with_adr` — extension requires a deviation ADR.
- `bridge_with_adr` — a bridge adapter can be admitted without
  adopting the standard directly, via a bridge-classed ADR.
- `temporarily_diverge_with_adr` — a bounded divergence with a
  named migration target.
- `no_standard_currently_adopted` — the row documents the audit
  trail for a domain where no standard is in force.
- `not_yet_committed_pending_standard_maturity` — the standard is
  a candidate, but the upstream specification (or its ecosystem)
  is not mature enough for Aureline to pin.

## 4. Domain posture summary

The table below is a navigational summary only. The authoritative
posture per row lives in `standards_matrix.yaml`; if the two
disagree, the YAML wins for tooling and this table MUST be updated
in the same change.

| Row id | Domain | Support class | Import | Export | Deviation policy |
|---|---|---|---|---|---|
| `standard.opentelemetry` | Observability / telemetry | custom-with-bridge planned | none planned | placeholder stub | extend-with-ADR |
| `standard.sarif` | Static-analysis findings | standard-shaped import + export | supported | supported | narrow-with-ADR |
| `standard.spdx` | SBOM + per-file license identifiers | standard-shaped export only | supported | required | narrow-with-ADR |
| `standard.cyclonedx` | SBOM alternate serialisation | standard, deferred placeholder | best-effort | deferred | not-yet-committed |
| `standard.reuse` | Per-file license hygiene | standard-shaped import + export | required | required | no deviation permitted |
| `standard.commonmark` | Docs authoring / rendering | standard-shaped import + export | required | required | extend-with-ADR |
| `standard.oidc` | Human authentication | standard-shaped import only | required | none planned | extend-with-ADR |
| `standard.scim` | Identity provisioning | standard-shaped import only | required | none planned | extend-with-ADR |
| `standard.oci_distribution` | Artefact distribution and signing | standard, deferred placeholder | best-effort | deferred | not-yet-committed |
| `standard.semver` | Public-surface versioning | standard-shaped import + export | required | required | narrow-with-ADR |
| `standard.openfeature` | Feature-flag evaluation | custom-but-mirrorable | none planned | none planned | bridge-with-ADR |
| `standard.opa_rego` | Policy bundles and evaluation | custom-but-mirrorable | none planned | none planned | bridge-with-ADR |
| `standard.json_schema_2020_12` | Schema validation | standard-shaped import + export | required | required | narrow-with-ADR |
| `standard.openapi_3_2` | API surface definition | standard, deferred placeholder | supported | deferred | not-yet-committed |
| `standard.wasm_component_model_wit` | Extension ABI / component model | standard-shaped import + export | required | required | extend-with-ADR |

## 5. Domain register

### 5.1 OpenTelemetry — `standard.opentelemetry`

Observability and telemetry export. Aureline governs telemetry,
diagnostics, and support-bundle payloads through the schema
registry and consent ledger first; an OTLP export lane is reserved
in the CLI surface contract but is not a live emission today. Any
widening of the OTLP surface past what the schema registry permits
requires a deviation ADR.

- Owner: `@ahmeddyounis` (telemetry foundation).
- Evidence today: bridge disclaimer and compatibility note
  (`docs/automation/cli_surface_contract.md`,
  `docs/governance/telemetry_and_support_schema_registry.md`).
- Deviation class: `extend_with_adr`.

### 5.2 SARIF — `standard.sarif`

Static-analysis findings. Aureline emits and consumes SARIF 2.1.0
so that hosted-review parity and CI tooling stay interoperable.
Aureline-specific rule ids and taxonomies are allowed only through
the SARIF propertyBag mechanism; narrowing what Aureline produces
(e.g. redacting a field, omitting a section) requires a deviation
ADR.

- Owner: `@ahmeddyounis` (QA / verification).
- Evidence paths: upstream validator, example export fixture,
  import round-trip fixture.
- Deviation class: `narrow_with_adr`.

### 5.3 SPDX — `standard.spdx`

Per-file license identifiers (mandatory) and SPDX SBOM documents
(required once release-grade emission lands). The placeholder CI
SBOM script does not claim SPDX conformance; the replacement lane
at `tools/governance/spdx_sbom.sh` will. Any file using a non-
default SPDX identifier must be justified by an ADR.

- Owner: `@ahmeddyounis` (release evidence).
- Evidence paths: compatibility note, conformance report artifact.
- Deviation class: `narrow_with_adr`.

### 5.4 CycloneDX — `standard.cyclonedx`

Alternate SBOM serialisation. Reserved as a sibling to SPDX and
lands only when a security-tooling consumer requires it. Seat is a
placeholder today; turning it into a live row requires an adoption
or deviation ADR.

- Owner: `@ahmeddyounis` (release evidence).
- Evidence today: compatibility note.
- Deviation class: `not_yet_committed_pending_standard_maturity`.

### 5.5 REUSE — `standard.reuse`

Per-file license hygiene. Non-negotiable on newly added or
modified source-bearing files; the workspace default is
`Apache-2.0`. Files outside that default must sit under a crate
whose declared license matches, and the discrepancy must be
justified by an ADR. No further deviation is expected.

- Owner: `@ahmeddyounis` (docs / public truth).
- Evidence paths: upstream validator harness, compatibility note.
- Deviation class: `no_deviation_permitted`.

### 5.6 CommonMark — `standard.commonmark`

Authoring and rendering baseline for `/docs/**`. Aureline tolerates
GitHub-flavoured additions where CommonMark does not cover the
need (tables, task lists, strikethrough) but any rendering path
that breaks CommonMark semantics requires a deviation ADR and a
bridge note in the docs-help contract.

- Owner: `@ahmeddyounis` (docs / public truth).
- Evidence paths: example export fixture, compatibility note.
- Deviation class: `extend_with_adr`.

### 5.7 OpenID Connect — `standard.oidc`

Human authentication on the self-hosted and managed identity modes.
The OIDC / SCIM / policy-bundle contract freeze travels with
ADR-0001 (identity modes); any protocol-profile widening or
claim-mapping extension requires a deviation ADR that cites
ADR-0001.

- Owner: `@ahmeddyounis` (identity / auth).
- Evidence paths: bridge disclaimer doc, compatibility note.
- Deviation class: `extend_with_adr`.

### 5.8 SCIM — `standard.scim`

Identity provisioning on self-hosted and managed modes. Schema
extensions beyond RFC 7643 require a deviation ADR and a crosswalk
to the offline entitlement / policy seed.

- Owner: `@ahmeddyounis` (identity / auth).
- Evidence paths: bridge disclaimer doc, compatibility note.
- Deviation class: `extend_with_adr`.

### 5.9 OCI distribution — `standard.oci_distribution`

Candidate transport for extension registry, docs pack, and
update-client artefact distribution. Seat is reserved; committing
Aureline to OCI distribution requires a deviation ADR that binds
the registry contract to this matrix row.

- Owner: `@ahmeddyounis` (release evidence).
- Evidence today: compatibility note.
- Deviation class: `not_yet_committed_pending_standard_maturity`.

### 5.10 SemVer — `standard.semver`

Public-surface versioning for schemas, CLI JSON, RPC envelopes,
manifests, and saved artefacts. SemVer semantics apply once a
surface is declared stable. Narrowing SemVer semantics for a
specific surface (for example, additive-only schema evolution on a
saved artefact) requires a deviation ADR.

- Owner: `@ahmeddyounis` (release evidence).
- Evidence paths: compatibility note, example export fixture.
- Deviation class: `narrow_with_adr`.

### 5.11 OpenFeature — `standard.openfeature`

Feature-flag evaluation API. Aureline uses the internal feature-
flag policy rather than adopting OpenFeature. An OpenFeature-shaped
adapter MAY land via a bridge ADR once an external consumer needs
it. Direct adoption of OpenFeature as the primary flag contract
requires a deviation ADR.

- Owner: `@ahmeddyounis` (governance / policy).
- Evidence paths: bridge disclaimer doc, compatibility note.
- Deviation class: `bridge_with_adr`.

### 5.12 OPA / Rego — `standard.opa_rego`

Policy bundles and evaluation. Aureline's policy-bundle contract
(offline entitlement seed, admin-policy narrowing ceiling) is
custom today; a Rego-shaped evaluation adapter may land via a
bridge ADR. Adopting Rego as the in-product policy language
requires a deviation ADR.

- Owner: `@ahmeddyounis` (governance / policy).
- Evidence paths: bridge disclaimer doc, compatibility note.
- Deviation class: `bridge_with_adr`.

### 5.13 JSON Schema Draft 2020-12 — `standard.json_schema_2020_12`

Schema validation for everything under `/schemas/**`. Schemas MAY
narrow (forbid `additionalProperties`, require a subset) but MUST
NOT invent keywords outside the JSON Schema vocabulary without a
deviation ADR.

- Owner: `@ahmeddyounis` (governance / policy).
- Evidence paths: upstream validator harness, example export
  fixture.
- Deviation class: `narrow_with_adr`.

### 5.14 OpenAPI 3.2+ — `standard.openapi_3_2`

API surface definition for any HTTP API Aureline exposes. Aureline
does not ship a hosted HTTP API at this milestone; the seat is
reserved. Admitting OpenAPI 3.1 as an interim fallback requires a
deviation ADR naming the migration target.

- Owner: `@ahmeddyounis` (automation / CLI).
- Evidence today: compatibility note.
- Deviation class: `not_yet_committed_pending_standard_maturity`.

### 5.15 WebAssembly Component Model / WIT — `standard.wasm_component_model_wit`

Extension ABI. The ABI is frozen in ADR-0019 against
`wit/aureline/*.wit` and the capability-world registry. Extending
the ABI, adding a capability world, or changing the Component
Model ABI range requires a deviation ADR that cites ADR-0019 and
this matrix row.

- Owner: `@ahmeddyounis` (extensions).
- Evidence paths: upstream validator harness, example export
  fixture, compatibility note.
- Deviation class: `extend_with_adr`.

## 6. Keeping the matrix current

- Adding a new row is a matrix change that MUST land in the same
  PR as any evidence it references. Rows without evidence are not
  usable and should not be cited from downstream artefacts.
- Retiring a row is **not** a delete. Set `support_class` to
  `standard_declined_with_rationale` and record the reason. The
  audit trail that "this posture once applied" survives.
- Schema changes to `standards_matrix.yaml` (adding fields, new
  enum values, new evidence paths) follow the same discipline as
  the decision-register schema — the change lands with the rows
  that need it, and reviewers verify that existing rows either
  adopt the new field or carry a documented reason for not doing
  so.
- The bar for any public claim of standard compatibility is this
  register. If a release note, claim manifest, or marketing page
  would reach beyond what the matrix supports, the fix is to
  update the matrix (and any deviation ADR) before the claim
  publishes — not to let the claim ride on tribal knowledge.
