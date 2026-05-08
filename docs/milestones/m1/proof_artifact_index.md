# Proof artifact index and storage conventions

This page is the reviewer-facing entry point for the M1 proof artifact index.
The canonical truth source is `artifacts/milestones/m1/artifact_index.yaml`;
downstream packets, dashboards, and checklists should reference that file
instead of scattering proof links across ad hoc folders.

## Canonical artifacts

- Human entry (this file): `docs/milestones/m1/proof_artifact_index.md`
- Proof artifact index (canonical): `artifacts/milestones/m1/artifact_index.yaml`
- Validator: `ci/check_m1_artifact_index.py`
- CI gate: `.github/workflows/check_m1_artifact_index.yml`

## Storage conventions (M1 proof)

Reviewer entrypoints live under `docs/milestones/m1/`. Proof packets, captures,
and other evidence live under `artifacts/milestones/m1/` using the folders
below:

- Packets: `artifacts/milestones/m1/proof_packets/`
- Validation captures (machine-readable): `artifacts/milestones/m1/captures/`
- Traces: `artifacts/milestones/m1/traces/`
- Screenshots: `artifacts/milestones/m1/screenshots/`
- Smoke outputs: `artifacts/milestones/m1/smoke_outputs/`
- Support exports: `artifacts/milestones/m1/support_exports/`
- Replay fixtures: `artifacts/milestones/m1/replay_fixtures/`
- Design audits: `artifacts/milestones/m1/design_audits/`

If a proof lane needs additional roots, the lane must declare them explicitly
in `artifacts/milestones/m1/artifact_index.yaml` and the validator must accept
them. Lanes must not write evidence outside declared roots.

### Naming

- Proof packets: `artifacts/milestones/m1/proof_packets/<lane_id>.md`
- Validation captures: `artifacts/milestones/m1/captures/<lane_id>_validation_capture.json`
- Other evidence: prefix with `<lane_id>_` and include a `YYYY-MM-DD` date (and
  optionally a short build identity) to keep captures sortable and traceable.

## Registration fields (per proof lane)

Every proof lane entry in `artifacts/milestones/m1/artifact_index.yaml` must:

- name one `owner_dri`;
- resolve to exactly one owning proof packet under
  `artifacts/milestones/m1/proof_packets/`;
- cite the exact build identity artifact (see `artifacts/build/build_identity.json`);
- declare the validation lane(s) and freshness state; and
- link to its latest capture (or declare it planned-only).

## How to validate

Run:

`python3 ci/check_m1_artifact_index.py --repo-root .`

Optional machine-readable report:

`python3 ci/check_m1_artifact_index.py --repo-root . --report target/m1-artifact-index/report.json`

## Update rules

1. Update `artifacts/milestones/m1/artifact_index.yaml` first.
2. Ensure the owning packet under `artifacts/milestones/m1/proof_packets/` is
   updated in the same change.
3. Run the validator locally.
4. If a lane's roots, owner, or capture refs changed, refresh the lane's
   `freshness` metadata and update its capture registration.

## Failure drill

To confirm the guardrail is live:

1. Temporarily move one capture referenced by a non-planned lane (for example,
   `artifacts/milestones/m1/captures/integration_checkpoint_validation_capture.json`) out of
   `artifacts/milestones/m1/`.
2. Rerun the validator; it must fail with an actionable error.
3. Undo the move and rerun; it must pass.
