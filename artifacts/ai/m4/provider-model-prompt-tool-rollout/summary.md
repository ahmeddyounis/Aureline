# AI pack rollout publication summary

**Packet:** `ai-rollout-publication:stable:2026-06-06`  
**Policy epoch:** `policy-epoch:stable:0004`  
**Routing policy:** `routing-policy:ai:stable:v4`  
**Schema:** `schemas/ai/ai-rollout-packet.schema.json`

This packet makes stable AI route truth independent from the desktop binary. It
publishes provider/model enablement, prompt-pack versions, tool-schema packs,
local-model packs, feature-level rollout objects, downgrade receipts, and
mirror/offline metadata as one checked artifact.

## Stable routes

| Route | Provider/model | Prompt pack | Tool-schema range | Local model provenance | Rollback levers |
|---|---|---|---|---|---|
| `ai-route:inline-chat-local-first:stable` | `provider-entry:first-party-local-chat:0002` / `model-entry:first-party-local-chat:small:0002` | `prompt-pack:inline-chat-stable:v4` | `tool-schema-range:no-tools:v1` | `local-model-pack:aureline-code-14b:2026-06#sha256:7b4d9a` | provider deny, prompt rollback, tool deny, local-pack withdraw, feature kill switch |
| `ai-route:review-chat-cheapest:stable` | `provider-entry:managed-hosted-chat:0002` / `model-entry:managed-chat:general:0002` | `prompt-pack:review-stable:v7` | `tool-schema-range:docs-connector:v3` | not applicable | provider deny, prompt rollback, tool disable, feature kill switch |
| `ai-route:ai-apply-scoped:stable` | `provider-entry:managed-hosted-apply:0001` / `model-entry:managed-apply:review:0001` | `prompt-pack:apply-stable:v3` | `tool-schema-range:apply-scoped:v2` | not applicable | provider deny, prompt rollback, tool disable, feature kill switch |

## Downgrade receipts

| Withdrawn object | Affected route | Fallback | Product outage |
|---|---|---|---|
| `rollout-object:provider-model:managed-chat:0002` | `ai-route:review-chat-cheapest:stable` | local model, BYOK, or manual review | false |
| `rollout-object:prompt-pack:review:v7` | `ai-route:review-chat-cheapest:stable` | local model or manual review | false |
| `rollout-object:tool-schema:docs-connector:v3` | `ai-route:review-chat-cheapest:stable` | manual docs search | false |
| `rollout-object:local-model:aureline-code-14b:2026-06` | `ai-route:inline-chat-local-first:stable` | BYOK or manual explanation | false |

## Mirror/offline publication

The mirror publication `mirror-publication:ai-packs:stable:2026-06-06` carries
prompt-pack, tool-schema, and local-model provenance, compatibility, revocation,
and downgrade manifests without vendor-network dependence. The local-model
publication manifest is
`artifacts/ai/m4/local-model-pack-publication/manifest.json`.

Verify with:

```sh
cargo test -p aureline-ai ai_pack_rollout
python3 tools/release/ai-pack-promotion/validate_ai_pack_promotion.py
```
