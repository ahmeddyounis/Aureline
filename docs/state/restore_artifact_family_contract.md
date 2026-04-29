# Restore artifact family contract

This document freezes the cross-surface vocabulary every startup,
session-restore, crash-recovery, support-export, diagnostics, and docs
flow uses when it names **which part of a remembered restore packet is
authoritative restorable state**, **which part is window-local view
topology**, **which part is best-effort machine or display metadata**,
**which restore-fidelity class the packet claims**, and **which
placeholder or topology-adjustment behavior the user is seeing when the
exact dependency is missing**.

The artifact family lives above the per-artifact contracts. It does not
replace them; it gives the workspace-authority checkpoint, the
window-topology snapshot, and the restore-fidelity vocabulary one
shared boundary so a reviewer, support engineer, diagnostics flow, or
docs page can reason about restore mechanically instead of negotiating
parallel field names.

The machine-readable schemas live at:

- [`/schemas/state/workspace_authority_checkpoint.schema.json`](../../schemas/state/workspace_authority_checkpoint.schema.json)
- [`/schemas/state/window_topology_snapshot.schema.json`](../../schemas/state/window_topology_snapshot.schema.json)

Worked fixtures live under:

- [`/fixtures/state/restore_artifact_family/`](../../fixtures/state/restore_artifact_family/)

This contract composes with:

- [`/docs/state/workspace_memory_contract.md`](./workspace_memory_contract.md)
- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
- [`/docs/state/portable_state_package_contract.md`](./portable_state_package_contract.md)
- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md)
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)

The pane-tree body referenced by the window-topology snapshot family
packet is validated by the workspace-shell schema:

- [`/schemas/workspace/pane_tree.schema.json`](../../schemas/workspace/pane_tree.schema.json)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or one of the specialized contracts above, those documents
win and this contract plus its schemas MUST be updated in the same
change. Where a downstream restore, diagnostics, support, or docs
surface mints a parallel boundary, fidelity, or placeholder vocabulary,
this contract wins and the surface is non-conforming.

## Why freeze this now

Restore drift starts when authority and topology are flattened into one
opaque blob. Without an explicit family boundary:

- A startup surface promises `Exact restore` even when extensions,
  remote targets, or display topology forced the result to degrade.
- A diagnostics surface treats stale monitor coordinates as durable
  truth and "loses" panes that fell off-screen.
- A support export collapses dirty-buffer journal identity, trusted
  roots, and active worksets into a single payload that cannot be
  reasoned about field by field.
- A docs page describes a restore class that the schema cannot
  validate.

The artifact family freezes which fields belong to authority, which
belong to view topology, which are merely best-effort hints, and which
fidelity vocabulary every surface quotes verbatim. Once that boundary
is named, restore behavior becomes inspectable mechanically.

## Scope

- Freeze the workspace-authority checkpoint as the authoritative,
  restorable body: dirty-buffer/journal identity, trusted roots,
  active workset IDs, restore class, related evidence IDs, and
  excluded live-authority classes.
- Freeze the window-topology snapshot packet as the window-local
  view body: pane-tree schema version, stable pane IDs, tab/group
  topology, visible inspectors, focus chain, presentation/follow
  state, and monitor-affinity hints, plus the placeholder and
  topology-adjustment fields that keep restore spatially honest.
- Freeze one shared restore-fidelity vocabulary — `exact_restore`,
  `compatible_restore`, `layout_only`, `recovered_drafts`, and
  `evidence_only` — and pin the producer build, source schema version,
  and downgrade-trigger fields every fidelity claim carries.
- Freeze the placeholder-behavior rules for missing extensions,
  unavailable remote targets, off-screen or topology-adjusted geometry,
  and recenter / re-dock outcomes so restore stays spatially honest
  instead of silently collapsing layout.
- Freeze the rule that live capabilities (PTY handles, debug sessions,
  live remote bindings, browser session tokens, kernel handles, secret
  material) are remembered only as context references or evidence,
  never as restored live authority.

## Out of scope

- The persistence engine, sync execution, checkpoint storage layout,
  or restore runtime. The vocabulary freeze lands here; production
  surfaces compose over it later.
- Final UI copy. Display copy may render `Exact restore`,
  `Compatible restore`, `Layout only`, `Recovered drafts`, and
  `Evidence only`; the closed machine set is fixed.
- Repurposing of fields in the upstream layout-serialization or
  migration-restore contracts. This packet adds a family-level
  boundary; it does not edit the per-artifact bodies.

## 1. Family boundary

