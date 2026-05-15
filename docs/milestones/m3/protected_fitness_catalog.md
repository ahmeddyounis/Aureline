# M3 protected fitness-function catalog

This page is the reviewer-facing entrypoint for the M3 protected
fitness-function dashboard. The canonical truth lives in the YAML/JSON
artifacts named below; release packets, dashboards, support exports,
and the governance review consume the generated dashboard snapshot
rather than restating tile prose.

The catalog freezes the M3 beta train's protected fitness-function
lanes. It projects rows from the canonical catalog
(`artifacts/bench/fitness_function_catalog.yaml`) into the M3 beta
scope, attaches threshold owners and waiver authorities, and emits a
machine-derived dashboard snapshot so promotion decisions can be made
from the catalog without consulting side spreadsheets.

## Canonical artifacts

- Catalog matrix: `artifacts/benchmarks/m3/protected_fitness_catalog.yaml`
- Waiver register: `artifacts/milestones/m3/waiver_register.yaml`
- Generated dashboard snapshot: `artifacts/benchmarks/m3/dashboard_snapshot.json`
- Generator and validator: `ci/check_m3_protected_fitness_catalog.py`
- Latest validation capture:
  `artifacts/benchmarks/m3/captures/protected_fitness_catalog_validation_capture.json`

## Upstream contracts inherited verbatim

- Canonical fitness-function catalog: `artifacts/bench/fitness_function_catalog.yaml`
- Fitness-state vocabulary register: `artifacts/governance/fitness_state_rows.yaml`
- Fitness-dashboard tile contract: `docs/governance/fitness_dashboard_contract.md`
- Waiver register contract: `docs/governance/waiver_register_contract.md`
- Claimed-surface register: `artifacts/milestones/m3/claimed_surface_register.json`
- Cohort guardrails: `artifacts/milestones/m3/cohort_guardrails.yaml`

## Protected lanes published at beta

| Lane | Row status | Canonical anchor | Beta surfaces gated | Waiver authority |
|---|---|---|---|---|
| Startup | seeded | `ff.warm_start_to_first_paint` | `beta_surface:packaging_update_rollback`, `beta_surface:support_export_diagnostics` | `performance_council` |
| Typing | seeded | `ff.input_to_paint` | `beta_surface:debug_test_task_model` | `performance_council` |
| Search | provisional | `ff.command_parity` | `beta_surface:debug_test_task_model`, `beta_surface:support_export_diagnostics` | `performance_council` |
| Rollback | seeded | `ff.vfs_save_conflict_handling` | `beta_surface:packaging_update_rollback`, `beta_surface:support_export_diagnostics`, `beta_surface:importer_and_migration` | `performance_council` |
| Supportability | seeded | `ff.benchmark_lab_health` | `beta_surface:support_export_diagnostics`, `beta_surface:debug_test_task_model` | `performance_council` |
| Extension isolation | provisional | _(canonical row pending)_ | `beta_surface:extension_runtime` | `architecture_council` |
| Policy / trust | provisional | _(canonical row pending)_ | `beta_surface:policy_proxy_transport`, `beta_surface:extension_runtime` | `release_council` |

Provisional lanes name a `completion_owner` and a `completion_target`
in the catalog matrix; the dashboard tile renders the provisional
placeholder verbatim under the
`provisional_no_action_until_seeded` mitigation class so the lane
cannot be cited as a fresh pass by mistake.

## Definition of green

The catalog is green when:

- every required protected lane (startup, typing, search, rollback,
  supportability, extension isolation, policy/trust) has a row in the
  M3 catalog with a named threshold owner, owning lane, and waiver
  authority;
- every seeded row binds to at least one canonical
  `fitness_function_catalog.yaml#rows` entry;
- every provisional row names a `completion_owner` and a
  `completion_target` so the canonical-row gap is inspectable;
- every row's `dashboard_tile.tile_state`,
  `evidence_freshness_class`, `mitigation_note_class`,
  `partial_profile_result_class`, `corpus_profile_identity_class`,
  and waiver authority resolve into
  `artifacts/governance/fitness_state_rows.yaml`;
