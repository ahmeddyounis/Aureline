# M5 Editor-Inline Component Surface Certification

- Packet: `m5-editor-inline-component-surface-certification:stable:0001`
- As of: `2026-07-12T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-editor-inline-proof/support_export.json`
- Profiles: 8 / 8 certified (2 green, 6 yellow, 0 red)
- Families covered: true
- Guardrails held: true
- Auto-narrowed profiles: 6
- Report clean: true

## Profiles

- **cert:live-trusted-inline-surface** — profile=live_trusted_inline_surface claimed=trusted_inline_result certified=trusted_inline_result status=green narrowed_axes=0
- **cert:reviewable-inline-structure** — profile=reviewable_inline_structure claimed=reviewable_inline_result certified=reviewable_inline_result status=green narrowed_axes=0
- **cert:drifted-anchor-surface** — profile=drifted_anchor_surface claimed=reviewable_inline_result certified=anchor_unverified_projection status=yellow narrowed_axes=1
- **cert:stale-severity-decoration** — profile=stale_severity_decoration claimed=reviewable_inline_result certified=severity_unverified_projection status=yellow narrowed_axes=1
- **cert:inferred-fix-chip** — profile=inferred_fix_chip claimed=reviewable_inline_result certified=fix_posture_unverified_projection status=yellow narrowed_axes=1
- **cert:stale-confidence-message** — profile=stale_confidence_message claimed=reviewable_inline_result certified=confidence_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-approval-thread** — profile=unverified_approval_thread claimed=reviewable_inline_result certified=approval_unverified_projection status=yellow narrowed_axes=1
- **cert:partial-evidence-timeline** — profile=partial_evidence_timeline claimed=reviewable_inline_result certified=evidence_lineage_projection status=yellow narrowed_axes=1
