# Credential picker, credential-state row anatomy, and reveal-on-demand contract

This contract freezes how Aureline explains and selects credential sources
without normalizing raw secret sprawl across auth sheets, registry login,
request workspaces, remote connectors, provider linking, and support/admin
exports.

Companion artifacts:

- [`/docs/auth/credential_state_and_secret_prompt_contract.md`](./credential_state_and_secret_prompt_contract.md)
  defines the canonical `credential_state_record` vocabulary and the
  `secret_access_prompt_record` used whenever a surface needs to read,
  write, rebind, import, unlock, or step-up.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  freezes secret classes, projection modes, redaction boundaries, and
  reveal-on-demand rules.
- [`/schemas/auth/credential_picker_state.schema.json`](../../schemas/auth/credential_picker_state.schema.json)
  defines `credential_picker_state_record`.
- [`/fixtures/auth/credential_picker_cases/`](../../fixtures/auth/credential_picker_cases/)
  contains worked examples for provider linking, package registry auth,
  request-workspace auth, remote connector auth, and policy/store-failure
  picker states.

If this document disagrees with the upstream architecture and UX specs,
the upstream specs win and this document plus the companion schemas and
fixtures must be updated in the same change.

## Principles

1. Users never have to guess whether Aureline stored raw secret material
   in a secure store boundary, holds only a handle/reference, is using a
   delegated identity, or has no usable source.
2. Storage-mode vocabulary is controlled and reused verbatim across UI,
   support exports, and audit trails: `system_credential_store`,
   `enterprise_secret_store`, `session_only`, `handle_only`, `delegated`,
   `not_configured`.
3. The picker never renders raw secret material. It renders only
   export-safe labels, opaque refs, scope labels, and policy/expiry
   posture.
4. Reveal-on-demand is exceptional, high-friction, per-handle, audited,
   clipboard-bounded, and governed by the secret-class matrix and policy.
5. All sources remain attributable: the product can explain which source
   supplied authority (store class + alias/handle ref + origin cues)
   without disclosing secret bytes.

## Credential-state row anatomy (published)

Every credential-bearing surface renders one credential-state row backed
by a `credential_state_record`. The row anatomy is stable even if the
surface is a settings table, a sheet, a status strip, a registry login
prompt, a request environment inspector, or a remote-attach flow.

The row MUST surface the fields below (labels may localize; the meaning
must not drift):

1. **Storage mode** (`authority.storage_mode_class`)
   - Display the controlled storage-mode value and (where helpful) a
     human label.
   - Pair it with the store/source class (`authority.store_source_class`)
     so users can tell whether the boundary is OS keychain, enterprise
     vault, policy injector, browser/device-code handoff, reference-only
     config, remote session broker, or no secure store.
2. **Scope** (`subject.scope_label`, `subject.target_label`)
   - Identify what the credential is for (target boundary) and what it
     is scoped to (workspace, tenant, registry host, remote target, etc).
3. **Expiry/refresh state** (`state_class`, `lifetime.expires_at`,
   `lifetime.refresh_after`)
   - Expired, revoked, rotated-successor-required, and store-unavailable
     states must be visually distinct from absent/missing.
4. **Auditability** (`diagnostics.audit_event_refs`, plus store/source
   and origin cues)
   - Provide an inspect path that shows metadata-only audit event refs
     and the declared store/source class. Do not expose secret bytes.
5. **Revoke/remove/repair action**
   - The row must include a safe primary action plus fallbacks
     (`safe_actions.primary_action`, `safe_actions.fallback_actions`)
     such as revoke/rotate/rebind/import/unlock, and must name what
     remains usable if the action is declined.
6. **Authority boundary (raw vs handle vs delegated vs none)**
   - The row must state one of:
     - secure-store material (OS keychain / enterprise vault boundary),
     - handle/reference only (no raw long-lived material in workspace or
       exports),
     - delegated identity (service-issued or policy-minted authority),
     - no usable source (`absent` / `not_configured`).
   - This is derived from `authority.storage_mode_class`,
     `authority.store_source_class`, and `authority.credential_origin_class`.

## Vault/keychain picker contract (published)

The vault/keychain picker is the selection surface that answers:

- **Which source will supply the credential?**
- **What is the access scope and origin?**
- **What is the reveal policy (if any)?**
- **What portability/export posture applies?**
- **What is the safe rotation/rebind path?**

