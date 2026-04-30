# Clear-data review sheet, low-disk / quota banner, and protected-class disclosure contract

This document is the **cleanup-and-pressure inspectability contract**
for Aureline's clear-data and disk-pressure UX truth. It freezes one
`clear_data_review_record` and one `low_disk_banner_record` so the
**class-selective clear-data review (selected classes, affected
workspaces, what will be rebuilt versus lost, pinned and policy-
protected items, and recovery / export options) and the low-disk /
quota pressure banner (pressure class, paused work, next eviction
order, protected classes, override / open-inspector affordance, and
deployment-context posture)** project the same record family rather
than per-surface "free up space" copy.

The contract exists so a clear-data review sheet, a low-disk banner
expansion, the cleanup-history lane, the pin manager, the support-
bundle storage section, the admin storage console, and the CLI text
formatter all consume one boundary record family without recomputing
field names; so generic clear-cache copy cannot silently include
authoritative user-owned recovery state, evidence packets, mirrored
or imported packs, or policy-protected admin artifacts; so a low-
disk banner cannot announce "your work is broken" when only
disposable disposable cache is being trimmed; so pressure handling
is honestly typed across local-only, mirrored / offline, and
managed-policy contexts; and so the **lost / rebuilt / retained /
pinned / blocked** state of every selected class is rendered with
typed copy that names the data-loss implication instead of hiding
it behind cache language.

The contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI/UX Spec, or design-system style guide, those sources win
and this document plus its schemas and fixtures update in the same
change. Where the clear-data review sheet, low-disk banner, pin
manager, cleanup-history lane, support bundle, admin console, or
CLI text formatter mints a parallel review sheet, parallel pressure
banner, parallel "what will be removed" projection, or parallel
override action, this contract wins and the surface is non-
conforming.

## Companion artifacts

- [`/schemas/storage/clear_data_review.schema.json`](../../schemas/storage/clear_data_review.schema.json)
  — boundary schema for the `clear_data_review_record`.
- [`/schemas/storage/low_disk_banner.schema.json`](../../schemas/storage/low_disk_banner.schema.json)
  — boundary schema for the `low_disk_banner_record`.
- [`/fixtures/storage/clear_data_low_disk_cases/`](../../fixtures/storage/clear_data_low_disk_cases/)
  — worked YAML cases covering at least the four acceptance
  scenarios (low-disk throttling, clear-cache of rebuildable
  classes, blocked clear of user-owned recovery state, and policy-
  protected evidence class).

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary that is already
frozen upstream; it consumes the frozen sets by name and by value:

- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md),
  [`/artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml),
  [`/artifacts/runtime/low_disk_drills.yaml`](../../artifacts/runtime/low_disk_drills.yaml),
  and [`/schemas/runtime/cache_entry_manifest.schema.json`](../../schemas/runtime/cache_entry_manifest.schema.json)
  — the six frozen `storage_class_id` values, the four
  `authority_class` values, the four `rebuild_cost_class` values,
  the five `gc_policy_class` values, the ten `pin_source_class`
  values, the four `clear_cache_protection_class` values, and the
  eight `low_disk_ladder_step` values. The clear-data review and
  low-disk banner records re-export these names byte-for-byte.
- [`/docs/storage/storage_inspector_contract.md`](./storage_inspector_contract.md)
  and [`/schemas/storage/storage_inspector_card.schema.json`](../../schemas/storage/storage_inspector_card.schema.json),
  [`/schemas/storage/storage_class_breakdown.schema.json`](../../schemas/storage/storage_class_breakdown.schema.json)
  — the parent `storage_inspector_card_record` and the
  `storage_class_breakdown_row_record` family. The clear-data
  review and the low-disk banner cite a parent inspector card by
  ref, re-export the `inspector_scope` block verbatim, and embed
  per-class projection rows whose `class_scope` is value-equal to
  the parent card's `inspector_scope`.
- [`/docs/storage/workspace_storage_detail_contract.md`](./workspace_storage_detail_contract.md)
  and [`/schemas/storage/workspace_storage_detail.schema.json`](../../schemas/storage/workspace_storage_detail.schema.json),
  [`/schemas/storage/rebuild_cost_hint.schema.json`](../../schemas/storage/rebuild_cost_hint.schema.json)
  — the per-entry detail row and the `rebuild_cost_hint_record`.
  The clear-data review preview embeds a per-class
  `rebuild_cost_hint_record` so the lost-or-rebuilt projection
  cannot collapse into a generic "rebuild cost" label, and links
  per-entry detail rows by ref when the user drills into a class.
- [`/docs/ux/deployment_summary_contract.md`](../ux/deployment_summary_contract.md)
  and [`/schemas/deployment/deployment_summary_card.schema.json`](../../schemas/deployment/deployment_summary_card.schema.json)
  — the `deployment_profile`, `mirror_offline_state_class`,
  `tenant_org_scope_class`, and `mirror_offline_artifact_row_record`
  values the deployment-context block re-exports for the local-
  only, mirrored / offline, and managed-policy framings.
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  — `authority_class` vocabulary
  (`user_authored_durable_truth`, `user_owned_recovery_state`,
  `admin_or_control_artifact`, `disposable_derived_cache`).

## Who reads this contract

- **Clear-data review sheet, low-disk banner expansion, pin
  manager, cleanup-history lane, admin storage console, support-
  bundle storage section, and CLI `--clear-data` / `--low-disk`
  text formatter** — to read **one** review-sheet and **one**
  banner record family instead of recomputing fields per surface.
- **Project doctor / attention inbox, status-bar storage cell, and
  About-panel storage excerpt** — to surface the pressure class,
  the paused-work projection, and the protected-class refusals.
- **Reviewers (release, security, accessibility, claim-manifest)** —
  to verify that disposable cache, rebuildable derived state,
  authoritative user-owned recovery state, mirrored / imported
  artifacts, and policy-protected evidence are typed differently
  on every clear-data review preview, that the lost / rebuilt /
  retained / pinned / blocked state vocabulary is enforced, that
  protected-class items remain visible on every clear-data review
  and every low-disk banner before any action, that pressure
  posture is honestly typed for local-only, mirrored / offline,
  and managed-policy contexts, and that override actions stay
  bounded.

## Two questions the contract answers

Any Aureline surface claiming to expose cleanup or pressure truth
MUST answer both questions mechanically, without per-surface copy:

1. **What would this clear actually remove, what would it leave
   behind, and at what rebuild cost?** Which storage classes are
   selected? Which workspaces / profiles / tenants does the action
   touch? For every class on the review, what bytes are
   `lost_after_clear`, what bytes are `rebuilt_after_clear`, what
   bytes are `retained_unchanged`, what bytes are `pinned_excluded`,
   and what bytes are `blocked_protected`? What is the embedded
   rebuild-cost hint (offline-rebuild risk, startup impact,
   provenance continuity) for every rebuildable class? Which
   per-entry detail rows are protected and which path is the only
   admissible clear (class-specific review, refused authoritative,
   refused policy-held)? What recovery / export options are offered
   before the user confirms?
2. **Why did Aureline slow or stop background work, what did it
   trim, and what is protected from being trimmed?** What is the
   pressure class (constrained / degraded / protect-core / quota-
   exceeded / connectivity-driven / managed-policy-driven / not
   pressure)? Which work lanes are paused or queued? Which low-
   disk ladder steps are next, and which classes does each step
   target? Which classes and pin sources are excluded from
   trimming under this banner? What is the deployment-context
   framing (local-only, mirrored / offline, managed-policy)? Which
   inspect-only override or open-inspector action does the user
   have, and is that action bounded to inspect-only navigation?

Generic prose like "free up space", "low disk", "your data is
big", "cache full", or "old data" is forbidden when a more
precise per-class, per-pressure-class, per-deployment-context
figure is knowable. The schema enforces typed vocabulary and
typed sentences; surfaces render those values.

## 1. Scope

This contract freezes:

- One **clear-data review record** (§3) emitted whenever a clear-
  data review sheet, low-disk drill expansion, pin manager, or
  cleanup-history lane needs to project the consequences of a
  class-selective clear. The record carries: `parent_card_id_ref`,
  the `review_scope`, the `selected_class_ids[]`, the
  `affected_workspaces[]`, the per-class `class_projection_rows[]`,
  the protected-class refusals (`protected_class_refusals[]`), the
  `recovery_export_options[]`, the `confirm_clear_action`, the
  `cancel_action`, the `deployment_context`, the
  `consumer_surfaces[]`, the `redaction_class`, the `export_safe`,
  and the `note`.
- One **low-disk / quota banner record** (§4) emitted whenever a
  banner needs to project current pressure posture and what
  Aureline has paused or trimmed. The record carries:
  `parent_card_id_ref`, the `banner_scope`, the
  `pressure_class`, the `pressure_source_class`, the
  `pressure_started_at` and `pressure_observed_at`, the
  `paused_work_lanes[]`, the `next_eviction_steps[]`, the
  `protected_class_visibility[]` and `protected_pin_sources[]`,
  the `override_action`, the `open_inspector_action`, the
  `deployment_context`, the `consumer_surfaces[]`, the
  `redaction_class`, the `export_safe`, and the `note`.
- The **per-class projection row shape** (§5) embedded inside the
  clear-data review record. Each row freezes the **lost /
  rebuilt / retained / pinned / blocked** state of one selected
  storage class on one review scope, the embedded
  `rebuild_cost_hint_record`, and the `class_consequence_state`.
- The **pressure-class vocabulary** (§6) — `not_pressure`,
  `pressure_constrained`, `pressure_degraded`,
  `pressure_protect_core`, `pressure_quota_exceeded`,
  `pressure_connectivity_constrained`,
  `pressure_managed_policy_throttle`, and
  `pressure_unknown_low_privilege_scan` — and the
  `pressure_source_class` vocabulary that names whether the
  pressure originated from a free-disk floor, a per-class quota,
  a per-tenant quota, an admin policy, a connectivity event, or
  the device governor.
- The **deployment-context block** (§7) re-exporting the
  `deployment_profile`, `mirror_offline_state_class`, and
  `tenant_org_scope_class` so a banner or review sheet on an
  air-gapped install, an enterprise managed tenant, or an
  individual local install renders with the right framing.
- The **typed copy vocabulary** (§8) the surface consumes for
  the lost / rebuilt / retained / pinned / blocked sentences so
  generic cache language cannot collapse data-loss implications.
- The **cross-record invariants** (§9) so authoritative state,
  policy-protected evidence, and mirrored / imported artifacts
  cannot be selected for a generic clear, every override and
  open-inspector action stays inspect-only or scope-local-only,
  and every banner that names paused work cites the matching
  protected-class refusal so the user can see what was preserved.

## 2. Out of scope

- The disk scanner, the background quota accountant, the pin
  reference counter, and the cache-manager GC engine. The cache
  manager (frozen in
  [`docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md))
  remains the source of truth for cache-entry admission, low-disk
  ordering, and pin-ref bookkeeping. This contract pins the
  inspectable record family for the review sheet and the banner;
  it does not implement garbage collection, quota enforcement, or
  any cleanup daemon.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and the shell-interaction-safety contract own the
  strings.
- Pixel-perfect chart and bar layout, the on-disk byte format of
  any store, and the live disk scanner / quota accountant.
- The class-specific review sheet for `evidence_support_cache`
  and `user_owned_recovery_state`. The cache-manifest schema and
  the local-history restore-preview contract freeze those review
  records; the clear-data review record links them by ref via the
  per-class projection row.
- The post-action attribution (cleanup-history) record family.
  That is frozen separately; the clear-data review record cites
  the cleanup-history lane by consumer-surface enum.

## 3. The clear-data review record

A clear-data review record is one structured projection of a
class-selective clear's consequences against one inspector scope.
The clear-data review sheet, low-disk drill expansion, pin
manager, cleanup-history lane, admin storage console, support
bundle, and CLI text formatter each render the same record
without changing field names.

### 3.1 Required fields

- `record_kind = clear_data_review_record`.
- `clear_data_review_schema_version = 1`.
- `review_id` — opaque, stable, safe to log and export.
- `emitted_at` — RFC 3339 UTC timestamp from a monotonic clock.
- `parent_card_id_ref` — opaque ref into the parent
  `storage_inspector_card_record` whose scope this review
  resolves against.
- `review_scope` — see §3.2. Value-equal to the parent inspector
  card's `inspector_scope`.
- `selected_class_ids[]` — non-empty, unique, ordered set of
  `storage_class_id` values the user selected for clear. The
  schema forbids `user_owned_recovery_state` and
  `evidence_support_cache` from appearing in the *selected*
  ids of a generic clear; those classes flow through the
  class-specific review path and are surfaced in
  `protected_class_refusals[]` instead.
- `affected_workspaces[]` — non-empty array of affected-workspace
  rows, each citing a `scope_class`, `scope_ref`, and
  `scope_label` for one affected workspace, profile, tenant, or
  slice. The schema forbids an empty list; a clear with no
  resolvable affected workspace (e.g. a device-total clear) MUST
  carry one row whose `scope_class` is `device_total` and whose
  `scope_ref` is `null`.
- `class_projection_rows[]` — non-empty array of per-class
  projection rows (§5). One row per `selected_class_ids[]` value
  plus, for `protected_class_refusals[]` whose refusal class is
  `class_specific_review_required` or
  `policy_held_clear_refused`, one row whose
  `class_consequence_state` is `blocked_protected`.
