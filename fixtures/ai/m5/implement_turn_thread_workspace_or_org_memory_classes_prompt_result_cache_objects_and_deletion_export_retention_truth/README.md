# Materialized AI Memory Class Fixtures

## held_backend_unsupported.json

A materialization fixture where the org-scoped reusable semantic memory object is
unavailable because the managed vector backend is not supported on this build.
Every other object — the per-turn ephemeral state, thread derived cache and
prompt-result cache, and the workspace and org saved memory — stays available.
The blocked object carries the `unavailable_unsupported_backend` availability
class plus a precise `degraded_label` rather than collapsing into a generic
"memory unavailable" state, demonstrating that a missing or policy-blocked memory
class degrades to a precise label.

The fixture validates against
`schemas/ai/implement-turn-thread-workspace-or-org-memory-classes-prompt-result-cache-objects-and-deletion-export-retention-truth.schema.json`.
