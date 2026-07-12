# M5 core-action-input component surface certification contract (M05-1131)

This is the closing B134 capstone over the frozen M5 core-action-input component matrix
(`schemas/ui/m5-core-action-input-component-matrix.schema.json`). Where the freeze matrix defines
the eight reusable **button**, **icon button**, **split button**, **text field**, **search
field**, **combobox**, **checkbox-radio-switch toggle control**, and **segmented control**
families, the M05-1125..1128 implement lanes narrow each one, the M05-1129 shared consumer lane
aligns their vocabulary, and the M05-1130 accessibility lane proves keyboard / screen-reader /
high-zoom / reduced-motion / CLI-export parity plus per-family auto-narrowing, this capstone
**certifies** that the shared control truth holds on every claimed M5 forms / settings / search /
entry / review / repair operating profile — and auto-narrows any profile that cannot sustain it.

- Boundary schema:
  [`schemas/ui/m5-core-action-input-component-surface-certification.schema.json`](../../schemas/ui/m5-core-action-input-component-surface-certification.schema.json)
- Canonical proof bundle (release):
  `artifacts/release/m5-core-action-input-component-surface-certification-proof/`
- Fixtures mirror:
  `fixtures/ui/m5-core-action-input-component-surface-certification/`
- Implementing module (aureline-ui):
  `m5_core_action_input_component_surface_certification`

## What it certifies

The packet is keyed on the claimed **profile** a user, reviewer, or support engineer operates a
reusable action / input control through — not on component family or implement lane. Eight
profiles are certified:

| Profile | Claim | Verdict | Families |
| --- | --- | --- | --- |
| `live_trusted_control_surface` | `trusted_control` | green | button, segmented control |
| `reviewable_control_structure` | `reviewable_control` | green | combobox |
| `unbound_command_surface` | narrows to `command_binding_unverified_projection` | yellow | button |
| `unlabeled_icon_surface` | narrows to `accessible_name_unverified_projection` | yellow | icon button |
| `riskier_split_default_surface` | narrows to `default_safety_unverified_projection` | yellow | split button |
| `stale_validation_field` | narrows to `validation_unverified_projection` | yellow | text field |
| `unverified_toggle_control` | narrows to `toggle_semantics_unverified_projection` | yellow | toggle control |
| `partial_retention_search_field` | narrows to `retention_disclosed_projection` | yellow | search field |

Every one of the eight frozen component families is certified on at least one profile, so the
forms, settings, search, entry, review, repair, and support/export lanes all trace back to the
B134 component family.

Each row is scored on eight truth axes, each appearing exactly once:

1. **visual** — interaction state, command binding, accessible name, default safety, validation,
   value source, toggle semantics, and selected mode on the primary surface, never by color alone.
2. **keyboard** — the same truth and its bounded local actions reachable without a pointer, never
   hover-only.
3. **screen_reader** — the same truth announced non-visually, never color/motion/glyph-only.
4. **high_zoom_reflow** — the same truth reflows legibly at high zoom.
5. **reduced_motion** — the same truth legible and usable with reduced motion.
6. **cli_export** *(always-on)* — the profile state reconstructable as text / JSON / Markdown.
7. **degraded_state** — a stale command binding, missing accessible name, unconfirmed default
   safety, stale validation anchor, unverified toggle semantic, or partial retention posture
   honestly downgrades the claim rather than reading as a fresh authoritative control.
8. **control_component_truth** — interaction state, command binding, accessible name, default
   safety, validation, value source, toggle semantics, selected mode, and the locked / read-only /
   degraded distinction stay explicit and never collapse into generic disabled chrome.

## Invariants

- **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
  `trusted_control` / `reviewable_control` claim while a truth axis is not current is over-claiming
  and **blocks (red)**. A profile that discloses the reduction by narrowing its claim (with a
  bound, non-generic reason and a frozen downgrade trigger) is honestly **yellow**.
- **Only a live first-party trusted control profile may certify `trusted_control`.** Any
  reviewable, unbound, unlabeled, riskier-default, stale, unverified, or partial-retention profile
  that keeps a trusted claim blocks.
- **CLI/export parity is always-on** and must stay certified so support and automation can
  reconstruct the interaction state, command binding, accessible name, default safety, validation,
  value source, toggle semantics, and retention posture from the same component identity the user
  saw.
- **Certification may only narrow a claim, never strengthen it.**
- **All six B134 guardrails must hold** on every row (a breach blocks):
  1. placeholder text must never be the only label;
  2. a loading control must not relabel or resize its action out of attribution;
  3. an icon-only destructive action must not be left unlabeled;
  4. a switch must not be blurred with a deferred checkbox;
  5. a split button must not default to a riskier alternate;
  6. locked / degraded semantics must not be hidden behind generic disabled chrome.

## Metadata-only boundary

Raw field values, option payloads, credentials, secrets, and endpoint refs never cross this
boundary. The packet carries only typed class tokens, opaque component refs, booleans, and
controlled labels so support, release, and diagnostics exports can reconstruct exactly what an
accessible fallback would have shown without leaking sensitive material.

## Regenerating the proof

The seed builder is the single source of truth shared by the tests and the on-disk export.
Regenerate the checked-in artifacts and fixtures with:

```sh
GEN_CORE_ACTION_INPUT_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_core_action_input_component_surface_certification::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the checked-in export drifts from
the seeded builder.
