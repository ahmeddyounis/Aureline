# Docs Validation Report

This document is the contract for the M5 docs validation report — the surface
that turns documented examples and links into typed, reviewable validation rows
instead of decorative pass/fail badges. A row is never an unlabeled green check:
it names a concrete **subject**, exactly one **validation mode**, an **outcome**
that must agree with that mode, an explicit **last-checked time** and
**environment/version scope**, the **validator** that produced it, a
**source/evidence trace**, the full **action set**, and a durable
**suppression**, so a harmless rendered preview is never confused with a
syntax-checked or an actually executed example, and a broken-link or
stale-example finding is an actionable review item rather than a badge.

Each row carries:

- a **subject** — the subject kind, the concrete doc ref, an optional
  snippet/link anchor, a human-readable display path, and a label;
- a **mode** — one validation mode (`rendered`, `syntax_checked`,
  `executed_local`, `executed_remote`, `skipped`, `stale`, `unsupported`,
  `broken_link`);
- an **outcome** — the result (`executed_pass`, `executed_fail`,
  `passed_with_warnings`, `rendered_preview_only`, `syntax_valid`,
  `syntax_invalid`, `link_broken`, `not_run`), which must agree with the mode;
- a **last-checked time** and a **scope** — the environment label, toolchain ref,
  target version ref, and version match the row was validated under;
- a **producer** — the validator (`rendered_preview_engine`, `syntax_checker`,
  `local_example_harness`, `remote_example_runner`, `link_checker`,
  `manual_reviewer`) plus its execution-context ref;
- a **chip set** — the freshness / version-match / locality labels a consumer
  projects verbatim;
- an **evidence provenance** disclosure (`first_party_verified`,
  `local_only_unverified`, `imported`, `mirrored`, `stale`, `derived_heuristic`)
  so a cached or imported result is never mistaken for authoritative live truth;
- a **source trace** — the failing source, the link target, or the drifted source
  an actionable finding points at;
- an **action set** — Open snippet, Open failing source, Compare current source,
  Suppress, and Rerun, plus a flag that the actions preserve the producing
  validator/context;
- a **suppression** — the durable history-backed state (`active`, `suppressed`,
  `reopened`), which stays attributable, previewable, and reopenable once a
  finding is suppressed.

An export preserves the mode / outcome / last-checked / environment-version-scope
/ freshness / provenance / producer / action-parity / suppression truth that
review, support, AI evidence, release, and diagnostics surfaces ingest rather than
cloning status text. The docs validation report, docs authoring surface, docs
review panel, docs browser shell, release center, AI-context inspector,
CLI/headless output, support exports, diagnostics, and Help/About all consume the
checked-in packet.

- Record kind: `docs_validation_report`
- Schema: [`schemas/docs/docs-validation-report.schema.json`](../../schemas/docs/docs-validation-report.schema.json)
- Canonical support export: [`artifacts/docs/m5/docs-validation-proof/support_export.json`](../../artifacts/docs/m5/docs-validation-proof/support_export.json)
- Summary artifact: [`artifacts/docs/m5/docs-validation-proof.md`](../../artifacts/docs/m5/docs-validation-proof.md)
- Fixtures: [`fixtures/docs/m5/example-link-validation/`](../../fixtures/docs/m5/example-link-validation/)
- Producer: `aureline_docs::current_stable_docs_validation_report_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_validation_report`

## Subjects and mode coverage

`rows` is the set of validation rows for one report run. Every row points at a
concrete `subject`:

| Field | Meaning |
| --- | --- |
| `subject_kind` | `code_example`, `shell_example`, `config_example`, `link`, `anchor_link` |
| `doc_ref` | repo-relative doc file the example/link lives in (never a raw body) |
| `snippet_anchor` | the snippet/link anchor within the doc, when applicable |
| `display_path` | the human-readable subject path the report renders |
| `label` | the human-readable subject label |

A row that does not name a concrete doc, display path, and label (or carries an
empty snippet anchor when one is recorded) is `subject_identity_missing` and
blocks promotion.

A report must cover the `code_example` and `link` subject kinds
(`required_subject_kind_missing`) and must demonstrate the `rendered`,
`executed_local`, `broken_link`, and `stale` modes
(`required_mode_coverage_missing`), so the report stays the qualified
cross-surface boundary — a harmless preview, a real execution, a broken-link
finding, and a stale-example finding — rather than a slice that overstates
coverage.

## Validation modes and the rendered-vs-executed distinction

The validation `mode` is the central distinction the report exists to keep
honest. A row's `outcome` must agree with its mode:

| Mode | Permitted outcomes |
| --- | --- |
| `rendered` | `rendered_preview_only` |
| `syntax_checked` | `syntax_valid`, `syntax_invalid` |
| `executed_local`, `executed_remote` | `executed_pass`, `executed_fail`, `passed_with_warnings` |
| `broken_link` | `link_broken` |
| `skipped`, `stale`, `unsupported` | `not_run` |

