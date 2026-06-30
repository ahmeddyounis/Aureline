# M5 community-handoff target review

Sheet set: `m5_community_handoff_target_sheet_set:default`

| Route | Trust class | Visibility | Auth | Data exit | Commitment | Guaranteed? |
| --- | --- | --- | --- | --- | --- | --- |
| `public_issue` | Official public | World-readable public | Community account typical | `metadata_safe_object_refs` | No commitment (public forum) | false |
| `security_disclosure` | Private / security | Private security channel | Security channel credential | `security_payloads_only` | Security handled privately | false |
| `docs_feedback` | Official public | World-readable public | Community account typical | `metadata_safe_object_refs` | No commitment (public forum) | false |
| `rfc_discussion` | Community | Community visible | Community account typical | `proposal_refs_only` | Best-effort community | false |
| `community_support` | Community | Community visible | Community account typical | `metadata_safe_object_refs` | Best-effort community | false |
| `official_support` | Official authenticated | Official account visible | Official account required | `redacted_support_packet` | Official supported commitment | true |

Every route carries a local-safe fallback that never leaves the product, and world-readable routes require prior review and never auto-open from a critical alert.
