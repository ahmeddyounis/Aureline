# Rendered Preview Boundary

- Boundary: `docs-preview-boundary:readme:split:0001`
- Label: `README rendered-preview boundary`
- Workspace: `docs-workspace:readme:split:0001`
- Artifact: `README.md`
- Active mode: `split`
- Surface owner: `docs_preview_sandbox`
- Sanitization: `sanitized_safe`
- Source: `project_docs` / freshness `warm_cached` / version `exact_build_match`
- Mirror/offline: `local_project_pack`

## Capability boundaries

| Capability | Request | Render | Authority | External |
| --- | --- | --- | --- | --- |
| `diagrams` | `granted_sandboxed` | `sandboxed_active` | `no_authority_expansion` | `not_required` |
| `front_matter` | `requested_awaiting_consent` | `static_only` | `no_authority_expansion` | `not_required` |
| `math` | `granted_sandboxed` | `sandboxed_active` | `no_authority_expansion` | `not_required` |
| `callouts` | `requested_awaiting_consent` | `static_only` | `no_authority_expansion` | `not_required` |
| `remote_assets` | `requested_awaiting_consent` | `blocked` | `no_authority_expansion` | `available` |
| `custom_components` | `not_requested` | `disabled` | `no_authority_expansion` | `not_required` |

## Escapes

- Recover to source: `docs.preview.recover_source` (Escape)
- Open source: `docs.preview.open_source`
