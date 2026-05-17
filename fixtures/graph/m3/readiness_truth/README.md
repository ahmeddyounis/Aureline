# Semantic graph readiness and exact-vs-imported fact label beta corpus

Protected corpus consumed by
`crates/aureline-graph/src/readiness/beta.rs` to validate the beta
projection that gates graph consumer surfaces (navigation, ai_context,
review, support_export) on a single closed fact-lane and readiness
vocabulary.

Each case binds one consumer surface to one claimed fact lane drawn
from the closed beta vocabulary
(`exact_local_graph_fact`, `imported_graph_fact`,
`inferred_graph_fact`, `partial_graph_fact`, `stale_graph_fact`,
`waiting_on_graph_provider`, `out_of_scope_graph_fact`,
`fallback_search_fact`), the lane the underlying alpha cue packet
actually observed, the alpha readiness state, the derived
`claim_alignment_state`
(`aligned`, `weaker_claim_accepted`, `overclaim_blocked`), a
metadata-safe `evidence_export` projection that preserves the fact
lane label, readiness token, consumer surface, and envelope packet
ref, and a closed downgrade label drawn from the readiness-beta
vocabulary.

Acceptance contracts pinned by the manifest:

- every required consumer surface is exercised by at least one case;
- every required fact lane appears as `observed_envelope_lane` in at
  least one case;
- at least one case declares `claim_alignment_state =
  overclaim_blocked` so the overclaim-guard contract is exercised by a
  fixture rather than an anecdote;
- aligned cases declare `downgrade_label = none` and no open gaps;
- non-aligned cases declare a closed downgrade label and at least one
  open gap with a non-`none` class;
- `evidence_export` preserves the fact lane label, readiness token,
  consumer surface label, and envelope packet ref, never admitting
  raw private material or ambient authority.

Schema: `schemas/search/graph_readiness_beta.schema.json`.
Reviewer doc: `docs/search/m3/graph_readiness_beta.md`.
Baseline report:
`artifacts/support/m3/graph_readiness_beta_report.md`.
