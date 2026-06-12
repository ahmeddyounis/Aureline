# M5 bundle scorecards — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-bundle-scorecards.json`. The full contract lives in
`docs/workspace/m5/m5-bundle-scorecards.md`; the typed model lives in the
`aureline-workspace` crate (`m5_bundle_scorecards`).

This artifact freezes one machine-readable **compatibility scorecard** per claimed M5 launch
bundle. Each scorecard scores a workflow-bundle manifest version and records supported platforms,
bundle dependencies with lifecycle markers, imported-versus-native confidence, certified
reference-workspace linkage, and current evidence freshness. From the claimed class, the confidence,
and the freshness it computes an **effective class** so imported or approximate behavior can no
longer inherit native or certified language by inertia.

## Scorecard roll-up (as of 2026-06-11)

| Bundle | Wedge | Manifest | Claimed | Confidence | Freshness | Bounded | Effective | Certified? |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `m5-wbm:notebook` | notebook | 1.0.0 | certified | native | fresh | no | **certified** | yes |
| `m5-wbm:data-and-api` | data/API | 1.2.0 | certified | native | fresh | yes | **certified** | yes |
| `m5-wbm:profiler` | profiler | 0.9.0 | certified | bridged | stale | no | **probable** | no |
| `m5-wbm:framework-pack` | framework-pack | 1.1.0 | probable | native | aging | no | **probable** | no |
| `m5-wbm:docs` | docs | 1.0.0 | community | native | fresh | no | **community** | no |
| `m5-wbm:companion` | companion | 0.4.0 | imported | approximated | missing | yes | **imported** | no |
| `m5-wbm:sync-handoff` | sync-handoff | 0.3.0 | preview | unverified | aging | no | **preview** | no |
| `m5-wbm:local-folder` | local folder | 0.1.0 | local_draft | native | fresh | no | **local_draft** | no |

## What the scorecards prove

- **Every claimed bundle has a scorecard.** Each of the eight M5 launch wedges resolves to one
  scorecard that explains scope, compatibility, confidence, and freshness in a single canonical
  packet — every claimed and every effective class is exercised.
- **No inheritance of certified language by inertia.** The profiler bundle *claims* `certified` but
  its evidence is `stale` and its behavior is only `bridged`, so the effective class narrows to
  `probable` and it does not present as certified. The companion bundle is imported and
  `approximated` with no recorded evidence, so it stays `imported`. Approximate or unverified
  confidence never presents as certified, and stale or missing evidence never presents as certified.
- **Public copy narrows when proof is stale or bounded.** The certified data/API bundle is
  platform-bounded to Linux and macOS, so it carries a caveat narrowing its certified claim to its
  supported platforms. Every narrowed scorecard carries a caveat; only a certified, native, fresh,
  full-platform bundle escapes one.
- **The classes stay distinct.** `certified`, `probable`, `community`, `imported`, `preview`, and
  `local_draft` are all present without one collapsing into another.
- **Dependencies are disclosed, not buried.** Across the corpus, six dependencies sit on non-stable
  lifecycle stages — one each of `policy_gated`, `bounded_platform`, `preview`, `labs`, and a
  `mirror_only`/`labs` pair on the companion bundle — surfaced as lifecycle markers rather than
  hidden.
- **Joined to existing proofs, not another silo.** Every scorecard carries a `manifest_ref` into the
  workflow-bundle manifest, a `compatibility_scorecard_ref` into the compatibility-scorecard packet,
  an `archetype_cert_ref` into the archetype-certification packet, and a `reference_workspace_ref`
  into a certified reference workspace.
- **One object model, many consumers.** Every scorecard carries start-center, migration-center,
  help/About, release-center, docs/help, support-export, and diagnostics refs, so public copy
  ingests the same packet support and release tooling read.

## How it is validated

The typed model parses the embedded packet and runs `validate()`, which checks the closed
vocabularies, full wedge coverage, the recomputed effective class, the platform-bounded
recomputation and well-formed platform set, the certified-presentation rule, complete linkage and
consumer refs, per-dependency consistency, required caveats, and the recomputed summary. See
`crates/aureline-workspace/src/m5_bundle_scorecards/tests.rs`.
