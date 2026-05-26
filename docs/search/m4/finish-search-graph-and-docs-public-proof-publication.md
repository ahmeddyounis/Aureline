# Finish search, graph, and docs public-proof publication — stable contract

Status: Stable lane proof for the public-proof publication of search,
graph, and docs surfaces with known limits and downgrade automation.

This document is the reviewer-facing contract for the stable public-proof
publication truth packet. The packet is the single source of truth that
the search shell, graph topology canvas, docs/help, CLI/headless
inspector, support export, release proof index, Help/About proof card,
and the published-truth dashboard all read; surfaces MUST NOT mint local
copies or paraphrase publication posture.

## What the packet asserts

For each governed *publication lane class × public-proof row* the packet
asserts:

1. The **publication lane class** — one of `search_public_proof_lane`,
   `graph_public_proof_lane`, `docs_public_proof_lane`. Every certified
   packet MUST carry at least one row for each of the three required
   lanes.
2. The **publication row class** — one of `search_public_truth`,
   `graph_public_truth`, `docs_public_truth`, `known_limit`, or
   `downgrade_automation`. A row class is permitted only on the matching
   lane (e.g., `search_public_truth` only on
   `search_public_proof_lane`); `known_limit` and `downgrade_automation`
   may be attached to any lane.
3. The **publication state** — one of `published_stable`,
   `narrowed_below_stable`, or `withheld_pending_gap`. The validator
   refuses to certify a row that claims `published_stable` while any
   binding is unbound.
4. The **known-limit class** — one of `none_declared`,
   `corpus_warmup_partial`, `scope_subset_only`,
   `imported_provider_only`, `archived_snapshot_only`, `heuristic_only`,
   `feature_preview_only`, or `limit_unbound`. A row whose known limit
   is `limit_unbound` is refused.
5. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_partial_warm`, `auto_withhold_on_provider_outage`,
   `auto_archive_on_stale_capture`, `auto_demote_on_low_confidence`,
   `auto_block_on_missing_evidence`, `manual_only_pending_review`, or
   `automation_unbound`. A row whose automation is `automation_unbound`
   is refused.
6. The **proof-artifact class** — one of `release_proof_index_entry`,
   `support_export_packet`, `help_about_proof_card`,
   `docs_published_truth_card`, `dashboard_published_truth_row`, or
   `cli_headless_inspector_row`. The artifact class names where the
   row's public proof is published.
7. The **publication confidence class** — `high_confidence`,
   `medium_confidence`, or `low_confidence`. A row that claims
   `published_stable` at `low_confidence` is narrowed below stable until
   evidence grows.
8. The **evidence refs** — every row preserves at least one
   repo-relative evidence ref proving the publication claim.
9. The **disclosure ref** — every row that is not `published_stable`,
   that declares a non-`none_declared` known limit, or that binds a
   non-`none` downgrade automation MUST carry a repo-relative disclosure
   ref shown to the user.

## Boundary safety

Every row carries `raw_query_material_excluded`, `secrets_excluded`, and
`ambient_authority_excluded`. The validator emits
`raw_query_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one of
those booleans to false. The packet never admits raw query text, raw
source bodies, secrets, ambient credentials, or provider payloads.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `published_stable` while its known-limit class is
  `limit_unbound` or its downgrade-automation class is
  `automation_unbound`,
- a row's row class is not permitted on its lane,
- a row is `narrowed_below_stable` or `withheld_pending_gap` but drops
  its disclosure ref,
- a row declares a non-`none_declared` known limit and drops its
  disclosure ref,
- a row binds a non-`none` downgrade automation and drops its disclosure
  ref,
- any of the eight required consumer projections is missing or collapses
  one of the closed vocabularies (lane, row class, publication state,
  known limit, downgrade automation, or proof artifact),
- raw query text, source bodies, secrets, or ambient credentials slip
  past the boundary,
- the stored promotion state disagrees with the derived findings.

## Required consumer projections

The packet REQUIRES one preserved projection per surface:
`search_shell`, `graph_topology`, `docs_help`, `cli_headless`,
`support_export`, `release_proof_index`, `help_about`, and
`dashboard_published_truth`. Each projection MUST keep the lane class,
row class, publication state, known-limit class, downgrade-automation
class, and proof-artifact class verbatim, MUST support JSON export, and
MUST exclude raw private material and ambient authority.

## How to read the packet

Consumers materialize the packet through
`PublicProofPublicationTruthPacket::materialize` and then read the
projection that matches their surface. The packet is metadata-only and
suitable for inclusion in any support export or release proof bundle.

## Closed vocabulary

