# Embedded-surface audit packet

Companion artifacts and contracts:

- ADR-0015 — `docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`
- Render-side contract — `docs/ux/embedded_surface_boundary_cards.md`
- Upstream record schema — `schemas/ux/embedded_surface_boundary.schema.json`
- Boundary-card schema — `schemas/ux/embedded_boundary_card.schema.json`
- Owner/origin chrome seed — `artifacts/ux/owner_origin_chrome_seed.yaml`
- Owner/origin chrome review — `artifacts/ux/owner_origin_chrome_review.yaml`
- Boundary-case corpus — `fixtures/ux/embedded_boundary_cases/`

This packet is the single shared format reviewers use to score embedded
surfaces under the boundary contract. It is a **review template**: every
column resolves to a closed vocabulary already frozen in ADR-0015, the
schemas above, and the render-side contract. Free text appears only in
the trailing `notes` column.

The packet covers every embedded surface family together so security,
UX, and docs/help review can read one boundary truth across surfaces:

- embedded docs / help panes,
- embedded marketplace and account pages,
- embedded service / customer-control-plane dashboards,
- webview-backed extension UIs,
- embedded auth-confirmation / handoff sheets and the auth-supporting
  exception register.

The packet does not invent new tokens. Every axis below is sourced from
one of the upstream contracts. Reviewers that need a new boundary state,
permission class, action partition role, browser-fallback posture,
fallback target, capability limitation, native-reserved surface, auth
flow class, audit event id, or denial reason open a decision row against
ADR-0015 instead of widening this template.

Per-task and per-milestone identifiers stay out of this packet. Rows are
attributable through `surface_id`, `card_id`, `exception_id`,
`audit_event_id`, and the canonical fixture path.

## 0. Packet header

Every audit packet SHOULD embed this header verbatim. Closed-vocabulary
fields without a stable ref use the seed values named in
`artifacts/ux/owner_origin_chrome_seed.yaml`.

- **Packet id:** `<embedded-surface-audit-packet-id>`
- **Packet state:** `draft` | `in_review` | `accepted` | `blocked` | `superseded`
- **Reviewer (DRI):** `@<handle>`
- **Co-reviewers:** `@<security-trust-review>`, `@<ux-review>`,
  `@<docs-help-review>`
- **Audit date:** `YYYY-MM-DDTHH:MM:SSZ`
- **Build identity:** `<exact_build_identity_ref>` from
  `schemas/build/exact_build_identity.schema.json`
- **Identity mode under test:** one of `account_free_local` |
  `self_hosted_org` | `managed_workspace`
- **Trust state under test:** `trusted` | `restricted`
- **Network posture:** `online` | `offline` | `mirror_only`
- **Surface families exercised:** subset of `embedded_docs_help`,
  `embedded_marketplace_or_account`, `embedded_service_dashboard`,
  `embedded_auth_confirmation`, `extension_hosted_surface`
- **Boundary states exercised:** subset of the seven closed states
  named in §2
- **Boundary records in scope:** list of upstream
  `embedded_surface_boundary_record` `surface_id` values
- **Boundary cards in scope:** list of `embedded_boundary_card_record`
  `card_id` values
- **Exception rows referenced:** list of `exception_id` values from
  active `embedded_auth_exception_record` rows
- **Audit events referenced:** list of `audit_event_id` rows from
  `embedded_surface_boundary_audit_event_record`
- **Active waivers:** waiver packet ids or `none`
- **Fixtures cited:** list of fixture paths from
  `fixtures/ux/embedded_boundary_cases/`
- **Notes:** one paragraph; never a substitute for a typed field.

## 1. Executive summary

Two or three sentences naming:

1. how many surface rows were exercised in this packet,
2. how many scored `pass`, `pass_with_notes`, or `fail` on the per-row
   verdict in §2,
3. whether any row missed a required chrome field, action partition
   role, browser-fallback posture, capability limitation, or
   native-reserved surface disclosure.

The summary feeds the owner/origin chrome review at
`artifacts/ux/owner_origin_chrome_review.yaml` without restating
per-row thresholds.

## 2. Per-row audit table

Reviewers fill **one row per embedded surface under test**. Every column
resolves to a closed vocabulary; free text goes only in the `notes`
column at the end of each row.

