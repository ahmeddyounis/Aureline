# M5 decision-feedback component surface certification contract (M05-1139)

This is the closing B135 capstone over the frozen M5 decision-feedback component matrix
(`schemas/ui/m5-decision-feedback-component-matrix.schema.json`). Where the freeze matrix defines
the eight reusable **badge-chip-pill**, **popover**, **dialog-sheet**, **banner-inline-notice**,
**toast**, **empty-state**, **loading-state**, and **consequence-block** families, the
M05-1133..1136 implement lanes narrow each one, the M05-1137 shared consumer lane aligns their
vocabulary, and the M05-1138 accessibility lane proves keyboard / screen-reader / high-zoom /
reduced-motion / CLI-export parity plus per-family auto-narrowing, this capstone **certifies** that
the shared decision-feedback truth holds on every claimed M5 shell / entry / trust / review /
repair / notification operating profile — and auto-narrows any profile that cannot sustain it.

- Boundary schema:
  [`schemas/ui/m5-decision-feedback-component-surface-certification.schema.json`](../../schemas/ui/m5-decision-feedback-component-surface-certification.schema.json)
- Canonical proof bundle (release):
  `artifacts/release/m5-decision-feedback-component-surface-certification-proof/`
- Fixtures mirror:
  `fixtures/ui/m5-decision-feedback-component-surface-certification/`
- Implementing module (aureline-ui):
  `m5_decision_feedback_component_surface_certification`

## What it certifies

The packet is keyed on the claimed **profile** a user, reviewer, or support engineer reads,
dismisses, reopens, or exports a reusable decision / feedback primitive through — not on component
family or implement lane. Eight profiles are certified:

| Profile | Claim | Verdict | Families |
| --- | --- | --- | --- |
| `live_trusted_decision_surface` | `trusted_decision_surface` | green | dialog sheet, consequence block |
| `reviewable_decision_structure` | `reviewable_decision_surface` | green | empty state |
| `stale_severity_badge_surface` | narrows to `severity_unverified_projection` | yellow | badge chip pill |
| `unscoped_notice_surface` | narrows to `scope_unverified_projection` | yellow | banner inline notice |
| `unanchored_popover_surface` | narrows to `focus_return_unverified_projection` | yellow | popover |
| `toast_only_durable_surface` | narrows to `durable_object_unverified_projection` | yellow | toast |
| `spinner_loading_surface` | narrows to `partial_capability_unverified_projection` | yellow | loading state |
| `partial_recovery_consequence_surface` | narrows to `recovery_path_disclosed_projection` | yellow | consequence block |

Every one of the eight frozen component families is certified on at least one profile, so the
shell, help, support, review, settings, updates, and CLI/support-export lanes all trace back to the
B135 component family.

Each row is scored on eight truth axes, each appearing exactly once:

1. **visual** — disposition, severity meaning, notice scope, rationale, next action, and durability
   on the primary surface, never by color alone.
2. **keyboard** — the same truth and its bounded local actions reachable without a pointer, never
   hover-only.
3. **screen_reader** — the same truth announced non-visually, never color/motion/glyph-only.
4. **high_zoom_reflow** — the same truth reflows legibly at high zoom.
5. **reduced_motion** — the same truth legible and usable with reduced motion.
6. **cli_export** *(always-on)* — the profile state reconstructable as text / JSON / Markdown.
7. **degraded_state** — a stale severity evidence, unconfirmed notice scope, unanchored focus
   return, missing durable-object linkage, partial capability, or partial recovery posture honestly
   downgrades the claim rather than reading as a fresh authoritative decision surface.
8. **decision_feedback_component_truth** — disposition, severity meaning, notice scope, rationale,
   focus-return anchor, durable-object linkage, partial-capability fidelity, and the recovery /
   rollback posture stay explicit and never collapse into generic something-went-wrong chrome.

## Invariants

- **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
  `trusted_decision_surface` / `reviewable_decision_surface` claim while a truth axis is not current
  is over-claiming and **blocks (red)**. A profile that discloses the reduction by narrowing its
  claim (with a bound, non-generic reason and a frozen downgrade trigger) is honestly **yellow**.
- **Only a live first-party trusted decision profile may certify `trusted_decision_surface`.** Any
  reviewable, stale-severity, unscoped, unanchored, toast-only, spinner, or partial-recovery profile
  that keeps a trusted claim blocks.
- **CLI/export parity is always-on** and must stay certified so support and automation can
  reconstruct the disposition, severity meaning, notice scope, focus-return anchor, durable-object
  linkage, partial-capability fidelity, and recovery / rollback posture from the same component
  identity the user saw.
- **Certification may only narrow a claim, never strengthen it.**
- **All six B135 guardrails must hold** on every row (a breach blocks):
  1. color must never be the only meaning for a badge / banner / inline-notice;
  2. a popover must not carry the only critical workflow instruction;
  3. generic Yes/No copy must not be used in a high-risk dialog;
  4. long-running or reviewable work must not be shown as toast-only truth;
  5. a useful pane must not be blanked during loading;
  6. a full-screen spinner must not be used where partial capability exists.

## Metadata-only boundary

Raw field values, copy payloads, credentials, secrets, and endpoint refs never cross this boundary.
The packet carries only typed class tokens, opaque component refs, booleans, and controlled labels
so support, release, and diagnostics exports can reconstruct exactly what an accessible fallback
would have shown without leaking sensitive material.

## Regenerating the proof

The seed builder is the single source of truth shared by the tests and the on-disk export.
Regenerate the checked-in artifacts and fixtures with:

```sh
GEN_DECISION_FEEDBACK_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_decision_feedback_component_surface_certification::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the checked-in export drifts from the
seeded builder.
