# M5 missing-surface placeholders — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-missing-surface-placeholders.json`. The full contract lives in
`docs/workspace/m5/m5-missing-surface-placeholders.md`; the typed model lives in the
`aureline-workspace` crate (`m5_missing_surface_placeholders`).

This packet makes **layout honesty first-class** when a restored pane cannot hydrate: one placeholder
card per missing surface keeps the pane role, slot, last-known provenance, and recovery actions
visible, so a missing extension, feature pack, remote target, or backing service never silently
deletes a pane, loses a tab, or substitutes a misleading empty state. It reuses the
exact/compatible/layout-only/manual-review vocabulary and the dependency/schema/topology/freshness
conditions from the serialization-and-restore matrix, and the re-entry-surface vocabulary from the
restore-provenance packet, rather than redefining them.

## The placeholders (as of 2026-06-16)

| Pane role | Re-entry surface | Missing | Behavior | Published fidelity | Recovery next step |
| --- | --- | --- | --- | --- | --- |
| Preview | desktop restore | extension | slot preserved | **layout-only** | relocate dependency (install) |
| Notebook | portable-state import | feature pack | slot preserved | **layout-only** | relocate dependency (install) |
| Query console | crash recovery | remote target | reopen as context | **layout-only** | relocate dependency (reconnect) |
| Incident workspace | companion re-entry | backing service | slot preserved | **manual review** | manual review (retry) |
| Profiler | support-packet replay | remote target | slot preserved | **manual review** | manual review (reconnect) |

All four missing-dependency classes are exercised; three placeholders publish layout-only and two are
held for manual review. No placeholder publishes an exact restore — a missing surface always narrows
below a full restore.

## What the gate guarantees

- **A missing surface is never silently deleted.** The substitution behavior must preserve the slot
  (`placeholder_slot_preserved` or `reopen_as_context`); `silent_delete` is reject-only.
- **A missing surface can never publish an exact restore.** The dependency ceiling caps a
  partial-missing dependency at a slot-preserving layout-only restore and a missing dependency root at
  manual review.
- **Published fidelity is the weakest ceiling** of the declared fidelity and the dependency / schema /
  topology / evidence-freshness conditions; the downgrade reasons always include the missing
  dependency, and the recovery path is recomputed.
- **The pane role, slot, and provenance are preserved.** Each placeholder keeps the original pane role
  and stable pane-tree slot, and carries a complete last-known provenance, so the slot never reads as
  a never-populated empty tab.
- **Every placeholder names a concrete next step.** Open-details is always offered, and the recovery
  action the missing-dependency class calls for — install, reconnect, or retry — is preserved.
- **Keyboard focus and narration stay sensible.** Each placeholder slot is keyboard-reachable and its
  narration announces the role, slot, missing reason, and recovery; every affordance is
  keyboard-complete, screen-reader-labelled, and scoped to the one slot.
- **The parity surfaces name the classes and counts.** Exported diagnostics, the support packet, the
  compare/export summary, and the companion handoff each bind to this packet and preserve the
  missing-dependency-class and pane-role labels, naming the affected-surface counts.

The record is metadata only: every placeholder excludes secrets, live authority, machine-local
anchors, and raw provider payloads.

Layout-only and manual-review scenarios across all four missing-dependency classes are exercised as
fixtures under `fixtures/workspace/m5/m5-missing-surface-placeholders/`; the fail-closed rejections —
including a silent layout delete, an exact restore published for a missing surface, and a placeholder
with no missing dependency — are exercised as synthetic gate drills in the crate's
`m5_missing_surface_placeholders` unit tests.