- `protected_class_refusals[]` — non-empty array of refusal rows
  (§3.3) for every protected class on the review scope (always
  includes `evidence_support_cache` and
  `user_owned_recovery_state` if the scope covers them).
- `recovery_export_options[]` — array of recovery / export
  affordance rows (§3.4). Required non-empty when any
  `class_projection_rows[]` row's `class_consequence_state` is
  `lost_after_clear` or `blocked_protected_export_required`.
- `confirm_clear_action` — see §3.5.
- `cancel_action` — see §3.5.
- `deployment_context` — see §7.
- `consumer_surfaces[]` — closed set of surfaces consuming the
  record (§3.6).
- `redaction_class` — one of `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only`. Exported records MUST NOT widen
  redaction beyond `operator_only_restricted`.
- `export_safe` — boolean.
- `note` — short reviewable sentence in product terms. No raw
  payload bytes, no raw paths, no raw URLs.

Optional fields: `linked_low_disk_banner_ref`,
`linked_pin_manager_route_ref`,
`linked_class_specific_review_refs[]`,
`linked_cleanup_history_lane_ref`,
`linked_deployment_summary_card_ref`.

### 3.2 `review_scope`

Every record carries a `review_scope` block whose `scope_class`,
`scope_ref`, and `scope_label` are value-equal to the parent
inspector card's `inspector_scope`. The schema enforces this
equivalence so a workspace-only review cannot bleed into another
workspace's bytes.

### 3.3 `protected_class_refusals[]` (closed)

Each entry is one refusal row carrying:

- `refused_class_id` — one of the six `storage_class_id` values.
- `refusal_class` — closed set:
  - `class_specific_review_required` — required for every
    `evidence_support_cache` row on the review scope. The
    surface MUST link the class-specific review path verbatim.
  - `authoritative_user_owned_clear_refused` — required for
    every `user_owned_recovery_state` row on the review scope.
    Generic clear is refused outright; the surface MUST link
    the class-specific review path that offers
    export-before-clear.
  - `policy_held_clear_refused` — required when an admin pin,
    a tenant pin, or a retention window holds bytes inside the
    refused class such that the class-specific review path is
    also gated.
  - `mirror_or_offline_pin_excluded` — required when a
    `mirror_snapshot_segment`, `offline_bundle_segment`, or
    other mirrored / imported / vendor-signed / customer-signed
    artifact would otherwise be eligible for trimming but is
    pinned by a release / offline-bundle / certified-template /
    policy-bundle ref.
- `linked_class_specific_review_ref` — opaque ref. Required
  non-null when `refusal_class` is
  `class_specific_review_required` or
  `authoritative_user_owned_clear_refused`; required null when
  `refusal_class` is `mirror_or_offline_pin_excluded`.
- `linked_pin_manager_route_ref` — opaque ref. Required non-null
  when `refusal_class` is `mirror_or_offline_pin_excluded` or
  `policy_held_clear_refused`; required null otherwise.
