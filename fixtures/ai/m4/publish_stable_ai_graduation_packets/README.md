# M4 stable graduation — fixture registry

This directory contains the test fixtures for the stable AI graduation release.

## Files

| File | Description |
|---|---|
| `registry_packet.json` | Full `ProviderModelRegistryPacket` for the stable registry state. Used by `publish_stable_ai_graduation_packets` tests. |

## Registry state

- **Registry ID:** `provider-model-registry:stable:2026-06-01`
- **Policy epoch:** `policy-epoch:stable:0004`

## Provider entries

| Entry | Class | Lifecycle | Selected by |
|---|---|---|---|
| `provider-entry:first-party-local-chat:0002` | `first_party_self_hosted` | `generally_admitted` | `surface:inline-chat-local-first` |
| `provider-entry:managed-hosted-chat:0002` | `first_party_managed` | `generally_admitted` | `surface:review-chat-cheapest` |
| `provider-entry:managed-hosted-apply:0001` | `first_party_managed` | `generally_admitted` | `surface:ai-apply-scoped` |

## Usage

These fixtures are loaded by tests in `crates/aureline-ai/src/publish_stable_ai_graduation_packets/tests.rs`.

```rust
let registry: ProviderModelRegistryPacket = serde_json::from_str(
    include_str!("../../../../../fixtures/ai/m4/publish_stable_ai_graduation_packets/registry_packet.json"),
)?;
```
