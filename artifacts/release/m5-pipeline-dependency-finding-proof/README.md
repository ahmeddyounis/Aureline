# M5 pipeline / dependency / finding component proof

This proof packet binds the frozen matrix at
`artifacts/design/m5-pipeline-dependency-finding-component-matrix.md` to the UI
schemas and first-consumer fixtures for reusable pipeline run rows, annotation
rows, dependency rows, manifest diff cards, and security finding cards. Security
finding cards prove finding class, affected scope, fix availability, controlled
suppression labels, remediation path, local validation, docs/help path, and
audit action parity instead of relying on a generic warning model.

Files:

- `proof_packet.json` records schema, fixture, consumer, degraded-state, copy/export, freshness, parity-check, and auto-narrowing coverage, including companion-safe manifest-diff projection.
- `support_export.json` is the support-safe projection of the same component baseline and includes the per-consumer pass/narrow state.
- `matrix.csv` is the release-review checklist row per component family.

Claim-bearing review, package, project-health, and companion surfaces must pass
the freshness SLO plus required-field, controlled-label, action-vocabulary,
degraded-state, suppression-visibility, and copy/export parity checks. Any failed
check narrows that surface claim.

Validate schemas and fixtures with the command in
`fixtures/ui/m5-pipeline-dependency-finding-components/README.md`.
