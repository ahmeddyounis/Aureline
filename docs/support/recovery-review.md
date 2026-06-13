# Recovery Review Support Contract

This contract defines the support-facing packet for bounded crash-loop recovery,
scoped reset or reattach review, quarantine or rollback review, and continuity
proof across M5 host families.

## Source records

- `fault_domain_view_packet` keeps host-lane identity, restart budgets, crash
  or quarantine banners, reattach reviews, visible partial-truth labels, and
  lane-filtered event markers together.
- `crash_store_viewer_packet` keeps exact-build identity, crash ids, restart
  lineage refs, and export-safe crash actions together.
- `crash_loop_recovery_center_record` keeps the bounded recovery choices,
  reopen mode, recent suspect changes, and preserved evidence for repeated
  failure together.

## Recovery-review packet

`recovery_review_packet` is metadata-only. It includes:

- `recovery_continuity_row` entries proving failures stayed local to one host
  lane and fault domain while other surfaces remained available.
- `crash_loop_review_row` entries carrying the visible crash id, build id,
  session ref, reopen mode, safe-mode path, open-without-restore path, targeted
  disable paths, logs path, and export path.
- `scoped_reset_review_row` entries comparing previous and current host
  identity, preserved state, lost state, replay risk, rerun requirement, and
  approval/auth drift before the lane is treated as current again.
- `quarantine_review_row` entries exposing the trigger, scope, evidence,
  recovery action, rollback candidate, and support/export path for repeated
  failure or restart-budget abuse.

Raw payloads, command lines, filesystem paths, dumps, credentials, prompt
bodies, provider responses, and workspace contents do not cross this boundary.

## Required truth

- crash-loop review keeps exact build and session identity visible
- scoped reset review blocks hidden rerun and shows preserved versus lost state
- quarantine review preserves rollback and support/export paths
- bounded continuity rows prove surrounding layout and checkpoints stayed
  available for notebook kernels, adapters, preview servers, provider-run
  sessions, and connectors
- fault-domain tokens remain stable across docs, support export, and shell
  recovery surfaces
