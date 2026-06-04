# Package mutation and registry review

This page describes the canonical package-mutation review packet used by
Aureline's package browser, operation review sheets, CLI/headless inspect,
Help, support exports, and release evidence. The source artifact is
`artifacts/deps/m4/package-mutation-and-registry-review.json`.

## What every package action must show

Every stable package add, update, remove, or resolve action must show:

- the ecosystem and manifest scope that will change;
- the requested range or source separately from the resolved exact version or
  source;
- affected manifests and lockfiles;
- registry or mirror source class, credential mode, freshness, reachability,
  and policy lock state;
- lockfile impact class and package-count blast radius;
- script, native-build, or new-egress risk;
- validation pack and rollback checkpoint.

AI and automation-suggested dependency changes use the same review surface as
manual changes. They do not bypass manifest, lockfile, advisory, validation, or
rollback detail.

## Search and registry states are not interchangeable

Package search and detail surfaces must render these states differently:

| State | Meaning |
|---|---|
| `no_results` | Search completed and found no matching package. |
| `auth_required` | Registry access needs credentials before results can be trusted. |
| `mirror_stale` | Mirror data exists but is stale. |
| `offline_snapshot_only` | Only offline snapshot metadata is available. |
| `unknown_or_stale` | The package state cannot support a stable write claim. |

These states must not collapse into one generic empty or offline result.

## Registry credential modes

Registry and mirror auth panels use the same credential vocabulary everywhere:

| Mode | Meaning |
|---|---|
| `anonymous` | No credential is required. |
| `os_store` | The secret broker resolves a credential from the OS store. |
| `token` | Access is token-backed, but exports carry only a handle or redacted source label. |
| `browser_or_device_sign_in` | The user must complete system-browser or device sign-in. |
| `policy_inherited` | Registry access is inherited from policy or workspace configuration. |

Raw tokens, authorization headers, and private registry URLs are excluded from
ordinary support exports.

## Lockfile impact

The operation review sheet classifies lockfile impact before write:

| Class | Meaning |
|---|---|
| `direct_bump` | A direct manifest dependency changes. |
| `security_patch` | A vulnerability or advisory remediation changes packages. |
| `grouped_refresh` | Related packages move together. |
| `lockfile_refresh_only` | The lockfile changes without a requested manifest bump. |
| `major_version_pilot` | A major-version change needs compatibility review. |
| `workspace_wide_convergence` | Multiple workspace manifests converge on one dependency set. |

The packet also records direct and transitive package counts so reviewers can
judge blast radius before apply.

## Recovery and support exports

Operation history rows preserve timestamp, operation ref, result class, recovery
action, and support-export ref. Support exports use the redaction-safe
projection from the `aureline-deps` crate and do not clone UI-only labels.

Programmatic consumers can load the checked-in packet:

```rust
use aureline_deps::package_mutation_and_registry_review;

let packet = package_mutation_and_registry_review::current_package_mutation_and_registry_review()?;
```
