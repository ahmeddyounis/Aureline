# Text-fidelity, Unicode-position, encoding / BOM / EOL, decode-recovery, and safe-save verification seed

This packet freezes one reviewer-facing story for text correctness
before buffer, VFS, formatter, or protocol-adapter work starts minting
per-surface position, encoding, or decode-recovery language. It exists
so text and IO work can cite one corpus for position correctness,
byte-fidelity expectations, and malformed-text save consequences, and
so support / export and future UX work inherit the same vocabulary.

If this packet, the machine-readable corpora, and the source-fidelity
packet disagree, the underlying ADRs and PRD clauses still win, but
the packet and its companions must update in the same change so review,
support, and future automation read one story.

Companion artifacts:

- [`/fixtures/text/unicode_position_manifest.yaml`](../../fixtures/text/unicode_position_manifest.yaml)
  — machine-readable corpus of internal-position vs protocol-coordinate
  translation cases (UTF-8 byte offsets, UTF-16 code units, UTF-32
  code points, grapheme clusters, combining marks, ZWJ emoji, bidi).
- [`/fixtures/text/encoding_eol_corpus/`](../../fixtures/text/encoding_eol_corpus/)
  — machine-readable corpus of encoding, BOM, and line-ending cases
  with explicit byte sequences and expected decode / save projections.
- [`/artifacts/text/decode_recovery_examples/`](../../artifacts/text/decode_recovery_examples/)
  — worked decode-recovery records covering invalid UTF-8 bytes,
  unknown-encoding fallback, explicit encoding override, BOM
  correction, lossy re-encode warning, and binary-like block.
- [`/artifacts/text/save_consequence_examples/`](../../artifacts/text/save_consequence_examples/)
  — worked save-consequence records covering exact round-trip, explicit
  EOL-rewrite warning, save refusal pending decode recovery, protocol
  coordinate translation, copy / paste representation choice, and
  support / export raw-byte capture.
- [`/docs/editor/decode_recovery_and_save_consequence_contract.md`](../editor/decode_recovery_and_save_consequence_contract.md)
  — editor-facing review contract for malformed, mixed-encoding, or
  save-sensitive text, including the finer save-now consequence classes
  and representation availability rules.
- [`/fixtures/editor/decode_recovery_cases/`](../../fixtures/editor/decode_recovery_cases/)
  — YAML fixtures that project the editor-facing review contract across
  editor, diff, import preview, support export, security review, and
  save-pipeline surfaces.
- [`/docs/verification/source_fidelity_and_undo_packet.md`](./source_fidelity_and_undo_packet.md)
  — sibling packet freezing save rewrite-class and recovery-label
  vocabulary; this packet composes with it rather than re-defining
  those fields.
- [`/artifacts/io/save_rewrite_classes.yaml`](../../artifacts/io/save_rewrite_classes.yaml)
  — rewrite-class taxonomy this packet references for save-consequence
  rows.
- [`/docs/adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md)
  — coordinate system, encoding boundary, decode-recovery handoff,
  and source-fidelity rules.
- [`/docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md)
  — save pipeline, compare-before-write, and degraded save mode rules.
