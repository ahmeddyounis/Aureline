# ADR 0015 — Embedded-surface boundary, owner/origin chrome, and system-browser-first auth handoff

- **Decision id:** D-0021 (must exist in `artifacts/governance/decision_index.yaml`)
- **Status:** Accepted
- **Decision date:** 2026-04-21
- **Freeze deadline:** 2026-10-15
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` with waiver `single-maintainer-backup`
- **Forum:** security_trust_review
- **Related requirement ids:** `none`

## Context

Safe preview, provider handoff, docs/help truth, and shell
interaction-safety already freeze adjacent parts of this problem, but
they stop short of defining one shared boundary contract for embedded
surfaces inside the desktop shell. The product now has enough parallel
requirements that this gap is dangerous: docs/help panes must disclose
source, version, and freshness; marketplace/account and service
dashboard views must disclose provider identity, scope, and reduced
authority; auth confirmation surfaces must prefer system-browser or
device-code paths; and every embedded page must be prevented from
looking like native Aureline security, update, trust, rollback, or
AI-apply chrome.

The failure mode is not only visual inconsistency. Without one host-owned
boundary model, each embedded surface can mint its own owner bar,
staleness badge, browser-handoff explanation, and auth fallback copy;
some will hide the acting identity behind a generic `Connected` state;
some will keep active affordances after certificate, policy, or
cross-origin guarantees are gone; and some will try to collect
credentials or approval inside the embedded page rather than through the
product-owned review and handoff lanes. That would make release evidence,
support export, parity audit, and security review depend on per-surface
guesswork.

This ADR therefore rides the existing contracts rather than replacing
them:

- `docs/security/safe_preview_trust_classes.md` already freezes the
  trust-class and downgrade posture for isolated remote / embedded
  content;
- ADR-0010 already freezes `browser_handoff_packet`,
  `provider_actor_class`, `grant_resolution_reason`, and host-owned
  approval separation;
- ADR-0013 already freezes docs/help `source_class`,
  `version_match_state`, `freshness_class`, and browser-handoff reason
  reuse;
- `docs/ux/shell_interaction_safety_contract.md` already freezes the
  rule that high-risk preview, apply, and authority renewal are
  product-owned and visible.

What is still missing is the contract that joins those pieces on the
embedded surface itself: the owner/origin chrome, the boundary-card
fields, the reduced-capability state machine, the explicit downgrade
behavior, the auth-handoff cue family, and the exception register that
keeps rare embedded credential-entry flows attributable and visibly
lower-trust.

## Decision

Aureline freezes one **host-owned embedded-surface boundary contract**
for embedded docs/help panes, marketplace/account pages, service
dashboards, auth confirmation surfaces, and extension-hosted web-like
surfaces. Every such surface resolves to exactly one
`embedded_surface_boundary_record` whose boundary card names owner,
origin, publisher/service identity, and data boundary in plain language;
every degraded surface resolves to one of the closed boundary states
below; every out-of-product handoff uses the ADR-0010
`browser_handoff_packet`; every high-risk approval, trust change,
rollback/restore action, update-verification action, or AI-apply review
remains product-owned/native; and every supported auth row is
system-browser-first with device-code or platform-native fallback, while
embedded password collection is permitted only through an explicit,
auditable exception-register row that renders visibly lower-trust cues.

All rules below are stated in terms of contract, vocabulary, and event
names rather than specific crates so later surface work is hygiene, not
re-litigation.

### Canonical host ownership (frozen)

The embedded page does not own the boundary truth. The **host shell**
owns the record that declares what the page is, where it came from, and
what it may honestly claim.

| Surface family | Canonical host owner | Reused contract inputs | Why host-owned |
|---|---|---|---|
| `embedded_docs_help` | docs/help host adapter quoting ADR-0013 `help_status_badge_record` | `source_class`, `version_match_state`, `freshness_class`, browser-handoff reason subset | Prevents embedded docs markup from inventing its own version/freshness truth |
| `embedded_marketplace_or_account` | marketplace/account host adapter quoting ADR-0010 connected-provider state | `provider_class`, `provider_actor_class`, grant scope, health, handoff packet | Prevents account or marketplace pages from collapsing actor, scope, or authority |
| `embedded_service_dashboard` | dashboard host adapter quoting provider/service state | provider identity, target object, health/freshness, browser handoff, boundary state | Prevents hosted dashboards from masquerading as local or native authority |
| `embedded_auth_confirmation` | auth handoff controller | `browser_handoff_packet`, device-code metadata, native step-up state | Prevents auth UI from becoming an ad hoc webview product inside the shell |
| `extension_hosted_surface` | extension host boundary adapter | safe-preview trust class, extension identity, provider identity where applicable | Prevents extension webviews from painting parallel native product chrome |

Rules (frozen):

1. Every embedded surface MUST resolve to exactly one host-owned
   `embedded_surface_boundary_record`. The embedded bytes MAY supply
   metadata claims, but they do not author the canonical owner/origin
   chrome.
2. Copy-only shadow boundary bars are forbidden. If Help/About,
   service-health, activity-center, support export, or diagnostics
   re-render an embedded surface row, they quote the boundary record by
   id rather than mending the text locally.
3. If the host cannot resolve owner, origin, publisher/service identity,
   or data boundary, the surface fails closed to `external_open_only` or
   a stronger degraded state; it does not render an unlabeled embed.
4. Host-owned chrome must remain visually and semantically distinct from
   the embedded body. A page may not make its own header look like the
   canonical boundary card.

### Boundary-card fields (frozen)

The boundary card is the compact, host-rendered object family reused by
embedded docs/help, marketplace/account, service dashboards, and auth
handoff sheets. The card vocabulary is closed.

| Field id | Meaning | Required on |
|---|---|---|
| `owner_label` | Human owner of the embedded surface (`Aureline project`, `Acme Cloud extension`, `GitHub account view`) | every surface |
| `origin_label` | Canonical origin or domain (`https://console.acme.example`, `local signed docs pack`) | every surface |
| `publisher_or_service_label` | Publisher, provider, or service identity distinct from the host shell | every surface |
| `data_boundary_label` | Plain-language boundary (`hosted dashboard`, `provider account surface`, `local mirrored docs snapshot`) | every surface |
| `boundary_state_label` | Current state (`Live verified`, `Policy blocked`, `Cross-origin limited`) | every surface |
| `primary_actions` | Minimum host-owned actions (`Reload`, `Open in browser`, `Copy code`, `Inspect certificate`) | every surface |
| `source_version_freshness` | Source class plus version-match plus freshness summary | `embedded_docs_help` |
| `provider_scope_and_actor` | Provider scope, acting identity, and health/expiry summary | `embedded_marketplace_or_account`, `embedded_service_dashboard` |
| `flow_type_and_return_target` | Exact auth path and return target (`System browser -> Aureline desktop`) | `embedded_auth_confirmation` |
| `fallback_state` | Exact fallback truth (`Device code available`, `Open in browser policy blocked`) | every surface whose live path narrowed |
| `capability_limitations` | Exact missing or forbidden capabilities (`DOM inspection unavailable`, `cannot request native approval`) | every surface with non-trivial reduction |

