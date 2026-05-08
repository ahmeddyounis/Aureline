# Late-M1 public-truth checkpoint pack

This page is the reviewer-facing entry point for the late-M1 public-truth
checkpoint pack. The canonical truth source is
`artifacts/milestones/m1/late_freeze_pack.yaml`; downstream packets, dashboards,
and CI gates should reference that file instead of restating “what must exist”
for Help/About, command diagnostics, Start Center truth, activity truth, and
embedded-boundary chrome.

## Canonical artifacts

- Human entry (this file): `docs/milestones/m1/late_freeze_pack.md`
- Checkpoint pack (canonical): `artifacts/milestones/m1/late_freeze_pack.yaml`
- Review workflow: `docs/review/m1_public_truth_packet.md`
- Proof artifact index (review entrypoint): `docs/milestones/m1/proof_artifact_index.md`
- Proof index (canonical): `artifacts/milestones/m1/artifact_index.yaml`
- Validator: `ci/check_m1_late_freeze_pack.py`
- CI gate: `.github/workflows/check_m1_late_freeze_pack.yml`

## How to validate

Run:

`python3 ci/check_m1_late_freeze_pack.py --repo-root .`

Optional machine-readable report:

`python3 ci/check_m1_late_freeze_pack.py --repo-root . --report target/m1-late-freeze-pack/report.json`

## Update rules

1. Update `artifacts/milestones/m1/late_freeze_pack.yaml` first.
2. Run the validator locally.
3. If canonical paths, storage roots, or consumer rules changed, update
   `artifacts/milestones/m1/artifact_index.yaml` in the same change.

## Protected walk

1. Open the canonical artifacts referenced by the pack (Help/About route
   contract, docs-pack manifest contract, command diagnostics contract, origin
   taxonomy, and support bundle contract).
2. Confirm each referenced surface uses the same source/version/freshness/build
   identity vocabulary and does not mint its own “About/help/diagnostics” truth.
3. If any surface needs to narrow a claim (or deny render) without proof,
   record the missing item by updating the pack rather than adding a local note.

## Failure drill

To confirm the guardrail is live:

1. Temporarily change one required output’s `seed_state` to `seeded` while
   pointing one of its `proof_artifact_refs` at a non-existent path.
2. Rerun the validator; it must fail with an actionable error.
3. Undo the change and rerun; it must pass.

