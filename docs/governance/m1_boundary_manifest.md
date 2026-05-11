# Internal boundary manifest and open/local capability matrix (M1 draft)

Status: seeded. Owner: `@ahmeddyounis`.

This page is the reviewer entry point for Aureline's internal-boundary
truth for the protected M1 surfaces. It exists so that, for every
protected M1 surface (shell, editor, workspace, search, terminal,
support), the boundary class, deployment-profile posture, and residual
dependencies are explicit and inspectable from one place — without
re-encoding the vocabulary into a per-surface spreadsheet or a one-off
markdown table.

The narrative strawman for the broader product boundary lives at
[`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md);
the open-vs-paid boundary register lives at
[`/artifacts/governance/open_paid_boundary_rows.yaml`](../../artifacts/governance/open_paid_boundary_rows.yaml).
This page consumes those sources rather than forking their vocabulary.

## Canonical sources

- **Capability matrix (machine-readable):**
  [`/artifacts/governance/m1_open_local_capability_matrix.yaml`](../../artifacts/governance/m1_open_local_capability_matrix.yaml)
- **Boundary manifest schema (frozen vocabulary):**
  [`/schemas/governance/boundary_manifest.schema.json`](../../schemas/governance/boundary_manifest.schema.json)
- **Deployment-profile register (canonical):**
  [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
- **Residual-dependency ledger (canonical):**
  [`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml)
- **Open-vs-paid boundary rows (canonical):**
  [`/artifacts/governance/open_paid_boundary_rows.yaml`](../../artifacts/governance/open_paid_boundary_rows.yaml)
