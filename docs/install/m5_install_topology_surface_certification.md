# M5 install-topology surface certification (M05-1179)

This contract is the **closing B140 surface-certification capstone** over the frozen M5 install-topology
matrix (`m5_install_topology_matrix`). Where the freeze matrix defines the five governed delivery-topology
families — **per-user-managed, per-machine-managed, side-by-side-stable-preview, portable-mode, and
offline-air-gap-bundle** — the 1173–1176 implementation lanes resolve their per-surface install-mode,
updater-ownership, state-root, channel-isolation, managed-operation, and rollback truth, the 1177 shared-consumer
lane aligns their grammar across surfaces, and the 1178 accessibility lane proves keyboard / screen-reader /
high-zoom / high-contrast / localization / CLI-export parity, this capstone **certifies** that the shared
install-topology truth holds on every claimed M5 **delivery profile** and auto-narrows any profile that cannot
sustain it.

- **Module:** `crates/aureline-ui/src/m5_install_topology_surface_certification/`
- **Schema:** `schemas/install/m5-install-topology-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-install-topology-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/install/m5-install-topology-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a user, reviewer, admin, or support engineer reads an
install-mode, updater-ownership, state-root, repair/verify, rollback, or rollout-ring surface through, not on the
reusable delivery-topology family it renders:

1. **Live trusted delivery surface** — a live, first-party, fully-current install topology. The **only** profile
   that may certify a `trusted_delivery_surface` claim.
2. **Reviewable delivery structure** — a self-sufficient, inspectable install-mode / policy-control / registry
   reference; certifies at most `reviewable_delivery_surface`.
3. **Disclosed state-boundary profile** — a side-by-side stable/preview surface whose isolation proof can only be
   partially disclosed; auto-narrows to `state_boundary_disclosed_projection`.
4. **Unverified repair/verify profile** — a portable / offline surface whose repair/verify coverage cannot be
   confirmed; auto-narrows to `repair_verify_unverified_projection`.
5. **Unverified rollout-evidence profile** — a surface whose rollout-ring promotion / rollback evidence has aged
   out or is policy-blocked; auto-narrows to `rollout_evidence_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and install-topology-component-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed delivery tier delivered;
- **yellow** — a truth axis is not current, so the delivery claim auto-narrows to the weakest supported ceiling
  with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh trusted claim, a hard invariant breaks, CLI/export
  parity drops, a non-live profile claims a trusted delivery surface, or the narrowing is inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a `trusted_delivery_surface`
   / `reviewable_delivery_surface` claim while one of its truth axes is not current over-claims and blocks.
2. **Only a live first-party profile may certify a trusted delivery surface.** Every other profile is at most a
   reviewable delivery structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the canonical
   install mode, updater owner, binary root, writable state roots, policy roots, rollback target, rollout ring,
   and registry reference as text / JSON / Markdown.
4. **Every B140 hard invariant holds per row.** No profile may let portable mode write hidden machine-global
   durable state, let a preview channel reuse a stable state namespace without an explicit import / handoff,
   narrow a rollback to only the primary executable while sidecars or metadata drift, hide updater ownership or
   admin control in a managed flow, or publish a deployment claim that outpaces ring or repair/verify evidence.
5. **One canonical proof bundle.** Every row cites exactly one canonical install-topology proof bundle
   (`artifacts/release/m5-install-topology-proof/support_export.json`) — the frozen install-topology matrix
   proof — so release, docs, and support consume a single delivery-topology certification source rather than
   hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_INSTALL_TOPOLOGY_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_install_topology_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
