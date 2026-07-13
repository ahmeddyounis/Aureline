# M5 Typography-Scale, Font-Stack, and Text-Overflow Registries

- Packet: `m5-typography-scale-font-stack-and-overflow-registries:stable:0001`
- Label: `M5 typography-scale, font-stack, and text-overflow registries with a canonical title / body / label / code hierarchy, stable UI-sans and code-mono stack selection, line-height guards, tabular numerals for counts / timings / diagnostics, and meaning-preserving overflow / truncation / wrap behavior across the shell, editor, review, docs, data, and support surfaces`
- Consumer surfaces: 6
- Text roles: title, body, label, code, numeric_data, role_unknown
- Font stacks: ui_sans_stack, code_mono_stack, local_font_stack_disallowed, stack_unknown
- Overflow treatments: truncate_with_tooltip, wrap_to_next_line, ellipsis_with_reveal, horizontal_scroll, silent_clip_disallowed, treatment_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell renders titles on the canonical display scale and sans stack with a stated case rule, and ellipsizes tab labels with a reveal that survives zoom; an unstated case rule and a zoom regression degrade honestly instead of reading as a clean pass
  - Type scale: 2 / overflow: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor renders code on the monospace stack and scrolls code-adjacent metadata so the full path stays reachable; a code role that selects the UI stack and an unreachable truncation degrade honestly
  - Type scale: 2 / overflow: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface renders body text on the shared body scale and wraps banner notices so meaning survives; a line-height drift and a density regression degrade honestly
  - Type scale: 2 / overflow: 2
- **docs_ui**: `stable`
  - Owner: Docs surface owner
  - Scope: The docs surface renders labels on the shared scale and truncates inspector fields with a tooltip that carries the full text, so type hierarchy and overflow behavior stay stable when the page is exported
  - Type scale: 1 / overflow: 1
- **data_ui**: `stable`
  - Owner: Data surface owner
  - Scope: The dense data surface renders counts / timings on tabular numerals and truncates rows with a tooltip at compact density; missing tabular numerals and a silent clip that destroys meaning degrade honestly
  - Type scale: 2 / overflow: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved type-scale and overflow truth, so a raw-value regression, an unstated token or role, and a raw-layout overflow are visible in evidence rather than hidden behind rendering
  - Type scale: 3 / overflow: 1
