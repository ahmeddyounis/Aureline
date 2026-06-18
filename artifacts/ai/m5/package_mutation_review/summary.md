# AI package-mutation review — support export summary

This support export is the AI composer's propose-only view of governed package
mutations. It is produced by the `aureline-ai` crate
(`package_mutation_review`) and binds by reference to the cross-surface
governance contract in `aureline-deps`
(`automation_governance`, `automation-governance:m5:v1`), the frozen
package-state matrix (`m5-package-state-mutation-matrix:m5:v1`), and the
reviewed-mutation contract (`reviewed-mutation-flows:m5:v1`).

Every AI proposal is **propose-only or inspect-only** — the AI surface never
executes a mutation. Each proposal is preview-first, routes through governed
review, carries no hidden scripting, requests the same validation tasks as a
direct operation, and mirrors the governed safe-fallback decision and result
class so AI convenience never becomes a bypass lane.

The export covers all four mutation intents (add, upgrade, remove, relock), both
AI write authorities (propose-only, inspect-only), every safe-fallback class
(proceed, narrow-to-inspect, narrow-to-export, browser/CLI handoff, blocked),
and every result class (preview-pending, reviewed-ready, narrowed-inspect-only,
handed-off, blocked-unsafe, committed-reviewed, rolled-back).

Schema: `schemas/ai/package-mutation-review.schema.json`.
Doc: `docs/ai/m5/package-mutation-review.md`.
