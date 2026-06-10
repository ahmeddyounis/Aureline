# Docs Authoring Suggestions, Stale-Link / Stale-Example Review, and Open-Raw / Open-Source Escapes

This document is the contract for the M5 docs authoring-and-review boundary — the
records that let a docs review item carry a suggested edit and a freshness
verdict without ever letting an unverified suggestion present a one-click apply, a
stale link or example look current, or an item strand the reader without a path
back to the underlying node and upstream source. Each item is one of:

- an **authoring suggestion** — a suggested edit to a docs node;
- a **stale-link review** — a review of a link that may be broken or redirected;
- a **stale-example review** — a review of an example that may have drifted from
  the code it documents;
- a **freshness review** — a review of a docs node's freshness against the build.

Every item carries the same source/version/freshness/locality/confidence chip set
the other docs lanes use, an **authoring-suggestion** block (*the suggested edit,
its apply posture, and the trigger that raised it*), a **stale review verdict**
(*the finding class and its severity* — the stale-link / stale-example review
truth), one **trust-class disclosure** (*how trustworthy the origin is*), the
live-vs-captured state, citation state, and the open-raw / open-source escapes. An
export preserves the item-kind / trust-class / source / confidence / apply-posture
/ finding-class / escape truth that support, AI evidence, and diagnostics surfaces
ingest rather than cloning status text. The docs authoring surface, docs browser
shell, docs review panel, stale-example review queue, AI-context inspector,
CLI/headless output, support exports, diagnostics, and Help/About all consume the
checked-in packet.

- Record kind: `docs_authoring_review_controls`
- Schema: [`schemas/docs/implement-docs-authoring-suggestions-stale-link-or-stale-example-review-and-open-raw-or-open-source-escapes.schema.json`](../../../schemas/docs/implement-docs-authoring-suggestions-stale-link-or-stale-example-review-and-open-raw-or-open-source-escapes.schema.json)
- Canonical support export: [`artifacts/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/support_export.json`](../../../artifacts/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/support_export.json)
- Summary artifact: [`artifacts/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes.md`](../../../artifacts/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes.md)
- Fixtures: [`fixtures/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/`](../../../fixtures/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/)
- Producer: `aureline_docs::current_stable_docs_authoring_review_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_authoring_review`

## The items and their chips

`items` is the set of docs review items for one session. Every item points at a
`subject_ref`, carries an `item_kind` (`authoring_suggestion`, `stale_link_review`,
`stale_example_review`, `freshness_review`), a `title`, a `detail` (the
human-readable summary — never a raw doc or diff body), and a `chips` block — the
five chips a consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `first_party_doc`, `signed_docs_pack`, `imported_docs_pack`, `mirrored_vendor_doc`, `live_mirror`, `derived_heuristic` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `locality` | `local`, `imported_pack`, `mirrored_pack`, `managed` |
| `confidence` | `high`, `medium`, `low`, `heuristic` |

Every packet must include at least one `authoring_suggestion`, one
`stale_link_review`, and one `stale_example_review` item — a partial set is
`required_item_kind_missing` and blocks promotion, so the set stays the qualified
authoring-and-review boundary rather than a slice that overstates coverage.

## Authoring-suggestion apply-posture truth

Each item carries a `suggestion` block — the `apply_posture` the suggested edit is
offered under (`preview_required`, `apply_available`, `apply_blocked_by_policy`,
`suggestion_only`, `apply_unavailable_disclosed`), the `trigger` that raised it
(`manual_authoring`, `stale_example_detected`, `broken_link_detected`,
`version_drift_detected`, `style_lint_hint`, `ai_authoring_assist`), and a `note`.
Two rules keep the apply action honest:

- An item whose origin is **unverified** (`live_mirror_suggestion` or
  `derived_heuristic_only`) may never present a one-click `apply_available` — it is
  `unverified_suggestion_apply_offered` and blocks promotion. An unverified
  suggestion may surface a preview, but a human applies it.
- A one-click `apply_available` may never be offered while the item's own review
  verdict is **blocking** — a broken or uncompilable example is
  `apply_offered_on_blocking_finding` and blocks promotion. The review must clear
  before the edit applies in one click.

