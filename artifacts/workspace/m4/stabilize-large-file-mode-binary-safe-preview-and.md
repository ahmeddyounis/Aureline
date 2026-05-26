# Stabilize Large-File Mode, Binary-Safe Preview, And Restricted-Write Posture — proof packet

Reviewer-facing proof packet for the stabilized large-file lane: large-file mode
activation, binary-safe preview, and the restricted-write posture composed into
one governed, export-safe record per posture. This packet is the stable-line
anchor for this lane: dashboards, docs, Help/About surfaces, and support exports
should ingest the typed sources below rather than cloning this packet's text.

## Canonical machine sources

- Posture projection and contract types:
  [`/crates/aureline-editor/src/large_file_posture/`](../../../crates/aureline-editor/src/large_file_posture/)
- At-open large-file classifier and constrained reader:
  [`/crates/aureline-editor/src/large_file/`](../../../crates/aureline-editor/src/large_file/)
- Limited-mode capability / write posture record:
  [`/crates/aureline-editor/src/large_file_mode/`](../../../crates/aureline-editor/src/large_file_mode/)
- Schema:
  [`/schemas/editor/large_file_posture.schema.json`](../../../schemas/editor/large_file_posture.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-editor/src/bin/aureline_large_file_posture.rs`](../../../crates/aureline-editor/src/bin/aureline_large_file_posture.rs)
- Fixtures:
  [`/fixtures/editor/m4/large_file_posture/`](../../../fixtures/editor/m4/large_file_posture/)
- Replay gate:
  [`/crates/aureline-editor/tests/large_file_posture_replay.rs`](../../../crates/aureline-editor/tests/large_file_posture_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/stabilize-large-file-mode-binary-safe-preview-and.md`](../../../docs/workspace/m4/stabilize-large-file-mode-binary-safe-preview-and.md)
- Typed consumer: `aureline_editor::project_large_file_posture`

## What this packet proves

1. **Source fidelity survives the constrained preview.** The constrained viewer
   reads raw bytes in bounded pages and never decodes-then-reencodes the whole
   file; the record reports `byte_faithful_read` (whole-file load into the normal
   buffer is blocked) and `bom_preserved` (a raw read preserves any detected
   BOM). Binary-like content is given a binary-safe preview rather than a lossy
   text render: `binary_safe_preview_selected`. A record that admits whole-file
   load narrows below Stable with `source_read_not_byte_faithful`; binary content
   served the paged raw-text preview narrows with `preview_not_binary_safe`.
   Worked examples:
   [`whole_file_load_narrowed.json`](../../../fixtures/editor/m4/large_file_posture/whole_file_load_narrowed.json),
   [`binary_without_safe_preview_narrowed.json`](../../../fixtures/editor/m4/large_file_posture/binary_without_safe_preview_narrowed.json),
   and the protective
   [`binary_safe_preview_stable.json`](../../../fixtures/editor/m4/large_file_posture/binary_safe_preview_stable.json).

2. **Writes land on the canonical target.** The record carries the VFS
   `canonical_uri` and reports `canonical_target_resolved`. An unresolved target
   narrows below Stable with `canonical_target_unresolved`.

3. **Restricted writes never silently re-run.** Whole-file save participants,
   whole-file format-on-save, and whole-file AI apply are blocked
   (`whole_file_participants_blocked`); only reviewed range-only writes are
   admitted (`range_only_reviewed_writes`); and the escalation route is explicit
   and disclosed (`override_disclosed`). A whole-file participant admitted in
   limited mode narrows with `whole_file_write_not_restricted`; an undisclosed
   escalation narrows with `override_route_not_disclosed`. Worked example:
   [`whole_file_write_narrowed.json`](../../../fixtures/editor/m4/large_file_posture/whole_file_write_narrowed.json).

4. **Inspection precedes destructive cleanup.** A destructive action
   (escalation to the normal buffer, or a constrained write) is always reachable,
   so the record requires the compare and checkpoint inspection hooks to be
   available before it. A missing compare or checkpoint path narrows with
   `destructive_action_no_checkpoint`. The full hook set is compare, checkpoint,
   export, and repair. Worked example:
   [`missing_checkpoint_narrowed.json`](../../../fixtures/editor/m4/large_file_posture/missing_checkpoint_narrowed.json).

5. **Lineage and export stay honest.** Every record sets
   `raw_payload_excluded = true` and embeds only the evaluated capability rows it
   reasoned over; it carries no raw source bytes. A source limited-mode record
   that is not export-safe narrows with `posture_export_unsafe`.

6. **The record is replay-gated.** The replay gate re-projects each fixture and
   asserts it equals the checked-in `expected`, so the projection cannot drift
   without failing CI.

## Fixture corpus

| Fixture                                | Trigger          | Qualification           | Proves                                  |
| -------------------------------------- | ---------------- | ----------------------- | --------------------------------------- |
| `oversized_text_stable`                | `size_threshold` | `stable`                | All pillars proven (text)               |
| `binary_safe_preview_stable`           | `classification` | `stable`                | Binary-safe preview = protective        |
| `binary_without_safe_preview_narrowed` | `classification` | `narrowed_below_stable` | Binary served as raw text               |
| `whole_file_load_narrowed`             | `size_threshold` | `narrowed_below_stable` | Read not byte-faithful                  |
| `whole_file_write_narrowed`            | `size_threshold` | `narrowed_below_stable` | Whole-file write not restricted         |
| `missing_checkpoint_narrowed`          | `size_threshold` | `narrowed_below_stable` | Destructive action with no checkpoint   |

## How to verify

```sh
# Unit + replay gate for the large-file posture projection.
cargo test -p aureline-editor --lib large_file_posture
cargo test -p aureline-editor --test large_file_posture_replay

# Truth sources (large-file classifier + constrained viewer, limited-mode record).
cargo test -p aureline-editor --lib large_file
cargo test -p aureline-editor --lib large_file_mode

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-editor --bin aureline_large_file_posture -- --lines \
  fixtures/editor/m4/large_file_posture/binary_without_safe_preview_narrowed.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and replay gate
above. The posture record self-describes its stable qualification: surfaces that
cannot prove the contract carry `stable_qualification.level =
narrowed_below_stable` with a named reason, so they never inherit an adjacent
green row. No public scope is widened from this row.
