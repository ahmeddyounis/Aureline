# Native browser, proxy, trust-store, and credential-store conformance packet

This packet freezes **per-OS authoritative integration surfaces** for:

- system-default-browser auth handoff and return;
- system proxy and PAC inheritance (plus explicit override rules);
- OS trust-store integration (plus enterprise CA overlay and client-certificate handling); and
- OS credential-store behavior (available/locked/denied/unavailable and visible session-only degradation).

It exists so platform adapters and feature lanes do not improvise “what counts as
system proxy”, “which browser return route is claim-bearing”, or “what happens
when the secure store is locked” differently per surface.

If this document and
[`/artifacts/platform/native_trust_integration_matrix.yaml`](../../artifacts/platform/native_trust_integration_matrix.yaml)
ever disagree, the YAML matrix wins for tooling and this document must be
updated in the same change.

## Companion artifacts

- [`/artifacts/platform/native_trust_integration_matrix.yaml`](../../artifacts/platform/native_trust_integration_matrix.yaml)
  — machine-readable integration matrix for claimed desktop profiles.
- [`/fixtures/platform/native_auth_trust_cases/`](../../fixtures/platform/native_auth_trust_cases/)
  — reviewer drill cases for browser return, proxy denial, certificate prompts,
  keychain/credential-store lock, and trust-store unavailability.

## Upstream contracts this packet composes with

- [`/docs/platform/desktop_platform_conformance_matrix.md`](./desktop_platform_conformance_matrix.md)
  — the broader claimed-desktop matrix this packet narrows in on.
