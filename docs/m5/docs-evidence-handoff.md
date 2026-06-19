# Docs Evidence Handoff

This document is the contract for the M5 docs-evidence handoff — the surface that
makes a docs change explainable by the code, schema, run, or release evidence
that motivated it, instead of free-form narrative alone. A handoff entry is never
an unattributed prose edit: it names a concrete **change**, binds it to one or
more typed **evidence** objects, and preserves the **scope**, **redaction**,
**provenance**, **freshness**, and **mirror/offline** truth of each binding, so
review, support, AI explanation, and release/public-truth surfaces can reopen the
same docs causality Aureline used in the authoring workspace.

Each entry carries:

- a **change** — the change kind (`readme_edit`, `changelog_entry`,
  `release_note_edit`, `help_edit`, `tutorial_edit`, `api_reference_edit`,
  `module_doc_edit`, `suggestion_proposal`), the concrete doc ref, an optional
  section anchor, a human-readable display path, a label, and an optional
  originating docs-suggestion ref when the change came from the suggestion panel;
- one or more **bindings** — each tying the change to one typed evidence object;
- an **entry scope** — the overall sharing/export scope for the change;
- a **reopen handle** — so the entry stays reopenable from both the review and
  support flows;
- a **detail** — a human-readable summary (never a raw body).

Each binding carries:

- an **evidence kind** — `source_file`, `symbol`, `api_contract`,
  `failing_example`, `test_run`, `release_object`, or `human_note`;
- a **target ref**, **display path**, and **label** — the concrete evidence
  object (never a raw body, diff, or URL);
- a **scope** — `local_only`, `review_handoff_scoped`, `export_safe_shared`, or
  `blocked_unscoped`;
- a **redaction state** — `metadata_safe`, `redacted_for_export`,
  `local_only_redaction_required`, or `not_applicable`;
- an **evidence provenance** disclosure — `first_party_verified`,
  `local_only_unverified`, `imported`, `mirrored`, `stale`, or
  `derived_heuristic` — so a cached or imported source is never mistaken for
  authoritative live truth;
- a **freshness** / **version_match** / **locality** chip set;
- a **mirror/offline** posture — `online_live`, `mirror_served`,
  `offline_cached_usable`, or `offline_unavailable` — so docs causality stays
  usable in air-gapped and mirror-first profiles;
- an **open-evidence ref** — so review, support, and AI flows can reopen the
  evidence object;
- a **cited** flag plus an optional citation ref.

An export preserves the change-subject / evidence-binding / scope / redaction /
provenance / freshness / mirror-offline / reopen truth that review, support, AI
explanation, release/public-truth, and diagnostics surfaces ingest rather than
restating prose causality. The docs evidence-handoff surface, docs authoring
surface, docs review panel, docs browser shell, AI explanation surface, release
center / public-truth lane, CLI/headless output, support exports, diagnostics,
and Help/About all consume the checked-in packet.

- Record kind: `docs_evidence_handoff`
- Schema: [`schemas/docs/docs-evidence-handoff.schema.json`](../../schemas/docs/docs-evidence-handoff.schema.json)
- Canonical support export: [`artifacts/docs/m5/docs-evidence-handoff-proof/support_export.json`](../../artifacts/docs/m5/docs-evidence-handoff-proof/support_export.json)
- Summary artifact: [`artifacts/docs/m5/docs-evidence-handoff-proof.md`](../../artifacts/docs/m5/docs-evidence-handoff-proof.md)
- Fixtures: [`fixtures/docs/m5/docs-evidence-handoff/`](../../fixtures/docs/m5/docs-evidence-handoff/)
- Producer: `aureline_docs::current_stable_docs_evidence_handoff_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_evidence_handoff`

## Concrete, typed traceability

`entries` is the set of handoff entries for one docs change sweep. Every entry
points at a concrete `change` (doc ref, display path, label, and a non-empty
section anchor / originating suggestion ref when one is recorded;
`change_subject_missing` otherwise) and carries a `detail`
(`change_detail_missing` if empty).

Every entry must bind its change to at least one **concrete** typed evidence
object — a `source_file`, `symbol`, `api_contract`, `failing_example`,
`test_run`, or `release_object`. An entry with no bindings is `bindings_empty`;
an entry whose only bindings are `human_note`s relies on narrative alone and is
`change_not_concretely_traced`. The packet as a whole must demonstrate the
`source_file`, `symbol`, `api_contract`, `failing_example`, `test_run`, and
`release_object` evidence kinds (`required_evidence_kind_missing`), so the handoff
proves a docs change can be traced to files, symbols, contracts, failing
examples, runs, and releases — not narrative alone.

