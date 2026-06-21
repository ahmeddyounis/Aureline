# M5 Structured Input, Parameter Provenance, Draft State, and Staged Review

- Packet: `m5-structured-input:stable:0001`
- Label: `M5 structured input — field provenance, validation, draft state, and staged review across mutation-capable forms`
- As of: `2026-06-21T00:00:00Z`
- Surfaces: 8
- Effective: 5 certified, 1 narrowed, 1 review overlay, 0 unsafe, 1 labs

| Surface | Kind | Lane | Mutation | Origin | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- |
| form:provider-credentials:0001 | structured_form | provider | provider_backed | provider_backed | surface_certified | surface_certified |
| form:settings-config:0001 | structured_form | settings | local | local_authoring | surface_certified | surface_certified |
| wizard:project-bootstrap:0001 | multi_step_wizard | projects | local | local_authoring | surface_certified | surface_certified |
| sheet:package-install-review:0001 | install_review_sheet | package | local | local_authoring | surface_certified | surface_certified |
| sheet:admin-policy-rollout:0001 | publish_review_dialog | admin | policy_locked | local_authoring | surface_certified | surface_certified |
| dialog:request-workspace-run:0001 | parameterized_workflow | request | remote | remote_target | surface_certified | surface_narrowed |
| dialog:migration-restore-review:0001 | import_restore_dialog | import | import_export | imported_or_restore | surface_review_overlay | surface_review_overlay |
| wizard:experimental-onboarding:0001 | multi_step_wizard | projects | local | local_authoring | surface_labs_not_claimed | surface_labs_not_claimed |

- dialog:request-workspace-run:0001: Held at surface_narrowed below the surface_certified claim: the verification proof is stale; the draft stays recoverable and reopenable until re-verified.
