# Beta publish, rollback, revocation, and advisory rehearsals

This packet is the beta release-control rehearsal entry point. It keeps
publish, rollback, revocation, and advisory drills tied to the same
artifact graph instead of leaving each path as a separate checklist.

Canonical machine source:

- `artifacts/release/m3/rehearsals/packet.json`
- `fixtures/release/m3/rehearsal_inputs/manifest.yaml`
- `artifacts/release/m3/rehearsals/support_export_projection.json`
- `artifacts/release/m3/rehearsals/captures/publish_rollback_revocation_rehearsal_validation_capture.json`

The headless consumer is:

```bash
python3 -m tools.ci.m3.rehearsals --repo-root . --check
```

## Rehearsed flows

| Flow | Current result | Release-control consequence | Mirror/offline consequence |
|---|---|---|---|
| Publish to design-partner feed and offline bundle | `passed_with_limits` | Hold channel widening while package bytes remain signature-pending. | Offline bundle export path is rehearsed, but a fresh disconnected export is still required. |
| Coordinated rollback to prior stable | `passed` | No gap; rollback remains eligible with review through the retained artifact set. | Mirror and offline paths quote the same prior exact-build target. |
| Revocation propagation | `blocked_with_decision` | Promotion blocker routes through the correction packet instead of becoming a release-time surprise. | Stale mirror metadata pins last-known-good state until successor revocation metadata is imported. |
| Advisory publication | `passed_with_downgrade` | Affected-build advisory can publish, while the packaging/update claim narrows until mirror revocation proof is current. | Advisory export calls out stale mirrored feeds and offline import requirements. |

## Acceptance rule

A rehearsal is current only when the packet validates, every required
flow has a matching fixture input, and every non-green result cites a
downgrade, hold, or blocker decision. The support projection is the
surface that Help, release docs, and support export quote; prose in this
document is secondary.

## Refresh triggers

Refresh the packet and regenerate the support projection when any of
these change:

- `artifacts/release/m3/artifact_graph.json`
- `artifacts/release/m3/release_center_pack/pack.json`
- `artifacts/release/m3/update_rollback/rollback_plan.json`
- `artifacts/release/m3/correction_train/packet.json`
- `artifacts/release/m3/claim_manifest.json`
- `fixtures/release/mirror_integrity_cases/stale_revocation_metadata_propagation.yaml`
- `fixtures/release/update_rollback_cases/offline_or_mirror_success.yaml`

## Failure drill

Change one fixture's `expected_result_state` to a value that does not
match the canonical packet, then run the headless command above. The
validator must fail with `fixtures.case.failed`. Restore the fixture and
rerun; the command must pass and the generated capture must remain
unchanged under `--check`.
