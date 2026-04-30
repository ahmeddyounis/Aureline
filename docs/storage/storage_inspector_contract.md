# Storage-inspector card, largest-consumer breakdown, and workspace/profile-scope contract

This document is the **cross-surface inspectability contract** for
Aureline's storage-inspector truth. It freezes one
`storage_inspector_card_record` and one
`storage_class_breakdown_row_record` so the **total disk used, the
class-by-class breakdown, the workspace/profile/tenant scope of every
row, the quota or policy source, the largest local consumers, the
mirrored / imported / policy-protected artifact posture, and the
stale / partial-scan / low-privilege scan posture** of an Aureline
install are inspectable as one boundary record family rather than a
maintenance dashboard hidden behind operator copy.

The contract exists so a settings storage inspector, a workspace-
storage detail view, a low-disk banner expansion, a clear-data review
sheet header, an admin storage console, and a support-bundle storage
section all project the same posture object without recomputing field
names; so disposable cache cannot be confused with rebuildable
artifact, with policy-protected evidence, or with authoritative user-
owned recovery state; so a workspace's bytes cannot be silently
counted under another workspace or profile; and so a stale,
partially-scanned, or low-privilege scan does not silently render as
"trusted total used" copy.

The contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX Spec, or design-system style guide, those sources win and this
document plus its schemas and fixtures update in the same change.
Where the storage inspector, workspace-storage detail, clear-data
review, low-disk banner, pin manager, cleanup-history lane, support
bundle, or admin console mints a parallel storage card, parallel
class breakdown, or parallel largest-consumer list, this contract
wins and the surface is non-conforming.

## Companion artifacts

- [`/schemas/storage/storage_inspector_card.schema.json`](../../schemas/storage/storage_inspector_card.schema.json)
  — boundary schema for `storage_inspector_card_record`.
- [`/schemas/storage/storage_class_breakdown.schema.json`](../../schemas/storage/storage_class_breakdown.schema.json)
  — boundary schema for `storage_class_breakdown_row_record` and the
  `largest_consumer_row` shape it embeds.
- [`/fixtures/storage/storage_inspector_cases/`](../../fixtures/storage/storage_inspector_cases/)
  — worked YAML cases covering at least the four acceptance
  scenarios (single-workspace local profile, shared-profile multi-
  workspace, policy-limited scan, mirror-heavy offline install) plus
  the stale-scan posture.

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary that is already frozen
upstream; it consumes the frozen sets by name and by value:

- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md),
  [`/artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml),
  and [`/schemas/runtime/cache_entry_manifest.schema.json`](../../schemas/runtime/cache_entry_manifest.schema.json)
  — the six frozen `storage_class_id` values
  (`interactive_hot_cache`, `knowledge_cache`, `artifact_cache`,
  `prebuild_environment_cache`, `evidence_support_cache`,
  `user_owned_recovery_state`), the four `authority_class` values,
  the four `rebuild_cost_class` values, the five `gc_policy_class`
  values, the six `storage_posture_class` values, the seven
  `quota_basis_class` values, the ten `pin_source_class` values, the
  six `inspectable_surface_class` values, the four
  `clear_cache_protection_class` values, and the eight
  `low_disk_ladder_step` values. The inspector card and breakdown
  rows re-export these names byte-for-byte.
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  — `authority_class` vocabulary
  (`user_authored_durable_truth`, `user_owned_recovery_state`,
  `admin_or_control_artifact`, `disposable_derived_cache`) and the
  state-plane / portability rules. Rows that name a workspace or
  profile scope cite scope refs minted there.
- [`/docs/governance/storage_and_retention_vocabulary.md`](../governance/storage_and_retention_vocabulary.md)
  and [`/artifacts/governance/storage_modes.yaml`](../../artifacts/governance/storage_modes.yaml)
  — the `artifact_storage_mode` vocabulary the
  `mirror_or_import_origin_class` field re-exports, plus the
  `retention_mode` vocabulary the protected-evidence rows cite.
- [`/docs/ux/deployment_summary_contract.md`](../ux/deployment_summary_contract.md)
  and [`/schemas/deployment/deployment_summary_card.schema.json`](../../schemas/deployment/deployment_summary_card.schema.json)
  — `mirror_freshness_class`, `signer_state_class`,
  `digest_state_class`, `offline_cache_posture_class`, and
  `mirror_source_class`. Mirror/imported artifact rows cite the
  matching deployment-summary `mirror_offline_artifact_row_record`
  by ref so the inspector and the deployment summary stay value-
  equal on the same posture.
- [`/docs/ux/persistence_inspector_contract.md`](../ux/persistence_inspector_contract.md)
  — the remembered-state-inspector shape this contract sits next to;
  the storage inspector covers bytes-on-disk per class, the
  persistence inspector covers per-artifact restore / portability
  posture. They link by opaque ref; neither paraphrases the other.

## Who reads this contract

- **Settings storage inspector, workspace-storage detail view,
  low-disk banner expansion, clear-data review sheet header, pin
  manager, cleanup-history lane header, admin storage console,
  support-bundle storage section, and CLI `--storage` text formatter**
  — to read **one** card-and-breakdown record family instead of
  recomputing fields per surface.
- **Project doctor / attention inbox, status-bar storage cell, and
  About-panel storage excerpt** — to render the same scope rows the
  inspector renders, with the same opaque scope refs.
- **Reviewers (release, security, accessibility, claim-manifest)** —
  to verify that disposable cache, rebuildable derived state,
  authoritative user-owned recovery state, mirrored / imported
  artifacts, and policy-protected evidence are typed differently and
  cannot be merged into a single "used" cell, that scope rows
  identify exactly which workspace or profile owns the bytes, that
  stale / partial-scan / low-privilege states are surfaced rather
  than hidden, and that the largest-consumer rows preserve authority
  posture instead of collapsing into raw bytes.

## Two questions the contract answers

Any Aureline surface claiming to expose current storage truth MUST
answer both questions mechanically, without per-surface copy:

1. **What is the storage posture right now?** What is the total
   used and the device or quota ceiling it is measured against?
   What is the per-class breakdown, and which workspace, profile,
   tenant, or machine scope owns each class's bytes? What is the
   freshness of the scan, and was the scan partial or low-privilege?
   What largest local consumers explain the bulk of the bytes, and
   what is each consumer's authority posture?
2. **What is reclaimable, what is rebuildable, and what is
   protected?** For each class: how many reclaimable bytes can a
   class-selective clear surface offer, which entries are pinned and
   why, which entries belong to mirrored / imported / policy-
   protected artifacts that must not appear as disposable cache, and
   which entries are authoritative user-owned recovery state that
   the inspector exposes for review without offering a generic
   clear?

Generic prose like "X GB used", "lots of cache", "your workspace is
big", "free up space", or "old data" is forbidden when a more
precise per-class, per-scope, per-authority figure is knowable. The
schema enforces typed vocabulary and typed sentences; surfaces
render those values.

## 1. Scope

This contract freezes:

- One **storage-inspector card record** (§3) emitted whenever an
  inspector, workspace-storage detail view, low-disk banner
  expansion, clear-data review header, admin console, support
  bundle, or CLI surface needs to project current storage posture.
  The card carries: `total_used_bytes`, the
  `quota_or_policy_source` block, the per-class
  `class_breakdown_row_refs[]`, the `largest_consumers[]` projection,
  the `inspector_scope`, the `scan_posture` (freshness +
  partial-scan + low-privilege state), the `open_details_action`,
  the `protected_class_visibility[]` guarantee, the
  `redaction_class`, and the `consumer_surfaces[]`.
- The **storage-class breakdown row record** (§4) for every storage
  class in scope on the card. Each row carries the
  `storage_class_id`, the `class_used_bytes`, the
  `reclaimable_bytes_estimate`, the `protected_bytes`, the
  `pinned_bytes`, the `authority_class`, the
  `rebuild_cost_class`, the `gc_policy_class`, the
  `clear_cache_protection_class`, the `quota_basis_class` and the
  `quota_ceiling_bytes` it resolves against, the
  `mirror_or_import_origin_class`, the optional
  `mirror_offline_artifact_row_ref` into the deployment-summary row
  for mirror-class artifacts, the `pinned_consumer_breakdown[]` for
  pinned bytes, the per-row `largest_consumers[]`, the
  `class_scope` matching the card scope, the `posture` (re-export of
  `storage_posture_class`), and the `last_class_scan_at`.
- The **largest-consumer row shape** (§5) embedded inside both the
  card and per-class rows. Each largest-consumer row names the
  `consumer_class`, the opaque `consumer_ref`, the bytes used, the
  `authority_class`, the `rebuild_cost_class`, the
  `mirror_or_import_origin_class`, the optional
  `pin_summary_class`, and the `inspect_only_open_action`. The
  largest-consumer rows are **never** sorted by bytes alone; the
  schema enforces a typed authority-aware ordering (§5.4) so an
  authoritative user-owned consumer is never hidden behind a larger
  disposable cache.
- The **scan-posture block** (§6) which freezes the
  `scan_freshness_class`, `partial_scan_reason_class`, and
  `low_privilege_scan_reason_class` vocabularies so an inspector
  cannot silently report a stale, partial, or low-privilege total
  as if it were a fresh device-wide scan.
- The **cross-record invariants** (§7) so disposable cache cannot
  be merged with authoritative recovery state, mirrored / imported
  / policy-protected bytes cannot appear as a generic disposable
  total, every byte is owned by a typed scope (workspace, profile,
  tenant, machine), and the largest-consumer rows preserve
  authority posture across surfaces.

## 2. Out of scope

- The disk scanner, the background quota accountant, the pin
  reference counter, and the cache-manager GC engine. The cache
  manager (frozen in
  [`docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md))
  remains the source of truth for cache-entry admission, low-disk
  ordering, and pin-ref bookkeeping. This contract pins the
  inspectable record family; it does not implement any of those
  services.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and the shell-interaction-safety contract own the strings.
- Storage telemetry wire format, opaque-ref minting algorithm, and
  the diagnostics-bundle envelope. The schema registry rows for the
  inspector, workspace-storage detail, support-bundle, and admin-
  storage packet families consume this record family separately.
- The clear-cache preview record itself. That is frozen in
  [`schemas/runtime/cache_entry_manifest.schema.json`](../../schemas/runtime/cache_entry_manifest.schema.json);
  the inspector card cites its review sheet by ref but does not re-
  mint the preview shape.

## 3. The storage-inspector card record

A storage-inspector card is one structured projection of current
storage posture against one inspector scope. The settings inspector,
workspace-storage detail, low-disk banner expansion, clear-data
review header, admin storage console, support-bundle storage
section, and CLI text formatter each render the same record without
changing field names.

### 3.1 Required fields

