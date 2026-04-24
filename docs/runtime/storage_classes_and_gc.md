# Storage classes, pinning, low-disk, and clear-cache safety contract

This document freezes the shared runtime contract Aureline uses to
decide **what a store is**, **who owns it**, **what happens under
disk pressure**, **what a clear-cache operation is allowed to
remove**, and **why a given entry is protected from garbage
collection**. It exists so search shards, graph indexes, model
packs, docs packs, prebuild layers, extension downloads, symbol
bundles, preview runtimes, crash envelopes, review packets, local
history, and rollback checkpoints cannot each invent their own
disk behavior or silently promote user-authored or evidence-bound
state into disposable cache.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document MUST be updated in the same change.

## Companion artifacts

- [`/artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml)
  — machine-readable storage-class matrix, pin-source vocabulary,
  invalidation-fingerprint vocabulary, per-class quota basis /
  eviction order / clear-cache admissibility / inspector-surface
  map, and the protected-class invariants.
- [`/artifacts/runtime/low_disk_drills.yaml`](../../artifacts/runtime/low_disk_drills.yaml)
  — machine-readable drill seed covering the low-disk ordering
  ladder, stale-index rebuild, pinned-evidence retention,
  mirror / offline artifact preservation, corrupt-cache targeted
  repair, and user-owned recovery state review.
- [`/schemas/runtime/cache_entry_manifest.schema.json`](../../schemas/runtime/cache_entry_manifest.schema.json)
  — boundary schema for the cache-entry manifest record, the
  clear-cache preview record, and the low-disk drill record every
  storage inspector, workspace-storage detail view, support
  export, and diagnostics surface reads.
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  — authority-class vocabulary (`user_authored_durable_truth`,
  `user_owned_recovery_state`, `admin_or_control_artifact`,
  `disposable_derived_cache`) this contract quotes rather than
  renames. The state map also names the durable locations every
  cache-manager store resolves against.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — re-exported `rebuild_cost_class`, `storage_posture_class`, and
  `gc_policy_class` vocabularies. This contract narrows those
  vocabularies onto the six runtime storage classes; it does not
  mint parallel rebuild-cost or GC-policy names.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  — recovery-ladder rung ids (`rung.cache_index_repair`,
  `rung.open_without_restore`, etc.) the clear-cache preview and
  the corrupt-cache drill cite when cleanup is the corrective
  path rather than a housekeeping path.
- [`/artifacts/release/cache_trust_classes.yaml`](../../artifacts/release/cache_trust_classes.yaml)
  — release-lane cache-trust vocabulary. This contract does not
  re-mint those classes; it pins where `artifact_cache` entries
  cite `cache_trust_class` refs for provenance.
- [`/docs/runtime/background_queue_contract.md`](./background_queue_contract.md)
  and
  [`/docs/runtime/resource_governor_contract.md`](./resource_governor_contract.md)
  — background-queue and resource-governor vocabularies low-disk
  drills cite when a drill pauses a workload lane or narrows a
  budget domain.

## Scope and authority

This contract does not introduce a second cache manager, GC engine,
or storage API. The runtime's shared cache manager remains the
canonical owner of:

- cache-entry admission, indexing, and quota accounting;
- low-disk eviction ordering and hysteresis;
- pin-ref resolution and ref-count bookkeeping; and
- the `storage_posture_class` every inspector, support export, and
  admin flow projects for a store.

This contract freezes four questions the general cache-manager
leaves at the class layer:

1. Which storage classes exist and which authority, rebuild-cost,
   sensitivity, quota-basis, and eviction-order each class carries.
2. Which manifest fields every cache-entry store MUST register at
   the cache manager so a generic inspector can reason about it
   without per-store plumbing.
3. What low-disk and clear-cache policies are admissible for each
   class — in particular, which classes are **protected** from
   generic clear-cache paths.
4. Which inspectable product surfaces project each class so later
   UX, support, admin, and diagnostics work cites one class
   vocabulary rather than inventing parallel surface-local
   dialects.

The eventual cache-manager Rust crate and the final storage-
inspector UI remain out of scope. This contract is the shape they
implement against.

## Shared vocabulary

### Storage classes

Six classes are frozen. No store may declare a seventh class; a
new class is an additive-minor change to
[`storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml)
and to the schema and requires a decision row.

- **`interactive_hot_cache`** — render atlases, viewport caches,
  quick-open hotsets, open-file syntax state, recent-query hot
  shards, ephemeral preview data. Disposable derived state.
