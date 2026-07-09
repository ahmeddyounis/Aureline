# Learning-mode toggles and tip cards

- Packet: `m5-learning-mode-toggle-tip-card-controls:stable:0001`
- Surface: `M5 learning-mode toggles and tip cards: opt-in learning state, user/workspace/feature-family scope, pause/snooze/reset actions, why-now context, and stable command/file/docs deep links across claimed onboarding and help surfaces`
- Learning-mode toggles: 6 (3 not active learning)
- Tip cards: 6 (4 not delivered)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Learning-mode toggles

- **Learning mode (this workspace)** — state `on`, scope `workspace` → `active`, deep link `command_reference`
- **Learning mode (review feature family)** — state `per_feature_family`, scope `feature_family` → `scoped_active`, deep link `command_reference`
- **Learning mode (sandbox practice)** — state `sandboxed_only`, scope `session` → `sandboxed_active`, deep link `docs_anchor`
- **Learning mode (all surfaces)** — state `paused`, scope `global` → `paused`, deep link `help_topic`
- **Learning mode (editor surface)** — state `off`, scope `surface` → `inactive`, deep link `file_location`
- **Learning mode (ended session)** — state `ended`, scope `unavailable` → `inactive`, deep link `help_topic`

## Tip cards

- **Open the exact object this references** — trigger `first_encounter`, dismissal `dismissible` → `delivered`, deep link `command_reference`
- **Jump to the next diff hunk** — trigger `feature_discovery`, dismissal `persistent_until_acted` → `delivered_persistent`, deep link `command_reference`
- **Recover from the last failed step** — trigger `error_recovery`, dismissal `snoozed` → `snoozed`, deep link `docs_anchor`
- **What changed when you switched modes** — trigger `mode_change`, dismissal `dismissed` → `withheld`, deep link `help_topic`
- **A quiet hint for when you are ready** — trigger `idle_hint`, dismissal `auto_expired` → `withheld`, deep link `docs_anchor`
- **A follow-up to your last action** — trigger `contextual_followup`, dismissal `suppressed_by_preference` → `withheld`, deep link `help_topic`
