# Stabilize Python daily-driver quality with interpreter, venv/uv/Poetry, pytest, and debugger truth — stable contract

Status: Stable lane proof for Python replacement-grade daily-driver
quality.

This document is the reviewer-facing contract for the stable Python
daily-driver quality truth packet. The packet is the single source of
truth that the editor language pack, framework pack panel, language
settings/help, CLI/headless inspector, support export, release proof
index, Help/About proof card, and the conformance dashboard all read;
surfaces MUST NOT mint local copies or paraphrase daily-driver
posture.

The packet pins Python daily-driver quality across three intertwined
truths beyond the bare daily loop:

1. The **Python interpreter selection truth** — every row that
   crosses the Python interpreter boundary (system Python vs
   `pyenv`, `asdf`, `conda`, or workspace-pinned `python_requires`
   interpreter; CPython vs PyPy runtime; `sys.executable` and the
   active `python_path`) binds a dedicated
   `interpreter_selection_row` and a disclosure ref so the
   daily-driver row never confuses one interpreter for another.
2. The **environment-manager truth** — every row that crosses the
   Python environment-manager boundary (`venv` / `virtualenv`, `uv`,
   Poetry, `pip-tools`, `pipenv`, or conda environments;
   `pyproject.toml` / `requirements*.txt` / `poetry.lock` /
   `uv.lock` dependency closure) binds a dedicated
   `environment_manager_row` and a disclosure ref so a beta-grade
   capability sample cannot masquerade as a replacement-grade
   Python daily driver.
3. The **test runner and debugger parity** — every row that
   certifies the run/test/debug step on Python archetype repos
   binds a dedicated `test_runner_row` (pytest, unittest, nose2,
   pytest-xdist, pytest-asyncio, tox) and a `debugger_row`
   (debugpy / DAP, pdb / `breakpoint()` launch profile, and
   `python -m pdb` / `pytest --pdb` recovery) so the run/test/debug
   row cannot inherit an adjacent test-runner or debugger row's
   evidence.

## What the packet asserts

For each governed *language lane × daily-driver row* the packet
asserts:

1. The **language lane class** — currently
   `python_daily_driver_lane`. Every certified packet MUST carry at
   least one row for each required lane.
2. The **daily-driver row class** — one of `daily_driver_quality`,
   `daily_loop_step`, `framework_pack`, `migration_evidence`,
   `archetype_repo_evidence`, `interpreter_selection_row`,
   `environment_manager_row`, `test_runner_row`, `debugger_row`,
   `unsupported_gap`, `known_limit`, or `downgrade_automation`. A
   `daily_loop_step` row MUST bind a real daily-loop step; no other
   row class is permitted to bind one.
3. The **support class** — one of `replacement_grade`,
   `daily_driver_below_replacement`, `beta_grade_only`,
   `preview_only`, `unsupported`, or `support_unbound`. The
   validator refuses to certify a row that claims
   `replacement_grade` while any binding is unbound (support, known
   limit, downgrade automation, or evidence).
4. The **daily-loop step class** — one of `open_or_import`,
   `navigate`, `edit`, `complete`, `refactor`, `run_test_debug`,
   `review`, `migrate`, `recover`, or `not_applicable`. A lane that
   claims `replacement_grade` daily-driver support MUST cover every
   certified daily-loop step.
5. The **evidence class** — one of `archetype_repo_evidence`,
   `framework_migration_evidence`, `design_partner_evidence`,
   `fixture_repo_evidence`, `conformance_suite_evidence`,
   `benchmark_evidence`, `interpreter_selection_evidence`,
   `environment_manager_evidence`, `test_runner_evidence`,
   `debugger_evidence`, `docs_disclosure_evidence`, or
   `evidence_unbound`. A row whose evidence class is
   `evidence_unbound` is refused.
