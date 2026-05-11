# Proof packet: M1 internal boundary manifest

Purpose: anchor proof captures for the unattended M1 lane that
validates the internal-boundary capability matrix against the
boundary-manifest schema and the canonical deployment-profile and
residual-dependency truth sources, and proves the matrix is consumable
by a named Help / About / service-health surface without re-encoding
the boundary or residual-dependency vocabulary.

Reviewer entry point:
[`/docs/governance/m1_boundary_manifest.md`](../../../docs/governance/m1_boundary_manifest.md).

Canonical sources (non-exhaustive):

- `artifacts/governance/m1_open_local_capability_matrix.yaml` —
  capability matrix the runner consumes; one row per protected M1
  surface family with `boundary_class`, `deployment_profiles`,
  `residual_dependencies` (with `per_profile_posture`),
  `carries_truth_fields`, and a named failure drill.
- `schemas/governance/boundary_manifest.schema.json` — boundary schema
  for the matrix; freezes closed vocabularies for `boundary_class`,
  `deployment_profile`, `residual_dependency_class`, `posture_class`,
  `surface_family`, `truth_field`, `manifest_status`, and
  `failure_drill_id`.
- `artifacts/governance/deployment_profiles.yaml` — canonical
  deployment-profile register; the matrix's
  `deployment_profile_vocabulary` MUST match this register verbatim.
- `artifacts/governance/residual_dependencies.yaml` — canonical
  residual-dependency ledger; the matrix's
  `residual_dependency_class_vocabulary` MUST be a subset of the
  ledger's `dependency_class_vocabulary`, posture values MUST resolve
  against the ledger's `posture_class_vocabulary`, and rows MUST NOT
  relax stricter postures declared on the ledger.
- `docs/product/boundary_manifest_strawman.md` — narrative strawman
  for the broader product boundary; the M1 matrix consumes its
  vocabulary, it does not fork it.
- `artifacts/governance/open_paid_boundary_rows.yaml` — open-vs-paid
  boundary register; the M1 matrix does not duplicate its rows.
- `tests/governance/m1_boundary_manifest_lane/run_m1_boundary_manifest_lane.py`
  — unattended runner that replays the matrix and emits the durable
  JSON capture.

Named runtime consumer:

- `artifacts/docs/help_parity_matrix.yaml` — Help/About truth-prototype
  parity matrix. Wired as the M1 named consumer through
  `consumer_bindings.named_runtime_consumer` on the matrix.

Live runtime consumers (read-only):

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

Validation captures:

- `artifacts/milestones/m1/captures/boundary_manifest_validation_capture.json`

Refresh: re-run the validation lane after a change to the capability
matrix, the boundary-manifest schema, the deployment-profile register,
the residual-dependency ledger, or the reviewer-facing landing page.

Closure rule: the lane stays open until the latest capture lands under
the governed proof root and every row reports PASS for closed-vocab
membership (`boundary_class`, `surface_family`, `manifest_status`,
`carries_truth_fields`, `failure_drill_id`), deployment-profile
membership, residual-dependency well-formedness (including per-profile
posture coverage and the anti-relax rule against the ledger), the
`local_only` rows' empty-residual invariant, the `local_core_continuity`
requirement, named-runtime-consumer existence, required surface
coverage, and its named failure drill — and the six required surface
families (`shell`, `editor`, `workspace`, `search`, `terminal`,
`support`) are all observed.
