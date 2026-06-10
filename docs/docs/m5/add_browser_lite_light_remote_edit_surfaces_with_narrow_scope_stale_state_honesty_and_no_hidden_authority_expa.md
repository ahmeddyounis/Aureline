# Light Remote Edit Surfaces (narrow scope, stale-state honesty, no hidden authority expansion)

This document is the contract for the M5 light-remote-edit boundary — the
browser-lite surface that lets the reader make a *small, scoped* edit without
ever looking like a full editor, a multi-file refactor, or a general automation
runtime. Four qualified scopes are explained through one shared vocabulary:

- a **doc-comment edit** — a scoped edit to a documentation comment;
- a **single-file text edit** — a scoped edit to one file;
- a **config-value edit** — a scoped edit to a single configuration value;
- a **review reply** — a scoped reply to a review comment.

Every surface carries the same source/version/freshness/locality/confidence chip
set the other docs lanes use, an explicit **edit intent** (*why* the edit was
offered), a non-empty **return path** (*how the reader gets back* — the
return-path safety guarantee), one **trust-class disclosure** (*how trustworthy
the destination is*), a **base-state disclosure** (*what state the edit was
prepared against, and whether a stale base is disclosed* — the stale-state
honesty guarantee), a granted-vs-effective **authority** pair (*the
no-hidden-authority-expansion guarantee*), the apply posture, the live-vs-captured
state, citation state, and the open-raw / open-source escapes. An export
preserves the scope / edit-intent / return-path / trust-class / source /
confidence / authority / stale-state / escape truth that support, AI evidence,
and diagnostics surfaces ingest rather than cloning status text. The docs
browser shell, review surface, light-edit surface, peek overlay, AI-context
inspector, CLI/headless output, support exports, diagnostics, and Help/About all
consume the checked-in packet.

- Record kind: `light_remote_edit_surfaces`
- Schema: [`schemas/docs/add-browser-lite-light-remote-edit-surfaces-with-narrow-scope-stale-state-honesty-and-no-hidden-authority-expa.schema.json`](../../../schemas/docs/add-browser-lite-light-remote-edit-surfaces-with-narrow-scope-stale-state-honesty-and-no-hidden-authority-expa.schema.json)
- Canonical support export: [`artifacts/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/support_export.json`](../../../artifacts/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/support_export.json)
- Summary artifact: [`artifacts/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa.md`](../../../artifacts/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa.md)
- Fixtures: [`fixtures/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/`](../../../fixtures/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/)
- Producer: `aureline_docs::current_stable_light_remote_edit_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_light_remote_edit_surfaces`

## The surfaces and their chips

`surfaces` is the set of light remote edit surfaces for one session. Every
surface points at a `subject_ref`, carries a `scope` (`doc_comment_edit`,
`single_file_text_edit`, `config_value_edit`, `review_reply`), a `title`, a
`headline`, and a `chips` block — the five chips a consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `project_docs`, `mirrored_official_docs`, `extension_docs_pack`, `live_provider_surface`, `review_host`, `generated_reference`, `derived_suggestion` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `locality` | `local`, `mirrored_pack`, `remote_helper`, `managed` |
| `confidence` | `high`, `medium`, `low`, `heuristic` |

Every packet must include at least one surface on each of the `doc_comment_edit`
and `single_file_text_edit` scopes — a partial set is `required_scope_missing`
and blocks promotion, so the surface stays the qualified light-edit boundary
rather than a slice that overstates coverage.

## Scope stays narrow

Each surface carries one `scope`. Only `doc_comment_edit`,
`single_file_text_edit`, `config_value_edit`, and `review_reply` are inside the
qualified M5 scope. A surface that declares a `multi_file_refactor`,
`repo_wide_automation`, or `arbitrary_command_execution` scope is
`surface_scope_out_of_bounds` and blocks promotion — the boundary never broadens
into a full editor, a multi-file refactor, or a general automation runtime.

## The edit intent

Each surface carries an `edit_intent` — the explicit, human-readable reason the
product offered the edit, with an `intent_kind`:

| Intent | Meaning |
| --- | --- |
| `fix_doc_typo` | Fix a typo in a doc comment. |
| `adjust_config_value` | Adjust a single configuration value. |
| `apply_review_suggestion` | Apply a suggestion raised in review. |
| `reply_to_review_comment` | Reply to a review comment. |
| `small_inline_correction` | A small inline correction. |

A surface with an empty edit-intent note is `edit_intent_missing` and blocks
promotion.

