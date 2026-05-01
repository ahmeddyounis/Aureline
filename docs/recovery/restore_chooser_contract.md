# Progressive recovery-level matrix, restore chooser, and remembered-choice expiry contract

This document freezes the cross-surface contract every startup,
crash-loop, diagnostics, support, and docs/help surface uses when it
answers a single question on a recovering launch:

> Given what failed, what is still safe to bring back, and which
> **recovery level** should the user be offered next?

Without this contract, that question collapses into one ambiguous
`Restore session?` prompt that:

- promises `Restore previous session` while the runtime cannot reattach,
  no dirty-buffer journal exists, and only forensics survive;
- silently skips a higher-risk recovery prompt because the user
  dismissed an earlier, lower-risk prompt for an unrelated failure;
- treats `Skip once` and `Open clean` as suppressing evidence the user
  never asked to discard;
- shows a single `Reopen` CTA that hides whether the system will
  rehydrate dirty buffers, walk a checkpoint rollback, or only render
  evidence.

The restore-chooser state record is the **shared inspectable body**
every startup, crash-loop, diagnostics, support, and docs/help surface
projects into the same five-class progressive recovery vocabulary. It
is **not** a session-restore engine, **not** a crash-recovery runner,
and **not** an automated fallback chain. It is the contract those
surfaces MUST conform to so progressive recovery stays
**level-coded** — exact session restore, context restore with
placeholders, dirty-buffer recovery, checkpoint rollback, and
evidence-only recovery — instead of collapsing into one generic
restore narrative.

The machine-readable schemas live at:

- [`/schemas/recovery/recovery_level.schema.json`](../../schemas/recovery/recovery_level.schema.json)
  — closed five-class recovery-level vocabulary, level-selection
  criteria, and per-level guarantees / forbidden claims.
- [`/schemas/recovery/restore_chooser_state.schema.json`](../../schemas/recovery/restore_chooser_state.schema.json)
  — restore-chooser state record (changed-since-failure summary,
  retained-evidence links, risk note, primary actions, remembered-
  choice expiry, packet/export linkage, honesty invariants).

Worked fixtures live under:

- [`/fixtures/recovery/recovery_level_cases/`](../../fixtures/recovery/recovery_level_cases/)

This contract composes with — and never re-defines — the recovery,
startup, restore, and reliability rules frozen elsewhere:

- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
  — entry-verb / target-kind / `restore_level` / missing-target /
  session-execution-posture / checkpoint-linked recovery vocabulary.
- [`/docs/ux/start_center_contract.md`](../ux/start_center_contract.md)
  — Start Center disclosure posture, restore-card record, and the
  three primary actions on the startup wedge.
- [`/docs/ux/recent_work_and_restore_card_contract.md`](../ux/recent_work_and_restore_card_contract.md)
  — recent-work row anatomy and per-row restore availability.
- [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../ux/crash_loop_and_restore_fidelity_contract.md)
  — crash-loop screen anatomy, restore-fidelity classes, and recovery
  action grammar.
- [`/docs/ux/entry_restore_truth_audit.md`](../ux/entry_restore_truth_audit.md)
  — `startup_state` tokens.
- [`/docs/reliability/recovery_scenario_contract.md`](../reliability/recovery_scenario_contract.md)
  — recovery-scenario card and safe-first-action matrix.
- [`/docs/reliability/autosave_journal_and_guided_replay_contract.md`](../reliability/autosave_journal_and_guided_replay_contract.md)
  — dirty-buffer journal and guided replay.
- [`/docs/reliability/local_history_restore_preview_contract.md`](../reliability/local_history_restore_preview_contract.md)
  — local-history snapshot and restore-preview vocabulary.
- [`/docs/reliability/continuity_status_card_contract.md`](../reliability/continuity_status_card_contract.md)
  — recovery-promise classes, restore-target inventory.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  — recovery-ladder rung packet (safe mode, extension quarantine,
  cache reset, restricted mode).
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  — support-bundle record shape; the chooser cites bundle refs by
  opaque id.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
  — chronology rules, remembered-decision waiver-expiry semantics,
  and timezone / monotonic timestamp distinctions the chooser reuses.

This contract is normative for the chooser disclosure posture. Where
it disagrees with the PRD, TAD, TDD, UI/UX spec, or one of the
upstream contracts above, those documents win and this contract plus
its schemas / fixtures MUST be updated in the same change. Where a
downstream startup, crash-loop, diagnostics, support, or docs/help
surface mints a parallel recovery-level vocabulary, this contract
wins and the surface is non-conforming.

This contract mints **no** new entry-verb, target-kind, resulting-
mode, restore-level, restore-fidelity, missing-target-state,
recovery-class, recovery-promise, scenario-family, probe-family,
approved-repair-class, completeness-outcome, or fault-domain values.
Every closed set is re-exported by reference from the upstream
contract that owns it.

## Why freeze this now

A recovering launch is the moment when copy drift hurts the user
most. A single ambiguous `Restore session?` prompt produces these
non-conforming patterns:

- **Level collapse.** The prompt says `Restore previous session`
  while only a recovery journal exists; the user expects exact
  layout and gets dirty-buffer recovery only.
- **Evidence loss by inference.** `Skip once` looks like `Discard`,
  so the user assumes the crash envelope and journals were deleted.
- **Suppression of higher-risk prompts.** A user dismissed yesterday's
  layout-only restore; today's launch carries a checkpoint-rollback
  candidate over a corrupt-restorable-state failure, but the prompt
  is suppressed because "the user already said no."
- **Suppression of newly available evidence.** A second launch
  recovered an additional evidence packet that wasn't visible at the
  first prompt; remembered dismissal hides it from the user.
- **Generic action chrome.** A single `Reopen` CTA hides whether the
  system will rehydrate dirty buffers, walk a checkpoint rollback,
  or only render forensics.
- **Cross-surface drift.** Diagnostics says `Compatible restore`, the
  Start Center card says `Restored last session`, and the support
  bundle says `Layout only`; reviewers cannot tell which is true.

