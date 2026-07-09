# M5 Credential Component Surface Certification

- Packet: `m5-credential-component-certification:stable:0001`
- As of: `2026-07-09T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-credential-component-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Credential truth preserved on every surface: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:connector-authorization** — surface=connector_authorization claimed=verified_brokered certified=verified_brokered status=green narrowed_axes=0 credential_truth_preserved=true
- **cert:registry-authentication** — surface=registry_authentication claimed=verified_brokered certified=verified_brokered status=green narrowed_axes=0 credential_truth_preserved=true
- **cert:support-export** — surface=support_export claimed=handle_ready_projection certified=handle_ready_projection status=green narrowed_axes=0 credential_truth_preserved=true
- **cert:cli-headless** — surface=cli_headless claimed=handle_ready_projection certified=handle_ready_projection status=green narrowed_axes=0 credential_truth_preserved=true
- **cert:database-credential-attach** — surface=database_credential_attach claimed=verified_brokered certified=unverified_store_projection status=yellow narrowed_axes=1 credential_truth_preserved=true
- **cert:remote-target-attach** — surface=remote_target_attach claimed=verified_brokered certified=expired_auth_projection status=yellow narrowed_axes=1 credential_truth_preserved=true
- **cert:docs-help** — surface=docs_help claimed=verified_brokered certified=drifted_delegation_projection status=yellow narrowed_axes=1 credential_truth_preserved=true
- **cert:credential-audit-export** — surface=credential_audit_export claimed=verified_brokered certified=reveal_blocked_projection status=yellow narrowed_axes=1 credential_truth_preserved=true
