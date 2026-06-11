# Per-ecosystem dependency, package, and code-quality certification

This document describes the canonical packet that certifies dependency
intelligence, package review, and code-quality or scanner maturity on every
marketed ecosystem, and that automatically narrows any underqualified row before
publication. It is the user-facing companion to the governed artifact at
`artifacts/deps/m5/ecosystem-qualification-certification.json` and the typed
model in the `aureline-deps` crate
(`ecosystem_qualification_certification`).

## What this packet covers

The packet is a **certification matrix**: one row for every (ecosystem, lane)
cell. The claimed ecosystems are `cargo`, `node_pnpm`, and `python_pip`, and the
qualification lanes are:

1. **`dependency_intelligence`** — advisory, vulnerability, license, notice, and
   SBOM intelligence.
2. **`package_review`** — package/manifest/lockfile mutation review.
3. **`code_quality`** — live code-quality / quality-profile depth.
4. **`scanner_import`** — imported scanner (SARIF) parity and maturity.

Each row answers, for its cell:

- **What does the lane claim?** A `declared_maturity` of `certified`,
  `provisional`, `underqualified`, or `unsupported`.
- **How fresh is the certification?** A `certification_freshness` of `current`,
  `stale`, `expired`, or `unknown`.
- **What is backing it?** A `qualification_packet_ref` to the lane's own proof
  packet and a `corpus_ref` to its own proof corpus.
- **What blocks promotion?** Zero or more `blocking_reasons`.
- **What does the gate publish?** A `published_maturity` and a `narrowing_action`
  derived from the inputs above.

## The promotion gate narrows automatically

The maturity a row may publish is **not** copied from `declared_maturity`. It is
recomputed from the row's freshness, blocking reasons, and evidence, and the
`published_maturity` and `narrowing_action` fields must equal that recomputation
or validation fails. The gate lowers the published maturity to the weakest of:

- the **declared maturity**;
- the **freshness ceiling** — `current` permits `certified`, `stale`/`unknown`
  cap at `provisional`, and `expired` caps at `underqualified`;
- each **blocking reason ceiling** — `stale` and `mirror_blocked` cap at
  `provisional`; `scanner_underqualified`, `missing_package_lockfile_evidence`,
  and `missing_corpus` cap at `underqualified`;
- an **evidence ceiling** — a row missing its qualification packet, proof corpus,
  or package/lockfile review evidence caps at `underqualified`.

The `narrowing_action` then names the result: `none` for a published
`certified`, `narrow_to_provisional`, `narrow_to_underqualified`, or
`withhold_from_publication` for `unsupported`.

This is what lets release tooling **prove** that stale or underqualified rows
narrow before publication: a row that is stale, mirror-blocked,
scanner-underqualified, or missing review/corpus evidence simply cannot carry a
`certified` published claim, because the recomputed gate decision overrides the
stored one.

## Certification stays row-specific

A strong ecosystem must never imply maturity on an unrelated one. The packet
enforces this two ways:

- Every claimed (ecosystem, lane) cell must carry exactly one row
  (`MissingMatrixCell` / `DuplicateMatrixCell` otherwise), so no lane inherits
  trust from an adjacent cell.
- Every row must carry its own non-empty `qualification_packet_ref` and
  `corpus_ref`, and a row may not cover an ecosystem outside the claimed set
  (`UnclaimedEcosystemRow`).

A promotable row — one that publishes `certified` — must additionally be clean:
current freshness and no blocking reason (`PromotedRowNotClean` otherwise).

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with the ecosystem, lane,
declared and published maturity, freshness, narrowing action, and blocking-reason
tokens, plus `promotable_count`, `narrowed_count`, and `withheld_count`. Help/About,
docs/migration, support exports, and release/public-truth packets — including the
M5 dependency/quality release freeze matrix — should ingest this projection
directly rather than restating certification status by hand, so the public and
internal claim surfaces use the same lifecycle, freshness, and downgrade
vocabulary as the underlying packet.

## Validation

`EcosystemQualificationCertification::validate()` reports every violation,
including unsupported schema version or record kind, non-canonical closed
vocabularies, empty required fields, duplicate row ids, duplicate or missing
matrix cells, unclaimed-ecosystem rows, duplicate blocking reasons, an
overstated published maturity, a narrowing action that disagrees with the gate, a
promotable row that is not clean, and a summary block that disagrees with the
rows.
