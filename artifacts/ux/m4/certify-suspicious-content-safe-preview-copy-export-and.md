# Suspicious-content, safe-preview, copy/export, and representation cues on shell-adjacent surfaces — release evidence

Reviewer-facing evidence packet for the lane that certifies **suspicious-content,
safe-preview, copy/export, and representation cues on shell-adjacent surfaces**:
one canonical record per shell-adjacent surface posture that binds the consumed
trust-class ladder, explicit representation cues (raw/reveal, representation
label, and Copy raw / Copy rendered / Copy escaped), surfaced suspicious-content
findings, cue survival across the notification / activity-center / browser-handoff
/ support-export / screenshot-evidence carriers, a stricter preview boundary shown
before commit for trust-sensitive actions, complete accessibility cues, per-OS
conformance, a public claim ceiling, an automatic narrow-below-Stable verdict,
recovery and route parity across the activity center / command palette / status
bar / menus, accessibility across normal / high-contrast / zoomed layouts, and
postures that stay available without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and/`](../../../fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and/)
- Schema: [`/schemas/ux/certify-suspicious-content-safe-preview-copy-export-and.schema.json`](../../../schemas/ux/certify-suspicious-content-safe-preview-copy-export-and.schema.json)
- Companion doc: [`/docs/ux/m4/certify-suspicious-content-safe-preview-copy-export-and.md`](../../../docs/ux/m4/certify-suspicious-content-safe-preview-copy-export-and.md)
- Typed source: `aureline_shell::shell_safe_preview_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_safe_preview_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/safe_preview_stable_fixtures.rs`
- Projected from: `aureline_content_safety` (suspicious-content detector, trust-class ladder, representation-transfer vocabulary)

## The claimed-stable matrix

| Record | Surface family | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `notification_bidi_warning_stable.json` | notification | **stable** | stable | — |
| `activity_center_invisible_warning_stable.json` | activity_center | **stable** | stable | — |
| `browser_handoff_open_external_stable.json` | open_external_handoff | **stable** | stable | — |
| `support_export_redacted_stable.json` | support_export | **stable** | stable | — |
| `screenshot_evidence_capture_stable.json` | screenshot_evidence | **stable** | stable | — |
| `install_review_stricter_class_stable.json` | install_review | **stable** | stable | — |
| `delete_review_action_stable.json` | delete_review | **stable** | stable | — |
| `support_export_flattened_preview_drill.json` | support_export | beta (narrowed) | stable | `cues_flattened_on_carrier` |
| `install_boundary_not_shown_drill.json` | install_review | beta (narrowed) | stable | `stricter_boundary_not_shown_before_commit` |
| `rendered_reveal_missing_drill.json` | browser_handoff | beta (narrowed) | stable | `representation_cues_not_explicit` |
| `preview_help_surface_posture.json` | notification | preview (narrowed) | preview | `surface_not_yet_stable` |

Coverage verdict: **7 Stable, 4 narrowed**, covering all five carrier surfaces
(notification, activity center, browser handoff, support export, screenshot/
evidence) and three trust-sensitive actions (open-external, install, delete). Each
narrowed row names a reason and drops below the launch cutline rather than
inheriting an adjacent green row.

## Acceptance criteria → evidence

- **Consume the stable safe-preview trust-class ladder rather than re-spelling
  it.** Every record's `trust_class` is one of `RawText`, `SanitizedRich`,
  `TrustedLocalActive`, `IsolatedRemoteActive`, and every `representation_choices[]`
  entry uses the content-safety action ids (`copy_raw`, `copy_rendered`,
  `copy_escaped`, `export_sanitized_snapshot`, `export_metadata_only`). The
  suspicious-content findings are produced by
  `aureline_content_safety::detect_suspicious_content` over fixed seed bytes —
  bidi-control, invisible-formatting, and mixed-script-confusable findings all
  appear — and `upstream` carries the content-safety contract ref and the
  trust-class / representation-policy schema versions.
- **Raw/reveal affordances, representation labels, and Copy raw / Copy rendered
  choices stay explicit whenever rendered meaning can differ from source bytes.**
  Every record's `representation` block proves `raw_reveal_available`,
  `representation_label_present`, `copy_raw_present`, and
  `explicit_when_meaning_differs`; whenever `renders_rich_content` is true,
  `copy_rendered_present` is also true. The `rendered_reveal_missing_drill` renders
  rich content but hides the raw-source reveal affordance and is narrowed with
  `representation_cues_not_explicit`.
- **Suspicious-content findings keep reveal affordances and a reachable
  escaped-copy path.** Every `suspicious_findings[]` row carries non-empty
  `reveal_affordances`, `raw_toggle_available`, and `escaped_copy_available`;
  `suspicious_content_present` agrees with the finding list.
- **Cues survive the notification, activity-center, browser-handoff,
  support-export, and screenshot/evidence carriers without flattening.** Every
  record's `cue_survival[]` covers all five carriers, and on Stable rows each
  carrier proves `preserves_trust_class`, `preserves_representation_label`,
  `preserves_suspicious_warning`, and `does_not_flatten_to_generic_preview`. The
  `support_export_flattened_preview_drill` flattens the support-export carrier and
  is narrowed with `cues_flattened_on_carrier`.
- **Trust-sensitive actions show a stricter preview boundary before commit.**
  Install, delete, and open-external records carry a `stricter_boundary` that
  enforces a stricter class (`IsolatedRemoteActive`) over ordinary browsing,
  `shows_boundary_before_commit`, and `commit_blocked_until_acknowledged`. The
  `install_boundary_not_shown_drill` enforces a stricter class but does not show
  the boundary before commit and is narrowed with
  `stricter_boundary_not_shown_before_commit`.
- **Accessibility cues are announced, not color-only, across layouts.** Every
  record's `a11y_cues` proves `warning_announced_not_color_only`,
  `representation_label_announced`, `trust_class_announced`, and
  `reveal_affordance_keyboard_reachable`, and the `accessibility` block holds across
  normal / high-contrast / zoomed layouts.
- **Per-OS conformance covers macOS, Windows, and Linux.** Every record's
  `platform_conformance[]` covers the three profiles with current proof and named
  safe-preview behaviors.
- **Below-Stable surfaces are narrowed, not inherited.**
  `preview_help_surface_posture` proves every pillar but binds a Help/About surface
  still in preview, so it is narrowed to Preview by its lowest binding surface
  marker.
- **Discover / operate / recover from keyboard and mouse, no account.** Every
  record exposes `recovery_routes[]` (reveal raw source, copy safe representation,
  inspect codepoints, open content-safety help, export safe-preview support),
  `routes[]` for the activity center / command palette / status bar / menu command
  (all keyboard reachable, all activating the same posture), an `accessibility`
  block holding across layouts, and `available_without_account` +
  `available_without_managed_services`.

## Reproduce

```sh
# Stable corpus index — scenario id, surface class, claim, marker.
cargo run -q -p aureline-shell --bin aureline_shell_safe_preview_stable -- index

# Per-record plaintext truth blocks (support-export form).
cargo run -q -p aureline-shell --bin aureline_shell_safe_preview_stable -- plaintext

# Refresh the on-disk fixtures.
cargo run -q -p aureline-shell --bin aureline_shell_safe_preview_stable -- emit-fixtures \
  fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and

# Replay + invariant gate.
cargo test -p aureline-shell --test safe_preview_stable_fixtures
```

## Guardrails honored

No hover-only routes, no focus ambiguity, no toast-only truth, no hard-coded
theme/state semantics, and no public-scope widening from this row alone. The
trust-class ladder, suspicious-content detector, and representation-transfer
grammar are consumed from `aureline_content_safety` rather than re-spelled here. A
posture that proves a narrower claim than planned downgrades and names the reason
in the record rather than papering over the gap; the carrier-flatten,
boundary-not-shown, and reveal-missing drills keep the "no flattened warning, no
unguarded commit, no hidden raw source" promise enforceable in CI.
