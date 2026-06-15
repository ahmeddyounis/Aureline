# M5 anti-abuse, continuity-history, and repro-export board — human-readable rendering

Human-readable rendering of the canonical M5 anti-abuse, continuity-history, and repro-export
board. This row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/ecosystem/m5/m5-anti-abuse-and-repro.json`.

## Per-family transparency row

| Family | Origin | Signing | Ranking | Quarantine history | Publisher continuity | Repro export | Bind | Disposition | Rendered badge |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| first_party_framework_pack | published_registry_backed | signed_verified | trust_led | clean | continuous | complete | not_applicable | visible_clean | enterprise_approved |
| docs_pack | published_registry_backed | signed_verified | trust_led | clean | continuous | complete | **bound_published_identity** | visible_clean | **registry_bound** |
| local_model_pack | local_dev_workspace | unsigned_local_dev | trust_led | clean | continuous | **incomplete** | **bind_review_required** | visible_clean | **unsigned_local_only** |
| signed_recipe_pack | local_dev_workspace | signed_verified | trust_led | clean | continuous | complete | stay_local | visible_clean | **unsigned_local_only** |
| template_artifact | published_registry_backed | signed_unverified | trust_led | clean | **publisher_transferred_disclosed** | complete | not_applicable | visible_with_history_disclosure | registry_bound |
| bridge_backed_package | published_registry_backed | signed_verified | **anti_abuse_led** | **prior_action_disclosed** | continuous | complete | not_applicable | visible_with_history_disclosure | verified_publisher |
| side_loaded_package | sideloaded_workspace | unsigned_sideload | **anti_abuse_led** | clean | continuous | complete | **bind_review_required** | visible_clean | **unsigned_local_only** |
| mirrored_registry_variant | mirror_backed | revoked_signature | **anti_abuse_led** | **currently_withheld** | **verified_publisher_lost** | complete | not_applicable | **withheld_quarantined** | **unsigned_local_only** |

## Ranking and anti-abuse reasons

- **first_party_framework_pack** — trust-led: conformance, security, publisher, and
  maintenance signals with no vanity metrics.
- **docs_pack** — trust-led with an `install_count_popularity` chip that does not dominate
  three substantive signals.
- **bridge_backed_package** — anti-abuse-led: an `anti_abuse_rate_limited` demotion leads the
  ranking even though the package is verified.
- **side_loaded_package** — anti-abuse-led: an `anti_abuse_ranking_demoted` demotion leads.
- **mirrored_registry_variant** — anti-abuse-led: an `anti_abuse_quarantined` chip reflects the
  current withholding directly in the ranking.

## Quarantine/removal and publisher history

- **template_artifact** — publisher transfer disclosed on the visible listing.
- **bridge_backed_package** — quarantined then cleared; the prior action stays disclosed.
- **mirrored_registry_variant** — published, verified, then verified-publisher lost, removed,
  reinstated, and quarantined again → currently withheld and verified-publisher-lost, both
  surfaced on the board rather than hidden in a moderation tool.

## Repro export packets

- Every family carries a self-contained repro export with package id, digest, host ABI,
  redacted logs, conformance results, and a manifest ref — no raw supervisor traces, no paid
  service.
- **local_model_pack** — discloses an `incomplete` export because conformance results have not
  been generated for the in-development build, rather than faking completeness.
- **mirrored_registry_variant** — keeps a complete, self-contained repro export available for
  investigation even while quarantined and withheld.

## Local-to-published rebinding

- **docs_pack** — completed a bind-published-identity review; bound to a published identity and
  capped at `registry_bound` rather than leaping to the verified-publisher badge it declares.
- **local_model_pack**, **side_loaded_package** — local/sideload-to-registry rebinds pending an
  explicit bind-published-identity review; each carries its review ref and renders local-only
  until the review completes.

## Non-inheritance

- **signed_recipe_pack** — signed and verified, but in a local-dev workspace; renders
  `unsigned_local_only`, proving a package never inherits a trusted badge just because the
  machine holds a trusted key.
- **local_model_pack**, **side_loaded_package**, **mirrored_registry_variant** — capped to
  `unsigned_local_only` despite declaring stronger badges.

## Summary

- 8 families, one transparency row each — no anti-abuse, history, or repro truth disappears
  from the board.
- 5 rows are visible-clean, 2 are visible-with-history-disclosure, and 1 is
  withheld-quarantined.
- 1 row discloses a publisher transfer, 1 a prior quarantine, and 1 a verified-publisher loss.
- 3 rows are anti-abuse-led, 5 trust-led; no row is vanity-dominated.
- 7 rows carry a complete repro export, 1 discloses an incomplete one; all are self-contained.
- 2 rows are mid local-to-published rebind (pending review), 1 is bound to a published identity.
- 4 rows render local-only; every row renders no stronger than the publish-preview gate would
  grant, so the transparency board and the publish preview project one trust truth.
