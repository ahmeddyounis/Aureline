# Notebook and structured preview truth

This note freezes the retained preview-row contract for notebook and
structured-configuration surfaces that are visible before the full product
depth exists. The implementation lives in
[`crates/aureline-shell/src/preview_truth/`](../../../crates/aureline-shell/src/preview_truth/mod.rs)
and is replayed by fixtures in
[`fixtures/notebook/m3/trust_repair_roundtrip/`](../../../fixtures/notebook/m3/trust_repair_roundtrip/).

## Required rows

Every retained row must publish one `preview_truth_record` with:

- a claim manifest whose visible label says `Preview` or `Limited` unless the
  row has stable qualification evidence;
- separate document, runtime, and output trust rows, each naming the actions it
  blocks or permits;
- round-trip risk rows that carry warning classes, apply gates, affected
  fields, raw-preservation refs, and compare refs when a write is lossy;
- repair lineage rows binding rerun, repair, trust raise, output clear, and
  structured apply flows to checkpoint refs and mutation-journal refs;
- safe-output rows so active, oversized, widget, or unknown-renderer content
  defaults to safe preview, explicit deeper render, explicit rerun, or policy
  block.

Structured-config rows additionally publish `Authored source`, `Effective
projection`, and `Live observed state` entries. Only authored source may be
write-eligible. Effective and live rows must carry resolution time, target
boundary, freshness, redaction/deferred-value posture, and inspect-only or
read-only write posture.

## Support export

`PreviewTruthRecord::support_export()` emits a
`preview_truth_support_export_record` that preserves:

- document/runtime/output trust tokens;
- round-trip risk and apply-gate tokens;
- repair lineage refs including checkpoint and mutation-journal refs;
- safe-output dispositions;
- validation violation tokens for failure drills.

The export contains opaque refs and reviewer-facing labels only. It does not
include raw notebook JSON, raw config bodies, output payloads, hostnames,
absolute paths, or secrets.

## Guardrails

Preview rows must not imply full notebook, kernel, widget, or general
structured-editor depth. If a retained row cannot prove round-trip safety, it
must fall back to source-only editing, compare-first review, safe preview, or a
typed refusal instead of guessing.

Verification:

```sh
cargo test -p aureline-shell preview_truth
cargo test -p aureline-shell --test wedge_inspector_overlay
```
