# Provider-linked object header, browser-handoff sheet, and return-anchor contract

This document freezes how Aureline renders provider-owned objects so
they feel like typed linked companions to local truth instead of either
opaque browser tabs or disguised local objects. It binds together three
things the product must not invent twice:

1. the **provider-linked object header** the desktop, CLI, companion,
   notification, support-export, activity-center, audit, and AI-context
   surfaces render on top of every provider-owned object,
2. the **browser-handoff sheet** that mediates every routed-through-the-
   browser action so destination class, reason code, privacy-or-data-
   loss consequence, expected authority on the destination, return
   anchor, replay expiry, and typed local-or-cached alternatives are
   visible *before* the system browser is launched,
3. the **notification, activity-center, and exported-evidence reuse
   rules** so provider class, canonical host, scope, freshness tier,
   handoff reason, and acting actor class survive together when a
   provider-linked object is rendered outside the originating panel.

The machine-readable schemas live at:

- [`/schemas/providers/provider_link_header.schema.json`](../../schemas/providers/provider_link_header.schema.json)
  — `provider_link_header_record`,
  `handoff_reason_reuse_record`,
  `header_degradation_event_record`.
- [`/schemas/providers/browser_handoff_sheet.schema.json`](../../schemas/providers/browser_handoff_sheet.schema.json)
  — `browser_handoff_sheet_record`,
  `local_or_cached_alternative_record`,
  `browser_handoff_sheet_audit_event_record`.

The owning `connected_provider_record` (the registry-row anchor every
record below points at through `connected_provider_record_id`) is
exported by
[`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
and is governed by
[`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md);
this contract does not redefine that record. Worked fixtures live at
[`/fixtures/providers/provider_link_cases/`](../../fixtures/providers/provider_link_cases/).

This contract **composes with and does not replace** the ADR-0010
browser-handoff-packet, approval-ticket, and connected-provider-record
contracts; the ADR-0007 secret-broker projection-mode and storage-class
contracts; the ADR-0001 workspace-trust posture; the ADR-0008 settings
/ policy-bundle resolver; the ADR-0009 execution-context model; the
[`connected-account registry contract`](./connected_account_registry_contract.md);
and the
[`provider-mode contract`](./provider_mode_contract.md).
Where this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does not ship a live header renderer, a live browser-
launch service, or a live notification / activity-center plumbing
layer. It freezes the record shapes those implementations will read
and write. The eventual provider-mode crate's Rust types are the
schema of record; the JSON Schema exports are the cross-tool boundary
every non-owning surface reads.

## Why freeze this now

Every later lane that surfaces a provider-owned object — a pull
request, an issue, a check run, a docs page, a release artifact, an
admin console, a consent flow — has to answer the same five questions
on every protected surface:

1. *Which provider class and canonical host is this object attached to,
   and what is its tenant / org / project scope?*
2. *Is the local side currently authoritative (an unsent draft), is the
   provider side authoritative (a remote-owned object the local side is
   a typed companion to), is this a mirror-derived rendering of provider
   state, or is it an inspect-only imported snapshot?*
3. *What edit mode is this header in right now — `draft`, `publish`,
   `browser-only`, `inspect-only`, or `blocked-for-repair` — and which
   actor class is the user acting as, with what badge label?*
4. *If we are about to leave Aureline scope into the system browser,
   what is the destination class, the reason code, the privacy-or-data-
   loss consequence, the expected authority on the destination, the
   return anchor, the replay expiry, and the typed local-or-cached
   alternative if full in-product parity is unavailable?*
5. *If the underlying actor is revoked / suspended, the freshness floor
   has drifted, the provider is offline, the source is mirror-derived
   rather than authoritative, or local policy blocks the surface, how
   does the header degrade visibly so inspect / copy / export remain
   available rather than disappearing or silently keeping its claim of
   authority?*

