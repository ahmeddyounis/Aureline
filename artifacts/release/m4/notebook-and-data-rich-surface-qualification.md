# Notebook and Data-Rich Surface Qualification

This artifact is the canonical release packet for promoted notebook and data-rich
surfaces. The machine-readable source is
`artifacts/release/m4/notebook-and-data-rich-surface-qualification.json`.

## Family Matrix

| Surface | Rendered label | Trust axes | Replay/export | Review posture |
| --- | --- | --- | --- | --- |
| Notebook document/runtime truth | Stable | document mixed, kernel trusted, output mixed | rerun after review, snapshot, redacted support export | cell-aware diff, metadata/output-aware review, snapshot/golden |
| Notebook review/export | Stable | document trusted, output captured | clean-output preview, paired export, snapshot, redacted support export | cell-aware diff with raw JSON fallback |
| Variable explorer snapshot | Stable | document trusted, kernel mixed, output stale | snapshot, redacted support export | snapshot/golden |
| Data table safe preview | Preview | imported document, captured output | snapshot, scoped handoff, redacted support export | snapshot/golden |
| Database/result grid preview | Preview | mixed document, captured output | snapshot, scoped handoff, redacted support export | snapshot/golden |
| Chart summary preview | Preview | trusted document, captured output | snapshot, scoped handoff, redacted support export | snapshot/golden |
| Experiment handoff preview | Preview | mixed document, mixed runtime, captured output | snapshot, scoped handoff, redacted support export | snapshot/golden |
| API/database response viewer preview | Preview | imported document, imported output | snapshot, scoped handoff, redacted support export | snapshot/golden |

## Notebook Document Runtime Truth

The Stable notebook row covers only the visible document/runtime/output truth
surface: notebook header, kernel bar, cells, output panes, variable explorer
summary, debugger-affordance labels, and restart/reconnect consequence review.
It carries a current proof packet and owner sign-off. No-kernel, disconnected
kernel, stale-output, imported-output, captured-output, mixed-trust, and safe
preview states remain visible before any live or rerunnable claim.

## Notebook Review Export

Notebook review defaults to cell-aware diff when the notebook parses. It falls
back to metadata/output-aware or raw JSON modes only with explicit parse-failure,
unsupported-version, or extension-payload-mismatch reasons. Paired export
descriptors, clean-output previews, and output trust markers identify canonical
source, derived content, captured evidence, and work that would require a fresh
rerun or runtime reattachment.

## Variable Explorer

The variable explorer is qualified as a snapshot surface. It preserves kernel
identity, target identity, freshness, truncation, and redaction truth, and it
must not imply live state after disconnect, restart, or policy block.

## Preview Rows

Data tables, result grids, chart summaries, experiment handoff cards, and
API/database response viewers remain Preview unless they carry their own current
family packet. The packet deliberately keeps database/result-grid and
experiment-platform depth narrower than notebook runtime truth.

## Support Export Projection

Support/export packets cite row ids, rendered labels, proof packet refs,
snapshot/golden refs, accessibility refs, replay/export posture, destination
class, row/column or artifact scope, freshness, lineage, and redaction mode.
They never silently rerun kernels, requests, or queries.
