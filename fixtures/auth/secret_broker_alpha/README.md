# Secret Broker Alpha Fixtures

These fixtures anchor the secret-broker alpha rows implemented in
[`crates/aureline-auth/src/secrets`](../../../crates/aureline-auth/src/secrets/).

They use opaque refs, aliases, class names, store posture, continuity results,
and recovery actions only. Raw secret values, raw provider payloads, raw
refresh tokens, raw delegated token bodies, raw request headers, raw private
keys, and raw handle ids in support exports never appear.

## Index

| Fixture | Record kind | What it proves |
|---|---|---|
| [`baseline_handle_session_delegated.json`](./baseline_handle_session_delegated.json) | `secret_broker_alpha_packet_record` | Claimed alpha rows can reference a credential-store handle, visible session-only broker memory, and a delegated credential without storing raw secret material. |
| [`failure_locked_unavailable_trust_changed.json`](./failure_locked_unavailable_trust_changed.json) | `secret_broker_alpha_packet_record` | Locked, unavailable, and trust-store-changed states produce exact continuity rows for registry auth, managed sign-in refresh, and database attach, with local-safe recovery and metadata-only support export. |