- **Product boundary strawman (narrative):**
  [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
- **Validation lane (unattended runner):**
  [`/tests/governance/m1_boundary_manifest_lane/run_m1_boundary_manifest_lane.py`](../../tests/governance/m1_boundary_manifest_lane/run_m1_boundary_manifest_lane.py)
- **Proof packet (evidence anchor):**
  [`/artifacts/milestones/m1/proof_packets/boundary_manifest.md`](../../artifacts/milestones/m1/proof_packets/boundary_manifest.md)
- **Validation capture (latest):**
  [`/artifacts/milestones/m1/captures/boundary_manifest_validation_capture.json`](../../artifacts/milestones/m1/captures/boundary_manifest_validation_capture.json)

## Boundary classes

Every row in the matrix carries exactly one `boundary_class`:

- **`local_only`** — MUST work with no network, no sign-in, no hosted
  service. The capability is reachable on every deployment profile and
  the row's `local_core_continuity` restates the invariant.
- **`provider_linked`** — optional user-supplied or customer-operated
  provider (BYOK / customer IdP / local model / signed-mirror catalog)
  attaches to the local surface. Without a provider, the surface
  narrows; it is not removed.
- **`managed`** — hosted-only convenience layered on top of a
  self-hostable protocol. The underlying workflow remains reachable
  via the self-hostable path; removing the managed form narrows the
  claim but does not reclassify the row.
- **`mirrored`** — reachable via a signed offline mirror or bundle in
  constrained-connectivity profiles. The mirror is the air-gapped
  profile's only source of registry / docs / catalog truth.
- **`unsupported`** — explicitly out of scope for this milestone.
  Reserved rows prevent implicit drift; rows in this class do not claim
  a hosted or managed deployment profile.

`local_only` rows MUST declare `residual_dependencies: []` so the
local-core invariant is inspectable from the row itself.

## Deployment profiles

The matrix consumes the five frozen deployment-profile ids from
[`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml):

| Profile id | Summary |
|------------------------|--------------------------------------------------------|
| `individual_local`     | Single user on a personal device. No account required. |
| `self_hosted`          | Customer-operated control plane, mirror, or registry.  |
| `enterprise_online`    | Enterprise online with vendor- or customer-managed services. |
| `air_gapped`           | No public internet. Offline bundles and local policy only.   |
| `managed_cloud`        | Vendor-operated SaaS control plane.                          |

Every capability row's `deployment_profiles` array MUST resolve to an
id in this register. The matrix never invents a new profile.

## Residual dependencies

Every row declares its residual-dependency surface using the closed
`dependency_class` vocabulary from
[`/artifacts/governance/residual_dependencies.yaml`](../../artifacts/governance/residual_dependencies.yaml).
For each residual-dependency class a row touches, it declares a
`per_profile_posture` map across every frozen deployment profile.

The matrix MUST NOT relax a stricter posture declared on the ledger.
In particular, when the ledger pins a dependency to `forbidden` on a
profile (for example, `remote_agent` is forbidden on `air_gapped`),
this matrix MUST honor the forbidden posture. The validation lane
fails the row otherwise.

The closed posture vocabulary is `required`, `optional`, `cached`,
`mirrored`, `forbidden`, `not_applicable_structural`. Silent posture
defaults are a validation failure.

## Carries-truth fields

Every row declares which truth fields it promises to surface honestly
when consumed by Help / About / service-health / support-export /
release tooling. The closed `truth_field_vocabulary` is:

- `build_identity`
- `channel`
- `install_mode`
- `client_scope`
- `service_health_state`
- `boundary_class`
- `residual_dependency_posture`
- `freshness_class`

Every row MUST declare at least one truth field; "inspectable
boundary truth" is non-trivial.

## Named runtime consumer

The matrix is only useful when at least one named consumer reads it.
The M1 named consumer is the Help/About truth-prototype parity matrix
at
[`/artifacts/docs/help_parity_matrix.yaml`](../../artifacts/docs/help_parity_matrix.yaml),
which is wired to render `boundary_class` and the per-row
`residual_dependency_posture` as client-scope badges and service-health
markers on the Help / About / service-health skeleton.

Additional consumers (release packet, support-bundle contract, the
unattended CI validator) are listed under
`consumer_bindings.additional_consumers` on the matrix.

## Surface coverage

The M1 matrix seeds at least one row for each of the protected M1
surface families:

- `shell`
- `editor`
- `workspace`
- `search`
- `terminal`
- `support`

The validation lane fails closed if any of these families is missing.
Later milestones may add additional surface-family members; removing a
member is breaking and requires a decision row.

## Protected walk

The reviewer protected walk for this lane is:

1. Open the Help / About / service-health skeleton on a protected
   dogfood row (see
   [`/artifacts/milestones/m1/dogfood_matrix.yaml`](../../artifacts/milestones/m1/dogfood_matrix.yaml))
   and verify the rendered `boundary_class` chip and per-row
   `residual_dependency_posture` match the matrix.
2. Open the support-bundle export and verify that
   `boundary_class` and `residual_dependency_posture` appear on each
   row of the export, sourced from this matrix (not re-keyed by
   surface).
3. Request a capability against the wrong boundary class — for
   example, attempt an `air_gapped` profile run that depends on
   `remote_agent`. Confirm the surface narrows or fails closed with
   the `boundary_manifest.posture_relaxes_ledger_forbidden` reason
   rather than a generic error.

## Failure drill

Run the unattended validation lane with `--force-drill` to replay any
of the seeded drills against pure data. The runner exits 0 only when
the row's named drill reproduces its `expected_check_id`:

```
python3 tests/governance/m1_boundary_manifest_lane/run_m1_boundary_manifest_lane.py \
    --repo-root . \
    --force-drill boundary_row:workspace.vfs_and_local_git:workspace_drill.posture_relaxes_forbidden
```

Seeded drills:

| Row | Drill id | Expected check id |
|-----|----------|-------------------|
| `boundary_row:shell.frame_and_status`         | `shell_drill.local_only_relaxed_to_managed`   | `boundary_manifest.local_only_must_have_no_residual_dependencies` |
| `boundary_row:editor.buffer_and_save`         | `editor_drill.local_core_continuity_dropped`  | `boundary_manifest.local_core_continuity_must_be_present`         |
| `boundary_row:workspace.vfs_and_local_git`    | `workspace_drill.posture_relaxes_forbidden`   | `boundary_manifest.posture_relaxes_ledger_forbidden`              |
| `boundary_row:search.local_index_and_grep`    | `search_drill.dependency_class_unknown`       | `boundary_manifest.residual_dependency_class_unknown`             |
| `boundary_row:terminal.local_pty_and_auth_callback` | `terminal_drill.boundary_class_unknown` | `boundary_manifest.boundary_class_unknown`                        |
| `boundary_row:support.bundle_export_and_about` | `support_drill.truth_field_unknown`          | `boundary_manifest.truth_field_unknown`                           |

## Cannot close if

The lane stays open if:

- the matrix does not validate against the schema;
- any seeded drill does not reproduce its declared
  `expected_check_id` when replayed with `--force-drill`;
- any required surface family is missing from the matrix;
- the named runtime consumer is missing or no longer reads the
  matrix's vocabulary;
- the latest validation capture is missing from
  `/artifacts/milestones/m1/captures/` or stale against the
  evidence-freshness rule in the proof packet.

## Out of scope

This page is the M1 *draft* of the internal boundary manifest. It does
not:

- claim entitlement-backend or org-switching truth (those live with
  the managed-control-plane lane);
- enforce managed-control-plane policy beyond the draft posture
  declarations;
- replace the broader product boundary strawman in
  [`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md);
- replace the open-vs-paid boundary register in
  [`/artifacts/governance/open_paid_boundary_rows.yaml`](../../artifacts/governance/open_paid_boundary_rows.yaml).
