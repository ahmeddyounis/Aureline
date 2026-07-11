# M5 workspace-expiry-banner and local-safe-continuation-card controls

This is the fourth and final implement lane over the frozen
[M5 build/remote-boundary component matrix](m5_build_remote_boundary_components_contract.md). It
turns the two expiry / fallback components the matrix names — the **workspace-expiry banner** and
the **local-safe-continuation card** — into resolvers that produce export-safe, honest projections
instead of a generic disconnect or a silent service loss.

The authoritative gate is the Rust validator in `crates/aureline-remote`
(`implement_the_m5_workspace_expiry_banner_and_local_safe_continuation_card_..._primitive`). The
checked-in support export under
`artifacts/release/m5-workspace-expiry-banner-local-safe-continuation-card-controls-proof/` and the
narrowed fixtures under
`fixtures/ui/m5-workspace-expiry-banner-local-safe-continuation-card-controls/` are minted only by
the seed builders through the headless emitter
`cargo run -p aureline-remote --example dump_m5_expiry_continuation_controls`.

## Goal

Keep expiry and fallback honest so managed-workspace loss does not become ambiguous or
panic-inducing: a user can see exactly when a workspace expires, who or what triggered it, what
capabilities stop working, what remains local-safe, and what must be reattached or rerun.

## Components

### Workspace-expiry banner

`resolve_workspace_expiry_banner` renders every banner with:

- the exact expiry timing — the governing expiry window (`idle_window`, `hibernation_window`,
  `hard_deadline`, or `control_plane_outage`), bound from `ExpiryClass`,
- the triggering owner / source (bound from `TransitionReasonClass`),
- the affected capabilities (terminals, ports, kernels, previews, background jobs, debug sessions,
  managed services), and
- the offered actions — export-before-loss, renew, or reopen where allowed.

A banner that leaves its exact timing or its triggering source unstated degrades to
`expiry_timing_unstated` / `triggering_source_unstated` and reads as a generic disconnect or a
silent service loss rather than a clean pass. A banner that offers no export-before-loss or renew /
reopen action degrades to `export_or_renew_action_missing`, and a banner that presents a gone
runtime as exact continuity degrades to `exact_continuity_overclaimed`.

### Local-safe-continuation card

`resolve_local_safe_continuation_card` renders every card with:

- the preserved files / context that remain local-safe (working-tree files, unsaved edits,
  checkpoints, notebook inputs, command history, environment config),
- the lost live state that must be reattached or rerun (terminals, ports, kernels, previews, ...),
  and
- the next safe actions — continue locally, reconnect, or rebuild (bound from
  `RecoveryOptionClass`).

A card that leaves its preserved files / context or its lost live state unstated degrades to
`preserved_context_unstated` / `lost_live_state_unstated`. A card that offers no local-safe
continuation route degrades to `local_safe_continuation_unavailable`, and a card that claims exact
continuity over a material change degrades to `exact_continuity_overclaimed`.

## Acceptance criteria

- **AC1** — expiry events no longer appear as generic disconnects or silent service loss.
- **AC2** — users can see what remains local-safe and what must be reattached or rerun.
- Expiry and fallback states stay visible in shell, companion handoff, incident packets, and support
  exports.

Each criterion is proven by the resolved examples the packet carries, not merely asserted by
governance flags.

## Hard invariants (every controls row)

- never imply exact continuity after an expiry / material change,
- never hide local-safe continuation or companion handoff behind overflow-only affordances,
- never let an expiry event read as a generic disconnect or a silent service loss, and
- never conceal preserved-vs-lost state or next safe actions behind generic status wording.

Raw secret values and private endpoints never cross this boundary.