- `record_kind = storage_inspector_card_record`.
- `storage_inspector_card_schema_version = 1`.
- `card_id` — opaque, stable, safe to log and export.
- `emitted_at` — RFC 3339 UTC timestamp from a monotonic clock.
- `inspector_scope` — see §3.2.
- `total_used_bytes` — non-negative integer; sum of every in-scope
  class row's `class_used_bytes`. The schema enforces
  `total_used_bytes` value-equal to the sum of the card's class
  rows (§7.4); a card whose total drifts from its rows is non-
  conforming.
- `quota_or_policy_source` — see §3.3.
- `class_breakdown_row_refs[]` — opaque refs into emitted
  `storage_class_breakdown_row_record` rows. Required non-empty;
  every in-scope class on the inspector scope MUST have a row.
- `largest_consumers[]` — array of largest-consumer rows (§5)
  resolved against the whole card scope. Bounded ordering (§5.4)
  applies. Required minimum length 0 only when
  `total_used_bytes = 0`; otherwise required minimum length 1.
- `scan_posture` — see §6.
- `protected_class_visibility[]` — required closed-set acknowledgment
  (§3.5) that the surface keeps `evidence_support_cache` and
  `user_owned_recovery_state` rows visible regardless of bytes.
- `open_details_action` — see §3.4.
- `consumer_surfaces[]` — closed set of surfaces consuming the card
  (§3.6).
- `redaction_class` — one of `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only`. The card's redaction class is the floor
  every per-class row resolves against; rows MUST NOT widen
  redaction relative to the card.
- `export_safe` — boolean. Required `true` when the card is
  exported through the support-bundle, admin-audit, or release-
  evidence surfaces. The schema enforces that exported cards never
  widen redaction beyond `operator_only_restricted`.

Optional fields: `linked_clear_cache_preview_ref`,
`linked_low_disk_drill_ref`, `linked_deployment_summary_card_ref`,
`linked_continuity_packet_ref`, `notes`.

### 3.2 `inspector_scope` (closed)

The inspector renders against one scope at a time. Required fields:

- `scope_class` — closed set:
  - `device_total` — every class on the device, summed across
    every workspace and profile.
  - `workspace_only` — bytes owned by exactly one workspace.
  - `workset_only` — bytes owned by exactly one workset.
  - `profile_only` — bytes owned by exactly one user-profile slice
    of cache that is profile-scoped (rare; most profile state is
    not cached).
  - `tenant_only` — bytes scoped to one tenant on a managed-cloud
    or enterprise deployment.
  - `slice_only` — bytes scoped to one slice (for example, a
    review slice or a remote-attached slice).
- `scope_ref` — opaque id within the scope class (workspace id,
  workset id, profile id, tenant id, slice id). Required non-null
  for every class except `device_total`. Required null for
  `device_total`.
- `scope_label` — short reviewable label naming the scope in
  product terms. Generic "your data" or "everything" copy is non-
  conforming when a more precise label is knowable.

The schema enforces that every breakdown row's `class_scope` is
value-equal to the card's `inspector_scope` so a workspace card
never silently mixes another workspace's bytes into its rows.

### 3.3 `quota_or_policy_source` (closed)

The inspector card MUST declare what its
`total_used_bytes` is measured against. Required fields:

- `quota_basis_class` — re-export of the runtime
  `quota_basis_class` vocabulary:
  `per_workspace_quota`, `global_device_quota`, `per_class_ceiling`,
  `per_tenant_quota`, `policy_bound_evidence_quota`,
  `retention_policy_only`, `digest_store_plus_class_ceiling`. The
  card-level basis is the most-restrictive basis active across the
  card scope's rows; per-row bases continue to live on each
  breakdown row.
- `quota_ceiling_bytes` — non-negative integer or `not_applicable`
  literal sentinel string. Required actionable value when
  `quota_basis_class` resolves against an enforceable ceiling
  (every basis except `retention_policy_only`); required
  `not_applicable` when no enforceable ceiling exists. A card that
  cites a basis with no ceiling but does not surface
  `not_applicable` is non-conforming.
- `quota_authority_class` — closed set:
  `user_local_authority`, `admin_policy_authority`,
  `tenant_policy_authority`, `device_governor_authority`,
  `not_applicable`. Identifies who set the ceiling. A card whose
  `quota_basis_class` is `per_tenant_quota` MUST set
  `quota_authority_class = tenant_policy_authority`; a card whose
  `quota_basis_class` is `policy_bound_evidence_quota` MUST set
  `quota_authority_class = admin_policy_authority`.
- `policy_source_ref` — opaque ref into the policy bundle row,
  retention window, admin console policy, or device-governor
  threshold that minted the ceiling. Required non-null whenever
  `quota_authority_class` is one of `admin_policy_authority`,
  `tenant_policy_authority`, or `device_governor_authority`;
  required `null` when `quota_authority_class` is
  `user_local_authority` or `not_applicable`.

### 3.4 `open_details_action`

Every card MUST carry exactly one open-details action that opens
the per-class breakdown drill-down. Required fields:

- `action_id` — opaque id.
- `label` — short reviewable label
  (e.g. "Open storage details for this workspace").
- `target_route_ref` — opaque ref to the detail route.
- `scope_class` — `scope_local_only`. The open-details action
  cannot reach beyond the local device.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `revalidation_on_open` — one of `none_already_fresh`,
  `snapshot_open_read_only`. Generic refresh-on-open is non-
  conforming because the card already serializes the resolved
  posture; rescans are surfaced through the
  `scan_posture.scan_freshness_class` field instead.
- `modal_prohibited` — `true`. Cards never raise a modal from the
  open-details action; the action navigates or expands inline.

