# M5 Evidence Chronology and Actor-Lineage Packet — Artifact Summary

Canonical fixture: `fixtures/governance/m5_evidence_chronology_lineage/canonical_packet.yaml`

Schema: `schemas/governance/m5_evidence_chronology_lineage.schema.json`

Human-readable companion: `docs/governance/m5_evidence_chronology_lineage.md`

Producer: `aureline-chronology::m5_evidence_chronology_lineage`
(`seeded_m5_evidence_chronology_packet`).

Reuses the chronology grammar primitives from
`aureline-chronology::stabilize_chronology_grammar_and_history_row_truth`
(`TimePosture`, `ChronologySourceClass`, `ChronologyImportedClass`, `ActorKind`,
`ActionVerb`, `ProvenanceBadge`).

## Purpose

This artifact freezes the canonical evidence-time contract for the durable M5
evidence families: incident support packets, offboarding exit packets, AI
retained evidence, managed sync/mirror ledgers, and provider-linked work items.
It makes one timezone-aware chronology and actor/source lineage model
authoritative across product, admin, and support/export surfaces so that
support, incident, AI, provider, and offboarding evidence does not rely on team
memory or one local time zone when reconstructing what happened, in what order,
and from which source class.

It is metadata-only and carries opaque actor refs and review-safe labels, never
credential bodies, raw provider payloads, or raw user identifiers.

## Invariants enforced by `validate()`

- Schema version and packet/row record kinds match the frozen constants.
- Every row carries an absolute timestamp, IANA time-zone basis, UTC offset, and
  relative-age label, plus a grammar sentence generated from
  actor/action/object/outcome.
- Every row carries a source class, a live/imported class, and at least one
  provenance badge whose imported/cached/AI/policy export marker is preserved.
- Stale or expired rows explain why they stay visible.
- Actor lineage is non-empty, contiguously ordered from step zero, begins with an
  originator hop, and that originator matches the row's primary actor. Each hop
  preserves its own absolute time, time-zone basis, and live/imported class.
- Local-only rows never claim a remote hold, remote export, or remote delete, and
  state their local-only boundary explicitly.
- Row ids and canonical event ids are unique, and every governed workflow class
  is covered.

## Surface projections

- `product_projection()` — inline chronology and provenance for product rows.
- `admin_projection()` — full lineage reconstruction plus absolute chronology.
- `support_export_projection()` — export-safe chronology, provenance markers, and
  ordered lineage actor refs.

Admin and support projections preserve the same canonical event id, absolute
timestamp, source class, imported class, and lineage length as the live row.