- [`/docs/adr/0002-renderer-text-stack-and-shaping-fallback.md`](../adr/0002-renderer-text-stack-and-shaping-fallback.md)
  — grapheme, word, and bidi segmentation contract the position corpus
  quotes rather than re-derives.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md` — §5.29 text model and Unicode correctness
  rules, `FR-EDIT-001`, `REL-IO-004`, `A11Y-TEXT-004`, and the
  `RFC-021` / `EPIC-022` text-fidelity register entries.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  `REL-FS-013` filesystem-identity rules and `REL-MUT-014` mutation
  journal / recovery-class contract.
- `.t2/docs/Aureline_Technical_Design_Document.md` — `TOOL-FMT-014`
  staged save participants and whole-file-rewrite disclosure.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — §11.11 text fidelity,
  Unicode, and decode recovery; invisible-character inspector, reveal
  modes, and copy-rendered / copy-escaped / copy-raw distinction.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.text_fidelity.unicode_and_encoding_seed
evidence_id: evidence.editor_io.text_fidelity_seed
title: Text-fidelity, Unicode-position, encoding / BOM / EOL, decode-recovery, and safe-save verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - FR-EDIT-001
    - REL-IO-004
    - REL-FS-013
    - REL-MUT-014
    - TOOL-FMT-014
    - A11Y-TEXT-004
    - RFC-021
    - EPIC-022
  claim_row_refs:
    - packet_row:text_fidelity.position_model_contract
    - packet_row:text_fidelity.protocol_coordinate_projection
    - packet_row:text_fidelity.encoding_bom_eol_field_set
    - packet_row:text_fidelity.decode_recovery_vocabulary
    - packet_row:text_fidelity.representation_choice
    - packet_row:text_fidelity.save_consequence_classes
    - packet_row:text_fidelity.seed_corpus
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: commit:working_tree
  trigger_revision: text_fidelity_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen position-model, encoding, decode-recovery,
    and save-fidelity contracts. No implementation or platform-lab pass
    is claimed yet. The packet provides one reviewable shape the buffer,
    VFS, formatter, and protocol-adapter lanes can project from.
artifact_links:
  supporting_evidence_ids:
    - evidence.editor_io.unicode_position_corpus
    - evidence.editor_io.encoding_eol_corpus
    - evidence.editor_io.decode_recovery_examples
    - evidence.editor_io.save_consequence_examples
    - evidence.editor_io.save_rewrite_class_vocabulary
    - evidence.editor_io.decode_recovery_review_cases
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/text/unicode_position_manifest.yaml
    - fixtures/text/encoding_eol_corpus/
    - artifacts/text/decode_recovery_examples/
    - artifacts/text/save_consequence_examples/
    - fixtures/editor/decode_recovery_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/adr/0002-renderer-text-stack-and-shaping-fallback.md
    - docs/adr/0003-buffer-undo-large-file.md
    - docs/adr/0006-vfs-save-cache-identity.md
    - docs/verification/source_fidelity_and_undo_packet.md
    - docs/editor/decode_recovery_and_save_consequence_contract.md
    - artifacts/io/save_rewrite_classes.yaml
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This packet freezes one reviewer-facing field set for text fidelity,
decode recovery, and save consequences. It does not claim the buffer,
VFS, formatter, or protocol-adapter implementation is complete; it
claims only that the required position model, coordinate projections,
encoding / BOM / EOL field set, decode-recovery vocabulary,
representation choices, and save-consequence classes now exist and can
be cited without per-feature text truth.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:text_fidelity.position_model_contract` | `FR-EDIT-001`, `A11Y-TEXT-004`, `RFC-021` | `seed_only` | `internal` | `evidence.editor_io.unicode_position_corpus` | Internal position model vocabulary and invariants. |
| `packet_row:text_fidelity.protocol_coordinate_projection` | `RFC-021`, `EPIC-022` | `seed_only` | `internal` | `evidence.editor_io.unicode_position_corpus` | Projection rules for UTF-8 byte offsets, UTF-16 code units, UTF-32 code points, and grapheme indices at external protocol boundaries. |
| `packet_row:text_fidelity.encoding_bom_eol_field_set` | `REL-IO-004`, `FR-EDIT-001`, `EPIC-022` | `seed_only` | `internal` | `evidence.editor_io.encoding_eol_corpus` | Encoding / BOM / EOL / final-newline fidelity projection field set. |
| `packet_row:text_fidelity.decode_recovery_vocabulary` | `REL-IO-004`, `REL-MUT-014`, `RFC-021` | `seed_only` | `internal` | `evidence.editor_io.decode_recovery_examples` | Decode-recovery label set and raw-byte preservation rules. |
| `packet_row:text_fidelity.representation_choice` | `REL-IO-004`, `A11Y-TEXT-004` | `seed_only` | `internal` | `evidence.editor_io.decode_recovery_examples` | Rendered / escaped / raw-byte representation vocabulary used by copy, inspection, and support capture. |
| `packet_row:text_fidelity.save_consequence_classes` | `REL-IO-004`, `REL-FS-013`, `TOOL-FMT-014` | `seed_only` | `internal` | `evidence.editor_io.save_consequence_examples` | Save-consequence classes when exact round-trip cannot be guaranteed. |
| `packet_row:text_fidelity.seed_corpus` | `REL-IO-004`, `REL-MUT-014`, `FR-EDIT-001` | `seed_only` | `internal` | `evidence.editor_io.unicode_position_corpus`, `evidence.editor_io.encoding_eol_corpus` | Stable corpus ids for position, encoding, decode-recovery, and save-consequence review. |

