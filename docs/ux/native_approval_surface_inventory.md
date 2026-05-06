# Native approval surface inventory, product-boundary chrome rules, and browser-fallback matrix

This document publishes the inventory of **host-native / product-owned**
approval surfaces and the **product-boundary chrome** rules that keep
trust, consent, and high-risk decisions legible when surrounding content
is embedded (webview-like) or externally hosted (system browser / vendor
dashboard).

The goal is simple: **high-risk approvals and security-sensitive
messaging remain product-owned** even when the user is currently reading
docs, browsing a marketplace/account view, looking at a service
dashboard, or completing an authentication handoff.

## Companion artifacts

- [`/artifacts/ux/native_approval_inventory.yaml`](../../artifacts/ux/native_approval_inventory.yaml)
  — machine-readable inventory of native approval surfaces, their
  upstream contracts/schemas, and the required separation rules.
- [`/fixtures/ux/open_in_browser_fallback_cases/`](../../fixtures/ux/open_in_browser_fallback_cases/)
  — worked examples that exercise the browser-fallback matrix and the
  “return to host-native review” boundary.

## Upstream contracts (authoritative)

This inventory does not re-mint vocabularies; it cites the existing
contracts that already freeze the surface boundaries:

- [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  — host-owned embedded-surface boundary model and the impersonation ban
  for native-reserved surfaces.
- [`/docs/ux/embedded_surface_boundary_cards.md`](./embedded_surface_boundary_cards.md)
  — render-side boundary-card contract (required owner/origin/data
  boundary fields + browser-fallback posture).
- [`/docs/web/scoped_browser_surface_matrix.md`](../web/scoped_browser_surface_matrix.md)
  — webview/system-browser capability matrix and “no raw URL launch”
  handoff rules.
- [`/docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  — prompt-time trust/policy/permission request contract.
- [`/docs/trust/capability_sheet_contract.md`](../trust/capability_sheet_contract.md)
  — durable permission/capability review surface (extension, AI, remote,
  package/script, policy).
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace trust states, restricted-mode continuity, and remembered
  trust decision rules.
- [`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md)
  — extension effective-permission and publisher/policy constraints.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — consequence-bearing interaction packet, preview/apply/revert, and
  protected-surface rules.
- [`/docs/auth/auth_handoff_interstitial_contract.md`](../auth/auth_handoff_interstitial_contract.md)
  and [`/artifacts/auth/native_approval_boundary_review.yaml`](../../artifacts/auth/native_approval_boundary_review.yaml)
  — auth handoff review sheet and the “no silent embedded fallback”
  approval boundary.
- [`/docs/auth/device_permission_and_capture_boundary_contract.md`](../auth/device_permission_and_capture_boundary_contract.md)
  — device-permission and capture consent surfaces, including
  embedded-origin honesty and escape hatches.

If this document disagrees with an upstream contract, the upstream
contract wins and this document + the companion inventory must update in
the same change.

## Definitions

### Product-owned / host-native surface

A **product-owned / host-native** surface is rendered by the Aureline
host shell (not by embedded HTML/JS) and is attributable through the
relevant boundary record(s). A native surface may still route through
platform steps (OS permission sheets) or provider steps (system browser),
but the **decision framing** and **final authority transition** remains
product-owned.

### Embedded / externally hosted surface

An **embedded** surface is rendered inside the desktop shell but is not
product-owned (docs/help webviews, marketplace/account pages, service
dashboards, extension-hosted web surfaces). An **externally hosted**
surface is outside the product (system browser or vendor console).

Embedded/external surfaces may:

- render *content* and *inspection* affordances within their declared
  boundary, and
- *request* that the host open a native approval surface.

They must not:

- impersonate native trust/update/security chrome, or
- complete a launch-bearing approval decision in the embedded body.

## Native approval surfaces (inventory)

This section is a human-readable summary of the inventory in
`/artifacts/ux/native_approval_inventory.yaml`.

### Native-reserved (must never be embedded)

These are the **native-reserved surfaces** frozen by ADR-0015 and
re-exported by the embedded-surface boundary record and boundary card.
Embedded surfaces may reference them as *reserved* and may request a
host-native route, but they may not render lookalikes.

| Native-reserved surface | What it covers (examples) | Primary sources |
|---|---|---|
| `product_security_messaging` | suspicious-content disclosure, taint/egress warning, boundary honesty, security state that must be attributable | ADR-0015; shell interaction-safety contract |
| `update_verification` | provenance/signature truth, update integrity, “what build is this?” verification | ADR-0015 |
| `workspace_trust_elevation` | restricted-mode exit, workspace trust grant/deny/renew, trust drift review | ADR-0015; workspace trust ADR; trust prompt contract |
| `rollback_or_restore_confirmation` | restore/rollback confirmation on durable state changes (migration rollback, recovery rollback, export-before-reset) | ADR-0015; recovery contracts; prompt grammar contract |
| `ai_apply_review` | AI-proposed mutation review, preview/apply/validate/keep-or-revert, citation/rollback obligations | ADR-0015; shell interaction-safety contract; AI contracts |
| `high_risk_approval_sheet` | “final approval” sheet for high-risk authority changes (permission grants, policy overrides, publish/promote/rollback, provider-side mutation) | ADR-0015; trust prompt + capability sheet contracts |

### Other host-native approval/consent surfaces (must remain product-owned)

These surfaces are not expressed as `native_reserved_surface` tokens
because they already have their own primary schemas/contracts, but they
still share the same boundary goal: **no embedded/external surface
finalizes the approval**.

| Surface family | What it covers | Primary sources |
|---|---|---|
| Extension permission grant review | extension install/update/activation permission review, transitive capability growth disclosure, revocation routes | capability sheet contract; trust prompt contract; extension permission ADR |
| Destructive confirmation | delete/purge/detach/overwrite confirmations, safe default focus, preview/export-before-change obligations | shell interaction-safety contract; prompt grammar contract; dialog/sheet contract |
| Identity and consent steps | auth handoff review, step-up authority, consent renewal prompts, device permission rows (mic/camera/screen/notifications) | auth handoff contracts + review packet; device permission/capture contract; trust prompt contract |

## Product-boundary chrome rules (docs/help, marketplace/account, auth, dashboards)

The **boundary card** is the stable contract that keeps “who owns this
surface” honest. Every claimed embedded/auth surface MUST expose:

- owner (who owns the surface),
- origin (where bytes are fetched from, plus verification state),
- data boundary (where session/state lives),
- action partition (product-owned vs embedded actions), and
- browser-fallback posture (open-in-browser availability + target class).

The authoritative field list and per-surface-family requirements live
in [`/docs/ux/embedded_surface_boundary_cards.md`](./embedded_surface_boundary_cards.md)
and are reviewed by
[`/artifacts/ux/owner_origin_chrome_review.yaml`](../../artifacts/ux/owner_origin_chrome_review.yaml).

Additional separation rules that apply across all embedded families:

1. **No impersonation.** Embedded surfaces MUST NEVER impersonate native
   security messaging, update verification, trust elevation, rollback /
   restore confirmation, AI apply review, or high-risk approval sheets.
2. **Host reevaluates at the boundary.** When an embedded surface
   requests a native approval, the host MUST re-evaluate policy, trust,
   route, and object identity; it must not inherit ambient authority
   from the embed.
3. **Degradation narrows capability.** When boundary state is below
   `live_verified`, surfaces narrow to inspect/copy/export/handoff; they
   do not keep mutating approval affordances visible as if trust were
   intact.

### Per-surface-family disclosure anchors (summary)

The boundary-card contract is the source of truth. This list is a
human-readable summary of the **minimum** disclosure anchors that must
remain visible and attributable per embedded family:

- **Docs/help panes** (`embedded_docs_help`)
  - MUST show docs source/version/freshness truth (quoting the upstream
    docs/help contracts) and MUST keep open-in-browser as a typed handoff
    rather than a raw URL.
  - MUST NOT render product-owned trust/rollback/update/AI-apply actions
    in embedded chrome; those route to native surfaces.
- **Marketplace/account** (`embedded_marketplace_or_account`)
  - MUST show provider identity, scope, and acting subject explicitly
    (a generic “Connected” chip is non-conforming).
  - MUST keep scope grants, billing/admin actions, and any high-risk
    approvals product-owned (native review or typed handoff).
- **Service dashboards** (`embedded_service_dashboard`)
  - MUST show service identity and boundary state (policy/cert/offline)
    without allowing the embedded body to paint its own “secure” chrome.
  - MUST narrow to inspection/evidence/handoff in degraded states.
- **Auth handoff rows** (`embedded_auth_confirmation`)
  - MUST remain system-browser-first with explicit fallback (device code
    or platform-native where claimed).
  - Embedded credential entry is admissible only through an explicit,
    auditable exception register row with visibly lower-trust cues.

## Browser fallback (“Open in browser”) matrix

Open-in-browser is a **product-owned handoff**, not an embedded
affordance. The product exposes it only when the surface’s declared
contract allows it and the current boundary state/posture admits it.

Rules (frozen by the upstream contracts cited above):

1. **No raw URL launch.** Open-in-browser uses a typed handoff packet
   (ADR-0010 family) and a declared reason code; embedded chrome may not
   launch raw URLs.
2. **Policy may block external open.** When policy blocks external open,
   the fallback posture is `external_open_blocked_by_policy` and the UI
   must offer policy inspection/support evidence rather than a dead
   “Open in browser” button.
3. **Offline may block external open.** When offline, the fallback
   posture is `external_open_unavailable_offline` and the surface must
   preserve local-only continuity (inspect/export/continue-local).
4. **External inspection does not equal approval.** If an external page
   (vendor console, hosted dashboard, system browser) is opened for
   inspection, any final trust/permission/destructive/rollback/AI-apply
   decision still routes back to a product-owned native approval
   surface.

Worked examples exercising these postures live in
`/fixtures/ux/open_in_browser_fallback_cases/`.
