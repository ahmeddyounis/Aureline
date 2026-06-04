# Fixtures: Package mutation and registry review

This directory contains fixture metadata for the
`package_mutation_and_registry_review` packet.

The canonical full corpus is checked in at:

`artifacts/deps/m4/package-mutation-and-registry-review.json`

## Coverage

- Cargo and pnpm are the only stable ecosystems claimed.
- Manifest-scoped add, update, remove, and resolve operations are represented.
- `no_results`, `auth_required`, `mirror_stale`, and
  `offline_snapshot_only` are separate package-search states.
- Requested ranges or sources stay separate from resolved exact versions or
  sources.
- Registry source class, credential mode, freshness, reachability, and policy
  lock state are represented without exporting raw secrets.
- Lockfile impact covers direct bump, security patch, grouped refresh,
  lockfile-only refresh, major-version pilot, and workspace-wide convergence.
- AI and automation-suggested dependency changes use the same review surface as
  manual package operations.
- Grouped-update plans and history/recovery rows point back to operation review
  rows, validation packs, and rollback checkpoints.
