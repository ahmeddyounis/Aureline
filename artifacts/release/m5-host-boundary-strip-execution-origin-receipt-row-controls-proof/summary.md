# M5 Host-Boundary-Strip and Execution-Origin-Receipt-Row Controls

- Packet: `m5-host-boundary-strip-execution-origin-receipt-row-controls:stable:0001`
- Label: `M5 host-boundary-strip and execution-origin-receipt-row controls with locality class, target label, owning runtime/service lane, reconnect/degraded state, action class, resolved target identity, execution-context provenance, and export-safe lineage truth`
- Consumer surfaces: 5
- Localities: local, ssh, container, devcontainer, managed, browser_bridge, service_plane
- Proof freshness SLO: 168 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **run_test_debug_ui**: `stable`
  - Owner: Run/test/debug surface owner
  - Scope: Every run, test, and debug target renders a host-boundary strip naming its locality class, target label, and owning runtime/service lane before the user trusts logs or actions; an execution-origin receipt row names the action class, resolved target identity, execution-context provenance, and export-safe lineage
  - Strip examples: 3 / receipt examples: 2
- **preview_ui**: `stable`
  - Owner: Preview surface owner
  - Scope: Preview targets reuse the same host-boundary strip vocabulary, distinguishing container and devcontainer execution and degrading honestly when the owning lane is unstated; the execution-origin receipt keeps host ownership through a restore instead of dropping it
  - Strip examples: 3 / receipt examples: 2
- **companion_ui**: `stable`
  - Owner: AI tool-routing owner
  - Scope: AI tool routing reads the same host-boundary strip so a managed or browser-bridge target is distinguishable before the model runs, debugs, or hands off work; the execution-origin receipt carries host ownership through a reconnecting restore so ownership never disappears
  - Strip examples: 2 / receipt examples: 1
- **incident_ui**: `stable`
  - Owner: Incident/ops owner
  - Scope: Incident and ops surfaces keep the same host-boundary language, distinguishing service-plane execution and degrading honestly when a reconnecting host hides its degraded state; the execution-origin receipt degrades rather than publish an unstated target identity
  - Strip examples: 2 / receipt examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved strip and receipt truth, so a stale provenance, an unattributed action class, or a dropped execution origin is visible in evidence rather than hidden behind feature-local prose, and the lineage stays reusable across diagnostics and release evidence
  - Strip examples: 1 / receipt examples: 3
