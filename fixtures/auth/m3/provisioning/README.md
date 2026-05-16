# Provisioning hook beta fixtures

Reviewer-facing fixtures for the SCIM and signed-file provisioning hook
projection consumed by the admin-audit export beta page.

The canonical record kind is
`security_admin_audit_export_beta_provisioning_event_record`. The module
lives at
[`/crates/aureline-auth/src/provisioning/mod.rs`](../../../../crates/aureline-auth/src/provisioning/mod.rs).
The full admin-audit export beta page (provisioning + policy-bundle history +
entitlement changes) lives under
[`/fixtures/security/m3/admin_audit_export/`](../../../security/m3/admin_audit_export/README.md)
and the contract docs live at
[`/docs/security/m3/provisioning_and_audit_beta.md`](../../../../docs/security/m3/provisioning_and_audit_beta.md).

## Files

| File | Purpose |
| --- | --- |
| `events.json` | All seeded SCIM and signed-file provisioning hook events across connected, mirror-only, offline, and enterprise-managed profiles. |

Regenerate via the headless inspector:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- provisioning-events > events.json
```
