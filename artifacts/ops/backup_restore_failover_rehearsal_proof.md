# Backup, Restore, and Failover Rehearsal Proof Packet

This packet is the reviewer entry point for the alpha continuity
taxonomy and rehearsal lane.

## Canonical Artifacts

- Taxonomy:
  [`/artifacts/ops/outage_taxonomy_alpha.yaml`](./outage_taxonomy_alpha.yaml)
- Rehearsal plan:
  [`/docs/ops/backup_restore_failover_rehearsal_plan.md`](../../docs/ops/backup_restore_failover_rehearsal_plan.md)
- Examples:
  [`/artifacts/ops/control_plane_vs_data_plane_examples.md`](./control_plane_vs_data_plane_examples.md)
- Protected fixtures:
  [`/fixtures/ops/backup_restore_failover_rehearsal_cases/manifest.yaml`](../../fixtures/ops/backup_restore_failover_rehearsal_cases/manifest.yaml)
- Validator:
  [`/ci/check_backup_restore_failover_alpha.py`](../../ci/check_backup_restore_failover_alpha.py)

## Acceptance Coverage

The protected lane proves:

- taxonomy coverage for `local_core_continuity`,
  `control_plane_impairment`, `data_plane_impairment`, and
  `full_target_loss`;
- expected product posture on every class;
- recovery action lists on every class;
- owner, cadence, and proof artifact disclosure in the rehearsal plan;
- protected fixture coverage for every class;
- metadata-only support/release projection with exact-build identity
  required and raw payload export disabled.

## Verification

Run:

```sh
python3 ci/check_backup_restore_failover_alpha.py --repo-root .
python3 ci/check_backup_restore_failover_alpha.py --repo-root . --render-support-projection
```

The first command validates the artifacts and fixtures. The second
renders the metadata-only projection support and release review can
consume without scraping prose.