The restore-chooser state record forecloses these patterns by
projecting one closed five-level vocabulary, one typed
changed-since-failure summary, typed retained-evidence links, one
typed risk note, one closed action set, and one closed remembered-
choice expiry-trigger set into a record every surface reads.

## Scope

Frozen here:

- one `recovery_level_class` closed five-class vocabulary —
  `exact_session_restore`, `context_restore_with_placeholders`,
  `dirty_buffer_recovery`, `checkpoint_rollback`,
  `evidence_only_recovery` — plus the deterministic selection
  criteria that bind a level to a startup posture;
- one `recovery_level_record` shape every chooser, diagnostics, and
  support surface emits to describe how the level was chosen for
  this launch (what evidence each level requires, what each level
  forbids claiming, what is restored, what reopens only as context
  / placeholder, what does not rerun);
- one `restore_chooser_state_record` shape that pairs the chosen
  level (and any lower-level fallbacks the chooser kept available)
  with a typed `changed_since_failure_summary`, typed retained-
  evidence links, one typed risk note, the closed primary-action
  set (`restore_now`, `skip_once`, `open_clean`, `safe_mode`, plus
  the optional companion actions), and the typed remembered-choice
  expiry block;
- the closed seven-class **expiry-trigger** vocabulary that names
  why a remembered dismissal / skip / open-clean decision MUST NOT
  suppress today's prompt: higher-risk recovery class available,
  newly available evidence (journal, checkpoint, packet), recovery-
  level class changed (e.g. yesterday's `layout_only`, today's
  `evidence_only`), restore manifest digest changed, build / schema
  identity changed, restore-fidelity downgrade, or remembered-
  decision waiver TTL elapsed;
- the typed **packet/export linkage** rules so startup, diagnostics,
  support, and docs/help all cite the same `recovery_level_record`
  and the same retained-evidence ids by opaque ref;
- **honesty invariants** — closed level vocabulary, named retained
  evidence, primary-action separation (no generic `Reopen`), no
  silent expiry shadowing, typed packet/export refs.

Out of scope:

- the actual session-restore engine, crash-recovery runner, or
  watchdog implementation;
- crash-reporting transport, symbolication execution, or hosted
  ticket submission;
- hierarchical fault-domain restart-budget arithmetic — the
  chooser cites the fault-domain id and budget by opaque ref and
  does not re-derive restart truth;
- final user-facing copy / microcopy or visual layout — those are
  pinned by the UX style guide and shell-zone contract;
- changing the upstream `restore_prompt_record`,
  `crash_loop_card_record`, `recovery_ladder_packet_record`, or
  `support_bundle_record` shapes.

## 1. Record model

The chooser emits two record shapes:

| Record | Purpose |
|---|---|
| `recovery_level_record` | One per chosen level. Names the closed five-class level, the deterministic selection-criterion class, what is restored, what reopens only as context or placeholder, what does not rerun, the level-specific forbidden-claim set, and the upstream `restore_level` / `restore_fidelity_class` reflection. |
| `restore_chooser_state_record` | One per recovering launch. Names the chosen `recovery_level_record` plus lower-level fallbacks the chooser kept available, the typed `changed_since_failure_summary`, retained-evidence links, the risk note, the closed primary-action set, the typed remembered-choice expiry block, the packet/export linkage, and the honesty invariants. |

A given launch emits at most one `restore_chooser_state_record`.
Multiple recovering paths in the same launch (e.g. one workspace
restoring exact, another restoring evidence-only after corruption)
emit one record each, scoped to their workspace authority.

## 2. Recovery-level vocabulary

Five closed levels. Every chooser resolves the chosen level to
exactly one. The set, rendered names, and selection criteria are
re-exported into the schema's `recovery_level_class` enum and into
[`/artifacts/recovery/safe_first_action_matrix.yaml`](../../artifacts/recovery/safe_first_action_matrix.yaml)
on a separate row.

| Level | Rendered name | When chosen | Restores | Reopens only as context / placeholder | Does not rerun |
|---|---|---|---|---|---|
| `exact_session_restore` | `Exact session restore` | The prior runtime survived and the same compatible state contract holds: workspace authority, build identity, extension manifest digests, monitor topology, and remote-target / managed-session authority all match the prior launch. | windows, pane tree, editor identity, dirty buffers, cursor / scroll, stable inspectors, non-mutating panels | n/a (no placeholder reopens at this level) | tasks, tests, debug attach, notebook cells, AI tool calls, publish / promote actions, remote mutations |
| `context_restore_with_placeholders` | `Context restore with placeholders` | Prior session can be translated through a compatible schema, version, or topology adjustment, OR one or more dependencies are missing (extension host, remote target, managed session, devcontainer, toolchain, profile, monitor topology, version change) so live state cannot be claimed. | layout, file / editor identity, local text state, panel positions, stable metadata | missing extension host, remote target, managed session, monitor topology, version change, devcontainer, toolchain, profile | live runtime reattach, silent reauth, silent reconnect, command rerun |
| `dirty_buffer_recovery` | `Dirty-buffer recovery` | Recovery journal carries unsaved bytes; full layout restore is unsafe or impossible. | unsaved text from journal, local autosave snapshots, journal identity | layout (where present) | save to disk, push to remote, send to external service |
| `checkpoint_rollback` | `Checkpoint rollback` | A typed checkpoint exists for the failing scope (workspace-authority, session, migration, settings-backup, local-history) and the rollback covers the failure cause. | last-known-good state at the named checkpoint | adjacent state outside the checkpoint scope | compensating side effects without naming them as compensating; calling rollback "undo" when it is `compensating_rollback` |
| `evidence_only_recovery` | `Evidence-only recovery` | The UI state cannot be safely restored; the chooser exposes the crash envelope, logs, restore diagnostics, forensic packets, and missing-dependency placeholders for inspection / export. | nothing live | n/a (no UI rehydration) | any prior pane, any task, any AI tool call, any rerun |

