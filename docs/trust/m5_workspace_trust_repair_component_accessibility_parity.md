# M5 Workspace-Trust / Repair-Component Accessibility & Auto-Narrowing (M05-1098)

This contract governs the **keyboard / screen-reader / high-zoom / reduced-motion / CLI / export
parity and honest automatic claim narrowing** capstone over the frozen M5 workspace-trust-repair
component matrix
(`schemas/ui/m5-workspace-trust-repair-component-matrix.schema.json`, workstream **B130**). It is the
accessibility / export / narrowing sibling of the eight implementation lanes (M05-1093 … M05-1097)
that resolve per-surface truth for the workspace-trust banner, trust-fact grid, trust-elevation
sheet, restricted-capability row, root-trust strip, repair-transaction preview card, rollback-class
strip, and repair-result receipt row.

- **Schema:** `schemas/ui/m5-workspace-trust-repair-component-accessibility-parity.schema.json`
- **Support export (canonical):**
  `artifacts/release/m5-workspace-trust-repair-component-accessibility-parity/support_export.json`
- **Matrix CSV:**
  `artifacts/release/m5-workspace-trust-repair-component-accessibility-parity/matrix.csv`
- **Report:** `artifacts/release/m5-workspace-trust-repair-component-accessibility-parity.md`
- **Mirror fixtures:**
  `fixtures/ui/m5-workspace-trust-repair-component-accessibility-parity/`
- **Rust module:** `aureline-shell` →
  `add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_trust_lineage_policy_epoch_checkpoint_state_or_reversal_evidence_weakens_across_claimed_m5_trust_and_repair_components`
- **Emitter:** `cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_a11y -- <support-export|report|csv|validate>`

## What each row certifies

Each row keys on one frozen `M5WorkspaceTrustRepairComponentFamily` and reuses the frozen required
labels, downgrade triggers, and consumer surfaces from the matrix (no parallel synonyms are minted).
A row certifies that the family:

1. **Reaches canonical truth via assistive tech.** A keyboard-complete, screen-reader-reachable,
   high-zoom-legible, reduced-motion-safe, and CLI/headless-reachable path exposes the same object
   identity, trust class, grant source, policy epoch, per-root trust, narrowed capability, checkpoint
   state, reversal class, and repair outcome the rich component shows — never a hover-only badge.
   Hierarchy-heavy families (the trust-fact grid's nested actor / object / scope / policy-source /
   capability-delta facts) additionally bind their grid to a flat list / textual path.
2. **Exports without a raw payload.** The support / release / CLI export reconstructs the component's
   meaning from typed tokens and opaque refs, copyable as text / JSON / Markdown; a raw grant token,
   policy body, or checkpoint payload is never the only export.
3. **Auto-narrows honestly.** When a trust / repair dimension weakens, the component's claim lowers
   from `full_trust_reviewed_result` / `reviewable_result` to the permitted projection ceiling, names
   the binding dimension and the frozen downgrade trigger, and preserves the canonical identity and
   truth continuity.

## Claim ceilings and condition states

| Condition state | Permitted ceiling | Frozen trigger | Cannot be shown as full trust |
| --- | --- | --- | --- |
| `full_trust_reviewed` | `full_trust_reviewed_result` | — | no (baseline) |
| `trust_lineage_stale` | `stale_lineage_projection` | `grant_source_unstated` | **yes** |
| `policy_epoch_expired` | `expired_epoch_projection` | `policy_epoch_unstated` | **yes** |
| `per_root_trust_mixed` | `mixed_root_projection` | `mixed_root_shown_as_uniform_trust` | **yes** |
| `capability_narrowed` | `narrowed_capability_projection` | `narrowed_capability_unstated` | no (honest restricted mode) |
| `checkpoint_missing` | `missing_checkpoint_projection` | `checkpoint_absence_hidden` | no (honest disclosed absence) |
| `reversal_evidence_partial` | `unproven_reversal_projection` | `reversal_limit_hidden` | **yes** |

A **stale trust lineage, an expired policy epoch, a mixed-root trust, or a partial reversal** are
weakened evidence: they can never keep a `full_trust_reviewed_result` claim — a stale trust lineage
never masquerades as full, blanket trust, and a partial reversal never reads as a generic success. A
**narrowed capability** and a **missing checkpoint** are honest restricted-mode / disclosed-absence
operations, not truth overstatements, so they are deliberately excluded from that flag while still
narrowing the claim.

## Qualification status

- **`parity` (green):** full parity across all five reach axes and export, with no narrowing.
- **`narrowed_disclosed` (yellow):** reduced but fully disclosed, reachable, and honestly
  auto-narrowed. A dense hierarchy-heavy grid whose screen-reader traversal narrows to a disclosed
  linear walk is legitimately yellow.
- **`stranded` (red):** strands assistive tech, needs a raw payload, over-claims full trust, or drops
  state silently. No red row may ship.

The seeded packet certifies **8 rows across all 8 families**: one green (the fully attributed
repair-result receipt row) and seven yellow (six auto-narrowed claims plus the disclosed-reduced
trust-fact grid), zero red. Every dimension, every condition state, every claim tier, and all nine
consumer surfaces are exercised end-to-end.

## Guardrails

- Trust surfaces never imply blanket approval across roots, profiles, or routes — mixed-root trust
  always narrows to a `mixed_root_projection`.
- Repair previews never hide checkpoint absence or reversal limits — a missing checkpoint narrows to a
  `missing_checkpoint_projection` that discloses the reversal limits before apply.
- Distinct exact / compensate / regenerate / manual / audit-only outcomes are never collapsed into a
  generic success — a partial reversal narrows to an `unproven_reversal_projection`.
- The packet is metadata-only: raw grant tokens, policy bodies, checkpoint payloads, credentials,
  secrets, and endpoint refs never cross the boundary.
