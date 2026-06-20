# Environment-truth certification proof packet

The canonical environment-truth certification packet is implemented in
[`crates/aureline-env/src/env_certification/mod.rs`](../../crates/aureline-env/src/env_certification/mod.rs)
and serialized to
[`artifacts/env/m5-env-certification-packet.json`](./m5-env-certification-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/env/m5-env-certification.md`](../../docs/env/m5-env-certification.md)
- the boundary schema at
  [`schemas/env/m5-env-certification.schema.json`](../../schemas/env/m5-env-certification.schema.json)
- fixture replay in
  [`crates/aureline-env/tests/env_certification.rs`](../../crates/aureline-env/tests/env_certification.rs)
- the fixture corpus under
  [`fixtures/env/m5-env-certification/`](../../fixtures/env/m5-env-certification/)

## What the packet certifies

For each claimed M5 target class — local native, container, remote host,
devcontainer, and managed cloud — the packet proves the five required
certification aspects (capsule identity, template composition, prebuild
compatibility, lifecycle-hook truth, and runtime-instance parity), each bound to
the real checked-in upstream lane artifact, and stamps the verdict and warm-start
posture the narrowing engine reaches.

A target class is `certified` only when every aspect is `current`. Partial
evidence narrows the claim to `beta`; stale evidence narrows it to `preview`;
missing evidence withholds the claim and blocks promotion. Stale or partial
capsule-identity / prebuild-compatibility evidence additionally narrows the
warm-start posture. The certification only narrows — it never widens a claim, and
a target class absent from the packet is uncertified rather than green.

## Certified target classes

| Row | Target class | Claimed | Effective | Verdict | Warm start |
| --- | --- | --- | --- | --- | --- |
| `env.cert.local_native` | local_native | `stable` | `stable` | `certified` | `warm_partial_reuse` |
| `env.cert.container` | container | `beta` | `beta` | `certified` | `warm_full_reuse` |
| `env.cert.remote_host` | remote_host | `beta` | `beta` | `certified` | `warm_partial_reuse` |
| `env.cert.devcontainer` | devcontainer | `beta` | `beta` | `certified` | `warm_partial_reuse` |
| `env.cert.managed_cloud` | managed_cloud | `beta` | `beta` | `certified` | `warm_full_reuse` |

In the checked-in packet every aspect is `current`, so the rolled-up `promotion`
decision certifies every target class and does not block promotion.

## Automatic narrowing rules

| Trigger evidence | Maturity floor | Warm-start floor (capsule/prebuild only) |
| --- | --- | --- |
| `partial` | `beta` | `warm_partial_reuse` |
| `stale` | `preview` | `cold_build` |
| `missing` | `withdrawn` | `cold_build` |

## Failure and recovery drills

One drill per target class injects a failure into a backing aspect, narrows or
withholds the claim, then recovers to `certified` after the evidence is refreshed.
The drills cover a partial template composition (local-native → beta), a stale
prebuild fingerprint (container → preview and a forced cold build), runtime-instance
skew (remote-host → preview), a missing lifecycle hook (devcontainer → withheld and
promotion blocked), and a stale capsule digest (managed-cloud → preview and a forced
cold build).

## Publication bindings

Every binding ingests the same packet id (`env.env_certification.v1`) and
preserves the per-row verdict, effective maturity, warm-start posture, and
narrowing tokens verbatim:

- `release_shiproom` — holds promotion for any narrowed or withheld release-scope
  target class and reads the rolled-up promotion decision.
- `support_export` — re-exports the verdict, warm-start posture, and narrowing
  tokens with no raw paths, credentials, or provider payloads.
- `docs` — quotes the certified aspects, freshness and warm-start rules, verdicts,
  and the promotion decision.
- `help` — reuses the same vocabulary in the why-this-environment inspector.
