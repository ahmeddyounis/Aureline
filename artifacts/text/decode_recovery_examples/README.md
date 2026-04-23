# Decode-recovery examples

Worked records paired with
[`/docs/verification/text_fidelity_packet.md`](../../../docs/verification/text_fidelity_packet.md)
and the encoding / BOM / EOL corpus at
[`/fixtures/text/encoding_eol_corpus/manifest.yaml`](../../../fixtures/text/encoding_eol_corpus/manifest.yaml).

These examples are reviewable shape, not runtime transcripts. They
freeze how a decode-recovery event names its raw-byte posture, its
representation choice, its resolution path, and the expected save
consequence. Future buffer, VFS, inspector, and support-export lanes
should cite these ids instead of minting per-surface decode language.

If this directory, the encoding corpus, and the text-fidelity packet
disagree, the packet and the underlying ADRs win and the examples must
update in the same change.

## Index

| File | `decode_recovery_class` | `representation_class` | Purpose |
|---|---|---|---|
| [`invalid_utf8_replacement.json`](./invalid_utf8_replacement.json) | `invalid_byte_sequence_detected` | `mixed_with_replacement` | proves invalid UTF-8 runs are inspectable without corrupting the source bytes |
| [`unknown_encoding_fallback.json`](./unknown_encoding_fallback.json) | `unknown_encoding_fallback` | `raw_bytes` | proves ambiguous sniffs block edit and save until the user chooses |
| [`explicit_user_override.json`](./explicit_user_override.json) | `explicit_user_override` | `rendered` | proves user-chosen encoding emits a `decode_recovery_change` transaction |
| [`bom_correction_added.json`](./bom_correction_added.json) | `clean_decode` (BOM state changed) | `rendered` | proves adding a BOM is an explicit conversion and projects rewrite-with-warning |
| [`lossy_reencode_warning.json`](./lossy_reencode_warning.json) | `lossy_reencode_warning` | `escaped` | proves save with lossy re-encode requires explicit acknowledgement |
| [`binary_like_blocked.json`](./binary_like_blocked.json) | `binary_like_blocked` | `raw_bytes` | proves NUL-dense content opens in binary or large-file mode |
