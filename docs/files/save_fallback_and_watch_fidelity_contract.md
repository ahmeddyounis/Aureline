# Save-fallback detail, watch-fidelity downgrade, and metadata-preservation contract

This contract freezes filesystem-degradation truth so a successful save
never hides weakened guarantees on the active root. It composes with
the existing save-target-token contract, the editor external-change
contract, and the autosave / journal recovery contract; it does not
replace them. It pins three things those contracts currently leave
implicit:

1. the **watch-fidelity** state the user is operating under, and the
   downstream truths (file-tree freshness, quick-open hits, search
   index, external-change detection, compare-before-write evidence)
   that may now lag because of it;
2. the **save-fallback detail row** every save / autosave / apply
   surface emits alongside the save-target token, capturing atomic
   replace availability, metadata preservation, compare-before-write
   posture, crash-journal guarantee, root class, canonical object
   identity, and presentation path in one record; and
3. the **first recovery surface** the product offers for each of the
   five scenarios the contract covers — save conflict, external change,
   cross-root rename, archive-backed edit attempt, and watch-fidelity
   downgrade — typed so none collapses into a generic "save failed" or
   "file missing" string.

Machine-readable companions:

- [`/schemas/files/watch_fidelity_state.schema.json`](../../schemas/files/watch_fidelity_state.schema.json)
  — the closed watch-fidelity state set, the downstream-truth lag
  vector, the watcher-source taxonomy, and the export-safe boundary
  the file tree, quick open, search index, external-change pipeline,
  and support bundle all read.
- [`/schemas/files/save_fallback_detail.schema.json`](../../schemas/files/save_fallback_detail.schema.json)
  — the save-fallback detail row, including every degraded-guarantee
  field the active save must disclose before bytes are written, the
  five typed recovery surfaces, and the support-export contract.
- [`/fixtures/files/save_fallback_cases/`](../../fixtures/files/save_fallback_cases/)
  — reviewable fixtures for the five required cases: local atomic-
  write downgrade, remote polling fallback, provider-backed compare-
  before-write, archive-backed edit block, and cross-root rename
  review.

This contract composes with:

- [`docs/io/save_target_token_and_write_guarantee_contract.md`](../io/save_target_token_and_write_guarantee_contract.md)
  for save-target tokens, write modes, save guarantee classes,
  compare-before-write phase ordering, and the support/export packet.
- [`docs/ux/editor_external_change_contract.md`](../ux/editor_external_change_contract.md)
  for external-change state classes, watcher uncertainty, and the
  review-choice matrix (`compare`, `overwrite`, `merge`, `cancel`,
  `reload`, `retry`, `save_as`).
