# M5 keyboard, assistive-tech, reduced-motion, and interruption-safe continuity

One canonical, export-safe model for the **accessibility and interruption-safety
contract** every M5 dense multi-step form, inline validation-link group, and
batch/staged-review sheet must hold so the shared structured-input model stays fully
usable under keyboard-only, assistive-tech, reduced-motion, reconnect, and restart
conditions. One contract is reused across the provider, admin, request, package,
settings, import, and project lanes instead of each domain re-inventing focus order,
screen-reader labelling, or recovery semantics — so an extension or provider-owned
surface cannot quietly regress accessibility or interruption behavior.

Where [the field/control-row model](m5-field-control-rows.md) freezes a single field's
label and validation anchor, [the form-validation model](m5-form-validation-and-blocked-submit.md)
freezes how a form rolls those anchors up and explains a blocked submit,
[the draft-state model](m5-draft-state-and-autosave.md) freezes the autosave journal a
draft recovers from, and [the staged-review model](m5-staged-review-sheets.md) freezes
the commit sheet a mutation stops at, this model freezes the **accessibility and
recovery behavior** of those same surfaces: that they stay keyboard-complete,
screen-reader reachable, reduced-motion-safe, and resume on the correct step after an
interruption.

- Schema: [`schemas/ux/m5-accessibility-and-continuity.schema.json`](../../schemas/ux/m5-accessibility-and-continuity.schema.json)
- Canonical support export: [`artifacts/ux/m5-accessibility-and-continuity/support_export.json`](../../artifacts/ux/m5-accessibility-and-continuity/support_export.json)
- Report: [`artifacts/ux/m5-accessibility-and-continuity/report.md`](../../artifacts/ux/m5-accessibility-and-continuity/report.md)
- Perturbation corpus: [`fixtures/ux/m5-accessibility-and-continuity/`](../../fixtures/ux/m5-accessibility-and-continuity/)
- Rust truth source: `crates/aureline-ui/src/m5_accessibility_and_continuity`
- Validator: `tools/release/accessibility_and_continuity.py`

## What a surface carries

Each `SurfaceRecord` (`#/$defs/surface`) is one mutation-capable form or review
sheet, identified by `surface_id`, its `surface_kind` (`multi_step_form`,
`inline_validation_links`, `batch_review_sheet`, `staged_review_sheet`,
`config_editor`), its `lane`, its `origin`, and its `claim_posture`, plus the
accessibility and continuity contract it must hold:

- **Keyboard reachability** (`accessibility.keyboard`) — a deterministic
  `focus_order_defined`, `roving_tabindex` over dense collections, every interactive
  control `all_controls_reachable`, `batch_actions_keyboard_parity` for batch-review
  actions, and an escapable, labelled focus trap (`focus_trap_escapable`,
  `focus_trap_escape_labeled`).
- **Assistive-tech reachability** (`accessibility.assistive_tech`) —
  `screen_reader_labels_present` on every control, inline `validation_links_announced`
  to AT (parity with the visual links), blocked-submit reasons in a
  `blocked_submit_live_region`, and the current `step_position_announced`.
- **Reduced-motion behavior** (`accessibility.reduced_motion`) — the shared design-system
  `substitution_class` (`crossfade_only`, `maintain_essential_keep_simplified`,
  `suppress_entirely`, `collapse_to_instant`, `non_motion_state_marker`), whether
  `state_conveyed_without_motion`, and whether a `progress_non_motion_marker` carries
  step progress. The substitution class is the same vocabulary the shell consults
  before running a transition, so a surface cannot opt out of the reduced-motion policy.
- **Interruption-safe continuity** (`continuity`) — whether a recovery `journal_backed`
  the surface and its `journal_state` (`complete`/`partial`/`stale`/`missing`), whether
  the `current_step_preserved`, `blocked_fields_preserved`, and `draft_state_preserved`,
  and whether the flow resumes after each interruption path (`resume_on_reconnect`,
  `resume_on_restart`, `resume_on_missing_dependency`, `resume_on_crash`).

`integrity` carries the headline invariants the surface re-derives rather than trusting
a grade; `declared_recovery_target` declares what a recovery returns the user to
(`surface_and_step`, `step_only`, `none_keyboard_fallback`); a `verification` proof ages
an unproven surface down; `lineage` (including `form_ref`, `provider_ref`,
`source_artifact_ref`, `policy_ref`, `journal_ref`, and `recovery_backlink_ref`)
attributes the surface; and `renderings[]` lists the consumer surfaces — the live
surface, a review sheet, diagnostics, the support export, an accessibility audit, inline
help/docs, and the CLI/headless path — and the claim each one shows.

## Effective claim

`SurfaceRecord::narrow` re-derives a `ContinuityClaim` per surface so it can never read
wider than its evidence:

