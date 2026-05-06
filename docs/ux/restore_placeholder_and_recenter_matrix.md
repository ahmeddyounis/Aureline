# Restore Placeholder and Recenter Matrix

This document publishes the cross-surface rules that prevent session
restore from silently collapsing layout, hiding missing dependencies, or
stranding windows off-screen when monitor topology drifts.

The matrix is a UX-facing crosswalk over already-frozen contracts. It
does not mint new restore vocabulary; it binds restore surfaces,
placeholders, and display-topology adjustments to one reviewable posture
so a reviewer can tell whether a surface restored **live**, restored as
**placeholder**, or restored as **stale/evidence context**.

Companion artifacts:

- [`/artifacts/ux/restore_placeholder_matrix.yaml`](../../artifacts/ux/restore_placeholder_matrix.yaml)
  &mdash; machine-readable placeholder-class rows and required recovery
  actions.
- [`/fixtures/ux/restore_topology_cases/`](../../fixtures/ux/restore_topology_cases/)
  &mdash; worked topology-drift cases that exercise recenter / re-dock
  and placeholder preservation without claiming exact restore.

This contract composes with (and defers to) the upstream sources:

- Restore fidelity and disclosure:
  [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](./crash_loop_and_restore_fidelity_contract.md),
  [`/schemas/ux/restore_fidelity.schema.json`](../../schemas/ux/restore_fidelity.schema.json)
