# Targeted cache / index repair, stale-label propagation, no-reset-everything fallback

Targeted repair of one corrupt or stale storage class — with stale-label
propagation and a no-reset-everything fallback — for the heavy artifact families
the M5 depth lanes add. The full contract — invariants, the matrix-backed
composer, and the support / export projection — lives in
[`/docs/storage/m5_cache_repair_contract.md`](../storage/m5_cache_repair_contract.md).

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  support-export projection: `crates/aureline-support/src/m5_cache_repair/`.
- The boundary schema:
  [`/schemas/storage/m5_cache_repair.schema.json`](../../schemas/storage/m5_cache_repair.schema.json).
- A scenario corpus exercising a corrupt search / graph index, a docs / model
  pack checksum mismatch, a torn generated preview, a quarantined evidence trace,
  a repaired-in-place recovery journal, and a failed prebuild repair with a
  targeted fallback:
  [`/fixtures/storage/m5_cache_repair_cases/`](../../fixtures/storage/m5_cache_repair_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_cache_repair/support_export.golden.json`](../../fixtures/storage/m5_cache_repair/support_export.golden.json).
- The human-readable summary:
  [`/artifacts/storage/m5_cache_repair.md`](../../artifacts/storage/m5_cache_repair.md).

## Why

M5 lands notebook outputs, profiler traces, replay bundles, docs / model /
template packs, generated previews, prebuild layers, support artifacts, and
review / incident evidence — every one of them a cache or index that can be
detected corrupt or stale. Those lanes are not complete until the shell can
repair *one* affected class without a vague "clear everything" or factory reset,
keep every affected surface honest about being stale or rebuilding until the
repair completes, and — when the targeted repair fails — fall back to a narrower
action rather than a delete-all. The cache-repair plan answers all three: it
names the affected class, the detected fault, and the narrowest sufficient repair
action; it quarantines any suspect copy that still holds user-owned data or
forensic value before any clear; and it propagates a stale / rebuild-needed /
corrupt label to every affected surface until the repair actually completes.

## Guarantees

- The **repair is targeted by construction**: no plan offers a factory reset,
  the scope is one storage class, and there is no global / reset-everything value
  in any vocabulary.
- A **suspect copy is quarantined before any clear** when it still holds
  user-owned data or forensic value; evidence and user-owned recovery classes are
  never disposable-only and never cleared without preservation.
- **Protected state is preserved, not rebuilt**: user-owned recovery state is
  repaired in place from a checkpoint, and evidence is quarantined for a
  class-specific review — neither is auto-rebuilt from a derived source.
- **Stale labels stay visible** on every affected surface until the repair
  completes, and they are never hidden in logs-only diagnostics.
- **A failed repair falls back narrower**, never to a reset-everything.
- The **repair action and protection posture are derived** from the canonical
  runtime storage-class profiles and the detected fault; no surface invents a
  private repair vocabulary.

## Proof

Automated proof lives in
`crates/aureline-support/src/m5_cache_repair/tests.rs`:

- the scenario corpus parses and validates with zero violations;
- every plan is export-safe, offers no factory reset, and avoids
  reset-everything;
- protected classes always quarantine the suspect copy; disposable classes need
  none;
- user-owned recovery is repaired in place and evidence is quarantined for
  review, with neither cleared;
- every affected surface label stays active until the repair completes;
- the matrix-backed composer reproduces every seeded plan exactly and quarantines
  before clearing when a non-protected class holds user-owned data;
- negative tests reject an offered factory reset, a label cleared before the
  repair completes, and a protected class without a quarantine copy;
- the metadata-safe support export matches its checked-in golden.