Rules (frozen):

1. **Exactly one chosen level per record.** A chooser that lists two
   "current" levels (`exact_session_restore` AND
   `context_restore_with_placeholders`) is non-conforming. Lower
   levels MAY appear in `available_lower_level_fallbacks[]` so the
   user can step down deliberately, but the chosen level is one.
2. **Down-only fallback.** `available_lower_level_fallbacks[]` MUST
   list strictly lower levels than the chosen level (e.g. a
   `checkpoint_rollback` chosen-level may offer
   `dirty_buffer_recovery` and / or `evidence_only_recovery` as
   fallbacks but never `exact_session_restore`).
3. **Selection is deterministic.** Each level cites exactly one
   `selection_criterion_class` (§2.1). The chooser does not invent
   a private criterion; a record without a typed criterion is
   non-conforming.
4. **Level pairs with `restore_level`.** Every record reflects a
   `restore_level_ref` from the upstream `restore_prompt_record`
   (entry-restore §3.1). The mapping is one-to-many but never
   contradictory:
   - `exact_session_restore` → `restore_level: exact_restore`;
   - `context_restore_with_placeholders` → `compatible_restore` or
     `layout_only`;
   - `dirty_buffer_recovery` → `recovered_drafts`;
   - `checkpoint_rollback` → any `restore_level` (the chosen
     `checkpoint_linked_recovery_class` carries the rollback
     semantics);
   - `evidence_only_recovery` → `evidence_only` or `no_restore`.
5. **Level pairs with `restore_fidelity_class`.** Every record
   reflects a `restore_fidelity_class_ref` from the crash-loop /
   restore-fidelity contract. The chooser MUST NOT promise a higher
   fidelity than the underlying class declares.
6. **Per-level forbidden claims.** Each level lists at least one
   `forbidden_claim_class` value (§2.2). The chooser refuses to
   render claim phrases the level forbids; surfaces that bypass
   this rule by emitting free-text restore prose are
   non-conforming.

### 2.1 Selection-criterion classes

Closed nine-class vocabulary. Every level cites exactly one. The
criterion is the deterministic reason the level was chosen — not a
free-text rationale.

- `prior_runtime_survived_compatible_contract` — exact-restore
  precondition. The runtime actually survived and the build /
  manifest / topology contract still matches.
- `compatible_translation_available` — compatible-restore
  precondition. State translates through a schema / version /
  topology adjustment with visible notes.
- `missing_dependency_layout_only` — layout-only precondition. One
  or more dependencies are missing; layout survives but live state
  does not.
- `recovery_journal_present_layout_unsafe` — dirty-buffer
  precondition. A recovery journal carries unsaved bytes; full
  layout restore is unsafe or impossible.
- `typed_checkpoint_covers_failure` — checkpoint-rollback
  precondition. A typed checkpoint covers the failing scope with a
  named `checkpoint_linked_recovery_class`.
- `corrupt_restorable_state_quarantined` — evidence-only
  precondition (corruption variant). The restorable state is
  quarantined; only forensics remain.
- `bounded_restart_exhausted_no_safe_path` — evidence-only
  precondition (crash-loop variant). The fault-domain restart
  budget is exhausted; no safe rehydration path exists.
- `policy_blocked_restore_evidence_remains` — evidence-only
  precondition (policy variant). Policy forbids restore on this
  envelope; evidence remains inspectable.
- `no_recoverable_state_evidence_only` — evidence-only
  precondition (residual variant). No layout, no dirty buffer, no
  checkpoint, no compatible runtime — only the crash envelope and
  logs remain.

The schema enforces that the criterion's allowed level set matches
the chosen level (e.g. `prior_runtime_survived_compatible_contract`
is the only criterion permitted on `exact_session_restore`).

### 2.2 Forbidden-claim classes

Closed eight-class vocabulary. Each level lists at least one. The
chooser MUST refuse to emit copy that asserts the forbidden claim;
surfaces that bypass by free-text wording are non-conforming.

- `claim_runtime_reattached` — forbidden by every level except
  `exact_session_restore` (and only when the runtime truly
  survived).
- `claim_remote_session_reauthed` — forbidden whenever the upstream
  authority did not actually carry across the launch.
- `claim_silent_command_rerun` — forbidden by every level. The
  shell never reruns mutating commands as part of restore.
- `claim_pixel_perfect_layout` — forbidden by `compatible_restore`
  and below.
- `claim_save_to_disk` — forbidden by `dirty_buffer_recovery`. The
  journal is not a save.
- `claim_undo_for_compensating_rollback` — forbidden by
  `checkpoint_rollback` when the bound recovery class is
  `compensating_rollback` or `regenerate_from_canonical_source`.
- `claim_evidence_was_deleted_by_skip` — forbidden by every level.
  `Skip once` and `Open clean` retain evidence.
- `claim_live_readiness_for_placeholder` — forbidden by every level
  that includes a placeholder pane; placeholder panes never claim
  live readiness.

## 3. Restore-chooser state record

Every recovering launch that surfaces a chooser emits one
`restore_chooser_state_record`. The shell, crash-loop screen,
diagnostics panel, support-export preview, and docs/help example
read this record verbatim; surfaces that mint a parallel chooser
record are non-conforming.

### 3.1 Required fields

