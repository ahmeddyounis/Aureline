# Provider sync-health view, replay-blast-radius summary, and degraded-import vocabulary contract

This document freezes how Aureline reports the sync health of a
connected provider — without ever falling back to a generic
`sync failed` label. It binds together four things the product must
not invent twice:

1. the **provider sync-health view record** — the typed snapshot of
   one provider's current sync mode, last successful and last failed
   provider-event references, cursor / reset state, and current
   blast-radius counts by object class for one (connected provider,
   object scope) pair;
2. the **replay-blast-radius summary** — the typed correlation
   block that links replayed, delayed, denied, and duplicate
   provider events to the impacted local objects, imported
   overlays, stale provider-link headers, and provider-linked
   notifications they touched;
3. the **degraded-import vocabulary** — the closed taxonomy
   (`partial`, `backfilled`, `mirror_derived`, `stale_permission`,
   `host_mismatch`) that imported provider state MUST be labelled
   with whenever it is not current canonical truth, so a partial,
   backfilled, mirror-derived, stale-permission, or host-mismatch
   import never masquerades as live state; and
4. the **escalation path** that pins toast / banner copy, detail-
   view copy, CLI inspection output, support-export packet copy,
   and audit-packet copy to the same typed sync-mode and degraded-
   import vocabulary so the language a user sees in a notification
   matches the language an auditor reads in a support packet.

The machine-readable schema lives at:

- [`/schemas/providers/provider_sync_health_view.schema.json`](../../schemas/providers/provider_sync_health_view.schema.json)
  — `provider_sync_health_view_record`.

Worked fixtures live at
[`/fixtures/providers/provider_sync_health_cases/`](../../fixtures/providers/provider_sync_health_cases/).

This contract **composes with and does not replace**:

- the
  [`provider event ingestion contract`](../integrations/provider_event_ingestion_contract.md)
  and its
  [`provider_event.schema.json`](../../schemas/integrations/provider_event.schema.json),
  [`import_session.schema.json`](../../schemas/integrations/import_session.schema.json),
  and
  [`webhook_replay_record.schema.json`](../../schemas/integrations/webhook_replay_record.schema.json)
  (event envelope, import session, replay ledger, blast-radius
  block);
