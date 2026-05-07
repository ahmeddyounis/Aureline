# Conflict-aware save sequence (source fidelity)

This artifact freezes the end-to-end control flow for saving an editor buffer
to durable storage **without losing source fidelity**, **without silently
overwriting external changes**, and **without hiding degraded write guarantees**.

It is intentionally written so editor chrome, diff/compare views, restore
provenance, and support/export packets can all point at the same vocabulary and
the same ordering of decisions.

## Sources and companion material

Normative sources (these win on disagreement):

- `.t2/docs/Aureline_PRD.md` (source fidelity + atomic save pipeline)
- `.t2/docs/Aureline_Technical_Architecture_Document.md` (conflict-aware save sequence)
- `.t2/docs/Aureline_Technical_Design_Document.md` (save pipeline staging and fidelity rules)

Contracts and schemas used by this sequence:

- `docs/io/save_target_token_and_write_guarantee_contract.md`
- `docs/files/save_fallback_and_watch_fidelity_contract.md`
- `docs/ux/editor_external_change_contract.md`
- `schemas/io/save_target_token.schema.json`
- `schemas/files/save_fallback_detail.schema.json`
- `schemas/editor/external_change_event.schema.json`
- `schemas/workspace/mutation_journal.schema.json`

Machine-readable companion:

- `artifacts/fs/save_review_choice_matrix.yaml`

Seeded corpus:

- `fixtures/fs/conflict_save_cases/README.md`
- `fixtures/fs/conflict_save_cases/*.yaml`

Cross-sequence failure + reconstruction index:

- `artifacts/architecture/critical_sequence_failure_matrix.yaml`
- `artifacts/support/critical_sequence_trace_reconstruction.md`

## Non-negotiable invariants

1. **Compare-before-write is the correctness floor.** Watchers reduce latency;
   they do not replace pre-write compare.
2. **Silent overwrite is forbidden.** No save, autosave, participant, extension,
   or apply path may overwrite newer external bytes without an explicit,
   recorded review choice.
3. **Uncertainty stays visible.** Watcher uncertainty, remote-root uncertainty,
   and alias ambiguity may not be relabeled as “current” merely because no
   error is currently visible.
4. **Preferred vs degraded write paths stay explicit.** A successful save may
   not mask that it ran on a degraded lane (in-place write, uncertain target,
   limited metadata preservation, remote fallback, etc).
5. **Destructive choices must be recoverable.** Overwrite, merge-then-write,
   and reload-with-discard require checkpoint/undo posture that can restore
   both local intent and the replaced external version.

## End-to-end sequence

The steps below describe one *attempt* to save a buffer. “Attempt” includes
blocked saves and review sessions; the sequence is about attribution and safety
as much as it is about writing bytes.

### 0) Preconditions (pinned identity and capability truth)

Before the user can trigger a durable write:

- The editor has a pinned `save_target_token_packet` (`schemas/io/save_target_token.schema.json`)
  describing the presentation path the user opened, the canonical object that
  would be mutated, the strongest available generation token, and the declared
  write mode and guarantee class.
- The active root’s watch-fidelity state and any lagging downstream truths are
  available for disclosure (`docs/files/save_fallback_and_watch_fidelity_contract.md`).

### 1) Stage a buffer snapshot (do not mutate the live buffer)

On save:

1. Capture an immutable buffer snapshot (bytes + selection-neutral metadata).
2. Record the snapshot’s `dirty_state` and its pinned `save_target_token_ref`.
3. Record the snapshot’s local generation token ref (the “basis” token for
   conflict compare and attribution).

### 2) Run save participants on staged content (formatters, code actions, apply)

Participants run only on the staged snapshot:

1. Run eligible participants on the staged bytes.
2. Any participant that cannot complete **must abort** the attempt; it must not
   “fall through” to a weaker write lane.
3. If participants require disk basis, they must use the same compare-before-
   write ordering as the save pipeline; they may not assume on-disk stability.

### 3) Fidelity validation (encoding and newline guarantees)

