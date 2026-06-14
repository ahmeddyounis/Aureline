# M5 keyboard-first modal parity, clipboard-drop safety, grouped-history honesty, and orientation-aid continuity certification

This contract is the **capstone release gate** for Aureline's switching-wedge
promise on Milestone 5. Where the frozen
[keyboard-continuity matrix](./freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md)
pins *which* canonical interaction vocabulary each claimed M5 surface resolves to,
and the per-axis consumers discharge each behavior — mode strips and leader
sequences, clipboard contracts, drag/drop transfer safety, grouped undo / history /
reopen, macro-replay review, and orientation aids — this certification decides
whether each claimed M5 **row** actually carries *current* proof for every
interaction-safety dimension it claims. A new notebook, data/API, preview, docs,
review, runtime, or companion-adjacent surface can only reinforce the switching
wedge when users can trust its modal / keymap state, copy / export semantics,
drag/drop verbs, undo / history lineage, reopen / recover paths, and orientation
aids without falling back to mouse-only or one-off surface behavior.

The canonical packet is built by
`aureline_shell::certify_keyboard_first_modal_parity_clipboard_drop_safety_grouped_history_honesty_and_orie`.

## What each row certifies

A `CertifiedSurfaceRow` ties a durable `KeyboardSurfaceSubject` (keyed by a
`KeyboardSurfaceKind`, a `SurfaceOriginClass`, and a non-display fingerprint, so a
provider-linked surface never reads as a local one) to a list of
`DimensionCertification` rows over the `ParityDimension` vocabulary:

| Dimension | Required core | What it certifies |
| --- | --- | --- |
| `modal_keyboard_parity` | yes | modal mode state, leader-key sequences, keyboard completeness |
| `clipboard_drop_safety` | yes | plain-text-preserving copy and disclosed drag/drop verbs / scope |
| `grouped_history_continuity` | yes | distinct exact / compensating / checkpoint undo classes and reopen / recover continuity |
| `orientation_aid_continuity` | yes | multi-cursor / fold / breadcrumb / minimap / overview aids that degrade honestly |
| `macro_replay_safety` | no | macro-replay review / downgrade for run-capable or cross-file replays |

Each `DimensionCertification` is **evidence-bound, not asserted**: it names an
`AxisProofCurrency` and, unless the proof is missing, a reopenable `proof_ref`
keyed by a non-display fingerprint, so certification review can reopen the same
mode-strip / clipboard / drop / history / orientation evidence object that backs
the grade.

## Auto-narrowing

A row **auto-narrows** below its claim (`CertifiedSurfaceRow::needs_narrow`)
whenever a required-core dimension is uncertified or any certified dimension lacks
current proof — stale, missing, requires-review, or imported proof standing in for
a local claim. A narrowed row must carry an effective `ContinuityParityGrade`
ranked strictly below its claim, a recorded `ParityDowngradeTrigger`, and a precise
narrowed label. A generic non-answer (`unavailable`, `error`, `downgraded`,
`unverified`, …) is rejected. A local row needs locally verified or cached proof;
a provider-linked / imported row needs current imported proof, and that imported
proof never backs a local claim — so an imported row can never read as a live local
result.

## Guardrails

`InteractionParityCertificationPacket::validate` refuses a packet that:

- silently approximates an unsupported modal sequence
  (`modal_sequence_approximated`);
- lets rich text become the only copy representation (`plain_text_copy_lost`);
- hides a drag/drop verb or its insertion / window scope (`drag_drop_verb_hidden`);
- flattens the exact / compensating / checkpoint undo classes into one opaque
  history label (`undo_classes_flattened`);
- drops orientation truth rather than degrading honestly
  (`orientation_truth_dropped`);
- lets a claimed row keep its grade despite uncurrent proof
  (`row_not_narrowed_on_uncurrent_proof`);
- lets a provider-linked surface read as a locally verified result
  (`imported_reads_as_local`).

The packet also requires coverage: every claimed surface kind is represented, every
required-core dimension is certified, at least one row demonstrates auto-narrowing,
at least one row carries current proof, and at least one imported / provider-linked
row is present.

## Consumer projection

Product, help / migration, accessibility, support, and release-control surfaces
ingest this single certification result rather than cloning switching-wedge
behavior text by hand, and narrowed rows are visibly labeled below their claim in
every surface.

## Boundary safety

Raw provider payloads, file contents, raw clipboard / drag payload bodies,
credentials, and absolute private paths never cross this boundary. The packet
carries only typed class tokens, booleans, opaque / relative ids, fingerprint
digests, and redaction-aware reviewable labels.

## Artifacts

- Schema:
  `schemas/interaction/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.schema.json`
- Support export:
  `artifacts/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie/support_export.json`
- Markdown summary:
  `artifacts/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie.md`
- Protected fixtures:
  `fixtures/interaction/m5/certify-keyboard-first-modal-parity-clipboard-drop-safety-grouped-history-honesty-and-orie/`

Regenerate the checked artifacts with
`cargo run -p aureline-shell --example dump_certify_interaction_parity [support|summary|fixture]`.