## The return path (return-path safety)

Each surface carries a `return_path` — where the reader returns to when leaving
the light edit surface, with a `return_kind` (`back_to_inline_peek`,
`back_to_docs_browser`, `back_to_review_panel`, `back_to_workspace`), a stable
`return_ref`, and a human-readable `label`. A surface that drops its return path
is `return_path_missing` and blocks promotion — every light remote edit surface
stays return-path safe.

## The trust-class disclosure

Each surface carries one `trust_class`:

| Trust class | Meaning |
| --- | --- |
| `first_party_workspace` | First-party workspace content (the workspace's own files). |
| `signed_mirror_backed_suggestion` | A pinned, signed mirror-backed suggestion. |
| `extension_pack_suggestion` | A signed extension / imported pack suggestion. |
| `live_provider_edit_surface` | A live provider edit surface — not verified at materialization. Must stay cited. |
| `derived_suggestion_only` | A derived / inferred suggestion only. Must stay cited. |

A surface whose trust class cannot back an authoritative claim
(`live_provider_edit_surface`, `derived_suggestion_only`) presented at `high`
confidence is `trust_class_disclosure_collapsed` and blocks promotion. An
untrusted destination that is not cited is `surface_not_cited`. A surface with an
empty trust-disclosure note is `trust_class_disclosure_missing`.

## Stale-state honesty

Each surface carries a `stale_state` disclosure — the `base_state_kind` the edit
was prepared against (`live_head`, `warm_snapshot`, `stale_snapshot`,
`unknown_base`), a `disclosed` flag, and a `note`. A `stale_snapshot` or
`unknown_base` edit that is not disclosed (or carries an empty note) is
`stale_state_not_disclosed` and blocks promotion, and a stale or unknown base
presented at `high` confidence is `stale_state_presented_confident` — a light
remote edit never hides that it was prepared against a stale view, and never
presents a stale base as a confident current edit.

## No hidden authority expansion

Each surface carries an `authority` pair — the `granted` authority the
user/policy gave and the `effective` authority the edit actually exercises, drawn
from an ordered ladder (`read_only` < `single_field_write` < `single_file_write`
< `multi_file_write` < `repo_wide`). If the effective authority exceeds the
granted authority the surface is `authority_expansion_detected`, and if it
exceeds the ceiling its scope permits (a doc-comment / single-file edit may write
at most a single file; a config-value / review-reply edit may write at most a
single field) the surface is `scope_authority_mismatch`. Both block promotion —
a light remote edit can never silently widen its authority.

## Capability and live-vs-captured

`apply_posture` (`local_direct_apply`, `remote_apply_available`,
`apply_blocked_by_policy`, `apply_unavailable_disclosed`) records whether the edit
can be applied, and `captured_vs_live` (`live`, `captured_snapshot`,
`narrowed_scope_rerun`) records what the reader is actually looking at. An apply
that is `apply_blocked_by_policy` but presented `live` is
`blocked_apply_presented_available` and blocks promotion.

## The export

`export` is the cited projection support, AI evidence, and diagnostics surfaces
ingest. It preserves the surface scope, edit intent, return path, trust class,
source class, confidence, authority, stale state, and escapes (the `preserves_*`
flags), and carries one `export_row` per surface. An export that drops a
preservation flag, references an unknown surface, drops a surface's row, or
disagrees with a surface's scope / trust class / source class / confidence /
authority / base-state disclosure / return-path presence blocks promotion.

## Degradations and promotion state

`edit_degradations` carry packet-level downgrades (`mirror_offline_snapshot`,
`live_provider_unreachable_captured_snapshot`, `apply_blocked_by_policy`,
`return_path_degraded`, `scope_narrowed_rerun`, `stale_base_state_narrowed`,
`authority_narrowed`, `broken_anchor`, `quarantined_source`) with a `severity`.
The computed `promotion_state` is:

- `stable` — no findings and no narrowing/blocking degradation;
- `narrowed_below_stable` — an otherwise-clean set carries a narrowing
  degradation, so the claim narrows rather than hiding the surfaces;
- `blocks_stable` — any blocking validation finding or blocking degradation.

## Boundary

Raw page bodies, raw URLs, raw edit diffs, raw source files, raw provider
payloads, and credentials never cross this boundary. The packet carries only
metadata, scope truth, edit intents, return paths, trust disclosures, stale-state
disclosures, authority truth, chip truth, cited refs, provenance, finding
summaries, and contract refs; `raw_boundary_material_present` blocks promotion if
forbidden material is found.
