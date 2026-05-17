# Beta first-useful-work packet manifest

This directory carries packet-shaped first-useful-work fixtures for
the marketed beta switching rows. The packet objects are intentionally
small and reference the richer existing fixtures under
`fixtures/ux/first_useful_work_cases/`, `fixtures/workspace/`, and
`fixtures/migration/` instead of duplicating their payloads.

Consumers:

- `artifacts/milestones/m3/first_useful_work_scorecard.json`
- `artifacts/migration/m3/import_diff_and_rollback_packet.md`
- `artifacts/migration/m3/restore_provenance_packet.md`
- `docs/migration/m3/marketed_switching_rows.md`
- `artifacts/release/m3/claim_manifest.json`

Rules:

- Every marketed switching row must resolve to at least one
  `packet_id` in `manifest.yaml`.
- Every packet records first-useful-work timing, manual repairs,
  blocked-by-policy causes, missing-dependency fallbacks, mapping-class
  outcomes, diagnostics, and rollback or restore provenance references.
- Account-free local paths and managed/provider-linked paths remain
  separate through `account_posture`.
- Stale or regressed packet state is claim-bearing evidence and must
  downgrade through the claim-manifest evidence hook instead of remaining
  green by default.
