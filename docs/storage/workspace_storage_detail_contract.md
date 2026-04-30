# Workspace-storage detail, authority-posture, and rebuild-cost contract

This document is the **per-entry inspectability contract** for
Aureline's workspace-storage detail truth. It freezes one
`workspace_storage_detail_row_record` and one
`rebuild_cost_hint_record` so the **per-class entry name, size, last-
used timestamp, rebuild cost in product terms, pin state, sensitivity
class, freshness state, corruption state, policy-protection state,
detail-authority posture, clear action, and export action** of every
entry in a workspace, workset, profile, tenant, or device-total
scope are inspectable as one boundary record family rather than a
maintenance dashboard hidden behind operator copy.

The contract exists so a workspace-storage detail view, a clear-data
review preview, a low-disk drill expansion, a pin manager row, a
cleanup-history attribution row, an admin storage console drill,
and a support-bundle storage section all project the same posture
object without recomputing field names; so disposable cache cannot
be confused with a rebuildable derived index, with a mirrored or
imported pack, with a policy-held evidence packet, or with
authoritative user-owned recovery state; so the rebuild cost a
class-selective clear is about to incur is rendered in product
terms (offline-rebuild risk, startup impact, provenance continuity)
rather than as a single opaque "rebuild cost" label; and so a
stale, partially-scanned, corrupt, or low-privilege row stays
visibly typed instead of being silently dropped.

The contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI/UX Spec, or design-system style guide, those sources win
and this document plus its schemas and fixtures update in the same
change. Where the workspace-storage detail view, clear-data review,
low-disk drill expansion, pin manager, cleanup-history lane,
support bundle, or admin console mints a parallel detail row, a
parallel rebuild-cost label, or a parallel clear-or-export action,
this contract wins and the surface is non-conforming.

## Companion artifacts

- [`/schemas/storage/workspace_storage_detail.schema.json`](../../schemas/storage/workspace_storage_detail.schema.json)
  — boundary schema for `workspace_storage_detail_row_record`.
- [`/schemas/storage/rebuild_cost_hint.schema.json`](../../schemas/storage/rebuild_cost_hint.schema.json)
  — boundary schema for the standalone `rebuild_cost_hint_record`
  shape that the detail row embeds and that other surfaces (clear-
  data review preview, low-disk drill expansion, support-bundle
  storage section) cite by ref.
- [`/fixtures/storage/workspace_storage_detail_cases/`](../../fixtures/storage/workspace_storage_detail_cases/)
  — worked YAML cases covering the five required scenarios (hot
  cache shard, workspace knowledge-cache search index, imported
  docs-pack mirror segment, policy-held evidence packet, user-
  owned recovery journal).

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary that is already frozen
upstream; it consumes the frozen sets by name and by value:

- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md),
  [`/artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml),
  and [`/schemas/runtime/cache_entry_manifest.schema.json`](../../schemas/runtime/cache_entry_manifest.schema.json)
  — the six frozen `storage_class_id` values, the four
  `authority_class` values, the four `rebuild_cost_class` values,
  the five `gc_policy_class` values, the four `sensitivity_class`
  values, the ten `pin_source_class` values, the six
  `inspectable_surface_class` values, and the four
  `clear_cache_protection_class` values. Detail rows re-export
  these names byte-for-byte.
- [`/docs/storage/storage_inspector_contract.md`](./storage_inspector_contract.md)
  and [`/schemas/storage/storage_inspector_card.schema.json`](../../schemas/storage/storage_inspector_card.schema.json),
  [`/schemas/storage/storage_class_breakdown.schema.json`](../../schemas/storage/storage_class_breakdown.schema.json)
  — the parent `storage_inspector_card_record` and the
  `storage_class_breakdown_row_record` family. Every detail row
  carries `card_id_ref` and `breakdown_row_id_ref` linking back to
  those records and a `detail_scope` block that is value-equal to
  the inspector card's `inspector_scope`. The
  `mirror_or_import_origin_class`, `inspector_scope_class`, and
  `entry_class` (a re-export of the upstream `consumer_class`
  vocabulary, plus the entry-level `interactive_hot_cache_shard`
  value the breakdown row aggregates) re-export verbatim.
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  — `authority_class` vocabulary
  (`user_authored_durable_truth`, `user_owned_recovery_state`,
  `admin_or_control_artifact`, `disposable_derived_cache`).
- [`/docs/governance/storage_and_retention_vocabulary.md`](../governance/storage_and_retention_vocabulary.md)
  — `artifact_storage_mode` and `retention_mode` vocabularies the
  policy-protection state and export-action class re-export by
  semantic name.
- [`/docs/ux/deployment_summary_contract.md`](../ux/deployment_summary_contract.md)
  and [`/schemas/deployment/deployment_summary_card.schema.json`](../../schemas/deployment/deployment_summary_card.schema.json)
  — the `mirror_offline_artifact_row_record` rows that mirror /
  imported / vendor-signed / customer-signed detail rows cite by
  ref so the workspace-detail view and the deployment summary
  agree on signer / digest / freshness posture for the same bytes.

## Who reads this contract

- **Workspace-storage detail view, low-disk drill expansion, pin
  manager, clear-data review preview, cleanup-history lane,
  admin storage console drill, and support-bundle storage
  section** — to read **one** detail-row record family instead of
  recomputing fields per surface.
- **Project doctor / attention inbox** — to surface the
  authoritative-user-owned and policy-held rows whose generic
  clear is refused, plus the corrupt or stale rows whose linked
  corruption-rescue path is the only admissible action.
- **Reviewers (release, security, accessibility, claim-manifest)**
  — to verify that disposable cache, rebuildable derived state,
  imported durable artifacts, policy-held evidence, and
  authoritative user-owned recovery state are typed differently,
  that the embedded rebuild-cost hint preserves the offline-risk
  / startup-impact / provenance-continuity distinction, that
  stale and corrupt rows stay visible, and that the clear /
  export action a row offers cannot be widened beyond what the
  storage-class authority admits.

## Two questions the contract answers

Any Aureline surface claiming to expose per-entry storage truth
MUST answer both questions mechanically, without per-surface copy:

1. **What is this entry, and what does it cost to lose?** What
   storage class is it in? What is its size, last-used timestamp,
   sensitivity, freshness, and corruption posture? If it were
   removed, what would the rebuild cost be in product terms — is
   it cheap to rebuild, expensive to rebuild, impossible to
   rebuild offline, or dangerous to delete because it is
   authoritative? What inputs would the rebuild need to consume?
2. **What can the user do with this entry right now?** What is
   the pin state and the policy-protection state? What is the
   admissible clear action — generic, after-pin-release, class-
   specific review, refused authoritative, or refused policy-
   held? What is the admissible export action — offered metadata-
   safe, offered operator-only, required before clear, or
   unsupported (already local-only or policy-bound)? Which
   inspect-only open action navigates to the per-entry detail?

Generic prose like "X MB used", "stale cache", "old data", "free up
space", or "rebuild required" is forbidden when a more precise
per-class, per-entry, per-rebuild-axis figure is knowable. The
schema enforces typed vocabulary and typed sentences; surfaces
render those values.

## 1. Scope

This contract freezes:

- One **workspace-storage detail row record** (§3) emitted whenever
  a workspace-storage detail view, clear-data review preview, low-
  disk drill expansion, pin manager, cleanup-history lane, admin
  storage console drill, or support-bundle storage section needs
  to project current per-entry storage posture. Each row carries:
  `storage_class_id`, the `entry_class` and `entry_ref` /
  `entry_label` triple, `size_bytes`, `last_used_at`, the embedded
  `rebuild_cost_hint`, the `authority_class` and the
  `detail_authority_posture_class` it resolves against, the
  `mirror_or_import_origin_class` and the optional
  `mirror_offline_artifact_row_ref`, the `sensitivity_class`, the
  `freshness_state`, the `corruption_state`, the
  `policy_protection_state`, the `pin_state` and its
  `pin_source_breakdown[]`, the `clear_cache_protection_class`,
  the `clear_action`, the `export_action`, the
  `inspect_only_open_action`, the `redaction_class`, the
  `export_safe`, and the `note`.
- One **rebuild-cost hint record** (§4) the row embeds and that
  other surfaces cite by ref. The hint carries the
  `rebuild_cost_class` re-export, the `offline_rebuild_risk_class`,
  the `startup_impact_class`, the `provenance_continuity_class`,
  the closed `rebuild_inputs_required[]` list, the
  `rebuild_safety_summary_class` summary, and the
  `rebuild_explanation` reviewable sentence.
- The **detail-authority posture vocabulary** (§5) — five values:
  `disposable_derived_state`, `correctness_relevant_derived_state`,
  `imported_durable_artifact`, `policy_held_evidence_state`,
  `authoritative_user_owned_state` — and the schema-enforced
  pairings between posture, storage class, authority class,
  origin class, and clear / export action so disposable cache
  cannot wear the same authority language as recovery state.
- The **rebuild-safety summary vocabulary** (§6) — four values:
  `cheap_to_rebuild_safe_to_remove`,
  `expensive_to_rebuild_but_safe`, `impossible_to_rebuild_offline`,
  `dangerous_to_delete_authoritative` — and the schema-enforced
  pairings between summary, offline-rebuild-risk, startup-impact,
  provenance-continuity, and rebuild-cost classes.
- The **freshness, corruption, and policy-protection state
  vocabularies** (§7) so a stale, partially-scanned, corrupt, or
  policy-protected row is rendered with typed copy rather than
  silently hidden.
- The **clear-action and export-action vocabularies** (§8) so a
  surface cannot offer a generic clear over an evidence row,
  cannot offer a generic clear over user-owned recovery state,
  and cannot collapse the export-before-clear requirement on
  authoritative entries.
- The **cross-record invariants** (§9) so authoritative state is
  never collapsed into disposable hint copy, mirrored / imported
  / policy-protected entries cannot appear as generic disposable
  rows, every row's scope is value-equal to the parent inspector
  card's scope, and inspect-only actions stay inspect-only.

## 2. Out of scope

- The disk scanner, the background quota accountant, the pin
  reference counter, and the cache-manager GC engine. The cache
  manager (frozen in
  [`docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md))
  remains the source of truth for cache-entry admission, low-disk
  ordering, and pin-ref bookkeeping. This contract pins the
  inspectable per-entry record family; it does not implement any
  of those services.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and the shell-interaction-safety contract own the
  strings.