6. The **known-limit class** — one of `none_declared`,
   `framework_subset_only`, `language_subset_only`,
   `archetype_subset_only`, `migration_subset_only`,
   `interpreter_subset_only`, `environment_manager_subset_only`,
   `test_runner_subset_only`, `debugger_subset_only`,
   `unsupported_runtime_target`, `beta_capability_sample_only`, or
   `limit_unbound`. A row whose known limit is `limit_unbound` is
   refused.
7. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_missing_fixture`,
   `auto_narrow_on_missing_archetype`,
   `auto_narrow_on_failed_migration`,
   `auto_narrow_on_framework_gap`,
   `auto_narrow_on_unproven_interpreter`,
   `auto_narrow_on_unproven_environment_manager`,
   `auto_narrow_on_unproven_test_runner`,
   `auto_narrow_on_unproven_debugger`,
   `auto_demote_on_low_confidence`,
   `auto_block_on_missing_evidence`,
   `manual_only_pending_review`, or `automation_unbound`. A row whose
   automation is `automation_unbound` is refused.
8. The **daily-driver confidence class** — `high_confidence`,
   `medium_confidence`, or `low_confidence`. A row that claims
   `replacement_grade` at `low_confidence` is narrowed below stable
   until evidence grows.
9. The **evidence refs** — every row preserves at least one
   repo-relative evidence ref proving the daily-driver claim.
10. The **disclosure ref** — every row that is not
    `replacement_grade`, that declares a non-`none_declared` known
    limit, or that binds a non-`none` downgrade automation MUST
    carry a repo-relative disclosure ref shown to the user.

## Boundary safety

Every row carries `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded`. The validator emits
`raw_source_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one
of those booleans to false. The packet never admits raw Python source
bodies, `pyproject.toml` secret values, `.env` secrets, ambient PyPI
credentials, or provider payloads.

## Required consumer projections

The packet is preserved verbatim across eight consumer projections:

| Projection              | Surface                              |
| ----------------------- | ------------------------------------ |
| `editor_language_pack`  | Editor language pack badge / hover   |
| `framework_pack_panel`  | Framework pack panel                 |
| `language_settings`     | Language settings / help surface     |
| `cli_headless`          | CLI/headless inspector               |
| `support_export`        | Support export bundle                |
| `release_proof_index`   | Release proof index entry            |
| `help_about`            | Help/About proof card                |
| `conformance_dashboard` | Conformance dashboard row            |

A projection that collapses any closed vocabulary, drops the packet
id, drops the lane class, row class, support class, daily-loop step,
known-limit, downgrade-automation, or evidence-class vocabulary, or
leaks raw private material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `replacement_grade` while its support, known-limit,
  downgrade-automation, or evidence class is unbound,
- a lane that claims `replacement_grade` daily-driver support is
  missing a certified `daily_loop_step` row for any of the nine
  required steps (open/import, navigate, edit, complete, refactor,
  run/test/debug, review, migrate, recover),
- a `daily_loop_step` row drops its daily-loop step binding,
- a non-`daily_loop_step` row binds a daily-loop step it cannot
  certify,
- a row narrowed below `replacement_grade` or with a non-default
  known limit / non-`none` downgrade automation drops its
  disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies,
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`PythonDailyDriverQualityTruthPacket::materialize` and then read the
projection that matches their surface. The packet is metadata-only
and suitable for inclusion in any support export or release proof
bundle.

## Where the packet lives

- Schema: `schemas/language/python_daily_driver_quality_truth.schema.json`
- Reviewer artifact: `artifacts/language/m4/stabilize-python-daily-driver-quality-with-interpreter-venv.md`
- Checked-in packet: `artifacts/language/m4/python_daily_driver_quality_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/python_daily_driver_quality_truth_packet/`
- Rust module: `crates/aureline-language/src/python_daily_driver_quality_truth_packet/mod.rs`
- Replay tests: `crates/aureline-language/tests/python_daily_driver_quality_truth_packet.rs`