- **`knowledge_cache`** — search shards, graph indexes, language
  snapshots, embeddings, docs indexes. Derived but correctness-
  relevant; rebuildable.
- **`artifact_cache`** — updates, extension packages, docs packs,
  model packs, symbol bundles, preview runtimes. Imported content-
  addressed artifacts; retained by digest refs.
- **`prebuild_environment_cache`** — container layers, toolchain
  packs, template expansions, dependency metadata, environment
  capsules. Derived / imported hybrid; retained by content digest
  and environment-capsule fingerprint.
- **`evidence_support_cache`** — crash envelopes, traces, review
  packets, validation artifacts, incident bundles, support
  exports. Durable but policy-bounded evidence.
- **`user_owned_recovery_state`** — local history, rollback
  checkpoints, dirty-buffer journals, session-restore state,
  deferred-intent outbox, terminal restore metadata. Authoritative
  user state the product holds on the user's behalf.

### Authority classes

Re-exported verbatim from
[`profile_and_state_map.md`](../state/profile_and_state_map.md).
Every cache-entry manifest names exactly one authority class:

- `user_authored_durable_truth`
- `user_owned_recovery_state`
- `admin_or_control_artifact`
- `disposable_derived_cache`

A cache-entry manifest for a `disposable_derived_cache` authority
MUST bind to `interactive_hot_cache`, `knowledge_cache`,
`artifact_cache`, or `prebuild_environment_cache`. A manifest for
`user_owned_recovery_state` authority MUST bind to
`user_owned_recovery_state` class. A manifest for
`admin_or_control_artifact` authority MUST bind to
`evidence_support_cache` or `artifact_cache`. A manifest for
`user_authored_durable_truth` authority MUST NOT be registered as
a cache entry at all — the cache manager refuses and the surface
routes through the profile / state-map storage path instead.

### Rebuild-cost classes

Re-exported from
[`schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json):

- `authoritative_no_rebuild`
- `high_rebuild_cost`
- `medium_rebuild_cost`
- `low_rebuild_cost`

Only `evidence_support_cache` and `user_owned_recovery_state` may
declare `authoritative_no_rebuild`. The four disposable or
derived classes MUST declare one of the three rebuildable costs.

### Sensitivity classes

Frozen here so cache stores and support-bundle redaction share one
vocabulary. Matches Appendix CY.2's memory / cache sensitivity
tiers:

- `t0_metadata_only`
- `t1_low_risk_derived`
- `t2_code_bearing_bounded`
- `t3_secret_adjacent_not_reusable_cache`

`t3_secret_adjacent_not_reusable_cache` is forbidden on every
class except `evidence_support_cache` (where the only admitted
contents are redacted crash envelopes under a reviewed export
flow). Registering a non-evidence cache with `t3` is non-
conforming.

### GC policies

Re-exported from
[`schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json):

- `never_gc_authoritative`
- `gc_on_version_replace`
- `gc_on_pressure_if_unpinned`
- `gc_on_case_close`
- `gc_on_explicit_reset_only`

Per-class admissibility is pinned in the matrix. In particular,
`user_owned_recovery_state` MAY declare only
`gc_on_explicit_reset_only` or `never_gc_authoritative`;
`evidence_support_cache` MAY declare only
`gc_on_case_close`, `gc_on_explicit_reset_only`, or
`never_gc_authoritative`. A disposable class declaring
`never_gc_authoritative` is non-conforming.

### Storage postures

Re-exported:

- `healthy`
- `rebuild_pending`
- `pressure_trimmed`
- `reset_candidate`
- `retained_for_evidence`
- `missing`

`retained_for_evidence` is admissible only for
`evidence_support_cache` and `user_owned_recovery_state` entries.

### Pin-ref sources

Every pinned entry carries at least one pin-ref from this closed
set, and names one of these sources in `pin_source_class`:

- `explicit_user_pin` — the user pinned the item via the pin
  manager or an equivalent affordance.
- `explicit_admin_policy_pin` — an admin policy pinned the item.
- `release_artifact_graph_ref` — a release manifest references
  the item by digest.
- `case_reference_ref` — an open support case references the item.
- `review_pack_ref` — a review pack references the item.
- `offline_bundle_ref` — an offline entitlement or mirror bundle
  references the item.
- `certified_archetype_or_template_ref` — a certified template or
  archetype pack references the item.
- `policy_bundle_last_known_good_ref` — the last-known-good
  admin policy bundle references the item.
