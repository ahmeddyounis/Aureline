# Repair-class matrix examples

These fixtures are worked examples for the repair classes frozen in:

- [`/docs/recovery/repair_class_matrix.md`](../../../docs/recovery/repair_class_matrix.md)
- [`/artifacts/recovery/repair_classes.yaml`](../../../artifacts/recovery/repair_classes.yaml)

They exist to make preview requirements, checkpoint expectations,
reversal honesty, side-effect scope, and marketing/support caveats
obvious to reviewers without relying on improvised prose.

**Scope rules**

- These fixtures do not redefine the repair-transaction, doctor-finding,
  or route schemas. Where stable tokens are cited, they refer to the
  existing frozen vocabularies (support repair transaction and Doctor
  finding contracts).
- Monotonic timestamps and stable ids are opaque; they read well rather
  than reflect any real clock.

## Index

| Fixture | Scenario | Covered class |
|---|---|---|
| [`cache_invalidation_dispose_rebuild.yaml`](./cache_invalidation_dispose_rebuild.yaml) | rebuild a derived cache/index after integrity drift | dispose/rebuild |
| [`extension_quarantine_disable_quarantine.yaml`](./extension_quarantine_disable_quarantine.yaml) | quarantine a suspect extension | disable/quarantine |
| [`route_revocation_revoke_expire_route.yaml`](./route_revocation_revoke_expire_route.yaml) | revoke an unsafe/expired share route | revoke/expire route |
| [`policy_session_refresh_refresh_policy_session.yaml`](./policy_session_refresh_refresh_policy_session.yaml) | refresh policy/entitlement snapshot or expired session | refresh policy/session |
| [`rollback_extension_rollback_reinstall.yaml`](./rollback_extension_rollback_reinstall.yaml) | roll back an extension to a prior verified version | rollback/reinstall |
| [`support_export_export_escalate.yaml`](./support_export_export_escalate.yaml) | produce a reproducibility/support export instead of mutating | export/escalate |