Rules (frozen):

1. Boundary-card fields MUST be written in plain language on the primary
   surface. Hover-only, inspector-only, or devtools-only disclosure is
   non-conforming.
2. Compact layouts may reorder or collapse spacing, but they may not
   hide the required field set for the surface family.
3. `owner_label`, `origin_label`, `publisher_or_service_label`, and
   `data_boundary_label` are separate fields even when one visual chip
   combines them.
4. A boundary card that offers `Open in browser` or a similar
   out-of-product action MUST quote an ADR-0010
   `browser_handoff_packet`; raw URL launch from ad hoc embed chrome is
   forbidden.
5. `flow_type_and_return_target` and `fallback_state` are part of the
   same card family used for browser and device-code auth. Device-code is
   not a separate vocabulary silo.

### Boundary states and required downgrade behavior (frozen)

Every embedded surface carries exactly one boundary state. The set is
closed.

| State | Meaning | Required downgrade behavior |
|---|---|---|
| `live_verified` | Owner/origin verified, policy allows the surface, certificate/trust checks passed, and the declared live capability set still holds | surface may render within its declared trust class; host-owned chrome remains visible |
| `stale_snapshot` | Last-known-good embedded content or metadata is older than its freshness window but still renderable as a truthful snapshot | remove live-mutation claims, show snapshot age, preserve object identity, allow browser handoff or refresh |
| `policy_blocked` | Policy or entitlement denies the embedded view or one of its declared capabilities | do not render blocked capability as available; show policy reason, support/evidence hook, and any allowed external-open path |
| `certificate_failed` | Certificate, trust-store, or transport-policy verification failed for the origin | do not render embedded body; show metadata-only notice, certificate/policy inspection action, and external-open only if policy explicitly allows |
| `cross_origin_limited` | Origin policy or browser/runtime boundary withholds part of the declared capability set | render only the honest subset, name the exact missing capability, and offer safe fallback such as external open |
| `offline_snapshot` | Network is unavailable but a cached/signed snapshot remains | render snapshot with age and origin, remove live auth/mutation claims, and preserve local-continuity note |
| `external_open_only` | Aureline does not render the body in-product; only the host-owned card and external/device-code/native fallback remain | no embedded body, no impersonated native prompt, preserve return path and object identity on the handoff object |

