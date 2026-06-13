# M5 Evidence Chronology, Actor Lineage, and Source/Live/Imported Truth

This page is the human-readable companion to the canonical M5
evidence-chronology packet emitted by `aureline-chronology` and mirrored at
`artifacts/governance/m5_evidence_chronology_lineage.md`. A frozen example of the
packet lives at
`fixtures/governance/m5_evidence_chronology_lineage/canonical_packet.yaml` and
its shape is validated by
`schemas/governance/m5_evidence_chronology_lineage.schema.json`.

## Why this packet exists

The M5 surfaces add AI evidence, provider-linked work items, incident and
companion packets, managed sync/offboarding records, and richer support/export
surfaces. When support or admin reviewers reconstruct what happened on these
rows, they should not have to rely on team memory or one local time zone, and an
imported, cached, or stale row must not be able to masquerade as live first-party
truth.

This packet makes one timezone-aware chronology and actor/source lineage model
authoritative for those families. It is metadata-only: lineage steps carry
opaque actor refs and review-safe labels, never credential bodies, raw provider
payloads, or raw user identifiers.

## Object model

Each governed evidence family contributes one row that carries:

- **Workflow class** — `incident_support`, `offboarding`, `ai_evidence`,
  `managed_sync`, or `provider_linked`.
- **Time posture** — the absolute UTC timestamp, the IANA time-zone basis, the
  UTC offset, a local time label, the live/imported class, and a deterministic
  relative-age hint. Stale or expired rows must explain why they remain visible.
- **Source and imported classes** — the `source_class` (first-party,
  provider-imported, AI-assisted, policy-authored, …) and the `imported_class`
  (`live`, `imported`, `cached`, `stale_cached`, `reconstructed`) so a retained
  artifact never looks like live system truth.
- **Provenance badges** — short visible badges whose export markers preserve the
  imported/cached/stale/AI/policy text verbatim.
- **Actor lineage** — the ordered chain of actors that produced, relayed,
  imported, reviewed, approved, or now hold the evidence. The first hop is the
  originator and matches the row's primary actor; the final custodian ref names
  who holds the evidence now. Every hop preserves its own absolute time,
  time-zone basis, and live/imported class.
- **Residency** — `local_only`, `managed_copy`, or `local_and_managed`. A
  `local_only` row may never claim a remote hold, remote export, or remote
  delete, and states its local-only boundary explicitly.

## Surface parity

The packet exposes three projections that read one vocabulary:

- **Product** — workflow class, grammar sentence, source and imported classes,
  absolute timestamp, relative age, residency, and the lineage hop count.
- **Admin** — the full reconstructed actor lineage plus the absolute timestamp,
  time-zone basis, source class, and imported class, so an admin reviewer can
  reconstruct lineage and chronology without guessing from inconsistent
  timestamps.
- **Support/export** — the canonical event id, grammar sentence, exported
  provenance markers, absolute timestamp, time-zone basis, source/imported
  classes, ordered lineage actor refs, residency, and any local-only boundary
  note.

Admin and support projections preserve the same canonical event id, absolute
timestamp, source class, imported class, and lineage length as the live row, so
the chronology stays stable across UI, CLI/headless, release packets, and support
exports.

## Invariants enforced by `validate()`

- Schema version and record kinds match the frozen constants.
- Every row carries a non-empty absolute timestamp, time-zone basis, UTC offset,
  and relative-age label, and a grammar sentence generated from
  actor/action/object/outcome.
- Every row carries at least one provenance badge, and imported/cached/AI/policy
  badges preserve their marker text into the export label.
- Stale or expired rows explain why they remain visible.
- Actor lineage is non-empty, contiguously ordered from step zero, begins with an
  originator hop, and that originator matches the row's primary actor.
- Local-only rows never claim a remote hold, remote export, or remote delete.
- Row ids and canonical event ids are unique, and every workflow class is
  covered.

## Keeping the fixture in sync

The checked-in fixture mirrors the seeded packet. Regenerate it with:

```bash
cargo run -p aureline-chronology --example dump_m5_evidence_chronology_lineage \
  > fixtures/governance/m5_evidence_chronology_lineage/canonical_packet.yaml
```

The crate's `checked_in_canonical_fixture_matches_seeded_packet` test fails if the
fixture drifts from the seeded packet.
