# Signed Template Registry, Provenance/Mirror, and Template-Health Rows

- Packet: `signed-template-registry:stable:0001`
- Label: `Signed Template Registry, Provenance/Mirror, and Template-Health Rows`
- Rows: 4 (3 admitted for generation)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Rows

- **template:first_party.rust.cli_tool:01** `0.4.2`: official_origin / officially_supported (core_certified)
  - Scope: Officially-supported Rust CLI starter anchored in the core signing root; trust, certification, and update behavior stay inspectable before generation
  - Trust: core_signing_root (signature: author_and_organization_signature)
  - Freshness: live_origin
  - Health: healthy_current (cadence: on_every_registry_refresh, admitted: true)
- **template:official.ts.web_application:01** `1.8.0`: org_mirror / officially_supported (org_approved)
  - Scope: Organization mirror of the official TypeScript web app starter; the mirror is stale, so generation requires refresh or review while the upstream origin and mirror freshness stay inspectable
  - Trust: org_mirror_signing_root (signature: organization_signature_only)
  - Freshness: mirror_stale
  - Health: stale_but_inspectable (cadence: daily, admitted: false)
- **template:community.python.data_workbench:07** `2.1.0`: community_origin / community_supported (community_reviewed)
  - Scope: Community-reviewed Python data workbench starter signed by its channel; support class and any known issues stay explicit and bridge behavior is never shown as exact first-party truth
  - Trust: community_channel_signature (signature: author_signature)
  - Freshness: live_origin
  - Health: known_issue_non_blocking (cadence: weekly, admitted: true)
- **template:repo_local.node.backend_service:02** `0.1.0`: repo_local_generator / support_unknown (repo_local_unreviewed)
  - Scope: Repo-local generator scoped to this workspace; resolves through workspace trust only and cannot claim certification by location
  - Trust: repo_local_workspace_trust (signature: unsigned)
  - Freshness: unknown_freshness
  - Health: healthy_cached (cadence: on_template_revision_change, admitted: true)