Rules (frozen):

1. Degradation MUST narrow capability, not blur it. A surface never
   fails from `live_verified` to “looks roughly the same but maybe
   broken.”
2. `certificate_failed` and `policy_blocked` are stronger than
   `stale_snapshot` and `offline_snapshot`. They MUST NOT silently reuse
   an old live affordance as if trust were intact.
3. `cross_origin_limited` MUST name the exact reduced capability rather
   than using a generic “limited” badge alone.
4. `stale_snapshot` and `offline_snapshot` MUST preserve object identity
   and snapshot age so later browser/device-code/open-external actions
   remain attributable to the same object family.
5. Any state below `live_verified` MUST remove or reroute native-reserved
   claims and high-risk approval affordances.

### Native-reserved surfaces and impersonation ban (frozen)

The following product-owned surfaces are reserved to native Aureline
chrome and may never be hosted or convincingly imitated inside embedded
content.

| Reserved surface | Why host-owned | Embedded behavior allowed |
|---|---|---|
| `product_security_messaging` | Security/trust claims must come from the product authority, not from the embedded page | page may request host-native disclosure only |
| `update_verification` | Signature and provenance verification are product-owned truths | page may link to release/docs detail, but host renders verification state |
| `workspace_trust_elevation` | Trust widening changes local authority and must remain attributable | embedded content may open a host-native trust review; it may not present final consent |
| `rollback_or_restore_confirmation` | Restore/rollback changes durable local state | embedded content may request the host-native flow with object identity only |
| `ai_apply_review` | AI apply review, evidence, and rollback are protected product trust surfaces | embedded surface may link or cite; it may not host final apply controls |
| `high_risk_approval_sheet` | High-risk approvals are governed by the interaction-safety and ticket model | embedded surface may request approval but never render the approval sheet itself |

Rules (frozen):

1. Embedded pages MUST NEVER impersonate native system chrome, product
   security messaging, update verification UI, trust prompts,
   irreversible confirmations, rollback/restore confirmations, or AI
   apply review.
2. The host MAY open a native review or approval surface from an
   embedded page, but it MUST re-evaluate policy, trust, route, and
   object identity at that boundary rather than inheriting ambient
   authority from the embed.
