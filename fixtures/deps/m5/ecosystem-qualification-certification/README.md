# Fixtures: Per-ecosystem qualification certification

This directory contains fixture metadata for the
`ecosystem_qualification_certification` packet.

The canonical full corpus is checked in at:

`artifacts/deps/m5/ecosystem-qualification-certification.json`

## Coverage

- `cargo`, `node_pnpm`, and `python_pip` are the only claimed ecosystems, and
  every (ecosystem, lane) cell carries exactly one row — no lane inherits trust
  from an adjacent cell.
- The four qualification lanes (`dependency_intelligence`, `package_review`,
  `code_quality`, `scanner_import`) each carry their own qualification packet and
  proof corpus on every ecosystem.
- Published maturity covers `certified`, `provisional`, `underqualified`, and
  `unsupported`, and the narrowing action covers `none`,
  `narrow_to_provisional`, `narrow_to_underqualified`, and
  `withhold_from_publication`.
- Certification freshness covers `current`, `stale`, `expired`, and `unknown`.
- Blocking reasons cover `stale`, `mirror_blocked`, `scanner_underqualified`,
  `missing_package_lockfile_evidence`, and `missing_corpus`.
- The promotion gate is exercised in both directions: clean rows promote to a
  full certification, while stale, mirror-blocked, scanner-underqualified,
  expired, and evidence-missing rows narrow automatically. Each row's
  `published_maturity` and `narrowing_action` equal the recomputed gate decision,
  so release tooling can prove underqualified rows narrow before publication.
