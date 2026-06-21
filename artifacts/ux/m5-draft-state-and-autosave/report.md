# M5 Draft State, Autosave Journals, and Recover-Draft Semantics

- Packet: `m5-draft-state:stable:0001`
- Label: `M5 draft state — local autosave journals, explicit draft-versus-applied truth, and recover-draft semantics across mutation-capable surfaces`
- As of: `2026-06-21T00:00:00Z`
- Surfaces: 8
- Effective: 5 certified, 1 narrowed, 1 review overlay, 0 blocked, 1 labs

| Surface | Lane | Origin | Persistence | Draft/Applied | Recovery | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- | --- |
| form:provider-connection:0001 | provider | provider_backed | local_journal | draft_only | recoverable | draft_certified | draft_certified |
| form:settings-config:0001 | settings | local_authoring | committed_local | applied | no_journal | draft_certified | draft_certified |
| wizard:project-bootstrap:0001 | projects | local_authoring | unsaved_in_memory | draft_only | no_journal | draft_certified | draft_certified |
| sheet:package-install:0001 | package | local_authoring | local_durable_checkpoint | partially_applied | recoverable | draft_certified | draft_certified |
| sheet:admin-policy:0001 | admin | remote_target | committed_remote | applied | no_journal | draft_certified | draft_certified |
| dialog:request-run:0001 | request | remote_target | local_journal | draft_only | recoverable | draft_certified | draft_narrowed |
| dialog:migration-restore:0001 | import | imported_or_restore | local_durable_checkpoint | draft_only | recovered | draft_review_overlay | draft_review_overlay |
| wizard:labs-onboarding:0001 | projects | local_authoring | unsaved_in_memory | draft_only | no_journal | draft_labs_not_claimed | draft_labs_not_claimed |

- dialog:request-run:0001: Held at draft_narrowed below the draft_certified claim: an autosave write is still in flight; the draft stays recoverable and reopenable until re-verified.