Without one frozen contract: the review surface invents a "Pull request
on github.com" header, the issues surface invents a "GitHub issue"
header, the CI surface invents a "Check run" header, the release
surface invents a "Release on the registry" header, the notification
center invents a fifth, the support export collapses all of them into
plain URLs, and a generic "Open in browser" affordance shows up on
every protected surface without naming destination class, reason code,
or return anchor — flattening typed handoff intent into raw URLs.

This contract closes that gap with **one header vocabulary, one sheet
vocabulary, one alternative vocabulary, one degradation vocabulary, and
one reuse rule set** every protected surface and every post-incident
consumer reads.

## Scope

Frozen at this revision:

- the **header record** the desktop, CLI, companion, support-export,
  activity-center, AI-context, and audit surfaces render on every
  provider-owned object — provider class, canonical host, object type,
  local-versus-provider source, last-sync time, freshness tier, scope,
  edit mode, current mutation mode, acting actor class, available
  inspect / copy / export actions, and trust posture;
- the **browser-handoff sheet record** that mediates every system-
  browser launch from a protected surface — destination class, reason
  code, reason summary, privacy-or-data-loss consequence class plus
  summary, expected authority on the destination plus summary, return
  anchor, replay posture, replay expiry, originating browser-handoff
  packet, originating provider-link header, sheet state, and at least
  one typed local-or-cached alternative;
- the **local-or-cached alternative record** the sheet offers when full
  in-product parity is unavailable, with a closed alternative-class
  vocabulary and required `linked_record_ref` / `repair_hook_ref`
  citations;
- the **notification, activity-center, and exported-evidence reuse
  rules** so provider class, canonical host, scope summary, freshness
  tier, handoff reason, and acting actor class survive together on
  every reuse surface;
- the **degradation event record** and the typed degradation classes,
  retained-action and removed-action vocabularies that govern how a
  header degrades for stale, revoked, offline, mirror-derived, or
  policy-blocked transitions;
- the **redaction posture** that keeps raw URLs, raw tokens, raw
  callback bodies, raw delegated-token bodies, raw policy-injector
  material, and raw provider payloads off this boundary on every
  surface;
- the **audit-event reuse** rules that route sheet-level intent
  (alternative chosen, user cancelled before launch, sheet expired)
  onto the ADR-0010 `provider_handoff` stream alongside the existing
  packet-level events.

## Out of scope

- Rendering browser-handoff UI. The header, sheet, alternative, and
  degradation records are the cross-tool boundary; the eventual desktop
  / companion / CLI surfaces will read these records and render them
  through their own design-system contracts.
- Implementing provider navigation. The system-browser launcher, the
  provider-side adapters, and the live callback listener are out of
  scope; this contract freezes the record shapes those implementations
  will read and write.
- Live policy-bundle authoring, settings UI, and admin review consoles.
  ADR-0008 owns the policy-resolver shape; this contract reuses the
  resolver's output through `policy_lock_class` and through the
  degradation event's `policy_blocked` class.
- Live freshness probes, live mirror-detection heuristics, and live
  provider-health probes. The contract names the typed freshness tier
  and degradation classes those probes will write into; the probes
  themselves land with each provider adapter.

## 1. Provider-linked object header

Every provider-owned object surfaced on a protected Aureline surface
MUST be rendered behind a typed `provider_link_header_record`. The
header is the contract: it names provider class, canonical host, object
type, local-versus-provider source, last-sync time, freshness tier,
scope, edit mode, current mutation mode, acting actor class, available
inspect / copy / export actions, and trust posture together. A surface
that surfaces only a subset (for example, an issue title without
provider class, or a pull-request body without freshness tier) is
forbidden.

### 1.1 Frozen vocabularies

