# M5 Lifecycle and Channel Badge Primitive

- Packet: `m5-lifecycle-and-channel-badge-primitive:stable:0001`
- Label: `M5 lifecycle and channel badge primitive: labs/preview/beta/stable/lts-surface/deprecated/removal-scheduled lifecycle and nightly/preview/beta/stable/lts channel as two distinct, composable cues`
- Badge consumers: 6 (6 stable)
- Lifecycle values: labs, preview, beta, stable, lts_surface, deprecated, removal_scheduled
- Channel values: nightly, preview, beta, stable, lts
- Effective-maturity postures: maturity_experimental, maturity_preview, maturity_beta, maturity_stable, maturity_long_term_supported, maturity_deprecated, maturity_removal_scheduled
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Badge consumers

- **Command Row**: `stable`
  - Owner: Command badge owner
  - Scope: The command row renders the shared lifecycle and channel badges as two distinct cues so a stable command on the stable channel reads as a stable-maturity claim, while a deprecated command still points to its native replacement command with a migration note that preserves the channel it was running on and offers a follow-migration-path next action
  - Worked resolutions: 2
    - lifecycle `stable` + channel `stable` → `maturity_stable` (note `no_migration`)
    - lifecycle `deprecated` + channel `stable` → `maturity_deprecated` (note `deprecated`)
- **Feature Surface**: `stable`
  - Owner: Feature badge owner
  - Scope: The feature surface renders the shared badges so a beta feature on the beta channel reads as beta-maturity, while a stable feature merely running on the preview channel still reads as a stable-maturity claim — proving the lifecycle never implies the channel and a preview channel never implies an experimental lifecycle
  - Worked resolutions: 2
    - lifecycle `beta` + channel `beta` → `maturity_beta` (note `no_migration`)
    - lifecycle `stable` + channel `preview` → `maturity_stable` (note `no_migration`)
- **Workflow Bundle**: `stable`
  - Owner: Workflow bundle badge owner
  - Scope: The workflow bundle launch card renders the shared badges so an LTS-surface bundle on the LTS channel reads as long-term-supported, while a bundle with a scheduled removal date points to its replacement bundle with a complete-migration-before-removal next action rather than becoming an inert warning
  - Worked resolutions: 2
    - lifecycle `lts_surface` + channel `lts` → `maturity_long_term_supported` (note `no_migration`)
    - lifecycle `removal_scheduled` + channel `stable` → `maturity_removal_scheduled` (note `removal_scheduled`)
- **Extension / Install Row**: `stable`
  - Owner: Extension install badge owner
  - Scope: The extension / install row renders the shared badges so a labs extension on the nightly channel reads as experimental-maturity and a preview extension on the preview channel reads as preview-maturity — the same two-cue vocabulary an install reviewer reads elsewhere, with the lifecycle and channel stated separately
  - Worked resolutions: 2
    - lifecycle `labs` + channel `nightly` → `maturity_experimental` (note `no_migration`)
    - lifecycle `preview` + channel `preview` → `maturity_preview` (note `no_migration`)
- **Release / Install Surface**: `stable`
  - Owner: Release install badge owner
  - Scope: The release / install surface renders the shared badges so a stable release on the stable channel reads as stable-maturity, while an LTS-surface capability being validated on the nightly channel still reads as long-term-supported — the channel a thing is running on never narrows or widens its lifecycle stage
  - Worked resolutions: 2
    - lifecycle `stable` + channel `stable` → `maturity_stable` (note `no_migration`)
    - lifecycle `lts_surface` + channel `nightly` → `maturity_long_term_supported` (note `no_migration`)
- **Ecosystem Lifecycle Review**: `stable`
  - Owner: Ecosystem lifecycle badge owner
  - Scope: The ecosystem lifecycle review lane renders the shared badges so a deprecated capability under review on the beta channel still points to its replacement with a preserved channel context, and a beta capability promoted to the stable channel still reads as beta-maturity — support for lifecycle and channel stay separate facts a reviewer reads together
  - Worked resolutions: 2
    - lifecycle `deprecated` + channel `beta` → `maturity_deprecated` (note `deprecated`)
    - lifecycle `beta` + channel `stable` → `maturity_beta` (note `no_migration`)
