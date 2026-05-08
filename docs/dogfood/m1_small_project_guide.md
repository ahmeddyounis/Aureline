# Small-project dogfood guide

This page is the reviewer-facing entry point for the daily small-project
dogfood lane. The canonical truth source is
`artifacts/milestones/m1/dogfood_matrix.yaml`; downstream packets, dashboards,
and checklists should reference that file instead of restating fixture lists.

## Canonical artifacts

- Human entry (this file): `docs/dogfood/m1_small_project_guide.md`
- Dogfood matrix (canonical): `artifacts/milestones/m1/dogfood_matrix.yaml`
- Proof index (first consumer): `artifacts/milestones/m1/dogfood_matrix_index.yaml`
- Validator: `ci/check_m1_dogfood_matrix.py`
- Fixture repos: `fixtures/repos/m1/`

## How to validate

Run:

`python3 ci/check_m1_dogfood_matrix.py --repo-root . --report target/m1-dogfood-matrix/report.json`

## Protected walk

1. Pick one protected fixture repo under `fixtures/repos/m1/`.
2. Run the dogfood recipe for that row in `artifacts/milestones/m1/dogfood_matrix.yaml`.
3. File a dogfood issue using `docs/governance/dogfood_issue_taxonomy.md` when outcomes differ.

## Failure drill

To confirm the guardrail is live:

1. Temporarily rename one directory under `fixtures/repos/m1/` (for example
   `plain_text_notes` → `plain_text_notes__moved`).
2. Rerun the validator; it must fail with an actionable missing-ref error.
3. Undo the rename and rerun; it must pass.

