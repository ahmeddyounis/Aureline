# M5 Experiment Component Surface Certification

- Packet: `m5-experiment-component-certification:stable:0001`
- As of: `2026-07-09T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-experiment-component-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Lineage preserved on every surface: true
- No surface implies apples-to-apples without parity: true
- No surface exposes raw payload by default: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:notebook-experiment-run** — surface=notebook_experiment_run claimed=exact_comparable_result certified=exact_comparable_result status=green narrowed_axes=0 lineage_preserved=true implies_parity=false raw_payload=false
- **cert:experiment-dashboard** — surface=experiment_dashboard claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0 lineage_preserved=true implies_parity=false raw_payload=false
- **cert:review-evidence** — surface=review_evidence claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0 lineage_preserved=true implies_parity=false raw_payload=false
- **cert:support-export** — surface=support_export claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0 lineage_preserved=true implies_parity=false raw_payload=false
- **cert:run-comparison** — surface=run_comparison claimed=exact_comparable_result certified=incomparable_runs_projection status=yellow narrowed_axes=1 lineage_preserved=true implies_parity=false raw_payload=false
- **cert:data-catalog** — surface=data_catalog claimed=exact_comparable_result certified=blocked_preview_projection status=yellow narrowed_axes=1 lineage_preserved=true implies_parity=false raw_payload=false
- **cert:artifact-lineage** — surface=artifact_lineage claimed=exact_comparable_result certified=stale_lineage_projection status=yellow narrowed_axes=1 lineage_preserved=true implies_parity=false raw_payload=false
- **cert:cli-headless** — surface=cli_headless claimed=exact_comparable_result certified=partial_fingerprint_projection status=yellow narrowed_axes=1 lineage_preserved=true implies_parity=false raw_payload=false
