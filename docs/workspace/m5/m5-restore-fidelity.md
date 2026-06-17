# M5 restore provenance and fidelity

Every M5 re-entry flow — desktop restore, portable-state import, crash recovery, support-packet
replay, and browser/companion re-entry — must disclose not only **that** something came back, but
**how well** it came back. The **restore-provenance card** is that disclosure: one card per re-entry
surface that attaches to a restore/import/handoff event its source, producer/build provenance,
restored schema version, redaction class, and the resulting restore fidelity, drawn from one shared
vocabulary so the labels mean the same thing on every surface.

- Typed model: `aureline-workspace` crate, module `m5_restore_provenance`.
- Packet: [`artifacts/workspace/m5/m5-restore-provenance.json`](../../../artifacts/workspace/m5/m5-restore-provenance.json).
- Reviewer artifact: [`artifacts/workspace/m5/m5-restore-provenance.md`](../../../artifacts/workspace/m5/m5-restore-provenance.md).
- Schema: [`schemas/workspace/m5-restore-provenance.schema.json`](../../../schemas/workspace/m5-restore-provenance.schema.json).
- Fixtures: [`fixtures/workspace/m5/m5-restore-provenance/`](../../../fixtures/workspace/m5/m5-restore-provenance/).
- Classification contract: [`docs/workspace/m5/m5-serialization-and-restore.md`](./m5-serialization-and-restore.md).

## The four fidelity labels — one vocabulary, every surface

Restore fidelity is the **canonical matrix vocabulary**, reused, never forked:

- **Exact restore** — the remembered state came back value-for-value.
- **Compatible restore** — the state came back through a forward schema migration or display
  adaptation; semantics preserved, but not byte-identical.
- **Layout-only** — only the pane/window layout came back; contents reopened as context or show a
  placeholder.
- **Manual review** — the state could not be applied automatically; it is surfaced for a human with
  the slot preserved.

The same four labels are used by desktop restore, import, crash recovery, support replay, and
companion/browser re-entry. There is no separate fidelity vocabulary per surface.

## What every card carries

Each [`RestoreProvenanceCard`] attaches to one restore/import/handoff event:

- the **source** — `auto_checkpoint`, `manual_export`, `backup`, `sync`, `import`, or
  `browser_companion_handoff`;
- **producer / version / build** provenance — which component, at which version and build, wrote the
  remembered state;
- the **restored schema version** and the observed schema, dependency, topology, and
  evidence-freshness conditions;
- the **redaction class** — secrets, live authority, machine-local anchors, and raw provider
  payloads are all excluded (the record is metadata only); and
- the **resulting restore fidelity**, with its downgrade reasons and the recovery next step.

## The fail-closed gate

The published fidelity is the **weakest ceiling** implied by the declared resulting fidelity, the
source ceiling, and the schema, dependency, topology, and evidence-freshness conditions
([`RestoreProvenanceCard::achieved_fidelity`]). A schema drift, a missing dependency, a changed
topology, or stale evidence narrows the published fidelity automatically — a restore can never claim
exact continuity by inertia.

A **browser/companion handoff is a contextual reopen**: its source ceiling is layout-only, so a
handoff can never imply a full (exact) restore even when its other conditions are clean. An exact
card must be genuinely clean — pristine conditions, a non-contextual source, no downgrade reason, and
no recovery step — so a downgraded or placeholder-heavy restore is never presented as exact
continuity. A missing dependency never silently deletes layout; it preserves the slot as a
placeholder or reopens the target as context.

## Open-details, compare, recovery — preserved when it matters

Every card offers a read-only **open-details** action, so a restore's provenance is never hidden.
Wherever the fidelity was narrowed or a dependency was missing, the card also preserves a **compare**
action (review the restore against another remembered state before relying on it) and a
**recovery-next-step** action (the concrete path that would restore more). Every affordance carries a
command id, a keyboard shortcut, a deterministic focus order, and a screen-reader label, and stays
scoped to the one restore event.

## Same record everywhere — no weaker summaries

[`ProvenanceConsumerBinding`] wires the parity surfaces — **exported diagnostics**, the **support
packet**, the **crash-recovery packet**, and the **companion handoff** — to this one packet. Each
attests that it carries the same provenance and fidelity record verbatim and narrows with it, so
support and crash-recovery evidence preserve exactly what the user saw rather than inventing a weaker
summary. `M5RestoreProvenance::card_view` produces the plain-language projection those surfaces
render, and `M5RestoreProvenance::support_export` preserves the record for evidence bundles.

[`RestoreProvenanceCard`]: ../../../crates/aureline-workspace/src/m5_restore_provenance/mod.rs
[`RestoreProvenanceCard::achieved_fidelity`]: ../../../crates/aureline-workspace/src/m5_restore_provenance/mod.rs
[`ProvenanceConsumerBinding`]: ../../../crates/aureline-workspace/src/m5_restore_provenance/mod.rs
