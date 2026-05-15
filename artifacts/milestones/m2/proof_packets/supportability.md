# External alpha supportability proof packet

```yaml
packet_id: review_packet:alpha.supportability.2026-05-15
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.supportability
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T08:32:32Z
stale_after: P14D
source_revision: git:b0ca733f3acd389de67a2270877d152f1b097a66
trigger_revision: alpha_supportability_contract_set@2026-05-15
exact_build_identity_ref: artifacts/build/build_identity.json
channel_context: preview
deployment_context:
  - individual_local
claim_change_state: no_claim_widening
same_change_truth_refs:
  docs_ref: docs/milestones/m2_alpha_scope.md
  migration_ref: docs/migration/source_ecosystem_coverage_matrix.md
  help_truth_ref: docs/docs/help_about_service_health_routes.md
  known_limits_ref: artifacts/feedback/external_alpha_known_limits.md
  support_export_ref: docs/support/support_bundle_contract.md
```

This packet registers the current supportability floor for the external alpha
wedges. It closes the blocked scoreboard state by proving the narrow support
path is reviewable across safe mode, Project Doctor, local support-bundle
preview/export, default redaction, and issue intake.

The packet does not widen the alpha product claim. It proves a supportability
wedge over checked-in contracts, fixtures, and Rust projections. Hosted ticket
submission, byte-level redaction implementation, live upload transport, full
support portal routing, and broad support automation remain outside this
packet.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.supportability`
- Support-bundle contract: `docs/support/support_bundle_contract.md`
- Project Doctor packet: `docs/support/project_doctor_packet.md`
- Recovery ladder packet: `docs/support/recovery_ladder_packet.md`
- Support intake contract: `docs/support/support_intake_and_escalation_contract.md`
- Object handoff packet: `docs/support/object_handoff_packet.md`
- Design-partner intake checklist: `artifacts/program/design_partner_intake_checklist.yaml`
- Known-limits packet: `artifacts/milestones/m2/known_limits_alpha.yaml`
- Reference workspace dry run: `artifacts/milestones/m2/reference_workspace_dry_run.md`
- Publication rehearsal: `artifacts/benchmarks/m2_publication_rehearsal.md`
- Latest capture: `artifacts/milestones/m2/captures/supportability_validation_capture.json`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_narrowed_supportability_floor`

The row is green because the repository now has current proof that:

- safe-mode entry preserves user state, disables risky launch paths, and keeps
  evidence export available without leaving safe mode;
- Project Doctor uses read-only alpha probes, stable finding codes, shared
  machine/human output vocabulary, and metadata-safe support export rows;
- support-bundle preview/export carries exact-build identity, local-first
  redaction defaults, a reopenable local preview, and a prohibited secret row
  in the failure drill; and
- issue intake can compose an incident workspace, runbook summary, missing-span
  disclosure, support-bundle ref, redaction controls, and local continuity
  state without claiming unavailable spans are present.

The row remains deliberately narrowed by
`known_limit:external_alpha.support_export_redaction_required`. Raw traces,
logs, screenshots, transcripts, and partner workspace content still require
redaction review before sharing.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/supportability.md` |
| Latest capture | `artifacts/milestones/m2/captures/supportability_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | `support_scenario_quality_proof`, `stale_after: P14D` |

## Protected Proof Path

Run the substrate checks cited by the capture:

```sh
cargo test -p aureline-shell --test recovery_protected_walk
cargo test -p aureline-support --test project_doctor_probe_pack_alpha
cargo test -p aureline-support --test support_bundle_seed_protected_walk
cargo test -p aureline-support --test recovery_ladder_alpha
cargo test -p aureline-incident --test incident_workspace_alpha
python3 ci/check_alpha_scope.py --repo-root .
```

## Coverage

The capture exercises the four required supportability legs:

- `safe_mode` - shell recovery profile and crash-loop containment expose safe
  mode, open without restore, extension/runtime quarantine, export evidence,
  and cache/index repair as visible rung options while preserving user state.
- `project_doctor` - the support crate consumes the read-only probe pack and
  the doctor runtime emits metadata-safe support/export rows from stable
  findings.
- `support_bundle_redaction` - the support bundle seed proves local preview,
  exact-build identity capture, local-first redaction defaults, and prohibited
  raw secret export in the failure drill.
- `issue_intake` - the incident workspace packet attaches support-bundle
  preview refs, runbook packet refs, action reconstruction context, local
  continuity, missing-span disclosure, and redacted export controls.

## Substrate Consumed

- `crates/aureline-shell/src/recovery/` owns safe-mode profile projection,
  crash-loop containment, suspicious-content handoff, and first-class recovery
  offers.
- `crates/aureline-doctor/src/probes/` owns read-only alpha Doctor diagnosis,
  stable finding codes, headless exit classes, and metadata-safe support
  export rows.
- `crates/aureline-support/src/project_doctor/` consumes the checked-in probe
  pack and validates shared machine/human output vocabulary.
- `crates/aureline-support/src/bundle/` owns local preview, exact-build
  capture, redaction defaults, preview parity, and prohibited high-risk rows.
- `crates/aureline-support/src/recovery_ladder/` owns bounded recovery ladder
  decisions and metadata-only support/release projections.
- `crates/aureline-shell/src/support_seed/` projects the support-bundle
  preview into shell-facing actions without enabling reserved upload or hosted
  intake actions.
- `crates/aureline-incident/` owns incident workspace packets, runbook
  summaries, missing-span disclosure, support-bundle linkages, and redacted
  export preview for issue intake.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture added.
- Scoreboard row moved to `green` and `conditional_go`.
- Scoreboard row now cites the recovery ladder, support intake, object
  handoff, owning packet, and latest capture alongside the existing support
  bundle, Project Doctor, design-partner, known-limit, dry-run, and rehearsal
  artifacts.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
- `known_limit:external_alpha.support_export_redaction_required` remains
  active and promotion-blocking for raw support artifacts.
