# Beta Admission Matrix

This page is the reviewer-facing entrypoint for the M3 beta admission freeze.
The canonical truth lives in the JSON/YAML/Mermaid artifacts named below;
downstream M3 proof packets, dashboards, docs, support exports, and issue
templates should cite those rows instead of restating beta scope or cohort
rules.

## Canonical artifacts

- Claimed-surface register: `artifacts/milestones/m3/claimed_surface_register.json`
- Cohort guardrails: `artifacts/milestones/m3/cohort_guardrails.yaml`
- Dependency graph (Mermaid): `artifacts/milestones/m3/dependency_graph.mmd`
- Beta admission validator: `ci/check_beta_admission.py`
- Latest validation capture: `artifacts/milestones/m3/captures/beta_admission_validation_capture.json`

## Upstream contracts inherited from M2

- Alpha wedge matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Alpha exit-gate scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- Alpha dependency graph: `artifacts/milestones/m2/dependency_graph.yaml`
- Alpha archetype seed rows: `artifacts/certification/m2_archetype_seed_rows.yaml`
- Reference-workspace rows: `artifacts/compat/reference_workspace_rows.yaml`
- Archetype rubric: `artifacts/compat/archetype_rubric.yaml`
- Deployment locality matrix: `artifacts/deployment/locality_matrix.yaml`
- Known-limits contract: `docs/product/known_limits_contract.md`
- Public-interface versioning policy: `docs/governance/public_interface_versioning_policy.md`
- Certified-archetype report template: `docs/release/certified_archetype_report_template.md`

## Definition of green

The beta admission freeze is green when:

- the claimed-surface register names every claimed beta surface, archetype, and
  cohort with one canonical id;
- every claimed surface points to at least one primary cohort and at least one
  downgrade rule;
- every claimed archetype row binds to a `archetype_row:*` id from
  `artifacts/compat/reference_workspace_rows.yaml`;
- every cohort lists intake requirements, evidence classes, graduation
  criteria, and downgrade rules;
- held and explicitly out-of-scope rows require release-council review and a
  known-limit note before they can widen;
- the dependency graph cites the validator as a first consumer; and
- the validator passes.

Feature proof rows may still be `experimental` or evidence-pending; that is
intentional. This artifact freezes the claim surface and cohort routing before
the feature lanes turn those rows green.

## Claimed beta surfaces

| Surface | Lifecycle label | Primary cohort | Support class target at beta exit |
|---|---|---|---|
| Extension runtime, SDK, and publication path | beta | `cohort:extension_author` | supported |
| Debug, test, and task execution model | beta | `cohort:certified_archetype` | supported |
| Packaging, update, and rollback | beta | `cohort:design_partner_managed_pilot` | supported |
| Enterprise policy, proxy, and transport baseline | beta | `cohort:design_partner_managed_pilot` | supported |
| Support export, diagnostics, and recovery | beta | `cohort:design_partner_managed_pilot` | supported |
| Importer, migration parity, and known-gap honesty | beta | `cohort:design_partner_managed_pilot` | supported |
| Certified-archetype compatibility publication | beta | `cohort:certified_archetype` | supported |

## Claimed beta archetype rows

| Archetype row | Inherited from | Lifecycle label | Beta-exit support class target | Stable support class target |
|---|---|---|---|---|
| `archetype_row:ts_web_app_or_service` | M2 alpha | beta | supported | certified |
| `archetype_row:python_service_or_data_app` | M2 alpha | beta | supported | certified |
| `archetype_row:java_or_kotlin_service` | M3 (new) | beta | supported | certified |
| `archetype_row:rust_workspace` | M3 (new) | beta | supported | certified |
| `archetype_row:go_service_or_monorepo_slice` | M3 (new) | beta | supported | certified |
| `archetype_row:c_or_cpp_native_project` | M3 (new) | beta | supported | certified |

## Beta cohorts

| Cohort id | Earliest milestone | Purpose |
|---|---|---|
| `cohort:internal_dogfood` | M1 (inherited) | Continue daily-driver dogfood honesty during beta. |
| `cohort:external_alpha_migration` | M2 (inherited) | Continue alpha task-completion, migration, and known-limit evidence. |
| `cohort:extension_author` | M3 | Stabilize SDK/runtime and publication path on real add-ons. |
| `cohort:design_partner_managed_pilot` | M3 | Validate packaging, policy, proxy, support export, and rollback in real orgs. |
| `cohort:certified_archetype` | M3 | Back stable claims with current certified reference workspaces. |

## Explicitly not claimed at beta

- Managed-cloud daily-driver parity remains a scoped handoff surface; widening
  requires a release-council review and a known-limit note.
- Long-tail framework breadth beyond the named archetypes is explicitly not a
  beta claim.
- `.NET service or app` and notebook-first data workflow remain held for later
  (earliest M4) and must enter through the same change-control rules.

## How to validate

Run:

`python3 ci/check_beta_admission.py --repo-root .`

Optional machine-readable report:

`python3 ci/check_beta_admission.py --repo-root . --report artifacts/milestones/m3/captures/beta_admission_validation_capture.json`

## Update rules

1. Update `artifacts/milestones/m3/claimed_surface_register.json` first.
2. Update `artifacts/milestones/m3/cohort_guardrails.yaml` so every claimed
   surface points to at least one cohort id with current downgrade rules.
3. Update `artifacts/milestones/m3/dependency_graph.mmd` when an upstream
   contract, fixture, consumer, or validator changes.
4. Refresh this page when the user-visible beta claim surface changes.
5. Run the validator and refresh the capture in the same change set.
