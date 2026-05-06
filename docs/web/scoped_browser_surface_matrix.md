# Scoped browser and web surface capability matrix

This document freezes what browser-delivered and webview-like surfaces
may do, what they may not do, and how they must disclose those
boundaries to users and admins. It exists so docs, auth, preview/share,
incident review, admin, and companion experiences can cite **one**
shared capability table instead of inventing per-surface exceptions.

This is the human-readable companion to:

- `artifacts/web/scoped_browser_capabilities.yaml` — machine-readable
  surface rows, capability envelopes, local-core fallback postures, and
  disclosure requirements.

It composes with (non-exhaustive):

- `docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md` —
  embedded-surface boundary record, owner/origin chrome, and
  system-browser-first auth posture.
- `docs/ux/embedded_surface_boundary_cards.md` — render-side boundary
  card rules (required fields, capability limitations, fallback rows).
- `docs/auth/system_browser_callback_packet.md` — callback correlation,
  preserved-local-work truth, and “no silent embedded fallback” rule.
- `docs/auth/auth_handoff_interstitial_contract.md` — handoff review
  sheet and embedded-auth exception register.
- `docs/runtime/browser_runtime_contract.md` — preview-route identity,
  cross-origin/storage mutation gating, and imported-handoff sessions.
- `docs/remote/attach_tunnel_port_forward_contract.md` — route truth,
  share-link rules, and browser-handoff disclosure for forwarded
  endpoints.
- `docs/companion/companion_surface_contract.md` — scoped companion
  posture and “companion captures; desktop mutates” rule.

Normative source: `.t2/docs/Aureline_Technical_Architecture_Document.md`
§9.9 and Appendix AP (Scoped Browser and Web Surface Matrix).

## Cross-surface invariants (always)

1. **No hidden write authority.** A browser/web surface cannot gain
   write capability (including provider mutation) unless the matrix row
   for that surface explicitly permits it and the surface renders the
   required disclosure for the write path.
2. **Secrets are not rendered into web content.** Secret bytes, raw
   tokens, raw cookies, and raw callback URLs never appear in embedded
   web surfaces, companion surfaces, logs, or shareable links. Web
   surfaces may reference secret handles or short-lived session-scoped
   envelopes only.
3. **System browser is the default for authentication.** Silent fallback
   to an embedded webview for sign-in, consent, or step-up is
   non-conforming. Embedded credential entry is admissible only through
   an explicit exception register row with visibly lower-trust cues.
4. **No ambient local filesystem access from web surfaces.** Embedded
   web content cannot enumerate local roots, read arbitrary files, or
   write local state. Any file interaction must be host-mediated, user-
   initiated, and scope-minimized to the selected object(s).
5. **Device permissions are host-owned.** Camera/mic/screen capture,
   notifications, and OS-level permission prompts are not delegated to
   embedded web content. Web surfaces disclose when a device permission
   is required and route through host-native permission flows.
6. **Offline behavior is explicit and truthful.** When network, control
   plane, or provider reachability is absent, surfaces degrade to
   snapshot/inspect-only/local-only paths with explicit state labels.
   “Looks live but might be stale” is forbidden.
7. **Owner/origin chrome is mandatory.** Embedded webview-like surfaces
   always render host-owned owner/origin chrome and a boundary card.
   System-browser flows always render a host-owned handoff explanation
   (destination, reason, privacy consequence, return path).
8. **Local-core fallback exists for critical workflows.** Docs, auth
   continuity, incident triage, and admin/policy inspection must have a
   non-web local-core fallback posture that preserves local usefulness
   even when browser surfaces are unavailable or disallowed.

## Surface matrix (summary)

This table is a compact summary. The machine-readable matrix in
`artifacts/web/scoped_browser_capabilities.yaml` is the source of truth.

