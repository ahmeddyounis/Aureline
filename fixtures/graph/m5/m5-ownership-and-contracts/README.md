# Fixtures: M5 ownership-and-contracts packet

This directory contains fixture metadata for the `m5_ownership_and_contracts_packet`.

The canonical packet is checked in at:

`artifacts/graph/m5/m5-ownership-and-contracts.json`

and validated by the typed model in the `aureline-graph` crate (`m5_ownership_and_contracts`) and
the JSON Schema at `schemas/graph/m5-ownership-and-contracts.schema.json`.

## Coverage

- **Distinct source classes.** The corpus declares descriptors of every source class — `curated`,
  `policy_derived`, `imported`, and `heuristic` — so a derived hint never collapses into curated
  truth. Every non-`curated` descriptor carries an explicit `source_reason`, and the `auth` owner
  exists as both a curated and (implicitly) lower-precedence answer so `authoritative_descriptor`
  resolves curated truth first.
- **Distinct human roles.** The corpus exercises `owner`, `reviewer`, `maintainer`,
  `support_contact`, and `change_control` separately rather than collapsing them into one generic
  owner field. The `change_control` descriptor carries its link in `change_control_url`; no other
  role does.
- **Inference never overwrites curated truth.** A refined heuristic reviewer supersedes an earlier
  heuristic reviewer (allowed — both are inferred), but no `imported` or `heuristic` descriptor
  supersedes a `curated` or `policy_derived` one.
- **Visibility never widens.** The corpus carries `public`, `internal`, and `private` descriptors.
  The restricted (`private`) security owner is visible in the in-product review hint but is withheld
  from the support export and redacted entirely from the export projection. The support-export
  binding carries every export-safe descriptor and no private one.
- **Carried beyond one panel.** Each of `review_hint`, `explainer_card`, `onboarding_context`,
  `ai_ownership_suggestion`, and `support_export` carries exactly one binding, stamped with the
  active snapshot and scope, and each preserves source-class labels.
- **Upstream provenance.** The packet binds to the canonical graph-depth governance matrix
  (`artifacts/graph/m5/m5-graph-governance.json`), the workset-scope packet
  (`artifacts/graph/m5/m5-workset-scope.json`), and the topology-identity packet
  (`artifacts/graph/m5/m5-topology-identity.json`) whose node identity space it reuses.

## Guardrails proven

- A non-curated descriptor with no `source_reason` fails validation (`MissingSourceReason`).
- A `change_control` descriptor with no link, or any other role that carries one, fails validation
  (`ChangeControlWithoutLink`, `NonChangeControlWithLink`).
- An `imported` or `heuristic` descriptor that supersedes a `curated` or `policy_derived` descriptor
  fails validation (`InferenceOverwritesCurated`).
- A binding that flattens source labels, or carries a descriptor beyond its visibility ceiling,
  fails validation (`SourceLabelsNotPreserved`, `VisibilityExceedsBinding`).
- An export-safe descriptor not carried by the support-export binding, or a `private` descriptor
  carried by it, fails validation (`ExportSafeDescriptorMissingFromSupportExport`,
  `PrivateDescriptorInSupportExport`).
