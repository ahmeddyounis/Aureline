# Appearance import, token-overlay, and checkpoint contract

This document freezes the appearance-session contract that theme
packages, imported themes, token overlays, extension surfaces, webview-like
surfaces, QA captures, docs/help, and support exports share. Appearance
is not just a color preference: it carries focus, severity, trust,
policy-lock, source-integrity, contrast, density, motion, and rollback
truth. Any reader that projects appearance state MUST preserve those
axes instead of inferring them from screenshots or surrounding chrome.

The machine-readable schemas live at:

- [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json)
- [`/schemas/ux/theme_import_report.schema.json`](../../schemas/ux/theme_import_report.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/appearance_cases/`](../../fixtures/ux/appearance_cases/)

Where this contract disagrees with the PRD, architecture document,
technical design, UI/UX spec, design-system style guide, or
design-token vocabulary, those sources win and this contract plus its
schemas and fixtures update in the same change.

## Companion contracts

This contract consumes existing vocabulary by reference:

- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  and
  [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)
  own token families, component states, theme classes, density classes,
  accessibility postures, semantic status, and trust visual states.
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  owns the four first-party theme-support rows and accessibility
  posture rows.
- [`/docs/ux/theme_and_visual_asset_contract.md`](./theme_and_visual_asset_contract.md)
  and
  [`/schemas/ux/theme_package_manifest.schema.json`](../../schemas/ux/theme_package_manifest.schema.json)
  own theme package manifests, icon slots, and motion presets.
- [`/schemas/ux/embedded_surface_boundary.schema.json`](../../schemas/ux/embedded_surface_boundary.schema.json)
  owns embedded-surface owner/origin boundary truth. This contract adds
  appearance inheritance truth for those surfaces.

## Record kinds

### `appearance_session_record`

One record describes the effective appearance state for a running
session, profile, or capture:

- active theme package and revision refs;
- follow-system posture;
- resolved theme class and contrast mode;
- accent source;
- density class;
- text scale;
- reduced-motion posture and source;
- preview state;
- current checkpoint and rollback refs;
- active import-report and token-overlay report refs;
- protected-cue preservation rows for trust, policy-lock, severity, and
  source-integrity cues;
- extension or embedded-surface appearance descriptor refs.

Every live preview MUST cite exactly one current checkpoint ref before
the preview mutates visible state. Revert MUST restore from that same
checkpoint ref, not from a best-effort reconstruction.

### `appearance_checkpoint_record`

One record describes an explicit checkpoint created before an appearance
change is previewed or applied. It carries:

- checkpoint ref, session ref, checkpoint class, and scope;
- pre-change snapshot ref and optional post-preview snapshot ref;
- changed appearance axes;
- atomicity class;
- rollback path;
- preflight checks for contrast, protected cues, import report, and
  extension inheritance;
- apply state.

Appearance checkpoints are single-root rollback handles. A preview or
apply path that requires multiple uncoordinated reversions is
non-conforming and MUST block durable apply until it can mint one
checkpoint-backed rollback path.

### `extension_surface_appearance_record`

One record describes a UI-bearing extension, embedded surface, or
webview-like surface that appears inside host chrome. It declares
whether the surface inherits the host:

- theme;
- focus treatment;
- contrast posture;
- density;
- reduced-motion posture.

Each axis uses the closed inheritance set:

| Value | Meaning |
|---|---|
| `inherits` | The surface consumes the host token/posture for this axis without redefining semantics. |
| `partial` | The surface inherits some host behavior but has named gaps. |
| `does_not_inherit` | The surface owns a separate behavior and must disclose the gap. |

The record also carries theme/contrast support rows for each claimed
mode. A surrounding shell that is themed correctly does not prove the
embedded body has parity; the body must publish its own rows or stay
unclaimed.

### `theme_import_report_record`

One report describes a translated theme import from another ecosystem.
It names:

- source ecosystem, source tool, source tool version, and source
  artifact ref;
- translated slots;
- unsupported slots;
- syntax-token coverage;
- unresolved mapping count;
- fallback token classes;
- protected-cue honesty checks;
- checkpoint and rollback path;
- parity claim state.

Imported themes MUST NOT claim parity unless the report is present,
unresolved mappings are zero, blocked honesty checks are zero, and the
requested theme/contrast modes are covered by evidence.

### `token_overlay_report_record`

One report describes explicit token overrides applied by user, profile,
workspace, policy, extension, or imported-theme scope. It names:

- overlay source and scope;
- per-token overlay entries;
- translated, unsupported, unresolved, substituted, and blocked counts;
- fallback token classes;
- unknown-token round-trip posture;
- protected-cue honesty checks;
- rollback path.

Unsupported and unknown tokens must survive round trip as visible inert
or blocked rows. Dropping an unknown token silently is non-conforming.

## Appearance-session rules

1. **One effective session object.** Settings UI, preview matrix,
   import review, support export, QA capture, and extension inheritance
   cards read the same `appearance_session_record`.
2. **Live preview is checkpointed.** A live preview cannot begin until
   `preview_state` cites a current checkpoint ref. Revert restores from
   that ref atomically.
3. **Apply and revert are atomic.** A half-themed shell, a mixed
   extension/body posture, or a partial token overlay is an apply
   failure. The session must revert or block durable apply.
4. **Platform limits are explicit.** If OS theme, contrast, accent, text
   scale, or reduced-motion changes require a surface reload or full
   restart, the record names that posture rather than implying live
   support.
5. **Density is layout density only.** Density changes row height,
   padding, pane chrome, and spacing-like tokens. It may not change
   information architecture, focus visibility, or command semantics.

## Import and overlay rules

1. **Reports are mandatory for imported parity claims.** Imported themes
   and token overlays that affect appearance must publish a report
   before they can claim dark/light/high-contrast parity.
2. **Unresolved mappings are visible.** Reports carry
   `unresolved_mapping_count`, `unsupported_slot_count`,
   `substituted_with_fallback_count`, and `blocked_honesty_count`.
   Readers surface these counts directly.
3. **Fallback classes are typed.** A fallback must name a
   `fallback_token_class` such as `focus_ring_default`,
   `trust_visual_state`, or `syntax_default`. "Best effort" is not a
   valid fallback class.
4. **Syntax coverage is separate.** Syntax scopes have their own
   coverage row so a theme cannot claim general UI parity while syntax
   highlighting is mostly unresolved.
5. **Rollback path is part of import review.** Every import or overlay
   report names the checkpoint and user-visible rollback action that
   restores the pre-import appearance.

## Protected-cue honesty rules

Trust, policy-lock, severity, and source-integrity cues are protected
appearance semantics. Imported, custom, extension-contributed, and
workspace overlays MUST preserve each protected cue with at least one
non-color cue such as shape, border, icon, text, label chip, position,
or accessible name.

The following are non-conforming:

- mapping a policy-lock or restricted-workspace cue to a hue-only
  difference;
- replacing severity icons with color-only text;
- hiding source-integrity state inside a muted foreground color;
- letting high-contrast or forced-colors mode erase trust or policy
  state;
- allowing an embedded body to omit protected cue labels while the host
  chrome still shows them.

Blocked protected-cue checks deny durable apply. A preview may render
only inside the import or theme review surface while the failed cue and
rollback path remain visible.

## Extension and embedded-surface inheritance

UI-bearing extensions and embedded surfaces publish inheritance rows for
theme, focus, contrast, density, and reduced motion. Each row names its
claim source and gap count. If a row is `partial` or
`does_not_inherit`, the surface must disclose the gap in the extension
appearance card, support export, and compatibility evidence.

Theme/contrast support rows are required when a surface claims parity in
dark, light, high-contrast, or forced-colors modes. A row can be
`supported`, `partial`, `unsupported`, or `not_claimed`. `partial` and
`unsupported` rows must name the gap and may not be summarized as
"themed".

## Denial reasons

Schemas use closed denial-reason sets so logs, QA, docs, and support
can join failures without string matching:

- missing checkpoint before preview;
- partial appearance apply;
- unresolved import mapping hidden;
- fallback class missing;
- syntax coverage missing;
- protected cue converted to color-only;
- extension inheritance gap hidden;
- claimed theme/contrast mode lacks support row;
- rollback path missing;
- unknown token dropped.

Adding a denial reason is additive-minor and bumps the relevant schema
version. Repurposing an existing reason is breaking.

## Fixture coverage

The fixture corpus covers:

- live preview with one checkpoint and rollback ref;
- import preview rollback from a checkpoint;
- imported theme report with unresolved mappings and fallback classes;
- token overlay blocked by protected-cue honesty checks;
- extension/webview partial inheritance with theme/contrast support
  rows.

## Out of scope

- Theme marketplace implementation.
- Final theme, icon, or syntax-token asset production.
- A runtime conformance runner. The schemas are precise enough for a
  future runner to validate sessions, reports, checkpoints, and
  extension inheritance claims against these records.
