# Suspicious-content, safe-preview, copy/export, and representation cues on shell-adjacent surfaces — contract

This is the reviewer-facing companion for the stable lane that certifies
**suspicious-content, safe-preview, copy/export, and representation cues on
shell-adjacent surfaces**: one governed record per shell-adjacent surface posture
that binds the **consumed trust-class ladder**, **explicit representation cues**,
**surfaced suspicious-content findings**, **cue survival across carriers**, a
**stricter preview boundary shown before commit**, **complete accessibility
cues**, **per-OS conformance**, and a **public claim ceiling** with an automatic
narrow-below-Stable verdict.

This lane consumes the content-safety stack (`aureline_content_safety`) rather
than re-spelling it. Where that crate owns the suspicious-content detector, the
trust-class ladder (`RawText`, `SanitizedRich`, `TrustedLocalActive`,
`IsolatedRemoteActive`), and the representation-transfer grammar (`Copy raw`,
`Copy rendered`, `Copy escaped`, sanitized snapshot, metadata-only), this lane
proves that *every* shell-adjacent surface that can render or hand off ambiguous
content keeps the trust class, the representation label, and the
suspicious-content warning intact instead of flattening them into a generic
"preview" string — and that trust-sensitive actions show their stricter boundary
before the user commits.

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and/`](../../../fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and/)
- Schema:
  [`/schemas/ux/certify-suspicious-content-safe-preview-copy-export-and.schema.json`](../../../schemas/ux/certify-suspicious-content-safe-preview-copy-export-and.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/certify-suspicious-content-safe-preview-copy-export-and.md`](../../../artifacts/ux/m4/certify-suspicious-content-safe-preview-copy-export-and.md)
- Typed source: `aureline_shell::shell_safe_preview_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_safe_preview_stable`
- Replay + invariant gate:
  `crates/aureline-shell/tests/safe_preview_stable_fixtures.rs`

## Why one governed safe-preview record

Shell-adjacent surfaces — notifications, the activity center, browser /
open-external handoff, support export, screenshot / evidence capture, and
trust-sensitive review actions like install, attach, approve, publish, and
delete — all converge on the same risk: a surface that renders or hands off
ambiguous content quietly flattens it into a generic preview string. When that
happens the bidi-control, invisible-formatting, or mixed-script warning
disappears, the trust class is lost, and the raw-vs-rendered representation choice
never reaches the user. A switching user then commits a trust-sensitive action
against bytes that do not mean what they appear to mean.

This lane mints one governed `shell_safe_preview_record` per posture. It does
**not** reinvent the trust-class ladder, the suspicious-content detector, or the
representation-transfer grammar: each record is a genuine projection of the live
content-safety stack — `aureline_content_safety::detect_suspicious_content` runs
over fixed seed bytes to produce the findings, and the trust class and
representation actions are consumed verbatim. The record binds, for one
shell-adjacent surface identity:

1. **The consumed trust-class ladder.** `trust_class` and `detector_outcome` come
   straight from `aureline_content_safety`; `upstream` keeps the content-safety
   contract ref and the trust-class / representation-policy schema versions so the
   provenance is auditable.
2. **Explicit representation cues.** `representation` proves
   `raw_reveal_available`, `representation_label_present`, `copy_raw_present`, and
   the derived `explicit_when_meaning_differs`; whenever `renders_rich_content` is
   true, `copy_rendered_present` is required too (pillar
   `representation_cues_explicit`).
3. **Surfaced suspicious-content findings.** Each `suspicious_findings[]` row keeps
   its `reveal_affordances`, `raw_toggle_available`, and `escaped_copy_available`
   so a finding never collapses to a single glyph (pillar
   `suspicious_findings_surfaced`).
4. **Labeled copy/export.** `representation_choices[]` carries explicit
   content-safety action ids, representation classes, body postures, and labels,
   and always offers a raw path and a safe-inspection path (pillar
   `copy_export_labeled`).
5. **Cue survival across carriers.** `cue_survival[]` covers the notification,
   activity-center, browser-handoff, support-export, and screenshot/evidence
   carriers; each proves the trust class, representation label, and warning survive
   without flattening (pillar `cues_survive_all_carriers`).
6. **A stricter boundary before commit.** For trust-sensitive actions
   (`is_trust_sensitive_action`), `stricter_boundary` enforces a stricter preview
   class than ordinary browsing, shows the boundary before commit, and blocks
   commit until acknowledged (pillar `stricter_boundary_shown_before_commit`).
7. **Complete accessibility cues.** `a11y_cues` proves the warning is announced
   (not color-only), the representation label and trust class are announced, and
   the reveal affordance is keyboard reachable; `accessibility` holds across
   normal / high-contrast / zoomed layouts (pillar `accessibility_cues_complete`).
8. **Per-OS conformance.** `platform_conformance[]` covers macOS, Windows, and
   Linux with current proof (pillar `platform_conformance_complete`).

## The claim ceiling and automatic narrowing

`claim_ceiling` is a hard ceiling: a posture may assert a pillar only when the
evidence proves it, and the builder rejects any over-claim. The
`stable_qualification` verdict is *derived* from the pillars and the lowest
binding-surface marker: a posture that cannot prove a pillar, or that binds a
surface still below Stable, is narrowed below Stable with a named
`narrowing_reasons` entry rather than inheriting an adjacent green row. The
`honesty_marker_present` flag is set whenever the row is narrowed or its binding
surface is below Stable.

## The claimed-stable matrix

The corpus spans the five carrier surfaces and three trust-sensitive actions and
includes four narrowed drills:

- **Stable carriers:** a notification with a bidi-control warning, an
  activity-center row with an invisible-formatting warning, a redacted support
  export, and a screenshot/evidence capture of a sanitized rich preview that keeps
  both Copy raw and Copy rendered explicit.
- **Stable trust-sensitive actions:** an open-external handoff of a confusable
  URL, an install review, and a delete review — each enforcing the stricter
  isolated-remote class and showing the boundary before commit.
- **Narrowed drills:** a support export that flattens the warning
  (`cues_flattened_on_carrier`), an install review that hides the stricter boundary
  before commit (`stricter_boundary_not_shown_before_commit`), a rendered browser
  handoff that hides the raw reveal (`representation_cues_not_explicit`), and a
  notification that proves every pillar but binds a Help/About surface still in
  preview (`surface_not_yet_stable`, narrowed to Preview).

## Binding surfaces and routes

The shell surface, the activity center, the CLI inspector, Help/About, and the
diagnostics support export each bind the shared record (`surface_projections[]`,
`reads_shared_record = true`) rather than cloning prose. The same posture opens
from the activity center, command palette, status bar, and a menu command
(`routes[]`), keyboard-first, and stays available without an account or managed
services. `recovery_routes[]` exposes reveal raw source, copy safe
representation, inspect codepoints, open content-safety help, and export
safe-preview support.

## Reproduce

```sh
cargo run -q -p aureline-shell --bin aureline_shell_safe_preview_stable -- index
cargo run -q -p aureline-shell --bin aureline_shell_safe_preview_stable -- plaintext
cargo run -q -p aureline-shell --bin aureline_shell_safe_preview_stable -- emit-fixtures \
  fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and
cargo test -p aureline-shell --test safe_preview_stable_fixtures
```
