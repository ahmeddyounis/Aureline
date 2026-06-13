# M5 transport-governance certification

This layer is the **certification** truth source of the networked-surface
transport-governance lane. The sibling lanes each own one slice of the
contract — the
[transport-decision log](./networked-surface-transport-decision.md) emits one
decision per action, the
[proxy-resolution layer](./networked-surface-proxy-resolution.md) freezes the
proxy precedence, the
[transport-trust layer](./networked-surface-transport-trust.md) freezes the
trust store and host proof, the
[mirror/offline continuity layer](./networked-surface-mirror-offline-continuity.md)
freezes per-family route handling, and the
[transport-automation layer](./networked-surface-transport-automation.md) gives
the failure surfaces one canonical denial vocabulary. This layer answers the
question those leave open for the milestone exit gate: **which marketed M5
enterprise/network/deployment profiles end with current proof across every
transport-governance dimension at once — and which must narrow or hold?**

The runtime owner is `aureline_remote::m5_transport_governance_certification`;
the boundary schema is
`schemas/network/m5_transport_governance_certification.schema.json`.

No raw URLs, raw hostnames, raw ports, raw paths, raw query strings, raw
cookies, raw headers, raw bearer or session tokens, raw private certificate
bytes, or raw SSH private material cross the boundary — only closed-vocabulary
tokens, opaque refs, UTC timestamps, counts, and plain-language summary
sentences.

## The certification matrix

The packet binds, for every required **profile**, one **cell** per **dimension**:

- **Profiles** — `local_oss`, `self_hosted`, `managed`, `air_gapped` (the
  canonical deployment vocabulary).
- **Dimensions** — `transport_decision`, `proxy_resolution`, `trust_store`,
  `host_proof`, `mirror_offline`, `denial_vocabulary`. Each dimension binds to
  the sibling lane that owns its underlying proof, so the certification is an
  *index* over existing proof rather than a second copy of the status text.

Each cell carries a **state** (`pass`, `partial`, `stale`, `missing`, `waived`)
and a **freshness** (`fresh`, `stale_within_window`, `expired_beyond_window`). A
cell is satisfied only when it is a fresh `pass` or a `waived` dimension that
does not apply to the profile (for example, proxy resolution and host proof are
waived on the no-egress `local_oss` profile).

## Verdicts and auto-narrowing

From those cells the layer derives one **verdict** per profile and an overall
verdict for the packet:

- `certified` — every required dimension passes fresh (or is waived), every
  surface resolved through the shared governance layer, replay queues are
  idempotent-only, and no mirror-only route silently fell through to the public
  internet.
- `narrowed` — a `stale` or `partial` proof narrows the profile
  (`transport_proof_stale`, `transport_proof_partial`).
- `held_back` — a required dimension, continuity coverage, or denial vocabulary
  is missing (`transport_proof_missing`, `continuity_coverage_missing`,
  `denial_vocabulary_missing`), or a required profile has no record
  (`required_profile_missing`).
- `withdrawn` — a hard guardrail was violated (`raw_material_exposed`,
  `shared_governance_bypassed`, `non_idempotent_replay_queued`,
  `silent_public_fallthrough`). A withdrawal taints the whole packet.

This is the auto-narrowing the exit gate depends on: a marketed
enterprise/network/deployment row only hardens where every proof dimension is
current at once. Stale evidence, a missing denial vocabulary, or missing
continuity coverage visibly narrows the affected profile rather than letting a
wider claim publish.

## Evidence index

The page exposes one `dimension_binding` per dimension, each carrying the
sibling lane's canonical contract ref and doc ref. Release center, shiproom,
Help/About, docs, and support exports follow these bindings instead of cloning
per-lane status text, so the certification packet is the single canonical
evidence map for M5 transport governance.

## Guardrails

- Only explicitly idempotent actions may enter offline-deferred or replay
  queues (`replay_idempotent_only`).
- A mirror-only or deny-all profile may never silently fall through to the
  public internet (`no_silent_public_fallthrough`).
- No raw secrets, cookies, bearer tokens, private certificate bytes, or SSH
  private material appear in any cell, row, defect, or support packet
  (`raw_private_material_excluded`).

## Consumers

The seeded packet is emitted as machine-readable JSON by the headless example
`dump_m5_transport_governance_certification_fixtures`, and the committed
fixtures under `fixtures/network/m5_transport_governance_certification/` are
pinned to it by the `m5_transport_governance_certification` integration test, so
the checked-in evidence and the typed truth model cannot drift apart silently.

## Contract ref

`remote:m5_transport_governance_certification:v1`
