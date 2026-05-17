# Beta Reference Workspaces

The beta reference-workspace corpus turns JVM, Rust, Go, and C/C++
launch-wedge claims into named, reproducible product assets. The
machine-readable source is
`artifacts/compat/m3/reference_workspace_register.yaml`; the workspace
packets live under `fixtures/reference_workspaces/m3/`.

The corpus is deliberately small. A workspace only enters the register
when it has a named owner, exact toolchain manifest, declared OS/arch
and mode scope, privacy/license clearance, workflow list, and pass/fail
harness entries.

The current generated report is published at
`artifacts/compat/m3/reference_workspace_report.json` with reviewer copies at
`artifacts/compat/m3/reference_workspace_report.md` and
`docs/compat/m3/reference_workspace_report.md`. Badge projections live at
`artifacts/compat/m3/reference_workspace_badges.json`. These generated
artifacts, not this overview, cap support-class claims in the claim manifest.

## Registered Workspaces

| Workspace | Register row | Reference id | Toolchain pins | Mode scope | Claim posture |
|---|---|---|---|---|---|
| Java/Kotlin service | `m3_reference_workspace:jvm_service` | `refws.java_kotlin_service_archetype_seed` | JDK `21.0.7`, Gradle `8.10.2`, Kotlin `2.0.21`, JUnit `5.10.3` | local only | Retest pending in the generated report |
| Rust workspace | `m3_reference_workspace:rust_workspace` | `refws.small_rust_self_host_slice` | Rust `1.84.0` from `rust-toolchain.toml` | local only | Retest pending in the generated report |
| Go service / monorepo slice | `m3_reference_workspace:go_service` | `refws.go_service_archetype_seed` | Go `1.22.5`, Delve `1.23.1` | local only | Retest pending in the generated report |
| C/C++ native project | `m3_reference_workspace:cpp_native` | `refws.c_cpp_native_archetype_seed` | CMake `3.29.6`, Ninja `1.12.1`, clangd/LLDB `18.1.8`, GDB `14.2` | local only | Retest pending in the generated report |

All rows target `macos_arm64`, `macos_x86_64`, `linux_x86_64`, and
`windows_x86_64`. Remote attach, managed workspace, and devcontainer
execution are explicit non-claims for this corpus revision.

## Workflow Coverage

Every workspace packet carries harness entries for:

- benchmark first-open or first-useful-work capture;
- run target discovery and execution;
- test discovery and normalized result capture;
- debug session or explicit debug handoff/capability evidence;
- migration-scorecard probe binding to the source ecosystem rows; and
- support-export projection with redacted toolchain and target evidence.

The harnesses live beside the packets:

- `fixtures/reference_workspaces/m3/jvm_service/harness.yaml`
- `fixtures/reference_workspaces/m3/rust_workspace/harness.yaml`
- `fixtures/reference_workspaces/m3/go_service/harness.yaml`
- `fixtures/reference_workspaces/m3/cpp_native/harness.yaml`

## Governance

The register treats each workspace as a protected product asset. A row
becomes stale when its descriptor, toolchain manifest, protected
workflow set, owner, privacy posture, or archetype scorecard changes.
Design-partner evidence can map to the same archetype vocabulary, but
it is supplemental evidence only; the public beta claim must still cite
the sanitized or synthetic workspace row in the register.

## Consumers

The register is cited by:

- `artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml`
- `artifacts/compat/m3/archetype_scorecards/*.md`
- `artifacts/compat/m3/reference_workspace_report.json`
- `artifacts/compat/m3/reference_workspace_badges.json`
- `artifacts/milestones/m3/claimed_surface_register.json`
- `artifacts/benchmarks/m3/publication_packet/packet.md`
- `docs/release/certified_archetype_report_template.md`
- `docs/release/release_evidence_packet_template.md`

## Verification

Run:

```sh
python3 ci/check_m3_reference_workspace_register.py --repo-root .
```

To refresh the validation capture:

```sh
python3 ci/check_m3_reference_workspace_register.py --repo-root . --report artifacts/compat/m3/captures/reference_workspace_register_validation_capture.json
```

To refresh report, badges, docs copy, and publication-gate capture:

```sh
python3 ci/check_m3_reference_workspace_report.py --repo-root .
```
