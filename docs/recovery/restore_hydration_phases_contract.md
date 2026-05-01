# Restore-hydration phases, shell-ready cue, and background-rebind honesty contract

This document freezes the cross-surface contract every startup,
restore, benchmark trace, diagnostics, support, release-evidence,
and docs/help surface uses when it answers two questions about a
warm or recovering launch:

> Which named phase is restore in right now, and which interactive
> readiness cue (if any) is the user actually allowed to trust?

Without this contract, restore-startup performance copy collapses
into one ambiguous `Ready` label that:

- promises `Ready` while the workspace authority has been re-bound
  but remote sessions, notebooks, debuggers, or provider-linked
  surfaces are still rebinding;
- lets a quick-open surface and a semantic-search surface both
  render `Ready` even though one is name-only and the other has
  not finished its index warmup;
- silently upgrades a `placeholder hydration` pane into a `live`
  pane without naming the transition, so the user cannot tell
  what changed under them;
- silently downgrades a previously-live pane into placeholder /
  evidence-only state without naming it as a degrade, so support
  cannot reason about why a benchmark suddenly differs;
- emits a benchmark trace `first_useful_chrome` event and a
  support-bundle `restore_phase` token that disagree on which
  phase finished first.

The hydration-phase event record is the **shared inspectable
body** every shell, restore explainer, benchmark trace,
diagnostics panel, support bundle, release-evidence packet, and
docs/help surface projects into the same closed phase vocabulary,
the same closed ready-cue vocabulary, and the same closed phase /
cue transition vocabulary. It is **not** a startup orchestrator,
**not** a UI rendering plan, and **not** a runtime scheduler. It
is the contract those surfaces MUST conform to so warm startup
and progressive restore stay **phase-coded** — chooser, shell
skeleton, workspace authority rebind, placeholder hydration,
live dependency rebind, and evidence-only fallback — instead of
collapsing into one generic `Ready` narrative.

The machine-readable schema lives at:

- [`/schemas/recovery/hydration_phase_event.schema.json`](../../schemas/recovery/hydration_phase_event.schema.json)
  — closed phase vocabulary, closed ready-cue vocabulary, closed
  cue-transition vocabulary, partiality fields, packet/export
  linkage, and honesty invariants.

Worked fixtures live under:

- [`/fixtures/recovery/hydration_phase_cases/`](../../fixtures/recovery/hydration_phase_cases/)

This contract composes with — and never re-defines — the
restore, layout, and recovery rules frozen elsewhere:

- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  — pane-tree schema and the original
  `chooser` → `skeleton` → `hydrate` → `rebind` →
  `evidence_only_fallback` restore-phase order.
- [`/docs/state/restore_artifact_family_contract.md`](../state/restore_artifact_family_contract.md)
  — workspace-authority checkpoint and window-topology snapshot
  shapes the rebind phases reference by opaque ref.
- [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md)
  — restore-provenance / placeholder card record the placeholder-
  hydration phase reads.
- [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../ux/crash_loop_and_restore_fidelity_contract.md)
  — restore-fidelity classes that pair with each phase's claimed
  cues.
- [`/docs/ux/entry_restore_truth_audit.md`](../ux/entry_restore_truth_audit.md)
  — `startup_state` tokens the phase event references when the
  shell is rendering a placeholder startup state.
- [`/docs/recovery/restore_chooser_contract.md`](./restore_chooser_contract.md)
  — restore-chooser state record the `chooser` phase event
  references by opaque ref.
- [`/docs/runtime/background_queue_contract.md`](../runtime/background_queue_contract.md)
  — shell-ready budget protection and the `ready` / `warming` /
  `partial` / `degraded` health-state vocabulary the phase event
  reuses without renaming.
- [`/docs/qa/multi_window_verification.md`](../qa/multi_window_verification.md)
  — the multi-window verification matrix that asserts placeholder
  hydration and authority rebind separately.

This contract is normative for the hydration-phase disclosure
posture. Where it disagrees with the PRD, TAD, TDD, UI/UX spec,
or one of the upstream contracts above, those documents win and
this contract plus its schema / fixtures MUST be updated in the
same change. Where a downstream startup, benchmark, diagnostics,
support, release-evidence, or docs/help surface mints a parallel
phase or readiness vocabulary, this contract wins and the
surface is non-conforming.

