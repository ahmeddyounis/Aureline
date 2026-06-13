# M5 Suspicious-Content, Safe-Preview, and Representation-Labeled Copy/Export Matrix

This document is the contract for the frozen M5 matrix that qualifies content
integrity, safe preview, and representation-labeled copy/export across every new
M5 artifact and viewer family. The matrix is the canonical M5 control source for
this lane: notebook, docs/browser, AI-evidence, pipeline/artifact, provider,
marketplace, remote-preview, incident/export, generated-artifact, and structured
compare surfaces ingest the checked-in packet rather than cloning policy text.

- Record kind: `freeze_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix`
- Schema: [`schemas/security/freeze-the-m5-suspicious-content-safe-preview-and-representation-copy-export-matrix.schema.json`](../../../schemas/security/freeze-the-m5-suspicious-content-safe-preview-and-representation-copy-export-matrix.schema.json)
- Canonical support export: [`artifacts/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/support_export.json`](../../../artifacts/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/support_export.json)
- Summary artifact: [`artifacts/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix.md`](../../../artifacts/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix.md)
- Fixtures: [`fixtures/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/`](../../../fixtures/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/)
- Producer: `aureline_content_safety::frozen_stable_m5_content_integrity_matrix_packet`
- Headless tool: `m5_content_integrity_matrix` (`--markdown`, `--validate <packet.json>`)

## Artifact / viewer families

Each family row binds a qualification class to its trust-class ladder,
raw-versus-rendered posture, active-content policy, copy/export representation
semantics, safe-preview limited-mode posture, decision-strictness display mode,
evidence requirement, required evidence packet refs, downgrade triggers, source
contracts, and the consumer surfaces that must project the family's qualification
truth.

| Family | Qualification | Raw / rendered | Active content | Copy / export | Display |
| --- | --- | --- | --- | --- | --- |
| `notebook_rich_output` | Beta | raw + rendered, both reachable | isolated remote sandbox only | raw + escaped labeled | ordinary |
| `docs_browser_panel` | Stable | rendered default, raw on demand | isolated remote sandbox only | escaped default, raw reachable | ordinary |
| `ai_evidence_viewer` | Stable | raw + rendered, both reachable | inert, never executes | raw + escaped labeled | ordinary |
| `pipeline_artifact_browser` | Stable | rendered default, raw on demand | inert, never executes | escaped default, raw reachable | ordinary |
| `provider_overlay` | Stable | rendered default, raw on demand | inert, never executes | escaped default, raw reachable | strict |
| `marketplace_install_update` | Stable | raw + rendered, both reachable | blocked pending review | raw + escaped labeled | strict |
| `remote_preview_target` | Beta | rendered default, raw on demand | isolated remote sandbox only | escaped default, raw reachable | strict |
| `incident_export_packet` | Stable | raw only, no rendering | inert, never executes | metadata only, no raw body | ordinary |
| `generated_artifact` | Beta | raw + rendered, both reachable | inert, never executes | raw + escaped labeled | ordinary |
| `structured_compare_view` | Stable | raw + rendered, both reachable | inert, never executes | raw + escaped labeled | ordinary |

## Track invariant

One shared suspicious-text and content-integrity policy library governs all
claimed M5 artifact and viewer surfaces; trust-decision surfaces use a stricter
display mode; raw inspection and raw copy remain reachable whenever the rendered
form differs materially; active content never executes outside its declared trust
class; and bidi/invisible/confusable fixes never rewrite bytes silently during
save, format, organize-imports, or AI apply. The `trust_review` block encodes
these as hard invariants — all must hold for the matrix to validate:

- `one_shared_policy_library_governs_all_families` and
  `trust_decision_surfaces_use_stricter_display_mode` — every family maps to the
  same suspicious-content/trust-class vocabulary, and install/update,
  attach/share, collaboration, and policy-review surfaces render owner and origin
  identity more strictly than ordinary browsing panes.
- `raw_inspection_and_copy_reachable_on_divergence` and
  `rendered_copy_never_masquerades_as_raw` — whenever raw and rendered forms can
  differ, both a raw inspection path and a distinctly labeled raw copy/export path
  stay reachable, and rendered copy is never presented as raw bytes.
- `active_content_never_executes_outside_trust_class` and
  `no_auto_execute_in_embedded_or_review_surfaces` — active rich content runs only
  in its declared trusted-local or isolated-remote class, and embedded/review
  surfaces (AI evidence, incident/export, generated artifacts, structured compare)
  keep it inert.
- `bidi_invisible_confusable_never_silently_rewritten` and
  `suspicious_bytes_not_normalized_away` — suspicious codepoints are annotated and
  escapable, never silently normalized out of the bytes on save, format,
  organize-imports, or AI apply.
- `downgrade_narrows_instead_of_hides` and
  `stale_or_underqualified_blocks_promotion` — a narrowed family shows a smaller
  claim rather than disappearing, and stale or underqualified rows block
  promotion automatically.

These map directly to enforced rules in
`M5ContentIntegrityMatrixPacket::validate`: a family with a divergent raw/rendered
posture must keep a raw copy/export path (`raw_copy_unreachable_on_divergence`),
strong-decision families must use strict display mode
(`strong_decision_display_mode_too_weak`), and embedded/review surfaces may not
allow active content to execute (`active_content_in_review_surface`).

## Consumer projection

The `consumer_projection` block records that every claimed consumer surface
projects this matrix's truth rather than cloning it: notebooks show trust class
and active-content state; docs/browser panels show raw/rendered and safe-preview
state; AI evidence viewers show representation labels; marketplace install/update
uses strict identity rendering; copy/export affordances label raw versus rendered;
and CLI/headless, support export, diagnostics, and Help/About all show
qualification truth. Families not covered by this packet are visibly labeled as
Preview/Labs.

## Source contracts

The matrix references upstream content-integrity contracts by id rather than
embedding them:

- [`schemas/security/trust_class.schema.json`](../../../schemas/security/trust_class.schema.json) — closed safe-preview trust-class vocabulary.
- [`schemas/security/text_representation_policy.schema.json`](../../../schemas/security/text_representation_policy.schema.json) — raw/rendered text-representation policy.
- [`schemas/trust/safe-preview-trust-class.schema.json`](../../../schemas/trust/safe-preview-trust-class.schema.json) — stabilized safe-preview trust-class contract.
- [`schemas/content/representation_export.schema.json`](../../../schemas/content/representation_export.schema.json) — representation-labeled export contract.

## Proof freshness and narrowing

`proof_freshness` carries a 168-hour SLO with `auto_narrow_on_stale` set, so a
stale proof packet narrows the affected family automatically before publication.
The narrowed fixtures under the fixtures directory demonstrate held and
preview-narrowed families while every other family stays at its canonical
qualification. Release and support tooling validate the checked-in export and the
narrowed fixtures with `M5ContentIntegrityMatrixPacket::validate` (via
`current_stable_m5_content_integrity_matrix_export` and the
`m5_content_integrity_matrix --validate` tool), so a missing family, an
unqualified strong-decision surface, an unreachable raw copy, or an executing
review surface fails closed.

## Scope

This matrix freezes only the content-integrity, safe-preview, raw/rendered, and
copy/export representation truth needed for claimed M5 viewers and trust surfaces.
It does not implement a general browser engine or media suite, and it does not
restate the per-family viewer implementations that consume it.
