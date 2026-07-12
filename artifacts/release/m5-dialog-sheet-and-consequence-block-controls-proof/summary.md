# M5 Dialog / Sheet and Consequence-Block Controls

- Packet: `m5-dialog-sheet-and-consequence-block-controls:stable:0001`
- Label: `M5 dialog / sheet and consequence-block controls with stable title/rationale/scope anatomy, explicit action labels, safe initial focus, cancel paths, help/docs hooks off generic Yes/No, focus-return and reopen continuity, and consequence blocks naming affected object, blast radius, and rollback/help posture across review, settings, update/install, repair, shell, and support surfaces`
- Consumer surfaces: 6
- Dialog action models: named_specific_actions, primary_and_cancel, destructive_confirm_named, rationale_and_scope_stated, dismissible_safe, generic_yes_no_disallowed
- Blast radii: single_object, multiple_objects, workspace_wide, deployment_wide, irreversible_external, radius_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer surfaces

- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review confirmation names its rationale, scope, and specific actions and carries a consequence block that names the blast radius; both degrade honestly when the dialog reduces to generic Yes/No or the consequence block cannot resolve its blast radius
  - Dialog examples: 2 / consequence examples: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings trust dialog names a primary action plus an explicit cancel and states its rationale, and its consequence block states rollback availability; both degrade honestly when the rationale is unstated or the rollback posture is unstated
  - Dialog examples: 2 / consequence examples: 2
- **updates_ui**: `stable`
  - Owner: Update / install owner
  - Scope: The update / install dialog names its destructive confirm and its consequence block states the deployment-wide, irreversible blast radius; both degrade honestly when the scope is unstated or the reversibility posture cannot be resolved
  - Dialog examples: 2 / consequence examples: 2
- **support_ui**: `stable`
  - Owner: Repair / support surface owner
  - Scope: The repair confirmation states rationale and scope with a safe initial focus, and its consequence block carries a help path reachable off-screenshot; both degrade honestly when the initial focus is unsafe or the consequence explanation is screenshot-only
  - Dialog examples: 2 / consequence examples: 2
- **shell_ui**: `stable`
  - Owner: Shell / entry surface owner
  - Scope: The shell trust dialog is dismissible with a safe default and a cancel path, and its consequence block names explicit actions; both degrade honestly when the cancel path is missing or the consequence block reduces to generic Yes/No ambiguity
  - Dialog examples: 2 / consequence examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved dialog and consequence truth, so a broken focus return on reopen or an unstated consequence label is visible in evidence rather than hidden behind a screenshot
  - Dialog examples: 2 / consequence examples: 2
