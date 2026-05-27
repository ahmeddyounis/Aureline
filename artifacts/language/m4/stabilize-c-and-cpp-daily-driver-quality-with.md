# C and C++ daily-driver quality truth packet — reviewer artifact

This is the reviewer-facing artifact for the M4 stable C and C++
daily-driver quality truth packet covering the open/import, navigate,
edit, complete, refactor, run/test/debug, review, migrate, and
recover daily-loop steps with replacement-grade support,
CMake/Ninja build-workspace evidence (`CMakeLists.txt` top-level and
subdirectory targets, `CMakePresets.json` / `CMakeUserPresets.json`
configure / build / test / package presets, `CMakeCache.txt`,
`cmake/` modules and toolchains, Ninja `build.ninja` and Ninja
Multi-Config generators, `CMAKE_BUILD_TYPE` (Debug / Release /
RelWithDebInfo / MinSizeRel), `CMAKE_TOOLCHAIN_FILE`, vcpkg
`vcpkg.json` / `vcpkg-configuration.json` and Conan `conanfile.txt`
/ `conanfile.py` integration, and `compile_commands.json` export),
compile/run/debug fidelity evidence (`cmake --build` driving Ninja
or Make targets, `ctest` / `ctest --output-on-failure` test
invocation, executable launch via `cmake --build --target <target>`
or `ninja <target>`, clang / gcc / MSVC column- and range-accurate
compile diagnostics, SARIF import from `clang-tidy` and `cppcheck`,
LLDB / GDB native debuggers, `lldb-dap` and CodeLLDB DAP bridges,
launch and attach modes, `gdbserver` / `lldb-server` remote-debug
transports, core-dump open), clangd rename/navigation evidence
(clangd LSP `textDocument/rename` / `textDocument/definition` /
`textDocument/references` / `textDocument/typeDefinition` /
extract / inline / code-action, type-hierarchy, call-hierarchy,
header/source pairing (`.h` / `.c`, `.hpp` / `.cc` / `.cpp` /
`.cxx`), include resolution from `compile_commands.json` `-I`
paths, cross-translation-unit background index, `clang-format`
formatting, `clang-tidy` quick-fixes), framework-migration
evidence, known limits, downgrade automation, and evidence binding.

The contract lives at
`docs/languages/m4/stabilize-c-and-cpp-daily-driver-quality-with.md`
and is replayed by
`crates/aureline-language/tests/c_and_cpp_daily_driver_quality_truth_packet.rs`.

## Stable claim

For the governed language lane class
(`c_and_cpp_daily_driver_lane`) the packet binds:

- at least one `daily_driver_quality` row (the lane's headline C
  and C++ daily-driver qualification),
- a `daily_loop_step` row per certified step (open/import, navigate,
  edit, complete, refactor, run/test/debug, review, migrate,
  recover) when the lane claims `replacement_grade`,
- at least one `framework_pack` row certifying a C or C++ framework
  pack (e.g., a CMake/Ninja library archetype, Qt/Boost/POCO service
  archetype, or single-executable embedded archetype),
- at least one `migration_evidence` row (e.g., C++14 → C++17 → C++20
  language standard migration, Autotools/Make → CMake migration, or
  single-target → multi-target CMake migration),
- at least one `archetype_repo_evidence` row certifying the
  archetype repos backing the daily loop,
- at least one `build_workspace_row` certifying the active
  CMake/Ninja build workspace contract,
- at least one `compile_run_debug_row` certifying the compile
  diagnostics, run-target, and debugger surface,
- at least one `clangd_navigation_row` certifying the clangd LSP
  rename and navigation surface,
- a closed `support_class` (no surface pretends `replacement_grade`
  while a binding is unbound),
- a closed `daily_loop_step_class` (every replacement-grade lane
  covers the full daily loop; non-loop rows bind `not_applicable`),
- a closed `evidence_class` (archetype-repo, framework-migration,
  design-partner, fixture-repo, conformance-suite, benchmark,
  build-workspace, compile/run/debug, clangd-navigation, or
  docs-disclosure),
- a closed `known_limit_class` (framework / language / archetype /
  migration / build-workspace / compile-run-debug /
  clangd-navigation subset, unsupported runtime target, beta
  capability sample, or `none_declared`),
- a closed `downgrade_automation_class` (auto-narrow on missing
  fixture/archetype, auto-narrow on failed migration / framework
  gap / unproven build-workspace / compile-run-debug /
  clangd-navigation, auto-demote on low confidence, auto-block on
  missing evidence, manual-only, or `none`),
- a closed `daily_driver_confidence_class`, and
- at least one `evidence_refs` entry plus a `disclosure_ref` whenever
  the row is not `replacement_grade`, declares a non-`none_declared`
  known limit, or binds a non-`none` downgrade automation.

## Companion artifacts

- Schema: `schemas/language/c_and_cpp_daily_driver_quality_truth.schema.json`
- Checked-in packet:
  `artifacts/language/m4/c_and_cpp_daily_driver_quality_truth_packet.json`
- Fixture corpus:
  `fixtures/language/m4/c_and_cpp_daily_driver_quality_truth_packet/`
- Rust contract:
  `crates/aureline-language/src/c_and_cpp_daily_driver_quality_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-language/tests/c_and_cpp_daily_driver_quality_truth_packet.rs`
- Reviewer doc:
  `docs/languages/m4/stabilize-c-and-cpp-daily-driver-quality-with.md`

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

- Schema: [`schemas/language/c_and_cpp_daily_driver_quality_truth.schema.json`](../../../schemas/language/c_and_cpp_daily_driver_quality_truth.schema.json)
- Reviewer doc: [`docs/languages/m4/stabilize-c-and-cpp-daily-driver-quality-with.md`](../../../docs/languages/m4/stabilize-c-and-cpp-daily-driver-quality-with.md)
- Fixture corpus: [`fixtures/language/m4/c_and_cpp_daily_driver_quality_truth_packet/`](../../../fixtures/language/m4/c_and_cpp_daily_driver_quality_truth_packet/)
- Rust module: [`crates/aureline-language/src/c_and_cpp_daily_driver_quality_truth_packet/mod.rs`](../../../crates/aureline-language/src/c_and_cpp_daily_driver_quality_truth_packet/mod.rs)