- `support_export_assembly_ref` — a support-export assembly in
  flight references the item.
- `retention_window_ref` — a declared retention window for
  user-owned recovery state or evidence keeps the item alive.

Every pin ref names the referenced object, the pin source, the
expiry or policy window (nullable when the pin is retention-only),
and the unpin path. Surfaces render pin rows by projecting these
fields; they do not mint surface-local pin vocabularies.

### Invalidation fingerprints

Every cache entry that can affect correctness carries the
fingerprints below. A cache entry MAY omit a fingerprint only
when its class rationale in the matrix marks that fingerprint as
`not_applicable`. A correctness-affecting entry that omits an
applicable fingerprint is non-conforming.

- `workspace_identity_ref` — workspace / slice / workset identity
  the entry was produced for.
- `toolchain_env_capsule_hash` — environment-capsule fingerprint
  (toolchain version, build flags, platform) for prebuild and
  language caches.
- `producing_schema_version` — the producing worker's schema
  version at the time the entry was written.
- `provider_model_identity_ref` — provider / model / tenant
  identity when the entry depends on an external provider.
- `policy_epoch_ref` — admin policy epoch at production time.
- `aureline_compatible_version_range` — earliest and latest
  Aureline versions the entry is valid for. An entry whose current
  runtime falls outside this range MUST be labeled `stale` before
  reuse.

### Protected classes

Two classes are **protected** from generic clear-cache paths:

- `evidence_support_cache`
- `user_owned_recovery_state`

A generic clear-cache operation MUST NOT target entries in these
classes. A class-specific review flow (documented in §6) is the
only admissible path, and only when the user has explicitly
selected the class in the review sheet and the surface has
offered an export-before-delete affordance for
`authoritative_no_rebuild` entries.

Two more classes are **conditionally protected** from generic
clear-cache — not because the data is authoritative, but because
deletion has offline-continuity or startup-cost consequences:

- `artifact_cache` — generic clear-cache MUST declare that
  pinned entries (release artifact graph refs, offline bundle
  refs, certified template refs, policy-bundle last-known-good
  refs) are excluded; the preview names which artifacts were
  skipped.
- `prebuild_environment_cache` — generic clear-cache MUST
  declare the startup-cost or offline-continuity consequence and
  MUST NOT remove entries pinned by an active environment capsule
  or an offline bundle.

## Cache-entry manifest fields

Every cache-entry store registers these fields at the cache
manager on first write. The schema in
[`schemas/runtime/cache_entry_manifest.schema.json`](../../schemas/runtime/cache_entry_manifest.schema.json)
is the boundary shape; the storage inspector, workspace-storage
detail, clear-cache review, low-disk banner, pin manager, and
cleanup-history lane all project these fields.

- **`storage_class_id`** — one of the six frozen classes.
- **`store_id`** — opaque, stable id. Safe to log and safe on
  support exports.
- **`scope_class`** — one of `machine`, `user`, `workspace`,
  `tenant`, `slice`.
- **`scope_ref`** — opaque id within the scope class (for
  example, workspace id, workset id, tenant id). Null only for
  `machine`-scoped stores.
- **`rebuild_cost_class`** — frozen rebuild-cost vocabulary.
- **`sensitivity_class`** — frozen sensitivity-class vocabulary.
- **`authority_class`** — frozen state-authority vocabulary.
- **`producing_schema_version`** — integer schema version of the
  producing worker at write time.
- **`gc_policy_class`** — frozen GC-policy vocabulary.
- **`quota_basis_class`** — how this store's capacity is
  accounted: `per_workspace_quota`, `global_device_quota`,
  `per_class_ceiling`, `per_tenant_quota`, `policy_bound_evidence_quota`,
  `retention_policy_only`, or `digest_store_plus_class_ceiling`.
- **`pin_refs`** — array of pin-ref objects. Empty when the entry
  is unpinned.
- **`invalidation_fingerprints`** — map with the six invalidation
  fingerprint keys above. Each key carries a value or the
  sentinel `not_applicable`.
- **`inspectable_on_surfaces`** — declared list of the surface
  classes that may project this entry: subset of
  `storage_inspector`, `workspace_storage_detail`,
  `clear_data_review`, `low_disk_banner`, `pin_manager`,
  `cleanup_history_lane`. Every class MUST declare at least one
  surface; a class whose list is empty is non-conforming.
- **`posture`** — current `storage_posture_class`.
- **`last_access_at`** and **`last_write_at`** — monotonic
  timestamps.
