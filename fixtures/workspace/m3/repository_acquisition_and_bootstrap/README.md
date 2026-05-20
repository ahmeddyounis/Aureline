# Repository-acquisition beta fixtures

Scenario fixtures for the M3 repository-acquisition beta lane. Each
fixture is a self-contained scenario that binds one
`source_locator_record`, one `checkout_plan_record`, and zero or more
`bootstrap_queue_item_record`s, then asserts the derived
`RepositoryAcquisitionBetaProjection` truth via an `expect` block.

The beta contract lives at
[`/docs/workspace/m3/repository_acquisition_beta.md`](../../../../docs/workspace/m3/repository_acquisition_beta.md);
the frozen seed vocabulary lives at
[`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../../../../docs/workspace/source_acquisition_and_bootstrap_seed.md).

The inner records validate against the three boundary schemas:

- [`/schemas/workspace/source_locator.schema.json`](../../../../schemas/workspace/source_locator.schema.json)
- [`/schemas/workspace/checkout_plan.schema.json`](../../../../schemas/workspace/checkout_plan.schema.json)
- [`/schemas/workspace/bootstrap_queue_item.schema.json`](../../../../schemas/workspace/bootstrap_queue_item.schema.json)

and the derived projection record is described by:

- [`/schemas/workspace/repository_acquisition.schema.json`](../../../../schemas/workspace/repository_acquisition.schema.json)

The integration test
[`crates/aureline-workspace/tests/repository_acquisition_beta.rs`](../../../../crates/aureline-workspace/tests/repository_acquisition_beta.rs)
replays every fixture, asserts the closed acceptance truth, and
round-trips each record through the Rust descriptors.

## Fixture shape

```jsonc
{
  "__fixture__": { "name": "...", "scenario": "...", "doc_sections": ["..."] },
  "surface": "start_center",            // AcquisitionSurface token
  "locator": { /* source_locator_record */ },
  "plan": { /* checkout_plan_record */ },
  "bootstrap_items": [ /* bootstrap_queue_item_record, ... */ ],
  "expect": {
    "acquisition_verb": "...",
    "checkout_mode": "...",
    "partial_or_sparse": false,
    "submodule_policy": "...",
    "lfs_policy": "...",
    "expected_cost_band": "...",
    "credential_posture": "...",
    "credential_reauth_required": false,
    "interrupted": false,
    "interrupted_branches": ["..."],
    "manual_followup_count": 0,
    "honesty_labels": ["..."],
    "guardrails_all_hold": true,
    "surface_must_disclose": false
  }
}
```

## Index

| Scenario | Verb | Highlights |
|----------|------|-----------|
| `open_local_folder` | open_local | Already-on-disk folder, no fetch, no setup. |
| `clone_remote_submodules_lfs` | clone | Submodule init pending, LFS pointer-only, three deferred items. |
| `open_snapshot_archive` | open_archive | Unsigned offline snapshot, read-only extraction. |
| `import_handoff_packet` | import | First-seen signer, read-only extracted bundle. |
| `open_template_with_bootstrap_queue` | open_template_or_prebuild | Typed toolchain / package / index / docs queue. |
| `resume_managed_workspace_reauth` | resume | Live attach, reauth required, credential provisioning. |
| `interrupted_mirror_clone_resume` | clone | Interrupted mirror clone; explicit resume / discard / open-read-only. |
| `airgap_signed_bundle` | import | Air-gap import; signed-offline freshness, signer rotation. |
| `policy_guided_generators_blocked` | clone | Fleet policy narrowing; generator-install policy-excluded. |
| `lfs_pointer_only_read_only` | clone | Shallow + LFS pointer-only read-only browse. |