Every restore packet emitted under this contract MUST resolve every
field to exactly one of the four layers below. Flattening them into one
payload is non-conforming.

| Layer | Authority | Default portability | Where it lives |
|---|---|---|---|
| **Workspace-authority checkpoint** | the workspace instance | local-only or workspace-shared, never user-profile portable | `workspace_authority_checkpoint_record` (this contract) |
| **Window-topology snapshot** | one window | local-only by default; exportable only as an explicit topology bundle or diagnostic artifact | `window_topology_snapshot_record` (this contract); pane-tree body validated by the workspace-shell pane-tree schema |
| **Profile defaults** | the user profile | portable | the portable-profile contract; this packet only references defaults by ref |
| **Machine or display hints** | this machine | best-effort only | embedded inside the topology packet as `monitor_affinity_hint`; never authoritative |

Rules (frozen):

1. A workspace-authority checkpoint is the only body that may carry
   dirty-buffer/journal identity, trusted-root state, active workset
   IDs, related evidence refs, restore class, downgrade triggers, and
   the list of intentionally excluded live-authority classes.
2. A window-topology snapshot packet MUST point at exactly one
   workspace-authority checkpoint by `workspace_authority_checkpoint_ref`.
   Many topology snapshots MAY share one checkpoint ref when they
   belong to the same workspace instance.
3. A topology packet MUST NOT serialize a live capability ticket,
   delegated approval, secret reference body, kernel handle, or
   browser session token as if it were layout truth. References by
   opaque id are allowed; live authority bodies are not.
4. Profile defaults MAY be referenced from either body, but they MUST
   NOT overwrite an explicit restored topology silently. Any default
   the packet seeded into an omitted optional field is recorded by
   ref, not by inlined value.
5. `monitor_affinity_hint` is best-effort metadata only. A restore
   that no longer fits current monitors MUST snap to safe bounds and
   record the adjustment under `topology_adjustments`; treating stale
   coordinates as authoritative is forbidden.

## 2. Workspace-authority checkpoint

The workspace-authority checkpoint is the durable, restorable body. It
is the only field-set inside this family that may claim authority over
buffers, journals, save locks, trust state, and active worksets.

Required fields (frozen):

- `checkpoint_id` — opaque stable id.
- `workspace_authority_ref` — opaque ref for the workspace instance
  this checkpoint belongs to. Many checkpoints over time MAY share one
  authority ref.
- `producer_build` — producer name, version, channel, and platform
  class. Pseudonymous; never a raw hostname.
- `source_schema_version` — opaque schema-version string the producer
  used.
- `restore_class` — exactly one value from the closed restore-fidelity
  vocabulary in §4.
- `dirty_buffer_journal_identities[]` — one row per dirty-buffer or
  recovery-journal stream the checkpoint can rehydrate. Each row names
  a `journal_id`, `journal_kind`, and an opaque
  `last_known_revision_ref`. Raw journal bodies never appear inline.
- `trusted_root_refs[]` — one row per workspace root that carried a
  trust decision. Each row names a `root_id`, the ADR-0001
  `trust_state` reused without redefinition, and an opaque scope ref.
- `active_workset_ids[]` — opaque workset ids the checkpoint considered
  active when it was emitted.
- `recovery_journal_refs[]`, `local_history_snapshot_refs[]`,
  `evidence_bundle_refs[]` — opaque refs to related recovery journals,
  local-history snapshots, and evidence bundles. They are part of the
  authority story even when their bodies live elsewhere.
- `excluded_live_authority_classes[]` — typed enum of classes the
  producer intentionally left out (raw secret material, live tokens or
  cookies, delegated approvals or unspent tickets, machine-unique
  handles, live process or session handles, raw provider payloads, raw
  URL/path/command/log content, raw source or user content, live
  remote or kernel bindings). The list re-exports the
  workspace-memory-contract vocabulary verbatim so support and docs
  do not invent parallel labels.
- `downgrade_triggers[]` — one row per typed reason that the resulting
  fidelity narrowed (see §4). Empty when the checkpoint claims
  `exact_restore`.
- `emitted_at` — producer-local monotonic timestamp.

Rules (frozen):

1. The checkpoint never carries raw secrets, raw tokens, raw provider
   payloads, raw absolute paths, raw command lines, raw logs, or raw
   source content. The classes above admit refs and excluded-class
   inventories only.
2. The checkpoint is the **only** body inside this family that may
   list `excluded_live_authority_classes[]`. The window-topology
   packet inherits the inventory by reference.
