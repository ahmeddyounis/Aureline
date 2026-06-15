# Fixtures: M5 anti-abuse, continuity-history, and repro-export board

This directory contains fixture metadata for the `m5_anti_abuse_and_repro_board`
packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-anti-abuse-and-repro.json`

## Coverage

- `first_party_framework_pack`, `docs_pack`, `local_model_pack`, `signed_recipe_pack`,
  `template_artifact`, `bridge_backed_package`, `side_loaded_package`, and
  `mirrored_registry_variant` are the only claimed artifact families, and each carries exactly
  one transparency row — so a package's anti-abuse, history, or repro truth never disappears
  from the board by losing its row.
- The ranking/anti-abuse reason chips are exercised across all twelve chip values and all four
  categories (trust, quality, anti-abuse demotion, vanity). The board proves both `trust_led`
  and `anti_abuse_led` rankings and never produces a `vanity_dominated` row: the `docs_pack`
  shows an install-count vanity chip that does not dominate three substantive signals, and the
  `bridge_backed_package`, `side_loaded_package`, and `mirrored_registry_variant` are
  anti-abuse-led because a demotion always leads the ranking.
- The quarantine/removal history is proven across all three states: `clean` (most families),
  `prior_action_disclosed` (the `bridge_backed_package` was quarantined then cleared), and
  `currently_withheld` (the `mirrored_registry_variant`). The mirrored variant's timeline
  exercises every history-event kind including `removed` and `reinstated`.
- The publisher continuity-or-loss state is proven across all three states: `continuous`,
  `publisher_transferred_disclosed` (the `template_artifact`), and `verified_publisher_lost`
  (the `mirrored_registry_variant`). A current quarantine is reflected in the ranking chips and
  a verified-publisher loss is surfaced on the visible board, so anti-abuse action is never
  hidden in a moderation-only tool.
- The repro-export packet is proven `complete` for seven families and `incomplete` for the
  in-development `local_model_pack` (conformance results not yet generated). Every export is
  self-contained — package id, digest, host ABI, redacted logs, conformance results, and a
  manifest ref, with no raw supervisor traces and no paid service.
- The local-to-published binding is proven across all four decisions:
  `not_applicable_published`, `stay_local` (the signed recipe pack stays local),
  `bind_review_required` (the local-model and side-loaded packs are mid-rebind), and
  `bound_published_identity` (the docs pack completed a review). A bound identity only appears
  on a published origin and is capped at `registry_bound` rather than leaping to a
  verified-publisher badge.
- The non-inheritance guardrail is proven directly: the `signed_recipe_pack` is
  `signed_verified` but lives in a `local_dev_workspace`, so it renders `unsigned_local_only` —
  a package never inherits a trusted badge just because the machine holds a trusted key. The
  `local_model_pack` (unsigned local-dev), `side_loaded_package` (unsigned sideload), and
  `mirrored_registry_variant` (revoked) also render `unsigned_local_only` despite declaring
  stronger badges.
- The publish-gate cross-check holds: every row renders no stronger than the publish-preview
  gate would grant the same family, so the transparency board and the publish preview project
  one trust truth.

Raw source code, raw absolute filesystem paths, raw wasm bytes, raw log bodies, raw crash
dumps, raw moderation note bodies, and raw signing-key material MUST NOT appear in any fixture.
