# Environment-capsule proof

This report is the human-readable proof for the typed environment-capsule
object and its why-this-environment inspector. The canonical implementation is
[`crates/aureline-env/src/capsules/mod.rs`](../../crates/aureline-env/src/capsules/mod.rs);
the capsule corpus and its expected inspection outcomes are checked in under
[`fixtures/env/environment-capsule/`](../../fixtures/env/environment-capsule/)
and validated by `crates/aureline-env/tests/environment_capsule.rs`.

## What the capsule is

An `EnvironmentCapsule` is the concrete, typed environment definition that a
template hydrates, a prebuild fingerprints, and a runtime materializes. It
carries every field the environment-truth lane needs as inspectable,
diffable, serde-serializable data:

- a `CapsuleIdentity` — id, version, profile, materialization class, transport,
  and a versioned content digest,
- typed `source_refs`, each pinned by a digest,
- a `target_plan` declaring how and where the environment materializes,
- a `service_graph` of services, ports, and dependencies,
- a `toolchain_plan` pinning language and runtime versions,
- trust-gated lifecycle `trust_hooks`,
- a `compatibility_fingerprint` over the inputs warm start reuses,
- a `materialization` parity status, and
- `observability` metadata.

The capsule never stores secrets or raw environment bodies. Lifecycle commands
and environment-variable values are reduced to digests, so the object is
metadata-first by construction.

## One inspector, one engine

`inspect_environment` folds a capsule's own typed fields into the seven
governance capsule dimensions and runs the **same** `certify_capsule_outcome`
narrowing engine the governance matrix
([`docs/env/m5-env-governance.md`](../../docs/env/m5-env-governance.md)) uses.
Desktop (`desktop_environment_inspection`), CLI / headless
(`headless_environment_inspection`), and support
(`support_environment_inspection`) all read the **same** `WhyThisEnvironment`
object, so a stale prebuild or an ungated hook downgrades visibly and
identically on every surface — not through a private explainability format.

`export_capsule_metadata` projects a redaction-safe support view (ids,
digests, versions, gate states only), and `diff_capsules` compares two
capsules field-by-field, so a capsule can be inspected, diffed, exported, and
tested across both local and non-local paths.

## Certified corpus

| Target class | Profile | Materialization | Transport | Claimed | Effective | Verdict | Warm start |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `local` | `workspace_template` | `local_native` | `local_process` | `stable` | `stable` | `certified` | `cold_build` → `cold_build` |
| `ssh` | `remote_container` | `remote_host` | `ssh` | `beta` | `beta` | `certified` | `warm_partial_reuse` → `warm_partial_reuse` |
| `container` | `prebuild` | `container` | `container` | `beta` | `beta` | `certified` | `warm_full_reuse` → `warm_full_reuse` |
| `devcontainer` | `devcontainer` | `devcontainer` | `container` | `beta` | `beta` | `certified` | `warm_partial_reuse` → `warm_partial_reuse` |
| `vm` | `starter` | `remote_host` | `virtual_machine` | `stable` | `stable` | `certified` | `warm_partial_reuse` → `warm_partial_reuse` |
| `managed_workspace` | `managed_workspace` | `managed_cloud` | `cloud_managed` | `beta` | `beta` | `certified` | `warm_full_reuse` → `warm_full_reuse` |

Every claimed target class — local, SSH, container, devcontainer, VM, and
managed workspace — is represented by a capsule that certifies at its claimed
maturity and warm-start posture on fully current evidence.

## Failure / recovery scenarios

| Scenario | Target class | Injected | Verdict | Maturity | Warm start |
| --- | --- | --- | --- | --- | --- |
| `container_prebuild_fingerprint_stale` | `container` | stale compatibility fingerprint | `narrowed` | `beta` → `preview` | `warm_full_reuse` → `cold_build` |
| `local_trust_hook_ungated` | `local` | ungated lifecycle hook | `withheld` | `stable` → `withdrawn` | `cold_build` |
| `ssh_toolchain_stale` | `ssh` | stale toolchain plan | `narrowed` | `beta` → `preview` | `warm_partial_reuse` (unchanged) |

These prove the guardrails end-to-end:

- **Prebuilds are accelerators, not authorities.** A stale fingerprint narrows
  the maturity to `preview` and forces a cold build instead of presenting a
  stale warm snapshot as current truth.
- **Lifecycle hooks stay trust-gated.** An ungated hook withholds the capsule
  entirely rather than running silently during hydration.
- **Narrowing is scoped.** A stale toolchain plan narrows the maturity but does
  not touch the warm-start posture, because the toolchain plan does not govern
  warm reuse.

## How to verify

```
cargo test -p aureline-env
cargo run -p aureline-env --example dump_environment_capsule fixtures
```
