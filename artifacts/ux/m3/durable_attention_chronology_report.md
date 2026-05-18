# Durable Attention Chronology Report

This report documents the shared chronology primitive and the
attention-inbox triage primitive that make durable, look-away-resilient
attention possible across the shell. It is the readable companion to:

- `crates/aureline-shell/src/activity_timeline/` — runtime, types,
  validator, and seeded conformance packet.
- `schemas/ux/activity_event_row.schema.json` — boundary contract for
  the event row, timeline group, and narrative summary card.
- `schemas/ux/attention_inbox_item.schema.json` — boundary contract
  for the inbox item and snapshot.
- `fixtures/ux/m3/activity_timeline_and_inbox/` — frozen
  cross-lane corpus used by the integration test and reviewers.

## Why one chronology primitive

Before this work landed, every lane that needed durable evidence —
activity center, AI evidence, policy change history, provider sync,
update history, reconnect flows, recovery — minted its own row shape
and its own dismissal vocabulary. Toasts and banners vanished after
focus loss, and the only surviving truth lived in the durable
activity-center row. Users could not answer "what happened while I was
focused elsewhere" without re-running work, and support exports had to
scrape rendered copy to reconstruct lineage.

The chronology primitive is the answer: one row shape, one
chronology lane discriminator, one closed verb / outcome / importance
vocabulary, and one non-truncating detail link contract. Every lane
projects into it, and every export reads the same shape.

## The shared event row

`activity_event_row_record` carries:

| Field | Purpose |
| --- | --- |
| `event_row_id` | Stable row identity. Multiple observations of the same canonical event collapse onto the same id. |
| `canonical_event_id`, `canonical_object_target_ref` | Bind the row to the same object identity the notification envelope and durable activity row use. |
| `chronology_lane` | Names the lane (activity center, AI evidence, policy changes, provider sync, update history, reconnect, recovery, approvals) so the chrome can render lane-specific layouts without breaking shared structure. |
| `source_subsystem`, `actor_kind`, `actor_or_subsystem_label` | Re-export the notification envelope's source / actor vocabulary verbatim. |
| `scope_object_kind` | Names what kind of canonical object the row is about (durable job, approval, policy decision, provider sync, update event, reconnect, recovery snapshot, ...). |
| `action_verb`, `outcome_class`, `importance_class` | Closed vocabularies for what happened, how it ended, and how loud the row should be. |
| `actionability_class` | Records the current user-facing actionability: none / open-details / reviewable / requires-user-action / requires-revalidation. |
| `summary_label`, `scope_label` | Short, privacy-safe labels — never paths, never raw payloads. |
| `monotonic_timestamp`, `minted_at`, `last_observed_at` | UTC ISO 8601 from a monotonic clock; canonical for ordering and export. |
| `detail_link` | One non-truncating link target. Generic home / external-URL fallbacks are not representable. |
| `linked_canonical_event_id_ref`, `grouped_burst_id_ref`, `supersedes_event_row_id_ref` | Cross-references that keep lineage stitched. |
| `quiet_hours_held`, `occurrence_count` | Suppression and dedupe bookkeeping. |

### Detail-link discipline

`detail_link_kind` enumerates `canonical_object_exact`,
`durable_activity_row`, `evidence_packet_row`, `review_sheet`,
`diff_view`, `placeholder_announced`,
`denied_requires_revalidation`, `audit_trail_only`,
`not_available_linkback_lost`. The validator (and the schema)
enforce:

- Consequential and safety-critical rows MUST carry a durable,
  non-truncating detail link that opens an exact target.
- `audit_trail_only` and `not_available_linkback_lost` MUST carry a
  truthful `unavailability_reason_label`.
- Generic home-screen, search, or external-URL fallbacks are not
  representable in this enum, so a lane cannot silently downgrade.

### Grouped timeline views and narrative summaries

Two structural overlays sit on top of the row corpus:

- `activity_timeline_group_record` — one grouping rule (same
  canonical object, same grouped burst, same linked canonical event,
  same lane phase, same actor within window, explicit user pin) over
  one or more rows. Groups cite member rows by id; collapsing or
  expanding a group never erases a row. The seeded test-run group
  exposes the `Preparing / Running / Failed` phase boundary set.
- `activity_narrative_summary_card_record` — one short summary card
  that cites member rows by id. A summary card cannot replace its
  cited rows; consequential and safety-critical cards must carry a
  non-truncating durable detail link.

These overlays make history-heavy workflows — multi-phase test runs,
multi-step provider syncs, multi-attempt recoveries — readable as a
single card while keeping every constituent row reachable by id.

