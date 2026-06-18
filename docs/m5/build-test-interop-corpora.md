# Build/test interop conformance corpora

The canonical build/test event interoperability packet
([build-test-event-interoperability](../runtime/m4/build-test-event-interoperability.md))
freezes *one* event envelope that joins native adapters, BSP discovery, Bazel
BEP/BES, structured-output importers (JUnit/SARIF), and problem-matcher /
heuristic parser fallbacks. That packet proves the contract holds **once**. This
lane turns that one-time implementation claim into a continually verified one: it
lands the named **corpora** and the **conformance suite** that re-run the adapter
contract across the claimed M5 tooling archetypes, so adapter drift is measurable
before it destabilizes the test, coverage, notebook, or pipeline surfaces
downstream.

The stable truth source is `InteropConformancePacket` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_interop_conformance/`). The headless inspector
and regenerator is
`cargo run -p aureline-runtime --example dump_m5_interop_conformance`.

## Four named corpora

Each corpus covers one adapter interoperability area, maps to a primary
normalized source kind, and ships its cases under a checked-in fixture directory:

| Corpus family | Source kind | Fixture corpus |
| --- | --- | --- |
| `bsp_discovery` | `bsp` | `fixtures/tooling/m5/bsp-discovery/` |
| `bazel_bep_bes` | `bazel-bep` | `fixtures/tooling/m5/bazel-bep-bes/` |
| `structured_output_junit_sarif` | `structured-output` | `fixtures/tooling/m5/structured-output-junit-sarif/` |
| `problem_matcher_heuristic` | `heuristic-parser` | `fixtures/tooling/m5/problem-matcher-heuristic/` |

Native truth is the implicit baseline every corpus normalizes onto. A missing
corpus family blocks stable, so the interop claim cannot silently shrink to the
families that still happen to pass.

## Corpora run on every claimed archetype

Each claimed M5 tooling archetype declares the corpus families it depends on, and
each corpus must include a case for every archetype that depends on it:

| Archetype | bsp_discovery | bazel_bep_bes | structured_output_junit_sarif | problem_matcher_heuristic |
| --- | :---: | :---: | :---: | :---: |
| `rust_cargo` | | | ✓ | ✓ |
| `node_workspace` | | | ✓ | ✓ |
| `python_pytest` | | | ✓ | ✓ |
| `jvm_build_server` | ✓ | | ✓ | |
| `bazel_monorepo` | ✓ | ✓ | ✓ | |
| `polyglot_ci` | | | ✓ | ✓ |

A corpus that drops an archetype it is supposed to cover blocks stable
(`missing_archetype_coverage`), so a claimed profile that depends on
native/BSP/BEP or importer interoperability can never go unexercised.

## Seven graded conformance dimensions

Every case is graded on the dimensions the docs require, and a case `conforms`
only when **all** pass:

- `capability_negotiation` — the adapter ran a capability handshake.
- `fallback_reason` — a degraded or unsupported capability names an explicit
  fallback reason, and a negotiated capability names none.
- `confidence_preservation` — the normalized confidence does not overclaim its
  source (a heuristic parser, or an explicitly unsupported capability, cannot
  claim more than `low`).
- `raw_payload_retention` — the raw adapter payload is retained behind a
  reference and digest with private material excluded.
- `replay_stability` — the case replays deterministically from canonical
  envelopes.
- `degraded_state_behavior` — a degraded/unsupported capability is visibly
  disclosed.
- `export_parity` — support / release / AI exports preserve source, confidence,
  and refs.

A case that fails any dimension emits a precise blocker finding
(`confidence_overclaim`, `raw_payload_not_retained`, `fallback_reason_missing`,
`degraded_state_not_disclosed`, `replay_unstable`, `export_parity_broken`,
`capability_negotiation_missing`) and blocks stable.

## Freshness narrows aged proof

Every corpus carries a recorded proof age and a freshness window. A corpus whose
proof has aged past its window emits a **warning** (`corpus_evidence_stale`) and
the packet **narrows below stable** rather than blocking — but it cannot stay
green. This is the stale-evidence narrowing the release lane relies on so an
interop claim cannot coast on aged proof.

## Release evidence binding

The derived `release_evidence` binding rolls the corpus results up for release
packets: it records whether every corpus is current, whether every case
conforms, and which families are narrowed (stale or non-conforming). Release
packets ingest this binding to show **current** interop proof instead of one-off
dogfood anecdotes.

## Stability rules

- All four corpus families must be present exactly once.
- Each corpus must be non-empty and cover every archetype that depends on it.
- Each case's source kind must match its corpus family.
- Every case must conform across all seven dimensions.
- A stale corpus narrows below stable (warning); a non-conforming case blocks
  stable (blocker).
- The stored per-case dimension outcomes, per-corpus freshness/conformance
  roll-ups, the corpus digest, and the release-evidence binding must all match
  the derivation; any drift blocks stable.
- A packet with any blocker finding is `blocks_stable`; a packet with only
  warnings is `narrowed_below_stable`; otherwise it is `stable`.

## Companion artifacts

- `schemas/tooling/interop-conformance.schema.json` — boundary schema for the
  packet, its support export, its evidence joins, and the CLI/headless view.
- `schemas/tooling/task-event-envelope.schema.json` — boundary schema for the
  per-event task-event envelope vocabulary this lane reuses.
- `artifacts/m5/tooling/interop-conformance/` — the checked-in packet, support
  export, AI evidence join, incident packet join, CLI/headless view, and compact
  rendering.
- `fixtures/tooling/m5/bsp-discovery/`,
  `fixtures/tooling/m5/bazel-bep-bes/`,
  `fixtures/tooling/m5/structured-output-junit-sarif/`, and
  `fixtures/tooling/m5/problem-matcher-heuristic/` — the named corpora and the
  blocking / narrowing mutation cases the typed consumer and the gate replay.
- `tools/ci/m5/interop_conformance_check.py` — the fail-closed gate.

The typed Rust consumer mints the same packet, so
`cargo test -p aureline-runtime --test m5_interop_conformance` enforces the same
structural invariants and that the fixtures and artifacts are bit-for-bit
derivable from the seed.
