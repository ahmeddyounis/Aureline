# Fixtures: M5 publish-preview sheet set

This directory contains fixture metadata for the `m5_publish_preview_sheet_set`
packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-publish-preview.json`

## Coverage

- `first_party_framework_pack`, `docs_pack`, `local_model_pack`,
  `signed_recipe_pack`, `template_artifact`, `bridge_backed_package`,
  `side_loaded_package`, and `mirrored_registry_variant` are the only claimed
  artifact families, and each carries exactly one publish-preview sheet — so a
  family never inherits a publish decision from an adjacent one.
- Every sheet reports a result for all seven named publish gates — schema
  validation, the conformance kit, accessibility smoke, performance smoke, docs
  completeness, template/sample completeness, and registry policy — so a publish
  can never hide a gate by omitting it. Each blocker and warning names the gate
  it came from.
- Blockers versus warnings stay explicit: the `first_party_framework_pack` is
  ready-to-publish with no findings; the `template_artifact` is
  publishable-with-warnings (performance-smoke warning, unverified provenance, and
  a disclosed publisher-loss history); the `docs_pack`, `local_model_pack`,
  `signed_recipe_pack`, `bridge_backed_package`, and `side_loaded_package` are
  blocked; and the `mirrored_registry_variant` is withheld because it is
  quarantined.
- The version-bump gate is proven in every direction: a covered minor bump
  (framework pack), an undersized bump (docs pack, patch under a feature change), a
  missing bump (bridge package, no bump under a breaking change), a downgrade
  (side-loaded package), and an invalid version (mirrored variant).
- The widening-review guardrail is proven directly: the `local_model_pack` widens
  both its runtime class and permissions and the `bridge_backed_package` adds an
  external executable; each raises a `manifest_widening_unreviewed` blocker and,
  where a hot reload is in play, a `hot_reload_widening_unreviewed` blocker, so
  widening never reaches the registry without a fresh review.
- The signer/namespace truth is proven directly: the `signed_recipe_pack` is
  `signed_verified` but its namespace is mid-transfer, so the published badge is
  capped to `registry_bound`; the `side_loaded_package` has an unclaimed namespace
  and the `mirrored_registry_variant` a mismatched namespace, so both publish
  `unsigned_local_only` and never inherit a trusted badge.
- The channel consequences are explicit: the `signed_recipe_pack` targets the
  stable channel while carrying warnings, so the channel adds a
  `channel_requires_clean_release` blocker; the `mirrored_registry_variant` targets
  the beta channel while unsigned-local-only, so the channel adds a
  `channel_requires_signed_release` blocker.
- The publish-gate cross-check holds: every sheet publishes no stronger than the
  author-lane publish gate would grant the same family, so the publish preview and
  the author lane project one trust truth.

Raw manifest bodies, raw absolute filesystem paths, raw signing-key material, and
raw provider payloads MUST NOT appear in any fixture — the `manifest_diff_ref`,
`signer_identity_ref`, and `namespace_ref` are opaque refs only.