## What this seed freezes

- One **position model** vocabulary describing the internal
  representation (UTF-8 bytes, line starts, grapheme clusters) and the
  supported external projections (UTF-8 byte offsets, UTF-16 code
  units, UTF-32 code points, grapheme indices, column positions).
- One **coordinate-translation** invariant set so editor commands,
  search, diagnostics, LSP / DAP / notebook adapters, copy / paste, and
  selection offsets cross the protocol boundary without silent drift.
- One **encoding / BOM / EOL fidelity field set** that composes with
  the `source_fidelity_and_undo` packet rather than re-defining those
  fields.
- One **decode-recovery label set** that pins raw-byte preservation
  and reuses the `decode_recovery_change` undo class from ADR 0003.
- One **representation-choice vocabulary** (`rendered`, `escaped`,
  `raw_bytes`, `mixed_with_replacement`) that copy, inspection, and
  support / export capture surfaces reuse verbatim.
- One **save-consequence class set** naming the three honest outcomes
  when bytes cannot be round-tripped exactly: `preserve_untouched`,
  `rewrite_with_warning`, `block_pending_review`.

## Position-model field set

Use these fields whenever Aureline explains where a cursor, selection,
diagnostic range, or protocol-provided range falls in a buffer. The
packet may be projected from a buffer snapshot, a selection record, or
a protocol range record.

| Packet field | Meaning | Allowed values / source |
|---|---|---|
| `internal_unit` | The unit the buffer exposes internally. | `utf8_byte_offset` |
| `line_addressing` | How the buffer indexes lines. | `line_start_index`, `line_column_grapheme`, `line_column_utf16` |
| `cluster_unit` | The user-visible unit for caret movement, selection, and deletion. | `grapheme_cluster` |
| `external_projections` | Supported external projection units. | `utf8_byte_offset`, `utf16_code_unit`, `utf32_code_point`, `grapheme_index`, `visual_column` |
| `coordinate_adapter_class` | Why a translation is being performed. | `lsp_adapter`, `dap_adapter`, `notebook_adapter`, `search_result_adapter`, `clipboard_exchange`, `support_export` |
| `translation_confidence` | Whether the projection is exact or approximate for the current buffer. | `exact`, `exact_for_bmp_only`, `lossy_surrogate_split_blocked`, `lossy_replacement_used`, `unavailable_in_decode_recovery` |
| `bidi_posture` | How directionality is exposed at the boundary. | `logical_order_preserved`, `reveal_controls_available`, `suspicious_controls_flagged` |

Rules:

1. The internal unit is always `utf8_byte_offset`. Adapters translate
   outward; they do not push external encodings inward. The editor
   command graph never sees non-UTF-8 bytes (ADR 0003).
2. Caret movement, selection, deletion, backspace, and multi-cursor
   operations are grapheme-aware. A projection that would split a
   surrogate pair, combining mark, ZWJ sequence, or regional-indicator
   flag MUST raise `lossy_surrogate_split_blocked` rather than silently
   truncating.
3. External projections are computed on demand at the protocol
   boundary; the buffer never caches a second authoritative coordinate
   system.
4. In decode-recovery state, external projections over the
   not-yet-resolved byte region MUST report
   `unavailable_in_decode_recovery`; adapters MAY still address
   resolved regions.

## Encoding / BOM / EOL fidelity field set

This set extends the `source_fidelity_and_undo` packet's save-fidelity
vocabulary with the **open-time detection** story. A committed save
projects both sets; an open or paste event projects only this set.

