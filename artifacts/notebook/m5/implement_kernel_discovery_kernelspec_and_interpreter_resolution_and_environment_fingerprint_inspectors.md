# Kernel discovery, kernelspec, interpreter resolution, and environment fingerprint inspectors

## Checked-in artifact

This artifact is the canonical packet for the M05-015 kernel discovery,
kernelspec, interpreter resolution, and environment fingerprint inspector
lane. It is embedded in `aureline-notebook` via `include_str!` and consumed
by docs, help, support, and CI surfaces.

### Packet contents

- **Closed vocabularies**: All 5 enum classes are listed in full so consumers
  can verify coverage.
- **Worked example kernelspecs**: Python 3.12 (local), R 4.3.0 (conda), Julia
  1.10 (remote).
- **Worked example interpreter resolutions**: uv-managed Python, conda-managed
  R, unresolved unknown.
- **Worked example environment fingerprints**: Fresh local, stale local,
  policy-blocked remote.
- **Worked example kernel discovery entries**: Compatible available local,
  compatible busy remote, policy-blocked remote.

### Validation

The packet validates against:
- `schemas/notebook/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors.schema.json`
- The Rust `KernelDiscoveryPacket::validate()` truth rules in
  `crates/aureline-notebook`

### Freshness

- Packet ID: `nb.kernel_discovery.packet.m5.01`
- Current as of: `2026-06-09T00:00:00Z`
- Schema version: `1`
