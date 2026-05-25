# Hard-Lock Source-Fidelity Save And Save-Participant Lineage

This contract hard-locks how Aureline preserves source fidelity on save —
encoding, BOM, line-ending mode, and final-newline posture — and how it governs
the quality participants (formatter, organize-imports, linter/code-action,
scanner, AI apply) that run on the hot save path. It composes the existing
staged-save pipeline truth with one governed, export-safe lineage record per
save posture so editing, save, restore, trust, and lineage stay trustworthy
under crashes, migrations, external changes, and policy-bound workflows.

## Canonical machine sources

Do not clone status text from this doc — ingest the typed sources:

- Lineage projection and contract types:
  [`crates/aureline-editor/src/save_fidelity_lineage/`](../../../crates/aureline-editor/src/save_fidelity_lineage/)
- Save pipeline truth source (risk review, compare-before-write, source
  fidelity):
  [`crates/aureline-workspace/src/save/`](../../../crates/aureline-workspace/src/save/)
- Boundary schema:
  [`schemas/editor/save_fidelity_lineage.schema.json`](../../../schemas/editor/save_fidelity_lineage.schema.json)
- Headless emitter / CLI:
  [`crates/aureline-editor/src/bin/aureline_save_fidelity_lineage.rs`](../../../crates/aureline-editor/src/bin/aureline_save_fidelity_lineage.rs)
- Fixtures:
  [`fixtures/editor/m4/save_fidelity_lineage/`](../../../fixtures/editor/m4/save_fidelity_lineage/)
- Replay gate:
  [`crates/aureline-editor/tests/save_fidelity_lineage_replay.rs`](../../../crates/aureline-editor/tests/save_fidelity_lineage_replay.rs)
- Proof packet:
  [`artifacts/workspace/m4/hard-lock-source-fidelity-save-encoding-line-ending.md`](../../../artifacts/workspace/m4/hard-lock-source-fidelity-save-encoding-line-ending.md)

## Runtime shape

The save path is layered, and the lineage record never re-derives an outcome —
it ingests the live truth source verbatim:

- The **staged save coordinator**
  ([`crates/aureline-workspace/src/save/coordinator.rs`](../../../crates/aureline-workspace/src/save/coordinator.rs))
  stages a buffer snapshot, runs save participants, performs
  compare-before-write against the pinned save target, preserves the open-time
  representation via
  [`source_fidelity.rs`](../../../crates/aureline-workspace/src/save/source_fidelity.rs),
  selects an atomic or declared-degraded write lane, and emits a
  `SaveParticipantRiskReview`.
- The **lineage projection**
  ([`crates/aureline-editor/src/save_fidelity_lineage/`](../../../crates/aureline-editor/src/save_fidelity_lineage/))
  consumes that risk review and the open-time `SourceFidelityRecord` and projects
  one governed `SaveFidelityLineageRecord` per save posture. It adds the ordered
  participant contract, the fix-action classification, preview-plus-checkpoint
  enforcement, the recovery mapping, and the stable-qualification posture without
  changing any save decision.

The projection is read-only: it never mutates buffers, never re-runs
participants, and never widens authority.

## Source-fidelity preservation (hard-locked)

The open-time `SourceFidelityRecord` pins encoding, BOM state, newline mode,
final-newline posture, and executable intent. On save, the coordinator
re-applies that posture before encoding durable bytes, so:

- A CRLF file stays CRLF; an LF file stays LF; a `CR`-only file stays `CR`.
- A file opened with a UTF-8 / UTF-16 / UTF-32 BOM is re-encoded with the same
  BOM; a file without a BOM never grows one.
- The final-newline posture (present or absent) is preserved exactly.
- A participant whose output would change the representation posture is held for
  review before commit rather than silently normalizing the file.
- A file whose open-time encoding cannot round-trip (`unknown_binary_like`) is
  surfaced as `round_trip_provable = false`, and the lineage narrows below
  Stable instead of guessing an encoding.

The lineage record carries the open-time posture plus any `source_fidelity_adjustments`
detected on staged output, so support export and Help/About can show what was
preserved or what would have changed.

## Save-participant contract (explicit ordering)

Every participant is pinned to a canonical hot-save-path **stage** with a fixed
ordering rank:

| Stage                  | Rank | Participants mapped here              |
| ---------------------- | ---- | ------------------------------------ |
| `format`               | 0    | formatter / pretty-printer           |
| `organize_imports`     | 1    | import organizer                     |
| `lint`                 | 2    | auto-fixing linter slot              |
| `code_action_apply`    | 3    | code action, AI apply                |
| `scan`                 | 4    | read-only scanner                    |
| `validate_after_apply` | 5    | read-only validation                 |
| `unsequenced`          | 6    | unknown participant (requires review)|

The record reports `participant_order_canonical`. If a participant runs out of
canonical order, the record narrows below Stable with
`participant_ordering_violation` rather than committing silently. Whole-file
rewrite class, generated/read-only gating, and external-change race handling are
all carried from the risk review and reflected in the fix-action class and the
recovery mapping.

## Fix-action classification and checkpoint-plus-preview

Each participant's fix action is classified into one of:

- `safe_inline` — deterministic local text edit bounded to the staged
  file/range. No preview or checkpoint required.
- `preview_required` — whole-file rewrite, representation change, external-change
  conflict, or a single-file policy block.
- `multi_file` — touches more than the visible file, protected paths, or
  workspace-wide state.
- `generated_scope` — touches generated artifacts; routes through regeneration
  lineage.
- `semantically_broad` — heuristic, AI, or otherwise unproven output.

Any class other than `safe_inline` crosses the declared threshold: it requires
both preview and checkpoint before durable mutation. The coordinator already
holds such participants for review before they commit. The record proves this
with `preview_and_checkpoint_enforced`: if a threshold-crossing action ever
committed durable bytes without a checkpoint, the record sets the flag false and
narrows below Stable with `preview_or_checkpoint_not_enforced`.

## Recovery mapping

Each participant maps to an export-safe recovery action so the user can always
see how a committed or refused save reverses before any destructive cleanup:

- `exact_undo` — buffer undo reverses a safe-inline edit exactly, or staged work
  that never committed reverts in-buffer.
- `compensation` — multi-file / cross-target effects need a compensating action.
- `regeneration` — generated companions are restored by re-running the generator.
- `checkpoint_restore` — broad or whole-file mutations restore from a checkpoint.
- `none_no_write` — nothing durable was written, so no recovery is required.

## Stable qualification (auto-narrow)

A lineage record is Stable-qualified only when it can prove the contract on the
claimed posture. It auto-narrows below Stable with a named reason when:

- `participant_failed` — a participant failed before producing proven output;
- `participant_ordering_violation` — a participant ran outside canonical order;
- `preview_or_checkpoint_not_enforced` — a threshold-crossing action committed
  without preview-plus-checkpoint;
- `source_fidelity_unprovable` — the open-time encoding cannot round-trip.

Correct protective outcomes (review-required, blocked-no-write, rebase-required)
stay Stable: the contract working as designed is a pass, not a gap.

## Consuming surfaces

`save_fidelity_lineage_lines` is the single human-readable projection shared by
the editor save-status surface, the headless CLI emitter
(`aureline_save_fidelity_lineage`), Help/About, and support export, so no
surface clones status text from another. The record excludes raw source, raw
patches, and raw tool logs, so support export is safe by construction
(`raw_payload_excluded = true`).
