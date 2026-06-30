# M5 assurance certification contract

This contract freezes the typed certification that qualifies every **claimed M5 deployment profile**
— `managed`, `self_hosted`, `regulated`, and `sovereign` — against the assurance / governance /
boundary-route / event-provenance contract and **narrows or blocks the profile claim
deterministically** when the backing proof is stale, drifting, or missing — instead of letting a
regulated or self-hosted profile keep a generic trust badge behind drifted evidence. It is the
qualification layer over the [assurance / governance / route-provenance governance
matrix](m5-assurance-route-governance-contract.md): the matrix freezes the nine governed facets and
certifies the *product surfaces* that read them; this certification projects those facets onto the
claimed **profile grid** and says whether each profile maps to fresh proof for assurance, governance,
and route / provenance explainability.

The certification is a **pure projection of the governance matrix** — it carries no parallel,
hand-maintained profile inventory. The release center, About / help, shiproom, support export, and
the procurement / evaluation pack read this one certification result rather than each maintaining a
local trust override.

- Packet schema: [`schemas/public-truth/m5-assurance-certification.schema.json`](../../schemas/public-truth/m5-assurance-certification.schema.json)
- Published inventory: [`artifacts/public-truth/m5-assurance-certification.json`](../../artifacts/public-truth/m5-assurance-certification.json)
- Rendered certification document: [`artifacts/public-truth/m5-assurance-certification.md`](../../artifacts/public-truth/m5-assurance-certification.md)
- Machine-readable grid export: [`artifacts/public-truth/m5-assurance-certification-grid.csv`](../../artifacts/public-truth/m5-assurance-certification-grid.csv)
- Release-grade parity proof: `artifacts/release/m5-assurance-certification-proof/certification.json` (+ `.md`)
- Per-state fixtures: `fixtures/public-truth/m5-assurance-certification/`
- Producer crate / module: `crates/aureline-release` → `m5_assurance_certification`
- Headless emitter: `aureline_release_m5_assurance_certification`

## What the certification qualifies

The unit of certification is a **claimed deployment profile** — every M5 posture the matrix already
claims (`managed`, `self_hosted`, `regulated`, `sovereign`). Each profile carries a claimed
[qualification class](../../crates/aureline-release/src/m5_descriptor_badge) of `stable` — the
assurance posture it wants to keep — and is qualified along four **proof dimensions**, each grouping
the governed facets so the certification reuses the matrix's facet proofs rather than restating them:

| Dimension | Backing facets |
|---|---|
| `assurance_center` | assurance claim, control proof, exception / waiver |
| `governance` | governance freshness, service ownership |
| `boundary_route` | capability boundary, route hop, approval ticket |
| `event_provenance` | event provenance |

## How a cell's outcome is derived

For one (`profile`, `dimension`) tuple, the certification gathers the governed facets that back the
dimension **and** scope to that profile:

- a dimension **no governed facet covers** for the profile is honestly labeled `not_applicable`
  rather than a hidden gap, and is excluded from the profile's gate. A facet that does not scope to a
  profile is simply dropped from that profile's backing set rather than overstated: the exception /
  waiver and approval-ticket facets do not scope to a `sovereign` / air-gapped deployment, so a
  sovereign profile's `assurance_center` cell is backed by the assurance-claim and control-proof
  facets only, and its `boundary_route` cell by capability-boundary and route-hop only — never an
  approval ticket it does not have;
- otherwise the cell takes the **worst** proof freshness and the **worst** assurance-state gate among
  the covering facets, so a cell can never make a narrowed control or stale proof read as fully
  proven. The cell's gate is `worst(freshness_gate, state_gate)`: `current` / governed → `governed`;
  `stale` or a narrowing assurance state → `narrowed`; `expired` / `missing` or a blocking assurance
  state → `blocked`. The named `gap_kind` reuses the governance matrix's frozen vocabulary
  (`proof_stale`, `proof_expired`, `proof_missing`, `assurance_state_narrowed`,
  `assurance_state_blocked`) and records *why* the cell could not stand, alongside the
  `worst_state_token` naming the precondition that drifted.

A profile's gate is the **worst of its applicable cells**, and its effective qualification is the
claimed class narrowed down that gate: `governed` keeps the Stable claim, `narrowed` floors it at
Beta, `blocked` floors it at Unavailable. This is the lane's guardrail against over-stating a
profile: `M5AssuranceCertification::validate` re-derives every cell, profile, consumer, the summary,
and the release gate from the cells and rejects any stored verdict that is less severe than its
evidence warrants (`cell_outcome_drift`, `claim_verdict_drift`, `consumer_verdict_drift`).

## How consumers read the certification

Each [consumer](../../crates/aureline-release/src/m5_assurance_certification) binds the dimensions it
surfaces and **derives** its posture and the exact profile claims it must narrow or block from the
grid — there is no hand-maintained per-consumer status. `release_center`, `shiproom`, and
`procurement_evaluation` surface every dimension; `help_about` surfaces the assurance, governance,
and boundary / route story (not the deep event-provenance ledger); `support_export` surfaces the
assurance, boundary / route, and event-provenance lineage a field investigation needs (not the
governance-freshness dashboard). So a missing proof blocks only the surfaces that depend on it: when
the event-provenance proof is missing, the four consumers that surface event provenance block while
`help_about` — which does not — stays certified.

## Narrowing is per claim, not behind a generic trust badge

Because each profile is qualified independently and per dimension, a stale facet narrows **only the
dimension it backs**, and the other dimensions keep standing. The drills make this concrete:

- `fixtures/public-truth/m5-assurance-certification/certification_stale_proof_narrowed.json` — the
  governance matrix's route-hop proof is stale. Route-hop backs `boundary_route`, so every profile
  narrows on the boundary / route dimension while its assurance, governance, and event-provenance
  dimensions stay certified; each profile's effective qualification floors at Beta and Stable
  promotion is not held.
- `fixtures/public-truth/m5-assurance-certification/certification_missing_proof_blocked.json` — the
  governance matrix's event-provenance proof is missing. Event-provenance backs `event_provenance`
  and scopes to every profile, so every profile blocks on that dimension and Stable promotion is
  held; the consumers that surface event provenance block while `help_about` stays certified.
- `fixtures/public-truth/m5-assurance-certification/certification_all_certified.json` — the
  all-current matrix, so every claimed profile stands at its claimed Stable qualification.

## Export safety

The packet carries metadata, refs, and message ids only — no credential bodies or raw provider
payloads — so the certification truth is exportable and reviewable outside the app. Every applicable
cell reduces its backing facets to refs-only proof paths, preserving owner / freshness / route
lineage without leaking secrets (`export_preserves_route_evidence_lineage`). The JSON, the
certification document, the compact proof report, and the per-cell CSV all render byte-identically
across the desktop, CLI / headless, and offline-export channels.

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- registry > artifacts/public-truth/m5-assurance-certification.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- document > artifacts/public-truth/m5-assurance-certification.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- csv      > artifacts/public-truth/m5-assurance-certification-grid.csv
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- registry > artifacts/release/m5-assurance-certification-proof/certification.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- markdown > artifacts/release/m5-assurance-certification-proof/certification.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- variant all-certified   > fixtures/public-truth/m5-assurance-certification/certification_all_certified.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- variant stale-narrowed  > fixtures/public-truth/m5-assurance-certification/certification_stale_proof_narrowed.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_certification -- variant missing-blocked > fixtures/public-truth/m5-assurance-certification/certification_missing_proof_blocked.json
```