| Field                         | Vocabulary                                                                                                                                                                                                                                                                                                                                                |
|-------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `provider_class`              | `review_or_code_host`, `issue_or_planning_tracker`, `ci_or_check_provider`, `docs_or_portal_provider`, `identity_or_enterprise_provider`, `callback_or_event_provider`, `ai_provider`, `package_registry_provider`, `release_publisher_provider`, `managed_admin_provider` (reused from ADR-0010).                                                          |
| `object_type`                 | `pull_request`, `issue_or_work_item`, `check_run`, `release_artifact`, `docs_page`, `audit_entry`, `admin_surface`, `consent_flow`, `package_version`, `registry_entry`, `principal_subject`, `install_target`, `tenant_or_org`, `other`.                                                                                                                  |
| `local_or_provider_source`    | `local_authoritative`, `provider_authoritative`, `mirror_derived`, `imported_snapshot_only`, `local_draft_only_no_provider_state`.                                                                                                                                                                                                                          |
| `freshness_tier`              | `live_authoritative`, `recently_synced_fresh`, `bounded_stale`, `unbounded_stale`, `unknown_freshness_repair_required`.                                                                                                                                                                                                                                     |
| `edit_mode`                   | `draft`, `publish`, `browser_only`, `inspect_only`, `blocked_for_repair`.                                                                                                                                                                                                                                                                                  |
| `actor_class`                 | `human_account`, `installation_or_app_grant`, `delegated_user_token`, `project_scoped_grant`, `policy_injected_service_identity`, `unknown_actor_class` (reused from ADR-0010).                                                                                                                                                                            |

### 1.2 Edit-mode ↔ mutation-mode binding

The header's `edit_mode` projects exactly one ADR-0010 mutation mode at
a time:

| `edit_mode`            | Required `current_mutation_mode`                |
|------------------------|-------------------------------------------------|
| `draft`                | `local_draft`                                   |
| `publish`              | `publish_now` or `deferred_publish`             |
| `browser_only`         | `open_in_provider`                              |
| `inspect_only`         | `inspect_only`                                  |
| `blocked_for_repair`   | (omitted)                                       |

Schema enforcement makes the binding mechanical. Surfaces MAY narrow
(an admin policy may forbid `publish_now` on a surface); no surface MAY
widen, redefine, or rename a mode.

### 1.3 Source-class ↔ supporting-record binding

| `local_or_provider_source`            | Required supporting record                                                                  |
|---------------------------------------|---------------------------------------------------------------------------------------------|
| `local_authoritative`                 | `originating_local_draft_ref` (via paired `provider_object_relation_record` when present).  |
| `provider_authoritative`              | `import_session_ref` MUST be present.                                                       |
| `mirror_derived`                      | `import_session_ref` MUST be present; the header MUST disclose the mirror posture.          |
| `imported_snapshot_only`              | `import_session_ref` MUST be present.                                                       |
| `local_draft_only_no_provider_state`  | MUST pair with `edit_mode = draft`; `import_session_ref` is forbidden.                       |

### 1.4 Rules (frozen)

1. A header without a `connected_provider_record_id` is forbidden.
2. A header whose `actor_class` is `unknown_actor_class` MUST pair
   with `edit_mode = blocked_for_repair`. A surface that cannot
   resolve the actor MUST route to repair, not silently publish under
   an unresolved actor.
3. A header whose `freshness_tier` is `unbounded_stale` MAY NOT pair
   with `edit_mode = publish` or `edit_mode = browser_only`; the
   header MUST degrade to `inspect_only` or `blocked_for_repair`
   until refreshed.
4. A header whose `freshness_tier` is `unknown_freshness_repair_required`
   MUST pair with `edit_mode = blocked_for_repair`.
5. A header whose `edit_mode` is `browser_only` MUST cite a
   `browser_handoff_packet_ref`. A generic `Open in browser` affordance
   without a typed packet (and without a paired
   `browser_handoff_sheet_record`) is forbidden.
