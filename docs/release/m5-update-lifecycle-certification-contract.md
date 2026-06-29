# M5 update / support-lifecycle certification contract

This contract freezes the typed certification that qualifies every **claimed M5 channel and
deployment profile** against the update / support-lifecycle contract and **narrows or blocks the
claim deterministically** when the backing proof is stale, expired, or missing — instead of letting
a channel keep a generic Stable promise behind drifted evidence. It is the qualification layer over
the [update / support-lifecycle governance matrix](m5-update-lifecycle-contract.md): the matrix
freezes the governed facets and certifies the *consumer surfaces* that read them; this certification
projects those facets onto the claimed **channel × profile grid** and says whether each pair maps to
fresh proof.

The certification is a **pure projection of the governance matrix** — it carries no parallel,
hand-maintained channel inventory. Release, update-center, Help/About, docs/help, support exports,
and shiproom read this one certification rather than each restating the channel matrix.

- Packet schema: [`schemas/release/m5-update-lifecycle-certification.schema.json`](../../schemas/release/m5-update-lifecycle-certification.schema.json)
- Published inventory: [`artifacts/release/m5-update-lifecycle-certification.json`](../../artifacts/release/m5-update-lifecycle-certification.json)
- Rendered certification document: [`artifacts/release/m5-update-lifecycle-certification.md`](../../artifacts/release/m5-update-lifecycle-certification.md)
- Machine-readable grid export: [`artifacts/release/m5-update-lifecycle-certification.csv`](../../artifacts/release/m5-update-lifecycle-certification.csv)
- Release-grade parity proof: `artifacts/release/m5-update-lifecycle-proof/update-lifecycle-certification.json` (+ `.md`)
- Per-state fixtures: `fixtures/release/m5-update-lifecycle-certification/`
- Producer crate / module: `crates/aureline-release` → `m5_update_lifecycle_certification`
- Headless emitter: `aureline_release_m5_update_lifecycle_certification`

## What the certification qualifies

The unit of certification is a **claimed (`channel`, `profile`) pair** — every M5 release channel
(`stable`, `beta`, `preview`, `nightly`, `lts`) on every deployment profile (`managed`,
`self_hosted`). Each pair carries a claimed [qualification class](../../crates/aureline-release/src/m5_descriptor_badge)
it wants to keep (`stable` → Stable, `beta` → Beta, `preview` → Preview, `nightly` → Experimental,
`lts` → Stable) and is qualified along four **proof dimensions**, each drawn straight from the
governed facets so the certification reuses the matrix's facet proofs rather than restating them:

| Dimension | Backing facets |
|---|---|
| `update_communication` | update availability, change impact, release-note evidence |
| `migration_guidance` | migration assistant |
| `lifecycle_windows` | support window, compatibility window, end-of-support |
| `stale_data_behavior` | service health |

## How a cell's outcome is derived

For one (`channel`, `profile`, `dimension`) tuple, the certification gathers the governed facets
that back the dimension **and** scope to that channel and profile:

- a dimension **no governed facet covers** for the pair is honestly labeled `not_applicable` rather
  than a hidden gap, and is excluded from the claim's gate (so `preview` and `nightly`, which have no
  support-window or migration coverage, read `n/a` there instead of failing);
- otherwise the cell takes the **worst** proof freshness and the **worst** lifecycle-state gate among
  the covering facets, so a cell can never make a downgraded copy read as live. The cell's gate is
  `worst(freshness_gate, lifecycle_gate)`: `current` / governed → `governed`; `stale` or a narrowing
  lifecycle state → `narrowed`; `expired` / `missing` or a blocking lifecycle state → `blocked`. The
  named `gap_kind` (`proof_stale`, `proof_expired`, `proof_missing`, `lifecycle_state_narrowed`,
  `lifecycle_state_blocked`) records *why* the cell could not stand.

A claim's gate is the **worst of its applicable cells**, and its effective qualification is the
claimed class narrowed down that gate: `governed` keeps the claim, `narrowed` floors it at Beta,
`blocked` floors it at Unavailable. This is the lane's guardrail against over-stating a channel:
`M5UpdateLifecycleCertification::validate` re-derives every cell, claim, consumer, the summary, and
the release gate from the cells and rejects any stored verdict that is less severe than its evidence
warrants (`cell_outcome_drift`, `claim_verdict_drift`, `consumer_verdict_drift`).

## How consumers read the certification

Each [consumer](../../crates/aureline-release/src/m5_update_lifecycle_certification) binds the
dimensions it surfaces and **derives** its posture and the exact channel/profile pairs it must narrow
or block from the grid — there is no hand-maintained per-consumer status. `release_center`,
`support_export`, and `shiproom` surface every dimension; `update_center`, `help_about`, and
`docs_help` surface the subset they show. So a missing proof blocks only the surfaces that depend on
it: when the service-health proof is missing, the five consumers that surface stale-data behavior
block while `docs_help` — which does not — stays certified.

## Narrowing is per claim, not behind a generic stable label

Because each pair is qualified independently, a stale facet narrows **only the channels it scopes
to**. The drills make this concrete:

- `fixtures/release/m5-update-lifecycle-certification/certification_stale_proof_narrowed.json` — the
  governance matrix's change-impact proof is stale. Change-impact backs `update_communication` and
  scopes to `stable` / `beta` / `preview` / `nightly`, so those eight claims narrow while the two
  `lts` claims — outside the change-impact scope — stay certified.
- `fixtures/release/m5-update-lifecycle-certification/certification_missing_proof_blocked.json` — the
  governance matrix's service-health proof is missing. Service-health backs `stale_data_behavior` and
  scopes to every channel, so every claim blocks on that dimension and Stable promotion is held; the
  consumers that surface stale-data behavior block while `docs_help` stays certified.
- `fixtures/release/m5-update-lifecycle-certification/certification_all_certified.json` — the
  all-current matrix, so every claim stands at its claimed qualification.

## Export safety

The packet carries metadata, refs, and message ids only — no credential bodies or raw provider
payloads — so the certification truth is exportable and reviewable outside the app. The JSON, the
certification document, the compact proof report, and the per-cell CSV all render byte-identically
across the desktop, CLI / headless, and offline-export channels.

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- registry > artifacts/release/m5-update-lifecycle-certification.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- document > artifacts/release/m5-update-lifecycle-certification.md
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- csv      > artifacts/release/m5-update-lifecycle-certification.csv
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- proof    > artifacts/release/m5-update-lifecycle-proof/update-lifecycle-certification.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- markdown > artifacts/release/m5-update-lifecycle-proof/update-lifecycle-certification.md
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- variant all-certified   > fixtures/release/m5-update-lifecycle-certification/certification_all_certified.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- variant stale-narrowed  > fixtures/release/m5-update-lifecycle-certification/certification_stale_proof_narrowed.json
cargo run -q -p aureline-release --bin aureline_release_m5_update_lifecycle_certification -- variant missing-blocked > fixtures/release/m5-update-lifecycle-certification/certification_missing_proof_blocked.json
```
