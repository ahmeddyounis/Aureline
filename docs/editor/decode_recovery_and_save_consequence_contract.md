# Decode-recovery review and save-consequence contract

This contract freezes the user-facing review model for malformed,
uncertain, mixed-encoding, or save-sensitive text. It exists so editor,
diff, import preview, support export, and security review surfaces answer
the same questions before any surface renders misleading text or rewrites
bytes:

- what bytes were found;
- how Aureline decoded or failed to decode those bytes;
- how confident that decision is; and
- what will happen if the user saves now.

The contract is a review and packet shape. It does not select the final
decoder library or implement a binary viewer.

## Companion artifacts

- [`/schemas/editor/decode_recovery_review.schema.json`](../../schemas/editor/decode_recovery_review.schema.json)
  defines the review record used by editor, diff, import preview,
  support export, security review, and save-pipeline projections.
- [`/schemas/editor/text_representation_choice.schema.json`](../../schemas/editor/text_representation_choice.schema.json)
  defines raw-byte, rendered-text, and escaped-text availability rows.
- [`/fixtures/editor/decode_recovery_cases/`](../../fixtures/editor/decode_recovery_cases/)
  contains worked YAML cases for mixed encodings, undecodable bytes,
  BOM/EOL conversion, import preview, diff view, lossy replacement, raw
  support export, and save blocking.
- [`/docs/verification/text_fidelity_packet.md`](../verification/text_fidelity_packet.md)
  remains the broader text-fidelity seed. This contract narrows its
  decode-recovery and representation vocabulary for editor-facing
  review flows.
- [`/docs/verification/source_fidelity_and_undo_packet.md`](../verification/source_fidelity_and_undo_packet.md)
  remains the source for packet-level save-fidelity fields,
  `rewrite_class`, and `recovery_class` vocabulary.
