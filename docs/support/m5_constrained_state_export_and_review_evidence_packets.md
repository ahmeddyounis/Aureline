# M5 Constrained-State Export and Review-Evidence Packets

Task: **M05-1263** — batch **B150** (constrained-file-state, canonical-source relation, and write-target-review
truth across claimed M5 editor, review, save, AI, repair, and export surfaces).

This lane is the **support / export and review-evidence packet** lane over the six constrained-current-object classes
frozen in the [constrained-file-state matrix](../../artifacts/program/m5-constrained-file-state-matrix.md). Where the
state-descriptor, badge-group, canonical-source-relation, write-review-sheet, cross-actor-gate, and drill-corpus lanes
make one honest constrained-object *loop* real inside the product, this lane keeps that loop **explainable once it
leaves the live UI**: a support bundle, a review / export packet, a piece of local-history / restore evidence, or a
docs / help example each preserves the constrained-state class, the canonical source-of-truth relation, the exact
write-target decision, and the chosen reviewed fallback path.

## What it preserves

Six seeded entry families — one per constrained-object class — are preserved across the four packet channels:

| Channel | Token |
| --- | --- |
| Support bundle | `support_bundle` |
| Review / export packet | `review_export_packet` |
| Local-history / restore evidence | `local_history_restore_evidence` |
| Docs / help example | `docs_help_example` |

Every binding preserves, in both **human-readable** (a plain-language line intelligible without the live UI) and
**machine-readable** (a structured record) form:

- the constrained-state class and its blocked-write reason (a pure function of the class);
- the canonical source-of-truth relation and the exact write-target ref;
- the chosen reviewed fallback path the gate offered, and the **resolved decision** — whether the operator
  duplicated, detached, overlaid, requested approval, regenerated, or **cancelled**;
- the required write disposition and checkpoint / undo class;
- the **preserved-versus-lost** record naming what was retained, what was lost, and the sync / regenerate path.

The chosen fallback path is keyed to the object class through the shared pure functions:

- `read_only` / `captured_snapshot` → **duplicate to an editable copy** (`read_only_blocked`)
- `generated` → **regenerate with preview** (`regenerate_only`)
- `policy_locked` → **request approval** (`approval_gated`)
- `managed` → **detach from the managed source** (`detach_required`)
- `projection` → **create an overlay patch** (`detach_required`)

## Acceptance criteria

1. **At least one support packet and one review / export packet, both forms.** The corpus covers all four channels
   including at least one `support_bundle` and one `review_export_packet`, and every binding preserves both a
   human-readable line and a machine-readable record whose tokens mirror the typed decisions.
2. **Intelligible without the live UI, never flattened.** Each binding carries the controlled constrained-state
   grammar whose specific state-class label the human-readable line must name, so a packet that collapses a
   `generated`, `managed`, `projection`, `policy_locked`, or `captured_snapshot` object into an undifferentiated
   "read only" is mechanically rejected.
3. **Redacted packets stay honest.** Every binding carries a redaction record; a `redacted_keep_state_class_and_fallback`
   binding always names the omission reason and keeps the state class and chosen fallback decision preserved, so
   redaction-aware export never hides that the object was constrained.

## Guardrails (must stay false)

- `flattens_constrained_state_into_generic_read_only_language`
- `drops_omission_reason_when_redacted`
- `lets_one_constrained_state_class_hide_another`
- `silently_falls_back_to_lossy_direct_write`
- `gives_ai_automation_import_or_repair_a_hidden_bypass`
- `leaves_canonical_source_or_exact_write_target_unstated`
- `presents_as_directly_writable_or_hides_recovery_path`

## Artifacts

- Boundary schema: [`schemas/program/m5-constrained-state-export-and-review-evidence-packets.schema.json`](../../schemas/program/m5-constrained-state-export-and-review-evidence-packets.schema.json)
- Support export: [`artifacts/support/m5-constrained-state-evidence/support_export.json`](../../artifacts/support/m5-constrained-state-evidence/support_export.json)
- Matrix CSV: [`artifacts/support/m5-constrained-state-evidence/matrix.csv`](../../artifacts/support/m5-constrained-state-evidence/matrix.csv)
- Markdown summary: [`artifacts/support/m5-constrained-state-evidence/summary.md`](../../artifacts/support/m5-constrained-state-evidence/summary.md)
- Health dashboard: [`dashboards/m5-constrained-state-evidence-health.json`](../../dashboards/m5-constrained-state-evidence-health.json)
- Narrowed fixtures: [`fixtures/editor/m5-constrained-state-evidence/`](../../fixtures/editor/m5-constrained-state-evidence/)

All artifacts are minted from truth by the headless emitter:

```text
cargo run -p aureline-ui --example dump_m5_constrained_state_export_and_review_evidence_packets -- support-export
cargo run -p aureline-ui --example dump_m5_constrained_state_export_and_review_evidence_packets -- csv
cargo run -p aureline-ui --example dump_m5_constrained_state_export_and_review_evidence_packets -- report
cargo run -p aureline-ui --example dump_m5_constrained_state_export_and_review_evidence_packets -- dashboard
cargo run -p aureline-ui --example dump_m5_constrained_state_export_and_review_evidence_packets -- fixture-redaction-narrowed
cargo run -p aureline-ui --example dump_m5_constrained_state_export_and_review_evidence_packets -- fixture-cancelled-decision-narrowed
cargo run -p aureline-ui --example dump_m5_constrained_state_export_and_review_evidence_packets -- validate
```
