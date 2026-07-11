# M5 host-boundary-strip and execution-origin-receipt-row controls

This is the second implement lane over the frozen
[M5 build/remote-boundary component matrix](m5_build_remote_boundary_components_contract.md). It
turns the two runtime-ownership components the matrix names — the **host-boundary strip** and the
**execution-origin receipt row** — into resolvers that produce export-safe, honest projections
instead of feature-local host or origin copy.

The authoritative gate is the Rust validator in `crates/aureline-remote`
(`implement_the_m5_host_boundary_strip_and_execution_origin_receipt_row_..._primitive`). The
checked-in support export under
`artifacts/release/m5-host-boundary-strip-execution-origin-receipt-row-controls-proof/` and the
narrowed fixtures under
`fixtures/ui/m5-host-boundary-strip-execution-origin-receipt-row-controls/` are minted only by the
seed builders through the headless emitter
`cargo run -p aureline-remote --example dump_m5_host_origin_controls`.

## Goal

Let users orient to where work is running before they trust logs, previews, shells, or actions, and
keep host ownership and target lineage legible in diagnostics, support exports, and release evidence.

## Components

### Host-boundary strip

`resolve_host_boundary_strip` renders every strip with:

- the current locality class (`local`, `ssh`, `container`, `devcontainer`, `managed`,
  `browser_bridge`, or `service_plane`), derived from the bound `HostKind` and a devcontainer flag,
- the resolved target label,
- the owning runtime / service lane,
- the connection state (bound from `ConnectionState`) and, whenever the connection is impaired, an
  explicit reconnect / degraded state, and
- an open-details affordance.

A strip that leaves its locality class, target label, or owning runtime / service lane unstated
degrades rather than reading as a clean pass, and an impaired (bridged, reconnecting, or stale)
connection that hides its degraded state degrades too, so host ownership never disappears on degrade.

### Execution-origin receipt row

`resolve_execution_origin_receipt_row` renders every receipt with:

- the action class it attests,
- the resolved target identity,
- the host kind and derived origin locus (bound from `HostKind` / `OriginLocus`),
- the receipt state (bound from `OriginReceiptState`), connection state, and derived origin
  confidence (bound from `AttributionConfidence`), plus any host-narrowing reason (bound from
  `HostNarrowingReason`), and
- an export-safe target lineage that stays stable enough for diagnostics, support, and evidence to
  reuse.

A receipt that leaves its action class, resolved target identity, or execution-context provenance
unstated degrades. A receipt whose lineage is not export-safe / reusable degrades to
`lineage_not_export_safe` (AC2). A restore, handoff, or degrade that drops the execution origin
degrades to `ownership_dropped_on_restore` — the host-ownership guardrail — so host ownership can
never silently vanish on recovery.

## Acceptance criteria

- **AC1** — users can distinguish local, SSH, container, devcontainer, managed, and browser-bridge
  execution without opening a separate inspector.
- **AC2** — receipts remain stable enough for diagnostics, support exports, and release evidence to
  reuse them without rewriting target lineage.
- **AC3** — host ownership never disappears when a surface is restored, handed off, or degraded.

Each criterion is proven by the resolved examples the packet carries, not merely asserted by
governance flags.

## Hard invariants (every controls row)

- never hide the host locality class or owning runtime / service lane,
- never drop the execution origin when a surface is restored, handed off, or degraded,
- never publish a receipt whose lineage is not stable enough for reuse, and
- never conceal a boundary or origin behind generic status wording.

Raw secret values and private endpoints never cross this boundary.
