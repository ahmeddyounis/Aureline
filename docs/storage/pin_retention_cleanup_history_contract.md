# Pin / retention manager, cleanup-history lane, and evidence-class expiry contract

This document is the **persistence-and-cleanup attribution contract**
for Aureline's pin manager, retention surface, and post-action
cleanup-history lane. It freezes one `pin_retention_entry_record` and
one `cleanup_history_event_record` so the **pin / retention manager
(pin source, expiry or policy window, referenced object, actor class,
unpin path, export path when the object is durable evidence) and the
cleanup-history lane (timestamp, actor class, affected class, bytes
reclaimed, blocked items, resulting state, reopen-inspector action)**
project the same record family rather than per-surface "what's pinned"
or "freed up space" copy.

The contract exists so a pin manager row, a clear-data review sheet's
protected-class refusal, a low-disk banner's protected-pin-source list,
the cleanup-history lane, the admin storage console, the support-bundle
storage section, and the CLI text formatter all consume one boundary
record family without recomputing field names; so durable evidence and
support artifacts whose retention is policy-bounded cannot be mistaken
for ordinary cache eviction; so user-owned recovery state and policy-
held evidence carry typed unpin and export paths instead of generic
"clear" copy; and so user, admin, policy, and automatic actor classes
each appear in pin and cleanup histories with a typed coarse framing
that names *who* or *what* held or reclaimed bytes without exposing
raw policy bodies, raw secret-bearing payloads, or raw paths.

The contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX Spec, or design-system style guide, those sources win and this
document plus its schemas and fixtures update in the same change.
Where the pin / retention manager, cleanup-history lane, clear-data
review sheet, low-disk banner, support bundle, admin console, or CLI
text formatter mints a parallel pin row, parallel history event,
parallel "who reclaimed it" projection, or parallel reopen action,
this contract wins and the surface is non-conforming.

## Companion artifacts

- [`/schemas/storage/pin_retention_entry.schema.json`](../../schemas/storage/pin_retention_entry.schema.json)
  — boundary schema for the `pin_retention_entry_record`.
- [`/schemas/storage/cleanup_history_event.schema.json`](../../schemas/storage/cleanup_history_event.schema.json)
  — boundary schema for the `cleanup_history_event_record`.
- [`/fixtures/storage/pin_retention_cleanup_cases/`](../../fixtures/storage/pin_retention_cleanup_cases/)
  — worked YAML cases covering at least the five acceptance
  scenarios (user pin, policy pin, expired retention window, blocked
  cleanup, and auto-eviction history event) plus a cleanup-blocked
  managed-policy throttle event and a support-export drain.

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary that is already frozen
upstream; it consumes the frozen sets by name and by value:

- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md),
  [`/artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml),
  [`/artifacts/runtime/low_disk_drills.yaml`](../../artifacts/runtime/low_disk_drills.yaml),
  and [`/schemas/runtime/cache_entry_manifest.schema.json`](../../schemas/runtime/cache_entry_manifest.schema.json)
  — the six frozen `storage_class_id` values, the four
  `authority_class` values, the ten `pin_source_class` values, the
  four `clear_cache_protection_class` values, the eight
  `low_disk_ladder_step` values, the nine `actor_class` values, the
  ten `pin_expiry_anchor` values, and the per-pin-source expiry rules
  (case closure, review-pack closure, release retention window,
  offline-bundle validity, certification window, next signed policy
  epoch, export-assembly in-flight, retention-window elapsed, user
  unpin only, admin unpin only). The pin / retention entry and the
  cleanup-history event records re-export these names byte-for-byte.
- [`/docs/storage/storage_inspector_contract.md`](./storage_inspector_contract.md),
  [`/docs/storage/workspace_storage_detail_contract.md`](./workspace_storage_detail_contract.md),
  [`/docs/storage/clear_data_and_low_disk_contract.md`](./clear_data_and_low_disk_contract.md),
  and the corresponding schemas — the parent
  `storage_inspector_card_record`, the
  `workspace_storage_detail_row_record`, the
  `clear_data_review_record`, and the `low_disk_banner_record`. Pin
  / retention entries cite a parent inspector card by ref, link the
  matching workspace-detail row, and link the originating clear-data
  review and low-disk banner refs when applicable. Cleanup-history
  events cite the originating clear-data review by `preview_ref`,
  the triggering low-disk banner by `triggering_low_disk_banner_ref`,
  and the held pin-retention entries by `linked_pin_retention_entry_refs`.
- [`/docs/ux/deployment_summary_contract.md`](../ux/deployment_summary_contract.md)
  and [`/schemas/deployment/deployment_summary_card.schema.json`](../../schemas/deployment/deployment_summary_card.schema.json)
  — the `deployment_profile`, `mirror_offline_state_class`,
  `tenant_org_scope_class`, and `mirror_offline_artifact_row_record`
  values the deployment-context block re-exports for the local-only,
  mirrored / offline, and managed-policy framings.
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  — `authority_class` vocabulary
  (`user_authored_durable_truth`, `user_owned_recovery_state`,
  `admin_or_control_artifact`, `disposable_derived_cache`).
- [`/docs/governance/storage_and_retention_vocabulary.md`](../governance/storage_and_retention_vocabulary.md)
  — the storage-mode and retention-mode vocabulary that surfaces
  consume alongside the pin / retention entry's evidence-export class.

## Who reads this contract

- **Pin / retention manager, clear-data review sheet, low-disk
  banner, cleanup-history lane, admin storage console, support-
  bundle storage section, and CLI `--pins` / `--cleanup-history`
  text formatter** — to read **one** pin-retention-entry record
  family and **one** cleanup-history-event record family instead of
  recomputing fields per surface.
- **Project doctor / attention inbox, status-bar storage cell, and
  About-panel storage excerpt** — to surface the held-pin count,
  the next-eviction posture, and the recently-reclaimed totals.
- **Reviewers (release, security, accessibility, claim-manifest)** —
  to verify that durable evidence and user-owned recovery state can
  never be mistaken for ordinary cache eviction; that every pin row
  names *who* declared it, *what* object it references, *when* it
  may expire, *which* unpin path is admissible, and (when evidence
  or recovery state is involved) *which* export path is offered
  before any clear; that every cleanup-history row names the actor
  class, the affected per-class outcome, and the inspect-only
  reopen-inspector action; and that user, admin, policy, and
  automatic actor classes are typed distinctly in both records.

## Two questions the contract answers

Any Aureline surface claiming to expose pin / retention or
cleanup-history truth MUST answer both questions mechanically, without
per-surface copy:

1. **Why is this artifact still on disk, when may it expire, who
   declared the pin, and what is the only admissible path that
   removes or unpins it (and exports it first if the object is
   durable evidence)?** Which `storage_class_id` is involved? Which
   `pin_source_class` and which coarse `pin_kind_class` does the
   surface render? Which `referenced_object_class` and which opaque
   `referenced_object_ref` does the pin point at? Which
   `actor_class` declared the pin and which coarse `actor_framing_class`
   reads alongside it? Which `pin_expiry_class` (anchor / timestamp /
   user-unpin-only / admin-unpin-only) describes the pin and what
   is the live `expires_at_state_class` posture? Which
   `unpin_path_class` and which opaque inspect-only route does the
   surface offer? When the referenced object is durable evidence or
   user-owned recovery state, which `evidence_export_class` does the
   surface offer (export-required-before-clear, export-offered-
   metadata-safe, export-offered-operator-only, export-unsupported,
   not-applicable-disposable)?
2. **Who or what reclaimed space, what did it touch, what did it
   leave behind, and how does the user reopen the inspector to see
   the current state?** Which `event_class` describes the cleanup
   (user clear, admin policy clear, automatic low-disk-governor
   eviction, automatic retention expiry, automatic schema-drift
   invalidation, automatic corrupt-cache repair, support-export
   drain, blocked-pinned-or-protected, blocked-managed-policy-
   throttle)? Which `actor_class` and `actor_framing_class` ran the
   cleanup? Which per-class `affected_class_rows[]` did it touch
   (storage class, bytes reclaimed, entries reclaimed, blocked
   counts, resulting posture, resulting class state, evidence
   disclosure)? Which `blocked_items_summary[]` rows name the
   blocked reason and link the matching pin-retention entry or
   class-specific review? Which originating clear-data review,
   triggering low-disk banner, ladder step, and follow-up actions
   does the row reference? Which inspect-only `reopen_inspector_action`
   takes the user back to the inspector card?

Generic prose like "files cleaned up", "system maintenance", "old
data removed", "your work is broken", or "cleared cache" is forbidden
when a more precise per-class, per-event-class, per-actor-class
figure is knowable. The schema enforces typed vocabulary and typed
sentences; surfaces render those values.

## 1. Scope

This contract freezes:

- One **pin / retention entry record** (§3) emitted whenever a pin
  manager, clear-data review sheet, low-disk banner, cleanup-history
  lane, admin storage console, support bundle, or CLI text formatter
  needs to project one held artifact's persistence rationale. The
  record carries: `parent_card_id_ref`, the `entry_scope`, the
  `storage_class_id`, the `authority_class`, the
  `referenced_object_class` and `referenced_object_ref`, the
  `pin_source_class` and the coarse `pin_kind_class`, the
  `declared_by_actor_class` and `actor_framing_class`, the
  `pin_expiry_class` and the `pin_expiry_or_policy_window`, the
  live `expires_at_state_class`, the `pinned_bytes` and
  `pinned_entry_count`, the `unpin_path_class` with its inspect-only
  `open_unpin_path_action`, the `evidence_export_class` with its
  inspect-only `open_export_path_action` (when applicable), the
  `protected_class_visibility[]` tokens, the `deployment_context`,
  the `consumer_surfaces[]`, the `redaction_class`, the
  `export_safe`, and the `note`.
- One **cleanup-history event record** (§4) emitted whenever a
  cleanup-history lane needs to project one cleanup outcome on one
  inspector scope. The record carries: `parent_card_id_ref`, the
  `event_scope`, the `event_class`, the `actor_class` and
  `actor_framing_class`, the `actor_label`, the
  `affected_class_rows[]` (per-class outcome), the
  `blocked_items_summary[]`, the totals (`bytes_reclaimed_total`,
  `entries_reclaimed_total`, `blocked_pinned_entries_total`,
  `blocked_protected_entries_total`), the
  `evidence_disclosure_class`, the originating `preview_ref`, the
  `triggering_low_disk_banner_ref`, the `ladder_step_applied`, the
  `follow_up_actions_required[]`, the inspect-only
  `reopen_inspector_action`, the `deployment_context`, the
  `consumer_surfaces[]`, the `redaction_class`, the `export_safe`,
  and the `note`.
- The **pin-kind-class to pin-source-class pairings** (§5) so the
  coarse pin kind a surface renders in product terms cannot drift
  from the precise pin-source class.
- The **pin-expiry-class taxonomy** (§6) — `expiry_required_with_anchor`,
  `expiry_required_with_timestamp`,
  `expiry_user_unpin_only_indefinite`,
  `expiry_admin_unpin_only_indefinite` — and the live
  `expires_at_state_class` vocabulary
  (`active_within_window`, `expiring_soon_within_grace`,
  `past_retention_eligible_for_expiry`,
  `past_grace_pending_explicit_review`, `never_expires_indefinite`).
- The **unpin-path-class taxonomy** (§7) — every pin row carries a
  single admissible unpin path so the surface cannot pretend a
  policy-held pin can be released by user action.
- The **evidence-export-class taxonomy** (§8) — typed export options
  for durable evidence, user-owned recovery state, and disposable
  rows so a pin or history row never offers a generic "clear" over
  evidence without first naming the export path.
- The **event-class taxonomy** (§9) for cleanup-history events, the
  **resulting-class-state vocabulary** (§9.2), the
  **blocked-reason vocabulary** (§9.3), and the
  **evidence-disclosure vocabulary** (§9.4) so durable evidence and
  user-owned recovery state cannot hide behind a generic cache-
  eviction row.
- The **actor-framing pairings** (§10) so user, admin or tenant
  policy, automatic system, and support-export actors each appear
  with the matching coarse framing.
- The **deployment-context block** (§11) re-exporting the
  `deployment_profile`, `mirror_offline_state_class`, and
  `tenant_org_scope_class` so a pin row or history event on an air-
  gapped install, an enterprise managed tenant, or an individual
  local install renders with the right framing.
- The **cross-record invariants** (§12) so user-owned recovery
  state, evidence support cache, mirror / offline / signed-pack
  artifacts, and managed-policy throttle each render distinctly,
  and so blocked cleanups never report non-zero reclaimed totals.

## 2. Out of scope

- The retention scheduler, the admin policy daemon, the cleanup
  daemon, and the cache-manager GC engine. The cache manager (frozen
  in [`docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md))
  remains the source of truth for cache-entry admission, low-disk
  ordering, retention enforcement, and pin-ref bookkeeping. This
  contract pins the inspectable record family for the pin manager
  and the cleanup-history lane; it does not implement retention
  jobs, cleanup workers, or any GC daemon.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and the shell-interaction-safety contract own the strings.
