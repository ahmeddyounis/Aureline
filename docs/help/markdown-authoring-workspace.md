# Markdown Authoring Workspace

The Markdown authoring workspace is the governed surface for editing README,
changelog, help, tutorial, and module documentation. It keeps the source
canonical at all times: you can move between **Source**, **Split**, and
**Rendered** modes without losing source identity, keyboard continuity, or
preview-safety disclosure, and a rendered view never masquerades as raw source.

Each open workspace is described by one export-safe truth packet,
`markdown_authoring_workspace_record`, so the same mode, preview-safety,
version/freshness, mirror/offline, and handoff truth is visible in the workspace
itself and to support, CLI/headless, release, and Help/About surfaces.

- Record kind: `markdown_authoring_workspace_record`
- Schema: [`schemas/docs/markdown-authoring-workspace.schema.json`](../../schemas/docs/markdown-authoring-workspace.schema.json)
- Canonical support export: [`artifacts/docs/m5/markdown-workspace-proof/support_export.json`](../../artifacts/docs/m5/markdown-workspace-proof/support_export.json)
- Summary artifact: [`artifacts/docs/m5/markdown-workspace-proof.md`](../../artifacts/docs/m5/markdown-workspace-proof.md)
- Fixtures: [`fixtures/docs/m5/markdown-workspace-modes/`](../../fixtures/docs/m5/markdown-workspace-modes/)
- Producer: `aureline_docs::current_stable_markdown_authoring_workspace_export`
- Headless inspector: `cargo run -p aureline-docs --bin aureline_docs_markdown_workspace -- packet`

## Modes, commands, and recovery

The workspace exposes three modes through stable command ids with full keyboard
parity, and remembers the last mode so it is restored on reopen:

| Mode | Command id | Default key | Renders |
| --- | --- | --- | --- |
| Source | `docs.authoring.mode.source` | `Mod+Alt+1` | nothing — raw Markdown only |
| Split | `docs.authoring.mode.split` | `Mod+Alt+2` | source beside a sanitized preview |
| Rendered | `docs.authoring.mode.rendered` | `Mod+Alt+3` | sanitized preview |

A dedicated recovery command, `docs.authoring.recover_source` (`Escape`), always
returns the workspace to raw source from any mode. The `open_source_action` and
the recovery command are always keyboard reachable; the mode toggle is too.
`remembered_mode_preference` and `active_mode` must both have a mode command, so
the remembered preference can always be honored.

## CommonMark baseline and extensions

`commonmark_baseline` is always `true` and `commonmark_baseline_note` discloses
the parsing baseline and the extensions enabled beyond it. `enabled_extensions`
lists the declared extensions; `active_extensions` lists those the renderer
actually activated and must be a subset — an undeclared active extension is a
hidden extension and fails validation.

## Rendered-preview safety

Rendered previews are sanitized and safe by default. `sanitization_state` is one
of:

- `sanitized_safe` — scripts, iframes, and event handlers were stripped;
- `raw_html_blocked` — raw embedded HTML was present and blocked;
- `raw_html_allowed_disclosed` — raw embedded HTML rendered under an explicit
  disclosure (a `sanitization_note` is then required);
- `not_applicable` — nothing is rendered (source mode).

Diagram, math, and custom-component capabilities are tracked separately in
`render_capabilities`. Each is `disabled`, `sandboxed_opt_in` (the strongest
active posture — rendering is always sandboxed and never privileged), `blocked`,
or `not_applicable`. There is deliberately no privileged or executing posture:
rendered preview and its diagram/math/custom-component engines are never a
privileged execution path.

When the active mode renders a preview, the packet must carry a concrete
sanitization posture (not `not_applicable`), concrete render capabilities, and a
`rendered_is_not_canonical_note` disclosing that the rendered view is not
canonical source or proof. In source mode nothing renders, so sanitization and
all render capabilities are `not_applicable`.

## Source, version, freshness, mirror, and handoff

The `source_version_badge` carries the source class, pack/revision refs, the
version or revision shown, the build stamp it was checked against, the
`freshness_class`, and the `version_match_state`, so rendered docs never appear
unqualified when source, provider, version, or freshness could affect
correctness.

`mirror_offline_state` records whether the source is a local project pack, a
verified mirror, a pinned offline pack, a warm cache, or live-online, and
`publish_boundary_state` records whether work stays local or crosses a scoped
review/publish boundary. `browser_handoff_availability` records whether a scoped
handoff to the browser companion is available; an `open_browser_action` may be
present only when handoff is `available`, and must be absent otherwise — the
workspace never offers a handoff it cannot honor or silently widens authority.

`anchor_context`, when present, preserves the initiating code or doc anchor
(symbol, file, section, review thread, release note, or search result) across
every mode switch.

## Boundary

Raw Markdown bodies, raw source files, rendered HTML, raw provider payloads, and
credentials never cross this boundary. The packet carries only metadata, mode and
command tokens, capability postures, source/version/freshness truth, and
disclosure notes.
