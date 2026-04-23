# Source-locator, checkout-plan, and bootstrap-queue fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../../../docs/workspace/source_acquisition_and_bootstrap_seed.md)
and validated by the three schemas:

- [`/schemas/workspace/source_locator.schema.json`](../../../schemas/workspace/source_locator.schema.json)
- [`/schemas/workspace/checkout_plan.schema.json`](../../../schemas/workspace/checkout_plan.schema.json)
- [`/schemas/workspace/bootstrap_queue_item.schema.json`](../../../schemas/workspace/bootstrap_queue_item.schema.json)

Each fixture names the record kind it exercises, the locator
class / trust stage / resume state / topology markers /
bootstrap-item class / execution class / absence class /
attributable-evidence classes / next-step decision hooks it
covers, and the worked-example section of the seed document it
motivates.

**Scope rules**

- Every fixture validates against exactly one of the three
  schemas; a fixture does not encode wire bytes, ADR-0004 RPC
  envelopes, or ADR-0005 subscription envelopes.
- A new fixture MUST exercise at least one frozen locator class,
  trust stage, resume state, topology marker, bootstrap-item
  class, absence class, or attributable-evidence class, and MUST
  cite the seed section that motivates it.
- Monotonic timestamps and opaque ids are chosen to read well
  rather than to reflect any real clock or system state.
- ADR-0001 trust-state, ADR-0006 filesystem-identity, ADR-0007
  credential-handle, and ADR-0010 connected-provider /
  browser-handoff approval-ticket vocabularies are quoted by
  reference and never redefined.

**Fixture filename convention**

Case-scoped fixtures share a common prefix so companion
records (locator, plan, bootstrap items) group together:

- `<case>__locator.json` — the `source_locator_record`.
- `<case>__plan.json` — the `checkout_plan_record`.
- `<case>__bootstrap_<item_class>.json` — one
  `bootstrap_queue_item_record` from the case's queue.

**Index**

| Case | Seed section | Locator | Plan | Bootstrap items |
|------|--------------|---------|------|-----------------|
| `clone_remote_with_submodules_and_lfs` | §4.1 | [`clone_remote_with_submodules_and_lfs__locator.json`](./clone_remote_with_submodules_and_lfs__locator.json) | [`clone_remote_with_submodules_and_lfs__plan.json`](./clone_remote_with_submodules_and_lfs__plan.json) | [`submodule_init`](./clone_remote_with_submodules_and_lfs__bootstrap_submodule_init.json), [`lfs_hydrate`](./clone_remote_with_submodules_and_lfs__bootstrap_lfs_hydrate.json), [`package_restore`](./clone_remote_with_submodules_and_lfs__bootstrap_package_restore.json) |
| `resume_interrupted_mirror_clone` | §4.2 | [`resume_interrupted_mirror_clone__locator.json`](./resume_interrupted_mirror_clone__locator.json) | [`resume_interrupted_mirror_clone__plan.json`](./resume_interrupted_mirror_clone__plan.json) | — |
| `snapshot_archive_import` | §4.3 | [`snapshot_archive_import__locator.json`](./snapshot_archive_import__locator.json) | [`snapshot_archive_import__plan.json`](./snapshot_archive_import__plan.json) | — |
| `prebuild_with_bootstrap_queue` | §4.4 | [`prebuild_with_bootstrap_queue__locator.json`](./prebuild_with_bootstrap_queue__locator.json) | [`prebuild_with_bootstrap_queue__plan.json`](./prebuild_with_bootstrap_queue__plan.json) | [`toolchain_install`](./prebuild_with_bootstrap_queue__bootstrap_toolchain_install.json), [`package_restore`](./prebuild_with_bootstrap_queue__bootstrap_package_restore.json), [`devcontainer_attach`](./prebuild_with_bootstrap_queue__bootstrap_devcontainer_attach.json) |
| `live_resume_managed_workspace` | §4.5 | [`live_resume_managed_workspace__locator.json`](./live_resume_managed_workspace__locator.json) | [`live_resume_managed_workspace__plan.json`](./live_resume_managed_workspace__plan.json) | [`credential_provisioning`](./live_resume_managed_workspace__bootstrap_credential_provisioning.json) |
| `lfs_pointer_only_read_only` | §4.6 | [`lfs_pointer_only_read_only__locator.json`](./lfs_pointer_only_read_only__locator.json) | [`lfs_pointer_only_read_only__plan.json`](./lfs_pointer_only_read_only__plan.json) | [`lfs_hydrate`](./lfs_pointer_only_read_only__bootstrap_lfs_hydrate.json) |
| `policy_guided_deployment_generators_blocked` | §4.7 | [`policy_guided_deployment_generators_blocked__locator.json`](./policy_guided_deployment_generators_blocked__locator.json) | [`policy_guided_deployment_generators_blocked__plan.json`](./policy_guided_deployment_generators_blocked__plan.json) | [`generator_install`](./policy_guided_deployment_generators_blocked__bootstrap_generator_install.json), [`package_restore`](./policy_guided_deployment_generators_blocked__bootstrap_package_restore.json) |
