# M5 adaptive-efficiency component accessibility parity (M05-1065)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5
adaptive-efficiency component matrix (power-state indicator, throttled-subsystem row,
background-work row / banner, per-workspace override sheet, override-policy note row,
resume-summary card, stale-result continuity note). Where the freeze matrix defines the
reusable primitives and the 1061–1064 implementation lanes resolve their per-surface
truth, this lane certifies — per component family — that adaptive-efficiency claims stay
**keyboard-complete, screen-reader-reachable, reduced-motion safe, high-contrast legible,
CLI/export-safe, and self-narrowing**.

- **Module:** `crates/aureline-shell/src/implement_keyboard_screen_reader_reduced_motion_high_contrast_cli_export_and_support_packet_parity_and_adaptive_efficiency_component_claim_auto_narrowing/`
- **Schema:** `schemas/ui/m5-efficiency-component-accessibility-parity.schema.json`
- **Proof artifacts:** `artifacts/release/m5-efficiency-component-accessibility-proof/`
- **Fixtures:** `fixtures/ui/m5-efficiency-component-accessibility-parity/`

## What it certifies

Each `EfficiencyAccessibilityRow` keys on one frozen `M5EfficiencyComponentFamily` and
reuses that frozen family vocabulary plus the frozen `M5EfficiencyRequiredLabel`,
`M5EfficiencyDowngradeTrigger`, and `M5EfficiencyConsumerSurface` sets rather than minting
parallel synonyms.

- **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and CLI/headless-reachable path into the same source-of-change,
  active efficiency state, slowed-versus-paused work, override availability, policy owner,
  resumed-work backlog, and stale-result continuity the rich surface shows — never a
  hover-only or toast-only card that strands assistive-tech or headless users.
  Hierarchy-heavy families (the per-workspace override sheet's nested current-mode /
  allowed-ceiling tree) additionally bind their tree to a flat list / textual path.
- **Export parity.** The support / release export reconstructs each component's meaning
  from typed tokens and opaque refs without a screenshot.
- **Honest auto-narrowing (AC1).** When an efficiency dimension weakens, the component's
  efficiency-support claim auto-narrows from `full_truth` / `resolved_truth` to `degraded`
  / `deferred` / `stale_shown` / `policy_blocked`, discloses the narrowing with a precise
  frozen trigger and binding dimension, and preserves the canonical identity rather than
  silently dropping it. A component with every dimension intact must NOT carry a spurious
  narrowing.
- **Cross-surface disclosure (AC3).** The same narrowed state surfaces in shell status
  chrome, the activity center, docs/help, headless CLI, and support/admin exports.

## Claim ceilings

| Condition state | Permitted efficiency-support claim |
| --------------- | ---------------------------------- |
| `intact`        | `full_truth`                       |
| `partial`       | `degraded`                         |
| `deferred`      | `deferred`                         |
| `stale_shown`   | `stale_shown`                      |
| `policy_blocked`| `policy_blocked`                   |

A weakened dimension can never keep an old `full_truth` label; the effective claim is
capped at the strongest permitted ceiling across all modeled dimensions.

## Weakening dimensions → frozen triggers

| Dimension                        | Frozen downgrade trigger          | Primary family                        |
| -------------------------------- | --------------------------------- | ------------------------------------- |
| `pressure_source_truth`          | `source_of_change_unstated`       | power-state indicator                 |
| `work_disposition_truth`         | `slowed_versus_paused_ambiguous`  | throttled row / background row+banner |
| `override_availability_truth`    | `override_availability_unstated`  | per-workspace override sheet          |
| `policy_owner_truth`             | `policy_owner_unstated`           | override-policy note row              |
| `resume_backlog_truth`           | `resume_backlog_hidden`           | resume-summary card                   |
| `stale_result_continuity_truth`  | `stale_result_continuity_cleared` | stale-result continuity note          |

## Guardrails held

- Battery saver, thermal pressure, user-selected low-power mode, and policy cap are not
  collapsed into one generic warning — generic low-power wording is rejected as a narrowed
  label.
- Paused work is never hidden behind toast-only messaging — a `view_only_trap` reach state
  strands (reds) the row.
- An override is never presented as available when policy blocks it — the override sheet
  narrows to `policy_blocked`.
- Stale-result context is never cleared merely because background work resumed — the
  stale-result note narrows to `stale_shown` and preserves canonical continuity.

## Acceptance criteria

- **AC1:** Accessibility and export reviews recover the same adaptive-efficiency truth the
  desktop shell shows interactively.
- **AC2:** No claimed M5 profile loses cause / state / override context when rendered in
  high-contrast, reduced-motion, CLI, or support-export form.

## Regenerating artifacts

```
GEN_EFFICIENCY_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts
```