| Packet field | Meaning | Allowed values / source |
|---|---|---|
| `detected_encoding` | The encoding the buffer adopted at open or paste. | `utf8`, `utf8_bom`, `utf16le_bom`, `utf16be_bom`, `utf32le_bom`, `utf32be_bom`, `declared_override`, `legacy_8bit_fallback`, `unknown_binary_like` |
| `detection_source` | Why that encoding was chosen. | `bom`, `declared_metadata`, `utf8_heuristic`, `platform_default_with_threshold`, `user_override`, `workspace_policy`, `decode_failed_no_choice` |
| `bom_state_detected` | Whether a BOM was present at open. | `present`, `absent`, `unknown_or_degraded` |
| `newline_mode_detected` | Dominant newline mode at open. | `lf`, `crlf`, `cr_only`, `mixed`, `unknown_or_degraded` |
| `final_newline_detected` | Whether a final newline was present at open. | `present`, `absent`, `unknown_or_degraded` |
| `encoding_state` | Save-time encoding projection. | See `source_fidelity_and_undo_packet.md` |
| `bom_state` | Save-time BOM projection. | See `source_fidelity_and_undo_packet.md` |
| `newline_mode_state` | Save-time newline projection. | See `source_fidelity_and_undo_packet.md` |
| `final_newline_state` | Save-time final-newline projection. | See `source_fidelity_and_undo_packet.md` |

Rules:

1. Detection runs once at open (BOM → declared metadata → heuristic →
   platform default → user override). The detected value is sticky for
   the lifetime of the buffer (ADR 0003). Re-detection requires an
   explicit user or policy action and emits a `decode_recovery_change`
   transaction.
2. BOM presence is preserved as detected. Adding or removing a BOM is
   an explicit command and MUST project `added_explicitly` or
   `removed_explicitly`; no formatter or save participant may flip
   `bom_state` silently.
3. Newline-mode changes are explicit commands and project
   `converted_explicitly`. Mixed-newline inputs preserve the dominant
   mode and project `mixed_input_preserved`.
4. An `unknown_or_degraded` detection MUST downgrade the save path to
   either `rewrite_with_warning` or `block_pending_review` (see
   save-consequence classes below).

## Decode-recovery vocabulary

Decode-recovery state is entered when detection and decoding cannot
produce a loss-free text view. Raw on-disk bytes are preserved verbatim
in the recovery journal; editing is gated until resolution.

| `decode_recovery_class` | Meaning | Raw-byte posture | Resolution requirement |
|---|---|---|---|
| `clean_decode` | Buffer decoded without loss. | n/a | n/a |
| `replacement_char_inserted` | Decoder produced U+FFFD for one or more byte runs. | preserved in recovery journal | explicit review or override before save |
| `invalid_byte_sequence_detected` | One or more byte runs cannot be decoded by the detected encoding. | preserved in recovery journal | explicit encoding override, binary mode, or inspection |
| `unknown_encoding_fallback` | Detection could not choose with confidence. | preserved in recovery journal | explicit encoding override or binary mode |
| `explicit_user_override` | User chose a different encoding for the buffer. | preserved in recovery journal | `decode_recovery_change` transaction committed |
| `binary_like_blocked` | Content looks non-text (NUL density, sniffed binary signature). | preserved unchanged | buffer opens in binary-view or large-file mode |
| `lossy_reencode_warning` | Re-encode on save would not round-trip exactly. | preserved until save resolves | explicit acknowledgment or encoding change |

Rules:

1. Entering decode-recovery state never destroys or rewrites the raw
   on-disk bytes. Save is blocked on the not-yet-resolved regions.
2. Resolving decode-recovery state emits a `decode_recovery_change`
   transaction (ADR 0003 undo-class taxonomy) with
   `reversal_class = restore_from_checkpoint`.
3. `lossy_reencode_warning` is the only state where a save may proceed
   after explicit acknowledgment; it MUST project
   `encoding_state = explicit_conversion` or
   `encoding_state = decode_recovery_override` and MUST carry a
   `rewrite_with_warning` save consequence.
4. `replacement_char_inserted` and `invalid_byte_sequence_detected` are
   not interchangeable. The first names a lossy decode that succeeded
   with substitutions; the second names byte runs that have no decode.

## Representation-choice vocabulary

Copy, inspection, support bundles, and export capture all choose a
representation when the rendered form, the escaped form, and the raw
bytes may diverge. Controlled vocabulary:

