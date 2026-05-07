# Editor External Change Contract

This contract freezes how Aureline reports and resolves files that change
outside the active editor buffer. Its purpose is simple: no surface may
quietly guess whether to overwrite, reload, merge, or ignore external
state. The buffer, file tree, quick open, editor chrome, compare views,
restore provenance, and support/export packets all read the same event
record and state vocabulary.

Companion artifacts:

- [`/schemas/editor/external_change_event.schema.json`](../../schemas/editor/external_change_event.schema.json)
  defines the export-safe event record, review-choice record, surface
  projection, diff metadata, checkpoint metadata, and authoritative state
  outcome.
- [`/artifacts/fs/save_review_choice_matrix.yaml`](../../artifacts/fs/save_review_choice_matrix.yaml)
  is the machine-readable review-choice matrix (choice keys, forbidden reasons,
  diff/checkpoint requirements, and journal implications) shared across
  external-change and save-conflict review surfaces.
- [`/fixtures/editor/external_change_cases/`](../../fixtures/editor/external_change_cases/)
  contains worked cases for clean reloads, dirty save conflicts, external
  move/delete/rename, watcher loss, remote-root uncertainty, alias
  ambiguity, and compare-based recovery.

This contract composes with:

- [`/docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md)
  for canonical filesystem object identity, watcher-health taxonomy,
  compare-before-write, save-target tokens, and root capability flags.
- [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md)
  for presentation paths, logical workspace identity, canonical filesystem
  objects, alias sets, and support/export parity.
- [`/docs/ux/editor_document_state_contract.md`](./editor_document_state_contract.md)
  for dirty, compare, read-only, conflict, recovered snapshot, and stale
  document-state badges.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  for stale, snapshot, partial, and captured-versus-live disclosure.
- [`/docs/reliability/local_history_contract.md`](../reliability/local_history_contract.md)
  for local history, journal, checkpoint, and restore provenance language.

If this document conflicts with a source product document in `.t2/docs/`,
that source wins and this contract, schema, and fixtures must update in
the same change.

Source anchors read for this contract:

- `.t2/docs/Aureline_PRD.md:1108` - virtual file system and file
  watching requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1628` -
  watcher model, filesystem identity, canonical path, and save
  coordination.
- `.t2/docs/Aureline_Technical_Design_Document.md:1478` - watcher and
  filesystem identity design.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:2013` and
  `.t2/docs/Aureline_UX_Design_System_Style_Guide.md:1125` - degraded
  state copy, "Partially ready", "Degraded", and "Read-only degraded"
  rules.

## Boundary

This contract does not implement file watching, diffs, saves, merge
algorithms, remote agents, or local-history storage. It freezes the
cross-surface contract those implementations must emit and consume when
the authoritative object may have changed outside Aureline.

Frozen here:

- state classes for external changes, watcher uncertainty, alias-path
  changes, remote-root uncertainty, compare-required, reload-safe,
  manual-review-required, and save-blocked cases;
- the compare-or-reload decision floor;
- stale-buffer disclosure and editor chrome requirements;
- checkpoint, undo, restore-provenance, and support/export expectations;
- special rules for removable volumes, case-only rename, symlink or
  junction ambiguity, and disconnected remote roots; and
- the review-choice matrix for compare, overwrite, merge, cancel, reload,
  and retry.

Out of scope: full three-way merge implementation, provider-specific
diff algorithms, visual style, platform-specific watcher backends, and
remote-agent transport internals.

## Core Rules

1. **Compare-before-write is the correctness floor.** Watchers reduce
   latency, but correctness comes from comparing the pinned
   `save_target_token` generation with the strongest current generation
   token before writing.
2. **Silent overwrite is forbidden.** A save, autosave, formatter,
   extension, AI apply, or review apply MUST NOT overwrite newer external
   content unless an explicit, recorded review choice admitted that exact
   replacement after uncertainty was resolved.
3. **Uncertainty stays visible.** A surface MUST NOT downgrade
   `watcher_uncertain`, `remote_root_uncertain`, or alias ambiguity into a
   green/current state merely because no error is currently visible.
4. **The same event feeds every surface.** Buffer state, editor chrome,
   file tree rows, quick-open rows, compare views, restore provenance, and
   support/export packets project the same `external_change_event_record`.
5. **Canonical object identity wins over path strings.** External-change
   decisions key on `canonical_filesystem_object` first and
   `presentation_path` second. Alias paths are disclosed, not treated as
   independent mutable files.
6. **Review choices are reconstructable.** The event records which
   choices were offered, which were disabled and why, which choice was
   selected, the diff/checkpoint basis for that choice, and the
   authoritative state after it.

## Event Record Shape

Every event emitted under this contract carries these field groups:

| Field group | Required purpose |
| --- | --- |
| `identity` | Presentation path, logical workspace identity, canonical filesystem object, and alias set. |
| `buffer_snapshot` | Buffer ref, dirty state, pinned save target, local generation token, and stale-buffer disclosure. |
| `watcher_snapshot` | Watcher source, watcher health, uncertainty reason, and whether event continuity is proven. |
| `external_observation` | Observed external state: rewrite, delete, move, rename, alias change, remote unknown, or unavailable root. |
| `active_state_classes` | One or more state classes from this contract. |
| `state_resolution` | Whether auto-refresh is allowed, whether explicit review is required, and which authoritative state is currently known. |
| `surface_projections` | Buffer, editor chrome, file tree, quick-open, diff view, restore provenance, and support/export projection rows. |
| `diff_metadata` | Diff availability, basis tokens, content kind, hunk summary, and why compare is blocked when it is blocked. |
| `checkpoints` | Local-buffer checkpoint, external-version checkpoint, restore-provenance ref, and undo expectation. |
| `review` | Offered choices, disabled/forbidden choices, selected choice, actor, timestamp, and authoritative state after selection. |
| `support_export` | Packet refs, redaction policy, parity signature, and reconstruction guarantees. |

Raw file bytes, raw absolute paths, raw URLs, raw provider payloads,
secret values, and raw diff hunks do not cross this boundary. Records use
opaque refs, bounded labels, counts, and digest/checkpoint refs.

## State Classes

State classes are stable machine keys. Visible labels may localize, but
the keys below MUST match in schemas, fixtures, support packets, event
logs, accessibility labels, and CLI/headless output.

| State class | Meaning | Required effect |
| --- | --- | --- |
| `external_change_detected` | The VFS observed an external rewrite, delete, move, rename, alias change, or save precondition mismatch for the canonical object or its presentation path. | Surface the event in editor chrome and state projections. Decide reload, compare, retry, or review from the event record. |
| `watcher_uncertain` | The watcher is warming, degraded, fallback polling, unavailable, or has a known event gap. Aureline cannot prove it saw every external change. | Keep uncertainty visible. Block actions that require proven current state until retry/compare/revalidation succeeds. |
| `alias_path_change` | The same canonical object is now reachable through a different presentation path, case variant, normalization variant, symlink, junction, hardlink sibling, or remote alias. | Converge dirty authority by canonical object and disclose the alias relationship. Do not duplicate mutable buffers by path string. |
| `remote_root_uncertain` | A remote or container root cannot provide current revision, object identity, watcher continuity, or write precondition state. | Keep local editing available where possible but block save/autosave writes that require current remote authority. Offer retry/reconnect and export-safe compare when available. |
| `compare_required` | A decision could lose local edits, external edits, path identity, restore provenance, or generated/source relation unless the user reviews a diff or equivalent comparison. | Open or offer compare before overwrite, merge, reload-with-discard, or restore. |
| `reload_safe` | The buffer has no unsaved local edits, the canonical object identity is stable, watcher/root state is healthy enough, and the external state can replace the buffer without losing local authored content. | Auto-refresh MAY run, but the event is still recorded and visible. Reload mints a checkpoint/undo record when the surface supports undo. |
| `manual_review_required` | The event cannot be resolved automatically because it involves dirty local state, delete/move/rename, alias ambiguity, watcher uncertainty, remote uncertainty, policy, binary/unmergeable content, or missing diff basis. | Require an explicit review choice. Do not default to overwrite or reload. |
| `save_blocked` | Save/autosave/apply attempted to write but the save target is stale, uncertain, read-only, policy blocked, or missing a required review. | Block the write. Surface the blocked save in editor chrome, status, diff/review surfaces, and support/export packets. |

Composite states are expected. A dirty save conflict usually carries
`external_change_detected`, `compare_required`,
`manual_review_required`, and `save_blocked`. A clean local rewrite can
carry `external_change_detected` and `reload_safe`.

## Surface Requirements

Every surface reads the event record and renders only the state classes
above.

| Surface | Required projection |
| --- | --- |
| Buffer | Dirty/clean state, stale generation disclosure, save-target token ref, and whether save/autosave is blocked. |
| Editor chrome | Header/context/status labels for external change, stale buffer, compare-required, reload-safe, manual-review-required, and save-blocked states. A colored dot or icon-only badge is insufficient. |
| File tree | Row badge for changed, moved, deleted, alias-path-changed, watcher-uncertain, or remote-uncertain state. It must not show the row as plain current while the editor is stale. |
| Quick open | Result row badge and disabled/secondary action reason when opening would land on a stale, deleted, moved, alias-ambiguous, watcher-uncertain, or remote-uncertain object. |
| Diff view | Named source roles, basis tokens, diff availability, local/external checkpoint refs, and disabled choices with reasons. |
| Restore provenance | Which checkpoint or external version can restore local state, external state, or merged state after a decision. |
| Support/export | Full event record or stable event ref, offered choices, selected choice, authoritative state after selection, redaction policy, and parity signature. |

Forbidden surface behavior:

- showing "current", "ready", or a green-only affordance while
  `watcher_uncertain`, `remote_root_uncertain`, or unresolved
  `compare_required` is active;
- placing save-conflict detail only in a tooltip;
- showing file tree and quick-open rows as current when the active buffer
  is stale or save-blocked;
- offering autosave overwrite as a background action; and
- exporting a screenshot or text summary without the event record or
  stable event ref needed to reconstruct the decision.

## Compare-Or-Reload Policy

Aureline may auto-refresh only when all of these are true:

1. `buffer_snapshot.dirty_state = clean`.
2. `canonical_filesystem_object` continuity is `same_canonical_object`.
3. Watcher health is `healthy` or the event was proven by
   compare-before-write with current generation tokens.
4. No `remote_root_uncertain`, `watcher_uncertain`, or unresolved
   `alias_path_change` state is active.
5. The external observation is a rewrite or metadata change, not delete,
   move to an unknown object, unsupported case-only rename, symlink or
   junction target change, or root disconnect.
6. The event records an external version ref or generation token and an
   undo/checkpoint expectation.

When those conditions hold, the event may include `reload_safe` and the
surface MAY reload the buffer automatically. The reload still records a
surface-visible event, updates stale-buffer disclosure, and preserves an
undo or local-history route where the buffer system supports it.

Explicit user review is required when any of these are true:

- the buffer is dirty, recovered dirty, generated dirty, or has pending
  save participants;
- a save/autosave/apply hit a generation-token mismatch;
- watcher continuity is degraded, unavailable, warming, or fallback
  polling without a fresh compare;
- a remote root is disconnected or cannot provide current revision or
  conditional-write authority;
- the target is on a removable volume whose identity token is unavailable
  or changed after reattach;
- the event is delete, move, rename, case-only rename on a root without
  explicit support, symlink/junction target change, or alias ambiguity;
- the diff basis is missing, binary-only, blocked by policy, or refers to
  a different canonical object; or
- overwrite, merge, reload-with-discard, or restore would destroy local
  or external authored content without a recorded checkpoint.

## Stale-Buffer Disclosure

An editor buffer is stale when its visible bytes, dirty authority, or
save-target token no longer describe the latest known canonical object.
While stale:

- the editor header or context row says the buffer is stale or externally
  changed;
- save/autosave/apply routes through the blocked-save or review path;
- the stale generation token and current observed token are stored as
  opaque refs;
- file tree and quick-open rows disclose the same state; and
- support/export packets preserve the stale disclosure even if the user
  later reloads, compares, or cancels.

`reload_safe` removes the stale disclosure only after the buffer adopts
the observed external generation and the event records the new
authoritative state.

## Review-Choice Matrix

Every review session uses the same choice keys. A disabled choice remains
in the event with a forbidden reason so support can reconstruct why the
user did not see it as enabled.

| Choice | May be offered when | Forbidden when | Diff metadata requirement | Checkpoint / undo implication | Resulting authoritative state |
| --- | --- | --- | --- | --- | --- |
| `compare` | There is any external change, dirty buffer, unclear reload basis, move/delete/rename, or policy that requires review. | Only when no comparable basis exists and no metadata-only comparison can be produced; then the event records compare as disabled. | Requires basis refs for local buffer and observed external state, or a reason such as `binary_only` / `blocked_uncertain`. | No mutation. May mint a review checkpoint so later merge/overwrite/reload choices are tied to the viewed basis. | `no_authoritative_state_yet` until a follow-up choice resolves it. |
| `overwrite` | The user has reviewed the external state, uncertainty is resolved, save authority is current, and an external-version checkpoint exists or policy admits no checkpoint with explicit disclosure. | Dirty external state newer than the buffer without compare, any watcher/remote/alias uncertainty, missing save target, read-only/policy-blocked root, removable identity uncertainty, or autosave/background execution. | Requires diff metadata or a recorded compare-disabled reason that the user explicitly admitted. | Must checkpoint the external version before write and record undo as restore-via-checkpoint where supported. | `authoritative_buffer_written` after conditional write succeeds. |
| `merge` | A merge strategy is available for the content kind and both local and external bases are known. | Binary/unmergeable content, generated content that must regenerate first, unknown canonical object, missing external version, watcher/remote/alias uncertainty not resolved, or unsupported merge tool. | Requires local, base, and external refs plus hunk/semantic summary. | Must checkpoint local and external versions before applying result. Undo returns to pre-merge buffer or restores written file through checkpoint if already saved. | `authoritative_merged_result` after validation/write succeeds. |
| `cancel` | Always, including uncertainty and blocked-save states. | Never forbidden. | None. | No mutation. Existing checkpoints remain available. Buffer stays stale if it was stale. | Prior authoritative state remains, often `no_authoritative_state_yet` or `last_known_good_stale`. |
| `reload` | The event carries `reload_safe`, or the user explicitly discards local edits after compare and local state is checkpointed. | Dirty buffer without explicit discard, unresolved watcher/remote/alias uncertainty, delete/move to unknown object, missing external generation, binary compare blocked with local edits, or policy requiring review. | For clean `reload_safe`, diff metadata may be summary-only. For dirty reload-with-discard, compare metadata is required. | Clean reload records buffer undo where available. Dirty reload-with-discard checkpoints the local buffer before replacing it. | `authoritative_external_loaded`. |
| `retry` | Watcher, remote, removable volume, policy, or transient identity state may become provable again. | When retry would mutate data, mask a known conflict, or repeatedly relabel unresolved uncertainty as current. | None initially; retry outcome mints a new event or updates with revalidation refs. | No mutation. May add a revalidation attempt ref. | `authoritative_remote_revalidated`, `authoritative_external_loaded`, or still `manual_review_required` depending on outcome. |

`save_as` is an escape path owned by the save-target-token contract. It
may be offered alongside these choices, but it does not resolve the
original external-change event unless the event records the original
buffer as exported to a distinct canonical object.

## Checkpoint And Undo Expectations

1. **Before destructive choices.** Overwrite, merge-then-save,
   reload-with-discard, and restore-to-external MUST create or cite the
   checkpoint that makes the choice recoverable. If no checkpoint is
   possible, the choice requires explicit disclosure and cannot be the
   default.
2. **Clean reload.** A clean reload MAY use editor undo rather than a
   durable file checkpoint, but the event still records the external
   generation adopted and the pre-reload buffer generation.
3. **Compare.** Opening compare does not mutate the file. It records the
   comparison basis so a later choice cannot pretend it was made against
   a different external version.
4. **Cancel.** Cancel preserves the stale state and pending checkpoints.
   It does not silently mark the buffer current.
5. **Restore provenance.** Support/export and restore surfaces must be
   able to answer which choice created each checkpoint and whether that
   checkpoint restores local buffer bytes, external bytes, or a merged
   result.

## Special Root And Path Rules

### Removable Volumes

When a removable volume disappears and reappears, generation tokens,
device identifiers, and path strings may no longer prove object
continuity. If the canonical object cannot be revalidated, the event
carries `watcher_uncertain` and `manual_review_required`; save is blocked
until compare/retry proves the target or the user chooses a distinct save
target. The file tree and quick open must label the row as uncertain, not
current.

### Case-Only Rename

On case-insensitive roots, a case-only rename is an alias-path change
until the root capability envelope confirms `supports_case_only_rename`
and the canonical object continuity is proven. A clean buffer may update
its presentation path after confirmation. A dirty buffer requires review
because save could recreate the old spelling or overwrite the renamed
object.

### Symlink Or Junction Ambiguity

If a symlink or junction target changes, Aureline treats the event as an
alias-path change plus manual review unless the canonical filesystem
object is proven unchanged. A save through a symlink/junction path is
blocked when `symlink_escape_policy = block` or when the target escapes
the trusted root without review.

### Disconnected Remote Roots

When a remote root disconnects, the buffer remains editable locally only
as local buffer authority. It cannot claim current remote authority.
Save/autosave against the remote target is blocked until a conditional
remote write token, provider object revision, or explicit offline export
path is available. Retry/reconnect is safe; overwrite is forbidden while
the remote authoritative state is unknown.

## Save-Conflict Surfacing

A save conflict is not a generic failed save. It must surface in:

- **Editor chrome:** header/context/status labels for stale buffer,
  external change, compare-required, and save-blocked state.
- **Diff views:** local buffer, external version, optional base, result
  role labels; diff availability; basis tokens; enabled and disabled
  review choices.
- **Restore provenance:** checkpoint refs for local, external, and merged
  states, plus undo expectation for the selected choice.
- **Support/export packets:** the event record, choice matrix, selected
  choice, actor, timestamp, authoritative state after choice, redaction
  policy, and parity signature.

Any surface that says only "save failed" while hiding external-change
state, stale-buffer basis, or review choices is non-conforming.

## Schema And Fixture Requirements

Records using
[`/schemas/editor/external_change_event.schema.json`](../../schemas/editor/external_change_event.schema.json)
must satisfy these invariants:

1. `active_state_classes` uses only the state classes in this contract.
2. `silent_overwrite_forbidden` and
   `downgrade_uncertainty_to_green_forbidden` are always true.
3. Buffer, editor chrome, file tree, quick open, and support/export
   projections cite the same event id and state classes.
4. If `save_blocked` is active, at least one review choice explains the
   safe next action.
5. If `reload_safe` is active, `manual_review_required` and
   `save_blocked` are absent.
6. If uncertainty is active, overwrite and dirty reload are disabled
   until retry/compare produces a new authoritative basis.
7. Every fixture records offered choices, selected choice (or explicit
   null), and authoritative state after selection.

## Changing This Contract

Adding a new state class, review choice, forbidden reason,
authoritative-state class, or surface class is additive-minor only when
the schema, this document, and at least one fixture land together.
Repurposing an existing key is breaking and requires a new architecture
decision.
