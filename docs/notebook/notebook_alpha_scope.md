# Notebook Alpha Scope

This document describes the bounded notebook alpha lane implemented in
`crates/aureline-shell/src/notebook_alpha`.

## Supported Lane

The lane publishes one inspectable `notebook_alpha_lane_record` for one
notebook document. It records:

- a canonical `.ipynb` document object with VFS canonical-object and route refs;
- stable cell objects with cell ids that survive reorder, review, and export;
- a kernel-session object with an execution-context ref;
- output records with distinct live, captured, stale, imported, replayed,
  orphaned, and widget-blocked postures;
- a review-session object and cell-aware diff posture;
- repair-preview records with repair-class, apply-mode, reversal, consequence,
  and checkpoint fields;
- export scopes that keep canonical `.ipynb`, rendered report, and raw
  artifact payloads separate;
- paired export and reproducibility objects when available.

## Trust And Repair Vocabulary

The lane reuses the existing notebook trust-badge vocabulary for workspace
trust, notebook trust rung, kernel availability, output trust, widget trust,
cell/output content class, representation state, and safe escape hatches.

Rendered notebook output uses the safe-preview `TrustClass`,
`RepresentationClass`, and `BodyPosture` vocabulary so raw, rendered,
sanitized, and metadata-only transfers stay labelled before copy or export.

Repair rows use the shared repair-transaction vocabulary:
`repair_class_family`, `apply_mode_class`, and
`transaction_reversal_class`. A repair that can rewrite durable notebook state
must cite a preview and, when required, a checkpoint. The lane does not apply
repairs automatically.

## Downgrade Behavior

- No kernel: the document remains open, editable, searchable, diffable, and
  exportable. Execution controls are unavailable until a kernel/session is
  admitted.
- Trust downgrade: live outputs become captured or stale evidence, widget live
  binding stays blocked, and no output is silently refreshed.
- Round-trip uncertainty: raw JSON fallback remains available and rewrites are
  refused until metadata, attachments, and output consequences are reviewable.
- Missing widget/runtime support: widgets render as static or metadata-only
  evidence with explicit safe next actions.

## Proof Fixtures

Fixtures live under `fixtures/notebook/notebook_trust_diff_alpha/`.

- `protected_trust_diff_repair_export.yaml` proves the clean lane.
- `failure_stale_imported_output_claimed_live.yaml` proves stale/imported
  output cannot masquerade as fresh live execution.

Run:

```sh
cargo test -p aureline-shell notebook_alpha
```
