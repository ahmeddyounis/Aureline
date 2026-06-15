# Offboarding-safe cleanup, offline / mirror continuity warnings, certified-workspace / policy-bundle pin protection

Honest offboarding for the heavy artifact families the M5 depth lanes add:
account offboarding, device reset, workspace wipe, and sign-out cleanup that
distinguish exportable durable state from non-portable derived data, name the
offline / mirror / certified-workspace continuity each removal would break, and
protect captured evidence, user-owned recovery state, and offline / certified /
policy-bundle pins unless explicitly reviewed away. The full contract —
invariants, the matrix-backed composer, and the support / export projection —
lives in
[`/docs/storage/m5_offboarding_continuity_contract.md`](../storage/m5_offboarding_continuity_contract.md).

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  support-export projection:
  `crates/aureline-support/src/m5_offboarding_continuity/`.
- The boundary schema:
  [`/schemas/storage/m5_offboarding_continuity.schema.json`](../../schemas/storage/m5_offboarding_continuity.schema.json).
- A scenario corpus across an account offboarding that retains durable state, a
  device reset that clears only caches, a sign-out cleanup that keeps offline /
  certified / policy pins, an offboarding that reviews offline packs away with
  continuity warnings, and a workspace wipe that exports evidence and recovery
  state before removing them:
  [`/fixtures/storage/m5_offboarding_continuity_cases/`](../../fixtures/storage/m5_offboarding_continuity_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_offboarding_continuity/support_export.golden.json`](../../fixtures/storage/m5_offboarding_continuity/support_export.golden.json).
- The human-readable summary:
  [`/artifacts/storage/m5_offboarding_continuity.md`](../../artifacts/storage/m5_offboarding_continuity.md).

## Why

M5 lands notebook outputs, profiler traces, replay bundles, docs / model /
template packs, generated previews, prebuild layers, support artifacts, and
review / incident evidence — plus the user-owned recovery state earlier
milestones own. When a user offboards, resets a device, or wipes a workspace,
those lanes are not complete until the shell can say honestly what is exportable
durable state the user should take with them, what is non-portable derived data
that simply rebuilds, what offline / mirror / certified-workspace continuity a
removal would break, and which families stay pinned for continuity. The
offboarding continuity plan answers all of that, and it never lets a cleanup imply
full data portability when only caches were removed.

## Guarantees

- **Protected and continuity-pinned families are never silently disposed.**
  Captured evidence, user-owned recovery state, and offline / certified / release /
  policy-pinned packs are retained by default; removal needs an explicit,
  exported review.
- **Protected classes require export-before-delete** in either bucket, with an
  export action linked.
- **Portability is honest**: the headline is computed and a caches-only
  offboarding can never claim it exported everything.
- **Continuity stays visible before deletion**: every row carries its continuity
  stakes, and accepting an offline / mirror / certified / policy loss surfaces a
  named guardrail notice rather than a logs-only diagnostic.
- **Disposition, portability, continuity, export posture, and the headline are
  derived** from the frozen artifact-family matrix and the pins actually present;
  no surface invents a private cleanup vocabulary, and pins a family's matrix row
  does not admit carry no continuity.
- **Storage pressure has no path here**: disposal is operator-driven and explicit,
  so managed quota or low disk can never wipe user-owned state through an
  offboarding side effect.

## Proof

Automated proof lives in
`crates/aureline-support/src/m5_offboarding_continuity/tests.rs`:

- the scenario corpus parses and validates with zero violations;
- every plan is export-safe and offers the inspector and class-selective review;
- protected and continuity-pinned families are never disposed without an explicit
  review, and protected classes always require export-before-delete;
- continuity warnings track the storage class and the pins present, and the
  portability headline never over-promises when only caches were removed;
- the matrix-backed composer reproduces every seeded plan exactly, retains an
  offline pin requested without review, exports durable state reviewed away, and
  drops pins inadmissible under the matrix;
- negative tests reject a protected row moved into the disposed bucket, a
  reviewed-away flag dropped from a disposed protected row, a mutated portability
  headline, a hidden continuity note, a tampered byte total, and a mis-derived
  portability class;
- the metadata-safe support export matches its checked-in golden.