**Publication lane classes** — `search_public_proof_lane`,
`graph_public_proof_lane`, `docs_public_proof_lane`.

**Publication row classes** — `search_public_truth`,
`graph_public_truth`, `docs_public_truth`, `known_limit`,
`downgrade_automation`.

**Publication states** — `published_stable`, `narrowed_below_stable`,
`withheld_pending_gap`.

**Known-limit classes** — `none_declared`, `corpus_warmup_partial`,
`scope_subset_only`, `imported_provider_only`,
`archived_snapshot_only`, `heuristic_only`, `feature_preview_only`,
`limit_unbound`.

**Downgrade-automation classes** — `none`,
`auto_narrow_on_partial_warm`, `auto_withhold_on_provider_outage`,
`auto_archive_on_stale_capture`, `auto_demote_on_low_confidence`,
`auto_block_on_missing_evidence`, `manual_only_pending_review`,
`automation_unbound`.

**Proof-artifact classes** — `release_proof_index_entry`,
`support_export_packet`, `help_about_proof_card`,
`docs_published_truth_card`, `dashboard_published_truth_row`,
`cli_headless_inspector_row`.

**Consumer surfaces** — `search_shell`, `graph_topology`, `docs_help`,
`cli_headless`, `support_export`, `release_proof_index`, `help_about`,
`dashboard_published_truth`.

**Finding kinds** — see
`schemas/search/public_proof_publication_truth.schema.json` for the
closed list. Notable invariants:

- `raw_query_material_present`, `secrets_present`,
  `ambient_authority_present` — boundary safety.
- `missing_known_limit`, `missing_downgrade_automation`,
  `published_stable_with_unbound_binding` — the row claims a stable
  publication without binding the limit or automation needed to defend
  it.
- `narrowed_row_missing_disclosure_ref`,
  `known_limit_missing_disclosure_ref`,
  `downgrade_automation_missing_disclosure_ref` — required disclosure
  refs were dropped from the row.
- `missing_consumer_projection`, `consumer_projection_drift`,
  `lane_vocabulary_collapsed`, `row_class_vocabulary_collapsed`,
  `publication_state_vocabulary_collapsed`,
  `known_limit_vocabulary_collapsed`,
  `downgrade_automation_vocabulary_collapsed`,
  `proof_artifact_vocabulary_collapsed` — surface drift.

## Companion artifacts

- Schema: `schemas/search/public_proof_publication_truth.schema.json`
- Checked-in packet:
  `artifacts/search/m4/public_proof_publication_truth_packet.json`
- Fixture corpus:
  `fixtures/search/m4/public_proof_publication_truth_packet/`
- Reviewer artifact:
  `artifacts/search/m4/finish-search-graph-and-docs-public-proof-publication.md`
- Rust contract:
  `crates/aureline-graph/src/public_proof_publication_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-graph/tests/public_proof_publication_truth_packet.rs`

## Anchored normative sources

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — search,
  graph, and docs publication-honesty rules.
- `.t2/docs/Aureline_Technical_Design_Document.md` — known-limit and
  downgrade-automation contracts for the knowledge plane.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — public-proof publication
  presentation rules across search, graph, docs, and the
  published-truth dashboard.
- `.t2/docs/Aureline_PRD.md` Appendix P FR-SEARCH-002 — search-row
  scope/freshness/provenance disclosure rules that the public-proof
  publication packet binds.

If any of those sources disagree with this document, the source wins
and this document plus the schema and fixtures MUST be updated in the
same change.

#corpus-warmup-partial

The `corpus_warmup_partial` known-limit class is paired with the
`auto_narrow_on_partial_warm` downgrade automation: the search lane
auto-narrows the published claim while the canonical corpus is still
warming, so a partial corpus never masquerades as the published-stable
search lane.

#imported-provider-only

The `imported_provider_only` known-limit class is paired with the
`auto_withhold_on_provider_outage` downgrade automation: the lane
auto-withholds its publication when the imported provider is
unavailable, so imported claims never inherit canonical certainty.

#archived-snapshot-only

The `archived_snapshot_only` known-limit class is paired with the
`auto_archive_on_stale_capture` downgrade automation: the lane
auto-demotes the row to an archived snapshot when the capture ages past
its freshness SLO.

#heuristic-only

The `heuristic_only` known-limit class is paired with the
`auto_demote_on_low_confidence` downgrade automation: the lane
auto-demotes heuristic rows whose confidence drops below the certified
bar.

#feature-preview-only

The `feature_preview_only` known-limit class is paired with the
`manual_only_pending_review` downgrade automation: the lane is held in
preview and waits for a manual review before claiming stable, so a
preview surface never inherits an adjacent stable row.