An empty apply note is `apply_posture_note_missing`.

## Stale-link / stale-example review truth

Each item carries a `review` verdict — the `finding_class` (`fresh_ok`,
`stale_link_broken`, `stale_link_redirected`, `stale_example_drifted`,
`stale_example_uncompilable`, `stale_example_version_mismatch`, `needs_review`,
`quarantined_source`), the `severity` of the verdict, and a `note`. A stale verdict
(any of the `stale_*` or `quarantined_source` classes) may never claim
live-authoritative freshness: a stale link or example whose `freshness` chip is
`authoritative_live` is `stale_verdict_freshness_mismatch` and blocks promotion — a
broken link can never read as current. A non-current `version_match` presented at
`high` confidence and `authoritative_live` freshness is `version_truth_collapsed`.
An empty review note is `review_verdict_note_missing`.

The review-verdict `severity` drives the promotion narrowing: a verdict at
`blocking` severity blocks the claim, a verdict at `narrowing` severity narrows it
below Stable, and an `advisory` verdict leaves a clean set Stable. A drifted
example narrows the claim and is shown as stale; it is not hidden.

## The trust-class disclosure

Each item carries one `trust_class`:

| Trust class | Meaning |
| --- | --- |
| `first_party_authored` | first-party docs authored in-repo |
| `signed_docs_pack` | a signed first-party docs pack |
| `imported_docs_pack` | a docs pack imported from a signed extension set |
| `live_mirror_suggestion` | a live-mirror suggestion, not verified at materialization time |
| `derived_heuristic_only` | a derived / inferred suggestion only |

An untrusted origin (`live_mirror_suggestion`, `derived_heuristic_only`) presented
at `high` confidence is `trust_class_disclosure_collapsed`, and an untrusted item
that is not cited is `item_not_cited`. Both block promotion. Every item carries a
`trust_disclosure_note`; an empty note is `trust_class_disclosure_missing`.

## Open-raw / open-source escapes

Every item carries an `open_raw_escape_ref` (open the underlying doc node) and an
`open_source_escape_ref` (open the upstream source). A missing escape is
`open_raw_open_source_escape_missing` and blocks promotion — the reader can always
leave the qualified surface for the raw truth.

## The export and consumer projections

`export` is the projection support, AI evidence, and diagnostics surfaces ingest:
one `export_row` per item preserving `item_kind`, `trust_class`, `source_class`,
`confidence`, `apply_posture`, `finding_class`, `review_severity`, `cited`, and the
escapes. An export that drops a preservation flag is `export_drops_preservation`;
a row that disagrees with its item is an `export_*_mismatch`; an orphan row or an
uncovered item is `export_row_orphan` / `export_coverage_missing`.

`consumer_projections` records how each surface projects the set. Every packet
must project the docs authoring surface, the docs review panel, the stale-example
review queue, and the support export — a missing surface is
`required_surface_coverage_missing`. A projection that drops a preservation flag is
`consumer_projection_drift`; one that references the wrong packet is
`consumer_projection_packet_id_mismatch`.

## Degradations and promotion

`review_degradations` records packet-level degradations (a mirror offline, the
example harness or link checker unavailable, the suggestion engine degraded, a
narrowed scope or review, a quarantined source, a broken anchor). Each carries a
`severity`; a blocking degradation blocks, a narrowing degradation narrows. The
computed `promotion_state` is the worst severity across the validation findings,
the per-item review verdicts, and the degradations:

- `stable` — no findings, every review verdict advisory, no narrowing/blocking
  degradation;
- `narrowed_below_stable` — a narrowing review verdict or degradation, no blocking
  finding;
- `blocks_stable` — any blocking validation finding, review verdict, or
  degradation.

## Boundary

The packet carries no raw document bodies, no raw source files, no raw URLs, no
diff bodies, no raw provider payloads, and no credentials — only metadata,
apply-posture truth, staleness truth, chip truth, cited refs, provenance, finding
summaries, and contract refs. Forbidden material in the export is
`raw_boundary_material_present` and blocks promotion.
