# M5 assurance / governance / route-provenance governance contract

This contract freezes the one governed baseline every claimed M5 assurance-center,
control-proof, exception / waiver, governance-dashboard, service-ownership,
capability-boundary, route-hop, approval-ticket, and event-provenance surface qualifies
against. It exists so that assurance claims, control proof, ownership / freshness posture,
and high-risk route / event explainability stop living as fragmented procurement reports,
admin panels, and hidden debug state, and instead resolve to one inspectable,
machine-readable matrix.

It does **not** invent new compliance frameworks or certification families, add managed
products, or rebuild the existing policy-bundle, runtime-approval, record / hold / deletion,
or publication objects. It is the missing claim / boundary / governance inspection layer
those objects bind to.

- Packet schema: [`schemas/release/m5-assurance-route-governance.schema.json`](../../schemas/release/m5-assurance-route-governance.schema.json)
- Published inventory: [`artifacts/release/m5-assurance-route-governance-summary.json`](../../artifacts/release/m5-assurance-route-governance-summary.json)
- Rendered governance matrix: [`artifacts/release/m5-assurance-route-governance.md`](../../artifacts/release/m5-assurance-route-governance.md)
- Machine-readable matrix: [`artifacts/release/m5-assurance-route-matrix.csv`](../../artifacts/release/m5-assurance-route-matrix.csv)
- Release-grade parity proof: `artifacts/release-proof/m5-assurance-route-governance/assurance-route-matrix.json` (+ `.md`)
- Per-state fixtures: `fixtures/release/m5-assurance-route/`
- Producer crate / module: `crates/aureline-release` → `m5_assurance_route_governance`
- Headless emitter: `aureline_release_m5_assurance_route_governance`

## What the matrix governs

The matrix has three parts, all minted from one source by the headless emitter so the
in-code packet, the published artifacts, and the fixtures can never drift.

### 1. Canonical assurance state families

Six ordered, gate-bound vocabularies. Every token binds to a gate posture
(`governed` / `narrowed` / `blocked`) drawn from the shared descriptor / badge runtime, and
to the effective qualification floor that posture implies (`stable` / `beta` / `unavailable`).
Surfaces reuse these tokens instead of restating assurance / governance / route state as ad
hoc labels — and a governance dashboard's `pass`, `stale`, `waived`, and `blocked` postures
resolve to one frozen vocabulary.

| Family | Governed | Narrowed | Blocked |
|--------|----------|----------|---------|
| `assurance_claim` | `proven`, `attested` | `under_review`, `exception_pending` | `unproven` |
| `governance` | `pass`, `monitored` | `stale`, `waived` | `blocked` |
| `capability_boundary` | `within_boundary`, `boundary_documented` | `at_boundary_edge`, `boundary_narrowed` | `outside_boundary` |
| `route_hop` | `local_only`, `attributed_remote` | `mirrored_route`, `route_degraded` | `unattributed_route` |
| `approval` | `pre_authorized`, `approved` | `approval_pending`, `approval_required` | `approval_denied` |
| `provenance` | `fully_traced`, `derived_traced` | `partial_provenance`, `provenance_stale` | `provenance_missing` |

### 2. Governed facets

The nine product surfaces the source set treats as governed assurance / governance / route
truth. Each facet owns exactly one proof path and an accountable owner role, names the state
family that governs it, and discloses the evidence classes it binds, the claimed posture
lines (`managed` / `self_hosted` / `regulated` / `sovereign`) and trust boundaries
(`local_first` / `control_plane`) it scopes to, and how it behaves under stale / mirrored /
no-live-data conditions.

| Facet | Dimension | State family | Owner role |
|-------|-----------|--------------|------------|
| `assurance_claim` | `claim_assurance` | `assurance_claim` | `assurance_center_owner` |
| `control_proof` | `claim_assurance` | `assurance_claim` | `control_proof_owner` |
| `exception_waiver` | `claim_assurance` | `governance` | `exception_waiver_owner` |
| `governance_freshness` | `governance_posture` | `governance` | `governance_dashboard_owner` |
| `service_ownership` | `governance_posture` | `governance` | `service_ownership_owner` |
| `capability_boundary` | `governance_posture` | `capability_boundary` | `capability_boundary_owner` |
| `route_hop` | `route_provenance` | `route_hop` | `route_explainability_owner` |
| `approval_ticket` | `route_provenance` | `approval` | `approval_authority_owner` |
| `event_provenance` | `route_provenance` | `provenance` | `event_provenance_owner` |

### 3. Claimed consumers

The eight surfaces that read the matrix. Each binds the facets it reads; the matrix derives
its covered dimensions, the union of evidence classes / postures / trust boundaries those
facets disclose, the proof paths backing them, the per-consumer coverage gaps, and the
verdict (status, gate decision, effective qualification) from those facets' proof freshness
and current assurance state — never a hand-maintained status.

| Consumer | Owner role |
|----------|------------|
| `assurance_center` | `assurance_center_owner` |
| `governance_dashboard` | `governance_dashboard_owner` |
| `capability_inspector` | `capability_inspector_owner` |
| `route_inspector` | `route_inspector_owner` |
| `admin_console` | `admin_console_owner` |
| `help_about` | `help_about_owner` |
| `procurement_evaluation` | `procurement_owner` |
| `support_export` | `support_export_owner` |

## How gaps fail the matrix

Two kinds of gap fail a consumer rather than remaining implicit, and each names its drifted
dimension:

- **Proof-currency gaps.** A read facet whose proof is `stale` narrows the consumer below
  Stable (`proof_stale`); a read facet whose proof is `expired` or `missing`, or a facet the
  matrix does not govern at all, blocks the consumer from Stable promotion (`proof_expired`,
  `proof_missing`, `facet_ungoverned`).
- **Assurance-state gaps.** A read facet whose current canonical state binds a `narrowed`
  posture narrows the consumer (`assurance_state_narrowed`); a state binding a `blocked`
  posture blocks it (`assurance_state_blocked`).

A consumer's effective qualification is its claim narrowed down the worst gate among its
gaps: any blocking gap floors it at `unavailable`; any narrowing gap floors it at `beta`; an
ungapped consumer stands at its claim. The packet-level release gate aggregates the
per-consumer gates and lists the blocked, narrowed, and certified consumers plus the drifted
dimensions, so release and shiproom tooling can fail promotion directly from the matrix.

## Export safety

The packet is metadata-only. It preserves route and evidence lineage as repo-relative proof
**refs**, never inline payloads, and a redaction scan rejects any export key that looks like
a credential, secret, password, API key, raw payload, or bearer token. A managed or vendor
outage never implies local inspection is unsafe: facets that stay `local_first` keep their
locally recorded lineage under the `local_lineage_only` degraded-data behavior.

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- registry   > artifacts/release/m5-assurance-route-governance-summary.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- governance > artifacts/release/m5-assurance-route-governance.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- csv        > artifacts/release/m5-assurance-route-matrix.csv
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- registry   > artifacts/release-proof/m5-assurance-route-governance/assurance-route-matrix.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- markdown   > artifacts/release-proof/m5-assurance-route-governance/assurance-route-matrix.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- variant canonical > fixtures/release/m5-assurance-route/assurance_route_all_current.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- variant stale     > fixtures/release/m5-assurance-route/assurance_route_stale_proof_narrowed.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_route_governance -- variant missing   > fixtures/release/m5-assurance-route/assurance_route_missing_proof_blocked.json
```