- the
  [`provider-mode contract`](./provider_mode_contract.md) and its
  [`publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json)
  (mutation modes, publish-later queue, account mapping, provider-
  object relation);
- the
  [`deferred-publish queue contract`](./deferred_publish_queue_contract.md)
  (queue-state set, stale-target risk class, retry policy);
- the
  [`provider conflict review contract`](./provider_conflict_review_contract.md)
  (conflict-class, drift-source, reconcile-action vocabulary);
- the
  [`connected-account registry contract`](./connected_account_registry_contract.md)
  (actor classes, acting-identity badges, account-invalidation
  events, effective-scope resolution);
- the
  [`provider-linked object header and browser-handoff sheet contract`](./provider_link_header_and_handoff_contract.md)
  (header, sheet, return anchors, header-degradation events);
- the ADR-0010 browser-handoff-packet, approval-ticket, and
  connected-provider-record contracts; the ADR-0008 settings /
  policy-bundle resolver; the ADR-0009 execution-context model;
  and the ADR-0001 workspace-trust posture.

Where this document disagrees with those sources, those sources win
and this document plus the schema are updated in the same change.

This document does not ship a live sync watcher, a webhook listener,
an import worker, or a provider-specific adapter. It freezes the
record shape those implementations will read and write so a sync-
health view reaches every protected surface — toast, banner, detail
view, CLI inspection, support export, audit packet — with the same
typed mode, the same typed degraded-import classes, the same typed
blast-radius counts, and the same typed correlation refs every
time.

## Why freeze this now

Every provider-linked surface — code-host headers, issue-tracker
overlays, CI status rows, docs / portal cached snapshots, artifact
registry pages, release-publisher dashboards, identity-provider
mapping rows, AI-provider chat threads, managed-admin consoles —
has to answer the same six questions every time provider sync is
not perfectly current:

1. *Which provider is this view for, what object scope does it
   cover, and what is its current sync mode (`live`, `delayed`,
   `partial`, `replayed`, `denied`, `offline`, or `mirror_derived`)?*
2. *When did sync last succeed, when did it last fail, and what
   typed failure class did the failure carry?*
3. *Where is the cursor — current, lagging, paused, missing-page-
   detected, page-gap-detected, reset-pending, reset-in-progress,
   reset-completed, or unknown-repair-required?*
4. *What is the current blast radius — how many objects of each
   class are impacted, replayed, delayed, denied, duplicated, or
   mirror-derived right now?*
5. *Which replayed, delayed, denied, or duplicate events are
   correlated with which impacted local objects, imported overlays,
   stale provider-link headers, and provider-linked notifications?*
6. *Where can a degraded import legitimately appear (partial,
   backfilled, mirror-derived, stale-permission, host-mismatch)
   without masquerading as live state, and what language does the
   toast, the banner, the detail view, the CLI, the support
   export, and the audit packet have to use?*

Without one frozen contract: the code-host overlay invents a
`sync failed` toast, the issues surface invents a `Last sync
failed at 09:02` banner, the CI lane invents a `Provider
unreachable` badge, the docs portal invents a `Stale` chip, the
artifact registry invents a `Replay required` notice, and a
single "sync failed" copy on one surface means something
different from a "sync failed" copy on another. Worse, every one
of those surfaces is a candidate to silently let imported state
keep masquerading as live truth because the typed degraded-import
class was never mechanically named.

This contract closes that gap with **one sync-mode vocabulary,
one degraded-import vocabulary, one cursor-state vocabulary, one
failure-class vocabulary, one blast-radius count shape, one
correlation-ref block, and one escalation path** every protected
surface and every post-incident consumer reads.

## Scope

Frozen at this revision:

- the **provider sync-health view record** — provider identity,
  object scope, current mode class, last-success and last-failure
  event refs, cursor-state class, blast-radius counts by object
  class, degraded-import entries, replay-blast-radius correlation
  refs, escalation path, support-export channels, origin
  disclosure, policy context, audit-event refs;
- the **sync-mode vocabulary** (`live`, `delayed`, `partial`,
  `replayed`, `denied`, `offline`, `mirror_derived`);
- the **degraded-import vocabulary** (`partial`, `backfilled`,
  `mirror_derived`, `stale_permission`, `host_mismatch`);
- the **cursor-state vocabulary** (`cursor_current`,
  `cursor_lagging`, `cursor_paused`, `cursor_missing_page_detected`,
  `cursor_page_gap_detected`, `cursor_reset_pending`,
  `cursor_reset_in_progress`, `cursor_reset_completed`,
  `cursor_unknown_repair_required`);
- the **failure-class vocabulary** (`network_unreachable`,
  `credential_expired_or_revoked`, `host_mismatch`, `tenant_switch`,
  `signature_denied`, `policy_blocked`, `rate_limited`,
  `cursor_reset_required`, `missing_page_detected`,
  `stale_permission`, `replay_held`, `partial_failure`,
  `provider_unreachable`, `unknown_failure_repair_required`);
- the **blast-radius count shape** — per-`object_class` row of
  `impacted_count`, `replayed_count`, `delayed_count`,
  `denied_count`, `duplicate_count`, `mirror_derived_count`;
- the **correlation-ref block** — typed `replayed_event_refs`,
  `delayed_event_refs`, `denied_event_refs`,
  `duplicate_event_refs`, `impacted_local_object_refs`,
  `imported_overlay_refs`, `stale_header_refs`,
  `provider_linked_notification_refs`;
- the **escalation path** — typed `escalation_surface_class`
  members `toast_or_banner`, `detail_view`, `cli_inspection`,
  `support_export`, `audit_packet`, all five required so the
  same language travels every consumer;
- the **support-export channel vocabulary** (`support_bundle`,
  `audit_packet`, `migration_note`, `admin_assisted_handoff_packet`,
  `object_handoff_packet`) with mandatory schema-enforced
  redaction-of-raw-credentials and redaction-of-tenant-private-
  payloads markers;
- the **redaction posture** that keeps raw URLs, raw tokens, raw
  callback bodies, raw delegated-token bodies, raw provider
  payloads, and raw imported-snapshot bodies off this boundary on
  every surface;
- the **schema-enforced floor on copy** — every record carries
  `forbids_generic_sync_failed_copy = true`; surfaces MAY NOT
  render a generic `sync failed` label whenever the record can
  name a narrower mode or degraded-import class.

## Out of scope

- Live network listeners, webhook delivery probes, polling jobs,
  and provider-specific import code. The contract names the typed
  view those services will populate; the services themselves
  land with each provider adapter.
- The replay drainer, manual operator replay UI, and live
  blast-radius computation. The contract names the typed counts
  and correlation refs those services will read; the live
  computation lands with each provider adapter.
- Provider-mode mutation flow, conflict reconciliation, queue
  drain. Those are owned by the provider-mode, conflict-review,
  and deferred-publish contracts; this view cites them through
  opaque refs without redefining them.
- Toast, banner, detail-view, CLI, and support-export UI. Those
  surfaces read the typed records the contract freezes and render
  them through their own design-system contracts.

## 1. Provider sync-health view record

Every connected provider whose object scope is or has recently been
out of perfect sync MUST publish a typed
`provider_sync_health_view_record`. The record is the contract: it
names provider identity, object scope, current mode, last success
and last failure refs, cursor state, blast-radius counts,
degraded-import entries, replay-blast-radius correlation refs, and
the typed escalation path together. A view record that surfaces
only a subset of these fields is forbidden; surfaces MUST route the
view through the record rather than mint a synthetic shape.

### 1.1 Frozen vocabularies

| Field                              | Vocabulary                                                                                                                                                                                                                                                                                                                                                                                  |
|------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `provider_class`                   | `review_or_code_host`, `issue_or_planning_tracker`, `ci_or_check_provider`, `docs_or_portal_provider`, `identity_or_enterprise_provider`, `callback_or_event_provider`, `ai_provider`, `package_registry_provider`, `release_publisher_provider`, `managed_admin_provider`.                                                                                                                  |
| `object_class`                     | `pull_request`, `issue_or_work_item`, `check_run`, `release_artifact`, `docs_page`, `audit_entry`, `admin_surface`, `consent_flow`, `package_version`, `registry_entry`, `principal_subject`, `install_target`, `tenant_or_org`, `other`.                                                                                                                                                    |
| `current_mode_class`               | `live`, `delayed`, `partial`, `replayed`, `denied`, `offline`, `mirror_derived`.                                                                                                                                                                                                                                                                                                            |
| `failure_class`                    | `network_unreachable`, `credential_expired_or_revoked`, `host_mismatch`, `tenant_switch`, `signature_denied`, `policy_blocked`, `rate_limited`, `cursor_reset_required`, `missing_page_detected`, `stale_permission`, `replay_held`, `partial_failure`, `provider_unreachable`, `unknown_failure_repair_required`.                                                                            |
| `cursor_state_class`               | `cursor_current`, `cursor_lagging`, `cursor_paused`, `cursor_missing_page_detected`, `cursor_page_gap_detected`, `cursor_reset_pending`, `cursor_reset_in_progress`, `cursor_reset_completed`, `cursor_unknown_repair_required`.                                                                                                                                                              |
| `degraded_import_class`            | `partial`, `backfilled`, `mirror_derived`, `stale_permission`, `host_mismatch`.                                                                                                                                                                                                                                                                                                              |
| `escalation_surface_class`         | `toast_or_banner`, `detail_view`, `cli_inspection`, `support_export`, `audit_packet`.                                                                                                                                                                                                                                                                                                       |
| `support_export_channel_class`     | `support_bundle`, `audit_packet`, `migration_note`, `admin_assisted_handoff_packet`, `object_handoff_packet`.                                                                                                                                                                                                                                                                               |

### 1.2 Rules (frozen)

1. A sync-health view record without a
   `connected_provider_record_id` is forbidden.
2. A sync-health view record MUST carry a typed
   `current_mode_class`. A generic `sync failed`,
   `last sync failed at`, `out of date`, or `unsynced` label MUST
   NOT appear on any surface that reads this record. Schema
   enforcement: every record sets
   `forbids_generic_sync_failed_copy = true` (`const true`).
3. A sync-health view record MUST cite at least one
   `object_scope` entry. An empty `object_scope` is forbidden;
   the record names the (provider, object scope) pair the view
   reports on.
4. A sync-health view record MUST carry a `last_success_event`
   block with the most recent verified-and-applied provider event
   for the scope, OR an explicit `no_prior_success_event` marker
   when no successful sync has ever occurred.
5. A non-`live` `current_mode_class` MUST cite a
   `last_failure_event` block whose `failure_class` is one of the
   frozen failure-class members. A `live` mode MUST NOT cite
   `last_failure_event`.
6. A sync-health view record MUST cite a typed `cursor_state`
   block with a `cursor_state_class` and a reviewable
   `state_summary`.
7. A sync-health view record MUST cite at least one
   `blast_radius_counts` row. Every row carries non-negative
   integer counts for `impacted_count`, `replayed_count`,
   `delayed_count`, `denied_count`, `duplicate_count`, and
   `mirror_derived_count`. Schema enforcement: counts are
   `integer >= 0`.
8. A sync-health view record MUST list every
   `escalation_surface_class` member exactly once in the
   `escalation_path` array (`toast_or_banner`, `detail_view`,
   `cli_inspection`, `support_export`, `audit_packet`); each entry
   carries a typed `surface_summary` reviewable sentence so the
   same language travels every consumer.
9. A sync-health view record MUST cite at least one
   `support_export_channel` entry; every entry sets
   `redacts_raw_credentials = true` and
   `redacts_tenant_private_payloads = true` (both
   schema-enforced `const true`).
10. The `current_mode_class` ↔ `degraded_import_entries` binding is
    mechanical:
    - `live` → `degraded_import_entries` MUST be empty;
    - `delayed`, `replayed`, `offline` → `degraded_import_entries`
      MAY be empty or carry typed entries;
    - `partial` → `degraded_import_entries` MUST contain at least
      one entry whose `degraded_import_class` is `partial` or
      `backfilled`;
    - `denied` with `failure_class = host_mismatch` or
      `tenant_switch` → `degraded_import_entries` MUST contain at
      least one entry whose `degraded_import_class` is
      `host_mismatch` so previously-imported state is labelled
      rather than rendered as live;
    - `denied` with `failure_class = stale_permission` →
      `degraded_import_entries` MUST contain at least one entry
      whose `degraded_import_class` is `stale_permission`;
    - `denied` with any other failure class (`signature_denied`,
      `policy_blocked`) → `degraded_import_entries` MAY be empty
      because the delivery was rejected before any state was
      imported;
    - `mirror_derived` → `degraded_import_entries` MUST contain at
      least one entry whose `degraded_import_class` is
      `mirror_derived`.

## 2. Sync modes

The seven frozen sync modes name the *current* state of provider
sync for the (provider, object scope) pair. Surfaces render the
mode verbatim; collapsing two modes into a single "sync failed"
label is forbidden.

| `current_mode_class` | Meaning                                                                                                                                                                                  |
|----------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `live`               | Sync is current. The cursor is `cursor_current`; the most recent verified delivery applied; `degraded_import_entries` is empty.                                                          |
| `delayed`            | Sync is provisionally behind: at least one delivery has been held, deferred, or backed off, but no replay has yet occurred. The view names the typed `failure_class` and the held event refs. |
| `partial`            | Some objects materialised; some did not. `degraded_import_entries` includes `partial` (and optionally `backfilled`); blast-radius counts disclose how many objects per class are partial. |
| `replayed`           | A replay (duplicate, out-of-order, missing-page, manual operator, cursor reset, or provider redelivery) has rewritten state. The view names the replay event refs and the typed cursor state. |
| `denied`             | Provider denied at least one delivery (signature failed, host mismatched, tenant switched, scope stale, policy blocked). `degraded_import_entries` includes `host_mismatch` or `stale_permission`. |
| `offline`            | Provider is unreachable or the connected account is suspended / revoked / expired. The view names the typed `failure_class` and the last successful event ref.                            |
| `mirror_derived`     | Rendered state is mirror-derived (self-hosted or organisation mirror). `degraded_import_entries` includes `mirror_derived`; the canonical host is named separately.                         |

A view in `live` mode MUST disclose `cursor_current` and an empty
`degraded_import_entries` array. A view in any non-`live` mode MUST
cite at least one `last_failure_event` (or, for `replayed` mode, at
least one entry in `correlation_refs.replayed_event_refs`).

## 3. Cursor / reset state

The `cursor_state` block discloses where the cursor is right now —
not just whether sync "succeeded". Surfaces MUST quote the typed
cursor-state class verbatim; a generic "behind" or "stuck" label
is forbidden.

| `cursor_state_class`               | Meaning                                                                                                                          |
|------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| `cursor_current`                   | Cursor is at the provider's latest known position.                                                                               |
| `cursor_lagging`                   | Cursor is bounded-stale; catch-up is in progress or queued.                                                                      |
| `cursor_paused`                    | Cursor is intentionally paused (admin pause, policy pause, rate-limit hold).                                                      |
| `cursor_missing_page_detected`     | A provider page the cursor expected was not retrievable.                                                                         |
| `cursor_page_gap_detected`         | A sequence gap was observed between two pages.                                                                                   |
| `cursor_reset_pending`             | A cursor reset has been requested but not yet started.                                                                           |
| `cursor_reset_in_progress`         | Cursor reset is rewinding through the provider.                                                                                  |
| `cursor_reset_completed`           | Cursor reset has finished; subsequent imports are labelled `replayed_import` until a fresh live verified delivery proves current. |
| `cursor_unknown_repair_required`   | Cursor state is unknown and requires operator repair.                                                                            |

## 4. Blast-radius counts

The `blast_radius_counts` array is the typed object-class roll-up
the view publishes so operators can judge the blast radius of a
provider issue without reading raw event logs.

Every row carries:

| Field                       | Meaning                                                                                       |
|-----------------------------|-----------------------------------------------------------------------------------------------|
| `object_class`              | The provider object class the row reports counts for.                                          |
| `impacted_count`            | Total number of local objects of this class touched by the current degraded sync window.       |
| `replayed_count`            | Number of those objects whose state was last materialised through a replay.                    |
| `delayed_count`             | Number of those objects whose latest delivery is held or deferred.                              |
| `denied_count`              | Number of those objects whose latest delivery was denied.                                       |
| `duplicate_count`           | Number of those objects whose latest delivery was suppressed as a duplicate.                    |
| `mirror_derived_count`      | Number of those objects whose current rendering is mirror-derived (not from the canonical host). |
| `count_summary`             | Reviewable sentence summarising the row for toast / banner / detail-view / CLI / support-export / audit-packet copy. |

Counts are `integer >= 0`. A row whose counts are all zero is
allowed (it explicitly says "no impacted objects of this class");
a row with at least one non-zero count is required when the
`current_mode_class` is non-`live`.

A view MUST cite at least one `blast_radius_counts` row even when
`current_mode_class` is `live` — the explicit zero-count row is
the typed signal that the live mode is reporting on the right
object scope.

## 5. Correlation refs (replay-blast-radius summary)

Every sync-health view carries a `correlation_refs` block that
links replayed, delayed, denied, and duplicate events to the
impacted local objects, imported overlays, stale provider-link
headers, and provider-linked notifications they touched. Surfaces
read these refs to pivot from a banner into a detail view, from a
CLI row into a support export, and from a support export into the
audit packet.

The block's typed arrays are:

- `replayed_event_refs` — opaque refs to
  [`webhook_replay_record`](../../schemas/integrations/webhook_replay_record.schema.json)
  rows whose disposition rewrote, replayed, or rewound state for
  the scope;
- `delayed_event_refs` — opaque refs to
  [`provider_event_record`](../../schemas/integrations/provider_event.schema.json)
  rows whose envelope was held, deferred, or rate-limited;
- `denied_event_refs` — opaque refs to
  `provider_event_record` rows whose verification or policy denied
  the delivery;
- `duplicate_event_refs` — opaque refs to
  `provider_event_record` rows suppressed by the replay ledger as
  duplicates;
- `impacted_local_object_refs` — opaque refs to local provider-
  linked records (`provider_object_relation_record`,
  `publish_later_queue_item_record`,
  `deferred_publish_queue_item_record`, conflict-review records)
  the degraded sync window touched;
- `imported_overlay_refs` — opaque refs to
  [`import_session_record`](../../schemas/integrations/import_session.schema.json)
  rows or imported-snapshot rows the view's degraded entries point
  at;
- `stale_header_refs` — opaque refs to
  [`provider_link_header`](../../schemas/providers/provider_link_header.schema.json)
  rows whose `local_or_provider_source` is now mirror-derived,
  imported-snapshot-only, or whose freshness has degraded because
  of this sync window;
- `provider_linked_notification_refs` — opaque refs to the
  notifications, toasts, banners, or activity-center rows the view
  is the source of truth for.

Every array is optional, but a non-`live` view MUST populate at
least one of `replayed_event_refs`, `delayed_event_refs`,
`denied_event_refs`, or `duplicate_event_refs` so the replay-
blast-radius summary is observable. An empty correlation block on
a non-`live` view is forbidden.

## 6. Degraded-import entries

The `degraded_import_entries` array is the closed taxonomy that
labels imported provider state as something narrower than live.
Imported state MUST NOT masquerade as current canonical truth;
every degraded import is rendered through one typed
`degraded_import_class`.

| `degraded_import_class` | When it applies                                                                                                                     |
|-------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| `partial`               | Some pages or objects in the declared scope did not materialise. Pairs with `failure_class = partial_failure` or `missing_page_detected`. |
| `backfilled`            | Imported state came through a backfill or rewind session, not a live verified delivery. Pairs with `replayed` or `delayed` mode.    |
| `mirror_derived`        | Imported state came through a self-hosted or organisation mirror. Pairs with `mirror_derived` mode and the
[`provider_link_header`](./provider_link_header_and_handoff_contract.md) `local_or_provider_source = mirror_derived` cue. |
| `stale_permission`      | The actor's effective scope no longer covers the imported object class. Pairs with `failure_class = stale_permission`.                |
| `host_mismatch`         | Imported state was last produced under a different canonical host or tenant scope. Pairs with `failure_class = host_mismatch` or `tenant_switch`. |

Every entry carries:

- `degraded_import_class` — one of the five frozen members;
- `entry_summary` — reviewable sentence safe to render on
  toast, banner, detail-view, CLI, support-export, and audit-
  packet surfaces;
- `scope_ref` — opaque ref to the import session, mirror session,
  or imported-overlay row the entry applies to;
- `impacted_count` — non-negative integer count of objects the
  entry covers.

A view MAY repeat a class with different `scope_ref` values when
multiple sessions exhibit the same degraded class. Aureline MAY NOT
collapse `partial` and `backfilled` into a generic "incomplete"
label, MAY NOT collapse `stale_permission` and `host_mismatch`
into a generic "permission error" label, and MAY NOT collapse
`mirror_derived` into a generic "cached" label.

## 7. Failure classes

Every non-`live` view cites a `last_failure_event` block whose
`failure_class` is one of the frozen members:

| `failure_class`                  | Meaning                                                                                                  |
|----------------------------------|----------------------------------------------------------------------------------------------------------|
| `network_unreachable`            | Aureline could not reach the provider host on the last attempt.                                           |
| `credential_expired_or_revoked`  | The connected account's credential is expired, revoked, or suspended. Pairs with the connected-account `health_state`.       |
| `host_mismatch`                  | Provider returned a different canonical host than the connected account names.                            |
| `tenant_switch`                  | Provider returned a different tenant / org scope than the connected account names.                        |
| `signature_denied`               | Webhook signature verification denied the delivery.                                                       |
| `policy_blocked`                 | Policy bundle denied the delivery.                                                                        |
| `rate_limited`                   | Provider rate-limit headers blocked or deferred the call.                                                 |
| `cursor_reset_required`          | Provider returned a reset token; cursor must rewind before sync can continue.                             |
| `missing_page_detected`          | A provider page the cursor expected was not retrievable.                                                  |
| `stale_permission`               | The acting identity's effective scope no longer covers the call.                                          |
| `replay_held`                    | Replay ledger held the delivery pending sequence resolution or operator review.                            |
| `partial_failure`                | Some objects materialised; some failed.                                                                   |
| `provider_unreachable`           | Provider returned 5xx or refused to respond after the configured retry budget.                            |
| `unknown_failure_repair_required` | Failure could not be classified; operator repair is required. The view MUST cite operator review refs.    |

A view in `denied` mode MUST cite a failure class from
`signature_denied`, `policy_blocked`, `host_mismatch`,
`tenant_switch`, or `stale_permission`. A view in `offline` mode
MUST cite `network_unreachable`, `credential_expired_or_revoked`,
or `provider_unreachable`. A view in `partial` mode MUST cite
`partial_failure`, `missing_page_detected`, `rate_limited`, or
`stale_permission`. Cross-binding to other classes is allowed
under additive-minor expansion of the failure-class vocabulary.

## 8. Escalation path

Every sync-health view MUST list all five
`escalation_surface_class` members exactly once in the
`escalation_path` array. The path pins the language a user sees
in a toast or banner to the language an auditor reads in a
support packet so the typed mode and degraded-import classes do
not fork across surfaces.

| `escalation_surface_class` | What it carries                                                                                                                                                                                       |
|----------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `toast_or_banner`          | Surface-side notification copy. Quotes the typed `current_mode_class` and the typed degraded-import class verbatim. A generic `sync failed` label is forbidden.                                       |
| `detail_view`              | Provider-linked detail-view copy. Quotes the typed mode, cursor state, and blast-radius counts.                                                                                                       |
| `cli_inspection`           | CLI / headless inspection copy. Renders the typed mode, cursor state, blast-radius counts, and degraded-import entries as structured fields.                                                          |
| `support_export`           | Support-export packet copy. Cites event refs, session refs, replay refs, and degraded-import scope refs through the support-export channel records.                                                   |
| `audit_packet`             | Audit-packet copy. Cites the provider-handoff audit-event refs the record carries; the audit packet is the most-redacted but still names typed mode, typed cursor state, and typed degraded classes. |

Every entry carries a `surface_summary` reviewable sentence that
quotes the typed mode and (when applicable) the typed degraded-
import class. A surface that ships a synthetic
`escalation_surface_class` member, or that drops one of the five,
is forbidden by the schema.

## 9. Export and support fields

Every sync-health view carries a `support_export_channels` array
listing the typed export channels the view may ride. Every entry
names a typed `support_export_channel_class`, a `channel_summary`
reviewable sentence, an opaque `linked_record_ref`, and two
schema-enforced boolean markers:

- `redacts_raw_credentials` — schema-enforced `const true`.
  Sync-health views MUST NOT carry raw credentials onto any
  export channel regardless of channel class.
- `redacts_tenant_private_payloads` — schema-enforced `const true`.
  Sync-health views MUST NOT carry raw tenant-private bodies onto
  any export channel; structured field summaries and opaque event
  refs only.

The schema rejects any record that sets either marker to false.
Adapters MAY raise the `redaction_class` on the record itself
(`metadata_safe_default` → `operator_only_restricted` →
`internal_support_restricted` → `signing_evidence_only`) but MAY
NOT widen the export-channel guarantees.

### 9.1 Export channels (frozen)

| `support_export_channel_class`        | When it applies                                                                                                            |
|---------------------------------------|----------------------------------------------------------------------------------------------------------------------------|
| `support_bundle`                      | The view rides a support export so support engineering can reproduce the sync state offline.                               |
| `audit_packet`                        | The view rides a typed audit packet alongside the `provider_handoff` audit-event refs the record cites.                     |
| `migration_note`                      | The view rides a typed migration note (e.g. when a tenant switch or host mismatch is being reconciled).                     |
| `admin_assisted_handoff_packet`       | The view rides the admin-assisted-handoff packet so a privileged actor can complete repair out of band.                     |
| `object_handoff_packet`               | The view rides the typed object-handoff packet so an external reviewer sees the same record set.                            |

Every sync-health view MUST cite at least one export channel; an
empty export-channel list is forbidden.

## 10. Generic-copy prohibition (frozen)

Every sync-health view record sets
`forbids_generic_sync_failed_copy = true` (`const true` in the
schema). Surfaces that read this record MAY NOT render any of the
following labels when the record can name a narrower mode or
degraded class:

- `Sync failed`
- `Last sync failed`
- `Out of date`
- `Out of sync`
- `Sync error`
- `Connection error`
- `Could not reach provider`
- `Stale` (without naming the typed degraded-import class)
- `Cached` (without naming `mirror_derived` when applicable)

Surfaces MUST quote the typed `current_mode_class`, the typed
`failure_class` (when present), and the typed
`degraded_import_class` (when present). Schema enforcement of
the `const true` marker is the mechanical floor; the contract
language pins the rendered copy.

## 11. Reusability across surfaces

The same record shape, the same vocabularies, the same
blast-radius count shape, the same correlation block, and the
same escalation path apply on every protected provider-linked
surface. A surface that mints a synthetic sync-health shape, or
that introduces a new `current_mode_class`, is forbidden; the
contract sets the floor, and additive-minor changes flow through
a `provider_sync_health_schema_version` bump.

## 12. Redaction posture (frozen)

Every sync-health view record declares a `redaction_class` from the
ADR-0007 / ADR-0010 set (`metadata_safe_default`,
`operator_only_restricted`, `internal_support_restricted`,
`signing_evidence_only`). Raw URLs, raw tokens, raw callback
bodies, raw delegated-token bodies, raw policy-injector material,
raw provider payloads, and raw imported-snapshot bodies MUST NOT
cross this boundary on any surface.

Support exports MAY name `connected_provider_record_id`,
`provider_identity`, `object_scope`, `current_mode_class`,
`last_success_event` and `last_failure_event` blocks (with their
typed event refs, captured-at timestamps, failure classes, and
reviewable summaries), `cursor_state`, `blast_radius_counts`,
`degraded_import_entries` (with their typed classes, scope refs,
counts, and reviewable summaries), `correlation_refs`,
`escalation_path`, `support_export_channels`, the typed
origin disclosure, the typed policy context, the typed audit-event
refs, and the typed `redaction_class`. They MUST NOT name raw URLs,
raw tokens, raw callback bodies, raw delegated-token bodies, raw
policy-injector material, raw provider payloads, or raw imported-
snapshot bodies.

Narrowing is permitted: admin policy MAY raise the
`redaction_class` to `operator_only_restricted`,
`internal_support_restricted`, or `signing_evidence_only`. Widening
beyond the frozen rules is forbidden.

## 13. Audit-event reuse

Every sync-health view transition fires on the ADR-0010
`provider_handoff` audit stream using the frozen event ids already
exported by the publish-later, deferred-publish, conflict-review,
and event-ingestion contracts. The most relevant ids:

- `provider_event_accepted`
- `provider_event_denied`
- `provider_event_held`
- `provider_event_replayed`
- `provider_action_proposed`
- `provider_action_denied`
- `provider_action_deferred`
- `policy_epoch_rolled_invalidations`
- `import_session_started`
- `import_session_completed`
- `import_session_failed_partial`
- `webhook_replay_applied_once`
- `webhook_replay_deduped_noop`
- `webhook_replay_held_pending_sequence`

No new audit-event id is introduced by this contract. The
sync-health view record is the *payload* those frozen events
reference; the `audit_event_refs` array on each record cites the
opaque event ids the listener emitted.

## 14. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                               | Where enforced                                                                                                                                                                                                                                                                  |
|----------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Generic `sync failed` copy is disallowed whenever Aureline can name a narrower degraded or replay state.                                            | Section 1.2 rule 2; section 10; schema enforcement on `forbids_generic_sync_failed_copy = const true`; the typed `current_mode_class`, `failure_class`, and `degraded_import_class` vocabularies on every entry that surfaces would render copy from.                            |
| Impacted object counts and scope remain visible so operators can judge the blast radius of a provider issue without reading raw logs.              | Section 4 (`blast_radius_counts` per `object_class`); section 1.2 rules 3 and 7; schema enforcement on `blast_radius_counts.minItems = 1`, `object_scope.minItems = 1`, and non-negative integer counts.                                                                          |
| Replay, denial, and delayed-import states are exportable and machine-readable for support and audit use.                                            | Section 5 (`correlation_refs` typed arrays); section 9 (`support_export_channels` frozen vocabulary, `redacts_raw_credentials` and `redacts_tenant_private_payloads` const true); section 8 (escalation path enumerates `support_export` and `audit_packet` so language travels). |

## 15. Schema-of-record posture (frozen)

Rust types in the eventual provider-mode crate are the source of
truth. The JSON Schema export at
`schemas/providers/provider_sync_health_view.schema.json` is the
cross-tool boundary every non-owning surface reads. The paired
`provider_event_record`, `import_session_record`,
`webhook_replay_record`, `provider_object_relation_record`,
`provider_link_header`, and `provider_conflict_review_record`
continue to be exported by their own schemas; this contract does
not redefine those records and cites them through opaque refs
(`replayed_event_refs`, `delayed_event_refs`, `denied_event_refs`,
`duplicate_event_refs`, `impacted_local_object_refs`,
`imported_overlay_refs`, `stale_header_refs`,
`provider_linked_notification_refs`).

Adding a new `current_mode_class`, `failure_class`,
`cursor_state_class`, `degraded_import_class`,
`escalation_surface_class`, `support_export_channel_class`, or
`object_class` is additive-minor and requires a
`provider_sync_health_schema_version` bump; repurposing an
existing value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, and ADR 0010.