- The class-specific review sheet for `evidence_support_cache`
  and `user_owned_recovery_state`. The cache-manifest schema and
  the local-history restore-preview contract freeze those review
  records; the workspace-detail row links them by ref.
- The corruption-rescue compare sheet itself. That is frozen in
  the corruption-rescue contract; this row links by ref.
- Eviction and rebuild workflows. This contract pins the
  inspectable record; the cache manager and the clear-cache
  preview own the workflow.

## 3. The workspace-storage detail row record

A workspace-storage detail row is one structured projection of one
entry on one inspector scope. The workspace-storage detail view,
clear-data review preview, low-disk drill expansion, pin manager,
cleanup-history lane, admin storage console drill, support-bundle
storage section, and CLI text formatter each render the same row
without changing field names.

### 3.1 Required fields

- `record_kind = workspace_storage_detail_row_record`.
- `workspace_storage_detail_row_schema_version = 1`.
- `row_id` — opaque, stable, safe to log and export.
- `card_id_ref` — opaque ref into the parent
  `storage_inspector_card_record`.
- `breakdown_row_id_ref` — opaque ref into the parent
  `storage_class_breakdown_row_record` whose class this detail
  row drills.
- `detail_scope` — see §3.2. Value-equal to the parent inspector
  card's `inspector_scope` and to the parent breakdown row's
  `class_scope`.
- `storage_class_id` — re-export of the runtime vocabulary.
- `entry_class` — re-export of the upstream `consumer_class`
  plus the `interactive_hot_cache_shard` entry-level value used
  for hot-cache rows.
- `entry_ref` — opaque, stable per-entry id (workspace id +
  artifact ref, mirror segment ref, evidence packet ref,
  recovery journal segment ref, etc.).
- `entry_label` — short reviewable label in product terms.
- `size_bytes` — non-negative integer.
- `last_used_at` — RFC 3339 UTC timestamp from a monotonic clock,
  or `null`. Required `null` when `freshness_state` is
  `unknown_no_scan_yet` or `not_applicable_no_freshness_signal`;
  required non-null otherwise.
- `rebuild_cost_hint` — embedded `rebuild_cost_hint_record` (§4).
- `authority_class` — re-export.
- `detail_authority_posture_class` — see §5.
- `mirror_or_import_origin_class` — re-export.
- `mirror_offline_artifact_row_ref` — opaque ref into the
  deployment-summary mirror row, or `null`. Required non-null
  when origin is `mirrored_copy`, `offline_bundle_local`,
  `vendor_signed_offline_local`, or `customer_signed_mirror_local`;
  required null otherwise.
- `sensitivity_class` — re-export. `t3_secret_adjacent_not_reusable_cache`
  is admissible only on `evidence_support_cache` rows.
- `freshness_state` — see §7.1.
- `corruption_state` — see §7.2.
- `policy_protection_state` — see §7.3.
- `pin_state` — see §3.3.
- `pin_source_breakdown[]` — array of pin-source rows (§3.3).
  Required non-empty when `pin_state` is any pinned class;
  required empty otherwise.
- `clear_cache_protection_class` — re-export.
- `clear_action` — see §8.1.
- `export_action` — see §8.2.
- `inspect_only_open_action` — see §3.4.
- `redaction_class` — one of `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only`. Exported rows MUST NOT widen
  redaction beyond `operator_only_restricted`.
- `export_safe` — boolean.
- `note` — short reviewable sentence. No raw payload bytes, no
  raw paths, no raw URLs.

