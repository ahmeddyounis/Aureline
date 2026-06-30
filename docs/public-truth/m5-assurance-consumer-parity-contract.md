# M5 assurance consumer-parity contract

This contract freezes the assurance consumer-parity model: the one converged object that the claimed
M5 **About / help**, **procurement export**, **evaluation packet**, **support export**, and
**shiproom / public-truth** surfaces all read from, so they can never restate the same assurance,
control-proof, boundary, route, or event truth independently.

It sits on top of the five lanes that already mint that truth — the
[assurance center](m5-assurance-center-contract.md), the
[assurance-claim reducer](m5-assurance-claim-reducer-contract.md), the
[governance / fitness dashboard](m5-governance-dashboard-contract.md), the
[capability-boundary inspector](m5-boundary-inspector-contract.md), and the
[event-provenance inspector](m5-event-provenance-contract.md). It does **not** re-derive their facts
or invent new claims; it normalizes every item those packets publish into one fact grammar and routes
every consumer surface through it, so a fact narrowed or blocked in one surface can never read
stronger in another.

- Packet schema: [`schemas/public-truth/m5-assurance-consumer-parity.schema.json`](../../schemas/public-truth/m5-assurance-consumer-parity.schema.json)
- Published inventory: [`artifacts/public-truth/m5-assurance-consumer-parity.json`](../../artifacts/public-truth/m5-assurance-consumer-parity.json)
- Rendered overview: [`artifacts/public-truth/m5-assurance-consumer-parity.md`](../../artifacts/public-truth/m5-assurance-consumer-parity.md)
- Machine-readable fact / consumer matrix: [`artifacts/public-truth/m5-assurance-consumer-parity-facts.csv`](../../artifacts/public-truth/m5-assurance-consumer-parity-facts.csv)
- Release-grade export proof: `artifacts/release/m5-assurance-export-proof/consumer-parity.json` (+ `.md`)
- Exported refs-only preview: `artifacts/release/m5-assurance-export-proof/export-preview.json`
- Per-state fixtures: `fixtures/public-truth/m5-assurance-consumers/`
- Producer crate / module: `crates/aureline-release` → `m5_assurance_consumer_parity`
- Headless emitter: `aureline_release_m5_assurance_consumer_parity`

## What the model holds

The packet is minted from one input by the headless emitter — the five already-published source
packets — so the in-code packet, the published artifacts, and the fixtures can never drift.

### 1. Source bindings

One per ingested source packet. The model binds to each source by id, record kind, and **registry
ref**, records how many facts it contributed, and records whether the source validated clean on
ingest. The packet never embeds a source body; the binding is the lineage to the authoritative record.

### 2. Unified facts

One per inspectable item across the five sources, normalized to a single grammar:

| Domain | Source | What it carries |
|--------|--------|-----------------|
| `assurance_claim` | assurance-claim reducer | a reduced regulated / sovereign / self-hosted claim |
| `control_proof` | assurance center | a control-proof row backing one or more claims |
| `governance_fitness` | governance dashboard | a fitness-function tile |
| `service_ownership` | governance dashboard | a service-ownership card |
| `decision_right` | governance dashboard | a decision-right card |
| `capability_boundary` | boundary inspector | a boundary summary for a high-risk action |
| `route_timeline` | boundary inspector | a route-hop timeline for a high-risk action |
| `approval_ticket` | boundary inspector | an approval-ticket inspector for a high-risk action |
| `event_provenance` | event-provenance inspector | a deferred / replayable event's verdict |

Every fact derives its coverage `status`, traffic-light `signal`, and `effective_qualification` from
its **gate** (`governed` → Stable, `narrowed` → Beta, `blocked` → Unavailable), so a fact can never
read stronger than the source item's gate. Each fact preserves its owner, its evidence-freshness
reading, and the repo-relative evidence refs behind it.

### 3. Consumer projections

Every fact carries one `consumer_projection` per consumer surface — About / help, procurement export,
evaluation packet, support export, and the release / public-truth manifest. Each projection reads the
**same** gate and qualification as its fact; `converges_with_fact` is the proof. This is the
guardrail: one fact governs all five surfaces, so a fact narrowed in one consumer can never be
silently strengthened in another.

### 4. Consumer views

One generated view per consumer surface. Each view reads the **whole** fact set — `reads_all_facts`
and `fact_count` are the proof — and summarizes it at the worst gate across the facts, so the
About / help panel, the procurement export, the evaluation packet, the support export, and the
public-truth manifest are generated views over one inventory rather than independent prose copies.

## How a source narrowing carries through

The model takes the five source packets and, for each item, takes the item's own effective gate (for
deferred events, the whole event verdict — the worst of the provenance, route-drift, and reapproval
facets). When any source narrows or blocks an item, the matching fact narrows or blocks, every
consumer projection reads the narrowed gate, every consumer view's worst gate moves with it, and the
matching source binding records that it blocks. When any fact is blocked, the packet holds Stable
promotion. A drill that perturbs exactly one source lane narrows or blocks exactly the facts that
depend on it and leaves the rest governed.

## Same grammar in product and in export

The exported `export_preview` reduces each fact to its domain, subject, gate, qualification, owner,
freshness, and evidence refs — drawn from the same controlled vocabulary the in-product facts show —
so an offline procurement / evaluation / support review and the live surfaces can never read
differently.

## Export safety

The packet is metadata-only. It binds to sources by ref rather than embedding raw bodies, preserves
owner / freshness / route lineage as repo-relative refs, and carries no credential bodies or raw
provider payloads; the `export_carries_no_raw_material` conformance flag scans the serialized export
for forbidden material. None of these facts require sign-in or live network access to read.

## Regenerating

Run the headless emitter and write its output to the checked-in artifacts:

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- registry > artifacts/public-truth/m5-assurance-consumer-parity.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- overview > artifacts/public-truth/m5-assurance-consumer-parity.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- csv      > artifacts/public-truth/m5-assurance-consumer-parity-facts.csv
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- registry > artifacts/release/m5-assurance-export-proof/consumer-parity.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- markdown > artifacts/release/m5-assurance-export-proof/consumer-parity.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_consumer_parity -- export   > artifacts/release/m5-assurance-export-proof/export-preview.json
```

The per-state fixtures under `fixtures/public-truth/m5-assurance-consumers/` are minted from the
`variant` subcommand (`canonical`, `claim`, `governance`, `boundary`, `event`).