3. A surface that claims authority restored is non-conforming if the
   checkpoint declares `restore_class = layout_only`,
   `recovered_drafts`, `evidence_only`, or `no_restore`. The
   topology packet is allowed to render placeholders even when the
   authority body itself restored cleanly.
4. The `downgrade_triggers[]` row vocabulary is the same enum the
   shared migration-and-restore playbook freezes for downgrade
   reasons. Free-form downgrade strings are non-conforming.

## 3. Window-topology snapshot packet

The window-topology snapshot packet is the window-local view body. It
is intentionally inventory-shaped so support, diagnostics, and docs can
reason about pane identity, tab grouping, inspectors, focus chain,
presentation state, and display hints without loading the full pane
tree.

Required fields (frozen):

- `snapshot_id`, `window_id` — opaque stable identifiers.
- `producer_build`, `source_schema_version`, `emitted_at` — same
  vocabulary as the authority checkpoint.
- `workspace_authority_checkpoint_ref` — opaque ref for the authority
  body this snapshot belongs to.
- `pane_tree_schema_version` — integer version frozen at `1` for the
  current pane-tree body shape.
- `pane_tree_record_ref` — opaque ref for the canonical recursive
  pane-tree body validated by
  [`/schemas/workspace/pane_tree.schema.json`](../../schemas/workspace/pane_tree.schema.json).
  The packet does not duplicate the recursive tree; it inventories the
  identifiers needed to reason about the boundary.
- `stable_pane_id_inventory[]` — one row per pane the producer
  remembered, naming `pane_id`, `surface_role`, `surface_class`,
  `availability_state`, `hydration_behavior`, and the
  `presentation_spotlighted` flag. The inventory uses the same closed
  enums as the workspace-shell pane-tree schema so cross-tool diff is
  possible without re-mapping.
- `tab_group_topology[]` — one row per tab group, naming `group_id`,
  `active_tab_id`, ordered `tab_ids[]`, and `pinned_tab_ids[]`.
- `visible_inspectors[]` — inspector inventory naming `inspector_id`,
  `inspector_kind`, `target_pane_ref`, `dock_position`, and `visible`.
- `focus_chain[]` — ordered focus-chain entries naming `target_kind`
  and `target_ref`.
- `follow_presentation_state` — follow mode, presentation mode,
  visible role badges, shared-control badge visibility, and audience
  breakaway policy. Topology only; never workspace authority.
- `monitor_affinity_hint` — best-effort display class, scale bucket,
  safe bounds hint, last-known display ref, and last-known topology
  hash. The `best_effort_only` flag is always true so producers cannot
  smuggle display geometry as durable truth.
- `placeholder_behaviors[]` — see §5.
- `topology_adjustments[]` — see §5.
- `restore_class` — same closed vocabulary as §4. The topology packet
  may declare a narrower class than the authority checkpoint (e.g.,
  `layout_only` while the authority restored cleanly). The reverse is
  forbidden.
- `downgrade_triggers[]` — same closed enum as §4. Empty when the
  packet claims `exact_restore`.

Rules (frozen):

1. The packet MUST NOT alter the workspace-authority body. It is a
   window-local view; authority truth lives in the checkpoint.
2. Closing, floating, moving, pinning, or replacing a pane mutates the
   topology snapshot for that window only. Sibling windows pointed at
   the same `workspace_authority_checkpoint_ref` are unaffected.
3. The packet MAY remember that a live pane existed, but it MUST NOT
   serialize live capability tickets, delegated approvals, kernel
   handles, or browser session tokens as topology fields. Live
   surfaces are remembered as inventory rows with the matching
   `placeholder_behavior` row when the dependency is unavailable.
4. `monitor_affinity_hint` is metadata only. A restored window that no
   longer fits current monitors MUST snap to safe bounds and record
   the adjustment under `topology_adjustments`. Restoring off-screen
   or unreachable geometry as durable truth is non-conforming.

## 4. Restore-fidelity vocabulary

The closed machine set is fixed. Display copy may render the title-case
labels shown below.

