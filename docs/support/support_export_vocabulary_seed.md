# Support / export vocabulary seed

This document is the frozen word list shared by the support-bundle
manifest, the shell preview copy, and the support docs. Every token
here matches a closed enum in
[`/schemas/support/support_bundle_manifest.schema.json`](../../schemas/support/support_bundle_manifest.schema.json)
or
[`/schemas/support/support_bundle_preview_item.schema.json`](../../schemas/support/support_bundle_preview_item.schema.json).
The seed never invents private synonyms.

If this document disagrees with the schemas, the schemas control.

## Diagnostic data class

| Token | Meaning | Default posture under local-first |
|---|---|---|
| `metadata_only` | Build ids, version, policy fingerprints, summary counters. | Included by default. |
| `environment_adjacent` | Toolchain versions, target classes, route summaries. | Included as metadata. |
| `code_adjacent` | Filenames, stack traces, snippets. | Omitted until reviewer opts in. |
| `high_risk` | Secret-bearing material, raw dumps, full transcripts. | Prohibited or retained local only. |

## High-risk content class

Required whenever `data_class = high_risk`. A row that says only
"high risk" without naming the subtype is non-conforming.

| Token | Meaning |
|---|---|
| `not_applicable` | Allowed only on lower-risk rows. |
| `secret_bearing` | Tokens, credentials, raw secret material. |
| `raw_dump_or_memory` | Crash dumps, memory captures. |
| `full_shell_history` | Full terminal/shell history. |
| `raw_trace_or_transcript` | Raw renderer or session traces. |
| `policy_prohibited_unknown` | Unknown content blocked by policy. |

## Redaction state (visible chip)

| Token | Reviewer label |
|---|---|
| `not_required_metadata` | Metadata only — no redaction needed |
| `redacted_summary` | Redacted summary |
| `sanitized_snapshot` | Sanitized snapshot |
| `retained_local_only` | Retained local only — not exported |
| `omitted_pending_opt_in` | Omitted, awaiting opt-in |
| `prohibited` | Prohibited — never exported |
| `policy_locked` | Policy locked |

## Review decision class

| Token | Meaning |
|---|---|
| `included_default` | Included under the default rule. |
| `included_after_opt_in` | Included only after explicit reviewer opt-in. |
| `omitted_user_deselected` | Reviewer (or default deselect) removed the row. |
| `omitted_policy_locked` | Active policy locked the row out. |
| `omitted_prohibited` | Row was queued and rewritten because raw bytes are prohibited. |
| `stronger_redaction_applied` | Reviewer pinned a stronger redaction. |
| `retained_local_only` | Row is kept on the local machine. |

## Excluded reason class

Used inside `excluded_classes[]`. Every excluded row names exactly one.

| Token | Meaning |
|---|---|
| `not_requested` | Row class was not asked for in this preview. |
| `user_deselected` | Reviewer removed the row from the bundle. |
| `policy_denied` | Active policy denies including this row. |
| `prohibited_secret_or_token` | Secret-bearing content is prohibited from export. |
| `prohibited_full_shell_history` | Full shell history is prohibited from local-first export. |
| `retained_local_only_pending_review` | High-risk capture stays on the local machine only. |
| `awaiting_explicit_opt_in` | Code-adjacent row awaits explicit opt-in. |
| `source_unavailable_or_expired` | Underlying source disappeared before preview. |
| `not_collected_on_this_platform` | Row class is not collected on this platform. |

## Actionability impact class

Drives the chrome's warning copy when a reviewer attempts to remove or
further redact a row.

| Token | Meaning |
|---|---|
| `none` | Removing the row has no diagnostic cost. |
| `low` | Removing the row mildly reduces diagnostic depth. |
| `medium` | Removing the row meaningfully reduces diagnostic depth. |
| `high` | Removing the row reduces the chance of first actionable diagnosis. |
| `blocks_first_actionable_diagnosis` | Removing the row blocks first-actionable diagnosis entirely. |

## Policy-note severity

Used inside `collection_context.policy_notes[].severity`.

| Token | Meaning |
|---|---|
| `info` | Informational only. |
| `warning` | Reviewer should pay attention before exporting. |
| `blocking` | Manifest blocks export until the note is resolved. |

## Actor class

Used inside `collection_context.actor_class`.

| Token | Meaning |
|---|---|
| `user_initiated` | Reviewer asked for the bundle. |
| `admin_initiated` | Workspace admin asked for the bundle. |
| `headless_cli` | Headless CLI requested the bundle. |
| `support_center_preview` | Support-center surface minted a preview. |

## Trust state

Used inside `collection_context.policy_context.trust_state`.

| Token | Meaning |
|---|---|
| `untrusted` | Workspace trust has not been established. |
| `restricted` | Workspace is in restricted mode. |
| `trusted` | Workspace trust granted. |
| `managed_admin` | Managed-admin policy in effect. |

## Release-channel class

Used inside `build_identity.release_channel_class`. Mirrors the
build-info channel vocabulary; unknown tokens settle on `dev_local`
so the manifest never silently labels an unknown channel as `stable`.

| Token | Meaning |
|---|---|
| `stable` | Stable release. |
| `preview` | Preview release. |
| `beta` | Beta release. |
| `lts` | LTS release. |
| `portable_stable` | Portable stable build. |
| `portable_preview` | Portable preview build. |
| `dev_local` | Local dev build. |

## Action reconstruction fields

Reviewed-command rows use manifest fields rather than rendered text:

| Field | Meaning |
|---|---|
| `command_id` | Command Aureline believed it was running. |
| `command_descriptor_ref` | Descriptor or revision ref used for the command. |
| `invocation_session_id` | Invocation session joined to command result and incident packets. |
| `target_identity_ref` | Target identity ref or typed target token. |
| `action_origin_class` | Origin token from the route taxonomy. |
| `action_target_class` | Target token from the route taxonomy. |
| `action_route_class` | Route token from the route taxonomy. |
| `action_exposure_class` | Exposure token from the route taxonomy. |
| `policy_source` | Policy source, epoch, trust state, and optional bundle/context ref. |

## Stable command ids the seed routes

| Command id | Where the chrome routes it |
|---|---|
| `cmd:support.open_local_preview` | Open the local preview snapshot. |
| `cmd:support.copy_manifest_json` | Copy the manifest JSON for support hand-off. |

Reserved (not routed in the seed; reviewers see them as disabled rows):

- Share or upload action.
- Open hosted support intake.

## Stable rule refs (local-first default profile)

| Rule ref | Mirrors |
|---|---|
| `support.redaction.local_first_default.metadata_core` | Default metadata-core rule. |
| `support.redaction.local_first_default.managed_refs` | Managed-packet by-reference rule. |
| `support.redaction.local_first_default.high_risk_local` | High-risk local-only rule. |
| `support.redaction.local_first_default.review_code` | Code-adjacent review-required rule. |
| `support.redaction.local_first_default.exclude_shell_history` | Always-excluded full shell history rule. |

## Where the chrome reads each field

- The reviewer-visible chip text comes from `RedactionState::label`.
- The reviewer-visible "Held back from export" copy comes from
  `LocalFirstDefaults::explicit_reason_for(...)`.
- The reviewer-visible reviewer summary line comes from
  `LocalFirstDefaults::REVIEWER_SUMMARY_DEFAULT_OK` /
  `LocalFirstDefaults::REVIEWER_SUMMARY_PROHIBITED_PRESENT`.

The chrome must not duplicate these strings inline; the source of
truth is the seed crate.
