# AI package-mutation-review fixtures

These fixtures exercise the AI composer's propose-only view of governed package
mutations (`aureline-ai`, `package_mutation_review`). Each file is an
`ai_package_mutation_review` packet validated with
`AiPackageMutationReviewPacket::validate`, and every proposal binds by reference
to the cross-surface governance contract in `aureline-deps`
(`automation_governance`).

- **`ai_add_proceed.json`** — an AI add proposal that is propose-only,
  preview-first, routed through governed review, and reflects a governed
  proceed → committed outcome.
- **`ai_capability_gap_blocked.json`** — an AI add proposal against an offline
  registry with unsatisfied auth that reflects a governed blocked outcome; the
  AI surface is inspect-only and never executes a fallback.
