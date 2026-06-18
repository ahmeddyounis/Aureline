# M5 Markdown-Authoring, Safe-Preview, Docs-Maintenance, and Docs-Evidence-Handoff Matrix

This document is the contract for the frozen M5 matrix that qualifies the five
docs-authoring surfaces. The matrix is the canonical M5 control source for this
lane: authoring workspaces, preview panes, docs-maintenance panels, Help/About
surfaces, release center, and support exports ingest the checked-in packet rather
than cloning status text or re-litigating docs-authoring truth.

- Record kind: `freeze_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix`
- Schema: [`schemas/docs/freeze-the-m5-markdown-authoring-safe-preview-docs-maintenance-and-docs-evidence-handoff-matrix.schema.json`](../../../schemas/docs/freeze-the-m5-markdown-authoring-safe-preview-docs-maintenance-and-docs-evidence-handoff-matrix.schema.json)
- Canonical support export: [`artifacts/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/support_export.json`](../../../artifacts/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/support_export.json)
- Summary artifact: [`artifacts/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md`](../../../artifacts/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md)
- Fixtures: [`fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/`](../../../fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/)
- Producer: `aureline_docs::current_stable_m5_markdown_authoring_matrix_export`

## Surfaces

| Surface | Qualification | Workspace modes | Preview safety | Source contract |
| --- | --- | --- | --- | --- |
| `markdown_authoring_workspace` | Stable | source / split / rendered | `sanitized_safe` | [`schemas/docs/docs_maintenance_row.schema.json`](../../../schemas/docs/docs_maintenance_row.schema.json) |
| `commonmark_preview` | Stable | split / rendered | `sanitized_safe` | [`schemas/docs/docs_maintenance_row.schema.json`](../../../schemas/docs/docs_maintenance_row.schema.json) |
| `docs_maintenance_suggestions` | Stable | source / split | `sanitized_safe` | [`schemas/docs/docs_suggestion_card.schema.json`](../../../schemas/docs/docs_suggestion_card.schema.json) |
| `docs_validation` | Stable | source | `not_applicable` | [`schemas/docs/docs_maintenance_row.schema.json`](../../../schemas/docs/docs_maintenance_row.schema.json) |
| `docs_evidence_handoff` | Beta | source | `not_applicable` | [`schemas/docs/docs_browser_truth_packet.schema.json`](../../../schemas/docs/docs_browser_truth_packet.schema.json) |

Each surface row binds a qualification class to its supported workspace modes,
rendered-preview safety class, validation states, docs-suggestion triggers,
evidence-handoff scope, evidence requirement, required evidence packet refs,
downgrade triggers, rollback posture, source contracts, and the consumer surfaces
that must project the surface's qualification truth.

## Controlled vocabulary

The matrix freezes one controlled vocabulary, mapped onto the canonical tokens
already owned by the docs-maintenance runtime so README, changelog, help, and
tutorial authoring never drift into feature-local conventions:

- **Workspace modes** — `source`, `split`, `rendered`.
- **Rendered-preview safety classes** — `sanitized_safe`, `raw_html_blocked`,
  `raw_html_allowed_disclosed`, `not_applicable`. Rendered previews are sanitized
  by default; raw embedded HTML is blocked unless it renders under an explicit
  disclosure.
- **Validation states** — `validated`, `suspected_stale`, `unchanged_unverified`,
  `unsupported`, `skipped`, `stale_rerun_required`, `not_validated`. Validation
  truth never silently upgrades to verified.
- **Docs-suggestion triggers** — `code_diff`, `stale_example`,
  `release_note_drift`, `failing_snippet`, `contract_change`, `human_note`.
- **Evidence-handoff scopes** — `local_only`, `review_handoff_scoped`,
  `publish_handoff_scoped`, `blocked_unscoped`.

## Track invariant

Docs stay source-canonical: rendered views remain safe and labeled, suggestions
stay diff-first, source/version/freshness truth stays visible, and browser
handoff never hides owner/origin/boundary changes or silently widens authority.
The `trust_review` block encodes these as hard invariants — all must hold for the
matrix to validate:

- `source_canonical_rendered_safe_and_labeled` and
  `rendered_preview_safe_by_default` — rendered views never replace the canonical
  source and are sanitized by default.
- `preview_not_privileged_execution_path` — rendered preview, diagram engines,
  and docs suggestions are never privileged execution paths.
- `suggestions_diff_first_never_auto_applied` — every suggestion is a reviewable
  draft or review diff.
- `source_version_freshness_truth_visible` and
  `validation_state_never_silently_upgraded` — provenance and validation truth
  stay visible.
- `evidence_handoff_source_linked`,
  `handoff_never_hides_owner_origin_or_boundary`, and
  `handoff_never_silently_widens_authority` — evidence handoff ties prose to
  code/schema/release truth without hiding boundary changes.
- `no_full_browser_collab_editor_or_remote_cms` — desktop/local-first scope only.
- `downgrade_narrows_instead_of_hides` and
  `stale_or_underqualified_blocks_promotion`.

## Release, mirror/offline, and support-export parity

`release_posture` binds the supporting release packet
(`evidence:docs-authoring-release-packet:m5`) and the mirror/offline packet
(`evidence:docs-authoring-mirror-offline-packet:m5`), and requires support/export
and mirror/offline parity across every authoring surface before authoring depth
widens. Each surface declares the consumer surfaces — including `cli_headless`,
`support_export`, `release_center`, and `help_about` — that must project its
qualification truth.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected surface. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`mirror_offline`, `source_version_mismatch`, `freshness_expired`,
`trust_narrowing`, `scope_expansion_unqualified`, `unsafe_preview_blocked`, and
`upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/)
show an unsafe-preview narrowing and a held evidence-handoff surface; both remain
valid packets because narrowing is explicit, not hidden.

## Boundary

Raw document bodies, raw source files, rendered HTML, raw provider payloads,
credentials, and live vendor-doc snapshots never cross this boundary. The packet
carries only metadata, qualification truth, and contract references.