Optional fields: `linked_clear_cache_preview_ref`,
`linked_class_specific_review_ref`,
`linked_pin_manager_route_ref`, `linked_low_disk_drill_ref`,
`linked_deployment_summary_card_ref`,
`linked_corruption_rescue_ref`.

### 3.2 `detail_scope`

Every row carries a `detail_scope` block whose `scope_class`,
`scope_ref`, and `scope_label` are value-equal to the parent
inspector card's `inspector_scope` and to the parent breakdown
row's `class_scope`. The schema enforces this equivalence so a
workspace-only detail row cannot bleed into another workspace's
breakdown.

### 3.3 `pin_state` and `pin_source_breakdown[]`

The `pin_state_class` is closed:

- `unpinned_no_sources` — the entry is reclaimable by a generic
  clear-cache path (subject to `clear_cache_protection_class`).
- `pinned_user_only` — exactly one explicit user pin.
- `pinned_admin_or_tenant_policy_only` — admin or tenant policy
  pin.
- `pinned_release_or_offline_bundle_only` — pinned by a release
  artifact graph ref or an offline-bundle ref.
- `pinned_evidence_case_or_review_only` — pinned by an open case
  ref or a review pack ref.
- `pinned_retention_window_only` — pinned by a retention window
  ref.
- `pinned_multiple_classes` — more than one pin source class.
- `not_applicable_authoritative_state` — required on
  `user_owned_recovery_state` rows whose authority refuses pin
  semantics for generic clear paths.

Each `pin_source_breakdown[]` entry carries:

- `pin_source_class` — re-export of the runtime
  `pin_source_class` vocabulary.
- `pin_ref_summary_id` — opaque, stable.
- Optional `pin_label` and `notes` for reviewable copy.

### 3.4 `inspect_only_open_action`

Every row MUST carry exactly one inspect-only open action. The
shape mirrors the storage-inspector card's open-details action:
`scope_class = scope_local_only`, `authority_class =
user_local_authority`, `consent_class =
no_consent_required_safe_default`,
`side_effects = ["no_side_effect_inspect_only"]`,
`preserves_evidence_context = true`, `modal_prohibited = true`,
and `revalidation_on_open` ∈ {`none_already_fresh`,
`snapshot_open_read_only`}. Stacking another effect on the
inspect-only action is non-conforming.

## 4. The rebuild-cost hint record

The rebuild-cost hint is the structured answer to "what does it
cost to remove this entry?" The detail row embeds it; the clear-
data review preview, the low-disk drill expansion, and the
support-bundle storage section consume the same shape so a single
"rebuild cost" copy line cannot collapse into "lots of work" copy.

### 4.1 Required fields

- `record_kind = rebuild_cost_hint_record`.
- `rebuild_cost_hint_schema_version = 1`.
- `rebuild_cost_class` — re-export of the runtime vocabulary
  (`authoritative_no_rebuild`, `high_rebuild_cost`,
  `medium_rebuild_cost`, `low_rebuild_cost`).
- `offline_rebuild_risk_class` — see §6.1.
- `startup_impact_class` — see §6.2.
- `provenance_continuity_class` — see §6.3.
- `rebuild_inputs_required[]` — closed list naming every input
  the rebuild needs to consume. Authoritative rows declare
  exactly `[no_input_authoritative_state]`; disposable rows
  declare at least one local input; mirror / offline / policy-
  protected rows declare the matching mirror / offline-bundle /
  policy-pack input.
- `rebuild_safety_summary_class` — see §6.4.
- `rebuild_explanation` — short reviewable sentence in product
  terms. Generic "rebuild required" copy is forbidden when a
  more precise per-class sentence is knowable.

## 5. The detail-authority posture vocabulary

Closed five-value vocabulary. Adding a value is additive-minor.

- `disposable_derived_state` — pure hot disposable cache
  (interactive_hot_cache, prebuild_environment_cache shards
  whose rebuild is fast and provenance-trivial). Generic clear
  is admissible. Rebuild-safety summary is
  `cheap_to_rebuild_safe_to_remove`.
- `correctness_relevant_derived_state` — knowledge cache,
  prebuild_environment_cache, artifact_cache (disposable
  derivation): rebuild is reproducible from on-device source,
  but removal is expensive and may break startup-time
  expectations until the rebuild completes. Surfaces MUST name
  the cost. Rebuild-safety summary is
  `expensive_to_rebuild_but_safe`.
- `imported_durable_artifact` — mirrored copies, offline-bundle
  segments, vendor-signed and customer-signed segments, policy-
  protected admin artifacts. Removal is unsafe under air-gapped
  or network-degraded conditions. Surfaces MUST name the mirror
  or offline-bundle dependency and link the deployment-summary
  card. Rebuild-safety summary is `impossible_to_rebuild_offline`
  on air-gapped or offline-only deployments;
  `expensive_to_rebuild_but_safe` is admissible when network is
  available and the source is a healthy network mirror.
