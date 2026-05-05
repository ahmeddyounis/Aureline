# Motion timing, easing, and semantic-equivalence contract

This document freezes Aureline’s **motion duration/easing tokens** and the
**semantic-equivalence rules** that keep state meaning intact when motion is
reduced, suppressed, or simplified (reduced-motion preferences, low-motion
setting, power-saver posture, and critical hot-path suppression).

Motion exists to clarify **origin, continuity, hierarchy, freshness, and
completion**. It must never become ambient decoration, and it must never become
the only carrier of state, risk, or completion.

This contract is normative. Where it disagrees with the PRD, technical
architecture/design documents, UI/UX spec, or the UX design-system style guide,
those sources win and this contract plus its companion artifacts MUST be
updated in the same change.

## Companion artifacts

- [`/artifacts/design/motion_tokens.yaml`](../../artifacts/design/motion_tokens.yaml)
  publishes the frozen duration/easing values and the role-level defaults.
- [`/schemas/design/motion_transition.schema.json`](../../schemas/design/motion_transition.schema.json)
  defines the boundary shape for the token ledger and motion-case fixtures.
- [`/fixtures/design/motion_cases/`](../../fixtures/design/motion_cases/)
  contains worked motion presets demonstrating semantic-equivalent defaults and
  low-motion fallbacks for common surfaces.
- [`/schemas/ux/motion_preset.schema.json`](../../schemas/ux/motion_preset.schema.json)
  defines the machine-readable `motion_preset_record` shape used by the motion
  cases.

## Composition, not duplication

This contract composes with existing canonical sources:

- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` owns the human-facing
  motion system tables and principles.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` owns motion intent and restrictions
  (“motion never competes with the editor”, reduced-motion behavior, and
  no-layout-shift expectations in dense panes).
- [`/docs/ux/theme_and_visual_asset_contract.md`](../ux/theme_and_visual_asset_contract.md)
  defines `motion_preset_record` semantics, reduced-motion substitution classes,
  and the safety-critical override boundaries.
- [`/docs/accessibility/visual_adaptation_contract.md`](../accessibility/visual_adaptation_contract.md)
  defines visual-mode adaptation expectations that must remain true under motion
  suppression (toasts/overlays/progress included).
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  freezes the accessibility posture ladder (`motion_standard`, `motion_reduced`,
  `motion_low_motion`, `motion_power_saver`, `motion_critical_hot_path`) and the
  motion-family suppression vocabulary.

## 1. Scope

Frozen here:

- the canonical **duration** tokens (`motion.*`) and **easing** tokens
  (`ease.*`) that all surfaces cite;
- the rule that **semantic meaning survives** when motion is reduced or removed;
- transition guardrails: **no motion-only state**, **no focus theft**, **no
  layout shifts during typing**, and **interruptible/cancelable transitions**;
- worked motion presets for:
  overlays, banners, toasts, durable-job progress, guided tours, and focus-follow
  transitions.

Out of scope:

- bespoke launch-polish animation per feature surface;
- long decorative sequences or cinematic transitions.

## 2. Token ledger (values of record)

The canonical values for motion duration and easing tokens are published in
`artifacts/design/motion_tokens.yaml` so:

- component contracts can cite stable token names;
- reduced-motion and low-power contracts can reason about the same timing set;
- tooling can diff “token use” rather than comparing raw numbers.

Rules (frozen):

1. Surfaces MUST use the published tokens; ad hoc per-surface milliseconds or
   private cubic-bezier curves are non-conforming.
2. Motion is never used as decoration in dense panes (Explorer/Search/Problems,
   editor typing paths, diagnostics refresh). If a surface cannot be understood
   without motion, it is non-conforming.
3. Reduced-motion and low-power postures preserve **focus visibility** and
   **state conveyance** (shape/border/icon/text), not just hue.

## 3. Semantic transition roles (enter/exit/emphasis/progress/attention/recovery)

The token ledger also publishes role-level defaults so later component packets
can say “this is an exit transition” without re-litigating easing rules.

Role rules (frozen):

- **Enter / Exit:** motion clarifies the relationship between source and
  destination. Prefer opacity + small transforms; avoid travel distances that
  imply content moved farther than it did.
- **Emphasis:** use tiny, non-looping transitions (opacity/border). Never pulse
  indefinitely.
- **Progress:** loops are permitted only for small progress indicators and only
  when the same progress is also conveyed statically (text/status).
- **Attention:** toasts and banners must never steal focus. Their content must
  remain readable with all motion suppressed.
- **Recovery:** acknowledge successful undo/repair without introducing new
  motion in constrained postures.

## 4. Interruption/cancellation and no-layout-shift guardrails

Rules (frozen):

1. **Transitions are interruptible.** If the user dismisses a toast, closes an
   overlay, or changes focus mid-transition, the transition MUST cancel without
   leaving a “half-applied” visual state.
2. **No layout shift during typing.** Motion MUST NOT animate reflow in typing
   hot paths. When a state change would otherwise reflow content, apply the
   layout change instantly and limit any animation to opacity/transform on a
   stable layout.
3. **No motion-only state.** Motion may communicate freshness/completion, but a
   static equivalent (text/icon/border/label chip) MUST exist and remain
   inspectable.

## 5. Low-motion semantic equivalence for common surfaces

These surfaces must preserve meaning under motion reduction:

- **Overlays (dialogs/sheets/spotlight frames):** default motion may fade/scale
  modestly; reduced-motion must resolve with crossfade-only or instant, with
  preserved title labels, focus placement, and escape routes.
- **Banners:** motion must not “push” editor content with animated height.
  Reduced-motion must show/hide without reflow animation; state text/icons are
  always present.
- **Toasts:** entry/exit motion is optional; reduced-motion and power-saver
  prefer instant or crossfade-only. Toasts never steal focus; countdown pauses
  on hover/focus/screen-reader announcement (UI/UX spec).
- **Durable-job progress:** loops are optional; reduced-motion replaces looping
  indicators with static progress text and completion markers.
- **Guided tours / teaching overlays:** spotlight/coachmark transitions must not
  be required to understand what is being highlighted; reduced-motion preserves
  labels, focus order, and reachability.
- **Focus-follow transitions:** focus movement is communicated through the focus
  ring and announcements; reduced-motion must avoid gliding focus or viewport
  drift. Focus visibility is never suppressed.

Worked motion presets in `fixtures/design/motion_cases/` show these defaults and
their reduced-motion / power-saver / critical hot-path equivalents.

