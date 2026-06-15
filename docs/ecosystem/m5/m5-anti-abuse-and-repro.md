# M5 anti-abuse, continuity-history, and repro-export board

This document describes the canonical packet that freezes the **post-publication
transparency** of each marketed M5 artifact family — the ranking and anti-abuse reasons, the
quarantine/removal history, the publisher continuity-or-loss state, and the reproducible
export packet that stay visible as a family moves through local-to-published, mirrored, and
sideload-to-registry flows. It is the user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-anti-abuse-and-repro.json` and the typed model in the
`aureline-ecosystem` crate (`m5_anti_abuse_and_repro`).

Where the [author-and-publish-preview matrix](m5-author-and-publish-preview.md) and the
[publish-preview sheet](m5-publish-preview.md) are the publish-control gate an author drives
**before** release, this packet is the lane that keeps ranking, anti-abuse, history, and
repro truth explicit **after** a family is published, mirrored, or rebound. It reuses the
same closed artifact-family, runtime-class, host/ABI, signing-state, trust-posture, and
workspace-origin vocabulary, so the transparency board, the publish gate, and the
marketplace describe the same artifact rather than a parallel synonym set.

## What each row carries

The packet carries one row for every claimed M5 artifact family. Each row answers:

- **What is it and how is it signed?** A `package_identity`, an opaque `source_path_ref`, a
  `runtime_class`, a `host_abi`, a workspace `origin`, a `signature_state`, a
  `declared_trust_posture`, and a `rendered_trust_posture`.
- **Why does it rank where it does?** A set of `ranking_reasons` chips — substantive trust
  and quality signals (`conformance_verified`, `security_review_passed`,
  `publisher_verified`, `maintained_actively`, `compatibility_current`, `docs_complete`),
  anti-abuse demotions (`anti_abuse_rate_limited`, `anti_abuse_ranking_demoted`,
  `anti_abuse_quarantined`), and popularity/vanity metrics (`install_count_popularity`,
  `star_rating_popularity`, `trending_velocity`) — plus a recomputed
  `ranking_explainability` of `trust_led`, `anti_abuse_led`, or the flagged
  `vanity_dominated`.
- **What is its history?** A sequenced `history_events` timeline whose folded
  `quarantine_history_state` (`clean`, `prior_action_disclosed`, `currently_withheld`) and
  `publisher_continuity_state` (`continuous`, `publisher_transferred_disclosed`,
  `verified_publisher_lost`) stay disclosed on the visible surface.
- **Can a build be reproduced?** A `repro_export` packet with `package_id`, `digest`,
  `host_abi`, `logs_ref`, `conformance_results_ref`, and `manifest_ref`, a `self_contained`
  flag, and a recomputed `state` (`complete` or `incomplete`).
- **Is it bound to a published identity?** A `bind_decision` of `not_applicable_published`,
  `stay_local`, `bind_review_required`, or `bound_published_identity`, with a
  `bind_review_ref`.
- **Is it visible?** A recomputed `transparency_disposition` of `visible_clean`,
  `visible_with_history_disclosure`, or `withheld_quarantined`.

## A row never inherits a trusted badge

The board caps the rendered trust posture by the signing state, the workspace origin, **and**
the registry-binding decision. A `local_dev_workspace` or `sideloaded_workspace` origin, an
unsigned or revoked signature, or a `stay_local`/`bind_review_required` binding each cap the
rendered badge at `unsigned_local_only`. So a recipe pack that is `signed_verified` on a
trusted machine but lives in a local-dev workspace renders `unsigned_local_only` — a package
never inherits a verified-publisher or enterprise-approved badge just because it was built on
the same machine as a trusted user. A freshly `bound_published_identity` family caps at
`registry_bound` rather than leaping to a verified-publisher badge.

## Vanity metrics never dominate the ranking

Install-count, star-rating, and trending metrics may appear as chips, but the recomputed
`ranking_explainability` is `trust_led` only when substantive signals are at least as many as
vanity metrics, and `anti_abuse_led` whenever any anti-abuse demotion is present. A row whose
vanity metrics outnumber its substantive signals computes `vanity_dominated` and fails
validation, so vanity metrics can be shown but never lead the decision.

## Anti-abuse and publisher loss stay disclosed, not hidden

A current quarantine must be reflected in the ranking chips (the `anti_abuse_quarantined`
chip is present exactly when the family is `currently_withheld`), so anti-abuse action is
never buried in a moderation-only tool. A prior, now-cleared quarantine, a publisher
transfer, and a verified-publisher loss all surface as `visible_with_history_disclosure` on
the visible board, and a currently-withheld family renders `withheld_quarantined`.

## Repro export is self-contained

Every `repro_export` packet must be `self_contained` — it carries the package id, digest,
host ABI, redacted logs, conformance results, and manifest refs needed to reproduce a build
without raw supervisor traces or a paid service. A row may disclose an `incomplete` export
when a content ref is genuinely unavailable, but it can never claim `complete` without all
three reproducible refs.

## Local-to-published rebinding is an explicit review

A package moving from a local or side-loaded workspace to a published registry identity must
pass through a bind-published-identity review: a `bind_review_required` or
`bound_published_identity` row carries its `bind_review_ref`, and a `bound_published_identity`
state can never appear on a still-local or side-loaded origin — the review must move it to a
published origin first.

## Cross-surface truth

The typed model exposes `export_projection()` for downstream surfaces — marketplace
discovery, authoring surfaces, diagnostics, support, and release surfaces — to render the
board rather than restating anti-abuse, history, or repro-export status text by hand. The
model's `cross_check_matrix()` proves every row renders no stronger than the publish-preview
gate would grant the same family, so the transparency board and the publish preview project
one trust truth.

## Freshness

The packet is current as of the `as_of` date embedded in the JSON artifact. The typed model
recomputes the rendered trust posture, the ranking explainability, the quarantine-history and
publisher-continuity states, the repro-export state, the transparency disposition, and the
summary counts from the observed facts and fails validation if the checked-in packet drifts.
