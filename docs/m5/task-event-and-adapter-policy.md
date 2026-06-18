# Task-event and adapter policy

This contract freezes the policy layer that the canonical build/test/task/debug
event envelope depends on, so notebooks, pipeline overlays, coverage and
snapshot/flaky intelligence, the CLI/headless JSON surface, AI evidence, and
support/export all consume one task-event model instead of inferring execution
truth from rendered logs.

It extends — and does not replace — the build/test event interoperability
contract frozen in `schemas/runtime/build-test-event-envelope.schema.json` and
`BuildTestEventInteroperabilityPacket`. It reuses that contract's source-kind,
confidence, lifecycle, retention-class, provenance, severity, and promotion
vocabulary verbatim. What it adds is the explicit **adapter-priority ladder**,
the **raw-payload-retention matrix**, the closed **downgrade vocabulary**, the
**M5 consumer bindings**, and **arbitration rows** that keep a lower-priority
adapter from masquerading as native/BSP/BEP truth.

The stable truth source is `TaskEventAdapterPolicyBaseline` in `aureline-runtime`
(`crates/aureline-runtime/src/m5_task_event_adapter_policy/`). The headless
inspector and regenerator is
`cargo run -p aureline-runtime --example dump_m5_task_event_adapter_policy`.

## Adapter-priority ladder

When more than one adapter can observe the same target and lifecycle kind, the
ladder fixes which emission is authoritative. Lower ranks are higher authority:

| Rank | Source kind | Confidence ceiling | Authoritative | Masquerade blocked |
|------|-------------|--------------------|---------------|--------------------|
| 1 | `native` | `high` | yes | no |
| 2 | `bsp` | `high` | yes | no |
| 3 | `bazel-bep` | `high` | yes | no |
| 4 | `structured-output` | `medium-high` | no | yes |
| 5 | `heuristic-parser` | `low` | yes → no | yes |

Native truth wins over BSP, which wins over Bazel BEP/BES, which wins over
imported structured output, which wins over a heuristic parser fallback. Only
the first three rungs are authoritative; structured-output and heuristic-parser
are never authoritative and must block masquerade. No source kind may assert a
confidence above its ceiling.

## Raw-payload-retention matrix

Every source kind names its allowed retention classes and exactly one default:

- `metadata_digest_only` — metadata and digest only (the safe default for
  `native`, `bsp`, `structured-output`, and `heuristic-parser`).
- `redacted_reference` — a redacted payload retained by reference (the default
  for `bazel-bep`, which carries artifact references).
- `support_approval_required` — support-only payload behind an approval gate;
  every cell in this class sets `approval_required = true`.

A source's default retention class must be allowed and must not require an
approval gate.

## Downgrade vocabulary

Reduced-certainty emissions name exactly one closed reason and stay visibly
downgraded on every consumer projection:

- `partial_support` — the adapter understood the source only partially.
- `heuristic_fallback` — a heuristic parser stood in for a structured or native
  adapter.
- `replay_gap` — replay could not reconstruct the full emission.
- `unsupported_adapter_capability` — an expected negotiated capability was
  unsupported.

## Consumer bindings

The six later M5 execution surfaces — `pipeline`, `coverage`, `snapshot_flaky`,
`notebook_run`, `cli_headless`, and `support_export` — each bind to the
canonical envelope and preserve its source kind, adapter priority rank,
confidence, downgrade reason, and retained raw-payload reference.

## Arbitration

Each arbitration row records one target and lifecycle kind observed by more than
one adapter. The winning emission is the highest-priority (lowest-rank) source
and is never downgraded. Every shadowing emission is strictly lower priority,
shares the winner's `trace_id`, `target_id`, and `event_kind`, and is visibly
downgraded with a downgrade reason and a confidence at or below its own ceiling.

## Stability rules

- The priority ladder must cover every source kind exactly once, in the
  canonical native-first order, with the canonical confidence ceilings and
  authority flags.
- The retention matrix must cover every source kind across all three retention
  classes with exactly one allowed, non-gated default per source.
- The downgrade vocabulary must equal the closed four-reason set.
- All six consumer bindings must be present and preserve envelope truth.
- Arbitration winners must be the highest-priority observed source; shadows must
  be lower priority and visibly downgraded.
- A baseline with any blocker finding is `blocks_stable`; otherwise it is
  `stable`. Raw payload bodies never cross the envelope boundary.

## Companion artifacts

- `schemas/tooling/adapter-capability.schema.json` — boundary schema for the
  policy baseline and its support export.
- `schemas/tooling/task-event-envelope.schema.json` — boundary schema for the
  per-event task-event envelope used in arbitration and replay.
- `artifacts/m5/tooling/event-interop-baseline/` — the checked-in baseline,
  support export, and compact rendering.
- `fixtures/tooling/m5/bsp-bep-native/` — the baseline and the blocking mutation
  cases the typed consumer and the gate replay.
- `tools/ci/m5/task_event_adapter_policy_check.py` — the fail-closed gate.

The typed Rust consumer mints the same baseline, so
`cargo test -p aureline-runtime --test m5_task_event_adapter_policy` enforces the
same structural invariants and that the fixtures are bit-for-bit derivable from
the seed.
