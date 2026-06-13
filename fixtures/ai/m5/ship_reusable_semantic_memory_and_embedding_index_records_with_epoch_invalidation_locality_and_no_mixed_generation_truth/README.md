# Reusable Semantic-Memory and Embedding-Index Record Fixtures

## stale_generation_recompute.json

A failure-drill fixture for a model epoch bump from embedding generation `gen-3`
to `gen-4`. The on-device workspace semantic memory has already been rebuilt onto
`gen-4` and stays `current`; the managed embedding index is mid-`recomputing` onto
`gen-4`; and the workspace-mirrored embedding index is still on `gen-3` and is
labeled `stale` rather than served as current retrieval truth. The org reusable
semantic memory remains `policy_blocked` by a region gate with its delete and
export still org-scoped.

The fixture demonstrates that, across a generation bump, stale and recomputing
lanes degrade to precise `degraded_label`s and never masquerade as current truth,
that the four locality states (local, mirrored, managed, policy-blocked) stay
distinct, and that every bound epoch carries its matching invalidation trigger.

The fixture validates against
`schemas/ai/ship-reusable-semantic-memory-and-embedding-index-records-with-epoch-invalidation-locality-and-no-mixed-generation-truth.schema.json`.
