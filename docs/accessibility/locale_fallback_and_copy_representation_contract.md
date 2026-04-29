# Locale Fallback And Copy Representation Contract

Status: seeded

This contract freezes the accessibility-facing row model for locale
fallback, suspicious mixed-direction text, invisible characters, and
representation-labeled copy or export actions. It is the row-level
bridge between the localization contract, the safe-preview trust-class
contract, the accessibility packet template, and dense product surfaces
that must explain whether a user is seeing raw text, rendered text,
escaped text, a sanitized snapshot, or metadata only.

Contract identity:

- `accessibility_locale_copy_contract_id:
  aureline.accessibility.locale_copy_representation`
- `accessibility_locale_copy_contract_revision: 1`
- `locale_fallback_row_schema_version: 1`
- `text_representation_action_schema_version: 1`

Companion artifacts:

- [`/schemas/accessibility/locale_fallback_row.schema.json`](../../schemas/accessibility/locale_fallback_row.schema.json)
  defines `locale_fallback_row_record` and
  `mixed_direction_text_inspector_record`.
- [`/schemas/security/text_representation_action.schema.json`](../../schemas/security/text_representation_action.schema.json)
  defines one representation action row with availability, defaulting,
  payload posture, and disclosure rules.
- [`/fixtures/accessibility/representation_review_cases/`](../../fixtures/accessibility/representation_review_cases/)
  contains review fixtures for locale-pack fallback, bidi controls,
  invisible characters, malformed text, and rendered-versus-raw
  divergence.
- [`/docs/ux/localization_and_locale_pack_contract.md`](../ux/localization_and_locale_pack_contract.md)
  remains the source of truth for message ids, locale packs, fallback
  chain resolution, and source-language escape hatches.
