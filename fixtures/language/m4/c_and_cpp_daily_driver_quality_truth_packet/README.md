# c_and_cpp_daily_driver_quality_truth_packet fixture corpus

Fixture corpus for the M4 stable C and C++ daily-driver quality
truth packet
(`schemas/language/c_and_cpp_daily_driver_quality_truth.schema.json`).

Each fixture is a `CAndCppDailyDriverQualityTruthPacketInput`
with an `expect` block that pins the materialized packet's promotion
state, finding count, lane and row-class token sets, support-class,
daily-loop step, known-limit, downgrade-automation, and
evidence-class tokens, and the support-export safety verdict. Tests
in
`crates/aureline-language/tests/c_and_cpp_daily_driver_quality_truth_packet.rs`
load each case and assert that
`CAndCppDailyDriverQualityTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — The C and C++ daily-driver lane
  carries a `daily_driver_quality` row at `replacement_grade` plus
  every certified `daily_loop_step` row (open_or_import, navigate,
  edit, complete, refactor, run_test_debug, review, migrate,
  recover), framework-pack, migration-evidence, and archetype-repo
  rows, plus build-workspace (CMake `CMakeLists.txt` top-level and
  subdirectory targets, `CMakePresets.json` /
  `CMakeUserPresets.json` presets, `CMakeCache.txt`, `cmake/`
  modules, Ninja `build.ninja` and Ninja Multi-Config,
  `CMAKE_BUILD_TYPE`, `CMAKE_TOOLCHAIN_FILE`, vcpkg `vcpkg.json` /
  `vcpkg-configuration.json`, Conan `conanfile.txt` /
  `conanfile.py`, `compile_commands.json` export), compile/run/debug
  (`cmake --build` driving Ninja or Make targets, `ctest` /
  `ctest --output-on-failure` test invocation, executable launch via
  `cmake --build --target <target>`, clang / gcc / MSVC column- and
  range-accurate compile diagnostics, SARIF import from `clang-tidy`
  and `cppcheck`, LLDB / GDB native debuggers, `lldb-dap` and
  CodeLLDB DAP bridges, launch and attach modes, `gdbserver` /
  `lldb-server` remote-debug, core-dump open), and clangd-navigation
  (clangd LSP `textDocument/rename`, `textDocument/definition`,
  `textDocument/references`, `textDocument/typeDefinition`,
  extract / inline / code-action, type-hierarchy, call-hierarchy,
  header / source pairing (`.h` / `.c`, `.hpp` / `.cc` / `.cpp` /
  `.cxx`), include resolution from `compile_commands.json` `-I`
  paths, cross-translation-unit background index, `clang-format`,
  `clang-tidy`) rows. Every row binds support, known limit,
  downgrade automation, and evidence classes; narrowed rows carry
  their disclosure refs, and all eight required consumer projections
  preserve the packet verbatim.
- `replacement_grade_with_unbound_evidence_blocks_stable.json` — The
  C and C++ daily-driver quality row claims `replacement_grade`
  while its evidence class is `evidence_unbound`; the packet blocks
  the stable claim because no archetype, fixture-repo, migration,
  build-workspace, compile/run/debug, clangd-navigation, or
  design-partner evidence backs the row.
- `missing_daily_loop_step_for_replacement_grade_blocks_stable.json`
  — The lane claims `replacement_grade` but the `recover` daily-loop
  step is missing; the packet blocks the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A
  known-limit row narrows below replacement grade with
  `build_workspace_subset_only` but drops its disclosure ref; the
  packet blocks the stable claim.
- `projection_collapses_evidence_class_vocabulary_blocks_stable.json`
  — The `help_about` consumer projection drops the evidence-class
  vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — A daily-driver row
  admits raw source bodies past the boundary; the packet blocks the
  stable claim because raw C or C++ source bodies,
  `compile_commands.json` argument streams, vcpkg or Conan registry
  credentials, `.env` secrets, and ambient `CMAKE_TOOLCHAIN_FILE` /
  `VCPKG_ROOT` / `CONAN_USER_HOME` / `CC` / `CXX` /
  `LD_LIBRARY_PATH` environment values must never leak through the
  daily-driver boundary.