Validate that the staged bytes preserve:

- encoding and BOM state
- dominant line-ending mode
- final-newline semantics
- other required intent the root supports (e.g., executable bit preservation)

If fidelity cannot be preserved, save must route through an explicit review
surface; it must not “best-effort” rewrite while reporting success.

### 4) Re-resolve the save target (wrong-target prevention)

Immediately before any durable write:

1. Re-resolve the canonical filesystem object.
2. If canonical object identity differs from the token, **block** (wrong-target
   prevention) and surface alias/details. Do not write.

This step is required even when watchers report “healthy”.

### 5) Compare-before-write (match, mismatch, or uncertainty)

Read the strongest available “now” generation token and compare it to the
token’s pinned basis:

- **Match:** proceed to the declared write lane (preferred path if available).
- **Mismatch:** emit an `external_change_event_record` and block the write.
- **Uncertain:** the attempt must not claim a strong guarantee; it must either:
  - block (capability uncertain / wrong-target risk), or
  - proceed only on an explicitly degraded lane that was disclosed *before*
    bytes are written (and only if policy admits that lane).

### 6) Publish the save-fallback detail row (preferred vs degraded is explicit)

Before writing any durable bytes, the save attempt emits one
`save_fallback_detail_row` (`schemas/files/save_fallback_detail.schema.json`)
alongside the save-target token packet. This row is the “single place” a
surface can answer:

- whether the attempt is on the preferred atomic lane or a degraded lane,
- which metadata preservation guarantees apply (or are unknown),
- whether compare-before-write is being treated as a hard gate,
- what crash journal / checkpoint posture applies, and
- what typed first recovery surface is offered if the attempt cannot proceed.

Rules:

- If the guarantee is weaker than `atomic_replace_preferred`, the degraded lane
  must be disclosed before bytes are written (`degraded_guarantee_disclosed=true`).
- A successful save on a degraded lane must not be presented as equivalent to
  a preferred atomic save; the detail row remains citable by support/export.

### 7) Preferred local write lane (temp-write + fsync + atomic rename)

When the save-target token declares `write_mode = atomic_replace` and
compare-before-write matches:

1. **Same-directory temp write.** Create a temp file in the same directory as
   the canonical target (same volume requirement).
2. **Write bytes.** Write the staged bytes in full.
3. **Durability barrier.** `fsync` (or platform equivalent) the temp file so the
   new contents are durable before rename.
4. **Atomic replace.** Atomically rename/replace the canonical target with the
   temp file.
5. **Directory durability (where applicable).** `fsync` the parent directory (or
   platform equivalent) so the rename is durable.
6. **Authoritative update.** Refresh the generation token, clear stale
   disclosure only after commit, and emit structured change events.

If any step fails:

- The attempt must not silently switch to in-place write.
- The save-fallback detail row and conflict event record must remain explicit
  about which step failed and which recovery surface is available.

### 8) Degraded write lanes (explicit, attributable, and review-gated)

Degraded lanes are allowed only when they are **explicit** and **recoverable**.
They exist because some roots cannot safely promise atomic replace or strong
identity tokens.

#### 8.1 In-place write with review

When a root cannot safely do atomic replace but can do an in-place write lane:

- The save-guarantee class becomes `in_place_write_with_review`.
- The user must see the degraded write guarantee before bytes are written.
- Destructive actions require checkpoints:
  - checkpoint the external version before overwrite,
  - checkpoint the local buffer before discard or merge application.

#### 8.2 Remote conditional write

When the root provides a revision/ETag/precondition token:

- The save-guarantee class becomes `remote_conditional_write`.
- The write must be conditional on the pinned revision token.
- A precondition failure is a conflict; it must open review rather than falling
  back to blind overwrite.

#### 8.3 Capability-uncertain (blocked)

When identity/generation/capability truth cannot be proven:

- The save-guarantee class becomes `capability_uncertain`.
- Durable write is blocked.
- Safe next actions are `retry` (revalidate), `compare` (if a basis exists),
  and `save_as` / export-to-new-target (preserve local intent without claiming
  the original target is safe).

