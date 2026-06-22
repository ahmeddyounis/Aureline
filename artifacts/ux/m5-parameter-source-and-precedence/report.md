# M5 Parameter-Source And Precedence Inspectors Across Forms

- Packet: `m5-parameter-source-and-precedence:stable:0001`
- Label: `M5 parameter-source and precedence inspectors — default, detected, imported, environment, policy, and user-override values with effective precedence, scope, and locks across M5 forms`
- As of: `2026-06-21T00:00:00Z`
- Fields: 7
- Effective: 4 certified, 1 narrowed, 1 review overlay, 0 unsafe, 1 labs

| Field | Form | Lane | Effective source | Origin | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- |
| field:provider-account-mapping:0001 | provider_account_mapping | provider | detected | provider_form | parameter_certified | parameter_certified |
| field:source-registration:0001 | source_registration | admin | policy_provided | remote_form | parameter_certified | parameter_certified |
| field:request-environment:0001 | request_environment | request | environment_resolved | remote_form | parameter_certified | parameter_narrowed |
| field:package-install-config:0001 | package_install_config | package | default | local_form | parameter_certified | parameter_certified |
| field:settings-config-editor:0001 | settings_config_editor | settings | user_override | local_form | parameter_certified | parameter_certified |
| field:import-migration-mapping:0001 | import_migration_mapping | import | imported | imported_review | parameter_review_overlay | parameter_review_overlay |
| field:project-bootstrap:0001 | project_bootstrap | projects | user_override | local_form | parameter_labs_not_claimed | parameter_labs_not_claimed |

- field:request-environment:0001: Held at parameter_narrowed below the parameter_certified claim: the verification proof is stale; the source stays inspectable until re-verified.
