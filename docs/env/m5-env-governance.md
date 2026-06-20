# Environment-capsule governance

This document describes the environment-capsule, workspace-template,
prebuild-fingerprint, and runtime-materialization governance lane for claimed M5
environment profiles. The canonical packet is implemented in
[`crates/aureline-env/src/m5_env_governance/mod.rs`](../../crates/aureline-env/src/m5_env_governance/mod.rs)
and serialized to
[`artifacts/env/m5-env-proof-packet.json`](../../artifacts/env/m5-env-proof-packet.json).

It composes the environment-relevant packets already frozen on the M5 line:

- the build identity at
  [`artifacts/build/build_identity.json`](../../artifacts/build/build_identity.json),
- the archetype-confidence rows at
  [`artifacts/workspace/archetype_confidence_rows.yaml`](../../artifacts/workspace/archetype_confidence_rows.yaml),
- the remote host-boundary matrix at
  [`artifacts/remote/host_boundary_matrix.yaml`](../../artifacts/remote/host_boundary_matrix.yaml),
- the install state-root matrix at
  [`artifacts/install/state_root_matrix.yaml`](../../artifacts/install/state_root_matrix.yaml),
- the runtime execution-scope matrix and authority classes at
  [`artifacts/runtime/execution_scope_matrix.yaml`](../../artifacts/runtime/execution_scope_matrix.yaml)
  and
  [`artifacts/runtime/authority_classes.yaml`](../../artifacts/runtime/authority_classes.yaml),
- the managed-workspace lifecycle at
  [`artifacts/runtime/managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml),
- the warm-start chooser and environment-starter summary contracts at
  [`artifacts/entry/warm_start_chooser_contract.md`](../../artifacts/entry/warm_start_chooser_contract.md)
  and
  [`artifacts/entry/environment_starter_summary_contract.md`](../../artifacts/entry/environment_starter_summary_contract.md).

## Why this exists

The M5 line already covers workflow bundles, project entry, install topology,
build intelligence, managed-workspace lifecycle, remote boundaries, and runtime
authority. What it leaves implicit is the actual *environment-definition
contract*: the typed capsule a template hydrates, a prebuild fingerprints, and a
runtime materializes. Without one governed matrix, a template, starter,
prebuild, devcontainer, remote container, or managed workspace can imply
trustworthy environment reuse while it only knows an approximate or stale warm
snapshot.

This lane closes that loophole. It turns environment truth into a
promotion-grade claim per claimed profile and narrows both the claim and the
warm-start reuse posture automatically when the backing evidence goes partial,
stale, or missing.

## The certified dimensions

Every claimed profile must prove seven capsule dimensions. A profile may not
present an environment as trustworthy unless all seven are canonical and
testable:

- **`source_digest`** — the capsule is identified by a typed, versioned digest of
  its defining inputs, so identity is inspectable and diffable.
- **`target_plan`** — the capsule declares its materialization target plan rather
  than inferring it from side effects.
- **`toolchain_plan`** — the capsule pins a deterministic toolchain plan.
- **`trust_hooks`** — lifecycle hooks are declared and trust-gated, never silently
  executed at hydration.
- **`service_graph`** — the capsule declares the service graph it materializes.
- **`prebuild_fingerprint`** — prebuild reuse is validated against the source-digest
  fingerprint and invalidates rather than serving a stale snapshot.
- **`materialization_parity`** — runtime materialization stays semantically aligned
  with the same capsule object across desktop, CLI, AI, support, and managed rows.

## The narrowing engine

Each dimension carries an `evidence_state`. One engine —
`certify_capsule_outcome` — folds the per-dimension evidence into a single
verdict, an effective maturity floor, **and** a narrowed warm-start posture. It
is the only place the downgrade rule lives; the rows, the drills, the fixtures,
the freshness rules, and the warm-start rules all read it.

| Evidence state | Maturity floor | Warm-start floor (source/prebuild only) |
| --- | --- | --- |
| `current` | none | none |
| `partial` | `beta` | `warm_partial_reuse` |
| `stale` | `preview` | `cold_build` |
| `missing` | `withdrawn` | `cold_build` |
| `not_applicable` | none | none |

The effective maturity is the worst (narrowest) of the claimed maturity and
every triggered floor. The warm-start posture is narrowed the same way, but only
the source-digest and prebuild-fingerprint dimensions govern it — warm reuse is
trustworthy only while the capsule's identity and its cached artifact are
current. The verdict follows:

- **`certified`** — the effective maturity equals the claimed maturity.
- **`narrowed`** — the effective maturity is below the claimed maturity but the
  claim still holds (beta or preview).
- **`withheld`** — a required dimension is missing, so the claim is withdrawn.

The certification only ever narrows. It never promotes a profile above its
claimed maturity or warm-start posture, and a profile absent from the packet is
uncertified rather than implicitly green.

## Certified profiles

| Profile | Materialization | Claimed maturity | Claimed warm start |
| --- | --- | --- | --- |
| `workspace_template` | `local_native` | `stable` | `cold_build` |
| `starter` | `local_native` | `stable` | `warm_partial_reuse` |
| `prebuild` | `container` | `beta` | `warm_full_reuse` |
| `devcontainer` | `devcontainer` | `beta` | `warm_partial_reuse` |
| `remote_container` | `remote_host` | `beta` | `warm_partial_reuse` |
| `managed_workspace` | `managed_cloud` | `beta` | `warm_full_reuse` |

In the checked-in packet every dimension is `current`, so every profile is
`certified` at its claimed maturity and warm-start posture.

## Prebuilds are accelerators, not authorities

The marquee guardrail: a `warm_full_reuse` claim drops to partial reuse or a cold
build whenever the source digest or prebuild fingerprint outruns current truth. A
stale prebuild fingerprint narrows the prebuild claim to `preview` **and** forces
a `cold_build`, so a new starter or runtime surface can never imply trustworthy
reuse when it only knows an approximate or stale warm snapshot.

## Failure and recovery drills

Each profile carries one failure / recovery drill. A drill injects a failure
into one dimension, observes the degraded evidence, watches the claim narrow or
withhold (and the warm-start posture downgrade where applicable), refreshes the
evidence, and recovers to `certified`. The degraded posture is computed from the
same engine the rows use, so a drill can never disagree with the certification.
The drill set covers a missing trust hook (template → withheld), partial source
digest (starter → beta), a stale prebuild fingerprint (prebuild → preview +
cold build), a stale service graph (devcontainer → preview), a stale toolchain
plan (remote container → preview), and materialization skew (managed workspace →
preview).

## One packet for every surface

Release/shiproom, support export, docs, and help all bind to this packet rather
than re-deriving environment staleness. Each binding preserves the per-row
verdict, effective maturity, warm-start posture, and narrowing tokens verbatim,
and narrows in lockstep with the packet, so the product tells one consistent
story about its environment guarantees.

## Regeneration

The proof packet and fixtures are projections of the seeded packet:

```bash
cargo run -q -p aureline-env --example dump_m5_env_governance -- packet \
  | python3 -c 'import json,sys; print(json.dumps(json.load(sys.stdin), indent=2, sort_keys=True))' \
  > artifacts/env/m5-env-proof-packet.json
```

The fixture corpus under
[`fixtures/env/m5-env-governance/`](../../fixtures/env/m5-env-governance/)
is generated the same way from the `fixtures` mode and split one file per
fixture. The replay gate in
[`crates/aureline-env/tests/m5_env_governance.rs`](../../crates/aureline-env/tests/m5_env_governance.rs)
fails CI if the artifact or fixtures drift from the seeded packet.