| `representation_class` | Meaning | Typical use |
|---|---|---|
| `rendered` | The visible glyphs as the editor rendered them. | ordinary copy into another editor surface |
| `escaped` | Code-point escapes (`\u{...}`, `\x..`) with invisible and bidi controls revealed. | copying into a chat, ticket, or review comment where rendering cannot be trusted |
| `raw_bytes` | The bytes as they exist on disk (or in the recovery journal, when in decode-recovery state). | security review, encoding debugging, support bundle capture |
| `mixed_with_replacement` | Rendered with `U+FFFD` substitutions for undecodable runs. | inspection-only, never the default copy choice |

Rules:

1. The three copy actions `Copy rendered text`, `Copy escaped text`,
   and `Copy raw bytes` (§11.11 UI/UX spec) MUST map to exactly one
   `representation_class` each.
2. A surface may default to `rendered`, but MUST make `escaped` and
   `raw_bytes` reachable whenever the buffer is in any
   `decode_recovery_class` other than `clean_decode`.
3. Support / export capture over a decode-recovery region MUST default
   to `raw_bytes` and MUST NOT present `rendered` without the
   corresponding `escaped` or `raw_bytes` form alongside it.

## Save-consequence classes

When the product cannot round-trip bytes exactly, exactly one of these
three consequences applies. This vocabulary composes with the
`rewrite_class` vocabulary from
[`save_rewrite_classes.yaml`](../../artifacts/io/save_rewrite_classes.yaml);
both MUST agree on any record.

| `save_consequence_class` | Meaning | Required disclosure label | Compatible `rewrite_class` |
|---|---|---|---|
| `preserve_untouched` | Bytes outside the declared edit ranges are written back verbatim. | `Preserved source fidelity` | `targeted_content_patch`, `no_write_needed` |
| `rewrite_with_warning` | A save participant or explicit command changed bytes Aureline cannot prove were the user's intent (EOL conversion, BOM add / remove, encoding conversion, whole-file format). | `Explicit rewrite on save` (composes with the matching `Whole-file rewrite` / `Whole-file rewrite fallback` label where applicable) | `whole_file_rewrite_declared`, `whole_file_rewrite_fallback`, `targeted_content_patch` (only with an explicit conversion acknowledgment) |
| `block_pending_review` | The save pipeline refused to write because decode recovery, unknown encoding, mixed-newline ambiguity, external change, or policy review has not resolved. | `No durable write` | `blocked_no_write` |

Rules:

1. A save MUST carry exactly one `save_consequence_class`. Surfaces
   that project a save event may NOT collapse `rewrite_with_warning`
   into a generic "Saved" or `preserve_untouched` into "Saved with
   changes".
2. `rewrite_with_warning` is the only consequence where Aureline
   changes bytes outside the user's declared edit ranges. Any save
   that changes BOM state, normalization form, newline mode, final
   newline, or encoding MUST project `rewrite_with_warning` even if
   the change was user-requested.
