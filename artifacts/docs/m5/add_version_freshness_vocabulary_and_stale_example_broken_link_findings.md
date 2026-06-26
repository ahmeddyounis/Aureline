# Docs Version/Freshness State And Stale-Example Findings

- Packet: `packet:docs_version_freshness_findings:001`
- Surface: `workflow:docs_version_freshness_state_and_stale_example_broken_link_findings:stable`
- Promotion: `stable` (0 validation findings)
- Cards: 8 / Findings: 5 / Surfaces: 6

## Cards

- **tokio::runtime spawn (exact)** (`card:exact:async-runtime-api`): state `exact` / confidence `current_exact`
- **reqwest client guide (nearby)** (`card:nearby:http-client-guide`): state `nearby` / confidence `qualified_nearby`
   - active `package:reqwest@0.12.5` vs viewed `docs:reqwest@0.12.2`
- **Workspace architecture overview (project)** (`card:project_specific:workspace-architecture`): state `project_specific` / confidence `project_scoped`
- **serde derive guide (mirrored)** (`card:mirrored:serde-derive`): state `mirrored` / confidence `mirrored_verified`
   - active `package:serde@1.0.203` vs viewed `mirror:serde@1.0.203`
- **cargo CLI reference (cached)** (`card:cached:cli-reference`): state `cached` / confidence `cached_unverified`
   - active `toolchain:cargo@1.84.0` vs viewed `docs-cache:cargo@1.81.0`
- **axum migration guide (stale)** (`card:stale:migration-guide`): state `stale` / confidence `not_current`
   - active `package:axum@0.8.1` vs viewed `docs:axum@0.7.0`
- **Enterprise rotation runbook (policy-blocked)** (`card:policy_blocked:enterprise-runbook`): state `policy_blocked` / confidence `inline_unavailable`
- **Vendor changelog (browser handoff)** (`card:browser_handoff:hosted-changelog`): state `browser_handoff_required` / confidence `inline_unavailable`

## Findings

- `finding:stale-example:axum-router` [stale_example/code_block/advisory] on `card:stale:migration-guide`: the router example predates the active major and no longer compiles
- `finding:broken-link:reqwest-anchor` [broken_link/link/advisory] on `card:nearby:http-client-guide`: the anchor moved between minors and the in-doc link no longer resolves
- `finding:nearby-version:reqwest-builder` [nearby_version_example/command/advisory] on `card:nearby:http-client-guide`: a nearer-version example exists for the active minor
- `finding:removed-api:cargo-flag` [removed_api_reference/api_reference/advisory] on `card:cached:cli-reference`: the referenced unstable flag was removed in the active toolchain
- `finding:changed-config:serde-rename` [changed_config_path/config_path/advisory] on `card:mirrored:serde-derive`: the documented attribute value differs from the current pack metadata
