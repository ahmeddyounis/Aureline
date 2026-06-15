# Certify storage-class truth, low-disk behavior, clear-data previews, and pin/retention integrity on M5 heavy-artifact profiles

One shared certification index over the M5 heavy-artifact storage lanes. For
every heavy-artifact family the M5 depth lanes add — generated previews,
notebook outputs, docs / model / template packs, extension downloads, prebuild
layers, profiler traces, replay bundles, support artifacts, review / incident
evidence, and the user-owned recovery state those lanes touch — on every claimed
M5 profile, the certification binds storage-class truth, class-selective
clear-data previews, low-disk / managed-quota pressure behavior, pin/retention
integrity, corruption-repair, and export-before-delete proof into one decision
surface. The full contract — published states, pressure-source posture,
downgrade automation, and shared-surface bindings — lives in
[`/docs/storage/m5_storage_certification_contract.md`](../storage/m5_storage_certification_contract.md).

## What shipped

- The canonical product object plus its validator, matrix cross-check, and
  shared-surface bindings:
  `crates/aureline-support/src/m5_storage_certification/`.
- The boundary schema:
  [`/schemas/storage/m5_storage_certification.schema.json`](../../schemas/storage/m5_storage_certification.schema.json).
- The reviewer contract and human-readable review:
  [`/docs/storage/m5_storage_certification_contract.md`](../storage/m5_storage_certification_contract.md)
  and
  [`/artifacts/storage/m5_storage_certification.md`](../../artifacts/storage/m5_storage_certification.md).
- A canonical fixture plus two degraded fixtures — a stale pin/retention audit
  that gates every protected family, and stale/blurred storage-class truth that
  blocks authoritative families and narrows disposable ones:
  [`/fixtures/storage/m5_storage_certification/`](../../fixtures/storage/m5_storage_certification/).
- The replay example and protected tests:
  `crates/aureline-support/examples/dump_m5_storage_certification_packet.rs` and
  `crates/aureline-support/tests/m5_storage_certification.rs`.

## Why

M5 adds heavy artifact families whose storage lanes are not complete until
Aureline can explain, per family and per profile, what is disposable, what is
rebuildable, what is pinned, what is user-owned recovery state, and what happens
under low-disk or managed-quota pressure. The individual lanes already prove
those behaviors; this certification ties them together so a row cannot stay
green while any of its underlying proofs is stale, and so no surface can blur
cache versus authoritative state or hide pressure behavior behind a stale proof.

## Guarantees

- **One certification index, no synonyms.** Each row reuses the
  storage-governance matrix's storage class, authority, and protection posture
  and cites the same metadata-safe golden support-export projections the sibling
  lanes already check in.
- **Managed quota never silently deletes user-owned state.** Protected families
  on `managed_cloud` are `managed_quota_protected_excluded`; only explicit,
  reviewed removal can free them.
- **Blur and hidden pressure downgrade automatically.** Stale storage-class
  truth blocks authoritative families; stale pressure proof narrows disposable
  ones; stale pin/retention or export-before-delete proof gates protected
  families behind an explicit review.
- **Shared-surface parity.** Help/About, service health, support export, and
  release manifest bindings all ingest the same packet id and preserve the same
  row fields verbatim; a broken binding blocks the broad claim.
- **Export safety.** The certification is metadata-only and by reference; raw
  artifact payloads, raw caches, raw logs, and secrets stay outside the boundary.

## Proof

Automated proof lives in
`crates/aureline-support/tests/m5_storage_certification.rs`:

- the canonical packet validates with zero violations, is export-safe, and is
  fully qualified across all twelve families and five profiles;
- every row stays consistent with the storage-governance matrix;
- managed-cloud rows protect user-owned and evidence-grade families from
  quota-driven deletion;
- the surface bindings all ingest one certification index and preserve the
  required verbatim fields;
- the stale-pin/retention fixture gates every protected family, and the
  blurred-cache-authority fixture blocks authoritative families and narrows
  disposable, pressure-evicted ones;
- the degraded fixtures cite only known downgrade rules and are never green;
- the checked-in schema, docs, artifact, and fixtures exist and replay exactly
  against the seeded packet.
