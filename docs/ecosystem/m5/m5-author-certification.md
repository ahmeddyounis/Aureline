# M5 author-side certification

Overview companion to the canonical `m5_author_certification` packet, the author-side
counterpart to the install-side `m5_ecosystem_certification` qualification layer.

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-author-certification.json`
- Boundary schema: `schemas/ecosystem/m5-author-certification.schema.json`
- Owning crate module: `crates/aureline-ecosystem/src/m5_author_certification/`
- Fixture corpus: `fixtures/ecosystem/m5/m5-author-certification/`

## What it is

Where install certification rolls the *end-user install* drills into one decision per
marketed M5 family, this packet decides whether that family's **author lane** still backs
the install claim it advertises. Each entry aggregates the per-lane evidence the author
drills produce and resolves to a real author-and-publish-preview matrix row and a real
install-certification entry, so it stays an aggregator rather than a parallel spreadsheet:

- `local_dev_workspace` — the local extension workspace strip (unsigned/local-only truth,
  build freshness);
- `sideload_review` — source identity, requested permissions, and the registry-binding
  decision;
- `sandbox_inspection` — the runtime inspector (host, granted capabilities, failures);
- `publish_preview` — the publish-preview blocker/warning suite and registry-policy
  consequences;
- `reload_continuity` — hot-reload/relaunch and last-loaded-build continuity;
- `anti_abuse_transparency` — the post-publication ranking, quarantine, and continuity
  board.

## What it recomputes

Every published value is recomputed from the entry's facts; a checked-in packet that drifts
fails `M5AuthorCertification::validate`.

- **Effective trust posture** — the weakest of the declared posture, the signing-state
  ceiling, the workspace-origin ceiling, and the registry-binding ceiling. A locally-built,
  side-loaded, or pending-rebind artifact never inherits a verified-publisher or
  enterprise-approved badge. A board-level cross-check proves no entry renders a stronger
  badge than the author-and-publish-preview gate grants the same family.
- **Effective author support class** — the weakest of the end-user install claim, the
  author-side ceiling (source class, evidence freshness, effective trust posture, and author
  publish readiness), and the disposition ceiling. When the author-side ceiling lands below
  the install claim the marketed row **narrows automatically**, and the
  `author_claim_below_install_claim` signal records the gap. The author claim may never
  exceed the install claim it guards.
- **Certification disposition** — the widest minimum disposition over every detected signal.
  A warnings lane or a publish-with-warnings gate narrows to `conditionally_certified`; a
  fresh-review-required or stale lane, stale evidence, or an author claim below the install
  claim narrow to `downgraded`; and a missing or failed lane, a missing owner, a blocked
  publish gate, or an anti-abuse quarantine hold each force `uncertified`, whose effective
  support collapses to `unsupported`.
- **Downgrade path** — every narrowed entry names the support class the marketed claim drops
  to, the effective trust posture, the signals that explain it, and the opaque
  requalification ref an author follows to recover.

## Narrowing / cross-check

- A widening hot reload (runtime class, permissions, or an external executable) raises a
  `fresh_review_required` lane and, through a blocked publish gate, an `uncertified` row, so
  authority never widens through hot reload without a fresh review.
- A signed artifact built in a local-dev workspace still renders `unsigned_local_only`, so a
  package never inherits a trusted badge just because it was built on a trusted machine.
- Downstream surfaces consume `export_projection()` — a certification index plus a flat
  downgrade report — rather than cloning status text.
