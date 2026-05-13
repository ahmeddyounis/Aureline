# Managed Workspace Truth Alpha

This document describes the bounded managed-workspace suspend/resume/reattach
truth lane implemented by `aureline-runtime`. It is a preview/runtime inspection
prototype, not a general managed workspace platform.

## Scope

Frozen here:

- one `managed_workspace_alpha_record` for a helper-backed preview/runtime row;
- one `managed_workspace_runtime_inspection_record` consumed by preview,
  runtime inspector, and support/export surfaces;
- typed lifecycle states for `suspended`, `resumed`, `reattached`, and
  `reconnect_required`;
- inspection labels for `local`, `helper_backed`, `resumed`, `stale`, and
  `inspect_only`;
- rerun posture, reachability, target freshness, and reapproval fields that
  block mutation when the runtime is stale or disconnected.

Out of scope:

- provisioning, billing, quotas, fleet orchestration, or broad control-plane UX;
- tunnel backends, browser automation, remote-agent startup, or real process
  attach;
- raw URLs, hostnames, ports, credentials, process arguments, or support-bundle
  bodies.

## Runtime Truth Rules

1. Runtime placement is explicit. Local rows label `local`; managed or helper
   rows label `helper_backed` and keep boundary chrome visible.
2. Resume does not imply restored authority. A resumed runtime can be
   `inspect_only` when session refresh, credential refresh, target review, or
   reattach is still required.
3. Reattach is allowed only against an accepted target witness. If the witness
   changed or became stale, the row becomes `reconnect_required` or
   `inspect_only`.
4. Stale preview state is labeled directly. Cached files or last-known preview
   output do not imply live ports, terminals, kernels, credentials, or rerun
   authority.
5. Rerun posture is part of the inspection record. Surfaces must not hide
   exact-prior versus current-context differences behind a normal rerun button.

## Artifacts

- Runtime model:
  [`/crates/aureline-runtime/src/managed_alpha/mod.rs`](../../crates/aureline-runtime/src/managed_alpha/mod.rs)
- Protected fixtures:
  [`/fixtures/remote/suspend_resume_reattach_alpha/`](../../fixtures/remote/suspend_resume_reattach_alpha/)
- Fixture replay:
  [`/crates/aureline-runtime/tests/managed_alpha.rs`](../../crates/aureline-runtime/tests/managed_alpha.rs)

## Verification

```sh
cargo test -p aureline-runtime --test managed_alpha
```

The wider runtime crate tests also exercise serialization and public exports:

```sh
cargo test -p aureline-runtime
```