| Field | Purpose |
|---|---|
| `record_kind = restore_chooser_state_record` | Discriminator. |
| `restore_chooser_state_schema_version = 1` | Const integer; additive-minor changes bump. |
| `chooser_id` | Stable opaque id for cross-surface linkage. |
| `emitted_at` | Producer-local monotonic timestamp. Opaque to the contract; the surface never re-reads system wall-clock from this field. |
| `surface_family` | One of `start_center_chooser`, `crash_loop_chooser`, `diagnostics_restore_panel`, `support_export_preview`, `docs_help_example`. The set is closed. |
| `startup_state_ref` | A `startup_state` token from entry-restore truth audit §6 (e.g. `startup_state:reopen_with_pending_restore`, `startup_state:restore_failed`, `startup_state:open_without_restore`). |
| `chosen_level_record` | One inline `recovery_level_record`. |
| `available_lower_level_fallbacks[]` | Optional ordered list of lower-level `recovery_level_record`s the chooser kept available. Strictly lower than the chosen level. |
| `changed_since_failure_summary` | §4. |
| `retained_evidence_links` | §5. |
| `risk_note` | §6. |
| `primary_actions[]` | Ordered list of `chooser_primary_action_record`s. MUST include `restore_now`, `skip_once`, `open_clean`, and `safe_mode` on every non-takeover record (§7). |
| `companion_actions[]` | Optional companion actions (`compare_to_disk`, `open_journal`, `disable_suspect_extension`, `export_evidence`, `open_logs`, `retry_reopen`, `open_recovery_ladder`). |
| `remembered_choice_expiry` | §8. |
| `packet_export_linkage` | §9. |
| `accessibility_contract` | Keyboard-reachable actions, focus target ref, screen-reader summary. Re-uses the `accessibility_contract` shape from the crash-loop card. |
| `honesty_invariants` | Const-true block (§10). |

### 3.2 Cross-surface uniformity

A `chooser_id` MUST be reusable across the Start Center chooser,
the crash-loop screen chooser, the diagnostics panel, and the
support-export preview for a single recovering launch. A
diagnostics panel that mints a parallel `chooser_id` for the same
launch — or that summarises a different chosen level than the
Start Center chooser — is non-conforming.

### 3.3 Surface family rules

1. `start_center_chooser` is the only family that pairs with the
   Start Center `restore_card_record` and renders inside
   `primary_work_resume`. It MUST cite the upstream
   `restore_prompt_ref` and `restore_card_ref`.
2. `crash_loop_chooser` is the only family that pairs with the
   `crash_loop_card_record`. It MUST cite the upstream
   `crash_loop_card_ref`, `fault_domain_id`, and
   `last_failure_summary` ref.
3. `diagnostics_restore_panel` and `support_export_preview` MUST
   reflect the same chosen level and the same retained-evidence
   ids the launch chooser exposed; they MUST NOT introduce a new
   chosen level.
4. `docs_help_example` MUST cite the same vocabulary and the same
   action ids; help text that softens a level (e.g. calling
   `evidence_only_recovery` "almost restored") is non-conforming.

## 4. Changed-since-failure summary

Every chooser record names what changed between the failing launch
and this launch so the user can see why today's offer differs from
yesterday's. The block is typed; free-text "things changed" prose
is non-conforming.

### 4.1 Fields

- `prior_failure_id` — opaque id of the failing launch (matches
  the upstream `last_failure_summary.crash_id`).
- `prior_failure_class` — re-exported from the crash-loop
  `exit_reason_class` set.
- `prior_recovery_level_record_ref` — opaque ref to the previously
  chosen `recovery_level_record` (when one was emitted). Null on
  a first failure.
- `changed_since_class[]` — closed set; at least one entry when the
  chooser is offered after a remembered dismissal or a prior
  failure. Multiple entries are allowed.
- `summary` — short, redaction-aware text restating the typed
  classes for the user.

### 4.2 `changed_since_class` vocabulary

Closed seven-class set:

- `higher_risk_recovery_class_available` — a
  `checkpoint_linked_recovery_class` exists today that did not
  exist at the prior failure (e.g. a workspace-authority checkpoint
  was captured between attempts).
- `newly_available_evidence` — at least one new evidence packet,
  forensic packet, journal entry, or local-history snapshot was
  retained since the last chooser was shown.
- `recovery_level_class_changed` — today's chosen level differs
  from the prior chosen level (e.g. yesterday `layout_only`,
  today `evidence_only`).
- `restore_manifest_digest_changed` — the upstream
  `restore_manifest_ref` digest changed (workspace authority,
  open editors, dirty-buffer set, checkpoint set, evidence set
  drifted).
- `build_or_schema_identity_changed` — the running build identity,
  extension manifest digest, profile schema, or workspace schema
  is different from the prior failing launch.
- `restore_fidelity_downgrade` — the underlying
  `restore_fidelity_class` is lower than the prior launch's class
  (e.g. prior `compatible_restore`, current `evidence_only`).
- `remembered_decision_ttl_elapsed` — the remembered-decision
  retention window for the prior dismissal / skip / open-clean
  decision elapsed (see §8 expiry rules).

The schema enforces that at least one entry is present whenever
`remembered_choice_expiry.was_prior_decision_remembered` is true;
silent expiry without a typed reason is non-conforming.

## 5. Retained-evidence links

The chooser cites every evidence anchor by opaque ref so the user
is never told the system is broken without a named anchor of what
remains inspectable, exportable, or operable. Free-text "logs are
saved somewhere" prose is non-conforming.

### 5.1 Fields

- `crash_envelope_ref` — opaque id (re-exported from the crash-loop
  `local_forensics_surface.crash_envelope_ref`).
- `restore_manifest_ref` — opaque id of the upstream
  `restore_prompt_record` / restore manifest.
- `recovery_journal_refs[]` — opaque ids of dirty-buffer journals
  retained for this scope.
- `local_history_snapshot_refs[]` — opaque ids of local-history
  snapshots eligible for restore.
- `checkpoint_refs[]` — opaque ids of typed checkpoints
  (workspace-authority, session, migration, settings-backup) the
  chooser may walk.
- `evidence_packet_refs[]` — opaque ids of evidence packets
  retained for this launch.
- `forensic_packet_refs[]` — opaque ids of forensic packets
  retained for this launch.
- `support_bundle_candidate_ref` — opaque id of the candidate
  support-bundle the user may export. Null when no bundle is
  staged.
- `redaction_class` — re-exported from the crash-loop
  `redaction_class` set.
- `export_posture` — re-exported from the crash-loop
  `export_posture` set.