- [`docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md)
  for the five-layer identity model (presentation path, logical
  workspace identity, canonical filesystem object, alias set, save-
  target token) and the semantic-readiness vocabulary.
- [`docs/fs/path_truth_packet.md`](../fs/path_truth_packet.md) for the
  path-truth chip, alias-inspector view, and difficult-case fixtures.
- ADR 0006 (`docs/adr/0006-vfs-save-cache-identity.md`) for the
  underlying VFS, watcher, save-pipeline, root-capability, and cache-
  identity rules.

Where this contract disagrees with the authoritative sources above,
those sources win and this packet, the schemas, and the fixtures
update in the same change. This contract does not mint a new
`alias_kind`, `root_class`, `safe_next_action`, or `save_guarantee_class`
vocabulary; it projects them.

## Scope

Frozen here:

- the four-class `watch_fidelity_state` vocabulary and the closed
  `watcher_source` taxonomy that produces it;
- the closed `lagging_downstream_truth` vector listing exactly which
  product truths may lag under each fidelity state;
- the `save_fallback_detail_row` shape and the rules every save /
  autosave / apply surface follows when emitting it;
- the `recovery_surface_kind` vocabulary and the rule that the five
  spec-named scenarios route to typed surfaces, not generic copy; and
- fixture coverage for local atomic-write downgrade, remote polling
  fallback, provider-backed compare-before-write, archive-backed edit
  block, and cross-root rename review.

Out of scope:

- implementing filesystem backends or OS watcher integrations;
- final platform-specific durability or fsync strategy;
- the VFS prototype harness (the fixtures are reviewer-facing YAML);
- visual chrome layout, copy localization, and platform-specific
  iconography;
- the full set of provider-mounted root adapters and their conditional-
  write semantics; and
- replacing or duplicating the save-target token, save-guarantee class,
  external-change state class, or readiness vocabulary frozen
  elsewhere.

## 1. Watch-fidelity state

Watch-fidelity names how trustworthy "I would notice if this file
changed outside Aureline" is right now on the active root. Every
surface that displays freshness chips, decides whether to rerun a
producer, gates a save, or exports support evidence reads exactly
one `watch_fidelity_state` per root and per canonical object.

### 1.1 Closed state set

| State                  | Meaning                                                                                                                                                                       |
|------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `live_watch`           | A native watcher is attached to the canonical object and its containing scope, with proven event continuity. External changes surface in milliseconds and compare-before-write rides on the watcher's evidence. |
| `reduced_fidelity_watch` | A native watcher is attached but with reduced coverage: coalesced events, dropped sub-tree coverage, attribute-only events, missing rename pairing, or a degraded backend that still proves at-least-once delivery within a bounded window. |
| `polling_fallback`     | The native watcher is unavailable or refused; the VFS falls back to bounded polling. External changes surface only on the next poll cycle and compare-before-write is the correctness floor for every write. |
| `manual_refresh_only`  | Neither watcher nor polling is admissible (no entitlement, root disconnected, archive view, fully sandboxed provider, user-disabled watch). The user must trigger refresh, reload, or compare to learn anything new. |

`live_watch` is the only state that may ride with
`freshness_floor = authoritative` on a save-target token without
adding a recompare hop. Every other state forces the save pipeline
to either re-read the strongest generation token immediately before
write (compare-before-write floor) or, where the root cannot provide
one, downgrade the save guarantee class.

### 1.2 Watcher source taxonomy

Each `watch_fidelity_state` cites exactly one `watcher_source` that
produced it. The set is closed:

- `os_native_inotify`
- `os_native_kqueue`
- `os_native_fsevents`
- `os_native_readdirectorychangesw`
- `polling_scanner_local`
- `remote_agent_event_stream`
- `remote_agent_polling_proxy`
- `provider_change_feed`
- `provider_polling_proxy`
- `archive_static_snapshot`
- `none_user_disabled`
- `none_entitlement_missing`
- `none_root_disconnected`
- `unknown`

`unknown` is admissible only on imported / replayed records; live
records resolve to a concrete source.

### 1.3 Lagging-downstream-truth vector

When the state is anything other than `live_watch`, the surface
records which downstream product truths may now lag. The vector is
closed; surfaces may not invent local labels.

- `file_tree_row_freshness`
- `quick_open_results`
- `search_index_freshness`
- `external_change_detection`
- `dirty_buffer_external_diff`
- `autosave_correctness_floor` (compare-before-write becomes the
  only proof that the buffer is not stale)
- `formatter_or_refactor_apply_basis`
- `language_server_disk_view`
- `task_runner_input_freshness`
- `git_status_view`
- `provider_revision_view`
- `cross_root_alias_resolution`
- `metadata_attribute_freshness` (xattrs, ACLs, mode bits, executable
  bit, mtime, ctime when the watcher does not deliver them)

Rules:

1. `live_watch` MUST publish an empty lag vector. A surface that
   shows `live_watch` while quoting any lag entry is non-conforming.
2. `reduced_fidelity_watch` MUST publish at least one lag entry and
   cite a `reduced_fidelity_reason` from §1.4.
3. `polling_fallback` MUST publish `external_change_detection` and
   `autosave_correctness_floor` at minimum. The poll cadence is
   recorded in `polling_interval_seconds`.
4. `manual_refresh_only` MUST publish every entry the surface knows
   may lag for the active root; the recovery surface explains how to
   refresh.

### 1.4 Reduced-fidelity reason vocabulary

When the state is `reduced_fidelity_watch`, the row cites exactly
one reason from the closed set:

- `event_coalescing_active`
- `subtree_coverage_dropped`
- `attribute_only_events`
- `rename_pairing_missing`
- `backend_degraded_at_least_once_window`
- `provider_change_feed_throttled`
- `remote_agent_partial_event_stream`

Adding a new reason is additive-minor; repurposing one is breaking.

### 1.5 Watch-fidelity record fields

Every emitted record carries:

- `record_kind = watch_fidelity_state_record`
- `schema_version = 1`
- `record_id` — opaque stable id.
- `observed_at` — producer-local timestamp.
- `state` — one of §1.1.
- `watcher_source` — one of §1.2.
- `reduced_fidelity_reason` — one of §1.4 (required when state is
  `reduced_fidelity_watch`, null otherwise).
- `polling_interval_seconds` — required when state is
  `polling_fallback`; null otherwise.
- `lagging_downstream_truths` — array drawn from §1.3.
- `root_class` — pass-through of the save-target-token root class.
- `presentation_path_ref` — opaque ref to the presentation path.
- `canonical_object_ref` — opaque ref to the canonical filesystem
  object.
- `last_proven_consistent_at` — timestamp the watcher last proved
  consistency, or null.
- `support_export` — packet family, redaction policy, parity
  signature.

Rules (frozen):

1. The record never inlines raw paths, raw URIs, raw provider URLs,
   or raw credentials; only the opaque refs and the closed
   vocabularies above cross this boundary.
2. The record is the same shape on the file-tree row, the editor
   chrome chip, the support bundle, and the CLI `doctor`-equivalent.
3. A surface that fetches a record and then renders a different
   state-class label for the user than for the support bundle is
   non-conforming.

## 2. Save-fallback detail row

The save-fallback detail row is the single record every save /
autosave / apply surface emits before bytes are written, alongside
the save-target token. It is the row support reads when a save
"succeeded" but the guarantees on the active root were weaker than
the user expected.

The row composes with — and references by opaque ref — the existing
`save_target_token_packet`. It does not duplicate token internals; it
makes the **degraded-guarantee summary** explicit so a successful
save cannot hide a weaker promise.

### 2.1 Required fields

Every row carries these field groups:

| Field group                          | Purpose                                                                                                                                                                |
|--------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `record_id`                          | Stable opaque diagnostic handle.                                                                                                                                       |
| `schema_version` (`= 1`)             | Version of this row contract.                                                                                                                                          |
| `emitted_at`                         | Producer-local timestamp.                                                                                                                                              |
| `save_target_token_ref`              | Opaque ref to the matching `save_target_token_packet.packet_id`. The row exists in tandem with that packet.                                                            |
| `presentation_path`                  | Pass-through of the layer-1 record. Never silently rewritten to the canonical URI.                                                                                     |
| `canonical_object_identity`          | Pass-through of the canonical filesystem object (canonical URI, root class, normalization form, identity token).                                                       |
| `root_class`                         | Pass-through of `root_class`.                                                                                                                                          |
| `atomic_replace_availability`        | Closed enum: `available`, `degraded_to_in_place_write`, `degraded_to_whole_file_rewrite`, `degraded_to_conditional_remote_write`, `unavailable_blocked`.                |
| `metadata_preservation`              | Closed-shape block (§2.2) covering POSIX mode, xattrs, ACLs, alternate data streams, executable bit, file times, sparseness, and resource forks.                       |
| `compare_before_write_posture`       | Closed enum: `live_token_match_required`, `recompare_required_each_write`, `provider_precondition_required`, `metadata_then_content_hash_recompare`, `unavailable_blocks`. |
| `crash_journal_guarantee`            | Pass-through of the recovery checkpoint posture vocabulary (`recovery_journal`, `local_history_checkpoint`, `workspace_checkpoint_group`, `provider_revision_checkpoint`, `metadata_only_checkpoint`, `not_required_no_write`, `unavailable_blocks`). |
| `watch_fidelity_ref`                 | Opaque ref to the active `watch_fidelity_state_record`.                                                                                                                |
| `save_guarantee_class`               | Pass-through of the save-guarantee class vocabulary.                                                                                                                   |
| `degraded_guarantee_disclosed`       | Boolean. MUST be `true` whenever any field above degrades from the strongest available promise on this root class.                                                     |
| `recovery_surface`                   | Closed-shape block (§3) describing the first recovery surface offered if the user takes action; null when none is appropriate.                                         |
| `support_export`                     | Packet family, redaction policy, included-fields list, parity signature.                                                                                               |

### 2.2 Metadata preservation block

The metadata preservation block is the explicit answer to "what
about my file did the save preserve". Each axis is a closed enum.
The row never renders "see logs" in place of the explicit axis.

| Axis                          | Closed values                                                                                                                                          |
|-------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
| `posix_mode_preservation`     | `preserved`, `preserved_with_umask_clamp`, `preserved_via_root_capability`, `not_applicable_on_root`, `dropped_during_atomic_replace`, `lost_due_to_provider_rewrite`, `unknown`. |
| `posix_owner_group_preservation` | `preserved`, `preserved_via_root_capability`, `not_applicable_on_root`, `dropped_during_atomic_replace`, `lost_due_to_provider_rewrite`, `unknown`. |
| `xattr_preservation`          | `preserved`, `partial_known_set_preserved`, `not_applicable_on_root`, `dropped_during_atomic_replace`, `lost_due_to_provider_rewrite`, `unsupported_by_root`, `unknown`. |
| `acl_preservation`            | `preserved`, `preserved_via_root_capability`, `not_applicable_on_root`, `dropped_during_atomic_replace`, `lost_due_to_provider_rewrite`, `unsupported_by_root`, `unknown`. |
| `alternate_data_stream_preservation` | `preserved`, `not_applicable_on_root`, `dropped_during_atomic_replace`, `unsupported_by_root`, `unknown`.                                       |
| `executable_bit_preservation` | `preserved`, `preserved_via_root_capability`, `dropped_during_atomic_replace`, `lost_due_to_provider_rewrite`, `not_applicable_on_root`, `unknown`.    |
| `file_times_preservation`     | `mtime_preserved_ctime_updated`, `mtime_updated_to_now`, `mtime_provider_assigned`, `unknown`.                                                         |
| `sparseness_preservation`     | `preserved`, `densified_during_atomic_replace`, `not_applicable_on_root`, `unsupported_by_root`, `unknown`.                                            |
| `resource_fork_preservation`  | `preserved`, `not_applicable_on_root`, `dropped_during_atomic_replace`, `unsupported_by_root`, `unknown`.                                              |

Rules (frozen):

1. A save that flips `executable_bit_preservation` to anything other
   than `preserved` MUST set `degraded_guarantee_disclosed = true`.
   Quietly losing the executable bit is the canonical "weakened
   guarantee hidden behind save success" case.
2. `xattr_preservation = partial_known_set_preserved` MUST cite the
   preserved set in the support-export `metadata_notes` field.
3. The block is not a free-text essay. Every axis is one of the
   closed values above; "see logs" is not an admissible value.
4. Adding an axis or value is additive-minor; repurposing one is
   breaking.

### 2.3 Disclosure rules

1. **Save success does not equal full guarantee.** A save that
   degraded any axis (atomic replace availability, metadata
   preservation, compare-before-write posture, crash-journal
   guarantee) MUST set `degraded_guarantee_disclosed = true` and
   surface it before bytes are written when the degradation is
   detectable in advance, or in the post-save chip and the
   support-export packet when the degradation was only observable
   after write (rare; provider rewrites).
2. **The active root class is preserved.** Every recovery surface,
   every chip, and every export quotes `root_class` and
   `canonical_object_identity` verbatim. A surface that says "save
   failed" without naming the canonical object and root class is
   non-conforming.
3. **Watch fidelity rides on every row.** Every detail row carries
   `watch_fidelity_ref`. A row that omits the ref is denied with
   `watch_fidelity_ref_missing` at the export boundary.
4. **Compare-before-write floor is not optional.** A
   `compare_before_write_posture = unavailable_blocks` row MUST set
   `atomic_replace_availability = unavailable_blocked` and the
   recovery surface MUST be one of the typed §3 surfaces.

## 3. Recovery surfaces

Five scenarios are named in the spec; each routes to a typed
recovery surface. The vocabulary is closed and shared across the
editor, file tree, quick open, save dialog, CLI save row, and the
support-export packet.

### 3.1 Scenario-to-surface map

| Scenario                       | First recovery surface kind                  | Forbidden collapse                                                  |
|--------------------------------|----------------------------------------------|---------------------------------------------------------------------|
| Save conflict                  | `save_conflict_review`                       | "Save failed" alone; no compare basis cited.                        |
| External change                | `external_change_review`                     | Quietly reloading without recording the pre-reload buffer.          |
| Cross-root rename              | `cross_root_rename_review`                   | "File missing" or "save as" without naming the source / target root.|
| Archive-backed edit attempt    | `archive_inspect_only_handoff`               | "Read-only" without naming the archive overlay or `save_as` route.  |
| Watch-fidelity downgrade       | `watch_fidelity_downgrade_review`            | "Indexing…" or "Loading…" while a write would proceed without compare. |

### 3.2 Recovery surface block fields

Every recovery surface block carries:

- `record_kind = recovery_surface_record`
- `schema_version = 1`
- `surface_kind` — one of §3.1.
- `subject` — the identity layer record the surface is about
  (presentation path, canonical object identity, root class).
- `preserves_object_identity` — boolean. MUST be `true` for every
  admissible surface; set to `false` only on imported / replayed
  records where the canonical object identity could not be resolved,
  in which case the surface kind is `watch_fidelity_downgrade_review`
  with `manual_refresh_only` cited.
- `preserves_root_class` — boolean. Same rule.
- `offered_actions` — ordered array drawn from the typed action set
  in §3.3; at least one entry is required.
- `default_action` — one of `offered_actions`; required.
- `forbidden_actions` — array of typed actions that are explicitly
  forbidden for this surface, each with a reason from §3.4.
- `disclosure_text_ref` — opaque ref to the bounded explainer body
  (the body itself rides on the support-export, not inline).
- `support_export` — packet family, redaction policy, parity
  signature.

### 3.3 Typed action set

Closed; pass-through of the existing review-choice and safe-next-
action vocabularies, plus one addition for the cross-root rename
case.

- `compare`
- `overwrite`
- `merge`
- `cancel`
- `reload`
- `retry`
- `save_as`
- `open_alias_details`
- `open_save_target_review`
- `open_external_handoff` *(this is the term every typed action
  set uses for the "send this to the root's external tool" handoff
  — for example, opening the archive in the OS archive viewer or
  re-running the rename through the workspace's cross-root rename
  flow; the action keeps the root class explicit)*
- `acknowledge_degraded_guarantee`
- `no_action_required`

Rules:

1. Every recovery surface MUST offer at least one action. A surface
   with an empty `offered_actions` is denied with
   `recovery_surface_actions_missing`.
2. `save_conflict_review` MUST offer `compare` and at least one of
   `overwrite`, `merge`, `cancel`, `save_as`. `overwrite` is
   forbidden until compare has produced a basis or the user
   explicitly admitted no-compare with disclosure.
3. `external_change_review` MUST offer `compare`, `reload`, and
   `cancel`. `reload` is forbidden when the buffer is dirty without
   an explicit discard checkpoint.
4. `cross_root_rename_review` MUST offer `save_as` (under the new
   root) and `cancel`. `overwrite` is forbidden across roots; the
   surface MUST cite both the source and the target `root_class`.
5. `archive_inspect_only_handoff` MUST offer `save_as` and
   `open_external_handoff`. `overwrite`, `reload`, `merge`, and
   `retry` are forbidden until the user takes the file out of the
   archive overlay.
6. `watch_fidelity_downgrade_review` MUST offer `retry` (re-attach
   watcher) and `acknowledge_degraded_guarantee`. `overwrite` is
   forbidden while `compare_before_write_posture =
   unavailable_blocks` is active.

### 3.4 Forbidden-action reasons

Closed vocabulary explaining why a typed action is forbidden on a
specific recovery surface:

- `silent_overwrite_forbidden`
- `dirty_buffer_requires_explicit_discard`
- `cross_root_overwrite_forbidden`
- `archive_inspect_only_root`
- `policy_blocked`
- `watch_fidelity_unavailable_blocks_write`
- `compare_basis_missing`
- `binary_or_unmergeable`
- `provider_revision_required`
- `removable_volume_uncertain`
- `read_only_mount`

## 4. Surface composition rules

These rules apply to every surface that displays, exports, or reasons
about a save / autosave / apply on a degraded root.

1. **One row per attempted save.** The save-fallback detail row is
   emitted once per save / autosave / apply attempt. The same
   `record_id` survives a recompare, a review-choice, and a retry;
   it is not minted afresh when a compare-before-write loops.
2. **Every degraded axis is declared.** The row declares each axis
   (atomic replace availability, metadata preservation,
   compare-before-write posture, crash-journal guarantee) even when
   the axis is not degraded; "absent" is not the same as "preserved".
3. **Object identity is preserved through recovery.** The recovery
   surface block always cites `presentation_path`,
   `canonical_object_identity`, and `root_class`. A recovery flow
   that drops one of those refs is non-conforming.
4. **Watch fidelity is one record per root.** The product caches one
   `watch_fidelity_state_record` per root and republishes the ref;
   it does not mint a fresh state per surface. A surface that
   renders a state class derived from a different
   `watch_fidelity_state_record` than the editor chrome is
   non-conforming.
5. **The five spec scenarios are typed.** A save conflict surface
   MUST be `save_conflict_review`; an external change surface MUST
   be `external_change_review`; a cross-root rename surface MUST be
   `cross_root_rename_review`; an archive-backed edit attempt MUST
   be `archive_inspect_only_handoff`; a watch-fidelity downgrade
   review MUST be `watch_fidelity_downgrade_review`. Generic "save
   failed" or "file missing" labels are non-conforming.
6. **Support parity.** The same record exports through the support
   bundle, the mutation journal, and the replay capture with the
   same fields it shows in chrome. A redaction policy may hide
   value strings; it never drops the row's class fields.

## 5. Forbidden collapses

The following collapses are explicitly forbidden:

- `live_watch` while quoting any lag entry in
  `lagging_downstream_truths`.
- `reduced_fidelity_watch` without a typed
  `reduced_fidelity_reason`.
- `polling_fallback` without `polling_interval_seconds` and without
  `external_change_detection` in the lag vector.
- `manual_refresh_only` without offering a `retry` action on the
  recovery surface.
- A successful save that flips any metadata-preservation axis off
  `preserved` while leaving `degraded_guarantee_disclosed = false`.
- An archive-backed edit that surfaces "read-only" without naming
  the archive overlay or routing through
  `archive_inspect_only_handoff`.
- A cross-root rename that surfaces "file missing" without citing
  the source and target `root_class` on
  `cross_root_rename_review`.
- A `watch_fidelity_downgrade_review` that offers `overwrite` while
  `compare_before_write_posture = unavailable_blocks` is active.
- A save-conflict surface that offers `overwrite` without first
  producing a compare basis or recording an explicit no-compare
  disclosure.

## 6. Fixture coverage

The seed fixture corpus under
[`/fixtures/files/save_fallback_cases/`](../../fixtures/files/save_fallback_cases/)
covers the five spec scenarios. Each fixture is one YAML file that
validates against
[`/schemas/files/save_fallback_detail.schema.json`](../../schemas/files/save_fallback_detail.schema.json)
and embeds the active `watch_fidelity_state_record` validating
against
[`/schemas/files/watch_fidelity_state.schema.json`](../../schemas/files/watch_fidelity_state.schema.json).

| Fixture                                  | Scenario                                                                                                                       |
|------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| `local_atomic_write_downgrade.yaml`      | Local POSIX root degrades from atomic replace to in-place write because the same-volume sibling write is denied by the volume; metadata preservation flips xattrs to `partial_known_set_preserved`; the save succeeds but the row records `degraded_guarantee_disclosed = true`. |
| `remote_polling_fallback.yaml`           | Remote agent loses its event stream and the watcher fidelity drops to `polling_fallback`; compare-before-write becomes `recompare_required_each_write`; the recovery surface is `watch_fidelity_downgrade_review` and the file tree quotes the lag vector. |
| `provider_backed_compare_before_write.yaml` | Cloud-backed sync root requires a provider precondition (`provider_revision_token`); the save uses `remote_conditional_write` and metadata preservation flips POSIX mode to `not_applicable_on_root`. |
| `archive_backed_edit_block.yaml`         | User opens a file inside an archive overlay and attempts an edit; the save-fallback row reports `atomic_replace_availability = unavailable_blocked`; the recovery surface is `archive_inspect_only_handoff` offering `save_as` and `open_external_handoff`. |
| `cross_root_rename_review.yaml`          | A rename target lives on a different root class than the source; the save-fallback row pins both root classes; the recovery surface is `cross_root_rename_review` and `overwrite` is forbidden with reason `cross_root_overwrite_forbidden`. |

## 7. Acceptance

- A save success cannot hide weakened guarantees on the active
  root: every metadata-preservation, compare-before-write, atomic-
  replace, or crash-journal degradation forces
  `degraded_guarantee_disclosed = true`, with the same fields
  exported to support and rendered in chrome.
- Recovery surfaces preserve `presentation_path`,
  `canonical_object_identity`, and `root_class` before offering
  reload, compare, retry, save-as, or external handoff. The five
  spec scenarios route to typed surfaces and never collapse to
  generic "save failed" or "file missing" copy.
- The fixture corpus covers the five required scenarios and
  validates against the schemas in this packet.
- The vocabularies in §1–§3 compose with the existing save-target
  token, external-change, identity, and readiness contracts; no
  parallel dialect is minted.

## 8. Change management

- Adding a new `watch_fidelity_state`, `watcher_source`,
  `lagging_downstream_truth`, `reduced_fidelity_reason`,
  `metadata_preservation` axis or value, `recovery_surface_kind`,
  typed action, or forbidden-action reason is additive-minor and
  lands in this document, the schemas, and at least one fixture in
  the same change.
- Repurposing an existing value is breaking and requires a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- The save-target token, external-change state class, identity
  vocabulary, and readiness vocabulary win on any disagreement;
  this packet updates in the same change when they evolve.

## 9. Source anchors

- `.t2/docs/Aureline_PRD.md:1108` — virtual file system and file
  watching requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1628` —
  watcher model, filesystem identity, canonical path, and save
  coordination.
- `.t2/docs/Aureline_Technical_Design_Document.md:1478` — watcher
  and filesystem identity design.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:2013` — Partially
  ready, Degraded, Read-only degraded copy rules.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md:1125` —
  lifecycle state copy rules.