6. A header whose `edit_mode` is `publish` MUST cite either an
   `approval_ticket_ref` (for `publish_now`) or a
   `publish_later_queue_item_ref` (for `deferred_publish`); the
   schema enforces this through an `anyOf` allOf clause.
7. The `available_actions` block MUST always carry typed inspect /
   copy / export action lists; degradation MAY shrink them but MAY
   NOT empty all three at once. Inspect / copy / export remain
   available where safe — disappearing the header is forbidden.
8. Copy and export actions carry opaque refs and structured summaries
   only; raw URLs, raw tokens, raw callback bodies, and raw provider
   bodies never enter their projections.

## 2. Browser-handoff sheet

Every system-browser launch from a protected provider-linked surface
MUST be mediated by a typed `browser_handoff_sheet_record` paired with
its underlying `browser_handoff_packet_record` (ADR-0010). The sheet
is the user-facing disclosure; the packet is the typed cross-process
authorization. A surface that launches the system browser without a
sheet — or with a sheet that is missing destination class, reason code,
privacy-or-data-loss consequence class, expected authority, return
anchor, replay expiry, or at least one typed local-or-cached
alternative — is forbidden.

### 2.1 Required fields

Every `browser_handoff_sheet_record` names:

- `sheet_id` — opaque, stable, safe to log.
- `originating_browser_handoff_packet_ref` — opaque ref to the
  ADR-0010 packet the sheet mediates.
- `originating_provider_link_header_ref` — opaque ref to the header
  the sheet was minted from (empty string only for sheets minted from
  a CLI / activity-center surface where no header is currently
  rendered).
- `connected_provider_record_id` — opaque ref to the registry row.
- `destination_class` — frozen vocabulary (reused from ADR-0010).
- `destination_summary` — short reviewable sentence naming the
  destination class, the canonical host, and the typed object the
  user will see on the destination. A generic "a website" / "an
  external page" summary is forbidden.
- `destination_object_identity` — opaque object identity of the
  destination object (omitted only for admin / consent-flow handoffs
  that have no provider-side object identity yet).
- `reason_code` — frozen vocabulary (reused from ADR-0010).
- `reason_summary` — short reviewable sentence quoting the reason in
  human-legible form. Surfaces render this verbatim; flattening it
  into a generic "External link" is forbidden.
- `privacy_or_data_loss_consequence_class` — frozen vocabulary
  (`none_disclosed`, `leaves_workspace_scope`, `crosses_canonical_host`,
  `provider_telemetry_disclosed`, `requires_provider_login_session`,
  `bypasses_local_redaction`,
  `shares_workspace_object_identity_with_provider`,
  `irreversible_external_publish`).
- `privacy_or_data_loss_consequence_summary` — required for every
  consequence class except `none_disclosed`.
- `expected_authority_on_destination` — frozen `provider_actor_class`.
- `expected_authority_summary` — short reviewable sentence quoting
  the actor class the user will be acting as on the destination.
- `return_anchor` — frozen anchor shape; extends ADR-0010 with
  `header_anchor` so a sheet that originates on a provider-link header
  can return to that header.
- `replay_posture` — frozen vocabulary (`single_use`, `bounded_reuse`,
  `read_only_resumable`).
- `replay_expires_at` — monotonic timestamp after which the sheet's
  underlying handoff packet is no longer admissible. The sheet's
  user-confirm action MUST refuse after this time and re-mint a fresh
  sheet rather than silently extending the window.
- `local_or_cached_alternative_refs` — opaque refs to at least one
  `local_or_cached_alternative_record` (empty list is forbidden).
- `approval_ticket_ref` — required for every reason code except
  `provider_consent_flow`.
- `sheet_state_class` — frozen vocabulary
  (`ready_for_user_confirmation`, `user_confirmed_pending_launch`,
  `launched_awaiting_callback`, `callback_validated`,
  `callback_rejected`, `user_cancelled`, `expired_unused`,
  `revoked_before_launch`, `alternative_chosen`).
