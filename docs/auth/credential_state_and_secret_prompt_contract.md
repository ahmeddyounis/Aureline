# Credential-state and secret-access prompt contract

This contract publishes the shared vocabulary Aureline uses to explain
credential authority across auth, registries, remote targets, AI
gateways, API request workspaces, database connectors, package actions,
and support/admin exports. It sits above the secret broker ADR: broker
classes define how material is held and projected; this contract defines
the user/admin-facing state row and prompt shape every consuming surface
quotes.

Companion artifacts:

- [`/schemas/auth/credential_state.schema.json`](../../schemas/auth/credential_state.schema.json)
  defines `credential_state_record`, `store_capability_record`, and
  `store_capability_matrix_record`.
- [`/schemas/auth/secret_access_prompt.schema.json`](../../schemas/auth/secret_access_prompt.schema.json)
  defines `secret_access_prompt_record`.
- [`/fixtures/auth/credential_state_cases/`](../../fixtures/auth/credential_state_cases/)
  contains worked examples for locked keychain launch, expired token
  handle, policy-blocked registry auth, missing remote credential,
  rebind after org switch, delegated credentials, browser/device-code
  handoff, and secure-store downgrade to session-only auth.

Upstream contracts this document composes with:

- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  for secret classes, trust-store classes, unlock states, projection
  modes, denial reasons, audit events, and redaction defaults.
- [`/docs/auth/system_browser_callback_packet.md`](./system_browser_callback_packet.md)
  for browser/device-code handoff, callback correlation, and
  credential-store lock-state packet vocabulary.
- [`/docs/auth/managed_auth_and_session_continuity_contract.md`](./managed_auth_and_session_continuity_contract.md)
  for managed-session continuity, org switch, step-up, and local-work
  preservation rules.
- [`/docs/package/package_action_contract.md`](../package/package_action_contract.md),
  [`/docs/api/request_workspace_contract.md`](../api/request_workspace_contract.md),
  and [`/docs/remote/attach_tunnel_port_forward_contract.md`](../remote/attach_tunnel_port_forward_contract.md)
  for the package, request, and remote surfaces that consume this
  vocabulary rather than minting local auth labels.

If this document disagrees with the upstream architecture and UX specs,
the upstream specs win and this document plus the companion schemas and
fixtures must be updated in the same change.

## Principles

1. A credential row answers four questions without exposing material:
   what authority exists, where it lives, who may consume it, and what
   fallback or repair path remains.
2. "Signed in", "configured", or "connected" never implies permanent
   raw-secret authority. The row must name handle, delegated,
   policy-injected, session-only, or unavailable posture explicitly.
3. Prompts are just-in-time, attributable, scope-bounded, and
   export-safe. A prompt must say whether raw material is being
   requested and whether a handle-only path exists.
4. Raw secret material, private keys, refresh tokens, and ambient
   delegated credentials do not appear in product UI, workspace files,
   profiles, recipes, handoff packets, logs, support bundles, AI
   evidence, or ordinary exports by default.
5. Secure-store loss is visible. A fallback to session-only in-memory
   auth is never represented as ordinary availability, and long-lived
   plaintext fallback is forbidden.

## Credential-state vocabulary

Every credential-bearing surface emits one `credential_state_record`.
The `state_class` values are closed vocabulary. Display copy may localize
the label, but the state meaning, capability implication, and safe action
remain stable.

