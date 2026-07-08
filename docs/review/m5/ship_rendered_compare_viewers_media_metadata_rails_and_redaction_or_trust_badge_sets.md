# Ship rendered compare viewers, media-metadata rails, and redaction-or-trust badge sets

Status: Implemented (M05-968, batch B114)

This contract narrows the `rendered_compare_viewer`, `media_metadata_rail`, and
`redaction_or_trust_badge_set` components frozen in
[`m5-structured-artifact-review-component-matrix`](freeze_the_m5_structured_artifact_review_component_matrix.md)
(M05-964) into implemented, export-safe review controls. It makes visual and
media-like artifact review safe, inspectable, and share-aware: a rendered report
or image-like artifact is never reviewed without its accessibility or trust
boundary, and share/export flows preserve the redaction or trust posture rather
than flattening rendered review into ambiguous attachments.

- Boundary schema: [`schemas/ui/m5-rendered-compare-media-trust-controls.schema.json`](../../../schemas/ui/m5-rendered-compare-media-trust-controls.schema.json)
- Producer: `aureline_review::current_media_compare_controls_export`
- Release proof: [`artifacts/release/m5-rendered-compare-media-trust-controls-proof/`](../../../artifacts/release/m5-rendered-compare-media-trust-controls-proof/)
- Protected fixtures: [`fixtures/ui/m5-rendered-compare-media-trust-controls/`](../../../fixtures/ui/m5-rendered-compare-media-trust-controls/)

## What the components carry

Every `RenderedCompareViewer` reuses the frozen `M5ArtifactComponent` tag and
answers, from the viewer alone:

- **Trust class** (`trust_class`: `sandboxed_trusted` / `sandboxed_untrusted` /
  `raw_text_fallback` / `redacted_withheld`) — a render never hides whether it was
  sandboxed and whether it is trusted.
- **Scale or dimension metadata** (`scale_or_dimension_metadata`, required).
- **Accessibility text fallback** (`alt_text_fallback`, required) — review never
  loses accessibility.
- Trust-derived notes (`sandbox_note`, `untrusted_render_note`,
  `raw_fallback_label`, `redaction_note`), each required for the state that needs
  it.
- **Actions** (`available_actions`), which must include `open_raw` and `export`,
  plus a required `raw_context_action` and the reused `schema_fidelity` and
  `rollback_posture`.

Every `MediaMetadataRail` names the media boundary:

- **Artifact kind** (`artifact_kind`) and **format** (`format_label`, required).
- **Measure** (`measure_kind`: `dimensions` / `duration` / `byte_size` /
  `page_count`) and **measure value** (`measure_value`, required) — size,
  duration, or dimensions.
- **Hidden-content state** (`hidden_content_state`:
  `no_embedded_sensitive_content` / `embedded_sensitive_content_present` /
  `embedded_content_scan_unknown`) and a **hidden-content note**
  (`hidden_content_note`), required when content is present or unknown.
- **Safety posture** (`safety_posture`: `raw_unsanitized` / `sanitized` /
  `sandboxed` / `export_safe`) and **share scope** (`share_scope`: `local_only` /
  `team_share` / `support_export`) with required **share guidance**.

Every `RedactionOrTrustBadgeSet` names the redaction and trust boundary:

- **Redaction state** (`redaction_state`) and **trust level** (`trust_level`).
- **Badges** (`available_badges`) and state-derived notes (`redaction_note`,
  `untrusted_note`).
- **Export posture preservation** (`export_posture_preserved`), which must be
  `true`, and required **share guidance**.

## Derived honesty (the delta this lane enforces)

Rendered-viewer disclosure is *derived* from the trust class by
`resolve_rendered_viewer_disclosure`:

- a `sandboxed_trusted` or `sandboxed_untrusted` render must carry a sandbox note
  (`sandbox_note_missing`);
- a `sandboxed_untrusted` render must carry an untrusted-render note
  (`untrusted_render_note_missing`);
- a `raw_text_fallback` render must explicitly label the raw fallback
  (`raw_fallback_label_missing`) so a structured render is never silently
  flattened to raw; and
- a `redacted_withheld` render must carry a redaction note
  (`render_redaction_note_missing`).

Every viewer must carry a non-empty `scale_or_dimension_metadata`
(`scale_or_dimension_metadata_missing`) and `alt_text_fallback`
(`alt_text_fallback_missing`), and must offer both `open_raw`
(`open_raw_action_missing`) and `export` (`export_action_missing`).

Media-rail disclosure is derived from the hidden-content state by
`resolve_media_rail_disclosure`: a rail whose content is present or unknown must
carry a hidden-content note (`hidden_content_note_missing`). A rail that would
share embedded sensitive content beyond the local boundary without a sanitized or
export-safe posture fails `unsanitized_hidden_content_shareable`.

Badge-set disclosure is derived from the redaction state and trust level by
`resolve_badge_set_disclosure`: a partially/fully-redacted or pending state must
carry a redaction note (`badge_redaction_note_missing`), and an untrusted or
unverified level must carry an untrusted note (`untrusted_badge_note_missing`).
Every badge set must preserve its redaction/trust posture on share or export
(`export_posture_not_preserved`).

Every rendered compare viewer and every media-metadata rail must be accompanied by
a redaction-or-trust badge set for the same `artifact_ref`
(`trust_badge_set_missing`), so the redaction and trust posture is always visible
where a media-like artifact is reviewed, shared, or exported.

## Coverage and invariants

- The viewers must cover the `sandboxed_trusted`, `sandboxed_untrusted`, and
  `raw_text_fallback` classes (`render_trust_class_coverage_missing`).
- The rails must cover the `no_embedded_sensitive_content`,
  `embedded_sensitive_content_present`, and `embedded_content_scan_unknown` states
  (`hidden_content_state_coverage_missing`).
- The badge sets must cover the `not_redacted`, `partially_redacted`, and
  `fully_redacted` states (`redaction_state_coverage_missing`).
- The trust-review and consumer-projection blocks assert that render trust is
  always explicit, an accessibility fallback is always present, scale/dimension
  metadata is present, hidden-content state is disclosed, sanitized/export-safe
  posture stays explicit, share guidance is explicit, redaction/trust posture is
  preserved on export, raw context is always reachable, and downgrade narrows
  instead of hiding.

Raw artifact bodies, raw render payloads, raw media bytes, credentials, and live
provider responses never cross this boundary; the export is metadata-only and
screened by an export-material heuristic (`raw_boundary_material_in_export`).
