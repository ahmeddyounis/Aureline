# Source-fidelity and undo-honesty verification seed

This packet makes save, rewrite-scope, and recovery claims auditable
before the runtime path hardens around hidden whole-file rewrites or
misleading reversal copy.

If this packet, the machine-readable corpus, and the supporting rewrite
class vocabulary disagree, the underlying ADRs and schemas still win,
but the packet and its companions must update in the same change so
review, support, and future automation read one story.

Companion artifacts:

- [`/fixtures/io/source_fidelity_corpus_manifest.yaml`](../../fixtures/io/source_fidelity_corpus_manifest.yaml)
  — machine-readable corpus rows and expected packet projections.
- [`/artifacts/io/save_rewrite_classes.yaml`](../../artifacts/io/save_rewrite_classes.yaml)
  — rewrite-class taxonomy and disclosure requirements.
- [`/artifacts/io/undo_recovery_examples/`](../../artifacts/io/undo_recovery_examples/)
  — worked save and mutation records for the required cases.
- [`/docs/adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md)
  — source-fidelity, undo-class, and recovery-journal rules.
- [`/docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md)
  — save-manifest, compare-before-write, and degraded save-mode rules.
- [`/docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md)
  — reversal-class vocabulary reused by mutation-journal consumers.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md` — `FR-EDIT-001`, `REL-IO-004`, and
  `EPIC-022`.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  `REL-FS-013`, `REL-MUT-014`, and the unified mutation-journal /
  undo-honesty rules.
- `.t2/docs/Aureline_Technical_Design_Document.md` —
  `TOOL-FMT-014`, staged save participants, and whole-file-rewrite
  disclosure.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — save/conflict copy,
  source-fidelity cards, and reversal-label wording.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` — save-fallback,
  portability-inspector, and source-fidelity template rows.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.source_fidelity_and_undo.editor_io_seed
evidence_id: evidence.editor_io.source_fidelity_and_undo_seed
title: Source-fidelity and undo-honesty verification seed
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
  claim_row_refs: []
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-22T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: commit:working_tree
  trigger_revision: source_fidelity_and_undo_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen save, source-fidelity, and mutation-lineage
    contracts. No implementation or platform-lab pass is claimed yet.
artifact_links:
  supporting_evidence_ids:
    - evidence.editor_io.save_rewrite_class_vocabulary
    - evidence.editor_io.source_fidelity_corpus
    - evidence.editor_io.undo_recovery_examples
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/io/source_fidelity_corpus_manifest.yaml
    - artifacts/io/undo_recovery_examples/
  archetype_refs: []
  source_anchor_refs:
    - docs/adr/0003-buffer-undo-large-file.md
    - docs/adr/0006-vfs-save-cache-identity.md
    - docs/workspace/mutation_lineage_model.md
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This packet freezes one reviewer-facing field set for source fidelity,
rewrite disclosure, and recovery honesty. It does not claim the editor,
VFS, formatter, or merge implementation is complete; it claims only that
the required evidence shape, disclosure copy, and corpus rows now exist
and can be cited without per-feature save truth.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:source_fidelity.field_set` | `REL-IO-004`, `FR-EDIT-001` | `seed_only` | `internal` | `evidence.editor_io.source_fidelity_corpus` | Freezes the packet-level field names and value sets used to explain save fidelity. |
| `packet_row:source_fidelity.whole_file_rewrite_disclosure` | `TOOL-FMT-014`, `REL-IO-004` | `seed_only` | `internal` | `evidence.editor_io.save_rewrite_class_vocabulary` | Whole-file rewrite and fallback copy is now controlled rather than ad hoc. |
| `packet_row:undo.recovery_label_honesty` | `REL-MUT-014` | `seed_only` | `internal` | `evidence.editor_io.undo_recovery_examples` | Recovery wording is pinned to machine-readable classes and may not overclaim exact undo. |
| `packet_row:corpus.required_cases` | `REL-IO-004`, `REL-FS-013`, `REL-MUT-014` | `seed_only` | `internal` | `evidence.editor_io.source_fidelity_corpus` | Required save, conflict, generated, and blocked cases now have stable ids and expected projections. |

## What this seed freezes

- One packet-level field set that support bundles, save inspectors,
  mutation summaries, and future apply/revert review sheets can all
  project from the same records.
- One rewrite-class taxonomy for `no write`, `targeted patch`,
  `whole-file rewrite`, `whole-file rewrite fallback`, `merge result`,
  `generator regeneration`, and `blocked no-write`.
- One recovery-label map that reuses mutation-journal reversal classes
  where a mutation exists and adds `no_state_change` for save attempts
  that committed no mutation.
- One stable corpus row set for no-op save, line-ending preservation,
  encoding-preserving edit, permission-preserving save, whole-file
  rewrite fallback, external-change merge/conflict, generated-file
  regeneration, and degraded or unsupported save.

## Packet field set

Use these packet-level fields whenever Aureline explains save fidelity or
recovery honesty. The packet may be projected from a `save_manifest`, a
`mutation_journal_entry`, a `mutation_group_record`, or a joined view of
those records.

| Packet field | Meaning | Allowed values / source |
|---|---|---|
| `encoding_state` | Whether the durable write kept the opened encoding. | `preserved`, `explicit_conversion`, `decode_recovery_override`, `unknown_or_degraded` |
| `bom_state` | Whether the BOM state was preserved. | `preserved`, `added_explicitly`, `removed_explicitly`, `unknown_or_degraded` |
| `newline_mode_state` | Whether dominant newline mode stayed stable. | `preserved`, `converted_explicitly`, `mixed_input_preserved`, `unknown_or_degraded` |
| `final_newline_state` | Whether final-newline posture stayed stable. | `preserved`, `added_explicitly`, `removed_explicitly`, `unknown_or_degraded` |
| `permission_state` | Whether mode or permission guarantees stayed stable. | `preserved`, `explicit_permission_change`, `write_blocked`, `metadata_guarantee_degraded` |
| `conflict_state` | Why the save did or did not commit cleanly. | `clean`, `no_write_needed`, `merge_required`, `merge_committed`, `generated_boundary_blocked`, `unsupported_or_degraded` |
| `rewrite_class` | How much of the durable target Aureline rewrote. | Values from [`save_rewrite_classes.yaml`](../../artifacts/io/save_rewrite_classes.yaml) |
| `recovery_class` | The strongest truthful recovery claim for the save outcome. | `exact_undo`, `compensating_undo`, `regenerate_or_recompute`, `restore_from_checkpoint`, `audit_only`, `no_state_change` |

Rules:

1. Packet fields are an explanatory layer, not a second mutation schema.
   When a mutation-journal record exists, `recovery_class` MUST align
   with its `reversal_class`.
2. `no_state_change` is reserved for save attempts that issued no
   durable mutation or for committed no-op saves. It is never written as
   a mutation-journal `reversal_class`; it is a packet-side explanation
   that the honest undo affordance is none.
3. `rewrite_class` and `conflict_state` are orthogonal. A save can be
   `blocked_no_write` because of `merge_required`, or it can be
   `generator_regeneration_write` with `conflict_state = clean`.

## Disclosure labels

These labels are controlled vocabulary. Product chrome, support
packets, and review sheets may add context around them, but they may
not soften or strengthen the underlying claim.

### Source-fidelity labels

| Situation | Required label |
|---|---|
| Encoding, newline mode, BOM state, and final newline were preserved on the committed path | `Preserved source fidelity` |
| Compact detail summary is shown | `UTF-8 · LF · final newline kept` or the equivalent exact encoding/newline detail for the file |

### Rewrite-scope labels

| `rewrite_class` | Required label | When it must appear |
|---|---|---|
| `whole_file_rewrite_declared` | `Whole-file rewrite` | Any save or save-participant path that rewrote the full file on its supported path |
| `whole_file_rewrite_fallback` | `Whole-file rewrite fallback` | Any save or save-participant path that widened from targeted patching to a full-file rewrite because the narrower path was not safe |
| `merge_resolution_write` | `Merge result after external change` | Any durable write produced by a reviewed merge or choose flow after external drift |
| `generator_regeneration_write` | `Generated output regenerated` | Any durable write where canonical inputs or a generator, not direct text inversion, are the authority |
| `blocked_no_write` | `No durable write` | Any save attempt that stopped before a durable write landed |

### Recovery-honesty labels

| `recovery_class` | Required label | Notes |
|---|---|---|
| `exact_undo` | `Undo exactly` | Use `Undo one grouped change` when one exact grouped mutation is the visible action. |
| `compensating_undo` | `Revert with a compensating action` | Never shorten to plain `Undo`. |
| `regenerate_or_recompute` | `Restore by regeneration` | The packet or UI should also name the source or generator where practical. |
| `restore_from_checkpoint` | `Restore checkpoint` | Recovery is via a checkpoint or journal, not an inverse edit. |
| `audit_only` | `No state change to undo` | Used when a record exists only for auditability or external effects. |
| `no_state_change` | `No state change to undo` | Used when save review or blocking preserved local state but committed no mutation. |

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.editor_io.save_rewrite_class_vocabulary` | `verification_corpus` | Freezes the rewrite-class vocabulary the packet projects | current with packet revision 1 | [`artifacts/io/save_rewrite_classes.yaml`](../../artifacts/io/save_rewrite_classes.yaml) |
| `evidence.editor_io.source_fidelity_corpus` | `verification_corpus` | Names the required cases and their expected packet projections | current with packet revision 1 | [`fixtures/io/source_fidelity_corpus_manifest.yaml`](../../fixtures/io/source_fidelity_corpus_manifest.yaml) |
| `evidence.editor_io.undo_recovery_examples` | `verification_corpus` | Worked records for exact, compensating, regenerate, checkpoint, and no-change outcomes | current with packet revision 1 | [`artifacts/io/undo_recovery_examples/`](../../artifacts/io/undo_recovery_examples/) |

## Corpus coverage

The machine-readable corpus is the canonical row set. The table below is
the reviewer-facing summary.

| Corpus row id | Primary case | Expected rewrite class | Expected recovery class | Primary artifact |
|---|---|---|---|---|
| `corpus.io.no_op_save` | save invoked with no durable diff | `no_write_needed` | `no_state_change` | [`no_op_save.json`](../../artifacts/io/undo_recovery_examples/no_op_save.json) |
| `corpus.io.line_ending_preserved_save` | one-line edit with CRLF preserved | `targeted_content_patch` | `exact_undo` | [`line_ending_preserved_save.json`](../../artifacts/io/undo_recovery_examples/line_ending_preserved_save.json) |
| `corpus.io.encoding_preserving_edit_save` | edit in non-default encoding with encoding/BOM preserved | `targeted_content_patch` | `exact_undo` | [`encoding_preserving_edit_save.json`](../../artifacts/io/undo_recovery_examples/encoding_preserving_edit_save.json) |
| `corpus.io.permission_preserving_save` | executable file edit that keeps mode and write target | `targeted_content_patch` | `exact_undo` | [`permission_preserving_save.json`](../../artifacts/io/undo_recovery_examples/permission_preserving_save.json) |
| `corpus.io.whole_file_rewrite_fallback` | save-participant path widened to a full-file rewrite | `whole_file_rewrite_fallback` | `compensating_undo` | [`whole_file_rewrite_fallback.json`](../../artifacts/io/undo_recovery_examples/whole_file_rewrite_fallback.json) |
| `corpus.io.external_change_merge_conflict` | compare-before-write detects drift and opens merge review | `blocked_no_write` | `no_state_change` | [`external_change_merge_conflict.json`](../../artifacts/io/undo_recovery_examples/external_change_merge_conflict.json) |
| `corpus.io.generated_file_regeneration` | generated artifact refreshed from canonical source | `generator_regeneration_write` | `regenerate_or_recompute` | [`generated_file_regeneration.json`](../../artifacts/io/undo_recovery_examples/generated_file_regeneration.json) |
| `corpus.io.degraded_unsupported_save` | save blocked or narrowed because the durable path is unsupported | `blocked_no_write` | `no_state_change` | [`degraded_unsupported_save.json`](../../artifacts/io/undo_recovery_examples/degraded_unsupported_save.json) |

Supplemental recovery reference:

| Corpus row id | Primary case | Expected recovery class | Primary artifact |
|---|---|---|---|
| `corpus.io.checkpoint_restore_decode_recovery` | decode recovery preserves raw bytes and resolves by checkpoint | `restore_from_checkpoint` | [`checkpoint_restore_decode_recovery.json`](../../artifacts/io/undo_recovery_examples/checkpoint_restore_decode_recovery.json) |

## Verification method

- **Verification classes used:** design review, corpus freeze, example
  record review
- **Procedure summary:** define the packet field set, freeze rewrite
  disclosure labels, project the required cases into stable corpus rows,
  and pair each row with a worked record that names the expected save
  or mutation truth explicitly
- **Automation refs:** no dedicated validator yet; this seed relies on
  the existing JSON / YAML parse checks and future packet-level
  validation work

## Known gaps and waivers

- **Waiver refs:** `none`
- **Known-limit refs:** `none`
- **Migration-packet refs:** `none`
- **Explicit gaps:** the packet does not yet claim live implementation
  coverage, platform parity, formatter-specific safety reports, or
  merge-tool UX completeness
- **Explicit gaps:** the current `save_manifest` schema does not embed
  the packet projection directly; the examples carry packet metadata in
  fixture blocks until a future schema or exporter joins the fields

## Reviewer signoff

- **Reviewer / forum:** `not_yet_reviewed`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-22`
- **Reviewed claim rows:** `packet_row:source_fidelity.field_set`,
  `packet_row:source_fidelity.whole_file_rewrite_disclosure`,
  `packet_row:undo.recovery_label_honesty`,
  `packet_row:corpus.required_cases`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `corpus_or_fixture_revision_changed`
- **Expected freshness window:** refresh within `P30D` or whenever the
  rewrite vocabulary, save-manifest shape, mutation-journal shape, or
  corpus rows change
- **Next packet family to update with the same evidence ids:** release
  evidence and support-export packets covering editor / IO truth