| Claim | Meaning |
| --- | --- |
| `continuity_certified` | Full keyboard-complete, AT-reachable, reduced-motion-safe, interruption-safe accessibility-and-continuity contract. |
| `continuity_narrowed` | A first-party surface held below certified by a labelled, recoverable gap (an un-announced step position, an unlabelled focus-trap escape or reduced-motion substitution, an unlabelled progress marker, a partial/stale journal, a stale/missing proof); the surface stays keyboard-complete and recoverable. |
| `continuity_review_overlay` | A read-only review of imported/migrated values; keyboard-complete and AT-reachable but never reads as an apply. |
| `continuity_unsafe` | The accessibility/continuity contract is broken; the surface falls back to an explicit blocked-submit state with a keyboard recovery path instead of a clean-but-false surface. |
| `continuity_labs_not_claimed` | Labs/unadvertised; makes no public claim and is never widened. |

A higher-rank claim asserts more authority, so a narrowing or floor moves strictly
lower, and a rendering that shows wider than the effective claim is itself a floor
(`surface_overclaims`).

### Floor reasons (drop to `continuity_unsafe`)

These break the accessibility/continuity contract outright:
`keyboard_path_incomplete`, `focus_order_undefined`,
`batch_actions_keyboard_unreachable`, `screen_reader_labels_missing`,
`validation_links_not_announced`, `blocked_submit_not_announced`, `motion_only_state`,
`current_step_lost`, `blocked_fields_lost`, `draft_state_lost`,
`imported_review_mutable`, `recovery_path_lost`, `surface_overclaims`, and
`continuity_journal_missing`. A floored surface keeps a keyboard recovery fallback
(`declared_recovery_target` of `step_only`/`none_keyboard_fallback` or a
`recovery_backlink_ref`) rather than a misleading clean submit.

### Narrowing reasons (hold at `continuity_narrowed`, stay usable)

`step_position_unannounced`, `focus_trap_escape_unlabeled`,
`reduced_motion_substitution_unlabeled`, `progress_marker_unlabeled`, `journal_partial`,
`continuity_proof_stale`, and `continuity_proof_missing`. On an imported/migration
review overlay, any non-floor gap drops the surface below the overlay because the
overlay is already the minimal honest claim.

### Continuity floors apply only to mutation-capable surfaces

The continuity floors (`current_step_lost`, `blocked_fields_lost`, `draft_state_lost`,
`continuity_journal_missing`) and the `blocked_submit_not_announced` floor apply only to
non-overlay surfaces: a read-only import/migration review has no draft to recover and no
submit to block, so it is exempt and stays a review overlay. The batch-action
keyboard-parity floor applies only where the kind has batch actions, and the
validation-link floor applies only where the kind carries inline validation links.

## Guardrails enforced by the validator

`M5AccessibilityContinuitySetPacket::validate` (Rust) and
`tools/release/accessibility_and_continuity.py validate` (the CI gate) both refuse a
packet that:

- drops a keyboard path, an undefined focus order, an unreachable batch action, a
  missing screen-reader label, an un-announced inline validation link or blocked submit,
  or a state carried only by motion;
- loses the current step, blocked-field context, or draft-state continuity across an
  interruption, ships a mutable imported review, or has no recovery journal backing a
  mutation-capable surface;
- loses the keyboard recovery path, lets a rendering overclaim, or floors a surface that
  loses its keyboard recovery fallback;
- fails to represent every surface kind, lane, origin, interruption path, reduced-motion
  substitution class, or consumer surface, or contains no surface that demonstrates the
  auto-narrowing rule;
- leaks raw credential/secret material across the export boundary.

## Accessibility and recovery stay first-class under every posture

The model keeps a surface's keyboard, assistive-tech, reduced-motion, and recovery
behavior explicit and independent. A dense multi-step form is keyboard-complete and its
step position is announced; a batch-review sheet's actions all have keyboard parity; an
inline validation link is reachable by both keyboard and assistive tech; the
reduced-motion substitution class is the same one the shell consults, so state is never
carried only by animation; and a recovery journal preserves the current step, blocked
fields, and draft so an interrupted flow resumes on the correct step. The same data is
what the support export, the accessibility-audit surface, the CLI/headless path, and the
docs/help references consume — they re-render it rather than re-describing it.

## Regenerating the artifacts

```bash
# Canonical support export + report (Rust seed is the source of truth)
cargo run -p aureline-ui --example dump_m5_accessibility_and_continuity \
  > artifacts/ux/m5-accessibility-and-continuity/support_export.json
cargo run -p aureline-ui --example dump_m5_accessibility_and_continuity report \
  > artifacts/ux/m5-accessibility-and-continuity/report.md

# Perturbation corpus
python3 tools/release/accessibility_and_continuity.py emit-corpus

# Verify everything (schema, re-derivation, corpus)
python3 tools/release/accessibility_and_continuity.py self-test
cargo test -p aureline-ui m5_accessibility
```

The Rust seed builder, the checked-in support export, and the Python re-derivation are
kept byte-aligned: a Rust test asserts the checked-in export equals the in-crate
builder, and the Python `self-test` re-derives every surface and corpus case so the
artifacts can never imply a wider claim than the current evidence backs.
