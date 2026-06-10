# Scoped Browser Surfaces (docs and review)

This document is the contract for the M5 scoped-browser-surface boundary — the
surface that hands the reader off to a *narrow* browser view without ever
looking like a general web-mode or full browser runtime. Three qualified scopes
are explained through one shared vocabulary:

- a **docs reading** surface — a scoped handoff to vendor / mirrored / live docs;
- a **review** surface — a scoped handoff to a hosted review thread or diff;
- a **light edit** surface — a scoped editor handoff for a small local edit.

Every surface carries the same source/version/freshness/locality/confidence chip
set the other docs lanes use, an explicit **handoff reason** (*why* the product
handed off), a non-empty **return path** (*how the reader gets back* — the
return-path safety guarantee), one **trust-class disclosure** (*how trustworthy
the destination is*), the handoff-capability posture, the live-vs-captured
state, citation state, and the open-raw / open-source escapes. A surface whose
trust class cannot back an authoritative claim (`live_provider_handoff`,
`derived_inference_only`) may not be presented at high confidence, a handoff
blocked by policy may not be presented as available, and every surface must keep
a return path — so a scoped browser surface never reads as more authoritative,
more available, or more escapable than it is. An export preserves the scope /
handoff-reason / return-path / trust-class / source / confidence / escape truth
that support, AI evidence, and diagnostics surfaces ingest rather than cloning
status text. The docs browser shell, review surface, browser handoff packet,
peek overlay, AI-context inspector, CLI/headless output, support exports,
diagnostics, and Help/About all consume the checked-in packet.

- Record kind: `scoped_browser_surfaces_for_docs_and_review`
- Schema: [`schemas/docs/implement-scoped-browser-surfaces-for-docs-and-review-with-handoff-reason-return-path-and-trust-class-disclosu.schema.json`](../../../schemas/docs/implement-scoped-browser-surfaces-for-docs-and-review-with-handoff-reason-return-path-and-trust-class-disclosu.schema.json)
- Canonical support export: [`artifacts/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/support_export.json`](../../../artifacts/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/support_export.json)
- Summary artifact: [`artifacts/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu.md`](../../../artifacts/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu.md)
- Fixtures: [`fixtures/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/`](../../../fixtures/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/)
- Producer: `aureline_docs::current_stable_scoped_browser_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_scoped_browser_surfaces`

## The surfaces and their chips

`surfaces` is the set of scoped browser handoffs for one session. Every surface
points at a `subject_ref`, carries a `scope` (`docs_reading`, `review`,
`light_edit`), a `title`, a `headline`, and a `chips` block — the five chips a
consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `project_docs`, `mirrored_official_docs`, `extension_docs_pack`, `live_external_docs`, `review_host`, `generated_reference`, `derived_explanation` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `locality` | `local`, `mirrored_pack`, `remote_helper`, `managed` |
| `confidence` | `high`, `medium`, `low`, `heuristic` |

Every packet must include at least one surface on each of the `docs_reading` and
`review` scopes — a partial set (docs without review, say) is
`required_scope_missing` and blocks promotion, so the surface stays the qualified
docs + review boundary rather than a slice that overstates coverage.

## Scope stays narrow

Each surface carries one `scope`. Only `docs_reading`, `review`, and
`light_edit` are inside the qualified M5 scope. A surface that declares a
`general_web` or `full_browser_runtime` scope is `surface_scope_out_of_bounds`
and blocks promotion — the boundary never broadens general web-mode or
browser-runtime claims beyond the narrow docs / review / light-edit surfaces.

## The handoff reason

Each surface carries a `handoff_reason` — the explicit, human-readable reason the
product handed off, with a `reason_kind`:

| Reason | Meaning |
| --- | --- |
| `exact_anchor_unavailable_locally` | The exact anchor is only on the upstream page. |
| `live_version_newer_than_mirror` | The live upstream version is newer than the mirror. |
| `source_not_mirrored` | The content is not mirrored; only the upstream has it. |
| `review_thread_requires_hosted_view` | A review thread requires the hosted review view. |
| `light_edit_requires_scoped_editor` | A light edit requires a scoped editor surface. |
| `user_requested_open_in_browser` | The reader explicitly asked to open in a browser. |

A surface with an empty handoff-reason note is `handoff_reason_missing` and
blocks promotion.

## The return path (return-path safety)

Each surface carries a `return_path` — where the reader returns to when leaving
the scoped browser surface, with a `return_kind` (`back_to_inline_peek`,
`back_to_docs_browser`, `back_to_review_panel`, `back_to_workspace`), a stable
`return_ref`, and a human-readable `label`. A surface that drops its return path
is `return_path_missing` and blocks promotion — every scoped browser surface
stays return-path safe.

## The trust-class disclosure

Each surface carries one `trust_class`:

| Trust class | Meaning |
| --- | --- |
| `first_party_authoritative` | First-party authoritative content (the workspace's own docs). |
| `signed_mirror_verified` | A pinned, signed mirror of upstream docs. |
| `extension_pack_signed` | A signed extension / imported docs pack. |
| `live_provider_handoff` | A live provider handoff — not verified at materialization. Must stay cited. |
| `derived_inference_only` | Derived / inferred content only. Must stay cited. |

A surface whose trust class cannot back an authoritative claim
(`live_provider_handoff`, `derived_inference_only`) presented at `high`
confidence is `trust_class_disclosure_collapsed` and blocks promotion. An
untrusted destination that is not cited is `surface_not_cited`. A surface with an
empty trust-disclosure note is `trust_class_disclosure_missing`.

## Capability and live-vs-captured

`handoff_capability` (`not_required_local`, `available_explicit`,
`blocked_by_policy`, `unavailable_disclosed`) records whether the handoff can be
taken, and `captured_vs_live` (`live`, `captured_snapshot`,
`narrowed_scope_rerun`) records what the reader is actually looking at. A handoff
that is `blocked_by_policy` but presented `live` is
`blocked_handoff_presented_available` and blocks promotion.

## The export

`export` is the cited projection support, AI evidence, and diagnostics surfaces
ingest. It preserves the surface scope, handoff reason, return path, trust class,
source class, confidence, and escapes (the `preserves_*` flags), and carries one
`export_row` per surface. An export that drops a preservation flag, references an
unknown surface, drops a surface's row, or disagrees with a surface's scope /
trust class / source class / confidence / return-path presence blocks promotion.

## Degradations and promotion state

`browser_degradations` carry packet-level downgrades
(`mirror_offline_snapshot`, `live_provider_unreachable_captured_snapshot`,
`handoff_blocked_by_policy`, `return_path_degraded`, `scope_narrowed_rerun`,
`broken_anchor`, `quarantined_source`) with a `severity`. The computed
`promotion_state` is:

- `stable` — no findings and no narrowing/blocking degradation;
- `narrowed_below_stable` — an otherwise-clean set carries a narrowing
  degradation, so the claim narrows rather than hiding the surfaces;
- `blocks_stable` — any blocking validation finding or blocking degradation.

## Boundary

Raw page bodies, raw URLs, raw review payloads, raw source files, raw provider
payloads, and credentials never cross this boundary. The packet carries only
metadata, scope truth, handoff reasons, return paths, trust disclosures, chip
truth, cited refs, provenance, finding summaries, and contract refs;
`raw_boundary_material_present` blocks promotion if forbidden material is found.
