# Embedded-surface boundary cards: owner/origin chrome, permission state, action partition, browser-fallback posture, and degraded-state contract

This document freezes the **render-side** contract for the boundary card
the host shell paints next to every embedded surface in Aureline. The
boundary card is the compact, host-rendered chrome that declares — in
plain language — who owns the embedded surface, where it came from, what
publisher or service identity it represents, what permissions it
currently has, where its data boundary lies, which actions are
product-owned versus embedded, and what browser/device-code fallback
posture is currently available. It applies to embedded docs/help panes,
marketplace and account pages, service dashboards, auth confirmation
sheets, and extension-hosted web-like surfaces.

The contract is normative. Where this document disagrees with the
upstream record contract in
[`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md),
the ADR wins and this document plus its schema and fixtures must change
in the same patch. Where this document disagrees with a downstream
surface's local boundary bar, identity badge, or auth-fallback wording,
this document wins and the surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/embedded_boundary_card.schema.json`](../../schemas/ux/embedded_boundary_card.schema.json)
  defines the machine-readable `embedded_boundary_card_record` carried at
  the cross-tool boundary. The card is a render projection of the
  upstream `embedded_surface_boundary_record`; both contracts share the
  same closed vocabularies.
- [`/fixtures/ux/embedded_boundary_cases/`](../../fixtures/ux/embedded_boundary_cases/)
  contains worked examples covering every surface family and every
  closed boundary state, including stale, offline, certificate-failed,
  cross-origin-limited, policy-blocked, external-open-only, and
  exception-active rows.

## Upstream and sibling contracts

This contract composes with existing owners and does not replace them:

- [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  freezes the host-owned `embedded_surface_boundary_record`,
  `embedded_auth_exception_record`, and
  `embedded_surface_boundary_audit_event_record` family plus the
  surface-family, owner-class, publisher/service-class, origin-class,
  data-boundary, boundary-state, capability-limitation, native-reserved
  surface, auth-flow-class, exception-reason, and audit-event
  vocabularies the card re-renders.
- [`/schemas/ux/embedded_surface_boundary.schema.json`](../../schemas/ux/embedded_surface_boundary.schema.json)
  is the upstream record schema. The boundary card is a render
  projection of that record; producers must not mint a card without a
  backing surface boundary record.
- ADR-0010 owns `browser_handoff_packet`, `provider_actor_class`, and
  `grant_resolution_reason`; the card cites those refs rather than
  re-minting them.
- ADR-0013 owns `source_class`, `version_match_state`,
  `freshness_class`, and the docs/help browser-handoff reason set; the
  card quotes those for `embedded_docs_help` rows.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  owns the rule that high-risk preview, apply, and authority renewal
  remain product-owned and visible. The card's reserved-native-surface
  set re-exports that rule onto every embedded surface family.
- [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  owns the `RawText` / `SanitizedRich` / `TrustedLocalActive` /
  `IsolatedRemoteActive` trust classes. The boundary card never
  re-derives trust posture; it cites the upstream record's
  `trust_class`.

The card carries refs to those owners rather than re-minting their
fields.

## Why this exists

ADR-0015 freezes the upstream truth: every embedded surface resolves to
exactly one host-owned boundary record. What it does not freeze is the
**render** contract for that record — the visible, plain-language card
that makes the boundary inspectable on the screen, on a screenshot, in a
support export, and in a release-evidence pack. Without a frozen card
contract:

- docs/help, marketplace/account, service-dashboard, auth-confirmation,
  and extension-hosted rows would each invent their own owner bar,
  staleness badge, browser-handoff explanation, and fallback wording;
- support and export flows would have to scrape per-surface UI copy to
  reconstruct who owned a page, where it came from, and why a fallback
  was offered;
- product-owned actions and embedded actions would blur into one
  toolbar, and native-reserved surfaces (security messaging, update
  verification, trust elevation, rollback/restore confirmation, AI apply
  review, high-risk approval sheets) would be one CSS variant away from
  embedded impersonation;
- browser-first auth posture would be expressed inconsistently, so
  device-code fallback, policy-blocked external-open, and offline
  external-open could not be distinguished without reading per-surface
  prose.

This contract closes that gap by publishing one render-side card with a
fixed anatomy, a closed boundary-state set, an explicit
permission-state row, an explicit action partition, an explicit
browser-fallback posture, and explicit chrome / layout constraints.

## Scope

Frozen at this revision:

- one card record kind (`embedded_boundary_card_record`) backed by the
  upstream embedded-surface boundary record;
- closed vocabularies for surface family, owner identity, publisher /
  service identity, origin identity, data boundary, boundary state,
  permission class, action partition role, browser-fallback posture,
  fallback target, capability limitation, native-reserved surface, auth
  flow class, layout constraint, and chrome inheritance axis;
- the seven-state boundary-state grammar (`live_verified`,
  `stale_snapshot`, `policy_blocked`, `certificate_failed`,
  `cross_origin_limited`, `offline_snapshot`, `external_open_only`) and
  the required degraded-state behavior of the card on each;
- chrome and layout constraints so embedded surfaces inherit theme,
  density, zoom, focus, reduced-motion, high-contrast, and forced-colors
  posture from the host shell while remaining visibly bounded;
- per-surface-family card requirements that distinguish product-owned
  docs/help rows from provider-owned marketplace/account/service rows
  and from extension-hosted surfaces.

Out of scope:

- the webview runtime, browser launcher, device-code broker,
  certificate store, or policy engine implementations themselves;
- the upstream embedded-surface boundary record, exception register,
  and audit event family (frozen in ADR-0015);
- per-provider docs, marketplace, dashboard, or auth UI specifics
  beyond the closed identity, scope, health, and fallback vocabularies
  the card renders.

## Card render invariant

A boundary card is the only legitimate way to render owner, origin,
publisher / service identity, permission state, data boundary, action
partition, and browser-fallback posture next to an embedded surface in
Aureline.

A surface that:

- paints its own owner bar, identity chip, staleness badge, browser /
  device-code copy, or fallback explanation without quoting an
  `embedded_boundary_card_record` by id;
- collapses owner, origin, publisher / service identity, or data
  boundary into a single chip;
- renders boundary chrome that visually impersonates host-native
  security messaging, update verification, trust elevation, rollback or
  restore confirmation, AI apply review, or a high-risk approval sheet;
- hides any required field behind a hover tooltip, an inspector
  affordance, or a devtools-only disclosure; or
- continues to render mutating affordances after the upstream record
  has narrowed the boundary state below `live_verified`,

is non-conforming. The card is **render of the truth**, not a separate
truth.

## Required card anatomy

Every `embedded_boundary_card_record` carries the same top-level shape.
Renderers may project subsets of optional rows, but they may not drop
the required slots or replace them with prose-only state.

| Slot | Required content | Non-conforming collapse |
| --- | --- | --- |
| `card_id` | Opaque, stable id of this card render. | Surface-local DOM id with no durable identity. |
| `surface_id_ref` | Opaque ref to the upstream `embedded_surface_boundary_record`. | Card minted without a backing record. |
| `surface_family` | One of `embedded_docs_help`, `embedded_marketplace_or_account`, `embedded_service_dashboard`, `embedded_auth_confirmation`, `extension_hosted_surface`. | Surface family inferred from URL or chrome heuristics. |
| `owner_identity` | Owner label plus closed `owner_class`. | Generic “Aureline” bar with no closed class. |
| `publisher_or_service_identity` | Publisher / service label plus closed `publisher_or_service_class`. | Same chip used for owner and publisher. |
| `origin_identity` | Origin class, plain origin label, host or domain label, and verification state. | Bare URL string with no class or verification state. |
| `data_boundary_class` and `data_boundary_label` | Where the bytes, session state, and authority live in plain language. | “Cloud” or “Hosted” with no closed class. |
| `boundary_state` and `boundary_state_label` | One of the seven closed boundary states plus a plain-language label. | Generic “Limited” or “Offline” chip with no closed state. |
| `plain_language_summary` | One-sentence host-rendered summary of what the surface is and what it can do right now. | Marketing copy or vendor tagline. |
| `permission_state` | Permission class, plain label, host-native step-up flag, and (for embedded lower-trust states) an `exception_id_ref`. | Mutating affordances exposed without a permission row. |
| `action_partition` | Every visible action, partitioned into `product_owned_native`, `product_owned_handoff`, `embedded_inspect_only`, or `embedded_request_only`. | One toolbar mixing product-owned and embedded actions with no role. |
| `browser_fallback` | Posture class plus fallback target class plus summary; required when the live in-product lane has narrowed. | A bare `Open in browser` button with no posture or target class. |
| `capability_limitations` | Closed list of host-rendered capability limitations that apply right now. | Free-form `Some features may be unavailable` copy. |
| `reserved_native_surfaces_host_owned` | Closed list of native-reserved surfaces this card explicitly keeps host-owned. | Empty set on a card that shows trust, update, rollback, or AI-apply adjacent affordances. |
| `layout_constraints` | Closed layout-constraint ids the host shell satisfies for this card. | Card painted inside the embedded body, hover-only fields, or actions overlapped by embedded chrome. |
| `chrome_inheritance_axes` | Closed inheritance axes (theme, density, zoom, focus, reduced-motion, high-contrast, forced-colors). | Embedded body painting parallel theme or density. |
| `policy_context` | Identity mode, policy epoch, trust state, optional execution context. | Card minted without policy context. |
| `redaction_class` | Closed redaction class for support / export. | Local synonym such as `safe`, `internal`, `restricted`. |
| `minted_at` | Monotonic timestamp from the host. | Surface-local wall-clock string. |

The schema enforces the required slots through `required` plus
per-`surface_family` `allOf` branches.

## Per-surface-family card rules

The card vocabulary is shared, but each surface family adds explicit
required rows so audit, support, and release evidence can read one
boundary contract across surfaces.

### Embedded docs/help (`embedded_docs_help`)

Required rows: `source_truth` (with `source_class`,
`version_match_state`, `freshness_class`,
`running_build_identity_ref`, optional `snapshot_age_label`).

Rules:

1. The card MUST quote `source_class`, `version_match_state`, and
   `freshness_class` from the upstream
   `help_status_badge_record`. It MUST NOT recalculate or paraphrase
   them locally.
2. Project docs outrank vendor / provider docs on in-scope topics. A
   card that renders vendor / provider docs in place of in-scope project
   docs without an explicit override disclosure is non-conforming.
3. `stale_snapshot` and `offline_snapshot` cards MUST display source,
   version, freshness, and `snapshot_age_label` together; “cached
   docs” alone is not enough.
4. The action partition MUST mark inspect / copy / open-in-browser as
   `embedded_inspect_only` or `product_owned_handoff` rather than
   `product_owned_native`. Docs/help embedded surfaces never carry a
   `product_owned_native` action that mutates trust, update, rollback,
   or AI-apply state.

### Embedded marketplace / account (`embedded_marketplace_or_account`)

Required rows: `provider_identity` (with `provider_class`,
`provider_label`, `provider_scope_label`, `provider_actor_class`,
`health_state`, optional `connected_provider_record_id`,
`health_summary_label`).

Rules:

1. A generic `Connected` chip is forbidden. The card MUST render
   provider class, scope, acting identity, and health/expiry as
   separate slots.
2. The action partition MUST keep account-link, scope-grant, billing,
   and admin-policy actions in `product_owned_native` or
   `product_owned_handoff` roles. The embedded body MAY render
   `embedded_inspect_only` and `embedded_request_only` rows but never
   `product_owned_native` rows.
3. `policy_blocked`, `stale_snapshot`, `offline_snapshot`, and
   `cross_origin_limited` states MUST degrade the card to inspect /
   copy / export or browser handoff. They may not strip the card and
   strand the user without context.
4. If the surface exceeds client scope or requires a live
   provider-owned mutation Aureline cannot honestly host, the card MUST
   preserve object identity and explain why the in-product lane ended
   before handing off externally.

### Embedded service dashboard (`embedded_service_dashboard`)

Required rows: `provider_identity` as above.

Rules:

1. Service-dashboard cards MUST disclose `provider_class`,
   `provider_scope_label`, target object label (carried via
   `plain_language_summary` or upstream record), health state, and a
   freshness or fetch-time label so dashboards never masquerade as
   local or native authority.
2. `certificate_failed` MUST withhold the embedded body. The card
   alone remains visible with the `inspect_certificate_details`
   action and a `system_browser_handoff_packet` or
   `local_inspect_or_export` fallback.
3. Dashboards MUST partition “open in provider console” as
   `product_owned_handoff` (the host owns the handoff packet and
   policy decision) rather than `embedded_request_only`.
4. Mirror, offline, and degraded postures MUST cite the upstream
   transport explainability record (ADR-0015 + transport-explainability
   contract) so support and release evidence can quote one transport
   truth.

### Embedded auth confirmation (`embedded_auth_confirmation`)

Required rows: `auth_handoff` (with `flow_class`,
`provider_domain_label`, `reason_label`, `return_target_label`,
optional `local_continuity_note`, `code_expiry_label`,
`exception_id_ref`).

Rules:

1. Supported auth rows are **system-browser first**. The default card
   carries `flow_class: system_browser` and a
   `system_browser_handoff_packet` fallback target.
2. When system-browser is unavailable or blocked, the card MUST switch
   to `flow_class: device_code` with a `device_code_companion_card`
   fallback target. Device-code copy, code-expiry label, and return /
   help fallback rows are required.
3. `flow_class: embedded_session_refresh` is allowed only when the
   permission state is `embedded_lower_trust_session_refresh` and no
   password is collected, no scope widens, and no native step-up is
   skipped. The card MUST mark `embedded_auth_lower_trust` as a
   capability limitation.
4. `flow_class: embedded_password_exception` requires
   `permission_class: embedded_lower_trust_password_exception`,
   `host_native_step_up_required: true`, an `exception_id_ref`, and
   a visible lower-trust badge through `capability_limitations`. The
   card MUST also render an exit path to system browser or device code
   where available.
5. The card MUST never render final approval, trust elevation,
   rollback / restore confirmation, AI apply review, update
   verification, or any other native-reserved surface, even when the
   embedded auth flow completes successfully.

### Extension-hosted surface (`extension_hosted_surface`)

No additional required rows beyond the shared anatomy, but:

1. `owner_identity.class` MUST be `extension_bundle` and the card
   MUST disclose the publisher / service identity separately from the
   host product.
2. The card MUST render the upstream `safe_preview` trust class
   summary in `plain_language_summary` so users can see whether the
   extension surface is `SanitizedRich`, `TrustedLocalActive`, or
   `IsolatedRemoteActive`.
3. Extension-hosted cards MUST keep their action partition strictly
   inside `embedded_inspect_only` and `embedded_request_only` unless
   the host shell has explicitly opted into a `product_owned_handoff`
   action for the extension family.
4. `cross_origin_limited` and `policy_blocked` states MUST name the
   exact reduced capability rather than collapsing to a generic
   “limited” badge.

## Boundary states and required degraded behavior

Every card carries exactly one boundary state. The set is closed.

| State | Card render rule | Required `browser_fallback` | Required `capability_limitations` |
| --- | --- | --- | --- |
| `live_verified` | All declared affordances render. Embedded body is shown within its trust class. | Optional; if absent, set posture to `browser_fallback_not_applicable`. | Empty set is allowed. |
| `stale_snapshot` | Live-mutation actions removed; `snapshot_age_label` and origin remain visible; embedded body may render as a snapshot. | Required; preferred posture is `system_browser_first` with a `system_browser_handoff_packet` target. | At minimum `live_network_mutation_disabled_when_offline` or `provider_scope_may_be_narrower_than_page_claims`. |
| `policy_blocked` | Embedded body withheld where the policy denies it; card remains visible with `inspect_policy_reason` and any allowed external-open path. | Required; posture is `external_open_blocked_by_policy` if external-open is also denied, otherwise `system_browser_first`. | At minimum the capability the policy denies. |
| `certificate_failed` | Embedded body withheld; card remains visible with `inspect_certificate_details` and a host-native or local-inspect fallback. | Required; posture is typically `external_open_blocked_by_policy` and target is `local_inspect_or_export` or `host_native_review_or_approval`. | At minimum `cross_origin_dom_or_storage_hidden` and any provider-scope narrowing. |
| `cross_origin_limited` | Only the honest subset of capabilities renders; the missing capability is named in `capability_limitations` and `plain_language_summary`. | Required; preferred posture is `system_browser_first` with `system_browser_handoff_packet` target. | At minimum `cross_origin_dom_or_storage_hidden`. |
| `offline_snapshot` | Snapshot-aged body may render; live auth and mutation claims removed; local-continuity note required. | Required; posture is typically `external_open_unavailable_offline` with `local_inspect_or_export` target. | At minimum `live_network_mutation_disabled_when_offline`. |
| `external_open_only` | No embedded body; only the card and an external / device-code / native fallback. | Required; `fallback_target_class` MUST be one of `system_browser_handoff_packet`, `device_code_companion_card`, `host_native_review_or_approval`, or `platform_authenticator_native`. | At minimum the capability that forced the narrowing. |

Rules (frozen):

1. Degradation MUST narrow capability, not blur it. A card never
   transitions from `live_verified` to a state that “looks roughly the
   same but maybe broken.”
2. `certificate_failed` and `policy_blocked` MUST NOT silently reuse a
   `stale_snapshot` or `offline_snapshot` affordance set as if trust
   were intact.
3. `stale_snapshot` and `offline_snapshot` MUST preserve object
   identity (origin label, publisher / service identity, target object
   label) and snapshot age together so later browser, device-code, or
   open-external actions remain attributable to the same object family.
4. Any state below `live_verified` MUST remove or reroute
   native-reserved affordances. The
   `reserved_native_surfaces_host_owned` set never shrinks across
   states.

## Permission state

`permission_state` is a separate slot. Boundary state names whether the
upstream truth still trusts the surface; permission state names what the
host shell is currently letting that surface do. The two are
correlated but neither implies the other.

| Permission class | Allowed action partition | Card requirements |
| --- | --- | --- |
| `host_owned_full_authority` | `product_owned_native`, `product_owned_handoff`, `embedded_inspect_only`, `embedded_request_only` allowed. | No exception ref required. |
| `host_owned_inspect_only` | Only `embedded_inspect_only` and `product_owned_handoff` allowed. | No mutating affordance may render. |
| `host_owned_browser_only` | Only `product_owned_handoff` allowed; embedded body may be withheld. | Browser-fallback posture MUST be `system_browser_first` or `device_code_fallback_offered`. |
| `host_owned_copy_export_only` | Only `embedded_inspect_only` rows whose action labels match copy / export verbs. | Capability limitations MUST name what is unavailable. |
| `host_owned_with_native_step_up_required` | `product_owned_native` actions are gated behind a host-native step-up. | `host_native_step_up_required` MUST be `true`. |
| `embedded_lower_trust_session_refresh` | Only narrow renewal inside an already-authenticated session. | `exception_id_ref` and `embedded_auth_lower_trust` capability limitation are required. |
| `embedded_lower_trust_password_exception` | Embedded password collection through an exception register row. | `exception_id_ref`, `host_native_step_up_required: true`, `embedded_auth_lower_trust` capability limitation, and a visible lower-trust badge are required. |
| `no_permission_within_product` | Card MAY render only inspect / external-open actions. | `boundary_state` MUST be `external_open_only`, `policy_blocked`, or `certificate_failed`. |

Rules (frozen):

1. A mutating affordance MUST NOT render unless the permission class
   admits it and the boundary state allows it. Surfaces that show a
   mutating control under any other combination are non-conforming.
2. Embedded lower-trust states never authorize native-reserved
   surfaces. They narrow further; they do not widen.
3. Expired or revoked exceptions immediately downgrade the card to
   `host_owned_browser_only`, `host_owned_inspect_only`, or
   `no_permission_within_product` as appropriate.

## Action partition

Every action visible near the embedded surface declares one of four
roles. The partition is closed.

| Role | Meaning | Render rule |
| --- | --- | --- |
| `product_owned_native` | Host-native action that carries final authority (security messaging, update verification, trust elevation, rollback / restore, AI apply review, high-risk approval). | Renders in host chrome, never inside the embedded body. |
| `product_owned_handoff` | Host-owned handoff to a system browser, device-code companion card, or platform-native authenticator. | Renders in host chrome with a quoted `browser_handoff_packet_ref` or `device_code_ref`. |
| `embedded_inspect_only` | Read-only / copy / export action whose effect stays inside the embedded body or the host clipboard. | MAY render inside the embedded body; MUST NOT claim mutation authority. |
| `embedded_request_only` | Action that the embedded surface uses to request a native review or approval. | MUST round-trip through the host shell, which re-evaluates policy, trust, route, and object identity at the boundary. |

Rules (frozen):

1. Native-reserved surfaces MUST be rendered as `product_owned_native`
   only. An action whose role is `embedded_inspect_only` or
   `embedded_request_only` may not be styled to look like a
   `product_owned_native` action.
2. Browser, device-code, and platform-authenticator handoffs MUST be
   `product_owned_handoff` and MUST quote an upstream
   `browser_handoff_packet` or `device_code_ref` so support and audit
   can attribute the handoff to the same object family.
3. The action partition is the single source of truth for chrome
   placement. A surface that lets the embedded body paint over or
   intercept a `product_owned_*` action is non-conforming.

## Browser-fallback posture

The card carries one `browser_fallback` row with a closed posture class
and a closed fallback target class.

| Posture | Meaning | Required fallback target |
| --- | --- | --- |
| `system_browser_first` | System browser is the primary fallback target; supported and policy-allowed. | `system_browser_handoff_packet`. |
| `device_code_fallback_offered` | Browser launch is blocked or non-returnable; device-code is the auditable next step. | `device_code_companion_card`. |
| `external_open_blocked_by_policy` | External-open is denied by policy; only host-native review, local inspect, or platform-authenticator paths remain. | `host_native_review_or_approval`, `local_inspect_or_export`, or `platform_authenticator_native`. |
| `external_open_unavailable_offline` | Network is unavailable; offline snapshot or local inspect is the safe fallback. | `local_inspect_or_export` or `no_fallback_available`. |
| `browser_fallback_not_applicable` | The surface is `live_verified` and no fallback is needed. | `no_fallback_available` or omitted. |

Rules (frozen):

1. A boundary state below `live_verified` MUST carry a
   `browser_fallback` row. The schema enforces this through an
   `if` / `then` branch.
2. `external_open_only` states MUST NOT use
   `browser_fallback_not_applicable`. Their fallback target class is
   one of the closed external-open / host-native / device-code /
   platform-authenticator targets.
3. `device_code_fallback_offered` MUST quote a `device_code_ref` and
   a `code_expiry_label` so the fallback is reviewable.
4. Browser-fallback posture MUST agree with the upstream
   `auth_handoff.flow_class` when the surface is
   `embedded_auth_confirmation`. Disagreement is non-conforming.

## Reserved native surfaces and impersonation ban

The following native-reserved surfaces are host-owned and may never be
hosted, painted, or convincingly imitated by embedded content:

- `product_security_messaging`
- `update_verification`
- `workspace_trust_elevation`
- `rollback_or_restore_confirmation`
- `ai_apply_review`
- `high_risk_approval_sheet`

Rules (frozen):

1. The card's `reserved_native_surfaces_host_owned` set is non-empty
   on every record, on every state, including `live_verified`. The
   host disclosure remains stable across states.
2. Embedded content MUST NEVER paint a header, banner, or sheet that
   visually impersonates any of these surfaces. The host MAY open a
   native review or approval surface from the embedded page through an
   `embedded_request_only` action, but it MUST re-evaluate policy,
   trust, route, and object identity at that boundary.
3. When a native-reserved action cannot be opened or revalidated, the
   card narrows to inspect, copy, export, or open-in-browser /
   device-code paths instead of rendering a weaker in-page substitute.

## Chrome and layout constraints

`layout_constraints` and `chrome_inheritance_axes` are closed
vocabularies. The schema enforces a minimum non-skippable subset
through `minItems`.

Mandatory layout constraints:

- `card_visually_distinct_from_embedded_body` — the card MUST be
  visually and semantically distinct from the embedded body. A page
  may not make its own header look like the canonical card.
- `card_not_obscured_by_embedded_body` — the embedded body MUST NOT
  scroll over, overlay, or replace the card.
- `card_required_fields_never_hover_only` — every required field
  renders in the primary view. Hover-only, inspector-only, or
  devtools-only disclosure is non-conforming.
- `card_compact_layout_preserves_required_fields` — compact layouts
  may reorder or collapse spacing, but they may not hide the required
  field set for the surface family.
- `card_actions_render_in_host_chrome` — `product_owned_native` and
  `product_owned_handoff` actions render in host chrome only.
- `embedded_body_cannot_overlap_card_actions` — embedded chrome MUST
  not paint over or intercept `product_owned_*` action targets.
- `card_remains_visible_when_embedded_body_is_withheld` — when the
  embedded body is withheld (`certificate_failed`, `policy_blocked`,
  `external_open_only`), the card and its actions stay visible.

Mandatory chrome inheritance axes:

- `theme_palette_inherits_from_host`
- `density_class_inherits_from_host`
- `zoom_level_inherits_from_host`
- `focus_ring_inherits_from_host`
- `reduced_motion_posture_inherits_from_host`
- `high_contrast_mode_inherits_from_host`
- `forced_colors_mode_inherits_from_host`

Rules (frozen):

1. The boundary card never paints a parallel theme, density, zoom,
   focus, motion, contrast, or forced-colors model. It inherits from
   the host shell so embedded content cannot invent its own visual
   model and feel native.
2. Density and zoom changes adjust spacing and sizing only. They MUST
   NOT change the field set, the action partition, or the
   reserved-native-surface disclosure.
3. Reduced-motion, high-contrast, and forced-colors postures MUST
   preserve the meaning of every required field. Color-only state
   distinctions are non-conforming on degraded states.
4. Focus order MUST start with the host-owned card before entering the
   embedded body. The embedded body MUST NOT trap focus; the host
   shell owns the escape sequence.

## Audit and export rules

The boundary card record is exportable. Rules:

1. The card MUST carry an opaque `surface_id_ref` to the upstream
   `embedded_surface_boundary_record` so support and audit can replay
   one boundary truth across desktop UI, support export, and release
   evidence.
2. Card audit emits typed boundary events through the upstream
   `embedded_surface_boundary_audit_event_record` family. The card
   itself MUST NOT mint new audit-event ids.
3. Raw HTML, raw cookies, raw passwords, raw device codes beyond what
   the visible cue already declares, raw external URLs, and raw
   provider tokens MUST NOT appear on the card or in any export of
   it. The `redaction_class` slot enforces the exporter's contract.
4. `policy_context.identity_mode`, `policy_context.policy_epoch`, and
   `policy_context.trust_state` MUST be present on every card so audit
   and release evidence can distinguish account-free, self-hosted-org,
   and managed-workspace renderings of the same surface family.

## Worked examples

The fixtures in
[`/fixtures/ux/embedded_boundary_cases/`](../../fixtures/ux/embedded_boundary_cases/)
exercise the full closed vocabulary. The YAML cases are render-side
projections; the existing JSON cases continue to model the upstream
record. Both share `surface_id_ref` so they remain attributable to the
same surface family.

YAML cases (this contract):

- `docs_help_live_verified_card.yaml` — embedded docs/help card with
  `live_verified` state, exact build match, and a host-rendered
  inspect / copy partition.
- `marketplace_account_offline_snapshot_card.yaml` — marketplace /
  account card with `offline_snapshot` state, narrowed scope, and an
  `external_open_unavailable_offline` browser-fallback posture.
- `service_dashboard_certificate_failed_card.yaml` — service-dashboard
  card with `certificate_failed` state, withheld body, and a
  host-native inspect-certificate fallback.
- `auth_confirmation_system_browser_first_card.yaml` — auth-confirmation
  card with `system_browser_first` posture and a `system_browser`
  flow class.
- `auth_confirmation_password_exception_card.yaml` — auth-confirmation
  card backed by an exception-register row, `embedded_password_exception`
  flow class, and visible lower-trust capability limitations.
- `extension_hosted_cross_origin_limited_card.yaml` — extension-hosted
  card with `cross_origin_limited` state and a `system_browser_first`
  fallback posture.
- `marketplace_account_external_open_only_card.yaml` — marketplace /
  account card with `external_open_only` state and a
  `device_code_companion_card` fallback target.

## Minimal review questions

Before a card variant is accepted, reviewers should be able to answer:

- which surface family the card belongs to and which required rows
  apply for that family;
- whether the card carries a backing `embedded_surface_boundary_record`
  and whether `surface_id_ref` quotes it correctly;
- whether the boundary state, permission class, action partition, and
  browser-fallback posture agree with each other;
- whether every native-reserved surface is rendered host-owned and
  visually distinct from the embedded body;
- which capability limitations are required for the current state and
  whether the card lists all of them;
- whether the card inherits theme, density, zoom, focus, reduced-motion,
  high-contrast, and forced-colors posture from the host shell;
- whether support / export can reconstruct owner, origin, publisher /
  service identity, and data boundary from the card alone.