- `policy_held_evidence_state` — evidence_support_cache rows held
  under a retention window, an open case, an admin pin, or a
  tenant pin. Generic clear is refused; the class-specific
  review path is the only admissible clear. Rebuild-safety
  summary is `dangerous_to_delete_authoritative` for
  authoritative_no_rebuild evidence;
  `impossible_to_rebuild_offline` for high-rebuild-cost evidence.
- `authoritative_user_owned_state` — user_owned_recovery_state
  rows. Generic clear is refused; the class-specific review path
  is the only admissible clear. Rebuild-safety summary is
  `dangerous_to_delete_authoritative` always.

The schema enforces these pairings with the storage class,
authority class, origin class, and clear / export action so a
surface cannot describe authoritative recovery state with the
same authority language as a hot disposable cache and cannot
describe a mirrored offline pack as a generic disposable cache
(§9.1, §9.2, §9.3).

## 6. Rebuild-cost hint axes

### 6.1 `offline_rebuild_risk_class` (closed)

- `safe_to_remove_offline` — rebuild reads only from on-device
  source. Admissible offline; pairs with disposable or
  correctness-relevant derived state.
- `rebuild_requires_network_resync` — rebuild needs a network
  round-trip to the provider, the index source, or the release
  ring. Pairs with `network_provider_or_index` rebuild input.
- `rebuild_requires_mirror_or_offline_bundle` — rebuild needs a
  customer-operated mirror, a vendor-published mirror, or an
  offline-bundle ref to be present and trusted. Pairs with the
  matching mirror / offline-bundle rebuild input. Forbidden
  when the deployment is air-gapped and the mirror is absent.
- `rebuild_requires_admin_or_policy_signed_pack` — rebuild needs
  an admin-signed pack, a policy bundle, or a tenant-managed
  pack. Pairs with the `policy_signed_pack` rebuild input.
- `not_rebuildable_after_removal` — authoritative posture for
  user-owned recovery state and admissible authoritative
  evidence. Forces `dangerous_to_delete_authoritative`.

### 6.2 `startup_impact_class` (closed)

- `no_user_visible_impact` — disposable hot-cache shard whose
  removal does not noticeably affect first-open or first-query.
- `slower_first_open_until_warm` — hot-cache rewarming;
  noticeable but bounded.
- `slower_first_query_until_reindexed` — knowledge-cache or
  search-index rebuild; bounded by the indexer.
- `slower_first_build_until_prebuilt` — prebuild / toolchain
  cache rebuild; bounded by the build system.
- `feature_unavailable_until_rebuilt` — mirror-only docs pack,
  mirror-only model pack, policy bundle whose removal disables
  a feature until re-acquired.
- `not_applicable_authoritative_state` — authoritative posture
  where "startup impact" is the wrong frame because the bytes
  are not rebuildable at all.

### 6.3 `provenance_continuity_class` (closed)

- `provenance_preserved_rebuild_from_local_truth` — derived
  index whose rebuild is reproducible from on-device source.
- `provenance_preserved_rebuild_from_signed_source` — mirrored
  or imported pack whose rebuild reads from the signed source
  and carries the signer chain forward.
- `provenance_breaks_until_resigned_or_re_imported` — class
  whose removal would force a re-import or a re-mirror before
  provenance can be re-established.
- `authoritative_provenance_irreplaceable` — user-owned
  recovery state and case-bound evidence whose provenance
  cannot be rebuilt at all.
- `not_applicable_disposable_no_provenance` — hot disposable
  cache whose provenance is not material to the user.

### 6.4 `rebuild_safety_summary_class` (closed four-value)

- `cheap_to_rebuild_safe_to_remove` — disposable hot-cache
  posture. Admitted only with `safe_to_remove_offline` offline
  risk, low-impact startup (`no_user_visible_impact` or
  `slower_first_open_until_warm`), preserved-from-local-truth
  or no-provenance continuity, and `low_rebuild_cost` or
  `medium_rebuild_cost` cost class.
- `expensive_to_rebuild_but_safe` — knowledge-cache or
  prebuild-cache posture. Admitted with safe-offline or
  network-resync risk, slower-rebuild startup-impact class
  (`slower_first_query_until_reindexed` /
  `slower_first_build_until_prebuilt` /
  `slower_first_open_until_warm`), preserved-provenance
  continuity, and `medium_rebuild_cost` or `high_rebuild_cost`
  cost class.
