# Kernel discovery, kernelspec, interpreter resolution, and environment fingerprint inspectors

## What this lane produces

This document describes the M05-015 implementation for kernel discovery,
kernelspec typing, interpreter resolution, and environment fingerprint
inspectors in Aureline.

### Records

- **`Kernelspec`** — Stable kernelspec identity with human-readable
  `display_name_label` and `language_label`, plus opaque refs to launch
  commands, resources, and metadata so raw filesystem paths never cross the
  boundary.
- **`InterpreterResolution`** — Captures interpreter version, environment
  manager class (`uv`, `venv`, `poetry`, `conda`, `pyenv`, `system`,
  `unknown`), and source manifest provenance.
- **`EnvironmentFingerprint`** — Human-readable environment identity,
  package/toolchain summaries, target origin, policy epoch, and freshness
  class (`fresh`, `stale`, `unresolved`, `policy_blocked`).
- **`KernelDiscoveryEntry`** — Binds a discovered kernelspec to its discovery
  source, interpreter resolution, environment fingerprint, and
  compatibility/availability state.
- **`KernelDiscoveryPacket`** — Checked-in artifact containing all closed
  vocabularies and worked examples.

### Closed vocabularies

| Vocabulary | Variants |
|---|---|
| `KernelspecDiscoverySourceClass` | `jupyter_data_dir`, `conda_env`, `virtual_env`, `system_path`, `remote_registry`, `managed_workspace`, `remote_agent`, `browser_bridge` |
| `InterpreterManagerClass` | `uv`, `venv`, `poetry`, `conda`, `pyenv`, `system`, `unknown` |
| `EnvironmentFingerprintFreshnessClass` | `fresh`, `stale`, `unresolved`, `policy_blocked` |
| `KernelDiscoveryCompatibilityClass` | `compatible`, `incompatible_language`, `incompatible_version`, `policy_narrowed`, `unresolved_dependencies` |
| `KernelDiscoveryAvailabilityClass` | `available`, `starting`, `busy`, `unavailable`, `policy_blocked` |

### Boundaries

- **No raw paths**: `launch_command_template_ref` and
  `interpreter_path_token_ref` are opaque refs, not raw filesystem paths.
- **No raw notebook payloads**: No cell source, output bytes, kernel protocol
  frames, widget state, or URLs appear on any record.
- **Human-readable by default**: Environment fingerprints communicate identity
  in labels the user can read and audit.

### Artifacts

| Artifact | Path |
|---|---|
| Checked-in packet JSON | `artifacts/notebook/m5/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors.json` |
| Boundary schema | `schemas/notebook/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors.schema.json` |
| Worked fixtures | `fixtures/notebook/m5/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors/` |
| Implementation | `crates/aureline-notebook/src/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors/` |

### Downstream consumers

- `aureline-notebook::runtime_truth` — `KernelSessionSummary` carries
  `kernelspec_ref` and `environment_fingerprint_ref` opaque refs that resolve
  to records shaped by this module.
- `aureline-notebook::implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state` —
  The kernel bar presents discovered kernels and their compatibility state.
- `aureline-runtime` — `ExecutionContextResolver` and environment detectors
  produce the raw data that feeds into these typed records.

## Guardrails

- Discovery entries never silently claim `compatible` when the availability is
  `policy_blocked`.
- Remote discovery sources (`remote_registry`, `managed_workspace`,
  `remote_agent`, `browser_bridge`) must not claim `local_host` target origin.
- `fresh` fingerprints must carry a `last_known_good_at` timestamp.
- `policy_blocked` fingerprints must carry a `policy_epoch_ref`.
