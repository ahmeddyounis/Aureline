# C / C++ native project

## Row binding

- Archetype row id: `archetype_row:c_or_cpp_native_project`
- Archetype id: `c_or_cpp_native_project`
- Initial support class: `experimental`
- Target support class: `certified`
- Inclusion target: `first_stable`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

CMake with Ninja, `clangd` as the language server, and `lldb` or `gdb`
as the debugger. The row is shaped around a compile-database-driven
project; alternative build systems (Bazel, Buck) are out of scope for
the certified row at first stable.

## Required-mode rationale

- `local_only` — native build, debug, and symbol-navigation flows are
  fully covered on a developer machine. The row deliberately does not
  promise a remote-attach or container baseline at first stable.

## Evidence already on file

- Reference workspace: `refws.c_cpp_native_archetype_seed`
  ([fixture](../../workspaces/reference/c_cpp_native_archetype_seed.json)).
- Beta packet and harness:
  `fixtures/reference_workspaces/m3/cpp_native/workspace.yaml`,
  `fixtures/reference_workspaces/m3/cpp_native/harness.yaml`.
- Corpus scenarios: `corpus.reference.c_cpp_native_archetype_seed`,
  `corpus.archetype.c_cpp_native_seed`,
  `corpus.workflow.first_useful_edit_c_cpp_native`.
- Design-partner input class: `sanitised_repo_admissible`.

## Open evidence questions

- Capture current pass/fail results for the seeded workflow harness
  before any graduation step.
- Decide whether `clangd` is the only certified language-server path
  or whether the row admits an alternate analyzer with caveats.
- Capture the debugger trust posture: `lldb` and `gdb` differ on
  symbol-loading, JIT-aware breakpoints, and hardware-watchpoint
  semantics; the certified row must name which behaviour is
  guaranteed.
