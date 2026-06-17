# Cross-ecosystem package-state descriptors

This document describes the canonical **package-state descriptor** — the product
object that carries one package's requested identity, resolved identity, source
provenance, effective policy, and finding/suppression linkage across every M5
package surface. It is the user-facing companion to the governed artifact at
`artifacts/deps/m5/package-state-descriptors.json`, the schema at
`schemas/deps/package-state-descriptors.schema.json`, and the typed model in the
`aureline-deps` crate (`package_state_descriptors`).

Where the
[package-state matrix](./freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.md)
*freezes the vocabulary* — the canonical package-state labels and the
registry/auth, lockfile-authority, and surface-binding control objects — the
descriptor is the object that **speaks that vocabulary** for a real package. One
descriptor is reused by package detail, the dependency tree, advisories,
license/compliance views, update proposals, the CLI inspect surface, and
support/export packets, so a dependency's provenance and current state survive
search, detail, finding, update, and export flows without semantic collapse.

## What a descriptor carries

Each descriptor records, in distinct fields:

- **Requested identity** — the ecosystem, package name, requested range/path/VCS
  ref, requested source kind, manifest scope, and whether policy pins it.
- **Resolved identity** — present only when the package resolved: the dependency
  relation (direct, transitive, workspace-local, or path/VCS), the exact resolved
  ref, the registry source (absent for workspace-local and path/VCS), the
  resolver, and the lockfile authority.
- **Resolution confidence** — whether the resolution is authoritative,
  cache/mirror-backed, an offline snapshot only, auth-gated and unresolved, or
  stale/unknown.
- **Auth mode and rollback class** — how the source was (or must be) reached and
  how a mutation of the package reverses.
- **Findings** — open advisories, suppressed-until findings (with suppression
  ref and expiry), and license-review-required overlays, kept separate from the
  package identity they sit on.

## Requested and resolved identity stay separate

The descriptor never flattens the two sides. A policy pin is a **requested**
constraint and is surfaced as such; the relation and exact-pin labels are
**resolved** facts. Because the two label sets are disjoint by construction, a
direct dependency can never be confused with a transitive one, and a requested
range can never be confused with the version it resolved to.

## States never overclaim certainty

`can_claim_resolved_exact` is `true` only for an authoritative or cache/mirror
resolution. When a package is:

- **auth-gated** — it is unresolved and renders an *auth-required* disclosure;
- **offline-snapshot only** — it keeps its pinned ref but renders an *offline
  snapshot* disclosure and cannot claim the pin is current;
- **stale or unknown** — it renders an *unknown/stale* disclosure rather than a
  generic "package not found" or "install failed".

A cache- or mirror-backed resolution is still exact but discloses its source
(`mirror_backed_source` / `cache_only_source`). No descriptor ever renders a
generic collapse message.

## The same object feeds every surface

The descriptor projects into:

- `view()` — the per-package detail/tree/finding view (reused by package detail,
  the dependency tree, and finding/license cards);
- `finding_cards()` and `license_compliance_row()` — advisory and
  license/compliance surfaces;
- `update_proposal()` — gates apply on auth, lockfile authority, and confidence;
- `export_row()` — the redaction-safe row reused by support/export packets and
  the CLI inspect surface;
- `surface_projection(surface)` — the descriptor rendered for a marketed surface
  with the write authority that surface may carry, pinned from the frozen matrix.

Every descriptor binds to the frozen matrix through `references_matrix_id`, and
every label it surfaces resolves to a frozen state row, so product, CLI, and
support/export paths express the same governed vocabulary mechanically.

## Cross-ecosystem coverage

The vocabulary is identical across ecosystems. The checked-in artifact exercises
Cargo, Node/pnpm, Python/pip, and another ecosystem across direct, transitive,
workspace-local, and path/VCS relations; public, private, mirror, and offline
sources; policy-pinned, auth-gated, offline, and stale states; and open,
suppressed, and license-review findings — so cross-ecosystem ambiguity is removed
rather than hidden behind a generic "installed" badge.