- `evidence_retained_after_skip` — boolean; const `true` on every
  conforming chooser. `Skip once`, `Open clean`, and remembered
  expiry never delete evidence.

### 5.2 Rules

1. **At least one anchor.** Every chooser MUST cite at least one
   non-null evidence anchor (crash envelope, restore manifest,
   recovery journal, checkpoint, evidence packet, or forensic
   packet). A chooser with all anchors null is non-conforming.
2. **Typed refs only.** Every anchor is an opaque ref, never a raw
   path, raw URL, raw credential, or raw provider payload.
3. **Skip-and-expire never deletes.** `evidence_retained_after_skip`
   is const `true`. Surfaces that imply `Skip once` or remembered-
   expiry deleted evidence are non-conforming.

## 6. Risk note

Every chooser carries one typed risk note explaining what could go
wrong if the user picks `Restore now` at the chosen level. Free-text
risk prose is non-conforming.

### 6.1 Fields

- `risk_class` — closed nine-class vocabulary (§6.2).
- `reversibility_class` — re-exported from
  `schemas/support/recovery_action.schema.json#reversal_class`
  (`exact_undo`, `compensating_action`, `regeneration`,
  `checkpoint_restore`, `no_undo_export_only`).
- `requires_pre_action[]` — optional; drawn from the recovery-
  scenario contract's pre-action set
  (`export_now_before_change`, `capture_local_checkpoint`,
  `import_offline_bundle`, `investigate_with_project_doctor`).
- `no_undo_acknowledgement_required` — boolean; const `true` when
  `risk_class` is `destructive_user_authored_no_undo_export_required`.
- `summary` — short, redaction-aware text.

### 6.2 `risk_class` vocabulary

Closed nine-class set. The first seven re-export the recovery-
scenario contract's destructive-risk vocabulary; the last two are
chooser-specific and cover the chooser-only restore postures the
recovery-scenario card does not cover (since the scenario card
covers large-failure recovery, not progressive restore selection).

- `non_destructive_read_only` — pure read; no writes anywhere
  (e.g. opening evidence-only chooser).
- `non_destructive_writes_local_evidence_only` — writes only
  the local evidence lane.
- `writes_disposable_state_only` — recreates derived disposable
  state during restore (cache rebuild on first paint).
- `writes_user_owned_recovery_state` — writes only into local-
  history / autosave / workspace-authority checkpoint state owned
  by the user.
- `mutates_workspace_bytes_with_checkpoint` — restore mutates
  workspace bytes; a workspace-authority checkpoint MUST precede.
- `mutates_profile_state_with_checkpoint_and_export` — restore
  mutates profile-wide durable state; checkpoint + export
  pre-actions required.
- `destructive_user_authored_no_undo_export_required` — the
  chosen level mutates user-authored durable state without an
  authoritative restore source. Reset-only verb. Export pre-action
  is required; no-undo acknowledgement is required.
- `restore_no_rerun_no_reattach` — chooser-specific. The chosen
  level rehydrates layout / dirty buffers / placeholders only;
  no command rerun, no reattach, no reauth.
- `evidence_only_no_state_change` — chooser-specific. The chooser
  exposes evidence; nothing about the prior session is rehydrated.

The schema enforces that `evidence_only_recovery` carries
`evidence_only_no_state_change`; that `dirty_buffer_recovery`
carries `writes_user_owned_recovery_state` or
`restore_no_rerun_no_reattach`; that `checkpoint_rollback` carries
one of `writes_user_owned_recovery_state`,
`mutates_workspace_bytes_with_checkpoint`, or
`mutates_profile_state_with_checkpoint_and_export`; and that
`destructive_user_authored_no_undo_export_required` is forbidden
on every level except `checkpoint_rollback` paired with an
explicit `compensating_rollback` recovery class plus the typed
pre-actions.

## 7. Primary actions

Every chooser carries the four required primary actions. Two
companion actions are also allowed when the chosen level supports
them; six other companion actions ride from the crash-loop card
without redefinition.

### 7.1 Required actions

The four ids below MUST appear on every non-takeover chooser:

- `restore_now` — commit the chosen level. Routes to the upstream
  restore prompt's commit path.
- `skip_once` — dismiss this chooser for this launch only;
  evidence remains, the prompt is retained for evidence, and a
  later launch MAY re-prompt under the §8 expiry rules.
- `open_clean` — explicit `open_without_restore` decision. Routes
  to `startup_state:open_without_restore`. Evidence remains.
- `safe_mode` — enter the recovery-ladder safe-mode rung (re-
  exported from the recovery-ladder packet contract). Available
  on every chooser; the rung's preserves / discards-or-defers
  effects are read from the rung packet, not redefined here.

### 7.2 Companion actions

Companion actions reuse the crash-loop card's `action_id` and
`action_record` shapes (re-exported, not redefined):

- `compare_to_disk` — available when the chosen level is
  `dirty_buffer_recovery` and a target file identity exists.
- `open_journal` — available when a recovery journal exists.
- `disable_suspect_extension` — available when the upstream crash-
  loop card names a suspect extension.
- `export_evidence` — always available when an evidence anchor
  exists.
- `open_logs` — always available when a log ref exists.
- `retry_reopen` — available only when the restart budget allows
  another automatic attempt; otherwise disabled with reason
  `restart_budget_exhausted`.
- `open_recovery_ladder` — always available; routes to the
  recovery-ladder packet record.

### 7.3 Action rules

1. **No generic `Reopen` CTA.** A chooser that collapses
   `restore_now` and `skip_once` into one button — or that hides
   the chosen level under a single `Continue` button — is
   non-conforming.
2. **Action labels reflect the level.** The rendered label of
   `restore_now` MUST quote the chosen level verbatim (e.g.
   `Restore now (Exact session restore)`,
   `Restore now (Dirty-buffer recovery)`,
   `Open evidence` for `evidence_only_recovery`). A chooser that
   reads `Restore now` without naming the level is non-conforming.