This contract mints **no** new restore-fidelity, restore-level,
recovery-level, missing-target, missing-dependency, exit-reason,
or recovery-class values. Every closed set re-exported here is
quoted by reference from the upstream contract that owns it.

## Why freeze this now

Warm startup and restore are the moments when the `Ready` label
hurts the user most. A single ambiguous readiness pulse produces
these non-conforming patterns:

- **Ready overclaim.** The shell projects `Ready` after the
  skeleton paints; the user types into quick open and the file
  index has not finished, so search returns nothing. The user
  concludes the file is gone.
- **Privileged-surface false live.** A debugger pane renders as
  if attached because the layout came back; in fact authority
  was not yet re-bound. The user clicks `Continue` and nothing
  happens.
- **Silent placeholder upgrade.** A pane that opened as a
  missing-extension placeholder turns live three seconds later
  with no `upgraded_from_placeholder` cue, so the user cannot
  trust what is now editable.
- **Silent live downgrade.** A semantic-search surface that
  emitted `semantic_ready` quietly returns to a name-only
  fallback when its index becomes stale; without an explicit
  `downgraded_from_live` cue, support cannot reason about why
  search results regressed.
- **Cross-surface drift.** A benchmark trace says
  `first_useful_chrome` finished before `placeholder_hydration`;
  the diagnostics panel says the same launch finished
  `placeholder_hydration` before `shell_skeleton`; the support
  bundle says only `restore_phase: ready`. Reviewers cannot tell
  which is true.

The hydration-phase event record forecloses these patterns by
projecting one closed six-class phase vocabulary, one closed
twelve-class ready-cue vocabulary, one closed seven-class cue-
transition vocabulary, typed partiality fields for stalled
rebinds and missing dependencies, typed packet/export linkage,
and one set of const honesty invariants every surface emits and
reads.

## Scope

Frozen here:

- one `hydration_phase_class` closed six-class vocabulary —
  `chooser`, `shell_skeleton`, `workspace_authority_rebind`,
  `placeholder_hydration`, `live_dependency_rebind`,
  `evidence_only_fallback` — plus the deterministic ordering
  rules and the bypass-allowed posture for each;
- one `phase_state_class` closed nine-class vocabulary
  describing what is actually happening in this phase
  (`entered`, `progressing`, `stalled`,
  `awaiting_workspace_authority`, `awaiting_live_dependency`,
  `completed`, `degraded_to_evidence_only`,
  `upgraded_from_placeholder_hydration`,
  `bypassed_not_applicable`);
- one `ready_cue_class` closed twelve-class vocabulary so the
  shell can light `shell_ready`, `quick_open_ready`,
  `command_entry_ready`, `first_editor_ready`, `search_ready`,
  `semantic_ready`, `remote_rebind_complete`,
  `provider_rebind_complete`, `notebook_rebind_complete`,
  `debugger_rebind_complete`, `task_runtime_rebind_complete`,
  and `restore_complete` distinctly — never collapsing them
  into one generic `Ready`;
- one `cue_transition_class` closed seven-class vocabulary
  (`not_yet_emitted`, `emitted_live`, `emitted_degraded`,
  `upgraded_from_degraded`, `downgraded_from_live`,
  `superseded_by_evidence_only`, `withdrawn_into_placeholder`)
  so a later surface, rebind, or semantic refinement cannot
  silently redefine an earlier `shell_ready` cue without an
  explicit transition;
- one `partiality_block` typed body that names what is still
  not live (`stalled_rebind`, `missing_dependency`,
  `expired_authority`, `pending_index_warmup`,
  `pending_remote_attach`, `pending_provider_overlay`) so
  background hydration progress is visible without overstating
  live capability;
- typed **packet/export linkage** rules so startup, benchmark
  traces, diagnostics, support bundles, release evidence, and
  docs/help all cite the same `phase_session_id`, the same
  upstream `restore_chooser_state_ref`,
  `restore_provenance_record_ref`,
  `window_topology_snapshot_ref`,
  `workspace_authority_checkpoint_ref`, and the same
  recovery-ladder packet ref;
- **honesty invariants** — closed phase vocabulary, distinct
  ready cues, named partiality, no silent cue redefinition,
  privileged surfaces never live before rebind, typed packet/
  export refs.

Out of scope:

- the actual startup orchestrator, restore engine, hydration
  scheduler, or watchdog implementation;
