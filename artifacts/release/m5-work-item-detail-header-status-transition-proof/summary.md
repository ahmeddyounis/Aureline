# Work-item detail headers and status-transition sheets

- Packet: `m5-work-item-detail-header-status-transition-controls:stable:0001`
- Surface: `M5 work-item detail headers and status-transition sheets: durable headers state provider space, canonical id, title, state, owner, derived write scope and freshness, and an open-external escape hatch, so a local draft never reads as a provider-backed object; transition sheets preview comment/state/assignment/link/field mutations, linked branch/review context, notification side effects, and the permission scope that can authorize the change, with confirm/export/cancel behavior and a metadata-safe export fallback before any publish`
- Detail headers: 5 (1 local drafts)
- Status-transition sheets: 5 (2 publish externally)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Detail headers

- **header-checkout-rounding** (PROJ-1421) [provider_writable / live_synced] → `In Progress`
- **header-local-triage-note** (LOCAL-0007) [local_draft_only / local_only] → `Draft`
- **header-imported-change-request** (EXT-5521) [read_only_mirror / stale_snapshot] → `Approved (snapshot)`
- **header-failover-incident** (INC-3390) [policy_blocked_write / live_synced] → `Investigating`
- **header-mirror-unknown-freshness** (MIR-8830) [read_only_mirror / unknown_freshness] → `Open`

## Status-transition sheets

- **sheet-local-triage-state** `Draft` → `Triaging` [local_draft_only] auth: current_user_authorized
- **sheet-publish-comment** `In Progress` → `In Review` [publishes_to_provider] auth: needs_provider_auth
- **sheet-open-in-provider-link** `In Review` → `Linked` [opens_in_provider] auth: needs_elevated_role
- **sheet-blocked-assignment** `Investigating` → `Assigned` [blocked_needs_permission] auth: needs_elevated_role
- **sheet-policy-blocked-field** `Assigned` → `Escalated` [policy_blocked_transition] auth: policy_restricted
