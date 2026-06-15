# Fixtures: M5 author-side certification

This directory contains fixture metadata for the `m5_author_certification` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-author-certification.json`

## Coverage

- `first_party_framework_pack`, `docs_pack`, `local_model_pack`, `signed_recipe_pack`,
  `template_artifact`, `bridge_backed_package`, `side_loaded_package`, and
  `mirrored_registry_variant` are the only claimed package kinds, and each carries exactly
  one author-certification entry — no family inherits an author-lane decision from an
  adjacent one.
- Each entry resolves to a real author-and-publish-preview matrix row
  (`m5-author:<family>`) and a real install-side certification entry
  (`certify:<family>`), so the certification stays an aggregator rather than a parallel
  spreadsheet.
- Each entry carries one evidence record for every author drill lane — `local_dev_workspace`,
  `sideload_review`, `sandbox_inspection`, `publish_preview`, `reload_continuity`, and
  `anti_abuse_transparency` — so a row can never be certified by running a subset of the
  author drills.

## What the corpus proves

- **Trust never inherits.** The effective trust posture is recomputed as the weakest of the
  signing-state, workspace-origin, and registry-binding ceilings. An unsigned local-dev
  build (`local_model_pack`), a **signed** recipe built in a local-dev workspace
  (`signed_recipe_pack`), an unsigned side-load (`side_loaded_package`), and a revoked
  mirror variant (`mirrored_registry_variant`) all render `unsigned_local_only`, proving a
  package never inherits a verified/enterprise badge just because the machine holds a
  trusted key. No entry renders a stronger badge than the publish-preview gate grants the
  same family.
- **The marketed row narrows automatically.** When the author-side ceiling lands below the
  end-user install claim, the entry records `author_claim_below_install_claim` and applies a
  downgrade. `local_model_pack` narrows a community install claim to unsupported because its
  local-dev build caps trust to local-only.
- **Publish preview stays a review, not a linter.** Blocker/warning truth flows straight
  from the publish gate: `docs_pack` is `conditionally_certified` on disclosed warnings, and
  `signed_recipe_pack`, `template_artifact`, `bridge_backed_package`, and
  `side_loaded_package` are `uncertified` because the publish gate is blocked.
- **Widening hot reloads force a fresh review.** `signed_recipe_pack` (permissions),
  `template_artifact` (runtime class), and `bridge_backed_package` (external executable) each
  carry a `fresh_review_required` reload lane and a blocked publish gate.
- **Drills cover the unhappy paths.** `template_artifact` exercises a stale lane and stale
  evidence; `side_loaded_package` exercises a failed build and a missing anti-abuse lane;
  `mirrored_registry_variant` exercises a quarantine hold.
- **Every disposition appears.** `certified` (first-party framework), `conditionally_certified`
  (docs), `downgraded` (local-model), and `uncertified` (the five remaining families).

## Regeneration

The packet is hand-authored and validated by `M5AuthorCertification::validate`, which
recomputes every entry's certification signals, disposition, effective trust posture,
effective author support class, and downgrade path. Any drift between a stored value and the
recomputation is a test failure in
`crates/aureline-ecosystem/src/m5_author_certification/tests.rs`.
