# Secret Broker Alpha

This page is the reviewer entry point for the first secret-broker runtime
contract in `aureline-auth`.

## Runtime Contract

The canonical packet is
[`SecretBrokerAlphaPacket`](../../crates/aureline-auth/src/secrets/mod.rs).
Each [`SecretBrokerAlphaRow`](../../crates/aureline-auth/src/secrets/mod.rs)
records:

- the affected capability class (`registry_auth`,
  `managed_sign_in_refresh`, `provider_reconnect`, `database_attach`, or
  `tunnel_reuse`);
- the reference mode (`handle`, `session_only`, or `delegated`);
- the secret class, consumer identity, target ref, and workspace scope;
- storage mode, store source, trust-store class, unlock state, and projection
  mode;
- a first-class continuity result with local continuation, exact affected
  capabilities, typed denial reason, and safe recovery actions; and
- a redaction posture that keeps support/export metadata useful while excluding
  raw secret values and raw handle ids.

The row validator rejects plaintext persistence, silent in-memory promotion,
stale-ticket reuse, raw secret material, missing handle/session/delegated refs,
and paused rows that do not name a typed denial reason and recovery path.

## Continuity Results

Credential-store and trust-store failures are represented as continuity
results, not generic auth failures:

| State | Meaning | Required posture |
|---|---|---|
| `paused_credential_store_locked` | OS credential store or other trust store is present but locked. | Pause the exact credentialed capability, offer unlock/retry and local-only continuation, export metadata only. |
| `paused_credential_store_unavailable` | Store is unreachable, corrupted, unsupported, or unavailable. | Pause the exact credentialed capability, offer recovery/reauth where safe, preserve local work, never reuse stale tickets. |
| `paused_trust_store_changed` | Trust evidence changed after handle issuance. | Rebind before reuse, keep cached/local work inspectable, export the typed denial reason only. |
| `degraded_session_only_visible` | Session-only broker memory is in use. | Show the degraded process-scoped posture and reprompt expectation; never promote it silently to persistent storage. |

## Support Export

[`SecretBrokerSupportExport`](../../crates/aureline-auth/src/secrets/mod.rs)
is the first support/export consumer. It projects each broker row into
metadata-only evidence:

- class, capability, consumer id, target ref, workspace scope;
- alias ref when exportable;
- handle/session/delegated ref presence, not raw runtime refs;
- store class, unlock state, projection mode, continuity state, denial reason,
  and recovery actions; and
- `raw_secret_values_exported = false` and `raw_handle_ids_exported = false`.

This support projection is intentionally local and schema-light for the alpha.
It can later feed the support-bundle manifest without changing the broker row
truth model.

## Protected Fixtures

Fixtures live under
[`fixtures/auth/secret_broker_alpha`](../../fixtures/auth/secret_broker_alpha/):

- `baseline_handle_session_delegated.json` proves handle, session-only, and
  delegated rows can coexist without raw secret persistence.
- `failure_locked_unavailable_trust_changed.json` proves locked,
  unavailable, and trust-store-changed rows surface specific paused-state
  continuity for registry auth, managed sign-in refresh, and database attach.

## Verification

```sh
cargo test -p aureline-auth secrets --no-fail-fast
cargo test -p aureline-auth --test secret_broker_alpha_cases --no-fail-fast
```
