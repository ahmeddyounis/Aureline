# M5 prompt-composer-component accessibility & auto-narrowing capstone contract

M05-890 is the B104 closing keyboard / screen-reader / CLI / export parity and
automatic-narrowing capstone over the frozen M5 prompt-composer component matrix
(`freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`).
Where the freeze matrix defines the reusable prompt-composer header, context-attachment
pill, mention resolver, slash-command row, budget / size strip, tainted-context warning,
draft-state row, attachment-stale banner, and split-send / review control primitives, and
the 885–888 lanes resolve their per-surface truth, this lane certifies — per component
family — that pre-send composition claims stay keyboard-complete, assistive-tech-reachable,
CLI/export-safe, and self-narrowing.

## Packet

- Packet type: `ComposerComponentAccessibilityPacket`, one
  `ComposerComponentAccessibilityRow` per frozen
  `M5PromptComposerComponentFamily` (9 families, 1:1).
- Builder: `seeded_m5_composer_component_a11y_fallback_packet()`; reader for the checked-in
  export: `current_m5_composer_component_a11y_fallback_export()`.
- The packet is metadata-only: typed class tokens, opaque summary / evidence refs,
  booleans, and redacted labels — never raw prompt bodies, attachment contents, provider
  credentials, or pasted external text.

## Reused frozen vocabulary

The capstone certifies the freeze matrix's own vocabulary rather than minting parallel
synonyms:

- `M5PromptComposerComponentFamily` (9 families) — the row key.
- `M5ComposerRequiredLabel` (mandatory `identity` / `state` / `keyboard_route`).
- `M5ComposerDowngradeTrigger` (12 frozen triggers) — the narrowing reason.
- `M5ComposerConsumerSurface` (9 consumer surfaces) — cross-surface disclosure coverage.

## Minted capstone vocabulary

- `M5ComposerSupportClaim` (6, strongest→weakest): `ready_to_send`,
  `reviewable_composition`, `narrowed_composition`, `local_only_composition`,
  `unresolved_composition`, `policy_blocked_composition`.
- `M5ComposerClaimDimension` (9, 1:1 with families): `route_readiness`,
  `attachment_trust`, `mention_resolution`, `command_availability`, `budget_headroom`,
  `context_taint`, `draft_locality`, `attachment_freshness`, `send_gate`. The five spec
  auto-narrow axes are `mention_resolution` (unresolved mentions),
  `attachment_freshness` (stale attachments), `budget_headroom` (over-budget
  composition), `draft_locality` (offline-local-only fallbacks), and `route_readiness`
  (policy-blocked routes).
- `M5ComposerConditionState` (5): `composed`, `narrowed_in_scope`, `local_only`,
  `unresolved`, `blocked`; each maps 1:1 to a permitted support-claim ceiling.
- `M5ComposerFallbackModality` (4), `M5ComposerRenderingSurface` (6),
  `ComposerNonVisualReachState` (3), `ComposerExportSummaryState` (3),
  `ComposerNarrowingDisclosureState` (3), `ComposerComponentAccessibilityStatus` (3).

## Auto-narrowing honesty

When any composer dimension weakens below the family's full claim, the component's support
claim auto-narrows to the permitted ceiling, names the ceiling-imposing dimension and its
frozen downgrade trigger, carries a precise non-generic label, and preserves both the
canonical composer / attachment / mention / route identity and the draft. A component with
every dimension intact must not carry a spurious narrowing. Each dimension names an
on-topic frozen trigger so the certified reason stays byte-identical to the matrix.

## Acceptance-criterion coverage

- **Accessibility and export parity are green on every claimed M5 surface.** Each row
  binds a keyboard / screen-reader / CLI reach state, a hierarchy-heavy family (the budget
  / size strip's omitted-context drawer) additionally binds a non-visual fallback, and the
  export reconstructs meaning without a screenshot (text / JSON / Markdown copy formats).
- **Blocked, stale, unresolved, and over-budget states narrow explicitly and preserve
  draft integrity.** `draft_preserved` and every narrow block's `preserves_draft_integrity`
  must hold; a dropped draft or a narrow that discards the draft strands (reds) the row.
- **Release and support exports reconstruct pre-send composition truth without
  screenshots or private team memory.** The support export, matrix CSV, and Markdown report
  carry the full mode / route / attachment / taint / budget / send-gate truth per family.

## Boundary

Validation (`ComposerComponentAccessibilityPacket::validate`) enforces: family / dimension
/ claim-tier / consumer-surface coverage, per-row completeness and mandatory labels,
hierarchy-heavy structured modality, claim honesty, assistive-tech reach, export meaning,
draft integrity, narrowing disclosure, ≥2-consumer parity, no stranded (red) rows, summary
agreement, and no raw composer material in the export
(`RawComposerMaterialInExport`).

## Source contracts

- Boundary schema: `schemas/ai/m5-prompt-composer-component-accessibility-fallback.schema.json`.
- Frozen component matrix: `schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json`.
- Prompt-composer draft: `schemas/ai/prompt_composer_draft.schema.json`.
- Prompt-context attachment: `schemas/ai/prompt_context_attachment.schema.json`.
- Tainted context: `schemas/ai/tainted_context.schema.json`.
- Context assembly: `schemas/ai/context_assembly.schema.json`.

## Artifacts

- Support export: `artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback/support_export.json`.
- Matrix CSV: same directory, `matrix.csv`.
- Markdown report: `artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback.md`.
- Fixtures: `fixtures/ai/m5/m5-prompt-composer-component-accessibility-fallback/`.
- Regenerate with `GEN_COMPOSER_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-ai generate_artifacts`.
