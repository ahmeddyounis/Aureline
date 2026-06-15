# M5 evidence pointer — anti-abuse ranking reasons, quarantine/publisher-loss history, and repro export packets

Reviewer contract for the canonical M5 anti-abuse, continuity-history, and repro-export board
that keeps a package's ranking and anti-abuse reasons, quarantine/removal history, publisher
continuity-or-loss, and reproducible export packet visible across local-to-published,
mirrored, and sideload-to-registry flows for each marketed M5 ecosystem artifact family. This
row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-anti-abuse-and-repro.json`
- Boundary schema: `schemas/ecosystem/m5-anti-abuse-and-repro.schema.json`
- Reviewer contract: `docs/m5/ship-anti-abuse-ranking-reasons-quarantine-or-publisher-loss-history-and-repro-export-packets-for-local-to-published-or-mirrored-m5-ecosystem-flows.md`
- Human-readable rendering: `artifacts/m5/ship-anti-abuse-ranking-reasons-quarantine-or-publisher-loss-history-and-repro-export-packets-for-local-to-published-or-mirrored-m5-ecosystem-flows.md`
- Overview companion: `docs/ecosystem/m5/m5-anti-abuse-and-repro.md`
- Fixture corpus: `fixtures/ecosystem/m5/m5-anti-abuse-and-repro/`
- Owning crate module: `crates/aureline-ecosystem/src/m5_anti_abuse_and_repro/`

## Reuses the frozen publish-preview gate and shared vocabulary

The transparency board is the post-publication counterpart to the publish-preview gate
(`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`) and the marketplace fact views
(`artifacts/ecosystem/m5/m5-marketplace-fact-views.json`). The packet reuses the closed
artifact-family, runtime-class, host/ABI, signing-state, trust-posture, and workspace-origin
vocabulary frozen by those lanes — one row per marketed family — rather than minting a
parallel set, and each row links back to its marketplace, authoring, and publish-preview rows.

## What the board proves

- **Ranking and anti-abuse reasons are explainable.** Each row recomputes a
  `ranking_explainability` (`trust_led`, `anti_abuse_led`, or the flagged `vanity_dominated`)
  from its ranking-reason chips. Install-count, star-rating, and trending vanity metrics are
  shown but can never outnumber the substantive trust/quality signals, and any anti-abuse
  demotion leads the ranking. The fixture exercises all twelve chips and proves both
  trust-led and anti-abuse-led rankings while never producing a vanity-dominated row.
- **Quarantine/removal history is preserved.** Each row folds a sequenced timeline into a
  `quarantine_history_state` of `clean`, `prior_action_disclosed`, or `currently_withheld`.
  The fixture proves a clean history, a prior quarantine that was cleared (bridge pack), and a
  currently-withheld package that was published, lost its verified badge, removed, reinstated,
  and quarantined again (mirrored variant).
- **Publisher continuity or loss stays visible.** Each row recomputes a
  `publisher_continuity_state` of `continuous`, `publisher_transferred_disclosed`, or
  `verified_publisher_lost`. A publisher transfer (template artifact) and a verified-publisher
  loss (mirrored variant) both surface as a `visible_with_history_disclosure` disposition
  rather than being hidden in a moderation tool.
- **Repro export packets are complete and self-contained.** Each row carries a `repro_export`
  with package id, digest, host ABI, redacted logs, conformance results, and a manifest ref.
  Every export is `self_contained` — no raw supervisor traces, no paid service — and a row may
  disclose an `incomplete` export (local-model pack, with conformance not yet generated)
  rather than faking completeness.
- **Local-to-published rebinding is an explicit review.** A `bind_review_required` row carries
  its review ref (local-model and side-loaded packs), and a `bound_published_identity` row
  (docs pack) carries its review ref and sits on a published origin, capped at
  `registry_bound` rather than leaping to a verified-publisher badge.
- **Local builds never inherit a trusted badge.** The board caps the rendered trust posture by
  the signing state, the workspace origin, and the binding decision, so a `signed_verified`
  recipe pack in a local-dev workspace, an unsigned local-model pack, an unsigned side-loaded
  package, and a revoked mirrored variant all render `unsigned_local_only`.

## Narrowing / cross-check

- The typed model recomputes the rendered trust posture, the ranking explainability, the
  quarantine-history and publisher-continuity states, the repro-export state, the transparency
  disposition, and the summary counts from the observed facts; a checked-in packet that drifts
  fails `M5AntiAbuseReproBoard::validate`.
- `M5AntiAbuseReproBoard::cross_check_matrix` proves every row renders no stronger than the
  publish-preview gate would grant the same family, so the transparency board and the publish
  preview project one trust truth.
- Downstream surfaces consume `export_projection()` rather than cloning status text.