- Pixel-perfect chart and bar layout, the on-disk byte format of
  any store, and the live retention scheduler.
- The class-specific review sheet for `evidence_support_cache` and
  `user_owned_recovery_state`. The cache-manifest schema and the
  local-history restore-preview contract freeze those review
  records; the pin-retention entry and the cleanup-history event
  link them by ref.
- Cleanup execution, retention sweeping, and admin policy
  evaluation. The records freeze the post-action attribution
  family; surfaces consume the records the cache manager and
  retention scheduler emit.

## 3. The pin / retention entry record

A pin / retention entry record is one structured projection of one
held artifact against one inspector scope. The pin manager, clear-
data review sheet, low-disk banner, cleanup-history lane, admin
storage console, support bundle, and CLI text formatter each render
the same record without changing field names.

### 3.1 Required fields

- `record_kind = pin_retention_entry_record`.
- `pin_retention_entry_schema_version = 1`.
- `entry_id` — opaque, stable, safe to log and export.
- `emitted_at` — RFC 3339 UTC timestamp from a monotonic clock.
- `parent_card_id_ref` — opaque ref into the parent
  `storage_inspector_card_record` whose scope this entry resolves
  against.
- `entry_scope` — see §3.2. Value-equal to the parent inspector
  card's `inspector_scope`.
- `storage_class_id` — re-export of the runtime vocabulary. The
  schema forbids `interactive_hot_cache` because the cache-manager
  protection class for the hot cache is
  `generic_clear_always_allowed` and the hot cache cannot be the
  target of a pin ref.
- `authority_class` — re-export. `user_authored_durable_truth` is
  forbidden (those classes route through the profile / state-map
  storage path instead).
- `referenced_object_class` — closed eleven-class taxonomy of
  upstream object kinds the entry references (§3.3).
- `referenced_object_ref` — opaque, stable ref into the upstream
  object.
