# Search, graph, and docs public-proof publication — reviewer artifact

This is the reviewer-facing artifact for the M4 stable public-proof
publication truth packet covering search, graph, and docs surfaces with
known limits and downgrade automation. The contract lives at
`docs/search/m4/finish-search-graph-and-docs-public-proof-publication.md`
and is replayed by
`crates/aureline-graph/tests/public_proof_publication_truth_packet.rs`.

## Stable claim

For every governed publication lane class (`search_public_proof_lane`,
`graph_public_proof_lane`, `docs_public_proof_lane`) the packet binds:

- at least one `*_public_truth` row per lane (the lane's published-proof
  truth),
- at least one `known_limit` row per lane (the disclosed known limit),
- at least one `downgrade_automation` row per lane (the automation that
  narrows or withholds the lane when a binding slips),
- a closed `publication_state` (no surface pretends `published_stable`
  while bindings are unbound),
- a closed `known_limit_class` (corpus warm-up, scope subset, imported
  provider, archived snapshot, heuristic, preview, or `none_declared`),
- a closed `downgrade_automation_class` (auto-narrow, auto-withhold,
  auto-archive, auto-demote, auto-block, manual-only, or `none`),
- a closed `proof_artifact_class`,
- a closed `publication_confidence_class`, and
- at least one `evidence_refs` entry plus a `disclosure_ref` whenever
  the row is not `published_stable`, declares a non-`none_declared`
  known limit, or binds a non-`none` downgrade automation.

## Companion artifacts

- Schema: `schemas/search/public_proof_publication_truth.schema.json`
- Checked-in packet:
  `artifacts/search/m4/public_proof_publication_truth_packet.json`
- Fixture corpus:
  `fixtures/search/m4/public_proof_publication_truth_packet/`
- Rust contract:
  `crates/aureline-graph/src/public_proof_publication_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-graph/tests/public_proof_publication_truth_packet.rs`
- Reviewer doc:
  `docs/search/m4/finish-search-graph-and-docs-public-proof-publication.md`

## Required consumer projections

The packet is preserved verbatim across eight consumer projections:

| Projection                  | Surface                                   |
| --------------------------- | ----------------------------------------- |
| `search_shell`              | Search shell results pane                 |
| `graph_topology`            | Graph topology canvas and table fallback  |
| `docs_help`                 | Docs/help reviewer surface                |
| `cli_headless`              | CLI/headless inspector                    |
| `support_export`            | Support export bundle                     |
| `release_proof_index`       | Release proof index entry                 |
| `help_about`                | Help/About proof card surface             |
| `dashboard_published_truth` | Published-truth dashboard surface         |

A projection that collapses any closed vocabulary, drops the packet id,
drops the row class, publication state, known limit,
downgrade-automation, or proof-artifact vocabulary, or leaks raw private
material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `published_stable` while its known-limit or
  downgrade-automation class is unbound,
- a row's row class is not permitted on its lane,
- a row is `narrowed_below_stable` or `withheld_pending_gap` but drops
  its disclosure ref,
- a row declares a non-`none_declared` known limit and drops its
  disclosure ref,
- a row binds a non-`none` downgrade automation and drops its disclosure
  ref,
- any of the eight required consumer projections is missing or collapses
  one of the closed vocabularies,
- raw query text, source bodies, secrets, or ambient credentials slip
  past the boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`PublicProofPublicationTruthPacket::materialize` and then read the
projection that matches their surface. The packet is metadata-only and
suitable for inclusion in any support export or release proof bundle.

## Where the packet lives

- Schema: [`schemas/search/public_proof_publication_truth.schema.json`](../../../schemas/search/public_proof_publication_truth.schema.json)
- Reviewer doc: [`docs/search/m4/finish-search-graph-and-docs-public-proof-publication.md`](../../../docs/search/m4/finish-search-graph-and-docs-public-proof-publication.md)
- Fixture corpus: [`fixtures/search/m4/public_proof_publication_truth_packet/`](../../../fixtures/search/m4/public_proof_publication_truth_packet/)
- Rust module: [`crates/aureline-graph/src/public_proof_publication_truth_packet/mod.rs`](../../../crates/aureline-graph/src/public_proof_publication_truth_packet/mod.rs)
