# Provider conflict review, compare/reconcile flow, and last-writer-wins prohibition contract

This document freezes how Aureline reconciles a provider-side race
or drift on an externally owned object — without ever falling back
to a hidden last-writer-wins default. It binds together four things
the product must not invent twice:

1. the **provider conflict review object** — the typed snapshot of
   one human or admin review pass that compares the local draft,
   the last imported remote snapshot, the current remote snapshot,
   and the intended publish result for a single externally owned
   provider object;
2. the **compare / reconcile flow** — the typed reconcile actions
   (compare, rebase, edit local draft, discard local draft, export
   local draft, open in provider, accept remote as authoritative,
   fork into new target, escalate for admin review) and the typed
   conditions under which each is admissible;
3. the **freshness, actor, and scope cues** that disclose whether
   the conflict came from remote edits, stale imports, changed
   permissions, or target identity drift; and
4. the **export / support surface** that lets a conflict review
   ride support bundles, audit packets, migration notes, admin-
   assisted handoff packets, and object handoff packets without
   revealing raw credentials or tenant-private payloads.

The machine-readable schema lives at:

- [`/schemas/providers/provider_conflict_review.schema.json`](../../schemas/providers/provider_conflict_review.schema.json)
  — `provider_conflict_review_record`.

Worked fixtures live at
[`/fixtures/providers/provider_conflict_cases/`](../../fixtures/providers/provider_conflict_cases/).

This contract **composes with and does not replace**:

