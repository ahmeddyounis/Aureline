# Notebook and structured preview truth fixtures

These fixtures exercise retained preview rows that must disclose trust layers,
round-trip risk, and repair lineage before runtime-heavy or structured writes.

- `notebook_mixed_trust_preview.yaml` covers document/runtime/output trust,
  captured outputs, safe preview defaults, rerun lineage, and output-clear
  lineage.
- `structured_config_roundtrip_risk_preview.yaml` covers authored, effective,
  and live structured-config projections with unknown-field, ordering, and
  comment-loss warnings.
- `failure_preview_overclaim_missing_lineage.yaml` proves a preview-only row is
  rejected when it looks stable, permits lossy apply too freely, and omits
  checkpoint or mutation-journal lineage.
