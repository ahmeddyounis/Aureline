# Architecture driver → protected journey SLO traceability matrix

This document freezes **why Aureline is built this way** by linking:

- business/product drivers (TAD §3.1),
- hard constraints (TAD §3.3),
- quality-attribute drivers (TAD §3.2), and
- protected journey SLOs and evidence lanes

into one traceable, challengeable join.

The intent is that reviewers can start from **any protected workflow** (or any claim that cites one)
and quickly find: the owning driver(s), the governing SLO row, the measurement lane, the evidence
packet(s), and the waiver/descoping consequence.

## Source of truth

Authoritative narrative sources:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — drivers, constraints, and journey SLO bars.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — protected journey UX bars and degradation posture.
- `.t2/docs/Aureline_PRD.md` — end-to-end budgets and critical-path rules as cited by perf ledgers.

Authoritative machine sources this matrix composes with:

- `artifacts/perf/protected_path_ledger.yaml` — stable protected-path ids + ownership + boundaries.
- `artifacts/perf/latency_budget_ledger.yaml` — per-path SLI targets + fail-soft posture + waivers.
- `artifacts/perf/evidence_linkage_seed.yaml` — joins from a protected path to traces, corpora, packets.
- `artifacts/bench/fitness_function_catalog.yaml` — protected fitness rows + driver/principle/journey tags.
- `artifacts/qe/quality_scenario_hooks.yaml` — QE scenario ids (rows this matrix must cite verbatim).

New control artifacts introduced by this change:

- `artifacts/architecture/quality_attribute_slos.yaml` — machine SLO row register (derived from TAD/UX).
- `artifacts/architecture/protected_journey_map.yaml` — per-journey join rows (drivers ↔ SLO ↔ evidence).
- `artifacts/architecture/driver_to_rejected_pattern_refs.yaml` — rejected patterns / prohibited shortcuts.

## How to use this matrix

### Start from a protected workflow (recommended)

1. Identify the `path_id` in `artifacts/perf/protected_path_ledger.yaml`.
2. Look up the corresponding row in `artifacts/architecture/protected_journey_map.yaml`.
3. From that row:
   - follow `slo_row_refs[]` into `artifacts/architecture/quality_attribute_slos.yaml`,
   - follow `evidence_row_ref` into `artifacts/perf/evidence_linkage_seed.yaml`,
   - follow `fitness_function_refs[]` into `artifacts/bench/fitness_function_catalog.yaml`,
   - review `rejected_pattern_refs[]` in `artifacts/architecture/driver_to_rejected_pattern_refs.yaml`.

### Start from a claim

Claims are expected to cite either protected path ids or fitness-function ids (directly or via evidence packets).

1. If the claim cites a `path_id`, follow the workflow above.
2. If the claim cites an `ff.*` id, resolve it in `artifacts/bench/fitness_function_catalog.yaml` and use its
   `protected_journeys[]` and `protected_slo_family` to find the matching join rows in
   `artifacts/architecture/protected_journey_map.yaml`.

## Driver → journey join (summary)

This page is intentionally a **summary**; the full join (including evidence lanes, packet families, and
waiver/descoping posture) is in `artifacts/architecture/protected_journey_map.yaml`.

### Business/product drivers (TAD §3.1)

| Driver | Primary protected journeys it shapes | Primary evidence lanes |
|---|---|---|
| Beat incumbent trade-offs | `path.shell.launch`, `path.shell.first_useful_chrome`, `path.editor.first_useful_edit` | `lane:benchmark_lab` |
| Win switching moments | `path.command_palette.open`, `path.editor.save`, `path.workspace.restore` | `lane:benchmark_lab`, QE scenario hooks |
| Remain open and portable | `path.editor.save`, `path.workspace.restore` | `lane:benchmark_lab`, release evidence packets |
| Scale to enterprise deployment | `journey.remote_reconnect_to_usable_session`, `journey.rerun_last_task_or_test_dispatch` | `lane:protected_sequence_packet_validation`, `lane:chaos_fault_injection` |

### Hard constraints (TAD §3.3)

| Constraint | Enforced / evidenced by | Protected journeys most impacted |
|---|---|---|
| UI hot path: no blocking FS/network/process I/O | `artifacts/architecture/protected_path_dependency_rules.yaml`, `tools/check_protected_dependencies.py` | startup + input + command entry rows |
| Core workflows without mandatory hosted services | `artifacts/perf/latency_budget_ledger.yaml` fail-soft posture + sequence packs | startup, open/edit/save, basic navigation |
| Unified command + execution contracts | `docs/commands/command_descriptor_contract.md`, `schemas/runtime/execution_context.schema.json` | command entry, rerun task/test, AI actions |
| Multi deployment modes (local/remote/managed/self-host/air-gapped) | deployment/profile truth artifacts + remote attach sequences | remote reconnect, portability/export |
| Signed/mirrorable/revocable artifacts | release build identity + update/security contracts | update/restore/support surfaces |

### Quality-attribute drivers (TAD §3.2)

| Quality attribute | Governing SLO families | Representative protected journeys |
|---|---|---|
| Latency | `startup_and_first_useful_work`, `input_response`, `quick_open_and_command_dispatch` | startup, command entry, edit latency |
| Correctness | `save_and_filesystem_correctness`, `recoverability_and_restore` | save, restore |
| Recoverability | `recoverability_and_restore` | restore + journal replay |
| Security | (constraint-gated; see rejected patterns) | remote attach, AI tool use, export |

## Rejected patterns (examples)

The driver/journey traceability mapping lives in `artifacts/architecture/driver_to_rejected_pattern_refs.yaml`.

The reviewer-facing rejected-pattern ledger (rejection rationale + reopen evidence requirements) lives in:

- `artifacts/architecture/rejected_pattern_rows.yaml`
- `artifacts/architecture/revisit_trigger_matrix.yaml`

Typical examples:

- Blocking I/O on protected input/render paths.
- Optional service calls on the core local-edit critical path.
- Unreviewable multi-file mutation (AI, refactor, migration) without preview/checkpoint.
- Public or stable claims made from seed-only or stale evidence.
