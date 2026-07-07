# M5 Contextual-Tip-Card Primitive

- Packet: `m5-contextual-tip-card-primitive:stable:0001`
- Label: `M5 contextual-tip-card primitive: tip trigger class, command-backing state, dismissal state, why-now relevance, concrete next action, stable command reference, derived delivery posture (delivered-actionable/delivered-informational/snoozed-for-later/withheld-for-quiet-hours/withheld-for-presentation-mode/withheld-already-resolved), and bounded try/request-approval/open-docs/snooze/dismiss actions`
- Learnability consumers: 5 (5 stable)
- Delivery postures: delivered_actionable, delivered_informational, snoozed_for_later, withheld_for_quiet_hours, withheld_for_presentation_mode, withheld_already_resolved
- Tip actions: try_next_action, request_approval, open_docs, snooze_tip, dismiss_tip
- Trigger classes: first_encounter, feature_discovery, error_recovery, mode_change, idle_hint, contextual_followup
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Learnability consumers

- **First-Run Onboarding Panel**: `stable`
  - Owner: First-run onboarding panel owner
  - Scope: The first-run onboarding panel renders the shared contextual tip card so a first-encounter tip whose concrete next action is a bound command is delivered in place as an actionable tip that can be tried, snoozed, or dismissed without leaving the task, and an idle-time hint with no command backing is delivered as an informational tip that still teaches and stays reversible — never a blocking tour
  - Worked tips: 2
    - `tip:onboarding:command-palette` (`first_encounter` / `dismissible`) → `delivered_actionable` (command-backed `true`, delivered `true`, approval `false`)
    - `tip:onboarding:activity-bar-orientation` (`idle_hint` / `dismissible`) → `delivered_informational` (command-backed `false`, delivered `true`, approval `false`)
- **Guided-Tour Overlay**: `stable`
  - Owner: Guided-tour overlay owner
  - Scope: The guided-tour overlay renders the shared contextual tip card so a feature-discovery tip whose underlying action requires approval is delivered as an actionable tip that offers request-approval rather than running the action directly — never bypassing the trust boundary — and a mode-change tip the user snoozed stays snoozed for later while remaining permanently dismissible
  - Worked tips: 2
    - `tip:tour:workspace-share` (`feature_discovery` / `dismissible`) → `delivered_actionable` (command-backed `true`, delivered `true`, approval `true`)
    - `tip:tour:learning-mode-layout` (`mode_change` / `snoozed`) → `snoozed_for_later` (command-backed `true`, delivered `false`, approval `false`)
- **Command-Palette Hint**: `stable`
  - Owner: Command-palette hint owner
  - Scope: The command-palette hint renders the shared contextual tip card so a contextual follow-up tip is withheld while quiet hours are active — respecting the do-not-disturb window rather than interrupting — and an error-recovery tip bound to a command and persistent until acted is delivered as an actionable tip that names the exact recovery command and stays reversible
  - Worked tips: 2
    - `tip:palette:search-refine` (`contextual_followup` / `dismissible`) → `withheld_for_quiet_hours` (command-backed `true`, delivered `false`, approval `false`)
    - `tip:palette:save-and-retry` (`error_recovery` / `persistent_until_acted`) → `delivered_actionable` (command-backed `true`, delivered `true`, approval `false`)
- **Inline Editor Tip**: `stable`
  - Owner: Inline editor tip owner
  - Scope: The inline editor tip renders the shared contextual tip card so a mode-change tip is withheld while presentation mode is active — never interrupting a live demo or screen share — and an idle hint the user already dismissed is withheld as already resolved so it is never re-shown, keeping tips non-spammy
  - Worked tips: 2
    - `tip:inline:zen-minimap` (`mode_change` / `dismissible`) → `withheld_for_presentation_mode` (command-backed `true`, delivered `false`, approval `false`)
    - `tip:inline:multi-cursor` (`idle_hint` / `dismissed`) → `withheld_already_resolved` (command-backed `true`, delivered `false`, approval `false`)
- **Support Tip Export**: `stable`
  - Owner: Support tip export owner
  - Scope: The support tip export renders the shared contextual tip card so a first-encounter tip is withheld because a like tip was recently dismissed — proving the non-spammy guard survives export — and a contextual follow-up tip bound to a deep-link command is delivered as an actionable tip whose export reconstructs its why-now relevance, command reference, and delivery posture without leaking raw docs bodies
  - Worked tips: 2
    - `tip:support:keyboard-shortcuts` (`first_encounter` / `dismissible`) → `withheld_already_resolved` (command-backed `true`, delivered `false`, approval `false`)
    - `tip:support:migration-report` (`contextual_followup` / `dismissible`) → `delivered_actionable` (command-backed `true`, delivered `true`, approval `false`)
