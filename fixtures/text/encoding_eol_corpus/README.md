# Encoding / BOM / EOL corpus

Reviewer-facing cases paired with
[`/docs/verification/text_fidelity_packet.md`](../../../docs/verification/text_fidelity_packet.md)
and its sibling
[`/docs/verification/source_fidelity_and_undo_packet.md`](../../../docs/verification/source_fidelity_and_undo_packet.md).

Each row in [`manifest.yaml`](./manifest.yaml) names the raw bytes,
expected open-time detection, expected save projection, and optional
decode-recovery posture. Byte sequences are expressed in hexadecimal
so the corpus does not depend on checked-in binary artifacts; future
buffer or VFS work may materialize physical files from these rows.

If this directory, the text-fidelity packet, and the save-fidelity
packet disagree, the packets and the underlying ADRs win and the
corpus must update in the same change.

## Index

| Row id | Case class | Primary case |
|---|---|---|
| `corpus.text.encoding.utf8_lf_no_bom_no_finalnl` | clean_detect_and_preserve_case | baseline UTF-8 LF without BOM or final newline |
| `corpus.text.encoding.utf8_bom_crlf_finalnl` | clean_detect_and_preserve_case | UTF-8 BOM with CRLF and a final newline |
| `corpus.text.encoding.utf16le_bom_lf` | clean_detect_and_preserve_case | UTF-16LE BOM with LF line endings |
| `corpus.text.encoding.utf16be_bom_crlf` | clean_detect_and_preserve_case | UTF-16BE BOM with CRLF line endings |
| `corpus.text.encoding.utf32le_bom_lf` | clean_detect_and_preserve_case | UTF-32LE BOM with LF line endings |
| `corpus.text.encoding.mixed_crlf_lf_dominant_lf` | clean_detect_and_preserve_case | mixed-newline input with LF dominant |
| `corpus.text.encoding.cr_only_classic` | clean_detect_and_preserve_case | lone-CR classic-Mac line endings |
| `corpus.text.encoding.legacy_8bit_fallback` | clean_detect_and_preserve_case | non-UTF-8 bytes fall through to platform default |
| `corpus.text.encoding.invalid_utf8_bytes` | decode_recovery_case | truncated multi-byte UTF-8 sequence |
| `corpus.text.encoding.unknown_encoding_ambiguous` | decode_recovery_case | sniff cannot choose between encodings |
| `corpus.text.encoding.binary_like_nul_dense` | decode_recovery_case | NUL-dense content opens in binary-like mode |
| `corpus.text.encoding.bom_correction_added` | explicit_conversion_case | user explicitly adds a BOM to a BOM-less file |