- final user-facing copy / microcopy for ready cues — those are
  pinned by the UX style guide, the shell-zone density contract,
  and the entry-restore truth audit;
- benchmark wire formats and protected-path budget targets —
  the spike-timing trace and benchmark metric registry remain
  the source of truth for those numbers;
- the recovery-level chosen for the launch — the chooser
  contract owns that record. This contract only references the
  chosen `recovery_level_class` by ref.

## 1. Record model

The hydration-phase contract emits one record shape:

| Record | Purpose |
|---|---|
| `hydration_phase_event_record` | One event per phase entry, advance, stall, partial completion, completion, upgrade, or degrade. Names the closed phase, the typed phase state, the cue transitions emitted with this event, the partiality block, the packet/export linkage, and the honesty invariants. |

A given phase session emits one or more events. Multiple
recovering scopes in the same launch (e.g. one workspace
restoring exact, another restoring evidence-only after
corruption) emit one event stream each, scoped to their
`phase_session_id`.

## 2. Phase vocabulary

Six closed phases. Every event resolves to exactly one. The set,
rendered names, and ordering rules are re-exported into the
schema's `hydration_phase_class` enum.

| Phase | Rendered name | Required job | MUST never claim |
|---|---|---|---|
| `chooser` | `Chooser` | Project the upstream `restore_chooser_state_record` (recovery-level matrix, primary actions, retained evidence, risk note). Mutates nothing. | live readiness for any pane; that a recovery level was chosen silently. |
| `shell_skeleton` | `Shell skeleton` | Rebuild window shells, pane tree, tab groups, focus chain, visible inspectors, and placeholder slots. Light the `shell_ready`, `quick_open_ready`, `command_entry_ready`, and `first_editor_ready` cues as soon as each becomes interactive against name-only / local-text data. | `search_ready`, `semantic_ready`, or any rebind-complete cue; live readiness for remote, debugger, notebook, or provider panes. |
| `workspace_authority_rebind` | `Workspace authority rebind` | Re-bind workspace authority (open buffers, dirty journals, save checkpoints, trust / policy state, attached execution contexts) after trust and policy reevaluation. Light `restore_complete` only when the chosen recovery level's authority commitments are met. | live runtime reattach; silent reauth; that authority was widened beyond what the chosen recovery level allowed. |
| `placeholder_hydration` | `Placeholder hydration` | Lazily resolve panes that can re-open safely, replacing unavailable panes with placeholders while preserving layout truth. Emit `upgraded_from_placeholder_hydration` only when a specific pane transitions from placeholder to live. | collapsing a missing pane out of the tree; silently turning a placeholder into a live pane; rerunning commands. |
| `live_dependency_rebind` | `Live dependency rebind` | Reconnect heavy / privileged surfaces — remote targets, providers, notebook kernels, debuggers, task runtimes — after explicit reauthorization. Light `remote_rebind_complete`, `provider_rebind_complete`, `notebook_rebind_complete`, `debugger_rebind_complete`, and `task_runtime_rebind_complete` per surface as each completes. | reusing an expired grant; silent reattach; emitting a rebind-complete cue while authority is still being re-checked. |
| `evidence_only_fallback` | `Evidence-only fallback` | Preserve titles, tabs, cwd hints, transcripts, outputs, provenance, and safe recovery actions for surfaces that cannot resume safely. Emit `superseded_by_evidence_only` for any cue that was previously live and is now evidence-only. | presenting evidence as a live session; replaying side effects. |

Rules (frozen):

1. **Exactly one phase per event.** A consumer that lists two
   "current" phases (`shell_skeleton` AND
   `workspace_authority_rebind`) in one event is non-conforming.
   Concurrent phases across scopes emit distinct events with
   distinct `phase_session_id` values.
2. **Deterministic ordering.** Within a single
   `phase_session_id`, the canonical phase order is
   `chooser` → `shell_skeleton` → `workspace_authority_rebind`
   → `placeholder_hydration` → `live_dependency_rebind`.
   `evidence_only_fallback` MAY ride beside `placeholder_hydration`
   or `live_dependency_rebind` for a specific scope and MUST be
   recorded as its own event. A consumer that reorders these
   phases on the same scope is non-conforming.