3. High-risk approvals remain product-owned/native even when the user is
   currently reading or acting inside an embedded surface.
4. When a native-reserved action cannot be opened or revalidated, the
   embedded surface narrows to inspect/copy/export/open-in-browser or
   device-code paths rather than rendering a weaker in-page substitute.

### Docs/help boundary disclosure (frozen)

Embedded docs/help surfaces reuse ADR-0013; they do not mint a second
docs-truth model.

| Required quoted fields | Source contract | Why they stay explicit |
|---|---|---|
| `source_class` | ADR-0013 `help_status_badge_record` | tells the user whether this is project docs, mirrored docs, vendor docs, etc. |
| `version_match_state` | ADR-0013 `help_status_badge_record` | prevents docs rendered in an embedded pane from implying build applicability they do not have |
| `freshness_class` | ADR-0013 `help_status_badge_record` | keeps stale/offline/help-mirror truth visible |
| `browser_handoff_reason` | ADR-0013 subset of ADR-0010 `reason_code` | keeps “why you are leaving the product” auditable |

Rules (frozen):

1. `embedded_docs_help` MUST quote `source_class`,
   `version_match_state`, and `freshness_class` from the canonical
   docs/help badge record. It MUST NOT recalculate or paraphrase them
   into a surface-local truth model.
2. Project docs still outrank vendor/provider docs on in-scope topics.
   An embedded vendor/provider docs page that is shown in place of
   in-scope project docs without an explicit override disclosure is
   non-conforming.
3. Live external docs remain external truth. When rendered in-product,
   their boundary card MUST still disclose the external/browser-handoff
   reason and privacy posture.
4. `stale_snapshot` and `offline_snapshot` docs surfaces MUST preserve
   source, version, freshness, and age together; “cached docs” alone is
   not enough.

### Marketplace/account and service-dashboard identity chrome (frozen)

Marketplace/account pages and service dashboards are provider-bearing or
service-bearing surfaces. Their identity chrome is not optional.

