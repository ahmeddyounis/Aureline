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

## Beta compatibility report

The beta compatibility report is generated from the checked-in
compatibility matrices (qualification matrix, skew-window declarations,
version-skew register, and the cohort/archetype scorecard register) so
release evidence, partner packets, docs, and Help/About surfaces all
read one machine-derived truth instead of restating compatibility prose.

- M3 skew-window matrix: `artifacts/compat/m3/skew_window_matrix.yaml`
- Generated report (Markdown): `artifacts/compat/m3/compatibility_report.md`
- Generated report (JSON): `artifacts/compat/m3/compatibility_report.json`
- Report schema: `schemas/governance/compatibility_report.schema.json`
- Generator and validator: `ci/check_m3_compatibility_report.py`
- Latest validation capture:
  `artifacts/compat/m3/captures/compatibility_report_validation_capture.json`

Each generated row names support class (declared and effective), skew
window (id, class, summary), declared downgrade behavior, and client
scope (deployment profiles, primary cohorts, claimed beta surfaces).
Spec-mandated row scopes (desktop, helper, extension, schema, provider,
deployment_profile) are all required to be present, and the validator
fails closed when the on-disk report drifts from the matrices.

## Cohort and archetype scorecards

Each named cohort and certified-archetype row owns a scorecard with owner,
evidence date, open waivers, downgrade policy, and owner handoff path. The
validator parses every scorecard, applies the downgrade automation rules
from the index files, and writes a derived register. Docs, release, support,
and migration packets consume the derived register rather than restating
freshness or downgrade rules.

- Cohort scorecard index: `artifacts/milestones/m3/cohorts/scorecard_index.yaml`
- Cohort scorecards:
  - `artifacts/milestones/m3/cohorts/design_partner_scorecard.md`
  - `artifacts/milestones/m3/cohorts/extension_author_scorecard.md`
  - `artifacts/milestones/m3/cohorts/managed_pilot_scorecard.md`
- Archetype scorecard index: `artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml`
- Archetype scorecards: `artifacts/compat/m3/archetype_scorecards/*.md`
- Scorecard validator: `ci/check_cohort_archetype_scorecards.py`
- Derived register: `artifacts/milestones/m3/captures/cohort_archetype_scorecard_register.json`
- Validation capture: `artifacts/milestones/m3/captures/cohort_archetype_scorecard_validation_capture.json`

Downgrade automation rules (see `downgrade_automation` in the cohort
scorecard index):

- Evidence older than `review_window_days` moves the row to `retest_pending`.
- Evidence older than `review_window_days * evidence_stale_multiplier` moves
  the row to `evidence_stale`.
- An active open waiver moves the row to `limited`, or `preview` when the
  display lifecycle label is `preview`.
- A row with no triggers fired inherits its declared support class.

## Protected fitness-function dashboard

The beta train governs itself through one shared protected-fitness-function
catalog. Each protected lane (startup, typing, search, rollback,
supportability, extension isolation, policy/trust) names a threshold owner,
a waiver authority, the canonical fitness-function-catalog rows it resolves
through, and the beta surfaces it gates. The generator emits a dashboard
snapshot quoted by release packets, support exports, and the governance
review.

- Catalog matrix: `artifacts/benchmarks/m3/protected_fitness_catalog.yaml`
- Waiver register: `artifacts/milestones/m3/waiver_register.yaml`
- Generated dashboard snapshot:
  `artifacts/benchmarks/m3/dashboard_snapshot.json`
- Reviewer-facing entrypoint: `docs/milestones/m3/protected_fitness_catalog.md`
- Generator and validator: `ci/check_m3_protected_fitness_catalog.py`
- Latest validation capture:
  `artifacts/benchmarks/m3/captures/protected_fitness_catalog_validation_capture.json`

## Docs / public-truth gate

The docs / public-truth gate enforces that every claimed beta row in the
generated claim_manifest stays inspectable through fresh, vocabulary-
consistent docs, Help/About, release, and release-notes surfaces. The
gate fails closed when a claimed row has stale docs, stale examples, or
mismatched truth vocabulary across docs, Help/About, and release assets.
Release notes must link back to the current claim_manifest and the
M3 compatibility_report rows before beta publication; the gate names
the exact row, packet, and freshness delta causing each block.

- Source map: `artifacts/ci/m3_docs_truth_source_map.yaml`
- Freshness gate: `tools/ci/m3/docs_freshness_gate.py`
- Stale-example checker: `tools/ci/m3/stale_example_checker.py`
- CI gate manifest: `ci/docs/m3_docs_truth_gate.yml`
- CI shell entry: `ci/check_m3_docs_truth.sh`
- Release-notes draft (gate input):
  `artifacts/release/m3/release_notes_draft.md`
- Generated truth report:
  `artifacts/docs/m3/docs_truth_report.md`
- Latest validation captures:
  - `artifacts/docs/m3/captures/m3_docs_freshness_validation_capture.json`
  - `artifacts/docs/m3/captures/m3_stale_example_validation_capture.json`

The gate replays named failure drills under `--force-drill` so the
pipeline is reproducible without hand-curated stale fixtures:

- `m3_docs_truth.release_notes_manifest_backlink_missing` — strips
  the manifest id from the release-notes draft and asserts the
  freshness gate fails with
  `release_notes.manifest_backlink_missing`.
- `m3_docs_truth.protected_example_vocabulary_pin_drifted` — rewrites
  a payload field to a token outside the manifest vocabulary and
  asserts the stale-example checker fails with
  `stale_examples.vocabulary_pin_not_in_manifest`.

## Downstream cutline lane

The beta scope feeds the M3 exit cutline; stable planning consumes the
cutline packet rather than this page. When the beta scope widens, refresh
both lanes in the same change set.

- Cutline packet: `docs/milestones/m3/cutline_packet.md`
- Unlock map: `artifacts/milestones/m3/unlock_map.yaml`
- Stable-planning handoff checklist: `artifacts/milestones/m3/stable_planning_handoff_checklist.md`
- Cutline validator: `ci/check_m3_cutline_packet.py`
- Latest cutline capture: `artifacts/milestones/m3/captures/cutline_packet_validation_capture.json`

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

Run the beta-admission validator:

`python3 ci/check_beta_admission.py --repo-root .`

Optional machine-readable report:

`python3 ci/check_beta_admission.py --repo-root . --report artifacts/milestones/m3/captures/beta_admission_validation_capture.json`

Run the cohort and archetype scorecard validator (writes the derived
register and the validation capture by default):

`python3 ci/check_cohort_archetype_scorecards.py --repo-root .`

Run the beta compatibility-report generator (writes the report JSON,
markdown rendering, and validation capture from the checked-in
matrices):

`python3 ci/check_m3_compatibility_report.py --repo-root .`

Use `--check` in CI to fail when the on-disk report or capture would
drift from the matrices.

## Update rules

1. Update `artifacts/milestones/m3/claimed_surface_register.json` first.
2. Update `artifacts/milestones/m3/cohort_guardrails.yaml` so every claimed
   surface points to at least one cohort id with current downgrade rules.
3. Update `artifacts/milestones/m3/dependency_graph.mmd` when an upstream
   contract, fixture, consumer, or validator changes.
4. Refresh this page when the user-visible beta claim surface changes.
5. Refresh the cohort and archetype scorecards plus their indices when
   evidence dates, waivers, or downgrade triggers change.
6. Run both validators and refresh the captures in the same change set.
