# M5 AI-review resolution-memory-row and finding-lifecycle-transition registries

Resolution-memory-row implement lane over the frozen
[M5 AI-review-assist matrix][matrix] (`m5_ai_review_assist_matrix`). It makes the matrix's
`resolution_memory_row` and `ai_review_finding_row` object classes operable by carrying resolved,
honest projections of two registries so review, AI, provider, pending-review, and support / export
surfaces inherit one canonical model of *what happened to each AI review finding over time* and
*how a published, outdated, or reopened state joins back to its original finding and diff scope* —
rather than dismissal, suppression, publish, staleness, and reopen decisions disappearing into UI
state. Each transition preserves the finding's stable ID, actor / source, timestamp, rationale
class, and any provider destination or local-draft relation, so a finding's history stays
attributable and reviewable after refresh, restart, export, and support capture.

## Registry-A — resolution-memory row

One durable resolution-memory row per finding-state transition, recording the finding's lifecycle
state across the six governed transitions:

- `open`, `dismissed`, `suppressed`, `published`, `outdated`, `reopened` — kept distinct so a
  `dismissed` finding never collapses into the same generic hidden state as a `suppressed` one, and
  a stale finding never keeps looking current after diff or instruction drift;
- the actor / source and rationale class captured on each transition, without shaming language or
  anthropomorphic copy;
- the reopen action and any provider destination or local-draft relation the row preserves;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A resolution-memory row that cannot bind its transition to a classified lifecycle state, that is a
hand-copied per-entry assumption instead of tracing to the shared registry, or that drops a
required field degrades honestly instead of reading as a clean, provable history entry. The
registry reuses the matrix `m5-ai-review-resolution-memory.schema.json` domain schema.

## Registry-B — finding-lifecycle transition

The typed lifecycle-transition join that ties a published, outdated, or reopened state back to the
original finding and diff scope so later review and support exports can reconstruct the full
lifecycle:

- `published_transition_joined` — a published state joined back to its originating finding and diff
  scope;
- `outdated_transition_joined` — an outdated / stale state joined back so the finding is never
  falsely presented as current;
- `reopened_transition_joined` — a reopened state joined back to the prior lineage rather than
  starting a fresh, disconnected finding.

Each transition keeps the same finding packet — stable ID, actor / source, rationale class, and
destination or local-draft relation — preserved in local history and support / export so the
lifecycle stays available after restart and never implies provider commitment where none exists. A
transition that hides the join back to the original finding, or lets a missing scope masquerade as a
committed provider state, degrades. The registry reuses the matrix `m5-ai-review-finding.schema.json`
domain schema for the joined-finding binding.

## Acceptance criteria proven by the resolved examples

1. Seeded findings retain a readable history through dismiss, suppress, publish, outdated, and
   reopen transitions with stable IDs; a row that would collapse a transition into an unclassified
   state degrades.
2. Dismissed or suppressed findings preserve actor / source and rationale class rather than
   collapsing into a generic hidden state.
3. Resolution-memory rows remain available after restart, export, and support capture — the same
   finding packet is preserved in local history and support / export — without implying provider
   commitment where none exists.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-ai-review-resolution-memory-and-finding-lifecycle-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_ai_review_assist_matrix/mod.rs
