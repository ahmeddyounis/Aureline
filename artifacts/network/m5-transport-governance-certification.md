# M5 Transport-Governance Certification — Certified Packet

- Packet: `remote:m5_transport_governance_certification:default`
- Schema version: `1`
- Contract ref: `remote:m5_transport_governance_certification:v1`
- Schema: `schemas/network/m5_transport_governance_certification.schema.json`
- Runtime owner: `aureline_remote::m5_transport_governance_certification`
- Doc: `docs/network/m5-transport-governance-certification.md`
- Evidence index: `artifacts/release/m5/xt12-evidence-index.md`
- Overall verdict: `certified` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Certified rows: all (4)

This certification packet is the milestone-exit truth source of the
networked-surface transport-governance lane. It binds the six transport-
governance lanes (shared decision, proxy resolution, trust store, host proof,
mirror/offline continuity, and denial vocabulary) into one verdict per named M5
deployment profile, and auto-narrows any profile whose proof is missing, stale,
or partial — so a marketed enterprise/network/deployment row only hardens where
every proof dimension is current at once.

## Certified profiles

| Profile | Verdict | Dimensions satisfied |
|---|---|---|
| `local_oss` | `certified` | 6 / 6 (proxy_resolution, host_proof waived) |
| `self_hosted` | `certified` | 6 / 6 |
| `managed` | `certified` | 6 / 6 |
| `air_gapped` | `certified` | 6 / 6 |

## Certified dimensions and their bound evidence

| Dimension | Evidence contract ref | Evidence doc |
|---|---|---|
| `transport_decision` | `remote:networked_surface_transport_decision:v1` | `docs/network/networked-surface-transport-decision.md` |
| `proxy_resolution` | `remote:networked_surface_proxy_resolution:v1` | `docs/network/networked-surface-proxy-resolution.md` |
| `trust_store` | `remote:networked_surface_transport_trust:v1` | `docs/network/networked-surface-transport-trust.md` |
| `host_proof` | `remote:networked_surface_transport_trust:v1` | `docs/network/networked-surface-transport-trust.md` |
| `mirror_offline` | `remote:networked_surface_mirror_offline_continuity:v1` | `docs/network/networked-surface-mirror-offline-continuity.md` |
| `denial_vocabulary` | `remote:networked_surface_transport_automation:v1` | `docs/network/networked-surface-transport-automation.md` |

## Auto-narrowing drills

Each drill flips one input and shows the packet narrowing the affected claim:

| Drill | Trigger | Outcome |
|---|---|---|
| `drill_stale_narrowed` | `self_hosted` trust proof expired | row `narrowed` (`transport_proof_stale`) |
| `drill_missing_continuity_held` | `managed` mirror/offline cell absent | row `held_back` (`continuity_coverage_missing`) |
| `drill_missing_profile_held` | `air_gapped` profile absent | packet `held_back` (`required_profile_missing`) |
| `drill_raw_material_withdrawn` | raw private material exposed | packet `withdrawn` (`raw_material_exposed`) |
| `drill_fallthrough_withdrawn` | mirror-only fell through to public | packet `withdrawn` (`silent_public_fallthrough`) |

## Guardrails

The packet refuses to certify when any of these hold: raw private material is
present, a surface bypassed the shared governance layer, a non-idempotent action
was queued for replay, or a mirror-only route silently fell through to the
public internet. Only closed-vocabulary tokens, opaque refs, counts, and
plain-language sentences cross the boundary.

## Regeneration

Regenerate this artifact's evidence from the typed model (do not hand-edit the
fixtures):

```sh
cargo run -q -p aureline-remote \
  --example dump_m5_transport_governance_certification_fixtures -- <subcommand>
```
