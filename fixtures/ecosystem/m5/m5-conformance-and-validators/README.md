# Fixtures: M5 conformance scorecards, validators, and reference-workspace linkage

This directory contains fixture metadata for the `m5_conformance_and_validators`
packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-conformance-and-validators.json`

## Coverage

- Eight scorecards cover every marketed package kind — `first_party_framework_pack`,
  `docs_pack`, `local_model_pack`, `signed_recipe_pack`, `template_artifact`,
  `bridge_backed_package`, `side_loaded_package`, and `mirrored_registry_variant` — so
  one conformance model is proven across all claimed M5 artifact families.
- Conformance labels cover the full stable vocabulary — `native`, `bridge`, `partial`,
  `unsupported`, and `retest_pending` — and dispositions cover `certified`,
  `conditionally_certified`, and `uncertified`.
- The guardrail is proven both ways. `certified` scorecards (the first-party framework
  pack and the signed recipe pack) name an owner, link an archetype and a reference
  workspace, cite current conformance and compatibility evidence, and publish a support
  claim. `uncertified` scorecards withdraw their claim: the template artifact for stale
  evidence, the bridge-backed package for a validator failure, the side-loaded package
  for missing owner/archetype/reference-workspace/evidence, and the mirrored variant for
  a retest-pending label.
- The mirrored-registry variant shares the first-party framework pack's `owner_ref`,
  `archetype_ref`, and `reference_workspace_refs`, proving that even a fully-linked
  family is held to `unsupported` while its conformance result is `retest_pending`.
- Each scorecard's `certification_signals`, `certification_disposition`, and
  `effective_support_class` equal the values recomputed from its facts. Guardrail-class
  signals — `evidence_not_current`, `owner_missing`, `archetype_unlinked`,
  `reference_workspace_unlinked`, `conformance_evidence_missing`, `validator_failure`,
  `retest_pending`, and `unsupported` — each force an `uncertified` disposition whose
  effective support class collapses to `unsupported`.
- Every validator diagnostic is actionable: it carries a stable `code`, a `domain`, a
  `severity`, a `message`, and a concrete `remediation`, and is exported flat for issue
  reports and release evidence.

## Validation

The typed model in the `aureline-ecosystem` crate
(`m5_conformance_and_validators::M5ConformanceAndValidators::validate`) is canonical: it
parses the embedded packet, recomputes every scorecard's signals, disposition, and
effective support class, and asserts the summary counts. The JSON Schema at
`schemas/ecosystem/m5-conformance-and-validators.schema.json` validates the artifact's
shape and closed vocabularies.
