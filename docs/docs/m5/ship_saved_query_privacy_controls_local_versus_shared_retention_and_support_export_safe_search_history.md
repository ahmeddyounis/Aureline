# Saved-Query Privacy Controls (local-versus-shared retention, support-export-safe history)

This document is the contract for the M5 saved-query privacy boundary — the
records that let a saved search query, a recent history entry, or a pinned query
carry an explicit privacy control without ever letting a private query leak into
a shared store, widen its audience beyond what the user granted, or carry a raw
query body into a support export. Each entry is explained through one shared
vocabulary:

- a **saved query** — a query the user explicitly saved;
- a **recent history** entry — a recent search-history record;
- a **pinned query** — a pinned query;
- a **suggested query** — a suggested query.

Every entry carries the same source/version/freshness/locality/confidence chip
set the other docs lanes use, a declared **privacy class** (*who may see it*), a
granted-vs-effective **visibility** pair (*the no-hidden-visibility-expansion
guarantee*), a **retention disclosure** (*what storage tier the query lives in,
how widely that tier exposes it, and whether a shared/synced tier is disclosed*
— the local-versus-shared retention guarantee), a **support-export safety**
block (*whether the entry may travel in a support export and how it is redacted*
— the support-export-safe history guarantee), one **trust-class disclosure**
(*how trustworthy the origin is*), a share posture, the live-vs-captured state,
citation state, and the open-raw / open-source escapes. An export preserves the
privacy / retention / visibility / trust-class / source / confidence / redaction
/ export-safe / escape truth that support, AI evidence, and diagnostics surfaces
ingest rather than cloning status text. The search surface, docs browser shell,
saved-query library, recent-history panel, AI-context inspector, CLI/headless
output, support exports, diagnostics, and Help/About all consume the checked-in
packet.

- Record kind: `saved_query_privacy_controls`
- Schema: [`schemas/docs/ship-saved-query-privacy-controls-local-versus-shared-retention-and-support-export-safe-search-history.schema.json`](../../../schemas/docs/ship-saved-query-privacy-controls-local-versus-shared-retention-and-support-export-safe-search-history.schema.json)
- Canonical support export: [`artifacts/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/support_export.json`](../../../artifacts/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/support_export.json)
- Summary artifact: [`artifacts/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history.md`](../../../artifacts/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history.md)
- Fixtures: [`fixtures/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/`](../../../fixtures/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/)
- Producer: `aureline_docs::current_stable_saved_query_privacy_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_saved_query_privacy`

## The entries and their chips

`entries` is the set of saved-query / history entries for one session. Every
entry points at a `subject_ref`, carries an `entry_kind` (`saved_query`,
`recent_history`, `pinned_query`, `suggested_query`), a `privacy_class`, a
`title`, a `query_label` (the human-readable label — never a raw query body), and
a `chips` block — the five chips a consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `user_saved_query`, `team_shared_library`, `synced_history`, `imported_query_set`, `suggested_query`, `derived_suggestion` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `locality` | `local`, `synced_private`, `shared_store`, `managed` |
| `confidence` | `high`, `medium`, `low`, `heuristic` |

Every packet must include at least one entry on each of the `private_local` and
`shared_team` privacy classes — a partial set is `required_privacy_class_missing`
and blocks promotion, so the set stays the qualified saved-query boundary rather
than a slice that overstates coverage.

## Privacy class stays in bounds

Each entry carries one `privacy_class`. Only `private_local`, `private_synced`,
`shared_team`, and `shared_org` are inside the qualified M5 scope. An entry that
declares a `public_listing` or `unscoped_export` privacy class is
`privacy_class_out_of_bounds` and blocks promotion — the boundary never broadens
into a broad public listing or an unscoped export.

Each class maps to a maximum audience ceiling, drawn from an ordered ladder
(`owner_only` < `owner_devices` < `team` < `organization` < `everyone`):

| Privacy class | Maximum visibility |
| --- | --- |
| `private_local` | `owner_only` |
| `private_synced` | `owner_devices` |
| `shared_team` | `team` |
| `shared_org` | `organization` |

## No hidden visibility expansion

Each entry carries a `visibility` pair — the `granted` visibility the user/policy
gave and the `effective` visibility the query actually exposes. If the effective
visibility exceeds the granted visibility the entry is
`visibility_expansion_detected`, and if it exceeds the ceiling its privacy class
permits the entry is `privacy_visibility_mismatch`. Both block promotion — a
saved query can never silently widen its audience beyond the privacy control.