### 9) Conflict event record and review flow (no silent overwrite)

When compare-before-write yields **mismatch** or **uncertainty that blocks a
safe write**, the system emits one `external_change_event_record`
(`schemas/editor/external_change_event.schema.json`) and routes through the
review-choice matrix.

The event record must carry:

- the identity layers (presentation path, logical workspace identity, canonical
  filesystem object, alias set),
- the buffer snapshot (dirty state, save-target token ref, basis token ref),
- watcher/remote uncertainty disclosure,
- diff metadata availability (or explicit unavailability reasons),
- checkpoint refs and undo expectation, and
- the offered choices (enabled/disabled with forbidden reasons).

Save remains blocked while `save_blocked` is active; no surface may “pretend
success” by hiding the blocked-save state behind a green icon.

### 10) Resolution choices (what each choice does to bytes and journals)

The choice vocabulary is frozen in `artifacts/fs/save_review_choice_matrix.yaml`
and must match `docs/ux/editor_external_change_contract.md`.

- **`compare`**
  - Opens a compare view without mutating durable bytes.
  - Pins the reviewed basis so later choices remain attributable.
  - May mint a review checkpoint; selecting compare does not clear stale state.
- **`overwrite`**
  - Requires that the user reviewed external state (or explicitly admitted a
    compare-disabled disclosure) and that uncertainty is resolved.
  - Must checkpoint the external version before write.
  - Re-runs compare-before-write against the reviewed basis immediately before
    committing.
  - Writes via the declared lane (preferred atomic replace when available).
- **`merge`**
  - Requires a merge strategy and known local/base/external refs.
  - Must checkpoint both local and external versions before applying result.
  - Validates fidelity on the merged staged bytes before durable write.
- **`reload`**
  - Clean, reload-safe: may be auto-selected; records undo posture and adopted
    external generation.
  - Reload-with-discard: requires explicit discard + local checkpoint first;
    cannot be offered as a default in dirty cases.
- **`retry`**
  - Never mutates durable bytes.
  - Revalidates identity/generation/remote authority and either resolves the
    uncertainty or produces a new event with updated evidence.
- **`cancel`**
  - Never mutates durable bytes.
  - Preserves stale/uncertain disclosure; it must not relabel the buffer as current.
- **`save_as`**
  - Exports the buffer to a distinct canonical object.
  - Does not resolve the original conflict unless explicitly recorded as such.

### 11) Authoritative-state update (only after evidence-backed commit)

The buffer and surfaces may clear “stale” disclosure only after one of these
becomes true:

- durable write committed on the reviewed basis, or
- the buffer explicitly adopted the observed external generation (reload), or
- alias convergence was proven and recorded as authoritative.

Any attempt that remained blocked must continue to surface:

- the stale buffer disclosure,
- the observed-vs-basis token refs, and
- the disabled choices with forbidden reasons.

## Journal + checkpoint packet (attribution and recovery)

This sequence requires enough linkage that support/export and local history can
answer “what happened?” without guessing.

Minimum linkage expectations:

- The save attempt cites the `save_target_token_packet.packet_id`.
- Any degraded or blocked attempt cites the `save_fallback_detail_row.record_id`
  describing the lane and recovery surface.
- Any conflict/uncertainty emits an `external_change_event_record.event_id`,
  including offered choices and the selected choice (or explicit null).
- Any destructive resolution (overwrite/merge/reload-with-discard/save-as)
  cites the checkpoint refs needed to restore:
  - local buffer intent,
  - replaced external version, and/or
  - the reviewed basis.
- The mutation journal entry (`schemas/workspace/mutation_journal.schema.json`)
  links the above refs via `checkpoint_refs[]`, `save_manifest_ref` (when a write
  was attempted), and `side_effect_summary`.

The corpus cases under `fixtures/fs/conflict_save_cases/` spell out the minimum
checkpoint and journal expectations for representative conflict scenarios.