- `policy_context` — policy epoch, trust state, execution-context id
  at mint time.
- `redaction_class` — declared redaction posture.
- `minted_at` — monotonic timestamp.
- `sheet_summary` — top-level reviewable sentence naming destination
  class, reason code, privacy-or-data-loss consequence class, expected
  authority, return anchor, and replay expiry together.

### 2.2 Sheet-state transitions (frozen)

| From                              | Allowed next states                                                                                                                                  |
|-----------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| `ready_for_user_confirmation`     | `user_confirmed_pending_launch`, `user_cancelled`, `alternative_chosen`, `expired_unused`, `revoked_before_launch`.                                  |
| `user_confirmed_pending_launch`   | `launched_awaiting_callback`, `revoked_before_launch`.                                                                                               |
| `launched_awaiting_callback`      | `callback_validated`, `callback_rejected`, `expired_unused`.                                                                                          |
| (terminal)                        | `callback_validated`, `callback_rejected`, `user_cancelled`, `alternative_chosen`, `expired_unused`, `revoked_before_launch`.                        |

Silent transitions are forbidden; every transition fires a typed
`browser_handoff_sheet_audit_event_record` on the ADR-0010
`provider_handoff` stream.

### 2.3 Local-or-cached alternative vocabulary (frozen)

| `alternative_class`                               | What it offers                                                                                          |
|---------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| `view_cached_read_only_shadow`                    | Render the cached `cached_read_only_shadow` relation in-product (freshness tier rendered explicitly).    |
| `inspect_only_imported_snapshot`                  | Open the `imported_provider_snapshot_record` in inspect-only mode.                                        |
| `local_draft_continue`                            | Continue authoring the local draft (the publish path queues for later).                                  |
| `defer_to_publish_later_queue`                    | Admit the action into the publish-later queue with a typed prerequisite.                                 |
| `download_local_export`                           | Export a typed support / evidence packet locally instead of leaving Aureline scope.                      |
| `copy_handoff_summary_for_offline_use`            | Copy the typed sheet summary (provider class, reason code, return anchor, expected authority, etc.).     |
| `request_admin_assisted_handoff`                  | Hand off the typed sheet to an admin queue so a privileged actor can complete the handoff out-of-band.    |
| `no_alternative_available`                        | Repair-only; MUST cite a `repair_hook_ref` so the absence of an alternative is itself routed.             |

### 2.4 Rules (frozen)

1. A `browser_handoff_sheet_record` whose
   `originating_browser_handoff_packet_ref` is empty is forbidden.
   The sheet always points at a typed packet.
2. `destination_class = external_generic_web` MUST pin
   `reason_code = external_docs_or_runbook` and MUST surface a typed
   local-or-cached alternative.
3. Every reason code except `provider_consent_flow` is mutation-class
   and MUST cite an `approval_ticket_ref`.
4. Every consequence class except `none_disclosed` MUST cite a
   `privacy_or_data_loss_consequence_summary`.
5. `irreversible_external_publish` consequence MAY NOT pair with
   `replay_posture = read_only_resumable`; an irreversible publish
   is single-use or bounded-reuse only.
6. `expected_authority_on_destination = unknown_actor_class` is
   repair-only and MUST NOT advance to `user_confirmed_pending_launch`;
   the sheet stays at `ready_for_user_confirmation` (offering the
   alternative) or transitions to `revoked_before_launch` /
   `callback_rejected`.
7. `local_or_cached_alternative_refs` MUST contain at least one entry.
   `no_alternative_available` is repair-only and MUST cite a
   `repair_hook_ref`. Every other alternative class MUST cite a
   `linked_record_ref` so the user has a concrete target to land on.
8. Sheets are typed audit payloads on the ADR-0010 `provider_handoff`
   stream; the sheet records carry the sheet-level intent the packet
   records do not (alternative chosen, user cancelled before launch,
   sheet expired).

