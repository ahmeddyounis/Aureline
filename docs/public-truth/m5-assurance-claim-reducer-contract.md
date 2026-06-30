# M5 assurance-claim reducer contract

This contract freezes the assurance-claim reducer: the one source of truth that **automatically
narrows** Aureline's regulated, self-hosted, sovereign, no-vendor, no-telemetry, and
customer-managed-key claims the moment a precondition behind them drifts, and drives every consumer
of those claims from that one output. It sits beside the
[assurance center](m5-assurance-center-contract.md): the assurance center derives each claim card's
state from the control proofs backing it; this lane closes the loop between the *condition* and the
*consumers*, reducing each claim to the single weakest state today's trust facts allow and projecting
that one state onto every surface that states the claim.

It does **not** invent new compliance frameworks or sell-sheet families, and it does not restate
control proof as marketing copy. Every weaker claim state is *derived* from a named precondition
drift, so a claim can never read stronger than the trust facts behind it, and a claim narrowed in one
consumer can never read stronger in another.

- Packet schema: [`schemas/public-truth/m5-assurance-claim-reducer.schema.json`](../../schemas/public-truth/m5-assurance-claim-reducer.schema.json)
- Published inventory: [`artifacts/public-truth/m5-assurance-claim-reducer.json`](../../artifacts/public-truth/m5-assurance-claim-reducer.json)
- Rendered overview: [`artifacts/public-truth/m5-assurance-claim-reducer.md`](../../artifacts/public-truth/m5-assurance-claim-reducer.md)
- Machine-readable claim / precondition matrix: [`artifacts/public-truth/m5-assurance-claim-reducer-claims.csv`](../../artifacts/public-truth/m5-assurance-claim-reducer-claims.csv)
- Release-grade narrowing proof: `artifacts/public-truth/m5-assurance-narrowing-proof/assurance-claim-reducer.json` (+ `.md`)
- Exported redaction-safe preview: `artifacts/public-truth/m5-assurance-narrowing-proof/export-preview.json`
- Per-state fixtures: `fixtures/public-truth/assurance-claim-narrowing/`
- Producer crate / module: `crates/aureline-release` → `m5_assurance_claim_reducer`
- Headless emitter: `aureline_release_m5_assurance_claim_reducer`

## What the reducer holds

The packet is minted from one input by the headless emitter — each precondition's current status —
so the in-code packet, the published artifacts, and the fixtures can never drift.

### 1. Preconditions

Each regulated claim depends on a subset of four trust preconditions. The set is exactly the four
drift dimensions the exit-gate names, framed as the *condition that must hold*:

| Precondition | Must hold | Drift narrows | Drift blocks |
|--------------|-----------|---------------|--------------|
| `evidence_freshness` | the supporting evidence is fresh | `stale_evidence` | `evidence_expired` |
| `hosted_dependency_boundary` | no hosted / vendor dependency crossed the boundary | `hosted_dependency_drift` | `boundary_dependency_added` |
| `key_residency` | keys and data stay pinned to the customer-owned residency | `key_residency_drift` | `key_residency_mismatch` |
| `policy_control_path` | the required policy / control path is intact | `policy_path_degraded` | `policy_path_regression` |

Each precondition status binds to one gate: `satisfied` → governed, `drifted` → narrowed,
`invalidated` → blocked.

### 2. Reduced claims

One per claim subject. A reduced claim never asserts a fixed state; it derives its `reduced_state`
from the **worst gate** among its required preconditions:

- all preconditions satisfied → `proven` (governed, Stable);
- a drifted precondition → `under_review` (narrowed, Beta);
- an invalidated precondition → `unproven` (blocked, Unavailable, holds Stable promotion).

Every weaker state records its `drifts` — which precondition drifted, the named drift token, the gate
it inflicts, and the `restoration_action` that would lift it. When a claim is not fully governed it
carries a `nearest_truthful` fallback: the weaker posture that is still true and the strongest state
that statement can carry (never `proven`), so the product states the nearest truthful current claim
instead of the one that no longer holds.

### 3. Consumer projections

Every reduced claim carries one `consumer_projection` per consumer surface — the About / help panel,
the assurance center, the exported evaluation packet, the procurement export, and the
release / public-truth manifest. Each projection reads the **same** reduced state and qualification;
`converges_with_reduced` is the proof. This is the guardrail: one reducer output governs all claimed
surfaces, so a claim narrowed in one consumer can never be silently strengthened in another.

## How drift narrows or blocks a claim

The reducer takes the deployment-wide precondition statuses and, for each claim, takes the worst gate
among the preconditions that claim depends on. A `drifted` precondition narrows every dependent
claim; an `invalidated` precondition blocks every dependent claim. A claim that depends on none of the
drifted preconditions stays proven. The drift is recorded against the precondition that changed, so
the change stays attributable to an evidence / boundary / residency / policy fact rather than a manual
copy edit.

## Same grammar in product and in export

The exported `export_preview` reduces each claim to its subject, reduced state, qualification, the
named drift tokens, the nearest truthful fallback, and the evidence refs — drawn from the same
controlled vocabulary the in-product reduced claims show, so an offline procurement / evaluator review
and the live surfaces can never read differently.

## Export safety

The packet is metadata-only. It preserves drift and evidence lineage as repo-relative refs and
carries no credential bodies or raw provider payloads; the `export_carries_no_raw_material`
conformance flag scans the serialized export for forbidden material.

## Regenerating

Run the headless emitter and write its output to the checked-in artifacts:

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- registry  > artifacts/public-truth/m5-assurance-claim-reducer.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- overview  > artifacts/public-truth/m5-assurance-claim-reducer.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- csv       > artifacts/public-truth/m5-assurance-claim-reducer-claims.csv
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- registry  > artifacts/public-truth/m5-assurance-narrowing-proof/assurance-claim-reducer.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- markdown  > artifacts/public-truth/m5-assurance-narrowing-proof/assurance-claim-reducer.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_claim_reducer -- export    > artifacts/public-truth/m5-assurance-narrowing-proof/export-preview.json
```

The per-state fixtures under `fixtures/public-truth/assurance-claim-narrowing/` are minted from the
`variant` subcommand (`canonical`, `stale-evidence`, `hosted-dependency`, `key-residency`,
`policy-path`).
