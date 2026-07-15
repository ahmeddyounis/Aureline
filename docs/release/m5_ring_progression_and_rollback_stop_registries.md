# M5 ring-progression and rollback-stop registries

This lane governs ring widening by explicit stop conditions rather than schedule optimism, over the frozen
[M5 launch-control matrix](./m5_launch_control_contract.md). It turns the *ring-progression* grammar (how each
widening transition — canary, broad internal dogfood, design-partner preview, public preview, and certified
stable — declares its minimum entry evidence, soak-window expectation, why widening is allowed, its known-limits
packet, issue-template linkage, claim-narrowing action, and the rollback-stop reference that immediately stops
it) and the *rollback-stop* grammar (how a launch-bearing lane records the rollback-stop condition — a crash /
data-loss / trust defect, a repeated protected-metric regression, or a stale readiness packet — that halts ring
progression while it is active) into registry resolvers that produce export-safe, honest projections, so the
shiproom, release-center, executive-steering, program-governance, diagnostics, docs, CLI, support, and
public-proof surfaces resolve one canonical ring-control truth instead of a per-ring, hand-copied mailing list.
The ring-progression rule and the rollback-stop record are separated in runtime and serialized state: the ring
widening transition, minimum entry evidence, soak-window expectation, widening-allow rationale, known-limits
packet, issue-template ref, claim-narrowing action, and rollback-stop reference live on the progression rule,
while the resolved transition identity, active stop-condition ledger, rollback-stop target reference,
protected-metric regression state, packet-freshness state, crash / data-loss / trust reference, and last
ring-transition revision live on the rollback-stop record, and a ring's known-limits and rollback-stop posture
stays visible so a ring never advances while a stop condition is active.

- **Canonical Rust module:** `crates/aureline-ui/src/m5_ring_progression_and_rollback_stop_registries` (the
  authoritative validator).
- **Combined schema:** `schemas/program/m5-ring-progression-and-rollback-stop-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-ring-progression.schema.json`](../../schemas/program/m5-ring-progression.schema.json)
  and
  [`schemas/program/m5-rollback-stop.schema.json`](../../schemas/program/m5-rollback-stop.schema.json)
  as its canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-ring-progression-and-rollback-stop-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-ring-progression-and-rollback-stop-registries/`
  (`ring_progression_beta_narrowed.json`, `rollback_stop_preview_narrowed.json`).

## Two registries

1. **Ring progression** (`resolve_ring_progression_entry`) — publishes one typed ring-progression object per
   widening transition: the ring widening transition and canonical transition mode, the minimum entry evidence,
   the soak-window expectation, the widening-allow rationale, the known-limits packet, the issue-template ref,
   the claim-narrowing action, and the rollback-stop reference. A clean entry names a canonical registry token, a
   classified ring widening transition, and a launch-control role, covers the canonical / accessible / audit
   resolution forms, publishes a complete object, keeps its known-limits and rollback-stop posture visible before
   widening, and keeps a public-facing ring's support language matched to ring proof. Otherwise it degrades
   honestly — a ring advancing without a visible rollback-stop and known-limits posture, or a public-facing ring
   running its support language ahead of proof, degrades to
   `ring_advances_without_rollback_stop_or_runs_support_ahead_of_proof`, the structured blocker reason a
   widen-without-stop attempt must surface.
2. **Rollback stop** (`resolve_rollback_stop_entry`) — keeps the rollback-stop record honest. A clean entry names
   a classified rollback-stop condition and provides the complete transition-identity / active-stop-condition-ledger
   / rollback-stop-target / protected-metric-regression / packet-freshness / crash-data-loss-or-trust /
   last-ring-transition-revision record; a record that would advance a ring while a stop condition is active,
   hide the rollback-stop, or let a protected-metric regression masquerade as covered degrades to
   `rollback_stop_advances_ring_while_active_or_drops_stop_evidence`.

## Per-entry ring reference

The ring widening transition carries its canonical mode, and the resolver publishes the full progression object,
so the registry — never a hand-copied per-ring mailing list — is the single source of truth.
`ring_progression_object_is_complete` rejects an object missing any field,
`ring_states_stop_and_rollback_before_widening` rejects a hidden rollback-stop / known-limits posture or partner
/ public support language running ahead of proof, and `rollback_stop_stays_honest` rejects a record that would
advance a ring while a stop condition is active.

| ring widening transition | transition mode | minimum entry evidence | claim-narrowing action | rollback-stop reference |
| --- | --- | --- | --- | --- |
| canary widening | canary_widening_transition | `repo.rows.core-team-canary-archetypes` | `rollback.target.canary-previous-stable` | `diagnostics.posture.full-telemetry` |
| broad internal dogfood widening | broad_internal_dogfood_widening_transition | `repo.rows.migration-alpha-archetypes` | `rollback.target.migration-previous-toolchain` | `diagnostics.posture.migration-telemetry` |
| certified stable widening | certified_stable_widening_transition | `repo.rows.certified-archetype-archetypes` | `rollback.target.certified-previous-stable` | `diagnostics.posture.certified-telemetry` |

A ring advancing without a visible rollback-stop degrades to
`ring_advances_without_rollback_stop_or_runs_support_ahead_of_proof`, an incomplete object degrades to
`ring_progression_object_incomplete`, and a stop condition left active while a ring advances degrades to
`rollback_stop_advances_ring_while_active_or_drops_stop_evidence`, so a widen-without-stop attempt, an incomplete
object, or an active-but-ignored stop condition can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every ring transition can state why widening is allowed and what immediately stops it.** Clean
  ring-progression entries cover the canonical canary / broad-internal-dogfood / extension-author /
  design-partner-preview / public-preview / certified-stable widening transitions and the first release-center /
  shiproom / executive-steering / program-governance / support surfaces, an object-incomplete example degrades,
  and no clean ring-progression entry published an incomplete object.
- **Known-limits and rollback posture are visible before any ring widens.** A widen-without-stop example and an
  unbound example degrade, a clean stop-and-rollback-visible ring-progression entry is present, and no clean
  entry hides its rollback-stop posture or is unbound.
- **Ring progression cannot advance on a claimed lane when rollback-stop conditions are active.** Clean
  rollback-stop entries cover the crash-data-loss-or-trust-defect / repeated-protected-metric-regression /
  stale-readiness-packet conditions with full resolution-form coverage while providing the complete record — the
  resolved transition identity and the rollback-stop target reference — and a record that would advance a ring
  while a stop condition is active or drop the stop evidence degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- support-export
cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- csv
cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- report
cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- ring-progression-table
cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- fixture-ring-progression-beta-narrowed
cargo run -p aureline-ui --example dump_m5_ring_progression_and_rollback_stop_registries -- fixture-rollback-stop-preview-narrowed
```