Each binding names a concrete `target_ref`, `display_path`, and `label`
(`binding_target_missing` otherwise), an `open_evidence_ref`
(`binding_open_evidence_missing` if empty), and a `provenance_disclosure_note`
(`provenance_disclosure_missing` if empty).

## Local-only versus shared/export scope and redaction

`scope` keeps docs work local unless it crosses an explicit, scoped review or
export boundary:

| Scope | Meaning |
| --- | --- |
| `local_only` | stays on this machine; never crosses a share/export boundary |
| `review_handoff_scoped` | staged for a scoped review handoff that stays inside review |
| `export_safe_shared` | export-safe; may cross the support/export/public-truth boundary |
| `blocked_unscoped` | an unscoped external share/export was attempted and is blocked |

Scope and redaction stay consistent:

- a binding whose redaction is `local_only_redaction_required` must stay
  `local_only`, and a binding that is `export_safe_shared` must carry an
  export-ready redaction (`metadata_safe` or `redacted_for_export`) —
  `scope_redaction_inconsistent` otherwise;
- a `local_only_unverified` binding may not be marked `export_safe_shared`
  (`local_only_marked_export_safe`);
- an entry's scope may never be wider than its bindings — an `export_safe_shared`
  entry may not contain a non-export-safe binding, and a review/export-scoped
  entry may not contain a `local_only` binding
  (`entry_scope_wider_than_bindings`).

So local-only evidence is never silently widened to shared/export, and the
handoff stays usable in air-gapped or mirror-first profiles.

## Mirror/offline continuity, provenance, and freshness truth

The chip set is the labels a consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `locality` | `local`, `imported_pack`, `mirrored_pack`, `managed` |
| `mirror_offline` | `online_live`, `mirror_served`, `offline_cached_usable`, `offline_unavailable` |

A `mirror_served` or offline binding may not claim `authoritative_live` freshness
(`offline_claims_live_freshness`). Only `first_party_verified` evidence is
authoritative; any other provenance:

- may not be presented as `authoritative_live` truth (`evidence_truth_collapsed`);
  and
- must stay cited (`binding_not_cited`).

A non-current `version_match` presented as `authoritative_live` truth is
`version_truth_collapsed`.

## Reopenable from review and support

Each entry carries a `reopen` handle with a non-empty `reopen_ref` that stays
`reopenable`, `available_in_review`, and `available_in_support`
(`entry_not_reopenable` otherwise) — so support and review can reopen the same
docs-evidence packet Aureline used in the authoring workspace.

## Export and consumer projections

The `export` mirrors every entry into a row preserving the change kind, doc ref,
the set of evidence kinds, the binding count, the entry scope, the export-safe
flag, the reopenable flag, and the cited flag. An export that drops a
preservation flag (`export_drops_preservation`), disagrees with its entry
(`export_*_mismatch`), references an unknown entry (`export_entry_orphan`), or
leaves an entry uncovered (`export_coverage_missing`) blocks promotion — so scope,
redaction, and mirror/offline truth stay visible through export and support flows.

`consumer_projections` records how each surface projects the handoff. The
`docs_review_panel`, `ai_explanation`, `release_public_truth`, and
`support_export` surfaces are required (`required_surface_coverage_missing`); a
projection that drops a preservation flag is `consumer_projection_drift` and one
that references the wrong packet id is `consumer_projection_packet_id_mismatch` —
so docs causality is never locked inside the authoring pane.

## Degradations and promotion

`handoff_degradations` records packet-level degradations (mirror offline, source
index unavailable, evidence refresh pending, scope narrowed for export, handoff
narrowed, quarantined source) with a severity. The promotion state is computed
from the handoff findings and the degradation severities:

- `stable` — no findings and no narrowing/blocking degradations;
- `narrowed_below_stable` — a narrowing degradation, but the entries stay visible
  and reopenable;
- `blocks_stable` — any blocking handoff finding or blocking degradation.

Every handoff finding blocks the Stable claim; narrowing comes only from
data-carried degradation severities so a degraded-but-honest handoff narrows
rather than hides its entries.

## Boundary

The packet is an inspectable, serde-serializable truth packet. Raw document
bodies, raw source files, raw diffs, raw URLs, rendered HTML, raw provider
payloads, and credentials never cross the boundary; a packet that smuggles
forbidden material is `raw_boundary_material_present`.