| Field | Vocabulary / source |
|---|---|
| `surface_id` | upstream `embedded_surface_boundary_record.surface_id` |
| `surface_family` | `embedded_surface_boundary.schema.json#/$defs/surface_family` |
| `container_surface_class` | `embedded_surface_boundary.schema.json#/$defs/container_surface_class` |
| `card_id` | `embedded_boundary_card_record.card_id` projecting the surface |
| `boundary_state` | `embedded_surface_boundary.schema.json#/$defs/boundary_state` |
| `permission_class` | `embedded_boundary_card.schema.json#/$defs/permission_class` |
| `auth_flow_class` | `embedded_surface_boundary.schema.json#/$defs/auth_flow_class` (or `not_applicable`) |
| `credential_collection_mode` | `embedded_surface_boundary.schema.json#/$defs/credential_collection_mode` (or `not_applicable`) |
| `browser_fallback_posture_class` | `embedded_boundary_card.schema.json#/$defs/browser_fallback_posture_class` |
| `fallback_target_class` | `embedded_boundary_card.schema.json#/$defs/fallback_target_class` |
| `chrome_field_set_complete` | `pass` if every required chrome field is rendered, `fail` otherwise |
| `native_actions_kept_native` | `pass` if every native-reserved surface stays in `product_owned_native` chrome, `fail` if any embedded body imitates one |
| `embedded_actions_kept_embedded` | `pass` if every `embedded_inspect_only` / `embedded_request_only` action stays inside the embedded body and never claims final authority, `fail` otherwise |
| `chrome_inheritance_complete` | `pass` if all seven inheritance axes are honored, `fail` otherwise |
| `layout_constraints_satisfied` | `pass` if the seven mandatory `layout_constraint_id` values render, `fail` otherwise |
| `freshness_disclosed` | `pass` / `not_applicable` / `fail` per §3 freshness semantics |
| `verdict` | `pass` | `pass_with_notes` | `fail` |
| `notes` | one paragraph; reviewer comment, never a substitute for a typed field |

Each row is attributable to exactly one upstream
`embedded_surface_boundary_record` and exactly one
`embedded_boundary_card_record`. Rows that disagree are non-conforming
and the upstream record wins per ADR-0015.

## 3. Boundary-state coverage matrix

The table below pins the minimum coverage every audit packet aims for.
A packet that does not exercise every cell SHOULD declare the missing
cell in §0 under `notes` rather than silently widening scope.

| Boundary state | Required surface families | Required corpus exemplar |
|---|---|---|
| `live_verified` | docs/help, marketplace/account, service-dashboard, auth-confirmation, extension-hosted | `docs_help_live_verified_card.yaml`, `auth_confirmation_system_browser_first_card.yaml` |
| `stale_snapshot` | docs/help, marketplace/account | `docs_help_stale_snapshot_card.yaml`, `marketplace_account_stale_scope.json` |
| `policy_blocked` | service-dashboard, marketplace/account, extension-hosted | `service_dashboard_policy_blocked_card.yaml`, `marketplace_account_external_open_only_card.yaml` |
| `certificate_failed` | service-dashboard, extension-hosted | `service_dashboard_certificate_failed_card.yaml`, `service_dashboard_certificate_failed.json` |
| `cross_origin_limited` | extension-hosted, service-dashboard | `extension_hosted_cross_origin_limited_card.yaml`, `extension_hosted_unsupported_capability_card.yaml` |
| `offline_snapshot` | marketplace/account, docs/help | `marketplace_account_offline_snapshot_card.yaml` |
| `external_open_only` | marketplace/account, auth-confirmation, docs/help | `marketplace_account_external_open_only_card.yaml`, `docs_help_external_open_only_card.yaml`, `auth_confirmation_device_code_fallback.json` |

For the `embedded_docs_help` rows, freshness disclosure means rendering
`source_class`, `version_match_state`, `freshness_class`, and (for
`stale_snapshot` / `offline_snapshot`) a `snapshot_age_label`. For the
`embedded_marketplace_or_account` and `embedded_service_dashboard`
rows, freshness disclosure means rendering provider scope, acting
identity, health state, and a fetch-time / staleness summary on the
card.

## 4. Owner / origin chrome review

Owner-and-origin honesty is graded against the closed
`required_chrome_field` set per surface family:

- `embedded_docs_help` → owner / origin / publisher-or-service / data
  boundary / state / primary actions / source-version-freshness /
  fallback state / capability limitations.
- `embedded_marketplace_or_account` and `embedded_service_dashboard` →
  owner / origin / publisher-or-service / data boundary / state /
  primary actions / provider-scope-and-actor / fallback state /
  capability limitations.
- `embedded_auth_confirmation` → owner / origin / publisher-or-service
  / data boundary / state / primary actions / flow-type-and-return-
  target / fallback state / capability limitations.
- `extension_hosted_surface` → owner / origin / publisher-or-service /
  data boundary / state / primary actions / provider-scope-and-actor
  (where applicable) / fallback state / capability limitations, plus
  `owner_identity.class == extension_bundle` and a publisher / service
  identity distinct from the host product.

The review explicitly distinguishes:

| Action partition role | Chrome posture |
|---|---|
| `product_owned_native` | Renders in host chrome only, carries final authority for security messaging, update verification, trust elevation, rollback / restore, AI apply review, and high-risk approval. |
| `product_owned_handoff` | Renders in host chrome only, with a quoted `browser_handoff_packet_ref` or `device_code_ref`. |
| `embedded_inspect_only` | May render inside the embedded body; never claims mutation authority. |
| `embedded_request_only` | Round-trips through the host shell, which re-evaluates policy, trust, route, and object identity at the boundary. |

Reviewers MUST be able to answer for each row:

1. which actions are native / product-owned and which are delegated to
   the embedded content,
2. whether the embedded body paints anything that visually impersonates
   `product_security_messaging`, `update_verification`,
   `workspace_trust_elevation`,
   `rollback_or_restore_confirmation`, `ai_apply_review`, or
   `high_risk_approval_sheet`,