3. **Bypass posture.** `chooser` MAY be bypassed on a routine
   reopen with no remembered decision and no failure to recover;
   the event stream MUST then declare `phase_state:
   bypassed_not_applicable` for the chooser phase or omit it
   entirely (consumers MUST treat absence as bypass).
   `evidence_only_fallback` MAY be bypassed when no surface fell
   back. No other phase may be bypassed.
4. **Phase / cue binding.** Every event lists the cues whose
   transition it emits. The schema enforces which cue classes
   each phase may emit; a `shell_skeleton` event that emits
   `semantic_ready` is non-conforming, and a
   `live_dependency_rebind` event that emits `shell_ready` is
   non-conforming.
5. **Privileged surfaces never live before rebind.** Rebind-
   complete cues — `remote_rebind_complete`,
   `provider_rebind_complete`, `notebook_rebind_complete`,
   `debugger_rebind_complete`, `task_runtime_rebind_complete` —
   may only be emitted from a `live_dependency_rebind` event
   whose `phase_state` is `completed`. A surface that emits any
   of these cues during `shell_skeleton`,
   `placeholder_hydration`, or `workspace_authority_rebind` is
   non-conforming.
6. **Restore-complete is workspace-authority-bound.** The
   `restore_complete` cue MAY only be emitted from a
   `workspace_authority_rebind` event whose `phase_state` is
   `completed`. It does not assert anything about live
   dependencies.

### 2.1 Phase-state classes

Closed nine-class vocabulary. Every event cites exactly one.

- `entered` — the phase just started; no cues emitted yet.
- `progressing` — work continues; partial cues may be emitted.
- `stalled` — the phase is awaiting something nameable; the
  partiality block names which class.
- `awaiting_workspace_authority` — only valid on
  `live_dependency_rebind`; the live dependency cannot rebind
  until `workspace_authority_rebind` completes.
- `awaiting_live_dependency` — only valid on
  `placeholder_hydration`; a pane is queued behind a specific
  live dependency that has not finished rebinding.
- `completed` — the phase finished. The schema requires the
  paired cue set (e.g. `shell_skeleton.completed` requires the
  shell-ready cue family to be `emitted_live` or
  `emitted_degraded`).
- `degraded_to_evidence_only` — only valid on
  `placeholder_hydration` or `live_dependency_rebind`; one or
  more surfaces fell back to evidence-only state.
- `upgraded_from_placeholder_hydration` — only valid on
  `placeholder_hydration` or `live_dependency_rebind`; a pane
  transitioned from placeholder to live and the event names the
  cue with `upgraded_from_degraded` transition.
- `bypassed_not_applicable` — only valid on `chooser` and
  `evidence_only_fallback`; the phase did not run for this
  launch. No cues are emitted on a bypass event.

## 3. Ready-cue vocabulary

Twelve closed ready cues. Each cue names a distinct interactive
capability surface; collapsing two cues into one is the single
biggest cause of "Ready" overclaim.

| Cue | Asserted capability | Emitted from phase |
|---|---|---|
| `shell_ready` | The shell skeleton paints and accepts input focus. Quick open, command entry, and the first editor are routable. | `shell_skeleton` |
| `quick_open_ready` | Quick open accepts input and serves at least the recently-used and name-only candidate set. Index-backed candidates may still be `pending_index_warmup`. | `shell_skeleton` |
| `command_entry_ready` | The command palette / menu surface accepts input and serves the static command set. Provider-contributed commands may still be pending. | `shell_skeleton` |
| `first_editor_ready` | The first editor pane is interactive against the local text source. Save MAY still be deferred until workspace authority rebinds; the cue does not promise save. | `shell_skeleton` |
| `search_ready` | File-name and lexical search returns results from the warmed local index. Semantic / cross-repo backed candidates may still be pending. | `shell_skeleton` (when local index already warm) or `placeholder_hydration` (when a warmup pass had to run). |
| `semantic_ready` | Semantic / index-backed candidates are available. Distinct from `search_ready`. | `placeholder_hydration` |
| `remote_rebind_complete` | A specific remote target finished rebinding under a fresh authority grant. | `live_dependency_rebind` |
| `provider_rebind_complete` | A specific provider-overlay target finished rebinding. | `live_dependency_rebind` |
| `notebook_rebind_complete` | A specific notebook kernel finished rebinding. Cell rerun is still an explicit user action. | `live_dependency_rebind` |
| `debugger_rebind_complete` | A specific debugger session reattached under an explicit user action. Step / continue is still user-initiated. | `live_dependency_rebind` |
| `task_runtime_rebind_complete` | A specific task / pipeline / preview runtime reattached. Retrigger is still an explicit user action. | `live_dependency_rebind` |
| `restore_complete` | The chosen recovery level's authority commitments are met. Distinct from per-dependency rebind cues; does not assert any live dependency is rebound. | `workspace_authority_rebind` |

