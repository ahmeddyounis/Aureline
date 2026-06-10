# Mirrored Docs-Pack Recall: Source/Version/Freshness Chips and Stale-Example Findings

This document is the contract for the M5 mirrored docs-pack recall feature. A
recall is a ranked query over the mirror-aware docs packs; each result row
carries the source/version/freshness truth a reader sees, and stale-example
findings keep nearby-version, stale-example, and quarantined-pack states
distinct. The checked-in packet is canonical: the docs browser, search shell, AI
context, retrieval inspector, CLI/headless output, support exports, and
Help/About ingest it rather than cloning status text.

- Record kind: `mirrored_docs_pack_recall_chips_and_stale_example_findings`
- Schema: [`schemas/docs/implement-mirrored-docs-pack-recall-source-or-version-or-freshness-chips-and-stale-example-findings.schema.json`](../../../schemas/docs/implement-mirrored-docs-pack-recall-source-or-version-or-freshness-chips-and-stale-example-findings.schema.json)
- Canonical support export: [`artifacts/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/support_export.json`](../../../artifacts/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/support_export.json)
- Summary artifact: [`artifacts/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings.md`](../../../artifacts/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings.md)
- Fixtures: [`fixtures/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/`](../../../fixtures/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/)
- Producer: `aureline_docs::current_stable_docs_pack_recall_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_pack_recall`

## The chip set

Every `result_row` carries a `chips` block — the five chips a consumer projects
verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `project_docs`, `generated_reference`, `mirrored_official_docs`, `curated_knowledge_pack`, `support_runbook`, `extension_docs_pack` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `locality` | `local`, `mirrored_pack`, `remote_helper`, `managed` |
| `confidence` | `high`, `medium`, `low`, `heuristic` |

Each row also carries an explicit `ranking_reason` and the `open_raw_escape_ref`
/ `open_source_escape_ref` escapes, so no derived result hides its provenance or
its path back to the raw node and the upstream source.

## Mirror awareness

The `mirror_awareness` block records the invariants that keep a pinned, signed
mirror from looking like live vendor truth:

- `pinned_signed_mirror_outranks_live` — a pinned, signed mirror outranks live
  vendor docs in the recall ordering.
- `live_vendor_docs_demoted_when_unpinned` — unpinned live vendor docs are
  demoted, not presented as authoritative.
- `mirror_signature_verified` — mirror signatures are verified before a mirrored
  row is trusted.

A `mirrored_official_docs` row that claims `authoritative_live` freshness without
a pinned, signature-verified pack triggers `live_mirror_looks_more_authoritative`
and blocks the Stable claim.

## Stale-example findings

Stale-example findings attach to a row by `result_id_ref` and keep their class
distinct: `nearby_version`, `stale_example`, `quarantined_pack`, `broken_link`,
`needs_review`, `missing_evidence`. A `nearby_version` finding must carry its
`nearby_version_ref`; a finding that references a result absent from the rows is
an orphan. Both block promotion.

## Promotion and downgrade

`promotion_state` is computed from the worst severity across the validation
findings and the attached stale-example findings:

- a `blocking` finding → `blocks_stable`;
- otherwise a `narrowing` finding → `narrowed_below_stable`;
- otherwise → `stable`.

The fixtures show a mirror-offline recall narrowing (`narrowed_below_stable`, no
blocking findings) and two blocked cases (`live_mirror_over_authoritative`,
`stale_state_collapsed`). `current_stable_docs_pack_recall_export` re-materializes
the checked-in packet and fails if the recorded promotion state drifts from the
freshly computed one, so a stale or under-attributed recall cannot be promoted
silently.

## Boundary

Raw query text, raw document bodies, raw provider payloads, and credentials never
cross this boundary. The packet carries only metadata, chip truth, ranking
reasons, finding summaries, and contract references; `query_digest_ref` is an
opaque digest, never the raw query.

## Out of scope

This feature does not broaden general web-mode or browser-runtime claims beyond
the narrow docs/review/light-edit surfaces qualified in M5. Browser handoff and
scoped browser-surface qualification stay in their own contracts.
