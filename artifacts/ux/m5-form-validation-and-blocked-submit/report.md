# M5 Form Validation, Cross-Field Dependencies, and Blocked-Submit Reasons

- Packet: `m5-form-validation:stable:0001`
- Label: `M5 form validation — form-level summaries, cross-field dependencies, and machine-readable blocked-submit reasons across mutation-capable forms`
- As of: `2026-06-21T00:00:00Z`
- Forms: 8
- Effective: 5 certified, 1 narrowed, 1 review overlay, 0 blocked, 1 labs

| Form | Lane | Origin | Deps | Blockers | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- |
| form:provider-connection:0001 | provider | provider_backed | 1 | 1 | form_certified | form_certified |
| form:settings-config:0001 | settings | local_authoring | 0 | 0 | form_certified | form_certified |
| wizard:project-bootstrap:0001 | projects | local_authoring | 0 | 1 | form_certified | form_certified |
| sheet:package-install:0001 | package | local_authoring | 1 | 2 | form_certified | form_certified |
| sheet:admin-policy-rollout:0001 | admin | local_authoring | 1 | 1 | form_certified | form_certified |
| dialog:request-run:0001 | request | remote_target | 1 | 1 | form_certified | form_narrowed |
| dialog:migration-restore:0001 | import | imported_or_restore | 1 | 1 | form_review_overlay | form_review_overlay |
| wizard:labs-onboarding:0001 | projects | local_authoring | 0 | 0 | form_labs_not_claimed | form_labs_not_claimed |

- dialog:request-run:0001: Held at form_narrowed below the form_certified claim: async validation is still pending; the form stays usable and reopenable until re-verified.
