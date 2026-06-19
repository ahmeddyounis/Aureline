# README / Changelog Maintenance

README, changelog, onboarding, and release-notes work is release-facing prose,
so Aureline gives it the same branch/release/channel truth it already expects for
code, release packets, and review surfaces. Each release-docs artifact has a
dedicated maintenance surface that makes the artifact kind, the
branch/release/channel scope, the pending suggestions, the compare history, and
the publish/export boundary visible **before** you edit or export text — and
keeps them inspectable **after** you leave the surface.

Each surface is described by one export-safe truth packet,
`release_docs_maintenance_surface_record`, so the same scope, evidence, compare,
and boundary truth is visible in the surface itself and to support, CLI/headless,
release-center, and Help/About surfaces.

- Record kind: `release_docs_maintenance_surface_record`
- Schema: [`schemas/docs/release-docs-maintenance.schema.json`](../../schemas/docs/release-docs-maintenance.schema.json)
- Canonical review packet: [`artifacts/docs/m5/release-docs-proof/review_packet.json`](../../artifacts/docs/m5/release-docs-proof/review_packet.json)
- Summary artifact: [`artifacts/docs/m5/release-docs-proof.md`](../../artifacts/docs/m5/release-docs-proof.md)
- Fixtures: [`fixtures/docs/m5/readme-changelog-scenarios/`](../../fixtures/docs/m5/readme-changelog-scenarios/)
- Producer: `aureline_docs::seeded_release_docs_maintenance_contract`
- Headless inspector: `cargo run -p aureline-docs --bin aureline_docs_release_docs_surface -- contract`

## Scope is visible before you edit or export

Every surface sets `scope_visible_before_edit` and carries a non-empty
`active_scope_summary`, so the branch/release/channel scope is shown before the
first edit. The `publish_scope` names the `branch_scope`, `release_scope`, and
`channel_scope` the work targets. A surface that crosses a review or publish
boundary must carry that scope; a release-facing workflow can never rely on an
unstated branch or channel assumption.

## Local-versus-shared evidence scope

`evidence_scope` keeps local-only and shared truth distinct so a note drafted for
the next beta or for a private branch is never presented as the currently
installed stable truth:

| Evidence scope | Meaning |
| --- | --- |
| `local_draft` | A local working draft that has never left the workspace. |
| `private_branch` | A draft on a private or feature branch; not shared. |
| `shared_review` | Shared for review inside a scoped review handoff. |
| `shared_prerelease` | Published to a prerelease/beta/next channel; not installed stable. |
| `installed_stable` | Matches the currently installed stable docs. |

Two guards keep this honest:

- **Masquerade guard** — any non-`installed_stable` artifact must name the
  branch/release/channel it targets, unless its boundary is `blocked_unscoped`
  (which is the explicit "no scope, so publish is blocked" state). This stops a
  beta or private-branch note from floating free and being mistaken for installed
  stable.
- **Stable-claim integrity** — a surface may only be labeled `installed_stable`
  when its source badge reports `exact_build_match` and `authoritative_live`
  freshness, so drifted prose can never be relabeled as installed stable.

## Pending suggestions, compare history, and the publish boundary

- **Pending suggestions** are diff-first, evidence-backed suggestion cards. The
  `pending_suggestion_count` must match `pending_suggestion_refs`, and every ref
  resolves to a card that blocks silent rewrites and carries evidence.
- **Compare history** records reopenable comparisons between two revisions. Each
  `compare_entry` is `reopenable`, names a distinct base and target, and uses one
  of `working_vs_installed`, `branch_vs_release`, `revision_vs_revision`, or
  `channel_vs_channel`. The base and target are stable opaque refs, never raw
  bodies.
- **Publish/export boundary** is one of `local_only`, `review_handoff_scoped`,
  `publish_handoff_scoped`, or `blocked_unscoped`. Any non-local boundary carries
  `publish_boundary_notes` shown before apply or export, and a `blocked_unscoped`
  surface exposes no apply/export action at all.

## Diff review, reopen, and in-product integration

`open_source_action` and `open_diff_review_action` are always available and
keyboard reachable, so you can open the canonical source or review the diff from
any surface, and any comparison can be reopened from history. Surfaces stay on an
in-product maintenance path — `in_product_maintenance_path` is always true — and
`integration_anchors` wire them into the release center, the help browser, the
About panel, and support export. There is no browser-only or vendor-console-only
maintenance path.

## Inspect after you leave

The contract projects a metadata-only, screenshot-free review packet
(`ReleaseDocsReviewPacket`) that preserves the surfaces, their pending
suggestions, compare history, and publish boundaries, and discloses the material
classes it omits. It never carries raw document bodies, rendered HTML, raw source
files, raw diffs, raw URLs, or credentials, so release-docs work can be reviewed
and exported without screenshots or copy/paste.

```sh
cargo run -p aureline-docs --bin aureline_docs_release_docs_surface -- review-packet
cargo run -p aureline-docs --bin aureline_docs_release_docs_surface -- validate
```
