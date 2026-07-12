# M5 Marketplace / Install-Component Accessibility & Auto-Narrowing (M05-1106)

This contract governs the **keyboard / screen-reader / high-zoom / reduced-motion / CLI / export
parity and honest automatic claim narrowing** capstone over the frozen M5 marketplace-install
component matrix
(`schemas/ui/m5-marketplace-install-component-matrix.schema.json`, workstream **B131**). It is the
accessibility / export / narrowing sibling of the implementation lanes (M05-1101 … M05-1105) that
resolve per-surface truth for the marketplace result row, marketplace detail fact grid, compatibility
label strip, permission-manifest summary, activation-budget band, install/update/disable/rollback
review sheet, publisher-continuity row, and installed-state diagnostics card.

- **Schema:** `schemas/ui/m5-marketplace-install-component-accessibility-parity.schema.json`
- **Support export (canonical):**
  `artifacts/release/m5-marketplace-install-component-accessibility-parity/support_export.json`
- **Matrix CSV:**
  `artifacts/release/m5-marketplace-install-component-accessibility-parity/matrix.csv`
- **Report:** `artifacts/release/m5-marketplace-install-component-accessibility-parity.md`
- **Mirror fixtures:**
  `fixtures/ui/m5-marketplace-install-component-accessibility-parity/`
- **Rust module:** `aureline-shell` →
  `add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_manifest_publisher_compatibility_or_activation_budget_evidence_weakens_across_claimed_m5_marketplace_components`
- **Regeneration:** `GEN_MARKETPLACE_INSTALL_A11Y_ARTIFACTS=1 cargo test -p aureline-shell --lib add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_manifest`

## What each row certifies

Each row keys on one frozen `M5MarketplaceInstallComponentFamily` and reuses the frozen required
labels, downgrade triggers, and consumer surfaces from the matrix (no parallel synonyms are minted).
A row certifies that the family:

1. **Reaches canonical truth via assistive tech.** A keyboard-complete, screen-reader-reachable,
   high-zoom-legible, reduced-motion-safe, and CLI/headless-reachable path exposes the same artifact
   identity, registry source class, compatibility range, host / runtime model, permission posture,
   activation-budget band, publisher continuity, disable scope, and rollback compatibility the rich
   component shows — never a hover-only badge. Hierarchy-heavy families (the marketplace detail fact
   grid's nested compatibility / host / permission / activation-budget / publisher / source facts)
   additionally bind their grid to a flat list / textual path.
2. **Exports without a raw payload.** The support / release / CLI export reconstructs the component's
   meaning from typed tokens and opaque refs, copyable as text / JSON / Markdown; a raw manifest body,
   permission token, or activation-budget payload is never the only export.
3. **Auto-narrows honestly.** When a marketplace / install dimension weakens, the component's claim
   lowers from `install_ready_result` / `reviewable_listing_result` to the permitted projection
   ceiling, names the binding dimension and the frozen downgrade trigger, and preserves the canonical
   identity and truth continuity.

## Claim ceilings and condition states

| Condition state | Permitted ceiling | Frozen trigger | Cannot be shown as install-ready |
| --- | --- | --- | --- |
| `fully_qualified` | `install_ready_result` | — | no (baseline) |
| `compatibility_evidence_stale` | `compatibility_unverified_projection` | `compatibility_range_unstated` | **yes** |
| `permission_evidence_partial` | `permission_unverified_projection` | `permission_widening_hidden` | **yes** |
| `activation_budget_stale` | `activation_budget_projection` | `activation_cost_hidden` | **yes** |
| `rollback_evidence_unverifiable` | `rollback_unverified_projection` | `rollback_incompatibility_hidden` | **yes** |
| `publisher_continuity_unverifiable` | `publisher_continuity_projection` | `publisher_transfer_hidden` | **yes** |
| `quarantine_history_partial` | `quarantine_history_projection` | `quarantine_history_hidden` | no (honest disclosed absence) |

A **stale compatibility signal, a partial permission manifest, a stale activation budget, an
unverifiable rollback, or an unverifiable publisher continuity** are weakened evidence: they can never
keep an `install_ready_result` claim — a stale compatibility signal never masquerades as a ready
install, and hidden permission widening never reads as cost-free. A **partial quarantine history** is
an honest disclosed-absence operation, not a truth overstatement, so it is deliberately excluded from
that flag while still narrowing the claim.

## Qualification status

- **`parity` (green):** full parity across all five reach axes and export, with no narrowing.
- **`narrowed_disclosed` (yellow):** reduced but fully disclosed, reachable, and honestly
  auto-narrowed. A dense hierarchy-heavy grid whose screen-reader traversal narrows to a disclosed
  linear walk is legitimately yellow.
- **`stranded` (red):** strands assistive tech, needs a raw payload, over-claims install-ready, or
  drops state silently. No red row may ship.

The seeded packet certifies **8 rows across all 8 families**: one green (the fully source-attributed
marketplace result row) and seven yellow (six auto-narrowed claims plus the disclosed-reduced
marketplace detail fact grid), zero red. Every dimension, every condition state, every claim tier, and
all nine consumer surfaces are exercised end-to-end.

## Guardrails

- Compact marketplace chrome never hides permission widening or activation cost — a partial permission
  manifest narrows to a `permission_unverified_projection` and a stale activation budget narrows to an
  `activation_budget_projection`.
- Publisher-transfer risk and rollback incompatibility are never hidden — an unverifiable publisher
  continuity narrows to a `publisher_continuity_projection`, and an unverifiable rollback narrows to a
  `rollback_unverified_projection` that names the disable scope and rollback limits before mutation.
- Public versus mirrored versus enterprise registry source class stays explicit before mutation or
  help / export handoff — the source class is a required label on every row.
- The packet is metadata-only: raw manifest bodies, permission tokens, activation-budget payloads,
  credentials, secrets, and endpoint refs never cross the boundary.
