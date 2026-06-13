# M5 Shared Suspicious-Text Detector Parity

This document is the contract for the shared suspicious-text detector parity
across the new M5 viewer and decision surfaces. The shared detector in
`aureline-content-safety` already governs the core source surfaces (editor,
diff, search, review); this lane extends the same detector, the same
threat-class cues, and the same raw-inspection affordances onto the new M5
surfaces so no surface is safer than another just because a different subsystem
renders the same bytes.

- Record kind: `m5_suspicious_text_detector_parity_packet`
- Schema: [`schemas/security/m5-suspicious-text-detector-parity.schema.json`](../../../schemas/security/m5-suspicious-text-detector-parity.schema.json)
- Canonical support export: [`artifacts/security/m5/m5_suspicious_text_detector_parity/support_export.json`](../../../artifacts/security/m5/m5_suspicious_text_detector_parity/support_export.json)
- Summary artifact: [`artifacts/security/m5/m5_suspicious_text_detector_parity.md`](../../../artifacts/security/m5/m5_suspicious_text_detector_parity.md)
- Fixtures: [`fixtures/security/m5/m5_suspicious_text_detector_parity/`](../../../fixtures/security/m5/m5_suspicious_text_detector_parity/)
- Producer: `aureline_content_safety::project_m5_suspicious_text_parity` /
  `frozen_m5_suspicious_text_parity_packet`
- Headless tool: `m5_suspicious_text_parity` (`--markdown`, `--clean`, `--validate <packet.json>`)

## Covered surfaces

The lane projects one shared detector run across exactly these new M5 surfaces:

| Surface | Display mode | Kind |
| --- | --- | --- |
| `notebook_output` | ordinary browsing | viewer |
| `docs_browser_panel` | ordinary browsing | viewer |
| `marketplace_install_update` | strong-decision strict identity | strong decision |
| `remote_host_attach` | strong-decision strict identity | strong decision |
| `collaboration_share` | strong-decision strict identity | strong decision |
| `ai_evidence_viewer` | ordinary browsing | viewer |
| `provider_policy_overlay` | strong-decision strict identity | strong decision |

The core source surfaces (editor, diff, search, review) keep their existing
projection in `aureline_content_safety::suspicious_text`; this lane does not
restate them.

## Shared threat-class vocabulary

Every surface maps the shared suspicious-content classes onto one threat-class
cue vocabulary, so a cue reads identically on a notebook cell, a marketplace
manifest, or a provider overlay:

| Content class | Threat class | Severity | Cue label |
| --- | --- | --- | --- |
| `bidi_control` | `text_reordering_spoof` | high | Text can display out of source order |
| `invisible_formatting` | `hidden_codepoint_smuggling` | high | Hidden characters between glyphs |
| `mixed_script_confusable` / `whole_script_confusable` | `identity_confusable_spoof` | critical | Identifier can impersonate another |
| `raw_rendered_divergence` | `rendered_source_divergence` | elevated | Rendered form differs from source |

Every cue in this vocabulary materially affects trust, which is why raw
inspection must stay reachable wherever any cue is shown.

## Invariants

`M5SuspiciousTextParityPacket::validate` enforces the lane invariants and fails
closed when any is broken:

- `surface_missing` — every new M5 surface must be present exactly once.
- `surfaces_disagree_on_classes` — every surface must expose the same
  content-class and threat-class set; one surface cannot drop or rename a
  warning the others show.
- `raw_inspection_unreachable` — wherever a cue materially affects trust, the
  warning must keep a raw reveal, an escaped reveal, a codepoint inspector, and
  a raw copy path reachable.
- `copy_choices_not_labeled` — a surface with warnings must expose a distinctly
  labeled raw copy and an escaped safe-representation copy; rendered copy never
  masquerades as raw bytes.
- `strong_decision_display_too_weak` — install/update, attach/share,
  collaboration, and policy-review surfaces must render in
  `strong_decision_strict_identity`, not ordinary browsing.
- `normalization_applied` — the projection never normalizes or strips suspicious
  bytes; matched codepoints survive verbatim in the raw snippet.
- `support_export_drops_cues` / `support_export_leaks_raw_bytes` — the
  support/admin export must preserve every threat-class cue and must carry only
  escaped exemplars, never raw suspicious bytes.

## Support/admin export

The `support_admin_export` block lets a support or admin reviewer preserve the
same threat-class cues without reproducing the warning in a specific pane. Each
summary carries the threat class, severity, cross-surface warning count, the
surfaces it appears on, an escaped exemplar, and the continuity refs that join
the same finding across surfaces. Re-running the detector over the export's
escaped strings must come back clean — this is the byte-leak guard.

## Scope

This lane delivers only the suspicious-text detector parity, threat-class cues,
raw-inspection affordances, and copy/export representation truth needed for the
claimed M5 viewer and decision surfaces. It does not implement a general browser
engine or media suite, and it does not restate the per-surface viewer
implementations that consume the packet.
