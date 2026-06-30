# M5 handoff-continuity review

Scenario set: `m5_handoff_continuity_scenario_set:default`

| Draft | Failure | Intended trust | Visibility | State | Intended data exit | Reusable offline? |
| --- | --- | --- | --- | --- | --- | --- |
| `handoff_draft:browser_blocked_public_issue` | Browser blocked | Official public | World-readable public | Captured offline | `metadata_safe_object_refs` | true |
| `handoff_draft:offline_community_support` | No network / offline | Community | Community visible | Captured offline | `metadata_safe_object_refs` | true |
| `handoff_draft:policy_denied_security` | Policy denied | Private / security | Private security channel | Staged for later | `security_payloads_only` | true |
| `handoff_draft:launch_failed_official_support` | Handoff launch failed | Official authenticated | Official account visible | Awaiting retry | `redacted_support_packet` | true |
| `handoff_draft:unsupported_profile_local` | Unsupported profile | Local only | Local, never leaves | Exported locally | `no_payload_leaves_product` | true |
| `handoff_draft:cleared_public_issue` | Browser blocked | Official public | World-readable public | Cleared | `metadata_safe_object_refs` | false |

Every preserved draft keeps the drafted text, attachments, redaction choices, and intended target class, nothing leaves the product while a draft is held, and a failed security/private route is never silently redirected to a public target.
