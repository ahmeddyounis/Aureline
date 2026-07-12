# M5 Badge / Chip / Pill and Popover Controls

- Packet: `m5-badge-chip-pill-and-popover-controls:stable:0001`
- Label: `M5 badge / chip / pill and popover controls with concise text, overflow rules, plain-language expansion off color and hover, preserved lifecycle/support/provider/policy/source/freshness taxonomy, and lightweight popovers with anchored focus return across help, settings, review, marketplace, repair, and support surfaces`
- Consumer surfaces: 6
- Badge expressions: text_label, icon_with_text, count_with_label, status_word, removable_chip, color_only_disallowed
- Meaning taxonomies: lifecycle_state, support_class, provider_origin, policy_source, source_freshness, taxonomy_unclassified
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **help_ui**: `stable`
  - Owner: Help surface owner
  - Scope: The help panel expands every lifecycle badge into a plain-language explanation reachable off-hover and keeps its glossary popover dismissible with anchored focus return; both degrade honestly when meaning is color-only or focus does not return to the trigger
  - Badge examples: 3 / popover examples: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings surface keeps support-class and policy-source badges classified and reachable by keyboard and screen reader, and never lets a policy popover carry the only critical instruction; both degrade honestly when the explanation is hover-only or the popover carries the only instruction
  - Badge examples: 3 / popover examples: 2
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review sheet keeps freshness badges classified and stable across surfaces and keeps its source popover a lightweight secondary control that never traps critical steps; both degrade honestly when the taxonomy drifts or critical steps are trapped inside the popover
  - Badge examples: 2 / popover examples: 2
- **updates_ui**: `stable`
  - Owner: Marketplace / updates owner
  - Scope: The marketplace listing keeps provider-origin badges legible with concise text and a named expansion path and keeps its publisher popover dismissible; both degrade honestly when the badge label is unstated or the popover is not dismissible
  - Badge examples: 2 / popover examples: 2
- **support_ui**: `stable`
  - Owner: Repair / support surface owner
  - Scope: The repair flow keeps support-class badges classified and reachable and keeps its hint popover reachable without hover; both degrade honestly when the meaning is unclassified or the popover content is hover-only
  - Badge examples: 2 / popover examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved badge and popover truth, so a missing plain-language explanation or a disallowed popover dismissal model is visible in evidence rather than hidden behind color or hover
  - Badge examples: 2 / popover examples: 2