### 3.5 `protected_class_visibility[]` (closed)

Closed set, every value of which the card MUST acknowledge as
**visible regardless of bytes**:

- `evidence_support_cache_visible` — the card surfaces an
  `evidence_support_cache` row even when its `class_used_bytes` is
  zero, so reviewers can see retention posture. The same applies
  for any non-zero bytes, where the row is rendered with its
  pin / retention reason instead of as a generic cache.
- `user_owned_recovery_state_visible` — the card surfaces a
  `user_owned_recovery_state` row even when its `class_used_bytes`
  is zero, so reviewers can see what the product holds on the
  user's behalf. The row is never offered for generic clear; the
  inspector links the class-specific review sheet instead.

The schema enforces both values appear on every card whose
`scope_class` covers either class. Surfaces that hide either class
because its bytes are small are non-conforming.

### 3.6 `consumer_surface_class` (closed)

Closed set of surfaces consuming the card.

- `storage_inspector` — the global inspector / settings page.
- `workspace_storage_detail` — per-workspace or per-slice detail.
- `clear_data_review` — class-selective review sheet header.
- `low_disk_banner` — pressure-banner expansion.
- `pin_manager` — pin / retention manager.
- `cleanup_history_lane` — post-action attribution lane header.
- `admin_storage_console` — admin / tenant storage console.
- `support_packet_export` — support-bundle storage section.
- `about_panel_storage_excerpt` — About-panel storage excerpt.
- `cli_text_formatter` — CLI text formatter render.

A `pin_manager` consumer on a card whose
`class_breakdown_row_refs[]` does not include any class with a
non-zero `pinned_bytes` is admissible (the manager renders an
empty state); a `low_disk_banner` consumer on a card whose
`scan_posture` is stale or partial is **not** admissible (§7.6) —
the banner MUST cite a fresh scan to act on.

## 4. The storage-class breakdown row record

The breakdown row is one structured projection of one storage
class on one card scope. The settings inspector, workspace-storage
detail, clear-data review, low-disk banner expansion, pin manager,
cleanup-history lane header, admin storage console, support-bundle
storage section, and CLI text formatter each render the same row
without changing field names.

### 4.1 Required fields

- `record_kind = storage_class_breakdown_row_record`.
- `storage_class_breakdown_row_schema_version = 1`.
- `row_id` — opaque, stable.
- `card_id_ref` — opaque ref into the parent
  `storage_inspector_card_record`.
- `storage_class_id` — re-export of the runtime
  `storage_class_id` vocabulary
  (`interactive_hot_cache`, `knowledge_cache`, `artifact_cache`,
  `prebuild_environment_cache`, `evidence_support_cache`,
  `user_owned_recovery_state`).
- `class_scope` — see §4.2. Value-equal to the parent card's
  `inspector_scope` block.
- `class_used_bytes` — non-negative integer.
- `reclaimable_bytes_estimate` — non-negative integer. Bytes a
  class-selective clear of this row would reclaim today, given
  current pin refs and protection posture. The schema enforces
  `reclaimable_bytes_estimate ≤ class_used_bytes - protected_bytes
  - pinned_bytes`; a row whose reclaimable estimate exceeds the
  unpinned-and-unprotected remainder is non-conforming.
- `protected_bytes` — non-negative integer. Bytes that a generic
  clear-cache MUST NOT remove because the row's
  `clear_cache_protection_class` is
  `protected_requires_class_specific_review` or
  `protected_never_generic_clear`. For
  `evidence_support_cache` and `user_owned_recovery_state` rows,
  `protected_bytes = class_used_bytes` always.
- `pinned_bytes` — non-negative integer. Bytes excluded from a
  generic clear because at least one pin ref keeps them alive.
- `authority_class` — re-export of
  `user_authored_durable_truth`,
  `user_owned_recovery_state`, `admin_or_control_artifact`,
  `disposable_derived_cache`. The schema enforces the runtime
  matrix: every disposable class binds
  `disposable_derived_cache`; `user_owned_recovery_state` class
  binds `user_owned_recovery_state` authority;
  `evidence_support_cache` class binds
  `admin_or_control_artifact` or `user_owned_recovery_state`;
  `artifact_cache` class binds
  `disposable_derived_cache` or `admin_or_control_artifact`.
- `rebuild_cost_class` — re-export of the four-value rebuild-cost
  vocabulary. `authoritative_no_rebuild` is admitted only on
  `evidence_support_cache` and `user_owned_recovery_state` rows.
- `gc_policy_class` — re-export of the five-value GC-policy
  vocabulary. The schema enforces the runtime matrix.
- `clear_cache_protection_class` — re-export of the four-value
  protection vocabulary. `protected_never_generic_clear` is
  required for `user_owned_recovery_state`;
  `protected_requires_class_specific_review` is required for
  `evidence_support_cache`.
- `quota_basis_class` — re-export. The per-row basis is the basis
  the cache manager applies to this class on this scope.
- `quota_ceiling_bytes` — non-negative integer or
  `not_applicable` literal sentinel string. Same shape as the
  card-level field.
- `mirror_or_import_origin_class` — see §4.3.
- `mirror_offline_artifact_row_ref` — opaque ref into a
  `mirror_offline_artifact_row_record` from the deployment-summary
  contract. Required non-null when
  `mirror_or_import_origin_class` resolves against a mirror or
  offline-bundle origin (`mirrored_copy`, `offline_bundle_local`,
  `vendor_signed_offline_local`, `customer_signed_mirror_local`);
  required null when origin is `local_authoritative`,
  `local_disposable_cache`, or `not_applicable`.