3. **Skip and Open clean retain evidence.** Both actions carry
   `evidence_retained_after_action = true`. Surfaces that imply
   either action deleted evidence are non-conforming.
4. **Safe mode is always reachable.** `safe_mode` is always
   present on every chooser, regardless of chosen level. A
   chooser without `safe_mode` is non-conforming.
5. **Keyboard reachability.** Every required and companion action
   MUST be keyboard-reachable. The chooser's
   `accessibility_contract.keyboard_reachable_actions` is `true`
   on every conforming record.

## 8. Remembered-choice expiry rules

Remembered dismissals, `Skip once` decisions, and explicit
`Open clean` decisions are durable per-workspace facts. They MUST
NOT silently suppress today's chooser when any of the typed
expiry triggers fire. The chooser carries one
`remembered_choice_expiry` block.

### 8.1 Fields

- `was_prior_decision_remembered` — boolean. `true` when a
  remembered dismissal / skip / open-clean decision was on file
  for this scope at the moment the chooser was prepared.
- `prior_decision_class` — closed three-class set
  (`skip_once`, `open_clean`, `safe_mode_taken`). Required when
  `was_prior_decision_remembered` is `true`.
- `prior_decision_at` — opaque monotonic timestamp of the prior
  decision. Required when `was_prior_decision_remembered` is
  `true`.
- `remembered_decision_ttl_class` — closed five-class set
  (`session_only`, `until_workspace_close`, `until_build_change`,
  `bounded_window_short`, `bounded_window_long`). Chooser-
  scoped TTL classes; absolute / monotonic timestamp semantics
  follow the governance chronology rules from
  [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md).
- `expiry_triggered` — boolean. `true` when the chooser is
  shown despite a remembered decision because at least one
  `expiry_trigger_class` fired.
- `expiry_trigger_classes[]` — closed seven-class set (§8.2).
  Required to be non-empty when `expiry_triggered` is `true`.
- `still_suppressed_until_class` — closed three-class set
  (`not_suppressed`, `next_failure_event`,
  `next_evidence_change`). Names the next expiry semantics if
  the user dismisses again.
- `summary` — short, redaction-aware text restating the typed
  trigger classes.

### 8.2 `expiry_trigger_class` vocabulary

Closed seven-class set. At least one MUST fire whenever a
remembered decision is overridden:

- `higher_risk_recovery_class_available` — a `checkpoint_linked_
  recovery_class` is available today that was not at the prior
  decision (e.g. workspace-authority checkpoint captured since).
  The chooser MUST be shown.
- `newly_available_evidence` — a new evidence packet, forensic
  packet, journal entry, or local-history snapshot was retained
  since the prior decision. The chooser MUST be shown.
- `recovery_level_class_changed` — the chosen level today differs
  from the level the user dismissed (e.g. yesterday `layout_only`,
  today `evidence_only`). The chooser MUST be shown.
- `restore_manifest_digest_changed` — the restore manifest digest
  changed between the prior decision and this launch.
- `build_or_schema_identity_changed` — the running build,
  extension manifest digest, profile schema, or workspace schema
  changed.
- `restore_fidelity_downgrade` — the underlying
  `restore_fidelity_class` is lower than the prior class (a
  silent downgrade is non-conforming).
- `remembered_decision_ttl_elapsed` — the remembered-decision TTL
  class elapsed (e.g. `bounded_window_short` window passed).

### 8.3 Rules

1. **No silent shadowing.** A chooser whose
   `was_prior_decision_remembered = true`,
   `expiry_triggered = false`, and chosen level is anything other
   than the level the user previously dismissed is
   non-conforming. Either the prior decision still suppresses
   (the chooser is not shown at all) or an expiry trigger fires
   and the typed class is named.
2. **Higher-risk overrides remembered skip.** A remembered
   `skip_once` MUST NOT suppress a chooser whose risk class is
   higher than the prior chooser's risk class
   (`mutates_*` > `writes_*` > `non_destructive_*`).
3. **Newly available evidence overrides remembered skip.** A
   remembered `open_clean` decision MUST NOT suppress today's
   chooser when at least one new
   `evidence_packet_ref` / `forensic_packet_ref` /
   `recovery_journal_ref` / `local_history_snapshot_ref` /
   `checkpoint_ref` was retained since the prior decision.
4. **TTL is per-scope.** Remembered decisions are scoped to
   workspace authority. A skip on workspace A MUST NOT suppress
   a chooser on workspace B even when the same recovery level
   class applies.
