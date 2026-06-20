# Environment-truth certification

This document describes the promotion-grade environment-truth certification lane
for claimed M5 target classes. The canonical packet is implemented in
[`crates/aureline-env/src/env_certification/mod.rs`](../../crates/aureline-env/src/env_certification/mod.rs)
and serialized to
[`artifacts/env/m5-env-certification-packet.json`](../../artifacts/env/m5-env-certification-packet.json).

It is the capstone of the environment lanes: rather than re-deriving
environment truth, it folds the six frozen lanes into one promotion decision per
target class. Each certification aspect binds the **real** checked-in upstream
lane artifact as its evidence:

- **capsule identity** — the capsule object and its seven-dimension governance
  matrix at
  [`artifacts/env/environment-capsule-proof.md`](../../artifacts/env/environment-capsule-proof.md)
  and
  [`artifacts/env/m5-env-proof-packet.json`](../../artifacts/env/m5-env-proof-packet.json),
- **template composition** — the workspace-template packet at
  [`artifacts/env/workspace-template-proof.md`](../../artifacts/env/workspace-template-proof.md),
- **prebuild compatibility** — the prebuild-fingerprint packet at
  [`artifacts/env/prebuild-fingerprint-packet.json`](../../artifacts/env/prebuild-fingerprint-packet.json),
- **lifecycle-hook truth** — the hook-review packet at
  [`artifacts/env/hook-review-and-repair.md`](../../artifacts/env/hook-review-and-repair.md),
- **runtime-instance parity** — the runtime-materialization packet at
  [`artifacts/env/runtime-materialization-proof.md`](../../artifacts/env/runtime-materialization-proof.md),
- and the portability / freshness runbook at
  [`artifacts/env/env-diagnostics-runbook.md`](../../artifacts/env/env-diagnostics-runbook.md).

## Why this exists

The per-lane packets each prove one slice of environment truth. None of them, on
its own, answers the promotion question: *for this claimed target class, is every
aspect of the environment-truth lane proven current, or must the claim narrow?* A
starter, prebuild, devcontainer, remote container, or managed workspace can open
once on a happy path and imply a trustworthy environment while its capsule,
template, or prebuild evidence is stale or incomplete.

This lane closes that loophole. It turns environment truth into a single
promotion-grade claim per target class and narrows both the claim and the
warm-start reuse posture automatically when any backing aspect goes partial,
stale, or missing — and blocks promotion outright when a required aspect cannot
be proven.

## The certified aspects

Every claimed target class must prove five aspects. A target class may not
present a trustworthy environment unless all five are canonical and testable:

- **`capsule_identity`** — the capsule is identified by a typed, versioned digest
  and certified across its seven capsule dimensions.
- **`template_composition`** — template hydration composes the same capsule
  object without forking the execution or trust model.
- **`prebuild_compatibility`** — prebuild reuse is keyed on a compatibility
  fingerprint that invalidates rather than serving a stale snapshot.
- **`lifecycle_hook_truth`** — repo-defined lifecycle hooks stay trust-gated and
  reviewable rather than silently executed.
- **`runtime_instance_parity`** — the runtime instance materialized for the
  capsule stays semantically aligned with its declared target across surfaces.

## The narrowing engine

Each aspect carries an `evidence_state`. One engine —
`certify_environment_lane` — folds the per-aspect evidence into a single verdict,
an effective maturity floor, **and** a narrowed warm-start posture. It reuses the
exact per-state floor functions the capsule-dimension matrix uses, so the lane
certification and the per-dimension governance packet can never disagree about a
downgrade.

| Evidence state | Maturity floor | Warm-start floor (capsule/prebuild only) |
| --- | --- | --- |
| `current` | none | none |
| `partial` | `beta` | `warm_partial_reuse` |
| `stale` | `preview` | `cold_build` |
| `missing` | `withdrawn` | `cold_build` |
| `not_applicable` | none | none |

The effective maturity is the worst (narrowest) of the claimed maturity and every
triggered floor. The warm-start posture narrows the same way, but only the
capsule-identity and prebuild-compatibility aspects govern it — warm reuse is
trustworthy only while the capsule's identity and its cached artifact are current.
The verdict follows:

- **`certified`** — the effective maturity equals the claimed maturity.
- **`narrowed`** — the effective maturity is below the claimed maturity but the
  claim still holds (beta or preview).
- **`withheld`** — a required aspect is missing, so the claim is withdrawn and
  promotion is blocked.

The certification only ever narrows. It never promotes a target class above its
claimed maturity or warm-start posture, and a target class absent from the packet
is uncertified rather than implicitly green.

## Certified target classes

| Target class | Claimed maturity | Claimed warm start |
| --- | --- | --- |
| `local_native` | `stable` | `warm_partial_reuse` |
| `container` | `beta` | `warm_full_reuse` |
| `remote_host` | `beta` | `warm_partial_reuse` |
| `devcontainer` | `beta` | `warm_partial_reuse` |
| `managed_cloud` | `beta` | `warm_full_reuse` |

In the checked-in packet every aspect is `current`, so every target class is
`certified` at its claimed maturity and warm-start posture, and the rolled-up
`promotion` decision does not block promotion.

## Prebuilds and capsules are accelerators, not authorities

The marquee guardrail: a `warm_full_reuse` claim drops to partial reuse or a cold
build whenever the capsule identity or prebuild compatibility outruns current
truth. A stale prebuild fingerprint narrows the container claim to `preview`
**and** forces a `cold_build`; a stale capsule digest does the same for the
managed-cloud claim, so a mirrored or cached warm snapshot can never outrun the
current source.

## Failure and recovery drills

Each target class carries one failure / recovery drill. A drill injects a failure
into one aspect, observes the degraded evidence, watches the claim narrow or
withhold (and the warm-start posture downgrade where applicable), refreshes the
evidence, and recovers to `certified`. The degraded posture is computed from the
same engine the rows use, so a drill can never disagree with the certification.
The drill set covers a partial template composition (local-native → beta), a stale
prebuild fingerprint (container → preview + cold build), runtime-instance skew
(remote-host → preview), a missing lifecycle hook (devcontainer → withheld and
promotion blocked), and a stale capsule digest (managed-cloud → preview + cold
build).

## One packet for every surface

Release/shiproom, support export, docs, and help all bind to this packet rather
than re-deriving environment staleness. Each binding preserves the per-row
verdict, effective maturity, warm-start posture, narrowing tokens, and the
rolled-up promotion decision verbatim, and narrows in lockstep with the packet,
so the product tells one consistent story about its environment and warm-start
guarantees.

## Regeneration

The proof packet and fixtures are projections of the seeded packet:

```bash
cargo run -q -p aureline-env --example dump_env_certification -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/env/m5-env-certification-packet.json
```

The fixture corpus under
[`fixtures/env/m5-env-certification/`](../../fixtures/env/m5-env-certification/)
is generated the same way from the `fixtures` mode and split one file per
fixture. The replay gate in
[`crates/aureline-env/tests/env_certification.rs`](../../crates/aureline-env/tests/env_certification.rs)
fails CI if the artifact or fixtures drift from the seeded packet, or if any
aspect or lane evidence ref stops pointing at a real checked-in artifact.
