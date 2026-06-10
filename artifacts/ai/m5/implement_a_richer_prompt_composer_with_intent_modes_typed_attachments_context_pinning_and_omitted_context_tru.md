# Richer Prompt Composer (M5)

- Packet: `m5-richer-prompt-composer:stable:0001`
- Schema: `schemas/ai/implement-a-richer-prompt-composer-with-intent-modes-typed-attachments-context-pinning-and-omitted-context-tru.schema.json`
- Support export: `artifacts/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/support_export.json`
- Fixture: `fixtures/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/`

## Coverage

- Intent modes carry behavior constraints (review-before-apply, explicit-tool-approval, read-only-context, generated-tests-not-coverage-proof, draft-only-no-in-place-apply, requires-scoped-apply-hardening, requires-evidence-packet).
- Typed attachments cover all seven stable source classes with semantic roles and provenance chains.
- Context pinning supports manual, auto-refresh-on-change, auto-refresh-on-interval, and stale-after-duration policies.
- Omitted-context truth includes inspectability, replay-explainability, and restoration paths (one-click-restore, requires-re-review, permanently-excluded, session-only-excluded).
- Cross-surface consistency is proven for editor-attached, sidebar, and detached composers.
- Evidence lineage carries inline-stub, operator-packet, support-packet, and compliance-audit-packet classes.
- Budget strip surfaces warning/overflow with decision rows and safe fallback paths.

## Safety

The packet enforces that no apply-capable AI lane can bypass review-before-apply, skip evidence packets, hide omitted-context truth, silently drop pinned stale context, or ship without cross-surface consistency proof. Raw boundary material never crosses the export boundary.
