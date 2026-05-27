# Stabilize C and C++ daily-driver quality with CMake/Ninja import, compile/run/debug fidelity, and clangd rename/navigation truth — stable contract

Status: Stable lane proof for C and C++ replacement-grade
daily-driver quality.

This document is the reviewer-facing contract for the stable C and
C++ daily-driver quality truth packet. The packet is the single
source of truth that the editor language pack, framework pack panel,
language settings/help, CLI/headless inspector, support export,
release proof index, Help/About proof card, and the conformance
dashboard all read; surfaces MUST NOT mint local copies or paraphrase
daily-driver posture.

The packet pins C and C++ daily-driver quality across three
intertwined truths beyond the bare daily loop:

1. The **CMake / Ninja build workspace truth** — every row that
   crosses the C / C++ build boundary (`CMakeLists.txt` top-level
   and subdirectory targets, `CMakePresets.json` /
   `CMakeUserPresets.json` configure / build / test / package
   presets, `CMakeCache.txt`, `cmake/` modules and toolchains,
   Ninja `build.ninja` and Ninja Multi-Config generators,
   `CMAKE_BUILD_TYPE` (Debug / Release / RelWithDebInfo /
   MinSizeRel), `CMAKE_TOOLCHAIN_FILE`, vcpkg `vcpkg.json` /
   `vcpkg-configuration.json` and Conan `conanfile.txt` /
   `conanfile.py` package-manager integration, and
   `compile_commands.json` export
   (`CMAKE_EXPORT_COMPILE_COMMANDS=ON`)) binds a dedicated
   `build_workspace_row` and a disclosure ref so the daily-driver
   row never confuses one workspace layout for another.
2. The **compile / run / debug fidelity parity** — every row that
   certifies the run/test/debug step on C or C++ archetype repos
   binds a dedicated `compile_run_debug_row` (`cmake --build`
   driving Ninja or Make targets, `ctest` /
   `ctest --output-on-failure` test invocation, executable launch
   via `cmake --build --target <target>` or `ninja <target>`;
   clang / gcc / MSVC column- and range-accurate compile
   diagnostics; SARIF import from `clang-tidy` and `cppcheck`;
   LLDB and GDB native debuggers; `lldb-dap` and CodeLLDB DAP
   bridges; launch and attach modes; `gdbserver` / `lldb-server`
   remote-debug transports; core-dump open) so a beta-grade
   capability sample cannot masquerade as a replacement-grade C /
   C++ run-debug daily-driver surface.
3. The **clangd rename / navigation parity** — every row that
   certifies symbol navigation, completion, or refactor on C or
   C++ archetype repos binds a dedicated `clangd_navigation_row`
   (clangd LSP `textDocument/rename` / `textDocument/definition` /
   `textDocument/references` / `textDocument/typeDefinition` /
   extract / inline / code-action; type-hierarchy and
   call-hierarchy; header / source pairing (`.h` / `.c`,
   `.hpp` / `.cc` / `.cpp` / `.cxx`); include resolution from
   `compile_commands.json` `-I` paths; cross-translation-unit
   background index; `clang-format` / `.clang-format` formatting
   and `clang-tidy` / `.clang-tidy` quick-fixes) so
   large-workspace rename and navigation posture is never inferred
   from a tiny single-translation-unit sample.

## What the packet asserts

For each governed *language lane × daily-driver row* the packet
asserts:

1. The **language lane class** — currently
   `c_and_cpp_daily_driver_lane`. Every certified packet MUST
   carry at least one row for each required lane.
2. The **daily-driver row class** — one of `daily_driver_quality`,
   `daily_loop_step`, `framework_pack`, `migration_evidence`,
   `archetype_repo_evidence`, `build_workspace_row`,
   `compile_run_debug_row`, `clangd_navigation_row`,
   `unsupported_gap`, `known_limit`, or `downgrade_automation`. A
   `daily_loop_step` row MUST bind a real daily-loop step; no other
   row class is permitted to bind one.
