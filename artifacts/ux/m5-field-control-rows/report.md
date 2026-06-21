# M5 Field And Control Row Primitives

- Packet: `m5-field-control-rows:stable:0001`
- Label: `M5 field and control rows — permanent labels, validation anchors, source-of-value tags, and lifecycle state across mutation-capable forms`
- As of: `2026-06-21T00:00:00Z`
- Rows: 13
- Effective: 10 certified, 1 narrowed, 1 review overlay, 0 blocked, 1 labs

| Row | Lane | Source | Lifecycle | Origin | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- |
| row:provider-endpoint:0001 | provider_account_mapping | detected_value | reconnect_required | first_party | row_certified | row_certified |
| row:provider-region:0001 | provider_account_mapping | default_value | none | first_party | row_certified | row_certified |
| row:provider-token:0001 | provider_account_mapping | user_override | trust_required | first_party | row_certified | row_certified |
| row:source-url:0001 | source_registration | user_override | none | first_party | row_certified | row_certified |
| row:source-kind:0001 | source_registration | default_value | none | first_party | row_certified | row_certified |
| row:source-trust-policy:0001 | source_registration | policy_locked | policy_blocked | first_party | row_certified | row_certified |
| row:request-environment-name:0001 | request_environment | detected_value | none | first_party | row_certified | row_certified |
| row:request-base-url:0001 | request_environment | required_unset | none | first_party | row_certified | row_certified |
| row:request-endpoint-health:0001 | request_environment | detected_value | reconnect_required | first_party | row_certified | row_narrowed |
| row:package-install-scope:0001 | package_install | detected_value | restart_required | first_party | row_certified | row_certified |
| row:package-target-dir:0001 | package_install | default_value | none | first_party | row_certified | row_certified |
| row:import-mapping:0001 | migration_import | imported_value | none | imported_or_restore | row_review_overlay | row_review_overlay |
| row:labs-import-preview:0001 | migration_import | default_value | none | first_party | row_labs_not_claimed | row_labs_not_claimed |

- row:request-endpoint-health:0001: Held at row_narrowed below the row_certified claim: async validation is still pending; the row stays usable and attributable until re-verified.
