# M5 retirement-tombstone and claim-block-gate registries

This lane adds retired-state tombstones and claim-block logic to the first consumer surfaces — install / update
pickers, marketplace / detail surfaces, help / About truth cards, CLI / headless inspect paths, and managed
new-tenant / new-workspace creation flows — over the frozen
[M5 retired-state matrix](./m5-retired-state-ops.md), so a retired M5 line or stable-facing surface stops looking
selectable or claimable while staying discoverable historically rather than disappearing silently or lingering as a
stale green selection. It emits one export-safe *retirement tombstone* per retired object — binding the stable
identity anchor, last-supported version marker, archival pointer, replacement / successor path, and removed
active-selection affordance (green / support badges and active enablement removed) to one retired-object identity —
and one typed *claim-block gate* per object that blocks it from being offered for new install, new tenant, or active
enablement. It records the *retirement-tombstone* grammar (one classified tombstone field per preserved fact —
stable identity anchor, last-supported version marker, archival pointer, replacement / successor path, removed
active-selection affordance, or historical-discoverability note — carrying its owning team and joined to the
retirement manifest and impact report) and the *claim-block-gate* grammar (the enablement flow a claim-block sits
in — new-install-selection, new-tenant-provisioning, or active-enablement-toggle, naming the active block reason)
into registry resolvers that produce export-safe, honest projections, so help / About, marketplace, and CLI /
headless inspection agree on one retired-state truth for the same object.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_retirement_tombstone_and_claim_block_gate_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-retirement-tombstone-and-claim-block-gate-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-retirement-tombstone.schema.json`](../../schemas/program/m5-retirement-tombstone.schema.json)
  (minted by this lane — the tombstone each retired object is recorded against)
  and
  [`schemas/program/m5-claim-block-gate.schema.json`](../../schemas/program/m5-claim-block-gate.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-retirement-tombstone-and-claim-block-gate-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first machine-readable
  retirement tombstone / claim-block loop — it demonstrates one tombstone / claim-block-gate loop end to end for the
  first retirement-bearing surfaces.
- **Narrowed fixtures:**
  `fixtures/release/m5-retirement-tombstone-and-claim-block-gate-registries/`
  (`retirement_tombstone_beta_narrowed.json`, `claim_block_gate_preview_narrowed.json`).

## Two registries

1. **Retirement tombstone** (`resolve_retirement_tombstone_entry`) — publishes one tombstone field per retired
   object: the classification (stable identity anchor, last-supported version marker, archival pointer,
   replacement / successor path, removed active-selection affordance, historical-discoverability note) and its
   canonical mode, the exact-build joins (repo rows, bundle IDs, install topology, toolchain envelope), the
   compatibility / known-limits state, the archival pointer / replacement path, and the owning team. A clean entry
   names a canonical registry token, a classified tombstone field, and a retirement role, covers the canonical /
   accessible / audit resolution forms, publishes a complete object, removes the active-selection affordance, and
   keeps a public-facing replacement / affordance field matched to the archived successor. Otherwise it degrades
   honestly.
2. **Claim-block gate** (`resolve_claim_block_gate_entry`) — surfaces a retired object's claim-block list before it
   can be newly selected. A clean entry names a classified claim-block scope (new-install-selection,
   new-tenant-provisioning, or active-enablement-toggle) and provides the complete gate object; a gate that would
   keep offering the retired object, hide the block, or let a retired surface masquerade as selectable degrades.

## Acceptance criteria (proven by resolved examples)

- **A retired line or capability is no longer offered for new install, new tenant, or active enablement in the first
  consumer surfaces.** Clean tombstone entries cover the canonical stable-identity-anchor /
  last-supported-version-marker / archival-pointer / replacement-path-pointer / removed-active-affordance-marker /
  historical-discoverability-note fields and the first release-center / help-docs / support / marketplace-registry /
  install-update surfaces, a still-selectable example degrades, and no clean tombstone entry left the active
  affordance in place.
- **Historical / tombstone views still expose stable identity, last supported version, and replacement / archive
  pointers instead of disappearing entirely.** A tombstone that would present a retired object as an active choice
  degrades, an unbound example degrades, a clean tombstone entry is present, and no clean entry is incomplete or
  unbound.
- **Help / About, marketplace, and CLI / headless inspection agree on retired-state truth for the same seeded
  object.** Clean claim-block-gate entries cover the new-install-selection / new-tenant-provisioning /
  active-enablement-toggle claim-block scopes with full resolution-form coverage while providing the complete gate
  object, and a gate that would keep offering the retired object or drop the block degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_retirement_tombstone_and_claim_block_gate_registries -- support-export
cargo run -p aureline-ui --example dump_m5_retirement_tombstone_and_claim_block_gate_registries -- csv
cargo run -p aureline-ui --example dump_m5_retirement_tombstone_and_claim_block_gate_registries -- report
cargo run -p aureline-ui --example dump_m5_retirement_tombstone_and_claim_block_gate_registries -- retirement-tombstone-table
cargo run -p aureline-ui --example dump_m5_retirement_tombstone_and_claim_block_gate_registries -- fixture-retirement-tombstone-beta-narrowed
cargo run -p aureline-ui --example dump_m5_retirement_tombstone_and_claim_block_gate_registries -- fixture-claim-block-gate-preview-narrowed
```