- `impossible_to_rebuild_offline` — mirrored / offline-bundle /
  policy-signed-pack posture. Admitted only with the matching
  mirror or signed-pack offline-risk class plus a
  feature-unavailable-or-slower-startup impact class plus a
  preserved-from-signed-source or provenance-breaks
  continuity class.
- `dangerous_to_delete_authoritative` — user-owned-recovery-
  state and authoritative-evidence posture. Admitted only with
  `authoritative_no_rebuild` cost, `not_rebuildable_after_removal`
  offline risk, `not_applicable_authoritative_state` startup
  impact, `authoritative_provenance_irreplaceable` provenance,
  and the `[no_input_authoritative_state]` rebuild input list.

The schema enforces every pairing in both directions: a surface
cannot label a row "cheap to rebuild" while declaring it not
rebuildable after removal, and cannot label a row
"impossible to rebuild offline" while declaring it safe to
remove offline.

## 7. Inspectability for stale, corrupt, and policy-protected entries

### 7.1 `freshness_state` (closed)

- `fresh_within_window` — last_used_at is inside the freshness
  window the cache manager publishes for the class.
- `stale_within_extended_window` — older than the fresh window
  but inside the extended window.
- `stale_past_extended_window` — older than the extended
  window; the surface renders a stale label and routes the user
  to rebuild / clear / pin path.
- `unknown_no_scan_yet` — no scan has produced a last-used
  timestamp; `last_used_at` is null.
- `not_applicable_no_freshness_signal` — the class does not
  emit a freshness signal (some authoritative rows);
  `last_used_at` is null.

### 7.2 `corruption_state` (closed)

- `not_corrupt` — the entry passed integrity validation.
- `suspected_corrupt_pending_revalidation` — the cache manager
  has flagged the entry; the surface renders the suspected
  state and links the corruption-rescue path.
- `confirmed_corrupt_quarantined` — the entry is quarantined
  and the surface MUST link the corruption-rescue path.
- `confirmed_corrupt_pending_rebuild_or_replace` — the entry is
  pending rebuild or replace.
- `unknown_low_privilege_scan` — the scan ran at low privilege
  and could not validate; the surface MUST surface the consent
  affordance instead of rendering a clean state.

The schema enforces that any `confirmed_corrupt_*` row carries a
non-null `linked_corruption_rescue_ref` so the surface can route
the user to inspect / rebuild / replace verbatim.

### 7.3 `policy_protection_state` (closed)

- `not_policy_protected` — no admin / tenant / retention /
  evidence pin holds the bytes.
- `protected_admin_policy_pin` — admin pin holds the bytes.
- `protected_tenant_policy_pin` — tenant pin holds the bytes.
- `protected_retention_window` — a retention window holds the
  bytes; the row may not be cleared until the window elapses.
- `protected_evidence_case_or_review` — an open case ref or a
  review pack ref holds the bytes.
- `protected_user_owned_authoritative` — required on
  `user_owned_recovery_state` rows.

Surfaces render the policy-protection state verbatim; collapsing
"protected by policy" into a single label is non-conforming when
a more precise class is knowable.

## 8. Clear and export actions

### 8.1 `clear_action` (closed)

- `clear_admissible_generic` — admitted only when
  `clear_cache_protection_class = generic_clear_always_allowed`,
  `pin_state = unpinned_no_sources`, and `authority_class =
  disposable_derived_cache`. Hot-cache rows.
- `clear_admissible_after_pin_release` — admitted when
  `clear_cache_protection_class = generic_clear_with_pin_exclusions`
  and the row is pinned. The surface MUST name the pin source(s).
- `clear_requires_class_specific_review` — required on every
  `evidence_support_cache` row. Surfaces MUST link the class-
  specific review path.
- `clear_refused_authoritative_user_owned` — required on every
  `user_owned_recovery_state` row. Generic clear is refused
  outright.
- `clear_refused_policy_held` — required when the row is held
  by an admin pin, a tenant pin, or a retention window such
  that even the class-specific review path is gated.

### 8.2 `export_action` (closed)

- `export_offered_metadata_safe` — safe baseline; admitted on
  rows whose `redaction_class` is `metadata_safe_default`.
- `export_offered_operator_only` — admitted on operator-only
  rows.
- `export_required_before_clear` — required on
  authoritative_no_rebuild evidence rows. The schema enforces
  that the embedded `rebuild_cost_hint.rebuild_cost_class` is
  `authoritative_no_rebuild` so the export-before-clear
  requirement cannot be falsely escalated onto a non-
  authoritative entry.
- `export_unsupported_already_local_only_disposable` — admitted
  only on disposable derived rows where there is nothing
  meaningful to export.
- `export_unsupported_class` — policy-bound posture where
  export is refused outright.

## 9. Cross-record invariants