| Display label | Machine enum | Meaning | Authority/topology rule |
|---|---|---|---|
| `Exact restore` | `exact_restore` | every requested authority and topology component round-tripped without translation, placeholder, or review | both bodies declare `exact_restore`; downgrade triggers and placeholder rows are empty |
| `Compatible restore` | `compatible_restore` | one or more components translated through a declared compatibility path without blocking review | producer build and source schema version are recorded; downgrade triggers list the typed reasons; the migration-and-restore playbook's equivalence-map and rollback-checkpoint refs apply |
| `Layout only` | `layout_only` | window-local topology and stable pane IDs survived, but authority did not restore as live (or was never requested) | topology packet may declare `layout_only` while the authority checkpoint stays explicit about what it carried |
| `Recovered drafts` | `recovered_drafts` | dirty-buffer or local-history bodies were rehydrated as drafts that the user must compare or accept before they ride as workspace truth | authority checkpoint MUST list the affected `dirty_buffer_journal_identities[]` rows; topology packet pairs each rehydrated draft pane with a `placeholder_behavior` row when the live surface is degraded |
| `Evidence only` | `evidence_only` | no live restore was attempted; only transcripts, snapshots, refs, and provenance survive | every live surface row in the topology packet carries an `evidence_only_placeholder` posture; the authority checkpoint records the excluded live-authority classes that prevented a higher class |
| `No restore` | `no_restore` | the producer chose not to restore; only the inventory and provenance refs survive | both bodies record the typed reason under `downgrade_triggers[]` and exclude any pane/authority claim that would imply live restore |

Producer build and source schema version are mandatory on every claim
above `exact_restore`. The `downgrade_triggers[]` enum is closed and
shared across both bodies:

- `schema_translation_required`
- `schema_meaning_changed`
- `missing_extension_dependency`
- `missing_remote_session`
- `missing_remote_authority`
- `unsupported_display_topology`
- `excluded_secret_material`
- `excluded_live_handle`
- `workspace_manifest_conflict`
- `policy_narrowing`
- `manual_repair_required`
- `producer_schema_downgrade_refused`

Rules (frozen):

1. The label set is closed. A surface that invents `partial`,
   `best_effort`, or another parallel label is non-conforming.
2. `exact_restore` is forbidden once any component required
   translation, placeholder fallback, manual review, or any topology
   adjustment listed in §5.
3. The topology packet's `restore_class` MAY be narrower than the
   authority checkpoint's. The reverse is forbidden: a topology packet
   that declares `exact_restore` while the authority checkpoint
   declares `compatible_restore` or lower is non-conforming.
4. Producer build and source schema version stay carried on every
   record so a downgrade can be reproduced and a fixture can replay
   the same fidelity decision.

## 5. Placeholder and topology-adjustment behavior

Placeholders and adjustments keep restore spatially honest when the
exact dependency is missing. The window-topology packet records both as
inventory rows so support, diagnostics, and docs can describe what the
user is actually seeing.

### 5.1 Placeholder behaviors

Each row in `placeholder_behaviors[]` names exactly one closed
`placeholder_reason` and at least one safe action.

| Placeholder reason | When it applies | Required safe-action set | Forbidden behavior |
|---|---|---|---|
| `missing_extension` | extension or feature pack absent | `locate_extension`, `install_extension`, `open_without`, `export_evidence`, `remove_pane` | silently dropping the pane from the tree |
| `missing_remote` | remote target or connector unreachable | `reconnect_remote`, `reauthenticate`, `export_evidence`, `remove_pane` | silently reusing a stale route grant or showing an empty live pane |
| `missing_remote_authority` | remote authority revoked or expired | `reauthenticate`, `open_restricted`, `export_evidence` | hidden authority reacquisition |
| `revoked_permission` | capability ticket or delegated approval no longer valid | `reauthenticate`, `open_restricted`, `export_evidence` | reusing the revoked grant |
| `unsupported_display_topology` | preferred display unavailable, off-screen, or scale-mismatched | paired with one or more `topology_adjustments[]` rows; `reflow_to_safe_bounds` action set | restoring off-screen or unreachable geometry as durable truth |
| `non_reentrant_live_surface` | live PTY, debug session, kernel, or runtime cannot reattach safely | `rerun_explicitly`, `rebind_existing_session`, `export_evidence`, `remove_pane` | replaying the prior live action automatically |
| `schema_migration_review_required` | meaning changed and the migration stopped at review | `compare_with_preserved_artifact`, `open_repair_instructions`, `export_evidence` | hiding the schema shift behind a success badge |
| `manual_recovery_required` | no automatic class applies; narrowest safe repair only | `open_repair_instructions`, `escalate_to_manual_repair`, `export_evidence` | masking the repair step behind a generic Open button |

Rules (frozen):

1. Placeholder rows preserve the original `pane_id`, surface role, and
   tab/split slot. A new `pane_id` minted to "stand in" for a missing
   pane is non-conforming.
