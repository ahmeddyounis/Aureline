# Badge aggregate — release evidence

Reviewer-facing evidence packet for the lane that finalizes **badge-aggregate
count-class semantics, cross-client dedupe, admin suppression, and persistent
attention summaries** on claimed-stable desktop shell surfaces: one canonical
record per whole-shell snapshot that binds typed count classes, one durable
object set behind every badge surface, cross-client / cross-window dedupe,
export-safe admin / quiet-hours suppression lineage, a `0`-means-none guarantee,
a persistent and inspectable attention summary, a public claim ceiling, an
automatic narrow-below-Stable verdict, recovery and route parity across the
activity center / command palette / status bar / menus, accessibility across
normal / high-contrast / zoomed layouts, and rows that stay available without an
account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression/`](../../../fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression/)
- Schema: [`/schemas/ux/finalize-badge-semantics-cross-client-dedupe-admin-suppression.schema.json`](../../../schemas/ux/finalize-badge-semantics-cross-client-dedupe-admin-suppression.schema.json)
- Companion doc: [`/docs/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression.md`](../../../docs/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression.md)
- Typed source: `aureline_shell::badge_aggregate_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_badge_aggregate_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/badge_aggregate_stable_fixtures.rs`

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `nominal.json` | nominal | **stable** | stable | — |
| `quiet_and_admin_suppression.json` | quiet hours + admin suppression | **stable** | stable | — |
| `companion_preview_surface.json` | companion badge surface in preview | preview (narrowed) | preview | `surface_not_yet_stable` |
| `cross_client_inflation_drill.json` | cross-client inflation drill | beta (narrowed) | stable | `one_durable_set_not_proven` |

Coverage verdict: **2 Stable, 2 narrowed**. Each narrowed row names a reason and
drops below the launch cutline rather than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **Every badge count is typed by count class, not by surface.** The
  `AggregateCountClass` taxonomy keys every count; `class_aggregates[]` and the
  per-surface `class_counts[]` carry the class token. The required classes
  (`pending_review_approval`, `failed_runs`, `queued_publish_later`,
  `provider_auth_attention`, `managed_advisories`, `muted_informational_backlog`)
  are exercised across the matrix — guarded by
  `required_count_classes_are_exercised_across_the_matrix`.
- **Dock/taskbar, title-bar, in-shell, and companion counts are computed from the
  same durable object set as the activity center.** `surface_projections[]` are
  reconciled against the authoritative active counts; the activity center is
  authoritative and never disables a class — guarded by
  `surfaces_derive_from_one_durable_set`. A surface that inflates a class drops
  the `one_durable_set_holds` pillar and narrows the row (see the inflation
  drill).
- **A badge count of zero means no current durable objects of that class.**
  Active counts are derived from Active durable objects only; the
  `nominal.json` muted-backlog class shows `active_count: 0` with
  `held_or_suppressed_count: 1` and a lineage entry — guarded by
  `zero_active_means_no_active_durable_objects`.
- **Cross-client and cross-window dedupe preserve count-class integrity.** Raw
  appearances collapse by canonical object identity; `cross_client_dedupe`
  records `raw_appearance_count`, `deduped_object_count`, and
  `cross_client_collapsed`. A repeat disagreeing on its class is rejected — a copy
  never multiplies a badge. Guarded by
  `cross_client_dedupe_preserves_class_integrity`.
- **Export-safe lineage for admin suppression, quiet-hours muting, and per-class
  badge disablement.** `quiet_and_admin_suppression.json` carries an
  `admin_policy_suppression` object entry, a `quiet_hours_muting` object entry,
  and a `per_class_badge_disabled` surface-class entry, each preserving the
  durable object and reopen target. Guarded by
  `suppression_lineage_is_export_safe_and_complete`.
- **Badge semantics, suppression reasons, and count classes are documented and
  inspectable in-product.** The `summary_digest` is persistent and inspectable;
  the count classes, suppression reasons, and surfaces live in the record's
  closed vocabularies and the schema, not only in release notes.
- **Surfaces still lacking stable qualification are automatically narrowed.** The
  companion-preview and inflation-drill rows drop below Stable with named reasons
  — guarded by `narrowed_rows_drop_below_cutline_and_name_a_reason` and
  `claim_ceiling_never_overclaims`.
- **Discover / operate / recover from keyboard and mouse without account
  requirements or toast-only truth.** Four recovery routes and four entry-route
  surfaces are keyboard-reachable across three layout modes; every row stays
  available without an account or managed services — guarded by
  `recovery_routes_are_complete_and_keyboard_reachable`,
  `routes_reach_every_surface_keyboard_first`,
  `accessibility_holds_in_every_layout`, and
  `rows_stay_available_without_account_or_managed_services`.

## Verification

```sh
# Unit + projection invariants
cargo test -p aureline-shell --lib badge_aggregate_stable

# Fixture replay + acceptance-criteria invariants
cargo test -p aureline-shell --test badge_aggregate_stable_fixtures

# Refresh fixtures from the in-code projection
cargo run -q -p aureline-shell \
  --bin aureline_shell_badge_aggregate_stable -- emit-fixtures \
  fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression

# Reviewer index / plaintext support export
cargo run -q -p aureline-shell --bin aureline_shell_badge_aggregate_stable -- index
cargo run -q -p aureline-shell --bin aureline_shell_badge_aggregate_stable -- plaintext
```

The fixtures are a literal projection of the in-code corpus, which is itself a
projection of the live attention router; the replay gate fails on any drift.