3. whether `product_owned_native` actions and embedded actions ever
   blur into one toolbar, and
4. whether any `Open in browser` / `Switch to device code` action
   quotes an upstream `browser_handoff_packet` / `device_code_ref`
   instead of a raw URL or local synonym.

A row that cannot answer all four cleanly fails the review.

## 5. Boundary-case corpus contract

The corpus at `fixtures/ux/embedded_boundary_cases/` carries one
exemplar per closed combination of surface family × boundary state ×
permission class × auth flow class that the audit packet exercises.
Cases are split into two contracts that share the same
`surface_id_ref`:

- Upstream record cases (`.json`) exercise the
  `embedded_surface_boundary_record`,
  `embedded_auth_exception_record`, and
  `embedded_surface_boundary_audit_event_record` family.
- Render-side card cases (`.yaml`) exercise the
  `embedded_boundary_card_record` projection.

For every case, reviewers MUST be able to confirm:

1. **Owner / origin honesty.** Owner label and class, publisher /
   service label and class, origin label, host or domain label, origin
   verification state, and data boundary class are visible and disjoint
   from the embedded body's local copy.
2. **Freshness honesty.** Stale, offline, certificate-failed, and
   policy-blocked rows render their freshness or trust-loss truth on
   the card; they do not silently reuse a `live_verified` affordance
   set.
3. **Fallback honesty.** Every state below `live_verified` carries a
   `browser_fallback` row whose posture and target class match the
   upstream record. `external_open_only` cards always cite a non-empty
   fallback target class.
4. **Capability honesty.** The capability limitation list names every
   missing capability the user can detect from the surface; the card
   never renders an action whose permission class no longer admits it.
5. **Native-reserved disclosure.** The
   `reserved_native_surfaces_host_owned` set is non-empty on every
   case, including `live_verified`. The set never shrinks across
   states.
6. **Audit attributability.** A denial or downgrade event on the row
   maps to a typed `audit_event_id` from the closed event vocabulary
   and (where relevant) a closed `denial_reason` token.

Cases that combine these axes in a single fixture (for example,
`certificate_failed` + `host_native_review_or_approval` fallback +
explicit capability narrowing) are encouraged so audit packets do not
have to braid coverage by hand.

## 6. Audit-event coverage table

The packet asks reviewers to confirm one row per audit event id that
the surfaces exercised, sourced from
`embedded_surface_boundary.schema.json#/$defs/audit_event_id`.

| Audit event id | Required denial reason | Required ref(s) |
|---|---|---|
| `embedded_surface_boundary_rendered` | none | `surface_id_ref` |
| `embedded_surface_boundary_downgraded` | none | `surface_id_ref`, prior + new `boundary_state` |
| `embedded_surface_opened_in_browser` | none | `surface_id_ref`, `browser_handoff_packet_ref` |
| `embedded_auth_handoff_started` | none | `surface_id_ref`, `browser_handoff_packet_ref`, `auth_flow_class` |
| `embedded_auth_device_code_offered` | none | `surface_id_ref`, `device_code_ref` |
| `embedded_auth_exception_rendered` | none | `surface_id_ref`, `exception_id_ref` |
| `embedded_native_surface_request_denied` | one of the closed `denial_reason` tokens | `surface_id_ref`, `denial_reason` |
| `embedded_surface_schema_version_bumped` | none | prior + new `embedded_surface_boundary_schema_version` |

Rows that emit `embedded_native_surface_request_denied` MUST cite a
`denial_reason` from the closed set in
`embedded_surface_boundary.schema.json#/$defs/denial_reason`. Rows
that emit `embedded_auth_exception_rendered` MUST cite an
`exception_id_ref` from `fixtures/ux/embedded_boundary_cases/` or the
live exception register.

## 7. Acceptance gates

A packet is accepted when:

1. Every row in §2 has a verdict of `pass` or `pass_with_notes`,
2. The §3 coverage matrix is either fully exercised or its missing
   cells are explicitly enumerated under `notes`,
3. The §4 owner / origin chrome review can be answered cleanly for
   every row (native vs embedded, no impersonation, no toolbar blur,
   handoff packets quoted),
4. The §5 corpus contract holds for every cited fixture, and
5. The §6 audit-event table cites a typed `audit_event_id` plus the
   required refs and denial reasons for every audit interaction the
   surfaces produced.

Failure on any gate means the packet returns to `draft`. The packet
MUST NOT be coerced into `accepted` by widening the closed
vocabularies; new tokens require a decision row against ADR-0015.

## 8. Out of scope

- the webview engine, certificate store, browser launcher, device-code
  broker, marketplace runtime, or extension host runtime;
- per-provider docs, marketplace, dashboard, or auth UI specifics
  beyond the closed identity, scope, health, and fallback vocabularies
  the boundary card renders;
- new boundary states, permission classes, action partition roles,
  browser-fallback postures, fallback targets, capability limitations,
  native-reserved surfaces, auth flow classes, audit event ids, or
  denial reasons. Any of these requires a decision row against
  ADR-0015 (decision register
  `artifacts/governance/decision_index.yaml`, decision id D-0021).
