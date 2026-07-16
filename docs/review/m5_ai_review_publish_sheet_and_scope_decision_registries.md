# M5 AI-review publish-to-review-sheet and publish-scope-decision registries

Publish-to-review-sheet implement lane over the frozen
[M5 AI-review-assist matrix][matrix] (`m5_ai_review_assist_matrix`). It makes the matrix's
`publish_to_review_sheet` and `resolution_memory_row` object classes operable by carrying
resolved, honest projections of two registries so review, AI, provider, pending-review, and
support / export surfaces inherit one canonical model of *what an outbound AI review publish
action would send* and *whether the provider write path is allowed, downgraded, or blocked* —
rather than hand-authored parallel prose that has to be kept consistent. Every outbound publish
action is diff-first and destination-explicit: a user reviews exactly what leaves the client
before anything becomes durable provider history, and missing or narrowed provider scope surfaces
as an explicit publish-state explanation with copy / export fallback rather than a generic failure.

## Registry-A — publish-to-review sheet

One machine-readable publish-to-review sheet per outbound publish action, carrying:

- the review artifact class and publish mode (`local_draft`, `publish_now_provider_comment`,
  `publish_now_suggested_patch`, `publish_now_check_annotation`, `open_in_provider`,
  `export_fallback_offline`) so an action never publishes or merges implicitly and never hides
  whether output stays local, becomes a provider comment, a suggested patch, or a
  provider-specific check annotation;
- the target provider and the thread or check-run target it would write to;
- the outbound text preview the action would send;
- the attribution state and redaction note;
- the publish / copy / export / cancel actions offered;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A publish sheet that cannot bind its destination to a classified publish mode, that is a
hand-copied per-entry assumption instead of tracing to the shared registry, or that publishes an
incomplete object degrades honestly instead of reading as ready to commit. The registry reuses
the matrix `m5-ai-review-publish-sheet.schema.json` domain schema.

## Registry-B — publish-scope decision

The typed permission-scope decision an outbound path resolves to, keyed to what the provider
write scope allows:

- `publish_scope_allowed` — current permission scope allows the publish path;
- `publish_scope_downgraded` — provider scope is narrowed, so the path downgrades to a copy /
  export fallback with the scope reason shown;
- `publish_scope_blocked` — provider write scope is missing, so the path is blocked with copy /
  export fallback preserved instead of a generic error.

Each decision keeps the same publish packet — attribution, destination, and redaction state —
preserved in local history and support / export so outbound review state stays auditable outside
the live provider UI. A decision that flattens a provider write failure into a generic error,
hides the scope reason, or lets a missing scope masquerade as writable degrades. The registry
reuses the matrix `m5-ai-review-resolution-memory.schema.json` domain schema for the
retained-in-local-history binding.

## Acceptance criteria proven by the resolved examples

1. Users can preview the exact outbound destination and text for seeded comment, suggestion, and
   check-annotation publish actions before commit; a sheet that would commit implicitly without an
   explicit outbound preview degrades.
2. Missing or narrowed provider scope appears as an explicit publish-state explanation with copy /
   export fallback (`publish_scope_downgraded` / `publish_scope_blocked`) instead of a generic
   publish failure.
3. Published review packets retain attribution, destination, and redaction state outside the live
   provider UI — the same publish packet is preserved in local history and support / export.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-ai-review-publish-sheet-and-scope-decision-registries.schema.json`) documents
the shape.

[matrix]: ../../crates/aureline-ui/src/m5_ai_review_assist_matrix/mod.rs
