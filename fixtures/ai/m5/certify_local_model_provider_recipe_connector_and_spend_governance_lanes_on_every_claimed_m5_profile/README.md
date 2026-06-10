# M5 Lane Certification Fixtures

## held_connector_lane_certification.json

A certification fixture where the external-connector lane is held pending
upstream provider graduation. Local-model packs, provider routing, and spend
governance stay Stable; recipe automation stays Beta. The held connector lane is
not a claimed lane — every profile row is unclaimed, so it carries no evidence
packet refs and is exempt from the claimed-row disclosure, locality, managed, and
authority gates — while still covering every disclosure dimension and every
profile, and narrowing to `unavailable` on stale proof.

Regenerate with:

```bash
cargo run -p aureline-ai --example dump_m5_lane_certification fixture \
  > fixtures/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/held_connector_lane_certification.json
```
