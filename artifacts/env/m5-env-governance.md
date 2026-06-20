# Environment-capsule governance proof packet

The canonical environment-capsule governance packet is implemented in
[`crates/aureline-env/src/m5_env_governance/mod.rs`](../../crates/aureline-env/src/m5_env_governance/mod.rs)
and serialized to
[`artifacts/env/m5-env-proof-packet.json`](./m5-env-proof-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/env/m5-env-governance.md`](../../docs/env/m5-env-governance.md)
- the boundary schema at
  [`schemas/env/m5-env-governance.schema.json`](../../schemas/env/m5-env-governance.schema.json)
- fixture replay in
  [`crates/aureline-env/tests/m5_env_governance.rs`](../../crates/aureline-env/tests/m5_env_governance.rs)
- the fixture corpus under
  [`fixtures/env/m5-env-governance/`](../../fixtures/env/m5-env-governance/)

## What the packet certifies

For each claimed M5 environment profile — workspace template, starter, prebuild,
devcontainer, remote container, and managed workspace — the packet proves the
seven required capsule dimensions (source digest, target plan, toolchain plan,
trust hooks, service graph, prebuild fingerprint, and materialization parity) and
stamps the verdict and warm-start posture the narrowing engine reaches.

A profile is `certified` only when every dimension is `current`. Partial evidence
narrows the claim to `beta`; stale evidence narrows it to `preview`; missing
evidence withholds the claim. Stale or partial source-digest / prebuild-fingerprint
evidence additionally narrows the warm-start posture. The certification only
narrows — it never widens a claim, and a profile absent from the packet is
uncertified rather than green.

## Certified rows

| Row | Profile | Claimed | Effective | Verdict | Warm start |
| --- | --- | --- | --- | --- | --- |
| `env.capsule.workspace_template` | workspace_template | `stable` | `stable` | `certified` | `cold_build` |
| `env.capsule.starter` | starter | `stable` | `stable` | `certified` | `warm_partial_reuse` |
| `env.capsule.prebuild` | prebuild | `beta` | `beta` | `certified` | `warm_full_reuse` |
| `env.capsule.devcontainer` | devcontainer | `beta` | `beta` | `certified` | `warm_partial_reuse` |
| `env.capsule.remote_container` | remote_container | `beta` | `beta` | `certified` | `warm_partial_reuse` |
| `env.capsule.managed_workspace` | managed_workspace | `beta` | `beta` | `certified` | `warm_full_reuse` |

## Automatic narrowing rules

| Trigger evidence | Maturity floor | Warm-start floor (source/prebuild only) |
| --- | --- | --- |
| `partial` | `beta` | `warm_partial_reuse` |
| `stale` | `preview` | `cold_build` |
| `missing` | `withdrawn` | `cold_build` |

## Failure and recovery drills

One drill per profile injects a failure into a backing dimension, narrows or
withholds the claim, then recovers to `certified` after the evidence is
refreshed. The drills cover a missing trust hook (template → withheld), partial
source digest (starter → beta), a stale prebuild fingerprint (prebuild → preview
and a forced cold build), a stale service graph (devcontainer → preview), a stale
toolchain plan (remote container → preview), and materialization skew (managed
workspace → preview).

## Publication bindings

Every binding ingests the same packet id (`env.m5_env_governance.v1`) and
preserves the per-row verdict, effective maturity, warm-start posture, and
narrowing tokens verbatim:

- `release_shiproom` — holds promotion for any narrowed or withheld release-scope
  profile.
- `support_export` — re-exports the verdict, warm-start posture, and narrowing
  tokens with no raw paths, credentials, or provider payloads.
- `docs` — quotes the certified dimensions, freshness and warm-start rules, and
  verdicts.
- `help` — reuses the same vocabulary in the why-this-environment inspector.
