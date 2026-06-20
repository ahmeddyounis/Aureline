# Typed environment capsule

This document describes the typed environment-capsule object and its
why-this-environment inspector. The canonical implementation is
[`crates/aureline-env/src/capsules/mod.rs`](../../crates/aureline-env/src/capsules/mod.rs);
the corpus and expected inspection outcomes are checked in under
[`fixtures/env/environment-capsule/`](../../fixtures/env/environment-capsule/)
and the human-readable proof is
[`artifacts/env/environment-capsule-proof.md`](../../artifacts/env/environment-capsule-proof.md).

It builds directly on the environment-capsule governance matrix described in
[`docs/env/m5-env-governance.md`](m5-env-governance.md): the governance lane
*certifies* environment-capsule truth per claimed profile, and this lane
*materializes the capsule object itself* and runs it through the same engine.

## Why this exists

The governance matrix proved that environment truth must narrow visibly when
its backing evidence goes partial, stale, or missing. But it left the capsule
itself implicit. Templates, starters, prebuilds, devcontainer / remote /
container flows, and managed workspaces all need one concrete, typed
environment definition to point at — otherwise each surface invents its own ad
hoc metadata blob and its own private explainability format.

This lane closes that gap. It materializes a single `EnvironmentCapsule`
object that is inspectable, diffable, mirrorable, and versioned, and a single
inspector that every surface reads.

## What the capsule carries

An `EnvironmentCapsule` is composed of typed fields, one per environment
concern:

- **`identity`** — a `CapsuleIdentity` with a stable id, a monotonic version,
  the claimed profile (reused from the governance vocabulary), the reused
  materialization class, the transport, and a versioned digest of the capsule's
  defining inputs.
- **`source_refs`** — typed `CapsuleSourceRef`s (template, lockfile,
  devcontainer config, toolchain manifest, service manifest, prebuild
  snapshot), each pinned by a `CapsuleDigest` so capsule identity is inspectable
  and diffable.
- **`target_plan`** — how and where the environment materializes: the reused
  `MaterializationClass`, the concrete `TargetTransport`, the host-boundary
  contract it obeys, and where its working tree roots.
- **`service_graph`** — the services, exposed ports, and dependencies the
  capsule stands up.
- **`toolchain_plan`** — the pinned language and runtime components.
- **`trust_hooks`** — declared lifecycle hooks, each carrying a `TrustGateState`
  and an authority-contract reference; the hook command is reduced to a digest.
- **`compatibility_fingerprint`** — the fingerprint and its inputs that warm
  start validates a prebuild against.
- **`materialization`** — the runtime parity status against the capsule object.
- **`observability`** — capsule lifecycle, span, and health-probe references.

## Metadata-first by construction

The capsule never stores secrets or raw environment bodies. Lifecycle hook
commands are stored as `command_digest`s, and environment-variable values are
stored as `value_digest`s — only names and digests cross the boundary. The
observability redaction class is fixed to `metadata_only`, and
`export_capsule_metadata` projects a support view of ids, digests, versions,
and gate states only. No secrets, raw env bodies, hook commands, or provider
payloads are serialized.

## One inspector, one engine

`inspect_environment` is the single explainability path. It folds the
capsule's own typed fields into the seven governance capsule dimensions —

- `source_digest` from the aggregate coverage of the `source_refs`,
- `target_plan`, `toolchain_plan`, `service_graph`, `prebuild_fingerprint`, and
  `materialization_parity` from their respective `coverage` / `parity_state`
  fields,
- `trust_hooks` from the hooks' gate states (an ungated hook is treated as
  missing evidence, a pending hook as partial) —

and runs the **same** `certify_capsule_outcome` narrowing engine the
governance matrix uses. The result is one `WhyThisEnvironment` report carrying
the effective maturity, verdict, warm-start posture, narrowing tokens, and
per-dimension reasons.

`desktop_environment_inspection`, `headless_environment_inspection`, and
`support_environment_inspection` all delegate to `inspect_environment`, so
desktop, CLI / headless, and support read the same object. A stale prebuild or
an ungated hook therefore downgrades identically on every surface, and the
inspector can never tell a greener story than the governance engine.

## Inspect, diff, export

- **Inspect** — `inspect_environment` returns the canonical
  `WhyThisEnvironment` report.
- **Diff** — `diff_capsules` compares two capsules field-by-field (identity
  digest and version, maturity, posture, source-ref digests, toolchain
  versions, transport, trust-hook gate states, fingerprint) and reports the
  changes as metadata tokens.
- **Export** — `export_capsule_metadata` projects a redaction-safe
  `CapsuleExport` wrapping the same inspection for support and release surfaces.

## Target classes covered

The fixture corpus covers every claimed target class — `local`, `ssh`,
`container`, `devcontainer`, `vm`, and `managed_workspace` — each certifying at
its claimed maturity on current evidence, plus degraded variants that drive the
inspector's narrowing, withholding, and warm-start downgrade. See the proof
report for the full table.

## Guardrails

- Templates, prebuilds, and managed rows may not bypass the capsule object with
  ad hoc metadata blobs: the inspector reads the capsule's typed fields, and the
  capsule is the only environment object the surfaces consume.
- The capsule object only narrows; the inspector never promotes a capsule above
  its claimed maturity or warm-start posture.
- The capsule does not redesign the execution context; it is the environment
  *definition* the existing runtime materializes.