- the
  [`provider-mode contract`](./provider_mode_contract.md) and its
  [`publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json)
  (mutation-mode set, conflict-policy class, consequence-preview,
  account-mapping, provider-object-relation);
- the
  [`deferred-publish queue contract`](./deferred_publish_queue_contract.md)
  and its
  [`deferred_publish_queue_item.schema.json`](../../schemas/providers/deferred_publish_queue_item.schema.json)
  and
  [`deferred_publish_review.schema.json`](../../schemas/providers/deferred_publish_review.schema.json)
  (queue-state set, stale-target risk class, dependency chain,
  retry policy, reauth / re-scope requirement, review action set);
- the
  [`connected-account registry contract`](./connected_account_registry_contract.md)
  (actor classes, acting-identity badges, account-invalidation
  events, effective-scope resolution);
- the
  [`provider-linked object header and browser-handoff sheet contract`](./provider_link_header_and_handoff_contract.md)
  (header, sheet, return anchors, header-degradation events);
- the ADR-0010 browser-handoff-packet, approval-ticket, and
  connected-provider-record contracts; the ADR-0008 settings /
  policy-bundle resolver; the ADR-0009 execution-context model; and
  the ADR-0001 workspace-trust posture.

Where this document disagrees with those sources, those sources win
and this document plus the schema are updated in the same change.

This document does not ship a merge engine, a provider-specific
resolver, or any provider-adapter implementation. It freezes the
record shape those implementations will read and write so a queued
provider mutation that meets a remote-side change reaches a typed
review record before it drains, on every protected surface, every
time.

## Why freeze this now

Every later lane that lets the user co-author state on an externally
owned provider object — an issue body, a pull-request title, a
review comment thread, a docs page, a release artifact, a
check-run rerun, a CI status update, a managed-admin surface, an
AI-provider chat thread — has to answer the same six questions on
every protected surface, every time it tries to publish:

1. *What did the user actually author locally, what did they think
   they were diverging from when they started, and what would the
   queued mutation actually write to the provider?*
2. *What did the provider state look like at the moment of import,
   and what does it look like now? What changed and which
   structured fields, comments, status updates, attachments, or
   permissions are involved?*
3. *Where did the conflict come from — a genuine remote edit since
   import, a stale import snapshot, a permission shift, a change in
   the actor's effective scope, or a target identity drift
   (rename, relocate, replace, delete)?*
4. *Which reconcile actions are actually admissible right now?
   Compare-only actions are always admissible; rebase is forbidden
   when the target identity has drifted or permissions changed;
   edit-local is forbidden when the draft has been deleted or
   rolled back; accept-remote is forbidden when the review is
   blocked by admin policy; fork-into-new-target is required when
   the target has been replaced.*
5. *If the user defers, escalates, or asks to export, do they get
   a typed reconcile action with a typed `admissible` flag and a
   typed `blocked_reason_summary`, or does the surface only offer
   an implicit "publish anyway" path that resolves the conflict as
   a hidden last-writer-wins?*
6. *If the conflict review is attached to a support bundle, an
   audit packet, a migration note, an admin-assisted handoff
   packet, or an object handoff packet, do raw URLs, raw tokens,
   raw provider payloads, or raw draft / remote bodies leak across
   the boundary? Or does the channel carry only structured field
   summaries and opaque snapshot refs?*

Without one frozen contract: the code-host conflict surface
invents a "Conflict — Resolve" toast, the release lane invents a
"Republish anyway?" button, the issue lane invents a "Sync from
remote" dialog, the docs portal invents a "Discard or keep" prompt,
the CI rerun queue invents a "Force rerun" override, and a single
"Resolve" action on one surface means something different from a
"Resolve" action on another. Worse, every one of those surfaces is
a candidate to silently choose last-writer-wins because the
"correct" reconcile action was never mechanically named.

This contract closes that gap with **one conflict-review
vocabulary, one reconcile-action vocabulary, one drift-source
vocabulary, one snapshot-class vocabulary, and one export-channel
vocabulary** every protected surface and every post-incident
consumer reads.

## Scope

Frozen at this revision:

- the **provider conflict review record** — surface class, target
  object identity, reviewer actor class, review-state class,
  blocked-reason class, freshness class, actor-drift class,
  scope-drift class, draft-state class, four typed snapshot
  envelopes (`local_draft_snapshot`,
  `last_imported_remote_snapshot`, `current_remote_snapshot`,
  `intended_publish_snapshot`), the typed conflict-signal table,
  the typed reconcile-action table, the typed support-export
  channels, the typed resolution disposition, origin disclosure,
  policy context, audit-event refs;
- the **conflict-class vocabulary** covering text, structured
  fields, review metadata, transitions, comments, status updates,
  attachments, permissions, and target identity drift;
- the **diff-signal vocabulary** that refines conflict classes
  with per-row signals (text diff, structured field change, review
  metadata change, transition change, comment added or removed,
  comment body changed, status update added, attachment added or
  removed, permission changed, target renamed / relocated /
  replaced / deleted);
- the **drift-source vocabulary** (remote edit since import, stale
  import snapshot, changed permissions, target identity drift,
  actor or scope change, unknown drift repair required);
- the **freshness, actor, and scope cue vocabularies** that
  disclose whether the conflict came from remote edits, stale
  imports, changed permissions, target identity drift, or actor /
  scope change;
- the **reconcile-action vocabulary** with typed admissibility
  conditions (compare with remote, compare intended publish with
  remote, rebase local on remote, edit local draft, discard local
  draft, export local draft, open in provider, accept remote as
  authoritative, fork into new target, escalate for admin review);
- the **resolution-disposition vocabulary** (pending, rebase local
  on remote, edit local then review again, discard local keep
  remote, discard local cancel, accept remote as authoritative,
  fork into new target, open in provider to resolve, escalate for
  admin review, cancel review);
- the **support-export channel vocabulary** (support bundle,
  audit packet, migration note, admin-assisted handoff packet,
  object handoff packet) with mandatory schema-enforced
  redaction-of-raw-credentials and redaction-of-tenant-private-
  payloads markers;
- the **redaction posture** that keeps raw URLs, raw tokens, raw
  callback bodies, raw delegated-token bodies, raw provider
  payloads, and raw preview / draft / remote-snapshot bodies off
  this boundary on every surface;
- the **audit-event reuse** rules that route conflict-review
  events onto the ADR-0010 `provider_handoff` stream alongside the
  existing publish-later and deferred-publish events.

## Out of scope

- Implementing merge algorithms or provider-specific resolvers.
  The contract names the typed reconcile-action set those
  implementations will route through; the algorithms themselves
  land with each provider adapter.
- Live freshness probes, live remote-snapshot diff engines, and
  live target-identity reconciliation. The contract names the
  typed snapshot envelopes those probes will populate; the probes
  themselves land with each provider adapter.
- Conflict-review UI, support-export UI, audit-packet UI, and
  migration-note UI. Those surfaces read the typed records the
  contract freezes and render them through their own design-system
  contracts.
- The eventual policy-bundle authoring surface and admin re-
  authorisation console. ADR-0008 owns the policy-resolver shape;
  this contract reuses the resolver's output through the policy
  context block.

## 1. Provider conflict review record

Every provider mutation whose target object has changed since the
local draft began MUST be reconciled through a typed
`provider_conflict_review_record` before the mutation drains. The
record is the contract: it names surface class, target object
identity, reviewer actor class, review-state, blocked-reason,
freshness, actor-drift, scope-drift, draft-state, the four typed
snapshots, the typed conflict-signal table, the typed reconcile-
action table, the typed support-export channels, and exactly one
typed resolution disposition together. A review record that
surfaces only a subset of these fields is forbidden; surfaces MUST
route the conflict to `inspect_only` or `open_in_provider` rather
than mint a synthetic review shape.

### 1.1 Frozen vocabularies

| Field                              | Vocabulary                                                                                                                                                                                                                                                                                                                                                                                  |
|------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `surface_class`                    | `code_host_surface`, `issue_or_planning_surface`, `ci_or_checks_surface`, `docs_or_portal_surface`, `artifact_registry_surface`, `release_publisher_surface`, `identity_provider_surface`, `ai_provider_surface`, `managed_admin_surface`.                                                                                                                                                   |
| `reviewer_actor_class`             | `human_account`, `installation_or_app_grant`, `delegated_user_token`, `project_scoped_grant`, `policy_injected_service_identity`, `admin_reviewer`, `unknown_actor_class`.                                                                                                                                                                                                                  |
| `review_state_class`               | `pending_user_review`, `in_review`, `blocked_by_freshness`, `blocked_by_actor_unknown`, `blocked_by_permission_loss`, `blocked_by_target_identity_drift`, `blocked_by_policy`, `resolved_with_rebase`, `resolved_with_edit_local`, `resolved_with_discard_local`, `resolved_with_accept_remote`, `resolved_with_fork_into_new_target`, `resolved_with_open_in_provider`, `cancelled`.        |
| `blocked_reason_class`             | `none`, `waiting_freshness_refresh`, `waiting_actor_resolution`, `waiting_permission_repair`, `waiting_target_identity_resolution`, `waiting_admin_policy_review`.                                                                                                                                                                                                                          |
| `freshness_class`                  | `fresh`, `bounded_stale`, `unbounded_stale`, `unknown_freshness_repair_required`.                                                                                                                                                                                                                                                                                                            |
| `actor_drift_class`                | `actor_unchanged`, `actor_subject_changed_since_draft`, `actor_class_changed_since_draft`, `actor_grant_revoked_since_draft`, `actor_token_expired_since_draft`, `actor_unknown_repair_required`.                                                                                                                                                                                            |
| `scope_drift_class`                | `scope_unchanged`, `additional_scopes_required_since_draft`, `surplus_scopes_since_draft`, `tenant_or_org_scope_realigned_since_draft`, `project_scope_realigned_since_draft`, `delegated_grant_lapsed_since_draft`.                                                                                                                                                                         |
| `drift_source_class`               | `remote_edit_since_import`, `stale_import_snapshot`, `changed_permissions`, `target_identity_drift`, `actor_or_scope_change`, `unknown_drift_repair_required`.                                                                                                                                                                                                                              |
| `conflict_class`                   | `text_conflict`, `structured_field_conflict`, `review_metadata_conflict`, `transition_conflict`, `comment_conflict`, `status_update_conflict`, `attachment_conflict`, `permission_conflict`, `target_identity_conflict`.                                                                                                                                                                    |
| `diff_signal_class`                | `text_diff`, `structured_field_change`, `review_metadata_change`, `transition_change`, `comment_added_or_removed`, `comment_body_changed`, `status_update_added`, `attachment_added_or_removed`, `permission_changed`, `target_renamed`, `target_relocated`, `target_replaced`, `target_deleted`.                                                                                            |
| `draft_state_class`                | `draft_unchanged_since_compare`, `draft_modified_since_compare`, `draft_rolled_back_since_compare`, `draft_deleted_since_compare`.                                                                                                                                                                                                                                                            |
| `reconcile_action_class`           | `compare_local_with_remote`, `compare_intended_publish_with_remote`, `rebase_local_on_remote`, `edit_local_draft`, `discard_local_draft`, `export_local_draft`, `open_in_provider`, `accept_remote_as_authoritative`, `fork_into_new_target`, `escalate_for_admin_review`.                                                                                                                  |
| `resolution_disposition_class`     | `pending`, `rebase_local_on_remote`, `edit_local_then_review_again`, `discard_local_keep_remote`, `discard_local_cancel`, `accept_remote_as_authoritative`, `fork_into_new_target`, `open_in_provider_to_resolve`, `escalate_for_admin_review`, `cancel_review`.                                                                                                                            |
| `support_export_channel_class`     | `support_bundle`, `audit_packet`, `migration_note`, `admin_assisted_handoff_packet`, `object_handoff_packet`.                                                                                                                                                                                                                                                                                |

### 1.2 Rules (frozen)

1. A conflict review record without a `connected_provider_record_id`
   is forbidden.
2. A conflict review record MUST cite all four typed snapshots
   (`local_draft_snapshot`, `last_imported_remote_snapshot`,
   `current_remote_snapshot`, `intended_publish_snapshot`); each
   snapshot envelope MUST carry the matching `snapshot_class` and
   a typed `freshness_class`. Reviewers MUST be able to
   reconstruct what the local draft would overwrite and what
   remote state changed since the draft began.
3. A conflict review record MUST contain at least one
   `conflict_signals` entry. An empty signal list is forbidden
   because a review without conflicts is not a conflict review.
4. Every blocked review state MUST cite a non-`none`
   `blocked_reason_class` and the binding is mechanical:
   `blocked_by_freshness` ↔ `waiting_freshness_refresh`,
   `blocked_by_actor_unknown` ↔ `waiting_actor_resolution`,
   `blocked_by_permission_loss` ↔ `waiting_permission_repair`,
   `blocked_by_target_identity_drift` ↔
   `waiting_target_identity_resolution`,
   `blocked_by_policy` ↔ `waiting_admin_policy_review`.
5. Every resolved review state MUST cite the matching disposition:
   `resolved_with_rebase` ↔ `rebase_local_on_remote`,
   `resolved_with_edit_local` ↔ `edit_local_then_review_again`,
   `resolved_with_discard_local` ↔ `discard_local_keep_remote` or
   `discard_local_cancel`,
   `resolved_with_accept_remote` ↔ `accept_remote_as_authoritative`,
   `resolved_with_fork_into_new_target` ↔ `fork_into_new_target`,
   `resolved_with_open_in_provider` ↔ `open_in_provider_to_resolve`,
   `cancelled` ↔ `cancel_review`. Schema enforcement keeps the
   binding mechanical; surfaces MUST NOT report a resolution that
   does not match the disposition.
6. Every non-`pending` `resolution_disposition_class` MUST quote a
   `resolution_summary` so the resolution path is observable.
7. A reviewer authoring under `unknown_actor_class` is repair-only
   and MUST disposition `escalate_for_admin_review`. An
   `actor_drift_class` of `actor_unknown_repair_required` forces
   `review_state_class = blocked_by_actor_unknown` and
   `resolution_disposition_class = escalate_for_admin_review`.
8. Aureline MAY NOT resolve a provider conflict through an
   unreviewed last-writer-wins default. Every conflict review MUST
   offer at least `compare_local_with_remote` and
   `compare_intended_publish_with_remote` as admissible read-only
   inspections, plus at least one resolving reconcile action
   (`rebase_local_on_remote`, `edit_local_draft`,
   `discard_local_draft`, `accept_remote_as_authoritative`,
   `fork_into_new_target`, `open_in_provider`, or
   `escalate_for_admin_review`) so a non-`pending` disposition is
   reachable through a typed action and never through an implicit
   fallback.

## 2. Snapshots

A conflict review reconciles four typed snapshots. Each is captured
through a `snapshot_envelope` carrying a typed `snapshot_class`, a
typed `captured_at` monotonic timestamp, a typed `freshness_class`,
a `snapshot_summary` reviewable sentence, and an opaque
`snapshot_ref` plus optional `evidence_store_ref`. Raw provider
payloads, raw draft bodies, and raw remote-snapshot bodies never
cross this boundary.

| Snapshot                          | What it carries                                                                                                                                                  |
|-----------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `local_draft_snapshot`            | What the user authored locally — the typed structured fields, comment thread, attachments, and permissions the local draft holds.                              |
| `last_imported_remote_snapshot`   | What the user thought they were diverging from when the draft started — the typed remote import the local draft was rebased on.                                |
| `current_remote_snapshot`         | What the provider currently holds — the typed remote snapshot at the moment the conflict was minted.                                                            |
| `intended_publish_snapshot`       | What the queued mutation would write — the typed merged result the deferred-publish queue (or paired publish-later queue) would commit if it drained.            |

The four snapshots are how a reviewer reconstructs (a) what the
local draft would overwrite (`local_draft_snapshot` ↔
`last_imported_remote_snapshot`) and (b) what remote state changed
since the draft began (`last_imported_remote_snapshot` ↔
`current_remote_snapshot`). The `intended_publish_snapshot` is the
merge proposal the queue would commit.

A conflict review whose four snapshot classes are not all present
is rejected by the schema; surfaces MUST NOT mint a partial review
record.

## 3. Conflict signals

Every conflict review carries an ordered table of typed
`conflict_signals` entries. Each entry names a typed
`conflict_class`, a typed `diff_signal_class`, a typed
`drift_source_class`, a `signal_summary` reviewable sentence, and
optional opaque refs to the structured field path and to the
local / remote / imported values at that path (never the raw
values). Surfaces render the signal table in
`conflict_signal_index` order so reviewers, support exports,
audit-packet consumers, and admin-reconciliation consoles see the
same signal sequence.

### 3.1 Conflict-class ↔ diff-signal binding

| `conflict_class`               | Typical `diff_signal_class` members                                                          |
|--------------------------------|---------------------------------------------------------------------------------------------|
| `text_conflict`                | `text_diff` (plus optionally `comment_body_changed`).                                       |
| `structured_field_conflict`    | `structured_field_change`.                                                                   |
| `review_metadata_conflict`     | `review_metadata_change`.                                                                    |
| `transition_conflict`          | `transition_change`.                                                                         |
| `comment_conflict`             | `comment_added_or_removed`, `comment_body_changed`.                                          |
| `status_update_conflict`       | `status_update_added`.                                                                       |
| `attachment_conflict`          | `attachment_added_or_removed`.                                                               |
| `permission_conflict`          | `permission_changed` (schema-enforced).                                                      |
| `target_identity_conflict`     | `target_renamed`, `target_relocated`, `target_replaced`, `target_deleted` (schema-enforced).|

Schema enforcement keeps the two strongest bindings mechanical:
`permission_conflict` MUST carry `permission_changed`;
`target_identity_conflict` MUST carry one of the four target-*
diff signals. Every other binding is the recommended pairing —
adapters MAY narrow further by widening the diff-signal vocabulary
through an additive-minor schema-version bump.

### 3.2 Drift source disclosure

Every conflict signal MUST carry a typed `drift_source_class`. A
review whose signal table mixes drift sources is allowed (a single
remote edit can simultaneously be a `remote_edit_since_import`
text change and a `changed_permissions` access change), but every
row MUST quote one. Aureline MAY NOT collapse `remote_edit_since_
import` and `stale_import_snapshot` into a generic "out of date"
label, and MAY NOT collapse `changed_permissions` and
`target_identity_drift` into a generic "permission error" label.

## 4. Reconcile actions and admissibility

The typed `reconcile_actions` table is the closed action set the
conflict-review surface offers. Every entry carries a typed
`reconcile_action_class`, a typed `action_summary`, a typed
`admissible` boolean, an optional `blocked_reason_summary` (required
when `admissible` is false), and an optional `linked_record_ref`
naming the typed record the action routes through.

### 4.1 Admissibility (frozen)

| `reconcile_action_class`                    | Admissible when                                                                                                                                                             |
|---------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `compare_local_with_remote`                 | Always. Read-only inspection. Schema-enforced as an always-admissible action.                                                                                                |
| `compare_intended_publish_with_remote`      | Always. Read-only inspection. Schema-enforced as an always-admissible action.                                                                                                |
| `rebase_local_on_remote`                    | No `target_identity_conflict` AND no `permission_conflict` signal in the table; `draft_state_class` is `draft_unchanged_since_compare` or `draft_modified_since_compare`.    |
| `edit_local_draft`                          | `draft_state_class` is not `draft_deleted_since_compare` or `draft_rolled_back_since_compare`.                                                                              |
| `discard_local_draft`                       | Always. Cancels the local draft; the queue item (if any) parks at the typed cancel disposition.                                                                              |
| `export_local_draft`                        | Always. Routes through the typed support / object-handoff packet so the local draft survives offline review.                                                                  |
| `open_in_provider`                          | Always. Routes through the typed browser-handoff sheet under the provider-link header contract; raw URLs never cross this boundary.                                          |
| `accept_remote_as_authoritative`            | `review_state_class` is not `blocked_by_policy`. The local draft is abandoned in favour of the current remote snapshot; downstream queue items transition to typed cancel.   |
| `fork_into_new_target`                      | At least one `target_identity_conflict` signal in the table. The local draft is forked into a new typed target object identity and a superseding queue item is minted.      |
| `escalate_for_admin_review`                 | Always. Hands the review to the admin-reconciliation queue. The schema requires this action when the reviewer is `unknown_actor_class` or the actor-drift class is repair-only. |

### 4.2 Schema-enforced floors

The schema enforces three reconcile-action floors:

1. Every conflict review MUST include `compare_local_with_remote`
   as an admissible action.
2. Every conflict review MUST include
   `compare_intended_publish_with_remote` as an admissible action.
3. Every conflict review MUST include at least one resolving
   action (`rebase_local_on_remote`, `edit_local_draft`,
   `discard_local_draft`, `accept_remote_as_authoritative`,
   `fork_into_new_target`, `open_in_provider`, or
   `escalate_for_admin_review`). The action MAY be inadmissible
   (`admissible = false`); the surface MUST disable it visibly with
   a typed `blocked_reason_summary` rather than drop it. An
   implicit retry-on-publish or last-writer-wins path is
   forbidden.

A review whose `reconcile_actions` table fails any of the three
floors is rejected by the schema; surfaces MUST NOT mint a
partial reconcile-action set.

## 5. Resolution disposition

Every conflict review records exactly one
`resolution_disposition_class`. Disposition values bind
mechanically to review-state classes (section 1.2 rule 5);
`pending` is the only disposition compatible with `pending_user_
review`, `in_review`, or any blocked state, and the schema
enforces the binding.

### 5.1 Disposition rules (frozen)

1. `rebase_local_on_remote` is forbidden when any
   `conflict_signals` entry carries `conflict_class =
   target_identity_conflict` or `conflict_class =
   permission_conflict`. Surfaces MUST route those reviews to
   `fork_into_new_target` or to repair instead.
2. `edit_local_then_review_again` is forbidden when
   `draft_state_class` is `draft_deleted_since_compare` or
   `draft_rolled_back_since_compare`. The local draft must still
   be editable before the disposition is admissible.
3. `fork_into_new_target` requires at least one
   `conflict_signals` entry whose `conflict_class` is
   `target_identity_conflict`.
4. `accept_remote_as_authoritative` is forbidden when
   `review_state_class` is `blocked_by_policy`. Admin policy review
   MUST clear before the local draft may be abandoned in favour of
   the current remote snapshot.
5. `escalate_for_admin_review` is the only disposition admissible
   under `reviewer_actor_class = unknown_actor_class` and the only
   disposition admissible under `actor_drift_class =
   actor_unknown_repair_required`.
6. Every non-`pending` disposition MUST quote a
   `resolution_summary` reviewable sentence so the resolution path
   is observable.
7. `cancel_review` is the only disposition compatible with
   `review_state_class = cancelled`.

## 6. Freshness, actor, and scope cues

The conflict review carries three top-level cue blocks so reviewers
can tell at a glance where the conflict came from before they read
the signal table:

- `freshness_class` — describes the last imported remote snapshot
  relative to the current remote snapshot (`fresh`,
  `bounded_stale`, `unbounded_stale`,
  `unknown_freshness_repair_required`). `unbounded_stale` and
  `unknown_freshness_repair_required` typically force
  `review_state_class = blocked_by_freshness`.
- `actor_drift_class` — describes whether the intended actor on
  the provider has changed between the moment the local draft was
  authored and the current review pass.
  `actor_unknown_repair_required` is repair-only and forces
  `blocked_by_actor_unknown` plus `escalate_for_admin_review`.
- `scope_drift_class` — describes whether the actor's effective
  scope still covers the intended publish action. A non-`scope_
  unchanged` value typically pairs with a `permission_conflict`
  signal whose `drift_source_class` is `actor_or_scope_change` or
  `changed_permissions`.

Surfaces MUST quote each cue verbatim. Collapsing `actor_drift_
class` and `scope_drift_class` into a single "permission error"
label, or collapsing `freshness_class` and `drift_source_class`
into a single "stale" label, is forbidden.

## 7. Export and support fields

Every conflict review carries a `support_export_channels` array
listing the typed export channels the review may ride. Every entry
names a typed `support_export_channel_class`, a `channel_summary`
reviewable sentence, an opaque `linked_record_ref`, and two
schema-enforced boolean markers:

- `redacts_raw_credentials` — schema-enforced `const true`.
  Conflict reviews MUST NOT carry raw credentials onto any export
  channel regardless of channel class.
- `redacts_tenant_private_payloads` — schema-enforced `const true`.
  Conflict reviews MUST NOT carry raw tenant-private bodies onto
  any export channel; structured field summaries and opaque
  snapshot refs only.

The schema rejects any record that sets either marker to false.
Adapters MAY raise the `redaction_class` on the record itself
(`metadata_safe_default` → `operator_only_restricted` →
`internal_support_restricted` → `signing_evidence_only`) but MAY
NOT widen the export-channel guarantees.

### 7.1 Export channels (frozen)

| `support_export_channel_class`        | When it applies                                                                                                            |
|---------------------------------------|----------------------------------------------------------------------------------------------------------------------------|
| `support_bundle`                      | The conflict review rides a support export so support engineering can reproduce the conflict offline.                        |
| `audit_packet`                        | The conflict review rides a typed audit packet alongside the `provider_handoff` audit-event refs the record cites.            |
| `migration_note`                      | The conflict review rides a typed migration note (e.g. when the conflict accompanies a target-identity migration).            |
| `admin_assisted_handoff_packet`       | The conflict review rides the admin-assisted-handoff packet so a privileged actor can complete the action out of band.        |
| `object_handoff_packet`               | The conflict review rides the typed object-handoff packet so an external reviewer sees the same record set.                  |

Every conflict review MUST cite at least one export channel; an
empty export-channel list is forbidden.

## 8. Reusability across surfaces

The `surface_class` vocabulary names the nine protected provider-
linked surfaces the conflict review record is reusable across:
`code_host_surface`, `issue_or_planning_surface`,
`ci_or_checks_surface`, `docs_or_portal_surface`,
`artifact_registry_surface`, `release_publisher_surface`,
`identity_provider_surface`, `ai_provider_surface`, and
`managed_admin_surface`. The same record shape, the same
vocabulary, the same reconcile-action floors, and the same
disposition rules apply on every surface. A surface that mints a
synthetic conflict-review shape, or that introduces a new
`reconcile_action_class`, is forbidden; the contract sets the
floor, and additive-minor changes flow through a
`provider_conflict_review_schema_version` bump.

## 9. Redaction posture (frozen)

Every conflict review record declares a `redaction_class` from the
ADR-0007 / ADR-0010 set (`metadata_safe_default`,
`operator_only_restricted`, `internal_support_restricted`,
`signing_evidence_only`). Raw URLs, raw tokens, raw callback
bodies, raw delegated-token bodies, raw policy-injector material,
raw provider payloads, and raw preview / draft / remote-snapshot
bodies MUST NOT cross this boundary on any surface.

Support exports MAY name `connected_provider_record_id`,
`surface_class`, `target_object_identity`,
`reviewer_actor_class`, `review_state_class`, `blocked_reason_class`,
`freshness_class`, `actor_drift_class`, `scope_drift_class`,
`draft_state_class`, the typed snapshot envelopes (with their
typed snapshot classes, captured-at timestamps, freshness classes,
and reviewable summaries), the typed conflict-signal table (with
its typed conflict classes, diff-signal classes, drift-source
classes, and reviewable signal summaries), the typed reconcile-
action table (with its typed action classes, admissibility, and
reviewable action summaries), the typed support-export channel
list, the typed resolution disposition, the typed origin
disclosure, the typed policy context, the typed audit-event refs,
and the typed `redaction_class`. They MUST NOT name raw URLs, raw
tokens, raw callback bodies, raw delegated-token bodies, raw
policy-injector material, raw provider payloads, or raw preview /
draft / remote-snapshot bodies.

Narrowing is permitted: admin policy MAY raise the
`redaction_class` to `operator_only_restricted`,
`internal_support_restricted`, or `signing_evidence_only`. Widening
beyond the frozen rules is forbidden.

## 10. Audit-event reuse

Every conflict-review transition fires on the ADR-0010
`provider_handoff` audit stream using the frozen event ids already
exported by the publish-later and deferred-publish contracts:

- `provider_action_proposed`
- `provider_action_denied`
- `provider_action_deferred`
- `provider_action_published`
- `provider_action_rolled_back`
- `deferred_publish_queue_drained`
- `deferred_publish_queue_rejected`
- `policy_epoch_rolled_invalidations`
- `browser_handoff_callback_validated`
- `browser_handoff_callback_rejected`
- `browser_handoff_revoked`

No new audit-event id is introduced by this contract. The
conflict review record is the *payload* those frozen events
reference; the `audit_event_refs` array on each record cites the
opaque event ids the listener emitted.

## 11. Acceptance criteria cross-walk

| Acceptance criterion                                                                                                                                          | Where enforced                                                                                                                                                                                                                                                                                                                                                                                                                  |
|---------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| No provider-linked mutation path may resolve conflicts through an unreviewed last-writer-wins default.                                                         | Section 1.2 rules 6 and 8; section 4.2 (three reconcile-action floors); section 5.1 (disposition rules; non-`pending` requires a typed reconcile action); schema enforcement on `compare_local_with_remote` and `compare_intended_publish_with_remote` always-admissible plus at least one resolving action.                                                                                                                       |
| Reviewers can reconstruct what the local draft would overwrite and what remote state changed since the draft began.                                          | Section 1.2 rules 2 and 3; section 2 (four typed snapshots); section 3 (conflict-signal table with conflict-class, diff-signal, drift-source, and per-row local / remote / imported value refs); schema enforcement on snapshot_class binding, on the four required snapshot fields, and on the non-empty `conflict_signals` array.                                                                                                |
| The conflict object model is reusable across issue, review, planning, CI/status, and provider-backed docs or portal surfaces.                                 | Section 1.1 `surface_class` vocabulary covering `code_host_surface`, `issue_or_planning_surface`, `ci_or_checks_surface`, `docs_or_portal_surface`, `artifact_registry_surface`, `release_publisher_surface`, `identity_provider_surface`, `ai_provider_surface`, `managed_admin_surface`; section 8; the same record shape governs every protected provider-linked surface.                                                       |

## 12. Schema-of-record posture (frozen)

Rust types in the eventual provider-mode crate are the source of
truth. The JSON Schema export at
`schemas/providers/provider_conflict_review.schema.json` is the
cross-tool boundary every non-owning surface reads. The paired
`deferred_publish_queue_item_record`,
`deferred_publish_review_record`,
`publish_later_queue_item_record`,
`provider_consequence_preview_record`,
`account_mapping_binding_record`, and
`provider_object_relation_record` continue to be exported by their
own schemas; this contract does not redefine those records and
cites them through opaque refs (`originating_local_draft_ref`,
`originating_queue_item_ref`,
`originating_consequence_preview_ref`,
`linked_provider_object_relation_ref`).

Adding a new `conflict_class`, `diff_signal_class`,
`drift_source_class`, `actor_drift_class`, `scope_drift_class`,
`freshness_class`, `reconcile_action_class`, `review_state_class`,
`blocked_reason_class`, `resolution_disposition_class`,
`support_export_channel_class`, or `surface_class` is additive-
minor and requires a `provider_conflict_review_schema_version`
bump; repurposing an existing value is breaking and requires a
new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, and ADR 0010.