The picker emits exactly one `credential_picker_state_record` describing
the subject being credentialed and the candidate sources.

### Picker option classes (frozen)

Each option MUST declare:

- `storage_mode_class` — the controlled storage-mode vocabulary above.
- `store_source_class` — the store/source class vocabulary from the
  credential-state contract (for example `os_keychain`,
  `enterprise_vault_adapter`, `file_backed_secret_ref`,
  `managed_policy_injector`, `browser_device_code_handoff`,
  `no_secure_store`).
- `credential_origin_class` — origin cues that keep local secret
  handles, imported references, delegated identity, policy-injection,
  and handoff acquisition distinct.
- `export_rules` — mechanical denial of raw-secret export, and the
  declared posture for support and automation exports.
- `safe_actions` — rotate/rebind/import/unlock/escalate actions that are
  honest for this option.

The picker MUST distinguish these user-visible option classes (display
labels may localize):

- **System credential store** — OS keychain-backed; long-lived material
  lives in the OS boundary. Rotation/revoke is available; reveal (if
  allowed) is governed by the secret class + policy.
- **Enterprise secret store** — enterprise vault-backed; may be policy
  owned. Rotation/rebind may require admin or policy admission.
- **Session-only** — in-memory only; must be visibly degraded and
  re-prompted after restart. Never a stealth fallback for long-lived
  credentials.
- **Workspace variable / reference** — reference-only; the workspace may
  carry aliases/handle refs, not raw secret bytes. Raw secrets observed
  in workspace state are represented as a denial/repair posture, not as
  a selectable "valid" source.
- **Environment passthrough** — reference-only; environment layers may
  carry handle refs or policy-owned injections, not raw literals in
  portable exports.
- **Delegated identity** — service-issued or policy-minted authority
  scoped and expiring, with an explicit stop/disable/revoke path.
- **Missing / unavailable / policy-denied** — explicit blocked states
  with repair actions; "blocked" never collapses into "unknown".

### Rotation, rebinding, and source switching (frozen)

1. Switching sources MUST be explicit and attributable (audit refs +
   source label) and MUST NOT silently widen scope.
2. When a scope/tenant/org change invalidates an alias binding, the
   picker must offer `rebind_handle` rather than reusing the old binding.
3. Rotation MUST name which workflows will be impacted and MUST preserve
   local continuity where possible (for example, local editing continues
   while a registry login token rotates).

## Reveal-on-demand, copy, and export behavior (frozen)

Reveal-on-demand rules are governed by the secret broker ADR and the
secret-class matrix (`artifacts/security/secret_class_rows.yaml`).

The picker and row surfaces MUST enforce:

1. Reveal is per-handle, user-initiated, high-friction, time-bounded,
   and audited. Bulk reveal is forbidden.
2. Clipboard projection uses a bounded timer with explicit "clears in N
   seconds" affordance and denies remote clipboard bridges by default.
3. Reveal is never offered for signing-key, device-secret, provider-
   session, or ephemeral-operation-token classes. For SSH keys and
   client certificates, private bytes are never revealed; public-key
   copy may be offered.
4. Default exports (profiles, support bundles, handoff packets, recipes,
   request workspace exports, and ordinary logs) are metadata-only:
   handles/aliases/source labels and audit refs may travel; raw secret
   bytes may not.
5. A picker MUST remain explainable while redacted: it must be possible
   to tell which store/source and which alias/handle supplied authority
   without exposing secret material.

## Browser/device-code handoff tie-ins (frozen)

When browser or device-code handoff is part of acquiring authority:

- The picker represents it as an acquisition path that returns a handle,
  not as a hidden credential type.
- Device codes, callback query strings, and raw provider payloads are
  never serialized into picker state.
- The picker may reference the handoff by opaque `browser_handoff_ref`
  and expiry metadata only, aligning with:
  - [`/docs/auth/system_browser_callback_packet.md`](./system_browser_callback_packet.md)
  - [`/docs/auth/auth_handoff_interstitial_contract.md`](./auth_handoff_interstitial_contract.md)

## Fixture coverage

The fixture corpus under `fixtures/auth/credential_picker_cases/`
demonstrates:

- provider linking (including browser/device-code acquisition paths);
- package-registry auth selection and rotation cues;
- request-workspace auth selection using reference-only environment
  sources;
- remote connector auth selection including delegated identity vs local
  handle distinction; and
- explicit policy-denied and store-unavailable states with safe repair
  actions.

