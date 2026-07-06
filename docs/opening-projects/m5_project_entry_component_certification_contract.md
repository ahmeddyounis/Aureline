# M5 Project-Entry Component Surface Certification (M05-843)

This is the closing certification capstone over the frozen M5 project-entry
component matrix (M05-836 / 839 / 840 / 841) and its first-consumer adoption lane
(M05-842). Where the freeze matrix defines the ten reusable start-center
quick-action, recent-work, workspace-switcher, restore-prompt, entry-chooser,
entry-review, destination-collision, post-entry-handoff, admission-checkpoint,
and archetype-readiness cards, rows, and sheets, and the consumer lane adopts
them across five consumer classes, this capstone **certifies** that the shared
components behave consistently on every claimed M5 project-entry surface.

## What is certified

The packet is keyed on the claimed **surface**, not on a component family or a
consumer group. Nine surfaces are certified exactly once each:

- `start_center`
- `command_palette`
- `system_open`
- `deep_link`
- `cli_headless`
- `template_prebuild`
- `clone`
- `import`
- `restore`

Each surface is scored on five truth axes:

1. `profile_remote_badge` — profile / remote badge parity.
2. `restore_class` — restore-fidelity class parity.
3. `trust_posture` — root identity, trust class, host/auth posture.
4. `first_useful_work_routing` — attributable routing + same-weight plain open.
5. `export_parity` — **always-on**: the surface state is copyable as
   text / JSON / Markdown for support and automation.

## Verdicts

Each surface derives one verdict — never asserted by the author, always
recomputed from its axes and tier narrowing:

- **Green** — every axis is certified and the surface delivers its claimed
  interactive tier.
- **Yellow** — a truth axis is not current, so the surface auto-narrows its
  interactive claim to the weakest supported ceiling (`full_entry` →
  `reviewed_entry` → `inspect_only` → `export_only`). The narrowing is disclosed
  with a bound reason, a visible downgrade trigger, and a `claim_auto_narrow`
  whose `binding_axis` is one of the narrowed axes.
- **Red (blocked)** — a degraded axis is hidden behind a full-truth claim
  inherited from a healthier lane, export parity drops, the narrowing is
  inconsistent, or the certified tier exceeds the claimed tier.

The governing invariant is: **a degraded axis must produce a visible tier
narrowing**. Unsupported or degraded entry paths narrow visibly instead of
inheriting full-truth labels from healthier lanes.

## Canonical bundle

Every row cites exactly one canonical release-proof bundle —
`artifacts/release/m5-project-entry-component-proof/packet.json` — rather than
cloning per-surface evidence. The M05-842 consumer support export is recorded as
a supporting evidence ref.

## Boundary

The packet is metadata-only. Raw file paths, clone URLs, credentials, remote
hosts, and device identifiers never cross this boundary; only typed class
tokens, opaque refs, booleans, and redacted labels do.

## Artifacts

- **Rust module:** `crates/aureline-shell/src/m5_project_entry_component_certification/`
- **Boundary schema:** `schemas/ui/m5-project-entry-component-certification.schema.json`
- **Release proof:** `artifacts/release/m5-project-entry-component-certification-proof/`

## Regenerate

```
cargo run -p aureline-shell --example dump_project_entry_component_certification
```