3. The **support class** — one of `replacement_grade`,
   `daily_driver_below_replacement`, `beta_grade_only`,
   `preview_only`, `unsupported`, or `support_unbound`. The validator
   refuses to certify a row that claims `replacement_grade` while any
   binding is unbound (support, known limit, downgrade automation, or
   evidence).
4. The **daily-loop step class** — one of `open_or_import`,
   `navigate`, `edit`, `complete`, `refactor`, `run_test_debug`,
   `review`, `migrate`, `recover`, or `not_applicable`. A lane that
   claims `replacement_grade` daily-driver support MUST cover every
   certified daily-loop step.
5. The **evidence class** — one of `archetype_repo_evidence`,
   `framework_migration_evidence`, `design_partner_evidence`,
   `fixture_repo_evidence`, `conformance_suite_evidence`,
   `benchmark_evidence`, `build_workspace_evidence`,
   `compile_run_debug_evidence`, `clangd_navigation_evidence`,
   `docs_disclosure_evidence`, or `evidence_unbound`. A row whose
   evidence class is `evidence_unbound` is refused.
6. The **known-limit class** — one of `none_declared`,
   `framework_subset_only`, `language_subset_only`,
   `archetype_subset_only`, `migration_subset_only`,
   `build_workspace_subset_only`, `compile_run_debug_subset_only`,
   `clangd_navigation_subset_only`, `unsupported_runtime_target`,
   `beta_capability_sample_only`, or `limit_unbound`. A row whose
   known limit is `limit_unbound` is refused.
7. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_missing_fixture`,
   `auto_narrow_on_missing_archetype`,
   `auto_narrow_on_failed_migration`,
   `auto_narrow_on_framework_gap`,
   `auto_narrow_on_unproven_build_workspace`,
   `auto_narrow_on_unproven_compile_run_debug`,
   `auto_narrow_on_unproven_clangd_navigation`,
   `auto_demote_on_low_confidence`,
   `auto_block_on_missing_evidence`, `manual_only_pending_review`,
   or `automation_unbound`. A row whose automation is
   `automation_unbound` is refused.
8. The **daily-driver confidence class** — `high_confidence`,
   `medium_confidence`, or `low_confidence`. A row that claims
   `replacement_grade` at `low_confidence` is narrowed below stable
   until evidence grows.
9. The **evidence refs** — every row preserves at least one
   repo-relative evidence ref proving the daily-driver claim.
10. The **disclosure ref** — every row that is not
    `replacement_grade`, that declares a non-`none_declared` known
    limit, or that binds a non-`none` downgrade automation MUST carry
    a repo-relative disclosure ref shown to the user.

## Boundary safety

Every row carries `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded`. The validator emits
`raw_source_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one
of those booleans to false. The packet never admits raw C or C++
source bodies, `compile_commands.json` argument streams, vcpkg or
Conan registry credentials, `.env` secrets, ambient
`CMAKE_TOOLCHAIN_FILE` / `VCPKG_ROOT` / `CONAN_USER_HOME` /
`CC` / `CXX` / `LD_LIBRARY_PATH` environment values, or provider
payloads.

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
  known limit / non-`none` downgrade automation drops its disclosure
  ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies,
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`CAndCppDailyDriverQualityTruthPacket::materialize` and then read
the projection that matches their surface. The packet is
metadata-only and suitable for inclusion in any support export or
release proof bundle.

## Where the packet lives

- Schema: `schemas/language/c_and_cpp_daily_driver_quality_truth.schema.json`
- Reviewer artifact: `artifacts/language/m4/stabilize-c-and-cpp-daily-driver-quality-with.md`
- Checked-in packet: `artifacts/language/m4/c_and_cpp_daily_driver_quality_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/c_and_cpp_daily_driver_quality_truth_packet/`
- Rust module: `crates/aureline-language/src/c_and_cpp_daily_driver_quality_truth_packet/mod.rs`
- Replay tests: `crates/aureline-language/tests/c_and_cpp_daily_driver_quality_truth_packet.rs`