## 3. Notification, activity-center, and exported-evidence reuse

A provider-linked header (and its mediating sheet) is rendered far
beyond the originating panel: notifications, activity-center rows,
support exports, audit events, companion provider headers, CLI status
lines, AI context captures, mutation-journal entries, review-overlay
labels, and queue-review rows all need to read the same provider class,
canonical host, scope, freshness tier, handoff reason (where present),
and acting actor class.

The `handoff_reason_reuse_record` is the projection. Every reuse
surface MUST emit one record per reuse so the rendered information is
inspectable later.

### 3.1 Frozen reuse-surface vocabulary

`desktop_provider_header`, `desktop_notification`, `activity_center_row`,
`support_export_summary`, `audit_event_payload`,
`companion_provider_header`, `cli_status_line`, `cli_command_output`,
`ai_context_capture_label`, `mutation_journal_entry`,
`review_overlay_label`, `queue_review_row`.

A surface that cannot be placed in this list MUST fall through to
`support_export_summary` or `audit_event_payload` rather than invent a
surface-local label.

### 3.2 Rules (frozen)

1. Every reuse record MUST cite an `originating_header_ref`. Reuse
   records that originate from a sheet (not a header) MUST also cite
   `originating_browser_handoff_sheet_ref`.
2. Every reuse record MUST quote `provider_class`, `canonical_host`,
   `freshness_tier`, `scope_summary`, `actor_class`, and (where
   present) `reason_code`. Flattening any of these into a plain URL or
   a generic "External" label is forbidden.
3. Reuse records carry opaque refs and structured summaries only; raw
   URLs, raw tokens, raw callback bodies, raw delegated-token bodies,
   and raw provider payloads never enter the reuse projection.
4. A reuse record whose `acting_identity_badge_ref` is empty MUST
   carry `actor_class = unknown_actor_class` and MUST be rendered
   with the `unknown_actor_repair_label` from the
   [`connected-account registry contract`](./connected_account_registry_contract.md).
5. Support exports MAY include `provider_class`, `canonical_host`,
   `tenant_or_org_scope`, `freshness_tier`, `scope_summary`,
   `reason_code`, `actor_class`, the reviewable summaries, and the
   sheet's `sheet_state_class`. They MUST NOT include raw URLs, raw
   tokens, raw callback bodies, raw delegated-token bodies, or raw
   provider payloads.

## 4. Degradation rules

A header degrades visibly rather than disappearing or silently keeping
its claim of authority. The `header_degradation_event_record` is the
typed transition.

### 4.1 Cause ↔ retained / removed action posture

