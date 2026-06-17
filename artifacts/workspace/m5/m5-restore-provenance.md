# M5 restore provenance — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-restore-provenance.json`. The full contract lives in
`docs/workspace/m5/m5-restore-fidelity.md`; the typed model lives in the `aureline-workspace` crate
(`m5_restore_provenance`).

This packet makes restore fidelity **first-class across every M5 re-entry flow**: one card per
re-entry surface discloses the source, producer/build provenance, restored schema version, redaction
class, and the resulting fidelity, using the **same** exact/compatible/layout-only/manual-review
vocabulary the serialization-and-restore matrix defines. It reuses, rather than redefines, that
vocabulary so restore meaning cannot fork by surface.

## The cards (as of 2026-06-16)

| Re-entry surface | Source | Restored | Published fidelity | Recovery next step |
| --- | --- | --- | --- | --- |
| Desktop restore | auto checkpoint | workspace authority checkpoint | **exact** | none |
| Portable-state import | import | portable state package | **compatible** | restore compatibly |
| Crash recovery | auto checkpoint | window topology snapshot | **manual review** | manual review |
| Support-packet replay | backup | compare / export summary | **compatible** | restore compatibly |
| Companion / browser re-entry | browser/companion handoff | portable state package | **layout-only** | reopen as context |

One card publishes exact, two compatible, one layout-only, and one manual-review — so all four labels
are exercised across the surfaces. Two cards were narrowed below the fidelity they declared.

## What the gate guarantees

- **Published fidelity is the weakest ceiling** of the declared fidelity, the source ceiling, and the
  schema / dependency / topology / evidence-freshness conditions. A schema drift, missing dependency,
  changed topology, or stale evidence narrows the restore automatically.
- **A browser/companion handoff can never imply a full restore.** Its source ceiling is layout-only,
  so even with otherwise-clean conditions it is published as a contextual reopen, not exact
  continuity.
- **An exact card is genuinely clean** — pristine conditions, a non-contextual source, no downgrade
  reason, no recovery step — so a downgraded or placeholder-heavy restore is never shown as exact.
- **A missing dependency never silently deletes layout.** The slot is preserved as a placeholder or
  reopened as context; `silent_delete` is reject-only.
- **Every card offers open-details**, and a narrowed card preserves the compare and
  recovery-next-step actions. Every affordance is keyboard-complete, screen-reader-labelled, and
  scoped to the one restore event.
- **The parity surfaces carry the same record.** Exported diagnostics, the support packet, the
  crash-recovery packet, and the companion handoff each bind to this packet and preserve its source
  and fidelity labels verbatim, narrowing with it rather than inventing a weaker summary.

The record is metadata only: every card excludes secrets, live authority, machine-local anchors, and
raw provider payloads.

Exact, compatible, manual-review, and layout-only scenarios are exercised as fixtures under
`fixtures/workspace/m5/m5-restore-provenance/`; the fail-closed rejections — including a handoff that
implies a full restore and an exact card that is not clean — are exercised as synthetic gate drills in
the crate's `m5_restore_provenance` unit tests.
