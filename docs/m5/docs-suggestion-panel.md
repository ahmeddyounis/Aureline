# Docs Suggestion Panel

This document is the contract for the M5 docs suggestion panel — the diff-first
surface that proposes prose edits to README, changelog, help, and tutorial docs
and ties every proposal back to the code, schema, or release change that raised
it. A suggestion is never a generic "recommended edit" blob: it names a concrete
**target**, a concrete **trigger source**, a **diff-based proposal**, the full
**action set**, and a durable **disposition**, so docs maintenance flows through
the same review/diff model the rest of Aureline uses rather than a prose-only
side channel.

Each suggestion carries:

- a **target** — the target document kind, the concrete file ref, an optional
  section anchor, and a human-readable display path;
- a **trigger** — one trigger source (`code_diff`, `symbol_rename`,
  `api_contract_change`, `failing_example`, `broken_link`,
  `release_metadata_change`, `manual_authoring`) with a non-empty detail and a
  triggering-evidence ref;
- a **chip set** — the confidence / freshness / version-match / locality labels a
  consumer projects verbatim;
- an **evidence provenance** disclosure (`first_party_verified`,
  `local_only_unverified`, `imported`, `mirrored`, `stale`, `derived_heuristic`)
  so a useful hint is never mistaken for authoritative repo truth;
- a **proposal** — a diff-based edit (hunks, a new section, a link repoint, or an
  example replacement) with a summary and a previewable ref, never a prose-only
  card;
- an **action set** — the Apply posture plus Open evidence, Open source, Dismiss,
  and Save for later parity;
- a **disposition** — the durable history-backed state (`pending`, `applied`,
  `dismissed`, `saved_for_later`, `superseded`), which stays attributable,
  previewable, and reopenable once it is resolved.

An export preserves the target / trigger-source / confidence / freshness /
apply-posture / provenance / action-parity / disposition truth that support, AI
evidence, and diagnostics surfaces ingest rather than cloning status text. The
docs suggestion panel, docs authoring surface, docs review panel, docs browser
shell, release center, AI-context inspector, CLI/headless output, support exports,
diagnostics, and Help/About all consume the checked-in packet.

- Record kind: `docs_suggestion_panel`
- Schema: [`schemas/docs/docs-suggestion-packet.schema.json`](../../schemas/docs/docs-suggestion-packet.schema.json)
- Canonical support export: [`artifacts/docs/m5/docs-suggestion-proof/support_export.json`](../../artifacts/docs/m5/docs-suggestion-proof/support_export.json)
- Summary artifact: [`artifacts/docs/m5/docs-suggestion-proof.md`](../../artifacts/docs/m5/docs-suggestion-proof.md)
- Fixtures: [`fixtures/docs/m5/docs-suggestion-triggers/`](../../fixtures/docs/m5/docs-suggestion-triggers/)
- Producer: `aureline_docs::current_stable_docs_suggestion_panel_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_suggestion_panel`

## Targets and trigger sources

`suggestions` is the set of suggestions for one panel session. Every suggestion
points at a concrete `target`:

| Field | Meaning |
| --- | --- |
| `target_kind` | `readme`, `changelog`, `help`, `tutorial`, `guide`, `api_reference` |
| `file_ref` | repo-relative target file (never a raw body) |
| `section_anchor` | the section the edit is scoped to, when applicable |
| `display_path` | the human-readable target path the panel renders |

A suggestion that does not name a concrete file (or carries an empty section
anchor when one is recorded) is `target_identity_missing` and blocks promotion —
every suggestion points at a concrete file/section rather than a generic
recommendation blob.

Every suggestion also names one trigger `source` with a non-empty `detail` and an
`evidence_ref` pointing at the change that raised it. A suggestion with no detail
or no evidence ref is `trigger_source_detail_missing` and blocks promotion.

A panel must cover the `readme`, `changelog`, `help`, and `tutorial` target kinds;
a partial set is `required_target_kind_missing` and blocks promotion, so the panel
stays the qualified cross-surface boundary rather than a slice that overstates
coverage.

## Chips and evidence-provenance visibility

