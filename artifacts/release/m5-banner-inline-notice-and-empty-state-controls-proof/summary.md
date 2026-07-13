# M5 Banner / Inline-Notice and Empty-State Controls

- Packet: `m5-banner-inline-notice-and-empty-state-controls:stable:0001`
- Label: `M5 banner / inline-notice and empty-state controls with explicit scope, cause, what-still-works, primary next action, and support/help back-links, reusable empty-state cards that state purpose, current emptiness, and best next action, and shared blocked-by-policy / partial / stale / offline / restricted degraded-state variants across review, settings, update/install, support, shell, and support surfaces with no generic-something-went-wrong drift`
- Consumer surfaces: 6
- Notice scopes: page_scoped, section_scoped, field_inline, global_system, actionable_with_next_step, unscoped_color_only_disallowed
- Degraded-state variants: blocked_by_policy, partial_capability, stale_data, offline, restricted_access, variant_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review banner is page-scoped, names its cause and what still works, and exposes a primary next action, and its empty state explains its purpose; both degrade honestly when generic failure language is used
  - Banner examples: 2 / empty-state examples: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings banner is section-scoped with a blocked-by-policy variant and a support back-link, and its empty state offers a next action; both degrade honestly when the notice is unscoped / color-only or the purpose is unstated
  - Banner examples: 2 / empty-state examples: 2
- **updates_ui**: `stable`
  - Owner: Update / install owner
  - Scope: The updates banner is actionable-with-next-step with a stale-data variant, and its empty state gives first-run guidance; both degrade honestly when the degraded-state variant cannot be resolved or the best next action is missing
  - Banner examples: 2 / empty-state examples: 2
- **support_ui**: `stable`
  - Owner: Support surface owner
  - Scope: The support banner is field-inline with an offline variant and a help reference, and its empty state explains a filtered no-results view; both degrade honestly when the primary next action is missing or the emptiness reason cannot be resolved
  - Banner examples: 2 / empty-state examples: 2
- **shell_ui**: `stable`
  - Owner: Shell / entry surface owner
  - Scope: The shell banner is global-system with a restricted-access variant, and its empty state explains why it is empty now; both degrade honestly when what still works is unstated or decorative marketing filler is used
  - Banner examples: 2 / empty-state examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved banner and empty-state truth, so a screenshot-only banner or empty state is visible in evidence rather than hidden, and the reason a pane was empty or bannered can be reconstructed at capture time
  - Banner examples: 2 / empty-state examples: 2
