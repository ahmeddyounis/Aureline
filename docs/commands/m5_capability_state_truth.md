## M5 capability-state truth

This contract defines the export-safe packet that projects M5 capability
lifecycle state and dependency-marker truth from the canonical command graph
into the surfaces that most easily overclaim stability:

- settings rows
- Help / About
- docs packs and command-reference surfaces
- release rows
- workflow bundles
- profile exports
- support packets
- desktop, CLI/headless, extension-metadata, and browser / companion inspectors

The runtime owner is [`aureline_commands::m5_capability_state_truth`]. It reads
the seeded M5 command-governance packet rather than inventing a second source of
truth, so lifecycle state, rollout refs, and route metadata stay aligned with
the command-owned parity packet.

### Required invariants

- The packet carries canonical state definitions for `Labs`, `Preview`, `Beta`,
  `Stable`, `Deprecated`, `DisabledByPolicy`, `RetestPending`, and `Removed`.
- Every M5 command family has one lifecycle-state row plus projection rows for
  settings, help/About, docs, release, workflow bundle, profile export, support
  packet, desktop inspector, CLI inspect, extension metadata, and browser /
  companion surfaces.
- Claim-bearing and saved-artifact projections preserve dependency markers
  instead of silently inheriting stable-looking wording from a narrower
  capability state.
- Stable-facing help/docs/release/workflow-bundle/profile-export rows fail
  closed: if the effective state is not `Stable`, if dependency markers are
  present, or if freshness has lapsed into `RetestPending`, stable wording stays
  hidden.
- `DisabledByPolicy` and `RetestPending` rows automatically narrow support
  wording and badges rather than leaving old green support copy in place.
- Desktop, CLI/headless, extension metadata, and browser / companion surfaces
  keep lifecycle state inspectable through stable detail refs plus the command
  route or handoff metadata.

### Seeded M5 posture

The seeded packet adapts the current codebase as follows:

- most M5 families remain explicitly `Beta`, and every claim-bearing surface
  carries a `beta_dependency` marker instead of implying stable support or
  portability;
- `trace_replay.replay_session` narrows to `Preview`, reflecting its structured
  preview dependency;
- `preview.open_live_preview` narrows to `Labs`, reflecting its experimental
  live-preview runtime posture;
- `sync.push_workspace_state` narrows to `DisabledByPolicy`, showing the
  managed-policy ceiling instead of disappearing; and
- `docs_browser.open_external` narrows to `RetestPending`, demonstrating the
  required proof-freshness narrowing path across help/docs/export consumers.

### Generated artifacts

- Packet fixture: `fixtures/commands/m5_capability_state_truth/packet.json`
- Support export: `artifacts/commands/m5_capability_state_truth/support_export.json`
- Summary: `artifacts/commands/m5_capability_state_truth/summary.md`
- Schema: `schemas/commands/m5_capability_state_truth.schema.json`

### Regeneration

```bash
cargo run -q -p aureline-commands --bin aureline_commands -- m5-capability-state-truth json
cargo run -q -p aureline-commands --bin aureline_commands -- m5-capability-state-truth support-export
cargo run -q -p aureline-commands --bin aureline_commands -- m5-capability-state-truth summary
```