- [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  remains the source of truth for trust classes and transfer-safe
  representation vocabulary.
- [`/docs/accessibility/a11y_ime_packet_template.md`](./a11y_ime_packet_template.md)
  consumes these rows for accessibility, IME, bidi, and copy-parity
  evidence.

Normative source anchors:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Section 19.7 defines the
  input-method state pill, mixed-direction inspector, locale fallback
  row, and copy / normalization review.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Section 23.41 requires
  mixed-direction technical-content fixtures, raw / rendered / escaped
  copy audits, locale-pack fallback tests, and high-zoom or
  screen-reader checks.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Sections
  11.2 and 11.8 require Unicode source fidelity, targeted suspicious
  text warnings, raw-byte inspection, and explicit raw versus rendered
  copy actions.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Section
  23.3.1 and `.t2/docs/Aureline_Technical_Design_Document.md`
  Section 8.10 require visible locale fallback chains and
  source-language escape hatches.
- `.t2/docs/Aureline_PRD.md` Sections 5.29 and 10.16 require bidi,
  invisible, and rendered-versus-raw ambiguity safeguards without
  silently normalizing source text.

## Scope

In scope:

- locale fallback rows in dense surfaces, settings, docs previews,
  reports, support exports, and assistive-technology captures;
- mixed-direction and invisible-text inspector rows for editor, diff,
  docs preview, install review, terminal transcript, support export,
  and security review surfaces;
- representation action rows for `copy_raw`, `copy_rendered`,
  `copy_escaped`, `export_sanitized_snapshot`, and
  `export_metadata_only`;
- cross-surface label parity so the same action means the same thing in
  editor, diff, docs preview, install review, terminal transcript,
  support export, and security review;
- high-zoom, dense-layout, keyboard-only, and screen-reader visibility
  guarantees for fallback and suspicious-text rows.

Out of scope:

- final locale-pack build tooling;
- final translation workflow;
- final Unicode detector heuristics;
- final visual styling of badges, markers, or sheets.

## Locale Fallback Row

A locale fallback row tells the user and assistive technology which
language source is currently active. It is shown when the requested
locale is fully localized, partially filled from a base or source
language, blocked by policy or signature failure, stale, or missing.

Minimum fields:

| Field | Meaning |
|---|---|
| `requested_locale` | BCP-47 locale the user or surface requested. |
| `active_locale` | BCP-47 locale actually contributing the visible string or row. |
| `source_language_locale` | Canonical source-language locale that remains available for inspection. |
| `active_locale_source` | Whether content came from the requested pack, a base pack, source language, a mirrored snapshot, or policy fallback. |
| `localization_state` | `fully_localized`, `partial_fallback`, `blocked_source_language_only`, `stale_pack_fallback`, `missing_pack_fallback`, or `signature_failed_fallback`. |
| `fallback_chain` | Ordered locale chain walked by the resolver, beginning with the request and ending at the active locale or source language. |
| `pack_freshness` | Pack ref, revision ref, freshness class, checked timestamp, and short freshness label. |
| `affected_string_counts` | Counts for localized, fallback, blocked, and total strings within the row scope. |
| `source_language_action` | The `open_in_source_language` action, availability, defaulting posture, command ref, and disabled reason if any. |
| `visibility_guarantees` | Dense layout, high zoom, keyboard-only, and assistive-technology availability guarantees. |
| `assistive_access` | Accessible name, short announcement, keyboard route, and live-region policy. |

Rules:

- Any non-authoritative fallback MUST keep `source_language_action`
  available unless policy explicitly blocks source-language viewing.
- A stale, missing, signature-failed, or policy-disabled pack MUST NOT
  render as if it were the requested locale.
- Pack freshness is part of the row, not a tooltip-only detail. Dense
  layout may abbreviate it, but the accessible description and inspector
  must retain it.
- Source-language fallback is acceptable for continuity, but the row
  must say whether it is partial, blocked, stale, missing, or
  signature-failed.
- The row must be reachable by keyboard and assistive technology in the
  same workflow where the localized or fallback text appears.

## Mixed-Direction And Invisible-Text Inspector

The mixed-direction inspector explains text whose rendered order or
visible shape can diverge from the stored source. It covers bidi
controls, invisible formatting characters, malformed or replacement
text, codepoint classes, and literal technical spans.

Minimum fields:

| Field | Meaning |
|---|---|
| `subject_ref` | Opaque ref to the token, row, hunk, transcript segment, package id, or support-export item. |
| `surface_family` | Surface where the inspector row is shown. |
| `directionality_note` | Short human-readable directionality summary. |
| `base_direction` | `ltr`, `rtl`, `mixed`, `neutral`, or `unknown`. |
| `control_character_count` | Total bidi, formatting, or replacement controls that materially affect review. |
| `codepoint_class_summary` | Counts by class such as `bidi_control`, `invisible_formatting`, `malformed_decode_replacement`, `mixed_script_confusable`, and `literal_code_or_path_span`. |
| `raw_view_action` | `open_raw_view` or `copy_raw` path, with availability and defaulting state. |
| `escaped_view_action` | `open_escaped_view` or `copy_escaped` path, with availability and defaulting state. |
| `path_code_span_preservation` | Whether paths, command flags, hostnames, code spans, command ids, or package ids remain literal and unmirrored. |
| `normalization_posture` | Whether bytes are preserved, only escaped for inspection, or require a previewed normalization fix. |
| `visibility_guarantees` | Dense layout, high zoom, keyboard-only, and assistive-technology availability guarantees. |

Rules:

- The inspector annotates and preserves suspicious characters. It does
  not silently normalize, delete, or rewrite them.
- Raw and escaped inspection stay available wherever bidi controls,
  invisible formatting, malformed text, or raw/rendered divergence can
  mislead review.
- Literal technical spans are not mirrored by RTL layout. Paths,
  command flags, hostnames, code spans, command ids, and package ids
  remain copyable as authored.
- The codepoint summary may use escaped samples such as `U+202E` or
  `\\u200B`; it must not require raw private workspace material in
  support exports.

## Representation Actions

Representation actions name what leaves the product. They are separate
from trust class, surface family, and the current visual style.

| Action id | Canonical label | Representation | Payload posture | Default rule |
|---|---|---|---|---|
| `copy_raw` | `Copy raw` | raw | exact source bytes or exact source text representation | Primary on editor, diff, install review, terminal transcript, and security review when source exists. |
| `copy_rendered` | `Copy rendered` | rendered | current rendered view | Primary on docs preview and sanitized rich previews; secondary wherever raw/rendered divergence exists. |
| `copy_escaped` | `Copy escaped` | escaped | source representation with controls and metacharacters escaped for inspection | Required wherever suspicious content, malformed text, or bidi/invisible controls are present. |
| `export_sanitized_snapshot` | `Export sanitized snapshot` | sanitized snapshot | static snapshot with active/scriptable content removed | Primary support/export path for rich or active content when a truthful snapshot can be produced. |
| `export_metadata_only` | `Export metadata only` | metadata only | envelope with body withheld | Fallback default when policy, redaction, missing source bytes, origin loss, or security review blocks the body. |

Rules:

- Generic `Copy` or `Export` is non-conforming when representations
  differ materially.
- `copy_rendered` on a raw/rendered-divergent surface MUST keep
  `copy_raw` visible in the same flow.
- `copy_escaped` is not optional decoration when suspicious content is
  present; it is the safe review path.
- `export_sanitized_snapshot` must say that active content was removed
  and preserve owner/origin and snapshot-age metadata when applicable.
- `export_metadata_only` must disclose the body-withheld reason and
  the last known trust or representation posture.

## Cross-Surface Parity

Every claiming surface uses the same action ids and canonical labels.
Surface-specific labels may add a noun, but they must preserve the
canonical stem:

- `Copy raw line`
- `Copy rendered preview`
- `Copy escaped publisher id`
- `Export sanitized transcript snapshot`
- `Export metadata only`

Parity requirements:

| Surface | Default label posture | Required secondary paths |
|---|---|---|
| Editor | `Copy raw` is primary for exact source text. | `Copy escaped` when suspicious characters or malformed text are present. |
| Diff | `Copy raw` is primary for hunk text. | `Copy escaped` and inspector access when visual order diverges. |
| Docs preview | `Copy rendered` is primary for the preview. | `Copy raw` when source exists; `Copy escaped` when suspicious content is present. |
| Install review | `Copy raw` is primary for publisher, package, artifact, and version identifiers. | `Copy escaped`; metadata-only export when body export is unsafe. |
| Terminal transcript | `Copy raw` is primary for transcript evidence. | `Copy rendered` only when ANSI-stripped rendering is explicit; `Copy escaped` for controls or decode recovery. |
| Support export | `Export sanitized snapshot` is primary when a body can leave safely. | `Export metadata only` when policy, redaction, source loss, or origin loss blocks the body. |
| Security review | `Copy raw` or `Export metadata only` is primary depending on policy. | `Copy escaped` for suspicious identifiers; sanitized snapshot only if active content is removed and provenance survives. |

## Dense, Zoom, And Assistive-Technology Rules

- The locale fallback row and text inspector remain reachable in dense
  layout. Surfaces may collapse them to a compact chip, but keyboard
  route, accessible name, and inspection command must remain.
- At high zoom, the row may wrap or move to an inspector sheet. It must
  not disappear behind hover-only UI.
- Assistive-technology output names the representation before the
  action completes. For example, a screen reader must announce that the
  selected command is `Copy escaped`, not merely `Copy`.
- Live-region updates are polite by default. They become assertive only
  when a representation or locale fallback change affects an active
  approval, install, delete, or security decision.
- Support and accessibility exports are metadata-first. They may carry
  opaque refs, counts, codepoint class summaries, escaped samples,
  labels, and redaction reasons. They do not need raw private source
  text to prove the contract.

## Fixture Coverage

The fixture corpus covers:

- locale-pack fallback with partial source-language fill;
- bidi controls in mixed-direction diff review text;
- invisible formatting characters in an install-review identifier;
- malformed terminal transcript text narrowed to metadata-only export;
- rendered-versus-raw divergence in docs preview copy actions;
- sanitized snapshot export from support review;
- raw and escaped copy parity for editor and security-review flows.

Each fixture uses a `# yaml-language-server: $schema=...` header that
points at the boundary schema for the record under review.
