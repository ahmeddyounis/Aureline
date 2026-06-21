# Execution-Evidence Causality Tool

`tools/release/execution_evidence_causality.py` freezes and certifies the M5
Problems / output-channel / execution-evidence causality matrix. Problems rows,
output channels, execution-evidence projections, and evidence bundles are one
causal system: a user must be able to answer **what ran, what produced this
message, how certain the parser was, what run/provider/channel it came from, and
how to reopen the originating evidence** without stitching raw logs together by
hand.

The canonical truth is the causality matrix support export
(`artifacts/tooling/m5-execution-evidence/support_export.json`). This tool ingests
it, **independently re-derives** each lane's effective causal claim, and projects a
single governed claim matrix that Problems, output, diagnostics, AI evidence,
support export, review, CLI/headless, and docs surfaces ingest instead of inventing
a parallel causal model.

See [`docs/tooling/m5-execution-evidence.md`](../../../docs/tooling/m5-execution-evidence.md)
for the frozen taxonomies, identity model, and the full narrowing-rule table.

## Causal-claim ladder

A lane's effective claim is the strongest of:

- `causal_chain_certified` — full first-party causal chain preserved, fresh,
  confidence honest, reopenable.
- `causal_chain_narrowed` — a first-party lane held below certified by a stale,
  missing, or labelled gap; lineage stays reopenable.
- `evidence_read_only_overlay` — remote/pipeline/imported evidence; attributable
  and reopenable but never claims live local authority.
- `causal_chain_unreconstructable` — lineage/channel/reopen broken or evidence
  missing; the lane surfaces a raw-output backlink or keyboard fallback instead of
  a clean-but-false causal claim.
- `causal_evidence_labs_not_claimed` — Labs/unadvertised; no public claim.

A claimed lane auto-narrows strictly below its headline claim when any
causal-chain axis fails or its verification evidence is stale or missing. Floor
reasons (`run_channel_lineage_flattened`, `channel_identity_flattened`,
`reopen_target_lost`, `raw_output_backlink_missing`, `export_packet_incomplete`,
`evidence_missing`, `imported_overlay_claims_live`) drop a lane to
`causal_chain_unreconstructable`; the rest hold a first-party lane at
`causal_chain_narrowed`. An overlay is already the minimal honest claim, so any
unresolved reason on an overlay floors it. Once the matrix verification-freshness
window elapses, any lane resting on a current proof narrows — so surfaces cannot
imply a broad causal claim without current evidence. Labs lanes never widen.

## Usage

```sh
# Regenerate the surface-facing artifacts (generated, never hand-edited):
python3 tools/release/execution_evidence_causality.py emit-matrix
python3 tools/release/execution_evidence_causality.py emit-report

# Gate: re-derive from the source matrix and fail on any overclaim/mismatch:
python3 tools/release/execution_evidence_causality.py validate

# Exercise the narrowing engine over the fixture corpus:
python3 tools/release/execution_evidence_causality.py corpus

# End-to-end: schema check, emit round-trips clean, checked-in artifacts fresh, corpus passes:
python3 tools/release/execution_evidence_causality.py self-test
```

`validate` fails (non-zero) when a surface projection renders wider than the
lane's effective claim, an imported/overlay lane drops its read-only marker, a
narrowed entry lacks a precise label or trigger, the recorded claim/reasons drift
from the re-derived truth, or the summary/publication decision does not match the
recomputed state.

## Outputs

- `artifacts/tooling/m5-execution-evidence/matrix.json` — the governed causal-claim matrix.
- `artifacts/tooling/m5-execution-evidence/report.md` — the certification report.
