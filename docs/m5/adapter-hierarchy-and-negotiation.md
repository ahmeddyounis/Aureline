# Adapter hierarchy and negotiation

This contract makes the *choice* of adapter a governed product object. For every
claimed build/test ecosystem it records which adapter produced execution truth,
walks the native-first ladder in priority order, and keeps an explicit
fallback-reason packet that names why each higher-priority adapter was skipped.
Unsupported capabilities on the resolved adapter stay named rather than inferred
from missing rows, and capability drift is surfaced before it can quietly degrade
trust in tests, coverage, pipelines, or incident flows.

It extends — and does not replace — the frozen adapter policy in
`schemas/tooling/adapter-capability.schema.json` and `TaskEventAdapterPolicyBaseline`.
It reuses that contract's source-kind, confidence, capability-state, severity, and
promotion vocabulary and the per-event envelope frozen in
`schemas/tooling/task-event-envelope.schema.json`. What it adds is the
**per-ecosystem candidate ladder**, the **explicit fallback-reason packet**, the
**named unsupported-capability set**, the **capability-drift signals**, and the
**disclosure-surface bindings** that expose the outcome to UI, CLI/headless, AI
evidence, and support/export.

The stable truth source is `AdapterNegotiationBaseline` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_adapter_hierarchy_negotiation/`). The headless
inspector and regenerator is
`cargo run -p aureline-runtime --example dump_m5_adapter_hierarchy_negotiation`.

## Ordered adapter resolution

Each resolution walks the full ladder (`native`, `bsp`, `bazel-bep`,
`structured-output`, `heuristic-parser`) in priority order and selects the
highest-priority **eligible** candidate. A candidate is eligible when its adapter
is available, its capability handshake did not fail, and it can negotiate at least
one usable capability. The seed shows each ecosystem landing on a different rung:

| Ecosystem | Selected source | Fallback class | Confidence | Downgraded |
|-----------|-----------------|----------------|------------|------------|
| `cargo` | `native` | `native_authoritative` | `high` | no |
| `gradle_jvm` | `bsp` | `negotiated_protocol` | `high` | no |
| `bazel` | `bazel-bep` | `negotiated_protocol` | `high` | no |
| `python_pytest` | `structured-output` | `structured_import` | `medium-high` | yes |
| `node_js` | `structured-output` | `structured_import` | `medium-high` | yes |
| `generic` | `heuristic-parser` | `heuristic_last_resort` | `low` | yes |

Native and negotiated-protocol (BSP, Bazel BEP/BES) truth is authoritative and is
never downgraded. Structured-output and heuristic-parser resolutions are never
authoritative and are always visibly downgraded — `structured_import` with
`partial_support`, `heuristic_last_resort` with `heuristic_fallback` — so users can
distinguish them from native/BSP/BEP truth. No resolution may assert a confidence
above its source's ceiling.

A lower-priority adapter must never displace a higher-priority one that was
available and could negotiate a usable capability. If a higher rung is eligible yet
skipped, the baseline blocks stable.

## Explicit fallback-reason packet

Every higher rung that the resolution passed over carries a closed-vocabulary skip
reason, and `fallback_reasons` re-states them so CLI/headless, AI evidence, and
support/export can read the reason without re-deriving it from the ladder:

- `adapter_unavailable` — the adapter is not installed or reachable.
- `ecosystem_unsupported` — the source kind does not apply to this ecosystem.
- `capability_unsupported` — the adapter is reachable but negotiated no usable
  capability.
- `negotiation_failed` — the adapter is reachable but the capability handshake
  failed.

`adapter_unavailable` and `ecosystem_unsupported` require an unavailable candidate;
`negotiation_failed` requires a reachable candidate whose handshake failed;
`capability_unsupported` requires a reachable candidate whose every capability is
unsupported. The fallback-reason packet must match the ladder exactly.

## Named unsupported capabilities

The six negotiated capabilities — `target_graph`, `lifecycle_events`,
`diagnostics`, `test_events`, `artifacts`, `progress` — are disclosed on the
selected adapter with an explicit `negotiated`, `degraded`, or `unsupported` state.
A capability the adapter cannot serve stays named in `unsupported_capabilities`
rather than being dropped from the row set, so missing support never reads as a
silent gap.

## Capability drift

Each drift signal records an ecosystem, source kind, optional capability, a closed
drift class (`capability_lost`, `capability_degraded`, `confidence_regressed`,
`fallback_deepened`, `adapter_unavailable`), and the prior and current state. Every
signal must set `visible_before_trust_loss = true`, encoding that adapter
capability drift is surfaced before it degrades trust.

## Disclosure surfaces

All four consumer surfaces — `ui`, `cli_headless`, `ai_evidence`, and
`support_export` — bind to the negotiation outcome and disclose the selected source
kind, the fallback reason, the unsupported capabilities, the capability drift, and
the confidence.

## Stability rules

- The resolutions must cover every ecosystem exactly once.
- Each candidate ladder must cover every source kind once in the canonical
  native-first order with the canonical ranks.
- Each resolution must select exactly one eligible candidate, and that candidate
  must be the highest-priority eligible rung.
- Every higher skipped rung must carry a consistent, closed-vocabulary skip reason,
  and the fallback-reason packet must match the ladder.
- The fallback class, confidence ceiling, and downgrade posture must match the
  selected source kind.
- The selected adapter must disclose all six capabilities, and the named
  unsupported set must equal the derived set.
- Every drift signal must be visible before trust loss, and all four disclosure
  surfaces must be present and preserve negotiation truth.
- A baseline with any blocker finding is `blocks_stable`; otherwise it is `stable`.

## Companion artifacts

- `schemas/tooling/adapter-negotiation.schema.json` — boundary schema for the
  negotiation baseline and its support export.
- `schemas/tooling/task-event-envelope.schema.json` — boundary schema for the
  per-event task-event envelope this lane resolves adapters for.
- `artifacts/m5/tooling/adapter-negotiation/` — the checked-in baseline, support
  export, and compact rendering.
- `fixtures/tooling/m5/bsp-bep-heuristic-fallbacks/` — the baseline and the
  blocking mutation cases the typed consumer and the gate replay.
- `tools/ci/m5/adapter_hierarchy_negotiation_check.py` — the fail-closed gate.

The typed Rust consumer mints the same baseline, so
`cargo test -p aureline-runtime --test m5_adapter_hierarchy_negotiation` enforces
the same structural invariants and that the fixtures are bit-for-bit derivable from
the seed.
