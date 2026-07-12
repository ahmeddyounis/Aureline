# M5 Permission-Manifest-Summary and Transitive-Capability-Drawer Controls

- Packet: `m5-permission-manifest-summary-transitive-capability-drawer-controls:stable:0001`
- Label: `M5 permission-manifest-summary and transitive-capability-drawer controls with required / optional / inherited capability classes, runtime / host model, data / network boundaries, and transitive-widening attribution across listing, detail, install, update, diagnostics, and export`
- Consumer surfaces: 5
- Permission postures: minimal, standard, elevated, widened_transitive, policy_restricted, posture_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **marketplace_ui**: `stable`
  - Owner: Marketplace catalog owner
  - Scope: The marketplace listing renders one permission-manifest summary per artifact naming the permission posture, required / optional / inherited capability classes, runtime / host model, and data / network boundaries so a compare decision needs no disconnected page, and degrades honestly when a capability-requesting posture names no grouping
  - Summary examples: 2 / transitive-drawer examples: 2
- **extensions_ui**: `stable`
  - Owner: Extensions manager owner
  - Scope: The extensions detail surface reuses the same grouping model, shows a transitively-widened artifact disclosing its widening with each dependency-contributed permission attributed to the dependency that contributed it, and degrades honestly when the data / network boundary or dependency attribution is hidden
  - Summary examples: 2 / transitive-drawer examples: 2
- **install_review_ui**: `stable`
  - Owner: Install-review owner
  - Scope: The install / update review sheet keeps the permission posture explicit before install trust silently continues, groups the inherited (dependency-contributed) class explicitly, and degrades honestly when the posture cannot be resolved or the drawer is severed from its canonical manifest digest
  - Summary examples: 2 / transitive-drawer examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved summary and drawer truth, so a manifest flattened into one vague full-access label, an unstated host model, a missing manifest digest, or an unattributed dependency-contributed permission is visible in evidence rather than hidden behind compact chrome
  - Summary examples: 3 / transitive-drawer examples: 2
- **product_ui**: `stable`
  - Owner: In-product diagnostics owner
  - Scope: In-product listing and diagnostics surfaces reuse the same permission grammar, keep the standard posture and its capability classes explicit, and degrade honestly when the artifact identity is missing or a transitively-widened posture hides its widening so no widened trust is quietly carried forward into installed-state diagnostics
  - Summary examples: 2 / transitive-drawer examples: 2