- `pinned_consumer_breakdown[]` — array of pin-source breakdown
  rows for the `pinned_bytes` total. Each entry names a
  `pin_source_class`, the bytes pinned by that source, and an
  opaque `pin_ref_summary_id`. Empty when `pinned_bytes = 0`.
- `largest_consumers[]` — array of largest-consumer rows (§5)
  resolved against this class on this scope. Required minimum
  length 0 only when `class_used_bytes = 0`; otherwise required
  minimum length 1 (the largest single consumer for the row).
- `posture` — re-export of `storage_posture_class`
  (`healthy`, `rebuild_pending`, `pressure_trimmed`,
  `reset_candidate`, `retained_for_evidence`, `missing`).
  `retained_for_evidence` is admissible only on
  `evidence_support_cache` and `user_owned_recovery_state` rows.
- `last_class_scan_at` — RFC 3339 UTC timestamp from a monotonic
  clock. Required non-null whenever the card's
  `scan_posture.scan_freshness_class` is anything other than
  `scan_unknown_no_scan_yet`.
- `inspectable_on_surfaces[]` — re-export of the runtime
  inspectable-surface vocabulary. The class's row MUST declare at
  least the surfaces the runtime matrix admits for the class.
- `redaction_class` — one of the four standard classes.
- `export_safe` — boolean.
- `note` — short reviewable sentence. No raw payload bytes, no raw
  paths, no raw URLs.

Optional fields: `linked_clear_cache_preview_ref`,
`linked_low_disk_drill_ref`, `linked_deployment_summary_card_ref`,
`linked_pin_manager_route_ref`,
`linked_class_specific_review_ref` (required non-null when
`clear_cache_protection_class` is
`protected_requires_class_specific_review` or
`protected_never_generic_clear`).

### 4.2 `class_scope`

Every row carries a `class_scope` block whose
`scope_class`, `scope_ref`, and `scope_label` are value-equal to
the parent card's `inspector_scope`. The schema enforces this
equivalence so a workspace-only card cannot mix a tenant or
device row.

### 4.3 `mirror_or_import_origin_class` (closed)

Closed set naming where the bytes came from. Adding a value is
additive-minor.

- `local_authoritative` — bytes the user produced on this device
  (only valid on `user_owned_recovery_state` rows; never used on
  cache rows because cache is by definition derived).
- `local_disposable_cache` — bytes derived locally from other
  truth on this device.
- `local_evidence_capture` — bytes captured locally as evidence
  (crash payloads awaiting case creation, validation artifacts);
  only valid on `evidence_support_cache` rows.
- `mirrored_copy` — bytes pulled from a mirror (a
  `mirror_offline_artifact_row_record` with
  `mirror_source_class = customer_operated_mirror` or
  `vendor_published_mirror_for_customer`). Pairs with
  `artifact_cache` or `prebuild_environment_cache` typically.
- `offline_bundle_local` — bytes derived from an offline bundle
  on this device (matches
  `mirror_source_class = offline_bundle_derived_mirror`).
- `vendor_signed_offline_local` — bytes derived from a vendor-
  signed offline bundle (the air-gapped pathway).
- `customer_signed_mirror_local` — bytes derived from a customer-
  signed mirror or offline bundle (customer-managed trust root).
- `policy_protected_admin_artifact` — bytes that came in via an
  admin policy bundle; only valid on `evidence_support_cache` or
  `artifact_cache` rows.
- `not_applicable` — class is not present on this scope. The row
  MUST set `class_used_bytes = 0`,
  `reclaimable_bytes_estimate = 0`, `protected_bytes = 0`,
  `pinned_bytes = 0`, and `largest_consumers[]` empty.

The schema enforces these `mirror_or_import_origin_class` /
`storage_class_id` pairings:

- `local_authoritative` ⇒ class is `user_owned_recovery_state`.
- `local_evidence_capture` ⇒ class is `evidence_support_cache`.
- `policy_protected_admin_artifact` ⇒ class is
  `evidence_support_cache` or `artifact_cache`.
- `mirrored_copy`, `offline_bundle_local`,
  `vendor_signed_offline_local`, `customer_signed_mirror_local` ⇒
  class is `artifact_cache`, `prebuild_environment_cache`, or
  `evidence_support_cache`.
- A row whose origin is `mirrored_copy`, `offline_bundle_local`,
  `vendor_signed_offline_local`, or
  `customer_signed_mirror_local` MUST cite a non-null
  `mirror_offline_artifact_row_ref` (§4.1) and MUST set
  `clear_cache_protection_class` to
  `generic_clear_with_pin_exclusions` or stricter (a generic
  clear cannot remove a pinned mirror or offline-bundle ref).

These pairings keep mirrored, imported, and policy-protected
bytes from being mistaken for disposable cache.

## 5. The largest-consumer row shape

Every storage card and every breakdown row carries a bounded list
of largest consumers — the per-store, per-corpus, per-workspace,
or per-policy entries that explain the bulk of the bytes. The
shape is shared between the card scope and the per-class scope.

### 5.1 Required fields

- `consumer_class` — closed set (§5.2).
- `consumer_ref` — opaque, stable consumer id.
- `consumer_label` — short reviewable label in product terms.
  Generic "Other" or "Misc" copy is admitted only when
  `consumer_class` is `aggregated_remainder_other` AND the
  aggregated bytes are ≤10% of the row total; otherwise generic
  labels are non-conforming.
