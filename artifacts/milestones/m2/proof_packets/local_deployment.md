# External alpha local deployment proof packet

```yaml
packet_id: review_packet:alpha.local_deployment.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.local_deployment
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T08:52:26Z
stale_after: P14D
source_revision: git:e37c76824aa67ede2adaed1b88dd8e7a1c7709aa
trigger_revision: alpha_local_deployment_contract_set@2026-05-15
exact_build_identity_ref: artifacts/build/build_identity.json
channel_context: preview
deployment_context:
  - individual_local
claim_change_state: no_claim_widening
same_change_truth_refs:
  docs_ref: docs/milestones/m2_alpha_scope.md
  migration_ref: docs/migration/source_ecosystem_coverage_matrix.md
  help_truth_ref: docs/docs/help_about_service_health_routes.md
  known_limits_ref: artifacts/feedback/external_alpha_known_limits.md
  support_export_ref: docs/support/support_bundle_contract.md
```

This packet promotes the local desktop deployment row to green by proving the
external alpha launch wedges resolve to the `individual_local` deployment
profile and do not widen into managed-cloud, broad remote, or helper-backed
service parity. It consumes the existing locality matrix, qualification matrix,
alpha wedge deployment rows, and Rust substrate that projects install and
managed-truth boundaries into product and support surfaces.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.local_deployment`
- Locality matrix: `artifacts/deployment/locality_matrix.yaml`
- Claimed locality row: `artifacts/deployment/locality_matrix.yaml#individual_local`
- Continuity case: `artifacts/deployment/locality_matrix.yaml#individual_local_baseline`
- Compatibility row: `artifacts/compat/qualification_matrix_seed.yaml#compat_row:deployment_profiles.boundary_manifest_truth`
- Alpha wedge matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Known limits: `artifacts/milestones/m2/known_limits_alpha.yaml`
- Public known-limits summary: `artifacts/feedback/external_alpha_known_limits.md`
- Latest capture: `artifacts/milestones/m2/captures/local_deployment_validation_capture.json`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_narrowed`

Evidence is fresh and scoped to the individual-local preview channel. The row
is green because both launch wedges bind their local desktop deployment claims
to `artifacts/deployment/locality_matrix.yaml#individual_local`, whose posture
is on-device processing, local-disk storage, single-user local tenancy, OS-store
key mode, no managed control-plane claim, and locally safe editing, save,
search, Git, tasks, docs inspection, export, and diagnostics.

The row remains deliberately narrowed by
`known_limit:external_alpha.deployment.local_or_helper_only`. This packet does
not claim managed-cloud daily-driver parity, broad remote attach parity,
helper-backed language-service parity, devcontainer helper parity, hosted
collaboration, managed sync, managed AI routing, or vendor control-plane
availability.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/local_deployment.md` |
| Latest capture | `artifacts/milestones/m2/captures/local_deployment_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | `docs_claim_truth_proof`, `stale_after: P14D` |

## Locality Matrix Row Claims

| Claim | Cited row | Evidence |
|---|---|---|
| TS/JS local desktop deployment | `artifacts/deployment/locality_matrix.yaml#individual_local` | `deployment.alpha.ts_js.local_desktop` in `artifacts/milestones/m2/alpha_wedge_matrix.yaml` |
| Python local desktop deployment | `artifacts/deployment/locality_matrix.yaml#individual_local` | `deployment.alpha.python.local_desktop` in `artifacts/milestones/m2/alpha_wedge_matrix.yaml` |
| Local continuity case | `artifacts/deployment/locality_matrix.yaml#individual_local_baseline` | `fixtures/deployment/continuity_cases/individual_local_baseline.json` |
| Deployment-profile qualification | `artifacts/compat/qualification_matrix_seed.yaml#compat_row:deployment_profiles.boundary_manifest_truth` | Compatibility row requires named profile ids and drill ids instead of prose-only scope |

## Protected Proof Path

Run the substrate and scope checks cited by the capture:

```sh
cargo test -p aureline-install --test topology_alpha
cargo test -p aureline-shell --test managed_region_residency_alpha
python3 ci/check_alpha_scope.py --repo-root .
```

## Coverage

The capture verifies:

- `individual_local` is present in `artifacts/deployment/locality_matrix.yaml`
  with `processing_location_class: on_device_only`,
  `storage_location_class: device_local_disk`,
  `tenant_org_scope_class: single_user_local`, and
  `key_mode_class: os_store`;
- all managed control-plane service classes for the local row are
  `not_applicable` except local docs-pack inspection, so the packet does not
  fabricate managed service outages or managed availability;
- all local data-plane capabilities listed by the row are
  `available_local_safe`;
- `deployment.alpha.ts_js.local_desktop` and
  `deployment.alpha.python.local_desktop` both cite
  `artifacts/deployment/locality_matrix.yaml#individual_local` and the local
  deployment scoreboard row;
- helper-backed deployment rows remain owned by
  `scoreboard_row:alpha_scope.helper_backed_deployment`; and
- managed-cloud daily-driver and broad remote parity remain out of scope under
  `known_limit:external_alpha.deployment.local_or_helper_only`.

## Substrate Consumed

- `crates/aureline-install/src/topology/` owns install topology validation,
  cross-surface truth fingerprints, and metadata-only support-export projection
  for About, Update, diagnostics, install-review, CLI, and support surfaces.
- `crates/aureline-shell/src/managed_truth/` keeps region, residency, tenant,
  key-mode, control-plane, and data-plane truth explicit so local-only rows can
  avoid stronger managed claims while managed/provider-linked rows remain
  separately inspectable.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture added.
- Scoreboard row moved to `green` and remains `conditional_go`.
- Scoreboard row now cites the locality matrix, qualification matrix, owning
  packet, and latest capture.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
- `known_limit:external_alpha.deployment.local_or_helper_only` remains active
  and blocks managed-cloud or broad-remote wording.
