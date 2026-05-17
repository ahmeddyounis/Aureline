# Support Center beta — shell surface, claim truth, and export routing

## What this beta lane owns

The Support Center beta is the **one keyboard-reachable shell surface**
where a blocked user can:

1. Launch safe mode, Project Doctor, extension bisect, repair preview,
   or the support-bundle preview — without searching docs.
2. See the **current claim and degraded-state truth** for the M3 beta
   lanes alongside those launch actions, instead of inferring it from
   chrome badges in unrelated panels.
3. Follow an **export route** to the local-first support-bundle
   preview, the typed Doctor finding record, the recovery-ladder
   support packet, the object-handoff packet draft, or the escalation
   packet draft — every route is local-first; no route ever resolves
   to an upload-first first action.

The surface is bounded to the projection layer at this milestone. It
does not own:

- The live recovery decisions. Those live in
  `crates/aureline-support/src/recovery_ladder/` and
  `crates/aureline-support/src/safe_mode/`.
- The live scorecards. Those live in
  `crates/aureline-support/src/scenario_scorecard/` and the M3 drill
  harness at `artifacts/support/m3/drill_harness_report.md`.
- The live support bundles. Those live in
  `crates/aureline-support/src/bundle/`.
- The intake-bound capability registry. That lives in
  `schemas/support/scenario_picker.schema.json` and the IA cards in
  `artifacts/support/support_center_routes.yaml`.

This doc is the reviewer contract for the shell-side **projection**
that consumes those owners.

## Acceptance contract

Every conforming `support_center_beta_surface_record` MUST:

- Pin `schema_version: 1` and the boundary schema at
  `schemas/support/support_center_beta.schema.json`.
- Name a `primary_command_id` that starts with `cmd:` and a
  `primary_keyboard_reach_class` so the surface is reachable from the
  command palette without pointer input.
- Carry at least **five launch-action rows**: one each for
  `enter_safe_mode`, `open_project_doctor`, `start_extension_bisect`,
  `open_repair_preview`, and at least one **export action**
  (`preview_support_bundle`, `export_support_bundle`,
  `export_object_handoff_packet`, or `export_escalation_packet_draft`).
- Quote at least one **claim-truth row** that names a beta lane, its
  `claim_state_class`, the `scorecard_target` id, and the
  `evidence_packet_ref` the user can open to read the claim's
  evidence.
- Quote at least one **degraded-truth row**. When nothing is
  degraded, the row MUST still be present with
  `degraded_truth_class: none_degraded`, `active: false`, and no
  `exit_command_id`. When something is degraded, the row MUST be
  `active: true` and MUST name an `exit_command_id` so the exit path
  is not buried in another panel.
- Quote at least one **export-route row** with
  `local_first_path_named: true` and
  `upload_required_for_first_action: false`. A row that resolves to
  `destination_class: no_export_destination` MUST carry the literal
  token `no_destination_schema` for `destination_schema_ref` rather
  than fabricate a path.
- Pin `no_account_required_for_local_only: true` and
  `no_hidden_service_required_for_local_only: true`.

On every launch-action row, the projection MUST additionally:

- Preserve `user_authored_files` in `preserved_state_classes`. A
  launch action that would delete user-owned state is non-conforming
  and the evaluator refuses it.
- Bind the row to a stable `cmd:` id; no fake commands.
- Pin `no_account_required` and `local_only` /
  `optional_mirror_cache` / `optional_managed_policy_sync` service
  dependency on every row whose `beta_lane_class` is a local-only
  recovery lane (safe mode, doctor probe packs, extension bisect,
  repair-transaction preview, project-doctor finding contract,
  recovery ladder, crash triage). Local-only recovery lanes never
  gate on an account or a hosted service.

The evaluator that proves these rules lives at
`crates/aureline-shell/src/support_center/mod.rs`. The protected
fixture corpus at `fixtures/ux/m3/support_center/` exercises three
deployment contexts: a local-only individual install, a post-crash
safe-mode-active surface, and a managed-workspace surface. The
integration test
`crates/aureline-shell/tests/support_center_beta_fixtures.rs` replays
the corpus and re-asserts each acceptance row.

## Guardrails

- **Narrow repair, explicit compensation.** Launch actions point at
  the typed repair-transaction preview skeleton from
  `crates/aureline-support/src/repair_transactions/`. The surface
  never offers a magical reset path.
- **Preserve last-known-good truth.** Every launch row preserves
  `user_authored_files`; rows that touch durable state must name the
  relevant preserved-state class so the projection cannot silently
  promise something the underlying lane refuses.
- **No hidden account or service gates.** The closed
  `account_requirement_class` and `service_dependency_class`
  vocabularies make any gate explicit. A surface that requires
  account creation or a hosted service for a local-only recovery lane
  is refused by `SupportCenterBetaEvaluator`.
- **Claim truth is opened, not inferred.** Every claim-truth row
  carries an `evidence_packet_ref`; the user can open the same
  evidence that release review consumes.

## Out of scope at this milestone

- Live runtime rendering of the surface in the running shell. The
  projection is the contract that the upcoming chrome implementation
  will consume.
- Hosted ticket intake, cross-tenant escalation, and managed
  case-management features. The escalation packet draft destination
  is local-first; the surface never auto-uploads.
- Cross-window or cross-device surface mirroring. The surface stays
  bound to one workspace at this milestone.
