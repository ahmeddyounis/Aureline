# Fixtures: M5 marketplace fact-views

This directory contains fixture metadata for the `m5_marketplace_fact_views` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-marketplace-fact-views.json`

## Coverage

- Eight result rows cover every marketed package kind — `first_party_framework_pack`,
  `docs_pack`, `local_model_pack`, `signed_recipe_pack`, `template_artifact`,
  `bridge_backed_package`, `side_loaded_package`, and `mirrored_registry_variant` —
  so one information architecture is proven across all claimed M5 artifact families.
- Each row carries a `governance_family_ref` that resolves to its row in
  `artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json`, and each has a
  matching detail fact grid that reproduces its facts and carries its own provenance,
  permission, compatibility, activation, rollback, and support-export refs.
- Source class covers `first_party`, `verified_partner`, `community`, and
  `unverified`; runtime origin covers all six signing classes; bridge/native state
  covers `native`, `bridge_backed`, and `local_model_hosted`; mirror posture covers
  `direct_first_party`, `enterprise_mirrored`, `private_registry`, and
  `manually_imported`; discovery channel covers `public_registry`,
  `enterprise_mirror`, `private_registry`, and `manual_import`; lifecycle covers all
  seven states; evidence freshness covers `current`, `stale`, `expired`, and
  `unknown`.
- Disclosure level covers `standard`, `caution`, and `heightened`, and each of the
  five disclosure reasons — `reduced_provenance`, `mirrored_or_private_distribution`,
  `evidence_not_current`, `support_narrowed`, and `non_native_runtime` — is exercised
  by at least one row. Every row's `disclosure_level` and `disclosure_reasons` equal
  the recomputation from its facts.
- The guardrail is proven in both directions: the mirrored first-party variant and
  the private-registry and manually-imported listings keep every fact and backing ref
  and widen their disclosure rather than collapsing fields, and reduced-provenance
  listings (the community template and the unverified side-loaded package) widen to
  `heightened`.
- Two compare views set listings side by side: a framework pack served directly
  versus its mirrored copy, and an enterprise-mirror, a private-registry, and a
  manual-import listing across redistribution flows. Each compare entry reproduces its
  row's facts exactly.
