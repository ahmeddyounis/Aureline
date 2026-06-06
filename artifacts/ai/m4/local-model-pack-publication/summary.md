# Local model pack publication summary

**Manifest:** `local-model-pack-publication:stable:2026-06-06`  
**Publication:** `mirror-publication:ai-packs:stable:2026-06-06`  
**Source packet:** `artifacts/ai/m4/provider-model-prompt-tool-rollout/rollout_packet.json`

The local-model pack publication manifest records provenance, compatibility,
revocation, and downgrade truth for `local-model-pack:aureline-code-14b:2026-06`.
It is mirrorable and verified for self-hosted mirror and air-gapped profiles
without vendor-network access.

| Field | Value |
|---|---|
| Artifact digest | `sha256:7b4d9a` |
| Runtime ABI | `runtime-abi:local-model:v2` |
| Size on disk | `8.4 GB` |
| Hardware note | `16 GB VRAM recommended; CPU fallback admitted with slower latency label` |
| Offline posture | `runs_entirely_on_this_machine` |
| Revocation state | `approved` |
| Downgrade contract | `fallback-contract:local-model:byok-manual` |

The revocation affects AI route selection only. Local editing, search, Git, and
manual review remain available.
