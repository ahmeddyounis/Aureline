# Notebook trust, diff, repair, and export alpha fixtures

These fixtures exercise the bounded notebook alpha lane implemented by
`crates/aureline-shell/src/notebook_alpha`.

The lane is intentionally narrow: one notebook document, stable cell and
output ids, one kernel-session ref, one review-session ref, one diff posture,
one repair preview path, and explicit export scopes for canonical `.ipynb`,
rendered report, and raw artifact payloads. The fixtures do not include raw
cell bodies, raw output payloads, kernel protocol frames, raw widget state, or
credentials.

Cases:

| File | Purpose |
|---|---|
| `protected_trust_diff_repair_export.yaml` | Clean protected lane proving distinct trust axes, stale/imported output honesty, unknown metadata and attachment preservation, repair consequence disclosure, and separated export scopes. |
| `failure_stale_imported_output_claimed_live.yaml` | Failure drill proving stale or imported output cannot claim live output trust, live viewer state, or live lineage without an honesty marker. |
