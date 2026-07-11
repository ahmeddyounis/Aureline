# M5 Notebook-Component Accessibility & Auto-Narrowing (M05-1089)

This lane is the large-output-virtualization / collapsed-output-summary / keyboard / screen-reader /
high-zoom / reduced-motion / CLI / export parity and honest auto-narrowing capstone over the frozen
M5 notebook-kernel-output component matrix
(`freeze_the_m5_notebook_document_header_...`). Where the freeze matrix defines the reusable notebook
document header, kernel state strip, kernel picker row, kernel origin pill, output trust banner,
output provenance chip group, restart consequence card, and kernel recovery card primitives — and
the 1085–1088 implementation lanes resolve their per-surface truth — this lane certifies, per
component family, that notebook claims stay **keyboard-complete, assistive-tech-reachable, high-zoom
/ reduced-motion-safe, CLI/export-safe, virtualization-honest, and self-narrowing**.

## What it guarantees

- **Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach.** Every family exposes a
  keyboard-complete, screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and
  CLI/headless-reachable path into the same document identity, selected kernel origin / class,
  execution / connection state, output trust class, output freshness, provenance lineage, and
  restart / recovery consequence the rich component shows — never a hover-only chip. The
  hierarchy-heavy output provenance chip group (nested source / transform / derived-output lineage
  chips) additionally binds its group to a flat list / textual path.
- **Large-output virtualization & collapsed-output summaries.** When a dense output is virtualized or
  collapsed to a summary, the truncated view keeps the run identity, output trust class, and
  stale-versus-live truth attached — a virtualized or collapsed output is never rendered as an
  anonymous, trust-less blob (AC1). A truncated output is an honest **disclosed reduction** (yellow),
  never a silent drop (red).
- **Export parity.** The support / release / CLI export reconstructs each component's meaning from
  typed tokens and opaque refs **without a raw payload**, preserving the document identity, kernel
  origin / class, output trust / freshness, provenance, and restart / recovery posture shown
  in-product — so support, docs, and release proof can reconstruct exactly what the user was actually
  shown without leaking a raw kernel session or output payload (AC2).
- **Honest auto-narrowing.** When kernel parity is partial, a debugger is unsupported, an output's
  trust evidence is stale, a kernel origin is degraded, an environment's provenance is severed, or no
  kernel is available, the component's claim auto-narrows from `live_trusted_result` /
  `reviewable_result` to a `partial_kernel_parity_projection` / `debugger_unsupported_projection` /
  `degraded_origin_projection` / `stale_output_projection` / `unprovenanced_environment_projection` /
  `no_kernel_projection`, discloses the narrowing with a precise trigger and binding dimension, and
  preserves the canonical document identity / kernel origin / output provenance. A partial-parity /
  degraded-origin / stale-output / severed-provenance state can never keep a live-trusted claim — a
  stale output never masquerades as live truth (AC3). An unsupported debugger and a kernel-free strip
  are honest capability / offline states, so they narrow visibly but are not treated as truth
  overstatements.
- **Cross-surface disclosure.** The same narrowed state surfaces in the notebook UI, kernel-manager
  UI, output-viewer UI, debugger UI, AI-context UI, review UI, CLI surface, support export, and
  product UI so product, docs, and release publication stay aligned on downgrade behavior.

## Claim ceiling

Each condition state imposes a ceiling on how strong a claim a surface may present:

| Condition state (dimension weakened) | Permitted ceiling | Frozen trigger | Never live-trusted |
| --- | --- | --- | --- |
| `live_trusted` | `live_trusted_result` | — | — |
| `kernel_parity_partial` | `partial_kernel_parity_projection` | `kernel_class_collapsed` | yes |
| `debugger_unsupported` | `debugger_unsupported_projection` | `proof_stale` | no (capability) |
| `kernel_origin_degraded` | `degraded_origin_projection` | `kernel_origin_unstated` | yes |
| `output_trust_stale` | `stale_output_projection` | `stale_output_shown_as_live` | yes |
| `environment_provenance_severed` | `unprovenanced_environment_projection` | `provenance_severed` | yes |
| `kernel_unavailable` | `no_kernel_projection` | `reconnect_shown_as_fresh` | no (offline) |

The effective claim never exceeds the permitted ceiling; when a dimension narrows below the family's
full claim, an honest narrow block names the ceiling-imposing dimension with its frozen trigger and
preserves canonical identity and notebook truth. A component with every dimension intact carries no
spurious narrow.

## Certified rows

The seed certifies all eight frozen families (one green baseline plus seven honest disclosed
reductions — six auto-narrowed claims and one collapsed-output provenance chip group):

- `notebook-document-header-live` — live trusted result, full output (green).
- `output-provenance-chip-group-complete` — reviewable result, hierarchy-heavy, collapsed-output
  summary that stays attributable (yellow).
- `kernel-state-strip-no-kernel` — narrows to a no-kernel projection (yellow).
- `kernel-picker-row-partial-parity` — narrows to a partial-kernel-parity projection (yellow).
- `kernel-origin-pill-degraded` — narrows to a degraded-origin projection (yellow).
- `output-trust-banner-stale` — narrows to a stale-output projection, virtualized dense output that
  stays attributable (yellow).
- `restart-consequence-card-debugger-unsupported` — narrows to a debugger-unsupported projection
  (yellow).
- `kernel-recovery-card-unprovenanced-environment` — narrows to an unprovenanced-environment
  projection, never implies a hidden rerun (yellow).

## Artifacts

- Schema: `schemas/ui/m5-notebook-kernel-output-component-accessibility-parity.schema.json`
- Support export (canonical):
  `artifacts/release/m5-notebook-kernel-output-component-accessibility-parity/support_export.json`
- Matrix CSV:
  `artifacts/release/m5-notebook-kernel-output-component-accessibility-parity/matrix.csv`
- Report:
  `artifacts/release/m5-notebook-kernel-output-component-accessibility-parity.md`
- Fixtures:
  `fixtures/ui/m5-notebook-kernel-output-component-accessibility-parity/`

The packet is metadata-only: raw kernel sessions, output payloads, credentials, tokens, request
bodies, and endpoint secrets never cross this boundary, and a raw payload is never the only export.

Regenerate the checked-in artifacts and fixtures with
`GEN_NOTEBOOK_KERNEL_OUTPUT_A11Y_ARTIFACTS=1 cargo test -p aureline-notebook generate_artifacts`.
