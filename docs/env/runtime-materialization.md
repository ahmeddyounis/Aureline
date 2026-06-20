# Runtime-instance materialization parity

This document describes the explicit runtime-instance object and the parity
engine that compares it against the environment capsule it materializes. The
canonical implementation is
[`crates/aureline-env/src/runtime_materialization/mod.rs`](../../crates/aureline-env/src/runtime_materialization/mod.rs);
the corpus and expected parity outcomes are checked in under
[`fixtures/env/runtime-materialization/`](../../fixtures/env/runtime-materialization/)
and the human-readable proof is
[`artifacts/env/runtime-materialization-proof.md`](../../artifacts/env/runtime-materialization-proof.md).

It builds directly on the typed environment capsule described in
[`docs/env/environment-capsule.md`](environment-capsule.md): the capsule lane
materializes the environment *definition*, and this lane materializes the
runtime *instance* the capsule declares and compares the two.

## Why this exists

The capsule object proves where the environment *said* it would run — its
target plan, transport, service graph, and working root. But the place code
*actually* ran was still implicit: a start could collapse a local, SSH,
container, devcontainer, VM, or managed-workspace runtime into one generic
"workspace started" label, hiding a wrong-target run or a half-up stack behind
the same copy.

This lane closes that gap. It makes the place code runs explainable in the
**same vocabulary** as the place the environment said it would run, so a
mismatch is a first-class, named outcome rather than a generic failure.

## The runtime instance

`derive_runtime_instance` projects an `EnvironmentCapsule` into an explicit
`RuntimeInstance`: the concrete runtime the capsule declares. It carries, in
the capsule's own vocabulary —

- **`process_namespace`** — a `ProcessNamespace` naming the `NamespaceKind` the
  processes ran in (host process, container namespace, remote-host session, VM
  guest, managed pod), with a namespace reference and the host boundary it sits
  behind. Never a raw pid or process table.
- **`mount_set`** — the `MountPoint`s the working tree, service volumes, and
  tool cache resolved to, each with a `MountState` (present / missing /
  divergent). Never a raw host path.
- **`port_map`** — the `PortMapping`s the declared service ports published to,
  each with a `PortState` (published / unpublished / conflicted).
- **`readiness_graph`** — one `ServiceReadiness` node per declared service, with
  a `ReadinessState` (ready / starting / unready / absent) and a health-probe
  reference, so a partial stack names the service that is not up.
- **`secret_projections`** — the `SecretProjection` points the capsule's
  declared environment is projected through, each carrying a `handle_ref` and a
  `ProjectionState` (projected / pending / missing). **The secret value is never
  carried** — only the handle.

The instance is metadata-first by construction and fixed to a `metadata_only`
redaction class.

## One engine, one object

`materialize_runtime` is the single parity engine. It derives the declared
target from the capsule, folds in the observed instance, and returns one
`RuntimeMaterialization` carrying:

- an explicit `RuntimeParity` — `aligned`, `degraded`, or `mismatched`,
- a per-facet `FacetEvaluation` for each of the six `RuntimeFacet`s (target
  identity, process namespace, mount set, port map, service readiness, secret
  projection),
- a per-service `ServiceReadinessEvaluation` saying which services were involved
  and which are not ready,
- a review-safe `where_code_ran` line, and
- the embedded runtime instance.

The parity starts aligned and narrows to the coldest facet contribution. Target
identity and process namespace are the only facets that can produce a
`mismatched` parity — a runtime that ran on the wrong target or in the wrong
namespace. The mount, port, readiness, and secret facets degrade the parity to
`degraded`: the right place, but not fully up. Each facet names the exact
element (mount, port, service, or projection) that forced the downgrade.

`desktop_runtime_materialization`, `headless_runtime_materialization`,
`ai_runtime_materialization`, and `support_runtime_materialization` all delegate
to `materialize_runtime`, so desktop, CLI / headless, AI, and support read the
**same** object. A wrong-target run or a partial-service stack therefore
downgrades identically on every surface.

## Narrowing in lockstep with the capsule

`RuntimeParity::materialization_parity_state` maps the parity back onto the
governance materialization-parity `EvidenceState`: `aligned` → `current`,
`degraded` → `partial`, `mismatched` → `stale`. So when a runtime is degraded or
mismatched, the capsule's materialization-parity dimension narrows in lockstep
rather than the runtime lane forking a parallel model.

## Inspect, export, diff

- **Materialize** — `materialize_runtime` returns the canonical
  `RuntimeMaterialization`.
- **Export** — `export_runtime_materialization` projects a redaction-safe
  `RuntimeExport` (parity, where code ran, degraded facets, involved and unready
  services) wrapping the same materialization for support and release surfaces.
- **Diff** — `diff_runtime_instances` compares two instances on target identity,
  namespace, and per-service readiness, so parity across claimed target classes
  stays visible.

## Target classes covered

The fixture corpus covers every claimed target class — `local`, `ssh`,
`container`, `devcontainer`, `vm`, and `managed_workspace` — each with an aligned
runtime, plus degraded variants (a partial multi-service stack, a missing mount,
an unpublished port, a pending secret projection) and mismatched variants (a
wrong target and a wrong namespace). See the proof report for the full table.

## Guardrails

- **Identity never collapses.** Local, SSH, container, devcontainer, VM, and
  managed-workspace runtimes keep distinct target, materialization, transport,
  and namespace identities. A runtime that ran on the wrong target is
  `mismatched`, not relabeled.
- **Partial is partial.** A service that is not ready, a missing mount, an
  unpublished port, or a pending secret projection is named and degrades the
  parity rather than being presented as fully up.
- **No secret values.** Secret projection points carry handles only; the runtime
  instance never carries a secret value, raw path, or provider payload.