- `consumer_used_bytes` — non-negative integer.
- `authority_class` — re-export. The consumer MUST carry the same
  authority class as its containing breakdown row, except that an
  aggregated `aggregated_remainder_other` row carries
  `disposable_derived_cache` only.
- `rebuild_cost_class` — re-export.
- `mirror_or_import_origin_class` — re-export. Value-equal to the
  containing breakdown row's origin, or `not_applicable` only on
  the aggregated-remainder consumer.
- `pin_summary_class` — closed set:
  - `no_pin_in_scope` — the consumer carries no pin refs.
  - `pinned_one_source` — exactly one pin source.
  - `pinned_multiple_sources` — multiple pin sources.
  - `not_applicable` — consumer is the aggregated remainder.
- `inspect_only_open_action` — see §5.3.

Optional fields: `linked_pin_ref_summary_id`, `notes`.

### 5.2 `consumer_class` (closed)

Adding a value is additive-minor and bumps
`storage_class_breakdown_row_schema_version`.

- `workspace_index_corpus` — a workspace's search / graph /
  embeddings corpus.
- `workspace_history_lane` — a workspace's local history /
  rollback checkpoint store.
- `workspace_recovery_journal` — a workspace's dirty-buffer
  recovery journal.
- `workset_session_restore_state` — workset / session restore
  state.
- `extension_pack` — one extension package or its sibling cache.
- `docs_pack_corpus` — one docs-pack corpus.
- `model_pack_blob` — one model-pack blob.
- `update_pack_blob` — one update or patch bundle.
- `policy_bundle_blob` — one signed policy bundle (active
  epoch and any pinned last-known-good).
- `prebuild_layer_blob` — one container or toolchain layer.
- `template_or_archetype_pack` — one certified template or
  archetype pack.
- `evidence_packet_blob` — one evidence / crash / review packet.
- `support_export_assembly_in_flight` — a support export in flight.
- `terminal_restore_metadata_store` — terminal-restore metadata.
- `mirror_snapshot_segment` — one mirror snapshot segment.
- `offline_bundle_segment` — one offline-bundle segment.
- `aggregated_remainder_other` — bounded "everything else" row,
  capped at 10% of the containing total.

### 5.3 `inspect_only_open_action`

Every largest-consumer row carries exactly one inspect-only open
action that navigates to the per-consumer detail (workspace
storage detail, pin manager, mirror artifact detail, or evidence
detail). Required fields:

- `action_id` — opaque id.
- `label` — short reviewable label.
- `scope_class` — `scope_local_only`.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `modal_prohibited` — `true`.
- `revalidation_on_open` — one of `none_already_fresh`,
  `snapshot_open_read_only`.

### 5.4 Authority-aware ordering

The largest-consumer rows on the card and on every breakdown row
are sorted with the following typed order:

1. Every consumer whose `authority_class` is
   `user_owned_recovery_state` MUST appear before every consumer
   whose `authority_class` is `disposable_derived_cache`,
   regardless of bytes.
2. Within the same authority class, rows are sorted by
   `consumer_used_bytes` descending.
3. The `aggregated_remainder_other` row, when present, is always
   the last row.
4. Ties on bytes are broken by `consumer_ref` ascending.

A surface that re-sorts the rows by raw bytes alone is non-
conforming. The schema enforces the authority-aware ordering on
the persisted record; renderers consume the order verbatim.

## 6. The scan-posture block

The card's `scan_posture` block freezes how fresh the inspector's
totals are and whether the scan ran fully and at full privilege.
Required fields:

- `scan_freshness_class` — closed set:
  - `scan_fresh_within_window` — the underlying scan completed
    inside the freshness window the cache manager publishes.
  - `scan_within_extended_window` — the scan is older than the
    fresh window but inside the extended window.
  - `scan_past_extended_window` — the scan is older than the
    extended window; the inspector renders a stale banner.
  - `scan_in_flight` — a rescan is in flight; the card carries
    the prior scan's totals plus an in-flight indicator.
  - `scan_unknown_no_scan_yet` — no scan has produced a total
    yet; `total_used_bytes` MUST be `0` and every row's
    `class_used_bytes` MUST be `0`.
- `last_full_scan_at` — RFC 3339 UTC timestamp from a monotonic
  clock. Required non-null for every freshness class except
  `scan_unknown_no_scan_yet`; required null for that class.
- `partial_scan_reason_class` — closed set:
  - `not_partial_full_scan_complete` — the scan covered every
    inspectable surface for the scope.
  - `partial_scope_filter_applied` — the scan was bounded by an
    explicit scope filter (workspace_only, workset_only, etc.);
    the inspector reports only the in-scope total.
  - `partial_scan_in_progress` — the scan is in flight and the
    in-flight surface is reported as a partial.
  - `partial_due_to_low_disk` — the scan was throttled under
    disk pressure; the inspector renders a banner.
  - `partial_due_to_quota_or_throttle` — the scan was throttled
    by a quota or rate-limit policy.
- `low_privilege_scan_reason_class` — closed set:
  - `not_low_privilege_full_inspection` — the scan ran with full
    inspection privilege.
  - `low_privilege_user_only_scan` — the scan ran as the user
    and could not enumerate machine-scope or admin-scope stores.
  - `low_privilege_managed_tenant_restriction` — managed tenant
    policy restricted enumeration; the inspector renders the
    restriction reason verbatim.
  - `low_privilege_admin_consent_required` — admin consent is
    required to enumerate the protected scope; the inspector
    surfaces a request-consent affordance, never silent zeros.
  - `low_privilege_offline_scope_only` — the scan ran in air-
    gapped or offline mode and could not query a managed
    quota source.
