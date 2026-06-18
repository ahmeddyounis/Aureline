# M5 Docs Authoring, Preview, Maintenance, and Evidence-Handoff Matrix

- Packet: `m5-docs-authoring-matrix:stable:0001`
- Schema: `schemas/docs/freeze-the-m5-markdown-authoring-safe-preview-docs-maintenance-and-docs-evidence-handoff-matrix.schema.json`
- Support export: `artifacts/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/support_export.json`
- Contract doc: `docs/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md`
- Fixtures: `fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/`

## Coverage

- The Markdown authoring workspace is qualified Stable: README, changelog, help, tutorial, and module docs author in source/split/rendered modes, with rendered views safe, labeled, and never a privileged execution path.
- The CommonMark preview baseline is qualified Stable: Markdown renders to a sanitized, labeled view; embedded raw HTML, scripts, iframes, and event handlers are stripped or blocked, and diagram engines stay opt-in and non-privileged.
- Docs-maintenance suggestions are qualified Stable: every suggestion is diff-first, tied to a trigger (code diff, stale example, release-note drift, failing snippet, contract change, or human note), and never silently applied to source.
- Docs validation is qualified Stable: validated, suspected-stale, unverified, unsupported, skipped, stale-rerun-required, and not-validated states stay visible and never silently upgrade to verified.
- Docs evidence handoff is qualified Beta: a prose change is tied back to the code, schema, or release truth it depends on; handoff is scoped and source-linked and never hides owner, origin, or boundary changes or silently widens authority.
- Every surface carries required evidence packet refs, downgrade triggers, rollback posture, and consumer-surface parity.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.
- The release posture binds the supporting release packet and mirror/offline packet and requires support/export and mirror/offline parity across every authoring surface.

## Trust guardrails

The matrix proves that docs stay source-canonical: rendered preview, diagram engines, and docs suggestions never become privileged execution paths, suggestions stay diff-first, source/version/freshness truth stays visible, validation state never silently upgrades to verified, and evidence handoff stays source-linked and never hides owner, origin, or boundary changes or silently widens authority. No full browser product, collaborative rich-text editor, or remote CMS is in scope, and stale or underqualified rows automatically narrow before publication rather than hiding the surface.
