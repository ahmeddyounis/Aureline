# M5 publish-preview sheets — human-readable rendering

Human-readable rendering of the canonical M5 publish-preview sheet set. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/ecosystem/m5/m5-publish-preview.json`.

## Per-family publish-preview sheet

| Family | Version (bump) | Channel | Signer / namespace | Published badge | Readiness | Blockers (source) | Warnings (source) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| first_party_framework_pack | 2.3.0 → 2.4.0 (minor) | stable | signed_verified / enterprise_managed | enterprise_approved | **ready_to_publish** | — | — |
| docs_pack | 1.2.0 → 1.2.1 (patch) | beta | signed_verified / publisher_verified | verified_publisher | blocked | version_bump (undersized) | docs_completeness |
| local_model_pack | 0.9.0 → 1.0.0 (major) | canary | unsigned_local_dev / publisher_owned | **unsigned_local_only** | blocked | conformance_kit (not_run), manifest_diff (widening), hot_reload_review | signer_identity (provenance) |
| signed_recipe_pack | 3.1.4 → 3.2.0 (minor) | stable | signed_verified / **transfer_pending** | **registry_bound** | blocked | schema_validation, template_sample_completeness, channel_selection (clean) | manifest_diff (narrowed), namespace (transfer) |
| template_artifact | 1.0.0 → 1.0.1 (patch) | edge | signed_unverified / publisher_owned | registry_bound | **publishable_with_warnings** | — | performance_smoke, signer_identity, anti_abuse (loss history) |
| bridge_backed_package | 4.1.0 → 4.1.0 (no_bump) | internal | signed_verified / publisher_verified | verified_publisher | blocked | manifest_diff (widening), version_bump (missing), hot_reload_review | conformance_kit, registry_policy |
| side_loaded_package | 2.0.0 → 1.9.0 (downgrade) | edge | unsigned_sideload / **unclaimed** | **unsigned_local_only** | blocked | accessibility_smoke, version_bump (downgrade), namespace (unclaimed), anti_abuse (undisclosed) | signer_identity (provenance) |
| mirrored_registry_variant | 5.2.0 → 5.2 (invalid) | beta | revoked_signature / **mismatch** | **unsigned_local_only** | **withheld_quarantined** | schema_validation (not_run), registry_policy, version_bump (invalid), signer_identity (revoked), namespace (mismatch), channel_selection (signed) | — |

## Blockers versus warnings, named by source

- **first_party_framework_pack** — all seven gates pass or are not applicable, the minor
  bump covers its feature-level diff, and nothing widens; it is ready to publish to stable.
- **docs_pack** — adds a feature-level page section but proposes only a patch bump, so the
  **version-bump** gate blocks; a docs-completeness warning is disclosed separately. The
  block comes from the version gate, not the docs gate.
- **signed_recipe_pack** — **schema validation** and **template/sample completeness** both
  fail; a narrowed permission and a mid-transfer namespace are warnings; the stable channel
  refuses a release that still carries warnings, so the channel adds its own blocker.

## Manifest diffs and version bumps

- A covered bump (framework, minor over feature), an undersized bump (docs, patch under
  feature), a missing bump (bridge, no bump under breaking), a downgrade (side-loaded), and
  an invalid version (mirrored) are all proven.

## Widening forces a fresh review

- **local_model_pack** — widens the runtime class and adds a permission; both the manifest
  widening and the widening hot reload block until freshly reviewed.
- **bridge_backed_package** — adds an external executable; the manifest widening and the
  widening hot reload block until freshly reviewed.

## Signer / namespace / channel truth

- **signed_recipe_pack** — signed and verified, but a mid-transfer namespace caps the badge
  to registry-bound.
- **side_loaded_package** (unclaimed namespace) and **mirrored_registry_variant** (mismatch)
  publish unsigned-local-only and never inherit a trusted badge.
- **mirrored_registry_variant** — the beta channel refuses an unsigned-local-only release, so
  the channel adds a `channel_requires_signed_release` blocker on top of the quarantine.

## Summary

- 8 families, one publish-preview sheet each.
- 1 ready to publish, 1 publishable-with-warnings, 5 blocked, 1 withheld (quarantined).
- 6 sheets carry at least one blocker; 6 carry at least one warning.
- 3 publish unsigned-local-only; 3 publish a verified-publisher or enterprise-approved badge.
- 2 sheets carry an unreviewed manifest widening; 2 are blocked by a namespace finding; 2 are
  blocked by a channel finding; 1 is quarantined.
- Every sheet publishes no stronger than the author-lane publish gate would grant, so the
  publish preview and the author lane project one trust truth.
