# Known gaps ledger

This page is the reviewer-facing entry point for the known-gaps ledger, blocker
taxonomy, and exit-gate checklist. The canonical truth source is
`artifacts/milestones/m1/known_gaps_ledger.yaml`; downstream packets, dashboards,
and checklists should reference that file (and its linked taxonomy/checklist)
instead of restating rows in local notes.

## Canonical artifacts

- Human entry (this file): `docs/milestones/m1/known_gaps.md`
- Known-gaps ledger (canonical): `artifacts/milestones/m1/known_gaps_ledger.yaml`
- Blocker taxonomy (canonical): `artifacts/milestones/m1/blocker_taxonomy.yaml`
- Exit-gate checklist (canonical): `artifacts/milestones/m1/exit_gate_checklist.yaml`
- Proof index: `artifacts/milestones/m1/artifact_index.yaml`
- Validator: `ci/check_m1_known_gaps.py`

## How to validate

Run:

`python3 ci/check_m1_known_gaps.py --repo-root .`

Optional machine-readable report:

`python3 ci/check_m1_known_gaps.py --repo-root . --report target/m1-known-gaps/report.json`

## Update rules

1. Update `artifacts/milestones/m1/known_gaps_ledger.yaml` first.
2. Run the validator locally.
3. If artifact locations or consumer rules changed, update
   `artifacts/milestones/m1/artifact_index.yaml` in the same change.

## Protected walk

1. Pick one protected small-project fixture repo under `fixtures/repos/m1/`.
2. If the dogfood flow is blocked, add a row to
   `artifacts/milestones/m1/known_gaps_ledger.yaml` that records:
   - the blocker class (from `artifacts/milestones/m1/blocker_taxonomy.yaml`)
   - the fixture repo ref
   - the exact build identity ref
   - owner DRI, severity, waiver status, and next action
3. Point the row at the exit-gate checklist item(s) it blocks so the exit review
   cannot silently ignore it.

## Failure drill

To confirm the guardrail is live:

1. Temporarily set one ledger row's `blocker_class` to a non-existent value (or
   delete a required field like `next_action`).
2. Rerun the validator; it must fail with an actionable error.
3. Undo the change and rerun; it must pass.