2. `evidence_retained` is required on every row so support knows
   whether a transcript, snapshot, or metadata-only summary survived.
3. The `safe_actions[]` set is closed and shared with the
   workspace-shell pane-tree schema. Free-form action labels are
   non-conforming.
4. A live-surface placeholder row MUST be paired with a typed
   no-rerun guardrail recorded in the layout-restore provenance the
   workspace-shell schema validates. This packet records the
   placeholder; the per-window provenance records the guardrails.

### 5.2 Topology adjustments

Each row in `topology_adjustments[]` names exactly one closed
`adjustment_class` and the affected pane or window scope.

| Adjustment class | When it applies | Required behavior |
|---|---|---|
| `snapped_to_safe_bounds` | saved geometry sat outside current safe bounds | clamp to safe bounds; preserve pane order |
| `moved_to_primary_display` | preferred display unavailable | move window to primary display; record affinity downgrade |
| `scale_normalized` | preferred display scale unavailable | normalize bounds for the new scale bucket; preserve pane order |
| `fullscreen_cleared` | saved fullscreen unsafe on current topology | exit fullscreen; record that the chrome state changed |
| `stacking_repaired` | window stacking would have placed a pane behind another window or off-screen | repair stacking; preserve focus chain |
| `recentered_to_visible_region` | window or pane drifted off-screen across topology change | recenter to a visible region; preserve pane order and focus chain |
| `redocked_to_safe_pane` | inspector or floating pane would dock outside a visible region | re-dock to a safe pane slot; preserve inspector identity |

Rules (frozen):

1. Every adjustment row carries a typed `adjustment_class` and an
   optional set of `affected_pane_ids[]`. Free-form adjustment
   strings are non-conforming.
2. Adjustments compose. A window may be both `moved_to_primary_display`
   and `snapped_to_safe_bounds` in one restore.
3. An adjustment row is required whenever the topology packet
   declares any `unsupported_display_topology` placeholder, unless
   the producer documents in `notes` why no adjustment was applied.
4. Topology adjustments never confer new authority. Reflowing a pane
   to a safe slot does not change `workspace_authority_checkpoint_ref`.

## 6. Live-capability inventory rule

Live capabilities — PTY handles, debug sessions, notebook kernels,
live remote bindings, browser session tokens, secret material — may be
**remembered only as context refs or evidence**, never as restored
live authority.

Rules (frozen):

1. The workspace-authority checkpoint lists every excluded class
   under `excluded_live_authority_classes[]`. The list is the floor
   for what cannot ride as restored authority.
2. The window-topology packet records each affected pane as a
   placeholder row with the matching `placeholder_reason` plus
   `evidence_retained = true` when a transcript, snapshot, or
   metadata summary survived.
3. A surface that infers "the live session continued" from absence of
   evidence is non-conforming. Continuity is reachable only when the
   runtime actually survived; the workspace-shell pane-tree schema's
   `live_session_continued` posture is the only place that marker is
   admitted, and it MUST cite a surviving binding.

## 7. Conformance checklist

A restore packet conforms when it can answer:

- Which `workspace_authority_checkpoint_record` is the authoritative
  body, and which `window_topology_snapshot_record` packets reference
  it?
- What is the producer build and source schema version of each body?
- Which `restore_class` does each body claim, and which
  `downgrade_triggers[]` narrowed the result?
- Which dirty-buffer/journal identities, trusted roots, active
  worksets, and related evidence refs travel with the authority body?
- Which live-authority classes are intentionally excluded?
- For each pane that did not restore exactly, which closed
  `placeholder_reason` applies and which safe actions are offered?
- For each window that needed display or topology repair, which
  `adjustment_class` rows were emitted?
- Does the packet's claim agree with the
  `state_restore_provenance_record` the migration-and-restore
  playbook validates?

If any answer requires new vocabulary, this contract and its schemas
are extended first.

## 8. Changing this vocabulary

- **Additive-minor** changes (new placeholder reason, new adjustment
  class, new excluded live-authority class, new downgrade trigger,
  new tab/inspector inventory field) land here and in the companion
  schemas in the same change. The change MUST cite the motivating
  fixture under
  [`/fixtures/state/restore_artifact_family/`](../../fixtures/state/restore_artifact_family/).
- **Repurposing** an existing restore-class label, placeholder
  reason, adjustment class, or excluded-class label is breaking and
  requires a governance decision row.
- The pane-tree schema version field stays bound to the
  workspace-shell pane-tree body. A bump there forces a bump on the
  topology packet's `pane_tree_schema_version`.
