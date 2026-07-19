# M5 badge-family matrix contract

Status: **Frozen (M5)** · Truth source: checked-in schema, seed builder, fixtures, and release proof.

This contract freezes the canonical **badge families** Aureline shows whenever a
support, freshness, lifecycle, deployment, or compatibility claim becomes
user-visible. It is the single source of truth for the badge vocabulary,
explanation drawers, axis-separation rules, and downgrade rules that marketplace,
Help/Docs, Settings, onboarding, diagnostics, runtime, and exported-evidence
surfaces all consume, so identical badge terms mean the same thing on every
surface.

- Rust module: `crates/aureline-release/src/freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`
- Boundary schema: [`schemas/ui/m5-badge-family-matrix.schema.json`](../../schemas/ui/m5-badge-family-matrix.schema.json)
- Support export: `artifacts/release/m5-badge-family-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-badge-family-proof/matrix.csv`
- Component report: `artifacts/components/m5-badge-family-components.md`
- Narrowed fixtures: `fixtures/ui/m5-badge-family-consumers/`

The Rust validator in `crates/aureline-release` is the authoritative gate; this
doc and the schema describe the shape. The headless emitter
(`aureline_release_freeze_m5_support_class_evidence`)
is the only mint-from-truth path for the artifacts above.

## Badge families (axes)

Each family is a distinct badge axis with its own controlled value vocabulary. No
family may imply, merge with, or stand in for another.

| Family | Values |
| --- | --- |
| `support_class` | `certified`, `fully_supported`, `community_supported`, `best_effort`, `deprecated`, `unsupported` |
| `evidence_freshness` | `fresh`, `recent`, `aging`, `stale`, `expired`, `unverified` |
| `lifecycle` | `stable`, `beta`, `preview`, `experimental`, `maintenance`, `end_of_life` |
| `channel` | `stable_channel`, `beta_channel`, `nightly_channel`, `edge_channel`, `lts_channel`, `custom_channel` |
| `deployment_scope` | `desktop_only`, `local_oss_scope`, `self_hosted_scope`, `managed_scope`, `air_gapped_scope`, `mirror_offline_scope` |
| `compatibility_state` | `compatible`, `minor_skew`, `major_skew`, `incompatible`, `migration_required`, `compatibility_unknown` |

## Explanation drawer

Every badge, on every surface, must be able to open an explanation drawer. The
controlled explanation fields are `what_it_means`, `why_shown`, `what_changes_it`,
`evidence_source`, `how_to_improve`, and `last_evaluated`. The first three are
**mandatory** on every badge — a badge is a compact contract *with an
explanation*, never an unexplained decoration.

## Axis-separation rules

Badges stay separate cues. These rules keep one family from implying another and
are the acceptance-criteria comparison rules for this lane:

- `support_class_does_not_imply_freshness` — Certified does **not** mean Fresh.
- `deployment_scope_does_not_imply_lifecycle` — Desktop-only does **not** mean Experimental.
- `lifecycle_does_not_imply_channel`
- `channel_does_not_imply_support_class`
- `compatibility_does_not_imply_support_class`
- `freshness_does_not_imply_compatibility`

## Hard invariants (must be false on every row)

- `collapses_multiple_axes_into_one_pill` — no overloaded pill merging two axes.
- `implies_freshness_from_support_class`
- `implies_lifecycle_from_deployment_scope`
- `drops_badge_meaning_in_export` — exported evidence never loses badge meaning.

## Mandatory labels

Every badge must be able to show `identity`, `value_state`, and `axis_name`
(mandatory), plus `explanation_drawer`, `evidence_source`, and `filter_key`. The
`axis_name` label is mandatory so a badge is never mistaken for a badge on another
axis, and `filter_key` keeps every axis separately filterable.

## Downgrade rules

A badge is auto-narrowed below its claim when any downgrade trigger fires:
`support_class_value_unstated`, `evidence_freshness_hidden`,
`lifecycle_value_unstated`, `channel_value_unstated`, `deployment_scope_unstated`,
`compatibility_state_unstated`, `explanation_drawer_missing`,
`axis_merged_into_another`, `freshness_implied_from_support_class`,
`filter_key_dropped`, `export_lost_badge_meaning`, or `proof_stale`. Stale proof
(older than the freshness SLO) narrows the family automatically.

## Consumers, deployment lines, accessibility

Consumer surfaces: `marketplace_ui`, `help_about`, `settings_ui`,
`onboarding_flow`, `diagnostics_surface`, `docs_portal`, `evaluation_pack`,
`support_export`, `cli_inspect`, `product_ui`. Every badge keeps the same meaning
across every deployment line (`local_oss`, `self_hosted`, `managed`, `air_gapped`,
`mirror_offline`) and declares non-visual accessibility routes, including
`non_color_encoded` (meaning is never color-alone) and `support_exportable`.

## Source contracts

This matrix layers on top of, and binds against, the already-claimed:

- `schemas/release/support_class_ledger.schema.json`
- `schemas/provenance/m5-freshness-descriptor.schema.json`
- `schemas/lifecycle/m5-lifecycle-vocabulary-parity.schema.json`
- `schemas/release/m5-compatibility-forecast.schema.json`
- `schemas/provenance/m5-badge-vocabulary.schema.json`

Raw URLs, raw keys, raw tokens, credentials, and user text bodies never cross this
boundary; the packet is metadata-only and export-safe.
