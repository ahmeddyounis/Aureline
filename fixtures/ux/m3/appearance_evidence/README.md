# Appearance parity evidence fixtures

This directory holds the consolidated beta appearance-parity evidence
packet and its metadata-safe support export. Both files are generated
from upstream truth by the gate in
[`tools/ci/m3/appearance_evidence/`](../../../../tools/ci/m3/appearance_evidence/__main__.py)
and cannot drift from it.

The packet joins three already-governed upstream sources into one
current, build-linked packet so shiproom, support, docs, marketplace
review, and design QA can answer appearance questions from the packet
alone. See
[`docs/ux/m3/appearance_evidence_consumption_guide.md`](../../../../docs/ux/m3/appearance_evidence_consumption_guide.md)
for how each consumer reads it.

All records carry the shared contract ref
`ux:appearance_parity_evidence:v1`.

## Index

| Fixture | Coverage |
| --- | --- |
| `appearance_evidence_packet.json` | Full packet: source-packet freshness, first-party and contributed appearance rows, extension inheritance-gap rows, high-contrast signoff, live-update audit, importer coverage, consumer-surface downgrade registry, and summary. |
| `support_export.json` | Metadata-safe support projection (`raw_private_material_excluded: true`, `manual_private_path_lookup_required: false`) with appearance rows, gap rows, signoff/audit/importer summaries, and the consumer-surface registry. |

## Upstream truth (read-only inputs)

- `fixtures/extensions/m3/appearance_inheritance/conformance_packet.json`
- `artifacts/ux/m3/component_state_screenshot_diff/packet.json`
- `fixtures/ux/m3/state_semantics/token_conformance_report.json`
- `fixtures/ux/m3/theme_import_and_live_change/appearance_session_beta_contract.json`
- `artifacts/release/m3/artifact_graph.json` (build identity)

## Generated reports

- `artifacts/ux/m3/appearance_parity_evidence_packet.md`
- `artifacts/ux/m3/high_contrast_and_live_change_audit.md`
- `artifacts/extensions/m3/extension_inheritance_gap_packet.md`
- `artifacts/ux/m3/captures/appearance_evidence_validation_capture.json`

## Rules

- The packet, support export, and reports are generated from upstream
  truth; hand-edits are overwritten and fail `--check`.
- Every first-party row binds to a passing structured state report
  (token conformance + screenshot-state diff + semantic stability) and a
  candidate build identity.
- Every contributed row binds to the extension conformance row, its
  per-axis support, and a package version.
- A row resting on stale evidence must downgrade out of `current_packet`.
- A `needs_review` extension row never implies full inheritance on any
  consuming surface.
- The high-contrast signoff and live-update audit must be attributable to
  the candidate exact build identity.

## Regenerate

```sh
python3 ci/check_m3_appearance_evidence.py --repo-root .
```

## Verify

```sh
python3 ci/check_m3_appearance_evidence.py --repo-root . --check
```
