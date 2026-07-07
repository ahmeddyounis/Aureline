# M5 Support-Intake / Escalation Component Surface Certification

- Packet: `m5-support-intake-escalation-component-certification:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-support-intake-escalation-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Lineage preserved on every surface: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:doctor-results** — surface=doctor_results claimed=ready_to_escalate certified=ready_to_escalate status=green narrowed_axes=0 lineage_preserved=true
- **cert:support-center** — surface=support_center claimed=ready_to_escalate certified=ready_to_escalate status=green narrowed_axes=0 lineage_preserved=true
- **cert:support-export** — surface=support_export claimed=reviewable_case certified=reviewable_case status=green narrowed_axes=0 lineage_preserved=true
- **cert:safe-mode** — surface=safe_mode claimed=reviewable_case certified=reviewable_case status=green narrowed_axes=0 lineage_preserved=true
- **cert:extension-bisect** — surface=extension_bisect claimed=ready_to_escalate certified=unclassified_scenario status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:docs-help** — surface=docs_help claimed=reviewable_case certified=evidence_incomplete_case status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:support-bundle-preview** — surface=support_bundle_preview claimed=ready_to_escalate certified=local_only_diagnosis status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:cli-headless** — surface=cli_headless claimed=ready_to_escalate certified=policy_blocked_repair status=yellow narrowed_axes=1 lineage_preserved=true
