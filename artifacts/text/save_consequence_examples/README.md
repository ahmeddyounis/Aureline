# Save-consequence examples

Worked records paired with
[`/docs/verification/text_fidelity_packet.md`](../../../docs/verification/text_fidelity_packet.md)
and its sibling
[`/docs/verification/source_fidelity_and_undo_packet.md`](../../../docs/verification/source_fidelity_and_undo_packet.md).

These examples freeze one reviewable shape for the three honest
outcomes when Aureline cannot round-trip bytes exactly:
`preserve_untouched`, `rewrite_with_warning`, and
`block_pending_review`. They also cover the adjacent boundary cases
the text-fidelity spec explicitly calls out — protocol coordinate
translation, copy / paste representation choice, and support / export
raw-byte capture.

If this directory, the text-fidelity packet, and the save-fidelity
packet disagree, the packets and the underlying ADRs win and the
examples must update in the same change.

## Index

| File | `save_consequence_class` | `rewrite_class` | Purpose |
|---|---|---|---|
| [`exact_round_trip_preserved.json`](./exact_round_trip_preserved.json) | `preserve_untouched` | `targeted_content_patch` | proves a save that keeps every untouched byte verbatim |
| [`explicit_eol_conversion_warning.json`](./explicit_eol_conversion_warning.json) | `rewrite_with_warning` | `targeted_content_patch` | proves explicit newline conversion projects rewrite-with-warning |
| [`blocked_pending_decode_recovery.json`](./blocked_pending_decode_recovery.json) | `block_pending_review` | `blocked_no_write` | proves save refusal while decode recovery is unresolved |
| [`protocol_coordinate_adapter_translation.json`](./protocol_coordinate_adapter_translation.json) | `preserve_untouched` | `targeted_content_patch` | proves LSP / DAP coordinate translation at the boundary without silent drift |
| [`copy_paste_representation_choice.json`](./copy_paste_representation_choice.json) | `preserve_untouched` (no save boundary) | n/a | proves the three copy actions map cleanly to representation classes |
| [`support_export_raw_bytes_capture.json`](./support_export_raw_bytes_capture.json) | `preserve_untouched` (no save boundary) | n/a | proves support / export defaults to raw_bytes for decode-recovery regions |
