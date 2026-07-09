# M5 Learning-Mode-Toggle, Tip-Card, Guided-Exercise-Step, Glossary-Chip-or-Card, Safe-Explanation-Banner, and Progress-Marker Component Matrix

- Packet: `m5-learning-components:stable:0001`
- Label: `M5 learning-mode-toggle, tip-card, guided-exercise-step, glossary-chip-or-card, safe-explanation-banner, and progress-marker component matrix`
- Component families: 6 (6 stable)
- Dispositions: learning_on, paused, replayable, sandboxed, cached, local_only, not_installed, no_hidden_apply
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Component families

- **learning_mode_toggle**: `stable`
  - Owner: Learning-mode toggle owner
  - Scope: One learning-mode-toggle model naming whether learning mode is on, off, paused, per feature family, sandboxed-only, or ended and how widely it applies (global, per workspace, per feature family, per session, per surface, or unavailable), so learning stays opt-in, never traps an expert in a tutorial, and its cached, local-only, and not-installed states stay visible
  - Dispositions: learning_on, paused, local_only, not_installed
  - Required labels: identity, state, keyboard_route, progress_ownership_and_privacy
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **tip_card**: `stable`
  - Owner: Tip card owner
  - Scope: One tip-card model naming why a teaching tip appears (first encounter, feature discovery, error recovery, mode change, idle hint, or contextual follow-up), the cited source behind it, and how it can be dismissed, so teaching stays contextual, dismissible, and citation-backed and never blocks the user or drifts from cited source truth
  - Dispositions: learning_on, cached
  - Required labels: identity, state, keyboard_route, citation_source
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **guided_exercise_step**: `stable`
  - Owner: Guided exercise step owner
  - Scope: One guided-exercise-step model naming the state of a practice step (not started, active, passed, failed but retryable, replayable, or sandboxed) and how it validates the learner's work (command-backed, sandboxed practice, read-only walkthrough, checkpoint-gated, self-paced, or no hidden apply), so an exercise is replayable, keeps explain and do separate, and never mutates live state without the ordinary preview and approval model
  - Dispositions: replayable, sandboxed, no_hidden_apply
  - Required labels: identity, state, keyboard_route, explain_versus_do_boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **glossary_chip_or_card**: `stable`
  - Owner: Glossary chip or card owner
  - Scope: One glossary-chip-or-card model naming where a definition comes from (cited docs, cited spec, cited help pack, a community note, an uncited draft, or an unknown source) and how current its citation is (current, version-matched, stale, cached, offline-unavailable, or missing), so glossary prose never drifts away from cited source truth and a definition never severs or hides its canonical citation
  - Dispositions: cached, local_only
  - Required labels: identity, state, keyboard_route, citation_source
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **safe_explanation_banner**: `stable`
  - Owner: Safe explanation banner owner
  - Scope: One safe-explanation-banner model naming how an explanation separates explain from do (explain only, explain then offer to do, preview required, approval required, sandboxed only, or no hidden apply) and what it will actually do (apply nothing, preview available, approval pending, applied with undo, blocked apply, or mutation declined), so an educational explanation never widens mutating authority and applies nothing without the same preview and approval model as ordinary work
  - Dispositions: no_hidden_apply, sandboxed
  - Required labels: identity, state, keyboard_route, explain_versus_do_boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **progress_marker**: `stable`
  - Owner: Progress marker owner
  - Scope: One progress-marker model naming who owns a learner's progress (local-only, user-owned and synced by choice, exported by choice, workspace-shared by choice, a cached snapshot, or not installed) and where that progress stands (not started, in progress, completed, paused, reset, or offline / local), so progress stays user-owned and default-local unless a supported sync or export path is explicitly chosen and an offline or local-only state is never left implicit
  - Dispositions: local_only, cached, not_installed, paused
  - Required labels: identity, state, keyboard_route, progress_ownership_and_privacy
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
