# Implement execution-surface classes, sandbox-profile descriptors, and unsupported-or-stricter-profile truth

This document is the canonical contract for the M5 execution-surface
resolution layer. It builds directly on the frozen M5 runtime-authority,
approval-ticket, sandbox-profile, and capability-envelope matrix and consumes
it rather than cloning its prose.

Where the frozen matrix states the *intended* default sandbox profile for each
claimed M5 executing surface, this layer answers the platform-specific
questions the matrix deliberately leaves open:

- Which concrete launch path (task, terminal, notebook, request, database,
  debug, connector, AI tool, browser-routed action, remote mutation) is governed
  by which matrix authority row?
- What is the stable profile **id**, **version**, and **backend class** an
  operator sees?
- When the default profile's enforcement backend is missing on this platform,
  does the surface stay supported, narrow to a stricter profile, fail closed, or
  disable with reason — and what capability envelope remains?

- Implementation: `crates/aureline-policy/src/implement_execution_surface_classes_sandbox_profile_descriptors_and_unsupported_or_stricter_profile_truth/`
- Boundary schema: `schemas/execution-auth/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.schema.json`
- Support export (truth source): `artifacts/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/support_export.json`
- Markdown summary: `artifacts/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.md`
- Per-platform fixtures: `fixtures/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/`
- Producer / validator: `cargo run -p aureline-policy --example dump_m5_execution_surface_resolution`

## Track invariant

Resolution never widens authority. A surface whose default isolation backend is
unavailable narrows to a **strictly more isolated** profile (ultimately inert
read-only) or fails closed; it never falls back to a less isolated profile, and
it never silently widens. Missing profile coverage narrows the affected rows
instead of letting them masquerade as parity-ready.

## Execution-surface classes (launch-path taxonomy)

Every concrete M5 launch path binds to exactly one matrix surface whose
authority row governs its profile, ticket posture, and capability envelope.
Several launch paths can share one row.

| Launch path | Governing matrix surface | Backend class |
| --- | --- | --- |
| `task_execution` | `scaffold_hook` | `local_isolated` |
| `terminal_session` | `notebook_kernel` | `local_isolated` |
| `notebook_cell` | `notebook_kernel` | `local_isolated` |
| `debug_session` | `notebook_kernel` | `local_isolated` |
| `request_send` | `request_api_send` | `brokered_network` |
| `connector_action` | `request_api_send` | `brokered_network` |
| `database_query` | `database_action` | `brokered_network` |
| `ai_tool_call` | `ai_tool` | `local_isolated` |
| `browser_routed_action` | `browser_routed_action` | `remote_isolated` |
| `remote_mutation` | `remote_mutation` | `remote_isolated` |

## Sandbox-profile descriptors

Each frozen sandbox profile carries a stable, dotted **profile id**, a
**version**, a coarse **backend class**, an **isolation rank** (higher is
stricter), a **network-lane** flag, an isolation summary, and a **capability
ceiling** (the maximum classes the profile can host). Desktop, CLI/headless,
diagnostics, and support surfaces display these exact fields.

| Profile | Backend class | Isolation rank | Network lane |
| --- | --- | --- | --- |
| `sandbox.in_process_trusted_local` | `in_process` | 0 | no |
| `sandbox.subprocess_isolated_local` | `local_isolated` | 1 | no |
| `sandbox.container_isolated_local` | `local_isolated` | 2 | no |
| `sandbox.isolated_remote_runtime` | `remote_isolated` | 3 | no |
| `sandbox.brokered_network_only` | `brokered_network` | 4 | yes |
| `sandbox.inert_no_execution` | `inert` | 5 | no |

The local-execution narrowing ladder is ordered by isolation rank. The
network-broker lane is never a narrowing target for a code-executing surface.

## Platform support table

| Platform | In-process | Subprocess | Container | Remote | Brokered network | Inert |
| --- | --- | --- | --- | --- | --- | --- |
| `linux_desktop` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `macos_desktop` | ✓ | ✓ |  | ✓ | ✓ | ✓ |
| `windows_desktop` | ✓ | ✓ |  | ✓ | ✓ | ✓ |
| `managed_remote_runtime` |  | ✓ | ✓ | ✓ | ✓ | ✓ |
| `headless_ci` |  | ✓ |  |  | ✓ | ✓ |

## Resolution behavior

For each surface on each platform the resolver compares the default profile to
the platform support table:

- **Supported** — the default profile's backend is available; the full
  capability envelope is honored.
- **Narrowed to stricter profile** — the default backend is missing and the
  matrix `unsupported_profile_behavior` is `narrow_to_stricter_profile`; the
  least-disruptive strictly-more-isolated available profile is chosen, the
  qualification narrows one notch, and stripped capabilities are listed. For
  example, a container-isolated preview server narrows to the isolated remote
  runtime on macOS/Windows and to inert read-only on the headless CI runner.
- **Unsupported / fail closed** — the default backend is missing and the matrix
  behavior is `fail_closed_unsupported` (or no stricter profile is available);
  the surface has no effective profile and its qualification is `unavailable`.
  Browser-routed actions and remote mutations fail closed on the headless CI
  runner.
- **Disabled with reason** — the surface is disabled with an inspectable reason
  and its qualification is `unavailable`.

Every resolved row carries a reduced-capability explanation that desktop, CLI,
diagnostics, support, and release surfaces render identically.

## Consumers

The support export is the single truth source. Desktop, command palette, policy
inspector, CLI/headless, diagnostics, help/About, support export, and release
evidence consume the same profile id, version, backend class, and
reduced-capability explanation instead of cloning per-surface prose. Narrowed or
disabled surfaces carry the Preview/Labs label.

## Regeneration

```sh
cargo run -q -p aureline-policy --example dump_m5_execution_surface_resolution \
  > artifacts/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/support_export.json
cargo run -q -p aureline-policy --example dump_m5_execution_surface_resolution -- markdown \
  > artifacts/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth.md
cargo run -q -p aureline-policy --example dump_m5_execution_surface_resolution -- platform macos_desktop \
  > fixtures/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/macos_preview_narrowed_to_remote_runtime.json
cargo run -q -p aureline-policy --example dump_m5_execution_surface_resolution -- platform headless_ci \
  > fixtures/execution-auth/m5/implement-execution-surface-classes-sandbox-profile-descriptors-and-unsupported-or-stricter-profile-truth/headless_ci_fail_closed_and_inert.json
```

A Rust test asserts the checked-in support export deserializes back to the
frozen in-code packet unchanged; drift fails CI.