| `degradation_class`                              | Retained actions (≥ 1 required)                                                                                                                          | Typical removed actions                                                          | Repair hook required?                                              |
|--------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------|--------------------------------------------------------------------|
| `freshness_drift_to_bounded_stale`               | `inspect_metadata`, `view_cached_snapshot`, `copy_link_header_summary`, `copy_object_identity_ref`, `export_provider_link_summary`.                       | (typically none — the header still publishes after a confirm-refresh).            | Optional.                                                          |
| `freshness_drift_to_unbounded_stale`             | `inspect_metadata`, `view_cached_snapshot`, `copy_link_header_summary`, `export_support_evidence`.                                                        | `publish_now_blocked`, `deferred_publish_blocked`, `open_in_provider_blocked`.    | Required (refresh / re-fetch hook).                                |
| `actor_revoked` / `actor_suspended`              | `inspect_metadata`, `view_cached_snapshot`, `copy_link_header_summary`, `copy_handoff_reason_summary`, `export_provider_link_summary`.                    | `publish_now_blocked`, `deferred_publish_blocked`, `open_in_provider_blocked`, `actor_acting_blocked`. | Required (account reselection / admin review).                     |
| `provider_offline` / `provider_unreachable`      | `inspect_metadata`, `view_cached_snapshot`, `copy_link_header_summary`, `export_support_evidence`.                                                        | `publish_now_blocked`, `open_in_provider_blocked`.                                | Required for `provider_unreachable`; optional for transient `provider_offline`. |
| `mirror_derived_disclosure`                      | `inspect_metadata`, `view_cached_snapshot`, `copy_link_header_summary`, `copy_object_identity_ref`, `export_provider_link_summary`.                       | (none — mirror posture is disclosed; publish path runs through the canonical host). | Optional.                                                          |
| `policy_blocked`                                 | `inspect_metadata`, `view_cached_snapshot`, `copy_link_header_summary`, `export_support_evidence`.                                                        | `publish_now_blocked`, `deferred_publish_blocked`, `open_in_provider_blocked`, `draft_authoring_blocked` (if policy forbids local authoring too). | Required (policy review / admin review).                            |
| `host_mismatch_detected` / `tenant_switch_detected` | `inspect_metadata`, `view_cached_snapshot`, `copy_link_header_summary`, `copy_handoff_reason_summary`.                                                  | `publish_now_blocked`, `deferred_publish_blocked`, `open_in_provider_blocked`, `actor_acting_blocked`. | Required (account reselection).                                    |
| `approval_ticket_revoked`                        | `inspect_metadata`, `view_cached_snapshot`, `copy_link_header_summary`, `export_support_evidence`.                                                        | `publish_now_blocked`, `deferred_publish_blocked`, `open_in_provider_blocked`.    | Required (re-approval).                                            |

### 4.2 Rules (frozen)

1. The `retained_actions` array MUST contain at least one entry.
   Inspect / copy / export remain available where safe; a header that
   degrades to *zero* actions is forbidden.
2. The `removed_actions` array MUST be explicit. Surfaces MUST quote
   the removed actions so reviewers can tell the header is degraded
   rather than missing data; silent removal is forbidden.
3. Severe degradations
   (`actor_revoked`, `actor_suspended`,
   `host_mismatch_detected`, `tenant_switch_detected`,
   `approval_ticket_revoked`, `policy_blocked`,
   `freshness_drift_to_unbounded_stale`, `provider_unreachable`)
   MUST cite a `repair_hook_ref` so the surface routes to a concrete
   reopen path.
4. `mirror_derived_disclosure` events MUST be rendered alongside
   `local_or_provider_source = mirror_derived` on the originating
   header. The header's `header_summary` MUST disclose the mirror
   posture; flattening a mirror-served object into a "live"
   authoritative claim is forbidden.
5. Copy / export retained-actions carry opaque refs and structured
   summaries only; raw URLs, raw tokens, raw callback bodies, and raw
   provider bodies never appear.

## 5. Redaction posture (frozen)

