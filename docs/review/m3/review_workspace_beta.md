# Review workspace beta: durable anchors and browser handoff

The beta review-workspace packet builds on the local review seed packet
and gives review, CLI/headless, support/export, and browser-companion
surfaces one shared object model. It does not add a provider adapter or
merge engine; it freezes the baseline records those systems must read.

The companion schema lives at:

- [`/schemas/review/review_workspace.schema.json`](../../../schemas/review/review_workspace.schema.json)

The canonical fixtures live under:

- [`/fixtures/review/m3/review_workspace_beta/`](../../../fixtures/review/m3/review_workspace_beta/)

The checked projection artifact lives at:

- [`/artifacts/review/m3/review_workspace_beta_projection.json`](../../../artifacts/review/m3/review_workspace_beta_projection.json)

The Rust types are exported from `aureline_review::workspace`, defined in
[`crates/aureline-review/src/workspace/mod.rs`](../../../crates/aureline-review/src/workspace/mod.rs).
The integration test
[`crates/aureline-review/tests/review_workspace_beta.rs`](../../../crates/aureline-review/tests/review_workspace_beta.rs)
replays the fixtures from the existing alpha seed fixture and validates
the closed acceptance states, including parity with the checked
projection artifact.

## Record Shape

One `review_workspace_beta_packet` contains:

| Block | Purpose |
| --- | --- |
| `review_workspace` | The canonical VCS `review_workspace_record` so local, provider-overlay, imported, and browser-handoff sources share identity and freshness vocabulary. |
| `diff_entries` | The diff surfaces opened inside the workspace, including stable anchor refs created by the alpha seed. |
| `durable_comment_anchors` | Comment/thread anchors that preserve the source anchor ID, fallback context hash, stable identity fields, drift state, and local/provider freshness. |
| `object_lineage` | Edges connecting local worktree/provider/diff/anchor/handoff/export objects so support and export flows can reconstruct the review context. |
| `check_freshness` | Current check rows with status, authority, freshness, evidence refs, and an explicit browser-state-independent flag. |
| `browser_handoff` | Optional typed handoff row with destination class, opaque destination ref, reason code, replay posture, return anchor, and `raw_url_export_allowed = false`. |
| `support_export` | Metadata-only export packet with reopen context, reopen command, anchor refs, check refs, lineage refs, source schema refs, and raw-body/raw-URL export flags closed. |
| `inspection` | Deterministic counts and booleans used by tests, support, and inspector surfaces. |

## Acceptance Rules

The validator enforces these rules:

1. Durable comment anchors must cite an alpha source anchor and keep
   provider object IDs out of the stable anchor hash.
2. Anchor drift state and required user action must be paired. Remapped
   anchors must cite a remap chain; archived anchors must cite
   `archived_at`; non-remapped anchors cannot silently carry a remap.
3. Check freshness rows must be independent of browser state. Stale or
   unavailable check rows that affect operator truth must set
   `blocks_operator_truth_claim_when_stale = true`.
4. Browser handoff rows must use a closed destination class and opaque
   destination ref. Raw URLs, SSH URLs, and raw Git remotes are rejected.
5. Browser handoff is reversible only when it carries a return anchor
   kind and return anchor ref.
6. Support/export packets must include durable anchor refs, check refs,
   lineage refs, `support_export`, `cli_headless_entry`, and the beta
   schema ref. Raw comment bodies, source bodies, and URLs are never
   exported through this packet.

## Fixtures

| Fixture | Coverage |
| --- | --- |
| `local_workspace_with_reversible_browser_handoff.json` | Preserves a bound comment anchor, current check freshness, typed reversible browser handoff, object lineage, and reopenable support export. |
| `stale_check_blocks_operator_truth.json` | Preserves a remapped anchor while a stale CI overlay check blocks operator-truth claims until refreshed. |

## Consumer Path

`ReviewWorkspaceBetaPacket::from_seed_packet` consumes the existing
`ReviewWorkspaceSeedPacket` and a `ReviewWorkspaceBetaInput`, then
materializes the durable anchors, lineage, check freshness, handoff, and
support export records. `project_review_workspace_beta_packet` parses a
materialized JSON packet into `ReviewWorkspaceBetaProjection`, which is
the compact CLI/headless and inspector projection.

The first shell consumer is
[`crates/aureline-shell/src/review/workspace_inspector/mod.rs`](../../../crates/aureline-shell/src/review/workspace_inspector/mod.rs).
It renders deterministic plaintext (`render_beta_review_workspace_plaintext`)
for CLI/headless/docs/support-style inspection from the checked-in
fixtures.

This keeps the lane inspectable without relying on hidden browser state
or provider pages. Browser handoff remains a typed transition; support
and export reopen the same review context rather than a raw URL.