5. **Trigger drives `changed_since_class`.** When a trigger
   fires, the chooser's `changed_since_failure_summary.changed_
   since_class[]` MUST include the matching trigger class
   (e.g. `expiry_trigger_class:newly_available_evidence` →
   `changed_since_class:newly_available_evidence`). A chooser
   with mismatched trigger / changed-since classes is
   non-conforming.

## 9. Packet / export linkage

Every chooser carries one typed `packet_export_linkage` block.
Free-form prose linkage is non-conforming.

### 9.1 Fields

- `restore_prompt_ref` — opaque ref to the upstream
  `restore_prompt_record` (entry-restore object model §3).
- `restore_card_ref` — opaque ref to the Start Center
  `restore_card_record` (when a Start Center card is rendered).
- `crash_loop_card_ref` — opaque ref to the
  `crash_loop_card_record` (when a crash-loop screen is
  rendered). Required when `surface_family` is
  `crash_loop_chooser`.
- `recovery_scenario_card_ref` — opaque ref to a
  `recovery_scenario_card_record` (when a large-failure scenario
  applies). Required when the chosen criterion is one of
  `corrupt_restorable_state_quarantined`,
  `bounded_restart_exhausted_no_safe_path`,
  `policy_blocked_restore_evidence_remains`, or
  `no_recoverable_state_evidence_only`.
- `recovery_ladder_packet_ref` — opaque ref to the recovery-
  ladder packet record. Always required so `Safe mode` and the
  rung sequence remain reachable.
- `support_bundle_candidate_ref` — opaque ref to the staged
  support-bundle (when present).
- `evidence_packet_refs[]` — opaque ids of evidence packets
  retained.
- `forensic_packet_refs[]` — opaque ids of forensic packets
  retained.
- `local_history_snapshot_refs[]` — opaque ids of local-history
  snapshots retained.
- `recovery_journal_refs[]` — opaque ids of recovery journals
  retained.
- `checkpoint_refs[]` — opaque ids of typed checkpoints
  retained.
- `docs_help_label` — short, redaction-aware text used by
  docs/help and support-export previews.

### 9.2 Rules

1. **Same chosen level across surfaces.** Startup, diagnostics,
   support, and docs/help that cite the same `chooser_id` MUST
   reflect the same chosen `recovery_level_class`. A diagnostics
   panel that names a different level than the Start Center
   chooser is non-conforming.
2. **Retained-evidence ids are stable.** Evidence ids cited in
   `retained_evidence_links` (§5) MUST match the ids cited in
   `packet_export_linkage`; a chooser with two divergent ref
   sets is non-conforming.
3. **Recovery-ladder always reachable.** A chooser without a
   `recovery_ladder_packet_ref` is non-conforming. The recovery
   ladder is the supportable next step on every level.

## 10. Honesty invariants

Every chooser MUST carry the `honesty_invariants` block with five
const-`true` fields:

- `recovery_level_is_closed: true` — the chooser resolves to
  exactly one of the five closed levels. No private level.
- `retained_evidence_named: true` — at least one evidence anchor
  is named. The user is never told the system failed without a
  named anchor.
- `primary_actions_distinct: true` — the four required actions
  appear distinctly. No generic `Reopen` collapse.
- `expiry_is_typed: true` — remembered-choice expiry is
  represented by the typed `expiry_trigger_classes[]` set;
  silent shadowing or free-text expiry is forbidden.
- `linkage_is_typed: true` — packet/export linkage refs are
  typed (restore prompt, restore card, crash-loop card,
  scenario card, recovery-ladder packet, evidence / forensic /
  journal / checkpoint / local-history ids); free-text linkage
  prose is non-conforming.

These are const guarantees in the schema. Any surface that emits
a chooser without them is non-conforming.

## 11. Surface rules

Apply to every surface that renders, logs, exports, or reasons
about restore-chooser state records.

1. **No private recovery level.** Every consumer resolves to one
   of the five closed levels; surfaces do not render a parallel
   "almost restored" or "safe enough" level.
2. **One chosen level per record.** Choosers do not list two
   chosen levels in one breath. Lower-level fallbacks ride in
   `available_lower_level_fallbacks[]`.
3. **No generic `Reopen`.** The four required primary actions
   render distinctly with their level-aware labels.
4. **No claim above the level.** A chooser that promises live
   readiness, runtime reattach, command rerun, or save-to-disk
   when the chosen level forbids the claim (§2.2) is
   non-conforming.
5. **Evidence retained on every action.** `Skip once`,
   `Open clean`, `Safe mode`, remembered expiry, and dismissal
   never delete evidence. Surfaces that infer deletion from
   absence are non-conforming.
6. **No silent expiry.** A chooser shown despite a remembered
   decision MUST cite at least one typed expiry trigger.
7. **Cross-surface consistency.** Startup, diagnostics, support,
   and docs/help reflect the same chosen level, the same
   retained-evidence ids, and the same risk note.
8. **Support-bundle linkage stays opaque.** Raw paths, raw
   credentials, raw URLs, raw provider payloads, and raw
   terminal scrollback never appear in a chooser record.
9. **Recovery-ladder always linked.** Every chooser cites a
   `recovery_ladder_packet_ref`; `Safe mode` is the supportable
   next step on every level.
10. **Workspace-scoped suppression.** Remembered decisions are
    workspace-scoped; cross-workspace suppression is
    non-conforming.

## 12. Composition with adjacent contracts

- **Entry / restore object model** owns `restore_level`,
  `missing_target_state`, `session_execution_posture`, and
  `checkpoint_linked_recovery_class`. The chooser cites these
  by ref; it never re-derives them.
- **Start Center contract** owns the restore-card primary actions
  (`Restore now`, `Skip once`, `Open clean`) on the startup wedge.
  The chooser quotes the same ids and adds `Safe mode`; it never
  redefines the card.
- **Crash-loop and restore-fidelity contract** owns
  `restore_fidelity_class`, the crash-loop screen anatomy, and
  the recovery action grammar (`compare_to_disk`, `open_journal`,
  `disable_suspect_extension`, `export_evidence`, `open_logs`,
  `retry_reopen`). The chooser re-uses these `action_id` values
  without redefinition.
- **Recovery-scenario contract** owns large-failure scenarios
  (profile corruption, workspace-index corruption, failed update,
  control-plane outage, device replacement, seat loss,
  credential-store unreadable, mirror-or-offline-bundle
  unavailable). The chooser cites a `recovery_scenario_card_ref`
  when the chosen criterion implies one of those scenarios.
- **Recovery-ladder packet contract** owns the rung sequence
  (safe mode → extension quarantine → cache reset → restricted
  mode). The chooser cites a `recovery_ladder_packet_ref`; the
  rung packet is the source of truth.
- **Autosave-journal contract** owns the dirty-buffer journal and
  guided replay. The chooser cites
  `recovery_journal_refs[]`; `dirty_buffer_recovery` is the
  chooser's level for replay choice.
- **Local-history restore-preview contract** owns local-history
  snapshots. The chooser cites
  `local_history_snapshot_refs[]`; the preview contract owns the
  preview record.
- **Continuity-status contract** owns recovery-promise classes
  (`authoritative_backup`, `local_checkpoint`, `sync_replica`,
  `mirror_cache`, `convenience_export`). The chooser cites a
  continuity-status card ref when the chosen level uses a
  recovery-promise covering source.
- **Governance record-state model** owns chronology rules and
  remembered-decision waiver TTL classes. The chooser cites
  `remembered_decision_ttl_class` from that model; expiry
  semantics never re-derive timezone or skew rules.
- **Support-bundle contract** owns bundle records. The chooser
  cites `support_bundle_candidate_ref`; bundles are not
  re-defined here.

## 13. Acceptance

- **Levels stay distinct.** The five `recovery_level_class`
  values are rendered verbatim across startup, crash-loop,
  diagnostics, support, and docs/help. No surface flattens
  exact session restore, context restore with placeholders,
  dirty-buffer recovery, checkpoint rollback, and evidence-only
  recovery into one generic restore state.
- **The chooser explains what restores, what reopens as
  context / placeholder, and what does not rerun.** Every
  `recovery_level_record` carries the three lists (§2). The
  chooser reads them verbatim.
- **Remembered decisions expire deterministically.** A remembered
  dismissal / skip / open-clean decision MUST NOT suppress
  today's chooser when any of the seven typed expiry triggers
  fires; silent shadowing is non-conforming.
- **Linkage stays typed.** Retained evidence and packet/export
  refs are opaque ids; free-text linkage prose is
  non-conforming.
- **Fixtures.** The fixtures under
  [`/fixtures/recovery/recovery_level_cases/`](../../fixtures/recovery/recovery_level_cases/)
  cover at least: exact session restore, context restore with
  placeholders, dirty-buffer recovery, checkpoint rollback,
  evidence-only recovery, remembered-skip overridden by
  newly available evidence, and remembered-skip overridden by
  higher-risk recovery class.

## 14. Changing this contract

- **Additive-minor** changes (new `recovery_level_class`,
  new `selection_criterion_class`, new
  `forbidden_claim_class`, new `changed_since_class`,
  new `expiry_trigger_class`, new `surface_family`) land in
  this document, both schemas, and at least one fixture in the
  same change. The change must cite the motivating fixture or
  packet.
- **Repurposing** an existing recovery level, selection
  criterion, expiry trigger, or honesty invariant is
  **breaking**. It opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section here.
- The schemas are the boundary. Any surface that adds a private
  field, collapses two levels, or emits a record without the
  `honesty_invariants` block is non-conforming.

## 15. Source anchors

- `.t2/docs/Aureline_PRD.md` §5.25 (line 1300) — crash recovery
  MUST degrade gracefully from exact session restore to
  recover dirty buffers to open clean with preserved evidence,
  never directly to silent loss.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` line 836–842 — the
  five recovery levels: Exact session restore, Context restore
  with placeholders, Dirty-buffer recovery, Checkpoint rollback,
  Evidence-only recovery.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` line 866–871 —
  controlled restore-fidelity terms reused in diagnostics, crash
  screens, support bundles, and docs/help.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` line 2257–2296 —
  session restore and crash-loop containment rules; restore
  summaries name how many windows / dirty buffers / transient
  tasks / notebooks / terminals / remote sessions / evidence
  packets were found before rehydration; users and support can
  inspect, export, clear, or ignore restore state without
  deleting unrelated workspace, profile, or evidence state.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6,
  §9.7, Appendix CP — control-plane / data-plane separation,
  recovery posture, and the local-history / checkpoint matrix.
