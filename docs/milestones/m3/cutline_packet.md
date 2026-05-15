# M3 cutline packet

This page is the reviewer-facing entrypoint for the M3 exit cutline. The
canonical truth lives in the JSON/YAML artifacts named below; the cutline
packet, unlock map, and stable-planning handoff checklist must agree, and any
widening of the beta scope must update all three in the same change set.

The cutline packet names the non-descopable M3 truths that gate stable
planning. It does not duplicate beta scope; the beta admission matrix freezes
*what* is in beta, and this packet freezes *what must be green* before any
stable lane is allowed to start.

## Canonical artifacts

- Cutline packet (this page): `docs/milestones/m3/cutline_packet.md`
- Unlock map: `artifacts/milestones/m3/unlock_map.yaml`
- Stable-planning handoff checklist: `artifacts/milestones/m3/stable_planning_handoff_checklist.md`
- Cutline validator: `ci/check_m3_cutline_packet.py`
- Latest validation capture: `artifacts/milestones/m3/captures/cutline_packet_validation_capture.json`

## Upstream contracts inherited from the M3 admission freeze

- Beta admission matrix: `docs/milestones/m3/beta_admission_matrix.md`
- Claimed-surface register: `artifacts/milestones/m3/claimed_surface_register.json`
- Cohort guardrails: `artifacts/milestones/m3/cohort_guardrails.yaml`
- Dependency graph: `artifacts/milestones/m3/dependency_graph.mmd`
- Beta admission validator: `ci/check_beta_admission.py`

## Definition of green

The M3 cutline is green when:

- every non-descopable truth row below names a primary cutline cohort, an
  evidence class, and at least one downgrade rule that propagates into the
  beta admission lane;
- every cutline row maps to one or more `beta_surface:` ids from the claimed
  surface register;
- the unlock map lists every M3 lane that must be green before stable API
  freeze, release-center rehearsal, and certified-archetype publication can
  proceed;
- the stable-planning handoff checklist names the artifacts M4 will consume
  and explicitly enumerates which beta-only surfaces stay beta-only;
- held / out-of-scope rows from the claimed-surface register are mirrored in
  the unlock map with the same widening rules so beta scope cannot widen by
  omission; and
- the cutline validator passes.

Individual lane proof rows may still be `evidence_pending`. The cutline only
freezes the *claim* that the row is non-descopable; the proof lanes turn the
rows green.

## Non-descopable M3 truths

These rows must be green before stable planning is allowed to consume the
handoff checklist. The IDs are stable; cite them from M4 proof packets,
dashboards, and exit reviews.

| Cutline id | Truth | Anchoring beta surface(s) | Evidence class |
|---|---|---|---|
| `m3_cutline:rollback_proven_in_partner_orgs` | Update and rollback are proven in real partner organizations on supported channels, with no user data loss and a documented downgrade path. | `beta_surface:packaging_update_rollback` | `rollback_drill`, `partner_scorecard`, `update_verification_capture` |
| `m3_cutline:policy_explainability_baseline` | Enterprise policy, proxy, and identity envelopes are inspectable end-to-end with a single command, and the policy bundle schema is frozen against silent drift. | `beta_surface:policy_proxy_transport` | `policy_bundle_packet`, `enterprise_proxy_lab`, `identity_envelope_capture` |
| `m3_cutline:extension_isolation_envelope` | Extension runtime, SDK, and publication path enforce an inspectable isolation envelope: capability budgets, quarantine triggers, and rollback drills are reproducible. | `beta_surface:extension_runtime` | `sdk_diff_report`, `extension_compatibility_report`, `extension_rollback_drill` |
| `m3_cutline:migration_honesty_register` | Importer, migration parity, and known-gap honesty are mirrored in one register that partners can read; no parity claim outruns its evidence. | `beta_surface:importer_and_migration` | `import_bridge_report`, `migration_diff_packet`, `known_gap_register` |
| `m3_cutline:supportability_baseline` | Support export, diagnostics, and recovery are reproducible on partner hardware; safe-mode and remote-attach degradation envelopes are documented and drilled. | `beta_surface:support_export_diagnostics` | `support_bundle_capture`, `safe_mode_drill`, `remote_attach_degradation_audit` |
| `m3_cutline:debug_test_task_truth` | Debug session, test runner, and task panels are inspectable, reproducible, and bound to current certified-archetype evidence. | `beta_surface:debug_test_task_model` | `reference_workspace_report`, `task_event_run_trace`, `debug_session_trace`, `test_runner_capture` |
| `m3_cutline:certified_archetype_publication` | Certified-archetype compatibility publication is reproducible end-to-end and the report template is wired to the canonical archetype rows. | `beta_surface:compatibility_publication` | `certified_archetype_report`, `compatibility_scorecard`, `bundle_archetype_matrix` |

Beta-only surfaces (do not block M4 stable planning, but must remain
honest):

- Managed-cloud daily-driver parity stays a scoped handoff; widening requires
  release-council review and a known-limit note.
- `.NET service or app` and notebook-first data workflow stay held for M4 or
  later and enter through the change-control rules in
  `artifacts/milestones/m3/claimed_surface_register.json`.
- Long-tail framework breadth beyond the named archetypes is not a beta
  claim and remains out of scope.

## Downstream consumers

- Stable API freeze (M4): see
  `docs/governance/public_interface_versioning_policy.md` and
  `docs/governance/interface_freeze_guide.md`; the unlock map names which
  cutline rows must be green first.
- Release-center rehearsal (M4): see
  `docs/release/release_center_object_model_contract.md` and
  `docs/release/shiproom_runbook.md`; the unlock map gates rehearsal entry on
  the rollback, policy, and supportability rows.
- Certified-archetype publication (M4): see
  `docs/release/certified_archetype_report_template.md` and
  `artifacts/compat/reference_workspace_rows.yaml`; the unlock map ties this
  to the compatibility-publication and debug/test/task rows.

## How to validate

Run:

`python3 ci/check_m3_cutline_packet.py --repo-root .`

Optional machine-readable report:

`python3 ci/check_m3_cutline_packet.py --repo-root . --report artifacts/milestones/m3/captures/cutline_packet_validation_capture.json`

The cutline validator does not replace the beta admission validator; run
both in the same change set when any of the canonical M3 artifacts move:

`python3 ci/check_beta_admission.py --repo-root .`

## Update rules

1. Land scope changes in `artifacts/milestones/m3/claimed_surface_register.json`
   and `artifacts/milestones/m3/cohort_guardrails.yaml` first.
2. Update this page so the non-descopable truths table mirrors the new beta
   surface set.
3. Update `artifacts/milestones/m3/unlock_map.yaml` so every downstream M4
   unlock cites the cutline rows it depends on.
4. Update `artifacts/milestones/m3/stable_planning_handoff_checklist.md` so
   M4 planning consumes one canonical list of artifacts.
5. Run both validators and refresh the captures in the same change set.