Rules (frozen):

1. **Cues are distinct.** A surface that renders one chip
   reading `Ready` and back-fills it with all twelve cues is
   non-conforming.
2. **Per-target rebind cues.** `remote_rebind_complete`,
   `provider_rebind_complete`, `notebook_rebind_complete`,
   `debugger_rebind_complete`, and `task_runtime_rebind_complete`
   are per-target. Each rebind event names a `target_ref`
   identifying the specific pane / surface / authority that
   completed.
3. **No semantic_ready without search_ready.** Emitting
   `semantic_ready` in an event whose `search_ready` cue has
   never been emitted (in this or an earlier event) is
   non-conforming.
4. **Shell cues are workspace-authority-independent.** The four
   `shell_skeleton` cues — `shell_ready`, `quick_open_ready`,
   `command_entry_ready`, `first_editor_ready` — MUST become
   emittable before workspace authority rebinds. A consumer
   that gates them on workspace authority rebind is
   non-conforming.

## 4. Cue-transition vocabulary

Closed seven-class set. Each cue cited in an event names exactly
one transition relative to the prior event:

- `not_yet_emitted` — the cue has not been emitted in this
  phase session.
- `emitted_live` — the cue is now emitted at full live
  fidelity.
- `emitted_degraded` — the cue is emitted, but the partiality
  block names what is narrowed (e.g. `quick_open_ready` is
  `emitted_degraded` while `pending_index_warmup` is true).
- `upgraded_from_degraded` — the cue was previously
  `emitted_degraded` and is now `emitted_live`. The event MUST
  reference the prior event id.
- `downgraded_from_live` — the cue was previously `emitted_live`
  and is now `emitted_degraded` (e.g. semantic search regressed
  to name-only). The event MUST cite the partiality reason.
- `superseded_by_evidence_only` — the cue was previously
  emitted at any fidelity, the bound surface fell back to
  evidence-only state, and the cue is withdrawn. The event MUST
  ride a `phase_state: degraded_to_evidence_only` event on
  `placeholder_hydration` or `live_dependency_rebind`, or an
  `evidence_only_fallback` event.
- `withdrawn_into_placeholder` — the cue was previously emitted
  for a live target, the target became a placeholder (e.g.
  extension host quit), and the cue is withdrawn while the
  surrounding pane structure is preserved.

Rules (frozen):

1. **No silent redefinition.** A later phase, rebind, or
   semantic refinement MUST NOT implicitly redefine an earlier
   `shell_ready` cue. Any change to a previously-emitted cue
   rides one of `upgraded_from_degraded`,
   `downgraded_from_live`, `superseded_by_evidence_only`, or
   `withdrawn_into_placeholder`.
2. **Upgrade cites prior.** Every `upgraded_from_degraded` /
   `downgraded_from_live` / `superseded_by_evidence_only` /
   `withdrawn_into_placeholder` cue MUST cite the
   `prior_event_ref` from the same `phase_session_id` whose
   cue last asserted the prior fidelity.
3. **Symmetric partiality.** `emitted_degraded` and
   `downgraded_from_live` cues MUST name at least one
   `partiality_class` in the event's partiality block.

## 5. Partiality block

Every event carries a typed `partiality_block`. Free-text "still
loading" prose is non-conforming.

### 5.1 Fields

- `partiality_classes[]` — closed eight-class set (§5.2);
  empty when the phase is `entered` with no cues emitted yet
  or when every emitted cue is `emitted_live`.
- `stalled_rebind_target_refs[]` — opaque ids of stalled
  rebind targets (remote / provider / notebook / debugger /
  task runtime). Required to be non-empty when
  `partiality_classes[]` contains `stalled_rebind`.
- `missing_dependency_class_refs[]` — opaque ids of
  missing-dependency placeholder cards from the upstream
  `restore_provenance_record`. Required when
  `partiality_classes[]` contains `missing_dependency`.
