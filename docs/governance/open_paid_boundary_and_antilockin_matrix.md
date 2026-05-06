# Open-vs-paid boundary, anti-lock-in rules, and publication-control matrix

This document turns Aureline’s open-core intent into **inspectable,
reviewable rules** so product packaging, docs, and hosted-service
expansion cannot silently erode:

- local-first usefulness,
- open-core credibility,
- self-host / mirror viability, and
- user and organization exit paths.

It complements (and does not replace) the product boundary manifest,
deployment-profile truth, the residual-dependency ledger, and the
data-portability/customer-exit matrix.

## Companion artifacts

- [`/artifacts/governance/open_paid_boundary_rows.yaml`](../../artifacts/governance/open_paid_boundary_rows.yaml)
  — machine-readable register. Every row conforms to the row schema
  below and includes explicit “prohibited hidden prerequisite” rules.
- [`/schemas/governance/open_paid_boundary_row.schema.json`](../../schemas/governance/open_paid_boundary_row.schema.json)
  — boundary schema for one `open_paid_boundary_row`.
- [`/fixtures/governance/publication_control_examples/`](../../fixtures/governance/publication_control_examples/)
  — worked examples for repository/publication controls (public source
  vs public spec, stable-surface schema/doc obligations, sample
  availability posture, issue-routing posture).
- [`/schemas/governance/publication_control_example.schema.json`](../../schemas/governance/publication_control_example.schema.json)
  — boundary schema for one publication-control worked example.

## Adjacent contracts this matrix composes over

- [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
  and [`/schemas/product/boundary_manifest.schema.json`](../../schemas/product/boundary_manifest.schema.json)
  — capability classification as `local_core`, `self_host_friendly`, or
  `managed_convenience`. This matrix adds “open vs paid” and
  publication-control constraints **without** making pricing decisions.
- [`/docs/governance/deployment_profile_truth.md`](./deployment_profile_truth.md) and
  [`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml)
  — per-profile dependency postures and explicit absence-narrowing
  semantics. This matrix reuses the same dependency-class vocabulary and
  forbids “hidden required service” drift.
- [`/docs/governance/data_portability_and_exit_matrix.md`](./data_portability_and_exit_matrix.md) and
  [`/artifacts/governance/portability_artifact_matrix.yaml`](../../artifacts/governance/portability_artifact_matrix.yaml)
  — export/offboarding/delete/withdrawal truth per artifact domain.
  This matrix requires rows to cite portability rows instead of
  inventing parallel “export story” language.
- [`/docs/governance/drift_blocking_rules.md`](./drift_blocking_rules.md) and
  [`/docs/governance/claim_manifest_contract.md`](./claim_manifest_contract.md)
  — “docs truth” fail-closed posture for protected publication surfaces.
- [`/docs/governance/public_interface_stability_matrix.md`](./public_interface_stability_matrix.md)
  — stability labels and the obligation to publish schemas and version
  advertisement for stable-labeled surfaces.
- [`/docs/governance/provenance_and_compliance_baseline.md`](./provenance_and_compliance_baseline.md)
  — DCO / SPDX / import registers / SBOM/notice baseline this matrix
  treats as non-optional for open-core credibility.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — canonical issue/RFC routing matrix this matrix requires surfaces to
  reference instead of inventing “contact support” as the only exit.

## Normative sources

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix AQ
  (publication/license lanes; repository/publication controls) and its
  anti-lock-in posture.
- `.t2/docs/Aureline_Technical_Design_Document.md` §4.6 (“Portability and
  open exit”).
- `.t2/docs/Aureline_PRD.md` deployment profiles and local-first rules
  (air-gapped / sovereign posture, mirror/offline paths, and export
  obligations).

If this document conflicts with those sources, those sources win and
this document plus the companion YAML and schemas update in the same
change.

## Why this exists

Without an explicit open-vs-paid boundary contract, “local-first” and
“open” drift into marketing language:

- managed or paid value quietly becomes a hidden prerequisite (sign-in,
  hosted metadata, vendor-only console controls),
- stable-labeled contracts stop being published publicly (schemas, WIT
  worlds, CLI JSON, export packet families),
- offboarding degrades into “contact support” with no enforceable export
  floor, and
- docs/help copy widens beyond what the canonical owner artifacts
  actually promise.

This matrix exists to foreclose those failure modes early.

## Row model (what every row must answer)

Each `open_paid_boundary_row` answers, for one capability family or
asset class:

1. **Open/local floor** — what must remain usable with no sign-in and no
   network (or what must remain usable when optional services are
   absent).
2. **Optional paid convenience** — allowed value-adds that must remain
   optional (dashboards, hosted aggregation, managed scale, curated
   convenience), explicitly listed so they cannot become accidental
   prerequisites.
3. **Managed-only controls** — controls that may exist only in managed
   form, but must not be required to operate the open/local floor.
4. **Enterprise/self-host lane** — what is required for “self-host” or
   “sovereign” claims to be truthful (public client contracts, mirror or
   offline-bundle path, parity of control surfaces, exit paths).
5. **Prohibited hidden prerequisites** — explicit “MUST NOT” rules that
   reviewers can enforce.
6. **Anti-lock-in rules** — export/offboarding floor, mirror/manual
   import, control-surface parity (no vendor-console-only), docs truth,
   license/notice availability, and absence fallback behavior.
7. **Publication controls** — minimum code/spec visibility, sample
   availability posture, issue-routing posture, and stable-surface
   schema/docs obligations.

Rows are intentionally written so a “paid advantage” can be described
without making local/core workflows dependent on a hidden service.

## Change discipline

Adding or changing a row requires all of the following in the same
change:

1. Add or update the row in
   [`open_paid_boundary_rows.yaml`](../../artifacts/governance/open_paid_boundary_rows.yaml).
2. If you add a new vocabulary value (family class, license lane class,
   publication-control enum), update
   [`open_paid_boundary_row.schema.json`](../../schemas/governance/open_paid_boundary_row.schema.json)
   and bump `open_paid_boundary_row_schema_version`.
3. Add or update at least one worked example under
   [`/fixtures/governance/publication_control_examples/`](../../fixtures/governance/publication_control_examples/)
   when the change affects publication rules (public schema/docs, sample
   availability, issue routing, or repo visibility).
4. When the change widens or narrows public claims, ensure the claim
   routes through the claim-manifest family and remains subject to the
   drift-blocking rules.