## Local-versus-shared retention truth

Each entry carries a `retention` disclosure — the `posture` the query is stored
under (`ephemeral_session`, `local_only`, `synced_private`, `shared_store`,
`managed_retention`), a `disclosed` flag, and a `note`. Each storage tier has a
shared boundary — the widest audience it exposes a retained query to
(`local_only`/`ephemeral_session` → `owner_only`, `synced_private` →
`owner_devices`, `shared_store` → `team`, `managed_retention` → `organization`).
If the storage tier exposes the query more widely than its privacy class allows
(for example a `private_local` query held in a `shared_store`) the entry is
`retention_privacy_mismatch` and blocks promotion — a private query never leaks
into a shared store. A `synced_private`, `shared_store`, or `managed_retention`
tier that is not disclosed (or carries an empty note) is
`retention_disclosure_missing` — a query that leaves the local device always
discloses where it went.

## Support-export-safe search history

Each entry carries an `export_safety` block — the `redaction_class` applied for
support export (`redacted_label_only`, `digest_only`, `raw_withheld`,
`needs_redaction`, `not_exportable`), an `export_safe` flag, and a `note`. An
entry marked `export_safe` whose redaction class is not actually export-safe
(`needs_redaction`, `not_exportable`) is `support_export_unsafe` and blocks
promotion, and an empty safety note is `support_export_unsafe` as well — a
search-history entry never travels in a support export unless it is genuinely
redaction-safe.

## The trust-class disclosure

Each entry carries one `trust_class`:

| Trust class | Meaning |
| --- | --- |
| `first_party_user_saved` | A first-party query the user saved themselves. |
| `signed_shared_library` | A pinned, signed shared-library query. Must stay cited. |
| `extension_imported_set` | A query imported from a signed extension set. Must stay cited. |
| `live_synced_suggestion` | A live-synced suggestion — not verified at materialization. Must stay cited. |
| `derived_suggestion_only` | A derived / inferred suggestion only. Must stay cited. |

An entry whose trust class cannot back an authoritative claim
(`live_synced_suggestion`, `derived_suggestion_only`) presented at `high`
confidence is `trust_class_disclosure_collapsed` and blocks promotion. An
untrusted origin that is not cited is `entry_not_cited`. An entry with an empty
trust-disclosure note is `trust_class_disclosure_missing`.

## Share posture and live-vs-captured

`share_posture` (`local_private_only`, `share_available`,
`share_blocked_by_policy`, `share_unavailable_disclosed`) records whether the
query can be shared, and `captured_vs_live` (`live`, `captured_snapshot`,
`narrowed_scope_rerun`) records what the reader is actually looking at. An entry
that is `share_blocked_by_policy` but presented `live` as a shared (beyond
`owner_devices`) entry is `blocked_share_presented_available` and blocks
promotion. A non-current `version_match` presented at `high` confidence and
`authoritative_live` freshness is `version_truth_collapsed`.

## The export

`export` is the cited projection support, AI evidence, and diagnostics surfaces
ingest. It preserves the entry privacy class, retention, visibility, trust class,
source class, confidence, redaction, export-safe flag, and escapes (the
`preserves_*` flags), and carries one `export_row` per entry. An export that
drops a preservation flag, references an unknown entry, drops an entry's row, or
disagrees with an entry's privacy class / retention / visibility / trust class /
source class / confidence / redaction class / export-safe flag blocks promotion.

## Degradations and promotion state

`query_degradations` carry packet-level downgrades (`sync_offline_snapshot`,
`shared_store_unreachable_captured_snapshot`, `share_blocked_by_policy`,
`retention_tier_degraded`, `scope_narrowed_rerun`, `privacy_narrowed`,
`redaction_upgraded`, `broken_anchor`, `quarantined_source`) with a `severity`.
The computed `promotion_state` is:

- `stable` — no findings and no narrowing/blocking degradation;
- `narrowed_below_stable` — an otherwise-clean set carries a narrowing
  degradation, so the claim narrows rather than hiding the entries;
- `blocks_stable` — any blocking validation finding or blocking degradation.

## Boundary

Raw query bodies, raw URLs, raw history payloads, raw source files, raw provider
payloads, and credentials never cross this boundary. The packet carries only
metadata, privacy truth, retention truth, redaction truth, visibility truth, chip
truth, cited refs, provenance, finding summaries, and contract refs;
`raw_boundary_material_present` blocks promotion if forbidden material is found.