| Required identity cue | Meaning |
|---|---|
| `provider_class` | Which provider or service family owns the remote state |
| `provider_scope_and_actor` | Host/org/project scope plus acting identity (`you`, `install`, `bot`, `delegated`, policy-injected`) |
| `health_or_expiry` | Whether the session is healthy, stale, limited, blocked, revoked, offline, or expiring |
| `mode_label` | Whether the surface is `inspect_only`, `browser_only`, or another explicitly disclosed bounded mode |
| `target_object_label` | Concrete object the surface is bound to (`payments-prod cluster dashboard`, `Marketplace account`, `Approved extension page`) |

Rules (frozen):

1. A generic `Connected` badge is forbidden. Marketplace/account and
   dashboard surfaces MUST render provider class, scope, health/expiry,
   and acting identity separately enough that audit and support can quote
   them field-for-field.
2. Provider-owned objects rendered in-product MUST remain labeled by
   source, fetch/snapshot time, and mode (`inspect_only`,
   `browser_only`, or another closed mode from the owning contract).
3. If a marketplace/account or dashboard action exceeds client scope or
   requires a live provider-owned mutation Aureline cannot honestly host,
   the surface MUST preserve object identity and explain why the
   in-product lane ended before handing off externally.
4. `policy_blocked`, `stale_snapshot`, `offline_snapshot`, and
   `cross_origin_limited` states must degrade to inspect/copy/export or
   browser handoff. They may not disappear and strand the user without
   context.

### Auth flow classes and handoff cues (frozen)

Authentication and reauthentication surfaces use one closed flow-class
set.

| Flow class | When used | Embedded credential posture | Required cue |
|---|---|---|---|
| `system_browser` | default primary path for sign-in, consent, or reauth where supported | no embedded credential collection | provider/domain, handoff reason, return target, local-continuity note |
| `device_code` | preferred fallback when browser launch is blocked or policy/profile requires non-return-based auth | code copy/entry only; no password entry inside the embed | provider/domain, device-code state, code/expiry, return/help fallback |
| `platform_authenticator_native` | passkey/WebAuthn or platform-native step-up handled by the host or browser/platform surface | not an embedded credential UI | exact reason for step-up and the protected action it gates |
| `embedded_session_refresh` | narrow renewal inside an already-authenticated session when no password entry occurs and no scope widens | session refresh without password collection | lower capability scope, no scope widening, host-native approval still separate |
| `embedded_password_exception` | rare, explicitly approved exception only | password entry exception; visibly lower-trust | exception id, lower-trust badge, exit path, no remembered approval |
| `not_applicable` | non-auth surfaces | none | no auth lane shown |

Rules (frozen):

1. Supported auth rows are **system-browser first**. When that path is
   unavailable or blocked, `device_code` is the preferred reviewable
   fallback unless a platform-native authenticator path or a registered
   exception applies.
2. `platform_authenticator_native` is a host-native or browser/platform
   surface, not a general-purpose embedded auth page.
3. `embedded_session_refresh` is allowed only when it does **not**
   collect a password, does not widen scope, and does not replace any
   required host-native approval or review.
4. `embedded_password_exception` is never the default happy path. It
   requires an active exception-register row, visible lower-trust cues,
   and a clear exit to system browser or device code where available.
5. Auth-handoff cards for `system_browser` and `device_code` belong to
   the same object family. They MUST carry provider/domain, flow type,
   reason for handoff, return target, local-continuity note, fallback
   state, and code/expiry if relevant.
6. Embedded credential collection MUST NOT claim or imply that final
   trust elevation, update verification, rollback approval, or AI apply
   review happened inside the embedded surface. Those remain
   product-owned/native.
7. A browser/device-code handoff card MUST preserve current object
   identity so the user returns to the same workspace/object family
   rather than to a generic “accounts” screen with lost context.

### Embedded-auth exception register (frozen)

Any embedded credential-entry deviation from the primary flow must be
enumerated in one exception register.

| Field | Meaning |
|---|---|
| `exception_id` | Stable, exportable id for the exception |
| `provider_or_domain` | Exact provider/domain/origin the exception applies to |
| `approved_embedded_flow_class` | Which embedded flow is permitted (`embedded_session_refresh` or `embedded_password_exception`) |
| `exception_reason` | Why the normal path is unavailable or insufficient |
| `exception_scope_class` | How narrow the exception is (`session_refresh_only`, `limited_account_link`, `no_high_risk_actions`) |
| `required_lower_trust_cues` | Visible cues that make the lower-trust posture explicit |
| `review_owner` / `review_state` / `review_by` | Who owns the exception and when it expires or must be renewed |
| `repair_or_exit_path` | What the user or operator can do instead or next |

Rules (frozen):

1. Any embedded password-entry flow requires an active exception row.
   Without that row, the surface is denied and must route to
   `system_browser`, `device_code`, or `external_open_only`.
2. Exception rows are reviewable and exportable from the same object
   family as the auth cue. Support/export, diagnostics, and release
   evidence do not retype them by hand.
3. Exception rows only narrow. They may not authorize native-reserved
   surfaces, skip host-native approval, or convert a high-risk protected
   action into an in-page confirmation.
4. Expired or revoked exception rows immediately downgrade the
   associated surface to a non-credential path.

### Boundary audit and export rules (frozen)

The boundary contract exports:

- `embedded_surface_boundary_record`
- `embedded_auth_exception_record`
- `embedded_surface_boundary_audit_event_record`

Rules (frozen):

1. Boundary audit events MUST carry ids and typed vocabulary only. Raw
   HTML, raw cookies, raw browser session storage, raw passwords, raw
   device codes beyond what the visible cue already declares, and raw
   external URLs are forbidden on the boundary packet.
2. Every downgrade, external-open path, auth-handoff start, exception
   render, or native-surface denial emits a typed audit event so later
   parity review can compare desktop UI, support export, and release
   evidence without scraping UI copy.
3. The boundary packet records the host-side truth even when the
   embedded body is withheld. `certificate_failed`,
   `policy_blocked`, and `external_open_only` are first-class rendered
   states, not gaps in telemetry.

## Consequences

- Embedded docs/help, marketplace/account, service-dashboard, and auth
  surfaces now share one boundary vocabulary instead of inventing local
  bars, badges, or auth fallback text.
- Future embedded surfaces have a closed degraded-state set and a
  product-owned/native list they cannot bypass or imitate.
- Browser and device-code auth flows now live in the same auditable card
  family, which makes release evidence and support export mechanical.
- Embedded password collection is no longer a “just this one page”
  escape hatch; it requires an explicit exception row and visibly
  lower-trust cues.
- Later marketplace, docs/help, extension-webview, service-dashboard,
  and auth drills can validate boundary cards and audit packets without
  inventing more vocabulary.
- Runtime implementation of webviews, browser launchers, device-code
  brokers, and host-side controllers remains follow-up work.

## Alternatives considered

- **Let each embedded surface define its own boundary bar and auth copy.**
  Rejected. That recreates the drift ADR-0010 and ADR-0013 already
  closed and makes parity audit impossible.
- **Treat embedded auth as equivalent to system-browser auth whenever the
  provider supports it.** Rejected. This would erase the security,
  accessibility, and impersonation boundary the source documents require.
- **Rely only on the safe-preview trust classes.** Rejected.
  `RawText`/`SanitizedRich`/`IsolatedRemoteActive` explain rendering and
  downgrade posture, but they do not carry docs source/version/freshness
  truth, provider actor/scope chrome, or auth-handoff cues.
- **Ban embedded hosted surfaces entirely and require the system browser
  for everything.** Rejected. The product still needs bounded in-product
  docs/help, account, dashboard, and extension-hosted surfaces; the
  problem is not the existence of an embed, but whether its boundary is
  explicit and host-owned.

If this ADR had not landed, the default narrowing posture would have
left only static or sanitized docs/help and explicit open-in-browser
hosted pages, with no embedded password collection and no in-product
marketplace/account/dashboard/auth lane beyond browser or device-code
disclosure. That did **not** happen; the broader shared boundary model
described above is now the frozen contract.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md:1336` —
  “browser write surfaces should reuse” desktop/CLI policy and audit.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1338` —
  “visibly reduced capability posture”.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:4368` —
  “system-browser first”.
- `.t2/docs/Aureline_Technical_Design_Document.md:1860` —
  docs must explain source, version, freshness, and browser send-out.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:8535` —
  browser/device-code handoff card fields.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:8546` —
  embedded webviews must never impersonate native/system security chrome.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:9444` —
  identity and origin disclosure on hosted surfaces.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:9447` —
  destructive/high-risk approval stays host-native.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:15909` —
  `Owner`, `Origin`, `Boundary`, `State`, `Actions` template.
- `docs/security/safe_preview_trust_classes.md:189` —
  embedded content needs visible origin and permission summary.
- `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md:223` —
  `open_in_provider` uses typed browser handoff.
- `docs/adr/0013-docs-help-service-health-truth.md:119` —
  source/version/freshness/browser-handoff fields remain separately
  addressable.

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0021`
- Boundary schema: `schemas/ux/embedded_surface_boundary.schema.json`
- Owner/origin seed: `artifacts/ux/owner_origin_chrome_seed.yaml`
- Worked fixtures: `fixtures/ux/embedded_boundary_cases/`
- Related contracts:
  `docs/security/safe_preview_trust_classes.md`,
  `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`,
  `docs/adr/0013-docs-help-service-health-truth.md`,
  `docs/ux/shell_interaction_safety_contract.md`

## Supersession history

None.