- **`reclaimable_bytes_estimate`** — bytes a trim of this store
  would reclaim. Null when the store has not been sized yet.
- **`note`** — short, reviewable sentence. No raw payload bytes.

## Low-disk ordering and clear-cache safety rules

### Low-disk ordering

Under disk pressure the cache manager MUST shed in this order.
Reordering is non-conforming; skipping a step because the step's
candidates are empty is admissible and documented in the drill
record.

1. **Stop speculative fetch and prefetch.** The resource-governor
   background-queue lanes `provider_overlay` and `maintenance`
   narrow before any cache is trimmed. No cache entry is touched
   at this step.
2. **Pause managed replication and background pack refresh.** The
   `upload_replication` queue lane pauses; `artifact_cache`
   downloads in flight pause at their next chunk boundary.
3. **Trim `interactive_hot_cache` to low-water mark.** The
   cache manager uses hysteresis: trim to the declared low-water
   mark, not the high-water mark, so pressure does not oscillate
   on every small write.
4. **Trim rebuildable `knowledge_cache` entries to low-water
   mark.** Entries are labeled `pressure_trimmed` and the surfaces
   that depend on them project `stale` or `rebuild_pending` until
   a reindex or rehydrate completes.
5. **Trim rebuildable `artifact_cache` entries that carry no
   pin ref.** Pinned artifacts (release graph refs, offline
   bundle refs, certified template refs, policy-bundle last-known-
   good refs) are skipped and the skip reason is recorded.
6. **Trim rebuildable `prebuild_environment_cache` entries
   that carry no active-environment-capsule pin.** Skipped entries
   name the pin and the startup-cost or offline-continuity
   consequence that the pin prevents.
7. **Expire unpinned `evidence_support_cache` entries that are
   past their declared retention expiry.** Entries still inside
   their retention window, still referenced by a case / review /
   release / export ref, or still in-flight for a support export
   are skipped.
8. **Review user-owned recovery state only under an explicit
   class-specific review.** The cache manager MUST NOT trim
   `user_owned_recovery_state` entries as a side effect of low-
   disk pressure. If policy truly requires it, the flow routes
   through the clear-cache review sheet and names the consequence
   explicitly.

### Clear-cache preview semantics

A clear-cache operation is a typed review record before it is a
destructive action. Every clear-cache path MUST:

1. **Name the selected classes.** The review sheet lists every
   class the operation will touch. Surfaces that render a bare
   "Clear cache" button without class selection are non-
   conforming.
2. **Name the protected exclusions.** The review sheet lists
   every class the operation will **not** touch by default
   (always `evidence_support_cache` and `user_owned_recovery_state`
   for a generic clear-cache; additionally any pinned entries in
   `artifact_cache` or `prebuild_environment_cache`).
3. **Declare a reclaimable-bytes estimate.** The estimate sums
   the `reclaimable_bytes_estimate` of every targeted entry.
4. **Declare consequences.** The sheet names what will be rebuilt
   (`search and graph will rebuild`,
   `prebuilds will re-download on next start`), whether offline
   continuity is affected, whether certified workspace evidence
   or last-known-good policy bundles are at risk, and the expected
   restart cost.
5. **Offer a recovery affordance.** The review sheet offers a
   cancel path and, for any class carrying
   `authoritative_no_rebuild` entries, an export-before-delete
   affordance.
6. **Never silently widen.** A clear-cache path that widens from
   the declared class set — for example, trimming
   `evidence_support_cache` because `knowledge_cache` was
   selected — is non-conforming.

### Protected-class invariants

1. **No user-authored durable truth is categorized as cache.** A
   manifest registering an `authority_class` of
   `user_authored_durable_truth` is refused; the surface routes
   through the profile / state-map storage path instead.
2. **No release or evidence-bound state is categorized as
   disposable cache.** An entry pinned by a release artifact
   graph ref, review pack ref, case reference, or approval record
   MUST declare `evidence_support_cache` class unless it is a
   pinned `artifact_cache` entry whose deletion would not invalidate
   the referencing record. A generic clear-cache operation MUST
   skip both.
3. **User-owned recovery state is never generic cache.** A
   manifest declaring `user_owned_recovery_state` authority MUST
   bind to `user_owned_recovery_state` class and MUST declare
   `gc_on_explicit_reset_only` or `never_gc_authoritative` GC
   policy.
