# Stable Provider Account / Install-Grant Registry

This artifact documents the stabilized provider account/install-grant registry
landed under `crates/aureline-provider/src/stabilize_provider_account_install_grant_registry/`.

## Goal

Turn provider-owned work items into governed local-first companion objects by
landing explicit provider account/install-grant records, board/project mapping,
and action-mode disclosure.

## Record family

- `StableProviderAccountRecord` — connected-account entry with provider descriptor,
  canonical host, org/tenant scope, acting-as identity, health state, and
  supported object types.
- `StableInstallGrantRecord` — installation-grant entry with issuer, bounded
  scope, grant lifecycle, and supported object types.
- `StableMappingReviewRow` — mapping-review row showing current target, fallback
  mapping, stale or policy-blocked state, and available action mode.
- `StableRegistryRecord` — top-level registry record binding the page.
- `StableRegistrySupportExportPacket` — redaction-safe support export.
- `StableRegistryInspectionRecord` — compact boolean projection for CLI and
  inspector surfaces.

## Action modes

Every claimed stable provider lane explicitly discloses one of:

- `read_only`
- `comment_or_link`
- `full_edit`
- `offline_capture_only`
- `publish_later`
- `handoff_only`

## Health states

- `healthy`
- `degraded_stale_credentials`
- `degraded_limited_scope_session`
- `blocked_policy_locked_mapping`
- `blocked_provider_unreachable`
- `blocked_auth_loss`
- `offline_capture_only`

## Guardrails

- No raw token material is present on any record.
- No raw provider payload crosses the support boundary.
- `raw_url_export_allowed` and `raw_provider_payload_export_allowed` are always
  `false` on support exports.
- Health-state / action-mode coherence is enforced: a health state that blocks
  mutation cannot pair with an action mode that admits mutation.

## Fixture

Canonical fixture: `fixtures/providers/m4/stabilize-provider-account-install-grant-registry/registry_packet.json`