Every header, sheet, alternative, reuse record, and degradation event
declares a `redaction_class` from the ADR-0007 / ADR-0010 set
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`). Raw URLs, raw
tokens, raw callback bodies, raw delegated-token bodies, raw policy-
injector material, and raw provider payloads MUST NOT cross this
boundary on any surface.

Support exports MAY name `provider_class`, `canonical_host`,
`tenant_or_org_scope`, `object_type`, `local_or_provider_source`,
`freshness_tier`, `edit_mode`, `actor_class`, `destination_class`,
`reason_code`, `privacy_or_data_loss_consequence_class`,
`alternative_class`, `degradation_class`, `retained_actions`,
`removed_actions`, `sheet_state_class`, and the human-legible summaries
projected by the records. They MUST NOT name raw URLs, raw tokens, raw
callback bodies, raw delegated-token bodies, raw policy-injector
material, or raw provider payloads.

Narrowing is permitted: admin policy MAY raise the `redaction_class`
to `operator_only_restricted`, `internal_support_restricted`, or
`signing_evidence_only`. Widening beyond the frozen rules is forbidden.

## 6. Audit-event reuse

Every observable header / sheet / alternative event fires on the
ADR-0010 `provider_handoff` audit stream. Two event families are in
play:

1. The packet-level event ids already exported by
   [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
   — `browser_handoff_packet_issued`, `browser_handoff_launched`,
   `browser_handoff_callback_validated`,
   `browser_handoff_callback_rejected`, `browser_handoff_revoked`.
2. The sheet-level event ids exported by
   [`/schemas/providers/browser_handoff_sheet.schema.json`](../../schemas/providers/browser_handoff_sheet.schema.json)
   — `browser_handoff_sheet_minted`,
   `browser_handoff_sheet_user_confirmed`,
   `browser_handoff_sheet_launched`,
   `browser_handoff_sheet_callback_validated`,
   `browser_handoff_sheet_callback_rejected`,
   `browser_handoff_sheet_user_cancelled`,
   `browser_handoff_sheet_alternative_chosen`,
   `browser_handoff_sheet_expired`,
   `browser_handoff_sheet_revoked`.

The sheet-level events carry the user-facing intent the packet records
do not (alternative chosen, user cancelled before launch, sheet
expired). No new audit-stream id is introduced; the records are the
*payload* the existing `provider_handoff` stream references.

## 7. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                                             | Where enforced                                                                                                                                                                                                                                                                                                                          |
|------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Generic `Open in browser` behavior is disallowed when the system can name the destination class, reason, and return path more precisely.                          | Section 2 (`browser_handoff_sheet_record` is required for every browser launch); section 1.4 rule 5 (`browser_only` edit mode MUST cite a `browser_handoff_packet_ref`); schema enforcement on `browser_handoff_sheet_record` requires `destination_class`, `reason_code`, `return_anchor`, `expected_authority_on_destination`, and at least one alternative. |
| Provider-linked object headers keep local draft state distinct from imported provider state and from browser-only actions.                                        | Section 1 (header record); section 1.2 (edit-mode ↔ mutation-mode binding); section 1.3 (source-class ↔ supporting-record binding); schema enforcement on `edit_mode` ↔ `current_mutation_mode` and on `local_or_provider_source` ↔ `import_session_ref`.                                                                                |
| Copy/export and support flows preserve handoff reason codes and provider identity instead of flattening them into plain URLs.                                    | Section 3 (notification, activity-center, exported-evidence reuse rules); section 3.2 rule 2 (every reuse record quotes provider class, canonical host, freshness tier, scope summary, actor class, and reason code); section 5 (redaction posture).                                                                                       |
| Inspect/copy/export remain available where safe instead of disappearing for stale, revoked, offline, mirror-derived, or policy-blocked links.                     | Section 4 (degradation rules); schema enforcement on `header_degradation_event_record.retained_actions` (≥ 1 required); section 1.4 rule 7 (the `available_actions` block MAY shrink under degradation but MAY NOT empty all three lists at once).                                                                                          |

## 8. Schema-of-record posture (frozen)

Rust types in the eventual provider-mode crate are the source of
truth. The JSON Schema exports at
`schemas/providers/provider_link_header.schema.json` and
`schemas/providers/browser_handoff_sheet.schema.json` are the
cross-tool boundary every non-owning surface reads. The owning
`connected_provider_record` continues to be exported by
`schemas/integration/browser_handoff_packet.schema.json` (ADR-0010);
this contract does not redefine that record.

Adding a new `provider_class`, `object_type_class`,
`local_or_provider_source_class`, `freshness_tier_class`,
`edit_mode_class`, `degradation_class`, `retained_action_class`,
`removed_action_class`, `reuse_surface_class`,
`privacy_or_data_loss_consequence_class`, `alternative_class`,
`sheet_state_class`, or `sheet_audit_event_id` is additive-minor and
requires the relevant `*_schema_version` bump
(`provider_link_header_schema_version` or
`browser_handoff_sheet_schema_version`); repurposing an existing value
is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, and ADR 0010.
