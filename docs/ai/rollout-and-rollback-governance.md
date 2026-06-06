# AI rollout and rollback governance

AI behavior is release-bearing even when the desktop binary does not change.
Stable AI rows must therefore consume the checked rollout packet instead of
inferring provider, prompt, tool, or local-model truth from generic badges.

## Truth source

The canonical packet is:

`artifacts/ai/m4/provider-model-prompt-tool-rollout/rollout_packet.json`

Its boundary schema is:

`schemas/ai/ai-rollout-packet.schema.json`

Docs/help, CLI/headless inspection, support exports, admin review, and release
packets should read the packet or its support projection directly.

## Required route facts

Every claimed stable AI route must show:

- rollout object refs for provider/model enablement, prompt pack, tool-schema pack, and feature rollout;
- local-model pack provenance when the route runs locally;
- exact provider and model registry refs;
- prompt-pack version ref and compatible tool-schema range ref;
- routing-policy version ref;
- current graduation packet ref;
- independent rollback, deny, withdraw, or kill-switch refs;
- explicit fallback contract to local model, BYOK provider, or manual workflow;
- mirror/offline publication ref when the deployment profile supports mirrors.

## Rollout object rules

Provider/model enablement, prompt packs, tool-schema packs, local-model packs,
and feature-level AI rollouts promote independently through canary, pilot, broad,
and LTS. A stable route may consume only objects with current promotion evidence,
compatibility metadata, rollback or deny levers, and a fallback contract.

If any object lacks compatibility range metadata, current evidence, or a rollback
lever, the route narrows or blocks instead of inferring a default.

## Withdrawal behavior

Withdrawing or disabling a provider, model, prompt pack, tool schema, or
local-model pack emits a downgrade receipt. The receipt must name affected
routes, the withdrawn object, the fallback route class, and the user-visible
reason. The receipt must not classify the event as a general product outage.

Allowed fallback classes are:

- `local_model`
- `byok_provider`
- `manual_workflow`

## Mirror and offline publication

Mirrored and air-gapped deployments publish approved prompt, tool-schema, and
local-model packs with provenance, compatibility, revocation, and downgrade
metadata. Vendor-network access is not required to verify the checked manifest.

The local-model publication manifest is:

`artifacts/ai/m4/local-model-pack-publication/manifest.json`

## Verification

Run:

```sh
cargo test -p aureline-ai ai_pack_rollout
python3 tools/release/ai-pack-promotion/validate_ai_pack_promotion.py
```
