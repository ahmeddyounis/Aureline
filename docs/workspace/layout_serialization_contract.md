# Workspace layout serialization contract

This document freezes the cross-surface vocabulary every startup,
graceful shutdown, crash recovery, remembered-state inspector,
portable-state review, diagnostics export, and support flow uses when
it names **which layout state is authoritative**, **which layout state
is window-local**, **which defaults are portable**, **which display
metadata is only a hint**, and **which restore phase or degraded pane
state the user is seeing**.

Window topology is a product truth surface, not an implementation
detail. If Aureline cannot say which part of a remembered layout is
portable, which part is local-only, which part is only display
best-effort metadata, and which part is shared across multiple windows
of the same workspace, then session restore, crash recovery, and
support flows will all drift into incompatible stories.

The machine-readable schema lives at:

- [`/schemas/workspace/pane_tree.schema.json`](../../schemas/workspace/pane_tree.schema.json)

The companion fixtures live under:

- [`/fixtures/workspace/layout_serialization_examples/`](../../fixtures/workspace/layout_serialization_examples/)

Adjacent contracts this document composes with:

- [`/docs/workspace/entry_restore_object_model.md`](./entry_restore_object_model.md)
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
or UI/UX spec sections cited in §9, those documents win and this
document plus the schema MUST be updated in the same change. Where a
later shell, restore, or diagnostics surface mints its own layout,
phase, or placeholder vocabulary, this document wins and the surface is
non-conforming.

## Why freeze this now

Session restore becomes dishonest the moment the product flattens four
different state classes into one blob:

- workspace authority, which is shared and authoritative;
- window topology, which is local to one window;
- profile defaults, which are portable preferences;
- machine or display hints, which are best-effort and disposable.

Without a frozen separation model, one surface calls a pane tree "saved
layout," another calls it "workspace state," a third treats monitor
coordinates as durable truth, and a fourth quietly serializes live
terminal or notebook authority as if it were safe to replay. That drift
would make multi-window restore, topology migration, placeholder
hydration, and support exports impossible to explain consistently.

## Scope

- Freeze one separation model for workspace authority, window topology,
  profile defaults, and machine or display hints.
- Freeze one versioned pane-tree schema with stable pane IDs, split and
  tab-group nodes, focus chain, visible inspectors, follow or
  presentation state, collaboration role badges, and monitor-affinity
  hints.
- Freeze one layout-restore provenance record that names the restore
  phases `chooser`, `skeleton`, `hydrate`, `rebind`, and
  `evidence-only fallback`.
- Freeze missing-surface placeholder rules for extension absence,
  missing remote targets, revoked or expired authority, display-topology
  mismatch, and non-reentrant live panes.
- Freeze no-rerun rules for terminals, debuggers, notebooks, remote
  shells, and similar live surfaces so later implementations cannot
  silently replay commands or reacquire authority.
- Seed worked example artifacts for one window-topology snapshot and one
  layout-restore provenance record.

## Out of scope

- The production session-restore engine, crash-loop detector, or
  multi-window runtime implementation.
- Concrete on-disk bytes for any future session database, checkpoint
  journal, or support bundle archive.
- Final window-manager behavior on each OS. This document freezes the
  vocabulary and guardrails the platform layer must honor.
- Full collaboration or shared-debug implementation. This contract only
  reserves the topology state those features will project into.

## 1. Boundary and portability model

Every remembered-layout artifact MUST resolve each field to exactly one
of the four layers below. Flattening them into one opaque payload is
non-conforming.

| Layer | Typical contents | Default portability | Explicit export posture | Restore posture |
|---|---|---|---|---|
| **Workspace authority** | open buffers, dirty journals, save checkpoints, trust or policy state, attached execution contexts, workset identity | local-only or workspace-shared; never user-profile portable | inspect or export only through a checkpoint or evidence flow; raw live authority is excluded | authoritative and shared across every window backed by the same workspace instance |
| **Window topology** | pane tree, tabs, focus chain, visible inspectors, zoom, fullscreen state, follow or presentation mode, collaboration role badges | local-only by default | exportable only as an explicit layout bundle or diagnostic artifact | restore `skeleton` first, then `hydrate`; preserve structure even when panes degrade |
| **Profile defaults** | density preset, panel defaults, theme, keymap, startup or open behavior | portable | included in portable profile artifacts when the user requests it | applied as defaults only; never overwrite explicit restored topology silently |
| **Machine or display hints** | monitor affinity, display fingerprint, scale bucket, safe bounds hint, last-known arrangement hash | local-only, best-effort only | diagnostics or evidence export only | never authoritative; safe remap or drop when the display topology changed |

Rules:

- Workspace authority is the only layer that can own trust, dirty state,
  save locks, execution contexts, or authority to mutate workspace
  truth.
- Window topology MAY remember that a live pane existed, but it MUST NOT
  serialize a live capability ticket, delegated approval, or remote
  authority as if that ticket were layout truth.
- Profile defaults MAY seed a new window or fill an omitted optional
  field during restore, but they MUST NOT overwrite an explicit
  snapshot's pane order, focus chain, or visible-inspector state.
