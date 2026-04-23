# Safe-preview and suspicious-content fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/security/safe_preview_trust_classes.md`](../../../docs/security/safe_preview_trust_classes.md)
and validated by:

- [`/schemas/security/trust_class.schema.json`](../../../schemas/security/trust_class.schema.json)
- [`/schemas/security/text_representation_policy.schema.json`](../../../schemas/security/text_representation_policy.schema.json)

Each fixture is a JSON array containing:

- one `surface_trust_resolution_record`;
- zero or one `suspicious_content_case_record`; and
- one or more `representation_transfer_record` entries.

The split is deliberate. The trust-class schema freezes how a surface is
allowed to render and narrow. The representation-policy schema freezes
what leaves the product when the user copies or exports from that
surface.

Scope rules:

- Raw bodies, raw paths, raw URLs, and raw secrets never appear in a
  fixture.
- Suspicious-content findings use the closed class set from the trust-
  class schema and stay attached to a concrete location kind.
- Transfer records use explicit labels (`Copy raw ...`,
  `Copy escaped ...`, `Export sanitized snapshot`,
  `Export metadata only`) so no fixture relies on a generic `Copy` or
  `Export` label.
- Metadata-only cases must say why the body is withheld.

Index:

| Fixture | Surfaces and focus |
|---|---|
| [`editor_bidi_identifier.json`](./editor_bidi_identifier.json) | `editor_content`; raw-text identifier with bidi and invisible-formatting findings plus raw/escaped copy paths |
| [`docs_help_markdown_representation.json`](./docs_help_markdown_representation.json) | `docs_help_page`; sanitized Markdown preview with rendered-versus-source divergence and raw/rendered/sanitized transfer labels |
| [`rich_preview_notebook_downgrade.json`](./rich_preview_notebook_downgrade.json) | `rich_preview`; notebook-style rich output narrowed from active behavior to a sanitized snapshot |
| [`install_review_confusable_publisher.json`](./install_review_confusable_publisher.json) | `install_review`; strict trust-decision display mode for a confusable publisher identifier |
| [`remote_attach_approval_host.json`](./remote_attach_approval_host.json) | `approval_surface`; remote-attach approval row with invisible formatting in the host label and approval-scope visibility requirements |
| [`support_export_sanitized_snapshot.json`](./support_export_sanitized_snapshot.json) | `support_export_surface`; sanitized support-export snapshot that preserves representation labels and suspicious-content summaries |
| [`delete_review_metadata_only.json`](./delete_review_metadata_only.json) | `delete_review_surface`; last-visible-evidence review after origin loss, narrowed to metadata-only export |
| [`embedded_webview_isolated_remote.json`](./embedded_webview_isolated_remote.json) | `rich_preview`; embedded remote webview held at `IsolatedRemoteActive` under verified origin, connectivity, and permissions, with sanitized-snapshot and metadata-only fallbacks |
| [`browser_handoff_blocked_execution.json`](./browser_handoff_blocked_execution.json) | `docs_help_page`; confusable link where in-product execution is blocked and the path forward is a system-browser handoff after raw-target inspection |
