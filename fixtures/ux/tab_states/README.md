# Tab state fixtures

Seed corpus for the tab-state contract frozen in:

- [`/docs/ux/tab_state_contract.md`](../../../docs/ux/tab_state_contract.md)
- [`/schemas/ux/tab_item.schema.json`](../../../schemas/ux/tab_item.schema.json)

Each fixture is a single `tab_state_record` JSON document. The fixtures are
intentionally small so reviewers can validate tab meaning (cues and accessible
names) without needing to scan a full editor-group snapshot.

## Index

| Fixture | Focus |
| --- | --- |
| `preview_disposable_clean.json` | Disposable preview cues and labeling |
| `preview_promoted_modified.json` | Preview promotion preserves identity and becomes dirty without “replacement” ambiguity |
| `pinned_readonly_policy.json` | Pinned read-only tab + close/unpin swap clarity |
| `shared_followed_readonly.json` | Followed read-only tab keeps role visible and attributable |
| `compare_source_tabbed.json` | Compare tab role cues for the source side |
| `compare_target_tabbed.json` | Compare tab role cues for the target side |
| `blocked_missing_dependency.json` | Blocked placeholder exposes reason and safe actions |