- every row's `beta_surface_refs` resolve into
  `artifacts/milestones/m3/claimed_surface_register.json#claimed_surfaces`,
  and every reachable cohort resolves into
  `artifacts/milestones/m3/cohort_guardrails.yaml#cohorts`;
- the waiver register has exactly one row per catalog row, with the
  typed `register_status_class`, `expiry_proximity_class`,
  `current_evidence_gap_class`, `renewal_requested_state_class`,
  `repeated_path_grouping_class`, and `claim_scope_narrowing_class`
  values mirrored from
  `docs/governance/waiver_register_contract.md`;
- the generated `dashboard_snapshot.json` reproduces verbatim against
  the matrix and the waiver register under `--check`; and
- the validator passes.

## Dashboard tile state grammar

The dashboard snapshot renders one tile per row. Each tile reuses the
closed six-class state grammar frozen in
`artifacts/governance/fitness_state_rows.yaml#tile_states`:

- `passing` — threshold met on fresh evidence with no live waiver;
- `warning` — early-signal drift on fresh evidence;
- `blocked` — threshold breached on fresh evidence;
- `waived` — failure held open by an active waiver;
- `waiver_expired` — waiver expired without renewal;
- `evidence_stale` — captured evidence has aged out by time, by named
  trigger, or is missing.

The snapshot does not invent a parallel chip vocabulary; consuming
surfaces render the same state token, threshold label, measured label,
freshness class, owner, corpus profile identity, mitigation class, and
waiver projection per row.

## Threshold ownership

Each row names a primary DRI, owning lane, optional co-owning lane,
backup owner posture, threshold mode, and decision-forum ref so
promotion decisions cite the catalog rather than side spreadsheets.
Threshold numerics that remain `to_be_set_by_benchmark_council` on the
canonical row stay that way here; the dashboard tile renders the
placeholder verbatim under the `provisional_no_action_until_seeded`
mitigation class.

## Waiver register projection

The waiver register at
`artifacts/milestones/m3/waiver_register.yaml` projects one row per
protected lane. Each row carries the typed `waiver_kind_class`,
`waiver_authority_class`, `register_status_class`,
`expiry_proximity_class`, `current_evidence_gap_class`,
`renewal_requested_state_class`, `repeated_path_grouping_class`,
`claim_scope_narrowing_class`, and the typed mitigation ledger from
`docs/governance/waiver_register_contract.md`. The dashboard snapshot
quotes the waiver register row verbatim alongside the tile state so a
waived failure cannot collapse into a clean pass on any consuming
surface.

The full canonical waiver_register_entry_record fixture corpus lives at
`fixtures/governance/waiver_cases/`; the M3 register is a milestone-
scoped projection that downstream M3 lanes cite.

## How to refresh

Run the generator to re-derive the dashboard snapshot and refresh the
validation capture in the same change set:

```
python3 ci/check_m3_protected_fitness_catalog.py --repo-root .
```

Use `--check` in CI to fail when the on-disk snapshot or capture would
drift from the catalog or upstream seeds.

## How to validate

```
python3 ci/check_m3_protected_fitness_catalog.py --repo-root . --check
```

The validator fails closed when:

- the catalog is missing a required protected lane;
- a seeded row has no `primary_canonical_row_ref`, or a provisional row
  has no `completion_owner`;
- a row cites an unknown canonical fitness function, beta surface, or
  cohort;
- the waiver register row vocabulary, the dashboard tile vocabulary,
  or the waiver authority does not resolve into
  `artifacts/governance/fitness_state_rows.yaml` or the waiver register
  contract;
- the waiver register has an orphan row (no matching catalog row);
- a row renders as `passing` with stale evidence, a `waived` row has no
  `active_waiver_ref`, or a `waiver_expired` row has no
  `previous_waiver_ref`; or
- the checked-in `dashboard_snapshot.json` drifts from regeneration.