| State | Meaning | Capability implication | Safe retry / escalation |
|---|---|---|---|
| `absent` | No alias, handle, delegated credential, or approved source exists for the requested scope. | The credentialed action cannot run; local work that does not depend on the credential remains available. | Import a credential, start browser/device-code handoff, continue without credential, or contact admin when policy owns the source. |
| `handle_only` | A broker alias or handle is present and consumers receive the handle, not raw material. | Operations may proceed only through permitted projection modes and named consumer identities. Raw reveal stays denied unless a separate high-friction path admits it. | Use the handle, renew if nearing expiry, or rebind if scope/tenant changed. |
| `available` | The credential source is available for the declared scope and projection mode. | The named consumer may run under the declared lifetime, policy epoch, and audit posture. No broader authority is implied. | Proceed, test, rotate, or revoke through bounded controls. |
| `locked` | A known store exists but is locked or waiting for user/platform unlock. | Reads, writes, rebinds, and raw projections are paused until unlock; local non-credential work continues. | Unlock store, continue without credential, or export support metadata. |
| `denied` | The broker or policy denied this request for a typed non-policy failure such as consumer mismatch, missing approval ticket, or unsupported projection. | The requested action fails closed; the product must not silently downgrade to weaker projection. | Request approval, switch to an allowed projection, or inspect diagnostics. |
| `expired` | A handle, delegated credential, session token, or callback window passed its expiry. | Existing cached authority is not reusable; in-flight work that depends on it must stop or re-acquire. | Renew, reauthenticate, restart browser/device-code handoff, or continue with cached/read-only local state where allowed. |
| `revoked` | The issuer, broker, admin policy, or user revoked the authority. | The credential cannot be used again. Dependent handles and remembered decisions are invalidated. | Re-acquire under current policy, rotate/rebind if permitted, or contact the policy owner. |
| `rotated` | The source credential changed and old handles are no longer authoritative. | New operations must bind to the successor alias/handle; old handles fail closed. | Rebind impacted workflows, test the successor, and update queued jobs or remembered decisions. |
| `store_unavailable` | The expected secure store is unreachable, corrupted, unsupported, or unavailable on this host. | Persistent credential operations are blocked. Session-only in-memory fallback is allowed only where policy and class rules permit it and remains visible. | Retry after store recovery, use an approved session-only fallback, or contact support/admin. |
| `policy_blocked` | Current org, workspace, trust, or admin policy forbids the credential source, projection, target, or action. | The action stays disabled until policy changes or an exception is issued. Local-safe alternatives remain visible. | Open policy detail, request an exception, use cached/read-only paths, or contact admin. |

## Storage mode and origin vocabulary

Credential state rows use a small storage-mode vocabulary in product
copy, docs, telemetry, support exports, and admin explanations:

| Storage mode | Meaning | Export posture |
|---|---|---|
| `system_credential_store` | Long-lived material is held by the host OS credential store. | Store class, alias, and state may export; raw material does not. |
| `enterprise_secret_store` | Material is held by an enterprise vault or approved broker. | Store class, handle class, policy owner, and alias may export; raw material does not. |
| `session_only` | Material exists only in the current broker process/session. | Expiry and restart expectations must be visible; raw material does not export. |
| `handle_only` | Consumers receive a handle or alias and never receive material directly. | Handle class, alias, consumer identity, and projection mode may export where allowed. |
| `delegated` | A narrower delegated grant, service-issued identity, or forwarded credential is in use. | Actor class, authority source, target scope, expiry, and revocation path may export; ambient delegated credentials do not. |
| `not_configured` | No usable source exists. | The row exports cause class and repair path only. |

The `credential_origin_class` keeps local, forwarded, delegated,
policy-injected, customer-managed, browser/device-code, and absent
identity distinct because their blast radius and support paths differ.

## Store-capability matrix

Every store or source posture consumed by a credential row also has a
`store_capability_record`. Store capability records are reusable by
remote, registry, provider handoff, request-workspace, and support flows
so each surface does not redefine storage mode or auditability.

| Store source | Storage modes it may represent | Long-lived material | Prompt support | Export/support rule |
|---|---|---|---|---|
| `os_keychain` | `system_credential_store`, `handle_only` | Yes, through platform protection only. | Read, write, unlock, step-up where the platform supports it. | Export state, alias, class, unlock state, and failure code only. |
| `enterprise_vault_adapter` | `enterprise_secret_store`, `handle_only`, `delegated` | Yes, through the enterprise broker/vault only. | Read, write, import by reference, rebind, step-up. | Export store class, policy owner, alias, handle class, scope, and audit refs only. |
| `agent_socket` | `handle_only`, `delegated` | Agent-owned; Aureline does not persist private material. | Read/use by sign-only or broker callback; unlock/step-up may be agent-mediated. | Export agent class, target scope, and sign/decrypt posture; private bytes never export. |
| `file_backed_secret_ref` | `handle_only`, `not_configured` | No raw long-lived material in workspace or portable files. Only references or aliases are admissible. | Import reference, rebind alias, or deny raw material. | Export alias/source label only; any raw material observed in a file is a denial event. |
| `remote_session_scoped_handle` | `session_only`, `delegated`, `handle_only` | Session-bounded only. | Read/use, rebind to remote broker, approve step-up. | Export broker scope, target, expiry, and revocation path; no forwarded raw material. |
| `hardware_backed_or_passkey_adjacent` | `system_credential_store`, `delegated`, `handle_only` | Hardware/platform mediated; raw private material is not exportable. | Unlock and approve step-up; sign/decrypt only where supported. | Export platform class, attestation/ref, and audit state; private keys and device secrets never export. |
| `managed_policy_injector` | `delegated`, `session_only`, `handle_only` | No user-held long-lived material; policy materializes a narrow credential at call time. | Approve step-up or policy admission when required. | Export policy owner, action scope, expiry, and audit refs only. |
| `no_secure_store` | `not_configured`, `session_only` | No. Session-only fallback may exist in broker memory if class and policy allow it. | Import credential, browser/device-code handoff, or continue without credential. | Export downgrade state and repair path only; plaintext fallback is forbidden. |

