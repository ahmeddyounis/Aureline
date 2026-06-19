# Rendered Preview Boundaries

A richer docs rendered preview — diagrams, front matter, math, callouts, remote
assets, custom components — must stay safe and honest. Aureline records the
capability boundaries a preview enforces in one export-safe truth packet,
`rendered_preview_boundary_record`, so the same request state, render posture,
escape route, owner, and freshness truth is visible in the preview itself and to
support, CLI/headless, release, and Help/About surfaces.

Where the [Markdown authoring workspace](markdown-authoring-workspace.md) packet
records *which* mode a workspace is in, this packet records *what a rendered
preview is allowed to do* in that mode. The two share the same recovery-to-source
command shape, source/version badge, and sanitization vocabulary.

- Record kind: `rendered_preview_boundary_record`
- Schema: [`schemas/docs/docs-rendered-preview-capabilities.schema.json`](../../schemas/docs/docs-rendered-preview-capabilities.schema.json)
- Canonical support export: [`artifacts/docs/m5/rendered-preview-boundary-proof/support_export.json`](../../artifacts/docs/m5/rendered-preview-boundary-proof/support_export.json)
- Summary artifact: [`artifacts/docs/m5/rendered-preview-boundary-proof.md`](../../artifacts/docs/m5/rendered-preview-boundary-proof.md)
- Fixtures: [`fixtures/docs/m5/rendered-preview-boundaries/`](../../fixtures/docs/m5/rendered-preview-boundaries/)
- Producer: `aureline_docs::current_stable_rendered_preview_boundary_export`
- Headless inspector: `cargo run -p aureline-docs --bin aureline_docs_rendered_preview_boundaries -- packet`

## Owner and origin

A rendered preview is never an unlabeled active surface. `surface_owner` names
who renders it and `origin_disclosure` discloses that origin. The only legitimate
owners are `docs_preview_sandbox` (Aureline's own sandboxed docs renderer) and
`disclosed_browser_companion` (a scoped, disclosed handoff). The
`impersonated_native_shell` and `impersonated_browser_control_plane` owners are
modeled only so validation can reject any preview that masquerades as a native
approval surface or a browser-owned control plane.

## Capability requests and render postures

Each of the six capabilities carries its own boundary in `capability_boundaries`:

| Capability | What it governs |
| --- | --- |
| `diagrams` | fenced diagram blocks rendered by a diagram engine |
| `front_matter` | YAML/TOML front matter processed into a metadata view |
| `math` | inline or block math rendering |
| `callouts` | admonition / callout blocks beyond the baseline |
| `remote_assets` | remote images, embeds, or other network-fetched assets |
| `custom_components` | extension or custom-component rendering |

Every capability is an explicit request. `request_state` is one of
`not_requested`, `requested_awaiting_consent`, `granted_sandboxed`,
`denied_by_policy`, or `not_applicable`. A capability never renders without an
explicit grant: a `sandboxed_active` render requires `granted_sandboxed` and a
recorded `consent_ref`, and the two stay in lock-step.

`render_posture` is what the preview is actually doing:

- `disabled` — the capability is off; content renders inert as source text;
- `sandboxed_active` — content renders under an explicit, sandboxed grant (the
  strongest active posture — never privileged, never executing);
- `static_only` — content renders as static, non-interactive output;
- `raw_fallback` — content degrades honestly to raw source text;
- `blocked` — content was present and was blocked, with a visible cue;
- `not_applicable` — the capability does not apply in this mode.

There is deliberately no privileged or executing posture. A `blocked` or
`denied_by_policy` capability must disclose why it degraded in its `note`, and
every capability carries a visible `boundary_cue`.

## Raw/source escapes and external opens

Every capability sets `escape_to_source_available`, and the packet carries an
always-available `recover_to_source_command` (`docs.preview.recover_source`,
`Escape`) plus an `open_source_action` — both keyboard reachable. Whenever a
rendered mode is partial, blocked, stale, or unsupported, the raw source is one
keystroke away.

Capabilities that reference network content (typically `remote_assets`) declare
an `external_open_state`. When it is `available`, an `open_externally_action` is
present; otherwise the action is absent — the preview never offers an open it
cannot honor. Remote content is opened externally rather than fetched inside the
preview, keeping the preview offline-safe.

## No authority expansion

Each capability's `authority_posture` must be `no_authority_expansion`, and the
packet carries a `no_authority_expansion_note`. The rendered preview never
approves actions, never grants permissions, and never impersonates the native
shell or a browser-owned control plane. The `impersonates_native_approval` and
`claims_browser_control_plane` postures exist only so validation can reject them.

## Accessibility parity

`accessibility_parity` records theme, zoom, density, reduced-motion, and keyboard
parity. Parity is preserved where feasible; when a dimension cannot be preserved
safely it degrades honestly and a `parity_note` discloses the gap. Keyboard
parity is mandatory — the raw/source escape is always keyboard reachable.

## Source, version, freshness, and mirror

The `source_version_badge` carries the source class, pack/revision refs, the
version or revision shown, the build stamp it was checked against, the
`freshness_class`, and the `version_match_state`, so a rendered preview never
appears unqualified when source, provider, version, or freshness could affect
correctness. `mirror_offline_state` records whether the source is a local project
pack, a verified mirror, a pinned offline pack, a warm cache, or live-online, and
`publish_boundary_state` records whether work stays local or crosses a scoped
review/publish boundary. These survive support export and release evidence so
capability, owner, and freshness truth is never lost.

## Boundary

Raw Markdown bodies, raw source files, rendered HTML, raw provider payloads, and
credentials never cross this boundary. The packet carries only metadata,
capability postures, escape and disclosure notes, and source/version/freshness
truth.
