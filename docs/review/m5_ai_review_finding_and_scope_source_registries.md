# M5 AI-review-finding-record and scope-source binding registries

First implement lane over the frozen [M5 AI-review-assist matrix][matrix]
(`m5_ai_review_assist_matrix`). It makes the matrix's `ai_review_finding_row` and
`review_scope_selector` object classes operable by carrying resolved, honest projections of
two registries so review, AI, provider, pending-review, and support / export surfaces inherit
one canonical set of durable AI review finding objects rather than a hand-authored parallel
prose that has to be kept consistent.

## Registry-A — AI review finding record

One machine-readable finding record per inspectable AI review finding, carrying:

- a stable finding identity that survives row virtualization, tray filtering, reopen / replay,
  export packets, and support bundles;
- the finding class (correctness bug, security risk, performance concern, maintainability
  smell, test gap, style nit) with its severity / confidence — never collapsed into generic
  prose;
- the analyzed evidence anchors (affected files / hunks, evidence links) bound to the finding;
- the repo-instruction / check-pack source that shaped the finding;
- the current local-draft-versus-provider-committed state;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A finding record that cannot bind its evidence to a classified finding class, that is a
hand-copied per-entry assumption instead of tracing to the shared registry, or that publishes
an incomplete object degrades honestly instead of reading as a clean, current finding. The
registry reuses the matrix `m5-ai-review-finding.schema.json` domain schema.

## Registry-B — scope-source binding

The typed binding event that names which analyzed diff scope, repository-instruction source,
or evidence anchor shaped a finding, so a changed diff scope or instruction source stays a
visible, typed event rather than a silent mutation that leaves a stale finding looking current.
The registry reuses the matrix `m5-ai-review-scope-selector.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Seeded AI review findings resolve with stable IDs and unchanged evidence anchors across
   every claimed consumer surface; an object-incomplete finding degrades.
2. Every visible finding can explain which diff scope and repository-instruction / check
   source shaped it; a finding that runs support language ahead of its proof degrades.
3. No finding row collapses finding class, severity / confidence, or local-versus-provider
   state into generic prose; the binding registry keeps each scope-source dimension distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-ai-review-finding-and-scope-source-registries.schema.json`) documents the
shape.

[matrix]: ../../crates/aureline-ui/src/m5_ai_review_assist_matrix/mod.rs