3. `block_pending_review` preserves the buffer and the on-disk file.
   The corresponding `rewrite_class` is always `blocked_no_write` and
   the corresponding `recovery_class` (from the
   `source_fidelity_and_undo` packet) is `no_state_change`.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.editor_io.unicode_position_corpus` | `verification_corpus` | Position-model and coordinate-translation rows | current with packet revision 1 | [`fixtures/text/unicode_position_manifest.yaml`](../../fixtures/text/unicode_position_manifest.yaml) |
| `evidence.editor_io.encoding_eol_corpus` | `verification_corpus` | Encoding, BOM, and EOL corpus with explicit byte sequences | current with packet revision 1 | [`fixtures/text/encoding_eol_corpus/`](../../fixtures/text/encoding_eol_corpus/) |
| `evidence.editor_io.decode_recovery_examples` | `verification_corpus` | Worked records for decode-recovery classes and representation choice | current with packet revision 1 | [`artifacts/text/decode_recovery_examples/`](../../artifacts/text/decode_recovery_examples/) |
| `evidence.editor_io.save_consequence_examples` | `verification_corpus` | Worked records for preserve-untouched, rewrite-with-warning, block-pending-review | current with packet revision 1 | [`artifacts/text/save_consequence_examples/`](../../artifacts/text/save_consequence_examples/) |
| `evidence.editor_io.save_rewrite_class_vocabulary` | `verification_corpus` | Composing rewrite-class vocabulary from the sibling save packet | current with packet revision 1 | [`artifacts/io/save_rewrite_classes.yaml`](../../artifacts/io/save_rewrite_classes.yaml) |
| `evidence.editor_io.decode_recovery_review_cases` | `verification_corpus` | Editor-facing decode-recovery review records for malformed, mixed-encoding, import preview, diff, support export, and save-now consequence cases | current with packet revision 1 | [`fixtures/editor/decode_recovery_cases/`](../../fixtures/editor/decode_recovery_cases/) |

## Corpus coverage

The machine-readable corpora are the canonical row sets. The tables
below are the reviewer-facing summaries.

Position and coordinate rows (`unicode_position_manifest.yaml`):

| Corpus row id | Primary case | Key projection |
|---|---|---|
| `corpus.text.position.ascii_ltr_baseline` | ASCII-only LTR line | `utf8_byte_offset == utf16_code_unit == grapheme_index` |
| `corpus.text.position.bmp_cjk_mixed` | CJK + Latin in the BMP | UTF-16 units differ from UTF-8 bytes; grapheme count unchanged |
| `corpus.text.position.supplementary_emoji` | Supplementary-plane emoji | UTF-16 surrogate pair, one grapheme, no split allowed |
| `corpus.text.position.combining_marks_nfc_nfd` | NFC vs NFD for the same visible string | grapheme count preserved across normalization |
| `corpus.text.position.zwj_emoji_cluster` | Family / profession ZWJ emoji | one grapheme spans many code points |
| `corpus.text.position.regional_indicator_flag` | Regional-indicator pair | one flag grapheme; pair may not be split |
| `corpus.text.position.variation_selector` | Text vs emoji variation selector | selector stays with base code point |
| `corpus.text.position.bidi_mixed_segment` | Latin + Arabic mixed line | logical order preserved; reveal-controls available |
| `corpus.text.position.invisible_controls` | Bidi control + NBSP + zero-width space | inspector exposes class, count, codepoint |
| `corpus.text.position.protocol_coordinate_lsp_utf16` | Buffer byte offset → LSP UTF-16 unit | exact projection on BMP; surrogate-split blocked on supplementary |
| `corpus.text.position.protocol_coordinate_dap_utf8` | Buffer byte offset → DAP UTF-8 byte | identity projection |
| `corpus.text.position.visual_column_tab_expanded` | Tab-sensitive column selection | visual column depends on tab width, not buffer unit |

Encoding / BOM / EOL rows (`encoding_eol_corpus/manifest.yaml`):

| Corpus row id | Primary case | Expected detection | Notes |
|---|---|---|---|
| `corpus.text.encoding.utf8_lf_no_bom_no_finalnl` | UTF-8, LF, no BOM, no final newline | `utf8` / `absent` / `lf` / `absent` | baseline preserve case |
| `corpus.text.encoding.utf8_bom_crlf_finalnl` | UTF-8 BOM, CRLF, final newline present | `utf8_bom` / `present` / `crlf` / `present` | BOM- and CRLF-preserving save |
| `corpus.text.encoding.utf16le_bom_lf` | UTF-16LE BOM, LF (0x0A 0x00) lines | `utf16le_bom` / `present` / `lf` / `present` | UTF-16 unit ≠ UTF-8 byte |
| `corpus.text.encoding.utf16be_bom_crlf` | UTF-16BE BOM, CRLF | `utf16be_bom` / `present` / `crlf` / `present` | covers big-endian path |
| `corpus.text.encoding.utf32le_bom_lf` | UTF-32LE BOM, LF | `utf32le_bom` / `present` / `lf` / `present` | covers 4-byte code-unit path |
| `corpus.text.encoding.mixed_crlf_lf_dominant_lf` | mixed line endings, LF dominant | `utf8` / `absent` / `mixed` / varies | mixed-input preservation |
| `corpus.text.encoding.cr_only_classic` | lone-CR legacy input | `utf8` / `absent` / `cr_only` / varies | classic-Mac posture |
| `corpus.text.encoding.legacy_8bit_fallback` | bytes > 0x7F with no BOM and no declared encoding | `legacy_8bit_fallback` / `absent` / detected / varies | detection confidence is platform-default threshold |
| `corpus.text.encoding.invalid_utf8_bytes` | truncated multi-byte sequence | decode-recovery | raises `invalid_byte_sequence_detected` |
| `corpus.text.encoding.unknown_encoding_ambiguous` | text that sniffs as neither UTF-8 nor a confident legacy 8-bit | decode-recovery | raises `unknown_encoding_fallback` |
| `corpus.text.encoding.binary_like_nul_dense` | NUL-dense content | decode-recovery | raises `binary_like_blocked` |
| `corpus.text.encoding.bom_correction_added` | user adds BOM to a previously BOM-less file | explicit conversion | `bom_state = added_explicitly`, `save_consequence_class = rewrite_with_warning` |

Decode-recovery examples (`artifacts/text/decode_recovery_examples/`):

| Corpus row id | Primary case | `decode_recovery_class` | Primary artifact |
|---|---|---|---|
| `corpus.text.decode.invalid_utf8_replacement` | invalid UTF-8 run replaced with U+FFFD on inspection only | `invalid_byte_sequence_detected` | [`invalid_utf8_replacement.json`](../../artifacts/text/decode_recovery_examples/invalid_utf8_replacement.json) |
| `corpus.text.decode.unknown_encoding_fallback` | detection cannot choose with confidence; user prompted | `unknown_encoding_fallback` | [`unknown_encoding_fallback.json`](../../artifacts/text/decode_recovery_examples/unknown_encoding_fallback.json) |
| `corpus.text.decode.explicit_user_override` | user picks a different encoding; buffer rereads bytes | `explicit_user_override` | [`explicit_user_override.json`](../../artifacts/text/decode_recovery_examples/explicit_user_override.json) |
| `corpus.text.decode.bom_correction_added` | BOM added to a previously BOM-less file by explicit command | `clean_decode` (BOM change only) | [`bom_correction_added.json`](../../artifacts/text/decode_recovery_examples/bom_correction_added.json) |
| `corpus.text.decode.lossy_reencode_warning` | save would lose bytes on re-encode; user prompted | `lossy_reencode_warning` | [`lossy_reencode_warning.json`](../../artifacts/text/decode_recovery_examples/lossy_reencode_warning.json) |
| `corpus.text.decode.binary_like_blocked` | content sniffs as binary; buffer blocks text edits | `binary_like_blocked` | [`binary_like_blocked.json`](../../artifacts/text/decode_recovery_examples/binary_like_blocked.json) |

Save-consequence examples (`artifacts/text/save_consequence_examples/`):

| Corpus row id | Primary case | `save_consequence_class` | Primary artifact |
|---|---|---|---|
| `corpus.text.save.exact_round_trip_preserved` | targeted patch with encoding, BOM, EOL, and final newline preserved | `preserve_untouched` | [`exact_round_trip_preserved.json`](../../artifacts/text/save_consequence_examples/exact_round_trip_preserved.json) |
| `corpus.text.save.explicit_eol_conversion_warning` | user converts CRLF → LF through an explicit command | `rewrite_with_warning` | [`explicit_eol_conversion_warning.json`](../../artifacts/text/save_consequence_examples/explicit_eol_conversion_warning.json) |
| `corpus.text.save.blocked_pending_decode_recovery` | save refused because decode recovery has not resolved | `block_pending_review` | [`blocked_pending_decode_recovery.json`](../../artifacts/text/save_consequence_examples/blocked_pending_decode_recovery.json) |
| `corpus.text.save.protocol_coordinate_adapter_translation` | LSP range translated across UTF-16 / UTF-8 boundary during apply | `preserve_untouched` | [`protocol_coordinate_adapter_translation.json`](../../artifacts/text/save_consequence_examples/protocol_coordinate_adapter_translation.json) |
| `corpus.text.save.copy_paste_representation_choice` | copy action chooses rendered / escaped / raw_bytes | `preserve_untouched` (no save boundary crossed) | [`copy_paste_representation_choice.json`](../../artifacts/text/save_consequence_examples/copy_paste_representation_choice.json) |
| `corpus.text.save.support_export_raw_bytes_capture` | support export captures raw bytes of a decode-recovery region | `preserve_untouched` (no save boundary crossed) | [`support_export_raw_bytes_capture.json`](../../artifacts/text/save_consequence_examples/support_export_raw_bytes_capture.json) |

Editor decode-recovery review cases (`fixtures/editor/decode_recovery_cases/`):

| Fixture | Primary case | Primary surface |
|---|---|---|
| [`mixed_encoding_import_preview.yaml`](../../fixtures/editor/decode_recovery_cases/mixed_encoding_import_preview.yaml) | mixed encoding suspicion blocks import commit until a decoder is chosen | `import_preview` |
| [`invalid_utf8_editor_blocked_save.yaml`](../../fixtures/editor/decode_recovery_cases/invalid_utf8_editor_blocked_save.yaml) | invalid UTF-8 run is inspectable with raw bytes preserved and save blocked | `editor` |
| [`bom_eol_normalization_warning.yaml`](../../fixtures/editor/decode_recovery_cases/bom_eol_normalization_warning.yaml) | explicit BOM and newline normalization projects rewrite-with-warning | `save_pipeline` |
| [`diff_view_raw_rendered_divergence.yaml`](../../fixtures/editor/decode_recovery_cases/diff_view_raw_rendered_divergence.yaml) | diff hunk keeps raw/escaped peers and suspicious-text links for mixed-direction content | `diff` |
| [`lossy_reencode_acknowledgment.yaml`](../../fixtures/editor/decode_recovery_cases/lossy_reencode_acknowledgment.yaml) | lossy target encoding blocks save until explicit acknowledgment | `save_pipeline` |
| [`replace_undecodable_spans_warning.yaml`](../../fixtures/editor/decode_recovery_cases/replace_undecodable_spans_warning.yaml) | explicit replacement commits U+FFFD only after warning review | `save_pipeline` |
| [`support_export_security_review_raw_bytes.yaml`](../../fixtures/editor/decode_recovery_cases/support_export_security_review_raw_bytes.yaml) | support/security export defaults to raw bytes with escaped companion evidence | `support_export` |

## Verification method

- **Verification classes used:** design review, corpus freeze, example
  record review.
- **Procedure summary:** derive the position-model and coordinate
  vocabulary from ADR 0003 and PRD §5.29; project encoding / BOM / EOL
  fidelity through the sibling save packet; pin decode-recovery and
  representation-choice vocabularies against UI/UX §11.11; author one
  worked record per decode-recovery and save-consequence case.
- **Automation refs:** no dedicated validator yet; this seed relies on
  JSON / YAML parse checks and future packet-level validation work.

## Known gaps and waivers

- **Waiver refs:** `none`
- **Known-limit refs:** `none`
- **Migration-packet refs:** `none`
- **Explicit gaps:** the packet does not yet claim live buffer, VFS,
  formatter, or protocol-adapter coverage; corpus rows are reviewable
  shape, not runtime transcripts.
- **Explicit gaps:** full language-server interoperability matrices
  (per-server UTF negotiation, DAP variant matrices, notebook protocol
  adapters) are out of scope for this seed.
- **Explicit gaps:** shipping the editor-surface warning and inspector
  chrome for every case is deferred to UX work; this packet freezes
  only the vocabulary those surfaces must reuse.

## Reviewer signoff

- **Reviewer / forum:** `not_yet_reviewed`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-23`
- **Reviewed claim rows:**
  `packet_row:text_fidelity.position_model_contract`,
  `packet_row:text_fidelity.protocol_coordinate_projection`,
  `packet_row:text_fidelity.encoding_bom_eol_field_set`,
  `packet_row:text_fidelity.decode_recovery_vocabulary`,
  `packet_row:text_fidelity.representation_choice`,
  `packet_row:text_fidelity.save_consequence_classes`,
  `packet_row:text_fidelity.seed_corpus`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `corpus_or_fixture_revision_changed`
- **Expected freshness window:** refresh within `P30D` or whenever the
  position-model, decode-recovery, or save-consequence vocabulary, the
  sibling `source_fidelity_and_undo` packet, or any of the four
  companion corpora change.
- **Next packet family to update with the same evidence ids:** release
  evidence and support-export packets covering editor / IO truth.
