# M5 Unsafe-Fix-Blocked-Note / Approved-Repair-Guidance Primitive

Status: Stable · Milestone: M5 · Lane: B106 support-intake / escalation components

This primitive implements two reusable M5 support components — the **unsafe-fix blocked
note** and the **approved-repair guidance card** — so support guidance about a destructive
repair stays bounded, attributable, and state-preserving instead of reading like normative
folklore. It narrows the `unsafe_fix_blocked_note` family frozen in the
[support-intake / escalation component matrix](m5_support_intake_escalation_component_matrix.md)
into two resolvers plus a parity matrix, and reuses the frozen block-reason, approved-repair,
scenario, finding, redaction, and case-disposition vocabulary verbatim.

- Rust module:
  `crates/aureline-support/src/implement_unsafe_fix_blocked_notes_and_approved_repair_guidance_with_blocked_action_block_reason_safer_repair_blast_radius_and_rollback_evidence_preservation_truth_across_claimed_m5_doctor_and_support_surfaces/`
- Boundary schema: `schemas/ui/m5-support-unsafe-fix-blocked-note.schema.json`
- Checked support export:
  `artifacts/release/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive-proof/support_export.json`
- Machine-readable matrix:
  `artifacts/release/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive-proof/matrix.csv`
- Design report:
  `artifacts/design/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive.md`
- Narrowed fixtures:
  `fixtures/ui/m5-support-unsafe-fix-blocked-note-approved-repair-guidance-primitive/`
- Headless emitter:
  `aureline_support_unsafe_fix_blocked_note_approved_repair_guidance_primitive`

## Unsafe-fix blocked note

`resolve_unsafe_fix_blocked_note` takes one note's id, its specific blocked action label, its
scenario family, its related finding families and opaque evidence ids, the unsafe-fix block
reason, the recommended safer repair, the redaction posture, the build / profile identity, and
the case disposition, and derives a note posture in a fixed order:

1. `no_safe_alternative` — the recommended repair is `no_safe_repair`; only evidence
   preservation and a local review remain.
2. `irreversible_blocked` — the blocked fix is irreversible; a reviewed safer repair replaces
   it.
3. `approval_required_blocked` — the fix is blocked pending an explicit approval.
4. `policy_blocked` — the fix is blocked by policy.
5. `evidence_or_scope_blocked` — the fix is blocked pending more evidence or a supported scope
   (insufficient evidence, out-of-scope repair, or unsupported scenario).

The note always offers **reveal-reason**, **preserve-evidence**, **dismiss**, and **export**
actions, and offers **view-safer-repair** whenever a safe repair is actually recommended. It
always keeps `rollback_preserved`, `evidence_preserved`, and
`distinct_from_reviewed_transaction` true: the blocked destructive action is never applied and
is never presented as equivalent to a reviewed repair transaction.

## Approved-repair guidance

`resolve_approved_repair_guidance` takes one repair's id, its approved repair class, its blast
radius, its changed and unchanged classes, and its reversibility, and derives a guidance
posture in a fixed order:

1. `no_repair_available` — the repair class is `no_safe_repair`.
2. `irreversible_repair` — the repair is irreversible.
3. `partially_reversible_repair` — the repair is only partially reversible.
4. `broad_reversible_repair` — the repair is reversible but profile- or device-wide.
5. `scoped_reviewed_repair` — the repair is a scoped, reviewed, reversible transaction.

The card always offers **reveal-blast-radius**, **view-changed-classes**, **decline**, and
**export** actions, and offers **request-approval** whenever the repair is not a fully reviewed
reversible transaction. It always keeps `decline_keeps_evidence` and
`changed_and_unchanged_explicit` true: a user can always decline while keeping the evidence,
and the changed / unchanged surface is never collapsed into one opaque blob.

## Parity matrix and invariants

One row per claimed Doctor / support consumer (Doctor repair review, support-center unsafe-fix
desk, recovery-center repair guidance, headless / CLI repair review, support repair export)
binds the two components to the shared anatomy, vocabulary, postures, actions, and export
fields, with worked resolution cases proving both resolvers across the full block-reason,
approved-repair, blast-radius, reversibility, and change-class vocabulary. Every row honours
four hard invariants: it never masks the block reason or the recommended safer repair, never
presents a destructive reset as equivalent to a reviewed repair transaction, never drops the
rollback or evidence posture, and never collapses the guidance into one opaque blob.

## Acceptance criteria

- **Destructive reset suggestions no longer appear equivalent to reviewed repair
  transactions.** The blocked note keeps `distinct_from_reviewed_transaction` true and derives
  an `irreversible_blocked` posture for a destructive fix, while the guidance card keeps a
  reviewed reversible transaction (`is_reviewed_transaction`) distinct from an irreversible
  repair.
- **Users can see why a recommended repair is safer and what evidence remains if they decline
  it.** The note names the recommended safer repair and keeps `evidence_preserved` true; the
  guidance card names the blast radius, the changed and unchanged classes, and keeps
  `decline_keeps_evidence` true.
