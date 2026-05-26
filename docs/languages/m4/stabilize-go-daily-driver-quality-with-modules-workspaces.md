# Stabilize Go daily-driver quality with modules/workspaces, test/debug, and symbol/refactor parity — stable contract

Status: Stable lane proof for Go replacement-grade daily-driver
quality.

This document is the reviewer-facing contract for the stable Go
daily-driver quality truth packet. The packet is the single source
of truth that the editor language pack, framework pack panel,
language settings/help, CLI/headless inspector, support export,
release proof index, Help/About proof card, and the conformance
dashboard all read; surfaces MUST NOT mint local copies or
paraphrase daily-driver posture.

The packet pins Go daily-driver quality across three intertwined
truths beyond the bare daily loop:

1. The **Go modules and workspaces truth** — every row that crosses
   the Go module / workspace boundary (single-module `go.mod` /
   `go.sum`, multi-module `go.work` / `go.work.sum`, `GO111MODULE`
   module-mode vs `GOPATH` legacy mode, `GOMODCACHE` / `GOPROXY` /
   `GOPRIVATE` resolution, and `replace` / `exclude` / `retract`
   directives) binds a dedicated `module_workspace_row` and a
   disclosure ref so the daily-driver row never confuses one module
   layout for another.
2. The **Go test runner and debugger parity** — every row that
   certifies the run/test/debug step on Go archetype repos binds a
   dedicated `test_runner_row` (`go test`, `go test -race`,
   `go test -cover`, `gotestsum`, `testify` table tests) and a
   `debugger_row` (Delve `dlv debug` / `dlv attach` / `dlv test`,
   DAP integration, and headless `dlv --headless` recovery) so the
   run/test/debug row cannot inherit an adjacent test-runner or
   debugger row's evidence.
3. The **Go symbol and refactor parity** — every row that certifies
   symbol navigation or refactor on Go archetype repos binds a
   dedicated `symbol_refactor_row` (`gopls` LSP symbol / rename /
   extract, `gorename`, `gofmt`, `goimports`, and `golangci-lint`
   review surface) so a beta-grade capability sample cannot
   masquerade as a replacement-grade Go daily driver.

## What the packet asserts

For each governed *language lane × daily-driver row* the packet
asserts:

1. The **language lane class** — currently `go_daily_driver_lane`.
   Every certified packet MUST carry at least one row for each
   required lane.
2. The **daily-driver row class** — one of `daily_driver_quality`,
   `daily_loop_step`, `framework_pack`, `migration_evidence`,
   `archetype_repo_evidence`, `module_workspace_row`,
   `test_runner_row`, `debugger_row`, `symbol_refactor_row`,
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
   `benchmark_evidence`, `module_workspace_evidence`,
   `test_runner_evidence`, `debugger_evidence`,
   `symbol_refactor_evidence`, `docs_disclosure_evidence`, or
   `evidence_unbound`. A row whose evidence class is
   `evidence_unbound` is refused.
6. The **known-limit class** — one of `none_declared`,
   `framework_subset_only`, `language_subset_only`,
   `archetype_subset_only`, `migration_subset_only`,
   `module_workspace_subset_only`, `test_runner_subset_only`,
   `debugger_subset_only`, `symbol_refactor_subset_only`,
   `unsupported_runtime_target`, `beta_capability_sample_only`, or
   `limit_unbound`. A row whose known limit is `limit_unbound` is
   refused.
7. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_missing_fixture`,
   `auto_narrow_on_missing_archetype`,
   `auto_narrow_on_failed_migration`,
   `auto_narrow_on_framework_gap`,
   `auto_narrow_on_unproven_module_workspace`,
   `auto_narrow_on_unproven_test_runner`,
   `auto_narrow_on_unproven_debugger`,
   `auto_narrow_on_unproven_symbol_refactor`,
   `auto_demote_on_low_confidence`,
   `auto_block_on_missing_evidence`,
   `manual_only_pending_review`, or `automation_unbound`. A row
   whose automation is `automation_unbound` is refused.
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
of those booleans to false. The packet never admits raw Go package
source bodies, `go.sum` secret values, `.env` secrets, ambient
`GOPROXY` / `GOPRIVATE` credentials, or provider payloads.

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
`GoDailyDriverQualityTruthPacket::materialize` and then read the
projection that matches their surface. The packet is metadata-only
and suitable for inclusion in any support export or release proof
bundle.

## Where the packet lives

- Schema: `schemas/language/go_daily_driver_quality_truth.schema.json`
- Reviewer artifact: `artifacts/language/m4/stabilize-go-daily-driver-quality-with-modules-workspaces.md`
- Checked-in packet: `artifacts/language/m4/go_daily_driver_quality_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/go_daily_driver_quality_truth_packet/`
- Rust module: `crates/aureline-language/src/go_daily_driver_quality_truth_packet/mod.rs`
- Replay tests: `crates/aureline-language/tests/go_daily_driver_quality_truth_packet.rs`
