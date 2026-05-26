# Stabilize Large-File Mode, Binary-Safe Preview, And Restricted-Write Posture

This contract stabilizes how Aureline keeps oversized, binary, and otherwise
hostile files trustworthy. It composes two existing truth sources — the at-open
large-file classifier and the limited-mode capability record — into one
governed, export-safe posture record per posture. The record never re-derives an
outcome: it ingests each live source verbatim and adds the proof that the
constrained preview is byte-faithful and binary-safe, that writes land on the
canonical target, that whole-file writes never silently re-run, and that the
user can inspect before any destructive cleanup.

## Canonical machine sources

Do not clone status text from this doc — ingest the typed sources:

- Posture projection and contract types:
  [`crates/aureline-editor/src/large_file_posture/`](../../../crates/aureline-editor/src/large_file_posture/)
- At-open large-file classifier and constrained reader:
  [`crates/aureline-editor/src/large_file/`](../../../crates/aureline-editor/src/large_file/)
- Limited-mode capability / write posture record:
  [`crates/aureline-editor/src/large_file_mode/`](../../../crates/aureline-editor/src/large_file_mode/)
- Boundary schema:
  [`schemas/editor/large_file_posture.schema.json`](../../../schemas/editor/large_file_posture.schema.json)
- Headless emitter / CLI:
  [`crates/aureline-editor/src/bin/aureline_large_file_posture.rs`](../../../crates/aureline-editor/src/bin/aureline_large_file_posture.rs)
- Fixtures:
  [`fixtures/editor/m4/large_file_posture/`](../../../fixtures/editor/m4/large_file_posture/)
- Replay gate:
  [`crates/aureline-editor/tests/large_file_posture_replay.rs`](../../../crates/aureline-editor/tests/large_file_posture_replay.rs)
- Proof packet:
  [`artifacts/workspace/m4/stabilize-large-file-mode-binary-safe-preview-and.md`](../../../artifacts/workspace/m4/stabilize-large-file-mode-binary-safe-preview-and.md)

## Runtime shape

The large-file path is layered, and the posture record never re-derives an
outcome — it ingests the live truth sources verbatim:

- The **at-open classifier**
  ([`crates/aureline-editor/src/large_file/classification.rs`](../../../crates/aureline-editor/src/large_file/classification.rs))
  runs once per open and decides the activation mode, the trigger, and the
  human-readable reason from a bounded sniff (binary signal, BOM, null bytes,
  max line length, non-printable ratio). It is observed as a serializable
  `LargeFileClassificationObservation`.
- The **constrained viewer**
  ([`crates/aureline-editor/src/large_file/`](../../../crates/aureline-editor/src/large_file/))
  serves a bounded-memory paged reader that reads raw bytes; it never loads the
  whole file into the normal piece-tree buffer unless the user explicitly opts
  in through the override route.
- The **limited-mode record**
  ([`crates/aureline-editor/src/large_file_mode/`](../../../crates/aureline-editor/src/large_file_mode/))
  carries the canonical write target, the safe-preview class, the edit/write
  policy, the explicit override route, and the reduced-capability table.
- The **posture projection**
  ([`crates/aureline-editor/src/large_file_posture/`](../../../crates/aureline-editor/src/large_file_posture/))
  consumes the classification observation and the limited-mode record and
  projects one governed `LargeFilePostureRecord` per posture. It adds the
  source-fidelity proof, the canonical-path-truth proof, the restricted-write
  proof, the inspection-hook contract, and the stable-qualification posture
  without changing any classification or limited-mode decision.

The projection is read-only: it never re-runs a participant, mutates a buffer,
opens a file, or widens authority.

## Source fidelity (byte-faithful, binary-safe preview)

The constrained viewer reads raw bytes in bounded pages. The record reports:

- `byte_faithful_read` — whole-file load into the normal buffer is blocked, so
  the read path never decodes-then-reencodes the whole file;
- `whole_file_load_blocked` — the `whole_file_load_into_ram` capability is denied;
- `binary_safe_preview_selected` — binary-like content (per the sniff) is given a
  `binary_safe_preview`, not a lossy paged raw-text render;
- `bom_preserved` — a raw read preserves any detected BOM rather than stripping it;
- `source_fidelity_proven` — the overall verdict.

A record that admits whole-file load narrows below Stable with
`source_read_not_byte_faithful`. Binary content served the paged raw-text
preview narrows with `preview_not_binary_safe`. A binary file correctly served a
binary-safe preview stays Stable — that is the contract protecting the user, not
a gap.

## Canonical-path truth (no wrong-target write)

The record carries the VFS `canonical_uri` and reports
`canonical_target_resolved`. A constrained or reviewed range write targets the
canonical object; an unresolved target narrows below Stable with
`canonical_target_unresolved`.

## Restricted writes are no-rerun

The record proves that no whole-file transform silently re-runs over a large or
binary file. It reports:

- `whole_file_participants_blocked` — whole-file save participants, whole-file
  format-on-save, and whole-file AI apply are all denied;
- `range_only_reviewed_writes` — reviewed range-only writes may still be admitted;
- `override_disclosed` — the escalation / override route carries a disclosure;
- `restricted_write_proven` — the overall verdict.

A whole-file participant admitted in limited mode narrows with
`whole_file_write_not_restricted`. An escalation route with no disclosure narrows
with `override_route_not_disclosed`.

## Inspection precedes destructive cleanup

A destructive action — escalating to the normal buffer (which may load the whole
file) or a constrained write — is always reachable. Before it, the record
requires the compare and checkpoint inspection hooks to be available. The full
hook set is:

- `compare` — compare the constrained preview against the current on-disk bytes;
- `checkpoint` — record a local-history recovery checkpoint before escalating or
  writing, so live state survives;
- `export` — export this posture record for support without raw bytes;
- `repair` — re-run classification and re-open without clearing local state.

A missing compare or checkpoint path narrows below Stable with
`destructive_action_no_checkpoint`.

## Lineage / export honesty

Every record sets `raw_payload_excluded = true` and embeds only the evaluated
capability rows it reasoned over; it carries no raw source bytes. A source
limited-mode record that is not export-safe narrows with `posture_export_unsafe`.

## Stable qualification (auto-narrow)

A large-file posture record is Stable-qualified only when it can prove the
contract on the captured posture. It auto-narrows below Stable with a named
reason when:

- `source_read_not_byte_faithful` — whole-file load is admitted in limited mode;
- `preview_not_binary_safe` — binary content is not given a binary-safe preview;
- `canonical_target_unresolved` — the canonical write target could not resolve;
- `whole_file_write_not_restricted` — a whole-file write participant is admitted;
- `override_route_not_disclosed` — the escalation route carries no disclosure;
- `destructive_action_no_checkpoint` — a destructive action has no compare +
  checkpoint inspection path;
- `posture_export_unsafe` — the record or its capability set is not export-safe.

Limited mode being active is itself the protective posture: a read-only
constrained viewer over a binary file is the contract working as designed, a
pass, not a gap. Narrowing fires only when a protection is missing.

## Consuming surfaces

`large_file_posture_lines` is the single human-readable projection shared by the
editor large-file status surface, the headless CLI emitter
(`aureline_large_file_posture`), Help/About, and support export, so no surface
clones status text from another. The record excludes raw source bytes, so support
export is safe by construction (`raw_payload_excluded = true`).