A non-executed row (rendered, syntax-checked, skipped, stale, unsupported, or
broken-link) that claims an `executed_pass` or `executed_fail` result is
`execution_claim_without_run` and blocks promotion — a harmless rendered preview
may never be presented as an actually executed example. Any other mode/outcome
disagreement is `mode_outcome_inconsistent`.

## Last-checked time and environment/version scope

Every row carries an explicit `last_checked_at` (`last_checked_missing` if empty)
and a `scope` with a non-empty `environment_label`, `toolchain_ref`, and
`target_version_ref` plus a `version_match` (`environment_scope_missing` if
incomplete). A result is never presented without saying when and in what
environment and against what version it was checked, so the same row stays
honest through review, support export, and release-facing docs lanes.

## Producer attribution

Every row names the `produced_by` validator and its `execution_context_ref`
(`producer_context_missing` if empty). The validator must be one the mode permits
(`producer_validator_mode_mismatch`) — a `rendered` row is produced by the
`rendered_preview_engine`, an `executed_local` row by the `local_example_harness`,
a `broken_link` row by the `link_checker`, and so on. The action set carries a
`preserves_producer` flag (`producer_not_preserved` if false) so suppress and
rerun keep the finding attributable to the validator/context that produced it.

## Provenance and cached-truth visibility

The chip set is the labels a consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `locality` | `local`, `imported_pack`, `mirrored_pack`, `managed` |

`provenance` keeps imported, local-only, mirrored, stale, and derived evidence
visible. Only `first_party_verified` evidence is authoritative; any other
provenance:

- may not be presented as an `executed_pass` at `authoritative_live` freshness
  (`result_truth_collapsed`); and
- must stay cited (`row_not_cited`).

A non-current `version_match` presented as an `executed_pass` at
`authoritative_live` freshness is `version_truth_collapsed`. Every row also
carries a `provenance_disclosure_note` (`provenance_disclosure_missing` if empty).

## Actionable, traced findings

A failing (`executed_fail`, `syntax_invalid`, `link_broken`) row, or a row whose
mode is `broken_link`, `stale`, or `unsupported`, must carry a non-empty
`source_trace_ref` (`finding_not_traced` otherwise) — broken-link and
stale-example findings are actionable review items with a source/evidence trace,
not decorative badges.

Each row carries an `actions` block. Open snippet, Open failing source, Compare
current source, Suppress, and Rerun are all available; a row that drops any of
them is `action_parity_incomplete`. The actions preserve the validator/execution
context that produced the finding, so suppressing or rerunning a finding keeps it
attributable.

## Durable, attributable suppression

Each row carries a `suppression` block — its history-backed `state`. An `active`
(or `reopened`) row is surfaced in review. A `suppressed` row must carry an
`attributed_to_ref` and a durable `history_ref` (`suppression_not_attributable`)
and stay `previewable` and `reopenable` (`suppression_not_reopenable`) —
suppressing a finding is always attributable, previewable, and reopenable from
durable history.

## Export and consumer projections

The `export` mirrors every row into a row preserving subject kind, mode, outcome,
last-checked time, environment/version scope, freshness, provenance, the
producing validator, suppression state, action parity, and citation state. A row
that drops a preservation flag (`export_drops_preservation`), disagrees with its
row (`export_*_mismatch`), references an unknown row (`export_row_orphan`), or
leaves a row uncovered (`export_coverage_missing`) blocks promotion — so the
environment/version scope and imported/cached state stay visible through export
and support flows.

`consumer_projections` records how each surface projects the report. The
`docs_validation_report`, `docs_review_panel`, `release_center`, and
`support_export` surfaces are required; a projection that drops a preservation
flag is `consumer_projection_drift` and one that references the wrong packet id is
`consumer_projection_packet_id_mismatch`.

## Degradations and promotion

`report_degradations` records packet-level degradations (mirror offline, example
harness unavailable, remote runner unavailable, link checker offline, render
engine degraded, scope narrowed, report narrowed, quarantined source) with a
severity. The promotion state is computed from the validation findings and the
degradation severities:

- `stable` — no findings and no narrowing/blocking degradations;
- `narrowed_below_stable` — a narrowing degradation, but the rows stay visible and
  attributable;
- `blocks_stable` — any blocking validation finding or blocking degradation.

Every validation finding blocks the Stable claim; narrowing comes only from
data-carried degradation severities so a degraded-but-honest report narrows rather
than hides its rows.

## Boundary

The packet is an inspectable, serde-serializable truth packet. Raw document
bodies, raw source files, raw URLs, rendered HTML, execution logs, raw provider
payloads, and credentials never cross the boundary; a packet that smuggles
forbidden material is `raw_boundary_material_present`.
