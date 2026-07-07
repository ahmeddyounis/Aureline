# M5 Provider-Account-Row Primitive

- Packet: `m5-provider-account-row-primitive:stable:0001`
- Label: `M5 provider-account-row primitive: provider identity, not-configured/signed-in/limited-scope/stale-session/offline-cached-read/policy-blocked connection state, tenant/org scope, effective write scope, token/session freshness, derived row posture, read/write/inspect access capability, and bounded reveal-scope/sign-in/retry/remove/export actions`
- Provider-surface consumers: 5 (5 stable)
- Row postures: not_configured_row, signed_in_row, limited_scope_row, stale_session_row, offline_cached_read_row, policy_blocked_row
- Access capabilities: can_read_and_write, can_read_write_limited, can_read_only_live, cached_inspect_only, no_access
- Connection states: not_configured, signed_in, limited_scope, stale_session, offline_cached_read, policy_blocked
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Provider-surface consumers

- **Account Settings Panel**: `stable`
  - Owner: Account settings panel owner
  - Scope: The account-settings panel renders the shared provider-account row so a signed-in organization member with full write and a fresh session reads as the highest-trust signed-in row exposing reveal/remove, and an unlinked account with nothing configured reads as a not-configured row that can only be signed in — never a generic connected chip
  - Worked rows: 2
    - `account:acme-eng:org-member` (`signed_in` / `full_write`) → `signed_in_row` (access `can_read_and_write`, can-write `true`, cached-only `false`)
    - `account:unconfigured:slot-1` (`not_configured` / `scope_unknown`) → `not_configured_row` (access `no_access`, can-write `false`, cached-only `false`)
- **Provider Status Bar**: `stable`
  - Owner: Provider status bar owner
  - Scope: The provider status bar renders the shared provider-account row so a limited-scope personal account with comment-only write reads as a limited-scope row that can still read and comment and offers retry, and a signed-in service account with a read-only scope reads as a live read-only row that never implies write — so a user can tell read from write from the row alone
  - Worked rows: 2
    - `account:personal:comment-scope` (`limited_scope` / `comment_only`) → `limited_scope_row` (access `can_read_write_limited`, can-write `true`, cached-only `false`)
    - `account:service:read-only` (`signed_in` / `read_only`) → `signed_in_row` (access `can_read_only_live`, can-write `false`, cached-only `false`)
- **Connection Picker**: `stable`
  - Owner: Connection picker owner
  - Scope: The connection picker renders the shared provider-account row so a stale-session org member with local drafts reads as a cached-inspect-only row that must re-authenticate before a live write yet keeps its drafts and offers retry, and an offline cached-read delegated credential reads as a cached-inspect-only row — neither ever presenting a cached read as a live read/write
  - Worked rows: 2
    - `account:acme-eng:stale-session` (`stale_session` / `full_write`) → `stale_session_row` (access `cached_inspect_only`, can-write `false`, cached-only `true`)
    - `account:delegated:offline-cached` (`offline_cached_read` / `no_write`) → `offline_cached_read_row` (access `cached_inspect_only`, can-write `false`, cached-only `true`)
- **Headless / CLI Accounts**: `stable`
  - Owner: Headless CLI accounts owner
  - Scope: The headless / CLI accounts surface renders the shared provider-account row so a policy-blocked installation grant with a revoked token reads as a no-access row that still offers retry and remove without blind re-entry, and a signed-in org member with status-only write on a near-expiry session reads as a limited read/write row that offers retry ahead of expiry — proving the same account grammar works headless
  - Worked rows: 2
    - `account:install-grant:policy-blocked` (`policy_blocked` / `no_write`) → `policy_blocked_row` (access `no_access`, can-write `false`, cached-only `false`)
    - `account:acme-eng:status-only` (`signed_in` / `status_only`) → `signed_in_row` (access `can_read_write_limited`, can-write `true`, cached-only `false`)
- **Support Account Export**: `stable`
  - Owner: Support account export owner
  - Scope: The support account export renders the shared provider-account row so a signed-in personal account with full write and local drafts reads as a read-and-write row whose export preserves support continuity without leaking credentials, and a limited-scope org member on a read-only scope near expiry reads as a live read-only row that offers retry — the same row a support agent reads elsewhere
  - Worked rows: 2
    - `account:personal:full-write` (`signed_in` / `full_write`) → `signed_in_row` (access `can_read_and_write`, can-write `true`, cached-only `false`)
    - `account:acme-eng:read-only-reviewer` (`limited_scope` / `read_only`) → `limited_scope_row` (access `can_read_only_live`, can-write `false`, cached-only `false`)
