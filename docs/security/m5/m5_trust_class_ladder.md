# M5 Trust-Class Ladders, Downgrade Rules & Compare-Only Fallbacks

This document is the contract for the runtime trust class of every new M5
preview and embedded surface. The frozen content-integrity matrix
(`freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix`)
locks the *static* qualification each M5 artifact/viewer family may claim, and
the raw-versus-rendered handoff lane (`m5_raw_rendered_handoff`) keeps raw/rendered
copy and export honest. This lane covers the orthogonal *runtime* gap they leave
open: given a surface's requested trust class and the live trust signals around
it, what posture does the surface actually resolve to before content renders or
executes?

- Record kind: `m5_trust_class_ladder_packet`
- Schema: [`schemas/security/m5-trust-class-ladder.schema.json`](../../../schemas/security/m5-trust-class-ladder.schema.json)
- Canonical support export: [`artifacts/security/m5/m5_trust_class_ladder/support_export.json`](../../../artifacts/security/m5/m5_trust_class_ladder/support_export.json)
- Summary artifact: [`artifacts/security/m5/m5_trust_class_ladder.md`](../../../artifacts/security/m5/m5_trust_class_ladder.md)
- Fixtures: [`fixtures/security/m5/m5_trust_class_ladder/`](../../../fixtures/security/m5/m5_trust_class_ladder/)
- Producer: `aureline_content_safety::project_m5_trust_class_ladder` /
  `frozen_m5_trust_class_ladder_packet`
- Headless tool: `m5_trust_class_ladder` (`--markdown`, `--clean`, `--validate <packet.json>`)

## The trust-class ladder

Every preview/embedded surface climbs an explicit ladder. The rungs are the
closed trust-class vocabulary shared with
[`schemas/security/trust_class.schema.json`](../../../schemas/security/trust_class.schema.json):

| Rung | Meaning |
| --- | --- |
| `raw_text` | Plain raw text; no rendering, no active behavior. |
| `sanitized_rich` | Sanitized rich rendering with active content neutralized. |
| `trusted_local_active` | Active content that may run only inside the declared trusted-local class. |
| `isolated_remote_active` | Active remote content confined to an isolated runtime class. |

A surface declares the rung it *requests*; resolution returns the *effective*
rung. When the requested rung cannot be trusted, the effective posture is one of
two additional, terminal fallbacks rather than the requested rung:

| Fallback rung | Meaning |
| --- | --- |
| `compare_only` | Raw and rendered are shown side by side, both inert, so the rendered form never claims sole authority. |
| `blocked` | Rendering is blocked; only redaction-safe metadata is shown. |

`raw_text` always stays at the bottom of every surface's ladder, so raw
inspection and raw copy are reachable underneath any fallback.

## Covered surfaces

| Surface | Display mode | Embedded/review | Strong-decision |
| --- | --- | --- | --- |
| `notebook_rich_output` | ordinary browsing | no | no |
| `docs_browser_panel` | ordinary browsing | no | no |
| `ai_evidence_viewer` | ordinary browsing | yes | no |
| `pipeline_artifact_browser` | ordinary browsing | yes | no |
| `provider_overlay` | strong-decision strict identity | no | yes |
| `marketplace_install_review` | strong-decision strict identity | no | yes |
| `remote_preview_target` | strong-decision strict identity | no | yes |
| `structured_compare_view` | ordinary browsing | yes | no |

Strong-decision surfaces (install/update, attach/share, collaboration, and
policy review) always render owner and origin identity in the stricter
`strong_decision_strict_identity` mode. Embedded and review surfaces never
auto-execute active content: any active request degrades to sanitized visibility.

## Active-content downgrade rules

Resolution walks a fixed catalog of named rules, most restrictive first, and
takes the most severe fallback any fired rule demands. The catalog is carried in
every packet (`downgrade_rules`) so support, diagnostics, and release tooling
ingest it directly rather than cloning prose.

| Rule | Trigger | Forces |
| --- | --- | --- |
| `policy_block_forces_blocked` | `policy_blocked` | `blocked_metadata_only` |
| `unresolved_divergence_forces_compare_only` | `raw_rendered_divergence_unresolved` | `compare_only` |
| `safe_preview_unavailable_forces_compare_only` | `safe_preview_unavailable` | `compare_only` |
| `isolation_unavailable_downgrades_active_to_sanitized` | `isolation_runtime_unavailable` | `sanitized_visibility` |
| `local_trust_absent_downgrades_active_to_sanitized` | `local_trust_not_established` | `sanitized_visibility` |
| `suspicious_content_downgrades_active_to_sanitized` | `suspicious_content_detected` | `sanitized_visibility` |
| `proof_stale_narrows_active_to_sanitized` | `proof_stale` | `sanitized_visibility` |
| `embedded_review_surface_never_executes` | `embedded_review_surface` | `sanitized_visibility` |

Fallback severity orders `no_fallback` < `sanitized_visibility` < `compare_only`
< `blocked_metadata_only`. A policy block always wins; an unresolved divergence
or missing safe preview forces compare-only even if a sanitized downgrade also
fired.

## Compare-only fallback

When the rendered form cannot be trusted — its bytes diverge unresolved from the
raw source, or safe preview is unavailable — the surface degrades to a
compare-only view: raw and rendered are shown side by side, both inert, so the
rendered form is never the sole authority and active content never executes.
Every renderable surface keeps a reachable compare-only floor
(`compare_only_fallback_available`); only a fully `blocked` surface does not.

## Invariants

The producer guarantees, and `validate` enforces, that:

- Active content never executes outside its declared trust class: an executing
  posture only appears when the effective class is the matching active class.
- Unsafe or unsupported states degrade to sanitized or compare-only visibility,
  or to a blocked metadata view — never silent execution and never opaque
  failure. Every downgraded surface carries a fallback mode, an applied rule,
  and a rationale.
- Resolution never escalates a surface above the class it requested.
- Raw inspection and raw copy stay reachable on every surface; suspicious bytes
  are surfaced (annotated), never normalized away; rendered copy never
  masquerades as raw.
- Embedded and review surfaces never auto-execute; strong-decision surfaces use
  stricter identity rendering.

The packet is metadata only: no raw suspicious bytes, raw rendered trees, raw
provider payloads, or credentials cross the export boundary.

## Consumers

The headless `m5_trust_class_ladder` tool is the first CLI/headless consumer; it
emits the canonical support export, the Markdown summary, the clean fixture, and
validates any packet. Support, diagnostics, and release tooling read the
machine-readable packet and rule catalog directly.