The chip set is the labels a consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `confidence` | `high`, `medium`, `low`, `heuristic` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `locality` | `local`, `imported_pack`, `mirrored_pack`, `managed` |

`provenance` keeps imported, local-only, mirrored, stale, and derived evidence
visible in the suggestion detail path. Only `first_party_verified` evidence is
authoritative; any other provenance:

- may not be presented at `high` confidence with `authoritative_live` freshness
  (`provenance_truth_collapsed`);
- must stay cited (`suggestion_not_cited`); and
- may never back a one-click apply (`unverified_apply_offered`).

A non-current `version_match` presented at `high` confidence with
`authoritative_live` freshness is `version_truth_collapsed`.

## Diff-first proposals

Each suggestion carries a `proposal` block — the `proposal_kind` (`diff_hunks`,
`new_section_diff`, `link_repoint_diff`, `example_replace_diff`, or
`prose_only_card`), the `hunk_count`, the `added_lines` / `removed_lines` counts,
a `summary`, and a `preview_ref`. A `prose_only_card`, or any proposal with zero
hunks, is `proposal_not_diff_based` and blocks promotion — docs-maintenance
suggestions do not bypass the shared review/diff model just because the target is
prose. A proposal missing its summary or preview ref is `proposal_summary_missing`.

Raw diff bodies never cross the boundary; the `summary` and `preview_ref` are
metadata that let a consumer open the previewable diff.

## Action parity: Apply, Open evidence, Open source, Dismiss, Save for later

Each suggestion carries an `actions` block. Open evidence, Open source, Dismiss,
and Save for later are always available; the only gated action is Apply. A
suggestion that drops any of the four non-apply actions is
`action_parity_incomplete`.

`apply_posture` is one of `preview_required`, `apply_available`,
`apply_blocked_by_policy`, or `apply_unavailable_disclosed`. Only
`first_party_verified` evidence may surface `apply_available`; an unverified
source may surface `preview_required` but never a one-click apply.

## Durable, attributable disposition

Each suggestion carries a `disposition` block — its history-backed `state`. A
`pending` suggestion is open. A resolved suggestion (`applied`, `dismissed`,
`saved_for_later`, `superseded`) must carry an `attributed_to_ref` and a durable
`history_ref` (`disposition_not_attributable`) and stay `previewable` and
`reopenable` (`disposition_not_reopenable`) — applying or dismissing a suggestion
is always attributable, previewable, and reopenable from durable history.

## Export and consumer projections

The `export` mirrors every suggestion into a row preserving target kind, trigger
source, confidence, freshness, apply posture, provenance, disposition state,
action parity, and citation state. A row that drops a preservation flag
(`export_drops_preservation`), disagrees with its suggestion
(`export_*_mismatch`), references an unknown suggestion (`export_row_orphan`), or
leaves a suggestion uncovered (`export_coverage_missing`) blocks promotion.

`consumer_projections` records how each surface projects the panel. The
`docs_suggestion_panel`, `docs_authoring_surface`, `docs_review_panel`, and
`support_export` surfaces are required; a projection that drops a preservation
flag is `consumer_projection_drift` and one that references the wrong packet id is
`consumer_projection_packet_id_mismatch`.

## Degradations and promotion

`panel_degradations` records packet-level degradations (mirror offline, harness
unavailable, link checker offline, diff engine degraded, scope narrowed, panel
narrowed, quarantined source) with a severity. The promotion state is computed
from the validation findings and the degradation severities:

- `stable` — no findings and no narrowing/blocking degradations;
- `narrowed_below_stable` — a narrowing degradation, but the suggestions stay
  visible and attributable;
- `blocks_stable` — any blocking validation finding or blocking degradation.

Every validation finding blocks the Stable claim; narrowing comes only from
data-carried degradation severities so a degraded-but-honest panel narrows rather
than hides its suggestions.

## Boundary

The packet is an inspectable, serde-serializable truth packet. Raw document
bodies, raw source files, raw URLs, diff bodies, raw provider payloads, and
credentials never cross the boundary; a packet that smuggles forbidden material is
`raw_boundary_material_present`.
