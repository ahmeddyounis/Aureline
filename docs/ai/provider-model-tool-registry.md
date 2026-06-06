# Provider, Model, Prompt-Pack, Local-Pack, and Tool Registry

This document describes the stable registry packet owned by
`aureline_ai::registry::ProviderModelRegistryPacket`.

The packet is the source of truth for claimed AI rows. UI, CLI/headless,
admin inspectors, support exports, and docs/help surfaces read the same
provider, model, prompt-pack, local-model-pack, tool-schema-pack, and
external-tool rows instead of copying provider nicknames or host strings.

## Stable Row Families

| Row family | Required stable identity |
|---|---|
| Provider | provider entry id, rollout-object id, transport, auth mode, retention posture, region posture, quota family, execution locus, timeout/retry envelope, allowed feature classes |
| Model | model entry id, rollout-object id, vendor/source label, tokenizer family, capability version, structured-output support, tool-call support, graduation-state ref |
| Prompt pack | prompt-pack id, prompt-pack version ref, rollout-object id, compatible provider/model refs, rollback ref, lifecycle state |
| Tool-schema pack | schema-pack id, version ref, compatible range, rollout-object id, side-effect class, execution locus, approval posture, evidence requirement, rollback ref |
| Local model pack | pack id, version ref, rollout-object id, artifact digest, model hash, runtime ABI, accelerator requirements, storage class, license/provenance note, withdrawal ref |
| External tool / MCP server | tool entry id, rollout-object id, transport, execution locus, auth mode, capability family, side-effect classes, allowed data classes, compatible tool-schema-pack refs |

## Route Receipts

Stable route receipts reconstruct the actual behavior from registry refs:

- provider entry id and model entry id;
- routing-policy version ref;
- prompt-pack version ref;
- tool-schema-pack version ref and compatible range for tool-assisted routes;
- local-model-pack provenance ref for local routes;
- external-tool execution locus refs;
- fallback chain refs;
- independent rollback or deny lever ref;
- decision cause shared by route blocks, downgrades, admin views, and support exports.

If a route cannot supply these refs, it must narrow below Stable or block before
dispatch. A third-party or remote tool hop cannot inherit the model route label;
it must carry its own execution locus, side-effect class, and auth posture.

## Support Export Contract

`ProviderModelRegistrySupportExport` projects provider, model, prompt-pack,
tool-schema-pack, local-model-pack, external-tool, and claimed-surface summaries
from one registry state ref. Support exports include only metadata-safe refs,
tokens, and labels. Raw endpoints, raw credential bodies, request/response
payloads, unredacted hostnames, exact token counts, and exact prices remain
outside this boundary.
