# Retrieval Locality Inspector Fixtures

## degraded_provider_overlay_context_pack.json

An inspector packet whose AI context pack draws on a region-pinned managed
provider overlay that is **recomputing** and therefore shown degraded. The
overlay lane carries `state: degraded` with `generation_state: recomputing`,
the surface's `provider_overlay_posture` is `overlay_degraded`, the degraded
lane is disclosed in `degraded_lanes`, two in-scope candidates are reported as
hidden, and the completeness claim is `degraded_subset` — never `complete`.

Search and docs recall remain unchanged from the canonical export. The fixture
demonstrates that a degraded provider overlay is disclosed rather than collapsed
into a generic provider error, that a recomputing generation surfaces as
degraded instead of masquerading as current, and that hidden scope forces an
honest completeness label. It validates against
`schemas/ai/add-retrieval-locality-inspectors-contribution-lanes-ranking-or-chunking-reasons-and-lexical-or-graph-or-docs-pack-or-em.schema.json`.