- `expired_authority_target_refs[]` — opaque ids of
  authority-expired targets. Required when
  `partiality_classes[]` contains `expired_authority`.
- `pending_index_warmup_class` — re-exported from the
  background-queue contract's index-warmup workload class.
  Optional.
- `summary` — short, redaction-aware text restating the typed
  classes for the user.
- `progress_visible` — boolean; const `true` whenever
  `partiality_classes[]` is non-empty. Background hydration
  progress, stalled rebinds, missing dependencies, and expired
  authority MUST be visible to the user, not silently buried.

### 5.2 `partiality_class` vocabulary

Closed eight-class set:

- `pending_index_warmup` — local index has not finished
  warming; lexical / semantic search candidates are partial.
- `pending_remote_attach` — a remote target rebind is queued
  or in flight.
- `pending_provider_overlay` — a provider overlay rebind is
  queued or in flight.
- `pending_notebook_kernel` — a notebook kernel rebind is
  queued.
- `pending_debugger_attach` — a debugger reattach is queued
  behind an explicit user action.
- `pending_task_runtime` — a task / pipeline / preview
  runtime reattach is queued.
- `stalled_rebind` — a rebind has exceeded its expected wait
  budget; the surface MUST name the target ref so support can
  cite it.
- `missing_dependency` — a placeholder card stands in for a
  missing extension, missing remote target, missing toolchain,
  missing managed session, or missing devcontainer; the cue
  cites the upstream missing-dependency card.
- `expired_authority` — a rebind cannot complete until
  reauthorization. The cue cites the expired target.

## 6. Packet / export linkage

Every event carries one typed `packet_export_linkage` block.
Free-text linkage prose is non-conforming.

### 6.1 Fields

- `phase_session_id` — opaque id stable across every event in
  the same recovering launch / scope. Reused verbatim by
  benchmark traces, diagnostics panels, support bundles,
  release-evidence packets, and docs/help examples.
- `restore_chooser_state_ref` — opaque id of the upstream
  `restore_chooser_state_record` for this scope (when a
  chooser was shown). Required when `phase_class` is
  `chooser`. Optional otherwise.
- `restore_provenance_record_ref` — opaque id of the upstream
  `state_restore_provenance_and_placeholder_record` (when a
  partial restore record exists).
- `window_topology_snapshot_ref` — opaque id of the
  `window_topology_snapshot_record` the skeleton paints from.
  Required when `phase_class` is `shell_skeleton` or later.
- `workspace_authority_checkpoint_ref` — opaque id of the
  workspace-authority checkpoint that drives the rebind.
  Required when `phase_class` is `workspace_authority_rebind`
  or later.
- `recovery_ladder_packet_ref` — opaque id of the recovery-
  ladder packet record. Always required so safe-mode and the
  rung sequence remain reachable.
- `support_bundle_candidate_ref` — opaque id of the candidate
  support-bundle (when present).
- `benchmark_trace_session_ref` — opaque id of the spike-timing
  trace session for this launch (when present). Optional.
- `release_evidence_packet_ref` — opaque id of the release-
  evidence packet citing this phase session (when present).
  Optional.
- `docs_help_label` — short, redaction-aware text used by
  docs/help and support-export previews.

### 6.2 Rules

1. **Same `phase_session_id` across surfaces.** Startup, the
   benchmark trace, the diagnostics panel, the support bundle,
   the release-evidence packet, and docs/help that cite the
   same recovering launch MUST quote the same
   `phase_session_id`. A diagnostics panel that mints a
   parallel `phase_session_id` for the same launch is
   non-conforming.
2. **Recovery-ladder always reachable.** An event without a
   `recovery_ladder_packet_ref` is non-conforming.
3. **Privileged-surface refs are stable.** Target refs cited in
   the partiality block (stalled rebinds, expired authority)
   MUST be stable across the same `phase_session_id`; a
   surface that re-mints a target ref for the same pane within
   one launch is non-conforming.

## 7. Honesty invariants

Every event MUST carry the `honesty_invariants` block with six
const-`true` fields:

- `phase_vocabulary_is_closed: true` — the event resolves to
  exactly one of the six closed phase classes. No private
  phase.
- `ready_cues_are_distinct: true` — the four `shell_skeleton`
  cues, `search_ready`, `semantic_ready`, the five rebind-
  complete cues, and `restore_complete` appear distinctly. No
  generic `Ready` collapse.
