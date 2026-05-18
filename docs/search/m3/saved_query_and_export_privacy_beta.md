# Saved Query and Export Privacy Beta

This contract promotes saved searches, query history, search deep links, and
exported result sets from incidental UI state into governed search artifacts.
The implementation lives in
[`crates/aureline-search/src/query_artifacts/mod.rs`](../../../crates/aureline-search/src/query_artifacts/mod.rs)
and wraps the existing planner-backed query session, saved-query privacy, and
export-packet models.

Boundary schemas:

- [`schemas/search/saved_query.schema.json`](../../../schemas/search/saved_query.schema.json)
- [`schemas/search/query_history.schema.json`](../../../schemas/search/query_history.schema.json)
- [`schemas/search/search_export_snapshot.schema.json`](../../../schemas/search/search_export_snapshot.schema.json)

Protected fixtures:
[`fixtures/search/m3/saved_query_privacy/`](../../../fixtures/search/m3/saved_query_privacy/).

## Artifact Model

The beta artifact set materializes five records for a saved search workflow:

| Record | Purpose |
| --- | --- |
| `scope_pack_binding` | Pins the captured scope class, stable scope id, workset id, scope label, and missing-scope reasons used by reopen and export flows. |
| `saved_query` | Stores saved-query identity, source class, privacy class, retention mode, sync class, redaction profile, migration state, query hash/text mode, and replay scope. |
| `query_history_entry` | Records a previous query session with stored-text mode and retention posture, defaulting to local-only history. |
| `search_deep_link` | Reopens search intent under the recipient's current permissions; it does not carry ambient result access. |
| `search_collection_snapshot` | Captures result refs, selected/included counts, hidden/omitted counts, source labels, and partiality reasons for support/docs/local export review. |

Each record carries:

- `schema_version`
- `source_class`
- `privacy_class`
- `retention_mode`
- `sync_class`
- `redaction_profile`
- `retention_widening_basis`
- scope honesty state
- live-versus-captured result semantics

## Privacy Defaults

User-authored saved queries and query-history rows default to
`local_only_default`, `local_only`, and `literal_local_only`. Raw query text can
remain only under that local-only literal profile.

Support, docs, workspace-shared, and policy-owned artifacts default to hashes,
scope summaries, result refs, and count/partiality disclosures:

- `support_export_redacted` becomes `hashes_scope_and_result_refs`.
- `policy_withheld` removes both literal text and hash material.
- support snapshots preserve result refs and partiality reasons, not result
  bodies or literal query text.

Any user-authored artifact that widens retention or sync must carry
`explicit_user_opt_in` or `policy_owned` as its widening basis. Repo-provided,
team-shared, policy-provided, and support-captured artifacts carry distinct
source classes so audit and support tooling can explain who created the search
artifact and why it exists.

## Deep Links

Search deep links are intent links. They set:

- `rerun_required = true`
- `recipient_re_resolves_under_current_permissions = true`
- `access_widening_allowed = false`
- `result_semantics = live_rerun_required`
- `scope_honesty_state = recipient_must_re_resolve`

A link that claims captured rows are current, or that allows access widening,
is non-conforming and emits a validation finding.

## Export Snapshots

`search_collection_snapshot` is a captured review/export artifact. It preserves:

- selected and included result refs
- source labels for included rows
- visible, selected, omitted, hidden-by-scope, hidden-by-policy, and hidden-by
  remote-cache counts
- partiality reasons such as `indexing_in_progress`, `hidden_by_policy`, and
  `omitted_unselected_results`
- `captured_snapshot` semantics plus `current_truth_requires_rerun = true`

Support and docs snapshots prove raw-query avoidance through
`export_avoids_raw_query_by_default()`, which requires a redacted destination,
no literal query text, and a redaction profile that admits only hash/metadata
material.

## Validation

The artifact validator reports closed finding tokens for:

- raw query material outside local-only literal retention
- retention or sync widening without explicit opt-in or policy ownership
- deep links that widen access or skip recipient re-resolution
- missing stable scope identity
- partial export snapshots without partiality reasons
- captured artifacts that claim current live results

These findings are meant for UI review sheets, support manifests, and conformance
tests. They are intentionally metadata-only.
