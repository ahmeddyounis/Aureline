# Implement low-disk or quota banners with visible eviction ordering

Low-disk and managed-quota pressure banners for the heavy artifact families the
M5 depth lanes add. The full contract — invariants, the matrix-backed composer,
and the support/export projection — lives in
[`/docs/storage/m5_storage_pressure_contract.md`](../storage/m5_storage_pressure_contract.md).

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  support-export projection: `crates/aureline-support/src/m5_storage_pressure/`.
- The boundary schema:
  [`/schemas/storage/m5_storage_pressure.schema.json`](../../schemas/storage/m5_storage_pressure.schema.json).
- A scenario corpus exercising constrained, degraded, and protect-core low-disk
  pressure, a managed-quota ceiling, and a quota case that refuses to delete
  protected state:
  [`/fixtures/storage/m5_storage_pressure_cases/`](../../fixtures/storage/m5_storage_pressure_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_storage_pressure/support_export.golden.json`](../../fixtures/storage/m5_storage_pressure/support_export.golden.json).
- The human-readable summary:
  [`/artifacts/storage/m5_storage_pressure.md`](../../artifacts/storage/m5_storage_pressure.md).

## Why

M5 lands notebook outputs, profiler traces, replay bundles, docs/model/template
packs, generated previews, prebuild layers, support artifacts, and
review/incident evidence — and it touches user-owned recovery state. When disk
or a managed quota runs low, the shell must not silently trim caches or, worse,
delete authoritative recovery or referenced evidence state. The banner turns
pressure into an inspectable disclosure: which pressure class fired, what work
paused, the frozen eviction order that applies, what stays protected, and where
to open the inspector and the class-selective review.

## Guarantees

- The **eviction order is the full frozen sequence** from the runtime low-disk
  ladder; no step is skipped, reordered, or hidden in logs-only diagnostics.
- Each **pressure tier auto-applies a bounded prefix** — constrained stops at
  the disposable hot cache, degraded at unpinned artifact/prebuild caches,
  protect-core at unpinned evidence past retention.
- **User-owned recovery state is never auto-trimmed** under any tier; its guard
  always reclaims zero bytes, and removal only ever moves under an explicit
  class-specific review.
- **Pinned and in-window evidence is retained**; only unpinned evidence past
  retention may expire, and only at protect-core.
- **Managed quota or disk pressure never silently deletes local user-owned
  state**; when only protected state remains over the ceiling, the banner asks
  for a reviewed escalation instead of deleting anything.
- Every banner **states the pressure class, paused work, next eviction order,
  protected classes, and the open-inspector action** using stable vocabulary,
  and reports no authoritative state loss.

## Proof

Automated proof lives in
`crates/aureline-support/src/m5_storage_pressure/tests.rs`:

- the scenario corpus parses and validates with zero violations;
- every banner lists the full frozen ladder in order and discloses the two
  pause steps and the open-inspector action;
- the user-owned recovery step is never auto-applied and its guard always
  reclaims zero bytes;
- evidence only expires unpinned-past-retention entries, and only at
  protect-core;
- a pending escalation never reclaims protected bytes;
- the matrix-backed composer reproduces the seeded banners and never auto-trims
  recovery state even at protect-core;
- the metadata-safe support export matches its checked-in golden.
