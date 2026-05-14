# Support-bundle redaction guide

This guide documents the redaction defaults the support-bundle seed
applies before any local preview is rendered. It is a reviewer-facing
companion to the local-first profile fixture at
[`/fixtures/support/redaction_profiles/local_first_default.yaml`](../../fixtures/support/redaction_profiles/local_first_default.yaml)
and the boundary schemas at
[`/schemas/support/support_bundle_manifest.schema.json`](../../schemas/support/support_bundle_manifest.schema.json)
and
[`/schemas/support/support_bundle_preview_item.schema.json`](../../schemas/support/support_bundle_preview_item.schema.json).

If this document disagrees with the schemas or the YAML profile, the
schemas and the YAML control. The seed never invents private posture
labels.

## Default profile

Profile id: `support.redaction.local_first_default`

The seed applies one rule per row, derived from the row's diagnostic
data class and (for high-risk rows) the high-risk subtype. A row's
posture answers four questions at once:

| Question | Where to read it |
|---|---|
| What chip does the reviewer see? | `redaction.redaction_state` |
| What did the default decision rule do with the row? | `review_decisions[].decision_class` |
| Is the row kept out of the export? | `excluded_classes[]` (when present) |
| Is the row prohibited (raw bytes never travel)? | `redaction_report.prohibited_items_confirmed_absent` |

## Posture matrix

| Data class | High-risk subtype | Redaction state | Decision | Held back from export? |
|---|---|---|---|---|
| `metadata_only` | n/a | `not_required_metadata` | `included_default` | No |
| `environment_adjacent` | n/a | `not_required_metadata` | `included_default` | No |
| `code_adjacent` | n/a | `omitted_pending_opt_in` | `omitted_user_deselected` | Yes — `awaiting_explicit_opt_in` |
| `high_risk` | `secret_bearing` | `prohibited` | `omitted_prohibited` | Yes — `prohibited_secret_or_token` |
| `high_risk` | `full_shell_history` | `prohibited` | `omitted_prohibited` | Yes — `prohibited_full_shell_history` |
| `high_risk` | `raw_dump_or_memory`, `raw_trace_or_transcript`, `policy_prohibited_unknown` | `retained_local_only` | `retained_local_only` | Yes — `retained_local_only_pending_review` |

The matrix is enforced in code by
`aureline_support::bundle::LocalFirstDefaults::posture_for(...)`. The
chrome and the export writer do not reimplement this rule; they read
the resolved row.

## How to interpret a redaction state

| State | Reviewer label | Meaning before export |
|---|---|---|
| `not_required_metadata` | "Metadata only — no redaction needed" | Row embeds in the export verbatim. |
| `redacted_summary` | "Redacted summary" | Row exports as a sanitized summary; raw bytes never travel. |
| `sanitized_snapshot` | "Sanitized snapshot" | Row exports as a snapshot with sensitive fields redacted. |
| `retained_local_only` | "Retained local only — not exported" | Row stays on the local machine; the manifest names it but does not embed it. |
| `omitted_pending_opt_in` | "Omitted, awaiting opt-in" | Row was queued but the local default holds it back until the reviewer opts in. |
| `prohibited` | "Prohibited — never exported" | Row was queued and rewritten; raw bytes never travel under any default. |
| `policy_locked` | "Policy locked" | Active policy locks the row out of the export; the manifest carries the typed reason. |

## Honesty markers

The preview surface lights `honesty_marker_present = true` whenever:

- at least one queued row was rewritten to `prohibited`, **or**
- at least one row's size estimate is unknown (the seed cannot
  estimate the bytes the export would carry).

When the marker is lit, the chrome must show the honesty banner; it
must not render an "all clear" copy line.

## Secret-scan summary contract

`redaction_report.secret_scan_summary.raw_secret_values_exported` is
pinned to `false` by the schema and by the seed. If a future profile
ever needs to flip this bit, that change has to land in a separate
profile id; this seed never writes `true` to that field.

## Redaction controls

Every preview row also has a matching `redaction_controls[]` entry in
the manifest. Controls expose the default state, the selected state, and
the narrower states a reviewer may choose without broadening capture.
They do not expose raw export in the alpha path:

- `raw_content_export_allowed` is always `false`;
- `broadening_requires_review` is always `true`;
- prohibited rows have no allowed narrower states and stay visible only
  as omission markers;
- code-adjacent rows can only be kept omitted or narrowed to local-only
  retention until a separate reviewed packet exists.

## Reviewing a manifest

A reviewer asking "did this bundle leak something it should not have?"
reads three fields in order:

1. `build_identity.exact_build_refs` — confirm the manifest names a
   real build. The seed pins this list to non-empty on every preview.
2. `excluded_classes[]` — every row the defaults held back is
   enumerated here with a typed reason and an explicit reason
   sentence.
3. `redaction_report.prohibited_items_confirmed_absent` — every row
   whose raw bytes would be a secret-class leak is named here. The
   list is empty on preview-only previews and non-empty on the failure
   drill.

If any of these fields are missing or empty when they should not be,
the manifest is non-conforming and intake must reject it.

## Failure drill

The failure drill is a synthetic queue that includes a row marked
`data_class = high_risk`, `high_risk_content_class = secret_bearing`.
The seed proves three things on this drill:

1. The row's `redaction_state` is rewritten to `prohibited` before any
   preview render.
2. The manifest carries an `excluded_classes[]` entry that names the
   row's `support.item.raw_secrets` id with the
   `prohibited_secret_or_token` reason.
3. The `redaction_report.high_risk_items` array carries an entry that
   records the handling summary; the row stays visible in preview so a
   reviewer can confirm the seed caught it, but no bytes from the row
   are exported.

The drill is exercised by the unit and integration tests under
`crates/aureline-support/` and `crates/aureline-shell/`.