- `referenced_object_label` — short reviewable label naming the
  object in product terms (e.g. "Case 42 — exported evidence
  packet"). Generic "Pinned object" copy is non-conforming when a
  more precise label is knowable.
- `pin_source_class` — re-export of the runtime vocabulary.
- `pin_kind_class` — coarse eight-class framing rendered alongside
  the precise pin source so a row reads at a glance (§5).
- `declared_by_actor_class` — re-export of the runtime
  `actor_class` vocabulary.
- `actor_framing_class` — coarse four-class framing
  (`user_acted`, `admin_or_tenant_policy_acted`,
  `automatic_system_acted`, `support_export_acted`) that pairs with
  the actor class. The pairing is schema-enforced (§10).
- `pin_expiry_class` — closed four-class vocabulary (§6.1).
- `pin_expiry_or_policy_window` — RFC 3339 timestamp, anchor block
  (`anchor`, `note`), or `null`. The shape pairs with
  `pin_expiry_class` (§6.2).
- `expires_at_state_class` — closed five-class live posture
  vocabulary (§6.3).
- `pinned_bytes` — non-negative integer.
- `pinned_entry_count` — non-negative integer.
- `unpin_path_class` — closed ten-class unpin-path vocabulary (§7).
- `open_unpin_path_action` — inspect-only open action shape (§3.4)
  routing to the unpin path.
- `evidence_export_class` — closed five-class evidence-export
  vocabulary (§8).
- `open_export_path_action` — inspect-only open action shape (§3.4)
  routing to the export path. Required non-null for
  `export_required_before_clear`, `export_offered_metadata_safe`,
  and `export_offered_operator_only`; required null for
  `export_unsupported_class` and `not_applicable_disposable`.
- `protected_class_visibility[]` — non-empty array of visibility
  tokens (§3.5) the surface MUST acknowledge.
- `deployment_context` — see §11.
- `consumer_surfaces[]` — closed seven-class set (§3.6).
- `redaction_class` — one of `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only`. Exported entries MUST NOT widen
  redaction beyond `operator_only_restricted`.
- `export_safe` — boolean.
- `note` — short reviewable sentence in product terms.

Optional fields: `linked_clear_data_review_ref`,
`linked_low_disk_banner_ref`, `linked_class_specific_review_ref`,
`linked_cleanup_history_lane_ref`,
`linked_workspace_storage_detail_row_ref`,
`linked_mirror_offline_artifact_row_ref`,
`linked_deployment_summary_card_ref`.

### 3.2 `entry_scope`

Every record carries an `entry_scope` block whose `scope_class`,
`scope_ref`, and `scope_label` are value-equal to the parent
inspector card's `inspector_scope`. The schema enforces this
equivalence so a workspace-only entry cannot bleed into another
workspace's bytes.

### 3.3 `referenced_object_class` (closed)

Closed eleven-class taxonomy:

- `cache_entry_manifest_node` — a registered cache-entry manifest
  whose `clear_cache_protection_class` is
  `generic_clear_with_pin_exclusions`.
- `workspace_storage_detail_row` — a workspace-detail row that
  carries a pin (the entry mirrors the row's pin state).
- `mirror_offline_artifact_row` — a mirror / offline artifact row
  pinned by an `offline_bundle_ref` or a release-artifact-graph
  ref.
- `case_or_review_pack_node` — a case or review-pack node holding
  evidence under retention.
- `release_artifact_graph_node` — a release-artifact-graph node.
- `offline_bundle_segment_node` — an offline-bundle segment.
- `certified_archetype_or_template_node` — a certified archetype
  or template.
- `policy_signed_pack_node` — a signed policy pack
  (`policy_bundle_last_known_good_ref`).
- `support_export_assembly_node` — an in-flight support-export
  assembly.
- `retention_window_node` — a retention-window node.
- `user_owned_recovery_journal_node` — a user-owned recovery-
  journal node (the only `user_owned_recovery_state` referenced
  object).

The schema enforces every pairing in §3.3 with the `pin_source_class`
in §5.

### 3.4 `open_unpin_path_action` and `open_export_path_action`

Both fields share the inspect-only open-action shape. Required
fields:

- `action_id` — opaque id.
- `label` — short reviewable label (e.g. "Open pin manager for
  Case 42 evidence" or "Open export-before-clear path for the
  local recovery journal"). Generic "Manage" or "Open" copy is
  non-conforming when a more precise label is knowable.
- `target_route_ref` — opaque ref to the unpin or export route.
- `scope_class` — `scope_local_only`.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `revalidation_on_open` — one of `none_already_fresh`,
  `snapshot_open_read_only`.
- `modal_prohibited` — `true`.

`open_export_path_action` is required null when
`evidence_export_class` is `not_applicable_disposable` or
`export_unsupported_class`; required non-null otherwise.

### 3.5 `protected_class_visibility[]` (closed)

Closed five-token set the entry surfaces so a pin manager row never
reads as a generic cache row when a protected class is involved:

- `evidence_support_cache_visible` — required on every entry whose
  `storage_class_id` is `evidence_support_cache`.
- `user_owned_recovery_state_visible` — required on every entry
  whose `storage_class_id` is `user_owned_recovery_state`.
- `mirror_or_offline_pack_visible` — required when
  `referenced_object_class` is `offline_bundle_segment_node` or
  `mirror_offline_artifact_row`. Forces a non-null
  `linked_mirror_offline_artifact_row_ref`.
- `policy_signed_pack_visible` — required when
  `referenced_object_class` is `policy_signed_pack_node`.
- `not_applicable` — admissible when none of the above apply (a
  pure disposable-cache pin row).

### 3.6 `consumer_surface_class` (closed)

Closed seven-class set of surfaces consuming the record:

- `pin_manager` — pin / retention manager.
- `clear_data_review` — clear-data review sheet.
- `low_disk_banner` — low-disk / quota banner.
- `cleanup_history_lane` — post-action attribution lane.
- `admin_storage_console` — admin / tenant storage console.
- `support_packet_export` — support-bundle storage section.
- `cli_text_formatter` — CLI text formatter render.

## 4. The cleanup-history event record

A cleanup-history event record is one structured post-action
attribution row in the cleanup-history lane. The lane, the clear-
data review sheet's history excerpt, the low-disk banner's history-
link affordance, the pin manager's blocked-history list, the admin
storage console, the support-bundle storage section, and the CLI
text formatter each render the same record without changing field
names.

### 4.1 Required fields

- `record_kind = cleanup_history_event_record`.
- `cleanup_history_event_schema_version = 1`.
- `event_id` — opaque, stable, safe to log and export.
- `emitted_at` — RFC 3339 UTC timestamp from a monotonic clock
  describing when the record was assembled.
- `event_at` — RFC 3339 UTC timestamp from a monotonic clock
  describing when the cleanup actually ran.
- `parent_card_id_ref` — opaque ref into the parent
  `storage_inspector_card_record`.
- `event_scope` — see §4.2. Value-equal to the parent inspector
  card's `inspector_scope`.
- `event_class` — closed nine-class vocabulary (§9.1).
- `actor_class` — re-export of the runtime vocabulary.
- `actor_framing_class` — coarse four-class framing (§10).
- `actor_label` — short reviewable label naming the actor in
  product terms.
- `affected_class_rows[]` — non-empty array of per-class outcome
  rows (§4.3).
- `blocked_items_summary[]` — array of blocked-item rows (§4.4).
  Required non-empty for `cleanup_blocked_pinned_or_protected` and
  `cleanup_blocked_managed_policy_throttle` events.
- `bytes_reclaimed_total` — non-negative integer. Required zero on
  blocked events.
- `entries_reclaimed_total` — non-negative integer. Required zero
  on blocked events.
- `blocked_pinned_entries_total` — non-negative integer.
- `blocked_protected_entries_total` — non-negative integer.
- `evidence_disclosure_class` — closed eight-class evidence-
  disclosure vocabulary (§9.4).
- `preview_ref` — opaque ref or `null`. Required non-null for
  `user_clear_data_committed` and `admin_policy_clear_committed`;
  required null otherwise.
- `triggering_low_disk_banner_ref` — opaque ref or `null`.
  Required non-null for `automatic_low_disk_governor_eviction` and
  `cleanup_blocked_managed_policy_throttle`; required null
  otherwise.
- `ladder_step_applied` — `low_disk_ladder_step` value or `null`.
  Required non-null for `automatic_low_disk_governor_eviction`;
  required null otherwise.
- `follow_up_actions_required[]` — non-empty closed seven-class
  set (§4.5).
- `reopen_inspector_action` — inspect-only open action shape
  (§4.6) routing back to the parent inspector card.
- `deployment_context` — see §11.
- `consumer_surfaces[]` — closed seven-class set (§4.7).
- `redaction_class` — one of the four standard classes. Exported
  records MUST NOT widen redaction beyond
  `operator_only_restricted`.
- `export_safe` — boolean.
- `note` — short reviewable sentence.

Optional fields: `linked_clear_data_review_ref`,
`linked_low_disk_banner_ref`, `linked_pin_retention_entry_refs[]`,
`linked_class_specific_review_refs[]`,
`linked_workspace_storage_detail_row_refs[]`,
`linked_deployment_summary_card_ref`.

### 4.2 `event_scope`

Every event carries an `event_scope` block whose `scope_class`,
`scope_ref`, and `scope_label` are value-equal to the parent
inspector card's `inspector_scope`.

### 4.3 `affected_class_rows[]`

Each row carries:

- `storage_class_id` — re-export.
- `bytes_reclaimed` — non-negative integer.
- `entries_reclaimed` — non-negative integer. Pairs with
  `bytes_reclaimed`: non-zero entries require non-zero bytes (and
  vice versa).
- `blocked_pinned_entries_count` — non-negative integer.
- `blocked_protected_entries_count` — non-negative integer.
- `resulting_posture` — re-export of `storage_posture_class`.
- `resulting_class_state_class` — closed ten-class vocabulary
  (§9.2).
- `evidence_disclosure_class` — closed eight-class disclosure
  vocabulary (§9.4) describing this row's evidence treatment.
- `class_label` — short reviewable label.
- `linked_class_specific_review_ref` — opaque ref or `null`.
  Required non-null for
  `retention_expired_evidence_now_eligible_for_review` and
  `blocked_class_unchanged_pending_class_specific_review` rows.
- `notes` — optional short reviewable sentence.

Schema-enforced row pairings:

- `user_owned_recovery_state` rows carry zero
  `bytes_reclaimed` / `entries_reclaimed` and a non-cleared
  resulting state (the class never disappears under generic
  clear).
- `evidence_support_cache` rows carry one of the four evidence-
  specific disclosure values (no `not_applicable` and no
  `evidence_class_not_touched`).
- Disposable cache rows carry `not_applicable` disclosure or, for
  the `support_export_assembly_drained` event class only,
  `support_export_assembly_pin_drained`.

### 4.4 `blocked_items_summary[]`

Each row carries `blocked_reason_class`, `blocked_storage_class_id`,
`blocked_entry_count`, `blocked_bytes`, the linked
`pin_retention_entry_ref` (required non-null for pin-bearing
reasons), the linked `class_specific_review_ref` (required non-
null for evidence and recovery-state reasons), the
`blocked_label`, and an optional `notes` sentence.

The blocked reasons (closed eight-class vocabulary) are defined
in §9.3.

### 4.5 `follow_up_actions_required[]`

Closed seven-class set re-exported from the cache-entry-manifest
schema and extended with the class-specific review re-runs:

- `rebuild_knowledge_cache`
- `rewarm_prebuild_environment_cache`
- `redownload_artifact_cache`
- `reissue_support_export_after_case_closure`
- `rerun_class_specific_review_for_evidence`
- `rerun_class_specific_review_for_user_owned_recovery_state`
- `no_follow_up_required`

The array is non-empty; surfaces use it to drive next-step copy
without inventing parallel "what to do next" labels.

### 4.6 `reopen_inspector_action`

Every event MUST carry exactly one inspect-only reopen action
returning to the parent inspector card. The shape mirrors the
storage-inspector card's `open_details_action`:

- `action_id` — opaque id.
- `label` — short reviewable label (e.g. "Reopen storage
  inspector for this workspace").
- `target_route_ref` — opaque ref to the inspector route.
- `scope_class` — `scope_local_only`.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `revalidation_on_open` — one of `none_already_fresh`,
  `snapshot_open_read_only`.
- `modal_prohibited` — `true`.

### 4.7 `consumer_surface_class` (closed)

- `cleanup_history_lane`
- `clear_data_review`
- `low_disk_banner`
- `pin_manager`
- `admin_storage_console`
- `support_packet_export`
- `cli_text_formatter`

## 5. `pin_kind_class` to `pin_source_class` pairings

The coarse `pin_kind_class` reads alongside the precise
`pin_source_class` so a pin row can be skimmed without raw policy
bodies. The schema enforces every pairing:

| `pin_kind_class`                          | required `pin_source_class`                                                | required `referenced_object_class`            | required `unpin_path_class`                          |
| ----------------------------------------- | -------------------------------------------------------------------------- | --------------------------------------------- | ---------------------------------------------------- |
| `user_pin`                                | `explicit_user_pin`                                                        | (any pinnable object)                         | `user_unpin_self`                                    |
| `admin_policy_pin`                        | `explicit_admin_policy_pin`                                                | (any pinnable object)                         | `admin_unpin_policy_action`                          |
| `tenant_policy_pin`                       | `explicit_admin_policy_pin`                                                | (any pinnable object)                         | `admin_unpin_policy_action`                          |
| `case_or_review_reference_pin`            | `case_reference_ref` or `review_pack_ref`                                  | `case_or_review_pack_node`                    | `case_closure_or_review_pack_closure_required`       |
| `release_or_offline_bundle_pin`           | `release_artifact_graph_ref` or `offline_bundle_ref`                       | `release_artifact_graph_node` / `offline_bundle_segment_node` | `release_artifact_pin_release_required` / `offline_bundle_unpin_required` |
| `certified_template_or_signed_pack_pin`   | `certified_archetype_or_template_ref` or `policy_bundle_last_known_good_ref` | `certified_archetype_or_template_node` / `policy_signed_pack_node` | `certified_template_recertify_or_release_required` / `policy_pack_resign_or_supersede_required` |
| `support_export_assembly_pin`             | `support_export_assembly_ref`                                              | `support_export_assembly_node`                | `support_export_assembly_complete_or_drain_required` |
| `retention_window_pin`                    | `retention_window_ref`                                                     | `retention_window_node`                       | `retention_window_must_elapse`                       |

The schema also enforces:

- `tenant_policy_pin` requires the `managed_policy` or
  `managed_policy_air_gapped` framing and a managed-tenant scope.
- `support_export_assembly_pin` pairs with the
  `support_export_assembly` actor and the `support_export_acted`
  framing.
- `retention_window_pin` pairs with the `system_retention_policy`
  actor and the `automatic_system_acted` framing.

## 6. Pin-expiry vocabulary

### 6.1 `pin_expiry_class` (closed)

Closed four-class vocabulary:

- `expiry_required_with_anchor` — pin expires when an upstream
  window elapses (case closure, review-pack closure, release-
  ring window, offline-bundle window, certification window,
  next-signed-policy epoch, export-assembly-in-flight, retention-
  window-elapsed). The `pin_expiry_or_policy_window` is an anchor
  block (`anchor`, `note`).
- `expiry_required_with_timestamp` — pin expires at a specific
  RFC 3339 timestamp (rare; only when the upstream window resolves
  to a fixed date). The `pin_expiry_or_policy_window` is a string
  timestamp.
- `expiry_user_unpin_only_indefinite` — pin remains until the
  user removes it. The `pin_expiry_or_policy_window` is `null`.
  Pairs with `explicit_user_pin`.
- `expiry_admin_unpin_only_indefinite` — pin remains until an
  admin removes it. The `pin_expiry_or_policy_window` is `null`.
  Pairs with `explicit_admin_policy_pin`.

### 6.2 `pin_expiry_or_policy_window`

Either:

1. A string RFC 3339 UTC timestamp (when
   `pin_expiry_class = expiry_required_with_timestamp`).
2. An anchor object with the closed
   [`pin_expiry_anchor`](#pin_expiry_anchor) value and a
   reviewable note (when
   `pin_expiry_class = expiry_required_with_anchor`).
3. `null` (when `pin_expiry_class` is one of the indefinite
   classes).

The anchor vocabulary is re-exported byte-for-byte from
`/schemas/runtime/cache_entry_manifest.schema.json`:

- `case_closure_plus_policy_window`
- `review_pack_closure_plus_policy_window`
- `release_ring_retention_window`
- `offline_bundle_validity_window`
- `certification_window`
- `next_signed_policy_epoch`
- `export_assembly_in_flight_only`
- `retention_window_elapsed`
- `user_unpin_only`
- `admin_unpin_only`

### 6.3 `expires_at_state_class` (closed)

Closed five-class live posture vocabulary:

- `active_within_window` — pin is active and the window has not
  begun to elapse.
- `expiring_soon_within_grace` — the window is within its grace
  period; the surface MAY render a typed "expiring soon" cue.
- `past_retention_eligible_for_expiry` — the window has elapsed
  and the pin is now eligible for expiry under the retention
  ladder. The schema requires the pin to carry an anchor- or
  timestamp-bounded expiry class.
- `past_grace_pending_explicit_review` — the window has elapsed
  but the class is `evidence_support_cache` or
  `user_owned_recovery_state` and the surface MUST link a class-
  specific review path before any clear can fire.
- `never_expires_indefinite` — pairs with the
  `expiry_user_unpin_only_indefinite` or
  `expiry_admin_unpin_only_indefinite` expiry classes.

## 7. `unpin_path_class` (closed)

Closed ten-class unpin-path vocabulary every pin row renders with
a typed inspect-only open action:

- `user_unpin_self`
- `admin_unpin_policy_action`
- `case_closure_or_review_pack_closure_required`
- `release_artifact_pin_release_required`
- `offline_bundle_unpin_required`
- `certified_template_recertify_or_release_required`
- `policy_pack_resign_or_supersede_required`
- `support_export_assembly_complete_or_drain_required`
- `retention_window_must_elapse`
- `class_specific_review_required_user_owned_or_evidence`

The schema enforces every pairing in §5 between the unpin path,
the pin source, and the referenced object class. The
`class_specific_review_required_user_owned_or_evidence` value is
required when `authority_class` is `user_owned_recovery_state` and
admissible (alongside the source-specific values) for evidence
rows whose class-specific-review path is the only admissible
unpin.

## 8. `evidence_export_class` (closed)

Closed five-class evidence-export vocabulary the entry surfaces
when the referenced object is durable evidence or user-owned
recovery state:

- `export_required_before_clear` — the surface MUST offer the
  export path before any clear; admissible only on
  `evidence_support_cache` and `user_owned_recovery_state` rows.
- `export_offered_metadata_safe` — metadata-safe export is
  offered (admissible on `evidence_support_cache` rows whose
  redaction class is `metadata_safe_default`).
- `export_offered_operator_only` — operator-only export is
  offered (admissible on `evidence_support_cache` and
  `user_owned_recovery_state` rows whose redaction class is
  `operator_only_restricted`).
- `export_unsupported_class` — admissible only on
  `user_owned_recovery_state` rows whose authoritative-no-rebuild
  cost makes export unsupported. `open_export_path_action` MUST
  be null.
- `not_applicable_disposable` — admissible only on
  `knowledge_cache` / `artifact_cache` /
  `prebuild_environment_cache` rows.
  `open_export_path_action` MUST be null.

The schema forbids `not_applicable_disposable` on
`evidence_support_cache` and `user_owned_recovery_state` rows so
durable evidence and user-owned recovery state cannot be
mistaken for ordinary cache eviction.

## 9. Cleanup-history vocabulary

### 9.1 `event_class` (closed)

Closed nine-class vocabulary covering every cleanup-history row:

- `user_clear_data_committed` — the user committed a clear-data
  review.
- `admin_policy_clear_committed` — an admin policy committed a
  clear-data review.
- `automatic_low_disk_governor_eviction` — the low-disk governor
  applied a ladder step.
- `automatic_retention_policy_expiry` — the retention scheduler
  expired evidence past its retention window.
- `automatic_schema_drift_invalidation` — the schema-drift
  invalidator dropped a class because its
  `producing_schema_version` no longer matched.
- `automatic_corrupt_cache_repair` — the corrupt-cache detector
  repaired or quarantined a class.
- `support_export_assembly_drained` — a support-export assembly
  pin completed and drained.
- `cleanup_blocked_pinned_or_protected` — the cleanup attempt was
  blocked because every candidate row was pinned or protected.
- `cleanup_blocked_managed_policy_throttle` — the cleanup
  attempt was blocked under managed-policy throttle.

### 9.2 `resulting_class_state_class` (closed)

Closed ten-class per-row resulting-state vocabulary:

- `cleared_class_now_disposable_only`
- `cleared_class_with_pin_exclusions_remaining`
- `cleared_class_with_blocked_protected_remaining`
- `retention_expired_evidence_now_eligible_for_review`
- `retention_window_held_no_change`
- `schema_drift_invalidated_class_rebuilt_or_pending`
- `corrupt_cache_repaired_class_now_clean`
- `support_export_assembly_drained_class_now_clean`
- `managed_policy_throttle_no_change`
- `blocked_class_unchanged_pending_class_specific_review`

### 9.3 Blocked-reason vocabulary (closed)

Closed eight-class blocked-reason vocabulary:

- `blocked_evidence_class_specific_review_required`
- `blocked_authoritative_user_owned_clear_refused`
- `blocked_policy_held_pin`
- `blocked_mirror_or_offline_pinned`
- `blocked_managed_policy_throttle`
- `blocked_user_pinned_indefinite`
- `blocked_admin_pinned_indefinite`
- `blocked_corrupt_cache_repair_pending`

The schema enforces:

- `blocked_evidence_class_specific_review_required` pairs with
  `evidence_support_cache` and a non-null class-specific review
  ref.
- `blocked_authoritative_user_owned_clear_refused` pairs with
  `user_owned_recovery_state` and a non-null class-specific
  review ref.
- `blocked_policy_held_pin`, `blocked_mirror_or_offline_pinned`,
  `blocked_user_pinned_indefinite`, and
  `blocked_admin_pinned_indefinite` require a non-null
  `linked_pin_retention_entry_ref`.

### 9.4 `evidence_disclosure_class` (closed)

Closed eight-class evidence-disclosure vocabulary applied at both
the parent record and per-class-row level:

- `evidence_class_not_touched`
- `evidence_class_retention_expired_eligible_for_review`
- `evidence_class_export_required_before_clear_unblocked`
- `evidence_class_blocked_class_specific_review_pending`
- `evidence_class_held_under_policy`
- `support_export_assembly_pin_drained`
- `user_owned_recovery_state_blocked_class_specific_review_pending`
- `not_applicable`

The schema forbids `not_applicable` on records whose
`affected_class_rows[]` includes any
`evidence_support_cache` or `user_owned_recovery_state` row. This
is the rule that prevents durable evidence from hiding behind a
generic cache-eviction record.

## 10. Actor framing

The schema enforces every pairing between `actor_class` and
`actor_framing_class`:

| `actor_class`                            | `actor_framing_class`             |
| ---------------------------------------- | --------------------------------- |
| `user_clear_data_review`                 | `user_acted`                      |
| `user_pin_manager_action`                | `user_acted`                      |
| `admin_policy_action`                    | `admin_or_tenant_policy_acted`    |
| `system_low_disk_governor`               | `automatic_system_acted`          |
| `system_retention_policy`                | `automatic_system_acted`          |
| `system_schema_drift_invalidator`        | `automatic_system_acted`          |
| `system_corrupt_cache_detector`          | `automatic_system_acted`          |
| `system_cache_manager_gc`                | `automatic_system_acted`          |
| `support_export_assembly`                | `support_export_acted`            |

Surfaces render the coarse framing alongside the precise actor so
a row reads "Admin policy paused this clear" instead of
`actor.admin.policy_42`.

## 11. Deployment-context block

Every pin / retention entry record and every cleanup-history event
record carries a `deployment_context` block whose shape and pairings
match
[`docs/storage/clear_data_and_low_disk_contract.md`](./clear_data_and_low_disk_contract.md)
§7. The schema enforces:

- `local_only_individual` framing requires
  `deployment_profile = individual_local` and
  `mirror_offline_state_class = not_applicable`.
- `mirrored_or_offline` framing requires a mirror / offline
  `mirror_offline_state_class`.
- `managed_policy` framing requires a managed deployment profile
  and a managed-tenant scope.
- `managed_policy_air_gapped` framing requires
  `air_gapped` profile and `offline_air_gapped` state.

## 12. Cross-record invariants

The schema enforces these invariants mechanically. A surface that
violates any of them is non-conforming.

1. **`user_authored_durable_truth` cannot be pinned or reclaimed.**
   The pin / retention entry refuses
   `authority_class = user_authored_durable_truth`; the cleanup-
   history event cannot include a row whose authority resolves to
   user-authored durable truth (the storage-inspector contract
   keeps that authority out of the cache-class projection).
2. **User-owned recovery state never disappears under generic
   clear.** Every cleanup-history row whose
   `storage_class_id = user_owned_recovery_state` carries zero
   `bytes_reclaimed` / `entries_reclaimed` and a non-cleared
   resulting state. Every pin / retention entry whose
   `authority_class = user_owned_recovery_state` carries the
   `class_specific_review_required_user_owned_or_evidence` unpin
   path, an `export_required_before_clear` /
   `export_offered_operator_only` / `export_unsupported_class`
   evidence-export class, and a non-null
   `linked_class_specific_review_ref`.
3. **Evidence is typed distinctly from cache eviction.** Every
   pin / retention entry whose
   `storage_class_id = evidence_support_cache` carries the
   `admin_or_control_artifact` authority, an evidence-class
   referenced object (case / review pack, retention window,
   support-export assembly, or signed policy pack), an export
   class drawn from the evidence-export trio, and a non-null
   `linked_class_specific_review_ref`. Every cleanup-history
   record whose `affected_class_rows[]` includes evidence or
   user-owned recovery state carries a non-`not_applicable`
   `evidence_disclosure_class`.
4. **Mirror / offline / signed-pack pins surface visibility.**
   Pin / retention entries whose `referenced_object_class` is
   `offline_bundle_segment_node` or `mirror_offline_artifact_row`
   carry the `mirror_or_offline_pack_visible` token and a non-
   null `linked_mirror_offline_artifact_row_ref`. Entries whose
   referenced object is `policy_signed_pack_node` carry the
   `policy_signed_pack_visible` token.
5. **Pin source pairs with kind, referenced object, and unpin
   path.** §5 pairings are schema-enforced.
6. **Pin expiry pairs with state and window.** §6 pairings are
   schema-enforced; `past_grace_pending_explicit_review` requires
   a class-specific review link and applies only to evidence and
   recovery state.
7. **Actor pairs with framing.** §10 pairings are schema-
   enforced on both records.
8. **Blocked cleanups never report non-zero reclaimed totals.**
   `cleanup_blocked_pinned_or_protected` and
   `cleanup_blocked_managed_policy_throttle` events MUST carry
   `bytes_reclaimed_total = 0`,
   `entries_reclaimed_total = 0`, and a non-empty
   `blocked_items_summary[]`.
   `cleanup_blocked_managed_policy_throttle` additionally
   requires the `managed_policy` or `managed_policy_air_gapped`
   framing, a non-null `triggering_low_disk_banner_ref`, and at
   least one `blocked_managed_policy_throttle` row.
9. **Auto-eviction events name the ladder step.**
   `automatic_low_disk_governor_eviction` events MUST carry a
   non-null `ladder_step_applied` and a non-null
   `triggering_low_disk_banner_ref`; every other event class
   MUST set `ladder_step_applied = null` and
   `triggering_low_disk_banner_ref = null` (except
   `cleanup_blocked_managed_policy_throttle`, which still carries
   the banner ref).
10. **User / admin clear events cite the originating preview.**
    `user_clear_data_committed` and `admin_policy_clear_committed`
    events MUST carry a non-null `preview_ref`; every other event
    class MUST set `preview_ref = null`.
11. **Inspect-only actions stay bounded.** Every
    `open_unpin_path_action`, `open_export_path_action`, and
    `reopen_inspector_action` declares exactly
    `["no_side_effect_inspect_only"]`,
    `scope_class = scope_local_only`,
    `authority_class = user_local_authority`,
    `consent_class = no_consent_required_safe_default`, and
    `modal_prohibited = true`.
12. **Export discipline preserves redaction.** Records exported
    under `export_safe = true` MUST keep
    `redaction_class ≤ operator_only_restricted`. Exports that
    widen redaction to `internal_support_restricted` or
    `signing_evidence_only` are non-conforming.
13. **Pin-bearing blocked rows link the pin entry.** Records
    carrying a non-empty `linked_pin_retention_entry_refs[]` MUST
    also include at least one `blocked_items_summary[]` row whose
    `linked_pin_retention_entry_ref` is non-null. Surfaces that
    name pin entries in the linked array but never reference them
    in the blocked items are non-conforming.

## 13. Worked acceptance scenarios

The fixture set covers at least these five scenarios required by
the acceptance criteria, plus two additional scenarios for
managed-policy throttle and support-export drain:

1. **User pin (knowledge cache pinned indefinitely by the user).**
   `pin_kind_class = user_pin`,
   `pin_source_class = explicit_user_pin`,
   `declared_by_actor_class = user_pin_manager_action`,
   `actor_framing_class = user_acted`,
   `pin_expiry_class = expiry_user_unpin_only_indefinite`,
   `expires_at_state_class = never_expires_indefinite`,
   `unpin_path_class = user_unpin_self`,
   `evidence_export_class = not_applicable_disposable`. Deployment
   context is `local_only_individual`.
2. **Policy pin (managed-tenant evidence pack held by an admin
   policy).** `pin_kind_class = tenant_policy_pin`,
   `pin_source_class = explicit_admin_policy_pin`,
   `declared_by_actor_class = admin_policy_action`,
   `actor_framing_class = admin_or_tenant_policy_acted`,
   `pin_expiry_class = expiry_admin_unpin_only_indefinite`,
   `expires_at_state_class = never_expires_indefinite`,
   `unpin_path_class = admin_unpin_policy_action`,
   `evidence_export_class = export_required_before_clear`.
   Deployment context is `managed_policy`.
3. **Expired retention window (case-closure window elapsed on
   evidence packet).** `pin_kind_class = case_or_review_reference_pin`,
   `pin_source_class = case_reference_ref`,
   `declared_by_actor_class = system_retention_policy`,
   `actor_framing_class = automatic_system_acted`,
   `pin_expiry_class = expiry_required_with_anchor`,
   `pin_expiry_or_policy_window.anchor = case_closure_plus_policy_window`,
   `expires_at_state_class = past_grace_pending_explicit_review`,
   `unpin_path_class = case_closure_or_review_pack_closure_required`,
   `evidence_export_class = export_offered_operator_only`. The
   surface MUST link the class-specific review path.
4. **Blocked cleanup (cleanup attempt blocked by a mix of pinned
   and protected items on a single workspace).**
   `event_class = cleanup_blocked_pinned_or_protected`,
   `bytes_reclaimed_total = 0`,
   `entries_reclaimed_total = 0`,
   `blocked_items_summary[]` includes
   `blocked_evidence_class_specific_review_required`,
   `blocked_authoritative_user_owned_clear_refused`, and
   `blocked_user_pinned_indefinite`. The
   `evidence_disclosure_class` is
   `evidence_class_blocked_class_specific_review_pending`.
5. **Auto-eviction history event (low-disk governor trims the
   knowledge cache and prebuild environment under the degraded
   ladder).** `event_class = automatic_low_disk_governor_eviction`,
   `actor_class = system_low_disk_governor`,
   `actor_framing_class = automatic_system_acted`,
   `ladder_step_applied = trim_knowledge_cache_rebuildable`,
   `triggering_low_disk_banner_ref` non-null,
   `affected_class_rows[]` reports rebuilt-pending knowledge cache
   and prebuild environment with non-zero
   `bytes_reclaimed`,
   `evidence_disclosure_class = evidence_class_not_touched`.
6. **Cleanup blocked under managed-policy throttle (managed-tenant
   install).** `event_class = cleanup_blocked_managed_policy_throttle`,
   `bytes_reclaimed_total = 0`,
   `triggering_low_disk_banner_ref` non-null,
   deployment framing `managed_policy`, the blocked-items summary
   includes the matching `blocked_managed_policy_throttle` reason
   plus an evidence pin row.
7. **Support-export assembly drained.**
   `event_class = support_export_assembly_drained`,
   `actor_class = support_export_assembly`,
   `actor_framing_class = support_export_acted`,
   `evidence_disclosure_class = support_export_assembly_pin_drained`,
   the affected-class row reports zero bytes reclaimed on
   `evidence_support_cache` (the export drained without
   reclaiming evidence) and a follow-up of
   `reissue_support_export_after_case_closure`.

## 14. Adding or changing vocabulary

Adding a value to any vocabulary frozen in this contract
(`pin_kind_class`, `referenced_object_class`, `pin_expiry_class`,
`expires_at_state_class`, `unpin_path_class`, `evidence_export_class`,
`protected_class_visibility_token`, `actor_framing_class`,
`event_class`, `resulting_class_state_class`,
`evidence_disclosure_class`, `blocked_reason_class`,
`follow_up_action_class`, `consumer_surface_class`) is **additive-
minor** and requires:

1. Updating the schema enum in
   `schemas/storage/pin_retention_entry.schema.json` or
   `schemas/storage/cleanup_history_event.schema.json`.
2. Updating this document.
3. Adding or updating a fixture under
   `fixtures/storage/pin_retention_cleanup_cases/` exercising
   the new value.
4. Bumping the corresponding `*_schema_version` integer.

Repurposing an existing value is **breaking** and requires:

1. A new decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecation of the old value, addition of the new value
   through an additive-minor landing, and a translation pass on
   the pin manager, clear-data review sheet, low-disk banner,
   cleanup-history lane, support bundle, admin storage console,
   and CLI consumers across the deprecation window.

Vocabularies that re-export from upstream seeds (storage-class
matrix, authority class, pin-source class, pin-expiry anchor,
storage-posture class, low-disk ladder step, actor class,
deployment profile, mirror-offline state, tenant-org scope) follow
the upstream change rules; this contract follows in the same
change.

## 15. Out of scope at this revision

- Final pin-manager / cleanup-history-lane layout, animation, and
  accessibility wiring. The contract pins the record families;
  the rendering surfaces own their own component contracts.
- Retention scheduling, cleanup execution, admin policy
  evaluation, and the corrupt-cache detector. The cache manager
  (frozen in
  [`docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md))
  and the resource-governor thresholds artifact own those.
- Localization-ready string tables; the contract carries
  reviewable English sentences and the localization layer is
  consumed separately.
- The class-specific review sheet for `evidence_support_cache`
  and `user_owned_recovery_state`. The cache-manifest schema and
  the local-history restore-preview contract freeze those review
  records; pin / retention entries and cleanup-history events
  link them by ref.
