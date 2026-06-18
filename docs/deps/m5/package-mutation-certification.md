# Package-mutation certification on every ecosystem, mirror, and offline row

This document describes the canonical packet that certifies the built-in
package-manager mutation claim — package-state truth, registry-auth continuity,
and lockfile-safe review — on every claimed ecosystem and every deployment
profile, and that automatically narrows any row whose evidence is stale or whose
cross-surface parity has broken before publication. It is the user-facing
companion to the governed artifact at
`artifacts/deps/m5/package-mutation-certification.json` and the typed model in
the `aureline-deps` crate (`package_mutation_certification`).

## What this packet covers

The packet is a **certification matrix**: one row for every (ecosystem,
deployment-profile) cell. The claimed ecosystems are `cargo`, `node_pnpm`, and
`python_pip`, and the deployment profiles are:

1. **`direct_registry`** — the ecosystem's primary registry, reached directly and
   online.
2. **`registry_mirror`** — an enterprise or organization registry mirror.
3. **`offline_snapshot`** — an offline / cache-only snapshot with no live registry
   reach.

A claim proven against a direct registry is **not** automatically trustworthy
through a mirror or from an offline snapshot, so each profile is certified as its
own row. Every claimed cell must carry exactly one row (`MissingMatrixCell` /
`DuplicateMatrixCell` otherwise).

## The four mutation-proof dimensions

Each row certifies four dimensions, each carrying a proof state of `proven`,
`degraded` (proven only under degraded mirror/offline conditions), `stale`
(evidence past its freshness SLO), or `unproven`:

1. **`package_state_truth`** — requested/resolved identity, relation, advisory and
   license overlays, and resolution environment.
2. **`registry_auth_continuity`** — a credential reaches the registry or mirror,
   and degradation states stay distinct from a generic failure.
3. **`lockfile_safe_review`** — the pre-apply review, lockfile diff class, and
   rollback checkpoint hold before any mutation commits.
4. **`cross_surface_parity`** — product, CLI, docs/help, and support packets
   express the same mutation truth.

## The publication gate fails closed

The claim a row may publish is **not** copied from `declared_claim`. It is
recomputed from the row's freshness and dimension proofs, and the
`published_claim` and `narrowing_action` fields must equal that recomputation or
validation fails. The gate lowers the published claim to the weakest of:

- the **declared claim**;
- the **freshness ceiling** — `current` permits `certified`, `stale`/`unknown`
  cap at `retest_pending`, and `expired` caps at `unsupported`;
- each **dimension ceiling** — `proven` permits `certified`, `degraded` caps at
  `limited`, `stale` caps at `retest_pending`, and `unproven` caps at
  `unsupported`.

The `narrowing_action` then names the result: `none` for a published `certified`,
`narrow_to_limited`, `narrow_to_retest_pending`, or `withhold_as_unsupported` for
`unsupported`.

This is what lets release/public-truth surfaces **prove** that stale or
underqualified rows narrow before publication: a row whose lockfile review is
stale, whose continuity is mirror/offline-only, or whose parity is broken simply
cannot carry a `certified` published claim, because the recomputed gate decision
overrides the stored one.

## Cross-surface parity is recomputed, not asserted

Every row carries a parity cell for each surface — `product`, `cli`, `docs_help`,
and `support_export` — with a state of `consistent`, `divergent`, or `absent`.
The recorded `cross_surface_parity` dimension must equal the state recomputed from
those cells (`ParityStateMismatch` otherwise): any **divergent** surface forces
`unproven`, any **absent** surface caps the dimension at `degraded`, and a fully
consistent set is `proven`. A row therefore cannot read green while product, CLI,
docs/help, and support packets disagree.

## Binding to the frozen matrix

`matrix_ref` pins the packet to the frozen package-state matrix at
`artifacts/deps/m5/freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.json`
(`MatrixRefMismatch` otherwise), so every claimed surface references the one
shared package-state vocabulary rather than ecosystem-local folklore.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with the ecosystem,
deployment profile, declared and published claim, freshness, narrowing action,
per-dimension proof states, per-surface parity states, and the limiting
dimensions, plus `promotable_count`, `narrowed_count`, `withheld_count`, and
`parity_break_count`. Help/About, docs/help, support exports, and
release/public-truth packets should ingest this projection directly rather than
restating certification status by hand, so the public and internal claim surfaces
use the same lifecycle, freshness, and downgrade vocabulary as the underlying
packet.

## Validation

`PackageMutationCertification::validate()` reports every violation, including an
unsupported schema version or record kind, a `matrix_ref` that is not the frozen
matrix path, non-canonical closed vocabularies, empty required fields, duplicate
row ids, duplicate or missing matrix cells, unclaimed-ecosystem rows, incomplete
or duplicated dimension proofs or surface parity cells, a parity dimension that
disagrees with the recomputed surfaces, an overstated published claim, a
narrowing action that disagrees with the gate, a promotable row that is not clean,
and a summary block that disagrees with the rows.
