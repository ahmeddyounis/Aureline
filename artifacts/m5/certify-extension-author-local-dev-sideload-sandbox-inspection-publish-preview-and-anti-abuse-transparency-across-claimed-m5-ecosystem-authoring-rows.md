# M5 author-side certification — human-readable rendering

Human-readable rendering of the canonical M5 author-side certification. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/ecosystem/m5/m5-author-certification.json`.

## Per-family author-certification row

| Family | Source | Signing | Origin | Binding | Declared | Effective badge | Install claim | Author claim | Readiness | Disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| first_party_framework_pack | first_party | signed_verified | published_registry_backed | not_applicable | enterprise_approved | enterprise_approved | fully_supported | fully_supported | ready_to_publish | **certified** |
| docs_pack | first_party | signed_verified | published_registry_backed | not_applicable | verified_publisher | verified_publisher | best_effort_supported | best_effort_supported | publishable_with_warnings | **conditionally_certified** |
| local_model_pack | community | unsigned_local_dev | local_dev_workspace | bind_review_required | verified_publisher | **unsigned_local_only** | community_supported | **unsupported** | publishable_with_warnings | **downgraded** |
| signed_recipe_pack | partner | signed_verified | **local_dev_workspace** | stay_local | verified_publisher | **unsigned_local_only** | best_effort_supported | **unsupported** | blocked_from_publish | **uncertified** |
| template_artifact | private_registry | signed_unverified | published_registry_backed | bound_published_identity | registry_bound | registry_bound | best_effort_supported | **unsupported** | blocked_from_publish | **uncertified** |
| bridge_backed_package | bridge_backed | signed_verified | published_registry_backed | not_applicable | verified_publisher | verified_publisher | best_effort_supported | **unsupported** | blocked_from_publish | **uncertified** |
| side_loaded_package | side_loaded | unsigned_sideload | sideloaded_workspace | stay_local | registry_bound | **unsigned_local_only** | unsupported | unsupported | blocked_from_publish | **uncertified** |
| mirrored_registry_variant | mirrored_registry | revoked_signature | mirror_backed | not_applicable | enterprise_approved | **unsigned_local_only** | best_effort_supported | **unsupported** | withheld_quarantined | **uncertified** |

## Trust never inherits

- **signed_recipe_pack** — signed and verified, but built in a local-dev workspace; the
  origin caps the effective badge to `unsigned_local_only`, proving a package never inherits
  a trusted badge just because the machine holds a trusted key.
- **local_model_pack** (unsigned local-dev), **side_loaded_package** (unsigned side-load),
  and **mirrored_registry_variant** (revoked signature) all render `unsigned_local_only`
  despite declaring stronger badges.
- Every row renders no stronger than the author-and-publish-preview publish gate grants the
  same family.

## The marketed row narrows automatically

- **local_model_pack** — a local-dev build caps trust to local-only, so the author-side
  ceiling lands at unsupported and the community install claim narrows to unsupported with
  `author_claim_below_install_claim`.
- **template_artifact**, **bridge_backed_package**, **signed_recipe_pack**, and
  **mirrored_registry_variant** — a blocked or withheld publish gate collapses the
  best-effort install claim to unsupported.

## Publish preview stays a review, not a linter

- **docs_pack** — disclosed warnings hold the author lane at best-effort
  (`conditionally_certified`), matching the install claim rather than blocking.
- **signed_recipe_pack** (permission widening), **template_artifact** (runtime-class
  widening), and **bridge_backed_package** (external executable) each carry a
  `fresh_review_required` reload lane and a blocked publish gate, so authority never widens
  through hot reload without a fresh review.

## The unhappy paths are exercised

- **template_artifact** — a stale sandbox-inspection lane and stale evidence
  (`lane_stale`, `evidence_not_current`).
- **side_loaded_package** — a failed local-dev build and a missing anti-abuse lane
  (`lane_failed`, `lane_missing`); the install claim was already unsupported, so no further
  downgrade applies.
- **mirrored_registry_variant** — a quarantine hold withholds the family
  (`quarantine_hold`).

## Summary

- 8 families, one author-certification entry each — no author-lane decision disappears from
  the board.
- 1 certified, 1 conditionally certified, 1 downgraded, and 5 uncertified.
- 5 rows apply a downgrade below the install claim; 1 uncertified side-load applies none
  because its install claim was already unsupported.
- 4 rows render `unsigned_local_only`; 1 row ends fully supported.
- Every entry resolves to a real author-matrix row and a real install-certification entry,
  and every published trust posture, support class, disposition, and downgrade path is
  recomputed from the entry's facts.
