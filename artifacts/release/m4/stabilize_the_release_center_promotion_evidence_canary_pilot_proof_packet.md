# Ring Promotion Control Proof Packet — M4

## Scope

This packet proves the checked-in ring promotion control artifact
`artifacts/release/stabilize_the_release_center_promotion_evidence_canary_pilot.json`
is current, governed, and referenced by the stable proof index.

## Evidence

- **Artifact**: `artifacts/release/stabilize_the_release_center_promotion_evidence_canary_pilot.json`
  - Schema version: 1
  - Record kind: `ring_promotion_control`
  - As-of date: 2026-06-02
  - Nine promotion subject rows covering all five subject kinds
  - All nine promotion states exercised
  - All four rings represented
  - All nine gap reasons covered by rules
  - Explicit soak windows for canary→pilot→broad transitions
  - Rollback-stop triggers per subject with ring scope and auto-rollback posture
  - Kill-switch posture tracked per subject

- **Companion doc**: `docs/m4/stabilize-the-release-center-promotion-evidence-canary-pilot.md`

- **Schema**: `schemas/release/stabilize_the_release_center_promotion_evidence_canary_pilot.schema.json`

- **Fixture cases**: `fixtures/release/m4/stabilize_the_release_center_promotion_evidence_canary_pilot/`

## Verification

1. `cargo test -p aureline-release` parses and validates the embedded artifact.
2. The artifact validates with zero violations against the Rust model.
3. Publication decision is `hold` because multiple rows carry active gaps, incomplete soaks, or armed kill switches.
4. Every subject kind appears at least once.
5. Every gap reason has a rule watching for it.
6. No row renders wider than its claim ceiling.
7. Rollback targets are at or narrower than current rings for every row.

## One-Build Identity

The artifact references:
- `artifacts/release/stable_claim_manifest.json` (canonical claim ceiling)
- `artifacts/release/stable_proof_index.json` (proof index)

It does not carry raw artifacts, logs, signatures, or credential material.

## Soak Windows

| Transition | Minimum Hours | Required Evidence | Status |
|---|---|---|---|
| internal→canary | 24 | ci:smoke_tests, fitness:startup_p50 | Complete |
| canary→pilot | 72 | ci:integration_tests, ci:compatibility_matrix, fitness:typing_p95 | Complete |
| pilot→broad | 168 | ci:full_regression, ci:benchmark_lab, fitness:search_p95 | Complete |
| pilot→broad (AI provider pack) | 168 | ci:provider_fallback_drill, ci:provider_health_check, fitness:provider_latency_p95 | In progress |
| canary→pilot (AI tool pack) | 72 | ci:tool_schema_validation | In progress |

## Rollback-Stop Triggers

| Trigger | Ring Scope | Auto-Rollback | Subjects |
|---|---|---|---|
| Crash | canary, pilot, broad | Yes | binary_artifact_graph |
| Performance regression | pilot, broad | No | binary_artifact_graph |
| Data loss | pilot, broad | Yes | ai_provider_pack |
| Trust regression | canary, pilot, broad | Yes | ai_model_pack |
| Protected path regression | canary, pilot | Yes | binary_artifact_graph_canary |
| Compatibility failure | pilot, broad | Yes | ai_prompt_pack |

## Kill-Switch Posture

| Subject | Posture | Rollback Target |
|---|---|---|
| binary_artifact_graph | disabled | pilot |
| ai_provider_pack | armed | canary |
| ai_model_pack | enabled | internal |
| ai_prompt_pack | disabled | internal |
| ai_tool_pack | disabled | internal |
| binary_artifact_graph_canary | armed | internal |
| ai_model_pack_waiver | disabled | canary |
| ai_prompt_pack_rollback | enabled | canary |
| ai_tool_pack_waiver | disabled | internal |

## Owner Sign-Off

- Owner: `release_engineering`
- Signed off: 2026-06-02

## Waiver Status

- `waiver:tool_schema_validation_pending` — active, expires 2026-07-01
- `waiver:model_pack_quantization_pending` — expired 2026-05-01

## Downgrade Behavior

Any subject whose proof packet ages out, whose soak window remains incomplete,
whose waiver expires, whose regression trigger fires, or whose kill switch
engages is structurally narrowed below the stable cutline. The artifact
automatically reflects this narrowing in `effective_label` and `promotion_state`;
shiproom and release tooling fail promotion directly from the register.