- [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
  — authoritative roster of claimed desktop profiles.
- [`/artifacts/platform/protocol_handler_ownership_matrix.yaml`](../../artifacts/platform/protocol_handler_ownership_matrix.yaml)
  — scheme ownership model and side-by-side relations (used by browser returns).
- [`/docs/auth/system_browser_callback_packet.md`](../auth/system_browser_callback_packet.md)
  — callback correlation, return-route vocabulary, and preserved-local-work rules.
- [`/docs/auth/auth_handoff_interstitial_contract.md`](../auth/auth_handoff_interstitial_contract.md)
  — browser handoff interstitial and callback-origin review contract.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — trust-store classes, unlock states, and session-only downgrade rules.
- [`/docs/network/transport_governance_packet_seed.md`](../network/transport_governance_packet_seed.md)
  and
  [`/docs/network/transport_explainability_surface_contract.md`](../network/transport_explainability_surface_contract.md)
  — shared proxy/trust/egress vocabulary and how it projects into settings,
  diagnostics, and support export.
- [`/docs/architecture/input_adapter_failure_modes.md`](../architecture/input_adapter_failure_modes.md)
  — shell degraded-state posture when keychain/secret store/trust store is
  unavailable.

Normative sources this packet projects from:

- `.t2/docs/Aureline_PRD.md` section 5.37 (desktop-platform rules) and 5.42
  (network, proxy, certificates, and transport governance).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` network/transport
  governance and identity/session rules.
- `.t2/docs/Aureline_Technical_Design_Document.md` Appendix AA (native OS
  conformance matrix) and the transport-governance appendices.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` transport/certificate templates and
  system-browser-first auth guidance.

If this document disagrees with those sources, the upstream sources win and the
matrix + fixtures update together.

## 1. Non-goals (explicit)

This packet does **not** implement:

- platform adapters for browser launch/return, proxy resolution, trust stores, or
  credential stores;
- a TLS stack, PAC engine, or keychain/secret-service client; or
- any policy engine behavior beyond what upstream contracts already freeze.

This packet is the *conformance disclosure*: what each claimed platform row
means, what is authoritative, and how degraded states surface without breaking
local-first posture.

## 2. Shared invariants (frozen)

1. **System browser is first-class.** Any supported auth row uses the system
   default browser by default. Embedded credential collection is forbidden
   unless an explicitly approved exception contract exists.
2. **One transport governance layer.** Desktop and CLI/headless flows must
   resolve proxy, trust store, mirror/offline posture, and denials through the
   shared transport governance packet.
3. **Local-first survives failures.** Proxy denial, trust failure, missing
   client certificates, or credential-store lock must not imply that local
   editing requires sign-in or network connectivity. These failures narrow only
   the specific networked capability.
4. **No plaintext fallback for long-lived secrets.** When a secure store is
   unavailable, the only admissible fallback is a *visible* session-only,
   in-memory posture where the owning contract allows it; durable plaintext
   storage is forbidden.

## 3. What “authoritative” means here

This packet answers, for each claimed desktop profile:

- **Browser registration:** how callback handlers are owned (and where side-by-side
  installs must avoid last-writer-wins behavior).
- **Browser return transport:** which `return_mode_class` is claim-bearing on that
  profile (deep link vs loopback vs manual resume).
- **Proxy source of truth:** what “system proxy” and “system PAC” mean on that
  profile, and when explicit overrides are allowed.
- **Trust-store integration:** what counts as OS trust, how org CA overlays are
  layered, and how client-certificate selection is referenced (handle-only).
- **Credential-store behavior:** how lock/deny/unavailable states are represented
  and which recovery paths are mandatory.

## 4. Claimed profile summary (human-readable)

The table below is a *reader* view. The authoritative rows live in the YAML
matrix.

| Claimed desktop profile | Browser return (claimed) | Proxy authority (system route) | Trust-store authority | Credential-store authority |
|---|---|---|---|---|
| `macos_15_plus_universal` | `platform_deep_link_return` (scheme pinned) | System Settings proxy + PAC | Keychain trust roots + org CA via profiles/keychains | Keychain Services |
| `windows_11_23h2_plus_x86_64` | `loopback_http_return` (port pinned) with explicit deep-link fallback disclosure | WinINET/WinHTTP effective proxy + PAC | Windows cert stores (CU/LM) + enterprise CA import | Credential Manager / DPAPI-backed storage |
| `linux_ubuntu_24_04_gnome_wayland_x86_64` | `platform_deep_link_return` (scheme pinned) | GNOME proxy settings + env proxy; PAC only when the session exposes it | distro trust bundle + enterprise CA tooling | Secret Service via `libsecret` / GNOME Keyring |
| `linux_ubuntu_24_04_gnome_x11_x86_64` | `platform_deep_link_return` (scheme pinned) | GNOME proxy settings + env proxy; PAC only when the session exposes it | distro trust bundle + enterprise CA tooling | Secret Service via `libsecret` / GNOME Keyring |
| `linux_fedora_current_gnome_wayland_x86_64` | `platform_deep_link_return` (scheme pinned) | GNOME proxy settings + env proxy; PAC only when the session exposes it | distro trust bundle + enterprise CA tooling | Secret Service via `libsecret` / GNOME Keyring |
| `linux_debian_stable_gnome_x11_x86_64` | `platform_deep_link_return` (scheme pinned) | GNOME proxy settings + env proxy; PAC only when the session exposes it | distro trust bundle + enterprise CA tooling | Secret Service via `libsecret` / GNOME Keyring |

## 5. Explicit override rules (proxy and trust)

Overrides are allowed only when they are *typed, inspectable, and attributable*.

### Proxy overrides

Proxy precedence and vocabulary are owned by
[`/docs/network/transport_governance_packet_seed.md`](../network/transport_governance_packet_seed.md).
This packet adds the per-OS “what counts as system proxy / PAC” disclosure.

Rules:

- A user or admin manual proxy is an explicit configuration that must remain
  inspectable and must not be treated as “system proxy”.
- Environment proxy variables are treated as `environment_proxy` and must not be
  silently ignored by one surface while honored by another.
- Direct/no-proxy is a selected posture, not a fallback, and must be permitted
  by the active profile/policy; it is never an automatic retry after proxy
  failure.

### Trust overrides

Trust-store layering and failure posture reuse the transport packet’s
`trust_store_source` vocabulary:

- `os_trust_store` (platform default),
- `os_trust_store_plus_org_ca_bundle` (enterprise CA overlay),
- `pinned_control_plane_trust_only` (pinned managed endpoints),
- `air_gap_offline_trust_root` (offline bundles/mirrors), and
- `trust_store_unknown` (repair-only placeholder, never a silent fallback).

Client certificates are referenced by handle only. Raw certificate material and
private keys do not appear in product UI, logs, fixtures, or support exports by
default.

## 6. Degraded-state projection (settings, diagnostics, support, handoff)

This packet does not mint new UI. It freezes what must be representable via
existing projection contracts.

### Browser handoff cards and callback return

When browser return succeeds or fails, surfaces project from:

- `auth_callback_packet_record` / `auth_handoff_interstitial_record` for handoff
  state and recovery actions; and
- transport decision/audit records for any proxy/trust failures encountered
  during networked follow-up (token exchange, discovery, policy fetch).

Degraded browser return MUST:

- fail closed on stale/superseded callbacks and wrong-tenant/workspace returns;
- render a typed recovery path that always includes “continue local without
  sign-in”; and
- preserve `preserved_local_work` so the product never implies that local
  editing depends on sign-in or connectivity.

### Settings and diagnostics

Degraded proxy/trust states MUST be explainable through:

- transport summary strip, endpoint rows, certificate/detail cards, denied-attempt
  history, and inspector packets (transport explainability contract); and
- credential-state and credential-store lock-state records (auth credential-state
  contract and callback packet contract).

### Policy-blocked network posture

When policy blocks network egress (deny-all profiles, policy-denied endpoints, or
admin-enforced mirror-only posture), the product must represent that posture via
the shared transport governance packet (`deny_policy` and `deny_all`-class
postures and their typed deny reasons), not as a generic “offline” state.

Policy-blocked network posture MUST:

- remain inspectable in settings/diagnostics/support exports with a typed denial
  reason and safe repair actions (open policy details, request admin change, or
  switch to an approved mirror where policy permits); and
- never imply that local editing, save, search, or local Git depends on network
  reachability or sign-in.

### Support bundle / admin export

Support bundles must carry *metadata-safe* identifiers and typed states:

- proxy mode, trust-store source, and typed denial/failure codes;
- credential store class and unlock/deny state; and
- the handoff/callback packet ids and preserved-local-work block.

Support exports must not include raw URLs, raw proxy credentials, raw tokens,
PAC bodies, certificate PEM, private keys, or other raw secret material.

## 7. Conformance case corpus

Worked reviewer drill cases live under
[`/fixtures/platform/native_auth_trust_cases/`](../../fixtures/platform/native_auth_trust_cases/):

- normal browser return to the bound context;
- stale callback return (superseded/expired);
- wrong-tenant/workspace callback return denied;
- proxy denial with no direct-origin bypass;
- client-certificate required (prompt/provision) and typed trust failure;
- credential store locked on startup with unlock/continue-local path; and
- trust store unavailable on a claimed profile (visible session-only downgrade
  where allowed, otherwise typed denial).
