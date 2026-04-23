# Source-fidelity and undo-recovery examples

Worked save and mutation records paired with
[`/docs/verification/source_fidelity_and_undo_packet.md`](../../../docs/verification/source_fidelity_and_undo_packet.md)
and
[`/fixtures/io/source_fidelity_corpus_manifest.yaml`](../../../fixtures/io/source_fidelity_corpus_manifest.yaml).

These examples are intentionally small and reviewable. They are not a
runtime transcript; they are the seed evidence shape future support
bundles, save inspectors, and apply/revert proofs should reuse.

If this directory, the corpus manifest, and the packet disagree, the
packet and the underlying ADRs win and the examples must update in the
same change.

## Index

| File | Record kind | Main rewrite class | Main recovery class | Purpose |
|---|---|---|---|---|
| [`no_op_save.json`](./no_op_save.json) | `save_manifest` | `no_write_needed` | `no_state_change` | proves that a clean no-op save does not overclaim a durable rewrite or an undoable mutation |
| [`line_ending_preserved_save.json`](./line_ending_preserved_save.json) | `save_manifest` | `targeted_content_patch` | `exact_undo` | proves CRLF preservation on ordinary save |
| [`encoding_preserving_edit_save.json`](./encoding_preserving_edit_save.json) | `mutation_journal_entry` | `targeted_content_patch` | `exact_undo` | proves non-default encoding and BOM survive an edit/save path |
| [`permission_preserving_save.json`](./permission_preserving_save.json) | `save_manifest` | `targeted_content_patch` | `exact_undo` | proves executable-mode preservation |
| [`whole_file_rewrite_fallback.json`](./whole_file_rewrite_fallback.json) | `mutation_group_record` | `whole_file_rewrite_fallback` | `compensating_undo` | proves explicit whole-file-rewrite disclosure and weaker recovery copy |
| [`external_change_merge_conflict.json`](./external_change_merge_conflict.json) | `save_manifest` | `blocked_no_write` | `no_state_change` | proves compare-before-write conflict honesty |
| [`generated_file_regeneration.json`](./generated_file_regeneration.json) | `mutation_journal_entry` | `generator_regeneration_write` | `regenerate_or_recompute` | proves generated-file save truth points to regeneration |
| [`degraded_unsupported_save.json`](./degraded_unsupported_save.json) | `save_manifest` | `blocked_no_write` | `no_state_change` | proves unsupported save paths stop before a durable write |
| [`checkpoint_restore_decode_recovery.json`](./checkpoint_restore_decode_recovery.json) | `mutation_journal_entry` | `no_write_needed` | `restore_from_checkpoint` | proves raw-byte recovery is checkpoint-based, not inverse-edit based |