## The attention inbox

`attention_inbox_item_record` is the triage primitive for items that
need explicit human action rather than background completion. It
carries:

- `why_shown_reason` — closed vocabulary of why this particular user
  is being asked: assigned reviewer, workspace owner, policy
  authority, originating actor, admin addressed, directly addressed,
  recovery / reconnect / provider-sync / update addressed.
- `authority_source_class` — who requested the user's attention
  (first-party shell, admin policy, AI agent, remote service,
  extension, user snooze expiry, recovery subsystem).
- `freshness_class` — fresh / recent /
  `stale_revalidation_recommended` /
  `stale_revalidation_required` / `reconstructed_from_backup`.
- A closed verb set on every item: **open**, **snooze**,
  **acknowledge**, **clear**, **mute**, **resolve**, **escalate**.
  Each verb is independently attributable in exported history so a
  reviewer can tell whether a user acknowledged the prompt,
  resolved the underlying issue, snoozed it, or muted the class.
- `suppression_note` — the quiet-hours / focus / admin modes active
  at mint, the suppression reasons that held a transient surface,
  and a `durable_history_preserved` invariant that the validator
  pins to `true`. Quiet hours never destroy durable inbox evidence.

The inbox snapshot also exposes structured counts
(`actionable_count`, `snoozed_count`,
`acknowledged_unresolved_count`, `resolved_count`,
`muted_count`) so badges, queues, history lanes, and support exports
agree on item class.

## Verb attribution and clear / mute / resolve discipline

The same canonical event can travel through several lifecycle verbs
on the chronology row: `proposed → accepted` (approvals), `started
→ progressed → failed` (jobs), `disconnected → reconnected`
(reconnect), `proposed → restored` (recovery), `narrowed` (policy
change). The inbox has its own closed verb set, separate from the
chronology verb set, so that:

- **Snooze** holds the item until a freshness boundary expires; the
  chronology row records a `snoozed` event.
- **Acknowledge** removes the badge but not the row; the chronology
  records `acknowledged`.
- **Clear** removes the inbox row from the active inbox view without
  resolving the underlying issue; the chronology row remains
  reachable.
- **Mute** marks the item's *class* as suppressed for this user;
  durable evidence is preserved and the mute is reversible.
- **Resolve** closes the underlying issue and parks the row in the
  history lane.
- **Escalate** routes the row to an admin / reviewer.

Because each verb maps to a distinct chronology row, support exports
never lose attribution: a reviewer reading the exported chronology
can tell who acknowledged what, when it was snoozed, and which event
finally resolved it.

## Notification handoff rows

Every inbox item carries `event_row_id_ref`, optionally back-linking
to a concrete chronology row. The chronology row's `detail_link`
points to a durable object (canonical object, durable activity row,
review sheet, evidence packet, diff view). The combination means:

- Activating a toast that pointed at a now-vanished transient
  surface still routes the user to the durable row.
- Reopening from an OS notification reopens the durable object, the
  durable activity row, or a truthful placeholder — never a generic
  home surface.

The chrome can render the chronology lane and the triage inbox
separately, but they share the same canonical object identity and the
same `chronology_lane`, so a row that originated in the activity
center and a triage prompt that escalates the same row stay
co-identifiable.

## Coverage and verification

The seeded packet exercises:

- All eight chronology lanes (activity center, AI evidence
  placeholder, approvals, policy changes, provider sync, update
  history, reconnect flow, recovery).
- The full importance ladder (routine, consequential,
  safety-critical) with the validator pinning the importance →
  detail-link rule.
- Quiet-hours-held flow (reconnect lane) with the durable inbox row
  preserved despite a held transient surface.
- Snoozed, acknowledged-but-unresolved, resolved, and muted inbox
  postures.
- A placeholder row in the recovery lane proving that
  expired-target rows remain visible with a truthful unavailability
  label.
- A superseded path (action verb `superseded` requires
  `supersedes_event_row_id_ref`).

`validate_activity_timeline_and_inbox_packet` rejects any of the
following: a row whose actor kind requires identity but has none, a
consequential/safety-critical row with a truncating detail link, an
inbox item whose suppression note does not preserve durable history,
or an inbox item missing the open/snooze/acknowledge/resolve verb
set. Reviewers can run the inspector binary and the fixture replay
test to see the corpus and the validator agree.

## Out of scope (intentionally)

- Enterprise analytics dashboards built on top of the chronology
  corpus.
- Non-actionable vanity feeds.
- Mobile / browser parity beyond the claimed beta surfaces.

These deliberately stay outside the chronology primitive so the
shared row vocabulary stays small enough to enforce.
