# M5 assurance center contract

This contract freezes the user / admin / evaluator-facing assurance center: the surface that turns
Aureline's regulated, sovereign, air-gapped, telemetry, residency, key-ownership, and local-first
continuity claims into inspectable product truth. It is the layer above the
[assurance / governance / route-provenance governance matrix](../release/m5-assurance-route-governance-contract.md):
the matrix certifies *whether* each claimed surface is mapped, narrowed, or blocked; this lane is
the assurance center a person actually reads.

It does **not** invent new compliance frameworks or certification families, and it does not restate
control proof as marketing copy. Every claim card derives its state from the controls backing it, so
the product can never imply a posture the active path does not satisfy now.

- Packet schema: [`schemas/public-truth/m5-assurance-center.schema.json`](../../schemas/public-truth/m5-assurance-center.schema.json)
- Published inventory: [`artifacts/public-truth/m5-assurance-center.json`](../../artifacts/public-truth/m5-assurance-center.json)
- Rendered overview: [`artifacts/public-truth/m5-assurance-center.md`](../../artifacts/public-truth/m5-assurance-center.md)
- Machine-readable claim / control matrix: [`artifacts/public-truth/m5-assurance-center-claims.csv`](../../artifacts/public-truth/m5-assurance-center-claims.csv)
- Release-grade parity proof: `artifacts/public-truth/m5-assurance-center-proof/assurance-center.json` (+ `.md`)
- Exported evaluation packet: `artifacts/public-truth/m5-assurance-center-proof/evaluation-packet.json`
- Per-state fixtures: `fixtures/public-truth/m5-assurance-center/`
- Producer crate / module: `crates/aureline-release` → `m5_assurance_center`
- Headless emitter: `aureline_release_m5_assurance_center`

## What the assurance center holds

The packet has four product parts, all minted from one source by the headless emitter — each
control's current proof state and evidence freshness — so the in-code packet, the published
artifacts, and the fixtures can never drift.

### 1. Claim cards

One per claim subject. A card never asserts a fixed state; it **derives** its active state from the
control-proof rows backing it, taking the worst gate among them, so a card can never read stronger
than its proof. The active state is drawn from the governance matrix's frozen assurance-claim
grammar (`proven` / `attested` / `under_review` / `exception_pending` / `unproven`). When the active
state is not fully governed the card carries a **nearest truthful fallback** naming the weaker
posture that is still proven, the controls still proven, and the controls that are not — the fallback
never reads `proven` and never reads above the claimed posture.

| Claim | Claimed posture | Required controls |
|-------|-----------------|-------------------|
| `local_first_continuity` | `managed` | `local_edit_continuity` |
| `telemetry_control` | `self_hosted` | `telemetry_egress_gate` |
| `key_ownership` | `self_hosted` | `customer_managed_key_custody`, `local_key_escrow` |
| `data_residency` | `regulated` | `data_residency_pin` |
| `regulated_operation` | `regulated` | `regulated_audit_trail`, `data_residency_pin` |
| `air_gap_containment` | `sovereign` | `vendor_path_severed`, `offline_update_path` |
| `sovereign_deployment` | `sovereign` | `sovereign_control_plane`, `customer_managed_key_custody`, `vendor_path_severed` |

### 2. Control-proof rows

The controls a claim asserts are proven. Each control owns one evidence class, owner role, and a
repo-relative proof ref drawn from the governance-matrix proofs under
`artifacts/release-proof/m5-assurance-route-governance/`, so the assurance center reuses the existing
proof lanes rather than minting a parallel evidence family. A control's effective gate is the more
restrictive of its proof state's gate and its evidence freshness's gate, so a `proven` control with
stale evidence still narrows the claims that read it.

### 3. Exception / waiver rows

The controls held under an accepted waiver. Each row reads `waived` and discloses its mitigation,
expiry, the compensating control standing in for the waived one, the responsible party
(`customer` / `admin` / `vendor`), and the action that clears it. A waived control sits in the
`exception_pending` proof state, so the claims that require it narrow to `exception_pending` and the
exception is never hidden.

### 4. Per-profile overviews

One per deployment profile (`managed` / `self_hosted` / `regulated` / `sovereign`). Each overview
lists the claims applicable to that profile, the claim-state and evidence-freshness summaries, the
open-exception count, the evaluation / export actions, and the **effective posture**: the strongest
posture every applicable claim is governed at. The effective posture auto-narrows below the profile
the moment a claim it would imply cannot be proven, and never reads above the profile.

## How drift narrows or blocks a claim

A claim is only as strong as its weakest required control:

- A **stale** control narrows the claim to `under_review` (`control_narrowed`); the claim floors at
  `beta`.
- A **waived** control narrows the claim to `exception_pending` (`control_narrowed`), disclosed in
  full; the claim floors at `beta`.
- An **expired / missing** control blocks the claim to `unproven` (`control_blocked`); the claim
  floors at `unavailable` and holds Stable promotion.

The summary aggregates the per-claim states and the release gate; any blocked claim sets
`blocks_stable_promotion`.

## Same grammar in product and in export

The exported evaluation packet reuses the exact claim-state and proof vocabulary the cards show —
claim states, control proof states, governance states, and postures are validated to be members of
the one canonical vocabulary, so an exported evaluation pack and the live assurance center can never
read differently.

## Export safety

The packet is metadata-only. It preserves proof lineage as repo-relative proof **refs**, never inline
payloads, and a redaction scan rejects any export key that looks like a credential, secret, password,
API key, raw payload, or bearer token. The local-first continuity claim is honored under every
profile, so a managed or vendor outage never implies local editing is unsafe.

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- registry   > artifacts/public-truth/m5-assurance-center.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- overview   > artifacts/public-truth/m5-assurance-center.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- csv        > artifacts/public-truth/m5-assurance-center-claims.csv
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- registry   > artifacts/public-truth/m5-assurance-center-proof/assurance-center.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- markdown   > artifacts/public-truth/m5-assurance-center-proof/assurance-center.md
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- evaluation > artifacts/public-truth/m5-assurance-center-proof/evaluation-packet.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- variant canonical > fixtures/public-truth/m5-assurance-center/assurance_center_all_proven.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- variant waiver    > fixtures/public-truth/m5-assurance-center/assurance_center_waiver_narrowed.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- variant stale     > fixtures/public-truth/m5-assurance-center/assurance_center_stale_evidence_narrowed.json
cargo run -q -p aureline-release --bin aureline_release_m5_assurance_center -- variant missing   > fixtures/public-truth/m5-assurance-center/assurance_center_missing_evidence_blocked.json
```