- Machine or display hints MUST be labeled as best-effort metadata. A
  restored window that no longer fits current monitors must snap to safe
  bounds rather than treating stale coordinates as authoritative.

## 2. Multi-window ownership rules

One workspace authority MAY back many simultaneous windows. That is the
central ownership rule the schema and fixtures preserve.

| Concern | Lives in | Shared across windows? | Must never imply |
|---|---|---|---|
| buffers, journals, save checkpoints, trust or policy state, attached execution contexts | workspace authority | yes | that a local window may fork durable workspace truth privately |
| pane tree, focus chain, visible inspectors, zoom, fullscreen, presentation or follow state | window topology | no | that moving or closing a pane changes workspace authority |
| collaboration role badges, follow state, presentation state | window topology | no, except by explicit session event | driver rights, debugger control, runbook approval, or trust grants |
| monitor affinity and display geometry | machine or display hints | no | durable ownership over a specific screen or coordinate set |

Rules:

- Every window-topology snapshot MUST carry exactly one
  `workspace_authority_ref`. Many window snapshots MAY point to the same
  authority ref.
- Shared control is not shared authority. A `presenter`, `co-presenter`,
  `observer`, `driver`, or `approver` badge lives in window topology and
  MUST remain visible after restore, but it MUST NOT confer trust,
  mutable authority, or control over another user's workspace state by
  implication.
- Closing, floating, moving, pinning, or replacing a pane mutates the
  pane tree for that window only unless the user invokes an explicit
  shared-workspace action outside this contract.
- A window-topology artifact MUST remain inspectable, exportable,
  clearable, and revertible without deleting unrelated workspace
  authority or profile state.

## 3. Pane-tree schema seed

The schema exports two record kinds:

- `window_topology_snapshot_record` — one window's remembered topology.
- `layout_restore_provenance_record` — one restore event's phase and
  degradation summary.

### 3.1 Node taxonomy

The pane tree is versioned by `pane_tree_schema_version` and uses stable
opaque IDs. The seed node kinds are:

| Node kind | Required identity | Required contents | What stays stable across restore |
|---|---|---|---|
| `split` | `split_id` | orientation, ordered children, optional weights | child order and split identity |
| `tab_group` | `group_id` | ordered tabs, active tab ID | group identity, tab order, active tab target |
| `leaf` | `pane_id` | surface role, surface class, hydration behavior, availability state | pane identity even if the live surface is replaced by a placeholder |

Rules:

- `pane_id` is the stable anchor every remembered-state inspector,
  layout diff, support export, and restore provenance record MUST use.
- Replacing a live pane with a placeholder during restore MUST NOT mint
  a new `pane_id`. The placeholder occupies the same pane slot.
- Focus history and visible inspectors are window-level metadata, not
  pane-local implementation detail. They are serialized beside the tree,
  not hidden inside an arbitrary leaf payload.

### 3.2 Remembered window-local metadata

Each `window_topology_snapshot_record` also carries:

- `focus_chain` — ordered refs to the last useful focus targets;
- `visible_inspectors` — inspector kind, target pane ref, and dock
  position;
- `follow_presentation_state` — explicit follow mode, presentation
  mode, visible role badges, and whether shared-control chrome was
  visible;
- `monitor_affinity_hint` — display class, scale bucket, safe bounds,
  and topology hash as best-effort metadata only.

## 4. Restore phases and shared recovery vocabulary

Every layout restore MUST speak the same five phase names. Human-facing
copy may render `evidence-only fallback`; the machine enum is
`evidence_only_fallback`.

| Phase | Required job | Must never do |
|---|---|---|
| `chooser` | select the candidate checkpoint, snapshot, or imported bundle; declare expected fidelity and missing-dependency posture before mutation | silently pick a different source once restore has committed |
| `skeleton` | rebuild window shells, pane tree, tab groups, focus chain, visible inspectors, and placeholder slots without touching live dependencies | block on remote attach, extension host start, or notebook kernel work |
| `hydrate` | lazily resolve panes that can re-open safely, replacing unavailable panes with placeholders while preserving layout truth | collapse missing panes, rerun commands, or hide missing dependencies |
| `rebind` | reconnect the restored window topology to workspace authority after trust, policy, and authority reevaluation | silently widen authority or reuse expired grants |
| `evidence-only fallback` | preserve titles, tabs, cwd hints, transcripts, outputs, provenance, and safe recovery actions when a live surface cannot resume safely | present evidence as a live session or replay side effects automatically |

Rules:

- Restore order is always `chooser` -> `skeleton` -> `hydrate` ->
  `rebind`. `evidence-only fallback` MAY run for individual panes during
  `hydrate` and MUST be recorded explicitly in provenance.
- `skeleton` completes before any remote, extension, or runtime attach
  attempt begins. This is the contract's "skeleton first, hydrate
  second" rule.
- `rebind` is where authority truth is checked. A pane that cannot be
  rebound safely degrades to placeholder or evidence-only state; it does
  not silently inherit old authority.

## 5. Placeholder and degradation rules