## Secret-access prompt contract

A `secret_access_prompt_record` is the only prompt shape for secret use.
It carries:

- actor: class, subject ref, surface ref, consumer identity, and whether
  the prompt is user, admin, policy, or system initiated;
- target: target class, target ref, target label, secret class, scope,
  and audience;
- action class: `read_secret`, `write_secret`, `rebind_handle`,
  `import_credential`, `unlock_store`, or `approve_step_up_access`;
- storage and projection: requested storage mode, store source,
  projection mode, handle-only availability, and raw-reveal posture;
- lifetime: operation-scoped, session-only, time-bounded, persistent
  until revoked, or unavailable;
- export posture: whether aliases/source labels may travel, and the
  explicit rule that raw secret material, private keys, refresh tokens,
  and ambient delegated credentials may not;
- decision options: allow once, allow until expiry, allow by policy,
  deny, continue without credential, unlock store, import credential,
  rebind handle, open policy detail, start browser handoff, or use
  device-code handoff; and
- no-raw-secret-echo rules: the prompt must not render, log,
  prepopulate, or support-export raw material.

Prompt actions do not grant ambient future authority. A remembered
decision compiles to a narrow reusable rule plus renewable short-lived
tickets, never to an unlimited bearer credential.

| Prompt action | Used when | Minimum fallback disclosure |
|---|---|---|
| `read_secret` | A consumer needs to resolve an existing alias or handle for an operation. | Whether a handle-only or broker-callback projection can satisfy the request, and what remains available if denied. |
| `write_secret` | A user or admin stores or rotates credential material into an approved store. | The destination store/source class, portability rule, and how to remove or rotate the entry later. |
| `rebind_handle` | Scope, tenant, org, target, or consumer identity changed and an alias must bind to a successor handle. | Which old scope is no longer valid and which queued or remembered work is affected. |
| `import_credential` | A source reference, browser/device-code result, or user-provided credential must become a broker alias. | Whether the import stores material or only a reference, and which raw values will be excluded from export. |
| `unlock_store` | A known store is locked or waiting for platform/agent unlock. | What local work continues while locked and which actions remain blocked until unlock completes. |
| `approve_step_up_access` | Policy, hardware, passkey-adjacent, or admin rules require stronger authentication for this operation. | The target boundary, lifetime, expiry, and deny/continue-local path. |

## Export, support, and automation rules

The schemas require every state and prompt to state export posture
mechanically:

- Handles, aliases, source labels, store/source classes, target classes,
  policy owner refs, expiry classes, denial reasons, and audit refs may
  travel where the row says they are exportable.
- Raw secret material, passwords, private keys, refresh tokens, client
  certificate private bytes, device secrets, raw OAuth codes, raw PKCE
  verifiers, raw request headers, and ambient delegated credentials do
  not travel by default.
- Support exports use metadata-only credential rows: store class,
  source class, alias, handle class, scope, unlock state, expiry state,
  denial reason, and repair path.
- Automation, recipes, run history, and request workspaces may carry
  aliases and required classes. They may not inline raw literals or
  durable handle ids as substitute secrets.
- AI tools and extensions cannot trigger reveal on the user's behalf.
  They receive aliases, handles, or broker callbacks only.
- A raw secret found in workspace state is represented as an audit or
  denial row; it is not normalized into a valid credential source.

## Fixture coverage

The fixture corpus covers the required cases:

- locked OS keychain on launch;
- expired token handle;
- policy-blocked registry auth;
- remote credential missing;
- rebind after org switch;
- explicit delegated credential row;
- explicit browser/device-code handoff row; and
- secure-store downgrade to session-only auth.

Each worked row cites the schema, carries opaque refs only, and includes
the safe fallback/export story that users, admins, and support tooling
can quote without seeing secret material.

## Change control

Adding a new credential state, prompt action, store source, credential
origin, target class, export posture, or safe action is additive-minor
and requires updates to this document, the matching schema, and at least
one fixture in the same change. Repurposing an existing value is
breaking and requires security, auth, support, and the owning product
lane to sign off.