- `partiality_is_named: true` — when any cue is
  `emitted_degraded` or `downgraded_from_live`, the partiality
  block names at least one typed `partiality_class`.
- `no_silent_cue_redefinition: true` — every transition off a
  previously-emitted cue rides one of `upgraded_from_degraded`,
  `downgraded_from_live`, `superseded_by_evidence_only`, or
  `withdrawn_into_placeholder`.
- `privileged_surfaces_never_live_before_rebind: true` —
  rebind-complete cues are emitted only from a
  `live_dependency_rebind` event whose `phase_state` is
  `completed`. Privileged surfaces never appear live during
  `shell_skeleton`, `placeholder_hydration`, or
  `workspace_authority_rebind`.
- `linkage_is_typed: true` — packet/export linkage refs are
  typed; free-text linkage prose is non-conforming.

These are const guarantees in the schema. Any surface that
emits an event without them is non-conforming.

## 8. Surface rules

Apply to every surface that renders, logs, exports, or reasons
about hydration-phase event records.

1. **No private phase.** Every consumer resolves to one of the
   six closed phases; surfaces do not render a parallel
   "almost ready" or "warming complete" phase.
2. **No generic `Ready` chip.** Surfaces render the cues
   distinctly with their level-aware labels.
3. **Privileged surfaces gated.** A surface that lights a
   notebook, debugger, remote, provider, or task-runtime
   `Ready` indicator before its rebind-complete cue is emitted
   is non-conforming.
4. **Background hydration is visible.** Stalled rebinds,
   missing dependencies, and expired authority are surfaced
   through the partiality block; surfaces that render an
   indeterminate spinner without naming the partiality class
   are non-conforming.
5. **No silent upgrade or downgrade.** A surface that turns a
   placeholder into a live pane, or a live pane into a
   placeholder / evidence-only pane, MUST emit the matching
   transition cue.
6. **Cross-surface consistency.** Shell, benchmark trace,
   diagnostics, support bundle, release-evidence packet, and
   docs/help reflect the same `phase_session_id`, the same
   chosen recovery-level ref, and the same emitted cue
   transitions.
7. **Support-bundle linkage stays opaque.** Raw paths, raw
   credentials, raw URLs, raw provider payloads, and raw
   terminal scrollback never appear in a hydration-phase
   event.
8. **Recovery-ladder always linked.** Every event cites a
   `recovery_ladder_packet_ref`.

## 9. Composition with adjacent contracts

- **Layout serialization** owns the original
  `chooser` → `skeleton` → `hydrate` → `rebind` →
  `evidence_only_fallback` restore-phase order. This contract
  refines `hydrate` into `placeholder_hydration` and `rebind`
  into `workspace_authority_rebind` plus
  `live_dependency_rebind`; the additive split is governed by
  this contract's rules and never overrides the layout-
  serialization order.
- **Restore-artifact family** owns the workspace-authority
  checkpoint and the window-topology snapshot. This contract
  cites them by ref.
- **Restore-provenance / placeholder** owns the partial-restore
  record. This contract cites it by ref; missing-dependency
  partiality classes match the upstream card classes.
- **Restore-chooser** owns the chosen recovery-level record.
  This contract cites it by ref through the `chooser` phase.
- **Crash-loop / restore-fidelity** owns the
  `restore_fidelity_class` set. This contract does not
  redefine it; the chooser ref carries the fidelity class for
  the launch.
- **Background queue** owns the shell-ready budget protection
  flag and the index-warmup workload class. This contract
  reuses `pending_index_warmup` from that vocabulary.
- **Benchmarks (spike timing)** own the wire format and
  protected-path budget targets. This contract cites the
  benchmark trace session by opaque ref.

## 10. Acceptance

- **Phases stay distinct.** The six `hydration_phase_class`
  values are rendered verbatim across startup, benchmarks,
  diagnostics, support, release-evidence, and docs/help. No
  surface flattens chooser, shell skeleton, workspace authority
  rebind, placeholder hydration, live dependency rebind, and
  evidence-only fallback into one generic restore state.
- **Shell-ready becomes interactive before remote.** Every
  conformant event stream lights `shell_ready`,
  `quick_open_ready`, `command_entry_ready`, and
  `first_editor_ready` before any rebind-complete cue.
  Privileged surfaces never appear live before
  `live_dependency_rebind` completes their target.
