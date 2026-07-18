# M5 collaboration-control accessibility & auto-narrowing parity (M05-1312)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5 shared terminal / debug
control-grant, presenter, consent, retention, and session-restore matrix
(`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`). Where
the freeze matrix defines the reusable **shared terminal / debug view, control grant, presenter token, consent
envelope, retention review, and session-restore view** objects, and the 1305–1311 implementation lanes resolve
their per-surface truth, this lane certifies — per object — that every control-authority, grant,
presenter-handoff, consent, retention, and restore claim survives beyond the pointer-rich desktop view and
**auto-narrows when its control-authority / active-driver / presenter-handoff / consent-scope / retention-state
/ restore-replay-safety proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_collaboration_control_accessibility_parity_and_narrowing_when_control_authority_active_driver_presenter_handoff_consent_scope_retention_or_restore_replay_evidence_is_stale/`
- **Schema:** `schemas/collaboration/m5-collaboration-control-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-collaboration-control-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/collaboration/m5-collaboration-control-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every object exposes a keyboard-complete,
   screen-reader-reachable, high-zoom-legible, high-contrast-safe, and CLI/headless-reachable path into the
   same object identity, control-authority source, single active driver, presenter holder / handoff chain,
   join-time consent scope, recording / retention state, and restore transcript class the rich object shows —
   never a color-only control badge, a hover-only active-driver pill, or a pointer-only grant affordance. The
   support / release / CLI export reconstructs each object's meaning from typed tokens and opaque refs
   **without a raw payload**, preserving the same control-authority, active-driver, presenter-handoff,
   consent-scope, retention-state, and restore-replay-safety labels visible in-product.

2. **Honest auto-narrowing.** When a shared terminal / debug view's control authority is unresolved or
   presence-implied, a control grant's single active driver is unprovable or contended, a presenter token's
   handoff is unprovable or contested, a consent envelope's join-time scope is undisclosed or would widen
   silently, a retention review's recording / retention state is stale or would broaden silently, or a
   session-restore view's replay-free restore safety is unprovable, the claim auto-narrows from
   `explicitly_granted_control_surface` / `view_first_observable_surface` to the matching projection, discloses
   the narrowing with a precise trigger and binding dimension, and preserves the canonical identity /
   last-known state. An object with every dimension intact must **not** carry a spurious narrowing, and a
   weakened object can never keep a fully explicitly-granted, single-driver controlled claim — presence never
   masquerades as control, no second active driver is shown on a sensitive surface, and no prior terminal /
   debug input is replayed on join or restore.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the shared terminal / debug view,
   collaboration join-review sheet, control-grant prompt, presenter-handoff sheet, paste / secret guard,
   collaboration retention sheet, session-restore view, support / export packet, and help / docs so product,
   help, and release publication stay aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `explicitly_granted_control_surface` | Fully control-authority-bound, single-active-driver, consent-disclosed, retention-bound, replay-free — safe to observe, request control, export, and restore. |
| `view_first_observable_surface` | Self-sufficient, view-first read-only object (a session-restore view a user can observe), not a mutating control-driving surface. |
| `control_authority_unverified_projection` | The shared terminal / debug view's control authority is unresolved or presence-implied (shared-terminal-debug-view). |
| `active_driver_unverified_projection` | The control grant's single active driver is unprovable or contended (control-grant). |
| `presenter_handoff_unverified_projection` | The presenter token's handoff is unprovable or contested (presenter-token). |
| `consent_scope_unverified_projection` | The consent envelope's join-time scope is undisclosed or would widen silently (consent-envelope). |
| `retention_state_unverified_projection` | The retention review's recording / retention state is stale or would broaden silently (retention-review). |
| `restore_replay_safety_unverified_projection` | The session-restore view's replay-free restore safety is unprovable (session-restore-view). |

## Weakening dimensions and their frozen triggers

Each object maps to a claim dimension; a weak condition state narrows to the matching projection and names
the on-topic frozen matrix downgrade trigger:

| Dimension (object) | Weak condition | Frozen trigger | Cannot be shown controlled |
| --- | --- | --- | --- |
| `control_authority_clarity` (shared-terminal-debug-view) | `control_authority_unresolved_or_presence_implied` | `control_authority_unstated` | yes |
| `active_driver_clarity` (control-grant) | `active_driver_unprovable_or_multi_driver` | `active_driver_unstated` | yes |
| `presenter_handoff_clarity` (presenter-token) | `presenter_handoff_unprovable_or_contested` | `view_first_default_unstated` | yes |
| `consent_scope_clarity` (consent-envelope) | `consent_scope_undisclosed_or_widened` | `consent_scope_unstated` | yes |
| `retention_state_clarity` (retention-review) | `retention_state_stale_or_broadened_silently` | `retention_state_unstated` | yes |
| `restore_replay_safety_clarity` (session-restore-view) | `restore_replay_safety_unprovable` | `restore_replay_safety_unstated` | yes |

Every weak collaboration-control condition is a genuine truth degradation, so all six flag as
`cannot_be_shown_trusted`: none may keep a fully explicitly-granted, single-driver controlled claim.

## Structure-heavy objects

The **presenter token** (presenter holder / handoff chain / moderation scope) and **session-restore view**
(restore transcript class / replay-free render summary / retention scope set) render a dense structured
surface, so they must additionally bind their structured layout to an equivalent flat list / textual path (a
`structured` fallback modality **plus** a non-visual list / textual / CLI path).

## Certified rows

Eight rows across the six objects: **2 green** (the control-authority-bound shared terminal / debug view —
explicitly granted; and the replay-free-bound session-restore view — view-first observable) and **6 yellow** —
one per spec narrowing axis (control authority, active driver, presenter handoff, consent scope, retention
state, restore replay safety), each auto-narrowing to its permitted projection. **No red rows may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_COLLABORATION_CONTROL_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_collaboration_control_accessibility_parity_and_narrowing_when_control_authority_active_driver_presenter_handoff_consent_scope_retention_or_restore_replay_evidence_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
