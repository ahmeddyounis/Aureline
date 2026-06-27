# Runbook certification

Aureline markets runbooks as *governed executable guidance*, and six lanes in
`aureline-runbooks` each freeze one slice of that truth and ship one proof artifact:
the governance matrix, the source register, the executable step library, the
execution history, the control-plane handoff register, and the companion-scoped
surface register. This document is the contract for the **certification capstone**:
how every product row that *claims* runbook-backed behavior is bound to those lane
proofs, so a claim only stands while the proof under it is current.

The `m5_runbook_certification` module owns the model and publishes the one
qualification packet every consuming surface reads. The machine-readable inventory
lives at
[`artifacts/runbooks/m5-runbook-certification.json`](../../artifacts/runbooks/m5-runbook-certification.json)
(human summary:
[`artifacts/runbooks/m5-runbook-certification.md`](../../artifacts/runbooks/m5-runbook-certification.md)),
and the schema is
[`schemas/runbooks/m5-runbook-certification.schema.json`](../../schemas/runbooks/m5-runbook-certification.schema.json).
The packet is **derived** from the same checked-in lane proofs the six lanes
publish, so a row's qualification can never claim more than the proof under it
supports.

## Proof lanes and certification facets

Each proof lane covers one **certification facet** — the aspect of runbook truth it
keeps honest. A facet is only certified when at least one lane covers it.

| Facet | Proof lanes | What it certifies |
|-------|-------------|-------------------|
| `source_truth` | `governance`, `sources` | Where each runbook's authority comes from. |
| `step_lineage` | `steps`, `executions` | The executable step taxonomy and its execution / deviation lineage. |
| `boundary_honesty` | `handoffs` | Browser/vendor-console pivots stay attributable handoffs, never hidden escapes. |
| `export_proof` | `companion` | Archived execution history and companion-scoped surfaces export truthfully. |

Every lane contract names the lane's source-of-truth schema, its published register,
its release proof artifact, the owner accountable for it, and a **proof-freshness
state** (`current`, `stale`, or `missing`). Every ref is derived from the lane
itself, so a lane contract can never cite a ref that drifts from the lane it
describes.

## Claimed rows bind the lanes they depend on

Each claimed **incident/operator row** declares the proof lanes it depends on. The
certification derives, per row, the exact coverage gaps, a gate decision, and an
effective claim from those lanes' freshness:

| Lane freshness on a bound lane | Gap | Row gate | Effective claim |
|--------------------------------|-----|----------|-----------------|
| `current` | — | `governed` | the claimed class (e.g. `stable`) |
| `stale` | `proof_stale` | `narrowed` | floored at `beta` |
| `missing` | `proof_missing` | `blocked` | `held` |
| lane not governed at all | `object_mapping_missing` | `blocked` | `held` |

The narrowing is **deterministic**: a stale lane proof always narrows every row that
binds it below Stable, and a missing or unmapped lane proof always blocks them. An
aged proof can never leave a claim standing as implied stable behavior, and the cause
is always named in the row's `gaps` rather than hidden.

## Auto-narrowing in practice

Two drills under
[`fixtures/runbooks/m5-certification-drills/`](../../fixtures/runbooks/m5-certification-drills/)
exercise the gate:

- **stale-proof drill** marks the `handoffs` lane `stale`. Every row that binds
  `handoffs` (operator history, the console-boundary pane, the support bundle, and
  the release gate) auto-narrows to `beta`; rows that do not bind it stay `stable`.
- **missing-proof drill** marks the `companion` lane `missing`. Every row that binds
  `companion` (the companion follow surface, the support bundle, and the release
  gate) is blocked from Stable promotion, with `proof_missing` named on the row.

The packet-level `release_gate` aggregates the per-row gates, so the release/public-
truth automation reads one `blocks_stable_promotion` signal plus the exact blocked,
narrowed, and certified row ids.

## One qualification, every surface

Help/About, the release shiproom, support exports, and the incident/operator
surfaces all consume *this* qualification rather than a private spreadsheet — the
`disclosure` block records that every one of those surfaces reads the same
machine-readable descriptors. The packet carries metadata and refs only: no
credential bodies and no raw provider/console payloads.

The headless emitter is the only mint-from-truth path:

```sh
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- support-export
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- markdown
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- fixture-stale-proof-narrowed
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- fixture-missing-proof-blocked
cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_certification -- validate
```