- `unscannable_class_ids[]` — array of `storage_class_id` values
  the scan could not enumerate at all. The inspector MUST list
  these as `posture = missing` with `class_used_bytes = 0` and
  surface the unscannable reason in the row's `note` field;
  hiding an unscannable class is non-conforming.

The schema enforces:

- `scan_in_flight` requires `partial_scan_reason_class` to be
  `partial_scan_in_progress`.
- `partial_due_to_low_disk` requires the parent card to cite a
  `linked_low_disk_drill_ref` so the partial scope is auditable.
- Any non-`not_low_privilege_full_inspection` class on a card
  whose `inspector_scope.scope_class` is `device_total` or
  `tenant_only` MUST be reflected in the card's
  `consumer_surfaces[]` by including
  `admin_storage_console` so the surface that can request
  consent is named.

## 7. Cross-record invariants

The schema enforces these invariants mechanically. A surface that
violates any of them is non-conforming.

1. **Authoritative state is never collapsed into disposable
   total.** A card's `total_used_bytes` is split across the in-
   scope class rows, and every row preserves its
   `authority_class` and `clear_cache_protection_class`. A card
   that aggregates `user_owned_recovery_state` or
   `evidence_support_cache` bytes into a single "cache" row is
   non-conforming.
2. **Mirrored, imported, and policy-protected artifacts cannot
   appear as disposable.** A row whose
   `mirror_or_import_origin_class` is `mirrored_copy`,
   `offline_bundle_local`, `vendor_signed_offline_local`,
   `customer_signed_mirror_local`, or
   `policy_protected_admin_artifact` MUST cite a non-null
   linkage (mirror artifact row ref or class-specific review
   ref) and MUST NOT carry
   `clear_cache_protection_class = generic_clear_always_allowed`.
3. **Protected classes stay visible.** Every card lists both
   `evidence_support_cache_visible` and
   `user_owned_recovery_state_visible` in
   `protected_class_visibility[]` whenever its scope covers the
   class, even when the class's bytes are zero. The inspector
   renders the row to expose retention and recovery posture.
4. **Card total equals row sum.** The card's `total_used_bytes`
   MUST equal the sum of `class_used_bytes` across the card's
   class breakdown rows, and the card's `largest_consumers[]`
   MUST sum to ≤ `total_used_bytes` (the largest-consumer
   projection MAY undercount because of the bounded list size,
   but it MUST NOT overcount).
5. **Reclaimable / protected / pinned arithmetic.** For every
   row, `protected_bytes + pinned_bytes + reclaimable_bytes_
   estimate ≤ class_used_bytes`. The remainder (bytes neither
   protected, pinned, nor reclaimable today) is the
   class's reserved-or-in-flight bytes; surfaces MAY render that
   remainder but MUST NOT label it reclaimable.
6. **Stale, partial, and low-privilege scans are surfaced, never
   hidden.** A card whose `scan_freshness_class` is
   `scan_past_extended_window` MUST set
   `consumer_surfaces[]` to include `storage_inspector` (or its
   substitute) and MUST NOT include `low_disk_banner`; a stale
   scan cannot drive low-disk action. A card whose
   `partial_scan_reason_class` is non-`not_partial_full_scan_
   complete` MUST include a `linked_low_disk_drill_ref` (when
   reason is `partial_due_to_low_disk`) or a non-empty
   `unscannable_class_ids[]` (when reason is
   `partial_due_to_quota_or_throttle` or
   `partial_scope_filter_applied`).
7. **Low-privilege scans surface a consent affordance.** A card
   whose `low_privilege_scan_reason_class` is
   `low_privilege_admin_consent_required` MUST include
   `admin_storage_console` in `consumer_surfaces[]`. A card
   whose reason is `low_privilege_managed_tenant_restriction`
   MUST cite a `linked_deployment_summary_card_ref` so the
   tenant restriction posture is inspectable.
8. **Workspace and profile scope rows do not bleed.** Every
   breakdown row's `class_scope` is value-equal to the card's
   `inspector_scope`. A `workspace_only` card cannot embed a
   `device_total` row; a `tenant_only` card cannot embed a
   single workspace's row without an explicit
   `workspace_only` sibling card.
9. **Pinned bytes name a source.** A row whose `pinned_bytes`
   is non-zero MUST carry a non-empty
   `pinned_consumer_breakdown[]` whose entries' bytes sum to
   `pinned_bytes`. Pinned bytes without a source breakdown are
   non-conforming.
10. **Inspect-only actions stay inspect-only.** The
    `open_details_action` on the card and every
    `inspect_only_open_action` on every largest-consumer row
    MUST declare exactly `["no_side_effect_inspect_only"]`.
    Stacking another effect on any of those actions is non-
    conforming.
11. **Export discipline preserves redaction.** A card or row
    exported under `export_safe = true` MUST keep its
    `redaction_class` ≤ `operator_only_restricted`. Exports
    that widen redaction to `internal_support_restricted` or
    `signing_evidence_only` are non-conforming.
12. **Authority-aware ordering is persisted.** The
    `largest_consumers[]` list on the card and on every row is
    persisted in the §5.4 order; surfaces consume the order
    verbatim and MUST NOT re-sort by raw bytes.

## 8. Workspace / profile / tenant scope rules

The acceptance criterion *users can understand total disk use and
major storage classes without guessing which workspace or profile
owns them* resolves through:

- The `inspector_scope` block on the card naming exactly one
  scope. Every row's `class_scope` is value-equal to the card's
  scope; a workspace card's rows cannot bleed into another
  workspace.
