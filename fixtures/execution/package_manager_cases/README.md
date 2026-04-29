# Package-Manager And Lockfile-Safety Worked Cases

These fixtures anchor the contract frozen in
[`/docs/execution/package_manager_and_lockfile_safety_contract.md`](../../../docs/execution/package_manager_and_lockfile_safety_contract.md)
and the schemas:

- [`/schemas/execution/package_change_plan.schema.json`](../../../schemas/execution/package_change_plan.schema.json)
- [`/schemas/execution/registry_source.schema.json`](../../../schemas/execution/registry_source.schema.json)

Each fixture is one pre-implementation record. Plans carry the same
review-sheet and no-hidden-mutation guard fields whether they came from
local UI, container execution, CI/provider import, managed workspace, or
support export. Registry-source fixtures carry source, mirror, auth,
network, remapping, and visibility truth before package writes or
network actions occur.

## Scope Rules

- Fixtures use opaque ids, class labels, counts, timestamps, and
  review-safe summaries only.
- Fixtures MUST NOT encode raw manifests, raw lockfiles, raw lifecycle
  scripts, raw command lines, raw package-manager cache paths, raw
  registry URLs, hostnames, tokens, certificates, absolute paths, or
  sidecar payloads.
- The same `package_change_plan_record` shape is used for install,
  update, remove, audit, and fix flows.
- Every package-change fixture sets every no-hidden-mutation guard to
  `true`.

## Index

| Fixture | Schema | Key coverage |
| --- | --- | --- |
| [`local_install_sandboxed_lockfile_plan.yaml`](./local_install_sandboxed_lockfile_plan.yaml) | `package_change_plan` | Local npm install with sandboxed lifecycle/postinstall scripts, lockfile additions, cache writes, and rollback checkpoint. |
| [`container_update_mirror_remap_plan.yaml`](./container_update_mirror_remap_plan.yaml) | `package_change_plan` | Container pnpm update where direct public registry access is remapped to a customer mirror before write/network. |
| [`provider_fix_advisory_command_result_plan.yaml`](./provider_fix_advisory_command_result_plan.yaml) | `package_change_plan` | CI/provider-imported fix plan with advisory/license delta, command result packet linkage, and provider-managed egress. |
| [`managed_remove_missing_checkpoint_blocked_plan.yaml`](./managed_remove_missing_checkpoint_blocked_plan.yaml) | `package_change_plan` | Managed workspace remove plan blocked because rollback/checkpoint evidence is missing. |
| [`support_audit_read_only_projection.yaml`](./support_audit_read_only_projection.yaml) | `package_change_plan` | Support-export audit projection with no mutation but comparable fields and dependency-ledger linkage. |
| [`managed_mirror_policy_injected_registry_source.yaml`](./managed_mirror_policy_injected_registry_source.yaml) | `registry_source` | Managed mirror registry source with policy-injected auth, mirror-only egress, and explicit remapping review. |

Removing one of these coverage classes is a breaking change to the
pre-implementation corpus.