- [`/docs/security/suspicious_content_packet.md`](../security/suspicious_content_packet.md)
  and [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  remain the source for suspicious-text detector and safe-preview
  linkage.

Normative design sources projected here:

- `.t2/docs/Aureline_PRD.md` section 5.29 and 10.16: source fidelity,
  encoding/newline preservation, decode failure preservation, raw versus
  rendered ambiguity, and content-integrity behavior.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections
  11.2 and 11.8: decoding failures never discard original bytes, and
  suspicious text uses shared cross-surface detectors.
- `.t2/docs/Aureline_Technical_Design_Document.md` section 8.15:
  suspicious-content warnings stay attached to exact content and copy,
  export, review, and evidence flows provide a safe representation path.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` section 11.11 and
  Appendix CK/CL: decode recovery sheets must explain attempted decode,
  raw-byte preservation, alternatives, save consequence, and mixed
  direction safe-copy actions.

## Scope

In scope:

- the review fields for bytes found, attempted decode, confidence,
  representation choices, and save-now consequence;
- raw-byte, rendered-text, and escaped-text availability rules across
  editor, diff, import preview, support export, and security review;
- user-facing save consequence classes that map back to the broader
  text-fidelity packet classes;
- required joins to suspicious-text indicators, mixed-direction
  inspectors, and support/export packets;
- fixture coverage for the required malformed and save-sensitive cases.

Out of scope:

- final decoder implementation, detection thresholds, or language
  binding selection;
- final binary-view implementation;
- final visual styling for review sheets, status items, or inspector
  chrome.

## Review record

Every reviewable decode-recovery surface emits exactly one
`decode_recovery_review_record` for the subject being shown or saved.
The record may be stored in a buffer review packet, save review packet,
support export, or security evidence packet.

Required fields:

| Field | Purpose |
|---|---|
| `review_id` | Stable opaque id for joins across editor, diff, import, support, and security records. |
| `trigger_surface` | Surface that caused the review: `editor`, `diff`, `import_preview`, `support_export`, `security_review`, or `save_pipeline`. |
| `subject` | Opaque content identity and bounded display label. Raw absolute paths or bodies do not appear here. |
| `decode_state` | Open-time detection, decode-recovery class, and raw-byte posture. |
| `byte_findings` | Byte ranges and raw hex samples that explain what was found. |
| `decode_attempts` | Ordered decoder attempts and their result. |
| `confidence` | User-facing confidence level and reason. |
| `representation_choices` | Raw-byte, rendered-text, and escaped-text availability rows. |
| `save_consequence` | What happens if the user saves now. |
| `linkage` | Joins to suspicious-text, mixed-direction, support export, and security review records. |
| `review_actions` | Available and blocked actions such as reopen, inspect raw bytes, copy escaped text, acknowledge rewrite, or block save. |
| `support_export` | Whether support/export must include raw bytes, escaped text, rendered text, or metadata only. |
| `invariants` | Machine-readable assertions the surface must preserve. |

### Decode state

The review record reuses the broad text-fidelity vocabulary and adds one
surface-facing class for mixed encoding suspicion:

| `decode_recovery_class` | Meaning | Save posture before resolution |
|---|---|---|
| `clean_decode` | Bytes decoded without loss. | Save may proceed if normal save checks pass. |
| `replacement_char_inserted` | Decoder produced one or more replacement markers. | Save is blocked or requires explicit acknowledgment before those markers can be committed. |
| `invalid_byte_sequence_detected` | One or more byte runs cannot be decoded by the selected encoding. | Save is blocked until the user chooses a decoder, binary view, or explicit replacement path. |
| `unknown_encoding_fallback` | Detection could not choose a decoder with enough confidence. | Save is blocked until the user chooses a decoder or binary posture. |
| `explicit_user_override` | User chose a different encoding and the buffer was reread from preserved bytes. | Save may proceed, but the override is a reviewable decode-recovery change. |
| `binary_like_blocked` | Content looks non-text, such as NUL-dense or a sniffed binary signature. | Text save is blocked; binary or large-file mode handles inspection. |
| `lossy_reencode_warning` | A requested save encoding cannot round-trip current text. | Save requires explicit acknowledgment or a different target encoding. |
| `mixed_encoding_suspected` | Different byte runs point to incompatible encodings and no single lossless decoder is proven. | Save is blocked in editor/import preview until the user chooses how to split, convert, or inspect. |

Raw-byte posture is separately recorded so a surface cannot claim
recovery while losing evidence:

| `raw_bytes_posture` | Meaning |
|---|---|
| `not_needed_clean_decode` | No recovery bytes are needed because decode was clean. |
| `preserved_in_open_buffer` | Exact bytes remain attached to the open buffer. |
| `preserved_in_recovery_journal` | Exact bytes are stored in a recovery journal and can be exported by reference. |
| `preserved_until_save_resolves` | Exact bytes remain available while a lossy re-encode or replacement decision is pending. |
| `unavailable_source_missing` | Source bytes are not available; only metadata can be shown. |
| `policy_redacted_metadata_only` | Policy prevents raw body export; metadata must explain the omission. |

## Byte findings

Each `byte_findings[]` row answers "what bytes were found" without
requiring the user to trust the rendered view.

Required fields:

| Field | Purpose |
|---|---|
| `finding_id` | Opaque id for a byte range finding. |
| `finding_class` | `valid_text_bytes`, `invalid_byte_sequence`, `replacement_character_emitted`, `mixed_encoding_run`, `bom_marker`, `newline_marker`, `nul_or_binary_signature`, or `unencodable_code_point`. |
| `byte_range` | Zero-based byte offset and length. |
| `raw_bytes_hex` | Space-separated `0xNN` bytes. |
| `rendered_projection` | Whether the run rendered exactly, rendered with replacement, only escapes safely, only raw bytes safely, or cannot render until a decoder is chosen. |
| `replacement_marker` | `U+FFFD`, `?`, or `null`. |
| `user_facing_summary` | Bounded explanation shown in review and exported evidence. |

Rules:

1. A review with undecodable, mixed, binary-like, BOM, or EOL-sensitive
   bytes must include at least one byte finding for the affected run.
2. Raw byte samples are hex encoded. User-facing surfaces may elide long
   bodies, but they must keep the byte range and raw-byte evidence
   reachable through review or export when policy permits.
3. Replacement markers are evidence of a projection, not proof that the
   source bytes were replaced.

## Confidence

Confidence is user-facing, not a hidden detector score.

| `confidence.level` | Meaning |
|---|---|
| `high` | BOM, declared metadata, explicit user override, or a strong heuristic makes the decode choice explainable. |
| `medium` | A fallback is plausible, but a review surface should still disclose why the choice was made. |
| `low` | Mixed encoding, replacement markers, or weak heuristics make save risky. |
| `unknown` | No decoder is chosen. Rendered text is unavailable or inspect-only. |
| `blocked` | Content is binary-like or policy prevents a safe text decode. |

Confidence must be paired with a `confidence.reason` such as
`bom_confirmed`, `mixed_encoding_suspected`, `decoder_replacement_used`,
`no_decoder_chosen`, or `lossy_reencode_detected`.

## Representation choices

Representation rows use
`text_representation_choice_record`. The three primary classes are:

| `representation_class` | Payload posture | Required label examples |
|---|---|---|
| `raw_bytes` | Exact source bytes or recovery-journal bytes. | `Copy raw bytes`, `Export raw bytes`, `Open raw bytes inspector`. |
| `rendered_text` | Glyphs as shown by the surface. | `Copy rendered text`, with replacement or suspicious text disclosed. |
| `escaped_text` | Code points and/or bytes expressed as escapes. | `Copy escaped text`, `Export escaped text`. |

Availability by surface:

| Surface | Default | Required availability |
|---|---|---|
| `editor` | `rendered_text` for clean decode; `escaped_text` or inspect-only when replacement markers are present. | `raw_bytes` and `escaped_text` must be reachable for any non-clean decode or suspicious text indicator. |
| `diff` | `rendered_text` for ordinary hunks. | `raw_bytes` and `escaped_text` must be reachable when inserted, deleted, or changed text includes replacement markers, bidi controls, invisible controls, or raw/rendered divergence. |
| `import_preview` | `escaped_text` or `raw_bytes` when decoder confidence is low or unknown. | `rendered_text` is unavailable until a decoder is chosen or must be paired with raw/escaped evidence when shown with replacement markers. |
| `support_export` | `raw_bytes` for decode-recovery regions when policy allows. | `escaped_text` must accompany raw bytes; `rendered_text` is included only if it is clearly labeled and paired with raw or escaped evidence. |
| `security_review` | `raw_bytes` or `escaped_text`. | `rendered_text` is never the only available representation where rendering can mislead. |

Rules:

1. Generic `Copy` and `Export` labels are non-conforming where
   representations differ materially.
2. `rendered_text` with `rendered_with_replacement_marker` requires a
   `replacement_marker_note` disclosure and at least one of `raw_bytes`
   or `escaped_text` as a peer representation.
3. `raw_bytes` requires preserved source bytes or recovery-journal
   bytes. If policy blocks the body, the surface must fall back to
   metadata-only support/export language and record why.
4. `escaped_text` remains available wherever raw bytes are preserved and
   the surface can safely express the bytes or code points as escapes.

## Save-consequence classes

The review contract uses a finer user-facing class set than the broader
text-fidelity packet. Every review class maps back to the packet-level
`save_consequence_class`.

| Review `consequence_class` | Meaning shown to the user | Packet mapping | Compatible rewrite posture |
|---|---|---|---|
| `preserve_untouched_raw_bytes` | Saving will keep raw bytes outside declared edit ranges unchanged. | `preserve_untouched` | `targeted_content_patch` or `no_write_needed`. |
| `normalize_encoding_or_newlines` | Saving will intentionally change encoding, BOM, newline mode, final newline, or normalization. | `rewrite_with_warning` | Usually `targeted_content_patch`, `whole_file_rewrite_declared`, or `whole_file_rewrite_fallback`. |
| `replace_undecodable_spans` | Saving will commit replacement markers or chosen substitutes for byte runs that did not decode. | `rewrite_with_warning` | Rewrite is allowed only after explicit user acknowledgment. |
| `block_save_pending_review` | Saving now performs no durable write because decode recovery, mixed encoding, binary-like content, external change, or policy review is unresolved. | `block_pending_review` | `blocked_no_write`. |
| `require_explicit_acknowledgment` | Saving is currently blocked until the user acknowledges a known lossy rewrite. | `block_pending_review` before acknowledgment; `rewrite_with_warning` after acknowledgment. | No write before acknowledgment; warning rewrite after acknowledgment. |

Rules:

1. A surface that offers save over a review record must display exactly
   one review `consequence_class` before durable write.
2. Any class that changes bytes outside the declared edit ranges must
   use `rewrite_with_warning` at packet level and keep the disclosure
   visible in save review, diff review, and support/export evidence.
3. `block_save_pending_review` and `require_explicit_acknowledgment`
   must report `bytes_outside_declared_ranges = not_written` until the
   user resolves the review.
4. Acknowledgment is not a decoder. If the user acknowledges a lossy
   rewrite, the post-acknowledgment projection still records the
   exact replacement or normalization consequence.

## Linkage requirements

Decode recovery does not live alone. The review record must link to
adjacent integrity records when they exist:

| Link field | Required when |
|---|---|
| `suspicious_text_indicator_refs` | Bidi controls, invisible formatting, confusables, replacement markers, or raw/rendered divergence are detected. |
| `mixed_direction_inspector_refs` | The affected text mixes RTL prose with LTR code, paths, flags, hostnames, or command-like spans. |
| `support_export_packet_refs` | The review is exported, attached to a support packet, or blocks a support/export body. |
| `security_review_refs` | The review participates in a trust, install, approval, delete, or security review surface. |
| `source_packet_refs` | Always includes the relevant text-fidelity, source-fidelity, or suspicious-content packet ids. |

Rules:

1. Suspicious-text indicators and decode-recovery markers must stay
   attached to the exact byte or range finding. A generic warning at
   the panel level is not enough.
2. Mixed-direction inspectors inherit the same representation choices:
   `Copy raw bytes` and `Copy escaped text` must stay reachable where
   rendering could reorder or hide technical content.
3. Support/export packets over decode-recovery regions default to raw
   bytes where policy allows, include escaped text, and never present
   rendered text without the matching decode context.

## Fixture coverage

The fixture corpus under
[`/fixtures/editor/decode_recovery_cases/`](../../fixtures/editor/decode_recovery_cases/)
is the reviewable seed for this contract:

| Fixture | Required coverage |
|---|---|
| `mixed_encoding_import_preview.yaml` | Mixed encodings, low confidence, import preview defaulting to raw/escaped, save blocked. |
| `invalid_utf8_editor_blocked_save.yaml` | Undecodable bytes in editor, replacement projection, raw bytes preserved, blocked save. |
| `bom_eol_normalization_warning.yaml` | BOM and EOL changes, explicit normalization, acknowledgment, rewrite warning. |
| `diff_view_raw_rendered_divergence.yaml` | Diff view, raw/rendered divergence, suspicious and mixed-direction linkage. |
| `lossy_reencode_acknowledgment.yaml` | Lossy re-encode, explicit acknowledgment before rewrite, replacement consequence. |
| `replace_undecodable_spans_warning.yaml` | Explicit replacement of undecodable spans, warning rewrite, raw-byte evidence retained until commit. |
| `support_export_security_review_raw_bytes.yaml` | Support export and security review default to raw bytes with escaped companion representation. |

## Verification method

Until runtime implementation exists, verification is by design review,
schema parse, fixture parse, and schema validation of fixture records.
The expected local checks are:

```sh
python3 -m json.tool schemas/editor/decode_recovery_review.schema.json >/dev/null
python3 -m json.tool schemas/editor/text_representation_choice.schema.json >/dev/null
ruby -ryaml -e 'ARGV.each { |p| YAML.safe_load(File.read(p), permitted_classes: [], aliases: false) }' fixtures/editor/decode_recovery_cases/*.yaml
```

When a schema validator is available, each YAML fixture should validate
against `schemas/editor/decode_recovery_review.schema.json`.
