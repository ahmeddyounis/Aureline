# Beta release notes — draft

This file is the draft release-notes surface the M3 docs / public-truth
gate enforces back-links against. It is a checked-in artifact: it
must always quote the live claim manifest and compatibility report
ids so the gate can verify that release notes, docs, Help/About, and
support exports read one row truth instead of restating beta scope
in prose.

Refresh this file in the same change set as
`artifacts/release/m3/claim_manifest.json`,
`artifacts/compat/m3/compatibility_report.json`, and
`artifacts/ci/m3_docs_truth_source_map.yaml`. Each row in the draft
quotes a `row_id` from the claim manifest and the compatibility row
ids the row depends on, so reviewers can resolve every line back to
machine-readable truth.

## Source-of-truth references

- Claim manifest id: `claim_manifest:m3.beta`
- Claim manifest as_of: `2026-05-15`
- Compatibility report id: `compat_report:m3.beta`
- Compatibility report as_of: `2026-05-15`
- Reviewer landing page:
  [`docs/milestones/m3/beta_admission_matrix.md`](../../../docs/milestones/m3/beta_admission_matrix.md)
- Claim-manifest contract:
  [`docs/governance/claim_manifest_contract.md`](../../../docs/governance/claim_manifest_contract.md)
- Compatibility-report template:
  [`docs/release/compatibility_report_template.md`](../../../docs/release/compatibility_report_template.md)
- Release-evidence packet template:
  [`docs/release/release_evidence_packet_template.md`](../../../docs/release/release_evidence_packet_template.md)
- Help/About truth source:
  [`docs/help/help_about_truth_source.md`](../../../docs/help/help_about_truth_source.md)
- About / provenance contract:
  [`docs/about/about_provenance_and_boundary_contract.md`](../../../docs/about/about_provenance_and_boundary_contract.md)

## What is published

This beta names a bounded surface, not a product release. Each row
below cites the canonical claim-manifest row by `row_id` and quotes
the compatibility rows the claim depends on; reviewers MUST resolve
every line back to the named row before sign-off.

### Docs freshness truth

- Claim-manifest row:
  `m3_claim_row:canonical.docs.freshness_truth`
- Compat row dependency:
  `compat_row:release_identity.exact_build_propagation`
- Public-truth statement: docs, Help/About, support exports, and
  release packets all render the same freshness vocabulary
  (`warm_cached` floor or stricter); stale rows narrow automatically
  to a typed limited posture instead of silently rendering certified
  wording.

### Compatibility / version-skew truth

- Claim-manifest row:
  `m3_claim_row:canonical.compat.version_skew_truth`
- Compat row dependency:
  `compat_row:desktop.platform_conformance_profiles`
- Public-truth statement: every claimed beta surface resolves through
  a named skew window; reports outside the window narrow the claim or
  refuse promotion rather than publish certified prose without proof.

### Certified-archetype compatibility publication

- Claim-manifest row:
  `m3_claim_row:beta_surface.compatibility_publication`
- Compat row dependency:
  `compat_row:deployment_profiles.boundary_manifest_truth`
- Public-truth statement: archetype compatibility reports publish
  through the same generated record shape the M3 compatibility report
  uses; the report state, support class, and evidence date are
  machine-derived, never hand-edited.

### Launch wedge — TypeScript / web archetype

- Claim-manifest row:
  `m3_claim_row:beta_archetype.ts_web_app_or_service`
- Compat row dependency:
  `compat_row:release_identity.exact_build_propagation`
- Public-truth statement: the TypeScript/web service archetype is the
  first beta-published archetype; certified wording remains gated on
  the named scorecard's effective support class and evidence date.

## How the gate enforces this draft

The freshness gate at `tools/ci/m3/docs_freshness_gate.py` fails
closed when this draft:

- omits the claim-manifest `manifest_id`;
- omits the claim-manifest `as_of`;
- omits the compatibility-report `report_id`;
- omits any row id listed under
  `enforced_rows[].manifest_row_id` in the source map; or
- omits any row id listed under
  `enforced_compat_rows[].compat_row_id` in the source map.

The reproducer drill
`m3_docs_truth.release_notes_manifest_backlink_missing` strips the
manifest id from this draft and re-runs the gate to verify the gate
fails with `release_notes.manifest_backlink_missing`.