4. **Clear-cache is class-selective and previewable.** A blanket
   "destroy everything" action is not admissible from the
   clear-cache path. Total reset is a separate flow that routes
   through the recovery-ladder packet.
5. **Pinned entries name why.** Every pin ref carries a pin
   source, a referenced-object ref, an expiry or policy window
   (nullable only for retention-only pins), and an unpin path.

## Inspectable-surface mapping

Every storage class binds to at least one inspectable product
surface so later UX work cannot invent parallel class vocabularies.
The companion YAML pins the exact set per class; the surfaces are:

- **`storage_inspector`** — the global inspector. Every class
  binds here.
- **`workspace_storage_detail`** — per-workspace or per-slice
  detail. `knowledge_cache`, `artifact_cache`,
  `prebuild_environment_cache`, `evidence_support_cache`, and
  `user_owned_recovery_state` bind here. `interactive_hot_cache`
  may bind when the store is workspace-scoped.
- **`clear_data_review`** — the class-selective review sheet.
  Every class binds here; `evidence_support_cache` and
  `user_owned_recovery_state` bind only under explicit class
  selection.
- **`low_disk_banner`** — the pressure banner. Every class binds
  here because every class is named in the pressure ladder.
- **`pin_manager`** — the pin / retention manager. Classes that
  admit pins bind here: `artifact_cache`,
  `prebuild_environment_cache`, `evidence_support_cache`, and
  `user_owned_recovery_state`.
- **`cleanup_history_lane`** — the post-action attribution lane.
  Every class binds here; a cleanup event whose class set is not
  named in the lane is non-conforming.

Surfaces project manifest rows by `storage_class_id`, pin rows by
pin-ref, and cleanup events by the cleanup-history event shape in
the schema. They do not mint surface-local class or pin
vocabularies.

## Seeded drill scenarios

The companion YAML carries the machine-readable drill rows. The
families are frozen here so later implementations map into one
shape:

- **Low-disk ordered eviction.** A device drops below the low-
  disk threshold. The cache manager applies steps 1–6 of the
  low-disk ladder. `evidence_support_cache` and
  `user_owned_recovery_state` are not touched.
  `pin_manager` surfaces the skipped artifacts; the cleanup
  history lane records the reclaimed bytes per class.
- **Stale-index rebuild after schema drift.** A workspace's
  producing-schema version advances. Affected `knowledge_cache`
  entries flip to `rebuild_pending`; rehydrate runs under
  `hot_set_scan` and `graph_warmup` queue lanes. No generic
  clear-cache is invoked; the invalidation is fingerprint-
  driven.
- **Pinned evidence retention under pressure.** A device is
  under disk pressure with an open support case. Every
  `evidence_support_cache` entry pinned by a `case_reference_ref`
  is skipped; the cleanup history lane names the case id.
  Export-before-delete is offered in the clear-data review sheet
  for any entry in the case.
- **Mirror / offline artifact preservation.** A device holds an
  offline entitlement bundle and a mirrored docs pack. Under a
  class-selective `artifact_cache` clear, every entry pinned by
  `offline_bundle_ref` is preserved; the review sheet names the
  offline-continuity consequence that the pin prevents.
- **Corrupt-cache targeted repair.** A `knowledge_cache` shard
  is detected corrupt. The recovery ladder rung
  `rung.cache_index_repair` is cited; the repair rebuilds only
  the affected shard, not the class. `user_owned_recovery_state`
  is never touched.
- **User-owned recovery state under explicit review.** The user
  chooses "Delete local history for this workspace" in the
  class-specific review sheet. The sheet names the newest
  retained item, offers export-before-delete, and warns that the
  operation is irreversible for the selected class. A generic
  clear-cache path is refused.

The important property is not the exact drill numbers. The
property is that every class, every pin, every clear-cache
preview, and every low-disk decision resolves one frozen
vocabulary — not a per-subsystem invention — so persistence,
recovery, support, and release tasks reason about the same shape.

## Out of scope

- The concrete GC engine, hysteresis tuning, or on-disk layout of
  any store. The vocabulary freeze lands here; the cache-manager
  Rust crate is the schema of record when it lands.
- The final storage-inspector, clear-data review sheet, pin
  manager, or low-disk banner microcopy. Copy lives with the
  UX style guide and the shell-interaction safety contract; this
  document pins the closed sets that copy resolves against.
- Admin-policy authorship for retention windows or class caps.
  This document assumes those values flow in from the admin
  policy bundle at runtime and names the references rather than
  redefining them.
