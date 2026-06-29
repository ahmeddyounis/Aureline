# M5 governance dashboard contract

This contract freezes the operator / admin / evaluator-facing governance dashboard: the surface that
turns Aureline's protected fitness functions, nightly governance runs, accepted waivers, service
ownership, and decision rights into inspectable product truth. It sits beside the
[assurance center](m5-assurance-center-contract.md): the assurance center reads "what does Aureline
claim and what proves it"; this dashboard reads the governance layer next to it — "which protected
fitness functions are passing right now, what is held under an accepted waiver, when does that waiver
expire, who owns each governed service, and who decides each governed change?"

It does **not** invent new fitness functions, mint a second waiver or ownership system, or restate
governance state as marketing copy. Every tile and card derives its state from the nightly inputs
backing it, so a tile can never read greener than its proof, and a waived or stale item never renders
as a clean pass.

- Packet schema: [`schemas/public-truth/m5-governance-dashboard.schema.json`](../../schemas/public-truth/m5-governance-dashboard.schema.json)
- Component schemas (validatable on their own):
  [`m5-fitness-tile.schema.json`](../../schemas/public-truth/m5-fitness-tile.schema.json),
  [`m5-waiver-queue-row.schema.json`](../../schemas/public-truth/m5-waiver-queue-row.schema.json),
  [`m5-decision-right-card.schema.json`](../../schemas/public-truth/m5-decision-right-card.schema.json)
- Published inventory: [`artifacts/public-truth/m5-governance-dashboard.json`](../../artifacts/public-truth/m5-governance-dashboard.json)
- Rendered overview: [`artifacts/public-truth/m5-governance-dashboard.md`](../../artifacts/public-truth/m5-governance-dashboard.md)
- Machine-readable fitness-tile matrix: [`artifacts/public-truth/m5-governance-dashboard-tiles.csv`](../../artifacts/public-truth/m5-governance-dashboard-tiles.csv)
- Release-grade parity proof: `artifacts/public-truth/m5-governance-dashboard-proof/governance-dashboard.json` (+ `.md`)
- Exported evaluation packet: `artifacts/public-truth/m5-governance-dashboard-proof/evaluation-packet.json`
- Per-state fixtures: `fixtures/public-truth/m5-governance-dashboard/`
- Producer crate / module: `crates/aureline-release` → `m5_governance_dashboard`
- Headless emitter: `aureline_release_m5_governance_dashboard`

## What the dashboard holds

The packet has five product parts, all minted from one source by the headless emitter — each fitness
function's measured nightly result, evidence freshness, run metadata, and (when present) an accepted
waiver and its expiry standing — so the in-code packet, the published artifacts, and the fixtures can
never drift. The whole packet is stamped with one `corpus_id` so a pass measured against one corpus
or deployment profile can never be read as a pass in another context.

### 1. Freshness-aware fitness tiles

One tile per protected fitness function. A tile never asserts a fixed colour; it **derives** its
state from the measured result, evidence freshness, and waiver standing, distinguishing six
colour-distinct states rather than flattening them into one pass / fail colour:

| State | Gate | Signal | Cause |
|-------|------|--------|-------|
| `passing` | `governed` | green | fresh evidence, measured pass, no waiver |
| `warning` | `narrowed` | yellow | a warning-threshold breach |
| `evidence_stale` | `narrowed` | yellow | the evidence is stale, so the result is no longer trusted |
| `waived` | `narrowed` | yellow | a failing / warning function held under an in-date waiver |
| `waiver_expired` | `blocked` | red | the waiver that held a failing function has expired |
| `blocked` | `blocked` | red | a hard fail, or expired / missing evidence |

The seven fitness functions are `package_boundary_integrity`, `protected_path_review`,
`schema_example_parity`, `evidence_freshness_slo`, `claim_no_overclaim`, `route_explainability`, and
`provenance_completeness`. Each is owned by one service, required under one weakest deployment
profile, and bound to one evidence class and a repo-relative proof ref drawn from the nightly
governance feed — never a parallel evidence family.

### 2. Nightly governance rows

One run record per fitness function: the last run timestamp, the state that run read, the measured
result and freshness, and the consecutive-passing-run streak. A nightly row reads the same proof ref
and derived state its tile reads, so the run log and the board never disagree.

### 3. Waiver-expiry queue

The accepted waivers, ordered by expiry urgency (expired first, then expiring-soon, then active).
Each row discloses the expiry date, a rationale (no secrets / no private incident content), the
responsible party, the action that clears it, and the governance ticket it rides. An in-date waiver
narrows its tile to `waived`; an expired waiver blocks it to `waiver_expired`. A waiver may only be
attached to a function that is not already a clean pass.

### 4. Service-ownership cards

One card per governed service (`package_governance`, `evidence_pipeline`, `claim_publication`,
`route_provenance`): its accountable owner role, decision forum, the fitness functions it owns, its
worst tile state, and its open / expired waiver counts. Ownership and forum are roles, never
individuals.

### 5. Decision-right cards

One card per governed decision right (`stable_promotion`, `waiver_acceptance`, `boundary_change`,
`exception_renewal`): the forum that exercises it, the accountable owner, the services it governs,
and whether the decision is currently `clear` (exercisable), `watch` (needs review), or `held` (its
scope is blocked). A narrowly scoped decision like `boundary_change` stays `clear` even when an
out-of-scope function blocks, so authority visibility never over-blocks.

## Per-profile overviews and corpus binding

One overview per deployment profile (`managed`, `self_hosted`, `regulated`, `sovereign`) summarises
the applicable functions' tile-state and freshness counts, the open / expired waiver counts, the
gate decision, and the strongest **honored** posture — which auto-narrows below the profile the
moment a function it would imply is not passing, and never reads above the profile. Every overview,
tile, nightly row, and the exported evaluation packet carries the packet's `corpus_id`, so a
reviewer can never overgeneralise one corpus's pass into another context.

## Exported evaluation packet

The packet carries an [`evaluation_packet`] export that reduces the tiles, waivers, service cards,
and decision cards to the exact state and proof vocabulary the in-product dashboard shows, so an
exported evaluation pack and the live UI can never read differently. The export is metadata-only: it
preserves proof lineage as refs and carries no credential bodies or raw provider payloads. The same
release, Help/About, support, and evaluation surfaces can validate a single tile, waiver row, or
decision card against the three component schemas above.

## Gate behaviour

- Every tile binds an evidence class, owner role, forum, and proof ref.
- Stale evidence narrows the tiles that read it deterministically.
- Missing / expired evidence and expired waivers block Stable promotion (`blocks_stable_promotion`).
- A waived or stale item never renders as a clean pass; the six states stay distinct.
- No overview reads a posture above its profile.

The packet's `conformance` block records each of these as a hard invariant; the
`M5GovernanceDashboard::validate` method re-derives every part and fails on any drift, and the
headless emitter refuses to mint a packet that does not validate.
