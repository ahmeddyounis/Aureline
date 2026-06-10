# M5 lane certification — local-model, provider, recipe, connector, and spend governance

This document is the contract for the M5 lane certification capstone. It locks
the canonical certification for the five governed service lanes —
local-model packs, provider routing, recipe automation, external connectors,
and spend governance — on every claimed M5 profile, so Milestone 5 can ship this
depth area with canonical implementation, proof, downgrade behavior, and
operator-facing truth instead of ad hoc prototypes or feature copy that outruns
evidence.

The packet is canonical: no product, help, or support surface in this lane may
present a greener claim than this certification.

## Source of truth

- Packet type: `M5LaneCertificationPacket`
  (`crates/aureline-ai/src/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/`).
- Boundary schema:
  `schemas/ai/certify-local-model-provider-recipe-connector-and-spend-governance-lanes-on-every-claimed-m5-profile.schema.json`.
- Checked support export:
  `artifacts/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/support_export.json`.
- Markdown summary:
  `artifacts/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile.md`.
- Protected fixtures:
  `fixtures/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/`.
- Conformance dump: `cargo run -p aureline-ai --example dump_m5_lane_certification [support|fixture]`.

## Certified lanes

Each lane is bound to the canonical source schema of the first-consumer packet it
certifies, required in `source_contract_refs` so the lane can never claim more
than its source schema admits.

| Lane | Canonical source |
| --- | --- |
| `local_model_pack` | local-model pack install / provenance / mirror schema |
| `provider_routing` | provider/model route disclosure schema |
| `recipe_automation` | signed/shared recipe-pack schema |
| `external_connector` | external-tool connector-manifest schema |
| `spend_governance` | long-running-agent budget schema |

## Claimed profiles

Profiles are the channel, profile, and provider families qualified in this batch.
Managed-service claims may not be widened beyond these families.

| Profile | Expected execution mode |
| --- | --- |
| `local_only` | `local` |
| `byok_direct` | `byok` |
| `managed_hosted` | `managed` |
| `offline_mirror` | `local` (on-device, mirrored channel) |
| `hybrid_managed` | `managed` |

## Disclosure scorecard

Every lane carries a scorecard covering all eight invariant axes that must remain
explicit and exportable on every claimed row — none may hide behind generic AI
language:

`provider_identity`, `model_route`, `execution_locality`, `region_residency`,
`retention_posture`, `cost_budget_owner`, `tool_side_effect`,
`automation_authority`.

Each row scores `0..=100` against a threshold and records a `pass` / `warn` /
`fail` status. A `Stable` lane must score `pass` on every dimension; a `Beta`
lane may carry `warn` but never `fail`.

## Per-profile coverage invariants

For every coverage row whose `claimed_on_profile` is true the certification
enforces:

- **Disclosure is complete.** `provider_disclosed`, `model_route_disclosed`,
  `region_disclosed`, `retention_disclosed`, and `cost_owner_disclosed` are all
  true — cost, provider, region, and retention can never be hidden.
- **Locality agrees with the family.** `execution_mode` equals the profile's
  expected mode, so the headline locality cannot disagree with where bytes run.
- **Managed claims stay in scope.** A claim on a managed family
  (`managed_hosted`, `hybrid_managed`) must declare
  `managed_claim_within_qualified_family`.
- **Authority fits the side effect.** The `automation_authority` rank is at least
  the floor required by the `side_effect_class`; an irreversible publish requires
  an admin-gated or managed-template authority. Signed/shared recipes and
  external-tool connectors follow the same preview, policy, and audit bar as
  first-party commands.
- **Claim flag matches qualification.** `claimed_on_profile` agrees with whether
  `profile_qualification` is a claimed class (`stable`/`beta`/`preview`), and a
  profile never claims more than the lane's headline qualification.

## Downgrade and proof freshness

Every lane carries a closed set of downgrade rules that narrow the claim instead
of hiding the lane. Each lane must carry a `proof_stale` rule, and every rule
must narrow to a strictly lower qualification than the headline claim. The packet
records a proof-freshness SLO and `auto_narrow_on_stale`, so stale, policy-blocked,
provider-unavailable, trust-narrowed, scope-expanded, or upstream-narrowed lanes
narrow automatically rather than keep an optimistic claim.

## Boundary

The packet carries only typed disclosure booleans and class tokens. Raw prompt
bodies, raw diffs, raw provider payloads, credentials, exact token counts, exact
cost amounts, and raw endpoint URLs never cross this boundary.

## Regenerating

After changing the packet shape, builder, or fixtures, regenerate the checked
artifacts so they stay byte-aligned with the in-crate builder:

```bash
cargo run -p aureline-ai --example dump_m5_lane_certification support \
  > artifacts/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/support_export.json
cargo run -p aureline-ai --example dump_m5_lane_certification fixture \
  > fixtures/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/held_connector_lane_certification.json
cargo test -p aureline-ai certify_local_model
```