The schema enforces these invariants mechanically. A surface that
violates any of them is non-conforming.

1. **Authoritative state never wears disposable hint copy.** A
   row whose `storage_class_id = user_owned_recovery_state` MUST
   carry `detail_authority_posture_class =
   authoritative_user_owned_state`,
   `clear_action = clear_refused_authoritative_user_owned`,
   `pin_state = not_applicable_authoritative_state`,
   `policy_protection_state = protected_user_owned_authoritative`,
   `mirror_or_import_origin_class = local_authoritative`, an
   embedded `rebuild_cost_hint.rebuild_safety_summary_class =
   dangerous_to_delete_authoritative`, and a non-null
   `linked_class_specific_review_ref`.
2. **Evidence rows carry typed evidence posture.** A row whose
   `storage_class_id = evidence_support_cache` MUST carry
   `detail_authority_posture_class = policy_held_evidence_state`,
   `clear_action = clear_requires_class_specific_review`, and
   one of the policy-protected `policy_protection_state`
   classes. The class-specific review ref is required.
3. **Mirrored / imported / vendor-signed / customer-signed rows
   cannot wear `generic_clear_always_allowed` protection.** Any
   row whose origin is `mirrored_copy`, `offline_bundle_local`,
   `vendor_signed_offline_local`, or
   `customer_signed_mirror_local` MUST cite a non-null
   `mirror_offline_artifact_row_ref` and MUST NOT carry
   `clear_cache_protection_class = generic_clear_always_allowed`.
4. **Rebuild-cost summary class is value-paired.** The schema
   forbids every disagreement between
   `rebuild_safety_summary_class`, `offline_rebuild_risk_class`,
   `startup_impact_class`, `provenance_continuity_class`, and
   `rebuild_cost_class` (see §6.4); the four summary classes
   each pin a typed range on the other axes.
5. **Authoritative inputs do not mix with non-authoritative.** A
   `rebuild_inputs_required` list that contains
   `no_input_authoritative_state` MUST contain only that value,
   and the embedded `rebuild_cost_class` MUST be
   `authoritative_no_rebuild`.
6. **Pinned states name a source.** Any non-unpinned, non-
   authoritative `pin_state` MUST carry a non-empty
   `pin_source_breakdown[]`; an unpinned or authoritative pin
   state MUST carry an empty breakdown.
7. **Clear actions match protection.** `clear_admissible_generic`
   requires `generic_clear_always_allowed` protection plus an
   unpinned state plus disposable derived authority;
   `clear_admissible_after_pin_release` requires
   `generic_clear_with_pin_exclusions` protection plus a pinned
   state; `clear_requires_class_specific_review` requires
   `evidence_support_cache` class; `clear_refused_authoritative_user_owned`
   requires `user_owned_recovery_state` class;
   `clear_refused_policy_held` requires an admin / tenant /
   retention policy-protection state.
8. **Export-before-clear is only for authoritative_no_rebuild.**
   `export_required_before_clear` requires the embedded
   `rebuild_cost_hint.rebuild_cost_class =
   authoritative_no_rebuild`.
9. **Sensitivity t3 stays on evidence rows.** Only
   `evidence_support_cache` rows may declare
   `sensitivity_class = t3_secret_adjacent_not_reusable_cache`.
10. **Confirmed-corrupt rows link the corruption-rescue path.**
    Any `confirmed_corrupt_quarantined` or
    `confirmed_corrupt_pending_rebuild_or_replace` row MUST cite
    a non-null `linked_corruption_rescue_ref`.
11. **Last-used timestamp matches freshness state.**
    `unknown_no_scan_yet` and `not_applicable_no_freshness_signal`
    require `last_used_at = null`; every other freshness state
    requires a non-null timestamp.
12. **Inspect-only actions stay inspect-only.** The
    `inspect_only_open_action` MUST declare exactly
    `["no_side_effect_inspect_only"]`. Stacking another effect
    is non-conforming.
13. **Export discipline preserves redaction.** A row exported
    under `export_safe = true` MUST keep its `redaction_class`
    ≤ `operator_only_restricted`. Exports that widen redaction
    to `internal_support_restricted` or `signing_evidence_only`
    are non-conforming.
14. **Detail scope matches inspector scope.** Every row's
    `detail_scope` is value-equal to the parent inspector
    card's `inspector_scope` and to the parent breakdown row's
    `class_scope`. Detail rows cannot bleed across workspaces.

## 10. Worked acceptance scenarios

The fixture set covers at least these five scenarios, one row per
storage class identified by the spec:

