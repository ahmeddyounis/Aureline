# AI pack rollout fixture lane

The canonical packet for this lane is
`artifacts/ai/m4/provider-model-prompt-tool-rollout/rollout_packet.json`.

Use this fixture directory for mutation cases that prove:

- stable route rows cannot omit provider/model, prompt-pack, tool-schema, or local-model provenance;
- withdrawn provider/model, prompt-pack, tool-schema, or local-model objects require downgrade receipts;
- downgrade receipts must not present AI-pack withdrawal as a general product outage;
- mirror/offline publication keeps provenance, compatibility, revocation, and downgrade metadata without vendor-network access.
