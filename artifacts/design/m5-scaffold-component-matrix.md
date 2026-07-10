# M5 Scaffold-Template-Card, Starter-Parameter-Row, Scaffold-Preflight-Card, Template-Health-Row, Generated-Project-Diff-Card, and Scaffold-Handoff-Banner Component Matrix

- Packet: `m5-scaffold-components:stable:0001`
- Label: `M5 scaffold-template-card, starter-parameter-row, scaffold-preflight-card, template-health-row, generated-project-diff-card, and scaffold-handoff-banner component matrix`
- Component families: 6 (6 stable)
- Dispositions: first_party, team_managed, community, local_only, create_empty, continue_without_starter, blocked, warning, optional
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Component families

- **scaffold_template_card**: `stable`
  - Owner: Scaffold template card owner
  - Scope: One scaffold-template-card model naming where a starter comes from (a first-party starter, a team-managed starter, a community starter, a local-only starter, a mirrored starter, or an unknown source) and how it is supported (officially supported, community supported, experimental, bridge behavior, deprecated, or unsupported), so a card never leaves its starter source or support class implicit and never presents bridge or heuristic behavior as exact first-party support
  - Dispositions: first_party, team_managed, community, local_only
  - Required labels: identity, state, keyboard_route, starter_source_and_support
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **starter_parameter_row**: `stable`
  - Owner: Starter parameter row owner
  - Scope: One starter-parameter-row model naming where a parameter value comes from (a default value, a user-provided value, a profile-inherited value, an environment-derived value, a computed value, or an unset required value) and whether its action is applied immediately or deferred (applied immediately, deferred after create, requires confirmation, blocked because invalid, optional and skippable, or not applicable), so a row never leaves the parameter source layer or the immediate-versus-deferred boundary implicit
  - Dispositions: optional, warning, blocked
  - Required labels: identity, state, keyboard_route, side_effect_disclosure
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **scaffold_preflight_card**: `stable`
  - Owner: Scaffold preflight card owner
  - Scope: One scaffold-preflight-card model naming what is checked before a starter writes files (required tooling present, dependency availability, network access, workspace writable, the host or managed-workspace boundary, or the credential scope) and each check's outcome (passed, warning, blocked, skipped because optional, not run, or unknown), so a generic Create never hides a network, dependency-install, remote-provisioning, trust, or managed-workspace side effect
  - Dispositions: blocked, warning, optional
  - Required labels: identity, state, keyboard_route, side_effect_disclosure
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **template_health_row**: `stable`
  - Owner: Template health row owner
  - Scope: One template-health-row model naming which health facet it reports (build health, dependency freshness, security advisories, test status, maintenance cadence, or compatibility) and how current the signal is (fresh, aging, stale, expired, never checked, or unavailable), so a row never presents a stale or never-checked health signal as fresh and always names what it is asserting
  - Dispositions: warning, optional
  - Required labels: identity, state, keyboard_route, starter_source_and_support
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **generated_project_diff_card**: `stable`
  - Owner: Generated-project diff card owner
  - Scope: One generated-project-diff-card model naming what a starter wrote versus what the user owns (generated only, user-owned, generated then hand-edited, runtime-only, a mixed zone, or zone unknown) and its diff-review state (a preview ready, review required before any write, no changes, a conflict detected, the diff unavailable, or blocked), so a card never blurs the generated-versus-user-owned boundary and never overwrites or deletes user-owned work silently
  - Dispositions: create_empty, continue_without_starter, blocked
  - Required labels: identity, state, keyboard_route, recovery_and_ownership_boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **scaffold_handoff_banner**: `stable`
  - Owner: Scaffold handoff banner owner
  - Scope: One scaffold-handoff-banner model naming the bootstrap outcome (create succeeded, a partial bootstrap, create failed, continued without a starter, created empty, or remote provisioning pending) and the recovery path it keeps explicit (open the workspace, retry the bootstrap, delete the generated output, continue without the starter, keep the partial output for review, or no recovery needed), so a partial or failed bootstrap is never presented as a clean create and delete-generated or continue-without-starter recovery is never hidden
  - Dispositions: create_empty, continue_without_starter, first_party
  - Required labels: identity, state, keyboard_route, recovery_and_ownership_boundary, side_effect_disclosure
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
