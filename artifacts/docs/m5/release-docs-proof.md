# Release-Docs Maintenance Surfaces

- Contract: `release-docs:readme-changelog:beta:v1`
- Version: `release-docs-rev:readme-changelog:2026.06.01-01`
- Schema: `schemas/docs/release-docs-maintenance.schema.json`
- Help: `docs/help/readme-changelog-maintenance.md`
- Fixtures: `fixtures/docs/m5/readme-changelog-scenarios/`
- Surfaces: 5 · Compare entries: 5 · Integration anchors: 7

The canonical, export-safe records are checked in under
[`artifacts/docs/m5/release-docs-proof/`](release-docs-proof/):

- [`review_packet.json`](release-docs-proof/review_packet.json) — the
  metadata-only, screenshot-free review packet (no raw bodies, no raw URLs).
- [`surface_projection.json`](release-docs-proof/surface_projection.json) — the
  render-ready surfaces plus the coverage summary.

Regenerate and validate with the headless inspector:

```sh
cargo run -p aureline-docs --bin aureline_docs_release_docs_surface -- review-packet
cargo run -p aureline-docs --bin aureline_docs_release_docs_surface -- projection
cargo run -p aureline-docs --bin aureline_docs_release_docs_surface -- validate
```

## Surfaces

| Surface | Artifact | Evidence scope | Publish boundary | Compare history |
| --- | --- | --- | --- | --- |
| `readme:installed-stable` | readme | `installed_stable` | `local_only` | `working_vs_installed` |
| `changelog:beta` | changelog | `shared_prerelease` | `publish_handoff_scoped` | `channel_vs_channel` |
| `onboarding:shared-review` | onboarding_note | `shared_review` | `review_handoff_scoped` | `branch_vs_release` |
| `release-notes:blocked` | release_notes | `private_branch` | `blocked_unscoped` | `revision_vs_revision` |
| `readme:next-draft` | readme | `local_draft` | `local_only` | `working_vs_installed` |

## Invariants proven

- **Scope before edit** — every surface sets `scope_visible_before_edit` and
  carries a non-empty `active_scope_summary`, so branch/release/channel scope is
  visible before a user edits or exports text.
- **Local-versus-shared truth** — the `installed_stable` README must match the
  running build, while the `shared_prerelease` changelog, the `private_branch`
  release notes, and the `local_draft` next-channel README each name the scope
  they target so they cannot masquerade as the installed stable truth.
- **Reopenable compare history** — every compare entry stays reopenable and the
  four compare kinds are exercised.
- **In-product path** — every surface stays on an in-product maintenance path and
  the release-center, help-browser, About-panel, and support-export integrations
  are all covered, so there is no browser-only or vendor-console-only path.
- **Boundary honesty** — the `blocked_unscoped` release notes expose no
  apply/export action and disclose why publish is blocked; the review packet
  preserves pending suggestions, compare history, and publish boundaries so they
  stay inspectable after the user leaves the surface.