- `.t2/docs/Aureline_Milestones_Document.md` line 1023 — Start
  Center keeps `Open`, `Clone`, `Import`, `Restore`, and
  `Recent work` distinct; restore prompts name the recovery
  class, resulting mode, and primary actions
  (`Restore now`, `Skip once`, `Open clean`) rather than one
  generic reopen CTA.

## 16. Linked artifacts

- Recovery-level schema:
  [`/schemas/recovery/recovery_level.schema.json`](../../schemas/recovery/recovery_level.schema.json).
- Restore-chooser state schema:
  [`/schemas/recovery/restore_chooser_state.schema.json`](../../schemas/recovery/restore_chooser_state.schema.json).
- Worked-example fixtures:
  [`/fixtures/recovery/recovery_level_cases/`](../../fixtures/recovery/recovery_level_cases/).
- Entry / restore object model (source of truth for
  `restore_level`, `missing_target_state`,
  `session_execution_posture`, `checkpoint_linked_recovery_class`):
  [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md).
- Start Center contract (source of truth for the restore-card
  three primary actions and Start Center disclosure posture):
  [`/docs/ux/start_center_contract.md`](../ux/start_center_contract.md).
- Crash-loop and restore-fidelity contract (source of truth for
  `restore_fidelity_class`, crash-loop screen anatomy, recovery
  action grammar):
  [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../ux/crash_loop_and_restore_fidelity_contract.md).
- Entry / restore truth audit (source of truth for
  `startup_state` tokens):
  [`/docs/ux/entry_restore_truth_audit.md`](../ux/entry_restore_truth_audit.md).
- Recovery-scenario card contract (source of truth for large-
  failure scenarios cited from the chooser):
  [`/docs/reliability/recovery_scenario_contract.md`](../reliability/recovery_scenario_contract.md).
- Autosave-journal and guided-replay contract (source of truth
  for the dirty-buffer journal and replay choices):
  [`/docs/reliability/autosave_journal_and_guided_replay_contract.md`](../reliability/autosave_journal_and_guided_replay_contract.md).
- Local-history restore-preview contract (source of truth for
  local-history snapshot refs cited from the chooser):
  [`/docs/reliability/local_history_restore_preview_contract.md`](../reliability/local_history_restore_preview_contract.md).
- Continuity-status card contract (source of truth for
  recovery-promise classes):
  [`/docs/reliability/continuity_status_card_contract.md`](../reliability/continuity_status_card_contract.md).
- Recovery-ladder packet contract (source of truth for safe-
  mode and the rung sequence):
  [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md).
- Support-bundle contract (source of truth for support-bundle
  refs):
  [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md).
- Governance record-state and policy-simulation models (source
  of truth for chronology rules and remembered-decision waiver
  TTL classes):
  [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md).