- Restore prompt and missing-target states:
  [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
- Window topology restore and placeholder rules:
  [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md),
  [`/schemas/workspace/pane_tree.schema.json`](../../schemas/workspace/pane_tree.schema.json)
- Display-topology change handling and restore history:
  [`/docs/ux/window_display_contract.md`](./window_display_contract.md),
  [`/schemas/platform/window_state.schema.json`](../../schemas/platform/window_state.schema.json)
- Restore provenance and placeholder-card fields:
  [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md),
  [`/schemas/state/restore_provenance_record.schema.json`](../../schemas/state/restore_provenance_record.schema.json)
- Empty/loading/placeholder honesty and stale-cached disclosure:
  [`/docs/ux/empty_loading_placeholder_contract.md`](./empty_loading_placeholder_contract.md),
  [`/schemas/ux/placeholder_state.schema.json`](../../schemas/ux/placeholder_state.schema.json)

Where this document disagrees with the PRD, TAD, TDD, or UI/UX spec, the
upstream source wins and this matrix plus its companion artifact and
fixtures update in the same change.

## 1. Core invariants (frozen)

1. **Layout structure survives dependency loss.** Missing dependencies
   replace *only* the affected pane with a placeholder; they do not
   delete the pane or silently re-shape the split/tab tree.
2. **No restored window is allowed off-screen.** When the prior display
   arrangement is unavailable, restore clamps into safe visible bounds.
3. **Recenter is reviewable, not a silent jump.** Any safe-bounds remap,
   recenter, or re-dock produces a durable restore-history/provenance
   trace and a user-visible `layout adjusted` cue when it materially
   changes what the user sees.
4. **Restore never overclaims liveness.** A surface that restored as
   placeholder, stale-cached content, transcript-only, or evidence-only
   must not look “healthy” or “ready” until it is actually live again.

## 2. How reviewers tell “live” vs “placeholder” vs “stale/evidence”

Review is anchored on three independent signals:

- **Restore-fidelity class** (`exact_restore`, `compatible_restore`,
  `layout_only`, `recovered_drafts`, `evidence_only`) emitted via the
  restore prompt and restore surfaces.
- **Pane posture**:
  - **Live** panes restore with `availability_state = ready`.
  - **Placeholder** panes restore with a missing-dependency placeholder
    card (per the provenance contracts) in the same pane slot.
  - **Stale/evidence** panes restore with a declared `stale_cached_content`
    / `transcript_restored_not_rerun` / `evidence_only` posture, never a
    live-looking surface.
- **Reviewable trace**:
  - display/topology adjustments in restore-history and layout-restore
    provenance; and
  - per-pane placeholder rows (with class + recovery actions).

A surface is *not* considered “live” merely because it appears; liveness
requires both the restore class and per-pane posture to agree.

## 3. Dependency-loss placeholder matrix

The placeholder classes below are the reviewable, typed outcomes used by
restore cards, provenance inspectors, support export, and docs/help.
Rows are authored in the companion YAML matrix; this section summarizes
the required UX posture.

### 3.1 Placeholder classes (required behavior)

| Placeholder class | Typical triggers (non-exhaustive) | Required placeholder behavior | Fidelity ceiling |
| --- | --- | --- | --- |
| `absent_extension` | missing extension host, missing feature pack, quarantined/disabled extension view | preserve pane slot/role; show missing-owner cue; offer install/locate/open-without actions; retain evidence when available | `layout_only` |
| `absent_remote_target` | remote host unreachable, connector missing, remote workspace endpoint unavailable | preserve pane slot/role; keep local context and last-known provenance; never reuse stale route grants; offer reconnect/reauth | `layout_only` |
| `revoked_permission` | expired/invalid scoped grant, revoked credential, expired managed-session ticket | preserve pane slot/role; show authority boundary; require explicit reauth or restricted-open path | `layout_only` |
| `stale_service_dependency` | managed service deprecated/offline, control-plane drift, endpoint invalidated | preserve pane slot/role; show stale/unsupported dependency cue; offer repair instructions and export | `layout_only` |
| `missing_workspace_authority` | referenced workspace authority checkpoint absent on this machine | preserve layout as evidence; provide compare/export/manual repair path | `evidence_only` or `layout_only` (best-effort) |
| `missing_schema_equivalence_map` | schema translation required but mapping missing/refused | preserve layout + prior artifact; block meaning-change behind review; require compare/repair | `compatible_restore` (never `exact_restore`) |

Rules:

- Placeholder cards **retain the original `pane_id` and pane role** and
  occupy the same tab/split slot. Replacing the pane with a new ID is
  non-conforming.
- Placeholder cards **name the missing dependency class** (from the
  closed set above) and **must not substitute free-form prose** for the
  class label.
- Placeholders **never claim live capability**. Any `Retry`/`Reconnect`
  path must still disclose that the surface is not live until it
  succeeds.

### 3.2 Required trace for dependency-loss placeholders

Every dependency-loss placeholder must be explainable from:

- `restore_prompt_record.missing_target_states[]` (pre-commit disclosure),
- `state_restore_provenance_and_placeholder_record.missing_dependency_classes[]`
  and `.missing_dependency_placeholder_cards[]` (post-commit inspection),
- `layout_restore_provenance_record.placeholder_results[]` for the
  per-window, per-pane restore event, and
- support/export pathways that quote the same class and action set.

If a restore inserted a placeholder but cannot surface it via provenance
and support export, it must not claim a compatible or exact restore.

## 4. Monitor topology drift, off-screen prevention, and recenter matrix

Display drift is treated as a typed restore event, not a generic resize.
The window/display contract is the source of truth; this section
summarises the non-negotiable UX posture.

### 4.1 Topology-change inputs that can force recenter/re-dock

The platform adapter reports a `topology_change_class` before focus is
restored. The closed vocabulary includes: display add/remove/reorder,
safe-bounds change, scale change, wake reconnect, reopen, dock/undock,
and related rewrite cases.

### 4.2 Required outcomes (safe restore)

| Situation | Required safe outcome | Forbidden outcome |
| --- | --- | --- |
| Display removed / safe bounds changed | remap unreachable windows/transients into nearest safe visible bounds (fallback to primary display when needed) | window or dialog stranded fully off-screen |
| Display returns later | do not silently jump windows back; offer an explicit restore-layout path | surprise teleportation to prior monitor |
| Scale (mixed-DPI) drift | update scale/hit testing first; then remap bounds and reflow; keep keyboard reachability | focus or recovery actions becoming unreachable |
| Fullscreen/snap rewritten | fall back to safe bounds or cleared mode and record the adjustment | restoring stale snapped geometry as durable truth |
| Wake/resume reconnect | treat as topology event; revalidate authority separately; keep focus visible | silent authority reacquisition during wake |
| Owned dialog loses safe bounds | recenter dialog to its owning window; return focus to invoker/owner on dismissal | ownerless global dialog surviving on detached display |

### 4.3 Required trace and user-visible note

When any remap/recenter/re-dock materially changes placement, the system
must leave both:

- a **reviewable trace** (`window_restore_history_record` and
  per-window `layout_restore_provenance_record.display_adjustments[]`),
  and
- a **user-visible layout-adjusted cue** that can be revisited (e.g.
  from restore details / diagnostics / persistence inspector), not only
  as a transient toast.

If the product cannot explain why it moved, resized, unfullscreened,
recentered, or re-docked a window or prompt, it must not claim
`exact_restore`.

## 5. Fidelity downgrade cues (exact vs compatible vs layout-only)

Fidelity is a truth label, not a marketing badge.

- `exact_restore` is valid only when:
  - no placeholders were inserted,
  - no topology adjustments materially changed placement/mode, and
  - no live authority rebind is pending for surfaces that would imply
    privileged continuity.
- `compatible_restore` is the normal landing when:
  - topology changed but the intent and structure were preserved with a
    recorded adjustment; or
  - schema translation occurred without stopping at manual review.
- `layout_only` is required when:
  - any pane restored as a missing-dependency placeholder; or
  - live runtime surfaces were preserved only as transcripts/snapshots.

The restore summary must state (in one place) what restored, what
degraded (placeholder vs stale/evidence), what moved (recenter/re-dock),
and which action restores capability.

