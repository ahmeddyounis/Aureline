# Deployment impairment-case fixtures

These fixtures are the concrete scenario records referenced by
[`/artifacts/support/deployment_drill_catalog_seed.yaml`](../../../artifacts/support/deployment_drill_catalog_seed.yaml).
They are seed inputs for continuity, disaster-recovery, and
control-plane/data-plane impairment planning.

Each fixture records:

- the `drill_id` from the catalog,
- one impairment-plane posture,
- the expected control-plane and workspace/runtime strip state,
- locality / region / tenant / key-mode / restore truth,
- retained local-safe capabilities,
- blocked managed-only capabilities, and
- the evidence outputs a later support, release, or boundary packet
  should carry.

Seeded fixtures:

- `benchmark_lab_local_only_capture.json`
- `docs_pack_mirror_only_truth.json`
- `mirror_import_offline_bundle_replay.json`
- `stale_policy_session_cached_local_safe.json`
- `remote_connector_loss_continue_local.json`
- `failover_boundary_recheck_future_managed_case.json`