| Surface | Allowed web delivery modes | Allowed write scope | Local-core fallback | Required disclosure anchors |
|---|---|---|---|---|
| Docs viewing | embedded webview (scoped), system browser handoff | none | yes | boundary card (`owner/origin`, source/freshness), offline snapshot labeling |
| Auth handoff | system browser handoff, device-code | none (auth only) | yes | interstitial review sheet, preserved-local-work block, callback origin disclosure |
| Preview / share | embedded webview (scoped), system browser handoff | none by default; explicit share-link publish only where claimed | yes | route truth (termination/audience/TTL), handoff reason, copy/share class |
| Incident / CI review | embedded webview (scoped) for rendered artifacts, system browser handoff for provider pages | comments/approvals only where policy allows | yes | artifact provenance, provider identity, freshness, redaction/export posture |
| Admin flows | system browser handoff, embedded webview (policy-gated) | policy edits only via host-native review | yes | policy boundary + signer/epoch, control-plane dependence, open-in-browser posture |
| Companion notifications | scoped companion surface, system browser handoff | none; capture-only intents | yes | capability-limited companion badge, handoff packet requirement, privacy/redaction |
| Embedded extension web surfaces | embedded webview (scoped) only | none by default | yes | owner/publisher identity, capability limitations, reserved-native-surface ban |

## Capability axes (what the matrix controls)

Every surface row resolves across the same axes:

- **File access** — whether the surface can read/write local or remote
  files, or only reference host-selected objects.
- **Secret handling** — whether the surface can reference secret handles
  (never bytes), and which auth/token envelopes are admissible.
- **Provider identity** — how provider/service identity, origin, and
  acting subject must be disclosed.
- **Copy/export** — whether copy/export is allowed, how redaction
  applies, and whether shareable links are policy-gated.
- **Device permissions** — whether device capture/notification prompts
  are admissible (host-native only) and how web surfaces disclose
  missing capability.
- **Offline behavior** — how the surface degrades when network/provider
  reachability is absent and what snapshot posture is allowed.
- **Managed dependency** — whether the surface depends on a control
  plane/service and how that dependence is disclosed.

## Boundary rules (shared)

### File access

- Embedded web surfaces have **no ambient filesystem authority**. They
  do not enumerate roots, read arbitrary paths, or write to disk.
- Any file interaction is **host-mediated and user-initiated**, scoped
  to an explicit object selection (file, diff hunk, artifact node).
- “Upload a file” from a system browser is treated as **copy-out** and
  must disclose the destination and retention posture before bytes
  leave the device.

### Secret handling

- Web surfaces must not render secret bytes or store them in web storage.
  When a surface needs secrets, it references secret handles and relies
  on host-owned brokering paths.
- Auth flows prefer **system browser** or **device code**; embedded
  credential entry requires an explicit exception register row and must
  display lower-trust cues.

### Provider identity and origin

- Embedded web surfaces always show host-owned `owner_label`,
  `origin_label`, and `publisher_or_service_label` via the boundary card.
- System-browser flows must explain destination, reason, privacy
  consequence, and return path in host-native chrome before launching.

### Copy/export limits

- Copy/export actions are typed and redaction-aware. Raw provider URLs,
  raw cookies, raw tokens, and raw callback URLs are not copyable from
  governed surfaces.
- Shareable links are policy-gated and always disclose audience, TTL,
  and revoke path.

### Device permissions

- Embedded web content cannot directly request device permissions.
  Camera/mic/screen/notification permissions are requested through
  host-native flows with explicit disclosure on the web surface.

### Offline behavior

- When offline or provider-unavailable, surfaces degrade to snapshot or
  inspect-only with explicit freshness labels.
- Surfaces do not keep live-mutation affordances visible when the
  boundary has narrowed (policy blocked, certificate failed, offline).

## Worked examples

See `fixtures/web/browser_surface_cases/` for illustrative cases that
exercise:

- embedded docs viewing with offline snapshot behavior,
- system-browser-first auth handoff with device-code fallback,
- preview/share handoff and share-link disclosure,
- incident/CI evidence viewing with provider handoff,
- admin portal open-in-browser with local-core policy inspection
  fallback, and
- extension-hosted embedded surfaces with explicit capability limits.