- The `largest_consumers[]` rows naming a per-consumer scope by
  citing `consumer_ref` (workspace id, workset id, profile id,
  tenant id, slice id, or pack digest). A consumer whose
  `consumer_class` is `workspace_index_corpus`,
  `workspace_history_lane`, `workspace_recovery_journal`, or
  `workset_session_restore_state` carries a workspace or workset
  ref; a `mirror_snapshot_segment` or `offline_bundle_segment`
  carries the segment ref minted by the deployment-summary
  contract.
- The card-level `quota_or_policy_source.quota_authority_class`
  naming who set the ceiling so a workspace card whose total is
  bounded by an admin policy bundle is explicit about the
  authority that bounded it.

A surface that renders "your data" without naming the workspace,
profile, or tenant scope is non-conforming when a more precise
scope is knowable.

## 9. Disposable / rebuildable / authoritative posture

The acceptance criterion *inspector totals and rows preserve the
difference between disposable, rebuildable, and authoritative
state* resolves through the §4.1 required fields and the §7
invariants. In particular:

- `disposable` rows carry `authority_class =
  disposable_derived_cache` and any rebuildable cost class.
- `rebuildable` rows are a subset of disposable; they carry
  `rebuild_cost_class ∈ {high_rebuild_cost, medium_rebuild_cost,
  low_rebuild_cost}`. The inspector renders the cost so a
  preview can warn about a slow rebuild.
- `authoritative` rows carry `authority_class ∈
  {user_owned_recovery_state, admin_or_control_artifact}` and
  `rebuild_cost_class = authoritative_no_rebuild`. The inspector
  refuses to offer them as targets of a generic clear; the
  class-specific review sheet is the only admissible path.

A row whose authority is `user_authored_durable_truth` is
schema-denied (the cache manager refuses to register it; the
storage path routes through the profile/state-map storage path,
not through the inspector card).

## 10. Mirrored / imported / policy-protected artifacts

The acceptance criterion *mirrored, imported, and policy-protected
artifacts appear in storage totals without being mistaken for
disposable cache* resolves through:

- §4.3's `mirror_or_import_origin_class` vocabulary, which names
  every non-disposable origin explicitly.
- §4.1's `mirror_offline_artifact_row_ref` field, which links
  every mirror / offline-bundle row to its
  `mirror_offline_artifact_row_record` from the deployment-
  summary contract so the inspector and About / diagnostics /
  support agree on signer / digest / freshness posture for the
  same bytes.
- §7.2's invariant forbidding generic-clear-always-allowed
  protection on mirrored / imported / policy-protected rows.
- The breakdown row's `notes` field surfacing a sentence in
  product terms (e.g. "Mirrored docs pack from customer mirror;
  removing requires a class-specific review.") instead of a
  generic cache label.

## 11. Stale, partial, and low-privilege scans

The acceptance criterion *stale, partial-scan, and low-privilege
states keep storage reporting honest when full inspection is
unavailable* resolves through:

- §6's `scan_posture` block freezing every reason a scan may not
  be fresh, full, or fully privileged.
- §7.6 / §7.7's invariants forbidding stale scans from driving
  low-disk action and forbidding low-privilege scans from being
  rendered without a typed consent or restriction affordance.
- §7.6's `unscannable_class_ids[]` field forcing unscannable
  classes to render as `posture = missing` with the reason in the
  row's `note` rather than silently omitted.

## 12. Adding or changing vocabulary

Adding a value to any vocabulary frozen in this contract
(`mirror_or_import_origin_class`, `consumer_class`,
`pin_summary_class`, `scan_freshness_class`,
`partial_scan_reason_class`, `low_privilege_scan_reason_class`,
`quota_authority_class`, `consumer_surface_class`,
`protected_class_visibility`) is **additive-minor** and requires:

1. Updating the schema enum in
   `schemas/storage/storage_inspector_card.schema.json` or
   `schemas/storage/storage_class_breakdown.schema.json`.
2. Updating this document.
3. Adding or updating a fixture under
   `fixtures/storage/storage_inspector_cases/` exercising the
   new value.
4. Bumping the corresponding `*_schema_version` integer.

Repurposing an existing value is **breaking** and requires:

1. A new decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecation of the old value, addition of the new value
   through an additive-minor landing, and a translation pass on
   the inspector, workspace-storage detail, clear-data review,
   pin manager, low-disk banner, cleanup-history lane, support
   bundle, admin console, and CLI consumers across the
   deprecation window.

Vocabularies that re-export from upstream seeds (storage-class
matrix, authority class, rebuild-cost class, GC-policy class,
storage-posture class, quota-basis class, pin-source class,
inspectable-surface class, clear-cache protection class, mirror
freshness class, signer state class, digest state class) follow
the upstream change rules; this contract follows in the same
change.

## 13. Out of scope at this revision

- Final inspector / workspace-storage detail / clear-data review /
  low-disk banner / pin manager / cleanup-history lane / admin
  storage console layout, animation, and accessibility wiring.
  The contract pins the record family; the rendering surfaces own
  their own component contracts.
- Pixel-perfect chart and bar layout, the on-disk byte format of
  any store, and the live disk scanner / quota accountant.
- Localization-ready string tables; the contract carries
  reviewable English sentences and the localization layer is
  consumed separately.
- The class-specific review sheet for `evidence_support_cache`
  and `user_owned_recovery_state`. The cache-manifest schema
  freezes the review record; the inspector links it by ref.