1. **Hot cache shard.** `interactive_hot_cache` /
   `interactive_hot_cache_shard` entry on a workspace_only
   scope. Disposable derived state; rebuild-safety summary is
   `cheap_to_rebuild_safe_to_remove`; offline-rebuild risk is
   `safe_to_remove_offline`; startup impact is
   `no_user_visible_impact` or `slower_first_open_until_warm`;
   provenance is `not_applicable_disposable_no_provenance`;
   clear action is `clear_admissible_generic`; export action is
   `export_unsupported_already_local_only_disposable`.
2. **Knowledge cache search index.** `knowledge_cache` /
   `workspace_index_corpus` entry. Correctness-relevant derived
   state; rebuild-safety summary is
   `expensive_to_rebuild_but_safe`; offline-rebuild risk is
   `safe_to_remove_offline`; startup impact is
   `slower_first_query_until_reindexed`; provenance is
   `provenance_preserved_rebuild_from_local_truth`; clear
   action is `clear_admissible_generic`; export action is
   `export_unsupported_already_local_only_disposable`.
3. **Imported docs pack mirror segment.** `artifact_cache` /
   `mirror_snapshot_segment` entry on a customer-signed mirror.
   Imported durable artifact; rebuild-safety summary is
   `impossible_to_rebuild_offline`; offline-rebuild risk is
   `rebuild_requires_mirror_or_offline_bundle`; startup impact
   is `feature_unavailable_until_rebuilt`; provenance is
   `provenance_preserved_rebuild_from_signed_source`; clear
   action is `clear_admissible_after_pin_release`; export
   action is `export_offered_operator_only`. The row cites the
   matching `mirror_offline_artifact_row_record` from the
   deployment-summary contract by ref.
4. **Policy-held evidence packet.** `evidence_support_cache` /
   `evidence_packet_blob` entry held by an open case ref.
   Policy-held evidence state; rebuild-safety summary is
   `dangerous_to_delete_authoritative`; clear action is
   `clear_requires_class_specific_review`; export action is
   `export_required_before_clear`; sensitivity_class is
   `t2_code_bearing_bounded` or `t3_secret_adjacent_not_reusable_cache`.
   The row links the class-specific review path verbatim.
5. **User-owned recovery journal.** `user_owned_recovery_state`
   / `workspace_recovery_journal` entry. Authoritative user-
   owned state; rebuild-safety summary is
   `dangerous_to_delete_authoritative`; clear action is
   `clear_refused_authoritative_user_owned`; export action is
   `export_required_before_clear`; pin state is
   `not_applicable_authoritative_state`; the row links the
   class-specific review path verbatim.

## 11. Adding or changing vocabulary

Adding a value to any vocabulary frozen in this contract
(`detail_authority_posture_class`, `pin_state_class`,
`freshness_state_class`, `corruption_state_class`,
`policy_protection_state_class`, `clear_action_class`,
`export_action_class`, `entry_class`,
`offline_rebuild_risk_class`, `startup_impact_class`,
`provenance_continuity_class`, `rebuild_input_class`,
`rebuild_safety_summary_class`) is **additive-minor** and
requires:

1. Updating the schema enum in
   `schemas/storage/workspace_storage_detail.schema.json` or
   `schemas/storage/rebuild_cost_hint.schema.json`.
2. Updating this document.
3. Adding or updating a fixture under
   `fixtures/storage/workspace_storage_detail_cases/` exercising
   the new value.
4. Bumping the corresponding `*_schema_version` integer.

Repurposing an existing value is **breaking** and requires:

1. A new decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecation of the old value, addition of the new value
   through an additive-minor landing, and a translation pass on
   the workspace-storage detail view, clear-data review
   preview, low-disk drill expansion, pin manager,
   cleanup-history lane, support bundle, and admin storage
   console consumers across the deprecation window.

Vocabularies that re-export from upstream seeds (storage-class
matrix, authority class, rebuild-cost class, GC-policy class,
sensitivity class, pin-source class, mirror-or-import origin
class, clear-cache protection class) follow the upstream change
rules; this contract follows in the same change.

## 12. Out of scope at this revision

- Final workspace-storage detail / clear-data review / low-disk
  drill expansion / pin manager / cleanup-history lane / admin
  storage console layout, animation, and accessibility wiring.
  The contract pins the per-entry record family; the rendering
  surfaces own their own component contracts.
- Pixel-perfect chart and bar layout, the on-disk byte format
  of any store, and the live disk scanner / quota accountant.
- Localization-ready string tables; the contract carries
  reviewable English sentences and the localization layer is
  consumed separately.
- Eviction and rebuild workflows. The cache manager (frozen in
  `docs/runtime/storage_classes_and_gc.md`) and the clear-cache
  preview record (frozen in
  `schemas/runtime/cache_entry_manifest.schema.json`) own those
  workflows.
