# ADR 0007 — Secret broker, credential handles, trust-store classes, and redaction defaults

- **Decision id:** D-0013 (see `artifacts/governance/decision_index.yaml#D-0013`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-08-01
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** security_trust_review
- **Related requirement ids:** none

## Context

Secrets are the fastest place a developer tool can lose trust without
corrupting any file: an AI key in a log line, a database password in a
support bundle, a refresh token copied into a recipe manifest, an SSH
key bytes projected into an environment variable that a crashed child
process dumped to the console. The product's value story — safer AI,
safer remote, safer databases, safer registries, safer signed releases —
only survives if every later lane that touches a secret (provider
adapter, registry client, database connector, remote agent, signing
runner, importer, scaffolder, evidence packer, support-bundle exporter,
crash reporter, replay / timeline, AI context builder) reads secrets
through the same broker, requests the same typed handles, declares the
same projection mode, and redacts to the same defaults.

The freeze matters now, ahead of the provider, registry, update, and
remote lanes landing: if those lanes proliferate before a shared secret
vocabulary is frozen, each will invent its own "short-lived token" or
"handle" shape and the redaction corpus will have to be retrofitted
lane by lane. Later identity, release, and replay work would then land
with incompatible assumptions about who may reference a secret, who may
reveal it, how long a handle lives, what persists across a restart, and
what leaks through a support export. This ADR closes `D-0013` (secret
broker, credential handle, trust store, and redaction defaults) so the
provider, registry, database, remote, signing, AI, importer, support-
export, crash, and mutation-journal lanes can instrument against one
contract.

The secret broker rides alongside the ADR-0001 identity modes (managed,
self-hosted, account-free local), the ADR-0004 RPC transport (no raw
secret bodies crossing the wire; handle references only), the ADR-0005
subscription envelope (every secret-affecting frame carries a redaction
class), and the ADR-0006 VFS save contract (save manifests name
secret-reference class only, never raw secret bytes). This ADR does not
redefine those contracts; it defines the secret fields they refer to.
Full keychain, enterprise-vault, and HSM integration are explicitly
out of scope; this freeze establishes the vocabulary and invariants
those later integrations will honour.

## Decision

Aureline freezes a single **secret broker contract**: every secret
flows through a named broker that issues **typed credential handles**,
binds them to a **trust-store class**, admits a declared **projection
mode** under an **operation-coupled approval ticket**, and emits every
observable action through a **stable audit event** with a declared
**redaction class**. No surface, adapter, extension, AI tool, recipe
step, or CLI invocation may obtain, persist, or project a raw secret
value except through this pipeline. Long-lived secrets MUST NOT fall
back to plaintext storage; the only acceptable fallback when a secure
store is unavailable is a visibly-degraded **session-only, in-memory,
process-scoped** cache for the current process life, after which the
next request re-prompts.

All rules below are stated in terms of contract, vocabulary, and event
names rather than specific crates so adapter changes are hygiene, not
re-litigation.

### Credential handle vs raw secret material

Every secret lives in one of two shapes. The product manipulates the
first; the second exists only inside the broker and inside the narrow
projection boundary the broker permits.

| Shape                  | Who may hold it                                              | Lifetime                                 | Transport rule                                          |
|------------------------|--------------------------------------------------------------|------------------------------------------|---------------------------------------------------------|
| `credential_handle`    | Any consumer in any process under its declared scope.        | Broker-issued; revocable at any moment.  | Safe on RPC, safe in manifests, safe in recipes, safe to log by id. |
| `raw_secret_material`  | Broker and the narrowly-scoped projection target only.       | Operation-scoped or session-scoped only. | Never on RPC, never in manifests, never in recipes, never in logs. |

A `credential_handle` carries:

- `handle_id` — opaque, stable for the handle's life, safe to log.
- `secret_class` — one of the frozen classes below.
- `consumer_identity` — which adapter / provider / driver is allowed
  to resolve the handle, with capability hash.
- `operation_class` — one of `read`, `sign`, `decrypt`, `exchange`,
  `inspect_metadata` (metadata-only reveal), `export_metadata`.
- `target_ref` — endpoint, host identity, registry, workspace, or
  workspace-scoped session the handle is bound to.
- `workspace_scope` — workspace or root id the handle applies under;
  unscoped ambient handles are forbidden on protected surfaces.
- `projection_mode` — see "Projection modes" below.
- `issued_at` / `expires_at` — monotonic issue and expiry.
- `approval_ticket_ref` — ADR-referenced runtime authority ticket that
  admits the use; handles without a ticket are inspect-only.
- `policy_context` — policy epoch, trust state, tenant scope.
- `source_trust_store` — which trust-store class holds the material.
- `redaction_class` — declared redaction class the handle defaults to.

Rules (frozen):

1. Only the broker may mint `credential_handle`s. Adapters request
   handles; they MUST NOT mint, cache, or forward them to unrelated
   consumers.
2. Raw secret material MUST NOT cross the RPC boundary. Handles cross;
   resolution happens at the narrowest possible projection boundary.
3. A `credential_handle` MUST NOT be persisted in workspace files,
   profiles, sync exports, recipes, scaffolds, shell history,
   mutation-journal entries, save manifests, or support bundles.
   Profiles and recipes reference **aliases** (user-stable labels the
   broker maps to a handle at resolution time), not handles.
4. Handle revocation (rotation, expiry, policy change, explicit
   revoke) propagates through a typed event; in-flight operations
   fail closed with `secret_handle_revoked` and offer re-acquire.
5. Handles inherit the ADR-0001 trust-state and ADR-0009 identity-mode
   envelope; a handle issued under a trust state MAY NOT silently
   survive a trust downgrade.

### Secret classes (frozen)

Every handle belongs to exactly one class. The class determines storage
authority, default projection, reveal posture, export rule, and audit
minimum. Adding a class is additive-minor with a schema change;
repurposing a class is breaking.

| Class                    | Typical examples                                              | Default reference / reveal posture                                                                |
|--------------------------|---------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| `ai_provider_token`      | AI provider API keys, gateway keys, BYOK session tokens.      | Reference: any adapter under capability. Reveal: denied by default; on-demand with just-in-time consent and policy. |
| `code_host_token`        | Git host tokens, code-review tokens, artifact-host tokens.    | Reference: adapter under capability. Reveal: denied unless protocol requires raw header bearer.   |
| `package_registry_token` | npm / crates / maven / pip / container registry tokens.       | Reference: registry adapter only. Reveal: denied unless registry protocol requires raw token.     |
| `database_credential`    | DB passwords, DSN tokens, warehouse credentials.              | Reference: connection broker only. Reveal: connection-scoped projection; raw reveal denied.       |
| `ssh_key_material`       | SSH private keys, deploy keys, known-hosts supplements.       | Reference: agent / remote adapter. Reveal: sign-only or agent-forward; raw byte reveal denied.    |
| `client_certificate`     | mTLS client certs and matching private keys, smartcard refs.  | Reference: transport adapter. Reveal: sign-only; raw private-key bytes never revealed.            |
| `signing_key_material`   | Release signing keys, notarization creds, SBOM signers.       | Reference: signing runner / HSM adapter only. Reveal: sign-only; material never user-exported.    |
| `provider_session`       | Refresh tokens, delegated collaboration session tokens.       | Reference: session store. Reveal: never; rotation is broker-internal.                             |
| `device_secret`          | Device-binding keys, recovery tokens, passkey credential refs.| Reference: platform adapter. Reveal: never; sign-only or platform-mediated use.                   |
| `ephemeral_operation_token` | Operation-scoped tokens minted by token-exchange flows.   | Reference: operation-scoped consumer. Reveal: never; expires with the operation.                  |

**Who may reference vs reveal.** Every class names a reference set
(which consumer identities may resolve a handle) and a reveal set
(which consumers may see raw material, and under what additional
gating). A surface that is not named in a class's reveal set MUST NOT
route raw material through its process, even transiently. The matrix
lives in `artifacts/security/secret_class_rows.yaml`; the ADR freezes
the class set and the posture shape, not per-class acronyms.

### Trust-store classes (frozen)

Every class names a storage authority. The broker selects the strongest
authority the host can provide, records the selection, and degrades
visibly when the preferred class is unavailable. Plaintext files are
never an acceptable long-lived store.

| Store class                   | Backed by                                                                           | Unlock model                                             | Fallback posture                                    |
|-------------------------------|-------------------------------------------------------------------------------------|----------------------------------------------------------|-----------------------------------------------------|
| `os_keychain`                 | macOS Keychain Services, Windows Credential Manager / DPAPI, Linux libsecret / kwallet / GNOME Keyring. | Unlocked by platform login; per-item prompts where the platform supports them. | Preferred for user-class secrets on desktop.        |
| `enterprise_vault_adapter`    | Brokered integration to an enterprise secret manager through a declared adapter.    | Enterprise-session token plus policy-bound re-auth.      | Preferred for managed-mode tenants.                 |
| `platform_agent`              | SSH agent, platform smartcard / YubiKey agent, OS biometric agent.                  | Agent-managed; sign-only surface; no raw reveal.         | Preferred for SSH, client-cert, and device classes. |
| `hsm_or_kms_backed`           | Hardware security module or cloud KMS reached via a declared signing surface.       | Service-mediated; sign / decrypt surface only.           | Required for signing-class secrets; no alternative. |
| `session_memory_cache`        | Process-local, in-memory, session-only cache in the broker.                         | Unlocked for the current process life only; never written to disk. | Degraded fallback only. See below.                  |
| `managed_policy_injector`     | Policy engine that materialises a narrow ephemeral credential at call time.         | Approval-ticket-gated per use.                           | Managed-mode only; not a long-lived store.          |

**Unlock states (frozen).** Every trust-store interaction runs in one
of six unlock states. Surfaces render the active state; support
tooling quotes it.

| Unlock state            | Meaning                                                                                                  |
|-------------------------|----------------------------------------------------------------------------------------------------------|
| `locked`                | Store known and present; not currently unlocked. Broker refuses resolution until an unlock flow runs.    |
| `unlocking`             | Unlock flow in progress (platform prompt, enterprise re-auth, agent touch).                              |
| `unlocked`              | Store available; broker may resolve handles under capability.                                            |
| `step_up_required`      | Store requires an additional authenticator (passkey, hardware key, admin re-auth) for this operation.    |
| `degraded_session_only` | No persistent secure store is available; broker is running in session-memory fallback. See posture below.|
| `unavailable`           | Store is known to be unreachable or corrupted. Broker refuses resolution; surfaces explain the reason.   |

**Plaintext fallback is not acceptable.** Long-lived secrets MUST NOT
be written to a plaintext file under any OS, profile, workspace, or
export location, and MUST NOT be injected into persistent environment
state (shell rc files, user-level systemd units, login scripts) by
the broker. When the preferred secure store is unavailable, the only
acceptable fallback is `degraded_session_only`: the broker caches the
material in-process, in memory, for the current process life only,
with a visible, non-dismissable surface badge, an audit event, and a
re-prompt requirement on the next process start. A
`degraded_session_only` state MUST NOT be used for signing-class
secrets (no signing without a stronger store), MUST NOT silently
upgrade to persistent storage when a store returns, and MUST expire
on the first of: process exit, trust-state change, policy-epoch roll,
or explicit user lock.

### Projection modes (frozen)

A projection is the narrow boundary where raw secret material is
allowed to materialise. The broker admits only the modes named below;
a consumer that cannot accept any of them does not get the secret.

| Mode                          | When allowed                                                                      | Required guard                                                                                     |
|-------------------------------|-----------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| `alias_only`                  | Any consumer. Default on all shareable surfaces.                                  | No raw material; handle alias only. Safe for recipes, profiles, manifests, logs, support bundles.  |
| `broker_callback`             | SDK / adapter supports deferred resolution.                                       | Broker resolves on call; raw value never returns to the consumer process after use.                |
| `request_header_signer`       | HTTP / gRPC protocols that need raw bearer material in a header.                  | Broker owns the outbound request construction; raw header never logged.                            |
| `ephemeral_fd`                | Legacy CLI / library requires file material.                                      | Broker provides a short-lived file descriptor or anonymous file; no long-lived file on disk.       |
| `bounded_mount`               | Containerised or agent flow requires file-system-visible material.                | Mount is tmpfs-class, lifetime-bound to the operation, unmounted on completion or failure.         |
| `env_var_isolated_child`      | Last-resort compatibility with a legacy CLI invoked in an isolated child process. | Var is process-scoped for that child, short-lived, scrubbed from support bundles, never inherited. |
| `sign_only`                   | HSM, agent, smartcard, or signing surface.                                        | Consumer sends data to sign; broker or agent returns signature. Raw private bytes never leave.     |
| `decrypt_only`                | HSM, agent, or KMS-backed decrypt surface.                                        | Consumer sends data to decrypt; broker returns plaintext. Raw private bytes never leave.           |
| `token_exchange`              | OAuth2-class flows minting a narrower credential.                                 | Broker exchanges handle for a `ephemeral_operation_token`; the narrower handle replaces the source.|
| `policy_materialised`         | Managed policy injector mints an ephemeral credential per call.                   | Injector returns a `broker_callback`-style handle; original credential never projects.             |
| `inspect_metadata`            | Diagnostics / Doctor / support flows inspecting state.                            | Returns class, alias, trust-store, unlock state, last-failure code only. No raw material.          |
| `reveal_on_demand`            | User explicitly requests reveal in a UI surface.                                  | See "Clipboard / reveal-on-demand" below.                                                          |

Any projection that cannot name one of the modes above is rejected by
the broker with `projection_denied_unsupported_mode`.

### Process-boundary constraints (frozen)

1. Raw secret bytes MUST NOT cross an RPC boundary. Handle references,
   aliases, and capability-scoped exchange tokens cross; projection is
   resolved on the narrowest side.
2. The broker runs in the host process with the strongest trust-store
   authority. Remote-agent brokers carry an attach-time `broker_scope`
   and MAY NOT serve handles bound to the host workspace or to a
   different agent scope.
3. Extension processes and untrusted child processes MUST NOT receive
   raw secret material unless a projection mode explicitly permits it
   and the handle's `consumer_identity` names the extension's capability
   hash. Extensions reach secrets through the broker adapter surface.
4. AI tool calls receive handles or aliases only; a tool call that
   requests raw secret bytes is denied with `ai_tool_reveal_denied` and
   an audit event.
5. Crash dumps and core files MUST NOT inherit projected environment
   variables by default. Child-process env projection uses
   `env_var_isolated_child` with coredump redaction enabled on
   supported platforms.
6. Mutation-journal entries and save manifests name secret-reference
   class, alias, and approval-ticket ref; they MUST NOT name raw
   bytes, raw headers, or raw env-var values.

### Redaction defaults (frozen)

Every surface that emits observable state declares a redaction class.
The broker-owned redaction pass runs before bytes reach any persistent
or exportable sink; lanes MAY NOT disable it to "see the real value"
in a shared surface.

| Surface                              | Default redaction rule                                                                                                                                      |
|--------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | Redact raw secret values, known secret-shape strings, known header / query / body fields; leave handle ids, aliases, consumer ids, class, and failure codes.|
| `traces_local`                       | Same as `logs_local` plus trace-attribute sanitiser; span names MAY NOT include raw material.                                                               |
| `support_bundle`                     | Include only class, alias, trust-store class, unlock state, projection mode, issue / use / failure counts, last failure code. Raw material excluded.        |
| `evidence_packet`                    | Include class, alias, signed proof ref, and the signature or attestation; raw private-key material never included, even under signing-class classes.        |
| `ai_context_capture`                 | Prompt, tool call, and retrieval log: strip raw secrets before retention; mark redacted spans with class and alias; never durable-cache raw material.       |
| `recipe_manifest`                    | Alias references only. Raw secret literals forbidden on protected recipe surfaces.                                                                          |
| `profile_export` / `sync`            | Alias references and posture metadata only. Raw material never exported.                                                                                    |
| `crash_dump`                         | Opt-in only, high-friction; redaction scan precedes packaging; denied by default for signing-class and provider-session handles.                            |
| `mutation_journal_entry`             | Secret-reference class, alias, approval-ticket ref, and projection mode only. No raw bytes.                                                                 |
| `save_manifest` (ADR 0006)           | Same as `mutation_journal_entry`.                                                                                                                           |
| `replay_or_timeline_capture`         | Imported and replayed frames carry secret-reference class and alias only; raw material is excluded at capture time and cannot be promoted on replay.        |
| `terminal_transcript`                | Redact known env-var injections, known command-line token shapes, agent prompt echoes; boundary-labelled raw-paste confirmation required before capture.   |
| `clipboard_projection`               | See "Clipboard / reveal-on-demand".                                                                                                                         |

**Override rules.** Redaction defaults are narrowable by admin policy
(stricter), not wider. Policy MUST NOT silently widen collection
beyond the class rules; any widening happens through a declared
policy bundle that itself redacts to its class. A user-initiated
reveal (see below) is per-operation, audited, and narrowly-scoped; it
MUST NOT flip a surface's default redaction class.

### Logs, support bundles, evidence packets, local developer tooling

- **Logs (local).** Structured logs include handle id, alias,
  consumer id, class, projection mode, and typed failure code; they
  MUST NOT include raw values. The broker exposes a structured
  `SecretEvent` carrier; free-form string interpolation of handles
  into log bodies is forbidden on protected surfaces.
- **Support bundles.** The support-bundle exporter includes the
  broker-state block (class counts, alias registry with aliases only,
  trust-store class and unlock state, last-issue / last-use / last-
  failure summary). It MUST NOT include raw secret material, raw
  environment variables known to hold secrets, raw client certs or
  private keys, raw request headers, or any projection output. Users
  previewing a bundle see the broker block in its final exported
  shape; there is no "include raw" affordance.
- **Evidence packets (release / provenance / attestation).** Evidence
  packets carry signatures and attestations produced by `sign_only`
  or `decrypt_only` projections. The packet carries the signer
  identity, the signing authority class, and the signature bytes;
  the packet MUST NOT carry raw signing-key material, raw
  notarization credentials, or raw release-publish tokens.
- **Local developer tooling.** Doctor probes, diagnostics inspectors,
  and CLI subcommands use `inspect_metadata` projection only; they
  surface class, alias, store class, unlock state, and failure codes.
  A developer CLI MUST NOT print raw secrets even on a local terminal
  unless the user invokes an explicit `reveal_on_demand` affordance.

### Clipboard / reveal-on-demand behaviour

Reveal on demand is the only path where a user sees raw secret bytes
outside a protected projection. It is high-friction by design.

Rules (frozen):

1. A reveal affordance MUST be tied to a specific `handle_id` and a
   specific user intent; bulk reveal is forbidden.
2. Reveal requires an approval ticket bound to the handle, a fresh
   re-authentication where policy requires step-up, and an audible /
   visible indicator for the reveal duration.
3. Revealed material is **session-ephemeral**: it lives in a bounded
   UI buffer for a bounded duration (class-specific, short by
   default), is not written to logs, traces, or support bundles, and
   is cleared on focus change, window blur, timeout, or explicit
   dismiss.
4. Clipboard projection of revealed material uses a
   `clipboard_projection` mode with: a short clipboard-lifetime
   timer, a clear-on-timeout action, a boundary-label indicator
   ("raw secret on clipboard — clears in N seconds"), and a denied-
   on-remote-clipboard-bridge default. OSC-52 or equivalent remote
   clipboard bridges are denied by default for secret classes;
   admin policy MAY widen only through an explicit allowlist.
5. A reveal MUST NOT be offered for `signing_key_material`,
   `device_secret`, `provider_session`, or `ephemeral_operation_token`.
   For `ssh_key_material` and `client_certificate`, reveal surfaces
   the metadata and offers a copy-public-key affordance only; raw
   private-key bytes are never revealed through the UI.
6. AI agents, extensions, recipes, and remote sessions MUST NOT
   trigger reveal on the user's behalf; a reveal is a user action
   initiated from a trusted surface.

### Denial posture

When the broker cannot safely project, it denies. Denial is typed,
visible, auditable, and repairable.

| Denial reason                                 | Meaning                                                                                                          |
|-----------------------------------------------|------------------------------------------------------------------------------------------------------------------|
| `secret_handle_unknown`                       | Alias resolves to no handle under the current consumer / scope / trust state.                                    |
| `secret_handle_revoked`                       | Handle exists but was revoked (rotation, policy change, explicit revoke). Consumer MUST re-acquire.              |
| `secret_handle_expired`                       | Handle past `expires_at`. Consumer MUST re-acquire.                                                              |
| `trust_store_locked`                          | Trust store known but `locked`. Surface offers unlock.                                                           |
| `trust_store_unavailable`                     | Trust store unreachable / corrupted. Surface explains the reason; no silent fallback to plaintext.               |
| `step_up_required`                            | Operation requires additional authenticator.                                                                     |
| `approval_ticket_missing`                     | No live runtime-authority ticket admits the projection. Surface routes to approval.                              |
| `policy_denied_projection`                    | Policy forbids the requested projection mode for this class / consumer / scope.                                  |
| `projection_denied_unsupported_mode`          | Consumer requested a projection mode not in the frozen set.                                                      |
| `projection_denied_class_rule`                | Class rules deny the requested reveal or export (for example, raw reveal on signing-class).                      |
| `consumer_capability_mismatch`                | Consumer identity does not match the handle's named `consumer_identity` / capability hash.                       |
| `trust_state_downgraded`                      | Handle's trust state was downgraded after issuance; handle invalidated.                                          |
| `ai_tool_reveal_denied`                       | AI tool call requested raw reveal; denied by default.                                                            |
| `remote_broker_scope_mismatch`                | Remote-agent broker scope does not cover the requested handle.                                                   |
| `session_fallback_class_forbidden`            | `degraded_session_only` cannot serve this class (for example, `signing_key_material`).                           |

Denials fail closed; they MUST NOT silently retry, MUST NOT downgrade
projection to a weaker mode, and MUST emit a `secret_denial` audit
event.

### Audit events (frozen)

Every observable broker action emits a structured event on the
`secret_broker` audit stream. Events carry the actor, handle id,
alias, class, consumer id, target ref, projection mode, unlock
state, approval-ticket ref, trust state, and a typed reason where
relevant. Events MUST NOT carry raw secret bytes.

| Event id                              | Fires when                                                                                    |
|---------------------------------------|-----------------------------------------------------------------------------------------------|
| `secret_handle_issued`                | Broker issues a new handle.                                                                   |
| `secret_handle_renewed`               | Broker extends or re-issues a handle under existing approval.                                 |
| `secret_handle_revoked`               | Handle is revoked (rotation, policy, explicit user or admin revoke).                          |
| `secret_handle_expired`               | Handle passes `expires_at`.                                                                   |
| `secret_projection_used`              | A projection mode is exercised against a handle.                                              |
| `secret_projection_failed`            | A projection attempt fails (with typed reason).                                               |
| `secret_denial`                       | Broker denies a request (any denial reason above).                                            |
| `secret_reveal_started` / `_ended`    | Reveal-on-demand flow starts / ends.                                                          |
| `secret_clipboard_projected` / `_cleared` | Clipboard projection of revealed material opens / clears.                                 |
| `secret_trust_store_unlocked` / `_locked` | Trust store unlock state changes.                                                         |
| `secret_trust_store_degraded`         | Broker transitions to `degraded_session_only` fallback.                                       |
| `secret_trust_store_recovered`        | Broker exits fallback to a persistent store.                                                  |
| `secret_rotation_started` / `_completed` / `_failed` | Rotation flow phases.                                                          |
| `secret_import_mapped`                | Importer / migration mapped a source-tool secret reference onto a broker alias.               |
| `secret_export_redaction_applied`     | Export surface applied redaction; names the class counts, not raw values.                     |
| `secret_policy_epoch_rolled`          | Policy epoch rolled; dependent handles invalidated.                                           |
| `secret_broker_remote_attached` / `_detached` | Remote-agent broker scope attaches / detaches.                                        |
| `secret_broker_health_changed`        | Broker health transitions (`healthy` / `degraded` / `unavailable`).                           |

Support tooling subscribes to this stream under `inspect_metadata`;
it never receives raw values.

### Support-bundle inclusion and exclusion (explicit defaults)

| Surface                                      | Default inclusion                                                                              | Overrides                                                                       |
|----------------------------------------------|------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------|
| Broker health summary                        | Included (metadata only: health state, store class, unlock state, counts).                     | None needed; already redacted.                                                  |
| Handle alias registry                        | Included (aliases, classes, consumer ids, last issue / last failure counts).                   | None needed.                                                                    |
| Raw handle ids                               | Excluded.                                                                                      | Admin policy MAY include hashed handle ids for correlation; raw ids remain out. |
| Raw secret bytes                             | Excluded.                                                                                      | No override. Plaintext in a bundle is always a defect.                          |
| Environment variables known to hold secrets  | Excluded by default; names included, values redacted to `[redacted: <class>]`.                 | Admin policy MAY widen env capture for a narrow allowlist; values stay redacted.|
| Recent projection events                     | Included under `inspect_metadata` (class, mode, result, reason).                               | None needed.                                                                    |
| Recent denial events                         | Included with typed reason.                                                                    | None needed.                                                                    |
| Reveal-on-demand events                      | Included (started / ended with durations) without raw material.                                | None needed.                                                                    |
| Clipboard projection events                  | Included (opened / cleared).                                                                   | None needed.                                                                    |
| Trust-store unlock traces                    | Included (state transitions); raw prompts and authenticator material excluded.                 | None needed.                                                                    |
| Rotation outcomes                            | Included (started / completed / failed with typed reason).                                     | None needed.                                                                    |
| Signing-class projection payloads            | Excluded entirely from bundles; release evidence lives in the evidence packet surface instead. | No override.                                                                    |

Overrides (where allowed) are narrowing only; they MAY reduce
collection beyond defaults but MAY NOT silently widen it beyond a
class's exclusion rules. Every override is itself audited under
`secret_export_redaction_applied`.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- Full keychain and enterprise-vault integration. This ADR freezes
  the contract those integrations will satisfy; the integrations
  themselves land under later rows.
- Full HSM / KMS integration for signing flows. This ADR freezes
  `sign_only` / `decrypt_only` projection and the signing-class
  class set; the HSM / KMS adapter ships under the release / signing
  lane.
- Passkey / WebAuthn enrolment and recovery flows. This ADR freezes
  `device_secret` class and `step_up_required` unlock state; the
  enrolment UX lands under the identity lane.
- The complete AI-context redaction matrix. This ADR freezes the
  redaction classes AI capture uses; the per-provider prompt-capture
  rules land under the AI lane.
- Per-platform coredump configuration. This ADR freezes that
  crash-dump capture MUST NOT inherit projected env by default; the
  platform-specific coredump flags land under the platform adapter.
- Full public-SDK stability of handle envelopes. These stay internal
  at the foundations milestone; the public SDK surface lands under a
  later decision row.

These lines move only by opening a new decision row, not by editing
this ADR.

### Tradeoff table

The structured tradeoff rows live in
`artifacts/security/secret_broker_tradeoff_rows.yaml`. Headline summary:

| Axis                                 | Chosen stack                                                                                                                | Best rejected alternative                                               | Why chosen wins                                                                                                         |
|--------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------|
| **Handle vs raw material**           | Typed credential handles; raw material confined to narrow projection boundary                                               | "Pass the token directly" wherever a function needs it                  | Direct passing leaks through RPC, logs, manifests, and crash dumps; handles keep raw material inside the broker only.   |
| **Trust-store authority**            | Named store classes with preferred-to-fallback ordering; plaintext files never acceptable for long-lived secrets            | Accept a plaintext config file when no store is available               | Plaintext storage on disk is the primary leak path. The session-only fallback keeps the user working without the leak.  |
| **Projection modes**                 | Enumerated modes, last-resort env via isolated child only                                                                   | Allow any consumer to ask for raw value "if it needs it"                | Unbounded env projection is the leading support-bundle leak class. Enumerated modes name the guard for every shape.     |
| **Redaction default posture**        | Declared redaction class per surface; default excludes raw material; overrides narrow only                                  | "Include everything; let the user redact on upload"                     | The support preview-and-redact flow is a secondary control; the primary control is collection-time exclusion.           |
| **Reveal on demand**                 | User-initiated, per-handle, high-friction, bounded-clipboard, denied for signing / session / device classes                 | Always-on reveal in the developer UI                                    | Reveal must be exceptional; the bounded-clipboard + audit shape gives power-user reveal without leaking into bundles.   |
| **Denial vs silent downgrade**       | Fail closed with a typed denial reason                                                                                      | Downgrade projection to a weaker mode on failure                        | Silent downgrade turns a refusal into a leak. Typed denial gives the user a visible repair path.                        |
| **Audit surface**                    | Structured `SecretEvent` stream with frozen event ids                                                                       | Free-form log strings in each lane                                      | Free-form logs re-introduce format drift and unredacted interpolation; the event stream keeps every lane on one vocab.  |
| **Schema of record**                 | Rust types in the eventual secret-broker crate; JSON Schema export at `schemas/security/secret_class_rows.schema.json`      | External IDL + codegen at this milestone                                | No second-language consumer yet; the JSON Schema export reserves a clean integration point.                             |

Each row carries reopen triggers in the YAML. A redaction-corpus
finding that a support bundle leaked a raw value, an audit finding
that a projection mode slipped past the broker, or a benchmark
finding that a hot-path surface cannot instrument `secret_projection_used`
within budget reopens the relevant row.

### Redaction-example fixtures

A small corpus of redaction fixtures lives under
`fixtures/security/redaction_examples/`. They are short, reviewable
scenarios (local log redaction, support-bundle inclusion /
exclusion, evidence-packet signature with key exclusion, clipboard
reveal-on-demand audit, denial posture when no safe projection
exists, `degraded_session_only` fallback) used by the broker, AI
lane, support-export lane, evidence-packet lane, and mutation-
journal lane to anchor the class names, the projection modes, the
unlock states, the denial reasons, and the audit event ids to
concrete inputs and observable outcomes. They are not a test suite;
they are the language the ADR's class matrix and event list refer to.

## Consequences

- **Frozen:** the secret-class set (`ai_provider_token`,
  `code_host_token`, `package_registry_token`, `database_credential`,
  `ssh_key_material`, `client_certificate`, `signing_key_material`,
  `provider_session`, `device_secret`, `ephemeral_operation_token`),
  the trust-store class set (`os_keychain`,
  `enterprise_vault_adapter`, `platform_agent`, `hsm_or_kms_backed`,
  `session_memory_cache`, `managed_policy_injector`), the unlock-
  state set (`locked`, `unlocking`, `unlocked`, `step_up_required`,
  `degraded_session_only`, `unavailable`), the projection-mode set
  (twelve modes above), the denial-reason set, and the audit-event
  id set.
- **Frozen:** long-lived secrets MUST NOT fall back to plaintext on
  disk or in persistent environment state. The only acceptable
  fallback is `session_memory_cache` with the
  `degraded_session_only` unlock state, visibly degraded, short-
  lived, and forbidden for signing-class secrets.
- **Frozen:** raw secret material MUST NOT cross the RPC boundary.
  Handles and aliases cross; projection resolves at the narrowest
  boundary.
- **Frozen:** the default redaction posture per surface. Overrides
  narrow; they do not widen. Every export surface inherits the
  surface's default redaction class without restating it.
- **Frozen:** the schema of record is Rust types in the eventual
  secret-broker crate; the boundary schema lives at
  `schemas/security/secret_class_rows.schema.json`; there is no
  external IDL or codegen toolchain at this milestone. This mirrors
  ADR 0004, ADR 0005, and ADR 0006.
- **Frozen:** reveal-on-demand is the only user-visible raw-reveal
  surface. It is per-handle, bounded, audited, denied for signing /
  session / device classes, and cannot be triggered by AI agents,
  extensions, recipes, or remote sessions on the user's behalf.
- **Permitted:** adding a new secret class, a new trust-store class,
  a new projection mode (with a guard rule), a new denial reason,
  or a new audit event id is an additive-minor change with a schema
  bump and a row in the class matrix; repurposing any existing
  value is breaking and requires a new decision row.
- **Permitted:** admin policy MAY narrow collection further on any
  surface. Policy MAY NOT silently widen collection beyond a
  class's frozen redaction rule; any widening lands through a
  declared policy bundle that itself redacts to its class.
- **Follow-up:** the provider, registry, database, remote-agent,
  signing, AI, importer, support-export, crash, and mutation-
  journal lanes instrument every broker event and respect every
  frozen class / projection / redaction rule before claiming secret-
  handling guarantees.
- **Follow-up:** the eventual keychain, enterprise-vault, and HSM /
  KMS integrations open follow-on decision rows that ride this
  contract rather than reshape it.
- **Follow-up:** the release / evidence lane consumes `sign_only`
  and `decrypt_only` projections through the signing-key-material
  class and carries signatures / attestations in evidence packets;
  the packet never carries raw signing-key bytes.
- **Ratifies:** the ADR-0005 subscription envelope's authority-class
  `policy_entitlement` now refers to the policy-epoch and trust-
  state fields frozen here. The ADR-0004 frozen error taxonomy's
  `policy` class absorbs the denial-reason set above as typed
  subcodes. The ADR-0006 save manifest's secret-reference fields
  name the class / alias / projection-mode / approval-ticket shape
  frozen here. The ADR-0001 identity modes (managed, self-hosted,
  account-free local) remain the envelope the broker's
  `trust_state` and handle scope live inside.

## Alternatives considered

- **Pass raw secrets directly wherever a function needs them.**
  Rejected: direct passing leaks raw material into RPC frames,
  structured logs, trace attributes, recipe manifests, sync
  exports, crash dumps, and terminal transcripts. The handle
  shape is what keeps raw material inside the broker while still
  letting every consumer read it through a narrow projection.
- **Accept a plaintext fallback file when no store is available.**
  Rejected: plaintext-on-disk is the primary leak path for
  developer-tool secrets (backups, file-sync providers, support
  bundles, shell-history accidents). The `degraded_session_only`
  fallback keeps the user working without the leak and forces a
  visible re-prompt every session.
- **Let any consumer request a raw value "if it needs it".**
  Rejected: unbounded projection is the leading support-bundle and
  log-leak class. The enumerated projection modes each name the
  guard (isolated child process, tmpfs mount, sign-only, token
  exchange) that keeps the raw material inside a boundary.
- **"Include everything; let the user redact at upload."**
  Rejected: the bundle preview is a secondary control at best and
  imposes an unrealistic review burden. Collection-time exclusion
  is the primary control; preview-time narrowing layers on top.
- **Always-on reveal in the developer UI.** Rejected: always-on
  reveal normalises raw material on screen, in recordings, and in
  screenshots. Per-handle, user-initiated, bounded reveal keeps
  the power-user path without flattening the posture for everyone.
- **Downgrade to a weaker projection on failure.** Rejected:
  silent downgrade turns a refusal into a leak (for example,
  falling back from `sign_only` to `env_var_isolated_child`
  silently ships a signing key into a child process). Typed
  denial with a visible repair path is the only auditable posture.
- **Free-form log strings in each lane.** Rejected: free-form
  interpolation routinely interpolates handles, tokens, or headers
  into log bodies and bypasses the redaction pass. The structured
  event stream keeps every lane on one vocabulary and one
  redaction pass.
- **External IDL + generator for the handle / projection
  payload.** Rejected: same argument ADR 0004, ADR 0005, and
  ADR 0006 make — an IDL without a second-language consumer costs
  more than it buys; the JSON Schema export reserves the
  integration point.
- **Defer to a later milestone.** Rejected: the default-if-
  unresolved narrowing on `D-0013` ("single projection mode
  (`alias_only`), no reveal affordance, no clipboard projection,
  no session-only fallback, `os_keychain` on desktop only") would
  block remote-agent, enterprise-vault, HSM, AI tool-call, and
  importer lanes exactly when later work needs the frozen
  vocabulary; the support, release-evidence, and mutation-
  journal lanes would land with incompatible assumptions the
  lane would then have to reconcile.

The `D-0013` `narrow` default-if-unresolved posture would have
locked the broker to a single projection mode, no reveal, no
clipboard, no session-only fallback, and keychain-only storage
until an ADR landed. Accepting this ADR replaces that narrowing
with the frozen class matrix, trust-store classes, unlock states,
projection modes, redaction defaults, reveal-on-demand posture,
denial posture, and audit-event list above; the narrowing default
does not apply.

## Source anchors

- `.t2/docs/Aureline_PRD.md:3069` — "10.22 Diagnostic data classes
  and support-bundle redaction".
- `.t2/docs/Aureline_PRD.md:3078` — "Secret-bearing / high risk ...
  never included by default; inclusion requires explicit, high-
  friction opt-in or is prohibited by policy".
- `.t2/docs/Aureline_PRD.md:3627` — "Secret fishing ... secret-
  aware deny list, path restrictions, redaction layer".
- `.t2/docs/Aureline_PRD.md:4085` — "Support bundles or diagnostics
  leak sensitive data ... default redaction, and high-friction
  inclusion for risky payloads".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4378` —
  "22.11 Secret broker, credential-handle, and vault-integration
  architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4390` —
  "Broker rules".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4391` —
  "raw secret values never land in repo files, portable profiles,
  workspace state, query history, recipe exports, telemetry, crash
  dumps, or support bundles by default".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4392` —
  "integrations request typed handles carrying scope, consumer
  identity, operation class, target, workspace, expiry, and allowed
  projection".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4394` —
  "environment injection is least-visibility by default".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4528` —
  "if the OS keychain or trust store is unavailable, Aureline
  degrades visibly and safely rather than falling back to plaintext
  secret storage".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4611` —
  "secret-broker, vault-adapter, and handle-issuance state" in
  support bundle.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4653` —
  "secret-broker authority classes, handle failure codes, and
  vault-adapter health" in support bundle.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4749` —
  "Secret-broker / vault adapter state ... metadata-only ...
  includes handle classes and failure codes, never raw secret
  values".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:5721` —
  "SEC-CRED-009 ... secret broker, scoped handles, vault adapters,
  and redaction/export boundaries".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:8783` —
  "Appendix BT — Secret Broker, Vault, and Credential-Handle
  Matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:8787` —
  "BT.1 Secret-class matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:8797` —
  "BT.2 Handle projection matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:8807` —
  "BT.3 Default redaction boundaries".

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0013`
- RFC: none.
- Tradeoff register (machine form):
  `artifacts/security/secret_broker_tradeoff_rows.yaml`.
- Secret-class matrix (machine form):
  `artifacts/security/secret_class_rows.yaml`.
- Redaction posture matrix (machine form):
  `artifacts/security/redaction_posture_matrix.yaml`.
- Boundary schema (machine form):
  `schemas/security/secret_class_rows.schema.json`.
- Redaction-example fixtures:
  `fixtures/security/redaction_examples/`.
- Identity-mode envelope this contract rides:
  `docs/adr/0001-identity-modes.md`.
- Transport boundary raw material MUST NOT cross:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`.
- Reactive-truth contract every broker event subscribes through:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`.
- Save manifest contract whose secret-reference fields this ADR names:
  `docs/adr/0006-vfs-save-cache-identity.md`.
- Affected lanes: `governance_lane:security_trust_review`,
  `governance_lane:support_export`,
  `governance_lane:release_evidence`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance. No supersession.