- **Cues differentiated.** The four `shell_skeleton` cues are
  distinct from `search_ready`, `semantic_ready`, the five
  rebind-complete cues, and `restore_complete`. No surface
  collapses them.
- **No silent redefinition.** A later service, rebind, or
  semantic refinement MUST NOT implicitly redefine an earlier
  `shell_ready` cue without an explicit
  `upgraded_from_degraded`, `downgraded_from_live`,
  `superseded_by_evidence_only`, or
  `withdrawn_into_placeholder` transition.
- **Linkage stays typed.** Packet / export refs are opaque
  ids; free-text linkage prose is non-conforming.
- **Fixtures.** The fixtures under
  [`/fixtures/recovery/hydration_phase_cases/`](../../fixtures/recovery/hydration_phase_cases/)
  cover at least: chooser shown, shell skeleton ready with
  index warmup pending, workspace authority rebound with
  restore complete, placeholder hydration with a missing
  extension, live dependency rebind for a remote target,
  evidence-only fallback after corrupt restorable state,
  semantic-ready upgrades after search-ready, and a downgraded-
  from-live regression.

## 11. Changing this contract

- **Additive-minor** changes (new `phase_state_class`, new
  `ready_cue_class`, new `cue_transition_class`, new
  `partiality_class`, new surface-family / linkage ref) land
  in this document, the schema, and at least one fixture in
  the same change. The change must cite the motivating
  fixture or packet.
- **Repurposing** an existing phase, ready cue, transition,
  partiality class, or honesty invariant is **breaking**. It
  opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section here.
- The schema is the boundary. Any surface that adds a private
  field, collapses two phases or two cues, or emits an event
  without the `honesty_invariants` block is non-conforming.

## 12. Source anchors

- `.t2/docs/Aureline_PRD.md` warm-startup and restore
  requirements — the shell may become interactive before
  remote sessions, notebooks, debuggers, or provider-linked
  surfaces are fully rebound, and live capability MUST never
  be overstated for partially rebound surfaces.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  startup and restore architecture — chooser, skeleton,
  hydrate, and rebind phases as the canonical restore order.
- `.t2/docs/Aureline_Technical_Design_Document.md` shell-ready
  and background-rebind sections — the `Ready` label MUST
  not be reused once the underlying state changes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` startup and
  restore copy — distinct cues for shell, search, semantic,
  and remote / provider rebind-complete.
- `.t2/docs/Aureline_Milestones_Document.md` warm-startup
  benchmarks — phase events MUST be readable from one trace
  vocabulary across spike-timing traces, diagnostics, support
  bundles, and release evidence.

## 13. Linked artifacts

- Hydration-phase event schema:
  [`/schemas/recovery/hydration_phase_event.schema.json`](../../schemas/recovery/hydration_phase_event.schema.json).
- Worked-example fixtures:
  [`/fixtures/recovery/hydration_phase_cases/`](../../fixtures/recovery/hydration_phase_cases/).
- Layout serialization contract (source of truth for the
  pane-tree schema and the original
  `chooser` → `skeleton` → `hydrate` → `rebind` →
  `evidence_only_fallback` order):
  [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md).
- Restore-artifact family contract (source of truth for the
  workspace-authority checkpoint and window-topology snapshot
  refs):
  [`/docs/state/restore_artifact_family_contract.md`](../state/restore_artifact_family_contract.md).
- Restore-provenance / placeholder contract (source of truth
  for missing-dependency placeholder cards):
  [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md).
- Restore-chooser contract (source of truth for the chosen
  recovery level and remembered-choice expiry):
  [`/docs/recovery/restore_chooser_contract.md`](./restore_chooser_contract.md).
- Crash-loop and restore-fidelity contract (source of truth
  for `restore_fidelity_class`):
  [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../ux/crash_loop_and_restore_fidelity_contract.md).
- Entry / restore truth audit (source of truth for
  `startup_state` tokens):
  [`/docs/ux/entry_restore_truth_audit.md`](../ux/entry_restore_truth_audit.md).
- Background-work queue contract (source of truth for
  shell-ready budget protection and index-warmup workload
  class):
  [`/docs/runtime/background_queue_contract.md`](../runtime/background_queue_contract.md).
- Recovery-ladder packet contract (source of truth for safe-
  mode and the rung sequence):
  [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md).
