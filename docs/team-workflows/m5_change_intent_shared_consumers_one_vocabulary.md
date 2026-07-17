# M5 Change-Intent Shared Consumers: One Vocabulary Across Surfaces

This lane is the B153 consumer-adoption capstone. It binds the six governed change-intent lifecycle objects
frozen by the [`m5_change_intent_and_engineering_lifecycle_matrix`](m5-change-intent-lifecycle-ops.md) — the
**change-intent record**, the **start-work sheet**, the **linked-change panel**, the **ready-for-review
handoff sheet**, the **resolve-or-close sheet**, and the **blocked-or-escalate card** — to the concrete
consumers that render them, and proves, by fixtures rather than screenshots, that the same seeded tracked-item
change-intent subject carries one identical vocabulary wherever Aureline starts work, links a change, packages
a handoff, retries a publish, resolves a ticket, or escalates a blocker.

- Rust module: `aureline_ui::m5_change_intent_shared_consumers_one_vocabulary_across_surfaces`
- Boundary schema: [`schemas/teamwork/m5-change-intent-shared-consumers.schema.json`](../../schemas/teamwork/m5-change-intent-shared-consumers.schema.json)
- Proof bundle: `artifacts/release/m5-change-intent-shared-consumers-proof/` (`support_export.json`, `matrix.csv`, `summary.md`)
- Fixtures: `fixtures/teamwork/m5-change-intent-shared-consumers/` (`compact_remote_narrowed.json`, `exported_redaction_narrowed.json`)
- Emitter: `cargo run -p aureline-ui --example dump_m5_change_intent_shared_consumers -- <support-export|report|csv|fixture-compact-remote-narrowed|fixture-exported-redaction-narrowed|validate>`

## Consumers

Nine shared consumer surfaces adopt the change-intent vocabulary: work-item detail, the start-work sheet, the
linked-change panel, review detail, the ready-for-review handoff, the resolve-or-close sheet, the
blocked-or-escalate card, the support / export packet, and the help / docs surface. Each of the six objects is
adopted by at least two distinct consumers, so an object is proven to be shared change-intent infrastructure
rather than a one-surface fork that invents its own relation labels.

## One vocabulary, no drift

For a given seeded tracked-item subject, every consumer surface must present identical
`ChangeIntentSharedStateFacetValues`: the same change-intent-role word, object word, registry-reference word,
commit-state word, surface-context word, and relation-source word. The change-intent-role word must be a token
from the frozen `M5ChangeIntentRole` vocabulary (`provider_ownership_disclosure`,
`local_versus_provider_state_disclosure`, `linked_engineering_identity_disclosure`, `side_effect_disclosure`,
`validation_evidence_disclosure`, `publish_later_fallback_disclosure`, `final_resolution_authority_disclosure`),
so no surface invents an alternate label for provider ownership, local-versus-provider commit state, or a stale
or broken relation.

A role that carries provider-ownership, local-versus-provider-state, linked-engineering-identity, or
side-effect meaning is a **gate role**: it must pair its surface presentation with a real
`relation_source_disclosed_and_commit_state_bound` continuity and never collapse to a masquerade sentinel
(`stale_relation_shown_as_provider_linked`, `suggested_relation_shown_as_provider_linked`,
`local_draft_shown_as_provider_committed`, `queued_publish_shown_as_provider_committed`).

## Narrowing is disclosed

A surface may narrow *how much* it renders across the desktop-full, compact, remote-projected, and
exported-redacted representations, but never reword the vocabulary. Every narrowed representation carries an
explicit `ChangeIntentSharedNarrowNote` naming the reason, the preserved vocabulary, and the next action;
remote and exported forms additionally name their remote-source and export-safe-detail boundaries.

## Map back to one object

Support / export consumers point at the canonical per-domain schema and the frozen matrix by id, so an exported
packet — and every copy / export / open-in-provider action — maps back to one shared contract object rather
than diverging into a surface-local payload or collapsing stable relation / source labels to generic prose.

## Guardrails

Each binding re-asserts the matrix's five hard invariants (all MUST be `false`): it never lets a local handoff
packet or queued publish masquerade as a provider-committed update, never silently creates a branch, worktree,
review draft, or provider link without separate disclosure, never flattens linked-by-provider, linked-locally,
suggested-by-Aureline, and stale-or-broken into one relation badge, never auto-resolves tracked work while
engineering blockers remain unresolved, and never drops local notes, handoff packets, or linked evidence when
a provider write fails.

## Acceptance criteria mapping

1. **The same tracked item shows the same change-intent state across work-item, review, Git, AI, help, and
   support entry points** — enforced by the per-subject facet identity and the
   `change_intent_vocabulary_drift_across_surfaces` violation over nine shared consumer surfaces.
2. **Exported packets and reopened views retain stable relation / source labels instead of collapsing to
   generic prose** — enforced by the frozen-role-token gate (`change_intent_role_word_outside_vocabulary`)
   over the shared commit-state / relation-source words, `points_at_canonical_contracts`, the
   `support_export_reference_missing` violation, and the `relation_source_missing_for_gate_role` gate.
