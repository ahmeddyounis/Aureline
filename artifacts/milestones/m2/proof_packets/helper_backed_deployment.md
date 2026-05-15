# External alpha helper-backed deployment proof packet

```yaml
packet_id: review_packet:alpha.helper_backed_deployment.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.helper_backed_deployment
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T08:56:03Z
stale_after: P14D
source_revision: git:83b8677172af3d4bcc444a1062c8fe894e6144d8
trigger_revision: alpha_helper_backed_deployment_contract_set@2026-05-15
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

This packet promotes the helper-backed local service row to green by proving
the external alpha launch wedges can claim only local parser, language, task,
Git, and environment helpers behind explicit host-boundary cues. The row stays
narrowed to the `individual_local` deployment profile and does not claim
managed-workspace parity, broad remote attach parity, hosted notebook/kernel
parity, managed sync, managed collaboration, or a vendor control plane.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.helper_backed_deployment`
- Host-boundary matrix: `artifacts/remote/host_boundary_matrix.yaml`
- Covered host-boundary rows:
  - `artifacts/remote/host_boundary_matrix.yaml#row.local_host.in_process.read`
  - `artifacts/remote/host_boundary_matrix.yaml#row.local_host.local_rpc.mutation`
  - `artifacts/remote/host_boundary_matrix.yaml#row.bridged_helper.compat_bridge`
- Locality matrix: `artifacts/deployment/locality_matrix.yaml#individual_local`
- Alpha wedge claims:
  - `deployment.alpha.ts_js.helper_backed_language_services`
  - `deployment.alpha.python.devcontainer_helper`
- Known limit: `known_limit:external_alpha.deployment.local_or_helper_only`
- Latest capture: `artifacts/milestones/m2/captures/helper_backed_deployment_validation_capture.json`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_narrowed`

Evidence is fresh and scoped to helper-backed local services. The row is green
because both launch wedges bind helper-backed claims to
`artifacts/deployment/locality_matrix.yaml#individual_local`, and the cited
host-boundary rows require visible target, route, reachability, freshness,
authority, and adapter-confidence fields before a helper-backed action can be
presented as reviewable.

The row remains deliberately narrowed by
`known_limit:external_alpha.deployment.local_or_helper_only`. This packet does
not treat every `helper_backed` target-confidence projection as an alpha
deployment claim; only the covered host-boundary rows above are in scope for
the external alpha helper-backed deployment row.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/helper_backed_deployment.md` |
| Latest capture | `artifacts/milestones/m2/captures/helper_backed_deployment_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | `docs_claim_truth_proof`, `stale_after: P14D` |

## Host-Boundary Matrix Coverage

| Claim | Cited row | Why it is in scope |
|---|---|---|
| Local parser and read-only language inspection | `row.local_host.in_process.read` | Keeps local read helper evidence on the local host boundary with metadata-safe export posture. |
| Local task, Git, and environment helper mutation | `row.local_host.local_rpc.mutation` | Requires `authority_linkage_class` for local RPC mutation so helper actions stay tied to local user authority. |
| Compatibility-bridged helper adapter | `row.bridged_helper.compat_bridge` | Requires `adapter_confidence_placeholder` and a `local_host_boundary` plus `bridged_host_boundary` cue stack before bridge-backed helpers can be claimed. |

The following host-boundary rows are cited only as exclusions and do not count
as covered helper-backed deployment evidence: `row.remote_ssh.remote_rpc.mutation`,
`row.remote_agent.attach.mutation`, `row.managed_workspace.control_plane.mutation`,
`row.managed_workspace.remote_agent_attach.nested`, and
`row.notebook_kernel_remote.on_managed_workspace`.

## Locality Matrix Coverage

| Claim | Cited row | Evidence |
|---|---|---|
| TS/JS helper-backed language services | `artifacts/deployment/locality_matrix.yaml#individual_local` | `deployment.alpha.ts_js.helper_backed_language_services` in `artifacts/milestones/m2/alpha_wedge_matrix.yaml` |
| Python helper-backed environment or devcontainer | `artifacts/deployment/locality_matrix.yaml#individual_local` | `deployment.alpha.python.devcontainer_helper` in `artifacts/milestones/m2/alpha_wedge_matrix.yaml` |
| Local data-plane continuity | `artifacts/deployment/locality_matrix.yaml#individual_local_baseline` | `fixtures/deployment/continuity_cases/individual_local_baseline.json` |

The `individual_local` row keeps managed control-plane service classes
`not_applicable` for sync, registry, relay, AI broker, auth, policy, catalog,
and telemetry. Local editing, save, search, Git, tasks, docs inspection,
export, and diagnostics remain `available_local_safe`.

## Protected Proof Path

Run the substrate and scope checks cited by the capture:

```sh
cargo test -p aureline-runtime --test managed_alpha
cargo test -p aureline-runtime targets::tests::cards_display_local_and_helper_boundary_truth
cargo test -p aureline-shell host_boundary_cues
cargo test -p aureline-shell --test drift_truth_alpha
cargo test -p aureline-shell managed_workspace_labels
python3 ci/check_alpha_scope.py --repo-root .
```

## Substrate Consumed

- `crates/aureline-runtime/src/targets/mod.rs` projects execution contexts into
  target-confidence cards, host-boundary rows, review rows, and support exports
  without exposing raw paths, command lines, environment bodies, or secrets.
- `crates/aureline-runtime/src/managed_alpha/mod.rs` keeps runtime inspection
  labels bounded and requires helper-boundary visibility whenever a runtime
  crosses a helper or managed boundary.
- `crates/aureline-shell/src/host_boundary_cues/mod.rs` preserves visible
  host-boundary and target-identity cues through terminal handoff, reconnect,
  degraded, policy-blocked, and closed states.
- `crates/aureline-shell/src/managed_workspace_labels/mod.rs` explicitly labels
  managed-workspace lifecycle truth as a bounded prototype path, not a managed
  control-plane implementation.
- `crates/aureline-shell/src/drift_truth/mod.rs` exports helper, provider, and
  saved-artifact drift states as metadata-only support and review rows, with
  mutation blocked until compatibility or freshness is reviewed.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture added.
- Scoreboard row moved to `green` and remains `conditional_go`.
- Scoreboard row now cites the host-boundary matrix, locality matrix, review
  template, substrate files, owning packet, and latest capture.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
- `known_limit:external_alpha.deployment.local_or_helper_only` remains active
  and blocks managed-cloud, broad-remote, and hosted-workspace wording.
