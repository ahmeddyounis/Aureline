# Certified reference workspaces, archetype pass matrices, and downgrade automation — proof packet

Reviewer-facing proof packet for the certification-evidence lane that hardens
every marketed Certified archetype with a current reference-workspace report,
binds each report to an archetype pass-matrix row, and automates downgrade when
freshness expires.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Artifact:
  [`/artifacts/release/harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation.json`](../harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation.json)
- Schema:
  [`/schemas/release/certified_reference_workspaces.schema.json`](../../../schemas/release/certified_reference_workspaces.schema.json)
- Companion doc:
  [`/docs/release/harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation.md`](../../../docs/release/harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation.md)
- Typed consumer:
  `aureline_release::harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation`

## What this packet proves

1. **Every marketed Certified archetype has a current reference-workspace report.**
   Each report binds an archetype to its certification-harness output, matrix
diff from the prior report, named owner, known-caveat summary, and validity
window. A missing or stale report is a blocking failure for the Certified badge
on every consuming surface.

2. **The archetype pass matrix ingests the report as a hard dependency.** Each
   matrix row names the archetype, claimed certification, pass criteria, owner
sign-off, and effective certification after narrowing. The matrix row may not
render Certified while its report is expired or missing.

3. **Downgrade automation narrows stale, missing, or manually edited reports**
   **before publication.** The `reference_workspace_report_stale`,
`reference_workspace_report_missing`, and
`reference_workspace_report_manually_edited` downgrade reasons fire blocking
rules that hold publication until the report is refreshed or the claim is
formally narrowed.

4. **The publication verdict is recomputed, not asserted.** The typed model and
   the CI gate both recompute the `hold`/`proceed` decision and the blocking
rule/row sets from the firing downgrade rules and fail on any drift.

## Current snapshot (as of 2026-06-02)

The checked-in artifact holds publication. Of four archetype pass-matrix rows,
two certify cleanly (Rust workspace self-host, TypeScript web-app — the latter
on an active waiver), and two are narrowed below certified:

- the **legacy remote-SSH** archetype narrowed because its reference-workspace
  report expired on 2026-05-20;
- the **extension-author** archetype narrowed because no reference-workspace
  report has been captured.

Both narrowed rows back claims still published Certified, so their reasons fire
blocking downgrade rules and hold the
`certified_reference_workspaces_publication` gate. Promotion clears once the
legacy remote-SSH report is refreshed and the extension-author report is
captured (or those public claims are formally narrowed).

## Accessibility of this lane

The artifact and its companion doc are text/JSON artifacts: the doc renders as
headed sections and plain Markdown tables, and the machine source carries the
same truth so Help/About, the release center, support exports, docs, and
shiproom dashboards ingest one record per archetype rather than restating status
text.

## How to re-verify

```
cargo test -p aureline-release
```

This runs the typed contract tests that bind the model to the checked-in
artifact, including the negative fixture cases.