- `refusal_label` — short reviewable label naming the refusal in
  product terms (e.g. "Local recovery journal — generic clear
  refused"). Generic "Protected" or "Cannot remove" copy is
  non-conforming when a more precise label is knowable.
- `notes` — optional short reviewable sentence.

### 3.4 `recovery_export_options[]` (closed)

Each entry carries:

- `option_class` — closed set:
  - `export_required_before_clear_offered` — required when at
    least one `class_projection_rows[]` row on the review has
    `class_consequence_state = blocked_protected_export_required`
    or carries an embedded
    `rebuild_cost_hint.rebuild_cost_class = authoritative_no_rebuild`.
  - `export_offered_metadata_safe` — admitted on review records
    whose `redaction_class` is `metadata_safe_default`.
  - `export_offered_operator_only` — admitted on operator-only
    records.
  - `recovery_review_offered_class_specific` — required when at
    least one refusal is `class_specific_review_required` or
    `authoritative_user_owned_clear_refused`; the option links
    the class-specific review path.
  - `pin_release_review_offered` — admitted when a pinned
    excluded class is named in
    `protected_class_refusals[]` whose refusal class is
    `mirror_or_offline_pin_excluded`. The option links the pin
    manager.
  - `no_recovery_required_disposable_only` — admitted only when
    every `class_projection_rows[]` row's
    `class_consequence_state` is `rebuilt_after_clear` or
    `retained_unchanged` and no refusal carries
    `class_specific_review_required` or
    `authoritative_user_owned_clear_refused`.
- `option_label` — short reviewable label.
- `linked_target_ref` — opaque ref into the export target,
  class-specific review, or pin manager route. Required non-null
  except when `option_class` is
  `no_recovery_required_disposable_only`.
- `notes` — optional short reviewable sentence.

### 3.5 `confirm_clear_action` and `cancel_action`

The review record MUST carry exactly one confirm action and one
cancel action.

#### `confirm_clear_action`

- `action_id` — opaque id.
- `label` — short reviewable label (e.g. "Clear selected caches
  for this workspace"). Generic "Confirm" or "OK" copy is non-
  conforming when a more precise label is knowable.
- `target_route_ref` — opaque ref to the cleanup target.
- `scope_class` — `scope_local_only`. Confirm cannot reach
  beyond the local device.
- `authority_class` — `user_local_authority`.
- `consent_class` — `explicit_user_confirm_required`. Confirm
  is the consent gate; it is never silent or auto-fired.
- `side_effects[]` — closed set re-exported from the destructive-
  action shape:
  - `removes_disposable_cache` — admissible on every confirm.
  - `removes_rebuildable_derived_state` — required when at
    least one `class_projection_rows[]` row's
    `class_consequence_state` is `rebuilt_after_clear`.
  - `pauses_pinned_excluded_items` — required when at least
    one refusal is `mirror_or_offline_pin_excluded`.
  - `preserves_authoritative_state` — required on every
    confirm; the schema forbids omitting this side-effect.
- `preserves_evidence_context` — `true`.
- `revalidation_on_confirm` — one of
  `none_already_fresh`, `snapshot_recompute_on_confirm`. The
  surface revalidates pin refs and protected-class refusals
  before firing.
- `irreversible_warning_required` — `true` when at least one
  `class_projection_rows[]` row has
  `class_consequence_state = lost_after_clear`; otherwise the
  schema admits `false`.
- `modal_prohibited` — `false`. The confirm action MAY raise a
  modal (clear-data review is the one place where a modal is
  admissible because the action is destructive); but the modal
  MUST render the typed projection rows, not a generic
  warning.

#### `cancel_action`

- `action_id` — opaque id.
- `label` — short reviewable label (e.g. "Cancel and keep
  everything").
- `scope_class` — `scope_local_only`.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `modal_prohibited` — `true`.

### 3.6 `consumer_surface_class` (closed)

Closed set of surfaces consuming the record.

- `clear_data_review` — the class-selective review sheet.
- `low_disk_banner_drill` — the low-disk banner expansion that
  drills into a clear preview.
- `pin_manager` — pin / retention manager.
- `cleanup_history_lane` — post-action attribution lane.
- `admin_storage_console` — admin / tenant storage console.
- `support_packet_export` — support-bundle storage section.
- `cli_text_formatter` — CLI text formatter render.

## 4. The low-disk / quota banner record

A low-disk / quota banner record is one structured projection of
current pressure posture. The settings inspector banner, the
status-bar storage cell expansion, the cleanup-history lane
header, the admin storage console, the support bundle, and the
CLI text formatter each render the same record without changing
field names.

### 4.1 Required fields

- `record_kind = low_disk_banner_record`.
- `low_disk_banner_schema_version = 1`.
- `banner_id` — opaque, stable, safe to log and export.
- `emitted_at` — RFC 3339 UTC timestamp from a monotonic clock.
- `parent_card_id_ref` — opaque ref into the parent
  `storage_inspector_card_record` whose scope this banner
  resolves against. Required non-null because a banner without
  a fresh inspector card cannot drive cleanup action (see §9.3).
- `banner_scope` — see §4.2. Value-equal to the parent inspector
  card's `inspector_scope`.
- `pressure_class` — see §6.1.
- `pressure_source_class` — see §6.2.
- `pressure_started_at` — RFC 3339 UTC timestamp or `null`.
  Required non-null whenever `pressure_class` is anything other
  than `not_pressure` or `pressure_unknown_low_privilege_scan`;
  required null otherwise.
- `pressure_observed_at` — RFC 3339 UTC timestamp from a
  monotonic clock. Always required.
- `paused_work_lanes[]` — array of paused-work-lane rows (§4.3).
  Required non-empty whenever `pressure_class` is anything other
  than `not_pressure`; required empty when `pressure_class` is
  `not_pressure`.
- `next_eviction_steps[]` — ordered array of
  `low_disk_ladder_step` re-export values (§4.4). Required
  ordered consistently with the cache-manager ladder; the schema
  enforces that `expire_unpinned_evidence_past_retention` and
  `user_owned_recovery_state_only_under_explicit_review` MUST
  NOT appear in any banner emitted under
  `pressure_managed_policy_throttle` or
  `pressure_connectivity_constrained` (those rungs are reserved
  for the explicit-review pathway, not for connectivity- or
  policy-throttle).
- `protected_class_visibility[]` — non-empty array of protected-
  class visibility tokens (§4.5). Always includes
  `evidence_support_cache_visible` and
  `user_owned_recovery_state_visible` whenever the banner scope
  covers either class.
- `protected_pin_sources[]` — array of `pin_source_class` values
  whose pinned entries this banner protects from trimming. The
  schema enforces that
  `case_reference_ref`, `support_export_assembly_ref`,
  `release_artifact_graph_ref`, `offline_bundle_ref`,
  `certified_archetype_or_template_ref`,
  `policy_bundle_last_known_good_ref`, and
  `retention_window_ref` are admissible; the
  `explicit_user_pin` value is admissible only on banners whose
  `pressure_class` is not `pressure_protect_core` (the protect-
  core ladder may exclude an explicit user pin only under the
  user_owned_recovery_state explicit-review pathway, never as a
  silent override).
- `override_action` — see §4.6.
- `open_inspector_action` — see §4.7.
- `deployment_context` — see §7.
- `consumer_surfaces[]` — closed set of surfaces consuming the
  banner (§4.8).
- `redaction_class` — one of the four standard classes.
  Exported banners MUST NOT widen redaction beyond
  `operator_only_restricted`.
- `export_safe` — boolean.
- `note` — short reviewable sentence.

Optional fields: `linked_clear_data_review_ref`,
`linked_pin_manager_route_ref`,
`linked_low_disk_drill_ref`,
`linked_deployment_summary_card_ref`,
`linked_continuity_packet_ref`.

### 4.2 `banner_scope`

Every banner carries a `banner_scope` block whose `scope_class`,
`scope_ref`, and `scope_label` are value-equal to the parent
inspector card's `inspector_scope`. The schema enforces this
equivalence so a workspace-only banner cannot announce pressure
on another workspace.

### 4.3 `paused_work_lanes[]`

Each entry carries:

- `lane_class` — closed set re-exported from the cache-manager
  paused-work projection: `provider_refresh`,
  `ai_context_expansion`, `extension_timer`, `telemetry_forward`,
  `mirror_pull`, `pack_refresh`, `prefetch`,
  `speculative_index_warmup`, `support_export_assembly`. Adding
  a value is additive-minor.
- `lane_state` — closed set:
  - `paused_under_pressure` — the lane is paused for the
    duration of the pressure window.
  - `queued_for_replay` — the lane is queued and will replay
    once pressure clears.
  - `throttled` — the lane is rate-limited but not stopped.
- `lane_label` — short reviewable label.
- `notes` — optional short reviewable sentence.

### 4.4 `next_eviction_steps[]`

Ordered, unique array of
[`low_disk_ladder_step_vocabulary`](../../artifacts/runtime/storage_classes.yaml)
values:

- `stop_speculative_fetch_and_prefetch`
- `pause_managed_replication_and_pack_refresh`
- `trim_interactive_hot_cache`
- `trim_knowledge_cache_rebuildable`
- `trim_artifact_cache_unpinned`
- `trim_prebuild_environment_unpinned`
- `expire_unpinned_evidence_past_retention`
- `user_owned_recovery_state_only_under_explicit_review`

The order MUST follow the cache-manager ladder; surfaces consume
the array verbatim.

### 4.5 `protected_class_visibility[]` (closed)

Closed set the banner MUST acknowledge as **visible regardless of
pressure**:

- `evidence_support_cache_visible` — surfaces a typed
  preservation note for evidence even when the pressure class
  is `pressure_protect_core`.
- `user_owned_recovery_state_visible` — surfaces a typed
  preservation note for user-owned recovery state. The schema
  forbids any banner from including
  `user_owned_recovery_state_only_under_explicit_review` in
  `next_eviction_steps[]` unless the banner cites a
  `linked_clear_data_review_ref` whose review record carries
  the explicit-review pathway. Silent inclusion is non-
  conforming.

### 4.6 `override_action`

Every banner MUST carry exactly one override action (the
"open clear-data review" or "open pin manager" affordance) so the
user can take typed action from the banner without bypassing the
review path. Required fields:

- `action_id` — opaque id.
- `label` — short reviewable label (e.g. "Open clear-data review
  for this workspace").
- `target_route_ref` — opaque ref to the review or pin-manager
  route.
- `override_class` — closed set:
  - `open_clear_data_review` — opens the class-selective review
    sheet for this banner's scope.
  - `open_pin_manager` — opens the pin manager for excluded
    pinned items.
  - `open_class_specific_review` — opens the class-specific
    review path for evidence or recovery state. Admissible only
    when at least one
    `protected_class_visibility[]` token names a protected
    class whose generic clear is refused.
  - `not_offered_managed_policy_throttle` — the override is
    refused under managed-policy throttle. Admissible only when
    `pressure_class` is `pressure_managed_policy_throttle`.
- `scope_class` — `scope_local_only`.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]` for
  every `override_class` except
  `not_offered_managed_policy_throttle`, which carries an
  empty `side_effects` array because there is no admissible
  action.
- `preserves_evidence_context` — `true`.
- `modal_prohibited` — `true`.

### 4.7 `open_inspector_action`

Every banner MUST carry exactly one open-inspector action that
opens the storage inspector for the banner's scope. The shape
mirrors the storage-inspector card's `open_details_action` (§3.4
of the storage-inspector contract):

- `action_id` — opaque id.
- `label` — short reviewable label.
- `target_route_ref` — opaque ref to the inspector route.
- `scope_class` — `scope_local_only`.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `revalidation_on_open` — one of `none_already_fresh`,
  `snapshot_open_read_only`.
- `modal_prohibited` — `true`.

### 4.8 `consumer_surface_class` (closed)

Closed set of surfaces consuming the banner.

- `low_disk_banner` — the inspector banner / settings page
  banner.
- `status_bar_storage_cell_expansion` — the status-bar storage
  cell.
- `clear_data_review` — clear-data review sheet header excerpt.
- `cleanup_history_lane` — post-action attribution lane.
- `pin_manager` — pin / retention manager.
- `admin_storage_console` — admin / tenant storage console.
- `support_packet_export` — support-bundle storage section.
- `about_panel_storage_excerpt` — About-panel storage excerpt.
- `cli_text_formatter` — CLI text formatter render.

## 5. The per-class projection row shape

Every clear-data review record carries a non-empty array of per-
class projection rows. Each row is one structured projection of
one selected class on one review scope.

### 5.1 Required fields

- `class_projection_row_id` — opaque, stable.
- `storage_class_id` — re-export of the runtime vocabulary.
- `class_consequence_state` — see §5.2.
- `lost_after_clear_bytes` — non-negative integer. Bytes that
  would be irretrievably lost (no rebuild possible) by this
  clear. The schema enforces
  `lost_after_clear_bytes = 0` whenever `storage_class_id` is
  not `evidence_support_cache` or `user_owned_recovery_state`.
- `rebuilt_after_clear_bytes` — non-negative integer. Bytes
  that would be removed but rebuildable from on-device or
  upstream-trusted source.
- `retained_unchanged_bytes` — non-negative integer. Bytes the
  clear leaves in place because they are protected, pinned, or
  outside the selection.
- `pinned_excluded_bytes` — non-negative integer. Bytes excluded
  from this clear because of at least one pin ref.
- `blocked_protected_bytes` — non-negative integer. Bytes
  blocked from this clear because the class is protected
  outright (evidence, recovery state, mirrored / imported pack,
  policy-protected admin artifact).
- `pin_source_breakdown[]` — array of pin-source breakdown rows
  for `pinned_excluded_bytes`. Required non-empty when
  `pinned_excluded_bytes > 0`; required empty otherwise.
- `rebuild_cost_hint` — embedded `rebuild_cost_hint_record`
  (re-exported from `/schemas/storage/rebuild_cost_hint.schema.json`,
  inlined here for self-containment) describing the rebuild
  cost of this class. Required on every row.
- `affected_entry_refs[]` — array of opaque `entry_ref` values
  pointing at `workspace_storage_detail_row_record` rows. The
  schema enforces a non-empty list when
  `lost_after_clear_bytes + rebuilt_after_clear_bytes +
  pinned_excluded_bytes + blocked_protected_bytes > 0`; empty
  otherwise.
- `class_label` — short reviewable label naming the class in
  product terms.
- `consequence_sentence` — short reviewable sentence drawing
  from the typed copy vocabulary (§8). Generic "rebuild
  required" or "free up space" copy is non-conforming.
- `linked_class_specific_review_ref` — opaque ref. Required
  non-null when `storage_class_id` is `evidence_support_cache`
  or `user_owned_recovery_state` (which by §5.4 forces
  `class_consequence_state = blocked_protected` or
  `blocked_protected_export_required`).
- `notes` — optional short reviewable sentence.

### 5.2 `class_consequence_state` (closed)

Closed five-value vocabulary:

- `lost_after_clear` — bytes are gone after the clear and
  cannot be rebuilt. Admissible only when
  `lost_after_clear_bytes > 0`. Forces an
  `irreversible_warning_required = true` on the parent
  `confirm_clear_action`.
- `rebuilt_after_clear` — bytes are removed but rebuildable.
  Admissible only when `rebuilt_after_clear_bytes > 0`. The
  embedded rebuild-cost hint is the surface's authority on the
  rebuild cost.
- `retained_unchanged` — bytes are not touched by this clear
  (the class is selected but every entry is protected or
  pinned, or the class has zero bytes on the scope).
- `pinned_excluded` — bytes are excluded because of pin refs.
  Admissible only when `pinned_excluded_bytes > 0` and the row
  carries a non-empty `pin_source_breakdown[]`.
- `blocked_protected` — bytes are protected outright (evidence,
  recovery state, mirrored / imported pack, policy-protected
  admin artifact). The row's `storage_class_id` MUST be
  `evidence_support_cache` or `user_owned_recovery_state`, OR
  the row's `affected_entry_refs[]` MUST resolve to detail rows
  whose `mirror_or_import_origin_class` is `mirrored_copy`,
  `offline_bundle_local`, `vendor_signed_offline_local`,
  `customer_signed_mirror_local`, or
  `policy_protected_admin_artifact`.
- `blocked_protected_export_required` — variant of
  `blocked_protected` for `evidence_support_cache` rows whose
  embedded `rebuild_cost_hint.rebuild_cost_class` is
  `authoritative_no_rebuild`. Forces a
  `recovery_export_options[]` entry whose `option_class` is
  `export_required_before_clear_offered`.

### 5.3 `pin_source_breakdown[]`

Each entry carries:

- `pin_source_class` — re-export of the runtime
  `pin_source_class` vocabulary.
- `pinned_bytes` — non-negative integer.
- `pin_ref_summary_id` — opaque, stable.
- `pin_label` — optional short reviewable label.
- `notes` — optional short reviewable sentence.

The schema enforces
`sum(pin_source_breakdown[].pinned_bytes) == pinned_excluded_bytes`
on every row.

### 5.4 Schema-enforced storage-class pairings

- `user_owned_recovery_state` MUST NOT appear in
  `selected_class_ids[]`. Surfaces that include it in a generic
  selection are non-conforming; the class flows through
  `protected_class_refusals[]` instead.
- `evidence_support_cache` MUST NOT appear in
  `selected_class_ids[]`. Surfaces that include it in a generic
  selection are non-conforming.
- A `class_projection_rows[]` row whose `storage_class_id` is
  `user_owned_recovery_state` MUST set
  `class_consequence_state = blocked_protected` and carry
  `lost_after_clear_bytes = 0`,
  `rebuilt_after_clear_bytes = 0`, and a non-null
  `linked_class_specific_review_ref`. The row exists to
  surface the refusal alongside the selected classes; it is
  the projection counterpart of the
  `authoritative_user_owned_clear_refused` refusal.
- A `class_projection_rows[]` row whose `storage_class_id` is
  `evidence_support_cache` MUST set
  `class_consequence_state ∈
  {blocked_protected, blocked_protected_export_required}`.
  `blocked_protected_export_required` is required when the
  embedded `rebuild_cost_hint.rebuild_cost_class` is
  `authoritative_no_rebuild`.

## 6. Pressure-class and pressure-source vocabularies

### 6.1 `pressure_class` (closed)

Closed eight-value vocabulary:

- `not_pressure` — no pressure. Banner is rendered only as a
  "healthy" state if surfaces choose to show it; the
  `paused_work_lanes[]` array MUST be empty.
- `pressure_constrained` — first low-disk floor breach
  (free-disk constrained). The banner pauses speculative work.
- `pressure_degraded` — second low-disk floor breach. The
  banner trims disposable cache and rebuildable derived state.
- `pressure_protect_core` — third low-disk floor breach. The
  banner surfaces the strict ladder; only authoritative state
  remains untouched.
- `pressure_quota_exceeded` — a per-workspace, per-class,
  per-tenant, or policy-bound evidence quota has been exceeded.
  The banner surfaces the quota source.
- `pressure_connectivity_constrained` — connectivity is
  constrained or the device is offline; pack refresh and mirror
  pull are paused. The banner surfaces the connectivity
  framing.
- `pressure_managed_policy_throttle` — a managed-tenant policy
  is throttling cleanup or pinning protected classes; the
  override action MUST be
  `not_offered_managed_policy_throttle`.
- `pressure_unknown_low_privilege_scan` — the underlying scan
  ran at low privilege and could not determine pressure; the
  banner surfaces a consent affordance instead of a typed
  pressure class.

### 6.2 `pressure_source_class` (closed)

Closed seven-value vocabulary:

- `free_disk_floor_breach` — pairs with `pressure_constrained`,
  `pressure_degraded`, or `pressure_protect_core`.
- `per_class_quota_exceeded` — pairs with
  `pressure_quota_exceeded`.
- `per_workspace_quota_exceeded` — pairs with
  `pressure_quota_exceeded`.
- `per_tenant_quota_exceeded` — pairs with
  `pressure_quota_exceeded` and forces
  `tenant_org_scope_class ∈ {customer_tenant, shared_multi_tenant}`
  on the deployment-context block.
- `policy_bound_evidence_quota_exceeded` — pairs with
  `pressure_quota_exceeded` and forces
  `pressure_class = pressure_quota_exceeded`. Re-exports the
  `policy_bound_evidence_quota` quota basis from the storage-
  inspector contract.
- `connectivity_event_observed` — pairs with
  `pressure_connectivity_constrained`.
- `managed_policy_throttle_active` — pairs with
  `pressure_managed_policy_throttle`.

The schema enforces these pairings.

## 7. The deployment-context block

Every clear-data review record and every low-disk banner record
carries a `deployment_context` block. Required fields:

- `deployment_profile` — re-export of `individual_local`,
  `self_hosted`, `enterprise_online`, `air_gapped`,
  `managed_cloud`.
- `mirror_offline_state_class` — re-export of `online_live_allowed`,
  `online_mirror_only`, `offline_grace_preserved`,
  `offline_air_gapped`, `deny_all_enforced`,
  `network_disabled_by_user`, `network_degraded_heuristic`,
  `not_applicable`.
- `tenant_org_scope_class` — re-export of `single_user_local`,
  `customer_tenant`, `shared_multi_tenant`,
  `tenant_boundary_recheck_required`, `not_applicable`.
- `framing_class` — closed set summarising the framing the
  banner / review sheet renders:
  - `local_only_individual` — `deployment_profile =
    individual_local` and `mirror_offline_state_class =
    not_applicable`. The banner / review sheet renders local-
    only language; no managed policy or mirror posture is
    surfaced.
  - `mirrored_or_offline` — `mirror_offline_state_class ∈
    {online_mirror_only, offline_grace_preserved,
    offline_air_gapped, network_disabled_by_user,
    network_degraded_heuristic}`. The banner / review sheet
    surfaces mirror / offline-bundle dependency.
  - `managed_policy` — `deployment_profile ∈ {enterprise_online,
    managed_cloud, self_hosted}` and `tenant_org_scope_class ∈
    {customer_tenant, shared_multi_tenant}`. The banner /
    review sheet surfaces tenant-policy posture.
  - `managed_policy_air_gapped` — air-gapped install with a
    managed policy. The banner / review sheet surfaces both
    the offline framing and the tenant-policy posture.

The schema enforces every pairing in §7 between
`framing_class`, `deployment_profile`, `mirror_offline_state_class`,
and `tenant_org_scope_class`.

## 8. Typed copy vocabulary

The `consequence_sentence` on every per-class projection row and
the `note` on every banner / review record draw from a typed
sentence vocabulary so generic cache language cannot collapse
data-loss implications.

### 8.1 `class_consequence_state` to sentence-class binding

The `consequence_sentence` field is a short reviewable sentence
(see schema). The schema does not freeze the string itself, but
freezes the `consequence_state_label_class` field that pairs
with the sentence:

- `consequence_state_label_class` — closed set:
  - `label_lost_no_rebuild` — required when
    `class_consequence_state = lost_after_clear`. Surfaces use
    "{class}: gone after this clear; cannot be rebuilt."
  - `label_rebuilt_with_cost` — required when
    `class_consequence_state = rebuilt_after_clear`. Surfaces
    use the embedded rebuild-cost hint to render typed
    rebuild copy ("Rebuild needs {inputs}; first {startup_impact}").
  - `label_retained_untouched` — required when
    `class_consequence_state = retained_unchanged`.
  - `label_pinned_excluded_named` — required when
    `class_consequence_state = pinned_excluded`. Surfaces name
    every pin source in product terms.
  - `label_blocked_protected_class_specific_review` — required
    when `class_consequence_state = blocked_protected` and
    `storage_class_id ∈ {evidence_support_cache,
    user_owned_recovery_state}`.
  - `label_blocked_protected_export_required` — required when
    `class_consequence_state = blocked_protected_export_required`.
  - `label_blocked_protected_mirror_or_offline_pack` — required
    when `class_consequence_state = blocked_protected` and
    `affected_entry_refs[]` resolves to mirror / offline-bundle
    rows.

### 8.2 Banner copy vocabulary

The banner's `note` field is short reviewable copy. The schema
freezes the `banner_copy_class` field:

- `banner_copy_class` — closed set:
  - `paused_speculative_only_disposable_safe` — admissible only
    when `pressure_class = pressure_constrained` and every
    `paused_work_lanes[]` row is `paused_under_pressure` or
    `throttled` and no eviction step beyond
    `stop_speculative_fetch_and_prefetch` is in
    `next_eviction_steps[]`.
  - `trimming_disposable_and_rebuildable` — admissible only
    when `pressure_class ∈ {pressure_degraded,
    pressure_protect_core}`.
  - `quota_exceeded_named_source` — admissible only when
    `pressure_class = pressure_quota_exceeded`. Surfaces name
    the quota source class in product terms.
  - `connectivity_constrained_pack_refresh_paused` —
    admissible only when `pressure_class =
    pressure_connectivity_constrained`.
  - `managed_policy_throttle_no_override` — admissible only
    when `pressure_class = pressure_managed_policy_throttle`.
  - `unknown_low_privilege_scan_consent_offered` — admissible
    only when `pressure_class =
    pressure_unknown_low_privilege_scan`.

The schema forbids "your work is broken" language by enforcing
that every banner whose `pressure_class` is anything other than
`pressure_protect_core` MUST NOT include in
`paused_work_lanes[]` more than the lanes admissible under that
pressure class (see §9.2).

## 9. Cross-record invariants

The schema enforces these invariants mechanically. A surface
that violates any of them is non-conforming.

1. **Authoritative and policy-held state cannot be selected for
   generic clear.** `selected_class_ids[]` MUST NOT contain
   `user_owned_recovery_state` or `evidence_support_cache`. The
   classes flow through `protected_class_refusals[]` and the
   blocked-protected projection rows instead. A surface that
   includes either class in `selected_class_ids[]` is non-
   conforming.
2. **Banners do not over-claim impact.** A banner whose
   `pressure_class` is `not_pressure` MUST carry an empty
   `paused_work_lanes[]` array. A banner whose `pressure_class`
   is `pressure_constrained` MUST NOT include any
   `next_eviction_steps[]` step beyond
   `stop_speculative_fetch_and_prefetch` and
   `pause_managed_replication_and_pack_refresh`. A banner whose
   `pressure_class` is `pressure_connectivity_constrained` or
   `pressure_managed_policy_throttle` MUST NOT include
   `expire_unpinned_evidence_past_retention` or
   `user_owned_recovery_state_only_under_explicit_review` in
   `next_eviction_steps[]`.
3. **Stale banners do not drive cleanup.** A banner whose
   parent inspector card's `scan_posture.scan_freshness_class`
   is `scan_past_extended_window` MUST NOT include
   `clear_data_review` or `low_disk_banner` in
   `consumer_surfaces[]`. The override action MUST be
   `open_pin_manager` or `not_offered_managed_policy_throttle`,
   never `open_clear_data_review`. (The schema enforces
   parent-card validation at the validation harness level via
   the `parent_card_id_ref` field; this rule is documented here
   for surface conformance.)
4. **Card total equals projection sum.** For every row in
   `class_projection_rows[]`,
   `lost_after_clear_bytes + rebuilt_after_clear_bytes +
   retained_unchanged_bytes + pinned_excluded_bytes +
   blocked_protected_bytes` MUST equal the parent breakdown
   row's `class_used_bytes` for that class (subject to the
   surface's snapshot-recompute on confirm). The schema does
   not cross-check arithmetic against the parent record (those
   are separate records); the pairing is checked by the
   contract-validation harness.
5. **Mirrored / imported / policy-protected entries cannot be
   silently trimmed.** A `class_projection_rows[]` row whose
   `affected_entry_refs[]` includes an entry whose
   `mirror_or_import_origin_class` is `mirrored_copy`,
   `offline_bundle_local`, `vendor_signed_offline_local`,
   `customer_signed_mirror_local`, or
   `policy_protected_admin_artifact` MUST set
   `class_consequence_state` to `pinned_excluded` or
   `blocked_protected`, never `rebuilt_after_clear` or
   `lost_after_clear`. The matching refusal row MUST appear in
   `protected_class_refusals[]` with refusal class
   `mirror_or_offline_pin_excluded` or
   `policy_held_clear_refused`.
6. **Pinned bytes name a source.** A `class_projection_rows[]`
   row whose `pinned_excluded_bytes > 0` MUST carry a non-empty
   `pin_source_breakdown[]` whose entries' `pinned_bytes` sum
   to `pinned_excluded_bytes`. Pinned bytes without a source
   breakdown are non-conforming.
7. **Inspect-only and override actions stay bounded.** The
   `cancel_action`, `override_action`, and
   `open_inspector_action` MUST declare exactly
   `["no_side_effect_inspect_only"]` or, for
   `not_offered_managed_policy_throttle`, an empty array. The
   `confirm_clear_action` MUST always carry
   `preserves_authoritative_state` in `side_effects` so the
   confirm cannot widen its blast radius beyond disposable /
   rebuildable / pinned classes.
8. **Lost bytes force the irreversible warning.** A
   `class_projection_rows[]` row with
   `class_consequence_state = lost_after_clear` MUST force
   `irreversible_warning_required = true` on the parent
   `confirm_clear_action`. The schema enforces this paired
   condition.
9. **Deployment-context framing pairs with profile.** The
   `framing_class` value MUST resolve consistently with the
   `deployment_profile` and `mirror_offline_state_class`
   values (see §7). A surface that mixes
   `framing_class = local_only_individual` with a
   `deployment_profile` other than `individual_local` is non-
   conforming.
10. **Protected-class visibility is mandatory on broad scopes.**
    A clear-data review record or a low-disk banner record
    whose `review_scope` / `banner_scope` `scope_class` is
    `device_total`, `workspace_only`, or `tenant_only` MUST
    list both `evidence_support_cache_visible` and
    `user_owned_recovery_state_visible` in
    `protected_class_visibility[]` (banner) or in
    `protected_class_refusals[]` via the matching refusal class
    (review).
11. **Export-before-clear forces a recovery option.** A
    `class_projection_rows[]` row with
    `class_consequence_state = blocked_protected_export_required`
    MUST be paired with a `recovery_export_options[]` entry
    whose `option_class` is
    `export_required_before_clear_offered` and whose
    `linked_target_ref` is non-null.
12. **Export discipline preserves redaction.** Records exported
    under `export_safe = true` MUST keep `redaction_class ≤
    operator_only_restricted`. Exports that widen redaction to
    `internal_support_restricted` or `signing_evidence_only`
    are non-conforming.
13. **Banner without a fresh inspector card is refused.**
    `parent_card_id_ref` is required non-null on every banner.
    A banner that cannot cite a parent card cannot drive
    cleanup action and cannot be rendered.

## 10. Local-only, mirrored / offline, and managed-policy contexts

The acceptance criterion *pressure and cleanup behavior appears
in local-only, mirrored / offline, and managed-policy contexts*
resolves through:

- The `deployment_context` block on every clear-data review
  record and every low-disk banner record (§7).
- The `framing_class` value pinning the framing the surface
  renders.
- The §6 pressure-source vocabulary, which names whether a
  per-tenant quota or a managed policy is the pressure source.
- The §9.2 invariant forbidding evidence-expiry and
  recovery-state-explicit-review steps on banners under
  connectivity- or managed-policy-throttle pressure.
- The schema-enforced pairing forcing
  `pressure_managed_policy_throttle` banners to carry
  `override_action.override_class =
  not_offered_managed_policy_throttle` so the user is not
  offered an override that the tenant policy would refuse.
- The schema-enforced pairing forcing
  `mirrored_or_offline` framing to carry at least one
  `protected_class_refusals[]` entry whose refusal class is
  `mirror_or_offline_pin_excluded` whenever
  `selected_class_ids[]` includes `artifact_cache` or
  `prebuild_environment_cache` (those are the classes that
  hold mirrored / offline-bundle segments).

## 11. Worked acceptance scenarios

The fixture set covers at least these four scenarios required
by the acceptance criteria:

1. **Low-disk throttling (mirror-heavy offline install).**
   `pressure_class = pressure_degraded`,
   `pressure_source_class = free_disk_floor_breach`,
   `paused_work_lanes[]` includes `provider_refresh`,
   `mirror_pull`, `pack_refresh`, and `prefetch`,
   `next_eviction_steps[]` lists
   `stop_speculative_fetch_and_prefetch`,
   `pause_managed_replication_and_pack_refresh`,
   `trim_interactive_hot_cache`,
   `trim_knowledge_cache_rebuildable`,
   `trim_artifact_cache_unpinned`, and
   `trim_prebuild_environment_unpinned`. Override action opens
   the clear-data review. Deployment context is
   `mirrored_or_offline`.
2. **Clear-cache of rebuildable classes (single-workspace
   local profile).** `selected_class_ids[]` is
   `[interactive_hot_cache, knowledge_cache, artifact_cache,
   prebuild_environment_cache]`. Per-class projection rows
   carry `rebuilt_after_clear` consequence with embedded
   rebuild-cost hints. Protected-class refusals list both
   `evidence_support_cache` (class-specific review required)
   and `user_owned_recovery_state` (authoritative-user-owned
   refused). Recovery / export options carry
   `recovery_review_offered_class_specific`. Deployment
   context is `local_only_individual`.
3. **Blocked clear of user-owned recovery state.** User
   attempts to include `user_owned_recovery_state` in the
   selection; the surface emits a record whose
   `selected_class_ids[]` includes only the disposable classes
   the surface accepted, with the recovery state's projection
   row carrying `class_consequence_state = blocked_protected`
   and the refusal row carrying
   `authoritative_user_owned_clear_refused`. The
   `recovery_export_options[]` entry is
   `recovery_review_offered_class_specific` with the class-
   specific review path linked.
4. **Policy-protected evidence class (managed-tenant
   install).** `pressure_class =
   pressure_managed_policy_throttle`,
   `pressure_source_class = managed_policy_throttle_active`,
   `next_eviction_steps[]` excludes
   `expire_unpinned_evidence_past_retention` and
   `user_owned_recovery_state_only_under_explicit_review`.
   `protected_class_refusals[]` carries an evidence refusal
   row whose `refusal_class` is `policy_held_clear_refused`.
   Deployment context is `managed_policy`. The override
   action is `not_offered_managed_policy_throttle`.

## 12. Adding or changing vocabulary

Adding a value to any vocabulary frozen in this contract
(`pressure_class`, `pressure_source_class`, `framing_class`,
`refusal_class`, `option_class`, `class_consequence_state`,
`consequence_state_label_class`, `banner_copy_class`,
`override_class`, `lane_class`, `lane_state`,
`consumer_surface_class`) is **additive-minor** and requires:

1. Updating the schema enum in
   `schemas/storage/clear_data_review.schema.json` or
   `schemas/storage/low_disk_banner.schema.json`.
2. Updating this document.
3. Adding or updating a fixture under
   `fixtures/storage/clear_data_low_disk_cases/` exercising
   the new value.
4. Bumping the corresponding `*_schema_version` integer.

Repurposing an existing value is **breaking** and requires:

1. A new decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecation of the old value, addition of the new value
   through an additive-minor landing, and a translation pass on
   the clear-data review sheet, low-disk banner, pin manager,
   cleanup-history lane, support bundle, admin storage console,
   and CLI consumers across the deprecation window.

Vocabularies that re-export from upstream seeds (storage-class
matrix, authority class, rebuild-cost class, GC-policy class,
clear-cache protection class, pin-source class,
mirror-or-import origin class, deployment profile, mirror-offline
state, tenant-org scope, low-disk ladder step) follow the
upstream change rules; this contract follows in the same change.

## 13. Out of scope at this revision

- Final clear-data review sheet / low-disk banner / pin manager
  / cleanup-history lane / admin storage console layout,
  animation, and accessibility wiring. The contract pins the
  record family; the rendering surfaces own their own component
  contracts.
- Garbage collection, quota enforcement, and cleanup daemons.
  The cache manager (frozen in
  [`docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md))
  and the resource-governor thresholds artifact own those.
- Localization-ready string tables; the contract carries
  reviewable English sentences and the localization layer is
  consumed separately.
- The class-specific review sheet for `evidence_support_cache`
  and `user_owned_recovery_state`. The cache-manifest schema and
  the local-history restore-preview contract freeze those review
  records; the clear-data review record links them by ref.
- Cleanup-history / post-action attribution lane records. The
  clear-data review record links the lane by consumer-surface
  enum but does not re-mint its shape.
