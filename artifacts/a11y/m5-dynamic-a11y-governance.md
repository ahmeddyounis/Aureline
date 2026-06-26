# M5 Accessibility-Bridge, Live-Announcement, Focus-Return, and Non-Visual Dynamic-Surface Matrix

- Packet: `m5-dynamic-surface-a11y-matrix:stable:0001`
- Label: `M5 Accessibility-Bridge, Live-Announcement, Focus-Return, and Non-Visual Dynamic-Surface Matrix`
- Objects: 6 (5 stable)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Objects

- **accessibility_surface_descriptor**: `stable`
  - Owner: Accessibility owner
  - Scope: Semantic-structure descriptor for one custom-rendered dynamic surface; exposes role, name, value, and state from the renderer's semantic model so the surface is never visual-only, and discloses its non-visual fidelity and bridge state instead of implying full parity
  - Vocabularies: semantic_role_class, non_visual_fidelity, bridge_state
  - Rollback: semantic_structure_preserved
- **screen_reader_label_model**: `stable`
  - Owner: Accessibility owner
  - Scope: Screen-reader name / role / value / state label model for a surface; labels resolve from controlled message-id sources, never drift from the semantic role, and carry a durable fallback so a transient live region is never the only carrier of meaning
  - Vocabularies: semantic_role_class, fallback_durability, bridge_state
  - Rollback: semantic_structure_preserved
- **live_announcement_class**: `stable`
  - Owner: Accessibility owner
  - Scope: Live-announcement class governing politeness, coalescing, and durable fallback; assertive is reserved for safety-critical state, polite is queued, bursts coalesce instead of spamming, and every announced state change carries a durable fallback so meaning survives a missed utterance
  - Vocabularies: announcement_politeness, coalescing_strategy, fallback_durability
  - Rollback: announcement_coalesced_not_spammed
- **focus_return_contract**: `stable`
  - Owner: Accessibility owner
  - Scope: Focus-return contract for asynchronous updates and overlay teardown; focus returns to a real owner — exact, nearest safe ancestor, current batch/detail owner, or an announced placeholder — and never teleports to an unrelated surface or vanishes, with a durable re-entry fallback when the prior owner is destroyed
  - Vocabularies: focus_return_disposition, fallback_durability
  - Rollback: focus_anchor_preserved
- **dense_surface_non_visual_summary**: `stable`
  - Owner: Accessibility owner
  - Scope: Dense-surface non-visual summary for lists, trees, grids, and logs; exposes position, selection scope, hidden-selected and blocked counts, and virtualization truth as a coalesced non-visual summary so a screen-reader user knows the real scope before acting, never just the visible rows
  - Vocabularies: non_visual_fidelity, coalescing_strategy, semantic_role_class
  - Rollback: semantic_structure_preserved
- **bridge_diagnostics_packet**: `beta`
  - Owner: Accessibility platform owner
  - Scope: OS accessibility-bridge diagnostics packet that names the active platform bridge and its connection state; when the bridge is partial, stale, or unavailable the packet discloses the degradation and the affected surfaces auto-narrow rather than claiming silent screen-reader completeness
  - Vocabularies: bridge_state, non_visual_fidelity
  - Rollback: bridge_degradation_disclosed