Unavailable surfaces preserve topology truth by replacing only the
failing pane, never the surrounding structure.

| Condition | Required placeholder payload | Safe recovery actions | Forbidden behavior |
|---|---|---|---|
| **Missing extension or feature pack** | original pane role, last-known title or provenance, dependency class = `missing_extension` | `Locate`/`Install extension`, `Open without`, `Export evidence`, `Remove pane` | deleting the pane from the tree silently |
| **Missing remote target or remote connector** | original pane role, remote provenance label, dependency class = `missing_remote` | `Reconnect`, `Reauthenticate`, `Export evidence`, `Remove pane` | silently reusing stale route grants or showing an empty live pane |
| **Revoked or expired authority** | original pane role, authority posture, dependency class = `missing_remote_authority` or `revoked_permission` | `Reauthenticate`, `Open restricted`, `Export evidence` | hidden authority reacquisition |
| **Display-topology mismatch** | preserved pane tree plus topology-adjustment note | `Reflow`, `Move to primary display`, `Keep here` when safe | restoring off-screen or unreachable geometry as durable truth |
| **Non-reentrant live surface** | transcript or snapshot placeholder, dependency class = `non_reentrant_live_surface` | `Rerun explicitly`, `Rebind existing session`, `Export evidence`, `Remove pane` | replaying the prior live action automatically |

Rules:

- Placeholder cards MUST retain the original `pane_id`, surface role,
  and surrounding tab or split position.
- The product MAY change a pane's `availability_state`, but it MUST NOT
  discard the surrounding tab group, focus chain membership, or
  inspector targeting just because one pane degraded.
- Missing-dependency fallback is allowed to be partial. One window may
  restore cleanly while a sibling window is full of placeholders; both
  still share the same workspace authority ref if they belonged to the
  same workspace instance.

## 6. Live-surface no-rerun rules

The contract draws a hard line between reopening context and re-running
effects.

| Surface class | Aureline may remember | Aureline must never do automatically | Required recovery path |
|---|---|---|---|
| **Terminal / remote shell** | title, tab order, cwd hint, transcript, exit state, provenance | rerun shell commands, replay PTY input, reacquire broader shell authority silently | `Rerun explicitly`, `Rebind existing session` if still valid, `Open transcript`, `Export evidence` |
| **Debugger** | target identity, launch-config ref, last frame summary, breakpoint-set ref, last known stop reason | continue, step, restart, attach, or widen debug authority silently | `Reattach`, `Relaunch from preview`, `Open evidence`, `Remove pane` |
| **Notebook** | kernel label, kernel provenance, selected notebook tab, cell-output snapshot, trust boundary | rerun cells, reconnect to a kernel silently, or conceal that output is a replay or snapshot | `Reconnect kernel`, `Open read-only`, `Export evidence`, `Remove pane` |
| **Task, pipeline, preview runtime, or other live service pane** | labels, target refs, last state, linked logs or artifacts | re-trigger work, hide expired authority, or claim a stale result is current | `Retry explicitly`, `Reconnect`, `Open logs`, `Export evidence`, `Remove pane` |

Rules:

- "Restore" for a live surface means re-opened context, not replayed
  side effects.
- A later implementation MAY offer explicit one-click rerun or reattach
  actions, but those actions are new user intents after restore, not
  hidden consequences of restore itself.
- A live surface that cannot prove authority continuity MUST degrade to
  placeholder or evidence-only state.

## 7. Worked examples

The seed fixtures illustrate the intended boundary:

- [`window_topology_snapshot_presentation_aux.json`](../../fixtures/workspace/layout_serialization_examples/window_topology_snapshot_presentation_aux.json)
  shows one auxiliary presentation window that shares a
  `workspace_authority_ref` with a sibling main window while keeping its
  pane tree, inspectors, follow state, and monitor-affinity hint
  window-local.
- [`layout_restore_provenance_missing_dependencies.json`](../../fixtures/workspace/layout_serialization_examples/layout_restore_provenance_missing_dependencies.json)
  shows a restore that preserves layout truth across monitor change,
  records every named restore phase, inserts extension and remote
  placeholders, and keeps a terminal pane in evidence-only state rather
  than rerunning it.

## 8. Relationship to adjacent contracts

- The entry and restore object model remains the source of truth for
  entry verbs, restore levels, missing-target language, and
  checkpoint-linked recovery affordances. This contract only freezes the
  window-topology and pane-hydration side of restore.
- The profile and state-map contract remains the source of truth for
  which state classes are portable, local-only, admin-owned, or
  excluded. This contract narrows that model specifically for workspace
  authority, window topology, profile defaults, and machine or display
  hints.

## 9. Normative source references

- PRD: workspace serialization and saved layouts, split editors and
  multi-window basics, terminal PTY lifecycle, and presentation follow
  requirements.
- TAD 12.7: workspace-window, split-layout, and session-restore
  architecture.
- TDD 7.1.11: workspace-window, split-layout, and session-restore
  architecture.
- UI/UX spec 6.14 and restore-no-rerun sections: window-topology
  snapshot, missing-surface placeholder, restore-provenance, and live
  surface restore behavior.
