# Fixtures: Package-mutation certification

This directory contains fixture metadata for the
`package_mutation_certification` packet.

The canonical full corpus is checked in at:

`artifacts/deps/m5/package-mutation-certification.json`

## Coverage

- `cargo`, `node_pnpm`, and `python_pip` are the only claimed ecosystems, and the
  deployment profiles are `direct_registry`, `registry_mirror`, and
  `offline_snapshot`. Every (ecosystem, profile) cell carries exactly one row, so
  a claim proven against a direct registry never silently extends to the mirror or
  offline rows it was never tested on.
- Each row certifies the four mutation-proof dimensions — `package_state_truth`,
  `registry_auth_continuity`, `lockfile_safe_review`, and `cross_surface_parity`
  — with a proof state of `proven`, `degraded`, `stale`, or `unproven`.
- Published claim covers `certified`, `limited`, `retest_pending`, and
  `unsupported`, and the narrowing action covers `none`, `narrow_to_limited`,
  `narrow_to_retest_pending`, and `withhold_as_unsupported`.
- Evidence freshness covers `current`, `stale`, `expired`, and `unknown`.
- Surface parity covers `consistent`, `divergent`, and `absent`, and the recorded
  `cross_surface_parity` dimension is recomputed from those per-surface cells.
- The publication gate is exercised in both directions: a clean row promotes to a
  full certification, while stale-freshness, mirror/offline-degraded,
  stale-dimension, parity-broken, expired, and unproven rows narrow automatically.
  Each row's `published_claim` and `narrowing_action` equal the recomputed gate
  decision, so release/public-truth surfaces can prove underqualified rows narrow
  before publication.
